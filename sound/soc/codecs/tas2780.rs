// SPDX-License-Identifier: GPL-2.0
// Driver for the Texas Instruments TAS2780 Mono
//		Audio amplifier
// Copyright (C) 2022 Texas Instruments Inc.

// Translated from the original C implementation source. Linux, ALSA SoC,
// regmap, GPIO, I2C, and TAS2780 header symbols are expected dependencies.

#[repr(C)]
pub struct tas2780_priv {
    pub component: *mut snd_soc_component,
    pub reset_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub v_sense_slot: core::ffi::c_int,
    pub i_sense_slot: core::ffi::c_int,
}

unsafe fn tas2780_reset(tas2780: *mut tas2780_priv) {
    let mut ret: core::ffi::c_int = 0;

    if !(*tas2780).reset_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2780).reset_gpio, 0);
        usleep_range(2000, 2050);
        gpiod_set_value_cansleep((*tas2780).reset_gpio, 1);
        usleep_range(2000, 2050);
    }

    ret = snd_soc_component_write((*tas2780).component, TAS2780_SW_RST, TAS2780_RST);
    if ret != 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Reset error!\n".as_ptr(),
            c"tas2780_reset".as_ptr(),
            ret,
        );
    }
}

// Original C conditional: #ifdef CONFIG_PM
unsafe fn tas2780_codec_suspend(component: *mut snd_soc_component) -> core::ffi::c_int {
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut ret: core::ffi::c_int = 0;

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_PWR_CTRL,
        TAS2780_PWR_CTRL_MASK,
        TAS2780_PWR_CTRL_SHUTDOWN,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%0x:power down error\n".as_ptr(),
            c"tas2780_codec_suspend".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = 0;
    regcache_cache_only((*tas2780).regmap, true);
    regcache_mark_dirty((*tas2780).regmap);
    ret
}

unsafe fn tas2780_codec_resume(component: *mut snd_soc_component) -> core::ffi::c_int {
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut ret: core::ffi::c_int;

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_PWR_CTRL,
        TAS2780_PWR_CTRL_MASK,
        TAS2780_PWR_CTRL_ACTIVE,
    );

    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%0x:power down error\n".as_ptr(),
            c"tas2780_codec_resume".as_ptr(),
            ret,
        );
        return ret;
    }
    regcache_cache_only((*tas2780).regmap, false);
    ret = regcache_sync((*tas2780).regmap);
    ret
}

static tas2780_ASI1_src: [*const core::ffi::c_char; 4] = [
    c"I2C offset".as_ptr(),
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"LeftRightDiv2".as_ptr(),
];

static tas2780_ASI1_src_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(TAS2780_TDM_CFG2, 4, tas2780_ASI1_src);

static tas2780_asi1_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"ASI1 Source".as_ptr(), tas2780_ASI1_src_enum);

static isense_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!(
    c"Switch".as_ptr(),
    TAS2780_PWR_CTRL,
    TAS2780_ISENSE_POWER_EN,
    1,
    1
);
static vsense_switch: snd_kcontrol_new = SOC_DAPM_SINGLE!(
    c"Switch".as_ptr(),
    TAS2780_PWR_CTRL,
    TAS2780_VSENSE_POWER_EN,
    1,
    1
);

static tas2780_dapm_widgets: [snd_soc_dapm_widget; 7] = [
    SND_SOC_DAPM_AIF_IN!(c"ASI1".as_ptr(), c"ASI1 Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!(c"ASI1 Sel".as_ptr(), SND_SOC_NOPM, 0, 0, &tas2780_asi1_mux),
    SND_SOC_DAPM_SWITCH!(c"ISENSE".as_ptr(), TAS2780_PWR_CTRL, TAS2780_ISENSE_POWER_EN, 1, &isense_switch),
    SND_SOC_DAPM_SWITCH!(c"VSENSE".as_ptr(), TAS2780_PWR_CTRL, TAS2780_VSENSE_POWER_EN, 1, &vsense_switch),
    SND_SOC_DAPM_OUTPUT!(c"OUT".as_ptr()),
    SND_SOC_DAPM_SIGGEN!(c"VMON".as_ptr()),
    SND_SOC_DAPM_SIGGEN!(c"IMON".as_ptr()),
];

static tas2780_audio_map: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"I2C offset".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"Left".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"Right".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"ASI1 Sel".as_ptr(), control: c"LeftRightDiv2".as_ptr(), source: c"ASI1".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: core::ptr::null(), source: c"ASI1 Sel".as_ptr() },
    snd_soc_dapm_route { sink: c"ISENSE".as_ptr(), control: c"Switch".as_ptr(), source: c"IMON".as_ptr() },
    snd_soc_dapm_route { sink: c"VSENSE".as_ptr(), control: c"Switch".as_ptr(), source: c"VMON".as_ptr() },
];

unsafe fn tas2780_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    _direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut ret: core::ffi::c_int = 0;

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_PWR_CTRL,
        TAS2780_PWR_CTRL_MASK,
        if mute != 0 { TAS2780_PWR_CTRL_MUTE } else { 0 },
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s: Failed to set powercontrol\n".as_ptr(),
            c"tas2780_mute".as_ptr(),
        );
        return ret;
    }
    ret = 0;
    ret
}

unsafe fn tas2780_set_bitwidth(
    tas2780: *mut tas2780_priv,
    bitwidth: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*tas2780).component;
    let mut sense_en: core::ffi::c_int;
    let mut val: core::ffi::c_int;
    let mut ret: core::ffi::c_int;
    let slot_size: core::ffi::c_int;

    match bitwidth {
        SNDRV_PCM_FORMAT_S16_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2780_TDM_CFG2,
                TAS2780_TDM_CFG2_RXW_MASK,
                TAS2780_TDM_CFG2_RXW_16BITS,
            );
            slot_size = TAS2780_TDM_CFG2_RXS_16BITS;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2780_TDM_CFG2,
                TAS2780_TDM_CFG2_RXW_MASK,
                TAS2780_TDM_CFG2_RXW_24BITS,
            );
            slot_size = TAS2780_TDM_CFG2_RXS_24BITS;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2780_TDM_CFG2,
                TAS2780_TDM_CFG2_RXW_MASK,
                TAS2780_TDM_CFG2_RXW_32BITS,
            );
            slot_size = TAS2780_TDM_CFG2_RXS_32BITS;
        }
        _ => {
            ret = -EINVAL;
            slot_size = 0;
        }
    }

    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x set bitwidth error\n".as_ptr(),
            c"tas2780_set_bitwidth".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG2,
        TAS2780_TDM_CFG2_RXS_MASK,
        slot_size,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x set RX slot size error\n".as_ptr(),
            c"tas2780_set_bitwidth".as_ptr(),
            ret,
        );
        return ret;
    }

    val = snd_soc_component_read((*tas2780).component, TAS2780_PWR_CTRL);
    if val < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x read PWR_CTRL error\n".as_ptr(),
            c"tas2780_set_bitwidth".as_ptr(),
            val,
        );
        ret = val;
        return ret;
    }

    if (val & (1 << TAS2780_VSENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2780_TDM_CFG5_VSNS_ENABLE;
    }

    ret = snd_soc_component_update_bits(
        (*tas2780).component,
        TAS2780_TDM_CFG5,
        TAS2780_TDM_CFG5_VSNS_ENABLE,
        sense_en,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x enable vSNS error\n".as_ptr(),
            c"tas2780_set_bitwidth".as_ptr(),
            ret,
        );
        return ret;
    }

    if (val & (1 << TAS2780_ISENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2780_TDM_CFG6_ISNS_ENABLE;
    }

    ret = snd_soc_component_update_bits(
        (*tas2780).component,
        TAS2780_TDM_CFG6,
        TAS2780_TDM_CFG6_ISNS_ENABLE,
        sense_en,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x enable iSNS error\n".as_ptr(),
            c"tas2780_set_bitwidth".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = 0;
    ret
}

unsafe fn tas2780_set_samplerate(
    tas2780: *mut tas2780_priv,
    samplerate: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*tas2780).component;
    let ramp_rate_val: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    match samplerate {
        48000 => {
            ramp_rate_val = TAS2780_TDM_CFG0_SMP_48KHZ | TAS2780_TDM_CFG0_44_1_48KHZ;
        }
        44100 => {
            ramp_rate_val = TAS2780_TDM_CFG0_SMP_44_1KHZ | TAS2780_TDM_CFG0_44_1_48KHZ;
        }
        96000 => {
            ramp_rate_val = TAS2780_TDM_CFG0_SMP_48KHZ | TAS2780_TDM_CFG0_88_2_96KHZ;
        }
        88200 => {
            ramp_rate_val = TAS2780_TDM_CFG0_SMP_44_1KHZ | TAS2780_TDM_CFG0_88_2_96KHZ;
        }
        _ => return -EINVAL,
    }
    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG0,
        TAS2780_TDM_CFG0_SMP_MASK | TAS2780_TDM_CFG0_MASK,
        ramp_rate_val,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set ramp_rate_val\n".as_ptr(),
            c"tas2780_set_samplerate".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = 0;
    ret
}

unsafe fn tas2780_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut ret: core::ffi::c_int;

    ret = tas2780_set_bitwidth(tas2780, params_format(params));
    if ret < 0 {
        return ret;
    }

    tas2780_set_samplerate(tas2780, params_rate(params))
}

unsafe fn tas2780_set_fmt(dai: *mut snd_soc_dai, fmt: core::ffi::c_uint) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut tdm_rx_start_slot: u8 = 0;
    let mut asi_cfg_1: u8 = 0;
    let iface: core::ffi::c_int;
    let mut ret: core::ffi::c_int = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            asi_cfg_1 = TAS2780_TDM_CFG1_RX_RISING as u8;
        }
        SND_SOC_DAIFMT_IB_NF => {
            asi_cfg_1 = TAS2780_TDM_CFG1_RX_FALLING as u8;
        }
        _ => {
            dev_err((*tas2780).dev, c"ASI format Inverse is not found\n".as_ptr());
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG1,
        TAS2780_TDM_CFG1_RX_MASK,
        asi_cfg_1 as core::ffi::c_int,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set asi_cfg_1\n".as_ptr(),
            c"tas2780_set_fmt".as_ptr(),
            ret,
        );
        return ret;
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_DSP_A => {
            iface = TAS2780_TDM_CFG2_SCFG_I2S;
            tdm_rx_start_slot = 1;
        }
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_B => {
            iface = TAS2780_TDM_CFG2_SCFG_LEFT_J;
            tdm_rx_start_slot = 0;
        }
        _ => {
            dev_err(
                (*tas2780).dev,
                c"%s:DAI Format is not found, fmt=0x%x\n".as_ptr(),
                c"tas2780_set_fmt".as_ptr(),
                fmt,
            );
            ret = -EINVAL;
            return ret;
        }
    }
    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG1,
        TAS2780_TDM_CFG1_MASK,
        (tdm_rx_start_slot as core::ffi::c_int) << TAS2780_TDM_CFG1_51_SHIFT,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set tdm_rx_start_slot\n".as_ptr(),
            c"tas2780_set_fmt".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG2,
        TAS2780_TDM_CFG2_SCFG_MASK,
        iface,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set iface\n".as_ptr(),
            c"tas2780_set_fmt".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = 0;
    ret
}

unsafe fn tas2780_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: core::ffi::c_uint,
    rx_mask: core::ffi::c_uint,
    slots: core::ffi::c_int,
    slot_width: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let left_slot: core::ffi::c_int;
    let right_slot: core::ffi::c_int;
    let slots_cfg: core::ffi::c_int;
    let slot_size: core::ffi::c_int;
    let mut ret: core::ffi::c_int = 0;

    if tx_mask == 0 || rx_mask != 0 {
        return -EINVAL;
    }

    left_slot = __ffs(tx_mask as core::ffi::c_ulong) as core::ffi::c_int;
    tx_mask &= !(1 << left_slot);
    if tx_mask == 0 {
        right_slot = left_slot;
    } else {
        right_slot = __ffs(tx_mask as core::ffi::c_ulong) as core::ffi::c_int;
        tx_mask &= !(1 << right_slot);
    }

    if tx_mask != 0 || left_slot >= slots || right_slot >= slots {
        return -EINVAL;
    }

    slots_cfg = (right_slot << TAS2780_TDM_CFG3_RXS_SHIFT) | left_slot;
    ret = snd_soc_component_write(component, TAS2780_TDM_CFG3, slots_cfg);
    if ret != 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set slots_cfg\n".as_ptr(),
            c"tas2780_set_dai_tdm_slot".as_ptr(),
            ret,
        );
        return ret;
    }

    match slot_width {
        16 => {
            slot_size = TAS2780_TDM_CFG2_RXS_16BITS;
        }
        24 => {
            slot_size = TAS2780_TDM_CFG2_RXS_24BITS;
        }
        32 => {
            slot_size = TAS2780_TDM_CFG2_RXS_32BITS;
        }
        _ => {
            ret = -EINVAL;
            return ret;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG2,
        TAS2780_TDM_CFG2_RXS_MASK,
        slot_size,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set slot_size\n".as_ptr(),
            c"tas2780_set_dai_tdm_slot".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG5,
        TAS2780_TDM_CFG5_50_MASK,
        (*tas2780).v_sense_slot,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set v_sense_slot\n".as_ptr(),
            c"tas2780_set_dai_tdm_slot".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2780_TDM_CFG6,
        TAS2780_TDM_CFG6_50_MASK,
        (*tas2780).i_sense_slot,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%x Failed to set i_sense_slot\n".as_ptr(),
            c"tas2780_set_dai_tdm_slot".as_ptr(),
            ret,
        );
        return ret;
    }
    ret = 0;
    ret
}

static tas2780_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(tas2780_mute),
    hw_params: Some(tas2780_hw_params),
    set_fmt: Some(tas2780_set_fmt),
    set_tdm_slot: Some(tas2780_set_dai_tdm_slot),
    no_capture_mute: 1,
};

const TAS2780_FORMATS: core::ffi::c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

const TAS2780_RATES: core::ffi::c_uint =
    SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_88200;

static mut tas2780_dai_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"tas2780 ASI1".as_ptr(),
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: c"ASI1 Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: TAS2780_RATES,
        formats: TAS2780_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"ASI1 Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: TAS2780_RATES,
        formats: TAS2780_FORMATS,
    },
    ops: &tas2780_dai_ops,
    symmetric_rate: 1,
}];

unsafe fn tas2780_codec_probe(component: *mut snd_soc_component) -> core::ffi::c_int {
    let tas2780: *mut tas2780_priv = snd_soc_component_get_drvdata(component) as *mut tas2780_priv;
    let mut ret: core::ffi::c_int = 0;

    (*tas2780).component = component;

    tas2780_reset(tas2780);
    ret = snd_soc_component_update_bits(
        component,
        TAS2780_IC_CFG,
        TAS2780_IC_CFG_MASK,
        TAS2780_IC_CFG_ENABLE,
    );
    if ret < 0 {
        dev_err(
            (*tas2780).dev,
            c"%s:errCode:0x%0x\n".as_ptr(),
            c"tas2780_codec_probe".as_ptr(),
            ret,
        );
    }

    ret
}

static tas2780_digital_tlv: [core::ffi::c_uint; TLV_DB_SCALE_ITEM_COUNT] =
    DECLARE_TLV_DB_SCALE!(1100, 50, 0);
static tas2780_playback_volume: [core::ffi::c_uint; TLV_DB_SCALE_ITEM_COUNT] =
    DECLARE_TLV_DB_SCALE!(-10000, 50, 0);

static tas2780_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE_TLV!(
        c"Speaker Volume".as_ptr(),
        TAS2780_DVC,
        0,
        TAS2780_DVC_MAX,
        1,
        tas2780_playback_volume
    ),
    SOC_SINGLE_TLV!(
        c"Amp Gain Volume".as_ptr(),
        TAS2780_CHNL_0,
        0,
        0x14,
        0,
        tas2780_digital_tlv
    ),
];

static soc_component_driver_tas2780: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(tas2780_codec_probe),
    // Original C conditional: these fields are present under #ifdef CONFIG_PM.
    suspend: Some(tas2780_codec_suspend),
    resume: Some(tas2780_codec_resume),
    controls: tas2780_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(tas2780_snd_controls),
    dapm_widgets: tas2780_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(tas2780_dapm_widgets),
    dapm_routes: tas2780_audio_map.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(tas2780_audio_map),
    idle_bias_on: 1,
    endianness: 1,
};

static tas2780_reg_defaults: [reg_default; 10] = [
    reg_default { reg: TAS2780_PAGE, def: 0x00 },
    reg_default { reg: TAS2780_SW_RST, def: 0x00 },
    reg_default { reg: TAS2780_PWR_CTRL, def: 0x1a },
    reg_default { reg: TAS2780_CHNL_0, def: 0x00 },
    reg_default { reg: TAS2780_TDM_CFG0, def: 0x09 },
    reg_default { reg: TAS2780_TDM_CFG1, def: 0x02 },
    reg_default { reg: TAS2780_TDM_CFG2, def: 0x0a },
    reg_default { reg: TAS2780_TDM_CFG3, def: 0x10 },
    reg_default { reg: TAS2780_TDM_CFG5, def: 0x42 },
    reg_default { reg: TAS2780_DVC, def: 0x00 },
];

static tas2780_regmap_ranges: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 1 * 128,
    selector_reg: TAS2780_PAGE,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];

static tas2780_i2c_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    reg_defaults: tas2780_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(tas2780_reg_defaults),
    cache_type: REGCACHE_RBTREE,
    ranges: tas2780_regmap_ranges.as_ptr(),
    num_ranges: ARRAY_SIZE!(tas2780_regmap_ranges),
    max_register: 1 * 128,
};

unsafe fn tas2780_parse_dt(
    dev: *mut device,
    tas2780: *mut tas2780_priv,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int = 0;

    (*tas2780).reset_gpio = devm_gpiod_get_optional((*tas2780).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas2780).reset_gpio as *const core::ffi::c_void) {
        if PTR_ERR((*tas2780).reset_gpio as *const core::ffi::c_void) == -EPROBE_DEFER {
            (*tas2780).reset_gpio = core::ptr::null_mut();
            return -EPROBE_DEFER;
        }
    }

    ret = fwnode_property_read_u32(
        (*dev).fwnode,
        c"ti,imon-slot-no".as_ptr(),
        &mut (*tas2780).i_sense_slot as *mut core::ffi::c_int as *mut u32,
    );
    if ret != 0 {
        (*tas2780).i_sense_slot = 0;
    }

    ret = fwnode_property_read_u32(
        (*dev).fwnode,
        c"ti,vmon-slot-no".as_ptr(),
        &mut (*tas2780).v_sense_slot as *mut core::ffi::c_int as *mut u32,
    );
    if ret != 0 {
        (*tas2780).v_sense_slot = 2;
    }

    0
}

unsafe fn tas2780_i2c_probe(client: *mut i2c_client) -> core::ffi::c_int {
    let tas2780: *mut tas2780_priv;
    let mut result: core::ffi::c_int;

    tas2780 = devm_kzalloc(
        &mut (*client).dev,
        core::mem::size_of::<tas2780_priv>(),
        GFP_KERNEL,
    ) as *mut tas2780_priv;
    if tas2780.is_null() {
        return -ENOMEM;
    }
    (*tas2780).dev = &mut (*client).dev;
    i2c_set_clientdata(client, tas2780 as *mut core::ffi::c_void);
    dev_set_drvdata(&mut (*client).dev, tas2780 as *mut core::ffi::c_void);

    (*tas2780).regmap = devm_regmap_init_i2c(client, &tas2780_i2c_regmap);
    if IS_ERR((*tas2780).regmap as *const core::ffi::c_void) {
        result = PTR_ERR((*tas2780).regmap as *const core::ffi::c_void) as core::ffi::c_int;
        dev_err(
            &mut (*client).dev,
            c"Failed to allocate register map: %d\n".as_ptr(),
            result,
        );
        return result;
    }

    if !(*client).dev.of_node.is_null() {
        result = tas2780_parse_dt(&mut (*client).dev, tas2780);
        if result != 0 {
            dev_err(
                (*tas2780).dev,
                c"%s: Failed to parse devicetree\n".as_ptr(),
                c"tas2780_i2c_probe".as_ptr(),
            );
            return result;
        }
    }

    devm_snd_soc_register_component(
        (*tas2780).dev,
        &soc_component_driver_tas2780,
        tas2780_dai_driver.as_mut_ptr(),
        ARRAY_SIZE!(tas2780_dai_driver),
    )
}

static tas2780_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"tas2780\0", driver_data: 0 },
    i2c_device_id { name: [0; I2C_NAME_SIZE], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, tas2780_i2c_id);

// Original C conditional: #if defined(CONFIG_OF)
static tas2780_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"ti,tas2780".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
MODULE_DEVICE_TABLE!(of, tas2780_of_match);

static mut tas2780_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tas2780".as_ptr(),
        of_match_table: of_match_ptr!(tas2780_of_match),
    },
    probe: Some(tas2780_i2c_probe),
    id_table: tas2780_i2c_id.as_ptr(),
};
module_i2c_driver!(tas2780_i2c_driver);

MODULE_AUTHOR!(c"Raphael Xu <raphael-xu@ti.com>".as_ptr());
MODULE_DESCRIPTION!(c"TAS2780 I2C Smart Amplifier driver".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
