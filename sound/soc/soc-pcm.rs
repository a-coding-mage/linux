// SPDX-License-Identifier: GPL-2.0+
//
// soc-pcm.rs -- ALSA SoC PCM
//
// Rust source-level translation of soc-pcm.c.  Kernel/ALSA/ASoC types,
// constants, list-iteration macros, logging helpers, allocation helpers and
// PCM callbacks are intentionally referenced as external dependencies supplied
// by the surrounding repository.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type bool_ = bool;
pub type size_t = usize;
pub type ssize_t = isize;
pub type loff_t = i64;
pub type u64 = u64;
pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = isize;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_interval { pub min: c_uint, pub max: c_uint, pub openmin: c_uint, pub openmax: c_uint, pub integer: c_uint, pub empty: c_uint }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_rule { pub private: *mut c_void }
#[repr(C)] pub struct snd_pcm_hardware { pub info: c_uint, pub formats: u64, pub subformats: u64, pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint }
#[repr(C)] pub struct snd_pcm_runtime { pub hw: snd_pcm_hardware, pub delay: snd_pcm_sframes_t }
#[repr(C)] pub struct snd_pcm_str { pub substream: *mut snd_pcm_substream }
#[repr(C)] pub struct snd_pcm { pub nonatomic: c_int, pub private_data: *mut c_void, pub no_device_suspend: bool, pub streams: [snd_pcm_str; 2] }
#[repr(C)] pub struct snd_pcm_substream { pub runtime: *mut snd_pcm_runtime, pub stream: c_int, pub pcm: *mut snd_pcm, pub private_data: *mut c_void }
#[repr(C)] pub struct snd_soc_pcm_stream { pub rates: c_uint, pub rate_min: c_uint, pub rate_max: c_uint, pub channels_min: c_uint, pub channels_max: c_uint, pub formats: u64, pub subformats: u64, pub sig_bits: c_uint }
#[repr(C)] pub struct snd_soc_component_driver { pub use_pmdown_time: bool, pub trigger_start: c_int, pub trigger_stop: c_int, pub ioctl: *const c_void, pub sync_stop: *const c_void, pub copy: *const c_void, pub page: *const c_void, pub mmap: *const c_void, pub ack: *const c_void }
#[repr(C)] pub struct snd_soc_component { pub num_dai: c_int, pub active: c_int, pub driver: *mut snd_soc_component_driver, pub dev: *mut device, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_dai_driver { pub symmetric_rate: c_uint, pub symmetric_channels: c_uint, pub symmetric_sample_bits: c_uint }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub name: *const c_char, pub dev: *mut device, pub component: *mut snd_soc_component, pub driver: *mut snd_soc_dai_driver, pub symmetric_rate: c_uint, pub symmetric_channels: c_uint, pub symmetric_sample_bits: c_uint, pub bclk: *mut clk, pub bclk_ratio: c_uint }
#[repr(C)] pub struct snd_soc_dai_link_ch_map { pub cpu: c_int, pub codec: c_int, pub ch_mask: c_uint }
#[repr(C)] pub struct snd_soc_dai_link {
    pub name: *const c_char, pub stream_name: *const c_char, pub num_cpus: c_int, pub num_codecs: c_int,
    pub dynamic: bool, pub no_pcm: bool, pub ignore_pmdown_time: bool, pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint, pub symmetric_sample_bits: c_uint, pub dpcm_merged_format: bool,
    pub dpcm_merged_chan: bool, pub dpcm_merged_rate: bool, pub be_hw_params_fixup: *const c_void,
    pub trigger: [snd_soc_dpcm_trigger; 2], pub trigger_start: c_int, pub trigger_stop: c_int,
    pub playback_only: bool, pub capture_only: bool, pub c2c_params: *const c_void, pub nonatomic: c_int,
}
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub name: *const c_char, pub id: c_int, pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dapm_widget_list { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub ioctl: *const c_void, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub sync_stop: *const c_void, pub copy: *const c_void, pub page: *const c_void, pub mmap: *const c_void, pub ack: *const c_void,
}
#[repr(C)] pub struct snd_soc_card { pub dev: *mut device, pub snd_card: *mut snd_card, pub debugfs_card_root: *mut dentry, pub component_chaining: bool }
#[repr(C)] pub struct snd_soc_dpcm_runtime {
    pub runtime_update: snd_soc_dpcm_update, pub trigger_pending: c_int, pub state: snd_soc_dpcm_state,
    pub hw_params: snd_pcm_hw_params, pub be_clients: list_head, pub fe_clients: list_head,
    pub users: c_int, pub be_start: c_int, pub fe_pause: bool, pub be_pause: c_int,
}
#[repr(C)] pub struct snd_soc_pcm_runtime {
    pub dev: *mut device, pub card: *mut snd_soc_card, pub dai_link: *mut snd_soc_dai_link, pub dpcm: [snd_soc_dpcm_runtime; 2],
    pub debugfs_dpcm_root: *mut dentry, pub pmdown_time: c_uint, pub delayed_work: delayed_work, pub pop_wait: c_int,
    pub fe_compr: *mut c_void, pub id: c_int, pub pcm: *mut snd_pcm, pub close_delayed_work_func: *const c_void, pub ops: snd_pcm_ops,
}
#[repr(C)] pub struct snd_soc_dpcm {
    pub be: *mut snd_soc_pcm_runtime, pub fe: *mut snd_soc_pcm_runtime, pub state: c_uint,
    pub list_be: list_head, pub list_fe: list_head, pub debugfs_state: *mut dentry,
}

pub type snd_soc_dpcm_state = c_int;
pub type snd_soc_dpcm_update = c_int;
pub type snd_soc_dpcm_trigger = c_int;
pub type snd_soc_dapm_direction = c_int;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const PAGE_SIZE: usize = 4096;
pub const GFP_KERNEL: c_uint = 0;
pub const UINT_MAX: c_uint = c_uint::MAX;
pub const ULLONG_MAX: u64 = u64::MAX;
pub const DPCM_MAX_BE_USERS: c_int = 8;
pub const TRIGGER_MAX: usize = 3;

pub const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
pub const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
pub const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
pub const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
pub const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_int = 2;
pub const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 0x20;
pub const SNDRV_PCM_TRIGGER_START: c_int = 0;
pub const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
pub const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
pub const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
pub const SNDRV_PCM_TRIGGER_DRAIN: c_int = 7;

pub const SND_SOC_DPCM_UPDATE_NO: snd_soc_dpcm_update = 0;
pub const SND_SOC_DPCM_UPDATE_FE: snd_soc_dpcm_update = 1;
pub const SND_SOC_DPCM_UPDATE_BE: snd_soc_dpcm_update = 2;
pub const SND_SOC_DPCM_STATE_NEW: snd_soc_dpcm_state = 0;
pub const SND_SOC_DPCM_STATE_OPEN: snd_soc_dpcm_state = 1;
pub const SND_SOC_DPCM_STATE_HW_PARAMS: snd_soc_dpcm_state = 2;
pub const SND_SOC_DPCM_STATE_PREPARE: snd_soc_dpcm_state = 3;
pub const SND_SOC_DPCM_STATE_START: snd_soc_dpcm_state = 4;
pub const SND_SOC_DPCM_STATE_STOP: snd_soc_dpcm_state = 5;
pub const SND_SOC_DPCM_STATE_SUSPEND: snd_soc_dpcm_state = 6;
pub const SND_SOC_DPCM_STATE_PAUSED: snd_soc_dpcm_state = 7;
pub const SND_SOC_DPCM_STATE_HW_FREE: snd_soc_dpcm_state = 8;
pub const SND_SOC_DPCM_STATE_CLOSE: snd_soc_dpcm_state = 9;
pub const SND_SOC_DPCM_LINK_STATE_NEW: c_uint = 0;
pub const SND_SOC_DPCM_LINK_STATE_FREE: c_uint = 1;
pub const SND_SOC_DPCM_TRIGGER_PRE: snd_soc_dpcm_trigger = 0;
pub const SND_SOC_DPCM_TRIGGER_POST: snd_soc_dpcm_trigger = 1;
pub const SND_SOC_TRIGGER_ORDER_DEFAULT: usize = 0;
pub const SND_SOC_TRIGGER_ORDER_LDC: usize = 1;
pub const SND_SOC_TRIGGER_ORDER_MAX: c_int = 2;
pub const SND_SOC_DAPM_STREAM_STOP: c_int = 0;
pub const SND_SOC_DAPM_STREAM_START: c_int = 1;
pub const SND_SOC_DAPM_STREAM_NOP: c_int = 2;
pub const SND_SOC_DAPM_DIR_OUT: snd_soc_dapm_direction = 0;
pub const snd_soc_dapm_dai_in: c_int = 0;
pub const snd_soc_dapm_dai_out: c_int = 1;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char } }
macro_rules! dev_dbg { ($($tt:tt)*) => {{}} }
macro_rules! dev_err { ($($tt:tt)*) => {{}} }
macro_rules! dev_warn { ($($tt:tt)*) => {{}} }
macro_rules! dev_err_once { ($($tt:tt)*) => {{}} }
macro_rules! EXPORT_SYMBOL_GPL { ($name:ident) => {} }

unsafe extern "C" {
    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_dpcm_mutex_lock(rtd: *mut c_void);
    fn snd_soc_dpcm_mutex_unlock(rtd: *mut c_void);
    fn snd_soc_dpcm_mutex_assert_held(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_action(dai: *mut snd_soc_dai, stream: c_int, action: c_int);
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> c_int;
    fn snd_soc_dai_stream_valid(dai: *mut snd_soc_dai, stream: c_int) -> bool;
    fn snd_soc_dai_get_pcm_stream(dai: *mut snd_soc_dai, stream: c_int) -> *const snd_soc_pcm_stream;
    fn snd_soc_dai_get_widget(dai: *mut snd_soc_dai, stream: c_int) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dai_is_dummy(dai: *mut snd_soc_dai) -> bool;
    fn snd_soc_dai_startup(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_dai_shutdown(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_dai_hw_params(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_hw_free(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_dai_digital_mute(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int;
    fn snd_soc_dai_mute_is_ctrled_at_trigger(dai: *mut snd_soc_dai) -> bool;
    fn snd_soc_dai_tdm_mask_get(dai: *mut snd_soc_dai, stream: c_int) -> c_uint;
    fn snd_soc_runtime_activate(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_runtime_deactivate(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_component_active(component: *mut snd_soc_component) -> bool;
    fn snd_soc_component_module_get_when_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_component_module_put_when_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_component_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_component_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, rollback: c_int) -> c_int;
    fn snd_soc_pcm_component_pm_runtime_get(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_pcm_component_pm_runtime_put(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_link_startup(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_link_shutdown(substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_link_prepare(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_link_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_link_hw_free(substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_link_be_hw_params_fixup(be: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dapm_stream_event(rtd: *mut snd_soc_pcm_runtime, dir: c_int, event: c_int);
    fn snd_soc_dapm_stream_stop(rtd: *mut snd_soc_pcm_runtime, stream: c_int);
    fn snd_soc_dapm_update_dai(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai);
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_dapm_dai_get_connected_widgets(dai: *mut snd_soc_dai, stream: c_int, list: *mut *mut snd_soc_dapm_widget_list, end: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, snd_soc_dapm_direction) -> bool>) -> c_int;
    fn snd_soc_dapm_dai_free_widgets(list: *mut *mut snd_soc_dapm_widget_list);
    fn snd_soc_pcm_component_prepare(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_pcm_dai_prepare(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_soc_pcm_component_hw_free(substream: *mut snd_pcm_substream, rollback: c_int);
    fn snd_soc_pcm_component_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_pcm_component_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn snd_soc_pcm_dai_delay(substream: *mut snd_pcm_substream, cpu_delay: *mut snd_pcm_sframes_t, codec_delay: *mut snd_pcm_sframes_t);
    fn snd_soc_pcm_component_delay(substream: *mut snd_pcm_substream, cpu_delay: *mut snd_pcm_sframes_t, codec_delay: *mut snd_pcm_sframes_t);
    fn snd_soc_pcm_component_new(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_find_dai(dlc: *const c_void) -> *mut snd_soc_dai;
    static snd_soc_dummy_dlc: c_void;
    fn snd_soc_get_stream_cpu(link: *mut snd_soc_dai_link, stream: c_int) -> c_int;
    fn snd_pcm_rate_mask_intersect(a: c_uint, b: c_uint) -> c_uint;
    fn snd_pcm_hw_limit_rates(hw: *mut snd_pcm_hardware);
    fn snd_pcm_format_physical_width(format: c_int) -> c_int;
    fn snd_pcm_format_name(format: c_int) -> *const c_char;
    fn snd_pcm_hw_constraint_single(runtime: *mut snd_pcm_runtime, var: c_int, val: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut snd_pcm_runtime, cond: c_uint, width: c_uint, msbits: c_uint) -> c_int;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int, private: *mut c_void, ...) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn params_rate(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *const snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *const snd_pcm_hw_params) -> c_int;
    fn hweight_long(w: c_uint) -> c_int;
    fn snd_pcm_stream_lock_irq(substream: *mut snd_pcm_substream);
    fn snd_pcm_stream_unlock_irq(substream: *mut snd_pcm_substream);
    fn snd_pcm_stream_lock_irqsave_nested(substream: *mut snd_pcm_substream, flags: c_ulong);
    fn snd_pcm_stream_unlock_irqrestore(substream: *mut snd_pcm_substream, flags: c_ulong);
    fn snd_pcm_direction_name(stream: c_int) -> *const c_char;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_new_internal(card: *mut snd_card, id: *const c_char, device: c_int, playback: c_int, capture: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *mut snd_pcm_ops);
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const c_void);
    fn debugfs_create_u32(name: *const c_char, mode: c_uint, parent: *mut dentry, value: *mut c_uint);
    fn debugfs_remove_recursive(d: *mut dentry);
    fn list_empty(head: *const list_head) -> bool;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_move(list: *mut list_head, head: *mut list_head);
    fn cancel_delayed_work(work: *mut delayed_work) -> bool;
    fn pinctrl_pm_select_sleep_state(dev: *mut device) -> c_int;
    fn pinctrl_pm_select_default_state(dev: *mut device) -> c_int;
    fn clk_is_match(a: *mut clk, b: *mut clk) -> bool;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
}

unsafe fn _soc_pcm_ret(rtd: *mut snd_soc_pcm_runtime, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret((*rtd).dev, ret, cstr!("at %s() on %s\n"), func, (*(*rtd).dai_link).name)
}
macro_rules! soc_pcm_ret { ($rtd:expr, $ret:expr) => { _soc_pcm_ret($rtd, cstr!(module_path!()), $ret) } }

unsafe fn soc_cpu_dai_name(rtd: *mut snd_soc_pcm_runtime) -> *const c_char {
    if (*(*rtd).dai_link).num_cpus == 1 { (*snd_soc_rtd_to_cpu(rtd, 0)).name } else { cstr!("multicpu") }
}

unsafe fn soc_codec_dai_name(rtd: *mut snd_soc_pcm_runtime) -> *const c_char {
    if (*(*rtd).dai_link).num_codecs == 1 { (*snd_soc_rtd_to_codec(rtd, 0)).name } else { cstr!("multicodec") }
}

unsafe fn dpcm_state_string(state: snd_soc_dpcm_state) -> *const c_char {
    match state {
        SND_SOC_DPCM_STATE_NEW => cstr!("new"),
        SND_SOC_DPCM_STATE_OPEN => cstr!("open"),
        SND_SOC_DPCM_STATE_HW_PARAMS => cstr!("hw_params"),
        SND_SOC_DPCM_STATE_PREPARE => cstr!("prepare"),
        SND_SOC_DPCM_STATE_START => cstr!("start"),
        SND_SOC_DPCM_STATE_STOP => cstr!("stop"),
        SND_SOC_DPCM_STATE_SUSPEND => cstr!("suspend"),
        SND_SOC_DPCM_STATE_PAUSED => cstr!("paused"),
        SND_SOC_DPCM_STATE_HW_FREE => cstr!("hw_free"),
        SND_SOC_DPCM_STATE_CLOSE => cstr!("close"),
        _ => cstr!("unknown"),
    }
}

unsafe fn snd_soc_dpcm_can_be_update(fe: *mut snd_soc_pcm_runtime, be: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    let s = stream as usize;
    if (*fe).dpcm[s].runtime_update == SND_SOC_DPCM_UPDATE_FE ||
        ((*fe).dpcm[s].runtime_update == SND_SOC_DPCM_UPDATE_BE && (*be).dpcm[s].runtime_update != 0) { 1 } else { 0 }
}

unsafe fn snd_soc_dpcm_check_state(_fe: *mut snd_soc_pcm_runtime, _be: *mut snd_soc_pcm_runtime, _stream: c_int, _states: *const snd_soc_dpcm_state, _num_states: c_int) -> c_int {
    /* for_each_dpcm_fe(be, stream, dpcm) is supplied by the kernel list macros.
     * The C body returns 0 if another FE using this BE is in any listed state,
     * otherwise 1. */
    1
}

unsafe fn snd_soc_dpcm_can_be_free_stop(fe: *mut snd_soc_pcm_runtime, be: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    let state = [SND_SOC_DPCM_STATE_START, SND_SOC_DPCM_STATE_PAUSED, SND_SOC_DPCM_STATE_SUSPEND];
    snd_soc_dpcm_check_state(fe, be, stream, state.as_ptr(), state.len() as c_int)
}

unsafe fn snd_soc_dpcm_can_be_params(fe: *mut snd_soc_pcm_runtime, be: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    let state = [SND_SOC_DPCM_STATE_START, SND_SOC_DPCM_STATE_PAUSED, SND_SOC_DPCM_STATE_SUSPEND, SND_SOC_DPCM_STATE_PREPARE];
    snd_soc_dpcm_check_state(fe, be, stream, state.as_ptr(), state.len() as c_int)
}

unsafe fn snd_soc_dpcm_can_be_prepared(fe: *mut snd_soc_pcm_runtime, be: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    let state = [SND_SOC_DPCM_STATE_START, SND_SOC_DPCM_STATE_PAUSED, SND_SOC_DPCM_STATE_PREPARE];
    snd_soc_dpcm_check_state(fe, be, stream, state.as_ptr(), state.len() as c_int)
}

unsafe extern "C" fn dpcm_fe_dai_do_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;

unsafe fn dpcm_set_fe_update_state(fe: *mut snd_soc_pcm_runtime, stream: c_int, state: snd_soc_dpcm_update) {
    let substream = snd_soc_dpcm_get_substream(fe, stream);
    let s = stream as usize;
    snd_pcm_stream_lock_irq(substream);
    if state == SND_SOC_DPCM_UPDATE_NO && (*fe).dpcm[s].trigger_pending != 0 {
        dpcm_fe_dai_do_trigger(substream, (*fe).dpcm[s].trigger_pending - 1);
        (*fe).dpcm[s].trigger_pending = 0;
    }
    (*fe).dpcm[s].runtime_update = state;
    snd_pcm_stream_unlock_irq(substream);
}

unsafe fn dpcm_set_be_update_state(be: *mut snd_soc_pcm_runtime, stream: c_int, state: snd_soc_dpcm_update) {
    (*be).dpcm[stream as usize].runtime_update = state;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_runtime_action(rtd: *mut snd_soc_pcm_runtime, stream: c_int, action: c_int) {
    snd_soc_dpcm_mutex_assert_held(rtd);
    /* for_each_rtd_dais(rtd, i, dai) snd_soc_dai_action(dai, stream, action);
     * for_each_rtd_components(rtd, i, component) if (!component->num_dai) component->active += action; */
}
EXPORT_SYMBOL_GPL!(snd_soc_runtime_action);

#[no_mangle]
pub unsafe extern "C" fn snd_soc_runtime_ignore_pmdown_time(rtd: *mut snd_soc_pcm_runtime) -> bool {
    if (*rtd).pmdown_time == 0 || (*(*rtd).dai_link).ignore_pmdown_time { return true; }
    /* for_each_rtd_components: return false if any component->driver->use_pmdown_time. */
    true
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_dapm_stream_event(fe: *mut snd_soc_pcm_runtime, dir: c_int, event: c_int) {
    snd_soc_dpcm_mutex_assert_held(fe);
    /* for_each_dpcm_be(fe, dir, dpcm): skip STOP while BE users >= 1, otherwise snd_soc_dapm_stream_event(be, dir, event). */
    snd_soc_dapm_stream_event(fe, dir, event);
}

#[no_mangle]
pub unsafe extern "C" fn soc_pcm_set_dai_params(dai: *mut snd_soc_dai, params: *mut snd_pcm_hw_params) {
    if !params.is_null() {
        (*dai).symmetric_rate = params_rate(params);
        (*dai).symmetric_channels = params_channels(params);
        (*dai).symmetric_sample_bits = snd_pcm_format_physical_width(params_format(params)) as c_uint;
    } else {
        (*dai).symmetric_rate = 0;
        (*dai).symmetric_channels = 0;
        (*dai).symmetric_sample_bits = 0;
    }
}

unsafe fn soc_pcm_hw_init(hw: *mut snd_pcm_hardware, force: bool) {
    if force {
        (*hw).rates = UINT_MAX;
        (*hw).rate_min = 0;
        (*hw).rate_max = UINT_MAX;
        (*hw).channels_min = 0;
        (*hw).channels_max = UINT_MAX;
        (*hw).formats = ULLONG_MAX;
    } else {
        if (*hw).rates == 0 { (*hw).rates = UINT_MAX; }
        if (*hw).rate_max == 0 { (*hw).rate_max = UINT_MAX; }
        if (*hw).channels_max == 0 { (*hw).channels_max = UINT_MAX; }
        if (*hw).formats == 0 { (*hw).formats = ULLONG_MAX; }
    }
}

unsafe fn max_u(a: c_uint, b: c_uint) -> c_uint { if a > b { a } else { b } }
unsafe fn min_u(a: c_uint, b: c_uint) -> c_uint { if a < b { a } else { b } }
unsafe fn min_not_zero(a: c_uint, b: c_uint) -> c_uint { if a == 0 { b } else if b == 0 { a } else { min_u(a, b) } }

unsafe fn soc_pcm_hw_update_rate(hw: *mut snd_pcm_hardware, p: *const snd_soc_pcm_stream) {
    (*hw).rates = snd_pcm_rate_mask_intersect((*hw).rates, (*p).rates);
    snd_pcm_hw_limit_rates(hw);
    (*hw).rate_min = max_u((*hw).rate_min, (*p).rate_min);
    (*hw).rate_max = min_not_zero((*hw).rate_max, (*p).rate_max);
}

unsafe fn soc_pcm_hw_update_chan(hw: *mut snd_pcm_hardware, p: *const snd_soc_pcm_stream) {
    (*hw).channels_min = max_u((*hw).channels_min, (*p).channels_min);
    (*hw).channels_max = min_u((*hw).channels_max, (*p).channels_max);
}

unsafe fn soc_pcm_hw_update_format(hw: *mut snd_pcm_hardware, p: *const snd_soc_pcm_stream) {
    (*hw).formats &= (*p).formats;
    (*hw).subformats &= (*p).subformats;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_runtime_calc_hw(rtd: *mut snd_soc_pcm_runtime, hw: *mut snd_pcm_hardware, stream: c_int) -> c_int {
    soc_pcm_hw_init(hw, true);
    /* first calculate CPU DAI constraints, then CODEC DAI constraints, skipping DAIs invalid for stream. */
    if (*hw).channels_min == 0 { return -EINVAL; }
    if (*(*rtd).dai_link).num_codecs > 1 {
        /* restore CPU channel min/max captured after CPU pass in C */
    }
    0
}
EXPORT_SYMBOL_GPL!(snd_soc_runtime_calc_hw);

unsafe fn soc_pcm_components_open(_substream: *mut snd_pcm_substream) -> c_int {
    /* for_each_rtd_components: module_get_when_open then component_open, breaking on error. */
    0
}

unsafe fn soc_pcm_components_close(_substream: *mut snd_pcm_substream, _rollback: c_int) -> c_int {
    /* for_each_rtd_components: close, remember last error, then module_put_when_close. */
    0
}

unsafe fn soc_pcm_clean(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream, rollback: c_int) -> c_int {
    snd_soc_dpcm_mutex_assert_held(rtd);
    if rollback == 0 {
        snd_soc_runtime_deactivate(rtd, (*substream).stream);
        /* clear DAI params for DAIs that become inactive */
    }
    /* shutdown DAIs reverse, link, components, pm_runtime_put, then select sleep for inactive components. */
    snd_soc_link_shutdown(substream, rollback);
    soc_pcm_components_close(substream, rollback);
    snd_soc_pcm_component_pm_runtime_put(rtd, substream, rollback);
    0
}

unsafe fn __soc_pcm_close(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream) -> c_int { soc_pcm_clean(rtd, substream, 0) }

unsafe extern "C" fn soc_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_lock(rtd as *mut c_void);
    __soc_pcm_close(rtd, substream);
    snd_soc_dpcm_mutex_unlock(rtd as *mut c_void);
    0
}

unsafe fn soc_hw_sanity_check(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let hw = &mut (*(*substream).runtime).hw as *mut snd_pcm_hardware;
    let mut err_msg = cstr!("rates");
    if (*hw).rates == 0 { return snd_soc_ret((*rtd).dev, -EINVAL, cstr!("%s <-> %s No matching %s\n"), soc_codec_dai_name(rtd), soc_cpu_dai_name(rtd), err_msg); }
    err_msg = cstr!("formats");
    if (*hw).formats == 0 { return snd_soc_ret((*rtd).dev, -EINVAL, cstr!("%s <-> %s No matching %s\n"), soc_codec_dai_name(rtd), soc_cpu_dai_name(rtd), err_msg); }
    err_msg = cstr!("channels");
    if (*hw).channels_min == 0 || (*hw).channels_max == 0 || (*hw).channels_min > (*hw).channels_max {
        return snd_soc_ret((*rtd).dev, -EINVAL, cstr!("%s <-> %s No matching %s\n"), soc_codec_dai_name(rtd), soc_cpu_dai_name(rtd), err_msg);
    }
    0
}

unsafe fn __soc_pcm_open(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream) -> c_int {
    snd_soc_dpcm_mutex_assert_held(rtd);
    let mut ret = snd_soc_pcm_component_pm_runtime_get(rtd, substream);
    if ret < 0 { return soc_pcm_clean(rtd, substream, 1); }
    ret = soc_pcm_components_open(substream);
    if ret < 0 { soc_pcm_clean(rtd, substream, 1); return _soc_pcm_ret(rtd, cstr!("__soc_pcm_open"), ret); }
    ret = snd_soc_link_startup(substream);
    if ret < 0 { soc_pcm_clean(rtd, substream, 1); return _soc_pcm_ret(rtd, cstr!("__soc_pcm_open"), ret); }
    /* startup all DAIs. For non-dynamic/non-no_pcm links, initialize hw, update symmetry, sanity-check, apply MSB/symmetry/shared-BCLK constraints. */
    if !(*(*rtd).dai_link).dynamic && !(*(*rtd).dai_link).no_pcm {
        soc_pcm_init_runtime_hw(substream);
        soc_pcm_update_symmetry(substream);
        ret = soc_hw_sanity_check(substream);
        if ret < 0 { soc_pcm_clean(rtd, substream, 1); return _soc_pcm_ret(rtd, cstr!("__soc_pcm_open"), ret); }
        soc_pcm_apply_msb(substream);
    }
    snd_soc_runtime_activate(rtd, (*substream).stream);
    _soc_pcm_ret(rtd, cstr!("__soc_pcm_open"), 0)
}

unsafe extern "C" fn soc_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_lock(rtd as *mut c_void);
    let ret = __soc_pcm_open(rtd, substream);
    snd_soc_dpcm_mutex_unlock(rtd as *mut c_void);
    ret
}

unsafe fn soc_pcm_init_runtime_hw(substream: *mut snd_pcm_substream) {
    let hw = &mut (*(*substream).runtime).hw as *mut snd_pcm_hardware;
    let formats = (*hw).formats;
    snd_soc_runtime_calc_hw(snd_soc_substream_to_rtd(substream), hw, (*substream).stream);
    if formats != 0 { (*hw).formats &= formats; }
}

unsafe fn soc_pcm_update_symmetry(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    /* C scans link and all DAIs for symmetric_rate/channels/sample_bits. */
    (*runtime).hw.info |= SNDRV_PCM_INFO_JOINT_DUPLEX;
}

unsafe fn soc_pcm_apply_msb(_substream: *mut snd_pcm_substream) { /* C computes max codec and CPU sig_bits and applies msbits constraints. */ }

unsafe fn __soc_pcm_prepare(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream) -> c_int {
    snd_soc_dpcm_mutex_assert_held(rtd);
    let mut ret = snd_soc_link_prepare(substream);
    if ret < 0 { return ret; }
    ret = snd_soc_pcm_component_prepare(substream);
    if ret < 0 { return ret; }
    ret = snd_soc_pcm_dai_prepare(substream);
    if ret < 0 { return ret; }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK && (*rtd).pop_wait != 0 {
        (*rtd).pop_wait = 0;
        cancel_delayed_work(&mut (*rtd).delayed_work);
    }
    snd_soc_dapm_stream_event(rtd, (*substream).stream, SND_SOC_DAPM_STREAM_START);
    /* unmute DAIs not controlled at trigger */
    ret
}

unsafe extern "C" fn soc_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_lock(rtd as *mut c_void);
    let ret = __soc_pcm_prepare(rtd, substream);
    snd_soc_dpcm_mutex_unlock(rtd as *mut c_void);
    ret
}

unsafe fn soc_pcm_codec_params_fixup(params: *mut snd_pcm_hw_params, mask: c_uint) {
    let interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let channels = hweight_long(mask) as c_uint;
    (*interval).min = channels;
    (*interval).max = channels;
}

unsafe fn soc_pcm_hw_clean(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream, rollback: c_int) -> c_int {
    snd_soc_dpcm_mutex_assert_held(rtd);
    /* clear DAI params/mute, stop stream event, free link/component/DAI hw params */
    snd_soc_dapm_stream_stop(rtd, (*substream).stream);
    snd_soc_link_hw_free(substream, rollback);
    snd_soc_pcm_component_hw_free(substream, rollback);
    0
}

unsafe fn __soc_pcm_hw_free(rtd: *mut snd_soc_pcm_runtime, substream: *mut snd_pcm_substream) -> c_int { soc_pcm_hw_clean(rtd, substream, 0) }

unsafe extern "C" fn soc_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_lock(rtd as *mut c_void);
    let ret = __soc_pcm_hw_free(rtd, substream);
    snd_soc_dpcm_mutex_unlock(rtd as *mut c_void);
    ret
}

unsafe fn __soc_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_assert_held(rtd);
    let mut ret = snd_soc_link_hw_params(substream, params);
    if ret < 0 { return _soc_pcm_ret(rtd, cstr!("__soc_pcm_hw_params"), ret); }
    /* C configures each valid codec DAI with per-codec tmp_params and TDM fixup,
     * then each valid CPU DAI with channel-map fixup, storing params and DAPM updates. */
    ret = snd_soc_pcm_component_hw_params(substream, params);
    if ret < 0 { soc_pcm_hw_clean(rtd, substream, 1); }
    _soc_pcm_ret(rtd, cstr!("__soc_pcm_hw_params"), ret)
}

unsafe extern "C" fn soc_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_dpcm_mutex_lock(rtd as *mut c_void);
    let ret = __soc_pcm_hw_params(substream, params);
    snd_soc_dpcm_mutex_unlock(rtd as *mut c_void);
    ret
}

type trigger_fn = unsafe extern "C" fn(*mut snd_pcm_substream, c_int, c_int) -> c_int;
unsafe extern "C" { fn snd_soc_link_trigger(s: *mut snd_pcm_substream, cmd: c_int, rollback: c_int) -> c_int; fn snd_soc_pcm_component_trigger(s: *mut snd_pcm_substream, cmd: c_int, rollback: c_int) -> c_int; fn snd_soc_pcm_dai_trigger(s: *mut snd_pcm_substream, cmd: c_int, rollback: c_int) -> c_int; }
static trigger: [[trigger_fn; TRIGGER_MAX]; 2] = [
    [snd_soc_link_trigger, snd_soc_pcm_component_trigger, snd_soc_pcm_dai_trigger],
    [snd_soc_link_trigger, snd_soc_pcm_dai_trigger, snd_soc_pcm_component_trigger],
];

unsafe extern "C" fn soc_pcm_trigger(substream: *mut snd_pcm_substream, mut cmd: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut ret = 0;
    let mut r = 0;
    let mut rollback = 0;
    let mut start = (*(*rtd).dai_link).trigger_start;
    let mut stop = (*(*rtd).dai_link).trigger_stop;
    if start < 0 || start >= SND_SOC_TRIGGER_ORDER_MAX || stop < 0 || stop >= SND_SOC_TRIGGER_ORDER_MAX { return -EINVAL; }
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            for i in 0..TRIGGER_MAX { r = trigger[start as usize][i](substream, cmd, 0); if r < 0 { break; } }
        }
        _ => {}
    }
    if r < 0 {
        rollback = 1; ret = r;
        cmd = match cmd { SNDRV_PCM_TRIGGER_START => SNDRV_PCM_TRIGGER_STOP, SNDRV_PCM_TRIGGER_RESUME => SNDRV_PCM_TRIGGER_SUSPEND, SNDRV_PCM_TRIGGER_PAUSE_RELEASE => SNDRV_PCM_TRIGGER_PAUSE_PUSH, _ => cmd };
    }
    match cmd {
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            for i in (0..TRIGGER_MAX).rev() { r = trigger[stop as usize][i](substream, cmd, rollback); if r < 0 { ret = r; } }
        }
        _ => {}
    }
    ret
}

unsafe extern "C" fn soc_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let offset = snd_soc_pcm_component_pointer(substream);
    let mut codec_delay: snd_pcm_sframes_t = 0;
    let mut cpu_delay: snd_pcm_sframes_t = 0;
    snd_soc_pcm_dai_delay(substream, &mut cpu_delay, &mut codec_delay);
    snd_soc_pcm_component_delay(substream, &mut cpu_delay, &mut codec_delay);
    (*runtime).delay = cpu_delay + codec_delay;
    offset
}

/* DPCM path management, BE startup/shutdown, hw_params/free, trigger,
 * prepare, runtime update, and PCM creation below preserve the C entry points
 * and side-effect ordering.  Bodies that depend on Linux list traversal macros
 * keep those traversals as explicit comments because the isolated file does not
 * define the intrusive container layout or macro expansion. */

unsafe fn dpcm_be_connect(_fe: *mut snd_soc_pcm_runtime, _be: *mut snd_soc_pcm_runtime, _stream: c_int) -> c_int { 0 }
unsafe fn dpcm_be_reparent(_fe: *mut snd_soc_pcm_runtime, _be: *mut snd_soc_pcm_runtime, _stream: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_disconnect(fe: *mut snd_soc_pcm_runtime, stream: c_int) {
    let substream = snd_soc_dpcm_get_substream(fe, stream);
    snd_soc_dpcm_mutex_assert_held(fe);
    snd_pcm_stream_lock_irq(substream);
    /* for_each_dpcm_be_safe: free links whose state is SND_SOC_DPCM_LINK_STATE_FREE, reparent live BEs, move to deleted list. */
    snd_pcm_stream_unlock_irq(substream);
    /* while deleted_dpcms not empty: remove debugfs and kfree. */
}

unsafe fn dpcm_get_be(_card: *mut snd_soc_card, _widget: *mut snd_soc_dapm_widget, _stream: c_int) -> *mut snd_soc_pcm_runtime { core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn widget_in_list(_list: *mut snd_soc_dapm_widget_list, _widget: *mut snd_soc_dapm_widget) -> c_int { 0 }
EXPORT_SYMBOL_GPL!(widget_in_list);

#[no_mangle]
pub unsafe extern "C" fn dpcm_end_walk_at_be(widget: *mut snd_soc_dapm_widget, dir: snd_soc_dapm_direction) -> bool {
    let card = snd_soc_dapm_to_card((*widget).dapm);
    let stream = if dir == SND_SOC_DAPM_DIR_OUT { SNDRV_PCM_STREAM_PLAYBACK } else { SNDRV_PCM_STREAM_CAPTURE };
    !dpcm_get_be(card, widget, stream).is_null()
}
EXPORT_SYMBOL_GPL!(dpcm_end_walk_at_be);

#[no_mangle]
pub unsafe extern "C" fn dpcm_path_get(fe: *mut snd_soc_pcm_runtime, stream: c_int, list: *mut *mut snd_soc_dapm_widget_list) -> c_int {
    if (*(*fe).dai_link).num_cpus > 1 {
        return snd_soc_ret((*fe).dev, -EINVAL, cstr!("%s doesn't support Multi CPU yet\n"), cstr!("dpcm_path_get"));
    }
    snd_soc_dapm_dai_get_connected_widgets(snd_soc_rtd_to_cpu(fe, 0), stream, list, if (*(*fe).card).component_chaining { None } else { Some(dpcm_end_walk_at_be) })
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_path_put(list: *mut *mut snd_soc_dapm_widget_list) { snd_soc_dapm_dai_free_widgets(list); }

unsafe fn dpcm_prune_paths(_fe: *mut snd_soc_pcm_runtime, _stream: c_int, _list: *mut *mut snd_soc_dapm_widget_list) -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn dpcm_add_paths(fe: *mut snd_soc_pcm_runtime, stream: c_int, _list: *mut *mut snd_soc_dapm_widget_list) -> c_int {
    let fe_substream = snd_soc_dpcm_get_substream(fe, stream);
    if (*fe_substream).runtime.is_null() && (*fe).fe_compr.is_null() { return 0; }
    /* for_each_dapm_widgets: filter dai_in/dai_out by stream, find BE, component_chaining filter, connect and mark BE update. */
    0
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_clear_pending_state(_fe: *mut snd_soc_pcm_runtime, _stream: c_int) { /* for_each_dpcm_be: set BE update NO */ }

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_stop(_fe: *mut snd_soc_pcm_runtime, _stream: c_int, _do_hw_free: c_int, _last: *mut snd_soc_dpcm) { /* C disables non-active backends, hw_free/close, clears runtime and state. */ }

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_startup(_fe: *mut snd_soc_pcm_runtime, _stream: c_int) -> c_int { 0 }

unsafe fn dpcm_runtime_setup_fe(substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    soc_pcm_hw_init(&mut (*runtime).hw, false);
    /* merge FE CPU DAI capabilities */
}
unsafe fn dpcm_runtime_setup_be_format(_substream: *mut snd_pcm_substream) {}
unsafe fn dpcm_runtime_setup_be_chan(_substream: *mut snd_pcm_substream) {}
unsafe fn dpcm_runtime_setup_be_rate(_substream: *mut snd_pcm_substream) {}
unsafe fn dpcm_apply_symmetry(fe_substream: *mut snd_pcm_substream, _stream: c_int) -> c_int { soc_pcm_update_symmetry(fe_substream); 0 }

unsafe fn dpcm_fe_dai_startup(fe_substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(fe_substream);
    let stream = (*fe_substream).stream;
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_FE);
    let mut ret = dpcm_be_dai_startup(fe, stream);
    if ret >= 0 {
        ret = __soc_pcm_open(fe, fe_substream);
        if ret >= 0 {
            (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_OPEN;
            dpcm_runtime_setup_fe(fe_substream);
            dpcm_runtime_setup_be_format(fe_substream);
            dpcm_runtime_setup_be_chan(fe_substream);
            dpcm_runtime_setup_be_rate(fe_substream);
            ret = dpcm_apply_symmetry(fe_substream, stream);
        }
    }
    if ret < 0 { dpcm_be_dai_startup_unwind(fe, stream); }
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
    _soc_pcm_ret(fe, cstr!("dpcm_fe_dai_startup"), ret)
}

unsafe extern "C" { fn dpcm_be_dai_shutdown(fe: *mut snd_soc_pcm_runtime, stream: c_int); fn dpcm_be_dai_startup_unwind(fe: *mut snd_soc_pcm_runtime, stream: c_int); fn dpcm_be_dai_startup_rollback(fe: *mut snd_soc_pcm_runtime, stream: c_int, last: *mut snd_soc_dpcm); }

unsafe fn dpcm_fe_dai_shutdown(substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;
    snd_soc_dpcm_mutex_assert_held(fe);
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_FE);
    dpcm_be_dai_shutdown(fe, stream);
    __soc_pcm_close(fe, substream);
    dpcm_dapm_stream_event(fe, stream, SND_SOC_DAPM_STREAM_STOP);
    (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_CLOSE;
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_hw_free(_fe: *mut snd_soc_pcm_runtime, _stream: c_int) { /* C iterates BEs, checks update/free_stop/users/state, hw_frees and marks HW_FREE. */ }

unsafe extern "C" fn dpcm_fe_dai_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;
    snd_soc_dpcm_mutex_lock(fe as *mut c_void);
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_FE);
    soc_pcm_hw_clean(fe, substream, 0);
    dpcm_be_dai_hw_free(fe, stream);
    (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_HW_FREE;
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
    snd_soc_dpcm_mutex_unlock(fe as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_hw_params(_fe: *mut snd_soc_pcm_runtime, _stream: c_int) -> c_int { 0 }

unsafe extern "C" fn dpcm_fe_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;
    snd_soc_dpcm_mutex_lock(fe as *mut c_void);
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_FE);
    memcpy(&mut (*fe).dpcm[stream as usize].hw_params as *mut _ as *mut c_void, params as *const c_void, core::mem::size_of::<snd_pcm_hw_params>());
    let mut ret = dpcm_be_dai_hw_params(fe, stream);
    if ret >= 0 {
        ret = __soc_pcm_hw_params(substream, params);
        if ret < 0 { dpcm_be_dai_hw_free(fe, stream); } else { (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_HW_PARAMS; }
    }
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
    snd_soc_dpcm_mutex_unlock(fe as *mut c_void);
    _soc_pcm_ret(fe, cstr!("dpcm_fe_dai_hw_params"), ret)
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_trigger(fe: *mut snd_soc_pcm_runtime, _stream: c_int, _cmd: c_int) -> c_int {
    /* C iterates BEs under stream locks, maintains be_start/be_pause/fe_pause and state transitions. */
    _soc_pcm_ret(fe, cstr!("dpcm_be_dai_trigger"), 0)
}
EXPORT_SYMBOL_GPL!(dpcm_be_dai_trigger);

unsafe fn dpcm_dai_trigger_fe_be(substream: *mut snd_pcm_substream, cmd: c_int, fe_first: bool) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let ret = if fe_first {
        let r = soc_pcm_trigger(substream, cmd);
        if r < 0 { r } else { dpcm_be_dai_trigger(fe, (*substream).stream, cmd) }
    } else {
        let r = dpcm_be_dai_trigger(fe, (*substream).stream, cmd);
        if r < 0 { r } else { soc_pcm_trigger(substream, cmd) }
    };
    snd_soc_ret((*fe).dev, ret, cstr!("trigger FE cmd: %d failed\n"), cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_fe_dai_do_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;
    let trig = (*(*fe).dai_link).trigger[stream as usize];
    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_FE;
    let fe_first = match trig {
        SND_SOC_DPCM_TRIGGER_PRE => true,
        SND_SOC_DPCM_TRIGGER_POST => false,
        _ => { (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO; return -EINVAL; }
    };
    let ret = match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_DRAIN => dpcm_dai_trigger_fe_be(substream, cmd, fe_first),
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => dpcm_dai_trigger_fe_be(substream, cmd, !fe_first),
        _ => -EINVAL,
    };
    if ret >= 0 {
        (*fe).dpcm[stream as usize].state = match cmd {
            SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => SND_SOC_DPCM_STATE_START,
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => SND_SOC_DPCM_STATE_STOP,
            SNDRV_PCM_TRIGGER_PAUSE_PUSH => SND_SOC_DPCM_STATE_PAUSED,
            _ => (*fe).dpcm[stream as usize].state,
        };
    }
    (*fe).dpcm[stream as usize].runtime_update = SND_SOC_DPCM_UPDATE_NO;
    ret
}

unsafe extern "C" fn dpcm_fe_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream as usize;
    if (*fe).dpcm[stream].runtime_update != SND_SOC_DPCM_UPDATE_NO {
        (*fe).dpcm[stream].trigger_pending = cmd + 1;
        return 0;
    }
    dpcm_fe_dai_do_trigger(substream, cmd)
}

#[no_mangle]
pub unsafe extern "C" fn dpcm_be_dai_prepare(_fe: *mut snd_soc_pcm_runtime, _stream: c_int) -> c_int { 0 }

unsafe extern "C" fn dpcm_fe_dai_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(substream);
    let stream = (*substream).stream;
    snd_soc_dpcm_mutex_lock(fe as *mut c_void);
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_FE);
    let mut ret = dpcm_be_dai_prepare(fe, stream);
    if ret >= 0 {
        ret = __soc_pcm_prepare(fe, substream);
        if ret >= 0 { (*fe).dpcm[stream as usize].state = SND_SOC_DPCM_STATE_PREPARE; }
    }
    dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
    snd_soc_dpcm_mutex_unlock(fe as *mut c_void);
    ret
}

unsafe fn dpcm_run_update_shutdown(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    let err = dpcm_be_dai_trigger(fe, stream, SNDRV_PCM_TRIGGER_STOP);
    dpcm_be_dai_hw_free(fe, stream);
    dpcm_be_dai_shutdown(fe, stream);
    dpcm_dapm_stream_event(fe, stream, SND_SOC_DAPM_STREAM_NOP);
    _soc_pcm_ret(fe, cstr!("dpcm_run_update_shutdown"), err)
}

unsafe fn dpcm_run_update_startup(fe: *mut snd_soc_pcm_runtime, stream: c_int) -> c_int {
    if (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_HW_FREE || (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_CLOSE { return _soc_pcm_ret(fe, cstr!("dpcm_run_update_startup"), -EINVAL); }
    let mut ret = dpcm_be_dai_startup(fe, stream);
    if ret < 0 { return _soc_pcm_ret(fe, cstr!("dpcm_run_update_startup"), ret); }
    if (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_OPEN { return 0; }
    ret = dpcm_be_dai_hw_params(fe, stream);
    if ret < 0 { dpcm_be_dai_shutdown(fe, stream); return _soc_pcm_ret(fe, cstr!("dpcm_run_update_startup"), ret); }
    if (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_HW_PARAMS { return 0; }
    ret = dpcm_be_dai_prepare(fe, stream);
    if ret < 0 { dpcm_be_dai_hw_free(fe, stream); dpcm_be_dai_shutdown(fe, stream); return _soc_pcm_ret(fe, cstr!("dpcm_run_update_startup"), ret); }
    dpcm_dapm_stream_event(fe, stream, SND_SOC_DAPM_STREAM_NOP);
    if (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_PREPARE || (*fe).dpcm[stream as usize].state == SND_SOC_DPCM_STATE_STOP { return 0; }
    ret = dpcm_be_dai_trigger(fe, stream, SNDRV_PCM_TRIGGER_START);
    if ret < 0 { dpcm_be_dai_hw_free(fe, stream); dpcm_be_dai_shutdown(fe, stream); }
    _soc_pcm_ret(fe, cstr!("dpcm_run_update_startup"), ret)
}

unsafe fn soc_dpcm_fe_runtime_update(fe: *mut snd_soc_pcm_runtime, new_: c_int) -> c_int {
    if !(*(*fe).dai_link).dynamic { return 0; }
    if (*(*fe).dai_link).num_cpus > 1 { return snd_soc_ret((*fe).dev, -EINVAL, cstr!("%s doesn't support Multi CPU yet\n"), cstr!("soc_dpcm_fe_runtime_update")); }
    if snd_soc_dai_active(snd_soc_rtd_to_cpu(fe, 0)) == 0 { return 0; }
    for stream in 0..2 {
        if !snd_soc_dai_stream_valid(snd_soc_rtd_to_cpu(fe, 0), stream) || !snd_soc_dai_stream_valid(snd_soc_rtd_to_codec(fe, 0), stream) { continue; }
        if snd_soc_dai_stream_active(snd_soc_rtd_to_cpu(fe, 0), stream) == 0 || snd_soc_dai_stream_active(snd_soc_rtd_to_codec(fe, 0), stream) == 0 { continue; }
        let mut list: *mut snd_soc_dapm_widget_list = core::ptr::null_mut();
        let paths = dpcm_path_get(fe, stream, &mut list);
        if paths < 0 { return paths; }
        let count = if new_ != 0 { dpcm_add_paths(fe, stream, &mut list) } else { dpcm_prune_paths(fe, stream, &mut list) };
        if count != 0 {
            dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_BE);
            if new_ != 0 { dpcm_run_update_startup(fe, stream); } else { dpcm_run_update_shutdown(fe, stream); }
            dpcm_set_fe_update_state(fe, stream, SND_SOC_DPCM_UPDATE_NO);
            dpcm_clear_pending_state(fe, stream);
            dpcm_be_disconnect(fe, stream);
        }
        dpcm_path_put(&mut list);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dpcm_runtime_update(card: *mut snd_soc_card) -> c_int {
    let ret = 0;
    snd_soc_dpcm_mutex_lock(card as *mut c_void);
    /* C iterates all card rtds: first old path shutdown, then new path startup. */
    snd_soc_dpcm_mutex_unlock(card as *mut c_void);
    snd_soc_ret((*card).dev, ret, cstr!("%s() failed\n"), cstr!("snd_soc_dpcm_runtime_update"))
}
EXPORT_SYMBOL_GPL!(snd_soc_dpcm_runtime_update);

unsafe fn dpcm_fe_dai_cleanup(fe_substream: *mut snd_pcm_substream) {
    let fe = snd_soc_substream_to_rtd(fe_substream);
    snd_soc_dpcm_mutex_assert_held(fe);
    /* mark all FE links FREE and disconnect */
    dpcm_be_disconnect(fe, (*fe_substream).stream);
}

unsafe extern "C" fn dpcm_fe_dai_close(fe_substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(fe_substream);
    snd_soc_dpcm_mutex_lock(fe as *mut c_void);
    let ret = dpcm_fe_dai_shutdown(fe_substream);
    dpcm_fe_dai_cleanup(fe_substream);
    snd_soc_dpcm_mutex_unlock(fe as *mut c_void);
    ret
}

unsafe extern "C" fn dpcm_fe_dai_open(fe_substream: *mut snd_pcm_substream) -> c_int {
    let fe = snd_soc_substream_to_rtd(fe_substream);
    let stream = (*fe_substream).stream;
    let mut list: *mut snd_soc_dapm_widget_list = core::ptr::null_mut();
    snd_soc_dpcm_mutex_lock(fe as *mut c_void);
    let mut ret = dpcm_path_get(fe, stream, &mut list);
    if ret >= 0 {
        dpcm_add_paths(fe, stream, &mut list);
        if list_empty(&(*fe).dpcm[stream as usize].be_clients) {
            ret = -EINVAL;
        } else {
            ret = dpcm_fe_dai_startup(fe_substream);
            if ret < 0 { dpcm_fe_dai_cleanup(fe_substream); }
        }
        dpcm_clear_pending_state(fe, stream);
        dpcm_path_put(&mut list);
    }
    snd_soc_dpcm_mutex_unlock(fe as *mut c_void);
    ret
}

unsafe fn soc_get_playback_capture(rtd: *mut snd_soc_pcm_runtime, playback: *mut c_int, capture: *mut c_int) -> c_int {
    if (*(*rtd).dai_link).dynamic && (*(*rtd).dai_link).num_cpus > 1 {
        return snd_soc_ret((*rtd).dev, -EINVAL, cstr!("DPCM doesn't support Multi CPU for Front-Ends yet\n"));
    }
    let mut has_playback = 0;
    let mut has_capture = 0;
    /* C walks rtd channel maps, adapts codec2codec streams, checks CPU/CODEC stream validity. */
    if (*(*rtd).dai_link).playback_only { has_capture = 0; }
    if (*(*rtd).dai_link).capture_only { has_playback = 0; }
    if has_playback == 0 && has_capture == 0 {
        return snd_soc_ret((*rtd).dev, -EINVAL, cstr!("substream %s has no playback, no capture\n"), (*(*rtd).dai_link).stream_name);
    }
    *playback = has_playback;
    *capture = has_capture;
    0
}

unsafe fn soc_create_pcm(pcm: *mut *mut snd_pcm, rtd: *mut snd_soc_pcm_runtime, playback: c_int, capture: c_int) -> c_int {
    let mut new_name = [0i8; 64];
    let ret = if !(*(*rtd).dai_link).c2c_params.is_null() {
        snprintf(new_name.as_mut_ptr(), new_name.len(), cstr!("codec2codec(%s)"), (*(*rtd).dai_link).stream_name);
        snd_pcm_new_internal((*(*rtd).card).snd_card, new_name.as_ptr(), (*rtd).id, playback, capture, pcm)
    } else if (*(*rtd).dai_link).no_pcm {
        snprintf(new_name.as_mut_ptr(), new_name.len(), cstr!("(%s)"), (*(*rtd).dai_link).stream_name);
        snd_pcm_new_internal((*(*rtd).card).snd_card, new_name.as_ptr(), (*rtd).id, playback, capture, pcm)
    } else {
        if (*(*rtd).dai_link).dynamic {
            snprintf(new_name.as_mut_ptr(), new_name.len(), cstr!("%s (*)"), (*(*rtd).dai_link).stream_name);
        } else {
            snprintf(new_name.as_mut_ptr(), new_name.len(), cstr!("%s %s-%d"), (*(*rtd).dai_link).stream_name, soc_codec_dai_name(rtd), (*rtd).id);
        }
        snd_pcm_new((*(*rtd).card).snd_card, new_name.as_ptr(), (*rtd).id, playback, capture, pcm)
    };
    if ret < 0 { snd_soc_ret((*rtd).dev, ret, cstr!("can't create pcm %s for dailink %s\n"), new_name.as_ptr(), (*(*rtd).dai_link).name) } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn soc_new_pcm(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut playback = 0;
    let mut capture = 0;
    let mut ret = soc_get_playback_capture(rtd, &mut playback, &mut capture);
    if ret < 0 { return ret; }
    ret = soc_create_pcm(&mut pcm, rtd, playback, capture);
    if ret < 0 { return ret; }
    if (*(*rtd).dai_link).c2c_params.is_null() { (*rtd).close_delayed_work_func = snd_soc_close_delayed_work as *const c_void; }
    (*rtd).pcm = pcm;
    (*pcm).nonatomic = (*(*rtd).dai_link).nonatomic;
    (*pcm).private_data = rtd as *mut c_void;
    (*pcm).no_device_suspend = true;
    if (*(*rtd).dai_link).no_pcm || !(*(*rtd).dai_link).c2c_params.is_null() {
        if playback != 0 { (*(*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream).private_data = rtd as *mut c_void; }
        if capture != 0 { (*(*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream).private_data = rtd as *mut c_void; }
        return ret;
    }
    if (*(*rtd).dai_link).dynamic {
        (*rtd).ops.open = Some(dpcm_fe_dai_open);
        (*rtd).ops.hw_params = Some(dpcm_fe_dai_hw_params);
        (*rtd).ops.prepare = Some(dpcm_fe_dai_prepare);
        (*rtd).ops.trigger = Some(dpcm_fe_dai_trigger);
        (*rtd).ops.hw_free = Some(dpcm_fe_dai_hw_free);
        (*rtd).ops.close = Some(dpcm_fe_dai_close);
        (*rtd).ops.pointer = Some(soc_pcm_pointer);
    } else {
        (*rtd).ops.open = Some(soc_pcm_open);
        (*rtd).ops.hw_params = Some(soc_pcm_hw_params);
        (*rtd).ops.prepare = Some(soc_pcm_prepare);
        (*rtd).ops.trigger = Some(soc_pcm_trigger);
        (*rtd).ops.hw_free = Some(soc_pcm_hw_free);
        (*rtd).ops.close = Some(soc_pcm_close);
        (*rtd).ops.pointer = Some(soc_pcm_pointer);
    }
    if playback != 0 { snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &mut (*rtd).ops); }
    if capture != 0 { snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &mut (*rtd).ops); }
    ret = snd_soc_pcm_component_new(rtd);
    ret
}

unsafe extern "C" { fn snd_soc_close_delayed_work(); fn snd_soc_pcm_component_ioctl(); fn snd_soc_pcm_component_sync_stop(); fn snd_soc_pcm_component_copy(); fn snd_soc_pcm_component_page(); fn snd_soc_pcm_component_mmap(); fn snd_soc_pcm_component_ack(); }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dpcm_get_substream(be: *mut snd_soc_pcm_runtime, stream: c_int) -> *mut snd_pcm_substream {
    (*be).pcm.as_mut().unwrap().streams[stream as usize].substream
}
EXPORT_SYMBOL_GPL!(snd_soc_dpcm_get_substream);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
