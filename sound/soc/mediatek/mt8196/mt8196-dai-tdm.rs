// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI TDM Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

// C dependencies:
// linux/regmap.h
// sound/pcm_params.h
// mt8196-afe-clk.h
// mt8196-afe-common.h
// mt8196-interconnection.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut mt8196_afe_private,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mt8196_afe_private {
    pub dai_priv: [*mut c_void; MT8196_DAI_NUM as usize],
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct mtk_afe_tdm_priv {
    bck_id: c_int,
    bck_rate: c_int,

    mclk_id: c_int,
    mclk_multiple: c_int, /* according to sample rate */
    mclk_rate: c_int,
    mclk_apll: c_int,
}

const TDM_WLEN_8_BIT: c_uint = 0;
const TDM_WLEN_16_BIT: c_uint = 1;
const TDM_WLEN_24_BIT: c_uint = 2;
const TDM_WLEN_32_BIT: c_uint = 3;

const TDM_CHANNEL_BCK_16: c_uint = 0;
const TDM_CHANNEL_BCK_24: c_uint = 1;
const TDM_CHANNEL_BCK_32: c_uint = 2;

const TDM_CHANNEL_NUM_2: c_uint = 0;
const TDM_CHANNEL_NUM_4: c_uint = 1;
const TDM_CHANNEL_NUM_8: c_uint = 2;

const TDM_CH_START_O30_O31: c_uint = 0;
const TDM_CH_START_O32_O33: c_uint = 1;
const TDM_CH_START_O34_O35: c_uint = 2;
const TDM_CH_START_O36_O37: c_uint = 3;
const TDM_CH_ZERO: c_uint = 4;

const DPTX_CHANNEL_2: c_uint = 0;
const DPTX_CHANNEL_8: c_uint = 1;

const DPTX_WLEN_24_BIT: c_uint = 0;
const DPTX_WLEN_16_BIT: c_uint = 1;

const DPTX_CH_EN_MASK_2CH: c_uint = 0x3;
const DPTX_CH_EN_MASK_4CH: c_uint = 0xf;
const DPTX_CH_EN_MASK_6CH: c_uint = 0x3f;
const DPTX_CH_EN_MASK_8CH: c_uint = 0xff;

unsafe fn get_tdm_wlen(format: snd_pcm_format_t) -> c_uint {
    (snd_pcm_format_physical_width(format) / 8 - 1) as c_uint
}

unsafe fn get_tdm_channel_bck(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        TDM_CHANNEL_BCK_16
    } else {
        TDM_CHANNEL_BCK_32
    }
}

unsafe fn get_tdm_lrck_width(format: snd_pcm_format_t) -> c_uint {
    (snd_pcm_format_physical_width(format) - 1) as c_uint
}

fn get_tdm_ch(ch: c_uint) -> c_uint {
    match ch {
        1 | 2 => TDM_CHANNEL_NUM_2,
        3 | 4 => TDM_CHANNEL_NUM_4,
        5 | 6 | 7 | 8 => TDM_CHANNEL_NUM_8,
        _ => TDM_CHANNEL_NUM_8,
    }
}

unsafe fn get_dptx_ch_enable_mask(dev: *mut device, ch: c_uint) -> c_uint {
    match ch {
        1 | 2 => DPTX_CH_EN_MASK_2CH,
        3 | 4 => DPTX_CH_EN_MASK_4CH,
        5 | 6 => DPTX_CH_EN_MASK_6CH,
        7 | 8 => DPTX_CH_EN_MASK_8CH,
        _ => {
            dev_warn(dev, c"invalid channel num, default use 2ch\n".as_ptr());
            DPTX_CH_EN_MASK_2CH
        }
    }
}

fn get_dptx_ch(ch: c_uint) -> c_uint {
    if ch == 2 {
        DPTX_CHANNEL_2
    } else {
        DPTX_CHANNEL_8
    }
}

unsafe fn get_dptx_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        DPTX_WLEN_16_BIT
    } else {
        DPTX_WLEN_24_BIT
    }
}

/* interconnection */
const HDMI_CONN_CH0: c_int = 0;
const HDMI_CONN_CH1: c_int = 1;
const HDMI_CONN_CH2: c_int = 2;
const HDMI_CONN_CH3: c_int = 3;
const HDMI_CONN_CH4: c_int = 4;
const HDMI_CONN_CH5: c_int = 5;
const HDMI_CONN_CH6: c_int = 6;
const HDMI_CONN_CH7: c_int = 7;

static hdmi_conn_mux_map: [*const c_char; 8] = [
    c"CH0".as_ptr(),
    c"CH1".as_ptr(),
    c"CH2".as_ptr(),
    c"CH3".as_ptr(),
    c"CH4".as_ptr(),
    c"CH5".as_ptr(),
    c"CH6".as_ptr(),
    c"CH7".as_ptr(),
];

static mut hdmi_conn_mux_map_value: [c_int; 8] = [
    HDMI_CONN_CH0,
    HDMI_CONN_CH1,
    HDMI_CONN_CH2,
    HDMI_CONN_CH3,
    HDMI_CONN_CH4,
    HDMI_CONN_CH5,
    HDMI_CONN_CH6,
    HDMI_CONN_CH7,
];

// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch0_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_0_SFT,
//                           HDMI_O_0_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch1_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_1_SFT,
//                           HDMI_O_1_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch2_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_2_SFT,
//                           HDMI_O_2_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch3_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_3_SFT,
//                           HDMI_O_3_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch4_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_4_SFT,
//                           HDMI_O_4_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch5_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_5_SFT,
//                           HDMI_O_5_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch6_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_6_SFT,
//                           HDMI_O_6_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)
// SOC_VALUE_ENUM_SINGLE_DECL(hdmi_ch7_mux_map_enum, AFE_HDMI_CONN0, HDMI_O_7_SFT,
//                           HDMI_O_7_MASK, hdmi_conn_mux_map, hdmi_conn_mux_map_value)

static mtk_dai_tdm_controls: [snd_kcontrol_new; 8] = [
    SOC_ENUM(c"HDMI_CH0_MUX".as_ptr(), hdmi_ch0_mux_map_enum),
    SOC_ENUM(c"HDMI_CH1_MUX".as_ptr(), hdmi_ch1_mux_map_enum),
    SOC_ENUM(c"HDMI_CH2_MUX".as_ptr(), hdmi_ch2_mux_map_enum),
    SOC_ENUM(c"HDMI_CH3_MUX".as_ptr(), hdmi_ch3_mux_map_enum),
    SOC_ENUM(c"HDMI_CH4_MUX".as_ptr(), hdmi_ch4_mux_map_enum),
    SOC_ENUM(c"HDMI_CH5_MUX".as_ptr(), hdmi_ch5_mux_map_enum),
    SOC_ENUM(c"HDMI_CH6_MUX".as_ptr(), hdmi_ch6_mux_map_enum),
    SOC_ENUM(c"HDMI_CH7_MUX".as_ptr(), hdmi_ch7_mux_map_enum),
];

static tdm_out_demux_texts: [*const c_char; 3] = [
    c"NONE".as_ptr(),
    c"TDMOUT".as_ptr(),
    c"DPTXOUT".as_ptr(),
];

// SOC_ENUM_SINGLE_DECL(tdm_out_demux_enum, SND_SOC_NOPM, 0, tdm_out_demux_texts)
static tdm_out_demux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM(c"TDM DEMUX ROUTE".as_ptr(), tdm_out_demux_enum);

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_TDM_MCK_EN: c_int = 1;
const SUPPLY_SEQ_TDM_BCK_EN: c_int = 2;
const SUPPLY_SEQ_TDM_DPTX_MCK_EN: c_int = 3;
const SUPPLY_SEQ_TDM_DPTX_BCK_EN: c_int = 4;
const SUPPLY_SEQ_TDM_CG_EN: c_int = 5;

unsafe fn get_tdm_id_by_name(name: *const c_char) -> c_int {
    if !strstr(name, c"DPTX".as_ptr()).is_null() {
        MT8196_DAI_TDM_DPTX
    } else {
        MT8196_DAI_TDM
    }
}

unsafe extern "C" fn mtk_tdm_bck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    dev_dbg(
        (*cmpnt).dev,
        c"name %s, event 0x%x, dai_id %d\n".as_ptr(),
        (*w).name,
        event,
        dai_id,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8196_mck_enable(afe, (*tdm_priv).bck_id, (*tdm_priv).bck_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8196_mck_disable(afe, (*tdm_priv).bck_id);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_tdm_mck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    dev_dbg(
        (*cmpnt).dev,
        c"name %s, event 0x%x, dai_id %d\n".as_ptr(),
        (*w).name,
        event,
        dai_id,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8196_mck_enable(afe, (*tdm_priv).mclk_id, (*tdm_priv).mclk_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*tdm_priv).mclk_rate = 0;
            mt8196_mck_disable(afe, (*tdm_priv).mclk_id);
        }
        _ => {}
    }

    0
}

static mtk_dai_tdm_widgets: [snd_soc_dapm_widget_item; 6] = [
    SND_SOC_DAPM_DEMUX(
        c"TDM_DEMUX".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        &tdm_out_demux_control,
    ),
    SND_SOC_DAPM_SUPPLY_S(
        c"TDM_BCK".as_ptr(),
        SUPPLY_SEQ_TDM_BCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_tdm_bck_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_SUPPLY_S(
        c"TDM_MCK".as_ptr(),
        SUPPLY_SEQ_TDM_MCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_tdm_mck_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_SUPPLY_S(
        c"TDM_DPTX_BCK".as_ptr(),
        SUPPLY_SEQ_TDM_DPTX_BCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_tdm_bck_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_SUPPLY_S(
        c"TDM_DPTX_MCK".as_ptr(),
        SUPPLY_SEQ_TDM_DPTX_MCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_tdm_mck_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    /* cg */
    SND_SOC_DAPM_SUPPLY_S(
        c"TDM_CG".as_ptr(),
        SUPPLY_SEQ_TDM_CG_EN,
        AUDIO_TOP_CON2,
        PDN_TDM_OUT_SFT,
        1,
        None,
        0,
    ),
];

unsafe extern "C" fn mtk_afe_tdm_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*sink).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*sink).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;
    let cur_apll: c_int;

    /* which apll */
    cur_apll = mt8196_get_apll_by_name(afe, (*source).name);

    if (*tdm_priv).mclk_apll == cur_apll {
        1
    } else {
        0
    }
}

static mtk_dai_tdm_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: c"TDM_DEMUX".as_ptr(), control: ptr::null(), source: c"HDMI".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM".as_ptr(), control: c"TDMOUT".as_ptr(), source: c"TDM_DEMUX".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM".as_ptr(), control: ptr::null(), source: c"TDM_BCK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM".as_ptr(), control: ptr::null(), source: c"TDM_CG".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_DPTX".as_ptr(), control: c"DPTXOUT".as_ptr(), source: c"TDM_DEMUX".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_DPTX".as_ptr(), control: ptr::null(), source: c"TDM_DPTX_BCK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_DPTX".as_ptr(), control: ptr::null(), source: c"TDM_CG".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_BCK".as_ptr(), control: ptr::null(), source: c"TDM_MCK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_DPTX_BCK".as_ptr(), control: ptr::null(), source: c"TDM_DPTX_MCK".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"TDM_MCK".as_ptr(), control: ptr::null(), source: APLL1_W_NAME, connected: Some(mtk_afe_tdm_apll_connect) },
    snd_soc_dapm_route { sink: c"TDM_MCK".as_ptr(), control: ptr::null(), source: APLL2_W_NAME, connected: Some(mtk_afe_tdm_apll_connect) },
    snd_soc_dapm_route { sink: c"TDM_DPTX_MCK".as_ptr(), control: ptr::null(), source: APLL1_W_NAME, connected: Some(mtk_afe_tdm_apll_connect) },
    snd_soc_dapm_route { sink: c"TDM_DPTX_MCK".as_ptr(), control: ptr::null(), source: APLL2_W_NAME, connected: Some(mtk_afe_tdm_apll_connect) },
];

/* dai ops */
unsafe fn mtk_dai_tdm_cal_mclk(
    afe: *mut mtk_base_afe,
    tdm_priv: *mut mtk_afe_tdm_priv,
    freq: c_int,
) -> c_int {
    let apll: c_int;
    let apll_rate: c_int;

    apll = mt8196_get_apll_by_rate(afe, freq);
    apll_rate = mt8196_get_apll_rate(afe, apll);

    if freq > apll_rate {
        return -EINVAL;
    }

    if apll_rate % freq != 0 {
        return -EINVAL;
    }

    (*tdm_priv).mclk_rate = freq;
    (*tdm_priv).mclk_apll = apll;

    0
}

unsafe extern "C" fn mtk_dai_tdm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let tdm_id = (*dai).id;
    let mut tdm_priv: *mut mtk_afe_tdm_priv;
    let rate = params_rate(params);
    let channels = params_channels(params);
    let format = params_format(params);
    let mut tdm_con: c_uint = 0;

    if tdm_id >= MT8196_DAI_NUM || tdm_id < 0 {
        return -EINVAL;
    }

    tdm_priv = (*afe_priv).dai_priv[tdm_id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        return -EINVAL;
    }

    /* calculate mclk_rate, if not set explicitly */
    if (*tdm_priv).mclk_rate == 0 {
        (*tdm_priv).mclk_rate = (rate as c_int) * (*tdm_priv).mclk_multiple;
        mtk_dai_tdm_cal_mclk(afe, tdm_priv, (*tdm_priv).mclk_rate);
    }

    /* calculate bck */
    (*tdm_priv).bck_rate =
        (rate * channels * snd_pcm_format_physical_width(format) as c_uint) as c_int;

    if (*tdm_priv).bck_rate > (*tdm_priv).mclk_rate {
        return -EINVAL;
    }

    if (*tdm_priv).mclk_rate % (*tdm_priv).bck_rate != 0 {
        return -EINVAL;
    }

    dev_dbg(
        (*afe).dev,
        c"id %d, rate %d, ch %d, fmt %d, mclk %d, bck %d\n".as_ptr(),
        tdm_id,
        rate,
        channels,
        format,
        (*tdm_priv).mclk_rate,
        (*tdm_priv).bck_rate,
    );

    /* set tdm */
    tdm_con = 0 << BCK_INVERSE_SFT;
    tdm_con |= 0 << LRCK_INVERSE_SFT;
    tdm_con |= 0 << DELAY_DATA_SFT;
    tdm_con |= 1 << LEFT_ALIGN_SFT;
    tdm_con |= get_tdm_wlen(format) << WLEN_SFT;
    tdm_con |= get_tdm_ch(channels) << CHANNEL_NUM_SFT;
    tdm_con |= get_tdm_channel_bck(format) << CHANNEL_BCK_CYCLES_SFT;
    tdm_con |= get_tdm_lrck_width(format) << LRCK_TDM_WIDTH_SFT;
    regmap_write((*afe).regmap, AFE_TDM_CON1, tdm_con);

    /* set dptx */
    if tdm_id == MT8196_DAI_TDM_DPTX {
        regmap_update_bits(
            (*afe).regmap,
            AFE_DPTX_CON,
            DPTX_CHANNEL_ENABLE_MASK_SFT,
            get_dptx_ch_enable_mask((*afe).dev, channels) << DPTX_CHANNEL_ENABLE_SFT,
        );
        regmap_update_bits(
            (*afe).regmap,
            AFE_DPTX_CON,
            DPTX_CHANNEL_NUMBER_MASK_SFT,
            get_dptx_ch(channels) << DPTX_CHANNEL_NUMBER_SFT,
        );
        regmap_update_bits(
            (*afe).regmap,
            AFE_DPTX_CON,
            DPTX_16BIT_MASK_SFT,
            get_dptx_wlen(format) << DPTX_16BIT_SFT,
        );
    }
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

    regmap_write((*afe).regmap, AFE_TDM_CON2, tdm_con);
    regmap_update_bits(
        (*afe).regmap,
        AFE_HDMI_OUT_CON0,
        HDMI_CH_NUM_MASK_SFT,
        channels << HDMI_CH_NUM_SFT,
    );

    0
}

unsafe extern "C" fn mtk_dai_tdm_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let tdm_id = (*dai).id;

    dev_dbg((*afe).dev, c"cmd %d, tdm_id %d\n".as_ptr(), cmd, tdm_id);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* enable Out control */
            regmap_update_bits(
                (*afe).regmap,
                AFE_HDMI_OUT_CON0,
                HDMI_OUT_ON_MASK_SFT,
                0x1 << HDMI_OUT_ON_SFT,
            );

            /* enable dptx */
            if tdm_id == MT8196_DAI_TDM_DPTX {
                regmap_update_bits(
                    (*afe).regmap,
                    AFE_DPTX_CON,
                    DPTX_ON_MASK_SFT,
                    0x1 << DPTX_ON_SFT,
                );
            }

            /* enable tdm */
            regmap_update_bits(
                (*afe).regmap,
                AFE_TDM_CON1,
                TDM_EN_MASK_SFT,
                0x1 << TDM_EN_SFT,
            );
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            /* disable tdm */
            regmap_update_bits((*afe).regmap, AFE_TDM_CON1, TDM_EN_MASK_SFT, 0);

            /* disable dptx */
            if tdm_id == MT8196_DAI_TDM_DPTX {
                regmap_update_bits((*afe).regmap, AFE_DPTX_CON, DPTX_ON_MASK_SFT, 0);
            }

            /* disable Out control */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, HDMI_OUT_ON_MASK_SFT, 0);
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn mtk_dai_tdm_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let afe = dev_get_drvdata((*dai).dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let tdm_priv: *mut mtk_afe_tdm_priv;

    if (*dai).id >= MT8196_DAI_NUM || (*dai).id < 0 {
        return -EINVAL;
    }

    tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        return -EINVAL;
    }

    if dir != SND_SOC_CLOCK_OUT {
        return -EINVAL;
    }

    dev_dbg((*afe).dev, c"freq %d\n".as_ptr(), freq);

    mtk_dai_tdm_cal_mclk(afe, tdm_priv, freq as c_int)
}

static mtk_dai_tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_tdm_hw_params),
    trigger: Some(mtk_dai_tdm_trigger),
    set_sysclk: Some(mtk_dai_tdm_set_sysclk),
};

/* dai driver */
const MTK_TDM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_TDM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_tdm_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"TDM".as_ptr(),
        id: MT8196_DAI_TDM,
        playback: snd_soc_pcm_stream {
            stream_name: c"TDM".as_ptr(),
            channels_min: 2,
            channels_max: 8,
            rates: MTK_TDM_RATES,
            formats: MTK_TDM_FORMATS,
        },
        ops: &mtk_dai_tdm_ops,
    },
    snd_soc_dai_driver {
        name: c"TDM_DPTX".as_ptr(),
        id: MT8196_DAI_TDM_DPTX,
        playback: snd_soc_pcm_stream {
            stream_name: c"TDM_DPTX".as_ptr(),
            channels_min: 2,
            channels_max: 8,
            rates: MTK_TDM_RATES,
            formats: MTK_TDM_FORMATS,
        },
        ops: &mtk_dai_tdm_ops,
    },
];

unsafe fn init_tdm_priv_data(afe: *mut mtk_base_afe, id: c_int) -> *mut mtk_afe_tdm_priv {
    let tdm_priv: *mut mtk_afe_tdm_priv;

    tdm_priv = devm_kzalloc(
        (*afe).dev,
        size_of::<mtk_afe_tdm_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_afe_tdm_priv;
    if tdm_priv.is_null() {
        return ptr::null_mut();
    }

    if id == MT8196_DAI_TDM_DPTX {
        (*tdm_priv).mclk_multiple = 256;
    } else {
        (*tdm_priv).mclk_multiple = 128;
    }

    (*tdm_priv).bck_id = MT8196_TDMOUT_BCK;
    (*tdm_priv).mclk_id = MT8196_TDMOUT_MCK;

    tdm_priv
}

#[no_mangle]
pub unsafe extern "C" fn mt8196_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let tdm_priv: *mut mtk_afe_tdm_priv;
    let tdm_dptx_priv: *mut mtk_afe_tdm_priv;
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    (*dai).dai_drivers = mtk_dai_tdm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_tdm_driver.len() as c_int;
    (*dai).controls = mtk_dai_tdm_controls.as_ptr();
    (*dai).num_controls = mtk_dai_tdm_controls.len() as c_int;
    (*dai).dapm_widgets = mtk_dai_tdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_tdm_widgets.len() as c_int;
    (*dai).dapm_routes = mtk_dai_tdm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_tdm_routes.len() as c_int;

    tdm_priv = init_tdm_priv_data(afe, MT8196_DAI_TDM);
    if tdm_priv.is_null() {
        return -ENOMEM;
    }

    tdm_dptx_priv = init_tdm_priv_data(afe, MT8196_DAI_TDM_DPTX);
    if tdm_dptx_priv.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*afe_priv).dai_priv[MT8196_DAI_TDM as usize] = tdm_priv as *mut c_void;
    (*afe_priv).dai_priv[MT8196_DAI_TDM_DPTX as usize] = tdm_dptx_priv as *mut c_void;

    0
}

unsafe extern "C" {
    static hdmi_ch0_mux_map_enum: c_void;
    static hdmi_ch1_mux_map_enum: c_void;
    static hdmi_ch2_mux_map_enum: c_void;
    static hdmi_ch3_mux_map_enum: c_void;
    static hdmi_ch4_mux_map_enum: c_void;
    static hdmi_ch5_mux_map_enum: c_void;
    static hdmi_ch6_mux_map_enum: c_void;
    static hdmi_ch7_mux_map_enum: c_void;
    static tdm_out_demux_enum: c_void;

    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;

    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mt8196_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int) -> c_int;
    fn mt8196_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8196_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8196_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
    fn mt8196_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);

    fn SOC_ENUM(name: *const c_char, x: c_void) -> snd_kcontrol_new;
    fn SOC_DAPM_ENUM(name: *const c_char, x: c_void) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_DEMUX(
        name: *const c_char,
        reg: c_int,
        shift: c_uint,
        invert: c_uint,
        kcontrol: *const snd_kcontrol_new,
    ) -> snd_soc_dapm_widget_item;
    fn SND_SOC_DAPM_SUPPLY_S(
        name: *const c_char,
        subseq: c_int,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        event_flags: c_int,
    ) -> snd_soc_dapm_widget_item;
}

extern "Rust" {
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static GFP_KERNEL: c_uint;
    static MT8196_DAI_NUM: c_int;
    static MT8196_DAI_TDM: c_int;
    static MT8196_DAI_TDM_DPTX: c_int;
    static MT8196_TDMOUT_BCK: c_int;
    static MT8196_TDMOUT_MCK: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_CLOCK_OUT: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static AUDIO_TOP_CON2: c_uint;
    static PDN_TDM_OUT_SFT: c_uint;
    static AFE_HDMI_CONN0: c_uint;
    static HDMI_O_0_SFT: c_uint;
    static HDMI_O_0_MASK: c_uint;
    static HDMI_O_1_SFT: c_uint;
    static HDMI_O_1_MASK: c_uint;
    static HDMI_O_2_SFT: c_uint;
    static HDMI_O_2_MASK: c_uint;
    static HDMI_O_3_SFT: c_uint;
    static HDMI_O_3_MASK: c_uint;
    static HDMI_O_4_SFT: c_uint;
    static HDMI_O_4_MASK: c_uint;
    static HDMI_O_5_SFT: c_uint;
    static HDMI_O_5_MASK: c_uint;
    static HDMI_O_6_SFT: c_uint;
    static HDMI_O_6_MASK: c_uint;
    static HDMI_O_7_SFT: c_uint;
    static HDMI_O_7_MASK: c_uint;
    static BCK_INVERSE_SFT: c_uint;
    static LRCK_INVERSE_SFT: c_uint;
    static DELAY_DATA_SFT: c_uint;
    static LEFT_ALIGN_SFT: c_uint;
    static WLEN_SFT: c_uint;
    static CHANNEL_NUM_SFT: c_uint;
    static CHANNEL_BCK_CYCLES_SFT: c_uint;
    static LRCK_TDM_WIDTH_SFT: c_uint;
    static AFE_TDM_CON1: c_uint;
    static AFE_TDM_CON2: c_uint;
    static AFE_DPTX_CON: c_uint;
    static DPTX_CHANNEL_ENABLE_MASK_SFT: c_uint;
    static DPTX_CHANNEL_ENABLE_SFT: c_uint;
    static DPTX_CHANNEL_NUMBER_MASK_SFT: c_uint;
    static DPTX_CHANNEL_NUMBER_SFT: c_uint;
    static DPTX_16BIT_MASK_SFT: c_uint;
    static DPTX_16BIT_SFT: c_uint;
    static ST_CH_PAIR_SOUT0_SFT: c_uint;
    static ST_CH_PAIR_SOUT1_SFT: c_uint;
    static ST_CH_PAIR_SOUT2_SFT: c_uint;
    static ST_CH_PAIR_SOUT3_SFT: c_uint;
    static AFE_HDMI_OUT_CON0: c_uint;
    static HDMI_CH_NUM_MASK_SFT: c_uint;
    static HDMI_CH_NUM_SFT: c_uint;
    static HDMI_OUT_ON_MASK_SFT: c_uint;
    static HDMI_OUT_ON_SFT: c_uint;
    static DPTX_ON_MASK_SFT: c_uint;
    static DPTX_ON_SFT: c_uint;
    static TDM_EN_MASK_SFT: c_uint;
    static TDM_EN_SFT: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
