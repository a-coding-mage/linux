/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license. When using or
 * redistributing this file, you may do so under either license.
 */

// Dependency declarations from <sound/sof/header.h> are supplied externally.

#[repr(i32)]
pub enum sof_comp_type {
    SOF_COMP_NONE = 0,
    SOF_COMP_HOST,
    SOF_COMP_DAI,
    SOF_COMP_SG_HOST,
    SOF_COMP_SG_DAI,
    SOF_COMP_VOLUME,
    SOF_COMP_MIXER,
    SOF_COMP_MUX,
    SOF_COMP_SRC,
    SOF_COMP_DEPRECATED0,
    SOF_COMP_TONE,
    SOF_COMP_DEPRECATED1,
    SOF_COMP_BUFFER,
    SOF_COMP_EQ_IIR,
    SOF_COMP_EQ_FIR,
    SOF_COMP_KEYWORD_DETECT,
    SOF_COMP_KPB,
    SOF_COMP_SELECTOR,
    SOF_COMP_DEMUX,
    SOF_COMP_ASRC,
    SOF_COMP_DCBLOCK,
    SOF_COMP_SMART_AMP,
    SOF_COMP_MODULE_ADAPTER,
    SOF_COMP_FILEREAD = 10000,
    SOF_COMP_FILEWRITE = 10001,
}

pub const SOF_XRUN_STOP: u32 = 1;
pub const SOF_XRUN_UNDER_ZERO: u32 = 2;
pub const SOF_XRUN_OVER_NULL: u32 = 4;

#[repr(C, packed(4))]
pub struct sof_ipc_comp {
    pub hdr: sof_ipc_cmd_hdr,
    pub id: u32,
    pub type_: u32,
    pub pipeline_id: u32,
    pub core: u32,
    pub ext_data_length: u32,
}

pub const SOF_MEM_CAPS_RAM: u32 = 1 << 0;
pub const SOF_MEM_CAPS_ROM: u32 = 1 << 1;
pub const SOF_MEM_CAPS_EXT: u32 = 1 << 2;
pub const SOF_MEM_CAPS_LP: u32 = 1 << 3;
pub const SOF_MEM_CAPS_HP: u32 = 1 << 4;
pub const SOF_MEM_CAPS_DMA: u32 = 1 << 5;
pub const SOF_MEM_CAPS_CACHE: u32 = 1 << 6;
pub const SOF_MEM_CAPS_EXEC: u32 = 1 << 7;
pub const SOF_MEM_CAPS_L3: u32 = 1 << 8;
pub const SOF_BUF_OVERRUN_PERMITTED: u32 = 1 << 0;
pub const SOF_BUF_UNDERRUN_PERMITTED: u32 = 1 << 1;
pub const SOF_UUID_SIZE: usize = 16;

#[repr(C, packed(4))]
pub struct sof_ipc_buffer { pub comp: sof_ipc_comp, pub size: u32, pub caps: u32, pub flags: u32, pub reserved: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_config {
    pub hdr: sof_ipc_cmd_hdr, pub periods_sink: u32, pub periods_source: u32,
    pub reserved1: u32, pub frame_fmt: u32, pub xrun_action: u32, pub reserved: [u32; 2],
}

#[repr(C, packed(4))]
pub struct sof_ipc_comp_host { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub direction: u32, pub no_irq: u32, pub dmac_config: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_dai { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub direction: u32, pub dai_index: u32, pub type_: u32, pub reserved: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_mixer { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config }

#[repr(i32)]
pub enum sof_volume_ramp { SOF_VOLUME_LINEAR = 0, SOF_VOLUME_LOG, SOF_VOLUME_LINEAR_ZC, SOF_VOLUME_LOG_ZC, SOF_VOLUME_WINDOWS_FADE, SOF_VOLUME_WINDOWS_NO_FADE }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_volume { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub channels: u32, pub min_value: u32, pub max_value: u32, pub ramp: u32, pub initial_ramp: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_src { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub source_rate: u32, pub sink_rate: u32, pub rate_mask: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_asrc { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub source_rate: u32, pub sink_rate: u32, pub asynchronous_mode: u32, pub operation_mode: u32, pub reserved: [u32; 4] }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_mux { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_tone { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub sample_rate: i32, pub frequency: i32, pub amplitude: i32, pub freq_mult: i32, pub ampl_mult: i32, pub length: i32, pub period: i32, pub repeats: i32, pub ramp_step: i32 }

#[repr(i32)]
pub enum sof_ipc_process_type { SOF_PROCESS_NONE = 0, SOF_PROCESS_EQFIR, SOF_PROCESS_EQIIR, SOF_PROCESS_KEYWORD_DETECT, SOF_PROCESS_KPB, SOF_PROCESS_CHAN_SELECTOR, SOF_PROCESS_MUX, SOF_PROCESS_DEMUX, SOF_PROCESS_DCBLOCK, SOF_PROCESS_SMART_AMP }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_process { pub comp: sof_ipc_comp, pub config: sof_ipc_comp_config, pub size: u32, pub type_: u32, pub reserved: [u32; 7], pub data: [u8; 0] }

#[repr(C, packed(4))]
pub struct sof_ipc_free { pub hdr: sof_ipc_cmd_hdr, pub id: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_comp_reply { pub rhdr: sof_ipc_reply, pub id: u32, pub offset: u32 }

#[repr(i32)]
pub enum sof_ipc_pipe_sched_time_domain { SOF_TIME_DOMAIN_DMA = 0, SOF_TIME_DOMAIN_TIMER }

#[repr(C, packed(4))]
pub struct sof_ipc_pipe_new { pub hdr: sof_ipc_cmd_hdr, pub comp_id: u32, pub pipeline_id: u32, pub sched_id: u32, pub core: u32, pub period: u32, pub priority: u32, pub period_mips: u32, pub frames_per_sched: u32, pub xrun_limit_usecs: u32, pub time_domain: u32 }

#[repr(C, packed(4))]
pub struct sof_ipc_pipe_ready { pub hdr: sof_ipc_cmd_hdr, pub comp_id: u32 }
#[repr(C, packed(4))]
pub struct sof_ipc_pipe_free { pub hdr: sof_ipc_cmd_hdr, pub comp_id: u32 }
#[repr(C, packed(4))]
pub struct sof_ipc_pipe_comp_connect { pub hdr: sof_ipc_cmd_hdr, pub source_id: u32, pub sink_id: u32 }

#[repr(i32)]
pub enum sof_event_types { SOF_EVENT_NONE = 0, SOF_KEYWORD_DETECT_DAPM_EVENT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
