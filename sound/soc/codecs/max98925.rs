// SPDX-License-Identifier: GPL-2.0-only
/*
 * max98925.c -- ALSA SoC Stereo MAX98925 driver
 * Copyright 2013-15 Maxim Integrated Products
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

// C dependencies removed from executable Rust:
// linux/delay.h, linux/i2c.h, linux/module.h, linux/regmap.h, linux/slab.h,
// linux/cdev.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h,
// and "max98925.h".

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
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
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device_with_of_node,
}

#[repr(C)]
pub struct device_with_of_node {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct max98925_priv {
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub sysclk: c_uint,
    pub ch_size: c_int,
    pub v_slot: c_uint,
    pub i_slot: c_uint,
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
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct rate_table_entry {
    pub rate: c_int,
    pub sr: c_int,
    pub divisors: [[c_int; 2]; 3],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk: Option<
        unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
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
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
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
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut max98925_priv;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device_with_of_node, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_property_read_u32(
        node: *mut device_node,
        propname: *const c_char,
        out_value: *mut u32,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device_with_of_node,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(matches: *const of_device_id) -> *const of_device_id;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe extern "C" {
    static MAX98925_CONFIGURATION: c_uint;
    static MAX98925_GAIN: c_uint;
    static MAX98925_FILTERS: c_uint;
    static MAX98925_BLOCK_ENABLE: c_uint;
    static MAX98925_GLOBAL_ENABLE: c_uint;
    static MAX98925_VBAT_DATA: c_uint;
    static MAX98925_VBST_DATA: c_uint;
    static MAX98925_LIVE_STATUS0: c_uint;
    static MAX98925_LIVE_STATUS1: c_uint;
    static MAX98925_LIVE_STATUS2: c_uint;
    static MAX98925_STATE0: c_uint;
    static MAX98925_STATE1: c_uint;
    static MAX98925_STATE2: c_uint;
    static MAX98925_FLAG0: c_uint;
    static MAX98925_FLAG1: c_uint;
    static MAX98925_FLAG2: c_uint;
    static MAX98925_REV_VERSION: c_uint;
    static MAX98925_IRQ_CLEAR0: c_uint;
    static MAX98925_IRQ_CLEAR1: c_uint;
    static MAX98925_IRQ_CLEAR2: c_uint;
    static MAX98925_ALC_HOLD_RLS: c_uint;
    static MAX98925_DOUT_CFG_VMON: c_uint;
    static MAX98925_DOUT_CFG_IMON: c_uint;
    static MAX98925_DAI_CLK_MODE1: c_uint;
    static MAX98925_DAI_CLK_MODE2: c_uint;
    static MAX98925_FORMAT: c_uint;
    static MAX98925_DAI_CLK_DIV_M_MSBS: c_uint;
    static MAX98925_DAI_CLK_DIV_M_LSBS: c_uint;
    static MAX98925_DAI_CLK_DIV_N_MSBS: c_uint;
    static MAX98925_DAI_CLK_DIV_N_LSBS: c_uint;
    static MAX98925_GAIN_RAMPING: c_uint;
    static MAX98925_THRESHOLD: c_uint;
    static MAX98925_TDM_SLOT_SELECT: c_uint;
    static MAX98925_DOUT_HIZ_CFG1: c_uint;
    static MAX98925_DOUT_HIZ_CFG2: c_uint;
    static MAX98925_DOUT_HIZ_CFG3: c_uint;
    static MAX98925_DOUT_HIZ_CFG4: c_uint;
    static MAX98925_ALC_CONFIGURATION: c_uint;
    static MAX98925_BOOST_LIMITER: c_uint;
    static MAX98925_VERSION: c_int;
    static MAX98925_VERSION1: c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 0;
const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
const M98925_BST_VOUT_SHIFT: c_uint = 0;
const M98925_BST_EN_MASK: c_uint = 0;
const M98925_ADC_IMON_EN_MASK: c_uint = 0;
const M98925_ADC_VMON_EN_MASK: c_uint = 0;
const M98925_DAI_VMON_EN_MASK: c_uint = 0;
const M98925_DAI_VMON_SLOT_MASK: c_uint = 0;
const M98925_DAI_VMON_SLOT_SHIFT: c_uint = 0;
const M98925_DAI_IMON_EN_MASK: c_uint = 0;
const M98925_DAI_IMON_SLOT_MASK: c_uint = 0;
const M98925_DAI_IMON_SLOT_SHIFT: c_uint = 0;
const M98925_DAI_MAS_MASK: c_uint = 0;
const M98925_DAI_WCI_MASK: c_uint = 0;
const M98925_DAI_BCI_MASK: c_uint = 0;
const M98925_DAI_BSEL_MASK: c_uint = 0;
const M98925_DAI_BSEL_32: c_uint = 0;
const M98925_DAI_BSEL_48: c_uint = 0;
const M98925_DAI_BSEL_64: c_uint = 0;
const M98925_MDLL_MULT_MCLKx16: c_uint = 0;
const M98925_MDLL_MULT_MCLKx8: c_uint = 0;
const M98925_DAI_SR_MASK: c_uint = 0;
const M98925_DAI_SR_SHIFT: c_uint = 0;
const M98925_MDLL_MULT_MASK: c_uint = 0;
const M98925_MDLL_MULT_SHIFT: c_uint = 0;
const M98925_DAI_CHANSZ_MASK: c_uint = 0;
const M98925_DAI_CHANSZ_16: c_uint = 0;
const M98925_DAI_CHANSZ_24: c_uint = 0;
const M98925_DAI_CHANSZ_32: c_uint = 0;
const M98925_DAI_CLK_SOURCE_MASK: c_uint = 0;
const M98925_DAI_DLY_MASK: c_uint = 0;
const M98925_SPK_EN_SHIFT: c_uint = 0;
const M98925_SPK_GAIN_SHIFT: c_uint = 0;
const M98925_SPK_GAIN_WIDTH: c_uint = 0;
const M98925_SPK_RMP_EN_SHIFT: c_uint = 0;
const M98925_SPK_ZCD_EN_SHIFT: c_uint = 0;
const M98925_ALC_EN_SHIFT: c_uint = 0;
const M98925_ALC_TH_SHIFT: c_uint = 0;
const M98925_ALC_TH_WIDTH: c_uint = 0;
const M98925_DAI_VMON_SLOT_1E_1F: u32 = 0;
const M98925_DAI_IMON_SLOT_1E_1F: u32 = 0;

static DAI_TEXT: [*const c_char; 4] = [
    b"Left\0".as_ptr().cast(),
    b"Right\0".as_ptr().cast(),
    b"LeftRight\0".as_ptr().cast(),
    b"LeftRightDiv2\0".as_ptr().cast(),
];

static MAX98925_BOOST_VOLTAGE_TEXT: [*const c_char; 16] = [
    b"8.5V\0".as_ptr().cast(),
    b"8.25V\0".as_ptr().cast(),
    b"8.0V\0".as_ptr().cast(),
    b"7.75V\0".as_ptr().cast(),
    b"7.5V\0".as_ptr().cast(),
    b"7.25V\0".as_ptr().cast(),
    b"7.0V\0".as_ptr().cast(),
    b"6.75V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
    b"6.5V\0".as_ptr().cast(),
];

// static SOC_ENUM_SINGLE_DECL(max98925_boost_voltage,
//     MAX98925_CONFIGURATION, M98925_BST_VOUT_SHIFT,
//     max98925_boost_voltage_text);
static MAX98925_BOOST_VOLTAGE: soc_enum = soc_enum { _private: [] };

static HPF_TEXT: [*const c_char; 6] = [
    b"Disable\0".as_ptr().cast(),
    b"DC Block\0".as_ptr().cast(),
    b"100Hz\0".as_ptr().cast(),
    b"200Hz\0".as_ptr().cast(),
    b"400Hz\0".as_ptr().cast(),
    b"800Hz\0".as_ptr().cast(),
];

static MAX98925_REG: [reg_default; 45] = [
    reg_default { reg: 0x0B, def: 0x00 }, /* IRQ Enable0 */
    reg_default { reg: 0x0C, def: 0x00 }, /* IRQ Enable1 */
    reg_default { reg: 0x0D, def: 0x00 }, /* IRQ Enable2 */
    reg_default { reg: 0x0E, def: 0x00 }, /* IRQ Clear0 */
    reg_default { reg: 0x0F, def: 0x00 }, /* IRQ Clear1 */
    reg_default { reg: 0x10, def: 0x00 }, /* IRQ Clear2 */
    reg_default { reg: 0x11, def: 0xC0 }, /* Map0 */
    reg_default { reg: 0x12, def: 0x00 }, /* Map1 */
    reg_default { reg: 0x13, def: 0x00 }, /* Map2 */
    reg_default { reg: 0x14, def: 0xF0 }, /* Map3 */
    reg_default { reg: 0x15, def: 0x00 }, /* Map4 */
    reg_default { reg: 0x16, def: 0xAB }, /* Map5 */
    reg_default { reg: 0x17, def: 0x89 }, /* Map6 */
    reg_default { reg: 0x18, def: 0x00 }, /* Map7 */
    reg_default { reg: 0x19, def: 0x00 }, /* Map8 */
    reg_default { reg: 0x1A, def: 0x06 }, /* DAI Clock Mode 1 */
    reg_default { reg: 0x1B, def: 0xC0 }, /* DAI Clock Mode 2 */
    reg_default { reg: 0x1C, def: 0x00 }, /* DAI Clock Divider Denominator MSBs */
    reg_default { reg: 0x1D, def: 0x00 }, /* DAI Clock Divider Denominator LSBs */
    reg_default { reg: 0x1E, def: 0xF0 }, /* DAI Clock Divider Numerator MSBs */
    reg_default { reg: 0x1F, def: 0x00 }, /* DAI Clock Divider Numerator LSBs */
    reg_default { reg: 0x20, def: 0x50 }, /* Format */
    reg_default { reg: 0x21, def: 0x00 }, /* TDM Slot Select */
    reg_default { reg: 0x22, def: 0x00 }, /* DOUT Configuration VMON */
    reg_default { reg: 0x23, def: 0x00 }, /* DOUT Configuration IMON */
    reg_default { reg: 0x24, def: 0x00 }, /* DOUT Configuration VBAT */
    reg_default { reg: 0x25, def: 0x00 }, /* DOUT Configuration VBST */
    reg_default { reg: 0x26, def: 0x00 }, /* DOUT Configuration FLAG */
    reg_default { reg: 0x27, def: 0xFF }, /* DOUT HiZ Configuration 1 */
    reg_default { reg: 0x28, def: 0xFF }, /* DOUT HiZ Configuration 2 */
    reg_default { reg: 0x29, def: 0xFF }, /* DOUT HiZ Configuration 3 */
    reg_default { reg: 0x2A, def: 0xFF }, /* DOUT HiZ Configuration 4 */
    reg_default { reg: 0x2B, def: 0x02 }, /* DOUT Drive Strength */
    reg_default { reg: 0x2C, def: 0x90 }, /* Filters */
    reg_default { reg: 0x2D, def: 0x00 }, /* Gain */
    reg_default { reg: 0x2E, def: 0x02 }, /* Gain Ramping */
    reg_default { reg: 0x2F, def: 0x00 }, /* Speaker Amplifier */
    reg_default { reg: 0x30, def: 0x0A }, /* Threshold */
    reg_default { reg: 0x31, def: 0x00 }, /* ALC Attack */
    reg_default { reg: 0x32, def: 0x80 }, /* ALC Atten and Release */
    reg_default { reg: 0x33, def: 0x00 }, /* ALC Infinite Hold Release */
    reg_default { reg: 0x34, def: 0x92 }, /* ALC Configuration */
    reg_default { reg: 0x35, def: 0x01 }, /* Boost Converter */
    reg_default { reg: 0x36, def: 0x00 }, /* Block Enable */
    reg_default { reg: 0x37, def: 0x00 }, /* Configuration */
    reg_default { reg: 0x38, def: 0x00 }, /* Global Enable */
    reg_default { reg: 0x3A, def: 0x00 }, /* Boost Limiter */
];

// static const struct soc_enum max98925_dai_enum =
//     SOC_ENUM_SINGLE(MAX98925_GAIN, 5, ARRAY_SIZE(dai_text), dai_text);
static MAX98925_DAI_ENUM: soc_enum = soc_enum { _private: [] };

// static const struct soc_enum max98925_hpf_enum =
//     SOC_ENUM_SINGLE(MAX98925_FILTERS, 0, ARRAY_SIZE(hpf_text), hpf_text);
static MAX98925_HPF_ENUM: soc_enum = soc_enum { _private: [] };

// static const struct snd_kcontrol_new max98925_hpf_sel_mux =
//     SOC_DAPM_ENUM("Rc Filter MUX Mux", max98925_hpf_enum);
static MAX98925_HPF_SEL_MUX: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

// static const struct snd_kcontrol_new max98925_dai_sel_mux =
//     SOC_DAPM_ENUM("DAI IN MUX Mux", max98925_dai_enum);
static MAX98925_DAI_SEL_MUX: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

unsafe extern "C" fn max98925_dac_event(
    _w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let max98925 = unsafe { snd_soc_component_get_drvdata(component) };

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            unsafe {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_BLOCK_ENABLE,
                    M98925_BST_EN_MASK | M98925_ADC_IMON_EN_MASK | M98925_ADC_VMON_EN_MASK,
                    M98925_BST_EN_MASK | M98925_ADC_IMON_EN_MASK | M98925_ADC_VMON_EN_MASK,
                );
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            unsafe {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_BLOCK_ENABLE,
                    M98925_BST_EN_MASK | M98925_ADC_IMON_EN_MASK | M98925_ADC_VMON_EN_MASK,
                    0,
                );
            }
        }
        _ => return 0,
    }
    0
}

// static const struct snd_soc_dapm_widget max98925_dapm_widgets[] = {
//     SND_SOC_DAPM_AIF_IN("DAI_OUT", "HiFi Playback", 0, SND_SOC_NOPM, 0, 0),
//     SND_SOC_DAPM_MUX("DAI IN MUX", SND_SOC_NOPM, 0, 0, &max98925_dai_sel_mux),
//     SND_SOC_DAPM_MUX("Rc Filter MUX", SND_SOC_NOPM, 0, 0, &max98925_hpf_sel_mux),
//     SND_SOC_DAPM_DAC_E("Amp Enable", NULL, MAX98925_BLOCK_ENABLE,
//         M98925_SPK_EN_SHIFT, 0, max98925_dac_event,
//         SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
//     SND_SOC_DAPM_SUPPLY("Global Enable", MAX98925_GLOBAL_ENABLE,
//         M98925_EN_SHIFT, 0, NULL, 0),
//     SND_SOC_DAPM_OUTPUT("BE_OUT"),
// };
static MAX98925_DAPM_WIDGETS: [snd_soc_dapm_widget_def; 0] = [];

static MAX98925_AUDIO_MAP: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: b"DAI IN MUX\0".as_ptr().cast(), control: b"Left\0".as_ptr().cast(), source: b"DAI_OUT\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"DAI IN MUX\0".as_ptr().cast(), control: b"Right\0".as_ptr().cast(), source: b"DAI_OUT\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"DAI IN MUX\0".as_ptr().cast(), control: b"LeftRight\0".as_ptr().cast(), source: b"DAI_OUT\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"DAI IN MUX\0".as_ptr().cast(), control: b"LeftRightDiv2\0".as_ptr().cast(), source: b"DAI_OUT\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"Disable\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"DC Block\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"100Hz\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"200Hz\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"400Hz\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Rc Filter MUX\0".as_ptr().cast(), control: b"800Hz\0".as_ptr().cast(), source: b"DAI IN MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Amp Enable\0".as_ptr().cast(), control: ptr::null(), source: b"Rc Filter MUX\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"BE_OUT\0".as_ptr().cast(), control: ptr::null(), source: b"Amp Enable\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"BE_OUT\0".as_ptr().cast(), control: ptr::null(), source: b"Global Enable\0".as_ptr().cast() },
];

unsafe extern "C" fn max98925_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    unsafe {
        match reg {
            r if r == MAX98925_VBAT_DATA
                || r == MAX98925_VBST_DATA
                || r == MAX98925_LIVE_STATUS0
                || r == MAX98925_LIVE_STATUS1
                || r == MAX98925_LIVE_STATUS2
                || r == MAX98925_STATE0
                || r == MAX98925_STATE1
                || r == MAX98925_STATE2
                || r == MAX98925_FLAG0
                || r == MAX98925_FLAG1
                || r == MAX98925_FLAG2
                || r == MAX98925_REV_VERSION =>
            {
                true
            }
            _ => false,
        }
    }
}

unsafe extern "C" fn max98925_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    unsafe {
        match reg {
            r if r == MAX98925_IRQ_CLEAR0
                || r == MAX98925_IRQ_CLEAR1
                || r == MAX98925_IRQ_CLEAR2
                || r == MAX98925_ALC_HOLD_RLS =>
            {
                false
            }
            _ => true,
        }
    }
}

// static DECLARE_TLV_DB_SCALE(max98925_spk_tlv, -600, 100, 0);
static MAX98925_SPK_TLV: [c_uint; 0] = [];

// static const struct snd_kcontrol_new max98925_snd_controls[] = {
//     SOC_SINGLE_TLV("Speaker Volume", MAX98925_GAIN,
//         M98925_SPK_GAIN_SHIFT, (1<<M98925_SPK_GAIN_WIDTH)-1, 0,
//         max98925_spk_tlv),
//     SOC_SINGLE("Ramp Switch", MAX98925_GAIN_RAMPING,
//         M98925_SPK_RMP_EN_SHIFT, 1, 0),
//     SOC_SINGLE("ZCD Switch", MAX98925_GAIN_RAMPING,
//         M98925_SPK_ZCD_EN_SHIFT, 1, 0),
//     SOC_SINGLE("ALC Switch", MAX98925_THRESHOLD,
//         M98925_ALC_EN_SHIFT, 1, 0),
//     SOC_SINGLE("ALC Threshold", MAX98925_THRESHOLD, M98925_ALC_TH_SHIFT,
//         (1<<M98925_ALC_TH_WIDTH)-1, 0),
//     SOC_ENUM("Boost Output Voltage", max98925_boost_voltage),
// };
static MAX98925_SND_CONTROLS: [snd_kcontrol_new; 0] = [];

/* codec sample rate and n/m dividers parameter table */
static RATE_TABLE: [rate_table_entry; 9] = [
    rate_table_entry { rate: 8000, sr: 0, divisors: [[1, 375], [5, 1764], [1, 384]] },
    rate_table_entry { rate: 11025, sr: 1, divisors: [[147, 40000], [1, 256], [147, 40960]] },
    rate_table_entry { rate: 12000, sr: 2, divisors: [[1, 250], [5, 1176], [1, 256]] },
    rate_table_entry { rate: 16000, sr: 3, divisors: [[2, 375], [5, 882], [1, 192]] },
    rate_table_entry { rate: 22050, sr: 4, divisors: [[147, 20000], [1, 128], [147, 20480]] },
    rate_table_entry { rate: 24000, sr: 5, divisors: [[1, 125], [5, 588], [1, 128]] },
    rate_table_entry { rate: 32000, sr: 6, divisors: [[4, 375], [5, 441], [1, 96]] },
    rate_table_entry { rate: 44100, sr: 7, divisors: [[147, 10000], [1, 64], [147, 10240]] },
    rate_table_entry { rate: 48000, sr: 8, divisors: [[2, 125], [5, 294], [1, 64]] },
];

unsafe fn max98925_rate_value(
    _component: *mut snd_soc_component,
    rate: c_int,
    clock: c_int,
    value: *mut c_int,
    n: *mut c_int,
    m: *mut c_int,
) -> c_int {
    let mut ret = -EINVAL;
    let mut i = 0usize;

    while i < RATE_TABLE.len() {
        if RATE_TABLE[i].rate >= rate {
            unsafe {
                *value = RATE_TABLE[i].sr;
                *n = RATE_TABLE[i].divisors[clock as usize][0];
                *m = RATE_TABLE[i].divisors[clock as usize][1];
            }
            ret = 0;
            break;
        }
        i += 1;
    }
    ret
}

unsafe fn max98925_set_sense_data(max98925: *mut max98925_priv) {
    /* set VMON slots */
    unsafe {
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DOUT_CFG_VMON,
            M98925_DAI_VMON_EN_MASK,
            M98925_DAI_VMON_EN_MASK,
        );
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DOUT_CFG_VMON,
            M98925_DAI_VMON_SLOT_MASK,
            (*max98925).v_slot << M98925_DAI_VMON_SLOT_SHIFT,
        );
    }
    /* set IMON slots */
    unsafe {
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DOUT_CFG_IMON,
            M98925_DAI_IMON_EN_MASK,
            M98925_DAI_IMON_EN_MASK,
        );
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DOUT_CFG_IMON,
            M98925_DAI_IMON_SLOT_MASK,
            (*max98925).i_slot << M98925_DAI_IMON_SLOT_SHIFT,
        );
    }
}

unsafe extern "C" fn max98925_dai_set_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let max98925 = unsafe { snd_soc_component_get_drvdata(component) };
    let mut invert: c_uint = 0;

    unsafe { dev_dbg((*component).dev, b"%s: fmt 0x%08X\n\0".as_ptr().cast(), b"max98925_dai_set_fmt\0".as_ptr(), fmt) };
    unsafe {
        match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
            SND_SOC_DAIFMT_CBC_CFC => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE2,
                    M98925_DAI_MAS_MASK,
                    0,
                );
                max98925_set_sense_data(max98925);
            }
            SND_SOC_DAIFMT_CBP_CFP => {
                /*
                 * set left channel DAI to provider mode,
                 * right channel always consumer
                 */
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE2,
                    M98925_DAI_MAS_MASK,
                    M98925_DAI_MAS_MASK,
                );
            }
            _ => {
                dev_err((*component).dev, b"DAI clock mode unsupported\0".as_ptr().cast());
                return -EINVAL;
            }
        }

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            SND_SOC_DAIFMT_NB_NF => {}
            SND_SOC_DAIFMT_NB_IF => {
                invert = M98925_DAI_WCI_MASK;
            }
            SND_SOC_DAIFMT_IB_NF => {
                invert = M98925_DAI_BCI_MASK;
            }
            SND_SOC_DAIFMT_IB_IF => {
                invert = M98925_DAI_BCI_MASK | M98925_DAI_WCI_MASK;
            }
            _ => {
                dev_err((*component).dev, b"DAI invert mode unsupported\0".as_ptr().cast());
                return -EINVAL;
            }
        }

        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_FORMAT,
            M98925_DAI_BCI_MASK | M98925_DAI_WCI_MASK,
            invert,
        );
    }
    0
}

unsafe fn max98925_set_clock(
    max98925: *mut max98925_priv,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut dai_sr: c_int = 0;
    let clock: c_int;
    let mdll: c_uint;
    let mut n: c_int = 0;
    let mut m: c_int = 0;
    let component = unsafe { (*max98925).component };
    let rate = unsafe { params_rate(params) };
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio = unsafe { params_channels(params) * (*max98925).ch_size };

    unsafe {
        match blr_clk_ratio {
            32 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE2,
                    M98925_DAI_BSEL_MASK,
                    M98925_DAI_BSEL_32,
                );
            }
            48 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE2,
                    M98925_DAI_BSEL_MASK,
                    M98925_DAI_BSEL_48,
                );
            }
            64 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE2,
                    M98925_DAI_BSEL_MASK,
                    M98925_DAI_BSEL_64,
                );
            }
            _ => return -EINVAL,
        }

        match (*max98925).sysclk {
            6000000 => {
                clock = 0;
                mdll = M98925_MDLL_MULT_MCLKx16;
            }
            11289600 => {
                clock = 1;
                mdll = M98925_MDLL_MULT_MCLKx8;
            }
            12000000 => {
                clock = 0;
                mdll = M98925_MDLL_MULT_MCLKx8;
            }
            12288000 => {
                clock = 2;
                mdll = M98925_MDLL_MULT_MCLKx8;
            }
            _ => {
                dev_info(
                    (*(*max98925).component).dev,
                    b"unsupported sysclk %d\n\0".as_ptr().cast(),
                    (*max98925).sysclk,
                );
                return -EINVAL;
            }
        }
    }

    if unsafe { max98925_rate_value(component, rate, clock, &mut dai_sr, &mut n, &mut m) } != 0 {
        return -EINVAL;
    }

    unsafe {
        /* set DAI_SR to correct LRCLK frequency */
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DAI_CLK_MODE2,
            M98925_DAI_SR_MASK,
            (dai_sr as c_uint) << M98925_DAI_SR_SHIFT,
        );
        /* set DAI m divider */
        regmap_write((*max98925).regmap, MAX98925_DAI_CLK_DIV_M_MSBS, (m >> 8) as c_uint);
        regmap_write((*max98925).regmap, MAX98925_DAI_CLK_DIV_M_LSBS, (m & 0xFF) as c_uint);
        /* set DAI n divider */
        regmap_write((*max98925).regmap, MAX98925_DAI_CLK_DIV_N_MSBS, (n >> 8) as c_uint);
        regmap_write((*max98925).regmap, MAX98925_DAI_CLK_DIV_N_LSBS, (n & 0xFF) as c_uint);
        /* set MDLL */
        regmap_update_bits(
            (*max98925).regmap,
            MAX98925_DAI_CLK_MODE1,
            M98925_MDLL_MULT_MASK,
            mdll << M98925_MDLL_MULT_SHIFT,
        );
    }
    0
}

unsafe extern "C" fn max98925_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let max98925 = unsafe { snd_soc_component_get_drvdata(component) };

    unsafe {
        match params_width(params) {
            16 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_FORMAT,
                    M98925_DAI_CHANSZ_MASK,
                    M98925_DAI_CHANSZ_16,
                );
                (*max98925).ch_size = 16;
            }
            24 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_FORMAT,
                    M98925_DAI_CHANSZ_MASK,
                    M98925_DAI_CHANSZ_24,
                );
                (*max98925).ch_size = 24;
            }
            32 => {
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_FORMAT,
                    M98925_DAI_CHANSZ_MASK,
                    M98925_DAI_CHANSZ_32,
                );
                (*max98925).ch_size = 32;
            }
            _ => {
                pr_err(
                    b"%s: format unsupported %d\0".as_ptr().cast(),
                    b"max98925_dai_hw_params\0".as_ptr(),
                    params_format(params),
                );
                return -EINVAL;
            }
        }
        dev_dbg(
            (*component).dev,
            b"%s: format supported %d\0".as_ptr().cast(),
            b"max98925_dai_hw_params\0".as_ptr(),
            params_format(params),
        );
        max98925_set_clock(max98925, params)
    }
}

unsafe extern "C" fn max98925_dai_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = unsafe { (*dai).component };
    let max98925 = unsafe { snd_soc_component_get_drvdata(component) };

    unsafe {
        match clk_id {
            0 => {
                /* use MCLK for Left channel, right channel always BCLK */
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE1,
                    M98925_DAI_CLK_SOURCE_MASK,
                    0,
                );
            }
            1 => {
                /* configure dai clock source to BCLK instead of MCLK */
                regmap_update_bits(
                    (*max98925).regmap,
                    MAX98925_DAI_CLK_MODE1,
                    M98925_DAI_CLK_SOURCE_MASK,
                    M98925_DAI_CLK_SOURCE_MASK,
                );
            }
            _ => return -EINVAL,
        }
        (*max98925).sysclk = freq;
    }
    0
}

const MAX98925_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static MAX98925_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(max98925_dai_set_sysclk),
    set_fmt: Some(max98925_dai_set_fmt),
    hw_params: Some(max98925_dai_hw_params),
};

static mut MAX98925_DAI: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"max98925-aif1\0".as_ptr().cast(),
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr().cast(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: MAX98925_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"HiFi Capture\0".as_ptr().cast(),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: MAX98925_FORMATS,
    },
    ops: &MAX98925_DAI_OPS,
}];

unsafe extern "C" fn max98925_probe(component: *mut snd_soc_component) -> c_int {
    let max98925 = unsafe { snd_soc_component_get_drvdata(component) };

    unsafe {
        (*max98925).component = component;
        regmap_write((*max98925).regmap, MAX98925_GLOBAL_ENABLE, 0x00);
        /* It's not the default but we need to set DAI_DLY */
        regmap_write((*max98925).regmap, MAX98925_FORMAT, M98925_DAI_DLY_MASK);
        regmap_write((*max98925).regmap, MAX98925_TDM_SLOT_SELECT, 0xC8);
        regmap_write((*max98925).regmap, MAX98925_DOUT_HIZ_CFG1, 0xFF);
        regmap_write((*max98925).regmap, MAX98925_DOUT_HIZ_CFG2, 0xFF);
        regmap_write((*max98925).regmap, MAX98925_DOUT_HIZ_CFG3, 0xFF);
        regmap_write((*max98925).regmap, MAX98925_DOUT_HIZ_CFG4, 0xF0);
        regmap_write((*max98925).regmap, MAX98925_FILTERS, 0xD8);
        regmap_write((*max98925).regmap, MAX98925_ALC_CONFIGURATION, 0xF8);
        regmap_write((*max98925).regmap, MAX98925_CONFIGURATION, 0xF0);
        /* Disable ALC muting */
        regmap_write((*max98925).regmap, MAX98925_BOOST_LIMITER, 0xF8);
    }
    0
}

static SOC_COMPONENT_DEV_MAX98925: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98925_probe),
    controls: MAX98925_SND_CONTROLS.as_ptr(),
    num_controls: MAX98925_SND_CONTROLS.len() as c_uint,
    dapm_routes: MAX98925_AUDIO_MAP.as_ptr(),
    num_dapm_routes: MAX98925_AUDIO_MAP.len() as c_uint,
    dapm_widgets: MAX98925_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: MAX98925_DAPM_WIDGETS.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static MAX98925_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: unsafe { MAX98925_REV_VERSION },
    reg_defaults: MAX98925_REG.as_ptr(),
    num_reg_defaults: MAX98925_REG.len() as c_uint,
    volatile_reg: Some(max98925_volatile_register),
    readable_reg: Some(max98925_readable_register),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn max98925_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut reg: c_int = 0;
    let mut value: u32 = 0;
    let max98925: *mut max98925_priv;

    unsafe {
        max98925 = devm_kzalloc(
            &mut (*i2c).dev,
            size_of::<max98925_priv>(),
            GFP_KERNEL,
        ) as *mut max98925_priv;
        if max98925.is_null() {
            return -ENOMEM;
        }

        i2c_set_clientdata(i2c, max98925.cast());
        (*max98925).regmap = devm_regmap_init_i2c(i2c, &MAX98925_REGMAP);
        if IS_ERR((*max98925).regmap.cast()) {
            ret = PTR_ERR((*max98925).regmap.cast());
            dev_err(
                &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                b"Failed to allocate regmap: %d\n\0".as_ptr().cast(),
                ret,
            );
            return ret;
        }

        if of_property_read_u32(
            (*i2c).dev.of_node,
            b"vmon-slot-no\0".as_ptr().cast(),
            &mut value,
        ) == 0
        {
            if value > M98925_DAI_VMON_SLOT_1E_1F {
                dev_err(
                    &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                    b"vmon slot number is wrong:\n\0".as_ptr().cast(),
                );
                return -EINVAL;
            }
            (*max98925).v_slot = value;
        }
        if of_property_read_u32(
            (*i2c).dev.of_node,
            b"imon-slot-no\0".as_ptr().cast(),
            &mut value,
        ) == 0
        {
            if value > M98925_DAI_IMON_SLOT_1E_1F {
                dev_err(
                    &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                    b"imon slot number is wrong:\n\0".as_ptr().cast(),
                );
                return -EINVAL;
            }
            (*max98925).i_slot = value;
        }

        ret = regmap_read((*max98925).regmap, MAX98925_REV_VERSION, &mut reg);
        if ret < 0 {
            dev_err(
                &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                b"Read revision failed\n\0".as_ptr().cast(),
            );
            return ret;
        }

        if reg != MAX98925_VERSION && reg != MAX98925_VERSION1 {
            ret = -ENODEV;
            dev_err(
                &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                b"Invalid revision (%d 0x%02X)\n\0".as_ptr().cast(),
                ret,
                reg,
            );
            return ret;
        }

        dev_info(
            &mut (*i2c).dev as *mut device_with_of_node as *mut device,
            b"device version 0x%02X\n\0".as_ptr().cast(),
            reg,
        );

        ret = devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &SOC_COMPONENT_DEV_MAX98925,
            MAX98925_DAI.as_mut_ptr(),
            MAX98925_DAI.len() as c_int,
        );
        if ret < 0 {
            dev_err(
                &mut (*i2c).dev as *mut device_with_of_node as *mut device,
                b"Failed to register component: %d\n\0".as_ptr().cast(),
                ret,
            );
        }
        ret
    }
}

static MAX98925_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id { name: b"max98925\0".as_ptr().cast() },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, max98925_i2c_id);

// #ifdef CONFIG_OF
static MAX98925_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"maxim,max98925\0".as_ptr().cast() },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, max98925_of_match);
// #endif

static MAX98925_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: b"max98925\0".as_ptr().cast(),
        of_match_table: unsafe { of_match_ptr(MAX98925_OF_MATCH.as_ptr()) },
    },
    probe: Some(max98925_i2c_probe),
    id_table: MAX98925_I2C_ID.as_ptr(),
};

// module_i2c_driver(max98925_i2c_driver)

// MODULE_DESCRIPTION("ALSA SoC MAX98925 driver");
// MODULE_AUTHOR("Ralph Birt <rdbirt@gmail.com>, Anish kumar <anish.kumar@maximintegrated.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
