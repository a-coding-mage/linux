// SPDX-License-Identifier: GPL-2.0
/*
 * hypervisor.c - /sys/hypervisor subsystem.
 *
 * Copyright (C) IBM Corp. 2006
 * Copyright (C) 2007 Greg Kroah-Hartman <gregkh@suse.de>
 * Copyright (C) 2007 Novell Inc.
 */

use core::ffi::c_char;

// Supplied by the Linux kobject and errno dependencies.
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn kobject_create_and_add(
        name: *const c_char,
        parent: *mut kobject,
    ) -> *mut kobject;
}

// C: struct kobject *hypervisor_kobj;
#[no_mangle]
pub static mut hypervisor_kobj: *mut kobject = core::ptr::null_mut();

// C: EXPORT_SYMBOL_GPL(hypervisor_kobj);

// C: __init attribute is a linker/init-section annotation.
#[no_mangle]
pub unsafe extern "C" fn hypervisor_init() -> i32 {
    hypervisor_kobj = kobject_create_and_add(
        b"hypervisor\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
    );
    if hypervisor_kobj.is_null() {
        return -12; // -ENOMEM
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
