// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2014 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

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
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub kind: c_uint,
    pub event: *mut c_void,
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
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
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

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 2;

const SND_SOC_DAPM_HP: c_uint = 0;
const SND_SOC_DAPM_LINE: c_uint = 1;

extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        codec_dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

unsafe extern "C" fn a370db_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let freq: c_uint;

    match params_rate(params) {
        48000 => {
            freq = 12288000;
        }
        96000 => {
            freq = 24576000;
        }
        _ => {
            freq = 11289600;
        }
    }

    snd_soc_dai_set_sysclk(codec_dai, 0, freq, SND_SOC_CLOCK_IN)
}

static A370DB_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(a370db_hw_params),
};

static A370DB_DAPM_WIDGETS: [snd_soc_dapm_widget; 2] = [
    /* SND_SOC_DAPM_HP("Out Jack", NULL) */
    snd_soc_dapm_widget {
        name: b"Out Jack\0".as_ptr() as *const c_char,
        kind: SND_SOC_DAPM_HP,
        event: ptr::null_mut(),
    },
    /* SND_SOC_DAPM_LINE("In Jack", NULL) */
    snd_soc_dapm_widget {
        name: b"In Jack\0".as_ptr() as *const c_char,
        kind: SND_SOC_DAPM_LINE,
        event: ptr::null_mut(),
    },
];

static A370DB_ROUTE: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"Out Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Out Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AIN1L\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"In Jack\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AIN1L\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"In Jack\0".as_ptr() as *const c_char,
    },
];

/* SND_SOC_DAILINK_DEFS(analog, ...) */
static mut ANALOG_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2s\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut ANALOG_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"cs42l51-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut ANALOG_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

/* SND_SOC_DAILINK_DEFS(spdif_out, ...) */
static mut SPDIF_OUT_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"spdif\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut SPDIF_OUT_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"dit-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut SPDIF_OUT_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

/* SND_SOC_DAILINK_DEFS(spdif_in, ...) */
static mut SPDIF_IN_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"spdif\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut SPDIF_IN_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: b"dir-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut SPDIF_IN_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut A370DB_DAI: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: b"CS42L51\0".as_ptr() as *const c_char,
        stream_name: b"analog\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC,
        ops: &A370DB_OPS,
        cpus: unsafe { ANALOG_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ANALOG_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { ANALOG_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"S/PDIF out\0".as_ptr() as *const c_char,
        stream_name: b"spdif-out\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC,
        ops: ptr::null(),
        cpus: unsafe { SPDIF_OUT_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { SPDIF_OUT_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { SPDIF_OUT_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"S/PDIF in\0".as_ptr() as *const c_char,
        stream_name: b"spdif-in\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_CBC_CFC,
        ops: ptr::null(),
        cpus: unsafe { SPDIF_IN_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { SPDIF_IN_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { SPDIF_IN_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
];

static mut A370DB: snd_soc_card = snd_soc_card {
    name: b"a370db\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dev: ptr::null_mut(),
    dai_link: unsafe { A370DB_DAI.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: A370DB_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: 2,
    dapm_routes: A370DB_ROUTE.as_ptr(),
    num_dapm_routes: 4,
};

unsafe extern "C" fn a370db_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut A370DB;

    (*card).dev = &mut (*pdev).dev;

    (*A370DB_DAI[0].cpus).of_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"marvell,audio-controller\0".as_ptr() as *const c_char,
        0,
    );
    (*A370DB_DAI[0].platforms).of_node = (*A370DB_DAI[0].cpus).of_node;

    (*A370DB_DAI[0].codecs).of_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"marvell,audio-codec\0".as_ptr() as *const c_char,
        0,
    );

    (*A370DB_DAI[1].cpus).of_node = (*A370DB_DAI[0].cpus).of_node;
    (*A370DB_DAI[1].platforms).of_node = (*A370DB_DAI[0].cpus).of_node;

    (*A370DB_DAI[1].codecs).of_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"marvell,audio-codec\0".as_ptr() as *const c_char,
        1,
    );

    (*A370DB_DAI[2].cpus).of_node = (*A370DB_DAI[0].cpus).of_node;
    (*A370DB_DAI[2].platforms).of_node = (*A370DB_DAI[0].cpus).of_node;

    (*A370DB_DAI[2].codecs).of_node = of_parse_phandle(
        (*pdev).dev.of_node,
        b"marvell,audio-codec\0".as_ptr() as *const c_char,
        2,
    );

    devm_snd_soc_register_card((*card).dev, card)
}

static A370DB_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"marvell,a370db-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, a370db_dt_ids); */

static mut A370DB_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"a370db-audio\0".as_ptr() as *const c_char,
        of_match_table: A370DB_DT_IDS.as_ptr(),
    },
    probe: Some(a370db_probe),
};

/* module_platform_driver(a370db_driver); */
#[no_mangle]
pub unsafe extern "C" fn init_module() -> c_int {
    platform_driver_register(&mut A370DB_DRIVER)
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_module() {
    platform_driver_unregister(&mut A370DB_DRIVER);
}

/* MODULE_AUTHOR("Thomas Petazzoni <thomas.petazzoni@free-electrons.com>"); */
/* MODULE_DESCRIPTION("ALSA SoC a370db audio client"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:a370db-audio"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
