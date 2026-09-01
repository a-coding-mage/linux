// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI ADDA Control
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/regmap.h, mt8192-afe-clk.h, mt8192-afe-common.h,
// mt8192-afe-gpio.h, mt8192-interconnection.h, mtk-dai-adda-common.h.

const UL_IIR_SW: i32 = 0;
const UL_IIR_5HZ: i32 = 1;
const UL_IIR_10HZ: i32 = 2;
const UL_IIR_25HZ: i32 = 3;
const UL_IIR_50HZ: i32 = 4;
const UL_IIR_75HZ: i32 = 5;

const AUDIO_SDM_LEVEL_MUTE: i32 = 0;
const AUDIO_SDM_LEVEL_NORMAL: i32 = 0x1d;
/* if you change level normal */
/* you need to change formula of hp impedance and dc trim too */

const AUDIO_SDM_2ND: i32 = 0;
const AUDIO_SDM_3RD: i32 = 1;

const SDM_AUTO_RESET_THRESHOLD: u32 = 0x190000;

/* dai component */
static mtk_adda_dl_ch1_mix: [snd_kcontrol_new; 16] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN3, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1", AFE_CONN3, I_DL12_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN3, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN3, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN3_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN3_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN3_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1", AFE_CONN3_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN3, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN3, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN3, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1", AFE_CONN3, I_GAIN1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN3, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN3, I_PCM_2_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH1", AFE_CONN3_1, I_SRC_1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_2_OUT_CH1", AFE_CONN3_1, I_SRC_2_OUT_CH1, 1, 0),
];

static mtk_adda_dl_ch2_mix: [snd_kcontrol_new; 21] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN4, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN4, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2", AFE_CONN4, I_DL12_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN4, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN4, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN4, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN4, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN4_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN4_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN4_1, I_DL6_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2", AFE_CONN4_1, I_DL8_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN4, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN4, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN4, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2", AFE_CONN4, I_GAIN1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN4, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN4, I_PCM_2_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN4, I_PCM_1_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN4, I_PCM_2_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH2", AFE_CONN4_1, I_SRC_1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_2_OUT_CH2", AFE_CONN4_1, I_SRC_2_OUT_CH2, 1, 0),
];

static mtk_adda_dl_ch3_mix: [snd_kcontrol_new; 13] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN52, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1", AFE_CONN52, I_DL12_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN52, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN52, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN52_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN52_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN52_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN52, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN52, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN52, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1", AFE_CONN52, I_GAIN1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN52, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN52, I_PCM_2_CAP_CH1, 1, 0),
];

static mtk_adda_dl_ch4_mix: [snd_kcontrol_new; 19] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN53, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN53, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2", AFE_CONN53, I_DL12_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN53, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN53, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN53, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN53, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN53_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN53_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN53_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN53, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN53, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN53, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2", AFE_CONN53, I_GAIN1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN53, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN53, I_PCM_2_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN53, I_PCM_1_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN53, I_PCM_2_CAP_CH2, 1, 0),
];

static mtk_stf_ch1_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN19, I_ADDA_UL_CH1, 1, 0),
];

static mtk_stf_ch2_mix: [snd_kcontrol_new; 1] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN20, I_ADDA_UL_CH2, 1, 0),
];

const SUPPLY_SEQ_ADDA_AFE_ON: i32 = 0;
const SUPPLY_SEQ_ADDA_DL_ON: i32 = 1;
const SUPPLY_SEQ_ADDA_AUD_PAD_TOP: i32 = 2;
const SUPPLY_SEQ_ADDA_MTKAIF_CFG: i32 = 3;
const SUPPLY_SEQ_ADDA6_MTKAIF_CFG: i32 = 4;
const SUPPLY_SEQ_ADDA_FIFO: i32 = 5;
const SUPPLY_SEQ_ADDA_AP_DMIC: i32 = 6;
const SUPPLY_SEQ_ADDA_UL_ON: i32 = 7;

unsafe fn mtk_adda_ul_src_dmic(afe: *mut mtk_base_afe, id: i32) -> i32 {
    let reg: u32;

    match id {
        MT8192_DAI_ADDA | MT8192_DAI_AP_DMIC => reg = AFE_ADDA_UL_SRC_CON0,
        MT8192_DAI_ADDA_CH34 | MT8192_DAI_AP_DMIC_CH34 => reg = AFE_ADDA6_UL_SRC_CON0,
        _ => return -EINVAL,
    }

    /* dmic mode, 3.25M*/
    regmap_update_bits((*afe).regmap, reg, DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT, 0x0);
    regmap_update_bits((*afe).regmap, reg, DMIC_LOW_POWER_MODE_CTL_MASK_SFT, 0x0);

    /* turn on dmic, ch1, ch2 */
    regmap_update_bits((*afe).regmap, reg, UL_SDM_3_LEVEL_CTL_MASK_SFT, 0x1 << UL_SDM_3_LEVEL_CTL_SFT);
    regmap_update_bits((*afe).regmap, reg, UL_MODE_3P25M_CH1_CTL_MASK_SFT, 0x1 << UL_MODE_3P25M_CH1_CTL_SFT);
    regmap_update_bits((*afe).regmap, reg, UL_MODE_3P25M_CH2_CTL_MASK_SFT, 0x1 << UL_MODE_3P25M_CH2_CTL_SFT);
    0
}

unsafe fn mtk_adda_ul_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let mtkaif_dmic = (*afe_priv).mtkaif_dmic;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA, 1);
            /* update setting to dmic */
            if mtkaif_dmic != 0 {
                /* mtkaif_rxif_data_mode = 1, dmic */
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x1, 0x1);
                /* dmic mode, 3.25M*/
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, MTKAIF_RXIF_VOICE_MODE_MASK_SFT, 0x0);
                mtk_adda_ul_src_dmic(afe, MT8192_DAI_ADDA);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA, 1);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_ch34_ul_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let mtkaif_dmic = (*afe_priv).mtkaif_dmic_ch34;
    let mtkaif_adda6_only = (*afe_priv).mtkaif_adda6_only;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA_CH34, 1);
            /* update setting to dmic */
            if mtkaif_dmic != 0 {
                /* mtkaif_rxif_data_mode = 1, dmic */
                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG0, 0x1, 0x1);
                /* dmic mode, 3.25M*/
                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG0, MTKAIF_RXIF_VOICE_MODE_MASK_SFT, 0x0);
                mtk_adda_ul_src_dmic(afe, MT8192_DAI_ADDA_CH34);
            }

            /* when using adda6 without adda enabled,
             * RG_ADDA6_MTKAIF_RX_SYNC_WORD2_DISABLE_SFT need to be set or
             * data cannot be received.
             */
            if mtkaif_adda6_only != 0 {
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_SYNCWORD_CFG, 0x1 << 23, 0x1 << 23);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA_CH34, 1);
            /* reset dmic */
            (*afe_priv).mtkaif_dmic_ch34 = 0;
            if mtkaif_adda6_only != 0 {
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_SYNCWORD_CFG, 0x1 << 23, 0x0 << 23);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_pad_top_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
                regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x38);
            } else {
                regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x30);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_mtkaif_cfg_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let mut delay_data: i32;
    let mut delay_cycle: i32;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
                /* set protocol 2 */
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x00010000);
                regmap_write((*afe).regmap, AFE_ADDA6_MTKAIF_CFG0, 0x00010000);

                if snd_soc_dapm_widget_name_cmp(w, "ADDA_MTKAIF_CFG\0".as_ptr() as *const i8) == 0
                    && ((*afe_priv).mtkaif_chosen_phase[0] < 0 || (*afe_priv).mtkaif_chosen_phase[1] < 0)
                {
                    dev_warn((*afe).dev, "%s(), mtkaif_chosen_phase[0/1]:%d/%d\n\0".as_ptr() as *const i8, "__func__\0".as_ptr() as *const i8, (*afe_priv).mtkaif_chosen_phase[0], (*afe_priv).mtkaif_chosen_phase[1]);
                    return 0;
                } else if snd_soc_dapm_widget_name_cmp(w, "ADDA6_MTKAIF_CFG\0".as_ptr() as *const i8) == 0
                    && (*afe_priv).mtkaif_chosen_phase[2] < 0
                {
                    dev_warn((*afe).dev, "%s(), mtkaif_chosen_phase[2]:%d\n\0".as_ptr() as *const i8, "__func__\0".as_ptr() as *const i8, (*afe_priv).mtkaif_chosen_phase[2]);
                    return 0;
                }

                /* mtkaif_rxif_clkinv_adc inverse for calibration */
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, MTKAIF_RXIF_CLKINV_ADC_MASK_SFT, 0x1 << MTKAIF_RXIF_CLKINV_ADC_SFT);
                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_CFG0, MTKAIF_RXIF_CLKINV_ADC_MASK_SFT, 0x1 << MTKAIF_RXIF_CLKINV_ADC_SFT);

                /* set delay for ch12 */
                if (*afe_priv).mtkaif_phase_cycle[0] >= (*afe_priv).mtkaif_phase_cycle[1] {
                    delay_data = DELAY_DATA_MISO1;
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[0] - (*afe_priv).mtkaif_phase_cycle[1];
                } else {
                    delay_data = DELAY_DATA_MISO2;
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[0];
                }

                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG2, MTKAIF_RXIF_DELAY_DATA_MASK_SFT, delay_data << MTKAIF_RXIF_DELAY_DATA_SFT);
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG2, MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT, delay_cycle << MTKAIF_RXIF_DELAY_CYCLE_SFT);

                /* set delay between ch3 and ch2 */
                if (*afe_priv).mtkaif_phase_cycle[2] >= (*afe_priv).mtkaif_phase_cycle[1] {
                    delay_data = DELAY_DATA_MISO1; /* ch3 */
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[2] - (*afe_priv).mtkaif_phase_cycle[1];
                } else {
                    delay_data = DELAY_DATA_MISO2; /* ch2 */
                    delay_cycle = (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[2];
                }

                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG2, MTKAIF_RXIF_DELAY_DATA_MASK_SFT, delay_data << MTKAIF_RXIF_DELAY_DATA_SFT);
                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG2, MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT, delay_cycle << MTKAIF_RXIF_DELAY_CYCLE_SFT);
            } else if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2 {
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x00010000);
                regmap_write((*afe).regmap, AFE_ADDA6_MTKAIF_CFG0, 0x00010000);
            } else {
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x0);
                regmap_write((*afe).regmap, AFE_ADDA6_MTKAIF_CFG0, 0x0);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_dl_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);

    match event {
        SND_SOC_DAPM_PRE_PMU => mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA, 0),
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA, 0);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_ch34_dl_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);

    match event {
        SND_SOC_DAPM_PRE_PMU => mt8192_afe_gpio_request((*afe).dev, true, MT8192_DAI_ADDA_CH34, 0),
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8192_afe_gpio_request((*afe).dev, false, MT8192_DAI_ADDA_CH34, 0);
        }
        _ => {}
    }

    0
}

/* stf */
unsafe fn stf_positive_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).stf_positive_gain_db as _;
    0
}

unsafe fn stf_positive_gain_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let gain_db: i32 = (*ucontrol).value.integer.value[0] as i32;
    let mut change: bool = false;

    (*afe_priv).stf_positive_gain_db = gain_db;

    if gain_db >= 0 && gain_db <= 24 {
        regmap_update_bits_check((*afe).regmap, AFE_SIDETONE_GAIN, POSITIVE_GAIN_MASK_SFT, (gain_db / 6) << POSITIVE_GAIN_SFT, &mut change);
    } else {
        return -EINVAL;
    }

    change as i32
}

unsafe fn mt8192_adda_dmic_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).mtkaif_dmic as _;
    0
}

unsafe fn mt8192_adda_dmic_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let dmic_on: i32 = (*ucontrol).value.integer.value[0] as i32;
    let change: bool = ((*afe_priv).mtkaif_dmic != dmic_on) || ((*afe_priv).mtkaif_dmic_ch34 != dmic_on);

    (*afe_priv).mtkaif_dmic = dmic_on;
    (*afe_priv).mtkaif_dmic_ch34 = dmic_on;

    change as i32
}

unsafe fn mt8192_adda6_only_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).mtkaif_adda6_only as _;
    0
}

unsafe fn mt8192_adda6_only_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;
    let mtkaif_adda6_only: i32 = (*ucontrol).value.integer.value[0] as i32;
    let change: bool = (*afe_priv).mtkaif_adda6_only != mtkaif_adda6_only;

    (*afe_priv).mtkaif_adda6_only = mtkaif_adda6_only;

    change as i32
}

static mtk_adda_controls: [snd_kcontrol_new; 5] = [
    SOC_SINGLE!("Sidetone_Gain", AFE_SIDETONE_GAIN, SIDE_TONE_GAIN_SFT, SIDE_TONE_GAIN_MASK, 0),
    SOC_SINGLE_EXT!("Sidetone_Positive_Gain_dB", SND_SOC_NOPM, 0, 24, 0, stf_positive_gain_get, stf_positive_gain_set),
    SOC_SINGLE!("ADDA_DL_GAIN", AFE_ADDA_DL_SRC2_CON1, DL_2_GAIN_CTL_PRE_SFT, DL_2_GAIN_CTL_PRE_MASK, 0),
    SOC_SINGLE_BOOL_EXT!("MTKAIF_DMIC Switch", 0, mt8192_adda_dmic_get, mt8192_adda_dmic_set),
    SOC_SINGLE_BOOL_EXT!("MTKAIF_ADDA6_ONLY Switch", 0, mt8192_adda6_only_get, mt8192_adda6_only_set),
];

static stf_ctl: snd_kcontrol_new = SOC_DAPM_SINGLE!("Switch", SND_SOC_NOPM, 0, 1, 0);

static stf_coeff_table_16k: [u16; 13] = [
    0x049C, 0x09E8, 0x09E0, 0x089C, 0xFF54, 0xF488, 0xEAFC, 0xEBAC, 0xfA40, 0x17AC, 0x3D1C, 0x6028, 0x7538,
];

static stf_coeff_table_32k: [u16; 29] = [
    0xFE52, 0x0042, 0x00C5, 0x0194, 0x029A, 0x03B7, 0x04BF, 0x057D, 0x05BE, 0x0555, 0x0426, 0x0230,
    0xFF92, 0xFC89, 0xF973, 0xF6C6, 0xF500, 0xF49D, 0xF603, 0xF970, 0xFEF3, 0x065F, 0x0F4F, 0x1928,
    0x2329, 0x2C80, 0x345E, 0x3A0D, 0x3D08,
];

static stf_coeff_table_48k: [u16; 31] = [
    0x0401, 0xFFB0, 0xFF5A, 0xFECE, 0xFE10, 0xFD28, 0xFC21, 0xFB08, 0xF9EF, 0xF8E8, 0xF80A, 0xF76C,
    0xF724, 0xF746, 0xF7E6, 0xF90F, 0xFACC, 0xFD1E, 0xFFFF, 0x0364, 0x0737, 0x0B62, 0x0FC1, 0x1431,
    0x188A, 0x1CA4, 0x2056, 0x237D, 0x25F9, 0x27B0, 0x2890,
];

unsafe fn mtk_stf_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: i32) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt);
    let half_tap_num: usize;
    let stf_coeff_table: *const u16;
    let mut ul_rate: u32 = 0;
    let mut reg_value: u32 = 0;
    let mut coef_addr: usize;

    regmap_read((*afe).regmap, AFE_ADDA_UL_SRC_CON0, &mut ul_rate);
    ul_rate = ul_rate >> UL_VOICE_MODE_CH1_CH2_CTL_SFT;
    ul_rate = ul_rate & UL_VOICE_MODE_CH1_CH2_CTL_MASK;

    if ul_rate == MTK_AFE_ADDA_UL_RATE_48K {
        half_tap_num = stf_coeff_table_48k.len();
        stf_coeff_table = stf_coeff_table_48k.as_ptr();
    } else if ul_rate == MTK_AFE_ADDA_UL_RATE_32K {
        half_tap_num = stf_coeff_table_32k.len();
        stf_coeff_table = stf_coeff_table_32k.as_ptr();
    } else {
        half_tap_num = stf_coeff_table_16k.len();
        stf_coeff_table = stf_coeff_table_16k.as_ptr();
    }

    regmap_read((*afe).regmap, AFE_SIDETONE_CON1, &mut reg_value);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            /* set side tone gain = 0 */
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_GAIN, SIDE_TONE_GAIN_MASK_SFT, 0);
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_GAIN, POSITIVE_GAIN_MASK_SFT, 0);
            /* don't bypass stf */
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_CON1, 0x1f << 27, 0x0);
            /* set stf half tap num */
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_CON1, SIDE_TONE_HALF_TAP_NUM_MASK_SFT, (half_tap_num as u32) << SIDE_TONE_HALF_TAP_NUM_SFT);

            /* set side tone coefficient */
            regmap_read((*afe).regmap, AFE_SIDETONE_CON0, &mut reg_value);
            coef_addr = 0;
            while coef_addr < half_tap_num {
                let old_w_ready: bool = ((reg_value >> W_RDY_SFT) & 0x1) != 0;
                let mut new_w_ready: bool = false;
                let mut try_cnt: i32 = 0;

                regmap_update_bits(
                    (*afe).regmap,
                    AFE_SIDETONE_CON0,
                    0x39FFFFF,
                    (1 << R_W_EN_SFT)
                        | (1 << R_W_SEL_SFT)
                        | (0 << SEL_CH2_SFT)
                        | ((coef_addr as u32) << SIDE_TONE_COEFFICIENT_ADDR_SFT)
                        | (*stf_coeff_table.add(coef_addr) as u32),
                );

                /* wait until flag write_ready changed */
                while try_cnt < 10 {
                    regmap_read((*afe).regmap, AFE_SIDETONE_CON0, &mut reg_value);
                    new_w_ready = ((reg_value >> W_RDY_SFT) & 0x1) != 0;

                    /* flip => ok */
                    if new_w_ready == old_w_ready {
                        udelay(3);
                        if try_cnt == 9 {
                            dev_warn((*afe).dev, "%s(), write coeff not ready\0".as_ptr() as *const i8, "__func__\0".as_ptr() as *const i8);
                        }
                    } else {
                        break;
                    }
                    try_cnt += 1;
                }
                /* need write -> read -> write to write next coeff */
                regmap_update_bits((*afe).regmap, AFE_SIDETONE_CON0, R_W_SEL_MASK_SFT, 0x0);
                coef_addr += 1;
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* bypass stf */
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_CON1, 0x1f << 27, 0x1f << 27);
            /* set side tone gain = 0 */
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_GAIN, SIDE_TONE_GAIN_MASK_SFT, 0);
            regmap_update_bits((*afe).regmap, AFE_SIDETONE_GAIN, POSITIVE_GAIN_MASK_SFT, 0);
        }
        _ => {}
    }

    0
}

/* stf mux */
const STF_SRC_ADDA_ADDA6: i32 = 0;
const STF_SRC_O19O20: i32 = 1;

static stf_o19o20_mux_map: [&str; 2] = ["ADDA_ADDA6", "O19O20"];
static mut stf_o19o20_mux_map_value: [i32; 2] = [STF_SRC_ADDA_ADDA6, STF_SRC_O19O20];

SOC_VALUE_ENUM_SINGLE_DECL!(stf_o19o20_mux_map_enum, AFE_SIDETONE_CON1, STF_SOURCE_FROM_O19O20_SFT, STF_SOURCE_FROM_O19O20_MASK, stf_o19o20_mux_map, stf_o19o20_mux_map_value);

static stf_o19O20_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("STF_O19O20_MUX", stf_o19o20_mux_map_enum);

const STF_SRC_ADDA: i32 = 0;
const STF_SRC_ADDA6: i32 = 1;

static stf_adda_mux_map: [&str; 2] = ["ADDA", "ADDA6"];
static mut stf_adda_mux_map_value: [i32; 2] = [STF_SRC_ADDA, STF_SRC_ADDA6];

SOC_VALUE_ENUM_SINGLE_DECL!(stf_adda_mux_map_enum, AFE_SIDETONE_CON1, STF_O19O20_OUT_EN_SEL_SFT, STF_O19O20_OUT_EN_SEL_MASK, stf_adda_mux_map, stf_adda_mux_map_value);

static stf_adda_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("STF_ADDA_MUX", stf_adda_mux_map_enum);

/* ADDA UL MUX */
const ADDA_UL_MUX_MTKAIF: i32 = 0;
const ADDA_UL_MUX_AP_DMIC: i32 = 1;
const ADDA_UL_MUX_MASK: i32 = 0x1;

static adda_ul_mux_map: [&str; 2] = ["MTKAIF", "AP_DMIC"];
static mut adda_ul_map_value: [i32; 2] = [ADDA_UL_MUX_MTKAIF, ADDA_UL_MUX_AP_DMIC];

SOC_VALUE_ENUM_SINGLE_DECL!(adda_ul_mux_map_enum, SND_SOC_NOPM, 0, ADDA_UL_MUX_MASK, adda_ul_mux_map, adda_ul_map_value);

static adda_ul_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("ADDA_UL_MUX Select", adda_ul_mux_map_enum);
static adda_ch34_ul_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("ADDA_CH34_UL_MUX Select", adda_ul_mux_map_enum);

static mtk_dai_adda_widgets: [snd_soc_dapm_widget; 33] = [
    /* inter-connections */
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH1", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch1_mix, mtk_adda_dl_ch1_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH2", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch2_mix, mtk_adda_dl_ch2_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH3", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch3_mix, mtk_adda_dl_ch3_mix.len()),
    SND_SOC_DAPM_MIXER!("ADDA_DL_CH4", SND_SOC_NOPM, 0, 0, mtk_adda_dl_ch4_mix, mtk_adda_dl_ch4_mix.len()),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Enable", SUPPLY_SEQ_ADDA_AFE_ON, AFE_ADDA_UL_DL_CON0, ADDA_AFE_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Playback Enable", SUPPLY_SEQ_ADDA_DL_ON, AFE_ADDA_DL_SRC2_CON0, DL_2_SRC_ON_TMP_CTL_PRE_SFT, 0, mtk_adda_dl_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA CH34 Playback Enable", SUPPLY_SEQ_ADDA_DL_ON, AFE_ADDA_3RD_DAC_DL_SRC2_CON0, DL_2_SRC_ON_TMP_CTL_PRE_SFT, 0, mtk_adda_ch34_dl_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA_UL_SRC_CON0, UL_SRC_ON_TMP_CTL_SFT, 0, mtk_adda_ul_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA CH34 Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA6_UL_SRC_CON0, UL_SRC_ON_TMP_CTL_SFT, 0, mtk_adda_ch34_ul_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("AUD_PAD_TOP", SUPPLY_SEQ_ADDA_AUD_PAD_TOP, AFE_AUD_PAD_TOP, RG_RX_FIFO_ON_SFT, 0, mtk_adda_pad_top_event, SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_MTKAIF_CFG", SUPPLY_SEQ_ADDA_MTKAIF_CFG, SND_SOC_NOPM, 0, 0, mtk_adda_mtkaif_cfg_event, SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("ADDA6_MTKAIF_CFG", SUPPLY_SEQ_ADDA6_MTKAIF_CFG, SND_SOC_NOPM, 0, 0, mtk_adda_mtkaif_cfg_event, SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_EN", SUPPLY_SEQ_ADDA_AP_DMIC, AFE_ADDA_UL_SRC_CON0, UL_AP_DMIC_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("AP_DMIC_CH34_EN", SUPPLY_SEQ_ADDA_AP_DMIC, AFE_ADDA6_UL_SRC_CON0, UL_AP_DMIC_ON_SFT, 0, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_FIFO", SUPPLY_SEQ_ADDA_FIFO, AFE_ADDA_UL_DL_CON0, AFE_ADDA_FIFO_AUTO_RST_SFT, 1, None, 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_CH34_FIFO", SUPPLY_SEQ_ADDA_FIFO, AFE_ADDA_UL_DL_CON0, AFE_ADDA6_FIFO_AUTO_RST_SFT, 1, None, 0),
    SND_SOC_DAPM_MUX!("ADDA_UL_Mux", SND_SOC_NOPM, 0, 0, &adda_ul_mux_control),
    SND_SOC_DAPM_MUX!("ADDA_CH34_UL_Mux", SND_SOC_NOPM, 0, 0, &adda_ch34_ul_mux_control),
    SND_SOC_DAPM_INPUT!("AP_DMIC_INPUT"),
    SND_SOC_DAPM_INPUT!("AP_DMIC_CH34_INPUT"),
    /* stf */
    SND_SOC_DAPM_SWITCH_E!("Sidetone Filter", AFE_SIDETONE_CON1, SIDE_TONE_ON_SFT, 0, &stf_ctl, mtk_stf_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_MUX!("STF_O19O20_MUX", SND_SOC_NOPM, 0, 0, &stf_o19O20_mux_control),
    SND_SOC_DAPM_MUX!("STF_ADDA_MUX", SND_SOC_NOPM, 0, 0, &stf_adda_mux_control),
    SND_SOC_DAPM_MIXER!("STF_CH1", SND_SOC_NOPM, 0, 0, mtk_stf_ch1_mix, mtk_stf_ch1_mix.len()),
    SND_SOC_DAPM_MIXER!("STF_CH2", SND_SOC_NOPM, 0, 0, mtk_stf_ch2_mix, mtk_stf_ch2_mix.len()),
    SND_SOC_DAPM_OUTPUT!("STF_OUTPUT"),
    /* clock */
    SND_SOC_DAPM_CLOCK_SUPPLY!("top_mux_audio_h"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac_clk"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac_predis_clk"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_3rd_dac_clk"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_3rd_dac_predis_clk"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adc_clk"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adda6_adc_clk"),
];

static mtk_dai_adda_routes: [snd_soc_dapm_route; 79] = [
    /* playback */
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL1_CH1", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL1_CH1", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL1_CH2", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL12_CH1", source: "DL12" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL12_CH2", source: "DL12" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL6_CH1", source: "DL6" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL6_CH2", source: "DL6" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL8_CH1", source: "DL8" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL8_CH2", source: "DL8" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL2_CH1", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL2_CH1", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL2_CH2", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL3_CH1", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL3_CH1", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL3_CH2", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL4_CH1", source: "DL4" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL4_CH2", source: "DL4" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH1", control: "DL5_CH1", source: "DL5" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH2", control: "DL5_CH2", source: "DL5" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "ADDA_DL_CH1" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "ADDA_DL_CH2" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "ADDA Playback Enable" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL1_CH1", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL1_CH1", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL1_CH2", source: "DL1" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL12_CH1", source: "DL12" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL12_CH2", source: "DL12" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL6_CH1", source: "DL6" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL6_CH2", source: "DL6" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL2_CH1", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL2_CH1", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL2_CH2", source: "DL2" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL3_CH1", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL3_CH1", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL3_CH2", source: "DL3" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL4_CH1", source: "DL4" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL4_CH2", source: "DL4" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH3", control: "DL5_CH1", source: "DL5" },
    snd_soc_dapm_route { sink: "ADDA_DL_CH4", control: "DL5_CH2", source: "DL5" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "ADDA_DL_CH3" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "ADDA_DL_CH4" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "ADDA CH34 Playback Enable" },
    /* capture */
    snd_soc_dapm_route { sink: "ADDA_UL_Mux", control: "MTKAIF", source: "ADDA Capture" },
    snd_soc_dapm_route { sink: "ADDA_UL_Mux", control: "AP_DMIC", source: "AP DMIC Capture" },
    snd_soc_dapm_route { sink: "ADDA_CH34_UL_Mux", control: "MTKAIF", source: "ADDA CH34 Capture" },
    snd_soc_dapm_route { sink: "ADDA_CH34_UL_Mux", control: "AP_DMIC", source: "AP DMIC CH34 Capture" },
    snd_soc_dapm_route { sink: "ADDA Capture", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "ADDA Capture", control: core::ptr::null(), source: "ADDA Capture Enable" },
    snd_soc_dapm_route { sink: "ADDA Capture", control: core::ptr::null(), source: "AUD_PAD_TOP" },
    snd_soc_dapm_route { sink: "ADDA Capture", control: core::ptr::null(), source: "ADDA_MTKAIF_CFG" },
    snd_soc_dapm_route { sink: "AP DMIC Capture", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "AP DMIC Capture", control: core::ptr::null(), source: "ADDA Capture Enable" },
    snd_soc_dapm_route { sink: "AP DMIC Capture", control: core::ptr::null(), source: "ADDA_FIFO" },
    snd_soc_dapm_route { sink: "AP DMIC Capture", control: core::ptr::null(), source: "AP_DMIC_EN" },
    snd_soc_dapm_route { sink: "ADDA CH34 Capture", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "ADDA CH34 Capture", control: core::ptr::null(), source: "ADDA CH34 Capture Enable" },
    snd_soc_dapm_route { sink: "ADDA CH34 Capture", control: core::ptr::null(), source: "AUD_PAD_TOP" },
    snd_soc_dapm_route { sink: "ADDA CH34 Capture", control: core::ptr::null(), source: "ADDA6_MTKAIF_CFG" },
    snd_soc_dapm_route { sink: "AP DMIC CH34 Capture", control: core::ptr::null(), source: "ADDA Enable" },
    snd_soc_dapm_route { sink: "AP DMIC CH34 Capture", control: core::ptr::null(), source: "ADDA CH34 Capture Enable" },
    snd_soc_dapm_route { sink: "AP DMIC CH34 Capture", control: core::ptr::null(), source: "ADDA_CH34_FIFO" },
    snd_soc_dapm_route { sink: "AP DMIC CH34 Capture", control: core::ptr::null(), source: "AP_DMIC_CH34_EN" },
    snd_soc_dapm_route { sink: "AP DMIC Capture", control: core::ptr::null(), source: "AP_DMIC_INPUT" },
    snd_soc_dapm_route { sink: "AP DMIC CH34 Capture", control: core::ptr::null(), source: "AP_DMIC_CH34_INPUT" },
    /* sidetone filter */
    snd_soc_dapm_route { sink: "STF_ADDA_MUX", control: "ADDA", source: "ADDA_UL_Mux" },
    snd_soc_dapm_route { sink: "STF_ADDA_MUX", control: "ADDA6", source: "ADDA_CH34_UL_Mux" },
    snd_soc_dapm_route { sink: "STF_O19O20_MUX", control: "ADDA_ADDA6", source: "STF_ADDA_MUX" },
    snd_soc_dapm_route { sink: "STF_O19O20_MUX", control: "O19O20", source: "STF_CH1" },
    snd_soc_dapm_route { sink: "STF_O19O20_MUX", control: "O19O20", source: "STF_CH2" },
    snd_soc_dapm_route { sink: "Sidetone Filter", control: "Switch", source: "STF_O19O20_MUX" },
    snd_soc_dapm_route { sink: "STF_OUTPUT", control: core::ptr::null(), source: "Sidetone Filter" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "Sidetone Filter" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "Sidetone Filter" },
    /* clk */
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "aud_dac_clk" },
    snd_soc_dapm_route { sink: "ADDA Playback", control: core::ptr::null(), source: "aud_dac_predis_clk" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "aud_3rd_dac_clk" },
    snd_soc_dapm_route { sink: "ADDA CH34 Playback", control: core::ptr::null(), source: "aud_3rd_dac_predis_clk" },
    snd_soc_dapm_route { sink: "ADDA Capture Enable", control: core::ptr::null(), source: "aud_adc_clk" },
    snd_soc_dapm_route { sink: "ADDA CH34 Capture Enable", control: core::ptr::null(), source: "aud_adda6_adc_clk" },
];

/* dai ops */
unsafe fn mtk_dai_adda_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> i32 {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let rate: u32 = params_rate(params);
    let id: i32 = (*dai).id;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        let mut dl_src2_con0: u32 = 0;
        let mut dl_src2_con1: u32 = 0;

        /* set sampling rate */
        dl_src2_con0 = mtk_adda_dl_rate_transform(afe, rate) << DL_2_INPUT_MODE_CTL_SFT;
        /* set output mode, UP_SAMPLING_RATE_X8 */
        dl_src2_con0 |= 0x3 << DL_2_OUTPUT_SEL_CTL_SFT;
        /* turn off mute function */
        dl_src2_con0 |= 0x01 << DL_2_MUTE_CH2_OFF_CTL_PRE_SFT;
        dl_src2_con0 |= 0x01 << DL_2_MUTE_CH1_OFF_CTL_PRE_SFT;
        /* set voice input data if input sample rate is 8k or 16k */
        if rate == 8000 || rate == 16000 {
            dl_src2_con0 |= 0x01 << DL_2_VOICE_MODE_CTL_PRE_SFT;
        }
        /* SA suggest apply -0.3db to audio/speech path */
        dl_src2_con1 = MTK_AFE_ADDA_DL_GAIN_NORMAL << DL_2_GAIN_CTL_PRE_SFT;
        /* turn on down-link gain */
        dl_src2_con0 |= 0x01 << DL_2_GAIN_ON_CTL_PRE_SFT;

        if id == MT8192_DAI_ADDA {
            /* clean predistortion */
            regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON0, 0);
            regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON1, 0);
            regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, dl_src2_con0);
            regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON1, dl_src2_con1);
            /* set sdm gain */
            regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, ATTGAIN_CTL_MASK_SFT, AUDIO_SDM_LEVEL_NORMAL << ATTGAIN_CTL_SFT);
            /* 2nd sdm */
            regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, USE_3RD_SDM_MASK_SFT, AUDIO_SDM_2ND << USE_3RD_SDM_SFT);
            /* sdm auto reset */
            regmap_write((*afe).regmap, AFE_ADDA_DL_SDM_AUTO_RESET_CON, SDM_AUTO_RESET_THRESHOLD);
            regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_AUTO_RESET_CON, ADDA_SDM_AUTO_RESET_ONOFF_MASK_SFT, 0x1 << ADDA_SDM_AUTO_RESET_ONOFF_SFT);
        } else {
            /* clean predistortion */
            regmap_write((*afe).regmap, AFE_ADDA_3RD_DAC_PREDIS_CON0, 0);
            regmap_write((*afe).regmap, AFE_ADDA_3RD_DAC_PREDIS_CON1, 0);
            regmap_write((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SRC2_CON0, dl_src2_con0);
            regmap_write((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SRC2_CON1, dl_src2_con1);
            /* set sdm gain */
            regmap_update_bits((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SDM_DCCOMP_CON, ATTGAIN_CTL_MASK_SFT, AUDIO_SDM_LEVEL_NORMAL << ATTGAIN_CTL_SFT);
            /* 2nd sdm */
            regmap_update_bits((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SDM_DCCOMP_CON, USE_3RD_SDM_MASK_SFT, AUDIO_SDM_2ND << USE_3RD_SDM_SFT);
            /* sdm auto reset */
            regmap_write((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SDM_AUTO_RESET_CON, SDM_AUTO_RESET_THRESHOLD);
            regmap_update_bits((*afe).regmap, AFE_ADDA_3RD_DAC_DL_SDM_AUTO_RESET_CON, ADDA_3RD_DAC_SDM_AUTO_RESET_ONOFF_MASK_SFT, 0x1 << ADDA_3RD_DAC_SDM_AUTO_RESET_ONOFF_SFT);
        }
    } else {
        let mut voice_mode: u32 = 0;
        let mut ul_src_con0: u32 = 0; /* default value */

        voice_mode = mtk_adda_ul_rate_transform(afe, rate);
        ul_src_con0 |= (voice_mode << 17) & (0x7 << 17);
        /* enable iir */
        ul_src_con0 |= (1 << UL_IIR_ON_TMP_CTL_SFT) & UL_IIR_ON_TMP_CTL_MASK_SFT;
        ul_src_con0 |= (UL_IIR_SW << UL_IIRMODE_CTL_SFT) & UL_IIRMODE_CTL_MASK_SFT;

        match id {
            MT8192_DAI_ADDA | MT8192_DAI_AP_DMIC => {
                /* 35Hz @ 48k */
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_02_01, 0x00000000);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_04_03, 0x00003FB8);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_06_05, 0x3FB80000);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_08_07, 0x3FB80000);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_10_09, 0x0000C048);
                regmap_write((*afe).regmap, AFE_ADDA_UL_SRC_CON0, ul_src_con0);
                /* Using Internal ADC */
                regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1 << 0, 0x0 << 0);
                /* mtkaif_rxif_data_mode = 0, amic */
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x1 << 0, 0x0 << 0);
            }
            MT8192_DAI_ADDA_CH34 | MT8192_DAI_AP_DMIC_CH34 => {
                /* 35Hz @ 48k */
                regmap_write((*afe).regmap, AFE_ADDA6_IIR_COEF_02_01, 0x00000000);
                regmap_write((*afe).regmap, AFE_ADDA6_IIR_COEF_04_03, 0x00003FB8);
                regmap_write((*afe).regmap, AFE_ADDA6_IIR_COEF_06_05, 0x3FB80000);
                regmap_write((*afe).regmap, AFE_ADDA6_IIR_COEF_08_07, 0x3FB80000);
                regmap_write((*afe).regmap, AFE_ADDA6_IIR_COEF_10_09, 0x0000C048);
                regmap_write((*afe).regmap, AFE_ADDA6_UL_SRC_CON0, ul_src_con0);
                /* Using Internal ADC */
                regmap_update_bits((*afe).regmap, AFE_ADDA6_TOP_CON0, 0x1 << 0, 0x0 << 0);
                /* mtkaif_rxif_data_mode = 0, amic */
                regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG0, 0x1 << 0, 0x0 << 0);
            }
            _ => {}
        }

        /* ap dmic */
        match id {
            MT8192_DAI_AP_DMIC | MT8192_DAI_AP_DMIC_CH34 => {
                mtk_adda_ul_src_dmic(afe, id);
            }
            _ => {}
        }
    }

    0
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
};

/* dai driver */
const MTK_ADDA_PLAYBACK_RATES: u32 = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const MTK_ADDA_CAPTURE_RATES: u32 = SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;
const MTK_ADDA_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: "ADDA",
        id: MT8192_DAI_ADDA,
        playback: snd_soc_pcm_stream {
            stream_name: "ADDA Playback",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "ADDA Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: "ADDA_CH34",
        id: MT8192_DAI_ADDA_CH34,
        playback: snd_soc_pcm_stream {
            stream_name: "ADDA CH34 Playback",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "ADDA CH34 Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: "AP_DMIC",
        id: MT8192_DAI_AP_DMIC,
        capture: snd_soc_pcm_stream {
            stream_name: "AP DMIC Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: "AP_DMIC_CH34",
        id: MT8192_DAI_AP_DMIC_CH34,
        capture: snd_soc_pcm_stream {
            stream_name: "AP DMIC CH34 Capture",
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

pub unsafe fn mt8192_dai_adda_register(afe: *mut mtk_base_afe) -> i32 {
    let dai: *mut mtk_base_afe_dai;
    let afe_priv: *mut mt8192_afe_private = (*afe).platform_priv as *mut mt8192_afe_private;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as _;

    (*dai).controls = mtk_adda_controls.as_ptr();
    (*dai).num_controls = mtk_adda_controls.len() as _;
    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as _;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as _;

    /* ap dmic priv share with adda */
    (*afe_priv).dai_priv[MT8192_DAI_AP_DMIC as usize] = (*afe_priv).dai_priv[MT8192_DAI_ADDA as usize];
    (*afe_priv).dai_priv[MT8192_DAI_AP_DMIC_CH34 as usize] = (*afe_priv).dai_priv[MT8192_DAI_ADDA_CH34 as usize];

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
