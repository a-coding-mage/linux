// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI ADDA Control
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 *         Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

// Dependencies from the original C includes:
// linux/bitfield.h, linux/delay.h, linux/regmap.h,
// mt8188-afe-clk.h, mt8188-afe-common.h, mt8188-reg.h,
// and ../common/mtk-dai-adda-common.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ADDA_HIRES_THRES: c_uint = 48000;

const SUPPLY_SEQ_ADDA_DL_ON: c_int = 0;
const SUPPLY_SEQ_ADDA_MTKAIF_CFG: c_int = 1;
const SUPPLY_SEQ_ADDA_UL_ON: c_int = 2;
const SUPPLY_SEQ_ADDA_AFE_ON: c_int = 3;

#[repr(C)]
struct mtk_dai_adda_priv {
    hires_required: bool,
}

unsafe extern "C" {
    fn regmap_set_bits(map: *mut c_void, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut c_void, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_dapm_to_component(dapm: *mut c_void) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut c_void, head: *mut c_void);
}

#[repr(C)]
struct mtk_base_afe {
    platform_priv: *mut mt8188_afe_private,
    regmap: *mut c_void,
    dev: *mut c_void,
    sub_dais: c_void,
}

#[repr(C)]
struct mt8188_afe_private {
    mtkaif_params: mtkaif_param,
    dai_priv: [*mut mtk_dai_adda_priv; MT8188_AFE_IO_NUM],
}

#[repr(C)]
struct mtkaif_param {
    mtkaif_calibration_ok: bool,
    mtkaif_phase_cycle: [c_int; MT8188_MTKAIF_MISO_NUM],
    mtkaif_dmic_on: c_int,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    dapm: *mut c_void,
}

#[repr(C)]
struct snd_kcontrol {
    id: snd_ctl_elem_id,
}

#[repr(C)]
struct snd_ctl_elem_id {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

type c_long = core::ffi::c_long;

#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
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
}

#[repr(C)]
struct snd_pcm_stream {
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
    playback: snd_pcm_stream,
    capture: snd_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: c_void,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
    connected:
        Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int>,
}

#[repr(C)]
struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}

type snd_soc_dapm_widget_item = snd_soc_dapm_widget;

const MT8188_AFE_IO_DL_SRC: usize = 0;
const MT8188_AFE_IO_UL_SRC: usize = 1;
const MT8188_AFE_IO_NUM: usize = 2;
const MT8188_MTKAIF_MISO_0: usize = 0;
const MT8188_MTKAIF_MISO_1: usize = 1;
const MT8188_MTKAIF_MISO_NUM: usize = 2;

unsafe fn FIELD_PREP(mask: c_uint, val: c_int) -> c_uint {
    ((val as c_uint) << mask.trailing_zeros()) & mask
}

unsafe extern "C" fn mt8188_adda_mtkaif_init(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv };
    let param = unsafe { &mut (*afe_priv).mtkaif_params as *mut mtkaif_param };
    let delay_data: c_int;
    let delay_cycle: c_int;
    let mut mask: c_uint = 0;
    let mut val: c_uint = 0;

    /* set rx protocol 2 & mtkaif_rxif_clkinv_adc inverse */
    unsafe {
        regmap_set_bits(
            (*afe).regmap,
            AFE_ADDA_MTKAIF_CFG0,
            MTKAIF_RXIF_CLKINV_ADC | MTKAIF_RXIF_PROTOCOL2,
        );
        regmap_set_bits((*afe).regmap, AFE_AUD_PAD_TOP, RG_RX_PROTOCOL2);
    }

    if unsafe { !(*param).mtkaif_calibration_ok } {
        unsafe { dev_info((*afe).dev, c"%s(), calibration fail\n".as_ptr(), c"mt8188_adda_mtkaif_init".as_ptr()) };
        return 0;
    }

    /* set delay for ch1, ch2 */
    if unsafe {
        (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_0]
            >= (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_1]
    } {
        delay_data = DELAY_DATA_MISO1;
        delay_cycle = unsafe {
            (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_0]
                - (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_1]
        };
    } else {
        delay_data = DELAY_DATA_MISO0;
        delay_cycle = unsafe {
            (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_1]
                - (*param).mtkaif_phase_cycle[MT8188_MTKAIF_MISO_0]
        };
    }

    mask = MTKAIF_RXIF_DELAY_DATA | MTKAIF_RXIF_DELAY_CYCLE_MASK;
    val |= unsafe { FIELD_PREP(MTKAIF_RXIF_DELAY_CYCLE_MASK, delay_cycle) };
    val |= unsafe { FIELD_PREP(MTKAIF_RXIF_DELAY_DATA, delay_data) };
    unsafe { regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG2, mask, val) };

    0
}

unsafe extern "C" fn mtk_adda_mtkaif_cfg_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };

    unsafe {
        dev_dbg(
            (*afe).dev,
            c"%s(), name %s, event 0x%x\n".as_ptr(),
            c"mtk_adda_mtkaif_cfg_event".as_ptr(),
            (*w).name,
            event,
        );
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => unsafe {
            mt8188_adda_mtkaif_init(afe);
        },
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_adda_dl_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };

    unsafe {
        dev_dbg(
            (*afe).dev,
            c"%s(), name %s, event 0x%x\n".as_ptr(),
            c"mtk_adda_dl_event".as_ptr(),
            (*w).name,
            event,
        );
    }

    match event {
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            unsafe { usleep_range(125, 135) };
        }
        _ => {}
    }

    0
}

unsafe fn mtk_adda_ul_mictype(afe: *mut mtk_base_afe, dmic: bool) {
    let reg: c_uint = AFE_ADDA_UL_SRC_CON0;
    let val: c_uint;

    val = UL_SDM3_LEVEL_CTL | UL_MODE_3P25M_CH1_CTL | UL_MODE_3P25M_CH2_CTL;

    /* turn on dmic, ch1, ch2 */
    if dmic {
        unsafe { regmap_set_bits((*afe).regmap, reg, val) };
    } else {
        unsafe { regmap_clear_bits((*afe).regmap, reg, val) };
    }
}

unsafe extern "C" fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };
    let afe_priv = unsafe { (*afe).platform_priv };
    let param = unsafe { &mut (*afe_priv).mtkaif_params as *mut mtkaif_param };

    unsafe {
        dev_dbg(
            (*afe).dev,
            c"%s(), name %s, event 0x%x\n".as_ptr(),
            c"mtk_adda_ul_event".as_ptr(),
            (*w).name,
            event,
        );
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => unsafe {
            mtk_adda_ul_mictype(afe, (*param).mtkaif_dmic_on != 0);
        },
        SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            unsafe { usleep_range(125, 135) };
        }
        _ => {}
    }

    0
}

unsafe fn get_adda_priv_by_name(
    afe: *mut mtk_base_afe,
    name: *const c_char,
) -> *mut mtk_dai_adda_priv {
    let afe_priv = unsafe { (*afe).platform_priv };

    if unsafe { !strstr(name, c"aud_adc_hires".as_ptr()).is_null() } {
        unsafe { (*afe_priv).dai_priv[MT8188_AFE_IO_UL_SRC] }
    } else if unsafe { !strstr(name, c"aud_dac_hires".as_ptr()).is_null() } {
        unsafe { (*afe_priv).dai_priv[MT8188_AFE_IO_DL_SRC] }
    } else {
        ptr::null_mut()
    }
}

unsafe extern "C" fn mtk_afe_adda_hires_connect(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let w = source;
    let cmpnt = unsafe { snd_soc_dapm_to_component((*w).dapm) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };
    let adda_priv: *mut mtk_dai_adda_priv;

    adda_priv = unsafe { get_adda_priv_by_name(afe, (*w).name) };

    if adda_priv.is_null() {
        unsafe { dev_dbg((*afe).dev, c"adda_priv == NULL".as_ptr()) };
        return 0;
    }

    if unsafe { (*adda_priv).hires_required } { 1 } else { 0 }
}

// Macro-created ASoC controls/widgets are kept as dependency-facing constants.
static mtk_dai_adda_o176_mix: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I000 Switch", AFE_CONN176, 0, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I002 Switch", AFE_CONN176, 2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I020 Switch", AFE_CONN176, 20, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I022 Switch", AFE_CONN176, 22, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I070 Switch", AFE_CONN176_2, 6, 1, 0),
];

static mtk_dai_adda_o177_mix: [snd_kcontrol_new; 5] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("I001 Switch", AFE_CONN177, 1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I003 Switch", AFE_CONN177, 3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I021 Switch", AFE_CONN177, 21, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I023 Switch", AFE_CONN177, 23, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I071 Switch", AFE_CONN177_2, 7, 1, 0),
];

static adda_dlgain_mux_map: [*const c_char; 2] = [c"Bypass".as_ptr(), c"Connect".as_ptr()];

SOC_ENUM_SINGLE_DECL!(
    adda_dlgain_mux_map_enum,
    SND_SOC_NOPM,
    0,
    adda_dlgain_mux_map
);

static adda_dlgain_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("DL_GAIN_MUX", adda_dlgain_mux_map_enum);

static mtk_dai_adda_widgets: [snd_soc_dapm_widget_item; 17] = [
    SND_SOC_DAPM_MIXER!("I168", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("I169", SND_SOC_NOPM, 0, 0, ptr::null(), 0),
    SND_SOC_DAPM_MIXER!(
        "O176",
        SND_SOC_NOPM,
        0,
        0,
        mtk_dai_adda_o176_mix.as_ptr(),
        mtk_dai_adda_o176_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "O177",
        SND_SOC_NOPM,
        0,
        0,
        mtk_dai_adda_o177_mix.as_ptr(),
        mtk_dai_adda_o177_mix.len()
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "ADDA Enable",
        SUPPLY_SEQ_ADDA_AFE_ON,
        AFE_ADDA_UL_DL_CON0,
        ADDA_AFE_ON_SHIFT,
        0,
        None,
        0
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "ADDA Playback Enable",
        SUPPLY_SEQ_ADDA_DL_ON,
        AFE_ADDA_DL_SRC2_CON0,
        DL_2_SRC_ON_TMP_CTRL_PRE_SHIFT,
        0,
        Some(mtk_adda_dl_event),
        SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "ADDA Capture Enable",
        SUPPLY_SEQ_ADDA_UL_ON,
        AFE_ADDA_UL_SRC_CON0,
        UL_SRC_ON_TMP_CTL_SHIFT,
        0,
        Some(mtk_adda_ul_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY_S!(
        "ADDA_MTKAIF_CFG",
        SUPPLY_SEQ_ADDA_MTKAIF_CFG,
        SND_SOC_NOPM,
        0,
        0,
        Some(mtk_adda_mtkaif_cfg_event),
        SND_SOC_DAPM_PRE_PMU
    ),
    SND_SOC_DAPM_MUX!("DL_GAIN_MUX", SND_SOC_NOPM, 0, 0, &adda_dlgain_mux_control),
    SND_SOC_DAPM_PGA!(
        "DL_GAIN",
        AFE_ADDA_DL_SRC2_CON0,
        DL_2_GAIN_ON_CTL_PRE_SHIFT,
        0,
        ptr::null(),
        0
    ),
    SND_SOC_DAPM_INPUT!("ADDA_INPUT"),
    SND_SOC_DAPM_OUTPUT!("ADDA_OUTPUT"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adc"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_dac_hires"),
    SND_SOC_DAPM_CLOCK_SUPPLY!("aud_adc_hires"),
];

static mtk_dai_adda_routes: [snd_soc_dapm_route; 33] = [
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"ADDA Enable".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"ADDA Capture Enable".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"ADDA_MTKAIF_CFG".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"aud_adc".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"aud_adc_hires".as_ptr(), connected: Some(mtk_afe_adda_hires_connect) },
    snd_soc_dapm_route { sink: c"I168".as_ptr(), control: ptr::null(), source: c"ADDA Capture".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"I169".as_ptr(), control: ptr::null(), source: c"ADDA Capture".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Playback".as_ptr(), control: ptr::null(), source: c"ADDA Enable".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Playback".as_ptr(), control: ptr::null(), source: c"ADDA Playback Enable".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Playback".as_ptr(), control: ptr::null(), source: c"aud_dac".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Playback".as_ptr(), control: ptr::null(), source: c"aud_dac_hires".as_ptr(), connected: Some(mtk_afe_adda_hires_connect) },
    snd_soc_dapm_route { sink: c"DL_GAIN".as_ptr(), control: ptr::null(), source: c"O176".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DL_GAIN".as_ptr(), control: ptr::null(), source: c"O177".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DL_GAIN_MUX".as_ptr(), control: c"Bypass".as_ptr(), source: c"O176".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DL_GAIN_MUX".as_ptr(), control: c"Bypass".as_ptr(), source: c"O177".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"DL_GAIN_MUX".as_ptr(), control: c"Connect".as_ptr(), source: c"DL_GAIN".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Playback".as_ptr(), control: ptr::null(), source: c"DL_GAIN_MUX".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O176".as_ptr(), control: c"I000 Switch".as_ptr(), source: c"I000".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O177".as_ptr(), control: c"I001 Switch".as_ptr(), source: c"I001".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O176".as_ptr(), control: c"I002 Switch".as_ptr(), source: c"I002".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O177".as_ptr(), control: c"I003 Switch".as_ptr(), source: c"I003".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O176".as_ptr(), control: c"I020 Switch".as_ptr(), source: c"I020".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O177".as_ptr(), control: c"I021 Switch".as_ptr(), source: c"I021".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O176".as_ptr(), control: c"I022 Switch".as_ptr(), source: c"I022".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O177".as_ptr(), control: c"I023 Switch".as_ptr(), source: c"I023".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O176".as_ptr(), control: c"I070 Switch".as_ptr(), source: c"I070".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"O177".as_ptr(), control: c"I071 Switch".as_ptr(), source: c"I071".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA Capture".as_ptr(), control: ptr::null(), source: c"ADDA_INPUT".as_ptr(), connected: None },
    snd_soc_dapm_route { sink: c"ADDA_OUTPUT".as_ptr(), control: ptr::null(), source: c"ADDA Playback".as_ptr(), connected: None },
];

unsafe extern "C" fn mt8188_adda_dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = unsafe { snd_kcontrol_chip(kcontrol) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };
    let afe_priv = unsafe { (*afe).platform_priv };
    let param = unsafe { &mut (*afe_priv).mtkaif_params as *mut mtkaif_param };

    unsafe {
        (*ucontrol).value.integer.value[0] = (*param).mtkaif_dmic_on as c_long;
    }
    0
}

unsafe extern "C" fn mt8188_adda_dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = unsafe { snd_kcontrol_chip(kcontrol) };
    let afe = unsafe { snd_soc_component_get_drvdata(cmpnt) };
    let afe_priv = unsafe { (*afe).platform_priv };
    let param = unsafe { &mut (*afe_priv).mtkaif_params as *mut mtkaif_param };
    let dmic_on: c_int;

    dmic_on = if unsafe { (*ucontrol).value.integer.value[0] != 0 } { 1 } else { 0 };

    unsafe {
        dev_dbg(
            (*afe).dev,
            c"%s(), kcontrol name %s, dmic_on %d\n".as_ptr(),
            c"mt8188_adda_dmic_set".as_ptr(),
            (*kcontrol).id.name,
            dmic_on,
        );
    }

    if unsafe { (*param).mtkaif_dmic_on == dmic_on } {
        return 0;
    }

    unsafe {
        (*param).mtkaif_dmic_on = dmic_on;
    }
    1
}

static mtk_dai_adda_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE!(
        "ADDA_DL_GAIN",
        AFE_ADDA_DL_SRC2_CON1,
        DL_2_GAIN_CTL_PRE_SHIFT,
        65535,
        0
    ),
    SOC_SINGLE_BOOL_EXT!(
        "MTKAIF_DMIC Switch",
        0,
        mt8188_adda_dmic_get,
        mt8188_adda_dmic_set
    ),
];

unsafe fn mtk_dai_da_configure(afe: *mut mtk_base_afe, rate: c_uint, _id: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;

    /* set sampling rate */
    mask |= DL_2_INPUT_MODE_CTL_MASK;
    val |= unsafe { FIELD_PREP(DL_2_INPUT_MODE_CTL_MASK, mtk_adda_dl_rate_transform(afe, rate) as c_int) };

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

    unsafe { regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, mask, val) };

    /* new 2nd sdm */
    unsafe { regmap_set_bits((*afe).regmap, AFE_ADDA_DL_SDM_DCCOMP_CON, DL_USE_NEW_2ND_SDM) };

    0
}

unsafe fn mtk_dai_ad_configure(afe: *mut mtk_base_afe, rate: c_uint, _id: c_int) -> c_int {
    let val: c_uint;
    let mask: c_uint;

    mask = UL_VOICE_MODE_CTL_MASK;
    val = unsafe { FIELD_PREP(UL_VOICE_MODE_CTL_MASK, mtk_adda_ul_rate_transform(afe, rate) as c_int) };

    unsafe {
        regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, mask, val);
    }
    0
}

unsafe extern "C" fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) };
    let afe_priv = unsafe { (*afe).platform_priv };
    let adda_priv = unsafe { (*afe_priv).dai_priv[(*dai).id as usize] };
    let rate: c_uint = unsafe { params_rate(params) };
    let id: c_int = unsafe { (*dai).id };
    let ret: c_int;

    unsafe {
        dev_dbg(
            (*afe).dev,
            c"%s(), id %d, stream %d, rate %u\n".as_ptr(),
            c"mtk_dai_adda_hw_params".as_ptr(),
            id,
            (*substream).stream,
            rate,
        );
    }

    unsafe {
        (*adda_priv).hires_required = rate > ADDA_HIRES_THRES;
    }

    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        ret = unsafe { mtk_dai_da_configure(afe, rate, id) };
    } else {
        ret = unsafe { mtk_dai_ad_configure(afe, rate, id) };
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

const MTK_ADDA_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"DL_SRC".as_ptr(),
        id: MT8188_AFE_IO_DL_SRC as c_int,
        playback: snd_pcm_stream {
            stream_name: c"ADDA Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: &mtk_dai_adda_ops,
    },
    snd_soc_dai_driver {
        name: c"UL_SRC".as_ptr(),
        id: MT8188_AFE_IO_UL_SRC as c_int,
        playback: snd_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_pcm_stream {
            stream_name: c"ADDA Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    },
];

unsafe fn init_adda_priv_data(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = unsafe { (*afe).platform_priv };
    let mut adda_priv: *mut mtk_dai_adda_priv;
    let adda_dai_list: [usize; 2] = [MT8188_AFE_IO_DL_SRC, MT8188_AFE_IO_UL_SRC];
    let mut i: usize;

    i = 0;
    while i < adda_dai_list.len() {
        adda_priv = unsafe {
            devm_kzalloc(
                (*afe).dev,
                core::mem::size_of::<mtk_dai_adda_priv>(),
                GFP_KERNEL,
            ) as *mut mtk_dai_adda_priv
        };
        if adda_priv.is_null() {
            return -ENOMEM;
        }

        unsafe {
            (*afe_priv).dai_priv[adda_dai_list[i]] = adda_priv;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8188_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = unsafe {
        devm_kzalloc(
            (*afe).dev,
            core::mem::size_of::<mtk_base_afe_dai>(),
            GFP_KERNEL,
        ) as *mut mtk_base_afe_dai
    };
    if dai.is_null() {
        return -ENOMEM;
    }

    unsafe {
        list_add(&mut (*dai).list as *mut c_void, &mut (*afe).sub_dais as *mut c_void);

        (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
        (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as c_uint;

        (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
        (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as c_uint;
        (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
        (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as c_uint;
        (*dai).controls = mtk_dai_adda_controls.as_ptr();
        (*dai).num_controls = mtk_dai_adda_controls.len() as c_uint;
    }

    unsafe { init_adda_priv_data(afe) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
