// SPDX-License-Identifier: GPL-2.0-only
/*
 * ASoC driver for TI DAVINCI EVM platform
 *
 * Author:      Vladimir Barinov, <vbarinov@embeddedalley.com>
 * Copyright:   (C) 2007 MontaVista Software, Inc., <source@mvista.com>
 */

// Rust translation of implementation source soc/ti/davinci-evm.c.
// C include dependencies:
// linux/module.h, linux/moduleparam.h, linux/timer.h, linux/interrupt.h,
// linux/platform_device.h, linux/i2c.h, linux/of_platform.h, linux/clk.h,
// sound/core.h, sound/pcm.h, sound/soc.h, asm/dma.h, asm/mach-types.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SND_SOC_CLOCK_OUT: c_int = 0;
const ENOTSUPP: c_int = 524;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;

#[repr(C)]
pub struct clk {
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
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
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card_drvdata_davinci {
    pub mclk: *mut clk,
    pub sysclk: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub num_links: c_int,
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
    pub ops: *const snd_soc_ops,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_uint,
    ) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn of_node_put(node: *mut device_node);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int {
    N as c_int
}

unsafe extern "C" fn evm_startup(substream: *mut snd_pcm_substream) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let soc_card: *mut snd_soc_card = unsafe { (*rtd).card };
    let drvdata: *mut snd_soc_card_drvdata_davinci =
        unsafe { snd_soc_card_get_drvdata(soc_card) as *mut snd_soc_card_drvdata_davinci };

    if unsafe { !(*drvdata).mclk.is_null() } {
        return unsafe { clk_prepare_enable((*drvdata).mclk) };
    }

    0
}

unsafe extern "C" fn evm_shutdown(substream: *mut snd_pcm_substream) {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let soc_card: *mut snd_soc_card = unsafe { (*rtd).card };
    let drvdata: *mut snd_soc_card_drvdata_davinci =
        unsafe { snd_soc_card_get_drvdata(soc_card) as *mut snd_soc_card_drvdata_davinci };

    unsafe { clk_disable_unprepare((*drvdata).mclk) };
}

unsafe extern "C" fn evm_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let cpu_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_cpu(rtd, 0) };
    let soc_card: *mut snd_soc_card = unsafe { (*rtd).card };
    let mut ret: c_int = 0;
    let sysclk: c_uint = unsafe {
        (*(snd_soc_card_get_drvdata(soc_card) as *mut snd_soc_card_drvdata_davinci)).sysclk
    };

    /* set the codec system clock */
    ret = unsafe { snd_soc_dai_set_sysclk(codec_dai, 0, sysclk, SND_SOC_CLOCK_OUT) };
    if ret < 0 {
        return ret;
    }

    /* set the CPU system clock */
    ret = unsafe { snd_soc_dai_set_sysclk(cpu_dai, 0, sysclk, SND_SOC_CLOCK_OUT) };
    if ret < 0 && ret != -ENOTSUPP {
        return ret;
    }

    0
}

static evm_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(evm_startup),
    shutdown: Some(evm_shutdown),
    hw_params: Some(evm_hw_params),
};

/* davinci-evm machine dapm widgets */
// SND_SOC_DAPM_HP("Headphone Jack", NULL), SND_SOC_DAPM_LINE("Line Out", NULL),
// SND_SOC_DAPM_MIC("Mic Jack", NULL), SND_SOC_DAPM_LINE("Line In", NULL).
static aic3x_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget {
        id: 0,
        name: c"Headphone Jack".as_ptr(),
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Line Out".as_ptr(),
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Mic Jack".as_ptr(),
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: c"Line In".as_ptr(),
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
];

/* davinci-evm machine audio_mapnections to the codec pins */
static audio_map: [snd_soc_dapm_route; 11] = [
    /* Headphone connected to HPLOUT, HPROUT */
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"HPLOUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ptr::null(),
        source: c"HPROUT".as_ptr(),
    },
    /* Line Out connected to LLOUT, RLOUT */
    snd_soc_dapm_route {
        sink: c"Line Out".as_ptr(),
        control: ptr::null(),
        source: c"LLOUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Line Out".as_ptr(),
        control: ptr::null(),
        source: c"RLOUT".as_ptr(),
    },
    /* Mic connected to (MIC3L | MIC3R) */
    snd_soc_dapm_route {
        sink: c"MIC3L".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"MIC3R".as_ptr(),
        control: ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Mic Bias".as_ptr(),
        control: ptr::null(),
        source: c"Mic Jack".as_ptr(),
    },
    /* Line In connected to (LINE1L | LINE2L), (LINE1R | LINE2R) */
    snd_soc_dapm_route {
        sink: c"LINE1L".as_ptr(),
        control: ptr::null(),
        source: c"Line In".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LINE2L".as_ptr(),
        control: ptr::null(),
        source: c"Line In".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LINE1R".as_ptr(),
        control: ptr::null(),
        source: c"Line In".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LINE2R".as_ptr(),
        control: ptr::null(),
        source: c"Line In".as_ptr(),
    },
];

/* Logic for a aic3x as connected on a davinci-evm */
unsafe extern "C" fn evm_aic3x_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let dapm: *mut snd_soc_dapm_context = unsafe { snd_soc_card_to_dapm((*rtd).card) };
    let np: *mut device_node = unsafe { (*(*card).dev).of_node };
    let ret: c_int;

    /* Add davinci-evm specific widgets */
    unsafe {
        snd_soc_dapm_new_controls(
            dapm,
            aic3x_dapm_widgets.as_ptr(),
            ARRAY_SIZE(&aic3x_dapm_widgets),
        )
    };

    if !np.is_null() {
        ret = unsafe { snd_soc_of_parse_audio_routing(card, c"ti,audio-routing".as_ptr()) };
        if ret != 0 {
            return ret;
        }
    } else {
        /* Set up davinci-evm specific audio path audio_map */
        unsafe { snd_soc_dapm_add_routes(dapm, audio_map.as_ptr(), ARRAY_SIZE(&audio_map)) };
    }

    /* not connected */
    unsafe { snd_soc_dapm_disable_pin(dapm, c"MONO_LOUT".as_ptr()) };
    unsafe { snd_soc_dapm_disable_pin(dapm, c"HPLCOM".as_ptr()) };
    unsafe { snd_soc_dapm_disable_pin(dapm, c"HPRCOM".as_ptr()) };

    0
}

/*
 * The struct is used as place holder. It will be completely
 * filled with data from dt node.
 */
// SND_SOC_DAILINK_DEFS(evm,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "tlv320aic3x-hifi")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut evm_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut evm_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: c"tlv320aic3x-hifi".as_ptr(),
    of_node: ptr::null_mut(),
}];

static mut evm_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut evm_dai_tlv320aic3x: snd_soc_dai_link = snd_soc_dai_link {
    name: c"TLV320AIC3X".as_ptr(),
    stream_name: c"AIC3X".as_ptr(),
    ops: &evm_ops,
    init: Some(evm_aic3x_init),
    dai_fmt: SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_CBP_CFP | SND_SOC_DAIFMT_IB_NF,
    // SND_SOC_DAILINK_REG(evm)
    cpus: unsafe { evm_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { evm_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { evm_platforms.as_mut_ptr() },
    num_platforms: 1,
};

static davinci_evm_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"ti,da830-evm-audio".as_ptr(),
        data: unsafe { &evm_dai_tlv320aic3x as *const snd_soc_dai_link as *const c_void },
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, davinci_evm_dt_ids);

/* davinci evm audio machine driver */
static mut evm_soc_card: snd_soc_card = snd_soc_card {
    owner: unsafe { THIS_MODULE },
    num_links: 1,
    dai_link: ptr::null_mut(),
    dev: ptr::null_mut(),
};

unsafe extern "C" fn davinci_evm_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = unsafe { (*pdev).dev.of_node };
    let dai: *mut snd_soc_dai_link;
    let mut drvdata: *mut snd_soc_card_drvdata_davinci = ptr::null_mut();
    let mut mclk: *mut clk;
    let mut ret: c_int = 0;

    dai = unsafe { device_get_match_data(&mut (*pdev).dev) as *mut snd_soc_dai_link };
    if dai.is_null() {
        unsafe { dev_err(&mut (*pdev).dev, c"Error: No device match found\n".as_ptr()) };
        return -ENODEV;
    }

    unsafe { evm_soc_card.dai_link = dai };

    unsafe {
        (*(*dai).codecs).of_node = of_parse_phandle(np, c"ti,audio-codec".as_ptr(), 0);
    }
    if unsafe { (*(*dai).codecs).of_node.is_null() } {
        return -EINVAL;
    }

    unsafe {
        (*(*dai).cpus).of_node = of_parse_phandle(np, c"ti,mcasp-controller".as_ptr(), 0);
    }
    if unsafe { (*(*dai).cpus).of_node.is_null() } {
        ret = -EINVAL;
        unsafe { goto_err_put(dai, ret) }
    } else {
        unsafe {
            (*(*dai).platforms).of_node = (*(*dai).cpus).of_node;

            evm_soc_card.dev = &mut (*pdev).dev;
        }
        ret = unsafe { snd_soc_of_parse_card_name(&raw mut evm_soc_card, c"ti,model".as_ptr()) };
        if ret != 0 {
            unsafe { goto_err_put(dai, ret) }
        } else {
            mclk = unsafe { devm_clk_get(&mut (*pdev).dev, c"mclk".as_ptr()) };
            if unsafe { PTR_ERR(mclk as *const c_void) == -(EPROBE_DEFER as isize) } {
                ret = -EPROBE_DEFER;
                unsafe { goto_err_put(dai, ret) }
            } else {
                if unsafe { IS_ERR(mclk as *const c_void) } {
                    unsafe { dev_dbg(&mut (*pdev).dev, c"mclk not found.\n".as_ptr()) };
                    mclk = ptr::null_mut();
                }

                drvdata = unsafe {
                    devm_kzalloc(
                        &mut (*pdev).dev,
                        size_of::<snd_soc_card_drvdata_davinci>(),
                        GFP_KERNEL,
                    ) as *mut snd_soc_card_drvdata_davinci
                };
                if drvdata.is_null() {
                    ret = -ENOMEM;
                    unsafe { goto_err_put(dai, ret) }
                } else {
                    unsafe {
                        (*drvdata).mclk = mclk;
                    }

                    ret = unsafe {
                        of_property_read_u32(
                            np,
                            c"ti,codec-clock-rate".as_ptr(),
                            &mut (*drvdata).sysclk,
                        )
                    };

                    if ret < 0 {
                        if unsafe { (*drvdata).mclk.is_null() } {
                            unsafe {
                                dev_err(
                                    &mut (*pdev).dev,
                                    c"No clock or clock rate defined.\n".as_ptr(),
                                )
                            };
                            ret = -EINVAL;
                            unsafe { goto_err_put(dai, ret) }
                        } else {
                            unsafe {
                                (*drvdata).sysclk = clk_get_rate((*drvdata).mclk);
                            }
                            unsafe { finish_probe(pdev, dai, drvdata, ret) }
                        }
                    } else if unsafe { !(*drvdata).mclk.is_null() } {
                        let requestd_rate: c_uint = unsafe { (*drvdata).sysclk };
                        unsafe {
                            clk_set_rate((*drvdata).mclk, (*drvdata).sysclk);
                            (*drvdata).sysclk = clk_get_rate((*drvdata).mclk);
                        }
                        if unsafe { (*drvdata).sysclk != requestd_rate } {
                            unsafe {
                                dev_warn(
                                    &mut (*pdev).dev,
                                    c"Could not get requested rate %u using %u.\n".as_ptr(),
                                    requestd_rate,
                                    (*drvdata).sysclk,
                                )
                            };
                        }
                        unsafe { finish_probe(pdev, dai, drvdata, ret) }
                    } else {
                        unsafe { finish_probe(pdev, dai, drvdata, ret) }
                    }
                }
            }
        }
    }
}

unsafe fn finish_probe(
    pdev: *mut platform_device,
    dai: *mut snd_soc_dai_link,
    drvdata: *mut snd_soc_card_drvdata_davinci,
    mut ret: c_int,
) -> c_int {
    unsafe { snd_soc_card_set_drvdata(&raw mut evm_soc_card, drvdata as *mut c_void) };
    ret = unsafe { devm_snd_soc_register_card(&mut (*pdev).dev, &raw mut evm_soc_card) };
    if ret != 0 {
        unsafe {
            dev_err_probe(
                &mut (*pdev).dev,
                ret,
                c"snd_soc_register_card() failed\n".as_ptr(),
            )
        };
        return unsafe { goto_err_put(dai, ret) };
    }

    ret
}

unsafe fn goto_err_put(dai: *mut snd_soc_dai_link, ret: c_int) -> c_int {
    unsafe {
        (*(*dai).platforms).of_node = ptr::null_mut();

        if !(*(*dai).cpus).of_node.is_null() {
            of_node_put((*(*dai).cpus).of_node);
            (*(*dai).cpus).of_node = ptr::null_mut();
        }

        if !(*(*dai).codecs).of_node.is_null() {
            of_node_put((*(*dai).codecs).of_node);
            (*(*dai).codecs).of_node = ptr::null_mut();
        }
    }

    ret
}

static mut davinci_evm_driver: platform_driver = platform_driver {
    probe: Some(davinci_evm_probe),
    driver: device_driver {
        name: c"davinci_evm".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
        of_match_table: davinci_evm_dt_ids.as_ptr(),
    },
};

// module_platform_driver(davinci_evm_driver);

// MODULE_AUTHOR("Vladimir Barinov");
// MODULE_DESCRIPTION("TI DAVINCI EVM ASoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
