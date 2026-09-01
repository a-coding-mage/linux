// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI TDM Control
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>

// Dependencies translated from:
// <linux/regmap.h>
// <sound/pcm_params.h>
// "mt8192-afe-clk.h"
// "mt8192-afe-common.h"
// "mt8192-afe-gpio.h"
// "mt8192-interconnection.h"

#[repr(C)]
pub struct mtk_afe_tdm_priv {
    id: i32,
    bck_id: i32,
    bck_rate: i32,
    tdm_out_mode: i32,
    bck_invert: i32,
    lck_invert: i32,
    mclk_id: i32,
    mclk_multiple: i32, /* according to sample rate */
    mclk_rate: i32,
    mclk_apll: i32,
}

const TDM_OUT_I2S: u32 = 0;
const TDM_OUT_DSP_A: u32 = 1;
const TDM_OUT_DSP_B: u32 = 2;

const TDM_BCK_NON_INV: i32 = 0;
const TDM_BCK_INV: i32 = 1;

const TDM_LCK_NON_INV: i32 = 0;
const TDM_LCK_INV: i32 = 1;

const TDM_WLEN_16_BIT: u32 = 1;
const TDM_WLEN_32_BIT: u32 = 2;

const TDM_CHANNEL_BCK_16: u32 = 0;
const TDM_CHANNEL_BCK_24: u32 = 1;
const TDM_CHANNEL_BCK_32: u32 = 2;

const TDM_CHANNEL_NUM_2: u32 = 0;
const TDM_CHANNEL_NUM_4: u32 = 1;
const TDM_CHANNEL_NUM_8: u32 = 2;

const TDM_CH_START_O30_O31: u32 = 0;
const TDM_CH_START_O32_O33: u32 = 1;
const TDM_CH_START_O34_O35: u32 = 2;
const TDM_CH_START_O36_O37: u32 = 3;
const TDM_CH_ZERO: u32 = 4;

unsafe fn get_tdm_wlen(format: snd_pcm_format_t) -> u32 {
    if snd_pcm_format_physical_width(format) <= 16 {
        TDM_WLEN_16_BIT
    } else {
        TDM_WLEN_32_BIT
    }
}

unsafe fn get_tdm_channel_bck(format: snd_pcm_format_t) -> u32 {
    if snd_pcm_format_physical_width(format) <= 16 {
        TDM_CHANNEL_BCK_16
    } else {
        TDM_CHANNEL_BCK_32
    }
}

unsafe fn get_tdm_lrck_width(format: snd_pcm_format_t) -> u32 {
    (snd_pcm_format_physical_width(format) - 1) as u32
}

fn get_tdm_ch(ch: u32) -> u32 {
    match ch {
        1 | 2 => TDM_CHANNEL_NUM_2,
        3 | 4 => TDM_CHANNEL_NUM_4,
        5 | 6 | 7 | 8 | _ => TDM_CHANNEL_NUM_8,
    }
}

fn get_tdm_ch_fixup(channels: u32) -> u32 {
    if channels > 4 {
        8
    } else if channels > 2 {
        4
    } else {
        2
    }
}

fn get_tdm_ch_per_sdata(mode: u32, channels: u32) -> u32 {
    if mode == TDM_OUT_DSP_A || mode == TDM_OUT_DSP_B {
        get_tdm_ch_fixup(channels)
    } else {
        2
    }
}

/* interconnection */
const HDMI_CONN_CH0: i32 = 0;
const HDMI_CONN_CH1: i32 = 1;
const HDMI_CONN_CH2: i32 = 2;
const HDMI_CONN_CH3: i32 = 3;
const HDMI_CONN_CH4: i32 = 4;
const HDMI_CONN_CH5: i32 = 5;
const HDMI_CONN_CH6: i32 = 6;
const HDMI_CONN_CH7: i32 = 7;

static hdmi_conn_mux_map: [*const c_char; 8] = [
    c_str!("CH0"),
    c_str!("CH1"),
    c_str!("CH2"),
    c_str!("CH3"),
    c_str!("CH4"),
    c_str!("CH5"),
    c_str!("CH6"),
    c_str!("CH7"),
];

static mut hdmi_conn_mux_map_value: [i32; 8] = [
    HDMI_CONN_CH0,
    HDMI_CONN_CH1,
    HDMI_CONN_CH2,
    HDMI_CONN_CH3,
    HDMI_CONN_CH4,
    HDMI_CONN_CH5,
    HDMI_CONN_CH6,
    HDMI_CONN_CH7,
];

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch0_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_0_SFT,
    HDMI_O_0_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch0_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH0_MUX", hdmi_ch0_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch1_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_1_SFT,
    HDMI_O_1_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch1_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH1_MUX", hdmi_ch1_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch2_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_2_SFT,
    HDMI_O_2_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch2_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH2_MUX", hdmi_ch2_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch3_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_3_SFT,
    HDMI_O_3_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch3_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH3_MUX", hdmi_ch3_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch4_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_4_SFT,
    HDMI_O_4_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch4_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH4_MUX", hdmi_ch4_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch5_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_5_SFT,
    HDMI_O_5_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch5_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH5_MUX", hdmi_ch5_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch6_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_6_SFT,
    HDMI_O_6_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch6_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH6_MUX", hdmi_ch6_mux_map_enum);

SOC_VALUE_ENUM_SINGLE_DECL!(
    hdmi_ch7_mux_map_enum,
    AFE_HDMI_CONN0,
    HDMI_O_7_SFT,
    HDMI_O_7_MASK,
    hdmi_conn_mux_map,
    hdmi_conn_mux_map_value
);
static hdmi_ch7_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("HDMI_CH7_MUX", hdmi_ch7_mux_map_enum);

const SUPPLY_SEQ_APLL: i32 = 0;
const SUPPLY_SEQ_TDM_MCK_EN: i32 = 1;
const SUPPLY_SEQ_TDM_BCK_EN: i32 = 2;
const SUPPLY_SEQ_TDM_EN: i32 = 3;

fn get_tdm_id_by_name(_name: *const c_char) -> i32 {
    MT8192_DAI_TDM
}

unsafe fn mtk_tdm_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn((*afe).dev, c_str!("%s(), tdm_priv == NULL"), c_str!("mtk_tdm_en_event"));
        return -EINVAL;
    }

    dev_dbg(
        (*cmpnt).dev,
        c_str!("%s(), name %s, event 0x%x\n"),
        c_str!("mtk_tdm_en_event"),
        (*w).name,
        event,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8192_afe_gpio_request((*afe).dev, true, (*tdm_priv).id, 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8192_afe_gpio_request((*afe).dev, false, (*tdm_priv).id, 0);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_tdm_bck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn((*afe).dev, c_str!("%s(), tdm_priv == NULL"), c_str!("mtk_tdm_bck_en_event"));
        return -EINVAL;
    }

    dev_dbg(
        (*cmpnt).dev,
        c_str!("%s(), name %s, event 0x%x, dai_id %d\n"),
        c_str!("mtk_tdm_bck_en_event"),
        (*w).name,
        event,
        dai_id,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8192_mck_enable(afe, (*tdm_priv).bck_id, (*tdm_priv).bck_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8192_mck_disable(afe, (*tdm_priv).bck_id);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_tdm_mck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn((*afe).dev, c_str!("%s(), tdm_priv == NULL"), c_str!("mtk_tdm_mck_en_event"));
        return -EINVAL;
    }

    dev_dbg(
        (*cmpnt).dev,
        c_str!("%s(), name %s, event 0x%x, dai_id %d\n"),
        c_str!("mtk_tdm_mck_en_event"),
        (*w).name,
        event,
        dai_id,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8192_mck_enable(afe, (*tdm_priv).mclk_id, (*tdm_priv).mclk_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*tdm_priv).mclk_rate = 0;
            mt8192_mck_disable(afe, (*tdm_priv).mclk_id);
        }
        _ => {}
    }

    0
}

static mtk_dai_tdm_widgets: [snd_soc_dapm_widget; 12] = [
    SND_SOC_DAPM_MUX!("HDMI_CH0_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch0_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH1_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch1_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH2_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch2_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH3_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch3_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH4_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch4_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH5_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch5_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH6_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch6_mux_control),
    SND_SOC_DAPM_MUX!("HDMI_CH7_MUX", SND_SOC_NOPM, 0, 0, &hdmi_ch7_mux_control),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_tdm_clk"),
    SND_SOC_DAPM_SUPPLY_S!(
        "TDM_EN",
        SUPPLY_SEQ_TDM_EN,
        AFE_TDM_CON1,
        TDM_EN_SFT,
        0,
        mtk_tdm_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "TDM_BCK",
        SUPPLY_SEQ_TDM_BCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        mtk_tdm_bck_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "TDM_MCK",
        SUPPLY_SEQ_TDM_MCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        mtk_tdm_mck_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
];

unsafe fn mtk_afe_tdm_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> i32 {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;
    let cur_apll: i32;

    /* which apll */
    cur_apll = mt8192_get_apll_by_name(afe, (*source).name);

    if (*tdm_priv).mclk_apll == cur_apll { 1 } else { 0 }
}

static mtk_dai_tdm_routes: [snd_soc_dapm_route; 78] = [
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH0_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH1_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH2_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH3_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH4_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH5_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH6_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH0", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH1", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH2", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH3", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH4", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH5", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH6", "HDMI"),
    SND_SOC_DAPM_ROUTE!("HDMI_CH7_MUX", "CH7", "HDMI"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH0_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH1_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH2_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH3_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH4_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH5_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH6_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "HDMI_CH7_MUX"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "aud_tdm_clk"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "TDM_BCK"),
    SND_SOC_DAPM_ROUTE!("TDM", null(), "TDM_EN"),
    SND_SOC_DAPM_ROUTE!("TDM_BCK", null(), "TDM_MCK"),
    SND_SOC_DAPM_ROUTE_FN!("TDM_MCK", null(), APLL1_W_NAME, mtk_afe_tdm_apll_connect),
    SND_SOC_DAPM_ROUTE_FN!("TDM_MCK", null(), APLL2_W_NAME, mtk_afe_tdm_apll_connect),
];

/* dai ops */
unsafe fn mtk_dai_tdm_cal_mclk(
    afe: *mut mtk_base_afe,
    tdm_priv: *mut mtk_afe_tdm_priv,
    freq: i32,
) -> i32 {
    let apll: i32;
    let apll_rate: i32;

    apll = mt8192_get_apll_by_rate(afe, freq);
    apll_rate = mt8192_get_apll_rate(afe, apll);

    if freq == 0 || freq > apll_rate {
        dev_warn((*afe).dev, c_str!("%s(), freq(%d Hz) invalid\n"), c_str!("mtk_dai_tdm_cal_mclk"), freq);
        return -EINVAL;
    }

    if apll_rate % freq != 0 {
        dev_warn((*afe).dev, c_str!("%s(), APLL cannot generate %d Hz"), c_str!("mtk_dai_tdm_cal_mclk"), freq);
        return -EINVAL;
    }

    (*tdm_priv).mclk_rate = freq;
    (*tdm_priv).mclk_apll = apll;

    0
}

unsafe fn mtk_dai_tdm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let tdm_id = (*dai).id;
    let tdm_priv = (*afe_priv).dai_priv[tdm_id as usize] as *mut mtk_afe_tdm_priv;
    let tdm_out_mode = (*tdm_priv).tdm_out_mode as u32;
    let rate = params_rate(params) as u32;
    let channels = params_channels(params) as u32;
    let out_channels_per_sdata = get_tdm_ch_per_sdata(tdm_out_mode, channels);
    let format = params_format(params);
    let mut tdm_con: u32 = 0;

    /* calculate mclk_rate, if not set explicitly */
    if (*tdm_priv).mclk_rate == 0 {
        (*tdm_priv).mclk_rate = (rate as i32).wrapping_mul((*tdm_priv).mclk_multiple);
        mtk_dai_tdm_cal_mclk(afe, tdm_priv, (*tdm_priv).mclk_rate);
    }

    /* calculate bck */
    (*tdm_priv).bck_rate = ((rate.wrapping_mul(out_channels_per_sdata))
        .wrapping_mul(snd_pcm_format_physical_width(format) as u32)) as i32;

    if (*tdm_priv).bck_rate > (*tdm_priv).mclk_rate {
        dev_warn((*afe).dev, c_str!("%s(), bck_rate > mclk_rate rate"), c_str!("mtk_dai_tdm_hw_params"));
    }

    if (*tdm_priv).mclk_rate % (*tdm_priv).bck_rate != 0 {
        dev_warn((*afe).dev, c_str!("%s(), bck cannot generate"), c_str!("mtk_dai_tdm_hw_params"));
    }

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), id %d, rate %d, channels %d, format %d, mclk_rate %d, bck_rate %d\n"),
        c_str!("mtk_dai_tdm_hw_params"),
        tdm_id,
        rate,
        channels,
        format,
        (*tdm_priv).mclk_rate,
        (*tdm_priv).bck_rate,
    );

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), out_channels_per_sdata = %d\n"),
        c_str!("mtk_dai_tdm_hw_params"),
        out_channels_per_sdata,
    );

    /* set tdm */
    if (*tdm_priv).bck_invert != 0 {
        regmap_update_bits(
            (*afe).regmap,
            AUDIO_TOP_CON3,
            BCK_INVERSE_MASK_SFT,
            0x1 << BCK_INVERSE_SFT,
        );
    }

    if (*tdm_priv).lck_invert != 0 {
        tdm_con |= 1 << LRCK_INVERSE_SFT;
    }

    if (*tdm_priv).tdm_out_mode == TDM_OUT_I2S as i32 {
        tdm_con |= 1 << DELAY_DATA_SFT;
        tdm_con |= get_tdm_lrck_width(format) << LRCK_TDM_WIDTH_SFT;
    } else if (*tdm_priv).tdm_out_mode == TDM_OUT_DSP_A as i32 {
        tdm_con |= 1 << DELAY_DATA_SFT;
        tdm_con |= 0 << LRCK_TDM_WIDTH_SFT;
    } else if (*tdm_priv).tdm_out_mode == TDM_OUT_DSP_B as i32 {
        tdm_con |= 0 << DELAY_DATA_SFT;
        tdm_con |= 0 << LRCK_TDM_WIDTH_SFT;
    }

    tdm_con |= 1 << LEFT_ALIGN_SFT;
    tdm_con |= get_tdm_wlen(format) << WLEN_SFT;
    tdm_con |= get_tdm_ch(out_channels_per_sdata) << CHANNEL_NUM_SFT;
    tdm_con |= get_tdm_channel_bck(format) << CHANNEL_BCK_CYCLES_SFT;
    regmap_write((*afe).regmap, AFE_TDM_CON1, tdm_con);

    if out_channels_per_sdata == 2 {
        match channels {
            1 | 2 => {
                tdm_con = TDM_CH_START_O30_O31 << ST_CH_PAIR_SOUT0_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT1_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT2_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT3_SFT;
            }
            3 | 4 => {
                tdm_con = TDM_CH_START_O30_O31 << ST_CH_PAIR_SOUT0_SFT;
                tdm_con |= TDM_CH_START_O32_O33 << ST_CH_PAIR_SOUT1_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT2_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT3_SFT;
            }
            5 | 6 => {
                tdm_con = TDM_CH_START_O30_O31 << ST_CH_PAIR_SOUT0_SFT;
                tdm_con |= TDM_CH_START_O32_O33 << ST_CH_PAIR_SOUT1_SFT;
                tdm_con |= TDM_CH_START_O34_O35 << ST_CH_PAIR_SOUT2_SFT;
                tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT3_SFT;
            }
            7 | 8 => {
                tdm_con = TDM_CH_START_O30_O31 << ST_CH_PAIR_SOUT0_SFT;
                tdm_con |= TDM_CH_START_O32_O33 << ST_CH_PAIR_SOUT1_SFT;
                tdm_con |= TDM_CH_START_O34_O35 << ST_CH_PAIR_SOUT2_SFT;
                tdm_con |= TDM_CH_START_O36_O37 << ST_CH_PAIR_SOUT3_SFT;
            }
            _ => {
                tdm_con = 0;
            }
        }
    } else {
        tdm_con = TDM_CH_START_O30_O31 << ST_CH_PAIR_SOUT0_SFT;
        tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT1_SFT;
        tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT2_SFT;
        tdm_con |= TDM_CH_ZERO << ST_CH_PAIR_SOUT3_SFT;
    }

    regmap_write((*afe).regmap, AFE_TDM_CON2, tdm_con);

    regmap_update_bits(
        (*afe).regmap,
        AFE_HDMI_OUT_CON0,
        HDMI_CH_NUM_MASK_SFT,
        channels << HDMI_CH_NUM_SFT,
    );
    0
}

unsafe fn mtk_dai_tdm_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: i32,
    freq: u32,
    dir: i32,
) -> i32 {
    let afe = dev_get_drvdata((*dai).dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn((*afe).dev, c_str!("%s(), tdm_priv == NULL"), c_str!("mtk_dai_tdm_set_sysclk"));
        return -EINVAL;
    }

    if dir != SND_SOC_CLOCK_OUT {
        dev_warn((*afe).dev, c_str!("%s(), dir != SND_SOC_CLOCK_OUT"), c_str!("mtk_dai_tdm_set_sysclk"));
        return -EINVAL;
    }

    dev_dbg((*afe).dev, c_str!("%s(), freq %d\n"), c_str!("mtk_dai_tdm_set_sysclk"), freq);

    mtk_dai_tdm_cal_mclk(afe, tdm_priv, freq as i32)
}

unsafe fn mtk_dai_tdm_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let afe = dev_get_drvdata((*dai).dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn((*afe).dev, c_str!("%s(), tdm_priv == NULL"), c_str!("mtk_dai_tdm_set_fmt"));
        return -EINVAL;
    }

    /* DAI mode*/
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_I2S as i32;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_DSP_A as i32;
        }
        SND_SOC_DAIFMT_DSP_B => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_DSP_B as i32;
        }
        _ => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_I2S as i32;
        }
    }

    /* DAI clock inversion*/
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            (*tdm_priv).bck_invert = TDM_BCK_NON_INV;
            (*tdm_priv).lck_invert = TDM_LCK_NON_INV;
        }
        SND_SOC_DAIFMT_NB_IF => {
            (*tdm_priv).bck_invert = TDM_BCK_NON_INV;
            (*tdm_priv).lck_invert = TDM_LCK_INV;
        }
        SND_SOC_DAIFMT_IB_NF => {
            (*tdm_priv).bck_invert = TDM_BCK_INV;
            (*tdm_priv).lck_invert = TDM_LCK_NON_INV;
        }
        SND_SOC_DAIFMT_IB_IF | _ => {
            (*tdm_priv).bck_invert = TDM_BCK_INV;
            (*tdm_priv).lck_invert = TDM_LCK_INV;
        }
    }

    0
}

static mtk_dai_tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_tdm_hw_params),
    set_sysclk: Some(mtk_dai_tdm_set_sysclk),
    set_fmt: Some(mtk_dai_tdm_set_fmt),
};

/* dai driver */
const MTK_TDM_RATES: u32 = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_TDM_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_tdm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c_str!("TDM"),
    id: MT8192_DAI_TDM,
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("TDM"),
        channels_min: 2,
        channels_max: 8,
        rates: MTK_TDM_RATES,
        formats: MTK_TDM_FORMATS,
    },
    ops: &mtk_dai_tdm_ops,
}];

unsafe fn init_tdm_priv_data(afe: *mut mtk_base_afe) -> *mut mtk_afe_tdm_priv {
    let tdm_priv: *mut mtk_afe_tdm_priv;

    tdm_priv = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_afe_tdm_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_afe_tdm_priv;
    if tdm_priv.is_null() {
        return null_mut();
    }

    (*tdm_priv).mclk_multiple = 512;
    (*tdm_priv).bck_id = MT8192_I2S4_BCK;
    (*tdm_priv).mclk_id = MT8192_I2S4_MCK;
    (*tdm_priv).id = MT8192_DAI_TDM;

    tdm_priv
}

pub unsafe fn mt8192_dai_tdm_register(afe: *mut mtk_base_afe) -> i32 {
    let afe_priv = (*afe).platform_priv as *mut mt8192_afe_private;
    let tdm_priv: *mut mtk_afe_tdm_priv;
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_tdm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_tdm_driver.len();

    (*dai).dapm_widgets = mtk_dai_tdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_tdm_widgets.len();
    (*dai).dapm_routes = mtk_dai_tdm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_tdm_routes.len();

    tdm_priv = init_tdm_priv_data(afe);
    if tdm_priv.is_null() {
        return -ENOMEM;
    }

    (*afe_priv).dai_priv[MT8192_DAI_TDM as usize] = tdm_priv as *mut _;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
