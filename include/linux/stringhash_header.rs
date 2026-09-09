/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Routines for hashing strings of bytes to a 32-bit hash value.
 *
 * These hash functions are NOT GUARANTEED STABLE between kernel
 * versions, architectures, or even repeated boots of the same kernel.
 * (E.g. they may depend on boot-time hardware detection or be
 * deliberately randomized.)
 *
 * They are also not intended to be secure against collisions caused by
 * malicious inputs; much slower hash functions are required for that.
 *
 * They are optimized for pathname components, meaning short strings.
 * Even if a majority of files have longer names, the dynamic profile of
 * pathname components skews short due to short directory names.
 * (E.g. /usr/lib/libsesquipedalianism.so.3.141.)
 */

/*
 * Version 1: one byte at a time.  Example of use:
 *
 * unsigned long hash = init_name_hash;
 * while (*p)
 *	hash = partial_name_hash(tolower(*p++), hash);
 * hash = end_name_hash(hash);
 *
 * Although this is designed for bytes, fs/hfsplus/unicode.c
 * abuses it to hash 16-bit values.
 */

use core::ffi::{c_char, c_ulong, c_void};

/* Hash courtesy of the R5 hash in reiserfs modulo sign bits */
#[inline]
pub const fn init_name_hash(salt: c_ulong) -> c_ulong {
    salt
}

/* partial hash update function. Assume roughly 4 bits per character */
#[inline]
pub fn partial_name_hash(c: c_ulong, prevhash: c_ulong) -> c_ulong {
    prevhash
        .wrapping_add(c.wrapping_shl(4))
        .wrapping_add(c.wrapping_shr(4))
        .wrapping_mul(11)
}

/*
 * Finally: cut down the number of bits to a int value (and try to avoid
 * losing bits).  This also has the property (wanted by the dcache)
 * that the msbits make a good hash table index.
 */
#[inline]
pub unsafe fn end_name_hash(hash: c_ulong) -> u32 {
    hash_long(hash, 32)
}

/*
 * Version 2: One word (32 or 64 bits) at a time.
 * If CONFIG_DCACHE_WORD_ACCESS is defined (meaning <asm/word-at-a-time.h>
 * exists, which describes major Linux platforms like x86 and ARM), then
 * this computes a different hash function much faster.
 *
 * If not set, this falls back to a wrapper around the preceding.
 */
extern "C" {
    /* __pure */
    pub fn full_name_hash(salt: *const c_void, name: *const c_char, len: u32) -> u32;

    /* __pure */
    pub fn hashlen_string(salt: *const c_void, name: *const c_char) -> u64;

    fn hash_long(value: c_ulong, bits: u32) -> u32;
}

/*
 * A hash_len is a u64 with the hash of a string in the low
 * half and the length in the high half.
 */
#[inline]
pub const fn hashlen_hash(hashlen: u64) -> u32 {
    hashlen as u32
}

#[inline]
pub const fn hashlen_len(hashlen: u64) -> u32 {
    (hashlen >> 32) as u32
}

#[inline]
pub const fn hashlen_create(hash: u32, len: u32) -> u64 {
    ((len as u64) << 32) | (hash as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
