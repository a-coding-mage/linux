// SPDX-License-Identifier: GPL-2.0
//
// mt8183-da7219-max98357.c
//	--  MT8183-DA7219-MAX98357 ALSA SoC machine driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Shunli Wang <shunli.wang@mediatek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const DA7219_CODEC_DAI: *const c_char = b"da7219-hifi\0".as_ptr() as *const c_char;
const DA7219_DEV_NAME: *const c_char = b"da7219.5-001a\0".as_ptr() as *const c_char;
const RT1015_CODEC_DAI: *const c_char = b"rt1015-aif\0".as_ptr() as *const c_char;
const RT1015_DEV0_NAME: *const c_char = b"rt1015.6-0028\0".as_ptr() as *const c_char;
const RT1015_DEV1_NAME: *const c_char = b"rt1015.6-0029\0".as_ptr() as *const c_char;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    static i2s3_max98357a_cpus: [snd_soc_dai_link_component; 1];
    static i2s3_max98357a_codecs: [snd_soc_dai_link_component; 2];
    static i2s3_max98357a_platforms: [snd_soc_dai_link_component; 1];
    static i2s3_rt1015_cpus: [snd_soc_dai_link_component; 1];
    static i2s3_rt1015_codecs: [snd_soc_dai_link_component; 3];
    static i2s3_rt1015_platforms: [snd_soc_dai_link_component; 1];
    static i2s3_rt1015p_cpus: [snd_soc_dai_link_component; 1];
    static i2s3_rt1015p_codecs: [snd_soc_dai_link_component; 2];
    static i2s3_rt1015p_platforms: [snd_soc_dai_link_component; 1];

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_reset_range(mask: *mut snd_mask, from: c_uint, to: c_uint);
    fn params_set_format(params: *mut snd_pcm_hw_params, val: c_int);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut snd_pcm_runtime, cond: c_uint, width: c_uint, msbits: c_uint) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_card_jack_new(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, driver_name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn mt8183_dai_i2s_set_share(afe: *mut mtk_base_afe, master: *const c_char, slave: *const c_char) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_node_put(node: *mut device_node);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_pinctrl_get_select(dev: *mut device, name: *const c_char) -> *mut pinctrl;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct mt8183_da7219_max98357_priv {
    headset_jack: snd_soc_jack,
    hdmi_jack: snd_soc_jack,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}

#[repr(C)]
struct snd_soc_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}

#[repr(C)]
struct snd_pcm_hardware {
    channels_max: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_hw_params;
#[repr(C)]
struct snd_mask;
#[repr(C)]
struct snd_jack;
#[repr(C)]
struct snd_soc_dai;
#[repr(C)]
struct mtk_base_afe;
#[repr(C)]
struct pinctrl;
#[repr(C)]
struct device_node;

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_soc_component {
    name: *const c_char,
    card: *mut snd_soc_card,
}

#[repr(C)]
struct snd_soc_dai_with_component {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    dev: *mut device,
    card: *mut snd_soc_card,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
    of_node: *mut device_node,
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
    be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    dai_fmt: c_uint,
    ignore: c_uint,
    cpus: *const snd_soc_dai_link_component,
    num_cpus: c_uint,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
}

#[repr(C)]
struct snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component,
    init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
struct snd_soc_codec_conf {
    dlc: snd_soc_dai_link_component,
    name_prefix: *const c_char,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const c_char,
    event: *mut c_void,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    dev: *mut device,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_uint,
    aux_dev: *mut snd_soc_aux_dev,
    num_aux_devs: c_uint,
    codec_conf: *mut snd_soc_codec_conf,
    num_configs: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct driver_private {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const c_void,
}

#[repr(C)]
struct platform_driver {
    driver: driver_private,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_LINEOUT: c_int = 0x0004;
const SND_JACK_AVOUT: c_int = 0x0008;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x0010;
const SND_JACK_BTN_1: c_int = 0x0020;
const SND_JACK_BTN_2: c_int = 0x0040;
const SND_JACK_BTN_3: c_int = 0x0080;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const KEY_VOICECOMMAND: c_int = 246;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const DA7219_CLKSRC_MCLK: c_int = 0;
const DA7219_PLL_FREQ_OUT_98304: c_uint = 98304000;
const DA7219_PLL_FREQ_OUT_90316: c_uint = 90316800;
const DA7219_SYSCLK_PLL_SRM: c_int = 1;
const DA7219_SYSCLK_MCLK: c_int = 0;
const RT1015_PLL_S_BCLK: c_int = 1;
const RT1015_SCLK_S_PLL: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_FORMAT_LAST: c_uint = 64;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SND_SOC_DPCM_TRIGGER_PRE: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_IB_IF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 4;
const GFP_KERNEL: c_uint = 0;
const PINCTRL_STATE_DEFAULT: *const c_char = b"default\0".as_ptr() as *const c_char;
const AFE_PCM_NAME: *const c_char = b"AFE_PCM\0".as_ptr() as *const c_char;

static mut mt8183_da7219_max98357_jack_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin { pin: b"Headphones\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
    snd_soc_jack_pin { pin: b"Line Out\0".as_ptr() as *const c_char, mask: SND_JACK_LINEOUT },
];

unsafe extern "C" fn mt8183_mt6358_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);

    snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8183_mt6358_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(mt8183_mt6358_i2s_hw_params),
    hw_free: None,
};

unsafe extern "C" fn mt8183_da7219_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai_with_component;
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 256;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let mut freq: c_uint;
    let mut ret: c_int = 0;
    let mut j: c_int = 0;

    ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk_fs, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err((*rtd).dev, b"failed to set cpu dai sysclk\n\0".as_ptr() as *const c_char);
    }

    /* for_each_rtd_codec_dais(rtd, j, codec_dai) */
    while next_rtd_codec_dai(rtd, &mut j, &mut codec_dai) {
        if strcmp((*(*codec_dai).component).name, DA7219_DEV_NAME) == 0 {
            ret = snd_soc_dai_set_sysclk(codec_dai as *mut snd_soc_dai, DA7219_CLKSRC_MCLK, mclk_fs, SND_SOC_CLOCK_IN);
            if ret < 0 {
                dev_err((*rtd).dev, b"failed to set sysclk\n\0".as_ptr() as *const c_char);
            }

            if rate % 8000 == 0 {
                freq = DA7219_PLL_FREQ_OUT_98304;
            } else {
                freq = DA7219_PLL_FREQ_OUT_90316;
            }

            ret = snd_soc_dai_set_pll(codec_dai as *mut snd_soc_dai, 0, DA7219_SYSCLK_PLL_SRM, 0, freq);
            if ret != 0 {
                dev_err((*rtd).dev, b"failed to start PLL: %d\n\0".as_ptr() as *const c_char, ret);
            }
        }
    }

    ret
}

unsafe extern "C" fn mt8183_da7219_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai_with_component;
    let mut ret: c_int = 0;
    let mut j: c_int = 0;

    /* for_each_rtd_codec_dais(rtd, j, codec_dai) */
    while next_rtd_codec_dai(rtd, &mut j, &mut codec_dai) {
        if strcmp((*(*codec_dai).component).name, DA7219_DEV_NAME) == 0 {
            ret = snd_soc_dai_set_pll(codec_dai as *mut snd_soc_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
            if ret < 0 {
                dev_err((*rtd).dev, b"failed to stop PLL: %d\n\0".as_ptr() as *const c_char, ret);
                break;
            }
        }
    }

    ret
}

static mt8183_da7219_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(mt8183_da7219_i2s_hw_params),
    hw_free: Some(mt8183_da7219_hw_free),
};

unsafe extern "C" fn mt8183_da7219_rt1015_i2s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mut codec_dai: *mut snd_soc_dai_with_component;
    let mut ret: c_int = 0;
    let mut i: c_int = 0;

    /* for_each_rtd_codec_dais(rtd, i, codec_dai) */
    while next_rtd_codec_dai(rtd, &mut i, &mut codec_dai) {
        if strcmp((*(*codec_dai).component).name, RT1015_DEV0_NAME) == 0
            || strcmp((*(*codec_dai).component).name, RT1015_DEV1_NAME) == 0
        {
            ret = snd_soc_dai_set_pll(codec_dai as *mut snd_soc_dai, 0, RT1015_PLL_S_BCLK, rate.wrapping_mul(64), rate.wrapping_mul(256));
            if ret != 0 {
                dev_err((*rtd).dev, b"failed to set pll\n\0".as_ptr() as *const c_char);
                return ret;
            }

            ret = snd_soc_dai_set_sysclk(codec_dai as *mut snd_soc_dai, RT1015_SCLK_S_PLL, rate.wrapping_mul(256), SND_SOC_CLOCK_IN);
            if ret != 0 {
                dev_err((*rtd).dev, b"failed to set sysclk\n\0".as_ptr() as *const c_char);
                return ret;
            }
        }
    }

    mt8183_da7219_i2s_hw_params(substream, params)
}

static mt8183_da7219_rt1015_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(mt8183_da7219_rt1015_i2s_hw_params),
    hw_free: Some(mt8183_da7219_hw_free),
};

unsafe extern "C" fn mt8183_i2s_hw_params_fixup(_rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    /* fix BE i2s format to S32_LE, clean param mask first */
    snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT), 0, SNDRV_PCM_FORMAT_LAST);

    params_set_format(params, SNDRV_PCM_FORMAT_S32_LE);

    0
}

unsafe extern "C" fn mt8183_rt1015_i2s_hw_params_fixup(_rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int {
    /* fix BE i2s format to S24_LE, clean param mask first */
    snd_mask_reset_range(hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT), 0, SNDRV_PCM_FORMAT_LAST);

    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    0
}

unsafe extern "C" fn mt8183_da7219_max98357_startup(substream: *mut snd_pcm_substream) -> c_int {
    static rates: [c_uint; 1] = [48000];
    static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 1,
        list: rates.as_ptr(),
        mask: 0,
    };
    static channels: [c_uint; 1] = [2];
    static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 1,
        list: channels.as_ptr(),
        mask: 0,
    };

    let runtime = (*substream).runtime;

    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    (*runtime).hw.channels_max = 2;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);

    (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE;
    snd_pcm_hw_constraint_msbits(runtime, 0, 16, 16);

    0
}

static mt8183_da7219_max98357_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_da7219_max98357_startup),
    hw_params: None,
    hw_free: None,
};

unsafe extern "C" fn mt8183_da7219_max98357_bt_sco_startup(substream: *mut snd_pcm_substream) -> c_int {
    static rates: [c_uint; 2] = [8000, 16000];
    static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 2,
        list: rates.as_ptr(),
        mask: 0,
    };
    static channels: [c_uint; 1] = [1];
    static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
        count: 1,
        list: channels.as_ptr(),
        mask: 0,
    };

    let runtime = (*substream).runtime;

    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    (*runtime).hw.channels_max = 1;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);

    (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE;
    snd_pcm_hw_constraint_msbits(runtime, 0, 16, 16);

    0
}

static mt8183_da7219_max98357_bt_sco_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_da7219_max98357_bt_sco_startup),
    hw_params: None,
    hw_free: None,
};

/* FE/BE SND_SOC_DAILINK_DEFS declarations from C are external macro expansions here. */

unsafe extern "C" fn mt8183_da7219_max98357_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut mt8183_da7219_max98357_priv;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new((*rtd).card, b"HDMI Jack\0".as_ptr() as *const c_char, SND_JACK_AVOUT, &mut (*priv_).hdmi_jack);
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack((*snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_dai_with_component).component, &mut (*priv_).hdmi_jack, ptr::null_mut())
}

unsafe extern "C" fn mt8183_bt_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let mut ret: c_int;

    ret = mt8183_dai_i2s_set_share(afe, b"I2S5\0".as_ptr() as *const c_char, b"I2S0\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err((*rtd).dev, b"Failed to set up shared clocks\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

unsafe extern "C" fn mt8183_da7219_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let mut ret: c_int;

    ret = mt8183_dai_i2s_set_share(afe, b"I2S2\0".as_ptr() as *const c_char, b"I2S3\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err((*rtd).dev, b"Failed to set up shared clocks\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

static mut mt8183_da7219_dai_links: [snd_soc_dai_link; 18] = [
    snd_soc_dai_link { name: b"Playback_1\0".as_ptr() as *const c_char, stream_name: b"Playback_1\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: &mt8183_da7219_max98357_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Playback_2\0".as_ptr() as *const c_char, stream_name: b"Playback_2\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: &mt8183_da7219_max98357_bt_sco_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Playback_3\0".as_ptr() as *const c_char, stream_name: b"Playback_3\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Capture_1\0".as_ptr() as *const c_char, stream_name: b"Capture_1\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: &mt8183_da7219_max98357_bt_sco_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Capture_2\0".as_ptr() as *const c_char, stream_name: b"Capture_2\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Capture_3\0".as_ptr() as *const c_char, stream_name: b"Capture_3\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: &mt8183_da7219_max98357_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Capture_Mono_1\0".as_ptr() as *const c_char, stream_name: b"Capture_Mono_1\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Playback_HDMI\0".as_ptr() as *const c_char, stream_name: b"Playback_HDMI\0".as_ptr() as *const c_char, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, no_pcm: 0, ignore_suspend: 0, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"Primary Codec\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"PCM 1\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"PCM 2\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"I2S0\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), ops: &mt8183_mt6358_i2s_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"I2S1\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), ops: &mt8183_mt6358_i2s_ops, init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"I2S2\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), ops: &mt8183_da7219_i2s_ops, init: Some(mt8183_da7219_init), dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"I2S3\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: None, ops: ptr::null(), init: None, dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"I2S5\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), ops: &mt8183_mt6358_i2s_ops, init: Some(mt8183_bt_init), dai_fmt: 0, ignore: 0, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
    snd_soc_dai_link { name: b"TDM\0".as_ptr() as *const c_char, stream_name: ptr::null(), trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, no_pcm: 1, ignore_suspend: 1, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), ops: ptr::null(), init: Some(mt8183_da7219_max98357_hdmi_init), dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_CBP_CFP, ignore: 1, cpus: ptr::null(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0 },
];

unsafe extern "C" fn mt8183_da7219_max98357_headset_init(component: *mut snd_soc_component) -> c_int {
    let mut ret: c_int;
    let priv_ = snd_soc_card_get_drvdata((*component).card) as *mut mt8183_da7219_max98357_priv;

    /* Enable Headset and 4 Buttons Jack detection */
    ret = snd_soc_card_jack_new_pins(
        (*component).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3 | SND_JACK_LINEOUT,
        &mut (*priv_).headset_jack,
        mt8183_da7219_max98357_jack_pins.as_mut_ptr(),
        mt8183_da7219_max98357_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    snd_soc_component_set_jack(component, &mut (*priv_).headset_jack, ptr::null_mut());

    0
}

static mut mt8183_da7219_max98357_headset_dev: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component { name: ptr::null(), dai_name: ptr::null(), of_node: ptr::null_mut() },
    init: Some(mt8183_da7219_max98357_headset_init),
};

static mut mt6358_codec_conf: [snd_soc_codec_conf; 1] = [
    snd_soc_codec_conf {
        dlc: snd_soc_dai_link_component { name: b"mt6358-sound\0".as_ptr() as *const c_char, dai_name: ptr::null(), of_node: ptr::null_mut() },
        name_prefix: b"Mt6358\0".as_ptr() as *const c_char,
    },
];

static mt8183_da7219_max98357_snd_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: b"Headphones\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Speakers\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Line Out\0".as_ptr() as *const c_char },
];

static mt8183_da7219_max98357_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { name: b"Headphones\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Speakers\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Line Out\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"TDM_OUT_PINCTRL\0".as_ptr() as *const c_char, event: ptr::null_mut() },
];

static mt8183_da7219_max98357_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TDM_OUT_PINCTRL\0".as_ptr() as *const c_char },
];

static mut mt8183_da7219_max98357_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_da7219_max98357\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    controls: mt8183_da7219_max98357_snd_controls.as_ptr(),
    num_controls: 4,
    dapm_widgets: mt8183_da7219_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: mt8183_da7219_max98357_dapm_routes.as_ptr(),
    num_dapm_routes: 2,
    dai_link: unsafe { mt8183_da7219_dai_links.as_mut_ptr() },
    num_links: 18,
    aux_dev: unsafe { &mut mt8183_da7219_max98357_headset_dev },
    num_aux_devs: 1,
    codec_conf: unsafe { mt6358_codec_conf.as_mut_ptr() },
    num_configs: 1,
};

static mut mt8183_da7219_rt1015_codec_conf: [snd_soc_codec_conf; 3] = [
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: b"mt6358-sound\0".as_ptr() as *const c_char, dai_name: ptr::null(), of_node: ptr::null_mut() }, name_prefix: b"Mt6358\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: RT1015_DEV0_NAME, dai_name: ptr::null(), of_node: ptr::null_mut() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: RT1015_DEV1_NAME, dai_name: ptr::null(), of_node: ptr::null_mut() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
];

static mt8183_da7219_rt1015_snd_controls: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { name: b"Headphones\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Left Spk\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Right Spk\0".as_ptr() as *const c_char },
    snd_kcontrol_new { name: b"Line Out\0".as_ptr() as *const c_char },
];

static mt8183_da7219_rt1015_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    snd_soc_dapm_widget { name: b"Headphones\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Left Spk\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Right Spk\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"Line Out\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { name: b"TDM_OUT_PINCTRL\0".as_ptr() as *const c_char, event: ptr::null_mut() },
];

static mt8183_da7219_rt1015_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Left SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Right SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S Playback\0".as_ptr() as *const c_char, control: ptr::null(), source: b"TDM_OUT_PINCTRL\0".as_ptr() as *const c_char },
];

static mut mt8183_da7219_rt1015_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_da7219_rt1015\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    controls: mt8183_da7219_rt1015_snd_controls.as_ptr(),
    num_controls: 5,
    dapm_widgets: mt8183_da7219_rt1015_dapm_widgets.as_ptr(),
    num_dapm_widgets: 6,
    dapm_routes: mt8183_da7219_rt1015_dapm_routes.as_ptr(),
    num_dapm_routes: 3,
    dai_link: unsafe { mt8183_da7219_dai_links.as_mut_ptr() },
    num_links: 18,
    aux_dev: unsafe { &mut mt8183_da7219_max98357_headset_dev },
    num_aux_devs: 1,
    codec_conf: unsafe { mt8183_da7219_rt1015_codec_conf.as_mut_ptr() },
    num_configs: 3,
};

static mut mt8183_da7219_rt1015p_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_da7219_rt1015p\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    controls: mt8183_da7219_max98357_snd_controls.as_ptr(),
    num_controls: 4,
    dapm_widgets: mt8183_da7219_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: mt8183_da7219_max98357_dapm_routes.as_ptr(),
    num_dapm_routes: 2,
    dai_link: unsafe { mt8183_da7219_dai_links.as_mut_ptr() },
    num_links: 18,
    aux_dev: unsafe { &mut mt8183_da7219_max98357_headset_dev },
    num_aux_devs: 1,
    codec_conf: unsafe { mt6358_codec_conf.as_mut_ptr() },
    num_configs: 1,
};

unsafe extern "C" fn mt8183_da7219_max98357_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut card: *mut snd_soc_card;
    let mut platform_node: *mut device_node;
    let mut hdmi_codec: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut priv_: *mut mt8183_da7219_max98357_priv;
    let mut pinctrl: *mut pinctrl;
    let mut ret: c_int;
    let mut i: c_int;

    platform_node = of_parse_phandle((*pdev).dev.of_node, b"mediatek,platform\0".as_ptr() as *const c_char, 0);
    if platform_node.is_null() {
        dev_err(&mut (*pdev).dev, b"Property 'platform' missing or invalid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    card = of_device_get_match_data(&mut (*pdev).dev) as *mut snd_soc_card;
    if card.is_null() {
        ret = -EINVAL;
        goto_put_platform_node(ret, platform_node);
        return ret;
    }

    (*card).dev = &mut (*pdev).dev;

    hdmi_codec = of_parse_phandle((*pdev).dev.of_node, b"mediatek,hdmi-codec\0".as_ptr() as *const c_char, 0);

    i = 0;
    /* for_each_card_prelinks(card, i, dai_link) */
    while next_card_prelink(card, &mut i, &mut dai_link) {
        if strcmp((*dai_link).name, b"I2S3\0".as_ptr() as *const c_char) == 0 {
            if card == &mut mt8183_da7219_max98357_card {
                (*dai_link).be_hw_params_fixup = Some(mt8183_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_da7219_i2s_ops;
                (*dai_link).cpus = i2s3_max98357a_cpus.as_ptr();
                (*dai_link).num_cpus = i2s3_max98357a_cpus.len() as c_uint;
                (*dai_link).codecs = i2s3_max98357a_codecs.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_codecs = i2s3_max98357a_codecs.len() as c_uint;
                (*dai_link).platforms = i2s3_max98357a_platforms.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_platforms = i2s3_max98357a_platforms.len() as c_uint;
            } else if card == &mut mt8183_da7219_rt1015_card {
                (*dai_link).be_hw_params_fixup = Some(mt8183_rt1015_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_da7219_rt1015_i2s_ops;
                (*dai_link).cpus = i2s3_rt1015_cpus.as_ptr();
                (*dai_link).num_cpus = i2s3_rt1015_cpus.len() as c_uint;
                (*dai_link).codecs = i2s3_rt1015_codecs.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_codecs = i2s3_rt1015_codecs.len() as c_uint;
                (*dai_link).platforms = i2s3_rt1015_platforms.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_platforms = i2s3_rt1015_platforms.len() as c_uint;
            } else if card == &mut mt8183_da7219_rt1015p_card {
                (*dai_link).be_hw_params_fixup = Some(mt8183_rt1015_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_da7219_i2s_ops;
                (*dai_link).cpus = i2s3_rt1015p_cpus.as_ptr();
                (*dai_link).num_cpus = i2s3_rt1015p_cpus.len() as c_uint;
                (*dai_link).codecs = i2s3_rt1015p_codecs.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_codecs = i2s3_rt1015p_codecs.len() as c_uint;
                (*dai_link).platforms = i2s3_rt1015p_platforms.as_ptr() as *mut snd_soc_dai_link_component;
                (*dai_link).num_platforms = i2s3_rt1015p_platforms.len() as c_uint;
            }
        }

        if !hdmi_codec.is_null() && strcmp((*dai_link).name, b"TDM\0".as_ptr() as *const c_char) == 0 {
            (*(*dai_link).codecs).of_node = hdmi_codec;
            (*dai_link).ignore = 0;
        }

        if (*(*dai_link).platforms).name.is_null() {
            (*(*dai_link).platforms).of_node = platform_node;
        }
    }

    mt8183_da7219_max98357_headset_dev.dlc.of_node =
        of_parse_phandle((*pdev).dev.of_node, b"mediatek,headset-codec\0".as_ptr() as *const c_char, 0);
    if mt8183_da7219_max98357_headset_dev.dlc.of_node.is_null() {
        dev_err(&mut (*pdev).dev, b"Property 'mediatek,headset-codec' missing/invalid\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
        goto_put_hdmi_codec(ret, hdmi_codec, platform_node);
        return ret;
    }

    priv_ = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mt8183_da7219_max98357_priv>(), GFP_KERNEL) as *mut mt8183_da7219_max98357_priv;
    if priv_.is_null() {
        ret = -ENOMEM;
        goto_put_hdmi_codec(ret, hdmi_codec, platform_node);
        return ret;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    pinctrl = devm_pinctrl_get_select(&mut (*pdev).dev, PINCTRL_STATE_DEFAULT);
    if IS_ERR(pinctrl as *const c_void) {
        ret = PTR_ERR(pinctrl as *const c_void);
        dev_err(&mut (*pdev).dev, b"%s failed to select default state %d\n\0".as_ptr() as *const c_char, b"mt8183_da7219_max98357_dev_probe\0".as_ptr() as *const c_char, ret);
        goto_put_hdmi_codec(ret, hdmi_codec, platform_node);
        return ret;
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);

    of_node_put(hdmi_codec);
    of_node_put(platform_node);
    ret
}

unsafe fn goto_put_hdmi_codec(ret: c_int, hdmi_codec: *mut device_node, platform_node: *mut device_node) {
    of_node_put(hdmi_codec);
    goto_put_platform_node(ret, platform_node);
}

unsafe fn goto_put_platform_node(_ret: c_int, platform_node: *mut device_node) {
    of_node_put(platform_node);
}

/* CONFIG_OF */
static mut mt8183_da7219_max98357_dt_match: [of_device_id; 4] = [
    of_device_id { compatible: b"mediatek,mt8183_da7219_max98357\0".as_ptr() as *const c_char, data: unsafe { &mt8183_da7219_max98357_card as *const _ as *const c_void } },
    of_device_id { compatible: b"mediatek,mt8183_da7219_rt1015\0".as_ptr() as *const c_char, data: unsafe { &mt8183_da7219_rt1015_card as *const _ as *const c_void } },
    of_device_id { compatible: b"mediatek,mt8183_da7219_rt1015p\0".as_ptr() as *const c_char, data: unsafe { &mt8183_da7219_rt1015p_card as *const _ as *const c_void } },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, mt8183_da7219_max98357_dt_match); */

static mut mt8183_da7219_max98357_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: b"mt8183_da7219\0".as_ptr() as *const c_char,
        of_match_table: unsafe { mt8183_da7219_max98357_dt_match.as_ptr() },
        pm: unsafe { &snd_soc_pm_ops as *const _ as *const c_void },
    },
    probe: Some(mt8183_da7219_max98357_dev_probe),
};

/* module_platform_driver(mt8183_da7219_max98357_driver); */

/* Module information */
/* MODULE_DESCRIPTION("MT8183-DA7219-MAX98357 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Shunli Wang <shunli.wang@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("mt8183_da7219_max98357 soc card"); */

unsafe fn next_rtd_codec_dai(_rtd: *mut snd_soc_pcm_runtime, _idx: &mut c_int, _codec_dai: &mut *mut snd_soc_dai_with_component) -> bool {
    /* External expansion point for for_each_rtd_codec_dais(rtd, idx, codec_dai). */
    false
}

unsafe fn next_card_prelink(card: *mut snd_soc_card, idx: &mut c_int, dai_link: &mut *mut snd_soc_dai_link) -> bool {
    if (*idx as c_uint) < (*card).num_links {
        *dai_link = (*card).dai_link.add(*idx as usize);
        *idx += 1;
        true
    } else {
        false
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
