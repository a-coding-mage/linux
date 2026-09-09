/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

/* C compiler-specific macros and build-time attributes have no direct Rust
 * equivalent; their conditional intent is preserved here. */

use core::ffi::{c_char, c_void};
use core::mem::align_of;

pub const CACHELINE_SIZE: usize = 64;

/* INLINE_KEYWORD, FORCE_INLINE_ATTR, WIN_CDECL, UNUSED_ATTR,
 * FORCE_INLINE_TEMPLATE, HINT_INLINE, MEM_STATIC, FORCE_NOINLINE,
 * TARGET_ATTRIBUTE, BMI2_TARGET_ATTRIBUTE, DONT_VECTORIZE, and
 * ZSTD_FALLTHROUGH are C compiler attributes/macros with no direct Rust
 * equivalent. */

#[inline]
pub unsafe fn PREFETCH_L1<T>(ptr_: *const T) {
    let _ = ptr_;
}

#[inline]
pub unsafe fn PREFETCH_L2<T>(ptr_: *const T) {
    let _ = ptr_;
}

#[inline]
pub unsafe fn PREFETCH_AREA(p: *const c_void, s: usize) {
    let ptr_ = p as *const c_char;
    let mut pos: usize = 0;
    while pos < s {
        PREFETCH_L2(ptr_.add(pos));
        pos = pos.wrapping_add(CACHELINE_SIZE);
    }
}

#[inline]
pub const fn LIKELY<T>(x: T) -> T { x }

#[inline]
pub const fn UNLIKELY<T>(x: T) -> T { x }

#[inline]
pub fn ZSTD_UNREACHABLE() -> ! {
    unreachable!()
}

/* ZSTD_HAS_C_ATTRIBUTE and ZSTD_HAS_CPP_ATTRIBUTE are build-time feature
 * checks. ZSTD_ALIGNOF and ZSTD_ALIGNED map to Rust's alignment facilities. */
pub const fn ZSTD_ALIGNOF<T>() -> usize { align_of::<T>() }

/* @return 1 if @u is a 2^n value, 0 otherwise */
#[inline]
pub fn ZSTD_isPower2(u: usize) -> i32 {
    ((u & u.wrapping_sub(1)) == 0) as i32
}

/*
 * Zstd relies on pointer overflow in its decompressor. These helpers preserve
 * the wrapping pointer operations used by the C implementation.
 */
#[inline]
pub unsafe fn ZSTD_wrappedPtrDiff(lhs: *const u8, rhs: *const u8) -> isize {
    (lhs as usize).wrapping_sub(rhs as usize) as isize
}

#[inline]
pub unsafe fn ZSTD_wrappedPtrAdd(ptr_: *const u8, add: isize) -> *const u8 {
    (ptr_ as usize).wrapping_add(add as usize) as *const u8
}

#[inline]
pub unsafe fn ZSTD_wrappedPtrSub(ptr_: *const u8, sub: isize) -> *const u8 {
    (ptr_ as usize).wrapping_sub(sub as usize) as *const u8
}

/* Defines NULL + 0 == NULL, unlike C pointer arithmetic. */
#[inline]
pub unsafe fn ZSTD_maybeNullPtrAdd(ptr_: *mut u8, add: isize) -> *mut u8 {
    if add > 0 { ptr_.add(add as usize) } else { ptr_ }
}

/* __MINGW32__ sets these sanitizer workspace-poisoning controls to 1. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
