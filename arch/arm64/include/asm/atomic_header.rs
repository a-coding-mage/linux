/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/atomic.h
 *
 * Copyright (C) 1996 Russell King.
 * Copyright (C) 2002 Deep Blue Solutions Ltd.
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/types.h, asm/barrier.h, asm/cmpxchg.h, asm/lse.h

extern "C" {
    fn __lse_ll_sc_body_atomic_andnot(i: core::ffi::c_int, v: *mut atomic_t);
    fn __lse_ll_sc_body_atomic_or(i: core::ffi::c_int, v: *mut atomic_t);
    fn __lse_ll_sc_body_atomic_xor(i: core::ffi::c_int, v: *mut atomic_t);
    fn __lse_ll_sc_body_atomic_add(i: core::ffi::c_int, v: *mut atomic_t);
    fn __lse_ll_sc_body_atomic_and(i: core::ffi::c_int, v: *mut atomic_t);
    fn __lse_ll_sc_body_atomic_sub(i: core::ffi::c_int, v: *mut atomic_t);

    fn __lse_ll_sc_body_atomic64_andnot(i: core::ffi::c_long, v: *mut atomic64_t);
    fn __lse_ll_sc_body_atomic64_or(i: core::ffi::c_long, v: *mut atomic64_t);
    fn __lse_ll_sc_body_atomic64_xor(i: core::ffi::c_long, v: *mut atomic64_t);
    fn __lse_ll_sc_body_atomic64_add(i: core::ffi::c_long, v: *mut atomic64_t);
    fn __lse_ll_sc_body_atomic64_and(i: core::ffi::c_long, v: *mut atomic64_t);
    fn __lse_ll_sc_body_atomic64_sub(i: core::ffi::c_long, v: *mut atomic64_t);
}

// The C ATOMIC_OP macro generates these architecture operations.
#[inline(always)] pub unsafe fn arch_atomic_andnot(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_andnot(i, v) }
#[inline(always)] pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_or(i, v) }
#[inline(always)] pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_xor(i, v) }
#[inline(always)] pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_add(i, v) }
#[inline(always)] pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_and(i, v) }
#[inline(always)] pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) { __lse_ll_sc_body_atomic_sub(i, v) }

macro_rules! atomic_fetch_ops {
    ($ty:ty, $atomic:ty, $($name:ident => $helper:ident),+ $(,)?) => {
        extern "C" { $(fn $helper(i: $ty, v: *mut $atomic) -> $ty;)+ }
        $(#[inline(always)] pub unsafe fn $name(i: $ty, v: *mut $atomic) -> $ty { $helper(i, v) })+
    };
}

atomic_fetch_ops!(i32, atomic_t,
    arch_atomic_fetch_andnot_relaxed => __lse_ll_sc_body_atomic_fetch_andnot_relaxed,
    arch_atomic_fetch_andnot_acquire => __lse_ll_sc_body_atomic_fetch_andnot_acquire,
    arch_atomic_fetch_andnot_release => __lse_ll_sc_body_atomic_fetch_andnot_release,
    arch_atomic_fetch_andnot => __lse_ll_sc_body_atomic_fetch_andnot,
    arch_atomic_fetch_or_relaxed => __lse_ll_sc_body_atomic_fetch_or_relaxed,
    arch_atomic_fetch_or_acquire => __lse_ll_sc_body_atomic_fetch_or_acquire,
    arch_atomic_fetch_or_release => __lse_ll_sc_body_atomic_fetch_or_release,
    arch_atomic_fetch_or => __lse_ll_sc_body_atomic_fetch_or,
    arch_atomic_fetch_xor_relaxed => __lse_ll_sc_body_atomic_fetch_xor_relaxed,
    arch_atomic_fetch_xor_acquire => __lse_ll_sc_body_atomic_fetch_xor_acquire,
    arch_atomic_fetch_xor_release => __lse_ll_sc_body_atomic_fetch_xor_release,
    arch_atomic_fetch_xor => __lse_ll_sc_body_atomic_fetch_xor,
    arch_atomic_fetch_add_relaxed => __lse_ll_sc_body_atomic_fetch_add_relaxed,
    arch_atomic_fetch_add_acquire => __lse_ll_sc_body_atomic_fetch_add_acquire,
    arch_atomic_fetch_add_release => __lse_ll_sc_body_atomic_fetch_add_release,
    arch_atomic_fetch_add => __lse_ll_sc_body_atomic_fetch_add,
    arch_atomic_fetch_and_relaxed => __lse_ll_sc_body_atomic_fetch_and_relaxed,
    arch_atomic_fetch_and_acquire => __lse_ll_sc_body_atomic_fetch_and_acquire,
    arch_atomic_fetch_and_release => __lse_ll_sc_body_atomic_fetch_and_release,
    arch_atomic_fetch_and => __lse_ll_sc_body_atomic_fetch_and,
    arch_atomic_fetch_sub_relaxed => __lse_ll_sc_body_atomic_fetch_sub_relaxed,
    arch_atomic_fetch_sub_acquire => __lse_ll_sc_body_atomic_fetch_sub_acquire,
    arch_atomic_fetch_sub_release => __lse_ll_sc_body_atomic_fetch_sub_release,
    arch_atomic_fetch_sub => __lse_ll_sc_body_atomic_fetch_sub,
    arch_atomic_add_return_relaxed => __lse_ll_sc_body_atomic_add_return_relaxed,
    arch_atomic_add_return_acquire => __lse_ll_sc_body_atomic_add_return_acquire,
    arch_atomic_add_return_release => __lse_ll_sc_body_atomic_add_return_release,
    arch_atomic_add_return => __lse_ll_sc_body_atomic_add_return,
    arch_atomic_sub_return_relaxed => __lse_ll_sc_body_atomic_sub_return_relaxed,
    arch_atomic_sub_return_acquire => __lse_ll_sc_body_atomic_sub_return_acquire,
    arch_atomic_sub_return_release => __lse_ll_sc_body_atomic_sub_return_release,
    arch_atomic_sub_return => __lse_ll_sc_body_atomic_sub_return,
);

atomic_fetch_ops!(i64, atomic64_t,
    arch_atomic64_fetch_andnot_relaxed => __lse_ll_sc_body_atomic64_fetch_andnot_relaxed,
    arch_atomic64_fetch_andnot_acquire => __lse_ll_sc_body_atomic64_fetch_andnot_acquire,
    arch_atomic64_fetch_andnot_release => __lse_ll_sc_body_atomic64_fetch_andnot_release,
    arch_atomic64_fetch_andnot => __lse_ll_sc_body_atomic64_fetch_andnot,
    arch_atomic64_fetch_or_relaxed => __lse_ll_sc_body_atomic64_fetch_or_relaxed,
    arch_atomic64_fetch_or_acquire => __lse_ll_sc_body_atomic64_fetch_or_acquire,
    arch_atomic64_fetch_or_release => __lse_ll_sc_body_atomic64_fetch_or_release,
    arch_atomic64_fetch_or => __lse_ll_sc_body_atomic64_fetch_or,
    arch_atomic64_fetch_xor_relaxed => __lse_ll_sc_body_atomic64_fetch_xor_relaxed,
    arch_atomic64_fetch_xor_acquire => __lse_ll_sc_body_atomic64_fetch_xor_acquire,
    arch_atomic64_fetch_xor_release => __lse_ll_sc_body_atomic64_fetch_xor_release,
    arch_atomic64_fetch_xor => __lse_ll_sc_body_atomic64_fetch_xor,
    arch_atomic64_fetch_add_relaxed => __lse_ll_sc_body_atomic64_fetch_add_relaxed,
    arch_atomic64_fetch_add_acquire => __lse_ll_sc_body_atomic64_fetch_add_acquire,
    arch_atomic64_fetch_add_release => __lse_ll_sc_body_atomic64_fetch_add_release,
    arch_atomic64_fetch_add => __lse_ll_sc_body_atomic64_fetch_add,
    arch_atomic64_fetch_and_relaxed => __lse_ll_sc_body_atomic64_fetch_and_relaxed,
    arch_atomic64_fetch_and_acquire => __lse_ll_sc_body_atomic64_fetch_and_acquire,
    arch_atomic64_fetch_and_release => __lse_ll_sc_body_atomic64_fetch_and_release,
    arch_atomic64_fetch_and => __lse_ll_sc_body_atomic64_fetch_and,
    arch_atomic64_fetch_sub_relaxed => __lse_ll_sc_body_atomic64_fetch_sub_relaxed,
    arch_atomic64_fetch_sub_acquire => __lse_ll_sc_body_atomic64_fetch_sub_acquire,
    arch_atomic64_fetch_sub_release => __lse_ll_sc_body_atomic64_fetch_sub_release,
    arch_atomic64_fetch_sub => __lse_ll_sc_body_atomic64_fetch_sub,
    arch_atomic64_add_return_relaxed => __lse_ll_sc_body_atomic64_add_return_relaxed,
    arch_atomic64_add_return_acquire => __lse_ll_sc_body_atomic64_add_return_acquire,
    arch_atomic64_add_return_release => __lse_ll_sc_body_atomic64_add_return_release,
    arch_atomic64_add_return => __lse_ll_sc_body_atomic64_add_return,
    arch_atomic64_sub_return_relaxed => __lse_ll_sc_body_atomic64_sub_return_relaxed,
    arch_atomic64_sub_return_acquire => __lse_ll_sc_body_atomic64_sub_return_acquire,
    arch_atomic64_sub_return_release => __lse_ll_sc_body_atomic64_sub_return_release,
    arch_atomic64_sub_return => __lse_ll_sc_body_atomic64_sub_return,
);

// 64-bit arch_atomic operations.
#[inline(always)] pub unsafe fn arch_atomic64_andnot(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_andnot(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_or(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_or(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_xor(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_xor(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_add(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_add(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_and(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_and(i, v) }
#[inline(always)] pub unsafe fn arch_atomic64_sub(i: i64, v: *mut atomic64_t) { __lse_ll_sc_body_atomic64_sub(i, v) }

pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64 {
    __lse_ll_sc_body_atomic64_dec_if_positive(v)
}

// The following C self-referential macros preserve architecture aliases.
// Rust callers use the corresponding functions directly:
// arch_atomic_read, arch_atomic_set, arch_atomic*_return_*,
// arch_atomic*_fetch_*, arch_atomic64_dec_if_positive.

// External types and the lse helper are supplied by the translated dependencies.
extern "C" {
    fn __lse_ll_sc_body_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
