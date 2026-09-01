// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI ADDA Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

// Dependencies in the original C source:
// linux/regmap.h, linux/delay.h
// mt8189-afe-clk.h, mt8189-afe-common.h, mt8189-interconnection.h

const VS1_MT6338_MASK_SFT: u32 = 0x1;
const RG_BUCK_VS1_VOTER_EN_LO: u32 = 0x189a;
const RG_BUCK_VS1_VOTER_EN_LO_SET: u32 = 0x189b;
const RG_BUCK_VS1_VOTER_EN_LO_CLR: u32 = 0x189c;

const AUDIO_SDM_LEVEL_NORMAL: u32 = 0x1d;
const MTK_AFE_ADDA_DL_GAIN_NORMAL: u32 = 0xf74f;
const SDM_AUTO_RESET_THRESHOLD: u32 = 0x190000;

const SUPPLY_SEQ_ADDA_AFE_ON: i32 = 0;
const SUPPLY_SEQ_ADDA_DL_ON: i32 = 1;
const SUPPLY_SEQ_ADDA_AUD_PAD_TOP: i32 = 2;
const SUPPLY_SEQ_ADDA_MTKAIF_CFG: i32 = 3;
const SUPPLY_SEQ_ADDA6_MTKAIF_CFG: i32 = 4;
const SUPPLY_SEQ_ADDA_FIFO: i32 = 5;
const SUPPLY_SEQ_ADDA_AP_DMIC: i32 = 6;
const SUPPLY_SEQ_ADDA_UL_ON: i32 = 7;

const UL_IIR_SW: u32 = 0;
const UL_IIR_5HZ: u32 = 1;
const UL_IIR_10HZ: u32 = 2;
const UL_IIR_25HZ: u32 = 3;
const UL_IIR_50HZ: u32 = 4;
const UL_IIR_75HZ: u32 = 5;

const AUDIO_SDM_2ND: u32 = 0;
const AUDIO_SDM_3RD: u32 = 1;

const DELAY_DATA_MISO1: i32 = 0;
const DELAY_DATA_MISO2: i32 = 1;

const MTK_AFE_ADDA_DL_RATE_8K: u32 = 0;
const MTK_AFE_ADDA_DL_RATE_11K: u32 = 1;
const MTK_AFE_ADDA_DL_RATE_12K: u32 = 2;
const MTK_AFE_ADDA_DL_RATE_16K: u32 = 4;
const MTK_AFE_ADDA_DL_RATE_22K: u32 = 5;
const MTK_AFE_ADDA_DL_RATE_24K: u32 = 6;
const MTK_AFE_ADDA_DL_RATE_32K: u32 = 8;
const MTK_AFE_ADDA_DL_RATE_44K: u32 = 9;
const MTK_AFE_ADDA_DL_RATE_48K: u32 = 10;
const MTK_AFE_ADDA_DL_RATE_88K: u32 = 13;
const MTK_AFE_ADDA_DL_RATE_96K: u32 = 14;
const MTK_AFE_ADDA_DL_RATE_176K: u32 = 17;
const MTK_AFE_ADDA_DL_RATE_192K: u32 = 18;
const MTK_AFE_ADDA_DL_RATE_352K: u32 = 21;
const MTK_AFE_ADDA_DL_RATE_384K: u32 = 22;

const MTK_AFE_ADDA_UL_RATE_8K: u32 = 0;
const MTK_AFE_ADDA_UL_RATE_16K: u32 = 1;
const MTK_AFE_ADDA_UL_RATE_32K: u32 = 2;
const MTK_AFE_ADDA_UL_RATE_48K: u32 = 3;
const MTK_AFE_ADDA_UL_RATE_96K: u32 = 4;
const MTK_AFE_ADDA_UL_RATE_192K: u32 = 5;
const MTK_AFE_ADDA_UL_RATE_48K_HD: u32 = 6;

#[repr(C)]
struct mtk_afe_adda_priv {
    dl_rate: core::ffi::c_int,
    ul_rate: core::ffi::c_int,
}

unsafe fn adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> u32 {
    match rate {
        8000 => MTK_AFE_ADDA_DL_RATE_8K,
        11025 => MTK_AFE_ADDA_DL_RATE_11K,
        12000 => MTK_AFE_ADDA_DL_RATE_12K,
        16000 => MTK_AFE_ADDA_DL_RATE_16K,
        22050 => MTK_AFE_ADDA_DL_RATE_22K,
        24000 => MTK_AFE_ADDA_DL_RATE_24K,
        32000 => MTK_AFE_ADDA_DL_RATE_32K,
        44100 => MTK_AFE_ADDA_DL_RATE_44K,
        48000 => MTK_AFE_ADDA_DL_RATE_48K,
        96000 => MTK_AFE_ADDA_DL_RATE_96K,
        192000 => MTK_AFE_ADDA_DL_RATE_192K,
        _ => {
            dev_warn!(
                (*afe).dev,
                "%s(), rate %d invalid, use 48kHz!!!\n",
                c_str!("adda_dl_rate_transform"),
                rate
            );
            MTK_AFE_ADDA_DL_RATE_48K
        }
    }
}

unsafe fn adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: u32) -> u32 {
    match rate {
        8000 => MTK_AFE_ADDA_UL_RATE_8K,
        16000 => MTK_AFE_ADDA_UL_RATE_16K,
        32000 => MTK_AFE_ADDA_UL_RATE_32K,
        48000 => MTK_AFE_ADDA_UL_RATE_48K,
        96000 => MTK_AFE_ADDA_UL_RATE_96K,
        192000 => MTK_AFE_ADDA_UL_RATE_192K,
        _ => {
            dev_warn!(
                (*afe).dev,
                "%s(), rate %d invalid, use 48kHz!!!\n",
                c_str!("adda_ul_rate_transform"),
                rate
            );
            MTK_AFE_ADDA_UL_RATE_48K
        }
    }
}

/* dai component */
static mtk_adda_dl_ch1_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("DL0_CH1", AFE_CONN014_1, I_DL0_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN014_1, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN014_1, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN014_1, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN014_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN014_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN014_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL7_CH1", AFE_CONN014_1, I_DL7_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1", AFE_CONN014_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH1", AFE_CONN014_1, I_DL_24CH_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL24_CH1", AFE_CONN014_2, I_DL24_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN014_0, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN014_0, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN014_0, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_GAIN0_OUT_CH1", AFE_CONN014_0, I_GAIN0_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH1", AFE_CONN014_4, I_PCM_0_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_0_OUT_CH1", AFE_CONN014_6, I_SRC_0_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_1_OUT_CH1", AFE_CONN014_6, I_SRC_1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_2_OUT_CH1", AFE_CONN014_6, I_SRC_2_OUT_CH1, 1, 0),
];

static mtk_adda_dl_ch2_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("DL0_CH2", AFE_CONN015_1, I_DL0_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL0_CH1", AFE_CONN015_1, I_DL0_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN015_1, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN015_1, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN015_1, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN015_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN015_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN015_1, I_DL6_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL7_CH2", AFE_CONN015_1, I_DL7_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2", AFE_CONN015_1, I_DL8_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH2", AFE_CONN015_1, I_DL_24CH_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL24_CH2", AFE_CONN015_2, I_DL24_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN015_0, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN015_0, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN015_0, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_GAIN0_OUT_CH2", AFE_CONN015_0, I_GAIN0_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH1", AFE_CONN015_4, I_PCM_0_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH2", AFE_CONN015_4, I_PCM_0_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_0_OUT_CH2", AFE_CONN015_6, I_SRC_0_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_1_OUT_CH2", AFE_CONN015_6, I_SRC_1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_2_OUT_CH2", AFE_CONN015_6, I_SRC_2_OUT_CH2, 1, 0),
];

static mtk_adda_dl_ch3_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("DL0_CH1", AFE_CONN016_1, I_DL0_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN016_1, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN016_1, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN016_1, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN016_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN016_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN016_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL7_CH1", AFE_CONN016_1, I_DL7_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1", AFE_CONN016_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH1", AFE_CONN016_1, I_DL_24CH_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH3", AFE_CONN016_1, I_DL_24CH_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL24_CH1", AFE_CONN016_2, I_DL24_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN016_0, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN016_0, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN016_0, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_GAIN0_OUT_CH1", AFE_CONN016_0, I_GAIN0_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH1", AFE_CONN016_4, I_PCM_0_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_0_OUT_CH1", AFE_CONN016_6, I_SRC_0_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_1_OUT_CH1", AFE_CONN016_6, I_SRC_1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_2_OUT_CH1", AFE_CONN016_6, I_SRC_2_OUT_CH1, 1, 0),
];

static mtk_adda_dl_ch4_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("DL0_CH2", AFE_CONN017_1, I_DL0_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN017_1, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN017_1, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN017_1, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN017_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN017_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN017_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL7_CH2", AFE_CONN017_1, I_DL7_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2", AFE_CONN017_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH2", AFE_CONN017_1, I_DL_24CH_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL_24CH_CH4", AFE_CONN017_1, I_DL_24CH_CH4, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL24_CH2", AFE_CONN017_2, I_DL24_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN017_0, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN017_0, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN017_0, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_GAIN0_OUT_CH2", AFE_CONN017_0, I_GAIN0_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH1", AFE_CONN017_4, I_PCM_0_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_0_CAP_CH2", AFE_CONN017_4, I_PCM_0_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_0_OUT_CH2", AFE_CONN017_6, I_SRC_0_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_1_OUT_CH2", AFE_CONN017_6, I_SRC_1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("HW_SRC_2_OUT_CH2", AFE_CONN017_6, I_SRC_2_OUT_CH2, 1, 0),
];

unsafe fn mtk_adda_ul_src_enable_dmic(afe: *mut mtk_base_afe, id: core::ffi::c_int) -> core::ffi::c_int {
    let reg: u32;
    let reg1: u32;

    match id {
        MT8189_DAI_ADDA => {
            reg = AFE_ADDA_UL0_SRC_CON0;
            reg1 = AFE_ADDA_UL0_SRC_CON1;
        }
        MT8189_DAI_AP_DMIC => {
            reg = AFE_ADDA_DMIC0_SRC_CON0;
            reg1 = AFE_ADDA_DMIC0_SRC_CON1;
        }
        MT8189_DAI_AP_DMIC_CH34 => {
            reg = AFE_ADDA_DMIC1_SRC_CON0;
            reg1 = AFE_ADDA_DMIC1_SRC_CON1;
        }
        _ => return -EINVAL,
    }

    /* choose Phase */
    regmap_update_bits((*afe).regmap, reg, UL_DMIC_PHASE_SEL_CH1_MASK_SFT, 0x0 << UL_DMIC_PHASE_SEL_CH1_SFT);
    regmap_update_bits((*afe).regmap, reg, UL_DMIC_PHASE_SEL_CH2_MASK_SFT, 0x4 << UL_DMIC_PHASE_SEL_CH2_SFT);

    /* dmic mode, 3.25M*/
    regmap_update_bits((*afe).regmap, reg, DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT, 0x0);
    regmap_update_bits((*afe).regmap, reg, DMIC_LOW_POWER_MODE_CTL_MASK_SFT, 0x0);

    /* turn on dmic, ch1, ch2 */
    regmap_update_bits((*afe).regmap, reg, UL_SDM_3_LEVEL_CTL_MASK_SFT, 0x1 << UL_SDM_3_LEVEL_CTL_SFT);
    regmap_update_bits((*afe).regmap, reg, UL_MODE_3P25M_CH1_CTL_MASK_SFT, 0x1 << UL_MODE_3P25M_CH1_CTL_SFT);
    regmap_update_bits((*afe).regmap, reg, UL_MODE_3P25M_CH2_CTL_MASK_SFT, 0x1 << UL_MODE_3P25M_CH2_CTL_SFT);

    /* ul gain:  gain = 0x7fff/positive_gain = 0x0/gain_mode = 0x10 */
    regmap_update_bits((*afe).regmap, reg1, ADDA_UL_GAIN_VALUE_MASK_SFT, 0x7fff << ADDA_UL_GAIN_VALUE_SFT);
    regmap_update_bits((*afe).regmap, reg1, ADDA_UL_POSTIVEGAIN_MASK_SFT, 0x0 << ADDA_UL_POSTIVEGAIN_SFT);
    /* gain_mode = 0x02: Add 0.5 gain at CIC output */
    regmap_update_bits((*afe).regmap, reg1, GAIN_MODE_MASK_SFT, 0x02 << GAIN_MODE_SFT);

    0
}

unsafe fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let mtkaif_dmic = (*afe_priv).mtkaif_dmic;

    dev_dbg!(
        (*afe).dev,
        "%s(), name %s, event 0x%x, mtkaif_dmic %d\n",
        c_str!("mtk_adda_ul_event"),
        (*w).name,
        event,
        mtkaif_dmic
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* update setting to dmic */
            if mtkaif_dmic != 0 {
                /* mtkaif_rxif_data_mode = 1, dmic */
                regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG0, RG_MTKAIF0_RXIF_DATA_MODE_MASK_SFT, 0x1);

                /* dmic mode, 3.25M*/
                regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG0, RG_MTKAIF0_RXIF_VOICE_MODE_MASK_SFT, 0x0);
                mtk_adda_ul_src_enable_dmic(afe, MT8189_DAI_ADDA);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(120, 130);

            /* reset dmic */
            (*afe_priv).mtkaif_dmic = 0;
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_pad_top_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;

    if event == SND_SOC_DAPM_PRE_PMU {
        if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
            regmap_write((*afe).regmap, AFE_AUD_PAD_TOP_CFG0, 0xB8);
        } else {
            regmap_write((*afe).regmap, AFE_AUD_PAD_TOP_CFG0, 0xB0);
        }
    }

    0
}

unsafe fn is_adda_mtkaif_need_phase_delay(afe_priv: *mut mt8189_afe_private) -> bool {
    (*afe_priv).mtkaif_chosen_phase[0] >= 0 && (*afe_priv).mtkaif_chosen_phase[1] >= 0
}

unsafe fn mtk_adda_mtkaif_cfg_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let mut delay_data: core::ffi::c_int;
    let mut delay_cycle: core::ffi::c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
                /* set protocol 2 */
                regmap_write((*afe).regmap, AFE_MTKAIF0_CFG0, 0x00010000);
                regmap_write((*afe).regmap, AFE_MTKAIF1_CFG0, 0x00010000);

                /* mtkaif_rxif_clkinv_adc inverse for calibration */
                regmap_update_bits((*afe).regmap, AFE_MTKAIF0_CFG0, RG_MTKAIF0_RXIF_CLKINV_MASK_SFT, 0x1 << RG_MTKAIF0_RXIF_CLKINV_SFT);
                regmap_update_bits((*afe).regmap, AFE_MTKAIF1_CFG0, RG_MTKAIF1_RXIF_CLKINV_ADC_MASK_SFT, 0x1 << RG_MTKAIF1_RXIF_CLKINV_ADC_SFT);

                /* This event align the phase of every miso pin */
                /* If only 1 miso is used, there is no need to do phase delay. */
                if strcmp((*w).name, c_str!("ADDA_MTKAIF_CFG")) == 0 && !is_adda_mtkaif_need_phase_delay(afe_priv) {
                    dev_dbg!(
                        (*afe).dev,
                        "%s(), check adda mtkaif_chosen_phase[0/1]:%d/%d\n",
                        c_str!("mtk_adda_mtkaif_cfg_event"),
                        (*afe_priv).mtkaif_chosen_phase[0],
                        (*afe_priv).mtkaif_chosen_phase[1]
                    );
                    return 0;
                } else if strcmp((*w).name, c_str!("ADDA6_MTKAIF_CFG")) == 0 && (*afe_priv).mtkaif_chosen_phase[2] < 0 {
                    dev_dbg!(
                        (*afe).dev,
                        "%s(), check adda6 mtkaif_chosen_phase[2]:%d\n",
                        c_str!("mtk_adda_mtkaif_cfg_event"),
                        (*afe_priv).mtkaif_chosen_phase[2]
                    );
                    return 0;
                }

                /* set delay for ch12 to align phase of miso0 and miso1 */
                if (*afe_priv).mtkaif_phase_cycle[0] >= (*afe_priv).mtkaif_phase_cycle[1] {
                    delay_data = DELAY_DATA_MISO1;
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[0] - (*afe_priv).mtkaif_phase_cycle[1];
                } else {
                    delay_data = DELAY_DATA_MISO2;
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[0];
                }

                regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG2, RG_MTKAIF0_RXIF_DELAY_DATA_MASK_SFT, (delay_data as u32) << RG_MTKAIF0_RXIF_DELAY_DATA_SFT);
                regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG2, RG_MTKAIF0_RXIF_DELAY_CYCLE_MASK_SFT, (delay_cycle as u32) << RG_MTKAIF0_RXIF_DELAY_CYCLE_SFT);

                /* set delay between ch3 and ch2 */
                if (*afe_priv).mtkaif_phase_cycle[2] >= (*afe_priv).mtkaif_phase_cycle[1] {
                    delay_data = DELAY_DATA_MISO1; /* ch3 */
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[2] - (*afe_priv).mtkaif_phase_cycle[1];
                } else {
                    delay_data = DELAY_DATA_MISO2; /* ch2 */
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[2];
                }

                regmap_update_bits((*afe).regmap, AFE_MTKAIF1_RX_CFG2, RG_MTKAIF1_RXIF_DELAY_DATA_MASK_SFT, (delay_data as u32) << RG_MTKAIF1_RXIF_DELAY_DATA_SFT);
                regmap_update_bits((*afe).regmap, AFE_MTKAIF1_RX_CFG2, RG_MTKAIF1_RXIF_DELAY_CYCLE_MASK_SFT, (delay_cycle as u32) << RG_MTKAIF1_RXIF_DELAY_CYCLE_SFT);
            } else if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2 {
                regmap_write((*afe).regmap, AFE_MTKAIF0_CFG0, 0x00010000);
                regmap_write((*afe).regmap, AFE_MTKAIF1_CFG0, 0x00010000);
            } else {
                regmap_write((*afe).regmap, AFE_MTKAIF0_CFG0, 0x0);
                regmap_write((*afe).regmap, AFE_MTKAIF1_CFG0, 0x0);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_dl_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg!(
        (*afe).dev,
        "%s(), name %s, event 0x%x\n",
        c_str!("mtk_adda_dl_event"),
        (*w).name,
        event
    );

    /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
    if event == SND_SOC_DAPM_POST_PMD {
        usleep_range(120, 130);
    }

    0
}

unsafe fn mt6363_vs1_vote(afe: *mut mtk_base_afe) {
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let pre_enable = (*afe_priv).is_mt6363_vote;
    let enable: bool;

    if (*afe_priv).pmic_regmap.is_null() {
        return;
    }

    enable = ((*afe_priv).is_adda_dl_on && (*afe_priv).is_adda_dl_max_vol) || (*afe_priv).is_adda_ul_on;
    if enable == pre_enable {
        dev_dbg!((*afe).dev, "%s() enable == pre_enable = %d\n", c_str!("mt6363_vs1_vote"), enable as i32);
        return;
    }

    (*afe_priv).is_mt6363_vote = enable;
    dev_dbg!((*afe).dev, "%s() enable = %d\n", c_str!("mt6363_vs1_vote"), enable as i32);

    if enable {
        regmap_update_bits((*afe_priv).pmic_regmap, RG_BUCK_VS1_VOTER_EN_LO_SET, VS1_MT6338_MASK_SFT, 0x1);
    } else {
        regmap_update_bits((*afe_priv).pmic_regmap, RG_BUCK_VS1_VOTER_EN_LO_CLR, VS1_MT6338_MASK_SFT, 0x1);
    }
}

unsafe fn mt_vs1_voter_dl_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: core::ffi::c_int) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;

    dev_dbg!((*afe).dev, "%s(), event = 0x%x\n", c_str!("mt_vs1_voter_dl_event"), event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*afe_priv).is_adda_dl_on = true;
            mt6363_vs1_vote(afe);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*afe_priv).is_adda_dl_on = false;
            mt6363_vs1_vote(afe);
        }
        _ => {}
    }

    0
}

unsafe fn mt_vs1_voter_ul_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: core::ffi::c_int) -> core::ffi::c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;

    dev_dbg!((*afe).dev, "%s(), event = 0x%x\n", c_str!("mt_vs1_voter_ul_event"), event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            (*afe_priv).is_adda_ul_on = true;
            mt6363_vs1_vote(afe);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*afe_priv).is_adda_ul_on = false;
            mt6363_vs1_vote(afe);
        }
        _ => {}
    }

    0
}

unsafe fn mt8189_adda_dmic_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> core::ffi::c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).mtkaif_dmic as _;

    0
}

unsafe fn mt8189_adda_dmic_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> core::ffi::c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let dmic_on: core::ffi::c_int;

    dmic_on = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };

    dev_dbg!(
        (*afe).dev,
        "%s(), kcontrol name %s, dmic_on %d\n",
        c_str!("mt8189_adda_dmic_set"),
        (*kcontrol).id.name,
        dmic_on
    );

    (*afe_priv).mtkaif_dmic = dmic_on;
    (*afe_priv).mtkaif_dmic_ch34 = dmic_on;

    0
}

unsafe fn mt8189_adda_dl_max_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> core::ffi::c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).is_adda_dl_max_vol as _;

    0
}

unsafe fn mt8189_adda_dl_max_vol_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> core::ffi::c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let is_adda_dl_max_vol: bool = (*ucontrol).value.integer.value[0] != 0;

    (*afe_priv).is_adda_dl_max_vol = is_adda_dl_max_vol;
    mt6363_vs1_vote(afe);

    0
}

static mtk_adda_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE!("ADDA_DL_GAIN", AFE_ADDA_DL_SRC_CON1, AFE_DL_GAIN1_CTL_PRE_SFT, AFE_DL_GAIN1_CTL_PRE_MASK, 0),
    SOC_SINGLE_BOOL_EXT!("MTKAIF_DMIC Switch", 0, mt8189_adda_dmic_get, mt8189_adda_dmic_set),
    SOC_SINGLE_BOOL_EXT!("ADDA_DL_MAX_VOL Switch", 0, mt8189_adda_dl_max_vol_get, mt8189_adda_dl_max_vol_set),
];

static adda_ul_mux_texts: &[*const core::ffi::c_char] = &[
    c_str!("MTKAIF"),
    c_str!("AP_DMIC"),
    c_str!("AP_DMIC_MULTI_CH"),
];

static adda_ul_mux_map_enum: soc_enum = SOC_ENUM_SINGLE_DECL!(SND_SOC_NOPM, 0, adda_ul_mux_texts);

static adda_ul_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("ADDA_UL_MUX Select", adda_ul_mux_map_enum);

static adda_ch34_ul_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("ADDA_CH34_UL_MUX Select", adda_ul_mux_map_enum);

static mtk_dai_adda_widgets: &[snd_soc_dapm_widget] = &[
    /* inter-connections */
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH1", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch1_mix, mtk_adda_dl_ch1_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH2", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch2_mix, mtk_adda_dl_ch2_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH3", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch3_mix, mtk_adda_dl_ch3_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH4", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch4_mix, mtk_adda_dl_ch4_mix.len()),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Enable", SUPPLY_SEQ_ADDA_AFE_ON, AUDIO_ENGEN_CON0, AUDIO_F3P25M_EN_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_DL0_CG", SUPPLY_SEQ_ADDA_DL_ON, AUDIO_TOP_CON0, PDN_DL0_DAC_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_UL0_CG", SUPPLY_SEQ_ADDA_UL_ON, AUDIO_TOP_CON1, PDN_UL0_ADC_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Playback Enable", SUPPLY_SEQ_ADDA_DL_ON, AFE_ADDA_DL_SRC_CON0, AFE_DL_SRC_ON_TMP_CTL_PRE_SFT, 0, Some(mtk_adda_dl_event), SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA_UL0_SRC_CON0, UL_SRC_ON_TMP_CTL_SFT, 0, Some(mtk_adda_ul_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("AP DMIC Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA_DMIC0_SRC_CON0, UL_SRC_ON_TMP_CTL_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP DMIC CH34 Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA_DMIC1_SRC_CON0, UL_SRC_ON_TMP_CTL_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AUD_PAD_TOP", SUPPLY_SEQ_ADDA_AUD_PAD_TOP, AFE_AUD_PAD_TOP_CFG0, RG_RX_FIFO_ON_SFT, 0, Some(mtk_adda_pad_top_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_MTKAIF_CFG", SUPPLY_SEQ_ADDA_MTKAIF_CFG, SND_SOC_NOPM, 0, 0, Some(mtk_adda_mtkaif_cfg_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("ADDA6_MTKAIF_CFG", SUPPLY_SEQ_ADDA6_MTKAIF_CFG, SND_SOC_NOPM, 0, 0, Some(mtk_adda_mtkaif_cfg_event), SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_EN", SUPPLY_SEQ_ADDA_AP_DMIC, AFE_ADDA_DMIC0_SRC_CON0, UL_AP_DMIC_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC0_CG", SUPPLY_SEQ_ADDA_AP_DMIC, AUDIO_TOP_CON1, PDN_DMIC0_ADC_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_CH34_EN", SUPPLY_SEQ_ADDA_AP_DMIC, AFE_ADDA_DMIC1_SRC_CON0, UL_AP_DMIC_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC1_CG", SUPPLY_SEQ_ADDA_AP_DMIC, AUDIO_TOP_CON1, PDN_DMIC1_ADC_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_FIFO", SUPPLY_SEQ_ADDA_FIFO, AFE_ADDA_UL0_SRC_CON1, FIFO_SOFT_RST_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_FIFO", SUPPLY_SEQ_ADDA_FIFO, AFE_ADDA_DMIC0_SRC_CON1, FIFO_SOFT_RST_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_CH34_FIFO", SUPPLY_SEQ_ADDA_FIFO, AFE_ADDA_DMIC1_SRC_CON1, FIFO_SOFT_RST_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("VS1_VOTER_DL", SUPPLY_SEQ_ADDA_AFE_ON, SND_SOC_NOPM, 0, 0, Some(mt_vs1_voter_dl_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("VS1_VOTER_UL", SUPPLY_SEQ_ADDA_AFE_ON, SND_SOC_NOPM, 0, 0, Some(mt_vs1_voter_ul_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX!("ADDA_UL_Mux", SND_SOC_NOPM, 0, 0, &adda_ul_mux_control),
    SND_SOC_DAPM_MUX!("ADDA_CH34_UL_Mux", SND_SOC_NOPM, 0, 0, &adda_ch34_ul_mux_control),
    SND_SOC_DAPM_INPUT!("AP_DMIC_INPUT"),
];

static mtk_dai_adda_routes: &[snd_soc_dapm_route] = &[
    /* playback */
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL0_CH1"), source: c_str!("DL0") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL0_CH1"), source: c_str!("DL0") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL0_CH2"), source: c_str!("DL0") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL1_CH1"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL1_CH2"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL2_CH1"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL2_CH2"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL3_CH1"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL3_CH2"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL4_CH1"), source: c_str!("DL4") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL4_CH2"), source: c_str!("DL4") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL5_CH1"), source: c_str!("DL5") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL5_CH2"), source: c_str!("DL5") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL6_CH1"), source: c_str!("DL6") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL6_CH2"), source: c_str!("DL6") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL7_CH1"), source: c_str!("DL7") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL7_CH2"), source: c_str!("DL7") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL8_CH1"), source: c_str!("DL8") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL8_CH2"), source: c_str!("DL8") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL_24CH_CH1"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL_24CH_CH2"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL24_CH1"), source: c_str!("DL24") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL24_CH2"), source: c_str!("DL24") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA_DL_CH1") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA_DL_CH2") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA Playback Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("AUD_PAD_TOP") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("VS1_VOTER_DL") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA_DL0_CG") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL0_CH1"), source: c_str!("DL0") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL0_CH2"), source: c_str!("DL0") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL1_CH1"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL1_CH2"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL2_CH1"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL2_CH2"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL3_CH1"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL3_CH2"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL4_CH1"), source: c_str!("DL4") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL4_CH2"), source: c_str!("DL4") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL5_CH1"), source: c_str!("DL5") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL5_CH2"), source: c_str!("DL5") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL6_CH1"), source: c_str!("DL6") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL6_CH2"), source: c_str!("DL6") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL7_CH1"), source: c_str!("DL7") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL7_CH2"), source: c_str!("DL7") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL8_CH1"), source: c_str!("DL8") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL8_CH2"), source: c_str!("DL8") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL_24CH_CH1"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL_24CH_CH2"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL_24CH_CH3"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL_24CH_CH4"), source: c_str!("DL_24CH") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH3"), control: c_str!("DL24_CH1"), source: c_str!("DL24") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH4"), control: c_str!("DL24_CH2"), source: c_str!("DL24") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA Capture Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("AUD_PAD_TOP") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA_MTKAIF_CFG") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("VS1_VOTER_UL") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA_UL0_CG") },
    /* capture */
    snd_soc_dapm_route { sink: c_str!("ADDA_UL_Mux"), control: c_str!("MTKAIF"), source: c_str!("ADDA Capture") },
    snd_soc_dapm_route { sink: c_str!("ADDA_UL_Mux"), control: c_str!("AP_DMIC"), source: c_str!("AP DMIC Capture") },
    snd_soc_dapm_route { sink: c_str!("ADDA_CH34_UL_Mux"), control: c_str!("AP_DMIC"), source: c_str!("AP DMIC CH34 Capture") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("AP DMIC Capture Enable") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_FIFO") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_EN") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC0_CG") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("AP DMIC CH34 Capture Enable") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_CH34_FIFO") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_CH34_EN") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC1_CG") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_INPUT") },
    snd_soc_dapm_route { sink: c_str!("AP DMIC CH34 Capture"), control: core::ptr::null(), source: c_str!("AP_DMIC_INPUT") },
];

/* dai ops */
unsafe fn set_playback_hw_params(params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let rate: u32 = params_rate(params);
    let adda_priv: *mut mtk_afe_adda_priv;
    let mut dl_src_con0: u32;
    let mut dl_src_con1: u32;
    let id = (*dai).id;

    adda_priv = (*afe_priv).dai_priv[id as usize] as *mut mtk_afe_adda_priv;
    if adda_priv.is_null() {
        return -EINVAL;
    }

    (*adda_priv).dl_rate = rate as core::ffi::c_int;

    /* set sampling rate */
    dl_src_con0 = adda_dl_rate_transform(afe, rate) << AFE_DL_INPUT_MODE_CTL_SFT;

    /* set output mode, UP_SAMPLING_RATE_X8 */
    dl_src_con0 |= 0x3 << AFE_DL_OUTPUT_SEL_CTL_SFT;

    /* turn off mute function */
    dl_src_con0 |= 0x01 << AFE_DL_MUTE_CH2_OFF_CTL_PRE_SFT;
    dl_src_con0 |= 0x01 << AFE_DL_MUTE_CH1_OFF_CTL_PRE_SFT;

    /* set voice input data if input sample rate is 8k or 16k */
    if rate == 8000 || rate == 16000 {
        dl_src_con0 |= 0x01 << AFE_DL_VOICE_MODE_CTL_PRE_SFT;
    }

    /* SA suggest apply -0.3db to audio/speech path */
    dl_src_con1 = MTK_AFE_ADDA_DL_GAIN_NORMAL << AFE_DL_GAIN1_CTL_PRE_SFT;
    dl_src_con1 |= MTK_AFE_ADDA_DL_GAIN_NORMAL << AFE_DL_GAIN2_CTL_PRE_SFT;

    /* turn on down-link gain */
    dl_src_con0 |= 0x01 << AFE_DL_GAIN_ON_CTL_PRE_SFT;

    if id == MT8189_DAI_ADDA {
        /* clean predistortion */
        regmap_write((*afe).regmap, AFE_ADDA_DL_PREDIS_CON0, 0);
        regmap_write((*afe).regmap, AFE_ADDA_DL_PREDIS_CON1, 0);

        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC_CON0, dl_src_con0);
        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC_CON1, dl_src_con1);

        /* set sdm gain */
        regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, AFE_DL_ATTGAIN_CTL_MASK_SFT, AUDIO_SDM_LEVEL_NORMAL << AFE_DL_ATTGAIN_CTL_SFT);

        /* 2nd sdm */
        regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, AFE_DL_USE_3RD_SDM_MASK_SFT, AUDIO_SDM_2ND << AFE_DL_USE_3RD_SDM_SFT);

        /* sdm auto reset */
        regmap_write((*afe).regmap, AFE_ADDA_DL_SDM_AUTO_RESET_CON, SDM_AUTO_RESET_THRESHOLD);
        regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_AUTO_RESET_CON, AFE_DL_SDM_AUTO_RESET_TEST_ON_SFT, 0x1 << AFE_DL_SDM_AUTO_RESET_TEST_ON_SFT);
    }

    0
}

unsafe fn set_capture_hw_params(params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let rate: u32 = params_rate(params);
    let adda_priv: *mut mtk_afe_adda_priv;
    let voice_mode: u32;
    let mut ul_src_con0: u32;
    let id = (*dai).id;

    adda_priv = (*afe_priv).dai_priv[id as usize] as *mut mtk_afe_adda_priv;
    if adda_priv.is_null() {
        return -EINVAL;
    }

    (*adda_priv).ul_rate = rate as core::ffi::c_int;

    voice_mode = adda_ul_rate_transform(afe, rate);

    ul_src_con0 = (voice_mode << UL_VOICE_MODE_CH1_CH2_CTL_SFT) & UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT;

    /* enable iir */
    ul_src_con0 |= (1 << UL_IIR_ON_TMP_CTL_SFT) & UL_IIR_ON_TMP_CTL_MASK_SFT;
    ul_src_con0 |= (UL_IIR_SW << UL_IIRMODE_CTL_SFT) & UL_IIRMODE_CTL_MASK_SFT;

    match id {
        MT8189_DAI_ADDA => {
            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_10_09, 0x0000C048);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_SRC_CON0, ul_src_con0);

            /* mtkaif_rxif_data_mode = 0, amic */
            regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG0, RG_MTKAIF0_RXIF_DATA_MODE_MASK_SFT, 0x0 << RG_MTKAIF0_RXIF_DATA_MODE_SFT);
        }
        MT8189_DAI_AP_DMIC => {
            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_IIR_COEF_10_09, 0x0000C048);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC0_SRC_CON0, ul_src_con0);
        }
        MT8189_DAI_AP_DMIC_CH34 => {
            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_IIR_COEF_10_09, 0x0000C048);
            regmap_write((*afe).regmap, AFE_ADDA_DMIC1_SRC_CON0, ul_src_con0);
        }
        _ => {}
    }

    /* ap dmic */
    if id == MT8189_DAI_AP_DMIC || id == MT8189_DAI_AP_DMIC_CH34 {
        mtk_adda_ul_src_enable_dmic(afe, id);
    }

    0
}

unsafe fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let id = (*dai).id;

    if id >= MT8189_DAI_NUM || id < 0 {
        return -EINVAL;
    }

    dev_dbg!(
        (*afe).dev,
        "%s(), id %d, stream %d, rate %d\n",
        c_str!("mtk_dai_adda_hw_params"),
        id,
        (*substream).stream,
        params_rate(params)
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return set_playback_hw_params(params, dai);
    } else {
        return set_capture_hw_params(params, dai);
    }
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
    ..unsafe { core::mem::zeroed() }
};

/* dai driver */
const MTK_ADDA_PLAYBACK_RATES: u32 = SNDRV_PCM_RATE_8000_48000;

const MTK_ADDA_CAPTURE_RATES: u32 = SNDRV_PCM_RATE_8000 |
    SNDRV_PCM_RATE_16000 |
    SNDRV_PCM_RATE_32000 |
    SNDRV_PCM_RATE_48000;

const MTK_ADDA_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: c_str!("ADDA"),
        id: MT8189_DAI_ADDA,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("ADDA Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("ADDA Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("ADDA_CH34"),
        id: MT8189_DAI_ADDA_CH34,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("ADDA CH34 Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("AP_DMIC"),
        id: MT8189_DAI_AP_DMIC,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("AP DMIC Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("AP_DMIC_CH34"),
        id: MT8189_DAI_AP_DMIC_CH34,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("AP DMIC CH34 Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn init_adda_priv_data(afe: *mut mtk_base_afe) -> core::ffi::c_int {
    let afe_priv: *mut mt8189_afe_private = (*afe).platform_priv as *mut mt8189_afe_private;
    let mut adda_priv: *mut mtk_afe_adda_priv;
    static adda_dai_list: [core::ffi::c_int; 2] = [
        MT8189_DAI_ADDA,
        MT8189_DAI_ADDA_CH34,
    ];

    for i in 0..adda_dai_list.len() {
        adda_priv = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_afe_adda_priv>(), GFP_KERNEL) as *mut mtk_afe_adda_priv;
        if adda_priv.is_null() {
            return -ENOMEM;
        }

        (*afe_priv).dai_priv[adda_dai_list[i] as usize] = adda_priv as *mut core::ffi::c_void;
    }

    /* ap dmic priv share with adda */
    (*afe_priv).dai_priv[MT8189_DAI_AP_DMIC as usize] =
        (*afe_priv).dai_priv[MT8189_DAI_ADDA as usize];
    (*afe_priv).dai_priv[MT8189_DAI_AP_DMIC_CH34 as usize] =
        (*afe_priv).dai_priv[MT8189_DAI_ADDA_CH34 as usize];

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8189_dai_adda_register(afe: *mut mtk_base_afe) -> core::ffi::c_int {
    let dai: *mut mtk_base_afe_dai;
    let ret: core::ffi::c_int;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as _;
    (*dai).controls = mtk_adda_controls.as_ptr();
    (*dai).num_controls = mtk_adda_controls.len() as _;
    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as _;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as _;

    ret = init_adda_priv_data(afe);
    if ret != 0 {
        return ret;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
