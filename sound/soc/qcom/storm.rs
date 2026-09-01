// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
 *
 * storm.c -- ALSA SoC machine driver for QTi ipq806x-based Storm board
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const STORM_SYSCLK_MULT: c_uint = 4;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

type SndPcmFormatT = c_int;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub ops: *const snd_soc_ops,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
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
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: module;

    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn params_format(params: *mut snd_pcm_hw_params) -> SndPcmFormatT;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_format_width(format: SndPcmFormatT) -> c_int;
    fn snd_soc_rtd_to_cpu(
        rtd: *mut snd_soc_pcm_runtime,
        num: c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

unsafe extern "C" fn storm_ops_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let soc_runtime = snd_soc_substream_to_rtd(substream);
    let card = (*soc_runtime).card;
    let format = params_format(params);
    let rate = params_rate(params);
    let sysclk_freq: c_uint;
    let bitwidth: c_int;
    let ret: c_int;

    bitwidth = snd_pcm_format_width(format);
    if bitwidth < 0 {
        dev_err(
            (*card).dev,
            c"invalid bit width given: %d\n".as_ptr(),
            bitwidth,
        );
        return bitwidth;
    }

    /*
     * as the CPU DAI is the I2S bus master and no system clock is needed by
     * the MAX98357a DAC, simply set the system clock to be a constant
     * multiple of the bit clock for the clock divider
     */
    sysclk_freq = rate
        .wrapping_mul(bitwidth as c_uint)
        .wrapping_mul(2)
        .wrapping_mul(STORM_SYSCLK_MULT);

    ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(soc_runtime, 0), 0, sysclk_freq, 0);
    if ret != 0 {
        dev_err(
            (*card).dev,
            c"error setting sysclk to %u: %d\n".as_ptr(),
            sysclk_freq,
            ret,
        );
        return ret;
    }

    0
}

static storm_soc_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(storm_ops_hw_params),
};

/* SND_SOC_DAILINK_DEFS(hifi,
 *     DAILINK_COMP_ARRAY(COMP_EMPTY()),
 *     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "HiFi")),
 *     DAILINK_COMP_ARRAY(COMP_EMPTY()));
 */
static mut hifi_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut hifi_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut hifi_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];

static mut storm_dai_link: snd_soc_dai_link = snd_soc_dai_link {
    name: c"Primary".as_ptr(),
    stream_name: c"Primary".as_ptr(),
    ops: &storm_soc_ops,
    /* SND_SOC_DAILINK_REG(hifi) */
    cpus: unsafe { hifi_cpus.as_mut_ptr() },
    codecs: unsafe { hifi_codecs.as_mut_ptr() },
    platforms: unsafe { hifi_platforms.as_mut_ptr() },
};

unsafe extern "C" fn storm_parse_of(dev: *mut device) -> c_int {
    let dai_link = &raw mut storm_dai_link;
    let np = (*dev).of_node;

    (*(*dai_link).cpus).of_node = of_parse_phandle(np, c"cpu".as_ptr(), 0);
    if (*(*dai_link).cpus).of_node.is_null() {
        dev_err(dev, c"error getting cpu phandle\n".as_ptr());
        return -EINVAL;
    }
    (*(*dai_link).platforms).of_node = (*(*dai_link).cpus).of_node;

    (*(*dai_link).codecs).of_node = of_parse_phandle(np, c"codec".as_ptr(), 0);
    if (*(*dai_link).codecs).of_node.is_null() {
        dev_err(dev, c"error getting codec phandle\n".as_ptr());
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn storm_platform_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card;
    let mut ret: c_int;

    card = devm_kzalloc(
        &raw mut (*pdev).dev,
        size_of::<snd_soc_card>(),
        GFP_KERNEL,
    ) as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    (*card).dev = &raw mut (*pdev).dev;
    (*card).owner = &raw mut THIS_MODULE;

    ret = snd_soc_of_parse_card_name(card, c"qcom,model".as_ptr());
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev,
            c"error parsing card name: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    (*card).dai_link = &raw mut storm_dai_link;
    (*card).num_links = 1;

    ret = storm_parse_of(&raw mut (*pdev).dev);
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev,
            c"error resolving dai links: %d\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev,
            c"error registering soundcard: %d\n".as_ptr(),
            ret,
        );
    }

    ret
}

/* #ifdef CONFIG_OF */
static storm_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"google,storm-audio".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, storm_device_id); */
/* #endif */

static mut storm_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"storm-audio".as_ptr(),
        of_match_table: storm_device_id.as_ptr(),
    },
    probe: Some(storm_platform_probe),
};

/* module_platform_driver(storm_platform_driver); */

/* MODULE_DESCRIPTION("QTi IPQ806x-based Storm Machine Driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
