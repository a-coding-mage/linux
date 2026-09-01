// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI TDM Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependencies: linux/regmap.h, sound/pcm_params.h,
// mt8186-afe-clk.h, mt8186-afe-common.h, mt8186-afe-gpio.h,
// mt8186-interconnection.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const TDM_HD_EN_W_NAME: &[u8] = b"TDM_HD_EN\0";
const TDM_MCLK_EN_W_NAME: &[u8] = b"TDM_MCLK_EN\0";
const MTK_AFE_TDM_KCONTROL_NAME: &[u8] = b"TDM_HD_Mux\0";

#[repr(C)]
pub struct mtk_afe_tdm_priv {
    pub id: c_uint,
    pub rate: c_uint, /* for determine which apll to use */
    pub bck_invert: c_uint,
    pub lck_invert: c_uint,
    pub lrck_width: c_uint,
    pub mclk_id: c_uint,
    pub mclk_multiple: c_uint, /* according to sample rate */
    pub mclk_rate: c_uint,
    pub mclk_apll: c_uint,
    pub tdm_mode: c_uint,
    pub data_mode: c_uint,
    pub slave_mode: c_uint,
    pub low_jitter_en: c_uint,
}

const TDM_IN_I2S: c_uint = 0;
const TDM_IN_LJ: c_uint = 1;
const TDM_IN_RJ: c_uint = 2;
const TDM_IN_DSP_A: c_uint = 4;
const TDM_IN_DSP_B: c_uint = 5;

const TDM_DATA_ONE_PIN: c_uint = 0;
const TDM_DATA_MULTI_PIN: c_uint = 1;

const TDM_BCK_NON_INV: c_uint = 0;
const TDM_BCK_INV: c_uint = 1;

const TDM_LCK_NON_INV: c_uint = 0;
const TDM_LCK_INV: c_uint = 1;

const SUPPLY_SEQ_APLL: c_uint = 0;
const SUPPLY_SEQ_TDM_MCK_EN: c_uint = 1;
const SUPPLY_SEQ_TDM_HD_EN: c_uint = 2;
const SUPPLY_SEQ_TDM_EN: c_uint = 3;

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
    pub id: snd_ctl_elem_id,
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: *const c_char,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
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
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
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
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub id: c_int,
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
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}
#[repr(C)]
pub struct soc_enum {
    pub items: c_uint,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut mt8186_afe_private,
    pub sub_dais: list_head,
}
#[repr(C)]
pub struct mt8186_afe_private {
    pub dai_priv: [*mut c_void; 0],
}
#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

type snd_pcm_format_t = c_int;
type c_long = i64;

extern "C" {
    static MT8186_DAI_TDM_IN: c_int;
    static MT8186_TDM_MCK: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static ETDM_IN1_CON0: c_uint;
    static ETDM_IN1_CON1: c_uint;
    static ETDM_IN1_CON2: c_uint;
    static ETDM_IN1_CON3: c_uint;
    static ETDM_IN1_CON4: c_uint;
    static ETDM_IN1_CON8: c_uint;
    static ETDM_IN_CON0_CTRL_MASK: c_uint;
    static ETDM_IN_CON1_CTRL_MASK: c_uint;
    static ETDM_IN_CON2_CTRL_MASK: c_uint;
    static ETDM_IN_CON3_CTRL_MASK: c_uint;
    static ETDM_IN_CON4_CTRL_MASK: c_uint;
    static ETDM_IN_CON8_CTRL_MASK: c_uint;
    static ETDM_IN1_CON0_REG_ETDM_IN_EN_SFT: c_uint;
    static ETDM_IN1_CON0_REG_SLAVE_MODE_SFT: c_uint;
    static ETDM_IN1_CON0_REG_FMT_SFT: c_uint;
    static ETDM_IN1_CON0_REG_BIT_LENGTH_SFT: c_uint;
    static ETDM_IN1_CON0_REG_WORD_LENGTH_SFT: c_uint;
    static ETDM_IN1_CON0_REG_CH_NUM_SFT: c_uint;
    static ETDM_IN1_CON0_REG_SYNC_MODE_SFT: c_uint;
    static ETDM_IN1_CON0_REG_RELATCH_1X_EN_SEL_DOMAIN_SFT: c_uint;
    static ETDM_IN1_CON1_REG_LRCK_AUTO_MODE_SFT: c_uint;
    static ETDM_IN1_CON1_PINMUX_MCLK_CTRL_OE_SFT: c_uint;
    static ETDM_IN1_CON1_REG_LRCK_WIDTH_SFT: c_uint;
    static ETDM_IN1_CON2_REG_CLOCK_SOURCE_SEL_SFT: c_uint;
    static ETDM_IN1_CON8_REG_ETDM_USE_AFIFO_SFT: c_uint;
    static ETDM_IN1_CON8_REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: c_uint;
    static ETDM_IN_CON4_CON0_SLAVE_LRCK_INV: c_uint;
    static ETDM_IN_CON4_CON0_SLAVE_BCK_INV: c_uint;
    static ETDM_IN_CON4_CON0_MASTER_LRCK_INV: c_uint;
    static ETDM_IN_CON4_CON0_MASTER_BCK_INV: c_uint;
    static ETDM_IN_CON2_MULTI_IP_2CH_MODE: c_uint;
    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;

    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn mt8186_afe_gpio_request(dev: *mut device, enable: bool, id: c_uint, flag: c_int) -> c_int;
    fn mt8186_mck_enable(afe: *mut mtk_base_afe, id: c_uint, rate: c_uint) -> c_int;
    fn mt8186_mck_disable(afe: *mut mtk_base_afe, id: c_uint);
    fn mt8186_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8186_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_uint) -> c_int;
    fn mt8186_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn mt8186_rate_transform(dev: *mut device, rate: c_uint, id: c_int) -> c_uint;
    fn mt8186_tdm_relatch_rate_transform(dev: *mut device, rate: c_uint) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn ETDM_IN_CON3_FS(rate: c_uint) -> c_uint;
    fn ETDM_IN_CON4_FS(rate: c_uint) -> c_uint;
    fn ETDM_IN_CON8_FS(rate: c_uint) -> c_uint;
    fn ETDM_IN_CON2_MULTI_IP_CH(channels: c_uint) -> c_uint;
}

#[inline]
unsafe fn dai_priv_tdm(afe_priv: *mut mt8186_afe_private, dai_id: c_int) -> *mut mtk_afe_tdm_priv {
    *(*afe_priv).dai_priv.as_mut_ptr().offset(dai_id as isize) as *mut mtk_afe_tdm_priv
}

unsafe fn get_tdm_lrck_width(format: snd_pcm_format_t, mode: c_uint) -> c_uint {
    if mode == TDM_IN_DSP_A || mode == TDM_IN_DSP_B {
        return 0;
    }

    (snd_pcm_format_physical_width(format) - 1) as c_uint
}

fn get_tdm_ch_fixup(channels: c_uint) -> c_uint {
    if channels > 4 {
        return 8;
    } else if channels > 2 {
        return 4;
    }

    2
}

fn get_tdm_ch_per_sdata(mode: c_uint, channels: c_uint) -> c_uint {
    if mode == TDM_IN_DSP_A || mode == TDM_IN_DSP_B {
        return get_tdm_ch_fixup(channels);
    }

    2
}

unsafe fn get_tdm_id_by_name(_name: *const c_char) -> c_int {
    MT8186_DAI_TDM_IN
}

unsafe extern "C" fn mtk_tdm_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    dev_dbg(
        (*cmpnt).dev,
        b"%s(), name %s, event 0x%x\n\0".as_ptr() as *const c_char,
        b"mtk_tdm_en_event\0".as_ptr() as *const c_char,
        (*w).name,
        event,
    );

    if event == SND_SOC_DAPM_PRE_PMU {
        mt8186_afe_gpio_request((*afe).dev, true, (*tdm_priv).id, 0);
    } else if event == SND_SOC_DAPM_POST_PMD {
        mt8186_afe_gpio_request((*afe).dev, false, (*tdm_priv).id, 0);
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
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    dev_dbg(
        (*cmpnt).dev,
        b"%s(), name %s, event 0x%x, dai_id %d\n\0".as_ptr() as *const c_char,
        b"mtk_tdm_mck_en_event\0".as_ptr() as *const c_char,
        (*w).name,
        event,
        dai_id,
    );

    if event == SND_SOC_DAPM_PRE_PMU {
        mt8186_mck_enable(afe, (*tdm_priv).mclk_id, (*tdm_priv).mclk_rate);
    } else if event == SND_SOC_DAPM_POST_PMD {
        (*tdm_priv).mclk_rate = 0;
        mt8186_mck_disable(afe, (*tdm_priv).mclk_id);
    }

    0
}

/* dai component */
/* tdm virtual mux to output widget */
static tdm_mux_map: [*const c_char; 2] = [
    b"Normal\0".as_ptr() as *const c_char,
    b"Dummy_Widget\0".as_ptr() as *const c_char,
];

static mut tdm_mux_map_value: [c_int; 2] = [0, 1];

// static SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL(tdm_mux_map_enum,
//                                               SND_SOC_NOPM, 0, 1,
//                                               tdm_mux_map,
//                                               tdm_mux_map_value);
extern "C" {
    static tdm_mux_map_enum: soc_enum;
}

// static const struct snd_kcontrol_new tdm_in_mux_control =
//      SOC_DAPM_ENUM("TDM In Select", tdm_mux_map_enum);
static tdm_in_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!(b"TDM In Select\0".as_ptr() as *const c_char, tdm_mux_map_enum);

static mtk_dai_tdm_widgets: [snd_soc_dapm_widget_desc; 6] = [
    SND_SOC_DAPM_CLOCK_SUPPLY!(b"aud_tdm_clk\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_SUPPLY_S!(
        b"TDM_EN\0".as_ptr() as *const c_char,
        SUPPLY_SEQ_TDM_EN,
        ETDM_IN1_CON0,
        ETDM_IN1_CON0_REG_ETDM_IN_EN_SFT,
        0,
        Some(mtk_tdm_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    /* tdm hd en */
    SND_SOC_DAPM_SUPPLY_S!(
        TDM_HD_EN_W_NAME.as_ptr() as *const c_char,
        SUPPLY_SEQ_TDM_HD_EN,
        ETDM_IN1_CON2,
        ETDM_IN1_CON2_REG_CLOCK_SOURCE_SEL_SFT,
        0,
        None,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        TDM_MCLK_EN_W_NAME.as_ptr() as *const c_char,
        SUPPLY_SEQ_TDM_MCK_EN,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_tdm_mck_en_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_INPUT!(b"TDM_DUMMY_IN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_MUX!(
        b"TDM_In_Mux\0".as_ptr() as *const c_char,
        SND_SOC_NOPM,
        0,
        0,
        &tdm_in_mux_control
    ),
];

unsafe extern "C" fn mtk_afe_tdm_mclk_connect(
    _source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    if (*tdm_priv).mclk_rate > 0 { 1 } else { 0 }
}

unsafe extern "C" fn mtk_afe_tdm_mclk_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    /* which apll */
    let cur_apll = mt8186_get_apll_by_name(afe, (*source).name);

    if (*tdm_priv).mclk_apll as c_int == cur_apll { 1 } else { 0 }
}

unsafe extern "C" fn mtk_afe_tdm_hd_connect(
    _source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    (*tdm_priv).low_jitter_en as c_int
}

unsafe extern "C" fn mtk_afe_tdm_apll_connect(
    source: *mut snd_soc_dapm_widget,
    sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*w).name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    /* which apll */
    let cur_apll = mt8186_get_apll_by_name(afe, (*source).name);

    /* choose APLL from tdm rate */
    let tdm_need_apll = mt8186_get_apll_by_rate(afe, (*tdm_priv).rate);

    if tdm_need_apll == cur_apll { 1 } else { 0 }
}

/* low jitter control */
static mt8186_tdm_hd_str: [*const c_char; 2] = [
    b"Normal\0".as_ptr() as *const c_char,
    b"Low_Jitter\0".as_ptr() as *const c_char,
];

static mt8186_tdm_enum: [soc_enum; 1] = [
    SOC_ENUM_SINGLE_EXT!(mt8186_tdm_hd_str.len(), mt8186_tdm_hd_str),
];

unsafe extern "C" fn mt8186_tdm_hd_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*kcontrol).id.name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);

    (*ucontrol).value.integer.value[0] = (*tdm_priv).low_jitter_en as c_long;

    0
}

unsafe extern "C" fn mt8186_tdm_hd_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let dai_id = get_tdm_id_by_name((*kcontrol).id.name);
    let tdm_priv = dai_priv_tdm(afe_priv, dai_id);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let hd_en: c_int;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    hd_en = (*ucontrol).value.integer.value[0] as c_int;

    dev_dbg(
        (*afe).dev,
        b"%s(), kcontrol name %s, hd_en %d\n\0".as_ptr() as *const c_char,
        b"mt8186_tdm_hd_set\0".as_ptr() as *const c_char,
        (*kcontrol).id.name,
        hd_en,
    );

    if (*tdm_priv).low_jitter_en == hd_en as c_uint {
        return 0;
    }

    (*tdm_priv).low_jitter_en = hd_en as c_uint;

    1
}

static mtk_dai_tdm_controls: [snd_kcontrol_new; 1] = [
    SOC_ENUM_EXT!(
        MTK_AFE_TDM_KCONTROL_NAME.as_ptr() as *const c_char,
        mt8186_tdm_enum[0],
        Some(mt8186_tdm_hd_get),
        Some(mt8186_tdm_hd_set)
    ),
];

static mtk_dai_tdm_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route { sink: b"TDM IN\0".as_ptr() as *const c_char, control: ptr::null(), source: b"aud_tdm_clk\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"TDM IN\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TDM_EN\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"TDM IN\0".as_ptr() as *const c_char, control: ptr::null(), source: TDM_HD_EN_W_NAME.as_ptr() as *const c_char, connected: Some(mtk_afe_tdm_hd_connect) },
    snd_soc_dapm_route { sink: TDM_HD_EN_W_NAME.as_ptr() as *const c_char, control: ptr::null(), source: unsafe { APLL1_W_NAME }, connected: Some(mtk_afe_tdm_apll_connect) },
    snd_soc_dapm_route { sink: TDM_HD_EN_W_NAME.as_ptr() as *const c_char, control: ptr::null(), source: unsafe { APLL2_W_NAME }, connected: Some(mtk_afe_tdm_apll_connect) },
    snd_soc_dapm_route { sink: b"TDM IN\0".as_ptr() as *const c_char, control: ptr::null(), source: TDM_MCLK_EN_W_NAME.as_ptr() as *const c_char, connected: Some(mtk_afe_tdm_mclk_connect) },
    snd_soc_dapm_route { sink: TDM_MCLK_EN_W_NAME.as_ptr() as *const c_char, control: ptr::null(), source: unsafe { APLL1_W_NAME }, connected: Some(mtk_afe_tdm_mclk_apll_connect) },
    snd_soc_dapm_route { sink: TDM_MCLK_EN_W_NAME.as_ptr() as *const c_char, control: ptr::null(), source: unsafe { APLL2_W_NAME }, connected: Some(mtk_afe_tdm_mclk_apll_connect) },
    /* allow tdm on without codec on */
    snd_soc_dapm_route { sink: b"TDM IN\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TDM_In_Mux\0".as_ptr() as *const c_char, connected: None },
    snd_soc_dapm_route { sink: b"TDM_In_Mux\0".as_ptr() as *const c_char, control: b"Dummy_Widget\0".as_ptr() as *const c_char, source: b"TDM_DUMMY_IN\0".as_ptr() as *const c_char, connected: None },
];

/* dai ops */
unsafe fn mtk_dai_tdm_cal_mclk(
    afe: *mut mtk_base_afe,
    tdm_priv: *mut mtk_afe_tdm_priv,
    freq: c_int,
) -> c_int {
    let apll = mt8186_get_apll_by_rate(afe, freq as c_uint);
    let apll_rate = mt8186_get_apll_rate(afe, apll);

    if freq == 0 || freq > apll_rate {
        dev_err(
            (*afe).dev,
            b"%s(), freq(%d Hz) invalid\n\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_cal_mclk\0".as_ptr() as *const c_char,
            freq,
        );
        return -EINVAL;
    }

    if apll_rate % freq != 0 {
        dev_err(
            (*afe).dev,
            b"%s(), APLL cannot generate %d Hz\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_cal_mclk\0".as_ptr() as *const c_char,
            freq,
        );
        return -EINVAL;
    }

    (*tdm_priv).mclk_rate = freq as c_uint;
    (*tdm_priv).mclk_apll = apll as c_uint;

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
    let tdm_priv = dai_priv_tdm(afe_priv, tdm_id);
    let tdm_mode = (*tdm_priv).tdm_mode;
    let data_mode = (*tdm_priv).data_mode;
    let rate = params_rate(params);
    let channels = params_channels(params);
    let format = params_format(params);
    let bit_width = snd_pcm_format_physical_width(format) as c_uint;
    let tdm_channels = if data_mode == TDM_DATA_ONE_PIN {
        get_tdm_ch_per_sdata(tdm_mode, channels)
    } else {
        2
    };
    let lrck_width = get_tdm_lrck_width(format, tdm_mode);
    let mut tdm_con: c_uint = 0;
    let slave_mode = (*tdm_priv).slave_mode != 0;
    let lrck_inv = (*tdm_priv).lck_invert != 0;
    let bck_inv = (*tdm_priv).bck_invert != 0;
    let tran_rate: c_uint;
    let tran_relatch_rate: c_uint;

    (*tdm_priv).rate = rate;
    tran_rate = mt8186_rate_transform((*afe).dev, rate, (*dai).id);
    tran_relatch_rate = mt8186_tdm_relatch_rate_transform((*afe).dev, rate);

    /* calculate mclk_rate, if not set explicitly */
    if (*tdm_priv).mclk_rate == 0 {
        (*tdm_priv).mclk_rate = rate.wrapping_mul((*tdm_priv).mclk_multiple);
        mtk_dai_tdm_cal_mclk(afe, tdm_priv, (*tdm_priv).mclk_rate as c_int);
    }

    /* ETDM_IN1_CON0 */
    tdm_con |= (slave_mode as c_uint) << ETDM_IN1_CON0_REG_SLAVE_MODE_SFT;
    tdm_con |= tdm_mode << ETDM_IN1_CON0_REG_FMT_SFT;
    tdm_con |= bit_width.wrapping_sub(1) << ETDM_IN1_CON0_REG_BIT_LENGTH_SFT;
    tdm_con |= bit_width.wrapping_sub(1) << ETDM_IN1_CON0_REG_WORD_LENGTH_SFT;
    tdm_con |= tdm_channels.wrapping_sub(1) << ETDM_IN1_CON0_REG_CH_NUM_SFT;
    /* need to disable sync mode otherwise this may cause latch data error */
    tdm_con |= 0 << ETDM_IN1_CON0_REG_SYNC_MODE_SFT;
    /* relatch 1x en clock fix to h26m */
    tdm_con |= 0 << ETDM_IN1_CON0_REG_RELATCH_1X_EN_SEL_DOMAIN_SFT;
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON0, ETDM_IN_CON0_CTRL_MASK, tdm_con);

    /* ETDM_IN1_CON1 */
    tdm_con = 0;
    tdm_con |= 0 << ETDM_IN1_CON1_REG_LRCK_AUTO_MODE_SFT;
    tdm_con |= 1 << ETDM_IN1_CON1_PINMUX_MCLK_CTRL_OE_SFT;
    tdm_con |= lrck_width.wrapping_sub(1) << ETDM_IN1_CON1_REG_LRCK_WIDTH_SFT;
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON1, ETDM_IN_CON1_CTRL_MASK, tdm_con);

    /* ETDM_IN1_CON3 */
    tdm_con = ETDM_IN_CON3_FS(tran_rate);
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON3, ETDM_IN_CON3_CTRL_MASK, tdm_con);

    /* ETDM_IN1_CON4 */
    tdm_con = ETDM_IN_CON4_FS(tran_relatch_rate);
    if slave_mode {
        if lrck_inv {
            tdm_con |= ETDM_IN_CON4_CON0_SLAVE_LRCK_INV;
        }
        if bck_inv {
            tdm_con |= ETDM_IN_CON4_CON0_SLAVE_BCK_INV;
        }
    } else {
        if lrck_inv {
            tdm_con |= ETDM_IN_CON4_CON0_MASTER_LRCK_INV;
        }
        if bck_inv {
            tdm_con |= ETDM_IN_CON4_CON0_MASTER_BCK_INV;
        }
    }
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON4, ETDM_IN_CON4_CTRL_MASK, tdm_con);

    /* ETDM_IN1_CON2 */
    tdm_con = 0;
    if data_mode == TDM_DATA_MULTI_PIN {
        tdm_con |= ETDM_IN_CON2_MULTI_IP_2CH_MODE;
        tdm_con |= ETDM_IN_CON2_MULTI_IP_CH(channels);
    }
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON2, ETDM_IN_CON2_CTRL_MASK, tdm_con);

    /* ETDM_IN1_CON8 */
    tdm_con = 0;
    if slave_mode {
        tdm_con |= 1 << ETDM_IN1_CON8_REG_ETDM_USE_AFIFO_SFT;
        tdm_con |= 0 << ETDM_IN1_CON8_REG_AFIFO_CLOCK_DOMAIN_SEL_SFT;
        tdm_con |= ETDM_IN_CON8_FS(tran_relatch_rate);
    } else {
        tdm_con |= 0 << ETDM_IN1_CON8_REG_ETDM_USE_AFIFO_SFT;
    }
    regmap_update_bits((*afe).regmap, ETDM_IN1_CON8, ETDM_IN_CON8_CTRL_MASK, tdm_con);

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
    let tdm_priv = dai_priv_tdm(afe_priv, (*dai).id);

    if dir != SND_SOC_CLOCK_IN {
        dev_err(
            (*afe).dev,
            b"%s(), dir != SND_SOC_CLOCK_OUT\0".as_ptr() as *const c_char,
            b"mtk_dai_tdm_set_sysclk\0".as_ptr() as *const c_char,
        );
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

unsafe extern "C" fn mtk_dai_tdm_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;
    let tdm_priv = dai_priv_tdm(afe_priv, (*dai).id);

    /* DAI mode*/
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            (*tdm_priv).tdm_mode = TDM_IN_I2S;
            (*tdm_priv).data_mode = TDM_DATA_MULTI_PIN;
        }
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            (*tdm_priv).tdm_mode = TDM_IN_LJ;
            (*tdm_priv).data_mode = TDM_DATA_MULTI_PIN;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {
            (*tdm_priv).tdm_mode = TDM_IN_RJ;
            (*tdm_priv).data_mode = TDM_DATA_MULTI_PIN;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            (*tdm_priv).tdm_mode = TDM_IN_DSP_A;
            (*tdm_priv).data_mode = TDM_DATA_ONE_PIN;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            (*tdm_priv).tdm_mode = TDM_IN_DSP_B;
            (*tdm_priv).data_mode = TDM_DATA_ONE_PIN;
        }
        _ => {
            dev_err(
                (*afe).dev,
                b"%s(), invalid DAIFMT_FORMAT_MASK\0".as_ptr() as *const c_char,
                b"mtk_dai_tdm_set_fmt\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    /* DAI clock inversion*/
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            (*tdm_priv).bck_invert = TDM_BCK_NON_INV;
            (*tdm_priv).lck_invert = TDM_LCK_NON_INV;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            (*tdm_priv).bck_invert = TDM_BCK_NON_INV;
            (*tdm_priv).lck_invert = TDM_LCK_INV;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            (*tdm_priv).bck_invert = TDM_BCK_INV;
            (*tdm_priv).lck_invert = TDM_LCK_NON_INV;
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            (*tdm_priv).bck_invert = TDM_BCK_INV;
            (*tdm_priv).lck_invert = TDM_LCK_INV;
        }
        _ => {
            dev_err(
                (*afe).dev,
                b"%s(), invalid DAIFMT_INV_MASK\0".as_ptr() as *const c_char,
                b"mtk_dai_tdm_set_fmt\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_BP_FP => {
            (*tdm_priv).slave_mode = false as c_uint;
        }
        x if x == SND_SOC_DAIFMT_BC_FC => {
            (*tdm_priv).slave_mode = true as c_uint;
        }
        _ => {
            dev_err(
                (*afe).dev,
                b"%s(), invalid DAIFMT_CLOCK_PROVIDER_MASK\0".as_ptr() as *const c_char,
                b"mtk_dai_tdm_set_fmt\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }
    }

    0
}

unsafe extern "C" fn mtk_dai_tdm_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    _rx_mask: c_uint,
    _slots: c_int,
    slot_width: c_int,
) -> c_int {
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;
    let tdm_priv = dai_priv_tdm(afe_priv, (*dai).id);

    dev_dbg(
        (*dai).dev,
        b"%s %d slot_width %d\n\0".as_ptr() as *const c_char,
        b"mtk_dai_tdm_set_tdm_slot\0".as_ptr() as *const c_char,
        (*dai).id,
        slot_width,
    );

    (*tdm_priv).lrck_width = slot_width as c_uint;

    0
}

static mtk_dai_tdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_tdm_hw_params),
    set_sysclk: Some(mtk_dai_tdm_set_sysclk),
    set_fmt: Some(mtk_dai_tdm_set_fmt),
    set_tdm_slot: Some(mtk_dai_tdm_set_tdm_slot),
};

/* dai driver */
static MTK_TDM_RATES: c_uint = unsafe {
    SNDRV_PCM_RATE_8000_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
};

static MTK_TDM_FORMATS: u64 = unsafe {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
};

static mut mtk_dai_tdm_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"TDM IN\0".as_ptr() as *const c_char,
    id: unsafe { MT8186_DAI_TDM_IN },
    capture: snd_soc_pcm_stream {
        stream_name: b"TDM IN\0".as_ptr() as *const c_char,
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
        return ptr::null_mut();
    }

    (*tdm_priv).mclk_multiple = 512;
    (*tdm_priv).mclk_id = MT8186_TDM_MCK;
    (*tdm_priv).id = MT8186_DAI_TDM_IN as c_uint;

    tdm_priv
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
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
    (*dai).num_dai_drivers = mtk_dai_tdm_driver.len() as c_uint;

    (*dai).controls = mtk_dai_tdm_controls.as_ptr();
    (*dai).num_controls = mtk_dai_tdm_controls.len() as c_uint;
    (*dai).dapm_widgets = mtk_dai_tdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_tdm_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_tdm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_tdm_routes.len() as c_uint;

    tdm_priv = init_tdm_priv_data(afe);
    if tdm_priv.is_null() {
        return -ENOMEM;
    }

    *(*afe_priv).dai_priv.as_mut_ptr().offset(MT8186_DAI_TDM_IN as isize) =
        tdm_priv as *mut c_void;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
