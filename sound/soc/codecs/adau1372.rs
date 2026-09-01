// SPDX-License-Identifier: GPL-2.0-only
/*
 * Analog Devices ADAU1372 Audio Codec driver
 *
 * Copyright 2016 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
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
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct adau1372 {
    pub regmap: *mut regmap,
    pub switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
    pub use_pll: bool,
    pub enabled: bool,
    pub clock_provider: bool,
    pub rate_constraints: snd_pcm_hw_constraint_list,
    pub slot_width: c_uint,
    pub mclk: *mut clk,
    pub pd_gpio: *mut gpio_desc,
    pub dev: *mut device,
}

const fn BIT(n: c_uint) -> c_uint {
    1u32 << n
}

const ADAU1372_REG_CLK_CTRL: c_uint = 0x00;
const fn ADAU1372_REG_PLL(x: c_uint) -> c_uint {
    0x01 + x
}
const ADAU1372_REG_DAC_SOURCE: c_uint = 0x11;
const ADAU1372_REG_SOUT_SOURCE_0_1: c_uint = 0x13;
const ADAU1372_REG_SOUT_SOURCE_2_3: c_uint = 0x14;
const ADAU1372_REG_SOUT_SOURCE_4_5: c_uint = 0x15;
const ADAU1372_REG_SOUT_SOURCE_6_7: c_uint = 0x16;
const ADAU1372_REG_ADC_SDATA_CH: c_uint = 0x17;
const ADAU1372_REG_ASRCO_SOURCE_0_1: c_uint = 0x18;
const ADAU1372_REG_ASRCO_SOURCE_2_3: c_uint = 0x19;
const ADAU1372_REG_ASRC_MODE: c_uint = 0x1a;
const ADAU1372_REG_ADC_CTRL0: c_uint = 0x1b;
const ADAU1372_REG_ADC_CTRL1: c_uint = 0x1c;
const ADAU1372_REG_ADC_CTRL2: c_uint = 0x1d;
const ADAU1372_REG_ADC_CTRL3: c_uint = 0x1e;
const fn ADAU1372_REG_ADC_VOL(x: c_uint) -> c_uint {
    0x1f + x
}
const fn ADAU1372_REG_PGA_CTRL(x: c_uint) -> c_uint {
    0x23 + x
}
const ADAU1372_REG_PGA_BOOST: c_uint = 0x28;
const ADAU1372_REG_MICBIAS: c_uint = 0x2d;
const ADAU1372_REG_DAC_CTRL: c_uint = 0x2e;
const fn ADAU1372_REG_DAC_VOL(x: c_uint) -> c_uint {
    0x2f + x
}
const ADAU1372_REG_OP_STAGE_MUTE: c_uint = 0x31;
const ADAU1372_REG_SAI0: c_uint = 0x32;
const ADAU1372_REG_SAI1: c_uint = 0x33;
const ADAU1372_REG_SOUT_CTRL: c_uint = 0x34;
const fn ADAU1372_REG_MODE_MP(x: c_uint) -> c_uint {
    0x38 + x
}
const ADAU1372_REG_OP_STAGE_CTRL: c_uint = 0x43;
const ADAU1372_REG_DECIM_PWR: c_uint = 0x44;
const ADAU1372_REG_INTERP_PWR: c_uint = 0x45;
const ADAU1372_REG_BIAS_CTRL0: c_uint = 0x46;
const ADAU1372_REG_BIAS_CTRL1: c_uint = 0x47;

const ADAU1372_CLK_CTRL_PLL_EN: c_uint = BIT(7);
const ADAU1372_CLK_CTRL_XTAL_DIS: c_uint = BIT(4);
const ADAU1372_CLK_CTRL_CLKSRC: c_uint = BIT(3);
const ADAU1372_CLK_CTRL_CC_MDIV: c_uint = BIT(1);
const ADAU1372_CLK_CTRL_MCLK_EN: c_uint = BIT(0);

const ADAU1372_SAI0_DELAY1: c_uint = 0x0 << 6;
const ADAU1372_SAI0_DELAY0: c_uint = 0x1 << 6;
const ADAU1372_SAI0_DELAY_MASK: c_uint = 0x3 << 6;
const ADAU1372_SAI0_SAI_I2S: c_uint = 0x0 << 4;
const ADAU1372_SAI0_SAI_TDM2: c_uint = 0x1 << 4;
const ADAU1372_SAI0_SAI_TDM4: c_uint = 0x2 << 4;
const ADAU1372_SAI0_SAI_TDM8: c_uint = 0x3 << 4;
const ADAU1372_SAI0_SAI_MASK: c_uint = 0x3 << 4;
const ADAU1372_SAI0_FS_48: usize = 0x0;
const ADAU1372_SAI0_FS_8: usize = 0x1;
const ADAU1372_SAI0_FS_12: usize = 0x2;
const ADAU1372_SAI0_FS_16: usize = 0x3;
const ADAU1372_SAI0_FS_24: usize = 0x4;
const ADAU1372_SAI0_FS_32: usize = 0x5;
const ADAU1372_SAI0_FS_96: usize = 0x6;
const ADAU1372_SAI0_FS_192: usize = 0x7;
const ADAU1372_SAI0_FS_MASK: c_uint = 0xf;

const ADAU1372_SAI1_TDM_TS: c_uint = BIT(7);
const ADAU1372_SAI1_BCLK_TDMC: c_uint = BIT(6);
const ADAU1372_SAI1_LR_MODE: c_uint = BIT(5);
const ADAU1372_SAI1_LR_POL: c_uint = BIT(4);
const ADAU1372_SAI1_BCLKRATE: c_uint = BIT(2);
const ADAU1372_SAI1_BCLKEDGE: c_uint = BIT(1);
const ADAU1372_SAI1_MS: c_uint = BIT(0);

static adau1372_rates: [c_uint; 8] = {
    let mut rates = [0; 8];
    rates[ADAU1372_SAI0_FS_8] = 8000;
    rates[ADAU1372_SAI0_FS_12] = 12000;
    rates[ADAU1372_SAI0_FS_16] = 16000;
    rates[ADAU1372_SAI0_FS_24] = 24000;
    rates[ADAU1372_SAI0_FS_32] = 32000;
    rates[ADAU1372_SAI0_FS_48] = 48000;
    rates[ADAU1372_SAI0_FS_96] = 96000;
    rates[ADAU1372_SAI0_FS_192] = 192000;
    rates
};

/* 8k, 12k, 24k, 48k */
const ADAU1372_RATE_MASK_TDM8: c_uint = 0x17;
/* + 16k, 96k */
const ADAU1372_RATE_MASK_TDM4_MASTER: c_uint = ADAU1372_RATE_MASK_TDM8 | 0x48 | 0x20;
/* +32k */
const ADAU1372_RATE_MASK_TDM4: c_uint = ADAU1372_RATE_MASK_TDM4_MASTER | 0x20;
/* + 192k */
const ADAU1372_RATE_MASK_TDM2: c_uint = ADAU1372_RATE_MASK_TDM4 | 0x80;

/* TLV, SOC_ENUM, SOC_VALUE_ENUM, SOC_SINGLE, SOC_DAPM, and DAI descriptor
 * initializers depend on ALSA SoC C macros from external headers. The following
 * comments preserve the source-level declarations and ordering for those macro
 * generated static objects.
 *
 * static const DECLARE_TLV_DB_MINMAX(adau1372_digital_tlv, -9563, 0);
 * static const DECLARE_TLV_DB_SCALE(adau1372_pga_tlv, -1200, 75, 0);
 * static const DECLARE_TLV_DB_SCALE(adau1372_pga_boost_tlv, 0, 1000, 0);
 */

static adau1372_bias_text: [&[u8]; 4] = [
    b"Normal operation\0",
    b"Extreme power saving\0",
    b"Enhanced performance\0",
    b"Power saving\0",
];

static adau1372_bias_adc_values: [c_uint; 3] = [0, 2, 3];

static adau1372_bias_adc_text: [&[u8]; 3] = [
    b"Normal operation\0",
    b"Enhanced performance\0",
    b"Power saving\0",
];

static adau1372_bias_dac_text: [&[u8]; 4] = [
    b"Normal operation\0",
    b"Power saving\0",
    b"Superior performance\0",
    b"Enhanced performance\0",
];

static adau1372_hpf_text: [&[u8]; 4] = [b"Off\0", b"1 Hz\0", b"4 Hz\0", b"8 Hz\0"];

static adau1372_decimator_mux_text: [&[u8]; 2] = [b"ADC\0", b"DMIC\0"];
static adau1372_asrco_mux_values: [c_uint; 4] = [4, 5, 6, 7];
static adau1372_asrco_mux_text: [&[u8]; 4] = [
    b"Decimator0\0",
    b"Decimator1\0",
    b"Decimator2\0",
    b"Decimator3\0",
];
static adau1372_sout_mux_values: [c_uint; 12] = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
static adau1372_sout_mux_text: [&[u8]; 12] = [
    b"Output ASRC0\0",
    b"Output ASRC1\0",
    b"Output ASRC2\0",
    b"Output ASRC3\0",
    b"Serial Input 0\0",
    b"Serial Input 1\0",
    b"Serial Input 2\0",
    b"Serial Input 3\0",
    b"Serial Input 4\0",
    b"Serial Input 5\0",
    b"Serial Input 6\0",
    b"Serial Input 7\0",
];
static adau1372_asrci_mux_text: [&[u8]; 4] = [
    b"Serial Input 0+1\0",
    b"Serial Input 2+3\0",
    b"Serial Input 4+5\0",
    b"Serial Input 6+7\0",
];
static adau1372_dac_mux_values: [c_uint; 2] = [12, 13];
static adau1372_dac_mux_text: [&[u8]; 2] = [b"Input ASRC0\0", b"Input ASRC1\0"];

/* static const struct snd_kcontrol_new adau1372_controls[] = { ... };
 * static const struct snd_soc_dapm_widget adau1372_dapm_widgets[] = { ... };
 */

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: core::ptr::null(),
            source: concat!($source, "\0").as_ptr() as *const c_char,
        }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: concat!($control, "\0").as_ptr() as *const c_char,
            source: concat!($source, "\0").as_ptr() as *const c_char,
        }
    };
}

macro_rules! ADAU1372_ASRCO_ROUTES {
    ($x:literal) => {
        route!(concat!("Output ASRC", $x, " Mux"), "Decimator0", "Decimator0 Mux"),
        route!(concat!("Output ASRC", $x, " Mux"), "Decimator1", "Decimator1 Mux"),
        route!(concat!("Output ASRC", $x, " Mux"), "Decimator2", "Decimator2 Mux"),
        route!(concat!("Output ASRC", $x, " Mux"), "Decimator3", "Decimator3 Mux")
    };
}

macro_rules! ADAU1372_SOUT_ROUTES {
    ($x:literal) => {
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Output ASRC0", "Output ASRC0 Mux"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Output ASRC1", "Output ASRC1 Mux"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Output ASRC2", "Output ASRC2 Mux"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Output ASRC3", "Output ASRC3 Mux"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 0", "Serial Input 0"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 1", "Serial Input 1"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 2", "Serial Input 2"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 3", "Serial Input 3"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 4", "Serial Input 4"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 5", "Serial Input 5"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 6", "Serial Input 6"),
        route!(concat!("Serial Output ", $x, " Capture Mux"), "Serial Input 7", "Serial Input 7"),
        route!(concat!("Serial Output ", $x), NULL, concat!("Serial Output ", $x, " Capture Mux")),
        route!("Capture", NULL, concat!("Serial Output ", $x))
    };
}

static adau1372_dapm_routes: [snd_soc_dapm_route; 171] = [
    route!("PGA0", NULL, "AIN0"),
    route!("PGA1", NULL, "AIN1"),
    route!("PGA2", NULL, "AIN2"),
    route!("PGA3", NULL, "AIN3"),
    route!("ADC0", NULL, "PGA0"),
    route!("ADC1", NULL, "PGA1"),
    route!("ADC2", NULL, "PGA2"),
    route!("ADC3", NULL, "PGA3"),
    route!("Decimator0 Mux", "ADC", "ADC0"),
    route!("Decimator1 Mux", "ADC", "ADC1"),
    route!("Decimator2 Mux", "ADC", "ADC2"),
    route!("Decimator3 Mux", "ADC", "ADC3"),
    route!("Decimator0 Mux", "DMIC", "DMIC0_1"),
    route!("Decimator1 Mux", "DMIC", "DMIC0_1"),
    route!("Decimator2 Mux", "DMIC", "DMIC2_3"),
    route!("Decimator3 Mux", "DMIC", "DMIC2_3"),
    route!("Decimator0 Mux", NULL, "ADC0 Filter"),
    route!("Decimator1 Mux", NULL, "ADC1 Filter"),
    route!("Decimator2 Mux", NULL, "ADC2 Filter"),
    route!("Decimator3 Mux", NULL, "ADC3 Filter"),
    route!("Output ASRC0 Mux", NULL, "Output ASRC Supply"),
    route!("Output ASRC1 Mux", NULL, "Output ASRC Supply"),
    route!("Output ASRC2 Mux", NULL, "Output ASRC Supply"),
    route!("Output ASRC3 Mux", NULL, "Output ASRC Supply"),
    route!("Output ASRC0 Mux", NULL, "Output ASRC0 Decimator"),
    route!("Output ASRC1 Mux", NULL, "Output ASRC1 Decimator"),
    route!("Output ASRC2 Mux", NULL, "Output ASRC2 Decimator"),
    route!("Output ASRC3 Mux", NULL, "Output ASRC3 Decimator"),
    ADAU1372_ASRCO_ROUTES!(0),
    ADAU1372_ASRCO_ROUTES!(1),
    ADAU1372_ASRCO_ROUTES!(2),
    ADAU1372_ASRCO_ROUTES!(3),
    ADAU1372_SOUT_ROUTES!(0),
    ADAU1372_SOUT_ROUTES!(1),
    ADAU1372_SOUT_ROUTES!(2),
    ADAU1372_SOUT_ROUTES!(3),
    ADAU1372_SOUT_ROUTES!(4),
    ADAU1372_SOUT_ROUTES!(5),
    ADAU1372_SOUT_ROUTES!(6),
    ADAU1372_SOUT_ROUTES!(7),
    route!("Serial Input 0", NULL, "Playback"),
    route!("Serial Input 1", NULL, "Playback"),
    route!("Serial Input 2", NULL, "Playback"),
    route!("Serial Input 3", NULL, "Playback"),
    route!("Serial Input 4", NULL, "Playback"),
    route!("Serial Input 5", NULL, "Playback"),
    route!("Serial Input 6", NULL, "Playback"),
    route!("Serial Input 7", NULL, "Playback"),
    route!("Input ASRC0 Mux", "Serial Input 0+1", "Serial Input 0"),
    route!("Input ASRC1 Mux", "Serial Input 0+1", "Serial Input 1"),
    route!("Input ASRC0 Mux", "Serial Input 2+3", "Serial Input 2"),
    route!("Input ASRC1 Mux", "Serial Input 2+3", "Serial Input 3"),
    route!("Input ASRC0 Mux", "Serial Input 4+5", "Serial Input 4"),
    route!("Input ASRC1 Mux", "Serial Input 4+5", "Serial Input 5"),
    route!("Input ASRC0 Mux", "Serial Input 6+7", "Serial Input 6"),
    route!("Input ASRC1 Mux", "Serial Input 6+7", "Serial Input 7"),
    route!("Input ASRC0 Mux", NULL, "Input ASRC Supply"),
    route!("Input ASRC1 Mux", NULL, "Input ASRC Supply"),
    route!("Input ASRC0 Mux", NULL, "Input ASRC0 Interpolator"),
    route!("Input ASRC1 Mux", NULL, "Input ASRC1 Interpolator"),
    route!("DAC 0 Mux", "Input ASRC0", "Input ASRC0 Mux"),
    route!("DAC 0 Mux", "Input ASRC1", "Input ASRC1 Mux"),
    route!("DAC 1 Mux", "Input ASRC0", "Input ASRC0 Mux"),
    route!("DAC 1 Mux", "Input ASRC1", "Input ASRC1 Mux"),
    route!("DAC0", NULL, "DAC 0 Mux"),
    route!("DAC1", NULL, "DAC 1 Mux"),
    route!("DAC0", NULL, "DAC0 Modulator"),
    route!("DAC1", NULL, "DAC1 Modulator"),
    route!("OP_STAGE_LP", NULL, "DAC0"),
    route!("OP_STAGE_LN", NULL, "DAC0"),
    route!("OP_STAGE_RP", NULL, "DAC1"),
    route!("OP_STAGE_RN", NULL, "DAC1"),
    route!("HPOUTL", NULL, "OP_STAGE_LP"),
    route!("HPOUTL", NULL, "OP_STAGE_LN"),
    route!("HPOUTR", NULL, "OP_STAGE_RP"),
    route!("HPOUTR", NULL, "OP_STAGE_RN"),
];

extern "C" {
    static mut adau1372_driver: snd_soc_component_driver;
    static mut adau1372_dai_driver: snd_soc_dai_driver;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_uint, l: *mut snd_pcm_hw_constraint_list) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn adau_calc_pll_cfg(freq_in: c_ulong, freq_out: c_ulong, regs: *mut u8) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_HIGH: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 0;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

unsafe extern "C" fn adau1372_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let adau1372 = snd_soc_dai_get_drvdata(dai) as *mut adau1372;
    let mut sai0: c_uint = 0;
    let mut sai1: c_uint = 0;
    let mut invert_lrclk = false;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            (*adau1372).clock_provider = true;
            sai1 |= ADAU1372_SAI1_MS;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            (*adau1372).clock_provider = false;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => invert_lrclk = false,
        SND_SOC_DAIFMT_NB_IF => invert_lrclk = true,
        SND_SOC_DAIFMT_IB_NF => {
            invert_lrclk = false;
            sai1 |= ADAU1372_SAI1_BCLKEDGE;
        }
        SND_SOC_DAIFMT_IB_IF => {
            invert_lrclk = true;
            sai1 |= ADAU1372_SAI1_BCLKEDGE;
        }
        _ => {}
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => sai0 |= ADAU1372_SAI0_DELAY1,
        SND_SOC_DAIFMT_LEFT_J => {
            sai0 |= ADAU1372_SAI0_DELAY0;
            invert_lrclk = !invert_lrclk;
        }
        SND_SOC_DAIFMT_DSP_A => {
            sai0 |= ADAU1372_SAI0_DELAY1;
            sai1 |= ADAU1372_SAI1_LR_MODE;
        }
        SND_SOC_DAIFMT_DSP_B => {
            sai0 |= ADAU1372_SAI0_DELAY0;
            sai1 |= ADAU1372_SAI1_LR_MODE;
        }
        _ => {}
    }

    if invert_lrclk {
        sai1 |= ADAU1372_SAI1_LR_POL;
    }

    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI0, ADAU1372_SAI0_DELAY_MASK, sai0);
    regmap_update_bits(
        (*adau1372).regmap,
        ADAU1372_REG_SAI1,
        ADAU1372_SAI1_MS | ADAU1372_SAI1_BCLKEDGE | ADAU1372_SAI1_LR_MODE | ADAU1372_SAI1_LR_POL,
        sai1,
    );

    0
}

unsafe extern "C" fn adau1372_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let adau1372 = snd_soc_dai_get_drvdata(dai) as *mut adau1372;
    let rate = params_rate(params);
    let mut slot_width: c_uint;
    let mut sai1: c_uint;
    let mut i: usize = 0;

    while i < adau1372_rates.len() {
        if rate == adau1372_rates[i] {
            break;
        }
        i += 1;
    }

    if i == adau1372_rates.len() {
        return -EINVAL;
    }

    let sai0 = i as c_uint;

    slot_width = (*adau1372).slot_width;
    if slot_width == 0 {
        slot_width = params_width(params);
    }

    match slot_width {
        16 => sai1 = ADAU1372_SAI1_BCLKRATE,
        24 | 32 => sai1 = 0,
        _ => return -EINVAL,
    }

    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI0, ADAU1372_SAI0_FS_MASK, sai0);
    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI1, ADAU1372_SAI1_BCLKRATE, sai1);

    0
}

unsafe extern "C" fn adau1372_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    width: c_int,
) -> c_int {
    let adau1372 = snd_soc_dai_get_drvdata(dai) as *mut adau1372;
    let sai0: c_uint;
    let sai1: c_uint;

    /* I2S mode */
    if slots == 0 {
        /* The other settings dont matter in I2S mode */
        regmap_update_bits(
            (*adau1372).regmap,
            ADAU1372_REG_SAI0,
            ADAU1372_SAI0_SAI_MASK,
            ADAU1372_SAI0_SAI_I2S,
        );
        (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM2;
        (*adau1372).slot_width = 0;
        return 0;
    }

    /* We have 8 channels anything outside that is not supported */
    if (tx_mask & !0xff) != 0 || (rx_mask & !0xff) != 0 {
        return -EINVAL;
    }

    match width {
        16 => sai1 = ADAU1372_SAI1_BCLK_TDMC,
        24 | 32 => sai1 = 0,
        _ => return -EINVAL,
    }

    match slots {
        2 => {
            sai0 = ADAU1372_SAI0_SAI_TDM2;
            (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM2;
        }
        4 => {
            sai0 = ADAU1372_SAI0_SAI_TDM4;
            if (*adau1372).clock_provider {
                (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM4_MASTER;
            } else {
                (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM4;
            }
        }
        8 => {
            sai0 = ADAU1372_SAI0_SAI_TDM8;
            (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM8;
        }
        _ => return -EINVAL,
    }

    (*adau1372).slot_width = width as c_uint;

    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI0, ADAU1372_SAI0_SAI_MASK, sai0);
    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI1, ADAU1372_SAI1_BCLK_TDMC, sai1);

    /* Mask is inverted in hardware */
    regmap_write((*adau1372).regmap, ADAU1372_REG_SOUT_CTRL, !tx_mask);

    0
}

unsafe extern "C" fn adau1372_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let adau1372 = snd_soc_dai_get_drvdata(dai) as *mut adau1372;
    let sai1 = if tristate != 0 { ADAU1372_SAI1_TDM_TS } else { 0 };

    regmap_update_bits((*adau1372).regmap, ADAU1372_REG_SAI1, ADAU1372_SAI1_TDM_TS, sai1)
}

unsafe extern "C" fn adau1372_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let adau1372 = snd_soc_dai_get_drvdata(dai) as *mut adau1372;

    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*adau1372).rate_constraints,
    );

    0
}

unsafe fn adau1372_enable_pll(adau1372: *mut adau1372) -> c_int {
    let mut val: c_uint = 0;
    let mut timeout: c_uint = 0;
    let mut ret: c_int;

    regmap_update_bits(
        (*adau1372).regmap,
        ADAU1372_REG_CLK_CTRL,
        ADAU1372_CLK_CTRL_PLL_EN,
        ADAU1372_CLK_CTRL_PLL_EN,
    );
    loop {
        /* Takes about 1ms to lock */
        usleep_range(1000, 2000);
        ret = regmap_read((*adau1372).regmap, ADAU1372_REG_PLL(5), &mut val);
        if ret != 0 {
            break;
        }
        timeout += 1;
        if (val & 1) != 0 || timeout >= 3 {
            break;
        }
    }

    if ret < 0 || (val & 1) == 0 {
        dev_err((*adau1372).dev, b"Failed to lock PLL\n\0".as_ptr() as *const c_char);
        return if ret < 0 { ret } else { -ETIMEDOUT };
    }

    0
}

unsafe fn adau1372_set_power(adau1372: *mut adau1372, enable: bool) -> c_int {
    if (*adau1372).enabled == enable {
        return 0;
    }

    if enable {
        let mut clk_ctrl = ADAU1372_CLK_CTRL_MCLK_EN;
        let mut ret: c_int;

        ret = clk_prepare_enable((*adau1372).mclk);
        if ret != 0 {
            return ret;
        }
        if !(*adau1372).pd_gpio.is_null() {
            gpiod_set_value((*adau1372).pd_gpio, 0);
        }

        if let Some(switch_mode) = (*adau1372).switch_mode {
            switch_mode((*adau1372).dev);
        }

        regcache_cache_only((*adau1372).regmap, false);

        /*
         * Clocks needs to be enabled before any other register can be
         * accessed.
         */
        if (*adau1372).use_pll {
            ret = adau1372_enable_pll(adau1372);
            if ret != 0 {
                if (*adau1372).pd_gpio.is_null() {
                    regmap_update_bits(
                        (*adau1372).regmap,
                        ADAU1372_REG_CLK_CTRL,
                        ADAU1372_CLK_CTRL_PLL_EN,
                        0,
                    );
                }
                regcache_cache_only((*adau1372).regmap, true);
                if !(*adau1372).pd_gpio.is_null() {
                    gpiod_set_value((*adau1372).pd_gpio, 1);
                }
                clk_disable_unprepare((*adau1372).mclk);
                return ret;
            }
            clk_ctrl |= ADAU1372_CLK_CTRL_CLKSRC;
        }

        regmap_update_bits(
            (*adau1372).regmap,
            ADAU1372_REG_CLK_CTRL,
            ADAU1372_CLK_CTRL_MCLK_EN | ADAU1372_CLK_CTRL_CLKSRC,
            clk_ctrl,
        );
        regcache_sync((*adau1372).regmap);
    } else {
        if !(*adau1372).pd_gpio.is_null() {
            /*
             * This will turn everything off and reset the register
             * map. No need to do any register writes to manually
             * turn things off.
             */
            gpiod_set_value((*adau1372).pd_gpio, 1);
            regcache_mark_dirty((*adau1372).regmap);
        } else {
            regmap_update_bits(
                (*adau1372).regmap,
                ADAU1372_REG_CLK_CTRL,
                ADAU1372_CLK_CTRL_MCLK_EN | ADAU1372_CLK_CTRL_PLL_EN,
                0,
            );
        }
        clk_disable_unprepare((*adau1372).mclk);
        regcache_cache_only((*adau1372).regmap, true);
    }

    (*adau1372).enabled = enable;

    0
}

unsafe extern "C" fn adau1372_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adau1372 = snd_soc_component_get_drvdata(component) as *mut adau1372;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => return adau1372_set_power(adau1372, true),
        snd_soc_bias_level::SND_SOC_BIAS_OFF => return adau1372_set_power(adau1372, false),
    }

    0
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub set_tristate: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
    pub sig_bits: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const ADAU1372_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static adau1372_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(adau1372_set_dai_fmt),
    set_tdm_slot: Some(adau1372_set_tdm_slot),
    set_tristate: Some(adau1372_set_tristate),
    hw_params: Some(adau1372_hw_params),
    startup: Some(adau1372_startup),
};

static mut adau1372_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"adau1372\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: ADAU1372_FORMATS,
        sig_bits: 24,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: ADAU1372_FORMATS,
        sig_bits: 24,
    },
    ops: &adau1372_dai_ops,
    symmetric_rate: 1,
};

static adau1372_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(adau1372_set_bias_level),
    controls: core::ptr::null(),
    num_controls: 0,
    dapm_widgets: core::ptr::null(),
    num_dapm_widgets: 0,
    dapm_routes: adau1372_dapm_routes.as_ptr(),
    num_dapm_routes: adau1372_dapm_routes.len(),
    endianness: 1,
};

unsafe fn adau1372_setup_pll(adau1372: *mut adau1372, rate: c_uint) -> c_int {
    let mut regs: [u8; 5] = [0; 5];
    let mut i: usize;
    let ret: c_int;

    ret = adau_calc_pll_cfg(rate as c_ulong, 49152000, regs.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    i = 0;
    while i < regs.len() {
        regmap_write((*adau1372).regmap, ADAU1372_REG_PLL(i as c_uint), regs[i] as c_uint);
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn adau1372_probe(
    dev: *mut device,
    regmap: *mut regmap,
    switch_mode: Option<unsafe extern "C" fn(dev: *mut device)>,
) -> c_int {
    let adau1372: *mut adau1372;
    let mut clk_ctrl: c_uint;
    let rate: c_ulong;
    let mut ret: c_int;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    adau1372 = devm_kzalloc(dev, core::mem::size_of::<adau1372>(), GFP_KERNEL) as *mut adau1372;
    if adau1372.is_null() {
        return -ENOMEM;
    }

    (*adau1372).mclk = devm_clk_get(dev, b"mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*adau1372).mclk as *const c_void) {
        return PTR_ERR((*adau1372).mclk as *const c_void);
    }

    (*adau1372).pd_gpio = devm_gpiod_get_optional(
        dev,
        b"powerdown\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*adau1372).pd_gpio as *const c_void) {
        return PTR_ERR((*adau1372).pd_gpio as *const c_void);
    }

    (*adau1372).regmap = regmap;
    (*adau1372).switch_mode = switch_mode;
    (*adau1372).dev = dev;
    (*adau1372).rate_constraints.list = adau1372_rates.as_ptr();
    (*adau1372).rate_constraints.count = adau1372_rates.len() as c_uint;
    (*adau1372).rate_constraints.mask = ADAU1372_RATE_MASK_TDM2;

    dev_set_drvdata(dev, adau1372 as *mut c_void);

    /*
     * The datasheet says that the internal MCLK always needs to run at
     * 12.288MHz. Automatically choose a valid configuration from the
     * external clock.
     */
    rate = clk_get_rate((*adau1372).mclk);

    match rate {
        12288000 => clk_ctrl = ADAU1372_CLK_CTRL_CC_MDIV,
        24576000 => clk_ctrl = 0,
        _ => {
            clk_ctrl = 0;
            ret = adau1372_setup_pll(adau1372, rate as c_uint);
            if ret < 0 {
                return ret;
            }
            (*adau1372).use_pll = true;
        }
    }

    /*
     * Most of the registers are inaccessible unless the internal clock is
     * enabled.
     */
    regcache_cache_only(regmap, true);

    regmap_update_bits(regmap, ADAU1372_REG_CLK_CTRL, ADAU1372_CLK_CTRL_CC_MDIV, clk_ctrl);

    /*
     * No pinctrl support yet, put the multi-purpose pins in the most
     * sensible mode for general purpose CODEC operation.
     */
    regmap_write(regmap, ADAU1372_REG_MODE_MP(1), 0x00); /* SDATA OUT */
    regmap_write(regmap, ADAU1372_REG_MODE_MP(6), 0x12); /* CLOCKOUT */

    regmap_write(regmap, ADAU1372_REG_OP_STAGE_MUTE, 0x0);

    regmap_write(regmap, 0x7, 0x01); /* CLOCK OUT */

    devm_snd_soc_register_component(dev, &adau1372_driver, &mut adau1372_dai_driver, 1)
}
/* EXPORT_SYMBOL(adau1372_probe); */

static adau1372_reg_defaults: [reg_default; 49] = [
    reg_default { reg: ADAU1372_REG_CLK_CTRL, def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(0), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(1), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(2), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(3), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(4), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PLL(5), def: 0x00 },
    reg_default { reg: ADAU1372_REG_DAC_SOURCE, def: 0x10 },
    reg_default { reg: ADAU1372_REG_SOUT_SOURCE_0_1, def: 0x54 },
    reg_default { reg: ADAU1372_REG_SOUT_SOURCE_2_3, def: 0x76 },
    reg_default { reg: ADAU1372_REG_SOUT_SOURCE_4_5, def: 0x54 },
    reg_default { reg: ADAU1372_REG_SOUT_SOURCE_6_7, def: 0x76 },
    reg_default { reg: ADAU1372_REG_ADC_SDATA_CH, def: 0x04 },
    reg_default { reg: ADAU1372_REG_ASRCO_SOURCE_0_1, def: 0x10 },
    reg_default { reg: ADAU1372_REG_ASRCO_SOURCE_2_3, def: 0x32 },
    reg_default { reg: ADAU1372_REG_ASRC_MODE, def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_CTRL0, def: 0x19 },
    reg_default { reg: ADAU1372_REG_ADC_CTRL1, def: 0x19 },
    reg_default { reg: ADAU1372_REG_ADC_CTRL2, def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_CTRL3, def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_VOL(0), def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_VOL(1), def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_VOL(2), def: 0x00 },
    reg_default { reg: ADAU1372_REG_ADC_VOL(3), def: 0x00 },
    reg_default { reg: ADAU1372_REG_PGA_CTRL(0), def: 0x40 },
    reg_default { reg: ADAU1372_REG_PGA_CTRL(1), def: 0x40 },
    reg_default { reg: ADAU1372_REG_PGA_CTRL(2), def: 0x40 },
    reg_default { reg: ADAU1372_REG_PGA_CTRL(3), def: 0x40 },
    reg_default { reg: ADAU1372_REG_PGA_BOOST, def: 0x00 },
    reg_default { reg: ADAU1372_REG_MICBIAS, def: 0x00 },
    reg_default { reg: ADAU1372_REG_DAC_CTRL, def: 0x18 },
    reg_default { reg: ADAU1372_REG_DAC_VOL(0), def: 0x00 },
    reg_default { reg: ADAU1372_REG_DAC_VOL(1), def: 0x00 },
    reg_default { reg: ADAU1372_REG_OP_STAGE_MUTE, def: 0x0f },
    reg_default { reg: ADAU1372_REG_SAI0, def: 0x00 },
    reg_default { reg: ADAU1372_REG_SAI1, def: 0x00 },
    reg_default { reg: ADAU1372_REG_SOUT_CTRL, def: 0x00 },
    reg_default { reg: ADAU1372_REG_MODE_MP(0), def: 0x00 },
    reg_default { reg: ADAU1372_REG_MODE_MP(1), def: 0x10 },
    reg_default { reg: ADAU1372_REG_MODE_MP(4), def: 0x00 },
    reg_default { reg: ADAU1372_REG_MODE_MP(5), def: 0x00 },
    reg_default { reg: ADAU1372_REG_MODE_MP(6), def: 0x11 },
    reg_default { reg: ADAU1372_REG_OP_STAGE_CTRL, def: 0x0f },
    reg_default { reg: ADAU1372_REG_DECIM_PWR, def: 0x00 },
    reg_default { reg: ADAU1372_REG_INTERP_PWR, def: 0x00 },
    reg_default { reg: ADAU1372_REG_BIAS_CTRL0, def: 0x00 },
    reg_default { reg: ADAU1372_REG_BIAS_CTRL1, def: 0x00 },
];

unsafe extern "C" fn adau1372_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    if reg == ADAU1372_REG_PLL(5) {
        return true;
    }

    false
}

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: usize,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

const REGCACHE_MAPLE: c_uint = 0;

#[no_mangle]
pub static adau1372_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 16,
    max_register: 0x4d,
    reg_defaults: adau1372_reg_defaults.as_ptr(),
    num_reg_defaults: adau1372_reg_defaults.len(),
    volatile_reg: Some(adau1372_volatile_register),
    cache_type: REGCACHE_MAPLE,
};
/* EXPORT_SYMBOL_GPL(adau1372_regmap_config); */

#[no_mangle]
pub static adau1372_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"adi,adau1372\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* EXPORT_SYMBOL_GPL(adau1372_of_match);
 * MODULE_DEVICE_TABLE(of, adau1372_of_match);
 *
 * MODULE_DESCRIPTION("ASoC ADAU1372 CODEC driver");
 * MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
