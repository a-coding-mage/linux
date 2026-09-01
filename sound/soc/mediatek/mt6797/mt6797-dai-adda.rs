// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI ADDA Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// Dependencies from the original C includes:
// linux/regmap.h, linux/delay.h, mt6797-afe-common.h,
// mt6797-interconnection.h, mt6797-reg.h,
// ../common/mtk-dai-adda-common.h

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type gfp_t = c_uint;

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
    _private: [u8; 0],
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
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len() as c_int
    };
}

// External C macro constructors translated as Rust macro calls; their concrete
// expansion is supplied by future dependencies corresponding to the original headers.
macro_rules! SOC_DAPM_SINGLE_AUTODISABLE {
    ($name:expr, $reg:expr, $shift:expr, $max:expr, $invert:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! SND_SOC_DAPM_MIXER {
    ($name:expr, $reg:expr, $shift:expr, $invert:expr, $controls:expr, $num_controls:expr) => {
        snd_soc_dapm_widget {
            name: c_str!($name),
            dapm: core::ptr::null_mut(),
        }
    };
}

macro_rules! SND_SOC_DAPM_SUPPLY_S {
    ($name:expr, $seq:expr, $reg:expr, $shift:expr, $invert:expr, $event:expr, $event_flags:expr) => {
        snd_soc_dapm_widget {
            name: c_str!($name),
            dapm: core::ptr::null_mut(),
        }
    };
}

macro_rules! SND_SOC_DAPM_CLOCK_SUPPLY {
    ($name:expr) => {
        snd_soc_dapm_widget {
            name: c_str!($name),
            dapm: core::ptr::null_mut(),
        }
    };
}

extern "C" {
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
    static SND_SOC_NOPM: c_uint;
    static AFE_ADDA_UL_DL_CON0: c_uint;
    static ADDA_AFE_ON_SFT: c_uint;
    static AFE_ADDA_DL_SRC2_CON0: c_uint;
    static DL_2_SRC_ON_TMP_CTL_PRE_SFT: c_uint;
    static AFE_ADDA_UL_SRC_CON0: c_uint;
    static UL_SRC_ON_TMP_CTL_SFT: c_uint;
    static AUDIO_TOP_CON0: c_uint;
    static PDN_DAC_SFT: c_uint;
    static PDN_DAC_PREDIS_SFT: c_uint;
    static PDN_ADC_SFT: c_uint;
    static AFE_ADDA_PREDIS_CON0: c_uint;
    static AFE_ADDA_PREDIS_CON1: c_uint;
    static AFE_ADDA_TOP_CON0: c_uint;
    static AFE_ADDA_NEWIF_CFG0: c_uint;
    static AFE_ADDA_NEWIF_CFG2: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static MT6797_DAI_ADDA: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: gfp_t;
}

const ENOMEM: c_int = 12;

/* dai component */
static mtk_adda_dl_ch1_mix: [snd_kcontrol_new; 7] = unsafe {
    [
        SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN3, I_DL1_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN3, I_DL2_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN3, I_DL3_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN3, I_ADDA_UL_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN3, I_ADDA_UL_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN3, I_PCM_1_CAP_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN3, I_PCM_2_CAP_CH1, 1, 0),
    ]
};

static mtk_adda_dl_ch2_mix: [snd_kcontrol_new; 12] = unsafe {
    [
        SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN4, I_DL1_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN4, I_DL1_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN4, I_DL2_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN4, I_DL2_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN4, I_DL3_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN4, I_DL3_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN4, I_ADDA_UL_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN4, I_ADDA_UL_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH1", AFE_CONN4, I_PCM_1_CAP_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH1", AFE_CONN4, I_PCM_2_CAP_CH1, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_1_CAP_CH2", AFE_CONN4, I_PCM_1_CAP_CH2, 1, 0),
        SOC_DAPM_SINGLE_AUTODISABLE!("PCM_2_CAP_CH2", AFE_CONN4, I_PCM_2_CAP_CH2, 1, 0),
    ]
};

unsafe extern "C" fn mtk_adda_ul_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), name %s, event 0x%x\n"),
        c_str!("mtk_adda_ul_event"),
        (*w).name,
        event,
    );

    match event {
        x if x == SND_SOC_DAPM_POST_PMD => {
            /* should delayed 1/fs(smallest is 8k) = 125us before afe off */
            usleep_range(125, 135);
        }
        _ => {}
    }

    0
}

const SUPPLY_SEQ_AUD_TOP_PDN: c_int = 0;
const SUPPLY_SEQ_ADDA_AFE_ON: c_int = 1;
const SUPPLY_SEQ_ADDA_DL_ON: c_int = 2;
const SUPPLY_SEQ_ADDA_UL_ON: c_int = 3;

static mtk_dai_adda_widgets: [snd_soc_dapm_widget; 9] = unsafe {
    [
        /* adda */
        SND_SOC_DAPM_MIXER!(
            "ADDA_DL_CH1",
            SND_SOC_NOPM,
            0,
            0,
            mtk_adda_dl_ch1_mix.as_ptr(),
            ARRAY_SIZE!(mtk_adda_dl_ch1_mix)
        ),
        SND_SOC_DAPM_MIXER!(
            "ADDA_DL_CH2",
            SND_SOC_NOPM,
            0,
            0,
            mtk_adda_dl_ch2_mix.as_ptr(),
            ARRAY_SIZE!(mtk_adda_dl_ch2_mix)
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "ADDA Enable",
            SUPPLY_SEQ_ADDA_AFE_ON,
            AFE_ADDA_UL_DL_CON0,
            ADDA_AFE_ON_SFT,
            0,
            None::<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
            0
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "ADDA Playback Enable",
            SUPPLY_SEQ_ADDA_DL_ON,
            AFE_ADDA_DL_SRC2_CON0,
            DL_2_SRC_ON_TMP_CTL_PRE_SFT,
            0,
            None::<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
            0
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "ADDA Capture Enable",
            SUPPLY_SEQ_ADDA_UL_ON,
            AFE_ADDA_UL_SRC_CON0,
            UL_SRC_ON_TMP_CTL_SFT,
            0,
            Some(mtk_adda_ul_event),
            SND_SOC_DAPM_POST_PMD
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "aud_dac_clk",
            SUPPLY_SEQ_AUD_TOP_PDN,
            AUDIO_TOP_CON0,
            PDN_DAC_SFT,
            1,
            None::<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
            0
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "aud_dac_predis_clk",
            SUPPLY_SEQ_AUD_TOP_PDN,
            AUDIO_TOP_CON0,
            PDN_DAC_PREDIS_SFT,
            1,
            None::<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
            0
        ),
        SND_SOC_DAPM_SUPPLY_S!(
            "aud_adc_clk",
            SUPPLY_SEQ_AUD_TOP_PDN,
            AUDIO_TOP_CON0,
            PDN_ADC_SFT,
            1,
            None::<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
            0
        ),
        SND_SOC_DAPM_CLOCK_SUPPLY!("mtkaif_26m_clk"),
    ]
};

static mtk_dai_adda_routes: [snd_soc_dapm_route; 22] = [
    /* playback */
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL1_CH1"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL1_CH1"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL1_CH2"), source: c_str!("DL1") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL2_CH1"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL2_CH1"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL2_CH2"), source: c_str!("DL2") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH1"), control: c_str!("DL3_CH1"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL3_CH1"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA_DL_CH2"), control: c_str!("DL3_CH2"), source: c_str!("DL3") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA_DL_CH1") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA_DL_CH2") },
    /* adda enable */
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("ADDA Playback Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA Enable") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("ADDA Capture Enable") },
    /* clk */
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("mtkaif_26m_clk") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("aud_dac_clk") },
    snd_soc_dapm_route { sink: c_str!("ADDA Playback"), control: core::ptr::null(), source: c_str!("aud_dac_predis_clk") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("mtkaif_26m_clk") },
    snd_soc_dapm_route { sink: c_str!("ADDA Capture"), control: core::ptr::null(), source: c_str!("aud_adc_clk") },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_adda_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let rate = params_rate(params);

    dev_dbg(
        (*afe).dev,
        c_str!("%s(), id %d, stream %d, rate %d\n"),
        c_str!("mtk_dai_adda_hw_params"),
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

        /* set input sampling rate */
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

        if rate < 96000 {
            /* SA suggest apply -0.3db to audio/speech path */
            dl_src2_con1 = 0xf74f0000;
        } else {
            /* SA suggest apply -0.3db to audio/speech path
             * with DL gain set to half,
             * 0xFFFF = 0dB -> 0x8000 = 0dB when 96k, 192k
             */
            dl_src2_con1 = 0x7ba70000;
        }

        /* turn on down-link gain */
        dl_src2_con0 = dl_src2_con0 | (0x01 << 1);

        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, dl_src2_con0);
        regmap_write((*afe).regmap, AFE_ADDA_DL_SRC2_CON1, dl_src2_con1);
    } else {
        let mut voice_mode: c_uint = 0;
        let mut ul_src_con0: c_uint = 0; /* default value */

        /* Using Internal ADC */
        regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1 << 0, 0x0 << 0);

        voice_mode = mtk_adda_ul_rate_transform(afe, rate);

        ul_src_con0 |= (voice_mode << 17) & (0x7 << 17);

        /* up8x txif sat on */
        regmap_write((*afe).regmap, AFE_ADDA_NEWIF_CFG0, 0x03F87201);

        if rate >= 96000 {
            /* hires */
            /* use hires format [1 0 23] */
            regmap_update_bits((*afe).regmap, AFE_ADDA_NEWIF_CFG0, 0x1 << 5, 0x1 << 5);

            regmap_update_bits((*afe).regmap, AFE_ADDA_NEWIF_CFG2, 0xf << 28, voice_mode << 28);
        } else {
            /* normal 8~48k */
            /* use fixed 260k anc path */
            regmap_update_bits((*afe).regmap, AFE_ADDA_NEWIF_CFG2, 0xf << 28, 8 << 28);

            /* ul_use_cic_out */
            ul_src_con0 |= 0x1 << 20;
        }

        regmap_update_bits((*afe).regmap, AFE_ADDA_NEWIF_CFG2, 0xf << 28, 8 << 28);

        regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0xfffffffe, ul_src_con0);
    }

    0
}

static mtk_dai_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(mtk_dai_adda_hw_params),
};

/* dai driver */
static MTK_ADDA_PLAYBACK_RATES: c_uint = unsafe {
    SNDRV_PCM_RATE_8000_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000
};

static MTK_ADDA_CAPTURE_RATES: c_uint = unsafe {
    SNDRV_PCM_RATE_8000
        | SNDRV_PCM_RATE_16000
        | SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_192000
};

static MTK_ADDA_FORMATS: u64 =
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE };

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 1] = unsafe {
    [snd_soc_dai_driver {
        name: c_str!("ADDA"),
        id: MT6797_DAI_ADDA,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("ADDA Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_PLAYBACK_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("ADDA Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ADDA_CAPTURE_RATES,
            formats: MTK_ADDA_FORMATS,
        },
        ops: &mtk_dai_adda_ops,
    }]
};

#[no_mangle]
pub unsafe extern "C" fn mt6797_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
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
    (*dai).num_dai_drivers = ARRAY_SIZE!(mtk_dai_adda_driver);

    (*dai).dapm_widgets = mtk_dai_adda_widgets.as_ptr();
    (*dai).num_dapm_widgets = ARRAY_SIZE!(mtk_dai_adda_widgets);
    (*dai).dapm_routes = mtk_dai_adda_routes.as_ptr();
    (*dai).num_dapm_routes = ARRAY_SIZE!(mtk_dai_adda_routes);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
