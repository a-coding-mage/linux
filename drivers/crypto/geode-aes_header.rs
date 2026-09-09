/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2003-2006, Advanced Micro Devices, Inc.
 */

/* driver logic flags */
pub const AES_MODE_ECB: u32 = 0;
pub const AES_MODE_CBC: u32 = 1;

pub const AES_DIR_DECRYPT: u32 = 0;
pub const AES_DIR_ENCRYPT: u32 = 1;

pub const AES_FLAGS_HIDDENKEY: u32 = 1 << 0;

/* Register definitions */

pub const AES_CTRLA_REG: u32 = 0x0000;

pub const AES_CTRL_START: u32 = 0x01;
pub const AES_CTRL_DECRYPT: u32 = 0x00;
pub const AES_CTRL_ENCRYPT: u32 = 0x02;
pub const AES_CTRL_WRKEY: u32 = 0x04;
pub const AES_CTRL_DCA: u32 = 0x08;
pub const AES_CTRL_SCA: u32 = 0x10;
pub const AES_CTRL_CBC: u32 = 0x20;

pub const AES_INTR_REG: u32 = 0x0008;

pub const AES_INTRA_PENDING: u32 = 1 << 16;
pub const AES_INTRB_PENDING: u32 = 1 << 17;

pub const AES_INTR_PENDING: u32 = AES_INTRA_PENDING | AES_INTRB_PENDING;
pub const AES_INTR_MASK: u32 = 0x07;

pub const AES_SOURCEA_REG: u32 = 0x0010;
pub const AES_DSTA_REG: u32 = 0x0014;
pub const AES_LENA_REG: u32 = 0x0018;
pub const AES_WRITEKEY0_REG: u32 = 0x0030;
pub const AES_WRITEIV0_REG: u32 = 0x0040;

/*  A very large counter that is used to gracefully bail out of an
 *  operation in case of trouble
 */

pub const AES_OP_TIMEOUT: u32 = 0x50000;

#[repr(C)]
pub struct geode_aes_tfm_ctx {
    pub key: [u8; AES_KEYSIZE_128],
    pub fallback: geode_aes_tfm_ctx_fallback,
    pub keylen: u32,
}

#[repr(C)]
pub union geode_aes_tfm_ctx_fallback {
    pub skcipher: *mut crypto_skcipher,
    pub cip: *mut crypto_cipher,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
