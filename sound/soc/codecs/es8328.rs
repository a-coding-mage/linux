// SPDX-License-Identifier: GPL-2.0-only
/*
 * es8328.c  --  ES8328 ALSA SoC Audio driver
 *
 * Copyright 2014 Sutajio Ko-Usagi PTE LTD
 *
 * Author: Sean Cross <xobs@kosagi.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// Dependencies originally supplied by Linux, ALSA SoC, TLV, and "es8328.h".

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
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
    pub stream: c_int,
    pub runtime: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
    pub consumer: *mut c_void,
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
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
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub cache_type: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub suspend_bias_off: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

pub type c_long = i64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut c_void,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn msleep(msecs: c_uint);
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

unsafe extern "C" {
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S18_3LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_HW_PARAM_RATE: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static GFP_KERNEL: c_uint;
    static REGCACHE_MAPLE: c_uint;

    static ES8328_ADCCONTROL1: c_uint;
    static ES8328_ADCCONTROL2: c_uint;
    static ES8328_ADCCONTROL3: c_uint;
    static ES8328_ADCCONTROL4: c_uint;
    static ES8328_ADCCONTROL4_ADCFORMAT_I2S: c_uint;
    static ES8328_ADCCONTROL4_ADCFORMAT_LJUST: c_uint;
    static ES8328_ADCCONTROL4_ADCFORMAT_MASK: c_uint;
    static ES8328_ADCCONTROL4_ADCFORMAT_RJUST: c_uint;
    static ES8328_ADCCONTROL4_ADCWL_MASK: c_uint;
    static ES8328_ADCCONTROL4_ADCWL_SHIFT: c_uint;
    static ES8328_ADCCONTROL5: c_uint;
    static ES8328_ADCCONTROL6: c_uint;
    static ES8328_ADCCONTROL7: c_uint;
    static ES8328_ADCCONTROL8: c_uint;
    static ES8328_ADCCONTROL9: c_uint;
    static ES8328_ADCPOWER: c_uint;
    static ES8328_ADCPOWER_ADCL_OFF: c_uint;
    static ES8328_ADCPOWER_ADCR_OFF: c_uint;
    static ES8328_ADCPOWER_ADC_BIAS_GEN_OFF: c_uint;
    static ES8328_ADCPOWER_AINL_OFF: c_uint;
    static ES8328_ADCPOWER_AINR_OFF: c_uint;
    static ES8328_ADCPOWER_MIC_BIAS_OFF: c_uint;
    static ES8328_CHIPPOWER: c_uint;
    static ES8328_CHIPPOWER_ADCDIG_OFF: c_uint;
    static ES8328_CHIPPOWER_ADCDLL_OFF: c_uint;
    static ES8328_CHIPPOWER_ADCSTM_RESET: c_uint;
    static ES8328_CHIPPOWER_ADCVREF_OFF: c_uint;
    static ES8328_CHIPPOWER_DACDIG_OFF: c_uint;
    static ES8328_CHIPPOWER_DACDLL_OFF: c_uint;
    static ES8328_CHIPPOWER_DACSTM_RESET: c_uint;
    static ES8328_CHIPPOWER_DACVREF_OFF: c_uint;
    static ES8328_CONTROL1: c_uint;
    static ES8328_CONTROL1_ENREF: c_uint;
    static ES8328_CONTROL1_VMIDSEL_500k: c_uint;
    static ES8328_CONTROL1_VMIDSEL_50k: c_uint;
    static ES8328_CONTROL1_VMIDSEL_5k: c_uint;
    static ES8328_CONTROL1_VMIDSEL_MASK: c_uint;
    static ES8328_CONTROL2: c_uint;
    static ES8328_CONTROL2_OVERCURRENT_ON: c_uint;
    static ES8328_CONTROL2_THERMAL_SHUTDOWN_ON: c_uint;
    static ES8328_DACCONTROL1: c_uint;
    static ES8328_DACCONTROL1_DACFORMAT_I2S: c_uint;
    static ES8328_DACCONTROL1_DACFORMAT_LJUST: c_uint;
    static ES8328_DACCONTROL1_DACFORMAT_MASK: c_uint;
    static ES8328_DACCONTROL1_DACFORMAT_RJUST: c_uint;
    static ES8328_DACCONTROL1_DACWL_MASK: c_uint;
    static ES8328_DACCONTROL1_DACWL_SHIFT: c_uint;
    static ES8328_DACCONTROL2: c_uint;
    static ES8328_DACCONTROL3: c_uint;
    static ES8328_DACCONTROL3_DACMUTE: c_uint;
    static ES8328_DACCONTROL6: c_uint;
    static ES8328_DACCONTROL6_DEEMPH_32k: c_uint;
    static ES8328_DACCONTROL6_DEEMPH_44_1k: c_uint;
    static ES8328_DACCONTROL6_DEEMPH_48k: c_uint;
    static ES8328_DACCONTROL6_DEEMPH_MASK: c_uint;
    static ES8328_DACCONTROL6_DEEMPH_OFF: c_uint;
    static ES8328_DACCONTROL16: c_uint;
    static ES8328_DACCONTROL17: c_uint;
    static ES8328_DACCONTROL18: c_uint;
    static ES8328_DACCONTROL19: c_uint;
    static ES8328_DACCONTROL20: c_uint;
    static ES8328_DACPOWER: c_uint;
    static ES8328_DACPOWER_LDAC_OFF: c_uint;
    static ES8328_DACPOWER_LOUT1_ON: c_uint;
    static ES8328_DACPOWER_LOUT2_ON: c_uint;
    static ES8328_DACPOWER_RDAC_OFF: c_uint;
    static ES8328_DACPOWER_ROUT1_ON: c_uint;
    static ES8328_DACPOWER_ROUT2_ON: c_uint;
    static ES8328_DACVOL_MAX: c_uint;
    static ES8328_LDACVOL: c_uint;
    static ES8328_LOUT1VOL: c_uint;
    static ES8328_LOUT2VOL: c_uint;
    static ES8328_MASTERMODE: c_uint;
    static ES8328_MASTERMODE_MCLKDIV2: c_uint;
    static ES8328_MASTERMODE_MSC: c_uint;
    static ES8328_OUT1VOL_MAX: c_uint;
    static ES8328_OUT2VOL_MAX: c_uint;
    static ES8328_RATEMASK: c_uint;
    static ES8328_RDACVOL: c_uint;
    static ES8328_REG_MAX: c_uint;
    static ES8328_ROUT1VOL: c_uint;
    static ES8328_ROUT2VOL: c_uint;
    static SND_SOC_NOPM: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

static rates_12288: [c_uint; 7] = [8000, 12000, 16000, 24000, 32000, 48000, 96000];
static ratios_12288: [c_int; 7] = [10, 7, 6, 4, 3, 2, 0];
static constraints_12288: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_12288.len() as c_uint,
    list: rates_12288.as_ptr(),
    mask: 0,
};

static rates_11289: [c_uint; 5] = [8018, 11025, 22050, 44100, 88200];
static ratios_11289: [c_int; 5] = [9, 7, 4, 2, 0];
static constraints_11289: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates_11289.len() as c_uint,
    list: rates_11289.as_ptr(),
    mask: 0,
};

/* regulator supplies for sgtl5000, VDDD is an optional external supply */
#[repr(C)]
enum sgtl5000_regulator_supplies {
    DVDD,
    AVDD,
    PVDD,
    HPVDD,
    ES8328_SUPPLY_NUM,
}

const ES8328_SUPPLY_NUM_USIZE: usize = sgtl5000_regulator_supplies::ES8328_SUPPLY_NUM as usize;

/* vddd is optional supply */
static supply_names: [*const c_char; ES8328_SUPPLY_NUM_USIZE] = [
    b"DVDD\0".as_ptr() as *const c_char,
    b"AVDD\0".as_ptr() as *const c_char,
    b"PVDD\0".as_ptr() as *const c_char,
    b"HPVDD\0".as_ptr() as *const c_char,
];

macro_rules! ES8328_RATES {
    () => {
        (SNDRV_PCM_RATE_192000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_8000_48000)
    };
}

macro_rules! ES8328_FORMATS {
    () => {
        (SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S18_3LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE)
    };
}

#[repr(C)]
struct es8328_priv {
    regmap: *mut regmap,
    clk: *mut clk,
    playback_fs: c_int,
    deemph: bool,
    mclkdiv2: c_int,
    sysclk_constraints: *const snd_pcm_hw_constraint_list,
    mclk_ratios: *const c_int,
    provider: bool,
    supplies: [regulator_bulk_data; ES8328_SUPPLY_NUM_USIZE],
}

/*
 * ES8328 Controls
 */

static adcpol_txt: [*const c_char; 4] = [
    b"Normal\0".as_ptr() as *const c_char,
    b"L Invert\0".as_ptr() as *const c_char,
    b"R Invert\0".as_ptr() as *const c_char,
    b"L + R Invert\0".as_ptr() as *const c_char,
];
SOC_ENUM_SINGLE_DECL!(adcpol, ES8328_ADCCONTROL6, 6, adcpol_txt);

static play_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-3000, 100, 0);
static dac_adc_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-9600, 50, 0);
static bypass_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1500, 300, 0);
static mic_tlv: [c_uint; 0] = DECLARE_TLV_DB_SCALE!(0, 300, 0);

#[repr(C)]
struct deemph_setting {
    rate: c_int,
    val: c_uint,
}

static deemph_settings: [deemph_setting; 4] = unsafe {
    [
        deemph_setting { rate: 0, val: ES8328_DACCONTROL6_DEEMPH_OFF },
        deemph_setting { rate: 32000, val: ES8328_DACCONTROL6_DEEMPH_32k },
        deemph_setting { rate: 44100, val: ES8328_DACCONTROL6_DEEMPH_44_1k },
        deemph_setting { rate: 48000, val: ES8328_DACCONTROL6_DEEMPH_48k },
    ]
};

unsafe extern "C" fn es8328_set_deemph(component: *mut snd_soc_component) -> c_int {
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let mut val: c_uint;
    let mut i: usize;
    let mut best: usize;

    /*
     * If we're using deemphasis select the nearest available sample
     * rate.
     */
    if (*es8328).deemph {
        best = 0;
        i = 1;
        while i < deemph_settings.len() {
            if (deemph_settings[i].rate - (*es8328).playback_fs).abs()
                < (deemph_settings[best].rate - (*es8328).playback_fs).abs()
            {
                best = i;
            }
            i += 1;
        }

        val = deemph_settings[best].val;
    } else {
        val = ES8328_DACCONTROL6_DEEMPH_OFF;
    }

    dev_dbg((*component).dev, b"Set deemphasis %d\n\0".as_ptr() as *const c_char, val);

    snd_soc_component_update_bits(
        component,
        ES8328_DACCONTROL6,
        ES8328_DACCONTROL6_DEEMPH_MASK,
        val,
    )
}

unsafe extern "C" fn es8328_get_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;

    (*ucontrol).value.integer.value[0] = (*es8328).deemph as c_long;
    0
}

unsafe extern "C" fn es8328_put_deemph(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let deemph = (*ucontrol).value.integer.value[0] as c_uint;
    let ret: c_int;

    if deemph > 1 {
        return -EINVAL;
    }

    if (*es8328).deemph as c_uint == deemph {
        return 0;
    }

    (*es8328).deemph = deemph != 0;
    ret = es8328_set_deemph(component);
    if ret < 0 {
        return ret;
    }

    1
}

static es8328_snd_controls: [snd_kcontrol_new; 12] = [
    SOC_DOUBLE_R_TLV!("Capture Digital Volume", ES8328_ADCCONTROL8, ES8328_ADCCONTROL9, 0, 0xc0, 1, dac_adc_tlv),
    SOC_SINGLE!("Capture ZC Switch", ES8328_ADCCONTROL7, 6, 1, 0),
    SOC_SINGLE_BOOL_EXT!("DAC Deemphasis Switch", 0, es8328_get_deemph, es8328_put_deemph),
    SOC_ENUM!("Capture Polarity", adcpol),
    SOC_SINGLE_TLV!("Left Mixer Left Bypass Volume", ES8328_DACCONTROL17, 3, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Left Mixer Right Bypass Volume", ES8328_DACCONTROL19, 3, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Right Mixer Left Bypass Volume", ES8328_DACCONTROL18, 3, 7, 1, bypass_tlv),
    SOC_SINGLE_TLV!("Right Mixer Right Bypass Volume", ES8328_DACCONTROL20, 3, 7, 1, bypass_tlv),
    SOC_DOUBLE_R_TLV!("PCM Volume", ES8328_LDACVOL, ES8328_RDACVOL, 0, ES8328_DACVOL_MAX, 1, dac_adc_tlv),
    SOC_DOUBLE_R_TLV!("Output 1 Playback Volume", ES8328_LOUT1VOL, ES8328_ROUT1VOL, 0, ES8328_OUT1VOL_MAX, 0, play_tlv),
    SOC_DOUBLE_R_TLV!("Output 2 Playback Volume", ES8328_LOUT2VOL, ES8328_ROUT2VOL, 0, ES8328_OUT2VOL_MAX, 0, play_tlv),
    SOC_DOUBLE_TLV!("Mic PGA Volume", ES8328_ADCCONTROL1, 4, 0, 8, 0, mic_tlv),
];

/*
 * DAPM Controls
 */

static es8328_line_texts: [*const c_char; 4] = [
    b"Line 1\0".as_ptr() as *const c_char,
    b"Line 2\0".as_ptr() as *const c_char,
    b"PGA\0".as_ptr() as *const c_char,
    b"Differential\0".as_ptr() as *const c_char,
];

static es8328_lline_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES8328_DACCONTROL16, 3, es8328_line_texts.len(), es8328_line_texts);
static es8328_left_line_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", es8328_lline_enum);

static es8328_rline_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES8328_DACCONTROL16, 0, es8328_line_texts.len(), es8328_line_texts);
static es8328_right_line_controls: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", es8328_rline_enum);

/* Left Mixer */
static es8328_left_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Left Bypass Switch", ES8328_DACCONTROL17, 6, 1, 0),
    SOC_DAPM_SINGLE!("Right Playback Switch", ES8328_DACCONTROL18, 7, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", ES8328_DACCONTROL18, 6, 1, 0),
];

/* Right Mixer */
static es8328_right_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Left Playback Switch", ES8328_DACCONTROL19, 7, 1, 0),
    SOC_DAPM_SINGLE!("Left Bypass Switch", ES8328_DACCONTROL19, 6, 1, 0),
    SOC_DAPM_SINGLE!("Right Bypass Switch", ES8328_DACCONTROL20, 6, 1, 0),
];

static es8328_pga_sel: [*const c_char; 4] = [
    b"Line 1\0".as_ptr() as *const c_char,
    b"Line 2\0".as_ptr() as *const c_char,
    b"Line 3\0".as_ptr() as *const c_char,
    b"Differential\0".as_ptr() as *const c_char,
];

/* Left PGA Mux */
static es8328_lpga_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES8328_ADCCONTROL2, 6, es8328_pga_sel.len(), es8328_pga_sel);
static es8328_left_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", es8328_lpga_enum);

/* Right PGA Mux */
static es8328_rpga_enum: soc_enum =
    SOC_ENUM_SINGLE!(ES8328_ADCCONTROL2, 4, es8328_pga_sel.len(), es8328_pga_sel);
static es8328_right_pga_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", es8328_rpga_enum);

/* Differential Mux */
static es8328_diff_sel: [*const c_char; 2] = [
    b"Line 1\0".as_ptr() as *const c_char,
    b"Line 2\0".as_ptr() as *const c_char,
];
SOC_ENUM_SINGLE_DECL!(diffmux, ES8328_ADCCONTROL3, 7, es8328_diff_sel);
static es8328_diffmux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", diffmux);

/* Mono ADC Mux */
static es8328_mono_mux: [*const c_char; 4] = [
    b"Stereo\0".as_ptr() as *const c_char,
    b"Mono (Left)\0".as_ptr() as *const c_char,
    b"Mono (Right)\0".as_ptr() as *const c_char,
    b"Digital Mono\0".as_ptr() as *const c_char,
];
SOC_ENUM_SINGLE_DECL!(monomux, ES8328_ADCCONTROL3, 3, es8328_mono_mux);
static es8328_monomux_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", monomux);

static es8328_dapm_widgets: [snd_soc_dapm_widget; 32] = [
    SND_SOC_DAPM_MUX!("Differential Mux", SND_SOC_NOPM, 0, 0, &es8328_diffmux_controls),
    SND_SOC_DAPM_MUX!("Left ADC Mux", SND_SOC_NOPM, 0, 0, &es8328_monomux_controls),
    SND_SOC_DAPM_MUX!("Right ADC Mux", SND_SOC_NOPM, 0, 0, &es8328_monomux_controls),
    SND_SOC_DAPM_MUX!("Left PGA Mux", ES8328_ADCPOWER, ES8328_ADCPOWER_AINL_OFF, 1, &es8328_left_pga_controls),
    SND_SOC_DAPM_MUX!("Right PGA Mux", ES8328_ADCPOWER, ES8328_ADCPOWER_AINR_OFF, 1, &es8328_right_pga_controls),
    SND_SOC_DAPM_MUX!("Left Line Mux", SND_SOC_NOPM, 0, 0, &es8328_left_line_controls),
    SND_SOC_DAPM_MUX!("Right Line Mux", SND_SOC_NOPM, 0, 0, &es8328_right_line_controls),
    SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", ES8328_ADCPOWER, ES8328_ADCPOWER_ADCR_OFF, 1),
    SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", ES8328_ADCPOWER, ES8328_ADCPOWER_ADCL_OFF, 1),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", ES8328_ADCPOWER, ES8328_ADCPOWER_MIC_BIAS_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias Gen", ES8328_ADCPOWER, ES8328_ADCPOWER_ADC_BIAS_GEN_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC STM", ES8328_CHIPPOWER, ES8328_CHIPPOWER_DACSTM_RESET, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC STM", ES8328_CHIPPOWER, ES8328_CHIPPOWER_ADCSTM_RESET, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC DIG", ES8328_CHIPPOWER, ES8328_CHIPPOWER_DACDIG_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC DIG", ES8328_CHIPPOWER, ES8328_CHIPPOWER_ADCDIG_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC DLL", ES8328_CHIPPOWER, ES8328_CHIPPOWER_DACDLL_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC DLL", ES8328_CHIPPOWER, ES8328_CHIPPOWER_ADCDLL_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC Vref", ES8328_CHIPPOWER, ES8328_CHIPPOWER_ADCVREF_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("DAC Vref", ES8328_CHIPPOWER, ES8328_CHIPPOWER_DACVREF_OFF, 1, ptr::null(), 0),
    SND_SOC_DAPM_DAC!("Right DAC", "Right Playback", ES8328_DACPOWER, ES8328_DACPOWER_RDAC_OFF, 1),
    SND_SOC_DAPM_DAC!("Left DAC", "Left Playback", ES8328_DACPOWER, ES8328_DACPOWER_LDAC_OFF, 1),
    SND_SOC_DAPM_MIXER!("Left Mixer", ES8328_DACCONTROL17, 7, 0, &es8328_left_mixer_controls[0], es8328_left_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("Right Mixer", ES8328_DACCONTROL20, 7, 0, &es8328_right_mixer_controls[0], es8328_right_mixer_controls.len()),
    SND_SOC_DAPM_PGA!("Right Out 2", ES8328_DACPOWER, ES8328_DACPOWER_ROUT2_ON, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Out 2", ES8328_DACPOWER, ES8328_DACPOWER_LOUT2_ON, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Right Out 1", ES8328_DACPOWER, ES8328_DACPOWER_ROUT1_ON, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Left Out 1", ES8328_DACPOWER, ES8328_DACPOWER_LOUT1_ON, 0, ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("LOUT1"),
    SND_SOC_DAPM_OUTPUT!("ROUT1"),
    SND_SOC_DAPM_OUTPUT!("LOUT2"),
    SND_SOC_DAPM_OUTPUT!("ROUT2"),
    SND_SOC_DAPM_INPUT!("LINPUT1"),
    SND_SOC_DAPM_INPUT!("LINPUT2"),
    SND_SOC_DAPM_INPUT!("RINPUT1"),
    SND_SOC_DAPM_INPUT!("RINPUT2"),
];

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route {
            sink: concat!($sink, "\0").as_ptr() as *const c_char,
            control: ptr::null(),
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

static es8328_dapm_routes: [snd_soc_dapm_route; 65] = [
    route!("Left Line Mux", "Line 1", "LINPUT1"),
    route!("Left Line Mux", "Line 2", "LINPUT2"),
    route!("Left Line Mux", "PGA", "Left PGA Mux"),
    route!("Left Line Mux", "Differential", "Differential Mux"),
    route!("Right Line Mux", "Line 1", "RINPUT1"),
    route!("Right Line Mux", "Line 2", "RINPUT2"),
    route!("Right Line Mux", "PGA", "Right PGA Mux"),
    route!("Right Line Mux", "Differential", "Differential Mux"),
    route!("Left PGA Mux", "Line 1", "LINPUT1"),
    route!("Left PGA Mux", "Line 2", "LINPUT2"),
    route!("Left PGA Mux", "Differential", "Differential Mux"),
    route!("Right PGA Mux", "Line 1", "RINPUT1"),
    route!("Right PGA Mux", "Line 2", "RINPUT2"),
    route!("Right PGA Mux", "Differential", "Differential Mux"),
    route!("Differential Mux", "Line 1", "LINPUT1"),
    route!("Differential Mux", "Line 1", "RINPUT1"),
    route!("Differential Mux", "Line 2", "LINPUT2"),
    route!("Differential Mux", "Line 2", "RINPUT2"),
    route!("Left ADC Mux", "Stereo", "Left PGA Mux"),
    route!("Left ADC Mux", "Mono (Left)", "Left PGA Mux"),
    route!("Left ADC Mux", "Digital Mono", "Left PGA Mux"),
    route!("Right ADC Mux", "Stereo", "Right PGA Mux"),
    route!("Right ADC Mux", "Mono (Right)", "Right PGA Mux"),
    route!("Right ADC Mux", "Digital Mono", "Right PGA Mux"),
    route!("Left ADC", NULL, "Left ADC Mux"),
    route!("Right ADC", NULL, "Right ADC Mux"),
    route!("ADC DIG", NULL, "ADC STM"),
    route!("ADC DIG", NULL, "ADC Vref"),
    route!("ADC DIG", NULL, "ADC DLL"),
    route!("Left ADC", NULL, "ADC DIG"),
    route!("Right ADC", NULL, "ADC DIG"),
    route!("Mic Bias", NULL, "Mic Bias Gen"),
    route!("LINPUT1", NULL, "Mic Bias"),
    route!("RINPUT1", NULL, "Mic Bias"),
    route!("LINPUT2", NULL, "Mic Bias"),
    route!("RINPUT2", NULL, "Mic Bias"),
    route!("Left Mixer", NULL, "Left DAC"),
    route!("Left Mixer", "Left Bypass Switch", "Left Line Mux"),
    route!("Left Mixer", "Right Playback Switch", "Right DAC"),
    route!("Left Mixer", "Right Bypass Switch", "Right Line Mux"),
    route!("Right Mixer", "Left Playback Switch", "Left DAC"),
    route!("Right Mixer", "Left Bypass Switch", "Left Line Mux"),
    route!("Right Mixer", NULL, "Right DAC"),
    route!("Right Mixer", "Right Bypass Switch", "Right Line Mux"),
    route!("DAC DIG", NULL, "DAC STM"),
    route!("DAC DIG", NULL, "DAC Vref"),
    route!("DAC DIG", NULL, "DAC DLL"),
    route!("Left DAC", NULL, "DAC DIG"),
    route!("Right DAC", NULL, "DAC DIG"),
    route!("Left Out 1", NULL, "Left Mixer"),
    route!("LOUT1", NULL, "Left Out 1"),
    route!("Right Out 1", NULL, "Right Mixer"),
    route!("ROUT1", NULL, "Right Out 1"),
    route!("Left Out 2", NULL, "Left Mixer"),
    route!("LOUT2", NULL, "Left Out 2"),
    route!("Right Out 2", NULL, "Right Mixer"),
    route!("ROUT2", NULL, "Right Out 2"),
];

unsafe extern "C" fn es8328_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    snd_soc_component_update_bits(
        (*dai).component,
        ES8328_DACCONTROL3,
        ES8328_DACCONTROL3_DACMUTE,
        if mute != 0 { ES8328_DACCONTROL3_DACMUTE } else { 0 },
    )
}

unsafe extern "C" fn es8328_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;

    if (*es8328).provider && !(*es8328).sysclk_constraints.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*es8328).sysclk_constraints,
        );
    }

    0
}

unsafe extern "C" fn es8328_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let mut ret: c_int;
    let mut i: c_uint;
    let reg: c_uint;
    let wl: c_int;
    let ratio: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        reg = ES8328_DACCONTROL2;
    } else {
        reg = ES8328_ADCCONTROL5;
    }

    if (*es8328).provider {
        if (*es8328).sysclk_constraints.is_null() {
            dev_err((*component).dev, b"No MCLK configured\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        i = 0;
        while i < (*(*es8328).sysclk_constraints).count {
            if *(*(*es8328).sysclk_constraints).list.add(i as usize) == params_rate(params) {
                break;
            }
            i += 1;
        }

        if i == (*(*es8328).sysclk_constraints).count {
            dev_err(
                (*component).dev,
                b"LRCLK %d unsupported with current clock\n\0".as_ptr() as *const c_char,
                params_rate(params),
            );
            return -EINVAL;
        }
        ratio = *(*es8328).mclk_ratios.add(i as usize);
    } else {
        ratio = 0;
        (*es8328).mclkdiv2 = 0;
    }

    ret = snd_soc_component_update_bits(
        component,
        ES8328_MASTERMODE,
        ES8328_MASTERMODE_MCLKDIV2,
        if (*es8328).mclkdiv2 != 0 { ES8328_MASTERMODE_MCLKDIV2 } else { 0 },
    );
    if ret < 0 {
        return ret;
    }

    match params_width(params) {
        16 => wl = 3,
        18 => wl = 2,
        20 => wl = 1,
        24 => wl = 0,
        32 => wl = 4,
        _ => return -EINVAL,
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = snd_soc_component_update_bits(
            component,
            ES8328_DACCONTROL1,
            ES8328_DACCONTROL1_DACWL_MASK,
            (wl as c_uint) << ES8328_DACCONTROL1_DACWL_SHIFT,
        );
        if ret < 0 {
            return ret;
        }

        (*es8328).playback_fs = params_rate(params) as c_int;
        ret = es8328_set_deemph(component);
        if ret < 0 {
            return ret;
        }
    } else {
        ret = snd_soc_component_update_bits(
            component,
            ES8328_ADCCONTROL4,
            ES8328_ADCCONTROL4_ADCWL_MASK,
            (wl as c_uint) << ES8328_ADCCONTROL4_ADCWL_SHIFT,
        );
        if ret < 0 {
            return ret;
        }
    }

    ret = snd_soc_component_update_bits(component, reg, ES8328_RATEMASK, ratio as c_uint);
    if ret < 0 {
        return ret;
    }
    0
}

unsafe extern "C" fn es8328_set_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let mut mclkdiv2: c_int = 0;
    let round_freq: c_uint;

    /*
     * Allow a small tolerance for frequencies within 100hz. Note
     * this value is chosen arbitrarily.
     */
    round_freq = ((freq + 50) / 100) * 100;

    match round_freq {
        0 => {
            (*es8328).sysclk_constraints = ptr::null();
            (*es8328).mclk_ratios = ptr::null();
        }
        22579200 => {
            mclkdiv2 = 1;
            (*es8328).sysclk_constraints = &constraints_11289;
            (*es8328).mclk_ratios = ratios_11289.as_ptr();
        }
        11289600 => {
            (*es8328).sysclk_constraints = &constraints_11289;
            (*es8328).mclk_ratios = ratios_11289.as_ptr();
        }
        24576000 => {
            mclkdiv2 = 1;
            (*es8328).sysclk_constraints = &constraints_12288;
            (*es8328).mclk_ratios = ratios_12288.as_ptr();
        }
        12288000 => {
            (*es8328).sysclk_constraints = &constraints_12288;
            (*es8328).mclk_ratios = ratios_12288.as_ptr();
        }
        _ => return -EINVAL,
    }

    (*es8328).mclkdiv2 = mclkdiv2;
    0
}

unsafe extern "C" fn es8328_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let mut ret: c_int;
    let mut dac_mode: u8 = 0;
    let mut adc_mode: u8 = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            /* Master serial port mode, with BCLK generated automatically */
            ret = snd_soc_component_update_bits(
                component,
                ES8328_MASTERMODE,
                ES8328_MASTERMODE_MSC,
                ES8328_MASTERMODE_MSC,
            );
            if ret < 0 {
                return ret;
            }
            (*es8328).provider = true;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            /* Slave serial port mode */
            ret = snd_soc_component_update_bits(component, ES8328_MASTERMODE, ES8328_MASTERMODE_MSC, 0);
            if ret < 0 {
                return ret;
            }
            (*es8328).provider = false;
        }
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            dac_mode |= ES8328_DACCONTROL1_DACFORMAT_I2S as u8;
            adc_mode |= ES8328_ADCCONTROL4_ADCFORMAT_I2S as u8;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            dac_mode |= ES8328_DACCONTROL1_DACFORMAT_RJUST as u8;
            adc_mode |= ES8328_ADCCONTROL4_ADCFORMAT_RJUST as u8;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            dac_mode |= ES8328_DACCONTROL1_DACFORMAT_LJUST as u8;
            adc_mode |= ES8328_ADCCONTROL4_ADCFORMAT_LJUST as u8;
        }
        _ => return -EINVAL,
    }

    /* clock inversion */
    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF {
        return -EINVAL;
    }

    ret = snd_soc_component_update_bits(
        component,
        ES8328_DACCONTROL1,
        ES8328_DACCONTROL1_DACFORMAT_MASK,
        dac_mode as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        ES8328_ADCCONTROL4,
        ES8328_ADCCONTROL4_ADCFORMAT_MASK,
        adc_mode as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn es8328_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}

        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            /* VREF, VMID=2x50k, digital enabled */
            ret = snd_soc_component_write(component, ES8328_CHIPPOWER, 0);
            if ret < 0 {
                return ret;
            }

            ret = snd_soc_component_update_bits(
                component,
                ES8328_CONTROL1,
                ES8328_CONTROL1_VMIDSEL_MASK | ES8328_CONTROL1_ENREF,
                ES8328_CONTROL1_VMIDSEL_50k | ES8328_CONTROL1_ENREF,
            );
            if ret < 0 {
                return ret;
            }
        }

        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                ret = snd_soc_component_update_bits(
                    component,
                    ES8328_CONTROL1,
                    ES8328_CONTROL1_VMIDSEL_MASK | ES8328_CONTROL1_ENREF,
                    ES8328_CONTROL1_VMIDSEL_5k | ES8328_CONTROL1_ENREF,
                );
                if ret < 0 {
                    return ret;
                }

                /* Charge caps */
                msleep(100);
            }

            ret = snd_soc_component_write(
                component,
                ES8328_CONTROL2,
                ES8328_CONTROL2_OVERCURRENT_ON | ES8328_CONTROL2_THERMAL_SHUTDOWN_ON,
            );
            if ret < 0 {
                return ret;
            }

            /* VREF, VMID=2*500k, digital stopped */
            ret = snd_soc_component_update_bits(
                component,
                ES8328_CONTROL1,
                ES8328_CONTROL1_VMIDSEL_MASK | ES8328_CONTROL1_ENREF,
                ES8328_CONTROL1_VMIDSEL_500k | ES8328_CONTROL1_ENREF,
            );
            if ret < 0 {
                return ret;
            }
        }

        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            ret = snd_soc_component_update_bits(
                component,
                ES8328_CONTROL1,
                ES8328_CONTROL1_VMIDSEL_MASK | ES8328_CONTROL1_ENREF,
                0,
            );
            if ret < 0 {
                return ret;
            }
        }
    }
    0
}

static es8328_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(es8328_startup),
    hw_params: Some(es8328_hw_params),
    mute_stream: Some(es8328_mute),
    set_sysclk: Some(es8328_set_sysclk),
    set_fmt: Some(es8328_set_dai_fmt),
    no_capture_mute: 1,
};

static mut es8328_dai: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        name: b"es8328-hifi-analog\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: ES8328_RATES!(),
            formats: ES8328_FORMATS!(),
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Capture\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: ES8328_RATES!(),
            formats: ES8328_FORMATS!(),
        },
        ops: &es8328_dai_ops,
        symmetric_rate: 1,
    }
};

unsafe extern "C" fn es8328_suspend(component: *mut snd_soc_component) -> c_int {
    let es8328: *mut es8328_priv;
    let ret: c_int;

    es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;

    clk_disable_unprepare((*es8328).clk);

    ret = regulator_bulk_disable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"unable to disable regulators\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

unsafe extern "C" fn es8328_resume(component: *mut snd_soc_component) -> c_int {
    let es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;
    let mut ret: c_int;

    ret = clk_prepare_enable((*es8328).clk);
    if ret != 0 {
        dev_err((*component).dev, b"unable to enable clock\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = regulator_bulk_enable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"unable to enable regulators\n\0".as_ptr() as *const c_char);
        clk_disable_unprepare((*es8328).clk);
        return ret;
    }

    regcache_mark_dirty((*es8328).regmap);
    ret = regcache_sync((*es8328).regmap);
    if ret != 0 {
        dev_err((*component).dev, b"unable to sync regcache\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
        clk_disable_unprepare((*es8328).clk);
        return ret;
    }

    0
}

unsafe extern "C" fn es8328_component_probe(component: *mut snd_soc_component) -> c_int {
    let es8328: *mut es8328_priv;
    let mut ret: c_int;

    es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;

    ret = regulator_bulk_enable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"unable to enable regulators\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Setup clocks */
    (*es8328).clk = devm_clk_get((*component).dev, ptr::null());
    if IS_ERR((*es8328).clk as *const c_void) {
        dev_err((*component).dev, b"codec clock missing or invalid\n\0".as_ptr() as *const c_char);
        ret = PTR_ERR((*es8328).clk as *const c_void);
        regulator_bulk_disable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
        return ret;
    }

    ret = clk_prepare_enable((*es8328).clk);
    if ret != 0 {
        dev_err((*component).dev, b"unable to prepare codec clk\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn es8328_remove(component: *mut snd_soc_component) {
    let es8328: *mut es8328_priv;

    es8328 = snd_soc_component_get_drvdata(component) as *mut es8328_priv;

    clk_disable_unprepare((*es8328).clk);

    regulator_bulk_disable(ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
}

#[no_mangle]
pub static es8328_regmap_config: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        max_register: ES8328_REG_MAX,
        cache_type: REGCACHE_MAPLE,
        use_single_read: true,
        use_single_write: true,
    }
};
// EXPORT_SYMBOL_GPL(es8328_regmap_config);

static es8328_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(es8328_component_probe),
    remove: Some(es8328_remove),
    suspend: Some(es8328_suspend),
    resume: Some(es8328_resume),
    set_bias_level: Some(es8328_set_bias_level),
    controls: es8328_snd_controls.as_ptr(),
    num_controls: es8328_snd_controls.len() as c_uint,
    dapm_widgets: es8328_dapm_widgets.as_ptr(),
    num_dapm_widgets: es8328_dapm_widgets.len() as c_uint,
    dapm_routes: es8328_dapm_routes.as_ptr(),
    num_dapm_routes: es8328_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub unsafe extern "C" fn es8328_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let es8328: *mut es8328_priv;
    let mut ret: c_int;
    let mut i: usize;

    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    es8328 = devm_kzalloc(dev, core::mem::size_of::<es8328_priv>(), GFP_KERNEL) as *mut es8328_priv;
    if es8328.is_null() {
        return -ENOMEM;
    }

    (*es8328).regmap = regmap;

    i = 0;
    while i < (*es8328).supplies.len() {
        (*es8328).supplies[i].supply = supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, ES8328_SUPPLY_NUM_USIZE as c_int, (*es8328).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, b"unable to get regulators\n\0".as_ptr() as *const c_char);
        return ret;
    }

    dev_set_drvdata(dev, es8328 as *mut c_void);

    devm_snd_soc_register_component(dev, &es8328_component_driver, &mut es8328_dai, 1)
}
// EXPORT_SYMBOL_GPL(es8328_probe);

// MODULE_DESCRIPTION("ASoC ES8328 driver");
// MODULE_AUTHOR("Sean Cross <xobs@kosagi.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
