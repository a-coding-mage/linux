/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Common values for the SM4 algorithm
 * Copyright (C) 2018 ARM Limited or its affiliates.
 * Copyright (c) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
 */

// C header guard: _CRYPTO_SM4_H
// C dependencies: <linux/types.h>, <linux/crypto.h>

pub const SM4_KEY_SIZE: usize = 16;
pub const SM4_BLOCK_SIZE: usize = 16;
pub const SM4_RKEY_WORDS: usize = 32;

#[repr(C)]
pub struct sm4_ctx {
    pub rkey_enc: [u32; SM4_RKEY_WORDS],
    pub rkey_dec: [u32; SM4_RKEY_WORDS],
}

extern "C" {
    pub static crypto_sm4_fk: [u32; 0];
    pub static crypto_sm4_ck: [u32; 0];
    pub static crypto_sm4_sbox: [u8; 0];

    /**
     * sm4_expandkey - Expands the SM4 key as described in GB/T 32907-2016
     * @ctx: The location where the computed key will be stored.
     * @in_key: The supplied key.
     * @key_len: The length of the supplied key.
     *
     * Returns 0 on success. The function fails only if an invalid key size (or
     * pointer) is supplied.
     */
    pub fn sm4_expandkey(
        ctx: *mut sm4_ctx,
        in_key: *const u8,
        key_len: u32,
    ) -> i32;

    /**
     * sm4_crypt_block - Encrypt or decrypt a single SM4 block
     * @rk: The rkey_enc for encrypt or rkey_dec for decrypt
     * @out: Buffer to store output data
     * @in: Buffer containing the input data
     */
    pub fn sm4_crypt_block(rk: *const u32, out: *mut u8, input: *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
