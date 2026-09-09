/* SPDX-License-Identifier: GPL-2.0 */

// External dependencies supplied by the surrounding translation unit:
// crypto/hash.h and linux/types.h

pub const MD5_DIGEST_SIZE: usize = 16;
pub const MD5_HMAC_BLOCK_SIZE: usize = 64;
pub const MD5_BLOCK_SIZE: usize = 64;
pub const MD5_BLOCK_WORDS: usize = 16;
pub const MD5_HASH_WORDS: usize = 4;
pub const MD5_STATE_SIZE: usize = 24;

pub const MD5_H0: u32 = 0x67452301;
pub const MD5_H1: u32 = 0xefcdab89;
pub const MD5_H2: u32 = 0x98badcfe;
pub const MD5_H3: u32 = 0x10325476;

// CRYPTO_MD5_STATESIZE = CRYPTO_HASH_STATESIZE(MD5_STATE_SIZE, MD5_HMAC_BLOCK_SIZE)

extern "C" {
    pub static md5_zero_message_hash: [u8; MD5_DIGEST_SIZE];
}

#[repr(C)]
pub struct md5_state {
    pub hash: [u32; MD5_HASH_WORDS],
    pub byte_count: u64,
    pub block: [u32; MD5_BLOCK_WORDS],
}

/* State for the MD5 compression function */
#[repr(C)]
pub struct md5_block_state {
    pub h: [u32; MD5_HASH_WORDS],
}

/**
 * struct md5_ctx - Context for hashing a message with MD5
 * @state: the compression function state
 * @bytecount: number of bytes processed so far
 * @buf: partial block buffer; bytecount % MD5_BLOCK_SIZE bytes are valid
 */
#[repr(C, align(8))]
pub struct md5_ctx {
    pub state: md5_block_state,
    pub bytecount: u64,
    pub buf: [u8; MD5_BLOCK_SIZE],
}

extern "C" {
    pub fn md5_init(ctx: *mut md5_ctx);
    pub fn md5_update(ctx: *mut md5_ctx, data: *const u8, len: usize);
    pub fn md5_final(ctx: *mut md5_ctx, out: *mut u8);
    pub fn md5(data: *const u8, len: usize, out: *mut u8);
}

/**
 * struct hmac_md5_key - Prepared key for HMAC-MD5
 * @istate: private
 * @ostate: private
 */
#[repr(C)]
pub struct hmac_md5_key {
    pub istate: md5_block_state,
    pub ostate: md5_block_state,
}

/**
 * struct hmac_md5_ctx - Context for computing HMAC-MD5 of a message
 * @hash_ctx: private
 * @ostate: private
 */
#[repr(C)]
pub struct hmac_md5_ctx {
    pub hash_ctx: md5_ctx,
    pub ostate: md5_block_state,
}

extern "C" {
    pub fn hmac_md5_preparekey(
        key: *mut hmac_md5_key,
        raw_key: *const u8,
        raw_key_len: usize,
    );
    pub fn hmac_md5_init(ctx: *mut hmac_md5_ctx, key: *const hmac_md5_key);
    pub fn hmac_md5_init_usingrawkey(
        ctx: *mut hmac_md5_ctx,
        raw_key: *const u8,
        raw_key_len: usize,
    );
}

/**
 * hmac_md5_update() - Update an HMAC-MD5 context with message data
 */
#[inline]
pub unsafe fn hmac_md5_update(ctx: *mut hmac_md5_ctx, data: *const u8, data_len: usize) {
    md5_update(&mut (*ctx).hash_ctx, data, data_len);
}

extern "C" {
    pub fn hmac_md5_final(ctx: *mut hmac_md5_ctx, out: *mut u8);
    pub fn hmac_md5(
        key: *const hmac_md5_key,
        data: *const u8,
        data_len: usize,
        out: *mut u8,
    );
    pub fn hmac_md5_usingrawkey(
        raw_key: *const u8,
        raw_key_len: usize,
        data: *const u8,
        data_len: usize,
        out: *mut u8,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
