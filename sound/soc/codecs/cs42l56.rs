// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs42l56.rs -- CS42L56 ALSA SoC audio driver
 *
 * Copyright 2014 CirrusLogic, Inc.
 *
 * Author: Brian Austin <brian.austin@cirrus.com>
 */

/* Translated from cs42l56.c.  Linux, ALSA SoC, regmap, regulator, GPIO,
 * input, workqueue, module, and cs42l56 register definitions are provided by
 * external kernel bindings corresponding to the original includes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

const CS42L56_NUM_SUPPLIES: usize = 3;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}
#[repr(C)]
pub struct input_dev {
    pub name: *const c_char,
    pub phys: *const c_char,
    pub id: input_id,
    pub evbit: [c_ulong; 1],
    pub sndbit: [c_ulong; 1],
    pub event: Option<unsafe extern "C" fn(*mut input_dev, c_uint, c_uint, c_int) -> c_int>,
    pub dev: device_link,
}
#[repr(C)]
pub struct input_id {
    pub bustype: c_uint,
}
#[repr(C)]
pub struct device_link {
    pub parent: *mut device,
}
#[repr(C)]
pub struct work_struct {
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub no_capture_mute: c_uint,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
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
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
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
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct i2c_client {
    pub dev: device,
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
pub struct i2c_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}
#[repr(C)]
pub struct i2c_driver {
    pub driver: i2c_driver_driver,
    pub id_table: *const i2c_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut i2c_client)>,
}

pub type ssize_t = isize;
pub type size_t = usize;
pub type u32 = u32;
pub type u8 = u8;

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
struct cs42l56_platform_data {
    /* GPIO for Reset */
    gpio_nreset: *mut gpio_desc,

    /* MICBIAS Level. Check datasheet Pg48 */
    micbias_lvl: c_uint,

    /* Analog Input 1A Reference 0=Single 1=Pseudo-Differential */
    ain1a_ref_cfg: c_uint,

    /* Analog Input 2A Reference 0=Single 1=Pseudo-Differential */
    ain2a_ref_cfg: c_uint,

    /* Analog Input 1B Reference 0=Single 1=Pseudo-Differential */
    ain1b_ref_cfg: c_uint,

    /* Analog Input 2B Reference 0=Single 1=Pseudo-Differential */
    ain2b_ref_cfg: c_uint,

    /* Charge Pump Freq. Check datasheet Pg62 */
    chgfreq: c_uint,

    /* HighPass Filter Right Channel Corner Frequency */
    hpfb_freq: c_uint,

    /* HighPass Filter Left Channel Corner Frequency */
    hpfa_freq: c_uint,

    /* Adaptive Power Control for LO/HP */
    adaptive_pwr: c_uint,
}

static cs42l56_supply_names: [*const c_char; CS42L56_NUM_SUPPLIES] = [
    b"VA\0".as_ptr() as *const c_char,
    b"VCP\0".as_ptr() as *const c_char,
    b"VLDO\0".as_ptr() as *const c_char,
];

#[repr(C)]
struct cs42l56_private {
    regmap: *mut regmap,
    component: *mut snd_soc_component,
    dev: *mut device,
    pdata: cs42l56_platform_data,
    supplies: [regulator_bulk_data; CS42L56_NUM_SUPPLIES],
    mclk: u32,
    mclk_prediv: u8,
    mclk_div2: u8,
    mclk_ratio: u8,
    iface: u8,
    iface_fmt: u8,
    iface_inv: u8,
    /* Present when CONFIG_INPUT is enabled in the original C source. */
    beep: *mut input_dev,
    beep_work: work_struct,
    beep_rate: c_int,
}

static cs42l56_reg_defaults: [reg_default; 44] = [
    reg_default { reg: 3, def: 0x7f },   /* r03 - Power Ctl 1 */
    reg_default { reg: 4, def: 0xff },   /* r04 - Power Ctl 2 */
    reg_default { reg: 5, def: 0x00 },   /* ro5 - Clocking Ctl 1 */
    reg_default { reg: 6, def: 0x0b },   /* r06 - Clocking Ctl 2 */
    reg_default { reg: 7, def: 0x00 },   /* r07 - Serial Format */
    reg_default { reg: 8, def: 0x05 },   /* r08 - Class H Ctl */
    reg_default { reg: 9, def: 0x0c },   /* r09 - Misc Ctl */
    reg_default { reg: 10, def: 0x80 },  /* r0a - INT Status */
    reg_default { reg: 11, def: 0x00 },  /* r0b - Playback Ctl */
    reg_default { reg: 12, def: 0x0c },  /* r0c - DSP Mute Ctl */
    reg_default { reg: 13, def: 0x00 },  /* r0d - ADCA Mixer Volume */
    reg_default { reg: 14, def: 0x00 },  /* r0e - ADCB Mixer Volume */
    reg_default { reg: 15, def: 0x00 },  /* r0f - PCMA Mixer Volume */
    reg_default { reg: 16, def: 0x00 },  /* r10 - PCMB Mixer Volume */
    reg_default { reg: 17, def: 0x00 },  /* r11 - Analog Input Advisory Volume */
    reg_default { reg: 18, def: 0x00 },  /* r12 - Digital Input Advisory Volume */
    reg_default { reg: 19, def: 0x00 },  /* r13 - Master A Volume */
    reg_default { reg: 20, def: 0x00 },  /* r14 - Master B Volume */
    reg_default { reg: 21, def: 0x00 },  /* r15 - Beep Freq / On Time */
    reg_default { reg: 22, def: 0x00 },  /* r16 - Beep Volume / Off Time */
    reg_default { reg: 23, def: 0x00 },  /* r17 - Beep Tone Ctl */
    reg_default { reg: 24, def: 0x88 },  /* r18 - Tone Ctl */
    reg_default { reg: 25, def: 0x00 },  /* r19 - Channel Mixer & Swap */
    reg_default { reg: 26, def: 0x00 },  /* r1a - AIN Ref Config / ADC Mux */
    reg_default { reg: 27, def: 0xa0 },  /* r1b - High-Pass Filter Ctl */
    reg_default { reg: 28, def: 0x00 },  /* r1c - Misc ADC Ctl */
    reg_default { reg: 29, def: 0x00 },  /* r1d - Gain & Bias Ctl */
    reg_default { reg: 30, def: 0x00 },  /* r1e - PGAA Mux & Volume */
    reg_default { reg: 31, def: 0x00 },  /* r1f - PGAB Mux & Volume */
    reg_default { reg: 32, def: 0x00 },  /* r20 - ADCA Attenuator */
    reg_default { reg: 33, def: 0x00 },  /* r21 - ADCB Attenuator */
    reg_default { reg: 34, def: 0x00 },  /* r22 - ALC Enable & Attack Rate */
    reg_default { reg: 35, def: 0xbf },  /* r23 - ALC Release Rate */
    reg_default { reg: 36, def: 0x00 },  /* r24 - ALC Threshold */
    reg_default { reg: 37, def: 0x00 },  /* r25 - Noise Gate Ctl */
    reg_default { reg: 38, def: 0x00 },  /* r26 - ALC, Limiter, SFT, ZeroCross */
    reg_default { reg: 39, def: 0x00 },  /* r27 - Analog Mute, LO & HP Mux */
    reg_default { reg: 40, def: 0x00 },  /* r28 - HP A Volume */
    reg_default { reg: 41, def: 0x00 },  /* r29 - HP B Volume */
    reg_default { reg: 42, def: 0x00 },  /* r2a - LINEOUT A Volume */
    reg_default { reg: 43, def: 0x00 },  /* r2b - LINEOUT B Volume */
    reg_default { reg: 44, def: 0x00 },  /* r2c - Limit Threshold Ctl */
    reg_default { reg: 45, def: 0x7f },  /* r2d - Limiter Ctl & Release Rate */
    reg_default { reg: 46, def: 0x00 },  /* r2e - Limiter Attack Rate */
];

unsafe extern "C" fn cs42l56_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42L56_CHIP_ID_1..=CS42L56_LIM_ATTACK_RATE => true,
        _ => false,
    }
}

unsafe extern "C" fn cs42l56_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS42L56_INT_STATUS => true,
        _ => false,
    }
}

DECLARE_TLV_DB_SCALE!(beep_tlv, -5000, 200, 0);
DECLARE_TLV_DB_SCALE!(hl_tlv, -6000, 50, 0);
DECLARE_TLV_DB_SCALE!(adv_tlv, -10200, 50, 0);
DECLARE_TLV_DB_SCALE!(adc_tlv, -9600, 100, 0);
DECLARE_TLV_DB_SCALE!(tone_tlv, -1050, 150, 0);
DECLARE_TLV_DB_SCALE!(preamp_tlv, 0, 1000, 0);
DECLARE_TLV_DB_SCALE!(pga_tlv, -600, 50, 0);

DECLARE_TLV_DB_RANGE!(ngnb_tlv,
    0, 1, TLV_DB_SCALE_ITEM!(-8200, 600, 0),
    2, 5, TLV_DB_SCALE_ITEM!(-7600, 300, 0)
);
DECLARE_TLV_DB_RANGE!(ngb_tlv,
    0, 2, TLV_DB_SCALE_ITEM!(-6400, 600, 0),
    3, 7, TLV_DB_SCALE_ITEM!(-4600, 300, 0)
);
DECLARE_TLV_DB_RANGE!(alc_tlv,
    0, 2, TLV_DB_SCALE_ITEM!(-3000, 600, 0),
    3, 7, TLV_DB_SCALE_ITEM!(-1200, 300, 0)
);

macro_rules! cstr_array {
    ($($s:literal),* $(,)?) => { [$(concat!($s, "\0").as_ptr() as *const c_char),*] };
}

static beep_config_text: [*const c_char; 4] = cstr_array!["Off", "Single", "Multiple", "Continuous"];
static beep_config_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_TONE_CFG, 6, beep_config_text.len(), beep_config_text);

static beep_pitch_text: [*const c_char; 16] = cstr_array![
    "C4", "C5", "D5", "E5", "F5", "G5", "A5", "B5",
    "C6", "D6", "E6", "F6", "G6", "A6", "B6", "C7"
];
static beep_pitch_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_FREQ_ONTIME, 4, beep_pitch_text.len(), beep_pitch_text);

static beep_ontime_text: [*const c_char; 16] = cstr_array![
    "86 ms", "430 ms", "780 ms", "1.20 s", "1.50 s",
    "1.80 s", "2.20 s", "2.50 s", "2.80 s", "3.20 s",
    "3.50 s", "3.80 s", "4.20 s", "4.50 s", "4.80 s", "5.20 s"
];
static beep_ontime_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_FREQ_ONTIME, 0, beep_ontime_text.len(), beep_ontime_text);

static beep_offtime_text: [*const c_char; 8] = cstr_array![
    "1.23 s", "2.58 s", "3.90 s", "5.20 s", "6.60 s", "8.05 s", "9.35 s", "10.80 s"
];
static beep_offtime_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_FREQ_OFFTIME, 5, beep_offtime_text.len(), beep_offtime_text);

static beep_treble_text: [*const c_char; 4] = cstr_array!["5kHz", "7kHz", "10kHz", "15kHz"];
static beep_treble_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_TONE_CFG, 3, beep_treble_text.len(), beep_treble_text);

static beep_bass_text: [*const c_char; 4] = cstr_array!["50Hz", "100Hz", "200Hz", "250Hz"];
static beep_bass_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_BEEP_TONE_CFG, 1, beep_bass_text.len(), beep_bass_text);

static pgaa_mux_text: [*const c_char; 3] = cstr_array!["AIN1A", "AIN2A", "AIN3A"];
static pgaa_mux_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_PGAA_MUX_VOLUME, 0, pgaa_mux_text.len(), pgaa_mux_text);
static pgaa_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pgaa_mux_enum);

static pgab_mux_text: [*const c_char; 3] = cstr_array!["AIN1B", "AIN2B", "AIN3B"];
static pgab_mux_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_PGAB_MUX_VOLUME, 0, pgab_mux_text.len(), pgab_mux_text);
static pgab_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pgab_mux_enum);

static adca_mux_text: [*const c_char; 4] = cstr_array!["PGAA", "AIN1A", "AIN2A", "AIN3A"];
static adca_mux_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AIN_REFCFG_ADC_MUX, 0, adca_mux_text.len(), adca_mux_text);
static adca_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adca_mux_enum);

static adcb_mux_text: [*const c_char; 4] = cstr_array!["PGAB", "AIN1B", "AIN2B", "AIN3B"];
static adcb_mux_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AIN_REFCFG_ADC_MUX, 2, adcb_mux_text.len(), adcb_mux_text);
static adcb_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adcb_mux_enum);

static left_swap_text: [*const c_char; 3] = cstr_array!["Left", "LR 2", "Right"];
static right_swap_text: [*const c_char; 3] = cstr_array!["Right", "LR 2", "Left"];
static swap_values: [c_uint; 3] = [0, 1, 3];

static adca_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L56_CHAN_MIX_SWAP, 0, 3, left_swap_text.len(), left_swap_text, swap_values);
static adca_swap_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adca_swap_enum);
static pcma_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L56_CHAN_MIX_SWAP, 4, 3, left_swap_text.len(), left_swap_text, swap_values);
static pcma_swap_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pcma_swap_enum);
static adcb_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L56_CHAN_MIX_SWAP, 2, 3, right_swap_text.len(), right_swap_text, swap_values);
static adcb_swap_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", adcb_swap_enum);
static pcmb_swap_enum: soc_enum = SOC_VALUE_ENUM_SINGLE!(CS42L56_CHAN_MIX_SWAP, 6, 3, right_swap_text.len(), right_swap_text, swap_values);
static pcmb_swap_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", pcmb_swap_enum);

static hpa_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L56_PWRCTL_2, 6, 1, 1);
static hpb_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L56_PWRCTL_2, 4, 1, 1);
static loa_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L56_PWRCTL_2, 2, 1, 1);
static lob_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", CS42L56_PWRCTL_2, 0, 1, 1);

static hploa_input_text: [*const c_char; 2] = cstr_array!["DACA", "PGAA"];
static lineouta_input_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AMUTE_HPLO_MUX, 2, hploa_input_text.len(), hploa_input_text);
static lineouta_input: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", lineouta_input_enum);
static hpa_input_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AMUTE_HPLO_MUX, 0, hploa_input_text.len(), hploa_input_text);
static hpa_input: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", hpa_input_enum);

static hplob_input_text: [*const c_char; 2] = cstr_array!["DACB", "PGAB"];
static lineoutb_input_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AMUTE_HPLO_MUX, 3, hplob_input_text.len(), hplob_input_text);
static lineoutb_input: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", lineoutb_input_enum);
static hpb_input_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_AMUTE_HPLO_MUX, 1, hplob_input_text.len(), hplob_input_text);
static hpb_input: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", hpb_input_enum);

static dig_mux_text: [*const c_char; 2] = cstr_array!["ADC", "DSP"];
static dig_mux_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_MISC_CTL, 7, dig_mux_text.len(), dig_mux_text);
static dig_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Route", dig_mux_enum);

static hpf_freq_text: [*const c_char; 4] = cstr_array!["1.8Hz", "119Hz", "236Hz", "464Hz"];
static hpfa_freq_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_HPF_CTL, 0, hpf_freq_text.len(), hpf_freq_text);
static hpfb_freq_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_HPF_CTL, 2, hpf_freq_text.len(), hpf_freq_text);

static ng_delay_text: [*const c_char; 4] = cstr_array!["50ms", "100ms", "150ms", "200ms"];
static ng_delay_enum: soc_enum = SOC_ENUM_SINGLE!(CS42L56_NOISE_GATE_CTL, 0, ng_delay_text.len(), ng_delay_text);

static cs42l56_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R_SX_TLV!("Master Volume", CS42L56_MASTER_A_VOLUME, CS42L56_MASTER_B_VOLUME, 0, 0x34, 0xE4, adv_tlv),
    SOC_DOUBLE!("Master Mute Switch", CS42L56_DSP_MUTE_CTL, 0, 1, 1, 1),
    SOC_DOUBLE_R_SX_TLV!("ADC Mixer Volume", CS42L56_ADCA_MIX_VOLUME, CS42L56_ADCB_MIX_VOLUME, 0, 0x88, 0x90, hl_tlv),
    SOC_DOUBLE!("ADC Mixer Mute Switch", CS42L56_DSP_MUTE_CTL, 6, 7, 1, 1),
    SOC_DOUBLE_R_SX_TLV!("PCM Mixer Volume", CS42L56_PCMA_MIX_VOLUME, CS42L56_PCMB_MIX_VOLUME, 0, 0x88, 0x90, hl_tlv),
    SOC_DOUBLE!("PCM Mixer Mute Switch", CS42L56_DSP_MUTE_CTL, 4, 5, 1, 1),
    SOC_SINGLE_TLV!("Analog Advisory Volume", CS42L56_ANAINPUT_ADV_VOLUME, 0, 0x00, 1, adv_tlv),
    SOC_SINGLE_TLV!("Digital Advisory Volume", CS42L56_DIGINPUT_ADV_VOLUME, 0, 0x00, 1, adv_tlv),
    SOC_DOUBLE_R_SX_TLV!("PGA Volume", CS42L56_PGAA_MUX_VOLUME, CS42L56_PGAB_MUX_VOLUME, 0, 0x34, 0x24, pga_tlv),
    SOC_DOUBLE_R_TLV!("ADC Volume", CS42L56_ADCA_ATTENUATOR, CS42L56_ADCB_ATTENUATOR, 0, 0x00, 1, adc_tlv),
    SOC_DOUBLE!("ADC Mute Switch", CS42L56_MISC_ADC_CTL, 2, 3, 1, 1),
    SOC_DOUBLE!("ADC Boost Switch", CS42L56_GAIN_BIAS_CTL, 3, 2, 1, 1),
    SOC_DOUBLE_R_SX_TLV!("Headphone Volume", CS42L56_HPA_VOLUME, CS42L56_HPB_VOLUME, 0, 0x44, 0x48, hl_tlv),
    SOC_DOUBLE_R_SX_TLV!("LineOut Volume", CS42L56_LOA_VOLUME, CS42L56_LOB_VOLUME, 0, 0x44, 0x48, hl_tlv),
    SOC_SINGLE_TLV!("Bass Shelving Volume", CS42L56_TONE_CTL, 0, 0x00, 1, tone_tlv),
    SOC_SINGLE_TLV!("Treble Shelving Volume", CS42L56_TONE_CTL, 4, 0x00, 1, tone_tlv),
    SOC_DOUBLE_TLV!("PGA Preamp Volume", CS42L56_GAIN_BIAS_CTL, 4, 6, 0x02, 1, preamp_tlv),
    SOC_SINGLE!("DSP Switch", CS42L56_PLAYBACK_CTL, 7, 1, 1),
    SOC_SINGLE!("Gang Playback Switch", CS42L56_PLAYBACK_CTL, 4, 1, 1),
    SOC_SINGLE!("Gang ADC Switch", CS42L56_MISC_ADC_CTL, 7, 1, 1),
    SOC_SINGLE!("Gang PGA Switch", CS42L56_MISC_ADC_CTL, 6, 1, 1),
    SOC_SINGLE!("PCMA Invert", CS42L56_PLAYBACK_CTL, 2, 1, 1),
    SOC_SINGLE!("PCMB Invert", CS42L56_PLAYBACK_CTL, 3, 1, 1),
    SOC_SINGLE!("ADCA Invert", CS42L56_MISC_ADC_CTL, 2, 1, 1),
    SOC_SINGLE!("ADCB Invert", CS42L56_MISC_ADC_CTL, 3, 1, 1),
    SOC_DOUBLE!("HPF Switch", CS42L56_HPF_CTL, 5, 7, 1, 1),
    SOC_DOUBLE!("HPF Freeze Switch", CS42L56_HPF_CTL, 4, 6, 1, 1),
    SOC_ENUM!("HPFA Corner Freq", hpfa_freq_enum),
    SOC_ENUM!("HPFB Corner Freq", hpfb_freq_enum),
    SOC_SINGLE!("Analog Soft Ramp", CS42L56_MISC_CTL, 4, 1, 1),
    SOC_DOUBLE!("Analog Soft Ramp Disable", CS42L56_ALC_LIM_SFT_ZC, 7, 5, 1, 1),
    SOC_SINGLE!("Analog Zero Cross", CS42L56_MISC_CTL, 3, 1, 1),
    SOC_DOUBLE!("Analog Zero Cross Disable", CS42L56_ALC_LIM_SFT_ZC, 6, 4, 1, 1),
    SOC_SINGLE!("Digital Soft Ramp", CS42L56_MISC_CTL, 2, 1, 1),
    SOC_SINGLE!("Digital Soft Ramp Disable", CS42L56_ALC_LIM_SFT_ZC, 3, 1, 1),
    SOC_SINGLE!("HL Deemphasis", CS42L56_PLAYBACK_CTL, 6, 1, 1),
    SOC_SINGLE!("ALC Switch", CS42L56_ALC_EN_ATTACK_RATE, 6, 1, 1),
    SOC_SINGLE!("ALC Limit All Switch", CS42L56_ALC_RELEASE_RATE, 7, 1, 1),
    SOC_SINGLE_RANGE!("ALC Attack", CS42L56_ALC_EN_ATTACK_RATE, 0, 0, 0x3f, 0),
    SOC_SINGLE_RANGE!("ALC Release", CS42L56_ALC_RELEASE_RATE, 0, 0x3f, 0, 0),
    SOC_SINGLE_TLV!("ALC MAX", CS42L56_ALC_THRESHOLD, 5, 0x07, 1, alc_tlv),
    SOC_SINGLE_TLV!("ALC MIN", CS42L56_ALC_THRESHOLD, 2, 0x07, 1, alc_tlv),
    SOC_SINGLE!("Limiter Switch", CS42L56_LIM_CTL_RELEASE_RATE, 7, 1, 1),
    SOC_SINGLE!("Limit All Switch", CS42L56_LIM_CTL_RELEASE_RATE, 6, 1, 1),
    SOC_SINGLE_RANGE!("Limiter Attack", CS42L56_LIM_ATTACK_RATE, 0, 0, 0x3f, 0),
    SOC_SINGLE_RANGE!("Limiter Release", CS42L56_LIM_CTL_RELEASE_RATE, 0, 0x3f, 0, 0),
    SOC_SINGLE_TLV!("Limiter MAX", CS42L56_LIM_THRESHOLD_CTL, 5, 0x07, 1, alc_tlv),
    SOC_SINGLE_TLV!("Limiter Cushion", CS42L56_ALC_THRESHOLD, 2, 0x07, 1, alc_tlv),
    SOC_SINGLE!("NG Switch", CS42L56_NOISE_GATE_CTL, 6, 1, 1),
    SOC_SINGLE!("NG All Switch", CS42L56_NOISE_GATE_CTL, 7, 1, 1),
    SOC_SINGLE!("NG Boost Switch", CS42L56_NOISE_GATE_CTL, 5, 1, 1),
    SOC_SINGLE_TLV!("NG Unboost Threshold", CS42L56_NOISE_GATE_CTL, 2, 0x07, 1, ngnb_tlv),
    SOC_SINGLE_TLV!("NG Boost Threshold", CS42L56_NOISE_GATE_CTL, 2, 0x07, 1, ngb_tlv),
    SOC_ENUM!("NG Delay", ng_delay_enum),
    SOC_ENUM!("Beep Config", beep_config_enum),
    SOC_ENUM!("Beep Pitch", beep_pitch_enum),
    SOC_ENUM!("Beep on Time", beep_ontime_enum),
    SOC_ENUM!("Beep off Time", beep_offtime_enum),
    SOC_SINGLE_SX_TLV!("Beep Volume", CS42L56_BEEP_FREQ_OFFTIME, 0, 0x07, 0x23, beep_tlv),
    SOC_SINGLE!("Beep Tone Ctl Switch", CS42L56_BEEP_TONE_CFG, 0, 1, 1),
    SOC_ENUM!("Beep Treble Corner Freq", beep_treble_enum),
    SOC_ENUM!("Beep Bass Corner Freq", beep_bass_enum),
];

static cs42l56_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_SIGGEN!("Beep"),
    SND_SOC_DAPM_SUPPLY!("VBUF", CS42L56_PWRCTL_1, 5, 1, ptr::null(), 0),
    SND_SOC_DAPM_MICBIAS!("MIC1 Bias", CS42L56_PWRCTL_1, 4, 1),
    SND_SOC_DAPM_SUPPLY!("Charge Pump", CS42L56_PWRCTL_1, 3, 1, ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("AIN1A"), SND_SOC_DAPM_INPUT!("AIN2A"),
    SND_SOC_DAPM_INPUT!("AIN1B"), SND_SOC_DAPM_INPUT!("AIN2B"),
    SND_SOC_DAPM_INPUT!("AIN3A"), SND_SOC_DAPM_INPUT!("AIN3B"),
    SND_SOC_DAPM_AIF_OUT!("SDOUT", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("SDIN", ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("Digital Output Mux", SND_SOC_NOPM, 0, 0, &dig_mux),
    SND_SOC_DAPM_PGA!("PGAA", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_PGA!("PGAB", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MUX!("PGAA Input Mux", SND_SOC_NOPM, 0, 0, &pgaa_mux),
    SND_SOC_DAPM_MUX!("PGAB Input Mux", SND_SOC_NOPM, 0, 0, &pgab_mux),
    SND_SOC_DAPM_MUX!("ADCA Mux", SND_SOC_NOPM, 0, 0, &adca_mux),
    SND_SOC_DAPM_MUX!("ADCB Mux", SND_SOC_NOPM, 0, 0, &adcb_mux),
    SND_SOC_DAPM_ADC!("ADCA", ptr::null(), CS42L56_PWRCTL_1, 1, 1),
    SND_SOC_DAPM_ADC!("ADCB", ptr::null(), CS42L56_PWRCTL_1, 2, 1),
    SND_SOC_DAPM_MUX!("ADCA Swap Mux", SND_SOC_NOPM, 0, 0, &adca_swap_mux),
    SND_SOC_DAPM_MUX!("ADCB Swap Mux", SND_SOC_NOPM, 0, 0, &adcb_swap_mux),
    SND_SOC_DAPM_MUX!("PCMA Swap Mux", SND_SOC_NOPM, 0, 0, &pcma_swap_mux),
    SND_SOC_DAPM_MUX!("PCMB Swap Mux", SND_SOC_NOPM, 0, 0, &pcmb_swap_mux),
    SND_SOC_DAPM_DAC!("DACA", ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DACB", ptr::null(), SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!("HPA"), SND_SOC_DAPM_OUTPUT!("LOA"),
    SND_SOC_DAPM_OUTPUT!("HPB"), SND_SOC_DAPM_OUTPUT!("LOB"),
    SND_SOC_DAPM_SWITCH!("Headphone Right", CS42L56_PWRCTL_2, 4, 1, &hpb_switch),
    SND_SOC_DAPM_SWITCH!("Headphone Left", CS42L56_PWRCTL_2, 6, 1, &hpa_switch),
    SND_SOC_DAPM_SWITCH!("Lineout Right", CS42L56_PWRCTL_2, 0, 1, &lob_switch),
    SND_SOC_DAPM_SWITCH!("Lineout Left", CS42L56_PWRCTL_2, 2, 1, &loa_switch),
    SND_SOC_DAPM_MUX!("LINEOUTA Input Mux", SND_SOC_NOPM, 0, 0, &lineouta_input),
    SND_SOC_DAPM_MUX!("LINEOUTB Input Mux", SND_SOC_NOPM, 0, 0, &lineoutb_input),
    SND_SOC_DAPM_MUX!("HPA Input Mux", SND_SOC_NOPM, 0, 0, &hpa_input),
    SND_SOC_DAPM_MUX!("HPB Input Mux", SND_SOC_NOPM, 0, 0, &hpb_input),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: ptr::null(), source: concat!($source, "\0").as_ptr() as *const c_char }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: concat!($sink, "\0").as_ptr() as *const c_char, control: concat!($control, "\0").as_ptr() as *const c_char, source: concat!($source, "\0").as_ptr() as *const c_char }
    };
}

static cs42l56_audio_map: &[snd_soc_dapm_route] = &[
    route!("HiFi Capture", "DSP", "Digital Output Mux"), route!("HiFi Capture", "ADC", "Digital Output Mux"),
    route!("Digital Output Mux", NULL, "ADCA"), route!("Digital Output Mux", NULL, "ADCB"),
    route!("ADCB", NULL, "ADCB Swap Mux"), route!("ADCA", NULL, "ADCA Swap Mux"),
    route!("ADCA Swap Mux", NULL, "ADCA"), route!("ADCB Swap Mux", NULL, "ADCB"),
    route!("DACA", "Left", "ADCA Swap Mux"), route!("DACA", "LR 2", "ADCA Swap Mux"), route!("DACA", "Right", "ADCA Swap Mux"),
    route!("DACB", "Left", "ADCB Swap Mux"), route!("DACB", "LR 2", "ADCB Swap Mux"), route!("DACB", "Right", "ADCB Swap Mux"),
    route!("ADCA Mux", NULL, "AIN3A"), route!("ADCA Mux", NULL, "AIN2A"), route!("ADCA Mux", NULL, "AIN1A"), route!("ADCA Mux", NULL, "PGAA"),
    route!("ADCB Mux", NULL, "AIN3B"), route!("ADCB Mux", NULL, "AIN2B"), route!("ADCB Mux", NULL, "AIN1B"), route!("ADCB Mux", NULL, "PGAB"),
    route!("PGAA", "AIN1A", "PGAA Input Mux"), route!("PGAA", "AIN2A", "PGAA Input Mux"), route!("PGAA", "AIN3A", "PGAA Input Mux"),
    route!("PGAB", "AIN1B", "PGAB Input Mux"), route!("PGAB", "AIN2B", "PGAB Input Mux"), route!("PGAB", "AIN3B", "PGAB Input Mux"),
    route!("PGAA Input Mux", NULL, "AIN1A"), route!("PGAA Input Mux", NULL, "AIN2A"), route!("PGAA Input Mux", NULL, "AIN3A"),
    route!("PGAB Input Mux", NULL, "AIN1B"), route!("PGAB Input Mux", NULL, "AIN2B"), route!("PGAB Input Mux", NULL, "AIN3B"),
    route!("LOB", "Switch", "LINEOUTB Input Mux"), route!("LOA", "Switch", "LINEOUTA Input Mux"),
    route!("LINEOUTA Input Mux", "PGAA", "PGAA"), route!("LINEOUTB Input Mux", "PGAB", "PGAB"),
    route!("LINEOUTA Input Mux", "DACA", "DACA"), route!("LINEOUTB Input Mux", "DACB", "DACB"),
    route!("HPA", "Switch", "HPB Input Mux"), route!("HPB", "Switch", "HPA Input Mux"),
    route!("HPA Input Mux", "PGAA", "PGAA"), route!("HPB Input Mux", "PGAB", "PGAB"),
    route!("HPA Input Mux", "DACA", "DACA"), route!("HPB Input Mux", "DACB", "DACB"),
    route!("DACA", NULL, "PCMA Swap Mux"), route!("DACB", NULL, "PCMB Swap Mux"),
    route!("PCMB Swap Mux", "Left", "HiFi Playback"), route!("PCMB Swap Mux", "LR 2", "HiFi Playback"), route!("PCMB Swap Mux", "Right", "HiFi Playback"),
    route!("PCMA Swap Mux", "Left", "HiFi Playback"), route!("PCMA Swap Mux", "LR 2", "HiFi Playback"), route!("PCMA Swap Mux", "Right", "HiFi Playback"),
];

#[repr(C)]
struct cs42l56_clk_para {
    mclk: u32,
    srate: u32,
    ratio: u8,
}

static clk_ratio_table: [cs42l56_clk_para; 48] = [
    /* 8k */
    cs42l56_clk_para { mclk: 6000000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_768 },
    cs42l56_clk_para { mclk: 6144000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_750 },
    cs42l56_clk_para { mclk: 12000000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_768 },
    cs42l56_clk_para { mclk: 12288000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_750 },
    cs42l56_clk_para { mclk: 24000000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_768 },
    cs42l56_clk_para { mclk: 24576000, srate: 8000, ratio: CS42L56_MCLK_LRCLK_750 },
    /* 11.025k */
    cs42l56_clk_para { mclk: 5644800, srate: 11025, ratio: CS42L56_MCLK_LRCLK_512 },
    cs42l56_clk_para { mclk: 11289600, srate: 11025, ratio: CS42L56_MCLK_LRCLK_512 },
    cs42l56_clk_para { mclk: 22579200, srate: 11025, ratio: CS42L56_MCLK_LRCLK_512 },
    /* 11.0294k */
    cs42l56_clk_para { mclk: 6000000, srate: 110294, ratio: CS42L56_MCLK_LRCLK_544 },
    cs42l56_clk_para { mclk: 12000000, srate: 110294, ratio: CS42L56_MCLK_LRCLK_544 },
    cs42l56_clk_para { mclk: 24000000, srate: 110294, ratio: CS42L56_MCLK_LRCLK_544 },
    /* 12k */
    cs42l56_clk_para { mclk: 6000000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_500 },
    cs42l56_clk_para { mclk: 6144000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_512 },
    cs42l56_clk_para { mclk: 12000000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_500 },
    cs42l56_clk_para { mclk: 12288000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_512 },
    cs42l56_clk_para { mclk: 24000000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_500 },
    cs42l56_clk_para { mclk: 24576000, srate: 12000, ratio: CS42L56_MCLK_LRCLK_512 },
    /* 16k */
    cs42l56_clk_para { mclk: 6000000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_375 },
    cs42l56_clk_para { mclk: 6144000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_384 },
    cs42l56_clk_para { mclk: 12000000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_375 },
    cs42l56_clk_para { mclk: 12288000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_384 },
    cs42l56_clk_para { mclk: 24000000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_375 },
    cs42l56_clk_para { mclk: 24576000, srate: 16000, ratio: CS42L56_MCLK_LRCLK_384 },
    /* 22.050k */
    cs42l56_clk_para { mclk: 5644800, srate: 22050, ratio: CS42L56_MCLK_LRCLK_256 },
    cs42l56_clk_para { mclk: 11289600, srate: 22050, ratio: CS42L56_MCLK_LRCLK_256 },
    cs42l56_clk_para { mclk: 22579200, srate: 22050, ratio: CS42L56_MCLK_LRCLK_256 },
    /* 22.0588k */
    cs42l56_clk_para { mclk: 6000000, srate: 220588, ratio: CS42L56_MCLK_LRCLK_272 },
    cs42l56_clk_para { mclk: 12000000, srate: 220588, ratio: CS42L56_MCLK_LRCLK_272 },
    cs42l56_clk_para { mclk: 24000000, srate: 220588, ratio: CS42L56_MCLK_LRCLK_272 },
    /* 24k */
    cs42l56_clk_para { mclk: 6000000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_250 },
    cs42l56_clk_para { mclk: 6144000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_256 },
    cs42l56_clk_para { mclk: 12000000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_250 },
    cs42l56_clk_para { mclk: 12288000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_256 },
    cs42l56_clk_para { mclk: 24000000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_250 },
    cs42l56_clk_para { mclk: 24576000, srate: 24000, ratio: CS42L56_MCLK_LRCLK_256 },
    /* 32k */
    cs42l56_clk_para { mclk: 6000000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_187P5 },
    cs42l56_clk_para { mclk: 6144000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_192 },
    cs42l56_clk_para { mclk: 12000000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_187P5 },
    cs42l56_clk_para { mclk: 12288000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_192 },
    cs42l56_clk_para { mclk: 24000000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_187P5 },
    cs42l56_clk_para { mclk: 24576000, srate: 32000, ratio: CS42L56_MCLK_LRCLK_192 },
    /* 44.118k */
    cs42l56_clk_para { mclk: 6000000, srate: 44118, ratio: CS42L56_MCLK_LRCLK_136 },
    cs42l56_clk_para { mclk: 12000000, srate: 44118, ratio: CS42L56_MCLK_LRCLK_136 },
    cs42l56_clk_para { mclk: 24000000, srate: 44118, ratio: CS42L56_MCLK_LRCLK_136 },
    /* 44.1k */
    cs42l56_clk_para { mclk: 5644800, srate: 44100, ratio: CS42L56_MCLK_LRCLK_128 },
    cs42l56_clk_para { mclk: 11289600, srate: 44100, ratio: CS42L56_MCLK_LRCLK_128 },
    cs42l56_clk_para { mclk: 22579200, srate: 44100, ratio: CS42L56_MCLK_LRCLK_128 },
    /* 48k */
    cs42l56_clk_para { mclk: 6000000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_125 },
    cs42l56_clk_para { mclk: 6144000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_128 },
    cs42l56_clk_para { mclk: 12000000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_125 },
    cs42l56_clk_para { mclk: 12288000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_128 },
    cs42l56_clk_para { mclk: 24000000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_125 },
    cs42l56_clk_para { mclk: 24576000, srate: 48000, ratio: CS42L56_MCLK_LRCLK_128 },
];

unsafe fn cs42l56_get_mclk_ratio(mclk: c_int, rate: c_int) -> c_int {
    let mut i = 0usize;
    while i < clk_ratio_table.len() {
        if clk_ratio_table[i].mclk == mclk as u32 && clk_ratio_table[i].srate == rate as u32 {
            return clk_ratio_table[i].ratio as c_int;
        }
        i += 1;
    }
    -EINVAL
}

unsafe extern "C" fn cs42l56_set_sysclk(codec_dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, _dir: c_int) -> c_int {
    let component = (*codec_dai).component;
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;

    match freq {
        CS42L56_MCLK_5P6448MHZ | CS42L56_MCLK_6MHZ | CS42L56_MCLK_6P144MHZ => {
            (*cs42l56).mclk_div2 = 0;
            (*cs42l56).mclk_prediv = 0;
        }
        CS42L56_MCLK_11P2896MHZ | CS42L56_MCLK_12MHZ | CS42L56_MCLK_12P288MHZ => {
            (*cs42l56).mclk_div2 = CS42L56_MCLK_DIV2;
            (*cs42l56).mclk_prediv = 0;
        }
        CS42L56_MCLK_22P5792MHZ | CS42L56_MCLK_24MHZ | CS42L56_MCLK_24P576MHZ => {
            (*cs42l56).mclk_div2 = CS42L56_MCLK_DIV2;
            (*cs42l56).mclk_prediv = CS42L56_MCLK_PREDIV;
        }
        _ => return -EINVAL,
    }
    (*cs42l56).mclk = freq;

    snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_MCLK_PREDIV_MASK, (*cs42l56).mclk_prediv as c_uint);
    snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_MCLK_DIV2_MASK, (*cs42l56).mclk_div2 as c_uint);
    0
}

unsafe extern "C" fn cs42l56_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => (*cs42l56).iface = CS42L56_MASTER_MODE,
        SND_SOC_DAIFMT_CBC_CFC => (*cs42l56).iface = CS42L56_SLAVE_MODE,
        _ => return -EINVAL,
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => (*cs42l56).iface_fmt = CS42L56_DIG_FMT_I2S,
        SND_SOC_DAIFMT_LEFT_J => (*cs42l56).iface_fmt = CS42L56_DIG_FMT_LEFT_J,
        _ => return -EINVAL,
    }

    /* sclk inversion */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => (*cs42l56).iface_inv = 0,
        SND_SOC_DAIFMT_IB_NF => (*cs42l56).iface_inv = CS42L56_SCLK_INV,
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_MS_MODE_MASK, (*cs42l56).iface as c_uint);
    snd_soc_component_update_bits(component, CS42L56_SERIAL_FMT, CS42L56_DIG_FMT_MASK, (*cs42l56).iface_fmt as c_uint);
    snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_SCLK_INV_MASK, (*cs42l56).iface_inv as c_uint);
    0
}

unsafe extern "C" fn cs42l56_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let component = (*dai).component;
    if mute != 0 {
        /* Hit the DSP Mixer first */
        snd_soc_component_update_bits(component, CS42L56_DSP_MUTE_CTL,
            CS42L56_ADCAMIX_MUTE_MASK | CS42L56_ADCBMIX_MUTE_MASK | CS42L56_PCMAMIX_MUTE_MASK |
            CS42L56_PCMBMIX_MUTE_MASK | CS42L56_MSTB_MUTE_MASK | CS42L56_MSTA_MUTE_MASK,
            CS42L56_MUTE_ALL);
        /* Mute ADC's */
        snd_soc_component_update_bits(component, CS42L56_MISC_ADC_CTL, CS42L56_ADCA_MUTE_MASK | CS42L56_ADCB_MUTE_MASK, CS42L56_MUTE_ALL);
        /* HP And LO */
        snd_soc_component_update_bits(component, CS42L56_HPA_VOLUME, CS42L56_HP_MUTE_MASK, CS42L56_MUTE_ALL);
        snd_soc_component_update_bits(component, CS42L56_HPB_VOLUME, CS42L56_HP_MUTE_MASK, CS42L56_MUTE_ALL);
        snd_soc_component_update_bits(component, CS42L56_LOA_VOLUME, CS42L56_LO_MUTE_MASK, CS42L56_MUTE_ALL);
        snd_soc_component_update_bits(component, CS42L56_LOB_VOLUME, CS42L56_LO_MUTE_MASK, CS42L56_MUTE_ALL);
    } else {
        snd_soc_component_update_bits(component, CS42L56_DSP_MUTE_CTL,
            CS42L56_ADCAMIX_MUTE_MASK | CS42L56_ADCBMIX_MUTE_MASK | CS42L56_PCMAMIX_MUTE_MASK |
            CS42L56_PCMBMIX_MUTE_MASK | CS42L56_MSTB_MUTE_MASK | CS42L56_MSTA_MUTE_MASK,
            CS42L56_UNMUTE);
        snd_soc_component_update_bits(component, CS42L56_MISC_ADC_CTL, CS42L56_ADCA_MUTE_MASK | CS42L56_ADCB_MUTE_MASK, CS42L56_UNMUTE);
        snd_soc_component_update_bits(component, CS42L56_HPA_VOLUME, CS42L56_HP_MUTE_MASK, CS42L56_UNMUTE);
        snd_soc_component_update_bits(component, CS42L56_HPB_VOLUME, CS42L56_HP_MUTE_MASK, CS42L56_UNMUTE);
        snd_soc_component_update_bits(component, CS42L56_LOA_VOLUME, CS42L56_LO_MUTE_MASK, CS42L56_UNMUTE);
        snd_soc_component_update_bits(component, CS42L56_LOB_VOLUME, CS42L56_LO_MUTE_MASK, CS42L56_UNMUTE);
    }
    0
}

unsafe extern "C" fn cs42l56_pcm_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;
    let ratio = cs42l56_get_mclk_ratio((*cs42l56).mclk as c_int, params_rate(params));
    if ratio >= 0 {
        snd_soc_component_update_bits(component, CS42L56_CLKCTL_2, CS42L56_CLK_RATIO_MASK, ratio as c_uint);
    } else {
        dev_err((*component).dev, b"unsupported mclk/sclk/lrclk ratio\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    0
}

unsafe extern "C" fn cs42l56_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int {
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {
            snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_MCLK_DIS_MASK, 0);
            snd_soc_component_update_bits(component, CS42L56_PWRCTL_1, CS42L56_PDN_ALL_MASK, 0);
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == snd_soc_bias_level::SND_SOC_BIAS_OFF {
                regcache_cache_only((*cs42l56).regmap, false);
                regcache_sync((*cs42l56).regmap);
                ret = regulator_bulk_enable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err((*cs42l56).dev, b"Failed to enable regulators: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
            }
            snd_soc_component_update_bits(component, CS42L56_PWRCTL_1, CS42L56_PDN_ALL_MASK, 1);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            snd_soc_component_update_bits(component, CS42L56_PWRCTL_1, CS42L56_PDN_ALL_MASK, 1);
            snd_soc_component_update_bits(component, CS42L56_CLKCTL_1, CS42L56_MCLK_DIS_MASK, 1);
            regcache_cache_only((*cs42l56).regmap, true);
            regulator_bulk_disable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
        }
    }
    0
}

const CS42L56_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const CS42L56_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S18_3LE |
    SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static cs42l56_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(cs42l56_pcm_hw_params),
    mute_stream: Some(cs42l56_mute),
    set_fmt: Some(cs42l56_set_dai_fmt),
    set_sysclk: Some(cs42l56_set_sysclk),
    no_capture_mute: 1,
};

static mut cs42l56_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"cs42l56\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: CS42L56_RATES,
        formats: CS42L56_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"HiFi Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: CS42L56_RATES,
        formats: CS42L56_FORMATS,
    },
    ops: &cs42l56_ops,
};

static beep_freq: [c_int; 16] = [
    261, 522, 585, 667, 706, 774, 889, 1000,
    1043, 1200, 1333, 1412, 1600, 1714, 2000, 2182,
];

unsafe extern "C" fn cs42l56_beep_work(work: *mut work_struct) {
    let cs42l56 = container_of!(work, cs42l56_private, beep_work);
    let component = (*cs42l56).component;
    let dapm = snd_soc_component_to_dapm(component);
    let mut i: c_int;
    let mut val: c_int = 0;
    let mut best: c_int = 0;

    if (*cs42l56).beep_rate != 0 {
        i = 0;
        while (i as usize) < beep_freq.len() {
            if abs((*cs42l56).beep_rate - beep_freq[i as usize]) < abs((*cs42l56).beep_rate - beep_freq[best as usize]) {
                best = i;
            }
            i += 1;
        }
        dev_dbg((*component).dev, b"Set beep rate %dHz for requested %dHz\n\0".as_ptr() as *const c_char, beep_freq[best as usize], (*cs42l56).beep_rate);
        val = best << CS42L56_BEEP_RATE_SHIFT;
        snd_soc_dapm_enable_pin(dapm, b"Beep\0".as_ptr() as *const c_char);
    } else {
        dev_dbg((*component).dev, b"Disabling beep\n\0".as_ptr() as *const c_char);
        snd_soc_dapm_disable_pin(dapm, b"Beep\0".as_ptr() as *const c_char);
    }

    snd_soc_component_update_bits(component, CS42L56_BEEP_FREQ_ONTIME, CS42L56_BEEP_FREQ_MASK, val as c_uint);
    snd_soc_dapm_sync(dapm);
}

/* For usability define a way of injecting beep events for the device -
 * many systems will not have a keyboard.
 */
unsafe extern "C" fn cs42l56_beep_event(dev: *mut input_dev, _type: c_uint, code: c_uint, mut hz: c_int) -> c_int {
    let component = input_get_drvdata(dev) as *mut snd_soc_component;
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;

    dev_dbg((*component).dev, b"Beep event %x %x\n\0".as_ptr() as *const c_char, code, hz);

    match code {
        SND_BELL => {
            if hz != 0 {
                hz = 261;
            }
        }
        SND_TONE => {}
        _ => return -1,
    }

    /* Kick the beep from a workqueue */
    (*cs42l56).beep_rate = hz;
    schedule_work(&mut (*cs42l56).beep_work);
    0
}

unsafe extern "C" fn beep_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, count: size_t) -> ssize_t {
    let cs42l56 = dev_get_drvdata(dev) as *mut cs42l56_private;
    let mut time: c_long = 0;
    let ret = kstrtol(buf, 10, &mut time);
    if ret != 0 {
        return ret as ssize_t;
    }
    input_event((*cs42l56).beep, EV_SND, SND_TONE, time as c_int);
    count as ssize_t
}

DEVICE_ATTR_WO!(beep);

unsafe fn cs42l56_init_beep(component: *mut snd_soc_component) {
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;
    let mut ret: c_int;

    (*cs42l56).beep = devm_input_allocate_device((*component).dev);
    if (*cs42l56).beep.is_null() {
        dev_err((*component).dev, b"Failed to allocate beep device\n\0".as_ptr() as *const c_char);
        return;
    }

    INIT_WORK!(&mut (*cs42l56).beep_work, cs42l56_beep_work);
    (*cs42l56).beep_rate = 0;
    (*(*cs42l56).beep).name = b"CS42L56 Beep Generator\0".as_ptr() as *const c_char;
    (*(*cs42l56).beep).phys = dev_name((*component).dev);
    (*(*cs42l56).beep).id.bustype = BUS_I2C;
    (*(*cs42l56).beep).evbit[0] = BIT_MASK!(EV_SND);
    (*(*cs42l56).beep).sndbit[0] = BIT_MASK!(SND_BELL) | BIT_MASK!(SND_TONE);
    (*(*cs42l56).beep).event = Some(cs42l56_beep_event);
    (*(*cs42l56).beep).dev.parent = (*component).dev;
    input_set_drvdata((*cs42l56).beep, component as *mut core::ffi::c_void);

    ret = input_register_device((*cs42l56).beep);
    if ret != 0 {
        (*cs42l56).beep = ptr::null_mut();
        dev_err((*component).dev, b"Failed to register beep device\n\0".as_ptr() as *const c_char);
    }

    ret = device_create_file((*component).dev, &dev_attr_beep);
    if ret != 0 {
        dev_err((*component).dev, b"Failed to create keyclick file: %d\n\0".as_ptr() as *const c_char, ret);
    }
}

unsafe fn cs42l56_free_beep(component: *mut snd_soc_component) {
    let cs42l56 = snd_soc_component_get_drvdata(component) as *mut cs42l56_private;
    device_remove_file((*component).dev, &dev_attr_beep);
    cancel_work_sync(&mut (*cs42l56).beep_work);
    (*cs42l56).beep = ptr::null_mut();
    snd_soc_component_update_bits(component, CS42L56_BEEP_TONE_CFG, CS42L56_BEEP_EN_MASK, 0);
}

unsafe extern "C" fn cs42l56_probe(component: *mut snd_soc_component) -> c_int {
    cs42l56_init_beep(component);
    0
}

unsafe extern "C" fn cs42l56_remove(component: *mut snd_soc_component) {
    cs42l56_free_beep(component);
}

static soc_component_dev_cs42l56: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs42l56_probe),
    remove: Some(cs42l56_remove),
    set_bias_level: Some(cs42l56_set_bias_level),
    controls: cs42l56_snd_controls.as_ptr(),
    num_controls: cs42l56_snd_controls.len() as c_uint,
    dapm_widgets: cs42l56_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs42l56_dapm_widgets.len() as c_uint,
    dapm_routes: cs42l56_audio_map.as_ptr(),
    num_dapm_routes: cs42l56_audio_map.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static cs42l56_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS42L56_MAX_REGISTER,
    reg_defaults: cs42l56_reg_defaults.as_ptr(),
    num_reg_defaults: cs42l56_reg_defaults.len() as c_uint,
    readable_reg: Some(cs42l56_readable_register),
    volatile_reg: Some(cs42l56_volatile_register),
    cache_type: REGCACHE_MAPLE,
};

unsafe fn cs42l56_handle_of_data(i2c_client: *mut i2c_client, pdata: *mut cs42l56_platform_data) -> c_int {
    let np = (*i2c_client).dev.of_node;
    let mut val32: u32 = 0;

    if of_property_read_bool(np, b"cirrus,ain1a-reference-cfg\0".as_ptr() as *const c_char) {
        (*pdata).ain1a_ref_cfg = true as c_uint;
    }
    if of_property_read_bool(np, b"cirrus,ain2a-reference-cfg\0".as_ptr() as *const c_char) {
        (*pdata).ain2a_ref_cfg = true as c_uint;
    }
    if of_property_read_bool(np, b"cirrus,ain1b-reference-cfg\0".as_ptr() as *const c_char) {
        (*pdata).ain1b_ref_cfg = true as c_uint;
    }
    if of_property_read_bool(np, b"cirrus,ain2b-reference-cfg\0".as_ptr() as *const c_char) {
        (*pdata).ain2b_ref_cfg = true as c_uint;
    }
    if of_property_read_u32(np, b"cirrus,micbias-lvl\0".as_ptr() as *const c_char, &mut val32) >= 0 {
        (*pdata).micbias_lvl = val32;
    }
    if of_property_read_u32(np, b"cirrus,chgfreq-divisor\0".as_ptr() as *const c_char, &mut val32) >= 0 {
        (*pdata).chgfreq = val32;
    }
    if of_property_read_u32(np, b"cirrus,adaptive-pwr-cfg\0".as_ptr() as *const c_char, &mut val32) >= 0 {
        (*pdata).adaptive_pwr = val32;
    }
    if of_property_read_u32(np, b"cirrus,hpf-left-freq\0".as_ptr() as *const c_char, &mut val32) >= 0 {
        (*pdata).hpfa_freq = val32;
    }
    if of_property_read_u32(np, b"cirrus,hpf-left-freq\0".as_ptr() as *const c_char, &mut val32) >= 0 {
        (*pdata).hpfb_freq = val32;
    }

    (*pdata).gpio_nreset = devm_gpiod_get_optional(&mut (*i2c_client).dev, b"cirrus,gpio-nreset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*pdata).gpio_nreset as *const core::ffi::c_void) {
        return PTR_ERR((*pdata).gpio_nreset as *const core::ffi::c_void);
    }
    gpiod_set_consumer_name((*pdata).gpio_nreset, b"CS42L56 /RST\0".as_ptr() as *const c_char);
    0
}

unsafe extern "C" fn cs42l56_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut devid: c_uint;
    let mut alpha_rev: c_uint;
    let mut metal_rev: c_uint;
    let mut reg: c_uint = 0;

    let cs42l56 = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<cs42l56_private>(), GFP_KERNEL) as *mut cs42l56_private;
    if cs42l56.is_null() {
        return -ENOMEM;
    }
    (*cs42l56).dev = &mut (*i2c_client).dev;

    (*cs42l56).regmap = devm_regmap_init_i2c(i2c_client, &cs42l56_regmap);
    if IS_ERR((*cs42l56).regmap as *const core::ffi::c_void) {
        ret = PTR_ERR((*cs42l56).regmap as *const core::ffi::c_void);
        dev_err(&mut (*i2c_client).dev, b"regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if !(*i2c_client).dev.of_node.is_null() {
        ret = cs42l56_handle_of_data(i2c_client, &mut (*cs42l56).pdata);
        if ret != 0 {
            return ret;
        }
    }

    if !(*cs42l56).pdata.gpio_nreset.is_null() {
        gpiod_set_value_cansleep((*cs42l56).pdata.gpio_nreset, 1);
        gpiod_set_value_cansleep((*cs42l56).pdata.gpio_nreset, 0);
    }

    i2c_set_clientdata(i2c_client, cs42l56 as *mut core::ffi::c_void);

    let mut i = 0usize;
    while i < (*cs42l56).supplies.len() {
        (*cs42l56).supplies[i].supply = cs42l56_supply_names[i];
        i += 1;
    }

    ret = devm_regulator_bulk_get(&mut (*i2c_client).dev, (*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regulator_bulk_enable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = regmap_read((*cs42l56).regmap, CS42L56_CHIP_ID_1, &mut reg);
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, b"Failed to read chip ID: %d\n\0".as_ptr() as *const c_char, ret);
        regulator_bulk_disable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
        return ret;
    }

    devid = reg & CS42L56_CHIP_ID_MASK;
    if devid != CS42L56_DEVID {
        dev_err(&mut (*i2c_client).dev, b"CS42L56 Device ID (%X). Expected %X\n\0".as_ptr() as *const c_char, devid, CS42L56_DEVID);
        ret = -EINVAL;
        regulator_bulk_disable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
        return ret;
    }
    alpha_rev = reg & CS42L56_AREV_MASK;
    metal_rev = reg & CS42L56_MTLREV_MASK;

    dev_info(&mut (*i2c_client).dev, b"Cirrus Logic CS42L56 \0".as_ptr() as *const c_char);
    dev_info(&mut (*i2c_client).dev, b"Alpha Rev %X Metal Rev %X\n\0".as_ptr() as *const c_char, alpha_rev, metal_rev);

    if (*cs42l56).pdata.ain1a_ref_cfg != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_AIN_REFCFG_ADC_MUX, CS42L56_AIN1A_REF_MASK, CS42L56_AIN1A_REF_MASK);
    }
    if (*cs42l56).pdata.ain1b_ref_cfg != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_AIN_REFCFG_ADC_MUX, CS42L56_AIN1B_REF_MASK, CS42L56_AIN1B_REF_MASK);
    }
    if (*cs42l56).pdata.ain2a_ref_cfg != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_AIN_REFCFG_ADC_MUX, CS42L56_AIN2A_REF_MASK, CS42L56_AIN2A_REF_MASK);
    }
    if (*cs42l56).pdata.ain2b_ref_cfg != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_AIN_REFCFG_ADC_MUX, CS42L56_AIN2B_REF_MASK, CS42L56_AIN2B_REF_MASK);
    }
    if (*cs42l56).pdata.micbias_lvl != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_GAIN_BIAS_CTL, CS42L56_MIC_BIAS_MASK, (*cs42l56).pdata.micbias_lvl);
    }
    if (*cs42l56).pdata.chgfreq != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_CLASSH_CTL, CS42L56_CHRG_FREQ_MASK, (*cs42l56).pdata.chgfreq);
    }
    if (*cs42l56).pdata.hpfb_freq != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_HPF_CTL, CS42L56_HPFB_FREQ_MASK, (*cs42l56).pdata.hpfb_freq);
    }
    if (*cs42l56).pdata.hpfa_freq != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_HPF_CTL, CS42L56_HPFA_FREQ_MASK, (*cs42l56).pdata.hpfa_freq);
    }
    if (*cs42l56).pdata.adaptive_pwr != 0 {
        regmap_update_bits((*cs42l56).regmap, CS42L56_CLASSH_CTL, CS42L56_ADAPT_PWR_MASK, (*cs42l56).pdata.adaptive_pwr);
    }

    ret = devm_snd_soc_register_component(&mut (*i2c_client).dev, &soc_component_dev_cs42l56, &mut cs42l56_dai, 1);
    if ret < 0 {
        regulator_bulk_disable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
        return ret;
    }
    0
}

unsafe extern "C" fn cs42l56_i2c_remove(client: *mut i2c_client) {
    let cs42l56 = i2c_get_clientdata(client) as *mut cs42l56_private;
    regulator_bulk_disable((*cs42l56).supplies.len() as c_uint, (*cs42l56).supplies.as_mut_ptr());
}

static cs42l56_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"cirrus,cs42l56\0".as_ptr() as *const c_char },
    of_device_id { compatible: ptr::null() },
];
MODULE_DEVICE_TABLE!(of, cs42l56_of_match);

static cs42l56_id: [i2c_device_id; 2] = [
    i2c_device_id { name: b"cs42l56\0".as_ptr() as *const c_char },
    i2c_device_id { name: ptr::null() },
];
MODULE_DEVICE_TABLE!(i2c, cs42l56_id);

static mut cs42l56_i2c_driver: i2c_driver = i2c_driver {
    driver: i2c_driver_driver {
        name: b"cs42l56\0".as_ptr() as *const c_char,
        of_match_table: cs42l56_of_match.as_ptr(),
    },
    id_table: cs42l56_id.as_ptr(),
    probe: Some(cs42l56_i2c_probe),
    remove: Some(cs42l56_i2c_remove),
};

module_i2c_driver!(cs42l56_i2c_driver);

MODULE_DESCRIPTION!("ASoC CS42L56 driver");
MODULE_AUTHOR!("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
MODULE_LICENSE!("GPL");

extern "C" {
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static BUS_I2C: c_uint;
    static EV_SND: c_uint;
    static SND_BELL: c_uint;
    static SND_TONE: c_uint;
    static SND_SOC_NOPM: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S18_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulong;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regulator_bulk_enable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn snd_soc_dapm_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn abs(i: c_int) -> c_int;
    fn input_get_drvdata(dev: *mut input_dev) -> *mut core::ffi::c_void;
    fn schedule_work(work: *mut work_struct) -> bool;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn kstrtol(s: *const c_char, base: c_uint, res: *mut c_long) -> c_int;
    fn input_event(dev: *mut input_dev, type_: c_uint, code: c_uint, value: c_int);
    fn devm_input_allocate_device(dev: *mut device) -> *mut input_dev;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn input_set_drvdata(dev: *mut input_dev, data: *mut core::ffi::c_void);
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn device_create_file(dev: *mut device, attr: *const device_attribute) -> c_int;
    fn device_remove_file(dev: *mut device, attr: *const device_attribute);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut core::ffi::c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut core::ffi::c_void);
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_uint, consumers: *mut regulator_bulk_data) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct device_attribute {
    _private: [u8; 0],
}

extern "C" {
    static dev_attr_beep: device_attribute;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
