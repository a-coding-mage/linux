// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019-2022 Red Hat, Inc. Daniel Bristot de Oliveira <bristot@kernel.org>
 *
 * Panic RV reactor:
 *   Prints the exception msg to the kernel message log and panic().
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct rv_reactor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub react: unsafe extern "C" fn(*const c_char, *mut c_void),
}

unsafe extern "C" {
    fn vpanic(msg: *const c_char, args: *mut c_void);
    fn rv_register_reactor(reactor: *mut rv_reactor);
    fn rv_unregister_reactor(reactor: *mut rv_reactor);
}

unsafe extern "C" fn rv_panic_reaction(msg: *const c_char, args: *mut c_void) {
    unsafe {
        vpanic(msg, args);
    }
}

static mut rv_panic: rv_reactor = rv_reactor {
    name: b"panic\0".as_ptr() as *const c_char,
    description: b"panic the system if an exception is found.\0".as_ptr() as *const c_char,
    react: rv_panic_reaction,
};

unsafe extern "C" fn register_react_panic() -> c_int {
    unsafe {
        rv_register_reactor(&raw mut rv_panic);
    }
    0
}

unsafe extern "C" fn unregister_react_panic() {
    unsafe {
        rv_unregister_reactor(&raw mut rv_panic);
    }
}

// module_init(register_react_panic);
// module_exit(unregister_react_panic);

#[used]
#[no_mangle]
pub static MODULE_AUTHOR: &[u8] = b"Daniel Bristot de Oliveira\0";

#[used]
#[no_mangle]
pub static MODULE_DESCRIPTION: &[u8] = b"panic rv reactor: panic if an exception is found.\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
