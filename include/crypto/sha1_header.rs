/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for SHA-1 algorithms
 */

// Dependency intent: the C header includes <linux/types.h> for u8, u32, and u64.

pub const SHA1_DIGEST_SIZE: usize = 20;
pub const SHA1_BLOCK_SIZE: usize = 64;
pub const SHA1_STATE_SIZE: usize = 28; // offsetof(struct sha1_state, buffer)

pub const SHA1_H0: u32 = 0x67452301;
pub const SHA1_H1: u32 = 0xefcdab89;
pub const SHA1_H2: u32 = 0x98badcfe;
pub const SHA1_H3: u32 = 0x10325476;
pub const SHA1_H4: u32 = 0xc3d2e1f0;

extern "C" {
    pub static sha1_zero_message_hash: [u8; SHA1_DIGEST_SIZE];
}

#[repr(C)]
pub struct sha1_state {
    pub state: [u32; SHA1_DIGEST_SIZE / 4],
    pub count: u64,
    pub buffer: [u8; SHA1_BLOCK_SIZE],
}

/* State for the SHA-1 compression function */
#[repr(C)]
pub struct sha1_block_state {
    pub h: [u32; SHA1_DIGEST_SIZE / 4],
}

/**
 * struct sha1_ctx - Context for hashing a message with SHA-1
 * @state: the compression function state
 * @bytecount: number of bytes processed so far
 * @buf: partial block buffer; bytecount % SHA1_BLOCK_SIZE bytes are valid
 */
#[repr(C)]
pub struct sha1_ctx {
    pub state: sha1_block_state,
    pub bytecount: u64,
    pub buf: [u8; SHA1_BLOCK_SIZE],
}

extern "C" {
    pub fn sha1_init(ctx: *mut sha1_ctx);
    pub fn sha1_update(ctx: *mut sha1_ctx, data: *const u8, len: usize);
    pub fn sha1_final(ctx: *mut sha1_ctx, out: *mut u8);
    pub fn sha1(data: *const u8, len: usize, out: *mut u8);
}

#[repr(C)]
pub struct hmac_sha1_key {
    pub istate: sha1_block_state,
    pub ostate: sha1_block_state,
}

#[repr(C)]
pub struct hmac_sha1_ctx {
    pub sha_ctx: sha1_ctx,
    pub ostate: sha1_block_state,
}

extern "C" {
    pub fn hmac_sha1_preparekey(
        key: *mut hmac_sha1_key,
        raw_key: *const u8,
        raw_key_len: usize,
    );
    pub fn hmac_sha1_init(ctx: *mut hmac_sha1_ctx, key: *const hmac_sha1_key);
    pub fn hmac_sha1_init_usingrawkey(
        ctx: *mut hmac_sha1_ctx,
        raw_key: *const u8,
        raw_key_len: usize,
    );
}

pub unsafe fn hmac_sha1_update(ctx: *mut hmac_sha1_ctx, data: *const u8, data_len: usize) {
    sha1_update(&mut (*ctx).sha_ctx, data, data_len);
}

extern "C" {
    pub fn hmac_sha1_final(ctx: *mut hmac_sha1_ctx, out: *mut u8);
    pub fn hmac_sha1(
        key: *const hmac_sha1_key,
        data: *const u8,
        data_len: usize,
        out: *mut u8,
    );
    pub fn hmac_sha1_usingrawkey(
        raw_key: *const u8,
        raw_key_len: usize,
        data: *const u8,
        data_len: usize,
        out: *mut u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
