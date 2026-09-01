// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI ADDA Control
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 */

// Dependencies from the original C source:
// linux/delay.h, linux/regmap.h, mt8195-afe-clk.h, mt8195-afe-common.h,
// mt8195-reg.h, and ../common/mtk-dai-adda-common.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ADDA_DL_GAIN_LOOPBACK: c_uint = 0x1800;
const ADDA_HIRES_THRES: c_uint = 48000;

const SUPPLY_SEQ_CLOCK_SEL: c_int = 0;
const SUPPLY_SEQ_CLOCK_ON: c_int = 1;
const SUPPLY_SEQ_ADDA_DL_ON: c_int = 2;
const SUPPLY_SEQ_ADDA_MTKAIF_CFG: c_int = 3;
const SUPPLY_SEQ_ADDA_UL_ON: c_int = 4;
const SUPPLY_SEQ_ADDA_AFE_ON: c_int = 5;

const MTK_AFE_ADDA: c_int = 0;
const MTK_AFE_ADDA6: c_int = 1;

#[repr(C)]
struct mtk_dai_adda_priv {
    hires_required: bool,
}

extern "C" {
    fn regmap_update_bits(regmap: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(regmap: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int;
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn usleep_range(min: c_uint, max: c_uint);
    fn strstr(s: *const c_char, find: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mt8195_afe_set_clk_parent(afe: *mut mtk_base_afe, clk: *mut clk, parent: *mut clk) -> c_int;
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct mtk_base_afe {
    dev: *mut c_void,
    regmap: *mut c_void,
    platform_priv: *mut mt8195_afe_private,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8195_afe_private {
    mtkaif_params: mtkaif_param,
    clk: [*mut clk; MT8195_CLK_NUM],
    dai_priv: [*mut c_void; MT8195_AFE_IO_NUM],
}

#[repr(C)]
struct mtkaif_param {
    mtkaif_calibration_ok: bool,
    mtkaif_phase_cycle: [c_int; MT8195_MTKAIF_MISO_NUM],
    mtkaif_dmic_on: c_int,
    mtkaif_adda6_only: c_int,
}

enum clk {}
enum snd_soc_component {}
enum snd_kcontrol {}
enum snd_soc_dapm_context {}
enum snd_pcm_hw_params {}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_union,
}

#[repr(C)]
union snd_ctl_elem_value_union {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

type c_long = isize;

type snd_kcontrol_new = c_void;
type snd_soc_dapm_route = c_void;
type snd_soc_dapm_widget_desc = c_void;

#[repr(C)]
struct snd_soc_dai_ops {
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget_desc,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}

unsafe fn mt8195_adda_mtkaif_init(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;
    let mut delay_data: c_int;
    let mut delay_cycle: c_int;
    let mut mask: c_uint;
    let mut val: c_uint;

    /* set rx protocol 2 & mtkaif_rxif_clkinv_adc inverse */
    mask = MTKAIF_RXIF_CLKINV_ADC | MTKAIF_RXIF_PROTOCOL2;
    val = MTKAIF_RXIF_CLKINV_ADC | MTKAIF_RXIF_PROTOCOL2;

    regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, mask, val);
    regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_CFG0, mask, val);

    mask = RG_RX_PROTOCOL2;
    val = RG_RX_PROTOCOL2;
    regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, mask, val);

    if !(*param).mtkaif_calibration_ok {
        dev_info((*afe).dev, c"%s(), calibration fail\n".as_ptr(), c"mt8195_adda_mtkaif_init".as_ptr());
        return 0;
    }

    /* set delay for ch1, ch2 */
    if (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_0]
        >= (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1]
    {
        delay_data = DELAY_DATA_MISO1;
        delay_cycle = (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_0]
            - (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1];
    } else {
        delay_data = DELAY_DATA_MISO0;
        delay_cycle = (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1]
            - (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_0];
    }

    val = 0;
    mask = MTKAIF_RXIF_DELAY_DATA | MTKAIF_RXIF_DELAY_CYCLE_MASK;
    val |= MTKAIF_RXIF_DELAY_CYCLE(delay_cycle) & MTKAIF_RXIF_DELAY_CYCLE_MASK;
    val |= (delay_data as c_uint) << MTKAIF_RXIF_DELAY_DATA_SHIFT;
    regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG2, mask, val);

    /* set delay between ch3 and ch2 */
    if (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_2]
        >= (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1]
    {
        delay_data = DELAY_DATA_MISO1;
        delay_cycle = (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_2]
            - (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1];
    } else {
        delay_data = DELAY_DATA_MISO2;
        delay_cycle = (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_1]
            - (*param).mtkaif_phase_cycle[MT8195_MTKAIF_MISO_2];
    }

    val = 0;
    mask = MTKAIF_RXIF_DELAY_DATA | MTKAIF_RXIF_DELAY_CYCLE_MASK;
    val |= MTKAIF_RXIF_DELAY_CYCLE(delay_cycle) & MTKAIF_RXIF_DELAY_CYCLE_MASK;
    val |= (delay_data as c_uint) << MTKAIF_RXIF_DELAY_DATA_SHIFT;
    regmap_update_bits((*afe).regmap, AFE_ADDA6_MTKAIF_RX_CFG2, mask, val);

    0
}

unsafe extern "C" fn mtk_adda_mtkaif_cfg_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg((*afe).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_adda_mtkaif_cfg_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8195_adda_mtkaif_init(afe);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_adda_dl_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg((*afe).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_adda_dl_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_ul_mictype(afe: *mut mtk_base_afe, adda: c_int, dmic: bool) {
    let reg: c_uint;
    let mask: c_uint;
    let mut val: c_uint = 0;

    match adda {
        MTK_AFE_ADDA => {
            reg = AFE_ADDA_UL_SRC_CON0;
        }
        MTK_AFE_ADDA6 => {
            reg = AFE_ADDA6_UL_SRC_CON0;
        }
        _ => {
            dev_info((*afe).dev, c"%s(), wrong parameter\n".as_ptr(), c"mtk_adda_ul_mictype".as_ptr());
            return;
        }
    }

    mask = UL_SDM3_LEVEL_CTL | UL_MODE_3P25M_CH1_CTL | UL_MODE_3P25M_CH2_CTL;

    /* turn on dmic, ch1, ch2 */
    if dmic {
        val = mask;
    }

    regmap_update_bits((*afe).regmap, reg, mask, val);
}

unsafe extern "C" fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;

    dev_dbg((*afe).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_adda_ul_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mtk_adda_ul_mictype(afe, MTK_AFE_ADDA, (*param).mtkaif_dmic_on != 0);
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_adda6_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;
    let val: c_uint;

    dev_dbg((*afe).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_adda6_ul_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mtk_adda_ul_mictype(afe, MTK_AFE_ADDA6, (*param).mtkaif_dmic_on != 0);

            val = if (*param).mtkaif_adda6_only != 0 {
                ADDA6_MTKAIF_RX_SYNC_WORD2_DISABLE
            } else {
                0
            };

            regmap_update_bits(
                (*afe).regmap,
                AFE_ADDA_MTKAIF_SYNCWORD_CFG,
                ADDA6_MTKAIF_RX_SYNC_WORD2_DISABLE,
                val,
            );
        }
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_audio_hires_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let clk = (*afe_priv).clk[MT8195_CLK_TOP_AUDIO_H_SEL];
    let clk_parent: *mut clk;

    dev_dbg((*afe).dev, c"%s(), name %s, event 0x%x\n".as_ptr(), c"mtk_audio_hires_event".as_ptr(), (*w).name, event);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            clk_parent = (*afe_priv).clk[MT8195_CLK_TOP_APLL1];
        }
        SND_SOC_DAPM_POST_PMD => {
            clk_parent = (*afe_priv).clk[MT8195_CLK_XTAL_26M];
        }
        _ => return 0,
    }
    mt8195_afe_set_clk_parent(afe, clk, clk_parent);

    0
}

unsafe fn get_adda_priv_by_name(
    afe: *mut mtk_base_afe,
    name: *const c_char,
) -> *mut mtk_dai_adda_priv {
    let afe_priv = (*afe).platform_priv;
    let dai_id: c_int;

    if !strstr(name, c"aud_adc_hires".as_ptr()).is_null() {
        dai_id = MT8195_AFE_IO_UL_SRC1;
    } else if !strstr(name, c"aud_adda6_adc_hires".as_ptr()).is_null() {
        dai_id = MT8195_AFE_IO_UL_SRC2;
    } else if !strstr(name, c"aud_dac_hires".as_ptr()).is_null() {
        dai_id = MT8195_AFE_IO_DL_SRC;
    } else {
        return ptr::null_mut();
    }

    (*afe_priv).dai_priv[dai_id as usize] as *mut mtk_dai_adda_priv
}

unsafe extern "C" fn mtk_afe_adda_hires_connect(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = source;
    let cmpnt = snd_soc_dapm_to_component((*w).dapm as *mut c_void);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let adda_priv: *mut mtk_dai_adda_priv;

    adda_priv = get_adda_priv_by_name(afe, (*w).name);

    if adda_priv.is_null() {
        dev_info((*afe).dev, c"adda_priv == NULL".as_ptr());
        return 0;
    }

    if (*adda_priv).hires_required { 1 } else { 0 }
}

static mtk_dai_adda_o176_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("I000 Switch", AFE_CONN176, 0, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I002 Switch", AFE_CONN176, 2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I020 Switch", AFE_CONN176, 20, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I022 Switch", AFE_CONN176, 22, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I070 Switch", AFE_CONN176_2, 6, 1, 0),
];

static mtk_dai_adda_o177_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("I001 Switch", AFE_CONN177, 1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I003 Switch", AFE_CONN177, 3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I021 Switch", AFE_CONN177, 21, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I023 Switch", AFE_CONN177, 23, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I071 Switch", AFE_CONN177_2, 7, 1, 0),
];

static adda_dlgain_mux_map: &[*const c_char] = &[
    c"Bypass".as_ptr(),
    c"Connect".as_ptr(),
];

SOC_ENUM_SINGLE_DECL!(
    adda_dlgain_mux_map_enum,
    SND_SOC_NOPM,
    0,
    adda_dlgain_mux_map
);

static adda_dlgain_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("DL_GAIN_MUX", adda_dlgain_mux_map_enum);

static mtk_dai_adda_widgets: &[snd_soc_dapm_widget_desc] = &[
    SND_SOC_DAPM_MIXER!("I168", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I169", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I170", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I171", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("O176", SND_SOC_NOPM, 0, 0, mtk_dai_adda_o176_mix, mtk_dai_adda_o176_mix.len()),
    SND_SOC_DAPM_MIXER!("O177", SND_SOC_NOPM, 0, 0, mtk_dai_adda_o177_mix, mtk_dai_adda_o177_mix.len()),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Enable", SUPPLY_SEQ_ADDA_AFE_ON, AFE_ADDA_UL_DL_CON0, ADDA_AFE_ON_SHIFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Playback Enable", SUPPLY_SEQ_ADDA_DL_ON, AFE_ADDA_DL_SRC2_CON0, DL_2_SRC_ON_TMP_CTRL_PRE_SHIFT, 0, mtk_adda_dl_event, SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA_UL_SRC_CON0, UL_SRC_ON_TMP_CTL_SHIFT, 0, mtk_adda_ul_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA6 Capture Enable", SUPPLY_SEQ_ADDA_UL_ON, AFE_ADDA6_UL_SRC_CON0, UL_SRC_ON_TMP_CTL_SHIFT, 0, mtk_adda6_ul_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("AUDIO_HIRES", SUPPLY_SEQ_CLOCK_SEL, SND_SOC_NOPM, 0, 0, mtk_audio_hires_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_SUPPLY_S!("ADDA_MTKAIF_CFG", SUPPLY_SEQ_ADDA_MTKAIF_CFG, SND_SOC_NOPM, 0, 0, mtk_adda_mtkaif_cfg_event, SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_MUX!("DL_GAIN_MUX", SND_SOC_NOPM, 0, 0, &adda_dlgain_mux_control),
    SND_SOC_DAPM_PGA!("DL_GAIN", AFE_ADDA_DL_SRC2_CON0, DL_2_GAIN_ON_CTL_PRE_SHIFT, 0, ptr::null(), 0),
    SND_SOC_DAPM_INPUT!("ADDA_INPUT"),
    SND_SOC_DAPM_OUTPUT!("ADDA_OUTPUT"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adc"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adda6_adc"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac_hires"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adc_hires"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adda6_adc_hires"),
];

static mtk_dai_adda_routes: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "ADDA Enable"),
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "ADDA Capture Enable"),
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "ADDA_MTKAIF_CFG"),
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "aud_adc"),
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "aud_adc_hires", mtk_afe_adda_hires_connect),
    snd_soc_dapm_route!("aud_adc_hires", ptr::null(), "AUDIO_HIRES"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "ADDA Enable"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "ADDA6 Capture Enable"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "ADDA_MTKAIF_CFG"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "aud_adda6_adc"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "aud_adda6_adc_hires", mtk_afe_adda_hires_connect),
    snd_soc_dapm_route!("aud_adda6_adc_hires", ptr::null(), "AUDIO_HIRES"),
    snd_soc_dapm_route!("I168", ptr::null(), "ADDA Capture"),
    snd_soc_dapm_route!("I169", ptr::null(), "ADDA Capture"),
    snd_soc_dapm_route!("I170", ptr::null(), "ADDA6 Capture"),
    snd_soc_dapm_route!("I171", ptr::null(), "ADDA6 Capture"),
    snd_soc_dapm_route!("ADDA Playback", ptr::null(), "ADDA Enable"),
    snd_soc_dapm_route!("ADDA Playback", ptr::null(), "ADDA Playback Enable"),
    snd_soc_dapm_route!("ADDA Playback", ptr::null(), "aud_dac"),
    snd_soc_dapm_route!("ADDA Playback", ptr::null(), "aud_dac_hires", mtk_afe_adda_hires_connect),
    snd_soc_dapm_route!("aud_dac_hires", ptr::null(), "AUDIO_HIRES"),
    snd_soc_dapm_route!("DL_GAIN", ptr::null(), "O176"),
    snd_soc_dapm_route!("DL_GAIN", ptr::null(), "O177"),
    snd_soc_dapm_route!("DL_GAIN_MUX", "Bypass", "O176"),
    snd_soc_dapm_route!("DL_GAIN_MUX", "Bypass", "O177"),
    snd_soc_dapm_route!("DL_GAIN_MUX", "Connect", "DL_GAIN"),
    snd_soc_dapm_route!("ADDA Playback", ptr::null(), "DL_GAIN_MUX"),
    snd_soc_dapm_route!("O176", "I000 Switch", "I000"),
    snd_soc_dapm_route!("O177", "I001 Switch", "I001"),
    snd_soc_dapm_route!("O176", "I002 Switch", "I002"),
    snd_soc_dapm_route!("O177", "I003 Switch", "I003"),
    snd_soc_dapm_route!("O176", "I020 Switch", "I020"),
    snd_soc_dapm_route!("O177", "I021 Switch", "I021"),
    snd_soc_dapm_route!("O176", "I022 Switch", "I022"),
    snd_soc_dapm_route!("O177", "I023 Switch", "I023"),
    snd_soc_dapm_route!("O176", "I070 Switch", "I070"),
    snd_soc_dapm_route!("O177", "I071 Switch", "I071"),
    snd_soc_dapm_route!("ADDA Capture", ptr::null(), "ADDA_INPUT"),
    snd_soc_dapm_route!("ADDA6 Capture", ptr::null(), "ADDA_INPUT"),
    snd_soc_dapm_route!("ADDA_OUTPUT", ptr::null(), "ADDA Playback"),
];

unsafe extern "C" fn mt8195_adda_dl_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(component);
    let reg = AFE_ADDA_DL_SRC2_CON1;
    let mask = DL_2_GAIN_CTL_PRE_MASK;
    let value = (*ucontrol).value.integer.value[0] as c_uint;

    regmap_update_bits((*afe).regmap, reg, mask, DL_2_GAIN_CTL_PRE(value));
    0
}

unsafe extern "C" fn mt8195_adda_dl_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(component);
    let reg = AFE_ADDA_DL_SRC2_CON1;
    let mask = DL_2_GAIN_CTL_PRE_MASK;
    let mut value: c_uint = 0;

    regmap_read((*afe).regmap, reg, &mut value);

    (*ucontrol).value.integer.value[0] = ((value & mask) >> DL_2_GAIN_CTL_PRE_SHIFT) as c_long;
    0
}

unsafe extern "C" fn mt8195_adda6_only_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;

    (*ucontrol).value.integer.value[0] = (*param).mtkaif_adda6_only as c_long;
    0
}

unsafe extern "C" fn mt8195_adda6_only_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;
    let mtkaif_adda6_only: c_int;

    mtkaif_adda6_only = (*ucontrol).value.integer.value[0] as c_int;

    dev_info((*afe).dev, c"%s(), kcontrol name %s, mtkaif_adda6_only %d\n".as_ptr(), c"mt8195_adda6_only_set".as_ptr(), snd_kcontrol_id_name(kcontrol), mtkaif_adda6_only);

    (*param).mtkaif_adda6_only = mtkaif_adda6_only;

    0
}

unsafe extern "C" fn mt8195_adda_dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;

    (*ucontrol).value.integer.value[0] = (*param).mtkaif_dmic_on as c_long;
    0
}

unsafe extern "C" fn mt8195_adda_dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let param = &mut (*afe_priv).mtkaif_params as *mut mtkaif_param;
    let dmic_on: c_int;

    dmic_on = (*ucontrol).value.integer.value[0] as c_int;

    dev_dbg((*afe).dev, c"%s(), kcontrol name %s, dmic_on %d\n".as_ptr(), c"mt8195_adda_dmic_set".as_ptr(), snd_kcontrol_id_name(kcontrol), dmic_on);

    (*param).mtkaif_dmic_on = dmic_on;
    0
}

static mtk_dai_adda_controls: &[snd_kcontrol_new] = &[
    SOC_SINGLE_EXT!("ADDA_DL_Gain", SND_SOC_NOPM, 0, 65535, 0, mt8195_adda_dl_gain_get, mt8195_adda_dl_gain_put),
    SOC_SINGLE_BOOL_EXT!("MTKAIF_DMIC", 0, mt8195_adda_dmic_get, mt8195_adda_dmic_set),
    SOC_SINGLE_BOOL_EXT!("MTKAIF_ADDA6_ONLY", 0, mt8195_adda6_only_get, mt8195_adda6_only_set),
];

unsafe fn mtk_dai_da_configure(afe: *mut mtk_base_afe, rate: c_uint, _id: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;

    /* set sampling rate */
    mask |= DL_2_INPUT_MODE_CTL_MASK;
    val |= DL_2_INPUT_MODE_CTL(mtk_adda_dl_rate_transform(afe, rate));

    /* turn off saturation */
    mask |= DL_2_CH1_SATURATION_EN_CTL;
    mask |= DL_2_CH2_SATURATION_EN_CTL;

    /* turn off mute function */
    mask |= DL_2_MUTE_CH1_OFF_CTL_PRE;
    mask |= DL_2_MUTE_CH2_OFF_CTL_PRE;
    val |= DL_2_MUTE_CH1_OFF_CTL_PRE;
    val |= DL_2_MUTE_CH2_OFF_CTL_PRE;

    /* set voice input data if input sample rate is 8k or 16k */
    mask |= DL_2_VOICE_MODE_CTL_PRE;
    if rate == 8000 || rate == 16000 {
        val |= DL_2_VOICE_MODE_CTL_PRE;
    }

    regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, mask, val);

    mask = 0;
    val = 0;

    /* new 2nd sdm */
    mask |= DL_USE_NEW_2ND_SDM;
    val |= DL_USE_NEW_2ND_SDM;
    regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, mask, val);

    0
}

unsafe fn mtk_dai_ad_configure(afe: *mut mtk_base_afe, rate: c_uint, id: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;

    mask |= UL_VOICE_MODE_CTL_MASK;
    val |= UL_VOICE_MODE_CTL(mtk_adda_ul_rate_transform(afe, rate));

    match id {
        MT8195_AFE_IO_UL_SRC1 => {
            regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, mask, val);
        }
        MT8195_AFE_IO_UL_SRC2 => {
            regmap_update_bits((*afe).regmap, AFE_ADDA6_UL_SRC_CON0, mask, val);
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let adda_priv: *mut mtk_dai_adda_priv;
    let rate = params_rate(params);
    let ret: c_int;

    if (*dai).id != MT8195_AFE_IO_DL_SRC
        && (*dai).id != MT8195_AFE_IO_UL_SRC1
        && (*dai).id != MT8195_AFE_IO_UL_SRC2
    {
        return -EINVAL;
    }
    adda_priv = (*afe_priv).dai_priv[(*dai).id as usize] as *mut mtk_dai_adda_priv;

    dev_dbg((*afe).dev, c"%s(), id %d, stream %d, rate %d\n".as_ptr(), c"mtk_dai_adda_hw_params".as_ptr(), (*dai).id, (*substream).stream, rate);

    if rate > ADDA_HIRES_THRES {
        (*adda_priv).hires_required = true;
    } else {
        (*adda_priv).hires_required = false;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = mtk_dai_da_configure(afe, rate, (*dai).id);
    } else {
        ret = mtk_dai_ad_configure(afe, rate, (*dai).id);
    }

    ret
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
};

/* dai driver */
const MTK_ADDA_PLAYBACK_RATES: c_uint =
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000;

const MTK_ADDA_CAPTURE_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_192000;

const MTK_ADDA_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: c"DL_SRC".as_ptr(),
        id: MT8195_AFE_IO_DL_SRC,
        playback: snd_soc_pcm_stream {
            stream_name: c"ADDA Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: c"UL_SRC1".as_ptr(),
        id: MT8195_AFE_IO_UL_SRC1,
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
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
        name: c"UL_SRC2".as_ptr(),
        id: MT8195_AFE_IO_UL_SRC2,
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"ADDA6 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
];

unsafe fn init_adda_priv_data(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut adda_priv: *mut mtk_dai_adda_priv;
    static adda_dai_list: [c_int; 3] = [
        MT8195_AFE_IO_DL_SRC,
        MT8195_AFE_IO_UL_SRC1,
        MT8195_AFE_IO_UL_SRC2,
    ];
    let mut i: usize;

    i = 0;
    while i < adda_dai_list.len() {
        adda_priv = devm_kzalloc(
            (*afe).dev,
            core::mem::size_of::<mtk_dai_adda_priv>(),
            GFP_KERNEL,
        ) as *mut mtk_dai_adda_priv;
        if adda_priv.is_null() {
            return -ENOMEM;
        }

        (*afe_priv).dai_priv[adda_dai_list[i] as usize] = adda_priv as *mut c_void;
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8195_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
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

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as c_uint;

    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as c_uint;
    (*dai).controls = mtk_dai_adda_controls.as_ptr();
    (*dai).num_controls = mtk_dai_adda_controls.len() as c_uint;

    init_adda_priv_data(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
