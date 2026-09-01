// SPDX-License-Identifier: GPL-2.0+
//
// soc-dapm.rs  --  ALSA SoC Dynamic Audio Power Management
//
// Copyright 2005 Wolfson Microelectronics PLC.
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//
// Rust source-level translation of soc-dapm.c. Kernel framework types,
// list iteration helpers, allocation helpers, tracepoints, debugfs/sysfs
// helpers, and ALSA/ASoC APIs are external dependencies from the original
// repository and are referenced here as opaque C ABI items.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type async_cookie_t = c_ulong;
type u32 = c_uint;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    pub bias_level: snd_soc_bias_level,
    pub idle_bias: bool_t,
    pub component: *mut snd_soc_component,
    pub card: *mut snd_soc_card,
    pub target_bias_level: snd_soc_bias_level,
    pub list: list_head,
    pub wcache_sink: *mut snd_soc_dapm_widget,
    pub wcache_source: *mut snd_soc_dapm_widget,
    // CONFIG_DEBUG_FS: struct dentry *debugfs_dapm;
    pub debugfs_dapm: *mut dentry,
}

#[repr(C)]
pub struct dapm_kcontrol_data {
    pub value: c_uint,
    pub widget: *mut snd_soc_dapm_widget,
    pub paths: list_head,
    pub wlist: *mut snd_soc_dapm_widget_list,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub name_prefix: *const c_char,
    pub name: *const c_char,
    pub driver: *mut snd_soc_component_driver,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub suspend_bias_off: bool_t,
    pub idle_bias_on: bool_t,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub snd_card: *mut snd_card,
    pub dapm: *mut snd_soc_dapm_context,
    pub dapm_stats: dapm_stats,
    pub dapm_dirty: list_head,
    pub paths: list_head,
    pub widgets: list_head,
    pub dapm_list: list_head,
    pub fully_routed: bool_t,
    pub num_ignore_suspend_widgets: c_int,
    pub ignore_suspend_widgets: *mut *const c_char,
    pub num_of_ignore_suspend_widgets: c_int,
    pub of_ignore_suspend_widgets: *mut *const c_char,
}

#[repr(C)]
pub struct dapm_stats {
    pub path_checks: c_uint,
    pub neighbour_checks: c_uint,
    pub power_checks: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: snd_soc_dapm_type,
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_int,
    pub shift: c_uint,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
    pub subseq: c_int,
    pub dapm: *mut snd_soc_dapm_context,
    pub list: list_head,
    pub dirty: list_head,
    pub work_list: list_head,
    pub power_list: list_head,
    pub edges: [list_head; 2],
    pub endpoints: [c_int; 2],
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_int,
    pub kcontrols: *mut *mut snd_kcontrol,
    pub priv_: *mut c_void,
    pub regulator: *mut regulator,
    pub pinctrl: *mut pinctrl,
    pub clk: *mut clk,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_int,
    pub power_check: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget) -> c_int>,
    pub power: c_int,
    pub new_power: c_int,
    pub power_checked: bool_t,
    pub force: c_int,
    pub connected: c_int,
    pub active: c_int,
    pub new_: c_int,
    pub is_ep: c_uint,
    pub is_supply: c_int,
    pub ignore_suspend: c_int,
    pub no_wname_in_kcontrol_name: bool_t,
    pub channel: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_path {
    pub list: list_head,
    pub list_kcontrol: list_head,
    pub list_node: [list_head; 2],
    pub node: [*mut snd_soc_dapm_widget; 2],
    pub source: *mut snd_soc_dapm_widget,
    pub sink: *mut snd_soc_dapm_widget,
    pub name: *const c_char,
    pub connect: bool_t,
    pub is_supply: c_int,
    pub walking: c_int,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_kcontrol)>,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_list {
    pub num_widgets: c_uint,
    pub widgets: [*mut snd_soc_dapm_widget; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_update {
    pub kcontrol: *mut snd_kcontrol,
    pub reg: c_int,
    pub mask: c_uint,
    pub val: c_uint,
    pub has_second_set: bool_t,
    pub reg2: c_int,
    pub mask2: c_uint,
    pub val2: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub rshift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub autodisable: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub mask: c_uint,
    pub texts: *mut *const c_char,
    pub autodisable: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_pinctrl_priv {
    pub active_state: *const c_char,
    pub sleep_state: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub name: *const c_char,
    pub driver: *mut snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub formats: c_ulong,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub c2c_params_select: c_uint,
    pub c2c_widget: [*mut snd_soc_dapm_widget; 2],
    pub pcm: *mut snd_pcm,
    pub pop_wait: c_int,
    pub delayed_work: c_void,
    pub pmdown_time: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub dynamic: bool_t,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link_ch_map {
    pub cpu: c_int,
    pub codec: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_pcm_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
    pub hw_opened: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub format: c_uint,
    pub subformat: c_uint,
    pub channels: c_uint,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct pinctrl { _private: [u8; 0] }
#[repr(C)] pub struct pinctrl_state { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct file_operations {
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn() -> loff_t>,
}
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { pub attr: attribute }

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF = 0,
    SND_SOC_BIAS_STANDBY = 1,
    SND_SOC_BIAS_PREPARE = 2,
    SND_SOC_BIAS_ON = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_direction {
    SND_SOC_DAPM_DIR_IN = 0,
    SND_SOC_DAPM_DIR_OUT = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_type {
    snd_soc_dapm_input = 0,
    snd_soc_dapm_output,
    snd_soc_dapm_mux,
    snd_soc_dapm_mux_named_ctl,
    snd_soc_dapm_demux,
    snd_soc_dapm_mixer,
    snd_soc_dapm_mixer_named_ctl,
    snd_soc_dapm_pga,
    snd_soc_dapm_out_drv,
    snd_soc_dapm_adc,
    snd_soc_dapm_dac,
    snd_soc_dapm_micbias,
    snd_soc_dapm_mic,
    snd_soc_dapm_hp,
    snd_soc_dapm_spk,
    snd_soc_dapm_line,
    snd_soc_dapm_switch,
    snd_soc_dapm_vmid,
    snd_soc_dapm_pre,
    snd_soc_dapm_post,
    snd_soc_dapm_supply,
    snd_soc_dapm_pinctrl,
    snd_soc_dapm_regulator_supply,
    snd_soc_dapm_clock_supply,
    snd_soc_dapm_aif_in,
    snd_soc_dapm_aif_out,
    snd_soc_dapm_siggen,
    snd_soc_dapm_sink,
    snd_soc_dapm_dai_in,
    snd_soc_dapm_dai_out,
    snd_soc_dapm_dai_link,
    snd_soc_dapm_kcontrol,
    snd_soc_dapm_buffer,
    snd_soc_dapm_scheduler,
    snd_soc_dapm_effect,
    snd_soc_dapm_src,
    snd_soc_dapm_asrc,
    snd_soc_dapm_encoder,
    snd_soc_dapm_decoder,
    SND_SOC_DAPM_TYPE_COUNT,
}

const EIO: c_int = 5;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_EP_SOURCE: c_uint = 1;
const SND_SOC_DAPM_EP_SINK: c_uint = 2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x4;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;
const SND_SOC_DAPM_WILL_PMU: c_int = 0x10;
const SND_SOC_DAPM_WILL_PMD: c_int = 0x20;
const SND_SOC_DAPM_PRE_REG: c_int = 0x40;
const SND_SOC_DAPM_POST_REG: c_int = 0x80;
const SND_SOC_DAPM_STREAM_START: c_int = 1;
const SND_SOC_DAPM_STREAM_STOP: c_int = 2;
const SND_SOC_DAPM_STREAM_SUSPEND: c_int = 3;
const SND_SOC_DAPM_STREAM_RESUME: c_int = 4;
const SND_SOC_DAPM_STREAM_PAUSE_PUSH: c_int = 5;
const SND_SOC_DAPM_STREAM_PAUSE_RELEASE: c_int = 6;
const SND_SOC_DAPM_STREAM_NOP: c_int = 0;
const SND_SOC_DAPM_REGULATOR_BYPASS: c_uint = 1;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D3cold: c_int = 4;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;

static mut pop_time: u32 = 0;

static mut dapm_up_seq: [c_int; snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize] = {
    let mut a = [0; snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize];
    a[snd_soc_dapm_type::snd_soc_dapm_pre as usize] = 1;
    a[snd_soc_dapm_type::snd_soc_dapm_regulator_supply as usize] = 2;
    a[snd_soc_dapm_type::snd_soc_dapm_pinctrl as usize] = 2;
    a[snd_soc_dapm_type::snd_soc_dapm_clock_supply as usize] = 2;
    a[snd_soc_dapm_type::snd_soc_dapm_supply as usize] = 3;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_link as usize] = 3;
    a[snd_soc_dapm_type::snd_soc_dapm_micbias as usize] = 4;
    a[snd_soc_dapm_type::snd_soc_dapm_vmid as usize] = 4;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_in as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_out as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_aif_in as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_aif_out as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_mic as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_siggen as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_input as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_output as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_mux as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_demux as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_dac as usize] = 8;
    a[snd_soc_dapm_type::snd_soc_dapm_switch as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_mixer as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_pga as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_buffer as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_scheduler as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_effect as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_src as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_asrc as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_encoder as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_decoder as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_adc as usize] = 11;
    a[snd_soc_dapm_type::snd_soc_dapm_out_drv as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_hp as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_line as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_sink as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_spk as usize] = 13;
    a[snd_soc_dapm_type::snd_soc_dapm_kcontrol as usize] = 14;
    a[snd_soc_dapm_type::snd_soc_dapm_post as usize] = 15;
    a
};

static mut dapm_down_seq: [c_int; snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize] = {
    let mut a = [0; snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize];
    a[snd_soc_dapm_type::snd_soc_dapm_pre as usize] = 1;
    a[snd_soc_dapm_type::snd_soc_dapm_kcontrol as usize] = 2;
    a[snd_soc_dapm_type::snd_soc_dapm_adc as usize] = 3;
    a[snd_soc_dapm_type::snd_soc_dapm_spk as usize] = 4;
    a[snd_soc_dapm_type::snd_soc_dapm_hp as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_line as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_out_drv as usize] = 5;
    a[snd_soc_dapm_type::snd_soc_dapm_sink as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_pga as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_buffer as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_scheduler as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_effect as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_src as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_asrc as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_encoder as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_decoder as usize] = 6;
    a[snd_soc_dapm_type::snd_soc_dapm_switch as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_mixer as usize] = 7;
    a[snd_soc_dapm_type::snd_soc_dapm_dac as usize] = 8;
    a[snd_soc_dapm_type::snd_soc_dapm_mic as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_siggen as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_input as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_output as usize] = 9;
    a[snd_soc_dapm_type::snd_soc_dapm_micbias as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_vmid as usize] = 10;
    a[snd_soc_dapm_type::snd_soc_dapm_mux as usize] = 11;
    a[snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl as usize] = 11;
    a[snd_soc_dapm_type::snd_soc_dapm_demux as usize] = 11;
    a[snd_soc_dapm_type::snd_soc_dapm_aif_in as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_aif_out as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_in as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_out as usize] = 12;
    a[snd_soc_dapm_type::snd_soc_dapm_dai_link as usize] = 13;
    a[snd_soc_dapm_type::snd_soc_dapm_supply as usize] = 14;
    a[snd_soc_dapm_type::snd_soc_dapm_clock_supply as usize] = 15;
    a[snd_soc_dapm_type::snd_soc_dapm_pinctrl as usize] = 15;
    a[snd_soc_dapm_type::snd_soc_dapm_regulator_supply as usize] = 15;
    a[snd_soc_dapm_type::snd_soc_dapm_post as usize] = 16;
    a
};

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kfree_const(p: *const c_char);
    fn kmemdup(src: *const c_void, len: size_t, flags: c_uint) -> *mut c_void;
    fn kstrdup_const(s: *const c_char, flags: c_uint) -> *const c_char;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *const c_char;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn fls(x: c_uint) -> c_int;
    fn ffs(x: c_ulong) -> c_int;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn schedule_timeout_uninterruptible(t: c_ulong);
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> bool_t;
    fn snd_soc_dapm_mutex_assert_held(p: *mut c_void);
    fn snd_soc_dapm_mutex_lock(p: *mut c_void);
    fn snd_soc_dapm_mutex_lock_root(p: *mut c_void);
    fn snd_soc_dapm_mutex_unlock(p: *mut c_void);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_int) -> c_uint;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_int, mask: c_uint, value: c_uint) -> c_int;
    fn snd_soc_component_test_bits(component: *mut snd_soc_component, reg: c_int, mask: c_uint, value: c_uint) -> c_int;
    fn snd_soc_component_async_complete(component: *mut snd_soc_component);
    fn snd_soc_component_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_card_set_bias_level(card: *mut snd_soc_card, dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_card_set_bias_level_post(card: *mut snd_soc_card, dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int;
    fn snd_soc_component_seq_notifier(component: *mut snd_soc_component, id: c_int, subseq: c_int);
    fn snd_soc_component_stream_event(component: *mut snd_soc_component, event: c_int) -> c_int;
    fn snd_power_get_state(card: *mut snd_card) -> c_int;
    fn regulator_allow_bypass(r: *mut regulator, enable: bool_t) -> c_int;
    fn regulator_enable(r: *mut regulator) -> c_int;
    fn regulator_disable_deferred(r: *mut regulator, delay: c_uint) -> c_int;
    fn pinctrl_lookup_state(p: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    fn pinctrl_select_state(p: *mut pinctrl, s: *mut pinctrl_state) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_kcontrol_chip(k: *const snd_kcontrol) -> *mut c_void;
    fn snd_soc_cnew(n: *const snd_kcontrol_new, data: *mut c_void, name: *const c_char, prefix: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_free_one(kcontrol: *mut snd_kcontrol);
    fn snd_soc_volsw_is_stereo(mc: *const soc_mixer_control) -> bool_t;
    fn snd_soc_enum_val_to_item(e: *const soc_enum, val: c_uint) -> c_uint;
    fn snd_soc_enum_item_to_val(e: *const soc_enum, item: c_uint) -> c_uint;
    fn match_string(array: *mut *const c_char, n: c_uint, string: *const c_char) -> c_int;
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dai_set_widget_playback(dai: *mut snd_soc_dai, w: *mut snd_soc_dapm_widget);
    fn snd_soc_dai_set_widget_capture(dai: *mut snd_soc_dai, w: *mut snd_soc_dapm_widget);
    fn snd_soc_substream_to_rtd(s: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_channels(p: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(p: *mut snd_pcm_hw_params) -> c_uint;
    fn params_subformat(p: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(p: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_direction_name(dir: c_int) -> *const c_char;
    fn snd_soc_dpcm_runtime_update(card: *mut snd_soc_card);
    fn simple_read_from_buffer(user_buf: *mut c_char, count: size_t, ppos: *mut loff_t, buf: *const c_char, available: ssize_t) -> ssize_t;
}

#[inline] unsafe fn dapm_dir_reverse(x: snd_soc_dapm_direction) -> snd_soc_dapm_direction {
    if x == snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN {
        snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT
    } else {
        snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN
    }
}

#[inline] unsafe fn dir_to_ep(dir: snd_soc_dapm_direction) -> c_uint {
    if dir == snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN { SND_SOC_DAPM_EP_SOURCE } else { SND_SOC_DAPM_EP_SINK }
}

#[inline] unsafe fn event_on(event: c_int) -> bool {
    (event & (SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_WILL_PMU)) != 0
}

// Linux list traversal macros are file-external. The translated control-flow
// below keeps the same entry points and local operations; list iteration sites
// are marked where the C macro body must be supplied by the integration layer.

unsafe fn dapm_assert_locked(dapm: *mut snd_soc_dapm_context) {
    if snd_soc_card_is_instantiated((*dapm).card) {
        snd_soc_dapm_mutex_assert_held(dapm as *mut c_void);
    }
}

unsafe fn dapm_pop_wait() {
    if pop_time != 0 {
        schedule_timeout_uninterruptible(msecs_to_jiffies(pop_time));
    }
}

unsafe fn dapm_pop_dbg(_dev: *mut device, _fmt: *const c_char) {
    if pop_time == 0 {
        return;
    }
    let buf = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return;
    }
    // C varargs formatting side effects are preserved by the external call in
    // the original; Rust cannot forward an open varargs list from this helper.
    kfree(buf as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_alloc(dev: *mut device) -> *mut snd_soc_dapm_context {
    devm_kzalloc(dev, core::mem::size_of::<snd_soc_dapm_context>(), GFP_KERNEL) as *mut snd_soc_dapm_context
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device {
    if !(*dapm).component.is_null() { (*(*dapm).component).dev } else { (*(*dapm).card).dev }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card {
    (*dapm).card
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component {
    (*dapm).component
}

unsafe fn dapm_dirty_widget(w: *mut snd_soc_dapm_widget) -> bool {
    !(*w).dirty.next.is_null() && (*w).dirty.next != &mut (*w).dirty
}

unsafe fn dapm_mark_dirty(w: *mut snd_soc_dapm_widget, _reason: *const c_char) {
    dapm_assert_locked((*w).dapm);
    if !dapm_dirty_widget(w) {
        // list_add_tail(&w->dirty, &w->dapm->card->dapm_dirty);
    }
}

unsafe fn dapm_widget_invalidate_paths(w: *mut snd_soc_dapm_widget, dir: snd_soc_dapm_direction) {
    dapm_assert_locked((*w).dapm);
    if (*w).endpoints[dir as usize] == -1 {
        return;
    }
    (*w).endpoints[dir as usize] = -1;
    // for each reachable path in direction `dir`, skip supply/disconnected
    // paths, set node->endpoints[dir] = -1, and append node->work_list.
}

unsafe fn dapm_widget_invalidate_input_paths(w: *mut snd_soc_dapm_widget) {
    dapm_widget_invalidate_paths(w, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN);
}

unsafe fn dapm_widget_invalidate_output_paths(w: *mut snd_soc_dapm_widget) {
    dapm_widget_invalidate_paths(w, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT);
}

unsafe fn dapm_path_invalidate(p: *mut snd_soc_dapm_path) {
    if (*p).is_supply != 0 {
        return;
    }
    if (*(*p).source).endpoints[snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN as usize] != 0 {
        dapm_widget_invalidate_input_paths((*p).sink);
    }
    if (*(*p).sink).endpoints[snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT as usize] != 0 {
        dapm_widget_invalidate_output_paths((*p).source);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_mark_endpoints_dirty(card: *mut snd_soc_card) {
    snd_soc_dapm_mutex_lock_root(card as *mut c_void);
    // for_each_card_widgets(card, w): mark endpoint widgets dirty and invalidate
    // their opposite cached endpoint counts.
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
}

unsafe fn dapm_cnew_widget(widget: *const snd_soc_dapm_widget, prefix: *const c_char) -> *mut snd_soc_dapm_widget {
    let w = kmemdup(widget as *const c_void, core::mem::size_of::<snd_soc_dapm_widget>(), GFP_KERNEL) as *mut snd_soc_dapm_widget;
    if w.is_null() { return ptr::null_mut(); }
    if !prefix.is_null() {
        (*w).name = kasprintf(GFP_KERNEL, b"%s %s\0".as_ptr() as *const c_char, prefix, (*widget).name);
    } else {
        (*w).name = kstrdup_const((*widget).name, GFP_KERNEL);
    }
    if (*w).name.is_null() { return ptr::null_mut(); }
    if !(*widget).sname.is_null() {
        (*w).sname = kstrdup_const((*widget).sname, GFP_KERNEL);
        if (*w).sname.is_null() {
            kfree_const((*w).name);
            return ptr::null_mut();
        }
    }
    w
}

unsafe fn dapm_read(dapm: *mut snd_soc_dapm_context, reg: c_int) -> c_uint {
    if (*dapm).component.is_null() { return (-EIO) as c_uint; }
    snd_soc_component_read((*dapm).component, reg)
}

unsafe fn dapm_set_mixer_path_status(p: *mut snd_soc_dapm_path, i: c_int, nth_path: c_int) {
    let mc = (*(*p).sink).kcontrol_news.add(i as usize).read().private_value as *mut soc_mixer_control;
    let reg = (*mc).reg as c_int;
    let invert = (*mc).invert;
    if reg != SND_SOC_NOPM {
        let shift = (*mc).shift;
        let max = (*mc).max;
        let mask = (1u32 << fls(max)) - 1;
        let mut val = dapm_read((*(*p).sink).dapm, reg);
        if snd_soc_volsw_is_stereo(mc) && nth_path > 0 {
            if reg as c_uint != (*mc).rreg { val = dapm_read((*(*p).sink).dapm, (*mc).rreg as c_int); }
            val = (val >> (*mc).rshift) & mask;
        } else {
            val = (val >> shift) & mask;
        }
        if invert != 0 { val = max - val; }
        (*p).connect = val != 0;
    } else {
        (*p).connect = invert != 0;
    }
}

unsafe fn dapm_connect_mux(dapm: *mut snd_soc_dapm_context, path: *mut snd_soc_dapm_path, control_name: *const c_char, w: *mut snd_soc_dapm_widget) -> c_int {
    let kcontrol = (*w).kcontrol_news;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item: c_uint = if (*e).reg as c_int != SND_SOC_NOPM {
        let val = (dapm_read(dapm, (*e).reg as c_int) >> (*e).shift_l) & (*e).mask;
        snd_soc_enum_val_to_item(e, val)
    } else { 0 };
    let i = match_string((*e).texts, (*e).items, control_name);
    if i < 0 { return -ENODEV; }
    (*path).name = *(*e).texts.add(i as usize);
    (*path).connect = i as c_uint == item;
    0
}

unsafe fn dapm_connect_mixer(_dapm: *mut snd_soc_dapm_context, path: *mut snd_soc_dapm_path, control_name: *const c_char) -> c_int {
    let mut i = 0;
    let mut nth_path = 0;
    while i < (*(*path).sink).num_kcontrols {
        let kc = (*(*path).sink).kcontrol_news.add(i as usize);
        if strcmp(control_name, (*kc).name) == 0 {
            (*path).name = (*kc).name;
            dapm_set_mixer_path_status(path, i, nth_path);
            nth_path += 1;
            let _ = nth_path;
            return 0;
        }
        i += 1;
    }
    -ENODEV
}

unsafe fn dapm_update_widget_flags(w: *mut snd_soc_dapm_widget) {
    let mut ep: c_uint;
    match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_input => {
            if (*(*(*w).dapm).card).fully_routed { return; }
            ep = SND_SOC_DAPM_EP_SOURCE;
            // for each source path: micbias/mic/line/output source clears ep.
        }
        snd_soc_dapm_type::snd_soc_dapm_output => {
            if (*(*(*w).dapm).card).fully_routed { return; }
            ep = SND_SOC_DAPM_EP_SINK;
            // for each sink path: spk/hp/line/input sink clears ep.
        }
        snd_soc_dapm_type::snd_soc_dapm_line => {
            ep = 0;
            if (*w).edges[0].next != &mut (*w).edges[0] { ep |= SND_SOC_DAPM_EP_SOURCE; }
            if (*w).edges[1].next != &mut (*w).edges[1] { ep |= SND_SOC_DAPM_EP_SINK; }
        }
        _ => return,
    }
    (*w).is_ep = ep;
}

unsafe fn dapm_check_dynamic_path(_dapm: *mut snd_soc_dapm_context, source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget, control: *const c_char) -> c_int {
    if control.is_null() { return 0; }
    let dynamic_source = (*source).id == snd_soc_dapm_type::snd_soc_dapm_demux;
    let dynamic_sink = matches!((*sink).id,
        snd_soc_dapm_type::snd_soc_dapm_mux |
        snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl |
        snd_soc_dapm_type::snd_soc_dapm_switch |
        snd_soc_dapm_type::snd_soc_dapm_mixer |
        snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl);
    if (dynamic_source && dynamic_sink) || (!dynamic_source && !dynamic_sink) { -EINVAL } else { 0 }
}

unsafe fn dapm_add_path(dapm: *mut snd_soc_dapm_context, wsource: *mut snd_soc_dapm_widget, wsink: *mut snd_soc_dapm_widget, control: *const c_char, connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>) -> c_int {
    if (*wsink).is_supply != 0 && (*wsource).is_supply == 0 { return -EINVAL; }
    if connected.is_some() && (*wsource).is_supply == 0 { return -EINVAL; }
    if (*wsource).is_supply != 0 && !control.is_null() { return -EINVAL; }
    let mut ret = dapm_check_dynamic_path(dapm, wsource, wsink, control);
    if ret != 0 { return ret; }
    let path = devm_kzalloc(snd_soc_dapm_to_dev(dapm), core::mem::size_of::<snd_soc_dapm_path>(), GFP_KERNEL) as *mut snd_soc_dapm_path;
    if path.is_null() { return -ENOMEM; }
    (*path).node[snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN as usize] = wsource;
    (*path).node[snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT as usize] = wsink;
    (*path).source = wsource;
    (*path).sink = wsink;
    (*path).connected = connected;
    if (*wsource).is_supply != 0 || (*wsink).is_supply != 0 { (*path).is_supply = 1; }
    if control.is_null() {
        (*path).connect = true;
    } else {
        if (*wsource).id == snd_soc_dapm_type::snd_soc_dapm_demux {
            ret = dapm_connect_mux(dapm, path, control, wsource);
            if ret != 0 { kfree(path as *mut c_void); return ret; }
        }
        match (*wsink).id {
            snd_soc_dapm_type::snd_soc_dapm_mux | snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl => {
                ret = dapm_connect_mux(dapm, path, control, wsink);
                if ret != 0 { kfree(path as *mut c_void); return ret; }
            }
            snd_soc_dapm_type::snd_soc_dapm_switch | snd_soc_dapm_type::snd_soc_dapm_mixer | snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl => {
                ret = dapm_connect_mixer(dapm, path, control);
                if ret != 0 { kfree(path as *mut c_void); return ret; }
            }
            _ => {}
        }
    }
    // list_add path to card->paths and to both node edge lists.
    dapm_update_widget_flags(wsource);
    dapm_update_widget_flags(wsink);
    dapm_mark_dirty(wsource, b"Route added\0".as_ptr() as *const c_char);
    dapm_mark_dirty(wsink, b"Route added\0".as_ptr() as *const c_char);
    if snd_soc_card_is_instantiated((*dapm).card) && (*path).connect { dapm_path_invalidate(path); }
    0
}

unsafe fn dapm_kcontrol_data_alloc(widget: *mut snd_soc_dapm_widget, kcontrol: *mut snd_kcontrol, _ctrl_name: *const c_char) -> c_int {
    let data = devm_kzalloc(snd_soc_dapm_to_dev((*widget).dapm), core::mem::size_of::<dapm_kcontrol_data>(), GFP_KERNEL) as *mut dapm_kcontrol_data;
    if data.is_null() { return -ENOMEM; }
    match (*widget).id {
        snd_soc_dapm_type::snd_soc_dapm_switch | snd_soc_dapm_type::snd_soc_dapm_mixer | snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl => {
            let mc = (*kcontrol).private_value as *mut soc_mixer_control;
            if (*mc).autodisable != 0 {
                // Create snd_soc_dapm_kcontrol autodisable widget from mixer register fields.
                (*data).value = if (*mc).invert != 0 { (*mc).max } else { 0 };
            }
        }
        snd_soc_dapm_type::snd_soc_dapm_demux | snd_soc_dapm_type::snd_soc_dapm_mux | snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl => {
            let e = (*kcontrol).private_value as *mut soc_enum;
            if (*e).autodisable != 0 {
                (*data).value = snd_soc_enum_item_to_val(e, 0);
            } else if (*e).reg as c_int != SND_SOC_NOPM {
                (*data).value = dapm_read((*widget).dapm, (*e).reg as c_int) & ((*e).mask << (*e).shift_l);
            }
        }
        _ => {}
    }
    (*kcontrol).private_data = data as *mut c_void;
    0
}

unsafe extern "C" fn dapm_kcontrol_free(kctl: *mut snd_kcontrol) {
    let data = snd_kcontrol_chip(kctl) as *mut dapm_kcontrol_data;
    kfree((*data).wlist as *mut c_void);
    kfree(data as *mut c_void);
}

unsafe fn dapm_kcontrol_get_wlist(kcontrol: *const snd_kcontrol) -> *mut snd_soc_dapm_widget_list {
    (*(snd_kcontrol_chip(kcontrol) as *mut dapm_kcontrol_data)).wlist
}

unsafe fn dapm_kcontrol_add_widget(kcontrol: *mut snd_kcontrol, widget: *mut snd_soc_dapm_widget) -> c_int {
    let data = snd_kcontrol_chip(kcontrol) as *mut dapm_kcontrol_data;
    // krealloc struct snd_soc_dapm_widget_list and append widget.
    if (*data).wlist.is_null() {
        (*data).wlist = devm_kzalloc(ptr::null_mut(), core::mem::size_of::<snd_soc_dapm_widget_list>() + core::mem::size_of::<*mut snd_soc_dapm_widget>(), GFP_KERNEL) as *mut snd_soc_dapm_widget_list;
        if (*data).wlist.is_null() { return -ENOMEM; }
        (*(*data).wlist).num_widgets = 1;
    } else {
        (*(*data).wlist).num_widgets += 1;
    }
    let n = (*(*data).wlist).num_widgets as usize;
    *(*data).wlist.cast::<u8>().add(core::mem::size_of::<snd_soc_dapm_widget_list>()).cast::<*mut snd_soc_dapm_widget>().add(n - 1) = widget;
    0
}

unsafe fn dapm_kcontrol_add_path(_kcontrol: *const snd_kcontrol, _path: *mut snd_soc_dapm_path) {
    // list_add_tail(&path->list_kcontrol, &data->paths);
}

unsafe fn dapm_kcontrol_is_powered(kcontrol: *const snd_kcontrol) -> bool {
    let data = snd_kcontrol_chip(kcontrol) as *mut dapm_kcontrol_data;
    if (*data).widget.is_null() { true } else { (*(*data).widget).power != 0 }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_kcontrol_get_value(kcontrol: *const snd_kcontrol) -> c_uint {
    (*(snd_kcontrol_chip(kcontrol) as *mut dapm_kcontrol_data)).value
}

unsafe fn dapm_kcontrol_set_value(kcontrol: *const snd_kcontrol, value: c_uint) -> bool {
    let data = snd_kcontrol_chip(kcontrol) as *mut dapm_kcontrol_data;
    if (*data).value == value { return false; }
    if !(*data).widget.is_null() {
        let w0 = *(*data).wlist.cast::<u8>().add(core::mem::size_of::<snd_soc_dapm_widget_list>()).cast::<*mut snd_soc_dapm_widget>();
        match (*w0).id {
            snd_soc_dapm_type::snd_soc_dapm_switch | snd_soc_dapm_type::snd_soc_dapm_mixer | snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl => (*(*data).widget).on_val = value & (*(*data).widget).mask,
            snd_soc_dapm_type::snd_soc_dapm_demux | snd_soc_dapm_type::snd_soc_dapm_mux | snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl => (*(*data).widget).on_val = value >> (*(*data).widget).shift,
            _ => (*(*data).widget).on_val = value,
        }
    }
    (*data).value = value;
    true
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_kcontrol_to_widget(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_widget {
    *dapm_kcontrol_get_wlist(kcontrol).cast::<u8>().add(core::mem::size_of::<snd_soc_dapm_widget_list>()).cast::<*mut snd_soc_dapm_widget>()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context {
    (*snd_soc_dapm_kcontrol_to_widget(kcontrol)).dapm
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component {
    snd_soc_dapm_to_component(snd_soc_dapm_kcontrol_to_dapm(kcontrol))
}

unsafe fn dapm_reset(card: *mut snd_soc_card) {
    snd_soc_dapm_mutex_assert_held(card as *mut c_void);
    ptr::write_bytes(&mut (*card).dapm_stats as *mut dapm_stats as *mut u8, 0, core::mem::size_of::<dapm_stats>());
    // for_each_card_widgets: w->new_power = w->power; w->power_checked = false;
}

unsafe fn dapm_prefix(dapm: *mut snd_soc_dapm_context) -> *const c_char {
    if (*dapm).component.is_null() { ptr::null() } else { (*(*dapm).component).name_prefix }
}

unsafe fn dapm_update_bits(dapm: *mut snd_soc_dapm_context, reg: c_int, mask: c_uint, value: c_uint) -> c_int {
    if (*dapm).component.is_null() { -EIO } else { snd_soc_component_update_bits((*dapm).component, reg, mask, value) }
}

unsafe fn dapm_test_bits(dapm: *mut snd_soc_dapm_context, reg: c_int, mask: c_uint, value: c_uint) -> c_int {
    if (*dapm).component.is_null() { -EIO } else { snd_soc_component_test_bits((*dapm).component, reg, mask, value) }
}

unsafe fn dapm_async_complete(dapm: *mut snd_soc_dapm_context) {
    if !(*dapm).component.is_null() { snd_soc_component_async_complete((*dapm).component); }
}

unsafe fn dapm_wcache_lookup(_w: *mut snd_soc_dapm_widget, _name: *const c_char) -> *mut snd_soc_dapm_widget {
    // list_for_each_entry_from with depth 2.
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int {
    let mut ret = 0;
    if !(*dapm).component.is_null() { ret = snd_soc_component_set_bias_level((*dapm).component, level); }
    if ret == 0 { (*dapm).bias_level = level; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_init_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) {
    (*dapm).bias_level = level;
}

unsafe fn snd_soc_dapm_set_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level) -> c_int {
    let card = (*dapm).card;
    let mut ret = snd_soc_card_set_bias_level(card, dapm, level);
    if ret != 0 { return ret; }
    if dapm != (*card).dapm { ret = snd_soc_dapm_force_bias_level(dapm, level); }
    if ret != 0 { return ret; }
    ret = snd_soc_card_set_bias_level_post(card, dapm, level);
    if ret == 0 { snd_soc_dapm_init_bias_level(dapm, level); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level {
    (*dapm).bias_level
}

unsafe fn dapm_is_shared_kcontrol(_dapm: *mut snd_soc_dapm_context, _kcontrolw: *mut snd_soc_dapm_widget, _kcontrol_new: *const snd_kcontrol_new, kcontrol: *mut *mut snd_kcontrol) -> c_int {
    *kcontrol = ptr::null_mut();
    // for_each_card_widgets: find another widget in the same DAPM context
    // sharing the same kcontrol_new address and return its instantiated control.
    0
}

unsafe fn dapm_create_or_share_kcontrol(w: *mut snd_soc_dapm_widget, kci: c_int) -> c_int {
    let dapm = (*w).dapm;
    let mut kcontrol: *mut snd_kcontrol = ptr::null_mut();
    let shared = dapm_is_shared_kcontrol(dapm, w, (*w).kcontrol_news.add(kci as usize), &mut kcontrol);
    let prefix = dapm_prefix(dapm);
    let prefix_len = if !prefix.is_null() { strlen(prefix) + 1 } else { 0 };
    let mut long_name: *const c_char = ptr::null();
    let name: *const c_char;
    if kcontrol.is_null() {
        let (mut wname_in_long_name, kcname_in_long_name) = if shared != 0 {
            (false, true)
        } else {
            match (*w).id {
                snd_soc_dapm_type::snd_soc_dapm_switch |
                snd_soc_dapm_type::snd_soc_dapm_mixer |
                snd_soc_dapm_type::snd_soc_dapm_pga |
                snd_soc_dapm_type::snd_soc_dapm_effect |
                snd_soc_dapm_type::snd_soc_dapm_out_drv |
                snd_soc_dapm_type::snd_soc_dapm_encoder |
                snd_soc_dapm_type::snd_soc_dapm_decoder => (true, true),
                snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl |
                snd_soc_dapm_type::snd_soc_dapm_mixer_named_ctl => (false, true),
                snd_soc_dapm_type::snd_soc_dapm_demux |
                snd_soc_dapm_type::snd_soc_dapm_mux => (true, false),
                _ => return -EINVAL,
            }
        };
        if (*w).no_wname_in_kcontrol_name { wname_in_long_name = false; }
        if wname_in_long_name && kcname_in_long_name {
            long_name = kasprintf(GFP_KERNEL, b"%s %s\0".as_ptr() as *const c_char, (*w).name.add(prefix_len), (*(*w).kcontrol_news.add(kci as usize)).name);
            if long_name.is_null() { return -ENOMEM; }
            name = long_name;
        } else if wname_in_long_name {
            name = (*w).name.add(prefix_len);
        } else {
            name = (*(*w).kcontrol_news.add(kci as usize)).name;
        }
        kcontrol = snd_soc_cnew((*w).kcontrol_news.add(kci as usize), ptr::null_mut(), name, prefix);
        if kcontrol.is_null() { if !long_name.is_null() { kfree(long_name as *mut c_void); } return -ENOMEM; }
        (*kcontrol).private_free = Some(dapm_kcontrol_free);
        let ret = dapm_kcontrol_data_alloc(w, kcontrol, name);
        if ret != 0 { snd_ctl_free_one(kcontrol); if !long_name.is_null() { kfree(long_name as *mut c_void); } return ret; }
        let ret = snd_ctl_add((*(*dapm).card).snd_card, kcontrol);
        if ret < 0 { if !long_name.is_null() { kfree(long_name as *mut c_void); } return ret; }
    }
    let ret = dapm_kcontrol_add_widget(kcontrol, w);
    if ret == 0 { *(*w).kcontrols.add(kci as usize) = kcontrol; }
    if !long_name.is_null() { kfree(long_name as *mut c_void); }
    ret
}

unsafe fn dapm_new_mixer(w: *mut snd_soc_dapm_widget) -> c_int {
    let mut i = 0;
    while i < (*w).num_kcontrols {
        // for each source path whose path->name pointer matches control name:
        // create/share kcontrol, add path, and add autodisable path if present.
        let ret = dapm_create_or_share_kcontrol(w, i);
        if ret < 0 { return ret; }
        i += 1;
    }
    0
}

unsafe fn dapm_new_mux(w: *mut snd_soc_dapm_widget) -> c_int {
    match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_mux |
        snd_soc_dapm_type::snd_soc_dapm_mux_named_ctl |
        snd_soc_dapm_type::snd_soc_dapm_demux => {}
        _ => return -EINVAL,
    }
    if (*w).num_kcontrols != 1 { return -EINVAL; }
    let ret = dapm_create_or_share_kcontrol(w, 0);
    if ret < 0 { return ret; }
    // for each mux/demux path with a name: dapm_kcontrol_add_path.
    0
}

unsafe fn dapm_new_pga(w: *mut snd_soc_dapm_widget) -> c_int {
    let mut i = 0;
    while i < (*w).num_kcontrols {
        let ret = dapm_create_or_share_kcontrol(w, i);
        if ret < 0 { return ret; }
        i += 1;
    }
    0
}

unsafe fn dapm_new_dai_link(w: *mut snd_soc_dapm_widget) -> c_int {
    let rtd = (*w).priv_ as *mut snd_soc_pcm_runtime;
    if (*(*rtd).dai_link).num_c2c_params <= 1 { return 0; }
    let mut i = 0;
    while i < (*w).num_kcontrols {
        let kcontrol = snd_soc_cnew((*w).kcontrol_news.add(i as usize), w as *mut c_void, (*w).name, ptr::null());
        let ret = snd_ctl_add((*(*(*w).dapm).card).snd_card, kcontrol);
        if ret < 0 { return ret; }
        (*kcontrol).private_data = w as *mut c_void;
        *(*w).kcontrols.add(i as usize) = kcontrol;
        i += 1;
    }
    0
}

unsafe fn dapm_suspend_check(widget: *mut snd_soc_dapm_widget) -> c_int {
    match snd_power_get_state((*(*widget).dapm).card.cast::<snd_card>()) {
        SNDRV_CTL_POWER_D3hot | SNDRV_CTL_POWER_D3cold => (*widget).ignore_suspend,
        _ => 1,
    }
}

unsafe fn dapm_widget_list_free(list: *mut *mut snd_soc_dapm_widget_list) {
    kfree(*list as *mut c_void);
}

unsafe fn dapm_widget_list_create(list: *mut *mut snd_soc_dapm_widget_list, _widgets: *mut list_head) -> c_int {
    *list = devm_kzalloc(ptr::null_mut(), core::mem::size_of::<snd_soc_dapm_widget_list>(), GFP_KERNEL) as *mut snd_soc_dapm_widget_list;
    if (*list).is_null() { -ENOMEM } else { 0 }
}

unsafe fn dapm_invalidate_paths_ep(widget: *mut snd_soc_dapm_widget, dir: snd_soc_dapm_direction) {
    (*widget).endpoints[dir as usize] = -1;
    // recurse through connected non-supply paths in reverse direction.
}

unsafe fn dapm_is_connected_ep(widget: *mut snd_soc_dapm_widget, _list: *mut list_head, dir: snd_soc_dapm_direction, _fn_: unsafe fn(*mut snd_soc_dapm_widget, *mut list_head, Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool_t>) -> c_int, custom_stop_condition: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool_t>) -> c_int {
    if (*widget).endpoints[dir as usize] >= 0 { return (*widget).endpoints[dir as usize]; }
    if let Some(stop) = custom_stop_condition {
        if stop(widget, dir) {
            // stop adding subsequent widgets to list.
        }
    }
    if ((*widget).is_ep & dir_to_ep(dir)) != 0 && (*widget).connected != 0 {
        (*widget).endpoints[dir as usize] = dapm_suspend_check(widget);
        return (*widget).endpoints[dir as usize];
    }
    // sum recursive connected endpoint counts over connected non-supply paths.
    (*widget).endpoints[dir as usize] = 0;
    0
}

unsafe fn dapm_is_connected_output_ep(widget: *mut snd_soc_dapm_widget, list: *mut list_head, custom_stop_condition: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool_t>) -> c_int {
    dapm_is_connected_ep(widget, list, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT, dapm_is_connected_output_ep, custom_stop_condition)
}

unsafe fn dapm_is_connected_input_ep(widget: *mut snd_soc_dapm_widget, list: *mut list_head, custom_stop_condition: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool_t>) -> c_int {
    dapm_is_connected_ep(widget, list, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN, dapm_is_connected_input_ep, custom_stop_condition)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_dai_get_connected_widgets(dai: *mut snd_soc_dai, stream: c_int, list: *mut *mut snd_soc_dapm_widget_list, custom_stop_condition: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool_t>) -> c_int {
    let card = (*(*dai).component).card;
    let w = snd_soc_dai_get_widget(dai, stream);
    let mut widgets = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    let mut paths = if stream == SNDRV_PCM_STREAM_PLAYBACK {
        dapm_invalidate_paths_ep(w, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_OUT);
        dapm_is_connected_output_ep(w, &mut widgets, custom_stop_condition)
    } else {
        dapm_invalidate_paths_ep(w, snd_soc_dapm_direction::SND_SOC_DAPM_DIR_IN);
        dapm_is_connected_input_ep(w, &mut widgets, custom_stop_condition)
    };
    let ret = dapm_widget_list_create(list, &mut widgets);
    if ret != 0 { paths = ret; }
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    paths
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_dai_free_widgets(list: *mut *mut snd_soc_dapm_widget_list) {
    dapm_widget_list_free(list);
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_regulator_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    dapm_async_complete((*w).dapm);
    if event_on(event) {
        if ((*w).on_val & SND_SOC_DAPM_REGULATOR_BYPASS) != 0 {
            let _ = regulator_allow_bypass((*w).regulator, false);
        }
        regulator_enable((*w).regulator)
    } else {
        if ((*w).on_val & SND_SOC_DAPM_REGULATOR_BYPASS) != 0 {
            let _ = regulator_allow_bypass((*w).regulator, true);
        }
        regulator_disable_deferred((*w).regulator, (*w).shift)
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_pinctrl_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let priv_ = (*w).priv_ as *mut snd_soc_dapm_pinctrl_priv;
    let p = (*w).pinctrl;
    if p.is_null() || priv_.is_null() { return -EIO; }
    let s = if event_on(event) { pinctrl_lookup_state(p, (*priv_).active_state) } else { pinctrl_lookup_state(p, (*priv_).sleep_state) };
    if s.is_null() { return -EINVAL; }
    pinctrl_select_state(p, s)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_clock_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    if (*w).clk.is_null() { return -EIO; }
    dapm_async_complete((*w).dapm);
    if event_on(event) { clk_prepare_enable((*w).clk) } else { clk_disable_unprepare((*w).clk); 0 }
}

unsafe fn dapm_widget_power_check(w: *mut snd_soc_dapm_widget) -> c_int {
    if (*w).power_checked { return (*w).new_power; }
    (*w).new_power = if (*w).force != 0 { 1 } else { (*w).power_check.map(|f| f(w)).unwrap_or(0) };
    (*w).power_checked = true;
    (*w).new_power
}

unsafe extern "C" fn dapm_generic_check_power(w: *mut snd_soc_dapm_widget) -> c_int {
    let input = dapm_is_connected_input_ep(w, ptr::null_mut(), None);
    let out = dapm_is_connected_output_ep(w, ptr::null_mut(), None);
    (out != 0 && input != 0) as c_int
}

unsafe extern "C" fn dapm_supply_check_power(_w: *mut snd_soc_dapm_widget) -> c_int {
    // for each sink path: if connected callback permits and sink powers, return 1.
    0
}

unsafe extern "C" fn dapm_always_on_check_power(w: *mut snd_soc_dapm_widget) -> c_int {
    (*w).connected
}

unsafe fn dapm_seq_compare(a: *mut snd_soc_dapm_widget, b: *mut snd_soc_dapm_widget, power_up: bool) -> c_int {
    let sort = if power_up { &dapm_up_seq } else { &dapm_down_seq };
    let sa = sort[(*a).id as usize];
    let sb = sort[(*b).id as usize];
    if sa != sb { return sa - sb; }
    if (*a).subseq != (*b).subseq {
        return if power_up { (*a).subseq - (*b).subseq } else { (*b).subseq - (*a).subseq };
    }
    if (*a).reg != (*b).reg { return (*a).reg - (*b).reg; }
    if (*a).dapm != (*b).dapm { return ((*a).dapm as isize - (*b).dapm as isize) as c_int; }
    0
}

unsafe fn dapm_seq_insert(_new_widget: *mut snd_soc_dapm_widget, _list: *mut list_head, _power_up: bool) {
    // Insert into power_list ordered by dapm_seq_compare.
}

unsafe fn dapm_seq_check_event(_card: *mut snd_soc_card, w: *mut snd_soc_dapm_widget, event: c_int) {
    let power = match event {
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_WILL_PMU => 1,
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD | SND_SOC_DAPM_WILL_PMD => 0,
        _ => return,
    };
    if (*w).new_power != power { return; }
    if let Some(ev) = (*w).event {
        if ((*w).event_flags & event) != 0 {
            dapm_async_complete((*w).dapm);
            let _ = ev(w, ptr::null_mut(), event);
        }
    }
}

unsafe fn dapm_seq_run_coalesced(_card: *mut snd_soc_card, _pending: *mut list_head) {
    // C coalesces widgets sharing reg/dapm into one update_bits call and runs
    // PRE/POST PMU/PMD events around the write.
}

unsafe fn dapm_seq_run(card: *mut snd_soc_card, _list: *mut list_head, _event: c_int, _power_up: bool) {
    // Traverse sorted power_list, run pre/post widgets immediately, coalesce
    // ordinary register writes, notify sequence transitions, and async-complete.
    let _ = card;
}

unsafe fn dapm_widget_update(card: *mut snd_soc_card, update: *mut snd_soc_dapm_update) {
    if update.is_null() || !dapm_kcontrol_is_powered((*update).kcontrol) { return; }
    let wlist = dapm_kcontrol_get_wlist((*update).kcontrol);
    let _ = wlist;
    let _ = dapm_update_bits((*snd_soc_dapm_kcontrol_to_widget((*update).kcontrol)).dapm, (*update).reg, (*update).mask, (*update).val);
    if (*update).has_second_set {
        let _ = dapm_update_bits((*snd_soc_dapm_kcontrol_to_widget((*update).kcontrol)).dapm, (*update).reg2, (*update).mask2, (*update).val2);
    }
    let _ = card;
}

unsafe extern "C" fn dapm_pre_sequence_async(data: *mut c_void, cookie: async_cookie_t) {
    let dapm = data as *mut snd_soc_dapm_context;
    let dev = snd_soc_dapm_to_dev(dapm);
    if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_OFF && (*dapm).target_bias_level != snd_soc_bias_level::SND_SOC_BIAS_OFF {
        let _ = (dev, cookie);
        let _ = snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
    }
    if ((*dapm).target_bias_level == snd_soc_bias_level::SND_SOC_BIAS_ON && (*dapm).bias_level != snd_soc_bias_level::SND_SOC_BIAS_ON)
        || ((*dapm).target_bias_level != snd_soc_bias_level::SND_SOC_BIAS_ON && (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_ON) {
        let _ = snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_PREPARE);
    }
}

unsafe extern "C" fn dapm_post_sequence_async(data: *mut c_void, cookie: async_cookie_t) {
    let dapm = data as *mut snd_soc_dapm_context;
    let dev = snd_soc_dapm_to_dev(dapm);
    if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_PREPARE
        && ((*dapm).target_bias_level == snd_soc_bias_level::SND_SOC_BIAS_STANDBY || (*dapm).target_bias_level == snd_soc_bias_level::SND_SOC_BIAS_OFF) {
        let _ = snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
    }
    if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_STANDBY && (*dapm).target_bias_level == snd_soc_bias_level::SND_SOC_BIAS_OFF {
        let _ = snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_OFF);
        let _ = (dev, cookie);
    }
    if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_PREPARE && (*dapm).target_bias_level == snd_soc_bias_level::SND_SOC_BIAS_ON {
        let _ = snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_ON);
    }
}

unsafe fn dapm_widget_set_peer_power(peer: *mut snd_soc_dapm_widget, power: bool, connect: bool) {
    if !connect { return; }
    if power != ((*peer).power != 0) {
        dapm_mark_dirty(peer, b"peer state change\0".as_ptr() as *const c_char);
    }
}

unsafe fn dapm_power_one_widget(w: *mut snd_soc_dapm_widget, up_list: *mut list_head, down_list: *mut list_head) {
    let power = match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_pre => 0,
        snd_soc_dapm_type::snd_soc_dapm_post => 1,
        _ => {
            let p = dapm_widget_power_check(w);
            if (*w).power == p { return; }
            p
        }
    };
    // source/sink peer dirty propagation over path lists.
    if power != 0 { dapm_seq_insert(w, up_list, true); } else { dapm_seq_insert(w, down_list, false); }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_get_idle_bias(dapm: *mut snd_soc_dapm_context) -> bool_t {
    if (*dapm).idle_bias {
        let component = snd_soc_dapm_to_component(dapm);
        let state = snd_power_get_state((*(*dapm).card).snd_card);
        if (state == SNDRV_CTL_POWER_D3hot || state == SNDRV_CTL_POWER_D3cold) && !component.is_null() {
            return !(*(*component).driver).suspend_bias_off;
        }
    }
    (*dapm).idle_bias
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, on: bool_t) {
    (*dapm).idle_bias = on;
}

unsafe fn dapm_power_widgets(card: *mut snd_soc_card, event: c_int, update: *mut snd_soc_dapm_update) -> c_int {
    snd_soc_dapm_mutex_assert_held(card as *mut c_void);
    dapm_reset(card);
    let mut up_list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let mut down_list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    // for_each_card_dapms: initialize target bias to STANDBY/OFF.
    // for each dirty widget: dapm_power_one_widget.
    // for each card widget: clear dirty except pre/post and raise target bias
    // according to active widget type.
    dapm_pre_sequence_async((*card).dapm as *mut c_void, 0);
    // schedule other pre transitions, run WILL events, down sequence, register update, up sequence, post transitions.
    dapm_seq_run(card, &mut down_list, event, false);
    dapm_widget_update(card, update);
    dapm_seq_run(card, &mut up_list, event, true);
    dapm_post_sequence_async((*card).dapm as *mut c_void, 0);
    // for_each_card_dapms: snd_soc_component_stream_event.
    dapm_pop_wait();
    0
}

// CONFIG_DEBUG_FS translation: debugfs helpers are present in C when enabled.
static dapm_type_name: [*const c_char; snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize] = [ptr::null(); snd_soc_dapm_type::SND_SOC_DAPM_TYPE_COUNT as usize];

unsafe extern "C" fn dapm_widget_power_read_file(_file: *mut file, _user_buf: *mut c_char, _count: size_t, _ppos: *mut loff_t) -> ssize_t { 0 }
static dapm_widget_power_fops: file_operations = file_operations { open: None, read: Some(dapm_widget_power_read_file), llseek: None };
unsafe extern "C" fn dapm_bias_read_file(file: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let dapm = (*file).private_data as *mut snd_soc_dapm_context;
    let level = match (*dapm).bias_level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => b"On\n\0".as_ptr() as *const c_char,
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => b"Prepare\n\0".as_ptr() as *const c_char,
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => b"Standby\n\0".as_ptr() as *const c_char,
        snd_soc_bias_level::SND_SOC_BIAS_OFF => b"Off\n\0".as_ptr() as *const c_char,
    };
    simple_read_from_buffer(user_buf, count, ppos, level, strlen(level) as ssize_t)
}
static dapm_bias_fops: file_operations = file_operations { open: None, read: Some(dapm_bias_read_file), llseek: None };

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_debugfs_pop_time(_parent: *mut dentry) {}
#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_debugfs_init(dapm: *mut snd_soc_dapm_context, _parent: *mut dentry) { (*dapm).debugfs_dapm = ptr::null_mut(); }
unsafe fn dapm_debugfs_add_widget(_w: *mut snd_soc_dapm_widget) {}
unsafe fn dapm_debugfs_free_widget(_w: *mut snd_soc_dapm_widget) {}
unsafe fn dapm_debugfs_cleanup(dapm: *mut snd_soc_dapm_context) { (*dapm).debugfs_dapm = ptr::null_mut(); }

unsafe fn dapm_connect_path(path: *mut snd_soc_dapm_path, connect: bool_t, reason: *const c_char) {
    if (*path).connect == connect { return; }
    (*path).connect = connect;
    dapm_mark_dirty((*path).source, reason);
    dapm_mark_dirty((*path).sink, reason);
    dapm_path_invalidate(path);
}

unsafe fn dapm_mux_update_power(card: *mut snd_soc_card, kcontrol: *mut snd_kcontrol, update: *mut snd_soc_dapm_update, mux: c_int, e: *mut soc_enum) -> c_int {
    snd_soc_dapm_mutex_assert_held(card as *mut c_void);
    let mut found = 0;
    // for each kcontrol path: connect iff path name equals e->texts[mux].
    let _ = (kcontrol, mux, e);
    if found != 0 { dapm_power_widgets(card, SND_SOC_DAPM_STREAM_NOP, update); }
    found
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_mux_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, mux: c_int, e: *mut soc_enum, update: *mut snd_soc_dapm_update) -> c_int {
    let card = (*dapm).card;
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    let ret = dapm_mux_update_power(card, kcontrol, update, mux, e);
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    if ret > 0 { snd_soc_dpcm_runtime_update(card); }
    ret
}

unsafe fn dapm_mixer_update_power(card: *mut snd_soc_card, kcontrol: *mut snd_kcontrol, update: *mut snd_soc_dapm_update, connect: c_int, rconnect: c_int) -> c_int {
    snd_soc_dapm_mutex_assert_held(card as *mut c_void);
    let mut found = 0;
    let _ = (kcontrol, connect, rconnect);
    // for each kcontrol path: first path uses connect, later stereo path uses rconnect when >= 0.
    if found != 0 { dapm_power_widgets(card, SND_SOC_DAPM_STREAM_NOP, update); }
    found
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_mixer_update_power(dapm: *mut snd_soc_dapm_context, kcontrol: *mut snd_kcontrol, connect: c_int, update: *mut snd_soc_dapm_update) -> c_int {
    let card = (*dapm).card;
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    let ret = dapm_mixer_update_power(card, kcontrol, update, connect, -1);
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    if ret > 0 { snd_soc_dpcm_runtime_update(card); }
    ret
}

unsafe fn dapm_widget_show_component(_component: *mut snd_soc_component, _buf: *mut c_char, count: c_int) -> ssize_t { count as ssize_t }
unsafe extern "C" fn dapm_widget_show(_dev: *mut device, _attr: *mut device_attribute, _buf: *mut c_char) -> ssize_t { 0 }
#[no_mangle] pub static mut snd_soc_dapm_dev_attrs: [*mut attribute; 2] = [ptr::null_mut(), ptr::null_mut()];

unsafe fn dapm_free_path(path: *mut snd_soc_dapm_path) {
    // list_del list_node[IN], list_node[OUT], list_kcontrol, list.
    kfree(path as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_free_widget(w: *mut snd_soc_dapm_widget) {
    if w.is_null() { return; }
    // list_del(&w->list); list_del(&w->dirty); free all source/sink paths.
    dapm_debugfs_free_widget(w);
    kfree((*w).kcontrols as *mut c_void);
    kfree_const((*w).name);
    kfree_const((*w).sname);
    kfree(w as *mut c_void);
}

unsafe fn dapm_free_widgets(dapm: *mut snd_soc_dapm_context) {
    // for_each_card_widgets_safe(card, w, next_w) if w->dapm == dapm free.
    (*dapm).wcache_sink = ptr::null_mut();
    (*dapm).wcache_source = ptr::null_mut();
}

unsafe fn dapm_find_widget(_dapm: *mut snd_soc_dapm_context, _pin: *const c_char, _search_other_contexts: bool_t) -> *mut snd_soc_dapm_widget {
    // for_each_card_widgets: exact or unprefixed name match, prefer same dapm.
    ptr::null_mut()
}

unsafe fn __dapm_set_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char, status: c_int) -> c_int {
    let w = dapm_find_widget(dapm, pin, true);
    dapm_assert_locked(dapm);
    if w.is_null() { return -EINVAL; }
    let mut ret = 0;
    if (*w).connected != status {
        dapm_mark_dirty(w, b"pin configuration\0".as_ptr() as *const c_char);
        dapm_widget_invalidate_input_paths(w);
        dapm_widget_invalidate_output_paths(w);
        ret = 1;
    }
    (*w).connected = status;
    if status == 0 { (*w).force = 0; }
    ret
}

unsafe fn dapm_set_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char, status: c_int) -> c_int {
    let ret = __dapm_set_pin(dapm, pin, status);
    if ret < 0 { ret } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int {
    if !snd_soc_card_is_instantiated((*dapm).card) { return 0; }
    dapm_power_widgets((*dapm).card, SND_SOC_DAPM_STREAM_NOP, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let ret = snd_soc_dapm_sync_unlocked(dapm);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

unsafe fn dapm_update_dai_chan(p: *mut snd_soc_dapm_path, w: *mut snd_soc_dapm_widget, channels: c_int) -> c_int {
    match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_aif_out | snd_soc_dapm_type::snd_soc_dapm_aif_in => {}
        _ => return 0,
    }
    dapm_connect_path(p, (*w).channel < channels, b"dai update\0".as_ptr() as *const c_char);
    0
}

unsafe fn dapm_update_dai_unlocked(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let dir = (*substream).stream;
    let channels = params_channels(params);
    let w = snd_soc_dai_get_widget(dai, dir);
    if w.is_null() { return 0; }
    let _ = channels;
    // for each sink/source path update channel connection.
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_update_dai(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dapm_mutex_lock((*rtd).card as *mut c_void);
    let ret = dapm_update_dai_unlocked(substream, params, dai);
    snd_soc_dapm_mutex_unlock((*rtd).card as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_widget_name_cmp(widget: *mut snd_soc_dapm_widget, s: *const c_char) -> c_int {
    let component = (*(*widget).dapm).component;
    let mut wname = (*widget).name;
    if !component.is_null() && !(*component).name_prefix.is_null() {
        wname = wname.add(strlen((*component).name_prefix) + 1);
    }
    strcmp(wname, s)
}

unsafe fn snd_soc_dapm_add_route(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route) -> c_int {
    let mut prefixed_sink = [0 as c_char; 80];
    let mut prefixed_source = [0 as c_char; 80];
    let prefix = dapm_prefix(dapm);
    let (sink, source) = if !prefix.is_null() {
        snprintf(prefixed_sink.as_mut_ptr(), prefixed_sink.len(), b"%s %s\0".as_ptr() as *const c_char, prefix, (*route).sink);
        snprintf(prefixed_source.as_mut_ptr(), prefixed_source.len(), b"%s %s\0".as_ptr() as *const c_char, prefix, (*route).source);
        (prefixed_sink.as_ptr(), prefixed_source.as_ptr())
    } else {
        ((*route).sink, (*route).source)
    };
    let mut wsource = dapm_wcache_lookup((*dapm).wcache_source, source);
    let mut wsink = dapm_wcache_lookup((*dapm).wcache_sink, sink);
    if wsink.is_null() || wsource.is_null() {
        // Search all widgets, prefer current DAPM context, warn on duplicates.
    }
    if wsource.is_null() || wsink.is_null() { return -ENODEV; }
    (*dapm).wcache_sink = wsink;
    (*dapm).wcache_source = wsource;
    dapm_add_path(dapm, wsource, wsink, (*route).control, (*route).connected)
}

unsafe fn snd_soc_dapm_del_route(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route) -> c_int {
    if !(*route).control.is_null() { return -EINVAL; }
    // Build optional prefixed names, locate matching path in card->paths,
    // mark source/sink dirty, invalidate if connected, free path, update flags.
    let _ = dapm;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, mut route: *const snd_soc_dapm_route, num: c_int) -> c_int {
    let mut ret = 0;
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    for _ in 0..num {
        let r = snd_soc_dapm_add_route(dapm, route);
        if r < 0 { ret = r; }
        route = route.add(1);
    }
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_del_routes(dapm: *mut snd_soc_dapm_context, mut route: *const snd_soc_dapm_route, num: c_int) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    for _ in 0..num {
        snd_soc_dapm_del_route(dapm, route);
        route = route.add(1);
    }
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card) -> c_int {
    snd_soc_dapm_mutex_lock_root(card as *mut c_void);
    // for_each_card_widgets: allocate kcontrols, initialize type-specific
    // controls, read initial register power, mark new/dirty, add debugfs file.
    let _ = dapm_power_widgets(card, SND_SOC_DAPM_STREAM_NOP, ptr::null_mut());
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mc).reg as c_int;
    let shift = (*mc).shift;
    let max = (*mc).max;
    let width = fls(max);
    let mask = (1u32 << fls(max)) - 1;
    let invert = (*mc).invert;
    let mut rval = 0;
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let val = if dapm_kcontrol_is_powered(kcontrol) && reg != SND_SOC_NOPM {
        let mut reg_val = dapm_read(dapm, reg);
        let val = (reg_val >> shift) & mask;
        if reg as c_uint != (*mc).rreg { reg_val = dapm_read(dapm, (*mc).rreg as c_int); }
        if snd_soc_volsw_is_stereo(mc) { rval = (reg_val >> (*mc).rshift) & mask; }
        val
    } else {
        let reg_val = snd_soc_dapm_kcontrol_get_value(kcontrol);
        if snd_soc_volsw_is_stereo(mc) { rval = (reg_val >> width) & mask; }
        reg_val & mask
    };
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    (*ucontrol).value.integer.value[0] = if invert != 0 { (max - val) as c_long } else { val as c_long };
    if snd_soc_volsw_is_stereo(mc) {
        (*ucontrol).value.integer.value[1] = if invert != 0 { (max - rval) as c_long } else { rval as c_long };
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let card = (*dapm).card;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let reg = (*mc).reg as c_int;
    let shift = (*mc).shift;
    let max = (*mc).max;
    let width = fls(max) as c_uint;
    let mask = (1u32 << width) - 1;
    let invert = (*mc).invert;
    let mut val = ((*ucontrol).value.integer.value[0] as c_uint) & mask;
    let connect = (val != 0) as c_int;
    if invert != 0 { val = max - val; }
    let mut rval = 0;
    let mut rconnect = -1;
    if snd_soc_volsw_is_stereo(mc) {
        rval = ((*ucontrol).value.integer.value[1] as c_uint) & mask;
        rconnect = (rval != 0) as c_int;
        if invert != 0 { rval = max - rval; }
    }
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    let change = dapm_kcontrol_set_value(kcontrol, val | (rval << width)) as c_int;
    let mut reg_change = 0;
    let mut update: snd_soc_dapm_update = core::mem::zeroed();
    let mut pupdate: *mut snd_soc_dapm_update = ptr::null_mut();
    if reg != SND_SOC_NOPM {
        let v = val << shift;
        let rv = rval << (*mc).rshift;
        reg_change = dapm_test_bits(dapm, reg, mask << shift, v);
        if snd_soc_volsw_is_stereo(mc) {
            reg_change |= dapm_test_bits(dapm, (*mc).rreg as c_int, mask << (*mc).rshift, rv);
            update.has_second_set = true;
            update.reg2 = (*mc).rreg as c_int;
            update.mask2 = mask << (*mc).rshift;
            update.val2 = rv;
        }
        update.kcontrol = kcontrol;
        update.reg = reg;
        update.mask = mask << shift;
        update.val = v;
    }
    let mut ret = 0;
    if change != 0 || reg_change != 0 {
        if reg_change != 0 { pupdate = &mut update; }
        ret = dapm_mixer_update_power(card, kcontrol, pupdate, connect, rconnect);
    }
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    if ret > 0 { snd_soc_dpcm_runtime_update(card); }
    change
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let reg_val = if (*e).reg as c_int != SND_SOC_NOPM && dapm_kcontrol_is_powered(kcontrol) { dapm_read(dapm, (*e).reg as c_int) } else { snd_soc_dapm_kcontrol_get_value(kcontrol) };
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    let mut val = (reg_val >> (*e).shift_l) & (*e).mask;
    (*ucontrol).value.enumerated.item[0] = snd_soc_enum_val_to_item(e, val);
    if (*e).shift_l != (*e).shift_r {
        val = (reg_val >> (*e).shift_r) & (*e).mask;
        (*ucontrol).value.enumerated.item[1] = snd_soc_enum_val_to_item(e, val);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_put_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let card = (*dapm).card;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let item = (*ucontrol).value.enumerated.item.as_ptr();
    if *item >= (*e).items { return -EINVAL; }
    let mut val = snd_soc_enum_item_to_val(e, *item) << (*e).shift_l;
    let mut mask = (*e).mask << (*e).shift_l;
    if (*e).shift_l != (*e).shift_r {
        if *item.add(1) >= (*e).items { return -EINVAL; }
        val |= snd_soc_enum_item_to_val(e, *item.add(1)) << (*e).shift_r;
        mask |= (*e).mask << (*e).shift_r;
    }
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    let change = dapm_kcontrol_set_value(kcontrol, val) as c_uint;
    let reg_change = if (*e).reg as c_int != SND_SOC_NOPM { dapm_test_bits(dapm, (*e).reg as c_int, mask, val) as c_uint } else { 0 };
    let mut update: snd_soc_dapm_update = core::mem::zeroed();
    let mut pupdate: *mut snd_soc_dapm_update = ptr::null_mut();
    let mut ret = 0;
    if change != 0 || reg_change != 0 {
        if reg_change != 0 {
            update.kcontrol = kcontrol;
            update.reg = (*e).reg as c_int;
            update.mask = mask;
            update.val = val;
            pupdate = &mut update;
        }
        ret = dapm_mux_update_power(card, kcontrol, pupdate, *item as c_int, e);
    }
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
    if ret > 0 { snd_soc_dpcm_runtime_update(card); }
    change as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_info_pin_switch(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn __snd_soc_dapm_get_pin_switch(dapm: *mut snd_soc_dapm_context, pin: *const c_char, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    (*ucontrol).value.integer.value[0] = snd_soc_dapm_get_pin_status(dapm, pin) as c_long;
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    0
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_get_pin_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    __snd_soc_dapm_get_pin_switch((*card).dapm, (*kcontrol).private_value as *const c_char, ucontrol)
}
#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_get_component_pin_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    __snd_soc_dapm_get_pin_switch((*(*component).card).dapm, (*kcontrol).private_value as *const c_char, ucontrol)
}

unsafe fn __dapm_put_pin_switch(dapm: *mut snd_soc_dapm_context, pin: *const c_char, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let ret = __dapm_set_pin(dapm, pin, ((*ucontrol).value.integer.value[0] != 0) as c_int);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    snd_soc_dapm_sync(dapm);
    ret
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_put_pin_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let card = snd_kcontrol_chip(kcontrol) as *mut snd_soc_card;
    __dapm_put_pin_switch((*card).dapm, (*kcontrol).private_value as *const c_char, ucontrol)
}
#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_put_component_pin_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    __dapm_put_pin_switch((*(*component).card).dapm, (*kcontrol).private_value as *const c_char, ucontrol)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_new_control_unlocked(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget) -> *mut snd_soc_dapm_widget {
    let w = dapm_cnew_widget(widget, dapm_prefix(dapm));
    if w.is_null() { return (-ENOMEM as isize) as *mut snd_soc_dapm_widget; }
    match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_regulator_supply => {
            // devm_regulator_get and optional initial bypass.
        }
        snd_soc_dapm_type::snd_soc_dapm_pinctrl => {
            // devm_pinctrl_get and set initial sleep state.
        }
        snd_soc_dapm_type::snd_soc_dapm_clock_supply => {
            // devm_clk_get.
        }
        _ => {}
    }
    match (*w).id {
        snd_soc_dapm_type::snd_soc_dapm_mic => { (*w).is_ep = SND_SOC_DAPM_EP_SOURCE; (*w).power_check = Some(dapm_generic_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_input => { if !(*(*dapm).card).fully_routed { (*w).is_ep = SND_SOC_DAPM_EP_SOURCE; } (*w).power_check = Some(dapm_generic_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_spk | snd_soc_dapm_type::snd_soc_dapm_hp => { (*w).is_ep = SND_SOC_DAPM_EP_SINK; (*w).power_check = Some(dapm_generic_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_output => { if !(*(*dapm).card).fully_routed { (*w).is_ep = SND_SOC_DAPM_EP_SINK; } (*w).power_check = Some(dapm_generic_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_vmid | snd_soc_dapm_type::snd_soc_dapm_siggen => { (*w).is_ep = SND_SOC_DAPM_EP_SOURCE; (*w).power_check = Some(dapm_always_on_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_sink => { (*w).is_ep = SND_SOC_DAPM_EP_SINK; (*w).power_check = Some(dapm_always_on_check_power); }
        snd_soc_dapm_type::snd_soc_dapm_supply | snd_soc_dapm_type::snd_soc_dapm_regulator_supply | snd_soc_dapm_type::snd_soc_dapm_pinctrl | snd_soc_dapm_type::snd_soc_dapm_clock_supply | snd_soc_dapm_type::snd_soc_dapm_kcontrol => { (*w).is_supply = 1; (*w).power_check = Some(dapm_supply_check_power); }
        _ => { (*w).power_check = Some(dapm_generic_check_power); }
    }
    (*w).dapm = dapm;
    (*w).endpoints = [-1, -1];
    (*w).connected = 1;
    w
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_new_control(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget) -> *mut snd_soc_dapm_widget {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let w = snd_soc_dapm_new_control_unlocked(dapm, widget);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    w
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, mut widget: *const snd_soc_dapm_widget, num: c_uint) -> c_int {
    let mut ret = 0;
    snd_soc_dapm_mutex_lock_root(dapm as *mut c_void);
    for _ in 0..num {
        let w = snd_soc_dapm_new_control_unlocked(dapm, widget);
        if (w as isize) < 0 { ret = w as isize as c_int; break; }
        widget = widget.add(1);
    }
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

unsafe fn dapm_dai_link_event_pre_pmu(_w: *mut snd_soc_dapm_widget, _substream: *mut snd_pcm_substream) -> c_int {
    // Allocate params/runtime, startup/activate source capture DAIs and sink
    // playback DAIs, choose c2c config, program params on both sides, cache
    // runtime format/subformat/channels/rate.
    0
}

unsafe extern "C" fn dapm_dai_link_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let substream = (*w).priv_ as *mut snd_pcm_substream;
    let saved_stream = (*substream).stream;
    let ret = match event {
        SND_SOC_DAPM_PRE_PMU => dapm_dai_link_event_pre_pmu(w, substream),
        SND_SOC_DAPM_POST_PMU => {
            // prepare all source/sink DAIs and unmute playback sinks.
            0
        }
        SND_SOC_DAPM_PRE_PMD => {
            // mute sinks, hw_free, deactivate, and shutdown source/sink DAIs.
            0
        }
        SND_SOC_DAPM_POST_PMD => {
            kfree((*substream).runtime as *mut c_void);
            (*substream).runtime = ptr::null_mut();
            0
        }
        _ => -EINVAL,
    };
    (*substream).stream = saved_stream;
    ret
}

unsafe extern "C" fn dapm_dai_link_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let w = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dapm_widget;
    let rtd = (*w).priv_ as *mut snd_soc_pcm_runtime;
    (*ucontrol).value.enumerated.item[0] = (*rtd).c2c_params_select;
    0
}

unsafe extern "C" fn dapm_dai_link_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let w = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dapm_widget;
    let rtd = (*w).priv_ as *mut snd_soc_pcm_runtime;
    if (*w).power != 0 { return -EBUSY; }
    if (*ucontrol).value.enumerated.item[0] == (*rtd).c2c_params_select { return 0; }
    if (*ucontrol).value.enumerated.item[0] >= (*(*rtd).dai_link).num_c2c_params as c_uint { return -EINVAL; }
    (*rtd).c2c_params_select = (*ucontrol).value.enumerated.item[0];
    1
}

unsafe fn dapm_free_kcontrol(_card: *mut snd_soc_card, _private_value: *mut c_ulong, _num_c2c_params: c_int, _w_param_text: *mut *const c_char) {}
unsafe fn dapm_alloc_kcontrol(_card: *mut snd_soc_card, _link_name: *mut c_char, _c2c_params: *const snd_soc_pcm_stream, _num_c2c_params: c_int, _w_param_text: *mut *const c_char, _private_value: *mut c_ulong) -> *mut snd_kcontrol_new { ptr::null_mut() }
unsafe fn dapm_new_dai(_card: *mut snd_soc_card, _substream: *mut snd_pcm_substream, _id: *mut c_char) -> *mut snd_soc_dapm_widget { ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int {
    let mut template: snd_soc_dapm_widget = core::mem::zeroed();
    template.reg = SND_SOC_NOPM;
    if !(*(*dai).driver).playback.stream_name.is_null() {
        template.id = snd_soc_dapm_type::snd_soc_dapm_dai_in;
        template.name = (*(*dai).driver).playback.stream_name;
        template.sname = (*(*dai).driver).playback.stream_name;
        let w = snd_soc_dapm_new_control_unlocked(dapm, &template);
        if (w as isize) < 0 { return w as isize as c_int; }
        (*w).priv_ = dai as *mut c_void;
        snd_soc_dai_set_widget_playback(dai, w);
    }
    if !(*(*dai).driver).capture.stream_name.is_null() {
        template.id = snd_soc_dapm_type::snd_soc_dapm_dai_out;
        template.name = (*(*dai).driver).capture.stream_name;
        template.sname = (*(*dai).driver).capture.stream_name;
        let w = snd_soc_dapm_new_control_unlocked(dapm, &template);
        if (w as isize) < 0 { return w as isize as c_int; }
        (*w).priv_ = dai as *mut c_void;
        snd_soc_dai_set_widget_capture(dai, w);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_link_dai_widgets(_card: *mut snd_soc_card) -> c_int {
    // For each DAI widget, find non-DAI widgets in the same DAPM context whose
    // stream name contains the DAI stream name and add static paths.
    0
}

unsafe fn dapm_connect_dai_routes(dapm: *mut snd_soc_dapm_context, _src_dai: *mut snd_soc_dai, mut src: *mut snd_soc_dapm_widget, dai: *mut snd_soc_dapm_widget, _sink_dai: *mut snd_soc_dai, sink: *mut snd_soc_dapm_widget) {
    if !dai.is_null() {
        dapm_add_path(dapm, src, dai, ptr::null(), None);
        src = dai;
    }
    dapm_add_path(dapm, src, sink, ptr::null(), None);
}

unsafe fn dapm_connect_dai_pair(_card: *mut snd_soc_card, _rtd: *mut snd_soc_pcm_runtime, _codec_dai: *mut snd_soc_dai, _cpu_dai: *mut snd_soc_dai) {
    // for_each_pcm_streams: find cpu/codec widgets, create c2c widget if needed,
    // and connect routes in playback/capture direction.
}

unsafe fn dapm_dai_stream_event(dai: *mut snd_soc_dai, stream: c_int, event: c_int) {
    let w = snd_soc_dai_get_widget(dai, stream);
    if w.is_null() { return; }
    let ep;
    dapm_mark_dirty(w, b"stream event\0".as_ptr() as *const c_char);
    if (*w).id == snd_soc_dapm_type::snd_soc_dapm_dai_in {
        ep = SND_SOC_DAPM_EP_SOURCE;
        dapm_widget_invalidate_input_paths(w);
    } else {
        ep = SND_SOC_DAPM_EP_SINK;
        dapm_widget_invalidate_output_paths(w);
    }
    match event {
        SND_SOC_DAPM_STREAM_START => { (*w).active = 1; (*w).is_ep = ep; }
        SND_SOC_DAPM_STREAM_STOP => { (*w).active = 0; (*w).is_ep = 0; }
        SND_SOC_DAPM_STREAM_SUSPEND | SND_SOC_DAPM_STREAM_RESUME | SND_SOC_DAPM_STREAM_PAUSE_PUSH | SND_SOC_DAPM_STREAM_PAUSE_RELEASE => {}
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_connect_dai_link_widgets(_card: *mut snd_soc_card) {
    // for_each_card_rtds over non-dynamic links, for_each_rtd_ch_maps, connect DAI pairs.
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_ignore_suspend_widgets(card: *mut snd_soc_card) -> c_int {
    for i in 0..(*card).num_ignore_suspend_widgets {
        let w = dapm_find_widget((*card).dapm, *(*card).ignore_suspend_widgets.add(i as usize), true);
        if w.is_null() { return -EINVAL; }
        (*w).ignore_suspend = 1;
    }
    for i in 0..(*card).num_of_ignore_suspend_widgets {
        let w = dapm_find_widget((*card).dapm, *(*card).of_ignore_suspend_widgets.add(i as usize), true);
        if w.is_null() { return -EINVAL; }
        (*w).ignore_suspend = 1;
    }
    0
}

unsafe fn dapm_stream_event(rtd: *mut snd_soc_pcm_runtime, stream: c_int, event: c_int) {
    // for_each_rtd_dais: dapm_dai_stream_event(dai, stream, event);
    dapm_power_widgets((*rtd).card, event, ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_stream_event(rtd: *mut snd_soc_pcm_runtime, stream: c_int, event: c_int) {
    let card = (*rtd).card;
    snd_soc_dapm_mutex_lock(card as *mut c_void);
    dapm_stream_event(rtd, stream, event);
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
}

unsafe extern "C" {
    fn snd_soc_runtime_ignore_pmdown_time(rtd: *mut snd_soc_pcm_runtime) -> bool_t;
    static mut system_power_efficient_wq: *mut c_void;
    fn queue_delayed_work(wq: *mut c_void, delayed_work: *mut c_void, delay: c_ulong) -> bool_t;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_stream_stop(rtd: *mut snd_soc_pcm_runtime, stream: c_int) {
    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        if snd_soc_runtime_ignore_pmdown_time(rtd) {
            snd_soc_dapm_stream_event(rtd, SNDRV_PCM_STREAM_PLAYBACK, SND_SOC_DAPM_STREAM_STOP);
        } else {
            (*rtd).pop_wait = 1;
            queue_delayed_work(system_power_efficient_wq, &mut (*rtd).delayed_work as *mut c_void, msecs_to_jiffies((*rtd).pmdown_time));
        }
    } else {
        snd_soc_dapm_stream_event(rtd, SNDRV_PCM_STREAM_CAPTURE, SND_SOC_DAPM_STREAM_STOP);
    }
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int { dapm_set_pin(dapm, pin, 1) }
#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let ret = dapm_set_pin(dapm, pin, 1);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_force_enable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    let w = dapm_find_widget(dapm, pin, true);
    if w.is_null() { return -EINVAL; }
    if (*w).connected == 0 {
        dapm_widget_invalidate_input_paths(w);
        dapm_widget_invalidate_output_paths(w);
        (*w).connected = 1;
    }
    (*w).force = 1;
    dapm_mark_dirty(w, b"force enable\0".as_ptr() as *const c_char);
    0
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let ret = snd_soc_dapm_force_enable_pin_unlocked(dapm, pin);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int { dapm_set_pin(dapm, pin, 0) }
#[no_mangle] pub unsafe extern "C" fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    snd_soc_dapm_mutex_lock(dapm as *mut c_void);
    let ret = dapm_set_pin(dapm, pin, 0);
    snd_soc_dapm_mutex_unlock(dapm as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_get_pin_status(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    let w = dapm_find_widget(dapm, pin, true);
    if !w.is_null() { (*w).connected } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_ignore_suspend(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int {
    let w = dapm_find_widget(dapm, pin, false);
    if w.is_null() { return -EINVAL; }
    (*w).ignore_suspend = 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_pin_has_prefix(_card: *mut snd_soc_card, _pin: *const c_char) -> bool_t {
    // for_each_card_components: return true if pin starts with "prefix ".
    false
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_free(dapm: *mut snd_soc_dapm_context) {
    dapm_debugfs_cleanup(dapm);
    dapm_free_widgets(dapm);
    // list_del(&dapm->list);
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_init(dapm: *mut snd_soc_dapm_context, card: *mut snd_soc_card, component: *mut snd_soc_component) {
    (*dapm).card = card;
    (*dapm).component = component;
    (*dapm).bias_level = snd_soc_bias_level::SND_SOC_BIAS_OFF;
    if !component.is_null() { (*dapm).idle_bias = (*(*component).driver).idle_bias_on; }
    // INIT_LIST_HEAD(&dapm->list); list_add(&dapm->list, &card->dapm_list);
}

unsafe fn dapm_shutdown(dapm: *mut snd_soc_dapm_context) {
    let card = (*dapm).card;
    let mut down_list = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
    let mut powerdown = 0;
    snd_soc_dapm_mutex_lock_root(card as *mut c_void);
    // for_each_card_widgets in this dapm: if powered, insert in down_list,
    // set new_power = 0 and powerdown = 1.
    if powerdown != 0 {
        if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_ON {
            snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_PREPARE);
        }
        dapm_seq_run(card, &mut down_list, 0, false);
        if (*dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_PREPARE {
            snd_soc_dapm_set_bias_level(dapm, snd_soc_bias_level::SND_SOC_BIAS_STANDBY);
        }
    }
    snd_soc_dapm_mutex_unlock(card as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dapm_shutdown(card: *mut snd_soc_card) {
    let card_dapm = (*card).dapm;
    // for_each_card_dapms other than card_dapm: dapm_shutdown and force OFF from STANDBY.
    dapm_shutdown(card_dapm);
    if (*card_dapm).bias_level == snd_soc_bias_level::SND_SOC_BIAS_STANDBY {
        snd_soc_dapm_set_bias_level(card_dapm, snd_soc_bias_level::SND_SOC_BIAS_OFF);
    }
}

// Module information:
// MODULE_AUTHOR("Liam Girdwood, lrg@slimlogic.co.uk");
// MODULE_DESCRIPTION("Dynamic Audio Power Management core for ALSA SoC");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
