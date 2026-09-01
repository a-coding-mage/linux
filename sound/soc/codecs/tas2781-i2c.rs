// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAS2563/TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2026 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2563/TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2563/TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Author: Kevin Lu <kevin-lu@ti.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

// C include dependencies intentionally remain external to this isolated translation:
// linux/cleanup.h, linux/crc8.h, linux/firmware.h, linux/gpio/consumer.h,
// linux/i2c.h, linux/init.h, linux/interrupt.h, linux/module.h, linux/of.h,
// linux/of_address.h, linux/of_irq.h, linux/regmap.h, linux/slab.h,
// sound/pcm_params.h, sound/soc.h, sound/tas2781.h,
// sound/tas2781-comlib-i2c.h, sound/tlv.h, sound/tas2x20-tlv.h,
// sound/tas2563-tlv.h, sound/tas2781-tlv.h, sound/tas5825-tlv.h,
// linux/unaligned.h.

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type u64 = u64;
type uintptr_t = usize;
type kernel_ulong_t = c_ulong;

extern "C" {
    static mut tas2x20_dvc_table: *const *const u8;
    static mut tas2563_dvc_table: *const *const u8;
    static tas2x20_amp_tlv: c_void;
    static tas2x20_dvc_tlv: c_void;
    static tas2781_amp_tlv: c_void;
    static tas2781_dvc_tlv: c_void;
    static tas5825_amp_tlv: c_void;
    static tas5825_dvc_tlv: c_void;
    static tas2563_dvc_tlv: c_void;
    static THIS_MODULE: *mut c_void;

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut tasdevice_priv;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut tasdevice_priv;
    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num_controls: c_uint) -> c_int;
    fn snd_soc_info_bool_ext(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_soc_bytes_info_ext(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;

    fn tasdevice_digital_getvol(priv_: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_digital_putvol(priv_: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_amp_getvol(priv_: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_amp_putvol(priv_: *mut tasdevice_priv, ucontrol: *mut snd_ctl_elem_value, mc: *mut soc_mixer_control) -> c_int;
    fn tasdevice_dev_bulk_read(priv_: *mut tasdevice_priv, dev: c_int, reg: c_uint, data: *mut u8, len: c_uint) -> c_int;
    fn tasdevice_dev_bulk_write(priv_: *mut tasdevice_priv, dev: c_int, reg: c_uint, data: *mut u8, len: c_uint) -> c_int;
    fn tasdevice_dev_read(priv_: *mut tasdevice_priv, dev: c_int, reg: c_uint, val: *mut c_int) -> c_int;
    fn tasdevice_dev_write(priv_: *mut tasdevice_priv, dev: c_int, reg: c_uint, val: c_uint) -> c_int;
    fn tasdev_chn_switch(priv_: *mut tasdevice_priv, dev_id: c_int) -> c_int;
    fn tasdevice_rca_parser(priv_: *mut tasdevice_priv, fmw: *const firmware) -> c_int;
    fn tasdevice_config_info_remove(priv_: *mut tasdevice_priv);
    fn tasdevice_dsp_remove(priv_: *mut tasdevice_priv);
    fn tasdevice_calbin_remove(priv_: *mut tasdevice_priv);
    fn tasdevice_dsp_parser(priv_: *mut tasdevice_priv) -> c_int;
    fn tas2781_load_calibration(priv_: *mut tasdevice_priv, name: *mut c_char, i: c_int) -> c_int;
    fn tasdevice_prmg_load(priv_: *mut tasdevice_priv, id: c_int) -> c_int;
    fn tasdevice_select_cfg_blk(priv_: *mut tasdevice_priv, id: c_int, blk: c_int) -> c_int;
    fn tasdevice_tuning_switch(priv_: *mut tasdevice_priv, state: c_int, capture: bool);
    fn tascodec_init(priv_: *mut tasdevice_priv, codec: *mut snd_soc_component, module: *mut c_void, cb: unsafe extern "C" fn(*const firmware, *mut c_void)) -> c_int;
    fn tasdevice_kzalloc(i2c: *mut i2c_client) -> *mut tasdevice_priv;
    fn tasdevice_init(priv_: *mut tasdevice_priv) -> c_int;
    fn tasdevice_reset(priv_: *mut tasdevice_priv);
    fn tasdevice_remove(priv_: *mut tasdevice_priv);

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kstrdup(dev: *mut device, s: *const c_char, flags: c_uint) -> *mut c_char;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn memdup_user(src: *const c_char, len: usize) -> *mut u8;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, len: usize) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
    fn release_firmware(fmw: *const firmware);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn i2c_get_match_data(i2c: *mut i2c_client) -> *const c_void;
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut tasdevice_priv;
    fn acpi_device_get_match_data(dev: *mut device) -> *const c_void;
    fn acpi_dev_gpio_irq_get(companion: *mut c_void, index: c_int) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, prop: *const c_char, vals: *mut c_uint, nval: usize) -> c_int;
    fn of_property_read_reg(np: *mut device_node, index: c_int, addr: *mut u64, size: *mut u64) -> c_int;
    fn of_irq_get(np: *mut device_node, index: c_int) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn simple_open(inode: *mut c_void, file: *mut file) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: size_t, ppos: *mut loff_t, from: *const c_void, available: size_t) -> ssize_t;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
}

#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct firmware { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct gpio_desc { _private: [u8; 0] }
#[repr(C)] struct dentry { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol { private_value: c_ulong }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct i2c_client { dev: device, addr: c_uint }
#[repr(C)] struct snd_soc_component { name_prefix: *mut c_char, debugfs_root: *mut dentry }
#[repr(C)] struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] struct snd_soc_dapm_widget { dapm: *mut c_void }

#[repr(C)]
struct bulk_reg_val {
    reg: c_uint,
    val: [u8; 4],
    val_len: c_uint,
    is_locked: bool,
}

#[repr(C)] struct i2c_device_id { name: [c_char; 20], driver_data: kernel_ulong_t }
#[repr(C)] struct of_device_id { compatible: *const c_char, data: *const c_void }
#[repr(C)] struct acpi_device_id { id: [c_char; 16], driver_data: kernel_ulong_t }

#[repr(C)] struct soc_mixer_control { reg: c_uint, max: c_uint }
#[repr(C)] struct soc_bytes_ext { max: c_uint }
#[repr(C)] struct cali_reg { r0_reg: c_uint, r0_low_reg: c_uint, invr0_reg: c_uint, pow_reg: c_uint, tlimit_reg: c_uint }
#[repr(C)] struct calidata { cali_reg_array: cali_reg, data: *mut u8, total_sz: c_uint, cali_dat_sz_per_dev: c_uint }
#[repr(C)] struct fct_param_address { tf_reg: [u8; 3], r0_reg: [u8; 3], a1_reg: [u8; 3], a2_reg: [u8; 3], thr: [u8; 3], plt_flg: [u8; 3], sin_gn: [u8; 3], sin_gn2: [u8; 3], thr2: [u8; 3] }
#[repr(C)] struct tasdevice_fw { nr_programs: c_uint, nr_configurations: c_uint, fct_par_addr: fct_param_address }
#[repr(C)] struct rcabin { profile_cfg_id: c_uint, capture_profile_id: c_uint, ncfgs: c_uint, init_profile_id: c_int }
#[repr(C)] struct tasdevice { dev_addr: c_uint, cali_data_backup: *mut bulk_reg_val, alp_cali_bckp: bulk_reg_val, cur_prog: c_int }
#[repr(C)] struct acoustic_data { len: c_int, id: u8, data: [u8; 128] }
#[repr(C)] struct tasdevice_priv {
    dev: *mut device,
    client: *mut c_void,
    tasdevice: *mut tasdevice,
    ndev: c_int,
    chip_id: c_uint,
    codec_lock: mutex,
    force_fwload_status: bool,
    cali_data: calidata,
    dspbin_typ: c_uint,
    fmw: *mut tasdevice_fw,
    rcabin: rcabin,
    codec: *mut snd_soc_component,
    dvc_tlv_table: *const *const u8,
    fw_state: c_int,
    name_prefix: *mut c_char,
    coef_binaryname: [c_char; 64],
    cal_binaryname: [[c_char; 64]; 8],
    dev_name: [c_char; 64],
    cur_prog: c_uint,
    cur_conf: c_uint,
    sysclk: c_uint,
    isacpi: bool,
    irq: c_int,
    reset: *mut gpio_desc,
    acou_data: acoustic_data,
}

#[repr(C)] struct snd_ctl_elem_integer { value: [c_long; 128] }
type c_long = isize;
#[repr(C)] struct snd_ctl_elem_bytes { data: *mut u8 }
#[repr(C)] union snd_ctl_elem_value_value { integer: core::mem::ManuallyDrop<snd_ctl_elem_integer>, bytes: core::mem::ManuallyDrop<snd_ctl_elem_bytes> }
#[repr(C)] struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }
#[repr(C)] struct snd_ctl_elem_info_integer { min: c_long, max: c_long }
#[repr(C)] union snd_ctl_elem_info_value { integer: core::mem::ManuallyDrop<snd_ctl_elem_info_integer> }
#[repr(C)] struct snd_ctl_elem_info { type_: c_uint, count: c_uint, value: snd_ctl_elem_info_value }

type kcontrol_info = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;
type kcontrol_get = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;
type kcontrol_put = unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int;

#[repr(C)]
struct snd_kcontrol_new {
    name: *mut c_char,
    iface: c_uint,
    info: Option<kcontrol_info>,
    get: Option<kcontrol_get>,
    put: Option<kcontrol_put>,
    private_value: c_ulong,
}

#[repr(C)] struct snd_soc_dai_ops { startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>, hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int> }
#[repr(C)] struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint }
#[repr(C)] struct snd_soc_dai_driver { name: *const c_char, id: c_int, playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream, ops: *const snd_soc_dai_ops, symmetric_rate: c_uint }
#[repr(C)] struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] struct snd_soc_component_driver { probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>, dapm_widgets: *const snd_soc_dapm_widget, num_dapm_widgets: c_uint, dapm_routes: *const snd_soc_dapm_route, num_dapm_routes: c_uint, idle_bias_on: c_uint, endianness: c_uint }
#[repr(C)] struct file { private_data: *mut c_void }
#[repr(C)] struct file_operations { open: Option<unsafe extern "C" fn(*mut c_void, *mut file) -> c_int>, read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>, write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t> }
#[repr(C)] struct i2c_driver_driver { name: *const c_char, of_match_table: *const of_device_id, acpi_match_table: *const acpi_device_id }
#[repr(C)] struct i2c_driver { driver: i2c_driver_driver, probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, remove: Option<unsafe extern "C" fn(*mut i2c_client)> }

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 1;
const SND_SOC_DAPM_POST_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const TASDEVICE_MAX_CHANNELS: usize = 8;
const TASDEVICE_RATES: c_uint = 0;
const TASDEVICE_FORMATS: c_uint = 0;
const TAS2020: c_uint = 0;
const TAS2118: c_uint = 1;
const TAS2120: c_uint = 2;
const TAS2320: c_uint = 3;
const TAS2563: c_uint = 4;
const TAS2568: c_uint = 5;
const TAS2570: c_uint = 6;
const TAS2572: c_uint = 7;
const TAS2573: c_uint = 8;
const TAS2574: c_uint = 9;
const TAS2781: c_uint = 10;
const TAS5802: c_uint = 11;
const TAS5806M: c_uint = 12;
const TAS5806MD: c_uint = 13;
const TAS5815: c_uint = 14;
const TAS5822: c_uint = 15;
const TAS5825: c_uint = 16;
const TAS5827: c_uint = 17;
const TAS5828: c_uint = 18;
const TAS5830: c_uint = 19;
const TAS5832: c_uint = 20;
const TAS_OTHERS: c_uint = 21;
const TASDEV_ALPHA: c_uint = 1;
const TASDEV_BASIC: c_uint = 0;
const TASDEVICE_RCA_FW_OK: c_int = 1;
const TASDEVICE_DSP_FW_ALL_OK: c_int = 2;
const TASDEVICE_DSP_FW_PENDING: c_int = 0;
const TASDEVICE_BIN_BLK_PRE_POWER_UP: c_int = 0;
const GPIOD_OUT_HIGH: c_int = 1;
const TAS2563_IDLE: c_uint = 0;
const TAS2563_PRM_ENFF_REG: c_uint = 0;
const TAS2563_PRM_DISTCK_REG: c_uint = 0;
const TAS2563_PRM_TE_SCTHR_REG: c_uint = 0;
const TAS2563_PRM_PLT_FLAG_REG: c_uint = 0;
const TAS2563_PRM_SINEGAIN_REG: c_uint = 0;
const TAS2563_TE_TA1_REG: c_uint = 0;
const TAS2563_TE_TA1_AT_REG: c_uint = 0;
const TAS2563_TE_TA2_REG: c_uint = 0;
const TAS2563_TE_AT_REG: c_uint = 0;
const TAS2563_TE_DT_REG: c_uint = 0;
const TAS2563_RUNTIME_RE_REG_TF: c_uint = 0;
const TAS2563_RUNTIME_RE_REG: c_uint = 0;
const TAS2563_PRM_R0_REG: c_uint = 0;
const TAS2563_DVC_LVL: c_uint = 0;
const TAS2781_PRM_INT_MASK_REG: c_uint = 0;
const TAS2781_PRM_CLK_CFG_REG: c_uint = 0;
const TAS2781_PRM_RSVD_REG: c_uint = 0;
const TAS2781_PRM_TEST_57_REG: c_uint = 0;
const TAS2781_PRM_TEST_62_REG: c_uint = 0;
const TAS2781_PRM_PVDD_UVLO_REG: c_uint = 0;
const TAS2781_PRM_CHNL_0_REG: c_uint = 0;
const TAS2781_PRM_NG_CFG0_REG: c_uint = 0;
const TAS2781_PRM_IDLE_CH_DET_REG: c_uint = 0;
const TAS2781_PRM_PLT_FLAG_REG: c_uint = 0;
const TAS2781_PRM_SINEGAIN_REG: c_uint = 0;
const TAS2781_PRM_SINEGAIN2_REG: c_uint = 0;
const TAS2781_TEST_UNLOCK_REG: c_uint = 0;
const TAS2781_TEST_PAGE_UNLOCK: c_uint = 0;
const TAS2781_RUNTIME_LATCH_RE_REG: c_uint = 0;
const TAS2781_RUNTIME_RE_REG_TF: c_uint = 0;
const TAS2781_RUNTIME_RE_REG: c_uint = 0;
const TAS2781_AMP_LEVEL: c_uint = 0;
const TAS2781_DVC_LVL: c_uint = 0;
const TAS2X20_AMP_LEVEL: c_uint = 0;
const TAS2X20_DVC_LEVEL: c_uint = 0;
const TAS5825_AMP_LEVEL: c_uint = 0;
const TAS5825_DVC_LEVEL: c_uint = 0;
const TASDEVICE_XM_A1_REG: c_uint = 0;
const TASDEVICE_XM_A2_REG: c_uint = 0;

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! dev_err { ($dev:expr, $fmt:literal $(, $arg:expr)* $(,)?) => { dev_err($dev, cstr!($fmt) $(, $arg)*); }; }
macro_rules! dev_dbg { ($dev:expr, $fmt:literal $(, $arg:expr)* $(,)?) => { dev_dbg($dev, cstr!($fmt) $(, $arg)*); }; }

const fn TASDEVICE_REG(book: u8, page: u8, reg: u8) -> c_uint { ((book as c_uint) << 16) | ((page as c_uint) << 8) | reg as c_uint }
const fn TASDEVICE_BOOK_ID(reg: c_uint) -> u8 { ((reg >> 16) & 0xff) as u8 }
const fn TASDEVICE_PAGE_ID(reg: c_uint) -> u8 { ((reg >> 8) & 0xff) as u8 }
const fn TASDEVICE_PAGE_REG(reg: c_uint) -> u8 { (reg & 0xff) as u8 }
const fn BIT(i: c_int) -> c_int { 1 << i }
fn clamp(v: c_int, lo: c_int, hi: c_int) -> c_int { if v < lo { lo } else if v > hi { hi } else { v } }
unsafe fn get_unaligned_be32(p: *const u8) -> c_uint { ((*p.add(0) as c_uint) << 24) | ((*p.add(1) as c_uint) << 16) | ((*p.add(2) as c_uint) << 8) | (*p.add(3) as c_uint) }
fn min_u(a: c_uint, b: c_uint) -> c_uint { if a < b { a } else { b } }
fn IS_ERR<T>(p: *mut T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
fn PTR_ERR<T>(p: *mut T) -> c_int { p as isize as c_int }
fn IS_ENABLED_CONFIG_OF() -> bool { true }
unsafe fn ACPI_HANDLE(_dev: *mut device) -> bool { false }
unsafe fn ACPI_COMPANION(_dev: *mut device) -> *mut c_void { ptr::null_mut() }

struct MutexGuard(*mut mutex);
impl Drop for MutexGuard { fn drop(&mut self) { unsafe { mutex_unlock(self.0) } } }
unsafe fn guard_mutex(lock: *mut mutex) -> MutexGuard { mutex_lock(lock); MutexGuard(lock) }

const fn brv(reg: c_uint, val: [u8; 4], val_len: c_uint, is_locked: bool) -> bulk_reg_val {
    bulk_reg_val { reg, val, val_len, is_locked }
}

static tas2563_cali_start_reg: [bulk_reg_val; 11] = [
    brv(TAS2563_IDLE, [0x00, 0, 0, 0], 1, false),
    brv(TAS2563_PRM_ENFF_REG, [0x40, 0x00, 0x00, 0x00], 4, false),
    brv(TAS2563_PRM_DISTCK_REG, [0x40, 0x00, 0x00, 0x00], 4, false),
    brv(TAS2563_PRM_TE_SCTHR_REG, [0x7f, 0xff, 0xff, 0xff], 4, false),
    brv(TAS2563_PRM_PLT_FLAG_REG, [0x40, 0x00, 0x00, 0x00], 4, false),
    brv(TAS2563_PRM_SINEGAIN_REG, [0x0a, 0x3d, 0x70, 0xa4], 4, false),
    brv(TAS2563_TE_TA1_REG, [0x00, 0x36, 0x91, 0x5e], 4, false),
    brv(TAS2563_TE_TA1_AT_REG, [0x00, 0x36, 0x91, 0x5e], 4, false),
    brv(TAS2563_TE_TA2_REG, [0x00, 0x06, 0xd3, 0x72], 4, false),
    brv(TAS2563_TE_AT_REG, [0x00, 0x36, 0x91, 0x5e], 4, false),
    brv(TAS2563_TE_DT_REG, [0x00, 0x36, 0x91, 0x5e], 4, false),
];

static tas2781_cali_start_reg: [bulk_reg_val; 13] = [
    brv(TAS2781_PRM_INT_MASK_REG, [0xfe, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_CLK_CFG_REG, [0xdd, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_RSVD_REG, [0x20, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_TEST_57_REG, [0x14, 0, 0, 0], 1, true),
    brv(TAS2781_PRM_TEST_62_REG, [0x45, 0, 0, 0], 1, true),
    brv(TAS2781_PRM_PVDD_UVLO_REG, [0x03, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_CHNL_0_REG, [0xa8, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_NG_CFG0_REG, [0xb9, 0, 0, 0], 1, false),
    brv(TAS2781_PRM_IDLE_CH_DET_REG, [0x92, 0, 0, 0], 1, false),
    /*
     * This register is pilot tone threshold, different with the
     * calibration tool version, it will be updated in
     * tas2781_calib_start_put(), set to 1mA.
     */
    brv(0, [0x00, 0x00, 0x00, 0x56], 4, false),
    brv(TAS2781_PRM_PLT_FLAG_REG, [0x40, 0x00, 0x00, 0x00], 4, false),
    brv(TAS2781_PRM_SINEGAIN_REG, [0, 0, 0, 0], 4, false),
    brv(TAS2781_PRM_SINEGAIN2_REG, [0, 0, 0, 0], 4, false),
];

const fn id(name: &[u8], driver_data: c_uint) -> i2c_device_id {
    let mut out = [0 as c_char; 20];
    let mut i = 0;
    while i < name.len() && i < 19 {
        out[i] = name[i] as c_char;
        i += 1;
    }
    i2c_device_id { name: out, driver_data: driver_data as kernel_ulong_t }
}

static tasdevice_id: [i2c_device_id; 22] = [
    id(b"tas2020", TAS2020), id(b"tas2118", TAS2118), id(b"tas2120", TAS2120),
    id(b"tas2320", TAS2320), id(b"tas2563", TAS2563), id(b"tas2568", TAS2568),
    id(b"tas2570", TAS2570), id(b"tas2572", TAS2572), id(b"tas2573", TAS2573),
    id(b"tas2574", TAS2574), id(b"tas2781", TAS2781), id(b"tas5802", TAS5802),
    id(b"tas5806m", TAS5806M), id(b"tas5806md", TAS5806MD), id(b"tas5815", TAS5815),
    id(b"tas5822", TAS5822), id(b"tas5825", TAS5825), id(b"tas5827", TAS5827),
    id(b"tas5828", TAS5828), id(b"tas5830", TAS5830), id(b"tas5832", TAS5832),
    id(b"", 0),
];

static tasdevice_of_match: [of_device_id; 22] = [
    of_device_id { compatible: cstr!("ti,tas2020"), data: unsafe { tasdevice_id.as_ptr().add(TAS2020 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2118"), data: unsafe { tasdevice_id.as_ptr().add(TAS2118 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2120"), data: unsafe { tasdevice_id.as_ptr().add(TAS2120 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2320"), data: unsafe { tasdevice_id.as_ptr().add(TAS2320 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2563"), data: unsafe { tasdevice_id.as_ptr().add(TAS2563 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2568"), data: unsafe { tasdevice_id.as_ptr().add(TAS2568 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2570"), data: unsafe { tasdevice_id.as_ptr().add(TAS2570 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2572"), data: unsafe { tasdevice_id.as_ptr().add(TAS2572 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2573"), data: unsafe { tasdevice_id.as_ptr().add(TAS2573 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2574"), data: unsafe { tasdevice_id.as_ptr().add(TAS2574 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas2781"), data: unsafe { tasdevice_id.as_ptr().add(TAS2781 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5802"), data: unsafe { tasdevice_id.as_ptr().add(TAS5802 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5806m"), data: unsafe { tasdevice_id.as_ptr().add(TAS5806M as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5806md"), data: unsafe { tasdevice_id.as_ptr().add(TAS5806MD as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5815"), data: unsafe { tasdevice_id.as_ptr().add(TAS5815 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5822"), data: unsafe { tasdevice_id.as_ptr().add(TAS5822 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5825"), data: unsafe { tasdevice_id.as_ptr().add(TAS5825 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5827"), data: unsafe { tasdevice_id.as_ptr().add(TAS5827 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5828"), data: unsafe { tasdevice_id.as_ptr().add(TAS5828 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5830"), data: unsafe { tasdevice_id.as_ptr().add(TAS5830 as usize) as *const c_void } },
    of_device_id { compatible: cstr!("ti,tas5832"), data: unsafe { tasdevice_id.as_ptr().add(TAS5832 as usize) as *const c_void } },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];

// MODULE_DEVICE_TABLE(of, tasdevice_of_match);

unsafe extern "C" fn tas2781_digital_getvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    tasdevice_digital_getvol(tas_priv, ucontrol, mc)
}

unsafe extern "C" fn tas2781_digital_putvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    tasdevice_digital_putvol(tas_priv, ucontrol, mc)
}

unsafe extern "C" fn tas2781_amp_getvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    tasdevice_amp_getvol(tas_priv, ucontrol, mc)
}

unsafe extern "C" fn tas2781_amp_putvol(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    tasdevice_amp_putvol(tas_priv, ucontrol, mc)
}

unsafe extern "C" fn tasdev_force_fwload_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(component);
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value.as_mut_ptr().write((*tas_priv).force_fwload_status as c_long);
    dev_dbg!((*tas_priv).dev, "%s : Force FWload %s\n", cstr!("tasdev_force_fwload_get"), if (*tas_priv).force_fwload_status { cstr!("ON") } else { cstr!("OFF") });
    0
}

unsafe extern "C" fn tasdev_force_fwload_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(component);
    let val = (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value[0] != 0;
    let change;
    if (*tas_priv).force_fwload_status == val {
        change = false;
    } else {
        change = true;
        (*tas_priv).force_fwload_status = val;
    }
    dev_dbg!((*tas_priv).dev, "%s : Force FWload %s\n", cstr!("tasdev_force_fwload_put"), if (*tas_priv).force_fwload_status { cstr!("ON") } else { cstr!("OFF") });
    change as c_int
}

unsafe fn bytes_data(ucontrol: *mut snd_ctl_elem_value) -> *mut u8 {
    core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.bytes)).data
}

unsafe fn integer_value0(ucontrol: *mut snd_ctl_elem_value) -> c_long {
    core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer)).value[0]
}

unsafe fn set_integer_value0(ucontrol: *mut snd_ctl_elem_value, v: c_long) {
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*ucontrol).value.integer))).value.as_mut_ptr().write(v);
}

unsafe extern "C" fn tasdev_cali_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp);
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let cali_data = &mut (*priv_).cali_data as *mut calidata;
    let p = &mut (*cali_data).cali_reg_array as *mut cali_reg;
    let dst = bytes_data(ucontrol);
    let data = (*cali_data).data;
    let mut i: c_uint = 0;
    let mut rc: c_int;
    let _guard = guard_mutex(&mut (*priv_).codec_lock);
    if (*p).r0_reg == 0 { return -1; }
    *dst.add(i as usize) = (*bytes_ext).max as u8; i += 1;
    *dst.add(i as usize) = b'r'; i += 1;
    for reg in [(*p).r0_reg, (*p).r0_low_reg, (*p).invr0_reg, (*p).pow_reg, (*p).tlimit_reg] {
        *dst.add(i as usize) = TASDEVICE_BOOK_ID(reg); i += 1;
        *dst.add(i as usize) = TASDEVICE_PAGE_ID(reg); i += 1;
        *dst.add(i as usize) = TASDEVICE_PAGE_REG(reg); i += 1;
    }
    let mut j: c_uint = 0;
    let mut k: c_uint = 0;
    while j < (*priv_).ndev as c_uint {
        if j == *data.add(k as usize) as c_uint {
            *dst.add(i as usize) = j as u8; i += 1; k += 1;
        } else {
            dev_err!((*priv_).dev, "chn %d device %u not match\n", j as c_int, *data.add(k as usize) as c_uint);
            k += 21;
            j += 1;
            continue;
        }
        let regs = [(*p).r0_reg, (*p).r0_low_reg, (*p).invr0_reg, (*p).pow_reg, (*p).tlimit_reg];
        let names = [cstr!("r0_data"), cstr!("r0_low"), cstr!("invr0"), cstr!("pow_reg"), cstr!("tlimit")];
        let mut idx = 0;
        while idx < 5 {
            rc = tasdevice_dev_bulk_read(priv_, j as c_int, regs[idx], dst.add(i as usize), 4);
            if rc < 0 {
                dev_err!((*priv_).dev, "chn %d bulk_rd err = %d\n", j as c_int, rc);
                let rem = (4 - idx) as c_uint * 4;
                i += rem; k += rem;
                break;
            }
            rc = memcmp(dst.add(i as usize) as *const c_void, data.add(k as usize) as *const c_void, 4);
            if rc != 0 { dev_dbg!((*priv_).dev, "chn %d %s is not same\n", j as c_int, names[idx]); }
            i += 4; k += 4; idx += 1;
        }
        j += 1;
    }
    0
}

unsafe fn calib_data_get(tas_priv: *mut tasdevice_priv, reg: c_int, dst: *mut u8) -> c_int {
    let clt = (*tas_priv).client as *mut i2c_client;
    let tasdev = (*tas_priv).tasdevice;
    let mut rc: c_int = -1;
    let mut i = 0;
    while i < (*tas_priv).ndev {
        if (*clt).addr == (*tasdev.add(i as usize)).dev_addr {
            *dst = i as u8;
            rc = tasdevice_dev_bulk_read(tas_priv, i, reg as c_uint, dst.add(1), 4);
            break;
        }
        i += 1;
    }
    rc
}

unsafe fn partial_cali_data_update(reg: *mut c_int, j: c_int) -> c_int {
    match tas2781_cali_start_reg[j as usize].reg {
        0 => *reg.add(0),
        TAS2781_PRM_PLT_FLAG_REG => *reg.add(1),
        TAS2781_PRM_SINEGAIN_REG => *reg.add(2),
        TAS2781_PRM_SINEGAIN2_REG => *reg.add(3),
        _ => 0,
    }
}

unsafe fn sngl_calib_start(tas_priv: *mut tasdevice_priv, i: c_int, reg: *mut c_int, dat: *mut u8) {
    let tasdev = (*tas_priv).tasdevice;
    let p = (*tasdev.add(i as usize)).cali_data_backup;
    let t = &mut (*tasdev.add(i as usize)).alp_cali_bckp as *mut bulk_reg_val;
    let sum = tas2781_cali_start_reg.len() as c_int;
    let mut val = [0u8; 4];
    if p.is_null() { return; }
    let mut j = 0;
    while j < sum {
        if (*p.add(j as usize)).val_len == 1 {
            if (*p.add(j as usize)).is_locked {
                tasdevice_dev_write(tas_priv, i, TAS2781_TEST_UNLOCK_REG, TAS2781_TEST_PAGE_UNLOCK);
            }
            tasdevice_dev_read(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr() as *mut c_int);
        } else {
            if (*tas_priv).dspbin_typ == 0 {
                let r = partial_cali_data_update(reg, j);
                if r != 0 { (*p.add(j as usize)).reg = r as c_uint; }
            }
            if (*p.add(j as usize)).reg != 0 {
                tasdevice_dev_bulk_read(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr(), 4);
            }
        }
        j += 1;
    }
    if (*tas_priv).dspbin_typ == TASDEV_ALPHA {
        tasdevice_dev_bulk_read(tas_priv, i, (*t).reg, (*t).val.as_mut_ptr(), 4);
    }
    j = 0;
    while j < sum - 4 {
        if (*p.add(j as usize)).val_len == 1 {
            if (*p.add(j as usize)).is_locked {
                tasdevice_dev_write(tas_priv, i, TAS2781_TEST_UNLOCK_REG, TAS2781_TEST_PAGE_UNLOCK);
            }
            tasdevice_dev_write(tas_priv, i, (*p.add(j as usize)).reg, tas2781_cali_start_reg[j as usize].val[0] as c_uint);
        }
        j += 1;
    }
    if (*tas_priv).dspbin_typ == TASDEV_ALPHA {
        val = [0x00, 0x00, 0x21, 0x8e];
    } else {
        val.copy_from_slice(&tas2781_cali_start_reg[j as usize].val);
    }
    tasdevice_dev_bulk_write(tas_priv, i, (*p.add(j as usize)).reg, val.as_mut_ptr(), 4);
    tasdevice_dev_bulk_write(tas_priv, i, (*p.add((j + 1) as usize)).reg, tas2781_cali_start_reg[(j + 1) as usize].val.as_ptr() as *mut u8, 4);
    tasdevice_dev_bulk_write(tas_priv, i, (*p.add((j + 2) as usize)).reg, dat.add(1), 4);
    tasdevice_dev_bulk_write(tas_priv, i, (*p.add((j + 3) as usize)).reg, dat.add(5), 4);
    if (*tas_priv).dspbin_typ == TASDEV_ALPHA {
        val = [0x00, 0x00, 0x2a, 0x0b];
        tasdevice_dev_bulk_read(tas_priv, i, (*t).reg, val.as_mut_ptr(), 4);
    }
}

unsafe extern "C" fn tas2781_calib_start_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp);
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dat = bytes_data(ucontrol);
    let mut reg = [0i32; 4];
    let mut j = 0;
    let _guard = guard_mutex(&mut (*priv_).codec_lock);
    if (*priv_).chip_id != TAS2781 || (*bytes_ext).max != *dat as c_uint || *dat.add(1) != b'r' {
        dev_err!((*priv_).dev, "%s: package fmt or chipid incorrect\n", cstr!("tas2781_calib_start_put"));
        return 0;
    }
    j += 2;
    let mut i = 0;
    while i < reg.len() {
        reg[i] = TASDEVICE_REG(*dat.add(j), *dat.add(j + 1), *dat.add(j + 2)) as c_int;
        j += 3;
        i += 1;
    }
    i = 0;
    while i < (*priv_).ndev as usize {
        let k = i * 9 + j;
        if *dat.add(k) != i as u8 {
            dev_err!((*priv_).dev, "%s:no cal-setting for dev %d\n", cstr!("tas2781_calib_start_put"), i as c_int);
        } else {
            sngl_calib_start(priv_, i as c_int, reg.as_mut_ptr(), dat.add(k));
        }
        i += 1;
    }
    1
}

unsafe fn tas2781_calib_stop_put(priv_: *mut tasdevice_priv) {
    let sum = tas2781_cali_start_reg.len() as c_int;
    let mut i = 0;
    while i < (*priv_).ndev {
        let tasdev = (*priv_).tasdevice;
        let p = (*tasdev.add(i as usize)).cali_data_backup;
        let t = &mut (*tasdev.add(i as usize)).alp_cali_bckp as *mut bulk_reg_val;
        if !p.is_null() {
            let mut j = 0;
            while j < sum {
                if (*p.add(j as usize)).val_len == 1 {
                    if (*p.add(j as usize)).is_locked {
                        tasdevice_dev_write(priv_, i, TAS2781_TEST_UNLOCK_REG, TAS2781_TEST_PAGE_UNLOCK);
                    }
                    tasdevice_dev_write(priv_, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val[0] as c_uint);
                } else if (*p.add(j as usize)).reg != 0 {
                    tasdevice_dev_bulk_write(priv_, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr(), 4);
                }
                j += 1;
            }
            if (*priv_).dspbin_typ == TASDEV_ALPHA {
                tasdevice_dev_bulk_write(priv_, i, (*t).reg, (*t).val.as_mut_ptr(), 4);
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn tas2563_calib_start_put(kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let q = tas2563_cali_start_reg.as_ptr() as *mut bulk_reg_val;
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let sum = tas2563_cali_start_reg.len() as c_int;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    if (*tas_priv).chip_id != TAS2563 { return -1; }
    let mut i = 0;
    while i < (*tas_priv).ndev {
        let tasdev = (*tas_priv).tasdevice;
        let p = (*tasdev.add(i as usize)).cali_data_backup;
        if !p.is_null() {
            let mut j = 0;
            while j < sum {
                if (*p.add(j as usize)).val_len == 1 {
                    tasdevice_dev_read(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr() as *mut c_int);
                } else {
                    tasdevice_dev_bulk_read(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr(), 4);
                }
                j += 1;
            }
            j = 0;
            while j < sum {
                if (*p.add(j as usize)).val_len == 1 {
                    tasdevice_dev_write(tas_priv, i, (*p.add(j as usize)).reg, (*q.add(j as usize)).val[0] as c_uint);
                } else {
                    tasdevice_dev_bulk_write(tas_priv, i, (*p.add(j as usize)).reg, (*q.add(j as usize)).val.as_mut_ptr(), 4);
                }
                j += 1;
            }
        }
        i += 1;
    }
    1
}

unsafe fn tas2563_calib_stop_put(tas_priv: *mut tasdevice_priv) {
    let sum = tas2563_cali_start_reg.len() as c_int;
    let mut i = 0;
    while i < (*tas_priv).ndev {
        let tasdev = (*tas_priv).tasdevice;
        let p = (*tasdev.add(i as usize)).cali_data_backup;
        if !p.is_null() {
            let mut j = 0;
            while j < sum {
                if (*p.add(j as usize)).val_len == 1 {
                    tasdevice_dev_write(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val[0] as c_uint);
                } else {
                    tasdevice_dev_bulk_write(tas_priv, i, (*p.add(j as usize)).reg, (*p.add(j as usize)).val.as_mut_ptr(), 4);
                }
                j += 1;
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn tasdev_calib_stop_put(kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp);
    let _guard = guard_mutex(&mut (*priv_).codec_lock);
    if (*priv_).chip_id == TAS2563 { tas2563_calib_stop_put(priv_); } else { tas2781_calib_stop_put(priv_); }
    let mut i = 0;
    while i < (*priv_).ndev {
        (*(*priv_).tasdevice.add(i as usize)).cur_prog = -1;
        i += 1;
    }
    1
}

unsafe extern "C" fn tasdev_cali_data_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(comp);
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let cali_data = &mut (*priv_).cali_data as *mut calidata;
    let p = &mut (*cali_data).cali_reg_array as *mut cali_reg;
    let src = bytes_data(ucontrol);
    let dst = (*cali_data).data;
    let mut i: c_int = 0;
    let _guard = guard_mutex(&mut (*priv_).codec_lock);
    if *src != (*bytes_ext).max as u8 || *src.add(1) != b'r' {
        dev_err!((*priv_).dev, "%s: pkg fmt invalid\n", cstr!("tasdev_cali_data_put"));
        return 0;
    }
    let mut j = 0;
    while j < (*priv_).ndev {
        if *src.add((17 + j * 21) as usize) != j as u8 {
            dev_err!((*priv_).dev, "%s: pkg fmt invalid\n", cstr!("tasdev_cali_data_put"));
            return 0;
        }
        j += 1;
    }
    i += 2;
    if (*priv_).dspbin_typ == TASDEV_BASIC {
        (*p).r0_reg = TASDEVICE_REG(*src.add(i as usize), *src.add((i + 1) as usize), *src.add((i + 2) as usize)); i += 3;
        (*p).r0_low_reg = TASDEVICE_REG(*src.add(i as usize), *src.add((i + 1) as usize), *src.add((i + 2) as usize)); i += 3;
        (*p).invr0_reg = TASDEVICE_REG(*src.add(i as usize), *src.add((i + 1) as usize), *src.add((i + 2) as usize)); i += 3;
        (*p).pow_reg = TASDEVICE_REG(*src.add(i as usize), *src.add((i + 1) as usize), *src.add((i + 2) as usize)); i += 3;
        (*p).tlimit_reg = TASDEVICE_REG(*src.add(i as usize), *src.add((i + 1) as usize), *src.add((i + 2) as usize)); i += 3;
    } else {
        i += 15;
    }
    memcpy(dst as *mut c_void, src.add(i as usize) as *const c_void, (*cali_data).total_sz as usize);
    1
}

unsafe extern "C" fn tas2781_latch_reg_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let clt = (*tas_priv).client as *mut i2c_client;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let tasdev = (*tas_priv).tasdevice;
    let dst = bytes_data(ucontrol);
    let mut val: c_int = 0;
    let mut rc: c_int = -1;
    *dst = (*bytes_ext).max as u8;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    let mut i = 0;
    while i < (*tas_priv).ndev {
        if (*clt).addr == (*tasdev.add(i as usize)).dev_addr {
            *dst.add(1) = i as u8;
            rc = tasdevice_dev_read(tas_priv, i, TAS2781_RUNTIME_LATCH_RE_REG, &mut val);
            if rc < 0 { dev_err!((*tas_priv).dev, "%s, get value error\n", cstr!("tas2781_latch_reg_get")); } else { *dst.add(2) = val as u8; }
            break;
        }
        i += 1;
    }
    rc
}

unsafe extern "C" fn tasdev_tf_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dst = bytes_data(ucontrol);
    let mut reg = TAS2781_RUNTIME_RE_REG_TF;
    if (*tas_priv).chip_id == TAS2781 {
        let tas_fmw = (*tas_priv).fmw;
        let p = &mut (*tas_fmw).fct_par_addr;
        reg = TAS2781_RUNTIME_RE_REG_TF;
        if (*tas_priv).dspbin_typ != 0 { reg = TASDEVICE_REG(p.tf_reg[0], p.tf_reg[1], p.tf_reg[2]); }
    } else {
        reg = TAS2563_RUNTIME_RE_REG_TF;
    }
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    *dst = (*bytes_ext).max as u8;
    calib_data_get(tas_priv, reg as c_int, dst.add(1))
}

unsafe extern "C" fn tasdev_re_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dst = bytes_data(ucontrol);
    let mut reg = TAS2781_RUNTIME_RE_REG;
    if (*tas_priv).chip_id == TAS2781 {
        let tas_fmw = (*tas_priv).fmw;
        let p = &mut (*tas_fmw).fct_par_addr;
        if (*tas_priv).dspbin_typ != 0 { reg = TASDEVICE_REG(p.r0_reg[0], p.r0_reg[1], p.r0_reg[2]); }
    } else {
        reg = TAS2563_RUNTIME_RE_REG;
    }
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    *dst = (*bytes_ext).max as u8;
    calib_data_get(tas_priv, reg as c_int, dst.add(1))
}

unsafe extern "C" fn tasdev_r0_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let cali_data = &mut (*tas_priv).cali_data as *mut calidata;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dst = bytes_data(ucontrol);
    let reg;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    if (*tas_priv).chip_id == TAS2563 { reg = TAS2563_PRM_R0_REG; }
    else if (*cali_data).cali_reg_array.r0_reg != 0 { reg = (*cali_data).cali_reg_array.r0_reg; }
    else { return -1; }
    *dst = (*bytes_ext).max as u8;
    calib_data_get(tas_priv, reg as c_int, dst.add(1))
}

unsafe extern "C" fn tasdev_XMA1_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let tas_fmw = (*tas_priv).fmw;
    let p = &mut (*tas_fmw).fct_par_addr;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dst = bytes_data(ucontrol);
    let mut reg = TASDEVICE_XM_A1_REG;
    if (*tas_priv).dspbin_typ != 0 { reg = TASDEVICE_REG(p.a1_reg[0], p.a1_reg[1], p.a1_reg[2]); }
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    *dst = (*bytes_ext).max as u8;
    calib_data_get(tas_priv, reg as c_int, dst.add(1))
}

unsafe extern "C" fn tasdev_XMA2_data_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let comp = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let tas_fmw = (*tas_priv).fmw;
    let p = &mut (*tas_fmw).fct_par_addr;
    let bytes_ext = (*kcontrol).private_value as *mut soc_bytes_ext;
    let dst = bytes_data(ucontrol);
    let mut reg = TASDEVICE_XM_A2_REG;
    if (*tas_priv).dspbin_typ != 0 { reg = TASDEVICE_REG(p.a2_reg[0], p.a2_reg[1], p.a2_reg[2]); }
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    *dst = (*bytes_ext).max as u8;
    calib_data_get(tas_priv, reg as c_int, dst.add(1))
}

unsafe extern "C" fn tasdev_nop_get(_kcontrol: *mut snd_kcontrol, _ucontrol: *mut snd_ctl_elem_value) -> c_int { 0 }

unsafe extern "C" fn tasdevice_digital_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_dev = snd_soc_component_get_drvdata(codec);
    let mut l: c_uint = 0;
    let mut r: c_uint = (*mc).max;
    let reg = (*mc).reg;
    let mut data = [0u8; 4];
    let _guard = guard_mutex(&mut (*tas_dev).codec_lock);
    let ret = tasdevice_dev_bulk_read(tas_dev, 0, reg, data.as_mut_ptr(), 4);
    if ret != 0 {
        dev_err!((*tas_dev).dev, "%s, get AMP vol error\n", cstr!("tasdevice_digital_gain_get"));
        return ret;
    }
    let target = get_unaligned_be32(data.as_ptr());
    while r > 1 + l {
        let mid = (l + r) / 2;
        let ar_mid = get_unaligned_be32(*(*tas_dev).dvc_tlv_table.add(mid as usize));
        if target < ar_mid { r = mid; } else { l = mid; }
    }
    let ar_l = get_unaligned_be32(*(*tas_dev).dvc_tlv_table.add(l as usize));
    let ar_r = get_unaligned_be32(*(*tas_dev).dvc_tlv_table.add(r as usize));
    set_integer_value0(ucontrol, if (target as i64 - ar_l as i64).abs() <= (target as i64 - ar_r as i64).abs() { l as c_long } else { r as c_long });
    0
}

unsafe extern "C" fn tasdevice_digital_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_dev = snd_soc_component_get_drvdata(codec);
    let mut vol = integer_value0(ucontrol) as c_int;
    let mut status = 0;
    let max = (*mc).max as c_int;
    let reg = (*mc).reg;
    let mut data = [0u8; 4];
    vol = clamp(vol, 0, max);
    let _guard = guard_mutex(&mut (*tas_dev).codec_lock);
    let mut ret = tasdevice_dev_bulk_read(tas_dev, 0, reg, data.as_mut_ptr(), 4);
    if ret != 0 {
        dev_err!((*tas_dev).dev, "%s, get AMP vol error\n", cstr!("tasdevice_digital_gain_put"));
        return -1;
    }
    let volrd = get_unaligned_be32(data.as_ptr());
    let volwr = get_unaligned_be32(*(*tas_dev).dvc_tlv_table.add(vol as usize));
    if volrd == volwr { return 0; }
    let mut i = 0;
    while i < (*tas_dev).ndev {
        ret = tasdevice_dev_bulk_write(tas_dev, i, reg, *(*tas_dev).dvc_tlv_table.add(vol as usize) as *mut u8, 4);
        if ret != 0 {
            dev_err!((*tas_dev).dev, "%s, set digital vol error in dev %d\n", cstr!("tasdevice_digital_gain_put"), i);
            status |= BIT(i);
        }
        i += 1;
    }
    if status != 0 { return -1; }
    1
}

// Static kcontrol arrays are translated as zeroed placeholders with the original macro intent.
// SOC_SINGLE_EXT, SND_SOC_BYTES_EXT and SOC_SINGLE_RANGE_EXT_TLV are ALSA C macros supplied externally.
static tasdevice_cali_controls: [snd_kcontrol_new; 6] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 6];
static tas2x20_snd_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 2];
static tas2781_snd_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 2];
static tas5825_snd_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 2];
static tas2781_cali_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 1];
static tas2563_snd_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 1];
static tas2563_cali_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { name: ptr::null_mut(), iface: 0, info: None, get: None, put: None, private_value: 0 }; 1];

unsafe extern "C" fn tasdevice_set_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mut ret = 0;
    if (*tas_priv).rcabin.profile_cfg_id != integer_value0(ucontrol) as c_uint {
        (*tas_priv).rcabin.profile_cfg_id = integer_value0(ucontrol) as c_uint;
        ret = 1;
    }
    ret
}

unsafe extern "C" fn tasdevice_set_capture_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let user_prof_id = integer_value0(ucontrol) as c_uint;
    let max_valid_id = (*tas_priv).rcabin.ncfgs - 1;
    let mut ret = 0;
    if (*tas_priv).rcabin.ncfgs == 0 || user_prof_id > max_valid_id { return -EINVAL; }
    if (*tas_priv).rcabin.capture_profile_id != user_prof_id {
        (*tas_priv).rcabin.capture_profile_id = user_prof_id;
        ret = 1;
    }
    ret
}

unsafe extern "C" fn tasdevice_info_active_num(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).min = 0;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).max = ((*tas_priv).ndev - 1) as c_long;
    0
}

unsafe extern "C" fn tasdevice_info_chip_id(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).min = TAS2020 as c_long;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).max = TAS_OTHERS as c_long;
    0
}

unsafe extern "C" fn tasdevice_info_programs(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let tas_fw = (*tas_priv).fmw;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).min = 0;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).max = (*tas_fw).nr_programs as c_long;
    0
}

unsafe extern "C" fn tasdevice_info_configurations(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let tas_fw = (*tas_priv).fmw;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).min = 0;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).max = (*tas_fw).nr_configurations as c_long - 1;
    0
}

unsafe extern "C" fn tasdevice_info_profile(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).min = 0;
    (*core::mem::ManuallyDrop::into_inner(ptr::read(&(*uinfo).value.integer))).max = (*tas_priv).rcabin.ncfgs as c_long - 1;
    0
}

unsafe extern "C" fn tasdevice_get_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    set_integer_value0(ucontrol, (*tas_priv).rcabin.profile_cfg_id as c_long);
    0
}

unsafe extern "C" fn tasdevice_get_capture_profile_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let max_valid_id = if (*tas_priv).rcabin.ncfgs > 0 { (*tas_priv).rcabin.ncfgs - 1 } else { 0 };
    let current_prof_id = (*tas_priv).rcabin.capture_profile_id;
    set_integer_value0(ucontrol, min_u(current_prof_id, max_valid_id) as c_long);
    0
}

unsafe extern "C" fn tasdevice_get_chip_id(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    set_integer_value0(ucontrol, (*tas_priv).chip_id as c_long);
    0
}

unsafe fn tasdevice_create_control(tas_priv: *mut tasdevice_priv) -> c_int {
    let mut ret;
    let nr_controls = 1;
    let mut mix_index = 0usize;
    let mut prof_ctrls = devm_kcalloc((*tas_priv).dev, nr_controls, size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if prof_ctrls.is_null() { return -ENOMEM; }
    let mut name = devm_kstrdup((*tas_priv).dev, cstr!("Speaker Profile Id"), GFP_KERNEL);
    if name.is_null() { return -ENOMEM; }
    (*prof_ctrls.add(mix_index)).name = name;
    (*prof_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*prof_ctrls.add(mix_index)).info = Some(tasdevice_info_profile);
    (*prof_ctrls.add(mix_index)).get = Some(tasdevice_get_profile_id);
    (*prof_ctrls.add(mix_index)).put = Some(tasdevice_set_profile_id);
    mix_index += 1;
    ret = snd_soc_add_component_controls((*tas_priv).codec, prof_ctrls, if nr_controls < mix_index { nr_controls } else { mix_index } as c_uint);
    mix_index = 0;
    match (*tas_priv).chip_id {
        TAS2563 | TAS2568 | TAS2570 | TAS2572 | TAS2573 | TAS2574 | TAS2781 => {
            prof_ctrls = devm_kcalloc((*tas_priv).dev, nr_controls, size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
            if prof_ctrls.is_null() { return -ENOMEM; }
            name = devm_kstrdup((*tas_priv).dev, cstr!("Speaker Capture Profile Id"), GFP_KERNEL);
            if name.is_null() { return -ENOMEM; }
            (*prof_ctrls.add(mix_index)).name = name;
            (*prof_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
            (*prof_ctrls.add(mix_index)).info = Some(tasdevice_info_profile);
            (*prof_ctrls.add(mix_index)).get = Some(tasdevice_get_capture_profile_id);
            (*prof_ctrls.add(mix_index)).put = Some(tasdevice_set_capture_profile_id);
            mix_index += 1;
            ret = snd_soc_add_component_controls((*tas_priv).codec, prof_ctrls, if nr_controls < mix_index { nr_controls } else { mix_index } as c_uint);
        }
        _ => {}
    }
    ret
}

unsafe extern "C" fn tasdevice_program_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    set_integer_value0(ucontrol, (*tas_priv).cur_prog as c_long);
    0
}

unsafe extern "C" fn tasdevice_program_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let nr_program = integer_value0(ucontrol) as c_uint;
    let mut ret = 0;
    if (*tas_priv).cur_prog != nr_program { (*tas_priv).cur_prog = nr_program; ret = 1; }
    ret
}

unsafe extern "C" fn tasdevice_configuration_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    set_integer_value0(ucontrol, (*tas_priv).cur_conf as c_long);
    0
}

unsafe extern "C" fn tasdevice_configuration_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let nr_configuration = integer_value0(ucontrol) as c_uint;
    let mut ret = 0;
    if (*tas_priv).cur_conf != nr_configuration { (*tas_priv).cur_conf = nr_configuration; ret = 1; }
    ret
}

unsafe extern "C" fn tasdevice_active_num_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let clt = (*tas_priv).client as *mut i2c_client;
    let tasdev = (*tas_priv).tasdevice;
    let mut i = 0;
    while i < (*tas_priv).ndev {
        if (*clt).addr == (*tasdev.add(i as usize)).dev_addr {
            set_integer_value0(ucontrol, i as c_long);
            return 0;
        }
        i += 1;
    }
    -1
}

unsafe extern "C" fn tasdevice_active_num_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mut dev_id = integer_value0(ucontrol) as c_int;
    let max = (*tas_priv).ndev - 1;
    dev_id = clamp(dev_id, 0, max);
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    tasdev_chn_switch(tas_priv, dev_id)
}

unsafe fn tasdevice_dsp_create_ctrls(tas_priv: *mut tasdevice_priv) -> c_int {
    let nr_controls = 5usize;
    let mut mix_index = 0usize;
    let dsp_ctrls = devm_kcalloc((*tas_priv).dev, nr_controls, size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if dsp_ctrls.is_null() { return -ENOMEM; }
    let prog_name = devm_kstrdup((*tas_priv).dev, cstr!("Speaker Program Id"), GFP_KERNEL);
    if prog_name.is_null() { return -ENOMEM; }
    (*dsp_ctrls.add(mix_index)).name = prog_name;
    (*dsp_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*dsp_ctrls.add(mix_index)).info = Some(tasdevice_info_programs);
    (*dsp_ctrls.add(mix_index)).get = Some(tasdevice_program_get);
    (*dsp_ctrls.add(mix_index)).put = Some(tasdevice_program_put);
    mix_index += 1;
    let conf_name = devm_kstrdup((*tas_priv).dev, cstr!("Speaker Config Id"), GFP_KERNEL);
    if conf_name.is_null() { return -ENOMEM; }
    (*dsp_ctrls.add(mix_index)).name = conf_name;
    (*dsp_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*dsp_ctrls.add(mix_index)).info = Some(tasdevice_info_configurations);
    (*dsp_ctrls.add(mix_index)).get = Some(tasdevice_configuration_get);
    (*dsp_ctrls.add(mix_index)).put = Some(tasdevice_configuration_put);
    mix_index += 1;
    let active_dev_num = devm_kstrdup((*tas_priv).dev, cstr!("Activate Tasdevice Num"), GFP_KERNEL);
    if active_dev_num.is_null() { return -ENOMEM; }
    (*dsp_ctrls.add(mix_index)).name = active_dev_num;
    (*dsp_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*dsp_ctrls.add(mix_index)).info = Some(tasdevice_info_active_num);
    (*dsp_ctrls.add(mix_index)).get = Some(tasdevice_active_num_get);
    (*dsp_ctrls.add(mix_index)).put = Some(tasdevice_active_num_put);
    mix_index += 1;
    let chip_id = devm_kstrdup((*tas_priv).dev, cstr!("Tasdevice Chip Id"), GFP_KERNEL);
    if chip_id.is_null() { return -ENOMEM; }
    (*dsp_ctrls.add(mix_index)).name = chip_id;
    (*dsp_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*dsp_ctrls.add(mix_index)).info = Some(tasdevice_info_chip_id);
    (*dsp_ctrls.add(mix_index)).get = Some(tasdevice_get_chip_id);
    mix_index += 1;
    let fw_load = devm_kstrdup((*tas_priv).dev, cstr!("Speaker Force Firmware Load"), GFP_KERNEL);
    if fw_load.is_null() { return -ENOMEM; }
    (*dsp_ctrls.add(mix_index)).name = fw_load;
    (*dsp_ctrls.add(mix_index)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*dsp_ctrls.add(mix_index)).info = Some(snd_soc_info_bool_ext);
    (*dsp_ctrls.add(mix_index)).put = Some(tasdev_force_fwload_put);
    (*dsp_ctrls.add(mix_index)).get = Some(tasdev_force_fwload_get);
    (*dsp_ctrls.add(mix_index)).private_value = 0;
    mix_index += 1;
    snd_soc_add_component_controls((*tas_priv).codec, dsp_ctrls, if nr_controls < mix_index { nr_controls } else { mix_index } as c_uint)
}

unsafe fn cali_reg_update(p: *mut bulk_reg_val, t: *mut fct_param_address) {
    let sum = tas2781_cali_start_reg.len();
    let mut j = 0;
    while j < sum {
        let reg = match tas2781_cali_start_reg[j].reg {
            0 => TASDEVICE_REG((*t).thr[0], (*t).thr[1], (*t).thr[2]),
            TAS2781_PRM_PLT_FLAG_REG => TASDEVICE_REG((*t).plt_flg[0], (*t).plt_flg[1], (*t).plt_flg[2]),
            TAS2781_PRM_SINEGAIN_REG => TASDEVICE_REG((*t).sin_gn[0], (*t).sin_gn[1], (*t).sin_gn[2]),
            TAS2781_PRM_SINEGAIN2_REG => TASDEVICE_REG((*t).sin_gn2[0], (*t).sin_gn2[1], (*t).sin_gn2[2]),
            _ => 0,
        };
        if reg != 0 { (*p.add(j)).reg = reg; }
        j += 1;
    }
}

unsafe fn alpa_cali_update(p: *mut bulk_reg_val, t: *mut fct_param_address) {
    (*p).is_locked = false;
    (*p).reg = TASDEVICE_REG((*t).thr2[0], (*t).thr2[1], (*t).thr2[2]);
    (*p).val_len = 4;
}

unsafe fn create_tas2781_cali_start_ktrl(priv_: *mut tasdevice_priv, cali_ctrl: *mut snd_kcontrol_new) -> c_int {
    let ext_cali_start = devm_kzalloc((*priv_).dev, size_of::<soc_bytes_ext>(), GFP_KERNEL) as *mut soc_bytes_ext;
    if ext_cali_start.is_null() { return -ENOMEM; }
    let cali_start_name = devm_kstrdup((*priv_).dev, cstr!("Calibration Start"), GFP_KERNEL);
    if cali_start_name.is_null() { return -ENOMEM; }
    (*ext_cali_start).max = (14 + (*priv_).ndev * 9) as c_uint;
    (*cali_ctrl).name = cali_start_name;
    (*cali_ctrl).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*cali_ctrl).info = Some(snd_soc_bytes_info_ext);
    (*cali_ctrl).put = Some(tas2781_calib_start_put);
    (*cali_ctrl).get = Some(tasdev_nop_get);
    (*cali_ctrl).private_value = ext_cali_start as c_ulong;
    0
}

unsafe fn tasdevice_create_cali_ctrls(priv_: *mut tasdevice_priv) -> c_int {
    let cali_data = &mut (*priv_).cali_data as *mut calidata;
    let tasdev = (*priv_).tasdevice;
    let fmw = (*priv_).fmw;
    let mut rc = snd_soc_add_component_controls((*priv_).codec, tasdevice_cali_controls.as_ptr(), tasdevice_cali_controls.len() as c_uint);
    if rc < 0 {
        dev_err!((*priv_).dev, "%s: Add cali controls err rc = %d", cstr!("tasdevice_create_cali_ctrls"), rc);
        return rc;
    }
    let cali_ctrls_base: *const snd_kcontrol_new;
    let mut nctrls: c_uint;
    let mut i = 0;
    if (*priv_).chip_id == TAS2781 {
        let t = &mut (*fmw).fct_par_addr as *mut fct_param_address;
        cali_ctrls_base = tas2781_cali_controls.as_ptr();
        nctrls = tas2781_cali_controls.len() as c_uint;
        while i < (*priv_).ndev {
            let p = kmemdup(tas2781_cali_start_reg.as_ptr() as *const c_void, size_of::<[bulk_reg_val; 13]>(), GFP_KERNEL) as *mut bulk_reg_val;
            (*tasdev.add(i as usize)).cali_data_backup = p;
            if p.is_null() { return -ENOMEM; }
            if (*priv_).dspbin_typ != 0 {
                cali_reg_update(p, t);
                if (*priv_).dspbin_typ == TASDEV_ALPHA {
                    alpa_cali_update(&mut (*tasdev.add(i as usize)).alp_cali_bckp, t);
                }
            }
            i += 1;
        }
    } else {
        cali_ctrls_base = tas2563_cali_controls.as_ptr();
        nctrls = tas2563_cali_controls.len() as c_uint;
        while i < (*priv_).ndev {
            (*tasdev.add(i as usize)).cali_data_backup = kmemdup(tas2563_cali_start_reg.as_ptr() as *const c_void, size_of::<[bulk_reg_val; 11]>(), GFP_KERNEL) as *mut bulk_reg_val;
            if (*tasdev.add(i as usize)).cali_data_backup.is_null() { return -ENOMEM; }
            i += 1;
        }
    }
    rc = snd_soc_add_component_controls((*priv_).codec, cali_ctrls_base, nctrls);
    if rc < 0 {
        dev_err!((*priv_).dev, "%s: Add chip cali ctrls err rc = %d", cstr!("tasdevice_create_cali_ctrls"), rc);
        return rc;
    }
    i = 0;
    nctrls = if (*priv_).chip_id == TAS2781 { 2 } else { 1 };
    let cali_ctrls = devm_kcalloc((*priv_).dev, nctrls as usize, size_of::<snd_kcontrol_new>(), GFP_KERNEL) as *mut snd_kcontrol_new;
    if cali_ctrls.is_null() { return -ENOMEM; }
    let ext_cali_data = devm_kzalloc((*priv_).dev, size_of::<soc_bytes_ext>(), GFP_KERNEL) as *mut soc_bytes_ext;
    if ext_cali_data.is_null() { return -ENOMEM; }
    let cali_name = devm_kstrdup((*priv_).dev, cstr!("Speaker Calibrated Data"), GFP_KERNEL);
    if cali_name.is_null() { return -ENOMEM; }
    (*cali_data).cali_dat_sz_per_dev = 20;
    (*ext_cali_data).max = ((*priv_).ndev as c_uint) * ((*cali_data).cali_dat_sz_per_dev + 1) + 1 + 15 + 1;
    (*priv_).cali_data.total_sz = ((*priv_).ndev as c_uint) * ((*cali_data).cali_dat_sz_per_dev + 1);
    (*cali_ctrls.add(i as usize)).name = cali_name;
    (*cali_ctrls.add(i as usize)).iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    (*cali_ctrls.add(i as usize)).info = Some(snd_soc_bytes_info_ext);
    (*cali_ctrls.add(i as usize)).get = Some(tasdev_cali_data_get);
    (*cali_ctrls.add(i as usize)).put = Some(tasdev_cali_data_put);
    (*cali_ctrls.add(i as usize)).private_value = ext_cali_data as c_ulong;
    i += 1;
    (*cali_data).data = devm_kzalloc((*priv_).dev, (*cali_data).total_sz as usize, GFP_KERNEL) as *mut u8;
    if (*cali_data).data.is_null() { return -ENOMEM; }
    *(*cali_data).data = 0xff;
    if (*priv_).chip_id == TAS2781 {
        rc = create_tas2781_cali_start_ktrl(priv_, cali_ctrls.add(i as usize));
        if rc != 0 { return rc; }
        i += 1;
    }
    snd_soc_add_component_controls((*priv_).codec, cali_ctrls, if nctrls < i as c_uint { nctrls } else { i as c_uint })
}

// CONFIG_SND_SOC_TAS2781_ACOUST_I2C conditional debugfs support.
unsafe extern "C" fn acoustic_ctl_read(file: *mut file, to: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let comp = (*file).private_data as *mut snd_soc_component;
    let tas_priv = snd_soc_component_get_drvdata(comp);
    let p = &mut (*tas_priv).acou_data as *mut acoustic_data;
    let mut ret: ssize_t = -1;
    if (*p).id == b'r' && (*p).len as usize == count && count <= size_of::<acoustic_data>() {
        ret = simple_read_from_buffer(to, count, ppos, p as *const c_void, (*p).len as usize);
    } else {
        dev_err!((*tas_priv).dev, "Not ready for get.\n");
    }
    ret
}

unsafe extern "C" fn acoustic_ctl_write(file: *mut file, from: *const c_char, count: size_t, _ppos: *mut loff_t) -> ssize_t {
    let comp = (*file).private_data as *mut snd_soc_component;
    let priv_ = snd_soc_component_get_drvdata(comp);
    let max_pkg_len = size_of::<acoustic_data>() as c_uint;
    let mut ret: c_int = -1;
    if count > size_of::<acoustic_data>() {
        dev_err!((*priv_).dev, "count(%u) is larger than max(%u).\n", count as c_uint, max_pkg_len);
        return ret as ssize_t;
    }
    let src = memdup_user(from, count);
    if IS_ERR(src) { return PTR_ERR(src) as ssize_t; }
    if *src > max_pkg_len as u8 && *src != count as u8 {
        dev_err!((*priv_).dev, "pkg(%u), max(%u), count(%u) mismatch.\n", *src as c_uint, max_pkg_len, count as c_uint);
        kfree(src as *mut c_void);
        return 0;
    }
    let len: c_int = match *src.add(1) {
        b'r' => *src.add(6) as c_int,
        b'w' => *src as c_int - 6,
        _ => {
            dev_err!((*priv_).dev, "%s Wrong code %02x.\n", cstr!("acoustic_ctl_write"), *src.add(1) as c_uint);
            kfree(src as *mut c_void);
            return 0;
        }
    };
    if len < 1 {
        dev_err!((*priv_).dev, "pkg fmt invalid %02x.\n", len);
        kfree(src as *mut c_void);
        return 0;
    }
    let mut j = 0;
    while j < (*priv_).ndev {
        if *src.add(2) as c_uint == (*(*priv_).tasdevice.add(j as usize)).dev_addr { break; }
        j += 1;
    }
    if j >= (*priv_).ndev {
        dev_err!((*priv_).dev, "no such device 0x%02x.\n", *src.add(2) as c_uint);
        kfree(src as *mut c_void);
        return 0;
    }
    let chn = j as c_int;
    let reg = TASDEVICE_REG(*src.add(3), *src.add(4), *src.add(5));
    let _guard = guard_mutex(&mut (*priv_).codec_lock);
    if *src.add(1) == b'w' {
        if len > 1 { ret = tasdevice_dev_bulk_write(priv_, chn, reg, src.add(6), len as c_uint); }
        else { ret = tasdevice_dev_write(priv_, chn, reg, *src.add(6) as c_uint); }
    } else {
        let p = &mut (*priv_).acou_data as *mut acoustic_data;
        let mut val: c_int = 0;
        memcpy(p as *mut c_void, src as *const c_void, 6);
        if len > 1 { ret = tasdevice_dev_bulk_read(priv_, chn, reg, (*p).data.as_mut_ptr(), len as c_uint); }
        else {
            ret = tasdevice_dev_read(priv_, chn, reg, &mut val);
            (*p).data[0] = val as u8;
        }
        (*p).len = len + 6;
    }
    if ret != 0 { dev_err!((*priv_).dev, "i2c communication error.\n"); } else { ret = count as c_int; }
    kfree(src as *mut c_void);
    ret as ssize_t
}

static acoustic_ctl_fops: file_operations = file_operations { open: Some(simple_open), read: Some(acoustic_ctl_read), write: Some(acoustic_ctl_write) };

unsafe extern "C" fn tasdevice_fw_ready(fmw: *const firmware, context: *mut c_void) {
    let tas_priv = context as *mut tasdevice_priv;
    let mut ret: c_int;
    let mut i: c_int;
    mutex_lock(&mut (*tas_priv).codec_lock);
    ret = tasdevice_rca_parser(tas_priv, fmw);
    if ret != 0 {
        tasdevice_config_info_remove(tas_priv);
        goto_out(tas_priv, fmw);
        return;
    }
    tasdevice_create_control(tas_priv);
    tasdevice_dsp_remove(tas_priv);
    tasdevice_calbin_remove(tas_priv);
    (*tas_priv).fw_state = TASDEVICE_RCA_FW_OK;
    match (*tas_priv).chip_id {
        TAS2020 | TAS2118 | TAS2120 | TAS2320 | TAS2568 | TAS2570 | TAS2572 | TAS2574 => {
            goto_out(tas_priv, fmw);
            return;
        }
        _ => {}
    }
    if !(*tas_priv).name_prefix.is_null() {
        scnprintf((*tas_priv).coef_binaryname.as_mut_ptr(), 64, cstr!("%s-%s_coef.bin"), (*tas_priv).name_prefix, (*tas_priv).dev_name.as_mut_ptr());
    } else {
        scnprintf((*tas_priv).coef_binaryname.as_mut_ptr(), 64, cstr!("%s_coef.bin"), (*tas_priv).dev_name.as_mut_ptr());
    }
    ret = tasdevice_dsp_parser(tas_priv);
    if ret != 0 {
        dev_err!((*tas_priv).dev, "dspfw load %s error\n", (*tas_priv).coef_binaryname.as_mut_ptr());
        goto_out(tas_priv, fmw);
        return;
    }
    ret = tasdevice_dsp_create_ctrls(tas_priv);
    if ret != 0 {
        dev_err!((*tas_priv).dev, "dsp controls error\n");
        goto_out(tas_priv, fmw);
        return;
    }
    (*tas_priv).fw_state = TASDEVICE_DSP_FW_ALL_OK;
    if (*tas_priv).chip_id == TAS2563 || (*tas_priv).chip_id == TAS2781 {
        ret = tasdevice_create_cali_ctrls(tas_priv);
        if ret != 0 {
            dev_err!((*tas_priv).dev, "cali controls error\n");
            goto_out(tas_priv, fmw);
            return;
        }
        i = 0;
        while i < (*tas_priv).ndev {
            if !(*tas_priv).name_prefix.is_null() {
                scnprintf((*tas_priv).cal_binaryname[i as usize].as_mut_ptr(), 64, cstr!("%s-%s_cal_0x%02x.bin"), (*tas_priv).name_prefix, (*tas_priv).dev_name.as_mut_ptr(), (*(*tas_priv).tasdevice.add(i as usize)).dev_addr);
            } else {
                scnprintf((*tas_priv).cal_binaryname[i as usize].as_mut_ptr(), 64, cstr!("%s_cal_0x%02x.bin"), (*tas_priv).dev_name.as_mut_ptr(), (*(*tas_priv).tasdevice.add(i as usize)).dev_addr);
            }
            ret = tas2781_load_calibration(tas_priv, (*tas_priv).cal_binaryname[i as usize].as_mut_ptr(), i);
            if ret != 0 {
                dev_err!((*tas_priv).dev, "%s: load %s error, keep default.\n", cstr!("tasdevice_fw_ready"), (*tas_priv).cal_binaryname[i as usize].as_mut_ptr());
            }
            i += 1;
        }
    }
    tasdevice_prmg_load(tas_priv, 0);
    (*tas_priv).cur_prog = 0;
    if (*tas_priv).rcabin.init_profile_id >= 0 {
        tasdevice_select_cfg_blk(tas_priv, (*tas_priv).rcabin.init_profile_id, TASDEVICE_BIN_BLK_PRE_POWER_UP);
    }
    // CONFIG_SND_SOC_TAS2781_ACOUST_I2C debugfs creation.
    let comp = (*tas_priv).codec;
    let debugfs_root = (*comp).debugfs_root;
    let acoustic_debugfs_node = if !(*tas_priv).name_prefix.is_null() {
        devm_kasprintf((*tas_priv).dev, GFP_KERNEL, cstr!("%s_acoustic_ctl"), (*tas_priv).name_prefix)
    } else {
        devm_kstrdup((*tas_priv).dev, cstr!("acoustic_ctl"), GFP_KERNEL)
    };
    debugfs_create_file(acoustic_debugfs_node, 0o644, debugfs_root, comp as *mut c_void, &acoustic_ctl_fops);
    goto_out(tas_priv, fmw);
}

unsafe fn goto_out(tas_priv: *mut tasdevice_priv, fmw: *const firmware) {
    if (*tas_priv).fw_state == TASDEVICE_RCA_FW_OK {
        match (*tas_priv).chip_id {
            TAS2563 | TAS2573 | TAS2781 | TAS5802 | TAS5806M | TAS5806MD | TAS5815 | TAS5822 | TAS5825 | TAS5827 | TAS5828 | TAS5830 | TAS5832 => {
                tasdevice_dsp_remove(tas_priv);
            }
            _ => {}
        }
    }
    mutex_unlock(&mut (*tas_priv).codec_lock);
    release_firmware(fmw);
}

unsafe extern "C" fn tasdevice_dapm_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mut state = 0;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    if event == SND_SOC_DAPM_PRE_PMD { state = 1; }
    tasdevice_tuning_switch(tas_priv, state, false);
    0
}

unsafe extern "C" fn tasdevice_capture_dapm_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let mut state = 0;
    let _guard = guard_mutex(&mut (*tas_priv).codec_lock);
    if event == SND_SOC_DAPM_PRE_PMD { state = 1; }
    tasdevice_tuning_switch(tas_priv, state, true);
    0
}

static tasdevice_dapm_widgets: [snd_soc_dapm_widget; 5] = [snd_soc_dapm_widget { _private: [] }; 5];
static tasdevice_audio_map: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: cstr!("SPK"), control: ptr::null(), source: cstr!("ASI") },
    snd_soc_dapm_route { sink: cstr!("OUT"), control: ptr::null(), source: cstr!("SPK") },
    snd_soc_dapm_route { sink: cstr!("ASI OUT"), control: ptr::null(), source: cstr!("DMIC") },
];

unsafe extern "C" fn tasdevice_startup(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let codec = (*dai).component;
    let tas_priv = snd_soc_component_get_drvdata(codec);
    match (*tas_priv).fw_state {
        TASDEVICE_RCA_FW_OK | TASDEVICE_DSP_FW_ALL_OK => 0,
        _ => -EINVAL,
    }
}

unsafe extern "C" fn tasdevice_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let tas_priv = snd_soc_dai_get_drvdata(dai);
    let fsrate = params_rate(params);
    match fsrate {
        48000 | 44100 => {}
        _ => {
            dev_err!((*tas_priv).dev, "%s: incorrect sample rate = %u\n", cstr!("tasdevice_hw_params"), fsrate);
            return -EINVAL;
        }
    }
    let slot_width = params_width(params);
    match slot_width {
        16 | 20 | 24 | 32 => {}
        _ => {
            dev_err!((*tas_priv).dev, "%s: incorrect slot width = %u\n", cstr!("tasdevice_hw_params"), slot_width);
            return -EINVAL;
        }
    }
    let bclk_rate = snd_soc_params_to_bclk(params);
    if bclk_rate < 0 {
        dev_err!((*tas_priv).dev, "%s: incorrect bclk rate = %d\n", cstr!("tasdevice_hw_params"), bclk_rate);
        return bclk_rate;
    }
    0
}

unsafe extern "C" fn tasdevice_set_dai_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let tas_priv = snd_soc_dai_get_drvdata(codec_dai);
    (*tas_priv).sysclk = freq;
    0
}

static tasdevice_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops { startup: Some(tasdevice_startup), hw_params: Some(tasdevice_hw_params), set_sysclk: Some(tasdevice_set_dai_sysclk) };
static mut tasdevice_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: cstr!("tasdev_codec"),
    id: 0,
    playback: snd_soc_pcm_stream { stream_name: cstr!("Playback"), channels_min: 1, channels_max: 4, rates: TASDEVICE_RATES, formats: TASDEVICE_FORMATS },
    capture: snd_soc_pcm_stream { stream_name: cstr!("Capture"), channels_min: 1, channels_max: 4, rates: TASDEVICE_RATES, formats: TASDEVICE_FORMATS },
    ops: &tasdevice_dai_ops,
    symmetric_rate: 1,
}];

unsafe extern "C" fn tasdevice_codec_probe(codec: *mut snd_soc_component) -> c_int {
    let tas_priv = snd_soc_component_get_drvdata(codec);
    let p: *const snd_kcontrol_new;
    let size: c_uint;
    match (*tas_priv).chip_id {
        TAS2020 | TAS2118 | TAS2120 | TAS2320 | TAS2568 | TAS2570 | TAS2572 | TAS2573 | TAS2574 => {
            p = tas2x20_snd_controls.as_ptr();
            size = tas2x20_snd_controls.len() as c_uint;
            (*tas_priv).dvc_tlv_table = tas2x20_dvc_table;
        }
        TAS2781 => {
            p = tas2781_snd_controls.as_ptr();
            size = tas2781_snd_controls.len() as c_uint;
        }
        TAS5802 | TAS5806M | TAS5806MD | TAS5815 | TAS5822 | TAS5825 | TAS5827 | TAS5828 | TAS5830 | TAS5832 => {
            p = tas5825_snd_controls.as_ptr();
            size = tas5825_snd_controls.len() as c_uint;
        }
        _ => {
            p = tas2563_snd_controls.as_ptr();
            size = tas2563_snd_controls.len() as c_uint;
            (*tas_priv).dvc_tlv_table = tas2563_dvc_table;
        }
    }
    let rc = snd_soc_add_component_controls(codec, p, size);
    if rc < 0 {
        dev_err!((*tas_priv).dev, "%s: Add control err rc = %d", cstr!("tasdevice_codec_probe"), rc);
        return rc;
    }
    (*tas_priv).name_prefix = (*codec).name_prefix;
    tascodec_init(tas_priv, codec, THIS_MODULE, tasdevice_fw_ready)
}

unsafe extern "C" fn tasdevice_deinit(context: *mut c_void) {
    let tas_priv = context as *mut tasdevice_priv;
    let tasdev = (*tas_priv).tasdevice;
    let mut i = 0;
    while i < (*tas_priv).ndev {
        kfree((*tasdev.add(i as usize)).cali_data_backup as *mut c_void);
        i += 1;
    }
    tasdevice_config_info_remove(tas_priv);
    tasdevice_dsp_remove(tas_priv);
    tasdevice_calbin_remove(tas_priv);
    (*tas_priv).fw_state = TASDEVICE_DSP_FW_PENDING;
}

unsafe extern "C" fn tasdevice_codec_remove(codec: *mut snd_soc_component) {
    let tas_priv = snd_soc_component_get_drvdata(codec);
    tasdevice_deinit(tas_priv as *mut c_void);
}

static soc_codec_driver_tasdevice: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tasdevice_codec_probe),
    remove: Some(tasdevice_codec_remove),
    dapm_widgets: tasdevice_dapm_widgets.as_ptr(),
    num_dapm_widgets: tasdevice_dapm_widgets.len() as c_uint,
    dapm_routes: tasdevice_audio_map.as_ptr(),
    num_dapm_routes: tasdevice_audio_map.len() as c_uint,
    idle_bias_on: 1,
    endianness: 1,
};

unsafe fn tasdevice_parse_dt(tas_priv: *mut tasdevice_priv) {
    let client = (*tas_priv).client as *mut i2c_client;
    let mut dev_addrs = [0u32; TASDEVICE_MAX_CHANNELS];
    let mut ndev: c_int = 0;
    if (*tas_priv).isacpi {
        ndev = device_property_read_u32_array(&mut (*client).dev, cstr!("ti,audio-slots"), ptr::null_mut(), 0);
        if ndev <= 0 {
            ndev = 1;
            dev_addrs[0] = (*client).addr;
        } else {
            ndev = if (ndev as usize) < dev_addrs.len() { ndev } else { dev_addrs.len() as c_int };
            let rc = device_property_read_u32_array(&mut (*client).dev, cstr!("ti,audio-slots"), dev_addrs.as_mut_ptr(), ndev as usize);
            if rc != 0 {
                ndev = 1;
                dev_addrs[0] = (*client).addr;
            }
        }
        (*tas_priv).irq = acpi_dev_gpio_irq_get(ACPI_COMPANION(&mut (*client).dev), 0);
    } else if IS_ENABLED_CONFIG_OF() {
        let np = (*(*tas_priv).dev).of_node;
        let mut addr: u64 = 0;
        let mut i = 0;
        while i < TASDEVICE_MAX_CHANNELS as c_int {
            if of_property_read_reg(np, i, &mut addr, ptr::null_mut()) != 0 { break; }
            dev_addrs[ndev as usize] = addr as u32;
            ndev += 1;
            i += 1;
        }
        (*tas_priv).irq = of_irq_get(np, 0);
    } else {
        ndev = 1;
        dev_addrs[0] = (*client).addr;
    }
    (*tas_priv).ndev = ndev;
    let mut i = 0;
    while i < ndev {
        (*(*tas_priv).tasdevice.add(i as usize)).dev_addr = dev_addrs[i as usize];
        i += 1;
    }
    (*tas_priv).reset = devm_gpiod_get_optional(&mut (*client).dev, cstr!("reset"), GPIOD_OUT_HIGH);
    if IS_ERR((*tas_priv).reset) {
        dev_err!((*tas_priv).dev, "%s Can't get reset GPIO\n", cstr!("tasdevice_parse_dt"));
    }
}

unsafe extern "C" fn tasdevice_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let tas_priv = tasdevice_kzalloc(i2c);
    if tas_priv.is_null() { return -ENOMEM; }
    dev_set_drvdata(&mut (*i2c).dev, tas_priv as *mut c_void);
    let id_data = if ACPI_HANDLE(&mut (*i2c).dev) {
        (*tas_priv).isacpi = true;
        acpi_device_get_match_data(&mut (*i2c).dev) as *mut i2c_device_id
    } else {
        (*tas_priv).isacpi = false;
        i2c_get_match_data(i2c) as *mut i2c_device_id
    };
    let mut ret;
    if id_data.is_null() {
        dev_err!(&mut (*i2c).dev, "No driver data\n");
        ret = -EINVAL;
    } else {
        (*tas_priv).chip_id = (*id_data).driver_data as uintptr_t as c_uint;
        strscpy((*tas_priv).dev_name.as_mut_ptr(), (*id_data).name.as_ptr(), size_of::<[c_char; 64]>());
        tasdevice_parse_dt(tas_priv);
        ret = tasdevice_init(tas_priv);
        if ret == 0 {
            tasdevice_reset(tas_priv);
            ret = devm_snd_soc_register_component((*tas_priv).dev, &soc_codec_driver_tasdevice, tasdevice_dai_driver.as_mut_ptr(), tasdevice_dai_driver.len() as c_int);
            if ret != 0 {
                dev_err!((*tas_priv).dev, "%s: codec register error:0x%08x\n", cstr!("tasdevice_i2c_probe"), ret);
            }
        }
    }
    if ret < 0 { tasdevice_remove(tas_priv); }
    ret
}

unsafe extern "C" fn tasdevice_i2c_remove(client: *mut i2c_client) {
    let tas_priv = i2c_get_clientdata(client);
    tasdevice_remove(tas_priv);
}

static tasdevice_acpi_match: [acpi_device_id; 22] = [
    acpi_device_id { id: *b"TXNW2020\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2020 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2118\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2118 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2120\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2120 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2320\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2320 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2563\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2563 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2568\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2568 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2570\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2570 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2572\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2572 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2573\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2573 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2574\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2574 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW2781\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS2781 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5802\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5802 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW806M\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5806M as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW806D\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5806MD as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5815\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5815 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5822\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5822 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5825\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5825 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5827\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5827 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5828\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5828 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5830\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5830 as usize) as kernel_ulong_t } },
    acpi_device_id { id: *b"TXNW5832\0\0\0\0\0\0\0\0" as [u8; 16] as [c_char; 16], driver_data: unsafe { tasdevice_id.as_ptr().add(TAS5832 as usize) as kernel_ulong_t } },
    acpi_device_id { id: [0; 16], driver_data: 0 },
];

// MODULE_DEVICE_TABLE(acpi, tasdevice_acpi_match);

static mut tasdevice_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_driver {
        name: cstr!("tasdev-codec"),
        of_match_table: tasdevice_of_match.as_ptr(),
        acpi_match_table: tasdevice_acpi_match.as_ptr(),
    },
    probe: Some(tasdevice_i2c_probe),
    remove: Some(tasdevice_i2c_remove),
};

// module_i2c_driver(tasdevice_i2c_driver);
// MODULE_AUTHOR("Shenghao Ding <shenghao-ding@ti.com>");
// MODULE_AUTHOR("Kevin Lu <kevin-lu@ti.com>");
// MODULE_DESCRIPTION("ASoC TAS2781 Driver");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_TAS2781_FMWLIB");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
