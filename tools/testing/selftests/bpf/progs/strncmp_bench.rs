// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2021. Huawei Technologies Co., Ltd */
/* Dependencies in the original C source:
 * #include <linux/types.h>
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 */

use core::ffi::{c_char, c_void};

const STRNCMP_STR_SZ: usize = 4096;

/* Will be updated by benchmark before program loading */
#[no_mangle]
pub static mut cmp_str_len: u32 = 1;

#[no_mangle]
pub static target: [c_char; STRNCMP_STR_SZ] = [0; STRNCMP_STR_SZ];

#[no_mangle]
pub static mut hits: i64 = 0;

#[no_mangle]
pub static mut str: [c_char; STRNCMP_STR_SZ] = [0; STRNCMP_STR_SZ];

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

extern "C" {
    fn bpf_strncmp(s1: *const c_char, sz: u32, s2: *const c_char) -> i32;
}

#[inline(always)]
unsafe fn local_strncmp(s1: *const c_char, sz: u32, s2: *const c_char) -> i32 {
    let mut ret: i32 = 0;
    let mut i: u32;

    i = 0;
    while i < sz {
        /* E.g. 0xff > 0x31 */
        let c1 = *s1.add(i as usize) as u8;
        let c2 = *s2.add(i as usize) as u8;
        ret = c1 as i32 - c2 as i32;
        if ret != 0 || *s1.add(i as usize) == 0 {
            break;
        }
        i += 1;
    }

    ret
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getpgid"]
pub unsafe extern "C" fn strncmp_no_helper(ctx: *mut c_void) -> i32 {
    let mut target_str: *const c_char = target.as_ptr();

    core::arch::asm!("", inout(reg) target_str, options(nostack, preserves_flags));
    if local_strncmp(
        str.as_ptr(),
        core::ptr::read_volatile(core::ptr::addr_of!(cmp_str_len)).wrapping_add(1),
        target_str,
    ) < 0
    {
        core::intrinsics::atomic_xadd_relaxed(core::ptr::addr_of_mut!(hits), 1);
    }
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getpgid"]
pub unsafe extern "C" fn strncmp_helper(ctx: *mut c_void) -> i32 {
    if bpf_strncmp(
        str.as_ptr(),
        core::ptr::read_volatile(core::ptr::addr_of!(cmp_str_len)).wrapping_add(1),
        target.as_ptr(),
    ) < 0
    {
        core::intrinsics::atomic_xadd_relaxed(core::ptr::addr_of_mut!(hits), 1);
    }
    let _ = ctx;
    0
}
