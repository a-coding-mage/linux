// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2017 Samsung Electronics Co., Ltd.

// Rust translation of soc/samsung/odroid.c.
// Kernel headers used by the C source:
// <linux/clk.h>, <linux/clk-provider.h>, <linux/of.h>, <linux/module.h>,
// <sound/soc.h>, <sound/pcm_params.h>, "i2s.h", "i2s-regs.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type bool_ = bool;

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub fully_routed: bool_,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub ops: *const snd_soc_ops,
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct odroid_priv {
    card: snd_soc_card,
    clk_i2s_bus: *mut clk,
    sclk_i2s: *mut clk,

    /* Spinlock protecting fields below */
    lock: spinlock_t,
    be_sample_rate: c_uint,
    be_active: bool_,
}

const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 3;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 5;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool_;
    fn snd_soc_of_parse_audio_simple_widgets(
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_get_child_by_name(np: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_count_phandle_with_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_name: *const c_char,
    ) -> c_int;
    fn snd_soc_of_get_dai_name(
        np: *mut device_node,
        dai_name: *mut *const c_char,
        index: c_int,
    ) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn snd_soc_of_get_dai_link_codecs(
        dev: *mut device,
        codec: *mut device_node,
        link: *mut snd_soc_dai_link,
    ) -> c_int;
    fn snd_soc_of_put_dai_link_codecs(link: *mut snd_soc_dai_link);
    fn of_clk_get_by_name(np: *mut device_node, name: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn clk_put(clk: *mut clk);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

type c_ulong = core::ffi::c_ulong;

unsafe extern "C" fn odroid_card_fe_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;

    snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_CHANNELS, 2);

    0
}

unsafe extern "C" fn odroid_card_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let priv_: *mut odroid_priv = snd_soc_card_get_drvdata((*rtd).card) as *mut odroid_priv;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
    if (*priv_).be_active && (*priv_).be_sample_rate != params_rate(params) {
        spin_unlock_irqrestore(&mut (*priv_).lock, flags);
        return -EINVAL;
    }
    spin_unlock_irqrestore(&mut (*priv_).lock, flags);

    0
}

static odroid_card_fe_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(odroid_card_fe_startup),
    hw_params: Some(odroid_card_fe_hw_params),
    trigger: None,
};

unsafe extern "C" fn odroid_card_be_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let priv_: *mut odroid_priv = snd_soc_card_get_drvdata((*rtd).card) as *mut odroid_priv;
    let pll_freq: c_uint;
    let rclk_freq: c_uint;
    let rfs: c_uint;
    let mut ret: c_int;

    match params_rate(params) {
        64000 => {
            pll_freq = 196608001_u32;
            rfs = 384;
        }
        44100 | 88200 => {
            pll_freq = 180633609_u32;
            rfs = 512;
        }
        32000 | 48000 | 96000 => {
            pll_freq = 196608001_u32;
            rfs = 512;
        }
        _ => return -EINVAL,
    }

    ret = clk_set_rate((*priv_).clk_i2s_bus, pll_freq / 2 + 1);
    if ret < 0 {
        return ret;
    }

    /*
     *  We add 2 to the rclk_freq value in order to avoid too low clock
     *  frequency values due to the EPLL output frequency not being exact
     *  multiple of the audio sampling rate.
     */
    rclk_freq = params_rate(params).wrapping_mul(rfs).wrapping_add(2);

    ret = clk_set_rate((*priv_).sclk_i2s, rclk_freq);
    if ret < 0 {
        return ret;
    }

    if (*(*rtd).dai_link).num_codecs > 1 {
        let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 1);

        ret = snd_soc_dai_set_sysclk(codec_dai, 0, rclk_freq, SND_SOC_CLOCK_IN);
        if ret < 0 {
            return ret;
        }
    }

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*priv_).lock, &mut flags);
        (*priv_).be_sample_rate = params_rate(params);
        spin_unlock_irqrestore(&mut (*priv_).lock, flags);
    }
    0
}

unsafe extern "C" fn odroid_card_be_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let priv_: *mut odroid_priv = snd_soc_card_get_drvdata((*rtd).card) as *mut odroid_priv;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*priv_).lock, &mut flags);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*priv_).be_active = true;
        }

        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*priv_).be_active = false;
        }
        _ => {}
    }

    spin_unlock_irqrestore(&mut (*priv_).lock, flags);

    0
}

static odroid_card_be_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(odroid_card_be_hw_params),
    trigger: Some(odroid_card_be_trigger),
};

/* DAPM routes for backward compatibility with old DTS */
static odroid_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"I2S Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mixer DAI TX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"HiFi Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mixer DAI TX\0".as_ptr() as *const c_char,
    },
];

// SND_SOC_DAILINK_DEFS(primary,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_DUMMY()),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("3830000.i2s")));
static mut primary_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];
static mut primary_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];
static mut primary_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    dai_name: b"3830000.i2s\0".as_ptr() as *const c_char,
}];

// SND_SOC_DAILINK_DEFS(mixer,
//     DAILINK_COMP_ARRAY(COMP_DUMMY()),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut mixer_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];
static mut mixer_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];

// SND_SOC_DAILINK_DEFS(secondary,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_DUMMY()),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("3830000.i2s-sec")));
static mut secondary_cpus: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];
static mut secondary_codecs: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { dai_name: ptr::null() }];
static mut secondary_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    dai_name: b"3830000.i2s-sec\0".as_ptr() as *const c_char,
}];

static mut odroid_card_dais: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        /* Primary FE <-> BE link */
        ops: &odroid_card_fe_ops,
        name: b"Primary\0".as_ptr() as *const c_char,
        stream_name: b"Primary\0".as_ptr() as *const c_char,
        dynamic: 1,
        playback_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        cpus: unsafe { primary_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { primary_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { primary_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
    snd_soc_dai_link {
        /* BE <-> CODECs link */
        name: b"I2S Mixer\0".as_ptr() as *const c_char,
        ops: &odroid_card_be_ops,
        stream_name: ptr::null(),
        dynamic: 0,
        no_pcm: 1,
        playback_only: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        cpus: unsafe { mixer_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { mixer_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: ptr::null_mut(),
        num_platforms: 0,
    },
    snd_soc_dai_link {
        /* Secondary FE <-> BE link */
        ops: &odroid_card_fe_ops,
        name: b"Secondary\0".as_ptr() as *const c_char,
        stream_name: b"Secondary\0".as_ptr() as *const c_char,
        dynamic: 1,
        playback_only: 1,
        no_pcm: 0,
        dai_fmt: 0,
        cpus: unsafe { secondary_cpus.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { secondary_codecs.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { secondary_platforms.as_mut_ptr() },
        num_platforms: 1,
    },
];

unsafe extern "C" fn odroid_audio_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut cpu_dai: *mut device_node = ptr::null_mut();
    let cpu: *mut device_node;
    let codec: *mut device_node;
    let priv_: *mut odroid_priv;
    let card: *mut snd_soc_card;
    let mut link: *mut snd_soc_dai_link;
    let codec_link: *mut snd_soc_dai_link;
    let num_pcms: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<odroid_priv>(), GFP_KERNEL) as *mut odroid_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = &mut (*priv_).card;
    (*card).dev = dev;

    (*card).owner = THIS_MODULE;
    (*card).fully_routed = true;

    spin_lock_init(&mut (*priv_).lock);
    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    ret = snd_soc_of_parse_card_name(card, b"model\0".as_ptr() as *const c_char);
    if ret < 0 {
        return ret;
    }

    if of_property_present(
        (*dev).of_node,
        b"samsung,audio-widgets\0".as_ptr() as *const c_char,
    ) {
        ret = snd_soc_of_parse_audio_simple_widgets(
            card,
            b"samsung,audio-widgets\0".as_ptr() as *const c_char,
        );
        if ret < 0 {
            return ret;
        }
    }

    ret = 0;
    if of_property_present((*dev).of_node, b"audio-routing\0".as_ptr() as *const c_char) {
        ret = snd_soc_of_parse_audio_routing(
            card,
            b"audio-routing\0".as_ptr() as *const c_char,
        );
    } else if of_property_present(
        (*dev).of_node,
        b"samsung,audio-routing\0".as_ptr() as *const c_char,
    ) {
        ret = snd_soc_of_parse_audio_routing(
            card,
            b"samsung,audio-routing\0".as_ptr() as *const c_char,
        );
    }
    if ret < 0 {
        return ret;
    }

    (*card).dai_link = odroid_card_dais.as_mut_ptr();
    (*card).num_links = odroid_card_dais.len() as c_int;

    cpu = of_get_child_by_name((*dev).of_node, b"cpu\0".as_ptr() as *const c_char);
    codec = of_get_child_by_name((*dev).of_node, b"codec\0".as_ptr() as *const c_char);
    link = (*card).dai_link;
    codec_link = (*card).dai_link.add(1);

    /*
     * For backwards compatibility create the secondary CPU DAI link only
     * if there are 2 CPU DAI entries in the cpu sound-dai property in DT.
     * Also add required DAPM routes not available in old DTS.
     */
    num_pcms = of_count_phandle_with_args(
        cpu,
        b"sound-dai\0".as_ptr() as *const c_char,
        b"#sound-dai-cells\0".as_ptr() as *const c_char,
    );
    if num_pcms == 1 {
        (*card).dapm_routes = odroid_dapm_routes.as_ptr();
        (*card).num_dapm_routes = odroid_dapm_routes.len() as c_int;
        (*card).num_links -= 1;
    }

    i = 0;
    while i < num_pcms {
        ret = snd_soc_of_get_dai_name(cpu, &mut (*(*link).cpus).dai_name, i);
        if ret < 0 {
            break;
        }
        i += 1;
        link = link.add(2);
    }
    if ret == 0 {
        cpu_dai = of_parse_phandle(cpu, b"sound-dai\0".as_ptr() as *const c_char, 0);
        if cpu_dai.is_null() {
            ret = -EINVAL;
        }
    }

    of_node_put(cpu);
    if ret < 0 {
        goto_err_put_node(codec, ret);
        return ret;
    }

    ret = snd_soc_of_get_dai_link_codecs(dev, codec, codec_link);
    if ret < 0 {
        of_node_put(cpu_dai);
        snd_soc_of_put_dai_link_codecs(codec_link);
        of_node_put(codec);
        return ret;
    }

    /* Set capture capability only for boards with the MAX98090 CODEC */
    if (*codec_link).num_codecs > 1 {
        (*(*card).dai_link.add(0)).playback_only = 0;
        (*(*card).dai_link.add(1)).playback_only = 0;
    }

    (*priv_).sclk_i2s = of_clk_get_by_name(cpu_dai, b"i2s_opclk1\0".as_ptr() as *const c_char);
    if IS_ERR((*priv_).sclk_i2s as *const c_void) {
        ret = PTR_ERR((*priv_).sclk_i2s as *const c_void);
        of_node_put(cpu_dai);
        snd_soc_of_put_dai_link_codecs(codec_link);
        of_node_put(codec);
        return ret;
    }

    (*priv_).clk_i2s_bus = of_clk_get_by_name(cpu_dai, b"iis\0".as_ptr() as *const c_char);
    if IS_ERR((*priv_).clk_i2s_bus as *const c_void) {
        ret = PTR_ERR((*priv_).clk_i2s_bus as *const c_void);
        clk_put((*priv_).sclk_i2s);
        of_node_put(cpu_dai);
        snd_soc_of_put_dai_link_codecs(codec_link);
        of_node_put(codec);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        dev_err_probe(
            dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const c_char,
        );
        clk_put((*priv_).clk_i2s_bus);
        clk_put((*priv_).sclk_i2s);
        of_node_put(cpu_dai);
        snd_soc_of_put_dai_link_codecs(codec_link);
        of_node_put(codec);
        return ret;
    }

    of_node_put(cpu_dai);
    of_node_put(codec);
    0
}

unsafe fn goto_err_put_node(codec: *mut device_node, _ret: c_int) {
    of_node_put(codec);
}

unsafe extern "C" fn odroid_audio_remove(pdev: *mut platform_device) {
    let priv_: *mut odroid_priv = platform_get_drvdata(pdev) as *mut odroid_priv;

    snd_soc_of_put_dai_link_codecs(&mut *(*priv_).card.dai_link.add(1));
    clk_put((*priv_).sclk_i2s);
    clk_put((*priv_).clk_i2s_bus);
}

static odroid_audio_of_match: [of_device_id; 5] = [
    of_device_id {
        compatible: b"hardkernel,odroid-xu3-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"hardkernel,odroid-xu4-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"samsung,odroid-xu3-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"samsung,odroid-xu4-audio\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, odroid_audio_of_match);

static mut odroid_audio_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"odroid-audio\0".as_ptr() as *const c_char,
        of_match_table: odroid_audio_of_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(odroid_audio_probe),
    remove: Some(odroid_audio_remove),
};
// module_platform_driver(odroid_audio_driver);

// MODULE_AUTHOR("Sylwester Nawrocki <s.nawrocki@samsung.com>");
// MODULE_DESCRIPTION("Odroid XU3/XU4 audio support");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
