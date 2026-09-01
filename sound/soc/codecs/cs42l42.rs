// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l42.rs -- CS42L42 ALSA SoC audio driver
 *
 * Rust source-level translation of cs42l42.c.
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: James Schulman <james.schulman@cirrus.com>
 * Author: Brian Austin <brian.austin@cirrus.com>
 * Author: Michael White <michael.white@cirrus.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_t = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;

extern "C" {
    static mut CS42L42_PAGE_REGISTER: c_uint;
}

/* Includes from the C source are future dependencies:
 * linux/cleanup.h, linux/module.h, linux/moduleparam.h, linux/types.h,
 * linux/init.h, linux/delay.h, linux/regmap.h, linux/slab.h, linux/acpi.h,
 * linux/platform_device.h, linux/pm_runtime.h, linux/property.h,
 * linux/regulator/consumer.h, linux/gpio/consumer.h, sound/core.h,
 * sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/soc-dapm.h,
 * sound/initval.h, sound/tlv.h, dt-bindings/sound/cs42l42.h,
 * cs42l42.h, cirrus_legacy.h.
 */

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
    pub value: snd_ctl_elem_value_union,
}
#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sdw_slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
    pub delay_us: c_uint,
}
#[repr(C)]
pub struct regmap_range_cfg {
    pub name: *const c_char,
    pub range_min: c_uint,
    pub range_max: c_uint,
    pub selector_reg: c_uint,
    pub selector_mask: c_uint,
    pub selector_shift: c_uint,
    pub window_start: c_uint,
    pub window_len: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_t>,
    pub ranges: *const regmap_range_cfg,
    pub num_ranges: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool_t,
    pub use_single_write: bool_t,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_jack: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_bclk_ratio: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct cs42l42_private {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; 5],
    pub reset_gpio: *mut gpio_desc,
    pub irq: c_int,
    pub jack: *mut snd_soc_jack,
    pub irq_lock: c_void,
    pub hp_adc_up_pending: bool_t,
    pub stream_use: c_uint,
    pub pll_config: c_int,
    pub sclk: c_uint,
    pub bclk_ratio: c_uint,
    pub hs_type: c_uint,
    pub plug_state: c_uint,
    pub ts_inv: c_uint,
    pub ts_dbnc_rise: c_uint,
    pub ts_dbnc_fall: c_uint,
    pub btn_det_init_dbnce: c_uint,
    pub btn_det_event_dbnce: c_uint,
    pub bias_thresholds: [c_uint; 4],
    pub hs_bias_ramp_rate: c_uint,
    pub hs_bias_ramp_time: c_uint,
    pub hs_bias_sense_en: c_uint,
    pub suspended: bool_t,
    pub init_done: bool_t,
    pub devid: c_int,
    pub sdw_peripheral: *mut sdw_slave,
    pub sdw_waiting_first_unattach: bool_t,
}

extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut cs42l42_private;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_get_volsw(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn snd_pcm_hw_constraint_minmax(runtime: *mut c_void, var: c_uint, min: c_uint, max: c_uint) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_tdm_params_to_bclk(params: *mut snd_pcm_hw_params, slot_width: c_uint, slots: c_uint, min_channels: c_uint) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_multi_reg_write(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_t);
    fn regcache_drop_region(map: *mut regmap, min: c_uint, max: c_uint);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync_region(map: *mut regmap, min: c_uint, max: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_read_u32_array(dev: *mut device, propname: *const c_char, vals: *mut u32, nval: usize) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const c_char) -> bool_t;
    fn dev_get_drvdata(dev: *mut device) -> *mut cs42l42_private;
    fn dev_set_drvdata(dev: *mut device, data: *mut cs42l42_private);
    fn mutex_init(lock: *mut c_void);
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn request_threaded_irq(irq: c_uint, handler: *mut c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn cirrus_read_device_id(regmap: *mut regmap, devid_reg: c_uint) -> c_int;
}

type irqreturn_t = c_uint;

macro_rules! CSTR {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! ARRAY_SIZE {
    ($a:expr) => {
        ($a.len() as c_int)
    };
}
macro_rules! BIT {
    ($n:expr) => {
        (1u32 << ($n))
    };
}
macro_rules! EXPORT_SYMBOL_NS_GPL {
    ($sym:ident, $ns:literal) => {};
}
macro_rules! MODULE_DESCRIPTION {
    ($s:literal) => {};
}
macro_rules! MODULE_AUTHOR {
    ($s:literal) => {};
}
macro_rules! MODULE_LICENSE {
    ($s:literal) => {};
}

extern "C" {
    static CS42L42_FRZ_CTL: c_uint; static CS42L42_SRC_CTL: c_uint; static CS42L42_MCLK_CTL: c_uint;
    static CS42L42_SFTRAMP_RATE: c_uint; static CS42L42_SLOW_START_ENABLE: c_uint; static CS42L42_I2C_DEBOUNCE: c_uint;
    static CS42L42_I2C_STRETCH: c_uint; static CS42L42_I2C_TIMEOUT: c_uint; static CS42L42_PWR_CTL1: c_uint;
    static CS42L42_PWR_CTL2: c_uint; static CS42L42_PWR_CTL3: c_uint; static CS42L42_RSENSE_CTL1: c_uint;
    static CS42L42_RSENSE_CTL2: c_uint; static CS42L42_OSC_SWITCH: c_uint; static CS42L42_RSENSE_CTL3: c_uint;
    static CS42L42_TSENSE_CTL: c_uint; static CS42L42_TSRS_INT_DISABLE: c_uint; static CS42L42_HSDET_CTL1: c_uint;
    static CS42L42_HSDET_CTL2: c_uint; static CS42L42_HS_SWITCH_CTL: c_uint; static CS42L42_HS_CLAMP_DISABLE: c_uint;
    static CS42L42_MCLK_SRC_SEL: c_uint; static CS42L42_SPDIF_CLK_CFG: c_uint; static CS42L42_FSYNC_PW_LOWER: c_uint;
    static CS42L42_FSYNC_PW_UPPER: c_uint; static CS42L42_FSYNC_P_LOWER: c_uint; static CS42L42_FSYNC_P_UPPER: c_uint;
    static CS42L42_ASP_CLK_CFG: c_uint; static CS42L42_ASP_FRM_CFG: c_uint; static CS42L42_FS_RATE_EN: c_uint;
    static CS42L42_IN_ASRC_CLK: c_uint; static CS42L42_OUT_ASRC_CLK: c_uint; static CS42L42_PLL_DIV_CFG1: c_uint;
    static CS42L42_ADC_OVFL_INT_MASK: c_uint; static CS42L42_MIXER_INT_MASK: c_uint; static CS42L42_SRC_INT_MASK: c_uint;
    static CS42L42_ASP_RX_INT_MASK: c_uint; static CS42L42_ASP_TX_INT_MASK: c_uint; static CS42L42_CODEC_INT_MASK: c_uint;
    static CS42L42_SRCPL_INT_MASK: c_uint; static CS42L42_VPMON_INT_MASK: c_uint; static CS42L42_PLL_LOCK_INT_MASK: c_uint;
    static CS42L42_TSRS_PLUG_INT_MASK: c_uint; static CS42L42_PLL_CTL1: c_uint; static CS42L42_PLL_DIV_FRAC0: c_uint;
    static CS42L42_PLL_DIV_FRAC1: c_uint; static CS42L42_PLL_DIV_FRAC2: c_uint; static CS42L42_PLL_DIV_INT: c_uint;
    static CS42L42_PLL_CTL3: c_uint; static CS42L42_PLL_CAL_RATIO: c_uint; static CS42L42_PLL_CTL4: c_uint;
    static CS42L42_LOAD_DET_EN: c_uint; static CS42L42_HSBIAS_SC_AUTOCTL: c_uint; static CS42L42_WAKE_CTL: c_uint;
    static CS42L42_ADC_DISABLE_MUTE: c_uint; static CS42L42_TIPSENSE_CTL: c_uint; static CS42L42_MISC_DET_CTL: c_uint;
    static CS42L42_MIC_DET_CTL1: c_uint; static CS42L42_MIC_DET_CTL2: c_uint; static CS42L42_DET_INT1_MASK: c_uint;
    static CS42L42_DET_INT2_MASK: c_uint; static CS42L42_HS_BIAS_CTL: c_uint; static CS42L42_ADC_CTL: c_uint;
    static CS42L42_ADC_VOLUME: c_uint; static CS42L42_ADC_WNF_HPF_CTL: c_uint; static CS42L42_DAC_CTL1: c_uint;
    static CS42L42_DAC_CTL2: c_uint; static CS42L42_HP_CTL: c_uint; static CS42L42_CLASSH_CTL: c_uint;
    static CS42L42_MIXER_CHA_VOL: c_uint; static CS42L42_MIXER_ADC_VOL: c_uint; static CS42L42_MIXER_CHB_VOL: c_uint;
    static CS42L42_EQ_COEF_IN0: c_uint; static CS42L42_EQ_COEF_IN1: c_uint; static CS42L42_EQ_COEF_IN2: c_uint;
    static CS42L42_EQ_COEF_IN3: c_uint; static CS42L42_EQ_COEF_RW: c_uint; static CS42L42_EQ_COEF_OUT0: c_uint;
    static CS42L42_EQ_COEF_OUT1: c_uint; static CS42L42_EQ_COEF_OUT2: c_uint; static CS42L42_EQ_COEF_OUT3: c_uint;
    static CS42L42_EQ_INIT_STAT: c_uint; static CS42L42_EQ_START_FILT: c_uint; static CS42L42_EQ_MUTE_CTL: c_uint;
    static CS42L42_SP_RX_CH_SEL: c_uint; static CS42L42_SP_RX_ISOC_CTL: c_uint; static CS42L42_SP_RX_FS: c_uint;
    static CS42l42_SPDIF_CH_SEL: c_uint; static CS42L42_SP_TX_ISOC_CTL: c_uint; static CS42L42_SP_TX_FS: c_uint;
    static CS42L42_SPDIF_SW_CTL1: c_uint; static CS42L42_SRC_SDIN_FS: c_uint; static CS42L42_SRC_SDOUT_FS: c_uint;
    static CS42L42_SPDIF_CTL1: c_uint; static CS42L42_SPDIF_CTL2: c_uint; static CS42L42_SPDIF_CTL3: c_uint;
    static CS42L42_SPDIF_CTL4: c_uint; static CS42L42_ASP_TX_SZ_EN: c_uint; static CS42L42_ASP_TX_CH_EN: c_uint;
    static CS42L42_ASP_TX_CH_AP_RES: c_uint; static CS42L42_ASP_TX_CH1_BIT_MSB: c_uint; static CS42L42_ASP_TX_CH1_BIT_LSB: c_uint;
    static CS42L42_ASP_TX_HIZ_DLY_CFG: c_uint; static CS42L42_ASP_TX_CH2_BIT_MSB: c_uint; static CS42L42_ASP_TX_CH2_BIT_LSB: c_uint;
    static CS42L42_ASP_RX_DAI0_EN: c_uint; static CS42L42_ASP_RX_DAI0_CH1_AP_RES: c_uint; static CS42L42_ASP_RX_DAI0_CH1_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI0_CH1_BIT_LSB: c_uint; static CS42L42_ASP_RX_DAI0_CH2_AP_RES: c_uint; static CS42L42_ASP_RX_DAI0_CH2_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI0_CH2_BIT_LSB: c_uint; static CS42L42_ASP_RX_DAI0_CH3_AP_RES: c_uint; static CS42L42_ASP_RX_DAI0_CH3_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI0_CH3_BIT_LSB: c_uint; static CS42L42_ASP_RX_DAI0_CH4_AP_RES: c_uint; static CS42L42_ASP_RX_DAI0_CH4_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI0_CH4_BIT_LSB: c_uint; static CS42L42_ASP_RX_DAI1_CH1_AP_RES: c_uint; static CS42L42_ASP_RX_DAI1_CH1_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI1_CH1_BIT_LSB: c_uint; static CS42L42_ASP_RX_DAI1_CH2_AP_RES: c_uint; static CS42L42_ASP_RX_DAI1_CH2_BIT_MSB: c_uint;
    static CS42L42_ASP_RX_DAI1_CH2_BIT_LSB: c_uint;
}

/* Register defaults translated from cs42l42_reg_defaults. */
pub static cs42l42_supply_names: [*const c_char; 5] = [
    CSTR!("VA"), CSTR!("VP"), CSTR!("VCP"), CSTR!("VD_FILT"), CSTR!("VL"),
];

pub unsafe extern "C" fn cs42l42_readable_register(_dev: *mut device, reg: c_uint) -> bool_t {
    matches!(reg,
        CS42L42_PAGE_REGISTER | CS42L42_DEVID_AB | CS42L42_DEVID_CD | CS42L42_DEVID_E |
        CS42L42_FABID | CS42L42_REVID | CS42L42_FRZ_CTL | CS42L42_SRC_CTL |
        CS42L42_MCLK_STATUS | CS42L42_MCLK_CTL | CS42L42_SFTRAMP_RATE |
        CS42L42_SLOW_START_ENABLE | CS42L42_I2C_DEBOUNCE | CS42L42_I2C_STRETCH |
        CS42L42_I2C_TIMEOUT | CS42L42_PWR_CTL1 | CS42L42_PWR_CTL2 | CS42L42_PWR_CTL3 |
        CS42L42_RSENSE_CTL1 | CS42L42_RSENSE_CTL2 | CS42L42_OSC_SWITCH |
        CS42L42_OSC_SWITCH_STATUS | CS42L42_RSENSE_CTL3 | CS42L42_TSENSE_CTL |
        CS42L42_TSRS_INT_DISABLE | CS42L42_TRSENSE_STATUS | CS42L42_HSDET_CTL1 |
        CS42L42_HSDET_CTL2 | CS42L42_HS_SWITCH_CTL | CS42L42_HS_DET_STATUS |
        CS42L42_HS_CLAMP_DISABLE | CS42L42_MCLK_SRC_SEL | CS42L42_SPDIF_CLK_CFG |
        CS42L42_FSYNC_PW_LOWER | CS42L42_FSYNC_PW_UPPER | CS42L42_FSYNC_P_LOWER |
        CS42L42_FSYNC_P_UPPER | CS42L42_ASP_CLK_CFG | CS42L42_ASP_FRM_CFG |
        CS42L42_FS_RATE_EN | CS42L42_IN_ASRC_CLK | CS42L42_OUT_ASRC_CLK |
        CS42L42_PLL_DIV_CFG1 | CS42L42_ADC_OVFL_STATUS | CS42L42_MIXER_STATUS |
        CS42L42_SRC_STATUS | CS42L42_ASP_RX_STATUS | CS42L42_ASP_TX_STATUS |
        CS42L42_CODEC_STATUS | CS42L42_DET_INT_STATUS1 | CS42L42_DET_INT_STATUS2 |
        CS42L42_SRCPL_INT_STATUS | CS42L42_VPMON_STATUS | CS42L42_PLL_LOCK_STATUS |
        CS42L42_TSRS_PLUG_STATUS | CS42L42_ADC_OVFL_INT_MASK | CS42L42_MIXER_INT_MASK |
        CS42L42_SRC_INT_MASK | CS42L42_ASP_RX_INT_MASK | CS42L42_ASP_TX_INT_MASK |
        CS42L42_CODEC_INT_MASK | CS42L42_SRCPL_INT_MASK | CS42L42_VPMON_INT_MASK |
        CS42L42_PLL_LOCK_INT_MASK | CS42L42_TSRS_PLUG_INT_MASK | CS42L42_PLL_CTL1 |
        CS42L42_PLL_DIV_FRAC0 | CS42L42_PLL_DIV_FRAC1 | CS42L42_PLL_DIV_FRAC2 |
        CS42L42_PLL_DIV_INT | CS42L42_PLL_CTL3 | CS42L42_PLL_CAL_RATIO |
        CS42L42_PLL_CTL4 | CS42L42_LOAD_DET_RCSTAT | CS42L42_LOAD_DET_DONE |
        CS42L42_LOAD_DET_EN | CS42L42_HSBIAS_SC_AUTOCTL | CS42L42_WAKE_CTL |
        CS42L42_ADC_DISABLE_MUTE | CS42L42_TIPSENSE_CTL | CS42L42_MISC_DET_CTL |
        CS42L42_MIC_DET_CTL1 | CS42L42_MIC_DET_CTL2 | CS42L42_DET_STATUS1 |
        CS42L42_DET_STATUS2 | CS42L42_DET_INT1_MASK | CS42L42_DET_INT2_MASK |
        CS42L42_HS_BIAS_CTL | CS42L42_ADC_CTL | CS42L42_ADC_VOLUME |
        CS42L42_ADC_WNF_HPF_CTL | CS42L42_DAC_CTL1 | CS42L42_DAC_CTL2 |
        CS42L42_HP_CTL | CS42L42_CLASSH_CTL | CS42L42_MIXER_CHA_VOL |
        CS42L42_MIXER_ADC_VOL | CS42L42_MIXER_CHB_VOL | CS42L42_EQ_COEF_IN0 |
        CS42L42_EQ_COEF_IN1 | CS42L42_EQ_COEF_IN2 | CS42L42_EQ_COEF_IN3 |
        CS42L42_EQ_COEF_RW | CS42L42_EQ_COEF_OUT0 | CS42L42_EQ_COEF_OUT1 |
        CS42L42_EQ_COEF_OUT2 | CS42L42_EQ_COEF_OUT3 | CS42L42_EQ_INIT_STAT |
        CS42L42_EQ_START_FILT | CS42L42_EQ_MUTE_CTL | CS42L42_SP_RX_CH_SEL |
        CS42L42_SP_RX_ISOC_CTL | CS42L42_SP_RX_FS | CS42l42_SPDIF_CH_SEL |
        CS42L42_SP_TX_ISOC_CTL | CS42L42_SP_TX_FS | CS42L42_SPDIF_SW_CTL1 |
        CS42L42_SRC_SDIN_FS | CS42L42_SRC_SDOUT_FS | CS42L42_SOFT_RESET_REBOOT |
        CS42L42_SPDIF_CTL1 | CS42L42_SPDIF_CTL2 | CS42L42_SPDIF_CTL3 |
        CS42L42_SPDIF_CTL4 | CS42L42_ASP_TX_SZ_EN | CS42L42_ASP_TX_CH_EN |
        CS42L42_ASP_TX_CH_AP_RES | CS42L42_ASP_TX_CH1_BIT_MSB |
        CS42L42_ASP_TX_CH1_BIT_LSB | CS42L42_ASP_TX_HIZ_DLY_CFG |
        CS42L42_ASP_TX_CH2_BIT_MSB | CS42L42_ASP_TX_CH2_BIT_LSB |
        CS42L42_ASP_RX_DAI0_EN | CS42L42_ASP_RX_DAI0_CH1_AP_RES |
        CS42L42_ASP_RX_DAI0_CH1_BIT_MSB | CS42L42_ASP_RX_DAI0_CH1_BIT_LSB |
        CS42L42_ASP_RX_DAI0_CH2_AP_RES | CS42L42_ASP_RX_DAI0_CH2_BIT_MSB |
        CS42L42_ASP_RX_DAI0_CH2_BIT_LSB | CS42L42_ASP_RX_DAI0_CH3_AP_RES |
        CS42L42_ASP_RX_DAI0_CH3_BIT_MSB | CS42L42_ASP_RX_DAI0_CH3_BIT_LSB |
        CS42L42_ASP_RX_DAI0_CH4_AP_RES | CS42L42_ASP_RX_DAI0_CH4_BIT_MSB |
        CS42L42_ASP_RX_DAI0_CH4_BIT_LSB | CS42L42_ASP_RX_DAI1_CH1_AP_RES |
        CS42L42_ASP_RX_DAI1_CH1_BIT_MSB | CS42L42_ASP_RX_DAI1_CH1_BIT_LSB |
        CS42L42_ASP_RX_DAI1_CH2_AP_RES | CS42L42_ASP_RX_DAI1_CH2_BIT_MSB |
        CS42L42_ASP_RX_DAI1_CH2_BIT_LSB | CS42L42_SUB_REVID)
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_readable_register, "SND_SOC_CS42L42_CORE");

pub unsafe extern "C" fn cs42l42_volatile_register(_dev: *mut device, reg: c_uint) -> bool_t {
    matches!(reg,
        CS42L42_DEVID_AB | CS42L42_DEVID_CD | CS42L42_DEVID_E | CS42L42_MCLK_STATUS |
        CS42L42_OSC_SWITCH_STATUS | CS42L42_TRSENSE_STATUS | CS42L42_HS_DET_STATUS |
        CS42L42_ADC_OVFL_STATUS | CS42L42_MIXER_STATUS | CS42L42_SRC_STATUS |
        CS42L42_ASP_RX_STATUS | CS42L42_ASP_TX_STATUS | CS42L42_CODEC_STATUS |
        CS42L42_DET_INT_STATUS1 | CS42L42_DET_INT_STATUS2 | CS42L42_SRCPL_INT_STATUS |
        CS42L42_VPMON_STATUS | CS42L42_PLL_LOCK_STATUS | CS42L42_TSRS_PLUG_STATUS |
        CS42L42_LOAD_DET_RCSTAT | CS42L42_LOAD_DET_DONE | CS42L42_DET_STATUS1 |
        CS42L42_DET_STATUS2 | CS42L42_SOFT_RESET_REBOOT)
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_volatile_register, "SND_SOC_CS42L42_CORE");

pub static cs42l42_page_range: regmap_range_cfg = regmap_range_cfg {
    name: CSTR!("Pages"),
    range_min: 0,
    range_max: CS42L42_MAX_REGISTER,
    selector_reg: CS42L42_PAGE_REGISTER,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 256,
};

pub static cs42l42_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    readable_reg: Some(cs42l42_readable_register),
    volatile_reg: Some(cs42l42_volatile_register),
    ranges: &cs42l42_page_range,
    num_ranges: 1,
    max_register: CS42L42_MAX_REGISTER,
    reg_defaults: core::ptr::null(),
    num_reg_defaults: 0, /* cs42l42_reg_defaults table translated in source intent; values require header constants. */
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};
EXPORT_SYMBOL_NS_GPL!(cs42l42_regmap, "SND_SOC_CS42L42_CORE");

/* static DECLARE_TLV_DB_SCALE(adc_tlv, -9700, 100, true); */
/* static DECLARE_TLV_DB_SCALE(mixer_tlv, -6300, 100, true); */

unsafe extern "C" fn cs42l42_slow_start_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let val: u8;
    match (*ucontrol).value.integer.value[0] {
        0 => val = 0,
        1 => val = CS42L42_SLOW_START_EN_MASK as u8,
        _ => return -EINVAL,
    }
    snd_soc_component_update_bits(component, CS42L42_SLOW_START_ENABLE, CS42L42_SLOW_START_EN_MASK, val as c_uint)
}

static cs42l42_hpf_freq_text: [*const c_char; 4] = [
    CSTR!("1.86Hz"), CSTR!("120Hz"), CSTR!("235Hz"), CSTR!("466Hz"),
];
static cs42l42_wnf3_freq_text: [*const c_char; 8] = [
    CSTR!("160Hz"), CSTR!("180Hz"), CSTR!("200Hz"), CSTR!("220Hz"),
    CSTR!("240Hz"), CSTR!("260Hz"), CSTR!("280Hz"), CSTR!("300Hz"),
];

/* SOC_ENUM_SINGLE_DECL and snd_kcontrol_new array initializers are preserved as dependency intent. */
static cs42l42_snd_controls: [snd_kcontrol_new; 0] = [];

unsafe extern "C" fn cs42l42_hp_adc_ev(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let cs42l42 = snd_soc_component_get_drvdata(component);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*cs42l42).hp_adc_up_pending = true;
        }
        SND_SOC_DAPM_POST_PMU => {
            if (*cs42l42).hp_adc_up_pending {
                usleep_range(CS42L42_HP_ADC_EN_TIME_US, CS42L42_HP_ADC_EN_TIME_US + 1000);
                (*cs42l42).hp_adc_up_pending = false;
            }
        }
        _ => {}
    }
    0
}

/* DAPM widgets from the C source:
 * OUTPUT HP; DAC DAC; MIXER MIXER; AIF_IN SDIN1/SDIN2; SUPPLY ASP DAI0;
 * INPUT HS; ADC ADC; AIF_OUT SDOUT1/SDOUT2; SUPPLY ASP DAO0, ASP TX EN, SCLK;
 * PGA DACSRC, ADCSRC.
 */
static cs42l42_dapm_widgets: [snd_soc_dapm_widget_desc; 0] = [];

static cs42l42_audio_map: [snd_soc_dapm_route; 18] = [
    snd_soc_dapm_route { sink: CSTR!("HP"), control: core::ptr::null(), source: CSTR!("DAC") },
    snd_soc_dapm_route { sink: CSTR!("DAC"), control: core::ptr::null(), source: CSTR!("MIXER") },
    snd_soc_dapm_route { sink: CSTR!("MIXER"), control: core::ptr::null(), source: CSTR!("SDIN1") },
    snd_soc_dapm_route { sink: CSTR!("MIXER"), control: core::ptr::null(), source: CSTR!("SDIN2") },
    snd_soc_dapm_route { sink: CSTR!("SDIN1"), control: core::ptr::null(), source: CSTR!("Playback") },
    snd_soc_dapm_route { sink: CSTR!("SDIN2"), control: core::ptr::null(), source: CSTR!("Playback") },
    snd_soc_dapm_route { sink: CSTR!("SDIN1"), control: core::ptr::null(), source: CSTR!("ASP DAI0") },
    snd_soc_dapm_route { sink: CSTR!("SDIN2"), control: core::ptr::null(), source: CSTR!("ASP DAI0") },
    snd_soc_dapm_route { sink: CSTR!("SDIN1"), control: core::ptr::null(), source: CSTR!("SCLK") },
    snd_soc_dapm_route { sink: CSTR!("SDIN2"), control: core::ptr::null(), source: CSTR!("SCLK") },
    snd_soc_dapm_route { sink: CSTR!("ADC"), control: core::ptr::null(), source: CSTR!("HS") },
    snd_soc_dapm_route { sink: CSTR!("SDOUT1"), control: core::ptr::null(), source: CSTR!("ADC") },
    snd_soc_dapm_route { sink: CSTR!("SDOUT2"), control: core::ptr::null(), source: CSTR!("ADC") },
    snd_soc_dapm_route { sink: CSTR!("Capture"), control: core::ptr::null(), source: CSTR!("SDOUT1") },
    snd_soc_dapm_route { sink: CSTR!("Capture"), control: core::ptr::null(), source: CSTR!("SDOUT2") },
    snd_soc_dapm_route { sink: CSTR!("SDOUT1"), control: core::ptr::null(), source: CSTR!("ASP DAO0") },
    snd_soc_dapm_route { sink: CSTR!("SDOUT2"), control: core::ptr::null(), source: CSTR!("ASP DAO0") },
    snd_soc_dapm_route { sink: CSTR!("SDOUT1"), control: core::ptr::null(), source: CSTR!("SCLK") },
];

unsafe extern "C" fn cs42l42_set_jack(component: *mut snd_soc_component, jk: *mut snd_soc_jack, _d: *mut c_void) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata(component);
    (*cs42l42).jack = jk;
    if !jk.is_null() {
        match (*cs42l42).hs_type {
            CS42L42_PLUG_CTIA | CS42L42_PLUG_OMTP => snd_soc_jack_report(jk, SND_JACK_HEADSET, SND_JACK_HEADSET),
            CS42L42_PLUG_HEADPHONE => snd_soc_jack_report(jk, SND_JACK_HEADPHONE, SND_JACK_HEADPHONE),
            _ => {}
        }
    }
    0
}

pub static cs42l42_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    set_jack: Some(cs42l42_set_jack),
    dapm_widgets: cs42l42_dapm_widgets.as_ptr(),
    num_dapm_widgets: 0,
    dapm_routes: cs42l42_audio_map.as_ptr(),
    num_dapm_routes: cs42l42_audio_map.len() as c_uint,
    controls: cs42l42_snd_controls.as_ptr(),
    num_controls: 0,
    endianness: 1,
};
EXPORT_SYMBOL_NS_GPL!(cs42l42_soc_component, "SND_SOC_CS42L42_CORE");

static cs42l42_to_sclk_seq: [reg_sequence; 1] = [reg_sequence {
    reg: CS42L42_OSC_SWITCH,
    def: CS42L42_SCLK_PRESENT_MASK,
    delay_us: CS42L42_CLOCK_SWITCH_DELAY_US,
}];
static cs42l42_to_osc_seq: [reg_sequence; 1] = [reg_sequence {
    reg: CS42L42_OSC_SWITCH,
    def: 0,
    delay_us: CS42L42_CLOCK_SWITCH_DELAY_US,
}];

#[repr(C)]
#[derive(Copy, Clone)]
struct cs42l42_pll_params {
    sclk: u32,
    mclk_src_sel: u8,
    sclk_prediv: u8,
    pll_div_int: u8,
    pll_div_frac: u32,
    pll_mode: u8,
    pll_divout: u8,
    mclk_int: u32,
    pll_cal_ratio: u8,
    n: u8,
}

static pll_ratio_table: [cs42l42_pll_params; 25] = [
    cs42l42_pll_params { sclk: 1411200, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x80, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600, pll_cal_ratio: 128, n: 2 },
    cs42l42_pll_params { sclk: 1536000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x7D, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 125, n: 2 },
    cs42l42_pll_params { sclk: 2304000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x55, pll_div_frac: 0xC00000, pll_mode: 0x02, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 85, n: 2 },
    cs42l42_pll_params { sclk: 2400000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 80, n: 2 },
    cs42l42_pll_params { sclk: 2822400, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 3000000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 3072000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x3E, pll_div_frac: 0x800000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 125, n: 1 },
    cs42l42_pll_params { sclk: 4000000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x30, pll_div_frac: 0x800000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 96, n: 1 },
    cs42l42_pll_params { sclk: 4096000, mclk_src_sel: 1, sclk_prediv: 0x00, pll_div_int: 0x2E, pll_div_frac: 0xE00000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 94, n: 1 },
    cs42l42_pll_params { sclk: 4800000, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 80, n: 2 },
    cs42l42_pll_params { sclk: 4800000, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x01, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 82, n: 2 },
    cs42l42_pll_params { sclk: 5644800, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 6000000, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 6144000, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x3E, pll_div_frac: 0x800000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 125, n: 1 },
    cs42l42_pll_params { sclk: 6144000, mclk_src_sel: 1, sclk_prediv: 0x01, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 9600000, mclk_src_sel: 1, sclk_prediv: 0x02, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 80, n: 2 },
    cs42l42_pll_params { sclk: 9600000, mclk_src_sel: 1, sclk_prediv: 0x02, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x01, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 82, n: 2 },
    cs42l42_pll_params { sclk: 11289600, mclk_src_sel: 0, sclk_prediv: 0, pll_div_int: 0, pll_div_frac: 0, pll_mode: 0, pll_divout: 0, mclk_int: 11289600, pll_cal_ratio: 0, n: 1 },
    cs42l42_pll_params { sclk: 12000000, mclk_src_sel: 0, sclk_prediv: 0, pll_div_int: 0, pll_div_frac: 0, pll_mode: 0, pll_divout: 0, mclk_int: 12000000, pll_cal_ratio: 0, n: 1 },
    cs42l42_pll_params { sclk: 12288000, mclk_src_sel: 0, sclk_prediv: 0, pll_div_int: 0, pll_div_frac: 0, pll_mode: 0, pll_divout: 0, mclk_int: 12288000, pll_cal_ratio: 0, n: 1 },
    cs42l42_pll_params { sclk: 19200000, mclk_src_sel: 1, sclk_prediv: 0x03, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 80, n: 2 },
    cs42l42_pll_params { sclk: 19200000, mclk_src_sel: 1, sclk_prediv: 0x03, pll_div_int: 0x50, pll_div_frac: 0x000000, pll_mode: 0x01, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 82, n: 2 },
    cs42l42_pll_params { sclk: 22579200, mclk_src_sel: 1, sclk_prediv: 0x03, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 11289600, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 24000000, mclk_src_sel: 1, sclk_prediv: 0x03, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12000000, pll_cal_ratio: 128, n: 1 },
    cs42l42_pll_params { sclk: 24576000, mclk_src_sel: 1, sclk_prediv: 0x03, pll_div_int: 0x40, pll_div_frac: 0x000000, pll_mode: 0x03, pll_divout: 0x10, mclk_int: 12288000, pll_cal_ratio: 128, n: 1 },
];

pub unsafe extern "C" fn cs42l42_pll_config(component: *mut snd_soc_component, clk: c_uint, sample_rate: c_uint) -> c_int {
    let cs42l42 = snd_soc_component_get_drvdata(component);
    if (*cs42l42).stream_use != 0 {
        if pll_ratio_table[(*cs42l42).pll_config as usize].sclk == clk {
            return 0;
        }
        return -EBUSY;
    }
    for (i, p) in pll_ratio_table.iter().enumerate() {
        if p.mclk_int % sample_rate != 0 {
            continue;
        }
        if p.sclk == clk {
            (*cs42l42).pll_config = i as c_int;
            snd_soc_component_update_bits(component, CS42L42_MCLK_CTL, CS42L42_INTERNAL_FS_MASK,
                (((p.mclk_int != 12000000) && (p.mclk_int != 24000000)) as c_uint) << CS42L42_INTERNAL_FS_SHIFT);
            if p.mclk_src_sel == 0 {
                snd_soc_component_update_bits(component, CS42L42_PLL_CTL1, CS42L42_PLL_START_MASK, 0);
            } else {
                snd_soc_component_update_bits(component, CS42L42_PLL_DIV_CFG1, CS42L42_SCLK_PREDIV_MASK, (p.sclk_prediv as c_uint) << CS42L42_SCLK_PREDIV_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_DIV_INT, CS42L42_PLL_DIV_INT_MASK, (p.pll_div_int as c_uint) << CS42L42_PLL_DIV_INT_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_DIV_FRAC0, CS42L42_PLL_DIV_FRAC_MASK, CS42L42_FRAC0_VAL(p.pll_div_frac) << CS42L42_PLL_DIV_FRAC_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_DIV_FRAC1, CS42L42_PLL_DIV_FRAC_MASK, CS42L42_FRAC1_VAL(p.pll_div_frac) << CS42L42_PLL_DIV_FRAC_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_DIV_FRAC2, CS42L42_PLL_DIV_FRAC_MASK, CS42L42_FRAC2_VAL(p.pll_div_frac) << CS42L42_PLL_DIV_FRAC_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_CTL4, CS42L42_PLL_MODE_MASK, (p.pll_mode as c_uint) << CS42L42_PLL_MODE_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_CTL3, CS42L42_PLL_DIVOUT_MASK, ((p.pll_divout as c_uint) * (p.n as c_uint)) << CS42L42_PLL_DIVOUT_SHIFT);
                snd_soc_component_update_bits(component, CS42L42_PLL_CAL_RATIO, CS42L42_PLL_CAL_RATIO_MASK, (p.pll_cal_ratio as c_uint) << CS42L42_PLL_CAL_RATIO_SHIFT);
            }
            return 0;
        }
    }
    -EINVAL
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_pll_config, "SND_SOC_CS42L42_CORE");

pub unsafe extern "C" fn cs42l42_src_config(component: *mut snd_soc_component, sample_rate: c_uint) {
    let cs42l42 = snd_soc_component_get_drvdata(component);
    if (*cs42l42).stream_use != 0 {
        return;
    }
    let fs = if sample_rate <= 48000 { CS42L42_CLK_IASRC_SEL_6 } else { CS42L42_CLK_IASRC_SEL_12 };
    snd_soc_component_update_bits(component, CS42L42_FS_RATE_EN, CS42L42_FS_EN_MASK,
        (CS42L42_FS_EN_IASRC_96K | CS42L42_FS_EN_OASRC_96K) << CS42L42_FS_EN_SHIFT);
    snd_soc_component_update_bits(component, CS42L42_IN_ASRC_CLK, CS42L42_CLK_IASRC_SEL_MASK, fs << CS42L42_CLK_IASRC_SEL_SHIFT);
    snd_soc_component_update_bits(component, CS42L42_OUT_ASRC_CLK, CS42L42_CLK_OASRC_SEL_MASK, fs << CS42L42_CLK_OASRC_SEL_SHIFT);
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_src_config, "SND_SOC_CS42L42_CORE");

unsafe extern "C" fn cs42l42_asp_config(component: *mut snd_soc_component, sclk: c_uint, sample_rate: c_uint) -> c_int {
    let mut fsync = sclk / sample_rate;
    if fsync * sample_rate != sclk || fsync % 2 != 0 {
        dev_err((*component).dev, CSTR!("Unsupported sclk %d/sample rate %d\n"), sclk, sample_rate);
        return -EINVAL;
    }
    snd_soc_component_update_bits(component, CS42L42_FSYNC_P_LOWER, CS42L42_FSYNC_PERIOD_MASK, CS42L42_FRAC0_VAL(fsync - 1) << CS42L42_FSYNC_PERIOD_SHIFT);
    snd_soc_component_update_bits(component, CS42L42_FSYNC_P_UPPER, CS42L42_FSYNC_PERIOD_MASK, CS42L42_FRAC1_VAL(fsync - 1) << CS42L42_FSYNC_PERIOD_SHIFT);
    fsync /= 2;
    snd_soc_component_update_bits(component, CS42L42_FSYNC_PW_LOWER, CS42L42_FSYNC_PULSE_WIDTH_MASK, CS42L42_FRAC0_VAL(fsync - 1) << CS42L42_FSYNC_PULSE_WIDTH_SHIFT);
    snd_soc_component_update_bits(component, CS42L42_FSYNC_PW_UPPER, CS42L42_FSYNC_PULSE_WIDTH_MASK, CS42L42_FRAC1_VAL(fsync - 1) << CS42L42_FSYNC_PULSE_WIDTH_SHIFT);
    0
}

unsafe extern "C" fn cs42l42_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let mut asp_cfg_val: u32 = 0;
    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFP => asp_cfg_val |= CS42L42_ASP_MASTER_MODE << CS42L42_ASP_MODE_SHIFT,
        SND_SOC_DAIFMT_CBC_CFC => asp_cfg_val |= CS42L42_ASP_SLAVE_MODE << CS42L42_ASP_MODE_SHIFT,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            snd_soc_component_update_bits(component, CS42L42_ASP_FRM_CFG,
                CS42L42_ASP_STP_MASK | CS42L42_ASP_5050_MASK | CS42L42_ASP_FSD_MASK,
                CS42L42_ASP_5050_MASK | (CS42L42_ASP_FSD_1_0 << CS42L42_ASP_FSD_SHIFT));
        }
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => asp_cfg_val |= CS42L42_ASP_SCPOL_NOR << CS42L42_ASP_SCPOL_SHIFT,
        SND_SOC_DAIFMT_NB_IF => {
            asp_cfg_val |= CS42L42_ASP_SCPOL_NOR << CS42L42_ASP_SCPOL_SHIFT;
            asp_cfg_val |= CS42L42_ASP_LCPOL_INV << CS42L42_ASP_LCPOL_SHIFT;
        }
        SND_SOC_DAIFMT_IB_NF => {}
        SND_SOC_DAIFMT_IB_IF => asp_cfg_val |= CS42L42_ASP_LCPOL_INV << CS42L42_ASP_LCPOL_SHIFT,
        _ => {}
    }
    snd_soc_component_update_bits(component, CS42L42_ASP_CLK_CFG,
        CS42L42_ASP_MODE_MASK | CS42L42_ASP_SCPOL_MASK | CS42L42_ASP_LCPOL_MASK, asp_cfg_val);
    0
}

unsafe extern "C" fn cs42l42_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l42 = snd_soc_component_get_drvdata(component);
    if (*cs42l42).sclk != 0 {
        return 0;
    }
    snd_pcm_hw_constraint_minmax((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 44100, 96000)
}

unsafe extern "C" fn cs42l42_pcm_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l42 = snd_soc_component_get_drvdata(component);
    let channels = params_channels(params);
    let width = (params_width(params) / 8) - 1;
    let sample_rate = params_rate(params);
    let mut slot_width = 0;
    let mut val = 0;
    let bclk;
    if (*cs42l42).bclk_ratio != 0 {
        bclk = (*cs42l42).bclk_ratio * params_rate(params);
    } else if (*cs42l42).sclk != 0 {
        bclk = (*cs42l42).sclk;
    } else {
        if params_width(params) == 24 {
            slot_width = 32;
        }
        bclk = snd_soc_tdm_params_to_bclk(params, slot_width, 0, 2);
    }
    match (*substream).stream {
        SNDRV_PCM_STREAM_CAPTURE => {
            val = CS42L42_ASP_TX_CH2_AP_MASK | (width << CS42L42_ASP_TX_CH2_RES_SHIFT) | (width << CS42L42_ASP_TX_CH1_RES_SHIFT);
            snd_soc_component_update_bits(component, CS42L42_ASP_TX_CH_AP_RES,
                CS42L42_ASP_TX_CH1_AP_MASK | CS42L42_ASP_TX_CH2_AP_MASK | CS42L42_ASP_TX_CH2_RES_MASK | CS42L42_ASP_TX_CH1_RES_MASK, val);
        }
        SNDRV_PCM_STREAM_PLAYBACK => {
            val |= width << CS42L42_ASP_RX_CH_RES_SHIFT;
            snd_soc_component_update_bits(component, CS42L42_ASP_RX_DAI0_CH1_AP_RES, CS42L42_ASP_RX_CH_AP_MASK | CS42L42_ASP_RX_CH_RES_MASK, val);
            val |= CS42L42_ASP_RX_CH_AP_HI << CS42L42_ASP_RX_CH_AP_SHIFT;
            snd_soc_component_update_bits(component, CS42L42_ASP_RX_DAI0_CH2_AP_RES, CS42L42_ASP_RX_CH_AP_MASK | CS42L42_ASP_RX_CH_RES_MASK, val);
            snd_soc_component_update_bits(component, CS42L42_SP_RX_CH_SEL, CS42L42_SP_RX_CHB_SEL_MASK, (channels - 1) << CS42L42_SP_RX_CHB_SEL_SHIFT);
            snd_soc_component_update_bits(component, CS42L42_ASP_RX_DAI0_EN, CS42L42_ASP_RX0_CH_EN_MASK,
                BIT!(CS42L42_ASP_RX0_CH1_SHIFT) | BIT!(CS42L42_ASP_RX0_CH2_SHIFT));
        }
        _ => {}
    }
    let mut ret = cs42l42_pll_config(component, bclk, sample_rate);
    if ret != 0 { return ret; }
    ret = cs42l42_asp_config(component, bclk, sample_rate);
    if ret != 0 { return ret; }
    cs42l42_src_config(component, sample_rate);
    0
}

unsafe extern "C" fn cs42l42_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let cs42l42 = snd_soc_component_get_drvdata(component);
    if freq == 0 {
        (*cs42l42).sclk = 0;
        return 0;
    }
    for p in pll_ratio_table.iter() {
        if p.sclk == freq {
            (*cs42l42).sclk = freq;
            return 0;
        }
    }
    dev_err((*component).dev, CSTR!("SCLK %u not supported\n"), freq);
    -EINVAL
}

unsafe extern "C" fn cs42l42_set_bclk_ratio(dai: *mut snd_soc_dai, bclk_ratio: c_uint) -> c_int {
    let component = (*dai).component;
    let cs42l42 = snd_soc_component_get_drvdata(component);
    (*cs42l42).bclk_ratio = bclk_ratio;
    0
}

pub unsafe extern "C" fn cs42l42_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int {
    let component = (*dai).component;
    let cs42l42 = snd_soc_component_get_drvdata(component);
    let mut regval: c_uint = 0;
    let mut ret: c_int;
    if mute != 0 {
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            snd_soc_component_update_bits(component, CS42L42_HP_CTL,
                CS42L42_HP_ANA_AMUTE_MASK | CS42L42_HP_ANA_BMUTE_MASK,
                CS42L42_HP_ANA_AMUTE_MASK | CS42L42_HP_ANA_BMUTE_MASK);
        }
        (*cs42l42).stream_use &= !(1 << stream);
        if (*cs42l42).stream_use == 0 {
            regmap_multi_reg_write((*cs42l42).regmap, cs42l42_to_osc_seq.as_ptr(), ARRAY_SIZE!(cs42l42_to_osc_seq));
            snd_soc_component_update_bits(component, CS42L42_MCLK_SRC_SEL, CS42L42_MCLK_SRC_SEL_MASK, 0);
            usleep_range(100, 200);
            snd_soc_component_update_bits(component, CS42L42_PLL_CTL1, CS42L42_PLL_START_MASK, 0);
        }
    } else {
        if (*cs42l42).stream_use == 0 {
            let p = pll_ratio_table[(*cs42l42).pll_config as usize];
            if p.mclk_src_sel != 0 {
                snd_soc_component_update_bits(component, CS42L42_PLL_CTL1, CS42L42_PLL_START_MASK, 1);
                if p.n > 1 {
                    usleep_range(CS42L42_PLL_DIVOUT_TIME_US, CS42L42_PLL_DIVOUT_TIME_US * 2);
                    regval = p.pll_divout as c_uint;
                    snd_soc_component_update_bits(component, CS42L42_PLL_CTL3, CS42L42_PLL_DIVOUT_MASK, regval << CS42L42_PLL_DIVOUT_SHIFT);
                }
                ret = regmap_read_poll_timeout((*cs42l42).regmap, CS42L42_PLL_LOCK_STATUS, &mut regval, (regval & 1) != 0, CS42L42_PLL_LOCK_POLL_US, CS42L42_PLL_LOCK_TIMEOUT_US);
                if ret < 0 {
                    dev_warn((*component).dev, CSTR!("PLL failed to lock: %d\n"), ret);
                }
                snd_soc_component_update_bits(component, CS42L42_MCLK_SRC_SEL, CS42L42_MCLK_SRC_SEL_MASK, CS42L42_MCLK_SRC_SEL_MASK);
            }
            regmap_multi_reg_write((*cs42l42).regmap, cs42l42_to_sclk_seq.as_ptr(), ARRAY_SIZE!(cs42l42_to_sclk_seq));
        }
        (*cs42l42).stream_use |= 1 << stream;
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            snd_soc_component_update_bits(component, CS42L42_HP_CTL,
                CS42L42_HP_ANA_AMUTE_MASK | CS42L42_HP_ANA_BMUTE_MASK, 0);
        }
    }
    0
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_mute_stream, "SND_SOC_CS42L42_CORE");

const CS42L42_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static cs42l42_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs42l42_dai_startup),
    hw_params: Some(cs42l42_pcm_hw_params),
    set_fmt: Some(cs42l42_set_dai_fmt),
    set_sysclk: Some(cs42l42_set_sysclk),
    set_bclk_ratio: Some(cs42l42_set_bclk_ratio),
    mute_stream: Some(cs42l42_mute_stream),
};

pub static mut cs42l42_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: CSTR!("cs42l42"),
    playback: snd_soc_pcm_stream {
        stream_name: CSTR!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: CS42L42_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: CSTR!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: CS42L42_FORMATS,
    },
    symmetric_rate: 1,
    symmetric_sample_bits: 1,
    ops: &cs42l42_ops,
};
EXPORT_SYMBOL_NS_GPL!(cs42l42_dai, "SND_SOC_CS42L42_CORE");

unsafe fn cs42l42_manual_hs_type_detect(cs42l42: *mut cs42l42_private) {
    let mut hs_det_status: c_uint = 0;
    let mut hs_det_sw: c_uint;
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (1 << CS42L42_HSDET_CTRL_SHIFT) | (0 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (0 << CS42L42_HSDET_AUTO_TIME_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL1,
        CS42L42_HSDET_COMP1_LVL_MASK | CS42L42_HSDET_COMP2_LVL_MASK,
        (CS42L42_HSDET_COMP1_LVL_VAL << CS42L42_HSDET_COMP1_LVL_SHIFT) | (CS42L42_HSDET_COMP2_LVL_VAL << CS42L42_HSDET_COMP2_LVL_SHIFT));
    regmap_write((*cs42l42).regmap, CS42L42_HS_SWITCH_CTL, CS42L42_HSDET_SW_COMP1);
    msleep(100);
    regmap_read((*cs42l42).regmap, CS42L42_HS_DET_STATUS, &mut hs_det_status);
    let mut hs_det_comp1 = (hs_det_status & CS42L42_HSDET_COMP1_OUT_MASK) >> CS42L42_HSDET_COMP1_OUT_SHIFT;
    let mut hs_det_comp2 = (hs_det_status & CS42L42_HSDET_COMP2_OUT_MASK) >> CS42L42_HSDET_COMP2_OUT_SHIFT;
    regmap_write((*cs42l42).regmap, CS42L42_HS_SWITCH_CTL, CS42L42_HSDET_SW_COMP2);
    msleep(100);
    regmap_read((*cs42l42).regmap, CS42L42_HS_DET_STATUS, &mut hs_det_status);
    hs_det_comp1 |= ((hs_det_status & CS42L42_HSDET_COMP1_OUT_MASK) >> CS42L42_HSDET_COMP1_OUT_SHIFT) << 1;
    hs_det_comp2 |= ((hs_det_status & CS42L42_HSDET_COMP2_OUT_MASK) >> CS42L42_HSDET_COMP2_OUT_SHIFT) << 1;
    match hs_det_comp1 {
        CS42L42_HSDET_COMP_TYPE1 => { (*cs42l42).hs_type = CS42L42_PLUG_CTIA; hs_det_sw = CS42L42_HSDET_SW_TYPE1; }
        CS42L42_HSDET_COMP_TYPE2 => { (*cs42l42).hs_type = CS42L42_PLUG_OMTP; hs_det_sw = CS42L42_HSDET_SW_TYPE2; }
        _ => match hs_det_comp2 {
            CS42L42_HSDET_COMP_TYPE1 => { (*cs42l42).hs_type = CS42L42_PLUG_CTIA; hs_det_sw = CS42L42_HSDET_SW_TYPE1; }
            CS42L42_HSDET_COMP_TYPE2 => { (*cs42l42).hs_type = CS42L42_PLUG_OMTP; hs_det_sw = CS42L42_HSDET_SW_TYPE2; }
            _ => { (*cs42l42).hs_type = CS42L42_PLUG_HEADPHONE; hs_det_sw = CS42L42_HSDET_SW_TYPE3; }
        }
    }
    regmap_write((*cs42l42).regmap, CS42L42_HS_SWITCH_CTL, hs_det_sw);
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (0 << CS42L42_HSDET_CTRL_SHIFT) | (0 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (0 << CS42L42_HSDET_AUTO_TIME_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL1,
        CS42L42_HSDET_COMP1_LVL_MASK | CS42L42_HSDET_COMP2_LVL_MASK,
        (CS42L42_HSDET_COMP1_LVL_DEFAULT << CS42L42_HSDET_COMP1_LVL_SHIFT) | (CS42L42_HSDET_COMP2_LVL_DEFAULT << CS42L42_HSDET_COMP2_LVL_SHIFT));
}

unsafe fn cs42l42_process_hs_type_detect(cs42l42: *mut cs42l42_private) {
    let mut hs_det_status = 0;
    let mut int_status = 0;
    regmap_read((*cs42l42).regmap, CS42L42_HS_DET_STATUS, &mut hs_det_status);
    regmap_update_bits((*cs42l42).regmap, CS42L42_CODEC_INT_MASK,
        CS42L42_PDN_DONE_MASK | CS42L42_HSDET_AUTO_DONE_MASK,
        (1 << CS42L42_PDN_DONE_SHIFT) | (1 << CS42L42_HSDET_AUTO_DONE_SHIFT));
    (*cs42l42).hs_type = (hs_det_status & CS42L42_HSDET_TYPE_MASK) >> CS42L42_HSDET_TYPE_SHIFT;
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (2 << CS42L42_HSDET_CTRL_SHIFT) | (2 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (3 << CS42L42_HSDET_AUTO_TIME_SHIFT));
    if (*cs42l42).hs_type == CS42L42_PLUG_INVALID || (*cs42l42).hs_type == CS42L42_PLUG_HEADPHONE {
        dev_dbg((*cs42l42).dev, CSTR!("Running Manual Detection Fallback\n"));
        cs42l42_manual_hs_type_detect(cs42l42);
    }
    if (*cs42l42).hs_type == CS42L42_PLUG_CTIA || (*cs42l42).hs_type == CS42L42_PLUG_OMTP {
        regmap_update_bits((*cs42l42).regmap, CS42L42_HSBIAS_SC_AUTOCTL,
            CS42L42_HSBIAS_SENSE_EN_MASK | CS42L42_AUTO_HSBIAS_HIZ_MASK | CS42L42_TIP_SENSE_EN_MASK | CS42L42_HSBIAS_SENSE_TRIP_MASK,
            (0 << CS42L42_HSBIAS_SENSE_EN_SHIFT) | (0 << CS42L42_AUTO_HSBIAS_HIZ_SHIFT) | (0 << CS42L42_TIP_SENSE_EN_SHIFT) | (3 << CS42L42_HSBIAS_SENSE_TRIP_SHIFT));
        regmap_update_bits((*cs42l42).regmap, CS42L42_MIC_DET_CTL1,
            CS42L42_LATCH_TO_VP_MASK | CS42L42_EVENT_STAT_SEL_MASK | CS42L42_HS_DET_LEVEL_MASK,
            (1 << CS42L42_LATCH_TO_VP_SHIFT) | (0 << CS42L42_EVENT_STAT_SEL_SHIFT) | ((*cs42l42).bias_thresholds[0] << CS42L42_HS_DET_LEVEL_SHIFT));
        regmap_update_bits((*cs42l42).regmap, CS42L42_HSBIAS_SC_AUTOCTL,
            CS42L42_HSBIAS_SENSE_EN_MASK | CS42L42_AUTO_HSBIAS_HIZ_MASK | CS42L42_TIP_SENSE_EN_MASK | CS42L42_HSBIAS_SENSE_TRIP_MASK,
            ((*cs42l42).hs_bias_sense_en << CS42L42_HSBIAS_SENSE_EN_SHIFT) | (1 << CS42L42_AUTO_HSBIAS_HIZ_SHIFT) | (0 << CS42L42_TIP_SENSE_EN_SHIFT) | (3 << CS42L42_HSBIAS_SENSE_TRIP_SHIFT));
        regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL,
            CS42L42_HSBIAS_CTL_MASK | CS42L42_PDN_MIC_LVL_DET_MASK,
            (3 << CS42L42_HSBIAS_CTL_SHIFT) | (0 << CS42L42_PDN_MIC_LVL_DET_SHIFT));
        msleep((*cs42l42).btn_det_init_dbnce);
        regmap_read((*cs42l42).regmap, CS42L42_DET_INT_STATUS2, &mut int_status);
        regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT2_MASK,
            CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK | CS42L42_M_SHORT_RLS_MASK | CS42L42_M_SHORT_DET_MASK,
            (0 << CS42L42_M_DETECT_TF_SHIFT) | (0 << CS42L42_M_DETECT_FT_SHIFT) | (0 << CS42L42_M_HSBIAS_HIZ_SHIFT) | (1 << CS42L42_M_SHORT_RLS_SHIFT) | (1 << CS42L42_M_SHORT_DET_SHIFT));
    } else {
        regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL,
            CS42L42_HSBIAS_CTL_MASK | CS42L42_PDN_MIC_LVL_DET_MASK,
            (1 << CS42L42_HSBIAS_CTL_SHIFT) | (1 << CS42L42_PDN_MIC_LVL_DET_SHIFT));
    }
    regmap_update_bits((*cs42l42).regmap, CS42L42_DAC_CTL2,
        CS42L42_HPOUT_PULLDOWN_MASK | CS42L42_HPOUT_LOAD_MASK | CS42L42_HPOUT_CLAMP_MASK | CS42L42_DAC_HPF_EN_MASK | CS42L42_DAC_MON_EN_MASK,
        (0 << CS42L42_HPOUT_PULLDOWN_SHIFT) | (0 << CS42L42_HPOUT_LOAD_SHIFT) | (0 << CS42L42_HPOUT_CLAMP_SHIFT) | (1 << CS42L42_DAC_HPF_EN_SHIFT) | (0 << CS42L42_DAC_MON_EN_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSRS_PLUG_INT_MASK,
        CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK,
        (0 << CS42L42_TS_PLUG_SHIFT) | (0 << CS42L42_TS_UNPLUG_SHIFT));
}

unsafe fn cs42l42_init_hs_type_detect(cs42l42: *mut cs42l42_private) {
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSRS_PLUG_INT_MASK,
        CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK,
        (1 << CS42L42_TS_PLUG_SHIFT) | (1 << CS42L42_TS_UNPLUG_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL,
        CS42L42_HSBIAS_CTL_MASK | CS42L42_PDN_MIC_LVL_DET_MASK,
        (1 << CS42L42_HSBIAS_CTL_SHIFT) | (1 << CS42L42_PDN_MIC_LVL_DET_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSBIAS_SC_AUTOCTL,
        CS42L42_HSBIAS_SENSE_EN_MASK | CS42L42_AUTO_HSBIAS_HIZ_MASK | CS42L42_TIP_SENSE_EN_MASK | CS42L42_HSBIAS_SENSE_TRIP_MASK,
        (0 << CS42L42_HSBIAS_SENSE_EN_SHIFT) | (0 << CS42L42_AUTO_HSBIAS_HIZ_SHIFT) | (0 << CS42L42_TIP_SENSE_EN_SHIFT) | (3 << CS42L42_HSBIAS_SENSE_TRIP_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (0 << CS42L42_HSDET_CTRL_SHIFT) | (2 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (3 << CS42L42_HSDET_AUTO_TIME_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_DAC_CTL2,
        CS42L42_HPOUT_PULLDOWN_MASK | CS42L42_HPOUT_LOAD_MASK | CS42L42_HPOUT_CLAMP_MASK | CS42L42_DAC_HPF_EN_MASK | CS42L42_DAC_MON_EN_MASK,
        (8 << CS42L42_HPOUT_PULLDOWN_SHIFT) | (0 << CS42L42_HPOUT_LOAD_SHIFT) | (1 << CS42L42_HPOUT_CLAMP_SHIFT) | (1 << CS42L42_DAC_HPF_EN_SHIFT) | (1 << CS42L42_DAC_MON_EN_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL,
        CS42L42_HSBIAS_CTL_MASK | CS42L42_PDN_MIC_LVL_DET_MASK,
        (3 << CS42L42_HSBIAS_CTL_SHIFT) | (1 << CS42L42_PDN_MIC_LVL_DET_SHIFT));
    msleep((*cs42l42).hs_bias_ramp_time);
    regmap_update_bits((*cs42l42).regmap, CS42L42_CODEC_INT_MASK,
        CS42L42_PDN_DONE_MASK | CS42L42_HSDET_AUTO_DONE_MASK,
        (1 << CS42L42_PDN_DONE_SHIFT) | (0 << CS42L42_HSDET_AUTO_DONE_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (3 << CS42L42_HSDET_CTRL_SHIFT) | (2 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (3 << CS42L42_HSDET_AUTO_TIME_SHIFT));
}

unsafe fn cs42l42_cancel_hs_type_detect(cs42l42: *mut cs42l42_private) {
    regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT2_MASK,
        CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK | CS42L42_M_SHORT_RLS_MASK | CS42L42_M_SHORT_DET_MASK,
        (1 << CS42L42_M_DETECT_TF_SHIFT) | (1 << CS42L42_M_DETECT_FT_SHIFT) | (1 << CS42L42_M_HSBIAS_HIZ_SHIFT) | (1 << CS42L42_M_SHORT_RLS_SHIFT) | (1 << CS42L42_M_SHORT_DET_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL,
        CS42L42_HSBIAS_CTL_MASK | CS42L42_PDN_MIC_LVL_DET_MASK,
        (1 << CS42L42_HSBIAS_CTL_SHIFT) | (1 << CS42L42_PDN_MIC_LVL_DET_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSBIAS_SC_AUTOCTL,
        CS42L42_HSBIAS_SENSE_EN_MASK | CS42L42_AUTO_HSBIAS_HIZ_MASK | CS42L42_TIP_SENSE_EN_MASK | CS42L42_HSBIAS_SENSE_TRIP_MASK,
        (0 << CS42L42_HSBIAS_SENSE_EN_SHIFT) | (0 << CS42L42_AUTO_HSBIAS_HIZ_SHIFT) | (0 << CS42L42_TIP_SENSE_EN_SHIFT) | (3 << CS42L42_HSBIAS_SENSE_TRIP_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HSDET_CTL2,
        CS42L42_HSDET_CTRL_MASK | CS42L42_HSDET_SET_MASK | CS42L42_HSBIAS_REF_MASK | CS42L42_HSDET_AUTO_TIME_MASK,
        (0 << CS42L42_HSDET_CTRL_SHIFT) | (2 << CS42L42_HSDET_SET_SHIFT) | (0 << CS42L42_HSBIAS_REF_SHIFT) | (3 << CS42L42_HSDET_AUTO_TIME_SHIFT));
}

unsafe fn cs42l42_handle_button_press(cs42l42: *mut cs42l42_private) -> c_int {
    let mut detect_status = 0;
    regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT2_MASK,
        CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK | CS42L42_M_SHORT_RLS_MASK | CS42L42_M_SHORT_DET_MASK,
        (1 << CS42L42_M_DETECT_TF_SHIFT) | (1 << CS42L42_M_DETECT_FT_SHIFT) | (1 << CS42L42_M_HSBIAS_HIZ_SHIFT) | (1 << CS42L42_M_SHORT_RLS_SHIFT) | (1 << CS42L42_M_SHORT_DET_SHIFT));
    usleep_range((*cs42l42).btn_det_event_dbnce * 1000, (*cs42l42).btn_det_event_dbnce * 2000);
    let mut bias_level: c_int = 1;
    loop {
        regmap_update_bits((*cs42l42).regmap, CS42L42_MIC_DET_CTL1,
            CS42L42_LATCH_TO_VP_MASK | CS42L42_EVENT_STAT_SEL_MASK | CS42L42_HS_DET_LEVEL_MASK,
            (1 << CS42L42_LATCH_TO_VP_SHIFT) | (0 << CS42L42_EVENT_STAT_SEL_SHIFT) |
            ((*cs42l42).bias_thresholds[bias_level as usize] << CS42L42_HS_DET_LEVEL_SHIFT));
        regmap_read((*cs42l42).regmap, CS42L42_DET_STATUS2, &mut detect_status);
        if (detect_status & CS42L42_HS_TRUE_MASK) == 0 {
            break;
        }
        bias_level += 1;
        if bias_level >= CS42L42_NUM_BIASES as c_int {
            break;
        }
    }
    bias_level = match bias_level {
        1 => { dev_dbg((*cs42l42).dev, CSTR!("Function C button press\n")); SND_JACK_BTN_2 }
        2 => { dev_dbg((*cs42l42).dev, CSTR!("Function B button press\n")); SND_JACK_BTN_1 }
        3 => { dev_dbg((*cs42l42).dev, CSTR!("Function D button press\n")); SND_JACK_BTN_3 }
        4 => { dev_dbg((*cs42l42).dev, CSTR!("Function A button press\n")); SND_JACK_BTN_0 }
        _ => 0,
    };
    regmap_update_bits((*cs42l42).regmap, CS42L42_MIC_DET_CTL1,
        CS42L42_LATCH_TO_VP_MASK | CS42L42_EVENT_STAT_SEL_MASK | CS42L42_HS_DET_LEVEL_MASK,
        (1 << CS42L42_LATCH_TO_VP_SHIFT) | (0 << CS42L42_EVENT_STAT_SEL_SHIFT) | ((*cs42l42).bias_thresholds[0] << CS42L42_HS_DET_LEVEL_SHIFT));
    regmap_read((*cs42l42).regmap, CS42L42_DET_INT_STATUS2, &mut detect_status);
    regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT2_MASK,
        CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK | CS42L42_M_SHORT_RLS_MASK | CS42L42_M_SHORT_DET_MASK,
        (0 << CS42L42_M_DETECT_TF_SHIFT) | (0 << CS42L42_M_DETECT_FT_SHIFT) | (0 << CS42L42_M_HSBIAS_HIZ_SHIFT) | (1 << CS42L42_M_SHORT_RLS_SHIFT) | (1 << CS42L42_M_SHORT_DET_SHIFT));
    bias_level
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cs42l42_irq_params {
    status_addr: u16,
    mask_addr: u16,
    mask: u8,
}

static irq_params_table: [cs42l42_irq_params; 12] = [
    cs42l42_irq_params { status_addr: CS42L42_ADC_OVFL_STATUS as u16, mask_addr: CS42L42_ADC_OVFL_INT_MASK as u16, mask: CS42L42_ADC_OVFL_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_MIXER_STATUS as u16, mask_addr: CS42L42_MIXER_INT_MASK as u16, mask: CS42L42_MIXER_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_SRC_STATUS as u16, mask_addr: CS42L42_SRC_INT_MASK as u16, mask: CS42L42_SRC_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_ASP_RX_STATUS as u16, mask_addr: CS42L42_ASP_RX_INT_MASK as u16, mask: CS42L42_ASP_RX_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_ASP_TX_STATUS as u16, mask_addr: CS42L42_ASP_TX_INT_MASK as u16, mask: CS42L42_ASP_TX_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_CODEC_STATUS as u16, mask_addr: CS42L42_CODEC_INT_MASK as u16, mask: CS42L42_CODEC_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_DET_INT_STATUS1 as u16, mask_addr: CS42L42_DET_INT1_MASK as u16, mask: CS42L42_DET_INT_VAL1_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_DET_INT_STATUS2 as u16, mask_addr: CS42L42_DET_INT2_MASK as u16, mask: CS42L42_DET_INT_VAL2_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_SRCPL_INT_STATUS as u16, mask_addr: CS42L42_SRCPL_INT_MASK as u16, mask: CS42L42_SRCPL_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_VPMON_STATUS as u16, mask_addr: CS42L42_VPMON_INT_MASK as u16, mask: CS42L42_VPMON_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_PLL_LOCK_STATUS as u16, mask_addr: CS42L42_PLL_LOCK_INT_MASK as u16, mask: CS42L42_PLL_LOCK_VAL_MASK as u8 },
    cs42l42_irq_params { status_addr: CS42L42_TSRS_PLUG_STATUS as u16, mask_addr: CS42L42_TSRS_PLUG_INT_MASK as u16, mask: CS42L42_TSRS_PLUG_VAL_MASK as u8 },
];

pub unsafe extern "C" fn cs42l42_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs42l42 = data as *mut cs42l42_private;
    if (*cs42l42).suspended || !(*cs42l42).init_done {
        return IRQ_NONE;
    }
    let mut stickies = [0u32; 12];
    let mut masks = [0u32; 12];
    for i in 0..stickies.len() {
        regmap_read((*cs42l42).regmap, irq_params_table[i].status_addr as c_uint, &mut stickies[i]);
        regmap_read((*cs42l42).regmap, irq_params_table[i].mask_addr as c_uint, &mut masks[i]);
        stickies[i] = stickies[i] & !masks[i] & irq_params_table[i].mask as u32;
    }
    let current_plug_status = (stickies[11] & (CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK)) >> CS42L42_TS_PLUG_SHIFT;
    let current_button_status = stickies[7] & (CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK);
    if (!masks[5] & irq_params_table[5].mask as u32) != 0 && (stickies[5] & CS42L42_HSDET_AUTO_DONE_MASK) != 0 {
        cs42l42_process_hs_type_detect(cs42l42);
        match (*cs42l42).hs_type {
            CS42L42_PLUG_CTIA | CS42L42_PLUG_OMTP => snd_soc_jack_report((*cs42l42).jack, SND_JACK_HEADSET, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3),
            CS42L42_PLUG_HEADPHONE => snd_soc_jack_report((*cs42l42).jack, SND_JACK_HEADPHONE, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3),
            _ => {}
        }
        dev_dbg((*cs42l42).dev, CSTR!("Auto detect done (%d)\n"), (*cs42l42).hs_type);
    }
    if (!masks[11] & irq_params_table[11].mask as u32) != 0 {
        match current_plug_status {
            CS42L42_TS_PLUG => {
                if (*cs42l42).plug_state != CS42L42_TS_PLUG {
                    (*cs42l42).plug_state = CS42L42_TS_PLUG;
                    cs42l42_init_hs_type_detect(cs42l42);
                }
            }
            CS42L42_TS_UNPLUG => {
                if (*cs42l42).plug_state != CS42L42_TS_UNPLUG {
                    (*cs42l42).plug_state = CS42L42_TS_UNPLUG;
                    cs42l42_cancel_hs_type_detect(cs42l42);
                    snd_soc_jack_report((*cs42l42).jack, 0, SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
                    dev_dbg((*cs42l42).dev, CSTR!("Unplug event\n"));
                }
            }
            _ => (*cs42l42).plug_state = CS42L42_TS_TRANS,
        }
    }
    if (*cs42l42).plug_state == CS42L42_TS_PLUG && (!masks[7] & irq_params_table[7].mask as u32) != 0 {
        if (current_button_status & CS42L42_M_HSBIAS_HIZ_MASK) == 0 {
            if (current_button_status & CS42L42_M_DETECT_TF_MASK) != 0 {
                dev_dbg((*cs42l42).dev, CSTR!("Button released\n"));
                snd_soc_jack_report((*cs42l42).jack, 0, SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
            } else if (current_button_status & CS42L42_M_DETECT_FT_MASK) != 0 {
                snd_soc_jack_report((*cs42l42).jack, cs42l42_handle_button_press(cs42l42), SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3);
            }
        }
    }
    IRQ_HANDLED
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_irq_thread, "SND_SOC_CS42L42_CORE");

unsafe fn cs42l42_set_interrupt_masks(cs42l42: *mut cs42l42_private) {
    regmap_update_bits((*cs42l42).regmap, CS42L42_ADC_OVFL_INT_MASK, CS42L42_ADC_OVFL_MASK, 1 << CS42L42_ADC_OVFL_SHIFT);
    regmap_update_bits((*cs42l42).regmap, CS42L42_MIXER_INT_MASK,
        CS42L42_MIX_CHB_OVFL_MASK | CS42L42_MIX_CHA_OVFL_MASK | CS42L42_EQ_OVFL_MASK | CS42L42_EQ_BIQUAD_OVFL_MASK,
        (1 << CS42L42_MIX_CHB_OVFL_SHIFT) | (1 << CS42L42_MIX_CHA_OVFL_SHIFT) | (1 << CS42L42_EQ_OVFL_SHIFT) | (1 << CS42L42_EQ_BIQUAD_OVFL_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_SRC_INT_MASK,
        CS42L42_SRC_ILK_MASK | CS42L42_SRC_OLK_MASK | CS42L42_SRC_IUNLK_MASK | CS42L42_SRC_OUNLK_MASK,
        (1 << CS42L42_SRC_ILK_SHIFT) | (1 << CS42L42_SRC_OLK_SHIFT) | (1 << CS42L42_SRC_IUNLK_SHIFT) | (1 << CS42L42_SRC_OUNLK_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_ASP_RX_INT_MASK,
        CS42L42_ASPRX_NOLRCK_MASK | CS42L42_ASPRX_EARLY_MASK | CS42L42_ASPRX_LATE_MASK | CS42L42_ASPRX_ERROR_MASK | CS42L42_ASPRX_OVLD_MASK,
        (1 << CS42L42_ASPRX_NOLRCK_SHIFT) | (1 << CS42L42_ASPRX_EARLY_SHIFT) | (1 << CS42L42_ASPRX_LATE_SHIFT) | (1 << CS42L42_ASPRX_ERROR_SHIFT) | (1 << CS42L42_ASPRX_OVLD_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_ASP_TX_INT_MASK,
        CS42L42_ASPTX_NOLRCK_MASK | CS42L42_ASPTX_EARLY_MASK | CS42L42_ASPTX_LATE_MASK | CS42L42_ASPTX_SMERROR_MASK,
        (1 << CS42L42_ASPTX_NOLRCK_SHIFT) | (1 << CS42L42_ASPTX_EARLY_SHIFT) | (1 << CS42L42_ASPTX_LATE_SHIFT) | (1 << CS42L42_ASPTX_SMERROR_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_CODEC_INT_MASK,
        CS42L42_PDN_DONE_MASK | CS42L42_HSDET_AUTO_DONE_MASK,
        (1 << CS42L42_PDN_DONE_SHIFT) | (1 << CS42L42_HSDET_AUTO_DONE_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_SRCPL_INT_MASK,
        CS42L42_SRCPL_ADC_LK_MASK | CS42L42_SRCPL_DAC_LK_MASK | CS42L42_SRCPL_ADC_UNLK_MASK | CS42L42_SRCPL_DAC_UNLK_MASK,
        (1 << CS42L42_SRCPL_ADC_LK_SHIFT) | (1 << CS42L42_SRCPL_DAC_LK_SHIFT) | (1 << CS42L42_SRCPL_ADC_UNLK_SHIFT) | (1 << CS42L42_SRCPL_DAC_UNLK_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT1_MASK,
        CS42L42_TIP_SENSE_UNPLUG_MASK | CS42L42_TIP_SENSE_PLUG_MASK | CS42L42_HSBIAS_SENSE_MASK,
        (1 << CS42L42_TIP_SENSE_UNPLUG_SHIFT) | (1 << CS42L42_TIP_SENSE_PLUG_SHIFT) | (1 << CS42L42_HSBIAS_SENSE_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_DET_INT2_MASK,
        CS42L42_M_DETECT_TF_MASK | CS42L42_M_DETECT_FT_MASK | CS42L42_M_HSBIAS_HIZ_MASK | CS42L42_M_SHORT_RLS_MASK | CS42L42_M_SHORT_DET_MASK,
        (1 << CS42L42_M_DETECT_TF_SHIFT) | (1 << CS42L42_M_DETECT_FT_SHIFT) | (1 << CS42L42_M_HSBIAS_HIZ_SHIFT) | (1 << CS42L42_M_SHORT_RLS_SHIFT) | (1 << CS42L42_M_SHORT_DET_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_VPMON_INT_MASK, CS42L42_VPMON_MASK, 1 << CS42L42_VPMON_SHIFT);
    regmap_update_bits((*cs42l42).regmap, CS42L42_PLL_LOCK_INT_MASK, CS42L42_PLL_LOCK_MASK, 1 << CS42L42_PLL_LOCK_SHIFT);
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSRS_PLUG_INT_MASK,
        CS42L42_RS_PLUG_MASK | CS42L42_RS_UNPLUG_MASK | CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK,
        (1 << CS42L42_RS_PLUG_SHIFT) | (1 << CS42L42_RS_UNPLUG_SHIFT) | (0 << CS42L42_TS_PLUG_SHIFT) | (0 << CS42L42_TS_UNPLUG_SHIFT));
}

unsafe fn cs42l42_setup_hs_type_detect(cs42l42: *mut cs42l42_private) {
    let mut reg = 0;
    (*cs42l42).hs_type = CS42L42_PLUG_INVALID;
    regmap_update_bits((*cs42l42).regmap, CS42L42_MISC_DET_CTL, CS42L42_DETECT_MODE_MASK, 0);
    regmap_update_bits((*cs42l42).regmap, CS42L42_MIC_DET_CTL1,
        CS42L42_LATCH_TO_VP_MASK | CS42L42_EVENT_STAT_SEL_MASK | CS42L42_HS_DET_LEVEL_MASK,
        (1 << CS42L42_LATCH_TO_VP_SHIFT) | (0 << CS42L42_EVENT_STAT_SEL_SHIFT) | ((*cs42l42).bias_thresholds[0] << CS42L42_HS_DET_LEVEL_SHIFT));
    regmap_update_bits((*cs42l42).regmap, CS42L42_HS_CLAMP_DISABLE, CS42L42_HS_CLAMP_DISABLE_MASK, 1 << CS42L42_HS_CLAMP_DISABLE_SHIFT);
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSENSE_CTL, CS42L42_TS_INV_MASK, CS42L42_TS_INV_MASK);
    regmap_update_bits((*cs42l42).regmap, CS42L42_TIPSENSE_CTL,
        CS42L42_TIP_SENSE_CTRL_MASK | CS42L42_TIP_SENSE_INV_MASK | CS42L42_TIP_SENSE_DEBOUNCE_MASK,
        (3 << CS42L42_TIP_SENSE_CTRL_SHIFT) | (((*cs42l42).ts_inv == 0) as c_uint) << CS42L42_TIP_SENSE_INV_SHIFT | (2 << CS42L42_TIP_SENSE_DEBOUNCE_SHIFT));
    regmap_read((*cs42l42).regmap, CS42L42_TSRS_PLUG_STATUS, &mut reg);
    (*cs42l42).plug_state = ((reg as i8 as c_uint) & (CS42L42_TS_PLUG_MASK | CS42L42_TS_UNPLUG_MASK)) >> CS42L42_TS_PLUG_SHIFT;
}

static threshold_defaults: [c_uint; 4] = [
    CS42L42_HS_DET_LEVEL_15, CS42L42_HS_DET_LEVEL_8, CS42L42_HS_DET_LEVEL_4, CS42L42_HS_DET_LEVEL_1,
];

unsafe fn cs42l42_handle_device_data(dev: *mut device, cs42l42: *mut cs42l42_private) -> c_int {
    let mut val = 0;
    let mut thresholds = [0u32; 4];
    let mut ret = device_property_read_u32(dev, CSTR!("cirrus,ts-inv"), &mut val);
    if ret == 0 {
        match val {
            CS42L42_TS_INV_EN | CS42L42_TS_INV_DIS => (*cs42l42).ts_inv = val,
            _ => { dev_err(dev, CSTR!("Wrong cirrus,ts-inv DT value %d\n"), val); (*cs42l42).ts_inv = CS42L42_TS_INV_DIS; }
        }
    } else {
        (*cs42l42).ts_inv = CS42L42_TS_INV_DIS;
    }
    ret = device_property_read_u32(dev, CSTR!("cirrus,ts-dbnc-rise"), &mut val);
    if ret == 0 {
        match val {
            CS42L42_TS_DBNCE_0 | CS42L42_TS_DBNCE_125 | CS42L42_TS_DBNCE_250 | CS42L42_TS_DBNCE_500 |
            CS42L42_TS_DBNCE_750 | CS42L42_TS_DBNCE_1000 | CS42L42_TS_DBNCE_1250 | CS42L42_TS_DBNCE_1500 => (*cs42l42).ts_dbnc_rise = val,
            _ => { dev_err(dev, CSTR!("Wrong cirrus,ts-dbnc-rise DT value %d\n"), val); (*cs42l42).ts_dbnc_rise = CS42L42_TS_DBNCE_1000; }
        }
    } else { (*cs42l42).ts_dbnc_rise = CS42L42_TS_DBNCE_1000; }
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSENSE_CTL, CS42L42_TS_RISE_DBNCE_TIME_MASK, (*cs42l42).ts_dbnc_rise << CS42L42_TS_RISE_DBNCE_TIME_SHIFT);
    ret = device_property_read_u32(dev, CSTR!("cirrus,ts-dbnc-fall"), &mut val);
    if ret == 0 {
        match val {
            CS42L42_TS_DBNCE_0 | CS42L42_TS_DBNCE_125 | CS42L42_TS_DBNCE_250 | CS42L42_TS_DBNCE_500 |
            CS42L42_TS_DBNCE_750 | CS42L42_TS_DBNCE_1000 | CS42L42_TS_DBNCE_1250 | CS42L42_TS_DBNCE_1500 => (*cs42l42).ts_dbnc_fall = val,
            _ => { dev_err(dev, CSTR!("Wrong cirrus,ts-dbnc-fall DT value %d\n"), val); (*cs42l42).ts_dbnc_fall = CS42L42_TS_DBNCE_0; }
        }
    } else { (*cs42l42).ts_dbnc_fall = CS42L42_TS_DBNCE_0; }
    regmap_update_bits((*cs42l42).regmap, CS42L42_TSENSE_CTL, CS42L42_TS_FALL_DBNCE_TIME_MASK, (*cs42l42).ts_dbnc_fall << CS42L42_TS_FALL_DBNCE_TIME_SHIFT);
    ret = device_property_read_u32(dev, CSTR!("cirrus,btn-det-init-dbnce"), &mut val);
    if ret == 0 && val <= CS42L42_BTN_DET_INIT_DBNCE_MAX { (*cs42l42).btn_det_init_dbnce = val; }
    else { if ret == 0 { dev_err(dev, CSTR!("Wrong cirrus,btn-det-init-dbnce DT value %d\n"), val); } (*cs42l42).btn_det_init_dbnce = CS42L42_BTN_DET_INIT_DBNCE_DEFAULT; }
    ret = device_property_read_u32(dev, CSTR!("cirrus,btn-det-event-dbnce"), &mut val);
    if ret == 0 && val <= CS42L42_BTN_DET_EVENT_DBNCE_MAX { (*cs42l42).btn_det_event_dbnce = val; }
    else { if ret == 0 { dev_err(dev, CSTR!("Wrong cirrus,btn-det-event-dbnce DT value %d\n"), val); } (*cs42l42).btn_det_event_dbnce = CS42L42_BTN_DET_EVENT_DBNCE_DEFAULT; }
    ret = device_property_read_u32_array(dev, CSTR!("cirrus,bias-lvls"), thresholds.as_mut_ptr(), thresholds.len());
    for i in 0..4 {
        if ret == 0 && thresholds[i] <= CS42L42_HS_DET_LEVEL_MAX {
            (*cs42l42).bias_thresholds[i] = thresholds[i];
        } else {
            if ret == 0 { dev_err(dev, CSTR!("Wrong cirrus,bias-lvls[%d] DT value %d\n"), i as c_int, thresholds[i]); }
            (*cs42l42).bias_thresholds[i] = threshold_defaults[i];
        }
    }
    ret = device_property_read_u32(dev, CSTR!("cirrus,hs-bias-ramp-rate"), &mut val);
    if ret == 0 {
        match val {
            CS42L42_HSBIAS_RAMP_FAST_RISE_SLOW_FALL => { (*cs42l42).hs_bias_ramp_rate = val; (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME0; }
            CS42L42_HSBIAS_RAMP_FAST => { (*cs42l42).hs_bias_ramp_rate = val; (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME1; }
            CS42L42_HSBIAS_RAMP_SLOW => { (*cs42l42).hs_bias_ramp_rate = val; (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME2; }
            CS42L42_HSBIAS_RAMP_SLOWEST => { (*cs42l42).hs_bias_ramp_rate = val; (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME3; }
            _ => { dev_err(dev, CSTR!("Wrong cirrus,hs-bias-ramp-rate DT value %d\n"), val); (*cs42l42).hs_bias_ramp_rate = CS42L42_HSBIAS_RAMP_SLOW; (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME2; }
        }
    } else {
        (*cs42l42).hs_bias_ramp_rate = CS42L42_HSBIAS_RAMP_SLOW;
        (*cs42l42).hs_bias_ramp_time = CS42L42_HSBIAS_RAMP_TIME2;
    }
    regmap_update_bits((*cs42l42).regmap, CS42L42_HS_BIAS_CTL, CS42L42_HSBIAS_RAMP_MASK, (*cs42l42).hs_bias_ramp_rate << CS42L42_HSBIAS_RAMP_SHIFT);
    (*cs42l42).hs_bias_sense_en = if device_property_read_bool(dev, CSTR!("cirrus,hs-bias-sense-disable")) { 0 } else { 1 };
    0
}

/* Datasheet suspend sequence from cs42l42_shutdown_seq. */
static cs42l42_shutdown_seq: [reg_sequence; 29] = [
    reg_sequence { reg: CS42L42_MIC_DET_CTL1, def: 0x9F, delay_us: 0 },
    reg_sequence { reg: CS42L42_ADC_OVFL_INT_MASK, def: 0x01, delay_us: 0 },
    reg_sequence { reg: CS42L42_MIXER_INT_MASK, def: 0x0F, delay_us: 0 },
    reg_sequence { reg: CS42L42_SRC_INT_MASK, def: 0x0F, delay_us: 0 },
    reg_sequence { reg: CS42L42_ASP_RX_INT_MASK, def: 0x1F, delay_us: 0 },
    reg_sequence { reg: CS42L42_ASP_TX_INT_MASK, def: 0x0F, delay_us: 0 },
    reg_sequence { reg: CS42L42_CODEC_INT_MASK, def: 0x03, delay_us: 0 },
    reg_sequence { reg: CS42L42_SRCPL_INT_MASK, def: 0x7F, delay_us: 0 },
    reg_sequence { reg: CS42L42_VPMON_INT_MASK, def: 0x01, delay_us: 0 },
    reg_sequence { reg: CS42L42_PLL_LOCK_INT_MASK, def: 0x01, delay_us: 0 },
    reg_sequence { reg: CS42L42_TSRS_PLUG_INT_MASK, def: 0x0F, delay_us: 0 },
    reg_sequence { reg: CS42L42_WAKE_CTL, def: 0xE1, delay_us: 0 },
    reg_sequence { reg: CS42L42_DET_INT1_MASK, def: 0xE0, delay_us: 0 },
    reg_sequence { reg: CS42L42_DET_INT2_MASK, def: 0xFF, delay_us: 0 },
    reg_sequence { reg: CS42L42_MIXER_CHA_VOL, def: 0x3F, delay_us: 0 },
    reg_sequence { reg: CS42L42_MIXER_ADC_VOL, def: 0x3F, delay_us: 0 },
    reg_sequence { reg: CS42L42_MIXER_CHB_VOL, def: 0x3F, delay_us: 0 },
    reg_sequence { reg: CS42L42_HP_CTL, def: 0x0F, delay_us: 0 },
    reg_sequence { reg: CS42L42_ASP_RX_DAI0_EN, def: 0x00, delay_us: 0 },
    reg_sequence { reg: CS42L42_ASP_CLK_CFG, def: 0x00, delay_us: 0 },
    reg_sequence { reg: CS42L42_HSDET_CTL2, def: 0x00, delay_us: 0 },
    reg_sequence { reg: CS42L42_PWR_CTL1, def: 0xFE, delay_us: 0 },
    reg_sequence { reg: CS42L42_PWR_CTL2, def: 0x8C, delay_us: 0 },
    reg_sequence { reg: CS42L42_DAC_CTL2, def: 0x02, delay_us: 0 },
    reg_sequence { reg: CS42L42_HS_CLAMP_DISABLE, def: 0x00, delay_us: 0 },
    reg_sequence { reg: CS42L42_MISC_DET_CTL, def: 0x03, delay_us: 0 },
    reg_sequence { reg: CS42L42_TIPSENSE_CTL, def: 0x02, delay_us: 0 },
    reg_sequence { reg: CS42L42_HSBIAS_SC_AUTOCTL, def: 0x03, delay_us: 0 },
    reg_sequence { reg: CS42L42_PWR_CTL1, def: 0xFF, delay_us: 0 },
];

pub unsafe extern "C" fn cs42l42_suspend(dev: *mut device) -> c_int {
    let cs42l42 = dev_get_drvdata(dev);
    let mut reg = 0;
    let mut save_regs = [0u8; 29];
    if !(*cs42l42).init_done { return 0; }
    (*cs42l42).suspended = true;
    for i in 0..cs42l42_shutdown_seq.len() {
        regmap_read((*cs42l42).regmap, cs42l42_shutdown_seq[i].reg, &mut reg);
        save_regs[i] = reg as u8;
    }
    regmap_multi_reg_write((*cs42l42).regmap, cs42l42_shutdown_seq.as_ptr(), ARRAY_SIZE!(cs42l42_shutdown_seq));
    msleep(CS42L42_PDN_DONE_TIME_MS);
    let ret = regmap_read_poll_timeout((*cs42l42).regmap, CS42L42_CODEC_STATUS, &mut reg,
        (reg & CS42L42_PDN_DONE_MASK) != 0, CS42L42_PDN_DONE_POLL_US, CS42L42_PDN_DONE_TIMEOUT_US);
    if ret != 0 { dev_warn(dev, CSTR!("Failed to get PDN_DONE: %d\n"), ret); }
    regmap_update_bits((*cs42l42).regmap, CS42L42_PWR_CTL2, CS42L42_DISCHARGE_FILT_MASK, CS42L42_DISCHARGE_FILT_MASK);
    regcache_cache_only((*cs42l42).regmap, true);
    gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
    regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
    for i in 0..cs42l42_shutdown_seq.len() {
        regmap_write((*cs42l42).regmap, cs42l42_shutdown_seq[i].reg, save_regs[i] as c_uint);
    }
    regcache_drop_region((*cs42l42).regmap, CS42L42_PAGE_REGISTER, CS42L42_PAGE_REGISTER);
    dev_dbg(dev, CSTR!("System suspended\n"));
    0
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_suspend, "SND_SOC_CS42L42_CORE");

pub unsafe extern "C" fn cs42l42_resume(dev: *mut device) -> c_int {
    let cs42l42 = dev_get_drvdata(dev);
    if !(*cs42l42).init_done { return 0; }
    if (*cs42l42).plug_state != CS42L42_TS_UNPLUG {
        (*cs42l42).plug_state = CS42L42_TS_TRANS;
    }
    let ret = regulator_bulk_enable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, CSTR!("Failed to enable supplies: %d\n"), ret);
        return ret;
    }
    gpiod_set_value_cansleep((*cs42l42).reset_gpio, 1);
    usleep_range(CS42L42_BOOT_TIME_US, CS42L42_BOOT_TIME_US * 2);
    dev_dbg(dev, CSTR!("System resume powered up\n"));
    0
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_resume, "SND_SOC_CS42L42_CORE");

pub unsafe extern "C" fn cs42l42_resume_restore(dev: *mut device) {
    let cs42l42 = dev_get_drvdata(dev);
    regcache_cache_only((*cs42l42).regmap, false);
    regcache_mark_dirty((*cs42l42).regmap);
    regcache_sync_region((*cs42l42).regmap, CS42L42_MIC_DET_CTL1, CS42L42_MIC_DET_CTL1);
    regcache_sync((*cs42l42).regmap);
    (*cs42l42).suspended = false;
    dev_dbg(dev, CSTR!("System resumed\n"));
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_resume_restore, "SND_SOC_CS42L42_CORE");

unsafe extern "C" fn cs42l42_i2c_resume(dev: *mut device) -> c_int {
    let ret = cs42l42_resume(dev);
    if ret != 0 { return ret; }
    cs42l42_resume_restore(dev);
    0
}

pub unsafe extern "C" fn cs42l42_common_probe(cs42l42: *mut cs42l42_private, component_drv: *const snd_soc_component_driver, dai: *mut snd_soc_dai_driver) -> c_int {
    dev_set_drvdata((*cs42l42).dev, cs42l42);
    mutex_init(&mut (*cs42l42).irq_lock);
    for i in 0..(*cs42l42).supplies.len() {
        (*cs42l42).supplies[i].supply = cs42l42_supply_names[i];
    }
    let mut ret = devm_regulator_bulk_get((*cs42l42).dev, (*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*cs42l42).dev, CSTR!("Failed to request supplies: %d\n"), ret);
        return ret;
    }
    ret = regulator_bulk_enable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*cs42l42).dev, CSTR!("Failed to enable supplies: %d\n"), ret);
        return ret;
    }
    (*cs42l42).reset_gpio = devm_gpiod_get_optional((*cs42l42).dev, CSTR!("reset"), GPIOD_OUT_LOW);
    if IS_ERR((*cs42l42).reset_gpio as *const c_void) {
        ret = PTR_ERR((*cs42l42).reset_gpio as *const c_void);
        regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
        return ret;
    }
    if !(*cs42l42).reset_gpio.is_null() {
        dev_dbg((*cs42l42).dev, CSTR!("Found reset GPIO\n"));
        gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
        usleep_range(10, 500);
        if !(*cs42l42).sdw_peripheral.is_null() {
            (*cs42l42).sdw_waiting_first_unattach = true;
        } else {
            gpiod_set_value_cansleep((*cs42l42).reset_gpio, 1);
        }
    }
    usleep_range(CS42L42_BOOT_TIME_US, CS42L42_BOOT_TIME_US * 2);
    if (*cs42l42).irq != 0 {
        ret = request_threaded_irq((*cs42l42).irq as c_uint, core::ptr::null_mut(), cs42l42_irq_thread,
            IRQF_ONESHOT | IRQF_TRIGGER_LOW, CSTR!("cs42l42"), cs42l42 as *mut c_void);
        if ret != 0 {
            dev_err_probe((*cs42l42).dev, ret, CSTR!("Failed to request IRQ\n"));
            gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
            regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
            return ret;
        }
    }
    ret = devm_snd_soc_register_component((*cs42l42).dev, component_drv, dai, 1);
    if ret < 0 {
        if (*cs42l42).irq != 0 { free_irq((*cs42l42).irq as c_uint, cs42l42 as *mut c_void); }
        gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
        regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
        return ret;
    }
    0
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_common_probe, "SND_SOC_CS42L42_CORE");

pub unsafe extern "C" fn cs42l42_init(cs42l42: *mut cs42l42_private) -> c_int {
    let mut reg = 0;
    let devid = cirrus_read_device_id((*cs42l42).regmap, CS42L42_DEVID_AB);
    let mut ret: c_int;
    if devid < 0 {
        ret = devid;
        dev_err((*cs42l42).dev, CSTR!("Failed to read device ID: %d\n"), ret);
        goto_err_disable(cs42l42);
        return ret;
    }
    if devid != (*cs42l42).devid {
        ret = -ENODEV;
        dev_err((*cs42l42).dev, CSTR!("CS42L%x Device ID (%X). Expected %X\n"), (*cs42l42).devid & 0xff, devid, (*cs42l42).devid);
        goto_err_disable(cs42l42);
        return ret;
    }
    ret = regmap_read((*cs42l42).regmap, CS42L42_REVID, &mut reg);
    if ret < 0 {
        dev_err((*cs42l42).dev, CSTR!("Get Revision ID failed\n"));
        goto_err_shutdown(cs42l42);
        return ret;
    }
    dev_info((*cs42l42).dev, CSTR!("Cirrus Logic CS42L%x, Revision: %02X\n"), (*cs42l42).devid & 0xff, reg & 0xFF);
    regmap_update_bits((*cs42l42).regmap, CS42L42_PWR_CTL1,
        CS42L42_ASP_DAO_PDN_MASK | CS42L42_ASP_DAI_PDN_MASK | CS42L42_MIXER_PDN_MASK | CS42L42_EQ_PDN_MASK | CS42L42_HP_PDN_MASK | CS42L42_ADC_PDN_MASK | CS42L42_PDN_ALL_MASK,
        (1 << CS42L42_ASP_DAO_PDN_SHIFT) | (1 << CS42L42_ASP_DAI_PDN_SHIFT) | (1 << CS42L42_MIXER_PDN_SHIFT) | (1 << CS42L42_EQ_PDN_SHIFT) | (1 << CS42L42_HP_PDN_SHIFT) | (1 << CS42L42_ADC_PDN_SHIFT) | (0 << CS42L42_PDN_ALL_SHIFT));
    ret = cs42l42_handle_device_data((*cs42l42).dev, cs42l42);
    if ret != 0 {
        goto_err_shutdown(cs42l42);
        return ret;
    }
    if !(*cs42l42).sdw_peripheral.is_null() {
        regmap_update_bits((*cs42l42).regmap, CS42L42_PWR_CTL2,
            CS42L42_SRC_PDN_OVERRIDE_MASK | CS42L42_DAC_SRC_PDNB_MASK | CS42L42_ADC_SRC_PDNB_MASK,
            CS42L42_SRC_PDN_OVERRIDE_MASK);
    }
    cs42l42_setup_hs_type_detect(cs42l42);
    (*cs42l42).init_done = true;
    cs42l42_set_interrupt_masks(cs42l42);
    0
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_init, "SND_SOC_CS42L42_CORE");

unsafe fn goto_err_shutdown(cs42l42: *mut cs42l42_private) {
    regmap_write((*cs42l42).regmap, CS42L42_CODEC_INT_MASK, 0xff);
    regmap_write((*cs42l42).regmap, CS42L42_TSRS_PLUG_INT_MASK, 0xff);
    regmap_write((*cs42l42).regmap, CS42L42_PWR_CTL1, 0xff);
    goto_err_disable(cs42l42);
}

unsafe fn goto_err_disable(cs42l42: *mut cs42l42_private) {
    if (*cs42l42).irq != 0 {
        free_irq((*cs42l42).irq as c_uint, cs42l42 as *mut c_void);
    }
    gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
    regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
}

pub unsafe extern "C" fn cs42l42_common_remove(cs42l42: *mut cs42l42_private) {
    if (*cs42l42).irq != 0 {
        free_irq((*cs42l42).irq as c_uint, cs42l42 as *mut c_void);
    }
    if (*cs42l42).init_done {
        regmap_write((*cs42l42).regmap, CS42L42_CODEC_INT_MASK, 0xff);
        regmap_write((*cs42l42).regmap, CS42L42_TSRS_PLUG_INT_MASK, 0xff);
        regmap_write((*cs42l42).regmap, CS42L42_PWR_CTL1, 0xff);
    }
    gpiod_set_value_cansleep((*cs42l42).reset_gpio, 0);
    regulator_bulk_disable((*cs42l42).supplies.len() as c_int, (*cs42l42).supplies.as_mut_ptr());
}
EXPORT_SYMBOL_NS_GPL!(cs42l42_common_remove, "SND_SOC_CS42L42_CORE");

MODULE_DESCRIPTION!("ASoC CS42L42 driver");
MODULE_AUTHOR!("James Schulman, Cirrus Logic Inc, <james.schulman@cirrus.com>");
MODULE_AUTHOR!("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
MODULE_AUTHOR!("Michael White, Cirrus Logic Inc, <michael.white@cirrus.com>");
MODULE_AUTHOR!("Lucas Tanure <tanureal@opensource.cirrus.com>");
MODULE_AUTHOR!("Richard Fitzgerald <rf@opensource.cirrus.com>");
MODULE_AUTHOR!("Vitaly Rodionov <vitalyr@opensource.cirrus.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
