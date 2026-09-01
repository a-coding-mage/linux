// SPDX-License-Identifier: MIT
/*
 * Machine driver for AMD ACP Audio engine using Realtek RT5645 codec
 *
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * This file is modified from rt288 machine driver
 */

/* Rust translation of includes:
 * <sound/core.h>
 * <sound/soc.h>
 * <sound/pcm.h>
 * <sound/pcm_params.h>
 * <sound/soc-dapm.h>
 * <sound/jack.h>
 * <linux/module.h>
 * <linux/i2c.h>
 * <linux/acpi.h>
 * "../codecs/rt5645.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const CZ_PLAT_CLK: c_uint = 24_000_000;

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    static RT5645_PLL1_S_MCLK: c_int;
    static RT5645_SCLK_S_PLL1: c_int;
    static SND_SOC_CLOCK_OUT: c_int;

    static SND_JACK_HEADPHONE: c_uint;
    static SND_JACK_MICROPHONE: c_uint;
    static SND_JACK_BTN_0: c_uint;
    static SND_JACK_BTN_1: c_uint;
    static SND_JACK_BTN_2: c_uint;
    static SND_JACK_BTN_3: c_uint;

    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn rt5645_set_jack_detect(
        component: *mut snd_soc_component,
        hp_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
        btn_jack: *mut snd_soc_jack,
    );
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
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
pub struct device {
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar_compat,
    pub invert: c_uchar_compat,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_int,
}

type c_uchar_compat = u8;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: *const c_void,
    pub get: *const c_void,
    pub put: *const c_void,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
    pub driver_data: c_ulong,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static mut cz_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut cz_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphones\0".as_ptr() as *const c_char,
        mask: unsafe { SND_JACK_HEADPHONE },
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: unsafe { SND_JACK_MICROPHONE },
    },
];

unsafe extern "C" fn cz_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int = 0;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);

    ret = snd_soc_dai_set_pll(
        codec_dai,
        0,
        RT5645_PLL1_S_MCLK,
        CZ_PLAT_CLK,
        params_rate(params).wrapping_mul(512),
    );
    if ret < 0 {
        dev_err(
            (*rtd).dev,
            b"can't set codec pll: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        RT5645_SCLK_S_PLL1,
        params_rate(params).wrapping_mul(512),
        SND_SOC_CLOCK_OUT,
    );
    if ret < 0 {
        dev_err(
            (*rtd).dev,
            b"can't set codec sysclk: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn cz_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ret: c_int;
    let card: *mut snd_soc_card;
    let codec: *mut snd_soc_component;

    codec = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    card = (*rtd).card;

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADPHONE
            | SND_JACK_MICROPHONE
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3,
        &mut cz_jack,
        cz_jack_pins.as_mut_ptr(),
        array_size(&cz_jack_pins),
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"HP jack creation failed %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    rt5645_set_jack_detect(codec, &mut cz_jack, &mut cz_jack, &mut cz_jack);

    0
}

static cz_aif1_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(cz_aif1_hw_params),
};

/* SND_SOC_DAILINK_DEF(designware1,
 *     DAILINK_COMP_ARRAY(COMP_CPU("designware-i2s.1")));
 */
static designware1: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"designware-i2s.1\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

/* SND_SOC_DAILINK_DEF(designware2,
 *     DAILINK_COMP_ARRAY(COMP_CPU("designware-i2s.2")));
 */
static designware2: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"designware-i2s.2\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

/* SND_SOC_DAILINK_DEF(codec,
 *     DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5650:00", "rt5645-aif1")));
 */
static codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-10EC5650:00\0".as_ptr() as *const c_char,
    dai_name: b"rt5645-aif1\0".as_ptr() as *const c_char,
}];

/* SND_SOC_DAILINK_DEF(platform,
 *     DAILINK_COMP_ARRAY(COMP_PLATFORM("acp_audio_dma.0")));
 */
static platform: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"acp_audio_dma.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut cz_dai_rt5650: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"amd-rt5645-play\0".as_ptr() as *const c_char,
        stream_name: b"RT5645_AIF1\0".as_ptr() as *const c_char,
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
        init: Some(cz_init),
        ops: &cz_aif1_ops,
        /* SND_SOC_DAILINK_REG(designware1, codec, platform) */
        cpus: designware1.as_ptr(),
        num_cpus: 1,
        codecs: codec.as_ptr(),
        num_codecs: 1,
        platforms: platform.as_ptr(),
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"amd-rt5645-cap\0".as_ptr() as *const c_char,
        stream_name: b"RT5645_AIF1\0".as_ptr() as *const c_char,
        dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
        init: None,
        ops: &cz_aif1_ops,
        /* SND_SOC_DAILINK_REG(designware2, codec, platform) */
        cpus: designware2.as_ptr(),
        num_cpus: 1,
        codecs: codec.as_ptr(),
        num_codecs: 1,
        platforms: platform.as_ptr(),
        num_platforms: 1,
    },
];

/* SND_SOC_DAPM_HP/SPK/MIC macro details are supplied by <sound/soc-dapm.h>. */
static cz_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"Headphones\0".as_ptr() as *const c_char,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"Speakers\0".as_ptr() as *const c_char,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"Headset Mic\0".as_ptr() as *const c_char,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"Int Mic\0".as_ptr() as *const c_char,
        reg: 0,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
];

static cz_audio_route: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: b"Headphones\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphones\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOR\0".as_ptr() as *const c_char,
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
];

/* SOC_DAPM_PIN_SWITCH macro details are supplied by <sound/soc.h>. */
static cz_mc_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: 0,
        name: b"Headphones\0".as_ptr() as *const c_char,
        info: ptr::null(),
        get: ptr::null(),
        put: ptr::null(),
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: 0,
        name: b"Speakers\0".as_ptr() as *const c_char,
        info: ptr::null(),
        get: ptr::null(),
        put: ptr::null(),
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: 0,
        name: b"Headset Mic\0".as_ptr() as *const c_char,
        info: ptr::null(),
        get: ptr::null(),
        put: ptr::null(),
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: 0,
        name: b"Int Mic\0".as_ptr() as *const c_char,
        info: ptr::null(),
        get: ptr::null(),
        put: ptr::null(),
        private_value: 0,
    },
];

static mut cz_card: snd_soc_card = snd_soc_card {
    name: b"acprt5650\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { cz_dai_rt5650.as_mut_ptr() },
    num_links: unsafe { array_size(&cz_dai_rt5650) as c_int },
    dapm_widgets: cz_widgets.as_ptr(),
    num_dapm_widgets: array_size(&cz_widgets) as c_int,
    dapm_routes: cz_audio_route.as_ptr(),
    num_dapm_routes: array_size(&cz_audio_route) as c_int,
    controls: cz_mc_controls.as_ptr(),
    num_controls: array_size(&cz_mc_controls) as c_int,
};

unsafe extern "C" fn cz_probe(pdev: *mut platform_device) -> c_int {
    let ret: c_int;
    let card: *mut snd_soc_card;

    card = &mut cz_card;
    cz_card.dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, card as *mut c_void);
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut cz_card);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"devm_snd_soc_register_card(%s) failed: %d\n\0".as_ptr() as *const c_char,
            cz_card.name,
            ret,
        );
        return ret;
    }
    0
}

/* #ifdef CONFIG_ACPI */
static cz_audio_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'A' as c_char,
            b'M' as c_char,
            b'D' as c_char,
            b'I' as c_char,
            b'1' as c_char,
            b'0' as c_char,
            b'0' as c_char,
            b'2' as c_char,
            0,
        ],
        driver_data: 0,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(acpi, cz_audio_acpi_match); */
/* #endif */

/* ACPI_PTR(cz_audio_acpi_match) preserves conditional pointer/null behavior in C. */
static mut cz_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"cz-rt5645\0".as_ptr() as *const c_char,
        acpi_match_table: cz_audio_acpi_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(cz_probe),
};

/* module_platform_driver(cz_pcm_driver); */

/* MODULE_AUTHOR("akshu.agrawal@amd.com"); */
/* MODULE_DESCRIPTION("cz-rt5645 audio support"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
