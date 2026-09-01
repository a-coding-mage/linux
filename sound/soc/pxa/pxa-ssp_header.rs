// SPDX-License-Identifier: GPL-2.0-only
/*
 * ASoC PXA SSP port support
 */

/* SSP clock sources */
pub const PXA_SSP_CLK_PLL: u32 = 0;
pub const PXA_SSP_CLK_EXT: u32 = 1;
pub const PXA_SSP_CLK_NET: u32 = 2;
pub const PXA_SSP_CLK_AUDIO: u32 = 3;
pub const PXA_SSP_CLK_NET_PLL: u32 = 4;

/* SSP audio dividers */
pub const PXA_SSP_AUDIO_DIV_ACDS: u32 = 0;
pub const PXA_SSP_AUDIO_DIV_SCDB: u32 = 1;
pub const PXA_SSP_DIV_SCR: u32 = 2;

/* SSP ACDS audio dividers values */
pub const PXA_SSP_CLK_AUDIO_DIV_1: u32 = 0;
pub const PXA_SSP_CLK_AUDIO_DIV_2: u32 = 1;
pub const PXA_SSP_CLK_AUDIO_DIV_4: u32 = 2;
pub const PXA_SSP_CLK_AUDIO_DIV_8: u32 = 3;
pub const PXA_SSP_CLK_AUDIO_DIV_16: u32 = 4;
pub const PXA_SSP_CLK_AUDIO_DIV_32: u32 = 5;

/* SSP divider bypass */
pub const PXA_SSP_CLK_SCDB_4: u32 = 0;
pub const PXA_SSP_CLK_SCDB_1: u32 = 1;
pub const PXA_SSP_CLK_SCDB_8: u32 = 2;

pub const PXA_SSP_PLL_OUT: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
