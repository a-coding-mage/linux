// SPDX-License-Identifier: GPL-2.0
//
// ASoC machine driver for Snow boards

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const FIN_PLL_RATE: c_uint = 24000000;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct clk {
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub dai_fmt: c_uint,
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
pub struct snow_priv {
    pub dai_link: snd_soc_dai_link,
    pub clk_i2s_bus: *mut clk,
}

static mut links_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut links_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut links_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_long) -> c_int;
    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_parse_phandle(
        node: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        codec: *mut device_node,
        link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn of_clk_get_by_name(node: *mut device_node, name: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_of_put_dai_link_codecs(link: *mut snd_soc_dai_link);
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn clk_put(clk: *mut clk);
}

unsafe extern "C" fn snow_card_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    static pll_rate: [c_uint; 5] = [73728000u32, 67737602u32, 49152000u32, 45158401u32, 32768001u32];
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let priv_: *mut snow_priv = snd_soc_card_get_drvdata((*rtd).card) as *mut snow_priv;
    let mut bfs: c_int;
    let mut psr: c_int;
    let rfs: c_int;
    let bitwidth: c_int;
    let rclk: c_ulong;
    let mut freq: c_long = -(EINVAL as c_long);
    let mut ret: c_int;

    bitwidth = snd_pcm_format_width(params_format(params));
    if bitwidth < 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"Invalid bit-width: %d\n\0".as_ptr() as *const c_char,
            bitwidth,
        );
        return bitwidth;
    }

    if bitwidth != 16 && bitwidth != 24 {
        dev_err(
            (*(*rtd).card).dev,
            b"Unsupported bit-width: %d\n\0".as_ptr() as *const c_char,
            bitwidth,
        );
        return -EINVAL;
    }

    bfs = 2 * bitwidth;

    match params_rate(params) {
        16000 | 22050 | 24000 | 32000 | 44100 | 48000 | 88200 | 96000 => {
            rfs = 8 * bfs;
        }
        64000 => {
            rfs = 384;
        }
        8000 | 11025 | 12000 => {
            rfs = 16 * bfs;
        }
        _ => {
            return -EINVAL;
        }
    }

    rclk = (params_rate(params) as c_ulong).wrapping_mul(rfs as c_ulong);

    psr = 8;
    while psr > 0 {
        let mut i: usize = 0;
        while i < pll_rate.len() {
            if pll_rate[i].wrapping_sub(rclk.wrapping_mul(psr as c_ulong) as c_uint) <= 2 {
                freq = pll_rate[i] as c_long;
                break;
            }
            i += 1;
        }
        psr /= 2;
    }
    if freq < 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"Unsupported RCLK rate: %lu\n\0".as_ptr() as *const c_char,
            rclk,
        );
        return -EINVAL;
    }

    ret = clk_set_rate((*priv_).clk_i2s_bus, freq);
    if ret < 0 {
        dev_err(
            (*(*rtd).card).dev,
            b"I2S bus clock rate set failed\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    0
}

static snow_card_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(snow_card_hw_params),
};

unsafe extern "C" fn snow_late_probe(card: *mut snd_soc_card) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));

    /* In the multi-codec case codec_dais 0 is MAX98095 and 1 is HDMI. */
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    /* Set the MCLK rate for the codec */
    snd_soc_dai_set_sysclk(codec_dai, 0, FIN_PLL_RATE, SND_SOC_CLOCK_IN)
}

static mut snow_snd: snd_soc_card = snd_soc_card {
    name: b"Snow-I2S\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    late_probe: Some(snow_late_probe),
    dai_link: ptr::null_mut(),
    num_links: 0,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn snow_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let card: *mut snd_soc_card = &mut snow_snd;
    let mut cpu: *mut device_node;
    let mut codec: *mut device_node;
    let link: *mut snd_soc_dai_link;
    let priv_: *mut snow_priv;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<snow_priv>(), GFP_KERNEL) as *mut snow_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    link = &mut (*priv_).dai_link;

    (*link).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;

    (*link).name = b"Primary\0".as_ptr() as *const c_char;
    (*link).stream_name = (*link).name;

    (*link).cpus = links_cpus.as_mut_ptr();
    (*link).num_cpus = links_cpus.len() as c_uint;
    (*link).codecs = links_codecs.as_mut_ptr();
    (*link).num_codecs = links_codecs.len() as c_uint;
    (*link).platforms = links_platforms.as_mut_ptr();
    (*link).num_platforms = links_platforms.len() as c_uint;

    (*card).dai_link = link;
    (*card).num_links = 1;
    (*card).dev = dev;

    /* Try new DT bindings with HDMI support first. */
    cpu = of_get_child_by_name((*dev).of_node, b"cpu\0".as_ptr() as *const c_char);

    if !cpu.is_null() {
        (*link).ops = &snow_card_ops;

        (*(*link).cpus).of_node =
            of_parse_phandle(cpu, b"sound-dai\0".as_ptr() as *const c_char, 0);
        of_node_put(cpu);

        if (*(*link).cpus).of_node.is_null() {
            dev_err(
                dev,
                b"Failed parsing cpu/sound-dai property\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }

        codec = of_get_child_by_name((*dev).of_node, b"codec\0".as_ptr() as *const c_char);
        ret = snd_soc_of_get_dai_link_codecs(dev, codec, link);
        of_node_put(codec);

        if ret < 0 {
            of_node_put((*(*link).cpus).of_node);
            dev_err(dev, b"Failed parsing codec node\n\0".as_ptr() as *const c_char);
            return ret;
        }

        (*priv_).clk_i2s_bus =
            of_clk_get_by_name((*(*link).cpus).of_node, b"i2s_opclk0\0".as_ptr() as *const c_char);
        if IS_ERR((*priv_).clk_i2s_bus as *const c_void) {
            snd_soc_of_put_dai_link_codecs(link);
            of_node_put((*(*link).cpus).of_node);
            return PTR_ERR((*priv_).clk_i2s_bus as *const c_void);
        }
    } else {
        (*(*link).codecs).dai_name = b"HiFi\0".as_ptr() as *const c_char;

        (*(*link).cpus).of_node = of_parse_phandle(
            (*dev).of_node,
            b"samsung,i2s-controller\0".as_ptr() as *const c_char,
            0,
        );
        if (*(*link).cpus).of_node.is_null() {
            dev_err(dev, b"i2s-controller property parse error\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        (*(*link).codecs).of_node = of_parse_phandle(
            (*dev).of_node,
            b"samsung,audio-codec\0".as_ptr() as *const c_char,
            0,
        );
        if (*(*link).codecs).of_node.is_null() {
            of_node_put((*(*link).cpus).of_node);
            dev_err(dev, b"audio-codec property parse error\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    (*(*link).platforms).of_node = (*(*link).cpus).of_node;

    /* Update card-name if provided through DT, else use default name */
    snd_soc_of_parse_card_name(card, b"samsung,model\0".as_ptr() as *const c_char);

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card failed\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

unsafe extern "C" fn snow_remove(pdev: *mut platform_device) {
    let priv_: *mut snow_priv = platform_get_drvdata(pdev) as *mut snow_priv;
    let link: *mut snd_soc_dai_link = &mut (*priv_).dai_link;

    of_node_put((*(*link).cpus).of_node);
    of_node_put((*(*link).codecs).of_node);
    snd_soc_of_put_dai_link_codecs(link);

    clk_put((*priv_).clk_i2s_bus);
}

static snow_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"google,snow-audio-max98090\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"google,snow-audio-max98091\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"google,snow-audio-max98095\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, snow_of_match); */

static mut snow_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"snow-audio\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
        of_match_table: snow_of_match.as_ptr(),
    },
    probe: Some(snow_probe),
    remove: Some(snow_remove),
};

/* module_platform_driver(snow_driver); */

/* MODULE_DESCRIPTION("ALSA SoC Audio machine driver for Snow"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
