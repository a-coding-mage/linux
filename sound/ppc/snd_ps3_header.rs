/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Audio support for PS3
 * Copyright (C) 2007 Sony Computer Entertainment Inc.
 * All rights reserved.
 * Copyright 2006, 2007 Sony Corporation
 */

/* C header dependency: <linux/irqreturn.h> */

pub const SND_PS3_DRIVER_NAME: &str = "snd_ps3";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ps3_out_channel {
    SND_PS3_OUT_SPDIF_0,
    SND_PS3_OUT_SPDIF_1,
    SND_PS3_OUT_SERIAL_0,
    SND_PS3_OUT_DEVS,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ps3_dma_filltype {
    SND_PS3_DMA_FILLTYPE_FIRSTFILL,
    SND_PS3_DMA_FILLTYPE_RUNNING,
    SND_PS3_DMA_FILLTYPE_SILENT_FIRSTFILL,
    SND_PS3_DMA_FILLTYPE_SILENT_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ps3_ch {
    SND_PS3_CH_L = 0,
    SND_PS3_CH_R = 1,
    SND_PS3_CH_MAX = 2,
}

#[repr(C)]
pub struct snd_ps3_avsetting_info {
    pub avs_audio_ch: u32,     /* fixed */
    pub avs_audio_rate: u32,
    pub avs_audio_width: u32,
    pub avs_audio_format: u32, /* fixed */
    pub avs_audio_source: u32, /* fixed */
    pub avs_cs_info: [u8; 8],
}

/*
 * PS3 audio 'card' instance
 * there should be only ONE hardware.
 */
#[repr(C)]
pub struct snd_ps3_card_info {
    pub ps3_dev: *mut ps3_system_bus_device,
    pub card: *mut snd_card,

    pub pcm: *mut snd_pcm,
    pub substream: *mut snd_pcm_substream,

    /* hvc info */
    pub audio_lpar_addr: u64,
    pub audio_lpar_size: u64,

    /* registers */
    /* C type: void __iomem * */
    pub mapped_mmio_vaddr: *mut core::ffi::c_void,

    /* irq */
    pub audio_irq_outlet: u64,
    pub irq_no: core::ffi::c_uint,

    /* remember avsetting */
    pub avs: snd_ps3_avsetting_info,

    /* dma buffer management */
    pub dma_lock: spinlock_t,
    /* dma_lock start */
    pub dma_start_vaddr: [*mut core::ffi::c_void; 2], /* 0 for L, 1 for R */
    pub dma_start_bus_addr: [dma_addr_t; 2],
    pub dma_buffer_size: usize,
    pub dma_last_transfer_vaddr: [*mut core::ffi::c_void; 2],
    pub dma_next_transfer_vaddr: [*mut core::ffi::c_void; 2],
    pub silent: core::ffi::c_int,
    /* dma_lock end */

    pub running: core::ffi::c_int,

    /* null buffer */
    pub null_buffer_start_vaddr: *mut core::ffi::c_void,
    pub null_buffer_start_dma_addr: dma_addr_t,

    /* start delay */
    pub start_delay: core::ffi::c_uint,
}

/* PS3 audio DMAC block size in bytes */
pub const PS3_AUDIO_DMAC_BLOCK_SIZE: usize = 128;
/* one stage (stereo)  of audio FIFO in bytes */
pub const PS3_AUDIO_FIFO_STAGE_SIZE: usize = 256;
/* how many stages the fifo have */
pub const PS3_AUDIO_FIFO_STAGE_COUNT: usize = 8;
/* fifo size 128 bytes * 8 stages * stereo (2ch) */
pub const PS3_AUDIO_FIFO_SIZE: usize =
    PS3_AUDIO_FIFO_STAGE_SIZE * PS3_AUDIO_FIFO_STAGE_COUNT;

/* PS3 audio DMAC max block count in one dma shot = 128 (0x80) blocks*/
pub const PS3_AUDIO_DMAC_MAX_BLOCKS: usize = (PS3_AUDIO_DMASIZE_BLOCKS_MASK + 1) as usize;

pub const PS3_AUDIO_NORMAL_DMA_START_CH: usize = 0;
pub const PS3_AUDIO_NORMAL_DMA_COUNT: usize = 8;
pub const PS3_AUDIO_NULL_DMA_START_CH: usize =
    PS3_AUDIO_NORMAL_DMA_START_CH + PS3_AUDIO_NORMAL_DMA_COUNT;
pub const PS3_AUDIO_NULL_DMA_COUNT: usize = 2;

pub const SND_PS3_MAX_VOL: usize = 0x0F;
pub const SND_PS3_MIN_VOL: usize = 0x00;
pub const SND_PS3_MIN_ATT: usize = SND_PS3_MIN_VOL;
pub const SND_PS3_MAX_ATT: usize = SND_PS3_MAX_VOL;

pub const SND_PS3_PCM_PREALLOC_SIZE: usize =
    PS3_AUDIO_DMAC_BLOCK_SIZE * PS3_AUDIO_DMAC_MAX_BLOCKS * 4;

pub const SND_PS3_DMA_REGION_SIZE: usize =
    SND_PS3_PCM_PREALLOC_SIZE + PAGE_SIZE;

pub const PS3_AUDIO_IOID: core::ffi::c_ulong = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
