// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ADAV80X Audio Codec driver supporting ADAV801, ADAV803
 *
 * Copyright 2011 Analog Devices Inc.
 * Author: Yi Li <yi.li@analog.com>
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const ADAV80X_PLAYBACK_CTRL: c_uint = 0x04;
const ADAV80X_AUX_IN_CTRL: c_uint = 0x05;
const ADAV80X_REC_CTRL: c_uint = 0x06;
const ADAV80X_AUX_OUT_CTRL: c_uint = 0x07;
const ADAV80X_DPATH_CTRL1: c_uint = 0x62;
const ADAV80X_DPATH_CTRL2: c_uint = 0x63;
const ADAV80X_DAC_CTRL1: c_uint = 0x64;
const ADAV80X_DAC_CTRL2: c_uint = 0x65;
const ADAV80X_DAC_CTRL3: c_uint = 0x66;
const ADAV80X_DAC_L_VOL: c_uint = 0x68;
const ADAV80X_DAC_R_VOL: c_uint = 0x69;
const ADAV80X_PGA_L_VOL: c_uint = 0x6c;
const ADAV80X_PGA_R_VOL: c_uint = 0x6d;
const ADAV80X_ADC_CTRL1: c_uint = 0x6e;
const ADAV80X_ADC_CTRL2: c_uint = 0x6f;
const ADAV80X_ADC_L_VOL: c_uint = 0x70;
const ADAV80X_ADC_R_VOL: c_uint = 0x71;
const ADAV80X_PLL_CTRL1: c_uint = 0x74;
const ADAV80X_PLL_CTRL2: c_uint = 0x75;
const ADAV80X_ICLK_CTRL1: c_uint = 0x76;
const ADAV80X_ICLK_CTRL2: c_uint = 0x77;
const ADAV80X_PLL_CLK_SRC: c_uint = 0x78;
const ADAV80X_PLL_OUTE: c_uint = 0x7a;

const fn bit(x: c_uint) -> c_uint {
    1u32 << x
}

const fn adav80x_pll_clk_src_pll_xin(_pll: c_uint) -> c_uint {
    0x00
}
const fn adav80x_pll_clk_src_pll_mclki(pll: c_uint) -> c_uint {
    0x40u32 << pll
}
const fn adav80x_pll_clk_src_pll_mask(pll: c_uint) -> c_uint {
    0x40u32 << pll
}

const fn adav80x_iclk_ctrl1_dac_src(src: c_uint) -> c_uint {
    src << 5
}
const fn adav80x_iclk_ctrl1_adc_src(src: c_uint) -> c_uint {
    src << 2
}
const fn adav80x_iclk_ctrl1_iclk2_src(src: c_uint) -> c_uint {
    src
}
const fn adav80x_iclk_ctrl2_iclk1_src(src: c_uint) -> c_uint {
    src << 3
}

const ADAV80X_PLL_CTRL1_PLLDIV: c_uint = 0x10;
const fn adav80x_pll_ctrl1_pllpd(pll: c_uint) -> c_uint {
    0x04u32 << pll
}
const ADAV80X_PLL_CTRL1_XTLPD: c_uint = 0x02;

const fn adav80x_pll_ctrl2_field(pll: c_uint, x: c_uint) -> c_uint {
    x << (pll * 4)
}
const fn adav80x_pll_ctrl2_fs_48(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x00)
}
const fn adav80x_pll_ctrl2_fs_32(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x08)
}
const fn adav80x_pll_ctrl2_fs_44(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x0c)
}
const fn adav80x_pll_ctrl2_sel(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x02)
}
const fn adav80x_pll_ctrl2_doub(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x01)
}
const fn adav80x_pll_ctrl2_pll_mask(pll: c_uint) -> c_uint {
    adav80x_pll_ctrl2_field(pll, 0x0f)
}

const ADAV80X_ADC_CTRL1_MODULATOR_MASK: c_uint = 0x80;
const ADAV80X_ADC_CTRL1_MODULATOR_128FS: c_uint = 0x00;
const ADAV80X_ADC_CTRL1_MODULATOR_64FS: c_uint = 0x80;

const ADAV80X_DAC_CTRL1_PD: c_uint = 0x80;

const ADAV80X_DAC_CTRL2_DIV1: c_uint = 0x00;
const ADAV80X_DAC_CTRL2_DIV1_5: c_uint = 0x10;
const ADAV80X_DAC_CTRL2_DIV2: c_uint = 0x20;
const ADAV80X_DAC_CTRL2_DIV3: c_uint = 0x30;
const ADAV80X_DAC_CTRL2_DIV_MASK: c_uint = 0x30;

const ADAV80X_DAC_CTRL2_INTERPOL_256FS: c_uint = 0x00;
const ADAV80X_DAC_CTRL2_INTERPOL_128FS: c_uint = 0x40;
const ADAV80X_DAC_CTRL2_INTERPOL_64FS: c_uint = 0x80;
const ADAV80X_DAC_CTRL2_INTERPOL_MASK: c_uint = 0xc0;

const ADAV80X_DAC_CTRL2_DEEMPH_NONE: c_uint = 0x00;
const ADAV80X_DAC_CTRL2_DEEMPH_44: c_uint = 0x01;
const ADAV80X_DAC_CTRL2_DEEMPH_32: c_uint = 0x02;
const ADAV80X_DAC_CTRL2_DEEMPH_48: c_uint = 0x03;
const ADAV80X_DAC_CTRL2_DEEMPH_MASK: c_uint = 0x01;

const ADAV80X_CAPTURE_MODE_MASTER: c_uint = 0x20;
const ADAV80X_CAPTURE_WORD_LEN24: c_uint = 0x00;
const ADAV80X_CAPTURE_WORD_LEN20: c_uint = 0x04;
const ADAV80X_CAPTRUE_WORD_LEN18: c_uint = 0x08;
const ADAV80X_CAPTURE_WORD_LEN16: c_uint = 0x0c;
const ADAV80X_CAPTURE_WORD_LEN_MASK: c_uint = 0x0c;

const ADAV80X_CAPTURE_MODE_LEFT_J: c_uint = 0x00;
const ADAV80X_CAPTURE_MODE_I2S: c_uint = 0x01;
const ADAV80X_CAPTURE_MODE_RIGHT_J: c_uint = 0x03;
const ADAV80X_CAPTURE_MODE_MASK: c_uint = 0x03;

const ADAV80X_PLAYBACK_MODE_MASTER: c_uint = 0x10;
const ADAV80X_PLAYBACK_MODE_LEFT_J: c_uint = 0x00;
const ADAV80X_PLAYBACK_MODE_I2S: c_uint = 0x01;
const ADAV80X_PLAYBACK_MODE_RIGHT_J_24: c_uint = 0x04;
const ADAV80X_PLAYBACK_MODE_RIGHT_J_20: c_uint = 0x05;
const ADAV80X_PLAYBACK_MODE_RIGHT_J_18: c_uint = 0x06;
const ADAV80X_PLAYBACK_MODE_RIGHT_J_16: c_uint = 0x07;
const ADAV80X_PLAYBACK_MODE_MASK: c_uint = 0x07;

const fn adav80x_pll_oute_sysclkpd(x: c_uint) -> c_uint {
    bit(2 - x)
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 0;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_RATE_32000: u64 = 0;
const SNDRV_PCM_RATE_44100: u64 = 0;
const SNDRV_PCM_RATE_48000: u64 = 0;
const SNDRV_PCM_RATE_64000: u64 = 0;
const SNDRV_PCM_RATE_88200: u64 = 0;
const SNDRV_PCM_RATE_96000: u64 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S18_3LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 0;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 0;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64 = 0;

type Adav80xClkSrc = c_int;
type Adav80xPllSrc = c_int;
const ADAV80X_CLK_XIN: c_int = 0;
const ADAV80X_CLK_XTAL: c_int = 1;
const ADAV80X_CLK_MCLKI: c_int = 2;
const ADAV80X_CLK_PLL1: c_int = 3;
const ADAV80X_CLK_PLL2: c_int = 4;
const ADAV80X_CLK_SYSCLK1: c_int = 5;
const ADAV80X_CLK_SYSCLK2: c_int = 6;
const ADAV80X_CLK_SYSCLK3: c_int = 7;
const ADAV80X_PLL_SRC_XTAL: c_int = 0;
const ADAV80X_PLL_SRC_XIN: c_int = 1;
const ADAV80X_PLL_SRC_MCLKI: c_int = 2;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
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
    pub connected:
        Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u64,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_uint,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub set_pll: Option<
        unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_uint) -> c_int,
    >,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub val_bits: c_uint,
    pub pad_bits: c_uint,
    pub reg_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

type snd_soc_bias_level = c_int;
const SND_SOC_BIAS_ON: snd_soc_bias_level = 0;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 2;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 3;

#[repr(C)]
struct adav80x {
    regmap: *mut regmap,
    clk_src: Adav80xClkSrc,
    sysclk: c_uint,
    pll_src: Adav80xPllSrc,
    dai_fmt: [c_uint; 2],
    rate: c_uint,
    deemph: bool,
    sysclk_pd: [bool; 3],
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_widget_name_cmp(widget: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_dapm_mutex_lock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_mutex_unlock(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_disable_pin_unlocked(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_force_enable_pin_unlocked(
        dapm: *mut snd_soc_dapm_context,
        pin: *const c_char,
    ) -> c_int;
    fn snd_soc_dapm_sync_unlocked(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const ADAV80X_REG_DEFAULTS: [reg_default; 23] = [
    reg_default { reg: ADAV80X_PLAYBACK_CTRL, def: 0x01 },
    reg_default { reg: ADAV80X_AUX_IN_CTRL, def: 0x01 },
    reg_default { reg: ADAV80X_REC_CTRL, def: 0x02 },
    reg_default { reg: ADAV80X_AUX_OUT_CTRL, def: 0x01 },
    reg_default { reg: ADAV80X_DPATH_CTRL1, def: 0xc0 },
    reg_default { reg: ADAV80X_DPATH_CTRL2, def: 0x11 },
    reg_default { reg: ADAV80X_DAC_CTRL1, def: 0x00 },
    reg_default { reg: ADAV80X_DAC_CTRL2, def: 0x00 },
    reg_default { reg: ADAV80X_DAC_CTRL3, def: 0x00 },
    reg_default { reg: ADAV80X_DAC_L_VOL, def: 0xff },
    reg_default { reg: ADAV80X_DAC_R_VOL, def: 0xff },
    reg_default { reg: ADAV80X_PGA_L_VOL, def: 0x00 },
    reg_default { reg: ADAV80X_PGA_R_VOL, def: 0x00 },
    reg_default { reg: ADAV80X_ADC_CTRL1, def: 0x00 },
    reg_default { reg: ADAV80X_ADC_CTRL2, def: 0x00 },
    reg_default { reg: ADAV80X_ADC_L_VOL, def: 0xff },
    reg_default { reg: ADAV80X_ADC_R_VOL, def: 0xff },
    reg_default { reg: ADAV80X_PLL_CTRL1, def: 0x00 },
    reg_default { reg: ADAV80X_PLL_CTRL2, def: 0x00 },
    reg_default { reg: ADAV80X_ICLK_CTRL1, def: 0x00 },
    reg_default { reg: ADAV80X_ICLK_CTRL2, def: 0x00 },
    reg_default { reg: ADAV80X_PLL_CLK_SRC, def: 0x00 },
    reg_default { reg: ADAV80X_PLL_OUTE, def: 0x00 },
];

static ADAV80X_MUX_TEXT: [*const c_char; 3] = [c"ADC".as_ptr(), c"Playback".as_ptr(), c"Aux Playback".as_ptr()];
static ADAV80X_MUX_VALUES: [c_uint; 3] = [0, 2, 3];

/*
 * ADAV80X_MUX_ENUM_DECL expands to SOC_VALUE_ENUM_DOUBLE_DECL in C.
 * The concrete enum/control/widget initializers are supplied by the ASoC macro
 * layer outside this isolated source translation.
 */
static ADAV80X_AUX_CAPTURE_ENUM: c_int = 0;
static ADAV80X_CAPTURE_ENUM: c_int = 0;
static ADAV80X_DAC_ENUM: c_int = 0;
static ADAV80X_AUX_CAPTURE_MUX_CTRL: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ADAV80X_CAPTURE_MUX_CTRL: snd_kcontrol_new = snd_kcontrol_new { _private: [] };
static ADAV80X_DAC_MUX_CTRL: snd_kcontrol_new = snd_kcontrol_new { _private: [] };

static ADAV80X_DAPM_WIDGETS: [snd_soc_dapm_widget_item; 17] = [
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
];

unsafe extern "C" fn adav80x_dapm_sysclk_check(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let clk: *const c_char;

    match (*adav80x).clk_src {
        ADAV80X_CLK_PLL1 => clk = c"PLL1".as_ptr(),
        ADAV80X_CLK_PLL2 => clk = c"PLL2".as_ptr(),
        ADAV80X_CLK_XTAL => clk = c"OSC".as_ptr(),
        _ => return 0,
    }

    (snd_soc_dapm_widget_name_cmp(source, clk) == 0) as c_int
}

unsafe extern "C" fn adav80x_dapm_pll_check(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    ((*adav80x).pll_src == ADAV80X_PLL_SRC_XTAL) as c_int
}

static ADAV80X_DAPM_ROUTES: [snd_soc_dapm_route; 29] = [
    snd_soc_dapm_route { sink: c"DAC Select".as_ptr(), control: c"ADC".as_ptr(), source: c"ADC".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DAC Select".as_ptr(), control: c"Playback".as_ptr(), source: c"AIFIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DAC Select".as_ptr(), control: c"Aux Playback".as_ptr(), source: c"AIFAUXIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: core::ptr::null(), source: c"DAC Select".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Capture Select".as_ptr(), control: c"ADC".as_ptr(), source: c"ADC".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Capture Select".as_ptr(), control: c"Playback".as_ptr(), source: c"AIFIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Capture Select".as_ptr(), control: c"Aux Playback".as_ptr(), source: c"AIFAUXIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFOUT".as_ptr(), control: core::ptr::null(), source: c"Capture Select".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Aux Capture Select".as_ptr(), control: c"ADC".as_ptr(), source: c"ADC".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Aux Capture Select".as_ptr(), control: c"Playback".as_ptr(), source: c"AIFIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Aux Capture Select".as_ptr(), control: c"Aux Playback".as_ptr(), source: c"AIFAUXIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFAUXOUT".as_ptr(), control: core::ptr::null(), source: c"Aux Capture Select".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"VOUTR".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"VOUTL".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Left PGA".as_ptr(), control: core::ptr::null(), source: c"VINL".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"Right PGA".as_ptr(), control: core::ptr::null(), source: c"VINR".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"Left PGA".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"Right PGA".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"SYSCLK".as_ptr(), control: core::ptr::null(), source: c"PLL1".as_ptr(), connected: Some(adav80x_dapm_sysclk_check) },
    snd_soc_dapm_route { sink: c"SYSCLK".as_ptr(), control: core::ptr::null(), source: c"PLL2".as_ptr(), connected: Some(adav80x_dapm_sysclk_check) },
    snd_soc_dapm_route { sink: c"SYSCLK".as_ptr(), control: core::ptr::null(), source: c"OSC".as_ptr(), connected: Some(adav80x_dapm_sysclk_check) },
    snd_soc_dapm_route { sink: c"PLL1".as_ptr(), control: core::ptr::null(), source: c"OSC".as_ptr(), connected: Some(adav80x_dapm_pll_check) },
    snd_soc_dapm_route { sink: c"PLL2".as_ptr(), control: core::ptr::null(), source: c"OSC".as_ptr(), connected: Some(adav80x_dapm_pll_check) },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFOUT".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFAUXOUT".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFIN".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"AIFAUXIN".as_ptr(), control: core::ptr::null(), source: c"SYSCLK".as_ptr(), connected: None },
];

unsafe extern "C" fn adav80x_set_deemph(component: *mut snd_soc_component) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let val: c_uint;

    if (*adav80x).deemph {
        match (*adav80x).rate {
            32000 => val = ADAV80X_DAC_CTRL2_DEEMPH_32,
            44100 => val = ADAV80X_DAC_CTRL2_DEEMPH_44,
            48000 | 64000 | 88200 | 96000 => val = ADAV80X_DAC_CTRL2_DEEMPH_48,
            _ => val = ADAV80X_DAC_CTRL2_DEEMPH_NONE,
        }
    } else {
        val = ADAV80X_DAC_CTRL2_DEEMPH_NONE;
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_DAC_CTRL2,
        ADAV80X_DAC_CTRL2_DEEMPH_MASK,
        val,
    )
}

unsafe extern "C" fn adav80x_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let deemph = (*ucontrol).value.integer.value[0] as c_uint;

    if deemph > 1 {
        return -EINVAL;
    }

    (*adav80x).deemph = deemph != 0;

    adav80x_set_deemph(component)
}

unsafe extern "C" fn adav80x_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    (*ucontrol).value.integer.value[0] = (*adav80x).deemph as i64;
    0
}

static ADAV80X_INPGA_TLV: [c_uint; 4] = [0, 0, 50, 0];
static ADAV80X_DIGITAL_TLV: [c_uint; 3] = [0, (-9563i32) as c_uint, 0];

/*
 * snd_kcontrol_new array entries are produced by ALSA SOC_* macros in C.
 * This isolated Rust translation preserves the array shape and references.
 */
static ADAV80X_CONTROLS: [snd_kcontrol_new; 7] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static mut ADAV80X_PORT_CTRL_REGS: [[c_uint; 2]; 2] = [
    [ADAV80X_REC_CTRL, ADAV80X_PLAYBACK_CTRL],
    [ADAV80X_AUX_OUT_CTRL, ADAV80X_AUX_IN_CTRL],
];

unsafe extern "C" fn adav80x_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let mut capture: c_uint = 0x00;
    let mut playback: c_uint = 0x00;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            capture |= ADAV80X_CAPTURE_MODE_MASTER;
            playback |= ADAV80X_PLAYBACK_MODE_MASTER;
        }
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            capture |= ADAV80X_CAPTURE_MODE_I2S;
            playback |= ADAV80X_PLAYBACK_MODE_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            capture |= ADAV80X_CAPTURE_MODE_LEFT_J;
            playback |= ADAV80X_PLAYBACK_MODE_LEFT_J;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            capture |= ADAV80X_CAPTURE_MODE_RIGHT_J;
            playback |= ADAV80X_PLAYBACK_MODE_RIGHT_J_24;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_PORT_CTRL_REGS[(*dai).id as usize][0],
        ADAV80X_CAPTURE_MODE_MASK | ADAV80X_CAPTURE_MODE_MASTER,
        capture,
    );
    regmap_write(
        (*adav80x).regmap,
        ADAV80X_PORT_CTRL_REGS[(*dai).id as usize][1],
        playback,
    );

    (*adav80x).dai_fmt[(*dai).id as usize] = fmt & SND_SOC_DAIFMT_FORMAT_MASK;

    0
}

unsafe extern "C" fn adav80x_set_adc_clock(
    component: *mut snd_soc_component,
    sample_rate: c_uint,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let val: c_uint;

    if sample_rate <= 48000 {
        val = ADAV80X_ADC_CTRL1_MODULATOR_128FS;
    } else {
        val = ADAV80X_ADC_CTRL1_MODULATOR_64FS;
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_ADC_CTRL1,
        ADAV80X_ADC_CTRL1_MODULATOR_MASK,
        val,
    );

    0
}

unsafe extern "C" fn adav80x_set_dac_clock(
    component: *mut snd_soc_component,
    sample_rate: c_uint,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let val: c_uint;

    if sample_rate <= 48000 {
        val = ADAV80X_DAC_CTRL2_DIV1 | ADAV80X_DAC_CTRL2_INTERPOL_256FS;
    } else {
        val = ADAV80X_DAC_CTRL2_DIV2 | ADAV80X_DAC_CTRL2_INTERPOL_128FS;
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_DAC_CTRL2,
        ADAV80X_DAC_CTRL2_DIV_MASK | ADAV80X_DAC_CTRL2_INTERPOL_MASK,
        val,
    );

    0
}

unsafe extern "C" fn adav80x_set_capture_pcm_format(
    component: *mut snd_soc_component,
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let val: c_uint;

    match params_width(params) {
        16 => val = ADAV80X_CAPTURE_WORD_LEN16,
        18 => val = ADAV80X_CAPTRUE_WORD_LEN18,
        20 => val = ADAV80X_CAPTURE_WORD_LEN20,
        24 => val = ADAV80X_CAPTURE_WORD_LEN24,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_PORT_CTRL_REGS[(*dai).id as usize][0],
        ADAV80X_CAPTURE_WORD_LEN_MASK,
        val,
    );

    0
}

unsafe extern "C" fn adav80x_set_playback_pcm_format(
    component: *mut snd_soc_component,
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let val: c_uint;

    if (*adav80x).dai_fmt[(*dai).id as usize] != SND_SOC_DAIFMT_RIGHT_J {
        return 0;
    }

    match params_width(params) {
        16 => val = ADAV80X_PLAYBACK_MODE_RIGHT_J_16,
        18 => val = ADAV80X_PLAYBACK_MODE_RIGHT_J_18,
        20 => val = ADAV80X_PLAYBACK_MODE_RIGHT_J_20,
        24 => val = ADAV80X_PLAYBACK_MODE_RIGHT_J_24,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_PORT_CTRL_REGS[(*dai).id as usize][1],
        ADAV80X_PLAYBACK_MODE_MASK,
        val,
    );

    0
}

unsafe extern "C" fn adav80x_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let rate = params_rate(params);

    if rate.wrapping_mul(256) != (*adav80x).sysclk {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        adav80x_set_playback_pcm_format(component, dai, params);
        adav80x_set_dac_clock(component, rate);
    } else {
        adav80x_set_capture_pcm_format(component, dai, params);
        adav80x_set_adc_clock(component, rate);
    }
    (*adav80x).rate = rate;
    adav80x_set_deemph(component);

    0
}

unsafe extern "C" fn adav80x_set_sysclk(
    component: *mut snd_soc_component,
    mut clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let dapm = snd_soc_component_to_dapm(component);

    if dir == SND_SOC_CLOCK_IN {
        match clk_id {
            ADAV80X_CLK_XIN | ADAV80X_CLK_XTAL | ADAV80X_CLK_MCLKI | ADAV80X_CLK_PLL1
            | ADAV80X_CLK_PLL2 => {}
            _ => return -EINVAL,
        }

        (*adav80x).sysclk = freq;

        if (*adav80x).clk_src != clk_id {
            let iclk_ctrl1: c_uint;
            let iclk_ctrl2: c_uint;

            (*adav80x).clk_src = clk_id;
            if clk_id == ADAV80X_CLK_XTAL {
                clk_id = ADAV80X_CLK_XIN;
            }

            iclk_ctrl1 = adav80x_iclk_ctrl1_dac_src(clk_id as c_uint)
                | adav80x_iclk_ctrl1_adc_src(clk_id as c_uint)
                | adav80x_iclk_ctrl1_iclk2_src(clk_id as c_uint);
            iclk_ctrl2 = adav80x_iclk_ctrl2_iclk1_src(clk_id as c_uint);

            regmap_write((*adav80x).regmap, ADAV80X_ICLK_CTRL1, iclk_ctrl1);
            regmap_write((*adav80x).regmap, ADAV80X_ICLK_CTRL2, iclk_ctrl2);

            snd_soc_dapm_sync(dapm);
        }
    } else {
        let mask: c_uint;

        match clk_id {
            ADAV80X_CLK_SYSCLK1 | ADAV80X_CLK_SYSCLK2 | ADAV80X_CLK_SYSCLK3 => {}
            _ => return -EINVAL,
        }

        clk_id -= ADAV80X_CLK_SYSCLK1;
        mask = adav80x_pll_oute_sysclkpd(clk_id as c_uint);

        if freq == 0 {
            regmap_update_bits((*adav80x).regmap, ADAV80X_PLL_OUTE, mask, mask);
            (*adav80x).sysclk_pd[clk_id as usize] = true;
        } else {
            regmap_update_bits((*adav80x).regmap, ADAV80X_PLL_OUTE, mask, 0);
            (*adav80x).sysclk_pd[clk_id as usize] = false;
        }

        snd_soc_dapm_mutex_lock(dapm);

        if (*adav80x).sysclk_pd[0] {
            snd_soc_dapm_disable_pin_unlocked(dapm, c"PLL1".as_ptr());
        } else {
            snd_soc_dapm_force_enable_pin_unlocked(dapm, c"PLL1".as_ptr());
        }

        if (*adav80x).sysclk_pd[1] || (*adav80x).sysclk_pd[2] {
            snd_soc_dapm_disable_pin_unlocked(dapm, c"PLL2".as_ptr());
        } else {
            snd_soc_dapm_force_enable_pin_unlocked(dapm, c"PLL2".as_ptr());
        }

        snd_soc_dapm_sync_unlocked(dapm);

        snd_soc_dapm_mutex_unlock(dapm);
    }

    0
}

unsafe extern "C" fn adav80x_set_pll(
    component: *mut snd_soc_component,
    pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    mut freq_out: c_uint,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let mut pll_ctrl1: c_uint = 0;
    let mut pll_ctrl2: c_uint = 0;
    let pll_src: c_uint;

    match source {
        ADAV80X_PLL_SRC_XTAL | ADAV80X_PLL_SRC_XIN | ADAV80X_PLL_SRC_MCLKI => {}
        _ => return -EINVAL,
    }

    if freq_out == 0 {
        return 0;
    }

    match freq_in {
        27000000 => {}
        54000000 => {
            if source == ADAV80X_PLL_SRC_XIN {
                pll_ctrl1 |= ADAV80X_PLL_CTRL1_PLLDIV;
            } else {
                return -EINVAL;
            }
        }
        _ => return -EINVAL,
    }

    if freq_out > 12288000 {
        pll_ctrl2 |= adav80x_pll_ctrl2_doub(pll_id as c_uint);
        freq_out /= 2;
    }

    /* freq_out = sample_rate * 256 */
    match freq_out {
        8192000 => pll_ctrl2 |= adav80x_pll_ctrl2_fs_32(pll_id as c_uint),
        11289600 => pll_ctrl2 |= adav80x_pll_ctrl2_fs_44(pll_id as c_uint),
        12288000 => pll_ctrl2 |= adav80x_pll_ctrl2_fs_48(pll_id as c_uint),
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_PLL_CTRL1,
        ADAV80X_PLL_CTRL1_PLLDIV,
        pll_ctrl1,
    );
    regmap_update_bits(
        (*adav80x).regmap,
        ADAV80X_PLL_CTRL2,
        adav80x_pll_ctrl2_pll_mask(pll_id as c_uint),
        pll_ctrl2,
    );

    if source != (*adav80x).pll_src {
        if source == ADAV80X_PLL_SRC_MCLKI {
            pll_src = adav80x_pll_clk_src_pll_mclki(pll_id as c_uint);
        } else {
            pll_src = adav80x_pll_clk_src_pll_xin(pll_id as c_uint);
        }

        regmap_update_bits(
            (*adav80x).regmap,
            ADAV80X_PLL_CLK_SRC,
            adav80x_pll_clk_src_pll_mask(pll_id as c_uint),
            pll_src,
        );

        (*adav80x).pll_src = source;

        snd_soc_dapm_sync(dapm);
    }

    0
}

unsafe extern "C" fn adav80x_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;
    let mask = ADAV80X_DAC_CTRL1_PD;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            regmap_update_bits((*adav80x).regmap, ADAV80X_DAC_CTRL1, mask, 0x00);
        }
        SND_SOC_BIAS_OFF => {
            regmap_update_bits((*adav80x).regmap, ADAV80X_DAC_CTRL1, mask, mask);
        }
        _ => {}
    }

    0
}

/* Enforce the same sample rate on all audio interfaces */
unsafe extern "C" fn adav80x_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    if snd_soc_component_active(component) == 0 || (*adav80x).rate == 0 {
        return 0;
    }

    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, (*adav80x).rate)
}

unsafe extern "C" fn adav80x_dai_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component = (*dai).component;
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    if snd_soc_component_active(component) == 0 {
        (*adav80x).rate = 0;
    }
}

static ADAV80X_SELECTABLE_FORMATS: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

static ADAV80X_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(adav80x_set_dai_fmt),
    hw_params: Some(adav80x_hw_params),
    startup: Some(adav80x_dai_startup),
    shutdown: Some(adav80x_dai_shutdown),
    auto_selectable_formats: &ADAV80X_SELECTABLE_FORMATS,
    num_auto_selectable_formats: 1,
};

const ADAV80X_PLAYBACK_RATES: u64 = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_64000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

const ADAV80X_CAPTURE_RATES: u64 = SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000;

const ADAV80X_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE;

static mut ADAV80X_DAIS: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"adav80x-hifi".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"HiFi Playback".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: ADAV80X_PLAYBACK_RATES,
            formats: ADAV80X_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"HiFi Capture".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: ADAV80X_CAPTURE_RATES,
            formats: ADAV80X_FORMATS,
        },
        ops: &ADAV80X_DAI_OPS,
    },
    snd_soc_dai_driver {
        name: c"adav80x-aux".as_ptr(),
        id: 1,
        playback: snd_soc_pcm_stream {
            stream_name: c"Aux Playback".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: ADAV80X_PLAYBACK_RATES,
            formats: ADAV80X_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Aux Capture".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: ADAV80X_CAPTURE_RATES,
            formats: ADAV80X_FORMATS,
        },
        ops: &ADAV80X_DAI_OPS,
    },
];

unsafe extern "C" fn adav80x_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    /* Force PLLs on for SYSCLK output */
    snd_soc_dapm_force_enable_pin(dapm, c"PLL1".as_ptr());
    snd_soc_dapm_force_enable_pin(dapm, c"PLL2".as_ptr());

    /* Power down S/PDIF receiver, since it is currently not supported */
    regmap_write((*adav80x).regmap, ADAV80X_PLL_OUTE, 0x20);
    /* Disable DAC zero flag */
    regmap_write((*adav80x).regmap, ADAV80X_DAC_CTRL3, 0x6);

    0
}

unsafe extern "C" fn adav80x_resume(component: *mut snd_soc_component) -> c_int {
    let adav80x = snd_soc_component_get_drvdata(component) as *mut adav80x;

    regcache_sync((*adav80x).regmap);

    0
}

static ADAV80X_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(adav80x_probe),
    resume: Some(adav80x_resume),
    set_bias_level: Some(adav80x_set_bias_level),
    set_pll: Some(adav80x_set_pll),
    set_sysclk: Some(adav80x_set_sysclk),
    controls: ADAV80X_CONTROLS.as_ptr(),
    num_controls: ADAV80X_CONTROLS.len() as c_uint,
    dapm_widgets: ADAV80X_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: ADAV80X_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: ADAV80X_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: ADAV80X_DAPM_ROUTES.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn adav80x_bus_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let adav80x: *mut adav80x;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    adav80x = devm_kzalloc(dev, core::mem::size_of::<adav80x>(), GFP_KERNEL) as *mut adav80x;
    if adav80x.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, adav80x as *mut c_void);
    (*adav80x).regmap = regmap;

    devm_snd_soc_register_component(
        dev,
        &ADAV80X_COMPONENT_DRIVER,
        ADAV80X_DAIS.as_mut_ptr(),
        ADAV80X_DAIS.len() as c_int,
    )
}

#[unsafe(no_mangle)]
pub static ADAV80X_REGMAP_CONFIG: regmap_config = regmap_config {
    val_bits: 8,
    pad_bits: 1,
    reg_bits: 7,
    max_register: ADAV80X_PLL_OUTE,
    cache_type: REGCACHE_MAPLE,
    reg_defaults: ADAV80X_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: ADAV80X_REG_DEFAULTS.len() as c_uint,
};

/*
 * EXPORT_SYMBOL_GPL(adav80x_bus_probe);
 * EXPORT_SYMBOL_GPL(adav80x_regmap_config);
 * MODULE_DESCRIPTION("ASoC ADAV80x driver");
 * MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
 * MODULE_AUTHOR("Yi Li <yi.li@analog.com>>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
