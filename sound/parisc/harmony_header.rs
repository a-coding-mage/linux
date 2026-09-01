/* SPDX-License-Identifier: GPL-2.0 */
/* Hewlett-Packard Harmony audio driver
 * Copyright (C) 2004, Kyle McMartin <kyle@parisc-linux.org>
 */

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct harmony_buffer {
    pub addr: c_ulong,
    pub buf: c_int,
    pub count: c_int,
    pub size: c_int,
    pub coherent: c_int,
}

#[repr(C)]
pub struct snd_harmony_st {
    pub gain: u32,
    pub rate: u32,
    pub format: u32,
    pub stereo: u32,
    pub playing: c_int,
    pub capturing: c_int,
}

#[repr(C)]
pub struct snd_harmony_stats {
    pub play_intr: c_ulong,
    pub rec_intr: c_ulong,
    pub graveyard_intr: c_ulong,
    pub silence_intr: c_ulong,
}

#[repr(C)]
pub struct snd_harmony {
    pub irq: c_int,

    pub hpa: c_ulong,          /* hard physical address */
    pub iobase: *mut c_void,   /* remapped io address; C had __iomem */

    pub dev: *mut parisc_device,

    pub st: snd_harmony_st,

    pub dma: snd_dma_device, /* playback/capture */
    pub pbuf: harmony_buffer,
    pub cbuf: harmony_buffer,

    pub gdma: snd_dma_buffer, /* graveyard */
    pub sdma: snd_dma_buffer, /* silence */

    pub stats: snd_harmony_stats,

    pub pcm: *mut snd_pcm,
    pub card: *mut snd_card,
    pub psubs: *mut snd_pcm_substream,
    pub csubs: *mut snd_pcm_substream,
    pub proc: *mut snd_info_entry,

    pub lock: spinlock_t,
    pub mixer_lock: spinlock_t,
}

pub const MAX_PCM_DEVICES: usize = 1;
pub const MAX_PCM_SUBSTREAMS: usize = 4;
pub const MAX_MIDI_DEVICES: usize = 0;

pub const HARMONY_SIZE: usize = 64;

pub const BUF_SIZE: usize = PAGE_SIZE;
pub const MAX_BUFS: usize = 16;
pub const MAX_BUF_SIZE: usize = MAX_BUFS * BUF_SIZE;

pub const PLAYBACK_BUFS: usize = MAX_BUFS;
pub const RECORD_BUFS: usize = MAX_BUFS;
pub const GRAVEYARD_BUFS: usize = 1;
pub const GRAVEYARD_BUFSZ: usize = GRAVEYARD_BUFS * BUF_SIZE;
pub const SILENCE_BUFS: usize = 1;
pub const SILENCE_BUFSZ: usize = SILENCE_BUFS * BUF_SIZE;

pub const HARMONY_ID: u32 = 0x000;
pub const HARMONY_RESET: u32 = 0x004;
pub const HARMONY_CNTL: u32 = 0x008;
pub const HARMONY_GAINCTL: u32 = 0x00c;
pub const HARMONY_PNXTADD: u32 = 0x010;
pub const HARMONY_PCURADD: u32 = 0x014;
pub const HARMONY_RNXTADD: u32 = 0x018;
pub const HARMONY_RCURADD: u32 = 0x01c;
pub const HARMONY_DSTATUS: u32 = 0x020;
pub const HARMONY_OV: u32 = 0x024;
pub const HARMONY_PIO: u32 = 0x028;
pub const HARMONY_DIAG: u32 = 0x03c;

pub const HARMONY_CNTL_C: u32 = 0x80000000;
pub const HARMONY_CNTL_ST: u32 = 0x00000020;
pub const HARMONY_CNTL_44100: u32 = 0x00000015; /* HARMONY_SR_44KHZ */
pub const HARMONY_CNTL_8000: u32 = 0x00000008; /* HARMONY_SR_8KHZ */

pub const HARMONY_DSTATUS_ID: u32 = 0x00000000; /* interrupts off */
pub const HARMONY_DSTATUS_PN: u32 = 0x00000200; /* playback fill */
pub const HARMONY_DSTATUS_RN: u32 = 0x00000002; /* record fill */
pub const HARMONY_DSTATUS_IE: u32 = 0x80000000; /* interrupts on */

pub const HARMONY_DF_16BIT_LINEAR: u32 = 0x00000000;
pub const HARMONY_DF_8BIT_ULAW: u32 = 0x00000001;
pub const HARMONY_DF_8BIT_ALAW: u32 = 0x00000002;

pub const HARMONY_SS_MONO: u32 = 0x00000000;
pub const HARMONY_SS_STEREO: u32 = 0x00000001;

pub const HARMONY_GAIN_SILENCE: u32 = 0x01F00FFF;
pub const HARMONY_GAIN_DEFAULT: u32 = 0x01F00FFF;

pub const HARMONY_GAIN_HE_SHIFT: u32 = 27; /* headphones enabled */
pub const HARMONY_GAIN_HE_MASK: u32 = 1_u32 << HARMONY_GAIN_HE_SHIFT;
pub const HARMONY_GAIN_LE_SHIFT: u32 = 26; /* line-out enabled */
pub const HARMONY_GAIN_LE_MASK: u32 = 1_u32 << HARMONY_GAIN_LE_SHIFT;
pub const HARMONY_GAIN_SE_SHIFT: u32 = 25; /* internal-speaker enabled */
pub const HARMONY_GAIN_SE_MASK: u32 = 1_u32 << HARMONY_GAIN_SE_SHIFT;
pub const HARMONY_GAIN_IS_SHIFT: u32 = 24; /* input select - 0 for line, 1 for mic */
pub const HARMONY_GAIN_IS_MASK: u32 = 1_u32 << HARMONY_GAIN_IS_SHIFT;

/* monitor attenuation */
pub const HARMONY_GAIN_MA: u32 = 0x0f;
pub const HARMONY_GAIN_MA_SHIFT: u32 = 20;
pub const HARMONY_GAIN_MA_MASK: u32 = HARMONY_GAIN_MA << HARMONY_GAIN_MA_SHIFT;

/* input gain */
pub const HARMONY_GAIN_IN: u32 = 0x0f;
pub const HARMONY_GAIN_LI_SHIFT: u32 = 16;
pub const HARMONY_GAIN_LI_MASK: u32 = HARMONY_GAIN_IN << HARMONY_GAIN_LI_SHIFT;
pub const HARMONY_GAIN_RI_SHIFT: u32 = 12;
pub const HARMONY_GAIN_RI_MASK: u32 = HARMONY_GAIN_IN << HARMONY_GAIN_RI_SHIFT;

/* output gain (master volume) */
pub const HARMONY_GAIN_OUT: u32 = 0x3f;
pub const HARMONY_GAIN_LO_SHIFT: u32 = 6;
pub const HARMONY_GAIN_LO_MASK: u32 = HARMONY_GAIN_OUT << HARMONY_GAIN_LO_SHIFT;
pub const HARMONY_GAIN_RO_SHIFT: u32 = 0;
pub const HARMONY_GAIN_RO_MASK: u32 = HARMONY_GAIN_OUT << HARMONY_GAIN_RO_SHIFT;

pub const HARMONY_MAX_OUT: u32 = HARMONY_GAIN_RO_MASK >> HARMONY_GAIN_RO_SHIFT;
pub const HARMONY_MAX_IN: u32 = HARMONY_GAIN_RI_MASK >> HARMONY_GAIN_RI_SHIFT;
pub const HARMONY_MAX_MON: u32 = HARMONY_GAIN_MA_MASK >> HARMONY_GAIN_MA_SHIFT;

pub const HARMONY_SR_8KHZ: u32 = 0x08;
pub const HARMONY_SR_16KHZ: u32 = 0x09;
pub const HARMONY_SR_27KHZ: u32 = 0x0A;
pub const HARMONY_SR_32KHZ: u32 = 0x0B;
pub const HARMONY_SR_48KHZ: u32 = 0x0E;
pub const HARMONY_SR_9KHZ: u32 = 0x0F;
pub const HARMONY_SR_5KHZ: u32 = 0x10;
pub const HARMONY_SR_11KHZ: u32 = 0x11;
pub const HARMONY_SR_18KHZ: u32 = 0x12;
pub const HARMONY_SR_22KHZ: u32 = 0x13;
pub const HARMONY_SR_37KHZ: u32 = 0x14;
pub const HARMONY_SR_44KHZ: u32 = 0x15;
pub const HARMONY_SR_33KHZ: u32 = 0x16;
pub const HARMONY_SR_6KHZ: u32 = 0x17;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
