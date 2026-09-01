// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 LAPIS Semiconductor Co., Ltd.
 */

/* Translated from Linux C source. Includes from linux/*, sound/*, and
 * "ml26124.h" are external dependencies supplied by the target repository.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 1;

const DVOL_CTL_DVMUTE_ON: u32 = BIT(4); /* Digital volume MUTE On */
const DVOL_CTL_DVMUTE_OFF: u32 = 0; /* Digital volume MUTE Off */
const ML26124_SAI_NO_DELAY: u32 = BIT(1);
const ML26124_SAI_FRAME_SYNC: u32 = BIT(5) | BIT(0); /* For mono (Telecodec) */
const ML26134_CACHESIZE: u32 = 212;
const ML26124_VMID: u32 = BIT(1);
const ML26124_RATES: u32 = SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;
const ML26124_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;
const ML26124_NUM_REGISTER: u32 = ML26134_CACHESIZE;

/* External constants from linux/sound headers and ml26124.h. */
extern "C" {
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_32000: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    static SND_SOC_NOPM: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;

    static ML26124_SAI_TRANS_CTL: c_uint;
    static ML26124_SAI_RCV_CTL: c_uint;
    static ML26124_RECORD_DIG_VOL: c_uint;
    static ML26124_PLBAK_DIG_VOL: c_uint;
    static ML26124_DIGI_BOOST_VOL: c_uint;
    static ML26124_EQ_GAIN_BRAND0: c_uint;
    static ML26124_EQ_GAIN_BRAND1: c_uint;
    static ML26124_EQ_GAIN_BRAND2: c_uint;
    static ML26124_EQ_GAIN_BRAND3: c_uint;
    static ML26124_EQ_GAIN_BRAND4: c_uint;
    static ML26124_ALC_TARGET_LEV: c_uint;
    static ML26124_ALC_MAXMIN_GAIN: c_uint;
    static ML26124_PL_MAXMIN_GAIN: c_uint;
    static ML26124_PLYBAK_BOST_VOL: c_uint;
    static ML26124_FILTER_EN: c_uint;
    static ML26124_PW_ZCCMP_PW_MNG: c_uint;
    static ML26124_DVOL_CTL: c_uint;
    static ML26124_SPK_AMP_OUT: c_uint;
    static ML26124_MIC_IF_CTL: c_uint;
    static ML26124_PW_LOUT_PW_MNG: c_uint;
    static ML26124_CLK_EN: c_uint;
    static ML26124_PW_REF_PW_MNG: c_uint;
    static ML26124_PW_DAC_PW_MNG: c_uint;
    static ML26124_PW_IN_PW_MNG: c_uint;
    static ML26124_CLK_CTL: c_uint;
    static ML26124_SMPLING_RATE: c_uint;
    static ML26124_PLLNL: c_uint;
    static ML26124_PLLNH: c_uint;
    static ML26124_PLLML: c_uint;
    static ML26124_PLLMH: c_uint;
    static ML26124_PLLDIV: c_uint;
    static ML26124_REC_PLYBAK_RUN: c_uint;
    static ML26124_SAI_MODE_SEL: c_uint;
    static ML26124_USE_PLLOUT: c_int;
    static ML26124_USE_MCLKI: c_int;
    static ML26124_PW_SPAMP_PW_MNG: c_uint;
    static ML26124_R26_MASK: c_uint;
    static ML26124_BLT_PREAMP_ON: c_uint;
    static ML26124_MICBEN_ON: c_uint;
    static ML26124_BLT_ALL_ON: c_uint;
    static ML26124_SW_RST: c_uint;
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
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

pub type snd_soc_bias_level = c_uint;

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct ml26124_priv {
    mclk: u32,
    rate: u32,
    regmap: *mut regmap,
    clk_in: c_int,
    substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct clk_coeff {
    mclk: u32,
    rate: u32,
    pllnl: u8,
    pllnh: u8,
    pllml: u8,
    pllmh: u8,
    plldiv: u8,
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
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: u32,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    suspend_bias_off: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    val_bits: c_uint,
    reg_bits: c_uint,
    max_register: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    cache_type: c_uint,
    write_flag_mask: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    name: [c_char; 20],
}

#[repr(C)]
pub struct i2c_driver_inner {
    name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    driver: i2c_driver_inner,
    probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    id_table: *const i2c_device_id,
}

/* ML26124 configuration. Kernel TLV/control/widget macros are external C
 * initializers; their original declarations are preserved as comments.
 */
/* static const DECLARE_TLV_DB_SCALE(digital_tlv, -7150, 50, 0); */
/* static const DECLARE_TLV_DB_SCALE(alclvl, -2250, 150, 0); */
/* static const DECLARE_TLV_DB_SCALE(mingain, -1200, 600, 0); */
/* static const DECLARE_TLV_DB_SCALE(maxgain, -675, 600, 0); */
/* static const DECLARE_TLV_DB_SCALE(boost_vol, -1200, 75, 0); */
static ML26124_COMPANDING_0: &[u8] = b"16bit PCM\0";
static ML26124_COMPANDING_1: &[u8] = b"u-law\0";
static ML26124_COMPANDING_2: &[u8] = b"A-law\0";
static ML26124_COMPANDING: [*const c_char; 3] = [
    ML26124_COMPANDING_0.as_ptr() as *const c_char,
    ML26124_COMPANDING_1.as_ptr() as *const c_char,
    ML26124_COMPANDING_2.as_ptr() as *const c_char,
];
/* SOC_ENUM_SINGLE_DECL(ml26124_adc_companding_enum, ML26124_SAI_TRANS_CTL, 6, ml26124_companding); */
/* SOC_ENUM_SINGLE_DECL(ml26124_dac_companding_enum, ML26124_SAI_RCV_CTL, 6, ml26124_companding); */
/* static const struct snd_kcontrol_new ml26124_snd_controls[] = { ... }; */
/* static const struct snd_kcontrol_new ml26124_output_mixer_controls[] = { ... }; */

/* Input mux */
static ML26124_INPUT_SELECT_0: &[u8] = b"Analog MIC SingleEnded in\0";
static ML26124_INPUT_SELECT_1: &[u8] = b"Digital MIC in\0";
static ML26124_INPUT_SELECT_2: &[u8] = b"Analog MIC Differential in\0";
static ML26124_INPUT_SELECT: [*const c_char; 3] = [
    ML26124_INPUT_SELECT_0.as_ptr() as *const c_char,
    ML26124_INPUT_SELECT_1.as_ptr() as *const c_char,
    ML26124_INPUT_SELECT_2.as_ptr() as *const c_char,
];
/* SOC_ENUM_SINGLE_DECL(ml26124_insel_enum, ML26124_MIC_IF_CTL, 0, ml26124_input_select); */
/* static const struct snd_kcontrol_new ml26124_input_mux_controls = SOC_DAPM_ENUM(...); */
/* static const struct snd_kcontrol_new ml26124_line_control = SOC_DAPM_SINGLE(...); */
/* static const struct snd_soc_dapm_widget ml26124_dapm_widgets[] = { ... }; */

static ML26124_INTERCON: [snd_soc_dapm_route; 16] = [
    /* Supply */
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MCLKEN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MCLKEN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLLEN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLLEN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLLOE\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"PLLOE\0".as_ptr() as *const c_char },
    /* output mixer */
    snd_soc_dapm_route { sink: b"Output Mixer\0".as_ptr() as *const c_char, control: b"DAC Switch\0".as_ptr() as *const c_char, source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Output Mixer\0".as_ptr() as *const c_char, control: b"Line in loopback Switch\0".as_ptr() as *const c_char, source: b"LIN\0".as_ptr() as *const c_char },
    /* outputs */
    snd_soc_dapm_route { sink: b"LOUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPOUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Output Mixer\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Line Out Enable\0".as_ptr() as *const c_char, control: ptr::null(), source: b"LOUT\0".as_ptr() as *const c_char },
    /* input */
    snd_soc_dapm_route { sink: b"ADC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Input Mux\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"Analog MIC SingleEnded in\0".as_ptr() as *const c_char, source: b"PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Input Mux\0".as_ptr() as *const c_char, control: b"Analog MIC Differential in\0".as_ptr() as *const c_char, source: b"PGA\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"PGA\0".as_ptr() as *const c_char, control: ptr::null(), source: b"MIN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

/* PLLOutputFreq(Hz) = InputMclkFreq(Hz) * PLLM / (PLLN * PLLDIV) */
static COEFF_DIV: [clk_coeff; 3] = [
    clk_coeff { mclk: 12288000, rate: 16000, pllnl: 0xc, pllnh: 0x0, pllml: 0x20, pllmh: 0x0, plldiv: 0x4 },
    clk_coeff { mclk: 12288000, rate: 32000, pllnl: 0xc, pllnh: 0x0, pllml: 0x20, pllmh: 0x0, plldiv: 0x4 },
    clk_coeff { mclk: 12288000, rate: 48000, pllnl: 0xc, pllnh: 0x0, pllml: 0x30, pllmh: 0x0, plldiv: 0x4 },
];

static ML26124_REG: [reg_default; 67] = [
    /* CLOCK control Register */
    reg_default { reg: 0x00, def: 0x00 }, /* Sampling Rate */
    reg_default { reg: 0x02, def: 0x00 }, /* PLL NL */
    reg_default { reg: 0x04, def: 0x00 }, /* PLLNH */
    reg_default { reg: 0x06, def: 0x00 }, /* PLLML */
    reg_default { reg: 0x08, def: 0x00 }, /* MLLMH */
    reg_default { reg: 0x0a, def: 0x00 }, /* PLLDIV */
    reg_default { reg: 0x0c, def: 0x00 }, /* Clock Enable */
    reg_default { reg: 0x0e, def: 0x00 }, /* CLK Input/Output Control */
    /* System Control Register */
    reg_default { reg: 0x10, def: 0x00 }, /* Software RESET */
    reg_default { reg: 0x12, def: 0x00 }, /* Record/Playback Run */
    reg_default { reg: 0x14, def: 0x00 }, /* Mic Input/Output control */
    /* Power Management Register */
    reg_default { reg: 0x20, def: 0x00 }, /* Reference Power Management */
    reg_default { reg: 0x22, def: 0x00 }, /* Input Power Management */
    reg_default { reg: 0x24, def: 0x00 }, /* DAC Power Management */
    reg_default { reg: 0x26, def: 0x00 }, /* SP-AMP Power Management */
    reg_default { reg: 0x28, def: 0x00 }, /* LINEOUT Power Management */
    reg_default { reg: 0x2a, def: 0x00 }, /* VIDEO Power Management */
    reg_default { reg: 0x2e, def: 0x00 }, /* AC-CMP Power Management */
    /* Analog reference Control Register */
    reg_default { reg: 0x30, def: 0x04 }, /* MICBIAS Voltage Control */
    /* Input/Output Amplifier Control Register */
    reg_default { reg: 0x32, def: 0x10 }, /* MIC Input Volume */
    reg_default { reg: 0x38, def: 0x00 }, /* Mic Boost Volume */
    reg_default { reg: 0x3a, def: 0x33 }, /* Speaker AMP Volume */
    reg_default { reg: 0x48, def: 0x00 }, /* AMP Volume Control Function Enable */
    reg_default { reg: 0x4a, def: 0x00 }, /* Amplifier Volume Fader Control */
    /* Analog Path Control Register */
    reg_default { reg: 0x54, def: 0x00 }, /* Speaker AMP Output Control */
    reg_default { reg: 0x5a, def: 0x00 }, /* Mic IF Control */
    /* Audio Interface Control Register */
    reg_default { reg: 0x60, def: 0x00 }, /* SAI-Trans Control */
    reg_default { reg: 0x62, def: 0x00 }, /* SAI-Receive Control */
    reg_default { reg: 0x64, def: 0x00 }, /* SAI Mode select */
    /* DSP Control Register */
    reg_default { reg: 0x66, def: 0x01 }, /* Filter Func Enable */
    reg_default { reg: 0x68, def: 0x00 }, /* Volume Control Func Enable */
    reg_default { reg: 0x6A, def: 0x00 }, /* Mixer & Volume Control*/
    reg_default { reg: 0x6C, def: 0xff }, /* Record Digital Volume */
    reg_default { reg: 0x70, def: 0xff }, /* Playback Digital Volume */
    reg_default { reg: 0x72, def: 0x10 }, /* Digital Boost Volume */
    reg_default { reg: 0x74, def: 0xe7 }, /* EQ gain Band0 */
    reg_default { reg: 0x76, def: 0xe7 }, /* EQ gain Band1 */
    reg_default { reg: 0x78, def: 0xe7 }, /* EQ gain Band2 */
    reg_default { reg: 0x7A, def: 0xe7 }, /* EQ gain Band3 */
    reg_default { reg: 0x7C, def: 0xe7 }, /* EQ gain Band4 */
    reg_default { reg: 0x7E, def: 0x00 }, /* HPF2 CutOff*/
    reg_default { reg: 0x80, def: 0x00 }, /* EQ Band0 Coef0L */
    reg_default { reg: 0x82, def: 0x00 }, /* EQ Band0 Coef0H */
    reg_default { reg: 0x84, def: 0x00 }, /* EQ Band0 Coef0L */
    reg_default { reg: 0x86, def: 0x00 }, /* EQ Band0 Coef0H */
    reg_default { reg: 0x88, def: 0x00 }, /* EQ Band1 Coef0L */
    reg_default { reg: 0x8A, def: 0x00 }, /* EQ Band1 Coef0H */
    reg_default { reg: 0x8C, def: 0x00 }, /* EQ Band1 Coef0L */
    reg_default { reg: 0x8E, def: 0x00 }, /* EQ Band1 Coef0H */
    reg_default { reg: 0x90, def: 0x00 }, /* EQ Band2 Coef0L */
    reg_default { reg: 0x92, def: 0x00 }, /* EQ Band2 Coef0H */
    reg_default { reg: 0x94, def: 0x00 }, /* EQ Band2 Coef0L */
    reg_default { reg: 0x96, def: 0x00 }, /* EQ Band2 Coef0H */
    reg_default { reg: 0x98, def: 0x00 }, /* EQ Band3 Coef0L */
    reg_default { reg: 0x9A, def: 0x00 }, /* EQ Band3 Coef0H */
    reg_default { reg: 0x9C, def: 0x00 }, /* EQ Band3 Coef0L */
    reg_default { reg: 0x9E, def: 0x00 }, /* EQ Band3 Coef0H */
    reg_default { reg: 0xA0, def: 0x00 }, /* EQ Band4 Coef0L */
    reg_default { reg: 0xA2, def: 0x00 }, /* EQ Band4 Coef0H */
    reg_default { reg: 0xA4, def: 0x00 }, /* EQ Band4 Coef0L */
    reg_default { reg: 0xA6, def: 0x00 }, /* EQ Band4 Coef0H */
    /* ALC Control Register */
    reg_default { reg: 0xb0, def: 0x00 }, /* ALC Mode */
    reg_default { reg: 0xb2, def: 0x02 }, /* ALC Attack Time */
    reg_default { reg: 0xb4, def: 0x03 }, /* ALC Decay Time */
    reg_default { reg: 0xb6, def: 0x00 }, /* ALC Hold Time */
    reg_default { reg: 0xb8, def: 0x0b }, /* ALC Target Level */
    reg_default { reg: 0xba, def: 0x70 }, /* ALC Max/Min Gain */
    reg_default { reg: 0xbc, def: 0x00 }, /* Noise Gate Threshold */
    reg_default { reg: 0xbe, def: 0x00 }, /* ALC ZeroCross TimeOut */
    /* Playback Limiter Control Register */
    reg_default { reg: 0xc0, def: 0x04 }, /* PL Attack Time */
    reg_default { reg: 0xc2, def: 0x05 }, /* PL Decay Time */
    reg_default { reg: 0xc4, def: 0x0d }, /* PL Target Level */
    reg_default { reg: 0xc6, def: 0x70 }, /* PL Max/Min Gain */
    reg_default { reg: 0xc8, def: 0x10 }, /* Playback Boost Volume */
    reg_default { reg: 0xca, def: 0x00 }, /* PL ZeroCross TimeOut */
    /* Video Amplifier Control Register */
    reg_default { reg: 0xd0, def: 0x01 }, /* VIDEO AMP Gain Control */
    reg_default { reg: 0xd2, def: 0x01 }, /* VIDEO AMP Setup 1 */
    reg_default { reg: 0xd4, def: 0x01 }, /* VIDEO AMP Control2 */
    /* Analog Path Control Register */
    reg_default { reg: 0xe8, def: 0x01 }, /* Mic Select Control */
];

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn msleep(msecs: c_uint);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

/* Get sampling rate value of sampling rate setting register (0x0) */
#[inline]
unsafe fn get_srate(rate: c_int) -> c_int {
    let srate: c_int;

    match rate {
        16000 => {
            srate = 3;
        }
        32000 => {
            srate = 6;
        }
        48000 => {
            srate = 8;
        }
        _ => {
            return -EINVAL;
        }
    }
    srate
}

#[inline]
unsafe fn get_coeff(mclk: c_int, rate: c_int) -> c_int {
    let mut i: usize = 0;

    while i < COEFF_DIV.len() {
        if COEFF_DIV[i].rate == rate as u32 && COEFF_DIV[i].mclk == mclk as u32 {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn ml26124_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ml26124_priv;
    let i = get_coeff((*priv_).mclk as c_int, params_rate(hw_params) as c_int);
    let srate: c_int;

    if i < 0 {
        return i;
    }
    (*priv_).substream = substream;
    (*priv_).rate = params_rate(hw_params);

    if (*priv_).clk_in != 0 {
        match (*priv_).mclk / params_rate(hw_params) {
            256 => {
                snd_soc_component_update_bits(component, ML26124_CLK_CTL, BIT(0) | BIT(1), 1);
            }
            512 => {
                snd_soc_component_update_bits(component, ML26124_CLK_CTL, BIT(0) | BIT(1), 2);
            }
            1024 => {
                snd_soc_component_update_bits(component, ML26124_CLK_CTL, BIT(0) | BIT(1), 3);
            }
            _ => {
                dev_err((*component).dev, b"Unsupported MCLKI\n\0".as_ptr() as *const c_char);
            }
        }
    } else {
        snd_soc_component_update_bits(component, ML26124_CLK_CTL, BIT(0) | BIT(1), 0);
    }

    srate = get_srate(params_rate(hw_params) as c_int);
    if srate < 0 {
        return srate;
    }

    snd_soc_component_update_bits(component, ML26124_SMPLING_RATE, 0xf, srate as c_uint);
    snd_soc_component_update_bits(component, ML26124_PLLNL, 0xff, COEFF_DIV[i as usize].pllnl as c_uint);
    snd_soc_component_update_bits(component, ML26124_PLLNH, 0x1, COEFF_DIV[i as usize].pllnh as c_uint);
    snd_soc_component_update_bits(component, ML26124_PLLML, 0xff, COEFF_DIV[i as usize].pllml as c_uint);
    snd_soc_component_update_bits(component, ML26124_PLLMH, 0x3f, COEFF_DIV[i as usize].pllmh as c_uint);
    snd_soc_component_update_bits(component, ML26124_PLLDIV, 0x1f, COEFF_DIV[i as usize].plldiv as c_uint);

    0
}

unsafe extern "C" fn ml26124_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ml26124_priv;

    match (*(*priv_).substream).stream {
        x if x == SNDRV_PCM_STREAM_CAPTURE => {
            snd_soc_component_update_bits(component, ML26124_REC_PLYBAK_RUN, BIT(0), 1);
        }
        x if x == SNDRV_PCM_STREAM_PLAYBACK => {
            snd_soc_component_update_bits(component, ML26124_REC_PLYBAK_RUN, BIT(1), 2);
        }
        _ => {}
    }

    if mute != 0 {
        snd_soc_component_update_bits(component, ML26124_DVOL_CTL, BIT(4), DVOL_CTL_DVMUTE_ON);
    } else {
        snd_soc_component_update_bits(component, ML26124_DVOL_CTL, BIT(4), DVOL_CTL_DVMUTE_OFF);
    }

    0
}

unsafe extern "C" fn ml26124_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: c_uint,
) -> c_int {
    let mode: u8;
    let component = (*codec_dai).component;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            mode = 1;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            mode = 0;
        }
        _ => {
            return -EINVAL;
        }
    }
    snd_soc_component_update_bits(component, ML26124_SAI_MODE_SEL, BIT(0), mode as c_uint);

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {}
        _ => {
            return -EINVAL;
        }
    }

    /* clock inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn ml26124_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ml26124_priv;

    match clk_id {
        x if x == ML26124_USE_PLLOUT => {
            (*priv_).clk_in = ML26124_USE_PLLOUT;
        }
        x if x == ML26124_USE_MCLKI => {
            (*priv_).clk_in = ML26124_USE_MCLKI;
        }
        _ => {
            return -EINVAL;
        }
    }

    (*priv_).mclk = freq;

    0
}

unsafe extern "C" fn ml26124_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut ml26124_priv;
    let dapm = snd_soc_component_to_dapm(component);

    if level == SND_SOC_BIAS_ON {
        snd_soc_component_update_bits(
            component,
            ML26124_PW_SPAMP_PW_MNG,
            ML26124_R26_MASK,
            ML26124_BLT_PREAMP_ON,
        );
        msleep(100);
        snd_soc_component_update_bits(
            component,
            ML26124_PW_SPAMP_PW_MNG,
            ML26124_R26_MASK,
            ML26124_MICBEN_ON | ML26124_BLT_ALL_ON,
        );
    } else if level == SND_SOC_BIAS_PREPARE {
    } else if level == SND_SOC_BIAS_STANDBY {
        /* VMID ON */
        if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
            snd_soc_component_update_bits(component, ML26124_PW_REF_PW_MNG, ML26124_VMID, ML26124_VMID);
            msleep(500);
            regcache_sync((*priv_).regmap);
        }
    } else if level == SND_SOC_BIAS_OFF {
        /* VMID OFF */
        snd_soc_component_update_bits(component, ML26124_PW_REF_PW_MNG, ML26124_VMID, 0);
    }
    0
}

static ML26124_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(ml26124_hw_params),
    mute_stream: Some(ml26124_mute),
    set_fmt: Some(ml26124_set_dai_fmt),
    set_sysclk: Some(ml26124_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut ML26124_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"ml26124-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: ML26124_RATES,
        formats: ML26124_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: ML26124_RATES,
        formats: ML26124_FORMATS,
    },
    ops: &ML26124_DAI_OPS,
    symmetric_rate: 1,
};

unsafe extern "C" fn ml26124_probe(component: *mut snd_soc_component) -> c_int {
    /* Software Reset */
    snd_soc_component_update_bits(component, ML26124_SW_RST, 0x01, 1);
    snd_soc_component_update_bits(component, ML26124_SW_RST, 0x01, 0);

    0
}

static SOC_COMPONENT_DEV_ML26124: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ml26124_probe),
    set_bias_level: Some(ml26124_set_bias_level),
    controls: ptr::null(), /* ml26124_snd_controls */
    num_controls: 0,       /* ARRAY_SIZE(ml26124_snd_controls) */
    dapm_widgets: ptr::null(), /* ml26124_dapm_widgets */
    num_dapm_widgets: 0,       /* ARRAY_SIZE(ml26124_dapm_widgets) */
    dapm_routes: ML26124_INTERCON.as_ptr(),
    num_dapm_routes: ML26124_INTERCON.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static ML26124_I2C_REGMAP: regmap_config = regmap_config {
    val_bits: 8,
    reg_bits: 8,
    max_register: ML26124_NUM_REGISTER,
    reg_defaults: ML26124_REG.as_ptr(),
    num_reg_defaults: ML26124_REG.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
    write_flag_mask: 0x01,
};

unsafe extern "C" fn ml26124_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let priv_: *mut ml26124_priv;
    let ret: c_int;

    priv_ = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<ml26124_priv>(), GFP_KERNEL)
        as *mut ml26124_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, priv_ as *mut c_void);

    (*priv_).regmap = devm_regmap_init_i2c(i2c, &ML26124_I2C_REGMAP);
    if IS_ERR((*priv_).regmap as *const c_void) {
        ret = PTR_ERR((*priv_).regmap as *const c_void);
        dev_err(
            &mut (*i2c).dev,
            b"regmap_init_i2c() failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &SOC_COMPONENT_DEV_ML26124,
        &mut ML26124_DAI,
        1,
    )
}

static ML26124_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id { name: [
        b'm' as c_char, b'l' as c_char, b'2' as c_char, b'6' as c_char,
        b'1' as c_char, b'2' as c_char, b'4' as c_char, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ] },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, ml26124_i2c_id); */

static mut ML26124_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: i2c_driver_inner {
        name: b"ml26124\0".as_ptr() as *const c_char,
    },
    probe: Some(ml26124_i2c_probe),
    id_table: ML26124_I2C_ID.as_ptr(),
};

/* module_i2c_driver(ml26124_i2c_driver); */

/* MODULE_AUTHOR("Tomoya MORINAGA <tomoya.rohm@gmail.com>"); */
/* MODULE_DESCRIPTION("LAPIS Semiconductor ML26124 ALSA SoC codec driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
