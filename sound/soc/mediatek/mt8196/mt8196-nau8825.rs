// SPDX-License-Identifier: GPL-2.0
/*
 *  mt8196-nau8825.rs  --  mt8196 nau8825 ALSA SoC machine driver
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const NAU8825_HS_PRESENT: c_uint = BIT(0);
const RT5682S_HS_PRESENT: c_uint = BIT(1);
const RT5650_HS_PRESENT: c_uint = BIT(2);

/*
 * Nau88l25
 */
const NAU8825_CODEC_DAI: *const c_char = c"nau8825-hifi".as_ptr();

/*
 * Rt5682s
 */
const RT5682S_CODEC_DAI: *const c_char = c"rt5682s-aif1".as_ptr();

/*
 * Rt5650
 */
const RT5650_CODEC_DAI: *const c_char = c"rt5645-aif1".as_ptr();

const SOF_DMA_DL1: *const c_char = c"SOF_DMA_DL1".as_ptr();
const SOF_DMA_DL_24CH: *const c_char = c"SOF_DMA_DL_24CH".as_ptr();
const SOF_DMA_UL0: *const c_char = c"SOF_DMA_UL0".as_ptr();
const SOF_DMA_UL1: *const c_char = c"SOF_DMA_UL1".as_ptr();
const SOF_DMA_UL2: *const c_char = c"SOF_DMA_UL2".as_ptr();

const SND_JACK_AVOUT: c_uint = 0x0001;
const SND_JACK_HEADPHONE: c_uint = 0x0002;
const SND_JACK_MICROPHONE: c_uint = 0x0004;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_uint = 0x0008;
const SND_JACK_BTN_1: c_uint = 0x0010;
const SND_JACK_BTN_2: c_uint = 0x0020;
const SND_JACK_BTN_3: c_uint = 0x0040;
const KEY_PLAYPAUSE: c_uint = 164;
const KEY_VOICECOMMAND: c_uint = 246;
const KEY_VOLUMEUP: c_uint = 115;
const KEY_VOLUMEDOWN: c_uint = 114;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DPCM_TRIGGER_PRE: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_LAST: c_int = 64;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 2;
const SND_SOC_DAIFMT_GATED: c_uint = 4;
const EINVAL: c_int = 22;
const NAU8825_CLK_FLL_BLK: c_int = 0;
const RT5682S_PLL1: c_int = 0;
const RT5682S_PLL_S_BCLK1: c_int = 0;
const RT5682S_SCLK_S_MCLK: c_int = 0;
const AFE_PCM_NAME: *const c_char = c"AFE_PCM".as_ptr();

#[repr(C)]
enum mt8196_jacks {
    MT8196_JACK_HEADSET,
    MT8196_JACK_DP,
    MT8196_JACK_HDMI,
    MT8196_JACK_MAX,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    id_name: *const c_char,
    reg: c_int,
    shift: c_int,
    invert: c_int,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    name: *const c_char,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    dai_name: *const c_char,
}

#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    trigger: [c_int; 2],
    dynamic: c_uint,
    playback_only: c_uint,
    capture_only: c_uint,
    no_pcm: c_uint,
    ignore_suspend: c_uint,
    ignore_pmdown_time: c_uint,
    dai_fmt: c_uint,
    ops: *const snd_soc_ops,
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    num_codecs: c_uint,
    codecs: *mut snd_soc_dai_link_component,
}

#[repr(C)]
struct snd_soc_card {
    owner: *mut c_void,
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
struct sof_conn_stream {
    sof_link: *const c_char,
    sof_dma: *const c_char,
    stream_dir: c_int,
}

#[repr(C)]
struct mtk_sof_priv {
    conn_streams: *const sof_conn_stream,
    num_streams: usize,
}

#[repr(C)]
struct mtk_platform_card_data {
    card: *mut snd_soc_card,
    num_jacks: c_int,
    flags: c_uint,
    jacks: *mut snd_soc_jack,
}

#[repr(C)]
struct mtk_soc_card_data {
    card_data: *mut mtk_platform_card_data,
}

#[repr(C)]
struct mtk_soundcard_pdata {
    card_name: *const c_char,
    card_data: *const mtk_platform_card_data,
    sof_priv: *const mtk_sof_priv,
    soc_probe: Option<unsafe extern "C" fn(*mut mtk_soc_card_data, bool) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, param: c_int) -> *mut snd_mask;
    fn snd_mask_reset_range(mask: *mut snd_mask, val: c_int, val2: c_int);
    fn params_set_format(params: *mut snd_pcm_hw_params, format: c_int);
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn pm_runtime_active(dev: *mut device) -> bool;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: usize,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num: usize,
    ) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut mtk_soc_card_data;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: usize,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn mtk_soundcard_common_probe(pdev: *mut c_void) -> c_int;
}

static mut mt8196_dp_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: c"DP".as_ptr(),
    mask: SND_JACK_AVOUT,
}];

static mut mt8196_hdmi_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: c"HDMI".as_ptr(),
    mask: SND_JACK_AVOUT,
}];

static mut nau8825_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone Jack".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

static mt8196_dumb_spk_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: c"Ext Spk".as_ptr(),
}];

static mt8196_dumb_spk_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id_name: c"Ext Spk".as_ptr(),
    reg: SND_SOC_NOPM,
    shift: 0,
    invert: 0,
}];

static mt8196_nau8825_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { id_name: c"Headphone Jack".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
    snd_soc_dapm_widget { id_name: c"Headset Mic".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
    snd_soc_dapm_widget { id_name: c"Ext Spk".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
    snd_soc_dapm_widget { id_name: c"DP".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
];

static mt8196_nau8825_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { name: c"Headphone Jack".as_ptr() },
    snd_kcontrol_new { name: c"Headset Mic".as_ptr() },
];

const EXT_SPK_AMP_W_NAME: *const c_char = c"Ext_Speaker_Amp".as_ptr();

static mt8196_nau8825_card_widgets: [snd_soc_dapm_widget; 3] = [
    /* SOF Uplink */
    snd_soc_dapm_widget { id_name: c"SOF_DMA_UL0".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
    snd_soc_dapm_widget { id_name: c"SOF_DMA_UL1".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },
    snd_soc_dapm_widget { id_name: c"SOF_DMA_UL2".as_ptr(), reg: SND_SOC_NOPM, shift: 0, invert: 0 },

    /*
     * SOF Downlink
     * the widgets on the machine driver cannot use the parameter with kcontrol
     * because the widget domain is its platform driver. so sof downlink route
     * is written in the i2s dai driver.
     */
];

static mt8196_nau8825_card_routes: [snd_soc_dapm_route; 6] = [
    /* SOF Uplink */
    snd_soc_dapm_route { sink: c"SOF_DMA_UL0".as_ptr(), control: ptr::null(), source: c"UL0_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"SOF_DMA_UL0".as_ptr(), control: ptr::null(), source: c"UL0_CH2".as_ptr() },
    /* SOF Uplink */
    snd_soc_dapm_route { sink: c"SOF_DMA_UL1".as_ptr(), control: ptr::null(), source: c"UL1_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"SOF_DMA_UL1".as_ptr(), control: ptr::null(), source: c"UL1_CH2".as_ptr() },
    /* SOF Uplink */
    snd_soc_dapm_route { sink: c"SOF_DMA_UL2".as_ptr(), control: ptr::null(), source: c"UL2_CH1".as_ptr() },
    snd_soc_dapm_route { sink: c"SOF_DMA_UL2".as_ptr(), control: ptr::null(), source: c"UL2_CH2".as_ptr() },
];

static mt8196_nau8825_card_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: EXT_SPK_AMP_W_NAME,
}];

/*
 * define mtk_spk_i2s_mck node in dts when need mclk,
 * BE i2s need assign snd_soc_ops = mt8196_nau8825_i2s_ops
 */
unsafe extern "C" fn mt8196_nau8825_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_sysclk(cpu_dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8196_nau8825_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8196_nau8825_i2s_hw_params),
};

unsafe extern "C" fn mt8196_dptx_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 256;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let dai = snd_soc_rtd_to_cpu(rtd, 0);

    snd_soc_dai_set_sysclk(dai, 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8196_dptx_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8196_dptx_hw_params),
};

unsafe extern "C" fn mt8196_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    dev_info((*rtd).dev, c"fix format to 32bit\n".as_ptr());

    /* fix BE i2s format to 32bit, clean param mask first */
    snd_mask_reset_range(
        hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
        0,
        SNDRV_PCM_FORMAT_LAST,
    );

    params_set_format(params, SNDRV_PCM_FORMAT_S32_LE);
    0
}

unsafe extern "C" fn mt8196_sof_be_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut cmpnt_afe: *mut snd_soc_component = ptr::null_mut();

    /* find afe component */
    let card = (*rtd).card;
    for i in 0..(*card).num_links {
        let runtime = (*card).dai_link.add(i as usize) as *mut snd_soc_pcm_runtime;
        cmpnt_afe = snd_soc_rtdcom_lookup(runtime, AFE_PCM_NAME);
        if !cmpnt_afe.is_null() {
            dev_info((*rtd).dev, c"component->name: %s\n".as_ptr(), (*cmpnt_afe).name);
            break;
        }
    }

    if !cmpnt_afe.is_null() && !pm_runtime_active((*cmpnt_afe).dev) {
        dev_err((*rtd).dev, c"afe pm runtime is not active!!\n".as_ptr());
        return -EINVAL;
    }

    0
}

static mt8196_sof_be_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8196_sof_be_hw_params),
};

static g_sof_conn_streams: [sof_conn_stream; 5] = [
    sof_conn_stream { sof_link: c"AFE_SOF_DL1".as_ptr(), sof_dma: SOF_DMA_DL1, stream_dir: SNDRV_PCM_STREAM_PLAYBACK },
    sof_conn_stream { sof_link: c"AFE_SOF_DL_24CH".as_ptr(), sof_dma: SOF_DMA_DL_24CH, stream_dir: SNDRV_PCM_STREAM_PLAYBACK },
    sof_conn_stream { sof_link: c"AFE_SOF_UL0".as_ptr(), sof_dma: SOF_DMA_UL0, stream_dir: SNDRV_PCM_STREAM_CAPTURE },
    sof_conn_stream { sof_link: c"AFE_SOF_UL1".as_ptr(), sof_dma: SOF_DMA_UL1, stream_dir: SNDRV_PCM_STREAM_CAPTURE },
    sof_conn_stream { sof_link: c"AFE_SOF_UL2".as_ptr(), sof_dma: SOF_DMA_UL2, stream_dir: SNDRV_PCM_STREAM_CAPTURE },
];

/* FE and BE SND_SOC_DAILINK_DEFS declarations are represented by dai_link entries below. */

static mut mt8196_nau8825_dai_links: [snd_soc_dai_link; 23] = [
    /*
     * The SOF topology expects PCM streams 0~4 to be available
     * for the SOF PCM streams. Put the SOF BE definitions here
     * so that the PCM device numbers are skipped over.
     * (BE dailinks do not have PCM devices created.)
     */
    snd_soc_dai_link { name: c"AFE_SOF_DL_24CH".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: &mt8196_sof_be_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AFE_SOF_DL1".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: &mt8196_sof_be_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AFE_SOF_UL0".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: &mt8196_sof_be_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AFE_SOF_UL1".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: &mt8196_sof_be_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AFE_SOF_UL2".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: &mt8196_sof_be_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    /* Front End DAI links */
    snd_soc_dai_link { name: c"HDMI_FE".as_ptr(), stream_name: c"HDMI Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"DL2_FE".as_ptr(), stream_name: c"DL2 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"UL_CM0_FE".as_ptr(), stream_name: c"UL_CM0 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"DL_24CH_FE".as_ptr(), stream_name: c"DL_24CH Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"DL1_FE".as_ptr(), stream_name: c"DL1 Playback".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"UL0_FE".as_ptr(), stream_name: c"UL0 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"UL1_FE".as_ptr(), stream_name: c"UL1 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"UL2_FE".as_ptr(), stream_name: c"UL2 Capture".as_ptr(), trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    /* Back End DAI links */
    snd_soc_dai_link { name: c"I2SIN6_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8196_nau8825_i2s_ops, be_hw_params_fixup: Some(mt8196_hw_params_fixup), init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"I2SOUT4_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8196_nau8825_i2s_ops, be_hw_params_fixup: Some(mt8196_hw_params_fixup), init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"I2SOUT6_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8196_nau8825_i2s_ops, be_hw_params_fixup: Some(mt8196_hw_params_fixup), init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AP_DMIC_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AP_DMIC_CH34_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"AP_DMIC_MULTICH_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"TDM_DPTX_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8196_dptx_ops, be_hw_params_fixup: Some(mt8196_hw_params_fixup), init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
    snd_soc_dai_link { name: c"I2SOUT3_BE".as_ptr(), stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, ignore_pmdown_time: 0, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_GATED, ops: &mt8196_nau8825_i2s_ops, be_hw_params_fixup: None, init: None, exit: None, num_codecs: 0, codecs: ptr::null_mut() },
];

unsafe extern "C" fn mt8196_dumb_amp_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int = 0;

    ret = snd_soc_dapm_new_controls(
        dapm,
        mt8196_dumb_spk_widgets.as_ptr(),
        mt8196_dumb_spk_widgets.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add Dumb Speaker dapm, ret %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(
        card,
        mt8196_dumb_spk_controls.as_ptr(),
        mt8196_dumb_spk_controls.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add Dumb card controls, ret %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8196_dptx_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = (*(*soc_card_data).card_data).jacks.add(mt8196_jacks::MT8196_JACK_DP as usize);
    let component = (*(snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_component)).name;
    let component_ptr = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_component;
    let mut ret: c_int = 0;

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"DP Jack".as_ptr(),
        SND_JACK_AVOUT,
        jack,
        mt8196_dp_jack_pins.as_mut_ptr(),
        mt8196_dp_jack_pins.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"new jack failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_jack(component_ptr, jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, c"set jack failed on %s (ret=%d)\n".as_ptr(), component, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8196_hdmi_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let soc_card_data = snd_soc_card_get_drvdata((*rtd).card);
    let jack = (*(*soc_card_data).card_data).jacks.add(mt8196_jacks::MT8196_JACK_HDMI as usize);
    let component_ptr = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_component;
    let mut ret: c_int = 0;

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"HDMI Jack".as_ptr(),
        SND_JACK_AVOUT,
        jack,
        mt8196_hdmi_jack_pins.as_mut_ptr(),
        mt8196_hdmi_jack_pins.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"new jack failed: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_jack(component_ptr, jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, c"set jack failed on %s (ret=%d)\n".as_ptr(), (*component_ptr).name, ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8196_headset_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let soc_card_data = snd_soc_card_get_drvdata(card);
    let jack = (*(*soc_card_data).card_data).jacks.add(mt8196_jacks::MT8196_JACK_HEADSET as usize);
    let component = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_component;
    let mut ret: c_int;
    let mut type_: c_int;

    ret = snd_soc_dapm_new_controls(
        dapm,
        mt8196_nau8825_widgets.as_ptr(),
        mt8196_nau8825_widgets.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add nau8825 card widget, ret %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(
        card,
        mt8196_nau8825_controls.as_ptr(),
        mt8196_nau8825_controls.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"unable to add nau8825 card controls, ret %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        c"Headset Jack".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        nau8825_jack_pins.as_mut_ptr(),
        nau8825_jack_pins.len(),
    );
    if ret != 0 {
        dev_err((*rtd).dev, c"Headset Jack creation failed: %d\n".as_ptr(), ret);
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    type_ = (SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3) as c_int;
    ret = snd_soc_component_set_jack(component, jack, (&mut type_ as *mut c_int).cast::<c_void>());

    if ret != 0 {
        dev_err((*rtd).dev, c"Headset Jack call-back failed: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8196_headset_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_component;

    snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn mt8196_nau8825_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let bit_width = params_width(params);
    let clk_freq: c_int = rate.wrapping_mul(2).wrapping_mul(bit_width) as c_int;
    let mut ret: c_int;

    /* Configure clock for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*codec_dai).dev, c"can't set BCLK clock %d\n".as_ptr(), ret);
        return ret;
    }

    /* Configure pll for codec */
    ret = snd_soc_dai_set_pll(
        codec_dai,
        0,
        0,
        clk_freq as c_uint,
        params_rate(params).wrapping_mul(256),
    );
    if ret < 0 {
        dev_err((*codec_dai).dev, c"can't set BCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static mt8196_nau8825_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8196_nau8825_hw_params),
};

unsafe extern "C" fn mt8196_rt5682s_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let card = (*rtd).card;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let rate = params_rate(params);
    let bitwidth: c_int;
    let mut ret: c_int;

    bitwidth = snd_pcm_format_width(params_format(params));
    if bitwidth < 0 {
        dev_err((*card).dev, c"invalid bit width: %d\n".as_ptr(), bitwidth);
        return bitwidth;
    }

    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x00, 0x0, 0x2, bitwidth);
    if ret != 0 {
        dev_err((*card).dev, c"failed to set tdm slot\n".as_ptr());
        return ret;
    }

    ret = snd_soc_dai_set_pll(
        codec_dai,
        RT5682S_PLL1,
        RT5682S_PLL_S_BCLK1,
        rate.wrapping_mul(32),
        rate.wrapping_mul(512),
    );
    if ret != 0 {
        dev_err((*card).dev, c"failed to set pll\n".as_ptr());
        return ret;
    }

    dev_info(
        (*card).dev,
        c"%s set mclk rate: %d\n".as_ptr(),
        c"mt8196_rt5682s_i2s_hw_params".as_ptr(),
        rate.wrapping_mul(512),
    );

    ret = snd_soc_dai_set_sysclk(codec_dai, RT5682S_SCLK_S_MCLK, rate.wrapping_mul(512), SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err((*card).dev, c"failed to set sysclk\n".as_ptr());
        return ret;
    }

    snd_soc_dai_set_sysclk(cpu_dai, 0, rate.wrapping_mul(512), SND_SOC_CLOCK_OUT)
}

static mt8196_rt5682s_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mt8196_rt5682s_i2s_hw_params),
};

unsafe extern "C" fn mt8196_nau8825_soc_card_probe(
    soc_card_data: *mut mtk_soc_card_data,
    legacy: bool,
) -> c_int {
    let card = (*(*soc_card_data).card_data).card;
    let mut init_nau8825 = false;
    let mut init_rt5682s = false;
    let mut init_rt5650 = false;
    let mut init_dumb = false;

    dev_info((*card).dev, c"legacy: %d\n".as_ptr(), legacy as c_int);

    for i in 0..(*card).num_links {
        let dai_link = (*card).dai_link.add(i as usize);
        if strcmp((*dai_link).name, c"TDM_DPTX_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0
                && strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0
            {
                (*dai_link).init = Some(mt8196_dptx_codec_init);
            }
        } else if strcmp((*dai_link).name, c"I2SOUT3_BE".as_ptr()) == 0 {
            if (*dai_link).num_codecs != 0
                && strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0
            {
                (*dai_link).init = Some(mt8196_hdmi_codec_init);
            }
        } else if strcmp((*dai_link).name, c"I2SOUT6_BE".as_ptr()) == 0
            || strcmp((*dai_link).name, c"I2SIN6_BE".as_ptr()) == 0
        {
            if strcmp((*(*dai_link).codecs).dai_name, NAU8825_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8196_nau8825_ops;
                if !init_nau8825 {
                    (*dai_link).init = Some(mt8196_headset_codec_init);
                    (*dai_link).exit = Some(mt8196_headset_codec_exit);
                    init_nau8825 = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5682S_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8196_rt5682s_i2s_ops;
                if !init_rt5682s {
                    (*dai_link).init = Some(mt8196_headset_codec_init);
                    (*dai_link).exit = Some(mt8196_headset_codec_exit);
                    init_rt5682s = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, RT5650_CODEC_DAI) == 0 {
                (*dai_link).ops = &mt8196_rt5682s_i2s_ops;
                if !init_rt5650 {
                    (*dai_link).init = Some(mt8196_headset_codec_init);
                    (*dai_link).exit = Some(mt8196_headset_codec_exit);
                    init_rt5650 = true;
                }
            } else if strcmp((*(*dai_link).codecs).dai_name, c"snd-soc-dummy-dai".as_ptr()) != 0 {
                if !init_dumb {
                    (*dai_link).init = Some(mt8196_dumb_amp_init);
                    init_dumb = true;
                }
            }
        }
    }

    0
}

static mt8196_sof_priv: mtk_sof_priv = mtk_sof_priv {
    conn_streams: g_sof_conn_streams.as_ptr(),
    num_streams: g_sof_conn_streams.len(),
};

static mut mt8196_nau8825_soc_card: snd_soc_card = snd_soc_card {
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: unsafe { mt8196_nau8825_dai_links.as_mut_ptr() },
    num_links: 23,
    dapm_widgets: mt8196_nau8825_card_widgets.as_ptr(),
    num_dapm_widgets: 3,
    dapm_routes: mt8196_nau8825_card_routes.as_ptr(),
    num_dapm_routes: 6,
    controls: mt8196_nau8825_card_controls.as_ptr(),
    num_controls: 1,
};

static mt8196_nau8825_platform_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8196_nau8825_soc_card },
    num_jacks: mt8196_jacks::MT8196_JACK_MAX as c_int,
    flags: NAU8825_HS_PRESENT,
    jacks: ptr::null_mut(),
};

static mt8196_nau8825_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8196_nau8825".as_ptr(),
    card_data: &mt8196_nau8825_platform_card_data,
    sof_priv: &mt8196_sof_priv,
    soc_probe: Some(mt8196_nau8825_soc_card_probe),
};

static mt8196_rt5682s_platform_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8196_nau8825_soc_card },
    num_jacks: mt8196_jacks::MT8196_JACK_MAX as c_int,
    flags: RT5682S_HS_PRESENT,
    jacks: ptr::null_mut(),
};

static mt8196_rt5682s_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8196_rt5682s".as_ptr(),
    card_data: &mt8196_rt5682s_platform_card_data,
    sof_priv: &mt8196_sof_priv,
    soc_probe: Some(mt8196_nau8825_soc_card_probe),
};

static mt8196_rt5650_platform_card_data: mtk_platform_card_data = mtk_platform_card_data {
    card: unsafe { &mut mt8196_nau8825_soc_card },
    num_jacks: mt8196_jacks::MT8196_JACK_MAX as c_int,
    flags: RT5650_HS_PRESENT,
    jacks: ptr::null_mut(),
};

static mt8196_rt5650_card: mtk_soundcard_pdata = mtk_soundcard_pdata {
    card_name: c"mt8196_rt5650".as_ptr(),
    card_data: &mt8196_rt5650_platform_card_data,
    sof_priv: &mt8196_sof_priv,
    soc_probe: Some(mt8196_nau8825_soc_card_probe),
};

static mt8196_nau8825_dt_match: [of_device_id; 4] = [
    of_device_id {
        compatible: c"mediatek,mt8196-nau8825-sound".as_ptr(),
        data: (&mt8196_nau8825_card as *const mtk_soundcard_pdata).cast::<c_void>(),
    },
    of_device_id {
        compatible: c"mediatek,mt8196-rt5682s-sound".as_ptr(),
        data: (&mt8196_rt5682s_card as *const mtk_soundcard_pdata).cast::<c_void>(),
    },
    of_device_id {
        compatible: c"mediatek,mt8196-rt5650-sound".as_ptr(),
        data: (&mt8196_rt5650_card as *const mtk_soundcard_pdata).cast::<c_void>(),
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, mt8196_nau8825_dt_match); */

static mut mt8196_nau8825_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"mt8196-nau8825".as_ptr(),
        of_match_table: mt8196_nau8825_dt_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(mtk_soundcard_common_probe),
};
/* module_platform_driver(mt8196_nau8825_driver); */

/* Module information */
/* MODULE_DESCRIPTION("MT8196 nau8825 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Darren Ye <darren.ye@mediatek.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("mt8196 nau8825 soc card"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
