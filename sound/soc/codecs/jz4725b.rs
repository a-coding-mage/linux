// SPDX-License-Identifier: GPL-2.0
//
// JZ4725B CODEC driver
//
// Copyright (C) 2019, Paul Cercueil <paul@crapouillou.net>

// Translated from Linux kernel C source. Kernel/ALSA helper macros, types, and
// registration helpers referenced here are external dependencies of the driver.

const ICDC_RGADW_OFFSET: usize = 0x00;
const ICDC_RGDATA_OFFSET: usize = 0x04;

/* ICDC internal register access control register(RGADW) */
const ICDC_RGADW_RGWR: u32 = BIT(16);

const ICDC_RGADW_RGADDR_OFFSET: u32 = 8;
const ICDC_RGADW_RGADDR_MASK: u32 = GENMASK(14, ICDC_RGADW_RGADDR_OFFSET);

const ICDC_RGADW_RGDIN_OFFSET: u32 = 0;
const ICDC_RGADW_RGDIN_MASK: u32 = GENMASK(7, ICDC_RGADW_RGDIN_OFFSET);

/* ICDC internal register data output register (RGDATA)*/
const ICDC_RGDATA_IRQ: u32 = BIT(8);

const ICDC_RGDATA_RGDOUT_OFFSET: u32 = 0;
const ICDC_RGDATA_RGDOUT_MASK: u32 = GENMASK(7, ICDC_RGDATA_RGDOUT_OFFSET);

/* JZ internal register space */
const JZ4725B_CODEC_REG_AICR: u32 = 0;
const JZ4725B_CODEC_REG_CR1: u32 = 1;
const JZ4725B_CODEC_REG_CR2: u32 = 2;
const JZ4725B_CODEC_REG_CCR1: u32 = 3;
const JZ4725B_CODEC_REG_CCR2: u32 = 4;
const JZ4725B_CODEC_REG_PMR1: u32 = 5;
const JZ4725B_CODEC_REG_PMR2: u32 = 6;
const JZ4725B_CODEC_REG_CRR: u32 = 7;
const JZ4725B_CODEC_REG_ICR: u32 = 8;
const JZ4725B_CODEC_REG_IFR: u32 = 9;
const JZ4725B_CODEC_REG_CGR1: u32 = 10;
const JZ4725B_CODEC_REG_CGR2: u32 = 11;
const JZ4725B_CODEC_REG_CGR3: u32 = 12;
const JZ4725B_CODEC_REG_CGR4: u32 = 13;
const JZ4725B_CODEC_REG_CGR5: u32 = 14;
const JZ4725B_CODEC_REG_CGR6: u32 = 15;
const JZ4725B_CODEC_REG_CGR7: u32 = 16;
const JZ4725B_CODEC_REG_CGR8: u32 = 17;
const JZ4725B_CODEC_REG_CGR9: u32 = 18;
const JZ4725B_CODEC_REG_CGR10: u32 = 19;
const JZ4725B_CODEC_REG_TR1: u32 = 20;
const JZ4725B_CODEC_REG_TR2: u32 = 21;
const JZ4725B_CODEC_REG_CR3: u32 = 22;
const JZ4725B_CODEC_REG_AGC1: u32 = 23;
const JZ4725B_CODEC_REG_AGC2: u32 = 24;
const JZ4725B_CODEC_REG_AGC3: u32 = 25;
const JZ4725B_CODEC_REG_AGC4: u32 = 26;
const JZ4725B_CODEC_REG_AGC5: u32 = 27;

const REG_AICR_CONFIG1_OFFSET: u32 = 0;
const REG_AICR_CONFIG1_MASK: u32 = 0xf << REG_AICR_CONFIG1_OFFSET;

const REG_CR1_SB_MICBIAS_OFFSET: u32 = 7;
const REG_CR1_MONO_OFFSET: u32 = 6;
const REG_CR1_DAC_MUTE_OFFSET: u32 = 5;
const REG_CR1_HP_DIS_OFFSET: u32 = 4;
const REG_CR1_DACSEL_OFFSET: u32 = 3;
const REG_CR1_BYPASS_OFFSET: u32 = 2;

const REG_CR2_DAC_DEEMP_OFFSET: u32 = 7;
const REG_CR2_DAC_ADWL_OFFSET: u32 = 5;
const REG_CR2_DAC_ADWL_MASK: u32 = 0x3 << REG_CR2_DAC_ADWL_OFFSET;
const REG_CR2_ADC_ADWL_OFFSET: u32 = 3;
const REG_CR2_ADC_ADWL_MASK: u32 = 0x3 << REG_CR2_ADC_ADWL_OFFSET;
const REG_CR2_ADC_HPF_OFFSET: u32 = 2;

const REG_CR3_SB_MIC1_OFFSET: u32 = 7;
const REG_CR3_SB_MIC2_OFFSET: u32 = 6;
const REG_CR3_SIDETONE1_OFFSET: u32 = 5;
const REG_CR3_SIDETONE2_OFFSET: u32 = 4;
const REG_CR3_MICDIFF_OFFSET: u32 = 3;
const REG_CR3_MICSTEREO_OFFSET: u32 = 2;
const REG_CR3_INSEL_OFFSET: u32 = 0;
const REG_CR3_INSEL_MASK: u32 = 0x3 << REG_CR3_INSEL_OFFSET;

const REG_CCR1_CONFIG4_OFFSET: u32 = 0;
const REG_CCR1_CONFIG4_MASK: u32 = 0xf << REG_CCR1_CONFIG4_OFFSET;

const REG_CCR2_DFREQ_OFFSET: u32 = 4;
const REG_CCR2_DFREQ_MASK: u32 = 0xf << REG_CCR2_DFREQ_OFFSET;
const REG_CCR2_AFREQ_OFFSET: u32 = 0;
const REG_CCR2_AFREQ_MASK: u32 = 0xf << REG_CCR2_AFREQ_OFFSET;

const REG_PMR1_SB_DAC_OFFSET: u32 = 7;
const REG_PMR1_SB_OUT_OFFSET: u32 = 6;
const REG_PMR1_SB_MIX_OFFSET: u32 = 5;
const REG_PMR1_SB_ADC_OFFSET: u32 = 4;
const REG_PMR1_SB_LIN_OFFSET: u32 = 3;
const REG_PMR1_SB_IND_OFFSET: u32 = 0;

const REG_PMR2_LRGI_OFFSET: u32 = 7;
const REG_PMR2_RLGI_OFFSET: u32 = 6;
const REG_PMR2_LRGOD_OFFSET: u32 = 5;
const REG_PMR2_RLGOD_OFFSET: u32 = 4;
const REG_PMR2_GIM_OFFSET: u32 = 3;
const REG_PMR2_SB_MC_OFFSET: u32 = 2;
const REG_PMR2_SB_OFFSET: u32 = 1;
const REG_PMR2_SB_SLEEP_OFFSET: u32 = 0;

const REG_IFR_RAMP_UP_DONE_OFFSET: u32 = 3;
const REG_IFR_RAMP_DOWN_DONE_OFFSET: u32 = 2;

const REG_CGR1_GODL_OFFSET: u32 = 4;
const REG_CGR1_GODL_MASK: u32 = 0xf << REG_CGR1_GODL_OFFSET;
const REG_CGR1_GODR_OFFSET: u32 = 0;
const REG_CGR1_GODR_MASK: u32 = 0xf << REG_CGR1_GODR_OFFSET;

const REG_CGR2_GO1R_OFFSET: u32 = 0;
const REG_CGR2_GO1R_MASK: u32 = 0x1f << REG_CGR2_GO1R_OFFSET;

const REG_CGR3_GO1L_OFFSET: u32 = 0;
const REG_CGR3_GO1L_MASK: u32 = 0x1f << REG_CGR3_GO1L_OFFSET;

const REG_CGR4_GO2R_OFFSET: u32 = 0;
const REG_CGR4_GO2R_MASK: u32 = 0x1f << REG_CGR4_GO2R_OFFSET;

const REG_CGR5_GO2L_OFFSET: u32 = 0;
const REG_CGR5_GO2L_MASK: u32 = 0x1f << REG_CGR5_GO2L_OFFSET;

const REG_CGR6_GO3R_OFFSET: u32 = 0;
const REG_CGR6_GO3R_MASK: u32 = 0x1f << REG_CGR6_GO3R_OFFSET;

const REG_CGR7_GO3L_OFFSET: u32 = 0;
const REG_CGR7_GO3L_MASK: u32 = 0x1f << REG_CGR7_GO3L_OFFSET;

const REG_CGR8_GOR_OFFSET: u32 = 0;
const REG_CGR8_GOR_MASK: u32 = 0x1f << REG_CGR8_GOR_OFFSET;

const REG_CGR9_GOL_OFFSET: u32 = 0;
const REG_CGR9_GOL_MASK: u32 = 0x1f << REG_CGR9_GOL_OFFSET;

const REG_CGR10_GIL_OFFSET: u32 = 0;
const REG_CGR10_GIR_OFFSET: u32 = 4;

#[repr(C)]
struct jz_icdc {
    regmap: *mut regmap,
    base: *mut core::ffi::c_void,
}

static jz4725b_adc_tlv: snd_ctl_tlv = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(0, 150, 0);
static jz4725b_dac_tlv: snd_ctl_tlv = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(-2250, 150, 0);
static jz4725b_mix_tlv: snd_ctl_tlv = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 11, TLV_DB_SCALE_ITEM!(-2250, 0, 0),
    12, 31, TLV_DB_SCALE_ITEM!(-2250, 150, 0),
);

static jz4725b_out_tlv: snd_ctl_tlv = SNDRV_CTL_TLVD_DECLARE_DB_RANGE!(
    0, 11, TLV_DB_SCALE_ITEM!(-3350, 200, 0),
    12, 23, TLV_DB_SCALE_ITEM!(-1050, 100, 0),
    24, 31, TLV_DB_SCALE_ITEM!(100, 50, 0),
);
static jz4725b_mic_boost_tlv: snd_ctl_tlv = SNDRV_CTL_TLVD_DECLARE_DB_SCALE!(0, 2000, 0);

static jz4725b_mic_mode_texts: [*const i8; 2] = [
    c"Single Ended".as_ptr(),
    c"Differential".as_ptr(),
];

static jz4725b_mic_mode_enum: soc_enum = SOC_ENUM_SINGLE!(
    JZ4725B_CODEC_REG_CR3,
    REG_CR3_MICDIFF_OFFSET,
    2,
    jz4725b_mic_mode_texts
);

static jz4725b_codec_controls: [snd_kcontrol_new; 11] = [
    SOC_DOUBLE_TLV!("DAC Playback Volume", JZ4725B_CODEC_REG_CGR1, REG_CGR1_GODL_OFFSET, REG_CGR1_GODR_OFFSET, 0xf, 1, jz4725b_dac_tlv),
    SOC_DOUBLE_TLV!("Master Capture Volume", JZ4725B_CODEC_REG_CGR10, REG_CGR10_GIL_OFFSET, REG_CGR10_GIR_OFFSET, 0xf, 0, jz4725b_adc_tlv),
    SOC_DOUBLE_R_TLV!("Mixer Line In Bypass Playback Volume", JZ4725B_CODEC_REG_CGR3, JZ4725B_CODEC_REG_CGR2, REG_CGR2_GO1R_OFFSET, 0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV!("Mixer Mic 1 Bypass Playback Volume", JZ4725B_CODEC_REG_CGR5, JZ4725B_CODEC_REG_CGR4, REG_CGR4_GO2R_OFFSET, 0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV!("Mixer Mic 2 Bypass Playback Volume", JZ4725B_CODEC_REG_CGR7, JZ4725B_CODEC_REG_CGR6, REG_CGR6_GO3R_OFFSET, 0x1f, 1, jz4725b_mix_tlv),
    SOC_DOUBLE_R_TLV!("Master Playback Volume", JZ4725B_CODEC_REG_CGR9, JZ4725B_CODEC_REG_CGR8, REG_CGR8_GOR_OFFSET, 0x1f, 1, jz4725b_out_tlv),
    SOC_SINGLE!("DAC Playback Switch", JZ4725B_CODEC_REG_CR1, REG_CR1_DAC_MUTE_OFFSET, 1, 1),
    SOC_SINGLE!("Deemphasize Filter Playback Switch", JZ4725B_CODEC_REG_CR2, REG_CR2_DAC_DEEMP_OFFSET, 1, 0),
    SOC_SINGLE!("High-Pass Filter Capture Switch", JZ4725B_CODEC_REG_CR2, REG_CR2_ADC_HPF_OFFSET, 1, 0),
    SOC_ENUM!("Mic Mode Capture Switch", jz4725b_mic_mode_enum),
    SOC_SINGLE_TLV!("Mic1 Boost Capture Volume", JZ4725B_CODEC_REG_PMR2, REG_PMR2_GIM_OFFSET, 1, 0, jz4725b_mic_boost_tlv),
];

static jz4725b_codec_adc_src_texts: [*const i8; 4] = [
    c"Mic 1".as_ptr(),
    c"Mic 2".as_ptr(),
    c"Line In".as_ptr(),
    c"Mixer".as_ptr(),
];
static jz4725b_codec_adc_src_values: [u32; 4] = [0, 1, 2, 3];
static jz4725b_codec_adc_src_enum: soc_enum = SOC_VALUE_ENUM_SINGLE_DECL!(
    JZ4725B_CODEC_REG_CR3,
    REG_CR3_INSEL_OFFSET,
    REG_CR3_INSEL_MASK,
    jz4725b_codec_adc_src_texts,
    jz4725b_codec_adc_src_values
);
static jz4725b_codec_adc_src_ctrl: snd_kcontrol_new =
    SOC_DAPM_ENUM!("ADC Source Capture Route", jz4725b_codec_adc_src_enum);

static jz4725b_codec_mixer_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_SINGLE!("Line In Bypass Playback Switch", JZ4725B_CODEC_REG_CR1, REG_CR1_BYPASS_OFFSET, 1, 0),
    SOC_DAPM_SINGLE!("Mic 1 Bypass Playback Switch", JZ4725B_CODEC_REG_CR3, REG_CR3_SIDETONE1_OFFSET, 1, 0),
    SOC_DAPM_SINGLE!("Mic 2 Bypass Playback Switch", JZ4725B_CODEC_REG_CR3, REG_CR3_SIDETONE2_OFFSET, 1, 0),
];

unsafe extern "C" fn jz4725b_out_stage_enable(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let codec: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let icdc: *mut jz_icdc = snd_soc_component_get_drvdata(codec) as *mut jz_icdc;
    let map: *mut regmap = (*icdc).regmap;
    let mut val: u32 = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_clear_bits(map, JZ4725B_CODEC_REG_IFR, BIT(REG_IFR_RAMP_UP_DONE_OFFSET))
        }
        SND_SOC_DAPM_POST_PMU => regmap_read_poll_timeout!(
            map,
            JZ4725B_CODEC_REG_IFR,
            val,
            (val & BIT(REG_IFR_RAMP_UP_DONE_OFFSET)) != 0,
            100000,
            500000
        ),
        SND_SOC_DAPM_PRE_PMD => regmap_clear_bits(
            map,
            JZ4725B_CODEC_REG_IFR,
            BIT(REG_IFR_RAMP_DOWN_DONE_OFFSET),
        ),
        SND_SOC_DAPM_POST_PMD => regmap_read_poll_timeout!(
            map,
            JZ4725B_CODEC_REG_IFR,
            val,
            (val & BIT(REG_IFR_RAMP_DOWN_DONE_OFFSET)) != 0,
            100000,
            500000
        ),
        _ => -EINVAL,
    }
}

static jz4725b_codec_dapm_widgets: [snd_soc_dapm_widget; 20] = [
    /* DAC */
    SND_SOC_DAPM_DAC!("DAC", "Playback", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_DAC_OFFSET, 1),
    /* ADC */
    SND_SOC_DAPM_ADC!("ADC", "Capture", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_ADC_OFFSET, 1),
    SND_SOC_DAPM_MUX!("ADC Source Capture Route", SND_SOC_NOPM, 0, 0, &jz4725b_codec_adc_src_ctrl),
    /* Mixer */
    SND_SOC_DAPM_MIXER!("Mixer", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_MIX_OFFSET, 1, jz4725b_codec_mixer_controls.as_ptr(), jz4725b_codec_mixer_controls.len()),
    SND_SOC_DAPM_MIXER!("DAC to Mixer", JZ4725B_CODEC_REG_CR1, REG_CR1_DACSEL_OFFSET, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Line In", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_LIN_OFFSET, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("HP Out", JZ4725B_CODEC_REG_CR1, REG_CR1_HP_DIS_OFFSET, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mic 1", JZ4725B_CODEC_REG_CR3, REG_CR3_SB_MIC1_OFFSET, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Mic 2", JZ4725B_CODEC_REG_CR3, REG_CR3_SB_MIC2_OFFSET, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER_E!("Out Stage", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_OUT_OFFSET, 1, core::ptr::null(), 0, jz4725b_out_stage_enable, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MIXER!("Mixer to ADC", JZ4725B_CODEC_REG_PMR1, REG_PMR1_SB_IND_OFFSET, 1, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("Mic Bias", JZ4725B_CODEC_REG_CR1, REG_CR1_SB_MICBIAS_OFFSET, 1, core::ptr::null(), 0),
    /* Pins */
    SND_SOC_DAPM_INPUT!("MIC1P"),
    SND_SOC_DAPM_INPUT!("MIC1N"),
    SND_SOC_DAPM_INPUT!("MIC2P"),
    SND_SOC_DAPM_INPUT!("MIC2N"),
    SND_SOC_DAPM_INPUT!("LLINEIN"),
    SND_SOC_DAPM_INPUT!("RLINEIN"),
    SND_SOC_DAPM_OUTPUT!("LHPOUT"),
    SND_SOC_DAPM_OUTPUT!("RHPOUT"),
];

static jz4725b_codec_dapm_routes: [snd_soc_dapm_route; 21] = [
    snd_soc_dapm_route { sink: c"Mic 1".as_ptr(), control: core::ptr::null(), source: c"MIC1P".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 1".as_ptr(), control: core::ptr::null(), source: c"MIC1N".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 2".as_ptr(), control: core::ptr::null(), source: c"MIC2P".as_ptr() },
    snd_soc_dapm_route { sink: c"Mic 2".as_ptr(), control: core::ptr::null(), source: c"MIC2N".as_ptr() },
    snd_soc_dapm_route { sink: c"Line In".as_ptr(), control: core::ptr::null(), source: c"LLINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Line In".as_ptr(), control: core::ptr::null(), source: c"RLINEIN".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer".as_ptr(), control: c"Mic 1 Bypass Playback Switch".as_ptr(), source: c"Mic 1".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer".as_ptr(), control: c"Mic 2 Bypass Playback Switch".as_ptr(), source: c"Mic 2".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer".as_ptr(), control: c"Line In Bypass Playback Switch".as_ptr(), source: c"Line In".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC to Mixer".as_ptr(), control: core::ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer".as_ptr(), control: core::ptr::null(), source: c"DAC to Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"Mixer to ADC".as_ptr(), control: core::ptr::null(), source: c"Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Source Capture Route".as_ptr(), control: c"Mixer".as_ptr(), source: c"Mixer to ADC".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Source Capture Route".as_ptr(), control: c"Line In".as_ptr(), source: c"Line In".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Source Capture Route".as_ptr(), control: c"Mic 1".as_ptr(), source: c"Mic 1".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC Source Capture Route".as_ptr(), control: c"Mic 2".as_ptr(), source: c"Mic 2".as_ptr() },
    snd_soc_dapm_route { sink: c"ADC".as_ptr(), control: core::ptr::null(), source: c"ADC Source Capture Route".as_ptr() },
    snd_soc_dapm_route { sink: c"Out Stage".as_ptr(), control: core::ptr::null(), source: c"Mixer".as_ptr() },
    snd_soc_dapm_route { sink: c"HP Out".as_ptr(), control: core::ptr::null(), source: c"Out Stage".as_ptr() },
    snd_soc_dapm_route { sink: c"LHPOUT".as_ptr(), control: core::ptr::null(), source: c"HP Out".as_ptr() },
    snd_soc_dapm_route { sink: c"RHPOUT".as_ptr(), control: core::ptr::null(), source: c"HP Out".as_ptr() },
];

unsafe extern "C" fn jz4725b_codec_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> i32 {
    let icdc: *mut jz_icdc = snd_soc_component_get_drvdata(component) as *mut jz_icdc;
    let map: *mut regmap = (*icdc).regmap;

    match level {
        SND_SOC_BIAS_ON => {
            regmap_clear_bits(map, JZ4725B_CODEC_REG_PMR2, BIT(REG_PMR2_SB_SLEEP_OFFSET));
        }
        SND_SOC_BIAS_PREPARE => {
            /* Enable sound hardware */
            regmap_clear_bits(map, JZ4725B_CODEC_REG_PMR2, BIT(REG_PMR2_SB_OFFSET));
            msleep(224);
        }
        SND_SOC_BIAS_STANDBY => {
            regmap_set_bits(map, JZ4725B_CODEC_REG_PMR2, BIT(REG_PMR2_SB_SLEEP_OFFSET));
        }
        SND_SOC_BIAS_OFF => {
            regmap_set_bits(map, JZ4725B_CODEC_REG_PMR2, BIT(REG_PMR2_SB_OFFSET));
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn jz4725b_codec_dev_probe(component: *mut snd_soc_component) -> i32 {
    let icdc: *mut jz_icdc = snd_soc_component_get_drvdata(component) as *mut jz_icdc;
    let map: *mut regmap = (*icdc).regmap;

    /* Write CONFIGn (n=1 to 8) bits.
     * The value 0x0f is specified in the datasheet as a requirement.
     */
    regmap_write(map, JZ4725B_CODEC_REG_AICR, 0xf << REG_AICR_CONFIG1_OFFSET);
    regmap_write(map, JZ4725B_CODEC_REG_CCR1, 0x0 << REG_CCR1_CONFIG4_OFFSET);

    0
}

static jz4725b_codec: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(jz4725b_codec_dev_probe),
    set_bias_level: Some(jz4725b_codec_set_bias_level),
    controls: jz4725b_codec_controls.as_ptr(),
    num_controls: jz4725b_codec_controls.len() as u32,
    dapm_widgets: jz4725b_codec_dapm_widgets.as_ptr(),
    num_dapm_widgets: jz4725b_codec_dapm_widgets.len() as u32,
    dapm_routes: jz4725b_codec_dapm_routes.as_ptr(),
    num_dapm_routes: jz4725b_codec_dapm_routes.len() as u32,
    suspend_bias_off: 1,
    use_pmdown_time: 1,
    ..unsafe { core::mem::zeroed() }
};

static jz4725b_codec_sample_rates: [u32; 11] = [
    96000, 48000, 44100, 32000,
    24000, 22050, 16000, 12000,
    11025, 9600, 8000,
];

unsafe extern "C" fn jz4725b_codec_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let icdc: *mut jz_icdc = snd_soc_component_get_drvdata((*dai).component) as *mut jz_icdc;
    let bit_width: u32;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => bit_width = 0,
        SNDRV_PCM_FORMAT_S18_3LE => bit_width = 1,
        SNDRV_PCM_FORMAT_S20_3LE => bit_width = 2,
        SNDRV_PCM_FORMAT_S24_3LE => bit_width = 3,
        _ => return -EINVAL,
    }

    let mut rate: usize = 0;
    while rate < jz4725b_codec_sample_rates.len() {
        if jz4725b_codec_sample_rates[rate] == params_rate(params) {
            break;
        }
        rate += 1;
    }

    if rate == jz4725b_codec_sample_rates.len() {
        return -EINVAL;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            (*icdc).regmap,
            JZ4725B_CODEC_REG_CR2,
            REG_CR2_DAC_ADWL_MASK,
            bit_width << REG_CR2_DAC_ADWL_OFFSET,
        );

        regmap_update_bits(
            (*icdc).regmap,
            JZ4725B_CODEC_REG_CCR2,
            REG_CCR2_DFREQ_MASK,
            (rate as u32) << REG_CCR2_DFREQ_OFFSET,
        );
    } else {
        regmap_update_bits(
            (*icdc).regmap,
            JZ4725B_CODEC_REG_CR2,
            REG_CR2_ADC_ADWL_MASK,
            bit_width << REG_CR2_ADC_ADWL_OFFSET,
        );

        regmap_update_bits(
            (*icdc).regmap,
            JZ4725B_CODEC_REG_CCR2,
            REG_CCR2_AFREQ_MASK,
            (rate as u32) << REG_CCR2_AFREQ_OFFSET,
        );
    }

    0
}

static jz4725b_codec_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(jz4725b_codec_hw_params),
    ..unsafe { core::mem::zeroed() }
};

const JZ_ICDC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE;

static mut jz4725b_codec_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"jz4725b-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_ICDC_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_96000,
        formats: JZ_ICDC_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &jz4725b_codec_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn jz4725b_codec_volatile(_dev: *mut device, reg: u32) -> bool {
    reg == JZ4725B_CODEC_REG_IFR
}

unsafe extern "C" fn jz4725b_codec_can_access_reg(_dev: *mut device, reg: u32) -> bool {
    (reg != JZ4725B_CODEC_REG_TR1) && (reg != JZ4725B_CODEC_REG_TR2)
}

unsafe fn jz4725b_codec_io_wait(icdc: *mut jz_icdc) -> i32 {
    let mut reg: u32 = 0;

    readl_poll_timeout!(
        ((*icdc).base as *mut u8).add(ICDC_RGADW_OFFSET),
        reg,
        (reg & ICDC_RGADW_RGWR) == 0,
        1000,
        10000
    )
}

unsafe extern "C" fn jz4725b_codec_reg_read(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: *mut u32,
) -> i32 {
    let icdc: *mut jz_icdc = context as *mut jz_icdc;
    let mut i: u32;
    let mut tmp: u32;
    let mut ret: i32;

    ret = jz4725b_codec_io_wait(icdc);
    if ret != 0 {
        return ret;
    }

    tmp = readl(((*icdc).base as *mut u8).add(ICDC_RGADW_OFFSET));
    tmp = (tmp & !ICDC_RGADW_RGADDR_MASK) | (reg << ICDC_RGADW_RGADDR_OFFSET);
    writel(tmp, ((*icdc).base as *mut u8).add(ICDC_RGADW_OFFSET));

    /* wait 6+ cycles */
    i = 0;
    while i < 6 {
        *val = readl(((*icdc).base as *mut u8).add(ICDC_RGDATA_OFFSET)) & ICDC_RGDATA_RGDOUT_MASK;
        i += 1;
    }

    0
}

unsafe extern "C" fn jz4725b_codec_reg_write(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: u32,
) -> i32 {
    let icdc: *mut jz_icdc = context as *mut jz_icdc;
    let mut ret: i32;

    ret = jz4725b_codec_io_wait(icdc);
    if ret != 0 {
        return ret;
    }

    writel(
        ICDC_RGADW_RGWR | (reg << ICDC_RGADW_RGADDR_OFFSET) | val,
        ((*icdc).base as *mut u8).add(ICDC_RGADW_OFFSET),
    );

    ret = jz4725b_codec_io_wait(icdc);
    if ret != 0 {
        return ret;
    }

    0
}

static jz4725b_codec_reg_defaults: [u8; 28] = [
    0x0c, 0xaa, 0x78, 0x00, 0x00, 0xff, 0x03, 0x51,
    0x3f, 0x00, 0x00, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x0a, 0x0a, 0x00, 0x00, 0x00, 0xc0, 0x34,
    0x07, 0x44, 0x1f, 0x00,
];

static jz4725b_codec_regmap_config: regmap_config = regmap_config {
    reg_bits: 7,
    val_bits: 8,
    max_register: JZ4725B_CODEC_REG_AGC5,
    volatile_reg: Some(jz4725b_codec_volatile),
    readable_reg: Some(jz4725b_codec_can_access_reg),
    writeable_reg: Some(jz4725b_codec_can_access_reg),
    reg_read: Some(jz4725b_codec_reg_read),
    reg_write: Some(jz4725b_codec_reg_write),
    reg_defaults_raw: jz4725b_codec_reg_defaults.as_ptr() as *const core::ffi::c_void,
    num_reg_defaults_raw: jz4725b_codec_reg_defaults.len() as u32,
    cache_type: REGCACHE_FLAT,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn jz4725b_codec_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let mut icdc: *mut jz_icdc;
    let mut clk: *mut clk;
    let mut ret: i32;

    icdc = devm_kzalloc(dev, core::mem::size_of::<jz_icdc>(), GFP_KERNEL) as *mut jz_icdc;
    if icdc.is_null() {
        return -ENOMEM;
    }

    (*icdc).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*icdc).base) {
        return PTR_ERR((*icdc).base);
    }

    (*icdc).regmap = devm_regmap_init(
        dev,
        core::ptr::null(),
        icdc as *mut core::ffi::c_void,
        &jz4725b_codec_regmap_config,
    );
    if IS_ERR((*icdc).regmap as *mut core::ffi::c_void) {
        return PTR_ERR((*icdc).regmap as *mut core::ffi::c_void);
    }

    clk = devm_clk_get_enabled(dev, c"aic".as_ptr());
    if IS_ERR(clk as *mut core::ffi::c_void) {
        return PTR_ERR(clk as *mut core::ffi::c_void);
    }

    platform_set_drvdata(pdev, icdc as *mut core::ffi::c_void);

    ret = devm_snd_soc_register_component(
        dev,
        &jz4725b_codec,
        &mut jz4725b_codec_dai,
        1,
    );
    if ret != 0 {
        dev_err(dev, c"Failed to register codec\n".as_ptr());
    }

    ret
}

static jz4725b_codec_of_matches: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ingenic,jz4725b-codec".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE!(of, jz4725b_codec_of_matches);

static mut jz4725b_codec_driver: platform_driver = platform_driver {
    probe: Some(jz4725b_codec_probe),
    driver: device_driver {
        name: c"jz4725b-codec".as_ptr(),
        of_match_table: jz4725b_codec_of_matches.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};
module_platform_driver!(jz4725b_codec_driver);

MODULE_DESCRIPTION!("JZ4725B SoC internal codec driver");
MODULE_AUTHOR!("Paul Cercueil <paul@crapouillou.net>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
