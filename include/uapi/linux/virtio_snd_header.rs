/* SPDX-License-Identifier: BSD-3-Clause */
/*
 * Copyright (C) 2021 OpenSynergy GmbH
 */

/* FEATURE BITS */
pub const VIRTIO_SND_F_CTLS: u32 = 0;

/* CONFIGURATION SPACE */
#[repr(C)]
pub struct virtio_snd_config {
    pub jacks: u32,
    pub streams: u32,
    pub chmaps: u32,
    pub controls: u32,
}

pub const VIRTIO_SND_VQ_CONTROL: u32 = 0;
pub const VIRTIO_SND_VQ_EVENT: u32 = 1;
pub const VIRTIO_SND_VQ_TX: u32 = 2;
pub const VIRTIO_SND_VQ_RX: u32 = 3;
pub const VIRTIO_SND_VQ_MAX: u32 = 4;

/* COMMON DEFINITIONS */
pub const VIRTIO_SND_D_OUTPUT: u32 = 0;
pub const VIRTIO_SND_D_INPUT: u32 = 1;

pub const VIRTIO_SND_R_JACK_INFO: u32 = 1;
pub const VIRTIO_SND_R_JACK_REMAP: u32 = 2;
pub const VIRTIO_SND_R_PCM_INFO: u32 = 0x0100;
pub const VIRTIO_SND_R_PCM_SET_PARAMS: u32 = 0x0101;
pub const VIRTIO_SND_R_PCM_PREPARE: u32 = 0x0102;
pub const VIRTIO_SND_R_PCM_RELEASE: u32 = 0x0103;
pub const VIRTIO_SND_R_PCM_START: u32 = 0x0104;
pub const VIRTIO_SND_R_PCM_STOP: u32 = 0x0105;
pub const VIRTIO_SND_R_CHMAP_INFO: u32 = 0x0200;
pub const VIRTIO_SND_R_CTL_INFO: u32 = 0x0300;
pub const VIRTIO_SND_R_CTL_ENUM_ITEMS: u32 = 0x0301;
pub const VIRTIO_SND_R_CTL_READ: u32 = 0x0302;
pub const VIRTIO_SND_R_CTL_WRITE: u32 = 0x0303;
pub const VIRTIO_SND_R_CTL_TLV_READ: u32 = 0x0304;
pub const VIRTIO_SND_R_CTL_TLV_WRITE: u32 = 0x0305;
pub const VIRTIO_SND_R_CTL_TLV_COMMAND: u32 = 0x0306;
pub const VIRTIO_SND_EVT_JACK_CONNECTED: u32 = 0x1000;
pub const VIRTIO_SND_EVT_JACK_DISCONNECTED: u32 = 0x1001;
pub const VIRTIO_SND_EVT_PCM_PERIOD_ELAPSED: u32 = 0x1100;
pub const VIRTIO_SND_EVT_PCM_XRUN: u32 = 0x1101;
pub const VIRTIO_SND_EVT_CTL_NOTIFY: u32 = 0x1200;
pub const VIRTIO_SND_S_OK: u32 = 0x8000;
pub const VIRTIO_SND_S_BAD_MSG: u32 = 0x8001;
pub const VIRTIO_SND_S_NOT_SUPP: u32 = 0x8002;
pub const VIRTIO_SND_S_IO_ERR: u32 = 0x8003;

#[repr(C)]
pub struct virtio_snd_hdr { pub code: u32 }
#[repr(C)]
pub struct virtio_snd_event { pub hdr: virtio_snd_hdr, pub data: u32 }
#[repr(C)]
pub struct virtio_snd_query_info { pub hdr: virtio_snd_hdr, pub start_id: u32, pub count: u32, pub size: u32 }
#[repr(C)]
pub struct virtio_snd_info { pub hda_fn_nid: u32 }

/* JACK CONTROL MESSAGES */
#[repr(C)]
pub struct virtio_snd_jack_hdr { pub hdr: virtio_snd_hdr, pub jack_id: u32 }
pub const VIRTIO_SND_JACK_F_REMAP: u32 = 0;
#[repr(C)]
pub struct virtio_snd_jack_info { pub hdr: virtio_snd_info, pub features: u32, pub hda_reg_defconf: u32, pub hda_reg_caps: u32, pub connected: u8, pub padding: [u8; 7] }
#[repr(C)]
pub struct virtio_snd_jack_remap { pub hdr: virtio_snd_jack_hdr, pub association: u32, pub sequence: u32 }

/* PCM CONTROL MESSAGES */
#[repr(C)]
pub struct virtio_snd_pcm_hdr { pub hdr: virtio_snd_hdr, pub stream_id: u32 }
pub const VIRTIO_SND_PCM_F_SHMEM_HOST: u32 = 0;
pub const VIRTIO_SND_PCM_F_SHMEM_GUEST: u32 = 1;
pub const VIRTIO_SND_PCM_F_MSG_POLLING: u32 = 2;
pub const VIRTIO_SND_PCM_F_EVT_SHMEM_PERIODS: u32 = 3;
pub const VIRTIO_SND_PCM_F_EVT_XRUNS: u32 = 4;

pub const VIRTIO_SND_PCM_FMT_IMA_ADPCM: u32 = 0;
pub const VIRTIO_SND_PCM_FMT_MU_LAW: u32 = 1;
pub const VIRTIO_SND_PCM_FMT_A_LAW: u32 = 2;
pub const VIRTIO_SND_PCM_FMT_S8: u32 = 3;
pub const VIRTIO_SND_PCM_FMT_U8: u32 = 4;
pub const VIRTIO_SND_PCM_FMT_S16: u32 = 5;
pub const VIRTIO_SND_PCM_FMT_U16: u32 = 6;
pub const VIRTIO_SND_PCM_FMT_S18_3: u32 = 7;
pub const VIRTIO_SND_PCM_FMT_U18_3: u32 = 8;
pub const VIRTIO_SND_PCM_FMT_S20_3: u32 = 9;
pub const VIRTIO_SND_PCM_FMT_U20_3: u32 = 10;
pub const VIRTIO_SND_PCM_FMT_S24_3: u32 = 11;
pub const VIRTIO_SND_PCM_FMT_U24_3: u32 = 12;
pub const VIRTIO_SND_PCM_FMT_S20: u32 = 13;
pub const VIRTIO_SND_PCM_FMT_U20: u32 = 14;
pub const VIRTIO_SND_PCM_FMT_S24: u32 = 15;
pub const VIRTIO_SND_PCM_FMT_U24: u32 = 16;
pub const VIRTIO_SND_PCM_FMT_S32: u32 = 17;
pub const VIRTIO_SND_PCM_FMT_U32: u32 = 18;
pub const VIRTIO_SND_PCM_FMT_FLOAT: u32 = 19;
pub const VIRTIO_SND_PCM_FMT_FLOAT64: u32 = 20;
pub const VIRTIO_SND_PCM_FMT_DSD_U8: u32 = 21;
pub const VIRTIO_SND_PCM_FMT_DSD_U16: u32 = 22;
pub const VIRTIO_SND_PCM_FMT_DSD_U32: u32 = 23;
pub const VIRTIO_SND_PCM_FMT_IEC958_SUBFRAME: u32 = 24;
pub const VIRTIO_SND_PCM_RATE_5512: u32 = 0;
pub const VIRTIO_SND_PCM_RATE_8000: u32 = 1;
pub const VIRTIO_SND_PCM_RATE_11025: u32 = 2;
pub const VIRTIO_SND_PCM_RATE_16000: u32 = 3;
pub const VIRTIO_SND_PCM_RATE_22050: u32 = 4;
pub const VIRTIO_SND_PCM_RATE_32000: u32 = 5;
pub const VIRTIO_SND_PCM_RATE_44100: u32 = 6;
pub const VIRTIO_SND_PCM_RATE_48000: u32 = 7;
pub const VIRTIO_SND_PCM_RATE_64000: u32 = 8;
pub const VIRTIO_SND_PCM_RATE_88200: u32 = 9;
pub const VIRTIO_SND_PCM_RATE_96000: u32 = 10;
pub const VIRTIO_SND_PCM_RATE_176400: u32 = 11;
pub const VIRTIO_SND_PCM_RATE_192000: u32 = 12;
pub const VIRTIO_SND_PCM_RATE_384000: u32 = 13;

#[repr(C)]
pub struct virtio_snd_pcm_info { pub hdr: virtio_snd_info, pub features: u32, pub formats: u64, pub rates: u64, pub direction: u8, pub channels_min: u8, pub channels_max: u8, pub padding: [u8; 5] }
#[repr(C)]
pub struct virtio_snd_pcm_set_params { pub hdr: virtio_snd_pcm_hdr, pub buffer_bytes: u32, pub period_bytes: u32, pub features: u32, pub channels: u8, pub format: u8, pub rate: u8, pub padding: u8 }

/* PCM I/O MESSAGES */
#[repr(C)]
pub struct virtio_snd_pcm_xfer { pub stream_id: u32 }
#[repr(C)]
pub struct virtio_snd_pcm_status { pub status: u32, pub latency_bytes: u32 }

/* CHANNEL MAP CONTROL MESSAGES */
#[repr(C)]
pub struct virtio_snd_chmap_hdr { pub hdr: virtio_snd_hdr, pub chmap_id: u32 }
pub const VIRTIO_SND_CHMAP_NONE: u32 = 0;
pub const VIRTIO_SND_CHMAP_NA: u32 = 1;
pub const VIRTIO_SND_CHMAP_MONO: u32 = 2;
pub const VIRTIO_SND_CHMAP_FL: u32 = 3;
pub const VIRTIO_SND_CHMAP_FR: u32 = 4;
pub const VIRTIO_SND_CHMAP_RL: u32 = 5;
pub const VIRTIO_SND_CHMAP_RR: u32 = 6;
pub const VIRTIO_SND_CHMAP_FC: u32 = 7;
pub const VIRTIO_SND_CHMAP_LFE: u32 = 8;
pub const VIRTIO_SND_CHMAP_SL: u32 = 9;
pub const VIRTIO_SND_CHMAP_SR: u32 = 10;
pub const VIRTIO_SND_CHMAP_RC: u32 = 11;
pub const VIRTIO_SND_CHMAP_FLC: u32 = 12;
pub const VIRTIO_SND_CHMAP_FRC: u32 = 13;
pub const VIRTIO_SND_CHMAP_RLC: u32 = 14;
pub const VIRTIO_SND_CHMAP_RRC: u32 = 15;
pub const VIRTIO_SND_CHMAP_FLW: u32 = 16;
pub const VIRTIO_SND_CHMAP_FRW: u32 = 17;
pub const VIRTIO_SND_CHMAP_FLH: u32 = 18;
pub const VIRTIO_SND_CHMAP_FCH: u32 = 19;
pub const VIRTIO_SND_CHMAP_FRH: u32 = 20;
pub const VIRTIO_SND_CHMAP_TC: u32 = 21;
pub const VIRTIO_SND_CHMAP_TFL: u32 = 22;
pub const VIRTIO_SND_CHMAP_TFR: u32 = 23;
pub const VIRTIO_SND_CHMAP_TFC: u32 = 24;
pub const VIRTIO_SND_CHMAP_TRL: u32 = 25;
pub const VIRTIO_SND_CHMAP_TRR: u32 = 26;
pub const VIRTIO_SND_CHMAP_TRC: u32 = 27;
pub const VIRTIO_SND_CHMAP_TFLC: u32 = 28;
pub const VIRTIO_SND_CHMAP_TFRC: u32 = 29;
pub const VIRTIO_SND_CHMAP_TSL: u32 = 30;
pub const VIRTIO_SND_CHMAP_TSR: u32 = 31;
pub const VIRTIO_SND_CHMAP_LLFE: u32 = 32;
pub const VIRTIO_SND_CHMAP_RLFE: u32 = 33;
pub const VIRTIO_SND_CHMAP_BC: u32 = 34;
pub const VIRTIO_SND_CHMAP_BLC: u32 = 35;
pub const VIRTIO_SND_CHMAP_BRC: u32 = 36;
pub const VIRTIO_SND_CHMAP_MAX_SIZE: usize = 18;
#[repr(C)]
pub struct virtio_snd_chmap_info { pub hdr: virtio_snd_info, pub direction: u8, pub channels: u8, pub positions: [u8; VIRTIO_SND_CHMAP_MAX_SIZE] }

/* CONTROL ELEMENTS MESSAGES */
#[repr(C)]
pub struct virtio_snd_ctl_hdr { pub hdr: virtio_snd_hdr, pub control_id: u32 }
pub const VIRTIO_SND_CTL_ROLE_UNDEFINED: u32 = 0;
pub const VIRTIO_SND_CTL_ROLE_VOLUME: u32 = 1;
pub const VIRTIO_SND_CTL_ROLE_MUTE: u32 = 2;
pub const VIRTIO_SND_CTL_ROLE_GAIN: u32 = 3;
pub const VIRTIO_SND_CTL_TYPE_BOOLEAN: u32 = 0;
pub const VIRTIO_SND_CTL_TYPE_INTEGER: u32 = 1;
pub const VIRTIO_SND_CTL_TYPE_INTEGER64: u32 = 2;
pub const VIRTIO_SND_CTL_TYPE_ENUMERATED: u32 = 3;
pub const VIRTIO_SND_CTL_TYPE_BYTES: u32 = 4;
pub const VIRTIO_SND_CTL_TYPE_IEC958: u32 = 5;
pub const VIRTIO_SND_CTL_ACCESS_READ: u32 = 0;
pub const VIRTIO_SND_CTL_ACCESS_WRITE: u32 = 1;
pub const VIRTIO_SND_CTL_ACCESS_VOLATILE: u32 = 2;
pub const VIRTIO_SND_CTL_ACCESS_INACTIVE: u32 = 3;
pub const VIRTIO_SND_CTL_ACCESS_TLV_READ: u32 = 4;
pub const VIRTIO_SND_CTL_ACCESS_TLV_WRITE: u32 = 5;
pub const VIRTIO_SND_CTL_ACCESS_TLV_COMMAND: u32 = 6;

#[repr(C)]
pub struct virtio_snd_ctl_info {
    pub hdr: virtio_snd_info, pub role: u32, pub type_: u32, pub access: u32, pub count: u32, pub index: u32, pub name: [u8; 44], pub value: virtio_snd_ctl_info_value,
}
#[repr(C)]
pub union virtio_snd_ctl_info_value { pub integer: virtio_snd_ctl_info_integer, pub integer64: virtio_snd_ctl_info_integer64, pub enumerated: virtio_snd_ctl_info_enumerated }
#[repr(C)]
pub struct virtio_snd_ctl_info_integer { pub min: u32, pub max: u32, pub step: u32 }
#[repr(C)]
pub struct virtio_snd_ctl_info_integer64 { pub min: u64, pub max: u64, pub step: u64 }
#[repr(C)]
pub struct virtio_snd_ctl_info_enumerated { pub items: u32 }
#[repr(C)]
pub struct virtio_snd_ctl_enum_item { pub item: [u8; 64] }
#[repr(C)]
pub struct virtio_snd_ctl_iec958 { pub status: [u8; 24], pub subcode: [u8; 147], pub pad: u8, pub dig_subframe: [u8; 4] }
#[repr(C)]
pub union virtio_snd_ctl_value_union { pub integer: [u32; 128], pub integer64: [u64; 64], pub enumerated: [u32; 128], pub bytes: [u8; 512], pub iec958: virtio_snd_ctl_iec958 }
#[repr(C)]
pub struct virtio_snd_ctl_value { pub value: virtio_snd_ctl_value_union }
pub const VIRTIO_SND_CTL_EVT_MASK_VALUE: u32 = 0;
pub const VIRTIO_SND_CTL_EVT_MASK_INFO: u32 = 1;
pub const VIRTIO_SND_CTL_EVT_MASK_TLV: u32 = 2;
#[repr(C)]
pub struct virtio_snd_ctl_event { pub hdr: virtio_snd_hdr, pub control_id: u16, pub mask: u16 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
