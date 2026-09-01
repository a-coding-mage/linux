// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm9081.rs  --  WM9081 ALSA SoC Audio driver
 *
 * Author: Mark Brown
 *
 * Copyright 2009-12 Wolfson Microelectronics plc
 *
 * Rust translation of soc/codecs/wm9081.c.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type Bool = bool;
type U16 = u16;
type U64 = u64;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const INT_MAX: c_int = c_int::MAX;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
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
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm9081_pdata {
    pub irq_high: c_int,
    pub irq_cmos: c_int,
    pub num_retune_configs: c_int,
    pub retune_configs: *mut wm9081_retune_mobile_setting,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm9081_retune_mobile_setting {
    pub name: *const c_char,
    pub rate: c_int,
    pub config: [c_uint; 20],
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
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
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
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_sysclk:
        Option<unsafe extern "C" fn(*mut snd_soc_component, c_int, c_int, c_uint, c_int) -> c_int>,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
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
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> Bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> Bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct i2c_driver_inner {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    static WM9081_SOFTWARE_RESET: c_uint;
    static WM9081_INTERRUPT_STATUS: c_uint;
    static WM9081_ANALOGUE_LINEOUT: c_uint;
    static WM9081_ANALOGUE_SPEAKER_PGA: c_uint;
    static WM9081_VMID_CONTROL: c_uint;
    static WM9081_BIAS_CONTROL_1: c_uint;
    static WM9081_ANALOGUE_MIXER: c_uint;
    static WM9081_ANTI_POP_CONTROL: c_uint;
    static WM9081_ANALOGUE_SPEAKER_1: c_uint;
    static WM9081_ANALOGUE_SPEAKER_2: c_uint;
    static WM9081_POWER_MANAGEMENT: c_uint;
    static WM9081_CLOCK_CONTROL_1: c_uint;
    static WM9081_CLOCK_CONTROL_2: c_uint;
    static WM9081_CLOCK_CONTROL_3: c_uint;
    static WM9081_FLL_CONTROL_1: c_uint;
    static WM9081_FLL_CONTROL_2: c_uint;
    static WM9081_FLL_CONTROL_3: c_uint;
    static WM9081_FLL_CONTROL_4: c_uint;
    static WM9081_FLL_CONTROL_5: c_uint;
    static WM9081_AUDIO_INTERFACE_1: c_uint;
    static WM9081_AUDIO_INTERFACE_2: c_uint;
    static WM9081_AUDIO_INTERFACE_3: c_uint;
    static WM9081_AUDIO_INTERFACE_4: c_uint;
    static WM9081_INTERRUPT_STATUS_MASK: c_uint;
    static WM9081_INTERRUPT_POLARITY: c_uint;
    static WM9081_INTERRUPT_CONTROL: c_uint;
    static WM9081_DAC_DIGITAL_1: c_uint;
    static WM9081_DAC_DIGITAL_2: c_uint;
    static WM9081_DRC_1: c_uint;
    static WM9081_DRC_2: c_uint;
    static WM9081_DRC_3: c_uint;
    static WM9081_DRC_4: c_uint;
    static WM9081_WRITE_SEQUENCER_1: c_uint;
    static WM9081_WRITE_SEQUENCER_2: c_uint;
    static WM9081_MW_SLAVE_1: c_uint;
    static WM9081_EQ_1: c_uint;
    static WM9081_EQ_2: c_uint;
    static WM9081_EQ_3: c_uint;
    static WM9081_EQ_4: c_uint;
    static WM9081_EQ_5: c_uint;
    static WM9081_EQ_6: c_uint;
    static WM9081_EQ_7: c_uint;
    static WM9081_EQ_8: c_uint;
    static WM9081_EQ_9: c_uint;
    static WM9081_EQ_10: c_uint;
    static WM9081_EQ_11: c_uint;
    static WM9081_EQ_12: c_uint;
    static WM9081_EQ_13: c_uint;
    static WM9081_EQ_14: c_uint;
    static WM9081_EQ_15: c_uint;
    static WM9081_EQ_16: c_uint;
    static WM9081_EQ_17: c_uint;
    static WM9081_EQ_18: c_uint;
    static WM9081_EQ_19: c_uint;
    static WM9081_EQ_20: c_uint;
    static WM9081_MAX_REGISTER: c_uint;
    static WM9081_SPK_MODE: c_uint;
    static WM9081_SPK_ENA: c_uint;
    static WM9081_SPK_INV_MUTE: c_uint;
    static WM9081_OUT_SPK_CTRL: c_uint;
    static WM9081_FLL_CLK_SRC_MASK: c_uint;
    static WM9081_FLL_CLK_REF_DIV_MASK: c_uint;
    static WM9081_FLL_CLK_REF_DIV_SHIFT: c_uint;
    static WM9081_FLL_ENA: c_uint;
    static WM9081_FLL_FRAC_MASK: c_uint;
    static WM9081_FLL_OUTDIV_SHIFT: c_uint;
    static WM9081_FLL_FRATIO_SHIFT: c_uint;
    static WM9081_FLL_N_MASK: c_uint;
    static WM9081_FLL_N_SHIFT: c_uint;
    static WM9081_FLL_GAIN_MASK: c_uint;
    static WM9081_CLK_SYS_ENA: c_uint;
    static WM9081_SYSCLK_MCLK: c_int;
    static WM9081_SYSCLK_FLL_MCLK: c_int;
    static WM9081_MCLKDIV2: c_uint;
    static WM9081_CLK_SRC_SEL: c_uint;
    static WM9081_CLK_SYS_RATE_MASK: c_uint;
    static WM9081_CLK_SYS_RATE_SHIFT: c_uint;
    static WM9081_SAMPLE_RATE_MASK: c_uint;
    static WM9081_SAMPLE_RATE_SHIFT: c_uint;
    static WM9081_AIF_WL_MASK: c_uint;
    static WM9081_BCLK_DIV_MASK: c_uint;
    static WM9081_LRCLK_RATE_MASK: c_uint;
    static WM9081_AIFDAC_TDM_MODE_MASK: c_uint;
    static WM9081_AIFDAC_TDM_MODE_SHIFT: c_uint;
    static WM9081_EQ_ENA: c_uint;
    static WM9081_DAC_MUTE: c_uint;
    static WM9081_AIFDAC_TDM_SLOT_MASK: c_uint;
    static WM9081_LINEOUTZC: c_uint;
    static WM9081_SPKPGAZC: c_uint;
    static WM9081_IRQ_POL: c_uint;
    static WM9081_IRQ_OP_CTRL: c_uint;
    static WM9081_AIF_BCLK_INV: c_uint;
    static WM9081_AIF_LRCLK_INV: c_uint;
    static WM9081_BCLK_DIR: c_uint;
    static WM9081_LRCLK_DIR: c_uint;
    static WM9081_AIF_FMT_MASK: c_uint;
    static WM9081_VMID_SEL_MASK: c_uint;
    static WM9081_STBY_BIAS_ENA: c_uint;
    static WM9081_LINEOUT_DISCH: c_uint;
    static WM9081_BIAS_SRC: c_uint;
    static WM9081_BIAS_ENA: c_uint;
    static WM9081_VMID_RAMP: c_uint;

    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBC_CFP: c_uint;
    static SND_SOC_DAIFMT_CBP_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static REGCACHE_MAPLE: c_uint;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: Bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn mdelay(ms: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> Bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! pr_err {
    ($($arg:tt)*) => {
        /* external kernel logging macro */
    };
}

macro_rules! pr_debug {
    ($($arg:tt)*) => {
        /* external kernel logging macro */
    };
}

macro_rules! dev_err {
    ($($arg:tt)*) => {
        /* external kernel logging macro */
    };
}

macro_rules! dev_dbg {
    ($($arg:tt)*) => {
        /* external kernel logging macro */
    };
}

static WM9081_REG: [reg_default; 60] = [
    reg_default { reg: 2, def: 0x00B9 },     /* R2  - Analogue Lineout */
    reg_default { reg: 3, def: 0x00B9 },     /* R3  - Analogue Speaker PGA */
    reg_default { reg: 4, def: 0x0001 },     /* R4  - VMID Control */
    reg_default { reg: 5, def: 0x0068 },     /* R5  - Bias Control 1 */
    reg_default { reg: 7, def: 0x0000 },     /* R7  - Analogue Mixer */
    reg_default { reg: 8, def: 0x0000 },     /* R8  - Anti Pop Control */
    reg_default { reg: 9, def: 0x01DB },     /* R9  - Analogue Speaker 1 */
    reg_default { reg: 10, def: 0x0018 },    /* R10 - Analogue Speaker 2 */
    reg_default { reg: 11, def: 0x0180 },    /* R11 - Power Management */
    reg_default { reg: 12, def: 0x0000 },    /* R12 - Clock Control 1 */
    reg_default { reg: 13, def: 0x0038 },    /* R13 - Clock Control 2 */
    reg_default { reg: 14, def: 0x4000 },    /* R14 - Clock Control 3 */
    reg_default { reg: 16, def: 0x0000 },    /* R16 - FLL Control 1 */
    reg_default { reg: 17, def: 0x0200 },    /* R17 - FLL Control 2 */
    reg_default { reg: 18, def: 0x0000 },    /* R18 - FLL Control 3 */
    reg_default { reg: 19, def: 0x0204 },    /* R19 - FLL Control 4 */
    reg_default { reg: 20, def: 0x0000 },    /* R20 - FLL Control 5 */
    reg_default { reg: 22, def: 0x0000 },    /* R22 - Audio Interface 1 */
    reg_default { reg: 23, def: 0x0002 },    /* R23 - Audio Interface 2 */
    reg_default { reg: 24, def: 0x0008 },    /* R24 - Audio Interface 3 */
    reg_default { reg: 25, def: 0x0022 },    /* R25 - Audio Interface 4 */
    reg_default { reg: 27, def: 0x0006 },    /* R27 - Interrupt Status Mask */
    reg_default { reg: 28, def: 0x0000 },    /* R28 - Interrupt Polarity */
    reg_default { reg: 29, def: 0x0000 },    /* R29 - Interrupt Control */
    reg_default { reg: 30, def: 0x00C0 },    /* R30 - DAC Digital 1 */
    reg_default { reg: 31, def: 0x0008 },    /* R31 - DAC Digital 2 */
    reg_default { reg: 32, def: 0x09AF },    /* R32 - DRC 1 */
    reg_default { reg: 33, def: 0x4201 },    /* R33 - DRC 2 */
    reg_default { reg: 34, def: 0x0000 },    /* R34 - DRC 3 */
    reg_default { reg: 35, def: 0x0000 },    /* R35 - DRC 4 */
    reg_default { reg: 38, def: 0x0000 },    /* R38 - Write Sequencer 1 */
    reg_default { reg: 39, def: 0x0000 },    /* R39 - Write Sequencer 2 */
    reg_default { reg: 40, def: 0x0002 },    /* R40 - MW Slave 1 */
    reg_default { reg: 42, def: 0x0000 },    /* R42 - EQ 1 */
    reg_default { reg: 43, def: 0x0000 },    /* R43 - EQ 2 */
    reg_default { reg: 44, def: 0x0FCA },    /* R44 - EQ 3 */
    reg_default { reg: 45, def: 0x0400 },    /* R45 - EQ 4 */
    reg_default { reg: 46, def: 0x00B8 },    /* R46 - EQ 5 */
    reg_default { reg: 47, def: 0x1EB5 },    /* R47 - EQ 6 */
    reg_default { reg: 48, def: 0xF145 },    /* R48 - EQ 7 */
    reg_default { reg: 49, def: 0x0B75 },    /* R49 - EQ 8 */
    reg_default { reg: 50, def: 0x01C5 },    /* R50 - EQ 9 */
    reg_default { reg: 51, def: 0x169E },    /* R51 - EQ 10 */
    reg_default { reg: 52, def: 0xF829 },    /* R52 - EQ 11 */
    reg_default { reg: 53, def: 0x07AD },    /* R53 - EQ 12 */
    reg_default { reg: 54, def: 0x1103 },    /* R54 - EQ 13 */
    reg_default { reg: 55, def: 0x1C58 },    /* R55 - EQ 14 */
    reg_default { reg: 56, def: 0xF373 },    /* R56 - EQ 15 */
    reg_default { reg: 57, def: 0x0A54 },    /* R57 - EQ 16 */
    reg_default { reg: 58, def: 0x0558 },    /* R58 - EQ 17 */
    reg_default { reg: 59, def: 0x0564 },    /* R59 - EQ 18 */
    reg_default { reg: 60, def: 0x0559 },    /* R60 - EQ 19 */
    reg_default { reg: 61, def: 0x4000 },    /* R61 - EQ 20 */
];

#[repr(C)]
struct ClkSysRate {
    ratio: c_int,
    clk_sys_rate: c_int,
}

static mut CLK_SYS_RATES: [ClkSysRate; 10] = [
    ClkSysRate { ratio: 64, clk_sys_rate: 0 },
    ClkSysRate { ratio: 128, clk_sys_rate: 1 },
    ClkSysRate { ratio: 192, clk_sys_rate: 2 },
    ClkSysRate { ratio: 256, clk_sys_rate: 3 },
    ClkSysRate { ratio: 384, clk_sys_rate: 4 },
    ClkSysRate { ratio: 512, clk_sys_rate: 5 },
    ClkSysRate { ratio: 768, clk_sys_rate: 6 },
    ClkSysRate { ratio: 1024, clk_sys_rate: 7 },
    ClkSysRate { ratio: 1408, clk_sys_rate: 8 },
    ClkSysRate { ratio: 1536, clk_sys_rate: 9 },
];

#[repr(C)]
struct SampleRate {
    rate: c_int,
    sample_rate: c_int,
}

static mut SAMPLE_RATES: [SampleRate; 11] = [
    SampleRate { rate: 8000, sample_rate: 0 },
    SampleRate { rate: 11025, sample_rate: 1 },
    SampleRate { rate: 12000, sample_rate: 2 },
    SampleRate { rate: 16000, sample_rate: 3 },
    SampleRate { rate: 22050, sample_rate: 4 },
    SampleRate { rate: 24000, sample_rate: 5 },
    SampleRate { rate: 32000, sample_rate: 6 },
    SampleRate { rate: 44100, sample_rate: 7 },
    SampleRate { rate: 48000, sample_rate: 8 },
    SampleRate { rate: 88200, sample_rate: 9 },
    SampleRate { rate: 96000, sample_rate: 10 },
];

#[repr(C)]
struct BclkDiv {
    div: c_int, /* *10 due to .5s */
    bclk_div: c_int,
}

static mut BCLK_DIVS: [BclkDiv; 21] = [
    BclkDiv { div: 10, bclk_div: 0 },
    BclkDiv { div: 15, bclk_div: 1 },
    BclkDiv { div: 20, bclk_div: 2 },
    BclkDiv { div: 30, bclk_div: 3 },
    BclkDiv { div: 40, bclk_div: 4 },
    BclkDiv { div: 50, bclk_div: 5 },
    BclkDiv { div: 55, bclk_div: 6 },
    BclkDiv { div: 60, bclk_div: 7 },
    BclkDiv { div: 80, bclk_div: 8 },
    BclkDiv { div: 100, bclk_div: 9 },
    BclkDiv { div: 110, bclk_div: 10 },
    BclkDiv { div: 120, bclk_div: 11 },
    BclkDiv { div: 160, bclk_div: 12 },
    BclkDiv { div: 200, bclk_div: 13 },
    BclkDiv { div: 220, bclk_div: 14 },
    BclkDiv { div: 240, bclk_div: 15 },
    BclkDiv { div: 250, bclk_div: 16 },
    BclkDiv { div: 300, bclk_div: 17 },
    BclkDiv { div: 320, bclk_div: 18 },
    BclkDiv { div: 440, bclk_div: 19 },
    BclkDiv { div: 480, bclk_div: 20 },
];

#[repr(C)]
struct wm9081_priv {
    regmap: *mut regmap,
    sysclk_source: c_int,
    mclk_rate: c_int,
    sysclk_rate: c_int,
    fs: c_int,
    bclk: c_int,
    master: c_int,
    fll_fref: c_int,
    fll_fout: c_int,
    tdm_width: c_int,
    pdata: wm9081_pdata,
}

unsafe extern "C" fn wm9081_volatile_register(_dev: *mut device, reg: c_uint) -> Bool {
    reg == WM9081_SOFTWARE_RESET || reg == WM9081_INTERRUPT_STATUS
}

unsafe extern "C" fn wm9081_readable_register(_dev: *mut device, reg: c_uint) -> Bool {
    matches!(
        reg,
        r if r == WM9081_SOFTWARE_RESET
            || r == WM9081_ANALOGUE_LINEOUT
            || r == WM9081_ANALOGUE_SPEAKER_PGA
            || r == WM9081_VMID_CONTROL
            || r == WM9081_BIAS_CONTROL_1
            || r == WM9081_ANALOGUE_MIXER
            || r == WM9081_ANTI_POP_CONTROL
            || r == WM9081_ANALOGUE_SPEAKER_1
            || r == WM9081_ANALOGUE_SPEAKER_2
            || r == WM9081_POWER_MANAGEMENT
            || r == WM9081_CLOCK_CONTROL_1
            || r == WM9081_CLOCK_CONTROL_2
            || r == WM9081_CLOCK_CONTROL_3
            || r == WM9081_FLL_CONTROL_1
            || r == WM9081_FLL_CONTROL_2
            || r == WM9081_FLL_CONTROL_3
            || r == WM9081_FLL_CONTROL_4
            || r == WM9081_FLL_CONTROL_5
            || r == WM9081_AUDIO_INTERFACE_1
            || r == WM9081_AUDIO_INTERFACE_2
            || r == WM9081_AUDIO_INTERFACE_3
            || r == WM9081_AUDIO_INTERFACE_4
            || r == WM9081_INTERRUPT_STATUS
            || r == WM9081_INTERRUPT_STATUS_MASK
            || r == WM9081_INTERRUPT_POLARITY
            || r == WM9081_INTERRUPT_CONTROL
            || r == WM9081_DAC_DIGITAL_1
            || r == WM9081_DAC_DIGITAL_2
            || r == WM9081_DRC_1
            || r == WM9081_DRC_2
            || r == WM9081_DRC_3
            || r == WM9081_DRC_4
            || r == WM9081_WRITE_SEQUENCER_1
            || r == WM9081_WRITE_SEQUENCER_2
            || r == WM9081_MW_SLAVE_1
            || r == WM9081_EQ_1
            || r == WM9081_EQ_2
            || r == WM9081_EQ_3
            || r == WM9081_EQ_4
            || r == WM9081_EQ_5
            || r == WM9081_EQ_6
            || r == WM9081_EQ_7
            || r == WM9081_EQ_8
            || r == WM9081_EQ_9
            || r == WM9081_EQ_10
            || r == WM9081_EQ_11
            || r == WM9081_EQ_12
            || r == WM9081_EQ_13
            || r == WM9081_EQ_14
            || r == WM9081_EQ_15
            || r == WM9081_EQ_16
            || r == WM9081_EQ_17
            || r == WM9081_EQ_18
            || r == WM9081_EQ_19
            || r == WM9081_EQ_20
    )
}

unsafe fn wm9081_reset(map: *mut regmap) -> c_int {
    regmap_write(map, WM9081_SOFTWARE_RESET, 0x9081)
}

/* TLV declarations translated from DECLARE_TLV_DB_SCALE/RANGE:
 * drc_in_tlv(-4500,75,0), drc_out_tlv(-2250,75,0),
 * drc_min_tlv(-1800,600,0), drc_max_tlv range {1200,1800,2400,3600},
 * drc_qr_tlv(1200,600,0), drc_startup_tlv(-300,50,0), eq_tlv(-1200,100,0),
 * in_tlv(-600,600,0), dac_tlv(-7200,75,1), out_tlv(-5700,100,0).
 */

static DRC_HIGH_TEXT: [*const c_char; 6] = [
    cstr!("1"),
    cstr!("1/2"),
    cstr!("1/4"),
    cstr!("1/8"),
    cstr!("1/16"),
    cstr!("0"),
];
static DRC_LOW_TEXT: [*const c_char; 5] = [cstr!("1"), cstr!("1/2"), cstr!("1/4"), cstr!("1/8"), cstr!("0")];
static DRC_ATK_TEXT: [*const c_char; 12] = [
    cstr!("181us"),
    cstr!("181us"),
    cstr!("363us"),
    cstr!("726us"),
    cstr!("1.45ms"),
    cstr!("2.9ms"),
    cstr!("5.8ms"),
    cstr!("11.6ms"),
    cstr!("23.2ms"),
    cstr!("46.4ms"),
    cstr!("92.8ms"),
    cstr!("185.6ms"),
];
static DRC_DCY_TEXT: [*const c_char; 9] = [
    cstr!("186ms"),
    cstr!("372ms"),
    cstr!("743ms"),
    cstr!("1.49s"),
    cstr!("2.97s"),
    cstr!("5.94s"),
    cstr!("11.89s"),
    cstr!("23.78s"),
    cstr!("47.56s"),
];
static DRC_QR_DCY_TEXT: [*const c_char; 3] = [cstr!("0.725ms"), cstr!("1.45ms"), cstr!("5.8ms")];
static DAC_DEEMPH_TEXT: [*const c_char; 4] = [cstr!("None"), cstr!("32kHz"), cstr!("44.1kHz"), cstr!("48kHz")];
static SPEAKER_MODE_TEXT: [*const c_char; 2] = [cstr!("Class D"), cstr!("Class AB")];

/* SOC_ENUM_SINGLE_DECL instances:
 * drc_high(WM9081_DRC_3,3,drc_high_text), drc_low(WM9081_DRC_3,0,drc_low_text),
 * drc_atk(WM9081_DRC_2,12,drc_atk_text), drc_dcy(WM9081_DRC_2,8,drc_dcy_text),
 * drc_qr_dcy(WM9081_DRC_2,4,drc_qr_dcy_text),
 * dac_deemph(WM9081_DAC_DIGITAL_2,1,dac_deemph_text),
 * speaker_mode(WM9081_ANALOGUE_SPEAKER_2,6,speaker_mode_text).
 */

unsafe extern "C" fn speaker_mode_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let reg = snd_soc_component_read(component, WM9081_ANALOGUE_SPEAKER_2);

    if (reg & WM9081_SPK_MODE) != 0 {
        (*ucontrol).value.enumerated.item[0] = 1;
    } else {
        (*ucontrol).value.enumerated.item[0] = 0;
    }
    0
}

/*
 * Stop any attempts to change speaker mode while the speaker is enabled.
 *
 * We also have some special anti-pop controls dependent on speaker
 * mode which must be changed along with the mode.
 */
unsafe extern "C" fn speaker_mode_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let reg_pwr = snd_soc_component_read(component, WM9081_POWER_MANAGEMENT);
    let mut reg2 = snd_soc_component_read(component, WM9081_ANALOGUE_SPEAKER_2);

    /* Are we changing anything? */
    if (*ucontrol).value.enumerated.item[0] == (((reg2 & WM9081_SPK_MODE) != 0) as c_uint) {
        return 0;
    }

    /* Don't try to change modes while enabled */
    if (reg_pwr & WM9081_SPK_ENA) != 0 {
        return -EINVAL;
    }

    if (*ucontrol).value.enumerated.item[0] != 0 {
        /* Class AB */
        reg2 &= !(WM9081_SPK_INV_MUTE | WM9081_OUT_SPK_CTRL);
        reg2 |= WM9081_SPK_MODE;
    } else {
        /* Class D */
        reg2 |= WM9081_SPK_INV_MUTE | WM9081_OUT_SPK_CTRL;
        reg2 &= !WM9081_SPK_MODE;
    }

    snd_soc_component_write(component, WM9081_ANALOGUE_SPEAKER_2, reg2);
    0
}

/* wm9081_snd_controls, wm9081_eq_controls and mixer are macro-generated
 * snd_kcontrol_new tables in C using SOC_SINGLE*, SOC_ENUM*, SOC_DAPM_SINGLE*.
 * They are preserved here as dependency-provided static tables.
 */
static WM9081_SND_CONTROLS: [snd_kcontrol_new; 0] = [];
static WM9081_EQ_CONTROLS: [snd_kcontrol_new; 0] = [];
static MIXER: [snd_kcontrol_new; 0] = [];

#[repr(C)]
struct _fll_div {
    fll_fratio: U16,
    fll_outdiv: U16,
    fll_clk_ref_div: U16,
    n: U16,
    k: U16,
}

/* The size in bits of the FLL divide multiplied by 10
 * to allow rounding later */
const FIXED_FLL_SIZE: U64 = ((1u64 << 16) * 10);

#[repr(C)]
struct FllFratio {
    min: c_uint,
    max: c_uint,
    fll_fratio: U16,
    ratio: c_int,
}

static mut FLL_FRATIOS: [FllFratio; 5] = [
    FllFratio { min: 0, max: 64000, fll_fratio: 4, ratio: 16 },
    FllFratio { min: 64000, max: 128000, fll_fratio: 3, ratio: 8 },
    FllFratio { min: 128000, max: 256000, fll_fratio: 2, ratio: 4 },
    FllFratio { min: 256000, max: 1000000, fll_fratio: 1, ratio: 2 },
    FllFratio { min: 1000000, max: 13500000, fll_fratio: 0, ratio: 1 },
];

unsafe fn fll_factors(fll_div: *mut _fll_div, mut fref: c_uint, fout: c_uint) -> c_int {
    let mut div: c_uint;
    let mut target: c_uint;
    let mut i: usize;

    /* Fref must be <=13.5MHz */
    div = 1;
    while (fref / div) > 13_500_000 {
        div *= 2;

        if div > 8 {
            pr_err!("Can't scale %dMHz input down to <=13.5MHz\n", fref);
            return -EINVAL;
        }
    }
    (*fll_div).fll_clk_ref_div = (div / 2) as U16;

    pr_debug!("Fref=%u Fout=%u\n", fref, fout);

    /* Apply the division for our remaining calculations */
    fref /= div;

    /* Fvco should be 90-100MHz; don't check the upper bound */
    div = 0;
    target = fout * 2;
    while target < 90_000_000 {
        div += 1;
        target *= 2;
        if div > 7 {
            pr_err!("Unable to find FLL_OUTDIV for Fout=%uHz\n", fout);
            return -EINVAL;
        }
    }
    (*fll_div).fll_outdiv = div as U16;

    pr_debug!("Fvco=%dHz\n", target);

    /* Find an appropriate FLL_FRATIO and factor it out of the target */
    i = 0;
    while i < FLL_FRATIOS.len() {
        if FLL_FRATIOS[i].min <= fref && fref <= FLL_FRATIOS[i].max {
            (*fll_div).fll_fratio = FLL_FRATIOS[i].fll_fratio;
            target /= FLL_FRATIOS[i].ratio as c_uint;
            break;
        }
        i += 1;
    }
    if i == FLL_FRATIOS.len() {
        pr_err!("Unable to find FLL_FRATIO for Fref=%uHz\n", fref);
        return -EINVAL;
    }

    let ndiv = target / fref;
    (*fll_div).n = ndiv as U16;
    let nmod = target % fref;
    pr_debug!("Nmod=%d\n", nmod);

    /* Calculate fractional part - scale up so we can round. */
    let mut kpart: U64 = FIXED_FLL_SIZE * nmod as U64;
    kpart /= fref as U64;
    let mut k: c_uint = (kpart & 0xFFFF_FFFF) as c_uint;

    if (k % 10) >= 5 {
        k += 5;
    }

    /* Move down to proper range now rounding is done */
    (*fll_div).k = (k / 10) as U16;

    pr_debug!(
        "N=%x K=%x FLL_FRATIO=%x FLL_OUTDIV=%x FLL_CLK_REF_DIV=%x\n",
        (*fll_div).n,
        (*fll_div).k,
        (*fll_div).fll_fratio,
        (*fll_div).fll_outdiv,
        (*fll_div).fll_clk_ref_div
    );

    0
}

unsafe fn wm9081_set_fll(
    component: *mut snd_soc_component,
    fll_id: c_int,
    fref: c_uint,
    fout: c_uint,
) -> c_int {
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let mut fll_div = _fll_div {
        fll_fratio: 0,
        fll_outdiv: 0,
        fll_clk_ref_div: 0,
        n: 0,
        k: 0,
    };

    /* Any change? */
    if fref as c_int == (*wm9081).fll_fref && fout as c_int == (*wm9081).fll_fout {
        return 0;
    }

    /* Disable the FLL */
    if fout == 0 {
        dev_dbg!((*component).dev, "FLL disabled\n");
        (*wm9081).fll_fref = 0;
        (*wm9081).fll_fout = 0;
        return 0;
    }

    let ret = fll_factors(&mut fll_div, fref, fout);
    if ret != 0 {
        return ret;
    }

    let mut reg5 = snd_soc_component_read(component, WM9081_FLL_CONTROL_5) as U16;
    reg5 &= !(WM9081_FLL_CLK_SRC_MASK as U16);

    if fll_id == WM9081_SYSCLK_FLL_MCLK {
        reg5 |= 0x1;
    } else {
        dev_err!((*component).dev, "Unknown FLL ID %d\n", fll_id);
        return -EINVAL;
    }

    /* Disable CLK_SYS while we reconfigure */
    let clk_sys_reg = snd_soc_component_read(component, WM9081_CLOCK_CONTROL_3) as c_int;
    if (clk_sys_reg as c_uint & WM9081_CLK_SYS_ENA) != 0 {
        snd_soc_component_write(
            component,
            WM9081_CLOCK_CONTROL_3,
            (clk_sys_reg as c_uint) & !WM9081_CLK_SYS_ENA,
        );
    }

    /* Any FLL configuration change requires that the FLL be
     * disabled first. */
    let mut reg1 = snd_soc_component_read(component, WM9081_FLL_CONTROL_1) as U16;
    reg1 &= !(WM9081_FLL_ENA as U16);
    snd_soc_component_write(component, WM9081_FLL_CONTROL_1, reg1 as c_uint);

    /* Apply the configuration */
    if fll_div.k != 0 {
        reg1 |= WM9081_FLL_FRAC_MASK as U16;
    } else {
        reg1 &= !(WM9081_FLL_FRAC_MASK as U16);
    }
    snd_soc_component_write(component, WM9081_FLL_CONTROL_1, reg1 as c_uint);

    snd_soc_component_write(
        component,
        WM9081_FLL_CONTROL_2,
        ((fll_div.fll_outdiv as c_uint) << WM9081_FLL_OUTDIV_SHIFT)
            | ((fll_div.fll_fratio as c_uint) << WM9081_FLL_FRATIO_SHIFT),
    );
    snd_soc_component_write(component, WM9081_FLL_CONTROL_3, fll_div.k as c_uint);

    let mut reg4 = snd_soc_component_read(component, WM9081_FLL_CONTROL_4) as U16;
    reg4 &= !(WM9081_FLL_N_MASK as U16);
    reg4 |= ((fll_div.n as c_uint) << WM9081_FLL_N_SHIFT) as U16;
    snd_soc_component_write(component, WM9081_FLL_CONTROL_4, reg4 as c_uint);

    reg5 &= !(WM9081_FLL_CLK_REF_DIV_MASK as U16);
    reg5 |= ((fll_div.fll_clk_ref_div as c_uint) << WM9081_FLL_CLK_REF_DIV_SHIFT) as U16;
    snd_soc_component_write(component, WM9081_FLL_CONTROL_5, reg5 as c_uint);

    /* Set gain to the recommended value */
    snd_soc_component_update_bits(component, WM9081_FLL_CONTROL_4, WM9081_FLL_GAIN_MASK, 0);

    /* Enable the FLL */
    snd_soc_component_write(component, WM9081_FLL_CONTROL_1, reg1 as c_uint | WM9081_FLL_ENA);

    /* Then bring CLK_SYS up again if it was disabled */
    if (clk_sys_reg as c_uint & WM9081_CLK_SYS_ENA) != 0 {
        snd_soc_component_write(component, WM9081_CLOCK_CONTROL_3, clk_sys_reg as c_uint);
    }

    dev_dbg!((*component).dev, "FLL enabled at %dHz->%dHz\n", fref, fout);

    (*wm9081).fll_fref = fref as c_int;
    (*wm9081).fll_fout = fout as c_int;

    0
}

unsafe fn configure_clock(component: *mut snd_soc_component) -> c_int {
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let mut new_sysclk: c_int;
    let mut target: c_int;
    let mut i: usize;
    let mut ret: c_int = 0;
    let mut mclkdiv: c_int = 0;
    let mut fll: c_int = 0;

    if (*wm9081).sysclk_source == WM9081_SYSCLK_MCLK {
        if (*wm9081).mclk_rate > 12_225_000 {
            mclkdiv = 1;
            (*wm9081).sysclk_rate = (*wm9081).mclk_rate / 2;
        } else {
            (*wm9081).sysclk_rate = (*wm9081).mclk_rate;
        }
        wm9081_set_fll(component, WM9081_SYSCLK_FLL_MCLK, 0, 0);
    } else if (*wm9081).sysclk_source == WM9081_SYSCLK_FLL_MCLK {
        /* If we have a sample rate calculate a CLK_SYS that
         * gives us a suitable DAC configuration, plus BCLK.
         * Ideally we would check to see if we can clock
         * directly from MCLK and only use the FLL if this is
         * not the case, though care must be taken with free
         * running mode.
         */
        if (*wm9081).master != 0 && (*wm9081).bclk != 0 {
            /* Make sure we can generate CLK_SYS and BCLK
             * and that we've got 3MHz for optimal
             * performance. */
            i = 0;
            loop {
                if i >= CLK_SYS_RATES.len() {
                    return -EINVAL;
                }
                target = (*wm9081).fs * CLK_SYS_RATES[i].ratio;
                new_sysclk = target;
                if target >= (*wm9081).bclk && target > 3_000_000 {
                    break;
                }
                i += 1;
            }
        } else if (*wm9081).fs != 0 {
            i = 0;
            loop {
                if i >= CLK_SYS_RATES.len() {
                    return -EINVAL;
                }
                new_sysclk = CLK_SYS_RATES[i].ratio * (*wm9081).fs;
                if new_sysclk > 3_000_000 {
                    break;
                }
                i += 1;
            }
        } else {
            new_sysclk = 12_288_000;
        }

        ret = wm9081_set_fll(
            component,
            WM9081_SYSCLK_FLL_MCLK,
            (*wm9081).mclk_rate as c_uint,
            new_sysclk as c_uint,
        );
        if ret == 0 {
            (*wm9081).sysclk_rate = new_sysclk;
            /* Switch SYSCLK over to FLL */
            fll = 1;
        } else {
            (*wm9081).sysclk_rate = (*wm9081).mclk_rate;
        }
    } else {
        return -EINVAL;
    }

    let mut reg = snd_soc_component_read(component, WM9081_CLOCK_CONTROL_1);
    if mclkdiv != 0 {
        reg |= WM9081_MCLKDIV2;
    } else {
        reg &= !WM9081_MCLKDIV2;
    }
    snd_soc_component_write(component, WM9081_CLOCK_CONTROL_1, reg);

    reg = snd_soc_component_read(component, WM9081_CLOCK_CONTROL_3);
    if fll != 0 {
        reg |= WM9081_CLK_SRC_SEL;
    } else {
        reg &= !WM9081_CLK_SRC_SEL;
    }
    snd_soc_component_write(component, WM9081_CLOCK_CONTROL_3, reg);

    dev_dbg!((*component).dev, "CLK_SYS is %dHz\n", (*wm9081).sysclk_rate);

    ret
}

unsafe extern "C" fn clk_sys_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;

    /* This should be done on init() for bypass paths */
    if (*wm9081).sysclk_source == WM9081_SYSCLK_MCLK {
        dev_dbg!((*component).dev, "Using %dHz MCLK\n", (*wm9081).mclk_rate);
    } else if (*wm9081).sysclk_source == WM9081_SYSCLK_FLL_MCLK {
        dev_dbg!(
            (*component).dev,
            "Using %dHz MCLK with FLL\n",
            (*wm9081).mclk_rate
        );
    } else {
        dev_err!((*component).dev, "System clock not configured\n");
        return -EINVAL;
    }

    if event == SND_SOC_DAPM_PRE_PMU {
        configure_clock(component);
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* Disable the FLL if it's running */
        wm9081_set_fll(component, 0, 0, 0);
    }

    0
}

/* wm9081_dapm_widgets is a macro-generated snd_soc_dapm_widget table:
 * INPUT IN1/IN2, DAC DAC, Mixer, LINEOUT PGA, Speaker PGA, Speaker,
 * OUTPUT LINEOUT/SPKN/SPKP, supplies CLK_SYS/CLK_DSP/TOCLK/TSENSE.
 */
static WM9081_DAPM_WIDGETS: [snd_soc_dapm_widget_desc; 0] = [];

static WM9081_AUDIO_PATHS: [snd_soc_dapm_route; 18] = [
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("CLK_SYS") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("CLK_DSP") },
    snd_soc_dapm_route { sink: cstr!("DAC"), control: ptr::null(), source: cstr!("AIF") },
    snd_soc_dapm_route { sink: cstr!("Mixer"), control: cstr!("IN1 Switch"), source: cstr!("IN1") },
    snd_soc_dapm_route { sink: cstr!("Mixer"), control: cstr!("IN2 Switch"), source: cstr!("IN2") },
    snd_soc_dapm_route { sink: cstr!("Mixer"), control: cstr!("Playback Switch"), source: cstr!("DAC") },
    snd_soc_dapm_route { sink: cstr!("LINEOUT PGA"), control: ptr::null(), source: cstr!("Mixer") },
    snd_soc_dapm_route { sink: cstr!("LINEOUT PGA"), control: ptr::null(), source: cstr!("TOCLK") },
    snd_soc_dapm_route { sink: cstr!("LINEOUT PGA"), control: ptr::null(), source: cstr!("CLK_SYS") },
    snd_soc_dapm_route { sink: cstr!("LINEOUT"), control: ptr::null(), source: cstr!("LINEOUT PGA") },
    snd_soc_dapm_route { sink: cstr!("Speaker PGA"), control: ptr::null(), source: cstr!("Mixer") },
    snd_soc_dapm_route { sink: cstr!("Speaker PGA"), control: ptr::null(), source: cstr!("TOCLK") },
    snd_soc_dapm_route { sink: cstr!("Speaker PGA"), control: ptr::null(), source: cstr!("CLK_SYS") },
    snd_soc_dapm_route { sink: cstr!("Speaker"), control: ptr::null(), source: cstr!("Speaker PGA") },
    snd_soc_dapm_route { sink: cstr!("Speaker"), control: ptr::null(), source: cstr!("TSENSE") },
    snd_soc_dapm_route { sink: cstr!("SPKN"), control: ptr::null(), source: cstr!("Speaker") },
    snd_soc_dapm_route { sink: cstr!("SPKP"), control: ptr::null(), source: cstr!("Speaker") },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn wm9081_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let dapm = snd_soc_component_to_dapm(component);

    if level == SND_SOC_BIAS_ON {
    } else if level == SND_SOC_BIAS_PREPARE {
        /* VMID=2*40k */
        snd_soc_component_update_bits(component, WM9081_VMID_CONTROL, WM9081_VMID_SEL_MASK, 0x2);

        /* Normal bias current */
        snd_soc_component_update_bits(component, WM9081_BIAS_CONTROL_1, WM9081_STBY_BIAS_ENA, 0);
    } else if level == SND_SOC_BIAS_STANDBY {
        /* Initial cold start */
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            regcache_cache_only((*wm9081).regmap, false);
            regcache_sync((*wm9081).regmap);

            /* Disable LINEOUT discharge */
            snd_soc_component_update_bits(component, WM9081_ANTI_POP_CONTROL, WM9081_LINEOUT_DISCH, 0);

            /* Select startup bias source */
            snd_soc_component_update_bits(
                component,
                WM9081_BIAS_CONTROL_1,
                WM9081_BIAS_SRC | WM9081_BIAS_ENA,
                WM9081_BIAS_SRC | WM9081_BIAS_ENA,
            );

            /* VMID 2*4k; Soft VMID ramp enable */
            snd_soc_component_update_bits(
                component,
                WM9081_VMID_CONTROL,
                WM9081_VMID_RAMP | WM9081_VMID_SEL_MASK,
                WM9081_VMID_RAMP | 0x6,
            );

            mdelay(100);

            /* Normal bias enable & soft start off */
            snd_soc_component_update_bits(component, WM9081_VMID_CONTROL, WM9081_VMID_RAMP, 0);

            /* Standard bias source */
            snd_soc_component_update_bits(component, WM9081_BIAS_CONTROL_1, WM9081_BIAS_SRC, 0);
        }

        /* VMID 2*240k */
        snd_soc_component_update_bits(component, WM9081_VMID_CONTROL, WM9081_VMID_SEL_MASK, 0x04);

        /* Standby bias current on */
        snd_soc_component_update_bits(
            component,
            WM9081_BIAS_CONTROL_1,
            WM9081_STBY_BIAS_ENA,
            WM9081_STBY_BIAS_ENA,
        );
    } else if level == SND_SOC_BIAS_OFF {
        /* Startup bias source and disable bias */
        snd_soc_component_update_bits(
            component,
            WM9081_BIAS_CONTROL_1,
            WM9081_BIAS_SRC | WM9081_BIAS_ENA,
            WM9081_BIAS_SRC,
        );

        /* Disable VMID with soft ramping */
        snd_soc_component_update_bits(
            component,
            WM9081_VMID_CONTROL,
            WM9081_VMID_RAMP | WM9081_VMID_SEL_MASK,
            WM9081_VMID_RAMP,
        );

        /* Actively discharge LINEOUT */
        snd_soc_component_update_bits(
            component,
            WM9081_ANTI_POP_CONTROL,
            WM9081_LINEOUT_DISCH,
            WM9081_LINEOUT_DISCH,
        );

        regcache_cache_only((*wm9081).regmap, true);
    }

    0
}

unsafe extern "C" fn wm9081_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let mut aif2 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_2);

    aif2 &= !(WM9081_AIF_BCLK_INV
        | WM9081_AIF_LRCLK_INV
        | WM9081_BCLK_DIR
        | WM9081_LRCLK_DIR
        | WM9081_AIF_FMT_MASK);

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => (*wm9081).master = 0,
        x if x == SND_SOC_DAIFMT_CBC_CFP => {
            aif2 |= WM9081_LRCLK_DIR;
            (*wm9081).master = 1;
        }
        x if x == SND_SOC_DAIFMT_CBP_CFC => {
            aif2 |= WM9081_BCLK_DIR;
            (*wm9081).master = 1;
        }
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            aif2 |= WM9081_LRCLK_DIR | WM9081_BCLK_DIR;
            (*wm9081).master = 1;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_B => {
            aif2 |= WM9081_AIF_LRCLK_INV;
            aif2 |= 0x3;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => aif2 |= 0x3,
        x if x == SND_SOC_DAIFMT_I2S => aif2 |= 0x2,
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => aif2 |= 0x1,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
            /* frame inversion not valid for DSP modes */
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                y if y == SND_SOC_DAIFMT_NB_NF => {}
                y if y == SND_SOC_DAIFMT_IB_NF => aif2 |= WM9081_AIF_BCLK_INV,
                _ => return -EINVAL,
            }
        }
        x if x == SND_SOC_DAIFMT_I2S || x == SND_SOC_DAIFMT_RIGHT_J || x == SND_SOC_DAIFMT_LEFT_J => {
            match fmt & SND_SOC_DAIFMT_INV_MASK {
                y if y == SND_SOC_DAIFMT_NB_NF => {}
                y if y == SND_SOC_DAIFMT_IB_IF => aif2 |= WM9081_AIF_BCLK_INV | WM9081_AIF_LRCLK_INV,
                y if y == SND_SOC_DAIFMT_IB_NF => aif2 |= WM9081_AIF_BCLK_INV,
                y if y == SND_SOC_DAIFMT_NB_IF => aif2 |= WM9081_AIF_LRCLK_INV,
                _ => return -EINVAL,
            }
        }
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM9081_AUDIO_INTERFACE_2, aif2);
    0
}

unsafe extern "C" fn wm9081_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let mut clk_ctrl2 = snd_soc_component_read(component, WM9081_CLOCK_CONTROL_2);
    clk_ctrl2 &= !(WM9081_CLK_SYS_RATE_MASK | WM9081_SAMPLE_RATE_MASK);

    let aif1 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_1);

    let mut aif2 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_2);
    aif2 &= !WM9081_AIF_WL_MASK;

    let mut aif3 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_3);
    aif3 &= !WM9081_BCLK_DIV_MASK;

    let mut aif4 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_4);
    aif4 &= !WM9081_LRCLK_RATE_MASK;

    (*wm9081).fs = params_rate(params);

    if (*wm9081).tdm_width != 0 {
        /* If TDM is set up then that fixes our BCLK. */
        let slots =
            (((aif1 & WM9081_AIFDAC_TDM_MODE_MASK) >> WM9081_AIFDAC_TDM_MODE_SHIFT) + 1) as c_int;
        (*wm9081).bclk = (*wm9081).fs * (*wm9081).tdm_width * slots;
    } else {
        /* Otherwise work out a BCLK from the sample size */
        (*wm9081).bclk = 2 * (*wm9081).fs;

        match params_width(params) {
            16 => (*wm9081).bclk *= 16,
            20 => {
                (*wm9081).bclk *= 20;
                aif2 |= 0x4;
            }
            24 => {
                (*wm9081).bclk *= 24;
                aif2 |= 0x8;
            }
            32 => {
                (*wm9081).bclk *= 32;
                aif2 |= 0xc;
            }
            _ => return -EINVAL,
        }
    }

    dev_dbg!((*component).dev, "Target BCLK is %dHz\n", (*wm9081).bclk);

    let ret = configure_clock(component);
    if ret != 0 {
        return ret;
    }

    /* Select nearest CLK_SYS_RATE */
    let mut best: usize = 0;
    let mut best_val = (((*wm9081).sysclk_rate / CLK_SYS_RATES[0].ratio) - (*wm9081).fs).abs();
    let mut i = 1usize;
    while i < CLK_SYS_RATES.len() {
        let cur_val = (((*wm9081).sysclk_rate / CLK_SYS_RATES[i].ratio) - (*wm9081).fs).abs();
        if cur_val < best_val {
            best = i;
            best_val = cur_val;
        }
        i += 1;
    }
    dev_dbg!((*component).dev, "Selected CLK_SYS_RATIO of %d\n", CLK_SYS_RATES[best].ratio);
    clk_ctrl2 |= (CLK_SYS_RATES[best].clk_sys_rate as c_uint) << WM9081_CLK_SYS_RATE_SHIFT;

    /* SAMPLE_RATE */
    best = 0;
    best_val = ((*wm9081).fs - SAMPLE_RATES[0].rate).abs();
    i = 1;
    while i < SAMPLE_RATES.len() {
        /* Closest match */
        let cur_val = ((*wm9081).fs - SAMPLE_RATES[i].rate).abs();
        if cur_val < best_val {
            best = i;
            best_val = cur_val;
        }
        i += 1;
    }
    dev_dbg!((*component).dev, "Selected SAMPLE_RATE of %dHz\n", SAMPLE_RATES[best].rate);
    clk_ctrl2 |= (SAMPLE_RATES[best].sample_rate as c_uint) << WM9081_SAMPLE_RATE_SHIFT;

    /* BCLK_DIV */
    best = 0;
    best_val = INT_MAX;
    i = 0;
    while i < BCLK_DIVS.len() {
        let cur_val = (((*wm9081).sysclk_rate * 10) / BCLK_DIVS[i].div) - (*wm9081).bclk;
        if cur_val < 0 {
            break; /* Table is sorted */
        }
        if cur_val < best_val {
            best = i;
            best_val = cur_val;
        }
        i += 1;
    }
    (*wm9081).bclk = ((*wm9081).sysclk_rate * 10) / BCLK_DIVS[best].div;
    dev_dbg!(
        (*component).dev,
        "Selected BCLK_DIV of %d for %dHz BCLK\n",
        BCLK_DIVS[best].div,
        (*wm9081).bclk
    );
    aif3 |= BCLK_DIVS[best].bclk_div as c_uint;

    /* LRCLK is a simple fraction of BCLK */
    dev_dbg!((*component).dev, "LRCLK_RATE is %d\n", (*wm9081).bclk / (*wm9081).fs);
    aif4 |= ((*wm9081).bclk / (*wm9081).fs) as c_uint;

    /* Apply a ReTune Mobile configuration if it's in use */
    if (*wm9081).pdata.num_retune_configs != 0 {
        let pdata = &mut (*wm9081).pdata as *mut wm9081_pdata;
        best = 0;
        best_val = ((*(*pdata).retune_configs.add(0)).rate - (*wm9081).fs).abs();
        i = 0;
        while i < (*pdata).num_retune_configs as usize {
            let cur_val = ((*(*pdata).retune_configs.add(i)).rate - (*wm9081).fs).abs();
            if cur_val < best_val {
                best_val = cur_val;
                best = i;
            }
            i += 1;
        }
        let s = (*pdata).retune_configs.add(best);

        dev_dbg!(
            (*component).dev,
            "ReTune Mobile %s tuned for %dHz\n",
            (*s).name,
            (*s).rate
        );

        /* If the EQ is enabled then disable it while we write out */
        let mut eq1 = snd_soc_component_read(component, WM9081_EQ_1) & WM9081_EQ_ENA;
        if (eq1 & WM9081_EQ_ENA) != 0 {
            snd_soc_component_write(component, WM9081_EQ_1, 0);
        }

        /* Write out the other values */
        i = 1;
        while i < (*s).config.len() {
            snd_soc_component_write(component, WM9081_EQ_1 + i as c_uint, (*s).config[i]);
            i += 1;
        }

        eq1 |= (*s).config[0] & !WM9081_EQ_ENA;
        snd_soc_component_write(component, WM9081_EQ_1, eq1);
    }

    snd_soc_component_write(component, WM9081_CLOCK_CONTROL_2, clk_ctrl2);
    snd_soc_component_write(component, WM9081_AUDIO_INTERFACE_2, aif2);
    snd_soc_component_write(component, WM9081_AUDIO_INTERFACE_3, aif3);
    snd_soc_component_write(component, WM9081_AUDIO_INTERFACE_4, aif4);

    0
}

unsafe extern "C" fn wm9081_mute(codec_dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*codec_dai).component;
    let mut reg = snd_soc_component_read(component, WM9081_DAC_DIGITAL_2);

    if mute != 0 {
        reg |= WM9081_DAC_MUTE;
    } else {
        reg &= !WM9081_DAC_MUTE;
    }

    snd_soc_component_write(component, WM9081_DAC_DIGITAL_2, reg);
    0
}

unsafe extern "C" fn wm9081_set_sysclk(
    component: *mut snd_soc_component,
    clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;

    if clk_id == WM9081_SYSCLK_MCLK || clk_id == WM9081_SYSCLK_FLL_MCLK {
        (*wm9081).sysclk_source = clk_id;
        (*wm9081).mclk_rate = freq as c_int;
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn wm9081_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    rx_mask: c_uint,
    mut slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;
    let mut aif1 = snd_soc_component_read(component, WM9081_AUDIO_INTERFACE_1);

    aif1 &= !(WM9081_AIFDAC_TDM_SLOT_MASK | WM9081_AIFDAC_TDM_MODE_MASK);

    if slots < 0 || slots > 4 {
        return -EINVAL;
    }

    (*wm9081).tdm_width = slot_width;

    if slots == 0 {
        slots = 1;
    }

    aif1 |= ((slots - 1) as c_uint) << WM9081_AIFDAC_TDM_MODE_SHIFT;

    match rx_mask {
        1 => {}
        2 => aif1 |= 0x10,
        4 => aif1 |= 0x20,
        8 => aif1 |= 0x30,
        _ => return -EINVAL,
    }

    snd_soc_component_write(component, WM9081_AUDIO_INTERFACE_1, aif1);
    0
}

unsafe fn wm9081_rates() -> c_uint {
    SNDRV_PCM_RATE_8000_96000
}

unsafe fn wm9081_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static WM9081_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm9081_hw_params),
    set_fmt: Some(wm9081_set_dai_fmt),
    mute_stream: Some(wm9081_mute),
    set_tdm_slot: Some(wm9081_set_tdm_slot),
    no_capture_mute: 1,
};

/* We report two channels because the CODEC processes a stereo signal, even
 * though it is only capable of handling a mono output.
 */
static mut WM9081_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: cstr!("wm9081-hifi"),
    playback: snd_soc_pcm_stream {
        stream_name: cstr!("AIF"),
        channels_min: 1,
        channels_max: 2,
        rates: 0,   /* WM9081_RATES: SNDRV_PCM_RATE_8000_96000 */
        formats: 0, /* WM9081_FORMATS: S16_LE | S20_3LE | S24_LE | S32_LE */
    },
    ops: &WM9081_DAI_OPS,
};

unsafe extern "C" fn wm9081_probe(component: *mut snd_soc_component) -> c_int {
    let wm9081 = snd_soc_component_get_drvdata(component) as *mut wm9081_priv;

    /* Enable zero cross by default */
    snd_soc_component_update_bits(
        component,
        WM9081_ANALOGUE_LINEOUT,
        WM9081_LINEOUTZC,
        WM9081_LINEOUTZC,
    );
    snd_soc_component_update_bits(
        component,
        WM9081_ANALOGUE_SPEAKER_PGA,
        WM9081_SPKPGAZC,
        WM9081_SPKPGAZC,
    );

    if (*wm9081).pdata.num_retune_configs == 0 {
        dev_dbg!((*component).dev, "No ReTune Mobile data, using normal EQ\n");
        snd_soc_add_component_controls(
            component,
            WM9081_EQ_CONTROLS.as_ptr(),
            WM9081_EQ_CONTROLS.len() as c_uint,
        );
    }

    0
}

static SOC_COMPONENT_DEV_WM9081: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(wm9081_probe),
    set_sysclk: Some(wm9081_set_sysclk),
    set_bias_level: Some(wm9081_set_bias_level),
    controls: WM9081_SND_CONTROLS.as_ptr(),
    num_controls: WM9081_SND_CONTROLS.len() as c_uint,
    dapm_widgets: WM9081_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: WM9081_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: WM9081_AUDIO_PATHS.as_ptr(),
    num_dapm_routes: WM9081_AUDIO_PATHS.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

static WM9081_REGMAP: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    max_register: 0, /* WM9081_MAX_REGISTER */
    reg_defaults: WM9081_REG.as_ptr(),
    num_reg_defaults: WM9081_REG.len() as c_uint,
    volatile_reg: Some(wm9081_volatile_register),
    readable_reg: Some(wm9081_readable_register),
    cache_type: 0, /* REGCACHE_MAPLE */
};

unsafe extern "C" fn wm9081_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let wm9081 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<wm9081_priv>(),
        GFP_KERNEL,
    ) as *mut wm9081_priv;
    if wm9081.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, wm9081 as *mut c_void);

    (*wm9081).regmap = devm_regmap_init_i2c(i2c, &WM9081_REGMAP);
    if IS_ERR((*wm9081).regmap as *const c_void) {
        let ret = PTR_ERR((*wm9081).regmap as *const c_void);
        dev_err!(&mut (*i2c).dev, "regmap_init() failed: %d\n", ret);
        return ret;
    }

    let mut reg: c_uint = 0;
    let mut ret = regmap_read((*wm9081).regmap, WM9081_SOFTWARE_RESET, &mut reg);
    if ret != 0 {
        dev_err!(&mut (*i2c).dev, "Failed to read chip ID: %d\n", ret);
        return ret;
    }
    if reg != 0x9081 {
        dev_err!(&mut (*i2c).dev, "Device is not a WM9081: ID=0x%x\n", reg);
        return -EINVAL;
    }

    ret = wm9081_reset((*wm9081).regmap);
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to issue reset\n");
        return ret;
    }

    if !dev_get_platdata(&mut (*i2c).dev).is_null() {
        memcpy(
            &mut (*wm9081).pdata as *mut _ as *mut c_void,
            dev_get_platdata(&mut (*i2c).dev) as *const c_void,
            core::mem::size_of_val(&(*wm9081).pdata),
        );
    }

    reg = 0;
    if (*wm9081).pdata.irq_high != 0 {
        reg |= WM9081_IRQ_POL;
    }
    if (*wm9081).pdata.irq_cmos == 0 {
        reg |= WM9081_IRQ_OP_CTRL;
    }
    regmap_update_bits(
        (*wm9081).regmap,
        WM9081_INTERRUPT_CONTROL,
        WM9081_IRQ_POL | WM9081_IRQ_OP_CTRL,
        reg,
    );

    regcache_cache_only((*wm9081).regmap, true);

    /* In C, WM9081_DAI playback.rates/formats are compile-time macro values. */
    WM9081_DAI.playback.rates = wm9081_rates();
    WM9081_DAI.playback.formats = wm9081_formats();

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &SOC_COMPONENT_DEV_WM9081,
        &mut WM9081_DAI,
        1,
    );
    if ret < 0 {
        return ret;
    }

    0
}

static WM9081_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'w' as c_char,
            b'm' as c_char,
            b'9' as c_char,
            b'0' as c_char,
            b'8' as c_char,
            b'1' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, wm9081_i2c_id); */

static WM9081_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner { name: cstr!("wm9081") },
    probe: Some(wm9081_i2c_probe),
    id_table: WM9081_I2C_ID.as_ptr(),
};

/* module_i2c_driver(wm9081_i2c_driver); */
/* MODULE_DESCRIPTION("ASoC WM9081 driver"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
