// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct chan_opts {
    pub announce: Option<unsafe extern "C" fn(*mut c_char, c_int)>,
    pub raw: c_int,
}

#[repr(C)]
pub struct chan_ops {
    pub type_: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut c_char, c_int, *const chan_opts) -> *mut c_void>,
    pub open: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_void, *mut *mut c_char) -> c_int>,
    pub close: Option<unsafe extern "C" fn(c_int, *mut c_void)>,
    pub read: Option<unsafe extern "C" fn(c_int, *mut c_void, *mut c_char, c_int) -> c_int>,
    pub write: Option<unsafe extern "C" fn(c_int, *mut c_void, *const c_char, c_int) -> c_int>,
    pub console_write: Option<unsafe extern "C" fn(c_int, *mut c_void, *const c_char, c_int) -> c_int>,
    pub window_size: Option<unsafe extern "C" fn(c_int, *mut c_void, c_int, c_int) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub winch: c_int,
}

#[repr(C)]
struct pty_chan {
    announce: Option<unsafe extern "C" fn(*mut c_char, c_int)>,
    dev: c_int,
    raw: c_int,
    tt: termios,
    dev_name: [c_char; core::mem::size_of::<[u8; 18]>()],
}

extern "C" {
    fn uml_kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn get_pty() -> c_int;
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn raw(fd: c_int) -> c_int;
    fn ptsname(fd: c_int) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn generic_close(fd: c_int, d: *mut c_void);
    fn generic_read(fd: c_int, d: *mut c_void, buf: *mut c_char, len: c_int) -> c_int;
    fn generic_write(fd: c_int, d: *mut c_void, buf: *const c_char, len: c_int) -> c_int;
    fn generic_console_write(fd: c_int, d: *mut c_void, buf: *const c_char, len: c_int) -> c_int;
    fn generic_window_size(fd: c_int, d: *mut c_void, rows: c_int, cols: c_int) -> c_int;
    fn generic_free(d: *mut c_void);
    fn printk(format: *const c_char, ...);
}

const UM_GFP_KERNEL: c_int = 0;
const O_RDWR: c_int = 2;
const R_OK: c_int = 4;
const W_OK: c_int = 2;
const ENOENT: c_int = 2;

unsafe extern "C" fn pty_chan_init(_str: *mut c_char, device: c_int, opts: *const chan_opts) -> *mut c_void {
    let data = uml_kmalloc(core::mem::size_of::<pty_chan>(), UM_GFP_KERNEL) as *mut pty_chan;
    if data.is_null() {
        return core::ptr::null_mut();
    }
    (*data).announce = (*opts).announce;
    (*data).dev = device;
    (*data).raw = (*opts).raw;
    data as *mut c_void
}

unsafe extern "C" fn pts_open(_input: c_int, _output: c_int, _primary: c_int, d: *mut c_void, dev_out: *mut *mut c_char) -> c_int {
    let data = d as *mut pty_chan;
    let fd = get_pty();
    if fd < 0 {
        printk(b"open_pts : Failed to open pts\0".as_ptr() as *const c_char);
        return -1;
    }
    if (*data).raw != 0 {
        let err = tcgetattr(fd, &mut (*data).tt);
        if err != 0 { close(fd); return err; }
        let err = raw(fd);
        if err != 0 { close(fd); return err; }
    }
    let dev = ptsname(fd);
    sprintf((*data).dev_name.as_mut_ptr(), b"%s\0".as_ptr() as *const c_char, dev);
    *dev_out = (*data).dev_name.as_mut_ptr();
    if let Some(announce) = (*data).announce { announce(dev, (*data).dev); }
    fd
}

unsafe extern "C" fn getmaster(line: *mut c_char) -> c_int {
    let mut buf = core::mem::MaybeUninit::<stat>::uninit();
    let pty = line.add(strlen(b"/dev/ptyp\0".as_ptr() as *const c_char));
    for bank in b"pqrs\0".iter().take(4) {
        *line.add(strlen(b"/dev/pty\0".as_ptr() as *const c_char)) = *bank as c_char;
        *pty = b'0' as c_char;
        if stat(line, buf.as_mut_ptr()) < 0 { break; }
        for cp in b"0123456789abcdef\0".iter().take(16) {
            *pty = *cp as c_char;
            let master = open(line, O_RDWR);
            if master >= 0 {
                let tp = line.add(strlen(b"/dev/\0".as_ptr() as *const c_char));
                *tp = b't' as c_char;
                let err = access(line, R_OK | W_OK);
                *tp = b'p' as c_char;
                if err == 0 { return master; }
                close(master);
            }
        }
    }
    printk(b"getmaster - no usable host pty devices\0".as_ptr() as *const c_char);
    -ENOENT
}

unsafe extern "C" fn pty_open(_input: c_int, _output: c_int, _primary: c_int, d: *mut c_void, dev_out: *mut *mut c_char) -> c_int {
    let data = d as *mut pty_chan;
    let mut dev = *b"/dev/ptyxx\0";
    let fd = getmaster(dev.as_mut_ptr() as *mut c_char);
    if fd < 0 { return fd; }
    if (*data).raw != 0 {
        let err = raw(fd);
        if err != 0 { close(fd); return err; }
    }
    if let Some(announce) = (*data).announce { announce(dev.as_mut_ptr() as *mut c_char, (*data).dev); }
    sprintf((*data).dev_name.as_mut_ptr(), b"%s\0".as_ptr() as *const c_char, dev.as_ptr());
    *dev_out = (*data).dev_name.as_mut_ptr();
    fd
}

#[no_mangle]
pub static pty_ops: chan_ops = chan_ops { type_: b"pty\0".as_ptr() as *const c_char, init: Some(pty_chan_init), open: Some(pty_open), close: Some(generic_close), read: Some(generic_read), write: Some(generic_write), console_write: Some(generic_console_write), window_size: Some(generic_window_size), free: Some(generic_free), winch: 0 };

#[no_mangle]
pub static pts_ops: chan_ops = chan_ops { type_: b"pts\0".as_ptr() as *const c_char, init: Some(pty_chan_init), open: Some(pts_open), close: Some(generic_close), read: Some(generic_read), write: Some(generic_write), console_write: Some(generic_console_write), window_size: Some(generic_window_size), free: Some(generic_free), winch: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
