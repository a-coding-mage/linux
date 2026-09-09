/* SPDX-License-Identifier: GPL-2.0
 *
 * Rust translation of soc-component.h.
 */

/* Dependencies supplied by the surrounding translation unit. */

pub const SND_SOC_COMP_ORDER_FIRST: i32 = -2;
pub const SND_SOC_COMP_ORDER_EARLY: i32 = -1;
pub const SND_SOC_COMP_ORDER_NORMAL: i32 = 0;
pub const SND_SOC_COMP_ORDER_LATE: i32 = 1;
pub const SND_SOC_COMP_ORDER_LAST: i32 = 2;

#[repr(C)]
pub struct snd_compress_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> i32>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> i32>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_codec) -> i32>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> i32>,
    pub get_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, i32) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> i32>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut i8, usize) -> i32>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut vm_area_struct) -> i32>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, usize) -> i32>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> i32>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> i32>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const i8,
    pub controls: *const snd_kcontrol_new, pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: u32,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub fixup_controls: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut snd_soc_component, u32) -> u32>,
    pub write: Option<unsafe extern "C" fn(*mut snd_soc_component, u32, u32) -> i32>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> i32>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm)>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, i32, u32, i32) -> i32>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, i32, i32, u32, u32) -> i32>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut core::ffi::c_void) -> i32>,
    pub get_jack_type: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub of_xlate_dai_name: Option<unsafe extern "C" fn(*mut snd_soc_component, *const of_phandle_args, *mut *const i8) -> i32>,
    pub of_xlate_dai_id: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut device_node) -> i32>,
    pub seq_notifier: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_dapm_type, i32)>,
    pub stream_event: Option<unsafe extern "C" fn(*mut snd_soc_component, i32) -> i32>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, u32, *mut core::ffi::c_void) -> i32>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> i32>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, i32) -> i32>,
    pub sync_stop: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub get_time_info: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut timespec64, *mut timespec64, *mut snd_pcm_audio_tstamp_config, *mut snd_pcm_audio_tstamp_report) -> i32>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, i32, usize, *mut iov_iter, usize) -> i32>,
    pub page: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, usize) -> *mut page>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut vm_area_struct) -> i32>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> i32>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    pub compress_ops: *const snd_compress_ops,
    pub probe_order: i32, pub remove_order: i32,
    pub trigger_start: snd_soc_trigger_order, pub trigger_stop: snd_soc_trigger_order,
    pub module_get_upon_open: u32, pub idle_bias_on: u32, pub suspend_bias_off: u32,
    pub use_pmdown_time: u32, pub endianness: u32, pub legacy_dai_naming: u32,
    pub ignore_machine: *const i8, pub topology_name_prefix: *const i8,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> i32>,
    pub use_dai_pcm_id: bool, pub be_pcm_base: i32, pub debugfs_prefix: *const i8,
}

#[repr(C)]
pub struct snd_soc_component {
    pub name: *const i8, pub name_prefix: *const i8, pub dev: *mut device, pub card: *mut snd_soc_card,
    pub active: u32, pub suspended: u32,
    pub list: list_head, pub card_aux_list: list_head, pub card_list: list_head,
    pub card_device_link: *mut device_link, pub driver: *const snd_soc_component_driver,
    pub dai_list: list_head, pub num_dai: i32, pub regmap: *mut regmap, pub io_mutex: mutex,
    pub dobj_list: list_head, pub dapm: *mut snd_soc_dapm_context,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub mark_module: *mut core::ffi::c_void, pub mark_open: *mut snd_pcm_substream,
    pub mark_hw_params: *mut snd_pcm_substream, pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_open: *mut snd_compr_stream, pub mark_pm: *mut core::ffi::c_void,
    pub debugfs_root: *mut dentry, pub priv_: *mut core::ffi::c_void,
}

macro_rules! for_each_comp_order { ($order:ident, $body:block) => { for $order in SND_SOC_COMP_ORDER_FIRST..=SND_SOC_COMP_ORDER_LAST $body }; }

pub unsafe fn snd_soc_component_to_dapm(c: *mut snd_soc_component) -> *mut snd_soc_dapm_context { (*c).dapm }
pub unsafe fn snd_soc_component_cache_sync(c: *mut snd_soc_component) -> i32 { regcache_sync((*c).regmap) }

extern "C" {
    pub fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    pub fn snd_soc_component_set_name(c: *mut snd_soc_component, name: *const i8);
    pub fn snd_soc_component_name(c: *mut snd_soc_component) -> *const i8;
    pub fn snd_soc_component_set_priv(c: *mut snd_soc_component, p: *mut core::ffi::c_void);
    pub fn snd_soc_component_to_priv(c: *mut snd_soc_component) -> *mut core::ffi::c_void;
    pub fn snd_soc_component_set_aux(c: *mut snd_soc_component, aux: *mut snd_soc_aux_dev);
    pub fn snd_soc_component_init(c: *mut snd_soc_component) -> i32;
    pub fn snd_soc_component_is_dummy(c: *mut snd_soc_component) -> i32;
    pub fn snd_soc_component_read(c: *mut snd_soc_component, reg: u32) -> u32;
    pub fn snd_soc_component_write(c: *mut snd_soc_component, reg: u32, val: u32) -> i32;
    pub fn snd_soc_component_update_bits(c: *mut snd_soc_component, reg: u32, mask: u32, val: u32) -> i32;
    pub fn snd_soc_component_update_bits_async(c: *mut snd_soc_component, reg: u32, mask: u32, val: u32) -> i32;
    pub fn snd_soc_component_async_complete(c: *mut snd_soc_component);
    pub fn snd_soc_component_test_bits(c: *mut snd_soc_component, reg: u32, mask: u32, value: u32) -> i32;
    pub fn snd_soc_component_read_field(c: *mut snd_soc_component, reg: u32, mask: u32) -> u32;
    pub fn snd_soc_component_write_field(c: *mut snd_soc_component, reg: u32, mask: u32, val: u32) -> i32;
    pub fn snd_soc_component_set_sysclk(c: *mut snd_soc_component, clk_id: i32, source: i32, freq: u32, dir: i32) -> i32;
    pub fn snd_soc_component_set_pll(c: *mut snd_soc_component, pll_id: i32, source: i32, freq_in: u32, freq_out: u32) -> i32;
    pub fn snd_soc_component_set_jack(c: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut core::ffi::c_void) -> i32;
    pub fn snd_soc_component_get_jack_type(c: *mut snd_soc_component) -> i32;
    pub fn snd_soc_component_seq_notifier(c: *mut snd_soc_component, typ: snd_soc_dapm_type, subseq: i32);
    pub fn snd_soc_component_stream_event(c: *mut snd_soc_component, event: i32) -> i32;
    pub fn snd_soc_component_set_bias_level(c: *mut snd_soc_component, level: snd_soc_bias_level) -> i32;
    pub fn snd_soc_component_regmap_val_bytes(c: *mut snd_soc_component) -> i32;
    pub fn snd_soc_component_module_get(c: *mut snd_soc_component, mark: *mut core::ffi::c_void, upon_open: i32) -> i32;
    pub fn snd_soc_component_module_put(c: *mut snd_soc_component, mark: *mut core::ffi::c_void, upon_open: i32, rollback: i32);
    pub fn snd_soc_component_get_kcontrol(c: *mut snd_soc_component, ctl: *const i8) -> *mut snd_kcontrol;
    pub fn snd_soc_component_notify_control(c: *mut snd_soc_component, ctl: *const i8) -> i32;
    pub fn snd_soc_component_open(c: *mut snd_soc_component, s: *mut snd_pcm_substream) -> i32;
    pub fn snd_soc_component_close(c: *mut snd_soc_component, s: *mut snd_pcm_substream, rollback: i32) -> i32;
    pub fn snd_soc_component_suspend(c: *mut snd_soc_component); pub fn snd_soc_component_resume(c: *mut snd_soc_component);
    pub fn snd_soc_component_is_suspended(c: *mut snd_soc_component) -> i32; pub fn snd_soc_component_probe(c: *mut snd_soc_component) -> i32;
    pub fn snd_soc_component_fixup_controls(c: *mut snd_soc_component) -> i32; pub fn snd_soc_component_remove(c: *mut snd_soc_component);
    pub fn snd_soc_component_of_xlate_dai_id(c: *mut snd_soc_component, ep: *mut device_node) -> i32;
    pub fn snd_soc_component_of_xlate_dai_name(c: *mut snd_soc_component, args: *const of_phandle_args, name: *mut *const i8) -> i32;
    pub fn snd_soc_component_compr_open(c: *mut snd_soc_component, s: *mut snd_compr_stream) -> i32;
    pub fn snd_soc_component_compr_free(c: *mut snd_soc_component, s: *mut snd_compr_stream, rollback: i32);
    pub fn snd_soc_component_compr_trigger(s: *mut snd_compr_stream, cmd: i32) -> i32;
    pub fn snd_soc_component_compr_set_params(s: *mut snd_compr_stream, p: *mut snd_compr_params) -> i32;
    pub fn snd_soc_component_compr_get_params(s: *mut snd_compr_stream, p: *mut snd_codec) -> i32;
    pub fn snd_soc_component_compr_get_caps(s: *mut snd_compr_stream, p: *mut snd_compr_caps) -> i32;
    pub fn snd_soc_component_compr_get_codec_caps(s: *mut snd_compr_stream, p: *mut snd_compr_codec_caps) -> i32;
    pub fn snd_soc_component_compr_ack(s: *mut snd_compr_stream, bytes: usize) -> i32;
    pub fn snd_soc_component_compr_pointer(s: *mut snd_compr_stream, t: *mut snd_compr_tstamp64) -> i32;
    pub fn snd_soc_component_compr_copy(s: *mut snd_compr_stream, buf: *mut i8, count: usize) -> i32;
    pub fn snd_soc_component_compr_set_metadata(s: *mut snd_compr_stream, m: *mut snd_compr_metadata) -> i32;
    pub fn snd_soc_component_compr_get_metadata(s: *mut snd_compr_stream, m: *mut snd_compr_metadata) -> i32;
    pub fn snd_soc_pcm_component_pointer(s: *mut snd_pcm_substream) -> i32;
    pub fn snd_soc_pcm_component_ioctl(s: *mut snd_pcm_substream, cmd: u32, arg: *mut core::ffi::c_void) -> i32;
    pub fn snd_soc_pcm_component_sync_stop(s: *mut snd_pcm_substream) -> i32;
    pub fn snd_soc_pcm_component_copy(s: *mut snd_pcm_substream, channel: i32, pos: usize, iter: *mut iov_iter, bytes: usize) -> i32;
    pub fn snd_soc_pcm_component_page(s: *mut snd_pcm_substream, offset: usize) -> *mut page;
    pub fn snd_soc_pcm_component_mmap(s: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> i32;
    pub fn snd_soc_pcm_component_new(rtd: *mut snd_soc_pcm_runtime) -> i32;
    pub fn snd_soc_pcm_component_free(rtd: *mut snd_soc_pcm_runtime);
    pub fn snd_soc_pcm_component_prepare(s: *mut snd_pcm_substream) -> i32;
    pub fn snd_soc_pcm_component_hw_params(s: *mut snd_pcm_substream, p: *mut snd_pcm_hw_params) -> i32;
    pub fn snd_soc_pcm_component_hw_free(s: *mut snd_pcm_substream, rollback: i32);
    pub fn snd_soc_pcm_component_trigger(s: *mut snd_pcm_substream, cmd: i32, rollback: i32) -> i32;
    pub fn snd_soc_pcm_component_pm_runtime_get(rtd: *mut snd_soc_pcm_runtime, stream: *mut core::ffi::c_void) -> i32;
    pub fn snd_soc_pcm_component_pm_runtime_put(rtd: *mut snd_soc_pcm_runtime, stream: *mut core::ffi::c_void, rollback: i32);
    pub fn snd_soc_pcm_component_ack(s: *mut snd_pcm_substream) -> i32;
    pub fn snd_soc_pcm_component_delay(s: *mut snd_pcm_substream, cpu_delay: *mut snd_pcm_sframes_t, codec_delay: *mut snd_pcm_sframes_t);
}

#[inline]
pub unsafe fn snd_soc_component_module_get_when_probe(c: *mut snd_soc_component) -> i32 {
    snd_soc_component_module_get(c, core::ptr::null_mut(), 0)
}
#[inline]
pub unsafe fn snd_soc_component_module_get_when_open(c: *mut snd_soc_component, s: *mut snd_pcm_substream) -> i32 {
    snd_soc_component_module_get(c, s.cast(), 1)
}
#[inline]
pub unsafe fn snd_soc_component_module_put_when_remove(c: *mut snd_soc_component) {
    snd_soc_component_module_put(c, core::ptr::null_mut(), 0, 0)
}
#[inline]
pub unsafe fn snd_soc_component_module_put_when_close(c: *mut snd_soc_component, s: *mut snd_pcm_substream, rollback: i32) {
    snd_soc_component_module_put(c, s.cast(), 1, rollback)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
