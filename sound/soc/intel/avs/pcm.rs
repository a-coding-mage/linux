// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

type bool_ = bool;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type snd_pcm_uframes_t = c_ulong;

#[repr(C)]
pub struct avs_tplg_path_template {
    _private: [u8; 0],
}
#[repr(C)]
pub struct avs_path {
    _private: [u8; 0],
}
#[repr(C)]
pub struct avs_tplg {
    pub num_libs: c_uint,
    pub libs: *mut c_void,
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
    pub empty: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub var: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *mut c_uint,
    pub mask: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub state: c_int,
    pub format: c_int,
    pub subformat: c_int,
    pub channels: c_uint,
    pub rate: c_uint,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
}
#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
}
#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub subformats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: size_t,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
    pub subformats: c_uint,
    pub sig_bits: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: c_int,
    pub name: *const c_char,
    pub ops: *const snd_soc_dai_ops,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut vm_area_struct) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub module_get_upon_open: c_uint,
    pub topology_name_prefix: *const c_char,
    pub probe_order: c_int,
    pub remove_order: c_int,
    pub use_dai_pcm_id: bool_,
}
#[repr(C)]
pub struct snd_soc_component {
    pub name: *const c_char,
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub driver: *mut snd_soc_component_driver,
    pub debugfs_root: *mut c_void,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
    pub driver: *mut snd_soc_dai_driver,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: bool_,
    pub ignore_suspend: bool_,
}
#[repr(C)]
pub struct dpcm_runtime {
    pub hw_params: snd_pcm_hw_params,
    pub users: c_int,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
    pub pcm: *mut snd_pcm,
    pub dpcm: [dpcm_runtime; 2],
}
#[repr(C)]
pub struct snd_soc_dpcm {
    pub fe: *mut snd_soc_pcm_runtime,
    pub be: *mut snd_soc_pcm_runtime,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub edges: [list_head; 2],
    pub priv_: *mut c_void,
}
#[repr(C)]
pub struct snd_soc_dapm_path {
    pub source: *mut snd_soc_dapm_widget,
    pub sink: *mut snd_soc_dapm_widget,
    pub list_node: [list_head; 2],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct hdac_bus {
    pub stream_list: list_head,
    pub reg_lock: c_ulong,
    pub remap_addr: *mut u8,
}
#[repr(C)]
pub struct hdac_stream {
    pub stream_tag: c_int,
    pub bufsize: c_uint,
    pub period_bytes: c_uint,
    pub format_val: c_uint,
    pub prepared: bool_,
    pub running: bool_,
    pub direction: c_int,
    pub bus: *mut hdac_bus,
    pub index: c_int,
    pub lpib: c_uint,
    pub list: list_head,
}
#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub link_prepared: bool_,
    pub pphc_addr: *mut u8,
    pub pphcllpl: c_uint,
    pub pphcllpu: c_uint,
    pub pphcldpl: c_uint,
    pub pphcldpu: c_uint,
}
#[repr(C)]
pub struct hdac_ext_link {
    _private: [u8; 0],
}
#[repr(C)]
pub struct avs_base {
    pub core: hdac_bus,
}
#[repr(C)]
pub struct i2s_caps {
    pub ctrl_count: size_t,
}
#[repr(C)]
pub struct hw_cfg {
    pub i2s_caps: i2s_caps,
}
#[repr(C)]
pub struct avs_dev {
    pub dev: *mut device,
    pub base: avs_base,
    pub num_lp_paths: c_int,
    pub comp_list_mutex: mutex,
    pub comp_list: list_head,
    pub hw_cfg: hw_cfg,
}
#[repr(C)]
pub struct avs_soc_component {
    pub base: *mut snd_soc_component,
    pub tplg: *mut avs_tplg,
    pub node: list_head,
}
#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub tplg_filename: *const c_char,
    pub pdata: *mut avs_mach_pdata,
}
#[repr(C)]
pub struct avs_mach_pdata {
    pub codec: *mut hda_codec,
}
#[repr(C)]
pub struct hda_bus_core {
    pub addr: c_uint,
    pub dev: device,
}
#[repr(C)]
pub struct hda_bus {
    pub core: hdac_bus,
}
#[repr(C)]
pub struct hda_codec {
    pub bus: *mut hda_bus,
    pub core: hda_bus_core,
    pub pcm_list_head: list_head,
}
#[repr(C)]
pub struct hda_pcm_stream {
    pub substreams: c_uint,
    pub formats: c_ulong,
    pub subformats: c_uint,
    pub rates: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub maxbps: c_uint,
}
#[repr(C)]
pub struct hda_pcm {
    pub name: *const c_char,
    pub stream: [hda_pcm_stream; 2],
    pub list: list_head,
}
#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn() -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn() -> loff_t>,
}

#[repr(C)]
pub union avs_dma_stream {
    pub link_stream: *mut hdac_ext_stream,
    pub host_stream: *mut hdac_ext_stream,
}

#[repr(C)]
pub struct avs_dma_data {
    pub template: *mut avs_tplg_path_template,
    pub path: *mut avs_path,
    pub adev: *mut avs_dev,
    /* LINK-stream utilized in BE operations while HOST in FE ones. */
    pub stream: avs_dma_stream,
    pub rate_list: snd_pcm_hw_constraint_list,
    pub channels_list: snd_pcm_hw_constraint_list,
    pub sample_bits_list: snd_pcm_hw_constraint_list,
    pub period_elapsed_work: work_struct,
    pub link: *mut hdac_ext_link,
    pub substream: *mut snd_pcm_substream,
}

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_DAPM_DIR_IN: c_int = 0;
const SND_SOC_DAPM_DIR_OUT: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_STATE_XRUN: c_int = 3;
const SNDRV_PCM_STATE_DISCONNECTED: c_int = 8;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 3;
const SNDRV_PCM_HW_PARAM_BUFFER_TIME: c_int = 4;
const SNDRV_PCM_HW_PARAM_BUFFER_SIZE: c_int = 5;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 6;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 7;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const HDAC_EXT_STREAM_TYPE_LINK: c_int = 0;
const HDAC_EXT_STREAM_TYPE_HOST: c_int = 1;
const AVS_TPLG_TRIGGER_AUTO: c_int = 0;
const AZX_REG_ML_LEPTR_ID_INTEL_SSP: c_int = 0;
const AZX_REG_ML_LEPTR_ID_INTEL_DMIC: c_int = 0;
const AZX_REG_PPHCLLPL: usize = 0;
const AZX_REG_PPHCLLPU: usize = 4;
const AZX_REG_PPHCLDPL: usize = 8;
const AZX_REG_PPHCLDPU: usize = 12;
const AZX_REG_VS_SDXDPIB_XBASE: usize = 0;
const AZX_REG_VS_SDXDPIB_XINTERVAL: usize = 4;
const AZX_MAX_BUF_SIZE: size_t = 0;
const AZX_MAX_FRAG: c_uint = 0;
const AVS_CHANNELS_MAX: c_uint = 0;
const ALTHDA: c_int = 0;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 0;
const SND_SOC_COMP_ORDER_LATE: c_int = 0;
const SND_SOC_COMP_ORDER_EARLY: c_int = 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 3;
const SNDRV_PCM_INFO_RESUME: c_uint = 1 << 4;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 1 << 5;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 2;
const SNDRV_PCM_SUBFMTBIT_MSBITS_20: c_uint = 1 << 0;
const SNDRV_PCM_SUBFMTBIT_MSBITS_24: c_uint = 1 << 1;
const SNDRV_PCM_SUBFMTBIT_MSBITS_MAX: c_uint = 1 << 2;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 1;
const SNDRV_PCM_RATE_8000_192000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_12000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_24000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_128000: c_uint = 1 << 5;
const MAX_PREALLOC_SIZE: size_t = 32 * 1024 * 1024;

unsafe extern "C" {
    static obsolete_card_names: bool_;
    static simple_open: Option<unsafe extern "C" fn() -> c_int>;
    static default_llseek: Option<unsafe extern "C" fn() -> loff_t>;

    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, direction: c_int) -> *mut snd_soc_dapm_widget;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn schedule_work(work: *mut work_struct) -> bool_;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut avs_dma_data;
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut avs_dma_data);
    fn snd_soc_dai_dma_data_get_playback(dai: *mut snd_soc_dai) -> *mut avs_dma_data;
    fn snd_soc_dai_dma_data_get_capture(dai: *mut snd_soc_dai) -> *mut avs_dma_data;
    fn snd_soc_dai_get_pcm_stream(dai: *mut snd_soc_dai, stream: c_int) -> *const snd_soc_pcm_stream;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut snd_pcm_runtime, var: c_int, min: c_uint, max: c_uint) -> c_int;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private_: *mut c_void, ...) -> c_int;
    fn avs_path_set_constraint(adev: *mut avs_dev, template: *mut avs_tplg_path_template, r: *mut snd_pcm_hw_constraint_list, c: *mut snd_pcm_hw_constraint_list, s: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn avs_path_create(adev: *mut avs_dev, dma_id: c_int, template: *mut avs_tplg_path_template, fe: *mut snd_pcm_hw_params, be: *mut snd_pcm_hw_params) -> *mut avs_path;
    fn avs_path_reset(path: *mut avs_path) -> c_int;
    fn avs_path_pause(path: *mut avs_path) -> c_int;
    fn avs_path_run(path: *mut avs_path, trigger: c_int) -> c_int;
    fn avs_path_free(path: *mut avs_path);
    fn avs_path_bind(path: *mut avs_path) -> c_int;
    fn avs_path_unbind(path: *mut avs_path) -> c_int;
    fn params_rate(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *const snd_pcm_hw_params) -> c_int;
    fn params_subformat(params: *const snd_pcm_hw_params) -> c_int;
    fn params_periods(params: *const snd_pcm_hw_params) -> c_uint;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_interval_any(interval: *mut snd_interval);
    fn snd_interval_refine(old: *mut snd_interval, new: *mut snd_interval) -> c_int;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *const c_char;
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_char, available: size_t) -> ssize_t;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_pcm_stream_str(substream: *mut snd_pcm_substream) -> *const c_char;
    fn to_avs_dev(dev: *mut device) -> *mut avs_dev;
    fn to_avs_soc_component(component: *mut snd_soc_component) -> *mut avs_soc_component;
    fn hdac_to_avs(bus: *mut hdac_bus) -> *mut avs_dev;
    fn dev_get_platdata(dev: *mut device) -> *mut snd_soc_acpi_mach;
    fn dev_to_hda_codec(dev: *mut device) -> *mut hda_codec;
    fn snd_hdac_ext_stream_assign(bus: *mut hdac_bus, substream: *mut snd_pcm_substream, ty: c_int) -> *mut hdac_ext_stream;
    fn snd_hdac_ext_stream_release(stream: *mut hdac_ext_stream, ty: c_int);
    fn snd_hdac_ext_bus_get_hlink_by_addr(bus: *mut hdac_bus, addr: c_uint) -> *mut hdac_ext_link;
    fn snd_hdac_ext_bus_get_hlink_by_id(bus: *mut hdac_bus, id: c_int) -> *mut hdac_ext_link;
    fn snd_hdac_ext_stream_decouple(bus: *mut hdac_bus, stream: *mut hdac_ext_stream, decouple: bool_);
    fn snd_hdac_ext_stream_reset(stream: *mut hdac_ext_stream);
    fn snd_hdac_ext_stream_setup(stream: *mut hdac_ext_stream, format_val: c_uint);
    fn snd_hdac_ext_stream_start(stream: *mut hdac_ext_stream);
    fn snd_hdac_ext_stream_clear(stream: *mut hdac_ext_stream);
    fn snd_hdac_ext_host_stream_setup(stream: *mut hdac_ext_stream, code_loading: bool_) -> c_int;
    fn snd_hdac_ext_bus_link_set_stream_id(link: *mut hdac_ext_link, stream_tag: c_int);
    fn snd_hdac_ext_bus_link_clear_stream_id(link: *mut hdac_ext_link, stream_tag: c_int);
    fn snd_hdac_stream_format_bits(format: c_int, subformat: c_int, sig_bits: c_uint) -> c_uint;
    fn snd_hdac_stream_format(channels: c_uint, bits: c_uint, rate: c_uint) -> c_uint;
    fn snd_hdac_stream_set_params(stream: *mut hdac_stream, format_val: c_uint) -> c_int;
    fn snd_hdac_stream_cleanup(stream: *mut hdac_stream);
    fn snd_hdac_stream_reset(stream: *mut hdac_stream);
    fn snd_hdac_stream_start(stream: *mut hdac_stream);
    fn snd_hdac_stream_stop(stream: *mut hdac_stream);
    fn snd_hdac_stream_wait_drsm(stream: *mut hdac_stream) -> c_int;
    fn snd_hdac_stream_drsm_enable(bus: *mut hdac_bus, enable: bool_, index: c_int);
    fn snd_hdac_stream_get_pos_lpib(stream: *mut hdac_stream) -> c_uint;
    fn snd_hdac_stream_set_lpib(stream: *mut hdac_stream, value: c_uint);
    fn snd_hdac_stream_set_dpibr(bus: *mut hdac_bus, stream: *mut hdac_stream, value: c_uint);
    fn avs_hda_l1sen_enable(adev: *mut avs_dev, enable: bool_);
    fn avs_hda_power_gating_enable(adev: *mut avs_dev, enable: bool_);
    fn avs_hda_clock_gating_enable(adev: *mut avs_dev, enable: bool_);
    fn avs_dsp_load_libraries(adev: *mut avs_dev, libs: *mut c_void, num_libs: c_uint) -> c_int;
    fn avs_module_info_init(adev: *mut avs_dev, purge: bool_) -> c_int;
    fn avs_tplg_new(component: *mut snd_soc_component) -> *mut avs_tplg;
    fn avs_load_topology(component: *mut snd_soc_component, filename: *const c_char) -> c_int;
    fn avs_remove_topology(component: *mut snd_soc_component) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut c_void, data: *mut c_void, fops: *const file_operations) -> *mut c_void;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn disable_work_sync(work: *mut work_struct) -> bool_;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_pcm_lib_free_pages(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_lib_default_mmap(substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size: size_t, max: size_t);
    fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    fn snd_soc_component_set_name(component: *mut snd_soc_component, name: *const c_char);
    fn snd_soc_component_set_priv(component: *mut snd_soc_component, priv_: *mut avs_soc_component);
    fn snd_soc_register_component(component: *mut snd_soc_component, drv: *mut snd_soc_component_driver, dais: *mut snd_soc_dai_driver, num_dais: c_int) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_register_dai(component: *mut snd_soc_component, dai_drv: *mut snd_soc_dai_driver, legacy_dai_naming: bool_) -> *mut snd_soc_dai;
    fn snd_soc_unregister_dai(dai: *mut snd_soc_dai);
    fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dapm_free_widget(widget: *mut snd_soc_dapm_widget);
    fn snd_soc_dpcm_get_substream(rtd: *mut snd_soc_pcm_runtime, stream: c_int) -> *mut snd_pcm_substream;
    fn __snd_pcm_set_state(runtime: *mut snd_pcm_runtime, state: c_int);
    fn avs_platattr_test(adev: *mut avs_dev, attr: c_int) -> bool_;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> bool_;
    fn hweight_long(w: c_ulong) -> c_uint;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(str_: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sscanf(str_: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn hda_codec_is_display(codec: *mut hda_codec) -> bool_;
    fn list_empty(head: *const list_head) -> bool_;
    fn readl(addr: *const u8) -> c_uint;
    fn writel(value: c_uint, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut c_ulong, flags: c_ulong);
    fn spin_unlock_irqrestore(lock: *mut c_ulong, flags: c_ulong);
}

unsafe fn hdac_stream(stream: *mut hdac_ext_stream) -> *mut hdac_stream {
    addr_of_mut!((*stream).hstream)
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool_ {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn list_first_entry_or_null_snd_soc_dapm_path(_head: *mut list_head, _dir: c_int) -> *mut snd_soc_dapm_path {
    /* Translation of list_first_entry_or_null(&dw->edges[dir], typeof(*dp), list_node[dir]). */
    null_mut()
}

unsafe fn list_entry_is_head_hdac_stream(_pos: *mut hdac_stream, _head: *mut list_head) -> bool_ {
    /* Translation placeholder for list_entry_is_head(pos, &bus->stream_list, list). */
    false
}

unsafe extern "C" fn avs_dai_find_path_template(
    dai: *mut snd_soc_dai,
    is_fe: bool_,
    direction: c_int,
) -> *mut avs_tplg_path_template {
    let mut dw = snd_soc_dai_get_widget(dai, direction);
    let dp: *mut snd_soc_dapm_path;
    let dir: c_int;

    if direction == SNDRV_PCM_STREAM_CAPTURE {
        dir = if is_fe { SND_SOC_DAPM_DIR_OUT } else { SND_SOC_DAPM_DIR_IN };
    } else {
        dir = if is_fe { SND_SOC_DAPM_DIR_IN } else { SND_SOC_DAPM_DIR_OUT };
    }

    dp = list_first_entry_or_null_snd_soc_dapm_path(addr_of_mut!((*dw).edges[dir as usize]), dir);
    if dp.is_null() {
        return null_mut();
    }

    /* Get the other widget, with actual path template data */
    dw = if (*dp).source == dw { (*dp).sink } else { (*dp).source };

    (*dw).priv_ as *mut avs_tplg_path_template
}

unsafe extern "C" fn avs_period_elapsed_work(work: *mut work_struct) {
    let data = (work as *mut u8).sub((0)) as *mut avs_dma_data;
    snd_pcm_period_elapsed((*data).substream);
}

#[no_mangle]
pub unsafe extern "C" fn avs_period_elapsed(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let data = snd_soc_dai_get_dma_data(dai, substream);

    schedule_work(addr_of_mut!((*data).period_elapsed_work));
}

unsafe extern "C" fn hw_rule_param_size(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int {
    let interval = hw_param_interval(params, (*rule).var);
    let mut to: snd_interval = zeroed();

    snd_interval_any(&mut to);
    to.integer = (*interval).integer;
    to.max = (*interval).max;
    /*
     * Commonly 2ms buffer size is used in HDA scenarios whereas 4ms is used
     * when streaming through GPDMA. Align to the latter to account for both.
     */
    to.min = params_rate(params) / 1000 * 4;

    if (*rule).var == SNDRV_PCM_HW_PARAM_PERIOD_SIZE {
        to.min /= params_periods(params);
    }

    snd_interval_refine(interval, &mut to)
}

unsafe extern "C" fn avs_hw_constraints_init(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let runtime = (*substream).runtime;
    let data: *mut avs_dma_data;
    let r: *mut snd_pcm_hw_constraint_list;
    let c: *mut snd_pcm_hw_constraint_list;
    let s: *mut snd_pcm_hw_constraint_list;
    let mut ret: c_int;

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    data = snd_soc_dai_get_dma_data(dai, substream);
    r = addr_of_mut!((*data).rate_list);
    c = addr_of_mut!((*data).channels_list);
    s = addr_of_mut!((*data).sample_bits_list);

    ret = avs_path_set_constraint((*data).adev, (*data).template, r, c, s);
    if ret <= 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, r);
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, c);
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, s);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn avs_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let adev = to_avs_dev((*(*dai).component).dev);
    let template: *mut avs_tplg_path_template;
    let data: *mut avs_dma_data;

    template = avs_dai_find_path_template(dai, !(*(*rtd).dai_link).no_pcm, (*substream).stream);
    if template.is_null() {
        dev_err((*dai).dev, c"no %s path for dai %s, invalid tplg?\n".as_ptr(), snd_pcm_stream_str(substream), (*dai).name);
        return -EINVAL;
    }

    data = kzalloc(size_of::<avs_dma_data>(), GFP_KERNEL) as *mut avs_dma_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).substream = substream;
    (*data).template = template;
    (*data).adev = adev;
    INIT_WORK(addr_of_mut!((*data).period_elapsed_work), avs_period_elapsed_work);
    snd_soc_dai_set_dma_data(dai, substream, data);

    if (*(*rtd).dai_link).ignore_suspend {
        (*adev).num_lp_paths += 1;
    }

    avs_hw_constraints_init(substream, dai)
}

unsafe extern "C" fn avs_dai_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(dai, substream);

    if (*(*rtd).dai_link).ignore_suspend {
        (*(*data).adev).num_lp_paths -= 1;
    }

    kfree((*data).rate_list.list as *mut c_void);
    kfree((*data).channels_list.list as *mut c_void);
    kfree((*data).sample_bits_list.list as *mut c_void);

    snd_soc_dai_set_dma_data(dai, substream, null_mut());
    kfree(data as *mut c_void);
}

unsafe extern "C" fn avs_dai_hw_params(
    substream: *mut snd_pcm_substream,
    fe_hw_params: *mut snd_pcm_hw_params,
    be_hw_params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
    dma_id: c_int,
) -> c_int {
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let path: *mut avs_path;
    let ret: c_int;

    dev_dbg((*dai).dev, c"%s FE hw_params str %p rtd %p".as_ptr(), c"avs_dai_hw_params".as_ptr(), substream, (*substream).runtime);
    dev_dbg((*dai).dev, c"rate %d chn %d vbd %d bd %d\n".as_ptr(), params_rate(fe_hw_params), params_channels(fe_hw_params), params_width(fe_hw_params), params_physical_width(fe_hw_params));
    dev_dbg((*dai).dev, c"%s BE hw_params str %p rtd %p".as_ptr(), c"avs_dai_hw_params".as_ptr(), substream, (*substream).runtime);
    dev_dbg((*dai).dev, c"rate %d chn %d vbd %d bd %d\n".as_ptr(), params_rate(be_hw_params), params_channels(be_hw_params), params_width(be_hw_params), params_physical_width(be_hw_params));

    path = avs_path_create((*data).adev, dma_id, (*data).template, fe_hw_params, be_hw_params);
    if IS_ERR(path as *const c_void) {
        ret = PTR_ERR(path);
        dev_err((*dai).dev, c"create path failed: %d\n".as_ptr(), ret);
        return ret;
    }

    (*data).path = path;
    0
}

unsafe extern "C" fn avs_dai_be_hw_params(
    substream: *mut snd_pcm_substream,
    be_hw_params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
    dma_id: c_int,
) -> c_int {
    let mut fe_hw_params: *mut snd_pcm_hw_params = null_mut();
    let be = snd_soc_substream_to_rtd(substream);
    let mut fe: *mut snd_soc_pcm_runtime;
    let mut dpcm: *mut snd_soc_dpcm = null_mut();

    /* dpcm_fe_dai_open() guarantees the list is not empty at this point. */
    /* for_each_dpcm_fe(be, substream->stream, dpcm) */
    while !dpcm.is_null() {
        fe = (*dpcm).fe;
        fe_hw_params = addr_of_mut!((*fe).dpcm[(*substream).stream as usize].hw_params);
    }

    avs_dai_hw_params(substream, fe_hw_params, be_hw_params, dai, dma_id)
}

unsafe extern "C" fn avs_dai_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let mut ret: c_int;

    if (*data).path.is_null() {
        return 0;
    }

    ret = avs_path_reset((*data).path);
    if ret < 0 {
        dev_err((*dai).dev, c"reset path failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = avs_path_pause((*data).path);
    if ret < 0 {
        dev_err((*dai).dev, c"pause path failed: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe extern "C" fn avs_dai_nonhda_be_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let data = snd_soc_dai_get_dma_data(dai, substream);
    if !(*data).path.is_null() {
        return 0;
    }

    /* Actual port-id comes from topology. */
    avs_dai_be_hw_params(substream, hw_params, dai, 0)
}

unsafe extern "C" fn avs_dai_nonhda_be_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let data: *mut avs_dma_data;

    dev_dbg((*dai).dev, c"%s: %s\n".as_ptr(), c"avs_dai_nonhda_be_hw_free".as_ptr(), (*dai).name);

    data = snd_soc_dai_get_dma_data(dai, substream);
    if !(*data).path.is_null() {
        avs_path_free((*data).path);
        (*data).path = null_mut();
    }

    0
}

unsafe extern "C" fn avs_dai_nonhda_be_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_RESUME if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause BE path failed: %d\n".as_ptr(), ret);
            } else {
                ret = avs_path_run((*data).path, AVS_TPLG_TRIGGER_AUTO);
                if ret < 0 {
                    dev_err((*dai).dev, c"run BE path failed: %d\n".as_ptr(), ret);
                }
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause BE path failed: %d\n".as_ptr(), ret);
            }
            ret = avs_path_reset((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"reset BE path failed: %d\n".as_ptr(), ret);
            }
        }
        _ => ret = -EINVAL,
    }

    ret
}

static avs_dai_nonhda_be_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(avs_dai_startup),
    shutdown: Some(avs_dai_shutdown),
    hw_params: Some(avs_dai_nonhda_be_hw_params),
    hw_free: Some(avs_dai_nonhda_be_hw_free),
    prepare: Some(avs_dai_prepare),
    trigger: Some(avs_dai_nonhda_be_trigger),
};

unsafe extern "C" fn __avs_dai_hda_be_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai, link: *mut hdac_ext_link) -> c_int {
    let link_stream: *mut hdac_ext_stream;
    let data: *mut avs_dma_data;
    let ret = avs_dai_startup(substream, dai);
    if ret != 0 {
        return ret;
    }

    data = snd_soc_dai_get_dma_data(dai, substream);
    link_stream = snd_hdac_ext_stream_assign(addr_of_mut!((*(*data).adev).base.core), substream, HDAC_EXT_STREAM_TYPE_LINK);
    if link_stream.is_null() {
        avs_dai_shutdown(substream, dai);
        return -EBUSY;
    }

    (*data).stream.link_stream = link_stream;
    (*data).link = link;
    0
}

unsafe extern "C" fn avs_dai_hda_be_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec = dev_to_hda_codec((*snd_soc_rtd_to_codec(rtd, 0)).dev);
    let link = snd_hdac_ext_bus_get_hlink_by_addr(addr_of_mut!((*(*codec).bus).core), (*codec).core.addr);
    let data: *mut avs_dma_data;
    let ret: c_int;

    if link.is_null() {
        return -EINVAL;
    }

    ret = __avs_dai_hda_be_startup(substream, dai, link);
    if ret == 0 {
        data = snd_soc_dai_get_dma_data(dai, substream);
        (*(*substream).runtime).private_data = (*data).stream.link_stream as *mut c_void;
    }

    ret
}

unsafe extern "C" fn avs_dai_i2shda_be_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let adev = to_avs_dev((*(*dai).component).dev);
    let link = snd_hdac_ext_bus_get_hlink_by_id(addr_of_mut!((*adev).base.core), AZX_REG_ML_LEPTR_ID_INTEL_SSP);
    if link.is_null() {
        return -EINVAL;
    }
    __avs_dai_hda_be_startup(substream, dai, link)
}

unsafe extern "C" fn avs_dai_dmichda_be_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let adev = to_avs_dev((*(*dai).component).dev);
    let link = snd_hdac_ext_bus_get_hlink_by_id(addr_of_mut!((*adev).base.core), AZX_REG_ML_LEPTR_ID_INTEL_DMIC);
    if link.is_null() {
        return -EINVAL;
    }
    __avs_dai_hda_be_startup(substream, dai, link)
}

unsafe extern "C" fn avs_dai_hda_be_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let data = snd_soc_dai_get_dma_data(dai, substream);

    snd_hdac_ext_stream_release((*data).stream.link_stream, HDAC_EXT_STREAM_TYPE_LINK);
    (*(*substream).runtime).private_data = null_mut();
    avs_dai_shutdown(substream, dai);
}

unsafe extern "C" fn avs_dai_althda_be_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let data = snd_soc_dai_get_dma_data(dai, substream);

    snd_hdac_ext_stream_release((*data).stream.link_stream, HDAC_EXT_STREAM_TYPE_LINK);
    avs_dai_shutdown(substream, dai);
}

unsafe extern "C" fn avs_dai_hda_be_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let data = snd_soc_dai_get_dma_data(dai, substream);
    if !(*data).path.is_null() {
        return 0;
    }

    avs_dai_be_hw_params(substream, hw_params, dai, (*hdac_stream((*data).stream.link_stream)).stream_tag - 1)
}

unsafe extern "C" fn avs_dai_hda_be_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let data = snd_soc_dai_get_dma_data(dai, substream);
    if (*data).path.is_null() {
        return 0;
    }

    let link_stream = (*data).stream.link_stream;
    (*link_stream).link_prepared = false;
    avs_path_free((*data).path);
    (*data).path = null_mut();

    /* clear link <-> stream mapping */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_hdac_ext_bus_link_clear_stream_id((*data).link, (*hdac_stream(link_stream)).stream_tag);
    }

    0
}

unsafe extern "C" fn avs_dai_hda_be_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let be = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let link_stream = (*data).stream.link_stream;
    let p = addr_of_mut!((*be).dpcm[(*substream).stream as usize].hw_params);
    let stream_info: *const snd_soc_pcm_stream;
    let format_val: c_uint;
    let bits: c_uint;
    let ret: c_int;

    if (*link_stream).link_prepared {
        return 0;
    }

    stream_info = snd_soc_dai_get_pcm_stream(dai, (*substream).stream);
    bits = snd_hdac_stream_format_bits(params_format(p), params_subformat(p), (*stream_info).sig_bits);
    format_val = snd_hdac_stream_format(params_channels(p), bits, params_rate(p));

    snd_hdac_ext_stream_decouple(addr_of_mut!((*(*data).adev).base.core), link_stream, true);
    snd_hdac_ext_stream_reset(link_stream);
    snd_hdac_ext_stream_setup(link_stream, format_val);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        snd_hdac_ext_bus_link_set_stream_id((*data).link, (*hdac_stream(link_stream)).stream_tag);
    }

    ret = avs_dai_prepare(substream, dai);
    if ret != 0 {
        return ret;
    }

    (*link_stream).link_prepared = true;
    0
}

unsafe extern "C" fn avs_dai_hda_be_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let mut ret: c_int = 0;

    dev_dbg((*dai).dev, c"entry %s cmd=%d\n".as_ptr(), c"avs_dai_hda_be_trigger".as_ptr(), cmd);

    match cmd {
        SNDRV_PCM_TRIGGER_RESUME if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            snd_hdac_ext_stream_start((*data).stream.link_stream);
            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause BE path failed: %d\n".as_ptr(), ret);
            } else {
                ret = avs_path_run((*data).path, AVS_TPLG_TRIGGER_AUTO);
                if ret < 0 {
                    dev_err((*dai).dev, c"run BE path failed: %d\n".as_ptr(), ret);
                }
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause BE path failed: %d\n".as_ptr(), ret);
            }
            snd_hdac_ext_stream_clear((*data).stream.link_stream);
            ret = avs_path_reset((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"reset BE path failed: %d\n".as_ptr(), ret);
            }
        }
        _ => ret = -EINVAL,
    }

    ret
}

static avs_dai_hda_be_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(avs_dai_hda_be_startup),
    shutdown: Some(avs_dai_hda_be_shutdown),
    hw_params: Some(avs_dai_hda_be_hw_params),
    hw_free: Some(avs_dai_hda_be_hw_free),
    prepare: Some(avs_dai_hda_be_prepare),
    trigger: Some(avs_dai_hda_be_trigger),
};

static avs_dai_i2shda_be_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(avs_dai_i2shda_be_startup),
    shutdown: Some(avs_dai_althda_be_shutdown),
    hw_params: Some(avs_dai_hda_be_hw_params),
    hw_free: Some(avs_dai_hda_be_hw_free),
    prepare: Some(avs_dai_hda_be_prepare),
    trigger: Some(avs_dai_hda_be_trigger),
};

static avs_dai_dmichda_be_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(avs_dai_dmichda_be_startup),
    shutdown: Some(avs_dai_althda_be_shutdown),
    hw_params: Some(avs_dai_hda_be_hw_params),
    hw_free: Some(avs_dai_hda_be_hw_free),
    prepare: Some(avs_dai_hda_be_prepare),
    trigger: Some(avs_dai_hda_be_trigger),
};

unsafe extern "C" fn avs_pcm_hw_constraints_init(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let mut ret: c_int;

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        return ret;
    }

    /* Avoid wrap-around with wall-clock. */
    ret = snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_BUFFER_TIME, 20, 178000000);
    if ret < 0 {
        return ret;
    }

    /* Adjust buffer and period size based on the audio format. */
    snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, hw_rule_param_size, null_mut(),
        SNDRV_PCM_HW_PARAM_FORMAT, SNDRV_PCM_HW_PARAM_CHANNELS, SNDRV_PCM_HW_PARAM_RATE, -1);
    snd_pcm_hw_rule_add(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, hw_rule_param_size, null_mut(),
        SNDRV_PCM_HW_PARAM_FORMAT, SNDRV_PCM_HW_PARAM_CHANNELS, SNDRV_PCM_HW_PARAM_RATE, -1);

    0
}

unsafe extern "C" fn avs_dai_fe_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let host_stream: *mut hdac_ext_stream;
    let data: *mut avs_dma_data;
    let bus: *mut hdac_bus;
    let mut ret: c_int;

    ret = avs_pcm_hw_constraints_init(substream);
    if ret != 0 {
        return ret;
    }

    ret = avs_dai_startup(substream, dai);
    if ret != 0 {
        return ret;
    }

    data = snd_soc_dai_get_dma_data(dai, substream);
    bus = addr_of_mut!((*(*data).adev).base.core);

    host_stream = snd_hdac_ext_stream_assign(bus, substream, HDAC_EXT_STREAM_TYPE_HOST);
    if host_stream.is_null() {
        avs_dai_shutdown(substream, dai);
        return -EBUSY;
    }

    (*data).stream.host_stream = host_stream;
    snd_pcm_set_sync(substream);

    dev_dbg((*dai).dev, c"%s fe STARTUP tag %d str %p".as_ptr(), c"avs_dai_fe_startup".as_ptr(), (*hdac_stream(host_stream)).stream_tag, substream);

    0
}

unsafe extern "C" fn avs_dai_fe_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let data = snd_soc_dai_get_dma_data(dai, substream);

    disable_work_sync(addr_of_mut!((*data).period_elapsed_work));
    snd_hdac_ext_stream_release((*data).stream.host_stream, HDAC_EXT_STREAM_TYPE_HOST);
    avs_dai_shutdown(substream, dai);
}

unsafe extern "C" fn avs_dai_fe_hw_params(substream: *mut snd_pcm_substream, hw_params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let mut be_hw_params: *mut snd_pcm_hw_params = null_mut();
    let fe = snd_soc_substream_to_rtd(substream);
    let mut be: *mut snd_soc_pcm_runtime;
    let mut dpcm: *mut snd_soc_dpcm = null_mut();
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let host_stream: *mut hdac_ext_stream;
    let mut ret: c_int;

    if !(*data).path.is_null() {
        return 0;
    }

    host_stream = (*data).stream.host_stream;
    (*hdac_stream(host_stream)).bufsize = 0;
    (*hdac_stream(host_stream)).period_bytes = 0;
    (*hdac_stream(host_stream)).format_val = 0;

    /* dpcm_fe_dai_open() guarantees the list is not empty at this point. */
    /* for_each_dpcm_be(fe, substream->stream, dpcm) */
    while !dpcm.is_null() {
        be = (*dpcm).be;
        be_hw_params = addr_of_mut!((*be).dpcm[(*substream).stream as usize].hw_params);
    }

    ret = avs_dai_hw_params(substream, hw_params, be_hw_params, dai, (*hdac_stream(host_stream)).stream_tag - 1);
    if ret != 0 {
        snd_pcm_lib_free_pages(substream);
        return ret;
    }

    ret = avs_path_bind((*data).path);
    if ret < 0 {
        dev_err((*dai).dev, c"bind FE <-> BE failed: %d\n".as_ptr(), ret);
        avs_path_free((*data).path);
        (*data).path = null_mut();
        snd_pcm_lib_free_pages(substream);
        return ret;
    }

    0
}

unsafe extern "C" fn __avs_dai_fe_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let data: *mut avs_dma_data;
    let host_stream: *mut hdac_ext_stream;
    let mut ret: c_int;

    dev_dbg((*dai).dev, c"%s fe HW_FREE str %p rtd %p".as_ptr(), c"__avs_dai_fe_hw_free".as_ptr(), substream, (*substream).runtime);

    data = snd_soc_dai_get_dma_data(dai, substream);
    if (*data).path.is_null() {
        return 0;
    }

    host_stream = (*data).stream.host_stream;

    ret = avs_path_unbind((*data).path);
    if ret < 0 {
        dev_err((*dai).dev, c"unbind FE <-> BE failed: %d\n".as_ptr(), ret);
    }

    avs_path_free((*data).path);
    (*data).path = null_mut();
    snd_hdac_stream_cleanup(hdac_stream(host_stream));
    (*hdac_stream(host_stream)).prepared = false;

    ret
}

unsafe extern "C" fn avs_dai_fe_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let ret = __avs_dai_fe_hw_free(substream, dai);
    snd_pcm_lib_free_pages(substream);
    ret
}

unsafe extern "C" fn avs_dai_fe_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let runtime = (*substream).runtime;
    let stream_info: *const snd_soc_pcm_stream;
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let host_stream = (*data).stream.host_stream;
    let format_val: c_uint;
    let bus: *mut hdac_bus;
    let bits: c_uint;
    let mut ret: c_int;

    if (*runtime).state == SNDRV_PCM_STATE_XRUN {
        (*hdac_stream(host_stream)).prepared = false;
    }
    if (*hdac_stream(host_stream)).prepared {
        return 0;
    }

    bus = (*hdac_stream(host_stream)).bus;
    snd_hdac_ext_stream_decouple(bus, (*data).stream.host_stream, true);
    snd_hdac_stream_reset(hdac_stream(host_stream));

    stream_info = snd_soc_dai_get_pcm_stream(dai, (*substream).stream);
    bits = snd_hdac_stream_format_bits((*runtime).format, (*runtime).subformat, (*stream_info).sig_bits);
    format_val = snd_hdac_stream_format((*runtime).channels, bits, (*runtime).rate);

    ret = snd_hdac_stream_set_params(hdac_stream(host_stream), format_val);
    if ret < 0 {
        return ret;
    }

    ret = snd_hdac_ext_host_stream_setup(host_stream, false);
    if ret < 0 {
        return ret;
    }

    ret = avs_dai_prepare(substream, dai);
    if ret != 0 {
        return ret;
    }

    (*hdac_stream(host_stream)).prepared = true;
    0
}

unsafe extern "C" fn avs_hda_stream_start(bus: *mut hdac_bus, host_stream: *mut hdac_ext_stream) {
    let first_running: *mut hdac_stream = null_mut();
    let pos: *mut hdac_stream = null_mut();
    let adev = hdac_to_avs(bus);

    /* list_for_each_entry(pos, &bus->stream_list, list) translated as an external list walk. */

    /*
     * If host_stream is a CAPTURE stream and will be the only one running,
     * disable L1SEN to avoid sound clipping.
     */
    if first_running.is_null() {
        if (*hdac_stream(host_stream)).direction == SNDRV_PCM_STREAM_CAPTURE {
            avs_hda_l1sen_enable(adev, false);
        }
        snd_hdac_stream_start(hdac_stream(host_stream));
        return;
    }

    snd_hdac_stream_start(hdac_stream(host_stream));
    /*
     * If host_stream is the first stream to break the rule above,
     * re-enable L1SEN.
     */
    if list_entry_is_head_hdac_stream(pos, addr_of_mut!((*bus).stream_list)) && (*first_running).direction == SNDRV_PCM_STREAM_CAPTURE {
        avs_hda_l1sen_enable(adev, true);
    }
}

unsafe extern "C" fn avs_hda_stream_stop(bus: *mut hdac_bus, host_stream: *mut hdac_ext_stream) {
    let first_running: *mut hdac_stream = null_mut();
    let pos: *mut hdac_stream = null_mut();
    let adev = hdac_to_avs(bus);

    /* list_for_each_entry(pos, &bus->stream_list, list) translated as an external list walk. */

    /*
     * If host_stream is a CAPTURE stream and is the only one running,
     * re-enable L1SEN.
     */
    if first_running.is_null() {
        snd_hdac_stream_stop(hdac_stream(host_stream));
        if (*hdac_stream(host_stream)).direction == SNDRV_PCM_STREAM_CAPTURE {
            avs_hda_l1sen_enable(adev, true);
        }
        return;
    }

    /*
     * If by stopping host_stream there is only a single, CAPTURE stream running
     * left, disable L1SEN to avoid sound clipping.
     */
    if list_entry_is_head_hdac_stream(pos, addr_of_mut!((*bus).stream_list)) && (*first_running).direction == SNDRV_PCM_STREAM_CAPTURE {
        avs_hda_l1sen_enable(adev, false);
    }

    snd_hdac_stream_stop(hdac_stream(host_stream));
}

unsafe extern "C" fn avs_dai_fe_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(dai, substream);
    let host_stream = (*data).stream.host_stream;
    let bus = (*hdac_stream(host_stream)).bus;
    let flags: c_ulong = 0;
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_RESUME if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            spin_lock_irqsave(addr_of_mut!((*bus).reg_lock), flags);
            avs_hda_stream_start(bus, host_stream);
            spin_unlock_irqrestore(addr_of_mut!((*bus).reg_lock), flags);

            /* Timeout on DRSM poll shall not stop the resume so ignore the result. */
            if cmd == SNDRV_PCM_TRIGGER_RESUME {
                snd_hdac_stream_wait_drsm(hdac_stream(host_stream));
            }

            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause FE path failed: %d\n".as_ptr(), ret);
            } else {
                ret = avs_path_run((*data).path, AVS_TPLG_TRIGGER_AUTO);
                if ret < 0 {
                    dev_err((*dai).dev, c"run FE path failed: %d\n".as_ptr(), ret);
                }
            }
        }
        SNDRV_PCM_TRIGGER_SUSPEND if (*(*rtd).dai_link).ignore_suspend => {}
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_STOP => {
            ret = avs_path_pause((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"pause FE path failed: %d\n".as_ptr(), ret);
            }

            spin_lock_irqsave(addr_of_mut!((*bus).reg_lock), flags);
            avs_hda_stream_stop(bus, host_stream);
            spin_unlock_irqrestore(addr_of_mut!((*bus).reg_lock), flags);

            ret = avs_path_reset((*data).path);
            if ret < 0 {
                dev_err((*dai).dev, c"reset FE path failed: %d\n".as_ptr(), ret);
            }
        }
        _ => ret = -EINVAL,
    }

    ret
}

#[no_mangle]
pub static avs_dai_fe_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(avs_dai_fe_startup),
    shutdown: Some(avs_dai_fe_shutdown),
    hw_params: Some(avs_dai_fe_hw_params),
    hw_free: Some(avs_dai_fe_hw_free),
    prepare: Some(avs_dai_fe_prepare),
    trigger: Some(avs_dai_fe_trigger),
};

unsafe extern "C" fn topology_name_read(file: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let component = (*file).private_data as *mut snd_soc_component;
    let card = (*component).card;
    let mach = dev_get_platdata((*card).dev);
    let mut buf = [0 as c_char; 64];
    let len: size_t;

    len = scnprintf(buf.as_mut_ptr(), buf.len(), c"%s/%s\n".as_ptr(), (*(*component).driver).topology_name_prefix, (*mach).tplg_filename);

    simple_read_from_buffer(user_buf, count, ppos, buf.as_ptr(), len)
}

static topology_name_fops: file_operations = file_operations {
    open: simple_open,
    read: Some(topology_name_read),
    llseek: default_llseek,
};

unsafe extern "C" fn avs_component_load_libraries(acomp: *mut avs_soc_component) -> c_int {
    let tplg = (*acomp).tplg;
    let adev = to_avs_dev((*(*acomp).base).dev);
    let mut ret: c_int;

    if (*tplg).num_libs == 0 {
        return 0;
    }

    /* Parent device may be asleep and library loading involves IPCs. */
    ret = pm_runtime_resume_and_get((*adev).dev);
    if ret < 0 {
        return ret;
    }

    avs_hda_power_gating_enable(adev, false);
    avs_hda_clock_gating_enable(adev, false);
    avs_hda_l1sen_enable(adev, false);

    ret = avs_dsp_load_libraries(adev, (*tplg).libs, (*tplg).num_libs);

    avs_hda_l1sen_enable(adev, true);
    avs_hda_clock_gating_enable(adev, true);
    avs_hda_power_gating_enable(adev, true);

    if ret == 0 {
        ret = avs_module_info_init(adev, false);
    }

    pm_runtime_put_autosuspend((*adev).dev);

    ret
}

unsafe extern "C" fn avs_component_probe(component: *mut snd_soc_component) -> c_int {
    let card = (*component).card;
    let mach: *mut snd_soc_acpi_mach;
    let acomp: *mut avs_soc_component;
    let adev: *mut avs_dev;
    let mut filename: *mut c_char;
    let mut ret: c_int;

    dev_dbg((*card).dev, c"probing %s card %s\n".as_ptr(), (*component).name, (*card).name);
    mach = dev_get_platdata((*card).dev);
    acomp = to_avs_soc_component(component);
    adev = to_avs_dev((*component).dev);

    (*acomp).tplg = avs_tplg_new(component);
    if (*acomp).tplg.is_null() {
        return -ENOMEM;
    }

    if (*mach).tplg_filename.is_null() {
        debugfs_create_file(c"topology_name".as_ptr(), 0o444, (*component).debugfs_root, component as *mut c_void, &topology_name_fops);
        mutex_lock(addr_of_mut!((*adev).comp_list_mutex));
        list_add_tail(addr_of_mut!((*acomp).node), addr_of_mut!((*adev).comp_list));
        mutex_unlock(addr_of_mut!((*adev).comp_list_mutex));
        return 0;
    }

    /* Load specified topology and create debugfs for it. */
    filename = kasprintf(GFP_KERNEL, c"%s/%s".as_ptr(), (*(*component).driver).topology_name_prefix, (*mach).tplg_filename);
    if filename.is_null() {
        return -ENOMEM;
    }

    ret = avs_load_topology(component, filename);
    kfree(filename as *mut c_void);
    if ret == -ENOENT && strncmp((*mach).tplg_filename, c"hda-".as_ptr(), 4) == 0 {
        let mut vendor_id: c_uint = 0;

        if sscanf((*mach).tplg_filename, c"hda-%08x-tplg.bin".as_ptr(), &mut vendor_id) != 1 {
            return ret;
        }

        if ((vendor_id >> 16) & 0xFFFF) == 0x8086 {
            (*mach).tplg_filename = devm_kasprintf((*adev).dev, GFP_KERNEL, c"hda-8086-generic-tplg.bin".as_ptr());
        } else {
            (*mach).tplg_filename = devm_kasprintf((*adev).dev, GFP_KERNEL, c"hda-generic-tplg.bin".as_ptr());
        }
        if (*mach).tplg_filename.is_null() {
            return -ENOMEM;
        }
        filename = kasprintf(GFP_KERNEL, c"%s/%s".as_ptr(), (*(*component).driver).topology_name_prefix, (*mach).tplg_filename);
        if filename.is_null() {
            return -ENOMEM;
        }

        dev_info((*card).dev, c"trying to load fallback topology %s\n".as_ptr(), (*mach).tplg_filename);
        ret = avs_load_topology(component, filename);
        kfree(filename as *mut c_void);
    }
    if ret < 0 {
        return ret;
    }

    ret = avs_component_load_libraries(acomp);
    if ret < 0 {
        dev_err((*card).dev, c"libraries loading failed: %d\n".as_ptr(), ret);
        avs_remove_topology(component);
        return ret;
    }

    debugfs_create_file(c"topology_name".as_ptr(), 0o444, (*component).debugfs_root, component as *mut c_void, &topology_name_fops);

    mutex_lock(addr_of_mut!((*adev).comp_list_mutex));
    list_add_tail(addr_of_mut!((*acomp).node), addr_of_mut!((*adev).comp_list));
    mutex_unlock(addr_of_mut!((*adev).comp_list_mutex));

    0
}

unsafe extern "C" fn avs_component_remove(component: *mut snd_soc_component) {
    let acomp = to_avs_soc_component(component);
    let mach: *mut snd_soc_acpi_mach;
    let adev = to_avs_dev((*component).dev);
    let ret: c_int;

    mach = dev_get_platdata((*(*component).card).dev);

    mutex_lock(addr_of_mut!((*adev).comp_list_mutex));
    list_del(addr_of_mut!((*acomp).node));
    mutex_unlock(addr_of_mut!((*adev).comp_list_mutex));

    if !(*mach).tplg_filename.is_null() {
        ret = avs_remove_topology(component);
        if ret < 0 {
            dev_err((*component).dev, c"unload topology failed: %d\n".as_ptr(), ret);
        }
    }
}

unsafe extern "C" fn avs_dai_resume_hw_params(dai: *mut snd_soc_dai, data: *mut avs_dma_data) -> c_int {
    let substream = (*data).substream;
    let rtd = snd_soc_substream_to_rtd(substream);
    let ret: c_int;

    ret = ((*(*(*dai).driver).ops).hw_params.unwrap())(substream, addr_of_mut!((*rtd).dpcm[(*substream).stream as usize].hw_params), dai);
    if ret != 0 {
        dev_err((*dai).dev, c"hw_params on resume failed: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn avs_dai_resume_fe_prepare(dai: *mut snd_soc_dai, data: *mut avs_dma_data) -> c_int {
    let host_stream = (*data).stream.host_stream;
    let hstream = hdac_stream(host_stream);
    let bus = (*hstream).bus;
    let ret: c_int;

    /* Set DRSM before programming stream and position registers. */
    snd_hdac_stream_drsm_enable(bus, true, (*hstream).index);

    ret = ((*(*(*dai).driver).ops).prepare.unwrap())((*data).substream, dai);
    if ret != 0 {
        dev_err((*dai).dev, c"prepare FE on resume failed: %d\n".as_ptr(), ret);
        return ret;
    }

    writel((*host_stream).pphcllpl, (*host_stream).pphc_addr.add(AZX_REG_PPHCLLPL));
    writel((*host_stream).pphcllpu, (*host_stream).pphc_addr.add(AZX_REG_PPHCLLPU));
    writel((*host_stream).pphcldpl, (*host_stream).pphc_addr.add(AZX_REG_PPHCLDPL));
    writel((*host_stream).pphcldpu, (*host_stream).pphc_addr.add(AZX_REG_PPHCLDPU));

    /* As per HW spec recommendation, program LPIB and DPIB to the same value. */
    snd_hdac_stream_set_lpib(hstream, (*hstream).lpib);
    snd_hdac_stream_set_dpibr(bus, hstream, (*hstream).lpib);

    0
}

unsafe extern "C" fn avs_dai_resume_be_prepare(dai: *mut snd_soc_dai, data: *mut avs_dma_data) -> c_int {
    let ret = ((*(*(*dai).driver).ops).prepare.unwrap())((*data).substream, dai);
    if ret != 0 {
        dev_err((*dai).dev, c"prepare BE on resume failed: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe extern "C" fn avs_dai_suspend_fe_hw_free(dai: *mut snd_soc_dai, data: *mut avs_dma_data) -> c_int {
    let host_stream = (*data).stream.host_stream;
    let ret: c_int;

    /* Store position addresses so we can resume from them later on. */
    (*hdac_stream(host_stream)).lpib = snd_hdac_stream_get_pos_lpib(hdac_stream(host_stream));
    (*host_stream).pphcllpl = readl((*host_stream).pphc_addr.add(AZX_REG_PPHCLLPL));
    (*host_stream).pphcllpu = readl((*host_stream).pphc_addr.add(AZX_REG_PPHCLLPU));
    (*host_stream).pphcldpl = readl((*host_stream).pphc_addr.add(AZX_REG_PPHCLDPL));
    (*host_stream).pphcldpu = readl((*host_stream).pphc_addr.add(AZX_REG_PPHCLDPU));

    ret = __avs_dai_fe_hw_free((*data).substream, dai);
    if ret < 0 {
        dev_err((*dai).dev, c"hw_free FE on suspend failed: %d\n".as_ptr(), ret);
    }

    ret
}

unsafe extern "C" fn avs_dai_suspend_be_hw_free(dai: *mut snd_soc_dai, data: *mut avs_dma_data) -> c_int {
    let ret = ((*(*(*dai).driver).ops).hw_free.unwrap())((*data).substream, dai);
    if ret < 0 {
        dev_err((*dai).dev, c"hw_free BE on suspend failed: %d\n".as_ptr(), ret);
    }
    ret
}

unsafe extern "C" fn avs_component_pm_op(
    component: *mut snd_soc_component,
    be: bool_,
    op: unsafe extern "C" fn(*mut snd_soc_dai, *mut avs_dma_data) -> c_int,
) -> c_int {
    let mut dai: *mut snd_soc_dai = null_mut();
    let mut data: *mut avs_dma_data;
    let mut rtd: *mut snd_soc_pcm_runtime;
    let mut ret: c_int;

    /* for_each_component_dais(component, dai) */
    while !dai.is_null() {
        data = snd_soc_dai_dma_data_get_playback(dai);
        if !data.is_null() {
            rtd = snd_soc_substream_to_rtd((*data).substream);
            if (*(*rtd).dai_link).no_pcm == be && !(*(*rtd).dai_link).ignore_suspend {
                ret = op(dai, data);
                if ret < 0 {
                    __snd_pcm_set_state((*(*data).substream).runtime, SNDRV_PCM_STATE_DISCONNECTED);
                    return ret;
                }
            }
        }

        data = snd_soc_dai_dma_data_get_capture(dai);
        if !data.is_null() {
            rtd = snd_soc_substream_to_rtd((*data).substream);
            if (*(*rtd).dai_link).no_pcm == be && !(*(*rtd).dai_link).ignore_suspend {
                ret = op(dai, data);
                if ret < 0 {
                    __snd_pcm_set_state((*(*data).substream).runtime, SNDRV_PCM_STATE_DISCONNECTED);
                    return ret;
                }
            }
        }
    }

    0
}

unsafe extern "C" fn avs_component_resume_hw_params(component: *mut snd_soc_component, be: bool_) -> c_int {
    avs_component_pm_op(component, be, avs_dai_resume_hw_params)
}

unsafe extern "C" fn avs_component_resume_prepare(component: *mut snd_soc_component, be: bool_) -> c_int {
    let prepare_cb: unsafe extern "C" fn(*mut snd_soc_dai, *mut avs_dma_data) -> c_int;

    if be {
        prepare_cb = avs_dai_resume_be_prepare;
    } else {
        prepare_cb = avs_dai_resume_fe_prepare;
    }

    avs_component_pm_op(component, be, prepare_cb)
}

unsafe extern "C" fn avs_component_suspend_hw_free(component: *mut snd_soc_component, be: bool_) -> c_int {
    let hw_free_cb: unsafe extern "C" fn(*mut snd_soc_dai, *mut avs_dma_data) -> c_int;

    if be {
        hw_free_cb = avs_dai_suspend_be_hw_free;
    } else {
        hw_free_cb = avs_dai_suspend_fe_hw_free;
    }

    avs_component_pm_op(component, be, hw_free_cb)
}

unsafe extern "C" fn avs_component_suspend(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;

    /*
     * When freeing paths, FEs need to be first as they perform
     * path unbinding.
     */
    ret = avs_component_suspend_hw_free(component, false);
    if ret != 0 {
        return ret;
    }

    avs_component_suspend_hw_free(component, true)
}

unsafe extern "C" fn avs_component_resume(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;

    /*
     * When creating paths, FEs need to be last as they perform
     * path binding.
     */
    ret = avs_component_resume_hw_params(component, true);
    if ret != 0 {
        return ret;
    }

    ret = avs_component_resume_hw_params(component, false);
    if ret != 0 {
        return ret;
    }

    /* It is expected that the LINK stream is prepared first. */
    ret = avs_component_resume_prepare(component, true);
    if ret != 0 {
        return ret;
    }

    avs_component_resume_prepare(component, false)
}

static avs_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
    subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
    rates: 0,
    rate_min: 0,
    rate_max: 0,
    channels_min: 0,
    channels_max: 0,
    buffer_bytes_max: AZX_MAX_BUF_SIZE,
    period_bytes_min: 128,
    period_bytes_max: AZX_MAX_BUF_SIZE / 2,
    periods_min: 2,
    periods_max: AZX_MAX_FRAG,
    fifo_size: 0,
};

unsafe extern "C" fn avs_component_open(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);

    /* only FE DAI links are handled here */
    if (*(*rtd).dai_link).no_pcm {
        return 0;
    }

    snd_soc_set_runtime_hwparams(substream, &avs_pcm_hardware)
}

unsafe extern "C" fn avs_hda_stream_dpib_read(stream: *mut hdac_ext_stream) -> c_uint {
    readl((*(*hdac_stream(stream)).bus).remap_addr.add(AZX_REG_VS_SDXDPIB_XBASE + (AZX_REG_VS_SDXDPIB_XINTERVAL * (*hdac_stream(stream)).index as usize)))
}

unsafe extern "C" fn avs_component_pointer(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let data = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    let host_stream: *mut hdac_ext_stream;
    let mut pos: c_uint;

    if (*data).stream.host_stream.is_null() {
        return 0;
    }

    host_stream = (*data).stream.host_stream;
    pos = avs_hda_stream_dpib_read(host_stream);

    if pos >= (*hdac_stream(host_stream)).bufsize {
        pos = 0;
    }

    bytes_to_frames((*substream).runtime, pos)
}

unsafe extern "C" fn avs_component_mmap(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int {
    snd_pcm_lib_default_mmap(substream, vma)
}

unsafe extern "C" fn avs_component_new(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dai = snd_soc_rtd_to_cpu(rtd, 0);
    let pcm = (*rtd).pcm;

    if (*(*dai).driver).playback.channels_min != 0 {
        snd_pcm_set_managed_buffer((*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream, SNDRV_DMA_TYPE_DEV_SG, (*component).dev, 0, MAX_PREALLOC_SIZE);
    }

    if (*(*dai).driver).capture.channels_min != 0 {
        snd_pcm_set_managed_buffer((*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream, SNDRV_DMA_TYPE_DEV_SG, (*component).dev, 0, MAX_PREALLOC_SIZE);
    }

    0
}

static mut avs_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: c"avs-pcm".as_ptr(),
    probe: Some(avs_component_probe),
    remove: Some(avs_component_remove),
    suspend: Some(avs_component_suspend),
    resume: Some(avs_component_resume),
    open: Some(avs_component_open),
    pointer: Some(avs_component_pointer),
    mmap: Some(avs_component_mmap),
    pcm_new: Some(avs_component_new),
    module_get_upon_open: 1, /* increment refcount when a pcm is opened */
    topology_name_prefix: c"intel/avs".as_ptr(),
    probe_order: 0,
    remove_order: 0,
    use_dai_pcm_id: false,
};

#[no_mangle]
pub unsafe extern "C" fn avs_register_component(
    dev: *mut device,
    name: *const c_char,
    drv: *mut snd_soc_component_driver,
    cpu_dais: *mut snd_soc_dai_driver,
    num_cpu_dais: c_int,
) -> c_int {
    let acomp: *mut avs_soc_component;
    let comp_name: *const c_char;

    acomp = devm_kzalloc(dev, size_of::<avs_soc_component>(), GFP_KERNEL) as *mut avs_soc_component;
    if acomp.is_null() {
        return -ENOMEM;
    }

    (*acomp).base = snd_soc_component_alloc(dev);
    if (*acomp).base.is_null() {
        return -ENOMEM;
    }

    comp_name = devm_kstrdup(dev, name, GFP_KERNEL);
    if comp_name.is_null() {
        return -ENOMEM;
    }

    INIT_LIST_HEAD(addr_of_mut!((*acomp).node));

    (*drv).use_dai_pcm_id = !obsolete_card_names;

    snd_soc_component_set_name((*acomp).base, comp_name);
    snd_soc_component_set_priv((*acomp).base, acomp);

    snd_soc_register_component((*acomp).base, drv, cpu_dais, num_cpu_dais)
}

static mut dmic_cpu_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        id: 0,
        name: c"DMIC Pin".as_ptr(),
        ops: null(),
        playback: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0, subformats: 0, sig_bits: 0 },
        capture: snd_soc_pcm_stream {
            stream_name: c"DMIC Rx".as_ptr(),
            channels_min: 1,
            channels_max: 4,
            rates: SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            subformats: 0,
            sig_bits: 0,
        },
    },
    snd_soc_dai_driver {
        id: 0,
        name: c"DMIC WoV Pin".as_ptr(),
        ops: null(),
        playback: snd_soc_pcm_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0, subformats: 0, sig_bits: 0 },
        capture: snd_soc_pcm_stream {
            stream_name: c"DMIC WoV Rx".as_ptr(),
            channels_min: 1,
            channels_max: 4,
            rates: SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            subformats: 0,
            sig_bits: 0,
        },
    },
];

#[no_mangle]
pub unsafe extern "C" fn avs_register_dmic_component(adev: *mut avs_dev, name: *const c_char) -> c_int {
    let ops: *const snd_soc_dai_ops;

    if avs_platattr_test(adev, ALTHDA) {
        ops = &avs_dai_dmichda_be_ops;
    } else {
        ops = &avs_dai_nonhda_be_ops;
    }

    dmic_cpu_dais[0].ops = ops;
    dmic_cpu_dais[1].ops = ops;
    avs_register_component((*adev).dev, name, addr_of_mut!(avs_component_driver), dmic_cpu_dais.as_mut_ptr(), dmic_cpu_dais.len() as c_int)
}

static i2s_dai_template: snd_soc_dai_driver = snd_soc_dai_driver {
    id: 0,
    name: null(),
    ops: null(),
    playback: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 1,
        channels_max: AVS_CHANNELS_MAX,
        rates: SNDRV_PCM_RATE_8000_192000 | SNDRV_PCM_RATE_12000 | SNDRV_PCM_RATE_24000 | SNDRV_PCM_RATE_128000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
        sig_bits: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 1,
        channels_max: AVS_CHANNELS_MAX,
        rates: SNDRV_PCM_RATE_8000_192000 | SNDRV_PCM_RATE_12000 | SNDRV_PCM_RATE_24000 | SNDRV_PCM_RATE_128000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
        sig_bits: 0,
    },
};

#[no_mangle]
pub unsafe extern "C" fn avs_register_i2s_component(
    adev: *mut avs_dev,
    name: *const c_char,
    port_mask: c_ulong,
    tdms: *mut c_ulong,
) -> c_int {
    let cpus: *mut snd_soc_dai_driver;
    let mut dai: *mut snd_soc_dai_driver;
    let ops: *const snd_soc_dai_ops;
    let ssp_count: size_t;
    let mut cpu_count: size_t;
    let mut i: c_int;
    let mut j: c_int;

    ssp_count = (*adev).hw_cfg.i2s_caps.ctrl_count;
    if avs_platattr_test(adev, ALTHDA) {
        ops = &avs_dai_i2shda_be_ops;
    } else {
        ops = &avs_dai_nonhda_be_ops;
    }

    cpu_count = 0;
    i = 0;
    while (i as size_t) < ssp_count {
        if (port_mask & (1u64 << i) as c_ulong) != 0 {
            if tdms.is_null() || test_bit(0, tdms.add(i as usize)) {
                cpu_count += 1;
            }
        }
        i += 1;
    }
    if !tdms.is_null() {
        i = 0;
        while (i as size_t) < ssp_count {
            if (port_mask & (1u64 << i) as c_ulong) != 0 {
                cpu_count += hweight_long(*tdms.add(i as usize)) as size_t;
            }
            i += 1;
        }
    }

    cpus = devm_kcalloc((*adev).dev, cpu_count, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if cpus.is_null() {
        return -ENOMEM;
    }

    dai = cpus;
    i = 0;
    while (i as size_t) < ssp_count {
        if (port_mask & (1u64 << i) as c_ulong) != 0 {
            if tdms.is_null() || test_bit(0, tdms.add(i as usize)) {
                memcpy(dai as *mut c_void, &i2s_dai_template as *const _ as *const c_void, size_of::<snd_soc_dai_driver>());

                (*dai).name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"SSP%d Pin".as_ptr(), i);
                (*dai).playback.stream_name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"ssp%d Tx".as_ptr(), i);
                (*dai).capture.stream_name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"ssp%d Rx".as_ptr(), i);

                if (*dai).name.is_null() || (*dai).playback.stream_name.is_null() || (*dai).capture.stream_name.is_null() {
                    return -ENOMEM;
                }
                (*dai).ops = ops;
                dai = dai.add(1);
            }
        }
        i += 1;
    }

    if !tdms.is_null() {
        i = 0;
        while (i as size_t) < ssp_count {
            if (port_mask & (1u64 << i) as c_ulong) != 0 {
                j = 0;
                while (j as c_uint) < AVS_CHANNELS_MAX {
                    if test_bit(j, tdms.add(i as usize)) {
                        memcpy(dai as *mut c_void, &i2s_dai_template as *const _ as *const c_void, size_of::<snd_soc_dai_driver>());

                        (*dai).name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"SSP%d:%d Pin".as_ptr(), i, j);
                        (*dai).playback.stream_name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"ssp%d:%d Tx".as_ptr(), i, j);
                        (*dai).capture.stream_name = devm_kasprintf((*adev).dev, GFP_KERNEL, c"ssp%d:%d Rx".as_ptr(), i, j);

                        if (*dai).name.is_null() || (*dai).playback.stream_name.is_null() || (*dai).capture.stream_name.is_null() {
                            return -ENOMEM;
                        }
                        (*dai).ops = ops;
                        dai = dai.add(1);
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }

    avs_register_component((*adev).dev, name, addr_of_mut!(avs_component_driver), cpus, cpu_count as c_int)
}

/* HD-Audio CPU DAI template */
static hda_cpu_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    id: 0,
    name: null(),
    ops: &avs_dai_hda_be_ops,
    playback: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 1,
        channels_max: AVS_CHANNELS_MAX,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
        sig_bits: 0,
    },
    capture: snd_soc_pcm_stream {
        stream_name: null(),
        channels_min: 1,
        channels_max: AVS_CHANNELS_MAX,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        subformats: SNDRV_PCM_SUBFMTBIT_MSBITS_20 | SNDRV_PCM_SUBFMTBIT_MSBITS_24 | SNDRV_PCM_SUBFMTBIT_MSBITS_MAX,
        sig_bits: 0,
    },
};

unsafe extern "C" fn avs_component_hda_unregister_dais(component: *mut snd_soc_component) {
    let mach = dev_get_platdata((*(*component).card).dev);
    let mut dai: *mut snd_soc_dai = null_mut();
    let mut save: *mut snd_soc_dai = null_mut();
    let pdata = (*mach).pdata;
    let codec = (*pdata).codec;
    let mut name = [0 as c_char; 32];

    snprintf(name.as_mut_ptr(), name.len(), c"%s-cpu".as_ptr(), dev_name(addr_of_mut!((*codec).core.dev)));

    /* for_each_component_dais_safe(component, dai, save) */
    while !dai.is_null() {
        let mut stream: c_int = 0;

        if strstr((*(*dai).driver).name, name.as_ptr()).is_null() {
            continue;
        }

        while stream <= SNDRV_PCM_STREAM_CAPTURE {
            snd_soc_dapm_free_widget(snd_soc_dai_get_widget(dai, stream));
            stream += 1;
        }

        snd_soc_unregister_dai(dai);
        let _ = save;
    }
}

unsafe extern "C" fn avs_component_hda_probe(component: *mut snd_soc_component) -> c_int {
    let dapm: *mut snd_soc_dapm_context;
    let dais: *mut snd_soc_dai_driver;
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let codec: *mut hda_codec;
    let mut pcm: *mut hda_pcm;
    let cname: *const c_char;
    let mut pcm_count: c_int = 0;
    let mut ret: c_int;
    let mut i: c_int;

    mach = dev_get_platdata((*(*component).card).dev);
    if mach.is_null() {
        return -EINVAL;
    }

    pdata = (*mach).pdata;
    codec = (*pdata).codec;
    if list_empty(addr_of_mut!((*codec).pcm_list_head)) {
        return -EINVAL;
    }
    /* list_for_each_entry(pcm, &codec->pcm_list_head, list) pcm_count++; */

    dais = devm_kcalloc((*component).dev, pcm_count as size_t, size_of::<snd_soc_dai_driver>(), GFP_KERNEL) as *mut snd_soc_dai_driver;
    if dais.is_null() {
        return -ENOMEM;
    }

    cname = dev_name(addr_of_mut!((*codec).core.dev));
    dapm = snd_soc_component_to_dapm(component);
    pcm = null_mut(); /* list_first_entry(&codec->pcm_list_head, struct hda_pcm, list) */

    i = 0;
    while i < pcm_count {
        let dai: *mut snd_soc_dai;

        memcpy(dais.add(i as usize) as *mut c_void, &hda_cpu_dai as *const _ as *const c_void, size_of::<snd_soc_dai_driver>());
        (*dais.add(i as usize)).id = i;
        (*dais.add(i as usize)).name = devm_kasprintf((*component).dev, GFP_KERNEL, c"%s-cpu%d".as_ptr(), cname, i);
        if (*dais.add(i as usize)).name.is_null() {
            ret = -ENOMEM;
            avs_component_hda_unregister_dais(component);
            return ret;
        }

        if (*pcm).stream[0].substreams != 0 {
            (*dais.add(i as usize)).playback.stream_name = devm_kasprintf((*component).dev, GFP_KERNEL, c"%s-cpu%d Tx".as_ptr(), cname, i);
            if (*dais.add(i as usize)).playback.stream_name.is_null() {
                ret = -ENOMEM;
                avs_component_hda_unregister_dais(component);
                return ret;
            }

            if !hda_codec_is_display(codec) {
                (*dais.add(i as usize)).playback.formats = (*pcm).stream[0].formats;
                (*dais.add(i as usize)).playback.subformats = (*pcm).stream[0].subformats;
                (*dais.add(i as usize)).playback.rates = (*pcm).stream[0].rates;
                (*dais.add(i as usize)).playback.channels_min = (*pcm).stream[0].channels_min;
                (*dais.add(i as usize)).playback.channels_max = (*pcm).stream[0].channels_max;
                (*dais.add(i as usize)).playback.sig_bits = (*pcm).stream[0].maxbps;
            }
        }

        if (*pcm).stream[1].substreams != 0 {
            (*dais.add(i as usize)).capture.stream_name = devm_kasprintf((*component).dev, GFP_KERNEL, c"%s-cpu%d Rx".as_ptr(), cname, i);
            if (*dais.add(i as usize)).capture.stream_name.is_null() {
                ret = -ENOMEM;
                avs_component_hda_unregister_dais(component);
                return ret;
            }

            if !hda_codec_is_display(codec) {
                (*dais.add(i as usize)).capture.formats = (*pcm).stream[1].formats;
                (*dais.add(i as usize)).capture.subformats = (*pcm).stream[1].subformats;
                (*dais.add(i as usize)).capture.rates = (*pcm).stream[1].rates;
                (*dais.add(i as usize)).capture.channels_min = (*pcm).stream[1].channels_min;
                (*dais.add(i as usize)).capture.channels_max = (*pcm).stream[1].channels_max;
                (*dais.add(i as usize)).capture.sig_bits = (*pcm).stream[1].maxbps;
            }
        }

        dai = snd_soc_register_dai(component, dais.add(i as usize), false);
        if dai.is_null() {
            dev_err((*component).dev, c"register dai for %s failed\n".as_ptr(), (*pcm).name);
            ret = -EINVAL;
            avs_component_hda_unregister_dais(component);
            return ret;
        }

        ret = snd_soc_dapm_new_dai_widgets(dapm, dai);
        if ret < 0 {
            dev_err((*component).dev, c"create widgets failed: %d\n".as_ptr(), ret);
            snd_soc_unregister_dai(dai);
            avs_component_hda_unregister_dais(component);
            return ret;
        }

        /* pcm = list_next_entry(pcm, list); */
        i += 1;
    }

    ret = avs_component_probe(component);
    if ret != 0 {
        avs_component_hda_unregister_dais(component);
    }

    ret
}

unsafe extern "C" fn avs_component_hda_remove(component: *mut snd_soc_component) {
    avs_component_remove(component);
    avs_component_hda_unregister_dais(component);
}

unsafe extern "C" fn avs_component_hda_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);

    if !(*(*rtd).dai_link).no_pcm {
        let mut hwparams = avs_pcm_hardware;
        let mut be: *mut snd_soc_pcm_runtime;
        let mut dpcm: *mut snd_soc_dpcm = null_mut();
        let dir = (*substream).stream;

        /*
         * Support the DPCM reparenting while still fulfilling expectations of HDAudio
         * common code - a valid stream pointer at substream->runtime->private_data -
         * by having all FEs point to the same private data.
         */
        /* for_each_dpcm_be(rtd, dir, dpcm) */
        while !dpcm.is_null() {
            let be_substream: *mut snd_pcm_substream;

            be = (*dpcm).be;
            if (*be).dpcm[dir as usize].users == 1 {
                break;
            }

            be_substream = snd_soc_dpcm_get_substream(be, dir);
            (*(*substream).runtime).private_data = (*(*be_substream).runtime).private_data;
            break;
        }

        /* RESUME unsupported for de-coupled HD-Audio capture. */
        if dir == SNDRV_PCM_STREAM_CAPTURE {
            hwparams.info &= !SNDRV_PCM_INFO_RESUME;
        }

        return snd_soc_set_runtime_hwparams(substream, &hwparams);
    }

    let _ = component;
    0
}

static mut avs_hda_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: c"avs-hda-pcm".as_ptr(),
    probe: Some(avs_component_hda_probe),
    remove: Some(avs_component_hda_remove),
    suspend: Some(avs_component_suspend),
    resume: Some(avs_component_resume),
    open: Some(avs_component_hda_open),
    pointer: Some(avs_component_pointer),
    mmap: Some(avs_component_mmap),
    pcm_new: Some(avs_component_new),
    /*
     * hda platform component's probe() is dependent on
     * codec->pcm_list_head, it needs to be initialized after codec
     * component. remove_order is here for completeness sake
     */
    probe_order: SND_SOC_COMP_ORDER_LATE,
    remove_order: SND_SOC_COMP_ORDER_EARLY,
    module_get_upon_open: 1,
    topology_name_prefix: c"intel/avs".as_ptr(),
    use_dai_pcm_id: false,
};

#[no_mangle]
pub unsafe extern "C" fn avs_register_hda_component(adev: *mut avs_dev, name: *const c_char) -> c_int {
    avs_register_component((*adev).dev, name, addr_of_mut!(avs_hda_component_driver), null_mut(), 0)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
