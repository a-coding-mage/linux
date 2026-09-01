// SPDX-License-Identifier: GPL-2.0
//
// soc-component.c
//
// Copyright 2009-2011 Wolfson Microelectronics PLC.
// Copyright (C) 2019 Renesas Electronics Corp.
//
// Mark Brown <broonie@opensource.wolfsonmicro.com>
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Translated from C. Kernel, ALSA, ASoC, regmap, PM runtime, and device
// definitions are external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type snd_pcm_sframes_t = isize;

const ENOTSUPP: c_int = 524;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const EIO: c_int = 5;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: usize = 44;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1 << 0;

extern "C" {
    static __func__: c_char;

    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_card_get_kcontrol(card: *mut snd_soc_card, name: *const c_char) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn regmap_get_val_bytes(map: *mut regmap) -> c_int;
    fn regmap_exit(map: *mut regmap);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut bool_,
    ) -> c_int;
    fn regmap_update_bits_check_async(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut bool_,
    ) -> c_int;
    fn regmap_async_complete(map: *mut regmap);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn ffs(x: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dpcm_mutex_lock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dpcm_mutex_unlock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_pcm_lib_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_noidle(dev: *mut device);
    fn pm_runtime_put_autosuspend(dev: *mut device);
}

#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { _private: [u8; 0] }
#[repr(C)] pub struct snd_ctl_elem_id { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_aux_dev { pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> }
#[repr(C)] pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_stream { pub private_data: *mut snd_soc_pcm_runtime }
#[repr(C)] pub struct snd_compr_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_codec { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_caps { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_codec_caps { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_tstamp64 { _private: [u8; 0] }
#[repr(C)] pub struct snd_compr_metadata { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }

#[repr(C)] pub struct device_driver { pub owner: *mut module }
#[repr(C)] pub struct device { pub driver: *mut device_driver }
#[repr(C)] pub struct snd_kcontrol { pub id: snd_ctl_elem_id }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }

#[repr(C)] pub enum snd_soc_dapm_type { SND_SOC_DAPM_TYPE_DUMMY = 0 }
#[repr(C)] pub enum snd_soc_bias_level { SND_SOC_BIAS_OFF = 0 }

#[repr(C)]
pub struct snd_soc_compress_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, c_int) -> c_int>,
    pub set_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_params) -> c_int>,
    pub get_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_codec) -> c_int>,
    pub get_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_caps) -> c_int>,
    pub get_codec_caps: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_codec_caps) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, size_t) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_tstamp64) -> c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut c_char, size_t) -> c_int>,
    pub set_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
    pub get_metadata: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_compr_stream, *mut snd_compr_metadata) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub seq_notifier: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_dapm_type, c_int)>,
    pub stream_event: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub get_jack_type: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub module_get_upon_open: bool_,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub fixup_controls: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub of_xlate_dai_id: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut device_node) -> c_int>,
    pub of_xlate_dai_name: Option<unsafe extern "C" fn(*mut snd_soc_component, *const of_phandle_args, *mut *const c_char) -> c_int>,
    pub compress_ops: *mut snd_soc_compress_ops,
    pub read: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint) -> c_uint>,
    pub write: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint, c_uint) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub delay: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_sframes_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_uint, *mut c_void) -> c_int>,
    pub sync_stop: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub copy: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int, c_ulong, *mut iov_iter, c_ulong) -> c_int>,
    pub page: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_ulong) -> *mut page>,
    pub mmap: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut vm_area_struct) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut c_void)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pub ack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub name: *const c_char,
    pub priv_: *mut c_void,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub driver: *mut snd_soc_component_driver,
    pub name_prefix: *const c_char,
    pub card: *mut snd_soc_card,
    pub regmap: *mut regmap,
    pub io_mutex: mutex,
    pub suspended: c_int,
    pub mark_module: *mut c_void,
    pub mark_open: *mut c_void,
    pub mark_compr_open: *mut c_void,
    pub mark_hw_params: *mut c_void,
    pub mark_trigger: *mut c_void,
    pub mark_pm: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub num_components: c_int,
    pub components: *mut *mut snd_soc_component,
    pub num_codecs: c_int,
    pub codec_dais: *mut *mut snd_soc_dai,
    pub pcm: *mut c_void,
}

unsafe fn _soc_component_ret(component: *mut snd_soc_component, func: *const c_char, ret: c_int) -> c_int {
    snd_soc_ret((*component).dev, ret, c"at %s() on %s\n".as_ptr(), func, (*component).name)
}

unsafe fn soc_component_ret(component: *mut snd_soc_component, ret: c_int) -> c_int {
    _soc_component_ret(component, &__func__, ret)
}

unsafe fn _soc_component_ret_reg_rw(component: *mut snd_soc_component, func: *const c_char, ret: c_int, reg: c_int) -> c_int {
    snd_soc_ret((*component).dev, ret, c"at %s() on %s for register: [0x%08x]\n".as_ptr(), func, (*component).name, reg)
}

unsafe fn soc_component_ret_reg_rw(component: *mut snd_soc_component, ret: c_int, reg: c_uint) -> c_int {
    _soc_component_ret_reg_rw(component, &__func__, ret, reg as c_int)
}

macro_rules! rtd_components {
    ($rtd:expr) => {
        0..(*$rtd).num_components
    };
}

unsafe fn rtd_component(rtd: *mut snd_soc_pcm_runtime, i: c_int) -> *mut snd_soc_component {
    *(*rtd).components.offset(i as isize)
}

unsafe fn rtd_codec_dai(rtd: *mut snd_soc_pcm_runtime, i: c_int) -> *mut snd_soc_dai {
    *(*rtd).codec_dais.offset(i as isize)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component {
    let component = devm_kzalloc(dev, core::mem::size_of::<snd_soc_component>(), GFP_KERNEL) as *mut snd_soc_component;
    if component.is_null() { return ptr::null_mut(); }
    (*component).dev = dev;
    component
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_component_set_name(component: *mut snd_soc_component, name: *const c_char) { (*component).name = name; }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_name(component: *mut snd_soc_component) -> *const c_char { (*component).name }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_set_priv(component: *mut snd_soc_component, priv_: *mut c_void) { (*component).priv_ = priv_; }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_to_priv(component: *mut snd_soc_component) -> *mut c_void { (*component).priv_ }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_set_aux(component: *mut snd_soc_component, aux: *mut snd_soc_aux_dev) {
    (*component).init = if !aux.is_null() { (*aux).init } else { None };
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_init(component: *mut snd_soc_component) -> c_int {
    let mut ret = 0;
    if let Some(init) = (*component).init { ret = init(component); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_set_sysclk(component: *mut snd_soc_component, clk_id: c_int, source: c_int, freq: c_uint, dir: c_int) -> c_int {
    let mut ret = -ENOTSUPP;
    if let Some(set_sysclk) = (*(*component).driver).set_sysclk { ret = set_sysclk(component, clk_id, source, freq, dir); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_set_pll(component: *mut snd_soc_component, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    let mut ret = -EINVAL;
    if let Some(set_pll) = (*(*component).driver).set_pll { ret = set_pll(component, pll_id, source, freq_in, freq_out); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_seq_notifier(component: *mut snd_soc_component, type_: snd_soc_dapm_type, subseq: c_int) {
    if let Some(seq_notifier) = (*(*component).driver).seq_notifier { seq_notifier(component, type_, subseq); }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_stream_event(component: *mut snd_soc_component, event: c_int) -> c_int {
    let mut ret = 0;
    if let Some(stream_event) = (*(*component).driver).stream_event { ret = stream_event(component, event); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let mut ret = 0;
    if let Some(set_bias_level) = (*(*component).driver).set_bias_level { ret = set_bias_level(component, level); }
    soc_component_ret(component, ret)
}

unsafe fn soc_get_kcontrol_name(component: *mut snd_soc_component, buf: *mut c_char, size: c_int, ctl: *const c_char) {
    /* When updating, change also snd_soc_dapm_widget_name_cmp() */
    if !(*component).name_prefix.is_null() {
        snprintf(buf, size as size_t, c"%s %s".as_ptr(), (*component).name_prefix, ctl);
    } else {
        snprintf(buf, size as size_t, c"%s".as_ptr(), ctl);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_get_kcontrol(component: *mut snd_soc_component, ctl: *const c_char) -> *mut snd_kcontrol {
    let mut name = [0 as c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN];
    soc_get_kcontrol_name(component, name.as_mut_ptr(), name.len() as c_int, ctl);
    snd_soc_card_get_kcontrol((*component).card, name.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_notify_control(component: *mut snd_soc_component, ctl: *const c_char) -> c_int {
    let kctl = snd_soc_component_get_kcontrol(component, ctl);
    if kctl.is_null() { return soc_component_ret(component, -EINVAL); }
    snd_ctl_notify((*(*component).card).snd_card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int {
    let mut ret = -ENOTSUPP;
    if let Some(set_jack) = (*(*component).driver).set_jack { ret = set_jack(component, jack, data); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_get_jack_type(component: *mut snd_soc_component) -> c_int {
    let mut ret = -ENOTSUPP;
    if let Some(get_jack_type) = (*(*component).driver).get_jack_type { ret = get_jack_type(component); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_module_get(component: *mut snd_soc_component, mark: *mut c_void, upon_open: c_int) -> c_int {
    let mut ret = 0;
    if (*(*component).driver).module_get_upon_open == (upon_open != 0) && try_module_get((*(*(*component).dev).driver).owner) == 0 {
        ret = -ENODEV;
    }
    if ret == 0 { (*component).mark_module = mark; }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_module_put(component: *mut snd_soc_component, mark: *mut c_void, upon_open: c_int, rollback: c_int) {
    if rollback != 0 && (*component).mark_module != mark { return; }
    if (*(*component).driver).module_get_upon_open == (upon_open != 0) { module_put((*(*(*component).dev).driver).owner); }
    (*component).mark_module = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_open(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let mut ret = 0;
    if let Some(open) = (*(*component).driver).open { ret = open(component, substream); }
    if ret == 0 { (*component).mark_open = substream as *mut c_void; }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_close(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, rollback: c_int) -> c_int {
    let mut ret = 0;
    if rollback != 0 && (*component).mark_open != substream as *mut c_void { return 0; }
    if let Some(close) = (*(*component).driver).close { ret = close(component, substream); }
    (*component).mark_open = ptr::null_mut();
    soc_component_ret(component, ret)
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_component_suspend(component: *mut snd_soc_component) { if let Some(suspend) = (*(*component).driver).suspend { suspend(component); } (*component).suspended = 1; }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_resume(component: *mut snd_soc_component) { if let Some(resume) = (*(*component).driver).resume { resume(component); } (*component).suspended = 0; }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_is_suspended(component: *mut snd_soc_component) -> c_int { (*component).suspended }

#[no_mangle] pub unsafe extern "C" fn snd_soc_component_probe(component: *mut snd_soc_component) -> c_int { let mut ret = 0; if let Some(probe) = (*(*component).driver).probe { ret = probe(component); } soc_component_ret(component, ret) }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_fixup_controls(component: *mut snd_soc_component) -> c_int { let mut ret = 0; if let Some(fixup_controls) = (*(*component).driver).fixup_controls { ret = fixup_controls(component); } soc_component_ret(component, ret) }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_remove(component: *mut snd_soc_component) { if let Some(remove) = (*(*component).driver).remove { remove(component); } }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_of_xlate_dai_id(component: *mut snd_soc_component, ep: *mut device_node) -> c_int {
    let mut ret = -ENOTSUPP;
    if let Some(of_xlate_dai_id) = (*(*component).driver).of_xlate_dai_id { ret = of_xlate_dai_id(component, ep); }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_of_xlate_dai_name(component: *mut snd_soc_component, args: *const of_phandle_args, dai_name: *mut *const c_char) -> c_int {
    if let Some(of_xlate_dai_name) = (*(*component).driver).of_xlate_dai_name { return of_xlate_dai_name(component, args, dai_name); }
    /*
     * Don't use soc_component_ret here because we may not want to report
     * the error just yet. If a device has more than one component, the
     * first may not match and we don't want spam the log with this.
     */
    -ENOTSUPP
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_regmap_val_bytes(component: *mut snd_soc_component) -> c_int {
    if (*component).regmap.is_null() { return 0; }
    let val_bytes = regmap_get_val_bytes((*component).regmap);
    if val_bytes < 0 { return 0; }
    val_bytes
}

// Original C gated these by CONFIG_REGMAP.
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap) { (*component).regmap = regmap; }
#[no_mangle] pub unsafe extern "C" fn snd_soc_component_exit_regmap(component: *mut snd_soc_component) { regmap_exit((*component).regmap); (*component).regmap = ptr::null_mut(); }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_open(component: *mut snd_soc_component, cstream: *mut snd_compr_stream) -> c_int {
    let mut ret = 0;
    let ops = (*(*component).driver).compress_ops;
    if !ops.is_null() {
        if let Some(open) = (*ops).open { ret = open(component, cstream); }
    }
    if ret == 0 { (*component).mark_compr_open = cstream as *mut c_void; }
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_free(component: *mut snd_soc_component, cstream: *mut snd_compr_stream, rollback: c_int) {
    if rollback != 0 && (*component).mark_compr_open != cstream as *mut c_void { return; }
    let ops = (*(*component).driver).compress_ops;
    if !ops.is_null() {
        if let Some(free) = (*ops).free { free(component, cstream); }
    }
    (*component).mark_compr_open = ptr::null_mut();
}

unsafe fn with_compress_ops(component: *mut snd_soc_component) -> *mut snd_soc_compress_ops {
    (*(*component).driver).compress_ops
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_trigger(cstream: *mut snd_compr_stream, cmd: c_int) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(trigger) = (*ops).trigger {
                let ret = trigger(component, cstream, cmd);
                if ret < 0 { return soc_component_ret(component, ret); }
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_set_params(cstream: *mut snd_compr_stream, params: *mut snd_compr_params) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(set_params) = (*ops).set_params {
                let ret = set_params(component, cstream, params);
                if ret < 0 { return soc_component_ret(component, ret); }
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_get_params(cstream: *mut snd_compr_stream, params: *mut snd_codec) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(get_params) = (*ops).get_params {
                let ret = get_params(component, cstream, params);
                return soc_component_ret(component, ret);
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_get_caps(cstream: *mut snd_compr_stream, caps: *mut snd_compr_caps) -> c_int {
    let rtd = (*cstream).private_data;
    let mut component: *mut snd_soc_component = ptr::null_mut();
    let mut ret = 0;
    snd_soc_dpcm_mutex_lock(rtd);
    for i in rtd_components!(rtd) {
        component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(get_caps) = (*ops).get_caps {
                ret = get_caps(component, cstream, caps);
                break;
            }
        }
    }
    snd_soc_dpcm_mutex_unlock(rtd);
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_get_codec_caps(cstream: *mut snd_compr_stream, codec: *mut snd_compr_codec_caps) -> c_int {
    let rtd = (*cstream).private_data;
    let mut component: *mut snd_soc_component = ptr::null_mut();
    let mut ret = 0;
    snd_soc_dpcm_mutex_lock(rtd);
    for i in rtd_components!(rtd) {
        component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(get_codec_caps) = (*ops).get_codec_caps {
                ret = get_codec_caps(component, cstream, codec);
                break;
            }
        }
    }
    snd_soc_dpcm_mutex_unlock(rtd);
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_ack(cstream: *mut snd_compr_stream, bytes: size_t) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(ack) = (*ops).ack {
                let ret = ack(component, cstream, bytes);
                if ret < 0 { return soc_component_ret(component, ret); }
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_pointer(cstream: *mut snd_compr_stream, tstamp: *mut snd_compr_tstamp64) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(pointer) = (*ops).pointer {
                let ret = pointer(component, cstream, tstamp);
                return soc_component_ret(component, ret);
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_copy(cstream: *mut snd_compr_stream, buf: *mut c_char, count: size_t) -> c_int {
    let rtd = (*cstream).private_data;
    let mut component: *mut snd_soc_component = ptr::null_mut();
    let mut ret = 0;
    snd_soc_dpcm_mutex_lock(rtd);
    for i in rtd_components!(rtd) {
        component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(copy) = (*ops).copy {
                ret = copy(component, cstream, buf, count);
                break;
            }
        }
    }
    snd_soc_dpcm_mutex_unlock(rtd);
    soc_component_ret(component, ret)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_set_metadata(cstream: *mut snd_compr_stream, metadata: *mut snd_compr_metadata) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(set_metadata) = (*ops).set_metadata {
                let ret = set_metadata(component, cstream, metadata);
                if ret < 0 { return soc_component_ret(component, ret); }
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_compr_get_metadata(cstream: *mut snd_compr_stream, metadata: *mut snd_compr_metadata) -> c_int {
    let rtd = (*cstream).private_data;
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ops = with_compress_ops(component);
        if !ops.is_null() {
            if let Some(get_metadata) = (*ops).get_metadata {
                let ret = get_metadata(component, cstream, metadata);
                return soc_component_ret(component, ret);
            }
        }
    }
    0
}

unsafe fn soc_component_read_no_lock(component: *mut snd_soc_component, reg: c_uint) -> c_uint {
    let mut val: c_uint = 0;
    let ret: c_int;
    if !(*component).regmap.is_null() {
        ret = regmap_read((*component).regmap, reg, &mut val);
    } else if let Some(read) = (*(*component).driver).read {
        ret = 0;
        val = read(component, reg);
    } else {
        ret = -EIO;
    }
    if ret < 0 { return soc_component_ret_reg_rw(component, ret, reg) as c_uint; }
    val
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint {
    mutex_lock(&mut (*component).io_mutex);
    let val = soc_component_read_no_lock(component, reg);
    mutex_unlock(&mut (*component).io_mutex);
    val
}

unsafe fn soc_component_write_no_lock(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int {
    let mut ret = -EIO;
    if !(*component).regmap.is_null() {
        ret = regmap_write((*component).regmap, reg, val);
    } else if let Some(write) = (*(*component).driver).write {
        ret = write(component, reg, val);
    }
    soc_component_ret_reg_rw(component, ret, reg)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int {
    mutex_lock(&mut (*component).io_mutex);
    let ret = soc_component_write_no_lock(component, reg, val);
    mutex_unlock(&mut (*component).io_mutex);
    ret
}

unsafe fn snd_soc_component_update_bits_legacy(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint, change: *mut bool_) -> c_int {
    let mut ret = 0;
    mutex_lock(&mut (*component).io_mutex);
    let old = soc_component_read_no_lock(component, reg);
    let new = (old & !mask) | (val & mask);
    *change = old != new;
    if *change { ret = soc_component_write_no_lock(component, reg, new); }
    mutex_unlock(&mut (*component).io_mutex);
    soc_component_ret_reg_rw(component, ret, reg)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let mut change = false;
    let ret = if !(*component).regmap.is_null() {
        regmap_update_bits_check((*component).regmap, reg, mask, val, &mut change)
    } else {
        snd_soc_component_update_bits_legacy(component, reg, mask, val, &mut change)
    };
    if ret < 0 { return soc_component_ret_reg_rw(component, ret, reg); }
    change as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_update_bits_async(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int {
    let mut change = false;
    let ret = if !(*component).regmap.is_null() {
        regmap_update_bits_check_async((*component).regmap, reg, mask, val, &mut change)
    } else {
        snd_soc_component_update_bits_legacy(component, reg, mask, val, &mut change)
    };
    if ret < 0 { return soc_component_ret_reg_rw(component, ret, reg); }
    change as c_int
}

unsafe fn soc_component_field_shift(component: *mut snd_soc_component, mask: c_uint) -> c_int {
    if mask == 0 {
        dev_err((*component).dev, c"ASoC: error field mask is zero for %s\n".as_ptr(), (*component).name);
        return 0;
    }
    ffs(mask as c_int) - 1
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_read_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint) -> c_uint {
    let mut val = snd_soc_component_read(component, reg);
    val = (val & mask) >> soc_component_field_shift(component, mask);
    val
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_write_field(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, mut val: c_uint) -> c_int {
    val = (val << soc_component_field_shift(component, mask)) & mask;
    snd_soc_component_update_bits(component, reg, mask, val)
}

#[no_mangle] pub unsafe extern "C" fn snd_soc_component_async_complete(component: *mut snd_soc_component) { if !(*component).regmap.is_null() { regmap_async_complete((*component).regmap); } }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_component_test_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, value: c_uint) -> c_int {
    let old = snd_soc_component_read(component, reg);
    let new = (old & !mask) | value;
    (old != new) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_pointer(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME: use 1st pointer */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(pointer) = (*(*component).driver).pointer { return pointer(component, substream); }
    }
    0
}

unsafe fn snd_soc_component_is_codec_on_rtd(rtd: *mut snd_soc_pcm_runtime, component: *mut snd_soc_component) -> bool_ {
    for i in 0..(*rtd).num_codecs {
        let dai = rtd_codec_dai(rtd, i);
        if (*dai).component == component { return true; }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_delay(substream: *mut snd_pcm_substream, cpu_delay: *mut snd_pcm_sframes_t, codec_delay: *mut snd_pcm_sframes_t) {
    let rtd = snd_soc_substream_to_rtd(substream);
    /*
     * We're looking for the delay through the full audio path so it needs to
     * be the maximum of the Components doing transmit and the maximum of the
     * Components doing receive (ie, all CPUs and all CODECs) rather than
     * just the maximum of all Components.
     */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(delay_fn) = (*(*component).driver).delay {
            let delay = delay_fn(component, substream);
            if snd_soc_component_is_codec_on_rtd(rtd, component) {
                *codec_delay = core::cmp::max(*codec_delay, delay);
            } else {
                *cpu_delay = core::cmp::max(*cpu_delay, delay);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME: use 1st ioctl */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(ioctl) = (*(*component).driver).ioctl {
            return soc_component_ret(component, ioctl(component, substream, cmd, arg));
        }
    }
    snd_pcm_lib_ioctl(substream, cmd, arg)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_sync_stop(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(sync_stop) = (*(*component).driver).sync_stop {
            let ret = sync_stop(component, substream);
            if ret < 0 { return soc_component_ret(component, ret); }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_copy(substream: *mut snd_pcm_substream, channel: c_int, pos: c_ulong, iter: *mut iov_iter, bytes: c_ulong) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME. it returns 1st copy now */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(copy) = (*(*component).driver).copy {
            return soc_component_ret(component, copy(component, substream, channel, pos, iter, bytes));
        }
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_page(substream: *mut snd_pcm_substream, offset: c_ulong) -> *mut page {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME. it returns 1st page now */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(page_fn) = (*(*component).driver).page {
            let page = page_fn(component, substream, offset);
            if !page.is_null() { return page; }
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_mmap(substream: *mut snd_pcm_substream, vma: *mut vm_area_struct) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME. it returns 1st mmap now */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(mmap) = (*(*component).driver).mmap {
            return soc_component_ret(component, mmap(component, substream, vma));
        }
    }
    -EINVAL
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_new(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(pcm_new) = (*(*component).driver).pcm_new {
            let ret = pcm_new(component, rtd);
            if ret < 0 { return soc_component_ret(component, ret); }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_free(rtd: *mut snd_soc_pcm_runtime) {
    if (*rtd).pcm.is_null() { return; }
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(pcm_free) = (*(*component).driver).pcm_free { pcm_free(component, (*rtd).pcm); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(prepare) = (*(*component).driver).prepare {
            let ret = prepare(component, substream);
            if ret < 0 { return soc_component_ret(component, ret); }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(hw_params) = (*(*component).driver).hw_params {
            let ret = hw_params(component, substream, params);
            if ret < 0 { return soc_component_ret(component, ret); }
        }
        (*component).mark_hw_params = substream as *mut c_void;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_hw_free(substream: *mut snd_pcm_substream, rollback: c_int) {
    let rtd = snd_soc_substream_to_rtd(substream);
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if rollback != 0 && (*component).mark_hw_params != substream as *mut c_void { continue; }
        if let Some(hw_free) = (*(*component).driver).hw_free {
            let ret = hw_free(component, substream);
            if ret < 0 { soc_component_ret(component, ret); }
        }
        (*component).mark_hw_params = ptr::null_mut();
    }
}

unsafe fn soc_component_trigger(component: *mut snd_soc_component, substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let mut ret = 0;
    if let Some(trigger) = (*(*component).driver).trigger { ret = trigger(component, substream, cmd); }
    soc_component_ret(component, ret)
}

extern "C" {
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_trigger(substream: *mut snd_pcm_substream, cmd: c_int, rollback: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut ret = 0;
    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE {
        for i in rtd_components!(rtd) {
            let component = rtd_component(rtd, i);
            ret = soc_component_trigger(component, substream, cmd);
            if ret < 0 { break; }
            (*component).mark_trigger = substream as *mut c_void;
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        for i in rtd_components!(rtd) {
            let component = rtd_component(rtd, i);
            if rollback != 0 && (*component).mark_trigger != substream as *mut c_void { continue; }
            let r = soc_component_trigger(component, substream, cmd);
            if r < 0 { ret = r; /* use last ret */ }
            (*component).mark_trigger = ptr::null_mut();
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_pm_runtime_get(rtd: *mut snd_soc_pcm_runtime, stream: *mut c_void) -> c_int {
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        let ret = pm_runtime_get_sync((*component).dev);
        if ret < 0 && ret != -EACCES {
            pm_runtime_put_noidle((*component).dev);
            return soc_component_ret(component, ret);
        }
        (*component).mark_pm = stream;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_pm_runtime_put(rtd: *mut snd_soc_pcm_runtime, stream: *mut c_void, rollback: c_int) {
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if rollback != 0 && (*component).mark_pm != stream { continue; }
        pm_runtime_put_autosuspend((*component).dev);
        (*component).mark_pm = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_pcm_component_ack(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    /* FIXME: use 1st pointer */
    for i in rtd_components!(rtd) {
        let component = rtd_component(rtd, i);
        if let Some(ack) = (*(*component).driver).ack { return ack(component, substream); }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
