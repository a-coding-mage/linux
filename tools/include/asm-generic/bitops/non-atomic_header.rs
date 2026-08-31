/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C dependency intent: <linux/bits.h> supplies BIT_MASK(), BIT_WORD(), and
 * BITS_PER_LONG.
 */

use core::ffi::c_ulong;

/**
 * ___set_bit - Set a bit in memory
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * Unlike set_bit(), this function is non-atomic and may be reordered.
 * If it's called on the same region of memory simultaneously, the effect
 * may be that only one operation succeeds.
 */
#[inline(always)]
pub unsafe fn ___set_bit(nr: c_ulong, addr: *mut c_ulong) {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);

    *p |= mask;
}

#[inline(always)]
pub unsafe fn ___clear_bit(nr: c_ulong, addr: *mut c_ulong) {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);

    *p &= !mask;
}

/**
 * ___change_bit - Toggle a bit in memory
 * @nr: the bit to change
 * @addr: the address to start counting from
 *
 * Unlike change_bit(), this function is non-atomic and may be reordered.
 * If it's called on the same region of memory simultaneously, the effect
 * may be that only one operation succeeds.
 */
#[inline(always)]
pub unsafe fn ___change_bit(nr: c_ulong, addr: *mut c_ulong) {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);

    *p ^= mask;
}

/**
 * ___test_and_set_bit - Set a bit and return its old value
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This operation is non-atomic and can be reordered.
 * If two examples of this operation race, one can appear to succeed
 * but actually fail.  You must protect multiple accesses with a lock.
 */
#[inline(always)]
pub unsafe fn ___test_and_set_bit(nr: c_ulong, addr: *mut c_ulong) -> bool {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);
    let old: c_ulong = *p;

    *p = old | mask;
    (old & mask) != 0
}

/**
 * ___test_and_clear_bit - Clear a bit and return its old value
 * @nr: Bit to clear
 * @addr: Address to count from
 *
 * This operation is non-atomic and can be reordered.
 * If two examples of this operation race, one can appear to succeed
 * but actually fail.  You must protect multiple accesses with a lock.
 */
#[inline(always)]
pub unsafe fn ___test_and_clear_bit(nr: c_ulong, addr: *mut c_ulong) -> bool {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);
    let old: c_ulong = *p;

    *p = old & !mask;
    (old & mask) != 0
}

/* WARNING: non atomic and it can be reordered! */
#[inline(always)]
pub unsafe fn ___test_and_change_bit(nr: c_ulong, addr: *mut c_ulong) -> bool {
    let mask: c_ulong = BIT_MASK(nr);
    let p: *mut c_ulong = addr.add(BIT_WORD(nr) as usize);
    let old: c_ulong = *p;

    *p = old ^ mask;
    (old & mask) != 0
}

/**
 * _test_bit - Determine whether a bit is set
 * @nr: bit number to test
 * @addr: Address to start counting from
 */
#[inline(always)]
pub unsafe fn _test_bit(nr: c_ulong, addr: *const c_ulong) -> bool {
    (1 as c_ulong) & (*addr.add(BIT_WORD(nr) as usize) >> (nr & (BITS_PER_LONG - 1))) != 0
}
