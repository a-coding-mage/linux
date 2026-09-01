// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra_wm8903.c - Tegra machine ASoC driver for boards using WM8903 codec.
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010-2012 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 *
 * (c) 2009, 2010 Nvidia Graphics Pvt. Ltd.
 *
 * Copyright 2007 Wolfson Microelectronics PLC.
 * Author: Graeme Gregory
 *         graeme.gregory@wolfsonmicro.com or linux@wolfsonmicro.com
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const SND_JACK_MICROPHONE: c_int = 0x0004;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
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
pub struct gpio_desc {
    _private: [u8; 0],
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
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub invert: bool,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
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
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
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
pub struct snd_soc_card {
    pub components: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub fully_routed: bool,
    pub dev: *mut device,
}

#[repr(C)]
pub struct tegra_asoc_data {
    pub mclk_rate: Option<unsafe extern "C" fn(c_uint) -> c_uint>,
    pub card: *mut snd_soc_card,
    pub hp_jack_gpio_active_low: bool,
    pub add_common_dapm_widgets: bool,
    pub add_common_controls: bool,
    pub add_common_snd_ops: bool,
    pub add_mic_jack: bool,
    pub add_hp_jack: bool,
}

#[repr(C)]
pub struct tegra_machine {
    pub asoc: *const tegra_asoc_data,
    pub gpiod_hp_det: *mut gpio_desc,
    pub hp_jack_gpio: *mut snd_soc_jack_gpio,
    pub gpiod_mic_det: *mut gpio_desc,
    pub mic_jack: *mut snd_soc_jack,
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn gpiod_is_active_low(desc: *mut gpio_desc) -> bool;
    fn tegra_asoc_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn wm8903_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        det: c_int,
        shrt: c_int,
    );
    fn snd_soc_dapm_force_enable_pin(
        dapm: *mut snd_soc_dapm_context,
        pin: *const c_char,
    ) -> c_int;
    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn tegra_asoc_machine_probe(pdev: *mut platform_device) -> c_int;
    fn __platform_driver_register(
        driver: *mut platform_driver,
        owner: *mut module,
    ) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

static mut TEGRA_WM8903_MIC_JACK_PINS: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: cstr(b"Mic Jack\0"),
    mask: SND_JACK_MICROPHONE,
}];

unsafe extern "C" fn tegra_wm8903_mclk_rate(srate: c_uint) -> c_uint {
    let mut mclk: c_uint;

    match srate {
        64000 | 88200 | 96000 => {
            mclk = 128u32.wrapping_mul(srate);
        }
        _ => {
            mclk = 256u32.wrapping_mul(srate);
        }
    }
    /* FIXME: Codec only requires >= 3MHz if OSR==0 */
    while mclk < 6000000 {
        mclk = mclk.wrapping_mul(2);
    }

    mclk
}

unsafe extern "C" fn tegra_wm8903_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let machine = snd_soc_card_get_drvdata((*rtd).card) as *mut tegra_machine;
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut err: c_int;

    /*
     * Older version of machine driver was ignoring GPIO polarity,
     * forcing it to active-low.  This means that all older device-trees
     * which set the polarity to active-high are wrong and we need to fix
     * them up.
     */
    if (*(*machine).asoc).hp_jack_gpio_active_low {
        let active_low = gpiod_is_active_low((*machine).gpiod_hp_det);

        (*(*machine).hp_jack_gpio).invert = !active_low;
    }

    err = tegra_asoc_machine_init(rtd);
    if err != 0 {
        return err;
    }

    if (*machine).gpiod_mic_det.is_null() && (*(*machine).asoc).add_mic_jack {
        let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
        let component = (*codec_dai).component;
        let mut shrt: c_int = 0;

        err = snd_soc_card_jack_new_pins(
            (*rtd).card,
            cstr(b"Mic Jack\0"),
            SND_JACK_MICROPHONE,
            (*machine).mic_jack,
            TEGRA_WM8903_MIC_JACK_PINS.as_mut_ptr(),
            TEGRA_WM8903_MIC_JACK_PINS.len() as c_uint,
        );
        if err != 0 {
            dev_err(
                (*rtd).dev,
                cstr(b"Mic Jack creation failed: %d\n\0"),
                err,
            );
            return err;
        }

        if of_property_read_bool((*(*card).dev).of_node, cstr(b"nvidia,headset\0")) {
            shrt = SND_JACK_MICROPHONE;
        }

        wm8903_mic_detect(
            component,
            (*machine).mic_jack,
            SND_JACK_MICROPHONE,
            shrt,
        );
    }

    snd_soc_dapm_force_enable_pin(dapm, cstr(b"MICBIAS\0"));

    0
}

unsafe extern "C" fn tegra_wm8903_remove(card: *mut snd_soc_card) -> c_int {
    let link = (*card).dai_link.offset(0);
    let rtd = snd_soc_get_pcm_runtime(card, link);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;

    wm8903_mic_detect(component, ptr::null_mut(), 0, 0);

    0
}

static mut HIFI_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut HIFI_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: cstr(b"wm8903-hifi\0"),
}];

static mut HIFI_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut TEGRA_WM8903_DAI: snd_soc_dai_link = snd_soc_dai_link {
    name: cstr(b"WM8903\0"),
    stream_name: cstr(b"WM8903 PCM\0"),
    init: Some(tegra_wm8903_init),
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    cpus: unsafe { HIFI_CPUS.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { HIFI_CODECS.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { HIFI_PLATFORMS.as_mut_ptr() },
    num_platforms: 1,
};

static mut SND_SOC_TEGRA_WM8903: snd_soc_card = snd_soc_card {
    components: cstr(b"codec:wm8903\0"),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut TEGRA_WM8903_DAI },
    num_links: 1,
    remove: Some(tegra_wm8903_remove),
    fully_routed: true,
    dev: ptr::null_mut(),
};

/* older device-trees used wrong polarity for the headphones-detection GPIO */
static TEGRA_WM8903_DATA_LEGACY: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_wm8903_mclk_rate),
    card: unsafe { &mut SND_SOC_TEGRA_WM8903 },
    hp_jack_gpio_active_low: true,
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
};

static TEGRA_WM8903_DATA: tegra_asoc_data = tegra_asoc_data {
    mclk_rate: Some(tegra_wm8903_mclk_rate),
    card: unsafe { &mut SND_SOC_TEGRA_WM8903 },
    hp_jack_gpio_active_low: false,
    add_common_dapm_widgets: true,
    add_common_controls: true,
    add_common_snd_ops: true,
    add_mic_jack: true,
    add_hp_jack: true,
};

static TEGRA_WM8903_OF_MATCH: [of_device_id; 10] = [
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"ad,tegra-audio-plutux\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"ad,tegra-audio-wm8903-medcom-wide\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"ad,tegra-audio-wm8903-tec\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903-cardhu\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903-harmony\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903-picasso\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903-seaboard\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903-ventana\0"),
        data: &TEGRA_WM8903_DATA_LEGACY as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: cstr(b"nvidia,tegra-audio-wm8903\0"),
        data: &TEGRA_WM8903_DATA as *const tegra_asoc_data as *const c_void,
    },
    of_device_id {
        name: ptr::null(),
        type_: ptr::null(),
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, tegra_wm8903_of_match); */

static mut TEGRA_WM8903_DRIVER: platform_driver = platform_driver {
    driver: driver_private {
        name: cstr(b"tegra-wm8903\0"),
        of_match_table: TEGRA_WM8903_OF_MATCH.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(tegra_asoc_machine_probe),
};

/*
 * module_platform_driver(tegra_wm8903_driver);
 * The C macro emits module init/exit functions that register and unregister the
 * platform driver.
 */
#[no_mangle]
pub unsafe extern "C" fn tegra_wm8903_driver_init() -> c_int {
    __platform_driver_register(&mut TEGRA_WM8903_DRIVER, THIS_MODULE)
}

#[no_mangle]
pub unsafe extern "C" fn tegra_wm8903_driver_exit() {
    platform_driver_unregister(&mut TEGRA_WM8903_DRIVER);
}

/* MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>"); */
/* MODULE_DESCRIPTION("Tegra+WM8903 machine ASoC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
