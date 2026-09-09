/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AES-XTS unauthenticated encryption and decryption
 *
 * Copyright 2026 Google LLC
 */

// Dependencies: `aes_key`, `aes_enckey`, `u8`, `AES_BLOCK_SIZE`, and `bool`
// are supplied by the corresponding translated crypto headers/runtime.

/**
 * struct aes_xts_key - A key prepared for AES-XTS encryption and decryption
 *
 * Note that (depending on the architecture) this typically is around 768 bytes,
 * which makes it a bit too large to allocate on the stack in most cases.
 */
#[repr(C)]
pub struct aes_xts_key {
    /* private: */
    pub main_key: aes_key,
    pub tweak_key: aes_enckey,
}

/**
 * aes_xts_preparekey() - Prepare a key for AES-XTS encryption and decryption
 * @key: (output) The key structure to initialize
 * @in_key: The raw AES-XTS key
 * @key_len: Length of the raw key in bytes
 * @flags: Optional flag XTS_FORBID_WEAK_KEYS to forbid keys whose two halves
 *         are the same.
 *
 * Users should use memzero_explicit() to zeroize the key struct at the end of
 * its lifetime.  (But if this function fails, zeroization is unnecessary.)
 *
 * Context: Any context.
 * Return:
 * * 0 on success
 * * -EINVAL if the key is rejected because its length isn't 32, 64, or (when
 *   FIPS mode isn't enabled) 48; or because its two halves are the same and
 *   either XTS_FORBID_WEAK_KEYS is given or FIPS mode is enabled.
 */
extern "C" {
    pub fn aes_xts_preparekey(
        key: *mut aes_xts_key,
        in_key: *const u8,
        key_len: usize,
        flags: i32,
    ) -> i32;

    /**
     * aes_xts_encrypt() - Encrypt data using AES-XTS
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *      overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to encrypt.  On non-final calls it must be a nonzero
     *      multiple of AES_BLOCK_SIZE.  On the final call it can be any value >=
     *      AES_BLOCK_SIZE, i.e. ciphertext stealing is supported.
     * @tweak: The tweak.  It is updated with the next value, unless @len isn't a
     *         multiple of AES_BLOCK_SIZE in which case the value is unspecified.
     * @key: The key, already prepared using aes_xts_preparekey()
     * @cont: %false to begin encrypting a new message (do the tweak encryption);
     *        %true to continue encrypting a message (skip tweak encryption)
     *
     * This supports both one-shot and incremental encryption.  On the first call,
     * pass @cont = %false.  On any later calls, pass @cont = %true and the updated
     * @tweak; all earlier @len must have been multiples of AES_BLOCK_SIZE.
     *
     * Context: Any context.
     */
    pub fn aes_xts_encrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        tweak: *mut u8,
        key: *const aes_xts_key,
        cont: bool,
    );

    /**
     * aes_xts_decrypt() - Decrypt data using AES-XTS
     * @dst: The destination buffer.  Can be in-place or out-of-place.  For other
     *      overlaps the behavior is unspecified.
     * @src: The source data
     * @len: Number of bytes to decrypt.  On non-final calls it must be a nonzero
     *      multiple of AES_BLOCK_SIZE.  On the final call it can be any value >=
     *      AES_BLOCK_SIZE, i.e. ciphertext stealing is supported.
     * @tweak: The tweak.  It is updated with the next value, unless @len isn't a
     *         multiple of AES_BLOCK_SIZE in which case the value is unspecified.
     * @key: The key, already prepared using aes_xts_preparekey()
     * @cont: %false to begin decrypting a new message (do the tweak encryption);
     *        %true to continue decrypting a message (skip tweak encryption)
     *
     * This supports both one-shot and incremental decryption.  On the first call,
     * pass @cont = %false.  On any later calls, pass @cont = %true and the updated
     * @tweak; all earlier @len must have been multiples of AES_BLOCK_SIZE.
     *
     * Context: Any context.
     */
    pub fn aes_xts_decrypt(
        dst: *mut u8,
        src: *const u8,
        len: usize,
        tweak: *mut u8,
        key: *const aes_xts_key,
        cont: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
