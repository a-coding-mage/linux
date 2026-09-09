// SPDX-License-Identifier: GPL-2.0
// External Linux kernel and Speakup dependencies are supplied by other files.

#[repr(C)]
struct SpkLdiscData {
    buf: ::core::ffi::c_char,
    completion: Completion,
    buf_free: bool,
    synth: *mut SpkSynth,
}

// This allows to catch within spk_ttyio_ldisc_open whether it is getting set
// on for a speakup-driven device.
static mut SPEAKUP_TTY: *mut TtyStruct = ::core::ptr::null_mut();
// This mutex serializes the use of such global speakup_tty variable
static SPEAKUP_TTY_MUTEX: Mutex = DEFINE_MUTEX!();

unsafe fn ser_to_dev(ser: i32, dev_no: *mut DevT) -> i32 {
    if ser < 0 || ser > (255 - 64) {
        pr_err!("speakup: Invalid ser param. Must be between 0 and 191 inclusive.\n");
        return -EINVAL;
    }

    *dev_no = MKDEV(4, 64 + ser);
    0
}

unsafe fn get_dev_to_use(synth: *mut SpkSynth, dev_no: *mut DevT) -> i32 {
    // use ser only when dev is not specified
    if strcmp((*synth).dev_name, SYNTH_DEFAULT_DEV) != 0
        || (*synth).ser == SYNTH_DEFAULT_SER
    {
        return tty_dev_name_to_number((*synth).dev_name, dev_no);
    }

    ser_to_dev((*synth).ser, dev_no)
}

unsafe extern "C" fn spk_ttyio_ldisc_open(tty: *mut TtyStruct) -> i32 {
    if tty != SPEAKUP_TTY {
        // Somebody tried to use this line discipline outside speakup
        return -ENODEV;
    }

    if (*(*tty).ops).write.is_none() {
        return -EOPNOTSUPP;
    }

    let ldisc_data = kmalloc_obj::<SpkLdiscData>();
    if ldisc_data.is_null() {
        return -ENOMEM;
    }

    init_completion(&mut (*ldisc_data).completion);
    (*ldisc_data).buf_free = true;
    (*tty).disc_data = ldisc_data as *mut _;
    0
}

unsafe extern "C" fn spk_ttyio_ldisc_close(tty: *mut TtyStruct) {
    kfree((*tty).disc_data);
}

unsafe extern "C" fn spk_ttyio_receive_buf2(
    tty: *mut TtyStruct,
    cp: *const u8,
    _fp: *const u8,
    count: usize,
) -> usize {
    let ldisc_data = (*tty).disc_data as *mut SpkLdiscData;
    let synth = (*ldisc_data).synth;

    if !(*synth).read_buff_add.is_none() {
        for i in 0..count {
            ((*synth).read_buff_add.unwrap())((*cp.add(i)));
        }
        return count;
    }

    if !(*ldisc_data).buf_free {
        // ttyio_in will tty_flip_buffer_push
        return 0;
    }

    // Make sure the consumer has read buf before we have seen
    // buf_free == true and overwrite buf
    mb!();

    (*ldisc_data).buf = *cp as ::core::ffi::c_char;
    (*ldisc_data).buf_free = false;
    complete(&mut (*ldisc_data).completion);
    1
}

static mut SPK_TTYIO_LDISC_OPS: TtyLdiscOps = TtyLdiscOps {
    owner: THIS_MODULE,
    num: N_SPEAKUP,
    name: b"speakup_ldisc\0".as_ptr() as *const _,
    open: Some(spk_ttyio_ldisc_open),
    close: Some(spk_ttyio_ldisc_close),
    receive_buf2: Some(spk_ttyio_receive_buf2),
};

unsafe extern "C" fn spk_ttyio_out(in_synth: *mut SpkSynth, ch: ::core::ffi::c_char) -> i32;
unsafe extern "C" fn spk_ttyio_out_unicode(in_synth: *mut SpkSynth, ch: u16) -> i32;
unsafe extern "C" fn spk_ttyio_send_xchar(in_synth: *mut SpkSynth, ch: ::core::ffi::c_char);
unsafe extern "C" fn spk_ttyio_tiocmset(in_synth: *mut SpkSynth, set: u32, clear: u32);
unsafe extern "C" fn spk_ttyio_in(in_synth: *mut SpkSynth) -> u8;
unsafe extern "C" fn spk_ttyio_in_nowait(in_synth: *mut SpkSynth) -> u8;
unsafe extern "C" fn spk_ttyio_flush_buffer(in_synth: *mut SpkSynth);
unsafe extern "C" fn spk_ttyio_wait_for_xmitr(in_synth: *mut SpkSynth) -> i32;

#[no_mangle]
pub static mut spk_ttyio_ops: SpkIoOps = SpkIoOps {
    synth_out: Some(spk_ttyio_out),
    synth_out_unicode: Some(spk_ttyio_out_unicode),
    send_xchar: Some(spk_ttyio_send_xchar),
    tiocmset: Some(spk_ttyio_tiocmset),
    synth_in: Some(spk_ttyio_in),
    synth_in_nowait: Some(spk_ttyio_in_nowait),
    flush_buffer: Some(spk_ttyio_flush_buffer),
    wait_for_xmitr: Some(spk_ttyio_wait_for_xmitr),
};

unsafe fn get_termios(tty: *mut TtyStruct, out_termios: *mut Ktermios) {
    down_read(&mut (*tty).termios_rwsem);
    *out_termios = (*tty).termios;
    up_read(&mut (*tty).termios_rwsem);
}

unsafe fn spk_ttyio_initialise_ldisc(synth: *mut SpkSynth) -> i32 {
    let mut ret = 0;
    let mut dev: DevT = 0;
    ret = get_dev_to_use(synth, &mut dev);
    if ret != 0 { return ret; }

    let tty = tty_kopen_exclusive(dev);
    if IS_ERR(tty) { return PTR_ERR(tty); }

    ret = if let Some(open) = (*(*tty).ops).open { open(tty, ::core::ptr::null_mut()) } else { -ENODEV };
    if ret != 0 { tty_unlock(tty); return ret; }

    clear_bit(TTY_HUPPED, &mut (*tty).flags);
    // ensure hardware flow control is enabled
    let mut tmp_termios = Ktermios::default();
    get_termios(tty, &mut tmp_termios);
    if tmp_termios.c_cflag & CRTSCTS == 0 {
        tmp_termios.c_cflag |= CRTSCTS;
        tty_set_termios(tty, &tmp_termios);
        // check c_cflag to see if it's updated as tty_set_termios
        // may not return error even when no tty bits are changed by the request.
        get_termios(tty, &mut tmp_termios);
        if tmp_termios.c_cflag & CRTSCTS == 0 { pr_warn!("speakup: Failed to set hardware flow control\n"); }
    }
    tty_unlock(tty);

    mutex_lock(&SPEAKUP_TTY_MUTEX);
    SPEAKUP_TTY = tty;
    ret = tty_set_ldisc(tty, N_SPEAKUP);
    SPEAKUP_TTY = ::core::ptr::null_mut();
    mutex_unlock(&SPEAKUP_TTY_MUTEX);

    if ret == 0 {
        let ldisc_data = (*tty).disc_data as *mut SpkLdiscData;
        (*ldisc_data).synth = synth;
        (*synth).dev = tty;
        return 0;
    }
    pr_err!("speakup: Failed to set N_SPEAKUP on tty\n");
    tty_lock(tty);
    if let Some(close) = (*(*tty).ops).close { close(tty, ::core::ptr::null_mut()); }
    tty_unlock(tty);
    tty_kclose(tty);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn spk_ttyio_register_ldisc() {
    if tty_register_ldisc(&mut SPK_TTYIO_LDISC_OPS) != 0 { pr_warn!("speakup: Error registering line discipline. Most synths won't work.\n"); }
}

#[no_mangle]
pub unsafe extern "C" fn spk_ttyio_unregister_ldisc() { tty_unregister_ldisc(&mut SPK_TTYIO_LDISC_OPS); }

unsafe extern "C" fn spk_ttyio_out(in_synth: *mut SpkSynth, ch: ::core::ffi::c_char) -> i32 {
    let tty = (*in_synth).dev;
    if !(*in_synth).alive || (*(*tty).ops).write.is_none() { return 0; }
    let ret = (*(*tty).ops).write.unwrap()(tty, &ch, 1);
    if ret == 0 { return 0; }
    if ret > 0 { return 1; }
    pr_warn!("%s: I/O error, deactivating speakup\n", (*in_synth).long_name);
    // No synth any more, so nobody will restart TTYs, and we thus need to do it ourselves.
    (*in_synth).alive = 0;
    speakup_start_ttys();
    0
}

unsafe extern "C" fn spk_ttyio_out_unicode(in_synth: *mut SpkSynth, ch: u16) -> i32 {
    let mut ret;
    if ch < 0x80 { ret = spk_ttyio_out(in_synth, ch as _); }
    else if ch < 0x800 { ret = spk_ttyio_out(in_synth, (0xc0 | (ch >> 6)) as _); ret &= spk_ttyio_out(in_synth, (0x80 | (ch & 0x3f)) as _); }
    else { ret = spk_ttyio_out(in_synth, (0xe0 | (ch >> 12)) as _); ret &= spk_ttyio_out(in_synth, (0x80 | ((ch >> 6) & 0x3f)) as _); ret &= spk_ttyio_out(in_synth, (0x80 | (ch & 0x3f)) as _); }
    ret
}

unsafe extern "C" fn spk_ttyio_send_xchar(s: *mut SpkSynth, ch: ::core::ffi::c_char) { let tty = (*s).dev; if let Some(f) = (*(*tty).ops).send_xchar { f(tty, ch); } }
unsafe extern "C" fn spk_ttyio_tiocmset(s: *mut SpkSynth, set: u32, clear: u32) { let tty = (*s).dev; if let Some(f) = (*(*tty).ops).tiocmset { f(tty, set, clear); } }
unsafe extern "C" fn spk_ttyio_wait_for_xmitr(_s: *mut SpkSynth) -> i32 { 1 }

unsafe fn ttyio_in(s: *mut SpkSynth, timeout: i32) -> u8 {
    let tty = (*s).dev;
    let d = (*tty).disc_data as *mut SpkLdiscData;
    if timeout == 0 { if !try_wait_for_completion(&mut (*d).completion) { return 0xff; } }
    else if wait_for_completion_timeout(&mut (*d).completion, usecs_to_jiffies(timeout)) == 0 { pr_warn!("spk_ttyio: timeout (%d)  while waiting for input\n", timeout); return 0xff; }
    let rv = (*d).buf as u8;
    mb!();
    (*d).buf_free = true;
    tty_flip_buffer_push((*tty).port);
    rv
}

unsafe extern "C" fn spk_ttyio_in(s: *mut SpkSynth) -> u8 { ttyio_in(s, SPK_SYNTH_TIMEOUT) }
unsafe extern "C" fn spk_ttyio_in_nowait(s: *mut SpkSynth) -> u8 { let rv = ttyio_in(s, 0); if rv == 0xff { 0 } else { rv } }
unsafe extern "C" fn spk_ttyio_flush_buffer(s: *mut SpkSynth) { let tty = (*s).dev; if let Some(f) = (*(*tty).ops).flush_buffer { f(tty); } }

#[no_mangle]
pub unsafe extern "C" fn spk_ttyio_synth_probe(synth: *mut SpkSynth) -> i32 { let rv = spk_ttyio_initialise_ldisc(synth); if rv != 0 { return rv; } (*synth).alive = 1; 0 }

#[no_mangle]
pub unsafe extern "C" fn spk_ttyio_release(s: *mut SpkSynth) {
    let tty = (*s).dev;
    if tty.is_null() { return; }
    tty_lock(tty);
    if let Some(close) = (*(*tty).ops).close { close(tty, ::core::ptr::null_mut()); }
    tty_ldisc_flush(tty);
    tty_unlock(tty);
    tty_kclose(tty);
    (*s).dev = ::core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn spk_ttyio_synth_immediate(s: *mut SpkSynth, mut buff: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    let tty = (*s).dev;
    loop {
        let mut ch = *buff as u8;
        if ch == 0 { return ::core::ptr::null(); }
        if ch == b'\n' { ch = (*s).procspeech as u8; }
        if tty_write_room(tty) < 1 || ((*s).io_ops).as_ref().unwrap().synth_out.unwrap()(s, ch as _) == 0 { return buff; }
        buff = buff.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
