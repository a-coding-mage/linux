// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation
//
// Translated from C source. Header dependencies from sound/pcm_params.h,
// sound/sof/ipc4/header.h, sof-audio.h, sof-priv.h, ops.h, ipc4-priv.h,
// ipc4-topology.h, and ipc4-fw-reg.h are expected to be supplied elsewhere.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u8_ = u8;
type s8_ = i8;
type u32_ = u32;
type u64_ = u64;
type size_t = usize;
type snd_pcm_sframes_t = i64;
type snd_pcm_uframes_t = u64;

const U32_MAX: u64 = u32::MAX as u64;
const DELAY_BOUNDARY: u64 = U32_MAX;
const DELAY_MAX: u64 = DELAY_BOUNDARY >> 1;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ETIMEDOUT: c_int = 110;
const EOPNOTSUPP: c_int = 95;
const GFP_KERNEL: u32_ = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_STOP: c_int = 5;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 2;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 0;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 1;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 2;

const SOF_IPC4_PIPE_RUNNING: c_int = 0;
const SOF_IPC4_PIPE_RESET: c_int = 1;
const SOF_IPC4_PIPE_PAUSED: c_int = 2;
const SOF_IPC4_INVALID_STREAM_POSITION: u64_ = !0;
const SOF_IPC4_INVALID_NODE_ID: u32_ = !0;
const SOF_IPC4_CHAIN_DMA_NODE_ID: u32_ = 0xffff_fffe;
const SOF_IPC4_CHAIN_DMA_BUF_SIZE_MS: c_int = 2;
const MSEC_PER_SEC: c_int = 1000;
const SOF_FW_CRASHED: c_int = 0;
const SOF_IPC4_FW_REGS_ABI_VER: u32_ = 0;
const SOF_IPC4_MAX_LLP_GPDMA_READING_SLOTS: c_int = 0;
const SOF_IPC4_MAX_LLP_SNDW_READING_SLOTS: c_int = 0;
const SOF_DAI_INTEL_SSP: c_int = 0;
const SOF_AUDIO_PCM_DRV_NAME: *const c_char = b"sof-audio-component\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct sof_ipc4_timestamp_info {
    pub host_copier: *mut sof_ipc4_copier,
    pub dai_copier: *mut sof_ipc4_copier,
    pub stream_start_offset: u64_,
    pub stream_end_offset: u64_,
    pub llp_offset: u32_,
    pub delay: snd_pcm_sframes_t,
}

#[repr(C)]
pub struct sof_ipc4_pcm_stream_priv {
    pub time_info: *mut sof_ipc4_timestamp_info,
    pub chain_dma_allocated: bool_,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32_,
    pub extension: u32_,
    pub data_size: u32_,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct ipc4_pipeline_set_state_data {
    pub count: c_int,
    pub pipeline_instance_ids: [u32_; 0],
}

#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub private: *mut c_void,
    pub pipeline_list: snd_sof_pcm_stream_pipeline_list,
    pub list: *mut c_void,
}

#[repr(C)]
pub struct snd_sof_pcm_stream_pipeline_list {
    pub pipelines: *mut *mut snd_sof_pipeline,
    pub count: c_int,
}

#[repr(C)]
pub struct snd_sof_pcm {
    pub stream: [snd_sof_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_sof_pipeline {
    pub pipe_widget: *mut snd_sof_widget,
    pub started_count: c_int,
    pub paused_count: c_int,
}

#[repr(C)]
pub struct snd_sof_widget {
    pub private: *mut c_void,
    pub instance_id: u32_,
    pub widget: *mut snd_soc_dapm_widget,
    pub spipe: *mut snd_sof_pipeline,
}

#[repr(C)]
pub struct sof_ipc4_pipeline {
    pub skip_during_fe_trigger: bool_,
    pub priority: s8_,
    pub state: c_int,
    pub use_chain_dma: bool_,
    pub msg: sof_ipc4_msg,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub private: *mut c_void,
    pub dev: *mut c_void,
    pub ipc: *mut c_void,
    pub fw_state: c_int,
    pub fw_info_box: sof_ipc4_fw_info_box,
    pub dai_link_list: list_head,
    pub dai_list: list_head,
}

#[repr(C)]
pub struct sof_ipc4_fw_info_box {
    pub offset: u32_,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub num_playback_streams: u32_,
    pub max_num_pipelines: c_int,
    pub pipeline_state_mutex: c_void,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_sof_dai_link {
    pub list: list_head,
    pub link: *mut snd_soc_dai_link,
    pub num_hw_configs: c_int,
    pub hw_configs: *mut snd_soc_tplg_hw_config,
}

#[repr(C)]
pub struct snd_sof_dai {
    pub list: list_head,
    pub name: *const c_char,
    pub private: *mut c_void,
    pub current_config: c_int,
}

#[repr(C)]
pub struct snd_soc_tplg_hw_config {
    pub fsync_rate: u32_,
    pub tdm_slot_width: u32_,
    pub tdm_slots: u32_,
    pub id: u32_,
}

#[repr(C)]
pub struct snd_pcm_hw_params;

#[repr(C)]
pub struct snd_interval {
    pub min: u32_,
    pub max: u32_,
}

#[repr(C)]
pub struct snd_mask;

#[repr(C)]
pub struct snd_soc_dai;

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub dobj: snd_soc_dobj,
}

#[repr(C)]
pub struct snd_soc_dobj {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_copier {
    pub available_fmt: sof_ipc4_available_audio_format,
    pub data: sof_ipc4_copier_data,
    pub dai_type: c_int,
}

#[repr(C)]
pub struct sof_ipc4_available_audio_format {
    pub input_pin_fmts: *mut sof_ipc4_pin_format,
    pub output_pin_fmts: *mut sof_ipc4_pin_format,
    pub num_input_formats: c_int,
    pub num_output_formats: c_int,
}

#[repr(C)]
pub struct sof_ipc4_pin_format {
    pub audio_fmt: sof_ipc4_audio_format,
}

#[repr(C)]
pub struct sof_ipc4_audio_format {
    pub sampling_frequency: u32_,
    pub fmt_cfg: u32_,
    pub bit_depth: u32_,
}

#[repr(C)]
pub struct sof_ipc4_copier_data {
    pub gtw_cfg: sof_ipc4_gtw_cfg,
    pub out_format: sof_ipc4_audio_format,
}

#[repr(C)]
pub struct sof_ipc4_gtw_cfg {
    pub node_id: u32_,
}

#[repr(C)]
pub struct sof_ipc4_llp_reading_slot {
    pub node_id: u32_,
    pub reading: sof_ipc4_llp_reading,
}

#[repr(C)]
pub struct sof_ipc4_llp_reading {
    pub llp_l: u32_,
    pub llp_u: u32_,
}

#[repr(C)]
pub struct sof_ipc4_pipeline_registers {
    pub stream_start_offset: u64_,
    pub stream_end_offset: u64_,
}

#[repr(C)]
pub struct sof_ipc4_fw_registers {
    pub abi_ver: u32_,
    pub pipeline_regs: [sof_ipc4_pipeline_registers; 0],
    pub llp_gpdma_reading_slots: [sof_ipc4_llp_reading_slot; 0],
    pub llp_sndw_reading_slots: [sof_ipc4_llp_reading_slot; 0],
    pub llp_evad_reading_slot: sof_ipc4_llp_reading_slot,
}

#[repr(C)]
pub struct snd_sof_platform_stream_params;

#[repr(C)]
pub struct sof_ipc_pcm_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_sof_platform_stream_params) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub dai_link_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub pcm_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm) -> c_int>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm)>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_uframes_t) -> c_int>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    pub ipc_first_on_start: bool_,
    pub platform_stop_during_hw_free: bool_,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

unsafe extern "C" {
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn kzalloc(size: size_t, flags: u32_) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_sof_find_spcm_dai(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> *mut snd_sof_pcm;
    fn snd_sof_find_dai(component: *mut snd_soc_component, name: *const c_char) -> *mut snd_sof_dai;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, dir: c_int) -> *mut snd_soc_dapm_widget;
    fn sof_ipc_tx_message_no_reply(ipc: *mut c_void, msg: *mut sof_ipc4_msg, size: u32_) -> c_int;
    fn sof_ipc4_pipeline_state_str(state: u32_) -> *const c_char;
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32_, dest: *mut c_void, size: size_t);
    fn snd_sof_pcm_get_dai_frame_counter(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64_;
    fn snd_sof_pcm_get_host_byte_counter(sdev: *mut snd_sof_dev, component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> u64_;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> u64_;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: u64_) -> u64_;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u64_) -> u64_;
    fn params_rate(params: *mut snd_pcm_hw_params) -> u32_;
    fn params_width(params: *mut snd_pcm_hw_params) -> u32_;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32_;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn sof_ipc4_copier_is_single_bitdepth(sdev: *mut snd_sof_dev, fmts: *mut sof_ipc4_pin_format, count: c_int) -> bool_;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn spcm_dbg(spcm: *mut snd_sof_pcm, stream: c_int, fmt: *const c_char, ...);
    fn spcm_dbg_ratelimited(spcm: *mut snd_sof_pcm, stream: c_int, fmt: *const c_char, ...);
    fn spcm_err(spcm: *mut snd_sof_pcm, stream: c_int, fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn SOF_IPC4_MSG_TYPE_SET(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_MSG_DIR(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_MSG_TARGET(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_GLB_PIPE_STATE_ID(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_GLB_CHAIN_DMA_HOST_ID(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_GLB_CHAIN_DMA_LINK_ID(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_NODE_INDEX(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT(v: u32_) -> u32_ { v }
unsafe fn SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH(v: u32_) -> u32_ { v }

const SOF_IPC4_GLB_SET_PIPELINE_STATE: u32_ = 0;
const SOF_IPC4_MSG_REQUEST: u32_ = 0;
const SOF_IPC4_FW_GEN_MSG: u32_ = 0;
const SOF_IPC4_GLB_PIPE_STATE_EXT_MULTI: u32_ = 0;
const SOF_IPC4_GLB_CHAIN_DMA: u32_ = 0;
const SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_MASK: u32_ = 0;
const SOF_IPC4_GLB_CHAIN_DMA_ENABLE_MASK: u32_ = 0;

unsafe fn sof_ipc4_sps_to_time_info(sps: *mut snd_sof_pcm_stream) -> *mut sof_ipc4_timestamp_info {
    let stream_priv = (*sps).private as *mut sof_ipc4_pcm_stream_priv;
    (*stream_priv).time_info
}

unsafe fn sof_ipc4_set_multi_pipeline_state_debug(
    _sdev: *mut snd_sof_dev,
    buf: *mut c_char,
    size: size_t,
    trigger_list: *mut ipc4_pipeline_set_state_data,
) -> *mut c_char {
    let mut i: c_int = 0;
    let mut offset: c_int = 0;

    while i < (*trigger_list).count {
        offset += snprintf(
            buf.add(offset as usize),
            size.wrapping_sub(offset as usize),
            cstr!(" %d"),
            (*trigger_list).pipeline_instance_ids.as_ptr().add(i as usize).read(),
        );

        if offset as size_t >= size.wrapping_sub(1) {
            *buf.add(size - 1) = 0;
            break;
        }
        i += 1;
    }
    buf
}

unsafe fn sof_ipc4_set_multi_pipeline_state(
    sdev: *mut snd_sof_dev,
    state: u32_,
    trigger_list: *mut ipc4_pipeline_set_state_data,
) -> c_int {
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let primary: u32_;
    let ipc_size: u32_;
    let mut debug_buf = [0 as c_char; 32];

    if (*trigger_list).count == 1 {
        return sof_ipc4_set_pipeline_state(
            sdev,
            (*trigger_list).pipeline_instance_ids.as_ptr().read(),
            state,
        );
    }

    dev_dbg(
        (*sdev).dev,
        cstr!("Set pipelines %s to state %d%s"),
        sof_ipc4_set_multi_pipeline_state_debug(sdev, debug_buf.as_mut_ptr(), debug_buf.len(), trigger_list),
        state,
        sof_ipc4_pipeline_state_str(state),
    );

    primary = state
        | SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_SET_PIPELINE_STATE)
        | SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST)
        | SOF_IPC4_MSG_TARGET(SOF_IPC4_FW_GEN_MSG);
    msg.primary = primary;
    msg.extension = SOF_IPC4_GLB_PIPE_STATE_EXT_MULTI;
    ipc_size = size_of::<u32_>() as u32_ * ((*trigger_list).count as u32_ + 1);
    msg.data_size = ipc_size;
    msg.data_ptr = trigger_list as *mut c_void;

    sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, ipc_size)
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_set_pipeline_state(
    sdev: *mut snd_sof_dev,
    instance_id: u32_,
    state: u32_,
) -> c_int {
    let mut msg: sof_ipc4_msg = core::mem::zeroed();
    let mut primary: u32_ = state;

    dev_dbg(
        (*sdev).dev,
        cstr!("Set pipeline %d to state %d%s"),
        instance_id,
        state,
        sof_ipc4_pipeline_state_str(state),
    );

    primary |= SOF_IPC4_GLB_PIPE_STATE_ID(instance_id);
    primary |= SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_SET_PIPELINE_STATE);
    primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_FW_GEN_MSG);
    msg.primary = primary;

    sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, 0)
}

unsafe fn sof_ipc4_add_pipeline_by_priority(
    trigger_list: *mut ipc4_pipeline_set_state_data,
    pipe_widget: *mut snd_sof_widget,
    pipe_priority: *mut s8_,
    ascend: bool_,
) {
    let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;
    let mut i: c_int = 0;
    let mut j: c_int;

    while i < (*trigger_list).count {
        if ascend && (*pipeline).priority < *pipe_priority.add(i as usize) {
            break;
        } else if !ascend && (*pipeline).priority > *pipe_priority.add(i as usize) {
            break;
        }
        i += 1;
    }

    j = (*trigger_list).count - 1;
    while j >= i {
        let ids = (*trigger_list).pipeline_instance_ids.as_mut_ptr();
        *ids.add((j + 1) as usize) = *ids.add(j as usize);
        *pipe_priority.add((j + 1) as usize) = *pipe_priority.add(j as usize);
        j -= 1;
    }

    *(*trigger_list).pipeline_instance_ids.as_mut_ptr().add(i as usize) = (*pipe_widget).instance_id;
    (*trigger_list).count += 1;
    *pipe_priority.add(i as usize) = (*pipeline).priority;
}

unsafe fn sof_ipc4_add_pipeline_to_trigger_list(
    _sdev: *mut snd_sof_dev,
    state: c_int,
    spipe: *mut snd_sof_pipeline,
    trigger_list: *mut ipc4_pipeline_set_state_data,
    pipe_priority: *mut s8_,
) {
    let pipe_widget = (*spipe).pipe_widget;
    let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

    if (*pipeline).skip_during_fe_trigger && state != SOF_IPC4_PIPE_RESET {
        return;
    }

    match state {
        SOF_IPC4_PIPE_RUNNING => {
            if (*spipe).started_count == (*spipe).paused_count {
                sof_ipc4_add_pipeline_by_priority(trigger_list, pipe_widget, pipe_priority, false);
            }
        }
        SOF_IPC4_PIPE_RESET => {
            if (*spipe).started_count == 0 && (*spipe).paused_count == 0 {
                sof_ipc4_add_pipeline_by_priority(trigger_list, pipe_widget, pipe_priority, true);
            }
        }
        SOF_IPC4_PIPE_PAUSED => {
            if (*spipe).paused_count == (*spipe).started_count - 1 {
                sof_ipc4_add_pipeline_by_priority(trigger_list, pipe_widget, pipe_priority, true);
            }
        }
        _ => {}
    }
}

unsafe fn sof_ipc4_update_pipeline_state(
    _sdev: *mut snd_sof_dev,
    state: c_int,
    cmd: c_int,
    spipe: *mut snd_sof_pipeline,
    trigger_list: *mut ipc4_pipeline_set_state_data,
) {
    let pipe_widget = (*spipe).pipe_widget;
    let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;
    let mut i: c_int = 0;

    if (*pipeline).skip_during_fe_trigger && state != SOF_IPC4_PIPE_RESET {
        return;
    }

    while i < (*trigger_list).count {
        if *(*trigger_list).pipeline_instance_ids.as_ptr().add(i as usize) == (*pipe_widget).instance_id {
            (*pipeline).state = state;
            break;
        }
        i += 1;
    }

    match state {
        SOF_IPC4_PIPE_PAUSED => match cmd {
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => (*spipe).paused_count += 1,
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => (*spipe).started_count -= 1,
            _ => {}
        },
        SOF_IPC4_PIPE_RUNNING => match cmd {
            SNDRV_PCM_TRIGGER_PAUSE_RELEASE => (*spipe).paused_count -= 1,
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => (*spipe).started_count += 1,
            _ => {}
        },
        _ => {}
    }
}

unsafe fn sof_ipc4_chain_dma_trigger(
    sdev: *mut snd_sof_dev,
    spcm: *mut snd_sof_pcm,
    direction: c_int,
    pipeline_list: *mut snd_sof_pcm_stream_pipeline_list,
    state: c_int,
    cmd: c_int,
) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let stream_priv = (*spcm).stream[direction as usize].private as *mut sof_ipc4_pcm_stream_priv;
    let (allocate, enable, set_fifo_size): (bool_, bool_, bool_);
    let mut msg: sof_ipc4_msg = core::mem::zeroed();

    match state {
        SOF_IPC4_PIPE_RUNNING => {
            allocate = true;
            enable = true;
            set_fifo_size = cmd != SNDRV_PCM_TRIGGER_PAUSE_RELEASE;
        }
        SOF_IPC4_PIPE_PAUSED => {
            allocate = true;
            enable = false;
            set_fifo_size = false;
        }
        SOF_IPC4_PIPE_RESET => {
            if !(*stream_priv).chain_dma_allocated {
                return 0;
            }
            allocate = false;
            enable = false;
            set_fifo_size = false;
        }
        _ => {
            spcm_err(spcm, direction, cstr!("Unexpected pipeline state %d\n"), state);
            return -EINVAL;
        }
    }

    msg.primary = SOF_IPC4_MSG_TYPE_SET(SOF_IPC4_GLB_CHAIN_DMA);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MSG_TARGET(SOF_IPC4_FW_GEN_MSG);

    let mut i: c_int = 0;
    while i < (*pipeline_list).count {
        let spipe = *(*pipeline_list).pipelines.add(i as usize);
        let pipe_widget = (*spipe).pipe_widget;
        let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

        if !(*pipeline).use_chain_dma {
            spcm_err(
                spcm,
                direction,
                cstr!("All pipelines in chained DMA path should have use_chain_dma attribute set."),
            );
            return -EINVAL;
        }

        msg.primary |= (*pipeline).msg.primary;
        if set_fifo_size {
            msg.extension |= (*pipeline).msg.extension;
        }
        i += 1;
    }

    if direction == SNDRV_PCM_STREAM_CAPTURE {
        msg.primary = msg.primary.wrapping_add(SOF_IPC4_GLB_CHAIN_DMA_HOST_ID((*ipc4_data).num_playback_streams));
        msg.primary = msg.primary.wrapping_add(SOF_IPC4_GLB_CHAIN_DMA_LINK_ID((*ipc4_data).num_playback_streams));
    }

    if allocate {
        msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_ALLOCATE_MASK;
    }
    if enable {
        msg.primary |= SOF_IPC4_GLB_CHAIN_DMA_ENABLE_MASK;
    }

    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut msg, 0);
    if ret == 0 {
        (*stream_priv).chain_dma_allocated = allocate;
    }
    ret
}

unsafe fn sof_ipc4_trigger_pipelines(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    state: c_int,
    cmd: c_int,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let rtd = snd_soc_substream_to_rtd(substream);
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    spcm_dbg(spcm, (*substream).stream, cstr!("cmd: %d, state: %d\n"), cmd, state);
    let pipeline_list = &mut (*spcm).stream[(*substream).stream as usize].pipeline_list as *mut _;

    if (*pipeline_list).pipelines.is_null() || (*pipeline_list).count == 0 {
        return 0;
    }

    let mut spipe = *(*pipeline_list).pipelines;
    let pipe_widget = (*spipe).pipe_widget;
    let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

    if (*pipeline).use_chain_dma {
        let time_info = sof_ipc4_sps_to_time_info(&mut (*spcm).stream[(*substream).stream as usize]);
        let ret = sof_ipc4_chain_dma_trigger(sdev, spcm, (*substream).stream, pipeline_list, state, cmd);
        if ret != 0 || time_info.is_null() {
            return ret;
        }
        if state == SOF_IPC4_PIPE_PAUSED {
            let pos = snd_sof_pcm_get_dai_frame_counter(sdev, component, substream);
            (*time_info).stream_end_offset = (*time_info).stream_end_offset.wrapping_add(pos);
        } else if state == SOF_IPC4_PIPE_RESET {
            (*time_info).stream_end_offset = 0;
        }
        return 0;
    }

    let trigger_size = size_of::<ipc4_pipeline_set_state_data>()
        + size_of::<u32_>() * (*pipeline_list).count as usize;
    let trigger_list = kzalloc(trigger_size, GFP_KERNEL) as *mut ipc4_pipeline_set_state_data;
    if trigger_list.is_null() {
        return -ENOMEM;
    }
    let pipe_priority = kzalloc((*pipeline_list).count as usize * size_of::<u8_>(), GFP_KERNEL) as *mut s8_;
    if pipe_priority.is_null() {
        kfree(trigger_list as *mut c_void);
        return -ENOMEM;
    }

    if state == SOF_IPC4_PIPE_RUNNING || state == SOF_IPC4_PIPE_RESET {
        let mut i = (*pipeline_list).count - 1;
        while i >= 0 {
            spipe = *(*pipeline_list).pipelines.add(i as usize);
            sof_ipc4_add_pipeline_to_trigger_list(sdev, state, spipe, trigger_list, pipe_priority);
            i -= 1;
        }
    } else {
        let mut i = 0;
        while i < (*pipeline_list).count {
            spipe = *(*pipeline_list).pipelines.add(i as usize);
            sof_ipc4_add_pipeline_to_trigger_list(sdev, state, spipe, trigger_list, pipe_priority);
            i += 1;
        }
    }

    let mut ret: c_int;
    if (*trigger_list).count == 0 {
        ret = 0;
        kfree(trigger_list as *mut c_void);
        kfree(pipe_priority as *mut c_void);
        return ret;
    }

    if !(state == SOF_IPC4_PIPE_RESET || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE) {
        ret = sof_ipc4_set_multi_pipeline_state(sdev, SOF_IPC4_PIPE_PAUSED as u32_, trigger_list);
        if ret < 0 {
            spcm_err(spcm, (*substream).stream, cstr!("failed to pause all pipelines\n"));
            if (*sdev).fw_state != SOF_FW_CRASHED && ret != -ETIMEDOUT {
                kfree(trigger_list as *mut c_void);
                kfree(pipe_priority as *mut c_void);
                return ret;
            }
            ret = 0;
        }

        let mut i = 0;
        while i < (*pipeline_list).count {
            spipe = *(*pipeline_list).pipelines.add(i as usize);
            sof_ipc4_update_pipeline_state(sdev, SOF_IPC4_PIPE_PAUSED, cmd, spipe, trigger_list);
            i += 1;
        }

        if state == SOF_IPC4_PIPE_PAUSED {
            let time_info = sof_ipc4_sps_to_time_info(&mut (*spcm).stream[(*substream).stream as usize]);
            if !time_info.is_null() {
                (*time_info).stream_start_offset = SOF_IPC4_INVALID_STREAM_POSITION;
            }
            kfree(trigger_list as *mut c_void);
            kfree(pipe_priority as *mut c_void);
            return ret;
        }
    }

    ret = sof_ipc4_set_multi_pipeline_state(sdev, state as u32_, trigger_list);
    if ret < 0 {
        spcm_err(
            spcm,
            (*substream).stream,
            cstr!("failed to set final state %d for all pipelines\n"),
            state,
        );
        if (*sdev).fw_state != SOF_FW_CRASHED && ret != -ETIMEDOUT {
            kfree(trigger_list as *mut c_void);
            kfree(pipe_priority as *mut c_void);
            return ret;
        }
        ret = 0;
    }

    let mut i = 0;
    while i < (*pipeline_list).count {
        spipe = *(*pipeline_list).pipelines.add(i as usize);
        sof_ipc4_update_pipeline_state(sdev, state, cmd, spipe, trigger_list);
        i += 1;
    }

    kfree(trigger_list as *mut c_void);
    kfree(pipe_priority as *mut c_void);
    let _ = ipc4_data;
    ret
}

unsafe extern "C" fn sof_ipc4_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let state = match cmd {
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START => SOF_IPC4_PIPE_RUNNING,
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => SOF_IPC4_PIPE_PAUSED,
        _ => {
            dev_err((*component).dev, cstr!("%s: unhandled trigger cmd %d\n"), cstr!("sof_ipc4_pcm_trigger"), cmd);
            return -EINVAL;
        }
    };
    sof_ipc4_trigger_pipelines(component, substream, state, cmd)
}

unsafe extern "C" fn sof_ipc4_pcm_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    sof_ipc4_trigger_pipelines(component, substream, SOF_IPC4_PIPE_RESET, 0)
}

unsafe fn ipc4_ssp_dai_config_pcm_params_match(
    sdev: *mut snd_sof_dev,
    link_name: *const c_char,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut slink: *mut snd_sof_dai_link = ptr::null_mut();
    let mut dai_link_found = false;
    let mut current_config: c_int = -1;
    let mut partial_match = false;

    /* list_for_each_entry(slink, &sdev->dai_link_list, list) */
    let mut pos = (*sdev).dai_link_list.next;
    while !pos.is_null() && pos != &mut (*sdev).dai_link_list {
        slink = pos as *mut snd_sof_dai_link;
        if strcmp((*(*slink).link).name, link_name) == 0 {
            dai_link_found = true;
            break;
        }
        pos = (*pos).next;
    }

    if !dai_link_found {
        return 0;
    }

    let mut i = 0;
    while i < (*slink).num_hw_configs {
        let hw_config = (*slink).hw_configs.add(i as usize);
        if params_rate(params) == u32::from_le((*hw_config).fsync_rate)
            && params_width(params) == u32::from_le((*hw_config).tdm_slot_width)
            && params_channels(params) <= u32::from_le((*hw_config).tdm_slots)
        {
            current_config = u32::from_le((*hw_config).id) as c_int;
            partial_match = false;
            break;
        } else if current_config < 0
            && params_rate(params) == u32::from_le((*hw_config).fsync_rate)
            && params_channels(params) <= u32::from_le((*hw_config).tdm_slots)
        {
            current_config = u32::from_le((*hw_config).id) as c_int;
            partial_match = true;
        }
        i += 1;
    }

    if current_config < 0 {
        dev_err(
            (*sdev).dev,
            cstr!("%s: No suitable hw_config found for %s (num_hw_configs: %d)\n"),
            cstr!("ipc4_ssp_dai_config_pcm_params_match"),
            (*(*slink).link).name,
            (*slink).num_hw_configs,
        );
        return -EINVAL;
    }

    dev_dbg(
        (*sdev).dev,
        cstr!("hw_config for %s: %d (num_hw_configs: %d) with %s match\n"),
        (*(*slink).link).name,
        current_config,
        (*slink).num_hw_configs,
        if partial_match { cstr!("partial") } else { cstr!("full") },
    );

    /* list_for_each_entry(dai, &sdev->dai_list, list) */
    pos = (*sdev).dai_list.next;
    while !pos.is_null() && pos != &mut (*sdev).dai_list {
        let dai = pos as *mut snd_sof_dai;
        if strcmp((*(*slink).link).name, (*dai).name) == 0 {
            (*dai).current_config = current_config;
        }
        pos = (*pos).next;
    }
    0
}

unsafe fn sof_ipc4_pcm_dai_link_fixup_rate(
    sdev: *mut snd_sof_dev,
    params: *mut snd_pcm_hw_params,
    ipc4_copier: *mut sof_ipc4_copier,
) -> c_int {
    let pin_fmts = (*ipc4_copier).available_fmt.input_pin_fmts;
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let num_input_formats = (*ipc4_copier).available_fmt.num_input_formats;
    let fe_rate = params_rate(params);
    let mut fe_be_rate_match = false;
    let mut single_be_rate = true;

    if num_input_formats == 0 {
        return -EINVAL;
    }

    let be_rate = (*pin_fmts).audio_fmt.sampling_frequency;
    let mut i = 0;
    while i < num_input_formats {
        let val = (*pin_fmts.add(i as usize)).audio_fmt.sampling_frequency;
        if val != be_rate {
            single_be_rate = false;
        }
        if val == fe_rate {
            fe_be_rate_match = true;
            break;
        }
        i += 1;
    }

    if !fe_be_rate_match {
        if !single_be_rate {
            dev_err((*sdev).dev, cstr!("Unable to select sampling rate for DAI link\n"));
            return -EINVAL;
        }
        (*rate).min = be_rate;
        (*rate).max = (*rate).min;
    }
    0
}

unsafe fn sof_ipc4_pcm_dai_link_fixup_channels(
    sdev: *mut snd_sof_dev,
    params: *mut snd_pcm_hw_params,
    ipc4_copier: *mut sof_ipc4_copier,
) -> c_int {
    let pin_fmts = (*ipc4_copier).available_fmt.input_pin_fmts;
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let num_input_formats = (*ipc4_copier).available_fmt.num_input_formats;
    let fe_channels = params_channels(params);
    let mut fe_be_match = false;
    let mut single_be_channels = true;

    if num_input_formats == 0 {
        return -EINVAL;
    }

    let be_channels = SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT((*pin_fmts).audio_fmt.fmt_cfg);
    let mut i = 0;
    while i < num_input_formats {
        let val = SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT((*pin_fmts.add(i as usize)).audio_fmt.fmt_cfg);
        if val != be_channels {
            single_be_channels = false;
        }
        if val == fe_channels {
            fe_be_match = true;
            break;
        }
        i += 1;
    }

    if !fe_be_match {
        if !single_be_channels {
            dev_err((*sdev).dev, cstr!("Unable to select channels for DAI link\n"));
            return -EINVAL;
        }
        (*channels).min = be_channels;
        (*channels).max = be_channels;
    }
    0
}

unsafe extern "C" fn sof_ipc4_pcm_dai_link_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let component = snd_soc_rtdcom_lookup(rtd, SOF_AUDIO_PCM_DRV_NAME);
    let dai = snd_sof_find_dai(component, (*(*rtd).dai_link).name);
    let fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let sdev = snd_soc_component_get_drvdata(component);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ipc4_fmt: *mut sof_ipc4_audio_format = ptr::null_mut();
    let mut single_bitdepth = false;
    let mut valid_bits: u32_ = 0;

    if dai.is_null() {
        dev_err((*component).dev, cstr!("%s: No DAI found with name %s\n"), cstr!("sof_ipc4_pcm_dai_link_fixup"), (*(*rtd).dai_link).name);
        return -EINVAL;
    }

    let ipc4_copier = (*dai).private as *mut sof_ipc4_copier;
    if ipc4_copier.is_null() {
        dev_err((*component).dev, cstr!("%s: No private data found for DAI %s\n"), cstr!("sof_ipc4_pcm_dai_link_fixup"), (*(*rtd).dai_link).name);
        return -EINVAL;
    }

    let mut dir = 0;
    while dir < 2 {
        let w = snd_soc_dai_get_widget(cpu_dai, dir);
        if !w.is_null() {
            let available_fmt = &mut (*ipc4_copier).available_fmt as *mut sof_ipc4_available_audio_format;
            let swidget = (*w).dobj.private as *mut snd_sof_widget;
            let pipe_widget = (*(*swidget).spipe).pipe_widget;
            let pipeline = (*pipe_widget).private as *mut sof_ipc4_pipeline;

            if (*pipeline).use_chain_dma {
                return 0;
            }

            if dir == SNDRV_PCM_STREAM_PLAYBACK {
                if sof_ipc4_copier_is_single_bitdepth(sdev, (*available_fmt).output_pin_fmts, (*available_fmt).num_output_formats) {
                    ipc4_fmt = &mut (*(*available_fmt).output_pin_fmts).audio_fmt;
                    single_bitdepth = true;
                }
            } else if sof_ipc4_copier_is_single_bitdepth(sdev, (*available_fmt).input_pin_fmts, (*available_fmt).num_input_formats) {
                ipc4_fmt = &mut (*(*available_fmt).input_pin_fmts).audio_fmt;
                single_bitdepth = true;
            }
        }
        dir += 1;
    }

    let mut ret = sof_ipc4_pcm_dai_link_fixup_rate(sdev, params, ipc4_copier);
    if ret != 0 {
        return ret;
    }
    ret = sof_ipc4_pcm_dai_link_fixup_channels(sdev, params, ipc4_copier);
    if ret != 0 {
        return ret;
    }

    if single_bitdepth {
        snd_mask_none(fmt);
        valid_bits = SOF_IPC4_AUDIO_FORMAT_CFG_V_BIT_DEPTH((*ipc4_fmt).fmt_cfg);
        dev_dbg((*component).dev, cstr!("Set %s to %d bit format\n"), (*dai).name, valid_bits);
    }

    match valid_bits {
        16 => snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S16_LE),
        24 => snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE),
        32 => snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S32_LE),
        _ => {}
    }

    if (*ipc4_copier).dai_type == SOF_DAI_INTEL_SSP {
        return ipc4_ssp_dai_config_pcm_params_match(sdev, (*(*rtd).dai_link).name as *mut c_char, params);
    }
    0
}

unsafe extern "C" fn sof_ipc4_pcm_free(_sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm) {
    let mut stream = 0;
    while stream < 2 {
        let pipeline_list = &mut (*spcm).stream[stream].pipeline_list;
        kfree(pipeline_list.pipelines as *mut c_void);
        pipeline_list.pipelines = ptr::null_mut();

        let stream_priv = (*spcm).stream[stream].private as *mut sof_ipc4_pcm_stream_priv;
        if !stream_priv.is_null() {
            kfree((*stream_priv).time_info as *mut c_void);
        }
        kfree((*spcm).stream[stream].private);
        (*spcm).stream[stream].private = ptr::null_mut();
        stream += 1;
    }
}

unsafe extern "C" fn sof_ipc4_pcm_setup(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let mut support_info = true;
    let mut abi_version: u32_ = 0;
    let abi_offset = 0u32;

    sof_mailbox_read(sdev, (*sdev).fw_info_box.offset + abi_offset, &mut abi_version as *mut _ as *mut c_void, size_of::<u32_>());
    if abi_version < SOF_IPC4_FW_REGS_ABI_VER {
        support_info = false;
    }

    /* For delay reporting the get_host_byte_counter callback is needed. */
    let _ = &mut support_info;

    let mut stream = 0;
    while stream < 2 {
        let pipeline_list = &mut (*spcm).stream[stream].pipeline_list;
        pipeline_list.pipelines = kzalloc(
            size_of::<*mut snd_sof_pipeline>() * (*ipc4_data).max_num_pipelines as usize,
            GFP_KERNEL,
        ) as *mut *mut snd_sof_pipeline;
        if pipeline_list.pipelines.is_null() {
            sof_ipc4_pcm_free(sdev, spcm);
            return -ENOMEM;
        }

        let stream_priv = kzalloc(size_of::<sof_ipc4_pcm_stream_priv>(), GFP_KERNEL) as *mut sof_ipc4_pcm_stream_priv;
        if stream_priv.is_null() {
            sof_ipc4_pcm_free(sdev, spcm);
            return -ENOMEM;
        }
        (*spcm).stream[stream].private = stream_priv as *mut c_void;

        if !support_info || stream as c_int == SNDRV_PCM_STREAM_CAPTURE {
            stream += 1;
            continue;
        }

        let time_info = kzalloc(size_of::<sof_ipc4_timestamp_info>(), GFP_KERNEL) as *mut sof_ipc4_timestamp_info;
        if time_info.is_null() {
            sof_ipc4_pcm_free(sdev, spcm);
            return -ENOMEM;
        }
        (*stream_priv).time_info = time_info;
        stream += 1;
    }
    0
}

unsafe fn sof_ipc4_build_time_info(sdev: *mut snd_sof_dev, sps: *mut snd_sof_pcm_stream) {
    let mut host_copier: *mut sof_ipc4_copier = ptr::null_mut();
    let mut dai_copier: *mut sof_ipc4_copier = ptr::null_mut();
    let mut llp_slot: sof_ipc4_llp_reading_slot = core::mem::zeroed();
    let mut dai: *mut snd_sof_dai;

    /* for_each_dapm_widgets(sps->list, i, widget) requires external list metadata. */
    let _ = &mut host_copier;
    let _ = &mut dai_copier;
    let _ = &mut dai;

    if host_copier.is_null() || dai_copier.is_null() {
        dev_err((*sdev).dev, cstr!("host or dai copier are not found\n"));
        return;
    }

    let time_info = sof_ipc4_sps_to_time_info(sps);
    (*time_info).host_copier = host_copier;
    (*time_info).dai_copier = dai_copier;
    (*time_info).llp_offset = (*sdev).fw_info_box.offset;

    let mut i = 0;
    while i < SOF_IPC4_MAX_LLP_GPDMA_READING_SLOTS {
        sof_mailbox_read(sdev, (*time_info).llp_offset, &mut llp_slot as *mut _ as *mut c_void, size_of::<sof_ipc4_llp_reading_slot>());
        if llp_slot.node_id == (*dai_copier).data.gtw_cfg.node_id {
            break;
        }
        (*time_info).llp_offset += size_of::<sof_ipc4_llp_reading_slot>() as u32_;
        i += 1;
    }
    if i < SOF_IPC4_MAX_LLP_GPDMA_READING_SLOTS {
        return;
    }

    (*time_info).llp_offset = (*sdev).fw_info_box.offset;
    i = 0;
    while i < SOF_IPC4_MAX_LLP_SNDW_READING_SLOTS {
        sof_mailbox_read(sdev, (*time_info).llp_offset, &mut llp_slot as *mut _ as *mut c_void, size_of::<sof_ipc4_llp_reading_slot>());
        if llp_slot.node_id == (*dai_copier).data.gtw_cfg.node_id {
            break;
        }
        (*time_info).llp_offset += size_of::<sof_ipc4_llp_reading_slot>() as u32_;
        i += 1;
    }
    if i < SOF_IPC4_MAX_LLP_SNDW_READING_SLOTS {
        return;
    }

    (*time_info).llp_offset = (*sdev).fw_info_box.offset;
    sof_mailbox_read(sdev, (*time_info).llp_offset, &mut llp_slot as *mut _ as *mut c_void, size_of::<sof_ipc4_llp_reading_slot>());
    if llp_slot.node_id != (*dai_copier).data.gtw_cfg.node_id {
        (*time_info).llp_offset = 0;
    }
}

unsafe extern "C" fn sof_ipc4_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    _platform_params: *mut snd_sof_platform_stream_params,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let rtd = snd_soc_substream_to_rtd(substream);
    let spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EINVAL;
    }

    let time_info = sof_ipc4_sps_to_time_info(&mut (*spcm).stream[(*substream).stream as usize]);
    if time_info.is_null() {
        return 0;
    }

    (*time_info).stream_start_offset = SOF_IPC4_INVALID_STREAM_POSITION;
    (*time_info).llp_offset = 0;
    sof_ipc4_build_time_info(sdev, &mut (*spcm).stream[(*substream).stream as usize]);
    0
}

unsafe fn sof_ipc4_frames_dai_to_host(time_info: *mut sof_ipc4_timestamp_info, mut value: u64_) -> u64_ {
    if (*time_info).dai_copier.is_null() || (*time_info).host_copier.is_null() {
        return value;
    }

    let dai_rate = (*(*time_info).dai_copier).data.out_format.sampling_frequency as u64_;
    let host_rate = (*(*time_info).host_copier).data.out_format.sampling_frequency as u64_;
    if dai_rate == 0 || host_rate == 0 || dai_rate == host_rate {
        return value;
    }

    if value > U32_MAX {
        value /= dai_rate;
        value *= host_rate;
    } else {
        value *= host_rate;
        value /= dai_rate;
    }
    value
}

unsafe fn sof_ipc4_get_stream_start_offset(
    sdev: *mut snd_sof_dev,
    substream: *mut snd_pcm_substream,
    _sps: *mut snd_sof_pcm_stream,
    time_info: *mut sof_ipc4_timestamp_info,
) -> c_int {
    let host_copier = (*time_info).host_copier;
    let dai_copier = (*time_info).dai_copier;
    let mut ppl_reg: sof_ipc4_pipeline_registers = core::mem::zeroed();

    if host_copier.is_null() || dai_copier.is_null() {
        return -EINVAL;
    }

    if (*host_copier).data.gtw_cfg.node_id == SOF_IPC4_INVALID_NODE_ID {
        return -EINVAL;
    } else if (*host_copier).data.gtw_cfg.node_id == SOF_IPC4_CHAIN_DMA_NODE_ID {
        let pre_ms = SOF_IPC4_CHAIN_DMA_BUF_SIZE_MS * 5 / 2 + 1;
        (*time_info).stream_start_offset = (pre_ms * (*(*substream).runtime).rate / MSEC_PER_SEC) as u64_;
    } else {
        let node_index = SOF_IPC4_NODE_INDEX((*host_copier).data.gtw_cfg.node_id);
        let offset = node_index * size_of::<sof_ipc4_pipeline_registers>() as u32_;
        sof_mailbox_read(sdev, (*sdev).fw_info_box.offset + offset, &mut ppl_reg as *mut _ as *mut c_void, size_of::<sof_ipc4_pipeline_registers>());
        if ppl_reg.stream_start_offset == SOF_IPC4_INVALID_STREAM_POSITION {
            return -EINVAL;
        }

        let ch = SOF_IPC4_AUDIO_FORMAT_CFG_CHANNELS_COUNT((*dai_copier).data.out_format.fmt_cfg);
        let dai_sample_size = ((*dai_copier).data.out_format.bit_depth >> 3) * ch;
        (*time_info).stream_start_offset = ppl_reg.stream_start_offset / dai_sample_size as u64_;
        (*time_info).stream_end_offset = ppl_reg.stream_end_offset / dai_sample_size as u64_;
        (*time_info).stream_start_offset = sof_ipc4_frames_dai_to_host(time_info, (*time_info).stream_start_offset);
        (*time_info).stream_end_offset = sof_ipc4_frames_dai_to_host(time_info, (*time_info).stream_end_offset);
    }

    (*time_info).delay = 0;
    0
}

unsafe extern "C" fn sof_ipc4_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    pointer: *mut snd_pcm_uframes_t,
) -> c_int {
    let sdev = snd_soc_component_get_drvdata(component);
    let rtd = snd_soc_substream_to_rtd(substream);
    let spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return -EOPNOTSUPP;
    }

    let sps = &mut (*spcm).stream[(*substream).stream as usize] as *mut snd_sof_pcm_stream;
    let time_info = sof_ipc4_sps_to_time_info(sps);
    if time_info.is_null() {
        return -EOPNOTSUPP;
    }

    if (*time_info).stream_start_offset == SOF_IPC4_INVALID_STREAM_POSITION {
        let ret = sof_ipc4_get_stream_start_offset(sdev, substream, sps, time_info);
        if ret < 0 {
            return -EOPNOTSUPP;
        }
    }

    let mut host_cnt = snd_sof_pcm_get_host_byte_counter(sdev, component, substream);
    let mut host_ptr = host_cnt;
    host_cnt /= frames_to_bytes((*substream).runtime, 1);

    let mut dai_cnt: u64_;
    if (*time_info).llp_offset == 0 {
        dai_cnt = snd_sof_pcm_get_dai_frame_counter(sdev, component, substream);
        if dai_cnt == 0 {
            return -EOPNOTSUPP;
        }
    } else {
        let mut llp: sof_ipc4_llp_reading_slot = core::mem::zeroed();
        sof_mailbox_read(sdev, (*time_info).llp_offset, &mut llp as *mut _ as *mut c_void, size_of::<sof_ipc4_llp_reading_slot>());
        dai_cnt = ((llp.reading.llp_u as u64_) << 32) | llp.reading.llp_l as u64_;
    }

    dai_cnt = sof_ipc4_frames_dai_to_host(time_info, dai_cnt);
    dai_cnt = dai_cnt.wrapping_add((*time_info).stream_end_offset);

    if dai_cnt < (*time_info).stream_start_offset {
        host_cnt = host_cnt.wrapping_add((*time_info).stream_start_offset - dai_cnt);
        dai_cnt = 0;
    } else {
        dai_cnt -= (*time_info).stream_start_offset;
    }

    dai_cnt &= DELAY_BOUNDARY;
    host_cnt &= DELAY_BOUNDARY;

    let (head_cnt, tail_cnt) = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (host_cnt, dai_cnt)
    } else {
        (dai_cnt, host_cnt)
    };

    if head_cnt < tail_cnt {
        (*time_info).delay = (DELAY_BOUNDARY - tail_cnt + head_cnt) as snd_pcm_sframes_t;
    } else {
        (*time_info).delay = (head_cnt - tail_cnt) as snd_pcm_sframes_t;
    }

    if (*time_info).delay as u64_ > DELAY_MAX {
        spcm_dbg_ratelimited(spcm, (*substream).stream, cstr!("inaccurate delay, host %llu dai_cnt %llu"), host_cnt, dai_cnt);
        (*time_info).delay = 0;
    }

    let buffer_bytes = snd_pcm_lib_buffer_bytes(substream);
    host_ptr %= buffer_bytes;
    *pointer = bytes_to_frames((*substream).runtime, host_ptr);
    0
}

unsafe extern "C" fn sof_ipc4_pcm_delay(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_sframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spcm = snd_sof_find_spcm_dai(component, rtd);
    if spcm.is_null() {
        return 0;
    }

    let time_info = sof_ipc4_sps_to_time_info(&mut (*spcm).stream[(*substream).stream as usize]);
    if !time_info.is_null() {
        return (*time_info).delay;
    }
    0
}

#[no_mangle]
pub static ipc4_pcm_ops: sof_ipc_pcm_ops = sof_ipc_pcm_ops {
    hw_params: Some(sof_ipc4_pcm_hw_params),
    trigger: Some(sof_ipc4_pcm_trigger),
    hw_free: Some(sof_ipc4_pcm_hw_free),
    dai_link_fixup: Some(sof_ipc4_pcm_dai_link_fixup),
    pcm_setup: Some(sof_ipc4_pcm_setup),
    pcm_free: Some(sof_ipc4_pcm_free),
    pointer: Some(sof_ipc4_pcm_pointer),
    delay: Some(sof_ipc4_pcm_delay),
    ipc_first_on_start: true,
    platform_stop_during_hw_free: true,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
