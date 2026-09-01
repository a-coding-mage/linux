// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sam9x5_wm8731   --	SoC audio for AT91SAM9X5-based boards
 *			that are using WM8731 as codec.
 *
 *  Copyright (C) 2011 Atmel,
 *		  Nicolas Ferre <nicolas.ferre@atmel.com>
 *
 *  Copyright (C) 2013 Paratronic,
 *		  Richard Genoud <richard.genoud@gmail.com>
 *
 * Based on sam9g20_wm8731.c by:
 * Sedji Gaouaou <sedji.gaouaou@atmel.com>
 */

// C dependencies:
// #include <linux/of.h>
// #include <linux/export.h>
// #include <linux/module.h>
// #include <linux/platform_device.h>
// #include <linux/device.h>
// #include <sound/soc.h>
// #include <sound/soc-dai.h>
// #include <sound/soc-dapm.h>
// #include "../codecs/wm8731.h"
// #include "atmel_ssc_dai.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const MCLK_RATE: c_uint = 12_288_000;

const DRV_NAME: &[u8] = b"sam9x5-snd-wm8731\0";

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const WM8731_SYSCLK_XTAL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub sname: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
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
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub drvdata: *mut c_void,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct sam9x5_drvdata {
    ssc_id: c_int,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_alias_get_id(np: *mut device_node, stem: *const c_char) -> c_int;
    fn atmel_ssc_set_audio(ssc_id: c_int) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn atmel_ssc_put_audio(ssc_id: c_int);
    fn of_node_put(node: *mut device_node);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;

    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

/*
 * Logic for a wm8731 as connected on a at91sam9x5ek based board.
 */
unsafe extern "C" fn sam9x5_wm8731_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let dev: *mut device = unsafe { (*rtd).dev };
    let mut ret: c_int;

    unsafe {
        dev_dbg(dev, c"%s called\n".as_ptr(), c"sam9x5_wm8731_init".as_ptr());
    }

    /* set the codec system clock for DAC and ADC */
    ret = unsafe {
        snd_soc_dai_set_sysclk(codec_dai, WM8731_SYSCLK_XTAL, MCLK_RATE, SND_SOC_CLOCK_IN)
    };
    if ret < 0 {
        unsafe {
            dev_err(
                dev,
                c"Failed to set WM8731 SYSCLK: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    0
}

/*
 * Audio paths on at91sam9x5ek board:
 *
 *  |A| ------------> |      | ---R----> Headphone Jack
 *  |T| <----\        |  WM  | ---L--/
 *  |9| ---> CLK <--> | 8731 | <--R----- Line In Jack
 *  |1| <------------ |      | <--L--/
 */
static sam9x5_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: c"Headphone Jack".as_ptr(),
        sname: core::ptr::null(),
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Line In Jack".as_ptr(),
        sname: core::ptr::null(),
    },
];

unsafe extern "C" fn sam9x5_wm8731_driver_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = unsafe { (*pdev).dev.of_node };
    let mut codec_np: *mut device_node;
    let mut cpu_np: *mut device_node;
    let card: *mut snd_soc_card;
    let dai: *mut snd_soc_dai_link;
    let priv_: *mut sam9x5_drvdata;
    let comp: *mut snd_soc_dai_link_component;
    let mut ret: c_int;

    if np.is_null() {
        unsafe {
            dev_err(&raw mut (*pdev).dev, c"No device node supplied\n".as_ptr());
        }
        return -EINVAL;
    }

    card = unsafe {
        devm_kzalloc(
            &raw mut (*pdev).dev,
            core::mem::size_of::<snd_soc_card>(),
            GFP_KERNEL,
        ) as *mut snd_soc_card
    };
    priv_ = unsafe {
        devm_kzalloc(
            &raw mut (*pdev).dev,
            core::mem::size_of::<sam9x5_drvdata>(),
            GFP_KERNEL,
        ) as *mut sam9x5_drvdata
    };
    dai = unsafe {
        devm_kzalloc(
            &raw mut (*pdev).dev,
            core::mem::size_of::<snd_soc_dai_link>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link
    };
    comp = unsafe {
        devm_kzalloc(
            &raw mut (*pdev).dev,
            3usize.wrapping_mul(core::mem::size_of::<snd_soc_dai_link_component>()),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if dai.is_null() || card.is_null() || priv_.is_null() || comp.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    unsafe {
        snd_soc_card_set_drvdata(card, priv_ as *mut c_void);
    }

    unsafe {
        (*card).dev = &raw mut (*pdev).dev;
        (*card).owner = THIS_MODULE;
        (*card).dai_link = dai;
        (*card).num_links = 1;
        (*card).dapm_widgets = sam9x5_dapm_widgets.as_ptr();
        (*card).num_dapm_widgets = sam9x5_dapm_widgets.len() as c_int;

        (*dai).cpus = comp.add(0);
        (*dai).num_cpus = 1;
        (*dai).codecs = comp.add(1);
        (*dai).num_codecs = 1;
        (*dai).platforms = comp.add(2);
        (*dai).num_platforms = 1;

        (*dai).name = c"WM8731".as_ptr();
        (*dai).stream_name = c"WM8731 PCM".as_ptr();
        (*(*dai).codecs).dai_name = c"wm8731-hifi".as_ptr();
        (*dai).init = Some(sam9x5_wm8731_init);
        (*dai).dai_fmt = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }

    ret = unsafe { snd_soc_of_parse_card_name(card, c"atmel,model".as_ptr()) };
    if ret != 0 {
        unsafe {
            dev_err(&raw mut (*pdev).dev, c"atmel,model node missing\n".as_ptr());
        }
        return ret;
    }

    ret = unsafe { snd_soc_of_parse_audio_routing(card, c"atmel,audio-routing".as_ptr()) };
    if ret != 0 {
        unsafe {
            dev_err(
                &raw mut (*pdev).dev,
                c"atmel,audio-routing node missing\n".as_ptr(),
            );
        }
        return ret;
    }

    codec_np = unsafe { of_parse_phandle(np, c"atmel,audio-codec".as_ptr(), 0) };
    if codec_np.is_null() {
        unsafe {
            dev_err(
                &raw mut (*pdev).dev,
                c"atmel,audio-codec node missing\n".as_ptr(),
            );
        }
        ret = -EINVAL;
        return ret;
    }

    unsafe {
        (*(*dai).codecs).of_node = codec_np;
    }

    cpu_np = unsafe { of_parse_phandle(np, c"atmel,ssc-controller".as_ptr(), 0) };
    if cpu_np.is_null() {
        unsafe {
            dev_err(
                &raw mut (*pdev).dev,
                c"atmel,ssc-controller node missing\n".as_ptr(),
            );
        }
        ret = -EINVAL;
        unsafe {
            of_node_put(codec_np);
        }
        return ret;
    }
    unsafe {
        (*(*dai).cpus).of_node = cpu_np;
        (*(*dai).platforms).of_node = cpu_np;

        (*priv_).ssc_id = of_alias_get_id(cpu_np, c"ssc".as_ptr());
    }

    ret = unsafe { atmel_ssc_set_audio((*priv_).ssc_id) };
    if ret != 0 {
        unsafe {
            dev_err(
                &raw mut (*pdev).dev,
                c"Failed to set SSC %d for audio: %d\n".as_ptr(),
                ret,
                (*priv_).ssc_id,
            );
            of_node_put(cpu_np);
            of_node_put(codec_np);
        }
        return ret;
    }

    ret = unsafe { devm_snd_soc_register_card(&raw mut (*pdev).dev, card) };
    if ret != 0 {
        unsafe {
            dev_err(
                &raw mut (*pdev).dev,
                c"Platform device allocation failed\n".as_ptr(),
            );
            atmel_ssc_put_audio((*priv_).ssc_id);
            of_node_put(cpu_np);
            of_node_put(codec_np);
        }
        return ret;
    }

    unsafe {
        dev_dbg(
            &raw mut (*pdev).dev,
            c"%s ok\n".as_ptr(),
            c"sam9x5_wm8731_driver_probe".as_ptr(),
        );

        of_node_put(cpu_np);
        of_node_put(codec_np);
    }

    ret
}

unsafe extern "C" fn sam9x5_wm8731_driver_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = unsafe { platform_get_drvdata(pdev) as *mut snd_soc_card };
    let priv_: *mut sam9x5_drvdata = unsafe { (*card).drvdata as *mut sam9x5_drvdata };

    unsafe {
        atmel_ssc_put_audio((*priv_).ssc_id);
    }
}

static sam9x5_wm8731_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"atmel,sam9x5-wm8731-audio".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, sam9x5_wm8731_of_match);

static mut sam9x5_wm8731_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
        // of_match_ptr(sam9x5_wm8731_of_match)
        of_match_table: sam9x5_wm8731_of_match.as_ptr(),
    },
    probe: Some(sam9x5_wm8731_driver_probe),
    remove: Some(sam9x5_wm8731_driver_remove),
};

// module_platform_driver(sam9x5_wm8731_driver);

/* Module information */
// MODULE_AUTHOR("Nicolas Ferre <nicolas.ferre@atmel.com>");
// MODULE_AUTHOR("Richard Genoud <richard.genoud@gmail.com>");
// MODULE_DESCRIPTION("ALSA SoC machine driver for AT91SAM9x5 - WM8731");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
