// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for the AT73C213 16-bit stereo DAC on Atmel ATSTK1000
 *
 * Copyright (C) 2006 - 2007 Atmel Corporation
 */

// DAC control register
pub const DAC_CTRL: u32 = 0x00;
pub const DAC_CTRL_ONPADRV: u32 = 7;
pub const DAC_CTRL_ONAUXIN: u32 = 6;
pub const DAC_CTRL_ONDACR: u32 = 5;
pub const DAC_CTRL_ONDACL: u32 = 4;
pub const DAC_CTRL_ONLNOR: u32 = 3;
pub const DAC_CTRL_ONLNOL: u32 = 2;
pub const DAC_CTRL_ONLNIR: u32 = 1;
pub const DAC_CTRL_ONLNIL: u32 = 0;

// DAC left line in gain register
pub const DAC_LLIG: u32 = 0x01;
pub const DAC_LLIG_LLIG: u32 = 0;

// DAC right line in gain register
pub const DAC_RLIG: u32 = 0x02;
pub const DAC_RLIG_RLIG: u32 = 0;

// DAC Left Master Playback Gain Register
pub const DAC_LMPG: u32 = 0x03;
pub const DAC_LMPG_LMPG: u32 = 0;

// DAC Right Master Playback Gain Register
pub const DAC_RMPG: u32 = 0x04;
pub const DAC_RMPG_RMPG: u32 = 0;

// DAC Left Line Out Gain Register
pub const DAC_LLOG: u32 = 0x05;
pub const DAC_LLOG_LLOG: u32 = 0;

// DAC Right Line Out Gain Register
pub const DAC_RLOG: u32 = 0x06;
pub const DAC_RLOG_RLOG: u32 = 0;

// DAC Output Level Control Register
pub const DAC_OLC: u32 = 0x07;
pub const DAC_OLC_RSHORT: u32 = 7;
pub const DAC_OLC_ROLC: u32 = 4;
pub const DAC_OLC_LSHORT: u32 = 3;
pub const DAC_OLC_LOLC: u32 = 0;

// DAC Mixer Control Register
pub const DAC_MC: u32 = 0x08;
pub const DAC_MC_INVR: u32 = 5;
pub const DAC_MC_INVL: u32 = 4;
pub const DAC_MC_RMSMIN2: u32 = 3;
pub const DAC_MC_RMSMIN1: u32 = 2;
pub const DAC_MC_LMSMIN2: u32 = 1;
pub const DAC_MC_LMSMIN1: u32 = 0;

// DAC Clock and Sampling Frequency Control Register
pub const DAC_CSFC: u32 = 0x09;
pub const DAC_CSFC_OVRSEL: u32 = 4;

// DAC Miscellaneous Register
pub const DAC_MISC: u32 = 0x0A;
pub const DAC_MISC_VCMCAPSEL: u32 = 7;
pub const DAC_MISC_DINTSEL: u32 = 4;
pub const DAC_MISC_DITHEN: u32 = 3;
pub const DAC_MISC_DEEMPEN: u32 = 2;
pub const DAC_MISC_NBITS: u32 = 0;

// DAC Precharge Control Register
pub const DAC_PRECH: u32 = 0x0C;
pub const DAC_PRECH_PRCHGPDRV: u32 = 7;
pub const DAC_PRECH_PRCHGAUX1: u32 = 6;
pub const DAC_PRECH_PRCHGLNOR: u32 = 5;
pub const DAC_PRECH_PRCHGLNOL: u32 = 4;
pub const DAC_PRECH_PRCHGLNIR: u32 = 3;
pub const DAC_PRECH_PRCHGLNIL: u32 = 2;
pub const DAC_PRECH_PRCHG: u32 = 1;
pub const DAC_PRECH_ONMSTR: u32 = 0;

// DAC Auxiliary Input Gain Control Register
pub const DAC_AUXG: u32 = 0x0D;
pub const DAC_AUXG_AUXG: u32 = 0;

// DAC Reset Register
pub const DAC_RST: u32 = 0x10;
pub const DAC_RST_RESMASK: u32 = 2;
pub const DAC_RST_RESFILZ: u32 = 1;
pub const DAC_RST_RSTZ: u32 = 0;

// Power Amplifier Control Register
pub const PA_CTRL: u32 = 0x11;
pub const PA_CTRL_APAON: u32 = 6;
pub const PA_CTRL_APAPRECH: u32 = 5;
pub const PA_CTRL_APALP: u32 = 4;
pub const PA_CTRL_APAGAIN: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
