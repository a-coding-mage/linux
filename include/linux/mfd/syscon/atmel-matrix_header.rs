/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  Copyright (C) 2014 Atmel Corporation.
 *
 * Memory Controllers (MATRIX, EBI) - System peripherals registers.
 */

// C header guard: _LINUX_MFD_SYSCON_ATMEL_MATRIX_H

pub const AT91SAM9260_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9260_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9260_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9260_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9260_MATRIX_EBICSA: u32 = 0x11c;

pub const AT91SAM9261_MATRIX_MRCR: u32 = 0x0;
pub const AT91SAM9261_MATRIX_SCFG: u32 = 0x4;
pub const AT91SAM9261_MATRIX_TCR: u32 = 0x24;
pub const AT91SAM9261_MATRIX_EBICSA: u32 = 0x30;
pub const AT91SAM9261_MATRIX_USBPUCR: u32 = 0x34;

pub const AT91SAM9263_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9263_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9263_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9263_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9263_MATRIX_TCR: u32 = 0x114;
pub const AT91SAM9263_MATRIX_EBI0CSA: u32 = 0x120;
pub const AT91SAM9263_MATRIX_EBI1CSA: u32 = 0x124;

pub const AT91SAM9RL_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9RL_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9RL_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9RL_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9RL_MATRIX_TCR: u32 = 0x114;
pub const AT91SAM9RL_MATRIX_EBICSA: u32 = 0x120;

pub const AT91SAM9G45_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9G45_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9G45_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9G45_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9G45_MATRIX_TCR: u32 = 0x110;
pub const AT91SAM9G45_MATRIX_DDRMPR: u32 = 0x118;
pub const AT91SAM9G45_MATRIX_EBICSA: u32 = 0x128;

pub const AT91SAM9N12_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9N12_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9N12_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9N12_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9N12_MATRIX_EBICSA: u32 = 0x118;

pub const AT91SAM9X5_MATRIX_MCFG: u32 = 0x00;
pub const AT91SAM9X5_MATRIX_SCFG: u32 = 0x40;
pub const AT91SAM9X5_MATRIX_PRS: u32 = 0x80;
pub const AT91SAM9X5_MATRIX_MRCR: u32 = 0x100;
pub const AT91SAM9X5_MATRIX_EBICSA: u32 = 0x120;

pub const SAMA5D3_MATRIX_MCFG: u32 = 0x00;
pub const SAMA5D3_MATRIX_SCFG: u32 = 0x40;
pub const SAMA5D3_MATRIX_PRS: u32 = 0x80;
pub const SAMA5D3_MATRIX_MRCR: u32 = 0x100;

macro_rules! AT91_MATRIX_MCFG { ($o:expr, $x:expr) => { ($o) + (($x) * 0x4) }; }
pub const AT91_MATRIX_ULBT: u32 = GENMASK!(2, 0);
pub const AT91_MATRIX_ULBT_INFINITE: u32 = 0 << 0;
pub const AT91_MATRIX_ULBT_SINGLE: u32 = 1 << 0;
pub const AT91_MATRIX_ULBT_FOUR: u32 = 2 << 0;
pub const AT91_MATRIX_ULBT_EIGHT: u32 = 3 << 0;
pub const AT91_MATRIX_ULBT_SIXTEEN: u32 = 4 << 0;

macro_rules! AT91_MATRIX_SCFG { ($o:expr, $x:expr) => { ($o) + (($x) * 0x4) }; }
pub const AT91_MATRIX_SLOT_CYCLE: u32 = GENMASK!(7, 0);
pub const AT91_MATRIX_DEFMSTR_TYPE: u32 = GENMASK!(17, 16);
pub const AT91_MATRIX_DEFMSTR_TYPE_NONE: u32 = 0 << 16;
pub const AT91_MATRIX_DEFMSTR_TYPE_LAST: u32 = 1 << 16;
pub const AT91_MATRIX_DEFMSTR_TYPE_FIXED: u32 = 2 << 16;
pub const AT91_MATRIX_FIXED_DEFMSTR: u32 = GENMASK!(20, 18);
pub const AT91_MATRIX_ARBT: u32 = GENMASK!(25, 24);
pub const AT91_MATRIX_ARBT_ROUND_ROBIN: u32 = 0 << 24;
pub const AT91_MATRIX_ARBT_FIXED_PRIORITY: u32 = 1 << 24;

pub const AT91_MATRIX_ITCM_SIZE: u32 = GENMASK!(3, 0);
pub const AT91_MATRIX_ITCM_0: u32 = 0 << 0;
pub const AT91_MATRIX_ITCM_16: u32 = 5 << 0;
pub const AT91_MATRIX_ITCM_32: u32 = 6 << 0;
pub const AT91_MATRIX_ITCM_64: u32 = 7 << 0;
pub const AT91_MATRIX_DTCM_SIZE: u32 = GENMASK!(7, 4);
pub const AT91_MATRIX_DTCM_0: u32 = 0 << 4;
pub const AT91_MATRIX_DTCM_16: u32 = 5 << 4;
pub const AT91_MATRIX_DTCM_32: u32 = 6 << 4;
pub const AT91_MATRIX_DTCM_64: u32 = 7 << 4;

macro_rules! AT91_MATRIX_PRAS { ($o:expr, $x:expr) => { ($o) + (($x) * 0x8) }; }
macro_rules! AT91_MATRIX_PRBS { ($o:expr, $x:expr) => { ($o) + (($x) * 0x8) + 0x4 }; }
macro_rules! AT91_MATRIX_MPR { ($x:expr) => { GENMASK!(($x) * 0x4 + 1, ($x) * 0x4) }; }
macro_rules! AT91_MATRIX_RCB { ($x:expr) => { BIT!($x) }; }
macro_rules! AT91_MATRIX_CSA { ($cs:expr, $val:expr) => { ($val) << ($cs) }; }
pub const AT91_MATRIX_DBPUC: u32 = BIT!(8);
pub const AT91_MATRIX_DBPDC: u32 = BIT!(9);
pub const AT91_MATRIX_VDDIOMSEL: u32 = BIT!(16);
pub const AT91_MATRIX_VDDIOMSEL_1_8V: u32 = 0 << 16;
pub const AT91_MATRIX_VDDIOMSEL_3_3V: u32 = 1 << 16;
pub const AT91_MATRIX_EBI_IOSR: u32 = BIT!(17);
pub const AT91_MATRIX_DDR_IOSR: u32 = BIT!(18);
pub const AT91_MATRIX_NFD0_SELECT: u32 = BIT!(24);
pub const AT91_MATRIX_DDR_MP_EN: u32 = BIT!(25);
pub const AT91_MATRIX_USBPUCR_PUON: u32 = BIT!(30);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
