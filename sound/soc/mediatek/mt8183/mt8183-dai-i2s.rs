// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Translated from mt8183-dai-i2s.c. Symbols originally provided by Linux,
// ALSA SoC, and local MediaTek headers are intentionally referenced as
// external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const I2S_FMT_EIAJ: c_int = 0;
const I2S_FMT_I2S: c_int = 1;

const I2S_WLEN_16_BIT: c_int = 0;
const I2S_WLEN_32_BIT: c_int = 1;

const I2S_HD_NORMAL: c_int = 0;
const I2S_HD_LOW_JITTER: c_int = 1;

const I2S1_SEL_O28_O29: c_int = 0;
const I2S1_SEL_O03_O04: c_int = 1;

const I2S_IN_PAD_CONNSYS: c_int = 0;
const I2S_IN_PAD_IO_MUX: c_int = 1;

#[repr(C)]
pub struct mtk_afe_i2s_priv {
    pub id: c_int,
    pub rate: c_int, /* for determine which apll to use */
    pub low_jitter_en: c_int,
    pub share_i2s_id: c_int,
    pub mclk_id: c_int,
    pub mclk_rate: c_int,
    pub mclk_apll: c_int,
    pub use_eiaj: c_int,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut c_void,
    pub regmap: *mut c_void,
    pub platform_priv: *mut mt8183_afe_private,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mt8183_afe_private {
    pub dai_priv: [*mut mtk_afe_i2s_priv; 256],
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

pub type c_long = isize;
pub type snd_pcm_format_t = c_int;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn regmap_update_bits(regmap: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mt8183_apll1_enable(afe: *mut mtk_base_afe);
    fn mt8183_apll2_enable(afe: *mut mtk_base_afe);
    fn mt8183_apll1_disable(afe: *mut mtk_base_afe);
    fn mt8183_apll2_disable(afe: *mut mtk_base_afe);
    fn mt8183_mck_enable(afe: *mut mtk_base_afe, mclk_id: c_int, mclk_rate: c_int);
    fn mt8183_mck_disable(afe: *mut mtk_base_afe, mclk_id: c_int);
    fn mt8183_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8183_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_uint) -> c_int;
    fn mt8183_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn mt8183_rate_transform(dev: *mut c_void, rate: c_uint, id: c_int) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
}

extern "C" {
    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0x0001;
const SND_SOC_DAIFMT_I2S: c_uint = 0x0002;
const SND_SOC_NOPM: c_int = -1;

extern "C" {
    static MT8183_DAI_I2S_0: c_int;
    static MT8183_DAI_I2S_1: c_int;
    static MT8183_DAI_I2S_2: c_int;
    static MT8183_DAI_I2S_3: c_int;
    static MT8183_DAI_I2S_5: c_int;
    static MT8183_I2S0_MCK: c_int;
    static MT8183_I2S1_MCK: c_int;
    static MT8183_I2S2_MCK: c_int;
    static MT8183_I2S3_MCK: c_int;
    static MT8183_I2S5_MCK: c_int;
}

extern "C" {
    static AFE_DAC_CON1: c_uint;
    static AFE_I2S_CON: c_uint;
    static AFE_I2S_CON1: c_uint;
    static AFE_I2S_CON2: c_uint;
    static AFE_I2S_CON3: c_uint;
    static AFE_I2S_CON4: c_uint;
}

extern "C" {
    static I2S_MODE_MASK_SFT: c_uint;
    static I2S_MODE_SFT: c_uint;
    static I2SIN_PAD_SEL_SFT: c_uint;
    static I2S_FMT_SFT: c_uint;
    static I2S_WLEN_SFT: c_uint;
    static I2S2_SEL_O03_O04_SFT: c_uint;
    static I2S2_OUT_MODE_SFT: c_uint;
    static I2S2_WLEN_SFT: c_uint;
    static I2S3_UPDATE_WORD_SFT: c_uint;
    static I2S3_OUT_MODE_SFT: c_uint;
    static I2S3_WLEN_SFT: c_uint;
    static I2S4_OUT_MODE_SFT: c_uint;
    static I2S4_WLEN_SFT: c_uint;
    static I2S5_OUT_MODE_SFT: c_uint;
    static I2S5_WLEN_SFT: c_uint;
}

extern "C" {
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
}

unsafe fn get_i2s_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        I2S_WLEN_16_BIT as c_uint
    } else {
        I2S_WLEN_32_BIT as c_uint
    }
}

const MTK_AFE_I2S0_KCONTROL_NAME: *const c_char = b"I2S0_HD_Mux\0".as_ptr() as *const c_char;
const MTK_AFE_I2S1_KCONTROL_NAME: *const c_char = b"I2S1_HD_Mux\0".as_ptr() as *const c_char;
const MTK_AFE_I2S2_KCONTROL_NAME: *const c_char = b"I2S2_HD_Mux\0".as_ptr() as *const c_char;
const MTK_AFE_I2S3_KCONTROL_NAME: *const c_char = b"I2S3_HD_Mux\0".as_ptr() as *const c_char;
const MTK_AFE_I2S5_KCONTROL_NAME: *const c_char = b"I2S5_HD_Mux\0".as_ptr() as *const c_char;

const I2S0_HD_EN_W_NAME: *const c_char = b"I2S0_HD_EN\0".as_ptr() as *const c_char;
const I2S1_HD_EN_W_NAME: *const c_char = b"I2S1_HD_EN\0".as_ptr() as *const c_char;
const I2S2_HD_EN_W_NAME: *const c_char = b"I2S2_HD_EN\0".as_ptr() as *const c_char;
const I2S3_HD_EN_W_NAME: *const c_char = b"I2S3_HD_EN\0".as_ptr() as *const c_char;
const I2S5_HD_EN_W_NAME: *const c_char = b"I2S5_HD_EN\0".as_ptr() as *const c_char;

const I2S0_MCLK_EN_W_NAME: *const c_char = b"I2S0_MCLK_EN\0".as_ptr() as *const c_char;
const I2S1_MCLK_EN_W_NAME: *const c_char = b"I2S1_MCLK_EN\0".as_ptr() as *const c_char;
const I2S2_MCLK_EN_W_NAME: *const c_char = b"I2S2_MCLK_EN\0".as_ptr() as *const c_char;
const I2S3_MCLK_EN_W_NAME: *const c_char = b"I2S3_MCLK_EN\0".as_ptr() as *const c_char;
const I2S5_MCLK_EN_W_NAME: *const c_char = b"I2S5_MCLK_EN\0".as_ptr() as *const c_char;

unsafe fn get_i2s_id_by_name(_afe: *mut mtk_base_afe, name: *const c_char) -> c_int {
    if strncmp(name, b"I2S0\0".as_ptr() as *const c_char, 4) == 0 {
        MT8183_DAI_I2S_0
    } else if strncmp(name, b"I2S1\0".as_ptr() as *const c_char, 4) == 0 {
        MT8183_DAI_I2S_1
    } else if strncmp(name, b"I2S2\0".as_ptr() as *const c_char, 4) == 0 {
        MT8183_DAI_I2S_2
    } else if strncmp(name, b"I2S3\0".as_ptr() as *const c_char, 4) == 0 {
        MT8183_DAI_I2S_3
    } else if strncmp(name, b"I2S5\0".as_ptr() as *const c_char, 4) == 0 {
        MT8183_DAI_I2S_5
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

/* low jitter control */
static mt8183_i2s_hd_str: [*const c_char; 2] = [
    b"Normal\0".as_ptr() as *const c_char,
    b"Low_Jitter\0".as_ptr() as *const c_char,
];

// SOC_ENUM_SINGLE_EXT(ARRAY_SIZE(mt8183_i2s_hd_str), mt8183_i2s_hd_str)
static mt8183_i2s_enum: [soc_enum; 1] = [
    soc_enum { items: mt8183_i2s_hd_str.len() as c_uint },
];

unsafe extern "C" fn mt8183_i2s_hd_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mt8183_i2s_hd_get\0".as_ptr());
        return -EINVAL;
    }

    (*ucontrol).value.integer.value[0] = (*i2s_priv).low_jitter_en as c_long;

    0
}

unsafe extern "C" fn mt8183_i2s_hd_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let hd_en: c_int;
    let change: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    hd_en = (*ucontrol).value.integer.value[0] as c_int;

    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mt8183_i2s_hd_set\0".as_ptr());
        return -EINVAL;
    }

    change = ((*i2s_priv).low_jitter_en != hd_en) as c_int;
    (*i2s_priv).low_jitter_en = hd_en;

    change
}

macro_rules! route {
    ($sink:expr, $control:expr, $source:expr) => {
        snd_soc_dapm_route { sink: $sink, control: $control, source: $source, connected: None }
    };
    ($sink:expr, $control:expr, $source:expr, $connected:ident) => {
        snd_soc_dapm_route { sink: $sink, control: $control, source: $source, connected: Some($connected) }
    };
}

macro_rules! s {
    ($x:literal) => {
        concat!($x, "\0").as_ptr() as *const c_char
    };
}

// The following control and widget arrays correspond to SOC_* macro
// initializers in C. Their concrete fields are supplied by ALSA headers.
extern "Rust" {
    static mtk_dai_i2s_controls: [snd_kcontrol_new; 5];
    static mtk_i2s3_ch1_mix: [snd_kcontrol_new; 6];
    static mtk_i2s3_ch2_mix: [snd_kcontrol_new; 8];
    static mtk_i2s1_ch1_mix: [snd_kcontrol_new; 6];
    static mtk_i2s1_ch2_mix: [snd_kcontrol_new; 8];
    static mtk_i2s5_ch1_mix: [snd_kcontrol_new; 6];
    static mtk_i2s5_ch2_mix: [snd_kcontrol_new; 8];
    static mtk_dai_i2s_widgets: [snd_soc_dapm_widget; 23];
}

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_I2S_MCLK_EN: c_int = 1;
const SUPPLY_SEQ_I2S_HD_EN: c_int = 2;
const SUPPLY_SEQ_I2S_EN: c_int = 3;

unsafe extern "C" fn mtk_apll_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 {
                mt8183_apll1_enable(afe);
            } else {
                mt8183_apll2_enable(afe);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 {
                mt8183_apll1_disable(afe);
            } else {
                mt8183_apll2_disable(afe);
            }
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
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_mclk_en_event\0".as_ptr());
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8183_mck_enable(afe, (*i2s_priv).mclk_id, (*i2s_priv).mclk_rate);
        }
        SND_SOC_DAPM_POST_PMD => {
            (*i2s_priv).mclk_rate = 0;
            mt8183_mck_disable(afe, (*i2s_priv).mclk_id);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_afe_i2s_share_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_afe_i2s_share_connect\0".as_ptr());
        return 0;
    }

    if (*i2s_priv).share_i2s_id < 0 {
        return 0;
    }

    ((*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name)) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_hd_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_afe_i2s_hd_connect\0".as_ptr());
        return 0;
    }

    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) {
        return (*i2s_priv).low_jitter_en;
    }

    /* check if share i2s need hd en */
    if (*i2s_priv).share_i2s_id < 0 {
        return 0;
    }

    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) {
        return (*i2s_priv).low_jitter_en;
    }

    0
}

unsafe extern "C" fn mtk_afe_i2s_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_afe_i2s_apll_connect\0".as_ptr());
        return 0;
    }

    /* which apll */
    let cur_apll = mt8183_get_apll_by_name(afe, (*source).name);

    /* choose APLL from i2s rate */
    let i2s_need_apll = mt8183_get_apll_by_rate(afe, (*i2s_priv).rate as c_uint);

    if i2s_need_apll == cur_apll { 1 } else { 0 }
}

unsafe extern "C" fn mtk_afe_i2s_mclk_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_afe_i2s_mclk_connect\0".as_ptr());
        return 0;
    }

    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) {
        return if (*i2s_priv).mclk_rate > 0 { 1 } else { 0 };
    }

    /* check if share i2s need mclk */
    if (*i2s_priv).share_i2s_id < 0 {
        return 0;
    }

    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) {
        return if (*i2s_priv).mclk_rate > 0 { 1 } else { 0 };
    }

    0
}

unsafe extern "C" fn mtk_afe_mclk_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_afe_mclk_apll_connect\0".as_ptr());
        return 0;
    }

    /* which apll */
    let cur_apll = mt8183_get_apll_by_name(afe, (*source).name);

    if (*i2s_priv).mclk_apll == cur_apll { 1 } else { 0 }
}

static mtk_dai_i2s_routes: [snd_soc_dapm_route; 103] = [
    /* i2s0 */
    route!(s!("I2S0"), ptr::null(), s!("I2S0_EN")),
    route!(s!("I2S0"), ptr::null(), s!("I2S1_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S0"), ptr::null(), s!("I2S2_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S0"), ptr::null(), s!("I2S3_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S0"), ptr::null(), s!("I2S5_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S0"), ptr::null(), I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S0"), ptr::null(), I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S0"), ptr::null(), I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S0"), ptr::null(), I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S0"), ptr::null(), I2S5_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(I2S0_HD_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_i2s_apll_connect),
    route!(I2S0_HD_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_i2s_apll_connect),
    route!(s!("I2S0"), ptr::null(), I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S0"), ptr::null(), I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S0"), ptr::null(), I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S0"), ptr::null(), I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S0"), ptr::null(), I2S5_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(I2S0_MCLK_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_mclk_apll_connect),
    route!(I2S0_MCLK_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_mclk_apll_connect),
    /* i2s1 */
    route!(s!("I2S1_CH1"), s!("DL1_CH1"), s!("DL1")),
    route!(s!("I2S1_CH2"), s!("DL1_CH2"), s!("DL1")),
    route!(s!("I2S1_CH1"), s!("DL2_CH1"), s!("DL2")),
    route!(s!("I2S1_CH2"), s!("DL2_CH2"), s!("DL2")),
    route!(s!("I2S1_CH1"), s!("DL3_CH1"), s!("DL3")),
    route!(s!("I2S1_CH2"), s!("DL3_CH2"), s!("DL3")),
    route!(s!("I2S1"), ptr::null(), s!("I2S1_CH1")),
    route!(s!("I2S1"), ptr::null(), s!("I2S1_CH2")),
    route!(s!("I2S1"), ptr::null(), s!("I2S0_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S1"), ptr::null(), s!("I2S1_EN")),
    route!(s!("I2S1"), ptr::null(), s!("I2S2_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S1"), ptr::null(), s!("I2S3_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S1"), ptr::null(), s!("I2S5_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S1"), ptr::null(), I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S1"), ptr::null(), I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S1"), ptr::null(), I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S1"), ptr::null(), I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S1"), ptr::null(), I2S5_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(I2S1_HD_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_i2s_apll_connect),
    route!(I2S1_HD_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_i2s_apll_connect),
    route!(s!("I2S1"), ptr::null(), I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S1"), ptr::null(), I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S1"), ptr::null(), I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S1"), ptr::null(), I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S1"), ptr::null(), I2S5_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(I2S1_MCLK_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_mclk_apll_connect),
    route!(I2S1_MCLK_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_mclk_apll_connect),
    /* i2s2 */
    route!(s!("I2S2"), ptr::null(), s!("I2S0_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S2"), ptr::null(), s!("I2S1_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S2"), ptr::null(), s!("I2S2_EN")),
    route!(s!("I2S2"), ptr::null(), s!("I2S3_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S2"), ptr::null(), s!("I2S5_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S2"), ptr::null(), I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S2"), ptr::null(), I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S2"), ptr::null(), I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S2"), ptr::null(), I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S2"), ptr::null(), I2S5_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(I2S2_HD_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_i2s_apll_connect),
    route!(I2S2_HD_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_i2s_apll_connect),
    route!(s!("I2S2"), ptr::null(), I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S2"), ptr::null(), I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S2"), ptr::null(), I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S2"), ptr::null(), I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S2"), ptr::null(), I2S5_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(I2S2_MCLK_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_mclk_apll_connect),
    route!(I2S2_MCLK_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_mclk_apll_connect),
    /* i2s3 */
    route!(s!("I2S3_CH1"), s!("DL1_CH1"), s!("DL1")),
    route!(s!("I2S3_CH2"), s!("DL1_CH2"), s!("DL1")),
    route!(s!("I2S3_CH1"), s!("DL2_CH1"), s!("DL2")),
    route!(s!("I2S3_CH2"), s!("DL2_CH2"), s!("DL2")),
    route!(s!("I2S3_CH1"), s!("DL3_CH1"), s!("DL3")),
    route!(s!("I2S3_CH2"), s!("DL3_CH2"), s!("DL3")),
    route!(s!("I2S3"), ptr::null(), s!("I2S3_CH1")),
    route!(s!("I2S3"), ptr::null(), s!("I2S3_CH2")),
    route!(s!("I2S3"), ptr::null(), s!("I2S0_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S3"), ptr::null(), s!("I2S1_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S3"), ptr::null(), s!("I2S2_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S3"), ptr::null(), s!("I2S3_EN")),
    route!(s!("I2S3"), ptr::null(), s!("I2S5_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S3"), ptr::null(), I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S3"), ptr::null(), I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S3"), ptr::null(), I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S3"), ptr::null(), I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S3"), ptr::null(), I2S5_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(I2S3_HD_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_i2s_apll_connect),
    route!(I2S3_HD_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_i2s_apll_connect),
    route!(s!("I2S3"), ptr::null(), I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S3"), ptr::null(), I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S3"), ptr::null(), I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S3"), ptr::null(), I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S3"), ptr::null(), I2S5_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(I2S3_MCLK_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_mclk_apll_connect),
    route!(I2S3_MCLK_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_mclk_apll_connect),
    /* i2s5 */
    route!(s!("I2S5_CH1"), s!("DL1_CH1"), s!("DL1")),
    route!(s!("I2S5_CH2"), s!("DL1_CH2"), s!("DL1")),
    route!(s!("I2S5_CH1"), s!("DL2_CH1"), s!("DL2")),
    route!(s!("I2S5_CH2"), s!("DL2_CH2"), s!("DL2")),
    route!(s!("I2S5_CH1"), s!("DL3_CH1"), s!("DL3")),
    route!(s!("I2S5_CH2"), s!("DL3_CH2"), s!("DL3")),
    route!(s!("I2S5"), ptr::null(), s!("I2S5_CH1")),
    route!(s!("I2S5"), ptr::null(), s!("I2S5_CH2")),
    route!(s!("I2S5"), ptr::null(), s!("I2S0_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S5"), ptr::null(), s!("I2S1_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S5"), ptr::null(), s!("I2S2_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S5"), ptr::null(), s!("I2S3_EN"), mtk_afe_i2s_share_connect),
    route!(s!("I2S5"), ptr::null(), s!("I2S5_EN")),
    route!(s!("I2S5"), ptr::null(), I2S0_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S5"), ptr::null(), I2S1_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S5"), ptr::null(), I2S2_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S5"), ptr::null(), I2S3_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(s!("I2S5"), ptr::null(), I2S5_HD_EN_W_NAME, mtk_afe_i2s_hd_connect),
    route!(I2S5_HD_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_i2s_apll_connect),
    route!(I2S5_HD_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_i2s_apll_connect),
    route!(s!("I2S5"), ptr::null(), I2S0_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S5"), ptr::null(), I2S1_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S5"), ptr::null(), I2S2_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S5"), ptr::null(), I2S3_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(s!("I2S5"), ptr::null(), I2S5_MCLK_EN_W_NAME, mtk_afe_i2s_mclk_connect),
    route!(I2S5_MCLK_EN_W_NAME, ptr::null(), APLL1_W_NAME, mtk_afe_mclk_apll_connect),
    route!(I2S5_MCLK_EN_W_NAME, ptr::null(), APLL2_W_NAME, mtk_afe_mclk_apll_connect),
];

/* dai ops */
unsafe fn mtk_dai_i2s_config(
    afe: *mut mtk_base_afe,
    params: *mut snd_pcm_hw_params,
    i2s_id: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let i2s_priv = (*afe_priv).dai_priv[i2s_id as usize];

    let rate = params_rate(params);
    let rate_reg = mt8183_rate_transform((*afe).dev, rate, i2s_id);
    let format = params_format(params);
    let mut i2s_con: c_uint = 0;
    let mut fmt_con: c_uint = (I2S_FMT_I2S as c_uint) << I2S_FMT_SFT;
    let mut ret: c_int = 0;

    if !i2s_priv.is_null() {
        (*i2s_priv).rate = rate as c_int;

        if (*i2s_priv).use_eiaj != 0 {
            fmt_con = (I2S_FMT_EIAJ as c_uint) << I2S_FMT_SFT;
        }
    } else {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_dai_i2s_config\0".as_ptr());
    }

    if i2s_id == MT8183_DAI_I2S_0 {
        regmap_update_bits((*afe).regmap, AFE_DAC_CON1, I2S_MODE_MASK_SFT, rate_reg << I2S_MODE_SFT);
        i2s_con = (I2S_IN_PAD_IO_MUX as c_uint) << I2SIN_PAD_SEL_SFT;
        i2s_con |= fmt_con;
        i2s_con |= get_i2s_wlen(format) << I2S_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON, 0xffffeffe, i2s_con);
    } else if i2s_id == MT8183_DAI_I2S_1 {
        i2s_con = (I2S1_SEL_O28_O29 as c_uint) << I2S2_SEL_O03_O04_SFT;
        i2s_con |= rate_reg << I2S2_OUT_MODE_SFT;
        i2s_con |= fmt_con;
        i2s_con |= get_i2s_wlen(format) << I2S2_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0xffffeffe, i2s_con);
    } else if i2s_id == MT8183_DAI_I2S_2 {
        i2s_con = 8 << I2S3_UPDATE_WORD_SFT;
        i2s_con |= rate_reg << I2S3_OUT_MODE_SFT;
        i2s_con |= fmt_con;
        i2s_con |= get_i2s_wlen(format) << I2S3_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON2, 0xffffeffe, i2s_con);
    } else if i2s_id == MT8183_DAI_I2S_3 {
        i2s_con = rate_reg << I2S4_OUT_MODE_SFT;
        i2s_con |= fmt_con;
        i2s_con |= get_i2s_wlen(format) << I2S4_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON3, 0xffffeffe, i2s_con);
    } else if i2s_id == MT8183_DAI_I2S_5 {
        i2s_con = rate_reg << I2S5_OUT_MODE_SFT;
        i2s_con |= fmt_con;
        i2s_con |= get_i2s_wlen(format) << I2S5_WLEN_SFT;
        regmap_update_bits((*afe).regmap, AFE_I2S_CON4, 0xffffeffe, i2s_con);
    } else {
        dev_warn((*afe).dev, b"%s(), id %d not support\n\0".as_ptr() as *const c_char, b"mtk_dai_i2s_config\0".as_ptr(), i2s_id);
        return -EINVAL;
    }

    /* set share i2s */
    if !i2s_priv.is_null() && (*i2s_priv).share_i2s_id >= 0 {
        ret = mtk_dai_i2s_config(afe, params, (*i2s_priv).share_i2s_id);
    }

    ret
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

    if i2s_priv.is_null() {
        dev_warn((*afe).dev, b"%s(), i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_sysclk\0".as_ptr());
        return -EINVAL;
    }

    if dir != SND_SOC_CLOCK_OUT {
        dev_warn((*afe).dev, b"%s(), dir != SND_SOC_CLOCK_OUT\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_sysclk\0".as_ptr());
        return -EINVAL;
    }

    let apll = mt8183_get_apll_by_rate(afe, freq);
    let apll_rate = mt8183_get_apll_rate(afe, apll);

    if freq > apll_rate as c_uint {
        dev_warn((*afe).dev, b"%s(), freq > apll rate\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_sysclk\0".as_ptr());
        return -EINVAL;
    }

    if apll_rate % (freq as c_int) != 0 {
        dev_warn((*afe).dev, b"%s(), APLL cannot generate freq Hz\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_sysclk\0".as_ptr());
        return -EINVAL;
    }

    (*i2s_priv).mclk_rate = freq as c_int;
    (*i2s_priv).mclk_apll = apll;

    if (*i2s_priv).share_i2s_id > 0 {
        let share_i2s_priv = (*afe_priv).dai_priv[(*i2s_priv).share_i2s_id as usize];
        if share_i2s_priv.is_null() {
            dev_warn((*afe).dev, b"%s(), share_i2s_priv == NULL\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_sysclk\0".as_ptr());
            return -EINVAL;
        }

        (*share_i2s_priv).mclk_rate = (*i2s_priv).mclk_rate;
        (*share_i2s_priv).mclk_apll = (*i2s_priv).mclk_apll;
    }

    0
}

unsafe extern "C" fn mtk_dai_i2s_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let i2s_priv: *mut mtk_afe_i2s_priv;

    if (*dai).id == MT8183_DAI_I2S_0
        || (*dai).id == MT8183_DAI_I2S_1
        || (*dai).id == MT8183_DAI_I2S_2
        || (*dai).id == MT8183_DAI_I2S_3
        || (*dai).id == MT8183_DAI_I2S_5
    {
    } else {
        dev_warn((*afe).dev, b"%s(), id %d not support\n\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_fmt\0".as_ptr(), (*dai).id);
        return -EINVAL;
    }
    i2s_priv = (*afe_priv).dai_priv[(*dai).id as usize];

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J => {
            (*i2s_priv).use_eiaj = 1;
        }
        SND_SOC_DAIFMT_I2S => {
            (*i2s_priv).use_eiaj = 0;
        }
        _ => {
            dev_warn((*afe).dev, b"%s(), DAI format %d not support\n\0".as_ptr() as *const c_char, b"mtk_dai_i2s_set_fmt\0".as_ptr(), fmt & SND_SOC_DAIFMT_FORMAT_MASK);
            return -EINVAL;
        }
    }

    0
}

static mtk_dai_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_i2s_hw_params),
    set_sysclk: Some(mtk_dai_i2s_set_sysclk),
    set_fmt: Some(mtk_dai_i2s_set_fmt),
};

/* dai driver */
unsafe fn MTK_I2S_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
}

unsafe fn MTK_I2S_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

const EMPTY_STREAM: snd_soc_pcm_stream = snd_soc_pcm_stream {
    stream_name: ptr::null(),
    channels_min: 0,
    channels_max: 0,
    rates: 0,
    formats: 0,
};

static mut mtk_dai_i2s_driver: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver {
        name: s!("I2S0"),
        id: 0,
        playback: EMPTY_STREAM,
        capture: snd_soc_pcm_stream { stream_name: s!("I2S0"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        ops: &mtk_dai_i2s_ops,
    },
    snd_soc_dai_driver {
        name: s!("I2S1"),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: s!("I2S1"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        capture: EMPTY_STREAM,
        ops: &mtk_dai_i2s_ops,
    },
    snd_soc_dai_driver {
        name: s!("I2S2"),
        id: 0,
        playback: EMPTY_STREAM,
        capture: snd_soc_pcm_stream { stream_name: s!("I2S2"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        ops: &mtk_dai_i2s_ops,
    },
    snd_soc_dai_driver {
        name: s!("I2S3"),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: s!("I2S3"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        capture: EMPTY_STREAM,
        ops: &mtk_dai_i2s_ops,
    },
    snd_soc_dai_driver {
        name: s!("I2S5"),
        id: 0,
        playback: snd_soc_pcm_stream { stream_name: s!("I2S5"), channels_min: 1, channels_max: 2, rates: 0, formats: 0 },
        capture: EMPTY_STREAM,
        ops: &mtk_dai_i2s_ops,
    },
];

/* this enum is merely for mtk_afe_i2s_priv declare */
const DAI_I2S0: usize = 0;
const DAI_I2S1: usize = 1;
const DAI_I2S2: usize = 2;
const DAI_I2S3: usize = 3;
const DAI_I2S5: usize = 4;
const DAI_I2S_NUM: usize = 5;

static mut mt8183_i2s_priv: [mtk_afe_i2s_priv; DAI_I2S_NUM] = [
    mtk_afe_i2s_priv { id: 0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: 0, mclk_rate: 0, mclk_apll: 0, use_eiaj: 0 },
    mtk_afe_i2s_priv { id: 0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: 0, mclk_rate: 0, mclk_apll: 0, use_eiaj: 0 },
    mtk_afe_i2s_priv { id: 0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: 0, mclk_rate: 0, mclk_apll: 0, use_eiaj: 0 },
    mtk_afe_i2s_priv { id: 0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: 0, mclk_rate: 0, mclk_apll: 0, use_eiaj: 0 },
    mtk_afe_i2s_priv { id: 0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: 0, mclk_rate: 0, mclk_apll: 0, use_eiaj: 0 },
];

unsafe fn mt8183_i2s_priv_init_constants() {
    mt8183_i2s_priv[DAI_I2S0].id = MT8183_DAI_I2S_0;
    mt8183_i2s_priv[DAI_I2S0].mclk_id = MT8183_I2S0_MCK;
    mt8183_i2s_priv[DAI_I2S1].id = MT8183_DAI_I2S_1;
    mt8183_i2s_priv[DAI_I2S1].mclk_id = MT8183_I2S1_MCK;
    mt8183_i2s_priv[DAI_I2S2].id = MT8183_DAI_I2S_2;
    mt8183_i2s_priv[DAI_I2S2].mclk_id = MT8183_I2S2_MCK;
    mt8183_i2s_priv[DAI_I2S3].id = MT8183_DAI_I2S_3;
    mt8183_i2s_priv[DAI_I2S3].mclk_id = MT8183_I2S3_MCK;
    mt8183_i2s_priv[DAI_I2S5].id = MT8183_DAI_I2S_5;
    mt8183_i2s_priv[DAI_I2S5].mclk_id = MT8183_I2S5_MCK;
}

/**
 * mt8183_dai_i2s_set_share() - Set up I2S ports to share a single clock.
 * @afe: Pointer to &struct mtk_base_afe
 * @main_i2s_name: The name of the I2S port that will provide the clock
 * @secondary_i2s_name: The name of the I2S port that will use this clock
 */
#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_i2s_set_share(
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
// EXPORT_SYMBOL_GPL(mt8183_dai_i2s_set_share);

unsafe fn mt8183_dai_i2s_set_priv(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut i: usize = 0;

    mt8183_i2s_priv_init_constants();

    while i < DAI_I2S_NUM {
        let i2s_priv = devm_kzalloc(
            (*afe).dev,
            size_of::<mtk_afe_i2s_priv>(),
            GFP_KERNEL,
        ) as *mut mtk_afe_i2s_priv;
        if i2s_priv.is_null() {
            return -ENOMEM;
        }

        memcpy(
            i2s_priv as *mut c_void,
            &mt8183_i2s_priv[i] as *const mtk_afe_i2s_priv as *const c_void,
            size_of::<mtk_afe_i2s_priv>(),
        );

        (*afe_priv).dai_priv[mt8183_i2s_priv[i].id as usize] = i2s_priv;
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int {
    let dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_i2s_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_i2s_driver.len() as c_int;

    (*dai).controls = mtk_dai_i2s_controls.as_ptr();
    (*dai).num_controls = mtk_dai_i2s_controls.len() as c_int;
    (*dai).dapm_widgets = mtk_dai_i2s_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_i2s_widgets.len() as c_int;
    (*dai).dapm_routes = mtk_dai_i2s_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_i2s_routes.len() as c_int;

    /* set all dai i2s private data */
    mt8183_dai_i2s_set_priv(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
