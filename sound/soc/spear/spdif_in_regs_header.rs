// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPEAr SPDIF IN controller header file
 *
 * Copyright (ST) 2011 Vipin Kumar (vipin.kumar@st.com)
 */

pub const SPDIF_IN_CTRL: u32 = 0x00;
pub const SPDIF_IN_PRTYEN: u32 = 1 << 20;
pub const SPDIF_IN_STATEN: u32 = 1 << 19;
pub const SPDIF_IN_USREN: u32 = 1 << 18;
pub const SPDIF_IN_VALEN: u32 = 1 << 17;
pub const SPDIF_IN_BLKEN: u32 = 1 << 16;

pub const SPDIF_MODE_24BIT: u32 = 8 << 12;
pub const SPDIF_MODE_23BIT: u32 = 7 << 12;
pub const SPDIF_MODE_22BIT: u32 = 6 << 12;
pub const SPDIF_MODE_21BIT: u32 = 5 << 12;
pub const SPDIF_MODE_20BIT: u32 = 4 << 12;
pub const SPDIF_MODE_19BIT: u32 = 3 << 12;
pub const SPDIF_MODE_18BIT: u32 = 2 << 12;
pub const SPDIF_MODE_17BIT: u32 = 1 << 12;
pub const SPDIF_MODE_16BIT: u32 = 0 << 12;
pub const SPDIF_MODE_MASK: u32 = 0x0F << 12;

pub const SPDIF_IN_VALID: u32 = 1 << 11;
pub const SPDIF_IN_SAMPLE: u32 = 1 << 10;
pub const SPDIF_DATA_SWAP: u32 = 1 << 9;
pub const SPDIF_IN_ENB: u32 = 1 << 8;
pub const SPDIF_DATA_REVERT: u32 = 1 << 7;
pub const SPDIF_XTRACT_16BIT: u32 = 1 << 6;
pub const SPDIF_FIFO_THRES_16: u32 = 16 << 0;

pub const SPDIF_IN_IRQ_MASK: u32 = 0x04;
pub const SPDIF_IN_IRQ: u32 = 0x08;
pub const SPDIF_IRQ_FIFOWRITE: u32 = 1 << 0;
pub const SPDIF_IRQ_EMPTYFIFOREAD: u32 = 1 << 1;
pub const SPDIF_IRQ_FIFOFULL: u32 = 1 << 2;
pub const SPDIF_IRQ_OUTOFRANGE: u32 = 1 << 3;

pub const SPDIF_IN_STA: u32 = 0x0C;
pub const SPDIF_IN_LOCK: u32 = 0x1 << 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
