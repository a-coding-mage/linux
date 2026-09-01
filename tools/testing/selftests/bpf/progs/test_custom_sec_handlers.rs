// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Facebook */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

use core::ffi::{c_int, c_long, c_void};
use core::ptr;

#[no_mangle]
pub static my_pid: c_int = 0;

#[no_mangle]
pub static mut abc1_called: bool = false;
#[no_mangle]
pub static mut abc2_called: bool = false;
#[no_mangle]
pub static mut custom1_called: bool = false;
#[no_mangle]
pub static mut custom2_called: bool = false;
#[no_mangle]
pub static mut kprobe1_called: bool = false;
#[no_mangle]
pub static mut xyz_called: bool = false;

extern "C" {
    fn bpf_copy_from_user(dst: *mut c_void, size: usize, src: *const c_void) -> c_long;
}

#[no_mangle]
#[link_section = "abc"]
pub unsafe extern "C" fn abc1(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    abc1_called = true;
    0
}

#[no_mangle]
#[link_section = "abc/whatever"]
pub unsafe extern "C" fn abc2(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    abc2_called = true;
    0
}

#[no_mangle]
#[link_section = "custom"]
pub unsafe extern "C" fn custom1(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    custom1_called = true;
    0
}

#[no_mangle]
#[link_section = "custom/something"]
pub unsafe extern "C" fn custom2(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    custom2_called = true;
    0
}

#[no_mangle]
#[link_section = "kprobe"]
pub unsafe extern "C" fn kprobe1(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    kprobe1_called = true;
    0
}

#[no_mangle]
#[link_section = "xyz/blah"]
pub unsafe extern "C" fn xyz(ctx: *mut c_void) -> c_int {
    let _ = ctx;
    let mut whatever: c_int = 0;

    /* use sleepable helper, custom handler should set sleepable flag */
    bpf_copy_from_user(
        &mut whatever as *mut c_int as *mut c_void,
        core::mem::size_of_val(&whatever),
        ptr::null(),
    );
    xyz_called = true;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
