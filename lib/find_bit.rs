// SPDX-License-Identifier: GPL-2.0-or-later
/* bit search implementation
 *
 * Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * Copyright (C) 2008 IBM Corporation
 * 'find_last_bit' is written by Rusty Russell <rusty@rustcorp.com.au>
 * (Inspired by David Howell's find_next_bit implementation)
 *
 * Rewritten by Yury Norov <yury.norov@gmail.com> to decrease
 * size and improve performance, 2015.
 */

// Dependencies: linux/bitops.h, linux/bitmap.h, linux/math.h,
// linux/minmax.h, linux/swab.h, linux/random.h.
//
// lib/Kconfig limits RUST_FIND_BIT to 64-bit little-endian kernels without
// architecture find_*_bit() overrides; the big-endian _le variants are not
// built there.

use core::cmp::min;
use kernel::bindings::{__bitmap_weight, __get_random_u32_below};
use kernel::ffi::{c_int, c_uint, c_ulong};

const BITS_PER_LONG: c_ulong = c_ulong::BITS as c_ulong;

/// Widens an unsigned int to unsigned long (`c_ulong` is `usize` here).
fn ul(x: u32) -> c_ulong {
    x as c_ulong
}

/// __ffs(): the index of the lowest set bit (undefined for 0 in C).
fn ffs(word: c_ulong) -> c_ulong {
    ul(word.trailing_zeros())
}

/// __fls(): the index of the highest set bit (undefined for 0 in C).
fn fls(word: c_ulong) -> c_ulong {
    (BITS_PER_LONG - 1).wrapping_sub(ul(word.leading_zeros()))
}

/// hweight_long().
fn hweight_long(word: c_ulong) -> c_ulong {
    ul(word.count_ones())
}

/// fns(): the index of the n-th set bit of a word, or BITS_PER_LONG.
fn fns(mut word: c_ulong, mut n: c_uint) -> c_uint {
    while word != 0 && {
        let remaining = n;
        n = n.wrapping_sub(1);
        remaining != 0
    } {
        word &= word - 1;
    }

    if word != 0 {
        word.trailing_zeros()
    } else {
        BITS_PER_LONG as c_uint
    }
}

/// BITMAP_FIRST_WORD_MASK(start).
fn first_word_mask(start: c_ulong) -> c_ulong {
    !0 << (start & (BITS_PER_LONG - 1))
}

/// BITMAP_LAST_WORD_MASK(nbits).
fn last_word_mask(nbits: c_ulong) -> c_ulong {
    !0 >> (nbits.wrapping_neg() & (BITS_PER_LONG - 1))
}

/*
 * Common helper for find_bit() function family
 * @FETCH: The expression that fetches and pre-processes each word of bitmap(s)
 * @MUNGE: The expression that post-processes a word containing found bit (may be empty)
 * @size: The bitmap size in bits
 */
/// FIND_FIRST_BIT(FETCH, /* nop */, size).
#[inline(always)]
fn find_first(fetch: impl Fn(c_ulong) -> c_ulong, size: c_ulong) -> c_ulong {
    let mut sz = size;
    let mut idx: c_ulong = 0;

    while idx.wrapping_mul(BITS_PER_LONG) < sz {
        let val = fetch(idx);
        if val != 0 {
            sz = min(idx.wrapping_mul(BITS_PER_LONG).wrapping_add(ffs(val)), sz);
            break;
        }
        idx = idx.wrapping_add(1);
    }

    sz
}

/*
 * Common helper for find_next_bit() function family
 * @FETCH: The expression that fetches and pre-processes each word of bitmap(s)
 * @MUNGE: The expression that post-processes a word containing found bit (may be empty)
 * @size: The bitmap size in bits
 * @start: The bitnumber to start searching at
 */
/// FIND_NEXT_BIT(FETCH, /* nop */, size, start).
#[inline(always)]
fn find_next(fetch: impl Fn(c_ulong) -> c_ulong, size: c_ulong, start: c_ulong) -> c_ulong {
    let sz = size;

    if start >= sz {
        return sz;
    }

    let mask = first_word_mask(start);
    let mut idx = start / BITS_PER_LONG;

    let mut tmp = fetch(idx) & mask;
    while tmp == 0 {
        if idx.wrapping_add(1).wrapping_mul(BITS_PER_LONG) >= sz {
            return sz;
        }
        idx = idx.wrapping_add(1);
        tmp = fetch(idx);
    }

    min(idx.wrapping_mul(BITS_PER_LONG).wrapping_add(ffs(tmp)), sz)
}

/// FIND_NTH_BIT(FETCH, size, num).
#[inline(always)]
fn find_nth(fetch: impl Fn(c_ulong) -> c_ulong, size: c_ulong, num: c_ulong) -> c_ulong {
    let sz = size;
    let mut nr = num;
    let mut idx: c_ulong = 0;
    let mut tmp: c_ulong = 0;

    'found: {
        while idx.wrapping_add(1).wrapping_mul(BITS_PER_LONG) <= sz {
            if idx.wrapping_mul(BITS_PER_LONG).wrapping_add(nr) >= sz {
                return sz;
            }

            tmp = fetch(idx);
            let w = hweight_long(tmp);
            if w > nr {
                break 'found;
            }

            nr -= w;
            idx = idx.wrapping_add(1);
        }

        if sz % BITS_PER_LONG != 0 {
            tmp = fetch(idx) & last_word_mask(sz);
        }
    }
    // found: fns() takes the count as unsigned int.
    idx.wrapping_mul(BITS_PER_LONG).wrapping_add(ul(fns(tmp, nr as c_uint)))
}

/// Reads word `idx` of a bitmap.
///
/// # Safety
/// `addr` covers word `idx`.
unsafe fn word(addr: *const c_ulong, idx: c_ulong) -> c_ulong {
    // SAFETY: forwarded caller contract.
    unsafe { *addr.add(idx as usize) }
}

/*
 * Find the first set bit in a memory region.
 */
/// # Safety
/// `addr` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_first(|idx| unsafe { word(addr, idx) }, size)
}
// EXPORT_SYMBOL(_find_first_bit);

/*
 * Find the first set bit in two memory regions.
 */
/// # Safety
/// `addr1` and `addr2` cover `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_first_and_bit(addr1: *const c_ulong, addr2: *const c_ulong, size: c_ulong) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_first(|idx| unsafe { word(addr1, idx) & word(addr2, idx) }, size)
}
// EXPORT_SYMBOL(_find_first_and_bit);

/*
 * Find the first bit set in 1st memory region and unset in 2nd.
 */
/// # Safety
/// `addr1` and `addr2` cover `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_first_andnot_bit(addr1: *const c_ulong, addr2: *const c_ulong, size: c_ulong) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_first(|idx| unsafe { word(addr1, idx) & !word(addr2, idx) }, size)
}
// EXPORT_SYMBOL(_find_first_andnot_bit);

/*
 * Find the first set bit in three memory regions.
 */
/// # Safety
/// `addr1`, `addr2` and `addr3` cover `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_first_and_and_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    addr3: *const c_ulong,
    size: c_ulong,
) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_first(|idx| unsafe { word(addr1, idx) & word(addr2, idx) & word(addr3, idx) }, size)
}
// EXPORT_SYMBOL(_find_first_and_and_bit);

/*
 * Find the first cleared bit in a memory region.
 */
/// # Safety
/// `addr` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_first(|idx| unsafe { !word(addr, idx) }, size)
}
// EXPORT_SYMBOL(_find_first_zero_bit);

/// # Safety
/// `addr` covers `nbits` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_next_bit(addr: *const c_ulong, nbits: c_ulong, start: c_ulong) -> c_ulong {
    // SAFETY: only words below nbits are fetched.
    find_next(|idx| unsafe { word(addr, idx) }, nbits, start)
}
// EXPORT_SYMBOL(_find_next_bit);

/// # Safety
/// `addr` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn __find_nth_bit(addr: *const c_ulong, size: c_ulong, n: c_ulong) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_nth(|idx| unsafe { word(addr, idx) }, size, n)
}
// EXPORT_SYMBOL(__find_nth_bit);

/// # Safety
/// `addr1` and `addr2` cover `size` bits.
#[no_mangle]
pub unsafe extern "C" fn __find_nth_and_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    size: c_ulong,
    n: c_ulong,
) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_nth(|idx| unsafe { word(addr1, idx) & word(addr2, idx) }, size, n)
}
// EXPORT_SYMBOL(__find_nth_and_bit);

/// # Safety
/// `addr1`, `addr2` and `addr3` cover `size` bits.
#[no_mangle]
pub unsafe extern "C" fn __find_nth_and_andnot_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    addr3: *const c_ulong,
    size: c_ulong,
    n: c_ulong,
) -> c_ulong {
    // SAFETY: only words below size are fetched.
    find_nth(|idx| unsafe { word(addr1, idx) & word(addr2, idx) & !word(addr3, idx) }, size, n)
}
// EXPORT_SYMBOL(__find_nth_and_andnot_bit);

/// # Safety
/// `addr1` and `addr2` cover `nbits` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_next_and_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    // SAFETY: only words below nbits are fetched.
    find_next(|idx| unsafe { word(addr1, idx) & word(addr2, idx) }, nbits, start)
}
// EXPORT_SYMBOL(_find_next_and_bit);

/// # Safety
/// `addr1` and `addr2` cover `nbits` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_next_andnot_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    // SAFETY: only words below nbits are fetched.
    find_next(|idx| unsafe { word(addr1, idx) & !word(addr2, idx) }, nbits, start)
}
// EXPORT_SYMBOL(_find_next_andnot_bit);

/// # Safety
/// `addr1` and `addr2` cover `nbits` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_next_or_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    // SAFETY: only words below nbits are fetched.
    find_next(|idx| unsafe { word(addr1, idx) | word(addr2, idx) }, nbits, start)
}
// EXPORT_SYMBOL(_find_next_or_bit);

/// # Safety
/// `addr` covers `nbits` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_next_zero_bit(addr: *const c_ulong, nbits: c_ulong, start: c_ulong) -> c_ulong {
    // SAFETY: only words below nbits are fetched.
    find_next(|idx| unsafe { !word(addr, idx) }, nbits, start)
}
// EXPORT_SYMBOL(_find_next_zero_bit);

/// # Safety
/// `addr` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn _find_last_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    if size != 0 {
        let mut val = last_word_mask(size);
        let mut idx = (size - 1) / BITS_PER_LONG;

        loop {
            // SAFETY: idx stays below the last word of the bitmap.
            val &= unsafe { word(addr, idx) };
            if val != 0 {
                return idx.wrapping_mul(BITS_PER_LONG).wrapping_add(fls(val));
            }

            val = !0;
            let current = idx;
            idx = idx.wrapping_sub(1);
            if current == 0 {
                break;
            }
        }
    }
    size
}
// EXPORT_SYMBOL(_find_last_bit);

/// # Safety
/// `addr` covers `size` bits and `clump` points to writable storage.
#[no_mangle]
pub unsafe extern "C" fn find_next_clump8(
    clump: *mut c_ulong,
    addr: *const c_ulong,
    size: c_ulong,
    mut offset: c_ulong,
) -> c_ulong {
    // SAFETY: the clump word lies below size, which find_next_bit() bounds.
    unsafe {
        offset = _find_next_bit(addr, size, offset);
        if offset == size {
            return size;
        }

        offset &= !7; // round_down(offset, 8)
        // bitmap_get_value8(addr, offset)
        *clump = (word(addr, offset / BITS_PER_LONG) >> (offset % BITS_PER_LONG)) & 0xff;
    }

    offset
}
// EXPORT_SYMBOL(find_next_clump8);

/**
 * find_random_bit - find a set bit at random position
 * @addr: The address to base the search on
 * @size: The bitmap size in bits
 *
 * Returns: a position of a random set bit; >= @size otherwise
 */
/// # Safety
/// `addr` covers `size` bits.
#[no_mangle]
pub unsafe extern "C" fn find_random_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    // SAFETY: forwarded caller contract; bitmap_weight() takes unsigned int.
    unsafe {
        let w = __bitmap_weight(addr, size as c_uint) as c_int;

        match w {
            0 => size,
            /* Performance trick for single-bit bitmaps */
            1 => _find_first_bit(addr, size),
            _ => {
                // find_nth_bit(addr, size, get_random_u32_below(w))
                let n = ul(__get_random_u32_below(w as u32));
                if n >= size {
                    size
                } else {
                    __find_nth_bit(addr, size, n)
                }
            }
        }
    }
}
// EXPORT_SYMBOL(find_random_bit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
