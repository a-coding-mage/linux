/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding translation unit. */

/**
 * arch_test_and_set_bit_lock - Set a bit and return its old value, for lock
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This operation is atomic and provides acquire barrier semantics if
 * the returned value is 0.
 * It can be used to implement bit locks.
 */
#[inline(always)]
pub unsafe fn arch_test_and_set_bit_lock(nr: u32, addr: *mut usize) -> i32 {
    let mask: usize = BIT_MASK(nr);

    let addr = addr.add(BIT_WORD(nr));
    if READ_ONCE(*addr) & mask != 0 {
        return 1;
    }

    let old: isize = raw_atomic_long_fetch_or_acquire(mask, addr as *mut atomic_long_t);
    (if old as usize & mask != 0 { 1 } else { 0 }) as i32
}

/**
 * arch_clear_bit_unlock - Clear a bit in memory, for unlock
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * This operation is atomic and provides release barrier semantics.
 */
#[inline(always)]
pub unsafe fn arch_clear_bit_unlock(nr: u32, addr: *mut usize) {
    let addr = addr.add(BIT_WORD(nr));
    raw_atomic_long_fetch_andnot_release(BIT_MASK(nr), addr as *mut atomic_long_t);
}

/**
 * arch___clear_bit_unlock - Clear a bit in memory, for unlock
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * A weaker form of clear_bit_unlock() as used by __bit_lock_unlock(). If all
 * the bits in the word are protected by this lock some archs can use weaker
 * ops to safely unlock.
 *
 * See for example x86's implementation.
 */
#[inline]
pub unsafe fn arch___clear_bit_unlock(nr: u32, addr: *mut usize) {
    let addr = addr.add(BIT_WORD(nr));
    let mut old: usize = READ_ONCE(*addr);
    old &= !BIT_MASK(nr);
    raw_atomic_long_set_release(addr as *mut atomic_long_t, old);
}

/* Preserved from the C conditional: define this only when the architecture
 * does not provide arch_xor_unlock_is_negative_byte. */
#[inline]
pub unsafe fn arch_xor_unlock_is_negative_byte(mask: usize, p: *mut usize) -> bool {
    let old: isize = raw_atomic_long_fetch_xor_release(mask, p as *mut atomic_long_t);
    old as usize & BIT(7) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
