// SPDX-License-Identifier: GPL-2.0-only
/*
 * cs35l33.c -- CS35L33 ALSA SoC audio driver
 *
 * Copyright 2016 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <paul.handrigan@cirrus.com>
 */

// Translated from the C implementation source. Linux/ALSA includes from the
// original file are external dependencies in Rust form.

pub const CS35L33_BOOT_DELAY: u32 = 50;

#[repr(C)]
pub struct cs35l33_private {
    pub component: *mut snd_soc_component,
    pub pdata: cs35l33_pdata,
    pub regmap: *mut regmap,
    pub reset_gpio: *mut gpio_desc,
    pub amp_cal: bool,
    pub irq_requested: bool,
    pub mclk_int: c_int,
    pub core_supplies: [regulator_bulk_data; 2],
    pub num_core_supplies: c_int,
    pub is_tdm_mode: bool,
    pub enable_soft_ramp: bool,
}

pub static cs35l33_reg: [reg_default; 36] = [
    reg_default { reg: CS35L33_PWRCTL1, def: 0x85 },
    reg_default { reg: CS35L33_PWRCTL2, def: 0xFE },
    reg_default { reg: CS35L33_CLK_CTL, def: 0x0C },
    reg_default { reg: CS35L33_BST_PEAK_CTL, def: 0x90 },
    reg_default { reg: CS35L33_PROTECT_CTL, def: 0x55 },
    reg_default { reg: CS35L33_BST_CTL1, def: 0x00 },
    reg_default { reg: CS35L33_BST_CTL2, def: 0x01 },
    reg_default { reg: CS35L33_ADSP_CTL, def: 0x00 },
    reg_default { reg: CS35L33_ADC_CTL, def: 0xC8 },
    reg_default { reg: CS35L33_DAC_CTL, def: 0x14 },
    reg_default { reg: CS35L33_DIG_VOL_CTL, def: 0x00 },
    reg_default { reg: CS35L33_CLASSD_CTL, def: 0x04 },
    reg_default { reg: CS35L33_AMP_CTL, def: 0x90 },
    reg_default { reg: CS35L33_INT_MASK_1, def: 0xFF },
    reg_default { reg: CS35L33_INT_MASK_2, def: 0xFF },
    reg_default { reg: CS35L33_DIAG_LOCK, def: 0x00 },
    reg_default { reg: CS35L33_DIAG_CTRL_1, def: 0x40 },
    reg_default { reg: CS35L33_DIAG_CTRL_2, def: 0x00 },
    reg_default { reg: CS35L33_HG_MEMLDO_CTL, def: 0x62 },
    reg_default { reg: CS35L33_HG_REL_RATE, def: 0x03 },
    reg_default { reg: CS35L33_LDO_DEL, def: 0x12 },
    reg_default { reg: CS35L33_HG_HEAD, def: 0x0A },
    reg_default { reg: CS35L33_HG_EN, def: 0x05 },
    reg_default { reg: CS35L33_TX_VMON, def: 0x00 },
    reg_default { reg: CS35L33_TX_IMON, def: 0x03 },
    reg_default { reg: CS35L33_TX_VPMON, def: 0x02 },
    reg_default { reg: CS35L33_TX_VBSTMON, def: 0x05 },
    reg_default { reg: CS35L33_TX_FLAG, def: 0x06 },
    reg_default { reg: CS35L33_TX_EN1, def: 0x00 },
    reg_default { reg: CS35L33_TX_EN2, def: 0x00 },
    reg_default { reg: CS35L33_TX_EN3, def: 0x00 },
    reg_default { reg: CS35L33_TX_EN4, def: 0x00 },
    reg_default { reg: CS35L33_RX_AUD, def: 0x40 },
    reg_default { reg: CS35L33_RX_SPLY, def: 0x03 },
    reg_default { reg: CS35L33_RX_ALIVE, def: 0x04 },
    reg_default { reg: CS35L33_BST_CTL4, def: 0x63 },
];

pub static cs35l33_patch: [reg_sequence; 7] = [
    reg_sequence { reg: 0x00, def: 0x99, delay_us: 0 },
    reg_sequence { reg: 0x59, def: 0x02, delay_us: 0 },
    reg_sequence { reg: 0x52, def: 0x30, delay_us: 0 },
    reg_sequence { reg: 0x39, def: 0x45, delay_us: 0 },
    reg_sequence { reg: 0x57, def: 0x30, delay_us: 0 },
    reg_sequence { reg: 0x2C, def: 0x68, delay_us: 0 },
    reg_sequence { reg: 0x00, def: 0x00, delay_us: 0 },
];

pub unsafe extern "C" fn cs35l33_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L33_DEVID_AB | CS35L33_DEVID_CD | CS35L33_DEVID_E | CS35L33_REV_ID
        | CS35L33_INT_STATUS_1 | CS35L33_INT_STATUS_2 | CS35L33_HG_STATUS => true,
        _ => false,
    }
}

pub unsafe extern "C" fn cs35l33_writeable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        /* these are read only registers */
        CS35L33_DEVID_AB | CS35L33_DEVID_CD | CS35L33_DEVID_E | CS35L33_REV_ID
        | CS35L33_INT_STATUS_1 | CS35L33_INT_STATUS_2 | CS35L33_HG_STATUS => false,
        _ => true,
    }
}

pub unsafe extern "C" fn cs35l33_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        CS35L33_DEVID_AB | CS35L33_DEVID_CD | CS35L33_DEVID_E | CS35L33_REV_ID
        | CS35L33_PWRCTL1 | CS35L33_PWRCTL2 | CS35L33_CLK_CTL | CS35L33_BST_PEAK_CTL
        | CS35L33_PROTECT_CTL | CS35L33_BST_CTL1 | CS35L33_BST_CTL2 | CS35L33_ADSP_CTL
        | CS35L33_ADC_CTL | CS35L33_DAC_CTL | CS35L33_DIG_VOL_CTL | CS35L33_CLASSD_CTL
        | CS35L33_AMP_CTL | CS35L33_INT_MASK_1 | CS35L33_INT_MASK_2
        | CS35L33_INT_STATUS_1 | CS35L33_INT_STATUS_2 | CS35L33_DIAG_LOCK
        | CS35L33_DIAG_CTRL_1 | CS35L33_DIAG_CTRL_2 | CS35L33_HG_MEMLDO_CTL
        | CS35L33_HG_REL_RATE | CS35L33_LDO_DEL | CS35L33_HG_HEAD | CS35L33_HG_EN
        | CS35L33_TX_VMON | CS35L33_TX_IMON | CS35L33_TX_VPMON | CS35L33_TX_VBSTMON
        | CS35L33_TX_FLAG | CS35L33_TX_EN1 | CS35L33_TX_EN2 | CS35L33_TX_EN3
        | CS35L33_TX_EN4 | CS35L33_RX_AUD | CS35L33_RX_SPLY | CS35L33_RX_ALIVE
        | CS35L33_BST_CTL4 => true,
        _ => false,
    }
}

DECLARE_TLV_DB_SCALE!(classd_ctl_tlv, 900, 100, 0);
DECLARE_TLV_DB_SCALE!(dac_tlv, -10200, 50, 0);

pub static cs35l33_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE_TLV!("SPK Amp Volume", CS35L33_AMP_CTL, 4, 0x09, 0, classd_ctl_tlv),
    SOC_SINGLE_SX_TLV!("DAC Volume", CS35L33_DIG_VOL_CTL, 0, 0x34, 0xE4, dac_tlv),
];

pub unsafe extern "C" fn cs35l33_spkrdrv_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if !(*priv_).amp_cal {
                usleep_range(8000, 9000);
                (*priv_).amp_cal = true;
                regmap_update_bits((*priv_).regmap, CS35L33_CLASSD_CTL, CS35L33_AMP_CAL, 0);
                dev_dbg((*component).dev, "Amp calibration done\n\0".as_ptr() as *const c_char);
            }
            dev_dbg((*component).dev, "Amp turned on\n\0".as_ptr() as *const c_char);
        }
        SND_SOC_DAPM_POST_PMD => {
            dev_dbg((*component).dev, "Amp turned off\n\0".as_ptr() as *const c_char);
        }
        _ => {
            dev_err((*component).dev, "Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event);
        }
    }
    0
}

pub unsafe extern "C" fn cs35l33_sdin_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;
    let mut val: c_uint;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL1, CS35L33_PDN_BST, 0);
            val = if (*priv_).is_tdm_mode { 0 } else { CS35L33_PDN_TDM };
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL2, CS35L33_PDN_TDM, val);
            dev_dbg((*component).dev, "BST turned on\n\0".as_ptr() as *const c_char);
        }
        SND_SOC_DAPM_POST_PMU => {
            dev_dbg((*component).dev, "SDIN turned on\n\0".as_ptr() as *const c_char);
            if !(*priv_).amp_cal {
                regmap_update_bits((*priv_).regmap, CS35L33_CLASSD_CTL, CS35L33_AMP_CAL, CS35L33_AMP_CAL);
                dev_dbg((*component).dev, "Amp calibration started\n\0".as_ptr() as *const c_char);
                usleep_range(10000, 11000);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL2, CS35L33_PDN_TDM, CS35L33_PDN_TDM);
            usleep_range(4000, 4100);
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL1, CS35L33_PDN_BST, CS35L33_PDN_BST);
            dev_dbg((*component).dev, "BST and SDIN turned off\n\0".as_ptr() as *const c_char);
        }
        _ => {
            dev_err((*component).dev, "Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event);
        }
    }
    0
}

pub unsafe extern "C" fn cs35l33_sdout_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*(*w).dapm));
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;
    let mask: c_uint = CS35L33_SDOUT_3ST_I2S | CS35L33_PDN_TDM;
    let mask2: c_uint = CS35L33_SDOUT_3ST_TDM;
    let val: c_uint;
    let val2: c_uint;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*priv_).is_tdm_mode {
                /* set sdout_3st_i2s and reset pdn_tdm */
                val = CS35L33_SDOUT_3ST_I2S;
                /* reset sdout_3st_tdm */
                val2 = 0;
            } else {
                /* reset sdout_3st_i2s and set pdn_tdm */
                val = CS35L33_PDN_TDM;
                /* set sdout_3st_tdm */
                val2 = CS35L33_SDOUT_3ST_TDM;
            }
            dev_dbg((*component).dev, "SDOUT turned on\n\0".as_ptr() as *const c_char);
        }
        SND_SOC_DAPM_PRE_PMD => {
            val = CS35L33_SDOUT_3ST_I2S | CS35L33_PDN_TDM;
            val2 = CS35L33_SDOUT_3ST_TDM;
            dev_dbg((*component).dev, "SDOUT turned off\n\0".as_ptr() as *const c_char);
        }
        _ => {
            dev_err((*component).dev, "Invalid event = 0x%x\n\0".as_ptr() as *const c_char, event);
            return 0;
        }
    }

    regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL2, mask, val);
    regmap_update_bits((*priv_).regmap, CS35L33_CLK_CTL, mask2, val2);
    0
}

pub static cs35l33_dapm_widgets: [snd_soc_dapm_widget; 9] = [
    SND_SOC_DAPM_OUTPUT!("SPK"),
    SND_SOC_DAPM_OUT_DRV_E!("SPKDRV", CS35L33_PWRCTL1, 7, 1, NULL, 0,
        cs35l33_spkrdrv_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_AIF_IN_E!("SDIN", NULL, 0, CS35L33_PWRCTL2, 2, 1,
        cs35l33_sdin_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT!("MON"),
    SND_SOC_DAPM_ADC!("VMON", NULL, CS35L33_PWRCTL2, CS35L33_PDN_VMON_SHIFT, 1),
    SND_SOC_DAPM_ADC!("IMON", NULL, CS35L33_PWRCTL2, CS35L33_PDN_IMON_SHIFT, 1),
    SND_SOC_DAPM_ADC!("VPMON", NULL, CS35L33_PWRCTL2, CS35L33_PDN_VPMON_SHIFT, 1),
    SND_SOC_DAPM_ADC!("VBSTMON", NULL, CS35L33_PWRCTL2, CS35L33_PDN_VBSTMON_SHIFT, 1),
    SND_SOC_DAPM_AIF_OUT_E!("SDOUT", NULL, 0, SND_SOC_NOPM, 0, 0,
        cs35l33_sdout_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
];

pub static cs35l33_audio_map: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: c"SDIN".as_ptr(), control: NULL, source: c"CS35L33 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKDRV".as_ptr(), control: NULL, source: c"SDIN".as_ptr() },
    snd_soc_dapm_route { sink: c"SPK".as_ptr(), control: NULL, source: c"SPKDRV".as_ptr() },
    snd_soc_dapm_route { sink: c"VMON".as_ptr(), control: NULL, source: c"MON".as_ptr() },
    snd_soc_dapm_route { sink: c"IMON".as_ptr(), control: NULL, source: c"MON".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: NULL, source: c"VMON".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: NULL, source: c"IMON".as_ptr() },
    snd_soc_dapm_route { sink: c"CS35L33 Capture".as_ptr(), control: NULL, source: c"SDOUT".as_ptr() },
];

pub static cs35l33_vphg_auto_route: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: c"SPKDRV".as_ptr(), control: NULL, source: c"VPMON".as_ptr() },
    snd_soc_dapm_route { sink: c"VPMON".as_ptr(), control: NULL, source: c"CS35L33 Playback".as_ptr() },
];

pub static cs35l33_vp_vbst_mon_route: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: NULL, source: c"VPMON".as_ptr() },
    snd_soc_dapm_route { sink: c"VPMON".as_ptr(), control: NULL, source: c"MON".as_ptr() },
    snd_soc_dapm_route { sink: c"SDOUT".as_ptr(), control: NULL, source: c"VBSTMON".as_ptr() },
    snd_soc_dapm_route { sink: c"VBSTMON".as_ptr(), control: NULL, source: c"MON".as_ptr() },
];

pub unsafe extern "C" fn cs35l33_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let mut val: c_uint = 0;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL1, CS35L33_PDN_ALL, 0);
            regmap_update_bits((*priv_).regmap, CS35L33_CLK_CTL, CS35L33_MCLKDIS, 0);
        }
        SND_SOC_BIAS_STANDBY => {
            regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL1, CS35L33_PDN_ALL, CS35L33_PDN_ALL);
            regmap_read((*priv_).regmap, CS35L33_INT_STATUS_2, &mut val);
            usleep_range(1000, 1100);
            if (val & CS35L33_PDN_DONE) != 0 {
                regmap_update_bits((*priv_).regmap, CS35L33_CLK_CTL, CS35L33_MCLKDIS, CS35L33_MCLKDIS);
            }
        }
        SND_SOC_BIAS_OFF => {}
        _ => return -EINVAL,
    }
    0
}

#[repr(C)]
pub struct cs35l33_mclk_div {
    pub mclk: c_int,
    pub srate: c_int,
    pub adsp_rate: u8,
    pub int_fs_ratio: u8,
}

pub static cs35l33_mclk_coeffs: [cs35l33_mclk_div; 21] = [
    /* MCLK, Sample Rate, adsp_rate, int_fs_ratio */
    cs35l33_mclk_div { mclk: 5644800, srate: 11025, adsp_rate: 0x4, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 5644800, srate: 22050, adsp_rate: 0x8, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 5644800, srate: 44100, adsp_rate: 0xC, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6000000, srate: 8000, adsp_rate: 0x1, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 11025, adsp_rate: 0x2, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 11029, adsp_rate: 0x3, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 12000, adsp_rate: 0x4, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 16000, adsp_rate: 0x5, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 22050, adsp_rate: 0x6, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 22059, adsp_rate: 0x7, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 24000, adsp_rate: 0x8, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 32000, adsp_rate: 0x9, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 44100, adsp_rate: 0xA, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 44118, adsp_rate: 0xB, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6000000, srate: 48000, adsp_rate: 0xC, int_fs_ratio: 0 },
    cs35l33_mclk_div { mclk: 6144000, srate: 8000, adsp_rate: 0x1, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6144000, srate: 12000, adsp_rate: 0x4, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6144000, srate: 16000, adsp_rate: 0x5, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6144000, srate: 24000, adsp_rate: 0x8, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6144000, srate: 32000, adsp_rate: 0x9, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
    cs35l33_mclk_div { mclk: 6144000, srate: 48000, adsp_rate: 0xC, int_fs_ratio: CS35L33_INT_FS_RATE as u8 },
];

pub unsafe extern "C" fn cs35l33_get_mclk_coeff(mclk: c_int, srate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < cs35l33_mclk_coeffs.len() {
        if cs35l33_mclk_coeffs[i].mclk == mclk && cs35l33_mclk_coeffs[i].srate == srate {
            return i as c_int;
        }
        i += 1;
    }
    -EINVAL
}

pub unsafe extern "C" fn cs35l33_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            regmap_update_bits((*priv_).regmap, CS35L33_ADSP_CTL, CS35L33_MS_MASK, CS35L33_MS_MASK);
            dev_dbg((*component).dev, "Audio port in master mode\n\0".as_ptr() as *const c_char);
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            regmap_update_bits((*priv_).regmap, CS35L33_ADSP_CTL, CS35L33_MS_MASK, 0);
            dev_dbg((*component).dev, "Audio port in slave mode\n\0".as_ptr() as *const c_char);
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            /*
             * tdm mode in cs35l33 resembles dsp-a mode very
             * closely, it is dsp-a with fsync shifted left by half bclk
             */
            (*priv_).is_tdm_mode = true;
            dev_dbg((*component).dev, "Audio port in TDM mode\n\0".as_ptr() as *const c_char);
        }
        SND_SOC_DAIFMT_I2S => {
            (*priv_).is_tdm_mode = false;
            dev_dbg((*component).dev, "Audio port in I2S mode\n\0".as_ptr() as *const c_char);
        }
        _ => return -EINVAL,
    }
    0
}

pub unsafe extern "C" fn cs35l33_pcm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;
    let mut sample_size: c_int = params_width(params);
    let coeff: c_int = cs35l33_get_mclk_coeff((*priv_).mclk_int, params_rate(params));

    if coeff < 0 {
        return coeff;
    }

    regmap_update_bits(
        (*priv_).regmap,
        CS35L33_CLK_CTL,
        CS35L33_ADSP_FS | CS35L33_INT_FS_RATE,
        cs35l33_mclk_coeffs[coeff as usize].int_fs_ratio as c_uint
            | cs35l33_mclk_coeffs[coeff as usize].adsp_rate as c_uint,
    );

    if (*priv_).is_tdm_mode {
        sample_size = (sample_size / 8) - 1;
        if sample_size > 2 {
            sample_size = 2;
        }
        regmap_update_bits(
            (*priv_).regmap,
            CS35L33_RX_AUD,
            CS35L33_AUDIN_RX_DEPTH,
            (sample_size as c_uint) << CS35L33_AUDIN_RX_DEPTH_SHIFT,
        );
    }

    dev_dbg((*component).dev, "sample rate=%d, bits per sample=%d\n\0".as_ptr() as *const c_char,
        params_rate(params), params_width(params));
    0
}

pub static cs35l33_src_rates: [c_uint; 12] = [
    8000, 11025, 11029, 12000, 16000, 22050,
    22059, 24000, 32000, 44100, 44118, 48000,
];

pub static cs35l33_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: cs35l33_src_rates.len() as c_uint,
    list: cs35l33_src_rates.as_ptr(),
};

pub unsafe extern "C" fn cs35l33_pcm_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &cs35l33_constraints);
    0
}

pub unsafe extern "C" fn cs35l33_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int {
    let component = (*dai).component;
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    if tristate != 0 {
        regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL2, CS35L33_SDOUT_3ST_I2S, CS35L33_SDOUT_3ST_I2S);
        regmap_update_bits((*priv_).regmap, CS35L33_CLK_CTL, CS35L33_SDOUT_3ST_TDM, CS35L33_SDOUT_3ST_TDM);
    } else {
        regmap_update_bits((*priv_).regmap, CS35L33_PWRCTL2, CS35L33_SDOUT_3ST_I2S, 0);
        regmap_update_bits((*priv_).regmap, CS35L33_CLK_CTL, CS35L33_SDOUT_3ST_TDM, 0);
    }
    0
}

pub unsafe extern "C" fn cs35l33_set_tdm_slot(
    dai: *mut snd_soc_dai,
    mut tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;
    let mut reg: c_uint;
    let mut bit_pos: c_uint;
    let mut i: c_uint;
    let mut slot: c_int;
    let mut slot_num: c_int;

    if slot_width != 8 {
        return -EINVAL;
    }

    /* scan rx_mask for aud slot */
    slot = ffs(rx_mask) - 1;
    if slot >= 0 {
        regmap_update_bits((*priv_).regmap, CS35L33_RX_AUD, CS35L33_X_LOC, slot as c_uint);
        dev_dbg((*component).dev, "Audio starts from slots %d\0".as_ptr() as *const c_char, slot);
    }

    /*
     * scan tx_mask: vmon(2 slots); imon (2 slots);
     * vpmon (1 slot) vbstmon (1 slot)
     */
    slot = ffs(tx_mask) - 1;
    slot_num = 0;

    i = 0;
    while i < 2 {
        /* disable vpmon/vbstmon: enable later if set in tx_mask */
        regmap_update_bits(
            (*priv_).regmap,
            CS35L33_TX_VPMON + i,
            CS35L33_X_STATE | CS35L33_X_LOC,
            CS35L33_X_STATE | CS35L33_X_LOC,
        );
        i += 1;
    }

    /* disconnect {vp,vbst}_mon routes: eanble later if set in tx_mask*/
    snd_soc_dapm_del_routes(dapm, cs35l33_vp_vbst_mon_route.as_ptr(), cs35l33_vp_vbst_mon_route.len() as c_int);

    while slot >= 0 {
        /* configure VMON_TX_LOC */
        if slot_num == 0 {
            regmap_update_bits((*priv_).regmap, CS35L33_TX_VMON, CS35L33_X_STATE | CS35L33_X_LOC, slot as c_uint);
            dev_dbg((*component).dev, "VMON enabled in slots %d-%d\0".as_ptr() as *const c_char, slot, slot + 1);
        }

        /* configure IMON_TX_LOC */
        if slot_num == 3 {
            regmap_update_bits((*priv_).regmap, CS35L33_TX_IMON, CS35L33_X_STATE | CS35L33_X_LOC, slot as c_uint);
            dev_dbg((*component).dev, "IMON enabled in slots %d-%d\0".as_ptr() as *const c_char, slot, slot + 1);
        }

        /* configure VPMON_TX_LOC */
        if slot_num == 4 {
            regmap_update_bits((*priv_).regmap, CS35L33_TX_VPMON, CS35L33_X_STATE | CS35L33_X_LOC, slot as c_uint);
            snd_soc_dapm_add_routes(dapm, cs35l33_vp_vbst_mon_route.as_ptr().add(0), 2);
            dev_dbg((*component).dev, "VPMON enabled in slots %d\0".as_ptr() as *const c_char, slot);
        }

        /* configure VBSTMON_TX_LOC */
        if slot_num == 5 {
            regmap_update_bits((*priv_).regmap, CS35L33_TX_VBSTMON, CS35L33_X_STATE | CS35L33_X_LOC, slot as c_uint);
            snd_soc_dapm_add_routes(dapm, cs35l33_vp_vbst_mon_route.as_ptr().add(2), 2);
            dev_dbg((*component).dev, "VBSTMON enabled in slots %d\0".as_ptr() as *const c_char, slot);
        }

        /* Enable the relevant tx slot */
        reg = CS35L33_TX_EN4 - ((slot / 8) as c_uint);
        bit_pos = (slot - ((slot / 8) * 8)) as c_uint;
        regmap_update_bits((*priv_).regmap, reg, 1u32 << bit_pos, 1u32 << bit_pos);

        tx_mask &= !(1u32 << (slot as c_uint));
        slot = ffs(tx_mask) - 1;
        slot_num += 1;
    }
    0
}

pub unsafe extern "C" fn cs35l33_component_set_sysclk(
    component: *mut snd_soc_component,
    _clk_id: c_int,
    _source: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let cs35l33 = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    match freq {
        CS35L33_MCLK_5644 | CS35L33_MCLK_6 | CS35L33_MCLK_6144 => {
            regmap_update_bits((*cs35l33).regmap, CS35L33_CLK_CTL, CS35L33_MCLKDIV2, 0);
            (*cs35l33).mclk_int = freq as c_int;
        }
        CS35L33_MCLK_11289 | CS35L33_MCLK_12 | CS35L33_MCLK_12288 => {
            regmap_update_bits((*cs35l33).regmap, CS35L33_CLK_CTL, CS35L33_MCLKDIV2, CS35L33_MCLKDIV2);
            (*cs35l33).mclk_int = (freq / 2) as c_int;
        }
        _ => {
            (*cs35l33).mclk_int = 0;
            return -EINVAL;
        }
    }

    dev_dbg((*component).dev, "external mclk freq=%d, internal mclk freq=%d\n\0".as_ptr() as *const c_char,
        freq, (*cs35l33).mclk_int);
    0
}

pub static cs35l33_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(cs35l33_pcm_startup),
    set_tristate: Some(cs35l33_set_tristate),
    set_fmt: Some(cs35l33_set_dai_fmt),
    hw_params: Some(cs35l33_pcm_hw_params),
    set_tdm_slot: Some(cs35l33_set_tdm_slot),
};

pub static mut cs35l33_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"cs35l33-dai".as_ptr(),
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: c"CS35L33 Playback".as_ptr(),
        channels_min: 1,
        channels_max: 1,
        rates: CS35L33_RATES,
        formats: CS35L33_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"CS35L33 Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: CS35L33_RATES,
        formats: CS35L33_FORMATS,
    },
    ops: &cs35l33_ops,
    symmetric_rate: 1,
};

pub unsafe extern "C" fn cs35l33_set_hg_data(
    component: *mut snd_soc_component,
    pdata: *mut cs35l33_pdata,
) -> c_int {
    let hg_config = &mut (*pdata).hg_config as *mut cs35l33_hg;
    let dapm = snd_soc_component_to_dapm(component);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    if (*hg_config).enable_hg_algo {
        regmap_update_bits((*priv_).regmap, CS35L33_HG_MEMLDO_CTL, CS35L33_MEM_DEPTH_MASK,
            (*hg_config).mem_depth << CS35L33_MEM_DEPTH_SHIFT);
        regmap_write((*priv_).regmap, CS35L33_HG_REL_RATE, (*hg_config).release_rate);
        regmap_update_bits((*priv_).regmap, CS35L33_HG_HEAD, CS35L33_HD_RM_MASK,
            (*hg_config).hd_rm << CS35L33_HD_RM_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_HG_MEMLDO_CTL, CS35L33_LDO_THLD_MASK,
            (*hg_config).ldo_thld << CS35L33_LDO_THLD_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_HG_MEMLDO_CTL, CS35L33_LDO_DISABLE_MASK,
            (*hg_config).ldo_path_disable << CS35L33_LDO_DISABLE_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_LDO_DEL, CS35L33_LDO_ENTRY_DELAY_MASK,
            (*hg_config).ldo_entry_delay << CS35L33_LDO_ENTRY_DELAY_SHIFT);
        if (*hg_config).vp_hg_auto {
            regmap_update_bits((*priv_).regmap, CS35L33_HG_EN, CS35L33_VP_HG_AUTO_MASK, CS35L33_VP_HG_AUTO_MASK);
            snd_soc_dapm_add_routes(dapm, cs35l33_vphg_auto_route.as_ptr(), cs35l33_vphg_auto_route.len() as c_int);
        }
        regmap_update_bits((*priv_).regmap, CS35L33_HG_EN, CS35L33_VP_HG_MASK,
            (*hg_config).vp_hg << CS35L33_VP_HG_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_LDO_DEL, CS35L33_VP_HG_RATE_MASK,
            (*hg_config).vp_hg_rate << CS35L33_VP_HG_RATE_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_LDO_DEL, CS35L33_VP_HG_VA_MASK,
            (*hg_config).vp_hg_va << CS35L33_VP_HG_VA_SHIFT);
        regmap_update_bits((*priv_).regmap, CS35L33_HG_EN, CS35L33_CLASS_HG_EN_MASK, CS35L33_CLASS_HG_EN_MASK);
    }
    0
}

pub unsafe extern "C" fn cs35l33_set_bst_ipk(component: *mut snd_soc_component, mut bst: c_uint) -> c_int {
    let cs35l33 = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;
    let mut ret: c_int = 0;
    let mut steps: c_int = 0;

    /* Boost current in uA */
    if bst > 3600000 || bst < 1850000 {
        dev_err((*component).dev, "Invalid boost current %d\n\0".as_ptr() as *const c_char, bst);
        ret = -EINVAL;
        return ret;
    }

    if (bst % 15625) != 0 {
        dev_err((*component).dev, "Current not a multiple of 15625uA (%d)\n\0".as_ptr() as *const c_char, bst);
        ret = -EINVAL;
        return ret;
    }

    while bst > 1850000 {
        bst -= 15625;
        steps += 1;
    }

    regmap_write((*cs35l33).regmap, CS35L33_BST_PEAK_CTL, (steps + 0x70) as c_uint);
    ret
}

pub unsafe extern "C" fn cs35l33_probe(component: *mut snd_soc_component) -> c_int {
    let cs35l33 = snd_soc_component_get_drvdata(component) as *mut cs35l33_private;

    (*cs35l33).component = component;
    pm_runtime_get_sync((*component).dev);

    regmap_update_bits((*cs35l33).regmap, CS35L33_PROTECT_CTL, CS35L33_ALIVE_WD_DIS, 0x8);
    regmap_update_bits((*cs35l33).regmap, CS35L33_BST_CTL2, CS35L33_ALIVE_WD_DIS2, CS35L33_ALIVE_WD_DIS2);

    /* Set Platform Data */
    regmap_update_bits((*cs35l33).regmap, CS35L33_BST_CTL1, CS35L33_BST_CTL_MASK, (*cs35l33).pdata.boost_ctl);
    regmap_update_bits((*cs35l33).regmap, CS35L33_CLASSD_CTL, CS35L33_AMP_DRV_SEL_MASK,
        (*cs35l33).pdata.amp_drv_sel << CS35L33_AMP_DRV_SEL_SHIFT);

    if (*cs35l33).pdata.boost_ipk != 0 {
        cs35l33_set_bst_ipk(component, (*cs35l33).pdata.boost_ipk);
    }

    if (*cs35l33).enable_soft_ramp {
        snd_soc_component_update_bits(component, CS35L33_DAC_CTL, CS35L33_DIGSFT, CS35L33_DIGSFT);
        snd_soc_component_update_bits(component, CS35L33_DAC_CTL, CS35L33_DSR_RATE, (*cs35l33).pdata.ramp_rate);
    } else {
        snd_soc_component_update_bits(component, CS35L33_DAC_CTL, CS35L33_DIGSFT, 0);
    }

    /* update IMON scaling rate if different from default of 0x8 */
    if (*cs35l33).pdata.imon_adc_scale != 0x8 {
        snd_soc_component_update_bits(component, CS35L33_ADC_CTL, CS35L33_IMON_SCALE, (*cs35l33).pdata.imon_adc_scale);
    }

    cs35l33_set_hg_data(component, &mut (*cs35l33).pdata);

    /*
     * unmask important interrupts that causes the chip to enter
     * speaker safe mode and hence deserves user attention
     */
    regmap_update_bits((*cs35l33).regmap, CS35L33_INT_MASK_1,
        CS35L33_M_OTE | CS35L33_M_OTW | CS35L33_M_AMP_SHORT | CS35L33_M_CAL_ERR, 0);

    pm_runtime_put_sync((*component).dev);
    0
}

pub static soc_component_dev_cs35l33: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(cs35l33_probe),
    set_bias_level: Some(cs35l33_set_bias_level),
    set_sysclk: Some(cs35l33_component_set_sysclk),
    controls: cs35l33_snd_controls.as_ptr(),
    num_controls: cs35l33_snd_controls.len() as c_uint,
    dapm_widgets: cs35l33_dapm_widgets.as_ptr(),
    num_dapm_widgets: cs35l33_dapm_widgets.len() as c_uint,
    dapm_routes: cs35l33_audio_map.as_ptr(),
    num_dapm_routes: cs35l33_audio_map.len() as c_uint,
    use_pmdown_time: 1,
    endianness: 1,
};

pub static cs35l33_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: CS35L33_MAX_REGISTER,
    reg_defaults: cs35l33_reg.as_ptr(),
    num_reg_defaults: cs35l33_reg.len() as c_uint,
    volatile_reg: Some(cs35l33_volatile_register),
    readable_reg: Some(cs35l33_readable_register),
    writeable_reg: Some(cs35l33_writeable_register),
    cache_type: REGCACHE_MAPLE,
    use_single_read: true,
    use_single_write: true,
};

pub unsafe extern "C" fn cs35l33_runtime_resume(dev: *mut device) -> c_int {
    let cs35l33 = dev_get_drvdata(dev) as *mut cs35l33_private;
    let mut ret: c_int;

    dev_dbg(dev, "%s\n\0".as_ptr() as *const c_char, "__func__\0".as_ptr() as *const c_char);
    gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);

    ret = regulator_bulk_enable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(dev, "Failed to enable core supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regcache_cache_only((*cs35l33).regmap, false);
    gpiod_set_value_cansleep((*cs35l33).reset_gpio, 1);
    msleep(CS35L33_BOOT_DELAY);

    ret = regcache_sync((*cs35l33).regmap);
    if ret != 0 {
        dev_err(dev, "Failed to restore register cache\n\0".as_ptr() as *const c_char);
        regcache_cache_only((*cs35l33).regmap, true);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    if (*cs35l33).irq_requested {
        enable_irq((*to_i2c_client(dev)).irq);
    }
    0
}

pub unsafe extern "C" fn cs35l33_runtime_suspend(dev: *mut device) -> c_int {
    let cs35l33 = dev_get_drvdata(dev) as *mut cs35l33_private;

    dev_dbg(dev, "%s\n\0".as_ptr() as *const c_char, "__func__\0".as_ptr() as *const c_char);

    /* redo the calibration in next power up */
    (*cs35l33).amp_cal = false;

    /* Drain and block the threaded IRQ before cache_only/power-off. */
    if (*cs35l33).irq_requested {
        disable_irq((*to_i2c_client(dev)).irq);
    }

    regcache_cache_only((*cs35l33).regmap, true);
    regcache_mark_dirty((*cs35l33).regmap);
    regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
    0
}

pub static cs35l33_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(cs35l33_runtime_suspend),
    runtime_resume: Some(cs35l33_runtime_resume),
    runtime_idle: None,
};

pub unsafe extern "C" fn cs35l33_get_hg_data(
    np: *const device_node,
    pdata: *mut cs35l33_pdata,
) -> c_int {
    let mut hg: *mut device_node;
    let hg_config = &mut (*pdata).hg_config as *mut cs35l33_hg;
    let mut val32: u32 = 0;

    hg = of_get_child_by_name(np, c"cirrus,hg-algo".as_ptr());
    (*hg_config).enable_hg_algo = !hg.is_null();

    if (*hg_config).enable_hg_algo {
        if of_property_read_u32(hg, c"cirrus,mem-depth".as_ptr(), &mut val32) >= 0 {
            (*hg_config).mem_depth = val32;
        }
        if of_property_read_u32(hg, c"cirrus,release-rate".as_ptr(), &mut val32) >= 0 {
            (*hg_config).release_rate = val32;
        }
        if of_property_read_u32(hg, c"cirrus,ldo-thld".as_ptr(), &mut val32) >= 0 {
            (*hg_config).ldo_thld = val32;
        }
        if of_property_read_u32(hg, c"cirrus,ldo-path-disable".as_ptr(), &mut val32) >= 0 {
            (*hg_config).ldo_path_disable = val32;
        }
        if of_property_read_u32(hg, c"cirrus,ldo-entry-delay".as_ptr(), &mut val32) >= 0 {
            (*hg_config).ldo_entry_delay = val32;
        }

        (*hg_config).vp_hg_auto = of_property_read_bool(hg, c"cirrus,vp-hg-auto".as_ptr());

        if of_property_read_u32(hg, c"cirrus,vp-hg".as_ptr(), &mut val32) >= 0 {
            (*hg_config).vp_hg = val32;
        }
        if of_property_read_u32(hg, c"cirrus,vp-hg-rate".as_ptr(), &mut val32) >= 0 {
            (*hg_config).vp_hg_rate = val32;
        }
        if of_property_read_u32(hg, c"cirrus,vp-hg-va".as_ptr(), &mut val32) >= 0 {
            (*hg_config).vp_hg_va = val32;
        }
    }

    of_node_put(hg);
    0
}

pub unsafe extern "C" fn cs35l33_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let cs35l33 = data as *mut cs35l33_private;
    let component = (*cs35l33).component;
    let mut sticky_val1: c_uint = 0;
    let mut sticky_val2: c_uint = 0;
    let mut current_val: c_uint = 0;
    let mut mask1: c_uint = 0;
    let mut mask2: c_uint = 0;

    regmap_read((*cs35l33).regmap, CS35L33_INT_STATUS_2, &mut sticky_val2);
    regmap_read((*cs35l33).regmap, CS35L33_INT_STATUS_1, &mut sticky_val1);
    regmap_read((*cs35l33).regmap, CS35L33_INT_MASK_2, &mut mask2);
    regmap_read((*cs35l33).regmap, CS35L33_INT_MASK_1, &mut mask1);

    /* Check to see if the unmasked bits are active,
     *  if not then exit.
     */
    if (sticky_val1 & !mask1) == 0 && (sticky_val2 & !mask2) == 0 {
        return IRQ_NONE;
    }

    regmap_read((*cs35l33).regmap, CS35L33_INT_STATUS_1, &mut current_val);

    /* handle the interrupts */

    if (sticky_val1 & CS35L33_AMP_SHORT) != 0 {
        dev_crit((*component).dev, "Amp short error\n\0".as_ptr() as *const c_char);
        if (current_val & CS35L33_AMP_SHORT) == 0 {
            dev_dbg((*component).dev, "Amp short error release\n\0".as_ptr() as *const c_char);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_AMP_SHORT_RLS, 0);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_AMP_SHORT_RLS, CS35L33_AMP_SHORT_RLS);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_AMP_SHORT_RLS, 0);
        }
    }

    if (sticky_val1 & CS35L33_CAL_ERR) != 0 {
        dev_err((*component).dev, "Cal error\n\0".as_ptr() as *const c_char);

        /* redo the calibration in next power up */
        (*cs35l33).amp_cal = false;

        if (current_val & CS35L33_CAL_ERR) == 0 {
            dev_dbg((*component).dev, "Cal error release\n\0".as_ptr() as *const c_char);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_CAL_ERR_RLS, 0);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_CAL_ERR_RLS, CS35L33_CAL_ERR_RLS);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_CAL_ERR_RLS, 0);
        }
    }

    if (sticky_val1 & CS35L33_OTE) != 0 {
        dev_crit((*component).dev, "Over temperature error\n\0".as_ptr() as *const c_char);
        if (current_val & CS35L33_OTE) == 0 {
            dev_dbg((*component).dev, "Over temperature error release\n\0".as_ptr() as *const c_char);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTE_RLS, 0);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTE_RLS, CS35L33_OTE_RLS);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTE_RLS, 0);
        }
    }

    if (sticky_val1 & CS35L33_OTW) != 0 {
        dev_err((*component).dev, "Over temperature warning\n\0".as_ptr() as *const c_char);
        if (current_val & CS35L33_OTW) == 0 {
            dev_dbg((*component).dev, "Over temperature warning release\n\0".as_ptr() as *const c_char);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTW_RLS, 0);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTW_RLS, CS35L33_OTW_RLS);
            regmap_update_bits((*cs35l33).regmap, CS35L33_AMP_CTL, CS35L33_OTW_RLS, 0);
        }
    }
    if (CS35L33_ALIVE_ERR & sticky_val1) != 0 {
        dev_err((*component).dev, "ERROR: ADSPCLK Interrupt\n\0".as_ptr() as *const c_char);
    }
    if (CS35L33_MCLK_ERR & sticky_val1) != 0 {
        dev_err((*component).dev, "ERROR: MCLK Interrupt\n\0".as_ptr() as *const c_char);
    }
    if (CS35L33_VMON_OVFL & sticky_val2) != 0 {
        dev_err((*component).dev, "ERROR: VMON Overflow Interrupt\n\0".as_ptr() as *const c_char);
    }
    if (CS35L33_IMON_OVFL & sticky_val2) != 0 {
        dev_err((*component).dev, "ERROR: IMON Overflow Interrupt\n\0".as_ptr() as *const c_char);
    }
    if (CS35L33_VPMON_OVFL & sticky_val2) != 0 {
        dev_err((*component).dev, "ERROR: VPMON Overflow Interrupt\n\0".as_ptr() as *const c_char);
    }
    IRQ_HANDLED
}

pub static cs35l33_core_supplies: [*const c_char; 2] = [
    c"VA".as_ptr(),
    c"VP".as_ptr(),
];

pub unsafe extern "C" fn cs35l33_of_get_pdata(
    dev: *mut device,
    cs35l33: *mut cs35l33_private,
) -> c_int {
    let np = (*dev).of_node;
    let pdata = &mut (*cs35l33).pdata as *mut cs35l33_pdata;
    let mut val32: u32 = 0;

    if np.is_null() {
        return 0;
    }

    if of_property_read_u32(np, c"cirrus,boost-ctl".as_ptr(), &mut val32) >= 0 {
        (*pdata).boost_ctl = val32;
        (*pdata).amp_drv_sel = 1;
    }

    if of_property_read_u32(np, c"cirrus,ramp-rate".as_ptr(), &mut val32) >= 0 {
        (*pdata).ramp_rate = val32;
        (*cs35l33).enable_soft_ramp = true;
    }

    if of_property_read_u32(np, c"cirrus,boost-ipk".as_ptr(), &mut val32) >= 0 {
        (*pdata).boost_ipk = val32;
    }

    if of_property_read_u32(np, c"cirrus,imon-adc-scale".as_ptr(), &mut val32) >= 0 {
        if val32 == 0x0 || val32 == 0x7 || val32 == 0x6 {
            (*pdata).imon_adc_scale = val32;
        } else {
            /* use default value */
            (*pdata).imon_adc_scale = 0x8;
        }
    } else {
        /* use default value */
        (*pdata).imon_adc_scale = 0x8;
    }

    cs35l33_get_hg_data(np, pdata);
    0
}

pub unsafe extern "C" fn cs35l33_i2c_probe(i2c_client: *mut i2c_client) -> c_int {
    let mut cs35l33: *mut cs35l33_private;
    let mut pdata = dev_get_platdata(&mut (*i2c_client).dev) as *mut cs35l33_pdata;
    let mut ret: c_int;
    let mut devid: c_int;
    let mut i: c_int;
    let mut reg: c_uint = 0;

    cs35l33 = devm_kzalloc(&mut (*i2c_client).dev, core::mem::size_of::<cs35l33_private>(), GFP_KERNEL) as *mut cs35l33_private;
    if cs35l33.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c_client, cs35l33 as *mut c_void);
    (*cs35l33).regmap = devm_regmap_init_i2c(i2c_client, &cs35l33_regmap);
    if IS_ERR((*cs35l33).regmap as *const c_void) {
        ret = PTR_ERR((*cs35l33).regmap as *const c_void);
        dev_err(&mut (*i2c_client).dev, "regmap_init() failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    regcache_cache_only((*cs35l33).regmap, true);

    i = 0;
    while (i as usize) < cs35l33_core_supplies.len() {
        (*cs35l33).core_supplies[i as usize].supply = cs35l33_core_supplies[i as usize];
        i += 1;
    }
    (*cs35l33).num_core_supplies = cs35l33_core_supplies.len() as c_int;

    ret = devm_regulator_bulk_get(&mut (*i2c_client).dev, (*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, "Failed to request core supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    if !pdata.is_null() {
        (*cs35l33).pdata = *pdata;
    } else {
        cs35l33_of_get_pdata(&mut (*i2c_client).dev, cs35l33);
        pdata = &mut (*cs35l33).pdata;
    }

    ret = devm_request_threaded_irq(
        &mut (*i2c_client).dev,
        (*i2c_client).irq,
        None,
        Some(cs35l33_irq_thread),
        IRQF_ONESHOT | IRQF_TRIGGER_LOW,
        c"cs35l33".as_ptr(),
        cs35l33 as *mut c_void,
    );
    if ret != 0 {
        dev_warn(&mut (*i2c_client).dev, "Failed to request IRQ: %d\n\0".as_ptr() as *const c_char, ret);
    } else {
        (*cs35l33).irq_requested = true;
    }

    /* We could issue !RST or skip it based on AMP topology */
    (*cs35l33).reset_gpio = devm_gpiod_get_optional(&mut (*i2c_client).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*cs35l33).reset_gpio as *const c_void) {
        dev_err(&mut (*i2c_client).dev, "%s ERROR: Can't get reset GPIO\n\0".as_ptr() as *const c_char,
            "__func__\0".as_ptr() as *const c_char);
        return PTR_ERR((*cs35l33).reset_gpio as *const c_void);
    }

    ret = regulator_bulk_enable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c_client).dev, "Failed to enable core supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    gpiod_set_value_cansleep((*cs35l33).reset_gpio, 1);
    msleep(CS35L33_BOOT_DELAY);
    regcache_cache_only((*cs35l33).regmap, false);

    /* initialize codec */
    devid = cirrus_read_device_id((*cs35l33).regmap, CS35L33_DEVID_AB);
    if devid < 0 {
        ret = devid;
        dev_err(&mut (*i2c_client).dev, "Failed to read device ID: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    if devid != CS35L33_CHIP_ID as c_int {
        dev_err(&mut (*i2c_client).dev, "CS35L33 Device ID (%X). Expected ID %X\n\0".as_ptr() as *const c_char,
            devid, CS35L33_CHIP_ID);
        ret = -EINVAL;
        gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    ret = regmap_read((*cs35l33).regmap, CS35L33_REV_ID, &mut reg);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, "Get Revision ID failed\n\0".as_ptr() as *const c_char);
        gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    dev_info(&mut (*i2c_client).dev, "Cirrus Logic CS35L33, Revision: %02X\n\0".as_ptr() as *const c_char, reg & 0xFF);

    ret = regmap_register_patch((*cs35l33).regmap, cs35l33_patch.as_ptr(), cs35l33_patch.len() as c_int);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, "Error in applying regmap patch: %d\n\0".as_ptr() as *const c_char, ret);
        gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    /* disable mclk and tdm */
    regmap_update_bits((*cs35l33).regmap, CS35L33_CLK_CTL,
        CS35L33_MCLKDIS | CS35L33_SDOUT_3ST_TDM,
        CS35L33_MCLKDIS | CS35L33_SDOUT_3ST_TDM);

    pm_runtime_set_autosuspend_delay(&mut (*i2c_client).dev, 100);
    pm_runtime_use_autosuspend(&mut (*i2c_client).dev);
    pm_runtime_set_active(&mut (*i2c_client).dev);
    pm_runtime_enable(&mut (*i2c_client).dev);

    ret = devm_snd_soc_register_component(&mut (*i2c_client).dev, &soc_component_dev_cs35l33, &mut cs35l33_dai, 1);
    if ret < 0 {
        dev_err(&mut (*i2c_client).dev, "%s: Register component failed\n\0".as_ptr() as *const c_char,
            "__func__\0".as_ptr() as *const c_char);
        gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);
        regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
        return ret;
    }

    0
}

pub unsafe extern "C" fn cs35l33_i2c_remove(client: *mut i2c_client) {
    let cs35l33 = i2c_get_clientdata(client) as *mut cs35l33_private;

    gpiod_set_value_cansleep((*cs35l33).reset_gpio, 0);

    pm_runtime_disable(&mut (*client).dev);
    regulator_bulk_disable((*cs35l33).num_core_supplies, (*cs35l33).core_supplies.as_mut_ptr());
}

pub static cs35l33_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"cirrus,cs35l33".as_ptr() },
    of_device_id { compatible: NULL },
];
MODULE_DEVICE_TABLE!(of, cs35l33_of_match);

pub static cs35l33_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"cs35l33".as_ptr() },
    i2c_device_id { name: NULL },
];
MODULE_DEVICE_TABLE!(i2c, cs35l33_id);

pub static mut cs35l33_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"cs35l33".as_ptr(),
        pm: pm_ptr(&cs35l33_pm_ops),
        of_match_table: cs35l33_of_match.as_ptr(),
    },
    id_table: cs35l33_id.as_ptr(),
    probe: Some(cs35l33_i2c_probe),
    remove: Some(cs35l33_i2c_remove),
};
module_i2c_driver!(cs35l33_i2c_driver);

MODULE_DESCRIPTION!("ASoC CS35L33 driver");
MODULE_AUTHOR!("Paul Handrigan, Cirrus Logic Inc, <paul.handrigan@cirrus.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
