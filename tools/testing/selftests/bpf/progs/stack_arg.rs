// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/* Translated from C. Original dependencies:
 * <vmlinux.h>, <stdbool.h>, <bpf/bpf_helpers.h>, "bpf_kfuncs.h"
 */

pub const CLOCK_MONOTONIC: i32 = 1;

#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_elem {
    pub timer: bpf_timer,
}

/* BPF map definition from C:
 * struct {
 *     __uint(type, BPF_MAP_TYPE_ARRAY);
 *     __uint(max_entries, 1);
 *     __type(key, int);
 *     __type(value, struct timer_elem);
 * } timer_map SEC(".maps");
 */
#[repr(C)]
pub struct timer_map {
    _private: [u8; 0],
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut timer_map: timer_map = timer_map { _private: [] };

#[unsafe(no_mangle)]
pub static mut timer_result: i32 = 0;

unsafe extern "C" {
    fn bpf_dynptr_size(ptr: *mut bpf_dynptr) -> i64;
    fn bpf_dynptr_from_skb(ctx: *mut core::ffi::c_void, flags: u64, ptr: *mut bpf_dynptr) -> i32;
    fn bpf_map_lookup_elem(map: *mut timer_map, key: *mut i32) -> *mut timer_elem;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut timer_map, clockid: i32) -> i32;
    fn bpf_timer_set_callback(
        timer: *mut bpf_timer,
        cb: unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut bpf_timer) -> i32,
    ) -> i32;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i32;
}

/* C condition:
 * #if (defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)) &&
 *     defined(__BPF_FEATURE_STACK_ARGUMENT)
 */

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub static HAS_STACK_ARG: bool = true;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub static has_stack_arg: bool = true;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
fn static_func_many_args(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32, j: i32) -> i32 {
    a + b + c + d + e + f + g + h + i + j
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn global_calls_many_args(a: i32, b: i32, c: i32) -> i32 {
    static_func_many_args(a, b, c, a + 3, a + 4, a + 5, a + 6, a + 7, a + 8, a + 9)
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_global_many_args() -> i32 {
    global_calls_many_args(1, 2, 3)
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct test_data {
    pub x: i64,
    pub y: i64,
}

/* 1+2+3+4+5+6+7+8+9+10+20 = 75 */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
unsafe fn func_with_ptr_stack_arg(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64, i: i64, p: *mut test_data) -> i64 {
    a + b + c + d + e + f + g + h + i + unsafe { (*p).x } + unsafe { (*p).y }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn global_ptr_stack_arg(a: i64, b: i64, c: i64, d: i64, e: i64) -> i64 {
    let mut data = test_data { x: 10, y: 20 };

    unsafe { func_with_ptr_stack_arg(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, &mut data) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_bpf2bpf_ptr_stack_arg() -> i32 {
    unsafe { global_ptr_stack_arg(1, 2, 3, 4, 5) as i32 }
}

/* 1+2+3+4+5+6+7+10+8+20 = 66 */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
unsafe fn func_with_mix_stack_args(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, p: *mut test_data, h: i64, q: *mut test_data) -> i64 {
    a + b + c + d + e + f + g + unsafe { (*p).x } + h + unsafe { (*q).y }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn global_mix_stack_args(a: i64, b: i64, c: i64, d: i64, e: i64) -> i64 {
    let mut p = test_data { x: 10, y: 0 };
    let mut q = test_data { x: 0, y: 20 };

    unsafe { func_with_mix_stack_args(a, b, c, d, e, e + 1, e + 2, &mut p, e + 3, &mut q) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_bpf2bpf_mix_stack_args() -> i32 {
    unsafe { global_mix_stack_args(1, 2, 3, 4, 5) as i32 }
}

/*
 * Nesting test: func_outer calls func_inner, both with struct pointer
 * as stack arg.
 *
 * func_inner: (a+1)+...+(i+1) + p->x + p->y
 *           = 2+3+4+5+6+7+8+9+10+10+20 = 84
 */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
unsafe fn func_inner_ptr(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64, i: i64, p: *mut test_data) -> i64 {
    a + b + c + d + e + f + g + h + i + unsafe { (*p).x } + unsafe { (*p).y }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
unsafe fn func_outer_ptr(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64, i: i64, p: *mut test_data) -> i64 {
    unsafe { func_inner_ptr(a + 1, b + 1, c + 1, d + 1, e + 1, f + 1, g + 1, h + 1, i + 1, p) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn global_nesting_ptr(a: i64, b: i64, c: i64, d: i64, e: i64) -> i64 {
    let mut data = test_data { x: 10, y: 20 };

    unsafe { func_outer_ptr(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, &mut data) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_bpf2bpf_nesting_stack_arg() -> i32 {
    unsafe { global_nesting_ptr(1, 2, 3, 4, 5) as i32 }
}

/* 1+2+3+4+5+6+7+8+9+sizeof(pkt_v4) = 45+54 = 99 */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
unsafe fn func_with_dynptr(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, g: i64, h: i64, i: i64, ptr: *mut bpf_dynptr) -> i64 {
    a + b + c + d + e + f + g + h + i + unsafe { bpf_dynptr_size(ptr) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn global_dynptr_stack_arg(ctx: *mut core::ffi::c_void, a: i64, b: i64, c: i64, d: i64) -> i64 {
    let mut ptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe { bpf_dynptr_from_skb(ctx, 0, ptr.as_mut_ptr()) };
    unsafe { func_with_dynptr(a, b, c, d, d + 1, d + 2, d + 3, d + 4, d + 5, ptr.as_mut_ptr()) }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_bpf2bpf_dynptr_stack_arg(skb: *mut __sk_buff) -> i32 {
    unsafe { global_dynptr_stack_arg(skb as *mut core::ffi::c_void, 1, 2, 3, 4) as i32 }
}

/* foo1: a+b+c+d+e+f+g+h+i+j */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
fn foo1(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32, j: i32) -> i32 {
    a + b + c + d + e + f + g + h + i + j
}

/* foo2: a+b+c+d+e+f+g+h+i+j+k+l */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(never)]
fn foo2(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32, i: i32, j: i32, k: i32, l: i32) -> i32 {
    a + b + c + d + e + f + g + h + i + j + k + l
}

/* global_two_callees calls foo1 (5 stack args) and foo2 (7 stack args).
 * The outgoing stack arg area is sized for foo2 (the larger callee).
 * Stores for foo1 are a subset of the area used by foo2.
 * Result: foo1(1..10) + foo2(1..12) = 55 + 78 = 133
 *
 * Pass a-e through so the compiler can't constant-fold the stack args away.
 */
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn global_two_callees(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut ret: i32;

    ret = foo1(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, a + 9);
    ret += foo2(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, a + 9, a + 10, a + 11);
    ret
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_two_callees() -> i32 {
    global_two_callees(1, 2, 3, 4, 5)
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub static timer_base: i32 = 10;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn timer_cb_many_args(_map: *mut core::ffi::c_void, _key: *mut i32, _timer: *mut bpf_timer) -> i32 {
    let v = timer_base;

    unsafe {
        timer_result = static_func_many_args(v, v * 2, v * 3, v * 4, v * 5, v * 6, v * 7, v * 8, v * 9, v * 10);
    }
    0
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub unsafe extern "C" fn test_async_cb_many_args() -> i32 {
    let mut elem: *mut timer_elem;
    let mut key: i32 = 0;

    elem = unsafe { bpf_map_lookup_elem(&raw mut timer_map, &mut key) };
    if elem.is_null() {
        return -1;
    }

    unsafe {
        bpf_timer_init(&mut (*elem).timer, &raw mut timer_map, CLOCK_MONOTONIC);
        bpf_timer_set_callback(&mut (*elem).timer, timer_cb_many_args);
        bpf_timer_start(&mut (*elem).timer, 1, 0);
    }
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
pub static has_stack_arg: bool = false;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_global_many_args() -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_bpf2bpf_ptr_stack_arg() -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_bpf2bpf_mix_stack_args() -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_bpf2bpf_nesting_stack_arg() -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_bpf2bpf_dynptr_stack_arg(_skb: *mut __sk_buff) -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_two_callees() -> i32 {
    0
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
#[unsafe(link_section = "tc")]
pub extern "C" fn test_async_cb_many_args() -> i32 {
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
