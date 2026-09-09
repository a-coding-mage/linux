/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the OpenIB.org BSD
 * license.
 */

// Dependency supplied by the surrounding UAPI translation: C __u16 is u16.

/* TLS socket options */
pub const TLS_TX: i32 = 1;
pub const TLS_RX: i32 = 2;
pub const TLS_TX_ZEROCOPY_RO: i32 = 3;
pub const TLS_RX_EXPECT_NO_PAD: i32 = 4;
pub const TLS_TX_MAX_PAYLOAD_LEN: i32 = 5;

/* Supported versions */
pub const TLS_1_2_VERSION_MAJOR: u16 = 0x3;
pub const TLS_1_2_VERSION_MINOR: u16 = 0x3;
pub const TLS_1_2_VERSION: u16 = ((TLS_1_2_VERSION_MAJOR & 0xff) << 8) | (TLS_1_2_VERSION_MINOR & 0xff);
pub const TLS_1_3_VERSION_MAJOR: u16 = 0x3;
pub const TLS_1_3_VERSION_MINOR: u16 = 0x4;
pub const TLS_1_3_VERSION: u16 = ((TLS_1_3_VERSION_MAJOR & 0xff) << 8) | (TLS_1_3_VERSION_MINOR & 0xff);

/* Supported ciphers */
pub const TLS_CIPHER_AES_GCM_128: i32 = 51;
pub const TLS_CIPHER_AES_GCM_128_IV_SIZE: usize = 8;
pub const TLS_CIPHER_AES_GCM_128_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_AES_GCM_128_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_AES_GCM_128_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_AES_GCM_256: i32 = 52;
pub const TLS_CIPHER_AES_GCM_256_IV_SIZE: usize = 8;
pub const TLS_CIPHER_AES_GCM_256_KEY_SIZE: usize = 32;
pub const TLS_CIPHER_AES_GCM_256_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_AES_GCM_256_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_AES_CCM_128: i32 = 53;
pub const TLS_CIPHER_AES_CCM_128_IV_SIZE: usize = 8;
pub const TLS_CIPHER_AES_CCM_128_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_AES_CCM_128_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_AES_CCM_128_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_CHACHA20_POLY1305: i32 = 54;
pub const TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE: usize = 12;
pub const TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE: usize = 32;
pub const TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE: usize = 0;
pub const TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_SM4_GCM: i32 = 55;
pub const TLS_CIPHER_SM4_GCM_IV_SIZE: usize = 8;
pub const TLS_CIPHER_SM4_GCM_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_SM4_GCM_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_SM4_GCM_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_SM4_CCM: i32 = 56;
pub const TLS_CIPHER_SM4_CCM_IV_SIZE: usize = 8;
pub const TLS_CIPHER_SM4_CCM_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_SM4_CCM_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_SM4_CCM_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_ARIA_GCM_128: i32 = 57;
pub const TLS_CIPHER_ARIA_GCM_128_IV_SIZE: usize = 8;
pub const TLS_CIPHER_ARIA_GCM_128_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_ARIA_GCM_128_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_ARIA_GCM_128_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE: usize = 8;
pub const TLS_CIPHER_ARIA_GCM_256: i32 = 58;
pub const TLS_CIPHER_ARIA_GCM_256_IV_SIZE: usize = 8;
pub const TLS_CIPHER_ARIA_GCM_256_KEY_SIZE: usize = 32;
pub const TLS_CIPHER_ARIA_GCM_256_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_ARIA_GCM_256_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE: usize = 8;

pub const TLS_SET_RECORD_TYPE: i32 = 1;
pub const TLS_GET_RECORD_TYPE: i32 = 2;

#[repr(C)]
pub struct tls_crypto_info { pub version: u16, pub cipher_type: u16 }

macro_rules! tls_crypto_info_struct {
    ($name:ident, $iv:expr, $key:expr, $salt:expr, $seq:expr) => {
        #[repr(C)]
        pub struct $name {
            pub info: tls_crypto_info,
            pub iv: [u8; $iv],
            pub key: [u8; $key],
            pub salt: [u8; $salt],
            pub rec_seq: [u8; $seq],
        }
    };
}

tls_crypto_info_struct!(tls12_crypto_info_aes_gcm_128, TLS_CIPHER_AES_GCM_128_IV_SIZE, TLS_CIPHER_AES_GCM_128_KEY_SIZE, TLS_CIPHER_AES_GCM_128_SALT_SIZE, TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_aes_gcm_256, TLS_CIPHER_AES_GCM_256_IV_SIZE, TLS_CIPHER_AES_GCM_256_KEY_SIZE, TLS_CIPHER_AES_GCM_256_SALT_SIZE, TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_aes_ccm_128, TLS_CIPHER_AES_CCM_128_IV_SIZE, TLS_CIPHER_AES_CCM_128_KEY_SIZE, TLS_CIPHER_AES_CCM_128_SALT_SIZE, TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_chacha20_poly1305, TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE, TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE, TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE, TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_sm4_gcm, TLS_CIPHER_SM4_GCM_IV_SIZE, TLS_CIPHER_SM4_GCM_KEY_SIZE, TLS_CIPHER_SM4_GCM_SALT_SIZE, TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_sm4_ccm, TLS_CIPHER_SM4_CCM_IV_SIZE, TLS_CIPHER_SM4_CCM_KEY_SIZE, TLS_CIPHER_SM4_CCM_SALT_SIZE, TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_aria_gcm_128, TLS_CIPHER_ARIA_GCM_128_IV_SIZE, TLS_CIPHER_ARIA_GCM_128_KEY_SIZE, TLS_CIPHER_ARIA_GCM_128_SALT_SIZE, TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE);
tls_crypto_info_struct!(tls12_crypto_info_aria_gcm_256, TLS_CIPHER_ARIA_GCM_256_IV_SIZE, TLS_CIPHER_ARIA_GCM_256_KEY_SIZE, TLS_CIPHER_ARIA_GCM_256_SALT_SIZE, TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE);

pub const TLS_INFO_UNSPEC: i32 = 0;
pub const TLS_INFO_VERSION: i32 = 1;
pub const TLS_INFO_CIPHER: i32 = 2;
pub const TLS_INFO_TXCONF: i32 = 3;
pub const TLS_INFO_RXCONF: i32 = 4;
pub const TLS_INFO_ZC_RO_TX: i32 = 5;
pub const TLS_INFO_RX_NO_PAD: i32 = 6;
pub const TLS_INFO_TX_MAX_PAYLOAD_LEN: i32 = 7;
pub const __TLS_INFO_MAX: i32 = 8;
pub const TLS_INFO_MAX: i32 = __TLS_INFO_MAX - 1;

pub const TLS_CONF_BASE: i32 = 1;
pub const TLS_CONF_SW: i32 = 2;
pub const TLS_CONF_HW: i32 = 3;
pub const TLS_CONF_HW_RECORD: i32 = 4; /* unused */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
