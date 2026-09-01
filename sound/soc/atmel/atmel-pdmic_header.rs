/* SPDX-License-Identifier: GPL-2.0 */

/* C header included <linux/bitops.h> for BIT() and GENMASK() register masks. */

pub const PDMIC_CR: u32 = 0x00000000;

pub const PDMIC_CR_SWRST: u32 = 0x1;
pub const PDMIC_CR_SWRST_MASK: u32 = 1u32 << 0;
pub const PDMIC_CR_SWRST_SHIFT: u32 = 0;

pub const PDMIC_CR_ENPDM_DIS: u32 = 0x0;
pub const PDMIC_CR_ENPDM_EN: u32 = 0x1;
pub const PDMIC_CR_ENPDM_MASK: u32 = 1u32 << 4;
pub const PDMIC_CR_ENPDM_SHIFT: u32 = 4;

pub const PDMIC_MR: u32 = 0x00000004;

pub const PDMIC_MR_CLKS_PCK: u32 = 0x0;
pub const PDMIC_MR_CLKS_GCK: u32 = 0x1;
pub const PDMIC_MR_CLKS_MASK: u32 = 1u32 << 4;
pub const PDMIC_MR_CLKS_SHIFT: u32 = 4;

pub const PDMIC_MR_PRESCAL_MASK: u32 = 0x00007f00;
pub const PDMIC_MR_PRESCAL_SHIFT: u32 = 8;

pub const PDMIC_CDR: u32 = 0x00000014;

pub const PDMIC_IER: u32 = 0x00000018;
pub const PDMIC_IER_OVRE: u32 = 1u32 << 25;

pub const PDMIC_IDR: u32 = 0x0000001c;
pub const PDMIC_IDR_OVRE: u32 = 1u32 << 25;

pub const PDMIC_IMR: u32 = 0x00000020;

pub const PDMIC_ISR: u32 = 0x00000024;
pub const PDMIC_ISR_OVRE: u32 = 1u32 << 25;

pub const PDMIC_DSPR0: u32 = 0x00000058;

pub const PDMIC_DSPR0_HPFBYP_DIS: u32 = 0x1;
pub const PDMIC_DSPR0_HPFBYP_EN: u32 = 0x0;
pub const PDMIC_DSPR0_HPFBYP_MASK: u32 = 1u32 << 1;
pub const PDMIC_DSPR0_HPFBYP_SHIFT: u32 = 1;

pub const PDMIC_DSPR0_SINBYP_DIS: u32 = 0x1;
pub const PDMIC_DSPR0_SINBYP_EN: u32 = 0x0;
pub const PDMIC_DSPR0_SINBYP_MASK: u32 = 1u32 << 2;
pub const PDMIC_DSPR0_SINBYP_SHIFT: u32 = 2;

pub const PDMIC_DSPR0_SIZE_16_BITS: u32 = 0x0;
pub const PDMIC_DSPR0_SIZE_32_BITS: u32 = 0x1;
pub const PDMIC_DSPR0_SIZE_MASK: u32 = 1u32 << 3;
pub const PDMIC_DSPR0_SIZE_SHIFT: u32 = 3;

pub const PDMIC_DSPR0_OSR_128: u32 = 0x0;
pub const PDMIC_DSPR0_OSR_64: u32 = 0x1;
pub const PDMIC_DSPR0_OSR_MASK: u32 = 0x00000070;
pub const PDMIC_DSPR0_OSR_SHIFT: u32 = 4;

pub const PDMIC_DSPR0_SCALE_MASK: u32 = 0x00000f00;
pub const PDMIC_DSPR0_SCALE_SHIFT: u32 = 8;

pub const PDMIC_DSPR0_SHIFT_MASK: u32 = 0x0000f000;
pub const PDMIC_DSPR0_SHIFT_SHIFT: u32 = 12;

pub const PDMIC_DSPR1: u32 = 0x0000005c;

pub const PDMIC_DSPR1_DGAIN_MASK: u32 = 0x00007fff;
pub const PDMIC_DSPR1_DGAIN_SHIFT: u32 = 0;

pub const PDMIC_DSPR1_OFFSET_MASK: u32 = 0xffff0000;
pub const PDMIC_DSPR1_OFFSET_SHIFT: u32 = 16;

pub const PDMIC_WPMR: u32 = 0x000000e4;

pub const PDMIC_WPSR: u32 = 0x000000e8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
