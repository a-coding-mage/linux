/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* Depends on declarations translated from:
 * cs46xx_dsp_scb_types.h
 * cs46xx_dsp_task_types.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const SYMBOL_CONSTANT: c_int = 0x0;
pub const SYMBOL_SAMPLE: c_int = 0x1;
pub const SYMBOL_PARAMETER: c_int = 0x2;
pub const SYMBOL_CODE: c_int = 0x3;

pub const SEGTYPE_SP_PROGRAM: c_int = 0x00000001;
pub const SEGTYPE_SP_PARAMETER: c_int = 0x00000002;
pub const SEGTYPE_SP_SAMPLE: c_int = 0x00000003;
pub const SEGTYPE_SP_COEFFICIENT: c_int = 0x00000004;

pub const DSP_SPOS_UU: c_ulong = 0x0dead; /* unused */
pub const DSP_SPOS_DC: c_ulong = 0x0bad; /* don't care */
pub const DSP_SPOS_DC_DC: c_ulong = 0x0bad0bad; /* don't care */
pub const DSP_SPOS_UUUU: c_ulong = 0xdeadc0ed; /* unused */
pub const DSP_SPOS_UUHI: c_ulong = 0xdead;
pub const DSP_SPOS_UULO: c_ulong = 0xc0ed;
pub const DSP_SPOS_DCDC: c_ulong = 0x0badf1d0; /* don't care */
pub const DSP_SPOS_DCDCHI: c_ulong = 0x0bad;
pub const DSP_SPOS_DCDCLO: c_ulong = 0xf1d0;

pub const DSP_MAX_TASK_NAME: usize = 60;
pub const DSP_MAX_SYMBOL_NAME: usize = 100;
pub const DSP_MAX_SCB_NAME: usize = 60;
pub const DSP_MAX_SCB_DESC: usize = 200;
pub const DSP_MAX_TASK_DESC: usize = 50;

pub const DSP_MAX_PCM_CHANNELS: usize = 32;
pub const DSP_MAX_SRC_NR: usize = 14;

pub const DSP_PCM_MAIN_CHANNEL: c_int = 1;
pub const DSP_PCM_REAR_CHANNEL: c_int = 2;
pub const DSP_PCM_CENTER_LFE_CHANNEL: c_int = 3;
pub const DSP_PCM_S71_CHANNEL: c_int = 4; /* surround 7.1 */
pub const DSP_IEC958_CHANNEL: c_int = 5;

pub const DSP_SPDIF_STATUS_OUTPUT_ENABLED: c_int = 1;
pub const DSP_SPDIF_STATUS_PLAYBACK_OPEN: c_int = 2;
pub const DSP_SPDIF_STATUS_HW_ENABLED: c_int = 4;
pub const DSP_SPDIF_STATUS_INPUT_CTRL_ENABLED: c_int = 8;

#[repr(C)]
pub struct dsp_symbol_entry {
    pub address: u32,
    pub symbol_name: [c_char; DSP_MAX_SYMBOL_NAME],
    pub symbol_type: c_int,

    /* initialized by driver */
    pub module: *mut dsp_module_desc,
    pub deleted: c_int,
}

#[repr(C)]
pub struct dsp_symbol_desc {
    pub nsymbols: c_int,

    pub symbols: *mut dsp_symbol_entry,

    /* initialized by driver */
    pub highest_frag_index: c_int,
}

#[repr(C)]
pub struct dsp_segment_desc {
    pub segment_type: c_int,
    pub offset: u32,
    pub size: u32,
    pub data: *mut u32,
}

#[repr(C)]
pub struct dsp_module_desc {
    pub module_name: *mut c_char,
    pub symbol_table: dsp_symbol_desc,
    pub nsegments: c_int,
    pub segments: *mut dsp_segment_desc,

    /* initialized by driver */
    pub overlay_begin_address: u32,
    pub load_address: u32,
    pub nfixups: c_int,
}

#[repr(C)]
pub struct dsp_scb_descriptor {
    pub scb_name: [c_char; DSP_MAX_SCB_NAME],
    pub address: u32,
    pub index: c_int,
    pub data: *mut u32,

    pub sub_list_ptr: *mut dsp_scb_descriptor,
    pub next_scb_ptr: *mut dsp_scb_descriptor,
    pub parent_scb_ptr: *mut dsp_scb_descriptor,

    pub task_entry: *mut dsp_symbol_entry,
    pub scb_symbol: *mut dsp_symbol_entry,

    pub proc_info: *mut snd_info_entry,
    pub ref_count: c_int,

    pub volume: [u16; 2],
    pub deleted_updated_volume_set: c_uint,
}

impl dsp_scb_descriptor {
    pub const DELETED_MASK: c_uint = 1 << 0;
    pub const UPDATED_MASK: c_uint = 1 << 1;
    pub const VOLUME_SET_MASK: c_uint = 1 << 2;

    pub fn deleted(&self) -> c_uint {
        (self.deleted_updated_volume_set & Self::DELETED_MASK) >> 0
    }

    pub fn set_deleted(&mut self, value: c_uint) {
        self.deleted_updated_volume_set =
            (self.deleted_updated_volume_set & !Self::DELETED_MASK)
                | ((value & 1) << 0);
    }

    pub fn updated(&self) -> c_uint {
        (self.deleted_updated_volume_set & Self::UPDATED_MASK) >> 1
    }

    pub fn set_updated(&mut self, value: c_uint) {
        self.deleted_updated_volume_set =
            (self.deleted_updated_volume_set & !Self::UPDATED_MASK)
                | ((value & 1) << 1);
    }

    pub fn volume_set(&self) -> c_uint {
        (self.deleted_updated_volume_set & Self::VOLUME_SET_MASK) >> 2
    }

    pub fn set_volume_set(&mut self, value: c_uint) {
        self.deleted_updated_volume_set =
            (self.deleted_updated_volume_set & !Self::VOLUME_SET_MASK)
                | ((value & 1) << 2);
    }
}

#[repr(C)]
pub struct dsp_task_descriptor {
    pub task_name: [c_char; DSP_MAX_TASK_NAME],
    pub size: c_int,
    pub address: u32,
    pub index: c_int,
    pub data: *mut u32,
}

#[repr(C)]
pub struct dsp_pcm_channel_descriptor {
    pub active: c_int,
    pub src_slot: c_int,
    pub pcm_slot: c_int,
    pub sample_rate: u32,
    pub unlinked: u32,
    pub pcm_reader_scb: *mut dsp_scb_descriptor,
    pub src_scb: *mut dsp_scb_descriptor,
    pub mixer_scb: *mut dsp_scb_descriptor,

    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct dsp_spos_instance {
    pub symbol_table: dsp_symbol_desc, /* currently available loaded symbols in SP */

    pub nmodules: c_int,
    pub modules: *mut dsp_module_desc, /* modules loaded into SP */

    pub code: dsp_segment_desc,

    /* Main PCM playback mixer */
    pub master_mix_scb: *mut dsp_scb_descriptor,
    pub dac_volume_right: u16,
    pub dac_volume_left: u16,

    /* Rear/surround PCM playback mixer */
    pub rear_mix_scb: *mut dsp_scb_descriptor,

    /* Center/LFE mixer */
    pub center_lfe_mix_scb: *mut dsp_scb_descriptor,

    pub npcm_channels: c_int,
    pub nsrc_scb: c_int,
    pub pcm_channels: [dsp_pcm_channel_descriptor; DSP_MAX_PCM_CHANNELS],
    pub src_scb_slots: [c_int; DSP_MAX_SRC_NR],

    /* cache this symbols */
    pub null_algorithm: *mut dsp_symbol_entry, /* used by PCMreaderSCB's */
    pub s16_up: *mut dsp_symbol_entry,         /* used by SRCtaskSCB's */

    /* proc fs */
    pub snd_card: *mut snd_card,
    pub proc_dsp_dir: *mut snd_info_entry,

    /* SCB's descriptors */
    pub nscb: c_int,
    pub scb_highest_frag_index: c_int,
    pub scbs: [dsp_scb_descriptor; DSP_MAX_SCB_DESC],
    pub the_null_scb: *mut dsp_scb_descriptor,

    /* Task's descriptors */
    pub ntask: c_int,
    pub tasks: [dsp_task_descriptor; DSP_MAX_TASK_DESC],

    /* SPDIF status */
    pub spdif_status_out: c_int,
    pub spdif_status_in: c_int,
    pub spdif_input_volume_right: u16,
    pub spdif_input_volume_left: u16,
    /* spdif channel status,
       left right and user validity bits */
    pub spdif_csuv_default: c_uint,
    pub spdif_csuv_stream: c_uint,

    /* SPDIF input sample rate converter */
    pub spdif_in_src: *mut dsp_scb_descriptor,
    /* SPDIF input asynch. receiver */
    pub asynch_rx_scb: *mut dsp_scb_descriptor,

    /* Capture record mixer SCB */
    pub record_mixer_scb: *mut dsp_scb_descriptor,

    /* CODEC input SCB */
    pub codec_in_scb: *mut dsp_scb_descriptor,

    /* reference snooper */
    pub ref_snoop_scb: *mut dsp_scb_descriptor,

    /* SPDIF output  PCM reference  */
    pub spdif_pcm_input_scb: *mut dsp_scb_descriptor,

    /* asynch TX task */
    pub asynch_tx_scb: *mut dsp_scb_descriptor,

    /* record sources */
    pub pcm_input: *mut dsp_scb_descriptor,
    pub adc_input: *mut dsp_scb_descriptor,

    pub spdif_in_sample_rate: c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
