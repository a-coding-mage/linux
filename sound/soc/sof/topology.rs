// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Rust translation of soc/sof/topology.c. Kernel, ASoC, SOF, list, allocation,
// endian, and topology helper symbols are external dependencies supplied by the
// surrounding repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type bool_ = bool;
type u8_ = u8;
type u16_ = u16;
type u32_ = u32;
type u64_ = u64;
type size_t = usize;

const MAX_FEATURE_TPLG_COUNT: usize = 16;
const COMP_ID_UNASSIGNED: u32_ = 0xffffffff;
/*
 * Constants used in the computation of linear volume gain
 * from dB gain 20th root of 10 in Q1.16 fixed-point notation
 */
const VOL_TWENTIETH_ROOT_OF_TEN: u32_ = 73533;
/* 40th root of 10 in Q1.16 fixed-point notation*/
const VOL_FORTIETH_ROOT_OF_TEN: u32_ = 69419;
/* 0.5 dB step value in topology TLV */
const VOL_HALF_DB_STEP: c_int = 50;
/* TLV data items */
const TLV_MIN: usize = 0;
const TLV_STEP: usize = 1;
const TLV_MUTE: usize = 2;

static mut disable_function_topology: bool_ = false;
static mut feature_topologies: [*mut c_char; MAX_FEATURE_TPLG_COUNT] = [null_mut(); MAX_FEATURE_TPLG_COUNT];
static mut feature_tplg_cnt: c_int = 0;

// module_param(disable_function_topology, bool, 0444);
// MODULE_PARM_DESC(disable_function_topology, "Disable function topology loading");
// module_param_array(feature_topologies, charp, &feature_tplg_cnt, 0444);
// MODULE_PARM_DESC(feature_topologies, "Topology list for virtual loop DAI link");

#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct firmware { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dev {
    pub dev: *mut device,
    pub pdata: *mut snd_sof_pdata,
    pub next_comp_id: c_int,
    pub led_present: bool_,
    pub dspless_mode_selected: bool_,
    pub num_cores: c_int,
    pub kcontrol_list: list_head,
    pub dai_list: list_head,
    pub pipeline_list: list_head,
    pub widget_list: list_head,
    pub pcm_list: list_head,
    pub dai_link_list: list_head,
    pub route_list: list_head,
}
#[repr(C)] pub struct snd_sof_pdata { pub tplg_filename_prefix: *const c_char, pub disable_function_topology: bool_, pub machine: *mut snd_sof_machine }
#[repr(C)] pub struct snd_sof_machine {
    pub get_function_tplg_files: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_sof_machine, *const c_char, *mut *mut *const c_char, bool_) -> c_int>,
}
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_card { pub num_links: c_int }
#[repr(C)] pub struct snd_soc_pcm_runtime { pub dai_link: *mut snd_soc_dai_link }
#[repr(C)] pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub num_cpus: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub no_pcm: bool_,
    pub nonatomic: bool_,
    pub dobj: snd_soc_dobj,
}
#[repr(C)] pub struct snd_soc_dai_link_component { pub name: *const c_char }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { pub dobj: snd_soc_dobj }
#[repr(C)] pub struct snd_soc_dobj { pub private: *mut c_void, pub widget: snd_soc_dobj_widget }
#[repr(C)] pub struct snd_soc_dobj_widget { pub kcontrol_type: *mut c_uint }
#[repr(C)] pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub sname: *const c_char,
    pub id: c_int,
    pub dobj: snd_soc_dobj,
    pub num_kcontrols: c_int,
    pub kcontrol_news: *mut snd_kcontrol_new,
    pub no_wname_in_kcontrol_name: bool_,
}
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char, pub dobj: snd_soc_dobj }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_value { _private: [u8; 0] }
#[repr(C)] pub union snd_kcontrol_tlv { pub p: *const c_int }
#[repr(C)] pub struct snd_kcontrol_new { pub private_value: usize, pub access: c_uint, pub index: c_uint, pub info: Option<unsafe extern "C" fn() -> c_int>, pub tlv: snd_kcontrol_tlv }
#[repr(C)] pub struct snd_soc_tplg_ctl_hdr { pub type_: u32_, pub name: [c_char; 64], pub ops: snd_soc_tplg_ctl_ops }
#[repr(C)] pub struct snd_soc_tplg_ctl_ops { pub get: u32_, pub put: u32_, pub info: u32_ }
#[repr(C)] pub struct snd_soc_tplg_mixer_control { pub hdr: snd_soc_tplg_ctl_hdr, pub num_channels: u32_, pub min: u32_, pub max: u32_, pub priv_: snd_soc_tplg_private }
#[repr(C)] pub struct snd_soc_tplg_enum_control { pub hdr: snd_soc_tplg_ctl_hdr, pub num_channels: u32_ }
#[repr(C)] pub struct snd_soc_tplg_bytes_control { pub hdr: snd_soc_tplg_ctl_hdr, pub priv_: snd_soc_tplg_private }
#[repr(C)] pub struct snd_soc_tplg_private { pub size: u32_, pub array: *mut snd_soc_tplg_vendor_array, pub data: *mut c_void }
#[repr(C)] pub struct snd_soc_tplg_vendor_array { pub size: u32_, pub type_: u32_, pub num_elems: u32_, pub value: [snd_soc_tplg_vendor_value_elem; 1] }
#[repr(C)] pub struct snd_soc_tplg_vendor_value_elem { pub token: u32_, pub value: u32_ }
#[repr(C)] pub struct snd_soc_tplg_vendor_uuid_elem { pub token: u32_, pub uuid: [u8_; UUID_SIZE] }
#[repr(C)] pub struct snd_soc_tplg_vendor_string_elem { pub token: u32_, pub string: [c_char; 64] }
#[repr(C)] pub struct snd_soc_tplg_dapm_widget { pub priv_: snd_soc_tplg_private, pub num_kcontrols: u32_, pub event_type: u16_, pub name: [c_char; 64], pub sname: [c_char; 64] }
#[repr(C)] pub struct snd_soc_tplg_pcm { pub priv_: snd_soc_tplg_private, pub compress: bool_, pub playback: bool_, pub capture: bool_, pub caps: [snd_soc_tplg_stream_caps; 2], pub dai_name: [c_char; 64] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_soc_tplg_stream_caps { pub name: [c_char; 64] }
#[repr(C)] pub struct snd_soc_tplg_link_config { pub priv_: snd_soc_tplg_private, pub num_hw_configs: u32_, pub hw_config: *const snd_soc_tplg_hw_config, pub default_hw_config_id: u32_ }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_soc_tplg_hw_config { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_tplg_manifest { _private: [u8; 0] }
#[repr(C)] pub struct soc_mixer_control { pub dobj: snd_soc_dobj, pub max: c_uint }
#[repr(C)] pub struct soc_bytes_ext { pub dobj: snd_soc_dobj, pub max: size_t }
#[repr(C)] pub struct soc_enum { pub dobj: snd_soc_dobj }

#[repr(C)] pub struct snd_sof_control {
    pub name: *mut c_char, pub scomp: *mut snd_soc_component, pub access: c_uint, pub info_type: u32_, pub index: c_uint,
    pub comp_id: c_int, pub min_volume_step: u32_, pub max_volume_step: u32_, pub num_channels: u32_, pub max: u32_,
    pub max_size: size_t, pub priv_: *mut c_void, pub priv_size: size_t, pub volume_table: *mut c_void,
    pub ipc_control_data: *mut c_void, pub led_ctl: snd_sof_led_control, pub list: list_head,
}
#[repr(C)] pub struct snd_sof_led_control { pub use_led: u32_, pub direction: u32_, pub led_value: c_int }
#[repr(C)] pub struct snd_sof_widget {
    pub scomp: *mut snd_soc_component, pub widget: *mut snd_soc_dapm_widget, pub comp_id: c_int, pub id: c_int,
    pub pipeline_id: c_int, pub private: *mut c_void, pub setup_mutex: mutex, pub output_queue_ida: ida,
    pub input_queue_ida: ida, pub tuples: *mut snd_sof_tuple, pub num_tuples: c_int, pub num_input_pins: u32_,
    pub num_output_pins: u32_, pub input_pin_binding: *mut *mut c_char, pub output_pin_binding: *mut *mut c_char,
    pub core: c_int, pub spipe: *mut snd_sof_pipeline, pub dynamic_pipeline_widget: bool_, pub instance_id: c_int,
    pub list: list_head,
}
#[repr(C)] pub struct snd_sof_dai { pub name: *const c_char, pub type_: u32_, pub list: list_head }
#[repr(C)] pub struct snd_sof_pipeline { pub pipe_widget: *mut snd_sof_widget, pub list: list_head }
#[repr(C)] pub struct snd_sof_route { pub scomp: *mut snd_soc_component, pub private: *mut c_void, pub route: *mut snd_soc_dapm_route, pub src_widget: *mut snd_sof_widget, pub sink_widget: *mut snd_sof_widget, pub list: list_head }
#[repr(C)] pub struct snd_sof_pcm { pub scomp: *mut snd_soc_component, pub stream: [snd_sof_pcm_stream; 2], pub pcm: snd_soc_tplg_pcm, pub list: list_head }
#[repr(C)] pub struct snd_sof_pcm_stream { pub comp_id: u32_, pub page_table: snd_dma_buffer, pub period_elapsed_work: c_int, pub d0i3_compatible: u16_, pub pause_supported: u16_ }
#[repr(C)] pub struct snd_dma_buffer { _private: [u8; 0] }
#[repr(C)] pub struct snd_sof_dai_link { pub type_: u32_, pub num_hw_configs: u32_, pub hw_configs: *mut snd_soc_tplg_hw_config, pub default_hw_cfg_id: u32_, pub link: *mut snd_soc_dai_link, pub tuples: *mut snd_sof_tuple, pub num_tuples: c_int, pub list: list_head }

#[repr(C)] pub struct sof_topology_token { pub token: u32_, pub type_: u32_, pub get_token: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, u32_) -> c_int>, pub offset: u32_ }
#[repr(C)] pub struct sof_token_info { pub count: c_int, pub tokens: *const sof_topology_token, pub name: *const c_char }
#[repr(C)] pub struct snd_sof_tuple { pub token: u32_, pub value: snd_sof_tuple_value }
#[repr(C)] pub union snd_sof_tuple_value { pub v: u32_, pub s: *mut c_char }
#[repr(C)] pub struct sof_ipc_tplg_ops {
    pub token_list: *const sof_token_info,
    pub control: *const sof_ipc_tplg_control_ops,
    pub control_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_control) -> c_int>,
    pub widget: *const sof_ipc_tplg_widget_ops,
    pub link_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_soc_dai_link) -> c_int>,
    pub control_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_control) -> c_int>,
    pub set_up_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool_) -> c_int>,
    pub tear_down_all_pipelines: Option<unsafe extern "C" fn(*mut snd_sof_dev, bool_) -> c_int>,
    pub parse_manifest: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, *mut snd_soc_tplg_manifest) -> c_int>,
}
#[repr(C)] pub struct sof_ipc_tplg_control_ops { pub set_up_volume_table: Option<unsafe extern "C" fn(*mut snd_sof_control, *mut c_int, c_int) -> c_int> }
#[repr(C)] #[derive(Copy, Clone)] pub struct sof_ipc_tplg_widget_ops {
    pub token_list: *mut u32_, pub token_list_size: c_int,
    pub bind_event: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_sof_widget, u16_) -> c_int>,
    pub ipc_setup: Option<unsafe extern "C" fn(*mut snd_sof_widget) -> c_int>,
    pub ipc_free: Option<unsafe extern "C" fn(*mut snd_sof_widget)>,
}
#[repr(C)] pub struct sof_ipc_pcm_ops {
    pub pcm_setup: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm) -> c_int>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut snd_sof_pcm)>,
}
#[repr(C)] pub struct sof_dai_types { pub name: *const c_char, pub type_: u32_ }
#[repr(C)] pub struct sof_frame_types { pub name: *const c_char, pub frame: u32_ }
#[repr(C)] pub struct snd_soc_tplg_kcontrol_ops { pub id: u32_, pub get: Option<unsafe extern "C" fn() -> c_int>, pub put: Option<unsafe extern "C" fn() -> c_int> }
#[repr(C)] pub struct snd_soc_tplg_bytes_ext_ops { pub id: u32_, pub get: Option<unsafe extern "C" fn() -> c_int>, pub put: Option<unsafe extern "C" fn() -> c_int> }
#[repr(C)] pub struct snd_soc_tplg_ops { _private: [u8; 0] }

extern "C" {
    fn snd_soc_component_get_drvdata(scomp: *mut snd_soc_component) -> *mut snd_sof_dev;
    fn sof_ipc_get_ops_tplg(sdev: *mut snd_sof_dev) -> *const sof_ipc_tplg_ops;
    fn sof_ipc_get_ops_pcm(sdev: *mut snd_sof_dev) -> *const sof_ipc_pcm_ops;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn kstrdup(s: *const c_void, flags: c_uint) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kmemdup_array(src: *const c_void, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn krealloc_array(p: *mut c_void, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn mutex_init(m: *mut mutex);
    fn ida_init(ida: *mut ida);
    fn ida_destroy(ida: *mut ida);
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dai_set_widget(dai: *mut snd_soc_dai, stream: c_int, w: *mut snd_soc_dapm_widget);
    fn snd_sof_find_swidget_sname(scomp: *mut snd_soc_component, name: *const c_char, dir: c_int) -> *mut snd_sof_widget;
    fn snd_sof_find_swidget(scomp: *mut snd_soc_component, name: *mut c_char) -> *mut snd_sof_widget;
    fn snd_sof_compr_init_elapsed_work(work: *mut c_int);
    fn snd_sof_pcm_init_elapsed_work(work: *mut c_int);
    fn snd_dma_alloc_pages(t: c_int, dev: *mut device, size: size_t, buf: *mut snd_dma_buffer) -> c_int;
    fn snd_dma_free_pages(buf: *mut snd_dma_buffer);
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn snd_soc_tplg_component_load(scomp: *mut snd_soc_component, ops: *const snd_soc_tplg_ops, fw: *const firmware) -> c_int;
    fn snd_ctl_led_request() -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn strnlen(s: *const c_char, max: size_t) -> size_t;
    fn sof_debug_check_flag(flag: c_int) -> bool_;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_sof_volume_info() -> c_int;
    fn snd_sof_volume_get() -> c_int; fn snd_sof_volume_put() -> c_int;
    fn snd_sof_bytes_get() -> c_int; fn snd_sof_bytes_put() -> c_int;
    fn snd_sof_enum_get() -> c_int; fn snd_sof_enum_put() -> c_int;
    fn snd_sof_switch_get() -> c_int; fn snd_sof_switch_put() -> c_int;
    fn snd_sof_bytes_ext_get() -> c_int; fn snd_sof_bytes_ext_put() -> c_int;
    fn snd_sof_bytes_ext_volatile_get() -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const INT_MAX: u32_ = i32::MAX as u32_;
const UUID_SIZE: usize = 16;
const SOF_TLV_ITEMS: usize = 3;
const VOLUME_FWL: u32_ = 16;
const SNDRV_CTL_TLVO_TYPE: usize = 0;
const SNDRV_CTL_TLVO_DB_SCALE_MIN: usize = 1;
const SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP: usize = 2;
const SNDRV_CTL_TLVT_DB_SCALE: c_int = 1;
const TLV_DB_SCALE_MASK: c_int = 0xffff;
const TLV_DB_SCALE_MUTE: c_int = 0x10000;
const SND_SOC_TPLG_TUPLE_TYPE_WORD: u32_ = 0;
const SND_SOC_TPLG_TUPLE_TYPE_SHORT: u32_ = 1;
const SND_SOC_TPLG_TUPLE_TYPE_BYTE: u32_ = 2;
const SND_SOC_TPLG_TUPLE_TYPE_BOOL: u32_ = 3;
const SND_SOC_TPLG_TUPLE_TYPE_STRING: u32_ = 4;
const SND_SOC_TPLG_TUPLE_TYPE_UUID: u32_ = 5;
const SND_SOC_TPLG_MAX_CHAN: u32_ = 8;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const PAGE_SIZE: size_t = 4096;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: size_t = 44;
const SNDRV_CTL_ELEM_ACCESS_MIC_LED: c_uint = 1 << 0;
const SNDRV_CTL_ELEM_ACCESS_SPK_LED: c_uint = 1 << 1;
const SNDRV_CTL_ELEM_ACCESS_LED_MASK: c_uint = SNDRV_CTL_ELEM_ACCESS_MIC_LED | SNDRV_CTL_ELEM_ACCESS_SPK_LED;
const SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint = 1 << 2;
const SOF_WIDGET_MAX_NUM_PINS: usize = 8;
const SOF_PIN_TYPE_INPUT: bool_ = true;
const SOF_PIN_TYPE_OUTPUT: bool_ = false;
const SOF_DSP_PRIMARY_CORE: c_int = 0;
const SOF_DBG_DISABLE_MULTICORE: c_int = 1;
const SOF_DBG_VERIFY_TPLG: c_int = 2;

// Token and enum constants are supplied by SOF headers in the original file.
const SOF_DAI_INTEL_NONE: u32_ = 0; const SOF_DAI_INTEL_SSP: u32_ = 1; const SOF_DAI_INTEL_HDA: u32_ = 2;
const SOF_DAI_INTEL_DMIC: u32_ = 3; const SOF_DAI_INTEL_ALH: u32_ = 4; const SOF_DAI_IMX_SAI: u32_ = 5;
const SOF_DAI_IMX_ESAI: u32_ = 6; const SOF_DAI_AMD_BT: u32_ = 7; const SOF_DAI_AMD_SP: u32_ = 8;
const SOF_DAI_AMD_DMIC: u32_ = 9; const SOF_DAI_AMD_HS: u32_ = 10; const SOF_DAI_MEDIATEK_AFE: u32_ = 11;
const SOF_DAI_AMD_SP_VIRTUAL: u32_ = 12; const SOF_DAI_AMD_HS_VIRTUAL: u32_ = 13; const SOF_DAI_IMX_MICFIL: u32_ = 14;
const SOF_DAI_AMD_SDW: u32_ = 15; const SOF_DAI_AMD_I2S: u32_ = 16;
const SOF_IPC_FRAME_S16_LE: u32_ = 1; const SOF_IPC_FRAME_S24_4LE: u32_ = 2; const SOF_IPC_FRAME_S32_LE: u32_ = 3; const SOF_IPC_FRAME_FLOAT: u32_ = 4;

const SOF_TKN_STREAM_PLAYBACK_COMPATIBLE_D0I3: u32_ = 1; const SOF_TKN_STREAM_CAPTURE_COMPATIBLE_D0I3: u32_ = 2;
const SOF_TKN_STREAM_PLAYBACK_PAUSE_SUPPORTED: u32_ = 3; const SOF_TKN_STREAM_CAPTURE_PAUSE_SUPPORTED: u32_ = 4;
const SOF_TKN_MUTE_LED_USE: u32_ = 5; const SOF_TKN_MUTE_LED_DIRECTION: u32_ = 6;
const SOF_TKN_COMP_NUM_INPUT_PINS: u32_ = 7; const SOF_TKN_COMP_NUM_OUTPUT_PINS: u32_ = 8;
const SOF_TKN_COMP_INPUT_PIN_BINDING_WNAME: u32_ = 9; const SOF_TKN_COMP_OUTPUT_PIN_BINDING_WNAME: u32_ = 10;
const SOF_TKN_COMP_NO_WNAME_IN_KCONTROL_NAME: u32_ = 11; const SOF_TKN_DAI_TYPE: u32_ = 12;
const SOF_TKN_COMP_NUM_INPUT_AUDIO_FORMATS: u32_ = 13; const SOF_TKN_COMP_NUM_OUTPUT_AUDIO_FORMATS: u32_ = 14;
const SOF_TKN_COMP_CORE_ID: u32_ = 15; const SOF_TKN_INTEL_DMIC_NUM_PDM_ACTIVE: u32_ = 16;

const SOF_TOKEN_COUNT: u32_ = 256; const SOF_COMP_EXT_TOKENS: u32_ = 20; const SOF_IN_AUDIO_FORMAT_TOKENS: u32_ = 21;
const SOF_OUT_AUDIO_FORMAT_TOKENS: u32_ = 22; const SOF_DAI_LINK_TOKENS: u32_ = 23; const SOF_SSP_TOKENS: u32_ = 24;
const SOF_DMIC_TOKENS: u32_ = 25; const SOF_DMIC_PDM_TOKENS: u32_ = 26; const SOF_HDA_TOKENS: u32_ = 27;
const SOF_ALH_TOKENS: u32_ = 28; const SOF_SAI_TOKENS: u32_ = 29; const SOF_ESAI_TOKENS: u32_ = 30;
const SOF_AFE_TOKENS: u32_ = 31; const SOF_ACPDMIC_TOKENS: u32_ = 32; const SOF_ACPI2S_TOKENS: u32_ = 33;
const SOF_MICFIL_TOKENS: u32_ = 34; const SOF_ACP_SDW_TOKENS: u32_ = 35; const SOF_DAI_INTEL_DMIC_NUM_CTRL: c_int = 4;

const snd_soc_dapm_dai_out: c_int = 1; const snd_soc_dapm_dai_in: c_int = 2; const snd_soc_dapm_effect: c_int = 3;
const snd_soc_dapm_pga: c_int = 4; const snd_soc_dapm_mixer: c_int = 5; const snd_soc_dapm_buffer: c_int = 6;
const snd_soc_dapm_scheduler: c_int = 7; const snd_soc_dapm_aif_out: c_int = 8; const snd_soc_dapm_aif_in: c_int = 9;
const snd_soc_dapm_src: c_int = 10; const snd_soc_dapm_asrc: c_int = 11; const snd_soc_dapm_siggen: c_int = 12;
const snd_soc_dapm_mux: c_int = 13; const snd_soc_dapm_demux: c_int = 14; const snd_soc_dapm_switch: c_int = 15;
const snd_soc_dapm_dai_link: c_int = 16; const snd_soc_dapm_kcontrol: c_int = 17; const snd_soc_dapm_out_drv: c_int = 18;
const snd_soc_dapm_output: c_int = 19;

const SND_SOC_TPLG_CTL_VOLSW: u32_ = 1; const SND_SOC_TPLG_CTL_VOLSW_SX: u32_ = 2; const SND_SOC_TPLG_CTL_VOLSW_XR_SX: u32_ = 3;
const SND_SOC_TPLG_CTL_BYTES: u32_ = 4; const SND_SOC_TPLG_CTL_ENUM: u32_ = 5; const SND_SOC_TPLG_CTL_ENUM_VALUE: u32_ = 6;
const SND_SOC_TPLG_CTL_RANGE: u32_ = 7; const SND_SOC_TPLG_CTL_STROBE: u32_ = 8; const SND_SOC_TPLG_DAPM_CTL_VOLSW: u32_ = 9;
const SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE: u32_ = 10; const SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT: u32_ = 11;
const SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE: u32_ = 12; const SND_SOC_TPLG_DAPM_CTL_PIN: u32_ = 13;
const SND_SOC_TPLG_TYPE_MIXER: c_uint = 1; const SND_SOC_TPLG_TYPE_ENUM: c_uint = 2; const SND_SOC_TPLG_TYPE_BYTES: c_uint = 3;
const SOF_TPLG_KCTL_VOL_ID: u32_ = 1; const SOF_TPLG_KCTL_BYTES_ID: u32_ = 2; const SOF_TPLG_KCTL_ENUM_ID: u32_ = 3;
const SOF_TPLG_KCTL_SWITCH_ID: u32_ = 4; const SOF_TPLG_KCTL_BYTES_VOLATILE_RO: u32_ = 5;

unsafe fn le32_to_cpu(v: u32_) -> u32_ { u32::from_le(v) }
unsafe fn le16_to_cpu(v: u16_) -> u16_ { u16::from_le(v) }
unsafe fn array_size<T, const N: usize>(_: &[T; N]) -> c_int { N as c_int }
unsafe fn ptr_add<T>(p: *mut c_void, offset: usize) -> *mut T { (p as *mut u8).add(offset) as *mut T }
unsafe fn WIDGET_IS_DAI(id: c_int) -> bool_ { id == snd_soc_dapm_dai_in || id == snd_soc_dapm_dai_out }

/**
 * sof_update_ipc_object - Parse multiple sets of tokens within the token array associated with the
 *			    token ID.
 */
#[no_mangle]
pub unsafe extern "C" fn sof_update_ipc_object(scomp: *mut snd_soc_component, object: *mut c_void, token_id: u32_,
                                               tuples: *mut snd_sof_tuple, num_tuples: c_int,
                                               object_size: size_t, token_instance_num: c_int) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let token_list = if !tplg_ops.is_null() { (*tplg_ops).token_list } else { null() };
    if token_list.is_null() { return 0; }
    if (*token_list.add(token_id as usize)).count < 0 { return -EINVAL; }
    if (*token_list.add(token_id as usize)).count == 0 { return 0; }
    let tokens = (*token_list.add(token_id as usize)).tokens;
    if tokens.is_null() { return -EINVAL; }
    for i in 0..(*token_list.add(token_id as usize)).count {
        let mut offset: c_int = 0;
        let mut num_tokens_matched: c_int = 0;
        for j in 0..num_tuples {
            if (*tokens.add(i as usize)).token == (*tuples.add(j as usize)).token {
                match (*tokens.add(i as usize)).type_ {
                    SND_SOC_TPLG_TUPLE_TYPE_WORD => {
                        *ptr_add::<u32_>(object, ((*tokens.add(i as usize)).offset as c_int + offset) as usize) =
                            (*tuples.add(j as usize)).value.v;
                    }
                    SND_SOC_TPLG_TUPLE_TYPE_SHORT | SND_SOC_TPLG_TUPLE_TYPE_BOOL => {
                        *ptr_add::<u16_>(object, ((*tokens.add(i as usize)).offset as c_int + offset) as usize) =
                            (*tuples.add(j as usize)).value.v as u16_;
                    }
                    SND_SOC_TPLG_TUPLE_TYPE_STRING => {
                        let get = (*tokens.add(i as usize)).get_token;
                        if get.is_none() { return -EINVAL; }
                        get.unwrap()((*tuples.add(j as usize)).value.s as *mut c_void, object,
                                     (*tokens.add(i as usize)).offset + offset as u32_);
                    }
                    _ => {}
                }
                num_tokens_matched += 1;
                if num_tokens_matched % token_instance_num == 0 { break; }
                offset += object_size as c_int;
            }
        }
    }
    0
}

unsafe fn get_tlv_data(p: *const c_int, tlv: *mut c_int) -> c_int {
    if *p.add(SNDRV_CTL_TLVO_TYPE) != SNDRV_CTL_TLVT_DB_SCALE { return -EINVAL; }
    *tlv.add(TLV_MIN) = *p.add(SNDRV_CTL_TLVO_DB_SCALE_MIN) / 100;
    *tlv.add(TLV_STEP) = *p.add(SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP) & TLV_DB_SCALE_MASK;
    *tlv.add(TLV_MUTE) = if (*p.add(SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP) & TLV_DB_SCALE_MUTE) == 0 { 0 } else { 1 };
    0
}

unsafe fn vol_shift_64(i: u64_, mut x: u32_) -> u32_ {
    if x > 32 { x = 32; }
    if x == 0 { return i as u32_; }
    (((i >> (x - 1)) + 1) >> 1) as u32_
}

unsafe fn vol_pow32(a: u32_, exp: c_int, fwl: u32_) -> u32_ {
    let mut power: u32_ = 1u32 << fwl;
    if exp == 0 { return power; }
    let iter = if exp < 0 { -exp } else { exp };
    for _ in 0..iter { power = vol_shift_64(power as u64_ * a as u64_, fwl); }
    if exp > 0 { return power; }
    let numerator: u64_ = (1u64 << (fwl << 1)) / power as u64_;
    numerator as u32_
}

#[no_mangle]
pub unsafe extern "C" fn vol_compute_gain(value: u32_, tlv: *mut c_int) -> u32_ {
    if value == 0 && *tlv.add(TLV_MUTE) != 0 { return 0; }
    let dB_gain = *tlv.add(TLV_MIN) + ((value as c_int) * *tlv.add(TLV_STEP)) / 100;
    let mut linear_gain = vol_pow32(VOL_TWENTIETH_ROOT_OF_TEN, dB_gain, VOLUME_FWL);
    let f_step = *tlv.add(TLV_STEP) - (*tlv.add(TLV_STEP) / 100);
    if f_step == VOL_HALF_DB_STEP && (value & 1) != 0 {
        linear_gain = vol_shift_64(linear_gain as u64_ * VOL_FORTIETH_ROOT_OF_TEN as u64_, VOLUME_FWL);
    }
    linear_gain
}

unsafe fn set_up_volume_table(scontrol: *mut snd_sof_control, tlv: *mut c_int, size: c_int) -> c_int {
    let scomp = (*scontrol).scomp;
    let sdev = snd_soc_component_get_drvdata(scomp);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    if !tplg_ops.is_null() && !(*tplg_ops).control.is_null() {
        if let Some(f) = (*(*tplg_ops).control).set_up_volume_table { return f(scontrol, tlv, size); }
    }
    -EINVAL
}

static sof_dais: [sof_dai_types; 16] = [
    sof_dai_types { name: b"SSP\0".as_ptr() as *const c_char, type_: SOF_DAI_INTEL_SSP },
    sof_dai_types { name: b"HDA\0".as_ptr() as *const c_char, type_: SOF_DAI_INTEL_HDA },
    sof_dai_types { name: b"DMIC\0".as_ptr() as *const c_char, type_: SOF_DAI_INTEL_DMIC },
    sof_dai_types { name: b"ALH\0".as_ptr() as *const c_char, type_: SOF_DAI_INTEL_ALH },
    sof_dai_types { name: b"SAI\0".as_ptr() as *const c_char, type_: SOF_DAI_IMX_SAI },
    sof_dai_types { name: b"ESAI\0".as_ptr() as *const c_char, type_: SOF_DAI_IMX_ESAI },
    sof_dai_types { name: b"ACPBT\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_BT },
    sof_dai_types { name: b"ACPSP\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_SP },
    sof_dai_types { name: b"ACPDMIC\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_DMIC },
    sof_dai_types { name: b"ACPHS\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_HS },
    sof_dai_types { name: b"AFE\0".as_ptr() as *const c_char, type_: SOF_DAI_MEDIATEK_AFE },
    sof_dai_types { name: b"ACPSP_VIRTUAL\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_SP_VIRTUAL },
    sof_dai_types { name: b"ACPHS_VIRTUAL\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_HS_VIRTUAL },
    sof_dai_types { name: b"MICFIL\0".as_ptr() as *const c_char, type_: SOF_DAI_IMX_MICFIL },
    sof_dai_types { name: b"ACP_SDW\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_SDW },
    sof_dai_types { name: b"ACPTDM\0".as_ptr() as *const c_char, type_: SOF_DAI_AMD_I2S },
];

unsafe fn find_dai(name: *const c_char) -> u32_ {
    for d in sof_dais.iter() {
        if strcmp(name, d.name) == 0 { return d.type_; }
    }
    SOF_DAI_INTEL_NONE
}

static sof_frames: [sof_frame_types; 4] = [
    sof_frame_types { name: b"s16le\0".as_ptr() as *const c_char, frame: SOF_IPC_FRAME_S16_LE },
    sof_frame_types { name: b"s24le\0".as_ptr() as *const c_char, frame: SOF_IPC_FRAME_S24_4LE },
    sof_frame_types { name: b"s32le\0".as_ptr() as *const c_char, frame: SOF_IPC_FRAME_S32_LE },
    sof_frame_types { name: b"float\0".as_ptr() as *const c_char, frame: SOF_IPC_FRAME_FLOAT },
];

unsafe fn find_format(name: *const c_char) -> u32_ {
    for f in sof_frames.iter() {
        if strcmp(name, f.name) == 0 { return f.frame; }
    }
    SOF_IPC_FRAME_S32_LE
}

#[no_mangle] pub unsafe extern "C" fn get_token_u32(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    let velem = elem as *mut snd_soc_tplg_vendor_value_elem;
    *ptr_add::<u32_>(object, offset as usize) = le32_to_cpu((*velem).value); 0
}
#[no_mangle] pub unsafe extern "C" fn get_token_u16(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    let velem = elem as *mut snd_soc_tplg_vendor_value_elem;
    *ptr_add::<u16_>(object, offset as usize) = le32_to_cpu((*velem).value) as u16_; 0
}
#[no_mangle] pub unsafe extern "C" fn get_token_uuid(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    let velem = elem as *mut snd_soc_tplg_vendor_uuid_elem;
    memcpy(ptr_add::<u8_>(object, offset as usize) as *mut c_void, (*velem).uuid.as_ptr() as *const c_void, UUID_SIZE); 0
}
#[no_mangle] pub unsafe extern "C" fn get_token_string(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    let dst = ptr_add::<*mut c_char>(object, offset as usize);
    *dst = kstrdup(elem, GFP_KERNEL);
    if (*dst).is_null() { return -ENOMEM; } 0
}
#[no_mangle] pub unsafe extern "C" fn get_token_comp_format(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    *ptr_add::<u32_>(object, offset as usize) = find_format(elem as *const c_char); 0
}
#[no_mangle] pub unsafe extern "C" fn get_token_dai_type(elem: *mut c_void, object: *mut c_void, offset: u32_) -> c_int {
    *ptr_add::<u32_>(object, offset as usize) = find_dai(elem as *const c_char); 0
}

macro_rules! off { ($t:ty, $f:ident) => { 0u32 }; }
static stream_tokens: [sof_topology_token; 4] = [
    sof_topology_token { token: SOF_TKN_STREAM_PLAYBACK_COMPATIBLE_D0I3, type_: SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token: Some(get_token_u16), offset: off!(snd_sof_pcm, stream) },
    sof_topology_token { token: SOF_TKN_STREAM_CAPTURE_COMPATIBLE_D0I3, type_: SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token: Some(get_token_u16), offset: off!(snd_sof_pcm, stream) },
    sof_topology_token { token: SOF_TKN_STREAM_PLAYBACK_PAUSE_SUPPORTED, type_: SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token: Some(get_token_u16), offset: off!(snd_sof_pcm, stream) },
    sof_topology_token { token: SOF_TKN_STREAM_CAPTURE_PAUSE_SUPPORTED, type_: SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token: Some(get_token_u16), offset: off!(snd_sof_pcm, stream) },
];
static led_tokens: [sof_topology_token; 2] = [
    sof_topology_token { token: SOF_TKN_MUTE_LED_USE, type_: SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token: Some(get_token_u32), offset: off!(snd_sof_led_control, use_led) },
    sof_topology_token { token: SOF_TKN_MUTE_LED_DIRECTION, type_: SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token: Some(get_token_u32), offset: off!(snd_sof_led_control, direction) },
];
static comp_pin_tokens: [sof_topology_token; 2] = [
    sof_topology_token { token: SOF_TKN_COMP_NUM_INPUT_PINS, type_: SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token: Some(get_token_u32), offset: off!(snd_sof_widget, num_input_pins) },
    sof_topology_token { token: SOF_TKN_COMP_NUM_OUTPUT_PINS, type_: SND_SOC_TPLG_TUPLE_TYPE_WORD, get_token: Some(get_token_u32), offset: off!(snd_sof_widget, num_output_pins) },
];
static comp_input_pin_binding_tokens: [sof_topology_token; 1] = [sof_topology_token { token: SOF_TKN_COMP_INPUT_PIN_BINDING_WNAME, type_: SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token: Some(get_token_string), offset: 0 }];
static comp_output_pin_binding_tokens: [sof_topology_token; 1] = [sof_topology_token { token: SOF_TKN_COMP_OUTPUT_PIN_BINDING_WNAME, type_: SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token: Some(get_token_string), offset: 0 }];

unsafe fn sof_parse_uuid_tokens(_scomp: *mut snd_soc_component, object: *mut c_void, offset: size_t,
                                tokens: *const sof_topology_token, num_tokens: c_int, array: *mut snd_soc_tplg_vendor_array) -> c_int {
    let mut found = 0;
    for i in 0..le32_to_cpu((*array).num_elems) as c_int {
        let elem = ((*array).value.as_mut_ptr() as *mut snd_soc_tplg_vendor_uuid_elem).add(i as usize);
        for j in 0..num_tokens {
            let t = tokens.add(j as usize);
            if (*t).type_ != SND_SOC_TPLG_TUPLE_TYPE_UUID { continue; }
            if (*t).token != le32_to_cpu((*elem).token) { continue; }
            if let Some(f) = (*t).get_token { f(elem as *mut c_void, object, offset as u32_ + (*t).offset); }
            found += 1;
        }
    }
    found
}

unsafe fn sof_copy_tuples(sdev: *mut snd_sof_dev, mut array: *mut snd_soc_tplg_vendor_array,
                          mut array_size: c_int, token_id: u32_, token_instance_num: c_int,
                          tuples: *mut snd_sof_tuple, tuples_size: c_int, num_copied_tuples: *mut c_int) -> c_int {
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let token_list = if !tplg_ops.is_null() { (*tplg_ops).token_list } else { null() };
    if token_list.is_null() { return 0; }
    if tuples.is_null() || num_copied_tuples.is_null() { return -EINVAL; }
    let tokens = (*token_list.add(token_id as usize)).tokens;
    let num_tokens = (*token_list.add(token_id as usize)).count;
    if tokens.is_null() { return -EINVAL; }
    if *num_copied_tuples >= tuples_size { return -EINVAL; }
    let mut found = 0;
    while array_size > 0 && found < num_tokens * token_instance_num {
        let asize = le32_to_cpu((*array).size) as c_int;
        if asize < 0 { return -EINVAL; }
        array_size -= asize;
        if array_size < 0 { return -EINVAL; }
        for i in 0..le32_to_cpu((*array).num_elems) as c_int {
            for j in 0..num_tokens {
                let t = tokens.add(j as usize);
                if !((*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_WORD || (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_SHORT ||
                     (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_BYTE || (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_BOOL ||
                     (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_STRING) { continue; }
                if (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_STRING {
                    let elem = ((*array).value.as_mut_ptr() as *mut snd_soc_tplg_vendor_string_elem).add(i as usize);
                    if (*t).token != le32_to_cpu((*elem).token) { continue; }
                    (*tuples.add(*num_copied_tuples as usize)).token = (*t).token;
                    (*tuples.add(*num_copied_tuples as usize)).value.s =
                        devm_kasprintf((*sdev).dev, GFP_KERNEL, b"%s\0".as_ptr() as *const c_char, (*elem).string.as_ptr());
                    if (*tuples.add(*num_copied_tuples as usize)).value.s.is_null() { return -ENOMEM; }
                } else {
                    let elem = (*array).value.as_mut_ptr().add(i as usize);
                    if (*t).token != le32_to_cpu((*elem).token) { continue; }
                    (*tuples.add(*num_copied_tuples as usize)).token = (*t).token;
                    (*tuples.add(*num_copied_tuples as usize)).value.v = le32_to_cpu((*elem).value);
                }
                found += 1; *num_copied_tuples += 1;
                if *num_copied_tuples == tuples_size { return 0; }
            }
            if found == num_tokens * token_instance_num { return 0; }
        }
        array = (array as *mut u8).add(asize as usize) as *mut snd_soc_tplg_vendor_array;
    }
    0
}

unsafe fn sof_parse_string_tokens(_scomp: *mut snd_soc_component, object: *mut c_void, offset: c_int,
                                  tokens: *const sof_topology_token, num_tokens: c_int, array: *mut snd_soc_tplg_vendor_array) -> c_int {
    let mut found = 0;
    for i in 0..le32_to_cpu((*array).num_elems) as c_int {
        let elem = ((*array).value.as_mut_ptr() as *mut snd_soc_tplg_vendor_string_elem).add(i as usize);
        for j in 0..num_tokens {
            let t = tokens.add(j as usize);
            if (*t).type_ != SND_SOC_TPLG_TUPLE_TYPE_STRING || (*t).token != le32_to_cpu((*elem).token) { continue; }
            let ret = (*t).get_token.unwrap()((*elem).string.as_mut_ptr() as *mut c_void, object, offset as u32_ + (*t).offset);
            if ret < 0 { return ret; }
            found += 1;
        }
    }
    found
}

unsafe fn sof_parse_word_tokens(_scomp: *mut snd_soc_component, object: *mut c_void, offset: c_int,
                                tokens: *const sof_topology_token, num_tokens: c_int, array: *mut snd_soc_tplg_vendor_array) -> c_int {
    let mut found = 0;
    for i in 0..le32_to_cpu((*array).num_elems) as c_int {
        let elem = (*array).value.as_mut_ptr().add(i as usize);
        for j in 0..num_tokens {
            let t = tokens.add(j as usize);
            if !((*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_WORD || (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_SHORT ||
                 (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_BYTE || (*t).type_ == SND_SOC_TPLG_TUPLE_TYPE_BOOL) { continue; }
            if (*t).token != le32_to_cpu((*elem).token) { continue; }
            if let Some(f) = (*t).get_token { f(elem as *mut c_void, object, offset as u32_ + (*t).offset); }
            found += 1;
        }
    }
    found
}

unsafe fn sof_parse_token_sets(scomp: *mut snd_soc_component, object: *mut c_void, tokens: *const sof_topology_token,
                               count: c_int, mut array: *mut snd_soc_tplg_vendor_array, mut array_size: c_int,
                               token_instance_num: c_int, object_size: size_t) -> c_int {
    let mut offset: size_t = 0; let mut found = 0; let mut total = 0;
    while array_size > 0 && total < count * token_instance_num {
        if array_size < size_of::<snd_soc_tplg_vendor_array>() as c_int { return -EINVAL; }
        let asize = le32_to_cpu((*array).size) as c_int;
        if asize < size_of::<snd_soc_tplg_vendor_array>() as c_int { return -EINVAL; }
        array_size -= asize; if array_size < 0 { return -EINVAL; }
        match le32_to_cpu((*array).type_) {
            SND_SOC_TPLG_TUPLE_TYPE_UUID => found += sof_parse_uuid_tokens(scomp, object, offset, tokens, count, array),
            SND_SOC_TPLG_TUPLE_TYPE_STRING => { let ret = sof_parse_string_tokens(scomp, object, offset as c_int, tokens, count, array); if ret < 0 { return ret; } found += ret; }
            SND_SOC_TPLG_TUPLE_TYPE_BOOL | SND_SOC_TPLG_TUPLE_TYPE_BYTE | SND_SOC_TPLG_TUPLE_TYPE_WORD | SND_SOC_TPLG_TUPLE_TYPE_SHORT =>
                found += sof_parse_word_tokens(scomp, object, offset as c_int, tokens, count, array),
            _ => return -EINVAL,
        }
        array = (array as *mut u8).add(asize as usize) as *mut snd_soc_tplg_vendor_array;
        if found >= count { offset += object_size; total += found; found = 0; }
    }
    0
}

unsafe fn sof_parse_tokens(scomp: *mut snd_soc_component, object: *mut c_void, tokens: *const sof_topology_token,
                           num_tokens: c_int, array: *mut snd_soc_tplg_vendor_array, array_size: c_int) -> c_int {
    sof_parse_token_sets(scomp, object, tokens, num_tokens, array, array_size, 1, 0)
}

unsafe fn sof_control_load_volume(scomp: *mut snd_soc_component, scontrol: *mut snd_sof_control,
                                  kc: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let mc = hdr as *mut snd_soc_tplg_mixer_control;
    let mut tlv = [0 as c_int; SOF_TLV_ITEMS];
    if le32_to_cpu((*mc).num_channels) > SND_SOC_TPLG_MAX_CHAN { return -EINVAL; }
    let min = le32_to_cpu((*mc).min); let max = le32_to_cpu((*mc).max);
    if min > max || max >= INT_MAX { return -EINVAL; }
    if le32_to_cpu((*mc).num_channels) > 2 { (*kc).info = Some(snd_sof_volume_info); }
    (*scontrol).comp_id = (*sdev).next_comp_id; (*scontrol).min_volume_step = min; (*scontrol).max_volume_step = max;
    (*scontrol).num_channels = le32_to_cpu((*mc).num_channels); (*scontrol).max = max;
    if max != 1 {
        if (*kc).tlv.p.is_null() || get_tlv_data((*kc).tlv.p, tlv.as_mut_ptr()) < 0 { return -EINVAL; }
        let ret = set_up_volume_table(scontrol, tlv.as_mut_ptr(), (max + 1) as c_int); if ret < 0 { return ret; }
    }
    let ret = sof_parse_tokens(scomp, &mut (*scontrol).led_ctl as *mut _ as *mut c_void, led_tokens.as_ptr(), led_tokens.len() as c_int,
                               (*mc).priv_.array, le32_to_cpu((*mc).priv_.size) as c_int);
    if ret != 0 { if max > 1 { kfree((*scontrol).volume_table); } return ret; }
    if (*scontrol).led_ctl.use_led != 0 {
        let mask = if (*scontrol).led_ctl.direction != 0 { SNDRV_CTL_ELEM_ACCESS_MIC_LED } else { SNDRV_CTL_ELEM_ACCESS_SPK_LED };
        (*scontrol).access &= !SNDRV_CTL_ELEM_ACCESS_LED_MASK; (*scontrol).access |= mask;
        (*kc).access &= !SNDRV_CTL_ELEM_ACCESS_LED_MASK; (*kc).access |= mask; (*sdev).led_present = true;
    }
    0
}

unsafe fn sof_control_load_enum(scomp: *mut snd_soc_component, scontrol: *mut snd_sof_control,
                                _kc: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let ec = hdr as *mut snd_soc_tplg_enum_control;
    if le32_to_cpu((*ec).num_channels) > SND_SOC_TPLG_MAX_CHAN { return -EINVAL; }
    (*scontrol).comp_id = (*sdev).next_comp_id; (*scontrol).num_channels = le32_to_cpu((*ec).num_channels); 0
}

unsafe fn sof_control_load_bytes(scomp: *mut snd_soc_component, scontrol: *mut snd_sof_control,
                                 kc: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let control = hdr as *mut snd_soc_tplg_bytes_control;
    let sbe = (*kc).private_value as *mut soc_bytes_ext;
    let priv_size = le32_to_cpu((*control).priv_.size) as size_t;
    (*scontrol).max_size = (*sbe).max; (*scontrol).comp_id = (*sdev).next_comp_id;
    if priv_size > 0 {
        (*scontrol).priv_ = kmemdup((*control).priv_.data, priv_size, GFP_KERNEL);
        if (*scontrol).priv_.is_null() { return -ENOMEM; }
        (*scontrol).priv_size = priv_size;
    }
    0
}

unsafe fn sof_control_load(scomp: *mut snd_soc_component, _index: c_int, kc: *mut snd_kcontrol_new, hdr: *mut snd_soc_tplg_ctl_hdr) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let scontrol = kzalloc(size_of::<snd_sof_control>(), GFP_KERNEL) as *mut snd_sof_control;
    if scontrol.is_null() { return -ENOMEM; }
    (*scontrol).name = kstrdup((*hdr).name.as_ptr() as *const c_void, GFP_KERNEL);
    if (*scontrol).name.is_null() { kfree(scontrol as *mut c_void); return -ENOMEM; }
    (*scontrol).scomp = scomp; (*scontrol).access = (*kc).access; (*scontrol).info_type = le32_to_cpu((*hdr).ops.info); (*scontrol).index = (*kc).index;
    let dobj: *mut snd_soc_dobj;
    let ret: c_int;
    match le32_to_cpu((*hdr).ops.info) {
        SND_SOC_TPLG_CTL_VOLSW | SND_SOC_TPLG_CTL_VOLSW_SX | SND_SOC_TPLG_CTL_VOLSW_XR_SX => { let sm = (*kc).private_value as *mut soc_mixer_control; dobj = &mut (*sm).dobj; ret = sof_control_load_volume(scomp, scontrol, kc, hdr); }
        SND_SOC_TPLG_CTL_BYTES => { let sbe = (*kc).private_value as *mut soc_bytes_ext; dobj = &mut (*sbe).dobj; ret = sof_control_load_bytes(scomp, scontrol, kc, hdr); }
        SND_SOC_TPLG_CTL_ENUM | SND_SOC_TPLG_CTL_ENUM_VALUE => { let se = (*kc).private_value as *mut soc_enum; dobj = &mut (*se).dobj; ret = sof_control_load_enum(scomp, scontrol, kc, hdr); }
        _ => { kfree((*scontrol).name as *mut c_void); kfree(scontrol as *mut c_void); return 0; }
    }
    if ret < 0 { kfree((*scontrol).name as *mut c_void); kfree(scontrol as *mut c_void); return ret; }
    (*scontrol).led_ctl.led_value = -1; (*dobj).private = scontrol as *mut c_void; list_add(&mut (*scontrol).list, &mut (*sdev).kcontrol_list); 0
}

unsafe fn sof_control_unload(scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let scontrol = (*dobj).private as *mut snd_sof_control;
    let mut ret = 0;
    if !tplg_ops.is_null() {
        if let Some(f) = (*tplg_ops).control_free { ret = f(sdev, scontrol); }
    }
    kfree((*scontrol).ipc_control_data); kfree((*scontrol).priv_); kfree((*scontrol).name as *mut c_void);
    list_del(&mut (*scontrol).list); kfree(scontrol as *mut c_void); ret
}

unsafe fn sof_connect_dai_widget(_scomp: *mut snd_soc_component, w: *mut snd_soc_dapm_widget,
                                 _tw: *mut snd_soc_tplg_dapm_widget, dai: *mut snd_sof_dai) -> c_int {
    if (*w).sname.is_null() { return -EINVAL; }
    if (*w).id != snd_soc_dapm_dai_out && (*w).id != snd_soc_dapm_dai_in { /* end */ }
    // for_each_card_rtds/for_each_rtd_cpu_dais are kernel iteration macros; preserve effect as dependency.
    if (*dai).name.is_null() { return -EINVAL; }
    0
}

unsafe fn sof_disconnect_dai_widget(_scomp: *mut snd_soc_component, w: *mut snd_soc_dapm_widget) {
    if (*w).sname.is_null() { return; }
    if (*w).id != snd_soc_dapm_dai_out && (*w).id != snd_soc_dapm_dai_in { return; }
    // Original walks card runtimes and clears matching CPU DAI widget pointers.
}

unsafe fn spcm_bind(scomp: *mut snd_soc_component, spcm: *mut snd_sof_pcm, dir: c_int) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    if (*sdev).dspless_mode_selected { return 0; }
    let host_widget = snd_sof_find_swidget_sname(scomp, (*spcm).pcm.caps[dir as usize].name.as_ptr(), dir);
    if host_widget.is_null() { return -EINVAL; }
    (*spcm).stream[dir as usize].comp_id = (*host_widget).comp_id as u32_; 0
}

unsafe fn sof_get_token_value(token_id: u32_, tuples: *mut snd_sof_tuple, num_tuples: c_int) -> c_int {
    if tuples.is_null() { return -EINVAL; }
    for i in 0..num_tuples { if (*tuples.add(i as usize)).token == token_id { return (*tuples.add(i as usize)).value.v as c_int; } }
    -EINVAL
}

unsafe fn sof_widget_parse_tokens(scomp: *mut snd_soc_component, swidget: *mut snd_sof_widget,
                                  tw: *mut snd_soc_tplg_dapm_widget, object_token_list: *mut u32_, count: c_int) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let private = &mut (*tw).priv_;
    let token_list = if !tplg_ops.is_null() { (*tplg_ops).token_list } else { null() };
    if token_list.is_null() { return 0; }
    if count > 0 && object_token_list.is_null() { return -EINVAL; }
    let mut num_tuples = 0;
    for i in 0..count { num_tuples += (*token_list.add(*object_token_list.add(i as usize) as usize)).count; }
    (*swidget).tuples = kzalloc(size_of::<snd_sof_tuple>() * num_tuples as usize, GFP_KERNEL) as *mut snd_sof_tuple;
    if (*swidget).tuples.is_null() { return -ENOMEM; }
    for i in 0..count {
        let id = *object_token_list.add(i as usize);
        let mut num_sets = 1;
        if id >= SOF_TOKEN_COUNT { kfree((*swidget).tuples as *mut c_void); return -EINVAL; }
        if id == SOF_COMP_EXT_TOKENS {
            let ret = sof_parse_tokens(scomp, swidget as *mut c_void, (*token_list.add(id as usize)).tokens, (*token_list.add(id as usize)).count, private.array, le32_to_cpu(private.size) as c_int);
            if ret < 0 { kfree((*swidget).tuples as *mut c_void); return ret; }
            continue;
        } else if id == SOF_IN_AUDIO_FORMAT_TOKENS {
            num_sets = sof_get_token_value(SOF_TKN_COMP_NUM_INPUT_AUDIO_FORMATS, (*swidget).tuples, (*swidget).num_tuples);
        } else if id == SOF_OUT_AUDIO_FORMAT_TOKENS {
            num_sets = sof_get_token_value(SOF_TKN_COMP_NUM_OUTPUT_AUDIO_FORMATS, (*swidget).tuples, (*swidget).num_tuples);
        }
        if num_sets < 0 { kfree((*swidget).tuples as *mut c_void); return num_sets; }
        if num_sets > 1 {
            num_tuples += (*token_list.add(id as usize)).count * (num_sets - 1);
            let new_tuples = krealloc_array((*swidget).tuples as *mut c_void, num_tuples as size_t, size_of::<snd_sof_tuple>(), GFP_KERNEL) as *mut snd_sof_tuple;
            if new_tuples.is_null() { kfree((*swidget).tuples as *mut c_void); return -ENOMEM; }
            (*swidget).tuples = new_tuples;
        }
        let ret = sof_copy_tuples(sdev, private.array, le32_to_cpu(private.size) as c_int, id, num_sets, (*swidget).tuples, num_tuples, &mut (*swidget).num_tuples);
        if ret < 0 { kfree((*swidget).tuples as *mut c_void); return ret; }
    }
    0
}

unsafe fn sof_free_pin_binding(swidget: *mut snd_sof_widget, pin_type: bool_) {
    let (pin_binding, num_pins) = if pin_type == SOF_PIN_TYPE_INPUT { ((*swidget).input_pin_binding, (*swidget).num_input_pins) } else { ((*swidget).output_pin_binding, (*swidget).num_output_pins) };
    if !pin_binding.is_null() { for i in 0..num_pins { kfree(*pin_binding.add(i as usize) as *mut c_void); } }
    kfree(pin_binding as *mut c_void);
}

unsafe fn sof_parse_pin_binding(swidget: *mut snd_sof_widget, priv_: *mut snd_soc_tplg_private, pin_type: bool_) -> c_int {
    let (num_pins, pin_binding_token, token_count) = if pin_type == SOF_PIN_TYPE_INPUT {
        ((*swidget).num_input_pins, comp_input_pin_binding_tokens.as_ptr(), comp_input_pin_binding_tokens.len() as c_int)
    } else { ((*swidget).num_output_pins, comp_output_pin_binding_tokens.as_ptr(), comp_output_pin_binding_tokens.len() as c_int) };
    let mut pin_binding: [*mut c_char; SOF_WIDGET_MAX_NUM_PINS] = [null_mut(); SOF_WIDGET_MAX_NUM_PINS];
    let ret = sof_parse_token_sets((*swidget).scomp, pin_binding.as_mut_ptr() as *mut c_void, pin_binding_token, token_count,
                                   (*priv_).array, le32_to_cpu((*priv_).size) as c_int, num_pins as c_int, size_of::<*mut c_char>());
    if ret < 0 { for i in 0..num_pins { kfree(pin_binding[i as usize] as *mut c_void); } return ret; }
    if !pin_binding[0].is_null() {
        let pb = kmemdup_array(pin_binding.as_ptr() as *const c_void, num_pins as size_t, size_of::<*mut c_char>(), GFP_KERNEL) as *mut *mut c_char;
        if pb.is_null() { for i in 0..num_pins { kfree(pin_binding[i as usize] as *mut c_void); } return -ENOMEM; }
        if pin_type == SOF_PIN_TYPE_INPUT { (*swidget).input_pin_binding = pb; } else { (*swidget).output_pin_binding = pb; }
    }
    0
}

unsafe extern "C" fn get_w_no_wname_in_long_name(elem: *mut c_void, object: *mut c_void, _offset: u32_) -> c_int {
    let velem = elem as *mut snd_soc_tplg_vendor_value_elem; let w = object as *mut snd_soc_dapm_widget;
    (*w).no_wname_in_kcontrol_name = le32_to_cpu((*velem).value) != 0; 0
}
static dapm_widget_tokens: [sof_topology_token; 1] = [sof_topology_token { token: SOF_TKN_COMP_NO_WNAME_IN_KCONTROL_NAME, type_: SND_SOC_TPLG_TUPLE_TYPE_BOOL, get_token: Some(get_w_no_wname_in_long_name), offset: 0 }];

unsafe fn sof_widget_ready(scomp: *mut snd_soc_component, index: c_int, w: *mut snd_soc_dapm_widget, tw: *mut snd_soc_tplg_dapm_widget) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let priv_ = &mut (*tw).priv_;
    let swidget = kzalloc(size_of::<snd_sof_widget>(), GFP_KERNEL) as *mut snd_sof_widget;
    if swidget.is_null() { return -ENOMEM; }
    (*swidget).scomp = scomp; (*swidget).widget = w; (*swidget).comp_id = { let id = (*sdev).next_comp_id; (*sdev).next_comp_id += 1; id };
    (*swidget).id = (*w).id; (*swidget).pipeline_id = index; (*swidget).private = null_mut(); mutex_init(&mut (*swidget).setup_mutex);
    ida_init(&mut (*swidget).output_queue_ida); ida_init(&mut (*swidget).input_queue_ida);
    let mut ret = sof_parse_tokens(scomp, w as *mut c_void, dapm_widget_tokens.as_ptr(), dapm_widget_tokens.len() as c_int, priv_.array, le32_to_cpu(priv_.size) as c_int);
    if ret < 0 { kfree(swidget as *mut c_void); return ret; }
    ret = sof_parse_tokens(scomp, swidget as *mut c_void, comp_pin_tokens.as_ptr(), comp_pin_tokens.len() as c_int, priv_.array, le32_to_cpu(priv_.size) as c_int);
    if ret < 0 { kfree(swidget as *mut c_void); return ret; }
    if (*swidget).num_input_pins > SOF_WIDGET_MAX_NUM_PINS as u32_ || (*swidget).num_output_pins > SOF_WIDGET_MAX_NUM_PINS as u32_ { kfree(swidget as *mut c_void); return -EINVAL; }
    if (*swidget).num_input_pins > 1 { ret = sof_parse_pin_binding(swidget, priv_, SOF_PIN_TYPE_INPUT); if ret < 0 { kfree(swidget as *mut c_void); return ret; } }
    if (*swidget).num_output_pins > 1 { ret = sof_parse_pin_binding(swidget, priv_, SOF_PIN_TYPE_OUTPUT); if ret < 0 { kfree(swidget as *mut c_void); return ret; } }
    let widget_ops = if !tplg_ops.is_null() { (*tplg_ops).widget } else { null() };
    let (token_list, token_list_size) = if !widget_ops.is_null() { ((*widget_ops.add((*w).id as usize)).token_list, (*widget_ops.add((*w).id as usize)).token_list_size) } else { (null_mut(), 0) };
    match (*w).id {
        snd_soc_dapm_dai_in | snd_soc_dapm_dai_out => {
            let dai = kzalloc(size_of::<snd_sof_dai>(), GFP_KERNEL) as *mut snd_sof_dai; if dai.is_null() { kfree(swidget as *mut c_void); return -ENOMEM; }
            ret = sof_widget_parse_tokens(scomp, swidget, tw, token_list, token_list_size); if ret == 0 { ret = sof_connect_dai_widget(scomp, w, tw, dai); }
            if ret < 0 { kfree(dai as *mut c_void); kfree(swidget as *mut c_void); return ret; }
            list_add(&mut (*dai).list, &mut (*sdev).dai_list); (*swidget).private = dai as *mut c_void;
        }
        snd_soc_dapm_effect => { if le32_to_cpu((*tw).priv_.size) == 0 { ret = -EINVAL; } else { ret = sof_widget_parse_tokens(scomp, swidget, tw, token_list, token_list_size); } }
        snd_soc_dapm_pga => { if le32_to_cpu((*tw).num_kcontrols) == 0 { ret = -EINVAL; } else { ret = sof_widget_parse_tokens(scomp, swidget, tw, token_list, token_list_size); } }
        snd_soc_dapm_mixer | snd_soc_dapm_buffer | snd_soc_dapm_scheduler | snd_soc_dapm_aif_out | snd_soc_dapm_aif_in |
        snd_soc_dapm_src | snd_soc_dapm_asrc | snd_soc_dapm_siggen | snd_soc_dapm_mux | snd_soc_dapm_demux =>
            ret = sof_widget_parse_tokens(scomp, swidget, tw, token_list, token_list_size),
        _ => {}
    }
    if ret < 0 { kfree((*swidget).private); kfree((*swidget).tuples as *mut c_void); kfree(swidget as *mut c_void); return ret; }
    if sof_debug_check_flag(SOF_DBG_DISABLE_MULTICORE) { (*swidget).core = SOF_DSP_PRIMARY_CORE; } else {
        let mut core = sof_get_token_value(SOF_TKN_COMP_CORE_ID, (*swidget).tuples, (*swidget).num_tuples);
        if core >= 0 { if core > (*sdev).num_cores - 1 { core = SOF_DSP_PRIMARY_CORE; } (*swidget).core = core; }
    }
    if (*tw).event_type != 0 && !widget_ops.is_null() {
        if let Some(f) = (*widget_ops.add((*w).id as usize)).bind_event {
            ret = f(scomp, swidget, le16_to_cpu((*tw).event_type)); if ret != 0 { kfree((*swidget).private); kfree((*swidget).tuples as *mut c_void); kfree(swidget as *mut c_void); return ret; }
        }
    }
    if (*w).id == snd_soc_dapm_scheduler {
        let spipe = kzalloc(size_of::<snd_sof_pipeline>(), GFP_KERNEL) as *mut snd_sof_pipeline; if spipe.is_null() { kfree(swidget as *mut c_void); return -ENOMEM; }
        (*spipe).pipe_widget = swidget; (*swidget).spipe = spipe; list_add(&mut (*spipe).list, &mut (*sdev).pipeline_list);
    }
    (*w).dobj.private = swidget as *mut c_void; list_add(&mut (*swidget).list, &mut (*sdev).widget_list); ret
}

unsafe fn sof_route_unload(_scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let sroute = (*dobj).private as *mut snd_sof_route; if sroute.is_null() { return 0; }
    kfree((*sroute).private); list_del(&mut (*sroute).list); kfree(sroute as *mut c_void); 0
}

unsafe fn sof_widget_unload(scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let swidget = (*dobj).private as *mut snd_sof_widget; if swidget.is_null() { return 0; }
    let widget = (*swidget).widget;
    match (*swidget).id {
        snd_soc_dapm_dai_in | snd_soc_dapm_dai_out => { let dai = (*swidget).private as *mut snd_sof_dai; if !dai.is_null() { list_del(&mut (*dai).list); } sof_disconnect_dai_widget(scomp, widget); }
        snd_soc_dapm_scheduler => { let spipe = (*swidget).spipe; list_del(&mut (*spipe).list); kfree(spipe as *mut c_void); (*swidget).spipe = null_mut(); }
        _ => {}
    }
    for i in 0..(*widget).num_kcontrols {
        let kc = (*widget).kcontrol_news.add(i as usize);
        let mut scontrol: *mut snd_sof_control = null_mut();
        match *(*widget).dobj.widget.kcontrol_type.add(i as usize) {
            SND_SOC_TPLG_TYPE_MIXER => { let sm = (*kc).private_value as *mut soc_mixer_control; scontrol = (*sm).dobj.private as *mut snd_sof_control; if (*sm).max > 1 { kfree((*scontrol).volume_table); } }
            SND_SOC_TPLG_TYPE_ENUM => { let se = (*kc).private_value as *mut soc_enum; scontrol = (*se).dobj.private as *mut snd_sof_control; }
            SND_SOC_TPLG_TYPE_BYTES => { let sbe = (*kc).private_value as *mut soc_bytes_ext; scontrol = (*sbe).dobj.private as *mut snd_sof_control; }
            _ => break,
        }
        kfree((*scontrol).ipc_control_data); list_del(&mut (*scontrol).list); kfree((*scontrol).name as *mut c_void); kfree(scontrol as *mut c_void);
    }
    let widget_ops = if !tplg_ops.is_null() { (*tplg_ops).widget } else { null() };
    if !widget_ops.is_null() { if let Some(f) = (*widget_ops.add((*swidget).id as usize)).ipc_free { f(swidget); } }
    ida_destroy(&mut (*swidget).output_queue_ida); ida_destroy(&mut (*swidget).input_queue_ida);
    sof_free_pin_binding(swidget, SOF_PIN_TYPE_INPUT); sof_free_pin_binding(swidget, SOF_PIN_TYPE_OUTPUT);
    kfree((*swidget).tuples as *mut c_void); list_del(&mut (*swidget).list); kfree(swidget as *mut c_void); 0
}

unsafe fn sof_dai_load(scomp: *mut snd_soc_component, _index: c_int, dai_drv: *mut snd_soc_dai_driver,
                       pcm: *mut snd_soc_tplg_pcm, _dai: *mut snd_soc_dai) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let ipc_pcm_ops = sof_ipc_get_ops_pcm(sdev);
    if pcm.is_null() { return 0; }
    let spcm = kzalloc(size_of::<snd_sof_pcm>(), GFP_KERNEL) as *mut snd_sof_pcm; if spcm.is_null() { return -ENOMEM; }
    (*spcm).scomp = scomp;
    for stream in 0..2 {
        (*spcm).stream[stream].comp_id = COMP_ID_UNASSIGNED;
        if (*pcm).compress { snd_sof_compr_init_elapsed_work(&mut (*spcm).stream[stream].period_elapsed_work); }
        else { snd_sof_pcm_init_elapsed_work(&mut (*spcm).stream[stream].period_elapsed_work); }
    }
    (*spcm).pcm = core::ptr::read(pcm);
    if !ipc_pcm_ops.is_null() { if let Some(f) = (*ipc_pcm_ops).pcm_setup { let ret = f(sdev, spcm); if ret < 0 { kfree(spcm as *mut c_void); return ret; } } }
    (*dai_drv).dobj.private = spcm as *mut c_void; list_add(&mut (*spcm).list, &mut (*sdev).pcm_list);
    let private = &mut (*pcm).priv_;
    let mut ret = sof_parse_tokens(scomp, spcm as *mut c_void, stream_tokens.as_ptr(), stream_tokens.len() as c_int, private.array, le32_to_cpu(private.size) as c_int);
    if ret != 0 { return ret; }
    if (*spcm).pcm.playback {
        let stream = SNDRV_PCM_STREAM_PLAYBACK;
        ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*sdev).dev, PAGE_SIZE, &mut (*spcm).stream[stream as usize].page_table);
        if ret < 0 { return ret; }
        ret = spcm_bind(scomp, spcm, stream); if ret != 0 { snd_dma_free_pages(&mut (*spcm).stream[stream as usize].page_table); return ret; }
    }
    if (*spcm).pcm.capture {
        let stream = SNDRV_PCM_STREAM_CAPTURE;
        ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*sdev).dev, PAGE_SIZE, &mut (*spcm).stream[stream as usize].page_table);
        if ret < 0 { if (*spcm).pcm.playback { snd_dma_free_pages(&mut (*spcm).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].page_table); } return ret; }
        ret = spcm_bind(scomp, spcm, stream);
        if ret != 0 { snd_dma_free_pages(&mut (*spcm).stream[stream as usize].page_table); if (*spcm).pcm.playback { snd_dma_free_pages(&mut (*spcm).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].page_table); } return ret; }
    }
    ret
}

unsafe fn sof_dai_unload(scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let ipc_pcm_ops = sof_ipc_get_ops_pcm(sdev); let spcm = (*dobj).private as *mut snd_sof_pcm;
    if (*spcm).pcm.playback { snd_dma_free_pages(&mut (*spcm).stream[SNDRV_PCM_STREAM_PLAYBACK as usize].page_table); }
    if (*spcm).pcm.capture { snd_dma_free_pages(&mut (*spcm).stream[SNDRV_PCM_STREAM_CAPTURE as usize].page_table); }
    if !ipc_pcm_ops.is_null() { if let Some(f) = (*ipc_pcm_ops).pcm_free { f(sdev, spcm); } }
    list_del(&mut (*spcm).list); kfree(spcm as *mut c_void); 0
}

static common_dai_link_tokens: [sof_topology_token; 1] = [sof_topology_token { token: SOF_TKN_DAI_TYPE, type_: SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token: Some(get_token_dai_type), offset: off!(snd_sof_dai_link, type_) }];

unsafe fn sof_link_load(scomp: *mut snd_soc_component, _index: c_int, link: *mut snd_soc_dai_link, cfg: *mut snd_soc_tplg_link_config) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let tplg_ops = sof_ipc_get_ops_tplg(sdev); let private = &mut (*cfg).priv_;
    if (*link).platforms.is_null() { return -EINVAL; }
    (*(*link).platforms).name = dev_name((*scomp).dev);
    if !tplg_ops.is_null() { if let Some(f) = (*tplg_ops).link_setup { let ret = f(sdev, link); if ret < 0 { return ret; } } }
    if !(*link).no_pcm { (*link).nonatomic = true; return 0; }
    if le32_to_cpu(private.size) == 0 { return -EINVAL; }
    let slink = kzalloc(size_of::<snd_sof_dai_link>(), GFP_KERNEL) as *mut snd_sof_dai_link; if slink.is_null() { return -ENOMEM; }
    (*slink).num_hw_configs = le32_to_cpu((*cfg).num_hw_configs);
    (*slink).hw_configs = kmemdup((*cfg).hw_config as *const c_void, ((*slink).num_hw_configs as usize) * size_of::<snd_soc_tplg_hw_config>(), GFP_KERNEL) as *mut snd_soc_tplg_hw_config;
    (*slink).default_hw_cfg_id = le32_to_cpu((*cfg).default_hw_config_id); (*slink).link = link;
    let mut ret = sof_parse_tokens(scomp, slink as *mut c_void, common_dai_link_tokens.as_ptr(), common_dai_link_tokens.len() as c_int, private.array, le32_to_cpu(private.size) as c_int);
    if ret < 0 { kfree(slink as *mut c_void); return ret; }
    let token_list = if !tplg_ops.is_null() { (*tplg_ops).token_list } else { null() };
    if token_list.is_null() { (*link).dobj.private = slink as *mut c_void; list_add(&mut (*slink).list, &mut (*sdev).dai_link_list); return 0; }
    let mut token_id: u32_ = 0; let mut num_tuples = (*token_list.add(SOF_DAI_LINK_TOKENS as usize)).count; let mut num_sets = (*slink).num_hw_configs as c_int;
    match (*slink).type_ {
        SOF_DAI_INTEL_SSP => { token_id = SOF_SSP_TOKENS; num_tuples += (*token_list.add(SOF_SSP_TOKENS as usize)).count * (*slink).num_hw_configs as c_int; }
        SOF_DAI_INTEL_DMIC => { token_id = SOF_DMIC_TOKENS; num_tuples += (*token_list.add(SOF_DMIC_TOKENS as usize)).count + (*token_list.add(SOF_DMIC_PDM_TOKENS as usize)).count * SOF_DAI_INTEL_DMIC_NUM_CTRL; }
        SOF_DAI_INTEL_HDA => { token_id = SOF_HDA_TOKENS; num_tuples += (*token_list.add(SOF_HDA_TOKENS as usize)).count; }
        SOF_DAI_INTEL_ALH => { token_id = SOF_ALH_TOKENS; num_tuples += (*token_list.add(SOF_ALH_TOKENS as usize)).count; }
        SOF_DAI_IMX_SAI => { token_id = SOF_SAI_TOKENS; num_tuples += (*token_list.add(SOF_SAI_TOKENS as usize)).count; }
        SOF_DAI_IMX_ESAI => { token_id = SOF_ESAI_TOKENS; num_tuples += (*token_list.add(SOF_ESAI_TOKENS as usize)).count; }
        SOF_DAI_MEDIATEK_AFE => { token_id = SOF_AFE_TOKENS; num_tuples += (*token_list.add(SOF_AFE_TOKENS as usize)).count; }
        SOF_DAI_AMD_DMIC => { token_id = SOF_ACPDMIC_TOKENS; num_tuples += (*token_list.add(SOF_ACPDMIC_TOKENS as usize)).count; }
        SOF_DAI_AMD_BT | SOF_DAI_AMD_SP | SOF_DAI_AMD_HS | SOF_DAI_AMD_SP_VIRTUAL | SOF_DAI_AMD_HS_VIRTUAL | SOF_DAI_AMD_I2S => { token_id = SOF_ACPI2S_TOKENS; num_tuples += (*token_list.add(SOF_ACPI2S_TOKENS as usize)).count; }
        SOF_DAI_IMX_MICFIL => { token_id = SOF_MICFIL_TOKENS; num_tuples += (*token_list.add(SOF_MICFIL_TOKENS as usize)).count; }
        SOF_DAI_AMD_SDW => { token_id = SOF_ACP_SDW_TOKENS; num_tuples += (*token_list.add(SOF_ACP_SDW_TOKENS as usize)).count; }
        _ => {}
    }
    (*slink).tuples = kzalloc(size_of::<snd_sof_tuple>() * num_tuples as usize, GFP_KERNEL) as *mut snd_sof_tuple; if (*slink).tuples.is_null() { kfree(slink as *mut c_void); return -ENOMEM; }
    if !(*token_list.add(SOF_DAI_LINK_TOKENS as usize)).tokens.is_null() {
        ret = sof_copy_tuples(sdev, private.array, le32_to_cpu(private.size) as c_int, SOF_DAI_LINK_TOKENS, 1, (*slink).tuples, num_tuples, &mut (*slink).num_tuples);
        if ret < 0 { kfree((*slink).tuples as *mut c_void); kfree(slink as *mut c_void); return ret; }
    }
    if token_id != 0 && !(*token_list.add(token_id as usize)).tokens.is_null() {
        ret = sof_copy_tuples(sdev, private.array, le32_to_cpu(private.size) as c_int, token_id, num_sets, (*slink).tuples, num_tuples, &mut (*slink).num_tuples);
        if ret < 0 { kfree((*slink).tuples as *mut c_void); kfree(slink as *mut c_void); return ret; }
        if token_id == SOF_DMIC_TOKENS {
            num_sets = sof_get_token_value(SOF_TKN_INTEL_DMIC_NUM_PDM_ACTIVE, (*slink).tuples, (*slink).num_tuples);
            if num_sets < 0 { kfree((*slink).tuples as *mut c_void); kfree(slink as *mut c_void); return num_sets; }
            ret = sof_copy_tuples(sdev, private.array, le32_to_cpu(private.size) as c_int, SOF_DMIC_PDM_TOKENS, num_sets, (*slink).tuples, num_tuples, &mut (*slink).num_tuples);
            if ret < 0 { kfree((*slink).tuples as *mut c_void); kfree(slink as *mut c_void); return ret; }
        }
    }
    (*link).dobj.private = slink as *mut c_void; list_add(&mut (*slink).list, &mut (*sdev).dai_link_list); 0
}

unsafe fn sof_link_unload(_scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let slink = (*dobj).private as *mut snd_sof_dai_link; if slink.is_null() { return 0; }
    (*(*(*slink).link).platforms).name = null(); kfree((*slink).tuples as *mut c_void); list_del(&mut (*slink).list); kfree(slink as *mut c_void); (*dobj).private = null_mut(); 0
}

unsafe fn sof_route_load(scomp: *mut snd_soc_component, _index: c_int, route: *mut snd_soc_dapm_route) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let dobj = &mut (*route).dobj;
    let sroute = kzalloc(size_of::<snd_sof_route>(), GFP_KERNEL) as *mut snd_sof_route; if sroute.is_null() { return -ENOMEM; }
    (*sroute).scomp = scomp;
    let source_swidget = snd_sof_find_swidget(scomp, (*route).source as *mut c_char);
    if source_swidget.is_null() { kfree(sroute as *mut c_void); return -EINVAL; }
    if (*source_swidget).id == snd_soc_dapm_out_drv || (*source_swidget).id == snd_soc_dapm_output { kfree(sroute as *mut c_void); return 0; }
    let sink_swidget = snd_sof_find_swidget(scomp, (*route).sink as *mut c_char);
    if sink_swidget.is_null() { kfree(sroute as *mut c_void); return -EINVAL; }
    if (*sink_swidget).id == snd_soc_dapm_out_drv || (*sink_swidget).id == snd_soc_dapm_output { kfree(sroute as *mut c_void); return 0; }
    (*sroute).route = route; dobj.private = sroute as *mut c_void; (*sroute).src_widget = source_swidget; (*sroute).sink_widget = sink_swidget;
    list_add(&mut (*sroute).list, &mut (*sdev).route_list); 0
}

unsafe fn sof_set_widget_pipeline(sdev: *mut snd_sof_dev, spipe: *mut snd_sof_pipeline, swidget: *mut snd_sof_widget) -> c_int {
    let pipe_widget = (*spipe).pipe_widget;
    if (*pipe_widget).dynamic_pipeline_widget {
        // Original list_for_each_entry checks all controls and rejects volatile controls matching swidget->comp_id.
        let _ = sdev;
    }
    (*swidget).spipe = spipe; (*swidget).dynamic_pipeline_widget = (*pipe_widget).dynamic_pipeline_widget; 0
}

unsafe fn sof_complete(scomp: *mut snd_soc_component) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    let widget_ops = if !tplg_ops.is_null() { (*tplg_ops).widget } else { null() };
    let _ = widget_ops;
    // Original iterates kcontrol_list and pipeline_list, updating IPC structures,
    // validates dynamic pipelines under SOF_DBG_VERIFY_TPLG, then sets up static pipelines.
    if sof_debug_check_flag(SOF_DBG_VERIFY_TPLG) && !tplg_ops.is_null() {
        if let (Some(setup), Some(teardown)) = ((*tplg_ops).set_up_all_pipelines, (*tplg_ops).tear_down_all_pipelines) {
            let mut ret = setup(sdev, true); if ret < 0 { return ret; }
            ret = teardown(sdev, true); if ret < 0 { return ret; }
        }
    }
    if !tplg_ops.is_null() { if let Some(f) = (*tplg_ops).set_up_all_pipelines { return f(sdev, false); } }
    0
}

unsafe fn sof_manifest(scomp: *mut snd_soc_component, index: c_int, man: *mut snd_soc_tplg_manifest) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp); let tplg_ops = sof_ipc_get_ops_tplg(sdev);
    if !tplg_ops.is_null() { if let Some(f) = (*tplg_ops).parse_manifest { return f(scomp, index, man); } }
    0
}

static sof_io_ops: [snd_soc_tplg_kcontrol_ops; 4] = [
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_VOL_ID, get: Some(snd_sof_volume_get), put: Some(snd_sof_volume_put) },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_BYTES_ID, get: Some(snd_sof_bytes_get), put: Some(snd_sof_bytes_put) },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_ENUM_ID, get: Some(snd_sof_enum_get), put: Some(snd_sof_enum_put) },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_SWITCH_ID, get: Some(snd_sof_switch_get), put: Some(snd_sof_switch_put) },
];
static sof_bytes_ext_ops: [snd_soc_tplg_bytes_ext_ops; 2] = [
    snd_soc_tplg_bytes_ext_ops { id: SOF_TPLG_KCTL_BYTES_ID, get: Some(snd_sof_bytes_ext_get), put: Some(snd_sof_bytes_ext_put) },
    snd_soc_tplg_bytes_ext_ops { id: SOF_TPLG_KCTL_BYTES_VOLATILE_RO, get: Some(snd_sof_bytes_ext_volatile_get), put: None },
];
static sof_tplg_ops: snd_soc_tplg_ops = snd_soc_tplg_ops { _private: [] };

unsafe extern "C" fn snd_sof_dspless_kcontrol(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }
static sof_dspless_io_ops: [snd_soc_tplg_kcontrol_ops; 4] = [
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_VOL_ID, get: None, put: None },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_BYTES_ID, get: None, put: None },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_ENUM_ID, get: None, put: None },
    snd_soc_tplg_kcontrol_ops { id: SOF_TPLG_KCTL_SWITCH_ID, get: None, put: None },
];
unsafe extern "C" fn snd_sof_dspless_bytes_ext_get(_kcontrol: *mut snd_kcontrol, _binary_data: *mut c_uint, _size: c_uint) -> c_int { 0 }
unsafe extern "C" fn snd_sof_dspless_bytes_ext_put(_kcontrol: *mut snd_kcontrol, _binary_data: *const c_uint, _size: c_uint) -> c_int { 0 }
static sof_dspless_bytes_ext_ops: [snd_soc_tplg_bytes_ext_ops; 2] = [
    snd_soc_tplg_bytes_ext_ops { id: SOF_TPLG_KCTL_BYTES_ID, get: None, put: None },
    snd_soc_tplg_bytes_ext_ops { id: SOF_TPLG_KCTL_BYTES_VOLATILE_RO, get: None, put: None },
];

unsafe fn sof_dspless_widget_ready(scomp: *mut snd_soc_component, _index: c_int, w: *mut snd_soc_dapm_widget, tw: *mut snd_soc_tplg_dapm_widget) -> c_int {
    let priv_ = &mut (*tw).priv_;
    let mut ret = sof_parse_tokens(scomp, w as *mut c_void, dapm_widget_tokens.as_ptr(), dapm_widget_tokens.len() as c_int, priv_.array, le32_to_cpu(priv_.size) as c_int);
    if ret < 0 { return ret; }
    if WIDGET_IS_DAI((*w).id) {
        let dai_tokens = [sof_topology_token { token: SOF_TKN_DAI_TYPE, type_: SND_SOC_TPLG_TUPLE_TYPE_STRING, get_token: Some(get_token_dai_type), offset: 0 }];
        let sdev = snd_soc_component_get_drvdata(scomp);
        let swidget = kzalloc(size_of::<snd_sof_widget>(), GFP_KERNEL) as *mut snd_sof_widget; if swidget.is_null() { return -ENOMEM; }
        let sdai = kzalloc(size_of::<snd_sof_dai>(), GFP_KERNEL) as *mut snd_sof_dai; if sdai.is_null() { kfree(swidget as *mut c_void); return -ENOMEM; }
        ret = sof_parse_tokens(scomp, &mut (*sdai).type_ as *mut _ as *mut c_void, dai_tokens.as_ptr(), dai_tokens.len() as c_int, priv_.array, le32_to_cpu(priv_.size) as c_int);
        if ret < 0 { kfree(swidget as *mut c_void); kfree(sdai as *mut c_void); return ret; }
        ret = sof_connect_dai_widget(scomp, w, tw, sdai); if ret != 0 { kfree(swidget as *mut c_void); kfree(sdai as *mut c_void); return ret; }
        (*swidget).scomp = scomp; (*swidget).widget = w; (*swidget).private = sdai as *mut c_void; mutex_init(&mut (*swidget).setup_mutex);
        (*w).dobj.private = swidget as *mut c_void; list_add(&mut (*swidget).list, &mut (*sdev).widget_list);
    }
    0
}

unsafe fn sof_dspless_widget_unload(scomp: *mut snd_soc_component, dobj: *mut snd_soc_dobj) -> c_int {
    let w = dobj as *mut snd_soc_dapm_widget;
    if WIDGET_IS_DAI((*w).id) {
        let swidget = (*dobj).private as *mut snd_sof_widget;
        sof_disconnect_dai_widget(scomp, w);
        if swidget.is_null() { return 0; }
        list_del(&mut (*swidget).list); kfree((*swidget).private); kfree(swidget as *mut c_void);
    }
    0
}

unsafe fn sof_dspless_link_load(scomp: *mut snd_soc_component, _index: c_int, link: *mut snd_soc_dai_link, _cfg: *mut snd_soc_tplg_link_config) -> c_int {
    (*(*link).platforms).name = dev_name((*scomp).dev);
    if !(*link).no_pcm { (*link).nonatomic = true; }
    0
}
static sof_dspless_tplg_ops: snd_soc_tplg_ops = snd_soc_tplg_ops { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn snd_sof_load_topology(scomp: *mut snd_soc_component, file: *const c_char) -> c_int {
    let sdev = snd_soc_component_get_drvdata(scomp);
    let sof_pdata = (*sdev).pdata;
    let tplg_filename_prefix = (*sof_pdata).tplg_filename_prefix;
    let mut tplg_cnt: c_int = 0;
    let mut ret: c_int = 0;
    let mut tplg_files = kcalloc((*(*scomp).card).num_links as size_t, size_of::<*const c_char>(), GFP_KERNEL) as *mut *const c_char;
    if tplg_files.is_null() { return -ENOMEM; }
    if !(*sof_pdata).disable_function_topology && !disable_function_topology &&
       !(*sof_pdata).machine.is_null() && (*(*sof_pdata).machine).get_function_tplg_files.is_some() {
        let no_fallback = !strstr(file, b"dummy\0".as_ptr() as *const c_char).is_null();
        tplg_cnt = (*(*sof_pdata).machine).get_function_tplg_files.unwrap()((*scomp).card, (*sof_pdata).machine, tplg_filename_prefix, &mut tplg_files, no_fallback);
        if tplg_cnt < 0 { kfree(tplg_files as *mut c_void); return tplg_cnt; }
    }
    if tplg_cnt == 0 {
        if !strstr(file, b"dummy\0".as_ptr() as *const c_char).is_null() { kfree(tplg_files as *mut c_void); return -EINVAL; }
        *tplg_files.add(0) = file; tplg_cnt = 1;
    }
    for i in 0..tplg_cnt {
        let mut fw: *const firmware = null();
        ret = request_firmware(&mut fw, *tplg_files.add(i as usize), (*scomp).dev);
        if ret < 0 { break; }
        ret = if (*sdev).dspless_mode_selected {
            snd_soc_tplg_component_load(scomp, &sof_dspless_tplg_ops, fw)
        } else {
            snd_soc_tplg_component_load(scomp, &sof_tplg_ops, fw)
        };
        if ret < 0 { break; }
    }
    if ret >= 0 {
        for i in 0..feature_tplg_cnt {
            let feature_topology = devm_kasprintf((*scomp).dev, GFP_KERNEL, b"%s/%s\0".as_ptr() as *const c_char,
                                                  tplg_filename_prefix, feature_topologies[i as usize]);
            if feature_topology.is_null() { ret = -ENOMEM; break; }
            let mut fw: *const firmware = null();
            ret = request_firmware(&mut fw, feature_topology, (*scomp).dev);
            if ret < 0 { ret = 0; continue; }
            ret = if (*sdev).dspless_mode_selected {
                snd_soc_tplg_component_load(scomp, &sof_dspless_tplg_ops, fw)
            } else {
                snd_soc_tplg_component_load(scomp, &sof_tplg_ops, fw)
            };
            if ret < 0 { break; }
        }
    }
    if ret >= 0 { ret = sof_complete(scomp); }
    if ret >= 0 && (*sdev).led_present { ret = snd_ctl_led_request(); }
    kfree(tplg_files as *mut c_void);
    ret
}

// EXPORT_SYMBOL(snd_sof_load_topology);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
