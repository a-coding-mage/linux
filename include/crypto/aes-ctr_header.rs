/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-CTR and AES-XCTR stream ciphers
 *
 * Copyright 2026 Google LLC
 */

// Dependency provided by crypto/aes.h.

/**
 * aes_ctr() - AES-CTR en/decryption
 * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
 *\toverlaps the behavior is unspecified.
 * @src: The source data
 * @len: Number of bytes to en/decrypt
 * @ctr: The counter.  It will be incremented by ceil(@len / AES_BLOCK_SIZE).
 * @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
 *
 * This implements AES in counter mode with a 128-bit big endian counter.
 *
 * This exists only for use by the implementation of modes built on top of CTR
 * (e.g., GCM and CCM) and some legacy protocols that use CTR mode directly.
 * Callers are expected to know how to use CTR mode appropriately, including
 * choosing (key, counter) pairs appropriately to avoid keystream reuse.
 *
 * This supports incremental en/decryption.  The length of each non-final chunk
 * must be a multiple of AES_BLOCK_SIZE, and the updated @ctr must be passed in
 * each time.
 *
 * Context: Any context.
 */
extern "C" {
    pub fn aes_ctr(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        ctr: *mut u8,
        key: aes_encrypt_arg,
    );

    /**
     * aes_xctr() - AES-XCTR en/decryption
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *\toverlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to en/decrypt
     * @ctr: The block counter (in host endianness).  For the first call, set it to
     *\t1.  It will be incremented by ceil(@len / AES_BLOCK_SIZE).
     * @iv: The initialization vector
     * @key: The key, already prepared using aes_preparekey() or aes_prepareenckey()
     *
     * This implements AES in XOR Counter mode, as specified in the paper
     * "Length-preserving encryption with HCTR2"
     * (https://eprint.iacr.org/2021/1441.pdf).
     *
     * This exists only for use by the implementation of modes built on top of XCTR.
     * Callers are expected to know how to use XCTR mode appropriately, including
     * choosing (key, IV) pairs appropriately to avoid keystream reuse.
     *
     * This supports incremental en/decryption.  The length of each non-final chunk
     * must be a multiple of AES_BLOCK_SIZE, and the updated @ctr must be passed in
     * each time.
     *
     * Context: Any context.
     */
    pub fn aes_xctr(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        ctr: *mut u64,
        iv: *const u8,
        key: aes_encrypt_arg,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
