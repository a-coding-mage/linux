// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/early_printk.c
 *
 *  Copyright (C) 2009 Sascha Hauer <s.hauer@pengutronix.de>
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    fn printascii(s: *const c_char);
    fn register_console(con: *mut console);
    static mut early_console: *mut console;
}

#[repr(C)]
pub struct console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>,
    pub flags: c_uint,
    pub index: c_int,
}

// CON_PRINTBUFFER | CON_BOOT, supplied by the console subsystem.
const CON_PRINTBUFFER: c_uint = 1 << 0;
const CON_BOOT: c_uint = 1 << 1;

unsafe extern "C" fn early_write(mut s: *const c_char, mut n: c_uint) {
    let mut buf = [0u8; 128];
    while n != 0 {
        let l = core::cmp::min(n as usize, core::mem::size_of_val(&buf) - 1);
        core::ptr::copy_nonoverlapping(s as *const u8, buf.as_mut_ptr(), l);
        buf[l] = 0;
        s = s.add(l);
        n -= l as c_uint;
        printascii(buf.as_ptr() as *const c_char);
    }
}

unsafe extern "C" fn early_console_write(
    _con: *mut console,
    s: *const c_char,
    n: c_uint,
) {
    early_write(s, n);
}

static mut early_console_dev: console = console {
    name: b"earlycon\0".as_ptr() as *const c_char,
    write: Some(early_console_write),
    flags: CON_PRINTBUFFER | CON_BOOT,
    index: -1,
};

unsafe extern "C" fn setup_early_printk(_buf: *mut c_char) -> c_int {
    early_console = &raw mut early_console_dev;
    register_console(&raw mut early_console_dev);
    0
}

// early_param("earlyprintk", setup_early_printk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
