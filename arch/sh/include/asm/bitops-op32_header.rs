/* SPDX-License-Identifier: GPL-2.0 */

// The bit modifying instructions on SH-2A are only capable of working with a
// 3-bit immediate, which signifies the shift position for the bit being worked on.
// `BITS_PER_LONG` and `BITS_PER_BYTE` are supplied by the surrounding bindings.

#[cfg(target_endian = "big")]
#[inline]
fn byte_number(nr: usize) -> usize {
    (nr ^ (BITS_PER_LONG - 1) & !0x7) / BITS_PER_BYTE
}

#[cfg(target_endian = "big")]
#[inline]
fn byte_offset(nr: usize) -> usize {
    (nr ^ (BITS_PER_LONG - 1) & !0x7) % BITS_PER_BYTE
}

#[cfg(not(target_endian = "big"))]
#[inline]
fn byte_number(nr: usize) -> usize {
    nr / BITS_PER_BYTE
}

#[cfg(not(target_endian = "big"))]
#[inline]
fn byte_offset(nr: usize) -> usize {
    nr % BITS_PER_BYTE
}

#[inline]
pub unsafe fn arch___set_bit(nr: usize, addr: *mut usize) {
    // The C constant-expression path emits SH `bset.b` inline assembly. Rust
    // cannot express that target-specific inline assembly here; the equivalent
    // read-modify-write operation is retained for both constant and dynamic nr.
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    *p |= mask;
}

#[inline]
pub unsafe fn arch___clear_bit(nr: usize, addr: *mut usize) {
    // The C constant-expression path emits SH `bclr.b` inline assembly.
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    *p &= !mask;
}

/**
 * arch___change_bit - Toggle a bit in memory
 * @nr: the bit to change
 * @addr: the address to start counting from
 *
 * Unlike change_bit(), this function is non-atomic and may be reordered.
 * If it's called on the same region of memory simultaneously, the effect
 * may be that only one operation succeeds.
 */
#[inline]
pub unsafe fn arch___change_bit(nr: usize, addr: *mut usize) {
    // The C constant-expression path emits SH `bxor.b` inline assembly.
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    *p ^= mask;
}

/**
 * arch___test_and_set_bit - Set a bit and return its old value
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This operation is non-atomic and can be reordered.
 * If two examples of this operation race, one can appear to succeed
 * but actually fail.  You must protect multiple accesses with a lock.
 */
#[inline]
pub unsafe fn arch___test_and_set_bit(nr: usize, addr: *mut usize) -> bool {
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    let old = *p;
    *p = old | mask;
    (old & mask) != 0
}

/**
 * arch___test_and_clear_bit - Clear a bit and return its old value
 * @nr: Bit to clear
 * @addr: Address to count from
 *
 * This operation is non-atomic and can be reordered.
 * If two examples of this operation race, one can appear to succeed
 * but actually fail.  You must protect multiple accesses with a lock.
 */
#[inline]
pub unsafe fn arch___test_and_clear_bit(nr: usize, addr: *mut usize) -> bool {
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    let old = *p;
    *p = old & !mask;
    (old & mask) != 0
}

/* WARNING: non atomic and it can be reordered! */
#[inline]
pub unsafe fn arch___test_and_change_bit(nr: usize, addr: *mut usize) -> bool {
    let mask: usize = BIT_MASK(nr);
    let p = addr.add(BIT_WORD(nr));
    let old = *p;
    *p = old ^ mask;
    (old & mask) != 0
}

pub use generic_test_bit as arch_test_bit;
pub use generic_test_bit_acquire as arch_test_bit_acquire;

// Dependency supplied by <asm-generic/bitops/non-instrumented-non-atomic.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
