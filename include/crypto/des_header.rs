/* SPDX-License-Identifier: GPL-2.0 */
/*
 * DES & Triple DES EDE Cipher Algorithms.
 */

pub const DES_KEY_SIZE: usize = 8;
pub const DES_EXPKEY_WORDS: usize = 32;
pub const DES_BLOCK_SIZE: usize = 8;

pub const DES3_EDE_KEY_SIZE: usize = 3 * DES_KEY_SIZE;
pub const DES3_EDE_EXPKEY_WORDS: usize = 3 * DES_EXPKEY_WORDS;
pub const DES3_EDE_BLOCK_SIZE: usize = DES_BLOCK_SIZE;

#[repr(C)]
pub struct des_ctx {
    pub expkey: [u32; DES_EXPKEY_WORDS],
}

#[repr(C)]
pub struct des3_ede_ctx {
    pub expkey: [u32; DES3_EDE_EXPKEY_WORDS],
}

unsafe extern "C" {
    pub fn des_encrypt(ctx: *const des_ctx, dst: *mut u8, src: *const u8);
    pub fn des_decrypt(ctx: *const des_ctx, dst: *mut u8, src: *const u8);

    pub fn des3_ede_encrypt(
        dctx: *const des3_ede_ctx,
        dst: *mut u8,
        src: *const u8,
    );
    pub fn des3_ede_decrypt(
        dctx: *const des3_ede_ctx,
        dst: *mut u8,
        src: *const u8,
    );

    /**
     * des_expand_key - Expand a DES input key into a key schedule
     * @ctx: the key schedule
     * @key: buffer containing the input key
     * @keylen: size of the buffer contents
     *
     * Returns: 0 on success, -EINVAL if the input key is rejected and -ENOKEY if
     * the key is accepted but has been found to be weak.
     */
    pub fn des_expand_key(ctx: *mut des_ctx, key: *const u8, keylen: c_uint) -> c_int;

    /**
     * des3_ede_expand_key - Expand a triple DES input key into a key schedule
     * @ctx: the key schedule
     * @key: buffer containing the input key
     * @keylen: size of the buffer contents
     *
     * Returns: 0 on success, -EINVAL if the input key is rejected and -ENOKEY if
     * the key is accepted but has been found to be weak. Note that weak keys will
     * be rejected (and -EINVAL will be returned) when running in FIPS mode.
     */
    pub fn des3_ede_expand_key(
        ctx: *mut des3_ede_ctx,
        key: *const u8,
        keylen: c_uint,
    ) -> c_int;
}

// linux/types.h: unsigned int and int
use core::ffi::{c_int, c_uint};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
