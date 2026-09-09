/* SPDX-License-Identifier: GPL-2.0 */

pub const AES_CR: u32 = 0x00;
pub const AES_CR_START: u32 = 1 << 0;
pub const AES_CR_SWRST: u32 = 1 << 8;
pub const AES_CR_LOADSEED: u32 = 1 << 16;

pub const AES_MR: u32 = 0x04;
pub const AES_MR_CYPHER_DEC: u32 = 0 << 0;
pub const AES_MR_CYPHER_ENC: u32 = 1 << 0;
pub const AES_MR_GTAGEN: u32 = 1 << 1;
pub const AES_MR_DUALBUFF: u32 = 1 << 3;
pub const AES_MR_PROCDLY_MASK: u32 = 0xF << 4;
pub const AES_MR_PROCDLY_OFFSET: u32 = 4;
pub const AES_MR_SMOD_MASK: u32 = 0x3 << 8;
pub const AES_MR_SMOD_MANUAL: u32 = 0x0 << 8;
pub const AES_MR_SMOD_AUTO: u32 = 0x1 << 8;
pub const AES_MR_SMOD_IDATAR0: u32 = 0x2 << 8;
pub const AES_MR_KEYSIZE_MASK: u32 = 0x3 << 10;
pub const AES_MR_KEYSIZE_128: u32 = 0x0 << 10;
pub const AES_MR_KEYSIZE_192: u32 = 0x1 << 10;
pub const AES_MR_KEYSIZE_256: u32 = 0x2 << 10;
pub const AES_MR_OPMOD_MASK: u32 = 0x7 << 12;
pub const AES_MR_OPMOD_ECB: u32 = 0x0 << 12;
pub const AES_MR_OPMOD_CBC: u32 = 0x1 << 12;
pub const AES_MR_OPMOD_OFB: u32 = 0x2 << 12;
pub const AES_MR_OPMOD_CFB: u32 = 0x3 << 12;
pub const AES_MR_OPMOD_CTR: u32 = 0x4 << 12;
pub const AES_MR_OPMOD_GCM: u32 = 0x5 << 12;
pub const AES_MR_OPMOD_XTS: u32 = 0x6 << 12;
pub const AES_MR_LOD: u32 = 0x1 << 15;
pub const AES_MR_CFBS_MASK: u32 = 0x7 << 16;
pub const AES_MR_CFBS_128b: u32 = 0x0 << 16;
pub const AES_MR_CFBS_64b: u32 = 0x1 << 16;
pub const AES_MR_CFBS_32b: u32 = 0x2 << 16;
pub const AES_MR_CFBS_16b: u32 = 0x3 << 16;
pub const AES_MR_CFBS_8b: u32 = 0x4 << 16;
pub const AES_MR_CKEY_MASK: u32 = 0xF << 20;
pub const AES_MR_CKEY_OFFSET: u32 = 20;
pub const AES_MR_CMTYP_MASK: u32 = 0x1F << 24;
pub const AES_MR_CMTYP_OFFSET: u32 = 24;

pub const AES_IER: u32 = 0x10;
pub const AES_IDR: u32 = 0x14;
pub const AES_IMR: u32 = 0x18;
pub const AES_ISR: u32 = 0x1C;
pub const AES_INT_DATARDY: u32 = 1 << 0;
pub const AES_INT_URAD: u32 = 1 << 8;
pub const AES_INT_TAGRDY: u32 = 1 << 16;
pub const AES_ISR_URAT_MASK: u32 = 0xF << 12;
pub const AES_ISR_URAT_IDR_WR_PROC: u32 = 0x0 << 12;
pub const AES_ISR_URAT_ODR_RD_PROC: u32 = 0x1 << 12;
pub const AES_ISR_URAT_MR_WR_PROC: u32 = 0x2 << 12;
pub const AES_ISR_URAT_ODR_RD_SUBK: u32 = 0x3 << 12;
pub const AES_ISR_URAT_MR_WR_SUBK: u32 = 0x4 << 12;
pub const AES_ISR_URAT_WOR_RD: u32 = 0x5 << 12;

pub const fn AES_KEYWR(x: u32) -> u32 { 0x20 + (x * 0x04) }
pub const fn AES_IDATAR(x: u32) -> u32 { 0x40 + (x * 0x04) }
pub const fn AES_ODATAR(x: u32) -> u32 { 0x50 + (x * 0x04) }
pub const fn AES_IVR(x: u32) -> u32 { 0x60 + (x * 0x04) }

pub const AES_AADLENR: u32 = 0x70;
pub const AES_CLENR: u32 = 0x74;
pub const fn AES_GHASHR(x: u32) -> u32 { 0x78 + (x * 0x04) }
pub const fn AES_TAGR(x: u32) -> u32 { 0x88 + (x * 0x04) }
pub const AES_CTRR: u32 = 0x98;
pub const fn AES_GCMHR(x: u32) -> u32 { 0x9c + (x * 0x04) }

pub const AES_EMR: u32 = 0xb0;
pub const AES_EMR_APEN: u32 = 1 << 0; // Auto Padding Enable
pub const AES_EMR_APM: u32 = 1 << 1; // Auto Padding Mode
pub const AES_EMR_APM_IPSEC: u32 = 0x0;
pub const AES_EMR_APM_SSL: u32 = 1 << 1;
pub const AES_EMR_PLIPEN: u32 = 1 << 4; // PLIP Enable
pub const AES_EMR_PLIPD: u32 = 1 << 5; // PLIP Decipher
pub const AES_EMR_PADLEN_MASK: u32 = 0xF << 8;
pub const AES_EMR_PADLEN_OFFSET: u32 = 8;
pub const fn AES_EMR_PADLEN(padlen: u32) -> u32 {
    (padlen << AES_EMR_PADLEN_OFFSET) & AES_EMR_PADLEN_MASK
}
pub const AES_EMR_NHEAD_MASK: u32 = 0xF << 16;
pub const AES_EMR_NHEAD_OFFSET: u32 = 16;
pub const fn AES_EMR_NHEAD(nhead: u32) -> u32 {
    (nhead << AES_EMR_NHEAD_OFFSET) & AES_EMR_NHEAD_MASK
}

pub const fn AES_TWR(x: u32) -> u32 { 0xc0 + (x * 0x04) }
pub const fn AES_ALPHAR(x: u32) -> u32 { 0xd0 + (x * 0x04) }

pub const AES_HW_VERSION: u32 = 0xFC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
