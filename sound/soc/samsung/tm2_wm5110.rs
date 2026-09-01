// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2015 - 2016 Samsung Electronics Co., Ltd.
//
// Authors: Inha Song <ideal.song@samsung.com>
//          Sylwester Nawrocki <s.nawrocki@samsung.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

/*
 * Dependencies from:
 * <linux/clk.h>, <linux/gpio/consumer.h>, <linux/module.h>, <linux/of.h>,
 * <sound/pcm_params.h>, <sound/soc.h>, "i2s.h", "../codecs/wm5110.h"
 */

/*
 * The source clock is XCLKOUT with its mux set to the external fixed rate
 * oscillator (XXTI).
 */
const MCLK_RATE: c_uint = 24000000_u32;

const TM2_DAI_AIF1: usize = 0;
const TM2_DAI_AIF2: usize = 1;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static SAMSUNG_I2S_DAI: [c_char; 0];

    static mut snd_soc_suspend: unsafe extern "C" fn(*mut device) -> c_int;
    static mut snd_soc_resume: unsafe extern "C" fn(*mut device) -> c_int;
    static mut snd_soc_poweroff: unsafe extern "C" fn(*mut device) -> c_int;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_component_set_pll(
        component: *mut snd_soc_component,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn snd_soc_find_dai(dlc: *mut snd_soc_dlc) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_channel_map(
        dai: *mut snd_soc_dai,
        tx_num: c_uint,
        tx_slot: *mut c_uint,
        rx_num: c_uint,
        rx_slot: *mut c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_count_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
    ) -> c_int;
    fn of_parse_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn of_parse_phandle_with_fixed_args(
        np: *mut device_node,
        list_name: *const c_char,
        cell_count: c_int,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn snd_soc_get_dai_name(args: *mut of_phandle_args, dai_name: *mut *const c_char) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

extern "Rust" {
    static WM5110_FLL1_REFCLK: c_int;
    static WM5110_FLL1: c_int;
    static WM5110_FLL2_REFCLK: c_int;
    static WM5110_FLL2: c_int;
    static ARIZONA_FLL_SRC_MCLK1: c_int;
    static ARIZONA_CLK_SYSCLK: c_int;
    static ARIZONA_CLK_ASYNCCLK: c_int;
    static ARIZONA_CLK_SRC_FLL1: c_int;
    static ARIZONA_CLK_SRC_FLL2: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SAMSUNG_I2S_OPCLK: c_int;
    static SAMSUNG_I2S_OPCLK_PCLK: c_int;
    static SAMSUNG_I2S_DIV_BCLK: c_int;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_HIGH: c_int;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct of_phandle_args {
    pub np: *mut device_node,
}
pub type snd_soc_bias_level = c_int;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 0;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;

#[repr(C)]
pub struct tm2_machine_priv {
    component: *mut snd_soc_component,
    sysclk_rate: c_uint,
    gpio_mic_bias: *mut gpio_desc,
}

unsafe extern "C" fn tm2_start_sysclk(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut tm2_machine_priv;
    let component = (*priv_).component;
    let mut ret: c_int;

    ret = snd_soc_component_set_pll(
        component,
        WM5110_FLL1_REFCLK,
        ARIZONA_FLL_SRC_MCLK1,
        MCLK_RATE,
        (*priv_).sysclk_rate,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to set FLL1 source: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_pll(
        component,
        WM5110_FLL1,
        ARIZONA_FLL_SRC_MCLK1,
        MCLK_RATE,
        (*priv_).sysclk_rate,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to start FLL1: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        component,
        ARIZONA_CLK_SYSCLK,
        ARIZONA_CLK_SRC_FLL1,
        (*priv_).sysclk_rate,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to set SYSCLK source: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tm2_stop_sysclk(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut tm2_machine_priv;
    let component = (*priv_).component;
    let mut ret: c_int;

    ret = snd_soc_component_set_pll(component, WM5110_FLL1, 0, 0, 0);
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to stop FLL1: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(component, ARIZONA_CLK_SYSCLK, ARIZONA_CLK_SRC_FLL1, 0, 0);
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to stop SYSCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tm2_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let priv_ = snd_soc_card_get_drvdata((*rtd).card) as *mut tm2_machine_priv;

    match params_rate(params) {
        4000 | 8000 | 12000 | 16000 | 24000 | 32000 | 48000 | 96000 | 192000 => {
            /* Highest possible SYSCLK frequency: 147.456MHz */
            (*priv_).sysclk_rate = 147456000_u32;
        }
        11025 | 22050 | 44100 | 88200 | 176400 => {
            /* Highest possible SYSCLK frequency: 135.4752 MHz */
            (*priv_).sysclk_rate = 135475200_u32;
        }
        _ => {
            dev_err(
                component_dev(component),
                c"Not supported sample rate: %d\n".as_ptr(),
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    tm2_start_sysclk((*rtd).card)
}

static tm2_aif1_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(tm2_aif1_hw_params),
    hw_free: None,
};

unsafe extern "C" fn tm2_aif2_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let asyncclk_rate: c_uint;
    let mut ret: c_int;

    match params_rate(params) {
        8000 | 12000 | 16000 => {
            /* Highest possible ASYNCCLK frequency: 49.152MHz */
            asyncclk_rate = 49152000_u32;
        }
        11025 => {
            /* Highest possible ASYNCCLK frequency: 45.1584 MHz */
            asyncclk_rate = 45158400_u32;
        }
        _ => {
            dev_err(
                component_dev(component),
                c"Not supported sample rate: %d\n".as_ptr(),
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    ret = snd_soc_component_set_pll(
        component,
        WM5110_FLL2_REFCLK,
        ARIZONA_FLL_SRC_MCLK1,
        MCLK_RATE,
        asyncclk_rate,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to set FLL2 source: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_pll(
        component,
        WM5110_FLL2,
        ARIZONA_FLL_SRC_MCLK1,
        MCLK_RATE,
        asyncclk_rate,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to start FLL2: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_component_set_sysclk(
        component,
        ARIZONA_CLK_ASYNCCLK,
        ARIZONA_CLK_SRC_FLL2,
        asyncclk_rate,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to set ASYNCCLK source: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn tm2_aif2_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let ret: c_int;

    /* disable FLL2 */
    ret = snd_soc_component_set_pll(component, WM5110_FLL2, ARIZONA_FLL_SRC_MCLK1, 0, 0);
    if ret < 0 {
        dev_err(component_dev(component), c"Failed to stop FLL2: %d\n".as_ptr(), ret);
    }

    ret
}

static tm2_aif2_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(tm2_aif2_hw_params),
    hw_free: Some(tm2_aif2_hw_free),
};

unsafe extern "C" fn tm2_hdmi_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let bfs: c_uint;
    let bitwidth: c_int;
    let mut ret: c_int;

    bitwidth = snd_pcm_format_width(params_format(params));
    if bitwidth < 0 {
        dev_err((*(*rtd).card).dev, c"Invalid bit-width: %d\n".as_ptr(), bitwidth);
        return bitwidth;
    }

    match bitwidth {
        48 => {
            bfs = 64;
        }
        16 => {
            bfs = 32;
        }
        _ => {
            dev_err((*(*rtd).card).dev, c"Unsupported bit-width: %d\n".as_ptr(), bitwidth);
            return -EINVAL;
        }
    }

    match params_rate(params) {
        48000 | 96000 | 192000 => {}
        _ => {
            dev_err(
                (*(*rtd).card).dev,
                c"Unsupported sample rate: %d\n".as_ptr(),
                params_rate(params),
            );
            return -EINVAL;
        }
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, SAMSUNG_I2S_OPCLK, 0, SAMSUNG_I2S_OPCLK_PCLK);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_clkdiv(cpu_dai, SAMSUNG_I2S_DIV_BCLK, bfs as c_int);
    if ret < 0 {
        return ret;
    }

    0
}

static tm2_hdmi_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(tm2_hdmi_hw_params),
    hw_free: None,
};

unsafe extern "C" fn tm2_mic_bias(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut tm2_machine_priv;

    match event {
        x if x == SND_SOC_DAPM_PRE_PMU => {
            gpiod_set_value_cansleep((*priv_).gpio_mic_bias, 1);
        }
        x if x == SND_SOC_DAPM_POST_PMD => {
            gpiod_set_value_cansleep((*priv_).gpio_mic_bias, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn tm2_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let card_dapm = snd_soc_card_to_dapm(card);
    let rtd: *mut snd_soc_pcm_runtime;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));

    if snd_soc_dapm_to_dev(dapm) != (*snd_soc_rtd_to_codec(rtd, 0)).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(card_dapm) == SND_SOC_BIAS_OFF {
                tm2_start_sysclk(card);
            }
        }
        SND_SOC_BIAS_OFF => {
            tm2_stop_sysclk(card);
        }
        _ => {}
    }

    0
}

static mut tm2_speaker_amp_dev: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dlc {
        of_node: ptr::null_mut(),
        dai_name: ptr::null(),
        name: ptr::null(),
    },
};

unsafe extern "C" fn tm2_late_probe(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut tm2_machine_priv;
    let mut ch_map: [c_uint; 2] = [0, 1];
    let amp_pdm_dai: *mut snd_soc_dai;
    let mut rtd: *mut snd_soc_pcm_runtime;
    let aif1_dai: *mut snd_soc_dai;
    let aif2_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(TM2_DAI_AIF1));
    aif1_dai = snd_soc_rtd_to_codec(rtd, 0);
    (*priv_).component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    ret = snd_soc_dai_set_sysclk(aif1_dai, ARIZONA_CLK_SYSCLK, 0, 0);
    if ret < 0 {
        dev_err((*aif1_dai).dev, c"Failed to set SYSCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(TM2_DAI_AIF2));
    aif2_dai = snd_soc_rtd_to_codec(rtd, 0);

    ret = snd_soc_dai_set_sysclk(aif2_dai, ARIZONA_CLK_ASYNCCLK, 0, 0);
    if ret < 0 {
        dev_err((*aif2_dai).dev, c"Failed to set ASYNCCLK: %d\n".as_ptr(), ret);
        return ret;
    }

    amp_pdm_dai = snd_soc_find_dai(&mut tm2_speaker_amp_dev.dlc);
    if amp_pdm_dai.is_null() {
        return -ENODEV;
    }

    /* Set the MAX98504 V/I sense PDM Tx DAI channel mapping */
    ret = snd_soc_dai_set_channel_map(amp_pdm_dai, ch_map.len() as c_uint, ch_map.as_mut_ptr(), 0, ptr::null_mut());
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(amp_pdm_dai, 0x3, 0x0, 2, 16);
    if ret < 0 {
        return ret;
    }

    0
}

static tm2_controls: [snd_kcontrol_new; 9] = [
    SOC_DAPM_PIN_SWITCH!(c"HP"),
    SOC_DAPM_PIN_SWITCH!(c"SPK"),
    SOC_DAPM_PIN_SWITCH!(c"RCV"),
    SOC_DAPM_PIN_SWITCH!(c"VPS"),
    SOC_DAPM_PIN_SWITCH!(c"HDMI"),
    SOC_DAPM_PIN_SWITCH!(c"Main Mic"),
    SOC_DAPM_PIN_SWITCH!(c"Sub Mic"),
    SOC_DAPM_PIN_SWITCH!(c"Third Mic"),
    SOC_DAPM_PIN_SWITCH!(c"Headset Mic"),
];

static tm2_dapm_widgets: [snd_soc_dapm_widget_desc; 9] = [
    SND_SOC_DAPM_HP!(c"HP", None),
    SND_SOC_DAPM_SPK!(c"SPK", None),
    SND_SOC_DAPM_SPK!(c"RCV", None),
    SND_SOC_DAPM_LINE!(c"VPS", None),
    SND_SOC_DAPM_LINE!(c"HDMI", None),
    SND_SOC_DAPM_MIC!(c"Main Mic", Some(tm2_mic_bias)),
    SND_SOC_DAPM_MIC!(c"Sub Mic", None),
    SND_SOC_DAPM_MIC!(c"Third Mic", None),
    SND_SOC_DAPM_MIC!(c"Headset Mic", None),
];

static tm2_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"tm2-audio".as_ptr(),
};

static mut tm2_ext_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"Voice call".as_ptr(),
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 48000,
            rates: unsafe { SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 48000,
            rates: unsafe { SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
    },
    snd_soc_dai_driver {
        name: c"Bluetooth".as_ptr(),
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 16000,
            rates: unsafe { SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 16000,
            rates: unsafe { SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 },
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
    },
];

SND_SOC_DAILINK_DEFS!(
    aif1,
    DAILINK_COMP_ARRAY!(COMP_CPU!(SAMSUNG_I2S_DAI)),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(ptr::null(), c"wm5110-aif1")),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    voice,
    DAILINK_COMP_ARRAY!(COMP_CPU!(SAMSUNG_I2S_DAI)),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(ptr::null(), c"wm5110-aif2")),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    bt,
    DAILINK_COMP_ARRAY!(COMP_CPU!(SAMSUNG_I2S_DAI)),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(ptr::null(), c"wm5110-aif3")),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    hdmi,
    DAILINK_COMP_ARRAY!(COMP_EMPTY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

static mut tm2_dai_links: [snd_soc_dai_link; 4] = [
    snd_soc_dai_link {
        name: c"WM5110 AIF1".as_ptr(),
        stream_name: c"HiFi Primary".as_ptr(),
        ops: &tm2_aif1_ops,
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
        ignore_suspend: 0,
        cpus: unsafe { aif1_cpus.as_mut_ptr() },
        codecs: unsafe { aif1_codecs.as_mut_ptr() },
        platforms: unsafe { aif1_platforms.as_mut_ptr() },
    },
    snd_soc_dai_link {
        name: c"WM5110 Voice".as_ptr(),
        stream_name: c"Voice call".as_ptr(),
        ops: &tm2_aif2_ops,
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
        ignore_suspend: 1,
        cpus: unsafe { voice_cpus.as_mut_ptr() },
        codecs: unsafe { voice_codecs.as_mut_ptr() },
        platforms: unsafe { voice_platforms.as_mut_ptr() },
    },
    snd_soc_dai_link {
        name: c"WM5110 BT".as_ptr(),
        stream_name: c"Bluetooth".as_ptr(),
        ops: ptr::null(),
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
        ignore_suspend: 1,
        cpus: unsafe { bt_cpus.as_mut_ptr() },
        codecs: unsafe { bt_codecs.as_mut_ptr() },
        platforms: unsafe { bt_platforms.as_mut_ptr() },
    },
    snd_soc_dai_link {
        name: c"HDMI".as_ptr(),
        stream_name: c"i2s1".as_ptr(),
        ops: &tm2_hdmi_ops,
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC },
        ignore_suspend: 0,
        cpus: unsafe { hdmi_cpus.as_mut_ptr() },
        codecs: unsafe { hdmi_codecs.as_mut_ptr() },
        platforms: unsafe { hdmi_platforms.as_mut_ptr() },
    },
];

static mut tm2_card: snd_soc_card = snd_soc_card {
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { tm2_dai_links.as_mut_ptr() },
    num_links: 0,
    controls: tm2_controls.as_ptr(),
    num_controls: tm2_controls.len() as c_uint,
    dapm_widgets: tm2_dapm_widgets.as_ptr(),
    num_dapm_widgets: tm2_dapm_widgets.len() as c_uint,
    aux_dev: unsafe { &mut tm2_speaker_amp_dev },
    num_aux_devs: 1,
    late_probe: Some(tm2_late_probe),
    set_bias_level: Some(tm2_set_bias_level),
};

unsafe extern "C" fn tm2_probe(pdev: *mut platform_device) -> c_int {
    let mut cpu_dai_node: [*mut device_node; 2] = [ptr::null_mut(); 2];
    let mut codec_dai_node: [*mut device_node; 2] = [ptr::null_mut(); 2];
    let mut cells_name: *const c_char = ptr::null();
    let dev = &mut (*pdev).dev as *mut device;
    let card = &mut tm2_card as *mut snd_soc_card;
    let priv_: *mut tm2_machine_priv;
    let mut dai_link: *mut snd_soc_dai_link;
    let num_codecs: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<tm2_machine_priv>(), GFP_KERNEL) as *mut tm2_machine_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);
    (*card).dev = dev;

    (*priv_).gpio_mic_bias = devm_gpiod_get(dev, c"mic-bias".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpio_mic_bias as *const c_void) {
        dev_err(dev, c"Failed to get mic bias gpio\n".as_ptr());
        return PTR_ERR((*priv_).gpio_mic_bias as *const c_void);
    }

    ret = snd_soc_of_parse_card_name(card, c"model".as_ptr());
    if ret < 0 {
        dev_err(dev, c"Card name is not specified\n".as_ptr());
        return ret;
    }

    ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
    if ret < 0 {
        /* Backwards compatible way */
        ret = snd_soc_of_parse_audio_routing(card, c"samsung,audio-routing".as_ptr());
        if ret < 0 {
            dev_err(dev, c"Audio routing is not specified or invalid\n".as_ptr());
            return ret;
        }
    }

    (*(*card).aux_dev.add(0)).dlc.of_node = of_parse_phandle((*dev).of_node, c"audio-amplifier".as_ptr(), 0);
    if (*(*card).aux_dev.add(0)).dlc.of_node.is_null() {
        dev_err(dev, c"audio-amplifier property invalid or missing\n".as_ptr());
        return -EINVAL;
    }

    num_codecs = of_count_phandle_with_args((*dev).of_node, c"audio-codec".as_ptr(), ptr::null());

    /* Skip the HDMI link if not specified in DT */
    if num_codecs > 1 {
        (*card).num_links = tm2_dai_links.len() as c_uint;
        cells_name = c"#sound-dai-cells".as_ptr();
    } else {
        (*card).num_links = (tm2_dai_links.len() - 1) as c_uint;
    }

    i = 0;
    while i < num_codecs {
        let mut args: of_phandle_args = core::mem::zeroed();

        ret = of_parse_phandle_with_args((*dev).of_node, c"i2s-controller".as_ptr(), cells_name, i, &mut args);
        if ret != 0 {
            dev_err(dev, c"i2s-controller property parse error: %d\n".as_ptr(), i);
            ret = -EINVAL;
            goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
            return ret;
        }
        cpu_dai_node[i as usize] = args.np;

        codec_dai_node[i as usize] = of_parse_phandle((*dev).of_node, c"audio-codec".as_ptr(), i);
        if codec_dai_node[i as usize].is_null() {
            dev_err(dev, c"audio-codec property parse error\n".as_ptr());
            ret = -EINVAL;
            goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
            return ret;
        }
        i += 1;
    }

    /* Initialize WM5110 - I2S and HDMI - I2S1 DAI links */
    i = 0;
    while i < (*card).num_links as c_int {
        let mut dai_index: c_uint = 0; /* WM5110 */
        dai_link = (*card).dai_link.add(i as usize);

        (*(*dai_link).cpus).name = ptr::null();
        (*(*dai_link).platforms).name = ptr::null();

        if num_codecs > 1 && i == (*card).num_links as c_int - 1 {
            dai_index = 1; /* HDMI */
        }

        (*(*dai_link).codecs).of_node = codec_dai_node[dai_index as usize];
        (*(*dai_link).cpus).of_node = cpu_dai_node[dai_index as usize];
        (*(*dai_link).platforms).of_node = cpu_dai_node[dai_index as usize];
        i += 1;
    }

    if num_codecs > 1 {
        let mut args: of_phandle_args = core::mem::zeroed();

        /* HDMI DAI link (I2S1) */
        i = (*card).num_links as c_int - 1;

        ret = of_parse_phandle_with_fixed_args((*dev).of_node, c"audio-codec".as_ptr(), 0, 1, &mut args);
        if ret != 0 {
            dev_err(dev, c"audio-codec property parse error\n".as_ptr());
            goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
            return ret;
        }

        ret = snd_soc_get_dai_name(&mut args, &mut (*(*card).dai_link.add(i as usize)).codecs.as_mut().unwrap().dai_name);
        if ret != 0 {
            dev_err(dev, c"Unable to get codec_dai_name\n".as_ptr());
            goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
            return ret;
        }
    }

    ret = devm_snd_soc_register_component(
        dev,
        &tm2_component,
        tm2_ext_dai.as_mut_ptr(),
        tm2_ext_dai.len() as c_int,
    );
    if ret < 0 {
        dev_err(dev, c"Failed to register component: %d\n".as_ptr(), ret);
        goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        dev_err_probe(dev, ret, c"Failed to register card\n".as_ptr());
        goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
        return ret;
    }

    goto_dai_node_put(ret, num_codecs, &mut codec_dai_node, &mut cpu_dai_node, card);
    ret
}

unsafe fn goto_dai_node_put(
    ret: c_int,
    num_codecs: c_int,
    codec_dai_node: &mut [*mut device_node; 2],
    cpu_dai_node: &mut [*mut device_node; 2],
    card: *mut snd_soc_card,
) {
    let mut i: c_int = 0;
    while i < num_codecs {
        of_node_put(codec_dai_node[i as usize]);
        of_node_put(cpu_dai_node[i as usize]);
        i += 1;
    }

    of_node_put((*(*card).aux_dev.add(0)).dlc.of_node);
    let _ = ret;
}

unsafe extern "C" fn tm2_pm_prepare(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_soc_card;

    tm2_stop_sysclk(card)
}

unsafe extern "C" fn tm2_pm_complete(dev: *mut device) {
    let card = dev_get_drvdata(dev) as *mut snd_soc_card;

    tm2_start_sysclk(card);
}

static tm2_pm_ops: dev_pm_ops = dev_pm_ops {
    prepare: Some(tm2_pm_prepare),
    suspend: unsafe { Some(snd_soc_suspend) },
    resume: unsafe { Some(snd_soc_resume) },
    complete: Some(tm2_pm_complete),
    freeze: unsafe { Some(snd_soc_suspend) },
    thaw: unsafe { Some(snd_soc_resume) },
    poweroff: unsafe { Some(snd_soc_poweroff) },
    restore: unsafe { Some(snd_soc_resume) },
};

static tm2_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"samsung,tm2-audio".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, tm2_of_match);

static mut tm2_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"tm2-audio".as_ptr(),
        pm: &tm2_pm_ops,
        of_match_table: tm2_of_match.as_ptr(),
    },
    probe: Some(tm2_probe),
};
module_platform_driver!(tm2_driver);

MODULE_AUTHOR!(c"Inha Song <ideal.song@samsung.com>");
MODULE_DESCRIPTION!(c"ALSA SoC Exynos TM2 Audio Support");
MODULE_LICENSE!(c"GPL v2");

#[inline]
unsafe fn component_dev(component: *mut snd_soc_component) -> *mut device {
    (*component).dev
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dlc {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_dlc,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub ignore_suspend: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_uint,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_uint,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dapm_context, snd_soc_bias_level) -> c_int>,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(*mut device)>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

macro_rules! SOC_DAPM_PIN_SWITCH {
    ($name:expr) => {
        snd_kcontrol_new { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_HP {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_SPK {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_LINE {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}
macro_rules! SND_SOC_DAPM_MIC {
    ($name:expr, $event:expr) => {
        snd_soc_dapm_widget_desc { _private: [] }
    };
}

macro_rules! MODULE_DEVICE_TABLE {
    ($bus:ident, $name:ident) => {};
}
macro_rules! module_platform_driver {
    ($driver:ident) => {};
}
macro_rules! MODULE_AUTHOR {
    ($author:expr) => {};
}
macro_rules! MODULE_DESCRIPTION {
    ($description:expr) => {};
}
macro_rules! MODULE_LICENSE {
    ($license:expr) => {};
}

macro_rules! SND_SOC_DAILINK_DEFS {
    ($name:ident, $cpus:expr, $codecs:expr, $platforms:expr) => {
        static mut $name _cpus: [snd_soc_dai_link_component; 1] = $cpus;
        static mut $name _codecs: [snd_soc_dai_link_component; 1] = $codecs;
        static mut $name _platforms: [snd_soc_dai_link_component; 1] = $platforms;
    };
}

macro_rules! DAILINK_COMP_ARRAY {
    ($comp:expr) => {
        [$comp]
    };
}
macro_rules! COMP_CPU {
    ($name:expr) => {
        snd_soc_dai_link_component {
            name: unsafe { $name.as_ptr() },
            of_node: ptr::null_mut(),
            dai_name: ptr::null(),
        }
    };
}
macro_rules! COMP_CODEC {
    ($of_node:expr, $dai_name:expr) => {
        snd_soc_dai_link_component {
            name: ptr::null(),
            of_node: $of_node,
            dai_name: $dai_name.as_ptr(),
        }
    };
}
macro_rules! COMP_EMPTY {
    () => {
        snd_soc_dai_link_component {
            name: ptr::null(),
            of_node: ptr::null_mut(),
            dai_name: ptr::null(),
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
