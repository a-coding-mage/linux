// SPDX-License-Identifier: GPL-2.0
//
// MAX9867 ALSA SoC codec driver
//
// Copyright 2013-2015 Maxim Integrated Products
// Copyright 2018 Ladislav Michl <ladis@linux-mips.org>
//

// Rust translation of dependencies originally included from:
// linux/clk.h, linux/delay.h, linux/i2c.h, linux/module.h, linux/regmap.h,
// sound/pcm_params.h, sound/soc.h, sound/tlv.h, and "max9867.h".

#[repr(C)]
pub struct max9867_priv {
    pub mclk: *mut clk,
    pub regmap: *mut regmap,
    pub constraints: *const snd_pcm_hw_constraint_list,
    pub sysclk: libc::c_uint,
    pub pclk: libc::c_uint,
    pub provider: bool,
    pub dsp_a: bool,
    pub adc_dac_active: libc::c_uint,
}

static max9867_spmode: [&'static str; 8] = [
    "Stereo Diff",
    "Mono Diff",
    "Stereo Cap",
    "Mono Cap",
    "Stereo Single",
    "Mono Single",
    "Stereo Single Fast",
    "Mono Single Fast",
];
static max9867_filter_text: [&'static str; 2] = ["IIR", "FIR"];

static max9867_adc_dac_filter_text: [&'static str; 6] = [
    "Disabled",
    "Elliptical/16/256",
    "Butterworth/16/500",
    "Elliptical/8/256",
    "Butterworth/8/500",
    "Butterworth/8-24",
];

#[repr(C)]
pub enum max9867_adc_dac {
    MAX9867_ADC_LEFT,
    MAX9867_ADC_RIGHT,
    MAX9867_DAC_LEFT,
    MAX9867_DAC_RIGHT,
}

pub unsafe extern "C" fn max9867_adc_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: libc::c_int,
) -> libc::c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let adc_dac: max9867_adc_dac;

    if snd_soc_dapm_widget_name_cmp(w, c"ADCL".as_ptr()) == 0 {
        adc_dac = max9867_adc_dac::MAX9867_ADC_LEFT;
    } else if snd_soc_dapm_widget_name_cmp(w, c"ADCR".as_ptr()) == 0 {
        adc_dac = max9867_adc_dac::MAX9867_ADC_RIGHT;
    } else if snd_soc_dapm_widget_name_cmp(w, c"DACL".as_ptr()) == 0 {
        adc_dac = max9867_adc_dac::MAX9867_DAC_LEFT;
    } else if snd_soc_dapm_widget_name_cmp(w, c"DACR".as_ptr()) == 0 {
        adc_dac = max9867_adc_dac::MAX9867_DAC_RIGHT;
    } else {
        return 0;
    }

    if SND_SOC_DAPM_EVENT_ON(event) != 0 {
        (*max9867).adc_dac_active |= BIT(adc_dac as libc::c_uint);
    } else if SND_SOC_DAPM_EVENT_OFF(event) != 0 {
        (*max9867).adc_dac_active &= !BIT(adc_dac as libc::c_uint);
    }

    0
}

pub unsafe extern "C" fn max9867_filter_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let mut reg: libc::c_uint = 0;
    let ret: libc::c_int;

    ret = regmap_read((*max9867).regmap, MAX9867_CODECFLTR, &mut reg);
    if ret != 0 {
        return -EINVAL;
    }

    if (reg & MAX9867_CODECFLTR_MODE) != 0 {
        (*ucontrol).value.enumerated.item[0] = 1;
    } else {
        (*ucontrol).value.enumerated.item[0] = 0;
    }

    0
}

pub unsafe extern "C" fn max9867_filter_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let mut reg: libc::c_uint = 0;
    let mut mode: libc::c_uint = (*ucontrol).value.enumerated.item[0];
    let ret: libc::c_int;

    if mode > 1 {
        return -EINVAL;
    }

    /* don't allow change if ADC/DAC active */
    if (*max9867).adc_dac_active != 0 {
        return -EBUSY;
    }

    /* read current filter mode */
    ret = regmap_read((*max9867).regmap, MAX9867_CODECFLTR, &mut reg);
    if ret != 0 {
        return -EINVAL;
    }

    if mode != 0 {
        mode = MAX9867_CODECFLTR_MODE;
    }

    /* check if change is needed */
    if (reg & MAX9867_CODECFLTR_MODE) == mode {
        return 0;
    }

    /* shutdown codec before switching filter mode */
    regmap_update_bits((*max9867).regmap, MAX9867_PWRMAN, MAX9867_PWRMAN_SHDN, 0);

    /* switch filter mode */
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_CODECFLTR,
        MAX9867_CODECFLTR_MODE,
        mode,
    );

    /* out of shutdown now */
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_PWRMAN,
        MAX9867_PWRMAN_SHDN,
        MAX9867_PWRMAN_SHDN,
    );

    0
}

static max9867_filter: soc_enum = SOC_ENUM_SINGLE_EXT_DECL!(max9867_filter_text);
static max9867_dac_filter: soc_enum =
    SOC_ENUM_SINGLE_DECL!(MAX9867_CODECFLTR, 0, max9867_adc_dac_filter_text);
static max9867_adc_filter: soc_enum =
    SOC_ENUM_SINGLE_DECL!(MAX9867_CODECFLTR, 4, max9867_adc_dac_filter_text);
static max9867_spkmode: soc_enum = SOC_ENUM_SINGLE_DECL!(MAX9867_MODECONFIG, 0, max9867_spmode);
static max9867_master_tlv: [libc::c_uint; 0] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0,
    2,
    TLV_DB_SCALE_ITEM!(-8600, 200, 1),
    3,
    17,
    TLV_DB_SCALE_ITEM!(-7800, 400, 0),
    18,
    25,
    TLV_DB_SCALE_ITEM!(-2000, 200, 0),
    26,
    34,
    TLV_DB_SCALE_ITEM!(-500, 100, 0),
    35,
    40,
    TLV_DB_SCALE_ITEM!(350, 50, 0),
);
static max9867_mic_tlv: [libc::c_uint; 0] = DECLARE_TLV_DB_SCALE!(0, 100, 0);
static max9867_line_tlv: [libc::c_uint; 0] = DECLARE_TLV_DB_SCALE!(-600, 200, 0);
static max9867_adc_tlv: [libc::c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1200, 100, 0);
static max9867_dac_tlv: [libc::c_uint; 0] = DECLARE_TLV_DB_SCALE!(-1500, 100, 0);
static max9867_dacboost_tlv: [libc::c_uint; 0] = DECLARE_TLV_DB_SCALE!(0, 600, 0);
static max9867_micboost_tlv: [libc::c_uint; 0] = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0,
    2,
    TLV_DB_SCALE_ITEM!(-2000, 2000, 1),
    3,
    3,
    TLV_DB_SCALE_ITEM!(3000, 0, 0),
);

static max9867_snd_controls: [snd_kcontrol_new; 15] = [
    SOC_DOUBLE_R_TLV!("Master Playback Volume", MAX9867_LEFTVOL, MAX9867_RIGHTVOL, 0, 40, 1, max9867_master_tlv),
    SOC_DOUBLE_R_TLV!("Line Capture Volume", MAX9867_LEFTLINELVL, MAX9867_RIGHTLINELVL, 0, 15, 1, max9867_line_tlv),
    SOC_DOUBLE_R_TLV!("Mic Capture Volume", MAX9867_LEFTMICGAIN, MAX9867_RIGHTMICGAIN, 0, 20, 1, max9867_mic_tlv),
    SOC_DOUBLE_R_TLV!("Mic Boost Capture Volume", MAX9867_LEFTMICGAIN, MAX9867_RIGHTMICGAIN, 5, 3, 0, max9867_micboost_tlv),
    SOC_SINGLE!("Digital Sidetone Volume", MAX9867_SIDETONE, 0, 31, 1),
    SOC_SINGLE_TLV!("Digital Playback Volume", MAX9867_DACLEVEL, 0, 15, 1, max9867_dac_tlv),
    SOC_SINGLE_TLV!("Digital Boost Playback Volume", MAX9867_DACLEVEL, 4, 3, 0, max9867_dacboost_tlv),
    SOC_DOUBLE_TLV!("Digital Capture Volume", MAX9867_ADCLEVEL, 4, 0, 15, 1, max9867_adc_tlv),
    SOC_ENUM!("Speaker Mode", max9867_spkmode),
    SOC_SINGLE!("Volume Smoothing Switch", MAX9867_MODECONFIG, 6, 1, 0),
    SOC_SINGLE!("Line ZC Switch", MAX9867_MODECONFIG, 5, 1, 0),
    SOC_ENUM_EXT!("DSP Filter", max9867_filter, max9867_filter_get, max9867_filter_set),
    SOC_ENUM!("ADC Filter", max9867_adc_filter),
    SOC_ENUM!("DAC Filter", max9867_dac_filter),
    SOC_SINGLE!("Mono Playback Switch", MAX9867_IFC1B, 3, 1, 0),
];

/* Input mixer */
static max9867_input_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_DOUBLE!("Line Capture Switch", MAX9867_INPUTCONFIG, 7, 5, 1, 0),
    SOC_DAPM_DOUBLE!("Mic Capture Switch", MAX9867_INPUTCONFIG, 6, 4, 1, 0),
];

/* Output mixer */
static max9867_output_mixer_controls: [snd_kcontrol_new; 1] = [SOC_DAPM_DOUBLE_R!(
    "Line Bypass Switch",
    MAX9867_LEFTLINELVL,
    MAX9867_RIGHTLINELVL,
    6,
    1,
    1,
)];

/* Sidetone mixer */
static max9867_sidetone_mixer_controls: [snd_kcontrol_new; 1] =
    [SOC_DAPM_DOUBLE!("Sidetone Switch", MAX9867_SIDETONE, 6, 7, 1, 0)];

/* Line out switch */
static max9867_line_out_control: snd_kcontrol_new =
    SOC_DAPM_DOUBLE_R!("Switch", MAX9867_LEFTVOL, MAX9867_RIGHTVOL, 6, 1, 1);

/* DMIC mux */
static dmic_mux_text: [&'static str; 2] = ["ADC", "DMIC"];
static left_dmic_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(MAX9867_MICCONFIG, 5, dmic_mux_text);
static right_dmic_mux_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(MAX9867_MICCONFIG, 4, dmic_mux_text);
static max9867_left_dmic_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DMICL Mux", left_dmic_mux_enum);
static max9867_right_dmic_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("DMICR Mux", right_dmic_mux_enum);

static max9867_dapm_widgets: [snd_soc_dapm_widget; 23] = [
    SND_SOC_DAPM_INPUT!("MICL"),
    SND_SOC_DAPM_INPUT!("MICR"),
    SND_SOC_DAPM_INPUT!("DMICL"),
    SND_SOC_DAPM_INPUT!("DMICR"),
    SND_SOC_DAPM_INPUT!("LINL"),
    SND_SOC_DAPM_INPUT!("LINR"),
    SND_SOC_DAPM_PGA!("Left Line Input", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right Line Input", SND_SOC_NOPM, 0, 0, NULL, 0),
    SND_SOC_DAPM_MIXER_NAMED_CTL!("Input Mixer", SND_SOC_NOPM, 0, 0, max9867_input_mixer_controls, ARRAY_SIZE!(max9867_input_mixer_controls)),
    SND_SOC_DAPM_MUX!("DMICL Mux", SND_SOC_NOPM, 0, 0, &max9867_left_dmic_mux),
    SND_SOC_DAPM_MUX!("DMICR Mux", SND_SOC_NOPM, 0, 0, &max9867_right_dmic_mux),
    SND_SOC_DAPM_ADC_E!("ADCL", "HiFi Capture", SND_SOC_NOPM, 0, 0, max9867_adc_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_ADC_E!("ADCR", "HiFi Capture", SND_SOC_NOPM, 0, 0, max9867_adc_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER!("Digital", SND_SOC_NOPM, 0, 0, max9867_sidetone_mixer_controls, ARRAY_SIZE!(max9867_sidetone_mixer_controls)),
    SND_SOC_DAPM_MIXER_NAMED_CTL!("Output Mixer", SND_SOC_NOPM, 0, 0, max9867_output_mixer_controls, ARRAY_SIZE!(max9867_output_mixer_controls)),
    SND_SOC_DAPM_DAC_E!("DACL", "HiFi Playback", SND_SOC_NOPM, 0, 0, max9867_adc_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("DACR", "HiFi Playback", SND_SOC_NOPM, 0, 0, max9867_adc_dac_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SWITCH!("Master Playback", SND_SOC_NOPM, 0, 0, &max9867_line_out_control),
    SND_SOC_DAPM_OUTPUT!("LOUT"),
    SND_SOC_DAPM_OUTPUT!("ROUT"),
];

static max9867_audio_map: [snd_soc_dapm_route; 23] = [
    snd_soc_dapm_route { sink: c"Left Line Input".as_ptr(), control: NULL, source: c"LINL".as_ptr() },
    snd_soc_dapm_route { sink: c"Right Line Input".as_ptr(), control: NULL, source: c"LINR".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Mic Capture Switch".as_ptr(), source: c"MICL".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Mic Capture Switch".as_ptr(), source: c"MICR".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Line Capture Switch".as_ptr(), source: c"Left Line Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Input Mixer".as_ptr(), control: c"Line Capture Switch".as_ptr(), source: c"Right Line Input".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICL Mux".as_ptr(), control: c"DMIC".as_ptr(), source: c"DMICL".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICR Mux".as_ptr(), control: c"DMIC".as_ptr(), source: c"DMICR".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICL Mux".as_ptr(), control: c"ADC".as_ptr(), source: c"Input Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICR Mux".as_ptr(), control: c"ADC".as_ptr(), source: c"Input Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"ADCL".as_ptr(), control: NULL, source: c"DMICL Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"ADCR".as_ptr(), control: NULL, source: c"DMICR Mux".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital".as_ptr(), control: c"Sidetone Switch".as_ptr(), source: c"ADCL".as_ptr() },
    snd_soc_dapm_route { sink: c"Digital".as_ptr(), control: c"Sidetone Switch".as_ptr(), source: c"ADCR".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: NULL, source: c"Digital".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: NULL, source: c"Digital".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: c"Line Bypass Switch".as_ptr(), source: c"Left Line Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: c"Line Bypass Switch".as_ptr(), source: c"Right Line Input".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: NULL, source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"Output Mixer".as_ptr(), control: NULL, source: c"DACR".as_ptr() },
    snd_soc_dapm_route { sink: c"Master Playback".as_ptr(), control: c"Switch".as_ptr(), source: c"Output Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"LOUT".as_ptr(), control: NULL, source: c"Master Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"ROUT".as_ptr(), control: NULL, source: c"Master Playback".as_ptr() },
];

static max9867_rates_44k1: [libc::c_uint; 3] = [11025, 22050, 44100];

static max9867_constraints_44k1: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: max9867_rates_44k1.as_ptr(),
    count: ARRAY_SIZE!(max9867_rates_44k1),
};

static max9867_rates_48k: [libc::c_uint; 4] = [8000, 16000, 32000, 48000];

static max9867_constraints_48k: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: max9867_rates_48k.as_ptr(),
    count: ARRAY_SIZE!(max9867_rates_48k),
};

pub unsafe extern "C" fn max9867_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let max9867: *mut max9867_priv =
        snd_soc_component_get_drvdata((*dai).component) as *mut max9867_priv;

    if !(*max9867).constraints.is_null() {
        snd_pcm_hw_constraint_list(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_RATE,
            (*max9867).constraints,
        );
    }

    0
}

pub unsafe extern "C" fn max9867_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let mut value: libc::c_int;
    let mut freq: libc::c_int = 0;
    let rate: libc::c_ulong;
    let ratio: libc::c_ulong;
    let component: *mut snd_soc_component = (*dai).component;
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let ni: libc::c_uint = DIV_ROUND_CLOSEST_ULL(
        96u64 * 0x10000u64 * params_rate(params) as u64,
        (*max9867).pclk as u64,
    ) as libc::c_uint;

    /* set up the ni value */
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_AUDIOCLKHIGH,
        MAX9867_NI_HIGH_MASK,
        (0xFF00 & ni) >> 8,
    );
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_AUDIOCLKLOW,
        MAX9867_NI_LOW_MASK,
        0x00FF & ni,
    );
    if (*max9867).provider {
        if (*max9867).dsp_a {
            value = MAX9867_IFC1B_48X as libc::c_int;
        } else {
            rate = (params_rate(params) * 2 * params_width(params)) as libc::c_ulong;
            ratio = ((*max9867).pclk as libc::c_ulong) / rate;
            match params_width(params) {
                8 | 16 => {
                    match ratio {
                        2 => value = MAX9867_IFC1B_PCLK_2 as libc::c_int,
                        4 => value = MAX9867_IFC1B_PCLK_4 as libc::c_int,
                        8 => value = MAX9867_IFC1B_PCLK_8 as libc::c_int,
                        16 => value = MAX9867_IFC1B_PCLK_16 as libc::c_int,
                        _ => return -EINVAL,
                    }
                }
                24 => value = MAX9867_IFC1B_48X as libc::c_int,
                32 => value = MAX9867_IFC1B_64X as libc::c_int,
                _ => return -EINVAL,
            }
        }
        regmap_update_bits(
            (*max9867).regmap,
            MAX9867_IFC1B,
            MAX9867_IFC1B_BCLK_MASK,
            value as libc::c_uint,
        );

        /* Exact integer mode available for 8kHz and 16kHz sample rates
         * and certain PCLK (prescaled MCLK) values.
         */
        if params_rate(params) == 8000 || params_rate(params) == 16000 {
            match (*max9867).pclk {
                12000000 => freq = 0x08,
                13000000 => freq = 0x0A,
                16000000 => freq = 0x0C,
                19200000 => freq = 0x0E,
                _ => {}
            }
        }
        if freq != 0 && params_rate(params) == 16000 {
            freq += 1;
        }

        /* If exact integer mode not available, the freq value
         * remains zero, i.e. normal mode is used.
         */
        regmap_update_bits(
            (*max9867).regmap,
            MAX9867_SYSCLK,
            MAX9867_FREQ_MASK,
            freq as libc::c_uint,
        );
    } else {
        /*
         * digital pll locks on to any externally supplied LRCLK signal
         * and also enable rapid lock mode.
         */
        regmap_update_bits(
            (*max9867).regmap,
            MAX9867_AUDIOCLKLOW,
            MAX9867_RAPID_LOCK,
            MAX9867_RAPID_LOCK,
        );
        regmap_update_bits(
            (*max9867).regmap,
            MAX9867_AUDIOCLKHIGH,
            MAX9867_PLL,
            MAX9867_PLL,
        );
    }
    0
}

pub unsafe extern "C" fn max9867_mute(
    dai: *mut snd_soc_dai,
    mute: libc::c_int,
    _direction: libc::c_int,
) -> libc::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;

    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_DACLEVEL,
        1 << 6,
        ((mute != 0) as libc::c_uint) << 6,
    )
}

pub unsafe extern "C" fn max9867_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: libc::c_int,
    freq: libc::c_uint,
    _dir: libc::c_int,
) -> libc::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let mut value: libc::c_int = 0;

    /* Set the prescaler based on the master clock frequency*/
    if freq >= 10000000 && freq <= 20000000 {
        value |= MAX9867_PSCLK_10_20 as libc::c_int;
        (*max9867).pclk = freq;
    } else if freq >= 20000000 && freq <= 40000000 {
        value |= MAX9867_PSCLK_20_40 as libc::c_int;
        (*max9867).pclk = freq / 2;
    } else if freq >= 40000000 && freq <= 60000000 {
        value |= MAX9867_PSCLK_40_60 as libc::c_int;
        (*max9867).pclk = freq / 4;
    } else {
        dev_err(
            (*component).dev,
            c"Invalid clock frequency %uHz (required 10-60MHz)\n".as_ptr(),
            freq,
        );
        return -EINVAL;
    }
    if freq % 48000 == 0 {
        (*max9867).constraints = &max9867_constraints_48k;
    } else if freq % 44100 == 0 {
        (*max9867).constraints = &max9867_constraints_44k1;
    } else {
        dev_warn(
            (*component).dev,
            c"Unable to set exact rate with %uHz clock frequency\n".as_ptr(),
            freq,
        );
    }
    (*max9867).sysclk = freq;
    value <<= MAX9867_PSCLK_SHIFT;
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_SYSCLK,
        MAX9867_PSCLK_MASK,
        value as libc::c_uint,
    );
    0
}

pub unsafe extern "C" fn max9867_dai_set_fmt(
    codec_dai: *mut snd_soc_dai,
    fmt: libc::c_uint,
) -> libc::c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let mut iface1A: u8;
    let mut iface1B: u8;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            (*max9867).provider = true;
            iface1A = MAX9867_MASTER as u8;
            iface1B = MAX9867_IFC1B_48X as u8;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            (*max9867).provider = false;
            iface1A = 0;
            iface1B = 0;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*max9867).dsp_a = false;
            iface1A |= MAX9867_I2S_DLY as u8;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*max9867).dsp_a = true;
            iface1A |= (MAX9867_TDM_MODE | MAX9867_SDOUT_HIZ) as u8;
        }
        _ => return -EINVAL,
    }

    /* Clock inversion bits, BCI and WCI */
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_IF => {
            iface1A |= (MAX9867_WCI_MODE | MAX9867_BCI_MODE) as u8;
        }
        SND_SOC_DAIFMT_IB_NF => {
            iface1A |= MAX9867_BCI_MODE as u8;
        }
        SND_SOC_DAIFMT_NB_IF => {
            iface1A |= MAX9867_WCI_MODE as u8;
        }
        _ => return -EINVAL,
    }

    regmap_write((*max9867).regmap, MAX9867_IFC1A, iface1A as libc::c_uint);
    regmap_update_bits(
        (*max9867).regmap,
        MAX9867_IFC1B,
        MAX9867_IFC1B_BCLK_MASK,
        iface1B as libc::c_uint,
    );

    0
}

static max9867_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(max9867_set_dai_sysclk),
    set_fmt: Some(max9867_dai_set_fmt),
    mute_stream: Some(max9867_mute),
    startup: Some(max9867_startup),
    hw_params: Some(max9867_dai_hw_params),
    no_capture_mute: 1,
};

static mut max9867_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"max9867-aif1".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HiFi Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &max9867_dai_ops,
    symmetric_rate: 1,
}];

// Original C conditional: #ifdef CONFIG_PM.
#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn max9867_suspend(component: *mut snd_soc_component) -> libc::c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_OFF);

    0
}

#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn max9867_resume(component: *mut snd_soc_component) -> libc::c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);

    0
}

// Original C #else maps max9867_suspend and max9867_resume to NULL.
#[cfg(not(CONFIG_PM))]
static max9867_suspend: *const libc::c_void = NULL;
#[cfg(not(CONFIG_PM))]
static max9867_resume: *const libc::c_void = NULL;

pub unsafe extern "C" fn max9867_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> libc::c_int {
    let mut err: libc::c_int;
    let max9867: *mut max9867_priv = snd_soc_component_get_drvdata(component) as *mut max9867_priv;
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_ON => {
            err = clk_prepare_enable((*max9867).mclk);
            if err != 0 {
                return err;
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                err = regcache_sync((*max9867).regmap);
                if err != 0 {
                    return err;
                }

                err = regmap_write((*max9867).regmap, MAX9867_PWRMAN, 0xff);
                if err != 0 {
                    return err;
                }
            }
        }
        SND_SOC_BIAS_OFF => {
            err = regmap_write((*max9867).regmap, MAX9867_PWRMAN, 0);
            if err != 0 {
                return err;
            }

            regcache_mark_dirty((*max9867).regmap);
            clk_disable_unprepare((*max9867).mclk);
        }
        _ => {}
    }

    0
}

static max9867_component: snd_soc_component_driver = snd_soc_component_driver {
    controls: max9867_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(max9867_snd_controls),
    dapm_routes: max9867_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(max9867_audio_map),
    dapm_widgets: max9867_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(max9867_dapm_widgets),
    suspend: max9867_suspend,
    resume: max9867_resume,
    set_bias_level: Some(max9867_set_bias_level),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

pub unsafe extern "C" fn max9867_volatile_register(
    _dev: *mut device,
    reg: libc::c_uint,
) -> bool {
    match reg {
        MAX9867_STATUS | MAX9867_JACKSTATUS | MAX9867_AUXHIGH | MAX9867_AUXLOW => true,
        _ => false,
    }
}

static max9867_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: MAX9867_REVISION,
    volatile_reg: Some(max9867_volatile_register),
    cache_type: REGCACHE_RBTREE,
};

pub unsafe extern "C" fn max9867_i2c_probe(i2c: *mut i2c_client) -> libc::c_int {
    let mut max9867: *mut max9867_priv;
    let mut ret: libc::c_int;
    let mut reg: libc::c_int = 0;

    max9867 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<max9867_priv>(),
        GFP_KERNEL,
    ) as *mut max9867_priv;
    if max9867.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, max9867 as *mut libc::c_void);
    (*max9867).regmap = devm_regmap_init_i2c(i2c, &max9867_regmap);
    if IS_ERR((*max9867).regmap as *const libc::c_void) != 0 {
        ret = PTR_ERR((*max9867).regmap as *const libc::c_void);
        dev_err(
            &mut (*i2c).dev,
            c"Failed to allocate regmap: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = regmap_read((*max9867).regmap, MAX9867_REVISION, &mut reg as *mut _ as *mut libc::c_uint);
    if ret < 0 {
        dev_err(&mut (*i2c).dev, c"Failed to read: %d\n".as_ptr(), ret);
        return ret;
    }
    dev_info(&mut (*i2c).dev, c"device revision: %x\n".as_ptr(), reg);
    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &max9867_component,
        max9867_dai.as_mut_ptr(),
        ARRAY_SIZE!(max9867_dai),
    );
    if ret < 0 {
        dev_err(
            &mut (*i2c).dev,
            c"Failed to register component: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    (*max9867).mclk = devm_clk_get(&mut (*i2c).dev, NULL);
    if IS_ERR((*max9867).mclk as *const libc::c_void) != 0 {
        return PTR_ERR((*max9867).mclk as *const libc::c_void);
    }

    0
}

static max9867_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: c"max9867".as_ptr(),
    },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, max9867_i2c_id);

// Original C conditional: #ifdef CONFIG_OF.
#[cfg(CONFIG_OF)]
static max9867_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"maxim,max9867".as_ptr(),
    },
    of_device_id::default(),
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, max9867_of_match);

static mut max9867_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max9867".as_ptr(),
        of_match_table: of_match_ptr!(max9867_of_match),
    },
    probe: Some(max9867_i2c_probe),
    id_table: max9867_i2c_id.as_ptr(),
};

module_i2c_driver!(max9867_i2c_driver);

MODULE_AUTHOR!("Ladislav Michl <ladis@linux-mips.org>");
MODULE_DESCRIPTION!("ASoC MAX9867 driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
