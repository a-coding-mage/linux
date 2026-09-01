// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017, Maxim Integrated
//
// Rust translation of soc/codecs/max98373.c. C include dependencies are expected
// to be supplied by the surrounding translated kernel/ASoC bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

extern "C" {
    static mut EACCES: c_int;

    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut max98373_priv;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_get_volsw(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn msleep(msecs: c_uint);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn pm_runtime_resume(dev: *mut device) -> c_int;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
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
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
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
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
}

#[repr(C)]
pub struct max98373_priv {
    pub regmap: *mut regmap,
    pub tdm_mode: bool,
    pub cache_num: c_int,
    pub cache: *mut max98373_cache,
    pub i_slot: c_int,
    pub v_slot: c_int,
    pub spkfb_slot: c_int,
    pub interleave_mode: bool,
    pub reset: *mut gpio_desc,
}

#[repr(C)]
pub struct max98373_cache {
    pub reg: c_uint,
    pub val: c_long,
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
pub struct gpio_desc {
    _private: [u8; 0],
}

unsafe extern "C" fn max98373_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98373 = snd_soc_component_get_drvdata(component);

    match event as c_uint {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits(
                (*max98373).regmap,
                MAX98373_R20FF_GLOBAL_SHDN,
                MAX98373_GLOBAL_EN_MASK,
                1,
            );
            usleep_range(30000, 31000);
        }
        SND_SOC_DAPM_PRE_PMD => {
            regmap_update_bits(
                (*max98373).regmap,
                MAX98373_R20FF_GLOBAL_SHDN,
                MAX98373_GLOBAL_EN_MASK,
                0,
            );
            usleep_range(30000, 31000);
            (*max98373).tdm_mode = false;
        }
        _ => return 0,
    }
    0
}

static max98373_switch_text: [*const c_char; 3] = [
    b"Left\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"LeftRight\0".as_ptr() as *const c_char,
];

static dai_sel_enum: soc_enum = SOC_ENUM_SINGLE!(
    MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1,
    MAX98373_PCM_TO_SPK_MONOMIX_CFG_SHIFT,
    3,
    max98373_switch_text
);

static max98373_dai_controls: snd_kcontrol_new = SOC_DAPM_ENUM!(b"DAI Sel\0", dai_sel_enum);
static max98373_vi_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(b"Switch\0", MAX98373_R202C_PCM_TX_EN, 0, 1, 0);
static max98373_spkfb_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!(b"Switch\0", MAX98373_R2043_AMP_EN, 1, 1, 0);

static max98373_dapm_widgets: [snd_soc_dapm_widget; 11] = [
    SND_SOC_DAPM_DAC_E!(
        b"Amp Enable\0",
        b"HiFi Playback\0",
        MAX98373_R202B_PCM_RX_EN,
        0,
        0,
        max98373_dac_event,
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_MUX!(b"DAI Sel Mux\0", SND_SOC_NOPM, 0, 0, &max98373_dai_controls),
    SND_SOC_DAPM_OUTPUT!(b"BE_OUT\0"),
    SND_SOC_DAPM_AIF_OUT!(
        b"Voltage Sense\0",
        b"HiFi Capture\0",
        0,
        MAX98373_R2047_IV_SENSE_ADC_EN,
        0,
        0
    ),
    SND_SOC_DAPM_AIF_OUT!(
        b"Current Sense\0",
        b"HiFi Capture\0",
        0,
        MAX98373_R2047_IV_SENSE_ADC_EN,
        1,
        0
    ),
    SND_SOC_DAPM_AIF_OUT!(b"Speaker FB Sense\0", b"HiFi Capture\0", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!(b"VI Sense\0", SND_SOC_NOPM, 0, 0, &max98373_vi_control),
    SND_SOC_DAPM_SWITCH!(b"SpkFB Sense\0", SND_SOC_NOPM, 0, 0, &max98373_spkfb_control),
    SND_SOC_DAPM_SIGGEN!(b"VMON\0"),
    SND_SOC_DAPM_SIGGEN!(b"IMON\0"),
    SND_SOC_DAPM_SIGGEN!(b"FBMON\0"),
];

static max98373_digital_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-6350, 50, 1);
static max98373_spk_tlv: [c_uint; 8] = DECLARE_TLV_DB_RANGE!(
    0, 8, TLV_DB_SCALE_ITEM!(0, 50, 0),
    9, 10, TLV_DB_SCALE_ITEM!(500, 100, 0)
);
static max98373_spkgain_max_tlv: [c_uint; 4] =
    DECLARE_TLV_DB_RANGE!(0, 9, TLV_DB_SCALE_ITEM!(800, 100, 0));
static max98373_dht_step_size_tlv: [c_uint; 8] = DECLARE_TLV_DB_RANGE!(
    0, 1, TLV_DB_SCALE_ITEM!(25, 25, 0),
    2, 4, TLV_DB_SCALE_ITEM!(100, 100, 0)
);
static max98373_dht_spkgain_min_tlv: [c_uint; 4] =
    DECLARE_TLV_DB_RANGE!(0, 9, TLV_DB_SCALE_ITEM!(800, 100, 0));
static max98373_dht_rotation_point_tlv: [c_uint; 24] = DECLARE_TLV_DB_RANGE!(
    0, 1, TLV_DB_SCALE_ITEM!(-3000, 500, 0),
    2, 4, TLV_DB_SCALE_ITEM!(-2200, 200, 0),
    5, 6, TLV_DB_SCALE_ITEM!(-1500, 300, 0),
    7, 9, TLV_DB_SCALE_ITEM!(-1000, 200, 0),
    10, 13, TLV_DB_SCALE_ITEM!(-500, 100, 0),
    14, 15, TLV_DB_SCALE_ITEM!(-100, 50, 0)
);
static max98373_limiter_thresh_tlv: [c_uint; 4] =
    DECLARE_TLV_DB_RANGE!(0, 15, TLV_DB_SCALE_ITEM!(-1500, 100, 0));
static max98373_bde_gain_tlv: [c_uint; 4] =
    DECLARE_TLV_DB_RANGE!(0, 60, TLV_DB_SCALE_ITEM!(-1500, 25, 0));

static max98373_output_voltage_lvl_text: [*const c_char; 10] = [
    b"5.43V\0".as_ptr() as *const c_char,
    b"6.09V\0".as_ptr() as *const c_char,
    b"6.83V\0".as_ptr() as *const c_char,
    b"7.67V\0".as_ptr() as *const c_char,
    b"8.60V\0".as_ptr() as *const c_char,
    b"9.65V\0".as_ptr() as *const c_char,
    b"10.83V\0".as_ptr() as *const c_char,
    b"12.15V\0".as_ptr() as *const c_char,
    b"13.63V\0".as_ptr() as *const c_char,
    b"15.29V\0".as_ptr() as *const c_char,
];

static max98373_out_volt_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(MAX98373_R203E_AMP_PATH_GAIN, 0, max98373_output_voltage_lvl_text);

static max98373_dht_attack_rate_text: [*const c_char; 8] = [
    b"17.5us\0".as_ptr() as *const c_char,
    b"35us\0".as_ptr() as *const c_char,
    b"70us\0".as_ptr() as *const c_char,
    b"140us\0".as_ptr() as *const c_char,
    b"280us\0".as_ptr() as *const c_char,
    b"560us\0".as_ptr() as *const c_char,
    b"1120us\0".as_ptr() as *const c_char,
    b"2240us\0".as_ptr() as *const c_char,
];

static max98373_dht_attack_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98373_R20D2_DHT_ATTACK_CFG,
    0,
    max98373_dht_attack_rate_text
);

static max98373_dht_release_rate_text: [*const c_char; 8] = [
    b"45ms\0".as_ptr() as *const c_char,
    b"225ms\0".as_ptr() as *const c_char,
    b"450ms\0".as_ptr() as *const c_char,
    b"1150ms\0".as_ptr() as *const c_char,
    b"2250ms\0".as_ptr() as *const c_char,
    b"3100ms\0".as_ptr() as *const c_char,
    b"4500ms\0".as_ptr() as *const c_char,
    b"6750ms\0".as_ptr() as *const c_char,
];

static max98373_dht_release_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98373_R20D3_DHT_RELEASE_CFG,
    0,
    max98373_dht_release_rate_text
);

static max98373_limiter_attack_rate_text: [*const c_char; 16] = [
    b"10us\0".as_ptr() as *const c_char,
    b"20us\0".as_ptr() as *const c_char,
    b"40us\0".as_ptr() as *const c_char,
    b"80us\0".as_ptr() as *const c_char,
    b"160us\0".as_ptr() as *const c_char,
    b"320us\0".as_ptr() as *const c_char,
    b"640us\0".as_ptr() as *const c_char,
    b"1.28ms\0".as_ptr() as *const c_char,
    b"2.56ms\0".as_ptr() as *const c_char,
    b"5.12ms\0".as_ptr() as *const c_char,
    b"10.24ms\0".as_ptr() as *const c_char,
    b"20.48ms\0".as_ptr() as *const c_char,
    b"40.96ms\0".as_ptr() as *const c_char,
    b"81.92ms\0".as_ptr() as *const c_char,
    b"16.384ms\0".as_ptr() as *const c_char,
    b"32.768ms\0".as_ptr() as *const c_char,
];

static max98373_limiter_attack_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98373_R20E1_LIMITER_ATK_REL_RATES,
    4,
    max98373_limiter_attack_rate_text
);

static max98373_limiter_release_rate_text: [*const c_char; 16] = [
    b"40us\0".as_ptr() as *const c_char,
    b"80us\0".as_ptr() as *const c_char,
    b"160us\0".as_ptr() as *const c_char,
    b"320us\0".as_ptr() as *const c_char,
    b"640us\0".as_ptr() as *const c_char,
    b"1.28ms\0".as_ptr() as *const c_char,
    b"2.56ms\0".as_ptr() as *const c_char,
    b"5.120ms\0".as_ptr() as *const c_char,
    b"10.24ms\0".as_ptr() as *const c_char,
    b"20.48ms\0".as_ptr() as *const c_char,
    b"40.96ms\0".as_ptr() as *const c_char,
    b"81.92ms\0".as_ptr() as *const c_char,
    b"163.84ms\0".as_ptr() as *const c_char,
    b"327.68ms\0".as_ptr() as *const c_char,
    b"655.36ms\0".as_ptr() as *const c_char,
    b"1310.72ms\0".as_ptr() as *const c_char,
];

static max98373_limiter_release_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98373_R20E1_LIMITER_ATK_REL_RATES,
    0,
    max98373_limiter_release_rate_text
);

static max98373_ADC_samplerate_text: [*const c_char; 4] = [
    b"333kHz\0".as_ptr() as *const c_char,
    b"192kHz\0".as_ptr() as *const c_char,
    b"64kHz\0".as_ptr() as *const c_char,
    b"48kHz\0".as_ptr() as *const c_char,
];

static max98373_adc_samplerate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98373_R2051_MEAS_ADC_SAMPLING_RATE,
    0,
    max98373_ADC_samplerate_text
);

unsafe extern "C" fn max98373_feedback_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let max98373 = snd_soc_component_get_drvdata(component);
    let mut i: c_int;

    if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF as c_int {
        /*
         * Register values will be cached before suspend. The cached value
         * will be a valid value and userspace will happy with that.
         */
        i = 0;
        while i < (*max98373).cache_num {
            let cache = (*max98373).cache.add(i as usize);
            if (*mc).reg == (*cache).reg {
                (*ucontrol).value.integer.value[0] = (*cache).val;
                return 0;
            }
            i += 1;
        }
    }

    snd_soc_get_volsw(kcontrol, ucontrol)
}

static max98373_snd_controls: [snd_kcontrol_new; 58] = [
    SOC_SINGLE!(b"Digital Vol Sel Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_VOL_SEL_SHIFT, 1, 0),
    SOC_SINGLE!(b"Volume Location Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_VOL_SEL_SHIFT, 1, 0),
    SOC_SINGLE!(b"Ramp Up Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_DSP_CFG_RMP_UP_SHIFT, 1, 0),
    SOC_SINGLE!(b"Ramp Down Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_DSP_CFG_RMP_DN_SHIFT, 1, 0),
    /* Speaker Amplifier Overcurrent Automatic Restart Enable */
    SOC_SINGLE!(b"OVC Autorestart Switch\0", MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, MAX98373_OVC_AUTORESTART_SHIFT, 1, 0),
    /* Thermal Shutdown Automatic Restart Enable */
    SOC_SINGLE!(b"THERM Autorestart Switch\0", MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, MAX98373_THERM_AUTORESTART_SHIFT, 1, 0),
    /* Clock Monitor Automatic Restart Enable */
    SOC_SINGLE!(b"CMON Autorestart Switch\0", MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, MAX98373_CMON_AUTORESTART_SHIFT, 1, 0),
    SOC_SINGLE!(b"CLK Monitor Switch\0", MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, MAX98373_CLOCK_MON_SHIFT, 1, 0),
    SOC_SINGLE!(b"Dither Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_DSP_CFG_DITH_SHIFT, 1, 0),
    SOC_SINGLE!(b"DC Blocker Switch\0", MAX98373_R203F_AMP_DSP_CFG, MAX98373_AMP_DSP_CFG_DCBLK_SHIFT, 1, 0),
    SOC_SINGLE_TLV!(b"Digital Volume\0", MAX98373_R203D_AMP_DIG_VOL_CTRL, 0, 0x7F, 1, max98373_digital_tlv),
    SOC_SINGLE_TLV!(b"Speaker Volume\0", MAX98373_R203E_AMP_PATH_GAIN, MAX98373_SPK_DIGI_GAIN_SHIFT, 10, 0, max98373_spk_tlv),
    SOC_SINGLE_TLV!(b"FS Max Volume\0", MAX98373_R203E_AMP_PATH_GAIN, MAX98373_FS_GAIN_MAX_SHIFT, 9, 0, max98373_spkgain_max_tlv),
    SOC_ENUM!(b"Output Voltage\0", max98373_out_volt_enum),
    /* Dynamic Headroom Tracking */
    SOC_SINGLE!(b"DHT Switch\0", MAX98373_R20D4_DHT_EN, MAX98373_DHT_EN_SHIFT, 1, 0),
    SOC_SINGLE_TLV!(b"DHT Min Volume\0", MAX98373_R20D1_DHT_CFG, MAX98373_DHT_SPK_GAIN_MIN_SHIFT, 9, 0, max98373_dht_spkgain_min_tlv),
    SOC_SINGLE_TLV!(b"DHT Rot Pnt Volume\0", MAX98373_R20D1_DHT_CFG, MAX98373_DHT_ROT_PNT_SHIFT, 15, 1, max98373_dht_rotation_point_tlv),
    SOC_SINGLE_TLV!(b"DHT Attack Step Volume\0", MAX98373_R20D2_DHT_ATTACK_CFG, MAX98373_DHT_ATTACK_STEP_SHIFT, 4, 0, max98373_dht_step_size_tlv),
    SOC_SINGLE_TLV!(b"DHT Release Step Volume\0", MAX98373_R20D3_DHT_RELEASE_CFG, MAX98373_DHT_RELEASE_STEP_SHIFT, 4, 0, max98373_dht_step_size_tlv),
    SOC_ENUM!(b"DHT Attack Rate\0", max98373_dht_attack_rate_enum),
    SOC_ENUM!(b"DHT Release Rate\0", max98373_dht_release_rate_enum),
    /* ADC configuration */
    SOC_SINGLE!(b"ADC PVDD CH Switch\0", MAX98373_R2056_MEAS_ADC_PVDD_CH_EN, 0, 1, 0),
    SOC_SINGLE!(b"ADC PVDD FLT Switch\0", MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG, MAX98373_FLT_EN_SHIFT, 1, 0),
    SOC_SINGLE!(b"ADC TEMP FLT Switch\0", MAX98373_R2053_MEAS_ADC_THERM_FLT_CFG, MAX98373_FLT_EN_SHIFT, 1, 0),
    SOC_SINGLE_EXT!(b"ADC PVDD\0", MAX98373_R2054_MEAS_ADC_PVDD_CH_READBACK, 0, 0xFF, 0, max98373_feedback_get, None),
    SOC_SINGLE_EXT!(b"ADC TEMP\0", MAX98373_R2055_MEAS_ADC_THERM_CH_READBACK, 0, 0xFF, 0, max98373_feedback_get, None),
    SOC_SINGLE!(b"ADC PVDD FLT Coeff\0", MAX98373_R2052_MEAS_ADC_PVDD_FLT_CFG, 0, 0x3, 0),
    SOC_SINGLE!(b"ADC TEMP FLT Coeff\0", MAX98373_R2053_MEAS_ADC_THERM_FLT_CFG, 0, 0x3, 0),
    SOC_ENUM!(b"ADC SampleRate\0", max98373_adc_samplerate_enum),
    /* Brownout Detection Engine */
    SOC_SINGLE!(b"BDE Switch\0", MAX98373_R20B5_BDE_EN, MAX98373_BDE_EN_SHIFT, 1, 0),
    SOC_SINGLE!(b"BDE LVL4 Mute Switch\0", MAX98373_R20B2_BDE_L4_CFG_2, MAX98373_LVL4_MUTE_EN_SHIFT, 1, 0),
    SOC_SINGLE!(b"BDE LVL4 Hold Switch\0", MAX98373_R20B2_BDE_L4_CFG_2, MAX98373_LVL4_HOLD_EN_SHIFT, 1, 0),
    SOC_SINGLE!(b"BDE LVL1 Thresh\0", MAX98373_R2097_BDE_L1_THRESH, 0, 0xFF, 0),
    SOC_SINGLE!(b"BDE LVL2 Thresh\0", MAX98373_R2098_BDE_L2_THRESH, 0, 0xFF, 0),
    SOC_SINGLE!(b"BDE LVL3 Thresh\0", MAX98373_R2099_BDE_L3_THRESH, 0, 0xFF, 0),
    SOC_SINGLE!(b"BDE LVL4 Thresh\0", MAX98373_R209A_BDE_L4_THRESH, 0, 0xFF, 0),
    SOC_SINGLE_EXT!(b"BDE Active Level\0", MAX98373_R20B6_BDE_CUR_STATE_READBACK, 0, 8, 0, max98373_feedback_get, None),
    SOC_SINGLE!(b"BDE Clip Mode Switch\0", MAX98373_R2092_BDE_CLIPPER_MODE, 0, 1, 0),
    SOC_SINGLE!(b"BDE Thresh Hysteresis\0", MAX98373_R209B_BDE_THRESH_HYST, 0, 0xFF, 0),
    SOC_SINGLE!(b"BDE Hold Time\0", MAX98373_R2090_BDE_LVL_HOLD, 0, 0xFF, 0),
    SOC_SINGLE!(b"BDE Attack Rate\0", MAX98373_R2091_BDE_GAIN_ATK_REL_RATE, 4, 0xF, 0),
    SOC_SINGLE!(b"BDE Release Rate\0", MAX98373_R2091_BDE_GAIN_ATK_REL_RATE, 0, 0xF, 0),
    SOC_SINGLE_TLV!(b"BDE LVL1 Clip Thresh Volume\0", MAX98373_R20A9_BDE_L1_CFG_2, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL2 Clip Thresh Volume\0", MAX98373_R20AC_BDE_L2_CFG_2, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL3 Clip Thresh Volume\0", MAX98373_R20AF_BDE_L3_CFG_2, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL4 Clip Thresh Volume\0", MAX98373_R20B2_BDE_L4_CFG_2, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL1 Clip Reduction Volume\0", MAX98373_R20AA_BDE_L1_CFG_3, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL2 Clip Reduction Volume\0", MAX98373_R20AD_BDE_L2_CFG_3, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL3 Clip Reduction Volume\0", MAX98373_R20B0_BDE_L3_CFG_3, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL4 Clip Reduction Volume\0", MAX98373_R20B3_BDE_L4_CFG_3, 0, 0x3C, 1, max98373_bde_gain_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL1 Limiter Thresh Volume\0", MAX98373_R20A8_BDE_L1_CFG_1, 0, 0xF, 1, max98373_limiter_thresh_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL2 Limiter Thresh Volume\0", MAX98373_R20AB_BDE_L2_CFG_1, 0, 0xF, 1, max98373_limiter_thresh_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL3 Limiter Thresh Volume\0", MAX98373_R20AE_BDE_L3_CFG_1, 0, 0xF, 1, max98373_limiter_thresh_tlv),
    SOC_SINGLE_TLV!(b"BDE LVL4 Limiter Thresh Volume\0", MAX98373_R20B1_BDE_L4_CFG_1, 0, 0xF, 1, max98373_limiter_thresh_tlv),
    /* Limiter */
    SOC_SINGLE!(b"Limiter Switch\0", MAX98373_R20E2_LIMITER_EN, MAX98373_LIMITER_EN_SHIFT, 1, 0),
    SOC_SINGLE!(b"Limiter Src Switch\0", MAX98373_R20E0_LIMITER_THRESH_CFG, MAX98373_LIMITER_THRESH_SRC_SHIFT, 1, 0),
    SOC_SINGLE_TLV!(b"Limiter Thresh Volume\0", MAX98373_R20E0_LIMITER_THRESH_CFG, MAX98373_LIMITER_THRESH_SHIFT, 15, 0, max98373_limiter_thresh_tlv),
    SOC_ENUM!(b"Limiter Attack Rate\0", max98373_limiter_attack_rate_enum),
    SOC_ENUM!(b"Limiter Release Rate\0", max98373_limiter_release_rate_enum),
];

static max98373_audio_map: [snd_soc_dapm_route; 11] = [
    /* Plabyack */
    snd_soc_dapm_route { sink: b"DAI Sel Mux\0".as_ptr() as *const c_char, control: b"Left\0".as_ptr() as *const c_char, source: b"Amp Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAI Sel Mux\0".as_ptr() as *const c_char, control: b"Right\0".as_ptr() as *const c_char, source: b"Amp Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DAI Sel Mux\0".as_ptr() as *const c_char, control: b"LeftRight\0".as_ptr() as *const c_char, source: b"Amp Enable\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"BE_OUT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAI Sel Mux\0".as_ptr() as *const c_char },
    /* Capture */
    snd_soc_dapm_route { sink: b"VI Sense\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"VMON\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"VI Sense\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"IMON\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SpkFB Sense\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"FBMON\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Voltage Sense\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"VI Sense\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Current Sense\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"VI Sense\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker FB Sense\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SpkFB Sense\0".as_ptr() as *const c_char },
];

#[no_mangle]
pub unsafe extern "C" fn max98373_reset(max98373: *mut max98373_priv, dev: *mut device) {
    let mut ret: c_int;
    let mut reg: c_int = 0;
    let mut count: c_int;

    /* Software Reset */
    ret = regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2000_SW_RESET,
        MAX98373_SOFT_RESET,
        MAX98373_SOFT_RESET,
    );
    if ret != 0 {
        dev_err(dev, b"Reset command failed. (ret:%d)\n\0".as_ptr() as *const c_char, ret);
    }

    count = 0;
    while count < 3 {
        usleep_range(10000, 11000);
        /* Software Reset Verification */
        ret = regmap_read((*max98373).regmap, MAX98373_R21FF_REV_ID, &mut reg);
        if ret == 0 {
            dev_info(dev, b"Reset completed (retry:%d)\n\0".as_ptr() as *const c_char, count);
            return;
        }
        count += 1;
    }
    dev_err(dev, b"Reset failed. (ret:%d)\n\0".as_ptr() as *const c_char, ret);
}

unsafe extern "C" fn max98373_probe(component: *mut snd_soc_component) -> c_int {
    let max98373 = snd_soc_component_get_drvdata(component);

    /* Software Reset */
    max98373_reset(max98373, (*component).dev);

    /* IV default slot configuration */
    regmap_write((*max98373).regmap, MAX98373_R2020_PCM_TX_HIZ_EN_1, 0xFF);
    regmap_write((*max98373).regmap, MAX98373_R2021_PCM_TX_HIZ_EN_2, 0xFF);
    /* L/R mix configuration */
    regmap_write((*max98373).regmap, MAX98373_R2029_PCM_TO_SPK_MONO_MIX_1, 0x80);
    regmap_write((*max98373).regmap, MAX98373_R202A_PCM_TO_SPK_MONO_MIX_2, 0x1);
    /* Enable DC blocker */
    regmap_write((*max98373).regmap, MAX98373_R203F_AMP_DSP_CFG, 0x3);
    /* Enable IMON VMON DC blocker */
    regmap_write((*max98373).regmap, MAX98373_R2046_IV_SENSE_ADC_DSP_CFG, 0x7);
    /* voltage, current slot configuration */
    regmap_write(
        (*max98373).regmap,
        MAX98373_R2022_PCM_TX_SRC_1,
        (((*max98373).i_slot << MAX98373_PCM_TX_CH_SRC_A_I_SHIFT) | (*max98373).v_slot) as c_uint
            & 0xFF,
    );
    if (*max98373).v_slot < 8 {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2020_PCM_TX_HIZ_EN_1,
            (1 << (*max98373).v_slot) as c_uint,
            0,
        );
    } else {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2021_PCM_TX_HIZ_EN_2,
            (1 << ((*max98373).v_slot - 8)) as c_uint,
            0,
        );
    }

    if (*max98373).i_slot < 8 {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2020_PCM_TX_HIZ_EN_1,
            (1 << (*max98373).i_slot) as c_uint,
            0,
        );
    } else {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2021_PCM_TX_HIZ_EN_2,
            (1 << ((*max98373).i_slot - 8)) as c_uint,
            0,
        );
    }

    /* enable auto restart function by default */
    regmap_write((*max98373).regmap, MAX98373_R20FE_DEVICE_AUTO_RESTART_CFG, 0xF);

    /* speaker feedback slot configuration */
    regmap_write(
        (*max98373).regmap,
        MAX98373_R2023_PCM_TX_SRC_2,
        ((*max98373).spkfb_slot & 0xFF) as c_uint,
    );

    /* Set interleave mode */
    if (*max98373).interleave_mode {
        regmap_update_bits(
            (*max98373).regmap,
            MAX98373_R2024_PCM_DATA_FMT_CFG,
            MAX98373_PCM_TX_CH_INTERLEAVE_MASK,
            MAX98373_PCM_TX_CH_INTERLEAVE_MASK,
        );
    }

    /* Speaker enable */
    regmap_update_bits(
        (*max98373).regmap,
        MAX98373_R2043_AMP_EN,
        MAX98373_SPK_EN_MASK,
        1,
    );

    0
}

#[no_mangle]
pub static soc_codec_dev_max98373: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98373_probe),
    controls: max98373_snd_controls.as_ptr(),
    num_controls: max98373_snd_controls.len() as c_uint,
    dapm_widgets: max98373_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98373_dapm_widgets.len() as c_uint,
    dapm_routes: max98373_audio_map.as_ptr(),
    num_dapm_routes: max98373_audio_map.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn max98373_sdw_probe(component: *mut snd_soc_component) -> c_int {
    let ret: c_int;

    ret = pm_runtime_resume((*component).dev);
    if ret < 0 && ret != -EACCES {
        return ret;
    }

    0
}

#[no_mangle]
pub static soc_codec_dev_max98373_sdw: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98373_sdw_probe),
    controls: max98373_snd_controls.as_ptr(),
    num_controls: max98373_snd_controls.len() as c_uint,
    dapm_widgets: max98373_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98373_dapm_widgets.len() as c_uint,
    dapm_routes: max98373_audio_map.as_ptr(),
    num_dapm_routes: max98373_audio_map.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

#[no_mangle]
pub unsafe extern "C" fn max98373_slot_config(dev: *mut device, max98373: *mut max98373_priv) {
    let mut value: c_int = 0;

    if device_property_read_u32(
        dev,
        b"maxim,vmon-slot-no\0".as_ptr() as *const c_char,
        &mut value,
    ) == 0
    {
        (*max98373).v_slot = value & 0xF;
    } else {
        (*max98373).v_slot = 0;
    }

    if device_property_read_u32(
        dev,
        b"maxim,imon-slot-no\0".as_ptr() as *const c_char,
        &mut value,
    ) == 0
    {
        (*max98373).i_slot = value & 0xF;
    } else {
        (*max98373).i_slot = 1;
    }

    /* This will assert RESET */
    (*max98373).reset = devm_gpiod_get_optional(
        dev,
        b"maxim,reset\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*max98373).reset as *const c_void) {
        dev_err(
            dev,
            b"error %ld looking up RESET GPIO line\n\0".as_ptr() as *const c_char,
            PTR_ERR((*max98373).reset as *const c_void),
        );
        return;
    }

    /* Cycle reset */
    if !(*max98373).reset.is_null() {
        gpiod_set_consumer_name(
            (*max98373).reset,
            b"MAX98373_RESET\0".as_ptr() as *const c_char,
        );
        gpiod_direction_output((*max98373).reset, 1);
        msleep(50);
        gpiod_direction_output((*max98373).reset, 0);
        msleep(20);
    }

    if device_property_read_u32(
        dev,
        b"maxim,spkfb-slot-no\0".as_ptr() as *const c_char,
        &mut value,
    ) == 0
    {
        (*max98373).spkfb_slot = value & 0xF;
    } else {
        (*max98373).spkfb_slot = 2;
    }
}

// MODULE_DESCRIPTION("ALSA SoC MAX98373 driver");
// MODULE_AUTHOR("Ryan Lee <ryans.lee@maximintegrated.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
