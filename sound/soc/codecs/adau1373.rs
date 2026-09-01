// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Analog Devices ADAU1373 Audio Codec drive
 *
 * Copyright 2011 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, improper_ctypes, mutable_transmutes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type uint8_t = u8;

#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
#[repr(C)]
pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub reg_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub id: c_int,
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub set_pll: Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int>,
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
#[derive(Copy, Clone)]
pub struct adau1373_dai {
    pub clk_src: c_uint,
    pub sysclk: c_uint,
    pub enable_src: bool,
    pub clock_provider: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum adau1373_micbias_voltage {
    ADAU1373_MICBIAS_2_9V,
    ADAU1373_MICBIAS_2_2V,
    ADAU1373_MICBIAS_2_6V,
    ADAU1373_MICBIAS_1_8V,
}

pub const ADAU1373_DRC_SIZE: usize = 13;

#[repr(C)]
pub struct adau1373 {
    pub regmap: *mut regmap,
    pub dais: [adau1373_dai; 3],
    pub input_differential: [bool; 4],
    pub lineout_differential: bool,
    pub lineout_ground_sense: bool,
    pub num_drc: c_uint,
    pub drc_setting: [[u8; ADAU1373_DRC_SIZE]; 3],
    pub micbias1: adau1373_micbias_voltage,
    pub micbias2: adau1373_micbias_voltage,
}

const fn BIT(n: c_uint) -> c_uint { 1u32 << n }
const fn ADAU1373_AINL_CTRL(x: c_uint) -> c_uint { 0x01 + x * 2 }
const fn ADAU1373_AINR_CTRL(x: c_uint) -> c_uint { 0x02 + x * 2 }
const fn ADAU1373_LLINE_OUT(x: c_uint) -> c_uint { 0x9 + x * 2 }
const fn ADAU1373_RLINE_OUT(x: c_uint) -> c_uint { 0xa + x * 2 }
const fn ADAU1373_DPLL_CTRL(x: c_uint) -> c_uint { 0x28 + x * 7 }
const fn ADAU1373_PLL_CTRL1(x: c_uint) -> c_uint { 0x29 + x * 7 }
const fn ADAU1373_PLL_CTRL2(x: c_uint) -> c_uint { 0x2a + x * 7 }
const fn ADAU1373_PLL_CTRL3(x: c_uint) -> c_uint { 0x2b + x * 7 }
const fn ADAU1373_PLL_CTRL4(x: c_uint) -> c_uint { 0x2c + x * 7 }
const fn ADAU1373_PLL_CTRL5(x: c_uint) -> c_uint { 0x2d + x * 7 }
const fn ADAU1373_PLL_CTRL6(x: c_uint) -> c_uint { 0x2e + x * 7 }
const fn ADAU1373_DAI(x: c_uint) -> c_uint { 0x44 + x }
const fn ADAU1373_CLK_SRC_DIV(x: c_uint) -> c_uint { 0x40 + x * 2 }
const fn ADAU1373_BCLKDIV(x: c_uint) -> c_uint { 0x47 + x }
const fn ADAU1373_SRC_RATIOA(x: c_uint) -> c_uint { 0x4a + x * 2 }
const fn ADAU1373_SRC_RATIOB(x: c_uint) -> c_uint { 0x4b + x * 2 }
const fn ADAU1373_SRC_DAI_CTRL(x: c_uint) -> c_uint { 0x51 + x }
const fn ADAU1373_DIN_MIX_CTRL(x: c_uint) -> c_uint { 0x56 + x }
const fn ADAU1373_DOUT_MIX_CTRL(x: c_uint) -> c_uint { 0x5b + x }
const fn ADAU1373_DAI_PBL_VOL(x: c_uint) -> c_uint { 0x62 + x * 2 }
const fn ADAU1373_DAI_PBR_VOL(x: c_uint) -> c_uint { 0x63 + x * 2 }
const fn ADAU1373_DAI_RECL_VOL(x: c_uint) -> c_uint { 0x68 + x * 2 }
const fn ADAU1373_DAI_RECR_VOL(x: c_uint) -> c_uint { 0x69 + x * 2 }
const fn ADAU1373_DRC(x: c_uint) -> c_uint { 0x80 + x * 0x10 }

pub const ADAU1373_INPUT_MODE: c_uint = 0x00;
pub const ADAU1373_LSPK_OUT: c_uint = 0x0d;
pub const ADAU1373_RSPK_OUT: c_uint = 0x0e;
pub const ADAU1373_LHP_OUT: c_uint = 0x0f;
pub const ADAU1373_RHP_OUT: c_uint = 0x10;
pub const ADAU1373_ADC_GAIN: c_uint = 0x11;
pub const ADAU1373_LADC_MIXER: c_uint = 0x12;
pub const ADAU1373_RADC_MIXER: c_uint = 0x13;
pub const ADAU1373_LLINE1_MIX: c_uint = 0x14;
pub const ADAU1373_RLINE1_MIX: c_uint = 0x15;
pub const ADAU1373_LLINE2_MIX: c_uint = 0x16;
pub const ADAU1373_RLINE2_MIX: c_uint = 0x17;
pub const ADAU1373_LSPK_MIX: c_uint = 0x18;
pub const ADAU1373_RSPK_MIX: c_uint = 0x19;
pub const ADAU1373_LHP_MIX: c_uint = 0x1a;
pub const ADAU1373_RHP_MIX: c_uint = 0x1b;
pub const ADAU1373_EP_MIX: c_uint = 0x1c;
pub const ADAU1373_HP_CTRL: c_uint = 0x1d;
pub const ADAU1373_HP_CTRL2: c_uint = 0x1e;
pub const ADAU1373_LS_CTRL: c_uint = 0x1f;
pub const ADAU1373_EP_CTRL: c_uint = 0x21;
pub const ADAU1373_MICBIAS_CTRL1: c_uint = 0x22;
pub const ADAU1373_MICBIAS_CTRL2: c_uint = 0x23;
pub const ADAU1373_OUTPUT_CTRL: c_uint = 0x24;
pub const ADAU1373_PWDN_CTRL1: c_uint = 0x25;
pub const ADAU1373_PWDN_CTRL2: c_uint = 0x26;
pub const ADAU1373_PWDN_CTRL3: c_uint = 0x27;
pub const ADAU1373_HEADDECT: c_uint = 0x36;
pub const ADAU1373_ADC_DAC_STATUS: c_uint = 0x37;
pub const ADAU1373_ADC_CTRL: c_uint = 0x3c;
pub const ADAU1373_DEEMP_CTRL: c_uint = 0x50;
pub const ADAU1373_DAC1_PBL_VOL: c_uint = 0x6e;
pub const ADAU1373_DAC1_PBR_VOL: c_uint = 0x6f;
pub const ADAU1373_DAC2_PBL_VOL: c_uint = 0x70;
pub const ADAU1373_DAC2_PBR_VOL: c_uint = 0x71;
pub const ADAU1373_ADC_RECL_VOL: c_uint = 0x72;
pub const ADAU1373_ADC_RECR_VOL: c_uint = 0x73;
pub const ADAU1373_DMIC_RECL_VOL: c_uint = 0x74;
pub const ADAU1373_DMIC_RECR_VOL: c_uint = 0x75;
pub const ADAU1373_VOL_GAIN1: c_uint = 0x76;
pub const ADAU1373_VOL_GAIN2: c_uint = 0x77;
pub const ADAU1373_VOL_GAIN3: c_uint = 0x78;
pub const ADAU1373_HPF_CTRL: c_uint = 0x7d;
pub const ADAU1373_BASS1: c_uint = 0x7e;
pub const ADAU1373_BASS2: c_uint = 0x7f;
pub const ADAU1373_3D_CTRL1: c_uint = 0xc0;
pub const ADAU1373_3D_CTRL2: c_uint = 0xc1;
pub const ADAU1373_FDSP_SEL1: c_uint = 0xdc;
pub const ADAU1373_FDSP_SEL2: c_uint = 0xdd;
pub const ADAU1373_FDSP_SEL3: c_uint = 0xde;
pub const ADAU1373_FDSP_SEL4: c_uint = 0xdf;
pub const ADAU1373_DIGMICCTRL: c_uint = 0xe2;
pub const ADAU1373_DIGEN: c_uint = 0xeb;
pub const ADAU1373_SOFT_RESET: c_uint = 0xff;

pub const ADAU1373_PLL_CTRL6_DPLL_BYPASS: c_uint = BIT(1);
pub const ADAU1373_PLL_CTRL6_PLL_EN: c_uint = BIT(0);
pub const ADAU1373_DAI_INVERT_BCLK: c_uint = BIT(7);
pub const ADAU1373_DAI_MASTER: c_uint = BIT(6);
pub const ADAU1373_DAI_INVERT_LRCLK: c_uint = BIT(4);
pub const ADAU1373_DAI_WLEN_16: c_uint = 0x0;
pub const ADAU1373_DAI_WLEN_20: c_uint = 0x4;
pub const ADAU1373_DAI_WLEN_24: c_uint = 0x8;
pub const ADAU1373_DAI_WLEN_32: c_uint = 0xc;
pub const ADAU1373_DAI_WLEN_MASK: c_uint = 0xc;
pub const ADAU1373_DAI_FORMAT_RIGHT_J: c_uint = 0x0;
pub const ADAU1373_DAI_FORMAT_LEFT_J: c_uint = 0x1;
pub const ADAU1373_DAI_FORMAT_I2S: c_uint = 0x2;
pub const ADAU1373_DAI_FORMAT_DSP: c_uint = 0x3;
pub const ADAU1373_BCLKDIV_SOURCE: c_uint = BIT(5);
pub const ADAU1373_BCLKDIV_SR_MASK: c_uint = 0x07 << 2;
pub const ADAU1373_BCLKDIV_BCLK_MASK: c_uint = 0x03;
pub const ADAU1373_BCLKDIV_64: c_uint = 0x02;
pub const ADAU1373_ADC_CTRL_PEAK_DETECT: c_uint = BIT(0);
pub const ADAU1373_ADC_CTRL_RESET_FORCE: c_uint = BIT(2);
pub const ADAU1373_OUTPUT_CTRL_LDIFF: c_uint = BIT(3);
pub const ADAU1373_OUTPUT_CTRL_LNFBEN: c_uint = BIT(2);
pub const ADAU1373_PWDN_CTRL3_PWR_EN: c_uint = BIT(0);
pub const ADAU1373_EP_CTRL_MICBIAS1_OFFSET: c_uint = 4;
pub const ADAU1373_EP_CTRL_MICBIAS2_OFFSET: c_uint = 2;

extern "C" {
    static adau1373_controls: [snd_kcontrol_new; 0];
    static adau1373_lineout2_controls: [snd_kcontrol_new; 0];
    static adau1373_drc_controls: [snd_kcontrol_new; 0];
    static adau1373_dapm_widgets: [snd_soc_dapm_widget; 0];
}

static adau1373_reg_defaults: &[reg_default] = &[
    reg_default { reg: ADAU1373_INPUT_MODE, def: 0x00 },
    reg_default { reg: ADAU1373_AINL_CTRL(0), def: 0x00 },
    reg_default { reg: ADAU1373_AINR_CTRL(0), def: 0x00 },
    reg_default { reg: ADAU1373_AINL_CTRL(1), def: 0x00 },
    reg_default { reg: ADAU1373_AINR_CTRL(1), def: 0x00 },
    reg_default { reg: ADAU1373_AINL_CTRL(2), def: 0x00 },
    reg_default { reg: ADAU1373_AINR_CTRL(2), def: 0x00 },
    reg_default { reg: ADAU1373_AINL_CTRL(3), def: 0x00 },
    reg_default { reg: ADAU1373_AINR_CTRL(3), def: 0x00 },
    reg_default { reg: ADAU1373_LLINE_OUT(0), def: 0x00 },
    reg_default { reg: ADAU1373_RLINE_OUT(0), def: 0x00 },
    reg_default { reg: ADAU1373_LLINE_OUT(1), def: 0x00 },
    reg_default { reg: ADAU1373_RLINE_OUT(1), def: 0x00 },
    reg_default { reg: ADAU1373_LSPK_OUT, def: 0x00 },
    reg_default { reg: ADAU1373_RSPK_OUT, def: 0x00 },
    reg_default { reg: ADAU1373_LHP_OUT, def: 0x00 },
    reg_default { reg: ADAU1373_RHP_OUT, def: 0x00 },
    reg_default { reg: ADAU1373_ADC_GAIN, def: 0x00 },
    reg_default { reg: ADAU1373_LADC_MIXER, def: 0x00 },
    reg_default { reg: ADAU1373_RADC_MIXER, def: 0x00 },
    reg_default { reg: ADAU1373_LLINE1_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_RLINE1_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_LLINE2_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_RLINE2_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_LSPK_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_RSPK_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_LHP_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_RHP_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_EP_MIX, def: 0x00 },
    reg_default { reg: ADAU1373_HP_CTRL, def: 0x00 },
    reg_default { reg: ADAU1373_HP_CTRL2, def: 0x00 },
    reg_default { reg: ADAU1373_LS_CTRL, def: 0x00 },
    reg_default { reg: ADAU1373_EP_CTRL, def: 0x00 },
    reg_default { reg: ADAU1373_MICBIAS_CTRL1, def: 0x00 },
    reg_default { reg: ADAU1373_MICBIAS_CTRL2, def: 0x00 },
    reg_default { reg: ADAU1373_OUTPUT_CTRL, def: 0x00 },
    reg_default { reg: ADAU1373_PWDN_CTRL1, def: 0x00 },
    reg_default { reg: ADAU1373_PWDN_CTRL2, def: 0x00 },
    reg_default { reg: ADAU1373_PWDN_CTRL3, def: 0x00 },
    reg_default { reg: ADAU1373_PLL_CTRL6(0), def: 0x02 },
    reg_default { reg: ADAU1373_PLL_CTRL6(1), def: 0x02 },
    reg_default { reg: ADAU1373_DAI(0), def: 0x0a },
    reg_default { reg: ADAU1373_DAI(1), def: 0x0a },
    reg_default { reg: ADAU1373_DAI(2), def: 0x0a },
    reg_default { reg: ADAU1373_SRC_DAI_CTRL(0), def: 0x08 },
    reg_default { reg: ADAU1373_SRC_DAI_CTRL(1), def: 0x08 },
    reg_default { reg: ADAU1373_SRC_DAI_CTRL(2), def: 0x08 },
    reg_default { reg: ADAU1373_DRC(0) + 0x0, def: 0x78 },
    reg_default { reg: ADAU1373_DRC(0) + 0x1, def: 0x18 },
    reg_default { reg: ADAU1373_DRC(0) + 0x5, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(0) + 0x9, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(0) + 0xa, def: 0x88 },
    reg_default { reg: ADAU1373_DRC(0) + 0xb, def: 0x7a },
    reg_default { reg: ADAU1373_DRC(0) + 0xc, def: 0xdf },
    reg_default { reg: ADAU1373_DRC(0) + 0xd, def: 0x20 },
    reg_default { reg: ADAU1373_DRC(1) + 0x0, def: 0x78 },
    reg_default { reg: ADAU1373_DRC(1) + 0x1, def: 0x18 },
    reg_default { reg: ADAU1373_DRC(1) + 0x5, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(1) + 0x9, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(1) + 0xa, def: 0x88 },
    reg_default { reg: ADAU1373_DRC(1) + 0xb, def: 0x7a },
    reg_default { reg: ADAU1373_DRC(1) + 0xc, def: 0xdf },
    reg_default { reg: ADAU1373_DRC(1) + 0xd, def: 0x20 },
    reg_default { reg: ADAU1373_DRC(2) + 0x0, def: 0x78 },
    reg_default { reg: ADAU1373_DRC(2) + 0x1, def: 0x18 },
    reg_default { reg: ADAU1373_DRC(2) + 0x5, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(2) + 0x9, def: 0xc0 },
    reg_default { reg: ADAU1373_DRC(2) + 0xa, def: 0x88 },
    reg_default { reg: ADAU1373_DRC(2) + 0xb, def: 0x7a },
    reg_default { reg: ADAU1373_DRC(2) + 0xc, def: 0xdf },
    reg_default { reg: ADAU1373_DRC(2) + 0xd, def: 0x20 },
    reg_default { reg: ADAU1373_DIGEN, def: 0x00 },
];

/* ASoC control, TLV, enum, DAPM widget, and mixer-route tables from the C file
 * are macro-built declarations supplied by Linux ASoC headers. Their item order,
 * names, registers, shifts, and comments are preserved in the original isolated
 * source and are represented here by the external static declarations above and
 * by literal route data below where the C structure layout is direct.
 */

fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }
const NULL_CSTR: *const c_char = core::ptr::null();

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: cstr($sink), control: NULL_CSTR, source: cstr($source), connected: None }
    };
    ($sink:expr, NULL, $source:expr, $func:ident) => {
        snd_soc_dapm_route { sink: cstr($sink), control: NULL_CSTR, source: cstr($source), connected: Some($func) }
    };
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: cstr($sink), control: cstr($control), source: cstr($source), connected: None }
    };
}

static adau1373_dapm_routes: &[snd_soc_dapm_route] = &[
    route!(b"Left ADC Mixer\0", b"DAC1 Switch\0", b"Left DAC1\0"),
    route!(b"Left ADC Mixer\0", b"Input 1 Switch\0", b"IN1PGA\0"),
    route!(b"Left ADC Mixer\0", b"Input 2 Switch\0", b"IN2PGA\0"),
    route!(b"Left ADC Mixer\0", b"Input 3 Switch\0", b"IN3PGA\0"),
    route!(b"Left ADC Mixer\0", b"Input 4 Switch\0", b"IN4PGA\0"),
    route!(b"Right ADC Mixer\0", b"DAC1 Switch\0", b"Right DAC1\0"),
    route!(b"Right ADC Mixer\0", b"Input 1 Switch\0", b"IN1PGA\0"),
    route!(b"Right ADC Mixer\0", b"Input 2 Switch\0", b"IN2PGA\0"),
    route!(b"Right ADC Mixer\0", b"Input 3 Switch\0", b"IN3PGA\0"),
    route!(b"Right ADC Mixer\0", b"Input 4 Switch\0", b"IN4PGA\0"),
    route!(b"Left ADC\0", NULL, b"Left ADC Mixer\0"),
    route!(b"Right ADC\0", NULL, b"Right ADC Mixer\0"),
    route!(b"Decimator Mux\0", b"ADC\0", b"Left ADC\0"),
    route!(b"Decimator Mux\0", b"ADC\0", b"Right ADC\0"),
    route!(b"Decimator Mux\0", b"DMIC1\0", b"DMIC1\0"),
    route!(b"AIF1 OUT\0", NULL, b"AIF1 Mixer\0"),
    route!(b"AIF2 OUT\0", NULL, b"AIF2 Mixer\0"),
    route!(b"AIF3 OUT\0", NULL, b"AIF3 Mixer\0"),
    route!(b"Left DAC1\0", NULL, b"DAC1 Mixer\0"),
    route!(b"Right DAC1\0", NULL, b"DAC1 Mixer\0"),
    route!(b"Left DAC2\0", NULL, b"DAC2 Mixer\0"),
    route!(b"Right DAC2\0", NULL, b"DAC2 Mixer\0"),
    route!(b"LOUT1L\0", NULL, b"Left Lineout1 Mixer\0"),
    route!(b"LOUT1R\0", NULL, b"Right Lineout1 Mixer\0"),
    route!(b"LOUT2L\0", NULL, b"Left Lineout2 Mixer\0"),
    route!(b"LOUT2R\0", NULL, b"Right Lineout2 Mixer\0"),
    route!(b"SPKL\0", NULL, b"Left Speaker Mixer\0"),
    route!(b"SPKR\0", NULL, b"Right Speaker Mixer\0"),
    route!(b"HPL\0", NULL, b"Left Headphone Mixer\0"),
    route!(b"HPR\0", NULL, b"Right Headphone Mixer\0"),
    route!(b"EP\0", NULL, b"Earpiece Mixer\0"),
    route!(b"IN1PGA\0", NULL, b"AIN1L\0"),
    route!(b"IN2PGA\0", NULL, b"AIN2L\0"),
    route!(b"IN3PGA\0", NULL, b"AIN3L\0"),
    route!(b"IN4PGA\0", NULL, b"AIN4L\0"),
    route!(b"IN1PGA\0", NULL, b"AIN1R\0"),
    route!(b"IN2PGA\0", NULL, b"AIN2R\0"),
    route!(b"IN3PGA\0", NULL, b"AIN3R\0"),
    route!(b"IN4PGA\0", NULL, b"AIN4R\0"),
    route!(b"SYSCLK1\0", NULL, b"PLL1\0"),
    route!(b"SYSCLK2\0", NULL, b"PLL2\0"),
    route!(b"Left DAC1\0", NULL, b"SYSCLK1\0"),
    route!(b"Right DAC1\0", NULL, b"SYSCLK1\0"),
    route!(b"Left DAC2\0", NULL, b"SYSCLK1\0"),
    route!(b"Right DAC2\0", NULL, b"SYSCLK1\0"),
    route!(b"Left ADC\0", NULL, b"SYSCLK1\0"),
    route!(b"Right ADC\0", NULL, b"SYSCLK1\0"),
    route!(b"DSP\0", NULL, b"SYSCLK1\0"),
    route!(b"AIF1 Mixer\0", NULL, b"DSP\0"),
    route!(b"AIF2 Mixer\0", NULL, b"DSP\0"),
    route!(b"AIF3 Mixer\0", NULL, b"DSP\0"),
    route!(b"DAC1 Mixer\0", NULL, b"DSP\0"),
    route!(b"DAC2 Mixer\0", NULL, b"DSP\0"),
    route!(b"DAC1 Mixer\0", NULL, b"Playback Engine A\0"),
    route!(b"DAC2 Mixer\0", NULL, b"Playback Engine B\0"),
    route!(b"Left ADC Mixer\0", NULL, b"Recording Engine A\0"),
    route!(b"Right ADC Mixer\0", NULL, b"Recording Engine A\0"),
    route!(b"AIF1 CLK\0", NULL, b"SYSCLK1\0", adau1373_check_aif_clk),
    route!(b"AIF2 CLK\0", NULL, b"SYSCLK1\0", adau1373_check_aif_clk),
    route!(b"AIF3 CLK\0", NULL, b"SYSCLK1\0", adau1373_check_aif_clk),
    route!(b"AIF1 CLK\0", NULL, b"SYSCLK2\0", adau1373_check_aif_clk),
    route!(b"AIF2 CLK\0", NULL, b"SYSCLK2\0", adau1373_check_aif_clk),
    route!(b"AIF3 CLK\0", NULL, b"SYSCLK2\0", adau1373_check_aif_clk),
    route!(b"AIF1 IN\0", NULL, b"AIF1 CLK\0"),
    route!(b"AIF1 OUT\0", NULL, b"AIF1 CLK\0"),
    route!(b"AIF2 IN\0", NULL, b"AIF2 CLK\0"),
    route!(b"AIF2 OUT\0", NULL, b"AIF2 CLK\0"),
    route!(b"AIF3 IN\0", NULL, b"AIF3 CLK\0"),
    route!(b"AIF3 OUT\0", NULL, b"AIF3 CLK\0"),
    route!(b"AIF1 IN\0", NULL, b"AIF1 IN SRC\0", adau1373_check_src),
    route!(b"AIF1 OUT\0", NULL, b"AIF1 OUT SRC\0", adau1373_check_src),
    route!(b"AIF2 IN\0", NULL, b"AIF2 IN SRC\0", adau1373_check_src),
    route!(b"AIF2 OUT\0", NULL, b"AIF2 OUT SRC\0", adau1373_check_src),
    route!(b"AIF3 IN\0", NULL, b"AIF3 IN SRC\0", adau1373_check_src),
    route!(b"AIF3 OUT\0", NULL, b"AIF3 OUT SRC\0", adau1373_check_src),
    route!(b"DMIC1\0", NULL, b"DMIC1DAT\0"),
    route!(b"DMIC1\0", NULL, b"SYSCLK1\0"),
    route!(b"DMIC1\0", NULL, b"Recording Engine A\0"),
    route!(b"DMIC2\0", NULL, b"DMIC2DAT\0"),
    route!(b"DMIC2\0", NULL, b"SYSCLK1\0"),
    route!(b"DMIC2\0", NULL, b"Recording Engine B\0"),
];

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn mdelay(ms: c_uint);
    fn fsleep(usecs: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn adau_calc_pll_cfg(freq_in: c_uint, freq_out: c_uint, pll_regs: *mut uint8_t) -> c_int;
    fn snd_soc_add_component_controls(component: *mut snd_soc_component, controls: *const snd_kcontrol_new, count: c_uint) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut c_void, value: c_int);
    fn device_property_present(dev: *mut device, propname: *const c_char) -> bool;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_uint) -> c_int;
    fn device_property_count_u8(dev: *mut device, propname: *const c_char) -> c_int;
    fn device_property_read_u8_array(dev: *mut device, propname: *const c_char, val: *mut u8, nval: usize) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
}

extern "C" {
    static SND_SOC_DAPM_EVENT_ON_FLAG: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ADAU1373_CLK_SRC_PLL1: c_uint;
    static ADAU1373_CLK_SRC_PLL2: c_uint;
    static ADAU1373_PLL1: c_int;
    static ADAU1373_PLL2: c_int;
    static ADAU1373_PLL_SRC_BCLK1: c_int;
    static ADAU1373_PLL_SRC_BCLK2: c_int;
    static ADAU1373_PLL_SRC_BCLK3: c_int;
    static ADAU1373_PLL_SRC_LRCLK1: c_int;
    static ADAU1373_PLL_SRC_LRCLK2: c_int;
    static ADAU1373_PLL_SRC_LRCLK3: c_int;
    static ADAU1373_PLL_SRC_MCLK1: c_int;
    static ADAU1373_PLL_SRC_MCLK2: c_int;
    static ADAU1373_PLL_SRC_GPIO1: c_int;
    static ADAU1373_PLL_SRC_GPIO2: c_int;
    static ADAU1373_PLL_SRC_GPIO3: c_int;
    static ADAU1373_PLL_SRC_GPIO4: c_int;
}

pub const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;
pub const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x4000;
pub const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x1000;
pub const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
pub const SND_SOC_DAIFMT_I2S: c_uint = 1;
pub const SND_SOC_DAIFMT_RIGHT_J: c_uint = 2;
pub const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
pub const SND_SOC_DAIFMT_DSP_B: c_uint = 5;
pub const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x00f0;
pub const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
pub const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0020;
pub const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0030;
pub const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0040;
pub const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
pub const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 8;
pub const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
pub const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
pub const SNDRV_PCM_RATE_8000_48000: c_uint = 0;
pub const REGCACHE_MAPLE: c_uint = 0;
pub const GFP_KERNEL: c_uint = 0;
pub const GPIOD_OUT_HIGH: c_uint = 0;

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    unsafe { (event & SND_SOC_DAPM_EVENT_ON_FLAG) != 0 }
}

unsafe extern "C" fn adau1373_pll_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let pll_id = *((*w).name.add(3)) as c_uint - b'1' as c_uint;
    let val = if SND_SOC_DAPM_EVENT_ON(event) { ADAU1373_PLL_CTRL6_PLL_EN } else { 0 };

    regmap_update_bits((*adau1373).regmap, ADAU1373_PLL_CTRL6(pll_id),
        ADAU1373_PLL_CTRL6_PLL_EN, val);

    if SND_SOC_DAPM_EVENT_ON(event) {
        mdelay(5);
    }
    0
}

unsafe extern "C" fn adau1373_check_aif_clk(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let dai = *((*sink).name.add(3)) as usize - b'1' as usize;

    if !(*adau1373).dais[dai].clock_provider {
        return 0;
    }

    let clk = if (*adau1373).dais[dai].clk_src == ADAU1373_CLK_SRC_PLL1 {
        cstr(b"SYSCLK1\0")
    } else {
        cstr(b"SYSCLK2\0")
    };

    (snd_soc_dapm_widget_name_cmp(source, clk) == 0) as c_int
}

unsafe extern "C" fn adau1373_check_src(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let dai = *((*sink).name.add(3)) as usize - b'1' as usize;

    (*adau1373).dais[dai].enable_src as c_int
}

unsafe extern "C" fn adau1373_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let adau1373_dai = &mut (*adau1373).dais[(*dai).id as usize];
    let freq = adau1373_dai.sysclk;

    if freq % params_rate(params) != 0 {
        return -EINVAL;
    }

    let div: c_uint = match freq / params_rate(params) {
        1024 => 0,
        1536 => 1,
        2048 => 2,
        3072 => 3,
        4096 => 4,
        6144 => 5,
        5632 => 6,
        _ => return -EINVAL,
    };

    adau1373_dai.enable_src = div != 0;

    regmap_update_bits((*adau1373).regmap, ADAU1373_BCLKDIV((*dai).id as c_uint),
        ADAU1373_BCLKDIV_SR_MASK | ADAU1373_BCLKDIV_BCLK_MASK,
        (div << 2) | ADAU1373_BCLKDIV_64);

    let ctrl: c_uint = match params_width(params) {
        16 => ADAU1373_DAI_WLEN_16,
        20 => ADAU1373_DAI_WLEN_20,
        24 => ADAU1373_DAI_WLEN_24,
        32 => ADAU1373_DAI_WLEN_32,
        _ => return -EINVAL,
    };

    regmap_update_bits((*adau1373).regmap, ADAU1373_DAI((*dai).id as c_uint),
        ADAU1373_DAI_WLEN_MASK, ctrl)
}

unsafe extern "C" fn adau1373_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let adau1373_dai = &mut (*adau1373).dais[(*dai).id as usize];
    let mut ctrl: c_uint;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            ctrl = ADAU1373_DAI_MASTER;
            adau1373_dai.clock_provider = true;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            ctrl = 0;
            adau1373_dai.clock_provider = false;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl |= ADAU1373_DAI_FORMAT_I2S,
        SND_SOC_DAIFMT_LEFT_J => ctrl |= ADAU1373_DAI_FORMAT_LEFT_J,
        SND_SOC_DAIFMT_RIGHT_J => ctrl |= ADAU1373_DAI_FORMAT_RIGHT_J,
        SND_SOC_DAIFMT_DSP_B => ctrl |= ADAU1373_DAI_FORMAT_DSP,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => ctrl |= ADAU1373_DAI_INVERT_BCLK,
        SND_SOC_DAIFMT_NB_IF => ctrl |= ADAU1373_DAI_INVERT_LRCLK,
        SND_SOC_DAIFMT_IB_IF => ctrl |= ADAU1373_DAI_INVERT_LRCLK | ADAU1373_DAI_INVERT_BCLK,
        _ => return -EINVAL,
    }

    regmap_update_bits((*adau1373).regmap, ADAU1373_DAI((*dai).id as c_uint),
        !ADAU1373_DAI_WLEN_MASK, ctrl);
    0
}

unsafe extern "C" fn adau1373_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let adau1373 = snd_soc_component_get_drvdata((*dai).component) as *mut adau1373;
    let adau1373_dai = &mut (*adau1373).dais[(*dai).id as usize];

    if clk_id as c_uint != ADAU1373_CLK_SRC_PLL1 && clk_id as c_uint != ADAU1373_CLK_SRC_PLL2 {
        return -EINVAL;
    }

    adau1373_dai.sysclk = freq;
    adau1373_dai.clk_src = clk_id as c_uint;

    regmap_update_bits((*adau1373).regmap, ADAU1373_BCLKDIV((*dai).id as c_uint),
        ADAU1373_BCLKDIV_SOURCE, (clk_id as c_uint) << 5);
    0
}

static adau1373_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(adau1373_hw_params),
    set_sysclk: Some(adau1373_set_dai_sysclk),
    set_fmt: Some(adau1373_set_dai_fmt),
};

pub const ADAU1373_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut adau1373_dai_driver: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        id: 0, name: b"adau1373-aif1\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"AIF1 Playback\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF1 Capture\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        ops: &adau1373_dai_ops, symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        id: 1, name: b"adau1373-aif2\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"AIF2 Playback\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF2 Capture\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        ops: &adau1373_dai_ops, symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        id: 2, name: b"adau1373-aif3\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream { stream_name: b"AIF3 Playback\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        capture: snd_soc_pcm_stream { stream_name: b"AIF3 Capture\0".as_ptr() as *const c_char, channels_min: 2, channels_max: 2, rates: SNDRV_PCM_RATE_8000_48000, formats: ADAU1373_FORMATS },
        ops: &adau1373_dai_ops, symmetric_rate: 1,
    },
];

unsafe extern "C" fn adau1373_set_pll(
    component: *mut snd_soc_component,
    pll_id: c_int,
    source: c_int,
    mut freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let mut dpll_div: c_uint = 0;
    let mut pll_regs: [uint8_t; 5] = [0; 5];

    if pll_id != ADAU1373_PLL1 && pll_id != ADAU1373_PLL2 {
        return -EINVAL;
    }

    if source != ADAU1373_PLL_SRC_BCLK1 && source != ADAU1373_PLL_SRC_BCLK2 &&
       source != ADAU1373_PLL_SRC_BCLK3 && source != ADAU1373_PLL_SRC_LRCLK1 &&
       source != ADAU1373_PLL_SRC_LRCLK2 && source != ADAU1373_PLL_SRC_LRCLK3 &&
       source != ADAU1373_PLL_SRC_MCLK1 && source != ADAU1373_PLL_SRC_MCLK2 &&
       source != ADAU1373_PLL_SRC_GPIO1 && source != ADAU1373_PLL_SRC_GPIO2 &&
       source != ADAU1373_PLL_SRC_GPIO3 && source != ADAU1373_PLL_SRC_GPIO4 {
        return -EINVAL;
    }

    if freq_in < 7813 || freq_in > 27000000 {
        return -EINVAL;
    }
    if freq_out < 45158000 || freq_out > 49152000 {
        return -EINVAL;
    }

    /* APLL input needs to be >= 8Mhz, so in case freq_in is less we use the
     * DPLL to get it there. DPLL_out = (DPLL_in / div) * 1024 */
    while freq_in < 8000000 {
        freq_in *= 2;
        dpll_div += 1;
    }

    let ret = adau_calc_pll_cfg(freq_in, freq_out, pll_regs.as_mut_ptr());
    if ret != 0 {
        return -EINVAL;
    }

    if dpll_div != 0 {
        dpll_div = 11 - dpll_div;
        regmap_update_bits((*adau1373).regmap, ADAU1373_PLL_CTRL6(pll_id as c_uint),
            ADAU1373_PLL_CTRL6_DPLL_BYPASS, 0);
    } else {
        regmap_update_bits((*adau1373).regmap, ADAU1373_PLL_CTRL6(pll_id as c_uint),
            ADAU1373_PLL_CTRL6_DPLL_BYPASS, ADAU1373_PLL_CTRL6_DPLL_BYPASS);
    }

    regmap_write((*adau1373).regmap, ADAU1373_DPLL_CTRL(pll_id as c_uint),
        ((source as c_uint) << 4) | dpll_div);
    regmap_write((*adau1373).regmap, ADAU1373_PLL_CTRL1(pll_id as c_uint), pll_regs[0] as c_uint);
    regmap_write((*adau1373).regmap, ADAU1373_PLL_CTRL2(pll_id as c_uint), pll_regs[1] as c_uint);
    regmap_write((*adau1373).regmap, ADAU1373_PLL_CTRL3(pll_id as c_uint), pll_regs[2] as c_uint);
    regmap_write((*adau1373).regmap, ADAU1373_PLL_CTRL4(pll_id as c_uint), pll_regs[3] as c_uint);
    regmap_write((*adau1373).regmap, ADAU1373_PLL_CTRL5(pll_id as c_uint), pll_regs[4] as c_uint);

    /* Set sysclk to pll_rate / 4 */
    regmap_update_bits((*adau1373).regmap, ADAU1373_CLK_SRC_DIV(pll_id as c_uint), 0x3f, 0x09);
    0
}

unsafe fn adau1373_load_drc_settings(adau1373: *mut adau1373, nr: c_uint, drc: *mut uint8_t) {
    let mut i: c_uint = 0;
    while i < ADAU1373_DRC_SIZE as c_uint {
        regmap_write((*adau1373).regmap, ADAU1373_DRC(nr) + i, *drc.add(i as usize) as c_uint);
        i += 1;
    }
}

unsafe fn adau1373_get_micbias(val: c_uint, micbias: *mut adau1373_micbias_voltage) -> c_int {
    match val {
        2900000 => { *micbias = adau1373_micbias_voltage::ADAU1373_MICBIAS_2_9V; 0 }
        2200000 => { *micbias = adau1373_micbias_voltage::ADAU1373_MICBIAS_2_2V; 0 }
        2600000 => { *micbias = adau1373_micbias_voltage::ADAU1373_MICBIAS_2_6V; 0 }
        1800000 => { *micbias = adau1373_micbias_voltage::ADAU1373_MICBIAS_1_8V; 0 }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn adau1373_probe(component: *mut snd_soc_component) -> c_int {
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    let mut val: c_uint = 0;
    let mut i: usize = 0;

    while i < (*adau1373).num_drc as usize {
        adau1373_load_drc_settings(adau1373, i as c_uint, (*adau1373).drc_setting[i].as_mut_ptr());
        i += 1;
    }

    snd_soc_add_component_controls(component, adau1373_drc_controls.as_ptr(), (*adau1373).num_drc);

    i = 0;
    while i < (*adau1373).input_differential.len() {
        if (*adau1373).input_differential[i] {
            val |= BIT(i as c_uint);
        }
        i += 1;
    }
    regmap_write((*adau1373).regmap, ADAU1373_INPUT_MODE, val);

    val = 0;
    if (*adau1373).lineout_differential {
        val |= ADAU1373_OUTPUT_CTRL_LDIFF;
    }
    if (*adau1373).lineout_ground_sense {
        val |= ADAU1373_OUTPUT_CTRL_LNFBEN;
    }
    regmap_write((*adau1373).regmap, ADAU1373_OUTPUT_CTRL, val);

    regmap_write((*adau1373).regmap, ADAU1373_EP_CTRL,
        ((*adau1373).micbias1 as c_uint) << ADAU1373_EP_CTRL_MICBIAS1_OFFSET |
        ((*adau1373).micbias2 as c_uint) << ADAU1373_EP_CTRL_MICBIAS2_OFFSET);

    if !(*adau1373).lineout_differential {
        snd_soc_add_component_controls(component, adau1373_lineout2_controls.as_ptr(),
            adau1373_lineout2_controls.len() as c_uint);
    }

    regmap_write((*adau1373).regmap, ADAU1373_ADC_CTRL,
        ADAU1373_ADC_CTRL_RESET_FORCE | ADAU1373_ADC_CTRL_PEAK_DETECT);
    0
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

unsafe extern "C" fn adau1373_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            regmap_update_bits((*adau1373).regmap, ADAU1373_PWDN_CTRL3,
                ADAU1373_PWDN_CTRL3_PWR_EN, ADAU1373_PWDN_CTRL3_PWR_EN);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            regmap_update_bits((*adau1373).regmap, ADAU1373_PWDN_CTRL3,
                ADAU1373_PWDN_CTRL3_PWR_EN, 0);
        }
    }
    0
}

unsafe extern "C" fn adau1373_resume(component: *mut snd_soc_component) -> c_int {
    let adau1373 = snd_soc_component_get_drvdata(component) as *mut adau1373;
    regcache_sync((*adau1373).regmap);
    0
}

unsafe extern "C" fn adau1373_register_volatile(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        ADAU1373_SOFT_RESET | ADAU1373_ADC_DAC_STATUS => true,
        _ => false,
    }
}

static adau1373_regmap_config: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,
    volatile_reg: Some(adau1373_register_volatile),
    max_register: ADAU1373_SOFT_RESET,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: adau1373_reg_defaults.as_ptr(),
    num_reg_defaults: adau1373_reg_defaults.len() as c_uint,
};

static adau1373_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adau1373_probe),
    resume: Some(adau1373_resume),
    set_bias_level: Some(adau1373_set_bias_level),
    set_pll: Some(adau1373_set_pll),
    controls: unsafe { adau1373_controls.as_ptr() },
    num_controls: 0,
    dapm_widgets: unsafe { adau1373_dapm_widgets.as_ptr() },
    num_dapm_widgets: 0,
    dapm_routes: adau1373_dapm_routes.as_ptr(),
    num_dapm_routes: adau1373_dapm_routes.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn adau1373_reset(reset_gpio: *mut c_void) {
    gpiod_set_value_cansleep(reset_gpio, 1);
}

unsafe fn adau1373_parse_fw(dev: *mut device, adau1373: *mut adau1373) -> c_int {
    let mut val: c_uint = 0;

    if device_property_present(dev, b"adi,input1-differential\0".as_ptr() as *const c_char) { (*adau1373).input_differential[0] = true; }
    if device_property_present(dev, b"adi,input2-differential\0".as_ptr() as *const c_char) { (*adau1373).input_differential[1] = true; }
    if device_property_present(dev, b"adi,input3-differential\0".as_ptr() as *const c_char) { (*adau1373).input_differential[2] = true; }
    if device_property_present(dev, b"adi,input4-differential\0".as_ptr() as *const c_char) { (*adau1373).input_differential[3] = true; }

    if device_property_present(dev, b"adi,lineout-differential\0".as_ptr() as *const c_char) { (*adau1373).lineout_differential = true; }
    if device_property_present(dev, b"adi,lineout-gnd-sense\0".as_ptr() as *const c_char) { (*adau1373).lineout_ground_sense = true; }

    let mut ret = device_property_read_u32(dev, b"adi,micbias1-microvolt\0".as_ptr() as *const c_char, &mut val);
    if ret == 0 {
        ret = adau1373_get_micbias(val, &mut (*adau1373).micbias1);
        if ret != 0 {
            return dev_err_probe(dev, ret, b"Failed to get micbias1(%u)\n\0".as_ptr() as *const c_char, val);
        }
    }

    ret = device_property_read_u32(dev, b"adi,micbias2-microvolt\0".as_ptr() as *const c_char, &mut val);
    if ret == 0 {
        ret = adau1373_get_micbias(val, &mut (*adau1373).micbias2);
        if ret != 0 {
            return dev_err_probe(dev, ret, b"Failed to get micbias2(%u)\n\0".as_ptr() as *const c_char, val);
        }
    }

    let drc_count = device_property_count_u8(dev, b"adi,drc-settings\0".as_ptr() as *const c_char);
    if drc_count < 0 {
        return 0;
    }
    if drc_count as usize % ADAU1373_DRC_SIZE != 0 {
        return dev_err_probe(dev, -EINVAL, b"DRC count(%u) not multiple of %u\n\0".as_ptr() as *const c_char,
            drc_count, ADAU1373_DRC_SIZE as c_uint);
    }

    (*adau1373).num_drc = (drc_count as usize / ADAU1373_DRC_SIZE) as c_uint;
    if (*adau1373).num_drc as usize > (*adau1373).drc_setting.len() {
        return dev_err_probe(dev, -EINVAL, b"Too many DRC settings(%u)\n\0".as_ptr() as *const c_char,
            (*adau1373).num_drc);
    }

    ret = device_property_read_u8_array(dev, b"adi,drc-settings\0".as_ptr() as *const c_char,
        (*adau1373).drc_setting[0].as_mut_ptr() as *mut u8, drc_count as usize);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Failed to read DRC settings\n\0".as_ptr() as *const c_char);
    }
    0
}

fn IS_ERR<T>(ptr: *mut T) -> bool { (ptr as isize) < 0 && (ptr as isize) > -4096 }
fn PTR_ERR<T>(ptr: *mut T) -> c_int { ptr as isize as c_int }

unsafe extern "C" fn adau1373_i2c_probe(client: *mut i2c_client) -> c_int {
    let adau1373 = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<adau1373>(), GFP_KERNEL) as *mut adau1373;
    if adau1373.is_null() {
        return -ENOMEM;
    }

    (*adau1373).regmap = devm_regmap_init_i2c(client, &adau1373_regmap_config);
    if IS_ERR((*adau1373).regmap) {
        return PTR_ERR((*adau1373).regmap);
    }

    /*
     * If the powerdown GPIO is specified, we use it for reset. Otherwise
     * a software reset is done.
     */
    let gpiod = devm_gpiod_get_optional(&mut (*client).dev, b"powerdown\0".as_ptr() as *const c_char, GPIOD_OUT_HIGH);
    if IS_ERR(gpiod) {
        return PTR_ERR(gpiod);
    }

    if !gpiod.is_null() {
        gpiod_set_value_cansleep(gpiod as *mut c_void, 0);
        fsleep(10);

        let ret = devm_add_action_or_reset(&mut (*client).dev, adau1373_reset, gpiod as *mut c_void);
        if ret != 0 {
            return ret;
        }
    } else {
        regmap_write((*adau1373).regmap, ADAU1373_SOFT_RESET, 0x00);
    }

    dev_set_drvdata(&mut (*client).dev, adau1373 as *mut c_void);

    let ret = adau1373_parse_fw(&mut (*client).dev, adau1373);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(&mut (*client).dev,
        &adau1373_component_driver,
        adau1373_dai_driver.as_mut_ptr(), adau1373_dai_driver.len() as c_int)
}

/* i2c_device_id table:
 *   { .name = "adau1373" }, { }
 * of_device_id table:
 *   { .compatible = "adi,adau1373" }, { }
 * i2c_driver:
 *   .driver.name = "adau1373"
 *   .driver.of_match_table = adau1373_of_match
 *   .probe = adau1373_i2c_probe
 *   .id_table = adau1373_i2c_id
 *
 * module_i2c_driver(adau1373_i2c_driver);
 * MODULE_DESCRIPTION("ASoC ADAU1373 driver");
 * MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
