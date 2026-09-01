// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 Freescale Semiconductor, Inc.
 */

// C dependencies:
// linux/module.h, linux/device.h, linux/of.h
// sound/core.h, sound/pcm.h, sound/soc.h, sound/jack.h, sound/soc-dapm.h
// ../codecs/sgtl5000.h, mxs-saif.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u32 = c_uint;

const EINVAL: c_int = 22;

extern "C" {
    static THIS_MODULE: *mut module;

    static SGTL5000_SYSCLK: c_int;
    static MXS_SAIF_MCLK: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn mxs_saif_get_mclk(saif_id: c_int, mclk: c_uint, rate: c_uint) -> c_int;
    fn mxs_saif_put_mclk(saif_id: c_int);
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct module {
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
    pub dev: *mut device,
}

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
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
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
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub playback_only: bool,
    pub capture_only: bool,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dev: *mut device,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
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

unsafe extern "C" fn mxs_sgtl5000_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let rate: c_uint = params_rate(params);
    let mclk: u32;
    let mut ret: c_int;

    /* sgtl5000 does not support 512*rate when in 96000 fs */
    match rate {
        96000 => {
            mclk = 256u32.wrapping_mul(rate);
        }
        _ => {
            mclk = 512u32.wrapping_mul(rate);
        }
    }

    /* Set SGTL5000's SYSCLK (provided by SAIF MCLK) */
    ret = snd_soc_dai_set_sysclk(codec_dai, SGTL5000_SYSCLK, mclk, 0);
    if ret != 0 {
        dev_err(
            (*codec_dai).dev,
            b"Failed to set sysclk to %u.%03uMHz\n\0".as_ptr() as *const c_char,
            mclk / 1000000,
            mclk / 1000 % 1000,
        );
        return ret;
    }

    /* The SAIF MCLK should be the same as SGTL5000_SYSCLK */
    ret = snd_soc_dai_set_sysclk(cpu_dai, MXS_SAIF_MCLK, mclk, 0);
    if ret != 0 {
        dev_err(
            (*cpu_dai).dev,
            b"Failed to set sysclk to %u.%03uMHz\n\0".as_ptr() as *const c_char,
            mclk / 1000000,
            mclk / 1000 % 1000,
        );
        return ret;
    }

    0
}

static mxs_sgtl5000_hifi_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(mxs_sgtl5000_hw_params),
};

fn MXS_SGTL5000_DAI_FMT() -> c_uint {
    unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC }
}

// SND_SOC_DAILINK_DEFS(hifi_tx,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "sgtl5000")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut hifi_tx_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];
static mut hifi_tx_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: b"sgtl5000\0".as_ptr() as *const c_char,
}];
static mut hifi_tx_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

// SND_SOC_DAILINK_DEFS(hifi_rx,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "sgtl5000")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut hifi_rx_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];
static mut hifi_rx_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: b"sgtl5000\0".as_ptr() as *const c_char,
}];
static mut hifi_rx_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    of_node: ptr::null_mut(),
    dai_name: ptr::null(),
}];

static mut mxs_sgtl5000_dai: [snd_soc_dai_link; 2] = unsafe {
    [
        snd_soc_dai_link {
            name: b"HiFi Tx\0".as_ptr() as *const c_char,
            stream_name: b"HiFi Playback\0".as_ptr() as *const c_char,
            dai_fmt: 0,
            ops: &mxs_sgtl5000_hifi_ops,
            playback_only: true,
            capture_only: false,
            cpus: hifi_tx_cpus.as_mut_ptr(),
            num_cpus: 1,
            codecs: hifi_tx_codecs.as_mut_ptr(),
            num_codecs: 1,
            platforms: hifi_tx_platforms.as_mut_ptr(),
            num_platforms: 1,
        },
        snd_soc_dai_link {
            name: b"HiFi Rx\0".as_ptr() as *const c_char,
            stream_name: b"HiFi Capture\0".as_ptr() as *const c_char,
            dai_fmt: 0,
            ops: &mxs_sgtl5000_hifi_ops,
            playback_only: false,
            capture_only: true,
            cpus: hifi_rx_cpus.as_mut_ptr(),
            num_cpus: 1,
            codecs: hifi_rx_codecs.as_mut_ptr(),
            num_codecs: 1,
            platforms: hifi_rx_platforms.as_mut_ptr(),
            num_platforms: 1,
        },
    ]
};

static mut mxs_sgtl5000_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    // SND_SOC_DAPM_MIC("Mic Jack", NULL)
    snd_soc_dapm_widget {
        name: b"Mic Jack\0".as_ptr() as *const c_char,
    },
    // SND_SOC_DAPM_LINE("Line In Jack", NULL)
    snd_soc_dapm_widget {
        name: b"Line In Jack\0".as_ptr() as *const c_char,
    },
    // SND_SOC_DAPM_HP("Headphone Jack", NULL)
    snd_soc_dapm_widget {
        name: b"Headphone Jack\0".as_ptr() as *const c_char,
    },
    // SND_SOC_DAPM_SPK("Line Out Jack", NULL)
    snd_soc_dapm_widget {
        name: b"Line Out Jack\0".as_ptr() as *const c_char,
    },
    // SND_SOC_DAPM_SPK("Ext Spk", NULL)
    snd_soc_dapm_widget {
        name: b"Ext Spk\0".as_ptr() as *const c_char,
    },
];

static mut mxs_sgtl5000: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"mxs_sgtl5000\0".as_ptr() as *const c_char,
        owner: THIS_MODULE,
        dai_link: mxs_sgtl5000_dai.as_mut_ptr(),
        num_links: mxs_sgtl5000_dai.len() as c_int,
        dev: ptr::null_mut(),
        dapm_widgets: ptr::null(),
        num_dapm_widgets: 0,
    }
};

unsafe extern "C" fn mxs_sgtl5000_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut mxs_sgtl5000;
    let mut ret: c_int;
    let mut i: c_int;
    let np: *mut device_node = (*pdev).dev.of_node;
    let mut saif_np: [*mut device_node; 2] = [ptr::null_mut(); 2];
    let codec_np: *mut device_node;

    saif_np[0] = of_parse_phandle(np, b"saif-controllers\0".as_ptr() as *const c_char, 0);
    saif_np[1] = of_parse_phandle(np, b"saif-controllers\0".as_ptr() as *const c_char, 1);
    codec_np = of_parse_phandle(np, b"audio-codec\0".as_ptr() as *const c_char, 0);
    if saif_np[0].is_null() || saif_np[1].is_null() || codec_np.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"phandle missing or invalid\n\0".as_ptr() as *const c_char,
        );
        of_node_put(codec_np);
        of_node_put(saif_np[0]);
        of_node_put(saif_np[1]);
        return -EINVAL;
    }

    i = 0;
    while i < 2 {
        (*mxs_sgtl5000_dai[i as usize].codecs).name = ptr::null();
        (*mxs_sgtl5000_dai[i as usize].codecs).of_node = codec_np;
        (*mxs_sgtl5000_dai[i as usize].cpus).dai_name = ptr::null();
        (*mxs_sgtl5000_dai[i as usize].cpus).of_node = saif_np[i as usize];
        (*mxs_sgtl5000_dai[i as usize].platforms).name = ptr::null();
        (*mxs_sgtl5000_dai[i as usize].platforms).of_node = saif_np[i as usize];
        i += 1;
    }

    of_node_put(codec_np);
    of_node_put(saif_np[0]);
    of_node_put(saif_np[1]);

    /*
     * Set an init clock(11.28Mhz) for sgtl5000 initialization(i2c r/w).
     * The Sgtl5000 sysclk is derived from saif0 mclk and it's range
     * should be >= 8MHz and <= 27M.
     */
    ret = mxs_saif_get_mclk(0, 44100 * 256, 44100);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to get mclk\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    (*card).dev = &mut (*pdev).dev;

    if of_property_present(np, b"audio-routing\0".as_ptr() as *const c_char) {
        (*card).dapm_widgets = mxs_sgtl5000_dapm_widgets.as_ptr();
        (*card).num_dapm_widgets = mxs_sgtl5000_dapm_widgets.len() as c_int;

        ret = snd_soc_of_parse_audio_routing(card, b"audio-routing\0".as_ptr() as *const c_char);
        if ret != 0 {
            mxs_saif_put_mclk(0);
            return ret;
        }
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        mxs_saif_put_mclk(0);
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card failed\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

unsafe extern "C" fn mxs_sgtl5000_remove(_pdev: *mut platform_device) {
    mxs_saif_put_mclk(0);
}

static mxs_sgtl5000_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fsl,mxs-audio-sgtl5000\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, mxs_sgtl5000_dt_ids);

static mxs_sgtl5000_audio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mxs-sgtl5000\0".as_ptr() as *const c_char,
        of_match_table: mxs_sgtl5000_dt_ids.as_ptr(),
    },
    probe: Some(mxs_sgtl5000_probe),
    remove: Some(mxs_sgtl5000_remove),
};

// module_platform_driver(mxs_sgtl5000_audio_driver);

// MODULE_AUTHOR("Freescale Semiconductor, Inc.");
// MODULE_DESCRIPTION("MXS ALSA SoC Machine driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:mxs-sgtl5000");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
