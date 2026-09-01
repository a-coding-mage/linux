// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI I2S Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// Dependencies from the original C includes:
// linux/regmap.h, sound/pcm_params.h,
// mt6797-afe-common.h, mt6797-interconnection.h, mt6797-reg.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
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

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub active: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

pub type HwParamsFn = unsafe extern "C" fn(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<HwParamsFn>,
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
pub enum AUD_TX_LCH_RPT {
    AUD_TX_LCH_RPT_NO_REPEAT = 0,
    AUD_TX_LCH_RPT_REPEAT = 1,
}

#[repr(C)]
pub enum AUD_VBT_16K_MODE {
    AUD_VBT_16K_MODE_DISABLE = 0,
    AUD_VBT_16K_MODE_ENABLE = 1,
}

#[repr(C)]
pub enum AUD_EXT_MODEM {
    AUD_EXT_MODEM_SELECT_INTERNAL = 0,
    AUD_EXT_MODEM_SELECT_EXTERNAL = 1,
}

#[repr(C)]
pub enum AUD_PCM_SYNC_TYPE {
    /* bck sync length = 1 */
    AUD_PCM_ONE_BCK_CYCLE_SYNC = 0,
    /* bck sync length = PCM_INTF_CON1[9:13] */
    AUD_PCM_EXTENDED_BCK_CYCLE_SYNC = 1,
}

#[repr(C)]
pub enum AUD_BT_MODE {
    AUD_BT_MODE_DUAL_MIC_ON_TX = 0,
    AUD_BT_MODE_SINGLE_MIC_ON_TX = 1,
}

#[repr(C)]
pub enum AUD_PCM_AFIFO_SRC {
    /* slave mode & external modem uses different crystal */
    AUD_PCM_AFIFO_ASRC = 0,
    /* slave mode & external modem uses the same crystal */
    AUD_PCM_AFIFO_AFIFO = 1,
}

#[repr(C)]
pub enum AUD_PCM_CLOCK_SOURCE {
    AUD_PCM_CLOCK_MASTER_MODE = 0,
    AUD_PCM_CLOCK_SLAVE_MODE = 1,
}

#[repr(C)]
pub enum AUD_PCM_WLEN {
    AUD_PCM_WLEN_PCM_32_BCK_CYCLES = 0,
    AUD_PCM_WLEN_PCM_64_BCK_CYCLES = 1,
}

#[repr(C)]
pub enum AUD_PCM_MODE {
    AUD_PCM_MODE_PCM_MODE_8K = 0,
    AUD_PCM_MODE_PCM_MODE_16K = 1,
    AUD_PCM_MODE_PCM_MODE_32K = 2,
    AUD_PCM_MODE_PCM_MODE_48K = 3,
}

#[repr(C)]
pub enum AUD_PCM_FMT {
    AUD_PCM_FMT_I2S = 0,
    AUD_PCM_FMT_EIAJ = 1,
    AUD_PCM_FMT_PCM_MODE_A = 2,
    AUD_PCM_FMT_PCM_MODE_B = 3,
}

#[repr(C)]
pub enum AUD_BCLK_OUT_INV {
    AUD_BCLK_OUT_INV_NO_INVERSE = 0,
    AUD_BCLK_OUT_INV_INVERSE = 1,
}

#[repr(C)]
pub enum AUD_PCM_EN {
    AUD_PCM_EN_DISABLE = 0,
    AUD_PCM_EN_ENABLE = 1,
}

extern "C" {
    static mut mtk_pcm_1_playback_ch1_mix: [snd_kcontrol_new; 2];
    static mut mtk_pcm_1_playback_ch2_mix: [snd_kcontrol_new; 2];
    static mut mtk_pcm_1_playback_ch4_mix: [snd_kcontrol_new; 1];
    static mut mtk_pcm_2_playback_ch1_mix: [snd_kcontrol_new; 2];
    static mut mtk_pcm_2_playback_ch2_mix: [snd_kcontrol_new; 2];
    static mut mtk_pcm_2_playback_ch4_mix: [snd_kcontrol_new; 1];

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn snd_soc_dai_get_widget_playback(dai: *mut snd_soc_dai) -> *mut snd_soc_dapm_widget;
    fn snd_soc_dai_get_widget_capture(dai: *mut snd_soc_dai) -> *mut snd_soc_dapm_widget;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mt6797_rate_transform(dev: *mut device, rate: c_uint, id: c_int) -> c_uint;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    static AFE_CONN7: c_uint;
    static AFE_CONN8: c_uint;
    static AFE_CONN17: c_uint;
    static AFE_CONN18: c_uint;
    static AFE_CONN24: c_uint;
    static AFE_CONN27: c_uint;
    static I_ADDA_UL_CH1: c_uint;
    static I_ADDA_UL_CH2: c_uint;
    static I_DL1_CH1: c_uint;
    static I_DL2_CH1: c_uint;
    static I_DL2_CH2: c_uint;
    static SND_SOC_NOPM: c_uint;
    static PCM_INTF_CON1: c_uint;
    static PCM2_INTF_CON: c_uint;
    static PCM_EN_SFT: c_uint;
    static PCM2_EN_SFT: c_uint;
    static PCM_BCLK_OUT_INV_SFT: c_uint;
    static PCM_TX_LCH_RPT_SFT: c_uint;
    static PCM_VBT_16K_MODE_SFT: c_uint;
    static PCM_EXT_MODEM_SFT: c_uint;
    static PCM_SYNC_LENGTH_SFT: c_uint;
    static PCM_SYNC_TYPE_SFT: c_uint;
    static PCM_BT_MODE_SFT: c_uint;
    static PCM_BYP_ASRC_SFT: c_uint;
    static PCM_SLAVE_SFT: c_uint;
    static PCM_MODE_SFT: c_uint;
    static PCM_FMT_SFT: c_uint;
    static PCM2_TX_LCH_RPT_SFT: c_uint;
    static PCM2_VBT_16K_MODE_SFT: c_uint;
    static PCM2_BT_MODE_SFT: c_uint;
    static PCM2_AFIFO_SFT: c_uint;
    static PCM2_WLEN_SFT: c_uint;
    static PCM2_MODE_SFT: c_uint;
    static PCM2_FMT_SFT: c_uint;
    static MT6797_DAI_PCM_1: c_int;
    static MT6797_DAI_PCM_2: c_int;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: c_uint;
}

unsafe extern "C" {
    fn SOC_DAPM_SINGLE_AUTODISABLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_MIXER(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        kcontrol_news: *const snd_kcontrol_new,
        num_kcontrols: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        invert: c_uint,
        event: *mut c_void,
        event_flags: c_uint,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
}

/* dai component */
// The original C file initializes these with ASoC macros:
// SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH1", AFE_CONN7, I_ADDA_UL_CH1, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH1", AFE_CONN7, I_DL2_CH1, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH2", AFE_CONN8, I_ADDA_UL_CH2, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH2", AFE_CONN8, I_DL2_CH2, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL1_CH1", AFE_CONN27, I_DL1_CH1, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH1", AFE_CONN17, I_ADDA_UL_CH1, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH1", AFE_CONN17, I_DL2_CH1, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("ADDA_UL_CH2", AFE_CONN18, I_ADDA_UL_CH2, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL2_CH2", AFE_CONN18, I_DL2_CH2, 1, 0)
// SOC_DAPM_SINGLE_AUTODISABLE("DL1_CH1", AFE_CONN24, I_DL1_CH1, 1, 0)

pub unsafe fn mtk_dai_pcm_widgets() -> [snd_soc_dapm_widget; 12] {
    [
        /* inter-connections */
        SND_SOC_DAPM_MIXER(
            c"PCM_1_PB_CH1".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_1_playback_ch1_mix.as_ptr(),
            mtk_pcm_1_playback_ch1_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            c"PCM_1_PB_CH2".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_1_playback_ch2_mix.as_ptr(),
            mtk_pcm_1_playback_ch2_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            c"PCM_1_PB_CH4".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_1_playback_ch4_mix.as_ptr(),
            mtk_pcm_1_playback_ch4_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            c"PCM_2_PB_CH1".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_2_playback_ch1_mix.as_ptr(),
            mtk_pcm_2_playback_ch1_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            c"PCM_2_PB_CH2".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_2_playback_ch2_mix.as_ptr(),
            mtk_pcm_2_playback_ch2_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            c"PCM_2_PB_CH4".as_ptr(),
            SND_SOC_NOPM,
            0,
            0,
            mtk_pcm_2_playback_ch4_mix.as_ptr(),
            mtk_pcm_2_playback_ch4_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_SUPPLY(c"PCM_1_EN".as_ptr(), PCM_INTF_CON1, PCM_EN_SFT, 0, null_mut(), 0),
        SND_SOC_DAPM_SUPPLY(c"PCM_2_EN".as_ptr(), PCM2_INTF_CON, PCM2_EN_SFT, 0, null_mut(), 0),
        SND_SOC_DAPM_INPUT(c"MD1_TO_AFE".as_ptr()),
        SND_SOC_DAPM_INPUT(c"MD2_TO_AFE".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"AFE_TO_MD1".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"AFE_TO_MD2".as_ptr()),
    ]
}

static mtk_dai_pcm_routes: [snd_soc_dapm_route; 20] = [
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: null_mut(), source: c"PCM_1_PB_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: null_mut(), source: c"PCM_1_PB_CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: null_mut(), source: c"PCM_1_PB_CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: null_mut(), source: c"PCM_2_PB_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: null_mut(), source: c"PCM_2_PB_CH2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: null_mut(), source: c"PCM_2_PB_CH4".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Playback".as_ptr(), control: null_mut(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Playback".as_ptr(), control: null_mut(), source: c"PCM_2_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Capture".as_ptr(), control: null_mut(), source: c"PCM_1_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Capture".as_ptr(), control: null_mut(), source: c"PCM_2_EN".as_ptr() },
    snd_soc_dapm_route { sink: c"AFE_TO_MD1".as_ptr(), control: null_mut(), source: c"PCM 2 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"AFE_TO_MD2".as_ptr(), control: null_mut(), source: c"PCM 1 Playback".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 2 Capture".as_ptr(), control: null_mut(), source: c"MD1_TO_AFE".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM 1 Capture".as_ptr(), control: null_mut(), source: c"MD2_TO_AFE".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH1".as_ptr(), control: c"DL2_CH1".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH2".as_ptr(), control: c"DL2_CH2".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_1_PB_CH4".as_ptr(), control: c"DL1_CH1".as_ptr(), source: c"DL1".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH1".as_ptr(), control: c"DL2_CH1".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH2".as_ptr(), control: c"DL2_CH2".as_ptr(), source: c"DL2".as_ptr() },
    snd_soc_dapm_route { sink: c"PCM_2_PB_CH4".as_ptr(), control: c"DL1_CH1".as_ptr(), source: c"DL1".as_ptr() },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let p: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_playback(dai);
    let c: *mut snd_soc_dapm_widget = snd_soc_dai_get_widget_capture(dai);
    let rate: c_uint = params_rate(params);
    let rate_reg: c_uint = mt6797_rate_transform((*afe).dev, rate, (*dai).id);
    let mut pcm_con: c_uint = 0;

    dev_dbg(
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

    if (*dai).id == MT6797_DAI_PCM_1 {
        pcm_con |= (AUD_BCLK_OUT_INV::AUD_BCLK_OUT_INV_NO_INVERSE as c_uint) << PCM_BCLK_OUT_INV_SFT;
        pcm_con |= (AUD_TX_LCH_RPT::AUD_TX_LCH_RPT_NO_REPEAT as c_uint) << PCM_TX_LCH_RPT_SFT;
        pcm_con |= (AUD_VBT_16K_MODE::AUD_VBT_16K_MODE_DISABLE as c_uint) << PCM_VBT_16K_MODE_SFT;
        pcm_con |= (AUD_EXT_MODEM::AUD_EXT_MODEM_SELECT_INTERNAL as c_uint) << PCM_EXT_MODEM_SFT;
        pcm_con |= 0 << PCM_SYNC_LENGTH_SFT;
        pcm_con |= (AUD_PCM_SYNC_TYPE::AUD_PCM_ONE_BCK_CYCLE_SYNC as c_uint) << PCM_SYNC_TYPE_SFT;
        pcm_con |= (AUD_BT_MODE::AUD_BT_MODE_DUAL_MIC_ON_TX as c_uint) << PCM_BT_MODE_SFT;
        pcm_con |= (AUD_PCM_AFIFO_SRC::AUD_PCM_AFIFO_AFIFO as c_uint) << PCM_BYP_ASRC_SFT;
        pcm_con |= (AUD_PCM_CLOCK_SOURCE::AUD_PCM_CLOCK_SLAVE_MODE as c_uint) << PCM_SLAVE_SFT;
        pcm_con |= rate_reg << PCM_MODE_SFT;
        pcm_con |= (AUD_PCM_FMT::AUD_PCM_FMT_PCM_MODE_B as c_uint) << PCM_FMT_SFT;

        regmap_update_bits((*afe).regmap, PCM_INTF_CON1, 0xfffffffe, pcm_con);
    } else if (*dai).id == MT6797_DAI_PCM_2 {
        pcm_con |= (AUD_TX_LCH_RPT::AUD_TX_LCH_RPT_NO_REPEAT as c_uint) << PCM2_TX_LCH_RPT_SFT;
        pcm_con |= (AUD_VBT_16K_MODE::AUD_VBT_16K_MODE_DISABLE as c_uint) << PCM2_VBT_16K_MODE_SFT;
        pcm_con |= (AUD_BT_MODE::AUD_BT_MODE_DUAL_MIC_ON_TX as c_uint) << PCM2_BT_MODE_SFT;
        pcm_con |= (AUD_PCM_AFIFO_SRC::AUD_PCM_AFIFO_AFIFO as c_uint) << PCM2_AFIFO_SFT;
        pcm_con |= (AUD_PCM_WLEN::AUD_PCM_WLEN_PCM_32_BCK_CYCLES as c_uint) << PCM2_WLEN_SFT;
        pcm_con |= rate_reg << PCM2_MODE_SFT;
        pcm_con |= (AUD_PCM_FMT::AUD_PCM_FMT_PCM_MODE_B as c_uint) << PCM2_FMT_SFT;

        regmap_update_bits((*afe).regmap, PCM2_INTF_CON, 0xfffffffe, pcm_con);
    } else {
        dev_warn(
            (*afe).dev,
            c"%s(), id %d not support\n".as_ptr(),
            c"mtk_dai_pcm_hw_params".as_ptr(),
            (*dai).id,
        );
        return -22;
    }

    0
}

static mtk_dai_pcm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_pcm_hw_params),
};

/* dai driver */
unsafe fn MTK_PCM_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000
}

unsafe fn MTK_PCM_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static mut mtk_dai_pcm_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"PCM 1".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"PCM 1 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"PCM 1 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,
            formats: 0,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
    snd_soc_dai_driver {
        name: c"PCM 2".as_ptr(),
        id: 0,
        playback: snd_soc_pcm_stream {
            stream_name: c"PCM 2 Playback".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"PCM 2 Capture".as_ptr(),
            channels_min: 1,
            channels_max: 2,
            rates: 0,
            formats: 0,
        },
        ops: &mtk_dai_pcm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    },
];

#[no_mangle]
pub unsafe extern "C" fn mt6797_dai_pcm_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -12;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    mtk_dai_pcm_driver[0].id = MT6797_DAI_PCM_1;
    mtk_dai_pcm_driver[0].playback.rates = MTK_PCM_RATES();
    mtk_dai_pcm_driver[0].playback.formats = MTK_PCM_FORMATS();
    mtk_dai_pcm_driver[0].capture.rates = MTK_PCM_RATES();
    mtk_dai_pcm_driver[0].capture.formats = MTK_PCM_FORMATS();

    mtk_dai_pcm_driver[1].id = MT6797_DAI_PCM_2;
    mtk_dai_pcm_driver[1].playback.rates = MTK_PCM_RATES();
    mtk_dai_pcm_driver[1].playback.formats = MTK_PCM_FORMATS();
    mtk_dai_pcm_driver[1].capture.rates = MTK_PCM_RATES();
    mtk_dai_pcm_driver[1].capture.formats = MTK_PCM_FORMATS();

    (*dai).dai_drivers = mtk_dai_pcm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_pcm_driver.len() as c_uint;

    let widgets = mtk_dai_pcm_widgets();
    (*dai).dapm_widgets = widgets.as_ptr();
    (*dai).num_dapm_widgets = widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_pcm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_pcm_routes.len() as c_uint;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
