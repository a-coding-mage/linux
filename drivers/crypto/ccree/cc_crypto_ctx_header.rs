/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* Translated from cc_crypto_ctx.h. */

pub const CC_DRV_DES_IV_SIZE: i32 = 8;
pub const CC_DRV_DES_BLOCK_SIZE: i32 = 8;

pub const CC_DRV_DES_ONE_KEY_SIZE: i32 = 8;
pub const CC_DRV_DES_DOUBLE_KEY_SIZE: i32 = 16;
pub const CC_DRV_DES_TRIPLE_KEY_SIZE: i32 = 24;
pub const CC_DRV_DES_KEY_SIZE_MAX: i32 = CC_DRV_DES_TRIPLE_KEY_SIZE;

pub const CC_AES_IV_SIZE: i32 = 16;
pub const CC_AES_IV_SIZE_WORDS: i32 = CC_AES_IV_SIZE >> 2;

pub const CC_AES_BLOCK_SIZE: i32 = 16;
pub const CC_AES_BLOCK_SIZE_WORDS: i32 = 4;

pub const CC_AES_128_BIT_KEY_SIZE: i32 = 16;
pub const CC_AES_128_BIT_KEY_SIZE_WORDS: i32 = CC_AES_128_BIT_KEY_SIZE >> 2;
pub const CC_AES_192_BIT_KEY_SIZE: i32 = 24;
pub const CC_AES_192_BIT_KEY_SIZE_WORDS: i32 = CC_AES_192_BIT_KEY_SIZE >> 2;
pub const CC_AES_256_BIT_KEY_SIZE: i32 = 32;
pub const CC_AES_256_BIT_KEY_SIZE_WORDS: i32 = CC_AES_256_BIT_KEY_SIZE >> 2;
pub const CC_AES_KEY_SIZE_MAX: i32 = CC_AES_256_BIT_KEY_SIZE;
pub const CC_AES_KEY_SIZE_WORDS_MAX: i32 = CC_AES_KEY_SIZE_MAX >> 2;

pub const CC_MD5_DIGEST_SIZE: i32 = 16;
pub const CC_SHA1_DIGEST_SIZE: i32 = 20;
pub const CC_SHA224_DIGEST_SIZE: i32 = 28;
pub const CC_SHA256_DIGEST_SIZE: i32 = 32;
pub const CC_SHA256_DIGEST_SIZE_IN_WORDS: i32 = 8;
pub const CC_SHA384_DIGEST_SIZE: i32 = 48;
pub const CC_SHA512_DIGEST_SIZE: i32 = 64;

pub const CC_SHA1_BLOCK_SIZE: i32 = 64;
pub const CC_SHA1_BLOCK_SIZE_IN_WORDS: i32 = 16;
pub const CC_MD5_BLOCK_SIZE: i32 = 64;
pub const CC_MD5_BLOCK_SIZE_IN_WORDS: i32 = 16;
pub const CC_SHA224_BLOCK_SIZE: i32 = 64;
pub const CC_SHA256_BLOCK_SIZE: i32 = 64;
pub const CC_SHA256_BLOCK_SIZE_IN_WORDS: i32 = 16;
pub const CC_SHA1_224_256_BLOCK_SIZE: i32 = 64;
pub const CC_SHA384_BLOCK_SIZE: i32 = 128;
pub const CC_SHA512_BLOCK_SIZE: i32 = 128;

pub const CC_DIGEST_SIZE_MAX: i32 = CC_SHA512_DIGEST_SIZE;
pub const CC_HASH_BLOCK_SIZE_MAX: i32 = CC_SHA512_BLOCK_SIZE; /* 1024b */
pub const CC_HMAC_BLOCK_SIZE_MAX: i32 = CC_HASH_BLOCK_SIZE_MAX;
pub const CC_DRV_ALG_MAX_BLOCK_SIZE: i32 = CC_HASH_BLOCK_SIZE_MAX;

pub const CC_CPP_NUM_SLOTS: i32 = 8;
pub const CC_CPP_NUM_ALGS: i32 = 2;

#[repr(i32)]
pub enum cc_cpp_alg {
    CC_CPP_SM4 = 1,
    CC_CPP_AES = 0,
}

/* S32_MAX is supplied by the translated Linux types dependency. */
#[repr(i32)]
pub enum drv_engine_type {
    DRV_ENGINE_NULL = 0,
    DRV_ENGINE_AES = 1,
    DRV_ENGINE_DES = 2,
    DRV_ENGINE_HASH = 3,
    DRV_ENGINE_RC4 = 4,
    DRV_ENGINE_DOUT = 5,
    DRV_ENGINE_RESERVE32B = S32_MAX,
}

#[repr(i32)]
pub enum drv_crypto_alg {
    DRV_CRYPTO_ALG_NULL = -1,
    DRV_CRYPTO_ALG_AES = 0,
    DRV_CRYPTO_ALG_DES = 1,
    DRV_CRYPTO_ALG_HASH = 2,
    DRV_CRYPTO_ALG_C2 = 3,
    DRV_CRYPTO_ALG_HMAC = 4,
    DRV_CRYPTO_ALG_AEAD = 5,
    DRV_CRYPTO_ALG_BYPASS = 6,
    DRV_CRYPTO_ALG_NUM = 7,
    DRV_CRYPTO_ALG_RESERVE32B = S32_MAX,
}

#[repr(i32)]
pub enum drv_crypto_direction {
    DRV_CRYPTO_DIRECTION_NULL = -1,
    DRV_CRYPTO_DIRECTION_ENCRYPT = 0,
    DRV_CRYPTO_DIRECTION_DECRYPT = 1,
    DRV_CRYPTO_DIRECTION_DECRYPT_ENCRYPT = 3,
    DRV_CRYPTO_DIRECTION_RESERVE32B = S32_MAX,
}

#[repr(i32)]
pub enum drv_cipher_mode {
    DRV_CIPHER_NULL_MODE = -1,
    DRV_CIPHER_ECB = 0,
    DRV_CIPHER_CBC = 1,
    DRV_CIPHER_CTR = 2,
    DRV_CIPHER_CBC_MAC = 3,
    DRV_CIPHER_XTS = 4,
    DRV_CIPHER_XCBC_MAC = 5,
    DRV_CIPHER_OFB = 6,
    DRV_CIPHER_CMAC = 7,
    DRV_CIPHER_CCM = 8,
    DRV_CIPHER_CBC_CTS = 11,
    DRV_CIPHER_GCTR = 12,
    DRV_CIPHER_ESSIV = 13,
    DRV_CIPHER_RESERVE32B = S32_MAX,
}

#[repr(i32)]
pub enum drv_hash_mode {
    DRV_HASH_NULL = -1,
    DRV_HASH_SHA1 = 0,
    DRV_HASH_SHA256 = 1,
    DRV_HASH_SHA224 = 2,
    DRV_HASH_SHA512 = 3,
    DRV_HASH_SHA384 = 4,
    DRV_HASH_MD5 = 5,
    DRV_HASH_CBC_MAC = 6,
    DRV_HASH_XCBC_MAC = 7,
    DRV_HASH_CMAC = 8,
    DRV_HASH_SM3 = 9,
    DRV_HASH_MODE_NUM = 10,
    DRV_HASH_RESERVE32B = S32_MAX,
}

#[repr(i32)]
pub enum drv_hash_hw_mode {
    DRV_HASH_HW_MD5 = 0,
    DRV_HASH_HW_SHA1 = 1,
    DRV_HASH_HW_SHA256 = 2,
    DRV_HASH_HW_SHA224 = 10,
    DRV_HASH_HW_SHA512 = 4,
    DRV_HASH_HW_SHA384 = 12,
    DRV_HASH_HW_GHASH = 6,
    DRV_HASH_HW_SM3 = 14,
    DRV_HASH_HW_RESERVE32B = S32_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
