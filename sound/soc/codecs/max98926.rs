// SPDX-License-Identifier: GPL-2.0-only
/*
 * max98926.rs -- ALSA SoC MAX98926 driver
 * Copyright 2013-15 Maxim Integrated Products
 *
 * Translated from max98926.c. Linux, ALSA SoC, regmap, and MAX98926 header
 * definitions are expected to be supplied by the surrounding crate/bindings.
 */

static max98926_boost_voltage_txt: [&str; 16] = [
    "8.5V", "8.25V", "8.0V", "7.75V", "7.5V", "7.25V", "7.0V", "6.75V",
    "6.5V", "6.5V", "6.5V", "6.5V", "6.5V", "6.5V", "6.5V", "6.5V",
];

static max98926_pdm_ch_text: [&str; 2] = [
    "Current", "Voltage",
];

static max98926_hpf_cutoff_txt: [&str; 6] = [
    "Disable", "DC Block", "100Hz",
    "200Hz", "400Hz", "800Hz",
];

static max98926_reg: [reg_default; 48] = [
    reg_default { reg: 0x0B, def: 0x00 }, /* IRQ Enable0 */
    reg_default { reg: 0x0C, def: 0x00 }, /* IRQ Enable1 */
    reg_default { reg: 0x0D, def: 0x00 }, /* IRQ Enable2 */
    reg_default { reg: 0x0E, def: 0x00 }, /* IRQ Clear0 */
    reg_default { reg: 0x0F, def: 0x00 }, /* IRQ Clear1 */
    reg_default { reg: 0x10, def: 0x00 }, /* IRQ Clear2 */
    reg_default { reg: 0x11, def: 0xC0 }, /* Map0 */
    reg_default { reg: 0x12, def: 0x00 }, /* Map1 */
    reg_default { reg: 0x13, def: 0x00 }, /* Map2 */
    reg_default { reg: 0x14, def: 0xF0 }, /* Map3 */
    reg_default { reg: 0x15, def: 0x00 }, /* Map4 */
    reg_default { reg: 0x16, def: 0xAB }, /* Map5 */
    reg_default { reg: 0x17, def: 0x89 }, /* Map6 */
    reg_default { reg: 0x18, def: 0x00 }, /* Map7 */
    reg_default { reg: 0x19, def: 0x00 }, /* Map8 */
    reg_default { reg: 0x1A, def: 0x04 }, /* DAI Clock Mode 1 */
    reg_default { reg: 0x1B, def: 0x00 }, /* DAI Clock Mode 2 */
    reg_default { reg: 0x1C, def: 0x00 }, /* DAI Clock Divider Denominator MSBs */
    reg_default { reg: 0x1D, def: 0x00 }, /* DAI Clock Divider Denominator LSBs */
    reg_default { reg: 0x1E, def: 0xF0 }, /* DAI Clock Divider Numerator MSBs */
    reg_default { reg: 0x1F, def: 0x00 }, /* DAI Clock Divider Numerator LSBs */
    reg_default { reg: 0x20, def: 0x50 }, /* Format */
    reg_default { reg: 0x21, def: 0x00 }, /* TDM Slot Select */
    reg_default { reg: 0x22, def: 0x00 }, /* DOUT Configuration VMON */
    reg_default { reg: 0x23, def: 0x00 }, /* DOUT Configuration IMON */
    reg_default { reg: 0x24, def: 0x00 }, /* DOUT Configuration VBAT */
    reg_default { reg: 0x25, def: 0x00 }, /* DOUT Configuration VBST */
    reg_default { reg: 0x26, def: 0x00 }, /* DOUT Configuration FLAG */
    reg_default { reg: 0x27, def: 0xFF }, /* DOUT HiZ Configuration 1 */
    reg_default { reg: 0x28, def: 0xFF }, /* DOUT HiZ Configuration 2 */
    reg_default { reg: 0x29, def: 0xFF }, /* DOUT HiZ Configuration 3 */
    reg_default { reg: 0x2A, def: 0xFF }, /* DOUT HiZ Configuration 4 */
    reg_default { reg: 0x2B, def: 0x02 }, /* DOUT Drive Strength */
    reg_default { reg: 0x2C, def: 0x90 }, /* Filters */
    reg_default { reg: 0x2D, def: 0x00 }, /* Gain */
    reg_default { reg: 0x2E, def: 0x02 }, /* Gain Ramping */
    reg_default { reg: 0x2F, def: 0x00 }, /* Speaker Amplifier */
    reg_default { reg: 0x30, def: 0x0A }, /* Threshold */
    reg_default { reg: 0x31, def: 0x00 }, /* ALC Attack */
    reg_default { reg: 0x32, def: 0x80 }, /* ALC Atten and Release */
    reg_default { reg: 0x33, def: 0x00 }, /* ALC Infinite Hold Release */
    reg_default { reg: 0x34, def: 0x92 }, /* ALC Configuration */
    reg_default { reg: 0x35, def: 0x01 }, /* Boost Converter */
    reg_default { reg: 0x36, def: 0x00 }, /* Block Enable */
    reg_default { reg: 0x37, def: 0x00 }, /* Configuration */
    reg_default { reg: 0x38, def: 0x00 }, /* Global Enable */
    reg_default { reg: 0x3A, def: 0x00 }, /* Boost Limiter */
];

static max98926_voltage_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE!(MAX98926_DAI_CLK_DIV_N_LSBS, 0,
        ARRAY_SIZE!(max98926_pdm_ch_text),
        max98926_pdm_ch_text),
];

static max98926_voltage_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", max98926_voltage_enum);

static max98926_current_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE!(MAX98926_DAI_CLK_DIV_N_LSBS,
        MAX98926_PDM_SOURCE_1_SHIFT,
        ARRAY_SIZE!(max98926_pdm_ch_text),
        max98926_pdm_ch_text),
];

static max98926_current_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", max98926_current_enum);

static max98926_mixer_controls: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE!("PCM Single Switch", MAX98926_SPK_AMP,
        MAX98926_INSELECT_MODE_SHIFT, 0, 0),
    SOC_DAPM_SINGLE!("PDM Single Switch", MAX98926_SPK_AMP,
        MAX98926_INSELECT_MODE_SHIFT, 1, 0),
];

static max98926_dai_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE!("Left", MAX98926_GAIN,
        MAX98926_DAC_IN_SEL_SHIFT, 0, 0),
    SOC_DAPM_SINGLE!("Right", MAX98926_GAIN,
        MAX98926_DAC_IN_SEL_SHIFT, 1, 0),
    SOC_DAPM_SINGLE!("LeftRight", MAX98926_GAIN,
        MAX98926_DAC_IN_SEL_SHIFT, 2, 0),
    SOC_DAPM_SINGLE!("(Left+Right)/2 Switch", MAX98926_GAIN,
        MAX98926_DAC_IN_SEL_SHIFT, 3, 0),
];

static max98926_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    SND_SOC_DAPM_AIF_IN!("DAI_OUT", "HiFi Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!("Amp Enable", core::ptr::null(), MAX98926_BLOCK_ENABLE, MAX98926_SPK_EN_SHIFT, 0),
    SND_SOC_DAPM_SUPPLY!("Global Enable", MAX98926_GLOBAL_ENABLE, MAX98926_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("VI Enable", MAX98926_BLOCK_ENABLE, MAX98926_ADC_IMON_EN_WIDTH | MAX98926_ADC_VMON_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("BST Enable", MAX98926_BLOCK_ENABLE, MAX98926_BST_EN_SHIFT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_OUTPUT!("BE_OUT"),
    SND_SOC_DAPM_MIXER!("PCM Sel", MAX98926_SPK_AMP, MAX98926_INSELECT_MODE_SHIFT, 0, &max98926_mixer_controls[0], ARRAY_SIZE!(max98926_mixer_controls)),
    SND_SOC_DAPM_MIXER!("DAI Sel", MAX98926_GAIN, MAX98926_DAC_IN_SEL_SHIFT, 0, &max98926_dai_controls[0], ARRAY_SIZE!(max98926_dai_controls)),
    SND_SOC_DAPM_MUX!("PDM CH1 Source", MAX98926_DAI_CLK_DIV_N_LSBS, MAX98926_PDM_CURRENT_SHIFT, 0, &max98926_current_control),
    SND_SOC_DAPM_MUX!("PDM CH0 Source", MAX98926_DAI_CLK_DIV_N_LSBS, MAX98926_PDM_VOLTAGE_SHIFT, 0, &max98926_voltage_control),
];

static max98926_audio_map: [snd_soc_dapm_route; 15] = [
    snd_soc_dapm_route { sink: "VI Enable", control: core::ptr::null(), source: "DAI_OUT" },
    snd_soc_dapm_route { sink: "DAI Sel", control: "Left", source: "VI Enable" },
    snd_soc_dapm_route { sink: "DAI Sel", control: "Right", source: "VI Enable" },
    snd_soc_dapm_route { sink: "DAI Sel", control: "LeftRight", source: "VI Enable" },
    snd_soc_dapm_route { sink: "DAI Sel", control: "LeftRightDiv2", source: "VI Enable" },
    snd_soc_dapm_route { sink: "PCM Sel", control: "PCM", source: "DAI Sel" },
    snd_soc_dapm_route { sink: "PDM CH1 Source", control: "Current", source: "DAI_OUT" },
    snd_soc_dapm_route { sink: "PDM CH1 Source", control: "Voltage", source: "DAI_OUT" },
    snd_soc_dapm_route { sink: "PDM CH0 Source", control: "Current", source: "DAI_OUT" },
    snd_soc_dapm_route { sink: "PDM CH0 Source", control: "Voltage", source: "DAI_OUT" },
    snd_soc_dapm_route { sink: "PCM Sel", control: "Analog", source: "PDM CH1 Source" },
    snd_soc_dapm_route { sink: "PCM Sel", control: "Analog", source: "PDM CH0 Source" },
    snd_soc_dapm_route { sink: "Amp Enable", control: core::ptr::null(), source: "PCM Sel" },
    snd_soc_dapm_route { sink: "BST Enable", control: core::ptr::null(), source: "Amp Enable" },
    snd_soc_dapm_route { sink: "BE_OUT", control: core::ptr::null(), source: "BST Enable" },
];

unsafe extern "C" fn max98926_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98926_VBAT_DATA |
        MAX98926_VBST_DATA |
        MAX98926_LIVE_STATUS0 |
        MAX98926_LIVE_STATUS1 |
        MAX98926_LIVE_STATUS2 |
        MAX98926_STATE0 |
        MAX98926_STATE1 |
        MAX98926_STATE2 |
        MAX98926_FLAG0 |
        MAX98926_FLAG1 |
        MAX98926_FLAG2 |
        MAX98926_VERSION => true,
        _ => false,
    }
}

unsafe extern "C" fn max98926_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98926_IRQ_CLEAR0 |
        MAX98926_IRQ_CLEAR1 |
        MAX98926_IRQ_CLEAR2 |
        MAX98926_ALC_HOLD_RLS => false,
        _ => true,
    }
}

static max98926_spk_tlv: Tlv = DECLARE_TLV_DB_SCALE!(-600, 100, 0);
static max98926_current_tlv: Tlv = DECLARE_TLV_DB_RANGE!(
    0, 11, TLV_DB_SCALE_ITEM!(20, 20, 0),
    12, 15, TLV_DB_SCALE_ITEM!(320, 40, 0),
);

static max98926_dac_hpf_cutoff: soc_enum =
    SOC_ENUM_SINGLE_DECL!(MAX98926_FILTERS, MAX98926_DAC_HPF_SHIFT, max98926_hpf_cutoff_txt);

static max98926_boost_voltage: soc_enum =
    SOC_ENUM_SINGLE_DECL!(MAX98926_CONFIGURATION, MAX98926_BST_VOUT_SHIFT, max98926_boost_voltage_txt);

static max98926_snd_controls: [snd_kcontrol_new; 10] = [
    SOC_SINGLE_TLV!("Speaker Volume", MAX98926_GAIN, MAX98926_SPK_GAIN_SHIFT, (1 << MAX98926_SPK_GAIN_WIDTH) - 1, 0, max98926_spk_tlv),
    SOC_SINGLE!("Ramp Switch", MAX98926_GAIN_RAMPING, MAX98926_SPK_RMP_EN_SHIFT, 1, 0),
    SOC_SINGLE!("ZCD Switch", MAX98926_GAIN_RAMPING, MAX98926_SPK_ZCD_EN_SHIFT, 1, 0),
    SOC_SINGLE!("ALC Switch", MAX98926_THRESHOLD, MAX98926_ALC_EN_SHIFT, 1, 0),
    SOC_SINGLE!("ALC Threshold", MAX98926_THRESHOLD, MAX98926_ALC_TH_SHIFT, (1 << MAX98926_ALC_TH_WIDTH) - 1, 0),
    SOC_ENUM!("Boost Output Voltage", max98926_boost_voltage),
    SOC_SINGLE_TLV!("Boost Current Limit", MAX98926_BOOST_LIMITER, MAX98926_BST_ILIM_SHIFT, (1 << MAX98926_BST_ILIM_SHIFT) - 1, 0, max98926_current_tlv),
    SOC_ENUM!("DAC HPF Cutoff", max98926_dac_hpf_cutoff),
    SOC_DOUBLE!("PDM Channel One", MAX98926_DAI_CLK_DIV_N_LSBS, MAX98926_PDM_CHANNEL_1_SHIFT, MAX98926_PDM_CHANNEL_1_HIZ, 1, 0),
    SOC_DOUBLE!("PDM Channel Zero", MAX98926_DAI_CLK_DIV_N_LSBS, MAX98926_PDM_CHANNEL_0_SHIFT, MAX98926_PDM_CHANNEL_0_HIZ, 1, 0),
];

#[repr(C)]
struct max98926_rate {
    rate: c_int,
    sr: c_int,
}

static rate_table: [max98926_rate; 9] = [
    max98926_rate { rate: 8000, sr: 0 },
    max98926_rate { rate: 11025, sr: 1 },
    max98926_rate { rate: 12000, sr: 2 },
    max98926_rate { rate: 16000, sr: 3 },
    max98926_rate { rate: 22050, sr: 4 },
    max98926_rate { rate: 24000, sr: 5 },
    max98926_rate { rate: 32000, sr: 6 },
    max98926_rate { rate: 44100, sr: 7 },
    max98926_rate { rate: 48000, sr: 8 },
];

unsafe extern "C" fn max98926_set_sense_data(max98926: *mut max98926_priv) {
    regmap_update_bits((*max98926).regmap, MAX98926_DOUT_CFG_VMON, MAX98926_DAI_VMON_EN_MASK, MAX98926_DAI_VMON_EN_MASK);
    regmap_update_bits((*max98926).regmap, MAX98926_DOUT_CFG_IMON, MAX98926_DAI_IMON_EN_MASK, MAX98926_DAI_IMON_EN_MASK);

    if !(*max98926).interleave_mode {
        /* set VMON slots */
        regmap_update_bits((*max98926).regmap, MAX98926_DOUT_CFG_VMON, MAX98926_DAI_VMON_SLOT_MASK, (*max98926).v_slot);
        /* set IMON slots */
        regmap_update_bits((*max98926).regmap, MAX98926_DOUT_CFG_IMON, MAX98926_DAI_IMON_SLOT_MASK, (*max98926).i_slot);
    } else {
        /* enable interleave mode */
        regmap_update_bits((*max98926).regmap, MAX98926_FORMAT, MAX98926_DAI_INTERLEAVE_MASK, MAX98926_DAI_INTERLEAVE_MASK);
        /* set interleave slots */
        regmap_update_bits((*max98926).regmap, MAX98926_DOUT_CFG_VBAT, MAX98926_DAI_INTERLEAVE_SLOT_MASK, (*max98926).v_slot);
    }
}

unsafe extern "C" fn max98926_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let max98926: *mut max98926_priv = snd_soc_component_get_drvdata(component) as *mut max98926_priv;
    let mut invert: c_uint = 0;

    dev_dbg!((*component).dev, "%s: fmt 0x%08X\n", __func__!(), fmt);

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            max98926_set_sense_data(max98926);
        }
        _ => {
            dev_err!((*component).dev, "DAI clock mode unsupported\n");
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => {
            invert = MAX98926_DAI_WCI_MASK;
        }
        SND_SOC_DAIFMT_IB_NF => {
            invert = MAX98926_DAI_BCI_MASK;
        }
        SND_SOC_DAIFMT_IB_IF => {
            invert = MAX98926_DAI_BCI_MASK | MAX98926_DAI_WCI_MASK;
        }
        _ => {
            dev_err!((*component).dev, "DAI invert mode unsupported\n");
            return -EINVAL;
        }
    }

    regmap_write((*max98926).regmap, MAX98926_FORMAT, MAX98926_DAI_DLY_MASK);
    regmap_update_bits((*max98926).regmap, MAX98926_FORMAT,
        MAX98926_DAI_BCI_MASK | MAX98926_DAI_WCI_MASK, invert);
    0
}

unsafe extern "C" fn max98926_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut dai_sr: c_int = -EINVAL;
    let rate: c_int = params_rate(params);
    let mut i: usize;
    let component: *mut snd_soc_component = (*dai).component;
    let max98926: *mut max98926_priv = snd_soc_component_get_drvdata(component) as *mut max98926_priv;
    let blr_clk_ratio: c_int;

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            regmap_update_bits((*max98926).regmap, MAX98926_FORMAT, MAX98926_DAI_CHANSZ_MASK, MAX98926_DAI_CHANSZ_16);
            (*max98926).ch_size = 16;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            regmap_update_bits((*max98926).regmap, MAX98926_FORMAT, MAX98926_DAI_CHANSZ_MASK, MAX98926_DAI_CHANSZ_24);
            (*max98926).ch_size = 24;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            regmap_update_bits((*max98926).regmap, MAX98926_FORMAT, MAX98926_DAI_CHANSZ_MASK, MAX98926_DAI_CHANSZ_32);
            (*max98926).ch_size = 32;
        }
        _ => {
            dev_dbg!((*component).dev, "format unsupported %d\n", params_format(params));
            return -EINVAL;
        }
    }

    /* BCLK/LRCLK ratio calculation */
    blr_clk_ratio = params_channels(params) * (*max98926).ch_size;

    match blr_clk_ratio {
        32 => {
            regmap_update_bits((*max98926).regmap, MAX98926_DAI_CLK_MODE2, MAX98926_DAI_BSEL_MASK, MAX98926_DAI_BSEL_32);
        }
        48 => {
            regmap_update_bits((*max98926).regmap, MAX98926_DAI_CLK_MODE2, MAX98926_DAI_BSEL_MASK, MAX98926_DAI_BSEL_48);
        }
        64 => {
            regmap_update_bits((*max98926).regmap, MAX98926_DAI_CLK_MODE2, MAX98926_DAI_BSEL_MASK, MAX98926_DAI_BSEL_64);
        }
        _ => {
            return -EINVAL;
        }
    }

    /* find the closest rate */
    i = 0;
    while i < ARRAY_SIZE!(rate_table) {
        if rate_table[i].rate >= rate {
            dai_sr = rate_table[i].sr;
            break;
        }
        i += 1;
    }
    if dai_sr < 0 {
        return -EINVAL;
    }

    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits((*max98926).regmap, MAX98926_DAI_CLK_MODE2,
        MAX98926_DAI_SR_MASK, (dai_sr << MAX98926_DAI_SR_SHIFT) as c_uint);
    0
}

const MAX98926_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static max98926_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(max98926_dai_set_fmt),
    hw_params: Some(max98926_dai_hw_params),
};

static mut max98926_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: "max98926-aif1",
        playback: snd_soc_pcm_stream {
            stream_name: "HiFi Playback",
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: MAX98926_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "HiFi Capture",
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: MAX98926_FORMATS,
        },
        ops: &max98926_dai_ops,
    },
];

unsafe extern "C" fn max98926_probe(component: *mut snd_soc_component) -> c_int {
    let max98926: *mut max98926_priv = snd_soc_component_get_drvdata(component) as *mut max98926_priv;

    (*max98926).component = component;

    /* Hi-Z all the slots */
    regmap_write((*max98926).regmap, MAX98926_DOUT_HIZ_CFG4, 0xF0);
    0
}

static soc_component_dev_max98926: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98926_probe),
    controls: max98926_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(max98926_snd_controls),
    dapm_routes: max98926_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(max98926_audio_map),
    dapm_widgets: max98926_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(max98926_dapm_widgets),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static max98926_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: MAX98926_VERSION,
    reg_defaults: max98926_reg.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(max98926_reg),
    volatile_reg: Some(max98926_volatile_register),
    readable_reg: Some(max98926_readable_register),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn max98926_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int;
    let mut reg: c_int = 0;
    let mut value: u32 = 0;
    let max98926: *mut max98926_priv;

    max98926 = devm_kzalloc(&mut (*i2c).dev,
        core::mem::size_of::<max98926_priv>(), GFP_KERNEL) as *mut max98926_priv;
    if max98926.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, max98926 as *mut c_void);
    (*max98926).regmap = devm_regmap_init_i2c(i2c, &max98926_regmap);
    if IS_ERR((*max98926).regmap as *const c_void) {
        ret = PTR_ERR((*max98926).regmap as *const c_void) as c_int;
        dev_err!(&mut (*i2c).dev, "Failed to allocate regmap: %d\n", ret);
        goto_err_out!(ret);
    }
    if of_property_read_bool((*i2c).dev.of_node, "maxim,interleave-mode") ||
        of_property_read_bool((*i2c).dev.of_node, "interleave-mode") {
        (*max98926).interleave_mode = true;
    }

    if of_property_read_u32((*i2c).dev.of_node, "vmon-slot-no", &mut value) == 0 {
        if value > MAX98926_DAI_VMON_SLOT_1E_1F {
            dev_err!(&mut (*i2c).dev, "vmon slot number is wrong:\n");
            return -EINVAL;
        }
        (*max98926).v_slot = value;
    }
    if of_property_read_u32((*i2c).dev.of_node, "imon-slot-no", &mut value) == 0 {
        if value > MAX98926_DAI_IMON_SLOT_1E_1F {
            dev_err!(&mut (*i2c).dev, "imon slot number is wrong:\n");
            return -EINVAL;
        }
        (*max98926).i_slot = value;
    }
    ret = regmap_read((*max98926).regmap, MAX98926_VERSION, &mut reg);
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to read: %x\n", reg);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*i2c).dev,
        &soc_component_dev_max98926,
        max98926_dai.as_mut_ptr(), ARRAY_SIZE!(max98926_dai));
    if ret < 0 {
        dev_err!(&mut (*i2c).dev, "Failed to register component: %d\n", ret);
    }
    dev_info!(&mut (*i2c).dev, "device version: %x\n", reg);
    ret
}

static max98926_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: "max98926" },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, max98926_i2c_id);

/* CONFIG_OF: Open Firmware device id table. */
static max98926_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "maxim,max98926" },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, max98926_of_match);

static mut max98926_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "max98926",
        of_match_table: of_match_ptr!(max98926_of_match),
    },
    probe: Some(max98926_i2c_probe),
    id_table: max98926_i2c_id.as_ptr(),
};

module_i2c_driver!(max98926_i2c_driver);
MODULE_DESCRIPTION!("ALSA SoC MAX98926 driver");
MODULE_AUTHOR!("Anish kumar <anish.kumar@maximintegrated.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
