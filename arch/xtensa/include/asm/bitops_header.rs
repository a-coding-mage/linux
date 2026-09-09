/*
 * include/asm-xtensa/bitops.h
 *
 * Atomic operations that C can't guarantee us. Useful for resource counting etc.
 * Rust translation of the Xtensa bit operations header.
 *
 * C-only includes and preprocessor guards are represented by the dependency
 * notes below; their declarations are supplied by the surrounding kernel.
 */

// Dependencies supplied by the surrounding kernel:
// asm/processor.h, asm/byteorder.h, asm/barrier.h,
// asm-generic/bitops/non-atomic.h, ffs.h, __ffs.h, ffz.h, fls.h, __fls.h,
// fls64.h, atomic.h, instrumented-atomic.h, le.h, ext2-atomic-setbit.h,
// hweight.h, lock.h, and sched.h.

#[inline]
pub fn __cntlz(x: u32) -> u32 {
    x.leading_zeros()
}

/* ffz: Find first zero in word. Undefined if no zero exists. */
#[inline]
pub const fn ffz(x: u32) -> i32 {
    31 - ((!x).wrapping_add(1)).leading_zeros() as i32
}

/* __ffs: Find first bit set in word. Return 0 for bit 0. */
#[inline]
pub const fn __ffs(x: u32) -> u32 {
    31u32.wrapping_sub(x.wrapping_neg().leading_zeros())
}

/* ffs: Find first bit set in word; differs in spirit from ffz. */
#[inline]
pub const fn ffs(x: u32) -> i32 {
    32 - x.wrapping_neg().leading_zeros() as i32
}

/* fls: Find last (most-significant) bit set in word. */
#[inline]
pub const fn fls(x: u32) -> i32 {
    32 - x.leading_zeros() as i32
}

/* __fls - find last (most-significant) set bit in a long word. */
#[inline]
pub const fn __fls(word: u32) -> u32 {
    31u32.wrapping_sub(word.leading_zeros())
}

/*
 * The generic implementations above are used when XCHAL_HAVE_NSA is false.
 * On Xtensa targets with NSA/NSAU they have the same 32-bit result as the
 * corresponding nsau-based C inline functions.
 */

#[inline]
unsafe fn bit_word(p: *mut u32, bit: u32) -> *mut u32 {
    p.add((bit >> 5) as usize)
}

#[inline]
unsafe fn atomic_bit_op(bit: u32, p: *mut u32, op: fn(u32, u32) -> u32) {
    use core::sync::atomic::{AtomicU32, Ordering};
    let word = bit_word(p, bit);
    let a = &*(word as *const AtomicU32);
    let mask = 1u32 << (bit & 31);
    let mut old = a.load(Ordering::Relaxed);
    loop {
        let new = op(old, mask);
        match a.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => return,
            Err(value) => old = value,
        }
    }
}

#[inline]
unsafe fn test_and_atomic_bit_op(bit: u32, p: *mut u32, op: fn(u32, u32) -> u32) -> i32 {
    use core::sync::atomic::{AtomicU32, Ordering};
    let word = bit_word(p, bit);
    let a = &*(word as *const AtomicU32);
    let mask = 1u32 << (bit & 31);
    let mut old = a.load(Ordering::Relaxed);
    loop {
        let new = op(old, mask);
        match a.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => return (old & mask) as i32,
            Err(value) => old = value,
        }
    }
}

#[inline]
pub unsafe fn arch_set_bit(bit: u32, p: *mut u32) { atomic_bit_op(bit, p, |v, m| v | m); }
#[inline]
pub unsafe fn arch_test_and_set_bit(bit: u32, p: *mut u32) -> i32 { test_and_atomic_bit_op(bit, p, |v, m| v | m) }
#[inline]
pub unsafe fn arch_clear_bit(bit: u32, p: *mut u32) { atomic_bit_op(bit, p, |v, m| v & !m); }
#[inline]
pub unsafe fn arch_test_and_clear_bit(bit: u32, p: *mut u32) -> i32 { test_and_atomic_bit_op(bit, p, |v, m| v & !m) }
#[inline]
pub unsafe fn arch_change_bit(bit: u32, p: *mut u32) { atomic_bit_op(bit, p, |v, m| v ^ m); }
#[inline]
pub unsafe fn arch_test_and_change_bit(bit: u32, p: *mut u32) -> i32 { test_and_atomic_bit_op(bit, p, |v, m| v ^ m) }

// The remaining declarations and generic bit-operation helpers are supplied
// by the asm-generic headers included by the original C header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
