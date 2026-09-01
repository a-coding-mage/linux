// SPDX-License-Identifier: GPL-2.0-only
//
// aw88081.c  --  AW88081 ALSA SoC Audio driver
//
// Copyright (c) 2024 awinic Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_t = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type int16_t = i16;

const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const I2C_FUNC_I2C: c_uint = 1;
const REGMAP_ENDIAN_LITTLE: c_uint = 0;
const REGMAP_ENDIAN_BIG: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint = 3;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_NOPM: c_int = -1;

// Constants and macros supplied by aw88081.h / aw88395_device.h.
extern "C" {
    static AW88081_REG_MAX: c_uint;
    static AW88083_REG_MAX: c_uint;
    static AW88081_SYSST_REG: c_uint;
    static AW88081_BIT_PLL_CHECK: c_uint;
    static AW88081_DEV_SYSST_CHECK_MAX: c_int;
    static AW88081_2000_US: c_uint;
    static AW88081_PLLCTRL1_REG: c_uint;
    static AW88081_CCO_MUX_MASK: c_uint;
    static AW88081_CCO_MUX_DIVIDED_VALUE: c_uint;
    static AW88081_CCO_MUX_BYPASS_VALUE: c_uint;
    static AW88081_PWMCTRL4_REG: c_uint;
    static AW88081_NOISE_GATE_EN_MASK: c_uint;
    static AW88081_NO_SWS_SYSST_CHECK: c_uint;
    static AW88081_SWS_SYSST_CHECK: c_uint;
    static AW88081_BIT_SYSST_CHECK_MASK: c_uint;
    static AW88081_I2SCTRL3_REG: c_uint;
    static AW88081_I2STXEN_MASK: c_uint;
    static AW88081_I2STXEN_ENABLE_VALUE: c_uint;
    static AW88081_I2STXEN_DISABLE_VALUE: c_uint;
    static AW88081_SYSCTRL_REG: c_uint;
    static AW88081_PWDN_MASK: c_uint;
    static AW88081_PWDN_POWER_DOWN_VALUE: c_uint;
    static AW88081_PWDN_WORKING_VALUE: c_uint;
    static AW88081_EN_PA_MASK: c_uint;
    static AW88081_EN_PA_POWER_DOWN_VALUE: c_uint;
    static AW88081_EN_PA_WORKING_VALUE: c_uint;
    static AW88083_I2C_WEN_MASK: c_uint;
    static AW88083_I2C_WEN_ENABLE_VALUE: c_uint;
    static AW88083_I2C_WEN_DISABLE_VALUE: c_uint;
    static AW88083_AMPPD_MASK: c_uint;
    static AW88083_AMPPD_POWER_DOWN_VALUE: c_uint;
    static AW88083_AMPPD_WORKING_VALUE: c_uint;
    static AW88083_PLL_PD_MASK: c_uint;
    static AW88083_PLL_PD_WORKING_VALUE: c_uint;
    static AW88083_PLL_PD_POWER_DOWN_VALUE: c_uint;
    static AW88081_SYSINT_REG: c_uint;
    static AW88081_SYSCTRL2_REG: c_uint;
    static AW88081_MUTE_VOL: c_uint;
    static AW88081_VOL_MASK: c_uint;
    static AW88081_VOL_START_BIT: c_uint;
    static AW88081_HMUTE_MASK: c_uint;
    static AW88081_HMUTE_ENABLE_VALUE: c_uint;
    static AW88081_HMUTE_DISABLE_VALUE: c_uint;
    static AW88081_ULS_HMUTE_MASK: c_uint;
    static AW88081_ULS_HMUTE_ENABLE_VALUE: c_uint;
    static AW88081_ULS_HMUTE_DISABLE_VALUE: c_uint;
    static AW88395_DATA_TYPE_REG: usize;
    static AW88081_DEV_PW_ON: c_int;
    static AW88081_DEV_PW_OFF: c_int;
    static AW88081_SOFT_RESET_VALUE: c_uint;
    static AW88081_ID_REG: c_uint;
    static AW88081_START_RETRIES: c_int;
    static AW88081_DEV_FW_OK: c_int;
    static AW88081_SYNC_START: bool_t;
    static AW88081_ASYNC_START: bool_t;
    static AW88081_START_WORK_DELAY_MS: c_ulong;
    static AW88081_RATES: c_uint;
    static AW88081_FORMATS: c_uint;
    static FADE_TIME_MAX: c_uint;
    static AW88081_DEV_DEFAULT_CH: u32;
    static AW88081_CHIP_ID: c_uint;
    static AW88083_CHIP_ID: c_uint;
    static AW88395_DEV_NONE_TYPE_ID: c_int;
    static AW88081_VOLUME_STEP_DB: c_int;
    static AW88081_1000_US: c_uint;
    static AW88081_500_US: c_int;
    static AW88081_INIT_PROFILE: c_int;
    static AW88081_DEV_FW_FAILED: c_int;
    static AW88081_ACF_FILE: *const c_char;
    static AW88081_I2C_NAME: *const c_char;
    static AW88083_I2C_NAME: *const c_char;
    static system_dfl_wq: *mut workqueue_struct;
}

#[repr(C)]
enum aw8808x_type {
    AW88081,
    AW88083,
}

#[repr(C)]
struct aw88081 {
    aw_pa: *mut aw_device,
    lock: mutex,
    start_work: delayed_work,
    regmap: *mut regmap,
    aw_cfg: *mut aw_container,
    devtype: aw8808x_type,
    phase_sync: bool_t,
}

#[repr(C)]
struct regmap_config {
    val_bits: c_uint,
    reg_bits: c_uint,
    max_register: c_uint,
    reg_format_endian: c_uint,
    val_format_endian: c_uint,
}

#[repr(C)] struct regmap { _private: [u8; 0] }
#[repr(C)] struct device_node { _private: [u8; 0] }
#[repr(C)] struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] struct snd_kcontrol { private_value: c_ulong }
#[repr(C)] struct snd_ctl_elem_info { type_: c_uint, count: c_uint, value: snd_ctl_elem_info_value }
#[repr(C)] struct snd_ctl_elem_info_value { enumerated: snd_ctl_elem_info_enumerated }
#[repr(C)] struct snd_ctl_elem_info_enumerated { items: c_uint, item: c_uint, name: [c_char; 64] }
#[repr(C)] struct snd_ctl_elem_value { value: snd_ctl_elem_value_value }
#[repr(C)] struct snd_ctl_elem_value_value { integer: snd_ctl_elem_value_integer }
#[repr(C)] struct snd_ctl_elem_value_integer { value: [c_long; 128] }
type c_long = i64;
#[repr(C)] struct snd_soc_component { dev: *mut device }
#[repr(C)] struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context }
#[repr(C)] struct soc_mixer_control { min: c_int, max: c_int }
#[repr(C)] struct firmware { size: usize, data: *const u8 }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct work_struct { _private: [u8; 0] }
#[repr(C)] struct delayed_work { work: work_struct }
#[repr(C)] struct device { of_node: *mut device_node }
#[repr(C)] struct i2c_client { dev: device, adapter: *mut i2c_adapter }
#[repr(C)] struct i2c_device_id { name: *const c_char, driver_data: aw8808x_type }
#[repr(C)] struct of_device_id { compatible: *const c_char }
#[repr(C)] struct device_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] struct i2c_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, id_table: *const i2c_device_id }

#[repr(C)] struct aw_volume_desc { init_volume: c_uint, ctl_volume: c_int, mute_volume: c_uint }
#[repr(C)] struct aw_sec_data_desc { data: *mut u8, len: c_uint }
#[repr(C)] struct aw_prof_desc { id: usize, sec_desc: *mut aw_sec_data_desc }
#[repr(C)] struct aw_prof_info { count: c_int, prof_desc: *mut aw_prof_desc, prof_name_list: *mut *mut c_char, prof_type: c_int }
#[repr(C)]
struct aw_device {
    regmap: *mut regmap,
    dev: *mut device,
    volume_desc: aw_volume_desc,
    fade_step: c_int,
    fade_in_time: c_int,
    fade_out_time: c_int,
    prof_cur: c_int,
    prof_index: c_int,
    prof_info: aw_prof_info,
    status: c_int,
    fw_status: c_int,
    i2c: *mut i2c_client,
    chip_id: c_uint,
    acf: *mut c_void,
    channel: u32,
}
#[repr(C)] struct aw_container { len: c_int, data: [u8; 0] }

#[repr(C)] struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint }
#[repr(C)] struct snd_soc_dai_driver { name: *const c_char, id: c_int, playback: snd_soc_pcm_stream, capture: snd_soc_pcm_stream }
#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget_def { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    dapm_widgets: *const snd_soc_dapm_widget_def,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    controls: *const snd_kcontrol_new,
    num_controls: usize,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aw88081;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn of_property_read_u32(np: *mut device_node, name: *const c_char, out: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, name: *const c_char) -> bool_t;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_t;
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool_t;
    fn i2c_check_functionality(adapter: *mut i2c_adapter, func: c_uint) -> bool_t;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn i2c_match_id(id: *const i2c_device_id, client: *mut i2c_client) -> *const i2c_device_id;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
}

const aw88081_regmap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 8,
    max_register: unsafe { AW88081_REG_MAX },
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_BIG,
};

const aw88083_regmap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 8,
    max_register: unsafe { AW88083_REG_MAX },
    reg_format_endian: REGMAP_ENDIAN_LITTLE,
    val_format_endian: REGMAP_ENDIAN_BIG,
};

unsafe fn aw88081_dev_get_iis_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88081_SYSST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & AW88081_BIT_PLL_CHECK) != AW88081_BIT_PLL_CHECK {
        dev_err((*aw_dev).dev, c"check pll lock fail,reg_val:0x%04x".as_ptr(), reg_val);
        return -EINVAL;
    }
    0
}

unsafe fn aw88081_dev_check_mode1_pll(aw_dev: *mut aw_device) -> c_int {
    for _i in 0..AW88081_DEV_SYSST_CHECK_MAX {
        let ret = aw88081_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode1 iis signal check error".as_ptr());
            usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
        } else { return 0; }
    }
    -EPERM
}

unsafe fn aw88081_dev_check_mode2_pll(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88081_PLLCTRL1_REG, &mut reg_val);
    if ret != 0 { return ret; }
    reg_val &= !AW88081_CCO_MUX_MASK;
    if reg_val == AW88081_CCO_MUX_DIVIDED_VALUE {
        dev_dbg((*aw_dev).dev, c"CCO_MUX is already divider".as_ptr());
        return -EPERM;
    }
    /* change mode2 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88081_PLLCTRL1_REG, !AW88081_CCO_MUX_MASK, AW88081_CCO_MUX_DIVIDED_VALUE);
    if ret != 0 { return ret; }
    for _i in 0..AW88081_DEV_SYSST_CHECK_MAX {
        ret = aw88081_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 iis check error".as_ptr());
            usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
        } else { break; }
    }
    /* change mode1 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88081_PLLCTRL1_REG, !AW88081_CCO_MUX_MASK, AW88081_CCO_MUX_BYPASS_VALUE);
    if ret == 0 {
        usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
        for _i in 0..AW88081_DEV_SYSST_CHECK_MAX {
            ret = aw88081_dev_check_mode1_pll(aw_dev);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"mode2 switch to mode1, iis check error".as_ptr());
                usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
            } else { break; }
        }
    }
    ret
}

unsafe fn aw88081_dev_check_syspll(aw_dev: *mut aw_device) -> c_int {
    let mut ret = aw88081_dev_check_mode1_pll(aw_dev);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"mode1 check iis failed try switch to mode2 check".as_ptr());
        ret = aw88081_dev_check_mode2_pll(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 check iis failed".as_ptr());
            return ret;
        }
    }
    0
}

unsafe fn aw88081_dev_check_sysst(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88081_PWMCTRL4_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val = if (reg_val & !AW88081_NOISE_GATE_EN_MASK) != 0 { AW88081_NO_SWS_SYSST_CHECK } else { AW88081_SWS_SYSST_CHECK };
    for _i in 0..AW88081_DEV_SYSST_CHECK_MAX {
        let ret = regmap_read((*aw_dev).regmap, AW88081_SYSST_REG, &mut reg_val);
        if ret != 0 { return ret; }
        let value = reg_val & !AW88081_BIT_SYSST_CHECK_MASK & check_val;
        if value != check_val {
            dev_err((*aw_dev).dev, c"check sysst fail, reg_val=0x%04x, check:0x%x".as_ptr(), reg_val, check_val);
            usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
        } else { return 0; }
    }
    -EPERM
}

unsafe fn aw88081_dev_i2s_tx_enable(aw_dev: *mut aw_device, flag: bool_t) {
    if flag {
        regmap_update_bits((*aw_dev).regmap, AW88081_I2SCTRL3_REG, !AW88081_I2STXEN_MASK, AW88081_I2STXEN_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_I2SCTRL3_REG, !AW88081_I2STXEN_MASK, AW88081_I2STXEN_DISABLE_VALUE);
    }
}

unsafe fn aw88081_dev_pwd(aw_dev: *mut aw_device, pwd: bool_t) {
    if pwd {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_PWDN_MASK, AW88081_PWDN_POWER_DOWN_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_PWDN_MASK, AW88081_PWDN_WORKING_VALUE);
    }
}

unsafe fn aw88081_dev_amppd(aw_dev: *mut aw_device, amppd: bool_t) {
    if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_EN_PA_MASK, AW88081_EN_PA_POWER_DOWN_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_EN_PA_MASK, AW88081_EN_PA_WORKING_VALUE);
    }
}

unsafe fn aw88083_i2c_wen(aw88081: *mut aw88081, flag: bool_t) {
    let aw_dev = (*aw88081).aw_pa;
    if (*aw88081).devtype as c_int != aw8808x_type::AW88083 as c_int { return; }
    if flag {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_I2C_WEN_MASK, AW88083_I2C_WEN_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_I2C_WEN_MASK, AW88083_I2C_WEN_DISABLE_VALUE);
    }
}

unsafe fn aw88083_dev_amppd(aw_dev: *mut aw_device, amppd: bool_t) {
    if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_AMPPD_MASK, AW88083_AMPPD_POWER_DOWN_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_AMPPD_MASK, AW88083_AMPPD_WORKING_VALUE);
    }
}

unsafe fn aw88083_dev_pllpd(aw_dev: *mut aw_device, pllpd: bool_t) {
    if pllpd {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_PLL_PD_MASK, AW88083_PLL_PD_WORKING_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88083_PLL_PD_MASK, AW88083_PLL_PD_POWER_DOWN_VALUE);
    }
}

unsafe fn aw88081_dev_clear_int_status(aw_dev: *mut aw_device) {
    let mut int_status: c_uint = 0;
    /* read int status and clear */
    regmap_read((*aw_dev).regmap, AW88081_SYSINT_REG, &mut int_status);
    /* make sure int status is clear */
    regmap_read((*aw_dev).regmap, AW88081_SYSINT_REG, &mut int_status);
    dev_dbg((*aw_dev).dev, c"read interrupt reg = 0x%04x".as_ptr(), int_status);
}

unsafe fn aw88081_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) {
    let vol_desc = &mut (*aw_dev).volume_desc;
    let volume = core::cmp::min(value + vol_desc.init_volume, AW88081_MUTE_VOL);
    regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL2_REG, !AW88081_VOL_MASK, volume);
}

unsafe fn aw88081_dev_fade_in(aw_dev: *mut aw_device) {
    let desc = &mut (*aw_dev).volume_desc;
    let fade_in_vol = desc.ctl_volume;
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_in_time == 0 {
        aw88081_dev_set_volume(aw_dev, fade_in_vol as c_uint);
        return;
    }
    let mut i = AW88081_MUTE_VOL as c_int;
    while i >= fade_in_vol {
        aw88081_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_in_time as c_uint, ((*aw_dev).fade_in_time + 10) as c_uint);
        i -= fade_step;
    }
    if i != fade_in_vol { aw88081_dev_set_volume(aw_dev, fade_in_vol as c_uint); }
}

unsafe fn aw88081_dev_fade_out(aw_dev: *mut aw_device) {
    let desc = &mut (*aw_dev).volume_desc;
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_out_time == 0 {
        aw88081_dev_set_volume(aw_dev, AW88081_MUTE_VOL);
        return;
    }
    let mut i = desc.ctl_volume;
    while i <= AW88081_MUTE_VOL as c_int {
        aw88081_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_out_time as c_uint, ((*aw_dev).fade_out_time + 10) as c_uint);
        i += fade_step;
    }
    if i != AW88081_MUTE_VOL as c_int { aw88081_dev_set_volume(aw_dev, AW88081_MUTE_VOL); }
}

unsafe fn aw88081_dev_mute(aw_dev: *mut aw_device, is_mute: bool_t) {
    if is_mute {
        aw88081_dev_fade_out(aw_dev);
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_HMUTE_MASK, AW88081_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_HMUTE_MASK, AW88081_HMUTE_DISABLE_VALUE);
        aw88081_dev_fade_in(aw_dev);
    }
}

unsafe fn aw88081_dev_uls_hmute(aw_dev: *mut aw_device, uls_hmute: bool_t) {
    if uls_hmute {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_ULS_HMUTE_MASK, AW88081_ULS_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88081_SYSCTRL_REG, !AW88081_ULS_HMUTE_MASK, AW88081_ULS_HMUTE_DISABLE_VALUE);
    }
}

unsafe fn aw88081_dev_reg_value_check(aw_dev: *mut aw_device, reg_addr: u8, reg_val: *mut u16) -> c_int {
    if reg_addr as c_uint == AW88081_SYSCTRL_REG {
        *reg_val &= !((!AW88081_EN_PA_MASK | !AW88081_PWDN_MASK | !AW88081_HMUTE_MASK | !AW88081_ULS_HMUTE_MASK) as u16);
        *reg_val |= (AW88081_EN_PA_POWER_DOWN_VALUE | AW88081_PWDN_POWER_DOWN_VALUE | AW88081_HMUTE_ENABLE_VALUE | AW88081_ULS_HMUTE_ENABLE_VALUE) as u16;
    }
    if reg_addr as c_uint == AW88081_SYSCTRL2_REG {
        let read_vol = ((*reg_val as c_uint & !AW88081_VOL_MASK) >> AW88081_VOL_START_BIT) as c_uint;
        (*aw_dev).volume_desc.init_volume = read_vol;
    }
    /* i2stxen */
    if reg_addr as c_uint == AW88081_I2SCTRL3_REG {
        /* close tx */
        *reg_val &= AW88081_I2STXEN_MASK as u16;
        *reg_val |= AW88081_I2STXEN_DISABLE_VALUE as u16;
    }
    0
}

unsafe fn aw88083_dev_reg_value_check(aw_dev: *mut aw_device, reg_addr: u8, reg_val: *mut u16) -> c_int {
    if reg_addr as c_uint == AW88081_SYSCTRL_REG {
        *reg_val &= !((!AW88083_AMPPD_MASK | !AW88081_PWDN_MASK | !AW88081_HMUTE_MASK | !AW88083_I2C_WEN_MASK) as u16);
        *reg_val |= (AW88083_AMPPD_POWER_DOWN_VALUE | AW88081_PWDN_POWER_DOWN_VALUE | AW88081_HMUTE_ENABLE_VALUE | AW88083_I2C_WEN_ENABLE_VALUE) as u16;
    }
    if reg_addr as c_uint == AW88081_SYSCTRL2_REG {
        let read_vol = ((*reg_val as c_uint & !AW88081_VOL_MASK) >> AW88081_VOL_START_BIT) as c_uint;
        (*aw_dev).volume_desc.init_volume = read_vol;
    }
    0
}

unsafe fn aw88081_reg_value_check(aw88081: *mut aw88081, reg_addr: u8, reg_val: *mut u16) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    match (*aw88081).devtype {
        aw8808x_type::AW88081 => aw88081_dev_reg_value_check(aw_dev, reg_addr, reg_val),
        aw8808x_type::AW88083 => aw88083_dev_reg_value_check(aw_dev, reg_addr, reg_val),
    }
}

unsafe fn aw88081_dev_reg_update(aw88081: *mut aw88081, data: *mut u8, len: c_uint) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    let vol_desc = &mut (*aw_dev).volume_desc;
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"reg data is null or len is 0".as_ptr());
        return -EINVAL;
    }
    let reg_data = data as *mut int16_t;
    let data_len = (len >> 1) as c_int;
    if (data_len & 0x1) != 0 {
        dev_err((*aw_dev).dev, c"data len:%d unsupported".as_ptr(), data_len);
        return -EINVAL;
    }
    let mut i = 0;
    while i < data_len {
        let reg_addr = *reg_data.offset(i as isize) as u8;
        let mut reg_val = *reg_data.offset((i + 1) as isize) as u16;
        let ret = aw88081_reg_value_check(aw88081, reg_addr, &mut reg_val);
        if ret != 0 { return ret; }
        let ret = regmap_write((*aw_dev).regmap, reg_addr as c_uint, reg_val as c_uint);
        if ret != 0 { return ret; }
        i += 2;
    }
    if (*aw_dev).prof_cur != (*aw_dev).prof_index { vol_desc.ctl_volume = 0; }
    /* keep min volume */
    aw88081_dev_set_volume(aw_dev, vol_desc.mute_volume);
    0
}

unsafe fn aw88081_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut *mut c_char) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info;
    if index >= prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"index[%d] overflow count[%d]".as_ptr(), index, prof_info.count);
        return -EINVAL;
    }
    let prof_desc = prof_info.prof_desc.offset(index as isize);
    *prof_name = *prof_info.prof_name_list.offset((*prof_desc).id as isize);
    0
}

unsafe fn aw88081_dev_get_prof_data(aw_dev: *mut aw_device, index: c_int, prof_desc: *mut *mut aw_prof_desc) -> c_int {
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"%s: index[%d] overflow count[%d]\n".as_ptr(), c"aw88081_dev_get_prof_data".as_ptr(), index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    *prof_desc = (*aw_dev).prof_info.prof_desc.offset(index as isize);
    0
}

unsafe fn aw88081_dev_fw_update(aw88081: *mut aw88081) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    let mut prof_index_desc: *mut aw_prof_desc = ptr::null_mut();
    let mut prof_name: *mut c_char = ptr::null_mut();
    let mut ret = aw88081_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"get prof name failed".as_ptr());
        return -EINVAL;
    }
    dev_dbg((*aw_dev).dev, c"start update %s".as_ptr(), prof_name);
    ret = aw88081_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc);
    if ret != 0 { return ret; }
    /* update reg */
    let sec_desc = (*prof_index_desc).sec_desc;
    ret = aw88081_dev_reg_update(aw88081, (*sec_desc.add(AW88395_DATA_TYPE_REG)).data, (*sec_desc.add(AW88395_DATA_TYPE_REG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"update reg failed".as_ptr());
        return ret;
    }
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    0
}

unsafe fn aw88081_dev_start(aw88081: *mut aw88081) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    if (*aw_dev).status == AW88081_DEV_PW_ON {
        dev_dbg((*aw_dev).dev, c"already power on".as_ptr());
        return 0;
    }
    /* power on */
    aw88081_dev_pwd(aw_dev, false);
    usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
    let mut ret = aw88081_dev_check_syspll(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"pll check failed cannot start".as_ptr());
        aw88081_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88081_DEV_PW_OFF;
        return ret;
    }
    /* amppd on */
    aw88081_dev_amppd(aw_dev, false);
    usleep_range(AW88081_1000_US, AW88081_1000_US + 50);
    /* check i2s status */
    ret = aw88081_dev_check_sysst(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"sysst check failed".as_ptr());
        aw88081_dev_i2s_tx_enable(aw_dev, false);
        aw88081_dev_clear_int_status(aw_dev);
        aw88081_dev_amppd(aw_dev, true);
        aw88081_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88081_DEV_PW_OFF;
        return ret;
    }
    /* enable tx feedback */
    aw88081_dev_i2s_tx_enable(aw_dev, true);
    /* close uls mute */
    aw88081_dev_uls_hmute(aw_dev, false);
    /* close mute */
    aw88081_dev_mute(aw_dev, false);
    /* clear inturrupt */
    aw88081_dev_clear_int_status(aw_dev);
    (*aw_dev).status = AW88081_DEV_PW_ON;
    0
}

unsafe fn aw88083_dev_start(aw88081: *mut aw88081) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    if (*aw_dev).status == AW88081_DEV_PW_ON {
        dev_dbg((*aw_dev).dev, c"already power on".as_ptr());
        return 0;
    }
    aw88083_i2c_wen(aw88081, true);
    /* power on */
    aw88081_dev_pwd(aw_dev, false);
    usleep_range(AW88081_2000_US, AW88081_2000_US + 10);
    aw88083_dev_pllpd(aw_dev, true);
    /* amppd on */
    aw88083_dev_amppd(aw_dev, false);
    usleep_range(AW88081_2000_US, AW88081_2000_US + 50);
    /* close mute */
    aw88081_dev_mute(aw_dev, false);
    aw88083_i2c_wen(aw88081, false);
    (*aw_dev).status = AW88081_DEV_PW_ON;
    0
}

unsafe fn aw88081_device_start(aw88081: *mut aw88081) -> c_int {
    match (*aw88081).devtype {
        aw8808x_type::AW88081 => aw88081_dev_start(aw88081),
        aw8808x_type::AW88083 => aw88083_dev_start(aw88081),
    }
}

unsafe fn aw88081_dev_stop(aw88081: *mut aw88081) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    if (*aw_dev).status == AW88081_DEV_PW_OFF {
        dev_dbg((*aw_dev).dev, c"already power off".as_ptr());
        return 0;
    }
    (*aw_dev).status = AW88081_DEV_PW_OFF;
    /* clear inturrupt */
    aw88081_dev_clear_int_status(aw_dev);
    aw88081_dev_uls_hmute(aw_dev, true);
    /* set mute */
    aw88081_dev_mute(aw_dev, true);
    /* close tx feedback */
    aw88081_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88081_1000_US, AW88081_1000_US + 100);
    /* enable amppd */
    aw88081_dev_amppd(aw_dev, true);
    /* set power down */
    aw88081_dev_pwd(aw_dev, true);
    0
}

unsafe fn aw88083_dev_stop(aw88081: *mut aw88081) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    if (*aw_dev).status == AW88081_DEV_PW_OFF {
        dev_dbg((*aw_dev).dev, c"already power off".as_ptr());
        return 0;
    }
    (*aw_dev).status = AW88081_DEV_PW_OFF;
    aw88083_i2c_wen(aw88081, true);
    /* set mute */
    aw88081_dev_mute(aw_dev, true);
    usleep_range(AW88081_2000_US, AW88081_2000_US + 100);
    /* enable amppd */
    aw88083_dev_amppd(aw_dev, true);
    aw88083_dev_pllpd(aw_dev, false);
    /* set power down */
    aw88081_dev_pwd(aw_dev, true);
    aw88083_i2c_wen(aw88081, false);
    0
}

unsafe fn aw88081_stop(aw88081: *mut aw88081) -> c_int {
    match (*aw88081).devtype {
        aw8808x_type::AW88081 => aw88081_dev_stop(aw88081),
        aw8808x_type::AW88083 => aw88083_dev_stop(aw88081),
    }
}

unsafe fn aw88081_reg_update(aw88081: *mut aw88081, force: bool_t) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    if force {
        let mut ret = regmap_write((*aw_dev).regmap, AW88081_ID_REG, AW88081_SOFT_RESET_VALUE);
        if ret != 0 { return ret; }
        ret = aw88081_dev_fw_update(aw88081);
        if ret != 0 { return ret; }
    } else if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        let ret = aw88081_dev_fw_update(aw88081);
        if ret != 0 { return ret; }
    }
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    0
}

unsafe fn aw88081_start_pa(aw88081: *mut aw88081) {
    for i in 0..AW88081_START_RETRIES {
        let mut ret = aw88081_reg_update(aw88081, (*aw88081).phase_sync);
        if ret != 0 {
            dev_err((*(*aw88081).aw_pa).dev, c"fw update failed, cnt:%d\n".as_ptr(), i);
            continue;
        }
        ret = aw88081_device_start(aw88081);
        if ret != 0 {
            dev_err((*(*aw88081).aw_pa).dev, c"aw88081 device start failed. retry = %d".as_ptr(), i);
            continue;
        } else {
            dev_dbg((*(*aw88081).aw_pa).dev, c"start success\n".as_ptr());
            break;
        }
    }
}

unsafe extern "C" fn aw88081_startup_work(work: *mut work_struct) {
    let aw88081 = (work as *mut u8).sub(offset_of_start_work_work()) as *mut aw88081;
    mutex_lock(&mut (*aw88081).lock);
    aw88081_start_pa(aw88081);
    mutex_unlock(&mut (*aw88081).lock);
}

fn offset_of_start_work_work() -> usize {
    // Rust file-local equivalent of container_of(work, struct aw88081, start_work.work).
    0
}

unsafe fn aw88081_start(aw88081: *mut aw88081, sync_start: bool_t) {
    if (*(*aw88081).aw_pa).fw_status != AW88081_DEV_FW_OK { return; }
    if (*(*aw88081).aw_pa).status == AW88081_DEV_PW_ON { return; }
    if sync_start == AW88081_SYNC_START {
        aw88081_start_pa(aw88081);
    } else {
        queue_delayed_work(system_dfl_wq, &mut (*aw88081).start_work, AW88081_START_WORK_DELAY_MS);
    }
}

static mut aw88081_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"aw88081-aif".as_ptr(),
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: c"Speaker_Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88081_RATES },
        formats: unsafe { AW88081_FORMATS },
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Speaker_Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88081_RATES },
        formats: unsafe { AW88081_FORMATS },
    },
}];

unsafe extern "C" fn aw88081_get_fade_in_time(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(component);
    let aw_dev = (*aw88081).aw_pa;
    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_in_time as c_long;
    0
}

unsafe extern "C" fn aw88081_set_fade_in_time(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88081).aw_pa;
    let time = (*ucontrol).value.integer.value[0] as c_int;
    if time < (*mc).min || time > (*mc).max { return -EINVAL; }
    if time != (*aw_dev).fade_in_time {
        (*aw_dev).fade_in_time = time;
        return 1;
    }
    0
}

unsafe extern "C" fn aw88081_get_fade_out_time(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(component);
    let aw_dev = (*aw88081).aw_pa;
    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_out_time as c_long;
    0
}

unsafe extern "C" fn aw88081_set_fade_out_time(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88081).aw_pa;
    let time = (*ucontrol).value.integer.value[0] as c_int;
    if time < (*mc).min || time > (*mc).max { return -EINVAL; }
    if time != (*aw_dev).fade_out_time {
        (*aw_dev).fade_out_time = time;
        return 1;
    }
    0
}

unsafe fn aw88081_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int {
    /* check the index whether is valid */
    if index >= (*aw_dev).prof_info.count || index < 0 { return -EINVAL; }
    /* check the index whether change */
    if (*aw_dev).prof_index == index { return -EPERM; }
    (*aw_dev).prof_index = index;
    0
}

unsafe extern "C" fn aw88081_profile_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    let mut prof_name: *mut c_char = ptr::null_mut();
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;
    let mut count = (*(*aw88081).aw_pa).prof_info.count;
    if count <= 0 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }
    (*uinfo).value.enumerated.items = count as c_uint;
    if (*uinfo).value.enumerated.item >= count as c_uint {
        (*uinfo).value.enumerated.item = (count - 1) as c_uint;
    }
    count = (*uinfo).value.enumerated.item as c_int;
    let ret = aw88081_dev_get_prof_name((*aw88081).aw_pa, count, &mut prof_name);
    if ret != 0 {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), c"null".as_ptr());
        return 0;
    }
    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);
    0
}

unsafe extern "C" fn aw88081_profile_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    (*ucontrol).value.integer.value[0] = (*(*aw88081).aw_pa).prof_index as c_long;
    0
}

unsafe extern "C" fn aw88081_profile_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    /* pa stop or stopping just set profile */
    mutex_lock(&mut (*aw88081).lock);
    let ret = aw88081_dev_set_profile_index((*aw88081).aw_pa, (*ucontrol).value.integer.value[0] as c_int);
    if ret != 0 {
        dev_dbg((*codec).dev, c"profile index does not change".as_ptr());
        mutex_unlock(&mut (*aw88081).lock);
        return 0;
    }
    if (*(*aw88081).aw_pa).status != 0 {
        aw88081_stop(aw88081);
        aw88081_start(aw88081, AW88081_SYNC_START);
    }
    mutex_unlock(&mut (*aw88081).lock);
    1
}

unsafe extern "C" fn aw88081_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    let vol_desc = &mut (*(*aw88081).aw_pa).volume_desc;
    (*ucontrol).value.integer.value[0] = vol_desc.ctl_volume as c_long;
    0
}

unsafe extern "C" fn aw88081_volume_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    let vol_desc = &mut (*(*aw88081).aw_pa).volume_desc;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max { return -EINVAL; }
    aw88083_i2c_wen(aw88081, true);
    if vol_desc.ctl_volume != value {
        vol_desc.ctl_volume = value;
        aw88081_dev_set_volume((*aw88081).aw_pa, vol_desc.ctl_volume as c_uint);
        return 1;
    }
    aw88083_i2c_wen(aw88081, false);
    0
}

unsafe extern "C" fn aw88081_get_fade_step(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    (*ucontrol).value.integer.value[0] = (*(*aw88081).aw_pa).fade_step as c_long;
    0
}

unsafe extern "C" fn aw88081_set_fade_step(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88081 = snd_soc_component_get_drvdata(codec);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max { return -EINVAL; }
    if (*(*aw88081).aw_pa).fade_step != value {
        (*(*aw88081).aw_pa).fade_step = value;
        return 1;
    }
    0
}

// SOC_SINGLE_EXT/AW88081_PROFILE_EXT expand to snd_kcontrol_new initializers in C.
static aw88081_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe fn aw88081_parse_channel_dt(aw88081: *mut aw88081) {
    let aw_dev = (*aw88081).aw_pa;
    let np = (*(*aw_dev).dev).of_node;
    let mut channel_value: u32 = AW88081_DEV_DEFAULT_CH;
    of_property_read_u32(np, c"awinic,audio-channel".as_ptr(), &mut channel_value);
    (*aw88081).phase_sync = of_property_read_bool(np, c"awinic,sync-flag".as_ptr());
    (*aw_dev).channel = channel_value;
}

unsafe fn aw88081_init(aw88081: *mut aw88081, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int {
    let mut chip_id: c_uint = 0;
    /* read chip id */
    let ret = regmap_read(regmap, AW88081_ID_REG, &mut chip_id);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"%s read chipid error. ret = %d".as_ptr(), c"aw88081_init".as_ptr(), ret);
        return ret;
    }
    if chip_id == AW88081_CHIP_ID {
        dev_dbg(&mut (*i2c).dev, c"chip id = 0x%x\n".as_ptr(), chip_id);
    } else if chip_id == AW88083_CHIP_ID {
        dev_dbg(&mut (*i2c).dev, c"chip id = 0x%x\n".as_ptr(), chip_id);
    } else {
        dev_err(&mut (*i2c).dev, c"unsupported device".as_ptr());
        return -ENXIO;
    }
    let aw_dev = devm_kzalloc(&mut (*i2c).dev, size_of::<aw_device>(), GFP_KERNEL) as *mut aw_device;
    if aw_dev.is_null() { return -ENOMEM; }
    (*aw88081).aw_pa = aw_dev;
    (*aw_dev).i2c = i2c;
    (*aw_dev).regmap = regmap;
    (*aw_dev).dev = &mut (*i2c).dev;
    (*aw_dev).chip_id = chip_id;
    (*aw_dev).acf = ptr::null_mut();
    (*aw_dev).prof_info.prof_desc = ptr::null_mut();
    (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
    (*aw_dev).fade_step = AW88081_VOLUME_STEP_DB;
    (*aw_dev).volume_desc.mute_volume = AW88081_MUTE_VOL;
    aw88081_parse_channel_dt(aw88081);
    0
}

unsafe fn aw88081_dev_init(aw88081: *mut aw88081, aw_cfg: *mut aw_container) -> c_int {
    let aw_dev = (*aw88081).aw_pa;
    let mut ret = aw88395_dev_cfg_load(aw_dev, aw_cfg);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"aw_dev acf parse failed".as_ptr());
        return -EINVAL;
    }
    ret = regmap_write((*aw_dev).regmap, AW88081_ID_REG, AW88081_SOFT_RESET_VALUE);
    if ret != 0 { return ret; }
    (*aw_dev).fade_in_time = AW88081_500_US;
    (*aw_dev).fade_out_time = AW88081_500_US;
    (*aw_dev).prof_cur = AW88081_INIT_PROFILE;
    (*aw_dev).prof_index = AW88081_INIT_PROFILE;
    ret = aw88081_dev_fw_update(aw88081);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"fw update failed ret = %d\n".as_ptr(), ret);
        return ret;
    }
    (*aw_dev).status = AW88081_DEV_PW_ON;
    aw88081_stop(aw88081);
    0
}

unsafe fn aw88081_request_firmware_file(aw88081: *mut aw88081) -> c_int {
    let mut cont: *const firmware = ptr::null();
    (*(*aw88081).aw_pa).fw_status = AW88081_DEV_FW_FAILED;
    let ret = request_firmware(&mut cont, AW88081_ACF_FILE, (*(*aw88081).aw_pa).dev);
    if ret != 0 { return ret; }
    dev_dbg((*(*aw88081).aw_pa).dev, c"loaded %s - size: %zu\n".as_ptr(), AW88081_ACF_FILE, if !cont.is_null() { (*cont).size } else { 0 });
    let size = size_of::<aw_container>() + (*cont).size;
    let aw_cfg = devm_kzalloc((*(*aw88081).aw_pa).dev, size, GFP_KERNEL) as *mut aw_container;
    if aw_cfg.is_null() { return -ENOMEM; }
    (*aw_cfg).len = (*cont).size as c_int;
    memcpy((*aw_cfg).data.as_mut_ptr() as *mut c_void, (*cont).data as *const c_void, (*cont).size);
    (*aw88081).aw_cfg = aw_cfg;
    let ret = aw88395_dev_load_acf_check((*aw88081).aw_pa, (*aw88081).aw_cfg);
    if ret != 0 { return ret; }
    mutex_lock(&mut (*aw88081).lock);
    let ret = aw88081_dev_init(aw88081, (*aw88081).aw_cfg);
    mutex_unlock(&mut (*aw88081).lock);
    ret
}

unsafe extern "C" fn aw88081_playback_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aw88081 = snd_soc_component_get_drvdata(component);
    mutex_lock(&mut (*aw88081).lock);
    match event {
        SND_SOC_DAPM_PRE_PMU => aw88081_start(aw88081, AW88081_ASYNC_START),
        SND_SOC_DAPM_POST_PMD => { aw88081_stop(aw88081); },
        _ => {},
    }
    mutex_unlock(&mut (*aw88081).lock);
    0
}

// SND_SOC_DAPM_* macros expand to widget initializers in C.
static aw88081_dapm_widgets: [snd_soc_dapm_widget_def; 4] = [
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
];

static aw88081_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"DAC Output".as_ptr(), control: ptr::null(), source: c"AIF_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF_TX".as_ptr(), control: ptr::null(), source: c"ADC Input".as_ptr() },
];

unsafe extern "C" fn aw88081_codec_probe(component: *mut snd_soc_component) -> c_int {
    let aw88081 = snd_soc_component_get_drvdata(component);
    INIT_DELAYED_WORK(&mut (*aw88081).start_work, aw88081_startup_work);
    let ret = aw88081_request_firmware_file(aw88081);
    if ret != 0 {
        dev_err((*(*aw88081).aw_pa).dev, c"%s: request firmware failed\n".as_ptr(), c"aw88081_codec_probe".as_ptr());
    }
    ret
}

unsafe extern "C" fn aw88081_codec_remove(aw_codec: *mut snd_soc_component) {
    let aw88081 = snd_soc_component_get_drvdata(aw_codec);
    cancel_delayed_work_sync(&mut (*aw88081).start_work);
}

static soc_codec_dev_aw88081: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw88081_codec_probe),
    remove: Some(aw88081_codec_remove),
    dapm_widgets: aw88081_dapm_widgets.as_ptr(),
    num_dapm_widgets: aw88081_dapm_widgets.len(),
    dapm_routes: aw88081_audio_map.as_ptr(),
    num_dapm_routes: aw88081_audio_map.len(),
    controls: aw88081_controls.as_ptr(),
    num_controls: aw88081_controls.len(),
};

static aw88081_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: unsafe { AW88081_I2C_NAME }, driver_data: aw8808x_type::AW88081 },
    i2c_device_id { name: unsafe { AW88083_I2C_NAME }, driver_data: aw8808x_type::AW88083 },
    i2c_device_id { name: ptr::null(), driver_data: aw8808x_type::AW88081 },
];
// MODULE_DEVICE_TABLE(i2c, aw88081_i2c_id);

unsafe extern "C" fn aw88081_i2c_probe(i2c: *mut i2c_client) -> c_int {
    if !i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) {
        return dev_err_probe(&mut (*i2c).dev, -ENXIO, c"check_functionality failed".as_ptr());
    }
    let aw88081 = devm_kzalloc(&mut (*i2c).dev, size_of::<aw88081>(), GFP_KERNEL) as *mut aw88081;
    if aw88081.is_null() { return -ENOMEM; }
    let id = i2c_match_id(aw88081_i2c_id.as_ptr(), i2c);
    (*aw88081).devtype = (*id).driver_data;
    mutex_init(&mut (*aw88081).lock);
    i2c_set_clientdata(i2c, aw88081 as *mut c_void);
    let regmap_config = match (*aw88081).devtype {
        aw8808x_type::AW88081 => &aw88081_regmap_config as *const regmap_config,
        aw8808x_type::AW88083 => &aw88083_regmap_config as *const regmap_config,
    };
    (*aw88081).regmap = devm_regmap_init_i2c(i2c, regmap_config);
    if IS_ERR((*aw88081).regmap as *const c_void) {
        return dev_err_probe(&mut (*i2c).dev, PTR_ERR((*aw88081).regmap as *const c_void), c"failed to init regmap\n".as_ptr());
    }
    /* aw pa init */
    let ret = aw88081_init(aw88081, i2c, (*aw88081).regmap);
    if ret != 0 { return ret; }
    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_dev_aw88081, aw88081_dai.as_mut_ptr(), aw88081_dai.len())
}

// #if defined(CONFIG_OF)
static aw88081_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"awinic,aw88081".as_ptr() },
    of_device_id { compatible: c"awinic,aw88083".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, aw88081_of_match);
// #endif

static mut aw88081_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: unsafe { AW88081_I2C_NAME },
        of_match_table: aw88081_of_match.as_ptr(),
    },
    probe: Some(aw88081_i2c_probe),
    id_table: aw88081_i2c_id.as_ptr(),
};
// module_i2c_driver(aw88081_i2c_driver);
// MODULE_DESCRIPTION("ASoC AW88081 Smart PA Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
