/* SPDX-License-Identifier: GPL-2.0-or-later */
/*********************************************************************
 *
 * msnd_pinnacle.h
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

pub const DSP_NUMIO: u32 = 0x08;

pub const IREG_LOGDEVICE: u32 = 0x07;
pub const IREG_ACTIVATE: u32 = 0x30;
pub const LD_ACTIVATE: u32 = 0x01;
pub const LD_DISACTIVATE: u32 = 0x00;
pub const IREG_EECONTROL: u32 = 0x3F;
pub const IREG_MEMBASEHI: u32 = 0x40;
pub const IREG_MEMBASELO: u32 = 0x41;
pub const IREG_MEMCONTROL: u32 = 0x42;
pub const IREG_MEMRANGEHI: u32 = 0x43;
pub const IREG_MEMRANGELO: u32 = 0x44;
pub const MEMTYPE_8BIT: u32 = 0x00;
pub const MEMTYPE_16BIT: u32 = 0x02;
pub const MEMTYPE_RANGE: u32 = 0x00;
pub const MEMTYPE_HIADDR: u32 = 0x01;
pub const IREG_IO0_BASEHI: u32 = 0x60;
pub const IREG_IO0_BASELO: u32 = 0x61;
pub const IREG_IO1_BASEHI: u32 = 0x62;
pub const IREG_IO1_BASELO: u32 = 0x63;
pub const IREG_IRQ_NUMBER: u32 = 0x70;
pub const IREG_IRQ_TYPE: u32 = 0x71;
pub const IRQTYPE_HIGH: u32 = 0x02;
pub const IRQTYPE_LOW: u32 = 0x00;
pub const IRQTYPE_LEVEL: u32 = 0x01;
pub const IRQTYPE_EDGE: u32 = 0x00;

pub const HP_DSPR: u32 = 0x04;
pub const HP_BLKS: u32 = 0x04;

pub const HPDSPRESET_OFF: u32 = 2;
pub const HPDSPRESET_ON: u32 = 0;

pub const HPBLKSEL_0: u32 = 2;
pub const HPBLKSEL_1: u32 = 3;

pub const HIMT_DAT_OFF: u32 = 0x03;

pub const HIDSP_PLAY_UNDER: u32 = 0x00;
pub const HIDSP_INT_PLAY_UNDER: u32 = 0x01;
pub const HIDSP_SSI_TX_UNDER: u32 = 0x02;
pub const HIDSP_RECQ_OVERFLOW: u32 = 0x08;
pub const HIDSP_INT_RECORD_OVER: u32 = 0x09;
pub const HIDSP_SSI_RX_OVERFLOW: u32 = 0x0a;

pub const HIDSP_MIDI_IN_OVER: u32 = 0x10;

pub const HIDSP_MIDI_FRAME_ERR: u32 = 0x11;
pub const HIDSP_MIDI_PARITY_ERR: u32 = 0x12;
pub const HIDSP_MIDI_OVERRUN_ERR: u32 = 0x13;

pub const HIDSP_INPUT_CLIPPING: u32 = 0x20;
pub const HIDSP_MIX_CLIPPING: u32 = 0x30;
pub const HIDSP_DAT_IN_OFF: u32 = 0x21;

pub const TIME_PRO_RESET_DONE: u32 = 0x028A;
pub const TIME_PRO_SYSEX: u32 = 0x001E;
pub const TIME_PRO_RESET: u32 = 0x0032;

pub const DAR_BUFF_SIZE: u32 = 0x1000;

pub const MIDQ_BUFF_SIZE: u32 = 0x800;
pub const DSPQ_BUFF_SIZE: u32 = 0x5A0;

pub const DSPQ_DATA_BUFF: u32 = 0x7860;

pub const MOP_WAVEHDR: u32 = 0;
pub const MOP_EXTOUT: u32 = 1;
pub const MOP_HWINIT: u32 = 0xfe;
pub const MOP_NONE: u32 = 0xff;
pub const MOP_MAX: u32 = 1;

pub const MIP_EXTIN: u32 = 0;
pub const MIP_WAVEHDR: u32 = 1;
pub const MIP_HWINIT: u32 = 0xfe;
pub const MIP_MAX: u32 = 1;

/* Pinnacle/Fiji SMA Common Data */
pub const SMA_wCurrPlayBytes: u32 = 0x0000;
pub const SMA_wCurrRecordBytes: u32 = 0x0002;
pub const SMA_wCurrPlayVolLeft: u32 = 0x0004;
pub const SMA_wCurrPlayVolRight: u32 = 0x0006;
pub const SMA_wCurrInVolLeft: u32 = 0x0008;
pub const SMA_wCurrInVolRight: u32 = 0x000a;
pub const SMA_wCurrMHdrVolLeft: u32 = 0x000c;
pub const SMA_wCurrMHdrVolRight: u32 = 0x000e;
pub const SMA_dwCurrPlayPitch: u32 = 0x0010;
pub const SMA_dwCurrPlayRate: u32 = 0x0014;
pub const SMA_wCurrMIDIIOPatch: u32 = 0x0018;
pub const SMA_wCurrPlayFormat: u32 = 0x001a;
pub const SMA_wCurrPlaySampleSize: u32 = 0x001c;
pub const SMA_wCurrPlayChannels: u32 = 0x001e;
pub const SMA_wCurrPlaySampleRate: u32 = 0x0020;
pub const SMA_wCurrRecordFormat: u32 = 0x0022;
pub const SMA_wCurrRecordSampleSize: u32 = 0x0024;
pub const SMA_wCurrRecordChannels: u32 = 0x0026;
pub const SMA_wCurrRecordSampleRate: u32 = 0x0028;
pub const SMA_wCurrDSPStatusFlags: u32 = 0x002a;
pub const SMA_wCurrHostStatusFlags: u32 = 0x002c;
pub const SMA_wCurrInputTagBits: u32 = 0x002e;
pub const SMA_wCurrLeftPeak: u32 = 0x0030;
pub const SMA_wCurrRightPeak: u32 = 0x0032;
pub const SMA_bMicPotPosLeft: u32 = 0x0034;
pub const SMA_bMicPotPosRight: u32 = 0x0035;
pub const SMA_bMicPotMaxLeft: u32 = 0x0036;
pub const SMA_bMicPotMaxRight: u32 = 0x0037;
pub const SMA_bInPotPosLeft: u32 = 0x0038;
pub const SMA_bInPotPosRight: u32 = 0x0039;
pub const SMA_bAuxPotPosLeft: u32 = 0x003a;
pub const SMA_bAuxPotPosRight: u32 = 0x003b;
pub const SMA_bInPotMaxLeft: u32 = 0x003c;
pub const SMA_bInPotMaxRight: u32 = 0x003d;
pub const SMA_bAuxPotMaxLeft: u32 = 0x003e;
pub const SMA_bAuxPotMaxRight: u32 = 0x003f;
pub const SMA_bInPotMaxMethod: u32 = 0x0040;
pub const SMA_bAuxPotMaxMethod: u32 = 0x0041;
pub const SMA_wCurrMastVolLeft: u32 = 0x0042;
pub const SMA_wCurrMastVolRight: u32 = 0x0044;
pub const SMA_wCalFreqAtoD: u32 = 0x0046;
pub const SMA_wCurrAuxVolLeft: u32 = 0x0048;
pub const SMA_wCurrAuxVolRight: u32 = 0x004a;
pub const SMA_wCurrPlay1VolLeft: u32 = 0x004c;
pub const SMA_wCurrPlay1VolRight: u32 = 0x004e;
pub const SMA_wCurrPlay2VolLeft: u32 = 0x0050;
pub const SMA_wCurrPlay2VolRight: u32 = 0x0052;
pub const SMA_wCurrPlay3VolLeft: u32 = 0x0054;
pub const SMA_wCurrPlay3VolRight: u32 = 0x0056;
pub const SMA_wCurrPlay4VolLeft: u32 = 0x0058;
pub const SMA_wCurrPlay4VolRight: u32 = 0x005a;
pub const SMA_wCurrPlay1PeakLeft: u32 = 0x005c;
pub const SMA_wCurrPlay1PeakRight: u32 = 0x005e;
pub const SMA_wCurrPlay2PeakLeft: u32 = 0x0060;
pub const SMA_wCurrPlay2PeakRight: u32 = 0x0062;
pub const SMA_wCurrPlay3PeakLeft: u32 = 0x0064;
pub const SMA_wCurrPlay3PeakRight: u32 = 0x0066;
pub const SMA_wCurrPlay4PeakLeft: u32 = 0x0068;
pub const SMA_wCurrPlay4PeakRight: u32 = 0x006a;
pub const SMA_wCurrPlayPeakLeft: u32 = 0x006c;
pub const SMA_wCurrPlayPeakRight: u32 = 0x006e;
pub const SMA_wCurrDATSR: u32 = 0x0070;
pub const SMA_wCurrDATRXCHNL: u32 = 0x0072;
pub const SMA_wCurrDATTXCHNL: u32 = 0x0074;
pub const SMA_wCurrDATRXRate: u32 = 0x0076;
pub const SMA_dwDSPPlayCount: u32 = 0x0078;
pub const SMA__size: u32 = 0x007c;

pub const INITCODEFILE: &str = "turtlebeach/pndspini.bin";
pub const PERMCODEFILE: &str = "turtlebeach/pndsperm.bin";
pub const LONGNAME: &str = "MultiSound (Pinnacle/Fiji)";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
