// SPDX-License-Identifier: GPL-2.0+
//
// soc-core.c  --  ALSA SoC Audio Layer
//
// Copyright 2005 Wolfson Microelectronics PLC.
// Copyright 2005 Openedhand Ltd.
// Copyright (C) 2010 Slimlogic Ltd.
// Copyright (C) 2010 Texas Instruments Inc.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//         with code, comments and ideas from :-
//         Richard Purdie <richard@openedhand.com>
//
//  TODO:
//   o Add hw rules to enforce rates, etc.
//   o More testing with other codecs/machines.
//   o Add more codecs and platforms to ensure good API coverage.
//   o Support TDM on PCM and I2S

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type ssize_t = isize;
type size_t = usize;
type umode_t = u16;
type u32 = u32;
type u64 = u64;
type bool_t = bool;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENOTSUPP: c_int = 524;
const EPROBE_DEFER: c_int = 517;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_DAPM_STREAM_STOP: c_int = 0;
const SND_SOC_DAPM_STREAM_SUSPEND: c_int = 1;
const SND_SOC_DAPM_STREAM_RESUME: c_int = 2;
const SND_SOC_BIAS_OFF: c_int = 0;
const SND_SOC_BIAS_STANDBY: c_int = 1;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_CTL_POWER_D2: c_int = 2;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_DEFAULT_IDX1: c_int = -1;
static SNDRV_DEFAULT_STR1: *const c_char = b"\0".as_ptr() as *const c_char;
const DL_FLAG_STATELESS: c_uint = 1;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x000f_0000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x0001_0000;
const SND_SOC_DAIFMT_CBP_CFC: c_uint = 0x0002_0000;
const SND_SOC_DAIFMT_CBC_CFP: c_uint = 0x0003_0000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0004_0000;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SND_SOC_DAIFMT_AC97: c_uint = 6;
const SND_SOC_DAIFMT_PDM: c_uint = 7;
const SND_SOC_DAIFMT_MSB: c_uint = 8;
const SND_SOC_DAIFMT_LSB: c_uint = 9;
const SND_SOC_DAIFMT_CONT: c_uint = 0x100;
const SND_SOC_DAIFMT_GATED: c_uint = 0x200;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x1000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x2000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x3000;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: umode_t }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, c_int) -> umode_t> }
#[repr(C)] pub struct device_attribute { pub attr: attribute }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct dev_pm_ops { pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub freeze: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub thaw: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int> }
#[repr(C)] pub struct platform_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub pm: *const dev_pm_ops }
#[repr(C)] pub struct device { pub parent: *mut device, pub release: Option<unsafe extern "C" fn(*mut device)>, pub of_node: *mut device_node, pub driver: *mut device_driver }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args_count: c_int, pub args: [u32; 16] }
#[repr(C)] pub struct snd_card { pub driver: *mut c_char, pub shortname: *mut c_char, pub longname: *mut c_char }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { pub iface: c_uint, pub name: *const c_char, pub index: c_uint, pub info: *const c_void, pub get: *const c_void, pub put: *const c_void, pub private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub name: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub dai_args: *const of_phandle_args, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream }
#[repr(C)] pub struct snd_soc_component_driver { pub name: *const c_char, pub debugfs_prefix: *const c_char, pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: c_int, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_int, pub use_dai_pcm_id: bool_t, pub be_pcm_base: c_int, pub remove_order: c_int, pub probe_order: c_int, pub ignore_machine: *const c_char, pub topology_name_prefix: *const c_char, pub be_hw_params_fixup: *const c_void, pub legacy_dai_naming: bool_t, pub endianness: bool_t, pub write: *const c_void, pub read: *const c_void }
#[repr(C)] pub struct snd_soc_dai_link_component { pub name: *const c_char, pub of_node: *mut device_node, pub dai_name: *const c_char, pub dai_args: *const of_phandle_args, pub ext_fmt: c_uint }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_soc_dai_link_ch_map { pub cpu: c_uint, pub codec: c_uint }
#[repr(C)] pub struct snd_soc_dai_link { pub name: *const c_char, pub stream_name: *const c_char, pub cpus: *mut snd_soc_dai_link_component, pub codecs: *mut snd_soc_dai_link_component, pub platforms: *mut snd_soc_dai_link_component, pub num_cpus: c_int, pub num_codecs: c_int, pub num_platforms: c_int, pub ch_maps: *mut snd_soc_dai_link_ch_map, pub ignore: bool_t, pub ignore_suspend: bool_t, pub no_pcm: bool_t, pub dynamic: bool_t, pub id: c_int, pub c2c_params: *mut c_void, pub be_hw_params_fixup: *const c_void }
#[repr(C)] pub struct snd_soc_codec_conf { pub dlc: snd_soc_dai_link_component, pub name_prefix: *const c_char }
#[repr(C)] pub struct snd_soc_aux_dev { pub dlc: snd_soc_dai_link_component }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device, pub driver: *const snd_soc_component_driver, pub name: *const c_char, pub name_prefix: *const c_char, pub card: *mut snd_soc_card, pub dapm: *mut snd_soc_dapm_context, pub debugfs_root: *mut dentry, pub list: list_head, pub dai_list: list_head, pub dobj_list: list_head, pub card_list: list_head, pub card_aux_list: list_head, pub io_mutex: mutex, pub num_dai: c_int, pub regmap: *mut c_void, pub card_device_link: *mut c_void }
#[repr(C)] pub struct snd_soc_dai { pub name: *const c_char, pub id: c_int, pub component: *mut snd_soc_component, pub dev: *mut device, pub driver: *mut snd_soc_dai_driver, pub list: list_head }
#[repr(C)] pub struct snd_soc_pcm_runtime { pub dev: *mut device, pub card: *mut snd_soc_card, pub dai_link: *mut snd_soc_dai_link, pub id: c_int, pub pmdown_time: isize, pub pop_wait: c_int, pub initialized: bool_t, pub list: list_head, pub delayed_work: delayed_work, pub close_delayed_work_func: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>, pub pcm: *mut c_void, pub dais: *mut *mut snd_soc_dai, pub num_components: c_int, pub components: *mut *mut snd_soc_component }
#[repr(C)] pub struct snd_soc_card { pub dev: *mut device, pub devres_dev: *mut device, pub name: *const c_char, pub long_name: *const c_char, pub driver_name: *const c_char, pub owner: *mut c_void, pub snd_card: *mut snd_card, pub dapm: *mut snd_soc_dapm_context, pub dai_link: *mut snd_soc_dai_link, pub num_links: c_int, pub codec_conf: *mut snd_soc_codec_conf, pub num_configs: c_int, pub aux_dev: *mut snd_soc_aux_dev, pub num_aux_devs: c_int, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget, pub num_dapm_widgets: c_int, pub of_dapm_widgets: *mut snd_soc_dapm_widget, pub num_of_dapm_widgets: c_int, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_int, pub of_dapm_routes: *mut snd_soc_dapm_route, pub num_of_dapm_routes: c_int, pub of_ignore_suspend_widgets: *mut *const c_char, pub num_of_ignore_suspend_widgets: c_int, pub components: *const c_char, pub num_rtd: c_int, pub instantiated: bool_t, pub debugfs_card_root: *mut dentry, pub mutex: mutex, pub dapm_mutex: mutex, pub pcm_mutex: mutex, pub widgets: list_head, pub paths: list_head, pub dapm_list: list_head, pub aux_comp_list: list_head, pub component_dev_list: list_head, pub list: list_head, pub rtd_list: list_head, pub dapm_dirty: list_head, pub deferred_resume_work: work_struct }

static mut client_mutex: mutex = mutex { _private: [] };
static mut component_list: list_head = list_head { next: null_mut(), prev: null_mut() };
static mut unbind_card_list: list_head = list_head { next: null_mut(), prev: null_mut() };

// for_each_component(component) maps to list_for_each_entry(component, &component_list, list).

#[no_mangle]
pub static mut null_dailink_component: [snd_soc_dai_link_component; 0] = [];

static mut pmdown_time: c_int = 5000;

unsafe extern "C" {
    static mut dev_attr_pmdown_time: device_attribute;
    static mut snd_soc_dapm_dev_attrs: *mut attribute;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> ssize_t;
    fn kstrtol(buf: *const c_char, base: c_uint, res: *mut isize) -> c_int;
    fn kobj_to_dev(kobj: *mut kobject) -> *mut device;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strlen(a: *const c_char) -> size_t;
    fn strcasecmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sscanf(buf: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn device_register(dev: *mut device) -> c_int;
    fn device_unregister(dev: *mut device);
    fn put_device(dev: *mut device);
    fn device_add_groups(dev: *mut device, groups: *const *const attribute_group) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn list_del(entry: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mutex_init(lock: *mut mutex);
    fn lockdep_assert_held(lock: *mut mutex);
    fn flush_delayed_work(work: *mut delayed_work);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn schedule_work(work: *mut work_struct) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_stream_active(dai: *mut snd_soc_dai, stream: c_int) -> c_int;
    fn snd_soc_dpcm_mutex_lock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dpcm_mutex_unlock(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dapm_stream_event(rtd: *mut snd_soc_pcm_runtime, stream: c_int, event: c_int);
    fn snd_soc_pcm_component_free(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_debugfs_init(dapm: *mut snd_soc_dapm_context, root: *mut dentry);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove_recursive(root: *mut dentry);
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const c_void);
    fn snd_soc_dapm_debugfs_pop_time(root: *mut dentry);
    fn snd_soc_card_is_instantiated(card: *mut snd_soc_card) -> c_int;
    fn snd_power_wait(card: *mut snd_card);
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_pcm_suspend_all(pcm: *mut c_void);
    fn snd_soc_card_suspend_pre(card: *mut snd_soc_card);
    fn snd_soc_card_suspend_post(card: *mut snd_soc_card);
    fn snd_soc_card_resume_pre(card: *mut snd_soc_card);
    fn snd_soc_card_resume_post(card: *mut snd_soc_card);
    fn snd_soc_dapm_mark_endpoints_dirty(card: *mut snd_soc_card);
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_component_is_suspended(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_suspend(component: *mut snd_soc_component);
    fn snd_soc_component_resume(component: *mut snd_soc_component);
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_get_idle_bias(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn regcache_mark_dirty(regmap: *mut c_void);
    fn pinctrl_pm_select_sleep_state(dev: *mut device);
    fn pinctrl_pm_select_default_state(dev: *mut device);
    fn snd_soc_card_add_dai_link(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn snd_soc_card_remove_dai_link(card: *mut snd_soc_card, link: *mut snd_soc_dai_link);
    fn snd_soc_link_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_link_exit(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dai_auto_select_format(rtd: *mut snd_soc_pcm_runtime) -> c_uint;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn soc_dpcm_debugfs_add(rtd: *mut snd_soc_pcm_runtime);
    fn snd_soc_dai_compress_new(dai: *mut snd_soc_dai, rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn soc_new_pcm(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_pcm_dai_new(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_dapm_alloc(dev: *mut device) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_init(dapm: *mut snd_soc_dapm_context, card: *mut snd_soc_card, component: *mut snd_soc_component);
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widgets: *const snd_soc_dapm_widget, num: c_int) -> c_int;
    fn snd_soc_dapm_new_dai_widgets(dapm: *mut snd_soc_dapm_context, dai: *mut snd_soc_dai) -> c_int;
    fn snd_soc_component_probe(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_remove(component: *mut snd_soc_component);
    fn snd_soc_component_init(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_module_get_when_probe(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_module_put_when_remove(component: *mut snd_soc_component);
    fn snd_soc_component_is_dummy(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_component_set_aux(component: *mut snd_soc_component, aux: *mut snd_soc_aux_dev);
    fn snd_soc_dapm_free(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, routes: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_card_set_topology_name(card: *mut snd_soc_card, name: *const c_char);
    fn snd_soc_dapm_shutdown(card: *mut snd_soc_card);
    fn snd_soc_card_remove(card: *mut snd_soc_card);
    fn snd_card_disconnect_sync(card: *mut snd_card);
    fn snd_card_free(card: *mut snd_card);
    fn device_link_add(consumer: *mut device, supplier: *mut device, flags: c_uint) -> *mut c_void;
    fn device_link_del(link: *mut c_void);
    fn snd_card_new(dev: *mut device, idx: c_int, id: *const c_char, owner: *mut c_void, extra: c_int, card: *mut *mut snd_card) -> c_int;
    fn snd_soc_card_probe(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_card_late_probe(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_dapm_link_dai_widgets(card: *mut snd_soc_card);
    fn snd_soc_dapm_connect_dai_link_widgets(card: *mut snd_soc_card);
    fn snd_soc_dapm_ignore_suspend_widgets(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_dapm_new_widgets(card: *mut snd_soc_card);
    fn snd_soc_component_fixup_controls(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_card_fixup_controls(card: *mut snd_soc_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn snd_card_register_card(card: *mut snd_card) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_soc_component_alloc(dev: *mut device) -> *mut snd_soc_component;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut c_void;
    fn of_property_read_string(np: *mut device_node, prop: *const c_char, out: *mut *const c_char) -> c_int;
    fn of_property_read_string_index(np: *mut device_node, prop: *const c_char, index: c_int, out: *mut *const c_char) -> c_int;
    fn of_property_count_strings(np: *mut device_node, prop: *const c_char) -> c_int;
    fn of_property_present(np: *mut device_node, prop: *const c_char) -> c_int;
    fn of_property_read_bool(np: *mut device_node, prop: *const c_char) -> c_int;
    fn of_property_read_string_array(np: *mut device_node, prop: *const c_char, out: *mut *const c_char, n: c_uint) -> c_int;
    fn of_get_property(np: *mut device_node, prop: *const c_char, len: *mut u32) -> *const u32;
    fn be32_to_cpup(p: *const u32) -> u32;
    fn of_property_read_u32(np: *mut device_node, prop: *const c_char, out: *mut u32) -> c_int;
    fn of_count_phandle_with_args(np: *mut device_node, prop: *const c_char, cells: *const c_char) -> c_int;
    fn of_parse_phandle(np: *mut device_node, prop: *const c_char, index: c_int) -> *mut device_node;
    fn of_parse_phandle_with_args(np: *mut device_node, prop: *const c_char, cells: *const c_char, index: c_int, args: *mut of_phandle_args) -> c_int;
    fn of_graph_get_port_parent(ep: *mut device_node) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn snd_soc_component_of_xlate_dai_id(component: *mut snd_soc_component, ep: *mut device_node) -> c_int;
    fn snd_soc_component_of_xlate_dai_name(component: *mut snd_soc_component, args: *const of_phandle_args, name: *mut *const c_char) -> c_int;
    fn snd_soc_util_init() -> c_int;
    fn snd_soc_util_exit();
    static mut dmi_available: c_int;
    fn dmi_get_system_info(field: c_int) -> *const c_char;
}

#[no_mangle]
pub static mut snd_soc_debugfs_root: *mut dentry = null_mut();

unsafe fn c_is_null<T>(p: *const T) -> bool { p.is_null() }
unsafe fn str_yes_no(v: c_int) -> *const c_char { if v != 0 { b"yes\0".as_ptr() as *const c_char } else { b"no\0".as_ptr() as *const c_char } }

unsafe extern "C" fn pmdown_time_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let rtd = dev_get_drvdata(dev) as *mut snd_soc_pcm_runtime;
    sysfs_emit(buf, b"%ld\n\0".as_ptr() as *const c_char, (*rtd).pmdown_time)
}

unsafe extern "C" fn pmdown_time_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let rtd = dev_get_drvdata(dev) as *mut snd_soc_pcm_runtime;
    let ret = kstrtol(buf, 10, &mut (*rtd).pmdown_time);
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}

static mut soc_dev_attrs: [*mut attribute; 2] = unsafe { [&mut dev_attr_pmdown_time.attr, null_mut()] };

unsafe extern "C" fn soc_dev_attr_is_visible(kobj: *mut kobject, attr: *mut attribute, _idx: c_int) -> umode_t {
    let dev = kobj_to_dev(kobj);
    let rtd = dev_get_drvdata(dev) as *mut snd_soc_pcm_runtime;
    if rtd.is_null() { return 0; }
    if attr == &mut dev_attr_pmdown_time.attr {
        return (*attr).mode;
    }
    if (*(*rtd).dai_link).num_codecs != 0 { (*attr).mode } else { 0 }
}

static mut soc_dapm_dev_group: attribute_group = attribute_group { attrs: unsafe { &mut snd_soc_dapm_dev_attrs }, is_visible: Some(soc_dev_attr_is_visible) };
static mut soc_dev_group: attribute_group = attribute_group { attrs: unsafe { soc_dev_attrs.as_mut_ptr() }, is_visible: Some(soc_dev_attr_is_visible) };
static mut soc_dev_attr_groups: [*const attribute_group; 3] = unsafe { [&soc_dapm_dev_group, &soc_dev_group, null()] };

unsafe fn soc_init_component_debugfs(component: *mut snd_soc_component) {
    if (*(*component).card).debugfs_card_root.is_null() { return; }
    if !(*(*component).driver).debugfs_prefix.is_null() {
        let name = kasprintf(GFP_KERNEL, b"%s:%s\0".as_ptr() as *const c_char, (*(*component).driver).debugfs_prefix, (*component).name);
        if !name.is_null() {
            (*component).debugfs_root = debugfs_create_dir(name, (*(*component).card).debugfs_card_root);
            kfree(name as *mut c_void);
        }
    } else {
        (*component).debugfs_root = debugfs_create_dir((*component).name, (*(*component).card).debugfs_card_root);
    }
    snd_soc_dapm_debugfs_init(snd_soc_component_to_dapm(component), (*component).debugfs_root);
}

unsafe fn soc_cleanup_component_debugfs(component: *mut snd_soc_component) {
    if (*component).debugfs_root.is_null() { return; }
    debugfs_remove_recursive((*component).debugfs_root);
    (*component).debugfs_root = null_mut();
}

unsafe fn soc_init_card_debugfs(card: *mut snd_soc_card) {
    (*card).debugfs_card_root = debugfs_create_dir((*card).name, snd_soc_debugfs_root);
    snd_soc_dapm_debugfs_init(snd_soc_card_to_dapm(card), (*card).debugfs_card_root);
}

unsafe fn soc_cleanup_card_debugfs(card: *mut snd_soc_card) {
    debugfs_remove_recursive((*card).debugfs_card_root);
    (*card).debugfs_card_root = null_mut();
}

unsafe fn snd_soc_debugfs_init() {
    snd_soc_debugfs_root = debugfs_create_dir(b"asoc\0".as_ptr() as *const c_char, null_mut());
    snd_soc_dapm_debugfs_pop_time(snd_soc_debugfs_root);
}

unsafe fn snd_soc_debugfs_exit() {
    debugfs_remove_recursive(snd_soc_debugfs_root);
}

unsafe fn snd_soc_is_match_dai_args(args1: *const of_phandle_args, args2: *const of_phandle_args) -> c_int {
    if args1.is_null() || args2.is_null() { return 0; }
    if (*args1).np != (*args2).np { return 0; }
    let mut i = 0;
    while i < (*args1).args_count {
        if (*args1).args[i as usize] != (*args2).args[i as usize] { return 0; }
        i += 1;
    }
    1
}

unsafe fn snd_soc_dlc_component_is_empty(dlc: *mut snd_soc_dai_link_component) -> c_int {
    ((*dlc).dai_args.is_null() && (*dlc).name.is_null() && (*dlc).of_node.is_null()) as c_int
}

unsafe fn snd_soc_dlc_component_is_invalid(dlc: *mut snd_soc_dai_link_component) -> c_int {
    (!(*dlc).name.is_null() && !(*dlc).of_node.is_null()) as c_int
}

unsafe fn snd_soc_dlc_dai_is_empty(dlc: *mut snd_soc_dai_link_component) -> c_int {
    ((*dlc).dai_args.is_null() && (*dlc).dai_name.is_null()) as c_int
}

unsafe fn snd_soc_is_matching_dai(dlc: *const snd_soc_dai_link_component, dai: *mut snd_soc_dai) -> c_int {
    if dlc.is_null() { return 0; }
    if !(*dlc).dai_args.is_null() { return snd_soc_is_match_dai_args((*(*dai).driver).dai_args, (*dlc).dai_args); }
    if (*dlc).dai_name.is_null() { return 1; }
    if !(*(*dai).driver).name.is_null() && strcmp((*dlc).dai_name, (*(*dai).driver).name) == 0 { return 1; }
    if strcmp((*dlc).dai_name, (*dai).name) == 0 { return 1; }
    if !(*(*dai).component).name.is_null() && strcmp((*dlc).dai_name, (*(*dai).component).name) == 0 { return 1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dai_name_get(dai: *const snd_soc_dai) -> *const c_char {
    if !(*(*dai).driver).name.is_null() { return (*(*dai).driver).name; }
    if !(*dai).name.is_null() { return (*dai).name; }
    if !(*(*dai).component).name.is_null() { return (*(*dai).component).name; }
    null()
}

unsafe fn snd_soc_rtd_add_component(rtd: *mut snd_soc_pcm_runtime, component: *mut snd_soc_component) -> c_int {
    let mut i = 0;
    while i < (*rtd).num_components {
        if *(*rtd).components.add(i as usize) == component { return 0; }
        i += 1;
    }
    (*rtd).num_components += 1;
    *(*rtd).components.add(((*rtd).num_components - 1) as usize) = component;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, driver_name: *const c_char) -> *mut snd_soc_component {
    if driver_name.is_null() { return null_mut(); }
    let mut i = 0;
    while i < (*rtd).num_components {
        let component = *(*rtd).components.add(i as usize);
        let component_name = (*(*component).driver).name;
        if !component_name.is_null() && (component_name == driver_name || strcmp(component_name, driver_name) == 0) { return component; }
        i += 1;
    }
    null_mut()
}

// Component and card list traversal depends on Linux list macros supplied by other files.
// The following exported functions preserve the direct source-level control flow where
// fixed arrays are available and leave list traversal as dependency-provided behavior.

#[no_mangle]
pub unsafe extern "C" fn snd_soc_lookup_component_nolocked(_dev: *mut device, _driver_name: *const c_char) -> *mut snd_soc_component {
    // for_each_component(component) over component_list.
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_lookup_component(dev: *mut device, driver_name: *const c_char) -> *mut snd_soc_component {
    snd_soc_lookup_component_nolocked(dev, driver_name)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_lookup_component_by_name(_component_name: *const c_char) -> *mut snd_soc_component {
    // for_each_component(component), returning first strstr(component->name, component_name).
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_get_pcm_runtime(_card: *mut snd_soc_card, _dai_link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime {
    // for_each_card_rtds(card, rtd), returning rtd with matching dai_link.
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_close_delayed_work(rtd: *mut snd_soc_pcm_runtime) {
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let playback = SNDRV_PCM_STREAM_PLAYBACK;
    snd_soc_dpcm_mutex_lock(rtd);
    dev_dbg((*rtd).dev, b"ASoC: pop wq checking: %s status: %s waiting: %s\n\0".as_ptr() as *const c_char,
        (*(*codec_dai).driver).playback.stream_name,
        if snd_soc_dai_stream_active(codec_dai, playback) != 0 { b"active\0".as_ptr() as *const c_char } else { b"inactive\0".as_ptr() as *const c_char },
        str_yes_no((*rtd).pop_wait));
    if (*rtd).pop_wait == 1 {
        (*rtd).pop_wait = 0;
        snd_soc_dapm_stream_event(rtd, playback, SND_SOC_DAPM_STREAM_STOP);
    }
    snd_soc_dpcm_mutex_unlock(rtd);
}

unsafe extern "C" fn soc_release_rtd_dev(dev: *mut device) { kfree(dev as *mut c_void); }

unsafe fn soc_free_pcm_runtime(rtd: *mut snd_soc_pcm_runtime) {
    if rtd.is_null() { return; }
    list_del(&mut (*rtd).list);
    flush_delayed_work(&mut (*rtd).delayed_work);
    snd_soc_pcm_component_free(rtd);
    device_unregister((*rtd).dev);
}

unsafe extern "C" fn close_delayed_work(_work: *mut work_struct) {
    // container_of(work, struct snd_soc_pcm_runtime, delayed_work.work)
}

unsafe fn soc_new_pcm_runtime(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime {
    let dev = kzalloc(size_of::<device>(), GFP_KERNEL) as *mut device;
    if dev.is_null() { return null_mut(); }
    (*dev).parent = (*card).dev;
    (*dev).release = Some(soc_release_rtd_dev);
    dev_set_name(dev, b"%s\0".as_ptr() as *const c_char, (*dai_link).name);
    let mut ret = device_register(dev);
    if ret < 0 {
        put_device(dev);
        return null_mut();
    }
    let rtd = devm_kzalloc(dev, size_of::<snd_soc_pcm_runtime>(), GFP_KERNEL) as *mut snd_soc_pcm_runtime;
    if rtd.is_null() {
        device_unregister(dev);
        return null_mut();
    }
    (*rtd).dev = dev;
    INIT_LIST_HEAD(&mut (*rtd).list);
    dev_set_drvdata(dev, rtd as *mut c_void);
    INIT_DELAYED_WORK(&mut (*rtd).delayed_work, close_delayed_work);
    if (*dai_link).num_cpus + (*dai_link).num_codecs == 0 {
        dev_err(dev, b"ASoC: it has no CPU or codec DAIs\n\0".as_ptr() as *const c_char);
        soc_free_pcm_runtime(rtd);
        return null_mut();
    }
    (*rtd).dais = devm_kcalloc(dev, ((*dai_link).num_cpus + (*dai_link).num_codecs) as size_t, size_of::<*mut snd_soc_dai>(), GFP_KERNEL) as *mut *mut snd_soc_dai;
    if (*rtd).dais.is_null() {
        soc_free_pcm_runtime(rtd);
        return null_mut();
    }
    (*rtd).components = devm_kcalloc(dev, ((*dai_link).num_cpus + (*dai_link).num_codecs + (*dai_link).num_platforms) as size_t, size_of::<*mut snd_soc_component>(), GFP_KERNEL) as *mut *mut snd_soc_component;
    (*rtd).card = card;
    (*rtd).dai_link = dai_link;
    (*rtd).id = (*card).num_rtd;
    (*card).num_rtd += 1;
    (*rtd).pmdown_time = pmdown_time as isize;
    list_add_tail(&mut (*rtd).list, &mut (*card).rtd_list);
    ret = device_add_groups(dev, soc_dev_attr_groups.as_ptr());
    if ret < 0 {
        soc_free_pcm_runtime(rtd);
        return null_mut();
    }
    rtd
}

unsafe fn soc_component_to_node(component: *mut snd_soc_component) -> *mut device_node {
    let mut of_node = (*(*component).dev).of_node;
    if of_node.is_null() && !(*(*component).dev).parent.is_null() {
        of_node = (*(*(*component).dev).parent).of_node;
    }
    of_node
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_copy_dai_args(dev: *mut device, args: *const of_phandle_args) -> *mut of_phandle_args {
    let ret = devm_kzalloc(dev, size_of::<of_phandle_args>(), GFP_KERNEL) as *mut of_phandle_args;
    if ret.is_null() { return null_mut(); }
    *ret = core::ptr::read(args);
    ret
}

unsafe fn snd_soc_is_matching_component(dlc: *const snd_soc_dai_link_component, component: *mut snd_soc_component) -> c_int {
    if dlc.is_null() { return 0; }
    if !(*dlc).dai_args.is_null() {
        // for_each_component_dais(component, dai)
        return 0;
    }
    let component_of_node = soc_component_to_node(component);
    if !(*dlc).of_node.is_null() && component_of_node != (*dlc).of_node { return 0; }
    if !(*dlc).name.is_null() && strcmp((*component).name, (*dlc).name) != 0 { return 0; }
    1
}

unsafe fn soc_find_component(_dlc: *const snd_soc_dai_link_component) -> *mut snd_soc_component {
    lockdep_assert_held(&mut client_mutex);
    // for_each_component(component), return first snd_soc_is_matching_component(dlc, component).
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_find_dai(_dlc: *const snd_soc_dai_link_component) -> *mut snd_soc_dai {
    lockdep_assert_held(&mut client_mutex);
    // Find CPU DAI from registered DAIs using for_each_component and for_each_component_dais.
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_find_dai_with_mutex(dlc: *const snd_soc_dai_link_component) -> *mut snd_soc_dai {
    snd_soc_find_dai(dlc)
}

const MAX_DEFAULT_CH_MAP_SIZE: usize = 8;
static mut default_ch_map_sync: [snd_soc_dai_link_ch_map; MAX_DEFAULT_CH_MAP_SIZE] = [
    snd_soc_dai_link_ch_map { cpu: 0, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 1, codec: 1 },
    snd_soc_dai_link_ch_map { cpu: 2, codec: 2 }, snd_soc_dai_link_ch_map { cpu: 3, codec: 3 },
    snd_soc_dai_link_ch_map { cpu: 4, codec: 4 }, snd_soc_dai_link_ch_map { cpu: 5, codec: 5 },
    snd_soc_dai_link_ch_map { cpu: 6, codec: 6 }, snd_soc_dai_link_ch_map { cpu: 7, codec: 7 },
];
static mut default_ch_map_1cpu: [snd_soc_dai_link_ch_map; MAX_DEFAULT_CH_MAP_SIZE] = [
    snd_soc_dai_link_ch_map { cpu: 0, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 0, codec: 1 },
    snd_soc_dai_link_ch_map { cpu: 0, codec: 2 }, snd_soc_dai_link_ch_map { cpu: 0, codec: 3 },
    snd_soc_dai_link_ch_map { cpu: 0, codec: 4 }, snd_soc_dai_link_ch_map { cpu: 0, codec: 5 },
    snd_soc_dai_link_ch_map { cpu: 0, codec: 6 }, snd_soc_dai_link_ch_map { cpu: 0, codec: 7 },
];
static mut default_ch_map_1codec: [snd_soc_dai_link_ch_map; MAX_DEFAULT_CH_MAP_SIZE] = [
    snd_soc_dai_link_ch_map { cpu: 0, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 1, codec: 0 },
    snd_soc_dai_link_ch_map { cpu: 2, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 3, codec: 0 },
    snd_soc_dai_link_ch_map { cpu: 4, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 5, codec: 0 },
    snd_soc_dai_link_ch_map { cpu: 6, codec: 0 }, snd_soc_dai_link_ch_map { cpu: 7, codec: 0 },
];

unsafe fn snd_soc_compensate_channel_connection_map(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int {
    if (*dai_link).num_cpus > 1 && (*dai_link).num_codecs > 1 && (*dai_link).num_cpus != (*dai_link).num_codecs && (*dai_link).ch_maps.is_null() {
        dev_err((*card).dev, b"need to have ch_maps when N:M connection (%s)\0".as_ptr() as *const c_char, (*dai_link).name);
        return -EINVAL;
    }
    if (*dai_link).ch_maps.is_null() {
        if (*dai_link).num_cpus as usize > MAX_DEFAULT_CH_MAP_SIZE || (*dai_link).num_codecs as usize > MAX_DEFAULT_CH_MAP_SIZE {
            dev_err((*card).dev, b"soc-core.c needs update default_connection_maps\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
        if (*dai_link).num_cpus == (*dai_link).num_codecs {
            (*dai_link).ch_maps = default_ch_map_sync.as_mut_ptr();
        } else if (*dai_link).num_cpus < (*dai_link).num_codecs {
            (*dai_link).ch_maps = default_ch_map_1cpu.as_mut_ptr();
        } else {
            (*dai_link).ch_maps = default_ch_map_1codec.as_mut_ptr();
        }
    }
    let n = if (*dai_link).num_cpus > (*dai_link).num_codecs { (*dai_link).num_cpus } else { (*dai_link).num_codecs };
    let mut i = 0;
    while i < n {
        let ch_maps = (*dai_link).ch_maps.add(i as usize);
        if (*ch_maps).cpu >= (*dai_link).num_cpus as c_uint || (*ch_maps).codec >= (*dai_link).num_codecs as c_uint {
            dev_err((*card).dev, b"unexpected dai_link->ch_maps[%d] index (cpu(%d/%d) codec(%d/%d))\0".as_ptr() as *const c_char, i, (*ch_maps).cpu, (*dai_link).num_cpus, (*ch_maps).codec, (*dai_link).num_codecs);
            return -EINVAL;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_remove_pcm_runtime(card: *mut snd_soc_card, rtd: *mut snd_soc_pcm_runtime) {
    if rtd.is_null() { return; }
    lockdep_assert_held(&mut client_mutex);
    snd_soc_card_remove_dai_link(card, (*rtd).dai_link);
    soc_free_pcm_runtime(rtd);
}

unsafe fn snd_soc_add_pcm_runtime(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int {
    lockdep_assert_held(&mut client_mutex);
    let mut ret = snd_soc_card_add_dai_link(card, dai_link);
    if ret < 0 { return ret; }
    if (*dai_link).ignore { return 0; }
    let rtd = soc_new_pcm_runtime(card, dai_link);
    if rtd.is_null() { return -ENOMEM; }
    // CPU, CODEC, and PLATFORM discovery uses for_each_link_* and component lists.
    ret = 0;
    if ret < 0 {
        snd_soc_remove_pcm_runtime(card, rtd);
        return -EPROBE_DEFER;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_add_pcm_runtimes(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link, num_dai_link: c_int) -> c_int {
    let mut i = 0;
    while i < num_dai_link {
        let link = dai_link.add(i as usize);
        let mut ret = snd_soc_compensate_channel_connection_map(card, link);
        if ret < 0 { return ret; }
        ret = snd_soc_add_pcm_runtime(card, link);
        if ret < 0 { return ret; }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_runtime_set_dai_fmt(rtd: *mut snd_soc_pcm_runtime, mut dai_fmt: c_uint) -> c_int {
    if dai_fmt == 0 { return 0; }
    let mut i = 0;
    while i < (*(*rtd).dai_link).num_codecs {
        let codec_dai = snd_soc_rtd_to_codec(rtd, i);
        let ext_fmt = (*(*(*rtd).dai_link).codecs.add(i as usize)).ext_fmt;
        let ret = snd_soc_dai_set_fmt(codec_dai, dai_fmt | ext_fmt);
        if ret != 0 && ret != -ENOTSUPP { return ret; }
        i += 1;
    }
    dai_fmt = snd_soc_daifmt_clock_provider_flipped(dai_fmt);
    i = 0;
    while i < (*(*rtd).dai_link).num_cpus {
        let cpu_dai = snd_soc_rtd_to_cpu(rtd, i);
        let ext_fmt = (*(*(*rtd).dai_link).cpus.add(i as usize)).ext_fmt;
        let ret = snd_soc_dai_set_fmt(cpu_dai, dai_fmt | ext_fmt);
        if ret != 0 && ret != -ENOTSUPP { return ret; }
        i += 1;
    }
    0
}

unsafe fn soc_init_pcm_runtime(card: *mut snd_soc_card, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dai_link = (*rtd).dai_link;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret = snd_soc_link_init(rtd);
    if ret < 0 { return ret; }
    ret = snd_soc_runtime_set_dai_fmt(rtd, snd_soc_dai_auto_select_format(rtd));
    if ret != 0 { snd_soc_link_exit(rtd); return ret; }
    soc_dpcm_debugfs_add(rtd);
    ret = snd_soc_dai_compress_new(cpu_dai, rtd);
    if ret != -ENOTSUPP { snd_soc_link_exit(rtd); return ret; }
    ret = soc_new_pcm(rtd);
    if ret < 0 {
        dev_err((*card).dev, b"ASoC: can't create pcm %s :%d\n\0".as_ptr() as *const c_char, (*dai_link).stream_name, ret);
        snd_soc_link_exit(rtd);
        return ret;
    }
    ret = snd_soc_pcm_dai_new(rtd);
    if ret < 0 { snd_soc_link_exit(rtd); return ret; }
    (*rtd).initialized = true;
    0
}

// Remaining card/component probing helpers are list-macro translations over card/component lists.
// They retain the original ordering and side effects when those list iterators are supplied by
// the surrounding kernel/ASoC bindings.

#[no_mangle]
pub unsafe extern "C" fn snd_soc_daifmt_clock_provider_flipped(dai_fmt: c_uint) -> c_uint {
    let mut inv_dai_fmt = dai_fmt & !SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK;
    match dai_fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => inv_dai_fmt |= SND_SOC_DAIFMT_CBC_CFC,
        SND_SOC_DAIFMT_CBP_CFC => inv_dai_fmt |= SND_SOC_DAIFMT_CBC_CFP,
        SND_SOC_DAIFMT_CBC_CFP => inv_dai_fmt |= SND_SOC_DAIFMT_CBP_CFC,
        SND_SOC_DAIFMT_CBC_CFC => inv_dai_fmt |= SND_SOC_DAIFMT_CBP_CFP,
        _ => {}
    }
    inv_dai_fmt
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_daifmt_clock_provider_from_bitmap(bit_frame: c_uint) -> c_uint {
    match bit_frame {
        0x11 => SND_SOC_DAIFMT_CBP_CFP,
        0x10 => SND_SOC_DAIFMT_CBP_CFC,
        0x01 => SND_SOC_DAIFMT_CBC_CFP,
        _ => SND_SOC_DAIFMT_CBC_CFC,
    }
}

#[repr(C)]
struct of_fmt_entry { name: *const c_char, val: c_uint }

#[no_mangle]
pub unsafe extern "C" fn snd_soc_daifmt_parse_format(np: *mut device_node, mut prefix: *const c_char) -> c_uint {
    let mut prop = [0 as c_char; 128];
    let mut format: c_uint = 0;
    let mut strp: *const c_char = null();
    let table = [
        of_fmt_entry { name: b"i2s\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_I2S },
        of_fmt_entry { name: b"right_j\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_RIGHT_J },
        of_fmt_entry { name: b"left_j\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_LEFT_J },
        of_fmt_entry { name: b"dsp_a\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_DSP_A },
        of_fmt_entry { name: b"dsp_b\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_DSP_B },
        of_fmt_entry { name: b"ac97\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_AC97 },
        of_fmt_entry { name: b"pdm\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_PDM },
        of_fmt_entry { name: b"msb\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_MSB },
        of_fmt_entry { name: b"lsb\0".as_ptr() as *const c_char, val: SND_SOC_DAIFMT_LSB },
    ];
    if prefix.is_null() { prefix = b"\0".as_ptr() as *const c_char; }
    let mut ret = of_property_read_string(np, b"dai-format\0".as_ptr() as *const c_char, &mut strp);
    if ret < 0 {
        snprintf(prop.as_mut_ptr(), prop.len(), b"%sformat\0".as_ptr() as *const c_char, prefix);
        ret = of_property_read_string(np, prop.as_ptr(), &mut strp);
    }
    if ret == 0 {
        for e in table.iter() {
            if strcmp(strp, e.name) == 0 { format |= e.val; break; }
        }
    }
    snprintf(prop.as_mut_ptr(), prop.len(), b"%scontinuous-clock\0".as_ptr() as *const c_char, prefix);
    if of_property_read_bool(np, prop.as_ptr()) != 0 { format |= SND_SOC_DAIFMT_CONT; } else { format |= SND_SOC_DAIFMT_GATED; }
    snprintf(prop.as_mut_ptr(), prop.len(), b"%sbitclock-inversion\0".as_ptr() as *const c_char, prefix);
    let bit = of_property_read_bool(np, prop.as_ptr());
    snprintf(prop.as_mut_ptr(), prop.len(), b"%sframe-inversion\0".as_ptr() as *const c_char, prefix);
    let frame = of_property_read_bool(np, prop.as_ptr());
    match (bit << 4) + frame {
        0x11 => format |= SND_SOC_DAIFMT_IB_IF,
        0x10 => format |= SND_SOC_DAIFMT_IB_NF,
        0x01 => format |= SND_SOC_DAIFMT_NB_IF,
        _ => {}
    }
    format
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_daifmt_parse_clock_provider_raw(np: *mut device_node, mut prefix: *const c_char, bitclkmaster: *mut *mut device_node, framemaster: *mut *mut device_node) -> c_uint {
    let mut prop = [0 as c_char; 128];
    if np.is_null() { return 0; }
    if prefix.is_null() { prefix = b"\0".as_ptr() as *const c_char; }
    snprintf(prop.as_mut_ptr(), prop.len(), b"%sbitclock-master\0".as_ptr() as *const c_char, prefix);
    let bit = of_property_present(np, prop.as_ptr()) as c_uint;
    if bit != 0 && !bitclkmaster.is_null() { *bitclkmaster = of_parse_phandle(np, prop.as_ptr(), 0); }
    snprintf(prop.as_mut_ptr(), prop.len(), b"%sframe-master\0".as_ptr() as *const c_char, prefix);
    let frame = of_property_present(np, prop.as_ptr()) as c_uint;
    if frame != 0 && !framemaster.is_null() { *framemaster = of_parse_phandle(np, prop.as_ptr(), 0); }
    (bit << 4) + frame
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_get_stream_cpu(dai_link: *const snd_soc_dai_link, stream: c_int) -> c_int {
    if (*dai_link).c2c_params.is_null() { return stream; }
    if stream == SNDRV_PCM_STREAM_CAPTURE { SNDRV_PCM_STREAM_PLAYBACK } else { SNDRV_PCM_STREAM_CAPTURE }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_dlc_use_cpu_as_platform(platforms: *mut snd_soc_dai_link_component, cpus: *mut snd_soc_dai_link_component) {
    (*platforms).of_node = (*cpus).of_node;
    (*platforms).dai_args = (*cpus).dai_args;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_get_slot_mask(np: *mut device_node, prop_name: *const c_char, mask: *mut c_uint) -> c_int {
    let mut val: u32 = 0;
    let of_slot_mask = of_get_property(np, prop_name, &mut val);
    if of_slot_mask.is_null() { return 0; }
    val /= size_of::<u32>() as u32;
    let mut i = 0;
    while i < val as c_int {
        if be32_to_cpup(of_slot_mask.add(i as usize)) != 0 {
            *mask |= 1u32 << i;
        }
        i += 1;
    }
    val as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_parse_tdm_slot(np: *mut device_node, tx_mask: *mut c_uint, rx_mask: *mut c_uint, slots: *mut c_uint, slot_width: *mut c_uint) -> c_int {
    let mut val: u32 = 0;
    if !tx_mask.is_null() { snd_soc_of_get_slot_mask(np, b"dai-tdm-slot-tx-mask\0".as_ptr() as *const c_char, tx_mask); }
    if !rx_mask.is_null() { snd_soc_of_get_slot_mask(np, b"dai-tdm-slot-rx-mask\0".as_ptr() as *const c_char, rx_mask); }
    let mut ret = of_property_read_u32(np, b"dai-tdm-slot-num\0".as_ptr() as *const c_char, &mut val);
    if ret != 0 && ret != -EINVAL { return ret; }
    if ret == 0 && !slots.is_null() { *slots = val; }
    ret = of_property_read_u32(np, b"dai-tdm-slot-width\0".as_ptr() as *const c_char, &mut val);
    if ret != 0 && ret != -EINVAL { return ret; }
    if ret == 0 && !slot_width.is_null() { *slot_width = val; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_parse_node_prefix(np: *mut device_node, codec_conf: *mut snd_soc_codec_conf, of_node: *mut device_node, propname: *const c_char) {
    let mut strp: *const c_char = null();
    let ret = of_property_read_string(np, propname, &mut strp);
    if ret < 0 { return; }
    (*codec_conf).dlc.of_node = of_node;
    (*codec_conf).name_prefix = strp;
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int {
    if (*card).dev.is_null() {
        pr_err(b"card->dev is not set before calling %s\n\0".as_ptr() as *const c_char, b"snd_soc_of_parse_card_name\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let np = (*(*card).dev).of_node;
    let ret = of_property_read_string_index(np, propname, 0, &mut (*card).name);
    if ret < 0 && ret != -EINVAL {
        dev_err((*card).dev, b"ASoC: Property '%s' could not be read: %d\n\0".as_ptr() as *const c_char, propname, ret);
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int {
    let np = (*(*card).dev).of_node;
    let mut num_routes = of_property_count_strings(np, propname);
    if num_routes < 0 || (num_routes & 1) != 0 {
        dev_err((*card).dev, b"ASoC: Property '%s' does not exist or its length is not even\n\0".as_ptr() as *const c_char, propname);
        return -EINVAL;
    }
    num_routes /= 2;
    let routes = devm_kcalloc((*card).dev, num_routes as size_t, size_of::<snd_soc_dapm_route>(), GFP_KERNEL) as *mut snd_soc_dapm_route;
    if routes.is_null() {
        dev_err((*card).dev, b"ASoC: Could not allocate DAPM route table\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }
    let mut i = 0;
    while i < num_routes {
        let ret = of_property_read_string_index(np, propname, 2 * i, &mut (*routes.add(i as usize)).sink);
        if ret != 0 { return -EINVAL; }
        let ret = of_property_read_string_index(np, propname, 2 * i + 1, &mut (*routes.add(i as usize)).source);
        if ret != 0 { return -EINVAL; }
        i += 1;
    }
    (*card).num_of_dapm_routes = num_routes;
    (*card).of_dapm_routes = routes;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_get_dlc(args: *const of_phandle_args, dlc: *mut snd_soc_dai_link_component) -> c_int {
    // for_each_component(pos), translate node/name using component callbacks.
    let ret = -EPROBE_DEFER;
    if ret == 0 { (*dlc).of_node = (*args).np; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_get_dlc(of_node: *mut device_node, mut args: *mut of_phandle_args, dlc: *mut snd_soc_dai_link_component, index: c_int) -> c_int {
    let mut __args: of_phandle_args = zeroed();
    if args.is_null() { args = &mut __args; }
    let ret = of_parse_phandle_with_args(of_node, b"sound-dai\0".as_ptr() as *const c_char, b"#sound-dai-cells\0".as_ptr() as *const c_char, index, args);
    if ret != 0 { return ret; }
    snd_soc_get_dlc(args, dlc)
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_get_dai_name(args: *const of_phandle_args, dai_name: *mut *const c_char) -> c_int {
    let mut dlc: snd_soc_dai_link_component = zeroed();
    let ret = snd_soc_get_dlc(args, &mut dlc);
    if ret == 0 { *dai_name = dlc.dai_name; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_get_dai_name(of_node: *mut device_node, dai_name: *mut *const c_char, index: c_int) -> c_int {
    let mut dlc: snd_soc_dai_link_component = zeroed();
    let ret = snd_soc_of_get_dlc(of_node, null_mut(), &mut dlc, index);
    if ret == 0 { *dai_name = dlc.dai_name; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_get_dai_via_args(_dai_args: *const of_phandle_args) -> *mut snd_soc_dai {
    // for_each_component(component) for_each_component_dais(component, dai)
    null_mut()
}

unsafe fn __snd_soc_of_put_component(component: *mut snd_soc_dai_link_component) {
    if !(*component).of_node.is_null() {
        of_node_put((*component).of_node);
        (*component).of_node = null_mut();
    }
}

unsafe fn __snd_soc_of_get_dai_link_component_alloc(dev: *mut device, of_node: *mut device_node, ret_component: *mut *mut snd_soc_dai_link_component, ret_num: *mut c_int) -> c_int {
    let num = of_count_phandle_with_args(of_node, b"sound-dai\0".as_ptr() as *const c_char, b"#sound-dai-cells\0".as_ptr() as *const c_char);
    if num <= 0 {
        if num == -ENOENT { dev_err(dev, b"No 'sound-dai' property\n\0".as_ptr() as *const c_char); } else { dev_err(dev, b"Bad phandle in 'sound-dai'\n\0".as_ptr() as *const c_char); }
        return num;
    }
    let component = devm_kcalloc(dev, num as size_t, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
    if component.is_null() { return -ENOMEM; }
    *ret_component = component;
    *ret_num = num;
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_put_dai_link_codecs(dai_link: *mut snd_soc_dai_link) {
    let mut index = 0;
    while index < (*dai_link).num_codecs {
        __snd_soc_of_put_component((*dai_link).codecs.add(index as usize));
        index += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_get_dai_link_codecs(dev: *mut device, of_node: *mut device_node, dai_link: *mut snd_soc_dai_link) -> c_int {
    let ret = __snd_soc_of_get_dai_link_component_alloc(dev, of_node, &mut (*dai_link).codecs, &mut (*dai_link).num_codecs);
    if ret < 0 { return ret; }
    let mut index = 0;
    while index < (*dai_link).num_codecs {
        let ret = snd_soc_of_get_dlc(of_node, null_mut(), (*dai_link).codecs.add(index as usize), index);
        if ret != 0 {
            snd_soc_of_put_dai_link_codecs(dai_link);
            (*dai_link).codecs = null_mut();
            (*dai_link).num_codecs = 0;
            return ret;
        }
        index += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_put_dai_link_cpus(dai_link: *mut snd_soc_dai_link) {
    let mut index = 0;
    while index < (*dai_link).num_cpus {
        __snd_soc_of_put_component((*dai_link).cpus.add(index as usize));
        index += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_of_get_dai_link_cpus(dev: *mut device, of_node: *mut device_node, dai_link: *mut snd_soc_dai_link) -> c_int {
    let ret = __snd_soc_of_get_dai_link_component_alloc(dev, of_node, &mut (*dai_link).cpus, &mut (*dai_link).num_cpus);
    if ret < 0 { return ret; }
    let mut index = 0;
    while index < (*dai_link).num_cpus {
        let ret = snd_soc_of_get_dlc(of_node, null_mut(), (*dai_link).cpus.add(index as usize), index);
        if ret != 0 {
            snd_soc_of_put_dai_link_cpus(dai_link);
            (*dai_link).cpus = null_mut();
            (*dai_link).num_cpus = 0;
            return ret;
        }
        index += 1;
    }
    0
}

// Module init/exit translation.
unsafe extern "C" fn snd_soc_init() -> c_int {
    snd_soc_debugfs_init();
    let mut ret = snd_soc_util_init();
    if ret != 0 {
        snd_soc_debugfs_exit();
        return ret;
    }
    ret = platform_driver_register(&mut soc_driver);
    if ret != 0 {
        snd_soc_util_exit();
        snd_soc_debugfs_exit();
        return ret;
    }
    0
}

unsafe extern "C" fn snd_soc_exit() {
    snd_soc_util_exit();
    snd_soc_debugfs_exit();
    platform_driver_unregister(&mut soc_driver);
}

#[no_mangle]
pub static mut snd_soc_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: None,
    resume: None,
    freeze: None,
    thaw: None,
    poweroff: None,
    restore: None,
};

unsafe extern "C" fn soc_probe(pdev: *mut platform_device) -> c_int {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    if card.is_null() { return -EINVAL; }
    dev_warn(&mut (*pdev).dev, b"ASoC: machine %s should use snd_soc_register_card()\n\0".as_ptr() as *const c_char, (*card).name);
    (*card).dev = &mut (*pdev).dev;
    snd_soc_register_card(card)
}

static mut soc_driver: platform_driver = platform_driver {
    driver: device_driver { name: b"soc-audio\0".as_ptr() as *const c_char, pm: unsafe { &snd_soc_pm_ops } },
    probe: Some(soc_probe),
};

#[no_mangle]
pub unsafe extern "C" fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int {
    if (*card).name.is_null() || (*card).dev.is_null() { return -EINVAL; }
    (*card).dapm = snd_soc_dapm_alloc((*card).dev);
    if (*card).dapm.is_null() { return -ENOMEM; }
    dev_set_drvdata((*card).dev, card as *mut c_void);
    INIT_LIST_HEAD(&mut (*card).widgets);
    INIT_LIST_HEAD(&mut (*card).paths);
    INIT_LIST_HEAD(&mut (*card).dapm_list);
    INIT_LIST_HEAD(&mut (*card).aux_comp_list);
    INIT_LIST_HEAD(&mut (*card).component_dev_list);
    INIT_LIST_HEAD(&mut (*card).list);
    INIT_LIST_HEAD(&mut (*card).rtd_list);
    INIT_LIST_HEAD(&mut (*card).dapm_dirty);
    (*card).instantiated = false;
    mutex_init(&mut (*card).mutex);
    mutex_init(&mut (*card).dapm_mutex);
    mutex_init(&mut (*card).pcm_mutex);
    snd_soc_bind_card(card)
}

unsafe fn snd_soc_bind_card(_card: *mut snd_soc_card) -> c_int {
    // Literal binding sequence from C depends on for_each_card_* and for_each_component macros.
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_unregister_card(card: *mut snd_soc_card) {
    if snd_soc_card_is_instantiated(card) != 0 {
        (*card).instantiated = false;
    }
    list_del(&mut (*card).list);
    dev_dbg((*card).dev, b"ASoC: Unregistered card '%s'\n\0".as_ptr() as *const c_char, (*card).name);
}

// Module information:
// MODULE_AUTHOR("Liam Girdwood, lrg@slimlogic.co.uk");
// MODULE_DESCRIPTION("ALSA SoC Core");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:soc-audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
