// SPDX-License-Identifier: GPL-2.0
//
// Driver for the Texas Instruments TAS2562 CODEC
// Copyright (C) 2019 Texas Instruments Inc.

// C dependencies removed from executable Rust:
// linux/module.h, linux/errno.h, linux/device.h, linux/i2c.h, linux/regmap.h,
// linux/slab.h, linux/gpio/consumer.h, linux/regulator/consumer.h,
// linux/delay.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dapm.h, sound/tlv.h, and "tas2562.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const TAS2562_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FORMAT_S32_LE;

/* DVC equation involves floating point math
 * round(10^(volume in dB/20)*2^30)
 * so create a lookup table for 2dB step
 */
static FLOAT_VOL_DB_LOOKUP: [c_uint; 56] = [
    0x00000d43, 0x000010b2, 0x00001505, 0x00001a67, 0x00002151,
    0x000029f1, 0x000034cd, 0x00004279, 0x000053af, 0x0000695b,
    0x000084a3, 0x0000a6fa, 0x0000d236, 0x000108a4, 0x00014d2a,
    0x0001a36e, 0x00021008, 0x000298c0, 0x000344df, 0x00041d8f,
    0x00052e5a, 0x000685c8, 0x00083621, 0x000a566d, 0x000d03a7,
    0x0010624d, 0x0014a050, 0x0019f786, 0x0020b0bc, 0x0029279d,
    0x0033cf8d, 0x004139d3, 0x00521d50, 0x00676044, 0x0082248a,
    0x00a3d70a, 0x00ce4328, 0x0103ab3d, 0x0146e75d, 0x019b8c27,
    0x02061b89, 0x028c423f, 0x03352529, 0x0409c2b0, 0x05156d68,
    0x06666666, 0x080e9f96, 0x0a24b062, 0x0cc509ab, 0x10137987,
    0x143d1362, 0x197a967f, 0x2013739e, 0x28619ae9, 0x32d64617,
    0x40000000,
];

#[repr(C)]
pub struct tas2562_data {
    component: *mut snd_soc_component,
    sdz_gpio: *mut gpio_desc,
    regmap: *mut regmap,
    dev: *mut device,
    client: *mut i2c_client,
    v_sense_slot: c_int,
    i_sense_slot: c_int,
    volume_lvl: c_int,
    model_id: c_int,
    dac_powered: bool,
    unmuted: bool,
}

#[repr(C)]
pub enum tas256x_model {
    TAS2562,
    TAS2564,
    TAS2110,
}

unsafe fn tas2562_set_samplerate(tas2562: *mut tas2562_data, samplerate: c_int) -> c_int {
    let samp_rate: c_int;
    let ramp_rate: c_int;

    match samplerate {
        7350 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_7305_8KHZ;
        }
        8000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_7305_8KHZ;
        }
        14700 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_14_7_16KHZ;
        }
        16000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_14_7_16KHZ;
        }
        22050 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_22_05_24KHZ;
        }
        24000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_22_05_24KHZ;
        }
        29400 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_29_4_32KHZ;
        }
        32000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_29_4_32KHZ;
        }
        44100 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_44_1_48KHZ;
        }
        48000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_44_1_48KHZ;
        }
        88200 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_88_2_96KHZ;
        }
        96000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_88_2_96KHZ;
        }
        176400 => {
            ramp_rate = TAS2562_TDM_CFG0_RAMPRATE_44_1;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_176_4_192KHZ;
        }
        192000 => {
            ramp_rate = 0;
            samp_rate = TAS2562_TDM_CFG0_SAMPRATE_176_4_192KHZ;
        }
        _ => {
            dev_info(
                (*tas2562).dev,
                c"%s, unsupported sample rate, %d\n".as_ptr(),
                c"tas2562_set_samplerate".as_ptr(),
                samplerate,
            );
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(
        (*tas2562).component,
        TAS2562_TDM_CFG0,
        TAS2562_TDM_CFG0_RAMPRATE_MASK,
        ramp_rate,
    );
    snd_soc_component_update_bits(
        (*tas2562).component,
        TAS2562_TDM_CFG0,
        TAS2562_TDM_CFG0_SAMPRATE_MASK,
        samp_rate,
    );

    0
}

unsafe fn tas2562_set_dai_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    _rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;
    let left_slot: c_int;
    let right_slot: c_int;
    let slots_cfg: c_int;
    let mut ret: c_int;

    if tx_mask == 0 {
        dev_err((*component).dev, c"tx masks must not be 0\n".as_ptr());
        return -EINVAL;
    }

    if slots == 1 {
        if tx_mask != 1 {
            return -EINVAL;
        }

        left_slot = 0;
        right_slot = 0;
    } else {
        left_slot = __ffs(tx_mask as c_ulong) as c_int;
        tx_mask &= !(1u32 << left_slot);
        if tx_mask == 0 {
            right_slot = left_slot;
        } else {
            right_slot = __ffs(tx_mask as c_ulong) as c_int;
        }
    }

    slots_cfg = (right_slot << TAS2562_RIGHT_SLOT_SHIFT) | left_slot;

    ret = snd_soc_component_write(component, TAS2562_TDM_CFG3, slots_cfg);
    if ret < 0 {
        return ret;
    }

    match slot_width {
        16 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXLEN_MASK,
                TAS2562_TDM_CFG2_RXLEN_16B,
            );
        }
        24 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXLEN_MASK,
                TAS2562_TDM_CFG2_RXLEN_24B,
            );
        }
        32 => {
            ret = snd_soc_component_update_bits(
                component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXLEN_MASK,
                TAS2562_TDM_CFG2_RXLEN_32B,
            );
        }
        0 => {
            /* Do not change slot width */
        }
        _ => {
            dev_err((*tas2562).dev, c"slot width not supported".as_ptr());
            ret = -EINVAL;
        }
    }

    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2562_TDM_CFG5,
        TAS2562_TDM_CFG5_VSNS_SLOT_MASK,
        (*tas2562).v_sense_slot,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2562_TDM_CFG6,
        TAS2562_TDM_CFG6_ISNS_SLOT_MASK,
        (*tas2562).i_sense_slot,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2562_set_bitwidth(tas2562: *mut tas2562_data, bitwidth: c_int) -> c_int {
    let mut ret: c_int;
    let val: c_int;
    let mut sense_en: c_int;

    match bitwidth {
        SNDRV_PCM_FORMAT_S16_LE => {
            snd_soc_component_update_bits(
                (*tas2562).component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXWLEN_MASK,
                TAS2562_TDM_CFG2_RXWLEN_16B,
            );
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            snd_soc_component_update_bits(
                (*tas2562).component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXWLEN_MASK,
                TAS2562_TDM_CFG2_RXWLEN_24B,
            );
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            snd_soc_component_update_bits(
                (*tas2562).component,
                TAS2562_TDM_CFG2,
                TAS2562_TDM_CFG2_RXWLEN_MASK,
                TAS2562_TDM_CFG2_RXWLEN_32B,
            );
        }
        _ => {
            dev_info((*tas2562).dev, c"Unsupported bitwidth format\n".as_ptr());
            return -EINVAL;
        }
    }

    val = snd_soc_component_read((*tas2562).component, TAS2562_PWR_CTRL);
    if val < 0 {
        return val;
    }

    if (val & (1 << TAS2562_VSENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2562_TDM_CFG5_VSNS_EN;
    }

    ret = snd_soc_component_update_bits(
        (*tas2562).component,
        TAS2562_TDM_CFG5,
        TAS2562_TDM_CFG5_VSNS_EN,
        sense_en,
    );
    if ret < 0 {
        return ret;
    }

    if (val & (1 << TAS2562_ISENSE_POWER_EN)) != 0 {
        sense_en = 0;
    } else {
        sense_en = TAS2562_TDM_CFG6_ISNS_EN;
    }

    ret = snd_soc_component_update_bits(
        (*tas2562).component,
        TAS2562_TDM_CFG6,
        TAS2562_TDM_CFG6_ISNS_EN,
        sense_en,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2562_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;
    let mut ret: c_int;

    ret = tas2562_set_bitwidth(tas2562, params_format(params));
    if ret != 0 {
        dev_err((*tas2562).dev, c"set bitwidth failed, %d\n".as_ptr(), ret);
        return ret;
    }

    ret = tas2562_set_samplerate(tas2562, params_rate(params));
    if ret != 0 {
        dev_err((*tas2562).dev, c"set sample rate failed, %d\n".as_ptr(), ret);
    }

    ret
}

unsafe fn tas2562_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;
    let mut asi_cfg_1: u8 = 0;
    let tdm_rx_start_slot: u8;
    let mut ret: c_int;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            asi_cfg_1 = 0;
        }
        SND_SOC_DAIFMT_IB_NF => {
            asi_cfg_1 |= TAS2562_TDM_CFG1_RX_FALLING as u8;
        }
        _ => {
            dev_err((*tas2562).dev, c"ASI format Inverse is not found\n".as_ptr());
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2562_TDM_CFG1,
        TAS2562_TDM_CFG1_RX_EDGE_MASK,
        asi_cfg_1 as c_int,
    );
    if ret < 0 {
        dev_err((*tas2562).dev, c"Failed to set RX edge\n".as_ptr());
        return ret;
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_B => {
            tdm_rx_start_slot = 0;
        }
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_DSP_A => {
            tdm_rx_start_slot = 1;
        }
        _ => {
            dev_err(
                (*tas2562).dev,
                c"DAI Format is not found, fmt=0x%x\n".as_ptr(),
                fmt,
            );
            return -EINVAL;
        }
    }

    ret = snd_soc_component_update_bits(
        component,
        TAS2562_TDM_CFG1,
        TAS2562_RX_OFF_MASK,
        (tdm_rx_start_slot << 1) as c_int,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2562_update_pwr_ctrl(tas2562: *mut tas2562_data) -> c_int {
    let component = (*tas2562).component;
    let val: c_uint;
    let ret: c_int;

    if (*tas2562).dac_powered {
        val = if (*tas2562).unmuted {
            TAS2562_ACTIVE
        } else {
            TAS2562_MUTE
        };
    } else {
        val = TAS2562_SHUTDOWN;
    }

    ret = snd_soc_component_update_bits(component, TAS2562_PWR_CTRL, TAS2562_MODE_MASK, val as c_int);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn tas2562_mute(dai: *mut snd_soc_dai, mute: c_int, _direction: c_int) -> c_int {
    let tas2562 = snd_soc_component_get_drvdata((*dai).component) as *mut tas2562_data;

    (*tas2562).unmuted = mute == 0;
    tas2562_update_pwr_ctrl(tas2562)
}

unsafe fn tas2562_codec_probe(component: *mut snd_soc_component) -> c_int {
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;

    (*tas2562).component = component;

    if !(*tas2562).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2562).sdz_gpio, 1);
    }

    0
}

// CONFIG_PM: when enabled, tas2562_suspend and tas2562_resume are registered;
// otherwise the C driver defines both callbacks as NULL.
unsafe fn tas2562_suspend(component: *mut snd_soc_component) -> c_int {
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;

    regcache_cache_only((*tas2562).regmap, true);
    regcache_mark_dirty((*tas2562).regmap);

    if !(*tas2562).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2562).sdz_gpio, 0);
    }

    0
}

unsafe fn tas2562_resume(component: *mut snd_soc_component) -> c_int {
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;

    if !(*tas2562).sdz_gpio.is_null() {
        gpiod_set_value_cansleep((*tas2562).sdz_gpio, 1);
    }

    regcache_cache_only((*tas2562).regmap, false);

    regcache_sync((*tas2562).regmap)
}

static TAS2562_ASI1_SRC: [*const c_char; 4] = [
    c"I2C offset".as_ptr(),
    c"Left".as_ptr(),
    c"Right".as_ptr(),
    c"LeftRightDiv2".as_ptr(),
];

// static SOC_ENUM_SINGLE_DECL(tas2562_ASI1_src_enum, TAS2562_TDM_CFG2, 4, tas2562_ASI1_src);
// static const struct snd_kcontrol_new tas2562_asi1_mux =
//     SOC_DAPM_ENUM("ASI1 Source", tas2562_ASI1_src_enum);

unsafe fn tas2562_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            (*tas2562).dac_powered = true;
            ret = tas2562_update_pwr_ctrl(tas2562);
        }
        SND_SOC_DAPM_PRE_PMD => {
            (*tas2562).dac_powered = false;
            ret = tas2562_update_pwr_ctrl(tas2562);
        }
        _ => {
            dev_err((*tas2562).dev, c"Not supported evevt\n".as_ptr());
            return -EINVAL;
        }
    }

    ret
}

unsafe fn tas2562_volume_control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;

    (*ucontrol).value.integer.value[0] = (*tas2562).volume_lvl as c_long;
    0
}

unsafe fn tas2562_volume_control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let tas2562 = snd_soc_component_get_drvdata(component) as *mut tas2562_data;
    let mut ret: c_int;
    let index: c_int;
    let reg_val: u32;

    if (*tas2562).volume_lvl == (*ucontrol).value.integer.value[0] as c_int {
        return 0;
    }

    index = ((*ucontrol).value.integer.value[0] / 2) as c_int;
    if index < 0 || index as usize >= FLOAT_VOL_DB_LOOKUP.len() {
        return -EINVAL;
    }

    reg_val = FLOAT_VOL_DB_LOOKUP[index as usize] as u32;

    /*
     * The device applies the 32-bit coefficient to the playback path on
     * the write to DVC_CFG4 (the LSB, book 0 page 2 reg 0x0F), so the
     * bytes must be written MSB first and DVC_CFG4 last. Writing CFG4
     * first latches a mix of the previous coefficient's upper bytes and
     * the new LSB instead of the requested value.
     */
    ret = snd_soc_component_write(component, TAS2562_DVC_CFG1, ((reg_val >> 24) & 0xff) as c_int);
    if ret != 0 {
        return ret;
    }
    ret = snd_soc_component_write(component, TAS2562_DVC_CFG2, ((reg_val >> 16) & 0xff) as c_int);
    if ret != 0 {
        return ret;
    }
    ret = snd_soc_component_write(component, TAS2562_DVC_CFG3, ((reg_val >> 8) & 0xff) as c_int);
    if ret != 0 {
        return ret;
    }
    ret = snd_soc_component_write(component, TAS2562_DVC_CFG4, (reg_val & 0xff) as c_int);
    if ret != 0 {
        return ret;
    }

    (*tas2562).volume_lvl = (*ucontrol).value.integer.value[0] as c_int;

    1
}

/* Digital Volume Control. From 0 dB to -110 dB in 1 dB steps */
// static const DECLARE_TLV_DB_SCALE(dvc_tlv, -11000, 100, 0);
// static DECLARE_TLV_DB_SCALE(tas2562_dac_tlv, 850, 50, 0);
// static const struct snd_kcontrol_new isense_switch =
//     SOC_DAPM_SINGLE("Switch", TAS2562_PWR_CTRL, TAS2562_ISENSE_POWER_EN, 1, 1);
// static const struct snd_kcontrol_new vsense_switch =
//     SOC_DAPM_SINGLE("Switch", TAS2562_PWR_CTRL, TAS2562_VSENSE_POWER_EN, 1, 1);
// static const struct snd_kcontrol_new tas2562_snd_controls[] = {
//     SOC_SINGLE_TLV("Amp Gain Volume", TAS2562_PB_CFG1, 1, 0x1c, 0, tas2562_dac_tlv),
//     SOC_SINGLE_EXT_TLV("Digital Volume Control", TAS2562_DVC_CFG1, 0, 110, 0,
//                        tas2562_volume_control_get, tas2562_volume_control_put, dvc_tlv),
// };
// static const struct snd_soc_dapm_widget tas2110_dapm_widgets[] = { ... };
// static const struct snd_soc_dapm_route tas2110_audio_map[] = { ... };
// static const struct snd_soc_component_driver soc_component_dev_tas2110 = { ... };
// static const struct snd_soc_dapm_widget tas2562_dapm_widgets[] = { ... };
// static const struct snd_soc_dapm_route tas2562_audio_map[] = { ... };
// static const struct snd_soc_component_driver soc_component_dev_tas2562 = { ... };
// static const struct snd_soc_dai_ops tas2562_speaker_dai_ops = { ... };
// static struct snd_soc_dai_driver tas2562_dai[] = { ... };

static TAS2562_RANGES: [regmap_range_cfg; 1] = [regmap_range_cfg {
    range_min: 0,
    range_max: 5 * 128,
    selector_reg: TAS2562_PAGE_CTRL,
    selector_mask: 0xff,
    selector_shift: 0,
    window_start: 0,
    window_len: 128,
}];

static TAS2562_REG_DEFAULTS: [reg_default; 10] = [
    reg_default { reg: TAS2562_PAGE_CTRL, def: 0x00 },
    reg_default { reg: TAS2562_SW_RESET, def: 0x00 },
    reg_default { reg: TAS2562_PWR_CTRL, def: 0x0e },
    reg_default { reg: TAS2562_PB_CFG1, def: 0x20 },
    reg_default { reg: TAS2562_TDM_CFG0, def: 0x09 },
    reg_default { reg: TAS2562_TDM_CFG1, def: 0x02 },
    reg_default { reg: TAS2562_DVC_CFG1, def: 0x40 },
    reg_default { reg: TAS2562_DVC_CFG2, def: 0x40 },
    reg_default { reg: TAS2562_DVC_CFG3, def: 0x00 },
    reg_default { reg: TAS2562_DVC_CFG4, def: 0x00 },
];

static TAS2562_REGMAP_CONFIG: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: 5 * 128,
    cache_type: REGCACHE_RBTREE,
    reg_defaults: TAS2562_REG_DEFAULTS.as_ptr(),
    num_reg_defaults: TAS2562_REG_DEFAULTS.len() as c_uint,
    ranges: TAS2562_RANGES.as_ptr(),
    num_ranges: TAS2562_RANGES.len() as c_uint,
};

unsafe fn tas2562_parse_dt(tas2562: *mut tas2562_data) -> c_int {
    let dev = (*tas2562).dev;
    let mut ret: c_int = 0;

    (*tas2562).sdz_gpio = devm_gpiod_get_optional(dev, c"shutdown".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*tas2562).sdz_gpio as *const c_void) {
        if PTR_ERR((*tas2562).sdz_gpio as *const c_void) == -EPROBE_DEFER as c_long {
            return -EPROBE_DEFER;
        }

        (*tas2562).sdz_gpio = core::ptr::null_mut();
    }

    /*
     * The shut-down property is deprecated but needs to be checked for
     * backwards compatibility.
     */
    if (*tas2562).sdz_gpio.is_null() {
        (*tas2562).sdz_gpio = devm_gpiod_get_optional(dev, c"shut-down".as_ptr(), GPIOD_OUT_HIGH);
        if IS_ERR((*tas2562).sdz_gpio as *const c_void) {
            if PTR_ERR((*tas2562).sdz_gpio as *const c_void) == -EPROBE_DEFER as c_long {
                return -EPROBE_DEFER;
            }

            (*tas2562).sdz_gpio = core::ptr::null_mut();
        }
    }

    if (*tas2562).model_id == tas256x_model::TAS2110 as c_int {
        return ret;
    }

    ret = fwnode_property_read_u32(
        (*dev).fwnode,
        c"ti,imon-slot-no".as_ptr(),
        &mut (*tas2562).i_sense_slot as *mut c_int as *mut u32,
    );
    if ret != 0 {
        dev_err(
            dev,
            c"Property %s is missing setting default slot\n".as_ptr(),
            c"ti,imon-slot-no".as_ptr(),
        );
        (*tas2562).i_sense_slot = 0;
    }

    ret = fwnode_property_read_u32(
        (*dev).fwnode,
        c"ti,vmon-slot-no".as_ptr(),
        &mut (*tas2562).v_sense_slot as *mut c_int as *mut u32,
    );
    if ret != 0 {
        dev_info(
            dev,
            c"Property %s is missing setting default slot\n".as_ptr(),
            c"ti,vmon-slot-no".as_ptr(),
        );
        (*tas2562).v_sense_slot = 2;
    }

    if (*tas2562).v_sense_slot < (*tas2562).i_sense_slot {
        dev_err(dev, c"Vsense slot must be greater than Isense slot\n".as_ptr());
        return -EINVAL;
    }

    ret
}

static TAS2562_ID: [i2c_device_id; 4] = [
    i2c_device_id { name: *b"tas2562\0", driver_data: tas256x_model::TAS2562 as c_ulong },
    i2c_device_id { name: *b"tas2564\0", driver_data: tas256x_model::TAS2564 as c_ulong },
    i2c_device_id { name: *b"tas2110\0", driver_data: tas256x_model::TAS2110 as c_ulong },
    i2c_device_id { name: [0; 8], driver_data: 0 },
];
// MODULE_DEVICE_TABLE(i2c, tas2562_id);

unsafe fn tas2562_probe(client: *mut i2c_client) -> c_int {
    let dev = &mut (*client).dev as *mut device;
    let data: *mut tas2562_data;
    let ret: c_int;

    data = devm_kzalloc(dev, core::mem::size_of::<tas2562_data>(), GFP_KERNEL) as *mut tas2562_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).client = client;
    (*data).dev = &mut (*client).dev;
    (*data).model_id = i2c_get_match_data(client) as usize as c_int;
    /* Register default is 0x40400000, this is closest */
    (*data).volume_lvl = ((FLOAT_VOL_DB_LOOKUP.len() - 1) * 2) as c_int;

    tas2562_parse_dt(data);

    (*data).regmap = devm_regmap_init_i2c(client, &TAS2562_REGMAP_CONFIG);
    if IS_ERR((*data).regmap as *const c_void) {
        ret = PTR_ERR((*data).regmap as *const c_void) as c_int;
        dev_err(dev, c"failed to allocate register map: %d\n".as_ptr(), ret);
        return ret;
    }

    dev_set_drvdata(&mut (*client).dev, data as *mut c_void);

    if (*data).model_id == tas256x_model::TAS2110 as c_int {
        return devm_snd_soc_register_component(
            dev,
            &SOC_COMPONENT_DEV_TAS2110,
            TAS2562_DAI.as_mut_ptr(),
            TAS2562_DAI.len() as c_int,
        );
    }

    devm_snd_soc_register_component(
        dev,
        &SOC_COMPONENT_DEV_TAS2562,
        TAS2562_DAI.as_mut_ptr(),
        TAS2562_DAI.len() as c_int,
    )
}

// CONFIG_OF:
// static const struct of_device_id tas2562_of_match[] = {
//     { .compatible = "ti,tas2562", },
//     { .compatible = "ti,tas2564", },
//     { .compatible = "ti,tas2110", },
//     { },
// };
// MODULE_DEVICE_TABLE(of, tas2562_of_match);

// static struct i2c_driver tas2562_i2c_driver = {
//     .driver = {
//         .name = "tas2562",
//         .of_match_table = of_match_ptr(tas2562_of_match),
//     },
//     .probe = tas2562_probe,
//     .id_table = tas2562_id,
// };
// module_i2c_driver(tas2562_i2c_driver);
// MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
// MODULE_DESCRIPTION("TAS2562 Audio amplifier driver");
// MODULE_LICENSE("GPL");

extern "C" {
    static SOC_COMPONENT_DEV_TAS2110: snd_soc_component_driver;
    static SOC_COMPONENT_DEV_TAS2562: snd_soc_component_driver;
    static mut TAS2562_DAI: [snd_soc_dai_driver; 1];
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    fwnode: *mut fwnode_handle,
}
#[repr(C)]
pub struct i2c_client {
    dev: device,
}
#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
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
pub struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}
#[repr(C)]
pub struct regmap_range_cfg {
    range_min: c_uint,
    range_max: c_uint,
    selector_reg: c_uint,
    selector_mask: c_uint,
    selector_shift: c_uint,
    window_start: c_uint,
    window_len: c_uint,
}
#[repr(C)]
pub struct reg_default {
    reg: c_uint,
    def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
    cache_type: c_uint,
    reg_defaults: *const reg_default,
    num_reg_defaults: c_uint,
    ranges: *const regmap_range_cfg,
    num_ranges: c_uint,
}
#[repr(C)]
pub struct i2c_device_id {
    name: [u8; 8],
    driver_data: c_ulong,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

type c_long = core::ffi::c_long;

extern "C" {
    fn dev_info(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_int) -> c_int;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn fwnode_property_read_u32(
        fwnode: *mut fwnode_handle,
        propname: *const c_char,
        val: *mut u32,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn i2c_get_match_data(client: *mut i2c_client) -> *const c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn __ffs(word: c_ulong) -> c_ulong;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

extern "C" {
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FORMAT_S32_LE: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAPM_POST_PMU: c_int;
    static SND_SOC_DAPM_PRE_PMD: c_int;
    static TAS2562_TDM_CFG0_RAMPRATE_44_1: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_7305_8KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_14_7_16KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_22_05_24KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_29_4_32KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_44_1_48KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_88_2_96KHZ: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_176_4_192KHZ: c_int;
    static TAS2562_TDM_CFG0: c_uint;
    static TAS2562_TDM_CFG0_RAMPRATE_MASK: c_int;
    static TAS2562_TDM_CFG0_SAMPRATE_MASK: c_int;
    static TAS2562_TDM_CFG1: c_uint;
    static TAS2562_TDM_CFG1_RX_FALLING: c_int;
    static TAS2562_TDM_CFG1_RX_EDGE_MASK: c_int;
    static TAS2562_RX_OFF_MASK: c_int;
    static TAS2562_TDM_CFG2: c_uint;
    static TAS2562_TDM_CFG2_RXLEN_MASK: c_int;
    static TAS2562_TDM_CFG2_RXLEN_16B: c_int;
    static TAS2562_TDM_CFG2_RXLEN_24B: c_int;
    static TAS2562_TDM_CFG2_RXLEN_32B: c_int;
    static TAS2562_TDM_CFG2_RXWLEN_MASK: c_int;
    static TAS2562_TDM_CFG2_RXWLEN_16B: c_int;
    static TAS2562_TDM_CFG2_RXWLEN_24B: c_int;
    static TAS2562_TDM_CFG2_RXWLEN_32B: c_int;
    static TAS2562_TDM_CFG3: c_uint;
    static TAS2562_TDM_CFG5: c_uint;
    static TAS2562_TDM_CFG5_VSNS_SLOT_MASK: c_int;
    static TAS2562_TDM_CFG5_VSNS_EN: c_int;
    static TAS2562_TDM_CFG6: c_uint;
    static TAS2562_TDM_CFG6_ISNS_SLOT_MASK: c_int;
    static TAS2562_TDM_CFG6_ISNS_EN: c_int;
    static TAS2562_RIGHT_SLOT_SHIFT: c_int;
    static TAS2562_PWR_CTRL: c_uint;
    static TAS2562_VSENSE_POWER_EN: c_int;
    static TAS2562_ISENSE_POWER_EN: c_int;
    static TAS2562_ACTIVE: c_uint;
    static TAS2562_MUTE: c_uint;
    static TAS2562_SHUTDOWN: c_uint;
    static TAS2562_MODE_MASK: c_int;
    static TAS2562_DVC_CFG1: c_uint;
    static TAS2562_DVC_CFG2: c_uint;
    static TAS2562_DVC_CFG3: c_uint;
    static TAS2562_DVC_CFG4: c_uint;
    static TAS2562_PAGE_CTRL: c_uint;
    static TAS2562_SW_RESET: c_uint;
    static TAS2562_PB_CFG1: c_uint;
    static EINVAL: c_int;
    static EPROBE_DEFER: c_int;
    static ENOMEM: c_int;
    static GPIOD_OUT_HIGH: c_int;
    static GFP_KERNEL: c_uint;
    static REGCACHE_RBTREE: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
