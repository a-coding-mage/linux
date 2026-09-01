// SPDX-License-Identifier: GPL-2.0+
//
// Littlemill audio support
//
// Copyright 2011 Wolfson Microelectronics

// C dependencies:
// sound/soc.h
// sound/soc-dapm.h
// sound/jack.h
// linux/module.h
// ../codecs/wm8994.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub set_bias_level: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub set_bias_level_post: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
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
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub formats: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub ignore_suspend: c_int,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_int,
    // SND_SOC_DAILINK_REG(cpu/baseband) expands to dependency-provided link fields.
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    // Constructed in C by SOC_DAPM_PIN_SWITCH().
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type snd_soc_bias_level = c_uint;

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
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
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn wm8958_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        det: *mut c_void,
        shrt: *mut c_void,
        btn: *mut c_void,
        pdata: *mut c_void,
    );
    fn wm8994_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        micbias: c_int,
    );
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;
const SND_SOC_CLOCK_IN: c_int = 0;
const WM8994_FLL1: c_int = 1;
const WM8994_FLL2: c_int = 2;
const WM8994_FLL_SRC_MCLK2: c_int = 2;
const WM8994_FLL_SRC_BCLK: c_int = 3;
const WM8994_SYSCLK_FLL1: c_int = 1;
const WM8994_SYSCLK_FLL2: c_int = 2;
const WM8994_SYSCLK_MCLK2: c_int = 3;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 10;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_MECHANICAL: c_int = 0x0004;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SND_JACK_BTN_4: c_int = 0x0400;
const SND_JACK_BTN_5: c_int = 0x0200;
const EINVAL: c_int = 22;

static mut sample_rate: c_int = 44100;

unsafe extern "C" fn littlemill_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let aif1_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));
    aif1_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*aif1_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_PREPARE => {
            /*
             * If we've not already clocked things via hw_params()
             * then do so now, otherwise these are noops.
             */
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
                ret = snd_soc_dai_set_pll(
                    aif1_dai,
                    WM8994_FLL1,
                    WM8994_FLL_SRC_MCLK2,
                    32768,
                    (sample_rate * 512) as c_uint,
                );
                if ret < 0 {
                    pr_err(b"Failed to start FLL: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }

                ret = snd_soc_dai_set_sysclk(
                    aif1_dai,
                    WM8994_SYSCLK_FLL1,
                    (sample_rate * 512) as c_uint,
                    SND_SOC_CLOCK_IN,
                );
                if ret < 0 {
                    pr_err(b"Failed to set SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn littlemill_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let aif1_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));
    aif1_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*aif1_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_STANDBY => {
            ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_MCLK2, 32768, SND_SOC_CLOCK_IN);
            if ret < 0 {
                pr_err(
                    b"Failed to switch away from FLL1: %d\n\0".as_ptr() as *const c_char,
                    ret,
                );
                return ret;
            }

            ret = snd_soc_dai_set_pll(aif1_dai, WM8994_FLL1, 0, 0, 0);
            if ret < 0 {
                pr_err(b"Failed to stop FLL1: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn littlemill_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    sample_rate = params_rate(params);

    ret = snd_soc_dai_set_pll(
        codec_dai,
        WM8994_FLL1,
        WM8994_FLL_SRC_MCLK2,
        32768,
        (sample_rate * 512) as c_uint,
    );
    if ret < 0 {
        pr_err(b"Failed to start FLL: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        WM8994_SYSCLK_FLL1,
        (sample_rate * 512) as c_uint,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        pr_err(b"Failed to set SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    0
}

static littlemill_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(littlemill_hw_params),
};

static baseband_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    rate_min: 8000,
    rate_max: 8000,
    channels_min: 2,
    channels_max: 2,
};

// SND_SOC_DAILINK_DEFS(cpu,
//     DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm8994-codec", "wm8994-aif1")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));
//
// SND_SOC_DAILINK_DEFS(baseband,
//     DAILINK_COMP_ARRAY(COMP_CPU("wm8994-aif2")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm1250-ev1.1-0027", "wm1250-ev1")));

static mut littlemill_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"CPU\0".as_ptr() as *const c_char,
        stream_name: b"CPU\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: &littlemill_ops,
        ignore_suspend: 0,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        // SND_SOC_DAILINK_REG(cpu),
    },
    snd_soc_dai_link {
        name: b"Baseband\0".as_ptr() as *const c_char,
        stream_name: b"Baseband\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: ptr::null(),
        ignore_suspend: 1,
        c2c_params: &baseband_params,
        num_c2c_params: 1,
        // SND_SOC_DAILINK_REG(baseband),
    },
];

unsafe extern "C" fn bbclk_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let rtd: *mut snd_soc_pcm_runtime;
    let aif2_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(1));
    aif2_dai = snd_soc_rtd_to_cpu(rtd, 0);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            ret = snd_soc_dai_set_pll(
                aif2_dai,
                WM8994_FLL2,
                WM8994_FLL_SRC_BCLK,
                64 * 8000,
                8000 * 256,
            );
            if ret < 0 {
                pr_err(b"Failed to start FLL: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }

            ret = snd_soc_dai_set_sysclk(
                aif2_dai,
                WM8994_SYSCLK_FLL2,
                8000 * 256,
                SND_SOC_CLOCK_IN,
            );
            if ret < 0 {
                pr_err(b"Failed to set SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            ret = snd_soc_dai_set_sysclk(aif2_dai, WM8994_SYSCLK_MCLK2, 32768, SND_SOC_CLOCK_IN);
            if ret < 0 {
                pr_err(
                    b"Failed to switch away from FLL2: %d\n\0".as_ptr() as *const c_char,
                    ret,
                );
                return ret;
            }

            ret = snd_soc_dai_set_pll(aif2_dai, WM8994_FLL2, 0, 0, 0);
            if ret < 0 {
                pr_err(b"Failed to stop FLL2: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

// static const struct snd_kcontrol_new controls[] = {
//     SOC_DAPM_PIN_SWITCH("Headphone"),
//     SOC_DAPM_PIN_SWITCH("Headset Mic"),
//     SOC_DAPM_PIN_SWITCH("WM1250 Input"),
//     SOC_DAPM_PIN_SWITCH("WM1250 Output"),
// };
static controls: [snd_kcontrol_new; 0] = [];

// static const struct snd_soc_dapm_widget widgets[] = {
//     SND_SOC_DAPM_HP("Headphone", NULL),
//     SND_SOC_DAPM_HP("Headset Mic", NULL),
//
//     SND_SOC_DAPM_MIC("AMIC", NULL),
//     SND_SOC_DAPM_MIC("DMIC", NULL),
//
//     SND_SOC_DAPM_SUPPLY_S("Baseband Clock", -1, SND_SOC_NOPM, 0, 0,
//                           bbclk_ev,
//                           SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
// };
static widgets: [snd_soc_dapm_widget; 0] = [];

static audio_paths: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOUT1L\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOUT1R\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AMIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"MICBIAS1\0".as_ptr() as *const c_char,
    }, // Default for AMICBIAS jumper
    snd_soc_dapm_route {
        sink: b"IN1LN\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"AMIC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"MICBIAS2\0".as_ptr() as *const c_char,
    }, // Default for DMICBIAS jumper
    snd_soc_dapm_route {
        sink: b"DMIC1DAT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DMIC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DMIC2DAT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DMIC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AIF2CLK\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Baseband Clock\0".as_ptr() as *const c_char,
    },
];

static mut littlemill_headset: snd_soc_jack = snd_soc_jack { _private: [] };

static mut littlemill_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn littlemill_late_probe(card: *mut snd_soc_card) -> c_int {
    let mut rtd: *mut snd_soc_pcm_runtime;
    let component: *mut snd_soc_component;
    let aif1_dai: *mut snd_soc_dai;
    let aif2_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));
    component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    aif1_dai = snd_soc_rtd_to_codec(rtd, 0);

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(1));
    aif2_dai = snd_soc_rtd_to_cpu(rtd, 0);

    ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_MCLK2, 32768, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(aif2_dai, WM8994_SYSCLK_MCLK2, 32768, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET
            | SND_JACK_MECHANICAL
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3
            | SND_JACK_BTN_4
            | SND_JACK_BTN_5,
        &mut littlemill_headset,
        littlemill_headset_pins.as_mut_ptr(),
        littlemill_headset_pins.len() as c_uint,
    );
    if ret != 0 {
        return ret;
    }

    /* This will check device compatibility itself */
    wm8958_mic_detect(
        component,
        &mut littlemill_headset,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );

    /* As will this */
    wm8994_mic_detect(component, &mut littlemill_headset, 1);

    0
}

static mut littlemill: snd_soc_card = snd_soc_card {
    name: b"Littlemill\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dai_link: unsafe { littlemill_dai.as_mut_ptr() },
    num_links: 2,
    dev: ptr::null_mut(),
    set_bias_level: Some(littlemill_set_bias_level),
    set_bias_level_post: Some(littlemill_set_bias_level_post),
    controls: controls.as_ptr(),
    num_controls: controls.len() as c_int,
    dapm_widgets: widgets.as_ptr(),
    num_dapm_widgets: widgets.len() as c_int,
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: audio_paths.len() as c_int,
    late_probe: Some(littlemill_late_probe),
};

unsafe extern "C" fn littlemill_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut littlemill;
    let ret: c_int;

    (*card).dev = &mut (*pdev).dev;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const c_char,
        );
    }

    ret
}

static mut littlemill_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"littlemill\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(littlemill_probe),
};

// module_platform_driver(littlemill_driver);
// MODULE_DESCRIPTION("Littlemill audio support");
// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:littlemill");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
