/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Define struct scx_enums that stores the load-time values of enums
 * used by the BPF program.
 *
 * Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
 */

// C header guard removed in Rust.

pub unsafe fn __ENUM_set(val: *mut u64, type_: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) {
    let res: bool;

    res = unsafe { __COMPAT_read_enum(type_, name, val) };
    if !res {
        unsafe {
            *val = 0;
        }
    }
}

// Translation of:
// #define SCX_ENUM_SET(skel, type, name) do { \
//     __ENUM_set(&skel->rodata->__##name, #type, #name); \
// } while (0)
//
// The original C macro depends on token pasting and stringification. Preserve
// the call shape with explicit field, type string, and name string operands.
#[macro_export]
macro_rules! SCX_ENUM_SET {
    ($skel:expr, $field:ident, $type_name:expr, $name:expr) => {{
        unsafe {
            __ENUM_set(
                ::core::ptr::addr_of_mut!((*(*$skel).rodata).__$field),
                concat!($type_name, "\0").as_ptr() as *const ::core::ffi::c_char,
                concat!($name, "\0").as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }};
}

unsafe extern "C" {
    fn __COMPAT_read_enum(
        type_: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        val: *mut u64,
    ) -> bool;
}

// Dependency intent from C include:
// #include "enums.autogen.h"
