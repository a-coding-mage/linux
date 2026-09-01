// SPDX-License-Identifier: GPL-2.0-only
//
// aw88166.rs -- ALSA SoC AW88166 codec support
//
// Copyright (c) 2025 AWINIC Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//
// Source-level Rust translation of aw88166.c.
// C include dependencies are expected to provide the referenced kernel, ALSA,
// regmap, I2C, GPIO, firmware, and AWINIC symbols.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};
use core::mem;
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type uint16_t = u16;
type int16_t = i16;
type uint32_t = u32;
type int32_t = i32;

#[repr(C)]
pub struct aw88166 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub start_work: delayed_work,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub check_val: c_uint,
    pub crc_init_val: c_uint,
    pub vcalb_init_val: c_uint,
    pub re_init_val: c_uint,
    pub dither_st: c_uint,
    pub phase_sync: bool,
}

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
}

#[repr(C)]
pub struct aw_device {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub dsp_lock: mutex,
    pub chip_id: c_uint,
    pub acf: *mut c_void,
    pub prof_info: aw_prof_info,
    pub channel: c_uint,
    pub fw_status: c_int,
    pub fade_step: c_int,
    pub fade_in_time: c_int,
    pub fade_out_time: c_int,
    pub volume_desc: aw_volume_desc,
    pub cali_desc: aw_cali_desc,
    pub status: c_int,
    pub dsp_cfg: c_int,
    pub dsp_fw_len: c_uint,
    pub dsp_cfg_len: c_uint,
    pub prof_cur: c_int,
    pub prof_index: c_int,
}

#[repr(C)]
pub struct aw_container {
    pub len: c_int,
    pub data: [c_uchar; 0],
}

#[repr(C)]
pub struct aw_volume_desc {
    pub init_volume: u16,
    pub ctl_volume: u16,
}

#[repr(C)]
pub struct aw_cali_desc {
    pub cali_re: c_int,
    pub ra: c_int,
}

#[repr(C)]
pub struct aw_prof_info {
    pub prof_desc: *mut aw_prof_desc,
    pub count: c_int,
    pub prof_type: c_int,
    pub prof_name_list: *mut *mut c_char,
}

#[repr(C)]
pub struct aw_prof_desc {
    pub id: c_int,
    pub fw_ver: c_uint,
    pub sec_desc: *mut aw_sec_data_desc,
}

#[repr(C)]
pub struct aw_sec_data_desc {
    pub data: *mut c_uchar,
    pub len: c_uint,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const c_uchar,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub enumerated: snd_ctl_elem_info_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_enumerated {
    pub items: c_uint,
    pub item: c_uint,
    pub name: [c_char; 64],
}

#[repr(C)]
pub struct soc_mixer_control {
    pub min: c_int,
    pub max: c_int,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
    pub adapter: *mut i2c_adapter,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct mutex {
    _priv: [u8; 0],
}

#[repr(C)] pub struct regmap { _priv: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _priv: [u8; 0] }
#[repr(C)] pub struct device_node { _priv: [u8; 0] }
#[repr(C)] pub struct i2c_adapter { _priv: [u8; 0] }

extern "C" {
    static mut system_dfl_wq: *mut c_void;

    static AW88166_REG_MAX: c_uint;
    static REGMAP_ENDIAN_LITTLE: c_uint;
    static REGMAP_ENDIAN_BIG: c_uint;
    static AW88166_SYSCTRL_REG: c_uint;
    static AW88166_PWDN_MASK: c_uint;
    static AW88166_PWDN_POWER_DOWN_VALUE: c_uint;
    static AW88166_PWDN_WORKING_VALUE: c_uint;
    static AW88166_SYSINT_REG: c_uint;
    static AW88166_SYSST_REG: c_uint;
    static AW88166_BIT_PLL_CHECK: c_uint;
    static AW88166_DEV_SYSST_CHECK_MAX: c_int;
    static AW88166_2000_US: c_uint;
    static AW88166_PLLCTRL2_REG: c_uint;
    static AW88166_CCO_MUX_MASK: c_uint;
    static AW88166_CCO_MUX_DIVIDED_VALUE: c_uint;
    static AW88166_CCO_MUX_BYPASS_VALUE: c_uint;
    static AW88166_PWMCTRL3_REG: c_uint;
    static AW88166_NOISE_GATE_EN_MASK: c_uint;
    static AW88166_BIT_SYSST_NOSWS_CHECK: c_uint;
    static AW88166_BIT_SYSST_SWS_CHECK: c_uint;
    static AW88166_BIT_SYSST_CHECK_MASK: c_uint;
    static AW88166_AMPPD_MASK: c_uint;
    static AW88166_AMPPD_POWER_DOWN_VALUE: c_uint;
    static AW88166_AMPPD_WORKING_VALUE: c_uint;
    static AW88166_DSPBY_MASK: c_uint;
    static AW88166_DSPBY_WORKING_VALUE: c_uint;
    static AW88166_DSPBY_BYPASS_VALUE: c_uint;
    static AW88166_EFRM2_REG: c_uint;
    static AW88166_EFRL_REG: c_uint;
    static AW88166_EF_ISN_GESLP_MASK: c_uint;
    static AW88166_EF_ISN_GESLP_SHIFT: c_uint;
    static AW88166_EF_ISN_H5BITS_MASK: c_uint;
    static AW88166_EF_ISN_H5BITS_SHIFT: c_uint;
    static AW88166_EF_ISN_H5BITS_SIGN_MASK: u16;
    static AW_EF_AND_CHECK: c_uint;
    static AW_EF_OR_CHECK: c_uint;
    static AW88166_ICALK_SIGN_MASK: u16;
    static AW88166_ICALK_NEG_MASK: u16;
    static AW88166_EF_VSN_GESLP_MASK: c_uint;
    static AW88166_EF_VSN_GESLP_SHIFT: c_uint;
    static AW88166_EF_VSN_H3BITS_MASK: c_uint;
    static AW88166_EF_VSN_H3BITS_SHIFT: c_uint;
    static AW88166_EF_VSN_H3BITS_SIGN_MASK: u16;
    static AW88166_VCALK_SIGN_MASK: u16;
    static AW88166_VCALK_NEG_MASK: u16;
    static AW88166_ICABLK_FACTOR: i32;
    static AW88166_VCABLK_FACTOR: i32;
    static AW88166_CABL_BASE_VALUE: i32;
    static AW88166_VCALB_ACCURACY: i32;
    static AW88166_VSCAL_FACTOR: i32;
    static AW88166_ISCAL_FACTOR: i32;
    static AW88166_VCALB_ADJ_FACTOR: c_uint;
    static AW88166_DSPVCALB_REG: c_uint;
    static AW88166_RECOVERY_SEC_DATA: c_int;
    static AW88166_RECORD_SEC_DATA: c_int;
    static AW88166_ACR1_REG: c_uint;
    static AW88166_ACR2_REG: c_uint;
    static AW88166_CALI_RE_MAX: c_int;
    static AW88166_CALI_RE_MIN: c_int;
    static AW88166_DSP_RE_SHIFT: c_uint;
    static AW88166_CALI_RE_HBITS_MASK: u32;
    static AW88166_CALI_RE_HBITS_SHIFT: c_uint;
    static AW88166_CALI_RE_LBITS_MASK: u32;
    static AW88166_CALI_RE_LBITS_SHIFT: c_uint;
    static AW_FW_ADDR_LEN: c_uint;
    static AW88166_CRC_FW_BASE_ADDR: u16;
    static AW88166_CRC_CFG_BASE_ADDR: u16;
    static AW88166_CRCCTRL_REG: c_uint;
    static AW88166_CRC_END_ADDR_MASK: c_uint;
    static AW88166_CRC_CODE_EN_MASK: c_uint;
    static AW88166_CRC_CODE_EN_ENABLE_VALUE: c_uint;
    static AW88166_CRC_CODE_EN_DISABLE_VALUE: c_uint;
    static AW88166_CRC_CFG_EN_MASK: c_uint;
    static AW88166_CRC_CFG_EN_ENABLE_VALUE: c_uint;
    static AW88166_CRC_CFG_EN_DISABLE_VALUE: c_uint;
    static AW88166_1000_US: c_uint;
    static AW88166_HAGCST_REG: c_uint;
    static AW88166_CRC_CHECK_BITS_MASK: c_uint;
    static AW88166_CRC_CHECK_START_BIT: c_uint;
    static AW88166_CRC_CHECK_PASS_VAL: u16;
    static AW88166_I2SCFG1_REG: c_uint;
    static AW88166_RAM_CG_BYP_MASK: c_uint;
    static AW88166_RAM_CG_BYP_BYPASS_VALUE: c_uint;
    static AW88166_RAM_CG_BYP_WORK_VALUE: c_uint;
    static AW88166_I2SCTRL3_REG: c_uint;
    static AW88166_I2STXEN_MASK: c_uint;
    static AW88166_I2STXEN_ENABLE_VALUE: c_uint;
    static AW88166_I2STXEN_DISABLE_VALUE: c_uint;
    static AW88166_WDT_REG: c_uint;
    static AW88166_WDT_CNT_MASK: c_uint;
    static AW88166_DEV_DSP_BYPASS: c_int;
    static AW88166_DEV_DSP_WORK: c_int;
    static AW88166_DEV_DSP_CHECK_MAX: c_int;
    static AW88166_MUTE_VOL: u16;
    static AW88166_SYSCTRL2_REG: c_uint;
    static AW88166_VOL_START_BIT: c_uint;
    static AW88166_VOL_MASK: u16;
    static AW88166_HMUTE_MASK: c_uint;
    static AW88166_HMUTE_ENABLE_VALUE: c_uint;
    static AW88166_HMUTE_DISABLE_VALUE: c_uint;
    static AW88166_DBGCTRL_REG: c_uint;
    static AW88166_DITHER_EN_MASK: c_uint;
    static AW88166_DITHER_EN_ENABLE_VALUE: c_uint;
    static AW88166_DITHER_EN_DISABLE_VALUE: c_uint;
    static AW88166_DEV_PW_ON: c_int;
    static AW88166_DEV_PW_OFF: c_int;
    static AW88166_MAX_RAM_WRITE_BYTE_SIZE: c_uint;
    static AW88166_DSPMADD_REG: c_uint;
    static AW88166_DSPMDAT_REG: c_uint;
    static AW88166_DSP_REG_CFG_ADPZ_RA: c_uint;
    static AW_DSP_32_DATA: c_uint;
    static AW_DSP_16_DATA: c_uint;
    static AW88166_DSP_CFG_ADDR: u16;
    static AW88166_DSP_FW_ADDR: u16;
    static AW88166_DSP_ROM_CHECK_ADDR: c_uint;
    static AW88166_DSP_ROM_CHECK_DATA: c_uint;
    static AW88166_DSP_ODD_NUM_BIT_TEST: c_uint;
    static AW88166_DEV_MEMCLK_PLL: c_uchar;
    static AW88166_DEV_MEMCLK_OSC: c_uchar;
    static AW88166_MEM_CLKSEL_MASK: c_uint;
    static AW88166_MEM_CLKSEL_DAPHCLK_VALUE: c_uint;
    static AW88166_MEM_CLKSEL_OSCCLK_VALUE: c_uint;
    static AW88166_EF_DBMD_MASK: c_uint;
    static AW88166_EF_DBMD_OR_VALUE: c_uint;
    static AW88395_DATA_TYPE_REG: isize;
    static AW88395_DATA_TYPE_DSP_FW: isize;
    static AW88395_DATA_TYPE_DSP_CFG: isize;
    static AW88166_FORCE_UPDATE_OFF: bool;
    static AW88166_FORCE_UPDATE_ON: bool;
    static AW88166_DEV_FW_FAILED: c_int;
    static AW88166_DEV_FW_OK: c_int;
    static AW88166_DSP_FW_UPDATE_ON: bool;
    static AW88166_DSP_FW_UPDATE_OFF: bool;
    static AW88166_START_RETRIES: c_int;
    static AW88166_SYNC_START: bool;
    static AW88166_ASYNC_START: bool;
    static AW88166_START_WORK_DELAY_MS: u64;
    static AW88166_BIT_SYSINT_CHECK: u16;
    static AW88166_4000_US: c_uint;
    static AW88166_RATES: c_uint;
    static AW88166_FORMATS: u64;
    static SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint;
    static FADE_TIME_MAX: c_int;
    static FADE_TIME_MIN: c_int;
    static AW88166_ACF_FILE: *const c_char;
    static GFP_KERNEL: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static AW88166_ID_REG: c_uint;
    static AW88395_DEV_NONE_TYPE_ID: c_int;
    static AW88166_DEV_DEFAULT_CH: c_uint;
    static AW88166_VOLUME_STEP_DB: c_int;
    static AW88166_VOL_DEFAULT_VALUE: u16;
    static I2C_FUNC_I2C: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static AW88166_I2C_NAME: *const c_char;
    static EINVAL: c_int;
    static EPERM: c_int;
    static ENOMEM: c_int;
    static ENXIO: c_int;

    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, len: c_uint) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn aw_dev_dsp_read(aw_dev: *mut aw_device, addr: c_uint, data: *mut c_uint, data_type: c_uint) -> c_int;
    fn aw_dev_dsp_write(aw_dev: *mut aw_device, addr: c_uint, data: c_uint, data_type: c_uint) -> c_int;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: u64) -> bool;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_check_functionality(adapter: *mut i2c_adapter, func: c_uint) -> bool;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

unsafe fn neg(err: c_int) -> c_int { -err }
unsafe fn min_u32(a: c_uint, b: c_uint) -> c_uint { if a < b { a } else { b } }
unsafe fn aw88166_from_cali_desc(cali_desc: *mut aw_cali_desc) -> *mut aw_device {
    (cali_desc as *mut u8).sub(mem::offset_of!(aw_device, cali_desc)) as *mut aw_device
}
unsafe fn aw88166_from_work(work: *mut work_struct) -> *mut aw88166 {
    let dw = (work as *mut u8).sub(mem::offset_of!(delayed_work, work)) as *mut delayed_work;
    (dw as *mut u8).sub(mem::offset_of!(aw88166, start_work)) as *mut aw88166
}

static aw88166_remap_config: regmap_config = regmap_config {
    val_bits: 16,
    reg_bits: 8,
    max_register: unsafe { AW88166_REG_MAX },
    reg_format_endian: unsafe { REGMAP_ENDIAN_LITTLE },
    val_format_endian: unsafe { REGMAP_ENDIAN_BIG },
};

unsafe extern "C" fn aw_dev_pwd(aw_dev: *mut aw_device, pwd: bool) {
    let ret: c_int;
    if pwd {
        ret = regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_PWDN_MASK, AW88166_PWDN_POWER_DOWN_VALUE);
    } else {
        ret = regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_PWDN_MASK, AW88166_PWDN_WORKING_VALUE);
    }
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_pwd".as_ptr());
    }
}

unsafe extern "C" fn aw_dev_get_int_status(aw_dev: *mut aw_device, int_status: *mut u16) {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88166_SYSINT_REG, &mut reg_val);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read interrupt reg fail, ret=%d".as_ptr(), ret);
    } else {
        *int_status = reg_val as u16;
    }
    dev_dbg((*aw_dev).dev, c"read interrupt reg=0x%04x".as_ptr(), *int_status as c_int);
}

unsafe extern "C" fn aw_dev_clear_int_status(aw_dev: *mut aw_device) {
    let mut int_status: u16 = 0;
    /* read int status and clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    /* make sure int status is clear */
    aw_dev_get_int_status(aw_dev, &mut int_status);
    if int_status != 0 {
        dev_dbg((*aw_dev).dev, c"int status(%d) is not cleaned.\n".as_ptr(), int_status as c_int);
    }
}

unsafe extern "C" fn aw_dev_get_iis_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88166_SYSST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & AW88166_BIT_PLL_CHECK) != AW88166_BIT_PLL_CHECK {
        dev_err((*aw_dev).dev, c"check pll lock fail, reg_val:0x%04x".as_ptr(), reg_val);
        return neg(EINVAL);
    }
    0
}

unsafe extern "C" fn aw_dev_check_mode1_pll(aw_dev: *mut aw_device) -> c_int {
    let mut i = 0;
    while i < AW88166_DEV_SYSST_CHECK_MAX {
        let ret = aw_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode1 iis signal check error".as_ptr());
            usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    neg(EPERM)
}

unsafe extern "C" fn aw_dev_check_mode2_pll(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88166_PLLCTRL2_REG, &mut reg_val);
    if ret != 0 { return ret; }
    reg_val &= !AW88166_CCO_MUX_MASK;
    if reg_val == AW88166_CCO_MUX_DIVIDED_VALUE {
        dev_dbg((*aw_dev).dev, c"CCO_MUX is already divider".as_ptr());
        return neg(EPERM);
    }
    /* change mode2 */
    ret = regmap_update_bits((*aw_dev).regmap, AW88166_PLLCTRL2_REG,
        !AW88166_CCO_MUX_MASK, AW88166_CCO_MUX_DIVIDED_VALUE);
    if ret != 0 { return ret; }
    let mut i = 0;
    while i < AW88166_DEV_SYSST_CHECK_MAX {
        ret = aw_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 iis signal check error".as_ptr());
            usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
        } else {
            break;
        }
        i += 1;
    }
    /* change mode1 */
    regmap_update_bits((*aw_dev).regmap, AW88166_PLLCTRL2_REG,
        !AW88166_CCO_MUX_MASK, AW88166_CCO_MUX_BYPASS_VALUE);
    if ret == 0 {
        usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
        i = 0;
        while i < AW88166_DEV_SYSST_CHECK_MAX {
            ret = aw_dev_get_iis_status(aw_dev);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"mode2 switch to mode1, iis signal check error".as_ptr());
                usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
            } else {
                break;
            }
            i += 1;
        }
    }
    ret
}

unsafe extern "C" fn aw_dev_check_syspll(aw_dev: *mut aw_device) -> c_int {
    let mut ret = aw_dev_check_mode1_pll(aw_dev);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"mode1 check iis failed try switch to mode2 check".as_ptr());
        ret = aw_dev_check_mode2_pll(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"mode2 check iis failed".as_ptr());
            return ret;
        }
    }
    0
}

unsafe extern "C" fn aw_dev_check_sysst(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88166_PWMCTRL3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val = if (reg_val & !AW88166_NOISE_GATE_EN_MASK) != 0 {
        AW88166_BIT_SYSST_NOSWS_CHECK
    } else {
        AW88166_BIT_SYSST_SWS_CHECK
    };
    let mut i = 0;
    while i < AW88166_DEV_SYSST_CHECK_MAX {
        let ret2 = regmap_read((*aw_dev).regmap, AW88166_SYSST_REG, &mut reg_val);
        if ret2 != 0 { return ret2; }
        if (reg_val & !AW88166_BIT_SYSST_CHECK_MASK & check_val) != check_val {
            dev_err((*aw_dev).dev, c"check sysst fail, cnt=%d, reg_val=0x%04x, check:0x%x".as_ptr(),
                i, reg_val, AW88166_BIT_SYSST_NOSWS_CHECK);
            usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    neg(EPERM)
}

unsafe extern "C" fn aw_dev_amppd(aw_dev: *mut aw_device, amppd: bool) {
    let ret = if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_AMPPD_MASK, AW88166_AMPPD_POWER_DOWN_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_AMPPD_MASK, AW88166_AMPPD_WORKING_VALUE)
    };
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_amppd".as_ptr());
    }
}

unsafe extern "C" fn aw_dev_dsp_enable(aw_dev: *mut aw_device, is_enable: bool) {
    let ret = if is_enable {
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_DSPBY_MASK, AW88166_DSPBY_WORKING_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_DSPBY_MASK, AW88166_DSPBY_BYPASS_VALUE)
    };
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed\n".as_ptr(), c"aw_dev_dsp_enable".as_ptr());
    }
}

unsafe extern "C" fn aw88166_dev_get_icalk(aw88166: *mut aw88166, icalk: *mut int16_t) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut efrm_reg_val: c_uint = 0;
    let mut efrl_reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88166_EFRM2_REG, &mut efrm_reg_val);
    if ret != 0 { return ret; }
    let ef_isn_geslp = ((efrm_reg_val & !AW88166_EF_ISN_GESLP_MASK) >> AW88166_EF_ISN_GESLP_SHIFT) as u16;
    ret = regmap_read((*aw_dev).regmap, AW88166_EFRL_REG, &mut efrl_reg_val);
    if ret != 0 { return ret; }
    let ef_isn_h5bits = ((efrl_reg_val & !AW88166_EF_ISN_H5BITS_MASK) >> AW88166_EF_ISN_H5BITS_SHIFT) as u16;
    let mut icalk_val = if (*aw88166).check_val == AW_EF_AND_CHECK {
        ef_isn_geslp & (ef_isn_h5bits | AW88166_EF_ISN_H5BITS_SIGN_MASK)
    } else {
        ef_isn_geslp | (ef_isn_h5bits & !AW88166_EF_ISN_H5BITS_SIGN_MASK)
    };
    if (icalk_val & !AW88166_ICALK_SIGN_MASK) != 0 {
        icalk_val |= AW88166_ICALK_NEG_MASK;
    }
    *icalk = icalk_val as int16_t;
    0
}

unsafe extern "C" fn aw88166_dev_get_vcalk(aw88166: *mut aw88166, vcalk: *mut int16_t) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut efrm_reg_val: c_uint = 0;
    let mut efrl_reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88166_EFRM2_REG, &mut efrm_reg_val);
    if ret != 0 { return ret; }
    let ef_vsn_geslp = ((efrm_reg_val & !AW88166_EF_VSN_GESLP_MASK) >> AW88166_EF_VSN_GESLP_SHIFT) as u16;
    ret = regmap_read((*aw_dev).regmap, AW88166_EFRL_REG, &mut efrl_reg_val);
    if ret != 0 { return ret; }
    let ef_vsn_h3bits = ((efrl_reg_val & !AW88166_EF_VSN_H3BITS_MASK) >> AW88166_EF_VSN_H3BITS_SHIFT) as u16;
    let mut vcalk_val = if (*aw88166).check_val == AW_EF_AND_CHECK {
        ef_vsn_geslp & (ef_vsn_h3bits | AW88166_EF_VSN_H3BITS_SIGN_MASK)
    } else {
        ef_vsn_geslp | (ef_vsn_h3bits & !AW88166_EF_VSN_H3BITS_SIGN_MASK)
    };
    if (vcalk_val & !AW88166_VCALK_SIGN_MASK) != 0 {
        vcalk_val |= AW88166_VCALK_NEG_MASK;
    }
    *vcalk = vcalk_val as int16_t;
    0
}

unsafe extern "C" fn aw88166_dev_set_vcalb(aw88166: *mut aw88166) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut icalk: int16_t = 0;
    let mut vcalk: int16_t = 0;
    let mut ret = aw88166_dev_get_icalk(aw88166, &mut icalk);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"get icalk failed\n".as_ptr());
        return ret;
    }
    let ical_k: i32 = (icalk as i32) * AW88166_ICABLK_FACTOR + AW88166_CABL_BASE_VALUE;
    ret = aw88166_dev_get_vcalk(aw88166, &mut vcalk);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"get vbcalk failed\n".as_ptr());
        return ret;
    }
    let vcal_k: i32 = (vcalk as i32) * AW88166_VCABLK_FACTOR + AW88166_CABL_BASE_VALUE;
    let mut vcalb: i32 = AW88166_VCALB_ACCURACY * AW88166_VSCAL_FACTOR /
        AW88166_ISCAL_FACTOR * ical_k / vcal_k * ((*aw88166).vcalb_init_val as i32);
    vcalb >>= AW88166_VCALB_ADJ_FACTOR;
    regmap_write((*aw_dev).regmap, AW88166_DSPVCALB_REG, vcalb as u32);
    0
}

unsafe extern "C" fn aw_dev_init_vcalb_update(aw88166: *mut aw88166, flag: c_int) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    if flag == AW88166_RECOVERY_SEC_DATA {
        regmap_write((*aw_dev).regmap, AW88166_DSPVCALB_REG, (*aw88166).vcalb_init_val)
    } else if flag == AW88166_RECORD_SEC_DATA {
        regmap_read((*aw_dev).regmap, AW88166_DSPVCALB_REG, &mut (*aw88166).vcalb_init_val)
    } else {
        dev_err((*aw_dev).dev, c"unsupported type:%d\n".as_ptr(), flag);
        neg(EINVAL)
    }
}

unsafe extern "C" fn aw_dev_init_re_update(aw88166: *mut aw88166, flag: c_int) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut re_temp_h: c_uint = 0;
    let mut re_temp_l: c_uint = 0;
    if flag == AW88166_RECOVERY_SEC_DATA {
        let mut ret = regmap_write((*aw_dev).regmap, AW88166_ACR1_REG, (*aw88166).re_init_val >> 16);
        if ret != 0 { return ret; }
        ret = regmap_write((*aw_dev).regmap, AW88166_ACR2_REG, (*aw88166).re_init_val as u16 as c_uint);
        if ret != 0 { return ret; }
        ret
    } else if flag == AW88166_RECORD_SEC_DATA {
        let mut ret = regmap_read((*aw_dev).regmap, AW88166_ACR1_REG, &mut re_temp_h);
        if ret != 0 { return ret; }
        ret = regmap_read((*aw_dev).regmap, AW88166_ACR2_REG, &mut re_temp_l);
        if ret != 0 { return ret; }
        (*aw88166).re_init_val = (re_temp_h << 16).wrapping_add(re_temp_l);
        ret
    } else {
        dev_err((*aw_dev).dev, c"unsupported type:%d\n".as_ptr(), flag);
        neg(EINVAL)
    }
}

unsafe extern "C" fn aw_dev_backup_sec_record(aw88166: *mut aw88166) {
    aw_dev_init_vcalb_update(aw88166, AW88166_RECORD_SEC_DATA);
    aw_dev_init_re_update(aw88166, AW88166_RECOVERY_SEC_DATA);
}

unsafe extern "C" fn aw_dev_backup_sec_recovery(aw88166: *mut aw88166) {
    aw_dev_init_vcalb_update(aw88166, AW88166_RECOVERY_SEC_DATA);
    aw_dev_init_re_update(aw88166, AW88166_RECOVERY_SEC_DATA);
}

unsafe fn AW88166_SHOW_RE_TO_DSP_RE(v: c_int, shift: c_uint) -> u32 { ((v as u32) << shift) }
unsafe fn AW88166_DSP_RE_TO_SHOW_RE(v: u32, shift: c_uint) -> c_int { (v >> shift) as c_int }

unsafe extern "C" fn aw_dev_update_cali_re(cali_desc: *mut aw_cali_desc) -> c_int {
    let aw_dev = aw88166_from_cali_desc(cali_desc);
    if (*aw_dev).cali_desc.cali_re >= AW88166_CALI_RE_MAX ||
        (*aw_dev).cali_desc.cali_re <= AW88166_CALI_RE_MIN {
        return neg(EINVAL);
    }
    let cali_re = AW88166_SHOW_RE_TO_DSP_RE((*aw_dev).cali_desc.cali_re + (*aw_dev).cali_desc.ra,
        AW88166_DSP_RE_SHIFT);
    let re_hbits = ((cali_re & !AW88166_CALI_RE_HBITS_MASK) >> AW88166_CALI_RE_HBITS_SHIFT) as u16;
    let re_lbits = ((cali_re & !AW88166_CALI_RE_LBITS_MASK) >> AW88166_CALI_RE_LBITS_SHIFT) as u16;
    let mut ret = regmap_write((*aw_dev).regmap, AW88166_ACR1_REG, re_hbits as c_uint);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"set cali re error".as_ptr());
        return ret;
    }
    ret = regmap_write((*aw_dev).regmap, AW88166_ACR2_REG, re_lbits as c_uint);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"set cali re error".as_ptr());
    }
    ret
}

unsafe extern "C" fn aw_dev_fw_crc_check(aw_dev: *mut aw_device) -> c_int {
    let fw_len_val: u16 = (((*aw_dev).dsp_fw_len / AW_FW_ADDR_LEN) - 1).wrapping_add(AW88166_CRC_FW_BASE_ADDR as c_uint) as u16;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_END_ADDR_MASK, fw_len_val as c_uint);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_CODE_EN_MASK, AW88166_CRC_CODE_EN_ENABLE_VALUE);
    usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
    let mut reg_val: c_uint = 0;
    regmap_read((*aw_dev).regmap, AW88166_HAGCST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val = ((reg_val & !AW88166_CRC_CHECK_BITS_MASK) >> AW88166_CRC_CHECK_START_BIT) as u16;
    ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_CODE_EN_MASK, AW88166_CRC_CODE_EN_DISABLE_VALUE);
    if ret != 0 { return ret; }
    if check_val != AW88166_CRC_CHECK_PASS_VAL {
        dev_err((*aw_dev).dev, c"%s failed, check_val 0x%x != 0x%x\n".as_ptr(),
            c"aw_dev_fw_crc_check".as_ptr(), check_val as c_int, AW88166_CRC_CHECK_PASS_VAL as c_int);
        ret = neg(EINVAL);
    }
    ret
}

unsafe extern "C" fn aw_dev_cfg_crc_check(aw_dev: *mut aw_device) -> c_int {
    let cfg_len_val: u16 = (((*aw_dev).dsp_cfg_len / AW_FW_ADDR_LEN) - 1).wrapping_add(AW88166_CRC_CFG_BASE_ADDR as c_uint) as u16;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_END_ADDR_MASK, cfg_len_val as c_uint);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_CFG_EN_MASK, AW88166_CRC_CFG_EN_ENABLE_VALUE);
    if ret != 0 { return ret; }
    usleep_range(AW88166_1000_US, AW88166_1000_US + 10);
    let mut reg_val: c_uint = 0;
    ret = regmap_read((*aw_dev).regmap, AW88166_HAGCST_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let check_val = ((reg_val & !AW88166_CRC_CHECK_BITS_MASK) >> AW88166_CRC_CHECK_START_BIT) as u16;
    ret = regmap_update_bits((*aw_dev).regmap, AW88166_CRCCTRL_REG,
        !AW88166_CRC_CFG_EN_MASK, AW88166_CRC_CFG_EN_DISABLE_VALUE);
    if ret != 0 { return ret; }
    if check_val != AW88166_CRC_CHECK_PASS_VAL {
        dev_err((*aw_dev).dev, c"crc_check failed, check val 0x%x != 0x%x\n".as_ptr(),
            check_val as c_int, AW88166_CRC_CHECK_PASS_VAL as c_int);
        ret = neg(EINVAL);
    }
    ret
}

unsafe extern "C" fn aw_dev_hw_crc_check(aw88166: *mut aw88166) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut ret = regmap_update_bits((*aw_dev).regmap, AW88166_I2SCFG1_REG,
        !AW88166_RAM_CG_BYP_MASK, AW88166_RAM_CG_BYP_BYPASS_VALUE);
    if ret != 0 { return ret; }
    ret = aw_dev_fw_crc_check(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"fw_crc_check failed\n".as_ptr());
        regmap_update_bits((*aw_dev).regmap, AW88166_I2SCFG1_REG,
            !AW88166_RAM_CG_BYP_MASK, AW88166_RAM_CG_BYP_WORK_VALUE);
        return ret;
    }
    ret = aw_dev_cfg_crc_check(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"cfg_crc_check failed\n".as_ptr());
        regmap_update_bits((*aw_dev).regmap, AW88166_I2SCFG1_REG,
            !AW88166_RAM_CG_BYP_MASK, AW88166_RAM_CG_BYP_WORK_VALUE);
        return ret;
    }
    ret = regmap_write((*aw_dev).regmap, AW88166_CRCCTRL_REG, (*aw88166).crc_init_val);
    if ret != 0 { return ret; }
    regmap_update_bits((*aw_dev).regmap, AW88166_I2SCFG1_REG,
        !AW88166_RAM_CG_BYP_MASK, AW88166_RAM_CG_BYP_WORK_VALUE)
}

unsafe extern "C" fn aw_dev_i2s_tx_enable(aw_dev: *mut aw_device, flag: bool) {
    let ret = if flag {
        regmap_update_bits((*aw_dev).regmap, AW88166_I2SCTRL3_REG,
            !AW88166_I2STXEN_MASK, AW88166_I2STXEN_ENABLE_VALUE)
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88166_I2SCTRL3_REG,
            !AW88166_I2STXEN_MASK, AW88166_I2STXEN_DISABLE_VALUE)
    };
    if ret != 0 {
        dev_dbg((*aw_dev).dev, c"%s failed".as_ptr(), c"aw_dev_i2s_tx_enable".as_ptr());
    }
}

unsafe extern "C" fn aw_dev_get_dsp_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88166_WDT_REG, &mut reg_val);
    if ret != 0 { return ret; }
    if (reg_val & !AW88166_WDT_CNT_MASK) == 0 { return neg(EPERM); }
    0
}

unsafe extern "C" fn aw_dev_dsp_check(aw_dev: *mut aw_device) -> c_int {
    let mut ret: c_int;
    if (*aw_dev).dsp_cfg == AW88166_DEV_DSP_BYPASS {
        dev_dbg((*aw_dev).dev, c"dsp bypass".as_ptr());
        ret = 0;
    } else if (*aw_dev).dsp_cfg == AW88166_DEV_DSP_WORK {
        aw_dev_dsp_enable(aw_dev, false);
        aw_dev_dsp_enable(aw_dev, true);
        usleep_range(AW88166_1000_US, AW88166_1000_US + 10);
        ret = 0;
        let mut i = 0;
        while i < AW88166_DEV_DSP_CHECK_MAX {
            ret = aw_dev_get_dsp_status(aw_dev);
            if ret != 0 {
                dev_err((*aw_dev).dev, c"dsp wdt status error=%d".as_ptr(), ret);
                usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
            }
            i += 1;
        }
    } else {
        dev_err((*aw_dev).dev, c"unknown dsp cfg=%d".as_ptr(), (*aw_dev).dsp_cfg);
        ret = neg(EINVAL);
    }
    ret
}

unsafe extern "C" fn aw_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) -> c_int {
    let vol_desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let mut real_value: u16 = min_u32(value + (*vol_desc).init_volume as c_uint, AW88166_MUTE_VOL as c_uint) as u16;
    let mut reg_value: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88166_SYSCTRL2_REG, &mut reg_value);
    if ret != 0 { return ret; }
    dev_dbg((*aw_dev).dev, c"value 0x%x , reg:0x%x".as_ptr(), value, real_value as c_int);
    real_value = ((real_value as c_uint) << AW88166_VOL_START_BIT | (reg_value & AW88166_VOL_MASK as c_uint)) as u16;
    regmap_write((*aw_dev).regmap, AW88166_SYSCTRL2_REG, real_value as c_uint)
}

unsafe extern "C" fn aw_dev_fade_in(aw_dev: *mut aw_device) {
    let desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let fade_in_vol = (*desc).ctl_volume as c_int;
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_in_time == 0 {
        aw_dev_set_volume(aw_dev, fade_in_vol as c_uint);
        return;
    }
    let mut i = AW88166_MUTE_VOL as c_int;
    while i >= fade_in_vol {
        aw_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_in_time as c_uint, (*aw_dev).fade_in_time as c_uint + 10);
        i -= fade_step;
    }
    if i != fade_in_vol {
        aw_dev_set_volume(aw_dev, fade_in_vol as c_uint);
    }
}

unsafe extern "C" fn aw_dev_fade_out(aw_dev: *mut aw_device) {
    let desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let fade_step = (*aw_dev).fade_step;
    if fade_step == 0 || (*aw_dev).fade_out_time == 0 {
        aw_dev_set_volume(aw_dev, AW88166_MUTE_VOL as c_uint);
        return;
    }
    let mut i = (*desc).ctl_volume as c_int;
    while i <= AW88166_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, i as c_uint);
        usleep_range((*aw_dev).fade_out_time as c_uint, (*aw_dev).fade_out_time as c_uint + 10);
        i += fade_step;
    }
    if i != AW88166_MUTE_VOL as c_int {
        aw_dev_set_volume(aw_dev, AW88166_MUTE_VOL as c_uint);
        usleep_range((*aw_dev).fade_out_time as c_uint, (*aw_dev).fade_out_time as c_uint + 10);
    }
}

unsafe extern "C" fn aw88166_dev_mute(aw_dev: *mut aw_device, is_mute: bool) {
    if is_mute {
        aw_dev_fade_out(aw_dev);
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_HMUTE_MASK, AW88166_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88166_SYSCTRL_REG,
            !AW88166_HMUTE_MASK, AW88166_HMUTE_DISABLE_VALUE);
        aw_dev_fade_in(aw_dev);
    }
}

unsafe extern "C" fn aw88166_dev_set_dither(aw88166: *mut aw88166, dither: bool) {
    let aw_dev = (*aw88166).aw_pa;
    if dither {
        regmap_update_bits((*aw_dev).regmap, AW88166_DBGCTRL_REG,
            !AW88166_DITHER_EN_MASK, AW88166_DITHER_EN_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88166_DBGCTRL_REG,
            !AW88166_DITHER_EN_MASK, AW88166_DITHER_EN_DISABLE_VALUE);
    }
}

unsafe extern "C" fn aw88166_dev_start(aw88166: *mut aw88166) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    if (*aw_dev).status == AW88166_DEV_PW_ON {
        dev_dbg((*aw_dev).dev, c"already power on".as_ptr());
        return 0;
    }
    aw88166_dev_set_dither(aw88166, false);
    /* power on */
    aw_dev_pwd(aw_dev, false);
    usleep_range(AW88166_2000_US, AW88166_2000_US + 10);
    let mut ret = aw_dev_check_syspll(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"pll check failed cannot start\n".as_ptr());
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88166_DEV_PW_OFF;
        return ret;
    }
    /* amppd on */
    aw_dev_amppd(aw_dev, false);
    usleep_range(AW88166_1000_US, AW88166_1000_US + 50);
    /* check i2s status */
    ret = aw_dev_check_sysst(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"sysst check failed\n".as_ptr());
        aw_dev_clear_int_status(aw_dev);
        aw_dev_amppd(aw_dev, true);
        aw_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88166_DEV_PW_OFF;
        return ret;
    }
    if (*aw_dev).dsp_cfg == AW88166_DEV_DSP_WORK {
        aw_dev_backup_sec_recovery(aw88166);
        ret = aw_dev_hw_crc_check(aw88166);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"dsp crc check failed\n".as_ptr());
            aw_dev_dsp_enable(aw_dev, false);
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88166_DEV_PW_OFF;
            return ret;
        }
        aw_dev_dsp_enable(aw_dev, false);
        aw88166_dev_set_vcalb(aw88166);
        aw_dev_update_cali_re(&mut (*aw_dev).cali_desc);
        ret = aw_dev_dsp_check(aw_dev);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"dsp status check failed\n".as_ptr());
            aw_dev_dsp_enable(aw_dev, false);
            aw_dev_clear_int_status(aw_dev);
            aw_dev_amppd(aw_dev, true);
            aw_dev_pwd(aw_dev, true);
            (*aw_dev).status = AW88166_DEV_PW_OFF;
            return ret;
        }
    } else {
        dev_dbg((*aw_dev).dev, c"start pa with dsp bypass".as_ptr());
    }
    /* enable tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, true);
    if (*aw88166).dither_st == AW88166_DITHER_EN_ENABLE_VALUE {
        aw88166_dev_set_dither(aw88166, true);
    }
    /* close mute */
    aw88166_dev_mute(aw_dev, false);
    /* clear inturrupt */
    aw_dev_clear_int_status(aw_dev);
    (*aw_dev).status = AW88166_DEV_PW_ON;
    0
}

unsafe extern "C" fn aw_dev_dsp_update_container(aw_dev: *mut aw_device,
    data: *mut c_uchar, len: c_uint, base: u16) -> c_int {
    let mut ret = regmap_write((*aw_dev).regmap, AW88166_DSPMADD_REG, base as c_uint);
    if ret != 0 { return ret; }
    let mut i: c_uint = 0;
    while i < len {
        let tmp_len = min_u32(len - i, AW88166_MAX_RAM_WRITE_BYTE_SIZE);
        ret = regmap_raw_write((*aw_dev).regmap, AW88166_DSPMDAT_REG,
            data.add(i as usize) as *const c_void, tmp_len);
        if ret != 0 { return ret; }
        i += AW88166_MAX_RAM_WRITE_BYTE_SIZE;
    }
    0
}

unsafe extern "C" fn aw_dev_get_ra(cali_desc: *mut aw_cali_desc) -> c_int {
    let aw_dev = aw88166_from_cali_desc(cali_desc);
    let mut dsp_ra: u32 = 0;
    let ret = aw_dev_dsp_read(aw_dev, AW88166_DSP_REG_CFG_ADPZ_RA, &mut dsp_ra, AW_DSP_32_DATA);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"read ra error\n".as_ptr());
        return ret;
    }
    (*cali_desc).ra = AW88166_DSP_RE_TO_SHOW_RE(dsp_ra, AW88166_DSP_RE_SHIFT);
    0
}

unsafe extern "C" fn aw_dev_dsp_update_cfg(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    dev_dbg((*aw_dev).dev, c"dsp config len:%d".as_ptr(), len);
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"dsp config data is null or len is 0\n".as_ptr());
        return neg(EINVAL);
    }
    let mut ret = aw_dev_dsp_update_container(aw_dev, data, len, AW88166_DSP_CFG_ADDR);
    if ret != 0 { return ret; }
    (*aw_dev).dsp_cfg_len = len;
    ret = aw_dev_get_ra(&mut (*aw_dev).cali_desc);
    ret
}

unsafe extern "C" fn aw_dev_dsp_update_fw(aw_dev: *mut aw_device, data: *mut c_uchar, len: c_uint) -> c_int {
    dev_dbg((*aw_dev).dev, c"dsp firmware len:%d".as_ptr(), len);
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, c"dsp firmware data is null or len is 0\n".as_ptr());
        return neg(EINVAL);
    }
    (*aw_dev).dsp_fw_len = len;
    aw_dev_dsp_update_container(aw_dev, data, len, AW88166_DSP_FW_ADDR)
}

unsafe extern "C" fn aw_dev_check_sram(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    /* read dsp_rom_check_reg */
    aw_dev_dsp_read(aw_dev, AW88166_DSP_ROM_CHECK_ADDR, &mut reg_val, AW_DSP_16_DATA);
    if reg_val != AW88166_DSP_ROM_CHECK_DATA {
        dev_err((*aw_dev).dev, c"check dsp rom failed, read[0x%x] != check[0x%x]\n".as_ptr(),
            reg_val, AW88166_DSP_ROM_CHECK_DATA);
        return neg(EPERM);
    }
    /* check dsp_cfg_base_addr */
    aw_dev_dsp_write(aw_dev, AW88166_DSP_CFG_ADDR as c_uint, AW88166_DSP_ODD_NUM_BIT_TEST, AW_DSP_16_DATA);
    aw_dev_dsp_read(aw_dev, AW88166_DSP_CFG_ADDR as c_uint, &mut reg_val, AW_DSP_16_DATA);
    if reg_val != AW88166_DSP_ODD_NUM_BIT_TEST {
        dev_err((*aw_dev).dev, c"check dsp cfg failed, read[0x%x] != write[0x%x]\n".as_ptr(),
            reg_val, AW88166_DSP_ODD_NUM_BIT_TEST);
        return neg(EPERM);
    }
    0
}

unsafe extern "C" fn aw_dev_select_memclk(aw_dev: *mut aw_device, flag: c_uchar) {
    if flag == AW88166_DEV_MEMCLK_PLL {
        let ret = regmap_update_bits((*aw_dev).regmap, AW88166_DBGCTRL_REG,
            !AW88166_MEM_CLKSEL_MASK, AW88166_MEM_CLKSEL_DAPHCLK_VALUE);
        if ret != 0 { dev_err((*aw_dev).dev, c"memclk select pll failed\n".as_ptr()); }
    } else if flag == AW88166_DEV_MEMCLK_OSC {
        let ret = regmap_update_bits((*aw_dev).regmap, AW88166_DBGCTRL_REG,
            !AW88166_MEM_CLKSEL_MASK, AW88166_MEM_CLKSEL_OSCCLK_VALUE);
        if ret != 0 { dev_err((*aw_dev).dev, c"memclk select OSC failed\n".as_ptr()); }
    } else {
        dev_err((*aw_dev).dev, c"unknown memclk config, flag=0x%x\n".as_ptr(), flag as c_int);
    }
}

unsafe extern "C" fn aw_dev_update_reg_container(aw88166: *mut aw88166,
    data: *mut c_uchar, len: c_uint) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let vol_desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let reg_data = data as *mut i16;
    let data_len: c_int = (len >> 1) as c_int;
    if (data_len & 0x1) != 0 {
        dev_err((*aw_dev).dev, c"data len:%d unsupported\n".as_ptr(), data_len);
        return neg(EINVAL);
    }
    let mut i = 0;
    while i < data_len {
        let reg_addr: u8 = *reg_data.add(i as usize) as u8;
        let mut reg_val: u16 = *reg_data.add((i + 1) as usize) as u16;
        if reg_addr as c_uint == AW88166_DSPVCALB_REG {
            (*aw88166).vcalb_init_val = reg_val as c_uint;
            i += 2;
            continue;
        }
        if reg_addr as c_uint == AW88166_SYSCTRL_REG {
            if (reg_val as c_uint & !AW88166_DSPBY_MASK) != 0 {
                (*aw_dev).dsp_cfg = AW88166_DEV_DSP_BYPASS;
            } else {
                (*aw_dev).dsp_cfg = AW88166_DEV_DSP_WORK;
            }
            reg_val = (reg_val as c_uint & (AW88166_HMUTE_MASK | AW88166_PWDN_MASK | AW88166_DSPBY_MASK)) as u16;
            reg_val |= (AW88166_HMUTE_ENABLE_VALUE | AW88166_PWDN_POWER_DOWN_VALUE | AW88166_DSPBY_BYPASS_VALUE) as u16;
        }
        if reg_addr as c_uint == AW88166_I2SCTRL3_REG {
            reg_val = (reg_val as c_uint & AW88166_I2STXEN_MASK) as u16;
            reg_val |= AW88166_I2STXEN_DISABLE_VALUE as u16;
        }
        if reg_addr as c_uint == AW88166_SYSCTRL2_REG {
            let read_vol = ((reg_val & !AW88166_VOL_MASK) as c_uint >> AW88166_VOL_START_BIT) as u16;
            (*aw_dev).volume_desc.init_volume = read_vol;
        }
        if reg_addr as c_uint == AW88166_DBGCTRL_REG {
            if (reg_val as c_uint & !AW88166_EF_DBMD_MASK) == AW88166_EF_DBMD_OR_VALUE {
                (*aw88166).check_val = AW_EF_OR_CHECK;
            } else {
                (*aw88166).check_val = AW_EF_AND_CHECK;
            }
            (*aw88166).dither_st = reg_val as c_uint & !AW88166_DITHER_EN_MASK;
        }
        if reg_addr as c_uint == AW88166_ACR1_REG {
            (*aw88166).re_init_val |= (reg_val as u32) << 16;
            i += 2;
            continue;
        }
        if reg_addr as c_uint == AW88166_ACR2_REG {
            (*aw88166).re_init_val |= reg_val as u32;
            i += 2;
            continue;
        }
        if reg_addr as c_uint == AW88166_CRCCTRL_REG {
            (*aw88166).crc_init_val = reg_val as c_uint;
        }
        let ret = regmap_write((*aw_dev).regmap, reg_addr as c_uint, reg_val as c_uint);
        if ret != 0 { return ret; }
        i += 2;
    }
    aw_dev_pwd(aw_dev, false);
    usleep_range(AW88166_1000_US, AW88166_1000_US + 10);
    if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        (*vol_desc).ctl_volume = 0;
    } else {
        aw_dev_set_volume(aw_dev, (*vol_desc).ctl_volume as c_uint);
    }
    0
}

unsafe extern "C" fn aw_dev_reg_update(aw88166: *mut aw88166, data: *mut c_uchar, len: c_uint) -> c_int {
    if len == 0 || data.is_null() {
        dev_err((*(*aw88166).aw_pa).dev, c"reg data is null or len is 0\n".as_ptr());
        return neg(EINVAL);
    }
    let ret = aw_dev_update_reg_container(aw88166, data, len);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"reg update failed\n".as_ptr());
    }
    ret
}

unsafe extern "C" fn aw88166_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut *mut c_char) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info as *mut aw_prof_info;
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"index[%d] overflow count[%d]\n".as_ptr(), index, (*aw_dev).prof_info.count);
        return neg(EINVAL);
    }
    let prof_desc = (*aw_dev).prof_info.prof_desc.add(index as usize);
    *prof_name = *(*prof_info).prof_name_list.add((*prof_desc).id as usize);
    0
}

unsafe extern "C" fn aw88166_dev_get_prof_data(aw_dev: *mut aw_device, index: c_int,
    prof_desc: *mut *mut aw_prof_desc) -> c_int {
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, c"%s: index[%d] overflow count[%d]\n".as_ptr(),
            c"aw88166_dev_get_prof_data".as_ptr(), index, (*aw_dev).prof_info.count);
        return neg(EINVAL);
    }
    *prof_desc = (*aw_dev).prof_info.prof_desc.add(index as usize);
    0
}

unsafe extern "C" fn aw88166_dev_fw_update(aw88166: *mut aw88166, up_dsp_fw_en: bool, force_up_en: bool) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut prof_index_desc: *mut aw_prof_desc = ptr::null_mut();
    let mut prof_name: *mut c_char = ptr::null_mut();
    if (*aw_dev).prof_cur == (*aw_dev).prof_index && force_up_en == AW88166_FORCE_UPDATE_OFF {
        dev_dbg((*aw_dev).dev, c"scene no change, not update".as_ptr());
        return 0;
    }
    if (*aw_dev).fw_status == AW88166_DEV_FW_FAILED {
        dev_err((*aw_dev).dev, c"fw status[%d] error\n".as_ptr(), (*aw_dev).fw_status);
        return neg(EPERM);
    }
    let mut ret = aw88166_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name);
    if ret != 0 { return ret; }
    dev_dbg((*aw_dev).dev, c"start update %s".as_ptr(), prof_name);
    ret = aw88166_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc);
    if ret != 0 { return ret; }
    /* update reg */
    let sec_desc = (*prof_index_desc).sec_desc;
    ret = aw_dev_reg_update(aw88166, (*sec_desc.offset(AW88395_DATA_TYPE_REG)).data,
        (*sec_desc.offset(AW88395_DATA_TYPE_REG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"update reg failed\n".as_ptr());
        return ret;
    }
    aw88166_dev_mute(aw_dev, true);
    if (*aw_dev).dsp_cfg == AW88166_DEV_DSP_WORK {
        aw_dev_dsp_enable(aw_dev, false);
    }
    aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_OSC);
    ret = aw_dev_check_sram(aw_dev);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"check sram failed\n".as_ptr());
        aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_PLL);
        return ret;
    }
    aw_dev_backup_sec_recovery(aw88166);
    if up_dsp_fw_en {
        dev_dbg((*aw_dev).dev, c"fw_ver: [%x]".as_ptr(), (*prof_index_desc).fw_ver);
        ret = aw_dev_dsp_update_fw(aw_dev, (*sec_desc.offset(AW88395_DATA_TYPE_DSP_FW)).data,
            (*sec_desc.offset(AW88395_DATA_TYPE_DSP_FW)).len);
        if ret != 0 {
            dev_err((*aw_dev).dev, c"update dsp fw failed\n".as_ptr());
            aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_PLL);
            return ret;
        }
    }
    /* update dsp config */
    ret = aw_dev_dsp_update_cfg(aw_dev, (*sec_desc.offset(AW88395_DATA_TYPE_DSP_CFG)).data,
        (*sec_desc.offset(AW88395_DATA_TYPE_DSP_CFG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"update dsp cfg failed\n".as_ptr());
        aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_PLL);
        return ret;
    }
    aw_dev_backup_sec_record(aw88166);
    aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_PLL);
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    0
}

unsafe extern "C" fn aw88166_start_pa(aw88166: *mut aw88166) {
    let mut i = 0;
    while i < AW88166_START_RETRIES {
        let mut ret = aw88166_dev_start(aw88166);
        if ret != 0 {
            dev_err((*(*aw88166).aw_pa).dev, c"aw88166 device start failed. retry = %d".as_ptr(), i);
            ret = aw88166_dev_fw_update(aw88166, AW88166_DSP_FW_UPDATE_ON, true);
            if ret != 0 {
                dev_err((*(*aw88166).aw_pa).dev, c"fw update failed".as_ptr());
                i += 1;
                continue;
            }
        } else {
            dev_dbg((*(*aw88166).aw_pa).dev, c"start success\n".as_ptr());
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn aw88166_startup_work(work: *mut work_struct) {
    let aw88166 = aw88166_from_work(work);
    mutex_lock(&mut (*aw88166).lock);
    aw88166_start_pa(aw88166);
    mutex_unlock(&mut (*aw88166).lock);
}

unsafe extern "C" fn aw88166_start(aw88166: *mut aw88166, sync_start: bool) {
    let mut ret: c_int;
    if (*(*aw88166).aw_pa).fw_status != AW88166_DEV_FW_OK { return; }
    if (*(*aw88166).aw_pa).status == AW88166_DEV_PW_ON { return; }
    ret = aw88166_dev_fw_update(aw88166, AW88166_DSP_FW_UPDATE_OFF, (*aw88166).phase_sync);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"fw update failed\n".as_ptr());
        return;
    }
    if sync_start == AW88166_SYNC_START {
        aw88166_start_pa(aw88166);
    } else {
        queue_delayed_work(system_dfl_wq, &mut (*aw88166).start_work, AW88166_START_WORK_DELAY_MS);
    }
}

unsafe extern "C" fn aw_dev_check_sysint(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: u16 = 0;
    aw_dev_get_int_status(aw_dev, &mut reg_val);
    if (reg_val & AW88166_BIT_SYSINT_CHECK) != 0 {
        dev_err((*aw_dev).dev, c"pa stop check fail:0x%04x\n".as_ptr(), reg_val as c_int);
        return neg(EINVAL);
    }
    0
}

unsafe extern "C" fn aw88166_stop(aw_dev: *mut aw_device) -> c_int {
    let dsp_cfg = &mut *(*(*aw_dev).prof_info.prof_desc.add((*aw_dev).prof_cur as usize))
        .sec_desc.offset(AW88395_DATA_TYPE_DSP_CFG);
    let dsp_fw = &mut *(*(*aw_dev).prof_info.prof_desc.add((*aw_dev).prof_cur as usize))
        .sec_desc.offset(AW88395_DATA_TYPE_DSP_FW);
    if (*aw_dev).status == AW88166_DEV_PW_OFF {
        dev_dbg((*aw_dev).dev, c"already power off".as_ptr());
        return 0;
    }
    (*aw_dev).status = AW88166_DEV_PW_OFF;
    aw88166_dev_mute(aw_dev, true);
    usleep_range(AW88166_4000_US, AW88166_4000_US + 100);
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88166_1000_US, AW88166_1000_US + 100);
    let int_st = aw_dev_check_sysint(aw_dev);
    aw_dev_dsp_enable(aw_dev, false);
    aw_dev_amppd(aw_dev, true);
    if int_st != 0 {
        aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_OSC);
        aw_dev_dsp_update_fw(aw_dev, dsp_fw.data, dsp_fw.len);
        aw_dev_dsp_update_cfg(aw_dev, dsp_cfg.data, dsp_cfg.len);
        aw_dev_select_memclk(aw_dev, AW88166_DEV_MEMCLK_PLL);
    }
    aw_dev_pwd(aw_dev, true);
    0
}

static mut aw88166_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"aw88166-aif".as_ptr(),
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: c"Speaker_Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88166_RATES },
        formats: unsafe { AW88166_FORMATS },
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Speaker_Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88166_RATES },
        formats: unsafe { AW88166_FORMATS },
    },
}];

unsafe extern "C" fn aw88166_get_fade_in_time(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    let aw_dev = (*aw88166).aw_pa;
    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_in_time as i64;
    0
}

unsafe extern "C" fn aw88166_set_fade_in_time(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88166).aw_pa;
    let time = (*ucontrol).value.integer.value[0] as c_int;
    if time < (*mc).min || time > (*mc).max { return neg(EINVAL); }
    if time != (*aw_dev).fade_in_time {
        (*aw_dev).fade_in_time = time;
        return 1;
    }
    0
}

unsafe extern "C" fn aw88166_get_fade_out_time(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    let aw_dev = (*aw88166).aw_pa;
    (*ucontrol).value.integer.value[0] = (*aw_dev).fade_out_time as i64;
    0
}

unsafe extern "C" fn aw88166_set_fade_out_time(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88166).aw_pa;
    let time = (*ucontrol).value.integer.value[0] as c_int;
    if time < (*mc).min || time > (*mc).max { return neg(EINVAL); }
    if time != (*aw_dev).fade_out_time {
        (*aw_dev).fade_out_time = time;
        return 1;
    }
    0
}

unsafe extern "C" fn aw88166_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int {
    /* check the index whether is valid */
    if index >= (*aw_dev).prof_info.count || index < 0 { return neg(EINVAL); }
    /* check the index whether change */
    if (*aw_dev).prof_index == index { return neg(EINVAL); }
    (*aw_dev).prof_index = index;
    dev_dbg((*aw_dev).dev, c"set prof[%s]".as_ptr(),
        *(*aw_dev).prof_info.prof_name_list.add((*(*aw_dev).prof_info.prof_desc.add(index as usize)).id as usize));
    0
}

unsafe extern "C" fn aw88166_profile_info(kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let mut prof_name: *mut c_char = ptr::null_mut();
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;
    let mut count = (*(*aw88166).aw_pa).prof_info.count;
    if count <= 0 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }
    (*uinfo).value.enumerated.items = count as c_uint;
    if (*uinfo).value.enumerated.item >= count as c_uint {
        (*uinfo).value.enumerated.item = (count - 1) as c_uint;
    }
    count = (*uinfo).value.enumerated.item as c_int;
    let ret = aw88166_dev_get_prof_name((*aw88166).aw_pa, count, &mut prof_name);
    if ret != 0 {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), c"null".as_ptr());
        return 0;
    }
    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);
    0
}

unsafe extern "C" fn aw88166_profile_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    (*ucontrol).value.integer.value[0] = (*(*aw88166).aw_pa).prof_index as i64;
    0
}

unsafe extern "C" fn aw88166_profile_set(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    mutex_lock(&mut (*aw88166).lock);
    let ret = aw88166_dev_set_profile_index((*aw88166).aw_pa, (*ucontrol).value.integer.value[0] as c_int);
    if ret != 0 {
        dev_dbg((*codec).dev, c"profile index does not change".as_ptr());
        mutex_unlock(&mut (*aw88166).lock);
        return 0;
    }
    if (*(*aw88166).aw_pa).status != 0 {
        aw88166_stop((*aw88166).aw_pa);
        aw88166_start(aw88166, AW88166_SYNC_START);
    }
    mutex_unlock(&mut (*aw88166).lock);
    1
}

unsafe extern "C" fn aw88166_volume_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let vol_desc = &mut (*(*aw88166).aw_pa).volume_desc;
    (*ucontrol).value.integer.value[0] = vol_desc.ctl_volume as i64;
    0
}

unsafe extern "C" fn aw88166_volume_set(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let vol_desc = &mut (*(*aw88166).aw_pa).volume_desc;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max { return neg(EINVAL); }
    if vol_desc.ctl_volume != value as u16 {
        vol_desc.ctl_volume = value as u16;
        aw_dev_set_volume((*aw88166).aw_pa, vol_desc.ctl_volume as c_uint);
        return 1;
    }
    0
}

unsafe extern "C" fn aw88166_get_fade_step(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    (*ucontrol).value.integer.value[0] = (*(*aw88166).aw_pa).fade_step as i64;
    0
}

unsafe extern "C" fn aw88166_set_fade_step(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max { return neg(EINVAL); }
    if (*(*aw88166).aw_pa).fade_step != value {
        (*(*aw88166).aw_pa).fade_step = value;
        return 1;
    }
    0
}

unsafe extern "C" fn aw88166_re_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let aw_dev = (*aw88166).aw_pa;
    (*ucontrol).value.integer.value[0] = (*aw_dev).cali_desc.cali_re as i64;
    0
}

unsafe extern "C" fn aw88166_re_set(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88166 = snd_soc_component_get_drvdata(codec) as *mut aw88166;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let aw_dev = (*aw88166).aw_pa;
    let value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max { return neg(EINVAL); }
    if (*aw_dev).cali_desc.cali_re != value {
        (*aw_dev).cali_desc.cali_re = value;
        return 1;
    }
    0
}

unsafe extern "C" fn aw88166_dev_init(aw88166: *mut aw88166, aw_cfg: *mut aw_container) -> c_int {
    let aw_dev = (*aw88166).aw_pa;
    let mut ret = aw88395_dev_cfg_load(aw_dev, aw_cfg);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"aw_dev acf parse failed\n".as_ptr());
        return neg(EINVAL);
    }
    (*aw_dev).fade_in_time = (AW88166_1000_US / 10) as c_int;
    (*aw_dev).fade_out_time = (AW88166_1000_US >> 1) as c_int;
    (*aw_dev).prof_cur = (*(*aw_dev).prof_info.prof_desc.add(0)).id;
    (*aw_dev).prof_index = (*(*aw_dev).prof_info.prof_desc.add(0)).id;
    ret = aw88166_dev_fw_update(aw88166, AW88166_FORCE_UPDATE_ON, AW88166_DSP_FW_UPDATE_ON);
    if ret != 0 {
        dev_err((*aw_dev).dev, c"fw update failed ret = %d\n".as_ptr(), ret);
        return ret;
    }
    aw88166_dev_mute(aw_dev, true);
    /* close tx feedback */
    aw_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88166_1000_US, AW88166_1000_US + 100);
    /* enable amppd */
    aw_dev_amppd(aw_dev, true);
    /* close dsp */
    aw_dev_dsp_enable(aw_dev, false);
    /* set power down */
    aw_dev_pwd(aw_dev, true);
    0
}

unsafe extern "C" fn aw88166_request_firmware_file(aw88166: *mut aw88166) -> c_int {
    let mut cont: *const firmware = ptr::null();
    let mut fw_name: *const c_char = ptr::null();
    (*(*aw88166).aw_pa).fw_status = AW88166_DEV_FW_FAILED;
    if device_property_read_string((*(*aw88166).aw_pa).dev, c"firmware-name".as_ptr(), &mut fw_name) < 0 {
        fw_name = AW88166_ACF_FILE;
    }
    let mut ret = request_firmware(&mut cont, fw_name, (*(*aw88166).aw_pa).dev);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"request [%s] failed!\n".as_ptr(), fw_name);
        return ret;
    }
    dev_dbg((*(*aw88166).aw_pa).dev, c"loaded %s - size: %zu\n".as_ptr(),
        fw_name, if !cont.is_null() { (*cont).size } else { 0 });
    let alloc_size = mem::size_of::<aw_container>() + (*cont).size;
    (*aw88166).aw_cfg = devm_kzalloc((*(*aw88166).aw_pa).dev, alloc_size, GFP_KERNEL) as *mut aw_container;
    if (*aw88166).aw_cfg.is_null() { return neg(ENOMEM); }
    (*(*aw88166).aw_cfg).len = (*cont).size as c_int;
    memcpy((*(*aw88166).aw_cfg).data.as_mut_ptr() as *mut c_void, (*cont).data as *const c_void, (*cont).size);
    ret = aw88395_dev_load_acf_check((*aw88166).aw_pa, (*aw88166).aw_cfg);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"load [%s] failed!\n".as_ptr(), fw_name);
        return ret;
    }
    mutex_lock(&mut (*aw88166).lock);
    /* aw device init */
    ret = aw88166_dev_init(aw88166, (*aw88166).aw_cfg);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"dev init failed\n".as_ptr());
    }
    mutex_unlock(&mut (*aw88166).lock);
    ret
}

/* The C aw88166_controls array uses SOC_SINGLE_EXT and AW88166_PROFILE_EXT
 * initializer macros. Their exact Rust item expansion depends on ALSA macro
 * definitions supplied by headers, so the controls are preserved as an empty
 * placeholder with the original functions translated above.
 */
static aw88166_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn aw88166_playback_event(w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    mutex_lock(&mut (*aw88166).lock);
    if event == SND_SOC_DAPM_PRE_PMU {
        aw88166_start(aw88166, AW88166_ASYNC_START);
    } else if event == SND_SOC_DAPM_POST_PMD {
        aw88166_stop((*aw88166).aw_pa);
    }
    mutex_unlock(&mut (*aw88166).lock);
    0
}

/* The C aw88166_dapm_widgets array uses SND_SOC_DAPM_* initializer macros.
 * The meaningful playback callback and routes are translated separately.
 */
static aw88166_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static aw88166_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"DAC Output".as_ptr(), control: ptr::null(), source: c"AIF_RX".as_ptr() },
    snd_soc_dapm_route { sink: c"AIF_TX".as_ptr(), control: ptr::null(), source: c"ADC Input".as_ptr() },
];

unsafe extern "C" fn aw88166_codec_probe(component: *mut snd_soc_component) -> c_int {
    let aw88166 = snd_soc_component_get_drvdata(component) as *mut aw88166;
    INIT_DELAYED_WORK(&mut (*aw88166).start_work, aw88166_startup_work);
    let ret = aw88166_request_firmware_file(aw88166);
    if ret != 0 {
        dev_err((*(*aw88166).aw_pa).dev, c"%s failed\n".as_ptr(), c"aw88166_codec_probe".as_ptr());
    }
    ret
}

unsafe extern "C" fn aw88166_codec_remove(aw_codec: *mut snd_soc_component) {
    let aw88166 = snd_soc_component_get_drvdata(aw_codec) as *mut aw88166;
    cancel_delayed_work_sync(&mut (*aw88166).start_work);
}

static soc_codec_dev_aw88166: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw88166_codec_probe),
    remove: Some(aw88166_codec_remove),
    dapm_widgets: aw88166_dapm_widgets.as_ptr(),
    num_dapm_widgets: aw88166_dapm_widgets.len() as c_uint,
    dapm_routes: aw88166_audio_map.as_ptr(),
    num_dapm_routes: aw88166_audio_map.len() as c_uint,
    controls: aw88166_controls.as_ptr(),
    num_controls: aw88166_controls.len() as c_uint,
};

unsafe extern "C" fn aw88166_hw_reset(aw88166: *mut aw88166) {
    if !(*aw88166).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*aw88166).reset_gpio, 1);
        usleep_range(AW88166_1000_US, AW88166_1000_US + 10);
        gpiod_set_value_cansleep((*aw88166).reset_gpio, 0);
        usleep_range(AW88166_1000_US, AW88166_1000_US + 10);
    }
}

unsafe extern "C" fn aw88166_parse_channel_dt(aw88166: *mut aw88166) {
    let aw_dev = (*aw88166).aw_pa;
    let np = (*(*aw_dev).dev).of_node;
    let mut channel_value: u32 = 0;
    of_property_read_u32(np, c"awinic,audio-channel".as_ptr(), &mut channel_value);
    (*aw_dev).channel = channel_value;
    (*aw88166).phase_sync = of_property_read_bool(np, c"awinic,sync-flag".as_ptr());
}

unsafe extern "C" fn aw88166_init(aw88166: *mut aw88166, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int {
    let mut chip_id: c_uint = 0;
    let ret = regmap_read(regmap, AW88166_ID_REG, &mut chip_id);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"%s read chipid error. ret = %d\n".as_ptr(),
            c"aw88166_init".as_ptr(), ret);
        return ret;
    }
    let aw_dev = devm_kzalloc(&mut (*i2c).dev, mem::size_of::<aw_device>(), GFP_KERNEL) as *mut aw_device;
    if aw_dev.is_null() { return neg(ENOMEM); }
    (*aw88166).aw_pa = aw_dev;
    (*aw_dev).i2c = i2c;
    (*aw_dev).dev = &mut (*i2c).dev;
    (*aw_dev).regmap = regmap;
    mutex_init(&mut (*aw_dev).dsp_lock);
    (*aw_dev).chip_id = chip_id;
    (*aw_dev).acf = ptr::null_mut();
    (*aw_dev).prof_info.prof_desc = ptr::null_mut();
    (*aw_dev).prof_info.count = 0;
    (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
    (*aw_dev).channel = AW88166_DEV_DEFAULT_CH;
    (*aw_dev).fw_status = AW88166_DEV_FW_FAILED;
    (*aw_dev).fade_step = AW88166_VOLUME_STEP_DB;
    (*aw_dev).volume_desc.ctl_volume = AW88166_VOL_DEFAULT_VALUE;
    aw88166_parse_channel_dt(aw88166);
    0
}

unsafe extern "C" fn aw88166_i2c_probe(i2c: *mut i2c_client) -> c_int {
    if !i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) {
        return dev_err_probe(&mut (*i2c).dev, neg(ENXIO), c"check_functionality failed\n".as_ptr());
    }
    let aw88166 = devm_kzalloc(&mut (*i2c).dev, mem::size_of::<aw88166>(), GFP_KERNEL) as *mut aw88166;
    if aw88166.is_null() { return neg(ENOMEM); }
    mutex_init(&mut (*aw88166).lock);
    i2c_set_clientdata(i2c, aw88166 as *mut c_void);
    (*aw88166).reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, c"reset".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*aw88166).reset_gpio as *const c_void) {
        return dev_err_probe(&mut (*i2c).dev, PTR_ERR((*aw88166).reset_gpio as *const c_void),
            c"reset gpio not defined\n".as_ptr());
    }
    aw88166_hw_reset(aw88166);
    (*aw88166).regmap = devm_regmap_init_i2c(i2c, &aw88166_remap_config);
    if IS_ERR((*aw88166).regmap as *const c_void) {
        return dev_err_probe(&mut (*i2c).dev, PTR_ERR((*aw88166).regmap as *const c_void),
            c"failed to init regmap\n".as_ptr());
    }
    /* aw pa init */
    let ret = aw88166_init(aw88166, i2c, (*aw88166).regmap);
    if ret != 0 { return ret; }
    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_dev_aw88166,
        aw88166_dai.as_mut_ptr(), aw88166_dai.len() as c_int)
}

static aw88166_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: unsafe { AW88166_I2C_NAME } },
    i2c_device_id { name: ptr::null() },
];
/* MODULE_DEVICE_TABLE(i2c, aw88166_i2c_id); */

static mut aw88166_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: unsafe { AW88166_I2C_NAME },
    },
    probe: Some(aw88166_i2c_probe),
    id_table: aw88166_i2c_id.as_ptr(),
};
/* module_i2c_driver(aw88166_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC AW88166 Smart PA Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
