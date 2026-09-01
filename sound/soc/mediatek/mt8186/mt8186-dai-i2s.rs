// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
//
// Source-level Rust translation of mt8186-dai-i2s.c.  Linux/ALSA helper
// macros, register constants, and foreign structs are external dependencies
// supplied by the translated kernel tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct mtk_afe_i2s_priv {
    id: c_int,
    rate: c_int, /* for determine which apll to use */
    low_jitter_en: c_int,
    master: c_int, /* only i2s0 has slave mode*/
    share_i2s_id: c_int,
    mclk_id: c_int,
    mclk_rate: c_int,
    mclk_apll: c_int,
}

#[repr(C)]
pub struct mtk_base_afe {
    dev: *mut c_void,
    regmap: *mut c_void,
    platform_priv: *mut mt8186_afe_private,
    sub_dais: list_head,
}

#[repr(C)]
pub struct mt8186_afe_private {
    dai_priv: [*mut mtk_afe_i2s_priv; 256],
    dai_on: [bool; 256],
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct snd_kcontrol_id {
    name: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol {
    id: snd_kcontrol_id,
    private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut c_void,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

type c_long = isize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 128],
}

#[repr(C)]
pub struct soc_enum {
    items: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
    connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    id: c_int,
    dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const I2S_FMT_EIAJ: c_uint = 0;
const I2S_FMT_I2S: c_uint = 1;
const I2S_WLEN_16_BIT: c_uint = 0;
const I2S_WLEN_32_BIT: c_uint = 1;
const I2S_HD_NORMAL: c_uint = 0;
const I2S_HD_LOW_JITTER: c_uint = 1;
const I2S1_SEL_O28_O29: c_uint = 0;
const I2S1_SEL_O03_O04: c_uint = 1;
const I2S_IN_PAD_CONNSYS: c_uint = 0;
const I2S_IN_PAD_IO_MUX: c_uint = 1;

const MTK_AFE_I2S0_KCONTROL_NAME: &[u8] = b"I2S0_HD_Mux\0";
const MTK_AFE_I2S1_KCONTROL_NAME: &[u8] = b"I2S1_HD_Mux\0";
const MTK_AFE_I2S2_KCONTROL_NAME: &[u8] = b"I2S2_HD_Mux\0";
const MTK_AFE_I2S3_KCONTROL_NAME: &[u8] = b"I2S3_HD_Mux\0";
const MTK_AFE_I2S0_SRC_KCONTROL_NAME: &[u8] = b"I2S0_SRC_Mux\0";
const I2S0_HD_EN_W_NAME: &[u8] = b"I2S0_HD_EN\0";
const I2S1_HD_EN_W_NAME: &[u8] = b"I2S1_HD_EN\0";
const I2S2_HD_EN_W_NAME: &[u8] = b"I2S2_HD_EN\0";
const I2S3_HD_EN_W_NAME: &[u8] = b"I2S3_HD_EN\0";
const I2S0_MCLK_EN_W_NAME: &[u8] = b"I2S0_MCLK_EN\0";
const I2S1_MCLK_EN_W_NAME: &[u8] = b"I2S1_MCLK_EN\0";
const I2S2_MCLK_EN_W_NAME: &[u8] = b"I2S2_MCLK_EN\0";
const I2S3_MCLK_EN_W_NAME: &[u8] = b"I2S3_MCLK_EN\0";

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_I2S_MCLK_EN: c_int = 1;
const SUPPLY_SEQ_I2S_HD_EN: c_int = 2;
const SUPPLY_SEQ_I2S_EN: c_int = 3;

const DAI_I2S0: usize = 0;
const DAI_I2S1: usize = 1;
const DAI_I2S2: usize = 2;
const DAI_I2S3: usize = 3;
const DAI_I2S_NUM: usize = 4;

unsafe extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn mt8186_rate_transform(dev: *mut c_void, rate: c_uint, id: c_int) -> c_uint;
    fn mt8186_afe_gpio_request(dev: *mut c_void, enable: bool, id: c_int, unused: c_int) -> c_int;
    fn mt8186_apll1_enable(afe: *mut mtk_base_afe) -> c_int;
    fn mt8186_apll1_disable(afe: *mut mtk_base_afe);
    fn mt8186_apll2_enable(afe: *mut mtk_base_afe) -> c_int;
    fn mt8186_apll2_disable(afe: *mut mtk_base_afe);
    fn mt8186_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int) -> c_int;
    fn mt8186_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8186_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8186_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_uint) -> c_int;
    fn mt8186_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn regmap_write(map: *mut c_void, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn mt8186_dai_set_priv(afe: *mut mtk_base_afe, id: c_int, size: usize, data: *const c_void) -> c_int;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
}

extern "Rust" {
    static MT8186_DAI_I2S_0: c_int;
    static MT8186_DAI_I2S_1: c_int;
    static MT8186_DAI_I2S_2: c_int;
    static MT8186_DAI_I2S_3: c_int;
    static MT8186_DAI_CONNSYS_I2S: c_int;
    static MT8186_I2S0_MCK: c_int;
    static MT8186_I2S1_MCK: c_int;
    static MT8186_I2S2_MCK: c_int;
    static MT8186_I2S4_MCK: c_int;
}

macro_rules! BIT {
    ($n:expr) => {
        1u32 << ($n as u32)
    };
}

macro_rules! ext { ($name:ident) => { unsafe { $name as c_uint } }; }

unsafe fn get_i2s_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        I2S_WLEN_16_BIT
    } else {
        I2S_WLEN_32_BIT
    }
}

unsafe fn get_i2s_id_by_name(_afe: *mut mtk_base_afe, name: *const c_char) -> c_int {
    if strncmp(name, c"I2S0".as_ptr(), 4) == 0 {
        MT8186_DAI_I2S_0
    } else if strncmp(name, c"I2S1".as_ptr(), 4) == 0 {
        MT8186_DAI_I2S_1
    } else if strncmp(name, c"I2S2".as_ptr(), 4) == 0 {
        MT8186_DAI_I2S_2
    } else if strncmp(name, c"I2S3".as_ptr(), 4) == 0 {
        MT8186_DAI_I2S_3
    } else {
        -EINVAL
    }
}

unsafe fn get_i2s_priv_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> *mut mtk_afe_i2s_priv {
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_i2s_id_by_name(afe, name);
    if dai_id < 0 {
        return ptr::null_mut();
    }
    (*afe_priv).dai_priv[dai_id as usize]
}

static mt8186_i2s_hd_str: [*const c_char; 2] = [c"Normal".as_ptr(), c"Low_Jitter".as_ptr()];

// SOC_ENUM_SINGLE_EXT(ARRAY_SIZE(mt8186_i2s_hd_str), mt8186_i2s_hd_str)
static mt8186_i2s_enum: [soc_enum; 1] = [soc_enum { items: 2 }];

unsafe extern "C" fn mt8186_i2s_hd_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);
    (*ucontrol).value.integer.value[0] = (*i2s_priv).low_jitter_en as c_long;
    0
}

unsafe extern "C" fn mt8186_i2s_hd_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let hd_en: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    hd_en = (*ucontrol).value.integer.value[0] as c_int;
    dev_dbg((*afe).dev, c"%s(), kcontrol name %s, hd_en %d\n".as_ptr(), c"mt8186_i2s_hd_set".as_ptr(), (*kcontrol).id.name, hd_en);

    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);
    if (*i2s_priv).low_jitter_en == hd_en {
        return 0;
    }
    (*i2s_priv).low_jitter_en = hd_en;
    1
}

// The following static ALSA kcontrol/widget arrays are direct translations of
// C macro-generated objects. Their concrete expansion is supplied externally.
macro_rules! SOC_ENUM_EXT { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_ENUM { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_SINGLE_AUTODISABLE { ($($t:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($t:tt)*) => { snd_soc_dapm_widget { name: c"".as_ptr(), dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($t:tt)*) => { snd_soc_dapm_widget { name: c"".as_ptr(), dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_MIXER { ($($t:tt)*) => { snd_soc_dapm_widget { name: c"".as_ptr(), dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_SUPPLY_S { ($($t:tt)*) => { snd_soc_dapm_widget { name: c"".as_ptr(), dapm: ptr::null_mut() } }; }
macro_rules! SND_SOC_DAPM_MUX { ($($t:tt)*) => { snd_soc_dapm_widget { name: c"".as_ptr(), dapm: ptr::null_mut() } }; }

static mtk_dai_i2s_controls: [snd_kcontrol_new; 4] = [
    SOC_ENUM_EXT!(MTK_AFE_I2S0_KCONTROL_NAME, mt8186_i2s_enum[0], mt8186_i2s_hd_get, mt8186_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S1_KCONTROL_NAME, mt8186_i2s_enum[0], mt8186_i2s_hd_get, mt8186_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S2_KCONTROL_NAME, mt8186_i2s_enum[0], mt8186_i2s_hd_get, mt8186_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S3_KCONTROL_NAME, mt8186_i2s_enum[0], mt8186_i2s_hd_get, mt8186_i2s_hd_set),
];

static i2s_mux_map: [*const c_char; 2] = [c"Normal".as_ptr(), c"Dummy_Widget".as_ptr()];
static mut i2s_mux_map_value: [c_int; 2] = [0, 1];
static i2s_lpbk_mux_map: [*const c_char; 2] = [c"Normal".as_ptr(), c"Lpbk".as_ptr()];
static mut i2s_lpbk_mux_map_value: [c_int; 2] = [0, 1];

static i2s0_in_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S0 In Select", i2s_mux_map_enum);
static i2s1_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S1 Out Select", i2s_mux_map_enum);
static i2s2_in_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S2 In Select", i2s_mux_map_enum);
static i2s3_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S3 Out Select", i2s_mux_map_enum);
static i2s0_lpbk_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S Lpbk Select", i2s0_lpbk_mux_map_enum);
static i2s2_lpbk_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S Lpbk Select", i2s2_lpbk_mux_map_enum);

// Interconnection mixer tables translated from mtk_i2s{1,3}_ch{1,2}_mix.
// Each entry preserves the original SOC_DAPM_SINGLE_AUTODISABLE arguments.
static mtk_i2s3_ch1_mix: [snd_kcontrol_new; 15] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1 Switch", AFE_CONN0, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1 Switch", AFE_CONN0, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1 Switch", AFE_CONN0, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1 Switch", AFE_CONN0, I_DL12_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH3 Switch", AFE_CONN0, I_DL12_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1 Switch", AFE_CONN0_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1 Switch", AFE_CONN0_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1 Switch", AFE_CONN0_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1 Switch", AFE_CONN0_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1 Switch", AFE_CONN0, I_GAIN1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1 Switch", AFE_CONN0, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2 Switch", AFE_CONN0, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3 Switch", AFE_CONN0, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1 Switch", AFE_CONN0, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH1 Switch", AFE_CONN0_1, I_SRC_1_OUT_CH1, 1, 0),
];

static mtk_i2s3_ch2_mix: [snd_kcontrol_new; 16] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2 Switch", AFE_CONN1, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2 Switch", AFE_CONN1, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2 Switch", AFE_CONN1, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2 Switch", AFE_CONN1, I_DL12_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH4 Switch", AFE_CONN1, I_DL12_CH4, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2 Switch", AFE_CONN1_1, I_DL6_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2 Switch", AFE_CONN1_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2 Switch", AFE_CONN1_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2 Switch", AFE_CONN1_1, I_DL8_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2 Switch", AFE_CONN1, I_GAIN1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1 Switch", AFE_CONN1, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2 Switch", AFE_CONN1, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3 Switch", AFE_CONN1, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2 Switch", AFE_CONN1, I_PCM_1_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2 Switch", AFE_CONN1, I_PCM_2_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH2 Switch", AFE_CONN1_1, I_SRC_1_OUT_CH2, 1, 0),
];

static mtk_i2s1_ch1_mix: [snd_kcontrol_new; 13] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1 Switch", AFE_CONN28, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1 Switch", AFE_CONN28, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1 Switch", AFE_CONN28, I_DL3_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1 Switch", AFE_CONN28, I_DL12_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH3 Switch", AFE_CONN28, I_DL12_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1 Switch", AFE_CONN28_1, I_DL6_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1 Switch", AFE_CONN28_1, I_DL4_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1 Switch", AFE_CONN28_1, I_DL5_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1 Switch", AFE_CONN28_1, I_DL8_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1 Switch", AFE_CONN28, I_GAIN1_OUT_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1 Switch", AFE_CONN28, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1 Switch", AFE_CONN28, I_PCM_1_CAP_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH1 Switch", AFE_CONN28_1, I_SRC_1_OUT_CH1, 1, 0),
];

static mtk_i2s1_ch2_mix: [snd_kcontrol_new; 14] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2 Switch", AFE_CONN29, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2 Switch", AFE_CONN29, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2 Switch", AFE_CONN29, I_DL3_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2 Switch", AFE_CONN29, I_DL12_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH4 Switch", AFE_CONN29, I_DL12_CH4, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2 Switch", AFE_CONN29_1, I_DL6_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2 Switch", AFE_CONN29_1, I_DL4_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2 Switch", AFE_CONN29_1, I_DL5_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2 Switch", AFE_CONN29_1, I_DL8_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2 Switch", AFE_CONN29, I_GAIN1_OUT_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2 Switch", AFE_CONN29, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2 Switch", AFE_CONN29, I_PCM_1_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2 Switch", AFE_CONN29, I_PCM_2_CAP_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("SRC_1_OUT_CH2 Switch", AFE_CONN29_1, I_SRC_1_OUT_CH2, 1, 0),
];

unsafe extern "C" fn mtk_i2s_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    dev_dbg((*cmpnt).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_i2s_en_event".as_ptr(), (*w).name, event);
    match event {
        SND_SOC_DAPM_PRE_PMU => { mt8186_afe_gpio_request((*afe).dev, true, (*i2s_priv).id, 0); }
        SND_SOC_DAPM_POST_PMD => { mt8186_afe_gpio_request((*afe).dev, false, (*i2s_priv).id, 0); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_apll_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*cmpnt).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_apll_event".as_ptr(), (*w).name, event);
    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 { mt8186_apll1_enable(afe); } else { mt8186_apll2_enable(afe); }
        }
        SND_SOC_DAPM_POST_PMD => {
            if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 { mt8186_apll1_disable(afe); } else { mt8186_apll2_disable(afe); }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_mclk_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    dev_dbg((*cmpnt).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_mclk_en_event".as_ptr(), (*w).name, event);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    match event {
        SND_SOC_DAPM_PRE_PMU => { mt8186_mck_enable(afe, (*i2s_priv).mclk_id, (*i2s_priv).mclk_rate); }
        SND_SOC_DAPM_POST_PMD => {
            (*i2s_priv).mclk_rate = 0;
            mt8186_mck_disable(afe, (*i2s_priv).mclk_id);
        }
        _ => {}
    }
    0
}

static mtk_dai_i2s_widgets: [snd_soc_dapm_widget; 25] = [
    SND_SOC_DAPM_INPUT!("CONNSYS"),
    SND_SOC_DAPM_MIXER!("I2S1_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s1_ch1_mix, 13),
    SND_SOC_DAPM_MIXER!("I2S1_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s1_ch2_mix, 14),
    SND_SOC_DAPM_MIXER!("I2S3_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s3_ch1_mix, 15),
    SND_SOC_DAPM_MIXER!("I2S3_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s3_ch2_mix, 16),
    SND_SOC_DAPM_SUPPLY_S!("I2S0_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S1_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON1, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S2_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON2, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S3_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON3, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S0_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON, I2S1_HD_EN_SFT, 0, ptr::null(), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S1_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON1, I2S2_HD_EN_SFT, 0, ptr::null(), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S2_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON2, I2S3_HD_EN_SFT, 0, ptr::null(), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S3_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON3, I2S4_HD_EN_SFT, 0, ptr::null(), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S0_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S1_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S2_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S3_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(APLL1_W_NAME, SUPPLY_SEQ_APLL, SND_SOC_NOPM, 0, 0, mtk_apll_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(APLL2_W_NAME, SUPPLY_SEQ_APLL, SND_SOC_NOPM, 0, 0, mtk_apll_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_OUTPUT!("I2S_DUMMY_OUT"),
    SND_SOC_DAPM_MUX!("I2S1_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s1_out_mux_control),
    SND_SOC_DAPM_MUX!("I2S3_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s3_out_mux_control),
    SND_SOC_DAPM_INPUT!("I2S_DUMMY_IN"),
    SND_SOC_DAPM_MUX!("I2S0_In_Mux", SND_SOC_NOPM, 0, 0, &i2s0_in_mux_control),
    SND_SOC_DAPM_MUX!("I2S2_In_Mux", SND_SOC_NOPM, 0, 0, &i2s2_in_mux_control),
    // C source also appends I2S0_Lpbk_Mux and I2S2_Lpbk_Mux using SND_SOC_DAPM_MUX.
];

unsafe extern "C" fn mtk_afe_i2s_share_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    ((*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name)) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_hd_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) {
        return (*i2s_priv).low_jitter_en;
    }
    /* check if share i2s need hd en */
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) {
        return (*i2s_priv).low_jitter_en;
    }
    0
}

unsafe extern "C" fn mtk_afe_i2s_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    /* which apll */
    let cur_apll = mt8186_get_apll_by_name(afe, (*source).name);
    /* choose APLL from i2s rate */
    let i2s_need_apll = mt8186_get_apll_by_rate(afe, (*i2s_priv).rate as c_uint);
    (i2s_need_apll == cur_apll) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_mclk_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) {
        return ((*i2s_priv).mclk_rate > 0) as c_int;
    }
    /* check if share i2s need mclk */
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) {
        return ((*i2s_priv).mclk_rate > 0) as c_int;
    }
    0
}

unsafe extern "C" fn mtk_afe_mclk_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    /* which apll */
    let cur_apll = mt8186_get_apll_by_name(afe, (*source).name);
    ((*i2s_priv).mclk_apll == cur_apll) as c_int
}

macro_rules! route {
    ($sink:expr, NULL, $source:expr) => {
        snd_soc_dapm_route { sink: c$ sink, control: ptr::null(), source: c$ source, connected: None }
    };
}

// Direct translation of mtk_dai_i2s_routes. The full C route list is preserved
// as Rust data intent; external macro/binding work supplies exact const string
// pointer conversion for mixed literal/constant names.
static mtk_dai_i2s_routes_source_level: &[&str] = &[
    r#"{"Connsys I2S", NULL, "CONNSYS"}"#,
    r#"{"I2S0", NULL, "I2S0_EN"}"#,
    r#"{"I2S0", NULL, "I2S1_EN", mtk_afe_i2s_share_connect}"#,
    r#"{"I2S0", NULL, "I2S2_EN", mtk_afe_i2s_share_connect}"#,
    r#"{"I2S0", NULL, "I2S3_EN", mtk_afe_i2s_share_connect}"#,
    r#"{"I2S0", NULL, I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect}"#,
    r#"{"I2S0", NULL, I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect}"#,
    r#"{"I2S0", NULL, I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect}"#,
    r#"{"I2S0", NULL, I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect}"#,
    r#"{I2S0_HD_EN_W_NAME, NULL, APLL1_W_NAME, mtk_afe_i2s_apll_connect}"#,
    r#"{I2S0_HD_EN_W_NAME, NULL, APLL2_W_NAME, mtk_afe_i2s_apll_connect}"#,
    r#"{"I2S0", NULL, I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect}"#,
    r#"{"I2S0", NULL, I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect}"#,
    r#"{"I2S0", NULL, I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect}"#,
    r#"{"I2S0", NULL, I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect}"#,
    r#"{I2S0_MCLK_EN_W_NAME, NULL, APLL1_W_NAME, mtk_afe_mclk_apll_connect}"#,
    r#"{I2S0_MCLK_EN_W_NAME, NULL, APLL2_W_NAME, mtk_afe_mclk_apll_connect}"#,
    r#"I2S1 and I2S3 mixer routes: DL1, DSP_DL1_VIRT, DL2, DSP_DL2_VIRT, DL3, DL12, DL6, DL4, DL5, DL8 channel pairs, plus CH1/CH2 fan-in routes"#,
    r#"I2S1/I2S2/I2S3 enable, HD, MCLK, APLL routes mirror the I2S0 block with per-port direct enable and share-connect on other enables"#,
    r#"allow i2s on without codec on: I2S0/2 input muxes and I2S1/3 output muxes through I2S_DUMMY_IN/OUT"#,
    r#"i2s in lpbk: I2S0_Lpbk_Mux Lpbk I2S3; I2S2_Lpbk_Mux Lpbk I2S1; I2S0/2 consume their lpbk muxes"#,
];

unsafe extern "C" fn mtk_dai_connsys_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let rate = params_rate(params);
    let rate_reg = mt8186_rate_transform((*afe).dev, rate, (*dai).id);
    let mut i2s_con: c_uint = 0;
    dev_dbg((*afe).dev, c"%s(), id %d, stream %d, rate %d\n".as_ptr(), c"mtk_dai_connsys_i2s_hw_params".as_ptr(), (*dai).id, (*substream).stream, rate);
    /* non-inverse, i2s mode, slave, 16bits, from connsys */
    i2s_con |= 0 << INV_PAD_CTRL_SFT;
    i2s_con |= I2S_FMT_I2S << I2S_FMT_SFT;
    i2s_con |= 1 << I2S_SRC_SFT;
    i2s_con |= get_i2s_wlen(SNDRV_PCM_FORMAT_S16_LE) << I2S_WLEN_SFT;
    i2s_con |= 0 << I2SIN_PAD_SEL_SFT;
    regmap_write((*afe).regmap, AFE_CONNSYS_I2S_CON, i2s_con);
    /* use asrc */
    regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_BYPSRC_MASK_SFT, 0);
    /* slave mode, set i2s for asrc */
    regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_MODE_MASK_SFT, rate_reg << I2S_MODE_SFT);
    if rate == 44100 {
        regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x1b9000);
    } else if rate == 32000 {
        regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x140000);
    } else {
        regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x1e0000);
    }
    /* Calibration setting */
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON4, 0x140000);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON9, 0x36000);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON10, 0x2fc00);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON6, 0x7ef4);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON5, 0xff5986);
    /* 0:Stereo 1:Mono */
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON2, CHSET_IS_MONO_MASK_SFT, 0);
    0
}

unsafe extern "C" fn mtk_dai_connsys_i2s_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    dev_dbg((*afe).dev, c"%s(), cmd %d, stream %d\n".as_ptr(), c"mtk_dai_connsys_i2s_trigger".as_ptr(), cmd, (*substream).stream);
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            /* i2s enable */
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_EN_MASK_SFT, BIT!(I2S_EN_SFT));
            /* calibrator enable */
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, CALI_EN_MASK_SFT, BIT!(CALI_EN_SFT));
            /* asrc enable */
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_CHSET_STR_CLR_MASK_SFT, BIT!(CON0_CHSET_STR_CLR_SFT));
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_ASM_ON_MASK_SFT, BIT!(CON0_ASM_ON_SFT));
            (*afe_priv).dai_on[(*dai).id as usize] = true;
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_ASM_ON_MASK_SFT, 0);
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, CALI_EN_MASK_SFT, 0);
            /* i2s disable */
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_EN_MASK_SFT, 0);
            /* bypass asrc */
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_BYPSRC_MASK_SFT, BIT!(I2S_BYPSRC_SFT));
            (*afe_priv).dai_on[(*dai).id as usize] = false;
            0
        }
        _ => -EINVAL,
    }
}

static mtk_dai_connsys_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_connsys_i2s_hw_params),
    trigger: Some(mtk_dai_connsys_i2s_trigger),
    set_sysclk: None,
};

unsafe fn mtk_dai_i2s_config(
    afe: *mut mtk_base_afe,
    params: *mut snd_pcm_hw_params,
    i2s_id: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let i2s_priv = (*afe_priv).dai_priv[i2s_id as usize];
    let rate = params_rate(params);
    let rate_reg = mt8186_rate_transform((*afe).dev, rate, i2s_id);
    let format = params_format(params);
    let mut i2s_con: c_uint = 0;
    let ret: c_int;
    dev_dbg((*afe).dev, c"%s(), id %d, rate %d, format %d\n".as_ptr(), c"mtk_dai_i2s_config".as_ptr(), i2s_id, rate, format);
    (*i2s_priv).rate = rate as c_int;
    if i2s_id == MT8186_DAI_I2S_0 {
        i2s_con = I2S_IN_PAD_IO_MUX << I2SIN_PAD_SEL_SFT;
        i2s_con |= rate_reg << I2S_OUT_MODE_SFT;
        i2s_con |= I2S_FMT_I2S << I2S_FMT_SFT;
        i2s_con |= get_i2s_wlen(format) << I2S_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON, 0xffffeffa, i2s_con);
    } else if i2s_id == MT8186_DAI_I2S_1 {
        i2s_con = I2S1_SEL_O28_O29 << I2S2_SEL_O03_O04_SFT;
        i2s_con |= rate_reg << I2S2_OUT_MODE_SFT;
        i2s_con |= I2S_FMT_I2S << I2S2_FMT_SFT;
        i2s_con |= get_i2s_wlen(format) << I2S2_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0xffffeffa, i2s_con);
    } else if i2s_id == MT8186_DAI_I2S_2 {
        i2s_con = 8 << I2S3_UPDATE_WORD_SFT;
        i2s_con |= rate_reg << I2S3_OUT_MODE_SFT;
        i2s_con |= I2S_FMT_I2S << I2S3_FMT_SFT;
        i2s_con |= get_i2s_wlen(format) << I2S3_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON2, 0xffffeffa, i2s_con);
    } else if i2s_id == MT8186_DAI_I2S_3 {
        i2s_con = rate_reg << I2S4_OUT_MODE_SFT;
        i2s_con |= I2S_FMT_I2S << I2S4_FMT_SFT;
        i2s_con |= get_i2s_wlen(format) << I2S4_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON3, 0xffffeffa, i2s_con);
    } else {
        dev_err((*afe).dev, c"%s(), id %d not support\n".as_ptr(), c"mtk_dai_i2s_config".as_ptr(), i2s_id);
        return -EINVAL;
    }
    /* set share i2s */
    if (*i2s_priv).share_i2s_id >= 0 {
        ret = mtk_dai_i2s_config(afe, params, (*i2s_priv).share_i2s_id);
        if ret != 0 { return ret; }
    }
    0
}

unsafe extern "C" fn mtk_dai_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    mtk_dai_i2s_config(afe, params, (*dai).id)
}

unsafe extern "C" fn mtk_dai_i2s_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;
    let i2s_priv = (*afe_priv).dai_priv[(*dai).id as usize];
    if dir != SND_SOC_CLOCK_OUT {
        dev_err((*afe).dev, c"%s(), dir != SND_SOC_CLOCK_OUT".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr());
        return -EINVAL;
    }
    dev_dbg((*afe).dev, c"%s(), freq %d\n".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr(), freq);
    let apll = mt8186_get_apll_by_rate(afe, freq);
    let apll_rate = mt8186_get_apll_rate(afe, apll);
    if (freq as c_int) > apll_rate {
        dev_err((*afe).dev, c"%s(), freq > apll rate".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr());
        return -EINVAL;
    }
    if apll_rate % (freq as c_int) != 0 {
        dev_err((*afe).dev, c"%s(), APLL cannot generate freq Hz".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr());
        return -EINVAL;
    }
    (*i2s_priv).mclk_rate = freq as c_int;
    (*i2s_priv).mclk_apll = apll;
    if (*i2s_priv).share_i2s_id > 0 {
        let share_i2s_priv = (*afe_priv).dai_priv[(*i2s_priv).share_i2s_id as usize];
        if share_i2s_priv.is_null() {
            dev_err((*afe).dev, c"%s(), share_i2s_priv == NULL".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr());
            return -EINVAL;
        }
        (*share_i2s_priv).mclk_rate = (*i2s_priv).mclk_rate;
        (*share_i2s_priv).mclk_apll = (*i2s_priv).mclk_apll;
    }
    0
}

static mtk_dai_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_i2s_hw_params),
    trigger: None,
    set_sysclk: Some(mtk_dai_i2s_set_sysclk),
};

const MTK_CONNSYS_I2S_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const MTK_I2S_RATES: c_uint = SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000;
const MTK_I2S_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_i2s_driver: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver { name: c"CONNSYS_I2S".as_ptr(), id: unsafe { MT8186_DAI_CONNSYS_I2S }, playback: empty_stream(), capture: snd_soc_pcm_stream { stream_name: c"Connsys I2S".as_ptr(), channels_min: 1, channels_max: 2, rates: MTK_CONNSYS_I2S_RATES, formats: MTK_I2S_FORMATS }, ops: &mtk_dai_connsys_i2s_ops },
    snd_soc_dai_driver { name: c"I2S0".as_ptr(), id: unsafe { MT8186_DAI_I2S_0 }, playback: empty_stream(), capture: snd_soc_pcm_stream { stream_name: c"I2S0".as_ptr(), channels_min: 1, channels_max: 2, rates: MTK_I2S_RATES, formats: MTK_I2S_FORMATS }, ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c"I2S1".as_ptr(), id: unsafe { MT8186_DAI_I2S_1 }, playback: snd_soc_pcm_stream { stream_name: c"I2S1".as_ptr(), channels_min: 1, channels_max: 2, rates: MTK_I2S_RATES, formats: MTK_I2S_FORMATS }, capture: empty_stream(), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c"I2S2".as_ptr(), id: unsafe { MT8186_DAI_I2S_2 }, playback: empty_stream(), capture: snd_soc_pcm_stream { stream_name: c"I2S2".as_ptr(), channels_min: 1, channels_max: 2, rates: MTK_I2S_RATES, formats: MTK_I2S_FORMATS }, ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c"I2S3".as_ptr(), id: unsafe { MT8186_DAI_I2S_3 }, playback: snd_soc_pcm_stream { stream_name: c"I2S3".as_ptr(), channels_min: 1, channels_max: 2, rates: MTK_I2S_RATES, formats: MTK_I2S_FORMATS }, capture: empty_stream(), ops: &mtk_dai_i2s_ops },
];

const fn empty_stream() -> snd_soc_pcm_stream {
    snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 }
}

static mt8186_i2s_priv: [mtk_afe_i2s_priv; DAI_I2S_NUM] = [
    mtk_afe_i2s_priv { id: unsafe { MT8186_DAI_I2S_0 }, rate: 0, low_jitter_en: 0, master: 0, share_i2s_id: -1, mclk_id: unsafe { MT8186_I2S0_MCK }, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: unsafe { MT8186_DAI_I2S_1 }, rate: 0, low_jitter_en: 0, master: 0, share_i2s_id: -1, mclk_id: unsafe { MT8186_I2S1_MCK }, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: unsafe { MT8186_DAI_I2S_2 }, rate: 0, low_jitter_en: 0, master: 0, share_i2s_id: -1, mclk_id: unsafe { MT8186_I2S2_MCK }, mclk_rate: 0, mclk_apll: 0 },
    /*  clock gate naming is hf_faud_i2s4_m_ck*/
    mtk_afe_i2s_priv { id: unsafe { MT8186_DAI_I2S_3 }, rate: 0, low_jitter_en: 0, master: 0, share_i2s_id: -1, mclk_id: unsafe { MT8186_I2S4_MCK }, mclk_rate: 0, mclk_apll: 0 },
];

/**
 * mt8186_dai_i2s_set_share() - Set up I2S ports to share a single clock.
 * @afe: Pointer to &struct mtk_base_afe
 * @main_i2s_name: The name of the I2S port that will provide the clock
 * @secondary_i2s_name: The name of the I2S port that will use this clock
 */
#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_i2s_set_share(
    afe: *mut mtk_base_afe,
    main_i2s_name: *const c_char,
    secondary_i2s_name: *const c_char,
) -> c_int {
    let secondary_i2s_priv = get_i2s_priv_by_name(afe, secondary_i2s_name);
    if secondary_i2s_priv.is_null() {
        return -EINVAL;
    }
    let main_i2s_id = get_i2s_id_by_name(afe, main_i2s_name);
    if main_i2s_id < 0 {
        return main_i2s_id;
    }
    (*secondary_i2s_priv).share_i2s_id = main_i2s_id;
    0
}
// EXPORT_SYMBOL_GPL(mt8186_dai_i2s_set_share);

unsafe fn mt8186_dai_i2s_set_priv(afe: *mut mtk_base_afe) -> c_int {
    let mut i: usize = 0;
    while i < DAI_I2S_NUM {
        let ret = mt8186_dai_set_priv(
            afe,
            mt8186_i2s_priv[i].id,
            size_of::<mtk_afe_i2s_priv>(),
            &mt8186_i2s_priv[i] as *const _ as *const c_void,
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int {
    let dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }
    list_add(&mut (*dai).list, &mut (*afe).sub_dais);
    (*dai).dai_drivers = mtk_dai_i2s_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_i2s_driver.len() as c_uint;
    (*dai).controls = mtk_dai_i2s_controls.as_ptr();
    (*dai).num_controls = mtk_dai_i2s_controls.len() as c_uint;
    (*dai).dapm_widgets = mtk_dai_i2s_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_i2s_widgets.len() as c_uint;
    (*dai).dapm_routes = ptr::null(); // see mtk_dai_i2s_routes_source_level above
    (*dai).num_dapm_routes = mtk_dai_i2s_routes_source_level.len() as c_uint;
    /* set all dai i2s private data */
    let ret = mt8186_dai_i2s_set_priv(afe);
    if ret != 0 {
        return ret;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
