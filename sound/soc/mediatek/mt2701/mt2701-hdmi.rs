// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-hdmi.c -- MT2701 HDMI ALSA SoC machine driver
 *
 * Copyright (c) 2026 Daniel Golle <daniel@makrotopia.org>
 *
 * Based on mt2701-cs42448.c
 */

/*
 * C dependencies:
 * #include <linux/module.h>
 * #include <linux/of.h>
 * #include <linux/platform_device.h>
 * #include <sound/soc.h>
 */

use core::ffi::{c_char, c_int, c_uint};
use core::ptr::{addr_of_mut, null, null_mut};

const DAI_LINK_FE_HDMI_OUT: usize = 0;
const DAI_LINK_BE_HDMI_I2S: usize = 1;

const EINVAL: c_int = 22;

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn of_parse_phandle(
        np: *const device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

#[repr(C)]
pub struct module {
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
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub trigger: [c_uint; 2],
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub no_pcm: c_uint,
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
    pub dev: *mut device,
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
}

const SND_SOC_DPCM_TRIGGER_POST: c_uint = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

static mut FE_HDMI_OUT_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"PCM_HDMI\0".as_ptr() as *const c_char,
    dai_name: null(),
    of_node: null_mut(),
}];

static mut FE_HDMI_OUT_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: null(),
    dai_name: null(),
    of_node: null_mut(),
}];

static mut FE_HDMI_OUT_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: null(),
    dai_name: null(),
    of_node: null_mut(),
}];

static mut BE_HDMI_I2S_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"HDMI I2S\0".as_ptr() as *const c_char,
    dai_name: null(),
    of_node: null_mut(),
}];

static mut BE_HDMI_I2S_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: null(),
    dai_name: b"i2s-hifi\0".as_ptr() as *const c_char,
    of_node: null_mut(),
}];

static mut BE_HDMI_I2S_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: null(),
    dai_name: null(),
    of_node: null_mut(),
}];

static mut mt2701_hdmi_dai_links: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"HDMI Playback\0".as_ptr() as *const c_char,
        stream_name: b"HDMI Playback\0".as_ptr() as *const c_char,
        trigger: [
            SND_SOC_DPCM_TRIGGER_POST,
            SND_SOC_DPCM_TRIGGER_POST,
        ],
        dynamic: 1,
        playback_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        cpus: unsafe { FE_HDMI_OUT_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { FE_HDMI_OUT_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { FE_HDMI_OUT_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: b"HDMI BE\0".as_ptr() as *const c_char,
        stream_name: null(),
        trigger: [0, 0],
        dynamic: 0,
        playback_only: 1,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { BE_HDMI_I2S_CPUS.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { BE_HDMI_I2S_CODECS.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { BE_HDMI_I2S_PLATFORMS.as_mut_ptr() },
        num_platforms: 1,
    },
];

static mut mt2701_hdmi_soc_card: snd_soc_card = snd_soc_card {
    name: b"mt2701-hdmi\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { mt2701_hdmi_dai_links.as_mut_ptr() },
    num_links: unsafe { mt2701_hdmi_dai_links.len() as c_int },
    dev: null_mut(),
};

unsafe extern "C" fn mt2701_hdmi_machine_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = addr_of_mut!(mt2701_hdmi_soc_card);
    let dev: *mut device = addr_of_mut!((*pdev).dev);
    let platform_node: *mut device_node;
    let codec_node: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let ret: c_int;
    let mut i: usize;

    platform_node = of_parse_phandle(
        (*dev).of_node,
        b"mediatek,platform\0".as_ptr() as *const c_char,
        0,
    );
    if platform_node.is_null() {
        return dev_err_probe(
            dev,
            -EINVAL,
            b"Property 'mediatek,platform' missing\n\0".as_ptr() as *const c_char,
        );
    }

    i = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        if !(*(*dai_link).platforms).name.is_null() {
            i += 1;
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
        i += 1;
    }

    codec_node = of_parse_phandle(
        (*dev).of_node,
        b"mediatek,audio-codec\0".as_ptr() as *const c_char,
        0,
    );
    if codec_node.is_null() {
        of_node_put(platform_node);
        return dev_err_probe(
            dev,
            -EINVAL,
            b"Property 'mediatek,audio-codec' missing\n\0".as_ptr() as *const c_char,
        );
    }
    (*mt2701_hdmi_dai_links[DAI_LINK_BE_HDMI_I2S].codecs).of_node = codec_node;

    (*card).dev = dev;

    ret = devm_snd_soc_register_card(dev, card);

    of_node_put(platform_node);
    of_node_put(codec_node);
    ret
}

static mt2701_hdmi_machine_dt_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"mediatek,mt2701-hdmi-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"mediatek,mt7623n-hdmi-audio\0".as_ptr() as *const c_char,
    },
    of_device_id { compatible: null() },
];

/* MODULE_DEVICE_TABLE(of, mt2701_hdmi_machine_dt_match); */

static mut mt2701_hdmi_machine: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mt2701-hdmi\0".as_ptr() as *const c_char,
        of_match_table: mt2701_hdmi_machine_dt_match.as_ptr(),
    },
    probe: Some(mt2701_hdmi_machine_probe),
};

/* module_platform_driver(mt2701_hdmi_machine); */

/* MODULE_DESCRIPTION("MT2701 HDMI ALSA SoC machine driver"); */
/* MODULE_AUTHOR("Daniel Golle <daniel@makrotopia.org>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:mt2701-hdmi"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
