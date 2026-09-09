/* Fast hashing routine for ints, longs and pointers.
 * (C) 2002 Nadia Yvette Chambers, IBM
 *
 * The original header selects these definitions according to BITS_PER_LONG
 * and may replace them with architecture-specific implementations.
 */

pub const GOLDEN_RATIO_32: u32 = 0x61C88647;
pub const GOLDEN_RATIO_64: u64 = 0x61C8864680B583EB;

/* GOLDEN_RATIO_PRIME is selected from the target word size. */
#[cfg(target_pointer_width = "32")]
pub const GOLDEN_RATIO_PRIME: u32 = GOLDEN_RATIO_32;
#[cfg(target_pointer_width = "64")]
pub const GOLDEN_RATIO_PRIME: u64 = GOLDEN_RATIO_64;

/* The generic versions exist so callers can compare against arch versions. */
#[inline]
pub fn __hash_32_generic(val: u32) -> u32 {
    val.wrapping_mul(GOLDEN_RATIO_32)
}

/* Architecture-specific __hash_32 implementations may replace this symbol. */
#[inline]
pub fn __hash_32(val: u32) -> u32 {
    __hash_32_generic(val)
}

#[inline]
pub fn hash_32(val: u32, bits: u32) -> u32 {
    /* High bits are more random, so use them. */
    __hash_32(val) >> (32 - bits)
}

#[inline]
pub fn hash_64_generic(val: u64, bits: u32) -> u32 {
    #[cfg(target_pointer_width = "64")]
    {
        /* 64x64-bit multiply is efficient on all 64-bit processors. */
        return val.wrapping_mul(GOLDEN_RATIO_64) as u32 >> (64 - bits);
    }

    #[cfg(target_pointer_width = "32")]
    {
        /* Hash 64 bits using only 32x32-bit multiply. */
        return hash_32((val as u32) ^ __hash_32((val >> 32) as u32), bits);
    }

    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    {
        panic!("Wordsize not 32 or 64");
    }
}

/* Architecture-specific hash_64 implementations may replace this symbol. */
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
pub fn hash_ptr(ptr: *const core::ffi::c_void, bits: u32) -> u32 {
    hash_long(ptr as usize, bits)
}

/* This really should be called fold32_ptr; it does no hashing to speak of. */
#[inline]
pub fn hash32_ptr(ptr: *const core::ffi::c_void) -> u32 {
    let mut val = ptr as usize;

    #[cfg(target_pointer_width = "64")]
    {
        val ^= val >> 32;
    }
    val as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
