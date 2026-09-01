// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI TDM Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// Dependencies from:
// <linux/regmap.h>
// <sound/pcm_params.h>
// "mt8183-afe-clk.h"
// "mt8183-afe-common.h"
// "mt8183-interconnection.h"
// "mt8183-reg.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type snd_pcm_format_t = c_int;

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
    connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
    id: c_int,
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct mtk_base_afe {
    dev: *mut device,
    regmap: *mut regmap,
    platform_priv: *mut mt8183_afe_private,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8183_afe_private {
    dai_priv: [*mut c_void; MT8183_DAI_NUM],
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

#[repr(C)]
struct mtk_afe_tdm_priv {
    bck_id: c_int,
    bck_rate: c_int,
    tdm_out_mode: c_int,
    bck_invert: c_int,
    lck_invert: c_int,
    mclk_id: c_int,
    mclk_multiple: c_int, /* according to sample rate */
    mclk_rate: c_int,
    mclk_apll: c_int,
}

const TDM_OUT_I2S: c_int = 0;
const TDM_OUT_TDM: c_int = 1;

const TDM_BCK_NON_INV: c_int = 0;
const TDM_BCK_INV: c_int = 1;

const TDM_LCK_NON_INV: c_int = 0;
const TDM_LCK_INV: c_int = 1;

const TDM_WLEN_16_BIT: c_uint = 1;
const TDM_WLEN_32_BIT: c_uint = 2;

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

const HDMI_BIT_WIDTH_16_BIT: c_uint = 0;
const HDMI_BIT_WIDTH_32_BIT: c_uint = 1;

unsafe extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn mt8183_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int);
    fn mt8183_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8183_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8183_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
    fn mt8183_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static hdmi_ch0_mux_control: snd_kcontrol_new;
    static hdmi_ch1_mux_control: snd_kcontrol_new;
    static hdmi_ch2_mux_control: snd_kcontrol_new;
    static hdmi_ch3_mux_control: snd_kcontrol_new;
    static hdmi_ch4_mux_control: snd_kcontrol_new;
    static hdmi_ch5_mux_control: snd_kcontrol_new;
    static hdmi_ch6_mux_control: snd_kcontrol_new;
    static hdmi_ch7_mux_control: snd_kcontrol_new;
}

unsafe fn get_hdmi_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        HDMI_BIT_WIDTH_16_BIT
    } else {
        HDMI_BIT_WIDTH_32_BIT
    }
}

unsafe fn get_tdm_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        TDM_WLEN_16_BIT
    } else {
        TDM_WLEN_32_BIT
    }
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

fn get_tdm_ch_fixup(channels: c_uint) -> c_uint {
    if channels > 4 {
        8
    } else if channels > 2 {
        4
    } else {
        2
    }
}

fn get_tdm_ch_per_sdata(mode: c_uint, channels: c_uint) -> c_uint {
    if mode == TDM_OUT_TDM as c_uint {
        get_tdm_ch_fixup(channels)
    } else {
        2
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
    b"CH0\0".as_ptr() as *const c_char,
    b"CH1\0".as_ptr() as *const c_char,
    b"CH2\0".as_ptr() as *const c_char,
    b"CH3\0".as_ptr() as *const c_char,
    b"CH4\0".as_ptr() as *const c_char,
    b"CH5\0".as_ptr() as *const c_char,
    b"CH6\0".as_ptr() as *const c_char,
    b"CH7\0".as_ptr() as *const c_char,
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

// SOC_VALUE_ENUM_SINGLE_DECL/SOC_DAPM_ENUM declarations are supplied by ALSA macros in C.
// Their generated objects are referenced above as external static controls.

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_TDM_MCK_EN: c_int = 1;
const SUPPLY_SEQ_TDM_BCK_EN: c_int = 2;

unsafe extern "C" fn mtk_tdm_bck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let tdm_priv = (*afe_priv).dai_priv[MT8183_DAI_TDM] as *mut mtk_afe_tdm_priv;

    dev_info(
        (*cmpnt).dev,
        b"%s(), name %s, event 0x%x\n\0".as_ptr() as *const c_char,
        b"mtk_tdm_bck_en_event\0".as_ptr(),
        (*w).name,
        event,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => mt8183_mck_enable(afe, (*tdm_priv).bck_id, (*tdm_priv).bck_rate),
        SND_SOC_DAPM_POST_PMD => mt8183_mck_disable(afe, (*tdm_priv).bck_id),
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
    let tdm_priv = (*afe_priv).dai_priv[MT8183_DAI_TDM] as *mut mtk_afe_tdm_priv;

    dev_info(
        (*cmpnt).dev,
        b"%s(), name %s, event 0x%x\n\0".as_ptr() as *const c_char,
        b"mtk_tdm_mck_en_event\0".as_ptr(),
        (*w).name,
        event,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => mt8183_mck_enable(afe, (*tdm_priv).mclk_id, (*tdm_priv).mclk_rate),
        SND_SOC_DAPM_POST_PMD => {
            (*tdm_priv).mclk_rate = 0;
            mt8183_mck_disable(afe, (*tdm_priv).mclk_id);
        }
        _ => {}
    }

    0
}

// SND_SOC_DAPM_* macro initializers require dependency-provided layouts.
static mtk_dai_tdm_widgets: [snd_soc_dapm_widget; 0] = [];

unsafe extern "C" fn mtk_afe_tdm_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let tdm_priv = (*afe_priv).dai_priv[MT8183_DAI_TDM] as *mut mtk_afe_tdm_priv;
    let cur_apll: c_int;

    /* which apll */
    cur_apll = mt8183_get_apll_by_name(afe, (*source).name);

    if (*tdm_priv).mclk_apll == cur_apll {
        1
    } else {
        0
    }
}

static mtk_dai_tdm_routes: [snd_soc_dapm_route; 77] = [
    route(b"HDMI_CH0_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH0_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH1_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH2_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH3_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH4_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH5_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH6_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH0\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH1\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH2\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH3\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH4\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH5\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH6\0", b"HDMI\0", None),
    route(b"HDMI_CH7_MUX\0", b"CH7\0", b"HDMI\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH0_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH1_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH2_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH3_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH4_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH5_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH6_MUX\0", None),
    route(b"TDM\0", b"\0", b"HDMI_CH7_MUX\0", None),
    route(b"TDM\0", b"\0", b"aud_tdm_clk\0", None),
    route(b"TDM\0", b"\0", b"TDM_BCK\0", None),
    route(b"TDM_BCK\0", b"\0", b"TDM_MCK\0", None),
    route_ptr(
        b"TDM_MCK\0",
        ptr::null(),
        APLL1_W_NAME,
        Some(mtk_afe_tdm_apll_connect),
    ),
    route_ptr(
        b"TDM_MCK\0",
        ptr::null(),
        APLL2_W_NAME,
        Some(mtk_afe_tdm_apll_connect),
    ),
];

const fn route(
    sink: &'static [u8],
    control: &'static [u8],
    source: &'static [u8],
    connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
) -> snd_soc_dapm_route {
    snd_soc_dapm_route {
        sink: sink.as_ptr() as *const c_char,
        control: if control.len() == 1 { ptr::null() } else { control.as_ptr() as *const c_char },
        source: source.as_ptr() as *const c_char,
        connected,
    }
}

const fn route_ptr(
    sink: &'static [u8],
    control: *const c_char,
    source: *const c_char,
    connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
) -> snd_soc_dapm_route {
    snd_soc_dapm_route {
        sink: sink.as_ptr() as *const c_char,
        control,
        source,
        connected,
    }
}

/* dai ops */
unsafe fn mtk_dai_tdm_cal_mclk(
    afe: *mut mtk_base_afe,
    tdm_priv: *mut mtk_afe_tdm_priv,
    freq: c_int,
) -> c_int {
    let apll: c_int;
    let apll_rate: c_int;

    apll = mt8183_get_apll_by_rate(afe, freq);
    apll_rate = mt8183_get_apll_rate(afe, apll);

    if freq == 0 || freq > apll_rate {
        dev_warn(
            (*afe).dev,
            b"%s(), freq(%d Hz) invalid\n\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_cal_mclk\0".as_ptr(),
            freq,
        );
        return -EINVAL;
    }

    if apll_rate % freq != 0 {
        dev_warn(
            (*afe).dev,
            b"%s(), APLL cannot generate %d Hz\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_cal_mclk\0".as_ptr(),
            freq,
        );
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
    let tdm_priv = (*afe_priv).dai_priv[tdm_id as usize] as *mut mtk_afe_tdm_priv;
    let tdm_out_mode = (*tdm_priv).tdm_out_mode as c_uint;
    let rate = params_rate(params);
    let channels = params_channels(params);
    let out_channels_per_sdata = get_tdm_ch_per_sdata(tdm_out_mode, channels);
    let format = params_format(params);
    let mut tdm_con: c_uint = 0;

    /* calculate mclk_rate, if not set explicitly */
    if (*tdm_priv).mclk_rate == 0 {
        (*tdm_priv).mclk_rate = (rate as c_int) * (*tdm_priv).mclk_multiple;
        mtk_dai_tdm_cal_mclk(afe, tdm_priv, (*tdm_priv).mclk_rate);
    }

    /* calculate bck */
    (*tdm_priv).bck_rate =
        (rate * out_channels_per_sdata * snd_pcm_format_physical_width(format) as c_uint) as c_int;

    if (*tdm_priv).bck_rate > (*tdm_priv).mclk_rate {
        dev_warn(
            (*afe).dev,
            b"%s(), bck_rate > mclk_rate rate\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_hw_params\0".as_ptr(),
        );
    }

    if (*tdm_priv).mclk_rate % (*tdm_priv).bck_rate != 0 {
        dev_warn(
            (*afe).dev,
            b"%s(), bck cannot generate\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_hw_params\0".as_ptr(),
        );
    }

    dev_info(
        (*afe).dev,
        b"%s(), id %d, rate %d, channels %d, format %d, mclk_rate %d, bck_rate %d\n\0".as_ptr()
            as *const c_char,
        b"mtk_dai_tdm_hw_params\0".as_ptr(),
        tdm_id,
        rate,
        channels,
        format,
        (*tdm_priv).mclk_rate,
        (*tdm_priv).bck_rate,
    );
    dev_info(
        (*afe).dev,
        b"%s(), out_channels_per_sdata = %d\n\0".as_ptr() as *const c_char,
        b"mtk_dai_tdm_hw_params\0".as_ptr(),
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

    if (*tdm_priv).tdm_out_mode == TDM_OUT_I2S {
        tdm_con |= 1 << DELAY_DATA_SFT;
        tdm_con |= get_tdm_lrck_width(format) << LRCK_TDM_WIDTH_SFT;
    } else if (*tdm_priv).tdm_out_mode == TDM_OUT_TDM {
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
        AFE_HDMI_OUT_CH_NUM_MASK_SFT,
        channels << AFE_HDMI_OUT_CH_NUM_SFT,
    );

    regmap_update_bits(
        (*afe).regmap,
        AFE_HDMI_OUT_CON0,
        AFE_HDMI_OUT_BIT_WIDTH_MASK_SFT,
        get_hdmi_wlen(format) << AFE_HDMI_OUT_BIT_WIDTH_SFT,
    );
    0
}

unsafe extern "C" fn mtk_dai_tdm_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* enable Out control */
            regmap_update_bits(
                (*afe).regmap,
                AFE_HDMI_OUT_CON0,
                AFE_HDMI_OUT_ON_MASK_SFT,
                0x1 << AFE_HDMI_OUT_ON_SFT,
            );
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
            /* disable Out control */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, AFE_HDMI_OUT_ON_MASK_SFT, 0);
        }
        _ => return -EINVAL,
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
    let tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn(
            (*afe).dev,
            b"%s(), tdm_priv == NULL\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_set_sysclk\0".as_ptr(),
        );
        return -EINVAL;
    }

    if dir != SND_SOC_CLOCK_OUT {
        dev_warn(
            (*afe).dev,
            b"%s(), dir != SND_SOC_CLOCK_OUT\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_set_sysclk\0".as_ptr(),
        );
        return -EINVAL;
    }

    dev_info(
        (*afe).dev,
        b"%s(), freq %d\n\0".as_ptr() as *const c_char,
        b"mtk_dai_tdm_set_sysclk\0".as_ptr(),
        freq,
    );

    mtk_dai_tdm_cal_mclk(afe, tdm_priv, freq as c_int)
}

unsafe extern "C" fn mtk_dai_tdm_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let afe = dev_get_drvdata((*dai).dev) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        dev_warn(
            (*afe).dev,
            b"%s(), tdm_priv == NULL\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_set_fmt\0".as_ptr(),
        );
        return -EINVAL;
    }

    /* DAI mode*/
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_I2S;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_TDM;
        }
        _ => {
            (*tdm_priv).tdm_out_mode = TDM_OUT_I2S;
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
    trigger: Some(mtk_dai_tdm_trigger),
    set_sysclk: Some(mtk_dai_tdm_set_sysclk),
    set_fmt: Some(mtk_dai_tdm_set_fmt),
};

/* dai driver */
const MTK_TDM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_TDM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_tdm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"TDM\0".as_ptr() as *const c_char,
    id: MT8183_DAI_TDM as c_int,
    playback: snd_soc_pcm_stream {
        stream_name: b"TDM\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 8,
        rates: MTK_TDM_RATES,
        formats: MTK_TDM_FORMATS,
    },
    ops: &mtk_dai_tdm_ops as *const snd_soc_dai_ops,
}];

#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut tdm_priv: *mut mtk_afe_tdm_priv;
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_tdm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_tdm_driver.len() as c_uint;

    (*dai).dapm_widgets = mtk_dai_tdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_tdm_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_tdm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_tdm_routes.len() as c_uint;

    tdm_priv = devm_kzalloc((*afe).dev, size_of::<mtk_afe_tdm_priv>(), GFP_KERNEL) as *mut mtk_afe_tdm_priv;
    if tdm_priv.is_null() {
        return -ENOMEM;
    }

    (*tdm_priv).mclk_multiple = 128;
    (*tdm_priv).bck_id = MT8183_I2S4_BCK;
    (*tdm_priv).mclk_id = MT8183_I2S4_MCK;

    (*afe_priv).dai_priv[MT8183_DAI_TDM] = tdm_priv as *mut c_void;
    0
}

unsafe extern "C" {
    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;
}

// Constants, register offsets, masks, shifts, trigger values, format flags,
// allocation flags, and SoC IDs named below are dependency-provided equivalents
// of the included C headers. They are intentionally referenced by name here.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
