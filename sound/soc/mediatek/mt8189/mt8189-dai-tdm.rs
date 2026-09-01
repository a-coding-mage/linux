// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI TDM Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

// Dependencies originally included from:
// <linux/regmap.h>
// <sound/pcm_params.h>
// "mt8189-afe-clk.h"
// "mt8189-afe-common.h"
// "mt8189-interconnection.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const DPTX_CH_EN_MASK_2CH: c_uint = 0x3;
const DPTX_CH_EN_MASK_4CH: c_uint = 0xf;
const DPTX_CH_EN_MASK_6CH: c_uint = 0x3f;
const DPTX_CH_EN_MASK_8CH: c_uint = 0xff;

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_TDM_MCK_EN: c_int = 1;
const SUPPLY_SEQ_TDM_BCK_EN: c_int = 2;
const SUPPLY_SEQ_TDM_DPTX_MCK_EN: c_int = 3;
const SUPPLY_SEQ_TDM_DPTX_BCK_EN: c_int = 4;
const SUPPLY_SEQ_TDM_CG_EN: c_int = 5;

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

#[repr(C)]
struct mtk_afe_tdm_priv {
    bck_id: c_int,
    bck_rate: c_int,
    mclk_id: c_int,
    mclk_multiple: c_int, /* according to sample rate */
    mclk_rate: c_int,
    mclk_apll: c_int,
}

type snd_pcm_format_t = c_int;

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
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
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
    dev: *mut device,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct mtk_base_afe {
    dev: *mut device,
    regmap: *mut regmap,
    platform_priv: *mut mt8189_afe_private,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8189_afe_private {
    dai_priv: [*mut c_void; MT8189_DAI_NUM as usize],
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_ulong,
}

type c_ulong = core::ffi::c_ulong;

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

extern "C" {
    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;

    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn mt8189_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int);
    fn mt8189_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8189_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8189_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_int) -> c_int;
    fn mt8189_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

extern "C" {
    static mut hdmi_ch0_mux_map_enum: c_void;
    static mut hdmi_ch1_mux_map_enum: c_void;
    static mut hdmi_ch2_mux_map_enum: c_void;
    static mut hdmi_ch3_mux_map_enum: c_void;
    static mut hdmi_ch4_mux_map_enum: c_void;
    static mut hdmi_ch5_mux_map_enum: c_void;
    static mut hdmi_ch6_mux_map_enum: c_void;
    static mut hdmi_ch7_mux_map_enum: c_void;
    static mut tdm_out_demux_enum: c_void;
    static tdm_out_demux_control: snd_kcontrol_new;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;

extern "C" {
    static MT8189_DAI_TDM: c_int;
    static MT8189_DAI_TDM_DPTX: c_int;
    static MT8189_DAI_NUM: c_int;
    static MT8189_TDMOUT_BCK: c_int;
    static MT8189_TDMOUT_MCK: c_int;
    static AFE_TDM_CON1: c_uint;
    static AFE_TDM_CON2: c_uint;
    static AFE_DPTX_CON: c_uint;
    static AFE_HDMI_OUT_CON0: c_uint;
    static AUDIO_TOP_CON2: c_uint;
    static PDN_TDM_OUT_SFT: c_uint;
    static LEFT_ALIGN_SFT: c_uint;
    static WLEN_SFT: c_uint;
    static CHANNEL_NUM_SFT: c_uint;
    static CHANNEL_BCK_CYCLES_SFT: c_uint;
    static LRCK_TDM_WIDTH_SFT: c_uint;
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
    static SNDRV_PCM_FMTBIT_S16_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S24_LE: c_ulong;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulong;
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
        _ => TDM_CHANNEL_NUM_8,
    }
}

fn get_dptx_ch_enable_mask(ch: c_uint) -> c_uint {
    match ch {
        1 | 2 => DPTX_CH_EN_MASK_2CH,
        3 | 4 => DPTX_CH_EN_MASK_4CH,
        5 | 6 => DPTX_CH_EN_MASK_6CH,
        7 | 8 => DPTX_CH_EN_MASK_8CH,
        _ => DPTX_CH_EN_MASK_2CH,
    }
}

fn get_dptx_ch(ch: c_uint) -> c_uint {
    if ch == 2 {
        return DPTX_CHANNEL_2;
    }

    DPTX_CHANNEL_8
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

// SOC_VALUE_ENUM_SINGLE_DECL declarations for hdmi_ch[0-7]_mux_map_enum are
// represented above as external values supplied by ALSA macro support.

static mtk_dai_tdm_controls: [snd_kcontrol_new; 8] = [
    /* SOC_ENUM("HDMI_CH0_MUX", hdmi_ch0_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH1_MUX", hdmi_ch1_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH2_MUX", hdmi_ch2_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH3_MUX", hdmi_ch3_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH4_MUX", hdmi_ch4_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH5_MUX", hdmi_ch5_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH6_MUX", hdmi_ch6_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
    /* SOC_ENUM("HDMI_CH7_MUX", hdmi_ch7_mux_map_enum) */
    snd_kcontrol_new { _private: [] },
];

static tdm_out_demux_texts: [*const c_char; 3] = [
    b"NONE\0".as_ptr() as *const c_char,
    b"TDMOUT\0".as_ptr() as *const c_char,
    b"DPTXOUT\0".as_ptr() as *const c_char,
];

// SOC_ENUM_SINGLE_DECL(tdm_out_demux_enum, SND_SOC_NOPM, 0, tdm_out_demux_texts)
// and SOC_DAPM_ENUM("TDM Playback Route", tdm_out_demux_enum) are represented
// by external ALSA macro-generated values.

unsafe fn get_tdm_id_by_name(name: *const c_char) -> c_int {
    if !strstr(name, b"DPTX\0".as_ptr() as *const c_char).is_null() {
        return MT8189_DAI_TDM_DPTX;
    }

    MT8189_DAI_TDM
}

unsafe extern "C" fn mtk_tdm_bck_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    dev_dbg(
        (*cmpnt).dev,
        b"name %s, event 0x%x, dai_id %d, bck: %d\n\0".as_ptr() as *const c_char,
        (*w).name,
        event,
        dai_id,
        (*tdm_priv).bck_rate,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8189_mck_enable(afe, (*tdm_priv).bck_id, (*tdm_priv).bck_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8189_mck_disable(afe, (*tdm_priv).bck_id);
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
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;

    dev_dbg(
        (*cmpnt).dev,
        b"name %s, event 0x%x, dai_id %d, mclk %d\n\0".as_ptr() as *const c_char,
        (*w).name,
        event,
        dai_id,
        (*tdm_priv).mclk_rate,
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8189_mck_enable(afe, (*tdm_priv).mclk_id, (*tdm_priv).mclk_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*tdm_priv).mclk_rate = 0;
            mt8189_mck_disable(afe, (*tdm_priv).mclk_id);
        }
        _ => {}
    }

    0
}

static mtk_dai_tdm_widgets: [snd_soc_dapm_widget; 6] = [
    /* SND_SOC_DAPM_DEMUX("TDM Playback Route", SND_SOC_NOPM, 0, 0, &tdm_out_demux_control) */
    snd_soc_dapm_widget {
        name: b"TDM Playback Route\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
    /* SND_SOC_DAPM_SUPPLY_S("TDM_BCK", SUPPLY_SEQ_TDM_BCK_EN, SND_SOC_NOPM, 0, 0, mtk_tdm_bck_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
    snd_soc_dapm_widget {
        name: b"TDM_BCK\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
    /* SND_SOC_DAPM_SUPPLY_S("TDM_MCK", SUPPLY_SEQ_TDM_MCK_EN, SND_SOC_NOPM, 0, 0, mtk_tdm_mck_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
    snd_soc_dapm_widget {
        name: b"TDM_MCK\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
    /* SND_SOC_DAPM_SUPPLY_S("TDM_DPTX_BCK", SUPPLY_SEQ_TDM_DPTX_BCK_EN, SND_SOC_NOPM, 0, 0, mtk_tdm_bck_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
    snd_soc_dapm_widget {
        name: b"TDM_DPTX_BCK\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
    /* SND_SOC_DAPM_SUPPLY_S("TDM_DPTX_MCK", SUPPLY_SEQ_TDM_DPTX_MCK_EN, SND_SOC_NOPM, 0, 0, mtk_tdm_mck_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD) */
    snd_soc_dapm_widget {
        name: b"TDM_DPTX_MCK\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
    /* SND_SOC_DAPM_SUPPLY_S("TDM_CG", SUPPLY_SEQ_TDM_CG_EN, AUDIO_TOP_CON2, PDN_TDM_OUT_SFT, 1, NULL, 0) */
    snd_soc_dapm_widget {
        name: b"TDM_CG\0".as_ptr() as *const c_char,
        dapm: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn mtk_afe_tdm_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*sink).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*sink).name);
    let tdm_priv = (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_afe_tdm_priv;
    let cur_apll: c_int;

    /* which apll */
    cur_apll = mt8189_get_apll_by_name(afe, (*source).name);

    if (*tdm_priv).mclk_apll == cur_apll {
        1
    } else {
        0
    }
}

static mtk_dai_tdm_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
    snd_soc_dapm_route { _private: [] },
];

/* dai ops */
unsafe fn mtk_dai_tdm_cal_mclk(
    afe: *mut mtk_base_afe,
    tdm_priv: *mut mtk_afe_tdm_priv,
    freq: c_int,
) -> c_int {
    let apll: c_int;
    let apll_rate: c_int;

    apll = mt8189_get_apll_by_rate(afe, freq);
    apll_rate = mt8189_get_apll_rate(afe, apll);

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
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let tdm_id = (*dai).id;
    let tdm_priv: *mut mtk_afe_tdm_priv;
    let rate = params_rate(params);
    let channels = params_channels(params);
    let format = params_format(params);
    let mut tdm_con: c_uint;

    if tdm_id >= MT8189_DAI_NUM || tdm_id < 0 {
        return -EINVAL;
    }

    tdm_priv = (*afe_priv).dai_priv[tdm_id as usize] as *mut mtk_afe_tdm_priv;

    /* calculate mclk_rate, if not set explicitly */
    if (*tdm_priv).mclk_rate == 0 {
        (*tdm_priv).mclk_rate = (rate as c_int) * (*tdm_priv).mclk_multiple;
        mtk_dai_tdm_cal_mclk(afe, tdm_priv, (*tdm_priv).mclk_rate);
    }

    /* calculate bck */
    (*tdm_priv).bck_rate =
        (rate as c_int) * (channels as c_int) * snd_pcm_format_physical_width(format);

    if (*tdm_priv).bck_rate > (*tdm_priv).mclk_rate {
        return -EINVAL;
    }

    if (*tdm_priv).mclk_rate % (*tdm_priv).bck_rate != 0 {
        return -EINVAL;
    }

    dev_dbg(
        (*afe).dev,
        b"id %d, rate %d, ch %d, fmt %d, mclk %d, bck %d\n\0".as_ptr() as *const c_char,
        tdm_id,
        rate,
        channels,
        format,
        (*tdm_priv).mclk_rate,
        (*tdm_priv).bck_rate,
    );

    /* set tdm */
    tdm_con = 1u32 << LEFT_ALIGN_SFT;
    tdm_con |= get_tdm_wlen(format) << WLEN_SFT;
    tdm_con |= get_tdm_ch(channels) << CHANNEL_NUM_SFT;
    tdm_con |= get_tdm_channel_bck(format) << CHANNEL_BCK_CYCLES_SFT;
    tdm_con |= get_tdm_lrck_width(format) << LRCK_TDM_WIDTH_SFT;
    regmap_write((*afe).regmap, AFE_TDM_CON1, tdm_con);

    /* set dptx */
    if tdm_id == MT8189_DAI_TDM_DPTX {
        regmap_update_bits(
            (*afe).regmap,
            AFE_DPTX_CON,
            DPTX_CHANNEL_ENABLE_MASK_SFT,
            get_dptx_ch_enable_mask(channels) << DPTX_CHANNEL_ENABLE_SFT,
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
    let afe = snd_soc_dai_get_drvdata(dai);
    let tdm_id = (*dai).id;

    dev_dbg(
        (*afe).dev,
        b"%s(), cmd %d, tdm_id %d\n\0".as_ptr() as *const c_char,
        b"mtk_dai_tdm_trigger\0".as_ptr() as *const c_char,
        cmd,
        tdm_id,
    );

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
            if tdm_id == MT8189_DAI_TDM_DPTX {
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
            if tdm_id == MT8189_DAI_TDM_DPTX {
                regmap_update_bits((*afe).regmap, AFE_DPTX_CON, DPTX_ON_MASK_SFT, 0);
            }

            /* disable Out control */
            regmap_update_bits((*afe).regmap, AFE_HDMI_OUT_CON0, HDMI_OUT_ON_MASK_SFT, 0);
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
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;
    let tdm_priv: *mut mtk_afe_tdm_priv;

    if (*dai).id >= MT8189_DAI_NUM || (*dai).id < 0 {
        return -EINVAL;
    }

    tdm_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_afe_tdm_priv;

    if tdm_priv.is_null() {
        return -EINVAL;
    }

    if dir != SND_SOC_CLOCK_OUT {
        return -EINVAL;
    }

    dev_dbg(
        (*afe).dev,
        b"%s(), freq %d\n\0".as_ptr() as *const c_char,
        b"mtk_dai_tdm_set_sysclk\0".as_ptr() as *const c_char,
        freq,
    );

    mtk_dai_tdm_cal_mclk(afe, tdm_priv, freq as c_int)
}

static mtk_dai_tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_tdm_hw_params),
    trigger: Some(mtk_dai_tdm_trigger),
    set_sysclk: Some(mtk_dai_tdm_set_sysclk),
};

/* dai driver */
unsafe fn MTK_TDM_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
}

unsafe fn MTK_TDM_FORMATS() -> c_ulong {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static mut mtk_dai_tdm_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"TDM\0".as_ptr() as *const c_char,
        id: 0, /* MT8189_DAI_TDM */
        playback: snd_soc_pcm_stream {
            stream_name: b"TDM\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: 0,   /* MTK_TDM_RATES */
            formats: 0, /* MTK_TDM_FORMATS */
        },
        ops: &mtk_dai_tdm_ops,
    },
    snd_soc_dai_driver {
        name: b"TDM_DPTX\0".as_ptr() as *const c_char,
        id: 0, /* MT8189_DAI_TDM_DPTX */
        playback: snd_soc_pcm_stream {
            stream_name: b"TDM_DPTX\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: 0,   /* MTK_TDM_RATES */
            formats: 0, /* MTK_TDM_FORMATS */
        },
        ops: &mtk_dai_tdm_ops,
    },
];

unsafe fn init_tdm_priv_data(afe: *mut mtk_base_afe, id: c_int) -> *mut mtk_afe_tdm_priv {
    let tdm_priv: *mut mtk_afe_tdm_priv;

    tdm_priv = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_afe_tdm_priv>(),
        GFP_KERNEL,
    ) as *mut mtk_afe_tdm_priv;
    if tdm_priv.is_null() {
        return core::ptr::null_mut();
    }

    if id == MT8189_DAI_TDM_DPTX {
        (*tdm_priv).mclk_multiple = 256;
    } else {
        (*tdm_priv).mclk_multiple = 128;
    }

    (*tdm_priv).bck_id = MT8189_TDMOUT_BCK;
    (*tdm_priv).mclk_id = MT8189_TDMOUT_MCK;

    tdm_priv
}

#[no_mangle]
pub unsafe extern "C" fn mt8189_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let tdm_priv: *mut mtk_afe_tdm_priv;
    let tdm_dptx_priv: *mut mtk_afe_tdm_priv;
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    (*dai).dai_drivers = mtk_dai_tdm_driver.as_mut_ptr();
    (*dai).num_dai_drivers =
        (core::mem::size_of_val(&mtk_dai_tdm_driver) / core::mem::size_of::<snd_soc_dai_driver>())
            as c_int;
    (*dai).controls = mtk_dai_tdm_controls.as_ptr();
    (*dai).num_controls =
        (core::mem::size_of_val(&mtk_dai_tdm_controls) / core::mem::size_of::<snd_kcontrol_new>())
            as c_int;
    (*dai).dapm_widgets = mtk_dai_tdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = (core::mem::size_of_val(&mtk_dai_tdm_widgets)
        / core::mem::size_of::<snd_soc_dapm_widget>()) as c_int;
    (*dai).dapm_routes = mtk_dai_tdm_routes.as_ptr();
    (*dai).num_dapm_routes = (core::mem::size_of_val(&mtk_dai_tdm_routes)
        / core::mem::size_of::<snd_soc_dapm_route>()) as c_int;

    tdm_priv = init_tdm_priv_data(afe, MT8189_DAI_TDM);
    if tdm_priv.is_null() {
        return -ENOMEM;
    }

    tdm_dptx_priv = init_tdm_priv_data(afe, MT8189_DAI_TDM_DPTX);
    if tdm_dptx_priv.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*afe_priv).dai_priv[MT8189_DAI_TDM as usize] = tdm_priv as *mut c_void;
    (*afe_priv).dai_priv[MT8189_DAI_TDM_DPTX as usize] = tdm_dptx_priv as *mut c_void;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
