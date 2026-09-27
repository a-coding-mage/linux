// SPDX-License-Identifier: GPL-2.0-only
/*****************************************************************************
 *
 * Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
 * Jean-Christian Hassler <jhassler@free.fr>
 *
 * This file is part of the Audiowerk2 ALSA driver
 *
 *****************************************************************************/

/* SAA7146 registers */
pub const PCI_BT_A: u32 = 0x4C;
pub const IICTFR: u32 = 0x8C;
pub const IICSTA: u32 = 0x90;
pub const BaseA1_in: u32 = 0x94;
pub const ProtA1_in: u32 = 0x98;
pub const PageA1_in: u32 = 0x9C;
pub const BaseA1_out: u32 = 0xA0;
pub const ProtA1_out: u32 = 0xA4;
pub const PageA1_out: u32 = 0xA8;
pub const BaseA2_in: u32 = 0xAC;
pub const ProtA2_in: u32 = 0xB0;
pub const PageA2_in: u32 = 0xB4;
pub const BaseA2_out: u32 = 0xB8;
pub const ProtA2_out: u32 = 0xBC;
pub const PageA2_out: u32 = 0xC0;
pub const IER: u32 = 0xDC;
pub const GPIO_CTRL: u32 = 0xE0;
pub const ACON1: u32 = 0xF4;
pub const ACON2: u32 = 0xF8;
pub const MC1: u32 = 0xFC;
pub const MC2: u32 = 0x100;
pub const ISR: u32 = 0x10C;
pub const PSR: u32 = 0x110;
pub const SSR: u32 = 0x114;
pub const PCI_ADP1: u32 = 0x12C;
pub const PCI_ADP2: u32 = 0x130;
pub const PCI_ADP3: u32 = 0x134;
pub const PCI_ADP4: u32 = 0x138;
pub const LEVEL_REP: u32 = 0x140;
pub const FB_BUFFER1: u32 = 0x144;
pub const FB_BUFFER2: u32 = 0x148;
pub const TSL1: u32 = 0x180;
pub const TSL2: u32 = 0x1C0;

pub const ME: kernel::ffi::c_ulong = 1 << 11;
pub const LIMIT: kernel::ffi::c_ulong = 1 << 4;
pub const PV: kernel::ffi::c_ulong = 1 << 3;

/* PSR/ISR/IER */
pub const PPEF: kernel::ffi::c_ulong = 1 << 31;
pub const PABO: kernel::ffi::c_ulong = 1 << 30;
pub const IIC_S: kernel::ffi::c_ulong = 1 << 17;
pub const IIC_E: kernel::ffi::c_ulong = 1 << 16;
pub const A2_in: kernel::ffi::c_ulong = 1 << 15;
pub const A2_out: kernel::ffi::c_ulong = 1 << 14;
pub const A1_in: kernel::ffi::c_ulong = 1 << 13;
pub const A1_out: kernel::ffi::c_ulong = 1 << 12;
pub const AFOU: kernel::ffi::c_ulong = 1 << 11;
pub const PIN3: kernel::ffi::c_ulong = 1 << 6;
pub const PIN2: kernel::ffi::c_ulong = 1 << 5;
pub const PIN1: kernel::ffi::c_ulong = 1 << 4;
pub const PIN0: kernel::ffi::c_ulong = 1 << 3;
pub const ECS: kernel::ffi::c_ulong = 1 << 2;
pub const EC3S: kernel::ffi::c_ulong = 1 << 1;
pub const EC0S: kernel::ffi::c_ulong = 1 << 0;

/* SSR */
pub const PRQ: kernel::ffi::c_ulong = 1 << 31;
pub const PMA: kernel::ffi::c_ulong = 1 << 30;
pub const IIC_EA: kernel::ffi::c_ulong = 1 << 21;
pub const IIC_EW: kernel::ffi::c_ulong = 1 << 20;
pub const IIC_ER: kernel::ffi::c_ulong = 1 << 19;
pub const IIC_EL: kernel::ffi::c_ulong = 1 << 18;
pub const IIC_EF: kernel::ffi::c_ulong = 1 << 17;
pub const AF2_in: kernel::ffi::c_ulong = 1 << 10;
pub const AF2_out: kernel::ffi::c_ulong = 1 << 9;
pub const AF1_in: kernel::ffi::c_ulong = 1 << 8;
pub const AF1_out: kernel::ffi::c_ulong = 1 << 7;
pub const EC5S: kernel::ffi::c_ulong = 1 << 3;
pub const EC4S: kernel::ffi::c_ulong = 1 << 2;
pub const EC2S: kernel::ffi::c_ulong = 1 << 1;
pub const EC1S: kernel::ffi::c_ulong = 1 << 0;

/* PCI_BT_A */
pub const BurstA1_in: kernel::ffi::c_ulong = 1 << 26;
pub const ThreshA1_in: kernel::ffi::c_ulong = 1 << 24;
pub const BurstA1_out: kernel::ffi::c_ulong = 1 << 18;
pub const ThreshA1_out: kernel::ffi::c_ulong = 1 << 16;
pub const BurstA2_in: kernel::ffi::c_ulong = 1 << 10;
pub const ThreshA2_in: kernel::ffi::c_ulong = 1 << 8;
pub const BurstA2_out: kernel::ffi::c_ulong = 1 << 2;
pub const ThreshA2_out: kernel::ffi::c_ulong = 1 << 0;

/* MC1 */
pub const MRST_N: kernel::ffi::c_ulong = 1 << 15;
pub const EAP: kernel::ffi::c_ulong = 1 << 9;
pub const EI2C: kernel::ffi::c_ulong = 1 << 8;
pub const TR_E_A2_OUT: kernel::ffi::c_ulong = 1 << 3;
pub const TR_E_A2_IN: kernel::ffi::c_ulong = 1 << 2;
pub const TR_E_A1_OUT: kernel::ffi::c_ulong = 1 << 1;
pub const TR_E_A1_IN: kernel::ffi::c_ulong = 1 << 0;

/* MC2 */
pub const UPLD_IIC: kernel::ffi::c_ulong = 1 << 0;

/* ACON1 */
pub const AUDIO_MODE: kernel::ffi::c_ulong = 1 << 29;
pub const MAXLEVEL: kernel::ffi::c_ulong = 1 << 22;
pub const A1_SWAP: kernel::ffi::c_ulong = 1 << 21;
pub const A2_SWAP: kernel::ffi::c_ulong = 1 << 20;
pub const WS0_CTRL: kernel::ffi::c_ulong = 1 << 18;
pub const WS0_SYNC: kernel::ffi::c_ulong = 1 << 16;
pub const WS1_CTRL: kernel::ffi::c_ulong = 1 << 14;
pub const WS1_SYNC: kernel::ffi::c_ulong = 1 << 12;
pub const WS2_CTRL: kernel::ffi::c_ulong = 1 << 10;
pub const WS2_SYNC: kernel::ffi::c_ulong = 1 << 8;
pub const WS3_CTRL: kernel::ffi::c_ulong = 1 << 6;
pub const WS3_SYNC: kernel::ffi::c_ulong = 1 << 4;
pub const WS4_CTRL: kernel::ffi::c_ulong = 1 << 2;
pub const WS4_SYNC: kernel::ffi::c_ulong = 1 << 0;

/* ACON2 */
pub const A1_CLKSRC: kernel::ffi::c_ulong = 1 << 27;
pub const A2_CLKSRC: kernel::ffi::c_ulong = 1 << 22;
pub const INVERT_BCLK1: kernel::ffi::c_ulong = 1 << 21;
pub const INVERT_BCLK2: kernel::ffi::c_ulong = 1 << 20;
pub const BCLK1_OEN: kernel::ffi::c_ulong = 1 << 19;
pub const BCLK2_OEN: kernel::ffi::c_ulong = 1 << 18;

/* IICSTA */
pub const IICCC: kernel::ffi::c_ulong = 1 << 8;
pub const ABORT: kernel::ffi::c_ulong = 1 << 7;
pub const SPERR: kernel::ffi::c_ulong = 1 << 6;
pub const APERR: kernel::ffi::c_ulong = 1 << 5;
pub const DTERR: kernel::ffi::c_ulong = 1 << 4;
pub const DRERR: kernel::ffi::c_ulong = 1 << 3;
pub const AL: kernel::ffi::c_ulong = 1 << 2;
pub const ERR: kernel::ffi::c_ulong = 1 << 1;
pub const BUSY: kernel::ffi::c_ulong = 1 << 0;

/* IICTFR */
pub const BYTE2: kernel::ffi::c_ulong = 1 << 24;
pub const BYTE1: kernel::ffi::c_ulong = 1 << 16;
pub const BYTE0: kernel::ffi::c_ulong = 1 << 8;
pub const ATRR2: kernel::ffi::c_ulong = 1 << 6;
pub const ATRR1: kernel::ffi::c_ulong = 1 << 4;
pub const ATRR0: kernel::ffi::c_ulong = 1 << 2;
/* C redefines ERR and BUSY here with the same values as in IICSTA. */

pub const START: u32 = 3;
pub const CONT: u32 = 2;
pub const STOP: u32 = 1;
pub const NOP: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
