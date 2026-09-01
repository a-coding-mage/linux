// SPDX-License-Identifier: GPL-2.0-only
//
// Driver for Cirrus Logic CS35L56 smart amp
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct regulator_bulk_data { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_slave { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_component { pub card: *mut snd_soc_card, pub debugfs_root: *mut dentry, pub name_prefix: *const c_char }
#[repr(C)]
pub struct snd_soc_card { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)]
pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai { pub component: *mut snd_soc_component, pub dev: *mut device }
#[repr(C)]
pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct sdw_stream_runtime { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)]
pub struct completion { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct file { pub private_data: *mut c_void }
#[repr(C)]
pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)]
pub struct acpi_device { _private: [u8; 0] }
#[repr(C)]
pub struct acpi_object_package { pub count: c_uint }
#[repr(C)]
pub union acpi_object_data { pub package: core::mem::ManuallyDrop<acpi_object_package> }
#[repr(C)]
pub struct acpi_object { pub data: acpi_object_data }
#[repr(C)]
pub struct acpi_gpio_params { pub crs_entry_index: c_uint, pub line_index: c_uint, pub active_low: bool }
#[repr(C)]
pub struct acpi_gpio_mapping { pub name: *const c_char, pub data: *const acpi_gpio_params, pub size: c_uint }

#[repr(C)]
pub struct cs35l56_fw_reg {
    pub transducer_actual_ps: c_uint,
    pub prot_sts: c_uint,
}

#[repr(C)]
pub struct cirrus_amp_cal_data { _private: [u8; 0] }
#[repr(C)]
pub struct cs35l56_base {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub irq_lock: mutex,
    pub secured: bool,
    pub cal_data_valid: bool,
    pub calibration_controls: *mut c_void,
    pub cal_data: cirrus_amp_cal_data,
    pub init_done: bool,
    pub can_hibernate: bool,
    pub fw_patched: bool,
    pub fw_reg: *mut cs35l56_fw_reg,
    pub rev: c_uint,
    pub type_: c_uint,
    pub reset_gpio: *mut gpio_desc,
    pub cal_index: c_int,
    pub num_onchip_spkid_gpios: c_int,
    pub onchip_spkid_gpios: [u32; 5],
    pub onchip_spkid_pulls: [u32; 5],
}

#[repr(C)]
pub struct cs_dsp { pub booted: bool }
#[repr(C)]
pub struct cs_dsp_coeff_ctl { _private: [u8; 0] }
#[repr(C)]
pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub bin_mandatory: bool,
    pub fwf_suffix: *const c_char,
    pub fwf_name: *mut c_char,
    pub system_name: *mut c_char,
    pub part: *mut c_char,
    pub fw: c_int,
    pub wmfw_optional: bool,
    pub control_add: Option<unsafe extern "C" fn(*mut wm_adsp, *mut cs_dsp_coeff_ctl) -> c_int>,
}

#[repr(C)]
pub struct cs35l56_private {
    pub base: cs35l56_base,
    pub sdw_peripheral: *mut sdw_slave,
    pub dsp_work: work_struct,
    pub dsp: wm_adsp,
    pub dsp_wq: *mut workqueue_struct,
    pub init_completion: completion,
    pub tdm_mode: bool,
    pub asp_slot_width: u8,
    pub asp_slot_count: u8,
    pub sysclk_set: bool,
    pub rx_mask: c_uint,
    pub tx_mask: c_uint,
    pub fallback_fw_suffix: *const c_char,
    pub component: *mut snd_soc_component,
    pub speaker_id: c_int,
    pub sdw_link_num: c_uint,
    pub sdw_unique_id: c_uint,
    pub ambient_ctl_value: c_int,
    pub supplies: [regulator_bulk_data; 2],
    pub soft_resetting: bool,
}

#[repr(C)]
pub struct snd_ctl_elem_value_value_bytes { pub data: [u8; 512] }
#[repr(C)]
pub struct snd_ctl_elem_value_value_integer { pub value: [i64; 128] }
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub bytes: core::mem::ManuallyDrop<snd_ctl_elem_value_value_bytes>,
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_value_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }

#[repr(C)]
pub struct sdw_stream_config {
    pub frame_rate: c_uint,
    pub bps: c_int,
    pub direction: c_int,
    pub ch_count: c_uint,
}
#[repr(C)]
pub struct sdw_port_config { pub num: c_uint, pub ch_mask: c_uint }

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, *mut c_void, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_dai_stream,
    pub capture: snd_soc_dai_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub suspend_bias_off: c_uint,
}

pub type snd_soc_bias_level = c_int;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENODATA: c_int = 61;
const ENXIO: c_int = 6;
const EACCES: c_int = 13;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EOVERFLOW: c_int = 75;
const EBUSY: c_int = 16;
const GFP_KERNEL: c_uint = 0;
const NULL_CTL: *const c_char = null();

/* Include dependencies from the C source are external kernel, ALSA, SoundWire,
 * Cirrus shared driver, and wm_adsp APIs. The local macros used to construct
 * ALSA controls/widgets are preserved below as comments where they have no
 * file-local Rust equivalent.
 */
unsafe extern "C" {
    fn sdw_write_no_pm(p: *mut sdw_slave, reg: c_uint, val: c_uint) -> c_int;
    fn sdw_read_no_pm(p: *mut sdw_slave, reg: c_uint) -> c_int;
    fn disable_irq(irq: c_int);
    fn enable_irq(irq: c_int);
    fn flush_work(work: *mut work_struct);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cs35l56_private;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_put_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn cs35l56_cal_set_status_get(base: *mut cs35l56_base, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn regmap_set_bits(map: *mut regmap, reg: c_uint, mask: c_uint) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn cs35l56_mbox_send(base: *mut cs35l56_base, cmd: c_uint) -> c_int;
    fn wm_adsp_event(w: *mut snd_soc_dapm_widget, k: *mut snd_kcontrol, event: c_int) -> c_int;
    fn cs35l56_set_asp_patch(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_get_bclk_freq_id(freq: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> u8;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_dai_set_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream, data: *mut c_void);
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut sdw_stream_runtime;
    fn snd_soc_dai_dma_data_set(dai: *mut snd_soc_dai, direction: c_int, data: *mut c_void);
    fn sdw_stream_add_slave(p: *mut sdw_slave, s: *mut sdw_stream_config, pc: *mut sdw_port_config, n: c_int, stream: *mut sdw_stream_runtime) -> c_int;
    fn sdw_stream_remove_slave(p: *mut sdw_slave, stream: *mut sdw_stream_runtime);
    fn wm_adsp_run(dsp: *mut wm_adsp) -> c_int;
    fn wm_adsp_stop(dsp: *mut wm_adsp);
    fn cs_amp_write_cal_coeffs(dsp: *mut cs_dsp, ctrls: *mut c_void, data: *const cirrus_amp_cal_data) -> c_int;
    fn wm_adsp_power_up(dsp: *mut wm_adsp, load_firmware: bool) -> c_int;
    fn wm_adsp_power_down(dsp: *mut wm_adsp);
    fn cs35l56_firmware_shutdown(base: *mut cs35l56_base) -> c_int;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn reinit_completion(c: *mut completion);
    fn init_completion(c: *mut completion);
    fn complete_all(c: *mut completion);
    fn wait_for_completion_timeout(c: *mut completion, timeout: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(ms: c_uint) -> c_ulong;
    fn cs35l56_system_reset(base: *mut cs35l56_base, is_sdw: bool);
    fn cs35l56_read_prot_status(base: *mut cs35l56_base, missing: *mut bool, version: *mut c_uint) -> c_int;
    fn cs35l56_warn_if_firmware_missing(base: *mut cs35l56_base);
    fn cs35l56_log_tuning(base: *mut cs35l56_base, dsp: *mut cs_dsp);
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn cs35l56_calibrate_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_ambient_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_data_debugfs_read(base: *mut cs35l56_base, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_cal_data_debugfs_write(base: *mut cs35l56_base, from: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    fn cs35l56_stash_calibration(base: *mut cs35l56_base, data: *const cirrus_amp_cal_data) -> c_int;
    fn cs_amp_write_ambient_temp(dsp: *mut cs_dsp, ctrls: *mut c_void, temperature: c_int) -> c_int;
    fn cs35l56_factory_calibrate(base: *mut cs35l56_base) -> c_int;
    fn snd_soc_card_get_pci_ssid(card: *mut snd_soc_card, vendor: *mut u16, device: *mut u16) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kasprintf(gfp: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn cs_amp_devm_get_vendor_specific_variant_id(dev: *mut device, vendor: c_int, device: c_int) -> *const c_char;
    fn PTR_ERR_OR_ZERO(p: *const c_char) -> c_int;
    fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_component_remove(dsp: *mut wm_adsp, component: *mut snd_soc_component);
    fn wm_adsp2_remove(dsp: *mut wm_adsp);
    fn debugfs_create_bool(name: *const c_char, mode: c_uint, root: *mut dentry, value: *mut bool);
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, n: c_uint) -> c_int;
    fn cs35l56_create_cal_debugfs(base: *mut cs35l56_base, fops: *const cs35l56_cal_debugfs_fops);
    fn cs35l56_remove_cal_debugfs(base: *mut cs35l56_base);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn dev_get_drvdata(dev: *mut device) -> *mut cs35l56_private;
    fn dev_set_drvdata(dev: *mut device, data: *mut cs35l56_private);
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn regulator_bulk_disable(n: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(n: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn cs35l56_wait_min_reset_pulse();
    fn cs35l56_wait_control_port_ready();
    fn cs35l56_is_fw_reload_needed(base: *mut cs35l56_base) -> c_int;
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn INIT_WORK(work: *mut work_struct, f: unsafe extern "C" fn(*mut work_struct));
    fn cs35l56_init_cs_dsp(base: *mut cs35l56_base, dsp: *mut cs_dsp);
    fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;
    fn fwnode_property_count_u32(node: *mut fwnode_handle, name: *const c_char) -> c_int;
    fn fwnode_get_name(node: *mut fwnode_handle) -> *const c_char;
    fn fwnode_property_read_u32_array(node: *mut fwnode_handle, name: *const c_char, dest: *mut u32, count: c_int) -> c_int;
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn fwnode_get_named_child_node(node: *mut fwnode_handle, name: *const c_char) -> *mut fwnode_handle;
    fn fwnode_handle_put(node: *mut fwnode_handle);
    fn cs35l56_check_and_save_onchip_spkid_gpios(base: *mut cs35l56_base, gpios: *mut u32, num_gpios: c_int, pulls: *mut u32, num_pulls: c_int) -> c_int;
    fn device_property_read_string(dev: *mut device, name: *const c_char, prop: *mut *const c_char) -> c_int;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, gfp: c_uint) -> *mut c_char;
    fn device_get_named_child_node(dev: *mut device, name: *const c_char) -> *mut fwnode_handle;
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn acpi_dev_get_property(adev: *mut acpi_device, name: *const c_char, typ: c_int, obj: *mut *const acpi_object) -> c_int;
    fn fwnode_property_present(node: *mut fwnode_handle, name: *const c_char) -> bool;
    fn to_acpi_device_node(node: *mut fwnode_handle) -> *mut acpi_device;
    fn acpi_dev_add_driver_gpios(adev: *mut acpi_device, mapping: *const acpi_gpio_mapping) -> c_int;
    fn acpi_dev_remove_driver_gpios(adev: *mut c_void);
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn fwnode_gpiod_get_index(node: *mut fwnode_handle, con_id: *const c_char, index: c_uint, flags: c_int, label: *const c_char) -> *mut gpio_desc;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_put(desc: *mut gpio_desc);
    fn cs35l56_fill_supply_names(supplies: *mut regulator_bulk_data);
    fn devm_regulator_bulk_get(dev: *mut device, n: c_uint, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn cs35l56_get_speaker_id(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_hw_init(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_set_patch(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_get_calibration(base: *mut cs35l56_base) -> c_int;
    fn cs35l56_irq_request(base: *mut cs35l56_base, irq: c_int) -> c_int;
    fn snd_soc_register_component(dev: *mut device, driver: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver, n: c_uint) -> c_int;
    fn snd_soc_unregister_component(dev: *mut device);
    fn devm_free_irq(dev: *mut device, irq: c_int, data: *mut c_void);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
    fn pm_runtime_dont_use_autosuspend(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_get_noresume(dev: *mut device);
    fn pm_runtime_put_noidle(dev: *mut device);
    fn dev_err_probe(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
unsafe fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
unsafe fn ERR_PTR<T>(err: c_int) -> *mut T { err as isize as *mut T }
fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint { N as c_uint }
fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 { (!0u32) << l } else { (((1u32 << (h + 1)) - 1) & !((1u32 << l) - 1)) }
}
fn hweight32(x: c_uint) -> c_uint { x.count_ones() }
fn round_up(x: c_uint, y: c_uint) -> c_uint { ((x + y - 1) / y) * y }

/* External constants from included headers. */
unsafe extern "C" {
    static CS35L56_SDW_GEN_INT_MASK_1: c_uint;
    static CS35L56_SDW_GEN_INT_STAT_1: c_uint;
    static CS35L56_SDW_INT_MASK_CODEC_IRQ: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static CS35L56_DSP_VIRTUAL1_MBOX_1: c_uint;
    static CS35L56_MBOX_CMD_AUDIO_PLAY: c_uint;
    static CS35L56_MBOX_CMD_AUDIO_PAUSE: c_uint;
    static CS35L56_MBOX_CMD_AUDIO_REINIT: c_uint;
    static CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE: c_uint;
    static CS35L56_PS0: c_uint;
    static CS35L56_PS0_POLL_US: c_uint;
    static CS35L56_PS0_TIMEOUT_US: c_uint;
    static CS35L56_ASP1_CONTROL1: c_uint;
    static CS35L56_ASP1_CONTROL2: c_uint;
    static CS35L56_ASP1_CONTROL3: c_uint;
    static CS35L56_ASP1_FRAME_CONTROL1: c_uint;
    static CS35L56_ASP1_FRAME_CONTROL5: c_uint;
    static CS35L56_ASP1_DATA_CONTROL1: c_uint;
    static CS35L56_ASP1_DATA_CONTROL5: c_uint;
    static CS35L56_ASP_FMT_MASK: c_uint;
    static CS35L56_ASP_FMT_DSP_A: c_uint;
    static CS35L56_ASP_FMT_I2S: c_uint;
    static CS35L56_ASP_FMT_SHIFT: c_uint;
    static CS35L56_ASP_FSYNC_INV_MASK: c_uint;
    static CS35L56_ASP_BCLK_INV_MASK: c_uint;
    static CS35L56_ASP1_DOUT_HIZ_CTRL_MASK: c_uint;
    static CS35L56_ASP_UNUSED_HIZ_OFF_HIZ: c_uint;
    static CS35L56_ASP_RX_WIDTH_MASK: c_uint;
    static CS35L56_ASP_RX_WIDTH_SHIFT: c_uint;
    static CS35L56_ASP_TX_WIDTH_MASK: c_uint;
    static CS35L56_ASP_TX_WIDTH_SHIFT: c_uint;
    static CS35L56_ASP_RX_WL_MASK: c_uint;
    static CS35L56_ASP_TX_WL_MASK: c_uint;
    static CS35L56_ASP_BCLK_FREQ_MASK: c_uint;
    static CS35L56_ASP_BCLK_FREQ_SHIFT: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SDW_DATA_DIR_RX: c_int;
    static SDW_DATA_DIR_TX: c_int;
    static CS35L56_SDW1_PLAYBACK_PORT: c_uint;
    static CS35L56_SDW1_CAPTURE_PORT: c_uint;
    static CS35L56_RATES: c_uint;
    static CS35L56_RX_FORMATS: u64;
    static CS35L56_TX_FORMATS: u64;
    static CS35L56_FIRMWARE_MISSING: c_uint;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static CS35L56_FW_REQ_ACTIVE_TIMEOUT_MS: c_int;
    static ACPI_TYPE_PACKAGE: c_int;
    static GPIOD_IN: c_int;
    static GPIOD_OUT_LOW: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_mask_soundwire_interrupts(cs35l56: *mut cs35l56_private) {
    /*
     * Mask unconditionally.
     *
     * The read of GEN_INT_STAT_1 is required as per the SoundWire spec
     * for interrupt status bits to clear.
     * GEN_INT_MASK_1 masks the _inputs_ to GEN_INT_STAT1.
     */
    unsafe {
        sdw_write_no_pm((*cs35l56).sdw_peripheral, CS35L56_SDW_GEN_INT_MASK_1, 0);
        sdw_read_no_pm((*cs35l56).sdw_peripheral, CS35L56_SDW_GEN_INT_STAT_1);
        sdw_write_no_pm((*cs35l56).sdw_peripheral, CS35L56_SDW_GEN_INT_STAT_1, 0xFF);
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_unmask_soundwire_interrupts(cs35l56: *mut cs35l56_private) {
    unsafe {
        if (*cs35l56).base.irq == 0 { return; }
        sdw_write_no_pm((*cs35l56).sdw_peripheral, CS35L56_SDW_GEN_INT_MASK_1,
                        CS35L56_SDW_INT_MASK_CODEC_IRQ);
    }
}

unsafe extern "C" fn cs35l56_disable_sdw_interrupts(cs35l56: *mut cs35l56_private) {
    unsafe {
        if (*cs35l56).sdw_peripheral.is_null() { return; }
        cs35l56_mask_soundwire_interrupts(cs35l56);
        if (*cs35l56).base.irq != 0 { disable_irq((*cs35l56).base.irq); }
    }
}

unsafe extern "C" fn cs35l56_enable_sdw_interrupts(cs35l56: *mut cs35l56_private) {
    unsafe {
        if (*cs35l56).sdw_peripheral.is_null() || (*cs35l56).base.irq == 0 { return; }
        enable_irq((*cs35l56).base.irq);
        cs35l56_unmask_soundwire_interrupts(cs35l56);
    }
}

unsafe extern "C" fn cs35l56_wait_dsp_ready(cs35l56: *mut cs35l56_private) {
    /* Wait for patching to complete */
    unsafe { flush_work(&mut (*cs35l56).dsp_work); }
}

unsafe extern "C" fn cs35l56_dspwait_get_volsw(kcontrol: *mut snd_kcontrol,
                                                ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        cs35l56_wait_dsp_ready(cs35l56);
        snd_soc_get_volsw(kcontrol, ucontrol)
    }
}

unsafe extern "C" fn cs35l56_dspwait_put_volsw(kcontrol: *mut snd_kcontrol,
                                                ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        cs35l56_wait_dsp_ready(cs35l56);
        snd_soc_put_volsw(kcontrol, ucontrol)
    }
}

/* static DECLARE_TLV_DB_SCALE(vol_tlv, -10000, 25, 0); */
static vol_tlv: [c_uint; 4] = [0, (-10000i32) as c_uint, 25, 0];

/* static SOC_ENUM_SINGLE_DECL(cs35l56_cal_set_status_enum, SND_SOC_NOPM, 0,
 *                             cs35l56_cal_set_status_text);
 */

unsafe extern "C" fn cs35l56_cal_set_status_ctl_get(kcontrol: *mut snd_kcontrol,
                                                    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        cs35l56_cal_set_status_get(&mut (*cs35l56).base, ucontrol)
    }
}

/* ALSA control macro initializers translated as dependency-preserving comments:
 * cs35l56_controls[]:
 *   SOC_SINGLE_EXT("Speaker Switch", CS35L56_MAIN_RENDER_USER_MUTE, 0, 1, 1,
 *                  cs35l56_dspwait_get_volsw, cs35l56_dspwait_put_volsw)
 *   SOC_SINGLE_S_EXT_TLV("Speaker Volume", CS35L56_MAIN_RENDER_USER_VOLUME,
 *                  CS35L56_MAIN_RENDER_USER_VOLUME_SHIFT,
 *                  CS35L56_MAIN_RENDER_USER_VOLUME_MIN,
 *                  CS35L56_MAIN_RENDER_USER_VOLUME_MAX,
 *                  CS35L56_MAIN_RENDER_USER_VOLUME_SIGNBIT, 0,
 *                  cs35l56_dspwait_get_volsw, cs35l56_dspwait_put_volsw, vol_tlv)
 *   SOC_SINGLE_EXT("Posture Number", CS35L56_MAIN_POSTURE_NUMBER, 0, 255, 0,
 *                  cs35l56_dspwait_get_volsw, cs35l56_dspwait_put_volsw)
 *   SOC_ENUM_EXT_ACC("CAL_SET_STATUS", cs35l56_cal_set_status_enum,
 *                  cs35l56_cal_set_status_ctl_get, NULL,
 *                  SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE)
 * cs35l63_controls[] mirrors the above using CS35L63_* register constants.
 */
static cs35l56_controls: [snd_kcontrol_new; 0] = [];
static cs35l63_controls: [snd_kcontrol_new; 0] = [];

/* SOC_VALUE_ENUM_SINGLE_DECL mux declarations for ASP1TX1..4 and SDW1TX1..4,
 * plus SOC_DAPM_ENUM controls asp1_tx1_mux..sdw1_tx4_mux, are macro-created in C.
 */
static asp1_tx1_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static asp1_tx2_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static asp1_tx3_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static asp1_tx4_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sdw1_tx1_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sdw1_tx2_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sdw1_tx3_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static sdw1_tx4_mux: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn cs35l56_play_event(w: *mut snd_soc_dapm_widget,
                                         _kcontrol: *mut snd_kcontrol,
                                         event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let mut val: c_uint = 0;
        let mut ret: c_int;

        dev_dbg((*cs35l56).base.dev, c"play: %d\n".as_ptr(), event);

        if event == SND_SOC_DAPM_PRE_PMU {
            /* Don't wait for ACK, we check in POST_PMU that it completed */
            return regmap_write((*cs35l56).base.regmap, CS35L56_DSP_VIRTUAL1_MBOX_1,
                                CS35L56_MBOX_CMD_AUDIO_PLAY);
        } else if event == SND_SOC_DAPM_POST_PMU {
            /* Wait for firmware to enter PS0 power state */
            ret = regmap_read_poll_timeout((*cs35l56).base.regmap,
                                           (*(*cs35l56).base.fw_reg).transducer_actual_ps,
                                           &mut val, CS35L56_PS0,
                                           CS35L56_PS0_POLL_US,
                                           CS35L56_PS0_TIMEOUT_US);
            if ret != 0 {
                dev_err((*cs35l56).base.dev, c"PS0 wait failed: %d\n".as_ptr(), ret);
            }
            return ret;
        } else if event == SND_SOC_DAPM_POST_PMD {
            return cs35l56_mbox_send(&mut (*cs35l56).base, CS35L56_MBOX_CMD_AUDIO_PAUSE);
        }
        0
    }
}

unsafe fn regmap_read_poll_timeout(map: *mut regmap, reg: c_uint, val: *mut c_uint,
                                   expect: c_uint, _sleep_us: c_uint, _timeout_us: c_uint) -> c_int {
    unsafe extern "C" { fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int; }
    unsafe {
        let ret = regmap_read(map, reg, val);
        if ret != 0 { ret } else if *val == expect { 0 } else { -EINVAL }
    }
}

/* cs35l56_dapm_widgets[] is constructed from SND_SOC_DAPM_* macros:
 * regulator supplies VDD_B/VDD_AMP; PLAY supply with cs35l56_play_event;
 * AMP output driver; SPK output; DSP1 PGA with cs35l56_dsp_event;
 * ASP1RX1/2 inputs; ASP1TX1..4 outputs; ASP1 and SDW1 muxes; VMON/IMON/
 * ERRVOL/CLASSH/VDDBMON/VBSTMON/TEMPMON signal generators; Calibrate input.
 */
static cs35l56_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

macro_rules! route {
    ($sink:literal, $control:expr, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char,
                             control: $control,
                             source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

macro_rules! CS35L56_SRC_ROUTE {
    ($name:literal) => {
        route!(concat!($name, " Source"), concat!("ASP1RX1", "\0").as_ptr() as *const c_char, "ASP1RX1"),
        route!(concat!($name, " Source"), concat!("ASP1RX2", "\0").as_ptr() as *const c_char, "ASP1RX2"),
        route!(concat!($name, " Source"), concat!("VMON", "\0").as_ptr() as *const c_char, "VMON ADC"),
        route!(concat!($name, " Source"), concat!("IMON", "\0").as_ptr() as *const c_char, "IMON ADC"),
        route!(concat!($name, " Source"), concat!("ERRVOL", "\0").as_ptr() as *const c_char, "ERRVOL ADC"),
        route!(concat!($name, " Source"), concat!("CLASSH", "\0").as_ptr() as *const c_char, "CLASSH ADC"),
        route!(concat!($name, " Source"), concat!("VDDBMON", "\0").as_ptr() as *const c_char, "VDDBMON ADC"),
        route!(concat!($name, " Source"), concat!("VBSTMON", "\0").as_ptr() as *const c_char, "VBSTMON ADC"),
        route!(concat!($name, " Source"), concat!("DSP1TX1", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX2", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX3", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX4", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX5", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX6", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX7", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("DSP1TX8", "\0").as_ptr() as *const c_char, "DSP1"),
        route!(concat!($name, " Source"), concat!("TEMPMON", "\0").as_ptr() as *const c_char, "TEMPMON ADC"),
        route!(concat!($name, " Source"), concat!("INTERPOLATOR", "\0").as_ptr() as *const c_char, "AMP"),
        route!(concat!($name, " Source"), concat!("SDW1RX1", "\0").as_ptr() as *const c_char, "SDW1 Playback"),
        route!(concat!($name, " Source"), concat!("SDW1RX2", "\0").as_ptr() as *const c_char, "SDW1 Playback")
    }
}

static cs35l56_audio_map: [snd_soc_dapm_route; 106] = [
    route!("AMP", null(), "VDD_B"),
    route!("AMP", null(), "VDD_AMP"),
    route!("ASP1 Playback", null(), "PLAY"),
    route!("SDW1 Playback", null(), "PLAY"),
    route!("ASP1RX1", null(), "ASP1 Playback"),
    route!("ASP1RX2", null(), "ASP1 Playback"),
    route!("DSP1", null(), "ASP1RX1"),
    route!("DSP1", null(), "ASP1RX2"),
    route!("DSP1", null(), "SDW1 Playback"),
    route!("DSP1", null(), "Calibrate"),
    route!("AMP", null(), "DSP1"),
    route!("SPK", null(), "AMP"),
    CS35L56_SRC_ROUTE!("ASP1 TX1"),
    CS35L56_SRC_ROUTE!("ASP1 TX2"),
    CS35L56_SRC_ROUTE!("ASP1 TX3"),
    CS35L56_SRC_ROUTE!("ASP1 TX4"),
    route!("ASP1TX1", null(), "ASP1 TX1 Source"),
    route!("ASP1TX2", null(), "ASP1 TX2 Source"),
    route!("ASP1TX3", null(), "ASP1 TX3 Source"),
    route!("ASP1TX4", null(), "ASP1 TX4 Source"),
    route!("ASP1 Capture", null(), "ASP1TX1"),
    route!("ASP1 Capture", null(), "ASP1TX2"),
    route!("ASP1 Capture", null(), "ASP1TX3"),
    route!("ASP1 Capture", null(), "ASP1TX4"),
    CS35L56_SRC_ROUTE!("SDW1 TX1"),
    CS35L56_SRC_ROUTE!("SDW1 TX2"),
    CS35L56_SRC_ROUTE!("SDW1 TX3"),
    CS35L56_SRC_ROUTE!("SDW1 TX4"),
    route!("SDW1 Capture", null(), "SDW1 TX1 Source"),
    route!("SDW1 Capture", null(), "SDW1 TX2 Source"),
    route!("SDW1 Capture", null(), "SDW1 TX3 Source"),
    route!("SDW1 Capture", null(), "SDW1 TX4 Source"),
];

unsafe extern "C" fn cs35l56_dsp_event(w: *mut snd_soc_dapm_widget,
                                        kcontrol: *mut snd_kcontrol,
                                        event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        dev_dbg((*cs35l56).base.dev, c"%s: %d\n".as_ptr(), c"cs35l56_dsp_event".as_ptr(), event);
        wm_adsp_event(w, kcontrol, event)
    }
}

unsafe extern "C" fn cs35l56_asp_dai_probe(codec_dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*codec_dai).component);
        cs35l56_set_asp_patch(&mut (*cs35l56).base)
    }
}

unsafe extern "C" fn cs35l56_asp_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*codec_dai).component);
        let mut val: c_uint;
        dev_dbg((*cs35l56).base.dev, c"%s: %#x\n".as_ptr(), c"cs35l56_asp_dai_set_fmt".as_ptr(), fmt);

        if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
            dev_err((*cs35l56).base.dev, c"Unsupported clock source mode\n".as_ptr());
            return -EINVAL;
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_DSP_A => {
                val = CS35L56_ASP_FMT_DSP_A << CS35L56_ASP_FMT_SHIFT;
                (*cs35l56).tdm_mode = true;
            }
            x if x == SND_SOC_DAIFMT_I2S => {
                val = CS35L56_ASP_FMT_I2S << CS35L56_ASP_FMT_SHIFT;
                (*cs35l56).tdm_mode = false;
            }
            _ => {
                dev_err((*cs35l56).base.dev, c"Unsupported DAI format\n".as_ptr());
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_IF => val |= CS35L56_ASP_FSYNC_INV_MASK,
            x if x == SND_SOC_DAIFMT_IB_NF => val |= CS35L56_ASP_BCLK_INV_MASK,
            x if x == SND_SOC_DAIFMT_IB_IF => val |= CS35L56_ASP_BCLK_INV_MASK | CS35L56_ASP_FSYNC_INV_MASK,
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            _ => {
                dev_err((*cs35l56).base.dev, c"Invalid clock invert\n".as_ptr());
                return -EINVAL;
            }
        }

        regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL2,
                           CS35L56_ASP_FMT_MASK | CS35L56_ASP_BCLK_INV_MASK | CS35L56_ASP_FSYNC_INV_MASK,
                           val);

        /* Hi-Z DOUT in unused slots and when all TX are disabled */
        regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL3,
                           CS35L56_ASP1_DOUT_HIZ_CTRL_MASK,
                           CS35L56_ASP_UNUSED_HIZ_OFF_HIZ);
        0
    }
}

unsafe extern "C" fn cs35l56_make_tdm_config_word(mut reg_val: c_uint, mask: c_ulong) -> c_uint {
    let mut channel_shift: c_uint = 0;
    let mut bit_num: c_int = 0;
    /* Enable consecutive TX1..TXn for each of the slots set in mask */
    while bit_num < 32 {
        if ((mask >> bit_num) & 1) != 0 {
            reg_val &= !(0x3f << channel_shift);
            reg_val |= (bit_num as c_uint) << channel_shift;
            channel_shift += 8;
            if channel_shift > 24 { break; }
        }
        bit_num += 1;
    }
    reg_val
}

unsafe extern "C" fn cs35l56_asp_dai_set_tdm_slot(dai: *mut snd_soc_dai, mut tx_mask: c_uint,
                                                   mut rx_mask: c_uint, slots: c_int,
                                                   slot_width: c_int) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        if slots == 0 || slot_width == 0 {
            dev_dbg((*cs35l56).base.dev, c"tdm config cleared\n".as_ptr());
            (*cs35l56).asp_slot_width = 0;
            (*cs35l56).asp_slot_count = 0;
            return 0;
        }
        if slot_width as c_uint > (CS35L56_ASP_RX_WIDTH_MASK >> CS35L56_ASP_RX_WIDTH_SHIFT) {
            dev_err((*cs35l56).base.dev, c"tdm invalid slot width %d\n".as_ptr(), slot_width);
            return -EINVAL;
        }
        /* More than 32 slots would give an unsupportable BCLK frequency */
        if slots > 32 {
            dev_err((*cs35l56).base.dev, c"tdm invalid slot count %d\n".as_ptr(), slots);
            return -EINVAL;
        }

        (*cs35l56).asp_slot_width = slot_width as u8;
        (*cs35l56).asp_slot_count = slots as u8;

        // Note: rx/tx is from point of view of the CPU end
        if tx_mask == 0 { tx_mask = 0x3; } /* ASPRX1/RX2 in slots 0 and 1 */
        if rx_mask == 0 { rx_mask = 0xf; } /* ASPTX1..TX4 in slots 0..3 */

        /* Default unused slots to 63 */
        regmap_write((*cs35l56).base.regmap, CS35L56_ASP1_FRAME_CONTROL1,
                     cs35l56_make_tdm_config_word(0x3f3f3f3f, rx_mask as c_ulong));
        regmap_write((*cs35l56).base.regmap, CS35L56_ASP1_FRAME_CONTROL5,
                     cs35l56_make_tdm_config_word(0x3f3f3f, tx_mask as c_ulong));

        dev_dbg((*cs35l56).base.dev, c"tdm slot width: %u count: %u tx_mask: %#x rx_mask: %#x\n".as_ptr(),
                (*cs35l56).asp_slot_width as c_uint, (*cs35l56).asp_slot_count as c_uint, tx_mask, rx_mask);
        0
    }
}

unsafe extern "C" fn cs35l56_asp_dai_hw_params(substream: *mut snd_pcm_substream,
                                                params: *mut snd_pcm_hw_params,
                                                dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        let rate = params_rate(params);
        let asp_wl = params_width(params);
        let asp_width = if (*cs35l56).asp_slot_width != 0 { (*cs35l56).asp_slot_width } else { asp_wl };

        dev_dbg((*cs35l56).base.dev, c"%s: wl=%d, width=%d, rate=%d".as_ptr(),
                c"cs35l56_asp_dai_hw_params".as_ptr(), asp_wl as c_int, asp_width as c_int, rate);

        if !(*cs35l56).sysclk_set {
            let mut slots = (*cs35l56).asp_slot_count as c_uint;
            if slots == 0 {
                slots = params_channels(params);
                /* I2S always has an even number of slots */
                if !(*cs35l56).tdm_mode { slots = round_up(slots, 2); }
            }
            let bclk_freq = (asp_width as c_uint).wrapping_mul(slots).wrapping_mul(rate);
            let freq_id = cs35l56_get_bclk_freq_id(bclk_freq);
            if freq_id < 0 {
                dev_err((*cs35l56).base.dev, c"%s: Invalid BCLK %u\n".as_ptr(),
                        c"cs35l56_asp_dai_hw_params".as_ptr(), bclk_freq);
                return -EINVAL;
            }
            regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL1,
                               CS35L56_ASP_BCLK_FREQ_MASK,
                               (freq_id as c_uint) << CS35L56_ASP_BCLK_FREQ_SHIFT);
        }

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL2,
                               CS35L56_ASP_RX_WIDTH_MASK,
                               (asp_width as c_uint) << CS35L56_ASP_RX_WIDTH_SHIFT);
            regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_DATA_CONTROL5,
                               CS35L56_ASP_RX_WL_MASK, asp_wl as c_uint);
        } else {
            regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL2,
                               CS35L56_ASP_TX_WIDTH_MASK,
                               (asp_width as c_uint) << CS35L56_ASP_TX_WIDTH_SHIFT);
            regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_DATA_CONTROL1,
                               CS35L56_ASP_TX_WL_MASK, asp_wl as c_uint);
        }
        0
    }
}

unsafe extern "C" fn cs35l56_asp_dai_set_sysclk(dai: *mut snd_soc_dai,
                                                 _clk_id: c_int,
                                                 freq: c_uint,
                                                 _dir: c_int) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        if freq == 0 {
            (*cs35l56).sysclk_set = false;
            return 0;
        }
        let freq_id = cs35l56_get_bclk_freq_id(freq);
        if freq_id < 0 { return freq_id; }
        regmap_update_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL1,
                           CS35L56_ASP_BCLK_FREQ_MASK,
                           (freq_id as c_uint) << CS35L56_ASP_BCLK_FREQ_SHIFT);
        (*cs35l56).sysclk_set = true;
        0
    }
}

static cs35l56_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(cs35l56_asp_dai_probe),
    set_fmt: Some(cs35l56_asp_dai_set_fmt),
    set_tdm_slot: Some(cs35l56_asp_dai_set_tdm_slot),
    hw_params: Some(cs35l56_asp_dai_hw_params),
    set_sysclk: Some(cs35l56_asp_dai_set_sysclk),
    shutdown: None,
    hw_free: None,
    set_stream: None,
};

unsafe extern "C" fn cs35l56_sdw_dai_shutdown(substream: *mut snd_pcm_substream,
                                               dai: *mut snd_soc_dai) {
    unsafe { snd_soc_dai_set_dma_data(dai, substream, null_mut()); }
}

unsafe extern "C" fn cs35l56_sdw_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint,
                                                   rx_mask: c_uint, _slots: c_int,
                                                   _slot_width: c_int) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        /* rx/tx are from point of view of the CPU end so opposite to our rx/tx */
        (*cs35l56).rx_mask = tx_mask;
        (*cs35l56).tx_mask = rx_mask;
        0
    }
}

unsafe extern "C" fn cs35l56_sdw_dai_hw_params(substream: *mut snd_pcm_substream,
                                                params: *mut snd_pcm_hw_params,
                                                dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
        let mut sconfig = sdw_stream_config { frame_rate: 0, bps: 0, direction: 0, ch_count: 0 };
        let mut pconfig = sdw_port_config { num: 0, ch_mask: 0 };

        dev_dbg((*cs35l56).base.dev, c"%s: rate %d\n".as_ptr(),
                c"cs35l56_sdw_dai_hw_params".as_ptr(), params_rate(params));

        if !(*cs35l56).base.init_done { return -ENODEV; }
        if sdw_stream.is_null() { return -EINVAL; }

        sconfig.frame_rate = params_rate(params);
        sconfig.bps = snd_pcm_format_width(params_format(params));

        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            sconfig.direction = SDW_DATA_DIR_RX;
            pconfig.num = CS35L56_SDW1_PLAYBACK_PORT;
            pconfig.ch_mask = (*cs35l56).rx_mask;
        } else {
            sconfig.direction = SDW_DATA_DIR_TX;
            pconfig.num = CS35L56_SDW1_CAPTURE_PORT;
            pconfig.ch_mask = (*cs35l56).tx_mask;
        }

        if pconfig.ch_mask == 0 {
            sconfig.ch_count = params_channels(params);
            pconfig.ch_mask = GENMASK(sconfig.ch_count - 1, 0);
        } else {
            sconfig.ch_count = hweight32(pconfig.ch_mask);
        }

        let ret = sdw_stream_add_slave((*cs35l56).sdw_peripheral, &mut sconfig, &mut pconfig,
                                       1, sdw_stream);
        if ret != 0 {
            dev_err((*dai).dev, c"Failed to add sdw stream: %d\n".as_ptr(), ret);
            return ret;
        }
        0
    }
}

unsafe extern "C" fn cs35l56_sdw_dai_hw_free(substream: *mut snd_pcm_substream,
                                              dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata((*dai).component);
        let sdw_stream = snd_soc_dai_get_dma_data(dai, substream);
        if (*cs35l56).sdw_peripheral.is_null() { return -EINVAL; }
        sdw_stream_remove_slave((*cs35l56).sdw_peripheral, sdw_stream);
        0
    }
}

unsafe extern "C" fn cs35l56_sdw_dai_set_stream(dai: *mut snd_soc_dai,
                                                 sdw_stream: *mut c_void,
                                                 direction: c_int) -> c_int {
    unsafe { snd_soc_dai_dma_data_set(dai, direction, sdw_stream); }
    0
}

static cs35l56_sdw_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: None,
    set_fmt: None,
    set_tdm_slot: Some(cs35l56_sdw_dai_set_tdm_slot),
    hw_params: Some(cs35l56_sdw_dai_hw_params),
    set_sysclk: None,
    shutdown: Some(cs35l56_sdw_dai_shutdown),
    hw_free: Some(cs35l56_sdw_dai_hw_free),
    set_stream: Some(cs35l56_sdw_dai_set_stream),
};

static mut cs35l56_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: c"cs35l56-asp1".as_ptr(),
        id: 0,
        playback: snd_soc_dai_stream { stream_name: c"ASP1 Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        capture: snd_soc_dai_stream { stream_name: c"ASP1 Capture".as_ptr(), channels_min: 1, channels_max: 4, rates: 0, formats: 0 },
        ops: &cs35l56_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: c"cs35l56-sdw1".as_ptr(),
        id: 1,
        playback: snd_soc_dai_stream { stream_name: c"SDW1 Playback".as_ptr(), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        capture: snd_soc_dai_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        ops: &cs35l56_sdw_dai_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 0,
    },
    snd_soc_dai_driver {
        name: c"cs35l56-sdw1c".as_ptr(),
        id: 2,
        playback: snd_soc_dai_stream { stream_name: null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 },
        capture: snd_soc_dai_stream { stream_name: c"SDW1 Capture".as_ptr(), channels_min: 1, channels_max: 4, rates: 0, formats: 0 },
        ops: &cs35l56_sdw_dai_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 0,
    },
];

unsafe extern "C" fn cs35l56_write_cal(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        if (*cs35l56).base.secured || !(*cs35l56).base.cal_data_valid { return -ENODATA; }
        let mut ret = wm_adsp_run(&mut (*cs35l56).dsp);
        if ret != 0 { return ret; }
        ret = cs_amp_write_cal_coeffs(&mut (*cs35l56).dsp.cs_dsp,
                                      (*cs35l56).base.calibration_controls,
                                      &(*cs35l56).base.cal_data);
        wm_adsp_stop(&mut (*cs35l56).dsp);
        if ret == 0 { dev_info((*cs35l56).base.dev, c"Calibration applied\n".as_ptr()); }
        ret
    }
}

unsafe extern "C" fn cs35l56_dsp_download_and_power_up(cs35l56: *mut cs35l56_private,
                                                        load_firmware: bool) -> c_int {
    unsafe {
        /*
         * Abort the first load if it didn't find the suffixed bins and
         * we have an alternate fallback suffix.
         */
        (*cs35l56).dsp.bin_mandatory = load_firmware && !(*cs35l56).fallback_fw_suffix.is_null();
        let mut ret = wm_adsp_power_up(&mut (*cs35l56).dsp, load_firmware);
        if ret == -ENOENT && (*cs35l56).dsp.bin_mandatory {
            (*cs35l56).dsp.fwf_suffix = (*cs35l56).fallback_fw_suffix;
            (*cs35l56).fallback_fw_suffix = null();
            (*cs35l56).dsp.bin_mandatory = false;
            ret = wm_adsp_power_up(&mut (*cs35l56).dsp, load_firmware);
        }
        if ret != 0 {
            dev_dbg((*cs35l56).base.dev, c"wm_adsp_power_up ret %d\n".as_ptr(), ret);
            return ret;
        }
        0
    }
}

unsafe extern "C" fn cs35l56_reinit_patch(cs35l56: *mut cs35l56_private) {
    unsafe {
        let ret = cs35l56_dsp_download_and_power_up(cs35l56, true);
        if ret != 0 { return; }
        cs35l56_write_cal(cs35l56);
        /* Always REINIT after applying patch or coefficients */
        cs35l56_mbox_send(&mut (*cs35l56).base, CS35L56_MBOX_CMD_AUDIO_REINIT);
    }
}

unsafe extern "C" fn cs35l56_patch(cs35l56: *mut cs35l56_private, firmware_missing: bool) {
    unsafe {
        /* Disable SoundWire interrupts to prevent race with IRQ handler thread */
        cs35l56_disable_sdw_interrupts(cs35l56);
        let mut ret = cs35l56_firmware_shutdown(&mut (*cs35l56).base);
        if ret == 0 {
            /*
             * Use wm_adsp to load and apply the firmware patch and coefficient files,
             * but only if firmware is missing. If firmware is already patched just
             * power-up wm_adsp without downloading firmware.
             */
            ret = cs35l56_dsp_download_and_power_up(cs35l56, firmware_missing);
        }
        if ret != 0 {
            cs35l56_enable_sdw_interrupts(cs35l56);
            return;
        }

        mutex_lock(&mut (*cs35l56).base.irq_lock);
        reinit_completion(&mut (*cs35l56).init_completion);
        (*cs35l56).soft_resetting = true;
        cs35l56_system_reset(&mut (*cs35l56).base, !(*cs35l56).sdw_peripheral.is_null());

        let mut ok = true;
        if !(*cs35l56).sdw_peripheral.is_null() {
            /*
             * The system-reset causes the CS35L56 to detach from the bus.
             * Wait for the manager to re-enumerate the CS35L56 and
             * cs35l56_init() to run again.
             */
            if wait_for_completion_timeout(&mut (*cs35l56).init_completion, msecs_to_jiffies(5000)) == 0 {
                dev_err((*cs35l56).base.dev, c"%s: init_completion timed out (SDW)\n".as_ptr(),
                        c"cs35l56_patch".as_ptr());
                ok = false;
            }
        } else if cs35l56_init(cs35l56) != 0 {
            ok = false;
        }

        if ok {
            /* Check if the firmware is still reported missing */
            cs35l56_warn_if_firmware_missing(&mut (*cs35l56).base);
            regmap_clear_bits((*cs35l56).base.regmap, (*(*cs35l56).base.fw_reg).prot_sts,
                              CS35L56_FIRMWARE_MISSING);
            (*cs35l56).base.fw_patched = true;
            if cs35l56_write_cal(cs35l56) == 0 {
                cs35l56_mbox_send(&mut (*cs35l56).base, CS35L56_MBOX_CMD_AUDIO_REINIT);
            }
        }
        mutex_unlock(&mut (*cs35l56).base.irq_lock);
        cs35l56_enable_sdw_interrupts(cs35l56);
    }
}

unsafe extern "C" fn cs35l56_dsp_work(work: *mut work_struct) {
    unsafe {
        let cs35l56 = (work as *mut u8).sub(offset_of_dsp_work()) as *mut cs35l56_private;
        let mut firmware_version: c_uint = 0;
        let mut firmware_missing = false;

        if !(*cs35l56).base.init_done { return; }

        let ret = cs35l56_read_prot_status(&mut (*cs35l56).base, &mut firmware_missing,
                                           &mut firmware_version);
        if ret != 0 { return; }

        /* Populate fw file qualifier with the revision and security state */
        kfree((*cs35l56).dsp.fwf_name as *mut c_void);
        if firmware_missing {
            (*cs35l56).dsp.fwf_name = kasprintf(GFP_KERNEL, c"%02x-dsp1".as_ptr(), (*cs35l56).base.rev);
        } else {
            /* Firmware files must match the running firmware version */
            (*cs35l56).dsp.fwf_name = kasprintf(GFP_KERNEL, c"%02x%s-%06x-dsp1".as_ptr(),
                                                (*cs35l56).base.rev,
                                                if (*cs35l56).base.secured { c"-s".as_ptr() } else { c"".as_ptr() },
                                                firmware_version);
        }
        if (*cs35l56).dsp.fwf_name.is_null() { return; }

        dev_dbg((*cs35l56).base.dev, c"DSP fwf name: '%s' system name: '%s'\n".as_ptr(),
                (*cs35l56).dsp.fwf_name, (*cs35l56).dsp.system_name);

        /*
         * The firmware cannot be patched if it is already running from
         * patch RAM. In this case the firmware files are versioned to
         * match the running firmware version and will only contain
         * tunings. We do not need to shutdown the firmware to apply
         * tunings so can use the lower cost reinit sequence instead.
         */
        if !firmware_missing { cs35l56_reinit_patch(cs35l56); } else { cs35l56_patch(cs35l56, firmware_missing); }
        cs35l56_log_tuning(&mut (*cs35l56).base, &mut (*cs35l56).dsp.cs_dsp);
    }
}

fn offset_of_dsp_work() -> usize { 0 /* container_of placeholder for file-local translation */ }

unsafe extern "C" fn cs35l56_power_up_for_cal(cs35l56: *mut cs35l56_private) -> *mut snd_soc_dapm_context {
    unsafe {
        let dapm = snd_soc_component_to_dapm((*cs35l56).component);
        let ret = snd_soc_dapm_enable_pin(dapm, c"Calibrate".as_ptr());
        if ret != 0 { return ERR_PTR(ret); }
        snd_soc_dapm_sync(dapm);
        dapm
    }
}

unsafe extern "C" fn cs35l56_power_down_after_cal(cs35l56: *mut cs35l56_private) {
    unsafe {
        let dapm = snd_soc_component_to_dapm((*cs35l56).component);
        snd_soc_dapm_disable_pin(dapm, c"Calibrate".as_ptr());
        snd_soc_dapm_sync(dapm);
    }
}

unsafe extern "C" fn cs35l56_private_from_base(base: *mut cs35l56_base) -> *mut cs35l56_private {
    base as *mut cs35l56_private
}

unsafe extern "C" fn cs35l56_debugfs_calibrate_write(file: *mut file, from: *const c_char,
                                                      count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        let cs35l56 = cs35l56_private_from_base(cs35l56_base);
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm) as ssize_t; }
        snd_soc_dapm_mutex_lock(dapm);
        let ret = cs35l56_calibrate_debugfs_write(&mut (*cs35l56).base, from, count, ppos);
        snd_soc_dapm_mutex_unlock(dapm);
        cs35l56_power_down_after_cal(cs35l56);
        ret
    }
}

unsafe extern "C" fn cs35l56_debugfs_cal_temperature_write(file: *mut file, from: *const c_char,
                                                           count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        let cs35l56 = cs35l56_private_from_base(cs35l56_base);
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm) as ssize_t; }
        let ret = cs35l56_cal_ambient_debugfs_write(&mut (*cs35l56).base, from, count, ppos);
        cs35l56_power_down_after_cal(cs35l56);
        ret
    }
}

unsafe extern "C" fn cs35l56_debugfs_cal_data_read(file: *mut file, to: *mut c_char,
                                                    count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        let cs35l56 = cs35l56_private_from_base(cs35l56_base);
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm) as ssize_t; }
        let ret = cs35l56_cal_data_debugfs_read(&mut (*cs35l56).base, to, count, ppos);
        cs35l56_power_down_after_cal(cs35l56);
        ret
    }
}

unsafe extern "C" fn cs35l56_new_cal_data_apply(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        if !(*cs35l56).base.cal_data_valid { return -ENXIO; }
        if (*cs35l56).base.secured { return -EACCES; }
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm); }
        snd_soc_dapm_mutex_lock(dapm);
        let mut ret = cs_amp_write_cal_coeffs(&mut (*cs35l56).dsp.cs_dsp,
                                              (*cs35l56).base.calibration_controls,
                                              &(*cs35l56).base.cal_data);
        if ret == 0 {
            cs35l56_mbox_send(&mut (*cs35l56).base, CS35L56_MBOX_CMD_AUDIO_REINIT);
        } else {
            ret = -EIO;
        }
        snd_soc_dapm_mutex_unlock(dapm);
        cs35l56_power_down_after_cal(cs35l56);
        ret
    }
}

unsafe extern "C" fn cs35l56_debugfs_cal_data_write(file: *mut file, from: *const c_char,
                                                     count: size_t, ppos: *mut loff_t) -> ssize_t {
    unsafe {
        let cs35l56_base = (*file).private_data as *mut cs35l56_base;
        let cs35l56 = cs35l56_private_from_base(cs35l56_base);
        let mut ret = cs35l56_cal_data_debugfs_write(&mut (*cs35l56).base, from, count, ppos) as c_int;
        if ret == -ENODATA { return count as ssize_t; } /* Ignore writes of empty cal blobs */
        else if ret < 0 { return -EIO as ssize_t; }
        ret = cs35l56_new_cal_data_apply(cs35l56);
        if ret != 0 { return ret as ssize_t; }
        count as ssize_t
    }
}

#[repr(C)]
pub struct file_operations { pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>, pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t> }
#[repr(C)]
pub struct cs35l56_cal_debugfs_fops { pub calibrate: file_operations, pub cal_temperature: file_operations, pub cal_data: file_operations }

static cs35l56_cal_debugfs_fops: cs35l56_cal_debugfs_fops = cs35l56_cal_debugfs_fops {
    calibrate: file_operations { read: None, write: Some(cs35l56_debugfs_calibrate_write) },
    cal_temperature: file_operations { read: None, write: Some(cs35l56_debugfs_cal_temperature_write) },
    cal_data: file_operations { read: Some(cs35l56_debugfs_cal_data_read), write: Some(cs35l56_debugfs_cal_data_write) },
};

unsafe extern "C" fn cs35l56_cal_data_rb_ctl_get(kcontrol: *mut snd_kcontrol,
                                                  ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        if !(*cs35l56).base.cal_data_valid { return -ENODATA; }
        core::ptr::copy_nonoverlapping(&(*cs35l56).base.cal_data as *const _ as *const u8,
                                       (*ucontrol).value.bytes.data.as_mut_ptr(),
                                       size_of::<cirrus_amp_cal_data>());
        0
    }
}

unsafe extern "C" fn cs35l56_cal_data_ctl_get(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        /*
         * This control is write-only but mixer libraries often try to read
         * a control before writing it. So we have to implement read.
         * Return zeros so a write of valid data will always be a change
         * from its "current value".
         */
        core::ptr::write_bytes((*ucontrol).value.bytes.data.as_mut_ptr(), 0,
                               size_of::<cirrus_amp_cal_data>());
        let _ = cs35l56;
        0
    }
}

unsafe extern "C" fn cs35l56_cal_data_ctl_set(kcontrol: *mut snd_kcontrol,
                                               ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let cal_data = (*ucontrol).value.bytes.data.as_ptr() as *const cirrus_amp_cal_data;
        if (*cs35l56).base.cal_data_valid { return -EACCES; }
        let mut ret = cs35l56_stash_calibration(&mut (*cs35l56).base, cal_data);
        if ret != 0 { return ret; }
        ret = cs35l56_new_cal_data_apply(cs35l56);
        if ret < 0 { return ret; }
        1
    }
}

unsafe extern "C" fn cs35l56_cal_ambient_ctl_get(kcontrol: *mut snd_kcontrol,
                                                  ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        (*ucontrol).value.integer.value[0] = (*cs35l56).ambient_ctl_value as i64;
        0
    }
}

unsafe extern "C" fn cs35l56_cal_ambient_ctl_set(kcontrol: *mut snd_kcontrol,
                                                  ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let temperature = (*ucontrol).value.integer.value[0] as c_int;
        if temperature == (*cs35l56).ambient_ctl_value { return 0; }
        if temperature < 0 || temperature > 40 { return -EINVAL; }
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm); }
        let ret = cs_amp_write_ambient_temp(&mut (*cs35l56).dsp.cs_dsp,
                                            (*cs35l56).base.calibration_controls,
                                            temperature);
        cs35l56_power_down_after_cal(cs35l56);
        if ret != 0 { return ret; }
        (*cs35l56).ambient_ctl_value = temperature;
        1
    }
}

unsafe extern "C" fn cs35l56_calibrate_ctl_get(_kcontrol: *mut snd_kcontrol,
                                                ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        /*
         * Allow reading because of user-side libraries that assume all
         * controls are readable. But always return false to prevent dumb
         * save-restore tools like alsactl accidentically triggering a
         * factory calibration when they restore.
         */
        (*ucontrol).value.integer.value[0] = 0;
        0
    }
}

unsafe extern "C" fn cs35l56_calibrate_ctl_set(kcontrol: *mut snd_kcontrol,
                                                ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        if (*ucontrol).value.integer.value[0] == 0 { return 0; }
        let dapm = cs35l56_power_up_for_cal(cs35l56);
        if IS_ERR(dapm) { return PTR_ERR(dapm); }
        snd_soc_dapm_mutex_lock(dapm);
        let ret = cs35l56_factory_calibrate(&mut (*cs35l56).base);
        snd_soc_dapm_mutex_unlock(dapm);
        cs35l56_power_down_after_cal(cs35l56);
        if ret < 0 { return ret; }
        1
    }
}

/* cs35l56_cal_data_restore_controls[] and cs35l56_cal_perform_controls[] are
 * generated by SND_SOC_BYTES_E*, SOC_SINGLE_EXT and SOC_SINGLE_BOOL_EXT_ACC.
 */
static cs35l56_cal_data_restore_controls: [snd_kcontrol_new; 0] = [];
static cs35l56_cal_perform_controls: [snd_kcontrol_new; 0] = [];

#[no_mangle]
pub unsafe extern "C" fn cs35l56_set_fw_suffix(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        let mut vendor: u16 = 0;
        let mut device: u16 = 0;
        let vendor_id: *const c_char;

        if !(*cs35l56).dsp.fwf_suffix.is_null() { return 0; }

        if !(*cs35l56).sdw_peripheral.is_null() {
            (*cs35l56).dsp.fwf_suffix = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL,
                                                        c"l%uu%u".as_ptr(),
                                                        (*cs35l56).sdw_link_num,
                                                        (*cs35l56).sdw_unique_id);
            if (*cs35l56).dsp.fwf_suffix.is_null() { return -ENOMEM; }
            /*
             * There are published firmware files for L56 B0 silicon using
             * the ALSA prefix as the filename suffix. Default to trying these
             * first, with the new SoundWire suffix as a fallback.
             * None of these older systems use a vendor-specific ID.
             */
            if (*cs35l56).base.type_ == 0x56 && (*cs35l56).base.rev == 0xb0 {
                (*cs35l56).fallback_fw_suffix = (*cs35l56).dsp.fwf_suffix;
                (*cs35l56).dsp.fwf_suffix = (*(*cs35l56).component).name_prefix;
                return 0;
            }
        }

        /*
         * Some manufacturers use the same SSID on multiple products and have
         * a vendor-specific qualifier to distinguish different models.
         * Models with the same SSID but different qualifier might require
         * different audio firmware, or they might all have the same audio
         * firmware.
         * Try searching for a firmware with this qualifier first, else
         * fallback to standard naming.
         */
        if snd_soc_card_get_pci_ssid((*(*cs35l56).component).card, &mut vendor, &mut device) < 0 {
            vendor_id = cs_amp_devm_get_vendor_specific_variant_id((*cs35l56).base.dev, -1, -1);
        } else {
            vendor_id = cs_amp_devm_get_vendor_specific_variant_id((*cs35l56).base.dev,
                                                                   vendor as c_int, device as c_int);
        }
        let ret = PTR_ERR_OR_ZERO(vendor_id);
        if ret == -ENOENT { return 0; }
        else if ret != 0 { return ret; }

        if !vendor_id.is_null() {
            if !(*cs35l56).dsp.fwf_suffix.is_null() {
                (*cs35l56).fallback_fw_suffix = (*cs35l56).dsp.fwf_suffix;
            } else {
                (*cs35l56).fallback_fw_suffix = (*(*cs35l56).component).name_prefix;
            }
            (*cs35l56).dsp.fwf_suffix = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL,
                                                        c"%s-%s".as_ptr(),
                                                        vendor_id,
                                                        (*cs35l56).fallback_fw_suffix);
            if (*cs35l56).dsp.fwf_suffix.is_null() { return -ENOMEM; }
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_set_fw_name(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let mut vendor: u16 = 0;
        let mut device: u16 = 0;
        let mut ret: c_int;

        if (*cs35l56).speaker_id < 0 && (*cs35l56).base.num_onchip_spkid_gpios != 0 {
            ret = cs35l56_configure_onchip_spkid_pads(&mut (*cs35l56).base);
            if ret != 0 { return ret; }
            ret = cs35l56_read_onchip_spkid(&mut (*cs35l56).base);
            if ret < 0 { return ret; }
            (*cs35l56).speaker_id = ret;
        }

        if (*cs35l56).dsp.system_name.is_null()
            && snd_soc_card_get_pci_ssid((*component).card, &mut vendor, &mut device) == 0 {
            /* Append a speaker qualifier if there is a speaker ID */
            if (*cs35l56).speaker_id >= 0 {
                (*cs35l56).dsp.system_name = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL,
                                                            c"%04x%04x-spkid%d".as_ptr(),
                                                            vendor as c_uint, device as c_uint,
                                                            (*cs35l56).speaker_id);
            } else {
                (*cs35l56).dsp.system_name = devm_kasprintf((*cs35l56).base.dev, GFP_KERNEL,
                                                            c"%04x%04x".as_ptr(),
                                                            vendor as c_uint, device as c_uint);
            }
            if (*cs35l56).dsp.system_name.is_null() { return -ENOMEM; }
        }
        0
    }
}

unsafe extern "C" { fn cs35l56_configure_onchip_spkid_pads(base: *mut cs35l56_base) -> c_int; fn cs35l56_read_onchip_spkid(base: *mut cs35l56_base) -> c_int; }

unsafe extern "C" fn _cs35l56_component_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let dapm = snd_soc_component_to_dapm(component);
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let debugfs_root = (*component).debugfs_root;
        let mut ret: c_int;

        /* BUILD_BUG_ON(ARRAY_SIZE(cs35l56_tx_input_texts) != ARRAY_SIZE(cs35l56_tx_input_values)); */

        if wait_for_completion_timeout(&mut (*cs35l56).init_completion, msecs_to_jiffies(5000)) == 0 {
            dev_err((*cs35l56).base.dev, c"%s: init_completion timed out\n".as_ptr(),
                    c"_cs35l56_component_probe".as_ptr());
            return -ENODEV;
        }

        (*cs35l56).dsp.part = kasprintf(GFP_KERNEL, c"cs35l%02x".as_ptr(), (*cs35l56).base.type_);
        if (*cs35l56).dsp.part.is_null() { return -ENOMEM; }

        (*cs35l56).component = component;
        ret = cs35l56_set_fw_name(component);
        if ret != 0 { return ret; }

        ret = cs35l56_set_fw_suffix(cs35l56);
        if ret != 0 { return ret; }

        wm_adsp2_component_probe(&mut (*cs35l56).dsp, component);

        debugfs_create_bool(c"init_done".as_ptr(), 0o444, debugfs_root, &mut (*cs35l56).base.init_done);
        debugfs_create_bool(c"can_hibernate".as_ptr(), 0o444, debugfs_root, &mut (*cs35l56).base.can_hibernate);
        debugfs_create_bool(c"fw_patched".as_ptr(), 0o444, debugfs_root, &mut (*cs35l56).base.fw_patched);

        match (*cs35l56).base.type_ {
            0x54 | 0x56 | 0x57 => {
                ret = snd_soc_add_component_controls(component, cs35l56_controls.as_ptr(),
                                                     ARRAY_SIZE(&cs35l56_controls));
            }
            0x63 | 0x62 => {
                ret = snd_soc_add_component_controls(component, cs35l63_controls.as_ptr(),
                                                     ARRAY_SIZE(&cs35l63_controls));
            }
            _ => ret = -ENODEV,
        }

        /* if IS_ENABLED(CONFIG_SND_SOC_CS35L56_CAL_SET_CTRL) */
        if ret == 0 {
            ret = snd_soc_add_component_controls(component,
                                                 cs35l56_cal_data_restore_controls.as_ptr(),
                                                 ARRAY_SIZE(&cs35l56_cal_data_restore_controls));
        }

        /* if IS_ENABLED(CONFIG_SND_SOC_CS35L56_CAL_PERFORM_CTRL) */
        if ret == 0 {
            ret = snd_soc_add_component_controls(component,
                                                 cs35l56_cal_perform_controls.as_ptr(),
                                                 ARRAY_SIZE(&cs35l56_cal_perform_controls));
        }

        if ret != 0 {
            return dev_err_probe((*cs35l56).base.dev, ret, c"unable to add controls\n".as_ptr());
        }

        ret = snd_soc_dapm_disable_pin(dapm, c"Calibrate".as_ptr());
        if ret != 0 { return ret; }

        /* if IS_ENABLED(CONFIG_SND_SOC_CS35L56_CAL_DEBUGFS) */
        cs35l56_create_cal_debugfs(&mut (*cs35l56).base, &cs35l56_cal_debugfs_fops);

        queue_work((*cs35l56).dsp_wq, &mut (*cs35l56).dsp_work);
        0
    }
}

unsafe extern "C" fn cs35l56_component_remove(component: *mut snd_soc_component) {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata(component);
        cancel_work_sync(&mut (*cs35l56).dsp_work);
        cs35l56_remove_cal_debugfs(&mut (*cs35l56).base);
        if (*cs35l56).dsp.cs_dsp.booted { wm_adsp_power_down(&mut (*cs35l56).dsp); }
        wm_adsp2_component_remove(&mut (*cs35l56).dsp, component);
        kfree((*cs35l56).dsp.part as *mut c_void);
        (*cs35l56).dsp.part = null_mut();
        kfree((*cs35l56).dsp.fwf_name as *mut c_void);
        (*cs35l56).dsp.fwf_name = null_mut();
        (*cs35l56).component = null_mut();
    }
}

unsafe extern "C" fn cs35l56_component_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let ret = _cs35l56_component_probe(component);
        if ret < 0 { cs35l56_component_remove(component); }
        ret
    }
}

unsafe extern "C" fn cs35l56_set_bias_level(component: *mut snd_soc_component,
                                             level: snd_soc_bias_level) -> c_int {
    unsafe {
        let cs35l56 = snd_soc_component_get_drvdata(component);
        let dapm = snd_soc_component_to_dapm(component);

        if level == SND_SOC_BIAS_STANDBY {
            /*
             * Wait for patching to complete when transitioning from
             * BIAS_OFF to BIAS_STANDBY
             */
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                cs35l56_wait_dsp_ready(cs35l56);
            }
        }
        0
    }
}

static soc_component_dev_cs35l56: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs35l56_component_probe),
    remove: Some(cs35l56_component_remove),
    dapm_widgets: cs35l56_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: cs35l56_audio_map.as_ptr(),
    num_dapm_routes: 106,
    set_bias_level: Some(cs35l56_set_bias_level),
    suspend_bias_off: 1, /* see cs35l56_system_resume() */
};

unsafe extern "C" fn cs35l56_runtime_suspend_i2c_spi(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        cs35l56_runtime_suspend_common(&mut (*cs35l56).base)
    }
}

unsafe extern "C" fn cs35l56_runtime_resume_i2c_spi(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        cs35l56_runtime_resume_common(&mut (*cs35l56).base, false)
    }
}

unsafe extern "C" { fn cs35l56_runtime_suspend_common(base: *mut cs35l56_base) -> c_int; fn cs35l56_runtime_resume_common(base: *mut cs35l56_base, is_sdw: bool) -> c_int; }

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_suspend(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_suspend\n".as_ptr());

        if !(*cs35l56).component.is_null() { flush_work(&mut (*cs35l56).dsp_work); }

        /*
         * The interrupt line is normally shared, but after we start suspending
         * we can't check if our device is the source of an interrupt, and can't
         * clear it. Prevent this race by temporarily disabling the parent irq
         * until we reach _no_irq.
         */
        if (*cs35l56).base.irq != 0 { disable_irq((*cs35l56).base.irq); }

        let ret = pm_runtime_force_suspend(dev);
        if ret < 0 && (*cs35l56).base.irq != 0 { enable_irq((*cs35l56).base.irq); }
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_suspend_late(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_suspend_late\n".as_ptr());

        /*
         * Assert RESET before removing supplies.
         * RESET is usually shared by all amps so it must not be asserted until
         * all driver instances have done their suspend() stage.
         */
        if !(*cs35l56).base.reset_gpio.is_null() {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
        }
        regulator_bulk_disable(ARRAY_SIZE(&(*cs35l56).supplies), (*cs35l56).supplies.as_mut_ptr());
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_suspend_no_irq(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_suspend_no_irq\n".as_ptr());
        /* Handlers are now disabled so the parent IRQ can safely be re-enabled. */
        if (*cs35l56).base.irq != 0 { enable_irq((*cs35l56).base.irq); }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_resume_no_irq(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_resume_no_irq\n".as_ptr());
        /*
         * WAKE interrupts unmask if the CS35L56 hibernates, which can cause
         * spurious interrupts, and the interrupt line is normally shared.
         * We can't check if our device is the source of an interrupt, and can't
         * clear it, until it has fully resumed. Prevent this race by temporarily
         * disabling the parent irq until we complete resume().
         */
        if (*cs35l56).base.irq != 0 { disable_irq((*cs35l56).base.irq); }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_resume_early(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_resume_early\n".as_ptr());

        /* Ensure a spec-compliant RESET pulse. */
        if !(*cs35l56).base.reset_gpio.is_null() {
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
        }

        /* Enable supplies before releasing RESET. */
        let ret = regulator_bulk_enable(ARRAY_SIZE(&(*cs35l56).supplies), (*cs35l56).supplies.as_mut_ptr());
        if ret != 0 {
            dev_err(dev, c"system_resume_early failed to enable supplies: %d\n".as_ptr(), ret);
            return ret;
        }

        /* Release shared RESET before drivers start resume(). */
        gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 1);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_system_resume(dev: *mut device) -> c_int {
    unsafe {
        let cs35l56 = dev_get_drvdata(dev);
        dev_dbg(dev, c"system_resume\n".as_ptr());

        /*
         * We might have done a hard reset or the CS35L56 was power-cycled
         * so wait for control port to be ready.
         */
        cs35l56_wait_control_port_ready();

        /* Undo pm_runtime_force_suspend() before re-enabling the irq */
        let mut ret = pm_runtime_force_resume(dev);
        if (*cs35l56).base.irq != 0 { enable_irq((*cs35l56).base.irq); }
        if ret != 0 { return ret; }

        /* Firmware won't have been loaded if the component hasn't probed */
        if (*cs35l56).component.is_null() { return 0; }

        ret = cs35l56_is_fw_reload_needed(&mut (*cs35l56).base);
        dev_dbg((*cs35l56).base.dev, c"fw_reload_needed: %d\n".as_ptr(), ret);
        if ret < 1 { return ret; }

        (*cs35l56).base.fw_patched = false;
        wm_adsp_power_down(&mut (*cs35l56).dsp);
        queue_work((*cs35l56).dsp_wq, &mut (*cs35l56).dsp_work);

        /*
         * suspend_bias_off ensures we are now in BIAS_OFF so there will be
         * a BIAS_OFF->BIAS_STANDBY transition to complete dsp patching.
         */
        0
    }
}

unsafe extern "C" fn cs35l56_control_add_nop(_dsp: *mut wm_adsp,
                                              _cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int {
    0
}

unsafe extern "C" fn cs35l56_dsp_init(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        (*cs35l56).dsp_wq = create_singlethread_workqueue(c"cs35l56-dsp".as_ptr());
        if (*cs35l56).dsp_wq.is_null() { return -ENOMEM; }

        INIT_WORK(&mut (*cs35l56).dsp_work, cs35l56_dsp_work);

        let dsp = &mut (*cs35l56).dsp as *mut wm_adsp;
        cs35l56_init_cs_dsp(&mut (*cs35l56).base, &mut (*dsp).cs_dsp);

        /*
         * dsp->part is filled in later as it is based on the DEVID. In a
         * SoundWire system that cannot be read until enumeration has occurred
         * and the device has attached.
         */
        (*dsp).fw = 12;
        (*dsp).wmfw_optional = true;

        /*
         * None of the firmware controls need to be exported so add a no-op
         * callback that suppresses creating an ALSA control.
         */
        (*dsp).control_add = Some(cs35l56_control_add_nop);

        dev_dbg((*cs35l56).base.dev, c"DSP system name: '%s'\n".as_ptr(), (*dsp).system_name);
        let ret = wm_halo_init(dsp);
        if ret != 0 {
            dev_err((*cs35l56).base.dev, c"wm_halo_init failed\n".as_ptr());
            return ret;
        }
        0
    }
}

unsafe extern "C" fn cs35l56_read_fwnode_u32_array(dev: *mut device,
                                                    parent_node: *mut fwnode_handle,
                                                    prop_name: *const c_char,
                                                    max_count: c_int,
                                                    dest: *mut u32) -> c_int {
    unsafe {
        let count = fwnode_property_count_u32(parent_node, prop_name);
        if count == 0 || count == -EINVAL || count == -ENODATA {
            dev_dbg(dev, c"%s not found in %s\n".as_ptr(), prop_name, fwnode_get_name(parent_node));
            return 0;
        }
        if count < 0 {
            dev_err(dev, c"Get %s error:%d\n".as_ptr(), prop_name, count);
            return count;
        }
        if count > max_count {
            dev_err(dev, c"%s too many entries (%d)\n".as_ptr(), prop_name, count);
            return -EOVERFLOW;
        }
        let ret = fwnode_property_read_u32_array(parent_node, prop_name, dest, count);
        if ret != 0 {
            dev_err(dev, c"Error reading %s: %d\n".as_ptr(), prop_name, ret);
            return ret;
        }
        count
    }
}

unsafe extern "C" fn cs35l56_process_xu_onchip_speaker_id(cs35l56: *mut cs35l56_private,
                                                           ext_node: *mut fwnode_handle) -> c_int {
    unsafe {
        let gpio_name = c"01fa-spk-id-gpios-onchip".as_ptr();
        let pull_name = c"01fa-spk-id-gpios-onchip-pull".as_ptr();
        let mut gpios = [0u32; 5];
        let mut pulls = [0u32; 5];

        /* static_assert(ARRAY_SIZE(gpios) == ARRAY_SIZE(cs35l56->base.onchip_spkid_gpios)); */
        /* static_assert(ARRAY_SIZE(pulls) == ARRAY_SIZE(cs35l56->base.onchip_spkid_pulls)); */

        let num_gpios = cs35l56_read_fwnode_u32_array((*cs35l56).base.dev, ext_node, gpio_name,
                                                      ARRAY_SIZE(&gpios) as c_int, gpios.as_mut_ptr());
        if num_gpios < 1 { return num_gpios; }

        let num_pulls = cs35l56_read_fwnode_u32_array((*cs35l56).base.dev, ext_node, pull_name,
                                                      ARRAY_SIZE(&pulls) as c_int, pulls.as_mut_ptr());
        if num_pulls < 0 { return num_pulls; }

        if num_pulls != 0 && num_pulls != num_gpios {
            dev_warn((*cs35l56).base.dev, c"%s count(%d) != %s count(%d)\n".as_ptr(),
                     pull_name, num_pulls, gpio_name, num_gpios);
        }

        let ret = cs35l56_check_and_save_onchip_spkid_gpios(&mut (*cs35l56).base,
                                                            gpios.as_mut_ptr(), num_gpios,
                                                            pulls.as_mut_ptr(), num_pulls);
        if ret != 0 {
            return dev_err_probe((*cs35l56).base.dev, ret, c"Error in %s/%s\n".as_ptr(),
                                 gpio_name, pull_name);
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_process_xu_properties(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        let mut ext_node: *mut fwnode_handle = null_mut();
        let mut link = fwnode_first_child_node(dev_fwnode((*cs35l56).base.dev));

        if (*cs35l56).sdw_peripheral.is_null() { return 0; }

        while !link.is_null() {
            ext_node = fwnode_get_named_child_node(link,
                                                   c"mipi-sdca-function-expansion-subproperties".as_ptr());
            if !ext_node.is_null() {
                fwnode_handle_put(link);
                break;
            }
            link = fwnode_next_child_node(dev_fwnode((*cs35l56).base.dev), link);
        }

        if ext_node.is_null() { return 0; }
        let ret = cs35l56_process_xu_onchip_speaker_id(cs35l56, ext_node);
        fwnode_handle_put(ext_node);
        ret
    }
}

unsafe extern "C" { fn fwnode_first_child_node(parent: *mut fwnode_handle) -> *mut fwnode_handle; fn fwnode_next_child_node(parent: *mut fwnode_handle, prev: *mut fwnode_handle) -> *mut fwnode_handle; }

#[no_mangle]
pub unsafe extern "C" fn cs35l56_get_firmware_uid(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        let dev = (*cs35l56).base.dev;
        let mut prop: *const c_char = null();
        let ret = device_property_read_string(dev, c"cirrus,firmware-uid".as_ptr(), &mut prop);
        /* If bad sw node property, return 0 and fallback to legacy firmware path */
        if ret < 0 { return 0; }

        /* Append a speaker qualifier if there is a speaker ID */
        if (*cs35l56).speaker_id >= 0 {
            (*cs35l56).dsp.system_name = devm_kasprintf(dev, GFP_KERNEL, c"%s-spkid%d".as_ptr(),
                                                        prop, (*cs35l56).speaker_id);
        } else {
            (*cs35l56).dsp.system_name = devm_kstrdup(dev, prop, GFP_KERNEL);
        }
        if (*cs35l56).dsp.system_name.is_null() { return -ENOMEM; }
        dev_dbg(dev, c"Firmware UID: %s\n".as_ptr(), (*cs35l56).dsp.system_name);
        0
    }
}

/*
 * Some SoundWire laptops have a spk-id-gpios property but it points to
 * the wrong ACPI Device node so can't be used to get the GPIO. Try to
 * find the SDCA node containing the GpioIo resource and add a GPIO
 * mapping to it.
 */
static cs35l56_af01_first_gpio: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: false };
static cs35l56_af01_spkid_gpios_mapping: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping { name: c"spk-id-gpios".as_ptr(), data: &cs35l56_af01_first_gpio, size: 1 },
    acpi_gpio_mapping { name: null(), data: null(), size: 0 },
];

unsafe extern "C" fn cs35l56_acpi_dev_release_driver_gpios(adev: *mut c_void) {
    unsafe { acpi_dev_remove_driver_gpios(adev); }
}

unsafe extern "C" fn cs35l56_try_get_broken_sdca_spkid_gpio(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        /* Find the SDCA node containing the GpioIo */
        let af01_fwnode = device_get_named_child_node((*cs35l56).base.dev, c"AF01".as_ptr());
        if af01_fwnode.is_null() {
            dev_dbg((*cs35l56).base.dev, c"No AF01 node\n".as_ptr());
            return -ENOENT;
        }

        let mut obj: *const acpi_object = null();
        let mut ret = acpi_dev_get_property(ACPI_COMPANION((*cs35l56).base.dev),
                                            c"spk-id-gpios".as_ptr(), ACPI_TYPE_PACKAGE, &mut obj);
        if ret != 0 {
            dev_dbg((*cs35l56).base.dev, c"Could not get spk-id-gpios package: %d\n".as_ptr(), ret);
            fwnode_handle_put(af01_fwnode);
            return -ENOENT;
        }

        /* The broken properties we can handle are a 4-element package (one GPIO) */
        if (*obj).data.package.count != 4 {
            dev_warn((*cs35l56).base.dev, c"Unexpected spk-id element count %d\n".as_ptr(),
                     (*obj).data.package.count);
            fwnode_handle_put(af01_fwnode);
            return -ENOENT;
        }

        /* Add a GPIO mapping if it doesn't already have one */
        if !fwnode_property_present(af01_fwnode, c"spk-id-gpios".as_ptr()) {
            let adev = to_acpi_device_node(af01_fwnode);
            /*
             * Can't use devm_acpi_dev_add_driver_gpios() because the
             * mapping isn't being added to the node pointed to by
             * ACPI_COMPANION().
             */
            ret = acpi_dev_add_driver_gpios(adev, cs35l56_af01_spkid_gpios_mapping.as_ptr());
            if ret != 0 {
                fwnode_handle_put(af01_fwnode);
                return dev_err_probe((*cs35l56).base.dev, ret,
                                     c"Failed to add gpio mapping to AF01\n".as_ptr());
            }

            ret = devm_add_action_or_reset((*cs35l56).base.dev,
                                           cs35l56_acpi_dev_release_driver_gpios,
                                           adev as *mut c_void);
            if ret != 0 {
                fwnode_handle_put(af01_fwnode);
                return ret;
            }

            dev_dbg((*cs35l56).base.dev, c"Added spk-id-gpios mapping to AF01\n".as_ptr());
        }

        let desc = fwnode_gpiod_get_index(af01_fwnode, c"spk-id".as_ptr(), 0, GPIOD_IN, null());
        if IS_ERR(desc) {
            fwnode_handle_put(af01_fwnode);
            ret = PTR_ERR(desc);
            return dev_err_probe((*cs35l56).base.dev, ret, c"Get GPIO from AF01 failed\n".as_ptr());
        }

        ret = gpiod_get_value_cansleep(desc);
        gpiod_put(desc);

        if ret < 0 {
            fwnode_handle_put(af01_fwnode);
            dev_err_probe((*cs35l56).base.dev, ret, c"Error reading spk-id GPIO\n".as_ptr());
            return ret;
        }

        fwnode_handle_put(af01_fwnode);
        dev_info((*cs35l56).base.dev, c"Got spk-id from AF01\n".as_ptr());
        ret
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_common_probe(cs35l56: *mut cs35l56_private, irq: c_int) -> c_int {
    unsafe {
        init_completion(&mut (*cs35l56).init_completion);
        mutex_init(&mut (*cs35l56).base.irq_lock);
        (*cs35l56).base.cal_index = -1;
        (*cs35l56).speaker_id = -ENOENT;

        dev_set_drvdata((*cs35l56).base.dev, cs35l56);

        cs35l56_fill_supply_names((*cs35l56).supplies.as_mut_ptr());
        let mut ret = devm_regulator_bulk_get((*cs35l56).base.dev, ARRAY_SIZE(&(*cs35l56).supplies),
                                              (*cs35l56).supplies.as_mut_ptr());
        if ret != 0 {
            return dev_err_probe((*cs35l56).base.dev, ret, c"Failed to request supplies\n".as_ptr());
        }

        /* Reset could be controlled by the BIOS or shared by multiple amps */
        (*cs35l56).base.reset_gpio = devm_gpiod_get_optional((*cs35l56).base.dev, c"reset".as_ptr(),
                                                             GPIOD_OUT_LOW);
        if IS_ERR((*cs35l56).base.reset_gpio) {
            ret = PTR_ERR((*cs35l56).base.reset_gpio);
            /*
             * If RESET is shared the first amp to probe will grab the reset
             * line and reset all the amps
             */
            if ret != -EBUSY {
                return dev_err_probe((*cs35l56).base.dev, ret, c"Failed to get reset GPIO\n".as_ptr());
            }
            dev_info((*cs35l56).base.dev, c"Reset GPIO busy, assume shared reset\n".as_ptr());
            (*cs35l56).base.reset_gpio = null_mut();
        }

        ret = regulator_bulk_enable(ARRAY_SIZE(&(*cs35l56).supplies), (*cs35l56).supplies.as_mut_ptr());
        if ret != 0 {
            return dev_err_probe((*cs35l56).base.dev, ret, c"Failed to enable supplies\n".as_ptr());
        }

        if !(*cs35l56).base.reset_gpio.is_null() {
            /* ACPI can override GPIOD_OUT_LOW flag so force it to start low */
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
            cs35l56_wait_min_reset_pulse();
            gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 1);
        }

        ret = cs35l56_get_speaker_id(&mut (*cs35l56).base);
        if !ACPI_COMPANION((*cs35l56).base.dev).is_null() && !(*cs35l56).sdw_peripheral.is_null() && ret == -ENOENT {
            ret = cs35l56_try_get_broken_sdca_spkid_gpio(cs35l56);
        }
        if ret < 0 && ret != -ENOENT { goto_err(cs35l56); return ret; }
        (*cs35l56).speaker_id = ret;

        ret = cs35l56_get_firmware_uid(cs35l56);
        if ret != 0 { goto_err(cs35l56); return ret; }

        ret = cs35l56_process_xu_properties(cs35l56);
        if ret != 0 { goto_err(cs35l56); return ret; }

        ret = cs35l56_dsp_init(cs35l56);
        if ret < 0 {
            dev_err_probe((*cs35l56).base.dev, ret, c"DSP init failed\n".as_ptr());
            goto_err(cs35l56);
            return ret;
        }

        /*
         * On SoundWire the cs35l56_init() cannot be run until after the
         * device has been enumerated by the SoundWire core.
         */
        if (*cs35l56).sdw_peripheral.is_null() {
            ret = cs35l56_init(cs35l56);
            if ret != 0 {
                wm_adsp2_remove(&mut (*cs35l56).dsp);
                goto_err(cs35l56);
                return ret;
            }
        }

        ret = cs35l56_irq_request(&mut (*cs35l56).base, irq);
        if ret != 0 {
            wm_adsp2_remove(&mut (*cs35l56).dsp);
            goto_err(cs35l56);
            return ret;
        }

        ret = snd_soc_register_component((*cs35l56).base.dev, &soc_component_dev_cs35l56,
                                         cs35l56_dai.as_mut_ptr(), ARRAY_SIZE(&cs35l56_dai));
        if ret < 0 {
            dev_err_probe((*cs35l56).base.dev, ret, c"Register codec failed\n".as_ptr());
            if (*cs35l56).base.irq != 0 {
                devm_free_irq((*cs35l56).base.dev, (*cs35l56).base.irq, &mut (*cs35l56).base as *mut _ as *mut c_void);
            }
            wm_adsp2_remove(&mut (*cs35l56).dsp);
            goto_err(cs35l56);
            return ret;
        }
        0
    }
}

unsafe extern "C" { fn mutex_init(m: *mut mutex); }

unsafe fn goto_err(cs35l56: *mut cs35l56_private) {
    unsafe {
        if pm_runtime_enabled((*cs35l56).base.dev) {
            pm_runtime_dont_use_autosuspend((*cs35l56).base.dev);
            pm_runtime_disable((*cs35l56).base.dev);
        }
        gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
        regulator_bulk_disable(ARRAY_SIZE(&(*cs35l56).supplies), (*cs35l56).supplies.as_mut_ptr());
        if !(*cs35l56).dsp_wq.is_null() { destroy_workqueue((*cs35l56).dsp_wq); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cs35l56_init(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        let mut ret: c_int;
        /*
         * Check whether the actions associated with soft reset or one time
         * init need to be performed.
         */
        if (*cs35l56).soft_resetting { return cs35l56_init_post_soft_reset(cs35l56); }
        if (*cs35l56).base.init_done { return 0; }

        pm_runtime_set_autosuspend_delay((*cs35l56).base.dev, CS35L56_FW_REQ_ACTIVE_TIMEOUT_MS + 50);
        pm_runtime_use_autosuspend((*cs35l56).base.dev);
        pm_runtime_set_active((*cs35l56).base.dev);
        pm_runtime_enable((*cs35l56).base.dev);

        ret = cs35l56_hw_init(&mut (*cs35l56).base);
        if ret < 0 { return ret; }
        ret = cs35l56_set_patch(&mut (*cs35l56).base);
        if ret != 0 { return ret; }
        ret = cs35l56_get_calibration(&mut (*cs35l56).base);
        if ret != 0 { return ret; }

        if (*cs35l56).base.reset_gpio.is_null() {
            dev_dbg((*cs35l56).base.dev, c"No reset gpio: using soft reset\n".as_ptr());
            (*cs35l56).soft_resetting = true;
            cs35l56_system_reset(&mut (*cs35l56).base, !(*cs35l56).sdw_peripheral.is_null());
            if !(*cs35l56).sdw_peripheral.is_null() {
                /* Keep alive while we wait for re-enumeration */
                pm_runtime_get_noresume((*cs35l56).base.dev);
                return 0;
            }
        }

        cs35l56_init_post_soft_reset(cs35l56)
    }
}

unsafe fn cs35l56_init_post_soft_reset(cs35l56: *mut cs35l56_private) -> c_int {
    unsafe {
        let mut ret: c_int;
        if (*cs35l56).soft_resetting {
            (*cs35l56).soft_resetting = false;

            /* Done re-enumerating after one-time init so release the keep-alive */
            if !(*cs35l56).sdw_peripheral.is_null() && !(*cs35l56).base.init_done {
                pm_runtime_put_noidle((*cs35l56).base.dev);
            }

            regcache_mark_dirty((*cs35l56).base.regmap);
            ret = cs35l56_wait_for_firmware_boot(&mut (*cs35l56).base);
            if ret != 0 { return ret; }

            dev_dbg((*cs35l56).base.dev, c"Firmware rebooted after soft reset\n".as_ptr());
            regcache_cache_only((*cs35l56).base.regmap, false);
        }

        /* Disable auto-hibernate so that runtime_pm has control */
        ret = cs35l56_mbox_send(&mut (*cs35l56).base, CS35L56_MBOX_CMD_PREVENT_AUTO_HIBERNATE);
        if ret != 0 { return ret; }

        /* Registers could be dirty after soft reset or SoundWire enumeration */
        regcache_sync((*cs35l56).base.regmap);

        /* Set ASP1 DOUT to high-impedance when it is not transmitting audio data. */
        ret = regmap_set_bits((*cs35l56).base.regmap, CS35L56_ASP1_CONTROL3,
                              CS35L56_ASP1_DOUT_HIZ_CTRL_MASK);
        if ret != 0 {
            return dev_err_probe((*cs35l56).base.dev, ret, c"Failed to write ASP1_CONTROL3\n".as_ptr());
        }

        (*cs35l56).base.init_done = true;
        complete_all(&mut (*cs35l56).init_completion);
        0
    }
}

unsafe extern "C" { fn cs35l56_wait_for_firmware_boot(base: *mut cs35l56_base) -> c_int; }

#[no_mangle]
pub unsafe extern "C" fn cs35l56_remove(cs35l56: *mut cs35l56_private) {
    unsafe {
        snd_soc_unregister_component((*cs35l56).base.dev);
        (*cs35l56).base.init_done = false;

        /*
         * WAKE IRQs unmask if CS35L56 hibernates so free the handler to
         * prevent it racing with remove().
         */
        if (*cs35l56).base.irq != 0 {
            devm_free_irq((*cs35l56).base.dev, (*cs35l56).base.irq, &mut (*cs35l56).base as *mut _ as *mut c_void);
        }

        destroy_workqueue((*cs35l56).dsp_wq);
        wm_adsp2_remove(&mut (*cs35l56).dsp);
        pm_runtime_dont_use_autosuspend((*cs35l56).base.dev);
        pm_runtime_suspend((*cs35l56).base.dev);
        pm_runtime_disable((*cs35l56).base.dev);
        regcache_cache_only((*cs35l56).base.regmap, true);
        gpiod_set_value_cansleep((*cs35l56).base.reset_gpio, 0);
        regulator_bulk_disable(ARRAY_SIZE(&(*cs35l56).supplies), (*cs35l56).supplies.as_mut_ptr());
    }
}

/* #if IS_ENABLED(CONFIG_SND_SOC_CS35L56_I2C) || IS_ENABLED(CONFIG_SND_SOC_CS35L56_SPI)
 * EXPORT_NS_GPL_DEV_PM_OPS(cs35l56_pm_ops_i2c_spi, SND_SOC_CS35L56_CORE) = {
 *     SET_RUNTIME_PM_OPS(cs35l56_runtime_suspend_i2c_spi, cs35l56_runtime_resume_i2c_spi, NULL)
 *     SYSTEM_SLEEP_PM_OPS(cs35l56_system_suspend, cs35l56_system_resume)
 *     LATE_SYSTEM_SLEEP_PM_OPS(cs35l56_system_suspend_late, cs35l56_system_resume_early)
 *     NOIRQ_SYSTEM_SLEEP_PM_OPS(cs35l56_system_suspend_no_irq, cs35l56_system_resume_no_irq)
 * };
 * #endif
 */

/* MODULE_DESCRIPTION("ASoC CS35L56 driver");
 * MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
 * MODULE_IMPORT_NS("SND_SOC_CS_AMP_LIB");
 * MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
 * MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
