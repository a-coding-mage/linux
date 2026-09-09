// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

use core::ffi::c_char;

// Supplied by the Linux device-tree interfaces.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut of_root: *mut device_node;

    pub fn of_property_read_string(
        np: *const device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> i32;

    pub fn of_property_read_string_index(
        np: *const device_node,
        propname: *const c_char,
        index: usize,
        out_string: *mut *const c_char,
    ) -> i32;
}

#[no_mangle]
pub static mut system_type: *mut c_char = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn get_system_type() -> *const c_char {
    let mut string: *const c_char;
    let mut err: i32;

    if !system_type.is_null() {
        return system_type as *const c_char;
    }

    err = of_property_read_string(
        of_root,
        b"model\0".as_ptr() as *const c_char,
        &mut string,
    );
    if err == 0 {
        return string;
    }

    err = of_property_read_string_index(
        of_root,
        b"compatible\0".as_ptr() as *const c_char,
        0,
        &mut string,
    );
    if err == 0 {
        return string;
    }

    b"Unknown\0".as_ptr() as *const c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
