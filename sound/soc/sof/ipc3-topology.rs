// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Intel Corporation
//
// Rust source-level translation of soc/sof/ipc3-topology.c.
// C include dependencies are intentionally left as external Rust dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut};

type u8 = ::core::ffi::c_uchar;
type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;
type size_t = usize;

const SOF_IPC3_TPLG_ABI_SIZE: u32 = 3;
const INTEL_ALH_DAI_INDEX_BASE: u32 = 2;

// Full volume for default values.
const VOL_ZERO_DB: u32 = 1u32 << VOLUME_FWL;

#[repr(C)]
struct sof_widget_data {
    ctrl_type: c_int,
    ipc_cmd: c_int,
    pdata: *mut c_void,
    pdata_size: size_t,
    control: *mut snd_sof_control,
}

#[repr(C)]
struct sof_process_types {
    name: *const c_char,
    type_: sof_ipc_process_type,
    comp_type: sof_comp_type,
}

static sof_process: [sof_process_types; 9] = [
    sof_process_types { name: c"EQFIR".as_ptr(), type_: SOF_PROCESS_EQFIR, comp_type: SOF_COMP_EQ_FIR },
    sof_process_types { name: c"EQIIR".as_ptr(), type_: SOF_PROCESS_EQIIR, comp_type: SOF_COMP_EQ_IIR },
    sof_process_types { name: c"KEYWORD_DETECT".as_ptr(), type_: SOF_PROCESS_KEYWORD_DETECT, comp_type: SOF_COMP_KEYWORD_DETECT },
    sof_process_types { name: c"KPB".as_ptr(), type_: SOF_PROCESS_KPB, comp_type: SOF_COMP_KPB },
    sof_process_types { name: c"CHAN_SELECTOR".as_ptr(), type_: SOF_PROCESS_CHAN_SELECTOR, comp_type: SOF_COMP_SELECTOR },
    sof_process_types { name: c"MUX".as_ptr(), type_: SOF_PROCESS_MUX, comp_type: SOF_COMP_MUX },
    sof_process_types { name: c"DEMUX".as_ptr(), type_: SOF_PROCESS_DEMUX, comp_type: SOF_COMP_DEMUX },
    sof_process_types { name: c"DCBLOCK".as_ptr(), type_: SOF_PROCESS_DCBLOCK, comp_type: SOF_COMP_DCBLOCK },
    sof_process_types { name: c"SMART_AMP".as_ptr(), type_: SOF_PROCESS_SMART_AMP, comp_type: SOF_COMP_SMART_AMP },
];

unsafe fn find_process(name: *const c_char) -> sof_ipc_process_type {
    for p in &sof_process {
        if strcmp(name, p.name) == 0 {
            return p.type_;
        }
    }
    SOF_PROCESS_NONE
}

unsafe extern "C" fn get_token_process_type(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int {
    let val = (object as *mut u8).add(offset as usize) as *mut u32;
    *val = find_process(elem as *const c_char) as u32;
    0
}

macro_rules! off {
    ($ty:ty, $field:tt) => {
        core::mem::offset_of!($ty, $field) as u32
    };
}

macro_rules! tok {
    ($id:expr, $kind:expr, $get:expr, $off:expr) => {
        sof_topology_token { token: $id, tuple_type: $kind, get_token: $get, offset: $off }
    };
}

/* Buffers */
static buffer_tokens: [sof_topology_token; 3] = [
    tok!(SOF_TKN_BUF_SIZE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_buffer, size)),
    tok!(SOF_TKN_BUF_CAPS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_buffer, caps)),
    tok!(SOF_TKN_BUF_FLAGS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_buffer, flags)),
];

/* DAI */
static dai_tokens: [sof_topology_token; 3] = [
    tok!(SOF_TKN_DAI_TYPE, SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token_dai_type, off!(sof_ipc_comp_dai, type_)),
    tok!(SOF_TKN_DAI_INDEX, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_dai, dai_index)),
    tok!(SOF_TKN_DAI_DIRECTION, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_dai, direction)),
];

/* BE DAI link */
static dai_link_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_DAI_TYPE, SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token_dai_type, off!(sof_ipc_dai_config, type_)),
    tok!(SOF_TKN_DAI_INDEX, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_config, dai_index)),
];

/* scheduling */
static sched_tokens: [sof_topology_token; 6] = [
    tok!(SOF_TKN_SCHED_PERIOD, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, period)),
    tok!(SOF_TKN_SCHED_PRIORITY, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, priority)),
    tok!(SOF_TKN_SCHED_MIPS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, period_mips)),
    tok!(SOF_TKN_SCHED_CORE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, core)),
    tok!(SOF_TKN_SCHED_FRAMES, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, frames_per_sched)),
    tok!(SOF_TKN_SCHED_TIME_DOMAIN, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_pipe_new, time_domain)),
];

static pipeline_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_SCHED_DYNAMIC_PIPELINE, SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token_u16, off!(snd_sof_widget, dynamic_pipeline_widget)),
];

/* volume */
static volume_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_VOLUME_RAMP_STEP_TYPE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_volume, ramp)),
    tok!(SOF_TKN_VOLUME_RAMP_STEP_MS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_volume, initial_ramp)),
];

/* SRC */
static src_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_SRC_RATE_IN, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_src, source_rate)),
    tok!(SOF_TKN_SRC_RATE_OUT, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_src, sink_rate)),
];

/* ASRC */
static asrc_tokens: [sof_topology_token; 4] = [
    tok!(SOF_TKN_ASRC_RATE_IN, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_asrc, source_rate)),
    tok!(SOF_TKN_ASRC_RATE_OUT, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_asrc, sink_rate)),
    tok!(SOF_TKN_ASRC_ASYNCHRONOUS_MODE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_asrc, asynchronous_mode)),
    tok!(SOF_TKN_ASRC_OPERATION_MODE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_asrc, operation_mode)),
];

/* EFFECT */
static process_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_PROCESS_TYPE, SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token_process_type, off!(sof_ipc_comp_process, type_)),
];

/* PCM */
static pcm_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_PCM_DMAC_CONFIG, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_host, dmac_config)),
];

/* Generic components */
static comp_tokens: [sof_topology_token; 3] = [
    tok!(SOF_TKN_COMP_PERIOD_SINK_COUNT, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_config, periods_sink)),
    tok!(SOF_TKN_COMP_PERIOD_SOURCE_COUNT, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp_config, periods_source)),
    tok!(SOF_TKN_COMP_FORMAT, SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token_comp_format, off!(sof_ipc_comp_config, frame_fmt)),
];

static ssp_tokens: [sof_topology_token; 7] = [
    tok!(SOF_TKN_INTEL_SSP_CLKS_CONTROL, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_ssp_params, clks_control)),
    tok!(SOF_TKN_INTEL_SSP_MCLK_ID, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_ssp_params, mclk_id)),
    tok!(SOF_TKN_INTEL_SSP_SAMPLE_BITS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_ssp_params, sample_valid_bits)),
    tok!(SOF_TKN_INTEL_SSP_FRAME_PULSE_WIDTH, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_ssp_params, frame_pulse_width)),
    tok!(SOF_TKN_INTEL_SSP_QUIRKS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_ssp_params, quirks)),
    tok!(SOF_TKN_INTEL_SSP_TDM_PADDING_PER_SLOT, SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token_u16, off!(sof_ipc_dai_ssp_params, tdm_per_slot_padding_flag)),
    tok!(SOF_TKN_INTEL_SSP_BCLK_DELAY, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_ssp_params, bclk_delay)),
];

static alh_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_INTEL_ALH_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_alh_params, rate)),
    tok!(SOF_TKN_INTEL_ALH_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_alh_params, channels)),
];

static dmic_tokens: [sof_topology_token; 9] = [
    tok!(SOF_TKN_INTEL_DMIC_DRIVER_VERSION, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, driver_ipc_version)),
    tok!(SOF_TKN_INTEL_DMIC_CLK_MIN, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, pdmclk_min)),
    tok!(SOF_TKN_INTEL_DMIC_CLK_MAX, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, pdmclk_max)),
    tok!(SOF_TKN_INTEL_DMIC_SAMPLE_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, fifo_fs)),
    tok!(SOF_TKN_INTEL_DMIC_DUTY_MIN, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_params, duty_min)),
    tok!(SOF_TKN_INTEL_DMIC_DUTY_MAX, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_params, duty_max)),
    tok!(SOF_TKN_INTEL_DMIC_NUM_PDM_ACTIVE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, num_pdm_active)),
    tok!(SOF_TKN_INTEL_DMIC_FIFO_WORD_LENGTH, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_params, fifo_bits)),
    tok!(SOF_TKN_INTEL_DMIC_UNMUTE_RAMP_TIME_MS, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_dmic_params, unmute_ramp_time)),
];

/* DMIC PDM Tokens: SOF_TKN_INTEL_DMIC_PDM_CTRL_ID must be first. */
static dmic_pdm_tokens: [sof_topology_token; 7] = [
    tok!(SOF_TKN_INTEL_DMIC_PDM_CTRL_ID, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, id)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_MIC_A_Enable, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, enable_mic_a)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_MIC_B_Enable, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, enable_mic_b)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_POLARITY_A, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, polarity_mic_a)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_POLARITY_B, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, polarity_mic_b)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_CLK_EDGE, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, clk_edge)),
    tok!(SOF_TKN_INTEL_DMIC_PDM_SKEW, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_dmic_pdm_ctrl, skew)),
];

static hda_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_INTEL_HDA_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_hda_params, rate)),
    tok!(SOF_TKN_INTEL_HDA_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_hda_params, channels)),
];

static esai_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_IMX_ESAI_MCLK_ID, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_esai_params, mclk_id)),
];
static sai_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_IMX_SAI_MCLK_ID, SND_SOC_TPLG_TUPLE_TYPE_SHORT, get_token_u16, off!(sof_ipc_dai_sai_params, mclk_id)),
];
static afe_tokens: [sof_topology_token; 3] = [
    tok!(SOF_TKN_MEDIATEK_AFE_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_mtk_afe_params, rate)),
    tok!(SOF_TKN_MEDIATEK_AFE_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_mtk_afe_params, channels)),
    tok!(SOF_TKN_MEDIATEK_AFE_FORMAT, SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token_comp_format, off!(sof_ipc_dai_mtk_afe_params, format)),
];
static acpdmic_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_AMD_ACPDMIC_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acpdmic_params, pdm_rate)),
    tok!(SOF_TKN_AMD_ACPDMIC_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acpdmic_params, pdm_ch)),
];
/* ACPI2S tokens fill sof_ipc_dai_acp_params; SOF_DAI_AMD_I2S reuses this tuple group. */
static acpi2s_tokens: [sof_topology_token; 4] = [
    tok!(SOF_TKN_AMD_ACPI2S_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_params, fsync_rate)),
    tok!(SOF_TKN_AMD_ACPI2S_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_params, tdm_slots)),
    tok!(SOF_TKN_AMD_ACPI2S_TDM_MODE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_params, tdm_mode)),
    tok!(SOF_TKN_AMD_ACPI2S_FORMAT, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_params, format)),
];
static micfil_pdm_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_IMX_MICFIL_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_micfil_params, pdm_rate)),
    tok!(SOF_TKN_IMX_MICFIL_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_micfil_params, pdm_ch)),
];
static acp_sdw_tokens: [sof_topology_token; 2] = [
    tok!(SOF_TKN_AMD_ACP_SDW_RATE, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_sdw_params, rate)),
    tok!(SOF_TKN_AMD_ACP_SDW_CH, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_dai_acp_sdw_params, channels)),
];
static core_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_COMP_CORE_ID, SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token_u32, off!(sof_ipc_comp, core)),
];
static comp_ext_tokens: [sof_topology_token; 1] = [
    tok!(SOF_TKN_COMP_UUID, SND_SOC_TPLG_TUPLE_TYPE_UUID, get_token_uuid, off!(snd_sof_widget, uuid)),
];

static ipc3_token_list: [sof_token_info; SOF_TOKEN_COUNT as usize] = sof_token_info_designated_array! {
    SOF_PCM_TOKENS => ("PCM tokens", pcm_tokens),
    SOF_PIPELINE_TOKENS => ("Pipeline tokens", pipeline_tokens),
    SOF_SCHED_TOKENS => ("Scheduler tokens", sched_tokens),
    SOF_COMP_TOKENS => ("Comp tokens", comp_tokens),
    SOF_CORE_TOKENS => ("Core tokens", core_tokens),
    SOF_COMP_EXT_TOKENS => ("AFE tokens", comp_ext_tokens),
    SOF_BUFFER_TOKENS => ("Buffer tokens", buffer_tokens),
    SOF_VOLUME_TOKENS => ("Volume tokens", volume_tokens),
    SOF_SRC_TOKENS => ("SRC tokens", src_tokens),
    SOF_ASRC_TOKENS => ("ASRC tokens", asrc_tokens),
    SOF_PROCESS_TOKENS => ("Process tokens", process_tokens),
    SOF_DAI_TOKENS => ("DAI tokens", dai_tokens),
    SOF_DAI_LINK_TOKENS => ("DAI link tokens", dai_link_tokens),
    SOF_HDA_TOKENS => ("HDA tokens", hda_tokens),
    SOF_SSP_TOKENS => ("SSP tokens", ssp_tokens),
    SOF_ALH_TOKENS => ("ALH tokens", alh_tokens),
    SOF_DMIC_TOKENS => ("DMIC tokens", dmic_tokens),
    SOF_DMIC_PDM_TOKENS => ("DMIC PDM tokens", dmic_pdm_tokens),
    SOF_ESAI_TOKENS => ("ESAI tokens", esai_tokens),
    SOF_SAI_TOKENS => ("SAI tokens", sai_tokens),
    SOF_AFE_TOKENS => ("AFE tokens", afe_tokens),
    SOF_ACPDMIC_TOKENS => ("ACPDMIC tokens", acpdmic_tokens),
    SOF_ACPI2S_TOKENS => ("ACPI2S tokens", acpi2s_tokens),
    SOF_MICFIL_TOKENS => ("MICFIL PDM tokens", micfil_pdm_tokens),
    SOF_ACP_SDW_TOKENS => ("ACP_SDW tokens", acp_sdw_tokens),
};

unsafe fn sof_comp_alloc(swidget: *mut snd_sof_widget, ipc_size: *mut size_t, index: c_int) -> *mut c_void {
    let mut total_size = *ipc_size;
    let ext_size = size_of_val(&(*swidget).uuid);

    if !guid_is_null(&(*swidget).uuid) {
        total_size += ext_size;
    }

    let comp = kzalloc(total_size, GFP_KERNEL) as *mut sof_ipc_comp;
    if comp.is_null() {
        return null_mut();
    }

    (*comp).hdr.size = total_size as u32;
    (*comp).hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_COMP_NEW;
    (*comp).id = (*swidget).comp_id;
    (*comp).pipeline_id = index;
    (*comp).core = (*swidget).core;

    if total_size > *ipc_size {
        copy_nonoverlapping(&(*swidget).uuid as *const _ as *const u8, (comp as *mut u8).add(*ipc_size), ext_size);
        (*comp).ext_data_length = ext_size as u32;
    }

    *ipc_size = total_size;
    comp as *mut c_void
}

unsafe fn sof_dbg_comp_config(scomp: *mut snd_soc_component, config: *mut sof_ipc_comp_config) {
    dev_dbg((*scomp).dev, c" config: periods snk %d src %d fmt %d\n".as_ptr(),
            (*config).periods_sink, (*config).periods_source, (*config).frame_fmt);
}

macro_rules! setup_comp_common {
    ($fn_name:ident, $ty:ty, $comp_kind:expr, $token_set:expr, $label:literal) => {
        unsafe fn $fn_name(swidget: *mut snd_sof_widget) -> c_int {
            let scomp = (*swidget).scomp;
            let mut ipc_size = size_of::<$ty>();
            let obj = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut $ty;
            if obj.is_null() { return -ENOMEM; }
            (*swidget).private = obj as *mut c_void;
            (*obj).comp.type_ = $comp_kind;
            (*obj).config.hdr.size = size_of_val(&(*obj).config) as u32;
            let ret = sof_update_ipc_object(scomp, &mut (*obj).config as *mut _ as *mut c_void,
                                            SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples,
                                            size_of_val(&(*obj).config), 1);
            if ret < 0 {
                kfree((*swidget).private);
                (*swidget).private = null_mut();
                return ret;
            }
            dev_dbg((*scomp).dev, concat!($label, " %s\n\0").as_ptr() as *const c_char, (*(*swidget).widget).name);
            sof_dbg_comp_config(scomp, &mut (*obj).config);
            0
        }
    }
}

unsafe fn sof_ipc3_widget_setup_comp_host(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let mut ipc_size = size_of::<sof_ipc_comp_host>();
    let host = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut sof_ipc_comp_host;
    if host.is_null() { return -ENOMEM; }
    (*swidget).private = host as *mut c_void;
    (*host).comp.type_ = SOF_COMP_HOST;
    (*host).config.hdr.size = size_of_val(&(*host).config) as u32;
    (*host).direction = if (*swidget).id == snd_soc_dapm_aif_out { SOF_IPC_STREAM_CAPTURE } else { SOF_IPC_STREAM_PLAYBACK };
    let mut ret = sof_update_ipc_object(scomp, host as *mut c_void, SOF_PCM_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_comp_host>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    ret = sof_update_ipc_object(scomp, &mut (*host).config as *mut _ as *mut c_void, SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of_val(&(*host).config), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    dev_dbg((*scomp).dev, c"loaded host %s\n".as_ptr(), (*(*swidget).widget).name);
    sof_dbg_comp_config(scomp, &mut (*host).config);
    0
}

unsafe fn sof_ipc3_widget_free_comp(swidget: *mut snd_sof_widget) {
    kfree((*swidget).private);
}

unsafe fn sof_ipc3_widget_setup_comp_tone(swidget: *mut snd_sof_widget) -> c_int {
    let ret = setup_typed_comp::<sof_ipc_comp_tone>(swidget, SOF_COMP_TONE);
    if ret == 0 {
        let tone = (*swidget).private as *mut sof_ipc_comp_tone;
        dev_dbg((*(*swidget).scomp).dev, c"tone %s: frequency %d amplitude %d\n".as_ptr(),
                (*(*swidget).widget).name, (*tone).frequency, (*tone).amplitude);
        sof_dbg_comp_config((*swidget).scomp, &mut (*tone).config);
    }
    ret
}

unsafe fn setup_typed_comp<T: HasCompConfig>(swidget: *mut snd_sof_widget, comp_type: sof_comp_type) -> c_int {
    let scomp = (*swidget).scomp;
    let mut ipc_size = size_of::<T>();
    let ptr = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut T;
    if ptr.is_null() { return -ENOMEM; }
    (*swidget).private = ptr as *mut c_void;
    (*ptr).comp_mut().type_ = comp_type;
    (*ptr).config_mut().hdr.size = size_of_val((*ptr).config_ref()) as u32;
    let ret = sof_update_ipc_object(scomp, (*ptr).config_mut() as *mut _ as *mut c_void, SOF_COMP_TOKENS,
                                    (*swidget).tuples, (*swidget).num_tuples, size_of_val((*ptr).config_ref()), 1);
    if ret < 0 {
        kfree((*swidget).private);
        (*swidget).private = null_mut();
        return ret;
    }
    0
}

unsafe fn sof_ipc3_widget_setup_comp_mixer(swidget: *mut snd_sof_widget) -> c_int {
    let ret = setup_typed_comp::<sof_ipc_comp_mixer>(swidget, SOF_COMP_MIXER);
    if ret == 0 {
        dev_dbg((*(*swidget).scomp).dev, c"loaded mixer %s\n".as_ptr(), (*(*swidget).widget).name);
        sof_dbg_comp_config((*swidget).scomp, &mut (*((*swidget).private as *mut sof_ipc_comp_mixer)).config);
    }
    ret
}

unsafe fn sof_ipc3_widget_setup_comp_pipeline(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let spipe = (*swidget).spipe;
    let pipeline = kzalloc(size_of::<sof_ipc_pipe_new>(), GFP_KERNEL) as *mut sof_ipc_pipe_new;
    if pipeline.is_null() { return -ENOMEM; }
    (*pipeline).hdr.size = size_of::<sof_ipc_pipe_new>() as u32;
    (*pipeline).hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_PIPE_NEW;
    (*pipeline).pipeline_id = (*swidget).pipeline_id;
    (*pipeline).comp_id = (*swidget).comp_id;
    (*swidget).private = pipeline as *mut c_void;
    let comp_swidget = snd_sof_find_swidget(scomp, (*(*swidget).widget).sname);
    if comp_swidget.is_null() {
        dev_err((*scomp).dev, c"scheduler %s refers to non existent widget %s\n".as_ptr(), (*(*swidget).widget).name, (*(*swidget).widget).sname);
        kfree((*swidget).private); (*swidget).private = null_mut(); return -EINVAL;
    }
    (*pipeline).sched_id = (*comp_swidget).comp_id;
    let mut ret = sof_update_ipc_object(scomp, pipeline as *mut c_void, SOF_SCHED_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_pipe_new>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    ret = sof_update_ipc_object(scomp, swidget as *mut c_void, SOF_PIPELINE_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<snd_sof_widget>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    if sof_debug_check_flag(SOF_DBG_DISABLE_MULTICORE) {
        (*pipeline).core = SOF_DSP_PRIMARY_CORE;
    } else if (*pipeline).core > (*sdev).num_cores - 1 {
        dev_info((*scomp).dev, c"out of range core id for %s, moving it %d -> %d\n".as_ptr(), (*(*swidget).widget).name, (*pipeline).core, SOF_DSP_PRIMARY_CORE);
        (*pipeline).core = SOF_DSP_PRIMARY_CORE;
    }
    if sof_debug_check_flag(SOF_DBG_DYNAMIC_PIPELINES_OVERRIDE) {
        (*swidget).dynamic_pipeline_widget = sof_debug_check_flag(SOF_DBG_DYNAMIC_PIPELINES_ENABLE);
    }
    (*swidget).core = (*pipeline).core;
    (*spipe).core_mask |= 1u32 << (*pipeline).core;
    0
}

unsafe fn sof_ipc3_widget_setup_comp_buffer(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let buffer = kzalloc(size_of::<sof_ipc_buffer>(), GFP_KERNEL) as *mut sof_ipc_buffer;
    if buffer.is_null() { return -ENOMEM; }
    (*swidget).private = buffer as *mut c_void;
    (*buffer).comp.hdr.size = size_of::<sof_ipc_buffer>() as u32;
    (*buffer).comp.hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_BUFFER_NEW;
    (*buffer).comp.id = (*swidget).comp_id;
    (*buffer).comp.type_ = SOF_COMP_BUFFER;
    (*buffer).comp.pipeline_id = (*swidget).pipeline_id;
    (*buffer).comp.core = (*swidget).core;
    let ret = sof_update_ipc_object(scomp, buffer as *mut c_void, SOF_BUFFER_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_buffer>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    dev_dbg((*scomp).dev, c"buffer %s: size %d caps 0x%x\n".as_ptr(), (*(*swidget).widget).name, (*buffer).size, (*buffer).caps);
    0
}

unsafe fn sof_ipc3_widget_setup_comp_src(swidget: *mut snd_sof_widget) -> c_int { setup_rate_comp::<sof_ipc_comp_src>(swidget, SOF_COMP_SRC, SOF_SRC_TOKENS) }
unsafe fn sof_ipc3_widget_setup_comp_asrc(swidget: *mut snd_sof_widget) -> c_int { setup_rate_comp::<sof_ipc_comp_asrc>(swidget, SOF_COMP_ASRC, SOF_ASRC_TOKENS) }

unsafe fn setup_rate_comp<T: HasCompConfig>(swidget: *mut snd_sof_widget, comp_type: sof_comp_type, token_set: sof_tokens) -> c_int {
    let scomp = (*swidget).scomp;
    let mut ipc_size = size_of::<T>();
    let ptr = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut T;
    if ptr.is_null() { return -ENOMEM; }
    (*swidget).private = ptr as *mut c_void;
    (*ptr).comp_mut().type_ = comp_type;
    (*ptr).config_mut().hdr.size = size_of_val((*ptr).config_ref()) as u32;
    let mut ret = sof_update_ipc_object(scomp, ptr as *mut c_void, token_set, (*swidget).tuples, (*swidget).num_tuples, size_of::<T>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    ret = sof_update_ipc_object(scomp, (*ptr).config_mut() as *mut _ as *mut c_void, SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of_val((*ptr).config_ref()), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    sof_dbg_comp_config(scomp, (*ptr).config_mut());
    0
}

/* Mux topology */
unsafe fn sof_ipc3_widget_setup_comp_mux(swidget: *mut snd_sof_widget) -> c_int {
    let ret = setup_typed_comp::<sof_ipc_comp_mux>(swidget, SOF_COMP_MUX);
    if ret == 0 {
        dev_dbg((*(*swidget).scomp).dev, c"loaded mux %s\n".as_ptr(), (*(*swidget).widget).name);
        sof_dbg_comp_config((*swidget).scomp, &mut (*((*swidget).private as *mut sof_ipc_comp_mux)).config);
    }
    ret
}

/* PGA Topology */
unsafe fn sof_ipc3_widget_setup_comp_pga(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut ipc_size = size_of::<sof_ipc_comp_volume>();
    let volume = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut sof_ipc_comp_volume;
    if volume.is_null() { return -ENOMEM; }
    (*swidget).private = volume as *mut c_void;
    (*volume).comp.type_ = SOF_COMP_VOLUME;
    (*volume).config.hdr.size = size_of_val(&(*volume).config) as u32;
    let mut ret = sof_update_ipc_object(scomp, volume as *mut c_void, SOF_VOLUME_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_comp_volume>(), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    ret = sof_update_ipc_object(scomp, &mut (*volume).config as *mut _ as *mut c_void, SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of_val(&(*volume).config), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); return ret; }
    list_for_each_snd_sof_control(&mut (*sdev).kcontrol_list, |scontrol| {
        if (*scontrol).comp_id == (*swidget).comp_id && !(*scontrol).volume_table.is_null() {
            let min_step = (*scontrol).min_volume_step;
            let max_step = (*scontrol).max_volume_step;
            (*volume).min_value = *(*scontrol).volume_table.add(min_step as usize);
            (*volume).max_value = *(*scontrol).volume_table.add(max_step as usize);
            (*volume).channels = (*scontrol).num_channels;
            false
        } else { true }
    });
    0
}

unsafe fn sof_get_control_data(scomp: *mut snd_soc_component, widget: *mut snd_soc_dapm_widget, wdata: *mut sof_widget_data, size: *mut size_t) -> c_int {
    *size = 0;
    for i in 0..(*widget).num_kcontrols {
        let kc = (*widget).kcontrol_news.add(i as usize);
        match *(*widget).dobj.widget.kcontrol_type.add(i as usize) {
            SND_SOC_TPLG_TYPE_MIXER => {
                let sm = (*kc).private_value as *mut soc_mixer_control;
                (*wdata.add(i as usize)).control = (*sm).dobj.private as *mut snd_sof_control;
            }
            SND_SOC_TPLG_TYPE_BYTES => {
                let sbe = (*kc).private_value as *mut soc_bytes_ext;
                (*wdata.add(i as usize)).control = (*sbe).dobj.private as *mut snd_sof_control;
            }
            SND_SOC_TPLG_TYPE_ENUM => {
                let se = (*kc).private_value as *mut soc_enum;
                (*wdata.add(i as usize)).control = (*se).dobj.private as *mut snd_sof_control;
            }
            _ => {
                dev_err((*scomp).dev, c"Unknown kcontrol type %u in widget %s\n".as_ptr(), *(*widget).dobj.widget.kcontrol_type.add(i as usize), (*widget).name);
                return -EINVAL;
            }
        }
        let wd = wdata.add(i as usize);
        if (*wd).control.is_null() {
            dev_err((*scomp).dev, c"No scontrol for widget %s\n".as_ptr(), (*widget).name);
            return -EINVAL;
        }
        let cdata = (*(*wd).control).ipc_control_data as *mut sof_ipc_ctrl_data;
        if *(*widget).dobj.widget.kcontrol_type.add(i as usize) == SND_SOC_TPLG_TYPE_BYTES {
            if (*(*cdata).data).magic != SOF_ABI_MAGIC { return -EINVAL; }
            (*wd).pdata = (*(*cdata).data).data.as_mut_ptr() as *mut c_void;
            (*wd).pdata_size = (*(*cdata).data).size as usize;
        } else {
            (*wd).pdata = (*cdata).chanv.as_mut_ptr() as *mut c_void;
            (*wd).pdata_size = (*(*wd).control).size - size_of::<sof_ipc_ctrl_data>();
        }
        *size += (*wd).pdata_size;
        match (*cdata).cmd {
            SOF_CTRL_CMD_VOLUME | SOF_CTRL_CMD_ENUM | SOF_CTRL_CMD_SWITCH => {
                (*wd).ipc_cmd = SOF_IPC_COMP_SET_VALUE;
                (*wd).ctrl_type = SOF_CTRL_TYPE_VALUE_CHAN_SET;
            }
            SOF_CTRL_CMD_BINARY => {
                (*wd).ipc_cmd = SOF_IPC_COMP_SET_DATA;
                (*wd).ctrl_type = SOF_CTRL_TYPE_DATA_SET;
            }
            _ => {}
        }
    }
    0
}

unsafe fn sof_process_load(scomp: *mut snd_soc_component, swidget: *mut snd_sof_widget, type_: c_int) -> c_int {
    let widget = (*swidget).widget;
    let mut wdata: *mut sof_widget_data = null_mut();
    let mut ipc_data_size: size_t = 0;
    let mut ret: c_int;
    if (*widget).num_kcontrols != 0 {
        wdata = kzalloc(size_of::<sof_widget_data>() * (*widget).num_kcontrols as usize, GFP_KERNEL) as *mut sof_widget_data;
        if wdata.is_null() { return -ENOMEM; }
        ret = sof_get_control_data(scomp, widget, wdata, &mut ipc_data_size);
        if ret < 0 { kfree(wdata as *mut c_void); return ret; }
    }
    let mut ipc_size = size_of::<sof_ipc_comp_process>() + ipc_data_size;
    if ipc_size > SOF_IPC_MSG_MAX_SIZE as usize {
        ipc_size -= ipc_data_size;
        ipc_data_size = 0;
    }
    let process = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut sof_ipc_comp_process;
    if process.is_null() { kfree(wdata as *mut c_void); return -ENOMEM; }
    (*swidget).private = process as *mut c_void;
    (*process).comp.type_ = type_;
    (*process).config.hdr.size = size_of_val(&(*process).config) as u32;
    ret = sof_update_ipc_object(scomp, &mut (*process).config as *mut _ as *mut c_void, SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of_val(&(*process).config), 1);
    if ret < 0 { kfree((*swidget).private); (*swidget).private = null_mut(); kfree(wdata as *mut c_void); return ret; }
    if ipc_data_size != 0 {
        let mut offset = 0usize;
        for i in 0..(*widget).num_kcontrols {
            let wd = wdata.add(i as usize);
            if (*wd).pdata_size == 0 { continue; }
            copy_nonoverlapping((*wd).pdata as *const u8, (*process).data.as_mut_ptr().add(offset), (*wd).pdata_size);
            offset += (*wd).pdata_size;
        }
    }
    (*process).size = ipc_data_size as u32;
    kfree(wdata as *mut c_void);
    0
}

unsafe fn find_process_comp_type(type_: sof_ipc_process_type) -> sof_comp_type {
    for p in &sof_process {
        if p.type_ == type_ {
            return p.comp_type;
        }
    }
    SOF_COMP_NONE
}

unsafe fn sof_widget_update_ipc_comp_process(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let mut config: sof_ipc_comp_process = zeroed();
    config.comp.core = (*swidget).core;
    let ret = sof_update_ipc_object(scomp, &mut config as *mut _ as *mut c_void, SOF_PROCESS_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_comp_process>(), 1);
    if ret < 0 { return ret; }
    sof_process_load(scomp, swidget, find_process_comp_type(config.type_) as c_int)
}

unsafe fn sof_dai_set_format(hw_config: *mut snd_soc_tplg_hw_config, config: *mut sof_ipc_dai_config) {
    (*config).format &= !SOF_DAI_FMT_CLOCK_PROVIDER_MASK;
    if (*hw_config).bclk_provider == SND_SOC_TPLG_BCLK_CP {
        (*config).format |= if (*hw_config).fsync_provider == SND_SOC_TPLG_FSYNC_CP { SOF_DAI_FMT_CBP_CFP } else { SOF_DAI_FMT_CBP_CFC };
    } else {
        (*config).format |= if (*hw_config).fsync_provider == SND_SOC_TPLG_FSYNC_CP { SOF_DAI_FMT_CBC_CFP } else { SOF_DAI_FMT_CBC_CFC };
    }
    (*config).format &= !SOF_DAI_FMT_INV_MASK;
    if (*hw_config).invert_bclk {
        (*config).format |= if (*hw_config).invert_fsync { SOF_DAI_FMT_IB_IF } else { SOF_DAI_FMT_IB_NF };
    } else {
        (*config).format |= if (*hw_config).invert_fsync { SOF_DAI_FMT_NB_IF } else { SOF_DAI_FMT_NB_NF };
    }
}

macro_rules! simple_link_load {
    ($name:ident, $field:ident, $tokens:expr, $pre:expr) => {
        unsafe fn $name(scomp: *mut snd_soc_component, slink: *mut snd_sof_dai_link, config: *mut sof_ipc_dai_config, dai: *mut snd_sof_dai) -> c_int {
            let private = (*dai).private as *mut sof_dai_private_data;
            let size = size_of::<sof_ipc_dai_config>() as u32;
            if $pre {
                sof_dai_set_format((*slink).hw_configs, config);
            }
            (*config).hdr.size = size;
            let ret = sof_update_ipc_object(scomp, &mut (*config).$field as *mut _ as *mut c_void, $tokens,
                                            (*slink).tuples, (*slink).num_tuples, size as usize, (*slink).num_hw_configs);
            if ret < 0 { return ret; }
            (*dai).number_configs = 1;
            (*dai).current_config = 0;
            (*private).dai_config = kmemdup(config as *const c_void, size as usize, GFP_KERNEL) as *mut sof_ipc_dai_config;
            if (*private).dai_config.is_null() { return -ENOMEM; }
            0
        }
    }
}

simple_link_load!(sof_link_hda_load, hda, SOF_HDA_TOKENS, false);
simple_link_load!(sof_link_sai_load, sai, SOF_SAI_TOKENS, true);
simple_link_load!(sof_link_esai_load, esai, SOF_ESAI_TOKENS, true);
simple_link_load!(sof_link_micfil_load, micfil, SOF_MICFIL_TOKENS, true);
simple_link_load!(sof_link_acp_dmic_load, acpdmic, SOF_ACPDMIC_TOKENS, true);
simple_link_load!(sof_link_acp_bt_load, acpbt, SOF_ACPI2S_TOKENS, true);
simple_link_load!(sof_link_acp_sp_load, acpsp, SOF_ACPI2S_TOKENS, true);
simple_link_load!(sof_link_acp_hs_load, acphs, SOF_ACPI2S_TOKENS, true);
simple_link_load!(sof_link_acp_sdw_load, acp_sdw, SOF_ACP_SDW_TOKENS, false);
simple_link_load!(sof_link_acp_i2s_load, acp_i2s, SOF_ACPI2S_TOKENS, true);
simple_link_load!(sof_link_afe_load, afe, SOF_AFE_TOKENS, false);
simple_link_load!(sof_link_alh_load, alh, SOF_ALH_TOKENS, false);

unsafe fn sof_link_ssp_load(scomp: *mut snd_soc_component, slink: *mut snd_sof_dai_link, config: *mut sof_ipc_dai_config, dai: *mut snd_sof_dai) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let hw_config = (*slink).hw_configs;
    let private = (*dai).private as *mut sof_dai_private_data;
    let size = size_of::<sof_ipc_dai_config>() as u32;
    let mut current_config = 0;
    let ret = sof_update_ipc_object(scomp, &mut (*config).ssp as *mut _ as *mut c_void, SOF_SSP_TOKENS, (*slink).tuples, (*slink).num_tuples, size as usize, (*slink).num_hw_configs);
    if ret < 0 { return ret; }
    for i in 0..(*slink).num_hw_configs {
        let cfg = config.add(i as usize);
        let hw = hw_config.add(i as usize);
        if le32_to_cpu((*hw).id) == (*slink).default_hw_cfg_id { current_config = i; }
        sof_dai_set_format(hw, cfg);
        (*cfg).hdr.size = size;
        if (*sdev).mclk_id_override {
            (*cfg).ssp.mclk_id = (*sdev).mclk_id_quirk;
        }
        (*cfg).ssp.mclk_rate = le32_to_cpu((*hw).mclk_rate);
        (*cfg).ssp.bclk_rate = le32_to_cpu((*hw).bclk_rate);
        (*cfg).ssp.fsync_rate = le32_to_cpu((*hw).fsync_rate);
        (*cfg).ssp.tdm_slots = le32_to_cpu((*hw).tdm_slots);
        (*cfg).ssp.tdm_slot_width = le32_to_cpu((*hw).tdm_slot_width);
        (*cfg).ssp.mclk_direction = (*hw).mclk_direction;
        (*cfg).ssp.rx_slots = le32_to_cpu((*hw).rx_slots);
        (*cfg).ssp.tx_slots = le32_to_cpu((*hw).tx_slots);
        if (*cfg).ssp.fsync_rate < 8000 || (*cfg).ssp.fsync_rate > 192000 { return -EINVAL; }
        if (*cfg).ssp.tdm_slots < 1 || (*cfg).ssp.tdm_slots > 8 { return -EINVAL; }
    }
    (*dai).number_configs = (*slink).num_hw_configs;
    (*dai).current_config = current_config;
    (*private).dai_config = kmemdup(config as *const c_void, size as usize * (*slink).num_hw_configs as usize, GFP_KERNEL) as *mut sof_ipc_dai_config;
    if (*private).dai_config.is_null() { return -ENOMEM; }
    0
}

unsafe fn sof_link_dmic_load(scomp: *mut snd_soc_component, slink: *mut snd_sof_dai_link, config: *mut sof_ipc_dai_config, dai: *mut snd_sof_dai) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let private = (*dai).private as *mut sof_dai_private_data;
    let v = &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version;
    let size = size_of::<sof_ipc_dai_config>();
    memset(&mut (*config).dmic as *mut _ as *mut c_void, 0, size_of_val(&(*config).dmic));
    let mut ret = sof_update_ipc_object(scomp, &mut (*config).dmic as *mut _ as *mut c_void, SOF_DMIC_TOKENS, (*slink).tuples, (*slink).num_tuples, size, (*slink).num_hw_configs);
    if ret < 0 { return ret; }
    ret = sof_update_ipc_object(scomp, &mut (*config).dmic.pdm[0] as *mut _ as *mut c_void, SOF_DMIC_PDM_TOKENS, (*slink).tuples, (*slink).num_tuples, size_of::<sof_ipc_dai_dmic_pdm_ctrl>(), (*config).dmic.num_pdm_active);
    if ret < 0 { return ret; }
    (*config).hdr.size = size as u32;
    if SOF_ABI_VER((*v).major, (*v).minor, (*v).micro) < SOF_ABI_VER(3, 0, 1) {
        (*config).dmic.fifo_bits_b = (*config).dmic.fifo_bits;
    }
    (*dai).number_configs = 1;
    (*dai).current_config = 0;
    (*private).dai_config = kmemdup(config as *const c_void, size, GFP_KERNEL) as *mut sof_ipc_dai_config;
    if (*private).dai_config.is_null() { return -ENOMEM; }
    0
}

unsafe fn sof_ipc3_widget_setup_comp_dai(swidget: *mut snd_sof_widget) -> c_int {
    let scomp = (*swidget).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let dai = (*swidget).private as *mut snd_sof_dai;
    let private = kzalloc(size_of::<sof_dai_private_data>(), GFP_KERNEL) as *mut sof_dai_private_data;
    if private.is_null() { return -ENOMEM; }
    (*dai).private = private as *mut c_void;
    let mut ipc_size = size_of::<sof_ipc_comp_dai>();
    (*private).comp_dai = sof_comp_alloc(swidget, &mut ipc_size, (*swidget).pipeline_id) as *mut sof_ipc_comp_dai;
    if (*private).comp_dai.is_null() { kfree(private as *mut c_void); (*dai).private = null_mut(); return -ENOMEM; }
    let comp_dai = (*private).comp_dai;
    (*comp_dai).comp.type_ = SOF_COMP_DAI;
    (*comp_dai).config.hdr.size = size_of_val(&(*comp_dai).config) as u32;
    let mut ret = sof_update_ipc_object(scomp, comp_dai as *mut c_void, SOF_DAI_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of::<sof_ipc_comp_dai>(), 1);
    if ret < 0 { kfree(comp_dai as *mut c_void); kfree(private as *mut c_void); (*dai).private = null_mut(); return ret; }
    ret = sof_update_ipc_object(scomp, &mut (*comp_dai).config as *mut _ as *mut c_void, SOF_COMP_TOKENS, (*swidget).tuples, (*swidget).num_tuples, size_of_val(&(*comp_dai).config), 1);
    if ret < 0 { kfree(comp_dai as *mut c_void); kfree(private as *mut c_void); (*dai).private = null_mut(); return ret; }
    if (*comp_dai).type_ == SOF_DAI_INTEL_ALH {
        if (*comp_dai).dai_index < INTEL_ALH_DAI_INDEX_BASE { kfree(comp_dai as *mut c_void); kfree(private as *mut c_void); (*dai).private = null_mut(); return -EINVAL; }
        (*comp_dai).dai_index -= INTEL_ALH_DAI_INDEX_BASE;
    }
    list_for_each_snd_sof_dai_link(&mut (*sdev).dai_link_list, |slink| {
        if strcmp((*(*slink).link).name, (*dai).name) != 0 { return true; }
        let config = kzalloc(size_of::<sof_ipc_dai_config>() * (*slink).num_hw_configs as usize, GFP_KERNEL) as *mut sof_ipc_dai_config;
        if config.is_null() { ret = -ENOMEM; return false; }
        let mut common_config: sof_ipc_dai_config = zeroed();
        ret = sof_update_ipc_object(scomp, &mut common_config as *mut _ as *mut c_void, SOF_DAI_LINK_TOKENS, (*slink).tuples, (*slink).num_tuples, size_of::<sof_ipc_dai_config>(), 1);
        if ret < 0 { kfree(config as *mut c_void); return false; }
        for i in 0..(*slink).num_hw_configs {
            (*config.add(i as usize)).hdr.cmd = SOF_IPC_GLB_DAI_MSG | SOF_IPC_DAI_CONFIG;
            (*config.add(i as usize)).format = le32_to_cpu((*(*slink).hw_configs.add(i as usize)).fmt);
            (*config.add(i as usize)).type_ = common_config.type_;
            (*config.add(i as usize)).dai_index = (*comp_dai).dai_index;
        }
        ret = match common_config.type_ {
            SOF_DAI_INTEL_SSP => sof_link_ssp_load(scomp, slink, config, dai),
            SOF_DAI_INTEL_DMIC => sof_link_dmic_load(scomp, slink, config, dai),
            SOF_DAI_INTEL_HDA => sof_link_hda_load(scomp, slink, config, dai),
            SOF_DAI_INTEL_ALH => sof_link_alh_load(scomp, slink, config, dai),
            SOF_DAI_IMX_SAI => sof_link_sai_load(scomp, slink, config, dai),
            SOF_DAI_IMX_ESAI => sof_link_esai_load(scomp, slink, config, dai),
            SOF_DAI_IMX_MICFIL => sof_link_micfil_load(scomp, slink, config, dai),
            SOF_DAI_AMD_BT => sof_link_acp_bt_load(scomp, slink, config, dai),
            SOF_DAI_AMD_SP | SOF_DAI_AMD_SP_VIRTUAL => sof_link_acp_sp_load(scomp, slink, config, dai),
            SOF_DAI_AMD_HS | SOF_DAI_AMD_HS_VIRTUAL => sof_link_acp_hs_load(scomp, slink, config, dai),
            SOF_DAI_AMD_DMIC => sof_link_acp_dmic_load(scomp, slink, config, dai),
            SOF_DAI_MEDIATEK_AFE => sof_link_afe_load(scomp, slink, config, dai),
            SOF_DAI_AMD_SDW => sof_link_acp_sdw_load(scomp, slink, config, dai),
            SOF_DAI_AMD_I2S => sof_link_acp_i2s_load(scomp, slink, config, dai),
            _ => 0,
        };
        kfree(config as *mut c_void);
        ret >= 0
    });
    if ret < 0 { kfree(comp_dai as *mut c_void); kfree(private as *mut c_void); (*dai).private = null_mut(); return ret; }
    0
}

unsafe fn sof_ipc3_widget_free_comp_dai(swidget: *mut snd_sof_widget) {
    match (*swidget).id {
        snd_soc_dapm_dai_in | snd_soc_dapm_dai_out => {
            let dai = (*swidget).private as *mut snd_sof_dai;
            if dai.is_null() { return; }
            let dai_data = (*dai).private as *mut sof_dai_private_data;
            if !dai_data.is_null() {
                kfree((*dai_data).comp_dai as *mut c_void);
                kfree((*dai_data).dai_config as *mut c_void);
                kfree(dai_data as *mut c_void);
            }
            kfree(dai as *mut c_void);
        }
        _ => {}
    }
}

unsafe fn sof_ipc3_route_setup(sdev: *mut snd_sof_dev, sroute: *mut snd_sof_route) -> c_int {
    let mut connect: sof_ipc_pipe_comp_connect = zeroed();
    connect.hdr.size = size_of::<sof_ipc_pipe_comp_connect>() as u32;
    connect.hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_COMP_CONNECT;
    connect.source_id = (*(*sroute).src_widget).comp_id;
    connect.sink_id = (*(*sroute).sink_widget).comp_id;
    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut connect as *mut _ as *mut c_void, size_of::<sof_ipc_pipe_comp_connect>());
    if ret < 0 {
        dev_err((*sdev).dev, c"%s: route %s -> %s failed\n".as_ptr(), c"sof_ipc3_route_setup".as_ptr(), (*(*(*sroute).src_widget).widget).name, (*(*(*sroute).sink_widget).widget).name);
    }
    ret
}

unsafe fn sof_ipc3_control_load_bytes(sdev: *mut snd_sof_dev, scontrol: *mut snd_sof_control) -> c_int {
    if (*scontrol).max_size < size_of::<sof_ipc_ctrl_data>() + size_of::<sof_abi_hdr>() { return -EINVAL; }
    if (*scontrol).priv_size > (*scontrol).max_size - size_of::<sof_ipc_ctrl_data>() { return -EINVAL; }
    (*scontrol).ipc_control_data = kzalloc((*scontrol).max_size, GFP_KERNEL);
    if (*scontrol).ipc_control_data.is_null() { return -ENOMEM; }
    (*scontrol).size = size_of::<sof_ipc_ctrl_data>() + (*scontrol).priv_size;
    let cdata = (*scontrol).ipc_control_data as *mut sof_ipc_ctrl_data;
    (*cdata).cmd = SOF_CTRL_CMD_BINARY;
    (*cdata).index = (*scontrol).index;
    if (*scontrol).priv_size > 0 {
        copy_nonoverlapping((*scontrol).priv as *const u8, (*cdata).data as *mut u8, (*scontrol).priv_size);
        kfree((*scontrol).priv); (*scontrol).priv = null_mut();
        if (*(*cdata).data).magic != SOF_ABI_MAGIC { kfree((*scontrol).ipc_control_data); (*scontrol).ipc_control_data = null_mut(); return -EINVAL; }
        if SOF_ABI_VERSION_INCOMPATIBLE(SOF_ABI_VERSION, (*(*cdata).data).abi) { kfree((*scontrol).ipc_control_data); (*scontrol).ipc_control_data = null_mut(); return -EINVAL; }
        if (*(*cdata).data).size as usize + size_of::<sof_abi_hdr>() != (*scontrol).priv_size { kfree((*scontrol).ipc_control_data); (*scontrol).ipc_control_data = null_mut(); return -EINVAL; }
    }
    0
}

unsafe fn sof_ipc3_control_load_volume(_sdev: *mut snd_sof_dev, scontrol: *mut snd_sof_control) -> c_int {
    (*scontrol).size = struct_size_sof_ipc_ctrl_data_chanv((*scontrol).num_channels);
    (*scontrol).ipc_control_data = kzalloc((*scontrol).size, GFP_KERNEL);
    if (*scontrol).ipc_control_data.is_null() { return -ENOMEM; }
    let cdata = (*scontrol).ipc_control_data as *mut sof_ipc_ctrl_data;
    (*cdata).index = (*scontrol).index;
    if (*scontrol).max == 1 { (*cdata).cmd = SOF_CTRL_CMD_SWITCH; return 0; }
    (*cdata).cmd = SOF_CTRL_CMD_VOLUME;
    for i in 0..(*scontrol).num_channels {
        (*cdata).chanv[i as usize].channel = i;
        (*cdata).chanv[i as usize].value = VOL_ZERO_DB;
    }
    0
}

unsafe fn sof_ipc3_control_load_enum(_sdev: *mut snd_sof_dev, scontrol: *mut snd_sof_control) -> c_int {
    (*scontrol).size = struct_size_sof_ipc_ctrl_data_chanv((*scontrol).num_channels);
    (*scontrol).ipc_control_data = kzalloc((*scontrol).size, GFP_KERNEL);
    if (*scontrol).ipc_control_data.is_null() { return -ENOMEM; }
    let cdata = (*scontrol).ipc_control_data as *mut sof_ipc_ctrl_data;
    (*cdata).index = (*scontrol).index;
    (*cdata).cmd = SOF_CTRL_CMD_ENUM;
    0
}

unsafe fn sof_ipc3_control_setup(sdev: *mut snd_sof_dev, scontrol: *mut snd_sof_control) -> c_int {
    match (*scontrol).info_type {
        SND_SOC_TPLG_CTL_VOLSW | SND_SOC_TPLG_CTL_VOLSW_SX | SND_SOC_TPLG_CTL_VOLSW_XR_SX => sof_ipc3_control_load_volume(sdev, scontrol),
        SND_SOC_TPLG_CTL_BYTES => sof_ipc3_control_load_bytes(sdev, scontrol),
        SND_SOC_TPLG_CTL_ENUM | SND_SOC_TPLG_CTL_ENUM_VALUE => sof_ipc3_control_load_enum(sdev, scontrol),
        _ => 0,
    }
}

unsafe fn sof_ipc3_control_free(sdev: *mut snd_sof_dev, scontrol: *mut snd_sof_control) -> c_int {
    let mut fcomp: sof_ipc_free = zeroed();
    fcomp.hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_COMP_FREE;
    fcomp.hdr.size = size_of::<sof_ipc_free>() as u32;
    fcomp.id = (*scontrol).comp_id;
    sof_ipc_tx_message_no_reply((*sdev).ipc, &mut fcomp as *mut _ as *mut c_void, size_of::<sof_ipc_free>())
}

unsafe fn sof_ipc3_keyword_detect_pcm_params(swidget: *mut snd_sof_widget, dir: c_int) -> c_int {
    let scomp = (*swidget).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let spcm = snd_sof_find_spcm_name(scomp, (*(*swidget).widget).sname);
    if spcm.is_null() { return -EINVAL; }
    let params = &mut (*spcm).params[dir as usize] as *mut snd_pcm_hw_params;
    let mut pcm: sof_ipc_pcm_params = zeroed();
    pcm.hdr.size = size_of::<sof_ipc_pcm_params>() as u32;
    pcm.hdr.cmd = SOF_IPC_GLB_STREAM_MSG | SOF_IPC_STREAM_PCM_PARAMS;
    pcm.comp_id = (*swidget).comp_id;
    pcm.params.hdr.size = size_of_val(&pcm.params) as u32;
    pcm.params.direction = dir;
    pcm.params.sample_valid_bytes = params_width(params) >> 3;
    pcm.params.buffer_fmt = SOF_IPC_BUFFER_INTERLEAVED;
    pcm.params.rate = params_rate(params);
    pcm.params.channels = params_channels(params);
    pcm.params.host_period_bytes = params_period_bytes(params);
    pcm.params.frame_fmt = match params_format(params) {
        SNDRV_PCM_FORMAT_S16 => SOF_IPC_FRAME_S16_LE,
        SNDRV_PCM_FORMAT_S24 => SOF_IPC_FRAME_S24_4LE,
        SNDRV_PCM_FORMAT_S32 => SOF_IPC_FRAME_S32_LE,
        _ => return -EINVAL,
    };
    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut pcm as *mut _ as *mut c_void, size_of::<sof_ipc_pcm_params>());
    if ret < 0 { dev_err((*scomp).dev, c"%s: PCM params failed for %s\n".as_ptr(), c"sof_ipc3_keyword_detect_pcm_params".as_ptr(), (*(*swidget).widget).name); }
    ret
}

unsafe fn sof_ipc3_keyword_detect_trigger(swidget: *mut snd_sof_widget, cmd: c_int) -> c_int {
    let scomp = (*swidget).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mut stream: sof_ipc_stream = zeroed();
    stream.hdr.size = size_of::<sof_ipc_stream>() as u32;
    stream.hdr.cmd = SOF_IPC_GLB_STREAM_MSG | cmd as u32;
    stream.comp_id = (*swidget).comp_id;
    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut stream as *mut _ as *mut c_void, size_of::<sof_ipc_stream>());
    if ret < 0 { dev_err((*scomp).dev, c"%s: Failed to trigger %s\n".as_ptr(), c"sof_ipc3_keyword_detect_trigger".as_ptr(), (*(*swidget).widget).name); }
    ret
}

unsafe extern "C" fn sof_ipc3_keyword_dapm_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let swidget = (*w).dobj.private as *mut snd_sof_widget;
    if swidget.is_null() { return 0; }
    let scomp = (*swidget).scomp;
    let stream = SNDRV_PCM_STREAM_CAPTURE;
    let spcm = snd_sof_find_spcm_name(scomp, (*(*swidget).widget).sname);
    if spcm.is_null() { return -EINVAL; }
    let mut ret = 0;
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*spcm).stream[stream as usize].suspend_ignored { return 0; }
            ret = sof_ipc3_keyword_detect_pcm_params(swidget, stream);
            if ret >= 0 { ret = sof_ipc3_keyword_detect_trigger(swidget, SOF_IPC_STREAM_TRIG_START); }
        }
        SND_SOC_DAPM_POST_PMD => {
            if (*spcm).stream[stream as usize].suspend_ignored { return 0; }
            ret = sof_ipc3_keyword_detect_trigger(swidget, SOF_IPC_STREAM_TRIG_STOP);
            if ret >= 0 { ret = sof_ipc3_keyword_detect_trigger(swidget, SOF_IPC_STREAM_PCM_FREE); }
        }
        _ => {}
    }
    ret
}

static sof_kwd_events: [snd_soc_tplg_widget_events; 1] = [
    snd_soc_tplg_widget_events { event_type: SOF_KEYWORD_DETECT_DAPM_EVENT, event_handler: Some(sof_ipc3_keyword_dapm_event) },
];

unsafe fn sof_ipc3_widget_bind_event(scomp: *mut snd_soc_component, swidget: *mut snd_sof_widget, event_type: u16) -> c_int {
    match event_type as u32 {
        SOF_KEYWORD_DETECT_DAPM_EVENT => {
            if (*swidget).id == snd_soc_dapm_effect {
                let ipc_comp = (*swidget).private as *mut sof_ipc_comp;
                if ipc_comp.is_null() || (*ipc_comp).type_ == SOF_COMP_KEYWORD_DETECT {
                    return snd_soc_tplg_widget_bind_event((*swidget).widget, sof_kwd_events.as_ptr(), sof_kwd_events.len() as u32, event_type);
                }
            }
        }
        _ => {}
    }
    dev_err((*scomp).dev, c"Invalid event type %d for widget %s\n".as_ptr(), event_type as c_int, (*(*swidget).widget).name);
    -EINVAL
}

unsafe fn sof_ipc3_complete_pipeline(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int {
    let mut ready: sof_ipc_pipe_ready = zeroed();
    ready.hdr.size = size_of::<sof_ipc_pipe_ready>() as u32;
    ready.hdr.cmd = SOF_IPC_GLB_TPLG_MSG | SOF_IPC_TPLG_PIPE_COMPLETE;
    ready.comp_id = (*swidget).comp_id;
    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut ready as *mut _ as *mut c_void, size_of::<sof_ipc_pipe_ready>());
    if ret < 0 { return ret; }
    1
}

unsafe fn sof_ipc3_widget_free(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int {
    let mut ipc_free: sof_ipc_free = zeroed();
    ipc_free.hdr.size = size_of::<sof_ipc_free>() as u32;
    ipc_free.hdr.cmd = SOF_IPC_GLB_TPLG_MSG;
    ipc_free.id = (*swidget).comp_id;
    if (*swidget).private.is_null() { return 0; }
    ipc_free.hdr.cmd |= match (*swidget).id {
        snd_soc_dapm_scheduler => SOF_IPC_TPLG_PIPE_FREE,
        snd_soc_dapm_buffer => SOF_IPC_TPLG_BUFFER_FREE,
        _ => SOF_IPC_TPLG_COMP_FREE,
    };
    let ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut ipc_free as *mut _ as *mut c_void, size_of::<sof_ipc_free>());
    if ret < 0 { dev_err((*sdev).dev, c"failed to free widget %s\n".as_ptr(), (*(*swidget).widget).name); }
    ret
}

unsafe fn sof_ipc3_dai_config(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget, flags: c_uint, data: *mut snd_sof_dai_config_data) -> c_int {
    let v = &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version;
    let dai = (*swidget).private as *mut snd_sof_dai;
    if dai.is_null() || (*dai).private.is_null() { return -EINVAL; }
    let private = (*dai).private as *mut sof_dai_private_data;
    if (*private).dai_config.is_null() { return -EINVAL; }
    let config = (*private).dai_config.add((*dai).current_config as usize);
    match (*config).type_ {
        SOF_DAI_INTEL_SSP => {
            if (*v).abi_version < SOF_ABI_VER(3, 18, 0) &&
               ((flags & SOF_DAI_CONFIG_FLAGS_HW_PARAMS) != 0 || (flags & SOF_DAI_CONFIG_FLAGS_HW_FREE) != 0) {
                return 0;
            }
        }
        SOF_DAI_INTEL_HDA => if !data.is_null() { (*config).hda.link_dma_ch = (*data).dai_data; },
        SOF_DAI_INTEL_ALH => if !data.is_null() {
            if (flags & SOF_DAI_CONFIG_FLAGS_HW_PARAMS) != 0 {
                if (*data).dai_index < INTEL_ALH_DAI_INDEX_BASE { return -EINVAL; }
                (*config).dai_index = (*data).dai_index - INTEL_ALH_DAI_INDEX_BASE;
            }
            (*config).alh.stream_id = (*data).dai_data;
        },
        _ => {}
    }
    if (flags & SOF_DAI_CONFIG_FLAGS_HW_PARAMS) != 0 {
        (*config).flags &= !SOF_DAI_CONFIG_FLAGS_CMD_MASK;
        (*config).flags |= flags;
    } else {
        (*config).flags = flags;
    }
    let mut ret = 0;
    if (*swidget).use_count > 0 {
        ret = sof_ipc_tx_message_no_reply((*sdev).ipc, config as *mut c_void, (*config).hdr.size as usize);
        if ret < 0 { dev_err((*sdev).dev, c"Failed to set dai config for %s\n".as_ptr(), (*dai).name); }
        (*config).flags = SOF_DAI_CONFIG_FLAGS_NONE;
    }
    ret
}

unsafe fn sof_ipc3_widget_setup(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int {
    if (*swidget).private.is_null() { return 0; }
    let ret = match (*swidget).id {
        snd_soc_dapm_dai_in | snd_soc_dapm_dai_out => {
            let dai = (*swidget).private as *mut snd_sof_dai;
            let dai_data = (*dai).private as *mut sof_dai_private_data;
            let comp = &mut (*(*dai_data).comp_dai).comp as *mut sof_ipc_comp;
            sof_ipc_tx_message_no_reply((*sdev).ipc, (*dai_data).comp_dai as *mut c_void, (*comp).hdr.size as usize)
        }
        snd_soc_dapm_scheduler => sof_ipc_tx_message_no_reply((*sdev).ipc, (*swidget).private, size_of::<sof_ipc_pipe_new>()),
        _ => {
            let hdr = (*swidget).private as *mut sof_ipc_cmd_hdr;
            sof_ipc_tx_message_no_reply((*sdev).ipc, (*swidget).private, (*hdr).size as usize)
        }
    };
    if ret < 0 { dev_err((*sdev).dev, c"Failed to setup widget %s\n".as_ptr(), (*(*swidget).widget).name); }
    ret
}

unsafe fn sof_ipc3_set_up_all_pipelines(sdev: *mut snd_sof_dev, verify: bool) -> c_int {
    let v = &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version;
    let mut ret = 0;
    list_for_each_snd_sof_widget(&mut (*sdev).widget_list, |swidget| {
        if !verify && (*swidget).dynamic_pipeline_widget { return true; }
        if (*v).abi_version < SOF_ABI_VER(3, 19, 0) && (*swidget).id == snd_soc_dapm_scheduler { return true; }
        if WIDGET_IS_DAI((*swidget).id) {
            let dai = (*swidget).private as *mut snd_sof_dai;
            if !dai.is_null() && !(*dai).private.is_null() {
                let private = (*dai).private as *mut sof_dai_private_data;
                if !(*private).dai_config.is_null() && (*(*private).dai_config).type_ == SOF_DAI_INTEL_HDA {
                    (*(*private).dai_config).hda.link_dma_ch = DMA_CHAN_INVALID;
                }
            }
        }
        ret = sof_widget_setup(sdev, swidget);
        ret >= 0
    });
    if ret < 0 { return ret; }
    list_for_each_snd_sof_route(&mut (*sdev).route_list, |sroute| {
        if !verify && ((*(*sroute).src_widget).dynamic_pipeline_widget || (*(*sroute).sink_widget).dynamic_pipeline_widget) { return true; }
        if (*(*sroute).src_widget).id != snd_soc_dapm_buffer && (*(*sroute).sink_widget).id != snd_soc_dapm_buffer { return true; }
        ret = sof_route_setup(sdev, (*(*sroute).src_widget).widget, (*(*sroute).sink_widget).widget);
        ret >= 0
    });
    if ret < 0 { return ret; }
    list_for_each_snd_sof_widget(&mut (*sdev).widget_list, |swidget| {
        if (*swidget).id != snd_soc_dapm_scheduler { return true; }
        if !verify && (*swidget).dynamic_pipeline_widget { return true; }
        if (*v).abi_version < SOF_ABI_VER(3, 19, 0) {
            ret = sof_widget_setup(sdev, swidget);
            if ret < 0 { return false; }
        }
        (*(*swidget).spipe).complete = sof_ipc3_complete_pipeline(sdev, swidget);
        (*(*swidget).spipe).complete >= 0
    });
    if ret < 0 { ret } else { 0 }
}

unsafe fn sof_tear_down_left_over_pipelines(sdev: *mut snd_sof_dev) -> c_int {
    let mut ret = sof_pcm_free_all_streams(sdev);
    if ret != 0 { return ret; }
    list_for_each_snd_sof_widget(&mut (*sdev).widget_list, |swidget| {
        if WIDGET_IS_DAI((*swidget).id) && (*swidget).use_count == 1 {
            ret = sof_widget_free(sdev, swidget);
            return ret >= 0;
        }
        true
    });
    ret
}

unsafe fn sof_ipc3_free_widgets_in_list(sdev: *mut snd_sof_dev, include_scheduler: bool, dyn_widgets: *mut bool, verify: bool) -> c_int {
    let v = &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version;
    let mut ret = 0;
    list_for_each_snd_sof_widget(&mut (*sdev).widget_list, |swidget| {
        if (*swidget).dynamic_pipeline_widget { *dyn_widgets = true; return true; }
        if !verify && !(*swidget).dynamic_pipeline_widget && SOF_FW_VER((*v).major, (*v).minor, (*v).micro) < SOF_FW_VER(2, 2, 0) {
            (*swidget).use_count = 0;
            if !(*swidget).spipe.is_null() { (*(*swidget).spipe).complete = 0; }
            return true;
        }
        if include_scheduler && (*swidget).id != snd_soc_dapm_scheduler { return true; }
        if !include_scheduler && (*swidget).id == snd_soc_dapm_scheduler { return true; }
        ret = sof_widget_free(sdev, swidget);
        ret >= 0
    });
    ret
}

unsafe fn sof_ipc3_tear_down_all_pipelines(sdev: *mut snd_sof_dev, verify: bool) -> c_int {
    let v = &mut (*sdev).fw_ready.version as *mut sof_ipc_fw_version;
    let mut dyn_widgets = false;
    let mut ret = sof_ipc3_free_widgets_in_list(sdev, false, &mut dyn_widgets, verify);
    if ret < 0 { return ret; }
    if !verify && (dyn_widgets || SOF_FW_VER((*v).major, (*v).minor, (*v).micro) >= SOF_FW_VER(2, 2, 0)) {
        ret = sof_tear_down_left_over_pipelines(sdev);
        if ret < 0 { return ret; }
    }
    ret = sof_ipc3_free_widgets_in_list(sdev, true, &mut dyn_widgets, verify);
    if ret < 0 { return ret; }
    list_for_each_snd_sof_route(&mut (*sdev).route_list, |sroute| { (*sroute).setup = false; true });
    list_for_each_snd_sof_widget(&mut (*sdev).widget_list, |swidget| {
        if (*swidget).use_count != 0 {
            dev_err((*sdev).dev, c"%s: widget %s is still in use: count %d\n".as_ptr(), c"sof_ipc3_tear_down_all_pipelines".as_ptr(), (*(*swidget).widget).name, (*swidget).use_count);
        }
        true
    });
    0
}

unsafe fn sof_ipc3_dai_get_param(sdev: *mut snd_sof_dev, dai: *mut snd_sof_dai, param_type: c_int) -> c_int {
    let private = (*dai).private as *mut sof_dai_private_data;
    if private.is_null() || (*private).dai_config.is_null() { return 0; }
    match (*(*private).dai_config).type_ {
        SOF_DAI_INTEL_SSP => match param_type {
            SOF_DAI_PARAM_INTEL_SSP_MCLK => (*(*private).dai_config).ssp.mclk_rate as c_int,
            SOF_DAI_PARAM_INTEL_SSP_BCLK => (*(*private).dai_config).ssp.bclk_rate as c_int,
            SOF_DAI_PARAM_INTEL_SSP_TDM_SLOTS => (*(*private).dai_config).ssp.tdm_slots as c_int,
            _ => { dev_err((*sdev).dev, c"invalid SSP param %d\n".as_ptr(), param_type); -EINVAL }
        },
        _ => {
            dev_err((*sdev).dev, c"DAI type %d not supported yet!\n".as_ptr(), (*(*private).dai_config).type_);
            -EINVAL
        }
    }
}

unsafe fn sof_ipc3_parse_manifest(scomp: *mut snd_soc_component, _index: c_int, man: *mut snd_soc_tplg_manifest) -> c_int {
    let size = le32_to_cpu((*man).priv_.size);
    if size == 0 {
        dev_dbg((*scomp).dev, c"No topology ABI info\n".as_ptr());
        return 0;
    }
    if size != SOF_IPC3_TPLG_ABI_SIZE {
        dev_err((*scomp).dev, c"%s: Invalid topology ABI size: %u\n".as_ptr(), c"sof_ipc3_parse_manifest".as_ptr(), size);
        return -EINVAL;
    }
    dev_info((*scomp).dev, c"Topology: ABI %d:%d:%d Kernel ABI %d:%d:%d\n".as_ptr(),
             (*man).priv_.data[0], (*man).priv_.data[1], (*man).priv_.data[2],
             SOF_ABI_MAJOR, SOF_ABI_MINOR, SOF_ABI_PATCH);
    let abi_version = SOF_ABI_VER((*man).priv_.data[0] as u32, (*man).priv_.data[1] as u32, (*man).priv_.data[2] as u32);
    if SOF_ABI_VERSION_INCOMPATIBLE(SOF_ABI_VERSION, abi_version) { return -EINVAL; }
    // C conditional: IS_ENABLED(CONFIG_SND_SOC_SOF_STRICT_ABI_CHECKS)
    if IS_ENABLED_CONFIG_SND_SOC_SOF_STRICT_ABI_CHECKS() && SOF_ABI_VERSION_MINOR(abi_version) > SOF_ABI_MINOR {
        return -EINVAL;
    }
    0
}

unsafe fn sof_ipc3_link_setup(_sdev: *mut snd_sof_dev, link: *mut snd_soc_dai_link) -> c_int {
    if (*link).no_pcm { return 0; }
    (*link).trigger[SNDRV_PCM_STREAM_PLAYBACK as usize] = SND_SOC_DPCM_TRIGGER_PRE;
    (*link).trigger[SNDRV_PCM_STREAM_CAPTURE as usize] = SND_SOC_DPCM_TRIGGER_POST;
    0
}

static mut host_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_PCM_TOKENS, SOF_COMP_TOKENS];
static mut comp_generic_token_list: [sof_tokens; 3] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_COMP_TOKENS];
static mut buffer_token_list: [sof_tokens; 1] = [SOF_BUFFER_TOKENS];
static mut pipeline_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_PIPELINE_TOKENS, SOF_SCHED_TOKENS];
static mut asrc_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_ASRC_TOKENS, SOF_COMP_TOKENS];
static mut src_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_SRC_TOKENS, SOF_COMP_TOKENS];
static mut pga_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_VOLUME_TOKENS, SOF_COMP_TOKENS];
static mut dai_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_DAI_TOKENS, SOF_COMP_TOKENS];
static mut process_token_list: [sof_tokens; 4] = [SOF_CORE_TOKENS, SOF_COMP_EXT_TOKENS, SOF_PROCESS_TOKENS, SOF_COMP_TOKENS];

static tplg_ipc3_widget_ops: [sof_ipc_tplg_widget_ops; SND_SOC_DAPM_TYPE_COUNT as usize] =
    sof_ipc_tplg_widget_ops_designated_array! {
        snd_soc_dapm_aif_in => (sof_ipc3_widget_setup_comp_host, sof_ipc3_widget_free_comp, host_token_list, None),
        snd_soc_dapm_aif_out => (sof_ipc3_widget_setup_comp_host, sof_ipc3_widget_free_comp, host_token_list, None),
        snd_soc_dapm_dai_in => (sof_ipc3_widget_setup_comp_dai, sof_ipc3_widget_free_comp_dai, dai_token_list, None),
        snd_soc_dapm_dai_out => (sof_ipc3_widget_setup_comp_dai, sof_ipc3_widget_free_comp_dai, dai_token_list, None),
        snd_soc_dapm_buffer => (sof_ipc3_widget_setup_comp_buffer, sof_ipc3_widget_free_comp, buffer_token_list, None),
        snd_soc_dapm_mixer => (sof_ipc3_widget_setup_comp_mixer, sof_ipc3_widget_free_comp, comp_generic_token_list, None),
        snd_soc_dapm_src => (sof_ipc3_widget_setup_comp_src, sof_ipc3_widget_free_comp, src_token_list, None),
        snd_soc_dapm_asrc => (sof_ipc3_widget_setup_comp_asrc, sof_ipc3_widget_free_comp, asrc_token_list, None),
        snd_soc_dapm_siggen => (sof_ipc3_widget_setup_comp_tone, sof_ipc3_widget_free_comp, comp_generic_token_list, None),
        snd_soc_dapm_scheduler => (sof_ipc3_widget_setup_comp_pipeline, sof_ipc3_widget_free_comp, pipeline_token_list, None),
        snd_soc_dapm_pga => (sof_ipc3_widget_setup_comp_pga, sof_ipc3_widget_free_comp, pga_token_list, None),
        snd_soc_dapm_mux => (sof_ipc3_widget_setup_comp_mux, sof_ipc3_widget_free_comp, comp_generic_token_list, None),
        snd_soc_dapm_demux => (sof_ipc3_widget_setup_comp_mux, sof_ipc3_widget_free_comp, comp_generic_token_list, None),
        snd_soc_dapm_effect => (sof_widget_update_ipc_comp_process, sof_ipc3_widget_free_comp, process_token_list, Some(sof_ipc3_widget_bind_event)),
    };

#[no_mangle]
pub static ipc3_tplg_ops: sof_ipc_tplg_ops = sof_ipc_tplg_ops {
    widget: tplg_ipc3_widget_ops.as_ptr(),
    control: unsafe { &tplg_ipc3_control_ops as *const _ },
    route_setup: Some(sof_ipc3_route_setup),
    control_setup: Some(sof_ipc3_control_setup),
    control_free: Some(sof_ipc3_control_free),
    pipeline_complete: Some(sof_ipc3_complete_pipeline),
    token_list: ipc3_token_list.as_ptr(),
    widget_free: Some(sof_ipc3_widget_free),
    widget_setup: Some(sof_ipc3_widget_setup),
    dai_config: Some(sof_ipc3_dai_config),
    dai_get_param: Some(sof_ipc3_dai_get_param),
    set_up_all_pipelines: Some(sof_ipc3_set_up_all_pipelines),
    tear_down_all_pipelines: Some(sof_ipc3_tear_down_all_pipelines),
    parse_manifest: Some(sof_ipc3_parse_manifest),
    link_setup: Some(sof_ipc3_link_setup),
};

// External kernel/SOF symbols, types, constants, macros, list iteration helpers,
// and C-layout field definitions are expected to be supplied by translated
// dependency units.  The declarations below intentionally mirror use sites in
// this source file without providing local implementations.
extern "C" {
    static tplg_ipc3_control_ops: sof_ipc_tplg_control_ops;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memset(dst: *mut c_void, val: c_int, count: size_t) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn guid_is_null(guid: *const guid_t) -> bool;
    fn snd_soc_component_get_drvdata(scomp: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn snd_sof_find_swidget(scomp: *mut snd_soc_component, name: *const c_char) -> *mut snd_sof_widget;
    fn snd_sof_find_spcm_name(scomp: *mut snd_soc_component, name: *const c_char) -> *mut snd_sof_pcm;
    fn sof_update_ipc_object(scomp: *mut snd_soc_component, object: *mut c_void, token_id: sof_tokens, tuples: *mut c_void, num_tuples: c_int, object_size: size_t, count: c_int) -> c_int;
    fn sof_ipc_tx_message_no_reply(ipc: *mut c_void, msg: *mut c_void, size: size_t) -> c_int;
    fn sof_debug_check_flag(flag: c_uint) -> bool;
    fn snd_soc_tplg_widget_bind_event(widget: *mut snd_soc_dapm_widget, events: *const snd_soc_tplg_widget_events, num_events: u32, event_type: u16) -> c_int;
    fn sof_widget_setup(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int;
    fn sof_widget_free(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int;
    fn sof_route_setup(sdev: *mut snd_sof_dev, source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int;
    fn sof_pcm_free_all_streams(sdev: *mut snd_sof_dev) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> u32;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn le32_to_cpu(v: u32) -> u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
