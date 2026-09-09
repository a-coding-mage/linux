/* SPDX-License-Identifier: GPL-2.0 */

pub const fn SHA_REG_DIGEST(x: u32) -> u32 {
    0x80u32.wrapping_add(x.wrapping_mul(0x04))
}

pub const fn SHA_REG_DIN(x: u32) -> u32 {
    0x40u32.wrapping_add(x.wrapping_mul(0x04))
}

pub const SHA_CR: u32 = 0x00;
pub const SHA_CR_START: u32 = 1 << 0;
pub const SHA_CR_FIRST: u32 = 1 << 4;
pub const SHA_CR_SWRST: u32 = 1 << 8;
pub const SHA_CR_WUIHV: u32 = 1 << 12;
pub const SHA_CR_WUIEHV: u32 = 1 << 13;

pub const SHA_MR: u32 = 0x04;
pub const SHA_MR_MODE_MASK: u32 = 0x3 << 0;
pub const SHA_MR_MODE_MANUAL: u32 = 0x0;
pub const SHA_MR_MODE_AUTO: u32 = 0x1;
pub const SHA_MR_MODE_PDC: u32 = 0x2;
pub const SHA_MR_MODE_IDATAR0: u32 = 0x2;
pub const SHA_MR_PROCDLY: u32 = 1 << 4;
pub const SHA_MR_UIHV: u32 = 1 << 5;
pub const SHA_MR_UIEHV: u32 = 1 << 6;
pub const SHA_MR_ALGO_MASK: u32 = 0x7 << 8;
pub const SHA_MR_ALGO_SHA1: u32 = 0 << 8;
pub const SHA_MR_ALGO_SHA256: u32 = 1 << 8;
pub const SHA_MR_ALGO_SHA384: u32 = 2 << 8;
pub const SHA_MR_ALGO_SHA512: u32 = 3 << 8;
pub const SHA_MR_ALGO_SHA224: u32 = 4 << 8;
pub const SHA_MR_HMAC: u32 = 1 << 11;
pub const SHA_MR_DUALBUFF: u32 = 1 << 16;

pub const SHA_FLAGS_ALGO_MASK: u32 = SHA_MR_ALGO_MASK;
pub const SHA_FLAGS_SHA1: u32 = SHA_MR_ALGO_SHA1;
pub const SHA_FLAGS_SHA256: u32 = SHA_MR_ALGO_SHA256;
pub const SHA_FLAGS_SHA384: u32 = SHA_MR_ALGO_SHA384;
pub const SHA_FLAGS_SHA512: u32 = SHA_MR_ALGO_SHA512;
pub const SHA_FLAGS_SHA224: u32 = SHA_MR_ALGO_SHA224;
pub const SHA_FLAGS_HMAC: u32 = SHA_MR_HMAC;
pub const SHA_FLAGS_HMAC_SHA1: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_SHA1;
pub const SHA_FLAGS_HMAC_SHA256: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_SHA256;
pub const SHA_FLAGS_HMAC_SHA384: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_SHA384;
pub const SHA_FLAGS_HMAC_SHA512: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_SHA512;
pub const SHA_FLAGS_HMAC_SHA224: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_SHA224;
pub const SHA_FLAGS_MODE_MASK: u32 = SHA_FLAGS_HMAC | SHA_FLAGS_ALGO_MASK;

pub const SHA_IER: u32 = 0x10;
pub const SHA_IDR: u32 = 0x14;
pub const SHA_IMR: u32 = 0x18;
pub const SHA_ISR: u32 = 0x1C;
pub const SHA_INT_DATARDY: u32 = 1 << 0;
pub const SHA_INT_ENDTX: u32 = 1 << 1;
pub const SHA_INT_TXBUFE: u32 = 1 << 2;
pub const SHA_INT_URAD: u32 = 1 << 8;
pub const SHA_ISR_URAT_MASK: u32 = 0x7 << 12;
pub const SHA_ISR_URAT_IDR: u32 = 0x0 << 12;
pub const SHA_ISR_URAT_ODR: u32 = 0x1 << 12;
pub const SHA_ISR_URAT_MR: u32 = 0x2 << 12;
pub const SHA_ISR_URAT_WO: u32 = 0x5 << 12;

pub const SHA_MSR: u32 = 0x20;
pub const SHA_BCR: u32 = 0x30;

pub const SHA_HW_VERSION: u32 = 0xFC;

pub const SHA_TPR: u32 = 0x108;
pub const SHA_TCR: u32 = 0x10C;
pub const SHA_TNPR: u32 = 0x118;
pub const SHA_TNCR: u32 = 0x11C;
pub const SHA_PTCR: u32 = 0x120;
pub const SHA_PTCR_TXTEN: u32 = 1 << 8;
pub const SHA_PTCR_TXTDIS: u32 = 1 << 9;
pub const SHA_PTSR: u32 = 0x124;
pub const SHA_PTSR_TXTEN: u32 = 1 << 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
