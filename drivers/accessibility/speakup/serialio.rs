// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/Speakup translation.

// WARNING: Do not change this to the equivalent serial header without testing
// that SERIAL_PORT_DFNS is defined to the appropriate value.

static mut SERSTATE: *const old_serial_port = core::ptr::null();
static mut TIMEOUTS: i32 = 0;

unsafe extern "C" {
    static rs_table: [old_serial_port; 0]; // SERIAL_PORT_DFNS supplies the entries.
}

unsafe fn start_serial_interrupt(irq: i32);

unsafe fn spk_serial_out(in_synth: *mut spk_synth, ch: i8) -> i32;
unsafe fn spk_serial_send_xchar(in_synth: *mut spk_synth, ch: i8);
unsafe fn spk_serial_tiocmset(in_synth: *mut spk_synth, set: u32, clear: u32);
unsafe fn spk_serial_in(in_synth: *mut spk_synth) -> u8;
unsafe fn spk_serial_in_nowait(in_synth: *mut spk_synth) -> u8;
unsafe fn spk_serial_flush_buffer(in_synth: *mut spk_synth);
unsafe fn spk_serial_wait_for_xmitr(in_synth: *mut spk_synth) -> i32;

#[no_mangle]
pub static mut spk_serial_io_ops: spk_io_ops = spk_io_ops {
    synth_out: Some(spk_serial_out),
    send_xchar: Some(spk_serial_send_xchar),
    tiocmset: Some(spk_serial_tiocmset),
    synth_in: Some(spk_serial_in),
    synth_in_nowait: Some(spk_serial_in_nowait),
    flush_buffer: Some(spk_serial_flush_buffer),
    wait_for_xmitr: Some(spk_serial_wait_for_xmitr),
};

pub unsafe fn spk_serial_init(index: i32) -> *const old_serial_port {
    let baud: i32 = 9600;
    let mut quot: i32 = 0;
    let mut cval: u32 = 0;
    let cflag: i32 = CREAD | HUPCL | CLOCAL | B9600 | CS8;
    let ser: *const old_serial_port;
    let mut err: i32;

    if index >= core::mem::size_of_val(&rs_table) as i32 / core::mem::size_of::<old_serial_port>() as i32 {
        pr_info!("no port info for ttyS{}\n", index);
        return core::ptr::null();
    }
    ser = rs_table.as_ptr().add(index as usize);

    quot = (*ser).baud_base / baud;
    cval = (cflag & (CSIZE | CSTOPB)) as u32;
    // On powerpc/alpha the C implementation shifts by 8; other targets shift by 4.
    #[cfg(any(target_arch = "powerpc", target_arch = "alpha"))]
    { cval >>= 8; }
    #[cfg(not(any(target_arch = "powerpc", target_arch = "alpha")))]
    { cval >>= 4; }
    if cflag & PARENB != 0 { cval |= UART_LCR_PARITY; }
    if cflag & PARODD == 0 { cval |= UART_LCR_EPAR; }
    if synth_request_region((*ser).port, 8) != 0 {
        pr_info!("Ports not available, trying to steal them\n");
        __release_region(&mut ioport_resource, (*ser).port, 8);
        err = synth_request_region((*ser).port, 8);
        if err != 0 {
            pr_warn!("Unable to allocate port at {:x}, errno {}", (*ser).port, err);
            return core::ptr::null();
        }
    }

    outb(cval | UART_LCR_DLAB, (*ser).port + UART_LCR);
    outb(quot & 0xff, (*ser).port + UART_DLL);
    outb(quot >> 8, (*ser).port + UART_DLM);
    outb(cval, (*ser).port + UART_LCR);
    outb(0, (*ser).port + UART_IER);
    outb(UART_MCR_DTR | UART_MCR_RTS, (*ser).port + UART_MCR);

    if inb((*ser).port + UART_LSR) == 0xff {
        synth_release_region((*ser).port, 8);
        SERSTATE = core::ptr::null();
        return core::ptr::null();
    }
    mdelay(1);
    speakup_info.port_tts = (*ser).port;
    SERSTATE = ser;
    start_serial_interrupt((*ser).irq);
    ser
}

unsafe extern "C" fn synth_readbuf_handler(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut flags: c_ulong = 0;
    let mut c: i32;
    spin_lock_irqsave(&mut speakup_info.spinlock, &mut flags);
    while inb_p(speakup_info.port_tts + UART_LSR) & UART_LSR_DR != 0 {
        c = inb_p(speakup_info.port_tts + UART_RX);
        (*synth).read_buff_add.unwrap() (c as u8);
    }
    spin_unlock_irqrestore(&mut speakup_info.spinlock, flags);
    IRQ_HANDLED
}

unsafe fn start_serial_interrupt(irq: i32) {
    if (*synth).read_buff_add.is_none() { return; }
    let rv = request_irq(irq, Some(synth_readbuf_handler), IRQF_SHARED, c"serial", synth_readbuf_handler as *mut core::ffi::c_void);
    if rv != 0 { pr_err!("Unable to request Speakup serial I R Q\n"); }
    outb(UART_MCR_DTR | UART_MCR_RTS | UART_MCR_OUT2, speakup_info.port_tts + UART_MCR);
    outb(UART_IER_MSI | UART_IER_RLSI | UART_IER_RDI, speakup_info.port_tts + UART_IER);
    inb(speakup_info.port_tts + UART_LSR); inb(speakup_info.port_tts + UART_RX);
    inb(speakup_info.port_tts + UART_IIR); inb(speakup_info.port_tts + UART_MSR);
    outb(1, speakup_info.port_tts + UART_FCR);
}

unsafe fn spk_serial_send_xchar(_in_synth: *mut spk_synth, ch: i8) {
    let mut timeout = SPK_XMITR_TIMEOUT;
    while spk_serial_tx_busy() != 0 { timeout -= 1; if timeout == 0 { break; } udelay(1); }
    outb(ch, speakup_info.port_tts);
}

unsafe fn spk_serial_tiocmset(_in_synth: *mut spk_synth, set: u32, clear: u32) {
    let old = inb(speakup_info.port_tts + UART_MCR);
    outb((old & !clear) | set, speakup_info.port_tts + UART_MCR);
}

pub unsafe fn spk_serial_synth_probe(synth: *mut spk_synth) -> i32 {
    let mut failed = 0;
    if (*synth).ser >= SPK_LO_TTY && (*synth).ser <= SPK_HI_TTY {
        let ser = spk_serial_init((*synth).ser);
        if ser.is_null() { failed = -1; } else { outb_p(0, (*ser).port); mdelay(1); outb_p(b'\r' as i32, (*ser).port); }
    } else { failed = -1; pr_warn!("ttyS{} is an invalid port\n", (*synth).ser); }
    if failed != 0 { pr_info!("{}: not found\n", (*synth).long_name); return -ENODEV; }
    pr_info!("{}: ttyS{}, Driver Version {}\n", (*synth).long_name, (*synth).ser, (*synth).version);
    (*synth).alive = 1; 0
}

pub unsafe fn spk_stop_serial_interrupt() {
    if speakup_info.port_tts == 0 || (*synth).read_buff_add.is_none() { return; }
    outb(0, speakup_info.port_tts + UART_IER);
    free_irq((*SERSTATE).irq, synth_readbuf_handler as *mut core::ffi::c_void);
}

unsafe fn spk_serial_wait_for_xmitr(in_synth: *mut spk_synth) -> i32 {
    let mut tmout = SPK_XMITR_TIMEOUT;
    if (*in_synth).alive != 0 && TIMEOUTS >= NUM_DISABLE_TIMEOUTS { pr_warn!("{}: too many timeouts, deactivating speakup\n", (*in_synth).long_name); (*in_synth).alive = 0; speakup_start_ttys(); TIMEOUTS = 0; return 0; }
    while spk_serial_tx_busy() != 0 { tmout -= 1; if tmout == 0 { pr_warn!("{}: timed out (tx busy)\n", (*in_synth).long_name); TIMEOUTS += 1; return 0; } udelay(1); }
    tmout = SPK_CTS_TIMEOUT;
    while inb_p(speakup_info.port_tts + UART_MSR) & UART_MSR_CTS == 0 { tmout -= 1; if tmout == 0 { TIMEOUTS += 1; return 0; } udelay(1); }
    TIMEOUTS = 0; 1
}

unsafe fn spk_serial_in(_in_synth: *mut spk_synth) -> u8 { let mut tmout = SPK_SERIAL_TIMEOUT; while inb_p(speakup_info.port_tts + UART_LSR) & UART_LSR_DR == 0 { tmout -= 1; if tmout == 0 { pr_warn!("time out while waiting for input.\n"); return 0xff; } udelay(1); } inb_p(speakup_info.port_tts + UART_RX) as u8 }
unsafe fn spk_serial_in_nowait(_in_synth: *mut spk_synth) -> u8 { let lsr = inb_p(speakup_info.port_tts + UART_LSR); if lsr & UART_LSR_DR == 0 { 0 } else { inb_p(speakup_info.port_tts + UART_RX) as u8 } }
unsafe fn spk_serial_flush_buffer(_in_synth: *mut spk_synth) { /* TODO: flush the UART 16550 buffer */ }
unsafe fn spk_serial_out(in_synth: *mut spk_synth, ch: i8) -> i32 { if (*in_synth).alive != 0 && spk_serial_wait_for_xmitr(in_synth) != 0 { outb_p(ch, speakup_info.port_tts); 1 } else { 0 } }

pub unsafe fn spk_serial_synth_immediate(synth: *mut spk_synth, mut buff: *const i8) -> *const i8 {
    let mut ch: i8;
    loop { ch = *buff; if ch == 0 { return core::ptr::null(); } if ch == b'\n' as i8 { ch = (*synth).procspeech; } if spk_serial_wait_for_xmitr(synth) != 0 { outb(ch, speakup_info.port_tts); } else { return buff; } buff = buff.add(1); }
}

pub unsafe fn spk_serial_release(_synth: *mut spk_synth) { spk_stop_serial_interrupt(); if speakup_info.port_tts == 0 { return; } synth_release_region(speakup_info.port_tts, 8); speakup_info.port_tts = 0; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
