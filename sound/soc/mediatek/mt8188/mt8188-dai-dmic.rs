// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI DMIC I/F Control
 *
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 *         Parker Yang <parker.yang@mediatek.com>
 */

/* C dependencies:
 * linux/delay.h, linux/pm_runtime.h, linux/regmap.h, sound/pcm_params.h,
 * mt8188-afe-clk.h, mt8188-afe-common.h, mt8188-reg.h
 */

/* DMIC HW Gain configuration maximum value. */
const DMIC_GAIN_MAX_STEP: u32 = GENMASK(19, 0);
const DMIC_GAIN_MAX_PER_STEP: u32 = GENMASK(7, 0);
const DMIC_GAIN_MAX_TARGET: u32 = GENMASK(27, 0);
const DMIC_GAIN_MAX_CURRENT: u32 = GENMASK(27, 0);

const CLK_PHASE_SEL_CH1: u32 = 0;
const CLK_PHASE_SEL_CH2: u32 = CLK_PHASE_SEL_CH1 + 4;

const DMIC1_SRC_SEL: u32 = 0;
const DMIC2_SRC_SEL: u32 = 0;
const DMIC3_SRC_SEL: u32 = 2;
const DMIC4_SRC_SEL: u32 = 0;
const DMIC5_SRC_SEL: u32 = 4;
const DMIC6_SRC_SEL: u32 = 0;
const DMIC7_SRC_SEL: u32 = 6;
const DMIC8_SRC_SEL: u32 = 0;

const SUPPLY_SEQ_DMIC_GAIN: u32 = 0;
const SUPPLY_SEQ_DMIC_CK: u32 = 1;

const DMIC0: u32 = 0;
const DMIC1: u32 = 1;
const DMIC2: u32 = 2;
const DMIC3: u32 = 3;
const DMIC_NUM: usize = 4;

#[repr(C)]
struct mtk_dai_dmic_ctrl_reg {
    con0: u32,
}

#[repr(C)]
struct mtk_dai_dmic_hw_gain_ctrl_reg {
    bypass: u32,
    con0: u32,
}

#[repr(C)]
struct mtk_dai_dmic_priv {
    gain_on: [u32; DMIC_NUM],
    channels: u32,
    hires_required: bool,
}

static dmic_ctrl_regs: [mtk_dai_dmic_ctrl_reg; DMIC_NUM] = [
    mtk_dai_dmic_ctrl_reg {
        con0: AFE_DMIC0_UL_SRC_CON0,
    },
    mtk_dai_dmic_ctrl_reg {
        con0: AFE_DMIC1_UL_SRC_CON0,
    },
    mtk_dai_dmic_ctrl_reg {
        con0: AFE_DMIC2_UL_SRC_CON0,
    },
    mtk_dai_dmic_ctrl_reg {
        con0: AFE_DMIC3_UL_SRC_CON0,
    },
];

unsafe fn get_dmic_ctrl_reg(id: i32) -> *const mtk_dai_dmic_ctrl_reg {
    if id < 0 || id >= DMIC_NUM as i32 {
        return core::ptr::null();
    }

    &dmic_ctrl_regs[id as usize] as *const mtk_dai_dmic_ctrl_reg
}

static dmic_hw_gain_ctrl_regs: [mtk_dai_dmic_hw_gain_ctrl_reg; DMIC_NUM] = [
    mtk_dai_dmic_hw_gain_ctrl_reg {
        bypass: DMIC_BYPASS_HW_GAIN,
        con0: DMIC_GAIN1_CON0,
    },
    mtk_dai_dmic_hw_gain_ctrl_reg {
        bypass: DMIC_BYPASS_HW_GAIN,
        con0: DMIC_GAIN2_CON0,
    },
    mtk_dai_dmic_hw_gain_ctrl_reg {
        bypass: DMIC_BYPASS_HW_GAIN,
        con0: DMIC_GAIN3_CON0,
    },
    mtk_dai_dmic_hw_gain_ctrl_reg {
        bypass: DMIC_BYPASS_HW_GAIN,
        con0: DMIC_GAIN4_CON0,
    },
];

unsafe fn get_dmic_hw_gain_ctrl_reg(
    afe: *mut mtk_base_afe,
    id: i32,
) -> *const mtk_dai_dmic_hw_gain_ctrl_reg {
    if id < 0 || id >= DMIC_NUM as i32 {
        dev_dbg!((*afe).dev, "%s invalid id\n", __func__);
        return core::ptr::null();
    }

    &dmic_hw_gain_ctrl_regs[id as usize] as *const mtk_dai_dmic_hw_gain_ctrl_reg
}

unsafe fn mtk_dai_dmic_hw_gain_bypass(afe: *mut mtk_base_afe, id: u32, bypass: bool) {
    let reg: *const mtk_dai_dmic_hw_gain_ctrl_reg;
    let msk: u32;

    reg = get_dmic_hw_gain_ctrl_reg(afe, id as i32);
    if reg.is_null() {
        return;
    }

    match id {
        DMIC0 => {
            msk = DMIC_BYPASS_HW_GAIN_DMIC1_BYPASS;
        }
        DMIC1 => {
            msk = DMIC_BYPASS_HW_GAIN_DMIC2_BYPASS;
        }
        DMIC2 => {
            msk = DMIC_BYPASS_HW_GAIN_DMIC3_BYPASS;
        }
        DMIC3 => {
            msk = DMIC_BYPASS_HW_GAIN_DMIC4_BYPASS;
        }
        _ => {
            return;
        }
    }

    if bypass {
        regmap_set_bits((*afe).regmap, (*reg).bypass, msk);
    } else {
        regmap_clear_bits((*afe).regmap, (*reg).bypass, msk);
    }
}

unsafe fn mtk_dai_dmic_hw_gain_on(afe: *mut mtk_base_afe, id: u32, on: bool) {
    let reg: *const mtk_dai_dmic_hw_gain_ctrl_reg = get_dmic_hw_gain_ctrl_reg(afe, id as i32);

    if reg.is_null() {
        return;
    }

    if on {
        regmap_set_bits((*afe).regmap, (*reg).con0, DMIC_GAIN_CON0_GAIN_ON);
    } else {
        regmap_clear_bits((*afe).regmap, (*reg).con0, DMIC_GAIN_CON0_GAIN_ON);
    }
}

static mtk_dai_dmic_iir_coeff_reg_defaults: [reg_sequence; 20] = [
    reg_sequence { reg: AFE_DMIC0_IIR_COEF_02_01, def: 0x00000000 },
    reg_sequence { reg: AFE_DMIC0_IIR_COEF_04_03, def: 0x00003FB8 },
    reg_sequence { reg: AFE_DMIC0_IIR_COEF_06_05, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC0_IIR_COEF_08_07, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC0_IIR_COEF_10_09, def: 0x0000C048 },
    reg_sequence { reg: AFE_DMIC1_IIR_COEF_02_01, def: 0x00000000 },
    reg_sequence { reg: AFE_DMIC1_IIR_COEF_04_03, def: 0x00003FB8 },
    reg_sequence { reg: AFE_DMIC1_IIR_COEF_06_05, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC1_IIR_COEF_08_07, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC1_IIR_COEF_10_09, def: 0x0000C048 },
    reg_sequence { reg: AFE_DMIC2_IIR_COEF_02_01, def: 0x00000000 },
    reg_sequence { reg: AFE_DMIC2_IIR_COEF_04_03, def: 0x00003FB8 },
    reg_sequence { reg: AFE_DMIC2_IIR_COEF_06_05, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC2_IIR_COEF_08_07, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC2_IIR_COEF_10_09, def: 0x0000C048 },
    reg_sequence { reg: AFE_DMIC3_IIR_COEF_02_01, def: 0x00000000 },
    reg_sequence { reg: AFE_DMIC3_IIR_COEF_04_03, def: 0x00003FB8 },
    reg_sequence { reg: AFE_DMIC3_IIR_COEF_06_05, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC3_IIR_COEF_08_07, def: 0x3FB80000 },
    reg_sequence { reg: AFE_DMIC3_IIR_COEF_10_09, def: 0x0000C048 },
];

unsafe fn mtk_dai_dmic_load_iir_coeff_table(afe: *mut mtk_base_afe) -> i32 {
    regmap_multi_reg_write(
        (*afe).regmap,
        mtk_dai_dmic_iir_coeff_reg_defaults.as_ptr(),
        mtk_dai_dmic_iir_coeff_reg_defaults.len(),
    )
}

unsafe fn mtk_dai_dmic_configure_array(dai: *mut snd_soc_dai) -> i32 {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let mask: u32 = PWR2_TOP_CON_DMIC8_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC7_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC6_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC5_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC4_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC3_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC2_SRC_SEL_MASK
        | PWR2_TOP_CON_DMIC1_SRC_SEL_MASK;
    let val: u32 = PWR2_TOP_CON_DMIC8_SRC_SEL_VAL(DMIC8_SRC_SEL)
        | PWR2_TOP_CON_DMIC7_SRC_SEL_VAL(DMIC7_SRC_SEL)
        | PWR2_TOP_CON_DMIC6_SRC_SEL_VAL(DMIC6_SRC_SEL)
        | PWR2_TOP_CON_DMIC5_SRC_SEL_VAL(DMIC5_SRC_SEL)
        | PWR2_TOP_CON_DMIC4_SRC_SEL_VAL(DMIC4_SRC_SEL)
        | PWR2_TOP_CON_DMIC3_SRC_SEL_VAL(DMIC3_SRC_SEL)
        | PWR2_TOP_CON_DMIC2_SRC_SEL_VAL(DMIC2_SRC_SEL)
        | PWR2_TOP_CON_DMIC1_SRC_SEL_VAL(DMIC1_SRC_SEL);

    regmap_update_bits((*afe).regmap, PWR2_TOP_CON0, mask, val)
}

/* This function assumes that the caller checked that channels is valid */
fn mtk_dmic_channels_to_dmic_number(channels: u32) -> u8 {
    match channels {
        1 => DMIC0 as u8,
        2 => DMIC1 as u8,
        3 => DMIC2 as u8,
        4 | _ => DMIC3 as u8,
    }
}

unsafe fn mtk_dai_dmic_hw_gain_enable(afe: *mut mtk_base_afe, channels: u32, enable: bool) {
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let dmic_num: u8;
    let mut i: i32;

    dmic_num = mtk_dmic_channels_to_dmic_number(channels);
    i = dmic_num as i32;
    while i >= DMIC0 as i32 {
        if enable && (*dmic_priv).gain_on[i as usize] != 0 {
            mtk_dai_dmic_hw_gain_bypass(afe, i as u32, false);
            mtk_dai_dmic_hw_gain_on(afe, i as u32, true);
        } else {
            mtk_dai_dmic_hw_gain_on(afe, i as u32, false);
            mtk_dai_dmic_hw_gain_bypass(afe, i as u32, true);
        }
        i -= 1;
    }
}

unsafe fn mtk_dmic_gain_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let cmpnt: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let channels: u32 = (*dmic_priv).channels;

    dev_dbg!(
        (*afe).dev,
        "%s(), name %s, event 0x%x\n",
        __func__,
        (*w).name,
        event
    );

    if channels == 0 {
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mtk_dai_dmic_hw_gain_enable(afe, channels, true);
        }
        SND_SOC_DAPM_POST_PMD => {
            mtk_dai_dmic_hw_gain_enable(afe, channels, false);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_dmic_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let cmpnt: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let mut reg: *const mtk_dai_dmic_ctrl_reg = core::ptr::null();
    let channels: u32 = (*dmic_priv).channels;
    let mut msk: u32;
    let dmic_num: u8;
    let mut i: i32;

    dev_dbg!(
        (*afe).dev,
        "%s(), name %s, event 0x%x\n",
        __func__,
        (*w).name,
        event
    );

    if channels == 0 {
        return -EINVAL;
    }

    dmic_num = mtk_dmic_channels_to_dmic_number(channels);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* request fifo soft rst */
            msk = 0;
            i = dmic_num as i32;
            while i >= DMIC0 as i32 {
                msk |= PWR2_TOP_CON1_DMIC_FIFO_SOFT_RST_EN(i as u32);
                i -= 1;
            }

            regmap_set_bits((*afe).regmap, PWR2_TOP_CON1, msk);

            msk = AFE_DMIC_UL_SRC_CON0_UL_MODE_3P25M_CH1_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_MODE_3P25M_CH2_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_SDM_3_LEVEL_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_IIR_ON_TMP_CTL;

            i = dmic_num as i32;
            while i >= DMIC0 as i32 {
                reg = get_dmic_ctrl_reg(i);
                if !reg.is_null() {
                    regmap_set_bits((*afe).regmap, (*reg).con0, msk);
                }
                i -= 1;
            }
        }
        SND_SOC_DAPM_POST_PMU => {
            msk = AFE_DMIC_UL_SRC_CON0_UL_SRC_ON_TMP_CTL;

            i = dmic_num as i32;
            while i >= DMIC0 as i32 {
                reg = get_dmic_ctrl_reg(i);
                if !reg.is_null() {
                    regmap_set_bits((*afe).regmap, (*reg).con0, msk);
                }
                i -= 1;
            }

            if (*dmic_priv).hires_required {
                mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES1 as usize]);
                mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES2 as usize]);
                mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES3 as usize]);
                mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES4 as usize]);
            }

            mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC1 as usize]);
            mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC2 as usize]);
            mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC3 as usize]);
            mt8188_afe_enable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC4 as usize]);

            /* release fifo soft rst */
            msk = 0;
            i = dmic_num as i32;
            while i >= DMIC0 as i32 {
                msk |= PWR2_TOP_CON1_DMIC_FIFO_SOFT_RST_EN(i as u32);
                i -= 1;
            }

            regmap_clear_bits((*afe).regmap, PWR2_TOP_CON1, msk);
        }
        SND_SOC_DAPM_PRE_PMD => {
            msk = AFE_DMIC_UL_SRC_CON0_UL_MODE_3P25M_CH1_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_MODE_3P25M_CH2_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_SRC_ON_TMP_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_IIR_ON_TMP_CTL
                | AFE_DMIC_UL_SRC_CON0_UL_SDM_3_LEVEL_CTL;

            i = dmic_num as i32;
            while i >= DMIC0 as i32 {
                reg = get_dmic_ctrl_reg(i);
                if !reg.is_null() {
                    regmap_set_bits((*afe).regmap, (*reg).con0, msk);
                }
                i -= 1;
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 126);

            mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC1 as usize]);
            mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC2 as usize]);
            mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC3 as usize]);
            mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_AFE_DMIC4 as usize]);

            if (*dmic_priv).hires_required {
                mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES1 as usize]);
                mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES2 as usize]);
                mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES3 as usize]);
                mt8188_afe_disable_clk(afe, (*afe_priv).clk[MT8188_CLK_AUD_DMIC_HIRES4 as usize]);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_dai_dmic_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let rate: u32 = params_rate(params);
    let channels: u32 = params_channels(params);
    let mut reg: *const mtk_dai_dmic_ctrl_reg = core::ptr::null();
    let mut val: u32 = AFE_DMIC_UL_SRC_CON0_UL_PHASE_SEL_CH1(CLK_PHASE_SEL_CH1)
        | AFE_DMIC_UL_SRC_CON0_UL_PHASE_SEL_CH2(CLK_PHASE_SEL_CH2)
        | AFE_DMIC_UL_SRC_CON0_UL_IIR_MODE_CTL(0);
    let msk: u32 = AFE_DMIC_UL_SRC_CON0_UL_TWO_WIRE_MODE_CTL
        | AFE_DMIC_UL_SRC_CON0_UL_PHASE_SEL_MASK
        | AFE_DMIC_UL_SRC_CON0_UL_IIR_MODE_CTL_MASK
        | AFE_DMIC_UL_VOICE_MODE_MASK;
    let dmic_num: u8;
    let mut ret: i32;
    let mut i: i32;

    if channels == 0 || channels > 8 {
        return -EINVAL;
    }

    ret = mtk_dai_dmic_configure_array(dai);
    if ret < 0 {
        return ret;
    }

    ret = mtk_dai_dmic_load_iir_coeff_table(afe);
    if ret < 0 {
        return ret;
    }

    match rate {
        96000 => {
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_96K;
            (*dmic_priv).hires_required = true;
        }
        48000 => {
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_48K;
            (*dmic_priv).hires_required = false;
        }
        32000 => {
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_32K;
            (*dmic_priv).hires_required = false;
        }
        16000 => {
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_16K;
            (*dmic_priv).hires_required = false;
        }
        8000 => {
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_8K;
            (*dmic_priv).hires_required = false;
        }
        _ => {
            dev_dbg!(
                (*afe).dev,
                "%s invalid rate %u, use 48000Hz\n",
                __func__,
                rate
            );
            val |= AFE_DMIC_UL_CON0_VOCIE_MODE_48K;
            (*dmic_priv).hires_required = false;
        }
    }

    dmic_num = mtk_dmic_channels_to_dmic_number(channels);
    i = dmic_num as i32;
    while i >= DMIC0 as i32 {
        reg = get_dmic_ctrl_reg(i);
        if !reg.is_null() {
            ret = regmap_update_bits((*afe).regmap, (*reg).con0, msk, val);
            if ret < 0 {
                return ret;
            }
        }
        i -= 1;
    }

    (*dmic_priv).channels = channels;

    0
}

static mtk_dai_dmic_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_dmic_hw_params),
};

const MTK_DMIC_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000;

const MTK_DMIC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_dmic_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c_str!("DMIC"),
    id: MT8188_AFE_IO_DMIC_IN,
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("DMIC Capture"),
        channels_min: 1,
        channels_max: 8,
        rates: MTK_DMIC_RATES,
        formats: MTK_DMIC_FORMATS,
    },
    ops: &mtk_dai_dmic_ops,
}];

static mtk_dai_dmic_widgets: [snd_soc_dapm_widget; 11] = [
    SND_SOC_DAPM_MIXER!("I004", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I005", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I006", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I007", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I008", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I009", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I010", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I011", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(
        "DMIC_GAIN_ON",
        SUPPLY_SEQ_DMIC_GAIN,
        SND_SOC_NOPM,
        0,
        0,
        mtk_dmic_gain_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "DMIC_CK_ON",
        SUPPLY_SEQ_DMIC_CK,
        PWR2_TOP_CON1,
        PWR2_TOP_CON1_DMIC_CKDIV_ON_SHIFT,
        0,
        mtk_dmic_event,
        SND_SOC_DAPM_PRE_POST_PMU | SND_SOC_DAPM_PRE_POST_PMD
    ),
    SND_SOC_DAPM_INPUT!("DMIC_INPUT"),
];

static mtk_dai_dmic_routes: [snd_soc_dapm_route; 11] = [
    snd_soc_dapm_route { sink: c_str!("I004"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I005"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I006"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I007"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I008"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I009"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I010"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("I011"), control: core::ptr::null(), source: c_str!("DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("DMIC Capture"), control: core::ptr::null(), source: c_str!("DMIC_CK_ON") },
    snd_soc_dapm_route { sink: c_str!("DMIC Capture"), control: core::ptr::null(), source: c_str!("DMIC_GAIN_ON") },
    snd_soc_dapm_route { sink: c_str!("DMIC Capture"), control: core::ptr::null(), source: c_str!("DMIC_INPUT") },
];

static mt8188_dmic_gain_enable_text: [*const i8; 2] = [c_str!("Bypass"), c_str!("Connect")];

SOC_ENUM_SINGLE_EXT_DECL!(
    dmic_gain_on_enum,
    mt8188_dmic_gain_enable_text
);

unsafe fn mtk_dai_dmic_hw_gain_ctrl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(component);
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let source: u32 = (*ucontrol).value.enumerated.item[0];
    let cached: *mut u32;

    if source >= (*e).items {
        return -EINVAL;
    }

    if strcmp((*kcontrol).id.name, c_str!("DMIC1_HW_GAIN_EN")) == 0 {
        cached = &mut (*dmic_priv).gain_on[0] as *mut u32;
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC2_HW_GAIN_EN")) == 0 {
        cached = &mut (*dmic_priv).gain_on[1] as *mut u32;
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC3_HW_GAIN_EN")) == 0 {
        cached = &mut (*dmic_priv).gain_on[2] as *mut u32;
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC4_HW_GAIN_EN")) == 0 {
        cached = &mut (*dmic_priv).gain_on[3] as *mut u32;
    } else {
        return -EINVAL;
    }

    if source == *cached {
        return 0;
    }

    *cached = source;
    1
}

unsafe fn mtk_dai_dmic_hw_gain_ctrl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(component);
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv =
        (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] as *mut mtk_dai_dmic_priv;
    let val: u32;

    if strcmp((*kcontrol).id.name, c_str!("DMIC1_HW_GAIN_EN")) == 0 {
        val = (*dmic_priv).gain_on[0];
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC2_HW_GAIN_EN")) == 0 {
        val = (*dmic_priv).gain_on[1];
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC3_HW_GAIN_EN")) == 0 {
        val = (*dmic_priv).gain_on[2];
    } else if strcmp((*kcontrol).id.name, c_str!("DMIC4_HW_GAIN_EN")) == 0 {
        val = (*dmic_priv).gain_on[3];
    } else {
        return -EINVAL;
    }

    (*ucontrol).value.enumerated.item[0] = val;
    0
}

static mtk_dai_dmic_controls: [snd_kcontrol_new; 28] = [
    SOC_ENUM_EXT!("DMIC1_HW_GAIN_EN", dmic_gain_on_enum, mtk_dai_dmic_hw_gain_ctrl_get, mtk_dai_dmic_hw_gain_ctrl_put),
    SOC_ENUM_EXT!("DMIC2_HW_GAIN_EN", dmic_gain_on_enum, mtk_dai_dmic_hw_gain_ctrl_get, mtk_dai_dmic_hw_gain_ctrl_put),
    SOC_ENUM_EXT!("DMIC3_HW_GAIN_EN", dmic_gain_on_enum, mtk_dai_dmic_hw_gain_ctrl_get, mtk_dai_dmic_hw_gain_ctrl_put),
    SOC_ENUM_EXT!("DMIC4_HW_GAIN_EN", dmic_gain_on_enum, mtk_dai_dmic_hw_gain_ctrl_get, mtk_dai_dmic_hw_gain_ctrl_put),
    SOC_SINGLE!("DMIC1_HW_GAIN_TARGET", DMIC_GAIN1_CON1, 0, DMIC_GAIN_MAX_TARGET, 0),
    SOC_SINGLE!("DMIC2_HW_GAIN_TARGET", DMIC_GAIN2_CON1, 0, DMIC_GAIN_MAX_TARGET, 0),
    SOC_SINGLE!("DMIC3_HW_GAIN_TARGET", DMIC_GAIN3_CON1, 0, DMIC_GAIN_MAX_TARGET, 0),
    SOC_SINGLE!("DMIC4_HW_GAIN_TARGET", DMIC_GAIN4_CON1, 0, DMIC_GAIN_MAX_TARGET, 0),
    SOC_SINGLE!("DMIC1_HW_GAIN_CURRENT", DMIC_GAIN1_CUR, 0, DMIC_GAIN_MAX_CURRENT, 0),
    SOC_SINGLE!("DMIC2_HW_GAIN_CURRENT", DMIC_GAIN2_CUR, 0, DMIC_GAIN_MAX_CURRENT, 0),
    SOC_SINGLE!("DMIC3_HW_GAIN_CURRENT", DMIC_GAIN3_CUR, 0, DMIC_GAIN_MAX_CURRENT, 0),
    SOC_SINGLE!("DMIC4_HW_GAIN_CURRENT", DMIC_GAIN4_CUR, 0, DMIC_GAIN_MAX_CURRENT, 0),
    SOC_SINGLE!("DMIC1_HW_GAIN_UP_STEP", DMIC_GAIN1_CON3, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC2_HW_GAIN_UP_STEP", DMIC_GAIN2_CON3, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC3_HW_GAIN_UP_STEP", DMIC_GAIN3_CON3, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC4_HW_GAIN_UP_STEP", DMIC_GAIN4_CON3, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC1_HW_GAIN_DOWN_STEP", DMIC_GAIN1_CON2, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC2_HW_GAIN_DOWN_STEP", DMIC_GAIN2_CON2, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC3_HW_GAIN_DOWN_STEP", DMIC_GAIN3_CON2, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC4_HW_GAIN_DOWN_STEP", DMIC_GAIN4_CON2, 0, DMIC_GAIN_MAX_STEP, 0),
    SOC_SINGLE!("DMIC1_HW_GAIN_SAMPLE_PER_STEP", DMIC_GAIN1_CON0, DMIC_GAIN_CON0_SAMPLE_PER_STEP_SHIFT, DMIC_GAIN_MAX_PER_STEP, 0),
    SOC_SINGLE!("DMIC2_HW_GAIN_SAMPLE_PER_STEP", DMIC_GAIN2_CON0, DMIC_GAIN_CON0_SAMPLE_PER_STEP_SHIFT, DMIC_GAIN_MAX_PER_STEP, 0),
    SOC_SINGLE!("DMIC3_HW_GAIN_SAMPLE_PER_STEP", DMIC_GAIN3_CON0, DMIC_GAIN_CON0_SAMPLE_PER_STEP_SHIFT, DMIC_GAIN_MAX_PER_STEP, 0),
    SOC_SINGLE!("DMIC4_HW_GAIN_SAMPLE_PER_STEP", DMIC_GAIN4_CON0, DMIC_GAIN_CON0_SAMPLE_PER_STEP_SHIFT, DMIC_GAIN_MAX_PER_STEP, 0),
];

unsafe fn init_dmic_priv_data(afe: *mut mtk_base_afe) -> i32 {
    let afe_priv: *mut mt8188_afe_private = (*afe).platform_priv as *mut mt8188_afe_private;
    let dmic_priv: *mut mtk_dai_dmic_priv;

    dmic_priv = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_dai_dmic_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_dai_dmic_priv;
    if dmic_priv.is_null() {
        return -ENOMEM;
    }

    (*afe_priv).dai_priv[MT8188_AFE_IO_DMIC_IN as usize] = dmic_priv as *mut core::ffi::c_void;
    0
}

pub unsafe fn mt8188_dai_dmic_register(afe: *mut mtk_base_afe) -> i32 {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list as *mut _, &mut (*afe).sub_dais as *mut _);

    (*dai).dai_drivers = mtk_dai_dmic_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_dmic_driver.len();
    (*dai).dapm_widgets = mtk_dai_dmic_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_dmic_widgets.len();
    (*dai).dapm_routes = mtk_dai_dmic_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_dmic_routes.len();
    (*dai).controls = mtk_dai_dmic_controls.as_ptr();
    (*dai).num_controls = mtk_dai_dmic_controls.len();

    init_dmic_priv_data(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
