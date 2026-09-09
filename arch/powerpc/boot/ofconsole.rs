// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OF console routines
 *
 * Copyright (C) Paul Mackerras 1997.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

// Declarations supplied by the surrounding repository headers.
#[repr(C)]
pub struct ConsoleOps {
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub write: Option<unsafe extern "C" fn(*const c_char, c_int)>,
}

unsafe extern "C" {
    static mut console_ops: ConsoleOps;

    fn of_finddevice(name: *const c_char) -> *mut c_void;
    fn of_getprop(
        device: *mut c_void,
        name: *const c_char,
        buf: *mut c_void,
        buflen: c_int,
    ) -> c_int;
    fn be32_to_cpu(value: c_uint) -> c_uint;
    fn of_call_prom(
        service: *const c_char,
        nargs: c_int,
        nret: c_int,
        handle: c_uint,
        buf: *const c_char,
        len: c_int,
    );
}

static mut of_stdout_handle: c_uint = 0;

unsafe extern "C" fn of_console_open() -> c_int {
    let mut devp: *mut c_void;

    devp = of_finddevice(b"/chosen\0".as_ptr() as *const c_char);
    if !devp.is_null()
        && of_getprop(
            devp,
            b"stdout\0".as_ptr() as *const c_char,
            core::ptr::addr_of_mut!(of_stdout_handle) as *mut c_void,
            size_of::<c_uint>() as c_int,
        ) == size_of::<c_uint>() as c_int
    {
        of_stdout_handle = be32_to_cpu(of_stdout_handle);
        return 0;
    }

    -1
}

unsafe extern "C" fn of_console_write(buf: *const c_char, len: c_int) {
    of_call_prom(
        b"write\0".as_ptr() as *const c_char,
        3,
        1,
        of_stdout_handle,
        buf,
        len,
    );
}

pub unsafe extern "C" fn of_console_init() {
    console_ops.open = Some(of_console_open);
    console_ops.write = Some(of_console_write);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
