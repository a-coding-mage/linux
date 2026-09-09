/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Header file for ES488/ES1688
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *  The declarations referenced from the original sound/control.h,
 *  sound/pcm.h, and linux/interrupt.h headers are supplied by other files.
 */

pub const ES1688_HW_AUTO: u16 = 0x0000;
pub const ES1688_HW_688: u16 = 0x0001;
pub const ES1688_HW_1688: u16 = 0x0002;
pub const ES1688_HW_UNDEF: u16 = 0x0003;

#[repr(C)]
pub struct snd_es1688 {
    pub card: *mut snd_card,
    pub port: ::core::ffi::c_ulong, /* port of ESS chip */
    pub res_port: *mut resource,
    pub mpu_port: ::core::ffi::c_ulong, /* MPU-401 port of ESS chip */
    pub irq: ::core::ffi::c_int, /* IRQ number of ESS chip */
    pub mpu_irq: ::core::ffi::c_int, /* MPU IRQ */
    pub dma8: ::core::ffi::c_int, /* 8-bit DMA */
    pub version: u16, /* version of ESS chip */
    pub hardware: u16, /* see to ES1688_HW_XXXX */
    pub trigger_value: u16,
    pub pad: u8,
    pub dma_size: ::core::ffi::c_uint,
    pub pcm: *mut snd_pcm,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub reg_lock: spinlock_t,
    pub mixer_lock: spinlock_t,
}

/* I/O ports */

/* ES1688P(codec, x) ((codec)->port + e_s_s_ESS1688##x) */
#[inline]
pub unsafe fn ES1688P(codec: *const snd_es1688, x: usize) -> usize {
    (*codec).port as usize + x
}

pub const e_s_s_ESS1688RESET: usize = 0x6;
pub const e_s_s_ESS1688READ: usize = 0xa;
pub const e_s_s_ESS1688WRITE: usize = 0xc;
pub const e_s_s_ESS1688COMMAND: usize = 0xc;
pub const e_s_s_ESS1688STATUS: usize = 0xc;
pub const e_s_s_ESS1688DATA_AVAIL: usize = 0xe;
pub const e_s_s_ESS1688DATA_AVAIL_16: usize = 0xf;
pub const e_s_s_ESS1688MIXER_ADDR: usize = 0x4;
pub const e_s_s_ESS1688MIXER_DATA: usize = 0x5;
pub const e_s_s_ESS1688OPL3_LEFT: usize = 0x0;
pub const e_s_s_ESS1688OPL3_RIGHT: usize = 0x2;
pub const e_s_s_ESS1688OPL3_BOTH: usize = 0x8;
pub const e_s_s_ESS1688ENABLE0: usize = 0x0;
pub const e_s_s_ESS1688ENABLE1: usize = 0x9;
pub const e_s_s_ESS1688ENABLE2: usize = 0xb;
pub const e_s_s_ESS1688INIT1: usize = 0x7;

pub const ES1688_DSP_CMD_DMAOFF: u8 = 0xd0;
pub const ES1688_DSP_CMD_SPKON: u8 = 0xd1;
pub const ES1688_DSP_CMD_SPKOFF: u8 = 0xd3;
pub const ES1688_DSP_CMD_DMAON: u8 = 0xd4;

pub const ES1688_PCM_DEV: u8 = 0x14;
pub const ES1688_MIC_DEV: u8 = 0x1a;
pub const ES1688_REC_DEV: u8 = 0x1c;
pub const ES1688_MASTER_DEV: u8 = 0x32;
pub const ES1688_FM_DEV: u8 = 0x36;
pub const ES1688_CD_DEV: u8 = 0x38;
pub const ES1688_AUX_DEV: u8 = 0x3a;
pub const ES1688_SPEAKER_DEV: u8 = 0x3c;
pub const ES1688_LINE_DEV: u8 = 0x3e;
pub const ES1688_RECLEV_DEV: u8 = 0xb4;

pub const ES1688_MIXS_MASK: u8 = 0x17;
pub const ES1688_MIXS_MIC: u8 = 0x00;
pub const ES1688_MIXS_MIC_MASTER: u8 = 0x01;
pub const ES1688_MIXS_CD: u8 = 0x02;
pub const ES1688_MIXS_AOUT: u8 = 0x03;
pub const ES1688_MIXS_MIC1: u8 = 0x04;
pub const ES1688_MIXS_REC_MIX: u8 = 0x05;
pub const ES1688_MIXS_LINE: u8 = 0x06;
pub const ES1688_MIXS_MASTER: u8 = 0x07;
pub const ES1688_MIXS_MUTE: u8 = 0x10;

unsafe extern "C" {
    pub fn snd_es1688_mixer_write(chip: *mut snd_es1688, reg: u8, data: u8);
    pub fn snd_es1688_create(
        card: *mut snd_card,
        chip: *mut snd_es1688,
        port: ::core::ffi::c_ulong,
        mpu_port: ::core::ffi::c_ulong,
        irq: ::core::ffi::c_int,
        mpu_irq: ::core::ffi::c_int,
        dma8: ::core::ffi::c_int,
        hardware: u16,
    ) -> ::core::ffi::c_int;
    pub fn snd_es1688_pcm(
        card: *mut snd_card,
        chip: *mut snd_es1688,
        device: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_es1688_mixer(card: *mut snd_card, chip: *mut snd_es1688) -> ::core::ffi::c_int;
    pub fn snd_es1688_reset(chip: *mut snd_es1688) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
