// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This driver supports the analog controls for the internal codec
 * found in Allwinner's A31s, A23, A33 and H3 SoCs.
 *
 * Copyright 2016 Chen-Yu Tsai <wens@csie.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct device {
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
pub struct snd_kcontrol_new {
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn msleep(msecs: c_uint);
    fn snd_soc_component_to_dapm(cmpnt: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_add_component_controls(
        cmpnt: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn sun8i_adda_pr_regmap_init(dev: *mut device, base: *mut c_void) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;

    fn SOC_DAPM_DOUBLE_R(
        name: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE_TLV(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DOUBLE(
        name: *const c_char,
        reg: c_uint,
        shift_left: c_uint,
        shift_right: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DAPM_ENUM(name: *const c_char, xenum: soc_enum) -> snd_kcontrol_new;
    fn SOC_ENUM_DOUBLE_DECL(
        reg: c_uint,
        shift_l: c_uint,
        shift_r: c_uint,
        texts: *const *const c_char,
    ) -> soc_enum;
    fn SND_SOC_DAPM_ADC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_DAC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIXER(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MUX(
        name: *const c_char,
        reg: c_int,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUT_DRV_E(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
        event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        flags: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        event: *const c_void,
        flags: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_REG(
        id: c_uint,
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        mask: c_uint,
        on_val: c_uint,
        off_val: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
    fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool;
}

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_uint = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_uint = 0x2;
const snd_soc_dapm_supply: c_uint = 0;
const TLV_DB_GAIN_MUTE: c_int = -9999999;

/* Codec analog control register offsets and bit fields */
const SUN8I_ADDA_HP_VOLC: c_uint = 0x00;
const SUN8I_ADDA_HP_VOLC_PA_CLK_GATE: c_uint = 7;
const SUN8I_ADDA_HP_VOLC_HP_VOL: c_uint = 0;
const SUN8I_ADDA_LOMIXSC: c_uint = 0x01;
const SUN8I_ADDA_LOMIXSC_MIC1: c_uint = 6;
const SUN8I_ADDA_LOMIXSC_MIC2: c_uint = 5;
const SUN8I_ADDA_LOMIXSC_PHONE: c_uint = 4;
const SUN8I_ADDA_LOMIXSC_PHONEN: c_uint = 3;
const SUN8I_ADDA_LOMIXSC_LINEINL: c_uint = 2;
const SUN8I_ADDA_LOMIXSC_DACL: c_uint = 1;
const SUN8I_ADDA_LOMIXSC_DACR: c_uint = 0;
const SUN8I_ADDA_ROMIXSC: c_uint = 0x02;
const SUN8I_ADDA_ROMIXSC_MIC1: c_uint = 6;
const SUN8I_ADDA_ROMIXSC_MIC2: c_uint = 5;
const SUN8I_ADDA_ROMIXSC_PHONE: c_uint = 4;
const SUN8I_ADDA_ROMIXSC_PHONEP: c_uint = 3;
const SUN8I_ADDA_ROMIXSC_LINEINR: c_uint = 2;
const SUN8I_ADDA_ROMIXSC_DACR: c_uint = 1;
const SUN8I_ADDA_ROMIXSC_DACL: c_uint = 0;
const SUN8I_ADDA_DAC_PA_SRC: c_uint = 0x03;
const SUN8I_ADDA_DAC_PA_SRC_DACAREN: c_uint = 7;
const SUN8I_ADDA_DAC_PA_SRC_DACALEN: c_uint = 6;
const SUN8I_ADDA_DAC_PA_SRC_RMIXEN: c_uint = 5;
const SUN8I_ADDA_DAC_PA_SRC_LMIXEN: c_uint = 4;
const SUN8I_ADDA_DAC_PA_SRC_RHPPAMUTE: c_uint = 3;
const SUN8I_ADDA_DAC_PA_SRC_LHPPAMUTE: c_uint = 2;
const SUN8I_ADDA_DAC_PA_SRC_RHPIS: c_uint = 1;
const SUN8I_ADDA_DAC_PA_SRC_LHPIS: c_uint = 0;
const SUN8I_ADDA_PHONEIN_GCTRL: c_uint = 0x04;
const SUN8I_ADDA_PHONEIN_GCTRL_PHONEPG: c_uint = 4;
const SUN8I_ADDA_PHONEIN_GCTRL_PHONENG: c_uint = 0;
const SUN8I_ADDA_LINEIN_GCTRL: c_uint = 0x05;
const SUN8I_ADDA_LINEIN_GCTRL_LINEING: c_uint = 4;
const SUN8I_ADDA_LINEIN_GCTRL_PHONEG: c_uint = 0;
const SUN8I_ADDA_MICIN_GCTRL: c_uint = 0x06;
const SUN8I_ADDA_MICIN_GCTRL_MIC1G: c_uint = 4;
const SUN8I_ADDA_MICIN_GCTRL_MIC2G: c_uint = 0;
const SUN8I_ADDA_PAEN_HP_CTRL: c_uint = 0x07;
const SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN: c_uint = 7;
const SUN8I_ADDA_PAEN_HP_CTRL_LINEOUTEN: c_uint = 7; /* H3 specific */
const SUN8I_ADDA_PAEN_HP_CTRL_HPCOM_FC: c_uint = 5;
const SUN8I_ADDA_PAEN_HP_CTRL_COMPTEN: c_uint = 4;
const SUN8I_ADDA_PAEN_HP_CTRL_PA_ANTI_POP_CTRL: c_uint = 2;
const SUN8I_ADDA_PAEN_HP_CTRL_LTRNMUTE: c_uint = 1;
const SUN8I_ADDA_PAEN_HP_CTRL_RTLNMUTE: c_uint = 0;
const SUN8I_ADDA_PHONEOUT_CTRL: c_uint = 0x08;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUTG: c_uint = 5;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUTEN: c_uint = 4;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_MIC1: c_uint = 3;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_MIC2: c_uint = 2;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_RMIX: c_uint = 1;
const SUN8I_ADDA_PHONEOUT_CTRL_PHONEOUT_LMIX: c_uint = 0;
const SUN8I_ADDA_PHONE_GAIN_CTRL: c_uint = 0x09;
const SUN8I_ADDA_PHONE_GAIN_CTRL_LINEOUT_VOL: c_uint = 3;
const SUN8I_ADDA_PHONE_GAIN_CTRL_PHONEPREG: c_uint = 0;
const SUN8I_ADDA_MIC2G_CTRL: c_uint = 0x0a;
const SUN8I_ADDA_MIC2G_CTRL_MIC2AMPEN: c_uint = 7;
const SUN8I_ADDA_MIC2G_CTRL_MIC2BOOST: c_uint = 4;
const SUN8I_ADDA_MIC2G_CTRL_LINEOUTLEN: c_uint = 3;
const SUN8I_ADDA_MIC2G_CTRL_LINEOUTREN: c_uint = 2;
const SUN8I_ADDA_MIC2G_CTRL_LINEOUTLSRC: c_uint = 1;
const SUN8I_ADDA_MIC2G_CTRL_LINEOUTRSRC: c_uint = 0;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL: c_uint = 0x0b;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIASEN: c_uint = 7;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MMICBIASEN: c_uint = 6;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIAS_MODE: c_uint = 5;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1AMPEN: c_uint = 3;
const SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1BOOST: c_uint = 0;
const SUN8I_ADDA_LADCMIXSC: c_uint = 0x0c;
const SUN8I_ADDA_LADCMIXSC_MIC1: c_uint = 6;
const SUN8I_ADDA_LADCMIXSC_MIC2: c_uint = 5;
const SUN8I_ADDA_LADCMIXSC_PHONE: c_uint = 4;
const SUN8I_ADDA_LADCMIXSC_PHONEN: c_uint = 3;
const SUN8I_ADDA_LADCMIXSC_LINEINL: c_uint = 2;
const SUN8I_ADDA_LADCMIXSC_OMIXRL: c_uint = 1;
const SUN8I_ADDA_LADCMIXSC_OMIXRR: c_uint = 0;
const SUN8I_ADDA_RADCMIXSC: c_uint = 0x0d;
const SUN8I_ADDA_RADCMIXSC_MIC1: c_uint = 6;
const SUN8I_ADDA_RADCMIXSC_MIC2: c_uint = 5;
const SUN8I_ADDA_RADCMIXSC_PHONE: c_uint = 4;
const SUN8I_ADDA_RADCMIXSC_PHONEP: c_uint = 3;
const SUN8I_ADDA_RADCMIXSC_LINEINR: c_uint = 2;
const SUN8I_ADDA_RADCMIXSC_OMIXR: c_uint = 1;
const SUN8I_ADDA_RADCMIXSC_OMIXL: c_uint = 0;
const SUN8I_ADDA_RES: c_uint = 0x0e;
const SUN8I_ADDA_RES_MMICBIAS_SEL: c_uint = 4;
const SUN8I_ADDA_RES_PA_ANTI_POP_CTRL: c_uint = 0;
const SUN8I_ADDA_ADC_AP_EN: c_uint = 0x0f;
const SUN8I_ADDA_ADC_AP_EN_ADCREN: c_uint = 7;
const SUN8I_ADDA_ADC_AP_EN_ADCLEN: c_uint = 6;
const SUN8I_ADDA_ADC_AP_EN_ADCG: c_uint = 0;

// TLV macro output is represented as raw TLV integer arrays.
static sun8i_codec_out_mixer_pregain_scale: [c_uint; 3] = [-450i32 as c_uint, 150, 0];
static sun8i_codec_mic_gain_scale: [c_uint; 9] = [0, 0, 0, 0, 0, 1, 7, 2400, 300];
static sun8i_codec_hp_vol_scale: [c_uint; 3] = [-6300i32 as c_uint, 100, 1];
static sun8i_codec_lineout_vol_scale: [c_uint; 9] = [
    0,
    1,
    TLV_DB_GAIN_MUTE as c_uint,
    0,
    1,
    2,
    31,
    (-4350i32) as c_uint,
    150,
];

/* mixer controls */
static mut sun8i_codec_mixer_controls: [snd_kcontrol_new; 5] = unsafe {
    [
        SOC_DAPM_DOUBLE_R(c"DAC Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_DACL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"DAC Reversed Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_DACR, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Line In Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_LINEINL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic1 Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_MIC1, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic2 Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_MIC2, 1, 0),
    ]
};

/* mixer controls */
static mut sun8i_v3s_codec_mixer_controls: [snd_kcontrol_new; 3] = unsafe {
    [
        SOC_DAPM_DOUBLE_R(c"DAC Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_DACL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"DAC Reversed Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_DACR, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic1 Playback Switch".as_ptr(), SUN8I_ADDA_LOMIXSC, SUN8I_ADDA_ROMIXSC, SUN8I_ADDA_LOMIXSC_MIC1, 1, 0),
    ]
};

/* ADC mixer controls */
static mut sun8i_codec_adc_mixer_controls: [snd_kcontrol_new; 5] = unsafe {
    [
        SOC_DAPM_DOUBLE_R(c"Mixer Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_OMIXRL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mixer Reversed Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_OMIXRR, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Line In Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_LINEINL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic1 Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_MIC1, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic2 Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_MIC2, 1, 0),
    ]
};

/* ADC mixer controls */
static mut sun8i_v3s_codec_adc_mixer_controls: [snd_kcontrol_new; 3] = unsafe {
    [
        SOC_DAPM_DOUBLE_R(c"Mixer Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_OMIXRL, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mixer Reversed Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_OMIXRR, 1, 0),
        SOC_DAPM_DOUBLE_R(c"Mic1 Capture Switch".as_ptr(), SUN8I_ADDA_LADCMIXSC, SUN8I_ADDA_RADCMIXSC, SUN8I_ADDA_LADCMIXSC_MIC1, 1, 0),
    ]
};

/* volume / mute controls */
static mut sun8i_codec_common_controls: [snd_kcontrol_new; 3] = unsafe {
    [
        /* Mixer pre-gain */
        SOC_SINGLE_TLV(c"Mic1 Playback Volume".as_ptr(), SUN8I_ADDA_MICIN_GCTRL, SUN8I_ADDA_MICIN_GCTRL_MIC1G, 0x7, 0, sun8i_codec_out_mixer_pregain_scale.as_ptr()),
        /* Microphone Amp boost gain */
        SOC_SINGLE_TLV(c"Mic1 Boost Volume".as_ptr(), SUN8I_ADDA_MIC1G_MICBIAS_CTRL, SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1BOOST, 0x7, 0, sun8i_codec_mic_gain_scale.as_ptr()),
        /* ADC */
        SOC_SINGLE_TLV(c"ADC Gain Capture Volume".as_ptr(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCG, 0x7, 0, sun8i_codec_out_mixer_pregain_scale.as_ptr()),
    ]
};

static mut sun8i_codec_common_widgets: [snd_soc_dapm_widget; 5] = unsafe {
    [
        /* ADC */
        SND_SOC_DAPM_ADC(c"Left ADC".as_ptr(), ptr::null(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0),
        SND_SOC_DAPM_ADC(c"Right ADC".as_ptr(), ptr::null(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCREN, 0),
        /* DAC */
        SND_SOC_DAPM_DAC(c"Left DAC".as_ptr(), ptr::null(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_DACALEN, 0),
        SND_SOC_DAPM_DAC(c"Right DAC".as_ptr(), ptr::null(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_DACAREN, 0),
        /*
         * Due to this component and the codec belonging to separate DAPM
         * contexts, we need to manually link the above widgets to their
         * stream widgets at the card level.
         */
        /* Microphone input */
        SND_SOC_DAPM_INPUT(c"MIC1".as_ptr()),
        /* Mic input path */
        SND_SOC_DAPM_PGA(c"Mic1 Amplifier".as_ptr(), SUN8I_ADDA_MIC1G_MICBIAS_CTRL, SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MIC1AMPEN, 0, ptr::null(), 0),
    ]
};

static mut sun8i_codec_mixer_widgets: [snd_soc_dapm_widget; 4] = unsafe {
    [
        SND_SOC_DAPM_MIXER(c"Left Mixer".as_ptr(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_LMIXEN, 0, sun8i_codec_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Right Mixer".as_ptr(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_RMIXEN, 0, sun8i_codec_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Left ADC Mixer".as_ptr(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0, sun8i_codec_adc_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_adc_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Right ADC Mixer".as_ptr(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCREN, 0, sun8i_codec_adc_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_adc_mixer_controls)),
    ]
};

static mut sun8i_v3s_codec_mixer_widgets: [snd_soc_dapm_widget; 4] = unsafe {
    [
        SND_SOC_DAPM_MIXER(c"Left Mixer".as_ptr(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_LMIXEN, 0, sun8i_v3s_codec_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_v3s_codec_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Right Mixer".as_ptr(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_RMIXEN, 0, sun8i_v3s_codec_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_v3s_codec_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Left ADC Mixer".as_ptr(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCLEN, 0, sun8i_v3s_codec_adc_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_v3s_codec_adc_mixer_controls)),
        SND_SOC_DAPM_MIXER(c"Right ADC Mixer".as_ptr(), SUN8I_ADDA_ADC_AP_EN, SUN8I_ADDA_ADC_AP_EN_ADCREN, 0, sun8i_v3s_codec_adc_mixer_controls.as_ptr(), ARRAY_SIZE(&sun8i_v3s_codec_adc_mixer_controls)),
    ]
};

static sun8i_codec_common_routes: [snd_soc_dapm_route; 1] = [
    /* Microphone Routes */
    snd_soc_dapm_route { sink: c"Mic1 Amplifier".as_ptr(), control: ptr::null(), source: c"MIC1".as_ptr() },
];

static sun8i_codec_mixer_routes: [snd_soc_dapm_route; 15] = [
    /* Left Mixer Routes */
    snd_soc_dapm_route { sink: c"Left Mixer".as_ptr(), control: c"DAC Playback Switch".as_ptr(), source: c"Left DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Left Mixer".as_ptr(), control: c"DAC Reversed Playback Switch".as_ptr(), source: c"Right DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Left Mixer".as_ptr(), control: c"Mic1 Playback Switch".as_ptr(), source: c"Mic1 Amplifier".as_ptr() },
    /* Right Mixer Routes */
    snd_soc_dapm_route { sink: c"Right Mixer".as_ptr(), control: c"DAC Playback Switch".as_ptr(), source: c"Right DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Mixer".as_ptr(), control: c"DAC Reversed Playback Switch".as_ptr(), source: c"Left DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Mixer".as_ptr(), control: c"Mic1 Playback Switch".as_ptr(), source: c"Mic1 Amplifier".as_ptr() },
    /* Left ADC Mixer Routes */
    snd_soc_dapm_route { sink: c"Left ADC Mixer".as_ptr(), control: c"Mixer Capture Switch".as_ptr(), source: c"Left Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Left ADC Mixer".as_ptr(), control: c"Mixer Reversed Capture Switch".as_ptr(), source: c"Right Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Left ADC Mixer".as_ptr(), control: c"Mic1 Capture Switch".as_ptr(), source: c"Mic1 Amplifier".as_ptr() },
    /* Right ADC Mixer Routes */
    snd_soc_dapm_route { sink: c"Right ADC Mixer".as_ptr(), control: c"Mixer Capture Switch".as_ptr(), source: c"Right Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Right ADC Mixer".as_ptr(), control: c"Mixer Reversed Capture Switch".as_ptr(), source: c"Left Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Right ADC Mixer".as_ptr(), control: c"Mic1 Capture Switch".as_ptr(), source: c"Mic1 Amplifier".as_ptr() },
    /* ADC Routes */
    snd_soc_dapm_route { sink: c"Left ADC".as_ptr(), control: ptr::null(), source: c"Left ADC Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Right ADC".as_ptr(), control: ptr::null(), source: c"Right ADC Mixer".as_ptr() },
];

/* headphone specific controls, widgets, and routes */
static mut sun8i_codec_headphone_controls: [snd_kcontrol_new; 2] = unsafe {
    [
        SOC_SINGLE_TLV(c"Headphone Playback Volume".as_ptr(), SUN8I_ADDA_HP_VOLC, SUN8I_ADDA_HP_VOLC_HP_VOL, 0x3f, 0, sun8i_codec_hp_vol_scale.as_ptr()),
        SOC_DOUBLE(c"Headphone Playback Switch".as_ptr(), SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_LHPPAMUTE, SUN8I_ADDA_DAC_PA_SRC_RHPPAMUTE, 1, 0),
    ]
};

static sun8i_codec_hp_src_enum_text: [*const c_char; 2] = [
    c"DAC".as_ptr(),
    c"Mixer".as_ptr(),
];

static mut sun8i_codec_hp_src_enum: soc_enum = unsafe {
    SOC_ENUM_DOUBLE_DECL(SUN8I_ADDA_DAC_PA_SRC, SUN8I_ADDA_DAC_PA_SRC_LHPIS, SUN8I_ADDA_DAC_PA_SRC_RHPIS, sun8i_codec_hp_src_enum_text.as_ptr())
};

static mut sun8i_codec_hp_src: [snd_kcontrol_new; 1] = unsafe {
    [SOC_DAPM_ENUM(c"Headphone Source Playback Route".as_ptr(), sun8i_codec_hp_src_enum)]
};

unsafe extern "C" fn sun8i_headphone_amp_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);

    if SND_SOC_DAPM_EVENT_ON(event) {
        snd_soc_component_update_bits(
            component,
            SUN8I_ADDA_PAEN_HP_CTRL,
            BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN),
            BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN),
        );
        /*
         * Need a delay to have the amplifier up. 700ms seems the best
         * compromise between the time to let the amplifier up and the
         * time not to feel this delay while playing a sound.
         */
        msleep(700);
    } else if SND_SOC_DAPM_EVENT_OFF(event) {
        snd_soc_component_update_bits(
            component,
            SUN8I_ADDA_PAEN_HP_CTRL,
            BIT(SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN),
            0x0,
        );
    }

    0
}

static mut sun8i_codec_headphone_widgets: [snd_soc_dapm_widget; 5] = unsafe {
    [
        SND_SOC_DAPM_MUX(c"Headphone Source Playback Route".as_ptr(), SND_SOC_NOPM, 0, 0, sun8i_codec_hp_src.as_ptr()),
        SND_SOC_DAPM_OUT_DRV_E(c"Headphone Amp".as_ptr(), SUN8I_ADDA_PAEN_HP_CTRL, SUN8I_ADDA_PAEN_HP_CTRL_HPPAEN, 0, ptr::null(), 0, Some(sun8i_headphone_amp_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
        SND_SOC_DAPM_SUPPLY(c"HPCOM Protection".as_ptr(), SUN8I_ADDA_PAEN_HP_CTRL, SUN8I_ADDA_PAEN_HP_CTRL_COMPTEN, 0, ptr::null(), 0),
        SND_SOC_DAPM_REG(snd_soc_dapm_supply, c"HPCOM".as_ptr(), SUN8I_ADDA_PAEN_HP_CTRL, SUN8I_ADDA_PAEN_HP_CTRL_HPCOM_FC, 0x3, 0x3, 0),
        SND_SOC_DAPM_OUTPUT(c"HP".as_ptr()),
    ]
};

static sun8i_codec_headphone_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: c"Headphone Source Playback Route".as_ptr(), control: c"DAC".as_ptr(), source: c"Left DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone Source Playback Route".as_ptr(), control: c"DAC".as_ptr(), source: c"Right DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone Source Playback Route".as_ptr(), control: c"Mixer".as_ptr(), source: c"Left Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone Source Playback Route".as_ptr(), control: c"Mixer".as_ptr(), source: c"Right Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone Amp".as_ptr(), control: ptr::null(), source: c"Headphone Source Playback Route".as_ptr() },
    snd_soc_dapm_route { sink: c"HPCOM".as_ptr(), control: ptr::null(), source: c"HPCOM Protection".as_ptr() },
    snd_soc_dapm_route { sink: c"HP".as_ptr(), control: ptr::null(), source: c"Headphone Amp".as_ptr() },
];

unsafe extern "C" fn sun8i_codec_add_headphone(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let mut ret: c_int;

    ret = snd_soc_add_component_controls(cmpnt, sun8i_codec_headphone_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_headphone_controls));
    if ret != 0 {
        dev_err(dev, c"Failed to add Headphone controls: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_headphone_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_headphone_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Headphone DAPM widgets: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_headphone_routes.as_ptr(), ARRAY_SIZE(&sun8i_codec_headphone_routes) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Headphone DAPM routes: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

/* mbias specific widget */
static mut sun8i_codec_mbias_widgets: [snd_soc_dapm_widget; 1] = unsafe {
    [SND_SOC_DAPM_SUPPLY(c"MBIAS".as_ptr(), SUN8I_ADDA_MIC1G_MICBIAS_CTRL, SUN8I_ADDA_MIC1G_MICBIAS_CTRL_MMICBIASEN, 0, ptr::null(), 0)]
};

unsafe extern "C" fn sun8i_codec_add_mbias(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_mbias_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_mbias_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add MBIAS DAPM widgets: %d\n".as_ptr(), ret);
    }

    ret
}

/* hmic specific widget */
static mut sun8i_codec_hmic_widgets: [snd_soc_dapm_widget; 1] = unsafe {
    [SND_SOC_DAPM_SUPPLY(c"HBIAS".as_ptr(), SUN8I_ADDA_MIC1G_MICBIAS_CTRL, SUN8I_ADDA_MIC1G_MICBIAS_CTRL_HMICBIASEN, 0, ptr::null(), 0)]
};

unsafe extern "C" fn sun8i_codec_add_hmic(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_hmic_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_hmic_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Mic3 DAPM widgets: %d\n".as_ptr(), ret);
    }

    ret
}

/* line in specific controls, widgets and rines */
static mut sun8i_codec_linein_controls: [snd_kcontrol_new; 1] = unsafe {
    [
        /* Mixer pre-gain */
        SOC_SINGLE_TLV(c"Line In Playback Volume".as_ptr(), SUN8I_ADDA_LINEIN_GCTRL, SUN8I_ADDA_LINEIN_GCTRL_LINEING, 0x7, 0, sun8i_codec_out_mixer_pregain_scale.as_ptr()),
    ]
};

static mut sun8i_codec_linein_widgets: [snd_soc_dapm_widget; 1] = unsafe {
    [
        /* Line input */
        SND_SOC_DAPM_INPUT(c"LINEIN".as_ptr()),
    ]
};

static sun8i_codec_linein_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"Left Mixer".as_ptr(), control: c"Line In Playback Switch".as_ptr(), source: c"LINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Mixer".as_ptr(), control: c"Line In Playback Switch".as_ptr(), source: c"LINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Left ADC Mixer".as_ptr(), control: c"Line In Capture Switch".as_ptr(), source: c"LINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Right ADC Mixer".as_ptr(), control: c"Line In Capture Switch".as_ptr(), source: c"LINEIN".as_ptr() },
];

unsafe extern "C" fn sun8i_codec_add_linein(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let mut ret: c_int;

    ret = snd_soc_add_component_controls(cmpnt, sun8i_codec_linein_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_linein_controls));
    if ret != 0 {
        dev_err(dev, c"Failed to add Line In controls: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_linein_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_linein_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Line In DAPM widgets: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_linein_routes.as_ptr(), ARRAY_SIZE(&sun8i_codec_linein_routes) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Line In DAPM routes: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

/* line out specific controls, widgets and routes */
static mut sun8i_codec_lineout_controls: [snd_kcontrol_new; 2] = unsafe {
    [
        SOC_SINGLE_TLV(c"Line Out Playback Volume".as_ptr(), SUN8I_ADDA_PHONE_GAIN_CTRL, SUN8I_ADDA_PHONE_GAIN_CTRL_LINEOUT_VOL, 0x1f, 0, sun8i_codec_lineout_vol_scale.as_ptr()),
        SOC_DOUBLE(c"Line Out Playback Switch".as_ptr(), SUN8I_ADDA_MIC2G_CTRL, SUN8I_ADDA_MIC2G_CTRL_LINEOUTLEN, SUN8I_ADDA_MIC2G_CTRL_LINEOUTREN, 1, 0),
    ]
};

static sun8i_codec_lineout_src_enum_text: [*const c_char; 2] = [
    c"Stereo".as_ptr(),
    c"Mono Differential".as_ptr(),
];

static mut sun8i_codec_lineout_src_enum: soc_enum = unsafe {
    SOC_ENUM_DOUBLE_DECL(SUN8I_ADDA_MIC2G_CTRL, SUN8I_ADDA_MIC2G_CTRL_LINEOUTLSRC, SUN8I_ADDA_MIC2G_CTRL_LINEOUTRSRC, sun8i_codec_lineout_src_enum_text.as_ptr())
};

static mut sun8i_codec_lineout_src: [snd_kcontrol_new; 1] = unsafe {
    [SOC_DAPM_ENUM(c"Line Out Source Playback Route".as_ptr(), sun8i_codec_lineout_src_enum)]
};

static mut sun8i_codec_lineout_widgets: [snd_soc_dapm_widget; 3] = unsafe {
    [
        SND_SOC_DAPM_MUX(c"Line Out Source Playback Route".as_ptr(), SND_SOC_NOPM, 0, 0, sun8i_codec_lineout_src.as_ptr()),
        /* It is unclear if this is a buffer or gate, model it as a supply */
        SND_SOC_DAPM_SUPPLY(c"Line Out Enable".as_ptr(), SUN8I_ADDA_PAEN_HP_CTRL, SUN8I_ADDA_PAEN_HP_CTRL_LINEOUTEN, 0, ptr::null(), 0),
        SND_SOC_DAPM_OUTPUT(c"LINEOUT".as_ptr()),
    ]
};

static sun8i_codec_lineout_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"Line Out Source Playback Route".as_ptr(), control: c"Stereo".as_ptr(), source: c"Left Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out Source Playback Route".as_ptr(), control: c"Stereo".as_ptr(), source: c"Right Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out Source Playback Route".as_ptr(), control: c"Mono Differential".as_ptr(), source: c"Left Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Line Out Source Playback Route".as_ptr(), control: c"Mono Differential".as_ptr(), source: c"Right Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"LINEOUT".as_ptr(), control: ptr::null(), source: c"Line Out Source Playback Route".as_ptr() },
    snd_soc_dapm_route { sink: c"LINEOUT".as_ptr(), control: ptr::null(), source: c"Line Out Enable".as_ptr() },
];

unsafe extern "C" fn sun8i_codec_add_lineout(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let mut ret: c_int;

    ret = snd_soc_add_component_controls(cmpnt, sun8i_codec_lineout_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_lineout_controls));
    if ret != 0 {
        dev_err(dev, c"Failed to add Line Out controls: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_lineout_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_lineout_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Line Out DAPM widgets: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_lineout_routes.as_ptr(), ARRAY_SIZE(&sun8i_codec_lineout_routes) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Line Out DAPM routes: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

/* mic2 specific controls, widgets and routes */
static mut sun8i_codec_mic2_controls: [snd_kcontrol_new; 2] = unsafe {
    [
        /* Mixer pre-gain */
        SOC_SINGLE_TLV(c"Mic2 Playback Volume".as_ptr(), SUN8I_ADDA_MICIN_GCTRL, SUN8I_ADDA_MICIN_GCTRL_MIC2G, 0x7, 0, sun8i_codec_out_mixer_pregain_scale.as_ptr()),
        /* Microphone Amp boost gain */
        SOC_SINGLE_TLV(c"Mic2 Boost Volume".as_ptr(), SUN8I_ADDA_MIC2G_CTRL, SUN8I_ADDA_MIC2G_CTRL_MIC2BOOST, 0x7, 0, sun8i_codec_mic_gain_scale.as_ptr()),
    ]
};

static mut sun8i_codec_mic2_widgets: [snd_soc_dapm_widget; 2] = unsafe {
    [
        /* Microphone input */
        SND_SOC_DAPM_INPUT(c"MIC2".as_ptr()),
        /* Mic input path */
        SND_SOC_DAPM_PGA(c"Mic2 Amplifier".as_ptr(), SUN8I_ADDA_MIC2G_CTRL, SUN8I_ADDA_MIC2G_CTRL_MIC2AMPEN, 0, ptr::null(), 0),
    ]
};

static sun8i_codec_mic2_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: c"Mic2 Amplifier".as_ptr(), control: ptr::null(), source: c"MIC2".as_ptr() },
    snd_soc_dapm_route { sink: c"Left Mixer".as_ptr(), control: c"Mic2 Playback Switch".as_ptr(), source: c"Mic2 Amplifier".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Mixer".as_ptr(), control: c"Mic2 Playback Switch".as_ptr(), source: c"Mic2 Amplifier".as_ptr() },
    snd_soc_dapm_route { sink: c"Left ADC Mixer".as_ptr(), control: c"Mic2 Capture Switch".as_ptr(), source: c"Mic2 Amplifier".as_ptr() },
    snd_soc_dapm_route { sink: c"Right ADC Mixer".as_ptr(), control: c"Mic2 Capture Switch".as_ptr(), source: c"Mic2 Amplifier".as_ptr() },
];

unsafe extern "C" fn sun8i_codec_add_mic2(cmpnt: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let mut ret: c_int;

    ret = snd_soc_add_component_controls(cmpnt, sun8i_codec_mic2_controls.as_ptr(), ARRAY_SIZE(&sun8i_codec_mic2_controls));
    if ret != 0 {
        dev_err(dev, c"Failed to add MIC2 controls: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_mic2_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_mic2_widgets) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add MIC2 DAPM widgets: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_mic2_routes.as_ptr(), ARRAY_SIZE(&sun8i_codec_mic2_routes) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add MIC2 DAPM routes: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

#[repr(C)]
struct sun8i_codec_analog_quirks {
    has_headphone: bool,
    has_hmic: bool,
    has_linein: bool,
    has_lineout: bool,
    has_mbias: bool,
    has_mic2: bool,
}

static sun8i_a23_quirks: sun8i_codec_analog_quirks = sun8i_codec_analog_quirks {
    has_headphone: true,
    has_hmic: true,
    has_linein: true,
    has_lineout: false,
    has_mbias: true,
    has_mic2: true,
};

static sun8i_h3_quirks: sun8i_codec_analog_quirks = sun8i_codec_analog_quirks {
    has_headphone: false,
    has_hmic: false,
    has_linein: true,
    has_lineout: true,
    has_mbias: true,
    has_mic2: true,
};

unsafe extern "C" fn sun8i_codec_analog_add_mixer(
    cmpnt: *mut snd_soc_component,
    quirks: *const sun8i_codec_analog_quirks,
) -> c_int {
    let dapm = snd_soc_component_to_dapm(cmpnt);
    let dev = (*cmpnt).dev;
    let mut ret: c_int;

    if !(*quirks).has_mic2 && !(*quirks).has_linein {
        /*
         * Apply the special widget set which has uses a control
         * without MIC2 and Line In, for SoCs without these.
         * TODO: not all special cases are supported now, this case
         * is present because it's the case of V3s.
         */
        ret = snd_soc_dapm_new_controls(dapm, sun8i_v3s_codec_mixer_widgets.as_ptr(), ARRAY_SIZE(&sun8i_v3s_codec_mixer_widgets) as c_int);
        if ret != 0 {
            dev_err(dev, c"Failed to add V3s Mixer DAPM widgets: %d\n".as_ptr(), ret);
            return ret;
        }
    } else {
        /* Apply the generic mixer widget set. */
        ret = snd_soc_dapm_new_controls(dapm, sun8i_codec_mixer_widgets.as_ptr(), ARRAY_SIZE(&sun8i_codec_mixer_widgets) as c_int);
        if ret != 0 {
            dev_err(dev, c"Failed to add Mixer DAPM widgets: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    ret = snd_soc_dapm_add_routes(dapm, sun8i_codec_mixer_routes.as_ptr(), ARRAY_SIZE(&sun8i_codec_mixer_routes) as c_int);
    if ret != 0 {
        dev_err(dev, c"Failed to add Mixer DAPM routes: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static sun8i_v3s_quirks: sun8i_codec_analog_quirks = sun8i_codec_analog_quirks {
    has_headphone: true,
    has_hmic: true,
    has_linein: false,
    has_lineout: false,
    has_mbias: false,
    has_mic2: false,
};

unsafe extern "C" fn sun8i_codec_analog_cmpnt_probe(cmpnt: *mut snd_soc_component) -> c_int {
    let dev = (*cmpnt).dev;
    let quirks: *const sun8i_codec_analog_quirks;
    let mut ret: c_int;

    /*
     * This would never return NULL unless someone directly registers a
     * platform device matching this driver's name, without specifying a
     * device tree node.
     */
    quirks = of_device_get_match_data(dev) as *const sun8i_codec_analog_quirks;

    /* Add controls, widgets, and routes for individual features */
    ret = sun8i_codec_analog_add_mixer(cmpnt, quirks);
    if ret != 0 {
        return ret;
    }

    if (*quirks).has_headphone {
        ret = sun8i_codec_add_headphone(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    if (*quirks).has_hmic {
        ret = sun8i_codec_add_hmic(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    if (*quirks).has_linein {
        ret = sun8i_codec_add_linein(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    if (*quirks).has_lineout {
        ret = sun8i_codec_add_lineout(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    if (*quirks).has_mbias {
        ret = sun8i_codec_add_mbias(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    if (*quirks).has_mic2 {
        ret = sun8i_codec_add_mic2(cmpnt);
        if ret != 0 {
            return ret;
        }
    }

    0
}

static sun8i_codec_analog_cmpnt_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: unsafe { sun8i_codec_common_controls.as_ptr() },
    num_controls: unsafe { ARRAY_SIZE(&sun8i_codec_common_controls) },
    dapm_widgets: unsafe { sun8i_codec_common_widgets.as_ptr() },
    num_dapm_widgets: unsafe { ARRAY_SIZE(&sun8i_codec_common_widgets) },
    dapm_routes: sun8i_codec_common_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&sun8i_codec_common_routes),
    probe: Some(sun8i_codec_analog_cmpnt_probe),
};

static sun8i_codec_analog_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: c"allwinner,sun8i-a23-codec-analog".as_ptr(),
        data: &sun8i_a23_quirks as *const sun8i_codec_analog_quirks as *const c_void,
    },
    of_device_id {
        compatible: c"allwinner,sun8i-h3-codec-analog".as_ptr(),
        data: &sun8i_h3_quirks as *const sun8i_codec_analog_quirks as *const c_void,
    },
    of_device_id {
        compatible: c"allwinner,sun8i-v3s-codec-analog".as_ptr(),
        data: &sun8i_v3s_quirks as *const sun8i_codec_analog_quirks as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, sun8i_codec_analog_of_match);

unsafe extern "C" fn sun8i_codec_analog_probe(pdev: *mut platform_device) -> c_int {
    let regmap: *mut regmap;
    let base: *mut c_void;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap = sun8i_adda_pr_regmap_init(&mut (*pdev).dev, base);
    if IS_ERR(regmap as *const c_void) {
        return dev_err_probe(&mut (*pdev).dev, PTR_ERR(regmap as *const c_void), c"Failed to create regmap\n".as_ptr());
    }

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sun8i_codec_analog_cmpnt_drv,
        ptr::null(),
        0,
    )
}

static mut sun8i_codec_analog_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"sun8i-codec-analog".as_ptr(),
        of_match_table: sun8i_codec_analog_of_match.as_ptr(),
    },
    probe: Some(sun8i_codec_analog_probe),
};
// module_platform_driver(sun8i_codec_analog_driver);

// MODULE_DESCRIPTION("Allwinner internal codec analog controls driver");
// MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:sun8i-codec-analog");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
