// SPDX-License-Identifier: GPL-2.0
//
// mt8183-mt6358.c  --
//      MT8183-MT6358-TS3A227-MAX98357 ALSA SoC machine driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Shunli Wang <shunli.wang@mediatek.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const RT1015_CODEC_DAI: *const c_char = b"rt1015-aif\0".as_ptr() as *const c_char;
const RT1015_DEV0_NAME: *const c_char = b"rt1015.6-0028\0".as_ptr() as *const c_char;
const RT1015_DEV1_NAME: *const c_char = b"rt1015.6-0029\0".as_ptr() as *const c_char;

const PIN_STATE_DEFAULT: usize = 0;
const PIN_TDM_OUT_ON: usize = 1;
const PIN_TDM_OUT_OFF: usize = 2;
const PIN_WOV: usize = 3;
const PIN_STATE_MAX: usize = 4;

static mt8183_pin_str: [*const c_char; PIN_STATE_MAX] = [
    b"default\0".as_ptr() as *const c_char,
    b"aud_tdm_out_on\0".as_ptr() as *const c_char,
    b"aud_tdm_out_off\0".as_ptr() as *const c_char,
    b"wov\0".as_ptr() as *const c_char,
];

#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub trigger: [c_uint; 2],
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub ignore_suspend: c_uint,
    pub ignore: c_uint,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub be_hw_params_fixup: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
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
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_dai_link_component,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_uint,
}

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct mt8183_mt6358_ts3a227_max98357_priv {
    pinctrl: *mut pinctrl,
    pin_states: [*mut pinctrl_state; PIN_STATE_MAX],
    headset_jack: snd_soc_jack,
    hdmi_jack: snd_soc_jack,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;
    static i2s3_max98357a_cpus: *mut snd_soc_dai_link_component;
    static i2s3_max98357a_codecs: *mut snd_soc_dai_link_component;
    static i2s3_max98357a_platforms: *mut snd_soc_dai_link_component;
    static i2s3_rt1015_cpus: *mut snd_soc_dai_link_component;
    static i2s3_rt1015_codecs: *mut snd_soc_dai_link_component;
    static i2s3_rt1015_platforms: *mut snd_soc_dai_link_component;
    static i2s3_rt1015p_cpus: *mut snd_soc_dai_link_component;
    static i2s3_rt1015p_codecs: *mut snd_soc_dai_link_component;
    static i2s3_rt1015p_platforms: *mut snd_soc_dai_link_component;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_reset_range(mask: *mut snd_mask, from: c_uint, to: c_uint);
    fn params_set_format(params: *mut snd_pcm_hw_params, val: c_int);
    fn snd_pcm_hw_constraint_list(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, l: *const snd_pcm_hw_constraint_list) -> c_int;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut snd_pcm_runtime, cond: c_uint, width: c_uint, msbits: c_uint) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn pinctrl_select_state(p: *mut pinctrl, state: *mut pinctrl_state) -> c_int;
    fn pinctrl_lookup_state(p: *mut pinctrl, name: *const c_char) -> *mut pinctrl_state;
    fn snd_soc_card_jack_new(card: *mut snd_soc_card, id: *const c_char, ty: c_uint, jack: *mut snd_soc_jack) -> c_int;
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, driver_name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn mt8183_dai_i2s_set_share(afe: *mut mtk_base_afe, master: *const c_char, slave: *const c_char) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, ty: c_uint, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn ts3a227e_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT1015_PLL_S_BCLK: c_int = 1;
const RT1015_SCLK_S_PLL: c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_FORMAT_LAST: c_uint = 64;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;
const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
const SND_SOC_DPCM_TRIGGER_PRE: c_uint = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_IB_IF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 4;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 8;
const SND_SOC_DAIFMT_NB_NF: c_uint = 16;
const SND_JACK_AVOUT: c_uint = 0x400;
const SND_JACK_HEADPHONE: c_uint = 0x01;
const SND_JACK_MICROPHONE: c_uint = 0x02;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_uint = 0x4000;
const SND_JACK_BTN_1: c_uint = 0x2000;
const SND_JACK_BTN_2: c_uint = 0x1000;
const SND_JACK_BTN_3: c_uint = 0x0800;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const AFE_PCM_NAME: *const c_char = b"mtk-afe-pcm\0".as_ptr() as *const c_char;

macro_rules! array_size {
    ($array:expr) => {
        ($array.len() as c_uint)
    };
}

unsafe extern "C" fn mt8183_mt6358_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);

    snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8183_mt6358_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    shutdown: None,
    hw_params: Some(mt8183_mt6358_i2s_hw_params),
};

unsafe extern "C" fn mt8183_mt6358_rt1015_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rate = params_rate(params);
    let mclk_fs_ratio: c_uint = 128;
    let mclk_fs = rate.wrapping_mul(mclk_fs_ratio);
    let card = (*rtd).card;
    let mut ret: c_int;

    let mut i: c_uint = 0;
    while i < (*rtd).card.as_ref().map_or(0, |_| 0) {
        i = i.wrapping_add(1);
    }
    /* for_each_rtd_codec_dais(rtd, i, codec_dai) */
    let mut codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    while !codec_dai.is_null() {
        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            RT1015_PLL_S_BCLK,
            rate.wrapping_mul(64),
            rate.wrapping_mul(256),
        );
        if ret < 0 {
            dev_err((*card).dev, b"failed to set pll\n\0".as_ptr() as *const c_char);
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(
            codec_dai,
            RT1015_SCLK_S_PLL,
            rate.wrapping_mul(256),
            SND_SOC_CLOCK_IN,
        );
        if ret < 0 {
            dev_err((*card).dev, b"failed to set sysclk\n\0".as_ptr() as *const c_char);
            return ret;
        }
        break;
    }

    snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(rtd, 0), 0, mclk_fs, SND_SOC_CLOCK_OUT)
}

static mt8183_mt6358_rt1015_i2s_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    shutdown: None,
    hw_params: Some(mt8183_mt6358_rt1015_i2s_hw_params),
};

unsafe extern "C" fn mt8183_i2s_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    dev_dbg((*rtd).dev, b"%s(), fix format to S32_LE\n\0".as_ptr() as *const c_char, b"mt8183_i2s_hw_params_fixup\0".as_ptr());

    /* fix BE i2s format to S32_LE, clean param mask first */
    snd_mask_reset_range(
        hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
        0,
        SNDRV_PCM_FORMAT_LAST,
    );

    params_set_format(params, SNDRV_PCM_FORMAT_S32_LE);
    0
}

unsafe extern "C" fn mt8183_rt1015_i2s_hw_params_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    dev_dbg((*rtd).dev, b"%s(), fix format to S24_LE\n\0".as_ptr() as *const c_char, b"mt8183_rt1015_i2s_hw_params_fixup\0".as_ptr());

    /* fix BE i2s format to S24_LE, clean param mask first */
    snd_mask_reset_range(
        hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT),
        0,
        SNDRV_PCM_FORMAT_LAST,
    );

    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
    0
}

unsafe extern "C" fn mt8183_mt6358_startup(substream: *mut snd_pcm_substream) -> c_int {
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

static mt8183_mt6358_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_mt6358_startup),
    shutdown: None,
    hw_params: None,
};

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_bt_sco_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
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

static mt8183_mt6358_ts3a227_max98357_bt_sco_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_mt6358_ts3a227_max98357_bt_sco_startup),
    shutdown: None,
    hw_params: None,
};

/* FE and BE SND_SOC_DAILINK_DEFS are supplied by the surrounding ASoC macro layer in C.
 * The generated component arrays are referenced by name below where the C source does.
 */

unsafe extern "C" fn mt8183_mt6358_tdm_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut mt8183_mt6358_ts3a227_max98357_priv;
    let mut ret: c_int;

    if IS_ERR((*priv_).pin_states[PIN_TDM_OUT_ON] as *const c_void) {
        return PTR_ERR((*priv_).pin_states[PIN_TDM_OUT_ON] as *const c_void);
    }

    ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_TDM_OUT_ON]);
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
            b"mt8183_mt6358_tdm_startup\0".as_ptr(),
            ret,
        );
    }

    ret
}

unsafe extern "C" fn mt8183_mt6358_tdm_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut mt8183_mt6358_ts3a227_max98357_priv;
    let ret: c_int;

    if IS_ERR((*priv_).pin_states[PIN_TDM_OUT_OFF] as *const c_void) {
        return;
    }

    ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_TDM_OUT_OFF]);
    if ret != 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
            b"mt8183_mt6358_tdm_shutdown\0".as_ptr(),
            ret,
        );
    }
}

static mt8183_mt6358_tdm_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_mt6358_tdm_startup),
    shutdown: Some(mt8183_mt6358_tdm_shutdown),
    hw_params: None,
};

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_wov_startup(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut mt8183_mt6358_ts3a227_max98357_priv;

    pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_WOV])
}

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_wov_shutdown(
    substream: *mut snd_pcm_substream,
) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut mt8183_mt6358_ts3a227_max98357_priv;
    let ret: c_int;

    ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_DEFAULT]);
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
            b"mt8183_mt6358_ts3a227_max98357_wov_shutdown\0".as_ptr(),
            ret,
        );
    }
}

static mt8183_mt6358_ts3a227_max98357_wov_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(mt8183_mt6358_ts3a227_max98357_wov_startup),
    shutdown: Some(mt8183_mt6358_ts3a227_max98357_wov_shutdown),
    hw_params: None,
};

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_hdmi_init(
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut mt8183_mt6358_ts3a227_max98357_priv;
    let mut ret: c_int;

    ret = snd_soc_card_jack_new(
        (*rtd).card,
        b"HDMI Jack\0".as_ptr() as *const c_char,
        SND_JACK_AVOUT,
        &mut (*priv_).hdmi_jack,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        &mut (*priv_).hdmi_jack,
        ptr::null_mut(),
    )
}

unsafe extern "C" fn mt8183_bt_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let ret = mt8183_dai_i2s_set_share(
        afe,
        b"I2S5\0".as_ptr() as *const c_char,
        b"I2S0\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"Failed to set up shared clocks\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

unsafe extern "C" fn mt8183_i2s2_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let cmpnt_afe = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(cmpnt_afe) as *mut mtk_base_afe;
    let ret = mt8183_dai_i2s_set_share(
        afe,
        b"I2S2\0".as_ptr() as *const c_char,
        b"I2S3\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"Failed to set up shared clocks\n\0".as_ptr() as *const c_char);
        return ret;
    }
    0
}

static mut mt8183_mt6358_ts3a227_dai_links: [snd_soc_dai_link; 19] = [
    snd_soc_dai_link { name: b"Playback_1\0".as_ptr() as *const c_char, stream_name: b"Playback_1\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: &mt8183_mt6358_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Playback_2\0".as_ptr() as *const c_char, stream_name: b"Playback_2\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: &mt8183_mt6358_ts3a227_max98357_bt_sco_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Playback_3\0".as_ptr() as *const c_char, stream_name: b"Playback_3\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Capture_1\0".as_ptr() as *const c_char, stream_name: b"Capture_1\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: &mt8183_mt6358_ts3a227_max98357_bt_sco_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Capture_2\0".as_ptr() as *const c_char, stream_name: b"Capture_2\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Capture_3\0".as_ptr() as *const c_char, stream_name: b"Capture_3\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: &mt8183_mt6358_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Capture_Mono_1\0".as_ptr() as *const c_char, stream_name: b"Capture_Mono_1\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 0, capture_only: 1, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Playback_HDMI\0".as_ptr() as *const c_char, stream_name: b"Playback_HDMI\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE], dynamic: 1, playback_only: 1, capture_only: 0, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Wake on Voice\0".as_ptr() as *const c_char, stream_name: b"Wake on Voice\0".as_ptr() as *const c_char, cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, ignore_suspend: 1, ignore: 1, no_pcm: 0, dai_fmt: 0, ops: &mt8183_mt6358_ts3a227_max98357_wov_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"Primary Codec\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"PCM 1\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"PCM 2\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"I2S0\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: &mt8183_mt6358_i2s_ops, be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"I2S1\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: &mt8183_mt6358_i2s_ops, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), init: None },
    snd_soc_dai_link { name: b"I2S2\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 1, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: &mt8183_mt6358_i2s_ops, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), init: Some(mt8183_i2s2_init) },
    snd_soc_dai_link { name: b"I2S3\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
    snd_soc_dai_link { name: b"I2S5\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, ignore_suspend: 1, ignore: 0, no_pcm: 1, dai_fmt: 0, ops: &mt8183_mt6358_i2s_ops, be_hw_params_fixup: None, init: Some(mt8183_bt_init) },
    snd_soc_dai_link { name: b"TDM\0".as_ptr() as *const c_char, stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 1, capture_only: 0, ignore_suspend: 1, ignore: 1, no_pcm: 1, dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_IB_IF | SND_SOC_DAIFMT_CBP_CFP, ops: &mt8183_mt6358_tdm_ops, be_hw_params_fixup: Some(mt8183_i2s_hw_params_fixup), init: Some(mt8183_mt6358_ts3a227_max98357_hdmi_init) },
    snd_soc_dai_link { name: ptr::null(), stream_name: ptr::null(), cpus: ptr::null_mut(), num_cpus: 0, codecs: ptr::null_mut(), num_codecs: 0, platforms: ptr::null_mut(), num_platforms: 0, trigger: [0, 0], dynamic: 0, playback_only: 0, capture_only: 0, ignore_suspend: 0, ignore: 0, no_pcm: 0, dai_fmt: 0, ops: ptr::null(), be_hw_params_fixup: None, init: None },
];

static mt8183_mt6358_ts3a227_max98357_snd_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static mt8183_mt6358_ts3a227_max98357_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static mut mt8183_mt6358_ts3a227_max98357_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];

static mut mt8183_mt6358_ts3a227_max98357_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_mt6358_ts3a227_max98357\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: unsafe { mt8183_mt6358_ts3a227_dai_links.as_mut_ptr() },
    num_links: 19,
    codec_conf: ptr::null_mut(),
    num_configs: 0,
    controls: mt8183_mt6358_ts3a227_max98357_snd_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt8183_mt6358_ts3a227_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 2,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 0,
};

static mut mt8183_mt6358_ts3a227_max98357b_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_mt6358_ts3a227_max98357b\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: unsafe { mt8183_mt6358_ts3a227_dai_links.as_mut_ptr() },
    num_links: 19,
    codec_conf: ptr::null_mut(),
    num_configs: 0,
    controls: mt8183_mt6358_ts3a227_max98357_snd_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt8183_mt6358_ts3a227_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 2,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 0,
};

static mut mt8183_mt6358_ts3a227_rt1015_amp_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: RT1015_DEV0_NAME, of_node: ptr::null_mut(), dai_name: ptr::null() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: RT1015_DEV1_NAME, of_node: ptr::null_mut(), dai_name: ptr::null() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
];

static mut mt8183_mt6358_ts3a227_rt1015_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_mt6358_ts3a227_rt1015\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: unsafe { mt8183_mt6358_ts3a227_dai_links.as_mut_ptr() },
    num_links: 19,
    codec_conf: unsafe { mt8183_mt6358_ts3a227_rt1015_amp_conf.as_mut_ptr() },
    num_configs: 2,
    controls: mt8183_mt6358_ts3a227_max98357_snd_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt8183_mt6358_ts3a227_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 2,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 0,
};

static mut mt8183_mt6358_ts3a227_rt1015p_card: snd_soc_card = snd_soc_card {
    name: b"mt8183_mt6358_ts3a227_rt1015p\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: unsafe { mt8183_mt6358_ts3a227_dai_links.as_mut_ptr() },
    num_links: 19,
    codec_conf: ptr::null_mut(),
    num_configs: 0,
    controls: mt8183_mt6358_ts3a227_max98357_snd_controls.as_ptr(),
    num_controls: 2,
    dapm_widgets: mt8183_mt6358_ts3a227_max98357_dapm_widgets.as_ptr(),
    num_dapm_widgets: 2,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 0,
};

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_headset_init(
    component: *mut snd_soc_component,
) -> c_int {
    let mut ret: c_int;
    let priv_ = snd_soc_card_get_drvdata((*component).card) as *mut mt8183_mt6358_ts3a227_max98357_priv;

    /* Enable Headset and 4 Buttons Jack detection */
    ret = snd_soc_card_jack_new_pins(
        (*component).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut (*priv_).headset_jack,
        mt8183_mt6358_ts3a227_max98357_jack_pins.as_mut_ptr(),
        array_size!(mt8183_mt6358_ts3a227_max98357_jack_pins),
    );
    if ret != 0 {
        return ret;
    }

    ret = ts3a227e_enable_jack_detect(component, &mut (*priv_).headset_jack);

    ret
}

static mut mt8183_mt6358_ts3a227_max98357_headset_dev: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component { name: ptr::null(), of_node: ptr::null_mut(), dai_name: ptr::null() },
    init: Some(mt8183_mt6358_ts3a227_max98357_headset_init),
};

unsafe extern "C" fn mt8183_mt6358_ts3a227_max98357_dev_probe(
    pdev: *mut platform_device,
) -> c_int {
    let mut card: *mut snd_soc_card;
    let mut platform_node: *mut device_node;
    let mut ec_codec: *mut device_node;
    let mut hdmi_codec: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut priv_: *mut mt8183_mt6358_ts3a227_max98357_priv;
    let mut ret: c_int;
    let mut i: usize;

    platform_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,platform\0".as_ptr() as *const c_char,
        0,
    );
    if platform_node.is_null() {
        dev_err(&mut (*pdev).dev, b"Property 'platform' missing or invalid\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    card = of_device_get_match_data(&mut (*pdev).dev) as *mut snd_soc_card;
    if card.is_null() {
        of_node_put(platform_node);
        return -EINVAL;
    }
    (*card).dev = &mut (*pdev).dev;

    ec_codec = of_parse_phandle((*pdev).dev.of_node, b"mediatek,ec-codec\0".as_ptr() as *const c_char, 0);
    hdmi_codec = of_parse_phandle((*pdev).dev.of_node, b"mediatek,hdmi-codec\0".as_ptr() as *const c_char, 0);

    i = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        if !ec_codec.is_null() && strcmp((*dai_link).name, b"Wake on Voice\0".as_ptr() as *const c_char) == 0 {
            (*(*dai_link).cpus.add(0)).name = ptr::null();
            (*(*dai_link).cpus.add(0)).of_node = ec_codec;
            (*(*dai_link).cpus.add(0)).dai_name = ptr::null();
            (*(*dai_link).codecs.add(0)).name = ptr::null();
            (*(*dai_link).codecs.add(0)).of_node = ec_codec;
            (*(*dai_link).codecs.add(0)).dai_name = b"Wake on Voice\0".as_ptr() as *const c_char;
            (*(*dai_link).platforms.add(0)).of_node = ec_codec;
            (*dai_link).ignore = 0;
        }

        if strcmp((*dai_link).name, b"I2S3\0".as_ptr() as *const c_char) == 0 {
            if card == &mut mt8183_mt6358_ts3a227_max98357_card
                || card == &mut mt8183_mt6358_ts3a227_max98357b_card
            {
                (*dai_link).be_hw_params_fixup = Some(mt8183_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_mt6358_i2s_ops;
                (*dai_link).cpus = i2s3_max98357a_cpus;
                (*dai_link).num_cpus = 1;
                (*dai_link).codecs = i2s3_max98357a_codecs;
                (*dai_link).num_codecs = 1;
                (*dai_link).platforms = i2s3_max98357a_platforms;
                (*dai_link).num_platforms = 1;
            } else if card == &mut mt8183_mt6358_ts3a227_rt1015_card {
                (*dai_link).be_hw_params_fixup = Some(mt8183_rt1015_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_mt6358_rt1015_i2s_ops;
                (*dai_link).cpus = i2s3_rt1015_cpus;
                (*dai_link).num_cpus = 1;
                (*dai_link).codecs = i2s3_rt1015_codecs;
                (*dai_link).num_codecs = 2;
                (*dai_link).platforms = i2s3_rt1015_platforms;
                (*dai_link).num_platforms = 1;
            } else if card == &mut mt8183_mt6358_ts3a227_rt1015p_card {
                (*dai_link).be_hw_params_fixup = Some(mt8183_rt1015_i2s_hw_params_fixup);
                (*dai_link).ops = &mt8183_mt6358_i2s_ops;
                (*dai_link).cpus = i2s3_rt1015p_cpus;
                (*dai_link).num_cpus = 1;
                (*dai_link).codecs = i2s3_rt1015p_codecs;
                (*dai_link).num_codecs = 1;
                (*dai_link).platforms = i2s3_rt1015p_platforms;
                (*dai_link).num_platforms = 1;
            }
        }

        if card == &mut mt8183_mt6358_ts3a227_max98357b_card {
            if strcmp((*dai_link).name, b"I2S2\0".as_ptr() as *const c_char) == 0
                || strcmp((*dai_link).name, b"I2S3\0".as_ptr() as *const c_char) == 0
            {
                (*dai_link).dai_fmt =
                    SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
            }
        }

        if !hdmi_codec.is_null() && strcmp((*dai_link).name, b"TDM\0".as_ptr() as *const c_char) == 0 {
            (*(*dai_link).codecs).of_node = hdmi_codec;
            (*dai_link).ignore = 0;
        }

        if (*(*dai_link).platforms).name.is_null() {
            (*(*dai_link).platforms).of_node = platform_node;
        }
        i += 1;
    }

    mt8183_mt6358_ts3a227_max98357_headset_dev.dlc.of_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"mediatek,headset-codec\0".as_ptr() as *const c_char,
        0,
    );
    if !mt8183_mt6358_ts3a227_max98357_headset_dev.dlc.of_node.is_null() {
        (*card).aux_dev = &mut mt8183_mt6358_ts3a227_max98357_headset_dev;
        (*card).num_aux_devs = 1;
    }

    priv_ = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<mt8183_mt6358_ts3a227_max98357_priv>(),
        GFP_KERNEL,
    ) as *mut mt8183_mt6358_ts3a227_max98357_priv;
    if priv_.is_null() {
        ret = -ENOMEM;
        goto_out(platform_node, ec_codec, hdmi_codec);
        return ret;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    (*priv_).pinctrl = devm_pinctrl_get(&mut (*pdev).dev);
    if IS_ERR((*priv_).pinctrl as *const c_void) {
        dev_err(&mut (*pdev).dev, b"%s devm_pinctrl_get failed\n\0".as_ptr() as *const c_char, b"mt8183_mt6358_ts3a227_max98357_dev_probe\0".as_ptr());
        ret = PTR_ERR((*priv_).pinctrl as *const c_void);
        goto_out(platform_node, ec_codec, hdmi_codec);
        return ret;
    }

    i = 0;
    while i < PIN_STATE_MAX {
        (*priv_).pin_states[i] = pinctrl_lookup_state((*priv_).pinctrl, mt8183_pin_str[i]);
        if IS_ERR((*priv_).pin_states[i] as *const c_void) {
            ret = PTR_ERR((*priv_).pin_states[i] as *const c_void);
            dev_info(
                &mut (*pdev).dev,
                b"%s Can't find pin state %s %d\n\0".as_ptr() as *const c_char,
                b"mt8183_mt6358_ts3a227_max98357_dev_probe\0".as_ptr(),
                mt8183_pin_str[i],
                ret,
            );
        }
        i += 1;
    }

    if !IS_ERR((*priv_).pin_states[PIN_TDM_OUT_OFF] as *const c_void) {
        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_TDM_OUT_OFF]);
        if ret != 0 {
            dev_info(
                &mut (*pdev).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8183_mt6358_ts3a227_max98357_dev_probe\0".as_ptr(),
                ret,
            );
        }
    }

    if !IS_ERR((*priv_).pin_states[PIN_STATE_DEFAULT] as *const c_void) {
        ret = pinctrl_select_state((*priv_).pinctrl, (*priv_).pin_states[PIN_STATE_DEFAULT]);
        if ret != 0 {
            dev_info(
                &mut (*pdev).dev,
                b"%s failed to select state %d\n\0".as_ptr() as *const c_char,
                b"mt8183_mt6358_ts3a227_max98357_dev_probe\0".as_ptr(),
                ret,
            );
        }
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);

    goto_out(platform_node, ec_codec, hdmi_codec);
    ret
}

unsafe fn goto_out(
    platform_node: *mut device_node,
    ec_codec: *mut device_node,
    hdmi_codec: *mut device_node,
) {
    of_node_put(platform_node);
    of_node_put(ec_codec);
    of_node_put(hdmi_codec);
}

/* CONFIG_OF */
static mt8183_mt6358_ts3a227_max98357_dt_match: [of_device_id; 5] = [
    of_device_id { compatible: b"mediatek,mt8183_mt6358_ts3a227_max98357\0".as_ptr() as *const c_char, data: unsafe { &mt8183_mt6358_ts3a227_max98357_card as *const _ as *const c_void } },
    of_device_id { compatible: b"mediatek,mt8183_mt6358_ts3a227_max98357b\0".as_ptr() as *const c_char, data: unsafe { &mt8183_mt6358_ts3a227_max98357b_card as *const _ as *const c_void } },
    of_device_id { compatible: b"mediatek,mt8183_mt6358_ts3a227_rt1015\0".as_ptr() as *const c_char, data: unsafe { &mt8183_mt6358_ts3a227_rt1015_card as *const _ as *const c_void } },
    of_device_id { compatible: b"mediatek,mt8183_mt6358_ts3a227_rt1015p\0".as_ptr() as *const c_char, data: unsafe { &mt8183_mt6358_ts3a227_rt1015p_card as *const _ as *const c_void } },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, mt8183_mt6358_ts3a227_max98357_dt_match); */

static mt8183_mt6358_ts3a227_max98357_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"mt8183_mt6358_ts3a227\0".as_ptr() as *const c_char,
        of_match_table: mt8183_mt6358_ts3a227_max98357_dt_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(mt8183_mt6358_ts3a227_max98357_dev_probe),
};

/* module_platform_driver(mt8183_mt6358_ts3a227_max98357_driver); */

/* Module information */
/* MODULE_DESCRIPTION("MT8183-MT6358-TS3A227-MAX98357 ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Shunli Wang <shunli.wang@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("mt8183_mt6358_ts3a227_max98357 soc card"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
