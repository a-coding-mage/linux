// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * max98390.rs  --  MAX98390 ALSA Soc Audio driver
 *
 * Copyright (C) 2020 Maxim Integrated Products
 *
 * Rust translation of max98390.c. Linux kernel headers and max98390.h provide
 * the external types, constants, macros, and helper functions referenced here.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const MAX98390_RATES: c_uint = SNDRV_PCM_RATE_8000_48000;
const MAX98390_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static max98390_reg_defaults: [reg_default; 143] = [
    reg_default { reg: MAX98390_INT_EN1, def: 0xf0 },
    reg_default { reg: MAX98390_INT_EN2, def: 0x00 },
    reg_default { reg: MAX98390_INT_EN3, def: 0x00 },
    reg_default { reg: MAX98390_INT_FLAG_CLR1, def: 0x00 },
    reg_default { reg: MAX98390_INT_FLAG_CLR2, def: 0x00 },
    reg_default { reg: MAX98390_INT_FLAG_CLR3, def: 0x00 },
    reg_default { reg: MAX98390_IRQ_CTRL, def: 0x01 },
    reg_default { reg: MAX98390_CLK_MON, def: 0x6d },
    reg_default { reg: MAX98390_DAT_MON, def: 0x03 },
    reg_default { reg: MAX98390_WDOG_CTRL, def: 0x00 },
    reg_default { reg: MAX98390_WDOG_RST, def: 0x00 },
    reg_default { reg: MAX98390_MEAS_ADC_THERM_WARN_THRESH, def: 0x75 },
    reg_default { reg: MAX98390_MEAS_ADC_THERM_SHDN_THRESH, def: 0x8c },
    reg_default { reg: MAX98390_MEAS_ADC_THERM_HYSTERESIS, def: 0x08 },
    reg_default { reg: MAX98390_PIN_CFG, def: 0x55 },
    reg_default { reg: MAX98390_PCM_RX_EN_A, def: 0x00 },
    reg_default { reg: MAX98390_PCM_RX_EN_B, def: 0x00 },
    reg_default { reg: MAX98390_PCM_TX_EN_A, def: 0x00 },
    reg_default { reg: MAX98390_PCM_TX_EN_B, def: 0x00 },
    reg_default { reg: MAX98390_PCM_TX_HIZ_CTRL_A, def: 0xff },
    reg_default { reg: MAX98390_PCM_TX_HIZ_CTRL_B, def: 0xff },
    reg_default { reg: MAX98390_PCM_CH_SRC_1, def: 0x00 },
    reg_default { reg: MAX98390_PCM_CH_SRC_2, def: 0x00 },
    reg_default { reg: MAX98390_PCM_CH_SRC_3, def: 0x00 },
    reg_default { reg: MAX98390_PCM_MODE_CFG, def: 0xc0 },
    reg_default { reg: MAX98390_PCM_MASTER_MODE, def: 0x1c },
    reg_default { reg: MAX98390_PCM_CLK_SETUP, def: 0x44 },
    reg_default { reg: MAX98390_PCM_SR_SETUP, def: 0x08 },
    reg_default { reg: MAX98390_ICC_RX_EN_A, def: 0x00 },
    reg_default { reg: MAX98390_ICC_RX_EN_B, def: 0x00 },
    reg_default { reg: MAX98390_ICC_TX_EN_A, def: 0x00 },
    reg_default { reg: MAX98390_ICC_TX_EN_B, def: 0x00 },
    reg_default { reg: MAX98390_ICC_HIZ_MANUAL_MODE, def: 0x00 },
    reg_default { reg: MAX98390_ICC_TX_HIZ_EN_A, def: 0x00 },
    reg_default { reg: MAX98390_ICC_TX_HIZ_EN_B, def: 0x00 },
    reg_default { reg: MAX98390_ICC_LNK_EN, def: 0x00 },
    reg_default { reg: MAX98390_R2039_AMP_DSP_CFG, def: 0x0f },
    reg_default { reg: MAX98390_R203A_AMP_EN, def: 0x81 },
    reg_default { reg: MAX98390_TONE_GEN_DC_CFG, def: 0x00 },
    reg_default { reg: MAX98390_SPK_SRC_SEL, def: 0x00 },
    reg_default { reg: MAX98390_SSM_CFG, def: 0x85 },
    reg_default { reg: MAX98390_MEAS_EN, def: 0x03 },
    reg_default { reg: MAX98390_MEAS_DSP_CFG, def: 0x0f },
    reg_default { reg: MAX98390_BOOST_CTRL0, def: 0x1c },
    reg_default { reg: MAX98390_BOOST_CTRL3, def: 0x01 },
    reg_default { reg: MAX98390_BOOST_CTRL1, def: 0x40 },
    reg_default { reg: MAX98390_MEAS_ADC_CFG, def: 0x07 },
    reg_default { reg: MAX98390_MEAS_ADC_BASE_MSB, def: 0x00 },
    reg_default { reg: MAX98390_MEAS_ADC_BASE_LSB, def: 0x23 },
    reg_default { reg: MAX98390_ADC_CH0_DIVIDE, def: 0x00 },
    reg_default { reg: MAX98390_ADC_CH1_DIVIDE, def: 0x00 },
    reg_default { reg: MAX98390_ADC_CH2_DIVIDE, def: 0x00 },
    reg_default { reg: MAX98390_ADC_CH0_FILT_CFG, def: 0x00 },
    reg_default { reg: MAX98390_ADC_CH1_FILT_CFG, def: 0x00 },
    reg_default { reg: MAX98390_ADC_CH2_FILT_CFG, def: 0x00 },
    reg_default { reg: MAX98390_PWR_GATE_CTL, def: 0x2c },
    reg_default { reg: MAX98390_BROWNOUT_EN, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_INFINITE_HOLD, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_INFINITE_HOLD_CLR, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL_HOLD, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL1_THRESH, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL2_THRESH, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL3_THRESH, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL4_THRESH, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_THRESH_HYSTERYSIS, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_AMP_LIMITER_ATK_REL, def: 0x1f },
    reg_default { reg: MAX98390_BROWNOUT_AMP_GAIN_ATK_REL, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_AMP1_CLIP_MODE, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL1_CUR_LIMIT, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL1_AMP1_CTRL1, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL1_AMP1_CTRL2, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL1_AMP1_CTRL3, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL2_CUR_LIMIT, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL2_AMP1_CTRL1, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL2_AMP1_CTRL2, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL2_AMP1_CTRL3, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL3_CUR_LIMIT, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL3_AMP1_CTRL1, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL3_AMP1_CTRL2, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL3_AMP1_CTRL3, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL4_CUR_LIMIT, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL4_AMP1_CTRL1, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL4_AMP1_CTRL2, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LVL4_AMP1_CTRL3, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_ILIM_HLD, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_LIM_HLD, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_CLIP_HLD, def: 0x00 },
    reg_default { reg: MAX98390_BROWNOUT_GAIN_HLD, def: 0x00 },
    reg_default { reg: MAX98390_ENV_TRACK_VOUT_HEADROOM, def: 0x0f },
    reg_default { reg: MAX98390_ENV_TRACK_BOOST_VOUT_DELAY, def: 0x80 },
    reg_default { reg: MAX98390_ENV_TRACK_REL_RATE, def: 0x07 },
    reg_default { reg: MAX98390_ENV_TRACK_HOLD_RATE, def: 0x07 },
    reg_default { reg: MAX98390_ENV_TRACK_CTRL, def: 0x01 },
    reg_default { reg: MAX98390_BOOST_BYPASS1, def: 0x49 },
    reg_default { reg: MAX98390_BOOST_BYPASS2, def: 0x2b },
    reg_default { reg: MAX98390_BOOST_BYPASS3, def: 0x08 },
    reg_default { reg: MAX98390_FET_SCALING1, def: 0x00 },
    reg_default { reg: MAX98390_FET_SCALING2, def: 0x03 },
    reg_default { reg: MAX98390_FET_SCALING3, def: 0x00 },
    reg_default { reg: MAX98390_FET_SCALING4, def: 0x07 },
    reg_default { reg: MAX98390_SPK_SPEEDUP, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_RELEASE_TIME_1, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_RELEASE_TIME_2, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_ATTACK_TIME_1, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_ATTACK_TIME_2, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_COMPRESSION_RATIO, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_COMPRESSION_THRESHOLD, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_MAKEUPGAIN, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_NOISE_GATE_THRESHOLD, def: 0x00 },
    reg_default { reg: DSMIG_WBDRC_HPF_ENABLE, def: 0x00 },
    reg_default { reg: DSMIG_WB_DRC_TEST_SMOOTHER_OUT_EN, def: 0x00 },
    reg_default { reg: DSMIG_PPR_THRESHOLD, def: 0x00 },
    reg_default { reg: DSM_STEREO_BASS_CHANNEL_SELECT, def: 0x00 },
    reg_default { reg: DSM_TPROT_THRESHOLD_BYTE0, def: 0x00 },
    reg_default { reg: DSM_TPROT_THRESHOLD_BYTE1, def: 0x00 },
    reg_default { reg: DSM_TPROT_ROOM_TEMPERATURE_BYTE0, def: 0x00 },
    reg_default { reg: DSM_TPROT_ROOM_TEMPERATURE_BYTE1, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_RDC_ROOM_BYTE0, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_RDC_ROOM_BYTE1, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_RDC_ROOM_BYTE2, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_TCONST_BYTE0, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_TCONST_BYTE1, def: 0x00 },
    reg_default { reg: DSM_TPROT_RECIP_TCONST_BYTE2, def: 0x00 },
    reg_default { reg: DSM_THERMAL_ATTENUATION_SETTINGS, def: 0x00 },
    reg_default { reg: DSM_THERMAL_PILOT_TONE_ATTENUATION, def: 0x00 },
    reg_default { reg: DSM_TPROT_PG_TEMP_THRESH_BYTE0, def: 0x00 },
    reg_default { reg: DSM_TPROT_PG_TEMP_THRESH_BYTE1, def: 0x00 },
    reg_default { reg: DSMIG_DEBUZZER_THRESHOLD, def: 0x00 },
    reg_default { reg: DSMIG_DEBUZZER_ALPHA_COEF_TEST_ONLY, def: 0x08 },
    reg_default { reg: DSM_VOL_ENA, def: 0x20 },
    reg_default { reg: DSM_VOL_CTRL, def: 0xa0 },
    reg_default { reg: DSMIG_EN, def: 0x00 },
    reg_default { reg: MAX98390_R23E1_DSP_GLOBAL_EN, def: 0x00 },
    reg_default { reg: MAX98390_R23FF_GLOBAL_EN, def: 0x00 },
];

unsafe fn max98390_dai_set_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    let mode: c_uint;
    let format: c_uint;
    let mut invert: c_uint = 0;

    dev_dbg((*component).dev, c"%s: fmt 0x%08X\n".as_ptr(), c"max98390_dai_set_fmt".as_ptr(), fmt);

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => mode = MAX98390_PCM_MASTER_MODE_SLAVE,
        SND_SOC_DAIFMT_CBP_CFP => {
            (*max98390).provider = true;
            mode = MAX98390_PCM_MASTER_MODE_MASTER;
        }
        _ => {
            dev_err((*component).dev, c"DAI clock mode unsupported\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98390).regmap, MAX98390_PCM_MASTER_MODE,
        MAX98390_PCM_MASTER_MODE_MASK, mode);

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => invert = MAX98390_PCM_MODE_CFG_PCM_BCLKEDGE,
        _ => {
            dev_err((*component).dev, c"DAI invert mode unsupported\n".as_ptr());
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98390).regmap, MAX98390_PCM_MODE_CFG,
        MAX98390_PCM_MODE_CFG_PCM_BCLKEDGE, invert);

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => format = MAX98390_PCM_FORMAT_I2S,
        SND_SOC_DAIFMT_LEFT_J => format = MAX98390_PCM_FORMAT_LJ,
        SND_SOC_DAIFMT_DSP_A => format = MAX98390_PCM_FORMAT_TDM_MODE1,
        SND_SOC_DAIFMT_DSP_B => format = MAX98390_PCM_FORMAT_TDM_MODE0,
        _ => return -EINVAL,
    }

    regmap_update_bits((*max98390).regmap, MAX98390_PCM_MODE_CFG,
        MAX98390_PCM_MODE_CFG_FORMAT_MASK,
        format << MAX98390_PCM_MODE_CFG_FORMAT_SHIFT);

    0
}

fn max98390_get_bclk_sel(bclk: c_int) -> c_int {
    let bclk_sel_table: [c_int; 10] = [32, 48, 64, 96, 128, 192, 256, 320, 384, 512];
    /* match BCLKs per LRCLK */
    for i in 0..bclk_sel_table.len() {
        if bclk_sel_table[i] == bclk {
            return i as c_int + 2;
        }
    }
    0
}

unsafe fn max98390_set_clock(component: *mut snd_soc_component,
    params: *mut snd_pcm_hw_params) -> c_int {
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    /* codec MCLK rate in master mode */
    let rate_table: [c_int; 10] = [
        5644800, 6000000, 6144000, 6500000, 9600000, 11289600, 12000000,
        12288000, 13000000, 19200000,
    ];
    /* BCLK/LRCLK ratio calculation */
    let blr_clk_ratio = params_channels(params) * snd_pcm_format_width(params_format(params));
    let value: c_int;

    if (*max98390).provider {
        let mut i = 0usize;
        /* match rate to closest value */
        while i < rate_table.len() {
            if rate_table[i] as c_uint >= (*max98390).sysclk {
                break;
            }
            i += 1;
        }
        if i == rate_table.len() {
            dev_err((*component).dev, c"failed to find proper clock rate.\n".as_ptr());
            return -EINVAL;
        }
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_MASTER_MODE,
            MAX98390_PCM_MASTER_MODE_MCLK_MASK,
            (i as c_uint) << MAX98390_PCM_MASTER_MODE_MCLK_RATE_SHIFT);
    }

    if !(*max98390).tdm_mode {
        /* BCLK configuration */
        value = max98390_get_bclk_sel(blr_clk_ratio);
        if value == 0 {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), params_format(params));
            return -EINVAL;
        }
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_CLK_SETUP,
            MAX98390_PCM_CLK_SETUP_BSEL_MASK, value as c_uint);
    }
    0
}

unsafe fn max98390_dai_hw_params(_substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let component = (*dai).component;
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    let sampling_rate: c_uint;
    let chan_sz: c_uint;

    /* pcm mode configuration */
    match snd_pcm_format_width(params_format(params)) {
        16 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), params_format(params));
            return -EINVAL;
        }
    }

    regmap_update_bits((*max98390).regmap, MAX98390_PCM_MODE_CFG,
        MAX98390_PCM_MODE_CFG_CHANSZ_MASK, chan_sz);
    dev_dbg((*component).dev, c"format supported %d".as_ptr(), params_format(params));

    /* sampling rate configuration */
    match params_rate(params) {
        8000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_8000,
        11025 => sampling_rate = MAX98390_PCM_SR_SET1_SR_11025,
        12000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_12000,
        16000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_16000,
        22050 => sampling_rate = MAX98390_PCM_SR_SET1_SR_22050,
        24000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_24000,
        32000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_32000,
        44100 => sampling_rate = MAX98390_PCM_SR_SET1_SR_44100,
        48000 => sampling_rate = MAX98390_PCM_SR_SET1_SR_48000,
        _ => {
            dev_err((*component).dev, c"rate %d not supported\n".as_ptr(), params_rate(params));
            return -EINVAL;
        }
    }

    /* set DAI_SR to correct LRCLK frequency */
    regmap_update_bits((*max98390).regmap, MAX98390_PCM_SR_SETUP,
        MAX98390_PCM_SR_SET1_SR_MASK, sampling_rate);
    max98390_set_clock(component, params)
}

unsafe fn max98390_dai_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint,
    rx_mask: c_uint, slots: c_int, slot_width: c_int) -> c_int {
    let component = (*dai).component;
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    let bsel: c_int;
    let chan_sz: c_uint;

    (*max98390).tdm_mode = !(!tx_mask != 0 && !rx_mask != 0 && slots == 0 && slot_width == 0);
    if tx_mask == 0 && rx_mask == 0 && slots == 0 && slot_width == 0 {
        (*max98390).tdm_mode = false;
    } else {
        (*max98390).tdm_mode = true;
    }

    dev_dbg((*component).dev, c"Tdm mode : %d\n".as_ptr(), (*max98390).tdm_mode as c_int);

    /* BCLK configuration */
    bsel = max98390_get_bclk_sel(slots * slot_width);
    if bsel == 0 {
        dev_err((*component).dev, c"BCLK %d not supported\n".as_ptr(), slots * slot_width);
        return -EINVAL;
    }
    regmap_update_bits((*max98390).regmap, MAX98390_PCM_CLK_SETUP,
        MAX98390_PCM_CLK_SETUP_BSEL_MASK, bsel as c_uint);

    /* Channel size configuration */
    match slot_width {
        16 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_16,
        24 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_24,
        32 => chan_sz = MAX98390_PCM_MODE_CFG_CHANSZ_32,
        _ => {
            dev_err((*component).dev, c"format unsupported %d\n".as_ptr(), slot_width);
            return -EINVAL;
        }
    }
    regmap_update_bits((*max98390).regmap, MAX98390_PCM_MODE_CFG,
        MAX98390_PCM_MODE_CFG_CHANSZ_MASK, chan_sz);

    /* Rx slot configuration */
    regmap_write((*max98390).regmap, MAX98390_PCM_RX_EN_A, rx_mask & 0xFF);
    regmap_write((*max98390).regmap, MAX98390_PCM_RX_EN_B, (rx_mask & 0xFF00) >> 8);

    /* Tx slot Hi-Z configuration */
    regmap_write((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_A, (!tx_mask) & 0xFF);
    regmap_write((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_B, ((!tx_mask) & 0xFF00) >> 8);
    0
}

unsafe fn max98390_dai_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int,
    freq: c_uint, _dir: c_int) -> c_int {
    let component = (*dai).component;
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    (*max98390).sysclk = freq;
    0
}

static max98390_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(max98390_dai_set_sysclk),
    set_fmt: Some(max98390_dai_set_fmt),
    hw_params: Some(max98390_dai_hw_params),
    set_tdm_slot: Some(max98390_dai_tdm_slot),
};

unsafe fn max98390_dac_event(w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*max98390).regmap, MAX98390_R203A_AMP_EN,
                MAX98390_AMP_EN_MASK, 1);
            regmap_update_bits((*max98390).regmap, MAX98390_R23FF_GLOBAL_EN,
                MAX98390_GLOBAL_EN_MASK, 1);
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*max98390).regmap, MAX98390_R23FF_GLOBAL_EN,
                MAX98390_GLOBAL_EN_MASK, 0);
            regmap_update_bits((*max98390).regmap, MAX98390_R203A_AMP_EN,
                MAX98390_AMP_EN_MASK, 0);
        }
        _ => {}
    }
    0
}

static max98390_switch_text: [*const c_char; 3] = [
    c"Left".as_ptr(), c"Right".as_ptr(), c"LeftRight".as_ptr(),
];

static max98390_boost_voltage_text: [*const c_char; 29] = [
    c"6.5V".as_ptr(), c"6.625V".as_ptr(), c"6.75V".as_ptr(), c"6.875V".as_ptr(),
    c"7V".as_ptr(), c"7.125V".as_ptr(), c"7.25V".as_ptr(), c"7.375V".as_ptr(),
    c"7.5V".as_ptr(), c"7.625V".as_ptr(), c"7.75V".as_ptr(), c"7.875V".as_ptr(),
    c"8V".as_ptr(), c"8.125V".as_ptr(), c"8.25V".as_ptr(), c"8.375V".as_ptr(),
    c"8.5V".as_ptr(), c"8.625V".as_ptr(), c"8.75V".as_ptr(), c"8.875V".as_ptr(),
    c"9V".as_ptr(), c"9.125V".as_ptr(), c"9.25V".as_ptr(), c"9.375V".as_ptr(),
    c"9.5V".as_ptr(), c"9.625V".as_ptr(), c"9.75V".as_ptr(), c"9.875V".as_ptr(),
    c"10V".as_ptr(),
];

static max98390_current_limit_text: [*const c_char; 66] = [
    c"0.00A".as_ptr(), c"0.50A".as_ptr(), c"1.00A".as_ptr(), c"1.05A".as_ptr(),
    c"1.10A".as_ptr(), c"1.15A".as_ptr(), c"1.20A".as_ptr(), c"1.25A".as_ptr(),
    c"1.30A".as_ptr(), c"1.35A".as_ptr(), c"1.40A".as_ptr(), c"1.45A".as_ptr(),
    c"1.50A".as_ptr(), c"1.55A".as_ptr(), c"1.60A".as_ptr(), c"1.65A".as_ptr(),
    c"1.70A".as_ptr(), c"1.75A".as_ptr(), c"1.80A".as_ptr(), c"1.85A".as_ptr(),
    c"1.90A".as_ptr(), c"1.95A".as_ptr(), c"2.00A".as_ptr(), c"2.05A".as_ptr(),
    c"2.10A".as_ptr(), c"2.15A".as_ptr(), c"2.20A".as_ptr(), c"2.25A".as_ptr(),
    c"2.30A".as_ptr(), c"2.35A".as_ptr(), c"2.40A".as_ptr(), c"2.45A".as_ptr(),
    c"2.50A".as_ptr(), c"2.55A".as_ptr(), c"2.60A".as_ptr(), c"2.65A".as_ptr(),
    c"2.70A".as_ptr(), c"2.75A".as_ptr(), c"2.80A".as_ptr(), c"2.85A".as_ptr(),
    c"2.90A".as_ptr(), c"2.95A".as_ptr(), c"3.00A".as_ptr(), c"3.05A".as_ptr(),
    c"3.10A".as_ptr(), c"3.15A".as_ptr(), c"3.20A".as_ptr(), c"3.25A".as_ptr(),
    c"3.30A".as_ptr(), c"3.35A".as_ptr(), c"3.40A".as_ptr(), c"3.45A".as_ptr(),
    c"3.50A".as_ptr(), c"3.55A".as_ptr(), c"3.60A".as_ptr(), c"3.65A".as_ptr(),
    c"3.70A".as_ptr(), c"3.75A".as_ptr(), c"3.80A".as_ptr(), c"3.85A".as_ptr(),
    c"3.90A".as_ptr(), c"3.95A".as_ptr(), c"4.00A".as_ptr(), c"4.05A".as_ptr(),
    c"4.10A".as_ptr(),
];

/* SOC_ENUM_SINGLE_DECL, DECLARE_TLV_DB_SCALE, SOC_* and SND_SOC_DAPM_* are
 * Linux ASoC macros. Their C initializers are preserved here as macro-style
 * external items supplied by the future Rust kernel binding layer.
 */
static max98390_boost_voltage: soc_enum =
    SOC_ENUM_SINGLE(MAX98390_BOOST_CTRL0, 0, 29, max98390_boost_voltage_text.as_ptr());
static max98390_spk_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(300, 300, 0);
static max98390_digital_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-8000, 50, 0);
static max98390_current_limit: soc_enum =
    SOC_ENUM_SINGLE(MAX98390_BOOST_CTRL1, 0, 66, max98390_current_limit_text.as_ptr());

unsafe fn max98390_ref_rdc_put(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    (*max98390).ref_rdc_value = (*ucontrol).value.integer.value[0] as c_uint;
    regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE0,
        (*max98390).ref_rdc_value & 0x000000ff);
    regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE1,
        ((*max98390).ref_rdc_value >> 8) & 0x000000ff);
    regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE2,
        ((*max98390).ref_rdc_value >> 16) & 0x000000ff);
    0
}

unsafe fn max98390_ref_rdc_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    (*ucontrol).value.integer.value[0] = (*max98390).ref_rdc_value as _;
    0
}

unsafe fn max98390_ambient_temp_put(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    (*max98390).ambient_temp_value = (*ucontrol).value.integer.value[0] as c_uint;
    regmap_write((*max98390).regmap, DSM_TPROT_ROOM_TEMPERATURE_BYTE1,
        ((*max98390).ambient_temp_value >> 8) & 0x000000ff);
    regmap_write((*max98390).regmap, DSM_TPROT_ROOM_TEMPERATURE_BYTE0,
        (*max98390).ambient_temp_value & 0x000000ff);
    0
}

unsafe fn max98390_ambient_temp_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    (*ucontrol).value.integer.value[0] = (*max98390).ambient_temp_value as _;
    0
}

unsafe fn max98390_adaptive_rdc_put(kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    dev_warn((*component).dev, c"Put adaptive rdc not supported\n".as_ptr());
    0
}

unsafe fn max98390_adaptive_rdc_get(kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mut rdc: c_uint = 0;
    let mut rdc0: c_uint = 0;
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    regmap_read((*max98390).regmap, THERMAL_RDC_RD_BACK_BYTE1, &mut rdc);
    regmap_read((*max98390).regmap, THERMAL_RDC_RD_BACK_BYTE0, &mut rdc0);
    (*ucontrol).value.integer.value[0] = (rdc0 | (rdc << 8)) as _;
    0
}

unsafe fn max98390_dsm_calib_get(_kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    /* Do nothing */
    0
}

unsafe fn max98390_dsm_calib_put(kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut rdc: c_uint = 0;
    let mut rdc_cal_result: c_uint = 0;
    let mut temp: c_uint = 0;
    let mut val: c_uint = 0;

    snd_soc_dapm_mutex_lock(dapm);
    regmap_read((*max98390).regmap, MAX98390_R23FF_GLOBAL_EN, &mut val);
    if val == 0 {
        /* Enable the codec for the duration of calibration readout */
        regmap_update_bits((*max98390).regmap, MAX98390_R203A_AMP_EN, MAX98390_AMP_EN_MASK, 1);
        regmap_update_bits((*max98390).regmap, MAX98390_R23FF_GLOBAL_EN, MAX98390_GLOBAL_EN_MASK, 1);
    }
    regmap_read((*max98390).regmap, THERMAL_RDC_RD_BACK_BYTE1, &mut rdc);
    regmap_read((*max98390).regmap, THERMAL_RDC_RD_BACK_BYTE0, &mut rdc_cal_result);
    regmap_read((*max98390).regmap, MAX98390_MEAS_ADC_CH2_READ, &mut temp);
    if val == 0 {
        /* Disable the codec if it was disabled */
        regmap_update_bits((*max98390).regmap, MAX98390_R23FF_GLOBAL_EN, MAX98390_GLOBAL_EN_MASK, 0);
        regmap_update_bits((*max98390).regmap, MAX98390_R203A_AMP_EN, MAX98390_AMP_EN_MASK, 0);
    }
    snd_soc_dapm_mutex_unlock(dapm);

    rdc_cal_result |= (rdc << 8) & 0x0000FFFF;
    if rdc_cal_result != 0 {
        (*max98390).ref_rdc_value = 268435456u32 / rdc_cal_result;
    }
    (*max98390).ambient_temp_value = temp.wrapping_mul(52).wrapping_sub(1188);
    let rdc_integer = rdc_cal_result.wrapping_mul(937) / 65536;
    let rdc_factor = ((rdc_cal_result.wrapping_mul(937).wrapping_mul(100)) / 65536)
        .wrapping_sub(rdc_integer.wrapping_mul(100));
    dev_info((*component).dev,
        c"rdc resistance about %d.%02d ohm, reg=0x%X temp reg=0x%X\n".as_ptr(),
        rdc_integer, rdc_factor, rdc_cal_result, temp);
    0
}

static max98390_snd_controls: [snd_kcontrol_new; 11] = [
    SOC_SINGLE_TLV!("Digital Volume", DSM_VOL_CTRL, 0, 184, 0, max98390_digital_tlv),
    SOC_SINGLE_TLV!("Speaker Volume", MAX98390_R203D_SPK_GAIN, 0, 6, 0, max98390_spk_tlv),
    SOC_SINGLE!("Ramp Up Bypass Switch", MAX98390_R2039_AMP_DSP_CFG, MAX98390_AMP_DSP_CFG_RMP_UP_SHIFT, 1, 0),
    SOC_SINGLE!("Ramp Down Bypass Switch", MAX98390_R2039_AMP_DSP_CFG, MAX98390_AMP_DSP_CFG_RMP_DN_SHIFT, 1, 0),
    SOC_SINGLE!("Boost Clock Phase", MAX98390_BOOST_CTRL3, MAX98390_BOOST_CLK_PHASE_CFG_SHIFT, 3, 0),
    SOC_ENUM!("Boost Output Voltage", max98390_boost_voltage),
    SOC_ENUM!("Current Limit", max98390_current_limit),
    SOC_SINGLE_EXT!("DSM Rdc", SND_SOC_NOPM, 0, 0xffffff, 0, max98390_ref_rdc_get, max98390_ref_rdc_put),
    SOC_SINGLE_EXT!("DSM Ambient Temp", SND_SOC_NOPM, 0, 0xffff, 0, max98390_ambient_temp_get, max98390_ambient_temp_put),
    SOC_SINGLE_EXT!("DSM Adaptive Rdc", SND_SOC_NOPM, 0, 0xffff, 0, max98390_adaptive_rdc_get, max98390_adaptive_rdc_put),
    SOC_SINGLE_EXT!("DSM Calibration", SND_SOC_NOPM, 0, 1, 0, max98390_dsm_calib_get, max98390_dsm_calib_put),
];

static dai_sel_enum: soc_enum =
    SOC_ENUM_SINGLE(MAX98390_PCM_CH_SRC_1, MAX98390_PCM_RX_CH_SRC_SHIFT, 3, max98390_switch_text.as_ptr());
static max98390_dai_controls: snd_kcontrol_new = SOC_DAPM_ENUM!("DAI Sel", dai_sel_enum);

static max98390_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    SND_SOC_DAPM_DAC_E!("Amp Enable", "HiFi Playback", SND_SOC_NOPM, 0, 0,
        max98390_dac_event, SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX!("DAI Sel Mux", SND_SOC_NOPM, 0, 0, &max98390_dai_controls),
    SND_SOC_DAPM_OUTPUT!("BE_OUT"),
];

static max98390_audio_map: [snd_soc_dapm_route; 4] = [
    /* Plabyack */
    snd_soc_dapm_route { sink: c"DAI Sel Mux".as_ptr(), control: c"Left".as_ptr(), source: c"Amp Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAI Sel Mux".as_ptr(), control: c"Right".as_ptr(), source: c"Amp Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"DAI Sel Mux".as_ptr(), control: c"LeftRight".as_ptr(), source: c"Amp Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"BE_OUT".as_ptr(), control: ptr::null(), source: c"DAI Sel Mux".as_ptr() },
];

fn max98390_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    (MAX98390_SOFTWARE_RESET..=MAX98390_INT_EN3).contains(&reg)
        || (MAX98390_IRQ_CTRL..=MAX98390_WDOG_CTRL).contains(&reg)
        || (MAX98390_MEAS_ADC_THERM_WARN_THRESH..=MAX98390_BROWNOUT_INFINITE_HOLD).contains(&reg)
        || (MAX98390_BROWNOUT_LVL_HOLD..=DSMIG_DEBUZZER_THRESHOLD).contains(&reg)
        || (DSM_VOL_ENA..=MAX98390_R24FF_REV_ID).contains(&reg)
}

fn max98390_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    (MAX98390_SOFTWARE_RESET..=MAX98390_INT_EN3).contains(&reg)
        || (MAX98390_MEAS_ADC_CH0_READ..=MAX98390_MEAS_ADC_CH2_READ).contains(&reg)
        || (MAX98390_PWR_GATE_STATUS..=MAX98390_BROWNOUT_STATUS).contains(&reg)
        || reg == MAX98390_BROWNOUT_LOWEST_STATUS
        || reg == MAX98390_ENV_TRACK_BOOST_VOUT_READ
        || (DSM_STBASS_HPF_B0_BYTE0..=DSM_DEBUZZER_ATTACK_TIME_BYTE2).contains(&reg)
        || (THERMAL_RDC_RD_BACK_BYTE1..=DSMIG_DEBUZZER_THRESHOLD).contains(&reg)
        || (DSM_THERMAL_GAIN..=DSM_WBDRC_GAIN).contains(&reg)
}

static mut max98390_dai: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"max98390-aif1".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MAX98390_RATES,
        formats: MAX98390_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"HiFi Capture".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: MAX98390_RATES,
        formats: MAX98390_FORMATS,
    },
    ops: &max98390_dai_ops,
}];

unsafe fn max98390_dsm_init(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let param_size: c_int;
    let param_start_addr: c_int;
    let mut filename = [0i8; 128];
    let vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    let product = dmi_get_system_info(DMI_PRODUCT_NAME);
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    let mut fw: *const firmware = ptr::null();

    if strcmp((*max98390).dsm_param_name, c"default".as_ptr()) == 0 {
        if !vendor.is_null() && !product.is_null() {
            snprintf(filename.as_mut_ptr(), filename.len(), c"dsm_param_%s_%s.bin".as_ptr(), vendor, product);
        } else {
            sprintf(filename.as_mut_ptr(), c"dsm_param.bin".as_ptr());
        }
    } else {
        snprintf(filename.as_mut_ptr(), filename.len(), c"%s".as_ptr(), (*max98390).dsm_param_name);
    }

    ret = request_firmware(&mut fw, filename.as_ptr(), (*component).dev);
    if ret != 0 {
        ret = request_firmware(&mut fw, c"dsm_param.bin".as_ptr(), (*component).dev);
        if ret != 0 {
            ret = request_firmware(&mut fw, c"dsmparam.bin".as_ptr(), (*component).dev);
            if ret != 0 {
                return ret;
            }
        }
    }

    dev_dbg((*component).dev, c"max98390: param fw size %zd\n".as_ptr(), (*fw).size);
    if (*fw).size < MAX98390_DSM_PARAM_MIN_SIZE as usize {
        dev_err((*component).dev, c"param fw is invalid.\n".as_ptr());
        return -EINVAL;
    }
    let mut dsm_param = (*fw).data as *mut c_char;
    param_start_addr = ((*dsm_param.add(0) as c_int) & 0xff)
        | (((*dsm_param.add(1) as c_int) & 0xff) << 8);
    param_size = ((*dsm_param.add(2) as c_int) & 0xff)
        | (((*dsm_param.add(3) as c_int) & 0xff) << 8);
    if param_size > MAX98390_DSM_PARAM_MAX_SIZE as c_int
        || param_start_addr < MAX98390_IRQ_CTRL as c_int
        || (*fw).size < (param_size as usize + MAX98390_DSM_PAYLOAD_OFFSET as usize) {
        dev_err((*component).dev, c"param fw is invalid.\n".as_ptr());
        release_firmware(fw);
        return -EINVAL;
    }
    regmap_write((*max98390).regmap, MAX98390_R203A_AMP_EN, 0x80);
    dsm_param = dsm_param.add(MAX98390_DSM_PAYLOAD_OFFSET as usize);
    regmap_bulk_write((*max98390).regmap, param_start_addr as c_uint,
        dsm_param as *const c_void, param_size as usize);
    regmap_write((*max98390).regmap, MAX98390_R23E1_DSP_GLOBAL_EN, 0x01);
    release_firmware(fw);
    0
}

unsafe fn max98390_init_regs(component: *mut snd_soc_component) {
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    regmap_write((*max98390).regmap, MAX98390_CLK_MON, 0x6f);
    regmap_write((*max98390).regmap, MAX98390_DAT_MON, 0x00);
    regmap_write((*max98390).regmap, MAX98390_PWR_GATE_CTL, 0x00);
    regmap_write((*max98390).regmap, MAX98390_PCM_RX_EN_A, 0x03);
    regmap_write((*max98390).regmap, MAX98390_ENV_TRACK_VOUT_HEADROOM, 0x0e);
    regmap_write((*max98390).regmap, MAX98390_BOOST_BYPASS1, 0x46);
    regmap_write((*max98390).regmap, MAX98390_FET_SCALING3, 0x03);

    /* voltage, current slot configuration */
    regmap_write((*max98390).regmap, MAX98390_PCM_CH_SRC_2,
        (((*max98390).i_l_slot << 4) | (*max98390).v_l_slot) & 0xFF);

    if (*max98390).v_l_slot < 8 {
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_A,
            1 << (*max98390).v_l_slot, 0);
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_EN_A,
            1 << (*max98390).v_l_slot, 1 << (*max98390).v_l_slot);
    } else {
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_B,
            1 << ((*max98390).v_l_slot - 8), 0);
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_EN_B,
            1 << ((*max98390).v_l_slot - 8), 1 << ((*max98390).v_l_slot - 8));
    }

    if (*max98390).i_l_slot < 8 {
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_A,
            1 << (*max98390).i_l_slot, 0);
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_EN_A,
            1 << (*max98390).i_l_slot, 1 << (*max98390).i_l_slot);
    } else {
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_HIZ_CTRL_B,
            1 << ((*max98390).i_l_slot - 8), 0);
        regmap_update_bits((*max98390).regmap, MAX98390_PCM_TX_EN_B,
            1 << ((*max98390).i_l_slot - 8), 1 << ((*max98390).i_l_slot - 8));
    }
}

unsafe fn max98390_probe(component: *mut snd_soc_component) -> c_int {
    let max98390 = snd_soc_component_get_drvdata(component) as *mut max98390_priv;
    regmap_write((*max98390).regmap, MAX98390_SOFTWARE_RESET, 0x01);
    /* Sleep reset settle time */
    msleep(20);
    /* Amp init setting */
    max98390_init_regs(component);
    /* Update dsm bin param */
    max98390_dsm_init(component);
    /* Dsm Setting */
    if (*max98390).ref_rdc_value != 0 {
        regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE0, (*max98390).ref_rdc_value & 0x000000ff);
        regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE1, ((*max98390).ref_rdc_value >> 8) & 0x000000ff);
        regmap_write((*max98390).regmap, DSM_TPROT_RECIP_RDC_ROOM_BYTE2, ((*max98390).ref_rdc_value >> 16) & 0x000000ff);
    }
    if (*max98390).ambient_temp_value != 0 {
        regmap_write((*max98390).regmap, DSM_TPROT_ROOM_TEMPERATURE_BYTE1, ((*max98390).ambient_temp_value >> 8) & 0x000000ff);
        regmap_write((*max98390).regmap, DSM_TPROT_ROOM_TEMPERATURE_BYTE0, (*max98390).ambient_temp_value & 0x000000ff);
    }
    0
}

unsafe fn max98390_suspend(dev: *mut device) -> c_int {
    let max98390 = dev_get_drvdata(dev) as *mut max98390_priv;
    dev_dbg(dev, c"%s:Enter\n".as_ptr(), c"max98390_suspend".as_ptr());
    regcache_cache_only((*max98390).regmap, true);
    regcache_mark_dirty((*max98390).regmap);
    0
}

unsafe fn max98390_resume(dev: *mut device) -> c_int {
    let max98390 = dev_get_drvdata(dev) as *mut max98390_priv;
    dev_dbg(dev, c"%s:Enter\n".as_ptr(), c"max98390_resume".as_ptr());
    regcache_cache_only((*max98390).regmap, false);
    let ret = regcache_sync((*max98390).regmap);
    if ret != 0 {
        regcache_cache_only((*max98390).regmap, true);
        regcache_mark_dirty((*max98390).regmap);
        return ret;
    }
    0
}

static max98390_pm: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(max98390_suspend, max98390_resume) */
    suspend: Some(max98390_suspend),
    resume: Some(max98390_resume),
};

static soc_codec_dev_max98390: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98390_probe),
    controls: max98390_snd_controls.as_ptr(),
    num_controls: max98390_snd_controls.len() as c_uint,
    dapm_widgets: max98390_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98390_dapm_widgets.len() as c_uint,
    dapm_routes: max98390_audio_map.as_ptr(),
    num_dapm_routes: max98390_audio_map.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static max98390_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    max_register: MAX98390_R24FF_REV_ID,
    reg_defaults: max98390_reg_defaults.as_ptr(),
    num_reg_defaults: max98390_reg_defaults.len() as c_uint,
    readable_reg: Some(max98390_readable_register),
    volatile_reg: Some(max98390_volatile_reg),
    cache_type: REGCACHE_RBTREE,
};

unsafe fn max98390_slot_config(i2c: *mut i2c_client, max98390: *mut max98390_priv) {
    let mut value: c_uint = 0;
    let dev = &mut (*i2c).dev as *mut device;
    if device_property_read_u32(dev, c"maxim,vmon-slot-no".as_ptr(), &mut value) == 0 {
        (*max98390).v_l_slot = value & 0xF;
    } else {
        (*max98390).v_l_slot = 0;
    }
    if device_property_read_u32(dev, c"maxim,imon-slot-no".as_ptr(), &mut value) == 0 {
        (*max98390).i_l_slot = value & 0xF;
    } else {
        (*max98390).i_l_slot = 1;
    }
}

unsafe fn max98390_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut ret: c_int = 0;
    let mut reg: c_uint = 0;
    let adapter = (*i2c).adapter;
    let reset_gpio: *mut gpio_desc;

    if !i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_BYTE_DATA) {
        dev_err(&mut (*i2c).dev, c"I2C check functionality failed\n".as_ptr());
        return -ENXIO;
    }

    let max98390 = devm_kzalloc(&mut (*i2c).dev,
        core::mem::size_of::<max98390_priv>(), GFP_KERNEL) as *mut max98390_priv;
    if max98390.is_null() {
        ret = -ENOMEM;
        return ret;
    }
    i2c_set_clientdata(i2c, max98390 as *mut c_void);

    ret = device_property_read_u32(&mut (*i2c).dev, c"maxim,temperature_calib".as_ptr(),
        &mut (*max98390).ambient_temp_value);
    if ret != 0 {
        dev_info(&mut (*i2c).dev,
            c"no optional property 'temperature_calib' found, default:\n".as_ptr());
    }
    ret = device_property_read_u32(&mut (*i2c).dev, c"maxim,r0_calib".as_ptr(),
        &mut (*max98390).ref_rdc_value);
    if ret != 0 {
        dev_info(&mut (*i2c).dev,
            c"no optional property 'r0_calib' found, default:\n".as_ptr());
    }
    dev_info(&mut (*i2c).dev, c"%s: r0_calib: 0x%x,temperature_calib: 0x%x".as_ptr(),
        c"max98390_i2c_probe".as_ptr(), (*max98390).ref_rdc_value,
        (*max98390).ambient_temp_value);

    ret = device_property_read_string(&mut (*i2c).dev, c"maxim,dsm_param_name".as_ptr(),
        &mut (*max98390).dsm_param_name);
    if ret != 0 {
        (*max98390).dsm_param_name = c"default".as_ptr();
    }

    /* voltage/current slot configuration */
    max98390_slot_config(i2c, max98390);

    /* regmap initialization */
    (*max98390).regmap = devm_regmap_init_i2c(i2c, &max98390_regmap);
    if IS_ERR((*max98390).regmap as *const c_void) {
        ret = PTR_ERR((*max98390).regmap as *const c_void);
        dev_err(&mut (*i2c).dev, c"Failed to allocate regmap: %d\n".as_ptr(), ret);
        return ret;
    }

    reset_gpio = devm_gpiod_get_optional(&mut (*i2c).dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR(reset_gpio as *const c_void) {
        return dev_err_probe(&mut (*i2c).dev, PTR_ERR(reset_gpio as *const c_void),
            c"Failed to get reset gpio\n".as_ptr());
    }

    /* Power on device */
    if !reset_gpio.is_null() {
        usleep_range(1000, 2000);
        /* bring out of reset */
        gpiod_set_value_cansleep(reset_gpio, 0);
        usleep_range(1000, 2000);
    }

    /* Check Revision ID */
    ret = regmap_read((*max98390).regmap, MAX98390_R24FF_REV_ID, &mut reg);
    if ret != 0 {
        dev_err(&mut (*i2c).dev, c"ret=%d, Failed to read: 0x%02X\n".as_ptr(),
            ret, MAX98390_R24FF_REV_ID);
        return ret;
    }
    dev_info(&mut (*i2c).dev, c"MAX98390 revisionID: 0x%02X\n".as_ptr(), reg);

    devm_snd_soc_register_component(&mut (*i2c).dev, &soc_codec_dev_max98390,
        max98390_dai.as_mut_ptr(), max98390_dai.len() as c_int)
}

static max98390_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"max98390\0", driver_data: 0 },
    i2c_device_id { name: [0; I2C_NAME_SIZE], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, max98390_i2c_id);

/* #if defined(CONFIG_OF) */
static max98390_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"maxim,max98390".as_ptr(), data: ptr::null() },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
MODULE_DEVICE_TABLE!(of, max98390_of_match);
/* #endif */

/* #ifdef CONFIG_ACPI */
static max98390_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"MX98390\0", driver_data: 0 },
    acpi_device_id { id: [0; ACPI_ID_LEN], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(acpi, max98390_acpi_match);
/* #endif */

static mut max98390_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max98390".as_ptr(),
        of_match_table: of_match_ptr(max98390_of_match.as_ptr()),
        acpi_match_table: ACPI_PTR(max98390_acpi_match.as_ptr()),
        pm: pm_ptr(&max98390_pm),
    },
    probe: Some(max98390_i2c_probe),
    id_table: max98390_i2c_id.as_ptr(),
};

module_i2c_driver!(max98390_i2c_driver);

MODULE_DESCRIPTION!("ALSA SoC MAX98390 driver");
MODULE_AUTHOR!("Steve Lee <steves.lee@maximintegrated.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
