// SPDX-License-Identifier: GPL-2.0-only
//
// aw88261.c  --  AW88261 ALSA SoC Audio driver
//
// Copyright (c) 2023 awinic Technology CO., LTD
//
// Author: Jimmy Zhang <zhangjianming@awinic.com>
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type int16_t = i16;
type size_t = usize;

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub reg_format_endian: c_uint,
    pub val_format_endian: c_uint,
}

#[repr(C)]
pub struct aw_volume_desc {
    pub ctl_volume: c_int,
    pub mute_volume: c_int,
    pub init_volume: c_int,
}

#[repr(C)]
pub struct aw_sec_data_desc {
    pub data: *mut u8,
    pub len: c_uint,
}

#[repr(C)]
pub struct aw_prof_desc {
    pub id: c_int,
    pub sec_desc: *mut aw_sec_data_desc,
}

#[repr(C)]
pub struct aw_prof_info {
    pub prof_desc: *mut aw_prof_desc,
    pub count: c_int,
    pub prof_type: c_int,
    pub prof_name_list: *mut *mut c_char,
}

#[repr(C)]
pub struct aw_device {
    pub i2c: *mut i2c_client,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub chip_id: c_uint,
    pub acf: *mut c_void,
    pub prof_info: aw_prof_info,
    pub channel: u32,
    pub fw_status: c_int,
    pub status: c_int,
    pub prof_cur: c_int,
    pub prof_index: c_int,
    pub volume_desc: aw_volume_desc,
}

#[repr(C)]
pub struct aw_container {
    pub len: c_int,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct aw88261 {
    pub aw_pa: *mut aw_device,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub lock: mutex,
    pub slot_num_value: c_uint,
    pub sr_value: c_uint,
    pub cco_mux_value: c_uint,
    pub fs_value: c_uint,
    pub bck_value: c_uint,
    pub bck_inv_value: c_uint,
    pub tdm_bck_value: c_uint,
    pub md_value: c_uint,
    pub tx_slotvld_mask: c_uint,
    pub rxl_slotvld_mask: c_uint,
    pub rxr_slotvld_mask: c_uint,
    pub phase_sync: bool,
    pub mute_st: c_int,
    pub amppd_st: c_int,
    pub efuse_check: c_int,
    pub frcset_en: c_int,
}

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct i2c_client { pub dev: device, pub adapter: *mut i2c_adapter }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dai { pub component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_info { pub type_: c_uint, pub count: c_uint, pub value: snd_ctl_elem_info_value }
#[repr(C)] pub union snd_ctl_elem_info_value { pub enumerated: snd_ctl_elem_info_enumerated }
#[repr(C)] pub struct snd_ctl_elem_info_enumerated { pub items: c_uint, pub item: c_uint, pub name: [c_char; 64] }
#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_value }
#[repr(C)] pub union snd_ctl_elem_value_value { pub integer: snd_ctl_elem_value_integer }
#[repr(C)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] pub struct soc_mixer_control { pub min: c_int, pub max: c_int }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dai_ops { pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>, pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { pub sink: *const c_char, pub control: *const c_char, pub source: *const c_char }
#[repr(C)] pub struct snd_soc_dapm_widget_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component_driver { pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int> }
#[repr(C)] pub struct i2c_device_id { pub name: [c_char; 32], pub driver_data: usize }
#[repr(C)] pub struct of_device_id { pub name: [c_char; 32], pub type_: [c_char; 32], pub compatible: *const c_char, pub data: *const c_void }
#[repr(C)] pub struct device_driver { pub name: *const c_char, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct i2c_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, pub id_table: *const i2c_device_id }
#[repr(C)] pub struct firmware { pub size: size_t, pub data: *const u8 }

extern "C" {
    static AW88261_REG_MAX: c_uint;
    static REGMAP_ENDIAN_LITTLE: c_uint;
    static REGMAP_ENDIAN_BIG: c_uint;
    static AW88261_MUTE_VOL: c_int;
    static AW88261_SYSCTRL2_REG: c_uint;
    static AW88261_VOL_MASK: c_uint;
    static AW88261_I2SCFG1_REG: c_uint;
    static AW88261_I2STXEN_MASK: c_uint;
    static AW88261_I2STXEN_ENABLE_VALUE: c_uint;
    static AW88261_I2STXEN_DISABLE_VALUE: c_uint;
    static AW88261_SYSCTRL_REG: c_uint;
    static AW88261_PWDN_MASK: c_uint;
    static AW88261_PWDN_POWER_DOWN_VALUE: c_uint;
    static AW88261_PWDN_WORKING_VALUE: c_uint;
    static AW88261_AMPPD_MASK: c_uint;
    static AW88261_AMPPD_POWER_DOWN_VALUE: c_uint;
    static AW88261_AMPPD_WORKING_VALUE: c_uint;
    static AW88261_HMUTE_MASK: c_uint;
    static AW88261_HMUTE_ENABLE_VALUE: c_uint;
    static AW88261_HMUTE_DISABLE_VALUE: c_uint;
    static AW88261_SYSINT_REG: c_uint;
    static AW88261_SYSST_REG: c_uint;
    static AW88261_BIT_PLL_CHECK: c_uint;
    static AW88261_DEV_SYSST_CHECK_MAX: c_int;
    static AW88261_2000_US: c_uint;
    static AW88261_I2SCTRL2_REG: c_uint;
    static AW88261_SLOT_NUM_MASK: c_uint;
    static AW88261_I2S_TX_SLOTVLD_MASK: c_uint;
    static AW88261_I2S_RXL_SLOTVLD_MASK: c_uint;
    static AW88261_I2S_RXR_SLOTVLD_MASK: c_uint;
    static AW88261_PLLCTRL1_REG: c_uint;
    static AW88261_CCO_MUX_MASK: c_uint;
    static AW88261_I2SCTRL1_REG: c_uint;
    static AW88261_I2SSR_MASK: c_uint;
    static AW88261_I2SBCK_MASK: c_uint;
    static AW88261_TDM_BCK_UNSET: c_uint;
    static AW88261_I2SFS_MASK: c_uint;
    static AW88261_I2SMD_MASK: c_uint;
    static AW88261_BCKINV_MASK: c_uint;
    static AW88261_BIT_SYSST_CHECK_MASK: c_uint;
    static AW88261_ULS_HMUTE_MASK: c_uint;
    static AW88261_ULS_HMUTE_ENABLE_VALUE: c_uint;
    static AW88261_ULS_HMUTE_DISABLE_VALUE: c_uint;
    static AW88261_FRCSET_ENABLE: c_int;
    static AW88261_BSTCTRL3_REG: c_uint;
    static AW88261_FORCE_PWM_MASK: c_uint;
    static AW88261_FORCE_PWM_FORCEMINUS_PWM_VALUE: c_uint;
    static AW88261_BSTCTRL5_REG: c_uint;
    static AW88261_BST_OS_WIDTH_MASK: c_uint;
    static AW88261_BST_OS_WIDTH_50NS_VALUE: c_uint;
    static AW88261_BSTCTRL6_REG: c_uint;
    static AW88261_BST_LOOPR_MASK: c_uint;
    static AW88261_BST_LOOPR_340K_VALUE: c_uint;
    static AW88261_BSTCTRL7_REG: c_uint;
    static AW88261_RSQN_DLY_MASK: c_uint;
    static AW88261_RSQN_DLY_35NS_VALUE: c_uint;
    static AW88261_BSTCTRL8_REG: c_uint;
    static AW88261_BURST_SSMODE_MASK: c_uint;
    static AW88261_BURST_SSMODE_FAST_VALUE: c_uint;
    static AW88261_BSTCTRL9_REG: c_uint;
    static AW88261_BST_BURST_MASK: c_uint;
    static AW88261_BST_BURST_30MA_VALUE: c_uint;
    static AW88261_EFRH4_REG: c_uint;
    static AW88261_EF_ISN_GESLP_H_MASK: u16;
    static AW88261_EFRL4_REG: c_uint;
    static AW88261_EF_ISN_GESLP_L_MASK: u16;
    static AW88261_ICALK_SHIFT: c_uint;
    static AW88261_ICALKL_SHIFT: c_uint;
    static AW88261_EF_ISN_GESLP_SIGN_MASK: u16;
    static AW88261_EF_ISN_GESLP_NEG: u16;
    static AW88261_EFRH3_REG: c_uint;
    static AW88261_EF_VSN_GESLP_H_MASK: u16;
    static AW88261_EFRL3_REG: c_uint;
    static AW88261_EF_VSN_GESLP_L_MASK: u16;
    static AW88261_VCALK_SHIFT: c_uint;
    static AW88261_VCALKL_SHIFT: c_uint;
    static AW88261_EF_VSN_GESLP_SIGN_MASK: u16;
    static AW88261_EF_VSN_GESLP_NEG: u16;
    static AW88261_CABL_BASE_VALUE: c_int;
    static AW88261_ICABLK_FACTOR: c_int;
    static AW88261_VCABLK_FACTOR: c_int;
    static AW88261_VCAL_FACTOR: c_int;
    static AW88261_VSNTM1_REG: c_uint;
    static AW88261_DBGCTRL_REG: c_uint;
    static AW88261_EF_DBMD_MASK: u16;
    static AW88261_OR_VALUE: u16;
    static AW88261_EF_OR_CHECK: c_int;
    static AW88261_EF_AND_CHECK: c_int;
    static AW88261_I2SCTRL3_REG: c_uint;
    static AW88261_VOL_START_BIT: c_uint;
    static AW88261_DEV_PW_ON: c_int;
    static AW88261_DEV_PW_OFF: c_int;
    static AW88261_1000_US: c_uint;
    static AW88261_ID_REG: c_uint;
    static AW88261_SOFT_RESET_VALUE: c_uint;
    static AW88261_START_RETRIES: c_int;
    static AW88261_DEV_FW_OK: c_int;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_MSB: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_LSB: c_uint;
    static AW88261_BCKINV_NOT_INVERT_VALUE: c_uint;
    static AW88261_BCKINV_INVERTED_VALUE: c_uint;
    static AW88261_I2SMD_PHILIPS_STANDARD_VALUE: c_uint;
    static AW88261_I2SMD_MSB_JUSTIFIED_VALUE: c_uint;
    static AW88261_I2SMD_LSB_JUSTIFIED_VALUE: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static AW88261_CCO_MUX_BYPASS_VALUE: c_uint;
    static AW88261_CCO_MUX_DIVIDED_VALUE: c_uint;
    static AW88261_I2SSR_8KHZ_VALUE: c_uint;
    static AW88261_I2SSR_11P025KHZ_VALUE: c_uint;
    static AW88261_I2SSR_12KHZ_VALUE: c_uint;
    static AW88261_I2SSR_16KHZ_VALUE: c_uint;
    static AW88261_I2SSR_22P05KHZ_VALUE: c_uint;
    static AW88261_I2SSR_24KHZ_VALUE: c_uint;
    static AW88261_I2SSR_32KHZ_VALUE: c_uint;
    static AW88261_I2SSR_44P1KHZ_VALUE: c_uint;
    static AW88261_I2SSR_48KHZ_VALUE: c_uint;
    static AW88261_I2SSR_96KHZ_VALUE: c_uint;
    static AW88261_I2SSR_192KHZ_VALUE: c_uint;
    static AW88261_I2SFS_16_BITS_VALUE: c_uint;
    static AW88261_I2SFS_20_BITS_VALUE: c_uint;
    static AW88261_I2SFS_24_BITS_VALUE: c_uint;
    static AW88261_I2SFS_32_BITS_VALUE: c_uint;
    static AW88261_I2SBCK_32FS_VALUE: c_uint;
    static AW88261_I2SBCK_48FS_VALUE: c_uint;
    static AW88261_I2SBCK_64FS_VALUE: c_uint;
    static AW88261_SLOT_NUM_I2S_MODE_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM1S_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM2S_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM4S_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM6S_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM8S_VALUE: c_uint;
    static AW88261_SLOT_NUM_TDM16S_VALUE: c_uint;
    static AW88261_I2S_TX_SLOTVLD_START_BIT: c_uint;
    static AW88261_I2S_RXL_SLOTVLD_START_BIT: c_uint;
    static AW88261_I2S_RXR_SLOTVLD_START_BIT: c_uint;
    static AW88261_RATES: c_uint;
    static AW88261_FORMATS: u64;
    static SNDRV_CTL_ELEM_TYPE_ENUMERATED: c_uint;
    static AW88261_CTL_MAX_VOL: c_int;
    static AW88261_TEMH_MASK: u16;
    static AW88261_TEML_MASK: u16;
    static AW88261_DEFAULT_CFG: u16;
    static AW88261_FRCSET_DISABLE: c_int;
    static AW88261_INIT_PROFILE: c_int;
    static AW88395_DATA_TYPE_REG: isize;
    static AW88261_DEV_FW_FAILED: c_int;
    static AW88261_ACF_FILE: *const c_char;
    static GFP_KERNEL: c_uint;
    static AW88261_DEV_DEFAULT_CH: u32;
    static AW88261_CHIP_ID: c_uint;
    static AW88395_DEV_NONE_TYPE_ID: c_int;
    static AW88261_CTL_DEFAULT_VOL: c_int;
    static I2C_FUNC_I2C: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;

    fn DB_TO_REG_VAL(value: c_uint) -> c_uint;
    fn REG_VAL_TO_DB(value: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aw88261;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn __ffs(word: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn device_property_read_string(dev: *mut device, propname: *const c_char, val: *mut *const c_char) -> c_int;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget_desc, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, num_controls: c_int) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn devm_regulator_get_enable(dev: *mut device, id: *const c_char) -> c_int;
    fn i2c_check_functionality(adap: *mut i2c_adapter, func: c_uint) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;

unsafe fn BIT(nr: c_int) -> c_uint {
    1u32 << nr
}

static aw88261_remap_config: regmap_config = unsafe {
    regmap_config {
        val_bits: 16,
        reg_bits: 8,
        max_register: AW88261_REG_MAX,
        reg_format_endian: REGMAP_ENDIAN_LITTLE,
        val_format_endian: REGMAP_ENDIAN_BIG,
    }
};

unsafe fn aw88261_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) {
    let volume: c_uint = core::cmp::min(value, AW88261_MUTE_VOL as c_uint);
    regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL2_REG, !AW88261_VOL_MASK, DB_TO_REG_VAL(volume));
}

unsafe fn aw88261_dev_i2s_tx_enable(aw_dev: *mut aw_device, flag: bool) {
    if flag {
        regmap_update_bits((*aw_dev).regmap, AW88261_I2SCFG1_REG, !AW88261_I2STXEN_MASK, AW88261_I2STXEN_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88261_I2SCFG1_REG, !AW88261_I2STXEN_MASK, AW88261_I2STXEN_DISABLE_VALUE);
    }
}

unsafe fn aw88261_dev_pwd(aw_dev: *mut aw_device, pwd: bool) {
    if pwd {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_PWDN_MASK, AW88261_PWDN_POWER_DOWN_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_PWDN_MASK, AW88261_PWDN_WORKING_VALUE);
    }
}

unsafe fn aw88261_dev_amppd(aw_dev: *mut aw_device, amppd: bool) {
    if amppd {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_AMPPD_MASK, AW88261_AMPPD_POWER_DOWN_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_AMPPD_MASK, AW88261_AMPPD_WORKING_VALUE);
    }
}

unsafe fn aw88261_dev_mute(aw_dev: *mut aw_device, is_mute: bool) {
    if is_mute {
        aw88261_dev_set_volume(aw_dev, AW88261_MUTE_VOL as c_uint);
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_HMUTE_MASK, AW88261_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_HMUTE_MASK, AW88261_HMUTE_DISABLE_VALUE);
        aw88261_dev_set_volume(aw_dev, (*aw_dev).volume_desc.ctl_volume as c_uint);
    }
}

unsafe fn aw88261_dev_clear_int_status(aw_dev: *mut aw_device) {
    let mut int_status: c_uint = 0;
    /* read int status and clear */
    regmap_read((*aw_dev).regmap, AW88261_SYSINT_REG, &mut int_status);
    /* make sure int status is clear */
    regmap_read((*aw_dev).regmap, AW88261_SYSINT_REG, &mut int_status);
    dev_dbg((*aw_dev).dev, b"read interrupt reg = 0x%04x\0".as_ptr() as *const c_char, int_status);
}

unsafe fn aw88261_dev_get_iis_status(aw_dev: *mut aw_device) -> c_int {
    let mut reg_val: c_uint = 0;
    let ret = regmap_read((*aw_dev).regmap, AW88261_SYSST_REG, &mut reg_val);
    if ret != 0 {
        return ret;
    }
    if (reg_val & AW88261_BIT_PLL_CHECK) != AW88261_BIT_PLL_CHECK {
        dev_dbg((*aw_dev).dev, b"check pll lock fail,reg_val:0x%04x\0".as_ptr() as *const c_char, reg_val);
        return -EINVAL;
    }
    ret
}

unsafe fn aw88261_dev_check_pll(aw_dev: *mut aw_device) -> c_int {
    let mut i = 0;
    while i < AW88261_DEV_SYSST_CHECK_MAX {
        let ret = aw88261_dev_get_iis_status(aw_dev);
        if ret != 0 {
            dev_dbg((*aw_dev).dev, b"mode1 iis signal check error\0".as_ptr() as *const c_char);
            usleep_range(AW88261_2000_US, AW88261_2000_US + 10);
        } else {
            return ret;
        }
        i += 1;
    }
    -EPERM
}

unsafe fn aw88261_dev_configure_syspll(aw88261: *mut aw88261) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    let mut ret: c_int;

    /* Configure TDM slots (I2S is represented as no slots) */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL2_REG, !AW88261_SLOT_NUM_MASK, (*aw88261).slot_num_value);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL2_REG, !AW88261_I2S_TX_SLOTVLD_MASK, (*aw88261).tx_slotvld_mask);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL2_REG, !AW88261_I2S_RXL_SLOTVLD_MASK, (*aw88261).rxl_slotvld_mask);
    if ret != 0 { return ret; }
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL2_REG, !AW88261_I2S_RXR_SLOTVLD_MASK, (*aw88261).rxr_slotvld_mask);
    if ret != 0 { return ret; }
    /* PLL divider must be used for 8/16/32 kHz modes */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_PLLCTRL1_REG, !AW88261_CCO_MUX_MASK, (*aw88261).cco_mux_value);
    if ret != 0 { return ret; }
    /* The word clock (WCK) defines the beginning of a frame */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL1_REG, !AW88261_I2SSR_MASK, (*aw88261).sr_value);
    if ret != 0 { return ret; }
    /* The bit clock (BCK) defines the length of a frame */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL1_REG, !AW88261_I2SBCK_MASK,
        if (*aw88261).tdm_bck_value != AW88261_TDM_BCK_UNSET { (*aw88261).tdm_bck_value } else { (*aw88261).bck_value });
    if ret != 0 { return ret; }
    /* The logical frame size is the width of data for 1 slot */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL1_REG, !AW88261_I2SFS_MASK, (*aw88261).fs_value);
    if ret != 0 { return ret; }
    /* The I2S interface mode (Philips standard, LSB/MSB justified) */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_I2SCTRL1_REG, !AW88261_I2SMD_MASK, (*aw88261).md_value);
    if ret != 0 { return ret; }
    /* The polarity of the bit clock (BCK) */
    ret = regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_BCKINV_MASK, (*aw88261).bck_inv_value);
    if ret != 0 { return ret; }
    aw88261_dev_check_pll(aw_dev)
}

unsafe fn aw88261_dev_check_sysst(aw_dev: *mut aw_device) -> c_int {
    let mut i = 0;
    while i < AW88261_DEV_SYSST_CHECK_MAX {
        let mut reg_val: c_uint = 0;
        let ret = regmap_read((*aw_dev).regmap, AW88261_SYSST_REG, &mut reg_val);
        if ret != 0 { return ret; }
        let check_val = reg_val & (!AW88261_BIT_SYSST_CHECK_MASK) & AW88261_BIT_PLL_CHECK;
        if check_val != AW88261_BIT_PLL_CHECK {
            dev_dbg((*aw_dev).dev, b"check sysst fail, reg_val=0x%04x, check:0x%x\0".as_ptr() as *const c_char, reg_val, AW88261_BIT_PLL_CHECK);
            usleep_range(AW88261_2000_US, AW88261_2000_US + 10);
        } else {
            return 0;
        }
        i += 1;
    }
    -EPERM
}

unsafe fn aw88261_dev_uls_hmute(aw_dev: *mut aw_device, uls_hmute: bool) {
    if uls_hmute {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_ULS_HMUTE_MASK, AW88261_ULS_HMUTE_ENABLE_VALUE);
    } else {
        regmap_update_bits((*aw_dev).regmap, AW88261_SYSCTRL_REG, !AW88261_ULS_HMUTE_MASK, AW88261_ULS_HMUTE_DISABLE_VALUE);
    }
}

unsafe fn aw88261_reg_force_set(aw88261: *mut aw88261) {
    if (*aw88261).frcset_en == AW88261_FRCSET_ENABLE {
        /* set FORCE_PWM */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL3_REG, !AW88261_FORCE_PWM_MASK, AW88261_FORCE_PWM_FORCEMINUS_PWM_VALUE);
        /* set BOOST_OS_WIDTH */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL5_REG, !AW88261_BST_OS_WIDTH_MASK, AW88261_BST_OS_WIDTH_50NS_VALUE);
        /* set BURST_LOOPR */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL6_REG, !AW88261_BST_LOOPR_MASK, AW88261_BST_LOOPR_340K_VALUE);
        /* set RSQN_DLY */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL7_REG, !AW88261_RSQN_DLY_MASK, AW88261_RSQN_DLY_35NS_VALUE);
        /* set BURST_SSMODE */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL8_REG, !AW88261_BURST_SSMODE_MASK, AW88261_BURST_SSMODE_FAST_VALUE);
        /* set BST_BURST */
        regmap_update_bits((*aw88261).regmap, AW88261_BSTCTRL9_REG, !AW88261_BST_BURST_MASK, AW88261_BST_BURST_30MA_VALUE);
    } else {
        dev_dbg((*(*aw88261).aw_pa).dev, b"needn't set reg value\0".as_ptr() as *const c_char);
    }
}

unsafe fn aw88261_dev_get_icalk(aw_dev: *mut aw_device, icalk: *mut int16_t) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88261_EFRH4_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let mut reg_icalk: u16 = (reg_val as u16) & !AW88261_EF_ISN_GESLP_H_MASK;
    ret = regmap_read((*aw_dev).regmap, AW88261_EFRL4_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let reg_icalkl: u16 = (reg_val as u16) & !AW88261_EF_ISN_GESLP_L_MASK;
    reg_icalk = (reg_icalk >> AW88261_ICALK_SHIFT) & (reg_icalkl >> AW88261_ICALKL_SHIFT);
    if (reg_icalk & !AW88261_EF_ISN_GESLP_SIGN_MASK) != 0 {
        reg_icalk |= !AW88261_EF_ISN_GESLP_NEG;
    }
    *icalk = reg_icalk as int16_t;
    ret
}

unsafe fn aw88261_dev_get_vcalk(aw_dev: *mut aw_device, vcalk: *mut int16_t) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw_dev).regmap, AW88261_EFRH3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let mut reg_vcalk: u16 = (reg_val as u16) & !AW88261_EF_VSN_GESLP_H_MASK;
    ret = regmap_read((*aw_dev).regmap, AW88261_EFRL3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let reg_vcalkl: u16 = (reg_val as u16) & !AW88261_EF_VSN_GESLP_L_MASK;
    reg_vcalk = (reg_vcalk >> AW88261_VCALK_SHIFT) & (reg_vcalkl >> AW88261_VCALKL_SHIFT);
    if (reg_vcalk & AW88261_EF_VSN_GESLP_SIGN_MASK) != 0 {
        reg_vcalk |= !AW88261_EF_VSN_GESLP_NEG;
    }
    *vcalk = reg_vcalk as int16_t;
    ret
}

unsafe fn aw88261_dev_set_vcalb(aw_dev: *mut aw_device) -> c_int {
    let mut icalk_val: int16_t = 0;
    let mut vcalk_val: int16_t = 0;
    let mut ret = aw88261_dev_get_icalk(aw_dev, &mut icalk_val);
    if ret != 0 { return ret; }
    ret = aw88261_dev_get_vcalk(aw_dev, &mut vcalk_val);
    if ret != 0 { return ret; }
    let icalk = AW88261_CABL_BASE_VALUE + AW88261_ICABLK_FACTOR * icalk_val as c_int;
    let vcalk = AW88261_CABL_BASE_VALUE + AW88261_VCABLK_FACTOR * vcalk_val as c_int;
    if vcalk == 0 { return -EINVAL; }
    let vcalb = AW88261_VCAL_FACTOR * icalk / vcalk;
    let reg_val: u32 = vcalb as c_uint;
    dev_dbg((*aw_dev).dev, b"icalk=%d, vcalk=%d, vcalb=%d, reg_val=0x%04x\0".as_ptr() as *const c_char, icalk, vcalk, vcalb, reg_val);
    regmap_write((*aw_dev).regmap, AW88261_VSNTM1_REG, reg_val)
}

unsafe fn aw88261_dev_reg_update(aw88261: *mut aw88261, data: *mut u8, len: c_uint) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    let vol_desc = &mut (*aw_dev).volume_desc as *mut aw_volume_desc;
    let mut ret: c_int = 0;
    if len == 0 || data.is_null() {
        dev_err((*aw_dev).dev, b"reg data is null or len is 0\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    let reg_data = data as *mut int16_t;
    let data_len = (len >> 1) as c_int;
    if (data_len & 0x1) != 0 {
        dev_err((*aw_dev).dev, b"data len:%d unsupported\0".as_ptr() as *const c_char, data_len);
        return -EINVAL;
    }
    let mut i = 0;
    while i < data_len {
        let reg_addr: u8 = *reg_data.offset(i as isize) as u8;
        let mut reg_val: u16 = *reg_data.offset((i + 1) as isize) as u16;
        if reg_addr as c_uint == AW88261_SYSCTRL_REG {
            (*aw88261).amppd_st = (reg_val as c_uint & !AW88261_AMPPD_MASK) as c_int;
            let mut read_val: c_uint = 0;
            ret = regmap_read((*aw_dev).regmap, reg_addr as c_uint, &mut read_val);
            if ret != 0 { break; }
            /* keep all three bits from current hw status */
            read_val &= (!AW88261_AMPPD_MASK) | (!AW88261_PWDN_MASK) | (!AW88261_HMUTE_MASK);
            reg_val &= (AW88261_AMPPD_MASK & AW88261_PWDN_MASK & AW88261_HMUTE_MASK) as u16;
            reg_val |= read_val as u16;
            /* enable uls hmute */
            reg_val &= AW88261_ULS_HMUTE_MASK as u16;
            reg_val |= AW88261_ULS_HMUTE_ENABLE_VALUE as u16;
        }
        if reg_addr as c_uint == AW88261_DBGCTRL_REG {
            let efcheck_val = reg_val & !AW88261_EF_DBMD_MASK;
            if efcheck_val == AW88261_OR_VALUE {
                (*aw88261).efuse_check = AW88261_EF_OR_CHECK;
            } else {
                (*aw88261).efuse_check = AW88261_EF_AND_CHECK;
            }
        }
        /* i2stxen */
        if reg_addr as c_uint == AW88261_I2SCTRL3_REG {
            /* close tx */
            reg_val &= AW88261_I2STXEN_MASK as u16;
            reg_val |= AW88261_I2STXEN_DISABLE_VALUE as u16;
        }
        if reg_addr as c_uint == AW88261_SYSCTRL2_REG {
            let read_vol = ((reg_val as c_uint & !AW88261_VOL_MASK) >> AW88261_VOL_START_BIT) as c_uint;
            (*aw_dev).volume_desc.init_volume = REG_VAL_TO_DB(read_vol);
        }
        if reg_addr as c_uint == AW88261_VSNTM1_REG {
            i += 2;
            continue;
        }
        ret = regmap_write((*aw_dev).regmap, reg_addr as c_uint, reg_val as c_uint);
        if ret != 0 { break; }
        i += 2;
    }
    ret = aw88261_dev_set_vcalb(aw_dev);
    if ret != 0 { return ret; }
    if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        (*vol_desc).ctl_volume = 0;
    }
    /* keep min volume */
    aw88261_dev_set_volume(aw_dev, (*vol_desc).mute_volume as c_uint);
    ret
}

unsafe fn aw88261_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut *mut c_char) -> c_int {
    let prof_info = &mut (*aw_dev).prof_info as *mut aw_prof_info;
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, b"index[%d] overflow count[%d]\0".as_ptr() as *const c_char, index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    let prof_desc = (*aw_dev).prof_info.prof_desc.offset(index as isize);
    *prof_name = *(*prof_info).prof_name_list.offset((*prof_desc).id as isize);
    0
}

unsafe fn aw88261_dev_get_prof_data(aw_dev: *mut aw_device, index: c_int, prof_desc: *mut *mut aw_prof_desc) -> c_int {
    if index >= (*aw_dev).prof_info.count || index < 0 {
        dev_err((*aw_dev).dev, b"%s: index[%d] overflow count[%d]\n\0".as_ptr() as *const c_char, b"aw88261_dev_get_prof_data\0".as_ptr() as *const c_char, index, (*aw_dev).prof_info.count);
        return -EINVAL;
    }
    *prof_desc = (*aw_dev).prof_info.prof_desc.offset(index as isize);
    0
}

unsafe fn aw88261_dev_fw_update(aw88261: *mut aw88261) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    let mut prof_index_desc: *mut aw_prof_desc = core::ptr::null_mut();
    let mut prof_name: *mut c_char = core::ptr::null_mut();
    let mut ret = aw88261_dev_get_prof_name(aw_dev, (*aw_dev).prof_index, &mut prof_name);
    if ret != 0 {
        dev_err((*aw_dev).dev, b"get prof name failed\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    dev_dbg((*aw_dev).dev, b"start update %s\0".as_ptr() as *const c_char, prof_name);
    ret = aw88261_dev_get_prof_data(aw_dev, (*aw_dev).prof_index, &mut prof_index_desc);
    if ret != 0 { return ret; }
    /* update reg */
    let sec_desc = (*prof_index_desc).sec_desc;
    ret = aw88261_dev_reg_update(aw88261, (*sec_desc.offset(AW88395_DATA_TYPE_REG)).data, (*sec_desc.offset(AW88395_DATA_TYPE_REG)).len);
    if ret != 0 {
        dev_err((*aw_dev).dev, b"update reg failed\0".as_ptr() as *const c_char);
        return ret;
    }
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    ret
}

unsafe fn aw88261_dev_start(aw88261: *mut aw88261) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    if (*aw_dev).status == AW88261_DEV_PW_ON {
        dev_dbg((*aw_dev).dev, b"already power on\0".as_ptr() as *const c_char);
        return 0;
    }
    /* power on */
    aw88261_dev_pwd(aw_dev, false);
    usleep_range(AW88261_2000_US, AW88261_2000_US + 10);
    let mut ret = aw88261_dev_configure_syspll(aw88261);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, b"pll check failed\0".as_ptr() as *const c_char);
        aw88261_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88261_DEV_PW_OFF;
        return ret;
    }
    /* amppd on */
    aw88261_dev_amppd(aw_dev, false);
    usleep_range(AW88261_1000_US, AW88261_1000_US + 50);
    /* check i2s status */
    ret = aw88261_dev_check_sysst(aw_dev);
    if ret != 0 {
        dev_dbg((*aw_dev).dev, b"sysst check failed\0".as_ptr() as *const c_char);
        aw88261_dev_i2s_tx_enable(aw_dev, false);
        aw88261_dev_clear_int_status(aw_dev);
        aw88261_dev_amppd(aw_dev, true);
        aw88261_dev_pwd(aw_dev, true);
        (*aw_dev).status = AW88261_DEV_PW_OFF;
        return ret;
    }
    /* enable tx feedback */
    aw88261_dev_i2s_tx_enable(aw_dev, true);
    if (*aw88261).amppd_st != 0 {
        aw88261_dev_amppd(aw_dev, true);
    }
    aw88261_reg_force_set(aw88261);
    /* close uls mute */
    aw88261_dev_uls_hmute(aw_dev, false);
    /* close mute */
    if (*aw88261).mute_st == 0 {
        aw88261_dev_mute(aw_dev, false);
    }
    /* clear inturrupt */
    aw88261_dev_clear_int_status(aw_dev);
    (*aw_dev).status = AW88261_DEV_PW_ON;
    0
}

unsafe fn aw88261_dev_stop(aw_dev: *mut aw_device) -> c_int {
    if (*aw_dev).status == AW88261_DEV_PW_OFF {
        dev_info((*aw_dev).dev, b"already power off\0".as_ptr() as *const c_char);
        return 0;
    }
    (*aw_dev).status = AW88261_DEV_PW_OFF;
    /* clear inturrupt */
    aw88261_dev_clear_int_status(aw_dev);
    aw88261_dev_uls_hmute(aw_dev, true);
    /* set mute */
    aw88261_dev_mute(aw_dev, true);
    /* close tx feedback */
    aw88261_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88261_1000_US, AW88261_1000_US + 100);
    /* enable amppd */
    aw88261_dev_amppd(aw_dev, true);
    /* set power down */
    aw88261_dev_pwd(aw_dev, true);
    0
}

unsafe fn aw88261_reg_update(aw88261: *mut aw88261, force: bool) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    let mut ret: c_int;
    if force {
        ret = regmap_write((*aw_dev).regmap, AW88261_ID_REG, AW88261_SOFT_RESET_VALUE);
        if ret != 0 { return ret; }
        ret = aw88261_dev_fw_update(aw88261);
        if ret != 0 { return ret; }
    } else if (*aw_dev).prof_cur != (*aw_dev).prof_index {
        ret = aw88261_dev_fw_update(aw88261);
        if ret != 0 { return ret; }
    } else {
        ret = 0;
    }
    (*aw_dev).prof_cur = (*aw_dev).prof_index;
    ret
}

unsafe fn aw88261_start_pa(aw88261: *mut aw88261) {
    let mut ret: c_int = 0;
    let mut i = 0;
    while i < AW88261_START_RETRIES {
        ret = aw88261_reg_update(aw88261, (*aw88261).phase_sync);
        if ret != 0 {
            dev_dbg((*(*aw88261).aw_pa).dev, b"aw88261_reg_update failed, cnt:%d, ret:%d\n\0".as_ptr() as *const c_char, i, ret);
            i += 1;
            continue;
        }
        ret = aw88261_dev_start(aw88261);
        if ret != 0 {
            dev_dbg((*(*aw88261).aw_pa).dev, b"aw88261_dev_start failed, cnt:%d, ret:%d\n\0".as_ptr() as *const c_char, i, ret);
            i += 1;
            continue;
        } else {
            dev_dbg((*(*aw88261).aw_pa).dev, b"start success\n\0".as_ptr() as *const c_char);
            break;
        }
    }
    if ret != 0 {
        dev_err((*(*aw88261).aw_pa).dev, b"start failure (%d)\n\0".as_ptr() as *const c_char, ret);
    }
}

unsafe fn aw88261_start(aw88261: *mut aw88261) {
    if (*(*aw88261).aw_pa).fw_status != AW88261_DEV_FW_OK {
        return;
    }
    if (*(*aw88261).aw_pa).status == AW88261_DEV_PW_ON {
        return;
    }
    aw88261_start_pa(aw88261);
}

unsafe extern "C" fn aw88261_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let aw88261 = snd_soc_component_get_drvdata(component);
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => (*aw88261).bck_inv_value = AW88261_BCKINV_NOT_INVERT_VALUE,
        x if x == SND_SOC_DAIFMT_IB_NF => (*aw88261).bck_inv_value = AW88261_BCKINV_INVERTED_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported invert mode 0x%x\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_INV_MASK);
            return -EINVAL;
        }
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_DSP_A => (*aw88261).md_value = AW88261_I2SMD_PHILIPS_STANDARD_VALUE,
        x if x == SND_SOC_DAIFMT_MSB || x == SND_SOC_DAIFMT_DSP_B => (*aw88261).md_value = AW88261_I2SMD_MSB_JUSTIFIED_VALUE,
        x if x == SND_SOC_DAIFMT_LSB => (*aw88261).md_value = AW88261_I2SMD_LSB_JUSTIFIED_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported DAI format 0x%x\n\0".as_ptr() as *const c_char, fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn aw88261_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let aw88261 = snd_soc_component_get_drvdata(component);
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        return 0;
    }
    (*aw88261).cco_mux_value = AW88261_CCO_MUX_BYPASS_VALUE;
    match params_rate(params) {
        8000 => { (*aw88261).sr_value = AW88261_I2SSR_8KHZ_VALUE; (*aw88261).cco_mux_value = AW88261_CCO_MUX_DIVIDED_VALUE; }
        11025 => (*aw88261).sr_value = AW88261_I2SSR_11P025KHZ_VALUE,
        12000 => (*aw88261).sr_value = AW88261_I2SSR_12KHZ_VALUE,
        16000 => { (*aw88261).sr_value = AW88261_I2SSR_16KHZ_VALUE; (*aw88261).cco_mux_value = AW88261_CCO_MUX_DIVIDED_VALUE; }
        22050 => (*aw88261).sr_value = AW88261_I2SSR_22P05KHZ_VALUE,
        24000 => (*aw88261).sr_value = AW88261_I2SSR_24KHZ_VALUE,
        32000 => { (*aw88261).sr_value = AW88261_I2SSR_32KHZ_VALUE; (*aw88261).cco_mux_value = AW88261_CCO_MUX_DIVIDED_VALUE; }
        44100 => (*aw88261).sr_value = AW88261_I2SSR_44P1KHZ_VALUE,
        48000 => (*aw88261).sr_value = AW88261_I2SSR_48KHZ_VALUE,
        96000 => (*aw88261).sr_value = AW88261_I2SSR_96KHZ_VALUE,
        192000 => (*aw88261).sr_value = AW88261_I2SSR_192KHZ_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported sample rate %d\n\0".as_ptr() as *const c_char, params_rate(params));
            return -EINVAL;
        }
    }
    match params_width(params) {
        16 => (*aw88261).fs_value = AW88261_I2SFS_16_BITS_VALUE,
        20 => (*aw88261).fs_value = AW88261_I2SFS_20_BITS_VALUE,
        24 => (*aw88261).fs_value = AW88261_I2SFS_24_BITS_VALUE,
        32 => (*aw88261).fs_value = AW88261_I2SFS_32_BITS_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported bit width %d\n\0".as_ptr() as *const c_char, params_width(params));
            return -EINVAL;
        }
    }
    match params_physical_width(params) {
        16 => (*aw88261).bck_value = AW88261_I2SBCK_32FS_VALUE,
        24 => (*aw88261).bck_value = AW88261_I2SBCK_48FS_VALUE,
        32 => (*aw88261).bck_value = AW88261_I2SBCK_64FS_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported physical bit width %d\n\0".as_ptr() as *const c_char, params_physical_width(params));
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn aw88261_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let aw88261 = snd_soc_component_get_drvdata(component);
    let mut chan: c_int = 0;
    match slots {
        0 => {
            /* Just reset everything TDM related to I2S values */
            (*aw88261).slot_num_value = AW88261_SLOT_NUM_I2S_MODE_VALUE;
            (*aw88261).tdm_bck_value = AW88261_TDM_BCK_UNSET;
            (*aw88261).tx_slotvld_mask = 0 << AW88261_I2S_TX_SLOTVLD_START_BIT;
            (*aw88261).rxl_slotvld_mask = 0 << AW88261_I2S_RXL_SLOTVLD_START_BIT;
            (*aw88261).rxr_slotvld_mask = 1 << AW88261_I2S_RXR_SLOTVLD_START_BIT;
            return 0;
        }
        1 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM1S_VALUE,
        2 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM2S_VALUE,
        4 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM4S_VALUE,
        6 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM6S_VALUE,
        8 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM8S_VALUE,
        16 => (*aw88261).slot_num_value = AW88261_SLOT_NUM_TDM16S_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported slot count %d\n\0".as_ptr() as *const c_char, slots);
            return -EINVAL;
        }
    }
    match slot_width {
        16 => (*aw88261).tdm_bck_value = AW88261_I2SBCK_32FS_VALUE,
        20 | 24 => (*aw88261).tdm_bck_value = AW88261_I2SBCK_48FS_VALUE,
        32 => (*aw88261).tdm_bck_value = AW88261_I2SBCK_64FS_VALUE,
        _ => {
            dev_err((*(*aw88261).aw_pa).dev, b"unsupported slot width %d\n\0".as_ptr() as *const c_char, slot_width);
            return -EINVAL;
        }
    }
    if tx_mask != 0 {
        chan = __ffs(tx_mask);
        if chan > 16 { return -EINVAL; }
        (*aw88261).tx_slotvld_mask = (chan as c_uint) << AW88261_I2S_TX_SLOTVLD_START_BIT;
    }
    if rx_mask != 0 {
        chan = __ffs(rx_mask);
        if chan > 16 { return -EINVAL; }
        (*aw88261).rxl_slotvld_mask = (chan as c_uint) << AW88261_I2S_RXL_SLOTVLD_START_BIT;
    }
    if (rx_mask & !BIT(chan)) != 0 {
        chan = __ffs(rx_mask & !BIT(chan));
        if chan > 16 { return -EINVAL; }
        (*aw88261).rxr_slotvld_mask = (chan as c_uint) << AW88261_I2S_RXR_SLOTVLD_START_BIT;
    }
    0
}

static aw88261_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(aw88261_set_fmt),
    hw_params: Some(aw88261_hw_params),
    set_tdm_slot: Some(aw88261_set_tdm_slot),
};

static mut aw88261_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"aw88261-aif\0".as_ptr() as *const c_char,
    id: 1,
    playback: snd_soc_pcm_stream {
        stream_name: b"Speaker_Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88261_RATES },
        formats: unsafe { AW88261_FORMATS },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Speaker_Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { AW88261_RATES },
        formats: unsafe { AW88261_FORMATS },
    },
    ops: &aw88261_dai_ops,
}];

unsafe fn aw88261_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int {
    /* check the index whether is valid */
    if index >= (*aw_dev).prof_info.count || index < 0 {
        return -EINVAL;
    }
    /* check the index whether change */
    if (*aw_dev).prof_index == index {
        return -EPERM;
    }
    (*aw_dev).prof_index = index;
    0
}

unsafe extern "C" fn aw88261_profile_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88261 = snd_soc_component_get_drvdata(codec);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_ENUMERATED;
    (*uinfo).count = 1;
    let mut count = (*(*aw88261).aw_pa).prof_info.count;
    if count <= 0 {
        (*uinfo).value.enumerated.items = 0;
        return 0;
    }
    (*uinfo).value.enumerated.items = count as c_uint;
    if (*uinfo).value.enumerated.item >= count as c_uint {
        (*uinfo).value.enumerated.item = (count - 1) as c_uint;
    }
    count = (*uinfo).value.enumerated.item as c_int;
    let mut prof_name: *mut c_char = core::ptr::null_mut();
    let ret = aw88261_dev_get_prof_name((*aw88261).aw_pa, count, &mut prof_name);
    if ret != 0 {
        strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), b"null\0".as_ptr() as *const c_char);
        return 0;
    }
    strscpy((*uinfo).value.enumerated.name.as_mut_ptr(), prof_name);
    0
}

unsafe extern "C" fn aw88261_profile_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88261 = snd_soc_component_get_drvdata(codec);
    (*ucontrol).value.integer.value[0] = (*(*aw88261).aw_pa).prof_index as i64;
    0
}

unsafe extern "C" fn aw88261_profile_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88261 = snd_soc_component_get_drvdata(codec);
    /* pa stop or stopping just set profile */
    /* C used guard(mutex)(&aw88261->lock); */
    let ret = aw88261_dev_set_profile_index((*aw88261).aw_pa, (*ucontrol).value.integer.value[0] as c_int);
    if ret != 0 {
        dev_dbg((*codec).dev, b"profile index does not change\0".as_ptr() as *const c_char);
        return 0;
    }
    if (*(*aw88261).aw_pa).status != 0 {
        aw88261_dev_stop((*aw88261).aw_pa);
        aw88261_start(aw88261);
    }
    1
}

unsafe extern "C" fn aw88261_volume_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88261 = snd_soc_component_get_drvdata(codec);
    let vol_desc = &mut (*(*aw88261).aw_pa).volume_desc as *mut aw_volume_desc;
    (*ucontrol).value.integer.value[0] = ((AW88261_MUTE_VOL - (*vol_desc).ctl_volume) / 2) as i64;
    0
}

unsafe extern "C" fn aw88261_volume_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let aw88261 = snd_soc_component_get_drvdata(codec);
    let vol_desc = &mut (*(*aw88261).aw_pa).volume_desc as *mut aw_volume_desc;
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let mut value = (*ucontrol).value.integer.value[0] as c_int;
    if value < (*mc).min || value > (*mc).max {
        return -EINVAL;
    }
    value = AW88261_MUTE_VOL - (value * 2);
    if (*vol_desc).ctl_volume != value {
        (*vol_desc).ctl_volume = value;
        aw88261_dev_set_volume((*aw88261).aw_pa, (*vol_desc).ctl_volume as c_uint);
        return 1;
    }
    0
}

/*
 * The field contains 4 bits in units of 6dB + 6 bits in units of 0.125dB
 * which is too precise for TLV (!) so we have to multiply the scale by 2.
 *
 * The range is clamped at -90dB to prevent overflowing the 4-bit part.
 *
 * C: static const DECLARE_TLV_DB_SCALE(volume_tlv, -9000, 25, 0);
 * C controls:
 *   SOC_SINGLE_EXT_TLV("PCM Playback Volume", AW88261_SYSCTRL2_REG, 6,
 *       AW88261_CTL_MAX_VOL, 1, aw88261_volume_get, aw88261_volume_set, volume_tlv)
 *   AW88261_PROFILE_EXT("Profile Set", aw88261_profile_info,
 *       aw88261_profile_get, aw88261_profile_set)
 */
static volume_tlv: [c_uint; 4] = [0, (-9000i32) as c_uint, 25, 0];
static aw88261_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn aw88261_playback_event(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let aw88261 = snd_soc_component_get_drvdata(component);
    /* C used guard(mutex)(&aw88261->lock); */
    match event {
        x if x == SND_SOC_DAPM_PRE_PMU => aw88261_start(aw88261),
        x if x == SND_SOC_DAPM_POST_PMD => { aw88261_dev_stop((*aw88261).aw_pa); }
        _ => {}
    }
    0
}

/*
 * C DAPM widgets:
 * SND_SOC_DAPM_AIF_IN_E("AIF_RX", "Speaker_Playback", 0, 0, 0, 0,
 *     aw88261_playback_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD)
 * SND_SOC_DAPM_OUTPUT("DAC Output")
 * SND_SOC_DAPM_AIF_OUT("AIF_TX", "Speaker_Capture", 0, SND_SOC_NOPM, 0, 0)
 * SND_SOC_DAPM_INPUT("ADC Input")
 */
static aw88261_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static aw88261_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"DAC Output\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"AIF_RX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AIF_TX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"ADC Input\0".as_ptr() as *const c_char },
];

unsafe fn aw88261_frcset_check(aw88261: *mut aw88261) -> c_int {
    let mut reg_val: c_uint = 0;
    let mut ret = regmap_read((*aw88261).regmap, AW88261_EFRH3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let temh: u16 = (reg_val as u16) & !AW88261_TEMH_MASK;
    ret = regmap_read((*aw88261).regmap, AW88261_EFRL3_REG, &mut reg_val);
    if ret != 0 { return ret; }
    let teml: u16 = (reg_val as u16) & !AW88261_TEML_MASK;
    let tem = if (*aw88261).efuse_check == AW88261_EF_OR_CHECK { temh | teml } else { temh & teml };
    if tem == AW88261_DEFAULT_CFG {
        (*aw88261).frcset_en = AW88261_FRCSET_ENABLE;
    } else {
        (*aw88261).frcset_en = AW88261_FRCSET_DISABLE;
    }
    dev_dbg((*(*aw88261).aw_pa).dev, b"tem is 0x%04x, frcset_en is %d\0".as_ptr() as *const c_char, tem as c_uint, (*aw88261).frcset_en);
    ret
}

unsafe fn aw88261_dev_init(aw88261: *mut aw88261, aw_cfg: *mut aw_container) -> c_int {
    let aw_dev = (*aw88261).aw_pa;
    let mut ret = aw88395_dev_cfg_load(aw_dev, aw_cfg);
    if ret != 0 {
        dev_err((*aw_dev).dev, b"aw_dev acf parse failed\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    ret = regmap_write((*aw_dev).regmap, AW88261_ID_REG, AW88261_SOFT_RESET_VALUE);
    if ret != 0 { return ret; }
    (*aw_dev).prof_cur = AW88261_INIT_PROFILE;
    (*aw_dev).prof_index = AW88261_INIT_PROFILE;
    ret = aw88261_dev_fw_update(aw88261);
    if ret != 0 {
        dev_err((*aw_dev).dev, b"fw update failed ret = %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = aw88261_frcset_check(aw88261);
    if ret != 0 {
        dev_err((*aw_dev).dev, b"aw88261_frcset_check ret = %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    aw88261_dev_clear_int_status(aw_dev);
    aw88261_dev_uls_hmute(aw_dev, true);
    aw88261_dev_mute(aw_dev, true);
    aw88261_dev_i2s_tx_enable(aw_dev, false);
    usleep_range(AW88261_1000_US, AW88261_1000_US + 100);
    aw88261_dev_amppd(aw_dev, true);
    aw88261_dev_pwd(aw_dev, true);
    0
}

unsafe fn aw88261_request_firmware_file(aw88261: *mut aw88261) -> c_int {
    let mut cont: *const firmware = core::ptr::null();
    let mut fw_name: *const c_char = core::ptr::null();
    (*(*aw88261).aw_pa).fw_status = AW88261_DEV_FW_FAILED;
    let mut ret = device_property_read_string((*(*aw88261).aw_pa).dev, b"firmware-name\0".as_ptr() as *const c_char, &mut fw_name);
    if ret != 0 {
        fw_name = AW88261_ACF_FILE;
    }
    ret = request_firmware(&mut cont, fw_name, (*(*aw88261).aw_pa).dev);
    if ret != 0 {
        return dev_err_probe((*(*aw88261).aw_pa).dev, ret, b"load [%s] failed!\0".as_ptr() as *const c_char, fw_name);
    }
    dev_info((*(*aw88261).aw_pa).dev, b"loaded %s - size: %zu\n\0".as_ptr() as *const c_char, fw_name, if !cont.is_null() { (*cont).size } else { 0 });
    let size = core::mem::size_of::<aw_container>() + if !cont.is_null() { (*cont).size } else { 0 };
    let aw_cfg = devm_kzalloc((*(*aw88261).aw_pa).dev, size, GFP_KERNEL) as *mut aw_container;
    if aw_cfg.is_null() {
        return -ENOMEM;
    }
    (*aw_cfg).len = (*cont).size as c_int;
    memcpy((*aw_cfg).data.as_ptr() as *mut c_void, (*cont).data as *const c_void, (*cont).size);
    (*aw88261).aw_cfg = aw_cfg;
    ret = aw88395_dev_load_acf_check((*aw88261).aw_pa, (*aw88261).aw_cfg);
    if ret != 0 {
        dev_err((*(*aw88261).aw_pa).dev, b"load [%s] failed !\0".as_ptr() as *const c_char, fw_name);
        return ret;
    }
    /* C used scoped_guard(mutex, &aw88261->lock) around device init. */
    ret = aw88261_dev_init(aw88261, (*aw88261).aw_cfg);
    if ret != 0 {
        dev_err((*(*aw88261).aw_pa).dev, b"dev init failed\0".as_ptr() as *const c_char);
    }
    ret
}

unsafe extern "C" fn aw88261_codec_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let aw88261 = snd_soc_component_get_drvdata(component);
    let mut ret = aw88261_request_firmware_file(aw88261);
    if ret != 0 {
        return dev_err_probe((*(*aw88261).aw_pa).dev, ret, b"aw88261_request_firmware_file failed\n\0".as_ptr() as *const c_char);
    }
    /* add widgets */
    ret = snd_soc_dapm_new_controls(dapm, aw88261_dapm_widgets.as_ptr(), aw88261_dapm_widgets.len() as c_int);
    if ret != 0 { return ret; }
    /* add route */
    ret = snd_soc_dapm_add_routes(dapm, aw88261_audio_map.as_ptr(), aw88261_audio_map.len() as c_int);
    if ret != 0 { return ret; }
    ret = snd_soc_add_component_controls(component, aw88261_controls.as_ptr(), aw88261_controls.len() as c_int);
    ret
}

static soc_codec_dev_aw88261: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(aw88261_codec_probe),
};

unsafe fn aw88261_parse_channel_dt(aw88261: *mut aw88261) {
    let aw_dev = (*aw88261).aw_pa;
    let np = (*(*aw_dev).dev).of_node;
    let mut channel_value: u32 = AW88261_DEV_DEFAULT_CH;
    of_property_read_u32(np, b"awinic,audio-channel\0".as_ptr() as *const c_char, &mut channel_value);
    (*aw88261).phase_sync = of_property_read_bool(np, b"awinic,sync-flag\0".as_ptr() as *const c_char);
    (*aw_dev).channel = channel_value;
}

unsafe fn aw88261_init(aw88261: *mut aw88261, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int {
    let mut ret = devm_regulator_get_enable(&mut (*i2c).dev, b"dvdd\0".as_ptr() as *const c_char);
    if ret != 0 {
        return dev_err_probe(&mut (*i2c).dev, ret, b"Failed to enable dvdd supply\n\0".as_ptr() as *const c_char);
    }
    /* read chip id */
    let mut chip_id: c_uint = 0;
    ret = regmap_read(regmap, AW88261_ID_REG, &mut chip_id);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"%s read chipid error. ret = %d\0".as_ptr() as *const c_char, b"aw88261_init\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if chip_id != AW88261_CHIP_ID {
        dev_err(&mut (*i2c).dev, b"unsupported device id = %x\0".as_ptr() as *const c_char, chip_id);
        return -ENXIO;
    }
    dev_info(&mut (*i2c).dev, b"chip id = %x\n\0".as_ptr() as *const c_char, chip_id);
    let aw_dev = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<aw_device>(), GFP_KERNEL) as *mut aw_device;
    if aw_dev.is_null() {
        return -ENOMEM;
    }
    (*aw88261).aw_pa = aw_dev;
    (*aw_dev).i2c = i2c;
    (*aw_dev).regmap = regmap;
    (*aw_dev).dev = &mut (*i2c).dev;
    (*aw_dev).chip_id = AW88261_CHIP_ID;
    (*aw_dev).acf = core::ptr::null_mut();
    (*aw_dev).prof_info.prof_desc = core::ptr::null_mut();
    (*aw_dev).prof_info.count = 0;
    (*aw_dev).prof_info.prof_type = AW88395_DEV_NONE_TYPE_ID;
    (*aw_dev).channel = 0;
    (*aw_dev).fw_status = AW88261_DEV_FW_FAILED;
    (*aw_dev).volume_desc.ctl_volume = AW88261_CTL_DEFAULT_VOL;
    (*aw_dev).volume_desc.mute_volume = AW88261_MUTE_VOL;
    aw88261_parse_channel_dt(aw88261);
    ret
}

unsafe extern "C" fn aw88261_i2c_probe(i2c: *mut i2c_client) -> c_int {
    if i2c_check_functionality((*i2c).adapter, I2C_FUNC_I2C) == 0 {
        return dev_err_probe(&mut (*i2c).dev, -ENXIO, b"check_functionality failed\0".as_ptr() as *const c_char);
    }
    let aw88261 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<aw88261>(), GFP_KERNEL) as *mut aw88261;
    if aw88261.is_null() {
        return -ENOMEM;
    }
    /* set defaults */
    (*aw88261).slot_num_value = AW88261_SLOT_NUM_I2S_MODE_VALUE;
    (*aw88261).sr_value = AW88261_I2SSR_48KHZ_VALUE;
    (*aw88261).cco_mux_value = AW88261_CCO_MUX_BYPASS_VALUE;
    (*aw88261).fs_value = AW88261_I2SFS_24_BITS_VALUE;
    (*aw88261).bck_value = AW88261_I2SBCK_64FS_VALUE;
    (*aw88261).bck_inv_value = AW88261_BCKINV_NOT_INVERT_VALUE;
    (*aw88261).tdm_bck_value = AW88261_TDM_BCK_UNSET;
    (*aw88261).md_value = AW88261_I2SMD_PHILIPS_STANDARD_VALUE;
    (*aw88261).rxr_slotvld_mask = 1 << AW88261_I2S_RXR_SLOTVLD_START_BIT;
    mutex_init(&mut (*aw88261).lock);
    i2c_set_clientdata(i2c, aw88261 as *mut c_void);
    (*aw88261).regmap = devm_regmap_init_i2c(i2c, &aw88261_remap_config);
    if IS_ERR((*aw88261).regmap as *const c_void) {
        let ret = PTR_ERR((*aw88261).regmap as *const c_void);
        return dev_err_probe(&mut (*i2c).dev, ret, b"failed to init regmap: %d\n\0".as_ptr() as *const c_char, ret);
    }
    /* aw pa init */
    let mut ret = aw88261_init(aw88261, i2c, (*aw88261).regmap);
    if ret != 0 {
        return ret;
    }
    ret = devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_dev_aw88261, aw88261_dai.as_mut_ptr(), aw88261_dai.len() as c_int);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"failed to register aw88261: %d\0".as_ptr() as *const c_char, ret);
    }
    ret
}

static aw88261_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'a' as c_char, b'w' as c_char, b'8' as c_char, b'8' as c_char, b'2' as c_char, b'6' as c_char, b'1' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], driver_data: 0 },
    i2c_device_id { name: [0; 32], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(i2c, aw88261_i2c_id); */

static aw88261_of_table: [of_device_id; 2] = [
    of_device_id { name: [0; 32], type_: [0; 32], compatible: b"awinic,aw88261\0".as_ptr() as *const c_char, data: core::ptr::null() },
    of_device_id { name: [0; 32], type_: [0; 32], compatible: core::ptr::null(), data: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, aw88261_of_table); */

static mut aw88261_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"aw88261\0".as_ptr() as *const c_char,
        of_match_table: aw88261_of_table.as_ptr(),
    },
    probe: Some(aw88261_i2c_probe),
    id_table: aw88261_i2c_id.as_ptr(),
};
/* module_i2c_driver(aw88261_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC AW88261 Smart PA Driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
