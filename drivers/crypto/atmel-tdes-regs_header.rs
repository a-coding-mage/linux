/* SPDX-License-Identifier: GPL-2.0 */

pub const TDES_CR: u32 = 0x00;
pub const TDES_CR_START: u32 = 1 << 0;
pub const TDES_CR_SWRST: u32 = 1 << 8;
pub const TDES_CR_LOADSEED: u32 = 1 << 16;

pub const TDES_MR: u32 = 0x04;
pub const TDES_MR_CYPHER_DEC: u32 = 0 << 0;
pub const TDES_MR_CYPHER_ENC: u32 = 1 << 0;
pub const TDES_MR_TDESMOD_MASK: u32 = 0x3 << 1;
pub const TDES_MR_TDESMOD_DES: u32 = 0x0 << 1;
pub const TDES_MR_TDESMOD_TDES: u32 = 0x1 << 1;
pub const TDES_MR_TDESMOD_XTEA: u32 = 0x2 << 1;
pub const TDES_MR_KEYMOD_3KEY: u32 = 0 << 4;
pub const TDES_MR_KEYMOD_2KEY: u32 = 1 << 4;
pub const TDES_MR_SMOD_MASK: u32 = 0x3 << 8;
pub const TDES_MR_SMOD_MANUAL: u32 = 0x0 << 8;
pub const TDES_MR_SMOD_AUTO: u32 = 0x1 << 8;
pub const TDES_MR_SMOD_PDC: u32 = 0x2 << 8;
pub const TDES_MR_OPMOD_MASK: u32 = 0x3 << 12;
pub const TDES_MR_OPMOD_ECB: u32 = 0x0 << 12;
pub const TDES_MR_OPMOD_CBC: u32 = 0x1 << 12;
pub const TDES_MR_OPMOD_OFB: u32 = 0x2 << 12;
pub const TDES_MR_OPMOD_CFB: u32 = 0x3 << 12;
pub const TDES_MR_LOD: u32 = 0x1 << 15;
pub const TDES_MR_CFBS_MASK: u32 = 0x3 << 16;
pub const TDES_MR_CFBS_64b: u32 = 0x0 << 16;
pub const TDES_MR_CFBS_32b: u32 = 0x1 << 16;
pub const TDES_MR_CFBS_16b: u32 = 0x2 << 16;
pub const TDES_MR_CFBS_8b: u32 = 0x3 << 16;
pub const TDES_MR_CKEY_MASK: u32 = 0xF << 20;
pub const TDES_MR_CKEY_OFFSET: u32 = 20;
pub const TDES_MR_CTYPE_MASK: u32 = 0x3F << 24;
pub const TDES_MR_CTYPE_OFFSET: u32 = 24;

pub const TDES_IER: u32 = 0x10;
pub const TDES_IDR: u32 = 0x14;
pub const TDES_IMR: u32 = 0x18;
pub const TDES_ISR: u32 = 0x1C;
pub const TDES_INT_DATARDY: u32 = 1 << 0;
pub const TDES_INT_ENDRX: u32 = 1 << 1;
pub const TDES_INT_ENDTX: u32 = 1 << 2;
pub const TDES_INT_RXBUFF: u32 = 1 << 3;
pub const TDES_INT_TXBUFE: u32 = 1 << 4;
pub const TDES_INT_URAD: u32 = 1 << 8;
pub const TDES_ISR_URAT_MASK: u32 = 0x3 << 12;
pub const TDES_ISR_URAT_IDR: u32 = 0x0 << 12;
pub const TDES_ISR_URAT_ODR: u32 = 0x1 << 12;
pub const TDES_ISR_URAT_MR: u32 = 0x2 << 12;
pub const TDES_ISR_URAT_WO: u32 = 0x3 << 12;

pub const TDES_KEY1W1R: u32 = 0x20;
pub const TDES_KEY1W2R: u32 = 0x24;
pub const TDES_KEY2W1R: u32 = 0x28;
pub const TDES_KEY2W2R: u32 = 0x2C;
pub const TDES_KEY3W1R: u32 = 0x30;
pub const TDES_KEY3W2R: u32 = 0x34;
pub const TDES_IDATA1R: u32 = 0x40;
pub const TDES_IDATA2R: u32 = 0x44;
pub const TDES_ODATA1R: u32 = 0x50;
pub const TDES_ODATA2R: u32 = 0x54;
pub const TDES_IV1R: u32 = 0x60;
pub const TDES_IV2R: u32 = 0x64;

pub const TDES_XTEARNDR: u32 = 0x70;
pub const TDES_XTEARNDR_XTEA_RNDS_MASK: u32 = 0x3F << 0;
pub const TDES_XTEARNDR_XTEA_RNDS_OFFSET: u32 = 0;

pub const TDES_HW_VERSION: u32 = 0xFC;

pub const TDES_RPR: u32 = 0x100;
pub const TDES_RCR: u32 = 0x104;
pub const TDES_TPR: u32 = 0x108;
pub const TDES_TCR: u32 = 0x10C;
pub const TDES_RNPR: u32 = 0x118;
pub const TDES_RNCR: u32 = 0x11C;
pub const TDES_TNPR: u32 = 0x118;
pub const TDES_TNCR: u32 = 0x11C;
pub const TDES_PTCR: u32 = 0x120;
pub const TDES_PTCR_RXTEN: u32 = 1 << 0;
pub const TDES_PTCR_RXTDIS: u32 = 1 << 1;
pub const TDES_PTCR_TXTEN: u32 = 1 << 8;
pub const TDES_PTCR_TXTDIS: u32 = 1 << 9;
pub const TDES_PTSR: u32 = 0x124;
pub const TDES_PTSR_RXTEN: u32 = 1 << 0;
pub const TDES_PTSR_TXTEN: u32 = 1 << 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
