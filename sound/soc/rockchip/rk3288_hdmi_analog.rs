// SPDX-License-Identifier: GPL-2.0-only
/*
 * Rockchip machine ASoC driver for RK3288 boards that have an HDMI and analog
 * audio output
 *
 * Copyright (c) 2016, Collabora Ltd.
 *
 * Authors: Sjoerd Simons <sjoerd.simons@collabora.com>,
 *	    Romain Perier <romain.perier@collabora.com>
 */

// C dependencies:
// linux/module.h, linux/platform_device.h, linux/slab.h,
// linux/gpio/consumer.h, sound/core.h, sound/jack.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-dapm.h, "rockchip_i2s.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

const DRV_NAME: &[u8] = b"rk3288-snd-hdmi-analog\0";

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUPP: c_int = 524;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
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
pub struct snd_soc_card {
    pub name: *const c_char,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub num_aux_devs: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
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
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_uint,
    pub debounce_time: c_int,
    pub gpiod_dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
}

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
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
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
struct rk_drvdata {
    gpio_hp_en: *mut gpio_desc,
}

unsafe extern "C" {
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn of_parse_phandle_with_fixed_args(
        np: *mut device_node,
        list_name: *const c_char,
        cell_count: c_int,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn snd_soc_get_dai_name(args: *mut of_phandle_args, dai_name: *mut *const c_char) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

const fn snd_soc_dapm_event_on(event: c_int) -> c_int {
    (event != 0) as c_int
}

unsafe extern "C" fn rk_hp_power(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let machine: *mut rk_drvdata = snd_soc_card_get_drvdata(card) as *mut rk_drvdata;

    gpiod_set_value_cansleep((*machine).gpio_hp_en, snd_soc_dapm_event_on(event));

    0
}

static mut headphone_jack: snd_soc_jack = snd_soc_jack { _private: [] };

static mut headphone_jack_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: b"Analog\0".as_ptr() as *const c_char,
    mask: SND_JACK_HEADPHONE,
}];

// SND_SOC_DAPM_HP("Analog", rk_hp_power), SND_SOC_DAPM_LINE("HDMI", NULL)
static rk_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: null_mut() },
    snd_soc_dapm_widget { dapm: null_mut() },
];

// SOC_DAPM_PIN_SWITCH("Analog"), SOC_DAPM_PIN_SWITCH("HDMI")
static rk_mc_controls: [snd_kcontrol_new; 2] =
    [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];

unsafe extern "C" fn rk_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let mut ret: c_int;
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mclk: c_int;

    match params_rate(params) {
        8000 | 16000 | 24000 | 32000 | 48000 | 64000 | 96000 => {
            mclk = 12288000;
        }
        192000 => {
            mclk = 24576000;
        }
        11025 | 22050 | 44100 | 88200 => {
            mclk = 11289600;
        }
        _ => {
            return -EINVAL;
        }
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk as c_uint, SND_SOC_CLOCK_OUT);

    if ret != 0 && ret != -ENOTSUPP {
        dev_err((*codec_dai).dev, b"Can't set cpu clock %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk as c_uint, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(
            (*codec_dai).dev,
            b"Can't set codec clock %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

static mut rk_hp_jack_gpio: snd_soc_jack_gpio = snd_soc_jack_gpio {
    name: b"rockchip,hp-det\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADPHONE,
    debounce_time: 150,
    gpiod_dev: null_mut(),
};

unsafe extern "C" fn rk_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = (*runtime).card;
    let dev: *mut device = (*card).dev;

    /* Enable optional Headset Jack detection */
    if of_property_present(
        (*dev).of_node,
        b"rockchip,hp-det-gpios\0".as_ptr() as *const c_char,
    ) {
        rk_hp_jack_gpio.gpiod_dev = dev;
        snd_soc_card_jack_new_pins(
            (*runtime).card,
            b"Headphone Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADPHONE,
            addr_of_mut!(headphone_jack),
            headphone_jack_pins.as_mut_ptr(),
            headphone_jack_pins.len() as c_uint,
        );
        snd_soc_jack_add_gpios(addr_of_mut!(headphone_jack), 1, addr_of_mut!(rk_hp_jack_gpio));
    }

    0
}

static rk_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(rk_hw_params),
};

// SND_SOC_DAILINK_DEFS(audio,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, NULL),
//                        COMP_CODEC("hdmi-audio-codec.2.auto", "i2s-hifi")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut audio_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: null_mut(),
    dai_name: null(),
}];

static mut audio_codecs: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component {
        of_node: null_mut(),
        dai_name: null(),
    },
    snd_soc_dai_link_component {
        of_node: null_mut(),
        dai_name: b"i2s-hifi\0".as_ptr() as *const c_char,
    },
];

static mut audio_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    of_node: null_mut(),
    dai_name: null(),
}];

static mut rk_dailink: snd_soc_dai_link = snd_soc_dai_link {
    name: b"Codecs\0".as_ptr() as *const c_char,
    stream_name: b"Audio\0".as_ptr() as *const c_char,
    init: Some(rk_init),
    ops: &rk_ops,
    /* Set codecs as slave */
    dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
    cpus: unsafe { audio_cpus.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { audio_codecs.as_mut_ptr() },
    num_codecs: 2,
    platforms: unsafe { audio_platforms.as_mut_ptr() },
    num_platforms: 1,
};

static mut snd_soc_card_rk: snd_soc_card = snd_soc_card {
    name: b"ROCKCHIP-I2S\0".as_ptr() as *const c_char,
    dev: null_mut(),
    dai_link: unsafe { addr_of_mut!(rk_dailink) },
    num_links: 1,
    num_aux_devs: 0,
    dapm_widgets: rk_dapm_widgets.as_ptr(),
    num_dapm_widgets: rk_dapm_widgets.len() as c_int,
    controls: rk_mc_controls.as_ptr(),
    num_controls: rk_mc_controls.len() as c_int,
};

unsafe extern "C" fn snd_rk_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let card: *mut snd_soc_card = addr_of_mut!(snd_soc_card_rk);
    let np: *mut device_node = (*pdev).dev.of_node;
    let machine: *mut rk_drvdata;
    let mut args: of_phandle_args = core::mem::zeroed();

    machine = devm_kzalloc(
        addr_of_mut!((*pdev).dev),
        size_of::<rk_drvdata>(),
        GFP_KERNEL,
    ) as *mut rk_drvdata;
    if machine.is_null() {
        return -ENOMEM;
    }

    (*card).dev = addr_of_mut!((*pdev).dev);

    (*machine).gpio_hp_en = devm_gpiod_get_optional(
        addr_of_mut!((*pdev).dev),
        b"rockchip,hp-en\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*machine).gpio_hp_en as *const c_void) {
        return PTR_ERR((*machine).gpio_hp_en as *const c_void);
    }
    gpiod_set_consumer_name(
        (*machine).gpio_hp_en,
        b"hp_en\0".as_ptr() as *const c_char,
    );

    ret = snd_soc_of_parse_card_name(card, b"rockchip,model\0".as_ptr() as *const c_char);
    if ret != 0 {
        return ret;
    }

    (*rk_dailink.codecs).of_node = of_parse_phandle(
        np,
        b"rockchip,audio-codec\0".as_ptr() as *const c_char,
        0,
    );
    if (*rk_dailink.codecs).of_node.is_null() {
        dev_err(
            addr_of_mut!((*pdev).dev),
            b"Property 'rockchip,audio-codec' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    ret = of_parse_phandle_with_fixed_args(
        np,
        b"rockchip,audio-codec\0".as_ptr() as *const c_char,
        0,
        0,
        &mut args,
    );
    if ret != 0 {
        dev_err(
            addr_of_mut!((*pdev).dev),
            b"Unable to parse property 'rockchip,audio-codec'\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret = snd_soc_get_dai_name(&mut args, addr_of_mut!((*rk_dailink.codecs).dai_name));
    if ret != 0 {
        return dev_err_probe(
            addr_of_mut!((*pdev).dev),
            ret,
            b"Unable to get codec_dai_name\n\0".as_ptr() as *const c_char,
        );
    }

    (*rk_dailink.cpus).of_node = of_parse_phandle(
        np,
        b"rockchip,i2s-controller\0".as_ptr() as *const c_char,
        0,
    );
    if (*rk_dailink.cpus).of_node.is_null() {
        dev_err(
            addr_of_mut!((*pdev).dev),
            b"Property 'rockchip,i2s-controller' missing or invalid\n\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }

    (*rk_dailink.platforms).of_node = (*rk_dailink.cpus).of_node;

    ret = snd_soc_of_parse_audio_routing(card, b"rockchip,routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(card, machine as *mut c_void);

    ret = devm_snd_soc_register_card(addr_of_mut!((*pdev).dev), card);
    if ret != 0 {
        return dev_err_probe(
            addr_of_mut!((*pdev).dev),
            ret,
            b"Soc register card failed\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

static rockchip_sound_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"rockchip,rk3288-hdmi-analog\0".as_ptr() as *const c_char,
    },
    of_device_id { compatible: null() },
];

// MODULE_DEVICE_TABLE(of, rockchip_sound_of_match);

static mut rockchip_sound_driver: platform_driver = platform_driver {
    probe: Some(snd_rk_mc_probe),
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: rockchip_sound_of_match.as_ptr(),
    },
};

// module_platform_driver(rockchip_sound_driver);
// MODULE_AUTHOR("Sjoerd Simons <sjoerd.simons@collabora.com>");
// MODULE_DESCRIPTION("Rockchip RK3288 machine ASoC driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
