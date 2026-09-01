// SPDX-License-Identifier: GPL-2.0+
//
// Speyside audio support
//
// Copyright 2011 Wolfson Microelectronics

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const MCLK_AUDIO_RATE: c_int = 512 * 48000;

type snd_soc_bias_level = c_int;
type gpiod_flags = c_int;

const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;

const SND_SOC_CLOCK_IN: c_int = 0;
const WM8996_SYSCLK_MCLK2: c_int = 0;
const WM8996_FLL_MCLK2: c_int = 0;
const WM8996_SYSCLK_FLL: c_int = 0;
const WM9081_SYSCLK_MCLK: c_int = 0;
const GPIOD_OUT_HIGH: gpiod_flags = 1;
const GPIOD_OUT_LOW: gpiod_flags = 0;
const GPIO_ACTIVE_HIGH: c_int = 0;

const SND_JACK_MICROPHONE: c_int = 0x0008;
const SND_JACK_LINEOUT: c_int = 0x0200;
const SND_JACK_HEADSET: c_int = 0x0003;
const SND_JACK_BTN_0: c_int = 0x4000;

const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SND_SOC_DAIFMT_I2S: u32 = 1;
const SND_SOC_DAIFMT_NB_NF: u32 = 0;
const SND_SOC_DAIFMT_CBP_CFP: u32 = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub formats: u64,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: u32,
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_int,
    pub ignore_suspend: c_int,
}

#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_dai_link_component,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
    pub connected: Option<
        unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_soc_dapm_widget) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
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
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dev: *mut device,
}

#[repr(C)]
pub struct gpiod_lookup {
    pub chip_label: *const c_char,
    pub chip_hwnum: u16,
    pub con_id: *const c_char,
    pub flags: c_int,
}

#[repr(C)]
pub struct gpiod_lookup_table {
    pub dev_id: *const c_char,
    pub table: [gpiod_lookup; 2],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_get_pcm_runtime(
        card: *mut snd_soc_card,
        dai_link: *mut snd_soc_dai_link,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_int,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_int,
        freq_out: c_int,
    ) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_widget_name_cmp(
        widget: *mut snd_soc_dapm_widget,
        name: *const c_char,
    ) -> c_int;
    fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn devm_gpiod_get(
        dev: *mut device,
        con_id: *const c_char,
        flags: gpiod_flags,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn wm8996_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        polarity_cb: unsafe extern "C" fn(*mut snd_soc_component, c_int),
    );
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_ignore_suspend(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_component_set_sysclk(
        component: *mut snd_soc_component,
        clk_id: c_int,
        source: c_int,
        freq: c_int,
        dir: c_int,
    ) -> c_int;
    fn gpiod_remove_lookup_table(table: *mut gpiod_lookup_table);
    fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe extern "C" fn speyside_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(1));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_STANDBY => {
            ret = snd_soc_dai_set_sysclk(codec_dai, WM8996_SYSCLK_MCLK2, 32768, SND_SOC_CLOCK_IN);
            if ret < 0 {
                return ret;
            }

            ret = snd_soc_dai_set_pll(codec_dai, WM8996_FLL_MCLK2, 0, 0, 0);
            if ret < 0 {
                pr_err(cstr!("Failed to stop FLL\n"));
                return ret;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn speyside_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime;
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(1));
    codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*codec_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_STANDBY {
                ret = snd_soc_dai_set_pll(
                    codec_dai,
                    0,
                    WM8996_FLL_MCLK2,
                    32768,
                    MCLK_AUDIO_RATE,
                );
                if ret < 0 {
                    pr_err(cstr!("Failed to start FLL\n"));
                    return ret;
                }

                ret = snd_soc_dai_set_sysclk(
                    codec_dai,
                    WM8996_SYSCLK_FLL,
                    MCLK_AUDIO_RATE,
                    SND_SOC_CLOCK_IN,
                );
                if ret < 0 {
                    return ret;
                }
            }
        }
        _ => {}
    }

    0
}

static mut speyside_headset: snd_soc_jack = snd_soc_jack { _private: [] };

/* Headset jack detection DAPM pins */
static mut speyside_headset_pins: [snd_soc_jack_pin; 1] = [snd_soc_jack_pin {
    pin: cstr!("Headset Mic"),
    mask: SND_JACK_MICROPHONE,
}];

static mut speyside_hpsel_gpio: *mut gpio_desc = ptr::null_mut();
/* Default the headphone selection to active high */
static mut speyside_jack_polarity: c_int = 0;

unsafe extern "C" fn speyside_get_micbias(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    if speyside_jack_polarity != 0
        && snd_soc_dapm_widget_name_cmp(source, cstr!("MICB1")) == 0
    {
        return 1;
    }
    if speyside_jack_polarity == 0
        && snd_soc_dapm_widget_name_cmp(source, cstr!("MICB2")) == 0
    {
        return 1;
    }

    0
}

unsafe extern "C" fn speyside_set_polarity(component: *mut snd_soc_component, polarity: c_int) {
    speyside_jack_polarity = if polarity == 0 { 1 } else { 0 };
    gpiod_direction_output(speyside_hpsel_gpio, speyside_jack_polarity);

    /* Re-run DAPM to make sure we're using the correct mic bias */
    snd_soc_dapm_sync(snd_soc_component_to_dapm(component));
}

unsafe extern "C" fn speyside_wm0010_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let ret: c_int;

    ret = snd_soc_dai_set_sysclk(dai, 0, MCLK_AUDIO_RATE, 0);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn speyside_wm8996_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component: *mut snd_soc_component = (*dai).component;
    let flags: gpiod_flags;
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(dai, WM8996_SYSCLK_MCLK2, 32768, 0);
    if ret < 0 {
        return ret;
    }

    if speyside_jack_polarity != 0 {
        flags = GPIOD_OUT_HIGH;
    } else {
        flags = GPIOD_OUT_LOW;
    }
    speyside_hpsel_gpio = devm_gpiod_get((*(*rtd).card).dev, cstr!("hp-sel"), flags);
    if IS_ERR(speyside_hpsel_gpio as *const c_void) {
        return PTR_ERR(speyside_hpsel_gpio as *const c_void);
    }

    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        cstr!("Headset"),
        SND_JACK_LINEOUT | SND_JACK_HEADSET | SND_JACK_BTN_0,
        &mut speyside_headset,
        speyside_headset_pins.as_mut_ptr(),
        speyside_headset_pins.len() as c_int,
    );
    if ret != 0 {
        return ret;
    }

    wm8996_detect(component, &mut speyside_headset, speyside_set_polarity);

    0
}

unsafe extern "C" fn speyside_late_probe(card: *mut snd_soc_card) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);

    snd_soc_dapm_ignore_suspend(dapm, cstr!("Headphone"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("Headset Mic"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("Main AMIC"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("Main DMIC"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("Main Speaker"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("WM1250 Output"));
    snd_soc_dapm_ignore_suspend(dapm, cstr!("WM1250 Input"));

    0
}

static dsp_codec_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    formats: SNDRV_PCM_FMTBIT_S32_LE,
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
};

/* SND_SOC_DAILINK_DEFS(cpu_dsp,
 *     DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("spi0.0", "wm0010-sdi1")),
 *     DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));
 *
 * SND_SOC_DAILINK_DEFS(dsp_codec,
 *     DAILINK_COMP_ARRAY(COMP_CPU("wm0010-sdi2")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("wm8996.1-001a", "wm8996-aif1")));
 *
 * SND_SOC_DAILINK_DEFS(baseband,
 *     DAILINK_COMP_ARRAY(COMP_CPU("wm8996-aif2")),
 *     DAILINK_COMP_ARRAY(COMP_CODEC("wm1250-ev1.1-0027", "wm1250-ev1")));
 */

static mut speyside_dai: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: cstr!("CPU-DSP"),
        stream_name: cstr!("CPU-DSP"),
        init: Some(speyside_wm0010_init),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        ignore_suspend: 0,
    },
    snd_soc_dai_link {
        name: cstr!("DSP-CODEC"),
        stream_name: cstr!("DSP-CODEC"),
        init: Some(speyside_wm8996_init),
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: &dsp_codec_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
    },
    snd_soc_dai_link {
        name: cstr!("Baseband"),
        stream_name: cstr!("Baseband"),
        init: None,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        ignore_suspend: 1,
    },
];

unsafe extern "C" fn speyside_wm9081_init(component: *mut snd_soc_component) -> c_int {
    /* At any time the WM9081 is active it will have this clock */
    snd_soc_component_set_sysclk(component, WM9081_SYSCLK_MCLK, 0, MCLK_AUDIO_RATE, 0)
}

static mut speyside_aux_dev: [snd_soc_aux_dev; 1] = [snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component {
        name: cstr!("wm9081.1-006c"),
        dai_name: ptr::null(),
    },
    init: Some(speyside_wm9081_init),
}];

static mut speyside_codec_conf: [snd_soc_codec_conf; 1] = [snd_soc_codec_conf {
    dlc: snd_soc_dai_link_component {
        name: cstr!("wm9081.1-006c"),
        dai_name: ptr::null(),
    },
    name_prefix: cstr!("Sub"),
}];

/* SOC_DAPM_PIN_SWITCH("...") entries from the C controls array. */
static controls: [snd_kcontrol_new; 6] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

/* SND_SOC_DAPM_HP/MIC/SPK widget declarations from the C widgets array. */
static widgets: [snd_soc_dapm_widget_desc; 5] = [
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
    snd_soc_dapm_widget_desc { _private: [] },
];

static audio_paths: [snd_soc_dapm_route; 21] = [
    snd_soc_dapm_route { sink: cstr!("IN1RN"), control: ptr::null(), source: cstr!("MICB1"), connected: None },
    snd_soc_dapm_route { sink: cstr!("IN1RP"), control: ptr::null(), source: cstr!("MICB1"), connected: None },
    snd_soc_dapm_route { sink: cstr!("IN1RN"), control: ptr::null(), source: cstr!("MICB2"), connected: None },
    snd_soc_dapm_route { sink: cstr!("IN1RP"), control: ptr::null(), source: cstr!("MICB2"), connected: None },
    snd_soc_dapm_route { sink: cstr!("MICB1"), control: ptr::null(), source: cstr!("Headset Mic"), connected: Some(speyside_get_micbias) },
    snd_soc_dapm_route { sink: cstr!("MICB2"), control: ptr::null(), source: cstr!("Headset Mic"), connected: Some(speyside_get_micbias) },
    snd_soc_dapm_route { sink: cstr!("IN1LP"), control: ptr::null(), source: cstr!("MICB2"), connected: None },
    snd_soc_dapm_route { sink: cstr!("IN1RN"), control: ptr::null(), source: cstr!("MICB1"), connected: None },
    snd_soc_dapm_route { sink: cstr!("MICB2"), control: ptr::null(), source: cstr!("Main AMIC"), connected: None },
    snd_soc_dapm_route { sink: cstr!("DMIC1DAT"), control: ptr::null(), source: cstr!("MICB1"), connected: None },
    snd_soc_dapm_route { sink: cstr!("DMIC2DAT"), control: ptr::null(), source: cstr!("MICB1"), connected: None },
    snd_soc_dapm_route { sink: cstr!("MICB1"), control: ptr::null(), source: cstr!("Main DMIC"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Headphone"), control: ptr::null(), source: cstr!("HPOUT1L"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Headphone"), control: ptr::null(), source: cstr!("HPOUT1R"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Sub IN1"), control: ptr::null(), source: cstr!("HPOUT2L"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Sub IN2"), control: ptr::null(), source: cstr!("HPOUT2R"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Main Speaker"), control: ptr::null(), source: cstr!("Sub SPKN"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Main Speaker"), control: ptr::null(), source: cstr!("Sub SPKP"), connected: None },
    snd_soc_dapm_route { sink: cstr!("Main Speaker"), control: ptr::null(), source: cstr!("SPKDAT"), connected: None },
];

static mut speyside: snd_soc_card = snd_soc_card {
    name: cstr!("Speyside"),
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { speyside_dai.as_mut_ptr() },
    num_links: 3,
    aux_dev: unsafe { speyside_aux_dev.as_mut_ptr() },
    num_aux_devs: 1,
    codec_conf: unsafe { speyside_codec_conf.as_mut_ptr() },
    num_configs: 1,

    set_bias_level: Some(speyside_set_bias_level),
    set_bias_level_post: Some(speyside_set_bias_level_post),

    controls: controls.as_ptr(),
    num_controls: 6,
    dapm_widgets: widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: audio_paths.as_ptr(),
    num_dapm_routes: 21,
    fully_routed: true,

    late_probe: Some(speyside_late_probe),
    dev: ptr::null_mut(),
};

static mut wm8996_gpiod_table: gpiod_lookup_table = gpiod_lookup_table {
    /* Hardcoded device name in board file mach-crag6410.c */
    dev_id: cstr!("speyside"),
    table: [
        /*
         * This line was hardcoded to 214 in the global GPIO
         * number space, S3C GPIO macros seems top set the
         * wm8996 codec GPIO start offset to 212, so this will
         * be GPIO 214 - 212 = 2 on the wm8996.
         */
        gpiod_lookup {
            chip_label: cstr!("wm8996"),
            chip_hwnum: 2,
            con_id: cstr!("hp-sel"),
            flags: GPIO_ACTIVE_HIGH,
        },
        gpiod_lookup {
            chip_label: ptr::null(),
            chip_hwnum: 0,
            con_id: ptr::null(),
            flags: 0,
        },
    ],
};

unsafe extern "C" fn speyside_gpiod_table_action(_data: *mut c_void) {
    gpiod_remove_lookup_table(&mut wm8996_gpiod_table);
}

unsafe extern "C" fn speyside_probe(pdev: *mut platform_device) -> c_int {
    let card: *mut snd_soc_card = &mut speyside;
    let mut ret: c_int;

    (*card).dev = &mut (*pdev).dev;

    gpiod_add_lookup_table(&mut wm8996_gpiod_table);
    ret = devm_add_action_or_reset(
        &mut (*pdev).dev,
        speyside_gpiod_table_action,
        ptr::null_mut(),
    );
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            cstr!("snd_soc_register_card() failed\n"),
        );
    }

    ret
}

static mut speyside_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: cstr!("speyside"),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(speyside_probe),
};

/* module_platform_driver(speyside_driver); */

/* MODULE_DESCRIPTION("Speyside audio support"); */
/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:speyside"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
