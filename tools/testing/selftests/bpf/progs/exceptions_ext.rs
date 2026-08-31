// SPDX-License-Identifier: GPL-2.0
// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_experimental.h"

use core::ffi::c_void;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_throw(cookie: u64);
}

#[unsafe(link_section = "?fentry")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pfentry(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "?fentry")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_fentry(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_throw(0);
    }
    return 0;
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exception_cb(cookie: u64) -> i32 {
    return cookie.wrapping_add(64) as i32;
}

#[unsafe(link_section = "?freplace")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extension(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "?freplace")]
// C annotation: __exception_cb(exception_cb)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_exception_cb_extension(cookie: u64) -> i32 {
    unsafe {
        bpf_throw(32);
    }
    let _ = cookie;
    return 0;
}

#[unsafe(link_section = "?freplace")]
// C annotation: __exception_cb(exception_cb)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_extension(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_throw(64);
    }
    return 0;
}

#[unsafe(link_section = "?fexit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pfexit(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "?fexit")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_fexit(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_throw(0);
    }
    return 0;
}

#[unsafe(link_section = "?fmod_ret")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pfmod_ret(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    return 0;
}

#[unsafe(link_section = "?fmod_ret")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn throwing_fmod_ret(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_throw(0);
    }
    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
