// SPDX-License-Identifier: GPL-2.0-only
//
//   intel_hdmi_audio.rs - Intel HDMI audio driver
//
//  Copyright (C) 2016 Intel Corp
//  Authors:	Sailaja Bandarupalli <sailaja.bandarupalli@intel.com>
//		Ramesh Babu K V	<ramesh.babu@intel.com>
//		Vaibhav Agarwal <vaibhav.agarwal@intel.com>
//		Jerome Anand <jerome.anand@intel.com>
//
// ALSA driver for Intel HDMI audio

use std::mem;

const INTEL_HDMI_AUDIO_SUSPEND_DELAY_MS: u32 = 5000;

// Module static variables (would be module parameters in actual kernel module)
static mut HDMI_CARD_INDEX: i32 = 0; // SNDRV_DEFAULT_IDX1
static mut HDMI_CARD_ID: *const u8 = std::ptr::null(); // SNDRV_DEFAULT_STR1
static SINGLE_PORT: bool = false;

// ELD SA bits in the CEA Speaker Allocation data block
const FL: i32 = 0x01;
const FR: i32 = 0x02;
const LFE: i32 = 0x04;
const FC: i32 = 0x08;
const RL: i32 = 0x10;
const RR: i32 = 0x20;
const RLC: i32 = 0x40;
const RRC: i32 = 0x80;
const FLC: i32 = 0x100;
const FRC: i32 = 0x200;
const RC: i32 = 0x400;

static ELD_SPEAKER_ALLOCATION_BITS: [i32; 8] = [
    FL | FR,
    LFE,
    FC,
    RL | RR,
    RC,
    FLC | FRC,
    RLC | RRC,
    0,
];

// ALSA channel map constants
const SNDRV_CHMAP_FL: u32 = 0;
const SNDRV_CHMAP_FR: u32 = 1;
const SNDRV_CHMAP_RL: u32 = 4;
const SNDRV_CHMAP_RR: u32 = 5;
const SNDRV_CHMAP_LFE: u32 = 2;
const SNDRV_CHMAP_FC: u32 = 3;
const SNDRV_CHMAP_RLC: u32 = 6;
const SNDRV_CHMAP_RRC: u32 = 7;
const SNDRV_CHMAP_LAST: u32 = 63;

const MAX_SMPL_WIDTH_20: u8 = 0;
const MAX_SMPL_WIDTH_24: u8 = 1;
const SMPL_WIDTH_16BITS: u8 = 0;
const SMPL_WIDTH_24BITS: u8 = 1;

const LAYOUT0: u8 = 0;
const LAYOUT1: u8 = 1;

const FIFO_THRESHOLD: u8 = 0;
const DMA_FIFO_THRESHOLD: u8 = 0;

const BYTES_PER_WORD: usize = 4;
const HAD_MAX_DIP_WORDS: usize = 8;
const VALID_DIP_WORDS: usize = 3;
const MAX_SPEAKERS: usize = 8;

const HAD_MIN_RATE: u32 = 32000;
const HAD_MAX_RATE: u32 = 192000;
const HAD_MIN_CHANNEL: u32 = 2;
const HAD_MAX_CHANNEL: u32 = 8;
const HAD_MAX_BUFFER: u64 = 0;
const HAD_MIN_PERIOD_BYTES: u64 = 0;
const HAD_MAX_PERIOD_BYTES: u64 = 0;
const HAD_MIN_PERIODS: u32 = 0;
const HAD_MAX_PERIODS: u32 = 0;
const HAD_FIFO_SIZE: u32 = 0;
const HAD_DEFAULT_BUFFER: u64 = 0;

const HAD_NUM_OF_RING_BUFS: usize = 4;
const MAX_PB_STREAMS: u32 = 1;
const MAX_CAP_STREAMS: u32 = 0;

const HDMI_MAX_ELD_BYTES: usize = 128;
const DRM_ELD_SPEAKER: usize = 7;

const HDMI_AUDIO_BUFFER_DONE: u32 = 0x80000000;
const HDMI_AUDIO_UNDERRUN: u32 = 0x40000000;
const AUD_HDMI_STATUS_MASK_UNDERRUN: u32 = 0xC0000000;

const HAD_REG_WIDTH: u32 = 0;
const AUD_BUF_VALID: u32 = 0x80000000;
const AUD_BUF_INTR_EN: u32 = 0x40000000;
const AUD_HDMI_STATUSG_MASK_FUNCRST: u32 = 0x20000000;

const AUD_CONFIG_OFFSET_A: u32 = 0x0;
const AUD_CONFIG_OFFSET_B: u32 = 0x100;
const AUD_CONFIG_OFFSET_C: u32 = 0x200;

const AUDIO_HDMI_CONFIG_A: u32 = 0x0;
const AUDIO_HDMI_CONFIG_B: u32 = 0x100;
const AUDIO_HDMI_CONFIG_C: u32 = 0x200;

const AUD_CONFIG: u32 = 0;
const AUD_CH_STATUS_0: u32 = 0;
const AUD_CH_STATUS_1: u32 = 0;
const AUD_CNTL_ST: u32 = 0;
const AUD_HDMIW_INFOFR: u32 = 0;
const AUD_N_ENABLE: u32 = 0;
const AUD_HDMI_CTS: u32 = 0;
const AUD_HDMI_STATUS: u32 = 0;
const AUD_BUF_CONFIG: u32 = 0;
const AUD_BUF_CH_SWAP: u32 = 0;
const AUD_BUF_A_ADDR: u32 = 0;
const AUD_BUF_A_LENGTH: u32 = 0;

const SWAP_LFE_CENTER: u32 = 0;

const DP_INFO_FRAME_WORD1: u32 = 0;
const HDMI_INFO_FRAME_WORD1: u32 = 0;

const DP_2_7_GHZ: u32 = 0;
const DP_1_62_GHZ: u32 = 0;

const AUD_SAMPLE_RATE_32: u32 = 32000;
const AUD_SAMPLE_RATE_44_1: u32 = 44100;
const AUD_SAMPLE_RATE_48: u32 = 48000;
const AUD_SAMPLE_RATE_88_2: u32 = 88200;
const AUD_SAMPLE_RATE_96: u32 = 96000;
const AUD_SAMPLE_RATE_176_4: u32 = 176400;
const AUD_SAMPLE_RATE_192: u32 = 192000;

const CH_STATUS_MAP_32KHZ: u8 = 3;
const CH_STATUS_MAP_44KHZ: u8 = 0;
const CH_STATUS_MAP_48KHZ: u8 = 2;
const CH_STATUS_MAP_88KHZ: u8 = 8;
const CH_STATUS_MAP_96KHZ: u8 = 10;
const CH_STATUS_MAP_176KHZ: u8 = 12;
const CH_STATUS_MAP_192KHZ: u8 = 14;

const AUD_SAMPLE_RATE_32_DP_2_7_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_44_1_DP_2_7_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_48_DP_2_7_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_88_2_DP_2_7_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_96_DP_2_7_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_176_4_DP_2_7_MAUD_VAL: u32 = 0;
const HAD_MAX_RATE_DP_2_7_MAUD_VAL: u32 = 0;

const AUD_SAMPLE_RATE_32_DP_1_62_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_44_1_DP_1_62_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_48_DP_1_62_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_88_2_DP_1_62_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_96_DP_1_62_MAUD_VAL: u32 = 0;
const AUD_SAMPLE_RATE_176_4_DP_1_62_MAUD_VAL: u32 = 0;
const HAD_MAX_RATE_DP_1_62_MAUD_VAL: u32 = 0;

const DP_NAUD_VAL: i32 = 32768;

const SNDRV_PCM_FORMAT_S16_LE: u32 = 2;
const SNDRV_PCM_FORMAT_S24_LE: u32 = 6;
const SNDRV_PCM_FORMAT_S32_LE: u32 = 10;

const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: u32 = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: u32 = 0;
const SNDRV_PCM_STREAM_PLAYBACK: u32 = 0;

const SNDRV_CTL_ELEM_TYPE_INTEGER: u32 = 1;
const SNDRV_CTL_ELEM_TYPE_BYTES: u32 = 4;
const SNDRV_CTL_ELEM_TYPE_IEC958: u32 = 5;

const SNDRV_CTL_ELEM_IFACE_PCM: u32 = 1;

const SNDRV_CTL_ELEM_ACCESS_READ: u32 = 1;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: u32 = 4;

const SNDRV_CTL_POWER_D0: u32 = 0;
const SNDRV_CTL_POWER_D3hot: u32 = 3;

const IEC958_AES0_NONAUDIO: u32 = 0x01;
const IEC958_AES3_CON_CLOCK: u32 = 0xF0;

const SND_JACK_AVOUT: u32 = 0x4000;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const EACCES: i32 = 13;
const EPIPE: i32 = 32;

const IRQ_HANDLED: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
union AudCfg {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudBufConfig {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudChStatus0 {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudChStatus1 {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudCtrlSt {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudInfoFrame2 {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union AudInfoFrame3 {
    regval: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CeaChannelSpeakerAllocation {
    ca_index: u8,
    speakers: [i32; 8],
    channels: u8,
    spk_mask: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ChannelMapTable {
    map: u32,
    map_val: u32,
    spk_mask: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SndPcmHardware {
    info: u32,
    formats: u64,
    rates: u64,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: u64,
    period_bytes_min: u64,
    period_bytes_max: u64,
    periods_min: u32,
    periods_max: u32,
    fifo_size: u32,
}

static CHANNEL_ALLOCATIONS: [CeaChannelSpeakerAllocation; 30] = [
    CeaChannelSpeakerAllocation {
        ca_index: 0x00,
        speakers: [0, 0, 0, 0, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x01,
        speakers: [0, 0, 0, 0, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x02,
        speakers: [0, 0, 0, 0, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x08,
        speakers: [0, 0, RR, RL, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x09,
        speakers: [0, 0, RR, RL, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0a,
        speakers: [0, 0, RR, RL, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0b,
        speakers: [0, 0, RR, RL, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0f,
        speakers: [0, RC, RR, RL, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x13,
        speakers: [RRC, RLC, RR, RL, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x03,
        speakers: [0, 0, 0, 0, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x04,
        speakers: [0, 0, 0, RC, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x05,
        speakers: [0, 0, 0, RC, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x06,
        speakers: [0, 0, 0, RC, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x07,
        speakers: [0, 0, 0, RC, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0c,
        speakers: [0, RC, RR, RL, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0d,
        speakers: [0, RC, RR, RL, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x0e,
        speakers: [0, RC, RR, RL, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x10,
        speakers: [RRC, RLC, RR, RL, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x11,
        speakers: [RRC, RLC, RR, RL, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x12,
        speakers: [RRC, RLC, RR, RL, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x14,
        speakers: [FRC, FLC, 0, 0, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x15,
        speakers: [FRC, FLC, 0, 0, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x16,
        speakers: [FRC, FLC, 0, 0, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x17,
        speakers: [FRC, FLC, 0, 0, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x18,
        speakers: [FRC, FLC, 0, RC, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x19,
        speakers: [FRC, FLC, 0, RC, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1a,
        speakers: [FRC, FLC, 0, RC, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1b,
        speakers: [FRC, FLC, 0, RC, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1c,
        speakers: [FRC, FLC, RR, RL, 0, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1d,
        speakers: [FRC, FLC, RR, RL, 0, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1e,
        speakers: [FRC, FLC, RR, RL, FC, 0, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
    CeaChannelSpeakerAllocation {
        ca_index: 0x1f,
        speakers: [FRC, FLC, RR, RL, FC, LFE, FR, FL],
        channels: 0,
        spk_mask: 0,
    },
];

static MAP_TABLES: [ChannelMapTable; 9] = [
    ChannelMapTable { map: SNDRV_CHMAP_FL, map_val: 0x00, spk_mask: FL },
    ChannelMapTable { map: SNDRV_CHMAP_FR, map_val: 0x01, spk_mask: FR },
    ChannelMapTable { map: SNDRV_CHMAP_RL, map_val: 0x04, spk_mask: RL },
    ChannelMapTable { map: SNDRV_CHMAP_RR, map_val: 0x05, spk_mask: RR },
    ChannelMapTable { map: SNDRV_CHMAP_LFE, map_val: 0x02, spk_mask: LFE },
    ChannelMapTable { map: SNDRV_CHMAP_FC, map_val: 0x03, spk_mask: FC },
    ChannelMapTable { map: SNDRV_CHMAP_RLC, map_val: 0x06, spk_mask: RLC },
    ChannelMapTable { map: SNDRV_CHMAP_RRC, map_val: 0x07, spk_mask: RRC },
    ChannelMapTable { map: 0, map_val: 0, spk_mask: 0 },
];

fn had_config_offset(pipe: i32) -> u32 {
    match pipe {
        0 => AUDIO_HDMI_CONFIG_A,
        1 => AUDIO_HDMI_CONFIG_B,
        2 => AUDIO_HDMI_CONFIG_C,
        _ => AUDIO_HDMI_CONFIG_A,
    }
}

fn had_read_register_raw(card_ctx: *const SndIntelhadsCard, pipe: i32, reg: u32) -> u32 {
    unsafe {
        let offset = had_config_offset(pipe);
        std::ptr::read_volatile(((*card_ctx).mmio_start as *mut u32).add((offset + reg) as usize))
    }
}

fn had_write_register_raw(card_ctx: *const SndIntelhadsCard, pipe: i32, reg: u32, val: u32) {
    unsafe {
        let offset = had_config_offset(pipe);
        std::ptr::write_volatile(((*card_ctx).mmio_start as *mut u32).add((offset + reg) as usize), val)
    }
}

fn had_read_register(ctx: *const SndIntelhad, reg: u32, val: *mut u32) {
    unsafe {
        if !(*ctx).connected {
            *val = 0;
        } else {
            *val = had_read_register_raw((*ctx).card_ctx, (*ctx).pipe, reg);
        }
    }
}

fn had_write_register(ctx: *const SndIntelhad, reg: u32, val: u32) {
    unsafe {
        if (*ctx).connected {
            had_write_register_raw((*ctx).card_ctx, (*ctx).pipe, reg, val);
        }
    }
}

fn had_enable_audio(intelhaddata: *mut SndIntelhad, enable: bool) {
    unsafe {
        (*intelhaddata).aud_config.regval = if enable { 1 } else { 0 };
        had_write_register(intelhaddata, AUD_CONFIG, (*intelhaddata).aud_config.regval);
    }
}

fn had_ack_irqs(ctx: *const SndIntelhad) {
    unsafe {
        if !(*ctx).connected {
            return;
        }
        let mut status_reg = 0u32;
        had_read_register(ctx, AUD_HDMI_STATUS, &mut status_reg);
        status_reg |= HDMI_AUDIO_BUFFER_DONE | HDMI_AUDIO_UNDERRUN;
        had_write_register(ctx, AUD_HDMI_STATUS, status_reg);
        had_read_register(ctx, AUD_HDMI_STATUS, &mut status_reg);
    }
}

fn had_reset_audio(intelhaddata: *mut SndIntelhad) {
    unsafe {
        had_write_register(intelhaddata, AUD_HDMI_STATUS, AUD_HDMI_STATUSG_MASK_FUNCRST);
        had_write_register(intelhaddata, AUD_HDMI_STATUS, 0);
    }
}

fn had_prog_status_reg(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) -> i32 {
    unsafe {
        let mut ch_stat0 = AudChStatus0 { regval: 0 };
        let mut ch_stat1 = AudChStatus1 { regval: 0 };

        // Extract and set channel status fields (implementation simplified)
        had_write_register(intelhaddata, AUD_CH_STATUS_0, ch_stat0.regval);
        had_write_register(intelhaddata, AUD_CH_STATUS_1, ch_stat1.regval);
        0
    }
}

fn had_init_audio_ctrl(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) -> i32 {
    unsafe {
        let mut buf_cfg = AudBufConfig { regval: 0 };
        let mut cfg_val = AudCfg { regval: 0 };

        had_prog_status_reg(_substream, intelhaddata);

        // Set buffer configuration fields (implementation simplified)
        had_write_register(intelhaddata, AUD_BUF_CONFIG, buf_cfg.regval);
        had_write_register(intelhaddata, AUD_CONFIG, cfg_val.regval);
        (*intelhaddata).aud_config = cfg_val;
        0
    }
}

fn init_channel_allocations() {
    // Channel allocations are statically initialized in Rust
}

fn had_channel_allocation(_intelhaddata: *const SndIntelhad, channels: i32) -> i32 {
    if channels <= 2 {
        return 0;
    }
    0
}

fn spk_to_chmap(_spk: i32) -> u32 {
    0
}

fn had_build_channel_allocation_map(_intelhaddata: *mut SndIntelhad) {
}

fn had_chmap_ctl_info(_kcontrol: *const (), uinfo: *mut ()) -> i32 {
    unsafe {
        *(uinfo as *mut u32) = SNDRV_CTL_ELEM_TYPE_INTEGER;
    }
    0
}

fn had_chmap_ctl_get(_kcontrol: *const (), _ucontrol: *mut ()) -> i32 {
    0
}

fn had_register_chmap_ctls(_intelhaddata: *mut SndIntelhad, _pcm: *const ()) -> i32 {
    0
}

fn had_prog_dip(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) {
    unsafe {
        let mut ctrl_state = AudCtrlSt { regval: 0 };
        let mut frame2 = AudInfoFrame2 { regval: 0 };
        let mut frame3 = AudInfoFrame3 { regval: 0 };

        had_write_register(intelhaddata, AUD_CNTL_ST, ctrl_state.regval);
        had_write_register(intelhaddata, AUD_HDMIW_INFOFR, 0);
        had_write_register(intelhaddata, AUD_HDMIW_INFOFR, frame2.regval);
        had_write_register(intelhaddata, AUD_HDMIW_INFOFR, frame3.regval);

        for _i in 0..(HAD_MAX_DIP_WORDS - VALID_DIP_WORDS) {
            had_write_register(intelhaddata, AUD_HDMIW_INFOFR, 0x0);
        }

        had_write_register(intelhaddata, AUD_CNTL_ST, ctrl_state.regval);
    }
}

fn had_calculate_maud_value(_aud_samp_freq: u32, _link_rate: u32) -> u32 {
    0
}

fn had_prog_cts(_aud_samp_freq: u32, _tmds: u32, _link_rate: u32, _n_param: u32, intelhaddata: *mut SndIntelhad) {
    unsafe {
        let cts_val = 0u32;
        had_write_register(intelhaddata, AUD_HDMI_CTS, (1u32 << 24) | cts_val);
    }
}

fn had_calculate_n_value(aud_samp_freq: u32) -> i32 {
    match aud_samp_freq {
        AUD_SAMPLE_RATE_32 => 4096,
        AUD_SAMPLE_RATE_44_1 => 6272,
        AUD_SAMPLE_RATE_48 => 6144,
        AUD_SAMPLE_RATE_88_2 => 12544,
        AUD_SAMPLE_RATE_96 => 12288,
        AUD_SAMPLE_RATE_176_4 => 25088,
        AUD_SAMPLE_RATE_192 => 24576,
        _ => -EINVAL,
    }
}

fn had_prog_n(aud_samp_freq: u32, n_param: *mut u32, intelhaddata: *mut SndIntelhad) -> i32 {
    unsafe {
        let n_val = if (*intelhaddata).dp_output {
            DP_NAUD_VAL
        } else {
            had_calculate_n_value(aud_samp_freq)
        };

        if n_val < 0 {
            return n_val;
        }

        had_write_register(intelhaddata, AUD_N_ENABLE, (1u32 << 24) | (n_val as u32));
        *n_param = n_val as u32;
        0
    }
}

fn aud_buf_addr(x: usize) -> u32 {
    AUD_BUF_A_ADDR + (x as u32) * HAD_REG_WIDTH
}

fn aud_buf_len(x: usize) -> u32 {
    AUD_BUF_A_LENGTH + (x as u32) * HAD_REG_WIDTH
}

fn had_prog_bd(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) {
    unsafe {
        let idx = (*intelhaddata).bd_head;
        let _ofs = (*intelhaddata).pcmbuf_filled * (*intelhaddata).period_bytes;

        let mut addr = 0u32;
        addr |= AUD_BUF_VALID;
        had_write_register(intelhaddata, aud_buf_addr(idx), addr);
        had_write_register(intelhaddata, aud_buf_len(idx), (*intelhaddata).period_bytes);

        (*intelhaddata).bd_head += 1;
        (*intelhaddata).bd_head %= (*intelhaddata).num_bds;
        (*intelhaddata).pcmbuf_filled += 1;
    }
}

fn had_invalidate_bd(intelhaddata: *mut SndIntelhad, idx: usize) {
    unsafe {
        had_write_register(intelhaddata, aud_buf_addr(idx), 0);
        had_write_register(intelhaddata, aud_buf_len(idx), 0);
    }
}

fn had_init_ringbuf(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) {
    unsafe {
        (*intelhaddata).num_bds = 4;
        (*intelhaddata).num_bds = (*intelhaddata).num_bds.max(2);
        (*intelhaddata).period_bytes = 4096;

        (*intelhaddata).bd_head = 0;
        (*intelhaddata).pcmbuf_head = 0;
        (*intelhaddata).pcmbuf_filled = 0;

        for i in 0..HAD_NUM_OF_RING_BUFS {
            if i < (*intelhaddata).num_bds {
                had_prog_bd(_substream, intelhaddata);
            } else {
                had_invalidate_bd(intelhaddata, i);
            }
        }

        (*intelhaddata).bd_head = 0;
    }
}

fn had_advance_ringbuf(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) {
    unsafe {
        had_prog_bd(_substream, intelhaddata);
        (*intelhaddata).pcmbuf_head += 1;
    }
}

fn had_process_ringbuf(_substream: *const SndPcmSubstream, intelhaddata: *mut SndIntelhad) -> i32 {
    unsafe {
        let mut processed = 0;
        loop {
            let mut len = 0i32;
            had_read_register(intelhaddata, aud_buf_len((*intelhaddata).bd_head), &mut (len as u32));

            if len < 0 || len > (*intelhaddata).period_bytes as i32 {
                return -EPIPE;
            }

            if len > 0 {
                break;
            }

            processed += 1;
            if processed >= (*intelhaddata).num_bds as i32 {
                return -EPIPE;
            }
            had_advance_ringbuf(_substream, intelhaddata);
        }

        let mut len = 0u32;
        had_read_register(intelhaddata, aud_buf_len((*intelhaddata).bd_head), &mut len);
        let len = (*intelhaddata).period_bytes as i32 - len as i32;
        (len + ((*intelhaddata).period_bytes as i32 * (*intelhaddata).pcmbuf_head as i32))
    }
}

fn had_process_buffer_done(_intelhaddata: *mut SndIntelhad) {
}

fn wait_clear_underrun_bit(_intelhaddata: *mut SndIntelhad) {
    for _i in 0..100 {
        unsafe {
            let mut val = 0u32;
            had_read_register(_intelhaddata, AUD_HDMI_STATUS, &mut val);
            if (val & AUD_HDMI_STATUS_MASK_UNDERRUN) == 0 {
                return;
            }
        }
    }
}

fn had_pcm_sync_stop(_substream: *const SndPcmSubstream) -> i32 {
    0
}

fn had_process_buffer_underrun(_intelhaddata: *mut SndIntelhad) {
}

fn had_pcm_open(_substream: *mut SndPcmSubstream) -> i32 {
    0
}

fn had_pcm_close(_substream: *mut SndPcmSubstream) -> i32 {
    0
}

fn had_pcm_hw_params(_substream: *mut SndPcmSubstream, _hw_params: *const ()) -> i32 {
    0
}

fn had_pcm_trigger(_substream: *mut SndPcmSubstream, _cmd: i32) -> i32 {
    0
}

fn had_pcm_prepare(_substream: *mut SndPcmSubstream) -> i32 {
    0
}

fn had_pcm_pointer(_substream: *const SndPcmSubstream) -> u64 {
    0
}

fn had_process_mode_change(_intelhaddata: *mut SndIntelhad) -> i32 {
    0
}

fn had_process_hot_plug(_intelhaddata: *mut SndIntelhad) {
}

fn had_process_hot_unplug(_intelhaddata: *mut SndIntelhad) {
}

fn had_iec958_info(_kcontrol: *const (), _uinfo: *mut ()) -> i32 {
    0
}

fn had_iec958_get(_kcontrol: *const (), _ucontrol: *mut ()) -> i32 {
    0
}

fn had_iec958_mask_get(_kcontrol: *const (), _ucontrol: *mut ()) -> i32 {
    0
}

fn had_iec958_put(_kcontrol: *const (), _ucontrol: *mut ()) -> i32 {
    0
}

fn had_ctl_eld_info(_kcontrol: *const (), _uinfo: *mut ()) -> i32 {
    0
}

fn had_ctl_eld_get(_kcontrol: *const (), _ucontrol: *mut ()) -> i32 {
    0
}

fn display_pipe_interrupt_handler(_irq: i32, dev_id: *mut ()) -> u32 {
    IRQ_HANDLED
}

fn notify_audio_lpe(_pdev: *const (), _port: i32) {
}

fn had_audio_wq(_work: *const ()) {
}

fn had_create_jack(_ctx: *mut SndIntelhad, _pcm: *const ()) -> i32 {
    0
}

fn hdmi_lpe_audio_suspend(_dev: *const ()) -> i32 {
    0
}

fn hdmi_lpe_audio_resume(_dev: *const ()) -> i32 {
    0
}

fn hdmi_lpe_audio_free(_card: *const SndsCard) {
}

fn __hdmi_lpe_audio_probe(_pdev: *const PlatformDevice) -> i32 {
    0
}

fn hdmi_lpe_audio_probe(_pdev: *const PlatformDevice) -> i32 {
    0
}

#[repr(C)]
struct SndIntelhad {
    card_ctx: *const SndIntelhadsCard,
    dev: *const (),
    port: i32,
    pipe: i32,
    connected: bool,
    dp_output: bool,
    tmds_clock_speed: u32,
    link_rate: u32,
    eld: [u8; HDMI_MAX_ELD_BYTES],
    aud_config: AudCfg,
    aes_bits: u32,
    jack: *const (),
    chmap: *const (),
    stream_info: StreamInfo,
    had_spinlock: SpinLock,
    mutex: Mutex,
    bd_head: usize,
    pcmbuf_head: usize,
    pcmbuf_filled: usize,
    num_bds: usize,
    period_bytes: usize,
    hdmi_audio_wq: WorkStruct,
}

#[repr(C)]
struct StreamInfo {
    substream: *const SndPcmSubstream,
    substream_refcount: i32,
}

#[repr(C)]
struct SndIntelhadsCard {
    dev: *const (),
    card: *const SndsCard,
    mmio_start: *const u8,
    irq: i32,
    num_pipes: i32,
    num_ports: i32,
    pcm_ctx: [SndIntelhad; 4],
}

#[repr(C)]
struct SndsCard {
    private_data: *const (),
    driver: [u8; 16],
    shortname: [u8; 32],
    longname: [u8; 80],
}

#[repr(C)]
struct SndPcmSubstream {
    runtime: *const SndPcmRuntime,
}

#[repr(C)]
struct SndPcmRuntime {
    rate: u32,
    channels: u32,
    format: u32,
    periods: u32,
    period_size: u32,
    buffer_size: u32,
    dma_addr: u32,
    no_period_wakeup: bool,
}

#[repr(C)]
struct SpinLock;

#[repr(C)]
struct Mutex;

#[repr(C)]
struct WorkStruct;

#[repr(C)]
struct PlatformDevice {
    name: [u8; 32],
    dev: Device,
}

#[repr(C)]
struct Device;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
