// SPDX-License-Identifier: GPL-2.0-or-later
/* bit search implementation
 *
 * Copied from lib/find_bit.c to tools/lib/find_bit.c
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

use core::ffi::c_ulong;

const BITS_PER_LONG: c_ulong = (core::mem::size_of::<c_ulong>() * 8) as c_ulong;

#[inline]
fn __ffs(word: c_ulong) -> c_ulong {
    word.trailing_zeros() as c_ulong
}

#[inline]
fn BITMAP_FIRST_WORD_MASK(start: c_ulong) -> c_ulong {
    c_ulong::MAX << (start & (BITS_PER_LONG - 1))
}

/*
 * Common helper for find_bit() function family
 * @FETCH: The expression that fetches and pre-processes each word of bitmap(s)
 * @MUNGE: The expression that post-processes a word containing found bit (may be empty)
 * @size: The bitmap size in bits
 */
unsafe fn FIND_FIRST_BIT<F, M>(mut fetch: F, munge: M, size: c_ulong) -> c_ulong
where
    F: FnMut(c_ulong) -> c_ulong,
    M: Fn(c_ulong) -> c_ulong,
{
    let mut idx: c_ulong;
    let val: c_ulong;
    let mut sz: c_ulong = size;

    idx = 0;
    while idx.wrapping_mul(BITS_PER_LONG) < sz {
        val = fetch(idx);
        if val != 0 {
            sz = core::cmp::min(
                idx.wrapping_mul(BITS_PER_LONG).wrapping_add(__ffs(munge(val))),
                sz,
            );
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
unsafe fn FIND_NEXT_BIT<F, M>(mut fetch: F, munge: M, size: c_ulong, start: c_ulong) -> c_ulong
where
    F: FnMut(c_ulong) -> c_ulong,
    M: Fn(c_ulong) -> c_ulong,
{
    let mut mask: c_ulong;
    let mut idx: c_ulong;
    let mut tmp: c_ulong;
    let mut sz: c_ulong = size;
    let __start: c_ulong = start;

    if __start >= sz {
        return sz;
    }

    mask = munge(BITMAP_FIRST_WORD_MASK(__start));
    idx = __start / BITS_PER_LONG;

    tmp = fetch(idx) & mask;
    while tmp == 0 {
        if idx.wrapping_add(1).wrapping_mul(BITS_PER_LONG) >= sz {
            return sz;
        }
        idx = idx.wrapping_add(1);
        tmp = fetch(idx);
    }

    sz = core::cmp::min(
        idx.wrapping_mul(BITS_PER_LONG).wrapping_add(__ffs(munge(tmp))),
        sz,
    );
    sz
}

/* The original C file conditionally omits each definition when an equivalent
 * macro or inline implementation is already provided by included headers.
 */

/*
 * Find the first set bit in a memory region.
 */
#[no_mangle]
pub unsafe extern "C" fn _find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    FIND_FIRST_BIT(|idx| unsafe { *addr.add(idx as usize) }, |val| val, size)
}

/*
 * Find the first set bit in two memory regions.
 */
#[no_mangle]
pub unsafe extern "C" fn _find_first_and_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    size: c_ulong,
) -> c_ulong {
    FIND_FIRST_BIT(
        |idx| unsafe { *addr1.add(idx as usize) & *addr2.add(idx as usize) },
        |val| val,
        size,
    )
}

/*
 * Find the first cleared bit in a memory region.
 */
#[no_mangle]
pub unsafe extern "C" fn _find_first_zero_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    FIND_FIRST_BIT(|idx| unsafe { !*addr.add(idx as usize) }, |val| val, size)
}

#[no_mangle]
pub unsafe extern "C" fn _find_next_bit(
    addr: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    FIND_NEXT_BIT(
        |idx| unsafe { *addr.add(idx as usize) },
        |val| val,
        nbits,
        start,
    )
}

#[no_mangle]
pub unsafe extern "C" fn _find_next_and_bit(
    addr1: *const c_ulong,
    addr2: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    FIND_NEXT_BIT(
        |idx| unsafe { *addr1.add(idx as usize) & *addr2.add(idx as usize) },
        |val| val,
        nbits,
        start,
    )
}

#[no_mangle]
pub unsafe extern "C" fn _find_next_zero_bit(
    addr: *const c_ulong,
    nbits: c_ulong,
    start: c_ulong,
) -> c_ulong {
    FIND_NEXT_BIT(
        |idx| unsafe { !*addr.add(idx as usize) },
        |val| val,
        nbits,
        start,
    )
}
