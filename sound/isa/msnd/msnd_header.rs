/* SPDX-License-Identifier: GPL-2.0-or-later */
/*********************************************************************
 *
 * msnd.h
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

use core::ffi::{c_int, c_long, c_uchar, c_uint, c_ulong, c_void};

/* C header dependency: <sound/pcm.h> */

pub const DEFSAMPLERATE: c_int = 44100;
pub const DEFSAMPLESIZE: c_int = SNDRV_PCM_FORMAT_S16;
pub const DEFCHANNELS: c_int = 1;

pub const SRAM_BANK_SIZE: c_int = 0x8000;
pub const SRAM_CNTL_START: c_int = 0x7F00;
pub const SMA_STRUCT_START: c_int = 0x7F40;

pub const DSP_BASE_ADDR: c_int = 0x4000;
pub const DSP_BANK_BASE: c_int = 0x4000;

pub const AGND: c_int = 0x01;
pub const SIGNAL: c_int = 0x02;

pub const EXT_DSP_BIT_DCAL: c_int = 0x0001;
pub const EXT_DSP_BIT_MIDI_CON: c_int = 0x0002;

pub const BUFFSIZE: c_int = 0x8000;
pub const HOSTQ_SIZE: c_int = 0x40;

pub const DAP_BUFF_SIZE: c_int = 0x2400;

pub const DAPQ_STRUCT_SIZE: c_int = 0x10;
pub const DARQ_STRUCT_SIZE: c_int = 0x10;
pub const DAPQ_BUFF_SIZE: c_int = 3 * 0x10;
pub const DARQ_BUFF_SIZE: c_int = 3 * 0x10;
pub const MODQ_BUFF_SIZE: c_int = 0x400;

pub const DAPQ_DATA_BUFF: c_int = 0x6C00;
pub const DARQ_DATA_BUFF: c_int = 0x6C30;
pub const MODQ_DATA_BUFF: c_int = 0x6C60;
pub const MIDQ_DATA_BUFF: c_int = 0x7060;

pub const DAPQ_OFFSET: c_int = SRAM_CNTL_START;
pub const DARQ_OFFSET: c_int = SRAM_CNTL_START + 0x08;
pub const MODQ_OFFSET: c_int = SRAM_CNTL_START + 0x10;
pub const MIDQ_OFFSET: c_int = SRAM_CNTL_START + 0x18;
pub const DSPQ_OFFSET: c_int = SRAM_CNTL_START + 0x20;

pub const HP_ICR: c_int = 0x00;
pub const HP_CVR: c_int = 0x01;
pub const HP_ISR: c_int = 0x02;
pub const HP_IVR: c_int = 0x03;
pub const HP_NU: c_int = 0x04;
pub const HP_INFO: c_int = 0x04;
pub const HP_TXH: c_int = 0x05;
pub const HP_RXH: c_int = 0x05;
pub const HP_TXM: c_int = 0x06;
pub const HP_RXM: c_int = 0x06;
pub const HP_TXL: c_int = 0x07;
pub const HP_RXL: c_int = 0x07;

pub const HP_ICR_DEF: c_int = 0x00;
pub const HP_CVR_DEF: c_int = 0x12;
pub const HP_ISR_DEF: c_int = 0x06;
pub const HP_IVR_DEF: c_int = 0x0f;
pub const HP_NU_DEF: c_int = 0x00;

pub const HP_IRQM: c_int = 0x09;

pub const HPR_BLRC: c_int = 0x08;
pub const HPR_SPR1: c_int = 0x09;
pub const HPR_SPR2: c_int = 0x0A;
pub const HPR_TCL0: c_int = 0x0B;
pub const HPR_TCL1: c_int = 0x0C;
pub const HPR_TCL2: c_int = 0x0D;
pub const HPR_TCL3: c_int = 0x0E;
pub const HPR_TCL4: c_int = 0x0F;

pub const HPICR_INIT: c_int = 0x80;
pub const HPICR_HM1: c_int = 0x40;
pub const HPICR_HM0: c_int = 0x20;
pub const HPICR_HF1: c_int = 0x10;
pub const HPICR_HF0: c_int = 0x08;
pub const HPICR_TREQ: c_int = 0x02;
pub const HPICR_RREQ: c_int = 0x01;

pub const HPCVR_HC: c_int = 0x80;

pub const HPISR_HREQ: c_int = 0x80;
pub const HPISR_DMA: c_int = 0x40;
pub const HPISR_HF3: c_int = 0x10;
pub const HPISR_HF2: c_int = 0x08;
pub const HPISR_TRDY: c_int = 0x04;
pub const HPISR_TXDE: c_int = 0x02;
pub const HPISR_RXDF: c_int = 0x01;

pub const HPIO_290: c_int = 0;
pub const HPIO_260: c_int = 1;
pub const HPIO_250: c_int = 2;
pub const HPIO_240: c_int = 3;
pub const HPIO_230: c_int = 4;
pub const HPIO_220: c_int = 5;
pub const HPIO_210: c_int = 6;
pub const HPIO_3E0: c_int = 7;

pub const HPMEM_NONE: c_int = 0;
pub const HPMEM_B000: c_int = 1;
pub const HPMEM_C800: c_int = 2;
pub const HPMEM_D000: c_int = 3;
pub const HPMEM_D400: c_int = 4;
pub const HPMEM_D800: c_int = 5;
pub const HPMEM_E000: c_int = 6;
pub const HPMEM_E800: c_int = 7;

pub const HPIRQ_NONE: c_int = 0;
pub const HPIRQ_5: c_int = 1;
pub const HPIRQ_7: c_int = 2;
pub const HPIRQ_9: c_int = 3;
pub const HPIRQ_10: c_int = 4;
pub const HPIRQ_11: c_int = 5;
pub const HPIRQ_12: c_int = 6;
pub const HPIRQ_15: c_int = 7;

pub const HIMT_PLAY_DONE: c_int = 0x00;
pub const HIMT_RECORD_DONE: c_int = 0x01;
pub const HIMT_MIDI_EOS: c_int = 0x02;
pub const HIMT_MIDI_OUT: c_int = 0x03;

pub const HIMT_MIDI_IN_UCHAR: c_int = 0x0E;
pub const HIMT_DSP: c_int = 0x0F;

pub const HDEX_BASE: c_int = 0x92;
pub const HDEX_PLAY_START: c_int = 0 + HDEX_BASE;
pub const HDEX_PLAY_STOP: c_int = 1 + HDEX_BASE;
pub const HDEX_PLAY_PAUSE: c_int = 2 + HDEX_BASE;
pub const HDEX_PLAY_RESUME: c_int = 3 + HDEX_BASE;
pub const HDEX_RECORD_START: c_int = 4 + HDEX_BASE;
pub const HDEX_RECORD_STOP: c_int = 5 + HDEX_BASE;
pub const HDEX_MIDI_IN_START: c_int = 6 + HDEX_BASE;
pub const HDEX_MIDI_IN_STOP: c_int = 7 + HDEX_BASE;
pub const HDEX_MIDI_OUT_START: c_int = 8 + HDEX_BASE;
pub const HDEX_MIDI_OUT_STOP: c_int = 9 + HDEX_BASE;
pub const HDEX_AUX_REQ: c_int = 10 + HDEX_BASE;

pub const HDEXAR_CLEAR_PEAKS: c_int = 1;
pub const HDEXAR_IN_SET_POTS: c_int = 2;
pub const HDEXAR_AUX_SET_POTS: c_int = 3;
pub const HDEXAR_CAL_A_TO_D: c_int = 4;
pub const HDEXAR_RD_EXT_DSP_BITS: c_int = 5;

/* Pinnacle only HDEXAR defs */
pub const HDEXAR_SET_ANA_IN: c_int = 0;
pub const HDEXAR_SET_SYNTH_IN: c_int = 4;
pub const HDEXAR_READ_DAT_IN: c_int = 5;
pub const HDEXAR_MIC_SET_POTS: c_int = 6;
pub const HDEXAR_SET_DAT_IN: c_int = 7;

pub const HDEXAR_SET_SYNTH_48: c_int = 8;
pub const HDEXAR_SET_SYNTH_44: c_int = 9;

pub const fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xFFFF) as u16
}

pub const fn LOWORD(l: u32) -> u16 {
    l as u16
}

pub const fn HIBYTE(w: u16) -> u8 {
    ((w >> 8) & 0xFF) as u8
}

pub const fn LOBYTE(w: u16) -> u8 {
    w as u8
}

pub const fn MAKELONG(low: u16, hi: u16) -> c_long {
    (low as u32 | ((hi as u32) << 16)) as c_long
}

pub const fn MAKEWORD(low: u8, hi: u8) -> u16 {
    low as u16 | ((hi as u16) << 8)
}

pub const fn PCTODSP_OFFSET(w: usize) -> u16 {
    (w / 2) as u16
}

pub const fn PCTODSP_BASED(w: usize) -> u16 {
    ((w / 2) + DSP_BASE_ADDR as usize) as u16
}

pub const fn DSPTOPC_BASED(w: usize) -> usize {
    (w - DSP_BASE_ADDR as usize) * 2
}

/*
 * If the C build defines SLOWIO, outb/inb are remapped to outb_p/inb_p.
 * This header-local preprocessor remapping has no direct Rust item equivalent.
 */

/* JobQueueStruct */
pub const JQS_wStart: c_int = 0x00;
pub const JQS_wSize: c_int = 0x02;
pub const JQS_wHead: c_int = 0x04;
pub const JQS_wTail: c_int = 0x06;
pub const JQS__size: c_int = 0x08;

/* DAQueueDataStruct */
pub const DAQDS_wStart: c_int = 0x00;
pub const DAQDS_wSize: c_int = 0x02;
pub const DAQDS_wFormat: c_int = 0x04;
pub const DAQDS_wSampleSize: c_int = 0x06;
pub const DAQDS_wChannels: c_int = 0x08;
pub const DAQDS_wSampleRate: c_int = 0x0A;
pub const DAQDS_wIntMsg: c_int = 0x0C;
pub const DAQDS_wFlags: c_int = 0x0E;
pub const DAQDS__size: c_int = 0x10;

#[repr(C)]
pub enum snd_msnd_type {
    msndClassic,
    msndPinnacle,
}

pub const F_RESETTING: c_int = 0;
pub const F_HAVEDIGITAL: c_int = 1;
pub const F_AUDIO_WRITE_INUSE: c_int = 2;
pub const F_WRITING: c_int = 3;
pub const F_WRITEBLOCK: c_int = 4;
pub const F_WRITEFLUSH: c_int = 5;
pub const F_AUDIO_READ_INUSE: c_int = 6;
pub const F_READING: c_int = 7;
pub const F_READBLOCK: c_int = 8;
pub const F_EXT_MIDI_INUSE: c_int = 9;
pub const F_HDR_MIDI_INUSE: c_int = 10;
pub const F_DISABLE_WRITE_NDELAY: c_int = 11;

pub const LEVEL_ENTRIES: usize = 32;

#[repr(C)]
pub struct snd_msnd {
    pub mappedbase: *mut c_void,
    pub play_period_bytes: c_int,
    pub playLimit: c_int,
    pub playPeriods: c_int,
    pub playDMAPos: c_int,
    pub banksPlayed: c_int,
    pub captureDMAPos: c_int,
    pub capturePeriodBytes: c_int,
    pub captureLimit: c_int,
    pub capturePeriods: c_int,
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,

    /* Hardware resources */
    pub io: c_long,
    pub memid: c_int,
    pub irqid: c_int,
    pub irq: c_int,
    pub irq_ref: c_int,
    pub base: c_ulong,

    /* Motorola 56k DSP SMA */
    pub SMA: *mut c_void,
    pub DAPQ: *mut c_void,
    pub DARQ: *mut c_void,
    pub MODQ: *mut c_void,
    pub MIDQ: *mut c_void,
    pub DSPQ: *mut c_void,
    pub dspq_data_buff: c_int,
    pub dspq_buff_size: c_int,

    /* State variables */
    pub type_: snd_msnd_type,
    pub mode: fmode_t,
    pub flags: c_ulong,
    pub lock: spinlock_t,
    pub mixer_lock: spinlock_t,
    pub nresets: c_int,
    pub recsrc: c_uint,
    pub pm_recsrc: u8,
    pub pm_mpu_input: bool,
    pub left_levels: [c_int; LEVEL_ENTRIES],
    pub right_levels: [c_int; LEVEL_ENTRIES],
    pub calibrate_signal: c_int,
    pub play_sample_size: c_int,
    pub play_sample_rate: c_int,
    pub play_channels: c_int,
    pub play_ndelay: c_int,
    pub capture_sample_size: c_int,
    pub capture_sample_rate: c_int,
    pub capture_channels: c_int,
    pub capture_ndelay: c_int,
    pub bCurrentMidiPatch: u8,

    pub last_playbank: c_int,
    pub last_recbank: c_int,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
}

unsafe extern "C" {
    pub fn snd_msnd_init_queue(base: *mut c_void, start: c_int, size: c_int);

    pub fn snd_msnd_send_dsp_cmd(chip: *mut snd_msnd, cmd: u8) -> c_int;
    pub fn snd_msnd_send_word(
        chip: *mut snd_msnd,
        high: c_uchar,
        mid: c_uchar,
        low: c_uchar,
    ) -> c_int;
    pub fn snd_msnd_upload_host(chip: *mut snd_msnd, bin: *const u8, len: c_int) -> c_int;
    pub fn snd_msnd_enable_irq(chip: *mut snd_msnd) -> c_int;
    pub fn snd_msnd_disable_irq(chip: *mut snd_msnd) -> c_int;
    pub fn snd_msnd_force_irq(chip: *mut snd_msnd, enable: bool) -> c_int;
    pub fn snd_msnd_dsp_halt(chip: *mut snd_msnd, file: *mut file);
    pub fn snd_msnd_DAPQ(chip: *mut snd_msnd, start: c_int) -> c_int;
    pub fn snd_msnd_DARQ(chip: *mut snd_msnd, start: c_int) -> c_int;
    pub fn snd_msnd_pcm(card: *mut snd_card, device: c_int) -> c_int;

    pub fn snd_msndmix_setup(chip: *mut snd_msnd);
    pub fn snd_msndmix_new(card: *mut snd_card) -> c_int;
    pub fn snd_msndmix_force_recsrc(chip: *mut snd_msnd, recsrc: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
