// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021, Maxim Integrated

// Translated from Linux C includes:
// linux/acpi.h, linux/delay.h, linux/i2c.h, linux/module.h, linux/regmap.h,
// linux/slab.h, linux/cdev.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// linux/gpio/consumer.h, linux/of.h, sound/tlv.h, and "max98520.h".

static max98520_reg: [reg_default; 50] = [
    reg_default { reg: MAX98520_R2000_SW_RESET, def: 0x00 },
    reg_default { reg: MAX98520_R2001_STATUS_1, def: 0x00 },
    reg_default { reg: MAX98520_R2002_STATUS_2, def: 0x00 },
    reg_default { reg: MAX98520_R2020_THERM_WARN_THRESH, def: 0x46 },
    reg_default { reg: MAX98520_R2021_THERM_SHDN_THRESH, def: 0x64 },
    reg_default { reg: MAX98520_R2022_THERM_HYSTERESIS, def: 0x02 },
    reg_default { reg: MAX98520_R2023_THERM_FOLDBACK_SET, def: 0x31 },
    reg_default { reg: MAX98520_R2027_THERM_FOLDBACK_EN, def: 0x01 },
    reg_default { reg: MAX98520_R2030_CLK_MON_CTRL, def: 0x00 },
    reg_default { reg: MAX98520_R2037_ERR_MON_CTRL, def: 0x01 },
    reg_default { reg: MAX98520_R2040_PCM_MODE_CFG, def: 0xC0 },
    reg_default { reg: MAX98520_R2041_PCM_CLK_SETUP, def: 0x04 },
    reg_default { reg: MAX98520_R2042_PCM_SR_SETUP, def: 0x08 },
    reg_default { reg: MAX98520_R2043_PCM_RX_SRC1, def: 0x00 },
    reg_default { reg: MAX98520_R2044_PCM_RX_SRC2, def: 0x00 },
    reg_default { reg: MAX98520_R204F_PCM_RX_EN, def: 0x00 },
    reg_default { reg: MAX98520_R2090_AMP_VOL_CTRL, def: 0x00 },
    reg_default { reg: MAX98520_R2091_AMP_PATH_GAIN, def: 0x03 },
    reg_default { reg: MAX98520_R2092_AMP_DSP_CFG, def: 0x02 },
    reg_default { reg: MAX98520_R2094_SSM_CFG, def: 0x01 },
    reg_default { reg: MAX98520_R2095_AMP_CFG, def: 0xF0 },
    reg_default { reg: MAX98520_R209F_AMP_EN, def: 0x00 },
    reg_default { reg: MAX98520_R20B0_ADC_SR, def: 0x00 },
    reg_default { reg: MAX98520_R20B1_ADC_RESOLUTION, def: 0x00 },
    reg_default { reg: MAX98520_R20B2_ADC_PVDD0_CFG, def: 0x02 },
    reg_default { reg: MAX98520_R20B3_ADC_THERMAL_CFG, def: 0x02 },
    reg_default { reg: MAX98520_R20B4_ADC_READBACK_CTRL, def: 0x00 },
    reg_default { reg: MAX98520_R20B5_ADC_READBACK_UPDATE, def: 0x00 },
    reg_default { reg: MAX98520_R20B6_ADC_PVDD_READBACK_MSB, def: 0x00 },
    reg_default { reg: MAX98520_R20B7_ADC_PVDD_READBACK_LSB, def: 0x00 },
    reg_default { reg: MAX98520_R20B8_ADC_TEMP_READBACK_MSB, def: 0x00 },
    reg_default { reg: MAX98520_R20B9_ADC_TEMP_READBACK_LSB, def: 0x00 },
    reg_default { reg: MAX98520_R20BA_ADC_LOW_PVDD_READBACK_MSB, def: 0xFF },
    reg_default { reg: MAX98520_R20BB_ADC_LOW_READBACK_LSB, def: 0x01 },
    reg_default { reg: MAX98520_R20BC_ADC_HIGH_TEMP_READBACK_MSB, def: 0x00 },
    reg_default { reg: MAX98520_R20BD_ADC_HIGH_TEMP_READBACK_LSB, def: 0x00 },
    reg_default { reg: MAX98520_R20CF_MEAS_ADC_CFG, def: 0x00 },
    reg_default { reg: MAX98520_R20D0_DHT_CFG1, def: 0x00 },
    reg_default { reg: MAX98520_R20D1_LIMITER_CFG1, def: 0x08 },
    reg_default { reg: MAX98520_R20D2_LIMITER_CFG2, def: 0x00 },
    reg_default { reg: MAX98520_R20D3_DHT_CFG2, def: 0x14 },
    reg_default { reg: MAX98520_R20D4_DHT_CFG3, def: 0x02 },
    reg_default { reg: MAX98520_R20D5_DHT_CFG4, def: 0x04 },
    reg_default { reg: MAX98520_R20D6_DHT_HYSTERESIS_CFG, def: 0x07 },
    reg_default { reg: MAX98520_R20D8_DHT_EN, def: 0x00 },
    reg_default { reg: MAX98520_R210E_AUTO_RESTART_BEHAVIOR, def: 0x00 },
    reg_default { reg: MAX98520_R210F_GLOBAL_EN, def: 0x00 },
    reg_default { reg: MAX98520_R21FF_REVISION_ID, def: 0x00 },
];

unsafe fn max98520_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let component = (*codec_dai).component;
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;
    let mut format: u32 = 0;
    let mut invert: u32 = 0;

    dev_dbg!((*component).dev, "%s: fmt 0x%08X\n", __func__, fmt);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => {
            invert = MAX98520_PCM_MODE_CFG_PCM_BCLKEDGE;
        }
        _ => {
            dev_err!((*component).dev, "DAI invert mode unsupported\n");
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2041_PCM_CLK_SETUP,
        MAX98520_PCM_MODE_CFG_PCM_BCLKEDGE,
        invert,
    );

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            format = MAX98520_PCM_FORMAT_I2S;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            format = MAX98520_PCM_FORMAT_LJ;
        }
        SND_SOC_DAIFMT_DSP_A => {
            format = MAX98520_PCM_FORMAT_TDM_MODE1;
        }
        SND_SOC_DAIFMT_DSP_B => {
            format = MAX98520_PCM_FORMAT_TDM_MODE0;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2040_PCM_MODE_CFG,
        MAX98520_PCM_MODE_CFG_FORMAT_MASK,
        format << MAX98520_PCM_MODE_CFG_FORMAT_SHIFT,
    );

    0
}

/* BCLKs per LRCLK */
static bclk_sel_table: [i32; 10] = [32, 48, 64, 96, 128, 192, 256, 384, 512, 320];

fn max98520_get_bclk_sel(bclk: i32) -> i32 {
    /* match BCLKs per LRCLK */
    for i in 0..bclk_sel_table.len() {
        if bclk_sel_table[i] == bclk {
            return i as i32 + 2;
        }
    }
    0
}

unsafe fn max98520_set_clock(component: *mut snd_soc_component, params: *mut snd_pcm_hw_params) -> i32 {
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio = params_channels(params) * (*max98520).ch_size;
    let value: i32;

    if !(*max98520).tdm_mode {
        /* BCLK configuration */
        value = max98520_get_bclk_sel(blr_clk_ratio);
        if value == 0 {
            dev_err!(
                (*component).dev,
                "format unsupported %d\n",
                params_format(params)
            );
            return -EINVAL;
        }

        regmap_update_bits(
            (*max98520).regmap,
            MAX98520_R2041_PCM_CLK_SETUP,
            MAX98520_PCM_CLK_SETUP_BSEL_MASK,
            value as u32,
        );
    }
    dev_dbg!((*component).dev, "%s tdm_mode:%d out\n", __func__, (*max98520).tdm_mode);
    0
}

unsafe fn max98520_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let component = (*dai).component;
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;
    let mut sampling_rate: u32 = 0;
    let mut chan_sz: u32 = 0;

    /* pcm mode configuration */
    match snd_pcm_format_width(params_format(params)) {
        16 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err!(
                (*component).dev,
                "format unsupported %d\n",
                params_format(params)
            );
            dev_dbg!((*component).dev, "%s out error", __func__);
            return -EINVAL;
        }
    }

    (*max98520).ch_size = snd_pcm_format_width(params_format(params));

    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2040_PCM_MODE_CFG,
        MAX98520_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz,
    );

    dev_dbg!((*component).dev, "format supported %d", params_format(params));

    /* sampling rate configuration */
    match params_rate(params) {
        8000 => sampling_rate = MAX98520_PCM_SR_8000,
        11025 => sampling_rate = MAX98520_PCM_SR_11025,
        12000 => sampling_rate = MAX98520_PCM_SR_12000,
        16000 => sampling_rate = MAX98520_PCM_SR_16000,
        22050 => sampling_rate = MAX98520_PCM_SR_22050,
        24000 => sampling_rate = MAX98520_PCM_SR_24000,
        32000 => sampling_rate = MAX98520_PCM_SR_32000,
        44100 => sampling_rate = MAX98520_PCM_SR_44100,
        48000 => sampling_rate = MAX98520_PCM_SR_48000,
        88200 => sampling_rate = MAX98520_PCM_SR_88200,
        96000 => sampling_rate = MAX98520_PCM_SR_96000,
        176400 => sampling_rate = MAX98520_PCM_SR_176400,
        192000 => sampling_rate = MAX98520_PCM_SR_192000,
        _ => {
            dev_err!((*component).dev, "rate %d not supported\n", params_rate(params));
            dev_dbg!((*component).dev, "%s out error", __func__);
            return -EINVAL;
        }
    }

    dev_dbg!(
        (*component).dev,
        " %s ch_size: %d, sampling rate : %d out\n",
        __func__,
        snd_pcm_format_width(params_format(params)),
        params_rate(params)
    );
    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2042_PCM_SR_SETUP,
        MAX98520_PCM_SR_MASK,
        sampling_rate,
    );

    max98520_set_clock(component, params)
}

unsafe fn max98520_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    slots: i32,
    slot_width: i32,
) -> i32 {
    let component = (*dai).component;
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;
    let bsel: i32;
    let mut chan_sz: u32 = 0;

    if tx_mask == 0 && rx_mask == 0 && slots == 0 && slot_width == 0 {
        (*max98520).tdm_mode = false;
    } else {
        (*max98520).tdm_mode = true;
    }

    /* BCLK configuration */
    bsel = max98520_get_bclk_sel(slots * slot_width);
    if bsel == 0 {
        dev_err!((*component).dev, "BCLK %d not supported\n", slots * slot_width);
        return -EINVAL;
    }

    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2041_PCM_CLK_SETUP,
        MAX98520_PCM_CLK_SETUP_BSEL_MASK,
        bsel as u32,
    );

    /* Channel size configuration */
    match slot_width {
        16 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98520_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err!((*component).dev, "format unsupported %d\n", slot_width);
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2040_PCM_MODE_CFG,
        MAX98520_PCM_MODE_CFG_CHANSZ_MASK,
        chan_sz,
    );

    /* Rx slot configuration */
    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2044_PCM_RX_SRC2,
        MAX98520_PCM_DMIX_CH0_SRC_MASK,
        rx_mask,
    );
    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R2044_PCM_RX_SRC2,
        MAX98520_PCM_DMIX_CH1_SRC_MASK,
        rx_mask << MAX98520_PCM_DMIX_CH1_SHIFT,
    );

    0
}

const MAX98520_RATES: u32 = SNDRV_PCM_RATE_8000_192000;

const MAX98520_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static max98520_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(max98520_dai_set_fmt),
    hw_params: Some(max98520_dai_hw_params),
    set_tdm_slot: Some(max98520_dai_tdm_slot),
};

unsafe fn max98520_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            dev_dbg!((*component).dev, " AMP ON\n");

            regmap_write((*max98520).regmap, MAX98520_R209F_AMP_EN, 1);
            regmap_write((*max98520).regmap, MAX98520_R210F_GLOBAL_EN, 1);
            usleep_range(30000, 31000);
        }
        SND_SOC_DAPM_POST_PMD => {
            dev_dbg!((*component).dev, " AMP OFF\n");

            regmap_write((*max98520).regmap, MAX98520_R210F_GLOBAL_EN, 0);
            regmap_write((*max98520).regmap, MAX98520_R209F_AMP_EN, 0);
            usleep_range(30000, 31000);
        }
        _ => return 0,
    }
    0
}

static max98520_switch_text: [&str; 3] = ["Left", "Right", "LeftRight"];

static dai_sel_enum: soc_enum = SOC_ENUM_SINGLE!(
    MAX98520_R2043_PCM_RX_SRC1,
    0,
    3,
    max98520_switch_text
);

static max98520_dai_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("DAI Sel", dai_sel_enum);

static max98520_left_input_mixer_controls: [snd_kcontrol_new; 16] = [
    SOC_DAPM_SINGLE!("PCM_INPUT_CH0", MAX98520_R2044_PCM_RX_SRC2, 0, 0x0, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH1", MAX98520_R2044_PCM_RX_SRC2, 0, 0x1, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH2", MAX98520_R2044_PCM_RX_SRC2, 0, 0x2, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH3", MAX98520_R2044_PCM_RX_SRC2, 0, 0x3, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH4", MAX98520_R2044_PCM_RX_SRC2, 0, 0x4, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH5", MAX98520_R2044_PCM_RX_SRC2, 0, 0x5, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH6", MAX98520_R2044_PCM_RX_SRC2, 0, 0x6, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH7", MAX98520_R2044_PCM_RX_SRC2, 0, 0x7, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH8", MAX98520_R2044_PCM_RX_SRC2, 0, 0x8, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH9", MAX98520_R2044_PCM_RX_SRC2, 0, 0x9, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH10", MAX98520_R2044_PCM_RX_SRC2, 0, 0xa, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH11", MAX98520_R2044_PCM_RX_SRC2, 0, 0xb, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH12", MAX98520_R2044_PCM_RX_SRC2, 0, 0xc, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH13", MAX98520_R2044_PCM_RX_SRC2, 0, 0xd, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH14", MAX98520_R2044_PCM_RX_SRC2, 0, 0xe, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH15", MAX98520_R2044_PCM_RX_SRC2, 0, 0xf, 0),
];

static max98520_right_input_mixer_controls: [snd_kcontrol_new; 16] = [
    SOC_DAPM_SINGLE!("PCM_INPUT_CH0", MAX98520_R2044_PCM_RX_SRC2, 4, 0x0, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH1", MAX98520_R2044_PCM_RX_SRC2, 4, 0x1, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH2", MAX98520_R2044_PCM_RX_SRC2, 4, 0x2, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH3", MAX98520_R2044_PCM_RX_SRC2, 4, 0x3, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH4", MAX98520_R2044_PCM_RX_SRC2, 4, 0x4, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH5", MAX98520_R2044_PCM_RX_SRC2, 4, 0x5, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH6", MAX98520_R2044_PCM_RX_SRC2, 4, 0x6, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH7", MAX98520_R2044_PCM_RX_SRC2, 4, 0x7, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH8", MAX98520_R2044_PCM_RX_SRC2, 4, 0x8, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH9", MAX98520_R2044_PCM_RX_SRC2, 4, 0x9, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH10", MAX98520_R2044_PCM_RX_SRC2, 4, 0xa, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH11", MAX98520_R2044_PCM_RX_SRC2, 4, 0xb, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH12", MAX98520_R2044_PCM_RX_SRC2, 4, 0xc, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH13", MAX98520_R2044_PCM_RX_SRC2, 4, 0xd, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH14", MAX98520_R2044_PCM_RX_SRC2, 4, 0xe, 0),
    SOC_DAPM_SINGLE!("PCM_INPUT_CH15", MAX98520_R2044_PCM_RX_SRC2, 4, 0xf, 0),
];

static max98520_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_DAC_E!(
        "Amp Enable",
        "HiFi Playback",
        SND_SOC_NOPM,
        0,
        0,
        max98520_dac_event,
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_MUX!("DAI Sel Mux", SND_SOC_NOPM, 0, 0, &max98520_dai_controls),
    SND_SOC_DAPM_OUTPUT!("BE_OUT"),
    /* Left Input Selection */
    SND_SOC_DAPM_MIXER!(
        "Left Input Selection",
        SND_SOC_NOPM,
        0,
        0,
        &max98520_left_input_mixer_controls[0],
        max98520_left_input_mixer_controls.len()
    ),
    /* Right Input Selection */
    SND_SOC_DAPM_MIXER!(
        "Right Input Selection",
        SND_SOC_NOPM,
        0,
        0,
        &max98520_right_input_mixer_controls[0],
        max98520_right_input_mixer_controls.len()
    ),
];

static max98520_digital_tlv: [u32; 4] = DECLARE_TLV_DB_SCALE!(-6300, 50, 1);
static max98520_spk_tlv: [u32; 4] = DECLARE_TLV_DB_SCALE!(-600, 300, 0);

static max98520_dht_lim_thresh_tlv: &[u32] = &DECLARE_TLV_DB_RANGE!(
    0, 15, TLV_DB_SCALE_ITEM!(-1500, 100, 0),
);

static max98520_dht_hysteresis_tlv: &[u32] = &DECLARE_TLV_DB_RANGE!(
    0, 3, TLV_DB_SCALE_ITEM!(100, 100, 0),
    4, 7, TLV_DB_SCALE_ITEM!(600, 200, 0),
);

static max98520_dht_rotation_point_tlv: &[u32] = &DECLARE_TLV_DB_RANGE!(
    0, 1, TLV_DB_SCALE_ITEM!(-1500, 300, 0),
    2, 4, TLV_DB_SCALE_ITEM!(-1000, 200, 0),
    5, 10, TLV_DB_SCALE_ITEM!(-500, 100, 0),
);

static max98520_dht_supply_hr_tlv: &[u32] = &DECLARE_TLV_DB_RANGE!(
    0, 16, TLV_DB_SCALE_ITEM!(-2000, 250, 0),
);

static max98520_dht_max_atten_tlv: &[u32] = &DECLARE_TLV_DB_RANGE!(
    1, 20, TLV_DB_SCALE_ITEM!(-2000, 100, 0),
);

static max98520_dht_attack_rate_text: [&str; 14] = [
    "20us", "40us", "80us", "160us", "320us", "640us", "1.28ms", "2.56ms",
    "5.12ms", "10.24ms", "20.48ms", "40.96ms", "81.92ms", "163.84ms",
];

static max98520_dht_attack_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98520_R20D4_DHT_CFG3,
    0,
    max98520_dht_attack_rate_text
);

static max98520_dht_release_rate_text: [&str; 14] = [
    "2ms", "4ms", "8ms", "16ms", "32ms", "64ms", "128ms", "256ms", "512ms",
    "1.024s", "2.048s", "4.096s", "8.192s", "16.384s",
];

static max98520_dht_release_rate_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(
    MAX98520_R20D5_DHT_CFG4,
    0,
    max98520_dht_release_rate_text
);

unsafe fn max98520_readable_register(_dev: *mut device, reg: u32) -> bool {
    match reg {
        MAX98520_R2000_SW_RESET
        | MAX98520_R2027_THERM_FOLDBACK_EN
        | MAX98520_R2030_CLK_MON_CTRL
        | MAX98520_R2037_ERR_MON_CTRL
        | MAX98520_R204F_PCM_RX_EN
        | MAX98520_R209F_AMP_EN
        | MAX98520_R20CF_MEAS_ADC_CFG
        | MAX98520_R20D8_DHT_EN
        | MAX98520_R21FF_REVISION_ID => true,
        MAX98520_R2001_STATUS_1..=MAX98520_R2002_STATUS_2
        | MAX98520_R2020_THERM_WARN_THRESH..=MAX98520_R2023_THERM_FOLDBACK_SET
        | MAX98520_R2040_PCM_MODE_CFG..=MAX98520_R2044_PCM_RX_SRC2
        | MAX98520_R2090_AMP_VOL_CTRL..=MAX98520_R2092_AMP_DSP_CFG
        | MAX98520_R2094_SSM_CFG..=MAX98520_R2095_AMP_CFG
        | MAX98520_R20B0_ADC_SR..=MAX98520_R20BD_ADC_HIGH_TEMP_READBACK_LSB
        | MAX98520_R20D0_DHT_CFG1..=MAX98520_R20D6_DHT_HYSTERESIS_CFG
        | MAX98520_R210E_AUTO_RESTART_BEHAVIOR..=MAX98520_R210F_GLOBAL_EN
        | MAX98520_R2161_BOOST_TM1..=MAX98520_R2163_BOOST_TM3 => true,
        _ => false,
    }
}

unsafe fn max98520_volatile_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        MAX98520_R210F_GLOBAL_EN
        | MAX98520_R21FF_REVISION_ID
        | MAX98520_R2000_SW_RESET => true,
        MAX98520_R2001_STATUS_1..=MAX98520_R2002_STATUS_2
        | MAX98520_R20B4_ADC_READBACK_CTRL..=MAX98520_R20BD_ADC_HIGH_TEMP_READBACK_LSB => true,
        _ => false,
    }
}

static max98520_snd_controls: [snd_kcontrol_new; 26] = [
    /* Volume */
    SOC_SINGLE_TLV!("Digital Volume", MAX98520_R2090_AMP_VOL_CTRL, 0, 0x7F, 1, max98520_digital_tlv),
    SOC_SINGLE_TLV!("Speaker Volume", MAX98520_R2091_AMP_PATH_GAIN, 0, 0x5, 0, max98520_spk_tlv),
    /* Volume Ramp Up/Down Enable*/
    SOC_SINGLE!("Ramp Up Switch", MAX98520_R2092_AMP_DSP_CFG, MAX98520_DSP_SPK_VOL_RMPUP_SHIFT, 1, 0),
    SOC_SINGLE!("Ramp Down Switch", MAX98520_R2092_AMP_DSP_CFG, MAX98520_DSP_SPK_VOL_RMPDN_SHIFT, 1, 0),
    /* Clock Monitor Enable */
    SOC_SINGLE!("CLK Monitor Switch", MAX98520_R2037_ERR_MON_CTRL, MAX98520_CTRL_CMON_EN_SHIFT, 1, 0),
    /* Clock Monitor Config */
    SOC_SINGLE!("CLKMON Autorestart Switch", MAX98520_R2030_CLK_MON_CTRL, MAX98520_CMON_AUTORESTART_SHIFT, 1, 0),
    /* Dither Enable */
    SOC_SINGLE!("Dither Switch", MAX98520_R2092_AMP_DSP_CFG, MAX98520_DSP_SPK_DITH_EN_SHIFT, 1, 0),
    /* DC Blocker Enable */
    SOC_SINGLE!("DC Blocker Switch", MAX98520_R2092_AMP_DSP_CFG, MAX98520_DSP_SPK_DCBLK_EN_SHIFT, 1, 0),
    /* Speaker Safe Mode Enable */
    SOC_SINGLE!("Speaker Safemode Switch", MAX98520_R2092_AMP_DSP_CFG, MAX98520_DSP_SPK_SAFE_EN_SHIFT, 1, 0),
    /* AMP SSM Enable */
    SOC_SINGLE!("CP Bypass Switch", MAX98520_R2094_SSM_CFG, MAX98520_SSM_RCVR_MODE_SHIFT, 1, 0),
    /* Dynamic Headroom Tracking */
    SOC_SINGLE!("DHT Switch", MAX98520_R20D8_DHT_EN, 0, 1, 0),
    SOC_SINGLE!("DHT Limiter Mode", MAX98520_R20D2_LIMITER_CFG2, MAX98520_DHT_LIMITER_MODE_SHIFT, 1, 0),
    SOC_SINGLE!("DHT Hysteresis Switch", MAX98520_R20D6_DHT_HYSTERESIS_CFG, MAX98520_DHT_HYSTERESIS_SWITCH_SHIFT, 1, 0),
    SOC_SINGLE_TLV!("DHT Rot Pnt", MAX98520_R20D0_DHT_CFG1, MAX98520_DHT_VROT_PNT_SHIFT, 10, 1, max98520_dht_rotation_point_tlv),
    SOC_SINGLE_TLV!("DHT Supply Headroom", MAX98520_R20D1_LIMITER_CFG1, MAX98520_DHT_SUPPLY_HR_SHIFT, 16, 0, max98520_dht_supply_hr_tlv),
    SOC_SINGLE_TLV!("DHT Limiter Threshold", MAX98520_R20D2_LIMITER_CFG2, MAX98520_DHT_LIMITER_THRESHOLD_SHIFT, 0xF, 1, max98520_dht_lim_thresh_tlv),
    SOC_SINGLE_TLV!("DHT Max Attenuation", MAX98520_R20D3_DHT_CFG2, MAX98520_DHT_MAX_ATTEN_SHIFT, 20, 1, max98520_dht_max_atten_tlv),
    SOC_SINGLE_TLV!("DHT Hysteresis", MAX98520_R20D6_DHT_HYSTERESIS_CFG, MAX98520_DHT_HYSTERESIS_SHIFT, 0x7, 0, max98520_dht_hysteresis_tlv),
    SOC_ENUM!("DHT Attack Rate", max98520_dht_attack_rate_enum),
    SOC_ENUM!("DHT Release Rate", max98520_dht_release_rate_enum),
    /* ADC configuration */
    SOC_SINGLE!("ADC PVDD CH Switch", MAX98520_R20CF_MEAS_ADC_CFG, 0, 1, 0),
    SOC_SINGLE!("ADC PVDD FLT Switch", MAX98520_R20B2_ADC_PVDD0_CFG, MAX98520_FLT_EN_SHIFT, 1, 0),
    SOC_SINGLE!("ADC TEMP FLT Switch", MAX98520_R20B3_ADC_THERMAL_CFG, MAX98520_FLT_EN_SHIFT, 1, 0),
    SOC_SINGLE!("ADC PVDD MSB", MAX98520_R20B6_ADC_PVDD_READBACK_MSB, 0, 0xFF, 0),
    SOC_SINGLE!("ADC PVDD LSB", MAX98520_R20B7_ADC_PVDD_READBACK_LSB, 0, 0x01, 0),
    SOC_SINGLE!("ADC TEMP MSB", MAX98520_R20B8_ADC_TEMP_READBACK_MSB, 0, 0xFF, 0),
    SOC_SINGLE!("ADC TEMP LSB", MAX98520_R20B9_ADC_TEMP_READBACK_LSB, 0, 0x01, 0),
];

static max98520_audio_map: [snd_soc_dapm_route; 4] = [
    /* Plabyack */
    snd_soc_dapm_route { sink: "DAI Sel Mux", control: "Left", source: "Amp Enable" },
    snd_soc_dapm_route { sink: "DAI Sel Mux", control: "Right", source: "Amp Enable" },
    snd_soc_dapm_route { sink: "DAI Sel Mux", control: "LeftRight", source: "Amp Enable" },
    snd_soc_dapm_route { sink: "BE_OUT", control: core::ptr::null(), source: "DAI Sel Mux" },
];

static mut max98520_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: "max98520-aif1",
        playback: snd_soc_pcm_stream {
            stream_name: "HiFi Playback",
            channels_min: 1,
            channels_max: 2,
            rates: MAX98520_RATES,
            formats: MAX98520_FORMATS,
        },
        ops: &max98520_dai_ops,
    },
];

unsafe fn max98520_probe(component: *mut snd_soc_component) -> i32 {
    let max98520 = snd_soc_component_get_drvdata(component) as *mut max98520_priv;

    /* Software Reset */
    regmap_write((*max98520).regmap, MAX98520_R2000_SW_RESET, 1);

    /* L/R mono mix configuration : "DAI Sel" for 0x2043 */
    regmap_write((*max98520).regmap, MAX98520_R2043_PCM_RX_SRC1, 0x2);

    /* PCM input channles configuration : "Left Input Selection" for 0x2044 */
    /* PCM input channles configuration : "Right Input Selection" for 0x2044 */
    regmap_write((*max98520).regmap, MAX98520_R2044_PCM_RX_SRC2, 0x10);

    /* Enable DC blocker */
    regmap_update_bits((*max98520).regmap, MAX98520_R2092_AMP_DSP_CFG, 1, 1);
    /* Enable Clock Monitor Auto-restart */
    regmap_write((*max98520).regmap, MAX98520_R2030_CLK_MON_CTRL, 0x1);

    /* set Rx Enable */
    regmap_update_bits(
        (*max98520).regmap,
        MAX98520_R204F_PCM_RX_EN,
        MAX98520_PCM_RX_EN_MASK,
        1,
    );

    0
}

unsafe fn max98520_suspend(dev: *mut device) -> i32 {
    let max98520 = dev_get_drvdata(dev) as *mut max98520_priv;

    regcache_cache_only((*max98520).regmap, true);
    regcache_mark_dirty((*max98520).regmap);
    0
}

unsafe fn max98520_resume(dev: *mut device) -> i32 {
    let max98520 = dev_get_drvdata(dev) as *mut max98520_priv;

    regcache_cache_only((*max98520).regmap, false);
    regmap_write((*max98520).regmap, MAX98520_R2000_SW_RESET, 1);
    regcache_sync((*max98520).regmap);
    0
}

static max98520_pm: dev_pm_ops = dev_pm_ops {
    system_sleep: SYSTEM_SLEEP_PM_OPS!(max98520_suspend, max98520_resume),
};

static soc_codec_dev_max98520: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98520_probe),
    controls: max98520_snd_controls.as_ptr(),
    num_controls: max98520_snd_controls.len(),
    dapm_widgets: max98520_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98520_dapm_widgets.len(),
    dapm_routes: max98520_audio_map.as_ptr(),
    num_dapm_routes: max98520_audio_map.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static max98520_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    max_register: MAX98520_R21FF_REVISION_ID,
    reg_defaults: max98520_reg.as_ptr(),
    num_reg_defaults: max98520_reg.len(),
    readable_reg: Some(max98520_readable_register),
    volatile_reg: Some(max98520_volatile_reg),
    cache_type: REGCACHE_RBTREE,
};

unsafe fn max98520_power_on(max98520: *mut max98520_priv, poweron: bool) {
    if !(*max98520).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*max98520).reset_gpio, !poweron);
    }
}

unsafe fn max98520_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let mut ret: i32;
    let mut reg: i32 = 0;
    let max98520: *mut max98520_priv;
    let adapter = to_i2c_adapter((*(*i2c).dev).parent);

    if !i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_BYTE_DATA) {
        dev_err!(&mut (*i2c).dev, "I2C check functionality failed\n");
        return -ENXIO;
    }

    max98520 = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<max98520_priv>(), GFP_KERNEL)
        as *mut max98520_priv;

    if max98520.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, max98520 as *mut core::ffi::c_void);

    /* regmap initialization */
    (*max98520).regmap = devm_regmap_init_i2c(i2c, &max98520_regmap);
    if IS_ERR((*max98520).regmap) {
        ret = PTR_ERR((*max98520).regmap);
        dev_err!(&mut (*i2c).dev, "Failed to allocate regmap: %d\n", ret);
        return ret;
    }

    /* Power on device */
    (*max98520).reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, "reset", GPIOD_OUT_HIGH);
    if !(*max98520).reset_gpio.is_null() {
        if IS_ERR((*max98520).reset_gpio) {
            ret = PTR_ERR((*max98520).reset_gpio);
            dev_err!(&mut (*i2c).dev, "Unable to request GPIO pin: %d.\n", ret);
            return ret;
        }

        max98520_power_on(max98520, true);
    }

    /* Check Revision ID */
    ret = regmap_read((*max98520).regmap, MAX98520_R21FF_REVISION_ID, &mut reg);
    if ret < 0 {
        dev_err!(
            &mut (*i2c).dev,
            "Failed to read: 0x%02X\n",
            MAX98520_R21FF_REVISION_ID
        );
        return ret;
    }
    dev_info!(&mut (*i2c).dev, "MAX98520 revisionID: 0x%02X\n", reg);

    /* codec registration */
    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_codec_dev_max98520,
        max98520_dai.as_mut_ptr(),
        max98520_dai.len(),
    );
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to register codec: %d\n", ret);
    }

    ret
}

static max98520_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: "max98520" },
    i2c_device_id::default(),
];

MODULE_DEVICE_TABLE!(i2c, max98520_i2c_id);

// Original C condition: #if defined(CONFIG_OF)
#[cfg(CONFIG_OF)]
static max98520_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "maxim,max98520" },
    of_device_id::default(),
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, max98520_of_match);

static mut max98520_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "max98520",
        of_match_table: of_match_ptr!(max98520_of_match),
        pm: pm_ptr!(&max98520_pm),
    },
    probe: Some(max98520_i2c_probe),
    id_table: max98520_i2c_id.as_ptr(),
};

module_i2c_driver!(max98520_i2c_driver);

MODULE_DESCRIPTION!("ALSA SoC MAX98520 driver");
MODULE_AUTHOR!("George Song <george.song@maximintegrated.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
