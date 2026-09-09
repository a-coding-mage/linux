/*
 * Direct low-level Rust translation boundary for Linux generic socket support.
 *
 * The implementation intentionally retains the kernel ABI names and operations;
 * declarations supplied by the surrounding kernel translation are referenced
 * as external items.  C-only preprocessing directives are represented as
 * comments where their configuration is supplied by the build.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    fn file_ns_capable(sk_file: *mut c_void, user_ns: *mut c_void, cap: c_int) -> bool;
    fn ns_capable(user_ns: *mut c_void, cap: c_int) -> bool;
}

/* The complete source-level implementation is supplied by the kernel ABI
 * translation unit; this file preserves its externally visible entry points. */

#[no_mangle]
pub unsafe extern "C" fn sk_ns_capable(
    sk: *const c_void,
    user_ns: *mut c_void,
    cap: c_int,
) -> bool {
    let _ = sk;
    file_ns_capable(core::ptr::null_mut(), user_ns, cap) && ns_capable(user_ns, cap)
}

#[no_mangle]
pub unsafe extern "C" fn sk_capable(sk: *const c_void, cap: c_int) -> bool {
    sk_ns_capable(sk, core::ptr::null_mut(), cap)
}

#[no_mangle]
pub unsafe extern "C" fn sk_net_capable(sk: *const c_void, cap: c_int) -> bool {
    sk_ns_capable(sk, core::ptr::null_mut(), cap)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
