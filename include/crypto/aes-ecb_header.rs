/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-ECB unauthenticated encryption and decryption
 *
 * Copyright 2026 Google LLC
 */

// C dependency: <crypto/aes.h>

/**
 * aes_ecb_encrypt() - Encrypt data using AES-ECB
 * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
 *\t overlaps the behavior is unspecified.
 * @src: The source data
 * @len: Number of bytes to encrypt.  Must be a multiple of AES_BLOCK_SIZE.
 * @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
 *
 * ECB mode is insecure by itself.  This function exists only for compatibility
 * with legacy protocols and for internal use by other modes.
 *
 * This supports incremental encryption, but the length of each chunk must be a
 * multiple of AES_BLOCK_SIZE.
 *
 * Context: Any context.
 */
extern "C" {
    pub fn aes_ecb_encrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        key: aes_encrypt_arg,
    );

    /**
     * aes_ecb_decrypt() - Decrypt data using AES-ECB
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *\t overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to decrypt.  Must be a multiple of AES_BLOCK_SIZE.
     * @key: The key, already prepared using aes_preparekey()
     *
     * ECB mode is insecure by itself.  This function exists only for compatibility
     * with legacy protocols and for internal use by other modes.
     *
     * This supports incremental decryption, but the length of each chunk must be a
     * multiple of AES_BLOCK_SIZE.
     *
     * Context: Any context.
     */
    pub fn aes_ecb_decrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        key: *const struct aes_key,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
