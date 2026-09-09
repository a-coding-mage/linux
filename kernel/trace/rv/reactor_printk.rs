// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019-2022 Red Hat, Inc. Daniel Bristot de Oliveira <bristot@kernel.org>
 *
 * Printk RV reactor:
 *   Prints the exception msg to the kernel message log.
 */

use core::ffi::{c_char, c_void};

// Supplied by the Linux kernel headers.
pub type VaList = *mut c_void;

#[repr(C)]
pub struct RvReactor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub react: Option<unsafe extern "C" fn(*const c_char, VaList)>,
}

unsafe extern "C" {
    fn vprintk_deferred(msg: *const c_char, args: VaList);
    fn rv_register_reactor(reactor: *mut RvReactor);
    fn rv_unregister_reactor(reactor: *mut RvReactor);
}

unsafe extern "C" fn rv_printk_reaction(msg: *const c_char, args: VaList) {
    unsafe {
        vprintk_deferred(msg, args);
    }
}

static mut RV_PRINTK: RvReactor = RvReactor {
    name: b"printk\0".as_ptr() as *const c_char,
    description: b"prints the exception msg to the kernel message log.\0".as_ptr()
        as *const c_char,
    react: Some(rv_printk_reaction),
};

unsafe extern "C" fn register_react_printk() -> i32 {
    unsafe {
        rv_register_reactor(&raw mut RV_PRINTK);
    }
    0
}

unsafe extern "C" fn unregister_react_printk() {
    unsafe {
        rv_unregister_reactor(&raw mut RV_PRINTK);
    }
}

// Equivalent to module_init(register_react_printk).
// Equivalent to module_exit(unregister_react_printk).

// Equivalent to MODULE_AUTHOR("Daniel Bristot de Oliveira").
// Equivalent to MODULE_DESCRIPTION("printk rv reactor: printk if an exception is hit.").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
