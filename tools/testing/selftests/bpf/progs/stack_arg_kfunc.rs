// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* C dependencies:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_kfuncs.h"
 * #include "../test_kmods/bpf_testmod_kfunc.h"
 */

/* Original C condition:
 * #if (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) && \
 *      defined(__BPF_FEATURE_STACK_ARGUMENT)
 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
pub const HAS_STACK_ARG_CFG_INTENT: bool = true;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[no_mangle]
pub static mut has_stack_arg: bool = true;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct bpf_iter_testmod_seq {
    pub __bindgen_padding_0: u64,
    pub __bindgen_padding_1: u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
extern "C" {
    pub fn bpf_iter_testmod_seq_new(it: *mut bpf_iter_testmod_seq, value: i64, cnt: i32) -> i32;
    pub fn bpf_iter_testmod_seq_destroy(it: *mut bpf_iter_testmod_seq);

    pub fn bpf_kfunc_call_stack_arg(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        arg10: i32,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_ptr(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        p: *mut prog_test_pass1,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_mix(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        p: *mut prog_test_pass1,
        arg8: i32,
        q: *mut prog_test_pass1,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_dynptr(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        ptr: *mut bpf_dynptr,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_mem(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        buf: *mut i8,
        len: usize,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_iter(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        it: *mut bpf_iter_testmod_seq,
    ) -> u64;
    pub fn bpf_kfunc_call_stack_arg_const_str(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        cstr: *const i8,
    ) -> i32;
    pub fn bpf_kfunc_call_stack_arg_timer(
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
        arg7: i32,
        arg8: i32,
        arg9: i32,
        timer: *mut bpf_timer,
    ) -> i32;
    pub fn bpf_dynptr_from_skb(skb: *mut __sk_buff, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct timer_map_value {
    pub timer: bpf_timer,
}

/* C map declaration:
 * struct {
 *      __uint(type, BPF_MAP_TYPE_ARRAY);
 *      __uint(max_entries, 1);
 *      __type(key, int);
 *      __type(value, struct timer_map_value);
 * } kfunc_timer_map SEC(".maps");
 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[no_mangle]
#[link_section = ".maps"]
pub static mut kfunc_timer_map: core::mem::MaybeUninit<core::ffi::c_void> =
    core::mem::MaybeUninit::uninit();

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_scalar(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    bpf_kfunc_call_stack_arg(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_ptr(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut p = prog_test_pass1 { x0: 10, x1: 20 };

    bpf_kfunc_call_stack_arg_ptr(1, 2, 3, 4, 5, 6, 7, 8, 9, &mut p)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mix(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut p = prog_test_pass1 { x0: 10, ..core::mem::zeroed() };
    let mut q = prog_test_pass1 { x1: 20, ..core::mem::zeroed() };

    bpf_kfunc_call_stack_arg_mix(1, 2, 3, 4, 5, 6, 7, &mut p, 8, &mut q)
}

/* 1+2+3+4+5+6+7+8+9+sizeof(pkt_v4) = 45+54 = 99 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_dynptr(skb: *mut __sk_buff) -> i32 {
    let mut ptr: bpf_dynptr = core::mem::zeroed();

    bpf_dynptr_from_skb(skb, 0, &mut ptr);
    bpf_kfunc_call_stack_arg_dynptr(1, 2, 3, 4, 5, 6, 7, 8, 9, &mut ptr)
}

/* 1 + 2 + 3 + 4 + 5 + (1 + 2 + ... + 16) = 15 + 136 = 151 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mem(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut buf: [i8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    bpf_kfunc_call_stack_arg_mem(1, 2, 3, 4, 5, buf.as_mut_ptr(), core::mem::size_of_val(&buf))
}

/* 1+2+3+4+5+6+7+8+9+100 = 145 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_iter(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut it: bpf_iter_testmod_seq = core::mem::zeroed();
    let ret: u64;

    bpf_iter_testmod_seq_new(&mut it, 100, 10);
    ret = bpf_kfunc_call_stack_arg_iter(1, 2, 3, 4, 5, 6, 7, 8, 9, &mut it);
    bpf_iter_testmod_seq_destroy(&mut it);
    ret as i32
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[no_mangle]
pub static cstr: [i8; 6] = [b'h' as i8, b'e' as i8, b'l' as i8, b'l' as i8, b'o' as i8, 0];

/* 1+2+3+4+5+6+7+8+9 = 45 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_const_str(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    bpf_kfunc_call_stack_arg_const_str(1, 2, 3, 4, 5, 6, 7, 8, 9, cstr.as_ptr())
}

/* 1+2+3+4+5+6+7+8+9 = 45 */
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_timer(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut val: *mut timer_map_value;
    let mut key: i32 = 0;

    val = bpf_map_lookup_elem(
        &mut kfunc_timer_map as *mut _ as *mut core::ffi::c_void,
        &mut key as *mut _ as *const core::ffi::c_void,
    ) as *mut timer_map_value;
    if val.is_null() {
        return 0;
    }
    bpf_kfunc_call_stack_arg_timer(1, 2, 3, 4, 5, 6, 7, 8, 9, &mut (*val).timer)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[no_mangle]
pub static mut has_stack_arg: bool = false;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_scalar(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_ptr(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mix(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_dynptr(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mem(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_iter(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_const_str(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_timer(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
