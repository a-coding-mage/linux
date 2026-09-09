/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

/* Dependency: <sound/sof/header.h> */

/*
 * Stream configuration.
 */

pub const SOF_IPC_MAX_CHANNELS: usize = 8;

/* common sample rates for use in masks */
pub const SOF_RATE_8000: u32 = 1 << 0; /**< 8000Hz  */
pub const SOF_RATE_11025: u32 = 1 << 1; /**< 11025Hz */
pub const SOF_RATE_12000: u32 = 1 << 2; /**< 12000Hz */
pub const SOF_RATE_16000: u32 = 1 << 3; /**< 16000Hz */
pub const SOF_RATE_22050: u32 = 1 << 4; /**< 22050Hz */
pub const SOF_RATE_24000: u32 = 1 << 5; /**< 24000Hz */
pub const SOF_RATE_32000: u32 = 1 << 6; /**< 32000Hz */
pub const SOF_RATE_44100: u32 = 1 << 7; /**< 44100Hz */
pub const SOF_RATE_48000: u32 = 1 << 8; /**< 48000Hz */
pub const SOF_RATE_64000: u32 = 1 << 9; /**< 64000Hz */
pub const SOF_RATE_88200: u32 = 1 << 10; /**< 88200Hz */
pub const SOF_RATE_96000: u32 = 1 << 11; /**< 96000Hz */
pub const SOF_RATE_176400: u32 = 1 << 12; /**< 176400Hz */
pub const SOF_RATE_192000: u32 = 1 << 13; /**< 192000Hz */

/* continuous and non-standard rates for flexibility */
pub const SOF_RATE_CONTINUOUS: u32 = 1 << 30; /**< range */
pub const SOF_RATE_KNOT: u32 = 1 << 31; /**< non-continuous */

/* generic PCM flags for runtime settings */
pub const SOF_PCM_FLAG_XRUN_STOP: u32 = 1 << 0; /**< Stop on any XRUN */

/* stream PCM frame format */
#[repr(C)]
pub enum sof_ipc_frame {
    SOF_IPC_FRAME_S16_LE = 0,
    SOF_IPC_FRAME_S24_4LE,
    SOF_IPC_FRAME_S32_LE,
    SOF_IPC_FRAME_FLOAT,
    /* other formats here */
}

/* stream buffer format */
#[repr(C)]
pub enum sof_ipc_buffer_format {
    SOF_IPC_BUFFER_INTERLEAVED,
    SOF_IPC_BUFFER_NONINTERLEAVED,
    /* other formats here */
}

/* stream direction */
#[repr(C)]
pub enum sof_ipc_stream_direction {
    SOF_IPC_STREAM_PLAYBACK = 0,
    SOF_IPC_STREAM_CAPTURE,
}

/* stream ring info */
#[repr(C, packed)]
pub struct sof_ipc_host_buffer {
    pub hdr: sof_ipc_hdr,
    pub phy_addr: u32,
    pub pages: u32,
    pub size: u32,
    pub reserved: [u32; 3],
}

#[repr(C, packed)]
pub struct sof_ipc_stream_params {
    pub hdr: sof_ipc_hdr,
    pub buffer: sof_ipc_host_buffer,
    pub direction: u32, /* enum sof_ipc_stream_direction */
    pub frame_fmt: u32, /* enum sof_ipc_frame */
    pub buffer_fmt: u32, /* enum sof_ipc_buffer_format */
    pub rate: u32,
    pub stream_tag: u16,
    pub channels: u16,
    pub sample_valid_bytes: u16,
    pub sample_container_bytes: u16,
    pub host_period_bytes: u32,
    pub no_stream_position: u16, /**< 1 means don't send stream position */
    pub cont_update_posn: u8, /**< 1 means continuous update stream position */
    pub reserved0: u8,
    pub ext_data_length: i16, /**< 0, means no extended data */
    pub reserved: [u8; 2],
    pub chmap: [u16; SOF_IPC_MAX_CHANNELS], /**< channel map - SOF_CHMAP_ */
    pub ext_data: [u8; 0], /**< extended data */
}

/* PCM params info - SOF_IPC_STREAM_PCM_PARAMS */
#[repr(C, packed)]
pub struct sof_ipc_pcm_params {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub flags: u32, /* generic PCM flags - SOF_PCM_FLAG_ */
    pub reserved: [u32; 2],
    pub params: sof_ipc_stream_params,
}

/* PCM params info reply - SOF_IPC_STREAM_PCM_PARAMS_REPLY */
#[repr(C, packed)]
pub struct sof_ipc_pcm_params_reply {
    pub rhdr: sof_ipc_reply,
    pub comp_id: u32,
    pub posn_offset: u32,
}

/* free stream - SOF_IPC_STREAM_PCM_PARAMS */
#[repr(C, packed)]
pub struct sof_ipc_stream {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
}

/* flags indicating which time stamps are in sync with each other */
pub const SOF_TIME_HOST_SYNC: u32 = 1 << 0;
pub const SOF_TIME_DAI_SYNC: u32 = 1 << 1;
pub const SOF_TIME_WALL_SYNC: u32 = 1 << 2;
pub const SOF_TIME_STAMP_SYNC: u32 = 1 << 3;

/* flags indicating which time stamps are valid */
pub const SOF_TIME_HOST_VALID: u32 = 1 << 8;
pub const SOF_TIME_DAI_VALID: u32 = 1 << 9;
pub const SOF_TIME_WALL_VALID: u32 = 1 << 10;
pub const SOF_TIME_STAMP_VALID: u32 = 1 << 11;

/* flags indicating time stamps are 64bit else 3use low 32bit */
pub const SOF_TIME_HOST_64: u32 = 1 << 16;
pub const SOF_TIME_DAI_64: u32 = 1 << 17;
pub const SOF_TIME_WALL_64: u32 = 1 << 18;
pub const SOF_TIME_STAMP_64: u32 = 1 << 19;

#[repr(C, packed)]
pub struct sof_ipc_stream_posn {
    pub rhdr: sof_ipc_reply,
    pub comp_id: u32, /**< host component ID */
    pub flags: u32, /**< SOF_TIME_ */
    pub wallclock_hz: u32, /**< frequency of wallclock in Hz */
    pub timestamp_ns: u32, /**< resolution of timestamp in ns */
    pub host_posn: u64, /**< host DMA position in bytes */
    pub dai_posn: u64, /**< DAI DMA position in bytes */
    pub comp_posn: u64, /**< comp position in bytes */
    pub wallclock: u64, /**< audio wall clock */
    pub timestamp: u64, /**< system time stamp */
    pub xrun_comp_id: u32, /**< comp ID of XRUN component */
    pub xrun_size: i32, /**< XRUN size in bytes */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
