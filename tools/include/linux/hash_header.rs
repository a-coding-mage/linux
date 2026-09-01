/* Fast hashing routine for ints,  longs and pointers.
   (C) 2002 Nadia Yvette Chambers, IBM */

/* Dependencies in the original header:
 * #include <asm/types.h>
 * #include <linux/compiler.h>
 */

/*
 * The "GOLDEN_RATIO_PRIME" is used in ifs/btrfs/brtfs_inode.h and
 * fs/inode.c.  It's not actually prime any more (the previous primes
 * were actively bad for hashing), but the name remains.
 */
#[cfg(target_pointer_width = "32")]
pub const GOLDEN_RATIO_PRIME: u32 = GOLDEN_RATIO_32;
#[cfg(target_pointer_width = "64")]
pub const GOLDEN_RATIO_PRIME: u64 = GOLDEN_RATIO_64;
/* Original C condition:
 * #error Wordsize not 32 or 64
 */

/*
 * This hash multiplies the input by a large odd number and takes the
 * high bits.  Since multiplication propagates changes to the most
 * significant end only, it is essential that the high bits of the
 * product be used for the hash value.
 *
 * Chuck Lever verified the effectiveness of this technique:
 * http://www.citi.umich.edu/techreports/reports/citi-tr-00-1.pdf
 *
 * Although a random odd number will do, it turns out that the golden
 * ratio phi = (sqrt(5)-1)/2, or its negative, has particularly nice
 * properties.  (See Knuth vol 3, section 6.4, exercise 9.)
 *
 * These are the negative, (1 - phi) = phi**2 = (3 - sqrt(5))/2,
 * which is very slightly easier to multiply by and makes no
 * difference to the hash distribution.
 */
pub const GOLDEN_RATIO_32: u32 = 0x61C88647;
pub const GOLDEN_RATIO_64: u64 = 0x61C8864680B583EBu64;

/* Original C condition:
 * #ifdef CONFIG_HAVE_ARCH_HASH
 * This header may use the GOLDEN_RATIO_xx constants.
 * #include <asm/hash.h>
 * #endif
 */

/*
 * The _generic versions exist only so lib/test_hash.c can compare
 * the arch-optimized versions with the generic.
 *
 * Note that if you change these, any <asm/hash.h> that aren't updated
 * to match need to have their HAVE_ARCH_* define values updated so the
 * self-test will not false-positive.
 */
/* Original C condition:
 * #ifndef HAVE_ARCH__HASH_32
 * #define __hash_32 __hash_32_generic
 * #endif
 */
#[inline]
pub fn __hash_32_generic(val: u32) -> u32 {
    val.wrapping_mul(GOLDEN_RATIO_32)
}

#[inline]
pub fn __hash_32(val: u32) -> u32 {
    __hash_32_generic(val)
}

#[inline]
pub fn hash_32(val: u32, bits: u32) -> u32 {
    /* High bits are more random, so use them. */
    __hash_32(val) >> (32u32.wrapping_sub(bits))
}

/* Original C condition:
 * #ifndef HAVE_ARCH_HASH_64
 * #define hash_64 hash_64_generic
 * #endif
 */
#[inline(always)]
pub fn hash_64_generic(val: u64, bits: u32) -> u32 {
    #[cfg(target_pointer_width = "64")]
    {
        /* 64x64-bit multiply is efficient on all 64-bit processors */
        (val.wrapping_mul(GOLDEN_RATIO_64) >> (64u32.wrapping_sub(bits))) as u32
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        /* Hash 64 bits using only 32x32-bit multiply. */
        hash_32(
            (val as u32) ^ __hash_32((val >> 32) as u32),
            bits,
        )
    }
}

#[inline]
pub fn hash_64(val: u64, bits: u32) -> u32 {
    hash_64_generic(val, bits)
}

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn hash_long(val: usize, bits: u32) -> u32 {
    hash_32(val as u32, bits)
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn hash_long(val: usize, bits: u32) -> u32 {
    hash_64(val as u64, bits)
}

#[inline]
pub unsafe fn hash_ptr(ptr: *const core::ffi::c_void, bits: u32) -> u32 {
    hash_long(ptr as usize, bits)
}

/* This really should be called fold32_ptr; it does no hashing to speak of. */
#[inline]
pub unsafe fn hash32_ptr(ptr: *const core::ffi::c_void) -> u32 {
    let mut val = ptr as usize;

    #[cfg(target_pointer_width = "64")]
    {
        val ^= val >> 32;
    }
    val as u32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
