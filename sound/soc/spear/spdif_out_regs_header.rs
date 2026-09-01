// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPEAr SPDIF OUT controller header file
 *
 * Copyright (ST) 2011 Vipin Kumar (vipin.kumar@st.com)
 */

pub const SPDIF_OUT_SOFT_RST: u32 = 0x00;
pub const SPDIF_OUT_RESET: u32 = 1 << 0;
pub const SPDIF_OUT_FIFO_DATA: u32 = 0x04;
pub const SPDIF_OUT_INT_STA: u32 = 0x08;
pub const SPDIF_OUT_INT_STA_CLR: u32 = 0x0C;
pub const SPDIF_INT_UNDERFLOW: u32 = 1 << 0;
pub const SPDIF_INT_EODATA: u32 = 1 << 1;
pub const SPDIF_INT_EOBLOCK: u32 = 1 << 2;
pub const SPDIF_INT_EOLATENCY: u32 = 1 << 3;
pub const SPDIF_INT_EOPD_DATA: u32 = 1 << 4;
pub const SPDIF_INT_MEMFULLREAD: u32 = 1 << 5;
pub const SPDIF_INT_EOPD_PAUSE: u32 = 1 << 6;

pub const SPDIF_OUT_INT_EN: u32 = 0x10;
pub const SPDIF_OUT_INT_EN_SET: u32 = 0x14;
pub const SPDIF_OUT_INT_EN_CLR: u32 = 0x18;
pub const SPDIF_OUT_CTRL: u32 = 0x1C;
pub const SPDIF_OPMODE_MASK: u32 = 7 << 0;
pub const SPDIF_OPMODE_OFF: u32 = 0 << 0;
pub const SPDIF_OPMODE_MUTE_PCM: u32 = 1 << 0;
pub const SPDIF_OPMODE_MUTE_PAUSE: u32 = 2 << 0;
pub const SPDIF_OPMODE_AUD_DATA: u32 = 3 << 0;
pub const SPDIF_OPMODE_ENCODE: u32 = 4 << 0;
pub const SPDIF_STATE_NORMAL: u32 = 1 << 3;
pub const SPDIF_DIVIDER_MASK: u32 = 0xff << 5;
pub const SPDIF_DIVIDER_SHIFT: u32 = 5;
pub const SPDIF_SAMPLEREAD_MASK: u32 = 0x1ffff << 15;
pub const SPDIF_SAMPLEREAD_SHIFT: u32 = 15;
pub const SPDIF_OUT_STA: u32 = 0x20;
pub const SPDIF_OUT_PA_PB: u32 = 0x24;
pub const SPDIF_OUT_PC_PD: u32 = 0x28;
pub const SPDIF_OUT_CL1: u32 = 0x2C;
pub const SPDIF_OUT_CR1: u32 = 0x30;
pub const SPDIF_OUT_CL2_CR2_UV: u32 = 0x34;
pub const SPDIF_OUT_PAUSE_LAT: u32 = 0x38;
pub const SPDIF_OUT_FRMLEN_BRST: u32 = 0x3C;
pub const SPDIF_OUT_CFG: u32 = 0x40;
pub const SPDIF_OUT_MEMFMT_16_0: u32 = 0 << 5;
pub const SPDIF_OUT_MEMFMT_16_16: u32 = 1 << 5;
pub const SPDIF_OUT_VALID_DMA: u32 = 0 << 3;
pub const SPDIF_OUT_VALID_HW: u32 = 1 << 3;
pub const SPDIF_OUT_USER_DMA: u32 = 0 << 2;
pub const SPDIF_OUT_USER_HW: u32 = 1 << 2;
pub const SPDIF_OUT_CHNLSTA_DMA: u32 = 0 << 1;
pub const SPDIF_OUT_CHNLSTA_HW: u32 = 1 << 1;
pub const SPDIF_OUT_PARITY_HW: u32 = 0 << 0;
pub const SPDIF_OUT_PARITY_DMA: u32 = 1 << 0;
pub const SPDIF_OUT_FDMA_TRIG_2: u32 = 2 << 8;
pub const SPDIF_OUT_FDMA_TRIG_6: u32 = 6 << 8;
pub const SPDIF_OUT_FDMA_TRIG_8: u32 = 8 << 8;
pub const SPDIF_OUT_FDMA_TRIG_10: u32 = 10 << 8;
pub const SPDIF_OUT_FDMA_TRIG_12: u32 = 12 << 8;
pub const SPDIF_OUT_FDMA_TRIG_16: u32 = 16 << 8;
pub const SPDIF_OUT_FDMA_TRIG_18: u32 = 18 << 8;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
