// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Bus error event handling code for 5000-series systems equipped
 * with parity error detection logic.
 *
 * Copyright (c) 2005, 2026 Maciej W. Rozycki
 */

// Linux and architecture-specific declarations are supplied by the surrounding
// translation unit; the original C includes are intentionally omitted.

#[inline]
unsafe fn dec_kn02xa_be_ack() {
    let mer = CKSEG1ADDR(KN02XA_MER) as *mut u32;
    let mem_intr = CKSEG1ADDR(KN02XA_MEM_INTR) as *mut u32;

    core::ptr::write_volatile(mer, KN02CA_MER_INTR); // Clear errors; keep the ARC IRQ.
    core::ptr::write_volatile(mem_intr, 0); // Any write clears the bus IRQ.
    iob();
}

unsafe fn dec_kn02xa_be_backend(regs: *mut pt_regs, is_fixup: i32, invoker: i32) -> i32 {
    let kn02xa_mer = CKSEG1ADDR(KN02XA_MER) as *mut u32;
    let kn02xa_ear = CKSEG1ADDR(KN02XA_EAR) as *mut u32;

    static EXCSTR: &[u8] = b"exception\0";
    static INTSTR: &[u8] = b"interrupt\0";
    static CPUSTR: &[u8] = b"CPU\0";
    static MREADSTR: &[u8] = b"memory read\0";
    static READSTR: &[u8] = b"read\0";
    static WRITESTR: &[u8] = b"write\0";
    static TIMESTR: &[u8] = b"timeout\0";
    static PARITYSTR: &[u8] = b"parity error\0";
    static LANESTAT: [&[u8; 4]; 2] = [b" OK\0", b"BAD\0"];

    // DEFINE_RATELIMIT_STATE(rs, DEFAULT_RATELIMIT_INTERVAL,
    //                        DEFAULT_RATELIMIT_BURST)
    static mut RS: ratelimit_state = ratelimit_state {
        interval: DEFAULT_RATELIMIT_INTERVAL,
        burst: DEFAULT_RATELIMIT_BURST,
        ..ratelimit_state::default()
    };

    let mer = core::ptr::read_volatile(kn02xa_mer);
    let ear = core::ptr::read_volatile(kn02xa_ear);
    let mut action = MIPS_BE_FATAL;

    // Ack ASAP, so that any subsequent errors get caught.
    dec_kn02xa_be_ack();

    let kind = if invoker != 0 { INTSTR.as_ptr() } else { EXCSTR.as_ptr() };

    // No DMA errors?
    let agent = CPUSTR.as_ptr();
    let address = (ear & KN02XA_EAR_ADDRESS) as c_ulong;

    // Low 256MB is decoded as memory, high -- as TC.
    let (cycle, event) = if address < 0x10000000 {
        (MREADSTR.as_ptr(), PARITYSTR.as_ptr())
    } else {
        (if invoker != 0 { WRITESTR.as_ptr() } else { READSTR.as_ptr() }, TIMESTR.as_ptr())
    };

    if is_fixup != 0 {
        action = MIPS_BE_FIXUP;
    }

    if action != MIPS_BE_FIXUP && __ratelimit(&raw mut RS) != 0 {
        printk(KERN_ALERT, b"Bus error %s: %s %s %s at %#010lx\n\0".as_ptr(),
               kind, agent, cycle, event, address);

        if address < 0x10000000 {
            printk(KERN_ALERT, b"  Byte lane status %#3x -- #3: %s, #2: %s, #1: %s, #0: %s\n\0".as_ptr(),
                   (mer & KN02XA_MER_BYTERR) >> 8,
                   LANESTAT[((mer & KN02XA_MER_BYTERR_3) != 0) as usize].as_ptr(),
                   LANESTAT[((mer & KN02XA_MER_BYTERR_2) != 0) as usize].as_ptr(),
                   LANESTAT[((mer & KN02XA_MER_BYTERR_1) != 0) as usize].as_ptr(),
                   LANESTAT[((mer & KN02XA_MER_BYTERR_0) != 0) as usize].as_ptr());
        }
    }

    action
}

pub unsafe fn dec_kn02xa_be_handler(regs: *mut pt_regs, is_fixup: i32) -> i32 {
    dec_kn02xa_be_backend(regs, is_fixup, 0)
}

pub unsafe fn dec_kn02xa_be_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let regs = get_irq_regs();
    let action = dec_kn02xa_be_backend(regs, 0, 1);

    if action == MIPS_BE_DISCARD {
        return IRQ_HANDLED;
    }

    /*
     * FIXME: Find the affected processes and kill them, otherwise
     * we must die.
     *
     * The interrupt is asynchronously delivered thus EPC and RA
     * may be irrelevant, but are printed for a reference.
     */
    printk(KERN_ALERT, b"Fatal bus interrupt, epc == %08lx, ra == %08lx\n\0".as_ptr(),
           (*regs).cp0_epc, (*regs).regs[31]);
    die(b"Unrecoverable bus error\0".as_ptr(), regs);
}

pub unsafe fn dec_kn02xa_be_init() {
    let mbcs = CKSEG1ADDR(KN4K_SLOT_BASE + KN4K_MB_CSR) as *mut u32;

    // For KN04 we need to make sure EE (?) is enabled in the MB.
    if current_cpu_type() == CPU_R4000SC {
        let value = core::ptr::read_volatile(mbcs);
        core::ptr::write_volatile(mbcs, value | KN4K_MB_CSR_EE);
    }
    fast_iob();

    // Clear any leftover errors from the firmware.
    dec_kn02xa_be_ack();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
