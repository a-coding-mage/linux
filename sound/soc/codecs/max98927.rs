// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * max98927.rs  --  MAX98927 ALSA Soc Audio driver
 *
 * Copyright (C) 2016-2017 Maxim Integrated Products
 * Author: Ryan Lee <ryans.lee@maximintegrated.com>
 *
 * Source-level Rust translation of soc/codecs/max98927.c.
 * C include dependencies are expected to be supplied by the surrounding kernel
 * binding layer.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type bool_ = bool;
type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_NOPM: c_uint = 0;

#[repr(C)]
pub struct device {
    pub of_node: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct gpio_desc;
#[repr(C)]
pub struct snd_pcm_substream;
#[repr(C)]
pub struct snd_pcm_hw_params;
#[repr(C)]
pub struct snd_kcontrol;
#[repr(C)]
pub struct snd_soc_dapm_context;

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct max98927_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub reset_gpio: *mut gpio_desc,
    pub provider: bool_,
    pub tdm_mode: bool_,
    pub interleave_mode: bool_,
    pub iface: c_uint,
    pub sysclk: c_uint,
    pub ch_size: c_int,
    pub v_l_slot: c_int,
    pub i_l_slot: c_int,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
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
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: *const c_char,
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut max98927_priv;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut max98927_priv;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int) -> c_int;
    fn of_property_read_bool(node: *mut core::ffi::c_void, propname: *const c_char) -> bool_;
    fn of_property_read_u32(node: *mut core::ffi::c_void, propname: *const c_char, val: *mut c_int) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut max98927_priv);
    fn i2c_get_clientdata(i2c: *mut i2c_client) -> *mut max98927_priv;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool_;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn usleep_range(min: c_uint, max: c_uint);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);

    static MAX98927_R0001_INT_RAW1: c_uint;
    static MAX98927_R0002_INT_RAW2: c_uint;
    static MAX98927_R0003_INT_RAW3: c_uint;
    static MAX98927_R0004_INT_STATE1: c_uint;
    static MAX98927_R0005_INT_STATE2: c_uint;
    static MAX98927_R0006_INT_STATE3: c_uint;
    static MAX98927_R0007_INT_FLAG1: c_uint;
    static MAX98927_R0008_INT_FLAG2: c_uint;
    static MAX98927_R0009_INT_FLAG3: c_uint;
    static MAX98927_R000A_INT_EN1: c_uint;
    static MAX98927_R000B_INT_EN2: c_uint;
    static MAX98927_R000C_INT_EN3: c_uint;
    static MAX98927_R000D_INT_FLAG_CLR1: c_uint;
    static MAX98927_R000E_INT_FLAG_CLR2: c_uint;
    static MAX98927_R000F_INT_FLAG_CLR3: c_uint;
    static MAX98927_R0010_IRQ_CTRL: c_uint;
    static MAX98927_R0011_CLK_MON: c_uint;
    static MAX98927_R0012_WDOG_CTRL: c_uint;
    static MAX98927_R0013_WDOG_RST: c_uint;
    static MAX98927_R0014_MEAS_ADC_THERM_WARN_THRESH: c_uint;
    static MAX98927_R0015_MEAS_ADC_THERM_SHDN_THRESH: c_uint;
    static MAX98927_R0016_MEAS_ADC_THERM_HYSTERESIS: c_uint;
    static MAX98927_R0017_PIN_CFG: c_uint;
    static MAX98927_R0018_PCM_RX_EN_A: c_uint;
    static MAX98927_R0019_PCM_RX_EN_B: c_uint;
    static MAX98927_R001A_PCM_TX_EN_A: c_uint;
    static MAX98927_R001B_PCM_TX_EN_B: c_uint;
    static MAX98927_R001C_PCM_TX_HIZ_CTRL_A: c_uint;
    static MAX98927_R001D_PCM_TX_HIZ_CTRL_B: c_uint;
    static MAX98927_R001E_PCM_TX_CH_SRC_A: c_uint;
    static MAX98927_R001F_PCM_TX_CH_SRC_B: c_uint;
    static MAX98927_R0020_PCM_MODE_CFG: c_uint;
    static MAX98927_R0021_PCM_MASTER_MODE: c_uint;
    static MAX98927_R0022_PCM_CLK_SETUP: c_uint;
    static MAX98927_R0023_PCM_SR_SETUP1: c_uint;
    static MAX98927_R0024_PCM_SR_SETUP2: c_uint;
    static MAX98927_R0025_PCM_TO_SPK_MONOMIX_A: c_uint;
    static MAX98927_R0026_PCM_TO_SPK_MONOMIX_B: c_uint;
    static MAX98927_R0027_ICC_RX_EN_A: c_uint;
    static MAX98927_R0028_ICC_RX_EN_B: c_uint;
    static MAX98927_R002B_ICC_TX_EN_A: c_uint;
    static MAX98927_R002C_ICC_TX_EN_B: c_uint;
    static MAX98927_R002E_ICC_HIZ_MANUAL_MODE: c_uint;
    static MAX98927_R002F_ICC_TX_HIZ_EN_A: c_uint;
    static MAX98927_R0030_ICC_TX_HIZ_EN_B: c_uint;
    static MAX98927_R0031_ICC_LNK_EN: c_uint;
    static MAX98927_R0032_PDM_TX_EN: c_uint;
    static MAX98927_R0033_PDM_TX_HIZ_CTRL: c_uint;
    static MAX98927_R0034_PDM_TX_CTRL: c_uint;
    static MAX98927_R0035_PDM_RX_CTRL: c_uint;
    static MAX98927_R0036_AMP_VOL_CTRL: c_uint;
    static MAX98927_R0037_AMP_DSP_CFG: c_uint;
    static MAX98927_R0038_TONE_GEN_DC_CFG: c_uint;
    static MAX98927_R0039_DRE_CTRL: c_uint;
    static MAX98927_R003A_AMP_EN: c_uint;
    static MAX98927_R003B_SPK_SRC_SEL: c_uint;
    static MAX98927_R003C_SPK_GAIN: c_uint;
    static MAX98927_R003D_SSM_CFG: c_uint;
    static MAX98927_R003E_MEAS_EN: c_uint;
    static MAX98927_R003F_MEAS_DSP_CFG: c_uint;
    static MAX98927_R0040_BOOST_CTRL0: c_uint;
    static MAX98927_R0041_BOOST_CTRL3: c_uint;
    static MAX98927_R0042_BOOST_CTRL1: c_uint;
    static MAX98927_R0043_MEAS_ADC_CFG: c_uint;
    static MAX98927_R0044_MEAS_ADC_BASE_MSB: c_uint;
    static MAX98927_R0045_MEAS_ADC_BASE_LSB: c_uint;
    static MAX98927_R0046_ADC_CH0_DIVIDE: c_uint;
    static MAX98927_R0047_ADC_CH1_DIVIDE: c_uint;
    static MAX98927_R0048_ADC_CH2_DIVIDE: c_uint;
    static MAX98927_R0049_ADC_CH0_FILT_CFG: c_uint;
    static MAX98927_R004A_ADC_CH1_FILT_CFG: c_uint;
    static MAX98927_R004B_ADC_CH2_FILT_CFG: c_uint;
    static MAX98927_R004C_MEAS_ADC_CH0_READ: c_uint;
    static MAX98927_R004D_MEAS_ADC_CH1_READ: c_uint;
    static MAX98927_R004E_MEAS_ADC_CH2_READ: c_uint;
    static MAX98927_R0051_BROWNOUT_STATUS: c_uint;
    static MAX98927_R0052_BROWNOUT_EN: c_uint;
    static MAX98927_R0053_BROWNOUT_INFINITE_HOLD: c_uint;
    static MAX98927_R0054_BROWNOUT_INFINITE_HOLD_CLR: c_uint;
    static MAX98927_R0055_BROWNOUT_LVL_HOLD: c_uint;
    static MAX98927_R005A_BROWNOUT_LVL1_THRESH: c_uint;
    static MAX98927_R005B_BROWNOUT_LVL2_THRESH: c_uint;
    static MAX98927_R005C_BROWNOUT_LVL3_THRESH: c_uint;
    static MAX98927_R005D_BROWNOUT_LVL4_THRESH: c_uint;
    static MAX98927_R005E_BROWNOUT_THRESH_HYSTERYSIS: c_uint;
    static MAX98927_R005F_BROWNOUT_AMP_LIMITER_ATK_REL: c_uint;
    static MAX98927_R0060_BROWNOUT_AMP_GAIN_ATK_REL: c_uint;
    static MAX98927_R0061_BROWNOUT_AMP1_CLIP_MODE: c_uint;
    static MAX98927_R0072_BROWNOUT_LVL1_CUR_LIMIT: c_uint;
    static MAX98927_R0073_BROWNOUT_LVL1_AMP1_CTRL1: c_uint;
    static MAX98927_R0074_BROWNOUT_LVL1_AMP1_CTRL2: c_uint;
    static MAX98927_R0075_BROWNOUT_LVL1_AMP1_CTRL3: c_uint;
    static MAX98927_R0076_BROWNOUT_LVL2_CUR_LIMIT: c_uint;
    static MAX98927_R0077_BROWNOUT_LVL2_AMP1_CTRL1: c_uint;
    static MAX98927_R0078_BROWNOUT_LVL2_AMP1_CTRL2: c_uint;
    static MAX98927_R0079_BROWNOUT_LVL2_AMP1_CTRL3: c_uint;
    static MAX98927_R007A_BROWNOUT_LVL3_CUR_LIMIT: c_uint;
    static MAX98927_R007B_BROWNOUT_LVL3_AMP1_CTRL1: c_uint;
    static MAX98927_R007C_BROWNOUT_LVL3_AMP1_CTRL2: c_uint;
    static MAX98927_R007D_BROWNOUT_LVL3_AMP1_CTRL3: c_uint;
    static MAX98927_R007E_BROWNOUT_LVL4_CUR_LIMIT: c_uint;
    static MAX98927_R007F_BROWNOUT_LVL4_AMP1_CTRL1: c_uint;
    static MAX98927_R0080_BROWNOUT_LVL4_AMP1_CTRL2: c_uint;
    static MAX98927_R0081_BROWNOUT_LVL4_AMP1_CTRL3: c_uint;
    static MAX98927_R0082_ENV_TRACK_VOUT_HEADROOM: c_uint;
    static MAX98927_R0083_ENV_TRACK_BOOST_VOUT_DELAY: c_uint;
    static MAX98927_R0084_ENV_TRACK_REL_RATE: c_uint;
    static MAX98927_R0085_ENV_TRACK_HOLD_RATE: c_uint;
    static MAX98927_R0086_ENV_TRACK_CTRL: c_uint;
    static MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ: c_uint;
    static MAX98927_R00FF_GLOBAL_SHDN: c_uint;
    static MAX98927_R0100_SOFT_RESET: c_uint;
    static MAX98927_R01FF_REV_ID: c_uint;

    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_PDM: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_uint;
    static SND_SOC_DAPM_POST_PMD: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    static MAX98927_PCM_MASTER_MODE_SLAVE: c_uint;
    static MAX98927_PCM_MASTER_MODE_MASTER: c_uint;
    static MAX98927_PCM_MASTER_MODE_MASK: c_uint;
    static MAX98927_PCM_MASTER_MODE_MCLK_MASK: c_uint;
    static MAX98927_PCM_MASTER_MODE_MCLK_RATE_SHIFT: c_uint;
    static MAX98927_PCM_MODE_CFG_PCM_BCLKEDGE: c_uint;
    static MAX98927_PCM_FORMAT_I2S: c_uint;
    static MAX98927_PCM_FORMAT_LJ: c_uint;
    static MAX98927_PCM_FORMAT_TDM_MODE1: c_uint;
    static MAX98927_PCM_FORMAT_TDM_MODE0: c_uint;
    static MAX98927_PCM_RX_CH0_EN: c_uint;
    static MAX98927_PCM_RX_CH1_EN: c_uint;
    static MAX98927_PCM_MODE_CFG_FORMAT_MASK: c_uint;
    static MAX98927_PCM_MODE_CFG_FORMAT_SHIFT: c_uint;
    static MAX98927_SPK_SRC_MASK: c_uint;
    static MAX98927_PDM_RX_EN_MASK: c_uint;
    static MAX98927_PCM_CLK_SETUP_BSEL_MASK: c_uint;
    static MAX98927_PCM_MODE_CFG_CHANSZ_16: c_uint;
    static MAX98927_PCM_MODE_CFG_CHANSZ_24: c_uint;
    static MAX98927_PCM_MODE_CFG_CHANSZ_32: c_uint;
    static MAX98927_PCM_MODE_CFG_CHANSZ_MASK: c_uint;
    static MAX98927_PCM_SR_SET1_SR_8000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_11025: c_uint;
    static MAX98927_PCM_SR_SET1_SR_12000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_16000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_22050: c_uint;
    static MAX98927_PCM_SR_SET1_SR_24000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_32000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_44100: c_uint;
    static MAX98927_PCM_SR_SET1_SR_48000: c_uint;
    static MAX98927_PCM_SR_SET1_SR_MASK: c_uint;
    static MAX98927_PCM_SR_SET2_SR_MASK: c_uint;
    static MAX98927_PCM_SR_SET2_SR_SHIFT: c_uint;
    static MAX98927_PCM_SR_SET2_IVADC_SR_MASK: c_uint;
    static MAX98927_AMP_EN_MASK: c_uint;
    static MAX98927_GLOBAL_EN_MASK: c_uint;
    static MAX98927_AMP_VOL_WIDTH: c_uint;
    static MAX98927_BROWNOUT_DSP_SHIFT: c_uint;
    static MAX98927_AMP_DSP_CFG_RMP_SHIFT: c_uint;
    static MAX98927_DRE_EN_SHIFT: c_uint;
    static MAX98927_AMP_VOL_SEL_SHIFT: c_uint;
    static MAX98927_SOFT_RESET: c_uint;
    static MAX98927_PCM_TX_CH_SRC_A_I_SHIFT: c_int;
    static MAX98927_PCM_TX_CH_INTERLEAVE_MASK: c_uint;
    static MAX98927_PCM_TO_SPK_MONOMIX_CFG_SHIFT: c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static max98927_reg: [reg_default; 117] = unsafe {
    [
        reg_default { reg: MAX98927_R0001_INT_RAW1, def: 0x00 },
        reg_default { reg: MAX98927_R0002_INT_RAW2, def: 0x00 },
        reg_default { reg: MAX98927_R0003_INT_RAW3, def: 0x00 },
        reg_default { reg: MAX98927_R0004_INT_STATE1, def: 0x00 },
        reg_default { reg: MAX98927_R0005_INT_STATE2, def: 0x00 },
        reg_default { reg: MAX98927_R0006_INT_STATE3, def: 0x00 },
        reg_default { reg: MAX98927_R0007_INT_FLAG1, def: 0x00 },
        reg_default { reg: MAX98927_R0008_INT_FLAG2, def: 0x00 },
        reg_default { reg: MAX98927_R0009_INT_FLAG3, def: 0x00 },
        reg_default { reg: MAX98927_R000A_INT_EN1, def: 0x00 },
        reg_default { reg: MAX98927_R000B_INT_EN2, def: 0x00 },
        reg_default { reg: MAX98927_R000C_INT_EN3, def: 0x00 },
        reg_default { reg: MAX98927_R000D_INT_FLAG_CLR1, def: 0x00 },
        reg_default { reg: MAX98927_R000E_INT_FLAG_CLR2, def: 0x00 },
        reg_default { reg: MAX98927_R000F_INT_FLAG_CLR3, def: 0x00 },
        reg_default { reg: MAX98927_R0010_IRQ_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0011_CLK_MON, def: 0x00 },
        reg_default { reg: MAX98927_R0012_WDOG_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0013_WDOG_RST, def: 0x00 },
        reg_default { reg: MAX98927_R0014_MEAS_ADC_THERM_WARN_THRESH, def: 0x75 },
        reg_default { reg: MAX98927_R0015_MEAS_ADC_THERM_SHDN_THRESH, def: 0x8c },
        reg_default { reg: MAX98927_R0016_MEAS_ADC_THERM_HYSTERESIS, def: 0x08 },
        reg_default { reg: MAX98927_R0017_PIN_CFG, def: 0x55 },
        reg_default { reg: MAX98927_R0018_PCM_RX_EN_A, def: 0x00 },
        reg_default { reg: MAX98927_R0019_PCM_RX_EN_B, def: 0x00 },
        reg_default { reg: MAX98927_R001A_PCM_TX_EN_A, def: 0x00 },
        reg_default { reg: MAX98927_R001B_PCM_TX_EN_B, def: 0x00 },
        reg_default { reg: MAX98927_R001C_PCM_TX_HIZ_CTRL_A, def: 0x00 },
        reg_default { reg: MAX98927_R001D_PCM_TX_HIZ_CTRL_B, def: 0x00 },
        reg_default { reg: MAX98927_R001E_PCM_TX_CH_SRC_A, def: 0x00 },
        reg_default { reg: MAX98927_R001F_PCM_TX_CH_SRC_B, def: 0x00 },
        reg_default { reg: MAX98927_R0020_PCM_MODE_CFG, def: 0x40 },
        reg_default { reg: MAX98927_R0021_PCM_MASTER_MODE, def: 0x00 },
        reg_default { reg: MAX98927_R0022_PCM_CLK_SETUP, def: 0x22 },
        reg_default { reg: MAX98927_R0023_PCM_SR_SETUP1, def: 0x00 },
        reg_default { reg: MAX98927_R0024_PCM_SR_SETUP2, def: 0x00 },
        reg_default { reg: MAX98927_R0025_PCM_TO_SPK_MONOMIX_A, def: 0x00 },
        reg_default { reg: MAX98927_R0026_PCM_TO_SPK_MONOMIX_B, def: 0x00 },
        reg_default { reg: MAX98927_R0027_ICC_RX_EN_A, def: 0x00 },
        reg_default { reg: MAX98927_R0028_ICC_RX_EN_B, def: 0x00 },
        reg_default { reg: MAX98927_R002B_ICC_TX_EN_A, def: 0x00 },
        reg_default { reg: MAX98927_R002C_ICC_TX_EN_B, def: 0x00 },
        reg_default { reg: MAX98927_R002E_ICC_HIZ_MANUAL_MODE, def: 0x00 },
        reg_default { reg: MAX98927_R002F_ICC_TX_HIZ_EN_A, def: 0x00 },
        reg_default { reg: MAX98927_R0030_ICC_TX_HIZ_EN_B, def: 0x00 },
        reg_default { reg: MAX98927_R0031_ICC_LNK_EN, def: 0x00 },
        reg_default { reg: MAX98927_R0032_PDM_TX_EN, def: 0x00 },
        reg_default { reg: MAX98927_R0033_PDM_TX_HIZ_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0034_PDM_TX_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0035_PDM_RX_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0036_AMP_VOL_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0037_AMP_DSP_CFG, def: 0x02 },
        reg_default { reg: MAX98927_R0038_TONE_GEN_DC_CFG, def: 0x00 },
        reg_default { reg: MAX98927_R0039_DRE_CTRL, def: 0x01 },
        reg_default { reg: MAX98927_R003A_AMP_EN, def: 0x00 },
        reg_default { reg: MAX98927_R003B_SPK_SRC_SEL, def: 0x00 },
        reg_default { reg: MAX98927_R003C_SPK_GAIN, def: 0x00 },
        reg_default { reg: MAX98927_R003D_SSM_CFG, def: 0x04 },
        reg_default { reg: MAX98927_R003E_MEAS_EN, def: 0x00 },
        reg_default { reg: MAX98927_R003F_MEAS_DSP_CFG, def: 0x04 },
        reg_default { reg: MAX98927_R0040_BOOST_CTRL0, def: 0x00 },
        reg_default { reg: MAX98927_R0041_BOOST_CTRL3, def: 0x00 },
        reg_default { reg: MAX98927_R0042_BOOST_CTRL1, def: 0x00 },
        reg_default { reg: MAX98927_R0043_MEAS_ADC_CFG, def: 0x00 },
        reg_default { reg: MAX98927_R0044_MEAS_ADC_BASE_MSB, def: 0x01 },
        reg_default { reg: MAX98927_R0045_MEAS_ADC_BASE_LSB, def: 0x00 },
        reg_default { reg: MAX98927_R0046_ADC_CH0_DIVIDE, def: 0x00 },
        reg_default { reg: MAX98927_R0047_ADC_CH1_DIVIDE, def: 0x00 },
        reg_default { reg: MAX98927_R0048_ADC_CH2_DIVIDE, def: 0x00 },
        reg_default { reg: MAX98927_R0049_ADC_CH0_FILT_CFG, def: 0x00 },
        reg_default { reg: MAX98927_R004A_ADC_CH1_FILT_CFG, def: 0x00 },
        reg_default { reg: MAX98927_R004B_ADC_CH2_FILT_CFG, def: 0x00 },
        reg_default { reg: MAX98927_R004C_MEAS_ADC_CH0_READ, def: 0x00 },
        reg_default { reg: MAX98927_R004D_MEAS_ADC_CH1_READ, def: 0x00 },
        reg_default { reg: MAX98927_R004E_MEAS_ADC_CH2_READ, def: 0x00 },
        reg_default { reg: MAX98927_R0051_BROWNOUT_STATUS, def: 0x00 },
        reg_default { reg: MAX98927_R0052_BROWNOUT_EN, def: 0x00 },
        reg_default { reg: MAX98927_R0053_BROWNOUT_INFINITE_HOLD, def: 0x00 },
        reg_default { reg: MAX98927_R0054_BROWNOUT_INFINITE_HOLD_CLR, def: 0x00 },
        reg_default { reg: MAX98927_R0055_BROWNOUT_LVL_HOLD, def: 0x00 },
        reg_default { reg: MAX98927_R005A_BROWNOUT_LVL1_THRESH, def: 0x00 },
        reg_default { reg: MAX98927_R005B_BROWNOUT_LVL2_THRESH, def: 0x00 },
        reg_default { reg: MAX98927_R005C_BROWNOUT_LVL3_THRESH, def: 0x00 },
        reg_default { reg: MAX98927_R005D_BROWNOUT_LVL4_THRESH, def: 0x00 },
        reg_default { reg: MAX98927_R005E_BROWNOUT_THRESH_HYSTERYSIS, def: 0x00 },
        reg_default { reg: MAX98927_R005F_BROWNOUT_AMP_LIMITER_ATK_REL, def: 0x00 },
        reg_default { reg: MAX98927_R0060_BROWNOUT_AMP_GAIN_ATK_REL, def: 0x00 },
        reg_default { reg: MAX98927_R0061_BROWNOUT_AMP1_CLIP_MODE, def: 0x00 },
        reg_default { reg: MAX98927_R0072_BROWNOUT_LVL1_CUR_LIMIT, def: 0x00 },
        reg_default { reg: MAX98927_R0073_BROWNOUT_LVL1_AMP1_CTRL1, def: 0x00 },
        reg_default { reg: MAX98927_R0074_BROWNOUT_LVL1_AMP1_CTRL2, def: 0x00 },
        reg_default { reg: MAX98927_R0075_BROWNOUT_LVL1_AMP1_CTRL3, def: 0x00 },
        reg_default { reg: MAX98927_R0076_BROWNOUT_LVL2_CUR_LIMIT, def: 0x00 },
        reg_default { reg: MAX98927_R0077_BROWNOUT_LVL2_AMP1_CTRL1, def: 0x00 },
        reg_default { reg: MAX98927_R0078_BROWNOUT_LVL2_AMP1_CTRL2, def: 0x00 },
        reg_default { reg: MAX98927_R0079_BROWNOUT_LVL2_AMP1_CTRL3, def: 0x00 },
        reg_default { reg: MAX98927_R007A_BROWNOUT_LVL3_CUR_LIMIT, def: 0x00 },
        reg_default { reg: MAX98927_R007B_BROWNOUT_LVL3_AMP1_CTRL1, def: 0x00 },
        reg_default { reg: MAX98927_R007C_BROWNOUT_LVL3_AMP1_CTRL2, def: 0x00 },
        reg_default { reg: MAX98927_R007D_BROWNOUT_LVL3_AMP1_CTRL3, def: 0x00 },
        reg_default { reg: MAX98927_R007E_BROWNOUT_LVL4_CUR_LIMIT, def: 0x00 },
        reg_default { reg: MAX98927_R007F_BROWNOUT_LVL4_AMP1_CTRL1, def: 0x00 },
        reg_default { reg: MAX98927_R0080_BROWNOUT_LVL4_AMP1_CTRL2, def: 0x00 },
        reg_default { reg: MAX98927_R0081_BROWNOUT_LVL4_AMP1_CTRL3, def: 0x00 },
        reg_default { reg: MAX98927_R0082_ENV_TRACK_VOUT_HEADROOM, def: 0x00 },
        reg_default { reg: MAX98927_R0083_ENV_TRACK_BOOST_VOUT_DELAY, def: 0x00 },
        reg_default { reg: MAX98927_R0084_ENV_TRACK_REL_RATE, def: 0x00 },
        reg_default { reg: MAX98927_R0085_ENV_TRACK_HOLD_RATE, def: 0x00 },
        reg_default { reg: MAX98927_R0086_ENV_TRACK_CTRL, def: 0x00 },
        reg_default { reg: MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ, def: 0x00 },
        reg_default { reg: MAX98927_R00FF_GLOBAL_SHDN, def: 0x00 },
        reg_default { reg: MAX98927_R0100_SOFT_RESET, def: 0x00 },
        reg_default { reg: MAX98927_R01FF_REV_ID, def: 0x40 },
    ]
};

unsafe extern "C" fn max98927_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let max98927 = snd_soc_component_get_drvdata(component);
    let mut mode: c_uint = 0;
    let mut format: c_uint = 0;
    let mut use_pdm = false;
    let mut invert: c_uint = 0;

    dev_dbg((*component).dev, cstr!("%s: fmt 0x%08X\n"), cstr!("max98927_dai_set_fmt"), fmt);

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            (*max98927).provider = false;
            mode = MAX98927_PCM_MASTER_MODE_SLAVE;
        }
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            (*max98927).provider = true;
            mode = MAX98927_PCM_MASTER_MODE_MASTER;
        }
        _ => {
            dev_err((*component).dev, cstr!("DAI clock mode unsupported\n"));
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98927).regmap, MAX98927_R0021_PCM_MASTER_MODE,
                       MAX98927_PCM_MASTER_MODE_MASK, mode);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        x if x == SND_SOC_DAIFMT_IB_NF => {
            invert = MAX98927_PCM_MODE_CFG_PCM_BCLKEDGE;
        }
        _ => {
            dev_err((*component).dev, cstr!("DAI invert mode unsupported\n"));
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98927).regmap, MAX98927_R0020_PCM_MODE_CFG,
                       MAX98927_PCM_MODE_CFG_PCM_BCLKEDGE, invert);

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => format = MAX98927_PCM_FORMAT_I2S,
        x if x == SND_SOC_DAIFMT_LEFT_J => format = MAX98927_PCM_FORMAT_LJ,
        x if x == SND_SOC_DAIFMT_DSP_A => format = MAX98927_PCM_FORMAT_TDM_MODE1,
        x if x == SND_SOC_DAIFMT_DSP_B => format = MAX98927_PCM_FORMAT_TDM_MODE0,
        x if x == SND_SOC_DAIFMT_PDM => use_pdm = true,
        _ => return -EINVAL,
    }
    (*max98927).iface = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    if !use_pdm {
        /* pcm channel configuration */
        regmap_update_bits((*max98927).regmap, MAX98927_R0018_PCM_RX_EN_A,
                           MAX98927_PCM_RX_CH0_EN | MAX98927_PCM_RX_CH1_EN,
                           MAX98927_PCM_RX_CH0_EN | MAX98927_PCM_RX_CH1_EN);
        regmap_update_bits((*max98927).regmap, MAX98927_R0020_PCM_MODE_CFG,
                           MAX98927_PCM_MODE_CFG_FORMAT_MASK,
                           format << MAX98927_PCM_MODE_CFG_FORMAT_SHIFT);
        regmap_update_bits((*max98927).regmap, MAX98927_R003B_SPK_SRC_SEL,
                           MAX98927_SPK_SRC_MASK, 0);
        regmap_update_bits((*max98927).regmap, MAX98927_R0035_PDM_RX_CTRL,
                           MAX98927_PDM_RX_EN_MASK, 0);
    } else {
        /* pdm channel configuration */
        regmap_update_bits((*max98927).regmap, MAX98927_R0035_PDM_RX_CTRL,
                           MAX98927_PDM_RX_EN_MASK, 1);
        regmap_update_bits((*max98927).regmap, MAX98927_R003B_SPK_SRC_SEL,
                           MAX98927_SPK_SRC_MASK, 3);
        regmap_update_bits((*max98927).regmap, MAX98927_R0018_PCM_RX_EN_A,
                           MAX98927_PCM_RX_CH0_EN | MAX98927_PCM_RX_CH1_EN, 0);
    }
    0
}

/* codec MCLK rate in master mode */
static rate_table: [c_int; 10] = [
    5644800, 6000000, 6144000, 6500000,
    9600000, 11289600, 12000000, 12288000,
    13000000, 19200000,
];

/* BCLKs per LRCLK */
static bclk_sel_table: [c_int; 9] = [
    32, 48, 64, 96, 128, 192, 256, 384, 512,
];

fn max98927_get_bclk_sel(bclk: c_int) -> c_int {
    let mut i = 0usize;
    /* match BCLKs per LRCLK */
    while i < bclk_sel_table.len() {
        if bclk_sel_table[i] == bclk {
            return i as c_int + 2;
        }
        i += 1;
    }
    0
}

unsafe fn max98927_set_clock(max98927: *mut max98927_priv, params: *mut snd_pcm_hw_params) -> c_int {
    let component = (*max98927).component;
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio = params_channels(params) * (*max98927).ch_size;
    let mut value: c_int;

    if (*max98927).provider {
        let mut i = 0usize;
        /* match rate to closest value */
        while i < rate_table.len() {
            if rate_table[i] as c_uint >= (*max98927).sysclk {
                break;
            }
            i += 1;
        }
        if i == rate_table.len() {
            dev_err((*component).dev, cstr!("failed to find proper clock rate.\n"));
            return -EINVAL;
        }
        regmap_update_bits((*max98927).regmap, MAX98927_R0021_PCM_MASTER_MODE,
                           MAX98927_PCM_MASTER_MODE_MCLK_MASK,
                           (i as c_uint) << MAX98927_PCM_MASTER_MODE_MCLK_RATE_SHIFT);
    }

    if !(*max98927).tdm_mode {
        /* BCLK configuration */
        value = max98927_get_bclk_sel(blr_clk_ratio);
        if value == 0 {
            dev_err((*component).dev, cstr!("format unsupported %d\n"), params_format(params));
            return -EINVAL;
        }
        regmap_update_bits((*max98927).regmap, MAX98927_R0022_PCM_CLK_SETUP,
                           MAX98927_PCM_CLK_SETUP_BSEL_MASK, value as c_uint);
    }
    0
}

unsafe extern "C" fn max98927_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let max98927 = snd_soc_component_get_drvdata(component);
    let mut sampling_rate: c_uint = 0;
    let chan_sz: c_uint;

    /* pcm mode configuration */
    match snd_pcm_format_width(params_format(params)) {
        16 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, cstr!("format unsupported %d\n"), params_format(params));
            return -EINVAL;
        }
    }

    (*max98927).ch_size = snd_pcm_format_width(params_format(params));

    regmap_update_bits((*max98927).regmap, MAX98927_R0020_PCM_MODE_CFG,
                       MAX98927_PCM_MODE_CFG_CHANSZ_MASK, chan_sz);

    dev_dbg((*component).dev, cstr!("format supported %d"), params_format(params));

    /* sampling rate configuration */
    match params_rate(params) {
        8000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_8000,
        11025 => sampling_rate = MAX98927_PCM_SR_SET1_SR_11025,
        12000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_12000,
        16000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_16000,
        22050 => sampling_rate = MAX98927_PCM_SR_SET1_SR_22050,
        24000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_24000,
        32000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_32000,
        44100 => sampling_rate = MAX98927_PCM_SR_SET1_SR_44100,
        48000 => sampling_rate = MAX98927_PCM_SR_SET1_SR_48000,
        _ => {
            dev_err((*component).dev, cstr!("rate %d not supported\n"), params_rate(params));
            return -EINVAL;
        }
    }

    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits((*max98927).regmap, MAX98927_R0023_PCM_SR_SETUP1,
                       MAX98927_PCM_SR_SET1_SR_MASK, sampling_rate);
    regmap_update_bits((*max98927).regmap, MAX98927_R0024_PCM_SR_SETUP2,
                       MAX98927_PCM_SR_SET2_SR_MASK,
                       sampling_rate << MAX98927_PCM_SR_SET2_SR_SHIFT);

    /* set sampling rate of IV */
    if (*max98927).interleave_mode && sampling_rate > MAX98927_PCM_SR_SET1_SR_16000 {
        regmap_update_bits((*max98927).regmap, MAX98927_R0024_PCM_SR_SETUP2,
                           MAX98927_PCM_SR_SET2_IVADC_SR_MASK, sampling_rate - 3);
    } else {
        regmap_update_bits((*max98927).regmap, MAX98927_R0024_PCM_SR_SETUP2,
                           MAX98927_PCM_SR_SET2_IVADC_SR_MASK, sampling_rate);
    }
    max98927_set_clock(max98927, params)
}

unsafe extern "C" fn max98927_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let max98927 = snd_soc_component_get_drvdata(component);
    let bsel: c_int;
    let chan_sz: c_uint;

    (*max98927).tdm_mode = true;

    /* BCLK configuration */
    bsel = max98927_get_bclk_sel(slots * slot_width);
    if bsel == 0 {
        dev_err((*component).dev, cstr!("BCLK %d not supported\n"), slots * slot_width);
        return -EINVAL;
    }

    regmap_update_bits((*max98927).regmap, MAX98927_R0022_PCM_CLK_SETUP,
                       MAX98927_PCM_CLK_SETUP_BSEL_MASK, bsel as c_uint);

    /* Channel size configuration */
    match slot_width {
        16 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98927_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, cstr!("format unsupported %d\n"), slot_width);
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98927).regmap, MAX98927_R0020_PCM_MODE_CFG,
                       MAX98927_PCM_MODE_CFG_CHANSZ_MASK, chan_sz);

    /* Rx slot configuration */
    regmap_write((*max98927).regmap, MAX98927_R0018_PCM_RX_EN_A, rx_mask & 0xFF);
    regmap_write((*max98927).regmap, MAX98927_R0019_PCM_RX_EN_B, (rx_mask & 0xFF00) >> 8);

    /* Tx slot configuration */
    regmap_write((*max98927).regmap, MAX98927_R001A_PCM_TX_EN_A, tx_mask & 0xFF);
    regmap_write((*max98927).regmap, MAX98927_R001B_PCM_TX_EN_B, (tx_mask & 0xFF00) >> 8);

    /* Tx slot Hi-Z configuration */
    regmap_write((*max98927).regmap, MAX98927_R001C_PCM_TX_HIZ_CTRL_A, (!tx_mask) & 0xFF);
    regmap_write((*max98927).regmap, MAX98927_R001D_PCM_TX_HIZ_CTRL_B, ((!tx_mask) & 0xFF00) >> 8);

    0
}

static MAX98927_RATES: c_uint = unsafe { SNDRV_PCM_RATE_8000_48000 };
static MAX98927_FORMATS: c_uint = unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE };

unsafe extern "C" fn max98927_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let max98927 = snd_soc_component_get_drvdata(component);

    (*max98927).sysclk = freq;
    0
}

static max98927_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(max98927_dai_set_sysclk),
    set_fmt: Some(max98927_dai_set_fmt),
    hw_params: Some(max98927_dai_hw_params),
    set_tdm_slot: Some(max98927_dai_tdm_slot),
};

unsafe extern "C" fn max98927_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98927 = snd_soc_component_get_drvdata(component);

    match event as c_uint {
        x if x == SND_SOC_DAPM_PRE_PMU => {
            (*max98927).tdm_mode = false;
        }
        x if x == SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*max98927).regmap, MAX98927_R003A_AMP_EN,
                               MAX98927_AMP_EN_MASK, 1);
            regmap_update_bits((*max98927).regmap, MAX98927_R00FF_GLOBAL_SHDN,
                               MAX98927_GLOBAL_EN_MASK, 1);
        }
        x if x == SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*max98927).regmap, MAX98927_R00FF_GLOBAL_SHDN,
                               MAX98927_GLOBAL_EN_MASK, 0);
            regmap_update_bits((*max98927).regmap, MAX98927_R003A_AMP_EN,
                               MAX98927_AMP_EN_MASK, 0);
        }
        _ => return 0,
    }
    0
}

static max98927_switch_text: [*const c_char; 3] = [
    cstr!("Left"), cstr!("Right"), cstr!("LeftRight")
];

static dai_sel_enum: soc_enum = unsafe {
    soc_enum {
        reg: MAX98927_R0025_PCM_TO_SPK_MONOMIX_A,
        shift_l: MAX98927_PCM_TO_SPK_MONOMIX_CFG_SHIFT,
        items: 3,
        texts: max98927_switch_text.as_ptr(),
    }
};

static max98927_dai_controls: snd_kcontrol_new = snd_kcontrol_new { name: cstr!("DAI Sel") };
static max98927_vi_control: snd_kcontrol_new = snd_kcontrol_new { name: cstr!("Switch") };

static max98927_dapm_widgets: [snd_soc_dapm_widget_desc; 8] = [
    snd_soc_dapm_widget_desc { name: cstr!("Amp Enable") },
    snd_soc_dapm_widget_desc { name: cstr!("DAI Sel Mux") },
    snd_soc_dapm_widget_desc { name: cstr!("BE_OUT") },
    snd_soc_dapm_widget_desc { name: cstr!("Voltage Sense") },
    snd_soc_dapm_widget_desc { name: cstr!("Current Sense") },
    snd_soc_dapm_widget_desc { name: cstr!("VI Sense") },
    snd_soc_dapm_widget_desc { name: cstr!("VMON") },
    snd_soc_dapm_widget_desc { name: cstr!("IMON") },
];

/* DECLARE_TLV_DB_SCALE(max98927_spk_tlv, 300, 300, 0); */
static max98927_spk_tlv: [c_uint; 4] = [0, 300u32, 300u32, 0];
/* DECLARE_TLV_DB_SCALE(max98927_digital_tlv, -1600, 25, 0); */
static max98927_digital_tlv: [c_int; 4] = [0, -1600, 25, 0];

unsafe extern "C" fn max98927_readable_register(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        x if x >= MAX98927_R0001_INT_RAW1 && x <= MAX98927_R0028_ICC_RX_EN_B => true,
        x if x >= MAX98927_R002B_ICC_TX_EN_A && x <= MAX98927_R002C_ICC_TX_EN_B => true,
        x if x >= MAX98927_R002E_ICC_HIZ_MANUAL_MODE && x <= MAX98927_R004E_MEAS_ADC_CH2_READ => true,
        x if x >= MAX98927_R0051_BROWNOUT_STATUS && x <= MAX98927_R0055_BROWNOUT_LVL_HOLD => true,
        x if x >= MAX98927_R005A_BROWNOUT_LVL1_THRESH && x <= MAX98927_R0061_BROWNOUT_AMP1_CLIP_MODE => true,
        x if x >= MAX98927_R0072_BROWNOUT_LVL1_CUR_LIMIT && x <= MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ => true,
        x if x == MAX98927_R00FF_GLOBAL_SHDN => true,
        x if x == MAX98927_R0100_SOFT_RESET => true,
        x if x == MAX98927_R01FF_REV_ID => true,
        _ => false,
    }
}

unsafe extern "C" fn max98927_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        x if x >= MAX98927_R0001_INT_RAW1 && x <= MAX98927_R0009_INT_FLAG3 => true,
        x if x == MAX98927_R004C_MEAS_ADC_CH0_READ => true,
        x if x == MAX98927_R004D_MEAS_ADC_CH1_READ => true,
        x if x == MAX98927_R004E_MEAS_ADC_CH2_READ => true,
        x if x == MAX98927_R0051_BROWNOUT_STATUS => true,
        x if x == MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ => true,
        x if x == MAX98927_R01FF_REV_ID => true,
        x if x == MAX98927_R0100_SOFT_RESET => true,
        _ => false,
    }
}

static max98927_boost_voltage_text: [*const c_char; 29] = [
    cstr!("6.5V"), cstr!("6.625V"), cstr!("6.75V"), cstr!("6.875V"),
    cstr!("7V"), cstr!("7.125V"), cstr!("7.25V"), cstr!("7.375V"),
    cstr!("7.5V"), cstr!("7.625V"), cstr!("7.75V"), cstr!("7.875V"),
    cstr!("8V"), cstr!("8.125V"), cstr!("8.25V"), cstr!("8.375V"),
    cstr!("8.5V"), cstr!("8.625V"), cstr!("8.75V"), cstr!("8.875V"),
    cstr!("9V"), cstr!("9.125V"), cstr!("9.25V"), cstr!("9.375V"),
    cstr!("9.5V"), cstr!("9.625V"), cstr!("9.75V"), cstr!("9.875V"),
    cstr!("10V"),
];

static max98927_boost_voltage: soc_enum = unsafe {
    soc_enum {
        reg: MAX98927_R0040_BOOST_CTRL0,
        shift_l: 0,
        items: 29,
        texts: max98927_boost_voltage_text.as_ptr(),
    }
};

static max98927_current_limit_text: [*const c_char; 32] = [
    cstr!("1.00A"), cstr!("1.10A"), cstr!("1.20A"), cstr!("1.30A"),
    cstr!("1.40A"), cstr!("1.50A"), cstr!("1.60A"), cstr!("1.70A"),
    cstr!("1.80A"), cstr!("1.90A"), cstr!("2.00A"), cstr!("2.10A"),
    cstr!("2.20A"), cstr!("2.30A"), cstr!("2.40A"), cstr!("2.50A"),
    cstr!("2.60A"), cstr!("2.70A"), cstr!("2.80A"), cstr!("2.90A"),
    cstr!("3.00A"), cstr!("3.10A"), cstr!("3.20A"), cstr!("3.30A"),
    cstr!("3.40A"), cstr!("3.50A"), cstr!("3.60A"), cstr!("3.70A"),
    cstr!("3.80A"), cstr!("3.90A"), cstr!("4.00A"), cstr!("4.10A"),
];

static max98927_current_limit: soc_enum = unsafe {
    soc_enum {
        reg: MAX98927_R0042_BOOST_CTRL1,
        shift_l: 1,
        items: 32,
        texts: max98927_current_limit_text.as_ptr(),
    }
};

static max98927_snd_controls: [snd_kcontrol_new; 8] = [
    snd_kcontrol_new { name: cstr!("Speaker Volume") },
    snd_kcontrol_new { name: cstr!("Digital Volume") },
    snd_kcontrol_new { name: cstr!("Amp DSP Switch") },
    snd_kcontrol_new { name: cstr!("Ramp Switch") },
    snd_kcontrol_new { name: cstr!("DRE Switch") },
    snd_kcontrol_new { name: cstr!("Volume Location Switch") },
    snd_kcontrol_new { name: cstr!("Boost Output Voltage") },
    snd_kcontrol_new { name: cstr!("Current Limit") },
];

static max98927_audio_map: [snd_soc_dapm_route; 8] = [
    /* Plabyack */
    snd_soc_dapm_route { sink: cstr!("DAI Sel Mux"), control: cstr!("Left"), source: cstr!("Amp Enable") },
    snd_soc_dapm_route { sink: cstr!("DAI Sel Mux"), control: cstr!("Right"), source: cstr!("Amp Enable") },
    snd_soc_dapm_route { sink: cstr!("DAI Sel Mux"), control: cstr!("LeftRight"), source: cstr!("Amp Enable") },
    snd_soc_dapm_route { sink: cstr!("BE_OUT"), control: core::ptr::null(), source: cstr!("DAI Sel Mux") },
    /* Capture */
    snd_soc_dapm_route { sink: cstr!("VI Sense"), control: cstr!("Switch"), source: cstr!("VMON") },
    snd_soc_dapm_route { sink: cstr!("VI Sense"), control: cstr!("Switch"), source: cstr!("IMON") },
    snd_soc_dapm_route { sink: cstr!("Voltage Sense"), control: core::ptr::null(), source: cstr!("VI Sense") },
    snd_soc_dapm_route { sink: cstr!("Current Sense"), control: core::ptr::null(), source: cstr!("VI Sense") },
];

static mut max98927_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: cstr!("max98927-aif1"),
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("HiFi Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("HiFi Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE },
        },
        ops: &max98927_dai_ops,
    },
];

unsafe extern "C" fn max98927_probe(component: *mut snd_soc_component) -> c_int {
    let max98927 = snd_soc_component_get_drvdata(component);

    (*max98927).component = component;

    /* Software Reset */
    regmap_write((*max98927).regmap, MAX98927_R0100_SOFT_RESET, MAX98927_SOFT_RESET);

    /* IV default slot configuration */
    regmap_write((*max98927).regmap, MAX98927_R001C_PCM_TX_HIZ_CTRL_A, 0xFF);
    regmap_write((*max98927).regmap, MAX98927_R001D_PCM_TX_HIZ_CTRL_B, 0xFF);
    regmap_write((*max98927).regmap, MAX98927_R0025_PCM_TO_SPK_MONOMIX_A, 0x80);
    regmap_write((*max98927).regmap, MAX98927_R0026_PCM_TO_SPK_MONOMIX_B, 0x1);
    /* Set inital volume (+13dB) */
    regmap_write((*max98927).regmap, MAX98927_R0036_AMP_VOL_CTRL, 0x38);
    regmap_write((*max98927).regmap, MAX98927_R003C_SPK_GAIN, 0x05);
    /* Enable DC blocker */
    regmap_write((*max98927).regmap, MAX98927_R0037_AMP_DSP_CFG, 0x03);
    /* Enable IMON VMON DC blocker */
    regmap_write((*max98927).regmap, MAX98927_R003F_MEAS_DSP_CFG, 0xF7);
    /* Boost Output Voltage & Current limit */
    regmap_write((*max98927).regmap, MAX98927_R0040_BOOST_CTRL0, 0x1C);
    regmap_write((*max98927).regmap, MAX98927_R0042_BOOST_CTRL1, 0x3E);
    /* Measurement ADC config */
    regmap_write((*max98927).regmap, MAX98927_R0043_MEAS_ADC_CFG, 0x04);
    regmap_write((*max98927).regmap, MAX98927_R0044_MEAS_ADC_BASE_MSB, 0x00);
    regmap_write((*max98927).regmap, MAX98927_R0045_MEAS_ADC_BASE_LSB, 0x24);
    /* Brownout Level */
    regmap_write((*max98927).regmap, MAX98927_R007F_BROWNOUT_LVL4_AMP1_CTRL1, 0x06);
    /* Envelope Tracking configuration */
    regmap_write((*max98927).regmap, MAX98927_R0082_ENV_TRACK_VOUT_HEADROOM, 0x08);
    regmap_write((*max98927).regmap, MAX98927_R0086_ENV_TRACK_CTRL, 0x01);
    regmap_write((*max98927).regmap, MAX98927_R0087_ENV_TRACK_BOOST_VOUT_READ, 0x10);

    /* voltage, current slot configuration */
    regmap_write(
        (*max98927).regmap,
        MAX98927_R001E_PCM_TX_CH_SRC_A,
        (((*max98927).i_l_slot << MAX98927_PCM_TX_CH_SRC_A_I_SHIFT) | (*max98927).v_l_slot) as c_uint & 0xFF,
    );

    if (*max98927).v_l_slot < 8 {
        regmap_update_bits((*max98927).regmap, MAX98927_R001C_PCM_TX_HIZ_CTRL_A,
                           1u32 << (*max98927).v_l_slot, 0);
        regmap_update_bits((*max98927).regmap, MAX98927_R001A_PCM_TX_EN_A,
                           1u32 << (*max98927).v_l_slot,
                           1u32 << (*max98927).v_l_slot);
    } else {
        regmap_update_bits((*max98927).regmap, MAX98927_R001D_PCM_TX_HIZ_CTRL_B,
                           1u32 << ((*max98927).v_l_slot - 8), 0);
        regmap_update_bits((*max98927).regmap, MAX98927_R001B_PCM_TX_EN_B,
                           1u32 << ((*max98927).v_l_slot - 8),
                           1u32 << ((*max98927).v_l_slot - 8));
    }

    if (*max98927).i_l_slot < 8 {
        regmap_update_bits((*max98927).regmap, MAX98927_R001C_PCM_TX_HIZ_CTRL_A,
                           1u32 << (*max98927).i_l_slot, 0);
        regmap_update_bits((*max98927).regmap, MAX98927_R001A_PCM_TX_EN_A,
                           1u32 << (*max98927).i_l_slot,
                           1u32 << (*max98927).i_l_slot);
    } else {
        regmap_update_bits((*max98927).regmap, MAX98927_R001D_PCM_TX_HIZ_CTRL_B,
                           1u32 << ((*max98927).i_l_slot - 8), 0);
        regmap_update_bits((*max98927).regmap, MAX98927_R001B_PCM_TX_EN_B,
                           1u32 << ((*max98927).i_l_slot - 8),
                           1u32 << ((*max98927).i_l_slot - 8));
    }

    /* Set interleave mode */
    if (*max98927).interleave_mode {
        regmap_update_bits((*max98927).regmap, MAX98927_R001F_PCM_TX_CH_SRC_B,
                           MAX98927_PCM_TX_CH_INTERLEAVE_MASK,
                           MAX98927_PCM_TX_CH_INTERLEAVE_MASK);
    }
    0
}

unsafe extern "C" fn max98927_suspend(dev: *mut device) -> c_int {
    let max98927 = dev_get_drvdata(dev);

    regcache_cache_only((*max98927).regmap, true);
    regcache_mark_dirty((*max98927).regmap);
    0
}

unsafe extern "C" fn max98927_resume(dev: *mut device) -> c_int {
    let max98927 = dev_get_drvdata(dev);
    let ret: c_int;

    regmap_write((*max98927).regmap, MAX98927_R0100_SOFT_RESET, MAX98927_SOFT_RESET);
    regcache_cache_only((*max98927).regmap, false);
    ret = regcache_sync((*max98927).regmap);
    if ret != 0 {
        regcache_cache_only((*max98927).regmap, true);
        regcache_mark_dirty((*max98927).regmap);
        return ret;
    }
    0
}

static max98927_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(max98927_suspend),
    resume: Some(max98927_resume),
};

static soc_component_dev_max98927: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98927_probe),
    controls: max98927_snd_controls.as_ptr(),
    num_controls: max98927_snd_controls.len() as c_uint,
    dapm_widgets: max98927_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98927_dapm_widgets.len() as c_uint,
    dapm_routes: max98927_audio_map.as_ptr(),
    num_dapm_routes: max98927_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static max98927_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 16,
        val_bits: 8,
        max_register: MAX98927_R01FF_REV_ID,
        reg_defaults: max98927_reg.as_ptr(),
        num_reg_defaults: max98927_reg.len() as c_uint,
        readable_reg: Some(max98927_readable_register),
        volatile_reg: Some(max98927_volatile_reg),
        cache_type: REGCACHE_RBTREE,
    }
};

unsafe fn max98927_slot_config(i2c: *mut i2c_client, max98927: *mut max98927_priv) {
    let mut value: c_int = 0;
    let dev = &mut (*i2c).dev as *mut device;

    if device_property_read_u32(dev, cstr!("vmon-slot-no"), &mut value) == 0 {
        (*max98927).v_l_slot = value & 0xF;
    } else {
        (*max98927).v_l_slot = 0;
    }

    if device_property_read_u32(dev, cstr!("imon-slot-no"), &mut value) == 0 {
        (*max98927).i_l_slot = value & 0xF;
    } else {
        (*max98927).i_l_slot = 1;
    }
}

unsafe extern "C" fn max98927_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int = 0;
    let mut value: c_int = 0;
    let mut reg: c_int = 0;
    let mut max98927: *mut max98927_priv = core::ptr::null_mut();

    max98927 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<max98927_priv>(), GFP_KERNEL) as *mut max98927_priv;
    if max98927.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    i2c_set_clientdata(i2c, max98927);

    /* update interleave mode info */
    if of_property_read_bool((*i2c).dev.of_node, cstr!("maxim,interleave-mode")) {
        (*max98927).interleave_mode = true;
    } else {
        if of_property_read_u32((*i2c).dev.of_node, cstr!("interleave_mode"), &mut value) == 0 {
            if value > 0 {
                (*max98927).interleave_mode = true;
            }
        }
    }

    /* regmap initialization */
    (*max98927).regmap = devm_regmap_init_i2c(i2c, &max98927_regmap);
    if IS_ERR((*max98927).regmap as *const core::ffi::c_void) {
        ret = PTR_ERR((*max98927).regmap as *const core::ffi::c_void);
        dev_err(&mut (*i2c).dev, cstr!("Failed to allocate regmap: %d\n"), ret);
        return ret;
    }

    (*max98927).reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, cstr!("reset"), GPIOD_OUT_HIGH);
    if IS_ERR((*max98927).reset_gpio as *const core::ffi::c_void) {
        ret = PTR_ERR((*max98927).reset_gpio as *const core::ffi::c_void);
        return dev_err_probe(&mut (*i2c).dev, ret, cstr!("failed to request GPIO reset pin"));
    }

    if !(*max98927).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*max98927).reset_gpio, 0);
        /* Wait for i2c port to be ready */
        usleep_range(5000, 6000);
    }

    /* Check Revision ID */
    ret = regmap_read((*max98927).regmap, MAX98927_R01FF_REV_ID, &mut reg);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, cstr!("Failed to read: 0x%02X\n"), MAX98927_R01FF_REV_ID);
        return ret;
    }
    dev_info(&mut (*i2c).dev, cstr!("MAX98927 revisionID: 0x%02X\n"), reg);

    /* voltage/current slot configuration */
    max98927_slot_config(i2c, max98927);

    /* codec registeration */
    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_max98927,
        max98927_dai.as_mut_ptr(),
        max98927_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(&mut (*i2c).dev, cstr!("Failed to register component: %d\n"), ret);
    }

    ret
}

unsafe extern "C" fn max98927_i2c_remove(i2c: *mut i2c_client) {
    let max98927 = i2c_get_clientdata(i2c);

    if !(*max98927).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*max98927).reset_gpio, 1);
    }
}

static max98927_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: cstr!("max98927"), driver_data: 0 },
    i2c_device_id { name: core::ptr::null(), driver_data: 0 },
];

/* MODULE_DEVICE_TABLE(i2c, max98927_i2c_id); */

/* #if defined(CONFIG_OF) */
static max98927_of_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("maxim,max98927") },
    of_device_id { compatible: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, max98927_of_match); */
/* #endif */

/* #ifdef CONFIG_ACPI */
static max98927_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: cstr!("MX98927"), driver_data: 0 },
    acpi_device_id { id: core::ptr::null(), driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(acpi, max98927_acpi_match); */
/* #endif */

static mut max98927_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: cstr!("max98927"),
        of_match_table: max98927_of_match.as_ptr(),
        acpi_match_table: max98927_acpi_match.as_ptr(),
        pm: &max98927_pm,
    },
    probe: Some(max98927_i2c_probe),
    remove: Some(max98927_i2c_remove),
    id_table: max98927_i2c_id.as_ptr(),
};

/* module_i2c_driver(max98927_i2c_driver) */

/* MODULE_DESCRIPTION("ALSA SoC MAX98927 driver"); */
/* MODULE_AUTHOR("Ryan Lee <ryans.lee@maximintegrated.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
