// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2020 MediaTek Inc.
// Author: Shane Chien <shane.chien@mediatek.com>
//

// C dependencies translated as external Rust dependencies:
// <linux/regmap.h>
// <sound/pcm_params.h>
// "mt8192-afe-common.h"
// "mt8192-interconnection.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
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
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
    pub active: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
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
    pub symmetric_rate: c_uint,
    pub symmetric_sample_bits: c_uint,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_get_widget_playback(dai: *mut snd_soc_dai) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dai_get_widget_capture(dai: *mut snd_soc_dai) -> *mut snd_soc_dapm_widget;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mt8192_rate_transform(dev: *mut device, rate: c_uint, dai_id: c_int) -> c_uint;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

const AUD_TX_LCH_RPT_NO_REPEAT: c_uint = 0;
const AUD_TX_LCH_RPT_REPEAT: c_uint = 1;

const AUD_VBT_16K_MODE_DISABLE: c_uint = 0;
const AUD_VBT_16K_MODE_ENABLE: c_uint = 1;

const AUD_EXT_MODEM_SELECT_INTERNAL: c_uint = 0;
const AUD_EXT_MODEM_SELECT_EXTERNAL: c_uint = 1;

/* bck sync length = 1 */
const AUD_PCM_ONE_BCK_CYCLE_SYNC: c_uint = 0;
/* bck sync length = PCM_INTF_CON1[9:13] */
const AUD_PCM_EXTENDED_BCK_CYCLE_SYNC: c_uint = 1;

const AUD_BT_MODE_DUAL_MIC_ON_TX: c_uint = 0;
const AUD_BT_MODE_SINGLE_MIC_ON_TX: c_uint = 1;

/* slave mode & external modem uses different crystal */
const AUD_PCM_AFIFO_ASRC: c_uint = 0;
/* slave mode & external modem uses the same crystal */
const AUD_PCM_AFIFO_AFIFO: c_uint = 1;

const AUD_PCM_CLOCK_MASTER_MODE: c_uint = 0;
const AUD_PCM_CLOCK_SLAVE_MODE: c_uint = 1;

const AUD_PCM_WLEN_PCM_32_BCK_CYCLES: c_uint = 0;
const AUD_PCM_WLEN_PCM_64_BCK_CYCLES: c_uint = 1;

const AUD_PCM_MODE_PCM_MODE_8K: c_uint = 0;
const AUD_PCM_MODE_PCM_MODE_16K: c_uint = 1;
const AUD_PCM_MODE_PCM_MODE_32K: c_uint = 2;
const AUD_PCM_MODE_PCM_MODE_48K: c_uint = 3;

const AUD_PCM_FMT_I2S: c_uint = 0;
const AUD_PCM_FMT_EIAJ: c_uint = 1;
const AUD_PCM_FMT_PCM_MODE_A: c_uint = 2;
const AUD_PCM_FMT_PCM_MODE_B: c_uint = 3;

const AUD_BCLK_OUT_INV_NO_INVERSE: c_uint = 0;
const AUD_BCLK_OUT_INV_INVERSE: c_uint = 1;

const AUD_PCM_EN_DISABLE: c_uint = 0;
const AUD_PCM_EN_ENABLE: c_uint = 1;

/* dai component */
static mtk_pcm_1_playback_ch1_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN7, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN7, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN7_1, I_DL4_CH1, 1, 0),
];

static mtk_pcm_1_playback_ch2_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN8, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN8, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN8_1, I_DL4_CH2, 1, 0),
];

static mtk_pcm_1_playback_ch4_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S0_CH1", AFE_CONN27, I_I2S0_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S0_CH2", AFE_CONN27, I_I2S0_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN27, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S2_CH1", AFE_CONN27, I_I2S2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S2_CH2", AFE_CONN27, I_I2S2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN27_1, I_DL4_CH1, 1, 0),
];

static mtk_pcm_2_playback_ch1_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN17, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN17, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN17, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN17, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN17_1, I_DL4_CH1, 1, 0),
];

static mtk_pcm_2_playback_ch2_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN18, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN18, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN18, I_ADDA_UL_CH3, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN18, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN18_1, I_DL4_CH2, 1, 0),
];

static mtk_pcm_2_playback_ch3_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH3", AFE_CONN23, I_ADDA_UL_CH3, 1, 0),
];

static mtk_pcm_2_playback_ch4_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S0_CH1", AFE_CONN24, I_I2S0_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S0_CH2", AFE_CONN24, I_I2S0_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN24, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S2_CH1", AFE_CONN24, I_I2S2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S2_CH2", AFE_CONN24, I_I2S2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH1", AFE_CONN24_1, I_DL4_CH1, 1, 0),
];

static mtk_pcm_2_playback_ch5_mix: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S0_CH2", AFE_CONN25, I_I2S0_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN25, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("I2S2_CH2", AFE_CONN25, I_I2S2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL4_CH2", AFE_CONN25_1, I_DL4_CH2, 1, 0),
];

unsafe extern "C" fn mtk_pcm_en_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;

    dev_info(
        (*afe).dev,
        c"%s(), name %s, event 0x%x\n".as_ptr(),
        c"mtk_pcm_en_event".as_ptr(),
        (*w).name,
        event,
    );
    0
}

static mtk_dai_pcm_widgets: &[snd_soc_dapm_widget] = &[
    /* inter-connections */
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch1_mix,
        mtk_pcm_1_playback_ch1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch2_mix,
        mtk_pcm_1_playback_ch2_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_1_PB_CH4",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_1_playback_ch4_mix,
        mtk_pcm_1_playback_ch4_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH1",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch1_mix,
        mtk_pcm_2_playback_ch1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH2",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch2_mix,
        mtk_pcm_2_playback_ch2_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH3",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch3_mix,
        mtk_pcm_2_playback_ch3_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH4",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch4_mix,
        mtk_pcm_2_playback_ch4_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "PCM_2_PB_CH5",
        SND_SOC_NOPM,
        0,
        0,
        mtk_pcm_2_playback_ch5_mix,
        mtk_pcm_2_playback_ch5_mix.len()
    ),
    SND_SOC_DAPM_SUPPLY!(
        "PCM_1_EN",
        PCM_INTF_CON1,
        PCM_EN_SFT,
        0,
        mtk_pcm_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_SUPPLY!(
        "PCM_2_EN",
        PCM2_INTF_CON,
        PCM2_EN_SFT,
        0,
        mtk_pcm_en_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
    SND_SOC_DAPM_INPUT!("MD1_TO_AFE"),
    SND_SOC_DAPM_INPUT!("MD2_TO_AFE"),
    SND_SOC_DAPM_OUTPUT!("AFE_TO_MD1"),
    SND_SOC_DAPM_OUTPUT!("AFE_TO_MD2"),
];

static mtk_dai_pcm_routes: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ptr::null(), source: c"PCM_1_PB_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ptr::null(), source: c"PCM_1_PB_CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ptr::null(), source: c"PCM_1_PB_CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_PB_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_PB_CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_PB_CH3".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_PB_CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_PB_CH5".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: ptr::null(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: ptr::null(), source: c"PCM_2_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Capture".as_ptr(), control: ptr::null(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Capture".as_ptr(), control: ptr::null(), source: c"PCM_2_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AFE_TO_MD1".as_ptr(), control: ptr::null(), source: c"PCM 2 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"AFE_TO_MD2".as_ptr(), control: ptr::null(), source: c"PCM 1 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Capture".as_ptr(), control: ptr::null(), source: c"MD1_TO_AFE".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Capture".as_ptr(), control: ptr::null(), source: c"MD2_TO_AFE".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH1".as_ptr(), control: c"DL2_CH1".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH2".as_ptr(), control: c"DL2_CH2".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH4".as_ptr(), control: c"DL1_CH1".as_ptr(), source: c"DL1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH1".as_ptr(), control: c"DL2_CH1".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH2".as_ptr(), control: c"DL2_CH2".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH4".as_ptr(), control: c"DL1_CH1".as_ptr(), source: c"DL1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH1".as_ptr(), control: c"DL4_CH1".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH2".as_ptr(), control: c"DL4_CH2".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH4".as_ptr(), control: c"DL4_CH1".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH1".as_ptr(), control: c"DL4_CH1".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH2".as_ptr(), control: c"DL4_CH2".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH4".as_ptr(), control: c"DL4_CH1".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH4".as_ptr(), control: c"I2S0_CH1".as_ptr(), source: c"I2S0".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH4".as_ptr(), control: c"I2S2_CH1".as_ptr(), source: c"I2S2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH5".as_ptr(), control: c"DL1_CH2".as_ptr(), source: c"DL1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH5".as_ptr(), control: c"DL4_CH2".as_ptr(), source: c"DL4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH5".as_ptr(), control: c"I2S0_CH2".as_ptr(), source: c"I2S0".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH5".as_ptr(), control: c"I2S2_CH2".as_ptr(), source: c"I2S2".as_ptr() },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let p = snd_soc_dai_get_widget_playback(dai);
    let c = snd_soc_dai_get_widget_capture(dai);
    let rate = params_rate(params);
    let rate_reg = mt8192_rate_transform((*afe).dev, rate, (*dai).id);
    let mut pcm_con: c_uint = 0;

    dev_info(
        (*afe).dev,
        c"%s(), id %d, stream %d, rate %d, rate_reg %d, widget active p %d, c %d\n".as_ptr(),
        c"mtk_dai_pcm_hw_params".as_ptr(),
        (*dai).id,
        (*substream).stream,
        rate,
        rate_reg,
        (*p).active,
        (*c).active,
    );

    if (*p).active != 0 || (*c).active != 0 {
        return 0;
    }

    match (*dai).id {
        MT8192_DAI_PCM_1 => {
            pcm_con |= AUD_BCLK_OUT_INV_NO_INVERSE << PCM_BCLK_OUT_INV_SFT;
            pcm_con |= AUD_TX_LCH_RPT_NO_REPEAT << PCM_TX_LCH_RPT_SFT;
            pcm_con |= AUD_VBT_16K_MODE_DISABLE << PCM_VBT_16K_MODE_SFT;
            pcm_con |= AUD_EXT_MODEM_SELECT_INTERNAL << PCM_EXT_MODEM_SFT;
            pcm_con |= 0 << PCM_SYNC_LENGTH_SFT;
            pcm_con |= AUD_PCM_ONE_BCK_CYCLE_SYNC << PCM_SYNC_TYPE_SFT;
            pcm_con |= AUD_BT_MODE_DUAL_MIC_ON_TX << PCM_BT_MODE_SFT;
            pcm_con |= AUD_PCM_AFIFO_AFIFO << PCM_BYP_ASRC_SFT;
            pcm_con |= AUD_PCM_CLOCK_SLAVE_MODE << PCM_SLAVE_SFT;
            pcm_con |= rate_reg << PCM_MODE_SFT;
            pcm_con |= AUD_PCM_FMT_PCM_MODE_B << PCM_FMT_SFT;

            regmap_update_bits((*afe).regmap, PCM_INTF_CON1, 0xfffffffe, pcm_con);
        }
        MT8192_DAI_PCM_2 => {
            pcm_con |= AUD_TX_LCH_RPT_NO_REPEAT << PCM2_TX_LCH_RPT_SFT;
            pcm_con |= AUD_VBT_16K_MODE_DISABLE << PCM2_VBT_16K_MODE_SFT;
            pcm_con |= AUD_BT_MODE_DUAL_MIC_ON_TX << PCM2_BT_MODE_SFT;
            pcm_con |= AUD_PCM_AFIFO_AFIFO << PCM2_AFIFO_SFT;
            pcm_con |= AUD_PCM_WLEN_PCM_32_BCK_CYCLES << PCM2_WLEN_SFT;
            pcm_con |= rate_reg << PCM2_MODE_SFT;
            pcm_con |= AUD_PCM_FMT_PCM_MODE_B << PCM2_FMT_SFT;

            regmap_update_bits((*afe).regmap, PCM2_INTF_CON, 0xfffffffe, pcm_con);
        }
        _ => {
            dev_warn(
                (*afe).dev,
                c"%s(), id %d not support\n".as_ptr(),
                c"mtk_dai_pcm_hw_params".as_ptr(),
                (*dai).id,
            );
            return -EINVAL;
        }
    }

    0
}

static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_pcm_hw_params),
};

/* dai driver */
const MTK_PCM_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;

const MTK_PCM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"PCM 1".as_ptr(),
        id: MT8192_DAI_PCM_1,
        playback: snd_soc_pcm_stream {
            stream_name: c"PCM 1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"PCM 1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: c"PCM 2".as_ptr(),
        id: MT8192_DAI_PCM_2,
        playback: snd_soc_pcm_stream {
            stream_name: c"PCM 2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"PCM 2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
];

#[no_mangle]
pub unsafe extern "C" fn mt8192_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dev_info((*afe).dev, c"%s()\n".as_ptr(), c"mt8192_dai_pcm_register".as_ptr());

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_pcm_driver.len() as c_uint;

    (*dai).dapm_widgets = mtk_dai_pcm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_pcm_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_pcm_routes.len() as c_uint;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
