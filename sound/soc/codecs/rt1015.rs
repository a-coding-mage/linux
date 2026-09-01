// SPDX-License-Identifier: GPL-2.0
//
// rt1015.c  --  RT1015 ALSA SoC audio amplifier driver
//
// Copyright 2019 Realtek Semiconductor Corp.
//
// Author: Jack Yu <jack.yu@realtek.com>
//
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
    pub value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1015_platform_data {
    pub power_up_delay_ms: c_uint,
}

#[repr(C)]
pub struct rt1015_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub pdata: rt1015_platform_data,
    pub sysclk: c_uint,
    pub sysclk_src: c_int,
    pub pll_in: c_uint,
    pub pll_out: c_uint,
    pub pll_src: c_int,
    pub boost_mode: c_int,
    pub bypass_boost: c_int,
    pub dac_is_used: c_int,
    pub cali_done: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: c_int,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
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
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn msleep(msecs: c_uint);
    fn rl6231_get_clk_info(sysclk: c_uint, lrck: c_int) -> c_int;
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn hweight_long(w: c_ulong) -> c_uint;
    fn __ffs(word: c_ulong) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static RT1015_RESET: c_uint;
    static RT1015_CLK2: c_uint;
    static RT1015_CLK3: c_uint;
    static RT1015_PLL1: c_uint;
    static RT1015_PLL2: c_uint;
    static RT1015_DUM_RW1: c_uint;
    static RT1015_DUM_RW2: c_uint;
    static RT1015_DUM_RW3: c_uint;
    static RT1015_DUM_RW4: c_uint;
    static RT1015_DUM_RW5: c_uint;
    static RT1015_DUM_RW6: c_uint;
    static RT1015_CLK_DET: c_uint;
    static RT1015_SIL_DET: c_uint;
    static RT1015_CUSTOMER_ID: c_uint;
    static RT1015_PCODE_FWVER: c_uint;
    static RT1015_VER_ID: c_uint;
    static RT1015_VENDOR_ID: c_uint;
    static RT1015_DEVICE_ID: c_uint;
    static RT1015_PAD_DRV1: c_uint;
    static RT1015_PAD_DRV2: c_uint;
    static RT1015_GAT_BOOST: c_uint;
    static RT1015_PRO_ALT: c_uint;
    static RT1015_OSCK_STA: c_uint;
    static RT1015_MAN_I2C: c_uint;
    static RT1015_DAC1: c_uint;
    static RT1015_DAC2: c_uint;
    static RT1015_DAC3: c_uint;
    static RT1015_ADC1: c_uint;
    static RT1015_ADC2: c_uint;
    static RT1015_TDM_MASTER: c_uint;
    static RT1015_TDM_TCON: c_uint;
    static RT1015_TDM1_1: c_uint;
    static RT1015_TDM1_2: c_uint;
    static RT1015_TDM1_3: c_uint;
    static RT1015_TDM1_4: c_uint;
    static RT1015_TDM1_5: c_uint;
    static RT1015_MIXER1: c_uint;
    static RT1015_MIXER2: c_uint;
    static RT1015_ANA_PROTECT1: c_uint;
    static RT1015_ANA_CTRL_SEQ1: c_uint;
    static RT1015_ANA_CTRL_SEQ2: c_uint;
    static RT1015_VBAT_DET_DEB: c_uint;
    static RT1015_VBAT_VOLT_DET1: c_uint;
    static RT1015_VBAT_VOLT_DET2: c_uint;
    static RT1015_VBAT_TEST_OUT1: c_uint;
    static RT1015_VBAT_TEST_OUT2: c_uint;
    static RT1015_VBAT_PROT_ATT: c_uint;
    static RT1015_VBAT_DET_CODE: c_uint;
    static RT1015_PWR1: c_uint;
    static RT1015_PWR4: c_uint;
    static RT1015_PWR5: c_uint;
    static RT1015_PWR6: c_uint;
    static RT1015_PWR7: c_uint;
    static RT1015_PWR8: c_uint;
    static RT1015_PWR9: c_uint;
    static RT1015_CLASSD_SEQ: c_uint;
    static RT1015_SMART_BST_CTRL1: c_uint;
    static RT1015_SMART_BST_CTRL2: c_uint;
    static RT1015_ANA_CTRL1: c_uint;
    static RT1015_ANA_CTRL2: c_uint;
    static RT1015_PWR_STATE_CTRL: c_uint;
    static RT1015_MONO_DYNA_CTRL: c_uint;
    static RT1015_MONO_DYNA_CTRL1: c_uint;
    static RT1015_MONO_DYNA_CTRL2: c_uint;
    static RT1015_MONO_DYNA_CTRL3: c_uint;
    static RT1015_MONO_DYNA_CTRL4: c_uint;
    static RT1015_MONO_DYNA_CTRL5: c_uint;
    static RT1015_SPK_VOL: c_uint;
    static RT1015_SHORT_DETTOP1: c_uint;
    static RT1015_SHORT_DETTOP2: c_uint;
    static RT1015_SPK_DC_DETECT1: c_uint;
    static RT1015_SPK_DC_DETECT2: c_uint;
    static RT1015_SPK_DC_DETECT3: c_uint;
    static RT1015_SPK_DC_DETECT4: c_uint;
    static RT1015_SPK_DC_DETECT5: c_uint;
    static RT1015_BAT_RPO_STEP1: c_uint;
    static RT1015_BAT_RPO_STEP2: c_uint;
    static RT1015_BAT_RPO_STEP3: c_uint;
    static RT1015_BAT_RPO_STEP4: c_uint;
    static RT1015_BAT_RPO_STEP5: c_uint;
    static RT1015_BAT_RPO_STEP6: c_uint;
    static RT1015_BAT_RPO_STEP7: c_uint;
    static RT1015_BAT_RPO_STEP8: c_uint;
    static RT1015_BAT_RPO_STEP9: c_uint;
    static RT1015_BAT_RPO_STEP10: c_uint;
    static RT1015_BAT_RPO_STEP11: c_uint;
    static RT1015_BAT_RPO_STEP12: c_uint;
    static RT1015_SPREAD_SPEC1: c_uint;
    static RT1015_SPREAD_SPEC2: c_uint;
    static RT1015_PAD_STATUS: c_uint;
    static RT1015_PADS_PULLING_CTRL1: c_uint;
    static RT1015_PADS_DRIVING: c_uint;
    static RT1015_SYS_RST1: c_uint;
    static RT1015_SYS_RST2: c_uint;
    static RT1015_SYS_GATING1: c_uint;
    static RT1015_TEST_MODE1: c_uint;
    static RT1015_TEST_MODE2: c_uint;
    static RT1015_TIMING_CTRL1: c_uint;
    static RT1015_PLL_INT: c_uint;
    static RT1015_TEST_OUT1: c_uint;
    static RT1015_DC_CALIB_CLSD1: c_uint;
    static RT1015_DC_CALIB_CLSD2: c_uint;
    static RT1015_DC_CALIB_CLSD3: c_uint;
    static RT1015_DC_CALIB_CLSD4: c_uint;
    static RT1015_DC_CALIB_CLSD5: c_uint;
    static RT1015_DC_CALIB_CLSD6: c_uint;
    static RT1015_DC_CALIB_CLSD7: c_uint;
    static RT1015_DC_CALIB_CLSD8: c_uint;
    static RT1015_DC_CALIB_CLSD9: c_uint;
    static RT1015_DC_CALIB_CLSD10: c_uint;
    static RT1015_CLSD_INTERNAL1: c_uint;
    static RT1015_CLSD_INTERNAL2: c_uint;
    static RT1015_CLSD_INTERNAL3: c_uint;
    static RT1015_CLSD_INTERNAL4: c_uint;
    static RT1015_CLSD_INTERNAL5: c_uint;
    static RT1015_CLSD_INTERNAL6: c_uint;
    static RT1015_CLSD_INTERNAL7: c_uint;
    static RT1015_CLSD_INTERNAL8: c_uint;
    static RT1015_CLSD_INTERNAL9: c_uint;
    static RT1015_CLSD_OCP_CTRL: c_uint;
    static RT1015_VREF_LV: c_uint;
    static RT1015_MBIAS1: c_uint;
    static RT1015_MBIAS2: c_uint;
    static RT1015_MBIAS3: c_uint;
    static RT1015_MBIAS4: c_uint;
    static RT1015_VREF_LV1: c_uint;
    static RT1015_S_BST_TIMING_INTER1: c_uint;
    static RT1015_S_BST_TIMING_INTER2: c_uint;
    static RT1015_S_BST_TIMING_INTER3: c_uint;
    static RT1015_S_BST_TIMING_INTER4: c_uint;
    static RT1015_S_BST_TIMING_INTER5: c_uint;
    static RT1015_S_BST_TIMING_INTER6: c_uint;
    static RT1015_S_BST_TIMING_INTER7: c_uint;
    static RT1015_S_BST_TIMING_INTER8: c_uint;
    static RT1015_S_BST_TIMING_INTER9: c_uint;
    static RT1015_S_BST_TIMING_INTER10: c_uint;
    static RT1015_S_BST_TIMING_INTER11: c_uint;
    static RT1015_S_BST_TIMING_INTER12: c_uint;
    static RT1015_S_BST_TIMING_INTER13: c_uint;
    static RT1015_S_BST_TIMING_INTER14: c_uint;
    static RT1015_S_BST_TIMING_INTER15: c_uint;
    static RT1015_S_BST_TIMING_INTER16: c_uint;
    static RT1015_S_BST_TIMING_INTER17: c_uint;
    static RT1015_S_BST_TIMING_INTER18: c_uint;
    static RT1015_S_BST_TIMING_INTER19: c_uint;
    static RT1015_S_BST_TIMING_INTER20: c_uint;
    static RT1015_S_BST_TIMING_INTER21: c_uint;
    static RT1015_S_BST_TIMING_INTER22: c_uint;
    static RT1015_S_BST_TIMING_INTER23: c_uint;
    static RT1015_S_BST_TIMING_INTER24: c_uint;
    static RT1015_S_BST_TIMING_INTER25: c_uint;
    static RT1015_S_BST_TIMING_INTER26: c_uint;
    static RT1015_S_BST_TIMING_INTER27: c_uint;
    static RT1015_S_BST_TIMING_INTER28: c_uint;
    static RT1015_S_BST_TIMING_INTER29: c_uint;
    static RT1015_S_BST_TIMING_INTER30: c_uint;
    static RT1015_S_BST_TIMING_INTER31: c_uint;
    static RT1015_S_BST_TIMING_INTER32: c_uint;
    static RT1015_S_BST_TIMING_INTER33: c_uint;
    static RT1015_S_BST_TIMING_INTER34: c_uint;
    static RT1015_S_BST_TIMING_INTER35: c_uint;
    static RT1015_S_BST_TIMING_INTER36: c_uint;
    static RT1015_ABST_AUTO_EN_MASK: c_uint;
    static RT1015_ABST_FIX_TGT_MASK: c_uint;
    static RT1015_BYPASS_SWR_REG_MASK: c_uint;
    static RT1015_ABST_REG_MODE: c_uint;
    static RT1015_ABST_FIX_TGT_DIS: c_uint;
    static RT1015_BYPASS_SWRREG_BYPASS: c_uint;
    static RT1015_ABST_AUTO_MODE: c_uint;
    static RT1015_BYPASS_SWRREG_PASS: c_uint;
    static RT1015_ABST_FIX_TGT_EN: c_uint;
    static RT1015_Bypass_Boost: c_int;
    static RT1015_Enable_Boost: c_int;
    static RT1015_DAC_VOL_SFT: c_uint;
    static RT1015_DA_MUTE_SFT: c_uint;
    static RT1015_DVOL_MUTE_FLAG_SFT: c_uint;
    static RT1015_PWR_PLL_BIT: c_uint;
    static RT1015_EN_BCLK_DET_MASK: c_uint;
    static RT1015_EN_BCLK_DET: c_uint;
    static RT1015_EN_CLA_D_DC_DET_MASK: c_uint;
    static RT1015_EN_CLA_D_DC_DET: c_uint;
    static RT1015_I2S_DL_20: c_uint;
    static RT1015_I2S_DL_24: c_uint;
    static RT1015_I2S_DL_8: c_uint;
    static RT1015_I2S_DL_MASK: c_uint;
    static RT1015_FS_PD_MASK: c_uint;
    static RT1015_FS_PD_SFT: c_uint;
    static RT1015_TCON_TDM_MS_M: c_uint;
    static RT1015_TCON_TDM_MS_S: c_uint;
    static RT1015_TCON_TDM_MS_MASK: c_uint;
    static RT1015_TDM_INV_BCLK: c_uint;
    static RT1015_TDM_INV_BCLK_MASK: c_uint;
    static RT1015_I2S_M_DF_LEFT: c_uint;
    static RT1015_I2S_M_DF_PCM_A: c_uint;
    static RT1015_I2S_M_DF_PCM_B: c_uint;
    static RT1015_I2S_M_DF_MASK: c_uint;
    static RT1015_CLK_SYS_PRE_SEL_MCLK: c_uint;
    static RT1015_CLK_SYS_PRE_SEL_PLL: c_uint;
    static RT1015_CLK_SYS_PRE_SEL_MASK: c_uint;
    static RT1015_PLL_SEL_MASK: c_uint;
    static RT1015_PLL_SEL_PLL_SRC2: c_uint;
    static RT1015_PLL_SEL_BCLK: c_uint;
    static RT1015_PLL_M_SFT: c_uint;
    static RT1015_PLL_M_BP_SFT: c_uint;
    static RT1015_I2S_TX_2CH: c_uint;
    static RT1015_I2S_TX_4CH: c_uint;
    static RT1015_I2S_TX_6CH: c_uint;
    static RT1015_I2S_TX_8CH: c_uint;
    static RT1015_I2S_CH_TX_LEN_16B: c_uint;
    static RT1015_I2S_CH_TX_LEN_20B: c_uint;
    static RT1015_I2S_CH_TX_LEN_24B: c_uint;
    static RT1015_I2S_CH_TX_LEN_32B: c_uint;
    static RT1015_TDM_I2S_TX_L_DAC1_1_MASK: c_uint;
    static RT1015_TDM_I2S_TX_R_DAC1_1_MASK: c_uint;
    static RT1015_TDM_I2S_TX_L_DAC1_1_SFT: c_uint;
    static RT1015_TDM_I2S_TX_R_DAC1_1_SFT: c_uint;
    static RT1015_I2S_CH_TX_MASK: c_uint;
    static RT1015_I2S_CH_RX_MASK: c_uint;
    static RT1015_I2S_CH_TX_LEN_MASK: c_uint;
    static RT1015_I2S_CH_RX_LEN_MASK: c_uint;
    static RT1015_SCLK_S_PLL: c_int;
    static RT1015_SCLK_S_MCLK: c_int;
    static RT1015_PLL_S_MCLK: c_int;
    static RT1015_PLL_S_BCLK: c_int;
    static RT1015_DEVICE_ID_VAL: c_uint;
    static RT1015_DEVICE_ID_VAL2: c_uint;
}

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 2;
const SND_SOC_NOPM: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMU: c_int = 0x2;
const SND_SOC_DAPM_POST_PMD: c_int = 0x4;
const SND_SOC_DAIFMT_MASTER_MASK: c_uint = 0x3000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x3000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x0000;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0010;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 4;
const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const BYPASS: c_int = 0;
const ADAPTIVE: c_int = 1;
const FIXED_ADAPTIVE: c_int = 2;

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static i2s_default_platform_data: rt1015_platform_data = rt1015_platform_data {
    power_up_delay_ms: 50,
};

static rt1015_reg: [reg_default; 150] = [
    reg_default { reg: 0x0000, def: 0x0000 },
    reg_default { reg: 0x0004, def: 0xa000 },
    reg_default { reg: 0x0006, def: 0x0003 },
    reg_default { reg: 0x000a, def: 0x081e },
    reg_default { reg: 0x000c, def: 0x0006 },
    reg_default { reg: 0x000e, def: 0x0000 },
    reg_default { reg: 0x0010, def: 0x0000 },
    reg_default { reg: 0x0012, def: 0x0000 },
    reg_default { reg: 0x0014, def: 0x0000 },
    reg_default { reg: 0x0016, def: 0x0000 },
    reg_default { reg: 0x0018, def: 0x0000 },
    reg_default { reg: 0x0020, def: 0x8000 },
    reg_default { reg: 0x0022, def: 0x8043 },
    reg_default { reg: 0x0076, def: 0x0000 },
    reg_default { reg: 0x0078, def: 0x0000 },
    reg_default { reg: 0x007a, def: 0x0002 },
    reg_default { reg: 0x007c, def: 0x10ec },
    reg_default { reg: 0x007d, def: 0x1015 },
    reg_default { reg: 0x00f0, def: 0x5000 },
    reg_default { reg: 0x00f2, def: 0x004c },
    reg_default { reg: 0x00f3, def: 0xecfe },
    reg_default { reg: 0x00f4, def: 0x0000 },
    reg_default { reg: 0x00f6, def: 0x0400 },
    reg_default { reg: 0x0100, def: 0x0028 },
    reg_default { reg: 0x0102, def: 0xff02 },
    reg_default { reg: 0x0104, def: 0xa213 },
    reg_default { reg: 0x0106, def: 0x200c },
    reg_default { reg: 0x010c, def: 0x0000 },
    reg_default { reg: 0x010e, def: 0x0058 },
    reg_default { reg: 0x0111, def: 0x0200 },
    reg_default { reg: 0x0112, def: 0x0400 },
    reg_default { reg: 0x0114, def: 0x0022 },
    reg_default { reg: 0x0116, def: 0x0000 },
    reg_default { reg: 0x0118, def: 0x0000 },
    reg_default { reg: 0x011a, def: 0x0123 },
    reg_default { reg: 0x011c, def: 0x4567 },
    reg_default { reg: 0x0300, def: 0x203d },
    reg_default { reg: 0x0302, def: 0x001e },
    reg_default { reg: 0x0311, def: 0x0000 },
    reg_default { reg: 0x0313, def: 0x6014 },
    reg_default { reg: 0x0314, def: 0x00a2 },
    reg_default { reg: 0x031a, def: 0x00a0 },
    reg_default { reg: 0x031c, def: 0x001f },
    reg_default { reg: 0x031d, def: 0xffff },
    reg_default { reg: 0x031e, def: 0x0000 },
    reg_default { reg: 0x031f, def: 0x0000 },
    reg_default { reg: 0x0320, def: 0x0000 },
    reg_default { reg: 0x0321, def: 0x0000 },
    reg_default { reg: 0x0322, def: 0xd7df },
    reg_default { reg: 0x0328, def: 0x10b2 },
    reg_default { reg: 0x0329, def: 0x0175 },
    reg_default { reg: 0x032a, def: 0x36ad },
    reg_default { reg: 0x032b, def: 0x7e55 },
    reg_default { reg: 0x032c, def: 0x0520 },
    reg_default { reg: 0x032d, def: 0xaa00 },
    reg_default { reg: 0x032e, def: 0x570e },
    reg_default { reg: 0x0330, def: 0xe180 },
    reg_default { reg: 0x0332, def: 0x0034 },
    reg_default { reg: 0x0334, def: 0x0001 },
    reg_default { reg: 0x0336, def: 0x0010 },
    reg_default { reg: 0x0338, def: 0x0000 },
    reg_default { reg: 0x04fa, def: 0x0030 },
    reg_default { reg: 0x04fc, def: 0x35c8 },
    reg_default { reg: 0x04fe, def: 0x0800 },
    reg_default { reg: 0x0500, def: 0x0400 },
    reg_default { reg: 0x0502, def: 0x1000 },
    reg_default { reg: 0x0504, def: 0x0000 },
    reg_default { reg: 0x0506, def: 0x04ff },
    reg_default { reg: 0x0508, def: 0x0010 },
    reg_default { reg: 0x050a, def: 0x001a },
    reg_default { reg: 0x0519, def: 0x1c68 },
    reg_default { reg: 0x051a, def: 0x0ccc },
    reg_default { reg: 0x051b, def: 0x0666 },
    reg_default { reg: 0x051d, def: 0x0000 },
    reg_default { reg: 0x051f, def: 0x0000 },
    reg_default { reg: 0x0536, def: 0x061c },
    reg_default { reg: 0x0538, def: 0x0000 },
    reg_default { reg: 0x053a, def: 0x0000 },
    reg_default { reg: 0x053c, def: 0x0000 },
    reg_default { reg: 0x053d, def: 0x0000 },
    reg_default { reg: 0x053e, def: 0x0000 },
    reg_default { reg: 0x053f, def: 0x0000 },
    reg_default { reg: 0x0540, def: 0x0000 },
    reg_default { reg: 0x0541, def: 0x0000 },
    reg_default { reg: 0x0542, def: 0x0000 },
    reg_default { reg: 0x0543, def: 0x0000 },
    reg_default { reg: 0x0544, def: 0x0000 },
    reg_default { reg: 0x0568, def: 0x0000 },
    reg_default { reg: 0x056a, def: 0x0000 },
    reg_default { reg: 0x1000, def: 0x0040 },
    reg_default { reg: 0x1002, def: 0x5405 },
    reg_default { reg: 0x1006, def: 0x5515 },
    reg_default { reg: 0x1007, def: 0x05f7 },
    reg_default { reg: 0x1009, def: 0x0b0a },
    reg_default { reg: 0x100a, def: 0x00ef },
    reg_default { reg: 0x100d, def: 0x0003 },
    reg_default { reg: 0x1010, def: 0xa433 },
    reg_default { reg: 0x1020, def: 0x0000 },
    reg_default { reg: 0x1200, def: 0x5a01 },
    reg_default { reg: 0x1202, def: 0x6524 },
    reg_default { reg: 0x1204, def: 0x1f00 },
    reg_default { reg: 0x1206, def: 0x0000 },
    reg_default { reg: 0x1208, def: 0x0000 },
    reg_default { reg: 0x120a, def: 0x0000 },
    reg_default { reg: 0x120c, def: 0x0000 },
    reg_default { reg: 0x120e, def: 0x0000 },
    reg_default { reg: 0x1210, def: 0x0000 },
    reg_default { reg: 0x1212, def: 0x0000 },
    reg_default { reg: 0x1300, def: 0x10a1 },
    reg_default { reg: 0x1302, def: 0x12ff },
    reg_default { reg: 0x1304, def: 0x0400 },
    reg_default { reg: 0x1305, def: 0x0844 },
    reg_default { reg: 0x1306, def: 0x4611 },
    reg_default { reg: 0x1308, def: 0x555e },
    reg_default { reg: 0x130a, def: 0x0000 },
    reg_default { reg: 0x130c, def: 0x2000 },
    reg_default { reg: 0x130e, def: 0x0100 },
    reg_default { reg: 0x130f, def: 0x0001 },
    reg_default { reg: 0x1310, def: 0x0000 },
    reg_default { reg: 0x1312, def: 0x0000 },
    reg_default { reg: 0x1314, def: 0x0000 },
    reg_default { reg: 0x1316, def: 0x0000 },
    reg_default { reg: 0x1318, def: 0x0000 },
    reg_default { reg: 0x131a, def: 0x0000 },
    reg_default { reg: 0x1322, def: 0x0029 },
    reg_default { reg: 0x1323, def: 0x4a52 },
    reg_default { reg: 0x1324, def: 0x002c },
    reg_default { reg: 0x1325, def: 0x0b02 },
    reg_default { reg: 0x1326, def: 0x002d },
    reg_default { reg: 0x1327, def: 0x6b5a },
    reg_default { reg: 0x1328, def: 0x002e },
    reg_default { reg: 0x1329, def: 0xcbb2 },
    reg_default { reg: 0x132a, def: 0x0030 },
    reg_default { reg: 0x132b, def: 0x2c0b },
    reg_default { reg: 0x1330, def: 0x0031 },
    reg_default { reg: 0x1331, def: 0x8c63 },
    reg_default { reg: 0x1332, def: 0x0032 },
    reg_default { reg: 0x1333, def: 0xecbb },
    reg_default { reg: 0x1334, def: 0x0034 },
    reg_default { reg: 0x1335, def: 0x4d13 },
    reg_default { reg: 0x1336, def: 0x0037 },
    reg_default { reg: 0x1337, def: 0x0dc3 },
    reg_default { reg: 0x1338, def: 0x003d },
    reg_default { reg: 0x1339, def: 0xef7b },
    reg_default { reg: 0x133a, def: 0x0044 },
    reg_default { reg: 0x133b, def: 0xd134 },
    reg_default { reg: 0x133c, def: 0x0047 },
    reg_default { reg: 0x133d, def: 0x91e4 },
    reg_default { reg: 0x133e, def: 0x004d },
    reg_default { reg: 0x133f, def: 0xc370 },
    reg_default { reg: 0x1340, def: 0x0053 },
    reg_default { reg: 0x1341, def: 0xf4fd },
    reg_default { reg: 0x1342, def: 0x0060 },
    reg_default { reg: 0x1343, def: 0x5816 },
    reg_default { reg: 0x1344, def: 0x006c },
    reg_default { reg: 0x1345, def: 0xbb2e },
    reg_default { reg: 0x1346, def: 0x0072 },
    reg_default { reg: 0x1347, def: 0xecbb },
    reg_default { reg: 0x1348, def: 0x0076 },
    reg_default { reg: 0x1349, def: 0x5d97 },
];

unsafe extern "C" fn rt1015_volatile_register(_dev: *mut device, reg: c_uint) -> bool_ {
    unsafe {
        match reg {
            x if x == RT1015_RESET ||
                x == RT1015_CLK_DET ||
                x == RT1015_SIL_DET ||
                x == RT1015_VER_ID ||
                x == RT1015_VENDOR_ID ||
                x == RT1015_DEVICE_ID ||
                x == RT1015_PRO_ALT ||
                x == RT1015_MAN_I2C ||
                x == RT1015_DAC3 ||
                x == RT1015_VBAT_TEST_OUT1 ||
                x == RT1015_VBAT_TEST_OUT2 ||
                x == RT1015_VBAT_PROT_ATT ||
                x == RT1015_VBAT_DET_CODE ||
                x == RT1015_SMART_BST_CTRL1 ||
                x == RT1015_SPK_DC_DETECT1 ||
                x == RT1015_SPK_DC_DETECT4 ||
                x == RT1015_SPK_DC_DETECT5 ||
                x == RT1015_DC_CALIB_CLSD1 ||
                x == RT1015_DC_CALIB_CLSD5 ||
                x == RT1015_DC_CALIB_CLSD6 ||
                x == RT1015_DC_CALIB_CLSD7 ||
                x == RT1015_DC_CALIB_CLSD8 ||
                x == RT1015_S_BST_TIMING_INTER1 ||
                x == RT1015_OSCK_STA ||
                x == RT1015_MONO_DYNA_CTRL1 ||
                x == RT1015_MONO_DYNA_CTRL5 => true,
            _ => false,
        }
    }
}

unsafe extern "C" fn rt1015_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    unsafe {
        match reg {
            x if x == RT1015_RESET ||
                x == RT1015_CLK2 ||
                x == RT1015_CLK3 ||
                x == RT1015_PLL1 ||
                x == RT1015_PLL2 ||
                x == RT1015_DUM_RW1 ||
                x == RT1015_DUM_RW2 ||
                x == RT1015_DUM_RW3 ||
                x == RT1015_DUM_RW4 ||
                x == RT1015_DUM_RW5 ||
                x == RT1015_DUM_RW6 ||
                x == RT1015_CLK_DET ||
                x == RT1015_SIL_DET ||
                x == RT1015_CUSTOMER_ID ||
                x == RT1015_PCODE_FWVER ||
                x == RT1015_VER_ID ||
                x == RT1015_VENDOR_ID ||
                x == RT1015_DEVICE_ID ||
                x == RT1015_PAD_DRV1 ||
                x == RT1015_PAD_DRV2 ||
                x == RT1015_GAT_BOOST ||
                x == RT1015_PRO_ALT ||
                x == RT1015_OSCK_STA ||
                x == RT1015_MAN_I2C ||
                x == RT1015_DAC1 ||
                x == RT1015_DAC2 ||
                x == RT1015_DAC3 ||
                x == RT1015_ADC1 ||
                x == RT1015_ADC2 ||
                x == RT1015_TDM_MASTER ||
                x == RT1015_TDM_TCON ||
                x == RT1015_TDM1_1 ||
                x == RT1015_TDM1_2 ||
                x == RT1015_TDM1_3 ||
                x == RT1015_TDM1_4 ||
                x == RT1015_TDM1_5 ||
                x == RT1015_MIXER1 ||
                x == RT1015_MIXER2 ||
                x == RT1015_ANA_PROTECT1 ||
                x == RT1015_ANA_CTRL_SEQ1 ||
                x == RT1015_ANA_CTRL_SEQ2 ||
                x == RT1015_VBAT_DET_DEB ||
                x == RT1015_VBAT_VOLT_DET1 ||
                x == RT1015_VBAT_VOLT_DET2 ||
                x == RT1015_VBAT_TEST_OUT1 ||
                x == RT1015_VBAT_TEST_OUT2 ||
                x == RT1015_VBAT_PROT_ATT ||
                x == RT1015_VBAT_DET_CODE ||
                x == RT1015_PWR1 ||
                x == RT1015_PWR4 ||
                x == RT1015_PWR5 ||
                x == RT1015_PWR6 ||
                x == RT1015_PWR7 ||
                x == RT1015_PWR8 ||
                x == RT1015_PWR9 ||
                x == RT1015_CLASSD_SEQ ||
                x == RT1015_SMART_BST_CTRL1 ||
                x == RT1015_SMART_BST_CTRL2 ||
                x == RT1015_ANA_CTRL1 ||
                x == RT1015_ANA_CTRL2 ||
                x == RT1015_PWR_STATE_CTRL ||
                x == RT1015_MONO_DYNA_CTRL ||
                x == RT1015_MONO_DYNA_CTRL1 ||
                x == RT1015_MONO_DYNA_CTRL2 ||
                x == RT1015_MONO_DYNA_CTRL3 ||
                x == RT1015_MONO_DYNA_CTRL4 ||
                x == RT1015_MONO_DYNA_CTRL5 ||
                x == RT1015_SPK_VOL ||
                x == RT1015_SHORT_DETTOP1 ||
                x == RT1015_SHORT_DETTOP2 ||
                x == RT1015_SPK_DC_DETECT1 ||
                x == RT1015_SPK_DC_DETECT2 ||
                x == RT1015_SPK_DC_DETECT3 ||
                x == RT1015_SPK_DC_DETECT4 ||
                x == RT1015_SPK_DC_DETECT5 ||
                x == RT1015_BAT_RPO_STEP1 ||
                x == RT1015_BAT_RPO_STEP2 ||
                x == RT1015_BAT_RPO_STEP3 ||
                x == RT1015_BAT_RPO_STEP4 ||
                x == RT1015_BAT_RPO_STEP5 ||
                x == RT1015_BAT_RPO_STEP6 ||
                x == RT1015_BAT_RPO_STEP7 ||
                x == RT1015_BAT_RPO_STEP8 ||
                x == RT1015_BAT_RPO_STEP9 ||
                x == RT1015_BAT_RPO_STEP10 ||
                x == RT1015_BAT_RPO_STEP11 ||
                x == RT1015_BAT_RPO_STEP12 ||
                x == RT1015_SPREAD_SPEC1 ||
                x == RT1015_SPREAD_SPEC2 ||
                x == RT1015_PAD_STATUS ||
                x == RT1015_PADS_PULLING_CTRL1 ||
                x == RT1015_PADS_DRIVING ||
                x == RT1015_SYS_RST1 ||
                x == RT1015_SYS_RST2 ||
                x == RT1015_SYS_GATING1 ||
                x == RT1015_TEST_MODE1 ||
                x == RT1015_TEST_MODE2 ||
                x == RT1015_TIMING_CTRL1 ||
                x == RT1015_PLL_INT ||
                x == RT1015_TEST_OUT1 ||
                x == RT1015_DC_CALIB_CLSD1 ||
                x == RT1015_DC_CALIB_CLSD2 ||
                x == RT1015_DC_CALIB_CLSD3 ||
                x == RT1015_DC_CALIB_CLSD4 ||
                x == RT1015_DC_CALIB_CLSD5 ||
                x == RT1015_DC_CALIB_CLSD6 ||
                x == RT1015_DC_CALIB_CLSD7 ||
                x == RT1015_DC_CALIB_CLSD8 ||
                x == RT1015_DC_CALIB_CLSD9 ||
                x == RT1015_DC_CALIB_CLSD10 ||
                x == RT1015_CLSD_INTERNAL1 ||
                x == RT1015_CLSD_INTERNAL2 ||
                x == RT1015_CLSD_INTERNAL3 ||
                x == RT1015_CLSD_INTERNAL4 ||
                x == RT1015_CLSD_INTERNAL5 ||
                x == RT1015_CLSD_INTERNAL6 ||
                x == RT1015_CLSD_INTERNAL7 ||
                x == RT1015_CLSD_INTERNAL8 ||
                x == RT1015_CLSD_INTERNAL9 ||
                x == RT1015_CLSD_OCP_CTRL ||
                x == RT1015_VREF_LV ||
                x == RT1015_MBIAS1 ||
                x == RT1015_MBIAS2 ||
                x == RT1015_MBIAS3 ||
                x == RT1015_MBIAS4 ||
                x == RT1015_VREF_LV1 ||
                x == RT1015_S_BST_TIMING_INTER1 ||
                x == RT1015_S_BST_TIMING_INTER2 ||
                x == RT1015_S_BST_TIMING_INTER3 ||
                x == RT1015_S_BST_TIMING_INTER4 ||
                x == RT1015_S_BST_TIMING_INTER5 ||
                x == RT1015_S_BST_TIMING_INTER6 ||
                x == RT1015_S_BST_TIMING_INTER7 ||
                x == RT1015_S_BST_TIMING_INTER8 ||
                x == RT1015_S_BST_TIMING_INTER9 ||
                x == RT1015_S_BST_TIMING_INTER10 ||
                x == RT1015_S_BST_TIMING_INTER11 ||
                x == RT1015_S_BST_TIMING_INTER12 ||
                x == RT1015_S_BST_TIMING_INTER13 ||
                x == RT1015_S_BST_TIMING_INTER14 ||
                x == RT1015_S_BST_TIMING_INTER15 ||
                x == RT1015_S_BST_TIMING_INTER16 ||
                x == RT1015_S_BST_TIMING_INTER17 ||
                x == RT1015_S_BST_TIMING_INTER18 ||
                x == RT1015_S_BST_TIMING_INTER19 ||
                x == RT1015_S_BST_TIMING_INTER20 ||
                x == RT1015_S_BST_TIMING_INTER21 ||
                x == RT1015_S_BST_TIMING_INTER22 ||
                x == RT1015_S_BST_TIMING_INTER23 ||
                x == RT1015_S_BST_TIMING_INTER24 ||
                x == RT1015_S_BST_TIMING_INTER25 ||
                x == RT1015_S_BST_TIMING_INTER26 ||
                x == RT1015_S_BST_TIMING_INTER27 ||
                x == RT1015_S_BST_TIMING_INTER28 ||
                x == RT1015_S_BST_TIMING_INTER29 ||
                x == RT1015_S_BST_TIMING_INTER30 ||
                x == RT1015_S_BST_TIMING_INTER31 ||
                x == RT1015_S_BST_TIMING_INTER32 ||
                x == RT1015_S_BST_TIMING_INTER33 ||
                x == RT1015_S_BST_TIMING_INTER34 ||
                x == RT1015_S_BST_TIMING_INTER35 ||
                x == RT1015_S_BST_TIMING_INTER36 => true,
            _ => false,
        }
    }
}

// static const DECLARE_TLV_DB_SCALE(dac_vol_tlv, -9525, 75, 0);
static dac_vol_tlv: [c_uint; 4] = [0, (-9525i32) as c_uint, 75, 0];

static rt1015_din_source_select: [*const c_char; 3] = [
    b"Left\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"Left + Right average\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(rt1015_mono_lr_sel, RT1015_PAD_DRV2, 4,
//     rt1015_din_source_select);
static rt1015_mono_lr_sel: c_uint = 0;

static rt1015_boost_mode: [*const c_char; 3] = [
    b"Bypass\0".as_ptr() as *const c_char,
    b"Adaptive\0".as_ptr() as *const c_char,
    b"Fixed Adaptive\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(rt1015_boost_mode_enum, 0, 0,
//     rt1015_boost_mode);
static rt1015_boost_mode_enum: c_uint = 0;

unsafe extern "C" fn rt1015_boost_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        (*ucontrol).value.integer.value[0] = (*rt1015).boost_mode as c_long;

        0
    }
}

unsafe extern "C" fn rt1015_boost_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;
        let boost_mode = (*ucontrol).value.integer.value[0] as c_int;

        match boost_mode {
            x if x == BYPASS => {
                snd_soc_component_update_bits(
                    component,
                    RT1015_SMART_BST_CTRL1,
                    RT1015_ABST_AUTO_EN_MASK | RT1015_ABST_FIX_TGT_MASK | RT1015_BYPASS_SWR_REG_MASK,
                    RT1015_ABST_REG_MODE | RT1015_ABST_FIX_TGT_DIS | RT1015_BYPASS_SWRREG_BYPASS,
                );
            }
            x if x == ADAPTIVE => {
                snd_soc_component_update_bits(
                    component,
                    RT1015_SMART_BST_CTRL1,
                    RT1015_ABST_AUTO_EN_MASK | RT1015_ABST_FIX_TGT_MASK | RT1015_BYPASS_SWR_REG_MASK,
                    RT1015_ABST_AUTO_MODE | RT1015_ABST_FIX_TGT_DIS | RT1015_BYPASS_SWRREG_PASS,
                );
            }
            x if x == FIXED_ADAPTIVE => {
                snd_soc_component_update_bits(
                    component,
                    RT1015_SMART_BST_CTRL1,
                    RT1015_ABST_AUTO_EN_MASK | RT1015_ABST_FIX_TGT_MASK | RT1015_BYPASS_SWR_REG_MASK,
                    RT1015_ABST_AUTO_MODE | RT1015_ABST_FIX_TGT_EN | RT1015_BYPASS_SWRREG_PASS,
                );
            }
            _ => {
                dev_err((*component).dev, b"Unknown boost control.\n\0".as_ptr() as *const c_char);
                return -EINVAL;
            }
        }

        (*rt1015).boost_mode = boost_mode;

        0
    }
}

unsafe extern "C" fn rt1015_bypass_boost_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        (*ucontrol).value.integer.value[0] = (*rt1015).bypass_boost as c_long;

        0
    }
}

unsafe fn rt1015_calibrate(rt1015: *mut rt1015_priv) {
    unsafe {
        let component = (*rt1015).component;
        let dapm = snd_soc_component_to_dapm(component);
        let regmap = (*rt1015).regmap;

        snd_soc_dapm_mutex_lock(dapm);
        regcache_cache_bypass(regmap, true);

        regmap_write(regmap, RT1015_CLK_DET, 0x0000);
        regmap_write(regmap, RT1015_PWR4, 0x00B2);
        regmap_write(regmap, RT1015_PWR_STATE_CTRL, 0x0009);
        msleep(100);
        regmap_write(regmap, RT1015_PWR_STATE_CTRL, 0x000A);
        msleep(100);
        regmap_write(regmap, RT1015_PWR_STATE_CTRL, 0x000C);
        msleep(100);
        regmap_write(regmap, RT1015_CLSD_INTERNAL8, 0x2028);
        regmap_write(regmap, RT1015_CLSD_INTERNAL9, 0x0140);
        regmap_write(regmap, RT1015_PWR_STATE_CTRL, 0x000D);
        msleep(300);
        regmap_write(regmap, RT1015_PWR_STATE_CTRL, 0x0008);
        regmap_write(regmap, RT1015_SYS_RST1, 0x05F5);
        regmap_write(regmap, RT1015_CLK_DET, 0x8000);

        regcache_cache_bypass(regmap, false);
        regcache_mark_dirty(regmap);
        regcache_sync(regmap);
        snd_soc_dapm_mutex_unlock(dapm);
    }
}

unsafe extern "C" fn rt1015_bypass_boost_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let component = snd_kcontrol_chip(kcontrol);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        if (*rt1015).dac_is_used != 0 {
            dev_err((*component).dev, b"DAC is being used!\n\0".as_ptr() as *const c_char);
            return -EBUSY;
        }

        (*rt1015).bypass_boost = (*ucontrol).value.integer.value[0] as c_int;
        if (*rt1015).bypass_boost == RT1015_Bypass_Boost && (*rt1015).cali_done == 0 {
            rt1015_calibrate(rt1015);
            (*rt1015).cali_done = 1;

            regmap_write((*rt1015).regmap, RT1015_MONO_DYNA_CTRL, 0x0010);
        }

        0
    }
}

static rt1015_dac_output_vol_select: [*const c_char; 4] = [
    b"immediate\0".as_ptr() as *const c_char,
    b"zero detection + immediate change\0".as_ptr() as *const c_char,
    b"zero detection + inc/dec change\0".as_ptr() as *const c_char,
    b"zero detection + soft inc/dec change\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(rt1015_dac_vol_ctl_enum,
//     RT1015_DAC3, 2, rt1015_dac_output_vol_select);
static rt1015_dac_vol_ctl_enum: c_uint = 0;

// static const struct snd_kcontrol_new rt1015_snd_controls[] = { ... };
// ALSA SOC_* initializer macros require external kernel macro expansion.
static rt1015_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn rt1015_is_sys_clk_from_pll(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*source).dapm);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        if (*rt1015).sysclk_src == RT1015_SCLK_S_PLL {
            1
        } else {
            0
        }
    }
}

unsafe extern "C" fn r1015_dac_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        match event {
            SND_SOC_DAPM_PRE_PMU => {
                (*rt1015).dac_is_used = 1;
                if (*rt1015).bypass_boost == RT1015_Enable_Boost {
                    snd_soc_component_write(component, RT1015_SYS_RST1, 0x05f7);
                    snd_soc_component_write(component, RT1015_SYS_RST2, 0x0b0a);
                    snd_soc_component_write(component, RT1015_GAT_BOOST, 0xacfe);
                    snd_soc_component_write(component, RT1015_PWR9, 0xaa00);
                    snd_soc_component_write(component, RT1015_GAT_BOOST, 0xecfe);
                } else {
                    snd_soc_component_write(component, 0x032d, 0xaa60);
                    snd_soc_component_write(component, RT1015_SYS_RST1, 0x05f7);
                    snd_soc_component_write(component, RT1015_SYS_RST2, 0x0b0a);
                    snd_soc_component_write(component, RT1015_PWR_STATE_CTRL, 0x008e);
                }
            }
            SND_SOC_DAPM_POST_PMD => {
                if (*rt1015).bypass_boost == RT1015_Enable_Boost {
                    snd_soc_component_write(component, RT1015_PWR9, 0xa800);
                    snd_soc_component_write(component, RT1015_SYS_RST1, 0x05f5);
                    snd_soc_component_write(component, RT1015_SYS_RST2, 0x0b9a);
                } else {
                    snd_soc_component_write(component, 0x032d, 0xaa60);
                    snd_soc_component_write(component, RT1015_PWR_STATE_CTRL, 0x0088);
                    snd_soc_component_write(component, RT1015_SYS_RST1, 0x05f5);
                    snd_soc_component_write(component, RT1015_SYS_RST2, 0x0b9a);
                }
                (*rt1015).dac_is_used = 0;
            }
            _ => {}
        }
        0
    }
}

unsafe extern "C" fn rt1015_amp_drv_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;
        let ret: c_uint;
        let ret2: c_uint;

        match event {
            SND_SOC_DAPM_PRE_PMU => {
                ret = snd_soc_component_read(component, RT1015_CLK_DET);
                ret2 = snd_soc_component_read(component, RT1015_SPK_DC_DETECT1);
                if ((ret >> 15) & 0x1) == 0 {
                    snd_soc_component_update_bits(component, RT1015_CLK_DET, RT1015_EN_BCLK_DET_MASK, RT1015_EN_BCLK_DET);
                    dev_dbg((*component).dev, b"BCLK Detection Enabled.\n\0".as_ptr() as *const c_char);
                }
                if ((ret2 >> 12) & 0x1) == 0 {
                    snd_soc_component_update_bits(component, RT1015_SPK_DC_DETECT1, RT1015_EN_CLA_D_DC_DET_MASK, RT1015_EN_CLA_D_DC_DET);
                    dev_dbg((*component).dev, b"Class-D DC Detection Enabled.\n\0".as_ptr() as *const c_char);
                }
            }
            SND_SOC_DAPM_POST_PMU => {
                msleep((*rt1015).pdata.power_up_delay_ms);
            }
            _ => {}
        }
        0
    }
}

// static const struct snd_soc_dapm_widget rt1015_dapm_widgets[] = { ... };
// SND_SOC_DAPM_* initializers are preserved by callback and route translations below.
static rt1015_dapm_widgets: [snd_soc_dapm_widget_item; 0] = [];

static rt1015_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AIFRX\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLL\0".as_ptr() as *const c_char, connected: Some(rt1015_is_sys_clk_from_pll) },
    snd_soc_dapm_route { sink: b"Amp Drv\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DAC\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"SPO\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Amp Drv\0".as_ptr() as *const c_char, connected: None },
];

unsafe extern "C" fn rt1015_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let component = (*dai).component;
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;
        let pre_div: c_int;
        let frame_size: c_int;
        let lrck: c_int;
        let mut val_len: c_uint = 0;

        lrck = params_rate(params);
        pre_div = rl6231_get_clk_info((*rt1015).sysclk, lrck);
        if pre_div < 0 {
            dev_err((*component).dev, b"Unsupported clock rate\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        frame_size = snd_soc_params_to_frame_size(params);
        if frame_size < 0 {
            dev_err((*component).dev, b"Unsupported frame size: %d\n\0".as_ptr() as *const c_char, frame_size);
            return -EINVAL;
        }

        dev_dbg((*component).dev, b"pre_div is %d for iis %d\n\0".as_ptr() as *const c_char, pre_div, (*dai).id);

        dev_dbg((*component).dev, b"lrck is %dHz and pre_div is %d for iis %d\n\0".as_ptr() as *const c_char, lrck, pre_div, (*dai).id);

        match params_width(params) {
            16 => {}
            20 => val_len = RT1015_I2S_DL_20,
            24 => val_len = RT1015_I2S_DL_24,
            8 => val_len = RT1015_I2S_DL_8,
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(component, RT1015_TDM_MASTER, RT1015_I2S_DL_MASK, val_len);
        snd_soc_component_update_bits(component, RT1015_CLK2, RT1015_FS_PD_MASK, (pre_div as c_uint) << RT1015_FS_PD_SFT);

        0
    }
}

unsafe extern "C" fn rt1015_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut reg_val: c_uint = 0;
        let mut reg_val2: c_uint = 0;

        match fmt & SND_SOC_DAIFMT_MASTER_MASK {
            SND_SOC_DAIFMT_CBP_CFP => reg_val |= RT1015_TCON_TDM_MS_M,
            SND_SOC_DAIFMT_CBC_CFC => reg_val |= RT1015_TCON_TDM_MS_S,
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_IB_NF => reg_val2 |= RT1015_TDM_INV_BCLK,
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            SND_SOC_DAIFMT_I2S => {}
            SND_SOC_DAIFMT_LEFT_J => reg_val |= RT1015_I2S_M_DF_LEFT,
            SND_SOC_DAIFMT_DSP_A => reg_val |= RT1015_I2S_M_DF_PCM_A,
            SND_SOC_DAIFMT_DSP_B => reg_val |= RT1015_I2S_M_DF_PCM_B,
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(component, RT1015_TDM_MASTER, RT1015_TCON_TDM_MS_MASK | RT1015_I2S_M_DF_MASK, reg_val);
        snd_soc_component_update_bits(component, RT1015_TDM1_1, RT1015_TDM_INV_BCLK_MASK, reg_val2);

        0
    }
}

unsafe extern "C" fn rt1015_set_component_sysclk(component: *mut snd_soc_component, clk_id: c_int, _source: c_int, freq: c_uint, _dir: c_int) -> c_int {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;
        let mut reg_val: c_uint = 0;

        if freq == (*rt1015).sysclk && clk_id == (*rt1015).sysclk_src {
            return 0;
        }

        if clk_id == RT1015_SCLK_S_MCLK {
            reg_val |= RT1015_CLK_SYS_PRE_SEL_MCLK;
        } else if clk_id == RT1015_SCLK_S_PLL {
            reg_val |= RT1015_CLK_SYS_PRE_SEL_PLL;
        } else {
            dev_err((*component).dev, b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char, clk_id);
            return -EINVAL;
        }

        (*rt1015).sysclk = freq;
        (*rt1015).sysclk_src = clk_id;

        dev_dbg((*component).dev, b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char, freq, clk_id);

        snd_soc_component_update_bits(component, RT1015_CLK2, RT1015_CLK_SYS_PRE_SEL_MASK, reg_val);

        0
    }
}

unsafe extern "C" fn rt1015_set_component_pll(component: *mut snd_soc_component, _pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;
        let mut pll_code = rl6231_pll_code { m_bp: 0, m_code: 0, n_code: 0, k_code: 0 };
        let ret: c_int;

        if freq_in == 0 || freq_out == 0 {
            dev_dbg((*component).dev, b"PLL disabled\n\0".as_ptr() as *const c_char);

            (*rt1015).pll_in = 0;
            (*rt1015).pll_out = 0;

            return 0;
        }

        if source == (*rt1015).pll_src && freq_in == (*rt1015).pll_in && freq_out == (*rt1015).pll_out {
            return 0;
        }

        if source == RT1015_PLL_S_MCLK {
            snd_soc_component_update_bits(component, RT1015_CLK2, RT1015_PLL_SEL_MASK, RT1015_PLL_SEL_PLL_SRC2);
        } else if source == RT1015_PLL_S_BCLK {
            snd_soc_component_update_bits(component, RT1015_CLK2, RT1015_PLL_SEL_MASK, RT1015_PLL_SEL_BCLK);
        } else {
            dev_err((*component).dev, b"Unknown PLL Source %d\n\0".as_ptr() as *const c_char, source);
            return -EINVAL;
        }

        ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
        if ret < 0 {
            dev_err((*component).dev, b"Unsupported input clock %d\n\0".as_ptr() as *const c_char, freq_in);
            return ret;
        }

        dev_dbg(
            (*component).dev,
            b"bypass=%d m=%d n=%d k=%d\n\0".as_ptr() as *const c_char,
            pll_code.m_bp,
            if pll_code.m_bp != 0 { 0 } else { pll_code.m_code },
            pll_code.n_code,
            pll_code.k_code,
        );

        snd_soc_component_write(
            component,
            RT1015_PLL1,
            (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint) << RT1015_PLL_M_SFT) |
                ((pll_code.m_bp as c_uint) << RT1015_PLL_M_BP_SFT) |
                pll_code.n_code as c_uint,
        );
        snd_soc_component_write(component, RT1015_PLL2, pll_code.k_code as c_uint);

        (*rt1015).pll_in = freq_in;
        (*rt1015).pll_out = freq_out;
        (*rt1015).pll_src = source;

        0
    }
}

unsafe extern "C" fn rt1015_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut val: c_uint = 0;
        let rx_slotnum: c_uint;
        let tx_slotnum: c_uint;
        let mut ret: c_int = 0;
        let first_bit: c_int;

        match slots {
            2 => val |= RT1015_I2S_TX_2CH,
            4 => val |= RT1015_I2S_TX_4CH,
            6 => val |= RT1015_I2S_TX_6CH,
            8 => val |= RT1015_I2S_TX_8CH,
            _ => {
                ret = -EINVAL;
                return ret;
            }
        }

        match slot_width {
            16 => val |= RT1015_I2S_CH_TX_LEN_16B,
            20 => val |= RT1015_I2S_CH_TX_LEN_20B,
            24 => val |= RT1015_I2S_CH_TX_LEN_24B,
            32 => val |= RT1015_I2S_CH_TX_LEN_32B,
            _ => {
                ret = -EINVAL;
                return ret;
            }
        }

        /* Rx slot configuration */
        rx_slotnum = hweight_long(rx_mask as c_ulong);
        if rx_slotnum != 1 {
            ret = -EINVAL;
            dev_err((*component).dev, b"too many rx slots or zero slot\n\0".as_ptr() as *const c_char);
            return ret;
        }

        /* This is an assumption that the system sends stereo audio to the amplifier typically.
         * And the stereo audio is placed in slot 0/2/4/6 as the starting slot.
         * The users could select the channel from L/R/L+R by "Mono LR Select" control.
         */
        first_bit = __ffs(rx_mask as c_ulong);
        match first_bit {
            0 | 2 | 4 | 6 => {
                snd_soc_component_update_bits(
                    component,
                    RT1015_TDM1_4,
                    RT1015_TDM_I2S_TX_L_DAC1_1_MASK | RT1015_TDM_I2S_TX_R_DAC1_1_MASK,
                    ((first_bit as c_uint) << RT1015_TDM_I2S_TX_L_DAC1_1_SFT) |
                        (((first_bit + 1) as c_uint) << RT1015_TDM_I2S_TX_R_DAC1_1_SFT),
                );
            }
            1 | 3 | 5 | 7 => {
                snd_soc_component_update_bits(
                    component,
                    RT1015_TDM1_4,
                    RT1015_TDM_I2S_TX_L_DAC1_1_MASK | RT1015_TDM_I2S_TX_R_DAC1_1_MASK,
                    (((first_bit - 1) as c_uint) << RT1015_TDM_I2S_TX_L_DAC1_1_SFT) |
                        ((first_bit as c_uint) << RT1015_TDM_I2S_TX_R_DAC1_1_SFT),
                );
            }
            _ => {
                ret = -EINVAL;
                return ret;
            }
        }

        /* Tx slot configuration */
        tx_slotnum = hweight_long(tx_mask as c_ulong);
        if tx_slotnum != 0 {
            ret = -EINVAL;
            dev_err((*component).dev, b"doesn't need to support tx slots\n\0".as_ptr() as *const c_char);
            return ret;
        }

        snd_soc_component_update_bits(
            component,
            RT1015_TDM1_1,
            RT1015_I2S_CH_TX_MASK | RT1015_I2S_CH_RX_MASK | RT1015_I2S_CH_TX_LEN_MASK | RT1015_I2S_CH_RX_LEN_MASK,
            val,
        );

        ret
    }
}

unsafe extern "C" fn rt1015_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        (*rt1015).component = component;

        0
    }
}

unsafe extern "C" fn rt1015_remove(component: *mut snd_soc_component) {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        regmap_write((*rt1015).regmap, RT1015_RESET, 0);
    }
}

const RT1015_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const RT1015_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

static rt1015_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1015_hw_params),
    set_fmt: Some(rt1015_set_dai_fmt),
    set_tdm_slot: Some(rt1015_set_tdm_slot),
};

static mut rt1015_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"rt1015-aif\0".as_ptr() as *const c_char,
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: b"AIF Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 4,
            rates: RT1015_STEREO_RATES,
            formats: RT1015_FORMATS,
        },
        ops: &rt1015_aif_dai_ops,
    },
];

// CONFIG_PM
unsafe extern "C" fn rt1015_suspend(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        regcache_cache_only((*rt1015).regmap, true);
        regcache_mark_dirty((*rt1015).regmap);

        0
    }
}

unsafe extern "C" fn rt1015_resume(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let rt1015 = snd_soc_component_get_drvdata(component) as *mut rt1015_priv;

        regcache_cache_only((*rt1015).regmap, false);
        regcache_sync((*rt1015).regmap);

        if (*rt1015).cali_done != 0 {
            rt1015_calibrate(rt1015);
        }

        0
    }
}

static soc_component_dev_rt1015: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1015_probe),
    remove: Some(rt1015_remove),
    suspend: Some(rt1015_suspend),
    resume: Some(rt1015_resume),
    controls: rt1015_snd_controls.as_ptr(),
    num_controls: array_size(&rt1015_snd_controls),
    dapm_widgets: rt1015_dapm_widgets.as_ptr(),
    num_dapm_widgets: array_size(&rt1015_dapm_widgets),
    dapm_routes: rt1015_dapm_routes.as_ptr(),
    num_dapm_routes: array_size(&rt1015_dapm_routes),
    set_sysclk: Some(rt1015_set_component_sysclk),
    set_pll: Some(rt1015_set_component_pll),
    use_pmdown_time: 1,
    endianness: 1,
};

static rt1015_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 16,
    max_register: unsafe { RT1015_S_BST_TIMING_INTER36 },
    volatile_reg: Some(rt1015_volatile_register),
    readable_reg: Some(rt1015_readable_register),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: rt1015_reg.as_ptr(),
    num_reg_defaults: array_size(&rt1015_reg),
};

static rt1015_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: [b'r' as c_char, b't' as c_char, b'1' as c_char, b'0' as c_char, b'1' as c_char, b'5' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, rt1015_i2c_id);

// CONFIG_OF
static rt1015_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"realtek,rt1015\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, rt1015_of_match);

// CONFIG_ACPI
static rt1015_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: [b'1' as c_char, b'0' as c_char, b'E' as c_char, b'C' as c_char, b'1' as c_char, b'0' as c_char, b'1' as c_char, b'5' as c_char, 0] },
    acpi_device_id { id: [0; 9] },
];
// MODULE_DEVICE_TABLE(acpi, rt1015_acpi_match);

unsafe fn rt1015_parse_dt(rt1015: *mut rt1015_priv, dev: *mut device) {
    unsafe {
        device_property_read_u32(
            dev,
            b"realtek,power-up-delay-ms\0".as_ptr() as *const c_char,
            &mut (*rt1015).pdata.power_up_delay_ms,
        );
    }
}

unsafe extern "C" fn rt1015_i2c_probe(i2c: *mut i2c_client) -> c_int {
    unsafe {
        let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut rt1015_platform_data;
        let rt1015: *mut rt1015_priv;
        let mut ret: c_int;
        let mut val: c_uint = 0;

        rt1015 = devm_kzalloc(&mut (*i2c).dev, size_of::<rt1015_priv>(), GFP_KERNEL) as *mut rt1015_priv;
        if rt1015.is_null() {
            return -ENOMEM;
        }

        i2c_set_clientdata(i2c, rt1015 as *mut c_void);

        (*rt1015).pdata = i2s_default_platform_data;

        if !pdata.is_null() {
            (*rt1015).pdata = *pdata;
        } else {
            rt1015_parse_dt(rt1015, &mut (*i2c).dev);
        }

        (*rt1015).regmap = devm_regmap_init_i2c(i2c, &rt1015_regmap);
        if IS_ERR((*rt1015).regmap as *const c_void) {
            ret = PTR_ERR((*rt1015).regmap as *const c_void);
            dev_err(&mut (*i2c).dev, b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = regmap_read((*rt1015).regmap, RT1015_DEVICE_ID, &mut val);
        if ret != 0 {
            dev_err(&mut (*i2c).dev, b"Failed to read device register: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        } else if val != RT1015_DEVICE_ID_VAL && val != RT1015_DEVICE_ID_VAL2 {
            dev_err(&mut (*i2c).dev, b"Device with ID register %x is not rt1015\n\0".as_ptr() as *const c_char, val);
            return -ENODEV;
        }

        devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &soc_component_dev_rt1015,
            rt1015_dai.as_mut_ptr(),
            array_size(&rt1015_dai) as c_int,
        )
    }
}

unsafe extern "C" fn rt1015_i2c_shutdown(client: *mut i2c_client) {
    unsafe {
        let rt1015 = i2c_get_clientdata(client) as *mut rt1015_priv;

        regmap_write((*rt1015).regmap, RT1015_RESET, 0);
    }
}

static mut rt1015_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt1015\0".as_ptr() as *const c_char,
        of_match_table: rt1015_of_match.as_ptr(),
        acpi_match_table: rt1015_acpi_match.as_ptr(),
    },
    probe: Some(rt1015_i2c_probe),
    shutdown: Some(rt1015_i2c_shutdown),
    id_table: rt1015_i2c_id.as_ptr(),
};
// module_i2c_driver(rt1015_i2c_driver);

// MODULE_DESCRIPTION("ASoC RT1015 driver");
// MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
