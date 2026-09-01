// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs53l30.rs  --  CS53l30 ALSA Soc Audio driver
 *
 * Copyright 2015 Cirrus Logic, Inc.
 *
 * Authors: Paul Handrigan <Paul.Handrigan@cirrus.com>,
 *          Tim Howe <Tim.Howe@cirrus.com>
 */

// Translated from Linux C source. Kernel headers and local headers
// ("cs53l30.h", "cirrus_legacy.h") are external dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *const device_node,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}
#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_private,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn msleep(msecs: c_uint);
    fn usleep_range(min: c_uint, max: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_is_active_low(desc: *mut gpio_desc) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(dev: *mut device, num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num: c_int, supplies: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn cirrus_read_device_id(map: *mut regmap, reg: c_uint) -> c_int;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn of_property_read_u8(np: *const device_node, propname: *const c_char, out_value: *mut u8) -> c_int;
    fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn __ffs(word: c_uint) -> c_uint;
}

const CS53L30_NUM_SUPPLIES: usize = 2;
static cs53l30_supply_names: [*const c_char; CS53L30_NUM_SUPPLIES] = [
    b"VA\0".as_ptr() as *const c_char,
    b"VP\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct cs53l30_private {
    pub supplies: [regulator_bulk_data; CS53L30_NUM_SUPPLIES],
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub mute_gpio: *mut gpio_desc,
    pub mclk: *mut clk,
    pub use_sdout2: bool,
    pub mclk_rate: u32,
}

static cs53l30_reg_defaults: [reg_default; 43] = [
    reg_default { reg: CS53L30_PWRCTL, def: CS53L30_PWRCTL_DEFAULT },
    reg_default { reg: CS53L30_MCLKCTL, def: CS53L30_MCLKCTL_DEFAULT },
    reg_default { reg: CS53L30_INT_SR_CTL, def: CS53L30_INT_SR_CTL_DEFAULT },
    reg_default { reg: CS53L30_MICBIAS_CTL, def: CS53L30_MICBIAS_CTL_DEFAULT },
    reg_default { reg: CS53L30_ASPCFG_CTL, def: CS53L30_ASPCFG_CTL_DEFAULT },
    reg_default { reg: CS53L30_ASP_CTL1, def: CS53L30_ASP_CTL1_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_CTL1, def: CS53L30_ASP_TDMTX_CTLx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_CTL2, def: CS53L30_ASP_TDMTX_CTLx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_CTL3, def: CS53L30_ASP_TDMTX_CTLx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_CTL4, def: CS53L30_ASP_TDMTX_CTLx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN1, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN2, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN3, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN4, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN5, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_TDMTX_EN6, def: CS53L30_ASP_TDMTX_ENx_DEFAULT },
    reg_default { reg: CS53L30_ASP_CTL2, def: CS53L30_ASP_CTL2_DEFAULT },
    reg_default { reg: CS53L30_SFT_RAMP, def: CS53L30_SFT_RMP_DEFAULT },
    reg_default { reg: CS53L30_LRCK_CTL1, def: CS53L30_LRCK_CTLx_DEFAULT },
    reg_default { reg: CS53L30_LRCK_CTL2, def: CS53L30_LRCK_CTLx_DEFAULT },
    reg_default { reg: CS53L30_MUTEP_CTL1, def: CS53L30_MUTEP_CTL1_DEFAULT },
    reg_default { reg: CS53L30_MUTEP_CTL2, def: CS53L30_MUTEP_CTL2_DEFAULT },
    reg_default { reg: CS53L30_INBIAS_CTL1, def: CS53L30_INBIAS_CTL1_DEFAULT },
    reg_default { reg: CS53L30_INBIAS_CTL2, def: CS53L30_INBIAS_CTL2_DEFAULT },
    reg_default { reg: CS53L30_DMIC1_STR_CTL, def: CS53L30_DMIC1_STR_CTL_DEFAULT },
    reg_default { reg: CS53L30_DMIC2_STR_CTL, def: CS53L30_DMIC2_STR_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADCDMIC1_CTL1, def: CS53L30_ADCDMICx_CTL1_DEFAULT },
    reg_default { reg: CS53L30_ADCDMIC1_CTL2, def: CS53L30_ADCDMIC1_CTL2_DEFAULT },
    reg_default { reg: CS53L30_ADC1_CTL3, def: CS53L30_ADCx_CTL3_DEFAULT },
    reg_default { reg: CS53L30_ADC1_NG_CTL, def: CS53L30_ADCx_NG_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC1A_AFE_CTL, def: CS53L30_ADCxy_AFE_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC1B_AFE_CTL, def: CS53L30_ADCxy_AFE_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC1A_DIG_VOL, def: CS53L30_ADCxy_DIG_VOL_DEFAULT },
    reg_default { reg: CS53L30_ADC1B_DIG_VOL, def: CS53L30_ADCxy_DIG_VOL_DEFAULT },
    reg_default { reg: CS53L30_ADCDMIC2_CTL1, def: CS53L30_ADCDMICx_CTL1_DEFAULT },
    reg_default { reg: CS53L30_ADCDMIC2_CTL2, def: CS53L30_ADCDMIC1_CTL2_DEFAULT },
    reg_default { reg: CS53L30_ADC2_CTL3, def: CS53L30_ADCx_CTL3_DEFAULT },
    reg_default { reg: CS53L30_ADC2_NG_CTL, def: CS53L30_ADCx_NG_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC2A_AFE_CTL, def: CS53L30_ADCxy_AFE_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC2B_AFE_CTL, def: CS53L30_ADCxy_AFE_CTL_DEFAULT },
    reg_default { reg: CS53L30_ADC2A_DIG_VOL, def: CS53L30_ADCxy_DIG_VOL_DEFAULT },
    reg_default { reg: CS53L30_ADC2B_DIG_VOL, def: CS53L30_ADCxy_DIG_VOL_DEFAULT },
    reg_default { reg: CS53L30_INT_MASK, def: CS53L30_DEVICE_INT_MASK },
];

unsafe extern "C" fn cs53l30_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg == CS53L30_IS { true } else { false }
}

unsafe extern "C" fn cs53l30_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS53L30_DEVID_AB | CS53L30_DEVID_CD | CS53L30_DEVID_E | CS53L30_REVID | CS53L30_IS => false,
        _ => true,
    }
}

unsafe extern "C" fn cs53l30_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS53L30_DEVID_AB | CS53L30_DEVID_CD | CS53L30_DEVID_E | CS53L30_REVID |
        CS53L30_PWRCTL | CS53L30_MCLKCTL | CS53L30_INT_SR_CTL | CS53L30_MICBIAS_CTL |
        CS53L30_ASPCFG_CTL | CS53L30_ASP_CTL1 | CS53L30_ASP_TDMTX_CTL1 |
        CS53L30_ASP_TDMTX_CTL2 | CS53L30_ASP_TDMTX_CTL3 | CS53L30_ASP_TDMTX_CTL4 |
        CS53L30_ASP_TDMTX_EN1 | CS53L30_ASP_TDMTX_EN2 | CS53L30_ASP_TDMTX_EN3 |
        CS53L30_ASP_TDMTX_EN4 | CS53L30_ASP_TDMTX_EN5 | CS53L30_ASP_TDMTX_EN6 |
        CS53L30_ASP_CTL2 | CS53L30_SFT_RAMP | CS53L30_LRCK_CTL1 | CS53L30_LRCK_CTL2 |
        CS53L30_MUTEP_CTL1 | CS53L30_MUTEP_CTL2 | CS53L30_INBIAS_CTL1 |
        CS53L30_INBIAS_CTL2 | CS53L30_DMIC1_STR_CTL | CS53L30_DMIC2_STR_CTL |
        CS53L30_ADCDMIC1_CTL1 | CS53L30_ADCDMIC1_CTL2 | CS53L30_ADC1_CTL3 |
        CS53L30_ADC1_NG_CTL | CS53L30_ADC1A_AFE_CTL | CS53L30_ADC1B_AFE_CTL |
        CS53L30_ADC1A_DIG_VOL | CS53L30_ADC1B_DIG_VOL | CS53L30_ADCDMIC2_CTL1 |
        CS53L30_ADCDMIC2_CTL2 | CS53L30_ADC2_CTL3 | CS53L30_ADC2_NG_CTL |
        CS53L30_ADC2A_AFE_CTL | CS53L30_ADC2B_AFE_CTL | CS53L30_ADC2A_DIG_VOL |
        CS53L30_ADC2B_DIG_VOL | CS53L30_INT_MASK => true,
        _ => false,
    }
}

// static DECLARE_TLV_DB_SCALE(adc_boost_tlv, 0, 2000, 0);
// static DECLARE_TLV_DB_SCALE(adc_ng_boost_tlv, 0, 3000, 0);
// static DECLARE_TLV_DB_SCALE(pga_tlv, -600, 50, 0);
// static DECLARE_TLV_DB_SCALE(dig_tlv, -9600, 100, 1);
// static DECLARE_TLV_DB_SCALE(pga_preamp_tlv, 0, 10000, 0);

static input1_sel_text: [*const c_char; 7] = [
    b"DMIC1 On AB In\0".as_ptr() as *const c_char,
    b"DMIC1 On A In\0".as_ptr() as *const c_char,
    b"DMIC1 On B In\0".as_ptr() as *const c_char,
    b"ADC1 On AB In\0".as_ptr() as *const c_char,
    b"ADC1 On A In\0".as_ptr() as *const c_char,
    b"ADC1 On B In\0".as_ptr() as *const c_char,
    b"DMIC1 Off ADC1 Off\0".as_ptr() as *const c_char,
];

static input1_sel_values: [c_uint; 7] = [
    CS53L30_CH_TYPE,
    CS53L30_ADCxB_PDN | CS53L30_CH_TYPE,
    CS53L30_ADCxA_PDN | CS53L30_CH_TYPE,
    CS53L30_DMICx_PDN,
    CS53L30_ADCxB_PDN | CS53L30_DMICx_PDN,
    CS53L30_ADCxA_PDN | CS53L30_DMICx_PDN,
    CS53L30_ADCxA_PDN | CS53L30_ADCxB_PDN | CS53L30_DMICx_PDN,
];

static input2_sel_text: [*const c_char; 7] = [
    b"DMIC2 On AB In\0".as_ptr() as *const c_char,
    b"DMIC2 On A In\0".as_ptr() as *const c_char,
    b"DMIC2 On B In\0".as_ptr() as *const c_char,
    b"ADC2 On AB In\0".as_ptr() as *const c_char,
    b"ADC2 On A In\0".as_ptr() as *const c_char,
    b"ADC2 On B In\0".as_ptr() as *const c_char,
    b"DMIC2 Off ADC2 Off\0".as_ptr() as *const c_char,
];

static input2_sel_values: [c_uint; 7] = [
    0x0,
    CS53L30_ADCxB_PDN,
    CS53L30_ADCxA_PDN,
    CS53L30_DMICx_PDN,
    CS53L30_ADCxB_PDN | CS53L30_DMICx_PDN,
    CS53L30_ADCxA_PDN | CS53L30_DMICx_PDN,
    CS53L30_ADCxA_PDN | CS53L30_ADCxB_PDN | CS53L30_DMICx_PDN,
];

static input1_route_sel_text: [*const c_char; 2] = [
    b"ADC1_SEL\0".as_ptr() as *const c_char,
    b"DMIC1_SEL\0".as_ptr() as *const c_char,
];

// static const struct soc_enum input1_route_sel_enum =
//     SOC_ENUM_SINGLE(CS53L30_ADCDMIC1_CTL1, CS53L30_CH_TYPE_SHIFT,
//                     ARRAY_SIZE(input1_route_sel_text), input1_route_sel_text);
// static SOC_VALUE_ENUM_SINGLE_DECL(input1_sel_enum, CS53L30_ADCDMIC1_CTL1, 0,
//                                   CS53L30_ADCDMICx_PDN_MASK, input1_sel_text,
//                                   input1_sel_values);
// static const struct snd_kcontrol_new input1_route_sel_mux =
//     SOC_DAPM_ENUM("Input 1 Route", input1_route_sel_enum);

static input2_route_sel_text: [*const c_char; 2] = [
    b"ADC2_SEL\0".as_ptr() as *const c_char,
    b"DMIC2_SEL\0".as_ptr() as *const c_char,
];

// Note: CS53L30_ADCDMIC1_CTL1 CH_TYPE controls inputs 1 and 2
// static const struct soc_enum input2_route_sel_enum =
//     SOC_ENUM_SINGLE(CS53L30_ADCDMIC1_CTL1, 0,
//                     ARRAY_SIZE(input2_route_sel_text), input2_route_sel_text);
// static SOC_VALUE_ENUM_SINGLE_DECL(input2_sel_enum, CS53L30_ADCDMIC2_CTL1, 0,
//                                   CS53L30_ADCDMICx_PDN_MASK, input2_sel_text,
//                                   input2_sel_values);
// static const struct snd_kcontrol_new input2_route_sel_mux =
//     SOC_DAPM_ENUM("Input 2 Route", input2_route_sel_enum);

/*
 * TB = 6144*(MCLK(int) scaling factor)/MCLK(internal)
 * TB - Time base
 * NOTE: If MCLK_INT_SCALE = 0, then TB=1
 */
static cs53l30_ng_delay_text: [*const c_char; 4] = [
    b"TB*50ms\0".as_ptr() as *const c_char,
    b"TB*100ms\0".as_ptr() as *const c_char,
    b"TB*150ms\0".as_ptr() as *const c_char,
    b"TB*200ms\0".as_ptr() as *const c_char,
];

// static const struct soc_enum adc1_ng_delay_enum = SOC_ENUM_SINGLE(...);
// static const struct soc_enum adc2_ng_delay_enum = SOC_ENUM_SINGLE(...);

// The noise gate threshold selected will depend on NG Boost
static cs53l30_ng_thres_text: [*const c_char; 8] = [
    b"-64dB/-34dB\0".as_ptr() as *const c_char,
    b"-66dB/-36dB\0".as_ptr() as *const c_char,
    b"-70dB/-40dB\0".as_ptr() as *const c_char,
    b"-73dB/-43dB\0".as_ptr() as *const c_char,
    b"-76dB/-46dB\0".as_ptr() as *const c_char,
    b"-82dB/-52dB\0".as_ptr() as *const c_char,
    b"-58dB\0".as_ptr() as *const c_char,
    b"-64dB\0".as_ptr() as *const c_char,
];

// static const struct soc_enum adc1_ng_thres_enum = SOC_ENUM_SINGLE(...);
// static const struct soc_enum adc2_ng_thres_enum = SOC_ENUM_SINGLE(...);

// Corner frequencies are with an Fs of 48kHz.
static hpf_corner_freq_text: [*const c_char; 4] = [
    b"1.86Hz\0".as_ptr() as *const c_char,
    b"120Hz\0".as_ptr() as *const c_char,
    b"235Hz\0".as_ptr() as *const c_char,
    b"466Hz\0".as_ptr() as *const c_char,
];

// static const struct soc_enum adc1_hpf_enum = SOC_ENUM_SINGLE(...);
// static const struct soc_enum adc2_hpf_enum = SOC_ENUM_SINGLE(...);
// static const struct snd_kcontrol_new cs53l30_snd_controls[] = { SOC_* controls as in source };
static cs53l30_snd_controls: [snd_kcontrol_new; 0] = [];
// static const struct snd_soc_dapm_widget cs53l30_dapm_widgets[] = { SND_SOC_DAPM_* widgets as in source };
static cs53l30_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static cs53l30_dapm_routes: [snd_soc_dapm_route; 16] = [
    snd_soc_dapm_route { sink: b"ADC1A\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN1_DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux 1\0".as_ptr() as *const c_char, control: b"ADC1_SEL\0".as_ptr() as *const c_char, source: b"ADC1A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC1B\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2A\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN3_DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux 2\0".as_ptr() as *const c_char, control: b"ADC2_SEL\0".as_ptr() as *const c_char, source: b"ADC2A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2B\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC1A\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIC1 Bias\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC1B\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIC2 Bias\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2A\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIC3 Bias\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC2B\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIC4 Bias\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN1_DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux 1\0".as_ptr() as *const c_char, control: b"DMIC1_SEL\0".as_ptr() as *const c_char, source: b"DMIC1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"IN3_DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux 2\0".as_ptr() as *const c_char, control: b"DMIC2_SEL\0".as_ptr() as *const c_char, source: b"DMIC2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

static cs53l30_dapm_routes_sdout1: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC1A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Input Mux 1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC1B\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC2A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Input Mux 2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC2B\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASP_SDOUT1\0".as_ptr() as *const c_char },
];

static cs53l30_dapm_routes_sdout2: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC1A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Input Mux 1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT1\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC1B\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC2A\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Input Mux 2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ASP_SDOUT2\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ADC2B\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASP_SDOUT1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"ASP_SDOUT2\0".as_ptr() as *const c_char },
];

#[repr(C)]
pub struct cs53l30_mclk_div {
    pub mclk_rate: u32,
    pub srate: u32,
    pub asp_rate: u8,
    pub internal_fs_ratio: u8,
    pub mclk_int_scale: u8,
}

static cs53l30_mclk_coeffs: [cs53l30_mclk_div; 34] = [
    cs53l30_mclk_div { mclk_rate: 5644800, srate: 11025, asp_rate: 0x4, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 5644800, srate: 22050, asp_rate: 0x8, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 5644800, srate: 44100, asp_rate: 0xC, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 8000, asp_rate: 0x1, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 11025, asp_rate: 0x2, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 12000, asp_rate: 0x4, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 16000, asp_rate: 0x5, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 22050, asp_rate: 0x6, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 24000, asp_rate: 0x8, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 32000, asp_rate: 0x9, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 44100, asp_rate: 0xA, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6000000, srate: 48000, asp_rate: 0xC, internal_fs_ratio: 0, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 8000, asp_rate: 0x1, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 11025, asp_rate: 0x2, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 12000, asp_rate: 0x4, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 16000, asp_rate: 0x5, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 22050, asp_rate: 0x6, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 24000, asp_rate: 0x8, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 32000, asp_rate: 0x9, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 44100, asp_rate: 0xA, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6144000, srate: 48000, asp_rate: 0xC, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 8000, asp_rate: 0x1, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 11025, asp_rate: 0x2, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 12000, asp_rate: 0x4, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 16000, asp_rate: 0x5, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 22050, asp_rate: 0x6, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 24000, asp_rate: 0x8, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 32000, asp_rate: 0x9, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 44100, asp_rate: 0xA, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 6400000, srate: 48000, asp_rate: 0xC, internal_fs_ratio: CS53L30_INTRNL_FS_RATIO as u8, mclk_int_scale: CS53L30_MCLK_INT_SCALE as u8 },
    cs53l30_mclk_div { mclk_rate: 0, srate: 0, asp_rate: 0, internal_fs_ratio: 0, mclk_int_scale: 0 },
    cs53l30_mclk_div { mclk_rate: 0, srate: 0, asp_rate: 0, internal_fs_ratio: 0, mclk_int_scale: 0 },
    cs53l30_mclk_div { mclk_rate: 0, srate: 0, asp_rate: 0, internal_fs_ratio: 0, mclk_int_scale: 0 },
    cs53l30_mclk_div { mclk_rate: 0, srate: 0, asp_rate: 0, internal_fs_ratio: 0, mclk_int_scale: 0 },
];

#[repr(C)]
pub struct cs53l30_mclkx_div {
    pub mclkx: u32,
    pub ratio: u8,
    pub mclkdiv: u8,
}

static cs53l30_mclkx_coeffs: [cs53l30_mclkx_div; 7] = [
    cs53l30_mclkx_div { mclkx: 5644800, ratio: 1, mclkdiv: CS53L30_MCLK_DIV_BY_1 as u8 },
    cs53l30_mclkx_div { mclkx: 6000000, ratio: 1, mclkdiv: CS53L30_MCLK_DIV_BY_1 as u8 },
    cs53l30_mclkx_div { mclkx: 6144000, ratio: 1, mclkdiv: CS53L30_MCLK_DIV_BY_1 as u8 },
    cs53l30_mclkx_div { mclkx: 11289600, ratio: 2, mclkdiv: CS53L30_MCLK_DIV_BY_2 as u8 },
    cs53l30_mclkx_div { mclkx: 12288000, ratio: 2, mclkdiv: CS53L30_MCLK_DIV_BY_2 as u8 },
    cs53l30_mclkx_div { mclkx: 12000000, ratio: 2, mclkdiv: CS53L30_MCLK_DIV_BY_2 as u8 },
    cs53l30_mclkx_div { mclkx: 19200000, ratio: 3, mclkdiv: CS53L30_MCLK_DIV_BY_3 as u8 },
];

unsafe extern "C" fn cs53l30_get_mclkx_coeff(mclkx: c_int) -> c_int {
    let mut i = 0usize;
    while i < cs53l30_mclkx_coeffs.len() {
        if cs53l30_mclkx_coeffs[i].mclkx == mclkx as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn cs53l30_get_mclk_coeff(mclk_rate: c_int, srate: c_int) -> c_int {
    let mut i = 0usize;
    while i < cs53l30_mclk_coeffs.len() {
        if cs53l30_mclk_coeffs[i].mclk_rate == mclk_rate as u32 &&
           cs53l30_mclk_coeffs[i].srate == srate as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn cs53l30_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    let mclkx_coeff = cs53l30_get_mclkx_coeff(freq as c_int);
    if mclkx_coeff < 0 {
        return mclkx_coeff;
    }
    let coeff = &cs53l30_mclkx_coeffs[mclkx_coeff as usize];
    let mclk_rate = coeff.mclkx / coeff.ratio as u32;
    regmap_update_bits((*priv_).regmap, CS53L30_MCLKCTL, CS53L30_MCLK_DIV_MASK, coeff.mclkdiv as c_uint);
    (*priv_).mclk_rate = mclk_rate;
    0
}

unsafe extern "C" fn cs53l30_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    let mut aspcfg: u8 = 0;
    let mut aspctl1: u8 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => aspcfg |= CS53L30_ASP_MS as u8,
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    // DAI mode
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => aspctl1 |= CS53L30_ASP_TDM_PDN as u8,
        SND_SOC_DAIFMT_DSP_A => aspctl1 |= CS53L30_SHIFT_LEFT as u8,
        _ => return -EINVAL,
    }

    // Check to see if the SCLK is inverted
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_IB_IF => aspcfg ^= CS53L30_ASP_SCLK_INV as u8,
        _ => {}
    }

    regmap_update_bits((*priv_).regmap, CS53L30_ASPCFG_CTL, CS53L30_ASP_MS | CS53L30_ASP_SCLK_INV, aspcfg as c_uint);
    regmap_update_bits((*priv_).regmap, CS53L30_ASP_CTL1, CS53L30_ASP_TDM_PDN | CS53L30_SHIFT_LEFT, aspctl1 as c_uint);
    0
}

unsafe extern "C" fn cs53l30_pcm_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    let srate = params_rate(params);
    let mclk_coeff = cs53l30_get_mclk_coeff((*priv_).mclk_rate as c_int, srate);
    if mclk_coeff < 0 {
        return -EINVAL;
    }
    let coeff = &cs53l30_mclk_coeffs[mclk_coeff as usize];
    regmap_update_bits((*priv_).regmap, CS53L30_INT_SR_CTL, CS53L30_INTRNL_FS_RATIO_MASK, coeff.internal_fs_ratio as c_uint);
    regmap_update_bits((*priv_).regmap, CS53L30_MCLKCTL, CS53L30_MCLK_INT_SCALE_MASK, coeff.mclk_int_scale as c_uint);
    regmap_update_bits((*priv_).regmap, CS53L30_ASPCFG_CTL, CS53L30_ASP_RATE_MASK, coeff.asp_rate as c_uint);
    0
}

unsafe extern "C" fn cs53l30_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs53l30_private;
    let bias_level = snd_soc_dapm_get_bias_level(dapm);
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            if bias_level as c_int == snd_soc_bias_level::SND_SOC_BIAS_STANDBY as c_int {
                regmap_update_bits((*priv_).regmap, CS53L30_PWRCTL, CS53L30_PDN_LP_MASK, 0);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if bias_level as c_int == snd_soc_bias_level::SND_SOC_BIAS_OFF as c_int {
                ret = clk_prepare_enable((*priv_).mclk);
                if ret != 0 {
                    dev_err((*component).dev, b"failed to enable MCLK: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
                regmap_update_bits((*priv_).regmap, CS53L30_MCLKCTL, CS53L30_MCLK_DIS_MASK, 0);
                regmap_update_bits((*priv_).regmap, CS53L30_PWRCTL, CS53L30_PDN_ULP_MASK, 0);
                msleep(50);
            } else {
                regmap_update_bits((*priv_).regmap, CS53L30_PWRCTL, CS53L30_PDN_ULP_MASK, CS53L30_PDN_ULP);
            }
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            regmap_update_bits((*priv_).regmap, CS53L30_INT_MASK, CS53L30_PDN_DONE, 0);
            /*
             * If digital softramp is set, the amount of time required
             * for power down increases and depends on the digital
             * volume setting.
             */
            regmap_read((*priv_).regmap, CS53L30_SFT_RAMP, &mut reg);
            let inter_max_check = if (reg & CS53L30_DIGSFT_MASK) != 0 { CS53L30_PDN_POLL_MAX as c_int } else { 10 };
            regmap_update_bits((*priv_).regmap, CS53L30_PWRCTL, CS53L30_PDN_ULP_MASK, CS53L30_PDN_ULP);
            msleep(20);
            regmap_read((*priv_).regmap, CS53L30_IS, &mut reg);
            let mut i = 0;
            while i < inter_max_check {
                if inter_max_check < 10 {
                    usleep_range(1000, 1100);
                    regmap_read((*priv_).regmap, CS53L30_IS, &mut reg);
                    if (reg & CS53L30_PDN_DONE) != 0 { break; }
                } else {
                    usleep_range(10000, 10100);
                    regmap_read((*priv_).regmap, CS53L30_IS, &mut reg);
                    if (reg & CS53L30_PDN_DONE) != 0 { break; }
                }
                i += 1;
            }
            regmap_update_bits((*priv_).regmap, CS53L30_INT_MASK, CS53L30_PDN_DONE, CS53L30_PDN_DONE);
            regmap_update_bits((*priv_).regmap, CS53L30_MCLKCTL, CS53L30_MCLK_DIS_MASK, CS53L30_MCLK_DIS);
            clk_disable_unprepare((*priv_).mclk);
        }
    }
    0
}

unsafe extern "C" fn cs53l30_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    let val = if tristate != 0 { CS53L30_ASP_3ST } else { 0 };
    regmap_update_bits((*priv_).regmap, CS53L30_ASP_CTL1, CS53L30_ASP_3ST_MASK, val)
}

/*
 * Note: CS53L30 counts the slot number per byte while ASoC counts the slot
 * number per slot_width. So there is a difference between the slots of ASoC
 * and the slots of CS53L30.
 */
unsafe extern "C" fn cs53l30_set_dai_tdm_slot(dai: *mut snd_soc_dai, _tx_mask: c_uint, mut rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    let mut loc: [c_uint; CS53L30_TDM_SLOT_MAX as usize] = [48, 48, 48, 48];
    let mut slot_next: c_uint;
    let slot_step: c_uint;
    let mut tx_enable: u64 = 0;
    let mut i: c_int;

    if rx_mask == 0 {
        dev_err((*dai).dev, b"rx masks must not be 0\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if slots <= 0 || slot_width <= 0 || slot_width > 64 {
        dev_err((*dai).dev, b"invalid slot number or slot width\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if (slot_width & 0x7) != 0 {
        dev_err((*dai).dev, b"slot width must count in byte\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    slot_step = (slot_width >> 3) as c_uint;
    i = 0;
    while rx_mask != 0 && i < CS53L30_TDM_SLOT_MAX as c_int {
        slot_next = __ffs(rx_mask);
        loc[i as usize] = slot_next * slot_step;
        tx_enable |= (((1u64 << slot_step) - 1) << loc[i as usize]) as u64;
        rx_mask &= !(1u32 << slot_next);
        i += 1;
    }

    if rx_mask != 0 && i == CS53L30_TDM_SLOT_MAX as c_int {
        dev_err((*dai).dev, b"rx_mask exceeds max slot number: %d\n\0".as_ptr() as *const c_char, CS53L30_TDM_SLOT_MAX);
        return -EINVAL;
    }

    slot_next = loc[(i - 1) as usize] + slot_step - 1;
    if slot_next > 47 {
        dev_err((*dai).dev, b"slot selection out of bounds: %u\n\0".as_ptr() as *const c_char, slot_next);
        return -EINVAL;
    }

    i = 0;
    while i < CS53L30_TDM_SLOT_MAX as c_int && loc[i as usize] != 48 {
        regmap_update_bits((*priv_).regmap, CS53L30_ASP_TDMTX_CTL(i as c_uint), CS53L30_ASP_CHx_TX_LOC_MASK, loc[i as usize]);
        dev_dbg((*dai).dev, b"loc[%d]=%x\n\0".as_ptr() as *const c_char, i, loc[i as usize]);
        i += 1;
    }

    i = 0;
    while i < CS53L30_ASP_TDMTX_ENx_MAX as c_int && tx_enable != 0 {
        regmap_write((*priv_).regmap, CS53L30_ASP_TDMTX_ENx(i as c_uint), (tx_enable & 0xff) as c_uint);
        tx_enable >>= 8;
        dev_dbg((*dai).dev, b"en_reg=%x, tx_enable=%llx\n\0".as_ptr() as *const c_char, CS53L30_ASP_TDMTX_ENx(i as c_uint), tx_enable & 0xff);
        i += 1;
    }

    0
}

unsafe extern "C" fn cs53l30_mute_stream(dai: *mut snd_soc_dai, mute: c_int, _stream: c_int) -> c_int {
    let priv_ = snd_soc_component_get_drvdata((*dai).component) as *mut cs53l30_private;
    gpiod_set_value_cansleep((*priv_).mute_gpio, mute);
    0
}

const CS53L30_RATES: c_uint = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_12000 | SNDRV_PCM_RATE_24000;
const CS53L30_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE;

static cs53l30_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs53l30_pcm_hw_params),
    set_fmt: Some(cs53l30_set_dai_fmt),
    set_sysclk: Some(cs53l30_set_sysclk),
    set_tristate: Some(cs53l30_set_tristate),
    set_tdm_slot: Some(cs53l30_set_dai_tdm_slot),
    mute_stream: Some(cs53l30_mute_stream),
};

static mut cs53l30_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cs53l30\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 4,
        rates: CS53L30_RATES,
        formats: CS53L30_FORMATS,
    },
    ops: &cs53l30_ops,
    symmetric_rate: 1,
};

unsafe extern "C" fn cs53l30_component_probe(component: *mut snd_soc_component) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs53l30_private;
    let dapm = snd_soc_component_to_dapm(component);
    if (*priv_).use_sdout2 {
        snd_soc_dapm_add_routes(dapm, cs53l30_dapm_routes_sdout2.as_ptr(), cs53l30_dapm_routes_sdout2.len() as c_int);
    } else {
        snd_soc_dapm_add_routes(dapm, cs53l30_dapm_routes_sdout1.as_ptr(), cs53l30_dapm_routes_sdout1.len() as c_int);
    }
    0
}

static cs53l30_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs53l30_component_probe),
    set_bias_level: Some(cs53l30_set_bias_level),
    controls: cs53l30_snd_controls.as_ptr(),
    num_controls: cs53l30_snd_controls.len() as c_uint,
    dapm_widgets: cs53l30_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs53l30_dapm_widgets.len() as c_uint,
    dapm_routes: cs53l30_dapm_routes.as_ptr(),
    num_dapm_routes: cs53l30_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs53l30_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS53L30_MAX_REGISTER,
    reg_defaults: cs53l30_reg_defaults.as_ptr(),
    num_reg_defaults: cs53l30_reg_defaults.len() as c_uint,
    volatile_reg: Some(cs53l30_volatile_register),
    writeable_reg: Some(cs53l30_writeable_register),
    readable_reg: Some(cs53l30_readable_register),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn cs53l30_i2c_probe(client: *mut i2c_client) -> c_int {
    let np = (*client).dev.of_node;
    let dev = &mut (*client).dev as *mut device;
    let mut reg: c_uint = 0;
    let mut ret: c_int = 0;
    let mut val: u8 = 0;

    let cs53l30 = devm_kzalloc(dev, core::mem::size_of::<cs53l30_private>(), GFP_KERNEL) as *mut cs53l30_private;
    if cs53l30.is_null() {
        return -ENOMEM;
    }

    let mut i = 0usize;
    while i < CS53L30_NUM_SUPPLIES {
        (*cs53l30).supplies[i].supply = cs53l30_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"failed to get supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regulator_bulk_enable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    // Reset the Device
    (*cs53l30).reset_gpio = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*cs53l30).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs53l30).reset_gpio as *const c_void);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    gpiod_set_value_cansleep((*cs53l30).reset_gpio, 1);
    i2c_set_clientdata(client, cs53l30 as *mut c_void);
    (*cs53l30).mclk_rate = 0;
    (*cs53l30).regmap = devm_regmap_init_i2c(client, &cs53l30_regmap);
    if IS_ERR((*cs53l30).regmap as *const c_void) {
        ret = PTR_ERR((*cs53l30).regmap as *const c_void);
        dev_err(dev, b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    // Initialize codec
    let devid = cirrus_read_device_id((*cs53l30).regmap, CS53L30_DEVID_AB);
    if devid < 0 {
        ret = devid;
        dev_err(dev, b"Failed to read device ID: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }
    if devid != CS53L30_DEVID as c_int {
        ret = -ENODEV;
        dev_err(dev, b"Device ID (%X). Expected %X\n\0".as_ptr() as *const c_char, devid, CS53L30_DEVID);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_read((*cs53l30).regmap, CS53L30_REVID, &mut reg);
    if ret < 0 {
        dev_err(dev, b"failed to get Revision ID: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    // Check if MCLK provided
    (*cs53l30).mclk = devm_clk_get_optional(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*cs53l30).mclk as *const c_void) {
        ret = PTR_ERR((*cs53l30).mclk as *const c_void);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    // Fetch the MUTE control
    (*cs53l30).mute_gpio = devm_gpiod_get_optional(dev, b"mute\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR((*cs53l30).mute_gpio as *const c_void) {
        ret = PTR_ERR((*cs53l30).mute_gpio as *const c_void);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }

    if !(*cs53l30).mute_gpio.is_null() {
        regmap_write((*cs53l30).regmap, CS53L30_MUTEP_CTL1, CS53L30_MUTEP_CTL1_MUTEALL);
        if gpiod_is_active_low((*cs53l30).mute_gpio) != 0 {
            regmap_update_bits((*cs53l30).regmap, CS53L30_MUTEP_CTL2, CS53L30_MUTE_PIN_POLARITY, 0);
        }
    }

    if of_property_read_u8(np, b"cirrus,micbias-lvl\0".as_ptr() as *const c_char, &mut val) == 0 {
        regmap_update_bits((*cs53l30).regmap, CS53L30_MICBIAS_CTL, CS53L30_MIC_BIAS_CTRL_MASK, val as c_uint);
    }
    if of_property_read_bool(np, b"cirrus,use-sdout2\0".as_ptr() as *const c_char) != 0 {
        (*cs53l30).use_sdout2 = true;
    }

    dev_info(dev, b"Cirrus Logic CS53L30, Revision: %02X\n\0".as_ptr() as *const c_char, reg & 0xFF);
    ret = devm_snd_soc_register_component(dev, &cs53l30_driver, &raw mut cs53l30_dai, 1);
    if ret != 0 {
        dev_err(dev, b"failed to register component: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
        regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
        return ret;
    }
    0
}

unsafe extern "C" fn cs53l30_i2c_remove(client: *mut i2c_client) {
    let cs53l30 = i2c_get_clientdata(client) as *mut cs53l30_private;
    // Hold down reset
    gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
    regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
}

unsafe extern "C" fn cs53l30_runtime_suspend(dev: *mut device) -> c_int {
    let cs53l30 = dev_get_drvdata(dev) as *mut cs53l30_private;
    regcache_cache_only((*cs53l30).regmap, true);
    // Hold down reset
    gpiod_set_value_cansleep((*cs53l30).reset_gpio, 0);
    regulator_bulk_disable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
    0
}

unsafe extern "C" fn cs53l30_runtime_resume(dev: *mut device) -> c_int {
    let cs53l30 = dev_get_drvdata(dev) as *mut cs53l30_private;
    let mut ret = regulator_bulk_enable(CS53L30_NUM_SUPPLIES as c_int, (*cs53l30).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    gpiod_set_value_cansleep((*cs53l30).reset_gpio, 1);
    regcache_cache_only((*cs53l30).regmap, false);
    ret = regcache_sync((*cs53l30).regmap);
    if ret != 0 {
        dev_err(dev, b"failed to synchronize regcache: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    0
}

// static const struct dev_pm_ops cs53l30_runtime_pm = {
//     RUNTIME_PM_OPS(cs53l30_runtime_suspend, cs53l30_runtime_resume, NULL)
// };
static cs53l30_runtime_pm: dev_pm_ops = dev_pm_ops { _private: [] };

static cs53l30_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"cirrus,cs53l30\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, cs53l30_of_match);

static cs53l30_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"cs53l30\0".as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, cs53l30_id);

static mut cs53l30_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_private {
        name: b"cs53l30\0".as_ptr() as *const c_char,
        of_match_table: cs53l30_of_match.as_ptr(),
        pm: &cs53l30_runtime_pm,
    },
    id_table: cs53l30_id.as_ptr(),
    probe: Some(cs53l30_i2c_probe),
    remove: Some(cs53l30_i2c_remove),
};

// module_i2c_driver(cs53l30_i2c_driver);
// MODULE_DESCRIPTION("ASoC CS53L30 driver");
// MODULE_AUTHOR("Paul Handrigan, Cirrus Logic Inc, <Paul.Handrigan@cirrus.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
