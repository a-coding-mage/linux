/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  compress_offload.h - compress offload header definations
 *
 *  Copyright (C) 2011 Intel Corporation
 *  Authors: Vinod Koul <vinod.koul@linux.intel.com>
 *           Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
 */

pub const SNDRV_COMPRESS_VERSION: u32 = SNDRV_PROTOCOL_VERSION(0, 4, 1);

/// compressed buffer
#[repr(C, packed(4))]
pub struct snd_compressed_buffer {
    pub fragment_size: __u32,
    pub fragments: __u32,
}

/// compressed stream params
#[repr(C, packed(4))]
pub struct snd_compr_params {
    pub buffer: snd_compressed_buffer,
    pub codec: snd_codec,
    pub no_wake_mode: __u8,
}

/// timestamp descriptor
#[repr(C, packed(4))]
pub struct snd_compr_tstamp {
    pub byte_offset: __u32,
    pub copied_total: __u32,
    pub pcm_frames: __u32,
    pub pcm_io_frames: __u32,
    pub sampling_rate: __u32,
}

/// timestamp descriptor with fields in 64 bit
#[repr(C, packed(4))]
pub struct snd_compr_tstamp64 {
    pub byte_offset: __u32,
    pub copied_total: __u64,
    pub pcm_frames: __u64,
    pub pcm_io_frames: __u64,
    pub sampling_rate: __u32,
}

/// avail descriptor
#[repr(C, packed(4))]
pub struct snd_compr_avail {
    pub avail: __u64,
    pub tstamp: snd_compr_tstamp,
}

/// avail descriptor with tstamp in 64 bit format
#[repr(C, packed(4))]
pub struct snd_compr_avail64 {
    pub avail: __u64,
    pub tstamp: snd_compr_tstamp64,
}

#[repr(C)]
pub enum snd_compr_direction {
    SND_COMPRESS_PLAYBACK = 0,
    SND_COMPRESS_CAPTURE,
    SND_COMPRESS_ACCEL,
}

/// caps descriptor
#[repr(C, packed(4))]
pub struct snd_compr_caps {
    pub num_codecs: __u32,
    pub direction: __u32,
    pub min_fragment_size: __u32,
    pub max_fragment_size: __u32,
    pub min_fragments: __u32,
    pub max_fragments: __u32,
    pub codecs: [__u32; MAX_NUM_CODECS],
    pub reserved: [__u32; 11],
}

/// query capability of codec
#[repr(C, packed(4))]
pub struct snd_compr_codec_caps {
    pub codec: __u32,
    pub num_descriptors: __u32,
    pub descriptor: [snd_codec_desc; MAX_NUM_CODEC_DESCRIPTORS],
}

/// encoder metadata key
#[repr(C)]
pub enum sndrv_compress_encoder {
    SNDRV_COMPRESS_ENCODER_PADDING = 1,
    SNDRV_COMPRESS_ENCODER_DELAY = 2,
}

/// compressed stream metadata
#[repr(C, packed(4))]
pub struct snd_compr_metadata {
    pub key: __u32,
    pub value: [__u32; 8],
}

/* flags for struct snd_compr_task */
pub const SND_COMPRESS_TFLG_NEW_STREAM: __u32 = 1 << 0; /* mark for the new stream data */

/// task primitive for non-realtime operation
#[repr(C, packed(4))]
pub struct snd_compr_task {
    pub seqno: __u64,
    pub origin_seqno: __u64,
    pub input_fd: i32,
    pub output_fd: i32,
    pub input_size: __u64,
    pub flags: __u32,
    pub reserved: [__u8; 16],
}

/// task state
#[repr(C)]
pub enum snd_compr_state {
    SND_COMPRESS_TASK_STATE_IDLE = 0,
    SND_COMPRESS_TASK_STATE_ACTIVE,
    SND_COMPRESS_TASK_STATE_FINISHED,
}

/// task status
#[repr(C, packed(4))]
pub struct snd_compr_task_status {
    pub seqno: __u64,
    pub input_size: __u64,
    pub output_size: __u64,
    pub output_flags: __u32,
    pub state: __u8,
    pub reserved: [__u8; 15],
}

/*
 * compress path ioctl definitions
 * SNDRV_COMPRESS_GET_CAPS: Query capability of DSP
 * SNDRV_COMPRESS_GET_CODEC_CAPS: Query capability of a codec
 * SNDRV_COMPRESS_SET_PARAMS: Set codec and stream parameters
 * Note: only codec params can be changed runtime and stream params cant be
 * SNDRV_COMPRESS_GET_PARAMS: Query codec params
 * SNDRV_COMPRESS_TSTAMP: get the current timestamp value
 * SNDRV_COMPRESS_TSTAMP64: get the current timestamp value in 64 bit format
 * SNDRV_COMPRESS_AVAIL: get the current buffer avail value.
 * This also queries the tstamp properties
 * SNDRV_COMPRESS_PAUSE: Pause the running stream
 * SNDRV_COMPRESS_RESUME: resume a paused stream
 * SNDRV_COMPRESS_START: Start a stream
 * SNDRV_COMPRESS_STOP: stop a running stream, discarding ring buffer content
 * and the buffers currently with DSP
 * SNDRV_COMPRESS_DRAIN: Play till end of buffers and stop after that
 * SNDRV_COMPRESS_IOCTL_VERSION: Query the API version
 */
pub const SNDRV_COMPRESS_IOCTL_VERSION: _ = _IOR('C', 0x00, i32);
pub const SNDRV_COMPRESS_GET_CAPS: _ = _IOWR('C', 0x10, snd_compr_caps);
pub const SNDRV_COMPRESS_GET_CODEC_CAPS: _ = _IOWR('C', 0x11, snd_compr_codec_caps);
pub const SNDRV_COMPRESS_SET_PARAMS: _ = _IOW('C', 0x12, snd_compr_params);
pub const SNDRV_COMPRESS_GET_PARAMS: _ = _IOR('C', 0x13, snd_codec);
pub const SNDRV_COMPRESS_SET_METADATA: _ = _IOW('C', 0x14, snd_compr_metadata);
pub const SNDRV_COMPRESS_GET_METADATA: _ = _IOWR('C', 0x15, snd_compr_metadata);
pub const SNDRV_COMPRESS_TSTAMP: _ = _IOR('C', 0x20, snd_compr_tstamp);
pub const SNDRV_COMPRESS_AVAIL: _ = _IOR('C', 0x21, snd_compr_avail);
pub const SNDRV_COMPRESS_TSTAMP64: _ = _IOR('C', 0x22, snd_compr_tstamp64);
pub const SNDRV_COMPRESS_AVAIL64: _ = _IOR('C', 0x23, snd_compr_avail64);
pub const SNDRV_COMPRESS_PAUSE: _ = _IO('C', 0x30);
pub const SNDRV_COMPRESS_RESUME: _ = _IO('C', 0x31);
pub const SNDRV_COMPRESS_START: _ = _IO('C', 0x32);
pub const SNDRV_COMPRESS_STOP: _ = _IO('C', 0x33);
pub const SNDRV_COMPRESS_DRAIN: _ = _IO('C', 0x34);
pub const SNDRV_COMPRESS_NEXT_TRACK: _ = _IO('C', 0x35);
pub const SNDRV_COMPRESS_PARTIAL_DRAIN: _ = _IO('C', 0x36);

pub const SNDRV_COMPRESS_TASK_CREATE: _ = _IOWR('C', 0x60, snd_compr_task);
pub const SNDRV_COMPRESS_TASK_FREE: _ = _IOW('C', 0x61, __u64);
pub const SNDRV_COMPRESS_TASK_START: _ = _IOWR('C', 0x62, snd_compr_task);
pub const SNDRV_COMPRESS_TASK_STOP: _ = _IOW('C', 0x63, __u64);
pub const SNDRV_COMPRESS_TASK_STATUS: _ = _IOWR('C', 0x68, snd_compr_task_status);

/*
 * TODO
 * 1. add mmap support
 */
pub const SND_COMPR_TRIGGER_DRAIN: i32 = 7; /*FIXME move this to pcm.h */
pub const SND_COMPR_TRIGGER_NEXT_TRACK: i32 = 8;
pub const SND_COMPR_TRIGGER_PARTIAL_DRAIN: i32 = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
