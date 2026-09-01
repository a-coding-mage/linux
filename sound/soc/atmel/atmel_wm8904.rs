// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * atmel_wm8904 - Atmel ASoC driver for boards with WM8904 codec.
 *
 * Copyright (C) 2012 Atmel
 *
 * Author: Bo Shen <voice.shen@atmel.com>
 */

/* Dependencies from the original C file:
 * linux/clk.h, linux/module.h, linux/of.h, sound/soc.h,
 * ../codecs/wm8904.h, and atmel_ssc_dai.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_dapm_widget {
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
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
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
pub struct module {
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
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
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
    pub num_dapm_widgets: c_int,
    pub fully_routed: bool,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
}

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_alias_get_id(np: *mut device_node, stem: *const c_char) -> c_int;
    fn atmel_ssc_set_audio(id: c_int) -> c_int;
    fn atmel_ssc_put_audio(id: c_int);
    fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut snd_soc_card;
    fn __dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn __pr_err(fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const WM8904_FLL_MCLK: c_int = 0;
const WM8904_CLK_FLL: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;

unsafe extern "C" {
    fn module_platform_driver(driver: *mut platform_driver);
}

macro_rules! pr_err {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {
        unsafe { __pr_err($fmt.as_ptr() as *const c_char $(, $arg)*); }
    };
}

macro_rules! dev_err {
    ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        unsafe { __dev_err($dev, $fmt.as_ptr() as *const c_char $(, $arg)*); }
    };
}

/* SND_SOC_DAPM_HP("Headphone Jack", NULL),
 * SND_SOC_DAPM_MIC("Mic", NULL),
 * SND_SOC_DAPM_LINE("Line In Jack", NULL),
 */
static atmel_asoc_wm8904_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

unsafe extern "C" fn atmel_asoc_wm8904_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_pll(
        codec_dai,
        WM8904_FLL_MCLK,
        WM8904_FLL_MCLK,
        32768,
        params_rate(params).wrapping_mul(256),
    );
    if ret < 0 {
        pr_err!(b"%s - failed to set wm8904 codec PLL.\0", b"atmel_asoc_wm8904_hw_params\0".as_ptr());
        return ret;
    }

    /*
     * As here wm8904 use FLL output as its system clock
     * so calling set_sysclk won't care freq parameter
     * then we pass 0
     */
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8904_CLK_FLL, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        pr_err!(b"%s -failed to set wm8904 SYSCLK\n\0", b"atmel_asoc_wm8904_hw_params\0".as_ptr());
        return ret;
    }

    0
}

static atmel_asoc_wm8904_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(atmel_asoc_wm8904_hw_params),
};

/* SND_SOC_DAILINK_DEFS(pcm,
 *     DAILINK_COMP_ARRAY(COMP_EMPTY()),
 *     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "wm8904-hifi")),
 *     DAILINK_COMP_ARRAY(COMP_EMPTY()));
 */
static mut pcm_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut pcm_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut pcm_platforms: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: ptr::null_mut() }];

static mut atmel_asoc_wm8904_dailink: snd_soc_dai_link = snd_soc_dai_link {
    name: b"WM8904\0".as_ptr() as *const c_char,
    stream_name: b"WM8904 PCM\0".as_ptr() as *const c_char,
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
    ops: &atmel_asoc_wm8904_ops as *const snd_soc_ops,
    cpus: unsafe { pcm_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { pcm_codecs.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { pcm_platforms.as_mut_ptr() },
    num_platforms: 1,
};

static mut atmel_asoc_wm8904_card: snd_soc_card = snd_soc_card {
    name: b"atmel_asoc_wm8904\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut atmel_asoc_wm8904_dailink as *mut snd_soc_dai_link },
    num_links: 1,
    dapm_widgets: atmel_asoc_wm8904_dapm_widgets.as_ptr(),
    num_dapm_widgets: atmel_asoc_wm8904_dapm_widgets.len() as c_int,
    fully_routed: true,
    dev: ptr::null_mut(),
};

unsafe extern "C" fn atmel_asoc_wm8904_dt_init(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let np: *mut device_node = (*dev).of_node;
    let mut codec_np: *mut device_node;
    let mut cpu_np: *mut device_node;
    let card: *mut snd_soc_card = &mut atmel_asoc_wm8904_card;
    let dailink: *mut snd_soc_dai_link = &mut atmel_asoc_wm8904_dailink;
    let mut ret: c_int;

    if np.is_null() {
        dev_err!(dev, b"only device tree supported\n\0");
        return -EINVAL;
    }

    ret = snd_soc_of_parse_card_name(card, b"atmel,model\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err!(dev, b"failed to parse card name\n\0");
        return ret;
    }

    ret = snd_soc_of_parse_audio_routing(
        card,
        b"atmel,audio-routing\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        dev_err!(dev, b"failed to parse audio routing\n\0");
        return ret;
    }

    cpu_np = of_parse_phandle(
        np,
        b"atmel,ssc-controller\0".as_ptr() as *const c_char,
        0,
    );
    if cpu_np.is_null() {
        dev_err!(dev, b"failed to get dai and pcm info\n\0");
        ret = -EINVAL;
        return ret;
    }
    (*(*dailink).cpus).of_node = cpu_np;
    (*(*dailink).platforms).of_node = cpu_np;
    of_node_put(cpu_np);

    codec_np = of_parse_phandle(np, b"atmel,audio-codec\0".as_ptr() as *const c_char, 0);
    if codec_np.is_null() {
        dev_err!(dev, b"failed to get codec info\n\0");
        ret = -EINVAL;
        return ret;
    }
    (*(*dailink).codecs).of_node = codec_np;
    of_node_put(codec_np);

    0
}

unsafe extern "C" fn atmel_asoc_wm8904_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut atmel_asoc_wm8904_card;
    let dailink: *mut snd_soc_dai_link = &mut atmel_asoc_wm8904_dailink;
    let id: c_int;
    let mut ret: c_int;

    (*card).dev = &mut (*pdev).dev;
    ret = atmel_asoc_wm8904_dt_init(pdev);
    if ret != 0 {
        dev_err!(&mut (*pdev).dev, b"failed to init dt info\n\0");
        return ret;
    }

    id = of_alias_get_id((*(*dailink).cpus).of_node as *mut device_node, b"ssc\0".as_ptr() as *const c_char);
    ret = atmel_ssc_set_audio(id);
    if ret != 0 {
        dev_err!(
            &mut (*pdev).dev,
            b"failed to set SSC %d for audio\n\0",
            id
        );
        return ret;
    }

    ret = snd_soc_register_card(card);
    if ret != 0 {
        dev_err!(&mut (*pdev).dev, b"snd_soc_register_card failed\n\0");
        atmel_ssc_put_audio(id);
        return ret;
    }

    0
}

unsafe extern "C" fn atmel_asoc_wm8904_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = platform_get_drvdata(pdev);
    let dailink: *mut snd_soc_dai_link = &mut atmel_asoc_wm8904_dailink;
    let id: c_int;

    id = of_alias_get_id((*(*dailink).cpus).of_node as *mut device_node, b"ssc\0".as_ptr() as *const c_char);

    snd_soc_unregister_card(card);
    atmel_ssc_put_audio(id);
}

/* Original C code placed this table under #ifdef CONFIG_OF. */
static atmel_asoc_wm8904_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"atmel,asoc-wm8904\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, atmel_asoc_wm8904_dt_ids); */

static mut atmel_asoc_wm8904_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"atmel-wm8904-audio\0".as_ptr() as *const c_char,
        of_match_table: atmel_asoc_wm8904_dt_ids.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const dev_pm_ops },
    },
    probe: Some(atmel_asoc_wm8904_probe),
    remove: Some(atmel_asoc_wm8904_remove),
};

unsafe fn register_atmel_asoc_wm8904_driver() {
    module_platform_driver(&mut atmel_asoc_wm8904_driver);
}

/* Module information */
/* MODULE_AUTHOR("Bo Shen <voice.shen@atmel.com>"); */
/* MODULE_DESCRIPTION("ALSA SoC machine driver for Atmel EK with WM8904 codec"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
