// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC Audio DAI eTDM Control
 *
 * Copyright (c) 2023 MediaTek Inc.
 * Authors: Vic Wu <vic.wu@mediatek.com>
 *          Maso Huang <maso.huang@mediatek.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const HOPPING_CLK: c_uint = 0;
const APLL_CLK: c_uint = 1;
const MTK_DAI_ETDM_FORMAT_I2S: c_uint = 0;
const MTK_DAI_ETDM_FORMAT_DSPA: c_uint = 4;
const MTK_DAI_ETDM_FORMAT_DSPB: c_uint = 5;

const MTK_ETDM_RATE_8K: c_uint = 0;
const MTK_ETDM_RATE_12K: c_uint = 1;
const MTK_ETDM_RATE_16K: c_uint = 2;
const MTK_ETDM_RATE_24K: c_uint = 3;
const MTK_ETDM_RATE_32K: c_uint = 4;
const MTK_ETDM_RATE_48K: c_uint = 5;
const MTK_ETDM_RATE_96K: c_uint = 7;
const MTK_ETDM_RATE_192K: c_uint = 9;
const MTK_ETDM_RATE_11K: c_uint = 16;
const MTK_ETDM_RATE_22K: c_uint = 17;
const MTK_ETDM_RATE_44K: c_uint = 18;
const MTK_ETDM_RATE_88K: c_uint = 19;
const MTK_ETDM_RATE_176K: c_uint = 20;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct clk_bulk_data {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct snd_soc_dai {
    id: c_int,
}

#[repr(C)]
struct mtk_base_afe {
    dev: *mut device,
    regmap: *mut regmap,
    platform_priv: *mut mt7986_afe_private,
    sub_dais: list_head,
}

#[repr(C)]
struct mt7986_afe_private {
    num_clks: c_int,
    clks: *mut clk_bulk_data,
    dai_priv: [*mut c_void; 0],
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

#[repr(C)]
struct mtk_dai_etdm_priv {
    bck_inv: bool,
    lrck_inv: bool,
    slave_mode: bool,
    format: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    capture: snd_soc_pcm_stream,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
    symmetric_rate: c_uint,
    symmetric_sample_bits: c_uint,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut clk_bulk_data);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mt7986_afe_rate_transform(dev: *mut device, rate: c_uint) -> c_uint;
    fn list_add(new: *mut list_head, head: *mut list_head);

    fn SOC_DAPM_SINGLE_AUTODISABLE(
        name: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_MIXER(
        name: *const c_char,
        reg: c_int,
        shift: c_uint,
        invert: c_uint,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> snd_soc_dapm_widget;
}

extern "Rust" {
    static AUDIO_TOP_CON2: c_uint;
    static CLK_OUT5_PDN_MASK: c_uint;
    static CLK_IN5_PDN_MASK: c_uint;
    static CLK_OUT5_PDN: c_uint;
    static CLK_IN5_PDN: c_uint;
    static ETDM_BIT_LEN_MASK: c_uint;
    static ETDM_WRD_LEN_MASK: c_uint;
    static ETDM_FMT_MASK: c_uint;
    static ETDM_CH_NUM_MASK: c_uint;
    static RELATCH_SRC_MASK: c_uint;
    static ETDM_OUT5_CON0: c_uint;
    static ETDM_OUT5_CON4: c_uint;
    static ETDM_OUT5_CON5: c_uint;
    static OUT_RELATCH_MASK: c_uint;
    static OUT_CLK_SRC_MASK: c_uint;
    static OUT_SEL_FS_MASK: c_uint;
    static ETDM_CLK_DIV_MASK: c_uint;
    static ETDM_CLK_DIV: c_uint;
    static ETDM_IN5_CON0: c_uint;
    static ETDM_IN5_CON2: c_uint;
    static ETDM_IN5_CON3: c_uint;
    static ETDM_IN5_CON4: c_uint;
    static ETDM_SYNC_MASK: c_uint;
    static ETDM_SYNC: c_uint;
    static IN_CLK_SRC_MASK: c_uint;
    static IN_SEL_FS_MASK: c_uint;
    static IN_RELATCH_MASK: c_uint;
    static ETDM_EN_MASK: c_uint;
    static ETDM_EN: c_uint;
    static SND_SOC_NOPM: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_DAIFMT_MASTER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static MT7986_DAI_ETDM: c_int;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint;
    fn OUT_RELATCH(rate: c_uint) -> c_uint;
    fn OUT_CLK_SRC(clk: c_uint) -> c_uint;
    fn OUT_SEL_FS(rate: c_uint) -> c_uint;
    fn IN_CLK_SRC(clk: c_uint) -> c_uint;
    fn IN_SEL_FS(rate: c_uint) -> c_uint;
    fn IN_RELATCH(rate: c_uint) -> c_uint;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn dai_priv_at(afe_priv: *mut mt7986_afe_private, id: c_int) -> *mut *mut c_void {
    (*afe_priv).dai_priv.as_mut_ptr().offset(id as isize)
}

unsafe fn mt7986_etdm_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_ETDM_RATE_8K,
        11025 => MTK_ETDM_RATE_11K,
        12000 => MTK_ETDM_RATE_12K,
        16000 => MTK_ETDM_RATE_16K,
        22050 => MTK_ETDM_RATE_22K,
        24000 => MTK_ETDM_RATE_24K,
        32000 => MTK_ETDM_RATE_32K,
        44100 => MTK_ETDM_RATE_44K,
        48000 => MTK_ETDM_RATE_48K,
        88200 => MTK_ETDM_RATE_88K,
        96000 => MTK_ETDM_RATE_96K,
        176400 => MTK_ETDM_RATE_176K,
        192000 => MTK_ETDM_RATE_192K,
        _ => {
            dev_warn(
                dev,
                cstr!("%s(), rate %u invalid, using %d!!!\n"),
                cstr!("mt7986_etdm_rate_transform"),
                rate,
                MTK_ETDM_RATE_48K,
            );
            MTK_ETDM_RATE_48K
        }
    }
}

fn get_etdm_wlen(bitwidth: c_uint) -> c_int {
    if bitwidth <= 16 { 16 } else { 32 }
}

/* dai component */
/* interconnection */

static o124_mix: [snd_kcontrol_new; 1] = unsafe {
    [SOC_DAPM_SINGLE_AUTODISABLE(
        cstr!("I032_Switch"),
        AFE_CONN124_1,
        0,
        1,
        0,
    )]
};

static o125_mix: [snd_kcontrol_new; 1] = unsafe {
    [SOC_DAPM_SINGLE_AUTODISABLE(
        cstr!("I033_Switch"),
        AFE_CONN125_1,
        1,
        1,
        0,
    )]
};

extern "Rust" {
    static AFE_CONN124_1: c_uint;
    static AFE_CONN125_1: c_uint;
}

static mtk_dai_etdm_widgets: [snd_soc_dapm_widget; 4] = unsafe {
    [
        /* DL */
        SND_SOC_DAPM_MIXER(cstr!("I150"), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
        SND_SOC_DAPM_MIXER(cstr!("I151"), SND_SOC_NOPM, 0, 0, ptr::null(), 0),
        /* UL */
        SND_SOC_DAPM_MIXER(
            cstr!("O124"),
            SND_SOC_NOPM,
            0,
            0,
            o124_mix.as_ptr(),
            o124_mix.len() as c_uint,
        ),
        SND_SOC_DAPM_MIXER(
            cstr!("O125"),
            SND_SOC_NOPM,
            0,
            0,
            o125_mix.as_ptr(),
            o125_mix.len() as c_uint,
        ),
    ]
};

static mtk_dai_etdm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: cstr!("I150"), control: ptr::null(), source: cstr!("ETDM Capture") },
    snd_soc_dapm_route { sink: cstr!("I151"), control: ptr::null(), source: cstr!("ETDM Capture") },
    snd_soc_dapm_route { sink: cstr!("ETDM Playback"), control: ptr::null(), source: cstr!("O124") },
    snd_soc_dapm_route { sink: cstr!("ETDM Playback"), control: ptr::null(), source: cstr!("O125") },
    snd_soc_dapm_route { sink: cstr!("O124"), control: cstr!("I032_Switch"), source: cstr!("I032") },
    snd_soc_dapm_route { sink: cstr!("O125"), control: cstr!("I033_Switch"), source: cstr!("I033") },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_etdm_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let ret: c_int;

    ret = clk_bulk_prepare_enable((*afe_priv).num_clks, (*afe_priv).clks);
    if ret != 0 {
        return dev_err_probe((*afe).dev, ret, cstr!("Failed to enable clocks\n"));
    }

    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON2, CLK_OUT5_PDN_MASK, 0);
    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON2, CLK_IN5_PDN_MASK, 0);

    0
}

unsafe extern "C" fn mtk_dai_etdm_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;

    regmap_update_bits(
        (*afe).regmap,
        AUDIO_TOP_CON2,
        CLK_OUT5_PDN_MASK,
        CLK_OUT5_PDN,
    );
    regmap_update_bits(
        (*afe).regmap,
        AUDIO_TOP_CON2,
        CLK_IN5_PDN_MASK,
        CLK_IN5_PDN,
    );

    clk_bulk_disable_unprepare((*afe_priv).num_clks, (*afe_priv).clks);
}

fn get_etdm_ch_fixup(channels: c_uint) -> c_uint {
    if channels > 16 {
        24
    } else if channels > 8 {
        16
    } else if channels > 4 {
        8
    } else if channels > 2 {
        4
    } else {
        2
    }
}

unsafe fn mtk_dai_etdm_config(
    afe: *mut mtk_base_afe,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
    stream: c_int,
) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let etdm_data = *dai_priv_at(afe_priv, (*dai).id) as *mut mtk_dai_etdm_priv;
    let rate = params_rate(params);
    let etdm_rate = mt7986_etdm_rate_transform((*afe).dev, rate);
    let afe_rate = mt7986_afe_rate_transform((*afe).dev, rate);
    let channels = params_channels(params);
    let bit_width = params_width(params);
    let wlen = get_etdm_wlen(bit_width);
    let mut val: c_uint = 0;
    let mut mask: c_uint = 0;

    dev_dbg(
        (*afe).dev,
        cstr!("%s(), stream %d, rate %u, bitwidth %u\n"),
        cstr!("mtk_dai_etdm_config"),
        stream,
        rate,
        bit_width,
    );

    /* CON0 */
    mask |= ETDM_BIT_LEN_MASK;
    val |= FIELD_PREP(ETDM_BIT_LEN_MASK, bit_width.wrapping_sub(1));
    mask |= ETDM_WRD_LEN_MASK;
    val |= FIELD_PREP(ETDM_WRD_LEN_MASK, (wlen - 1) as c_uint);
    mask |= ETDM_FMT_MASK;
    val |= FIELD_PREP(ETDM_FMT_MASK, (*etdm_data).format);
    mask |= ETDM_CH_NUM_MASK;
    val |= FIELD_PREP(ETDM_CH_NUM_MASK, get_etdm_ch_fixup(channels).wrapping_sub(1));
    mask |= RELATCH_SRC_MASK;
    val |= FIELD_PREP(RELATCH_SRC_MASK, APLL_CLK);

    match stream {
        x if x == SNDRV_PCM_STREAM_PLAYBACK => {
            /* set ETDM_OUT5_CON0 */
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON0, mask, val);

            /* set ETDM_OUT5_CON4 */
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON4, OUT_RELATCH_MASK, OUT_RELATCH(afe_rate));
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON4, OUT_CLK_SRC_MASK, OUT_CLK_SRC(APLL_CLK));
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON4, OUT_SEL_FS_MASK, OUT_SEL_FS(etdm_rate));

            /* set ETDM_OUT5_CON5 */
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON5, ETDM_CLK_DIV_MASK, ETDM_CLK_DIV);
        }
        x if x == SNDRV_PCM_STREAM_CAPTURE => {
            /* set ETDM_IN5_CON0 */
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON0, mask, val);
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON0, ETDM_SYNC_MASK, ETDM_SYNC);

            /* set ETDM_IN5_CON2 */
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON2, IN_CLK_SRC_MASK, IN_CLK_SRC(APLL_CLK));

            /* set ETDM_IN5_CON3 */
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON3, IN_SEL_FS_MASK, IN_SEL_FS(etdm_rate));

            /* set ETDM_IN5_CON4 */
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON4, IN_RELATCH_MASK, IN_RELATCH(afe_rate));
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_dai_etdm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rate = params_rate(params);
    let afe = snd_soc_dai_get_drvdata(dai);

    match rate {
        8000 | 12000 | 16000 | 24000 | 32000 | 48000 | 96000 | 192000 => {
            mtk_dai_etdm_config(afe, params, dai, SNDRV_PCM_STREAM_PLAYBACK);
            mtk_dai_etdm_config(afe, params, dai, SNDRV_PCM_STREAM_CAPTURE);
            0
        }
        _ => {
            dev_err(
                (*afe).dev,
                cstr!("Sample rate %d invalid. Supported rates: 8/12/16/24/32/48/96/192 kHz\n"),
                rate,
            );
            -EINVAL
        }
    }
}

unsafe extern "C" fn mtk_dai_etdm_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);

    dev_dbg(
        (*afe).dev,
        cstr!("%s(), cmd %d, dai id %d\n"),
        cstr!("mtk_dai_etdm_trigger"),
        cmd,
        (*dai).id,
    );
    match cmd {
        x if x == SNDRV_PCM_TRIGGER_START || x == SNDRV_PCM_TRIGGER_RESUME => {
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON0, ETDM_EN_MASK, ETDM_EN);
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON0, ETDM_EN_MASK, ETDM_EN);
        }
        x if x == SNDRV_PCM_TRIGGER_STOP || x == SNDRV_PCM_TRIGGER_SUSPEND => {
            regmap_update_bits((*afe).regmap, ETDM_IN5_CON0, ETDM_EN_MASK, 0);
            regmap_update_bits((*afe).regmap, ETDM_OUT5_CON0, ETDM_EN_MASK, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn mtk_dai_etdm_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);
    let afe_priv = (*afe).platform_priv;
    let etdm_data: *mut mtk_dai_etdm_priv;
    let priv_data: *mut c_void;

    match (*dai).id {
        x if x == MT7986_DAI_ETDM => {}
        _ => {
            dev_warn(
                (*afe).dev,
                cstr!("%s(), id %d not support\n"),
                cstr!("mtk_dai_etdm_set_fmt"),
                (*dai).id,
            );
            return -EINVAL;
        }
    }

    priv_data = devm_kzalloc((*afe).dev, size_of::<mtk_dai_etdm_priv>(), GFP_KERNEL);
    if priv_data.is_null() {
        return -ENOMEM;
    }

    *dai_priv_at(afe_priv, (*dai).id) = priv_data;
    etdm_data = *dai_priv_at(afe_priv, (*dai).id) as *mut mtk_dai_etdm_priv;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            (*etdm_data).format = MTK_DAI_ETDM_FORMAT_I2S;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            (*etdm_data).format = MTK_DAI_ETDM_FORMAT_DSPA;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            (*etdm_data).format = MTK_DAI_ETDM_FORMAT_DSPB;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {
            (*etdm_data).bck_inv = false;
            (*etdm_data).lrck_inv = false;
        }
        x if x == SND_SOC_DAIFMT_NB_IF => {
            (*etdm_data).bck_inv = false;
            (*etdm_data).lrck_inv = true;
        }
        x if x == SND_SOC_DAIFMT_IB_NF => {
            (*etdm_data).bck_inv = true;
            (*etdm_data).lrck_inv = false;
        }
        x if x == SND_SOC_DAIFMT_IB_IF => {
            (*etdm_data).bck_inv = true;
            (*etdm_data).lrck_inv = true;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            (*etdm_data).slave_mode = true;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            (*etdm_data).slave_mode = false;
        }
        _ => return -EINVAL,
    }

    0
}

static mtk_dai_etdm_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mtk_dai_etdm_startup),
    shutdown: Some(mtk_dai_etdm_shutdown),
    hw_params: Some(mtk_dai_etdm_hw_params),
    trigger: Some(mtk_dai_etdm_trigger),
    set_fmt: Some(mtk_dai_etdm_set_fmt),
};

/* dai driver */
fn MTK_ETDM_RATES() -> c_uint {
    unsafe {
        SNDRV_PCM_RATE_8000_48000
            | SNDRV_PCM_RATE_88200
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_176400
            | SNDRV_PCM_RATE_192000
    }
}

fn MTK_ETDM_FORMATS() -> u64 {
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE }
}

static mut mtk_dai_etdm_driver: [snd_soc_dai_driver; 1] = unsafe {
    [snd_soc_dai_driver {
        name: cstr!("ETDM"),
        id: MT7986_DAI_ETDM,
        capture: snd_soc_pcm_stream {
            stream_name: cstr!("ETDM Capture"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ETDM_RATES(),
            formats: MTK_ETDM_FORMATS(),
        },
        playback: snd_soc_pcm_stream {
            stream_name: cstr!("ETDM Playback"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_ETDM_RATES(),
            formats: MTK_ETDM_FORMATS(),
        },
        ops: &mtk_dai_etdm_ops,
        symmetric_rate: 1,
        symmetric_sample_bits: 1,
    }]
};

#[no_mangle]
pub unsafe extern "C" fn mt7986_dai_etdm_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_etdm_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_etdm_driver.len() as c_uint;

    (*dai).dapm_widgets = mtk_dai_etdm_widgets.as_ptr();
    (*dai).num_dapm_widgets = mtk_dai_etdm_widgets.len() as c_uint;
    (*dai).dapm_routes = mtk_dai_etdm_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_etdm_routes.len() as c_uint;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
