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

const BITS_PER_LONG: usize = usize::BITS as usize;

extern "C" {
    fn __ffs(x: usize) -> usize;
    fn __fls(x: usize) -> usize;
    fn fns(x: usize, n: usize) -> usize;
    fn hweight_long(x: usize) -> usize;
    fn bitmap_weight(addr: *const usize, size: usize) -> i32;
    fn get_random_u32_below(n: i32) -> u32;
    fn find_next_bit(addr: *const usize, size: usize, start: usize) -> usize;
    fn find_first_bit(addr: *const usize, size: usize) -> usize;
    fn find_nth_bit(addr: *const usize, size: usize, n: usize) -> usize;
    fn bitmap_get_value8(addr: *const usize, offset: usize) -> usize;
}

#[inline]
unsafe fn find_first_bit_impl<F, M>(fetch: F, munge: M, size: usize) -> usize
where
    F: Fn(usize) -> usize,
    M: Fn(usize) -> usize,
{
    let mut idx = 0usize;
    let mut sz = size;
    while idx.wrapping_mul(BITS_PER_LONG) < sz {
        let val = fetch(idx);
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

#[inline]
unsafe fn find_next_bit_impl<F, M>(fetch: F, munge: M, size: usize, start: usize) -> usize
where
    F: Fn(usize) -> usize,
    M: Fn(usize) -> usize,
{
    let mut sz = size;
    let start = start;
    if start >= sz {
        return sz;
    }
    let mask = munge((!0usize).wrapping_shl((start % BITS_PER_LONG) as u32));
    let mut idx = start / BITS_PER_LONG;
    let mut tmp = fetch(idx) & mask;
    while tmp == 0 {
        if (idx + 1).wrapping_mul(BITS_PER_LONG) >= sz {
            return sz;
        }
        idx += 1;
        tmp = fetch(idx);
    }
    sz = core::cmp::min(idx.wrapping_mul(BITS_PER_LONG).wrapping_add(__ffs(munge(tmp))), sz);
    sz
}

#[inline]
unsafe fn find_nth_bit_impl<F>(fetch: F, size: usize, num: usize) -> usize
where
    F: Fn(usize) -> usize,
{
    let mut sz = size;
    let mut nr = num;
    let mut idx = 0usize;
    let mut tmp = 0usize;
    while (idx + 1).wrapping_mul(BITS_PER_LONG) <= sz {
        if idx.wrapping_mul(BITS_PER_LONG).wrapping_add(nr) >= sz {
            return sz;
        }
        tmp = fetch(idx);
        let w = hweight_long(tmp);
        if w > nr {
            break;
        }
        nr -= w;
        idx += 1;
    }
    if sz % BITS_PER_LONG != 0 {
        tmp = fetch(idx) & ((!0usize) >> (BITS_PER_LONG - (sz % BITS_PER_LONG)));
    }
    sz = idx.wrapping_mul(BITS_PER_LONG).wrapping_add(fns(tmp, nr));
    sz
}

pub unsafe fn _find_first_bit(addr: *const usize, size: usize) -> usize {
    find_first_bit_impl(|idx| *addr.add(idx), |x| x, size)
}

pub unsafe fn _find_first_and_bit(addr1: *const usize, addr2: *const usize, size: usize) -> usize {
    find_first_bit_impl(|idx| *addr1.add(idx) & *addr2.add(idx), |x| x, size)
}

pub unsafe fn _find_first_andnot_bit(addr1: *const usize, addr2: *const usize, size: usize) -> usize {
    find_first_bit_impl(|idx| *addr1.add(idx) & !*addr2.add(idx), |x| x, size)
}

pub unsafe fn _find_first_and_and_bit(addr1: *const usize, addr2: *const usize, addr3: *const usize, size: usize) -> usize {
    find_first_bit_impl(|idx| *addr1.add(idx) & *addr2.add(idx) & *addr3.add(idx), |x| x, size)
}

pub unsafe fn _find_first_zero_bit(addr: *const usize, size: usize) -> usize {
    find_first_bit_impl(|idx| !*addr.add(idx), |x| x, size)
}

pub unsafe fn _find_next_bit(addr: *const usize, nbits: usize, start: usize) -> usize {
    find_next_bit_impl(|idx| *addr.add(idx), |x| x, nbits, start)
}

pub unsafe fn __find_nth_bit(addr: *const usize, size: usize, n: usize) -> usize {
    find_nth_bit_impl(|idx| *addr.add(idx), size, n)
}

pub unsafe fn __find_nth_and_bit(addr1: *const usize, addr2: *const usize, size: usize, n: usize) -> usize {
    find_nth_bit_impl(|idx| *addr1.add(idx) & *addr2.add(idx), size, n)
}

pub unsafe fn __find_nth_and_andnot_bit(addr1: *const usize, addr2: *const usize, addr3: *const usize, size: usize, n: usize) -> usize {
    find_nth_bit_impl(|idx| *addr1.add(idx) & *addr2.add(idx) & !*addr3.add(idx), size, n)
}

pub unsafe fn _find_next_and_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize {
    find_next_bit_impl(|idx| *addr1.add(idx) & *addr2.add(idx), |x| x, nbits, start)
}

pub unsafe fn _find_next_andnot_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize {
    find_next_bit_impl(|idx| *addr1.add(idx) & !*addr2.add(idx), |x| x, nbits, start)
}

pub unsafe fn _find_next_or_bit(addr1: *const usize, addr2: *const usize, nbits: usize, start: usize) -> usize {
    find_next_bit_impl(|idx| *addr1.add(idx) | *addr2.add(idx), |x| x, nbits, start)
}

pub unsafe fn _find_next_zero_bit(addr: *const usize, nbits: usize, start: usize) -> usize {
    find_next_bit_impl(|idx| !*addr.add(idx), |x| x, nbits, start)
}

pub unsafe fn _find_last_bit(addr: *const usize, size: usize) -> usize {
    if size != 0 {
        let mut val = !0usize >> (BITS_PER_LONG - (size % BITS_PER_LONG).max(1));
        let mut idx = (size - 1) / BITS_PER_LONG;
        loop {
            val &= *addr.add(idx);
            if val != 0 {
                return idx * BITS_PER_LONG + __fls(val);
            }
            val = !0usize;
            if idx == 0 { break; }
            idx -= 1;
        }
    }
    size
}

pub unsafe fn find_next_clump8(clump: *mut usize, addr: *const usize, size: usize, mut offset: usize) -> usize {
    offset = find_next_bit(addr, size, offset);
    if offset == size { return size; }
    offset &= !7usize;
    *clump = bitmap_get_value8(addr, offset);
    offset
}

pub unsafe fn find_random_bit(addr: *const usize, size: usize) -> usize {
    let w = bitmap_weight(addr, size);
    match w {
        0 => size,
        1 => find_first_bit(addr, size),
        _ => find_nth_bit(addr, size, get_random_u32_below(w) as usize),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
