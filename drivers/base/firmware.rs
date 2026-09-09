// SPDX-License-Identifier: GPL-2.0
/*
 * firmware.rs - firmware subsystem hoohaw.
 *
 * Copyright (c) 2002-3 Patrick Mochel
 * Copyright (c) 2002-3 Open Source Development Labs
 * Copyright (c) 2007 Greg Kroah-Hartman <gregkh@suse.de>
 * Copyright (c) 2007 Novell Inc.
 */

// Declarations supplied by the Linux kernel and base subsystem.
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn kobject_create_and_add(name: *const u8, parent: *mut kobject) -> *mut kobject;
}

pub static mut firmware_kobj: *mut kobject = core::ptr::null_mut();

// EXPORT_SYMBOL_GPL(firmware_kobj);

// __init
pub unsafe extern "C" fn firmware_init() -> i32 {
    firmware_kobj = kobject_create_and_add(b"firmware\0".as_ptr(), core::ptr::null_mut());
    if firmware_kobj.is_null() {
        return -12;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
