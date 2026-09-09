/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-CBC and AES-CBC-CTS unauthenticated encryption and decryption
 *
 * Copyright 2026 Google LLC
 */

// Dependency intent: declarations from <crypto/aes.h> are supplied externally.

/**
 * aes_cbc_encrypt() - Encrypt data using AES-CBC
 * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
 *\t overlaps the behavior is unspecified.
 * @src: The source data
 * @len: Number of bytes to encrypt.  Must be a multiple of AES_BLOCK_SIZE.
 * @iv: The initialization vector.  It is updated with the next value, i.e. the
 * last ciphertext block (or left unchanged if @len == 0).
 * @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
 *
 * This supports incremental encryption.  The length of each chunk must be a
 * multiple of AES_BLOCK_SIZE, and the updated @iv must be passed in each time.
 *
 * Context: Any context.
 */
extern "C" {
    pub fn aes_cbc_encrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        iv: *mut u8,
        key: aes_encrypt_arg,
    );

    /**
     * aes_cbc_decrypt() - Decrypt data using AES-CBC
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *\t overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to decrypt.  Must be a multiple of AES_BLOCK_SIZE.
     * @iv: The initialization vector.  It is updated with the next value, i.e. the
     * last ciphertext block (or left unchanged if @len == 0).
     * @key: The key, already prepared using aes_preparekey()
     *
     * This supports incremental decryption.  The length of each chunk must be a
     * multiple of AES_BLOCK_SIZE, and the updated @iv must be passed in each time.
     *
     * Context: Any context.
     */
    pub fn aes_cbc_decrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        iv: *mut u8,
        key: *const aes_key,
    );

    /**
     * aes_cbc_cts_encrypt() - Encrypt data using AES-CBC-CTS (CS3 variant)
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *\t overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to encrypt, at least AES_BLOCK_SIZE
     * @iv: The initialization vector, clobbered by this function
     * @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
     *
     * Context: Any context.
     */
    pub fn aes_cbc_cts_encrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        iv: *mut u8,
        key: aes_encrypt_arg,
    );

    /**
     * aes_cbc_cts_decrypt() - Decrypt data using AES-CBC-CTS (CS3 variant)
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *\t overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to decrypt, at least AES_BLOCK_SIZE
     * @iv: The initialization vector, clobbered by this function
     * @key: The key, already prepared using aes_preparekey()
     *
     * Context: Any context.
     */
    pub fn aes_cbc_cts_decrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        iv: *mut u8,
        key: *const aes_key,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
