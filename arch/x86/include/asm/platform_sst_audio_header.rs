/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * platform_sst_audio.h:  sst audio platform data header file
 *
 * Copyright (C) 2012-14 Intel Corporation
 * Author: Jeeja KP <jeeja.kp@intel.com>
 * 	Omair Mohammed Abdullah <omair.m.abdullah@intel.com>
 *	Vinod Koul ,vinod.koul@intel.com>
 */

pub const MAX_NUM_STREAMS_MRFLD: i32 = 25;
pub const MAX_NUM_STREAMS: i32 = MAX_NUM_STREAMS_MRFLD;

#[repr(i32)]
pub enum sst_audio_task_id_mrfld {
    SST_TASK_ID_NONE = 0,
    SST_TASK_ID_SBA = 1,
    SST_TASK_ID_MEDIA = 3,
    SST_TASK_ID_MAX = SST_TASK_ID_MEDIA,
}

/* Device IDs for Merrifield are Pipe IDs,
 * ref: DSP spec v0.75 */
#[repr(i32)]
pub enum sst_audio_device_id_mrfld {
    /* Output pipeline IDs */
    PIPE_ID_OUT_START = 0x0,
    PIPE_CODEC_OUT0 = 0x2,
    PIPE_CODEC_OUT1 = 0x3,
    PIPE_SPROT_LOOP_OUT = 0x4,
    PIPE_MEDIA_LOOP1_OUT = 0x5,
    PIPE_MEDIA_LOOP2_OUT = 0x6,
    PIPE_VOIP_OUT = 0xC,
    PIPE_PCM0_OUT = 0xD,
    PIPE_PCM1_OUT = 0xE,
    PIPE_PCM2_OUT = 0xF,
    PIPE_MEDIA0_OUT = 0x12,
    PIPE_MEDIA1_OUT = 0x13,
    /* Input Pipeline IDs */
    PIPE_ID_IN_START = 0x80,
    PIPE_CODEC_IN0 = 0x82,
    PIPE_CODEC_IN1 = 0x83,
    PIPE_SPROT_LOOP_IN = 0x84,
    PIPE_MEDIA_LOOP1_IN = 0x85,
    PIPE_MEDIA_LOOP2_IN = 0x86,
    PIPE_VOIP_IN = 0x8C,
    PIPE_PCM0_IN = 0x8D,
    PIPE_PCM1_IN = 0x8E,
    PIPE_MEDIA0_IN = 0x8F,
    PIPE_MEDIA1_IN = 0x90,
    PIPE_MEDIA2_IN = 0x91,
    PIPE_MEDIA3_IN = 0x9C,
    PIPE_RSVD = 0xFF,
}

/* The stream map for each platform consists of an array of the below
 * stream map structure.
 */
#[repr(C)]
pub struct sst_dev_stream_map {
    pub dev_num: u8, /* device id */
    pub subdev_num: u8, /* substream */
    pub direction: u8,
    pub device_id: u8, /* fw id */
    pub task_id: u8, /* fw task */
    pub status: u8,
}

#[repr(C)]
pub struct sst_platform_data {
    /* Intel software platform id*/
    pub pdev_strm_map: *mut sst_dev_stream_map,
    pub strm_map_size: u32,
}

#[repr(C)]
pub struct sst_info {
    pub iram_start: u32,
    pub iram_end: u32,
    pub iram_use: bool,
    pub dram_start: u32,
    pub dram_end: u32,
    pub dram_use: bool,
    pub imr_start: u32,
    pub imr_end: u32,
    pub imr_use: bool,
    pub mailbox_start: u32,
    pub use_elf: bool,
    pub lpe_viewpt_rqd: bool,
    pub max_streams: u32,
    pub dma_max_len: u32,
    pub num_probes: u8,
}

#[repr(C)]
pub struct sst_lib_dnld_info {
    pub mod_base: u32,
    pub mod_end: u32,
    pub mod_table_offset: u32,
    pub mod_table_size: u32,
    pub mod_ddr_dnld: bool,
}

#[repr(C)]
pub struct sst_res_info {
    pub shim_offset: u32,
    pub shim_size: u32,
    pub shim_phy_addr: u32,
    pub ssp0_offset: u32,
    pub ssp0_size: u32,
    pub dma0_offset: u32,
    pub dma0_size: u32,
    pub dma1_offset: u32,
    pub dma1_size: u32,
    pub iram_offset: u32,
    pub iram_size: u32,
    pub dram_offset: u32,
    pub dram_size: u32,
    pub mbox_offset: u32,
    pub mbox_size: u32,
    pub acpi_lpe_res_index: u32,
    pub acpi_ddr_index: u32,
    pub acpi_ipc_irq_index: u32,
}

#[repr(C)]
pub struct sst_ipc_info {
    pub ipc_offset: i32,
    pub mbox_recv_off: u32,
}

#[repr(C)]
pub struct sst_platform_info {
    pub probe_data: *const sst_info,
    pub ipc_info: *const sst_ipc_info,
    pub res_info: *const sst_res_info,
    pub lib_info: *const sst_lib_dnld_info,
    pub platform: *const core::ffi::c_char,
    pub streams_lost_on_suspend: bool,
}

extern "C" {
    pub fn add_sst_platform_device() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
