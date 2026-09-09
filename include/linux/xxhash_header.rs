/*
 * xxHash - Extremely Fast Hash algorithm
 * Copyright (C) 2012-2016, Yann Collet.
 *
 * BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)
 *
 * This Rust translation preserves the declarations and inline implementation
 * from the original C header.
 */

/* The original header includes <linux/types.h>. Rust primitive integer types
 * provide the corresponding fixed-width types used by this interface. */

/*-****************************
 * Simple Hash Functions
 *****************************/

/**
 * xxh32() - calculate the 32-bit hash of the input with a given seed.
 */
pub unsafe extern "C" {
    pub fn xxh32(input: *const core::ffi::c_void, length: usize, seed: u32) -> u32;
}

/**
 * xxh64() - calculate the 64-bit hash of the input with a given seed.
 */
pub unsafe extern "C" {
    pub fn xxh64(input: *const core::ffi::c_void, length: usize, seed: u64) -> u64;
}

/**
 * xxhash() - calculate wordsize hash of the input with a given seed.
 *
 * The original implementation selects xxh64() when BITS_PER_LONG == 64 and
 * xxh32() otherwise. The target pointer width is the Rust equivalent here.
 */
#[inline]
pub unsafe fn xxhash(
    input: *const core::ffi::c_void,
    length: usize,
    seed: u64,
) -> usize {
    #[cfg(target_pointer_width = "64")]
    {
        xxh64(input, length, seed) as usize
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        xxh32(input, length, seed as u32) as usize
    }
}

/*-****************************
 * Streaming Hash Functions
 *****************************/

/*
 * These definitions are only meant to allow allocation of XXH state
 * statically, on stack, or in a struct for example.
 * Do not use members directly.
 */

/**
 * struct xxh64_state - private xxh64 state, do not use members directly
 */
#[repr(C)]
pub struct xxh64_state {
    pub total_len: u64,
    pub v1: u64,
    pub v2: u64,
    pub v3: u64,
    pub v4: u64,
    pub mem64: [u64; 4],
    pub memsize: u32,
}

/**
 * xxh64_reset() - reset the xxh64 state to start a new hashing operation
 */
pub unsafe extern "C" {
    pub fn xxh64_reset(state: *mut xxh64_state, seed: u64);
}

/**
 * xxh64_update() - hash the data given and update the xxh64 state
 */
pub unsafe extern "C" {
    pub fn xxh64_update(
        state: *mut xxh64_state,
        input: *const core::ffi::c_void,
        length: usize,
    ) -> core::ffi::c_int;
}

/**
 * xxh64_digest() - produce the current xxh64 hash
 */
pub unsafe extern "C" {
    pub fn xxh64_digest(state: *const xxh64_state) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
