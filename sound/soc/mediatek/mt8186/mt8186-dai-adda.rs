// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI ADDA Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// Depends on Linux regmap/delay, mt8186 AFE clock/common/gpio/interconnection,
// and common MediaTek DAI ADDA definitions.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const UL_IIR_SW: c_int = 0;
const UL_IIR_5HZ: c_int = 1;
const UL_IIR_10HZ: c_int = 2;
const UL_IIR_25HZ: c_int = 3;
const UL_IIR_50HZ: c_int = 4;
const UL_IIR_75HZ: c_int = 5;

const AUDIO_SDM_LEVEL_MUTE: c_int = 0;
const AUDIO_SDM_LEVEL_NORMAL: c_int = 0x1d;
/* if you change level normal */
/* you need to change formula of hp impedance and dc trim too */

const AUDIO_SDM_2ND: c_int = 0;
const AUDIO_SDM_3RD: c_int = 1;

const SDM_AUTO_RESET_THRESHOLD: c_uint = 0x190000;

#[repr(C)]
struct mtk_afe_adda_priv {
    dl_rate: c_int,
    ul_rate: c_int,
}

extern "C" {
    static mut mtk_adda_dl_ch1_mix: [snd_kcontrol_new; 15];
    static mut mtk_adda_dl_ch2_mix: [snd_kcontrol_new; 18];
}

unsafe fn get_adda_priv_by_name(
    afe: *mut mtk_base_afe,
    name: *const c_char,
) -> *mut mtk_afe_adda_priv {
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let dai_id: c_int;

    if strncmp(name, c"aud_dac".as_ptr(), 7) == 0 || strncmp(name, c"aud_adc".as_ptr(), 7) == 0 {
        dai_id = MT8186_DAI_ADDA;
    } else {
        return ptr::null_mut();
    }

    (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_adda_priv
}

/* dai component */
// static const struct snd_kcontrol_new mtk_adda_dl_ch1_mix[] = {
//     SOC_DAPM_SINGLE_AUTODISABLE("DL1_CH1 Switch", AFE_CONN3, I_DL1_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL12_CH1 Switch", AFE_CONN3, I_DL12_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH1 Switch", AFE_CONN3, I_DL2_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL3_CH1 Switch", AFE_CONN3, I_DL3_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL4_CH1 Switch", AFE_CONN3_1, I_DL4_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL5_CH1 Switch", AFE_CONN3_1, I_DL5_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL6_CH1 Switch", AFE_CONN3_1, I_DL6_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL8_CH1 Switch", AFE_CONN3_1, I_DL8_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH2 Switch", AFE_CONN3, I_ADDA_UL_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH1 Switch", AFE_CONN3, I_ADDA_UL_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("GAIN1_OUT_CH1 Switch", AFE_CONN3, I_GAIN1_OUT_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("PCM_1_CAP_CH1 Switch", AFE_CONN3, I_PCM_1_CAP_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("PCM_2_CAP_CH1 Switch", AFE_CONN3, I_PCM_2_CAP_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("SRC_1_OUT_CH1 Switch", AFE_CONN3_1, I_SRC_1_OUT_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("SRC_2_OUT_CH1 Switch", AFE_CONN3_1, I_SRC_2_OUT_CH1, 1, 0),
// };
//
// static const struct snd_kcontrol_new mtk_adda_dl_ch2_mix[] = {
//     SOC_DAPM_SINGLE_AUTODISABLE("DL1_CH1 Switch", AFE_CONN4, I_DL1_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL1_CH2 Switch", AFE_CONN4, I_DL1_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL12_CH2 Switch", AFE_CONN4, I_DL12_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH1 Switch", AFE_CONN4, I_DL2_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH2 Switch", AFE_CONN4, I_DL2_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL3_CH1 Switch", AFE_CONN4, I_DL3_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL3_CH2 Switch", AFE_CONN4, I_DL3_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL4_CH2 Switch", AFE_CONN4_1, I_DL4_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL5_CH2 Switch", AFE_CONN4_1, I_DL5_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL6_CH2 Switch", AFE_CONN4_1, I_DL6_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("DL8_CH2 Switch", AFE_CONN4_1, I_DL8_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH2 Switch", AFE_CONN4, I_ADDA_UL_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH1 Switch", AFE_CONN4, I_ADDA_UL_CH1, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("GAIN1_OUT_CH2 Switch", AFE_CONN4, I_GAIN1_OUT_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("PCM_1_CAP_CH2 Switch", AFE_CONN4, I_PCM_1_CAP_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("PCM_2_CAP_CH2 Switch", AFE_CONN4, I_PCM_2_CAP_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("SRC_1_OUT_CH2 Switch", AFE_CONN4_1, I_SRC_1_OUT_CH2, 1, 0),
//     SOC_DAPM_SINGLE_AUTODISABLE("SRC_2_OUT_CH2 Switch", AFE_CONN4_1, I_SRC_2_OUT_CH2, 1, 0),
// };

const SUPPLY_SEQ_ADDA_AFE_ON: c_int = 0;
const SUPPLY_SEQ_ADDA_DL_ON: c_int = 1;
const SUPPLY_SEQ_ADDA_AUD_PAD_TOP: c_int = 2;
const SUPPLY_SEQ_ADDA_MTKAIF_CFG: c_int = 3;
const SUPPLY_SEQ_ADDA_FIFO: c_int = 4;
const SUPPLY_SEQ_ADDA_AP_DMIC: c_int = 5;
const SUPPLY_SEQ_ADDA_UL_ON: c_int = 6;

unsafe fn mtk_adda_ul_src_dmic(afe: *mut mtk_base_afe, id: c_int) -> c_int {
    let reg: c_uint;

    match id {
        MT8186_DAI_ADDA | MT8186_DAI_AP_DMIC => {
            reg = AFE_ADDA_UL_SRC_CON0;
        }
        _ => return -EINVAL,
    }

    /* dmic mode, 3.25M*/
    regmap_update_bits((*afe).regmap, reg, DIGMIC_3P25M_1P625M_SEL_MASK_SFT, 0);
    regmap_update_bits((*afe).regmap, reg, DMIC_LOW_POWER_CTL_MASK_SFT, 0);

    /* turn on dmic, ch1, ch2 */
    regmap_update_bits(
        (*afe).regmap,
        reg,
        UL_SDM_3_LEVEL_MASK_SFT,
        BIT(UL_SDM_3_LEVEL_SFT),
    );
    regmap_update_bits(
        (*afe).regmap,
        reg,
        UL_MODE_3P25M_CH1_CTL_MASK_SFT,
        BIT(UL_MODE_3P25M_CH1_CTL_SFT),
    );
    regmap_update_bits(
        (*afe).regmap,
        reg,
        UL_MODE_3P25M_CH2_CTL_MASK_SFT,
        BIT(UL_MODE_3P25M_CH2_CTL_SFT),
    );

    0
}

unsafe fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mtkaif_dmic = (*afe_priv).mtkaif_dmic;

    dev_dbg(
        (*afe).dev,
        c"%s(), name %s, event 0x%x, mtkaif_dmic %d\n".as_ptr(),
        c"mtk_adda_ul_event".as_ptr(),
        (*w).name,
        event,
        mtkaif_dmic,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8186_afe_gpio_request((*afe).dev, true, MT8186_DAI_ADDA, 1);

            /* update setting to dmic */
            if mtkaif_dmic != 0 {
                /* mtkaif_rxif_data_mode = 1, dmic */
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x1, 0x1);

                /* dmic mode, 3.25M*/
                regmap_update_bits(
                    (*afe).regmap,
                    AFE_ADDA_MTKAIF_RX_CFG0,
                    MTKAIF_RXIF_VOICE_MODE_MASK_SFT,
                    0x0,
                );
                mtk_adda_ul_src_dmic(afe, MT8186_DAI_ADDA);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8186_afe_gpio_request((*afe).dev, false, MT8186_DAI_ADDA, 1);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_pad_top_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
                regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x39);
            } else {
                regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x31);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_mtkaif_cfg_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut delay_data: c_int;
    let mut delay_cycle: c_int;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2_CLK_P2 {
                /* set protocol 2 */
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x10000);
                /* mtkaif_rxif_clkinv_adc inverse */
                regmap_update_bits(
                    (*afe).regmap,
                    AFE_ADDA_MTKAIF_CFG0,
                    MTKAIF_RXIF_CLKINV_ADC_MASK_SFT,
                    BIT(MTKAIF_RXIF_CLKINV_ADC_SFT),
                );

                if snd_soc_dapm_widget_name_cmp(w, c"ADDA_MTKAIF_CFG".as_ptr()) == 0 {
                    if (*afe_priv).mtkaif_chosen_phase[0] < 0
                        && (*afe_priv).mtkaif_chosen_phase[1] < 0
                    {
                        dev_err(
                            (*afe).dev,
                            c"%s(), calib fail mtkaif_chosen_phase[0/1]:%d/%d\n".as_ptr(),
                            c"mtk_adda_mtkaif_cfg_event".as_ptr(),
                            (*afe_priv).mtkaif_chosen_phase[0],
                            (*afe_priv).mtkaif_chosen_phase[1],
                        );
                    } else if (*afe_priv).mtkaif_chosen_phase[0] < 0
                        || (*afe_priv).mtkaif_chosen_phase[1] < 0
                    {
                        dev_err(
                            (*afe).dev,
                            c"%s(), skip delay setting mtkaif_chosen_phase[0/1]:%d/%d\n".as_ptr(),
                            c"mtk_adda_mtkaif_cfg_event".as_ptr(),
                            (*afe_priv).mtkaif_chosen_phase[0],
                            (*afe_priv).mtkaif_chosen_phase[1],
                        );
                    } else {
                        if (*afe_priv).mtkaif_phase_cycle[0] >= (*afe_priv).mtkaif_phase_cycle[1] {
                            delay_data = DELAY_DATA_MISO1;
                            delay_cycle =
                                (*afe_priv).mtkaif_phase_cycle[0] - (*afe_priv).mtkaif_phase_cycle[1];
                        } else {
                            delay_data = DELAY_DATA_MISO2;
                            delay_cycle =
                                (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[0];
                        }

                        regmap_update_bits(
                            (*afe).regmap,
                            AFE_ADDA_MTKAIF_RX_CFG2,
                            MTKAIF_RXIF_DELAY_DATA_MASK_SFT,
                            (delay_data as c_uint) << MTKAIF_RXIF_DELAY_DATA_SFT,
                        );

                        regmap_update_bits(
                            (*afe).regmap,
                            AFE_ADDA_MTKAIF_RX_CFG2,
                            MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT,
                            (delay_cycle as c_uint) << MTKAIF_RXIF_DELAY_CYCLE_SFT,
                        );
                    }
                } else {
                    if (*afe_priv).mtkaif_phase_cycle[0] >= (*afe_priv).mtkaif_phase_cycle[1] {
                        delay_data = DELAY_DATA_MISO1;
                        delay_cycle =
                            (*afe_priv).mtkaif_phase_cycle[0] - (*afe_priv).mtkaif_phase_cycle[1];
                    } else {
                        delay_data = DELAY_DATA_MISO2;
                        delay_cycle =
                            (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[0];
                    }

                    regmap_update_bits(
                        (*afe).regmap,
                        AFE_ADDA_MTKAIF_RX_CFG2,
                        MTKAIF_RXIF_DELAY_DATA_MASK_SFT,
                        (delay_data as c_uint) << MTKAIF_RXIF_DELAY_DATA_SFT,
                    );

                    regmap_update_bits(
                        (*afe).regmap,
                        AFE_ADDA_MTKAIF_RX_CFG2,
                        MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT,
                        (delay_cycle as c_uint) << MTKAIF_RXIF_DELAY_CYCLE_SFT,
                    );
                }
            } else if (*afe_priv).mtkaif_protocol == MTKAIF_PROTOCOL_2 {
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x10000);
            } else {
                regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0);
            }
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_dl_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;

    dev_dbg(
        (*afe).dev,
        c"%s(), name %s, event 0x%x\n".as_ptr(),
        c"mtk_adda_dl_event".as_ptr(),
        (*w).name,
        event,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8186_afe_gpio_request((*afe).dev, true, MT8186_DAI_ADDA, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
            mt8186_afe_gpio_request((*afe).dev, false, MT8186_DAI_ADDA, 0);
        }
        _ => {}
    }

    0
}

unsafe fn mt8186_adda_dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;

    (*ucontrol).value.integer.value[0] = (*afe_priv).mtkaif_dmic as _;

    0
}

unsafe fn mt8186_adda_dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let dmic_on: c_int;

    dmic_on = (*ucontrol).value.integer.value[0] as c_int;

    dev_dbg(
        (*afe).dev,
        c"%s(), kcontrol name %s, dmic_on %d\n".as_ptr(),
        c"mt8186_adda_dmic_set".as_ptr(),
        (*kcontrol).id.name.as_ptr(),
        dmic_on,
    );

    if (*afe_priv).mtkaif_dmic == dmic_on {
        return 0;
    }

    (*afe_priv).mtkaif_dmic = dmic_on;

    1
}

// static const struct snd_kcontrol_new mtk_adda_controls[] = {
//     SOC_SINGLE("ADDA_DL_GAIN", AFE_ADDA_DL_SRC2_CON1,
//                DL_2_GAIN_CTL_PRE_SFT, DL_2_GAIN_CTL_PRE_MASK, 0),
//     SOC_SINGLE_BOOL_EXT("MTKAIF_DMIC Switch", 0,
//                         mt8186_adda_dmic_get, mt8186_adda_dmic_set),
// };

/* ADDA UL MUX */
const ADDA_UL_MUX_MTKAIF: c_int = 0;
const ADDA_UL_MUX_AP_DMIC: c_int = 1;
const ADDA_UL_MUX_MASK: c_int = 0x1;

static adda_ul_mux_map: [*const c_char; 2] = [c"MTKAIF".as_ptr(), c"AP_DMIC".as_ptr()];

static mut adda_ul_map_value: [c_int; 2] = [ADDA_UL_MUX_MTKAIF, ADDA_UL_MUX_AP_DMIC];

// static SOC_VALUE_ENUM_SINGLE_DECL(adda_ul_mux_map_enum,
//                                   SND_SOC_NOPM,
//                                   0,
//                                   ADDA_UL_MUX_MASK,
//                                   adda_ul_mux_map,
//                                   adda_ul_map_value);
//
// static const struct snd_kcontrol_new adda_ul_mux_control =
//     SOC_DAPM_ENUM("ADDA_UL_MUX Select", adda_ul_mux_map_enum);
//
// static const struct snd_soc_dapm_widget mtk_dai_adda_widgets[] = {
//     /* inter-connections */
//     SND_SOC_DAPM_MIXER("ADDA_DL_CH1", SND_SOC_NOPM, 0, 0,
//                        mtk_adda_dl_ch1_mix,
//                        ARRAY_SIZE(mtk_adda_dl_ch1_mix)),
//     SND_SOC_DAPM_MIXER("ADDA_DL_CH2", SND_SOC_NOPM, 0, 0,
//                        mtk_adda_dl_ch2_mix,
//                        ARRAY_SIZE(mtk_adda_dl_ch2_mix)),
//     SND_SOC_DAPM_SUPPLY_S("ADDA Enable", SUPPLY_SEQ_ADDA_AFE_ON,
//                           AFE_ADDA_UL_DL_CON0, ADDA_AFE_ON_SFT, 0, NULL, 0),
//     SND_SOC_DAPM_SUPPLY_S("ADDA Playback Enable", SUPPLY_SEQ_ADDA_DL_ON,
//                           AFE_ADDA_DL_SRC2_CON0, DL_2_SRC_ON_CTL_PRE_SFT, 0,
//                           mtk_adda_dl_event,
//                           SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
//     SND_SOC_DAPM_SUPPLY_S("ADDA Capture Enable", SUPPLY_SEQ_ADDA_UL_ON,
//                           AFE_ADDA_UL_SRC_CON0, UL_SRC_ON_CTL_SFT, 0,
//                           mtk_adda_ul_event,
//                           SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
//     SND_SOC_DAPM_SUPPLY_S("AUD_PAD_TOP", SUPPLY_SEQ_ADDA_AUD_PAD_TOP,
//                           AFE_AUD_PAD_TOP, RG_RX_FIFO_ON_SFT, 0,
//                           mtk_adda_pad_top_event, SND_SOC_DAPM_PRE_PMU),
//     SND_SOC_DAPM_SUPPLY_S("ADDA_MTKAIF_CFG", SUPPLY_SEQ_ADDA_MTKAIF_CFG,
//                           SND_SOC_NOPM, 0, 0,
//                           mtk_adda_mtkaif_cfg_event, SND_SOC_DAPM_PRE_PMU),
//     SND_SOC_DAPM_SUPPLY_S("AP_DMIC_EN", SUPPLY_SEQ_ADDA_AP_DMIC,
//                           AFE_ADDA_UL_SRC_CON0, UL_AP_DMIC_ON_SFT, 0, NULL, 0),
//     SND_SOC_DAPM_SUPPLY_S("ADDA_FIFO", SUPPLY_SEQ_ADDA_FIFO,
//                           AFE_ADDA_UL_DL_CON0, AFE_ADDA_FIFO_AUTO_RST_SFT, 1, NULL, 0),
//     SND_SOC_DAPM_MUX("ADDA_UL_Mux", SND_SOC_NOPM, 0, 0, &adda_ul_mux_control),
//     SND_SOC_DAPM_INPUT("AP_DMIC_INPUT"),
//     /* clock */
//     SND_SOC_DAPM_CLOCK_SUPPLY("top_mux_audio_h"),
//     SND_SOC_DAPM_CLOCK_SUPPLY("aud_dac_clk"),
//     SND_SOC_DAPM_CLOCK_SUPPLY("aud_dac_hires_clk"),
//     SND_SOC_DAPM_CLOCK_SUPPLY("aud_dac_predis_clk"),
//     SND_SOC_DAPM_CLOCK_SUPPLY("aud_adc_clk"),
//     SND_SOC_DAPM_CLOCK_SUPPLY("aud_adc_hires_clk"),
// };

const HIRES_THRESHOLD: c_int = 48000;

unsafe fn mtk_afe_dac_hires_connect(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = source;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let adda_priv: *mut mtk_afe_adda_priv;

    adda_priv = get_adda_priv_by_name(afe, (*w).name);

    if adda_priv.is_null() {
        dev_err((*afe).dev, c"%s(), adda_priv == NULL".as_ptr(), c"mtk_afe_dac_hires_connect".as_ptr());
        return 0;
    }

    if (*adda_priv).dl_rate > HIRES_THRESHOLD { 1 } else { 0 }
}

unsafe fn mtk_afe_adc_hires_connect(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = source;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let adda_priv: *mut mtk_afe_adda_priv;

    adda_priv = get_adda_priv_by_name(afe, (*w).name);

    if adda_priv.is_null() {
        dev_err((*afe).dev, c"%s(), adda_priv == NULL".as_ptr(), c"mtk_afe_adc_hires_connect".as_ptr());
        return 0;
    }

    if (*adda_priv).ul_rate > HIRES_THRESHOLD { 1 } else { 0 }
}

// static const struct snd_soc_dapm_route mtk_dai_adda_routes[] = {
//     /* playback */
//     {"ADDA_DL_CH1", "DL1_CH1 Switch", "DL1"},
//     {"ADDA_DL_CH2", "DL1_CH1 Switch", "DL1"},
//     {"ADDA_DL_CH2", "DL1_CH2 Switch", "DL1"},
//     {"ADDA_DL_CH1", "DL12_CH1 Switch", "DL12"},
//     {"ADDA_DL_CH2", "DL12_CH2 Switch", "DL12"},
//     {"ADDA_DL_CH1", "DL6_CH1 Switch", "DL6"},
//     {"ADDA_DL_CH2", "DL6_CH2 Switch", "DL6"},
//     {"ADDA_DL_CH1", "DL8_CH1 Switch", "DL8"},
//     {"ADDA_DL_CH2", "DL8_CH2 Switch", "DL8"},
//     {"ADDA_DL_CH1", "DL2_CH1 Switch", "DL2"},
//     {"ADDA_DL_CH2", "DL2_CH1 Switch", "DL2"},
//     {"ADDA_DL_CH2", "DL2_CH2 Switch", "DL2"},
//     {"ADDA_DL_CH1", "DL3_CH1 Switch", "DL3"},
//     {"ADDA_DL_CH2", "DL3_CH1 Switch", "DL3"},
//     {"ADDA_DL_CH2", "DL3_CH2 Switch", "DL3"},
//     {"ADDA_DL_CH1", "DL4_CH1 Switch", "DL4"},
//     {"ADDA_DL_CH2", "DL4_CH2 Switch", "DL4"},
//     {"ADDA_DL_CH1", "DL5_CH1 Switch", "DL5"},
//     {"ADDA_DL_CH2", "DL5_CH2 Switch", "DL5"},
//     {"ADDA Playback", NULL, "ADDA_DL_CH1"},
//     {"ADDA Playback", NULL, "ADDA_DL_CH2"},
//     {"ADDA Playback", NULL, "ADDA Enable"},
//     {"ADDA Playback", NULL, "ADDA Playback Enable"},
//     /* capture */
//     {"ADDA_UL_Mux", "MTKAIF", "ADDA Capture"},
//     {"ADDA_UL_Mux", "AP_DMIC", "AP DMIC Capture"},
//     {"ADDA Capture", NULL, "ADDA Enable"},
//     {"ADDA Capture", NULL, "ADDA Capture Enable"},
//     {"ADDA Capture", NULL, "AUD_PAD_TOP"},
//     {"ADDA Capture", NULL, "ADDA_MTKAIF_CFG"},
//     {"AP DMIC Capture", NULL, "ADDA Enable"},
//     {"AP DMIC Capture", NULL, "ADDA Capture Enable"},
//     {"AP DMIC Capture", NULL, "ADDA_FIFO"},
//     {"AP DMIC Capture", NULL, "AP_DMIC_EN"},
//     {"AP DMIC Capture", NULL, "AP_DMIC_INPUT"},
//     /* clk */
//     {"ADDA Playback", NULL, "aud_dac_clk"},
//     {"ADDA Playback", NULL, "aud_dac_predis_clk"},
//     {"ADDA Playback", NULL, "aud_dac_hires_clk", mtk_afe_dac_hires_connect},
//     {"ADDA Capture Enable", NULL, "aud_adc_clk"},
//     {"ADDA Capture Enable", NULL, "aud_adc_hires_clk", mtk_afe_adc_hires_connect},
//     /* hires source from apll1 */
//     {"top_mux_audio_h", NULL, APLL2_W_NAME},
//     {"aud_dac_hires_clk", NULL, "top_mux_audio_h"},
//     {"aud_adc_hires_clk", NULL, "top_mux_audio_h"},
// };

/* dai ops */
unsafe fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let rate: c_uint = params_rate(params);
    let id: c_int = (*dai).id;
    let adda_priv = (*afe_priv).dai_priv[id as usize] as *mut mtk_afe_adda_priv;

    dev_dbg(
        (*afe).dev,
        c"%s(), id %d, stream %d, rate %d\n".as_ptr(),
        c"mtk_dai_adda_hw_params".as_ptr(),
        id,
        (*substream).stream,
        rate,
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        let mut dl_src2_con0: c_uint;
        let dl_src2_con1: c_uint;

        (*adda_priv).dl_rate = rate as c_int;

        /* set sampling rate */
        dl_src2_con0 = mtk_adda_dl_rate_transform(afe, rate) << DL_2_INPUT_MODE_CTL_SFT;

        /* set output mode, UP_SAMPLING_RATE_X8 */
        dl_src2_con0 |= 0x3 << DL_2_OUTPUT_SEL_CTL_SFT;

        /* turn off mute function */
        dl_src2_con0 |= BIT(DL_2_MUTE_CH2_OFF_CTL_PRE_SFT);
        dl_src2_con0 |= BIT(DL_2_MUTE_CH1_OFF_CTL_PRE_SFT);

        /* set voice input data if input sample rate is 8k or 16k */
        if rate == 8000 || rate == 16000 {
            dl_src2_con0 |= BIT(DL_2_VOICE_MODE_CTL_PRE_SFT);
        }

        /* SA suggest apply -0.3db to audio/speech path */
        dl_src2_con1 = MTK_AFE_ADDA_DL_GAIN_NORMAL << DL_2_GAIN_CTL_PRE_SFT;

        /* turn on down-link gain */
        dl_src2_con0 |= BIT(DL_2_GAIN_ON_CTL_PRE_SFT);

        if id == MT8186_DAI_ADDA {
            /* clean predistortion */
            regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON0, 0);
            regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON1, 0);

            regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, dl_src2_con0);
            regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON1, dl_src2_con1);

            /* set sdm gain */
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_DL_SDM_DCCOMP_CON,
                ATTGAIN_CTL_MASK_SFT,
                (AUDIO_SDM_LEVEL_NORMAL as c_uint) << ATTGAIN_CTL_SFT,
            );

            /* Use new 2nd sdm */
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_DL_SDM_DITHER_CON,
                AFE_DL_SDM_DITHER_64TAP_EN_MASK_SFT,
                BIT(AFE_DL_SDM_DITHER_64TAP_EN_SFT),
            );
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_DL_SDM_AUTO_RESET_CON,
                AFE_DL_USE_NEW_2ND_SDM_MASK_SFT,
                BIT(AFE_DL_USE_NEW_2ND_SDM_SFT),
            );
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_DL_SDM_DCCOMP_CON,
                USE_3RD_SDM_MASK_SFT,
                (AUDIO_SDM_2ND as c_uint) << USE_3RD_SDM_SFT,
            );

            /* sdm auto reset */
            regmap_write((*afe).regmap, AFE_ADDA_DL_SDM_AUTO_RESET_CON, SDM_AUTO_RESET_THRESHOLD);
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_DL_SDM_AUTO_RESET_CON,
                SDM_AUTO_RESET_TEST_ON_MASK_SFT,
                BIT(SDM_AUTO_RESET_TEST_ON_SFT),
            );
        }
    } else {
        let mut ul_src_con0: c_uint = 0;
        let voice_mode: c_uint = mtk_adda_ul_rate_transform(afe, rate);

        (*adda_priv).ul_rate = rate as c_int;
        ul_src_con0 |= (voice_mode << 17) & (0x7 << 17);

        /* enable iir */
        ul_src_con0 |= (1 << UL_IIR_ON_TMP_CTL_SFT) & UL_IIR_ON_TMP_CTL_MASK_SFT;
        ul_src_con0 |= ((UL_IIR_SW as c_uint) << UL_IIRMODE_CTL_SFT) & UL_IIRMODE_CTL_MASK_SFT;
        match id {
            MT8186_DAI_ADDA | MT8186_DAI_AP_DMIC => {
                /* 35Hz @ 48k */
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_02_01, 0);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_04_03, 0x3fb8);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_06_05, 0x3fb80000);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_08_07, 0x3fb80000);
                regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_10_09, 0xc048);

                regmap_write((*afe).regmap, AFE_ADDA_UL_SRC_CON0, ul_src_con0);

                /* Using Internal ADC */
                regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, BIT(0), 0);

                /* mtkaif_rxif_data_mode = 0, amic */
                regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, BIT(0), 0);
            }
            _ => {}
        }

        /* ap dmic */
        match id {
            MT8186_DAI_AP_DMIC => {
                mtk_adda_ul_src_dmic(afe, id);
            }
            _ => {}
        }
    }

    0
}

// static const struct snd_soc_dai_ops mtk_dai_adda_ops = {
//     .hw_params = mtk_dai_adda_hw_params,
// };

/* dai driver */
const MTK_ADDA_PLAYBACK_RATES: c_uint =
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;

const MTK_ADDA_CAPTURE_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;

const MTK_ADDA_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

extern "C" {
    static mut mtk_adda_controls: [snd_kcontrol_new; 2];
    static mut mtk_dai_adda_widgets: [snd_soc_dapm_widget; 15];
    static mut mtk_dai_adda_routes: [snd_soc_dapm_route; 42];
    static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 2];
}

// static struct snd_soc_dai_driver mtk_dai_adda_driver[] = {
//     {
//         .name = "ADDA",
//         .id = MT8186_DAI_ADDA,
//         .playback = {
//             .stream_name = "ADDA Playback",
//             .channels_min = 1,
//             .channels_max = 2,
//             .rates = MTK_ADDA_PLAYBACK_RATES,
//             .formats = MTK_ADDA_FORMATS,
//         },
//         .capture = {
//             .stream_name = "ADDA Capture",
//             .channels_min = 1,
//             .channels_max = 2,
//             .rates = MTK_ADDA_CAPTURE_RATES,
//             .formats = MTK_ADDA_FORMATS,
//         },
//         .ops = &mtk_dai_adda_ops,
//     },
//     {
//         .name = "AP_DMIC",
//         .id = MT8186_DAI_AP_DMIC,
//         .capture = {
//             .stream_name = "AP DMIC Capture",
//             .channels_min = 1,
//             .channels_max = 2,
//             .rates = MTK_ADDA_CAPTURE_RATES,
//             .formats = MTK_ADDA_FORMATS,
//         },
//         .ops = &mtk_dai_adda_ops,
//     },
// };

pub unsafe extern "C" fn mt8186_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;
    let afe_priv = (*afe).platform_priv as *mut mt8186_afe_private;
    let ret: c_int;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as c_int;

    (*dai).controls = mtk_adda_controls.as_mut_ptr();
    (*dai).num_controls = mtk_adda_controls.len() as c_int;
    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_mut_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as c_int;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_mut_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as c_int;

    /* set dai priv */
    ret = mt8186_dai_set_priv(
        afe,
        MT8186_DAI_ADDA,
        size_of::<mtk_afe_adda_priv>(),
        ptr::null_mut(),
    );
    if ret != 0 {
        return ret;
    }

    /* ap dmic priv share with adda */
    (*afe_priv).dai_priv[MT8186_DAI_AP_DMIC as usize] =
        (*afe_priv).dai_priv[MT8186_DAI_ADDA as usize];

    0
}

extern "C" {
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn regmap_update_bits(regmap: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(regmap: *mut c_void, reg: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, s: *const c_char) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mt8186_afe_gpio_request(dev: *mut c_void, enable: bool, dai: c_int, uplink: c_int) -> c_int;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn mt8186_dai_set_priv(
        afe: *mut mtk_base_afe,
        id: c_int,
        size: usize,
        data: *mut c_void,
    ) -> c_int;
}

extern "Rust" {
    fn BIT(nr: c_uint) -> c_uint;
}

#[repr(C)]
struct mtk_base_afe {
    platform_priv: *mut c_void,
    regmap: *mut c_void,
    dev: *mut c_void,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8186_afe_private {
    dai_priv: [*mut c_void; 0],
    mtkaif_dmic: c_int,
    mtkaif_protocol: c_int,
    mtkaif_chosen_phase: [c_int; 2],
    mtkaif_phase_cycle: [c_int; 2],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_int,
    controls: *mut snd_kcontrol_new,
    num_controls: c_int,
    dapm_widgets: *mut snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *mut snd_soc_dapm_route,
    num_dapm_routes: c_int,
}

#[repr(C)]
struct snd_kcontrol_new;
#[repr(C)]
struct snd_soc_component;
#[repr(C)]
struct snd_soc_dapm_route;
#[repr(C)]
struct snd_soc_dai_driver;
#[repr(C)]
struct snd_pcm_hw_params;

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut c_void,
}

#[repr(C)]
struct snd_kcontrol {
    id: snd_ctl_elem_id,
}

#[repr(C)]
struct snd_ctl_elem_id {
    name: [c_char; 0],
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [i64; 1],
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
}

extern "C" {
    static MT8186_DAI_ADDA: c_int;
    static MT8186_DAI_AP_DMIC: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static MTKAIF_PROTOCOL_2_CLK_P2: c_int;
    static MTKAIF_PROTOCOL_2: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static DELAY_DATA_MISO1: c_int;
    static DELAY_DATA_MISO2: c_int;
    static AFE_ADDA_UL_SRC_CON0: c_uint;
    static DIGMIC_3P25M_1P625M_SEL_MASK_SFT: c_uint;
    static DMIC_LOW_POWER_CTL_MASK_SFT: c_uint;
    static UL_SDM_3_LEVEL_MASK_SFT: c_uint;
    static UL_SDM_3_LEVEL_SFT: c_uint;
    static UL_MODE_3P25M_CH1_CTL_MASK_SFT: c_uint;
    static UL_MODE_3P25M_CH1_CTL_SFT: c_uint;
    static UL_MODE_3P25M_CH2_CTL_MASK_SFT: c_uint;
    static UL_MODE_3P25M_CH2_CTL_SFT: c_uint;
    static AFE_ADDA_MTKAIF_RX_CFG0: c_uint;
    static MTKAIF_RXIF_VOICE_MODE_MASK_SFT: c_uint;
    static AFE_AUD_PAD_TOP: c_uint;
    static AFE_ADDA_MTKAIF_CFG0: c_uint;
    static MTKAIF_RXIF_CLKINV_ADC_MASK_SFT: c_uint;
    static MTKAIF_RXIF_CLKINV_ADC_SFT: c_uint;
    static AFE_ADDA_MTKAIF_RX_CFG2: c_uint;
    static MTKAIF_RXIF_DELAY_DATA_MASK_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_DATA_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_CYCLE_SFT: c_uint;
    static AFE_ADDA_DL_SRC2_CON0: c_uint;
    static DL_2_INPUT_MODE_CTL_SFT: c_uint;
    static DL_2_OUTPUT_SEL_CTL_SFT: c_uint;
    static DL_2_MUTE_CH2_OFF_CTL_PRE_SFT: c_uint;
    static DL_2_MUTE_CH1_OFF_CTL_PRE_SFT: c_uint;
    static DL_2_VOICE_MODE_CTL_PRE_SFT: c_uint;
    static MTK_AFE_ADDA_DL_GAIN_NORMAL: c_uint;
    static DL_2_GAIN_CTL_PRE_SFT: c_uint;
    static DL_2_GAIN_ON_CTL_PRE_SFT: c_uint;
    static AFE_ADDA_PREDIS_CON0: c_uint;
    static AFE_ADDA_PREDIS_CON1: c_uint;
    static AFE_ADDA_DL_SRC2_CON1: c_uint;
    static AFE_ADDA_DL_SDM_DCCOMP_CON: c_uint;
    static ATTGAIN_CTL_MASK_SFT: c_uint;
    static ATTGAIN_CTL_SFT: c_uint;
    static AFE_ADDA_DL_SDM_DITHER_CON: c_uint;
    static AFE_DL_SDM_DITHER_64TAP_EN_MASK_SFT: c_uint;
    static AFE_DL_SDM_DITHER_64TAP_EN_SFT: c_uint;
    static AFE_ADDA_DL_SDM_AUTO_RESET_CON: c_uint;
    static AFE_DL_USE_NEW_2ND_SDM_MASK_SFT: c_uint;
    static AFE_DL_USE_NEW_2ND_SDM_SFT: c_uint;
    static USE_3RD_SDM_MASK_SFT: c_uint;
    static USE_3RD_SDM_SFT: c_uint;
    static SDM_AUTO_RESET_TEST_ON_MASK_SFT: c_uint;
    static SDM_AUTO_RESET_TEST_ON_SFT: c_uint;
    static UL_IIR_ON_TMP_CTL_SFT: c_uint;
    static UL_IIR_ON_TMP_CTL_MASK_SFT: c_uint;
    static UL_IIRMODE_CTL_SFT: c_uint;
    static UL_IIRMODE_CTL_MASK_SFT: c_uint;
    static AFE_ADDA_IIR_COEF_02_01: c_uint;
    static AFE_ADDA_IIR_COEF_04_03: c_uint;
    static AFE_ADDA_IIR_COEF_06_05: c_uint;
    static AFE_ADDA_IIR_COEF_08_07: c_uint;
    static AFE_ADDA_IIR_COEF_10_09: c_uint;
    static AFE_ADDA_TOP_CON0: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
