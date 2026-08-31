// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/*
 * Copyright (c) 2025-2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2025-2026 Emil Tsalapatis <emil@etsalapatis.com>
 */

// Translated from C implementation source.
// Original dependencies:
//   #include <libarena/common.h>
//   #include <libarena/asan.h>
//   #include <libarena/bitmap.h>
// C attributes/qualifiers such as __weak, __arena, and __always_inline are
// represented here with exported unsafe extern functions and comments.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_void};
use core::ptr;

pub type size_t = usize;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;

pub const BITS_PER_LONG_LONG: size_t = 64;

#[repr(C)]
pub struct arena_bitmap {
    pub bits: [u64; 0],
}

extern "C" {
    static zero: u32;
    static can_loop: bool;

    fn arena_malloc(size: size_t) -> *mut c_void;
    fn arena_free(ptr: *mut c_void);
    fn arena_stderr(fmt: *const c_char, ...);
    fn cmpxchg(ptr: *mut u64, old: u64, new: u64) -> u64;
}

#[inline(always)]
const fn BITS_TO_LONG_LONGS(bits: size_t) -> size_t {
    (bits + BITS_PER_LONG_LONG - 1) / BITS_PER_LONG_LONG
}

#[inline(always)]
const fn BIT_WORD(bit: u32) -> size_t {
    (bit as size_t) / BITS_PER_LONG_LONG
}

#[inline(always)]
const fn BIT_MASK(bit: u32) -> u64 {
    1u64 << ((bit as size_t) % BITS_PER_LONG_LONG)
}

#[inline(always)]
unsafe fn bmp_bits(bmp: *mut arena_bitmap) -> *mut u64 {
    ptr::addr_of_mut!((*bmp).bits) as *mut u64
}

#[inline(always)]
unsafe fn bmp_bits_const(bmp: *mut arena_bitmap) -> *const u64 {
    ptr::addr_of!((*bmp).bits) as *const u64
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_alloc(bits: size_t) -> *mut arena_bitmap {
    let mut bmp: *mut arena_bitmap;
    let size: size_t = BITS_TO_LONG_LONGS(bits) * core::mem::size_of::<u64>();

    /* Assume long-aligned masks. */
    if bits % BITS_PER_LONG_LONG != 0 {
        return ptr::null_mut();
    }

    bmp = arena_malloc(size) as *mut arena_bitmap;
    if bmp.is_null() {
        return ptr::null_mut();
    }

    bmp_clear(bits, bmp);

    bmp
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_free(bmp: *mut arena_bitmap) {
    arena_free(bmp as *mut c_void);
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn __bmp_set_bit(bit: u32, bmp: *mut arena_bitmap) {
    let p = bmp_bits(bmp).add(BIT_WORD(bit));
    *p |= BIT_MASK(bit);
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn __bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap) {
    let p = bmp_bits(bmp).add(BIT_WORD(bit));
    *p &= !BIT_MASK(bit);
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_bit(bit: u32, bmp: *mut arena_bitmap) -> bool {
    (*bmp_bits_const(bmp).add(BIT_WORD(bit)) & BIT_MASK(bit)) != 0
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_and_clear_bit(bit: u32, bmp: *mut arena_bitmap) -> bool {
    let val: u64 = BIT_MASK(bit);
    let idx: u32 = BIT_WORD(bit) as u32;
    let mut old: u64;
    let new: u64;
    let mut actual: u64;

    loop {
        old = *bmp_bits_const(bmp).add(idx as size_t);

        if (old & val) == 0 {
            return false;
        }

        new = old & !val;
        actual = cmpxchg(bmp_bits(bmp).add(idx as size_t), old, new);

        if actual == old {
            return true;
        }

        if !can_loop {
            break;
        }
    }

    false
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_test_and_set_bit(bit: u32, bmp: *mut arena_bitmap) -> bool {
    let val: u64 = BIT_MASK(bit);
    let idx: u32 = BIT_WORD(bit) as u32;
    let mut old: u64;
    let new: u64;
    let mut actual: u64;

    loop {
        old = *bmp_bits_const(bmp).add(idx as size_t);

        if (old & val) != 0 {
            return true;
        }

        new = old | val;
        actual = cmpxchg(bmp_bits(bmp).add(idx as size_t), old, new);

        if actual == old {
            return false;
        }

        if !can_loop {
            break;
        }
    }

    false
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_clear_bit(bit: u32, bmp: *mut arena_bitmap) {
    let val: u64 = BIT_MASK(bit);
    let idx: u32 = BIT_WORD(bit) as u32;
    let mut old: u64;
    let new: u64;
    let mut actual: u64;

    loop {
        old = *bmp_bits_const(bmp).add(idx as size_t);
        new = old & !val;
        actual = cmpxchg(bmp_bits(bmp).add(idx as size_t), old, new);

        if !(actual != old && can_loop) {
            break;
        }
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_set_bit(bit: u32, bmp: *mut arena_bitmap) {
    let val: u64 = BIT_MASK(bit);
    let idx: u32 = BIT_WORD(bit) as u32;
    let mut old: u64;
    let new: u64;
    let mut actual: u64;

    loop {
        old = *bmp_bits_const(bmp).add(idx as size_t);
        new = old | val;
        actual = cmpxchg(bmp_bits(bmp).add(idx as size_t), old, new);

        if !(actual != old && can_loop) {
            break;
        }
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_clear(bits: size_t, bmp: *mut arena_bitmap) {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        *bmp_bits(bmp).add(i as size_t) = 0;
        i = i.wrapping_add(1);
    }
}

// static __always_inline
#[inline(always)]
unsafe fn bmp_last_word_mask(bits: size_t) -> u64 {
    let rem: u32 = (bits % BITS_PER_LONG_LONG) as u32;

    if rem != 0 {
        (1u64 << rem) - 1
    } else {
        !0u64
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_and(
    bits: size_t,
    dst: *mut arena_bitmap,
    src1: *mut arena_bitmap,
    src2: *mut arena_bitmap,
) {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        *bmp_bits(dst).add(i as size_t) =
            *bmp_bits_const(src1).add(i as size_t) & *bmp_bits_const(src2).add(i as size_t);
        i = i.wrapping_add(1);
    }

    if nwords != 0 && bits % BITS_PER_LONG_LONG != 0 {
        *bmp_bits(dst).add(nwords - 1) &= bmp_last_word_mask(bits);
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_or(
    bits: size_t,
    dst: *mut arena_bitmap,
    src1: *mut arena_bitmap,
    src2: *mut arena_bitmap,
) {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        *bmp_bits(dst).add(i as size_t) =
            *bmp_bits_const(src1).add(i as size_t) | *bmp_bits_const(src2).add(i as size_t);
        i = i.wrapping_add(1);
    }

    if nwords != 0 && bits % BITS_PER_LONG_LONG != 0 {
        *bmp_bits(dst).add(nwords - 1) &= bmp_last_word_mask(bits);
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_empty(bits: size_t, bmp: *mut arena_bitmap) -> bool {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        let mask: u64 = if (i as size_t) == nwords - 1 {
            bmp_last_word_mask(bits)
        } else {
            !0u64
        };

        if (*bmp_bits_const(bmp).add(i as size_t) & mask) != 0 {
            return false;
        }

        i = i.wrapping_add(1);
    }

    true
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_copy(bits: size_t, dst: *mut arena_bitmap, src: *mut arena_bitmap) {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        *bmp_bits(dst).add(i as size_t) = *bmp_bits_const(src).add(i as size_t);
        i = i.wrapping_add(1);
    }

    if nwords != 0 && bits % BITS_PER_LONG_LONG != 0 {
        *bmp_bits(dst).add(nwords - 1) &= bmp_last_word_mask(bits);
    }
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_subset(
    bits: size_t,
    big: *mut arena_bitmap,
    small: *mut arena_bitmap,
) -> bool {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        let mask: u64 = if (i as size_t) == nwords - 1 {
            bmp_last_word_mask(bits)
        } else {
            !0u64
        };

        if (!*bmp_bits_const(big).add(i as size_t) & *bmp_bits_const(small).add(i as size_t) & mask)
            != 0
        {
            return false;
        }

        i = i.wrapping_add(1);
    }

    true
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_intersects(
    bits: size_t,
    arg1: *mut arena_bitmap,
    arg2: *mut arena_bitmap,
) -> bool {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        let mask: u64 = if (i as size_t) == nwords - 1 {
            bmp_last_word_mask(bits)
        } else {
            !0u64
        };

        if (*bmp_bits_const(arg1).add(i as size_t)
            & *bmp_bits_const(arg2).add(i as size_t)
            & mask)
            != 0
        {
            return true;
        }

        i = i.wrapping_add(1);
    }

    false
}

// __weak
#[no_mangle]
pub unsafe extern "C" fn bmp_print(bits: size_t, bmp: *mut arena_bitmap) {
    let nwords: size_t = BITS_TO_LONG_LONGS(bits);
    let mut i: u32 = zero;

    while (i as size_t) < nwords && can_loop {
        arena_stderr(
            b"%016llx \0".as_ptr() as *const c_char,
            *bmp_bits_const(bmp).add(i as size_t),
        );
        i = i.wrapping_add(1);
    }
}
