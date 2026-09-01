// SPDX-License-Identifier: GPL-2.0-or-later
/*********************************************************************
 *
 * msnd_classic.h
 *
 * Turtle Beach MultiSound Sound Card Driver for Linux
 *
 * Some parts of this header file were derived from the Turtle Beach
 * MultiSound Driver Development Kit.
 *
 * Copyright (C) 1998 Andrew Veliath
 * Copyright (C) 1993 Turtle Beach Systems, Inc.
 *
 ********************************************************************/

pub const DSP_NUMIO: u32 = 0x10;

pub const HP_MEMM: u32 = 0x08;

pub const HP_BITM: u32 = 0x0E;
pub const HP_WAIT: u32 = 0x0D;
pub const HP_DSPR: u32 = 0x0A;
pub const HP_PROR: u32 = 0x0B;
pub const HP_BLKS: u32 = 0x0C;

pub const HPPRORESET_OFF: u32 = 0;
pub const HPPRORESET_ON: u32 = 1;

pub const HPDSPRESET_OFF: u32 = 0;
pub const HPDSPRESET_ON: u32 = 1;

pub const HPBLKSEL_0: u32 = 0;
pub const HPBLKSEL_1: u32 = 1;

pub const HPWAITSTATE_0: u32 = 0;
pub const HPWAITSTATE_1: u32 = 1;

pub const HPBITMODE_16: u32 = 0;
pub const HPBITMODE_8: u32 = 1;

pub const HIDSP_INT_PLAY_UNDER: u32 = 0x00;
pub const HIDSP_INT_RECORD_OVER: u32 = 0x01;
pub const HIDSP_INPUT_CLIPPING: u32 = 0x02;
pub const HIDSP_MIDI_IN_OVER: u32 = 0x10;
pub const HIDSP_MIDI_OVERRUN_ERR: u32 = 0x13;

pub const TIME_PRO_RESET_DONE: u32 = 0x028A;
pub const TIME_PRO_SYSEX: u32 = 0x0040;
pub const TIME_PRO_RESET: u32 = 0x0032;

pub const DAR_BUFF_SIZE: u32 = 0x2000;

pub const MIDQ_BUFF_SIZE: u32 = 0x200;
pub const DSPQ_BUFF_SIZE: u32 = 0x40;

pub const DSPQ_DATA_BUFF: u32 = 0x7260;

pub const MOP_SYNTH: u32 = 0x10;
pub const MOP_EXTOUT: u32 = 0x32;
pub const MOP_EXTTHRU: u32 = 0x02;
pub const MOP_OUTMASK: u32 = 0x01;

pub const MIP_EXTIN: u32 = 0x01;
pub const MIP_SYNTH: u32 = 0x00;
pub const MIP_INMASK: u32 = 0x32;

/* Classic SMA Common Data */
pub const SMA_wCurrPlayBytes: u32 = 0x0000;
pub const SMA_wCurrRecordBytes: u32 = 0x0002;
pub const SMA_wCurrPlayVolLeft: u32 = 0x0004;
pub const SMA_wCurrPlayVolRight: u32 = 0x0006;
pub const SMA_wCurrInVolLeft: u32 = 0x0008;
pub const SMA_wCurrInVolRight: u32 = 0x000a;
pub const SMA_wUser_3: u32 = 0x000c;
pub const SMA_wUser_4: u32 = 0x000e;
pub const SMA_dwUser_5: u32 = 0x0010;
pub const SMA_dwUser_6: u32 = 0x0014;
pub const SMA_wUser_7: u32 = 0x0018;
pub const SMA_wReserved_A: u32 = 0x001a;
pub const SMA_wReserved_B: u32 = 0x001c;
pub const SMA_wReserved_C: u32 = 0x001e;
pub const SMA_wReserved_D: u32 = 0x0020;
pub const SMA_wReserved_E: u32 = 0x0022;
pub const SMA_wReserved_F: u32 = 0x0024;
pub const SMA_wReserved_G: u32 = 0x0026;
pub const SMA_wReserved_H: u32 = 0x0028;
pub const SMA_wCurrDSPStatusFlags: u32 = 0x002a;
pub const SMA_wCurrHostStatusFlags: u32 = 0x002c;
pub const SMA_wCurrInputTagBits: u32 = 0x002e;
pub const SMA_wCurrLeftPeak: u32 = 0x0030;
pub const SMA_wCurrRightPeak: u32 = 0x0032;
pub const SMA_wExtDSPbits: u32 = 0x0034;
pub const SMA_bExtHostbits: u32 = 0x0036;
pub const SMA_bBoardLevel: u32 = 0x0037;
pub const SMA_bInPotPosRight: u32 = 0x0038;
pub const SMA_bInPotPosLeft: u32 = 0x0039;
pub const SMA_bAuxPotPosRight: u32 = 0x003a;
pub const SMA_bAuxPotPosLeft: u32 = 0x003b;
pub const SMA_wCurrMastVolLeft: u32 = 0x003c;
pub const SMA_wCurrMastVolRight: u32 = 0x003e;
pub const SMA_bUser_12: u32 = 0x0040;
pub const SMA_bUser_13: u32 = 0x0041;
pub const SMA_wUser_14: u32 = 0x0042;
pub const SMA_wUser_15: u32 = 0x0044;
pub const SMA_wCalFreqAtoD: u32 = 0x0046;
pub const SMA_wUser_16: u32 = 0x0048;
pub const SMA_wUser_17: u32 = 0x004a;
pub const SMA__size: u32 = 0x004c;

pub const INITCODEFILE: &str = "turtlebeach/msndinit.bin";
pub const PERMCODEFILE: &str = "turtlebeach/msndperm.bin";
pub const LONGNAME: &str = "MultiSound (Classic/Monterey/Tahiti)";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
