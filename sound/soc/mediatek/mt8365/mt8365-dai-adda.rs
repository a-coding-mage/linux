// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 ALSA SoC Audio DAI ADDA Control
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

// C dependencies translated as external dependencies:
// <linux/bitops.h>, <linux/regmap.h>, <sound/pcm_params.h>
// "mt8365-afe-clk.h", "mt8365-afe-common.h",
// "../common/mtk-dai-adda-common.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

extern "C" {
    static mut AFE_ADDA_DL_VOICE_DATA: c_uint;
    static mut AFE_ADDA_DL_SAMPLING_RATE: c_uint;
    static mut AFE_ADDA_DL_8X_UPSAMPLE: c_uint;
    static mut AFE_ADDA_DL_MUTE_OFF_CH1: c_uint;
    static mut AFE_ADDA_DL_MUTE_OFF_CH2: c_uint;
    static mut AFE_ADDA_DL_DEGRADE_GAIN: c_uint;
    static mut AFE_ADDA_PREDIS_CON0: c_uint;
    static mut AFE_ADDA_PREDIS_CON1: c_uint;
    static mut AFE_ADDA_DL_SRC2_CON0: c_uint;
    static mut AFE_ADDA_DL_SRC2_CON1: c_uint;
    static mut AFE_ADDA_DL_SDM_DCCOMP_CON: c_uint;
    static mut AFE_ADDA_UL_SAMPLING_RATE: c_uint;
    static mut AFE_ADDA_UL_SRC_CON0: c_uint;
    static mut AFE_ADDA_TOP_CON0: c_uint;
    static mut AFE_ADDA_UL_DL_CON0: c_uint;
    static mut AFE_ADDA_UL_DL_ADDA_AFE_ON: c_uint;
    static mut AFE_AUD_PAD_TOP: c_uint;
    static mut SNDRV_PCM_STREAM_PLAYBACK: c_uint;
    static mut SNDRV_PCM_STREAM_CAPTURE: c_uint;
    static mut MT8365_TOP_CG_DAC: c_int;
    static mut MT8365_TOP_CG_DAC_PREDIS: c_int;
    static mut MT8365_TOP_CG_ADC: c_int;
    static mut MT8365_AFE_BACKEND_BASE: c_int;
    static mut MT8365_AFE_IO_INT_ADDA: c_int;
    static mut SNDRV_PCM_RATE_8000_48000: c_uint;
    static mut SNDRV_PCM_RATE_16000: c_uint;
    static mut SNDRV_PCM_RATE_32000: c_uint;
    static mut SNDRV_PCM_RATE_48000: c_uint;
    static mut SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static mut SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static mut AFE_CONN3: c_uint;
    static mut AFE_CONN4: c_uint;
    static mut SND_SOC_NOPM: c_int;
    static mut GFP_KERNEL: c_uint;
    static mut ENOMEM: c_int;

    fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint;
    fn mtk_adda_dl_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn mtk_adda_ul_rate_transform(afe: *mut mtk_base_afe, rate: c_uint) -> c_uint;
    fn regmap_update_bits(regmap: *mut c_void, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
    fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe) -> c_int;
    fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe) -> c_int;
    fn mt8365_afe_enable_top_cg(afe: *mut mtk_base_afe, cg: c_int) -> c_int;
    fn mt8365_afe_disable_top_cg(afe: *mut mtk_base_afe, cg: c_int) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn mt8365_afe_set_i2s_out(afe: *mut mtk_base_afe, rate: c_uint, bit_width: c_int) -> c_int;
    fn mt8365_afe_set_i2s_out_enable(afe: *mut mtk_base_afe, enable: bool);
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn dev_info(dev: *mut c_void, fmt: *const c_char, ...);
    fn snd_pcm_stream_str(substream: *mut snd_pcm_substream) -> *const c_char;
    fn usleep_range(min: c_uint, max: c_uint);
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub regmap: *mut c_void,
    pub platform_priv: *mut c_void,
    pub dev: *mut c_void,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mt8365_afe_private {
    pub afe_ctrl_lock: c_void,
    pub be_data: *mut mt8365_be_dai_data,
}

#[repr(C)]
pub struct mt8365_be_dai_data {
    pub prepared: [bool; 2],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
    pub format: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_uint,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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

static mut adda_afe_on_ref_cnt: c_int = 0;

/* DAI Drivers */

unsafe extern "C" fn mt8365_dai_set_adda_out(afe: *mut mtk_base_afe, rate: c_uint) -> c_int {
    let mut val: c_uint;

    if rate == 8000 || rate == 16000 {
        val = AFE_ADDA_DL_VOICE_DATA;
    } else {
        val = 0;
    }

    val |= FIELD_PREP(AFE_ADDA_DL_SAMPLING_RATE, mtk_adda_dl_rate_transform(afe, rate));
    val |= AFE_ADDA_DL_8X_UPSAMPLE
        | AFE_ADDA_DL_MUTE_OFF_CH1
        | AFE_ADDA_DL_MUTE_OFF_CH2
        | AFE_ADDA_DL_DEGRADE_GAIN;

    regmap_update_bits((*afe).regmap, AFE_ADDA_PREDIS_CON0, 0xffffffff, 0);
    regmap_update_bits((*afe).regmap, AFE_ADDA_PREDIS_CON1, 0xffffffff, 0);
    regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, 0xffffffff, val);
    /* SA suggest apply -0.3db to audio/speech path */
    regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SRC2_CON1, 0xffffffff, 0xf74f0000);
    /* SA suggest use default value for sdm */
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_DL_SDM_DCCOMP_CON,
        0xffffffff,
        0x0700701e,
    );

    0
}

unsafe extern "C" fn mt8365_dai_set_adda_in(afe: *mut mtk_base_afe, rate: c_uint) -> c_int {
    let mut val: c_uint;

    val = FIELD_PREP(AFE_ADDA_UL_SAMPLING_RATE, mtk_adda_ul_rate_transform(afe, rate));
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_UL_SRC_CON0,
        AFE_ADDA_UL_SAMPLING_RATE,
        val,
    );
    /* Using Internal ADC */
    regmap_update_bits((*afe).regmap, AFE_ADDA_TOP_CON0, 0x1, 0x0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_enable_adda_on(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv as *mut mt8365_afe_private;

    // C used guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock).
    let _lock = &mut (*afe_priv).afe_ctrl_lock;

    adda_afe_on_ref_cnt += 1;
    if adda_afe_on_ref_cnt == 1 {
        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_UL_DL_CON0,
            AFE_ADDA_UL_DL_ADDA_AFE_ON,
            AFE_ADDA_UL_DL_ADDA_AFE_ON,
        );
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_disable_adda_on(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv as *mut mt8365_afe_private;

    // C used guard(spinlock_irqsave)(&afe_priv->afe_ctrl_lock).
    let _lock = &mut (*afe_priv).afe_ctrl_lock;

    adda_afe_on_ref_cnt -= 1;
    if adda_afe_on_ref_cnt == 0 {
        regmap_update_bits(
            (*afe).regmap,
            AFE_ADDA_UL_DL_CON0,
            AFE_ADDA_UL_DL_ADDA_AFE_ON,
            !AFE_ADDA_UL_DL_ADDA_AFE_ON,
        );
    } else if adda_afe_on_ref_cnt < 0 {
        adda_afe_on_ref_cnt = 0;
        dev_warn(
            (*afe).dev,
            b"Abnormal adda_on ref count. Force it to 0\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

unsafe extern "C" fn mt8365_dai_set_adda_out_enable(afe: *mut mtk_base_afe, enable: bool) {
    regmap_update_bits((*afe).regmap, AFE_ADDA_DL_SRC2_CON0, 0x1, enable as c_uint);

    if enable {
        mt8365_dai_enable_adda_on(afe);
    } else {
        mt8365_dai_disable_adda_on(afe);
    }
}

unsafe extern "C" fn mt8365_dai_set_adda_in_enable(afe: *mut mtk_base_afe, enable: bool) {
    if enable {
        regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x1, 0x1);
        mt8365_dai_enable_adda_on(afe);
        /* enable aud_pad_top fifo */
        regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, 0xffffffff, 0x31);
    } else {
        /* disable aud_pad_top fifo */
        regmap_update_bits((*afe).regmap, AFE_AUD_PAD_TOP, 0xffffffff, 0x30);
        regmap_update_bits((*afe).regmap, AFE_ADDA_UL_SRC_CON0, 0x1, 0x0);
        /* de suggest disable ADDA_UL_SRC at least wait 125us */
        usleep_range(150, 300);
        mt8365_dai_disable_adda_on(afe);
    }
}

unsafe extern "C" fn mt8365_dai_int_adda_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let stream: c_uint = (*substream).stream;

    mt8365_afe_enable_main_clk(afe);

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DAC);
        mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DAC_PREDIS);
    } else if stream == SNDRV_PCM_STREAM_CAPTURE {
        mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_ADC);
    }

    0
}

unsafe extern "C" fn mt8365_dai_int_adda_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv as *mut mt8365_afe_private;
    let be: *mut mt8365_be_dai_data =
        (*afe_priv).be_data.offset(((*dai).id - MT8365_AFE_BACKEND_BASE) as isize);
    let stream: c_uint = (*substream).stream;

    if (*be).prepared[stream as usize] {
        if stream == SNDRV_PCM_STREAM_PLAYBACK {
            mt8365_dai_set_adda_out_enable(afe, false);
            mt8365_afe_set_i2s_out_enable(afe, false);
        } else {
            mt8365_dai_set_adda_in_enable(afe, false);
        }
        (*be).prepared[stream as usize] = false;
    }

    if stream == SNDRV_PCM_STREAM_PLAYBACK {
        mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DAC_PREDIS);
        mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DAC);
    } else if stream == SNDRV_PCM_STREAM_CAPTURE {
        mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_ADC);
    }

    mt8365_afe_disable_main_clk(afe);
}

unsafe extern "C" fn mt8365_dai_int_adda_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv: *mut mt8365_afe_private = (*afe).platform_priv as *mut mt8365_afe_private;
    let be: *mut mt8365_be_dai_data =
        (*afe_priv).be_data.offset(((*dai).id - MT8365_AFE_BACKEND_BASE) as isize);
    let rate: c_uint = (*(*substream).runtime).rate;
    let bit_width: c_int = snd_pcm_format_width((*(*substream).runtime).format);
    let mut ret: c_int;

    dev_info(
        (*afe).dev,
        b"%s '%s' rate = %u\n\0".as_ptr() as *const c_char,
        b"mt8365_dai_int_adda_prepare\0".as_ptr() as *const c_char,
        snd_pcm_stream_str(substream),
        rate,
    );

    if (*be).prepared[(*substream).stream as usize] {
        dev_info(
            (*afe).dev,
            b"%s '%s' prepared already\n\0".as_ptr() as *const c_char,
            b"mt8365_dai_int_adda_prepare\0".as_ptr() as *const c_char,
            snd_pcm_stream_str(substream),
        );
        return 0;
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = mt8365_dai_set_adda_out(afe, rate);
        if ret != 0 {
            return ret;
        }

        ret = mt8365_afe_set_i2s_out(afe, rate, bit_width);
        if ret != 0 {
            return ret;
        }

        mt8365_dai_set_adda_out_enable(afe, true);
        mt8365_afe_set_i2s_out_enable(afe, true);
    } else {
        ret = mt8365_dai_set_adda_in(afe, rate);
        if ret != 0 {
            return ret;
        }

        mt8365_dai_set_adda_in_enable(afe, true);
    }
    (*be).prepared[(*substream).stream as usize] = true;
    0
}

static mt8365_afe_int_adda_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_dai_int_adda_startup),
    shutdown: Some(mt8365_dai_int_adda_shutdown),
    prepare: Some(mt8365_dai_int_adda_prepare),
};

static mut mtk_dai_adda_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"INT ADDA\0".as_ptr() as *const c_char,
    id: unsafe { MT8365_AFE_IO_INT_ADDA },
    playback: snd_soc_pcm_stream {
        stream_name: b"INT ADDA Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"INT ADDA Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    ops: &mt8365_afe_int_adda_ops,
}];

/* DAI Controls */

// static const struct snd_kcontrol_new mtk_adda_dl_ch1_mix[] = {
//     SOC_DAPM_SINGLE_AUTODISABLE("GAIN1_OUT_CH1 Switch", AFE_CONN3, 10, 1, 0),
// };
extern "C" {
    static mtk_adda_dl_ch1_mix: [snd_kcontrol_new; 1];
}

// static const struct snd_kcontrol_new mtk_adda_dl_ch2_mix[] = {
//     SOC_DAPM_SINGLE_AUTODISABLE("GAIN1_OUT_CH2 Switch", AFE_CONN4, 11, 1, 0),
// };
extern "C" {
    static mtk_adda_dl_ch2_mix: [snd_kcontrol_new; 1];
}

// static const struct snd_kcontrol_new int_adda_o03_o04_enable_ctl =
//     SOC_DAPM_SINGLE_VIRT("Switch", 1);
extern "C" {
    static int_adda_o03_o04_enable_ctl: snd_kcontrol_new;
}

/* DAI widget */

// static const struct snd_soc_dapm_widget mtk_dai_adda_widgets[] = {
//     SND_SOC_DAPM_SWITCH("INT ADDA O03_O04", SND_SOC_NOPM, 0, 0,
//                         &int_adda_o03_o04_enable_ctl),
//     /* inter-connections */
//     SND_SOC_DAPM_MIXER("ADDA_DL_CH1", SND_SOC_NOPM, 0, 0,
//                        mtk_adda_dl_ch1_mix,
//                        ARRAY_SIZE(mtk_adda_dl_ch1_mix)),
//     SND_SOC_DAPM_MIXER("ADDA_DL_CH2", SND_SOC_NOPM, 0, 0,
//                        mtk_adda_dl_ch2_mix,
//                        ARRAY_SIZE(mtk_adda_dl_ch2_mix)),
// };
extern "C" {
    static mtk_dai_adda_widgets: [snd_soc_dapm_widget; 3];
}

/* DAI route */

static mtk_dai_adda_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: b"INT ADDA O03_O04\0".as_ptr() as *const c_char,
        control: b"Switch\0".as_ptr() as *const c_char,
        source: b"O03\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"INT ADDA O03_O04\0".as_ptr() as *const c_char,
        control: b"Switch\0".as_ptr() as *const c_char,
        source: b"O04\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"INT ADDA Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"INT ADDA O03_O04\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"INT ADDA Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"INT ADDA Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AIN Mux\0".as_ptr() as *const c_char,
        control: b"INT ADC\0".as_ptr() as *const c_char,
        source: b"INT ADDA Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
        control: b"GAIN1_OUT_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless FM DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"GAIN1_OUT_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless FM DL\0".as_ptr() as *const c_char,
    },
];

#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_adda_register(afe: *mut mtk_base_afe) -> c_int {
    let mut dai: *mut mtk_base_afe_dai;

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
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
