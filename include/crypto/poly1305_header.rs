/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for the Poly1305 algorithm
 */

// The C header includes <linux/types.h>; the corresponding integer types are
// represented by Rust's fixed-width integer types here.

pub const POLY1305_BLOCK_SIZE: usize = 16;
pub const POLY1305_KEY_SIZE: usize = 32;
pub const POLY1305_DIGEST_SIZE: usize = 16;

/* The poly1305_key and poly1305_state types are mostly opaque and
 * implementation-defined. Limbs might be in base 2^64 or base 2^26, or
 * different yet. The union type provided keeps these 64-bit aligned for the
 * case in which this is implemented using 64x64 multiplies.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union poly1305_key_union {
    pub r: [u32; 5],
    pub r64: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_key {
    pub value: poly1305_key_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_core_key {
    pub key: poly1305_key,
    pub precomputed_s: poly1305_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union poly1305_state_union {
    pub h: [u32; 5],
    pub h64: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_state {
    pub value: poly1305_state_union,
}

/* Combined state for block function. */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_block_state {
    /* accumulator */
    pub h: poly1305_state,
    /* key */
    pub key: poly1305_block_state_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union poly1305_block_state_key {
    pub opaque_r: [poly1305_key; CONFIG_CRYPTO_LIB_POLY1305_RSIZE],
    pub core_r: poly1305_core_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct poly1305_desc_ctx {
    /* partial buffer */
    pub buf: [u8; POLY1305_BLOCK_SIZE],
    /* bytes used in partial buffer */
    pub buflen: u32,
    /* finalize key */
    pub s: [u32; 4],
    pub state: poly1305_block_state,
}

extern "C" {
    pub fn poly1305_init(desc: *mut poly1305_desc_ctx, key: *const u8);
    pub fn poly1305_update(desc: *mut poly1305_desc_ctx, src: *const u8, nbytes: u32);
    pub fn poly1305_final(desc: *mut poly1305_desc_ctx, digest: *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
