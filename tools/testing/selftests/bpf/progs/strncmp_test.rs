// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */

// C dependencies: <stdbool.h>, <linux/types.h>, <linux/bpf.h>,
// <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

use core::ffi::c_void;

const STRNCMP_STR_SZ: usize = 8;

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;
}

#[no_mangle]
pub static target: [i8; STRNCMP_STR_SZ] = [
    b'E' as i8,
    b'E' as i8,
    b'E' as i8,
    b'E' as i8,
    b'E' as i8,
    b'E' as i8,
    b'E' as i8,
    0,
];

#[no_mangle]
pub static mut str: [i8; STRNCMP_STR_SZ] = [0; STRNCMP_STR_SZ];

#[no_mangle]
pub static mut cmp_ret: i32 = 0;

#[no_mangle]
pub static mut target_pid: i32 = 0;

#[no_mangle]
pub static no_str_target: [i8; STRNCMP_STR_SZ] = [
    b'1' as i8,
    b'2' as i8,
    b'3' as i8,
    b'4' as i8,
    b'5' as i8,
    b'6' as i8,
    b'7' as i8,
    b'8' as i8,
];

#[no_mangle]
pub static mut writable_target: [i8; STRNCMP_STR_SZ] = [0; STRNCMP_STR_SZ];

#[no_mangle]
pub static mut no_const_str_size: u32 = STRNCMP_STR_SZ as u32;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

#[no_mangle]
#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn do_strncmp(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) as i32 != target_pid {
        return 0;
    }

    cmp_ret = bpf_strncmp(
        core::ptr::addr_of!(str).cast::<i8>(),
        STRNCMP_STR_SZ as u32,
        target.as_ptr(),
    );
    0
}

#[no_mangle]
#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn strncmp_bad_not_const_str_size(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* The value of string size is not const, so will fail */
    cmp_ret = bpf_strncmp(
        core::ptr::addr_of!(str).cast::<i8>(),
        no_const_str_size,
        target.as_ptr(),
    );
    0
}

#[no_mangle]
#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn strncmp_bad_writable_target(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* Compared target is not read-only, so will fail */
    cmp_ret = bpf_strncmp(
        core::ptr::addr_of!(str).cast::<i8>(),
        STRNCMP_STR_SZ as u32,
        core::ptr::addr_of!(writable_target).cast::<i8>(),
    );
    0
}

#[no_mangle]
#[link_section = "?tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn strncmp_bad_not_null_term_target(ctx: *mut c_void) -> i32 {
    let _ = ctx;

    /* Compared target is not null-terminated, so will fail */
    cmp_ret = bpf_strncmp(
        core::ptr::addr_of!(str).cast::<i8>(),
        STRNCMP_STR_SZ as u32,
        no_str_target.as_ptr(),
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
