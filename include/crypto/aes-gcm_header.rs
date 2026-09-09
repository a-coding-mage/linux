/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-GCM authenticated encryption and decryption
 *
 * Copyright 2026 Google LLC
 */

// C dependencies: crypto/aes.h, crypto/gcm.h, crypto/gf128hash.h

/**
 * struct aes_gcm_key - A key prepared for AES-GCM encryption and decryption
 */
#[repr(C)]
pub struct aes_gcm_key {
    /* private: */
    pub aes: aes_enckey,
    pub ghash: ghash_key,
    pub authtag_len: usize, /* Length of authentication tags in bytes */
}

/**
 * struct aes_gcm_ctx - Context for incrementally en/decrypting a message
 */
#[repr(C)]
pub struct aes_gcm_ctx {
    /* private: */
    /*
     * Pointer to the key, which is assumed to live at least as long as this
     * struct.
     */
    pub key: *const aes_gcm_key,
    /* The current GHASH context */
    pub ghash: ghash_ctx,
    /*
     * The current counter.  This can be viewed as either a 128-bit big
     * endian counter, or as a 96-bit nonce followed by a 32-bit big endian
     * counter; it doesn't matter, since the last 32-bit word starts at 1,
     * and AES-GCM is undefined for messages that would overflow that part.
     * In practice this means that code optimized for AES-GCM can just
     * increment the last 32-bit word (wrapping at 2^32), but when needed it
     * can still call AES-CTR code that does a 128-bit increment.
     *
     * 'long' alignment is for crypto_xor() to work more efficiently.
     */
    pub ctr: aes_gcm_ctx_ctr,
    /* Buffered keystream for partial block updates */
    pub keystream: [u8; AES_BLOCK_SIZE],
    /* Encrypted counter of 1.  This gets XOR'ed with the tag at the end. */
    pub j0_enc: [u8; AES_BLOCK_SIZE],
    /* Number of associated data bytes processed so far */
    pub ad_len: u64,
    /* Number of en/decrypted bytes processed so far */
    pub data_len: u64,
}

#[repr(C, align(8))]
pub union aes_gcm_ctx_ctr {
    pub ctr: [u8; AES_BLOCK_SIZE],
    pub ctr32: [__be32; AES_BLOCK_SIZE / 4],
}

/**
 * aes_gcm_preparekey() - Prepare a key for AES-GCM encryption and decryption
 * @key: (output) The key structure to initialize
 * @in_key: The raw AES-GCM key
 * @key_len: Length of the raw key in bytes: 16, 24, or 32
 * @authtag_len: Length of the authentication tag in bytes:
 *		 4, 8, 12, 13, 14, 15, or 16.  16 is recommended.
 *
 * Users should use memzero_explicit() to zeroize the key struct at the end of
 * its lifetime.  (But if this function fails, zeroization is unnecessary.)
 *
 * Context: Any context.
 * Return:
 * * 0 on success
 * * -EINVAL if either of the lengths is invalid
 */
extern "C" {
    pub fn aes_gcm_preparekey(
        key: *mut aes_gcm_key,
        in_key: *const u8,
        key_len: usize,
        authtag_len: usize,
    ) -> i32;

    pub fn aes_gcm_encrypt(
        dst: *mut u8,
        src: *const u8,
        data_len: usize,
        authtag: *mut u8,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        key: *const aes_gcm_key,
    );

    pub fn aes_gcm_decrypt(
        dst: *mut u8,
        src: *const u8,
        data_len: usize,
        authtag: *const u8,
        ad: *const u8,
        ad_len: usize,
        nonce: *const u8,
        key: *const aes_gcm_key,
    ) -> i32;

    pub fn aes_gcm_init(
        ctx: *mut aes_gcm_ctx,
        nonce: *const u8,
        key: *const aes_gcm_key,
    );

    pub fn aes_gcm_auth_update(ctx: *mut aes_gcm_ctx, ad: *const u8, len: usize);

    pub fn aes_gcm_encrypt_update(
        ctx: *mut aes_gcm_ctx,
        dst: *mut u8,
        src: *const u8,
        len: usize,
    );

    pub fn aes_gcm_decrypt_update(
        ctx: *mut aes_gcm_ctx,
        dst: *mut u8,
        src: *const u8,
        len: usize,
    );

    pub fn aes_gcm_encrypt_final(ctx: *mut aes_gcm_ctx, authtag: *mut u8);

    pub fn aes_gcm_decrypt_final(ctx: *mut aes_gcm_ctx, authtag: *const u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
