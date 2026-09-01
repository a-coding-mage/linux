// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI ADDA Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

// C dependencies translated as external repository dependencies:
// <linux/delay.h>, <linux/regmap.h>
// "mt8196-afe-clk.h", "mt8196-afe-common.h", "mt8196-interconnection.h"

#[repr(C)]
pub struct mtk_afe_adda_priv {
    pub dl_rate: core::ffi::c_int,
    pub ul_rate: core::ffi::c_int,
}

pub const UL_IIR_SW: u32 = 0;
pub const UL_IIR_5HZ: u32 = 1;
pub const UL_IIR_10HZ: u32 = 2;
pub const UL_IIR_25HZ: u32 = 3;
pub const UL_IIR_50HZ: u32 = 4;
pub const UL_IIR_75HZ: u32 = 5;

pub const MTK_AFE_ADDA_UL_RATE_8K: u32 = 0;
pub const MTK_AFE_ADDA_UL_RATE_16K: u32 = 1;
pub const MTK_AFE_ADDA_UL_RATE_32K: u32 = 2;
pub const MTK_AFE_ADDA_UL_RATE_48K: u32 = 3;
pub const MTK_AFE_ADDA_UL_RATE_96K: u32 = 4;
pub const MTK_AFE_ADDA_UL_RATE_192K: u32 = 5;
pub const MTK_AFE_ADDA_UL_RATE_48K_HD: u32 = 6;

pub const MTK_AFE_MTKAIF_RATE_8K: u32 = 0;
pub const MTK_AFE_MTKAIF_RATE_12K: u32 = 1;
pub const MTK_AFE_MTKAIF_RATE_16K: u32 = 2;
pub const MTK_AFE_MTKAIF_RATE_24K: u32 = 3;
pub const MTK_AFE_MTKAIF_RATE_32K: u32 = 4;
pub const MTK_AFE_MTKAIF_RATE_48K: u32 = 5;
pub const MTK_AFE_MTKAIF_RATE_64K: u32 = 6;
pub const MTK_AFE_MTKAIF_RATE_96K: u32 = 7;
pub const MTK_AFE_MTKAIF_RATE_128K: u32 = 8;
pub const MTK_AFE_MTKAIF_RATE_192K: u32 = 9;
pub const MTK_AFE_MTKAIF_RATE_256K: u32 = 10;
pub const MTK_AFE_MTKAIF_RATE_384K: u32 = 11;
pub const MTK_AFE_MTKAIF_RATE_11K: u32 = 0x10;
pub const MTK_AFE_MTKAIF_RATE_22K: u32 = 0x11;
pub const MTK_AFE_MTKAIF_RATE_44K: u32 = 0x12;
pub const MTK_AFE_MTKAIF_RATE_88K: u32 = 0x13;
pub const MTK_AFE_MTKAIF_RATE_176K: u32 = 0x14;
pub const MTK_AFE_MTKAIF_RATE_352K: u32 = 0x15;

unsafe fn adda_ul_rate_transform(
    afe: *mut mtk_base_afe,
    rate: core::ffi::c_uint,
) -> core::ffi::c_uint {
    match rate {
        8000 => MTK_AFE_ADDA_UL_RATE_8K,
        16000 => MTK_AFE_ADDA_UL_RATE_16K,
        32000 => MTK_AFE_ADDA_UL_RATE_32K,
        48000 => MTK_AFE_ADDA_UL_RATE_48K,
        96000 => MTK_AFE_ADDA_UL_RATE_96K,
        192000 => MTK_AFE_ADDA_UL_RATE_192K,
        _ => {
            dev_warn((*afe).dev, c"rate %d invalid, use 48kHz!!!\n".as_ptr(), rate);
            MTK_AFE_ADDA_UL_RATE_48K
        }
    }
}

unsafe fn mtkaif_rate_transform(
    afe: *mut mtk_base_afe,
    rate: core::ffi::c_uint,
) -> core::ffi::c_uint {
    match rate {
        8000 => MTK_AFE_MTKAIF_RATE_8K,
        11025 => MTK_AFE_MTKAIF_RATE_11K,
        12000 => MTK_AFE_MTKAIF_RATE_12K,
        16000 => MTK_AFE_MTKAIF_RATE_16K,
        22050 => MTK_AFE_MTKAIF_RATE_22K,
        24000 => MTK_AFE_MTKAIF_RATE_24K,
        32000 => MTK_AFE_MTKAIF_RATE_32K,
        44100 => MTK_AFE_MTKAIF_RATE_44K,
        48000 => MTK_AFE_MTKAIF_RATE_48K,
        96000 => MTK_AFE_MTKAIF_RATE_96K,
        192000 => MTK_AFE_MTKAIF_RATE_192K,
        _ => {
            dev_warn((*afe).dev, c"rate %d invalid, use 48kHz!!!\n".as_ptr(), rate);
            MTK_AFE_MTKAIF_RATE_48K
        }
    }
}

pub const SUPPLY_SEQ_ADDA_AFE_ON: i32 = 0;
pub const SUPPLY_SEQ_ADDA_FIFO: i32 = 1;
pub const SUPPLY_SEQ_ADDA_AP_DMIC: i32 = 2;
pub const SUPPLY_SEQ_ADDA_UL_ON: i32 = 3;

unsafe fn mtk_adda_ul_src_set_dmic_phase_sync(afe: *mut mtk_base_afe) -> core::ffi::c_int {
    dev_dbg((*afe).dev, c"set dmic phase sync\n".as_ptr());
    // ul0~1
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        UL0_PHASE_SYNC_HCLK_SET_MASK_SFT,
        0x1 << UL0_PHASE_SYNC_HCLK_SET_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        UL0_PHASE_SYNC_FCLK_SET_MASK_SFT,
        0x1 << UL0_PHASE_SYNC_FCLK_SET_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        UL1_PHASE_SYNC_HCLK_SET_MASK_SFT,
        0x1 << UL1_PHASE_SYNC_HCLK_SET_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        UL1_PHASE_SYNC_FCLK_SET_MASK_SFT,
        0x1 << UL1_PHASE_SYNC_FCLK_SET_SFT,
    );
    // dmic 0
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        DMIC0_PHASE_SYNC_FCLK_SET_MASK_SFT,
        0x1 << DMIC0_PHASE_SYNC_FCLK_SET_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        DMIC0_PHASE_SYNC_HCLK_SET_MASK_SFT,
        0x1 << DMIC0_PHASE_SYNC_HCLK_SET_SFT,
    );
    // dmic 1
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        DMIC1_PHASE_SYNC_FCLK_SET_MASK_SFT,
        0x1 << DMIC1_PHASE_SYNC_FCLK_SET_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON1,
        DMIC1_PHASE_SYNC_HCLK_SET_MASK_SFT,
        0x1 << DMIC1_PHASE_SYNC_HCLK_SET_SFT,
    );
    // ul0~1 phase sync clock
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        DMIC1_PHASE_HCLK_SEL_MASK_SFT,
        0x1 << DMIC1_PHASE_HCLK_SEL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        DMIC1_PHASE_FCLK_SEL_MASK_SFT,
        0x1 << DMIC1_PHASE_FCLK_SEL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        DMIC0_PHASE_HCLK_SEL_MASK_SFT,
        0x1 << DMIC0_PHASE_HCLK_SEL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        DMIC0_PHASE_FCLK_SEL_MASK_SFT,
        0x1 << DMIC0_PHASE_FCLK_SEL_SFT,
    );
    // dmic 0
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL1_PHASE_HCLK_SEL_MASK_SFT,
        0x2 << UL1_PHASE_HCLK_SEL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL1_PHASE_FCLK_SEL_MASK_SFT,
        0x2 << UL1_PHASE_FCLK_SEL_SFT,
    );
    // dmic 1
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL0_PHASE_HCLK_SEL_MASK_SFT,
        0x2 << UL0_PHASE_HCLK_SEL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL0_PHASE_FCLK_SEL_MASK_SFT,
        0x2 << UL0_PHASE_FCLK_SEL_SFT,
    );

    0
}

unsafe fn mtk_adda_ul_src_set_dmic_phase_sync_clock(
    afe: *mut mtk_base_afe,
) -> core::ffi::c_int {
    dev_dbg((*afe).dev, c"dmic turn on phase sync clk\n".as_ptr());
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL_PHASE_SYNC_HCLK_1_ON_MASK_SFT,
        0x1 << UL_PHASE_SYNC_HCLK_1_ON_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL_PHASE_SYNC_HCLK_0_ON_MASK_SFT,
        0x1 << UL_PHASE_SYNC_HCLK_0_ON_SFT,
    );

    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL_PHASE_SYNC_FCLK_1_ON_MASK_SFT,
        0x1 << UL_PHASE_SYNC_FCLK_1_ON_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_ULSRC_PHASE_CON0,
        UL_PHASE_SYNC_FCLK_0_ON_MASK_SFT,
        0x1 << UL_PHASE_SYNC_FCLK_0_ON_SFT,
    );

    0
}

unsafe fn mtk_adda_ul_src_enable_dmic(
    afe: *mut mtk_base_afe,
    id: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut reg_con0: core::ffi::c_uint = 0;
    let mut reg_con1: core::ffi::c_uint = 0;

    dev_dbg((*afe).dev, c"id: %d\n".as_ptr(), id);

    match id {
        MT8196_DAI_ADDA | MT8196_DAI_AP_DMIC => {
            reg_con0 = AFE_ADDA_UL0_SRC_CON0;
            reg_con1 = AFE_ADDA_UL0_SRC_CON1;
        }
        MT8196_DAI_ADDA_CH34 | MT8196_DAI_AP_DMIC_CH34 => {
            reg_con0 = AFE_ADDA_UL1_SRC_CON0;
            reg_con1 = AFE_ADDA_UL1_SRC_CON1;
        }
        _ => return -EINVAL,
    }

    match id {
        MT8196_DAI_AP_DMIC => {
            dev_dbg((*afe).dev, c"clear mtkaifv4 ul ch1ch2 mux\n".as_ptr());
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_MTKAIFV4_RX_CFG0,
                MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK_SFT,
                0x0 << MTKAIFV4_UL_CH1CH2_IN_EN_SEL_SFT,
            );
        }
        MT8196_DAI_AP_DMIC_CH34 => {
            dev_dbg((*afe).dev, c"clear mtkaifv4 ul ch3ch4 mux\n".as_ptr());
            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_MTKAIFV4_RX_CFG0,
                MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK_SFT,
                0x0 << MTKAIFV4_UL_CH3CH4_IN_EN_SEL_SFT,
            );
        }
        _ => return -EINVAL,
    }

    /* choose Phase */
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        UL_DMIC_PHASE_SEL_CH1_MASK_SFT,
        0x0 << UL_DMIC_PHASE_SEL_CH1_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        UL_DMIC_PHASE_SEL_CH2_MASK_SFT,
        0x4 << UL_DMIC_PHASE_SEL_CH2_SFT,
    );

    /* dmic mode, 3.25M*/
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT,
        0x0,
    );
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        DMIC_LOW_POWER_MODE_CTL_MASK_SFT,
        0x0,
    );

    /* turn on dmic, ch1, ch2 */
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        UL_SDM_3_LEVEL_CTL_MASK_SFT,
        0x1 << UL_SDM_3_LEVEL_CTL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        UL_MODE_3P25M_CH1_CTL_MASK_SFT,
        0x1 << UL_MODE_3P25M_CH1_CTL_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        reg_con0,
        UL_MODE_3P25M_CH2_CTL_MASK_SFT,
        0x1 << UL_MODE_3P25M_CH2_CTL_SFT,
    );

    /* ul gain:  gain = 0x7fff/positive_gain = 0x0/gain_mode = 0x10 */
    regmap_update_bits(
        (*afe).regmap,
        reg_con1,
        ADDA_UL_GAIN_VALUE_MASK_SFT,
        0x7fff << ADDA_UL_GAIN_VALUE_SFT,
    );
    regmap_update_bits(
        (*afe).regmap,
        reg_con1,
        ADDA_UL_POSTIVEGAIN_MASK_SFT,
        0x0 << ADDA_UL_POSTIVEGAIN_SFT,
    );
    /* gain_mode = 0x10: Add 0.5 gain at CIC output */
    regmap_update_bits(
        (*afe).regmap,
        reg_con1,
        GAIN_MODE_MASK_SFT,
        0x02 << GAIN_MODE_SFT,
    );
    0
}

unsafe fn mtk_adda_sleep_on_pmd_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: core::ffi::c_int,
) -> core::ffi::c_int {
    let cmpnt: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;

    dev_dbg(
        (*afe).dev,
        c"name %s, event 0x%x\n".as_ptr(),
        (*w).name,
        event,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {}
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(120, 130);
        }
        _ => {}
    }

    0
}

/* ADDA UL MUX */
pub const ADDA_UL_MUX_MASK: u32 = 0x3;
pub const ADDA_UL_MUX_MTKAIF: i32 = 0;
pub const ADDA_UL_MUX_AP_DMIC: i32 = 1;
pub const ADDA_UL_MUX_AP_DMIC_MULTICH: i32 = 2;

static adda_ul_mux_map: [*const core::ffi::c_char; 3] = [
    c"MTKAIF".as_ptr(),
    c"AP_DMIC".as_ptr(),
    c"AP_DMIC_MULTI_CH".as_ptr(),
];

static mut adda_ul_map_value: [core::ffi::c_int; 3] = [
    ADDA_UL_MUX_MTKAIF,
    ADDA_UL_MUX_AP_DMIC,
    ADDA_UL_MUX_AP_DMIC_MULTICH,
];

static adda_ul_mux_map_enum: soc_enum = SOC_VALUE_ENUM_SINGLE_DECL!(
    SND_SOC_NOPM,
    0,
    ADDA_UL_MUX_MASK,
    adda_ul_mux_map,
    adda_ul_map_value
);

static adda_ul_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"ADDA_UL_MUX Select".as_ptr(), adda_ul_mux_map_enum);

static adda_ch34_ul_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!(c"ADDA_CH34_UL_MUX Select".as_ptr(), adda_ul_mux_map_enum);

static mtk_dai_adda_widgets: [snd_soc_dapm_widget; 10] = [
    /* inter-connections */
    SND_SOC_DAPM_SUPPLY_S!(
        c"ADDA Enable".as_ptr(),
        SUPPLY_SEQ_ADDA_AFE_ON,
        AUDIO_ENGEN_CON0,
        AUDIO_F3P25M_EN_ON_SFT,
        0,
        core::ptr::null_mut(),
        0
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"ADDA Capture Enable".as_ptr(),
        SUPPLY_SEQ_ADDA_UL_ON,
        AFE_ADDA_UL0_SRC_CON0,
        UL_SRC_ON_TMP_CTL_SFT,
        0,
        mtk_adda_sleep_on_pmd_event,
        SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"ADDA CH34 Capture Enable".as_ptr(),
        SUPPLY_SEQ_ADDA_UL_ON,
        AFE_ADDA_UL1_SRC_CON0,
        UL_SRC_ON_TMP_CTL_SFT,
        0,
        mtk_adda_sleep_on_pmd_event,
        SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"AP_DMIC_EN".as_ptr(),
        SUPPLY_SEQ_ADDA_AP_DMIC,
        AFE_ADDA_UL0_SRC_CON0,
        UL_AP_DMIC_ON_SFT,
        0,
        mtk_adda_sleep_on_pmd_event,
        SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"AP_DMIC_CH34_EN".as_ptr(),
        SUPPLY_SEQ_ADDA_AP_DMIC,
        AFE_ADDA_UL1_SRC_CON0,
        UL_AP_DMIC_ON_SFT,
        0,
        mtk_adda_sleep_on_pmd_event,
        SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"ADDA_FIFO".as_ptr(),
        SUPPLY_SEQ_ADDA_FIFO,
        AFE_ADDA_UL0_SRC_CON1,
        FIFO_SOFT_RST_SFT,
        1,
        core::ptr::null_mut(),
        0
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        c"ADDA_CH34_FIFO".as_ptr(),
        SUPPLY_SEQ_ADDA_FIFO,
        AFE_ADDA_UL1_SRC_CON1,
        FIFO_SOFT_RST_SFT,
        1,
        core::ptr::null_mut(),
        0
    ),
    SND_SOC_DAPM_MUX!(
        c"ADDA_UL_Mux".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        &adda_ul_mux_control
    ),
    SND_SOC_DAPM_MUX!(
        c"ADDA_CH34_UL_Mux".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        &adda_ch34_ul_mux_control
    ),
    SND_SOC_DAPM_INPUT!(c"AP_DMIC_INPUT".as_ptr()),
];

static mtk_dai_adda_routes: [snd_soc_dapm_route; 27] = [
    /* capture */
    snd_soc_dapm_route { sink: c"ADDA_UL_Mux".as_ptr(), control: c"MTKAIF".as_ptr(), source: c"ADDA Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"ADDA_UL_Mux".as_ptr(), control: c"AP_DMIC".as_ptr(), source: c"AP DMIC Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"ADDA_UL_Mux".as_ptr(), control: c"AP_DMIC_MULTI_CH".as_ptr(), source: c"AP DMIC MULTICH Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"ADDA_CH34_UL_Mux".as_ptr(), control: c"MTKAIF".as_ptr(), source: c"ADDA CH34 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"ADDA_CH34_UL_Mux".as_ptr(), control: c"AP_DMIC".as_ptr(), source: c"AP DMIC CH34 Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"ADDA_CH34_UL_Mux".as_ptr(), control: c"AP_DMIC_MULTI_CH".as_ptr(), source: c"AP DMIC MULTICH Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA Capture Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA_FIFO".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC CH34 Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC CH34 Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA CH34 Capture Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC CH34 Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA_CH34_FIFO".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC CH34 Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_CH34_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA Capture Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA CH34 Capture Enable".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA_FIFO".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"ADDA_CH34_FIFO".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_CH34_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_INPUT".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC CH34 Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_INPUT".as_ptr() },
    snd_soc_dapm_route { sink: c"AP DMIC MULTICH Capture".as_ptr(), control: core::ptr::null(), source: c"AP_DMIC_INPUT".as_ptr() },
];

/* dai ops */
unsafe fn set_playback_hw_params(
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
    let rate: core::ffi::c_uint = params_rate(params);
    let adda_priv: *mut mtk_afe_adda_priv;
    let mut mtkaif_rate: core::ffi::c_uint = 0;
    let id: core::ffi::c_int = (*dai).id;

    adda_priv = (*afe_priv).dai_priv[id as usize] as *mut mtk_afe_adda_priv;
    if adda_priv.is_null() {
        return -EINVAL;
    }

    (*adda_priv).dl_rate = rate as core::ffi::c_int;

    /* get mtkaif dl rate */
    mtkaif_rate = mtkaif_rate_transform(afe, (*adda_priv).dl_rate as core::ffi::c_uint);

    if id == MT8196_DAI_ADDA {
        /* MTKAIF sample rate config */
        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_MTKAIFV4_TX_CFG0,
            MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT,
            mtkaif_rate << MTKAIFV4_TXIF_INPUT_MODE_SFT,
        );
        /* AFE_ADDA_MTKAIFV4_TX_CFG0 */
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_TX_CFG0, MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT, 0x0 << MTKAIFV4_TXIF_FOUR_CHANNEL_SFT);
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_TX_CFG0, MTKAIFV4_ADDA_OUT_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_ADDA_OUT_EN_SEL_SFT);
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_TX_CFG0, MTKAIFV4_ADDA6_OUT_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_ADDA6_OUT_EN_SEL_SFT);
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_TX_CFG0, MTKAIFV4_TXIF_V4_MASK_SFT, 0x1 << MTKAIFV4_TXIF_V4_SFT);
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_TX_CFG0, MTKAIFV4_TXIF_EN_SEL_MASK_SFT, 0x0 << MTKAIFV4_TXIF_EN_SEL_SFT);
        /* clean predistortion */
    } else {
        /* MTKAIF sample rate config */
        regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_TX_CFG0, ADDA6_MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT, mtkaif_rate << ADDA6_MTKAIFV4_TXIF_INPUT_MODE_SFT);
        /* AFE_ADDA6_MTKAIFV4_TX_CFG0 */
        regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_TX_CFG0, ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT, 0x0 << ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_SFT);
        regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_TX_CFG0, ADDA6_MTKAIFV4_TXIF_EN_SEL_MASK_SFT, 0x1 << ADDA6_MTKAIFV4_TXIF_EN_SEL_SFT);
    }

    0
}

unsafe fn set_capture_hw_params(
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
    let rate: core::ffi::c_uint = params_rate(params);
    let adda_priv: *mut mtk_afe_adda_priv;
    let mut voice_mode: core::ffi::c_uint = 0;
    let mut ul_src_con0: core::ffi::c_uint = 0;
    let mut mtkaif_rate: core::ffi::c_uint = 0;
    let id: core::ffi::c_int = (*dai).id;

    adda_priv = (*afe_priv).dai_priv[id as usize] as *mut mtk_afe_adda_priv;
    if adda_priv.is_null() {
        return -EINVAL;
    }

    (*adda_priv).ul_rate = rate as core::ffi::c_int;

    /* get mtkaif dl rate */
    mtkaif_rate = mtkaif_rate_transform(afe, (*adda_priv).ul_rate as core::ffi::c_uint);

    voice_mode = adda_ul_rate_transform(afe, rate);

    ul_src_con0 |= (voice_mode << 17) & (0x7 << 17);

    /* enable iir */
    ul_src_con0 |= (1 << UL_IIR_ON_TMP_CTL_SFT) & UL_IIR_ON_TMP_CTL_MASK_SFT;
    ul_src_con0 |= (UL_IIR_SW << UL_IIRMODE_CTL_SFT) & UL_IIRMODE_CTL_MASK_SFT;

    regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT, mtkaif_rate << MTKAIFV4_RXIF_INPUT_MODE_SFT);
    regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_RX_CFG0, ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT, mtkaif_rate << ADDA6_MTKAIFV4_RXIF_INPUT_MODE_SFT);

    match id {
        MT8196_DAI_ADDA | MT8196_DAI_AP_DMIC | MT8196_DAI_AP_DMIC_MULTICH => {
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT, mtkaif_rate << MTKAIFV4_RXIF_INPUT_MODE_SFT);
            /* AFE_ADDA_MTKAIFV4_RX_CFG0 */
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT, 0x1 << MTKAIFV4_RXIF_FOUR_CHANNEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_EN_SEL_MASK_SFT, 0x0 << MTKAIFV4_RXIF_EN_SEL_SFT);
            /* [28] loopback mode
             * 0: loopback adda tx to adda rx
             * 1: loopback adda6 tx to adda rx
             */
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_TXIF_EN_SEL_MASK_SFT, 0x0 << MTKAIFV4_TXIF_EN_SEL_SFT);

            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_UL_CH1CH2_IN_EN_SEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_UL_CH3CH4_IN_EN_SEL_SFT);

            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_10_09, 0x0000C048);

            regmap_write((*afe).regmap, AFE_ADDA_UL1_SRC_CON0, ul_src_con0);

            /* mtkaif_rxif_data_mode = 0, amic */
            regmap_update_bits((*afe).regmap, AFE_MTKAIF1_RX_CFG0, 0x1 << 0, 0x0 << 0);

            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL0_IIR_COEF_10_09, 0x0000C048);

            regmap_write((*afe).regmap, AFE_ADDA_UL0_SRC_CON0, ul_src_con0);

            /* mtkaif_rxif_data_mode = 0, amic */
            regmap_update_bits((*afe).regmap, AFE_MTKAIF0_RX_CFG0, 0x1 << 0, 0x0 << 0);
        }
        MT8196_DAI_ADDA_CH34 | MT8196_DAI_AP_DMIC_CH34 => {
            /* AFE_ADDA_MTKAIFV4_RX_CFG0 */
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT, 0x1 << MTKAIFV4_RXIF_FOUR_CHANNEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_RXIF_EN_SEL_MASK_SFT, 0x0 << MTKAIFV4_RXIF_EN_SEL_SFT);

            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_UL_CH1CH2_IN_EN_SEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_UL_CH3CH4_IN_EN_SEL_SFT);

            /* 35Hz @ 48k */
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_02_01, 0x00000000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_04_03, 0x00003FB8);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_06_05, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_08_07, 0x3FB80000);
            regmap_write((*afe).regmap, AFE_ADDA_UL1_IIR_COEF_10_09, 0x0000C048);

            regmap_write((*afe).regmap, AFE_ADDA_UL1_SRC_CON0, ul_src_con0);

            /* mtkaif_rxif_data_mode = 0, amic */
            regmap_update_bits((*afe).regmap, AFE_MTKAIF1_RX_CFG0, 0x1 << 0, 0x0 << 0);
        }
        MT8196_DAI_ADDA_CH56 => {
            regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_RX_CFG0, ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT, mtkaif_rate << ADDA6_MTKAIFV4_RXIF_INPUT_MODE_SFT);
            /* AFE_ADDA6_MTKAIFV4_RX_CFG0 */
            regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_RX_CFG0, ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT, 0x1 << ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIFV4_RX_CFG0, MTKAIFV4_UL_CH5CH6_IN_EN_SEL_MASK_SFT, 0x1 << MTKAIFV4_UL_CH5CH6_IN_EN_SEL_SFT);
            regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIFV4_RX_CFG0, ADDA6_MTKAIFV4_RXIF_EN_SEL_MASK_SFT, 0x1 << ADDA6_MTKAIFV4_RXIF_EN_SEL_SFT);
        }
        _ => {}
    }

    /* ap dmic */
    match id {
        MT8196_DAI_AP_DMIC | MT8196_DAI_AP_DMIC_CH34 => {
            mtk_adda_ul_src_enable_dmic(afe, id);
        }
        MT8196_DAI_AP_DMIC_MULTICH => {
            regmap_update_bits((*afe).regmap, AFE_ADDA_ULSRC_PHASE_CON1, DMIC_CLK_PHASE_SYNC_SET_MASK_SFT, 0x1 << DMIC_CLK_PHASE_SYNC_SET_SFT);
            mtk_adda_ul_src_set_dmic_phase_sync(afe);
            mtk_adda_ul_src_enable_dmic(afe, MT8196_DAI_AP_DMIC);
            mtk_adda_ul_src_enable_dmic(afe, MT8196_DAI_AP_DMIC_CH34);
            mtk_adda_ul_src_set_dmic_phase_sync_clock(afe);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let id: core::ffi::c_int = (*dai).id;

    if id >= MT8196_DAI_NUM || id < 0 {
        return -EINVAL;
    }

    dev_dbg(
        (*afe).dev,
        c"id %d, stream %d, rate %d\n".as_ptr(),
        id,
        (*substream).stream,
        params_rate(params),
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return set_playback_hw_params(params, dai);
    } else {
        return set_capture_hw_params(params, dai);
    }
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
};

/* dai driver */
pub const MTK_ADDA_PLAYBACK_RATES: u32 =
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;

pub const MTK_ADDA_CAPTURE_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;

pub const MTK_ADDA_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 6] = [
    snd_soc_dai_driver {
        name: c"ADDA".as_ptr(),
        id: MT8196_DAI_ADDA,
        playback: snd_soc_pcm_stream {
            stream_name: c"ADDA Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"ADDA Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: c"ADDA_CH34".as_ptr(),
        id: MT8196_DAI_ADDA_CH34,
        playback: snd_soc_pcm_stream {
            stream_name: c"ADDA CH34 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"ADDA CH34 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: c"ADDA_CH56".as_ptr(),
        id: MT8196_DAI_ADDA_CH56,
        capture: snd_soc_pcm_stream {
            stream_name: c"ADDA CH56 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c"AP_DMIC".as_ptr(),
        id: MT8196_DAI_AP_DMIC,
        capture: snd_soc_pcm_stream {
            stream_name: c"AP DMIC Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c"AP_DMIC_CH34".as_ptr(),
        id: MT8196_DAI_AP_DMIC_CH34,
        capture: snd_soc_pcm_stream {
            stream_name: c"AP DMIC CH34 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
    /*
     * Multich DMIC combines two DMIC controllers for use together,
     * so AP_DMIC and Multich DMIC cannot be used at the same time.
     */
    snd_soc_dai_driver {
        name: c"AP_DMIC_MULTICH".as_ptr(),
        id: MT8196_DAI_AP_DMIC_MULTICH,
        capture: snd_soc_pcm_stream {
            stream_name: c"AP DMIC MULTICH Capture".as_ptr(),
            channels_min: 1,
            channels_max: 4,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn init_adda_priv_data(afe: *mut mtk_base_afe) -> core::ffi::c_int {
    let afe_priv: *mut mt8196_afe_private = (*afe).platform_priv as *mut mt8196_afe_private;
    let mut adda_priv: *mut mtk_afe_adda_priv;
    static adda_dai_list: [core::ffi::c_int; 4] = [
        MT8196_DAI_ADDA,
        MT8196_DAI_ADDA_CH34,
        MT8196_DAI_ADDA_CH56,
        MT8196_DAI_AP_DMIC_MULTICH,
    ];
    let mut i: usize = 0;

    while i < adda_dai_list.len() {
        adda_priv = devm_kzalloc(
            (*afe).dev,
            core::mem::size_of::<mtk_afe_adda_priv>(),
            GFP_KERNEL,
        ) as *mut mtk_afe_adda_priv;
        if adda_priv.is_null() {
            return -ENOMEM;
        }

        (*afe_priv).dai_priv[adda_dai_list[i] as usize] = adda_priv as *mut core::ffi::c_void;
        i += 1;
    }

    /* ap dmic priv share with adda */
    (*afe_priv).dai_priv[MT8196_DAI_AP_DMIC as usize] =
        (*afe_priv).dai_priv[MT8196_DAI_ADDA as usize];
    (*afe_priv).dai_priv[MT8196_DAI_AP_DMIC_CH34 as usize] =
        (*afe_priv).dai_priv[MT8196_DAI_ADDA_CH34 as usize];

    0
}

pub unsafe extern "C" fn mt8196_dai_adda_register(afe: *mut mtk_base_afe) -> core::ffi::c_int {
    let dai: *mut mtk_base_afe_dai;
    let ret: core::ffi::c_int;

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as core::ffi::c_int;
    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as core::ffi::c_int;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as core::ffi::c_int;

    ret = init_adda_priv_data(afe);
    if ret != 0 {
        return ret;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
