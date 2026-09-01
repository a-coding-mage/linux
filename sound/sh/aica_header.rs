/* SPDX-License-Identifier: GPL-2.0-only */
/* aica.h
 * Header file for ALSA driver for
 * Sega Dreamcast Yamaha AICA sound
 * Copyright Adrian McMenamin
 * <adrian@mcmen.demon.co.uk>
 * 2006
 */

/* Original C header dependencies are expected to provide these types. */
use crate::{snd_card, snd_pcm_substream, timer_list, work_struct};

/* SPU memory and register constants etc */
pub const G2_FIFO: u32 = 0xa05f688c;
pub const SPU_MEMORY_BASE: u32 = 0xA0800000;
pub const ARM_RESET_REGISTER: u32 = 0xA0702C00;
pub const SPU_REGISTER_BASE: u32 = 0xA0700000;

/* AICA channels stuff */
pub const AICA_CONTROL_POINT: u32 = 0xA0810000;
pub const AICA_CONTROL_CHANNEL_SAMPLE_NUMBER: u32 = 0xA0810008;
pub const AICA_CHANNEL0_CONTROL_OFFSET: u32 = 0x10004;

/* Command values */
pub const AICA_CMD_KICK: u32 = 0x80000000;
pub const AICA_CMD_NONE: u32 = 0;
pub const AICA_CMD_START: u32 = 1;
pub const AICA_CMD_STOP: u32 = 2;
pub const AICA_CMD_VOL: u32 = 3;

/* Sound modes */
pub const SM_8BIT: u32 = 1;
pub const SM_16BIT: u32 = 0;
pub const SM_ADPCM: u32 = 2;

/* Buffer and period size */
pub const AICA_BUFFER_SIZE: u32 = 0x8000;
pub const AICA_PERIOD_SIZE: u32 = 0x800;
pub const AICA_PERIOD_NUMBER: u32 = 16;

pub const AICA_CHANNEL0_OFFSET: u32 = 0x11000;
pub const AICA_CHANNEL1_OFFSET: u32 = 0x21000;
pub const CHANNEL_OFFSET: u32 = 0x10000;

pub const AICA_DMA_CHANNEL: u32 = 5;
pub const AICA_DMA_MODE: u32 = 5;

pub const SND_AICA_DRIVER: &str = "AICA";

#[repr(C)]
pub struct aica_channel {
    pub cmd: u32,    /* Command ID           */
    pub pos: u32,    /* Sample position      */
    pub length: u32, /* Sample length        */
    pub freq: u32,   /* Frequency            */
    pub vol: u32,    /* Volume 0-255         */
    pub pan: u32,    /* Pan 0-255            */
    pub sfmt: u32,   /* Sound format         */
    pub flags: u32,  /* Bit flags            */
}

#[repr(C)]
pub struct snd_card_aica {
    pub spu_dma_work: work_struct,
    pub card: *mut snd_card,
    pub channel: *mut aica_channel,
    pub substream: *mut snd_pcm_substream,
    pub clicks: i32,
    pub current_period: i32,
    pub timer: timer_list,
    pub master_volume: i32,
    pub dma_check: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
