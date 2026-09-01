// SPDX-License-Identifier: GPL-2.0
/*
 *  MediaTek ALSA SoC Audio DAI I2S Control
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Rust translation of dependencies originally included from:
 * linux/bitops.h, linux/regmap.h, sound/pcm_params.h,
 * mt8189-afe-clk.h, mt8189-afe-common.h, mt8189-interconnection.h,
 * ../common/mtk-afe-fe-dai.h.
 */

const I2SIN0_MCLK_EN_W_NAME: *const c_char = c"I2SIN0_MCLK_EN".as_ptr();
const I2SIN1_MCLK_EN_W_NAME: *const c_char = c"I2SIN1_MCLK_EN".as_ptr();
const I2SOUT0_MCLK_EN_W_NAME: *const c_char = c"I2SOUT0_MCLK_EN".as_ptr();
const I2SOUT1_MCLK_EN_W_NAME: *const c_char = c"I2SOUT1_MCLK_EN".as_ptr();
const I2SOUT4_MCLK_EN_W_NAME: *const c_char = c"I2SOUT4_MCLK_EN".as_ptr();

const SUPPLY_SEQ_APLL: c_int = 0;
const SUPPLY_SEQ_I2S_MCLK_EN: c_int = 1;
const SUPPLY_SEQ_I2S_CG_EN: c_int = 2;
const SUPPLY_SEQ_I2S_EN: c_int = 3;

/* this enum is merely for mtk_afe_i2s_priv declare */
const DAI_I2SIN0: usize = 0;
const DAI_I2SIN1: usize = 1;
const DAI_I2SOUT0: usize = 2;
const DAI_I2SOUT1: usize = 3;
const DAI_I2SOUT4: usize = 4;
const DAI_I2S_NUM: usize = 5;

const ETDM_CLK_SOURCE_H26M: c_uint = 0;
const ETDM_CLK_SOURCE_APLL: c_uint = 1;
const ETDM_CLK_SOURCE_SPDIF: c_uint = 2;
const ETDM_CLK_SOURCE_HDMI: c_uint = 3;
const ETDM_CLK_SOURCE_EARC: c_uint = 4;
const ETDM_CLK_SOURCE_LINEIN: c_uint = 5;

const ETDM_RELATCH_SEL_H26M: c_uint = 0;
const ETDM_RELATCH_SEL_APLL: c_uint = 1;

const ETDM_RATE_8K: c_uint = 0;
const ETDM_RATE_12K: c_uint = 1;
const ETDM_RATE_16K: c_uint = 2;
const ETDM_RATE_24K: c_uint = 3;
const ETDM_RATE_32K: c_uint = 4;
const ETDM_RATE_48K: c_uint = 5;
const ETDM_RATE_64K: c_uint = 6;
const ETDM_RATE_96K: c_uint = 7;
const ETDM_RATE_128K: c_uint = 8;
const ETDM_RATE_192K: c_uint = 9;
const ETDM_RATE_256K: c_uint = 10;
const ETDM_RATE_384K: c_uint = 11;
const ETDM_RATE_11025: c_uint = 16;
const ETDM_RATE_22050: c_uint = 17;
const ETDM_RATE_44100: c_uint = 18;
const ETDM_RATE_88200: c_uint = 19;
const ETDM_RATE_176400: c_uint = 20;
const ETDM_RATE_352800: c_uint = 21;

const ETDM_CONN_8K: c_uint = 0;
const ETDM_CONN_11K: c_uint = 1;
const ETDM_CONN_12K: c_uint = 2;
const ETDM_CONN_16K: c_uint = 4;
const ETDM_CONN_22K: c_uint = 5;
const ETDM_CONN_24K: c_uint = 6;
const ETDM_CONN_32K: c_uint = 8;
const ETDM_CONN_44K: c_uint = 9;
const ETDM_CONN_48K: c_uint = 10;
const ETDM_CONN_88K: c_uint = 13;
const ETDM_CONN_96K: c_uint = 14;
const ETDM_CONN_176K: c_uint = 17;
const ETDM_CONN_192K: c_uint = 18;
const ETDM_CONN_352K: c_uint = 21;
const ETDM_CONN_384K: c_uint = 22;

const ETDM_WLEN_8_BIT: c_uint = 0x7;
const ETDM_WLEN_16_BIT: c_uint = 0xf;
const ETDM_WLEN_32_BIT: c_uint = 0x1f;

const ETDM_SLAVE_SEL_ETDMIN0_MASTER: c_uint = 0;
const ETDM_SLAVE_SEL_ETDMIN0_SLAVE: c_uint = 1;
const ETDM_SLAVE_SEL_ETDMIN1_MASTER: c_uint = 2;
const ETDM_SLAVE_SEL_ETDMIN1_SLAVE: c_uint = 3;
const ETDM_SLAVE_SEL_ETDMIN2_MASTER: c_uint = 4;
const ETDM_SLAVE_SEL_ETDMIN2_SLAVE: c_uint = 5;
const ETDM_SLAVE_SEL_ETDMIN3_MASTER: c_uint = 6;
const ETDM_SLAVE_SEL_ETDMIN3_SLAVE: c_uint = 7;
const ETDM_SLAVE_SEL_ETDMOUT0_MASTER: c_uint = 8;
const ETDM_SLAVE_SEL_ETDMOUT0_SLAVE: c_uint = 9;
const ETDM_SLAVE_SEL_ETDMOUT1_MASTER: c_uint = 10;
const ETDM_SLAVE_SEL_ETDMOUT1_SLAVE: c_uint = 11;
const ETDM_SLAVE_SEL_ETDMOUT2_MASTER: c_uint = 12;
const ETDM_SLAVE_SEL_ETDMOUT2_SLAVE: c_uint = 13;
const ETDM_SLAVE_SEL_ETDMOUT3_MASTER: c_uint = 14;
const ETDM_SLAVE_SEL_ETDMOUT3_SLAVE: c_uint = 15;

type snd_pcm_format_t = c_int;
type u32 = c_uint;
type size_t = usize;

#[repr(C)]
pub struct mtk_afe_i2s_priv {
    id: c_int,
    rate: c_int, /* for determine which apll to use */
    low_jitter_en: c_int,
    i2s_low_power_mask: c_uint,
    share_property_name: *const c_char,
    share_i2s_id: c_int,
    mclk_id: c_int,
    mclk_rate: c_int,
    mclk_apll: c_int,
    ch_num: c_int,
    sync: c_int,
    ip_mode: c_int,
    slave_mode: c_int,
    lpbk_mode: c_int,
}

extern "C" {
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn regmap_update_bits(regmap: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mt8189_apll1_enable(afe: *mut mtk_base_afe);
    fn mt8189_apll2_enable(afe: *mut mtk_base_afe);
    fn mt8189_apll1_disable(afe: *mut mtk_base_afe);
    fn mt8189_apll2_disable(afe: *mut mtk_base_afe);
    fn mt8189_mck_enable(afe: *mut mtk_base_afe, id: c_int, rate: c_int);
    fn mt8189_mck_disable(afe: *mut mtk_base_afe, id: c_int);
    fn mt8189_get_apll_by_name(afe: *mut mtk_base_afe, name: *const c_char) -> c_int;
    fn mt8189_get_apll_by_rate(afe: *mut mtk_base_afe, rate: c_uint) -> c_int;
    fn mt8189_get_apll_rate(afe: *mut mtk_base_afe, apll: c_int) -> c_int;
    fn of_property_read_string(np: *const device_node, propname: *const c_char, out_string: *mut *const c_char) -> c_int;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

#[repr(C)] pub struct device { of_node: *const device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_component { dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_widget { name: *const c_char, dapm: *mut snd_soc_dapm_context }
#[repr(C)] pub struct snd_soc_dai { id: c_int, dev: *mut device }
#[repr(C)] pub struct snd_soc_dai_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mtk_base_afe {
    dev: *mut device,
    regmap: *mut regmap,
    platform_priv: *mut mt8189_afe_private,
    sub_dais: list_head,
}
#[repr(C)] pub struct mt8189_afe_private { dai_priv: [*mut mtk_afe_i2s_priv; MT8189_DAI_NUM as usize] }
#[repr(C)] pub struct mtk_base_afe_dai {
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
#[repr(C)] pub struct snd_kcontrol_new { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai_driver { _private: [u8; 0] }

unsafe fn get_etdm_wlen(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 16 { ETDM_WLEN_16_BIT } else { ETDM_WLEN_32_BIT }
}

unsafe fn get_etdm_lrck_width(format: snd_pcm_format_t) -> c_uint {
    if snd_pcm_format_physical_width(format) <= 1 {
        return 0;
    }
    /* The valid data bit number should be larger than 7 due to hardware limitation. */
    (snd_pcm_format_physical_width(format) - 1) as c_uint
}

unsafe fn get_etdm_rate(rate: c_uint) -> c_uint {
    match rate {
        8000 => ETDM_RATE_8K,
        12000 => ETDM_RATE_12K,
        16000 => ETDM_RATE_16K,
        24000 => ETDM_RATE_24K,
        32000 => ETDM_RATE_32K,
        48000 => ETDM_RATE_48K,
        64000 => ETDM_RATE_64K,
        96000 => ETDM_RATE_96K,
        128000 => ETDM_RATE_128K,
        192000 => ETDM_RATE_192K,
        256000 => ETDM_RATE_256K,
        384000 => ETDM_RATE_384K,
        11025 => ETDM_RATE_11025,
        22050 => ETDM_RATE_22050,
        44100 => ETDM_RATE_44100,
        88200 => ETDM_RATE_88200,
        176400 => ETDM_RATE_176400,
        352800 => ETDM_RATE_352800,
        _ => 0,
    }
}

unsafe fn get_etdm_inconn_rate(rate: c_uint) -> c_uint {
    match rate {
        8000 => ETDM_CONN_8K,
        12000 => ETDM_CONN_12K,
        16000 => ETDM_CONN_16K,
        24000 => ETDM_CONN_24K,
        32000 => ETDM_CONN_32K,
        48000 => ETDM_CONN_48K,
        96000 => ETDM_CONN_96K,
        192000 => ETDM_CONN_192K,
        384000 => ETDM_CONN_384K,
        11025 => ETDM_CONN_11K,
        22050 => ETDM_CONN_22K,
        44100 => ETDM_CONN_44K,
        88200 => ETDM_CONN_88K,
        176400 => ETDM_CONN_176K,
        352800 => ETDM_CONN_352K,
        _ => 0,
    }
}

unsafe fn get_i2s_id_by_name(_afe: *mut mtk_base_afe, name: *const c_char) -> c_int {
    if strncmp(name, c"I2SIN0".as_ptr(), 6) == 0 {
        MT8189_DAI_I2S_IN0
    } else if strncmp(name, c"I2SIN1".as_ptr(), 6) == 0 {
        MT8189_DAI_I2S_IN1
    } else if strncmp(name, c"I2SOUT0".as_ptr(), 7) == 0 {
        MT8189_DAI_I2S_OUT0
    } else if strncmp(name, c"I2SOUT1".as_ptr(), 7) == 0 {
        MT8189_DAI_I2S_OUT1
    } else if strncmp(name, c"I2SOUT4".as_ptr(), 7) == 0 {
        MT8189_DAI_I2S_OUT4
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

static ETDM_0_3_LOOPBACK_TEXTS: [*const c_char; 4] = [
    c"etdmin0".as_ptr(), c"etdmin1".as_ptr(), c"etdmout0".as_ptr(), c"etdmout1".as_ptr(),
];
static ETDM_LOOPBACK_VALUES: [u32; 4] = [0, 2, 8, 10];

/* ALSA SoC macro declarations translated in macro form:
 * SOC_VALUE_ENUM_SINGLE_DECL(i2sin0_loopback_enum, ETDM_0_3_COWORK_CON1,
 *     ETDM_IN0_SDATA0_SEL_SFT, ETDM_IN0_SDATA0_SEL_MASK, etdm_0_3_loopback_texts,
 *     etdm_loopback_values);
 * SOC_VALUE_ENUM_SINGLE_DECL(i2sin1_loopback_enum, ETDM_0_3_COWORK_CON1,
 *     ETDM_IN1_SDATA0_SEL_SFT, ETDM_IN1_SDATA0_SEL_MASK, etdm_0_3_loopback_texts,
 *     etdm_loopback_values);
 * mtk_dai_i2s_controls[] = {
 *     SOC_ENUM("I2SIN0 Loopback", i2sin0_loopback_enum),
 *     SOC_ENUM("I2SIN1 Loopback", i2sin1_loopback_enum),
 * };
 *
 * I2S virtual mux to output widget
 * If the I2S interface is required but not connected to an actual codec dai,
 * a Dummy_Widget must be used to establish the connection.
 * i2s_mux_map[] = {"Normal", "Dummy_Widget"};
 * i2s_mux_map_value[] = {0, 1};
 * SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL(i2s_mux_map_enum, SND_SOC_NOPM, 0, 1,
 *     i2s_mux_map, i2s_mux_map_value);
 * i2s_in0_mux_control = SOC_DAPM_ENUM("I2S IN0 Select", i2s_mux_map_enum);
 * i2s_in1_mux_control = SOC_DAPM_ENUM("I2S IN1 Select", i2s_mux_map_enum);
 * i2s_out0_mux_control = SOC_DAPM_ENUM("I2S OUT0 Select", i2s_mux_map_enum);
 * i2s_out1_mux_control = SOC_DAPM_ENUM("I2S OUT1 Select", i2s_mux_map_enum);
 * i2s_out4_mux_control = SOC_DAPM_ENUM("I2S OUT4 Select", i2s_mux_map_enum);
 *
 * Mixer control arrays mtk_i2sout0_ch1_mix through mtk_i2sout4_ch8_mix,
 * DAPM widgets mtk_dai_i2s_widgets, and DAPM routes mtk_dai_i2s_routes are
 * direct data initializers in the C source built from SOC_DAPM_* macros. They
 * are preserved here as external static dependencies because their concrete Rust
 * type layout is supplied by the ALSA SoC bindings, not by this isolated file.
 */
extern "C" {
    static mtk_dai_i2s_controls: [snd_kcontrol_new; 2];
    static mtk_dai_i2s_widgets: [snd_soc_dapm_widget; 31];
    static mtk_dai_i2s_routes: [snd_soc_dapm_route; 185];
}

unsafe extern "C" fn mtk_apll_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg((*cmpnt).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_apll_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            if strcmp((*w).name, APLL1_W_NAME) == 0 {
                mt8189_apll1_enable(afe);
            } else {
                mt8189_apll2_enable(afe);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if strcmp((*w).name, APLL1_W_NAME) == 0 {
                mt8189_apll1_disable(afe);
            } else {
                mt8189_apll2_disable(afe);
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_mclk_en_event(w: *mut snd_soc_dapm_widget, _kcontrol: *mut snd_kcontrol, event: c_int) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg((*cmpnt).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_mclk_en_event".as_ptr(), (*w).name, event);

    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);
    if i2s_priv.is_null() {
        return -EINVAL;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => mt8189_mck_enable(afe, (*i2s_priv).mclk_id, (*i2s_priv).mclk_rate),
        SND_SOC_DAPM_POST_PMD => {
            (*i2s_priv).mclk_rate = 0;
            mt8189_mck_disable(afe, (*i2s_priv).mclk_id);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_afe_i2s_share_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*sink).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        return 0;
    }
    if (*i2s_priv).share_i2s_id < 0 {
        return 0;
    }

    ((*i2s_priv).share_i2s_id == get_i2s_id_by_name(afe, (*source).name)) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*sink).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        return 0;
    }

    /* which apll */
    let cur_apll = mt8189_get_apll_by_name(afe, (*source).name);
    /* choose APLL from i2s rate */
    let needed_apll = mt8189_get_apll_by_rate(afe, (*i2s_priv).rate as c_uint);

    (needed_apll == cur_apll) as c_int
}

unsafe extern "C" fn mtk_afe_i2s_mclk_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*sink).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*sink).name);

    if i2s_priv.is_null() {
        return 0;
    }

    let i2s_num = get_i2s_id_by_name(afe, (*source).name);
    if get_i2s_id_by_name(afe, (*sink).name) == i2s_num {
        return ((*i2s_priv).mclk_rate > 0) as c_int;
    }

    /* check if share i2s need mclk */
    if (*i2s_priv).share_i2s_id < 0 {
        return 0;
    }

    if (*i2s_priv).share_i2s_id == i2s_num {
        return ((*i2s_priv).mclk_rate > 0) as c_int;
    }

    0
}

unsafe extern "C" fn mtk_afe_mclk_apll_connect(source: *mut snd_soc_dapm_widget, sink: *mut snd_soc_dapm_widget) -> c_int {
    let w = sink;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let i2s_priv = get_i2s_priv_by_name(afe, (*w).name);

    if i2s_priv.is_null() {
        return 0;
    }

    /* which apll */
    let cur_apll = mt8189_get_apll_by_name(afe, (*source).name);

    ((*i2s_priv).mclk_apll == cur_apll) as c_int
}

/* i2s dai ops */
unsafe fn mtk_dai_i2s_config(afe: *mut mtk_base_afe, params: *mut snd_pcm_hw_params, i2s_id: c_int) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let rate = params_rate(params);
    let format = params_format(params);
    let mut ret: c_int;

    if i2s_id >= MT8189_DAI_NUM || i2s_id < 0 {
        return -EINVAL;
    }

    let i2s_priv = (*afe_priv).dai_priv[i2s_id as usize];
    if i2s_priv.is_null() {
        return -EINVAL;
    }

    (*i2s_priv).rate = rate as c_int;

    dev_dbg((*afe).dev, c"%s(), id %d, rate %d, format %d\n".as_ptr(),
            c"mtk_dai_i2s_config".as_ptr(), i2s_id, rate, format);

    match i2s_id {
        MT8189_DAI_I2S_IN0 => {
            /* ---etdm in --- */
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON1, REG_INITIAL_COUNT_MASK_SFT, 0x5 << REG_INITIAL_COUNT_SFT);
            /* 3: pad top 5: no pad top */
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON1, REG_INITIAL_POINT_MASK_SFT, 0x5 << REG_INITIAL_POINT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON1, REG_LRCK_RESET_MASK_SFT, 0x1 << REG_LRCK_RESET_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON2, REG_CLOCK_SOURCE_SEL_MASK_SFT, ETDM_CLK_SOURCE_APLL << REG_CLOCK_SOURCE_SEL_SFT);
            /* 0: manual 1: auto */
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON2, REG_CK_EN_SEL_AUTO_MASK_SFT, 0x1 << REG_CK_EN_SEL_AUTO_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON3, REG_FS_TIMING_SEL_MASK_SFT, get_etdm_rate(rate) << REG_FS_TIMING_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON4, REG_RELATCH_1X_EN_SEL_MASK_SFT, get_etdm_inconn_rate(rate) << REG_RELATCH_1X_EN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON8, REG_ETDM_USE_AFIFO_MASK_SFT, 0x0 << REG_ETDM_USE_AFIFO_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON8, REG_AFIFO_MODE_MASK_SFT, 0x0 << REG_AFIFO_MODE_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON9, REG_ALMOST_END_CH_COUNT_MASK_SFT, 0x0 << REG_ALMOST_END_CH_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON9, REG_ALMOST_END_BIT_COUNT_MASK_SFT, 0x0 << REG_ALMOST_END_BIT_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON9, REG_OUT2LATCH_TIME_MASK_SFT, 0x6 << REG_OUT2LATCH_TIME_SFT);
            /* 5:  TDM Mode */
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON0, REG_FMT_MASK_SFT, 0x0 << REG_FMT_SFT);
            /* APLL */
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON0, REG_RELATCH_1X_EN_DOMAIN_SEL_MASK_SFT, ETDM_RELATCH_SEL_APLL << REG_RELATCH_1X_EN_DOMAIN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON0, REG_BIT_LENGTH_MASK_SFT, get_etdm_lrck_width(format) << REG_BIT_LENGTH_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN0_CON0, REG_WORD_LENGTH_MASK_SFT, get_etdm_wlen(format) << REG_WORD_LENGTH_SFT);
            /* ---etdm cowork --- */
            regmap_update_bits((*afe).regmap, ETDM_0_3_COWORK_CON0, ETDM_IN0_SLAVE_SEL_MASK_SFT, ETDM_SLAVE_SEL_ETDMOUT0_MASTER << ETDM_IN0_SLAVE_SEL_SFT);
        }
        MT8189_DAI_I2S_IN1 => {
            /* ---etdm in --- */
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON1, REG_INITIAL_COUNT_MASK_SFT, 0x5 << REG_INITIAL_COUNT_SFT);
            /* 3: pad top 5: no pad top */
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON1, REG_INITIAL_POINT_MASK_SFT, 0x5 << REG_INITIAL_POINT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON1, REG_LRCK_RESET_MASK_SFT, 0x1 << REG_LRCK_RESET_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON2, REG_CLOCK_SOURCE_SEL_MASK_SFT, ETDM_CLK_SOURCE_APLL << REG_CLOCK_SOURCE_SEL_SFT);
            /* 0: manual 1: auto */
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON2, REG_CK_EN_SEL_AUTO_MASK_SFT, 0x1 << REG_CK_EN_SEL_AUTO_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON3, REG_FS_TIMING_SEL_MASK_SFT, get_etdm_rate(rate) << REG_FS_TIMING_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON4, REG_RELATCH_1X_EN_SEL_MASK_SFT, get_etdm_inconn_rate(rate) << REG_RELATCH_1X_EN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON8, REG_ETDM_USE_AFIFO_MASK_SFT, 0x0 << REG_ETDM_USE_AFIFO_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON8, REG_AFIFO_MODE_MASK_SFT, 0x0 << REG_AFIFO_MODE_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON9, REG_ALMOST_END_CH_COUNT_MASK_SFT, 0x0 << REG_ALMOST_END_CH_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON9, REG_ALMOST_END_BIT_COUNT_MASK_SFT, 0x0 << REG_ALMOST_END_BIT_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON9, REG_OUT2LATCH_TIME_MASK_SFT, 0x6 << REG_OUT2LATCH_TIME_SFT);
            /* 5:  TDM Mode */
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON0, REG_FMT_MASK_SFT, 0x0 << REG_FMT_SFT);
            /* APLL */
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON0, REG_RELATCH_1X_EN_DOMAIN_SEL_MASK_SFT, ETDM_RELATCH_SEL_APLL << REG_RELATCH_1X_EN_DOMAIN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON0, REG_BIT_LENGTH_MASK_SFT, get_etdm_lrck_width(format) << REG_BIT_LENGTH_SFT);
            regmap_update_bits((*afe).regmap, ETDM_IN1_CON0, REG_WORD_LENGTH_MASK_SFT, get_etdm_wlen(format) << REG_WORD_LENGTH_SFT);
            /* ---etdm cowork --- */
            regmap_update_bits((*afe).regmap, ETDM_0_3_COWORK_CON1, ETDM_IN1_SLAVE_SEL_MASK_SFT, ETDM_SLAVE_SEL_ETDMOUT1_MASTER << ETDM_IN1_SLAVE_SEL_SFT);
        }
        MT8189_DAI_I2S_OUT0 => {
            /* ---etdm out --- */
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON1, OUT_REG_INITIAL_COUNT_MASK_SFT, 0x5 << OUT_REG_INITIAL_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON1, OUT_REG_INITIAL_POINT_MASK_SFT, 0x6 << OUT_REG_INITIAL_POINT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON1, OUT_REG_LRCK_RESET_MASK_SFT, 0x1 << OUT_REG_LRCK_RESET_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON4, OUT_REG_FS_TIMING_SEL_MASK_SFT, get_etdm_rate(rate) << OUT_REG_FS_TIMING_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON4, OUT_REG_CLOCK_SOURCE_SEL_MASK_SFT, ETDM_CLK_SOURCE_APLL << OUT_REG_CLOCK_SOURCE_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON4, OUT_REG_RELATCH_EN_SEL_MASK_SFT, get_etdm_inconn_rate(rate) << OUT_REG_RELATCH_EN_SEL_SFT);
            /* 5:  TDM Mode */
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON0, OUT_REG_FMT_MASK_SFT, 0x0 << OUT_REG_FMT_SFT);
            /* APLL */
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON0, OUT_REG_RELATCH_DOMAIN_SEL_MASK_SFT, ETDM_RELATCH_SEL_APLL << OUT_REG_RELATCH_DOMAIN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON0, OUT_REG_BIT_LENGTH_MASK_SFT, get_etdm_lrck_width(format) << OUT_REG_BIT_LENGTH_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT0_CON0, OUT_REG_WORD_LENGTH_MASK_SFT, get_etdm_wlen(format) << OUT_REG_WORD_LENGTH_SFT);
            /* ---etdm cowork --- */
            regmap_update_bits((*afe).regmap, ETDM_0_3_COWORK_CON0, ETDM_OUT0_SLAVE_SEL_MASK_SFT, ETDM_SLAVE_SEL_ETDMIN0_MASTER << ETDM_OUT0_SLAVE_SEL_SFT);
        }
        MT8189_DAI_I2S_OUT1 => {
            /* ---etdm out --- */
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON1, OUT_REG_INITIAL_COUNT_MASK_SFT, 0x5 << OUT_REG_INITIAL_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON1, OUT_REG_INITIAL_POINT_MASK_SFT, 0x6 << OUT_REG_INITIAL_POINT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON1, OUT_REG_LRCK_RESET_MASK_SFT, 0x1 << OUT_REG_LRCK_RESET_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON4, OUT_REG_FS_TIMING_SEL_MASK_SFT, get_etdm_rate(rate) << OUT_REG_FS_TIMING_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON4, OUT_REG_CLOCK_SOURCE_SEL_MASK_SFT, ETDM_CLK_SOURCE_APLL << OUT_REG_CLOCK_SOURCE_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON4, OUT_REG_RELATCH_EN_SEL_MASK_SFT, get_etdm_inconn_rate(rate) << OUT_REG_RELATCH_EN_SEL_SFT);
            /* 5:  TDM Mode */
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON0, OUT_REG_FMT_MASK_SFT, 0x0 << OUT_REG_FMT_SFT);
            /* APLL */
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON0, OUT_REG_RELATCH_DOMAIN_SEL_MASK_SFT, ETDM_RELATCH_SEL_APLL << OUT_REG_RELATCH_DOMAIN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON0, OUT_REG_BIT_LENGTH_MASK_SFT, get_etdm_lrck_width(format) << OUT_REG_BIT_LENGTH_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT1_CON0, OUT_REG_WORD_LENGTH_MASK_SFT, get_etdm_wlen(format) << OUT_REG_WORD_LENGTH_SFT);
            /* ---etdm cowork --- */
            regmap_update_bits((*afe).regmap, ETDM_0_3_COWORK_CON0, ETDM_OUT1_SLAVE_SEL_MASK_SFT, ETDM_SLAVE_SEL_ETDMIN1_MASTER << ETDM_OUT1_SLAVE_SEL_SFT);
        }
        MT8189_DAI_I2S_OUT4 => {
            /* ---etdm out --- */
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON1, OUT_REG_INITIAL_COUNT_MASK_SFT, 0x5 << OUT_REG_INITIAL_COUNT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON1, OUT_REG_INITIAL_POINT_MASK_SFT, 0x6 << OUT_REG_INITIAL_POINT_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON1, OUT_REG_LRCK_RESET_MASK_SFT, 0x1 << OUT_REG_LRCK_RESET_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON4, OUT_REG_FS_TIMING_SEL_MASK_SFT, get_etdm_rate(rate) << OUT_REG_FS_TIMING_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON4, OUT_REG_CLOCK_SOURCE_SEL_MASK_SFT, ETDM_CLK_SOURCE_APLL << OUT_REG_CLOCK_SOURCE_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON4, OUT_REG_RELATCH_EN_SEL_MASK_SFT, get_etdm_inconn_rate(rate) << OUT_REG_RELATCH_EN_SEL_SFT);
            /* 5:  TDM Mode */
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON0, OUT_REG_FMT_MASK_SFT, 0x0 << OUT_REG_FMT_SFT);
            /* APLL */
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON0, OUT_REG_RELATCH_DOMAIN_SEL_MASK_SFT, ETDM_RELATCH_SEL_APLL << OUT_REG_RELATCH_DOMAIN_SEL_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON0, OUT_REG_BIT_LENGTH_MASK_SFT, get_etdm_lrck_width(format) << OUT_REG_BIT_LENGTH_SFT);
            regmap_update_bits((*afe).regmap, ETDM_OUT4_CON0, OUT_REG_WORD_LENGTH_MASK_SFT, get_etdm_wlen(format) << OUT_REG_WORD_LENGTH_SFT);
        }
        _ => {
            dev_err((*afe).dev, c"%s(), id %d not support\n".as_ptr(), c"mtk_dai_i2s_config".as_ptr(), i2s_id);
            return -EINVAL;
        }
    }

    /* set share i2s */
    if (*i2s_priv).share_i2s_id >= 0 {
        ret = mtk_dai_i2s_config(afe, params, (*i2s_priv).share_i2s_id);
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn mtk_dai_i2s_hw_params(_substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    mtk_dai_i2s_config(afe, params, (*dai).id)
}

unsafe extern "C" fn mtk_dai_i2s_set_sysclk(dai: *mut snd_soc_dai, _clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let afe = dev_get_drvdata((*dai).dev);
    let afe_priv = (*afe).platform_priv;

    if (*dai).id >= MT8189_DAI_NUM || (*dai).id < 0 || dir != SND_SOC_CLOCK_OUT {
        return -EINVAL;
    }

    let i2s_priv = (*afe_priv).dai_priv[(*dai).id as usize];
    if i2s_priv.is_null() {
        return -EINVAL;
    }

    dev_dbg((*afe).dev, c"%s(), freq %d\n".as_ptr(), c"mtk_dai_i2s_set_sysclk".as_ptr(), freq);

    let apll = mt8189_get_apll_by_rate(afe, freq);
    let apll_rate = mt8189_get_apll_rate(afe, apll);

    if freq > apll_rate as c_uint || (apll_rate as c_uint) % freq != 0 {
        dev_err((*afe).dev, c"%s(), freq %d, apll_rate %d\n".as_ptr(),
                c"mtk_dai_i2s_set_sysclk".as_ptr(), freq, apll_rate);
        return -EINVAL;
    }

    (*i2s_priv).mclk_rate = freq as c_int;
    (*i2s_priv).mclk_apll = apll;

    if (*i2s_priv).share_i2s_id > 0 {
        let share_i2s_priv = (*afe_priv).dai_priv[(*i2s_priv).share_i2s_id as usize];
        if share_i2s_priv.is_null() {
            return -EINVAL;
        }

        (*share_i2s_priv).mclk_rate = (*i2s_priv).mclk_rate;
        (*share_i2s_priv).mclk_apll = (*i2s_priv).mclk_apll;
    }

    0
}

static MTK_DAI_I2S_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_i2s_hw_params),
    set_sysclk: Some(mtk_dai_i2s_set_sysclk),
};

/* dai driver */
const MTK_ETDM_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const MTK_ETDM_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* MT8189_I2S_DAI(_name, _id, max_ch, dir) macro translated as the corresponding
 * snd_soc_dai_driver initializers. The concrete snd_soc_dai_driver layout is
 * supplied externally by ASoC bindings.
 */
extern "C" {
    static mut mtk_dai_i2s_driver: [snd_soc_dai_driver; 5];
}

static MT8189_I2S_PRIV: [mtk_afe_i2s_priv; DAI_I2S_NUM] = [
    mtk_afe_i2s_priv {
        id: MT8189_DAI_I2S_IN0,
        rate: 0,
        low_jitter_en: 0,
        i2s_low_power_mask: 0,
        share_property_name: c"i2sin0-share".as_ptr(),
        share_i2s_id: MT8189_DAI_I2S_OUT0,
        mclk_id: MT8189_I2SIN0_MCK,
        mclk_rate: 0,
        mclk_apll: 0,
        ch_num: 0,
        sync: 0,
        ip_mode: 0,
        slave_mode: 0,
        lpbk_mode: 0,
    },
    mtk_afe_i2s_priv {
        id: MT8189_DAI_I2S_IN1,
        rate: 0,
        low_jitter_en: 0,
        i2s_low_power_mask: 0,
        share_property_name: c"i2sin1-share".as_ptr(),
        share_i2s_id: MT8189_DAI_I2S_OUT1,
        mclk_id: MT8189_I2SIN1_MCK,
        mclk_rate: 0,
        mclk_apll: 0,
        ch_num: 0,
        sync: 0,
        ip_mode: 0,
        slave_mode: 0,
        lpbk_mode: 0,
    },
    mtk_afe_i2s_priv {
        id: MT8189_DAI_I2S_OUT0,
        rate: 0,
        low_jitter_en: 0,
        i2s_low_power_mask: 0,
        share_property_name: c"i2sout0-share".as_ptr(),
        share_i2s_id: -1,
        mclk_id: MT8189_I2SOUT0_MCK,
        mclk_rate: 0,
        mclk_apll: 0,
        ch_num: 0,
        sync: 0,
        ip_mode: 0,
        slave_mode: 0,
        lpbk_mode: 0,
    },
    mtk_afe_i2s_priv {
        id: MT8189_DAI_I2S_OUT1,
        rate: 0,
        low_jitter_en: 0,
        i2s_low_power_mask: 0,
        share_property_name: c"i2sout1-share".as_ptr(),
        share_i2s_id: -1,
        mclk_id: MT8189_I2SOUT1_MCK,
        mclk_rate: 0,
        mclk_apll: 0,
        ch_num: 0,
        sync: 0,
        ip_mode: 0,
        slave_mode: 0,
        lpbk_mode: 0,
    },
    mtk_afe_i2s_priv {
        id: MT8189_DAI_I2S_OUT4,
        rate: 0,
        low_jitter_en: 0,
        i2s_low_power_mask: 0,
        share_property_name: c"i2sout4-share".as_ptr(),
        share_i2s_id: -1,
        mclk_id: MT8189_I2SIN1_MCK,
        mclk_rate: 0,
        mclk_apll: 0,
        ch_num: 0,
        sync: 0,
        ip_mode: 0,
        slave_mode: 0,
        lpbk_mode: 0,
    },
];

unsafe fn mt8189_dai_i2s_get_share(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let of_node = (*(*afe).dev).of_node;

    for i in 0..DAI_I2S_NUM {
        let mut of_str: *const c_char = ptr::null();
        let i2s_priv = (*afe_priv).dai_priv[MT8189_I2S_PRIV[i].id as usize];
        let property_name = MT8189_I2S_PRIV[i].share_property_name;

        if of_property_read_string(of_node, property_name, &mut of_str) != 0 {
            continue;
        }

        (*i2s_priv).share_i2s_id = get_i2s_id_by_name(afe, of_str);
    }

    0
}

unsafe fn init_i2s_priv_data(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;

    for i in 0..DAI_I2S_NUM {
        let id = MT8189_I2S_PRIV[i].id;
        let size = size_of::<mtk_afe_i2s_priv>();

        if id >= MT8189_DAI_NUM || id < 0 {
            return -EINVAL;
        }

        let i2s_priv = devm_kzalloc((*afe).dev, size, GFP_KERNEL) as *mut mtk_afe_i2s_priv;
        if i2s_priv.is_null() {
            return -ENOMEM;
        }

        memcpy(i2s_priv as *mut c_void, &MT8189_I2S_PRIV[i] as *const _ as *const c_void, size);

        (*afe_priv).dai_priv[id as usize] = i2s_priv;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8189_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int {
    let mut ret: c_int;

    let dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    (*dai).dai_drivers = mtk_dai_i2s_driver.as_mut_ptr();
    (*dai).num_dai_drivers = 5;

    (*dai).controls = mtk_dai_i2s_controls.as_ptr();
    (*dai).num_controls = 2;
    (*dai).dapm_widgets = mtk_dai_i2s_widgets.as_ptr();
    (*dai).num_dapm_widgets = 31;
    (*dai).dapm_routes = mtk_dai_i2s_routes.as_ptr();
    (*dai).num_dapm_routes = 185;

    /* set all dai i2s private data */
    ret = init_i2s_priv_data(afe);
    if ret != 0 {
        return ret;
    }

    /* parse share i2s */
    ret = mt8189_dai_i2s_get_share(afe);
    if ret != 0 {
        return ret;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
