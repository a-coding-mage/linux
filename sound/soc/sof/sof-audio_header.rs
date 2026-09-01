/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2019 Intel Corporation
 *
 * Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
 */

/* Rust translation of soc/sof/sof-audio.h. */
/* Dependencies from the original includes are expected to be supplied elsewhere. */

pub const SOF_AUDIO_PCM_DRV_NAME: &[u8] = b"sof-audio-component\0";

/*
 * The ipc4 firmware only supports up to 8 sink or source pins
 * per widget, because only 3 bits are used for queue(pin) ID
 * in ipc4 protocol.
 */
pub const SOF_WIDGET_MAX_NUM_PINS: u32 = 8;

/* Widget pin type */
pub const SOF_PIN_TYPE_INPUT: u32 = 0;
pub const SOF_PIN_TYPE_OUTPUT: u32 = 1;

/* max number of FE PCMs before BEs */
pub const SOF_BE_PCM_BASE: u32 = 16;

pub const DMA_CHAN_INVALID: u32 = 0xFFFFFFFF;

pub unsafe fn WIDGET_IS_DAI(id: i32) -> bool {
    id == snd_soc_dapm_dai_in || id == snd_soc_dapm_dai_out
}

pub unsafe fn WIDGET_IS_AIF(id: i32) -> bool {
    id == snd_soc_dapm_aif_in || id == snd_soc_dapm_aif_out
}

pub unsafe fn WIDGET_IS_AIF_OR_DAI(id: i32) -> bool {
    WIDGET_IS_DAI(id) || WIDGET_IS_AIF(id)
}

pub unsafe fn WIDGET_IS_COPIER(id: i32) -> bool {
    WIDGET_IS_AIF_OR_DAI(id) || id == snd_soc_dapm_buffer
}

pub const SOF_DAI_PARAM_INTEL_SSP_MCLK: u32 = 0;
pub const SOF_DAI_PARAM_INTEL_SSP_BCLK: u32 = 1;
pub const SOF_DAI_PARAM_INTEL_SSP_TDM_SLOTS: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sof_widget_op {
    SOF_WIDGET_PREPARE,
    SOF_WIDGET_SETUP,
    SOF_WIDGET_FREE,
    SOF_WIDGET_UNPREPARE,
}

/*
 * Volume fractional word length define to 16 sets
 * the volume linear gain value to use Qx.16 format
 */
pub const VOLUME_FWL: u32 = 16;

pub const SOF_TLV_ITEMS: usize = 3;

pub unsafe fn mixer_to_ipc(value: ::core::ffi::c_uint, volume_map: *mut u32, size: i32) -> u32 {
    if value >= size as ::core::ffi::c_uint {
        return *volume_map.offset((size - 1) as isize);
    }

    *volume_map.offset(value as isize)
}

pub unsafe fn ipc_to_mixer(value: u32, volume_map: *mut u32, size: i32) -> u32 {
    let mut i: i32 = 0;

    while i < size {
        if *volume_map.offset(i as isize) >= value {
            return i as u32;
        }
        i += 1;
    }

    (i - 1) as u32
}

pub enum snd_soc_component {}
pub enum snd_pcm_substream {}
pub enum snd_pcm_hw_params {}
pub enum snd_sof_platform_stream_params {}
pub enum snd_soc_pcm_runtime {}
pub enum snd_sof_dev {}
pub enum snd_ctl_elem_value {}
pub enum snd_soc_dapm_widget {}
pub enum snd_soc_tplg_manifest {}
pub enum snd_soc_dai_link {}
pub enum snd_dma_buffer {}
pub enum sof_ipc_stream_posn {}
pub enum snd_compr_stream {}
pub enum work_struct {}
pub enum snd_soc_dapm_widget_list {}
pub enum list_head {}
pub enum snd_soc_tplg_pcm {}
pub enum snd_soc_tplg_hw_config {}
pub enum mutex {}
pub enum guid_t {}
pub enum ida {}
pub enum snd_soc_dapm_route {}
pub enum snd_kcontrol {}
pub enum snd_ctl_elem_info {}
pub enum sof_ipc_ctrl_data {}

pub type size_t = usize;
pub type snd_pcm_uframes_t = usize;
pub type snd_pcm_sframes_t = isize;

unsafe extern "C" {
    pub static snd_soc_dapm_dai_in: i32;
    pub static snd_soc_dapm_dai_out: i32;
    pub static snd_soc_dapm_aif_in: i32;
    pub static snd_soc_dapm_aif_out: i32;
    pub static snd_soc_dapm_buffer: i32;

    pub fn snd_soc_component_get_drvdata(scomp: *mut snd_soc_component) -> *mut snd_sof_dev;
    pub fn le32_to_cpu(value: u32) -> u32;
}

#[repr(C)]
pub struct snd_sof_dai_config_data {
    pub dai_index: i32,
    pub dai_data: i32, /* contains DAI-specific information */
    pub dai_node_id: i32, /* contains DAI-specific information for Gateway configuration */
}

/**
 * struct sof_ipc_pcm_ops - IPC-specific PCM ops
 */
#[repr(C)]
pub struct sof_ipc_pcm_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_sof_platform_stream_params) -> i32>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, i32) -> i32>,
    pub dai_link_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> i32>,
    pub pcm_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm) -> i32>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm)>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_uframes_t) -> i32>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    pub reset_hw_params_during_stop: bool,
    pub ipc_first_on_start: bool,
    pub platform_stop_during_hw_free: bool,
    pub d0i3_supported_in_s0ix: bool,
}

/**
 * struct sof_ipc_tplg_control_ops - IPC-specific ops for topology kcontrol IO
 */
#[repr(C)]
pub struct sof_ipc_tplg_control_ops {
    pub volume_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool>,
    pub volume_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> i32>,
    pub switch_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool>,
    pub switch_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> i32>,
    pub enum_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> bool>,
    pub enum_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> i32>,
    pub bytes_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> i32>,
    pub bytes_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut snd_ctl_elem_value) -> i32>,
    pub bytes_ext_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *const ::core::ffi::c_uint, ::core::ffi::c_uint) -> i32>,
    pub bytes_ext_volatile_get: Option<unsafe extern "C" fn(*mut snd_sof_control, *const ::core::ffi::c_uint, ::core::ffi::c_uint) -> i32>,
    pub bytes_ext_put: Option<unsafe extern "C" fn(*mut snd_sof_control, *const ::core::ffi::c_uint, ::core::ffi::c_uint) -> i32>,
    /* update control data based on notification from the DSP */
    pub update: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut ::core::ffi::c_void)>,
    /* Optional callback to setup kcontrols associated with an swidget */
    pub widget_kcontrol_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> i32>,
    /* mandatory callback to set up volume table for volume kcontrols */
    pub set_up_volume_table: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut [i32; SOF_TLV_ITEMS], i32) -> i32>,
}

#[repr(C)]
pub struct sof_ipc_tplg_widget_ops {
    pub ipc_setup: Option<unsafe extern "C" fn(*mut snd_sof_widget) -> i32>,
    pub ipc_free: Option<unsafe extern "C" fn(*mut snd_sof_widget)>,
    pub token_list: *mut sof_tokens,
    pub token_list_size: i32,
    pub bind_event: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_sof_widget, u16) -> i32>,
    pub ipc_prepare: Option<unsafe extern "C" fn(*mut snd_sof_widget, *mut snd_pcm_hw_params, *mut snd_sof_platform_stream_params, *mut snd_pcm_hw_params, i32) -> i32>,
    pub ipc_unprepare: Option<unsafe extern "C" fn(*mut snd_sof_widget)>,
}

#[repr(C)]
pub struct sof_ipc_tplg_ops {
    pub widget: *const sof_ipc_tplg_widget_ops,
    pub control: *const sof_ipc_tplg_control_ops,
    pub route_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_route) -> i32>,
    pub route_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_route) -> i32>,
    pub token_list: *const sof_token_info,
    pub control_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_control) -> i32>,
    pub control_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_control) -> i32>,
    pub pipeline_complete: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> i32>,
    pub widget_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> i32>,
    pub widget_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget) -> i32>,
    pub dai_config: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget, ::core::ffi::c_uint, *mut snd_sof_dai_config_data) -> i32>,
    pub host_config: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_widget, *mut snd_sof_platform_stream_params)>,
    pub dai_get_param: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_dai, i32) -> i32>,
    pub set_up_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool) -> i32>,
    pub tear_down_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool) -> i32>,
    pub parse_manifest: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, *mut snd_soc_tplg_manifest) -> i32>,
    pub link_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai_link) -> i32>,
}

/** struct snd_sof_tuple - Tuple info */
#[repr(C)]
pub union snd_sof_tuple_value {
    pub v: u32,
    pub s: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_sof_tuple {
    pub token: u32,
    pub value: snd_sof_tuple_value,
}

/*
 * List of SOF token ID's. The order of ID's does not matter as token arrays are looked up based on
 * the ID.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sof_tokens {
    SOF_PCM_TOKENS,
    SOF_PIPELINE_TOKENS,
    SOF_SCHED_TOKENS,
    SOF_ASRC_TOKENS,
    SOF_SRC_TOKENS,
    SOF_COMP_TOKENS,
    SOF_BUFFER_TOKENS,
    SOF_VOLUME_TOKENS,
    SOF_PROCESS_TOKENS,
    SOF_DAI_TOKENS,
    SOF_DAI_LINK_TOKENS,
    SOF_HDA_TOKENS,
    SOF_SSP_TOKENS,
    SOF_ALH_TOKENS,
    SOF_DMIC_TOKENS,
    SOF_DMIC_PDM_TOKENS,
    SOF_ESAI_TOKENS,
    SOF_SAI_TOKENS,
    SOF_AFE_TOKENS,
    SOF_CORE_TOKENS,
    SOF_COMP_EXT_TOKENS,
    SOF_IN_AUDIO_FORMAT_TOKENS,
    SOF_OUT_AUDIO_FORMAT_TOKENS,
    SOF_COPIER_DEEP_BUFFER_TOKENS,
    SOF_COPIER_TOKENS,
    SOF_AUDIO_FMT_NUM_TOKENS,
    SOF_COPIER_FORMAT_TOKENS,
    SOF_GAIN_TOKENS,
    SOF_ACPDMIC_TOKENS,
    SOF_ACPI2S_TOKENS,
    SOF_MICFIL_TOKENS,
    SOF_ACP_SDW_TOKENS,

    /* this should be the last */
    SOF_TOKEN_COUNT,
}

#[repr(C)]
pub struct sof_topology_token {
    pub token: u32,
    pub type_: u32,
    pub get_token: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, u32) -> i32>,
    pub offset: u32,
}

#[repr(C)]
pub struct sof_token_info {
    pub name: *const ::core::ffi::c_char,
    pub tokens: *const sof_topology_token,
    pub count: i32,
}

#[repr(C)]
pub struct snd_sof_pcm_stream_pipeline_list {
    pub pipelines: *mut *mut snd_sof_pipeline,
    pub count: u32,
}

/* PCM stream, mapped to FW component  */
#[repr(C)]
pub struct snd_sof_pcm_stream {
    pub comp_id: u32,
    pub page_table: snd_dma_buffer,
    pub posn: sof_ipc_stream_posn,
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub period_elapsed_work: work_struct,
    pub list: *mut snd_soc_dapm_widget_list, /* list of connected DAPM widgets */
    pub d0i3_compatible: bool, /* DSP can be in D0I3 when this pcm is opened */
    pub pause_supported: bool, /* PCM device supports PAUSE operation */
    pub dsp_max_burst_size_in_ms: ::core::ffi::c_uint, /* The maximum size of the host DMA burst in ms */
    /*
     * flag to indicate that the DSP pipelines should be kept
     * active or not while suspending the stream
     */
    pub suspend_ignored: bool,
    pub pipeline_list: snd_sof_pcm_stream_pipeline_list,

    /* used by IPC implementation and core does not touch it */
    pub private: *mut ::core::ffi::c_void,
}

/* ALSA SOF PCM device */
#[repr(C)]
pub struct snd_sof_pcm {
    pub scomp: *mut snd_soc_component,
    pub stream: [snd_sof_pcm_stream; 2],
    pub list: list_head, /* list in sdev pcm list */
    pub params: [snd_pcm_hw_params; 2],
    pub platform_params: [snd_sof_platform_stream_params; 2],
    pub prepared: [bool; 2], /* PCM_PARAMS set successfully */
    pub setup_done: [bool; 2], /* the setup of the SOF PCM device is done */
    pub pending_stop: [bool; 2], /* only used if (!pcm_ops->platform_stop_during_hw_free) */

    /* Must be last - ends in a flex-array member. */
    pub pcm: snd_soc_tplg_pcm,
}

#[repr(C)]
pub struct snd_sof_led_control {
    pub use_led: ::core::ffi::c_uint,
    pub direction: ::core::ffi::c_uint,
    pub led_value: i32,
}

/* ALSA SOF Kcontrol device */
#[repr(C)]
pub struct snd_sof_control {
    pub scomp: *mut snd_soc_component,
    pub name: *const ::core::ffi::c_char,
    pub comp_id: i32,
    pub min_volume_step: i32, /* min volume step for volume_table */
    pub max_volume_step: i32, /* max volume step for volume_table */
    pub num_channels: i32,
    pub access: ::core::ffi::c_uint,
    pub info_type: i32,
    pub index: i32, /* pipeline ID */
    pub priv_: *mut ::core::ffi::c_void, /* private data copied from topology */
    pub priv_size: size_t, /* size of private data */
    pub max_size: size_t,
    pub ipc_control_data: *mut ::core::ffi::c_void,
    pub old_ipc_control_data: *mut ::core::ffi::c_void,
    pub max: i32, /* applicable to volume controls */
    pub size: u32, /* cdata size */
    pub volume_table: *mut u32, /* volume table computed from tlv data*/

    pub list: list_head, /* list in sdev control list */

    pub led_ctl: snd_sof_led_control,

    /* if true, the control's data needs to be updated from Firmware */
    pub comp_data_dirty: bool,
}

#[repr(C)]
pub struct snd_sof_dai_link {
    pub tuples: *mut snd_sof_tuple,
    pub num_tuples: i32,
    pub link: *mut snd_soc_dai_link,
    pub num_hw_configs: i32,
    pub default_hw_cfg_id: i32,
    pub type_: i32,
    pub list: list_head,
    /* C flexible array: struct snd_soc_tplg_hw_config hw_configs[] __counted_by(num_hw_configs); */
    pub hw_configs: [snd_soc_tplg_hw_config; 0],
}

/* ASoC SOF DAPM widget */
#[repr(C)]
pub struct snd_sof_widget {
    pub scomp: *mut snd_soc_component,
    pub comp_id: i32,
    pub pipeline_id: i32,
    /*
     * the prepared flag is used to indicate that a widget has been prepared for getting set
     * up in the DSP.
     */
    pub prepared: bool,

    pub setup_mutex: mutex, /* to protect the swidget setup and free operations */

    /*
     * use_count is protected by the PCM mutex held by the core and the
     * setup_mutex against non stream domain races (kcontrol access for
     * example)
     */
    pub use_count: i32,

    pub core: i32,
    pub id: i32, /* id is the DAPM widget type */
    /*
     * Instance ID is set dynamically when the widget gets set up in the FW. It should be
     * unique for each module type across all pipelines. This will not be used in SOF_IPC.
     */
    pub instance_id: i32,

    /*
     * Flag indicating if the widget should be set up dynamically when a PCM is opened.
     * This flag is only set for the scheduler type widget in topology. During topology
     * loading, this flag is propagated to all the widgets belonging to the same pipeline.
     * When this flag is not set, a widget is set up at the time of topology loading
     * and retained until the DSP enters D3. It will need to be set up again when resuming
     * from D3.
     */
    pub dynamic_pipeline_widget: bool,

    /* Scheduling domain (enum sof_comp_domain), unset, Low Latency, or Data Processing */
    pub comp_domain: u32,

    /* Module instance's memory configuration. */
    pub domain_id: u32, /* Module instance's userspace domain ID */
    pub stack_bytes: u32, /* Module instance's stack size requirement */
    pub heap_bytes: u32, /* Module instance's heap size requirement */

    pub widget: *mut snd_soc_dapm_widget,
    pub list: list_head, /* list in sdev widget list */
    pub spipe: *mut snd_sof_pipeline,
    pub module_info: *mut ::core::ffi::c_void,

    pub uuid: guid_t,

    pub num_tuples: i32,
    pub tuples: *mut snd_sof_tuple,

    /*
     * The allowed range for num_input/output_pins is [0, SOF_WIDGET_MAX_NUM_PINS].
     * Widgets may have zero input or output pins, for example the tone widget has
     * zero input pins.
     */
    pub num_input_pins: u32,
    pub num_output_pins: u32,

    /*
     * The input/output pin binding array, it takes the form of
     * [widget_name_connected_to_pin0, widget_name_connected_to_pin1, ...],
     * with the index as the queue ID.
     *
     * The array is used for special pin binding. Note that even if there
     * is only one input/output pin requires special pin binding, pin binding
     * should be defined for all input/output pins in topology, for pin(s) that
     * are not used, give the value "NotConnected".
     *
     * If pin binding is not defined in topology, nothing to parse in the kernel,
     * input_pin_binding and output_pin_binding shall be NULL.
     */
    pub input_pin_binding: *mut *mut ::core::ffi::c_char,
    pub output_pin_binding: *mut *mut ::core::ffi::c_char,

    pub output_queue_ida: ida,
    pub input_queue_ida: ida,

    pub private: *mut ::core::ffi::c_void, /* core does not touch this */
}

#[repr(C)]
pub struct snd_sof_pipeline {
    pub pipe_widget: *mut snd_sof_widget,
    pub started_count: i32,
    pub paused_count: i32,
    pub complete: i32,
    pub core_mask: ::core::ffi::c_ulong,
    pub list: list_head,
    pub direction_valid: bool,
    pub direction: u32,
}

/* ASoC SOF DAPM route */
#[repr(C)]
pub struct snd_sof_route {
    pub scomp: *mut snd_soc_component,

    pub route: *mut snd_soc_dapm_route,
    pub list: list_head, /* list in sdev route list */
    pub src_widget: *mut snd_sof_widget,
    pub sink_widget: *mut snd_sof_widget,
    pub setup: bool,

    pub src_queue_id: i32,
    pub dst_queue_id: i32,

    pub private: *mut ::core::ffi::c_void,
}

/* ASoC DAI device */
#[repr(C)]
pub struct snd_sof_dai {
    pub scomp: *mut snd_soc_component,
    pub name: *const ::core::ffi::c_char,
    pub type_: u32,

    pub number_configs: i32,
    pub current_config: i32,
    pub list: list_head, /* list in sdev dai list */
    /* core should not touch this */
    pub platform_private: *const ::core::ffi::c_void,
    pub private: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    /*
     * Kcontrols.
     */
    pub fn snd_sof_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_volume_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_volume_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32;
    pub fn snd_sof_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_enum_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_enum_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_bytes_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_bytes_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32;
    pub fn snd_sof_bytes_ext_put(kcontrol: *mut snd_kcontrol, binary_data: *const ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> i32;
    pub fn snd_sof_bytes_ext_get(kcontrol: *mut snd_kcontrol, binary_data: *mut ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> i32;
    pub fn snd_sof_bytes_ext_volatile_get(kcontrol: *mut snd_kcontrol, binary_data: *mut ::core::ffi::c_uint, size: ::core::ffi::c_uint) -> i32;
    pub fn snd_sof_control_notify(sdev: *mut snd_sof_dev, cdata: *mut sof_ipc_ctrl_data);

    /*
     * Topology.
     * There is no snd_sof_free_topology since topology components will
     * be freed by snd_soc_unregister_component,
     */
    pub fn snd_sof_load_topology(scomp: *mut snd_soc_component, file: *const ::core::ffi::c_char) -> i32;

    /*
     * Stream IPC
     */
    pub fn snd_sof_ipc_stream_posn(scomp: *mut snd_soc_component, spcm: *mut snd_sof_pcm, direction: i32, posn: *mut sof_ipc_stream_posn) -> i32;

    pub fn snd_sof_find_swidget(scomp: *mut snd_soc_component, name: *const ::core::ffi::c_char) -> *mut snd_sof_widget;
    pub fn snd_sof_find_swidget_sname(scomp: *mut snd_soc_component, pcm_name: *const ::core::ffi::c_char, dir: i32) -> *mut snd_sof_widget;
    pub fn snd_sof_find_dai(scomp: *mut snd_soc_component, name: *const ::core::ffi::c_char) -> *mut snd_sof_dai;
}

/*
 * static inline snd_sof_find_spcm_dai() iterates sdev->pcm_list with
 * list_for_each_entry and returns the PCM whose pcm.dai_id matches
 * rtd->dai_link->id. The exact Rust body depends on the external Linux
 * list_head and topology struct definitions supplied by other headers.
 */
pub unsafe fn snd_sof_find_spcm_dai(
    _scomp: *mut snd_soc_component,
    _rtd: *mut snd_soc_pcm_runtime,
) -> *mut snd_sof_pcm {
    ::core::ptr::null_mut()
}

unsafe extern "C" {
    pub fn snd_sof_find_spcm_name(scomp: *mut snd_soc_component, name: *const ::core::ffi::c_char) -> *mut snd_sof_pcm;
    pub fn snd_sof_find_spcm_comp(scomp: *mut snd_soc_component, comp_id: ::core::ffi::c_uint, direction: *mut i32) -> *mut snd_sof_pcm;
    pub fn snd_sof_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    pub fn snd_sof_pcm_init_elapsed_work(work: *mut work_struct);
}

/*
 * snd_sof_pcm specific wrappers for dev_dbg() and dev_err() to provide
 * consistent and useful prints.
 *
 * spcm_dbg, spcm_dbg_ratelimited, and spcm_err are variadic C logging macros.
 * Their Rust equivalents require external dev_dbg/dev_err formatting support.
 */

/*
 * Original condition: #if IS_ENABLED(CONFIG_SND_SOC_SOF_COMPRESS)
 * When enabled, these are external declarations; otherwise the C header provides
 * empty inline functions.
 */
unsafe extern "C" {
    pub fn snd_sof_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
    pub fn snd_sof_compr_init_elapsed_work(work: *mut work_struct);

    /* DAI link fixup */
    pub fn sof_pcm_dai_link_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> i32;

    /* PM */
    pub fn snd_sof_stream_suspend_ignored(sdev: *mut snd_sof_dev) -> bool;
    pub fn snd_sof_dsp_only_d0i3_compatible_stream_active(sdev: *mut snd_sof_dev) -> bool;

    /* Machine driver enumeration */
    pub fn sof_machine_register(sdev: *mut snd_sof_dev, pdata: *mut ::core::ffi::c_void) -> i32;
    pub fn sof_machine_unregister(sdev: *mut snd_sof_dev, pdata: *mut ::core::ffi::c_void);

    pub fn sof_widget_setup(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> i32;
    pub fn sof_widget_free(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> i32;
    pub fn sof_route_setup(sdev: *mut snd_sof_dev, wsource: *mut snd_soc_dapm_widget, wsink: *mut snd_soc_dapm_widget) -> i32;

    /* PCM */
    pub fn sof_widget_list_setup(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, fe_params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params, dir: i32) -> i32;
    pub fn sof_widget_list_prepare(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, fe_params: *mut snd_pcm_hw_params, platform_params: *mut snd_sof_platform_stream_params, dir: i32) -> i32;
    pub fn sof_widget_list_unprepare(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: i32);
    pub fn sof_widget_list_free(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: i32) -> i32;
    pub fn sof_pcm_dsp_pcm_free(substream: *mut snd_pcm_substream, sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm) -> i32;
    pub fn sof_pcm_free_all_streams(sdev: *mut snd_sof_dev) -> i32;
    pub fn get_token_u32(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn get_token_u16(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn get_token_comp_format(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn get_token_dai_type(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn get_token_uuid(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn get_token_string(elem: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, offset: u32) -> i32;
    pub fn sof_update_ipc_object(scomp: *mut snd_soc_component, object: *mut ::core::ffi::c_void, token_id: sof_tokens, tuples: *mut snd_sof_tuple, num_tuples: i32, object_size: size_t, token_instance_num: i32) -> i32;
    pub fn vol_compute_gain(value: u32, tlv: *mut i32) -> u32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
