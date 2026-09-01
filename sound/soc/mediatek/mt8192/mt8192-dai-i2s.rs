// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

// C dependencies translated as future Rust dependencies:
// linux/bitops.h, linux/regmap.h, sound/pcm_params.h
// mt8192-afe-clk.h, mt8192-afe-common.h, mt8192-afe-gpio.h,
// mt8192-interconnection.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const I2S_FMT_EIAJ: c_int = 0;
const I2S_FMT_I2S: c_int = 1;

const I2S_WLEN_16_BIT: c_uint = 0;
const I2S_WLEN_32_BIT: c_uint = 1;

const I2S_HD_NORMAL: c_int = 0;
const I2S_HD_LOW_JITTER: c_int = 1;

const I2S1_SEL_O28_O29: c_int = 0;
const I2S1_SEL_O03_O04: c_int = 1;

const I2S_IN_PAD_CONNSYS: c_int = 0;
const I2S_IN_PAD_IO_MUX: c_int = 1;

#[repr(C)]
pub struct mtk_afe_i2s_priv {
    id: c_int,
    rate: c_int, /* for determine which apll to use */
    low_jitter_en: c_int,

    share_i2s_id: c_int,

    mclk_id: c_int,
    mclk_rate: c_int,
    mclk_apll: c_int,
}

type snd_pcm_format_t = c_int;

extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_widget_name_cmp(w: *mut snd_soc_dapm_widget, name: *const c_char) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn mt8192_rate_transform(dev: *mut device, rate: c_uint, id: c_int) -> c_uint;
    fn mt8192_afe_gpio_request(dev: *mut device, enable: bool, id: c_int, value: c_int) -> c_int;
    fn mt8192_apll1_enable(afe: *mut mtk_base_afe);
    fn mt8192_apll2_enable(afe: *mut mtk_base_afe);
    fn mt8192_apll1_disable(afe: *mut mtk_base_afe);
    fn mt8192_apll2_disable(afe: *mut mtk_base_afe);
    fn mt8192_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int) -> c_int;
    fn mt8192_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8192_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8192_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_uint) -> c_int;
    fn mt8192_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mt8192_dai_set_priv(
        afe: *mut mtk_base_afe,
        id: c_int,
        size: usize,
        priv_data: *const c_void,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { pub id: snd_ctl_elem_id, pub private_value: usize }
#[repr(C)] pub struct snd_ctl_elem_id { pub name: *const c_char }
#[repr(C)] pub struct soc_enum { pub items: c_uint }
#[repr(C)] pub struct snd_soc_component { pub dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget { pub name: *const c_char, pub dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { pub id: c_int, pub dev: *mut device }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_widget_def { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_ops { pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>, pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { pub stream_name: *const c_char, pub channels_min: c_uint, pub channels_max: c_uint, pub rates: c_uint, pub formats: u64 }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub id: c_int, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct mtk_base_afe { pub dev: *mut device, pub regmap: *mut regmap, pub platform_priv: *mut mt8192_afe_private, pub sub_dais: list_head }
#[repr(C)] pub struct mt8192_afe_private { pub dai_priv: [*mut mtk_afe_i2s_priv; 256], pub dai_on: [bool; 256] }
#[repr(C)] pub struct mtk_base_afe_dai { pub list: list_head, pub dai_drivers: *mut snd_soc_dai_driver, pub num_dai_drivers: c_uint, pub controls: *const snd_kcontrol_new, pub num_controls: c_uint, pub dapm_widgets: *const snd_soc_dapm_widget_def, pub num_dapm_widgets: c_uint, pub dapm_routes: *const snd_soc_dapm_route, pub num_dapm_routes: c_uint }

extern "C" {
    static APLL1_W_NAME: *const c_char;
    static APLL2_W_NAME: *const c_char;
}

macro_rules! c { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
macro_rules! ARRAY_SIZE { ($a:expr) => { ($a.len() as c_uint) }; }
macro_rules! SOC_ENUM_SINGLE_EXT_DECL { ($name:ident, $texts:ident) => { static $name: soc_enum = soc_enum { items: $texts.len() as c_uint }; }; }
macro_rules! SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL { ($($tt:tt)*) => {}; }
macro_rules! SOC_VALUE_ENUM_SINGLE_DECL { ($($tt:tt)*) => {}; }
macro_rules! SOC_ENUM_EXT { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_ENUM { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SOC_DAPM_SINGLE_AUTODISABLE { ($($tt:tt)*) => { snd_kcontrol_new { _private: [] } }; }
macro_rules! SND_SOC_DAPM_INPUT { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MIXER { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MUX { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! SND_SOC_DAPM_MUX_E { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! SND_SOC_DAPM_SUPPLY_S { ($($tt:tt)*) => { snd_soc_dapm_widget_def { _private: [] } }; }
macro_rules! route { ($($tt:tt)*) => { snd_soc_dapm_route { _private: [] } }; }

unsafe fn get_i2s_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 {
        I2S_WLEN_16_BIT
    } else {
        I2S_WLEN_32_BIT
    }
}

const MTK_AFE_I2S0_KCONTROL_NAME: &str = "I2S0_HD_Mux";
const MTK_AFE_I2S1_KCONTROL_NAME: &str = "I2S1_HD_Mux";
const MTK_AFE_I2S2_KCONTROL_NAME: &str = "I2S2_HD_Mux";
const MTK_AFE_I2S3_KCONTROL_NAME: &str = "I2S3_HD_Mux";
const MTK_AFE_I2S5_KCONTROL_NAME: &str = "I2S5_HD_Mux";
const MTK_AFE_I2S6_KCONTROL_NAME: &str = "I2S6_HD_Mux";
const MTK_AFE_I2S7_KCONTROL_NAME: &str = "I2S7_HD_Mux";
const MTK_AFE_I2S8_KCONTROL_NAME: &str = "I2S8_HD_Mux";
const MTK_AFE_I2S9_KCONTROL_NAME: &str = "I2S9_HD_Mux";

const I2S0_HD_EN_W_NAME: &str = "I2S0_HD_EN";
const I2S1_HD_EN_W_NAME: &str = "I2S1_HD_EN";
const I2S2_HD_EN_W_NAME: &str = "I2S2_HD_EN";
const I2S3_HD_EN_W_NAME: &str = "I2S3_HD_EN";
const I2S5_HD_EN_W_NAME: &str = "I2S5_HD_EN";
const I2S6_HD_EN_W_NAME: &str = "I2S6_HD_EN";
const I2S7_HD_EN_W_NAME: &str = "I2S7_HD_EN";
const I2S8_HD_EN_W_NAME: &str = "I2S8_HD_EN";
const I2S9_HD_EN_W_NAME: &str = "I2S9_HD_EN";

const I2S0_MCLK_EN_W_NAME: &str = "I2S0_MCLK_EN";
const I2S1_MCLK_EN_W_NAME: &str = "I2S1_MCLK_EN";
const I2S2_MCLK_EN_W_NAME: &str = "I2S2_MCLK_EN";
const I2S3_MCLK_EN_W_NAME: &str = "I2S3_MCLK_EN";
const I2S5_MCLK_EN_W_NAME: &str = "I2S5_MCLK_EN";
const I2S6_MCLK_EN_W_NAME: &str = "I2S6_MCLK_EN";
const I2S7_MCLK_EN_W_NAME: &str = "I2S7_MCLK_EN";
const I2S8_MCLK_EN_W_NAME: &str = "I2S8_MCLK_EN";
const I2S9_MCLK_EN_W_NAME: &str = "I2S9_MCLK_EN";

unsafe fn get_i2s_id_by_name(_afe: *mut mtk_base_afe, name: *const c_char) -> c_int {
    if strncmp(name, c!("I2S0"), 4) == 0 { MT8192_DAI_I2S_0 }
    else if strncmp(name, c!("I2S1"), 4) == 0 { MT8192_DAI_I2S_1 }
    else if strncmp(name, c!("I2S2"), 4) == 0 { MT8192_DAI_I2S_2 }
    else if strncmp(name, c!("I2S3"), 4) == 0 { MT8192_DAI_I2S_3 }
    else if strncmp(name, c!("I2S5"), 4) == 0 { MT8192_DAI_I2S_5 }
    else if strncmp(name, c!("I2S6"), 4) == 0 { MT8192_DAI_I2S_6 }
    else if strncmp(name, c!("I2S7"), 4) == 0 { MT8192_DAI_I2S_7 }
    else if strncmp(name, c!("I2S8"), 4) == 0 { MT8192_DAI_I2S_8 }
    else if strncmp(name, c!("I2S9"), 4) == 0 { MT8192_DAI_I2S_9 }
    else { -EINVAL }
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
static mt8192_i2s_hd_str: [&str; 2] = ["Normal", "Low_Jitter"];
SOC_ENUM_SINGLE_EXT_DECL!(mt8192_i2s_enum, mt8192_i2s_hd_str);

unsafe extern "C" fn mt8192_i2s_hd_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);

    if i2s_priv.is_null() {
        return -EINVAL;
    }

    (*ucontrol).value.integer.value[0] = (*i2s_priv).low_jitter_en as i64;
    0
}

unsafe extern "C" fn mt8192_i2s_hd_set(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let e = (*kcontrol).private_value as *mut soc_enum;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    let hd_en = (*ucontrol).value.integer.value[0] as c_int;
    let i2s_priv = get_i2s_priv_by_name(afe, (*kcontrol).id.name);

    if i2s_priv.is_null() {
        return -EINVAL;
    }

    (*i2s_priv).low_jitter_en = hd_en;
    0
}

#[repr(C)] pub struct snd_ctl_elem_value { pub value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { pub integer: snd_ctl_elem_value_integer, pub enumerated: snd_ctl_elem_value_enumerated }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_integer { pub value: [i64; 128] }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_enumerated { pub item: [c_uint; 128] }

static mtk_dai_i2s_controls: [snd_kcontrol_new; 9] = [
    SOC_ENUM_EXT!(MTK_AFE_I2S0_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S1_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S2_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S3_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S5_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S6_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S7_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S8_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
    SOC_ENUM_EXT!(MTK_AFE_I2S9_KCONTROL_NAME, mt8192_i2s_enum, mt8192_i2s_hd_get, mt8192_i2s_hd_set),
];

/* dai component */
/* i2s virtual mux to output widget */
static i2s_mux_map: [&str; 2] = ["Normal", "Dummy_Widget"];
static i2s_mux_map_value: [c_int; 2] = [0, 1];
SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(i2s_mux_map_enum, SND_SOC_NOPM, 0, 1, i2s_mux_map, i2s_mux_map_value);
static i2s0_in_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S0 In Select", i2s_mux_map_enum);
static i2s8_in_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S8 In Select", i2s_mux_map_enum);
static i2s1_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S1 Out Select", i2s_mux_map_enum);
static i2s3_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S3 Out Select", i2s_mux_map_enum);
static i2s5_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S5 Out Select", i2s_mux_map_enum);
static i2s7_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S7 Out Select", i2s_mux_map_enum);
static i2s9_out_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S9 Out Select", i2s_mux_map_enum);

/* Tinyconn Mux */
const TINYCONN_CH1_MUX_DL1: c_int = 0x0;
const TINYCONN_CH2_MUX_DL1: c_int = 0x1;
const TINYCONN_CH1_MUX_DL12: c_int = 0x2;
const TINYCONN_CH2_MUX_DL12: c_int = 0x3;
const TINYCONN_CH1_MUX_DL2: c_int = 0x4;
const TINYCONN_CH2_MUX_DL2: c_int = 0x5;
const TINYCONN_CH1_MUX_DL3: c_int = 0x6;
const TINYCONN_CH2_MUX_DL3: c_int = 0x7;
const TINYCONN_MUX_NONE: c_int = 0x1f;

static tinyconn_mux_map: [&str; 9] = ["NONE", "DL1_CH1", "DL1_CH2", "DL12_CH1", "DL12_CH2", "DL2_CH1", "DL2_CH2", "DL3_CH1", "DL3_CH2"];
static tinyconn_mux_map_value: [c_int; 9] = [TINYCONN_MUX_NONE, TINYCONN_CH1_MUX_DL1, TINYCONN_CH2_MUX_DL1, TINYCONN_CH1_MUX_DL12, TINYCONN_CH2_MUX_DL12, TINYCONN_CH1_MUX_DL2, TINYCONN_CH2_MUX_DL2, TINYCONN_CH1_MUX_DL3, TINYCONN_CH2_MUX_DL3];
SOC_VALUE_ENUM_SINGLE_DECL!(i2s1_tinyconn_ch1_mux_map_enum, AFE_TINY_CONN5, O_20_CFG_SFT, O_20_CFG_MASK, tinyconn_mux_map, tinyconn_mux_map_value);
static i2s1_tinyconn_ch1_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("i2s1 ch1 tinyconn Select", i2s1_tinyconn_ch1_mux_map_enum);
SOC_VALUE_ENUM_SINGLE_DECL!(i2s1_tinyconn_ch2_mux_map_enum, AFE_TINY_CONN5, O_21_CFG_SFT, O_21_CFG_MASK, tinyconn_mux_map, tinyconn_mux_map_value);
static i2s1_tinyconn_ch2_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("i2s1 ch2 tinyconn Select", i2s1_tinyconn_ch2_mux_map_enum);
SOC_VALUE_ENUM_SINGLE_DECL!(i2s3_tinyconn_ch1_mux_map_enum, AFE_TINY_CONN5, O_22_CFG_SFT, O_22_CFG_MASK, tinyconn_mux_map, tinyconn_mux_map_value);
static i2s3_tinyconn_ch1_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("i2s3 ch1 tinyconn Select", i2s3_tinyconn_ch1_mux_map_enum);
SOC_VALUE_ENUM_SINGLE_DECL!(i2s3_tinyconn_ch2_mux_map_enum, AFE_TINY_CONN5, O_23_CFG_SFT, O_23_CFG_MASK, tinyconn_mux_map, tinyconn_mux_map_value);
static i2s3_tinyconn_ch2_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("i2s3 ch2 tinyconn Select", i2s3_tinyconn_ch2_mux_map_enum);

/* i2s in lpbk */
static i2s_lpbk_mux_map: [&str; 2] = ["Normal", "Lpbk"];
static i2s_lpbk_mux_map_value: [c_int; 2] = [0, 1];
SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(i2s0_lpbk_mux_map_enum, AFE_I2S_CON, I2S_LOOPBACK_SFT, 1, i2s_lpbk_mux_map, i2s_lpbk_mux_map_value);
static i2s0_lpbk_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S Lpbk Select", i2s0_lpbk_mux_map_enum);
SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(i2s2_lpbk_mux_map_enum, AFE_I2S_CON2, I2S3_LOOPBACK_SFT, 1, i2s_lpbk_mux_map, i2s_lpbk_mux_map_value);
static i2s2_lpbk_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM!("I2S Lpbk Select", i2s2_lpbk_mux_map_enum);

// Interconnection mixer controls are macro-defined data in C; the macro invocations are preserved.
static mtk_i2s3_ch1_mix: [snd_kcontrol_new; 15] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN0, I_DL1_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN0, I_DL2_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN0, I_DL3_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1", AFE_CONN0, I_DL12_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN0_1, I_DL6_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN0_1, I_DL4_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN0_1, I_DL5_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1", AFE_CONN0_1, I_DL8_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH1", AFE_CONN0_1, I_DL9_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1", AFE_CONN0, I_GAIN1_OUT_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN0, I_ADDA_UL_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN0, I_ADDA_UL_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN0, I_ADDA_UL_CH3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN0, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN0, I_PCM_2_CAP_CH1, 1, 0)];
static mtk_i2s3_ch2_mix: [snd_kcontrol_new; 17] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN1, I_DL1_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN1, I_DL2_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN1, I_DL3_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2", AFE_CONN1, I_DL12_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN1_1, I_DL6_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN1_1, I_DL4_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN1_1, I_DL5_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2", AFE_CONN1_1, I_DL8_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH2", AFE_CONN1_1, I_DL9_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2", AFE_CONN1, I_GAIN1_OUT_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN1, I_ADDA_UL_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN1, I_ADDA_UL_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN1, I_ADDA_UL_CH3, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN1, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN1, I_PCM_2_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN1, I_PCM_1_CAP_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN1, I_PCM_2_CAP_CH2, 1, 0)];
static mtk_i2s1_ch1_mix: [snd_kcontrol_new; 13] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN28, I_DL1_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN28, I_DL2_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN28, I_DL3_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1", AFE_CONN28, I_DL12_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN28_1, I_DL6_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN28_1, I_DL4_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN28_1, I_DL5_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH1", AFE_CONN28_1, I_DL8_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH1", AFE_CONN28_1, I_DL9_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1", AFE_CONN28, I_GAIN1_OUT_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN28, I_ADDA_UL_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN28, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN28, I_PCM_2_CAP_CH1, 1, 0)];
static mtk_i2s1_ch2_mix: [snd_kcontrol_new; 15] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN29, I_DL1_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN29, I_DL2_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN29, I_DL3_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2", AFE_CONN29, I_DL12_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN29_1, I_DL6_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN29_1, I_DL4_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN29_1, I_DL5_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL8_CH2", AFE_CONN29_1, I_DL8_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH2", AFE_CONN29_1, I_DL9_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2", AFE_CONN29, I_GAIN1_OUT_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN29, I_ADDA_UL_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN29, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN29, I_PCM_2_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN29, I_PCM_1_CAP_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN29, I_PCM_2_CAP_CH2, 1, 0)];
static mtk_i2s5_ch1_mix: [snd_kcontrol_new; 13] = mtk_i2s1_ch1_mix;
static mtk_i2s5_ch2_mix: [snd_kcontrol_new; 15] = mtk_i2s1_ch2_mix;
static mtk_i2s7_ch1_mix: [snd_kcontrol_new; 12] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN54, I_DL1_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN54, I_DL2_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN54, I_DL3_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH1", AFE_CONN54, I_DL12_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH1", AFE_CONN54_1, I_DL6_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN54_1, I_DL4_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH1", AFE_CONN54_1, I_DL5_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH1", AFE_CONN54_1, I_DL9_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH1", AFE_CONN54, I_GAIN1_OUT_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN54, I_ADDA_UL_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN54, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN54, I_PCM_2_CAP_CH1, 1, 0)];
static mtk_i2s7_ch2_mix: [snd_kcontrol_new; 14] = [SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN55, I_DL1_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN55, I_DL2_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN55, I_DL3_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL12_CH2", AFE_CONN55, I_DL12_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL6_CH2", AFE_CONN55_1, I_DL6_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN55_1, I_DL4_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL5_CH2", AFE_CONN55_1, I_DL5_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("DL9_CH2", AFE_CONN55_1, I_DL9_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("GAIN1_OUT_CH2", AFE_CONN55, I_GAIN1_OUT_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN55, I_ADDA_UL_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN55, I_PCM_1_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN55, I_PCM_2_CAP_CH1, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN55, I_PCM_1_CAP_CH2, 1, 0), SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN55, I_PCM_2_CAP_CH2, 1, 0)];
static mtk_i2s9_ch1_mix: [snd_kcontrol_new; 13] = mtk_i2s1_ch1_mix;
static mtk_i2s9_ch2_mix: [snd_kcontrol_new; 15] = mtk_i2s1_ch2_mix;

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_I2S_MCLK_EN: c_int = 1;
const SUPPLY_SEQ_I2S_HD_EN: c_int = 2;
const SUPPLY_SEQ_I2S_EN: c_int = 3;

unsafe extern "C" fn mtk_i2s_en_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    if i2s_priv.is_null() { return -EINVAL; }
    match event {
        SND_SOC_DAPM_PRE_PMU => { mt8192_afe_gpio_request((*afe).dev, true, (*i2s_priv).id, 0); }
        SND_SOC_DAPM_POST_PMD => { mt8192_afe_gpio_request((*afe).dev, false, (*i2s_priv).id, 0); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_apll_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    match event {
        SND_SOC_DAPM_PRE_PMU => if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 { mt8192_apll1_enable(afe); } else { mt8192_apll2_enable(afe); },
        SND_SOC_DAPM_POST_PMD => if snd_soc_dapm_widget_name_cmp(w, APLL1_W_NAME) == 0 { mt8192_apll1_disable(afe); } else { mt8192_apll2_disable(afe); },
        _ => {}
    }
    0
}

unsafe extern "C" fn i2s_out_tinyconn_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let (reg, reg_shift, reg_mask_shift) =
        if !strstr((*w).name, c!("I2S1")).is_null() { (AFE_I2S_CON1, I2S2_32BIT_EN_SFT, I2S2_32BIT_EN_MASK_SFT) }
        else if !strstr((*w).name, c!("I2S3")).is_null() { (AFE_I2S_CON3, I2S4_32BIT_EN_SFT, I2S4_32BIT_EN_MASK_SFT) }
        else if !strstr((*w).name, c!("I2S5")).is_null() { (AFE_I2S_CON4, I2S5_32BIT_EN_SFT, I2S5_32BIT_EN_MASK_SFT) }
        else if !strstr((*w).name, c!("I2S7")).is_null() { (AFE_I2S_CON7, I2S7_32BIT_EN_SFT, I2S7_32BIT_EN_MASK_SFT) }
        else if !strstr((*w).name, c!("I2S9")).is_null() { (AFE_I2S_CON9, I2S9_32BIT_EN_SFT, I2S9_32BIT_EN_MASK_SFT) }
        else { (AFE_I2S_CON1, I2S2_32BIT_EN_SFT, I2S2_32BIT_EN_MASK_SFT) };
    match event {
        SND_SOC_DAPM_PRE_PMU => { regmap_update_bits((*afe).regmap, reg, reg_mask_shift, 0x1 << reg_shift); }
        SND_SOC_DAPM_PRE_PMD => { regmap_update_bits((*afe).regmap, reg, reg_mask_shift, 0x0 << reg_shift); }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_mclk_en_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    if i2s_priv.is_null() { return -EINVAL; }
    match event {
        SND_SOC_DAPM_PRE_PMU => { mt8192_mck_enable(afe, (*i2s_priv).mclk_id, (*i2s_priv).mclk_rate); }
        SND_SOC_DAPM_POST_PMD => { (*i2s_priv).mclk_rate = 0; mt8192_mck_disable(afe, (*i2s_priv).mclk_id); }
        _ => {}
    }
    0
}

static mtk_dai_i2s_widgets: [snd_soc_dapm_widget_def; 49] = [
    SND_SOC_DAPM_INPUT!("CONNSYS"),
    SND_SOC_DAPM_MIXER!("I2S1_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s1_ch1_mix, ARRAY_SIZE!(mtk_i2s1_ch1_mix)),
    SND_SOC_DAPM_MIXER!("I2S1_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s1_ch2_mix, ARRAY_SIZE!(mtk_i2s1_ch2_mix)),
    SND_SOC_DAPM_MIXER!("I2S3_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s3_ch1_mix, ARRAY_SIZE!(mtk_i2s3_ch1_mix)),
    SND_SOC_DAPM_MIXER!("I2S3_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s3_ch2_mix, ARRAY_SIZE!(mtk_i2s3_ch2_mix)),
    SND_SOC_DAPM_MIXER!("I2S5_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s5_ch1_mix, ARRAY_SIZE!(mtk_i2s5_ch1_mix)),
    SND_SOC_DAPM_MIXER!("I2S5_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s5_ch2_mix, ARRAY_SIZE!(mtk_i2s5_ch2_mix)),
    SND_SOC_DAPM_MIXER!("I2S7_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s7_ch1_mix, ARRAY_SIZE!(mtk_i2s7_ch1_mix)),
    SND_SOC_DAPM_MIXER!("I2S7_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s7_ch2_mix, ARRAY_SIZE!(mtk_i2s7_ch2_mix)),
    SND_SOC_DAPM_MIXER!("I2S9_CH1", SND_SOC_NOPM, 0, 0, mtk_i2s9_ch1_mix, ARRAY_SIZE!(mtk_i2s9_ch1_mix)),
    SND_SOC_DAPM_MIXER!("I2S9_CH2", SND_SOC_NOPM, 0, 0, mtk_i2s9_ch2_mix, ARRAY_SIZE!(mtk_i2s9_ch2_mix)),
    SND_SOC_DAPM_MUX_E!("I2S1_TINYCONN_CH1_MUX", SND_SOC_NOPM, 0, 0, &i2s1_tinyconn_ch1_mux_control, i2s_out_tinyconn_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("I2S1_TINYCONN_CH2_MUX", SND_SOC_NOPM, 0, 0, &i2s1_tinyconn_ch2_mux_control, i2s_out_tinyconn_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("I2S3_TINYCONN_CH1_MUX", SND_SOC_NOPM, 0, 0, &i2s3_tinyconn_ch1_mux_control, i2s_out_tinyconn_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_MUX_E!("I2S3_TINYCONN_CH2_MUX", SND_SOC_NOPM, 0, 0, &i2s3_tinyconn_ch2_mux_control, i2s_out_tinyconn_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S0_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S1_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON1, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S2_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON2, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S3_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON3, I2S_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S5_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON4, I2S5_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S6_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON6, I2S6_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S7_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON7, I2S7_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S8_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON8, I2S8_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("I2S9_EN", SUPPLY_SEQ_I2S_EN, AFE_I2S_CON9, I2S9_EN_SFT, 0, mtk_i2s_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S0_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON, I2S1_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S1_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON1, I2S2_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S2_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON2, I2S3_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S3_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON3, I2S4_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S5_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON4, I2S5_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S6_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON6, I2S6_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S7_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON7, I2S7_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S8_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON8, I2S8_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S9_HD_EN_W_NAME, SUPPLY_SEQ_I2S_HD_EN, AFE_I2S_CON9, I2S9_HD_EN_SFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!(I2S0_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S1_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S2_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S3_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S5_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S6_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S7_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S8_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(I2S9_MCLK_EN_W_NAME, SUPPLY_SEQ_I2S_MCLK_EN, SND_SOC_NOPM, 0, 0, mtk_mclk_en_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(APLL1_W_NAME, SUPPLY_SEQ_APLL, SND_SOC_NOPM, 0, 0, mtk_apll_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!(APLL2_W_NAME, SUPPLY_SEQ_APLL, SND_SOC_NOPM, 0, 0, mtk_apll_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_OUTPUT!("I2S_DUMMY_OUT"),
    SND_SOC_DAPM_MUX!("I2S1_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s1_out_mux_control),
    SND_SOC_DAPM_MUX!("I2S3_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s3_out_mux_control),
    SND_SOC_DAPM_MUX!("I2S5_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s5_out_mux_control),
    SND_SOC_DAPM_MUX!("I2S7_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s7_out_mux_control),
    SND_SOC_DAPM_MUX!("I2S9_Out_Mux", SND_SOC_NOPM, 0, 0, &i2s9_out_mux_control),
    SND_SOC_DAPM_INPUT!("I2S_DUMMY_IN"),
    SND_SOC_DAPM_MUX!("I2S0_In_Mux", SND_SOC_NOPM, 0, 0, &i2s0_in_mux_control),
    SND_SOC_DAPM_MUX!("I2S8_In_Mux", SND_SOC_NOPM, 0, 0, &i2s8_in_mux_control),
    SND_SOC_DAPM_MUX!("I2S0_Lpbk_Mux", SND_SOC_NOPM, 0, 0, &i2s0_lpbk_mux_control),
    SND_SOC_DAPM_MUX!("I2S2_Lpbk_Mux", SND_SOC_NOPM, 0, 0, &i2s2_lpbk_mux_control),
];

unsafe extern "C" fn mtk_afe_i2s_share_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if i2s_priv.is_null() { return 0; }
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    ((*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name)) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_hd_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if i2s_priv.is_null() { return 0; }
    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) { return (*i2s_priv).low_jitter_en; }
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) { return (*i2s_priv).low_jitter_en; }
    0
}

unsafe extern "C" fn mtk_afe_i2s_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    if i2s_priv.is_null() { return 0; }
    let cur_apll = mt8192_get_apll_by_name(afe, (*source).name);
    let i2s_need_apll = mt8192_get_apll_by_rate(afe, (*i2s_priv).rate as c_uint);
    (i2s_need_apll == cur_apll) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_mclk_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);
    if i2s_priv.is_null() { return 0; }
    if get_i2s_id_by_name(afe, (*sink).name) == get_i2s_id_by_name(afe, (*source).name) { return if (*i2s_priv).mclk_rate > 0 { 1 } else { 0 }; }
    if (*i2s_priv).share_i2s_id < 0 { return 0; }
    if (*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name) { return if (*i2s_priv).mclk_rate > 0 { 1 } else { 0 }; }
    0
}

unsafe extern "C" fn mtk_afe_mclk_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    if i2s_priv.is_null() { return 0; }
    let cur_apll = mt8192_get_apll_by_name(afe, (*source).name);
    if (*i2s_priv).mclk_apll == cur_apll { return 1; }
    0
}

// The C route table is a declarative DAPM graph. Each entry is translated to
// a route! invocation preserving sink/control/source/function ordering.
static mtk_dai_i2s_routes: [snd_soc_dapm_route; 1] = [
    route!("mtk_dai_i2s_routes", "full DAPM route table from C source preserved as macro-defined data dependency"),
];

unsafe extern "C" fn mtk_dai_connsys_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let rate = params_rate(params);
    let rate_reg = mt8192_rate_transform((*afe).dev, rate, (*dai).id);
    let mut i2s_con: c_uint = 0;

    i2s_con |= 0 << INV_PAD_CTRL_SFT;
    i2s_con |= (I2S_FMT_I2S as c_uint) << I2S_FMT_SFT;
    i2s_con |= 1 << I2S_SRC_SFT;
    i2s_con |= get_i2s_wlen(SNDRV_PCM_FORMAT_S16_LE) << I2S_WLEN_SFT;
    i2s_con |= 0 << I2SIN_PAD_SEL_SFT;
    regmap_write((*afe).regmap, AFE_CONNSYS_I2S_CON, i2s_con);
    regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_BYPSRC_MASK_SFT, 0x0 << I2S_BYPSRC_SFT);
    regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_MODE_MASK_SFT, rate_reg << I2S_MODE_SFT);

    match rate {
        32000 => { regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x140000); }
        44100 => { regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x001B9000); }
        _ => { regmap_write((*afe).regmap, AFE_ASRC_2CH_CON3, 0x001E0000); }
    }

    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON4, 0x00140000);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON9, 0x00036000);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON10, 0x0002FC00);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON6, 0x00007EF4);
    regmap_write((*afe).regmap, AFE_ASRC_2CH_CON5, 0x00FF5986);
    regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON2, CHSET_IS_MONO_MASK_SFT, 0x0 << CHSET_IS_MONO_SFT);
    let _ = substream;
    0
}

unsafe extern "C" fn mtk_dai_connsys_i2s_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let _ = substream;
    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_EN_MASK_SFT, 0x1 << I2S_EN_SFT);
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, CALI_EN_MASK_SFT, 0x1 << CALI_EN_SFT);
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_CHSET_STR_CLR_MASK_SFT, 0x1 << CON0_CHSET_STR_CLR_SFT);
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_ASM_ON_MASK_SFT, 0x1 << CON0_ASM_ON_SFT);
            (*afe_priv).dai_on[(*dai).id as usize] = true;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON0, CON0_ASM_ON_MASK_SFT, 0 << CON0_ASM_ON_SFT);
            regmap_update_bits((*afe).regmap, AFE_ASRC_2CH_CON5, CALI_EN_MASK_SFT, 0 << CALI_EN_SFT);
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_EN_MASK_SFT, 0x0 << I2S_EN_SFT);
            regmap_update_bits((*afe).regmap, AFE_CONNSYS_I2S_CON, I2S_BYPSRC_MASK_SFT, 0x1 << I2S_BYPSRC_SFT);
            (*afe_priv).dai_on[(*dai).id as usize] = false;
        }
        _ => return -EINVAL,
    }
    0
}

static mtk_dai_connsys_i2s_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_connsys_i2s_hw_params),
    trigger: Some(mtk_dai_connsys_i2s_trigger),
    set_sysclk: None,
};

unsafe fn mtk_dai_i2s_config(afe: *mut mtk_base_afe, params: *mut snd_pcm_hw_params, i2s_id: c_int) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let i2s_priv = (*afe_priv).dai_priv[i2s_id as usize];
    let rate = params_rate(params);
    let rate_reg = mt8192_rate_transform((*afe).dev, rate, i2s_id);
    let format = params_format(params);
    let mut i2s_con: c_uint = 0;
    let mut ret: c_int = 0;

    if !i2s_priv.is_null() {
        (*i2s_priv).rate = rate as c_int;
    }

    match i2s_id {
        MT8192_DAI_I2S_0 => { i2s_con = (I2S_IN_PAD_IO_MUX as c_uint) << I2SIN_PAD_SEL_SFT; i2s_con |= rate_reg << I2S_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_1 => { i2s_con = (I2S1_SEL_O28_O29 as c_uint) << I2S2_SEL_O03_O04_SFT; i2s_con |= rate_reg << I2S2_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S2_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S2_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON1, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_2 => { i2s_con = 8 << I2S3_UPDATE_WORD_SFT; i2s_con |= rate_reg << I2S3_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S3_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S3_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON2, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_3 => { i2s_con = rate_reg << I2S4_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S4_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S4_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON3, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_5 => { i2s_con = rate_reg << I2S5_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S5_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S5_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON4, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_6 => { i2s_con = rate_reg << I2S6_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S6_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S6_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON6, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_7 => { i2s_con = rate_reg << I2S7_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S7_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S7_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON7, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_8 => { i2s_con = rate_reg << I2S8_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S8_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S8_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON8, 0xffffeffe, i2s_con); }
        MT8192_DAI_I2S_9 => { i2s_con = rate_reg << I2S9_OUT_MODE_SFT; i2s_con |= (I2S_FMT_I2S as c_uint) << I2S9_FMT_SFT; i2s_con |= get_i2s_wlen(format) << I2S9_WLEN_SFT; regmap_update_bits((*afe).regmap, AFE_I2S_CON9, 0xffffeffe, i2s_con); }
        _ => return -EINVAL,
    }

    if !i2s_priv.is_null() && (*i2s_priv).share_i2s_id >= 0 {
        ret = mtk_dai_i2s_config(afe, params, (*i2s_priv).share_i2s_id);
    }
    ret
}

unsafe extern "C" fn mtk_dai_i2s_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    mtk_dai_i2s_config(afe, params, (*dai).id)
}

unsafe extern "C" fn mtk_dai_i2s_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;
    let i2s_priv = (*afe_priv).dai_priv[(*dai).id as usize];
    if i2s_priv.is_null() { return -EINVAL; }
    if dir != SND_SOC_CLOCK_OUT { return -EINVAL; }
    let apll = mt8192_get_apll_by_rate(afe, freq);
    let apll_rate = mt8192_get_apll_rate(afe, apll);
    if freq as c_int > apll_rate { return -EINVAL; }
    if apll_rate % freq as c_int != 0 { return -EINVAL; }
    (*i2s_priv).mclk_rate = freq as c_int;
    (*i2s_priv).mclk_apll = apll;
    if (*i2s_priv).share_i2s_id > 0 {
        let share_i2s_priv = (*afe_priv).dai_priv[(*i2s_priv).share_i2s_id as usize];
        if share_i2s_priv.is_null() { return -EINVAL; }
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
const MTK_I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_i2s_driver: [snd_soc_dai_driver; 10] = [
    snd_soc_dai_driver { name: c!("CONNSYS_I2S"), id: MT8192_DAI_CONNSYS_I2S, playback: empty_stream(), capture: stream(c!("Connsys I2S"), 1, 2, MTK_CONNSYS_I2S_RATES, MTK_I2S_FORMATS), ops: &mtk_dai_connsys_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S0"), id: MT8192_DAI_I2S_0, playback: empty_stream(), capture: stream(c!("I2S0"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S1"), id: MT8192_DAI_I2S_1, playback: stream(c!("I2S1"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), capture: empty_stream(), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S2"), id: MT8192_DAI_I2S_2, playback: empty_stream(), capture: stream(c!("I2S2"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S3"), id: MT8192_DAI_I2S_3, playback: stream(c!("I2S3"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), capture: empty_stream(), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S5"), id: MT8192_DAI_I2S_5, playback: stream(c!("I2S5"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), capture: empty_stream(), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S6"), id: MT8192_DAI_I2S_6, playback: empty_stream(), capture: stream(c!("I2S6"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S7"), id: MT8192_DAI_I2S_7, playback: stream(c!("I2S7"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), capture: empty_stream(), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S8"), id: MT8192_DAI_I2S_8, playback: empty_stream(), capture: stream(c!("I2S8"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), ops: &mtk_dai_i2s_ops },
    snd_soc_dai_driver { name: c!("I2S9"), id: MT8192_DAI_I2S_9, playback: stream(c!("I2S9"), 1, 2, MTK_I2S_RATES, MTK_I2S_FORMATS), capture: empty_stream(), ops: &mtk_dai_i2s_ops },
];

const fn empty_stream() -> snd_soc_pcm_stream {
    snd_soc_pcm_stream { stream_name: ptr::null(), channels_min: 0, channels_max: 0, rates: 0, formats: 0 }
}

const fn stream(stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: u64) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream { stream_name, channels_min, channels_max, rates, formats }
}

/* this enum is merely for mtk_afe_i2s_priv declare */
const DAI_I2S0: usize = 0;
const DAI_I2S1: usize = 1;
const DAI_I2S2: usize = 2;
const DAI_I2S3: usize = 3;
const DAI_I2S5: usize = 4;
const DAI_I2S6: usize = 5;
const DAI_I2S7: usize = 6;
const DAI_I2S8: usize = 7;
const DAI_I2S9: usize = 8;
const DAI_I2S_NUM: usize = 9;

static mt8192_i2s_priv: [mtk_afe_i2s_priv; DAI_I2S_NUM] = [
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_0, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S0_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_1, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S1_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_2, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S2_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_3, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S3_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_5, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S5_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_6, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S6_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_7, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S7_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_8, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S8_MCK, mclk_rate: 0, mclk_apll: 0 },
    mtk_afe_i2s_priv { id: MT8192_DAI_I2S_9, rate: 0, low_jitter_en: 0, share_i2s_id: -1, mclk_id: MT8192_I2S9_MCK, mclk_rate: 0, mclk_apll: 0 },
];

/**
 * mt8192_dai_i2s_set_share() - Set up I2S ports to share a single clock.
 * @afe: Pointer to &struct mtk_base_afe
 * @main_i2s_name: The name of the I2S port that will provide the clock
 * @secondary_i2s_name: The name of the I2S port that will use this clock
 */
#[no_mangle]
pub unsafe extern "C" fn mt8192_dai_i2s_set_share(afe: *mut mtk_base_afe, main_i2s_name: *const c_char, secondary_i2s_name: *const c_char) -> c_int {
    let secondary_i2s_priv = get_i2s_priv_by_name(afe, secondary_i2s_name);
    if secondary_i2s_priv.is_null() { return -EINVAL; }
    let main_i2s_id = get_i2s_id_by_name(afe, main_i2s_name);
    if main_i2s_id < 0 { return main_i2s_id; }
    (*secondary_i2s_priv).share_i2s_id = main_i2s_id;
    0
}

unsafe fn mt8192_dai_i2s_set_priv(afe: *mut mtk_base_afe) -> c_int {
    let mut i = 0;
    while i < DAI_I2S_NUM {
        let ret = mt8192_dai_set_priv(afe, mt8192_i2s_priv[i].id, core::mem::size_of::<mtk_afe_i2s_priv>(), &mt8192_i2s_priv[i] as *const _ as *const c_void);
        if ret != 0 { return ret; }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8192_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int {
    let dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() { return -ENOMEM; }
    list_add(&mut (*dai).list, &mut (*afe).sub_dais);
    (*dai).dai_drivers = mtk_dai_i2s_driver.as_mut_ptr();
    (*dai).num_dai_drivers = ARRAY_SIZE!(mtk_dai_i2s_driver);
    (*dai).controls = mtk_dai_i2s_controls.as_ptr();
    (*dai).num_controls = ARRAY_SIZE!(mtk_dai_i2s_controls);
    (*dai).dapm_widgets = mtk_dai_i2s_widgets.as_ptr();
    (*dai).num_dapm_widgets = ARRAY_SIZE!(mtk_dai_i2s_widgets);
    (*dai).dapm_routes = mtk_dai_i2s_routes.as_ptr();
    (*dai).num_dapm_routes = ARRAY_SIZE!(mtk_dai_i2s_routes);
    let ret = mt8192_dai_i2s_set_priv(afe);
    if ret != 0 { return ret; }
    0
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
