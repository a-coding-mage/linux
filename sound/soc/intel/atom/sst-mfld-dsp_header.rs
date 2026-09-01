/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  sst_mfld_dsp.h - Intel SST Driver for audio engine
 *
 *  Copyright (C) 2008-14 Intel Corporation
 *  Authors:	Vinod Koul <vinod.koul@linux.intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

/* Rust translation of declarations from sst-mfld-dsp.h. */

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type s32 = ::core::ffi::c_int;
pub type __u16 = u16;
pub type __u32 = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub const SST_MAX_BIN_BYTES: u32 = 1024;

pub const MAX_DBG_RW_BYTES: u32 = 80;
pub const MAX_NUM_SCATTER_BUFFERS: u32 = 8;
pub const MAX_LOOP_BACK_DWORDS: u32 = 8;
/* IPC base address and mailbox, timestamp offsets */
pub const SST_MAILBOX_SIZE: u32 = 0x0400;
pub const SST_MAILBOX_SEND: u32 = 0x0000;
pub const SST_TIME_STAMP: u32 = 0x1800;
pub const SST_TIME_STAMP_MRFLD: u32 = 0x800;
pub const SST_RESERVED_OFFSET: u32 = 0x1A00;
pub const SST_SCU_LPE_MAILBOX: u32 = 0x1000;
pub const SST_LPE_SCU_MAILBOX: u32 = 0x1400;
pub const SST_SCU_LPE_LOG_BUF: u32 = SST_SCU_LPE_MAILBOX + 16;
pub const PROCESS_MSG: u32 = 0x80;

/* Message ID's for IPC messages */
/* Bits B7: SST or IA/SC ; B6-B4: Msg Category; B3-B0: Msg Type */

/* I2L Firmware/Codec Download msgs */
pub const IPC_IA_PREP_LIB_DNLD: u32 = 0x01;
pub const IPC_IA_LIB_DNLD_CMPLT: u32 = 0x02;
pub const IPC_IA_GET_FW_VERSION: u32 = 0x04;
pub const IPC_IA_GET_FW_BUILD_INF: u32 = 0x05;
pub const IPC_IA_GET_FW_INFO: u32 = 0x06;
pub const IPC_IA_GET_FW_CTXT: u32 = 0x07;
pub const IPC_IA_SET_FW_CTXT: u32 = 0x08;
pub const IPC_IA_PREPARE_SHUTDOWN: u32 = 0x31;
/* I2L Codec Config/control msgs */
pub const IPC_PREP_D3: u32 = 0x10;
pub const IPC_IA_SET_CODEC_PARAMS: u32 = 0x10;
pub const IPC_IA_GET_CODEC_PARAMS: u32 = 0x11;
pub const IPC_IA_SET_PPP_PARAMS: u32 = 0x12;
pub const IPC_IA_GET_PPP_PARAMS: u32 = 0x13;
pub const IPC_SST_PERIOD_ELAPSED_MRFLD: u32 = 0xA;
pub const IPC_IA_ALG_PARAMS: u32 = 0x1A;
pub const IPC_IA_TUNING_PARAMS: u32 = 0x1B;
pub const IPC_IA_SET_RUNTIME_PARAMS: u32 = 0x1C;
pub const IPC_IA_SET_PARAMS: u32 = 0x1;
pub const IPC_IA_GET_PARAMS: u32 = 0x2;

pub const IPC_EFFECTS_CREATE: u32 = 0xE;
pub const IPC_EFFECTS_DESTROY: u32 = 0xF;

/* I2L Stream config/control msgs */
pub const IPC_IA_ALLOC_STREAM_MRFLD: u32 = 0x2;
pub const IPC_IA_ALLOC_STREAM: u32 = 0x20; /* Allocate a stream ID */
pub const IPC_IA_FREE_STREAM_MRFLD: u32 = 0x03;
pub const IPC_IA_FREE_STREAM: u32 = 0x21; /* Free the stream ID */
pub const IPC_IA_SET_STREAM_PARAMS: u32 = 0x22;
pub const IPC_IA_SET_STREAM_PARAMS_MRFLD: u32 = 0x12;
pub const IPC_IA_GET_STREAM_PARAMS: u32 = 0x23;
pub const IPC_IA_PAUSE_STREAM: u32 = 0x24;
pub const IPC_IA_PAUSE_STREAM_MRFLD: u32 = 0x4;
pub const IPC_IA_RESUME_STREAM: u32 = 0x25;
pub const IPC_IA_RESUME_STREAM_MRFLD: u32 = 0x5;
pub const IPC_IA_DROP_STREAM: u32 = 0x26;
pub const IPC_IA_DROP_STREAM_MRFLD: u32 = 0x07;
pub const IPC_IA_DRAIN_STREAM: u32 = 0x27; /* Short msg with str_id */
pub const IPC_IA_DRAIN_STREAM_MRFLD: u32 = 0x8;
pub const IPC_IA_CONTROL_ROUTING: u32 = 0x29;
pub const IPC_IA_VTSV_UPDATE_MODULES: u32 = 0x20;
pub const IPC_IA_VTSV_DETECTED: u32 = 0x21;

pub const IPC_IA_START_STREAM_MRFLD: u32 = 0x06;
pub const IPC_IA_START_STREAM: u32 = 0x30; /* Short msg with str_id */

pub const IPC_IA_SET_GAIN_MRFLD: u32 = 0x21;
/* Debug msgs */
pub const IPC_IA_DBG_MEM_READ: u32 = 0x40;
pub const IPC_IA_DBG_MEM_WRITE: u32 = 0x41;
pub const IPC_IA_DBG_LOOP_BACK: u32 = 0x42;
pub const IPC_IA_DBG_LOG_ENABLE: u32 = 0x45;
pub const IPC_IA_DBG_SET_PROBE_PARAMS: u32 = 0x47;

/* L2I Firmware/Codec Download msgs */
pub const IPC_IA_FW_INIT_CMPLT: u32 = 0x81;
pub const IPC_IA_FW_INIT_CMPLT_MRFLD: u32 = 0x01;
pub const IPC_IA_FW_ASYNC_ERR_MRFLD: u32 = 0x11;

/* L2I Codec Config/control msgs */
pub const IPC_SST_FRAGMENT_ELPASED: u32 = 0x90; /* Request IA more data */

pub const IPC_SST_BUF_UNDER_RUN: u32 = 0x92; /* PB Under run and stopped */
pub const IPC_SST_BUF_OVER_RUN: u32 = 0x93; /* CAP Under run and stopped */
pub const IPC_SST_DRAIN_END: u32 = 0x94; /* PB Drain complete and stopped */
pub const IPC_SST_CHNGE_SSP_PARAMS: u32 = 0x95; /* PB SSP parameters changed */
pub const IPC_SST_STREAM_PROCESS_FATAL_ERR: u32 = 0x96; /* error in processing a stream */
pub const IPC_SST_PERIOD_ELAPSED: u32 = 0x97; /* period elapsed */

pub const IPC_SST_ERROR_EVENT: u32 = 0x99; /* Buffer over run occurred */
/* L2S messages */
pub const IPC_SC_DDR_LINK_UP: u32 = 0xC0;
pub const IPC_SC_DDR_LINK_DOWN: u32 = 0xC1;
pub const IPC_SC_SET_LPECLK_REQ: u32 = 0xC2;
pub const IPC_SC_SSP_BIT_BANG: u32 = 0xC3;

/* L2I Error reporting msgs */
pub const IPC_IA_MEM_ALLOC_FAIL: u32 = 0xE0;
pub const IPC_IA_PROC_ERR: u32 = 0xE1; /* error in processing a
                                          stream can be used by playback and
                                          capture modules */

/* L2I Debug msgs */
pub const IPC_IA_PRINT_STRING: u32 = 0xF0;

/* Buffer under-run */
pub const IPC_IA_BUF_UNDER_RUN_MRFLD: u32 = 0x0B;

/* Mrfld specific defines:
 * For asynchronous messages(INIT_CMPLT, PERIOD_ELAPSED, ASYNC_ERROR)
 * received from FW, the format is:
 *  - IPC High: pvt_id is set to zero. Always short message.
 *  - msg_id is in lower 16-bits of IPC low payload.
 *  - pipe_id is in higher 16-bits of IPC low payload for period_elapsed.
 *  - error id is in higher 16-bits of IPC low payload for async errors.
 */
pub const SST_ASYNC_DRV_ID: u32 = 0;

/* Command Response or Acknowledge message to any IPC message will have
 * same message ID and stream ID information which is sent.
 * There is no specific Ack message ID. The data field is used as response
 * meaning.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ackData {
    IPC_ACK_SUCCESS = 0,
    IPC_ACK_FAILURE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ipc_ia_msg_id {
    IPC_CMD = 1,        /*!< Task Control message ID */
    IPC_SET_PARAMS = 2, /*!< Task Set param message ID */
    IPC_GET_PARAMS = 3, /*!< Task Get param message ID */
    IPC_INVALID = 0xFF, /*!<Task Get param message ID */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_codec_types {
    /*  AUDIO/MUSIC	CODEC Type Definitions */
    SST_CODEC_TYPE_UNKNOWN = 0,
    SST_CODEC_TYPE_PCM = 1, /* Pass through Audio codec */
    SST_CODEC_TYPE_MP3 = 2,
    SST_CODEC_TYPE_MP24 = 3,
    SST_CODEC_TYPE_AAC = 4,
    SST_CODEC_TYPE_AACP = 5,
    SST_CODEC_TYPE_eAACP = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stream_type {
    SST_STREAM_TYPE_NONE = 0,
    SST_STREAM_TYPE_MUSIC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sst_error_codes {
    /* Error code,response to msgId: Description */
    /* Common error codes */
    SST_SUCCESS = 0,        /* Success */
    SST_ERR_INVALID_STREAM_ID = 1,
    SST_ERR_INVALID_MSG_ID = 2,
    SST_ERR_INVALID_STREAM_OP = 3,
    SST_ERR_INVALID_PARAMS = 4,
    SST_ERR_INVALID_CODEC = 5,
    SST_ERR_INVALID_MEDIA_TYPE = 6,
    SST_ERR_STREAM_ERR = 7,

    SST_ERR_STREAM_IN_USE = 15,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipc_dsp_hdr {
    pub _bitfield_1: u16,
    pub mod_id: u16, /* Pipe_id */
    pub cmd_id: u16, /* Module ID = lpe_algo_types_t */
    pub length: u16, /* Length of the payload only */
}

impl ipc_dsp_hdr {
    #[inline]
    pub fn mod_index_id(&self) -> u16 {
        self._bitfield_1 & 0xff
    }

    #[inline]
    pub fn pipe_id(&self) -> u16 {
        (self._bitfield_1 >> 8) & 0xff
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header_high {
    pub part: ipc_header_high_part,
    pub full: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipc_header_high_part {
    pub _bitfield_1: u32,
}

impl ipc_header_high_part {
    #[inline]
    pub fn msg_id(&self) -> u32 {
        self._bitfield_1 & 0xff
    }

    #[inline]
    pub fn task_id(&self) -> u32 {
        (self._bitfield_1 >> 8) & 0xf
    }

    #[inline]
    pub fn drv_id(&self) -> u32 {
        (self._bitfield_1 >> 12) & 0xf
    }

    #[inline]
    pub fn rsvd1(&self) -> u32 {
        (self._bitfield_1 >> 16) & 0xff
    }

    #[inline]
    pub fn result(&self) -> u32 {
        (self._bitfield_1 >> 24) & 0xf
    }

    #[inline]
    pub fn res_rqd(&self) -> u32 {
        (self._bitfield_1 >> 28) & 0x1
    }

    #[inline]
    pub fn large(&self) -> u32 {
        (self._bitfield_1 >> 29) & 0x1
    }

    #[inline]
    pub fn done(&self) -> u32 {
        (self._bitfield_1 >> 30) & 0x1
    }

    #[inline]
    pub fn busy(&self) -> u32 {
        (self._bitfield_1 >> 31) & 0x1
    }
}

/* IPC header */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header_mrfld {
    pub p: ipc_header_mrfld_p,
    pub full: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipc_header_mrfld_p {
    pub header_low_payload: u32,
    pub header_high: ipc_header_high,
}

/* CAUTION NOTE: All IPC message body must be multiple of 32 bits.*/

/* IPC Header */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header {
    pub part: ipc_header_part,
    pub full: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipc_header_part {
    pub _bitfield_1: u32,
}

impl ipc_header_part {
    #[inline]
    pub fn msg_id(&self) -> u32 {
        self._bitfield_1 & 0xff
    }

    #[inline]
    pub fn str_id(&self) -> u32 {
        (self._bitfield_1 >> 8) & 0x1f
    }

    #[inline]
    pub fn large(&self) -> u32 {
        (self._bitfield_1 >> 13) & 0x1
    }

    #[inline]
    pub fn reserved(&self) -> u32 {
        (self._bitfield_1 >> 14) & 0x3
    }

    #[inline]
    pub fn data(&self) -> u32 {
        (self._bitfield_1 >> 16) & 0x3fff
    }

    #[inline]
    pub fn done(&self) -> u32 {
        (self._bitfield_1 >> 30) & 0x1
    }

    #[inline]
    pub fn busy(&self) -> u32 {
        (self._bitfield_1 >> 31) & 0x1
    }
}

/* Firmware build info */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct sst_fw_build_info {
    pub date: [::core::ffi::c_uchar; 16], /* Firmware build date */
    pub time: [::core::ffi::c_uchar; 16], /* Firmware build time */
}

/* Firmware Version info */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_fw_version {
    pub build: u8, /* build number*/
    pub minor: u8, /* minor number*/
    pub major: u8, /* major number*/
    pub type_: u8, /* build type */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ipc_header_fw_init {
    pub fw_version: snd_sst_fw_version, /* Firmware version details */
    pub build_info: sst_fw_build_info,
    pub result: u16,     /* Fw init result */
    pub module_id: u8,   /* Module ID in case of error */
    pub debug_info: u8,  /* Debug info from Module ID in case of fail */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_tstamp {
    pub ring_buffer_counter: u64, /* PB/CP: Bytes copied from/to DDR. */
    pub hardware_counter: u64,    /* PB/CP: Bytes DMAed to/from SSP. */
    pub frames_decoded: u64,
    pub bytes_decoded: u64,
    pub bytes_copied: u64,
    pub sampling_frequency: u32,
    pub channel_peak: [u32; 8],
}

/* Stream type params structure for Alloc stream */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_str_type {
    pub codec_type: u8,    /* Codec type */
    pub str_type: u8,      /* 1 = voice 2 = music */
    pub operation: u8,     /* Playback or Capture */
    pub protected_str: u8, /* 0=Non DRM, 1=DRM */
    pub time_slots: u8,
    pub reserved: u8, /* Reserved */
    pub result: u16,  /* Result used for acknowledgment */
}

/* Library info structure */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct module_info {
    pub lib_version: u32,
    pub lib_type: u32, /*TBD- KLOCKWORK u8 lib_type;*/
    pub media_type: u32,
    pub lib_name: [u8; 12],
    pub lib_caps: u32,
    pub b_date: [::core::ffi::c_uchar; 16], /* Lib build date */
    pub b_time: [::core::ffi::c_uchar; 16], /* Lib build time */
}

/* Library slot info */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct lib_slot_info {
    pub slot_num: u8, /* 1 or 2 */
    pub reserved1: u8,
    pub reserved2: u16,
    pub iram_size: u32,   /* slot size in IRAM */
    pub dram_size: u32,   /* slot size in DRAM */
    pub iram_offset: u32, /* starting offset of slot in IRAM */
    pub dram_offset: u32, /* starting offset of slot in DRAM */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_ppp_mixer_params {
    pub type_: __u32,              /*Type of the parameter */
    pub size: __u32,
    pub input_stream_bitmap: __u32, /*Input stream Bit Map*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_lib_download {
    pub lib_info: module_info,     /* library info type, capabilities etc */
    pub slot_info: lib_slot_info,  /* slot info to be downloaded */
    pub mod_entry_pt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_lib_download_info {
    pub dload_lib: snd_sst_lib_download,
    pub result: u16,   /* Result used for acknowledgment */
    pub pvt_id: u8,    /* Private ID */
    pub reserved: u8,  /* for alignment */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_pcm_params {
    pub num_chan: u8,         /* 1=Mono, 2=Stereo */
    pub pcm_wd_sz: u8,        /* 16/24 - bit*/
    pub use_offload_path: u8, /* 0-PCM using period elpased & ALSA interfaces
                                1-PCM stream via compressed interface  */
    pub reserved2: u8,
    pub sfreq: u32,    /* Sampling rate in Hz */
    pub channel_map: [u8; 8],
}

/* MP3 Music Parameters Message */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_mp3_params {
    pub num_chan: u8,  /* 1=Mono, 2=Stereo	*/
    pub pcm_wd_sz: u8, /* 16/24 - bit*/
    pub crc_check: u8, /* crc_check - disable (0) or enable (1) */
    pub reserved1: u8, /* unused*/
    pub reserved2: u16, /* Unused */
}

pub const AAC_BIT_STREAM_ADTS: u32 = 0;
pub const AAC_BIT_STREAM_ADIF: u32 = 1;
pub const AAC_BIT_STREAM_RAW: u32 = 2;

/* AAC Music Parameters Message */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_aac_params {
    pub num_chan: u8,       /* 1=Mono, 2=Stereo*/
    pub pcm_wd_sz: u8,      /* 16/24 - bit*/
    pub bdownsample: u8,    /*SBR downsampling 0 - disable 1 -enabled AAC+ only */
    pub bs_format: u8,      /* input bit stream format adts=0, adif=1, raw=2 */
    pub reser2: u16,
    pub externalsr: u32,    /*sampling rate of basic AAC raw bit stream*/
    pub sbr_signalling: u8, /*disable/enable/set automode the SBR tool.AAC+*/
    pub reser1: u8,
    pub reser3: u16,
}

/* WMA Music Parameters Message */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_wma_params {
    pub num_chan: u8,     /* 1=Mono, 2=Stereo */
    pub pcm_wd_sz: u8,    /* 16/24 - bit*/
    pub reserved1: u16,
    pub brate: u32,       /* Use the hard coded value. */
    pub sfreq: u32,       /* Sampling freq eg. 8000, 441000, 48000 */
    pub channel_mask: u32, /* Channel Mask */
    pub format_tag: u16,  /* Format Tag */
    pub block_align: u16, /* packet size */
    pub wma_encode_opt: u16, /* Encoder option */
    pub op_align: u8,    /* op align 0- 16 bit, 1- MSB, 2 LSB */
    pub reserved: u8,    /* reserved */
}

/* Codec params structure */
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_sst_codec_params {
    pub pcm_params: snd_pcm_params,
    pub mp3_params: snd_mp3_params,
    pub aac_params: snd_aac_params,
    pub wma_params: snd_wma_params,
}

/* Address and size info of a frame buffer */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_address_info {
    pub addr: u32, /* Address at IA */
    pub size: u32, /* Size of the buffer */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_params_ext {
    pub sg_count: __u16,
    pub reserved: __u16,
    pub frag_size: __u32, /*Number of samples after which period elapsed
                            message is sent valid only if path  = 0*/
    pub ring_buf_info: [sst_address_info; 8],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_stream_params {
    pub uc: snd_sst_codec_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_params {
    pub result: u32,
    pub stream_id: u32,
    pub codec: u8,
    pub ops: u8,
    pub stream_type: u8,
    pub device_type: u8,
    pub task: u8,
    pub sparams: snd_sst_stream_params,
    pub aparams: snd_sst_alloc_params_ext,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_mrfld {
    pub codec_type: u16,
    pub operation: u8,
    pub sg_count: u8,
    pub ring_buf_info: [sst_address_info; 8],
    pub frag_size: u32,
    pub ts: u32,
    pub codec_params: snd_sst_stream_params,
}

/* Alloc stream params structure */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_params {
    pub str_type: snd_sst_str_type,
    pub stream_params: snd_sst_stream_params,
    pub alloc_params: snd_sst_alloc_params_ext,
}

/* Alloc stream response message */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_response {
    pub str_type: snd_sst_str_type,       /* Stream type for allocation */
    pub lib_dnld: snd_sst_lib_download,   /* Valid only for codec dnld */
}

/* Drop response */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_drop_response {
    pub result: u32,
    pub bytes: u32,
}

#[repr(C)]
pub struct snd_sst_async_msg {
    pub msg_id: u32, /* Async msg id */
    pub payload: [u32; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_async_err_msg {
    pub fw_resp: u32,  /* Firmware Result */
    pub lib_resp: u32, /*Library result */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_vol {
    pub stream_id: u32,
    pub volume: s32,
    pub ramp_duration: u32,
    pub ramp_type: u32, /* Ramp type, default=0 */
}

/* Gain library parameters for mrfld
 * based on DSP command spec v0.82
 */
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_gain_v2 {
    pub gain_cell_num: u16,  /* num of gain cells to modify*/
    pub cell_nbr_idx: u8,    /* instance index*/
    pub cell_path_idx: u8,   /* pipe-id */
    pub module_id: u16,      /*module id */
    pub left_cell_gain: u16, /* left gain value in dB*/
    pub right_cell_gain: u16, /* right gain value in dB*/
    pub gain_time_const: u16, /* gain time constant*/
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_mute {
    pub stream_id: u32,
    pub mute: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_runtime_params {
    pub type_: u8,
    pub str_id: u8,
    pub size: u8,
    pub rsvd: u8,
    pub addr: *mut ::core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stream_param_type {
    SST_SET_TIME_SLOT = 0,
    SST_SET_CHANNEL_INFO = 1,
    OTHERS = 2, /*reserved for future params*/
}

/* CSV Voice call routing structure */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_control_routing {
    pub control: u8,      /* 0=start, 1=Stop */
    pub reserved: [u8; 3], /* Reserved- for 32 bit alignment */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_post {
    pub node: list_head,
    pub header: ipc_header, /* driver specific */
    pub is_large: bool,
    pub is_process_reply: bool,
    pub mrfld_header: ipc_header_mrfld,
    pub mailbox_data: *mut ::core::ffi::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_ctxt_params {
    pub address: u32, /* Physical Address in DDR where the context is stored */
    pub size: u32,    /* size of the context */
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_lpe_log_params {
    pub dbg_type: u8,
    pub module_id: u8,
    pub log_level: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_sst_bytes_type {
    SND_SST_BYTES_SET = 0x1,
    SND_SST_BYTES_GET = 0x2,
}

#[repr(C)]
pub struct snd_sst_bytes_v2 {
    pub type_: u8,
    pub ipc_msg: u8,
    pub block: u8,
    pub task_id: u8,
    pub pipe_id: u8,
    pub rsvd: u8,
    pub len: u16,
    pub bytes: [::core::ffi::c_char; 0],
}

pub const MAX_VTSV_FILES: usize = 2;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct snd_sst_vtsv_info {
    pub vfiles: [sst_address_info; MAX_VTSV_FILES],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
