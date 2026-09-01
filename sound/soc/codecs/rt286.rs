// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt286.c  --  RT286 ALSA SoC audio codec driver
 *
 * Copyright 2013 Realtek Semiconductor Corp.
 * Author: Bard Liao <bardliao@realtek.com>
 */

// Translated from C. Kernel/ALSA include dependencies are expected to provide
// the referenced opaque types, constants, functions, and construction macros.

pub const RT286_VENDOR_ID: u32 = 0x10ec0286;
pub const RT288_VENDOR_ID: u32 = 0x10ec0288;

#[repr(C)]
pub struct rt286_priv {
    pub index_cache: *mut reg_default,
    pub index_cache_size: i32,
    pub regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub pdata: rt286_platform_data,
    pub i2c: *mut i2c_client,
    pub jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub sys_clk: i32,
    pub clk_id: i32,
}

pub static rt286_index_def: [reg_default; 20] = [
    reg_default { reg: 0x01, def: 0xaaaa },
    reg_default { reg: 0x02, def: 0x8aaa },
    reg_default { reg: 0x03, def: 0x0002 },
    reg_default { reg: 0x04, def: 0xaf01 },
    reg_default { reg: 0x08, def: 0x000d },
    reg_default { reg: 0x09, def: 0xd810 },
    reg_default { reg: 0x0a, def: 0x0120 },
    reg_default { reg: 0x0b, def: 0x0000 },
    reg_default { reg: 0x0d, def: 0x2800 },
    reg_default { reg: 0x0f, def: 0x0000 },
    reg_default { reg: 0x19, def: 0x0a17 },
    reg_default { reg: 0x20, def: 0x0020 },
    reg_default { reg: 0x33, def: 0x0208 },
    reg_default { reg: 0x49, def: 0x0004 },
    reg_default { reg: 0x4f, def: 0x50e9 },
    reg_default { reg: 0x50, def: 0x2000 },
    reg_default { reg: 0x63, def: 0x2902 },
    reg_default { reg: 0x67, def: 0x1111 },
    reg_default { reg: 0x68, def: 0x1016 },
    reg_default { reg: 0x69, def: 0x273f },
];
pub const INDEX_CACHE_SIZE: usize = rt286_index_def.len();

pub static rt286_reg: [reg_default; 39] = [
    reg_default { reg: 0x00170500, def: 0x00000400 },
    reg_default { reg: 0x00220000, def: 0x00000031 },
    reg_default { reg: 0x00239000, def: 0x0000007f },
    reg_default { reg: 0x0023a000, def: 0x0000007f },
    reg_default { reg: 0x00270500, def: 0x00000400 },
    reg_default { reg: 0x00370500, def: 0x00000400 },
    reg_default { reg: 0x00830000, def: 0x000000c3 },
    reg_default { reg: 0x00870500, def: 0x00000400 },
    reg_default { reg: 0x00920000, def: 0x00000031 },
    reg_default { reg: 0x00930000, def: 0x000000c3 },
    reg_default { reg: 0x00935000, def: 0x000000c3 },
    reg_default { reg: 0x00936000, def: 0x000000c3 },
    reg_default { reg: 0x00970500, def: 0x00000400 },
    reg_default { reg: 0x00b37000, def: 0x00000097 },
    reg_default { reg: 0x00b37200, def: 0x00000097 },
    reg_default { reg: 0x00b37300, def: 0x00000097 },
    reg_default { reg: 0x00c37000, def: 0x00000000 },
    reg_default { reg: 0x00c37100, def: 0x00000080 },
    reg_default { reg: 0x01270500, def: 0x00000400 },
    reg_default { reg: 0x01270700, def: 0x00000000 },
    reg_default { reg: 0x01370500, def: 0x00000400 },
    reg_default { reg: 0x01371f00, def: 0x411111f0 },
    reg_default { reg: 0x01439000, def: 0x00000080 },
    reg_default { reg: 0x0143a000, def: 0x00000080 },
    reg_default { reg: 0x01470100, def: 0x00000000 },
    reg_default { reg: 0x01470500, def: 0x00000400 },
    reg_default { reg: 0x01470700, def: 0x00000000 },
    reg_default { reg: 0x01470c00, def: 0x00000000 },
    reg_default { reg: 0x01837000, def: 0x00000000 },
    reg_default { reg: 0x01870500, def: 0x00000400 },
    reg_default { reg: 0x01870700, def: 0x00000020 },
    reg_default { reg: 0x02050000, def: 0x00000000 },
    reg_default { reg: 0x02139000, def: 0x00000080 },
    reg_default { reg: 0x0213a000, def: 0x00000080 },
    reg_default { reg: 0x02170100, def: 0x00000000 },
    reg_default { reg: 0x02170500, def: 0x00000400 },
    reg_default { reg: 0x02170700, def: 0x00000000 },
    reg_default { reg: 0x02270100, def: 0x00000000 },
    reg_default { reg: 0x02370100, def: 0x00000000 },
];

pub unsafe fn rt286_volatile_register(_dev: *mut device, reg: u32) -> bool {
    match reg {
        0x00..=0xff => true,
        x if x == RT286_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID) => true,
        x if x == RT286_GET_HP_SENSE => true,
        x if x == RT286_GET_MIC1_SENSE => true,
        x if x == RT286_PROC_COEF => true,
        _ => false,
    }
}

pub unsafe fn rt286_readable_register(_dev: *mut device, reg: u32) -> bool {
    match reg {
        0x00..=0xff => true,
        x if x == RT286_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID) => true,
        x if x == RT286_GET_HP_SENSE => true,
        x if x == RT286_GET_MIC1_SENSE => true,
        x if x == RT286_SET_AUDIO_POWER => true,
        x if x == RT286_SET_HPO_POWER => true,
        x if x == RT286_SET_SPK_POWER => true,
        x if x == RT286_SET_DMIC1_POWER => true,
        x if x == RT286_SPK_MUX => true,
        x if x == RT286_HPO_MUX => true,
        x if x == RT286_ADC0_MUX => true,
        x if x == RT286_ADC1_MUX => true,
        x if x == RT286_SET_MIC1 => true,
        x if x == RT286_SET_PIN_HPO => true,
        x if x == RT286_SET_PIN_SPK => true,
        x if x == RT286_SET_PIN_DMIC1 => true,
        x if x == RT286_SPK_EAPD => true,
        x if x == RT286_SET_AMP_GAIN_HPO => true,
        x if x == RT286_SET_DMIC2_DEFAULT => true,
        x if x == RT286_DACL_GAIN => true,
        x if x == RT286_DACR_GAIN => true,
        x if x == RT286_ADCL_GAIN => true,
        x if x == RT286_ADCR_GAIN => true,
        x if x == RT286_MIC_GAIN => true,
        x if x == RT286_SPOL_GAIN => true,
        x if x == RT286_SPOR_GAIN => true,
        x if x == RT286_HPOL_GAIN => true,
        x if x == RT286_HPOR_GAIN => true,
        x if x == RT286_F_DAC_SWITCH => true,
        x if x == RT286_F_RECMIX_SWITCH => true,
        x if x == RT286_REC_MIC_SWITCH => true,
        x if x == RT286_REC_I2S_SWITCH => true,
        x if x == RT286_REC_LINE_SWITCH => true,
        x if x == RT286_REC_BEEP_SWITCH => true,
        x if x == RT286_DAC_FORMAT => true,
        x if x == RT286_ADC_FORMAT => true,
        x if x == RT286_COEF_INDEX => true,
        x if x == RT286_PROC_COEF => true,
        x if x == RT286_SET_AMP_GAIN_ADC_IN1 => true,
        x if x == RT286_SET_AMP_GAIN_ADC_IN2 => true,
        x if x == RT286_SET_GPIO_MASK => true,
        x if x == RT286_SET_GPIO_DIRECTION => true,
        x if x == RT286_SET_GPIO_DATA => true,
        x if x == RT286_SET_POWER(RT286_DAC_OUT1) => true,
        x if x == RT286_SET_POWER(RT286_DAC_OUT2) => true,
        x if x == RT286_SET_POWER(RT286_ADC_IN1) => true,
        x if x == RT286_SET_POWER(RT286_ADC_IN2) => true,
        x if x == RT286_SET_POWER(RT286_DMIC2) => true,
        x if x == RT286_SET_POWER(RT286_MIC1) => true,
        _ => false,
    }
}

// CONFIG_PM
pub unsafe fn rt286_index_sync(component: *mut snd_soc_component) {
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;
    let mut i = 0;
    while i < INDEX_CACHE_SIZE {
        snd_soc_component_write(
            component,
            (*(*rt286).index_cache.add(i)).reg,
            (*(*rt286).index_cache.add(i)).def,
        );
        i += 1;
    }
}

pub static mut rt286_support_power_controls: [i32; 9] = [
    RT286_DAC_OUT1,
    RT286_DAC_OUT2,
    RT286_ADC_IN1,
    RT286_ADC_IN2,
    RT286_MIC1,
    RT286_DMIC1,
    RT286_DMIC2,
    RT286_SPK_OUT,
    RT286_HP_OUT,
];
pub const RT286_POWER_REG_LEN: usize = 9;

pub unsafe fn rt286_jack_detect(rt286: *mut rt286_priv, hp: *mut bool, mic: *mut bool) -> i32 {
    let dapm: *mut snd_soc_dapm_context;
    let mut val: u32 = 0;
    let mut buf: u32 = 0;

    *hp = false;
    *mic = false;

    if (*rt286).component.is_null() {
        return -EINVAL;
    }

    dapm = snd_soc_component_to_dapm((*rt286).component);

    if (*rt286).pdata.cbj_en {
        regmap_read((*rt286).regmap, RT286_GET_HP_SENSE, &mut buf);
        *hp = (buf & 0x80000000) != 0;
        if *hp {
            /* power on HV,VERF */
            regmap_update_bits((*rt286).regmap, RT286_DC_GAIN, 0x200, 0x200);

            snd_soc_dapm_force_enable_pin(dapm, c"HV".as_ptr());
            snd_soc_dapm_force_enable_pin(dapm, c"VREF".as_ptr());
            /* power LDO1 */
            snd_soc_dapm_force_enable_pin(dapm, c"LDO1".as_ptr());
            snd_soc_dapm_sync(dapm);

            regmap_write((*rt286).regmap, RT286_SET_MIC1, 0x24);
            msleep(50);

            regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0xfcc0, 0xd400);
            msleep(300);
            regmap_read((*rt286).regmap, RT286_CBJ_CTRL2, &mut val);

            if 0x0070 == (val & 0x0070) {
                *mic = true;
            } else {
                regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0xfcc0, 0xe400);
                msleep(300);
                regmap_read((*rt286).regmap, RT286_CBJ_CTRL2, &mut val);
                if 0x0070 == (val & 0x0070) {
                    *mic = true;
                } else {
                    *mic = false;
                    regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0xfcc0, 0xc400);
                }
            }

            regmap_update_bits((*rt286).regmap, RT286_DC_GAIN, 0x200, 0x0);
        } else {
            *mic = false;
            regmap_write((*rt286).regmap, RT286_SET_MIC1, 0x20);
            regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0x0400, 0x0000);
        }
    } else {
        regmap_read((*rt286).regmap, RT286_GET_HP_SENSE, &mut buf);
        *hp = (buf & 0x80000000) != 0;
        regmap_read((*rt286).regmap, RT286_GET_MIC1_SENSE, &mut buf);
        *mic = (buf & 0x80000000) != 0;
    }

    if !*hp {
        snd_soc_dapm_disable_pin(dapm, c"HV".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"VREF".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"LDO1".as_ptr());
        snd_soc_dapm_sync(dapm);
    }

    0
}

pub unsafe fn rt286_jack_detect_work(work: *mut work_struct) {
    let rt286 = container_of!(work, rt286_priv, jack_detect_work.work);
    let mut status: i32 = 0;
    let mut hp = false;
    let mut mic = false;

    rt286_jack_detect(rt286, &mut hp, &mut mic);

    if hp {
        status |= SND_JACK_HEADPHONE;
    }
    if mic {
        status |= SND_JACK_MICROPHONE;
    }

    snd_soc_jack_report(
        (*rt286).jack,
        status,
        SND_JACK_MICROPHONE | SND_JACK_HEADPHONE,
    );
}

pub unsafe fn rt286_mic_detect(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _data: *mut core::ffi::c_void,
) -> i32 {
    let dapm = snd_soc_component_to_dapm(component);
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    (*rt286).jack = jack;

    if !jack.is_null() {
        /* enable IRQ */
        if ((*(*rt286).jack).status & SND_JACK_HEADPHONE) != 0 {
            snd_soc_dapm_force_enable_pin(dapm, c"LDO1".as_ptr());
        }
        regmap_update_bits((*rt286).regmap, RT286_IRQ_CTRL, 0x2, 0x2);
        /* Send an initial empty report */
        snd_soc_jack_report(
            (*rt286).jack,
            (*(*rt286).jack).status,
            SND_JACK_MICROPHONE | SND_JACK_HEADPHONE,
        );
    } else {
        /* disable IRQ */
        regmap_update_bits((*rt286).regmap, RT286_IRQ_CTRL, 0x2, 0x0);
        snd_soc_dapm_disable_pin(dapm, c"LDO1".as_ptr());
    }
    snd_soc_dapm_sync(dapm);

    0
}

pub unsafe fn is_mclk_mode(source: *mut snd_soc_dapm_widget, _sink: *mut snd_soc_dapm_widget) -> i32 {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    if (*rt286).clk_id == RT286_SCLK_S_MCLK {
        1
    } else {
        0
    }
}

DECLARE_TLV_DB_SCALE!(out_vol_tlv, -6350, 50, 0);
DECLARE_TLV_DB_SCALE!(mic_vol_tlv, 0, 1000, 0);

pub static rt286_snd_controls: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_R_TLV!("DAC0 Playback Volume", RT286_DACL_GAIN, RT286_DACR_GAIN, 0, 0x7f, 0, out_vol_tlv),
    SOC_DOUBLE_R!("ADC0 Capture Switch", RT286_ADCL_GAIN, RT286_ADCR_GAIN, 7, 1, 1),
    SOC_DOUBLE_R_TLV!("ADC0 Capture Volume", RT286_ADCL_GAIN, RT286_ADCR_GAIN, 0, 0x7f, 0, out_vol_tlv),
    SOC_SINGLE_TLV!("AMIC Volume", RT286_MIC_GAIN, 0, 0x3, 0, mic_vol_tlv),
    SOC_DOUBLE_R!("Speaker Playback Switch", RT286_SPOL_GAIN, RT286_SPOR_GAIN, RT286_MUTE_SFT, 1, 1),
];

/* Digital Mixer */
pub static rt286_front_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DAC Switch", RT286_F_DAC_SWITCH, RT286_MUTE_SFT, 1, 1),
    SOC_DAPM_SINGLE!("RECMIX Switch", RT286_F_RECMIX_SWITCH, RT286_MUTE_SFT, 1, 1),
];

/* Analog Input Mixer */
pub static rt286_rec_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("Mic1 Switch", RT286_REC_MIC_SWITCH, RT286_MUTE_SFT, 1, 1),
    SOC_DAPM_SINGLE!("I2S Switch", RT286_REC_I2S_SWITCH, RT286_MUTE_SFT, 1, 1),
    SOC_DAPM_SINGLE!("Line1 Switch", RT286_REC_LINE_SWITCH, RT286_MUTE_SFT, 1, 1),
    SOC_DAPM_SINGLE!("Beep Switch", RT286_REC_BEEP_SWITCH, RT286_MUTE_SFT, 1, 1),
];

pub static spo_enable_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE!("Switch", RT286_SET_PIN_SPK, RT286_SET_PIN_SFT, 1, 0);
pub static hpol_enable_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", RT286_HPOL_GAIN, RT286_MUTE_SFT, 1, 1);
pub static hpor_enable_control: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", RT286_HPOR_GAIN, RT286_MUTE_SFT, 1, 1);

/* ADC0 source */
pub static rt286_adc_src: [&CStr; 3] = [c"Mic", c"RECMIX", c"Dmic"];
pub static rt286_adc_values: [i32; 3] = [0, 4, 5];

SOC_VALUE_ENUM_SINGLE_DECL!(
    rt286_adc0_enum,
    RT286_ADC0_MUX,
    RT286_ADC_SEL_SFT,
    RT286_ADC_SEL_MASK,
    rt286_adc_src,
    rt286_adc_values
);
pub static rt286_adc0_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC 0 source", rt286_adc0_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    rt286_adc1_enum,
    RT286_ADC1_MUX,
    RT286_ADC_SEL_SFT,
    RT286_ADC_SEL_MASK,
    rt286_adc_src,
    rt286_adc_values
);
pub static rt286_adc1_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("ADC 1 source", rt286_adc1_enum);

pub static rt286_dac_src: [&CStr; 2] = [c"Front", c"Surround"];
/* HP-OUT source */
SOC_ENUM_SINGLE_DECL!(rt286_hpo_enum, RT286_HPO_MUX, 0, rt286_dac_src);
pub static rt286_hpo_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("HPO source", rt286_hpo_enum);

/* SPK-OUT source */
SOC_ENUM_SINGLE_DECL!(rt286_spo_enum, RT286_SPK_MUX, 0, rt286_dac_src);
pub static rt286_spo_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("SPO source", rt286_spo_enum);

pub unsafe fn rt286_spk_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => {
            snd_soc_component_write(component, RT286_SPK_EAPD, RT286_SET_EAPD_HIGH);
        }
        SND_SOC_DAPM_PRE_PMD => {
            snd_soc_component_write(component, RT286_SPK_EAPD, RT286_SET_EAPD_LOW);
        }
        _ => return 0,
    }

    0
}

pub unsafe fn rt286_set_dmic1_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => snd_soc_component_write(component, RT286_SET_PIN_DMIC1, 0x20),
        SND_SOC_DAPM_PRE_PMD => snd_soc_component_write(component, RT286_SET_PIN_DMIC1, 0),
        _ => return 0,
    }

    0
}

pub unsafe fn rt286_ldo2_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_POST_PMU => snd_soc_component_update_bits(component, RT286_POWER_CTRL2, 0x38, 0x08),
        SND_SOC_DAPM_PRE_PMD => snd_soc_component_update_bits(component, RT286_POWER_CTRL2, 0x38, 0x30),
        _ => return 0,
    }

    0
}

pub unsafe fn rt286_mic1_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            snd_soc_component_update_bits(component, RT286_A_BIAS_CTRL3, 0xc000, 0x8000);
            snd_soc_component_update_bits(component, RT286_A_BIAS_CTRL2, 0xc000, 0x8000);
        }
        SND_SOC_DAPM_POST_PMD => {
            snd_soc_component_update_bits(component, RT286_A_BIAS_CTRL3, 0xc000, 0x0000);
            snd_soc_component_update_bits(component, RT286_A_BIAS_CTRL2, 0xc000, 0x0000);
        }
        _ => return 0,
    }

    0
}

pub static rt286_dapm_widgets: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_SUPPLY_S!("HV", 1, RT286_POWER_CTRL1, 12, 1, None, 0),
    SND_SOC_DAPM_SUPPLY!("VREF", RT286_POWER_CTRL1, 0, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("LDO1", 1, RT286_POWER_CTRL2, 2, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("LDO2", 2, RT286_POWER_CTRL1, 13, 1, rt286_ldo2_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SUPPLY!("MCLK MODE", RT286_PLL_CTRL1, 5, 0, None, 0),
    SND_SOC_DAPM_SUPPLY!("MIC1 Input Buffer", SND_SOC_NOPM, 0, 0, rt286_mic1_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT!("DMIC1 Pin"),
    SND_SOC_DAPM_INPUT!("DMIC2 Pin"),
    SND_SOC_DAPM_INPUT!("MIC1"),
    SND_SOC_DAPM_INPUT!("LINE1"),
    SND_SOC_DAPM_INPUT!("Beep"),
    SND_SOC_DAPM_PGA_E!("DMIC1", RT286_SET_POWER(RT286_DMIC1), 0, 1, None, 0, rt286_set_dmic1_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_PGA!("DMIC2", RT286_SET_POWER(RT286_DMIC2), 0, 1, None, 0),
    SND_SOC_DAPM_SUPPLY!("DMIC Receiver", SND_SOC_NOPM, 0, 0, None, 0),
    SND_SOC_DAPM_MIXER!("RECMIX", SND_SOC_NOPM, 0, 0, rt286_rec_mix, rt286_rec_mix.len()),
    SND_SOC_DAPM_ADC!("ADC 0", None, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_ADC!("ADC 1", None, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("ADC 0 Mux", RT286_SET_POWER(RT286_ADC_IN1), 0, 1, &rt286_adc0_mux),
    SND_SOC_DAPM_MUX!("ADC 1 Mux", RT286_SET_POWER(RT286_ADC_IN2), 0, 1, &rt286_adc1_mux),
    SND_SOC_DAPM_AIF_IN!("AIF1RX", "AIF1 Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF1TX", "AIF1 Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("AIF2RX", "AIF2 Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("AIF2TX", "AIF2 Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DAC 0", None, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("DAC 1", None, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("SPK Mux", SND_SOC_NOPM, 0, 0, &rt286_spo_mux),
    SND_SOC_DAPM_MUX!("HPO Mux", SND_SOC_NOPM, 0, 0, &rt286_hpo_mux),
    SND_SOC_DAPM_SUPPLY!("HP Power", RT286_SET_PIN_HPO, RT286_SET_PIN_SFT, 0, None, 0),
    SND_SOC_DAPM_MIXER!("Front", RT286_SET_POWER(RT286_DAC_OUT1), 0, 1, rt286_front_mix, rt286_front_mix.len()),
    SND_SOC_DAPM_PGA!("Surround", RT286_SET_POWER(RT286_DAC_OUT2), 0, 1, None, 0),
    SND_SOC_DAPM_SWITCH_E!("SPO", SND_SOC_NOPM, 0, 0, &spo_enable_control, rt286_spk_event, SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU),
    SND_SOC_DAPM_SWITCH!("HPO L", SND_SOC_NOPM, 0, 0, &hpol_enable_control),
    SND_SOC_DAPM_SWITCH!("HPO R", SND_SOC_NOPM, 0, 0, &hpor_enable_control),
    SND_SOC_DAPM_OUTPUT!("SPOL"),
    SND_SOC_DAPM_OUTPUT!("SPOR"),
    SND_SOC_DAPM_OUTPUT!("HPO Pin"),
    SND_SOC_DAPM_OUTPUT!("SPDIF"),
];

pub static rt286_dapm_routes: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c"ADC 0".as_ptr(), control: core::ptr::null(), source: c"MCLK MODE".as_ptr(), connected: Some(is_mclk_mode) },
    snd_soc_dapm_route { sink: c"ADC 1".as_ptr(), control: core::ptr::null(), source: c"MCLK MODE".as_ptr(), connected: Some(is_mclk_mode) },
    snd_soc_dapm_route { sink: c"Front".as_ptr(), control: core::ptr::null(), source: c"MCLK MODE".as_ptr(), connected: Some(is_mclk_mode) },
    snd_soc_dapm_route { sink: c"Surround".as_ptr(), control: core::ptr::null(), source: c"MCLK MODE".as_ptr(), connected: Some(is_mclk_mode) },
    SND_SOC_DAPM_ROUTE!("HP Power", None, "LDO1"),
    SND_SOC_DAPM_ROUTE!("HP Power", None, "LDO2"),
    SND_SOC_DAPM_ROUTE!("MIC1", None, "LDO1"),
    SND_SOC_DAPM_ROUTE!("MIC1", None, "LDO2"),
    SND_SOC_DAPM_ROUTE!("MIC1", None, "HV"),
    SND_SOC_DAPM_ROUTE!("MIC1", None, "VREF"),
    SND_SOC_DAPM_ROUTE!("MIC1", None, "MIC1 Input Buffer"),
    SND_SOC_DAPM_ROUTE!("SPO", None, "LDO1"),
    SND_SOC_DAPM_ROUTE!("SPO", None, "LDO2"),
    SND_SOC_DAPM_ROUTE!("SPO", None, "HV"),
    SND_SOC_DAPM_ROUTE!("SPO", None, "VREF"),
    SND_SOC_DAPM_ROUTE!("DMIC1", None, "DMIC1 Pin"),
    SND_SOC_DAPM_ROUTE!("DMIC2", None, "DMIC2 Pin"),
    SND_SOC_DAPM_ROUTE!("DMIC1", None, "DMIC Receiver"),
    SND_SOC_DAPM_ROUTE!("DMIC2", None, "DMIC Receiver"),
    SND_SOC_DAPM_ROUTE!("RECMIX", Some("Beep Switch"), "Beep"),
    SND_SOC_DAPM_ROUTE!("RECMIX", Some("Line1 Switch"), "LINE1"),
    SND_SOC_DAPM_ROUTE!("RECMIX", Some("Mic1 Switch"), "MIC1"),
    SND_SOC_DAPM_ROUTE!("ADC 0 Mux", Some("Dmic"), "DMIC1"),
    SND_SOC_DAPM_ROUTE!("ADC 0 Mux", Some("RECMIX"), "RECMIX"),
    SND_SOC_DAPM_ROUTE!("ADC 0 Mux", Some("Mic"), "MIC1"),
    SND_SOC_DAPM_ROUTE!("ADC 1 Mux", Some("Dmic"), "DMIC2"),
    SND_SOC_DAPM_ROUTE!("ADC 1 Mux", Some("RECMIX"), "RECMIX"),
    SND_SOC_DAPM_ROUTE!("ADC 1 Mux", Some("Mic"), "MIC1"),
    SND_SOC_DAPM_ROUTE!("ADC 0", None, "ADC 0 Mux"),
    SND_SOC_DAPM_ROUTE!("ADC 1", None, "ADC 1 Mux"),
    SND_SOC_DAPM_ROUTE!("AIF1TX", None, "ADC 0"),
    SND_SOC_DAPM_ROUTE!("AIF2TX", None, "ADC 1"),
    SND_SOC_DAPM_ROUTE!("DAC 0", None, "AIF1RX"),
    SND_SOC_DAPM_ROUTE!("DAC 1", None, "AIF2RX"),
    SND_SOC_DAPM_ROUTE!("Front", Some("DAC Switch"), "DAC 0"),
    SND_SOC_DAPM_ROUTE!("Front", Some("RECMIX Switch"), "RECMIX"),
    SND_SOC_DAPM_ROUTE!("Surround", None, "DAC 1"),
    SND_SOC_DAPM_ROUTE!("SPK Mux", Some("Front"), "Front"),
    SND_SOC_DAPM_ROUTE!("SPK Mux", Some("Surround"), "Surround"),
    SND_SOC_DAPM_ROUTE!("HPO Mux", Some("Front"), "Front"),
    SND_SOC_DAPM_ROUTE!("HPO Mux", Some("Surround"), "Surround"),
    SND_SOC_DAPM_ROUTE!("SPO", Some("Switch"), "SPK Mux"),
    SND_SOC_DAPM_ROUTE!("HPO L", Some("Switch"), "HPO Mux"),
    SND_SOC_DAPM_ROUTE!("HPO R", Some("Switch"), "HPO Mux"),
    SND_SOC_DAPM_ROUTE!("HPO L", None, "HP Power"),
    SND_SOC_DAPM_ROUTE!("HPO R", None, "HP Power"),
    SND_SOC_DAPM_ROUTE!("SPOL", None, "SPO"),
    SND_SOC_DAPM_ROUTE!("SPOR", None, "SPO"),
    SND_SOC_DAPM_ROUTE!("HPO Pin", None, "HPO L"),
    SND_SOC_DAPM_ROUTE!("HPO Pin", None, "HPO R"),
];

pub unsafe fn rt286_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let component = (*dai).component;
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;
    let mut val: u32 = 0;
    let d_len_code: i32;

    match params_rate(params) {
        44100 => val |= 0x4000,
        48000 => {}
        _ => {
            dev_err((*component).dev, c"Unsupported sample rate %d\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }
    match (*rt286).sys_clk {
        12288000 | 24576000 => {
            if params_rate(params) != 48000 {
                dev_err((*component).dev, c"Sys_clk is not matched (%d %d)\n".as_ptr(), params_rate(params), (*rt286).sys_clk);
                return -EINVAL;
            }
        }
        11289600 | 22579200 => {
            if params_rate(params) != 44100 {
                dev_err((*component).dev, c"Sys_clk is not matched (%d %d)\n".as_ptr(), params_rate(params), (*rt286).sys_clk);
                return -EINVAL;
            }
        }
        _ => {}
    }

    if params_channels(params) <= 16 {
        /* bit 3:0 Number of Channel */
        val |= (params_channels(params) - 1) as u32;
    } else {
        dev_err((*component).dev, c"Unsupported channels %d\n".as_ptr(), params_channels(params));
        return -EINVAL;
    }

    match params_width(params) {
        16 => {
            d_len_code = 0;
            val |= 0x1 << 4;
        }
        32 => {
            d_len_code = 2;
            val |= 0x4 << 4;
        }
        20 => {
            d_len_code = 1;
            val |= 0x2 << 4;
        }
        24 => {
            d_len_code = 2;
            val |= 0x3 << 4;
        }
        8 => {
            d_len_code = 3;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x0018, (d_len_code << 3) as u32);
    dev_dbg((*component).dev, c"format val = 0x%x\n".as_ptr(), val);

    snd_soc_component_update_bits(component, RT286_DAC_FORMAT, 0x407f, val);
    snd_soc_component_update_bits(component, RT286_ADC_FORMAT, 0x407f, val);

    0
}

pub unsafe fn rt286_set_dai_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let component = (*dai).component;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x800, 0x800),
        SND_SOC_DAIFMT_CBC_CFC => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x800, 0x0),
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x300, 0x0),
        SND_SOC_DAIFMT_LEFT_J => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x300, 0x1 << 8),
        SND_SOC_DAIFMT_DSP_A => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x300, 0x2 << 8),
        SND_SOC_DAIFMT_DSP_B => snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x300, 0x3 << 8),
        _ => return -EINVAL,
    }
    /* bit 15 Stream Type 0:PCM 1:Non-PCM */
    snd_soc_component_update_bits(component, RT286_DAC_FORMAT, 0x8000, 0);
    snd_soc_component_update_bits(component, RT286_ADC_FORMAT, 0x8000, 0);

    0
}

pub unsafe fn rt286_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: i32,
    freq: u32,
    _dir: i32,
) -> i32 {
    let component = (*dai).component;
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    dev_dbg((*component).dev, c"%s freq=%d\n".as_ptr(), c"rt286_set_dai_sysclk".as_ptr(), freq);

    if RT286_SCLK_S_MCLK == clk_id {
        snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x0100, 0x0);
        snd_soc_component_update_bits(component, RT286_PLL_CTRL1, 0x20, 0x20);
    } else {
        snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x0100, 0x0100);
        snd_soc_component_update_bits(component, RT286_PLL_CTRL, 0x4, 0x4);
        snd_soc_component_update_bits(component, RT286_PLL_CTRL1, 0x20, 0x0);
    }

    match freq {
        19200000 => {
            if RT286_SCLK_S_MCLK == clk_id {
                dev_err((*component).dev, c"Should not use MCLK\n".as_ptr());
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x40, 0x40);
        }
        24000000 => {
            if RT286_SCLK_S_MCLK == clk_id {
                dev_err((*component).dev, c"Should not use MCLK\n".as_ptr());
                return -EINVAL;
            }
            snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x40, 0x0);
        }
        12288000 | 11289600 => {
            snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x8, 0x0);
            snd_soc_component_update_bits(component, RT286_CLK_DIV, 0xfc1e, 0x0004);
        }
        24576000 | 22579200 => {
            snd_soc_component_update_bits(component, RT286_I2S_CTRL2, 0x8, 0x8);
            snd_soc_component_update_bits(component, RT286_CLK_DIV, 0xfc1e, 0x5406);
        }
        _ => {
            dev_err((*component).dev, c"Unsupported system clock\n".as_ptr());
            return -EINVAL;
        }
    }

    (*rt286).sys_clk = freq as i32;
    (*rt286).clk_id = clk_id;

    0
}

pub unsafe fn rt286_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: u32) -> i32 {
    let component = (*dai).component;

    dev_dbg((*component).dev, c"%s ratio=%d\n".as_ptr(), c"rt286_set_bclk_ratio".as_ptr(), ratio);
    if 50 == ratio {
        snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x1000, 0x1000);
    } else {
        snd_soc_component_update_bits(component, RT286_I2S_CTRL1, 0x1000, 0x0);
    }

    0
}

pub unsafe fn rt286_set_bias_level(component: *mut snd_soc_component, level: snd_soc_bias_level) -> i32 {
    let dapm = snd_soc_component_to_dapm(component);

    match level {
        SND_SOC_BIAS_PREPARE => {
            if SND_SOC_BIAS_STANDBY == snd_soc_dapm_get_bias_level(dapm) {
                snd_soc_component_write(component, RT286_SET_AUDIO_POWER, AC_PWRST_D0);
                snd_soc_component_update_bits(component, RT286_DC_GAIN, 0x200, 0x200);
            }
        }
        SND_SOC_BIAS_ON => {
            mdelay(10);
            snd_soc_component_update_bits(component, RT286_DC_GAIN, 0x200, 0x0);
        }
        SND_SOC_BIAS_STANDBY => {
            snd_soc_component_write(component, RT286_SET_AUDIO_POWER, AC_PWRST_D3);
        }
        _ => {}
    }

    0
}

pub unsafe fn rt286_irq(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let rt286 = data as *mut rt286_priv;
    let mut hp = false;
    let mut mic = false;
    let mut status: i32 = 0;

    rt286_jack_detect(rt286, &mut hp, &mut mic);

    /* Clear IRQ */
    regmap_update_bits((*rt286).regmap, RT286_IRQ_CTRL, 0x1, 0x1);

    if hp {
        status |= SND_JACK_HEADPHONE;
    }
    if mic {
        status |= SND_JACK_MICROPHONE;
    }

    snd_soc_jack_report((*rt286).jack, status, SND_JACK_MICROPHONE | SND_JACK_HEADPHONE);

    pm_wakeup_event(&mut (*(*rt286).i2c).dev, 300);

    IRQ_HANDLED
}

pub unsafe fn rt286_probe(component: *mut snd_soc_component) -> i32 {
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    (*rt286).component = component;
    INIT_DELAYED_WORK!(&mut (*rt286).jack_detect_work, rt286_jack_detect_work);

    if (*(*rt286).i2c).irq != 0 {
        schedule_delayed_work(&mut (*rt286).jack_detect_work, msecs_to_jiffies(50));
    }
    0
}

pub unsafe fn rt286_remove(component: *mut snd_soc_component) {
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    cancel_delayed_work_sync(&mut (*rt286).jack_detect_work);
    (*rt286).component = core::ptr::null_mut();
}

// CONFIG_PM
pub unsafe fn rt286_suspend(component: *mut snd_soc_component) -> i32 {
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    regcache_cache_only((*rt286).regmap, true);
    regcache_mark_dirty((*rt286).regmap);

    0
}

// CONFIG_PM
pub unsafe fn rt286_resume(component: *mut snd_soc_component) -> i32 {
    let rt286 = snd_soc_component_get_drvdata(component) as *mut rt286_priv;

    regcache_cache_only((*rt286).regmap, false);
    rt286_index_sync(component);
    regcache_sync((*rt286).regmap);

    0
}

pub const RT286_STEREO_RATES: u32 = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
pub const RT286_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

pub static rt286_aif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt286_hw_params),
    set_fmt: Some(rt286_set_dai_fmt),
    set_sysclk: Some(rt286_set_dai_sysclk),
    set_bclk_ratio: Some(rt286_set_bclk_ratio),
};

pub static mut rt286_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"rt286-aif1".as_ptr(),
        id: RT286_AIF1,
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT286_STEREO_RATES,
            formats: RT286_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT286_STEREO_RATES,
            formats: RT286_FORMATS,
        },
        ops: &rt286_aif_dai_ops,
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: c"rt286-aif2".as_ptr(),
        id: RT286_AIF2,
        playback: snd_soc_pcm_stream {
            stream_name: c"AIF2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT286_STEREO_RATES,
            formats: RT286_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AIF2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: RT286_STEREO_RATES,
            formats: RT286_FORMATS,
        },
        ops: &rt286_aif_dai_ops,
        symmetric_rate: 1,
    },
];

pub static soc_component_dev_rt286: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt286_probe),
    remove: Some(rt286_remove),
    suspend: Some(rt286_suspend),
    resume: Some(rt286_resume),
    set_bias_level: Some(rt286_set_bias_level),
    set_jack: Some(rt286_mic_detect),
    controls: rt286_snd_controls.as_ptr(),
    num_controls: rt286_snd_controls.len(),
    dapm_widgets: rt286_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt286_dapm_widgets.len(),
    dapm_routes: rt286_dapm_routes.as_ptr(),
    num_dapm_routes: rt286_dapm_routes.len(),
    use_pmdown_time: 1,
    endianness: 1,
};

pub static rt286_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    max_register: 0x02370100,
    volatile_reg: Some(rt286_volatile_register),
    readable_reg: Some(rt286_readable_register),
    reg_write: Some(rl6347a_hw_write),
    reg_read: Some(rl6347a_hw_read),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: rt286_reg.as_ptr(),
    num_reg_defaults: rt286_reg.len(),
};

pub static rt286_i2c_id: [i2c_device_id; 3] = [
    i2c_device_id { name: *b"rt286\0" },
    i2c_device_id { name: *b"rt288\0" },
    i2c_device_id { name: [0; 6] },
];
MODULE_DEVICE_TABLE!(i2c, rt286_i2c_id);

// CONFIG_ACPI
pub static rt286_acpi_match: [acpi_device_id; 3] = [
    ACPI_DEVICE_ID!("10EC0286"),
    ACPI_DEVICE_ID!("INT343A"),
    ACPI_DEVICE_ID!(""),
];
MODULE_DEVICE_TABLE!(acpi, rt286_acpi_match);

pub static force_combo_jack_table: [dmi_system_id; 5] = [
    DMI_SYSTEM_ID!("Intel Wilson Beach", DMI_MATCH!(DMI_BOARD_NAME, "Wilson Beach SDS")),
    DMI_SYSTEM_ID!("Intel Skylake RVP", DMI_MATCH!(DMI_PRODUCT_NAME, "Skylake Client platform")),
    DMI_SYSTEM_ID!("Intel Kabylake RVP", DMI_MATCH!(DMI_PRODUCT_NAME, "Kabylake Client platform")),
    DMI_SYSTEM_ID!(
        "Thinkpad Helix 2nd",
        DMI_MATCH!(DMI_SYS_VENDOR, "LENOVO"),
        DMI_MATCH!(DMI_PRODUCT_VERSION, "ThinkPad Helix 2nd")
    ),
    DMI_SYSTEM_ID_EMPTY!(),
];

pub static dmi_dell: [dmi_system_id; 2] = [
    DMI_SYSTEM_ID!("Dell", DMI_MATCH!(DMI_SYS_VENDOR, "Dell Inc.")),
    DMI_SYSTEM_ID_EMPTY!(),
];

pub unsafe fn rt286_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut rt286_platform_data;
    let rt286: *mut rt286_priv;
    let mut i: i32;
    let mut ret: i32;
    let mut vendor_id: u32 = 0;

    rt286 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<rt286_priv>(), GFP_KERNEL) as *mut rt286_priv;
    if rt286.is_null() {
        return -ENOMEM;
    }

    (*rt286).regmap = devm_regmap_init(&mut (*i2c).dev, core::ptr::null_mut(), i2c as *mut core::ffi::c_void, &rt286_regmap);
    if IS_ERR((*rt286).regmap) {
        ret = PTR_ERR((*rt286).regmap);
        dev_err(&mut (*i2c).dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = regmap_read((*rt286).regmap, RT286_GET_PARAM(AC_NODE_ROOT, AC_PAR_VENDOR_ID), &mut vendor_id);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"I2C error %d\n".as_ptr(), ret);
        return ret;
    }
    if vendor_id != RT286_VENDOR_ID && vendor_id != RT288_VENDOR_ID {
        dev_err(&mut (*i2c).dev, c"Device with ID register %#x is not rt286\n".as_ptr(), vendor_id);
        return -ENODEV;
    }

    (*rt286).index_cache = devm_kmemdup(
        &mut (*i2c).dev,
        rt286_index_def.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&rt286_index_def),
        GFP_KERNEL,
    ) as *mut reg_default;
    if (*rt286).index_cache.is_null() {
        return -ENOMEM;
    }

    (*rt286).index_cache_size = INDEX_CACHE_SIZE as i32;
    (*rt286).i2c = i2c;
    i2c_set_clientdata(i2c, rt286 as *mut core::ffi::c_void);

    /* restore codec default */
    i = 0;
    while (i as usize) < INDEX_CACHE_SIZE {
        regmap_write(
            (*rt286).regmap,
            (*(*rt286).index_cache.add(i as usize)).reg,
            (*(*rt286).index_cache.add(i as usize)).def,
        );
        i += 1;
    }
    i = 0;
    while (i as usize) < rt286_reg.len() {
        regmap_write((*rt286).regmap, rt286_reg[i as usize].reg, rt286_reg[i as usize].def);
        i += 1;
    }

    if !pdata.is_null() {
        (*rt286).pdata = *pdata;
    }

    if (vendor_id == RT288_VENDOR_ID && dmi_check_system(dmi_dell.as_ptr())) || dmi_check_system(force_combo_jack_table.as_ptr()) {
        (*rt286).pdata.cbj_en = true;
    }

    regmap_write((*rt286).regmap, RT286_SET_AUDIO_POWER, AC_PWRST_D3);

    i = 0;
    while (i as usize) < RT286_POWER_REG_LEN {
        regmap_write(
            (*rt286).regmap,
            RT286_SET_POWER(rt286_support_power_controls[i as usize]),
            AC_PWRST_D1,
        );
        i += 1;
    }

    if !(*rt286).pdata.cbj_en {
        regmap_write((*rt286).regmap, RT286_CBJ_CTRL2, 0x0000);
        regmap_write((*rt286).regmap, RT286_MIC1_DET_CTRL, 0x0816);
        regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0xf000, 0xb000);
    } else {
        regmap_update_bits((*rt286).regmap, RT286_CBJ_CTRL1, 0xf000, 0x5000);
    }

    mdelay(10);

    if !(*rt286).pdata.gpio2_en {
        regmap_write((*rt286).regmap, RT286_SET_DMIC2_DEFAULT, 0x40);
    } else {
        regmap_write((*rt286).regmap, RT286_SET_DMIC2_DEFAULT, 0);
    }

    mdelay(10);

    regmap_write((*rt286).regmap, RT286_MISC_CTRL1, 0x0000);
    /* Power down LDO, VREF */
    regmap_update_bits((*rt286).regmap, RT286_POWER_CTRL2, 0xc, 0x0);
    regmap_update_bits((*rt286).regmap, RT286_POWER_CTRL1, 0x1001, 0x1001);

    /* Set depop parameter */
    regmap_update_bits((*rt286).regmap, RT286_DEPOP_CTRL2, 0x403a, 0x401a);
    regmap_update_bits((*rt286).regmap, RT286_DEPOP_CTRL3, 0xf777, 0x4737);
    regmap_update_bits((*rt286).regmap, RT286_DEPOP_CTRL4, 0x00ff, 0x003f);

    if vendor_id == RT288_VENDOR_ID && dmi_check_system(dmi_dell.as_ptr()) {
        regmap_update_bits((*rt286).regmap, RT286_SET_GPIO_MASK, 0x40, 0x40);
        regmap_update_bits((*rt286).regmap, RT286_SET_GPIO_DIRECTION, 0x40, 0x40);
        regmap_update_bits((*rt286).regmap, RT286_SET_GPIO_DATA, 0x40, 0x40);
        regmap_update_bits((*rt286).regmap, RT286_GPIO_CTRL, 0xc, 0x8);
    }

    if (*i2c).irq != 0 {
        ret = devm_request_threaded_irq(
            &mut (*i2c).dev,
            (*i2c).irq,
            None,
            Some(rt286_irq),
            IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
            c"rt286".as_ptr(),
            rt286 as *mut core::ffi::c_void,
        );
        if ret != 0 {
            dev_err(&mut (*i2c).dev, c"Failed to request IRQ: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_rt286,
        rt286_dai.as_mut_ptr(),
        rt286_dai.len(),
    );

    ret
}

pub static mut rt286_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"rt286".as_ptr(),
        acpi_match_table: ACPI_PTR!(rt286_acpi_match),
    },
    probe: Some(rt286_i2c_probe),
    id_table: rt286_i2c_id.as_ptr(),
};

module_i2c_driver!(rt286_i2c_driver);

MODULE_DESCRIPTION!("ASoC RT286 driver");
MODULE_AUTHOR!("Bard Liao <bardliao@realtek.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
