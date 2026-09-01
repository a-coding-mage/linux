// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI ADDA Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>
//
// Translated from C. Dependencies originally came from:
// linux/regmap.h, linux/delay.h, mt8183-afe-common.h,
// mt8183-interconnection.h, mt8183-reg.h, and mtk-dai-adda-common.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: c_ulong,
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
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub platform_priv: *mut mt8183_afe_private,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mt8183_afe_private {
    pub mtkaif_dmic: c_int,
    pub mtkaif_protocol: c_int,
    pub mtkaif_phase_cycle: [c_int; 2],
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub kind: c_int,
    pub name: *const c_char,
    pub reg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub enum_data: *const soc_enum,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_enum {
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget_desc {
    pub kind: c_int,
    pub name: *const c_char,
    pub seq: c_int,
    pub reg: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" {
    static AFE_CONN3: c_uint;
    static AFE_CONN4: c_uint;
    static I_DL1_CH1: c_uint;
    static I_DL1_CH2: c_uint;
    static I_DL2_CH1: c_uint;
    static I_DL2_CH2: c_uint;
    static I_DL3_CH1: c_uint;
    static I_DL3_CH2: c_uint;
    static I_ADDA_UL_CH2: c_uint;
    static I_ADDA_UL_CH1: c_uint;
    static I_PCM_1_CAP_CH1: c_uint;
    static I_PCM_2_CAP_CH1: c_uint;
    static I_PCM_1_CAP_CH2: c_uint;
    static I_PCM_2_CAP_CH2: c_uint;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static AFE_ADDA_MTKAIF_RX_CFG0: c_uint;
    static AFE_ADDA_UL_SRC_CON0: c_uint;
    static EINVAL: c_int;
    static SND_SOC_NOPM: c_uint;
    static AFE_ADDA_UL_DL_CON0: c_uint;
    static ADDA_AFE_ON_SFT: c_uint;
    static AFE_ADDA_DL_SRC2_CON0: c_uint;
    static DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_uint;
    static UL_SRC_ON_TMP_CTL_SFT: c_uint;
    static MT8183_MTKAIF_PROTOCOL_2_CLK_P2: c_int;
    static MT8183_MTKAIF_PROTOCOL_2: c_int;
    static MT8183_MTKAIF_PROTOCOL_1: c_int;
    static AFE_AUD_PAD_TOP: c_uint;
    static AFE_ADDA_MTKAIF_CFG0: c_uint;
    static DELAY_DATA_MISO1: c_int;
    static DELAY_DATA_MISO2: c_int;
    static AFE_ADDA_MTKAIF_RX_CFG2: c_uint;
    static MTKAIF_RXIF_DELAY_DATA_MASK_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_DATA_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT: c_uint;
    static MTKAIF_RXIF_DELAY_CYCLE_SFT: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static AFE_ADDA_PREDIS_CON0: c_uint;
    static AFE_ADDA_PREDIS_CON1: c_uint;
    static AFE_ADDA_DL_SRC2_CON1: c_uint;
    static AFE_ADDA_DL_SDM_DCCOMP_CON: c_uint;
    static ATTGAIN_CTL_MASK_SFT: c_uint;
    static ATTGAIN_CTL_SFT: c_uint;
    static AFE_ADDA_TOP_CON0: c_uint;
    static UL_IIR_ON_TMP_CTL_MASK_SFT: c_uint;
    static AFE_ADDA_IIR_COEF_02_01: c_uint;
    static AFE_ADDA_IIR_COEF_04_03: c_uint;
    static AFE_ADDA_IIR_COEF_06_05: c_uint;
    static AFE_ADDA_IIR_COEF_08_07: c_uint;
    static AFE_ADDA_IIR_COEF_10_09: c_uint;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static MT8183_DAI_ADDA: c_int;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
}

const AUDIO_SDM_LEVEL_MUTE: c_uint = 0;
const AUDIO_SDM_LEVEL_NORMAL: c_uint = 0x1d;
/* if you change level normal */
/* you need to change formula of hp impedance and dc trim too */

const KCONTROL_AUTODISABLE: c_int = 1;
const KCONTROL_ENUM_EXT: c_int = 2;
const WIDGET_MIXER: c_int = 1;
const WIDGET_SUPPLY_S: c_int = 2;
const WIDGET_CLOCK_SUPPLY: c_int = 3;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! soc_dapm_single_autodisable {
    ($name:literal, $reg:expr, $shift:expr, $max:expr, $invert:expr) => {
        snd_kcontrol_new {
            kind: KCONTROL_AUTODISABLE,
            name: cstr!($name),
            reg: $reg,
            shift: $shift,
            max: $max,
            invert: $invert,
            enum_data: ptr::null(),
            get: None,
            put: None,
        }
    };
}

macro_rules! snd_soc_dapm_route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route {
            sink: cstr!($sink),
            control: ptr::null(),
            source: cstr!($source),
        }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route {
            sink: cstr!($sink),
            control: cstr!($control),
            source: cstr!($source),
        }
    };
}

static mtk_adda_dl_ch1_mix: [snd_kcontrol_new; 7] = unsafe {
    [
        soc_dapm_single_autodisable!("DL1_CH1", AFE_CONN3, I_DL1_CH1, 1, 0),
        soc_dapm_single_autodisable!("DL2_CH1", AFE_CONN3, I_DL2_CH1, 1, 0),
        soc_dapm_single_autodisable!("DL3_CH1", AFE_CONN3, I_DL3_CH1, 1, 0),
        soc_dapm_single_autodisable!("ADDA_UL_CH2", AFE_CONN3, I_ADDA_UL_CH2, 1, 0),
        soc_dapm_single_autodisable!("ADDA_UL_CH1", AFE_CONN3, I_ADDA_UL_CH1, 1, 0),
        soc_dapm_single_autodisable!("PCM_1_CAP_CH1", AFE_CONN3, I_PCM_1_CAP_CH1, 1, 0),
        soc_dapm_single_autodisable!("PCM_2_CAP_CH1", AFE_CONN3, I_PCM_2_CAP_CH1, 1, 0),
    ]
};

static mtk_adda_dl_ch2_mix: [snd_kcontrol_new; 12] = unsafe {
    [
        soc_dapm_single_autodisable!("DL1_CH1", AFE_CONN4, I_DL1_CH1, 1, 0),
        soc_dapm_single_autodisable!("DL1_CH2", AFE_CONN4, I_DL1_CH2, 1, 0),
        soc_dapm_single_autodisable!("DL2_CH1", AFE_CONN4, I_DL2_CH1, 1, 0),
        soc_dapm_single_autodisable!("DL2_CH2", AFE_CONN4, I_DL2_CH2, 1, 0),
        soc_dapm_single_autodisable!("DL3_CH1", AFE_CONN4, I_DL3_CH1, 1, 0),
        soc_dapm_single_autodisable!("DL3_CH2", AFE_CONN4, I_DL3_CH2, 1, 0),
        soc_dapm_single_autodisable!("ADDA_UL_CH2", AFE_CONN4, I_ADDA_UL_CH2, 1, 0),
        soc_dapm_single_autodisable!("ADDA_UL_CH1", AFE_CONN4, I_ADDA_UL_CH1, 1, 0),
        soc_dapm_single_autodisable!("PCM_1_CAP_CH1", AFE_CONN4, I_PCM_1_CAP_CH1, 1, 0),
        soc_dapm_single_autodisable!("PCM_2_CAP_CH1", AFE_CONN4, I_PCM_2_CAP_CH1, 1, 0),
        soc_dapm_single_autodisable!("PCM_1_CAP_CH2", AFE_CONN4, I_PCM_1_CAP_CH2, 1, 0),
        soc_dapm_single_autodisable!("PCM_2_CAP_CH2", AFE_CONN4, I_PCM_2_CAP_CH2, 1, 0),
    ]
};

unsafe extern "C" fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;

    dev_dbg(
        (*afe).dev,
        cstr!("%s(), name %s, event 0x%x\n"),
        cstr!("mtk_adda_ul_event"),
        (*w).name,
        event,
    );

    if event == SND_SOC_DAPM_PRE_PMU {
        /* update setting to dmic */
        if (*afe_priv).mtkaif_dmic != 0 {
            /* mtkaif_rxif_data_mode = 1, dmic */
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x1, 0x1);

            /* dmic mode, 3.25M*/
            regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x0, 0xf << 20);
            regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x0, 0x1 << 5);
            regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x0, 0x3 << 14);

            /* turn on dmic, ch1, ch2 */
            regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x1 << 1, 0x1 << 1);
            regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x3 << 21, 0x3 << 21);
        }
    } else if event == SND_SOC_DAPM_POST_PMD {
        /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
        usleep_range(125, 135);
    }

    0
}

/* mtkaif dmic */
static mt8183_adda_off_on_str: [*const c_char; 2] = [cstr!("Off"), cstr!("On")];

static mt8183_adda_enum: [soc_enum; 1] = [soc_enum {
    items: mt8183_adda_off_on_str.len() as c_uint,
    texts: mt8183_adda_off_on_str.as_ptr(),
}];

unsafe extern "C" fn mt8183_adda_dmic_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;

    (*ucontrol).value.integer.value[0] = (*afe_priv).mtkaif_dmic as i64;

    0
}

unsafe extern "C" fn mt8183_adda_dmic_set(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let e = (*kcontrol).private_value as *mut soc_enum;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    (*afe_priv).mtkaif_dmic = (*ucontrol).value.integer.value[0] as c_int;

    dev_info(
        (*afe).dev,
        cstr!("%s(), kcontrol name %s, mtkaif_dmic %d\n"),
        cstr!("mt8183_adda_dmic_set"),
        (*kcontrol).id.name,
        (*afe_priv).mtkaif_dmic,
    );

    0
}

static mtk_adda_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    kind: KCONTROL_ENUM_EXT,
    name: cstr!("MTKAIF_DMIC"),
    reg: 0,
    shift: 0,
    max: 0,
    invert: 0,
    enum_data: unsafe { &mt8183_adda_enum[0] as *const soc_enum },
    get: Some(mt8183_adda_dmic_get),
    put: Some(mt8183_adda_dmic_set),
}];

const SUPPLY_SEQ_ADDA_AFE_ON: c_int = 0;
const SUPPLY_SEQ_ADDA_DL_ON: c_int = 1;
const SUPPLY_SEQ_ADDA_UL_ON: c_int = 2;

static mtk_dai_adda_widgets: [snd_soc_dapm_widget_desc; 9] = unsafe {
    [
        /* adda */
        snd_soc_dapm_widget_desc {
            kind: WIDGET_MIXER,
            name: cstr!("ADDA_DL_CH1"),
            seq: 0,
            reg: SND_SOC_NOPM,
            shift: 0,
            invert: 0,
            controls: mtk_adda_dl_ch1_mix.as_ptr(),
            num_controls: mtk_adda_dl_ch1_mix.len() as c_uint,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_MIXER,
            name: cstr!("ADDA_DL_CH2"),
            seq: 0,
            reg: SND_SOC_NOPM,
            shift: 0,
            invert: 0,
            controls: mtk_adda_dl_ch2_mix.as_ptr(),
            num_controls: mtk_adda_dl_ch2_mix.len() as c_uint,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_SUPPLY_S,
            name: cstr!("ADDA Enable"),
            seq: SUPPLY_SEQ_ADDA_AFE_ON,
            reg: AFE_ADDA_UL_DL_CON0,
            shift: ADDA_AFE_ON_SFT,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_SUPPLY_S,
            name: cstr!("ADDA Playback Enable"),
            seq: SUPPLY_SEQ_ADDA_DL_ON,
            reg: AFE_ADDA_DL_SRC2_CON0,
            shift: DL_2_SRC_ON_TMP_CTL_PRE_SFT,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_SUPPLY_S,
            name: cstr!("ADDA Capture Enable"),
            seq: SUPPLY_SEQ_ADDA_UL_ON,
            reg: AFE_ADDA_UL_SRC_CON0,
            shift: UL_SRC_ON_TMP_CTL_SFT,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: Some(mtk_adda_ul_event),
            event_flags: SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_CLOCK_SUPPLY,
            name: cstr!("aud_dac_clk"),
            seq: 0,
            reg: 0,
            shift: 0,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_CLOCK_SUPPLY,
            name: cstr!("aud_dac_predis_clk"),
            seq: 0,
            reg: 0,
            shift: 0,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_CLOCK_SUPPLY,
            name: cstr!("aud_adc_clk"),
            seq: 0,
            reg: 0,
            shift: 0,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
        snd_soc_dapm_widget_desc {
            kind: WIDGET_CLOCK_SUPPLY,
            name: cstr!("mtkaif_26m_clk"),
            seq: 0,
            reg: 0,
            shift: 0,
            invert: 0,
            controls: ptr::null(),
            num_controls: 0,
            event: None,
            event_flags: 0,
        },
    ]
};

static mtk_dai_adda_routes: [snd_soc_dapm_route; 20] = [
    /* playback */
    snd_soc_dapm_route!("ADDA_DL_CH1", "DL1_CH1", "DL1"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL1_CH1", "DL1"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL1_CH2", "DL1"),
    snd_soc_dapm_route!("ADDA_DL_CH1", "DL2_CH1", "DL2"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL2_CH1", "DL2"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL2_CH2", "DL2"),
    snd_soc_dapm_route!("ADDA_DL_CH1", "DL3_CH1", "DL3"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL3_CH1", "DL3"),
    snd_soc_dapm_route!("ADDA_DL_CH2", "DL3_CH2", "DL3"),
    snd_soc_dapm_route!("ADDA Playback", NULL, "ADDA_DL_CH1"),
    snd_soc_dapm_route!("ADDA Playback", NULL, "ADDA_DL_CH2"),
    /* adda enable */
    snd_soc_dapm_route!("ADDA Playback", NULL, "ADDA Enable"),
    snd_soc_dapm_route!("ADDA Playback", NULL, "ADDA Playback Enable"),
    snd_soc_dapm_route!("ADDA Capture", NULL, "ADDA Enable"),
    snd_soc_dapm_route!("ADDA Capture", NULL, "ADDA Capture Enable"),
    /* clk */
    snd_soc_dapm_route!("ADDA Playback", NULL, "mtkaif_26m_clk"),
    snd_soc_dapm_route!("ADDA Playback", NULL, "aud_dac_clk"),
    snd_soc_dapm_route!("ADDA Playback", NULL, "aud_dac_predis_clk"),
    snd_soc_dapm_route!("ADDA Capture", NULL, "mtkaif_26m_clk"),
    snd_soc_dapm_route!("ADDA Capture", NULL, "aud_adc_clk"),
];

unsafe fn set_mtkaif_rx(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let delay_data: c_int;
    let delay_cycle: c_int;

    if (*afe_priv).mtkaif_protocol == MT8183_MTKAIF_PROTOCOL_2_CLK_P2 {
        regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x38);
        regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x39);
        /* mtkaif_rxif_clkinv_adc inverse for calibration */
        regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x80010000);

        if (*afe_priv).mtkaif_phase_cycle[0] >= (*afe_priv).mtkaif_phase_cycle[1] {
            delay_data = DELAY_DATA_MISO1;
            delay_cycle = (*afe_priv).mtkaif_phase_cycle[0] - (*afe_priv).mtkaif_phase_cycle[1];
        } else {
            delay_data = DELAY_DATA_MISO2;
            delay_cycle = (*afe_priv).mtkaif_phase_cycle[1] - (*afe_priv).mtkaif_phase_cycle[0];
        }

        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_MTKAIF_RX_CFG2,
            MTKAIF_RXIF_DELAY_DATA_MASK_SFT,
            (delay_data as c_uint) << MTKAIF_RXIF_DELAY_DATA_SFT,
        );

        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_MTKAIF_RX_CFG2,
            MTKAIF_RXIF_DELAY_CYCLE_MASK_SFT,
            (delay_cycle as c_uint) << MTKAIF_RXIF_DELAY_CYCLE_SFT,
        );
    } else if (*afe_priv).mtkaif_protocol == MT8183_MTKAIF_PROTOCOL_2 {
        regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x31);
        regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x00010000);
    } else if (*afe_priv).mtkaif_protocol == MT8183_MTKAIF_PROTOCOL_1 {
        regmap_write((*afe).regmap, AFE_AUD_PAD_TOP, 0x31);
        regmap_write((*afe).regmap, AFE_ADDA_MTKAIF_CFG0, 0x0);
    }

    0
}

unsafe extern "C" fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let rate = params_rate(params);

    dev_dbg(
        (*afe).dev,
        cstr!("%s(), id %d, stream %d, rate %d\n"),
        cstr!("mtk_dai_adda_hw_params"),
        (*dai).id,
        (*substream).stream,
        rate,
    );

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        let mut dl_src2_con0: c_uint = 0;
        let dl_src2_con1: c_uint;

        /* clean predistortion */
        regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON0, 0);
        regmap_write((*afe).regmap, AFE_ADDA_PREDIS_CON1, 0);

        /* set sampling rate */
        dl_src2_con0 = mtk_adda_dl_rate_transform(afe, rate) << 28;

        /* set output mode */
        match rate {
            192000 => {
                dl_src2_con0 |= 0x1 << 24; /* UP_SAMPLING_RATE_X2 */
                dl_src2_con0 |= 1 << 14;
            }
            96000 => {
                dl_src2_con0 |= 0x2 << 24; /* UP_SAMPLING_RATE_X4 */
                dl_src2_con0 |= 1 << 14;
            }
            _ => {
                dl_src2_con0 |= 0x3 << 24; /* UP_SAMPLING_RATE_X8 */
            }
        }

        /* turn off mute function */
        dl_src2_con0 |= 0x03 << 11;

        /* set voice input data if input sample rate is 8k or 16k */
        if rate == 8000 || rate == 16000 {
            dl_src2_con0 |= 0x01 << 5;
        }

        /* SA suggest apply -0.3db to audio/speech path */
        dl_src2_con1 = 0xf74f0000;

        /* turn on down-link gain */
        dl_src2_con0 = dl_src2_con0 | (0x01 << 1);

        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, dl_src2_con0);
        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON1, dl_src2_con1);

        /* set sdm gain */
        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_DL_SDM_DCCOMP_CON,
            ATTGAIN_CTL_MASK_SFT,
            AUDIO_SDM_LEVEL_NORMAL << ATTGAIN_CTL_SFT,
        );
    } else {
        let mut voice_mode: c_uint = 0;
        let mut ul_src_con0: c_uint = 0; /* default value */

        /* set mtkaif protocol */
        set_mtkaif_rx(afe);

        /* Using Internal ADC */
        regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1 << 0, 0x0 << 0);

        voice_mode = mtk_adda_ul_rate_transform(afe, rate);

        ul_src_con0 |= (voice_mode << 17) & (0x7 << 17);

        /* enable iir */
        ul_src_con0 |= (1 << UL_IIR_ON_TMP_CTL_SFT) & UL_IIR_ON_TMP_CTL_MASK_SFT;

        /* 35Hz @ 48k */
        regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_02_01, 0x00000000);
        regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_04_03, 0x00003FB8);
        regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_06_05, 0x3FB80000);
        regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_08_07, 0x3FB80000);
        regmap_write((*afe).regmap, AFE_ADDA_IIR_COEF_10_09, 0x0000C048);

        regmap_write((*afe).regmap, AFE_ADDA_UL_SRC_CON0, ul_src_con0);

        /* mtkaif_rxif_data_mode = 0, amic */
        regmap_update_bits((*afe).regmap, AFE_ADDA_MTKAIF_RX_CFG0, 0x1 << 0, 0x0 << 0);
    }

    0
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
};

/* dai driver */
unsafe fn MTK_ADDA_PLAYBACK_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000
}

unsafe fn MTK_ADDA_CAPTURE_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000
}

unsafe fn MTK_ADDA_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 1] = unsafe {
    [snd_soc_dai_driver {
        name: cstr!("ADDA"),
        id: MT8183_DAI_ADDA,
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("ADDA Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES(),
            formats: MTK_ADDA_FORMATS(),
        },
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("ADDA Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES(),
            formats: MTK_ADDA_FORMATS(),
        },
        ops: &mtk_dai_adda_ops as *const snd_soc_dai_ops,
    }]
};

#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list as *mut list_head, &mut (*afe).sub_dais as *mut list_head);

    (*dai).dai_drivers = mtk_dai_adda_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_adda_driver.len() as c_uint;

    (*dai).controls = mtk_adda_controls.as_ptr();
    (*dai).num_controls = mtk_adda_controls.len() as c_uint;
    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_adda_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_adda_routes.len() as c_uint;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
