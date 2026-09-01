// SPDX-License-Identifier: GPL-2.0+
/*
 * This driver supports the analog controls for the internal codec
 * found in Allwinner's A64 SoC.
 *
 * Copyright (C) 2016 Chen-Yu Tsai <wens@csie.org>
 * Copyright (C) 2017 Marcus Cooper <codekipper@gmail.com>
 * Copyright (C) 2018 Vasily Khoruzhick <anarsoul@gmail.com>
 *
 * Based on sun8i-codec-analog.c
 *
 */

/* Dependencies from linux/io.h, linux/kernel.h, linux/module.h,
 * linux/platform_device.h, linux/regmap.h, sound/soc.h, sound/soc-dapm.h,
 * sound/tlv.h, and "sun8i-adda-pr-regmap.h" are expected from the target tree.
 */

type c_int = i32;
type u32 = u32;

extern "C" {
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static TLV_DB_GAIN_MUTE: c_int;

    fn BIT(nr: c_int) -> u32;
    fn ARRAY_SIZE<T, const N: usize>(array: &[T; N]) -> usize;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_int, mask: u32, val: u32) -> c_int;
    fn regmap_clear_bits(regmap: *mut regmap, reg: c_int, mask: u32) -> c_int;
    fn regmap_set_bits(regmap: *mut regmap, reg: c_int, mask: u32) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_pin_status(dapm: *mut snd_soc_dapm_context, pin: *const u8) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_int) -> *mut core::ffi::c_void;
    fn sun8i_adda_pr_regmap_init(dev: *mut device, base: *mut core::ffi::c_void) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const u8, ...) -> c_int;
    fn device_property_read_bool(dev: *mut device, propname: *const u8) -> bool;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut core::ffi::c_void,
        num_dai: c_int,
    ) -> c_int;
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
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const u8,
    control: *const u8,
    source: *const u8,
}

#[repr(C)]
pub struct snd_soc_component {
    regmap: *mut regmap,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_OFF,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_ON,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    controls: *const snd_kcontrol_new,
    num_controls: usize,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: usize,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: usize,
    set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    idle_bias_on: bool,
    suspend_bias_off: bool,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const u8,
}

#[repr(C)]
pub struct device_driver {
    name: *const u8,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

/* Codec analog control register offsets and bit fields */
const SUN50I_ADDA_HP_CTRL: c_int = 0x00;
const SUN50I_ADDA_HP_CTRL_PA_CLK_GATE: c_int = 7;
const SUN50I_ADDA_HP_CTRL_HPPA_EN: c_int = 6;
const SUN50I_ADDA_HP_CTRL_HPVOL: c_int = 0;

const SUN50I_ADDA_OL_MIX_CTRL: c_int = 0x01;
const SUN50I_ADDA_OL_MIX_CTRL_MIC1: c_int = 6;
const SUN50I_ADDA_OL_MIX_CTRL_MIC2: c_int = 5;
const SUN50I_ADDA_OL_MIX_CTRL_PHONE: c_int = 4;
const SUN50I_ADDA_OL_MIX_CTRL_PHONEN: c_int = 3;
const SUN50I_ADDA_OL_MIX_CTRL_LINEINL: c_int = 2;
const SUN50I_ADDA_OL_MIX_CTRL_DACL: c_int = 1;
const SUN50I_ADDA_OL_MIX_CTRL_DACR: c_int = 0;

const SUN50I_ADDA_OR_MIX_CTRL: c_int = 0x02;
const SUN50I_ADDA_OR_MIX_CTRL_MIC1: c_int = 6;
const SUN50I_ADDA_OR_MIX_CTRL_MIC2: c_int = 5;
const SUN50I_ADDA_OR_MIX_CTRL_PHONE: c_int = 4;
const SUN50I_ADDA_OR_MIX_CTRL_PHONEP: c_int = 3;
const SUN50I_ADDA_OR_MIX_CTRL_LINEINR: c_int = 2;
const SUN50I_ADDA_OR_MIX_CTRL_DACR: c_int = 1;
const SUN50I_ADDA_OR_MIX_CTRL_DACL: c_int = 0;

const SUN50I_ADDA_EARPIECE_CTRL0: c_int = 0x03;
const SUN50I_ADDA_EARPIECE_CTRL0_EAR_RAMP_TIME: c_int = 4;
const SUN50I_ADDA_EARPIECE_CTRL0_ESPSR: c_int = 0;

const SUN50I_ADDA_EARPIECE_CTRL1: c_int = 0x04;
const SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_EN: c_int = 7;
const SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_MUTE: c_int = 6;
const SUN50I_ADDA_EARPIECE_CTRL1_ESP_VOL: c_int = 0;

const SUN50I_ADDA_LINEOUT_CTRL0: c_int = 0x05;
const SUN50I_ADDA_LINEOUT_CTRL0_LEN: c_int = 7;
const SUN50I_ADDA_LINEOUT_CTRL0_REN: c_int = 6;
const SUN50I_ADDA_LINEOUT_CTRL0_LSRC_SEL: c_int = 5;
const SUN50I_ADDA_LINEOUT_CTRL0_RSRC_SEL: c_int = 4;

const SUN50I_ADDA_LINEOUT_CTRL1: c_int = 0x06;
const SUN50I_ADDA_LINEOUT_CTRL1_VOL: c_int = 0;

const SUN50I_ADDA_MIC1_CTRL: c_int = 0x07;
const SUN50I_ADDA_MIC1_CTRL_MIC1G: c_int = 4;
const SUN50I_ADDA_MIC1_CTRL_MIC1AMPEN: c_int = 3;
const SUN50I_ADDA_MIC1_CTRL_MIC1BOOST: c_int = 0;

const SUN50I_ADDA_MIC2_CTRL: c_int = 0x08;
const SUN50I_ADDA_MIC2_CTRL_MIC2G: c_int = 4;
const SUN50I_ADDA_MIC2_CTRL_MIC2AMPEN: c_int = 3;
const SUN50I_ADDA_MIC2_CTRL_MIC2BOOST: c_int = 0;

const SUN50I_ADDA_LINEIN_CTRL: c_int = 0x09;
const SUN50I_ADDA_LINEIN_CTRL_LINEING: c_int = 0;

const SUN50I_ADDA_MIX_DAC_CTRL: c_int = 0x0a;
const SUN50I_ADDA_MIX_DAC_CTRL_DACAREN: c_int = 7;
const SUN50I_ADDA_MIX_DAC_CTRL_DACALEN: c_int = 6;
const SUN50I_ADDA_MIX_DAC_CTRL_RMIXEN: c_int = 5;
const SUN50I_ADDA_MIX_DAC_CTRL_LMIXEN: c_int = 4;
const SUN50I_ADDA_MIX_DAC_CTRL_RHPPAMUTE: c_int = 3;
const SUN50I_ADDA_MIX_DAC_CTRL_LHPPAMUTE: c_int = 2;
const SUN50I_ADDA_MIX_DAC_CTRL_RHPIS: c_int = 1;
const SUN50I_ADDA_MIX_DAC_CTRL_LHPIS: c_int = 0;

const SUN50I_ADDA_L_ADCMIX_SRC: c_int = 0x0b;
const SUN50I_ADDA_L_ADCMIX_SRC_MIC1: c_int = 6;
const SUN50I_ADDA_L_ADCMIX_SRC_MIC2: c_int = 5;
const SUN50I_ADDA_L_ADCMIX_SRC_PHONE: c_int = 4;
const SUN50I_ADDA_L_ADCMIX_SRC_PHONEN: c_int = 3;
const SUN50I_ADDA_L_ADCMIX_SRC_LINEINL: c_int = 2;
const SUN50I_ADDA_L_ADCMIX_SRC_OMIXRL: c_int = 1;
const SUN50I_ADDA_L_ADCMIX_SRC_OMIXRR: c_int = 0;

const SUN50I_ADDA_R_ADCMIX_SRC: c_int = 0x0c;
const SUN50I_ADDA_R_ADCMIX_SRC_MIC1: c_int = 6;
const SUN50I_ADDA_R_ADCMIX_SRC_MIC2: c_int = 5;
const SUN50I_ADDA_R_ADCMIX_SRC_PHONE: c_int = 4;
const SUN50I_ADDA_R_ADCMIX_SRC_PHONEP: c_int = 3;
const SUN50I_ADDA_R_ADCMIX_SRC_LINEINR: c_int = 2;
const SUN50I_ADDA_R_ADCMIX_SRC_OMIXR: c_int = 1;
const SUN50I_ADDA_R_ADCMIX_SRC_OMIXL: c_int = 0;

const SUN50I_ADDA_ADC_CTRL: c_int = 0x0d;
const SUN50I_ADDA_ADC_CTRL_ADCREN: c_int = 7;
const SUN50I_ADDA_ADC_CTRL_ADCLEN: c_int = 6;
const SUN50I_ADDA_ADC_CTRL_ADCG: c_int = 0;

const SUN50I_ADDA_HS_MBIAS_CTRL: c_int = 0x0e;
const SUN50I_ADDA_HS_MBIAS_CTRL_MMICBIASEN: c_int = 7;

const SUN50I_ADDA_MDET_CTRL: c_int = 0x1c;
const SUN50I_ADDA_MDET_CTRL_SELDETADC_FS: c_int = 4;
const SUN50I_ADDA_MDET_CTRL_SELDETADC_DB: c_int = 2;
const SUN50I_ADDA_MDET_CTRL_SELDETADC_BF: c_int = 0;

const SUN50I_ADDA_JACK_MIC_CTRL: c_int = 0x1d;
const SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN: c_int = 7;
const SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN: c_int = 6;
const SUN50I_ADDA_JACK_MIC_CTRL_HMICBIASEN: c_int = 5;
const SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN: c_int = 4;

/* mixer controls */
static sun50i_a64_codec_mixer_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_DOUBLE_R!("Mic1 Playback Switch\0", SUN50I_ADDA_OL_MIX_CTRL, SUN50I_ADDA_OR_MIX_CTRL, SUN50I_ADDA_OL_MIX_CTRL_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R!("Mic2 Playback Switch\0", SUN50I_ADDA_OL_MIX_CTRL, SUN50I_ADDA_OR_MIX_CTRL, SUN50I_ADDA_OL_MIX_CTRL_MIC2, 1, 0),
    SOC_DAPM_DOUBLE_R!("Line In Playback Switch\0", SUN50I_ADDA_OL_MIX_CTRL, SUN50I_ADDA_OR_MIX_CTRL, SUN50I_ADDA_OL_MIX_CTRL_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R!("DAC Playback Switch\0", SUN50I_ADDA_OL_MIX_CTRL, SUN50I_ADDA_OR_MIX_CTRL, SUN50I_ADDA_OL_MIX_CTRL_DACL, 1, 0),
    SOC_DAPM_DOUBLE_R!("DAC Reversed Playback Switch\0", SUN50I_ADDA_OL_MIX_CTRL, SUN50I_ADDA_OR_MIX_CTRL, SUN50I_ADDA_OL_MIX_CTRL_DACR, 1, 0),
];

/* ADC mixer controls */
static sun50i_codec_adc_mixer_controls: [snd_kcontrol_new; 5] = [
    SOC_DAPM_DOUBLE_R!("Mic1 Capture Switch\0", SUN50I_ADDA_L_ADCMIX_SRC, SUN50I_ADDA_R_ADCMIX_SRC, SUN50I_ADDA_L_ADCMIX_SRC_MIC1, 1, 0),
    SOC_DAPM_DOUBLE_R!("Mic2 Capture Switch\0", SUN50I_ADDA_L_ADCMIX_SRC, SUN50I_ADDA_R_ADCMIX_SRC, SUN50I_ADDA_L_ADCMIX_SRC_MIC2, 1, 0),
    SOC_DAPM_DOUBLE_R!("Line In Capture Switch\0", SUN50I_ADDA_L_ADCMIX_SRC, SUN50I_ADDA_R_ADCMIX_SRC, SUN50I_ADDA_L_ADCMIX_SRC_LINEINL, 1, 0),
    SOC_DAPM_DOUBLE_R!("Mixer Capture Switch\0", SUN50I_ADDA_L_ADCMIX_SRC, SUN50I_ADDA_R_ADCMIX_SRC, SUN50I_ADDA_L_ADCMIX_SRC_OMIXRL, 1, 0),
    SOC_DAPM_DOUBLE_R!("Mixer Reversed Capture Switch\0", SUN50I_ADDA_L_ADCMIX_SRC, SUN50I_ADDA_R_ADCMIX_SRC, SUN50I_ADDA_L_ADCMIX_SRC_OMIXRR, 1, 0),
];

static sun50i_codec_out_mixer_pregain_scale: [u32; 0] =
    DECLARE_TLV_DB_SCALE!(sun50i_codec_out_mixer_pregain_scale, -450, 150, 0);
static sun50i_codec_mic_gain_scale: [u32; 0] = DECLARE_TLV_DB_RANGE!(
    sun50i_codec_mic_gain_scale,
    0, 0, TLV_DB_SCALE_ITEM!(0, 0, 0),
    1, 7, TLV_DB_SCALE_ITEM!(2400, 300, 0),
);

static sun50i_codec_hp_vol_scale: [u32; 0] =
    DECLARE_TLV_DB_SCALE!(sun50i_codec_hp_vol_scale, -6300, 100, 1);

static sun50i_codec_lineout_vol_scale: [u32; 0] = DECLARE_TLV_DB_RANGE!(
    sun50i_codec_lineout_vol_scale,
    0, 1, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    2, 31, TLV_DB_SCALE_ITEM!(-4350, 150, 0),
);

static sun50i_codec_earpiece_vol_scale: [u32; 0] = DECLARE_TLV_DB_RANGE!(
    sun50i_codec_earpiece_vol_scale,
    0, 1, TLV_DB_SCALE_ITEM!(TLV_DB_GAIN_MUTE, 0, 1),
    2, 31, TLV_DB_SCALE_ITEM!(-4350, 150, 0),
);

/* volume / mute controls */
static sun50i_a64_codec_controls: [snd_kcontrol_new; 9] = [
    SOC_SINGLE_TLV!("Headphone Playback Volume\0", SUN50I_ADDA_HP_CTRL, SUN50I_ADDA_HP_CTRL_HPVOL, 0x3f, 0, sun50i_codec_hp_vol_scale),
    /* Mixer pre-gain */
    SOC_SINGLE_TLV!("Mic1 Playback Volume\0", SUN50I_ADDA_MIC1_CTRL, SUN50I_ADDA_MIC1_CTRL_MIC1G, 0x7, 0, sun50i_codec_out_mixer_pregain_scale),
    /* Microphone Amp boost gain */
    SOC_SINGLE_TLV!("Mic1 Boost Volume\0", SUN50I_ADDA_MIC1_CTRL, SUN50I_ADDA_MIC1_CTRL_MIC1BOOST, 0x7, 0, sun50i_codec_mic_gain_scale),
    /* Mixer pre-gain */
    SOC_SINGLE_TLV!("Mic2 Playback Volume\0", SUN50I_ADDA_MIC2_CTRL, SUN50I_ADDA_MIC2_CTRL_MIC2G, 0x7, 0, sun50i_codec_out_mixer_pregain_scale),
    /* Microphone Amp boost gain */
    SOC_SINGLE_TLV!("Mic2 Boost Volume\0", SUN50I_ADDA_MIC2_CTRL, SUN50I_ADDA_MIC2_CTRL_MIC2BOOST, 0x7, 0, sun50i_codec_mic_gain_scale),
    /* ADC */
    SOC_SINGLE_TLV!("ADC Gain Capture Volume\0", SUN50I_ADDA_ADC_CTRL, SUN50I_ADDA_ADC_CTRL_ADCG, 0x7, 0, sun50i_codec_out_mixer_pregain_scale),
    /* Mixer pre-gain */
    SOC_SINGLE_TLV!("Line In Playback Volume\0", SUN50I_ADDA_LINEIN_CTRL, SUN50I_ADDA_LINEIN_CTRL_LINEING, 0x7, 0, sun50i_codec_out_mixer_pregain_scale),
    SOC_SINGLE_TLV!("Line Out Playback Volume\0", SUN50I_ADDA_LINEOUT_CTRL1, SUN50I_ADDA_LINEOUT_CTRL1_VOL, 0x1f, 0, sun50i_codec_lineout_vol_scale),
    SOC_SINGLE_TLV!("Earpiece Playback Volume\0", SUN50I_ADDA_EARPIECE_CTRL1, SUN50I_ADDA_EARPIECE_CTRL1_ESP_VOL, 0x1f, 0, sun50i_codec_earpiece_vol_scale),
];

static sun50i_codec_hp_src_enum_text: [*const u8; 2] = [
    b"DAC\0".as_ptr(),
    b"Mixer\0".as_ptr(),
];

static sun50i_codec_hp_src_enum: _ = SOC_ENUM_DOUBLE_DECL!(
    sun50i_codec_hp_src_enum,
    SUN50I_ADDA_MIX_DAC_CTRL,
    SUN50I_ADDA_MIX_DAC_CTRL_LHPIS,
    SUN50I_ADDA_MIX_DAC_CTRL_RHPIS,
    sun50i_codec_hp_src_enum_text,
);

static sun50i_codec_hp_src: [snd_kcontrol_new; 1] = [
    SOC_DAPM_ENUM!("Headphone Source Playback Route\0", sun50i_codec_hp_src_enum),
];

static sun50i_codec_hp_switch: snd_kcontrol_new =
    SOC_DAPM_DOUBLE!("Headphone Playback Switch\0", SUN50I_ADDA_MIX_DAC_CTRL, SUN50I_ADDA_MIX_DAC_CTRL_LHPPAMUTE, SUN50I_ADDA_MIX_DAC_CTRL_RHPPAMUTE, 1, 0);

static sun50i_codec_lineout_src_enum_text: [*const u8; 2] = [
    b"Stereo\0".as_ptr(),
    b"Mono Differential\0".as_ptr(),
];

static sun50i_codec_lineout_src_enum: _ = SOC_ENUM_DOUBLE_DECL!(
    sun50i_codec_lineout_src_enum,
    SUN50I_ADDA_LINEOUT_CTRL0,
    SUN50I_ADDA_LINEOUT_CTRL0_LSRC_SEL,
    SUN50I_ADDA_LINEOUT_CTRL0_RSRC_SEL,
    sun50i_codec_lineout_src_enum_text,
);

static sun50i_codec_lineout_src: [snd_kcontrol_new; 1] = [
    SOC_DAPM_ENUM!("Line Out Source Playback Route\0", sun50i_codec_lineout_src_enum),
];

static sun50i_codec_lineout_switch: snd_kcontrol_new =
    SOC_DAPM_DOUBLE!("Line Out Playback Switch\0", SUN50I_ADDA_LINEOUT_CTRL0, SUN50I_ADDA_LINEOUT_CTRL0_LEN, SUN50I_ADDA_LINEOUT_CTRL0_REN, 1, 0);

static sun50i_codec_earpiece_src_enum_text: [*const u8; 4] = [
    b"DACR\0".as_ptr(),
    b"DACL\0".as_ptr(),
    b"Right Mixer\0".as_ptr(),
    b"Left Mixer\0".as_ptr(),
];

static sun50i_codec_earpiece_src_enum: _ = SOC_ENUM_SINGLE_DECL!(
    sun50i_codec_earpiece_src_enum,
    SUN50I_ADDA_EARPIECE_CTRL0,
    SUN50I_ADDA_EARPIECE_CTRL0_ESPSR,
    sun50i_codec_earpiece_src_enum_text,
);

static sun50i_codec_earpiece_src: [snd_kcontrol_new; 1] = [
    SOC_DAPM_ENUM!("Earpiece Source Playback Route\0", sun50i_codec_earpiece_src_enum),
];

static sun50i_codec_earpiece_switch: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE!("Earpiece Playback Switch\0", SUN50I_ADDA_EARPIECE_CTRL1, SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_MUTE, 1, 0),
];

unsafe extern "C" fn sun50i_codec_hbias_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let value: u32 = (SND_SOC_DAPM_EVENT_ON(event) != 0) as u32;

    regmap_update_bits(
        (*component).regmap,
        SUN50I_ADDA_JACK_MIC_CTRL,
        BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
        value << SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN,
    );

    0
}

static sun50i_a64_codec_widgets: [snd_soc_dapm_widget; 31] = [
    /* DAC */
    SND_SOC_DAPM_DAC!("Left DAC\0", core::ptr::null(), SUN50I_ADDA_MIX_DAC_CTRL, SUN50I_ADDA_MIX_DAC_CTRL_DACALEN, 0),
    SND_SOC_DAPM_DAC!("Right DAC\0", core::ptr::null(), SUN50I_ADDA_MIX_DAC_CTRL, SUN50I_ADDA_MIX_DAC_CTRL_DACAREN, 0),
    /* ADC */
    SND_SOC_DAPM_ADC!("Left ADC\0", core::ptr::null(), SUN50I_ADDA_ADC_CTRL, SUN50I_ADDA_ADC_CTRL_ADCLEN, 0),
    SND_SOC_DAPM_ADC!("Right ADC\0", core::ptr::null(), SUN50I_ADDA_ADC_CTRL, SUN50I_ADDA_ADC_CTRL_ADCREN, 0),
    /*
     * Due to this component and the codec belonging to separate DAPM
     * contexts, we need to manually link the above widgets to their
     * stream widgets at the card level.
     */
    SND_SOC_DAPM_REGULATOR_SUPPLY!("cpvdd\0", 0, 0),
    SND_SOC_DAPM_MUX!("Left Headphone Source\0", SND_SOC_NOPM, 0, 0, sun50i_codec_hp_src),
    SND_SOC_DAPM_MUX!("Right Headphone Source\0", SND_SOC_NOPM, 0, 0, sun50i_codec_hp_src),
    SND_SOC_DAPM_SWITCH!("Left Headphone Switch\0", SND_SOC_NOPM, 0, 0, &sun50i_codec_hp_switch),
    SND_SOC_DAPM_SWITCH!("Right Headphone Switch\0", SND_SOC_NOPM, 0, 0, &sun50i_codec_hp_switch),
    SND_SOC_DAPM_OUT_DRV!("Left Headphone Amp\0", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUT_DRV!("Right Headphone Amp\0", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Headphone Amp\0", SUN50I_ADDA_HP_CTRL, SUN50I_ADDA_HP_CTRL_HPPA_EN, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("HP\0"),
    SND_SOC_DAPM_MUX!("Left Line Out Source\0", SND_SOC_NOPM, 0, 0, sun50i_codec_lineout_src),
    SND_SOC_DAPM_MUX!("Right Line Out Source\0", SND_SOC_NOPM, 0, 0, sun50i_codec_lineout_src),
    SND_SOC_DAPM_SWITCH!("Left Line Out Switch\0", SND_SOC_NOPM, 0, 0, &sun50i_codec_lineout_switch),
    SND_SOC_DAPM_SWITCH!("Right Line Out Switch\0", SND_SOC_NOPM, 0, 0, &sun50i_codec_lineout_switch),
    SND_SOC_DAPM_OUTPUT!("LINEOUT\0"),
    SND_SOC_DAPM_MUX!("Earpiece Source Playback Route\0", SND_SOC_NOPM, 0, 0, sun50i_codec_earpiece_src),
    SOC_MIXER_NAMED_CTL_ARRAY!("Earpiece Switch\0", SND_SOC_NOPM, 0, 0, sun50i_codec_earpiece_switch),
    SND_SOC_DAPM_OUT_DRV!("Earpiece Amp\0", SUN50I_ADDA_EARPIECE_CTRL1, SUN50I_ADDA_EARPIECE_CTRL1_ESPPA_EN, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("EARPIECE\0"),
    /* Microphone inputs */
    SND_SOC_DAPM_INPUT!("MIC1\0"),
    /* Microphone Bias */
    SND_SOC_DAPM_SUPPLY!("MBIAS\0", SUN50I_ADDA_HS_MBIAS_CTRL, SUN50I_ADDA_HS_MBIAS_CTRL_MMICBIASEN, 0, core::ptr::null(), 0),
    /* Mic input path */
    SND_SOC_DAPM_PGA!("Mic1 Amplifier\0", SUN50I_ADDA_MIC1_CTRL, SUN50I_ADDA_MIC1_CTRL_MIC1AMPEN, 0, core::ptr::null(), 0),
    /* Microphone input */
    SND_SOC_DAPM_INPUT!("MIC2\0"),
    /* Microphone Bias */
    SND_SOC_DAPM_SUPPLY!("HBIAS\0", SUN50I_ADDA_JACK_MIC_CTRL, SUN50I_ADDA_JACK_MIC_CTRL_HMICBIASEN, 0, sun50i_codec_hbias_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD),
    /* Mic input path */
    SND_SOC_DAPM_PGA!("Mic2 Amplifier\0", SUN50I_ADDA_MIC2_CTRL, SUN50I_ADDA_MIC2_CTRL_MIC2AMPEN, 0, core::ptr::null(), 0),
    /* Line input */
    SND_SOC_DAPM_INPUT!("LINEIN\0"),
    /* Mixers */
    SND_SOC_DAPM_MIXER!("Left Mixer\0", SUN50I_ADDA_MIX_DAC_CTRL, SUN50I_ADDA_MIX_DAC_CTRL_LMIXEN, 0, sun50i_a64_codec_mixer_controls, ARRAY_SIZE(&sun50i_a64_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Right Mixer\0", SUN50I_ADDA_MIX_DAC_CTRL, SUN50I_ADDA_MIX_DAC_CTRL_RMIXEN, 0, sun50i_a64_codec_mixer_controls, ARRAY_SIZE(&sun50i_a64_codec_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Left ADC Mixer\0", SND_SOC_NOPM, 0, 0, sun50i_codec_adc_mixer_controls, ARRAY_SIZE(&sun50i_codec_adc_mixer_controls)),
    SND_SOC_DAPM_MIXER!("Right ADC Mixer\0", SND_SOC_NOPM, 0, 0, sun50i_codec_adc_mixer_controls, ARRAY_SIZE(&sun50i_codec_adc_mixer_controls)),
];

static sun50i_a64_codec_routes: [snd_soc_dapm_route; 56] = [
    /* Left Mixer Routes */
    snd_soc_dapm_route { sink: b"Left Mixer\0".as_ptr(), control: b"Mic1 Playback Switch\0".as_ptr(), source: b"Mic1 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Mixer\0".as_ptr(), control: b"Mic2 Playback Switch\0".as_ptr(), source: b"Mic2 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Mixer\0".as_ptr(), control: b"Line In Playback Switch\0".as_ptr(), source: b"LINEIN\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Mixer\0".as_ptr(), control: b"DAC Playback Switch\0".as_ptr(), source: b"Left DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Mixer\0".as_ptr(), control: b"DAC Reversed Playback Switch\0".as_ptr(), source: b"Right DAC\0".as_ptr() },
    /* Right Mixer Routes */
    snd_soc_dapm_route { sink: b"Right Mixer\0".as_ptr(), control: b"Mic1 Playback Switch\0".as_ptr(), source: b"Mic1 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Mixer\0".as_ptr(), control: b"Mic2 Playback Switch\0".as_ptr(), source: b"Mic2 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Mixer\0".as_ptr(), control: b"Line In Playback Switch\0".as_ptr(), source: b"LINEIN\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Mixer\0".as_ptr(), control: b"DAC Playback Switch\0".as_ptr(), source: b"Right DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Mixer\0".as_ptr(), control: b"DAC Reversed Playback Switch\0".as_ptr(), source: b"Left DAC\0".as_ptr() },
    /* Left ADC Mixer Routes */
    snd_soc_dapm_route { sink: b"Left ADC Mixer\0".as_ptr(), control: b"Mic1 Capture Switch\0".as_ptr(), source: b"Mic1 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left ADC Mixer\0".as_ptr(), control: b"Mic2 Capture Switch\0".as_ptr(), source: b"Mic2 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left ADC Mixer\0".as_ptr(), control: b"Line In Capture Switch\0".as_ptr(), source: b"LINEIN\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left ADC Mixer\0".as_ptr(), control: b"Mixer Capture Switch\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left ADC Mixer\0".as_ptr(), control: b"Mixer Reversed Capture Switch\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    /* Right ADC Mixer Routes */
    snd_soc_dapm_route { sink: b"Right ADC Mixer\0".as_ptr(), control: b"Mic1 Capture Switch\0".as_ptr(), source: b"Mic1 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right ADC Mixer\0".as_ptr(), control: b"Mic2 Capture Switch\0".as_ptr(), source: b"Mic2 Amplifier\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right ADC Mixer\0".as_ptr(), control: b"Line In Capture Switch\0".as_ptr(), source: b"LINEIN\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right ADC Mixer\0".as_ptr(), control: b"Mixer Capture Switch\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right ADC Mixer\0".as_ptr(), control: b"Mixer Reversed Capture Switch\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    /* ADC Routes */
    snd_soc_dapm_route { sink: b"Left ADC\0".as_ptr(), control: core::ptr::null(), source: b"Left ADC Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right ADC\0".as_ptr(), control: core::ptr::null(), source: b"Right ADC Mixer\0".as_ptr() },
    /* Headphone Routes */
    snd_soc_dapm_route { sink: b"Left Headphone Source\0".as_ptr(), control: b"DAC\0".as_ptr(), source: b"Left DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Headphone Source\0".as_ptr(), control: b"Mixer\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Headphone Switch\0".as_ptr(), control: b"Headphone Playback Switch\0".as_ptr(), source: b"Left Headphone Source\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Headphone Amp\0".as_ptr(), control: core::ptr::null(), source: b"Left Headphone Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Headphone Amp\0".as_ptr(), control: core::ptr::null(), source: b"Headphone Amp\0".as_ptr() },
    snd_soc_dapm_route { sink: b"HP\0".as_ptr(), control: core::ptr::null(), source: b"Left Headphone Amp\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Headphone Source\0".as_ptr(), control: b"DAC\0".as_ptr(), source: b"Right DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Headphone Source\0".as_ptr(), control: b"Mixer\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Headphone Switch\0".as_ptr(), control: b"Headphone Playback Switch\0".as_ptr(), source: b"Right Headphone Source\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Headphone Amp\0".as_ptr(), control: core::ptr::null(), source: b"Right Headphone Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Headphone Amp\0".as_ptr(), control: core::ptr::null(), source: b"Headphone Amp\0".as_ptr() },
    snd_soc_dapm_route { sink: b"HP\0".as_ptr(), control: core::ptr::null(), source: b"Right Headphone Amp\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Headphone Amp\0".as_ptr(), control: core::ptr::null(), source: b"cpvdd\0".as_ptr() },
    /* Microphone Routes */
    snd_soc_dapm_route { sink: b"Mic1 Amplifier\0".as_ptr(), control: core::ptr::null(), source: b"MIC1\0".as_ptr() },
    /* Microphone Routes */
    snd_soc_dapm_route { sink: b"Mic2 Amplifier\0".as_ptr(), control: core::ptr::null(), source: b"MIC2\0".as_ptr() },
    /* Line-out Routes */
    snd_soc_dapm_route { sink: b"Left Line Out Source\0".as_ptr(), control: b"Stereo\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Line Out Source\0".as_ptr(), control: b"Mono Differential\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Line Out Source\0".as_ptr(), control: b"Mono Differential\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Left Line Out Switch\0".as_ptr(), control: b"Line Out Playback Switch\0".as_ptr(), source: b"Left Line Out Source\0".as_ptr() },
    snd_soc_dapm_route { sink: b"LINEOUT\0".as_ptr(), control: core::ptr::null(), source: b"Left Line Out Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Line Out Switch\0".as_ptr(), control: b"Line Out Playback Switch\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Line Out Source\0".as_ptr(), control: b"Stereo\0".as_ptr(), source: b"Right Line Out Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Line Out Source\0".as_ptr(), control: b"Mono Differential\0".as_ptr(), source: b"Left Line Out Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"LINEOUT\0".as_ptr(), control: core::ptr::null(), source: b"Right Line Out Source\0".as_ptr() },
    /* Earpiece Routes */
    snd_soc_dapm_route { sink: b"Earpiece Source Playback Route\0".as_ptr(), control: b"DACL\0".as_ptr(), source: b"Left DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Earpiece Source Playback Route\0".as_ptr(), control: b"DACR\0".as_ptr(), source: b"Right DAC\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Earpiece Source Playback Route\0".as_ptr(), control: b"Left Mixer\0".as_ptr(), source: b"Left Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Earpiece Source Playback Route\0".as_ptr(), control: b"Right Mixer\0".as_ptr(), source: b"Right Mixer\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Earpiece Switch\0".as_ptr(), control: b"Earpiece Playback Switch\0".as_ptr(), source: b"Earpiece Source Playback Route\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Earpiece Amp\0".as_ptr(), control: core::ptr::null(), source: b"Earpiece Switch\0".as_ptr() },
    snd_soc_dapm_route { sink: b"EARPIECE\0".as_ptr(), control: core::ptr::null(), source: b"Earpiece Amp\0".as_ptr() },
];

unsafe extern "C" fn sun50i_a64_codec_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);
    let mut hbias: c_int;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            regmap_clear_bits(
                (*component).regmap,
                SUN50I_ADDA_JACK_MIC_CTRL,
                BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN) | BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
            );

            regmap_set_bits(
                (*component).regmap,
                SUN50I_ADDA_HP_CTRL,
                BIT(SUN50I_ADDA_HP_CTRL_PA_CLK_GATE),
            );
        }
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            regmap_clear_bits(
                (*component).regmap,
                SUN50I_ADDA_HP_CTRL,
                BIT(SUN50I_ADDA_HP_CTRL_PA_CLK_GATE),
            );

            hbias = snd_soc_dapm_get_pin_status(dapm, b"HBIAS\0".as_ptr());
            regmap_update_bits(
                (*component).regmap,
                SUN50I_ADDA_JACK_MIC_CTRL,
                BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN) | BIT(SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
                BIT(SUN50I_ADDA_JACK_MIC_CTRL_JACKDETEN)
                    | ((hbias as u32) << SUN50I_ADDA_JACK_MIC_CTRL_MICADCEN),
            );
        }
        _ => {}
    }

    0
}

static sun50i_codec_analog_cmpnt_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: sun50i_a64_codec_controls.as_ptr(),
    num_controls: sun50i_a64_codec_controls.len(),
    dapm_widgets: sun50i_a64_codec_widgets.as_ptr(),
    num_dapm_widgets: sun50i_a64_codec_widgets.len(),
    dapm_routes: sun50i_a64_codec_routes.as_ptr(),
    num_dapm_routes: sun50i_a64_codec_routes.len(),
    set_bias_level: Some(sun50i_a64_codec_set_bias_level),
    idle_bias_on: true,
    suspend_bias_off: true,
};

static sun50i_codec_analog_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"allwinner,sun50i-a64-codec-analog\0".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, sun50i_codec_analog_of_match);

unsafe extern "C" fn sun50i_codec_analog_probe(pdev: *mut platform_device) -> c_int {
    let mut regmap: *mut regmap;
    let base: *mut core::ffi::c_void;
    let enable: bool;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap = sun8i_adda_pr_regmap_init(&mut (*pdev).dev, base);
    if IS_ERR(regmap as *const core::ffi::c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR(regmap as *const core::ffi::c_void),
            b"Failed to create regmap\n\0".as_ptr(),
        );
    }

    enable = device_property_read_bool(
        &mut (*pdev).dev,
        b"allwinner,internal-bias-resistor\0".as_ptr(),
    );
    regmap_update_bits(
        regmap,
        SUN50I_ADDA_JACK_MIC_CTRL,
        BIT(SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN),
        (enable as u32) << SUN50I_ADDA_JACK_MIC_CTRL_INNERRESEN,
    );

    /* Select sample interval of the ADC sample to 16ms */
    regmap_update_bits(
        regmap,
        SUN50I_ADDA_MDET_CTRL,
        (0x7 << SUN50I_ADDA_MDET_CTRL_SELDETADC_FS)
            | (0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_BF),
        (0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_FS)
            | (0x3 << SUN50I_ADDA_MDET_CTRL_SELDETADC_BF),
    );

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &sun50i_codec_analog_cmpnt_drv,
        core::ptr::null_mut(),
        0,
    )
}

static mut sun50i_codec_analog_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"sun50i-codec-analog\0".as_ptr(),
        of_match_table: sun50i_codec_analog_of_match.as_ptr(),
    },
    probe: Some(sun50i_codec_analog_probe),
};
module_platform_driver!(sun50i_codec_analog_driver);

MODULE_DESCRIPTION!("Allwinner internal codec analog controls driver for A64");
MODULE_AUTHOR!("Vasily Khoruzhick <anarsoul@gmail.com>");
MODULE_LICENSE!("GPL");
MODULE_ALIAS!("platform:sun50i-codec-analog");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
