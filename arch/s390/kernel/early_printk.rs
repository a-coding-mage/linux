// SPDX-License-Identifier: GPL-2.0
/*
 *    Copyright IBM Corp. 2017
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct Console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(con: *mut Console, s: *const c_char, len: c_uint)>,
    pub flags: c_uint,
    pub index: c_int,
}

extern "C" {
    static mut early_console: *mut Console;
    static mut sclp: Sclp;

    fn __sclp_early_printk(s: *const c_char, len: c_uint);
    fn register_console(con: *mut Console);
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
}

#[repr(C)]
pub struct Sclp {
    pub has_linemode: bool,
    pub has_vt220: bool,
}

const CON_PRINTBUFFER: c_uint = 1 << 0;
const CON_BOOT: c_uint = 1 << 1;

unsafe extern "C" fn sclp_early_write(
    _con: *mut Console,
    s: *const c_char,
    len: c_uint,
) {
    __sclp_early_printk(s, len);
}

static mut sclp_early_console: Console = Console {
    name: b"earlysclp\0".as_ptr() as *const c_char,
    write: Some(sclp_early_write),
    flags: CON_PRINTBUFFER | CON_BOOT,
    index: -1,
};

pub unsafe extern "C" fn register_early_console() {
    if early_console != core::ptr::null_mut() {
        return;
    }
    if !sclp.has_linemode && !sclp.has_vt220 {
        return;
    }
    early_console = &raw mut sclp_early_console;
    register_console(early_console);
}

unsafe extern "C" fn setup_early_printk(buf: *mut c_char) -> c_int {
    if early_console != core::ptr::null_mut() {
        return 0;
    }
    /* Accept only "earlyprintk" and "earlyprintk=sclp" */
    if !buf.is_null()
        && !str_has_prefix(buf as *const c_char, b"sclp\0".as_ptr() as *const c_char)
    {
        return 0;
    }
    register_early_console();
    0
}

// early_param("earlyprintk", setup_early_printk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
