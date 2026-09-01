// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip machine ASoC driver for boards using a RT5645/RT5650 CODEC.
 *
 * Copyright (c) 2015, ROCKCHIP CORPORATION.  All rights reserved.
 */

// Dependencies in the original C source:
// <linux/module.h>, <linux/platform_device.h>, <linux/slab.h>,
// <linux/delay.h>, <sound/core.h>, <sound/jack.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, <sound/soc.h>, "rockchip_i2s.h",
// "../codecs/rt5645.h"

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

const DRV_NAME: &[u8] = b"rockchip-snd-rt5645\0";

const EINVAL: c_int = 22;

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn rt5645_set_jack_detect(
        component: *mut snd_soc_component,
        hs_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
        btn_jack: *mut snd_soc_jack,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn of_node_put(node: *mut device_node);
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
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
pub struct snd_soc_component {
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
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub kind: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;

const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_SOC_CLOCK_IN: c_int = 0;

const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

const DAPM_HP: c_int = 0;
const DAPM_SPK: c_int = 1;
const DAPM_MIC: c_int = 2;

static mut HEADSET_JACK: snd_soc_jack = snd_soc_jack { _private: [] };

static mut HEADSET_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphones\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

static RK_DAPM_WIDGETS: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget {
        name: b"Headphones\0".as_ptr() as *const c_char,
        kind: DAPM_HP,
    },
    snd_soc_dapm_widget {
        name: b"Speakers\0".as_ptr() as *const c_char,
        kind: DAPM_SPK,
    },
    snd_soc_dapm_widget {
        name: b"Headset Mic\0".as_ptr() as *const c_char,
        kind: DAPM_MIC,
    },
    snd_soc_dapm_widget {
        name: b"Int Mic\0".as_ptr() as *const c_char,
        kind: DAPM_MIC,
    },
];

static RK_AUDIO_MAP: [snd_soc_dapm_route; 8] = [
    /* Input Lines */
    snd_soc_dapm_route {
        sink: b"DMIC L2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Int Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMIC R2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Int Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RECMIXL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"RECMIXR\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
    /* Output Lines */
    snd_soc_dapm_route {
        sink: b"Headphones\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphones\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speakers\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speakers\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPOR\0".as_ptr() as *const c_char,
    },
];

static RK_MC_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        name: b"Headphones\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"Speakers\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"Headset Mic\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        name: b"Int Mic\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn rk_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int = 0;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk: c_int;

    match params_rate(params) {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => {
            mclk = 12288000;
        }
        11025 | 22050 | 44100 | 88200 => {
            mclk = 11289600;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk as c_uint, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Can't set codec clock %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk as c_uint, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*codec_dai).dev,
            b"Can't set codec clock %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn rk_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = (*runtime).card;
    let mut ret: c_int;

    /* Enable Headset and 4 Buttons Jack detection */
    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADPHONE
            | SND_JACK_MICROPHONE
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3,
        &mut HEADSET_JACK,
        HEADSET_JACK_PINS.as_mut_ptr(),
        HEADSET_JACK_PINS.len() as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"New Headset Jack failed! (%d)\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    rt5645_set_jack_detect(
        (*snd_soc_rtd_to_codec(runtime, 0)).component,
        &mut HEADSET_JACK,
        &mut HEADSET_JACK,
        &mut HEADSET_JACK,
    )
}

static RK_AIF1_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(rk_aif1_hw_params),
};

// SND_SOC_DAILINK_DEFS(pcm,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "rt5645-aif1")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut PCM_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];
static mut PCM_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: b"rt5645-aif1\0".as_ptr() as *const c_char,
}];
static mut PCM_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut RK_DAILINK: snd_soc_dai_link = snd_soc_dai_link {
    name: b"rt5645\0".as_ptr() as *const c_char,
    stream_name: b"rt5645 PCM\0".as_ptr() as *const c_char,
    init: Some(rk_init),
    ops: &RK_AIF1_OPS,
    /* set rt5645 as slave */
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    cpus: unsafe { PCM_CPUS.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { PCM_CODECS.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { PCM_PLATFORMS.as_mut_ptr() },
    num_platforms: 1,
};

static mut SND_SOC_CARD_RK: snd_soc_card = snd_soc_card {
    name: b"I2S-RT5650\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut RK_DAILINK },
    num_links: 1,
    dapm_widgets: RK_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: RK_DAPM_WIDGETS.len() as c_uint,
    dapm_routes: RK_AUDIO_MAP.as_ptr(),
    num_dapm_routes: RK_AUDIO_MAP.len() as c_uint,
    controls: RK_MC_CONTROLS.as_ptr(),
    num_controls: RK_MC_CONTROLS.len() as c_uint,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn snd_rk_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int = 0;
    let card: *mut snd_soc_card = &mut SND_SOC_CARD_RK;
    let np: *mut device_node = (*pdev).dev.of_node;

    /* register the soc card */
    (*card).dev = &mut (*pdev).dev;

    (*RK_DAILINK.codecs).of_node = of_parse_phandle(
        np,
        b"rockchip,audio-codec\0".as_ptr() as *const c_char,
        0,
    );
    if (*RK_DAILINK.codecs).of_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'rockchip,audio-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    (*RK_DAILINK.cpus).of_node = of_parse_phandle(
        np,
        b"rockchip,i2s-controller\0".as_ptr() as *const c_char,
        0,
    );
    if (*RK_DAILINK.cpus).of_node.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"Property 'rockchip,i2s-controller' missing or invalid\n\0".as_ptr()
                as *const c_char,
        );
        ret = -EINVAL;
        of_node_put((*RK_DAILINK.codecs).of_node);
        (*RK_DAILINK.codecs).of_node = ptr::null_mut();
        return ret;
    }

    (*RK_DAILINK.platforms).of_node = (*RK_DAILINK.cpus).of_node;

    ret = snd_soc_of_parse_card_name(card, b"rockchip,model\0".as_ptr() as *const c_char);
    if ret != 0 {
        of_node_put((*RK_DAILINK.cpus).of_node);
        (*RK_DAILINK.cpus).of_node = ptr::null_mut();
        of_node_put((*RK_DAILINK.codecs).of_node);
        (*RK_DAILINK.codecs).of_node = ptr::null_mut();
        return ret;
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Soc register card failed %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        of_node_put((*RK_DAILINK.cpus).of_node);
        (*RK_DAILINK.cpus).of_node = ptr::null_mut();
        of_node_put((*RK_DAILINK.codecs).of_node);
        (*RK_DAILINK.codecs).of_node = ptr::null_mut();
        return ret;
    }

    ret
}

unsafe extern "C" fn snd_rk_mc_remove(_pdev: *mut platform_device) {
    of_node_put((*RK_DAILINK.cpus).of_node);
    (*RK_DAILINK.cpus).of_node = ptr::null_mut();
    of_node_put((*RK_DAILINK.codecs).of_node);
    (*RK_DAILINK.codecs).of_node = ptr::null_mut();
}

static ROCKCHIP_RT5645_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"rockchip,rockchip-audio-rt5645\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, rockchip_rt5645_of_match);

static mut SND_RK_MC_DRIVER: platform_driver = platform_driver {
    probe: Some(snd_rk_mc_probe),
    remove: Some(snd_rk_mc_remove),
    driver: platform_driver_inner {
        name: DRV_NAME.as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: ROCKCHIP_RT5645_OF_MATCH.as_ptr(),
    },
};

// module_platform_driver(snd_rk_mc_driver);
// MODULE_AUTHOR("Xing Zheng <zhengxing@rock-chips.com>");
// MODULE_DESCRIPTION("Rockchip rt5645 machine ASoC driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
