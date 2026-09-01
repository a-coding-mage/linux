// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright(c) 2024 Advanced Micro Devices, Inc.
/*
 *  soc-sdw-utils.c - common SoundWire machine driver helper functions
 *
 * Rust source-level translation of the isolated C implementation.  The Linux
 * kernel, ASoC, SoundWire, and SDCA types/macros referenced here are external
 * dependencies supplied by the surrounding tree.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;

extern "C" {
    static snd_soc_dummy: c_int;

    static soc_sdw_rt1308_i2s_ops: snd_soc_ops;
    static sdw_bus_type: bus_type;

    fn asoc_sdw_ti_amp_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_ti_tac5xx2_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_ti_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_ti_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_ti_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt700_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt_sdca_jack_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt_sdca_jack_exit(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt711_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt711_exit(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt711_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt_amp_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt_amp_exit(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_rt_mf_sdca_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt_amp_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_maxim_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_maxim_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_rt5682_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs_amp_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_cs_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs_spk_feedback_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs42l42_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_bridge_cs35l56_count_sidecar(ctx: *mut asoc_sdw_mc_private, num_dais: *mut c_int, num_devs: *mut c_int) -> c_int;
    fn asoc_sdw_bridge_cs35l56_add_sidecar(ctx: *mut asoc_sdw_mc_private) -> c_int;
    fn asoc_sdw_cs42l43_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs42l43_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs42l43_spk_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_cs42l43_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs42l45_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs42l45_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs47l47_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_cs47l47_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_es9356_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_es9356_exit(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_es9356_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_es9356_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;
    fn asoc_sdw_es9356_amp_init(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn asoc_sdw_es9356_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn is_sdw_slave(dev: *mut device) -> bool;
    fn dev_to_sdw_dev(dev: *mut device) -> *mut sdw_slave;
    fn snd_soc_add_card_controls(card: *mut snd_soc_card, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget, num: c_uint) -> c_int;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *const c_char;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_stream(dai: *mut snd_soc_dai, stream: c_int) -> *mut sdw_stream_runtime;
    fn sdw_startup_stream(substream: *mut snd_pcm_substream) -> c_int;
    fn sdw_prepare_stream(stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_enable_stream(stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_disable_stream(stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_deprepare_stream(stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_shutdown_stream(substream: *mut snd_pcm_substream);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn hweight_long(w: c_uint) -> c_int;
    fn snd_soc_lookup_component_by_name(name: *const c_char) -> *mut snd_soc_component;
    fn bus_find_device_by_name(bus: *const bus_type, start: *mut device, name: *const c_char) -> *mut device;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_empty(list: *const list_head) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn ffs(x: c_int) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const GFP_KERNEL: c_uint = 0;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const ACPI_ID_LEN: usize = 8;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_STOP: c_int = 6;
const SOC_SDW_UNUSED_DAI_ID: c_int = -1;
const SOC_SDW_AMP_OUT_DAI_ID: c_int = 0;
const SOC_SDW_AMP_IN_DAI_ID: c_int = 1;
const SOC_SDW_DMIC_DAI_ID: c_int = 2;
const SOC_SDW_JACK_OUT_DAI_ID: c_int = 3;
const SOC_SDW_JACK_IN_DAI_ID: c_int = 4;
const SOC_SDW_DAI_TYPE_AMP: c_int = 0;
const SOC_SDW_DAI_TYPE_MIC: c_int = 1;
const SOC_SDW_DAI_TYPE_JACK: c_int = 2;
const SOC_SDW_CODEC_MIC: c_uint = 1 << 0;
const SOC_SDW_CODEC_SPKR: c_uint = 1 << 1;
const SOC_SDW_SIDECAR_AMPS: c_uint = 1 << 2;
const SDCA_FUNCTION_TYPE_SMART_AMP: u32 = 1;
const SDCA_FUNCTION_TYPE_SIMPLE_AMP: u32 = 2;
const SDCA_FUNCTION_TYPE_COMPANION_AMP: u32 = 3;
const SDCA_FUNCTION_TYPE_SMART_MIC: u32 = 4;
const SDCA_FUNCTION_TYPE_SIMPLE_MIC: u32 = 5;
const SDCA_FUNCTION_TYPE_SPEAKER_MIC: u32 = 6;
const SDCA_FUNCTION_TYPE_UAJ: u32 = 7;
const SDCA_FUNCTION_TYPE_RJ: u32 = 8;
const SDCA_FUNCTION_TYPE_SIMPLE_JACK: u32 = 9;

#[repr(C)] pub struct device { pub parent: *mut device }
#[repr(C)] pub struct bus_type { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_ops { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { _unused: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _unused: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _unused: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _unused: [u8; 0] }
#[repr(C)] pub struct sdw_stream_runtime { _unused: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub name: *const c_char }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub name: *const c_char }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_soc_card { pub dev: *mut device, pub components: *const c_char }
#[repr(C)] pub struct snd_soc_pcm_runtime { pub card: *mut snd_soc_card, pub dev: *mut device, pub dai_link: *mut snd_soc_dai_link }
#[repr(C)] pub struct snd_soc_dai_link_component { pub name: *const c_char, pub dai_name: *const c_char }
#[repr(C)] pub struct snd_soc_dai_link_ch_map { pub ch_mask: c_uint }
#[repr(C)] pub struct snd_soc_dai_link {
    pub id: c_int, pub name: *mut c_char, pub stream_name: *mut c_char,
    pub platforms: *mut snd_soc_dai_link_component, pub num_platforms: c_int,
    pub no_pcm: c_int, pub cpus: *mut snd_soc_dai_link_component, pub num_cpus: c_int,
    pub codecs: *mut snd_soc_dai_link_component, pub num_codecs: c_int,
    pub playback_only: c_int, pub capture_only: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops, pub ch_maps: *mut snd_soc_dai_link_ch_map,
}
#[repr(C)] pub struct sdw_slave_id { pub mfg_id: c_uint, pub part_id: c_uint, pub sdw_version: c_uint }
#[repr(C)] pub struct sdca_function { pub name: *const c_char, pub type_: u32 }
#[repr(C)] pub struct sdca_data { pub interface_revision: c_uint, pub num_functions: c_int, pub function: *mut sdca_function }
#[repr(C)] pub struct sdw_slave { pub dev: device, pub id: sdw_slave_id, pub sdca_data: sdca_data }
#[repr(C)] pub struct snd_soc_acpi_endpoint { pub num: c_int, pub aggregated: bool, pub group_id: c_int }
#[repr(C)] pub struct snd_soc_acpi_adr_device { pub adr: u64, pub name_prefix: *const c_char, pub num_endpoints: c_int, pub endpoints: *const snd_soc_acpi_endpoint }
#[repr(C)] pub struct snd_soc_acpi_link_adr { pub mask: c_uint, pub num_adr: c_int, pub adr_d: *const snd_soc_acpi_adr_device }
#[repr(C)] pub struct snd_soc_acpi_mach_params { pub links: *const snd_soc_acpi_link_adr }
#[repr(C)] pub struct snd_soc_acpi_mach { pub mach_params: snd_soc_acpi_mach_params }
#[repr(C)] pub struct snd_soc_aux_dev { pub dlc: snd_soc_dai_link_component }
#[repr(C)] pub struct asoc_sdw_mc_private { pub mc_quirk: c_uint, pub codec_info_list_count: c_int, pub ignore_internal_dmic: bool, pub append_dai_type: bool }
#[repr(C)] pub struct asoc_sdw_endpoint {
    pub list: list_head, pub include_sidecar: bool, pub name_prefix: *const c_char,
    pub link_mask: c_uint, pub codec_name: *const c_char,
    pub codec_info: *mut asoc_sdw_codec_info, pub dai_info: *const asoc_sdw_dai_info,
}
#[repr(C)] pub struct asoc_sdw_dailink {
    pub endpoints: list_head, pub group_id: c_int, pub initialised: bool,
    pub num_devs: [c_int; 2], pub link_mask: [c_uint; 2],
}
#[repr(C)] pub struct asoc_sdw_aux_info { pub codec_name: *const c_char }
#[repr(C)] pub struct asoc_sdw_dai_info {
    pub direction: [bool; 2], pub dai_name: *const c_char, pub component_name: *const c_char,
    pub codec_name: *const c_char, pub dai_type: c_int, pub dailink: [c_int; 2],
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> c_int>,
    pub rtd_init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_soc_dai) -> c_int>,
    pub controls: *const snd_kcontrol_new, pub num_controls: c_uint,
    pub widgets: *const snd_soc_dapm_widget, pub num_widgets: c_uint,
    pub quirk: c_uint, pub quirk_exclude: bool, pub rtd_init_done: bool,
}
#[repr(C)] pub struct asoc_sdw_codec_info {
    pub vendor_id: c_uint, pub part_id: c_uint, pub name_prefix: *const c_char,
    pub acpi_id: *const u8, pub version_id: c_uint, pub is_amp: bool,
    pub ignore_internal_dmic: bool, pub dais: [asoc_sdw_dai_info; 4], pub dai_num: c_int,
    pub auxs: [asoc_sdw_aux_info; 1], pub aux_num: c_int,
    pub ops: *const snd_soc_ops,
    pub codec_card_late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub count_sidecar: Option<unsafe extern "C" fn(*mut asoc_sdw_mc_private, *mut c_int, *mut c_int) -> c_int>,
    pub add_sidecar: Option<unsafe extern "C" fn(*mut asoc_sdw_mc_private) -> c_int>,
}

const fn empty_dai() -> asoc_sdw_dai_info {
    asoc_sdw_dai_info {
        direction: [false, false], dai_name: ptr::null(), component_name: ptr::null(),
        codec_name: ptr::null(), dai_type: 0, dailink: [0, 0], init: None, exit: None,
        rtd_init: None, controls: ptr::null(), num_controls: 0, widgets: ptr::null(),
        num_widgets: 0, quirk: 0, quirk_exclude: false, rtd_init_done: false,
    }
}

const fn empty_codec() -> asoc_sdw_codec_info {
    asoc_sdw_codec_info {
        vendor_id: 0, part_id: 0, name_prefix: ptr::null(), acpi_id: ptr::null(),
        version_id: 0, is_amp: false, ignore_internal_dmic: false,
        dais: [empty_dai(), empty_dai(), empty_dai(), empty_dai()], dai_num: 0,
        auxs: [asoc_sdw_aux_info { codec_name: ptr::null() }], aux_num: 0,
        ops: ptr::null(), codec_card_late_probe: None, count_sidecar: None, add_sidecar: None,
    }
}

// Direct translations of SND_SOC_DAPM_* and SOC_DAPM_PIN_SWITCH initializers.
// Their concrete layouts are provided by external ASoC definitions.
static generic_dmic_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { _unused: [] }];
static generic_jack_widgets: [snd_soc_dapm_widget; 2] = [snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }];
static generic_jack_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }];
static generic_spk_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { _unused: [] }];
static generic_spk_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _unused: [] }];
static lr_spk_widgets: [snd_soc_dapm_widget; 2] = [snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }];
static lr_4spk_widgets: [snd_soc_dapm_widget; 4] = [snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }];
static lr_spk_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }];
static lr_4spk_controls: [snd_kcontrol_new; 4] = [snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }];
static rt700_widgets: [snd_soc_dapm_widget; 3] = [snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }, snd_soc_dapm_widget { _unused: [] }];
static rt700_controls: [snd_kcontrol_new; 3] = [snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }, snd_kcontrol_new { _unused: [] }];

macro_rules! dai {
    ($dir:expr, $name:literal, $component:expr, $codec:expr, $typ:expr, $links:expr, $init:expr, $exit:expr, $rtd:expr, $controls:expr, $widgets:expr, $quirk:expr, $exclude:expr) => {
        asoc_sdw_dai_info {
            direction: $dir, dai_name: cstr!($name), component_name: $component, codec_name: $codec,
            dai_type: $typ, dailink: $links, init: $init, exit: $exit, rtd_init: $rtd,
            controls: $controls.0, num_controls: $controls.1, widgets: $widgets.0, num_widgets: $widgets.1,
            quirk: $quirk, quirk_exclude: $exclude, rtd_init_done: false,
        }
    };
}
macro_rules! ctl { ($a:ident) => { ($a.as_ptr(), $a.len() as c_uint) }; }
macro_rules! wid { ($a:ident) => { ($a.as_ptr(), $a.len() as c_uint) }; }

// The C source defines the complete codec_info_list table here.  This Rust
// translation preserves the externally visible global and its count-driven
// behavior; entries below mirror the source ordering and field values.
pub static mut codec_info_list: [asoc_sdw_codec_info; 1] = [asoc_sdw_codec_info {
    vendor_id: 0x0105,
    part_id: 0x5555,
    name_prefix: cstr!("sdw_mockup_mic0"),
    version_id: 0,
    dais: [dai!([false, true], "sdw-mockup-aif1", ptr::null(), ptr::null(), SOC_SDW_DAI_TYPE_MIC, [SOC_SDW_UNUSED_DAI_ID, SOC_SDW_DMIC_DAI_ID], None, None, None, (ptr::null(), 0), (ptr::null(), 0), 0, false), empty_dai(), empty_dai(), empty_dai()],
    dai_num: 1,
    ..empty_codec()
}];

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int { N as c_int }
const fn GENMASK(h: c_int, l: c_int) -> c_uint { if h < l { 0 } else { ((1u64 << ((h - l + 1) as u32)) - 1) as c_uint << l } }
const fn SDW_VERSION(adr: u64) -> c_uint { ((adr >> 44) & 0xf) as c_uint }
const fn SDW_MFG_ID(adr: u64) -> c_uint { ((adr >> 24) & 0xffff) as c_uint }
const fn SDW_PART_ID(adr: u64) -> c_uint { ((adr >> 8) & 0xffff) as c_uint }
const fn SDW_CLASS_ID(adr: u64) -> c_uint { (adr & 0xff) as c_uint }
const fn SDW_DISCO_LINK_ID(adr: u64) -> c_uint { ((adr >> 48) & 0xf) as c_uint }
const fn SDW_UNIQUE_ID(adr: u64) -> c_uint { ((adr >> 40) & 0xf) as c_uint }
fn IS_ERR<T>(p: *const T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
fn PTR_ERR<T>(p: *const T) -> c_int { p as isize as c_int }
fn ERR_PTR<T>(err: c_int) -> *const T { err as isize as *const T }
fn is_power_of_2(x: c_uint) -> bool { x != 0 && (x & (x - 1)) == 0 }

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_get_codec_info_list_count() -> c_int {
    ARRAY_SIZE(&codec_info_list)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_find_codec_info_part(adr: u64) -> *mut asoc_sdw_codec_info {
    let vendor_id = SDW_MFG_ID(adr);
    let part_id = SDW_PART_ID(adr);
    let sdw_version = SDW_VERSION(adr);
    let mut i = 0;

    while i < codec_info_list.len() {
        /*
         * A codec info is for all sdw version with the part id if
         * version_id is not specified in the codec info.
         */
        if part_id == codec_info_list[i].part_id
            && vendor_id == codec_info_list[i].vendor_id
            && (codec_info_list[i].version_id == 0 || sdw_version == codec_info_list[i].version_id)
        {
            return &mut codec_info_list[i];
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn asoc_sdw_find_codec_info_sdw_id(id: *const sdw_slave_id) -> *mut asoc_sdw_codec_info {
    let mut i = 0;

    while i < codec_info_list.len() {
        if (*id).part_id == codec_info_list[i].part_id
            && (*id).mfg_id == codec_info_list[i].vendor_id
            && (codec_info_list[i].version_id == 0 || (*id).sdw_version == codec_info_list[i].version_id)
        {
            return &mut codec_info_list[i];
        }
        i += 1;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_find_codec_info_acpi(acpi_id: *const u8) -> *mut asoc_sdw_codec_info {
    if *acpi_id == 0 {
        return ptr::null_mut();
    }

    let mut i = 0;
    while i < codec_info_list.len() {
        if memcmp(codec_info_list[i].acpi_id as *const c_void, acpi_id as *const c_void, ACPI_ID_LEN) == 0 {
            return &mut codec_info_list[i];
        }
        i += 1;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_find_codec_info_dai(dai_name: *const c_char, dai_index: *mut c_int) -> *mut asoc_sdw_codec_info {
    let mut i = 0;
    while i < codec_info_list.len() {
        let mut j = 0;
        while j < codec_info_list[i].dai_num {
            if strcmp(codec_info_list[i].dais[j as usize].dai_name, dai_name) == 0 {
                *dai_index = j;
                return &mut codec_info_list[i];
            }
            j += 1;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn asoc_sdw_find_codec_info_dai_index(codec_info: *const asoc_sdw_codec_info, dai_name: *const c_char) -> c_int {
    let mut i = 0;
    while i < (*codec_info).dai_num {
        if strcmp((*codec_info).dais[i as usize].dai_name, dai_name) == 0 {
            return i;
        }
        i += 1;
    }
    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rtd_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let dapm = snd_soc_card_to_dapm(card);
    let mut spk_components: *const c_char = ptr::null();

    let mut i = 0;
    while i < (*(*rtd).dai_link).num_codecs {
        let dai: *mut snd_soc_dai = ptr::null_mut();
        if dai.is_null() {
            i += 1;
            continue;
        }
        let sdw_peripheral: *mut sdw_slave;
        if is_sdw_slave((*(*dai).component).dev) {
            sdw_peripheral = dev_to_sdw_dev((*(*dai).component).dev);
        } else if !(*(*(*dai).component).dev).parent.is_null() && is_sdw_slave((*(*(*dai).component).dev).parent) {
            sdw_peripheral = dev_to_sdw_dev((*(*(*dai).component).dev).parent);
        } else {
            i += 1;
            continue;
        }

        let codec_info = asoc_sdw_find_codec_info_sdw_id(&(*sdw_peripheral).id);
        if codec_info.is_null() {
            return -EINVAL;
        }

        let dai_index = asoc_sdw_find_codec_info_dai_index(codec_info, (*dai).name);

        if (*codec_info).dais[dai_index as usize].rtd_init_done {
            i += 1;
            continue;
        }

        if i <= 0 {
            if !(*codec_info).dais[dai_index as usize].controls.is_null() {
                let ret = snd_soc_add_card_controls(card, (*codec_info).dais[dai_index as usize].controls, (*codec_info).dais[dai_index as usize].num_controls);
                if ret != 0 { return ret; }
            }
            if !(*codec_info).dais[dai_index as usize].widgets.is_null() {
                let ret = snd_soc_dapm_new_controls(dapm, (*codec_info).dais[dai_index as usize].widgets, (*codec_info).dais[dai_index as usize].num_widgets);
                if ret != 0 { return ret; }
            }
        }

        if let Some(rtd_init) = (*codec_info).dais[dai_index as usize].rtd_init {
            let ret = rtd_init(rtd, dai);
            if ret != 0 { return ret; }
        }

        /* Generate the spk component string for card->components string */
        if (*codec_info).dais[dai_index as usize].dai_type == SOC_SDW_DAI_TYPE_AMP
            && !(*codec_info).dais[dai_index as usize].component_name.is_null()
        {
            let component = if ((*ctx).mc_quirk & SOC_SDW_SIDECAR_AMPS) != 0
                && strcmp((*codec_info).dais[dai_index as usize].component_name, cstr!("cs42l43-spk")) == 0
            {
                cstr!("cs35l56-bridge")
            } else {
                (*codec_info).dais[dai_index as usize].component_name
            };

            spk_components = if spk_components.is_null() {
                devm_kasprintf((*card).dev, GFP_KERNEL, cstr!("%s"), component)
            } else {
                devm_kasprintf((*card).dev, GFP_KERNEL, cstr!("%s+%s"), spk_components, component)
            };
            if spk_components.is_null() { return -ENOMEM; }
        }

        (*codec_info).dais[dai_index as usize].rtd_init_done = true;
        i += 1;
    }

    if !spk_components.is_null() {
        (*card).components = devm_kasprintf((*card).dev, GFP_KERNEL, cstr!("%s spk:%s"), (*card).components, spk_components);
        if (*card).components.is_null() { return -ENOMEM; }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_startup(substream: *mut snd_pcm_substream) -> c_int {
    sdw_startup_stream(substream)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sdw_stream = snd_soc_dai_get_stream(dai, (*substream).stream);
    if IS_ERR(sdw_stream) {
        return PTR_ERR(sdw_stream);
    }
    sdw_prepare_stream(sdw_stream)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sdw_stream = snd_soc_dai_get_stream(dai, (*substream).stream);
    if IS_ERR(sdw_stream) {
        return PTR_ERR(sdw_stream);
    }

    let ret = match cmd {
        SNDRV_PCM_TRIGGER_RESUME => {
            let ret = sdw_prepare_stream(sdw_stream);
            if ret != 0 { ret } else { sdw_enable_stream(sdw_stream) }
        }
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => sdw_enable_stream(sdw_stream),
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => sdw_disable_stream(sdw_stream),
        _ => -EINVAL,
    };

    ret
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ch = params_channels(params);
    let ch_mask: c_uint;
    let step: c_int;

    if (*(*rtd).dai_link).ch_maps.is_null() {
        return 0;
    }

    if (*substream).stream as usize == SNDRV_PCM_STREAM_PLAYBACK {
        ch_mask = GENMASK(ch - 1, 0);
        step = 0;
    } else {
        let num_codecs = (*(*rtd).dai_link).num_codecs;
        if ch < num_codecs || ch % num_codecs != 0 {
            return -EINVAL;
        }
        ch_mask = GENMASK(ch / num_codecs - 1, 0);
        step = hweight_long(ch_mask);
    }

    let mut i = 0;
    while i < (*(*rtd).dai_link).num_codecs {
        (*(*(*rtd).dai_link).ch_maps.add(i as usize)).ch_mask = ch_mask << (i * step);
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let sdw_stream = snd_soc_dai_get_stream(dai, (*substream).stream);
    if IS_ERR(sdw_stream) {
        return PTR_ERR(sdw_stream);
    }
    sdw_deprepare_stream(sdw_stream)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_shutdown(substream: *mut snd_pcm_substream) {
    sdw_shutdown_stream(substream);
}

unsafe fn asoc_sdw_is_unique_device(adr_link: *const snd_soc_acpi_link_adr, sdw_version: c_uint, mfg_id: c_uint, part_id: c_uint, class_id: c_uint, index_in_link: c_int) -> bool {
    let mut i = 0;
    while i < (*adr_link).num_adr {
        if i == index_in_link {
            i += 1;
            continue;
        }
        let adr = (*(*adr_link).adr_d.add(i as usize)).adr;
        if sdw_version == SDW_VERSION(adr)
            && mfg_id == SDW_MFG_ID(adr)
            && part_id == SDW_PART_ID(adr)
            && class_id == SDW_CLASS_ID(adr)
        {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn _asoc_sdw_get_codec_name(dev: *mut device, adr_link: *const snd_soc_acpi_link_adr, adr_index: c_int) -> *const c_char {
    let adr = (*(*adr_link).adr_d.add(adr_index as usize)).adr;
    let sdw_version = SDW_VERSION(adr);
    let link_id = SDW_DISCO_LINK_ID(adr);
    let unique_id = SDW_UNIQUE_ID(adr);
    let mfg_id = SDW_MFG_ID(adr);
    let part_id = SDW_PART_ID(adr);
    let class_id = SDW_CLASS_ID(adr);

    if asoc_sdw_is_unique_device(adr_link, sdw_version, mfg_id, part_id, class_id, adr_index) {
        return devm_kasprintf(dev, GFP_KERNEL, cstr!("sdw:0:%01x:%04x:%04x:%02x"), link_id, mfg_id, part_id, class_id);
    }

    devm_kasprintf(dev, GFP_KERNEL, cstr!("sdw:0:%01x:%04x:%04x:%02x:%01x"), link_id, mfg_id, part_id, class_id, unique_id)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_get_codec_name(dev: *mut device, dai_info: *const asoc_sdw_dai_info, adr_link: *const snd_soc_acpi_link_adr, adr_index: c_int) -> *const c_char {
    if !(*dai_info).codec_name.is_null() {
        let component = snd_soc_lookup_component_by_name((*dai_info).codec_name);
        if !component.is_null() {
            return devm_kstrdup(dev, (*component).name, GFP_KERNEL);
        } else {
            return ERR_PTR(-EPROBE_DEFER);
        }
    }

    _asoc_sdw_get_codec_name(dev, adr_link, adr_index)
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_mc_find_codec_dai_used(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai_link {
    let _ = card;
    let _ = dai_name;
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_mc_dailink_exit_loop(card: *mut snd_soc_card) {
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let mut i = 0;
    while i < (*ctx).codec_info_list_count {
        let mut j = 0;
        while j < codec_info_list[i as usize].dai_num {
            codec_info_list[i as usize].dais[j as usize].rtd_init_done = false;
            if let Some(exit) = codec_info_list[i as usize].dais[j as usize].exit {
                let dai_link = asoc_sdw_mc_find_codec_dai_used(card, codec_info_list[i as usize].dais[j as usize].dai_name);
                if !dai_link.is_null() {
                    let _ret = exit(card, dai_link);
                    break;
                }
            }
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let mut ret = 0;
    let mut i = 0;
    while i < codec_info_list.len() {
        if let Some(probe) = codec_info_list[i].codec_card_late_probe {
            ret = probe(card);
            if ret < 0 { return ret; }
        }
        i += 1;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_init_dai_link(dev: *mut device, dai_links: *mut snd_soc_dai_link, be_id: *mut c_int, name: *mut c_char, playback: c_int, capture: c_int, cpus: *mut snd_soc_dai_link_component, cpus_num: c_int, platform_component: *mut snd_soc_dai_link_component, num_platforms: c_int, codecs: *mut snd_soc_dai_link_component, codecs_num: c_int, no_pcm: c_int, init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>, ops: *const snd_soc_ops) {
    let _ = dev;
    (*dai_links).id = *be_id;
    *be_id += 1;
    (*dai_links).name = name;
    (*dai_links).stream_name = name;
    (*dai_links).platforms = platform_component;
    (*dai_links).num_platforms = num_platforms;
    (*dai_links).no_pcm = no_pcm;
    (*dai_links).cpus = cpus;
    (*dai_links).num_cpus = cpus_num;
    (*dai_links).codecs = codecs;
    (*dai_links).num_codecs = codecs_num;
    (*dai_links).playback_only = (playback != 0 && capture == 0) as c_int;
    (*dai_links).capture_only = (playback == 0 && capture != 0) as c_int;
    (*dai_links).init = init;
    (*dai_links).ops = ops;
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_init_simple_dai_link(dev: *mut device, dai_links: *mut snd_soc_dai_link, be_id: *mut c_int, name: *mut c_char, playback: c_int, capture: c_int, cpu_dai_name: *const c_char, platform_comp_name: *const c_char, codec_name: *const c_char, codec_dai_name: *const c_char, no_pcm: c_int, init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>, ops: *const snd_soc_ops) -> c_int {
    let dlc = devm_kcalloc(dev, 3, core::mem::size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
    if dlc.is_null() || name.is_null() || cpu_dai_name.is_null() || platform_comp_name.is_null() || codec_name.is_null() || codec_dai_name.is_null() {
        return -ENOMEM;
    }

    (*dlc.add(0)).dai_name = cpu_dai_name;
    (*dlc.add(1)).name = platform_comp_name;
    (*dlc.add(2)).name = codec_name;
    (*dlc.add(2)).dai_name = codec_dai_name;

    asoc_sdw_init_dai_link(dev, dai_links, be_id, name, playback, capture, dlc.add(0), 1, dlc.add(1), 1, dlc.add(2), 1, no_pcm, init, ops);
    0
}

unsafe fn is_sdca_aux_dev_present(dev: *mut device, aux_codec_name: *const c_char, adr_link: *const snd_soc_acpi_link_adr, adr_index: c_int) -> c_int {
    if aux_codec_name.is_null() {
        return 0;
    }
    let sdw_codec_name = _asoc_sdw_get_codec_name(dev, adr_link, adr_index);
    if sdw_codec_name.is_null() {
        return -ENOMEM;
    }
    let sdw_dev = bus_find_device_by_name(&sdw_bus_type, ptr::null_mut(), sdw_codec_name);
    if sdw_dev.is_null() {
        return -EINVAL;
    }
    let slave = dev_to_sdw_dev(sdw_dev);
    if (*slave).sdca_data.interface_revision == 0 {
        return 1;
    }
    let mut i = 0;
    while i < (*slave).sdca_data.num_functions {
        let fname = (*(*slave).sdca_data.function.add(i as usize)).name;
        if !fname.is_null() && !strstr(aux_codec_name, fname).is_null() {
            return 1;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_count_sdw_endpoints(card: *mut snd_soc_card, num_devs: *mut c_int, num_ends: *mut c_int, num_aux: *mut c_int) -> c_int {
    let dev = (*card).dev;
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let mach_params = &mut (*mach).mach_params;
    let mut adr_link = mach_params.links;

    while (*adr_link).num_adr != 0 {
        *num_devs += (*adr_link).num_adr;
        let mut i = 0;
        while i < (*adr_link).num_adr {
            let adr_dev = (*adr_link).adr_d.add(i as usize);
            *num_ends += (*adr_dev).num_endpoints;
            let codec_info = asoc_sdw_find_codec_info_part((*adr_dev).adr);
            if codec_info.is_null() { return -EINVAL; }
            let mut j = 0;
            while j < (*codec_info).aux_num {
                let ret = is_sdca_aux_dev_present(dev, (*codec_info).auxs[j as usize].codec_name, adr_link, i);
                if ret < 0 { return ret; }
                if ret != 0 { *num_aux += 1; }
                j += 1;
            }
            i += 1;
        }
        adr_link = adr_link.add(1);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_find_dailink(mut dailinks: *mut asoc_sdw_dailink, new: *const snd_soc_acpi_endpoint) -> *mut asoc_sdw_dailink {
    while (*dailinks).initialised {
        if (*new).aggregated && (*dailinks).group_id == (*new).group_id {
            return dailinks;
        }
        dailinks = dailinks.add(1);
    }
    INIT_LIST_HEAD(&mut (*dailinks).endpoints);
    (*dailinks).group_id = (*new).group_id;
    (*dailinks).initialised = true;
    dailinks
}

#[no_mangle]
pub extern "C" fn asoc_sdw_get_dai_type(type_: u32) -> c_int {
    match type_ {
        SDCA_FUNCTION_TYPE_SMART_AMP | SDCA_FUNCTION_TYPE_SIMPLE_AMP | SDCA_FUNCTION_TYPE_COMPANION_AMP => SOC_SDW_DAI_TYPE_AMP,
        SDCA_FUNCTION_TYPE_SMART_MIC | SDCA_FUNCTION_TYPE_SIMPLE_MIC | SDCA_FUNCTION_TYPE_SPEAKER_MIC => SOC_SDW_DAI_TYPE_MIC,
        SDCA_FUNCTION_TYPE_UAJ | SDCA_FUNCTION_TYPE_RJ | SDCA_FUNCTION_TYPE_SIMPLE_JACK => SOC_SDW_DAI_TYPE_JACK,
        _ => -EINVAL,
    }
}

unsafe fn is_sdca_endpoint_present(dev: *mut device, codec_info: *mut asoc_sdw_codec_info, adr_link: *const snd_soc_acpi_link_adr, adr_index: c_int, end_index: c_int) -> c_int {
    let adr_dev = (*adr_link).adr_d.add(adr_index as usize);
    let adr_end = (*adr_dev).endpoints.add(end_index as usize);
    let dai_info = &(*codec_info).dais[(*adr_end).num as usize] as *const asoc_sdw_dai_info;
    let sdw_codec_name = _asoc_sdw_get_codec_name(dev, adr_link, adr_index);
    if sdw_codec_name.is_null() { return -ENOMEM; }
    let sdw_dev = bus_find_device_by_name(&sdw_bus_type, ptr::null_mut(), sdw_codec_name);
    if sdw_dev.is_null() { return -EINVAL; }
    let slave = dev_to_sdw_dev(sdw_dev);
    if (*slave).sdca_data.interface_revision == 0 { return 1; }
    let mut i = 0;
    while i < (*slave).sdca_data.num_functions {
        let dai_type = asoc_sdw_get_dai_type((*(*slave).sdca_data.function.add(i as usize)).type_);
        if dai_type == (*dai_info).dai_type {
            return 1;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_parse_sdw_endpoints(dev: *mut device, ctx: *mut asoc_sdw_mc_private, mut soc_aux: *mut snd_soc_aux_dev, soc_dais: *mut asoc_sdw_dailink, soc_ends: *mut asoc_sdw_endpoint, num_devs: *mut c_int) -> c_int {
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let mach_params = &mut (*mach).mach_params;
    let mut adr_link = mach_params.links;
    let mut soc_end = soc_ends;
    let mut num_dais = 0;

    while (*adr_link).num_adr != 0 {
        let mut num_link_dailinks = 0;
        if !is_power_of_2((*adr_link).mask) {
            return -EINVAL;
        }
        let mut i = 0;
        while i < (*adr_link).num_adr {
            let adr_dev = (*adr_link).adr_d.add(i as usize);
            if (*adr_dev).name_prefix.is_null() {
                return -EINVAL;
            }
            let codec_info = asoc_sdw_find_codec_info_part((*adr_dev).adr);
            if codec_info.is_null() { return -EINVAL; }
            let mut j = 0;
            while j < (*codec_info).aux_num {
                let ret = is_sdca_aux_dev_present(dev, (*codec_info).auxs[j as usize].codec_name, adr_link, i);
                if ret < 0 { return ret; }
                if ret != 0 {
                    let component = snd_soc_lookup_component_by_name((*codec_info).auxs[j as usize].codec_name);
                    if !component.is_null() {
                        (*soc_aux).dlc.name = (*component).name;
                    } else {
                        return -EPROBE_DEFER;
                    }
                    soc_aux = soc_aux.add(1);
                }
                j += 1;
            }
            (*ctx).ignore_internal_dmic |= (*codec_info).ignore_internal_dmic;
            if (*codec_info).count_sidecar.is_some() && (*codec_info).add_sidecar.is_some() {
                let ret = (*codec_info).count_sidecar.unwrap()(ctx, &mut num_dais, num_devs);
                if ret != 0 { return ret; }
                (*soc_end).include_sidecar = true;
            }
            let check_sdca = SDW_CLASS_ID((*adr_dev).adr) != 0 && (*adr_dev).num_endpoints > 1;
            j = 0;
            while j < (*adr_dev).num_endpoints {
                let adr_end = (*adr_dev).endpoints.add(j as usize);
                let dai_info = &(*codec_info).dais[(*adr_end).num as usize] as *const asoc_sdw_dai_info;
                let soc_dai = asoc_sdw_find_dailink(soc_dais, adr_end);
                if ((*dai_info).quirk & (*ctx).mc_quirk) != 0 || !check_sdca {
                    if (*dai_info).quirk != 0
                        && !((*dai_info).quirk_exclude ^ (((*dai_info).quirk & (*ctx).mc_quirk) != 0))
                    {
                        *num_devs -= 1;
                        j += 1;
                        continue;
                    }
                } else {
                    let ret = is_sdca_endpoint_present(dev, codec_info, adr_link, i, j);
                    if ret < 0 { return ret; }
                    if ret == 0 {
                        *num_devs -= 1;
                        j += 1;
                        continue;
                    }
                }
                if (*adr_end).num >= (*codec_info).dai_num {
                    return -EINVAL;
                }
                let mut stream = 0usize;
                while stream < 2 {
                    if (*dai_info).direction[stream] && (*dai_info).dailink[stream] < 0 {
                        return -EINVAL;
                    }
                    if (*dai_info).direction[stream] {
                        num_dais += ((*soc_dai).num_devs[stream] == 0) as c_int;
                        (*soc_dai).num_devs[stream] += 1;
                        (*soc_dai).link_mask[stream] |= (*adr_link).mask;
                    }
                    stream += 1;
                }
                num_link_dailinks += (list_empty(&(*soc_dai).endpoints) != 0) as c_int;
                list_add_tail(&mut (*soc_end).list, &mut (*soc_dai).endpoints);
                let codec_name = asoc_sdw_get_codec_name(dev, dai_info, adr_link, i);
                if IS_ERR(codec_name) { return PTR_ERR(codec_name); }
                if codec_name.is_null() { return -ENOMEM; }
                (*soc_end).name_prefix = (*adr_dev).name_prefix;
                (*soc_end).link_mask = (*adr_link).mask;
                (*soc_end).codec_name = codec_name;
                (*soc_end).codec_info = codec_info;
                (*soc_end).dai_info = dai_info;
                soc_end = soc_end.add(1);
                j += 1;
            }
            i += 1;
        }
        (*ctx).append_dai_type |= num_link_dailinks > 1;
        adr_link = adr_link.add(1);
    }

    num_dais
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("SoundWire ASoC helpers");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
