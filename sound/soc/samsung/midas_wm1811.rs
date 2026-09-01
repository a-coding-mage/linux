// SPDX-License-Identifier: GPL-2.0+
//
// Midas audio support
//
// Copyright (C) 2018 Simon Shields <simon@lineageos.org>
// Copyright (C) 2020 Samsung Electronics Co., Ltd.

// C dependencies translated from:
// linux/clk.h, linux/gpio/consumer.h, linux/iio/consumer.h,
// linux/mfd/wm8994/registers.h, linux/input-event-codes.h, linux/module.h,
// linux/of.h, sound/jack.h, sound/soc.h, sound/soc-dapm.h, i2s.h,
// ../codecs/wm8994.h

use core::ffi::{c_char, c_int, c_uint, c_void};

/*
 * The MCLK1 clock source is XCLKOUT with its mux set to the external fixed rate
 * oscillator (XXTI).
 */
const MCLK1_RATE: c_uint = 24000000;
const MCLK2_RATE: c_uint = 32768;
const DEFAULT_FLL1_RATE: c_uint = 11289600;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const UINT_MAX: c_uint = c_uint::MAX;

const GPIOD_OUT_HIGH: c_int = 0;
const GPIOD_IN: c_int = 1;
const IIO_VOLTAGE: iio_chan_type = 0;

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

const KEY_MEDIA: c_uint = 226;
const KEY_VOLUMEUP: c_uint = 115;
const KEY_VOLUMEDOWN: c_uint = 114;

const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAPM_PRE_PMU: c_int = 0;
const SND_SOC_DAPM_POST_PMD: c_int = 1;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;
const SND_SOC_BIAS_PREPARE: snd_soc_bias_level = 2;

const WM8994_SYSCLK_MCLK2: c_int = 0;
const WM8994_SYSCLK_FLL1: c_int = 1;
const WM8994_FLL1: c_int = 1;
const WM8994_FLL_SRC_MCLK1: c_int = 1;
const WM8994_SPKOUT_MIXERS: c_uint = 0;
const WM8994_SPKMIXR_TO_SPKOUTL_MASK: c_uint = 0;
const WM8994_SPKMIXR_TO_SPKOUTL: c_uint = 0;

const SAMSUNG_I2S_OPCLK: c_int = 0;
const SAMSUNG_I2S_OPCLK_PCLK: c_int = 0;

const SNDRV_PCM_RATE_8000: c_uint = 0x0001;
const SNDRV_PCM_RATE_16000: c_uint = 0x0002;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 4;

type iio_chan_type = c_int;
type snd_soc_bias_level = c_int;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_channel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
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
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
    pub status: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_zone {
    pub min_mv: c_uint,
    pub max_mv: c_uint,
    pub jack_type: c_int,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub debounce_time: c_int,
    pub jack_status_check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub data: *mut c_void,
    pub desc: *mut gpio_desc,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub dev: *mut device,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub set_bias_level: Option<
        unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dapm_context, snd_soc_bias_level) -> c_int,
    >,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
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
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
    pub ignore_suspend: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub cpus: *mut snd_soc_dai_link_component,
    pub platforms: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct midas_priv {
    gpio_fm_sel: *mut gpio_desc,
    gpio_lineout_sel: *mut gpio_desc,
    gpio_headset_detect: *mut gpio_desc,
    gpio_headset_key: *mut gpio_desc,
    adc_headset_detect: *mut iio_channel,
    fll1_rate: c_uint,

    headset_jack: snd_soc_jack,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;

    fn iio_read_channel_processed(chan: *mut iio_channel, val: *mut c_int) -> c_int;
    fn devm_iio_channel_get(dev: *mut device, consumer_channel: *const c_char) -> *mut iio_channel;
    fn iio_get_channel_type(chan: *mut iio_channel, type_: *mut iio_chan_type) -> c_int;

    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_dapm_force_enable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn snd_soc_jack_get_type(jack: *mut snd_soc_jack, micbias_voltage: c_int) -> c_int;
    fn snd_soc_jack_add_zones(
        jack: *mut snd_soc_jack,
        count: c_int,
        zones: *mut snd_soc_jack_zone,
    ) -> c_int;
    fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;

    fn wm8958_mic_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        det: *mut c_void,
        hp_cb: *mut c_void,
        mic_cb: *mut c_void,
        det_cb: *mut c_void,
    );

    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_uint);

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn msleep(msecs: c_uint);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;

    fn of_property_read_u32_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut u32,
        sz: usize,
    ) -> c_int;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
}

static mut HEADSET_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

/*
 * min_mv/max_mv values in this struct are set up based on DT values.
 */
static mut HEADSET_JACK_ZONES: [snd_soc_jack_zone; 3] = [
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_HEADSET,
    },
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_HEADPHONE,
    },
];

/*
 * This is used for manual detection in headset_key_check, we reuse the
 * structure since it's convenient.
 *
 * min_mv/max_mv values in this struct are set up based on DT values.
 */
static mut HEADSET_KEY_ZONES: [snd_soc_jack_zone; 3] = [
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_BTN_0,
    }, /* Media */
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_BTN_1,
    }, /* Volume Up */
    snd_soc_jack_zone {
        min_mv: 0,
        max_mv: 0,
        jack_type: SND_JACK_BTN_2,
    }, /* Volume Down */
];

unsafe extern "C" fn headset_jack_check(data: *mut c_void) -> c_int {
    let codec = data as *mut snd_soc_component;
    let dapm = snd_soc_component_to_dapm(codec);
    let priv_ = snd_soc_card_get_drvdata((*codec).card) as *mut midas_priv;
    let mut adc: c_int = 0;
    let mut ret: c_int;
    let mut jack_type: c_int = 0;

    if gpiod_get_value_cansleep((*priv_).gpio_headset_detect) == 0 {
        return 0;
    }

    /* Enable headset mic bias regulator so that the ADC reading works */
    ret = snd_soc_dapm_force_enable_pin(dapm, c"headset-mic-bias".as_ptr());
    if ret < 0 {
        pr_err(
            c"%s: Failed to enable headset mic bias regulator (%d), assuming headphones\n".as_ptr(),
            c"headset_jack_check".as_ptr(),
            ret,
        );
        return SND_JACK_HEADPHONE;
    }
    snd_soc_dapm_sync(dapm);

    /* Sleep for a small amount of time to get the value to stabilize */
    msleep(20);

    ret = iio_read_channel_processed((*priv_).adc_headset_detect, &mut adc);
    if ret != 0 {
        pr_err(
            c"%s: Failed to read ADC (%d), assuming headphones\n".as_ptr(),
            c"headset_jack_check".as_ptr(),
            ret,
        );
        jack_type = SND_JACK_HEADPHONE;
    } else {
        pr_debug(c"%s: ADC value is %d\n".as_ptr(), c"headset_jack_check".as_ptr(), adc);
        jack_type = snd_soc_jack_get_type(&mut (*priv_).headset_jack, adc);
    }

    ret = snd_soc_dapm_disable_pin(dapm, c"headset-mic-bias".as_ptr());
    if ret < 0 {
        pr_err(
            c"%s: Failed to disable headset mic bias regulator (%d)\n".as_ptr(),
            c"headset_jack_check".as_ptr(),
            ret,
        );
    }
    snd_soc_dapm_sync(dapm);

    jack_type
}

unsafe extern "C" fn headset_key_check(data: *mut c_void) -> c_int {
    let codec = data as *mut snd_soc_component;
    let priv_ = snd_soc_card_get_drvdata((*codec).card) as *mut midas_priv;
    let mut adc: c_int = 0;
    let mut i: usize;
    let ret: c_int;

    if gpiod_get_value_cansleep((*priv_).gpio_headset_key) == 0 {
        return 0;
    }

    /* Filter out keypresses when 4 pole jack not detected */
    if ((*priv_).headset_jack.status & SND_JACK_MICROPHONE) == 0 {
        return 0;
    }

    ret = iio_read_channel_processed((*priv_).adc_headset_detect, &mut adc);
    if ret != 0 {
        pr_err(
            c"%s: Failed to read ADC (%d), can't detect key type\n".as_ptr(),
            c"headset_key_check".as_ptr(),
            ret,
        );
        return 0;
    }
    pr_debug(c"%s: ADC value is %d\n".as_ptr(), c"headset_key_check".as_ptr(), adc);

    i = 0;
    while i < HEADSET_KEY_ZONES.len() {
        if adc >= HEADSET_KEY_ZONES[i].min_mv as c_int && adc <= HEADSET_KEY_ZONES[i].max_mv as c_int {
            return HEADSET_KEY_ZONES[i].jack_type;
        }
        i += 1;
    }

    0
}

static mut HEADSET_GPIO: [snd_soc_jack_gpio; 2] = [
    snd_soc_jack_gpio {
        name: c"Headset Jack".as_ptr(),
        report: SND_JACK_HEADSET,
        debounce_time: 150,
        jack_status_check: Some(headset_jack_check),
        data: core::ptr::null_mut(),
        desc: core::ptr::null_mut(),
    },
    snd_soc_jack_gpio {
        name: c"Headset Key".as_ptr(),
        report: SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2,
        debounce_time: 30,
        jack_status_check: Some(headset_key_check),
        data: core::ptr::null_mut(),
        desc: core::ptr::null_mut(),
    },
];

unsafe extern "C" fn midas_start_fll1(rtd: *mut snd_soc_pcm_runtime, mut rate: c_uint) -> c_int {
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut midas_priv;
    let aif1_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    if rate == 0 {
        rate = (*priv_).fll1_rate;
    }
    /*
     * If no new rate is requested, set FLL1 to a sane default for jack
     * detection.
     */
    if rate == 0 {
        rate = DEFAULT_FLL1_RATE;
    }

    if rate != (*priv_).fll1_rate && (*priv_).fll1_rate != 0 {
        /* while reconfiguring, switch to MCLK2 for SYSCLK */
        ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_MCLK2, MCLK2_RATE, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*card).dev, c"Unable to switch to MCLK2: %d\n".as_ptr(), ret);
            return ret;
        }
    }

    ret = snd_soc_dai_set_pll(aif1_dai, WM8994_FLL1, WM8994_FLL_SRC_MCLK1, MCLK1_RATE, rate);
    if ret < 0 {
        dev_err((*card).dev, c"Failed to set FLL1 rate: %d\n".as_ptr(), ret);
        return ret;
    }
    (*priv_).fll1_rate = rate;

    ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_FLL1, (*priv_).fll1_rate, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*card).dev, c"Failed to set SYSCLK source: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(cpu_dai, SAMSUNG_I2S_OPCLK, 0, SAMSUNG_I2S_OPCLK_PCLK);
    if ret < 0 {
        dev_err((*card).dev, c"Failed to set OPCLK source: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn midas_stop_fll1(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut midas_priv;
    let aif1_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_MCLK2, MCLK2_RATE, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*card).dev, c"Unable to switch to MCLK2: %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_pll(aif1_dai, WM8994_FLL1, 0, 0, 0);
    if ret < 0 {
        dev_err((*card).dev, c"Unable to stop FLL1: %d\n".as_ptr(), ret);
        return ret;
    }

    (*priv_).fll1_rate = 0;

    0
}

unsafe extern "C" fn midas_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let pll_out: c_uint;

    /* AIF1CLK should be at least 3MHz for "optimal performance" */
    if params_rate(params) == 8000 || params_rate(params) == 11025 {
        pll_out = params_rate(params).wrapping_mul(512);
    } else {
        pll_out = params_rate(params).wrapping_mul(256);
    }

    midas_start_fll1(rtd, pll_out)
}

static MIDAS_AIF1_OPS: snd_soc_ops = snd_soc_ops {
    hw_params: Some(midas_aif1_hw_params),
};

/*
 * We only have a single external speaker, so mix stereo data
 * to a single mono stream.
 */
unsafe extern "C" fn midas_ext_spkmode(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let codec = snd_soc_dapm_to_component((*w).dapm);
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            ret = snd_soc_component_update_bits(
                codec,
                WM8994_SPKOUT_MIXERS,
                WM8994_SPKMIXR_TO_SPKOUTL_MASK,
                WM8994_SPKMIXR_TO_SPKOUTL,
            );
        }
        SND_SOC_DAPM_POST_PMD => {
            ret = snd_soc_component_update_bits(codec, WM8994_SPKOUT_MIXERS, WM8994_SPKMIXR_TO_SPKOUTL_MASK, 0);
        }
        _ => {}
    }

    ret
}

unsafe extern "C" fn midas_fm_set(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut midas_priv;

    if (*priv_).gpio_fm_sel.is_null() {
        return 0;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => gpiod_set_value_cansleep((*priv_).gpio_fm_sel, 1),
        SND_SOC_DAPM_POST_PMD => gpiod_set_value_cansleep((*priv_).gpio_fm_sel, 0),
        _ => {}
    }

    0
}

unsafe extern "C" fn midas_line_set(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut midas_priv;

    if (*priv_).gpio_lineout_sel.is_null() {
        return 0;
    }

    match event {
        SND_SOC_DAPM_PRE_PMU => gpiod_set_value_cansleep((*priv_).gpio_lineout_sel, 1),
        SND_SOC_DAPM_POST_PMD => gpiod_set_value_cansleep((*priv_).gpio_lineout_sel, 0),
        _ => {}
    }

    0
}

static MIDAS_CONTROLS: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new { name: c"HP".as_ptr() },
    snd_kcontrol_new { name: c"SPK".as_ptr() },
    snd_kcontrol_new { name: c"RCV".as_ptr() },
    snd_kcontrol_new { name: c"LINE".as_ptr() },
    snd_kcontrol_new { name: c"HDMI".as_ptr() },
    snd_kcontrol_new { name: c"Main Mic".as_ptr() },
    snd_kcontrol_new { name: c"Sub Mic".as_ptr() },
    snd_kcontrol_new { name: c"Headset Mic".as_ptr() },
    snd_kcontrol_new { name: c"FM In".as_ptr() },
];

static MIDAS_DAPM_WIDGETS: [snd_soc_dapm_widget; 14] = [
    snd_soc_dapm_widget { name: c"HP".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"SPK".as_ptr(), event: Some(midas_ext_spkmode), dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"RCV".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    /* FIXME: toggle MAX77693 on i9300/i9305 */
    snd_soc_dapm_widget { name: c"LINE".as_ptr(), event: Some(midas_line_set), dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"HDMI".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"FM In".as_ptr(), event: Some(midas_fm_set), dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"Headphone".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"Headset Mic".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"headset-mic-bias".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"Main Mic".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"mic-bias".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"Sub Mic".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: c"submic-bias".as_ptr(), event: None, dapm: core::ptr::null_mut() },
    snd_soc_dapm_widget { name: core::ptr::null(), event: None, dapm: core::ptr::null_mut() },
];

/* Default routing; supplemented by audio-routing DT property */
static MIDAS_DAPM_ROUTES: [snd_soc_dapm_route; 3] = [
    /* Bind microphones with their respective regulator supplies */
    snd_soc_dapm_route { sink: c"Main Mic".as_ptr(), control: core::ptr::null(), source: c"mic-bias".as_ptr() },
    snd_soc_dapm_route { sink: c"Sub Mic".as_ptr(), control: core::ptr::null(), source: c"submic-bias".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: core::ptr::null(), source: c"headset-mic-bias".as_ptr() },
];

unsafe extern "C" fn midas_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));
    let aif1_dai = snd_soc_rtd_to_codec(rtd, 0);

    if snd_soc_dapm_to_dev(dapm) != (*aif1_dai).dev {
        return 0;
    }

    match level {
        SND_SOC_BIAS_STANDBY => return midas_stop_fll1(rtd),
        SND_SOC_BIAS_PREPARE => return midas_start_fll1(rtd, 0),
        _ => {}
    }

    0
}

unsafe extern "C" fn midas_late_probe(card: *mut snd_soc_card) -> c_int {
    let rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link.add(0));
    let aif1_dai = snd_soc_rtd_to_codec(rtd, 0);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut midas_priv;
    let mut ret: c_int;

    /* Use MCLK2 as SYSCLK for boot */
    ret = snd_soc_dai_set_sysclk(aif1_dai, WM8994_SYSCLK_MCLK2, MCLK2_RATE, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*aif1_dai).dev, c"Failed to switch to MCLK2: %d\n".as_ptr(), ret);
        return ret;
    }

    if (*priv_).gpio_headset_detect.is_null() {
        ret = snd_soc_card_jack_new_pins(
            card,
            c"Headset".as_ptr(),
            SND_JACK_HEADSET
                | SND_JACK_MECHANICAL
                | SND_JACK_BTN_0
                | SND_JACK_BTN_1
                | SND_JACK_BTN_2
                | SND_JACK_BTN_3
                | SND_JACK_BTN_4
                | SND_JACK_BTN_5,
            &mut (*priv_).headset_jack,
            HEADSET_JACK_PINS.as_mut_ptr(),
            HEADSET_JACK_PINS.len() as c_uint,
        );
        if ret != 0 {
            return ret;
        }

        wm8958_mic_detect(
            (*aif1_dai).component,
            &mut (*priv_).headset_jack,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    } else {
        /* Some devices (n8000, t310) use a GPIO to detect the jack. */
        ret = snd_soc_card_jack_new_pins(
            card,
            c"Headset".as_ptr(),
            SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2,
            &mut (*priv_).headset_jack,
            HEADSET_JACK_PINS.as_mut_ptr(),
            HEADSET_JACK_PINS.len() as c_uint,
        );
        if ret != 0 {
            dev_err((*card).dev, c"Failed to set up headset pins: %d\n".as_ptr(), ret);
            return ret;
        }

        ret = snd_soc_jack_add_zones(&mut (*priv_).headset_jack, HEADSET_JACK_ZONES.len() as c_int, HEADSET_JACK_ZONES.as_mut_ptr());
        if ret != 0 {
            dev_err((*card).dev, c"Failed to set up headset zones: %d\n".as_ptr(), ret);
            return ret;
        }

        HEADSET_GPIO[0].data = (*aif1_dai).component as *mut c_void;
        HEADSET_GPIO[0].desc = (*priv_).gpio_headset_detect;

        HEADSET_GPIO[1].data = (*aif1_dai).component as *mut c_void;
        HEADSET_GPIO[1].desc = (*priv_).gpio_headset_key;

        snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_0, KEY_MEDIA);
        snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
        snd_jack_set_key((*priv_).headset_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);

        ret = snd_soc_jack_add_gpios(&mut (*priv_).headset_jack, HEADSET_GPIO.len() as c_int, HEADSET_GPIO.as_mut_ptr());
        if ret != 0 {
            dev_err((*card).dev, c"Failed to set up headset jack GPIOs: %d\n".as_ptr(), ret);
        }

        return ret;
    }

    0
}

static mut MIDAS_EXT_DAI: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"Voice call".as_ptr(),
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 16000,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 16000,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
    },
    snd_soc_dai_driver {
        name: c"Bluetooth".as_ptr(),
        playback: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 16000,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream {
            channels_min: 1,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 16000,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
    },
];

static MIDAS_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: c"midas-audio".as_ptr(),
};

// SND_SOC_DAILINK_DEFS(wm1811_hifi,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "wm8994-aif1")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut WM1811_HIFI_CPUS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_HIFI_CODECS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_HIFI_PLATFORMS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];

// SND_SOC_DAILINK_DEFS(wm1811_voice,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "wm8994-aif2")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut WM1811_VOICE_CPUS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_VOICE_CODECS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_VOICE_PLATFORMS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];

// SND_SOC_DAILINK_DEFS(wm1811_bt,
//     DAILINK_COMP_ARRAY(COMP_EMPTY()),
//     DAILINK_COMP_ARRAY(COMP_CODEC(NULL, "wm8994-aif3")),
//     DAILINK_COMP_ARRAY(COMP_EMPTY()));
static mut WM1811_BT_CPUS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_BT_CODECS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];
static mut WM1811_BT_PLATFORMS: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { of_node: core::ptr::null_mut() }];

static mut MIDAS_DAI: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"WM8994 AIF1".as_ptr(),
        stream_name: c"HiFi Primary".as_ptr(),
        ops: &MIDAS_AIF1_OPS,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ignore_suspend: 0,
        codecs: unsafe { WM1811_HIFI_CODECS.as_mut_ptr() },
        cpus: unsafe { WM1811_HIFI_CPUS.as_mut_ptr() },
        platforms: unsafe { WM1811_HIFI_PLATFORMS.as_mut_ptr() },
    },
    snd_soc_dai_link {
        name: c"WM1811 Voice".as_ptr(),
        stream_name: c"Voice call".as_ptr(),
        ops: core::ptr::null(),
        dai_fmt: 0,
        ignore_suspend: 1,
        codecs: unsafe { WM1811_VOICE_CODECS.as_mut_ptr() },
        cpus: unsafe { WM1811_VOICE_CPUS.as_mut_ptr() },
        platforms: unsafe { WM1811_VOICE_PLATFORMS.as_mut_ptr() },
    },
    snd_soc_dai_link {
        name: c"WM1811 BT".as_ptr(),
        stream_name: c"Bluetooth".as_ptr(),
        ops: core::ptr::null(),
        dai_fmt: 0,
        ignore_suspend: 1,
        codecs: unsafe { WM1811_BT_CODECS.as_mut_ptr() },
        cpus: unsafe { WM1811_BT_CPUS.as_mut_ptr() },
        platforms: unsafe { WM1811_BT_PLATFORMS.as_mut_ptr() },
    },
];

static mut MIDAS_CARD: snd_soc_card = snd_soc_card {
    name: c"Midas WM1811".as_ptr(),
    owner: unsafe { THIS_MODULE },

    dev: core::ptr::null_mut(),
    dai_link: unsafe { MIDAS_DAI.as_mut_ptr() },
    num_links: 3,
    controls: MIDAS_CONTROLS.as_ptr(),
    num_controls: 9,
    dapm_widgets: MIDAS_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: 14,
    dapm_routes: MIDAS_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: 3,

    set_bias_level: Some(midas_set_bias_level),
    late_probe: Some(midas_late_probe),
};

unsafe extern "C" fn midas_probe(pdev: *mut platform_device) -> c_int {
    let mut cpu_dai_node: *mut device_node = core::ptr::null_mut();
    let mut codec_dai_node: *mut device_node = core::ptr::null_mut();
    let mut cpu: *mut device_node = core::ptr::null_mut();
    let mut codec: *mut device_node = core::ptr::null_mut();
    let card: *mut snd_soc_card = &mut MIDAS_CARD;
    let dev: *mut device = &mut (*pdev).dev;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut channel_type: iio_chan_type = 0;
    let mut fourpole_threshold: [u32; 2] = [0; 2];
    let mut button_threshold: [u32; 3] = [0; 3];
    let priv_: *mut midas_priv;
    let mut ret: c_int;
    let mut i: usize;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<midas_priv>(), GFP_KERNEL) as *mut midas_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);
    (*card).dev = dev;

    (*priv_).gpio_fm_sel = devm_gpiod_get_optional(dev, c"fm-sel".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpio_fm_sel as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpio_fm_sel as *const c_void),
            c"Failed to get FM selection GPIO\n".as_ptr(),
        );
    }

    (*priv_).gpio_lineout_sel = devm_gpiod_get_optional(dev, c"lineout-sel".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpio_lineout_sel as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpio_lineout_sel as *const c_void),
            c"Failed to get line out selection GPIO\n".as_ptr(),
        );
    }

    (*priv_).gpio_headset_detect = devm_gpiod_get_optional(dev, c"headset-detect".as_ptr(), GPIOD_IN);
    if IS_ERR((*priv_).gpio_headset_detect as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpio_headset_detect as *const c_void),
            c"Failed to get headset jack detect GPIO\n".as_ptr(),
        );
    }

    if !(*priv_).gpio_headset_detect.is_null() {
        (*priv_).adc_headset_detect = devm_iio_channel_get(dev, c"headset-detect".as_ptr());
        if IS_ERR((*priv_).adc_headset_detect as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*priv_).adc_headset_detect as *const c_void),
                c"Failed to get ADC channel\n".as_ptr(),
            );
        }

        ret = iio_get_channel_type((*priv_).adc_headset_detect, &mut channel_type);
        if ret != 0 {
            dev_err(dev, c"Failed to get ADC channel type\n".as_ptr());
            return ret;
        }

        if channel_type != IIO_VOLTAGE {
            dev_err(dev, c"ADC channel is not voltage\n".as_ptr());
            return -EINVAL;
        }

        (*priv_).gpio_headset_key = devm_gpiod_get(dev, c"headset-key".as_ptr(), GPIOD_IN);
        if IS_ERR((*priv_).gpio_headset_key as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*priv_).gpio_headset_key as *const c_void),
                c"Failed to get headset key GPIO\n".as_ptr(),
            );
        }

        ret = of_property_read_u32_array(
            (*dev).of_node,
            c"samsung,headset-4pole-threshold-microvolt".as_ptr(),
            fourpole_threshold.as_mut_ptr(),
            fourpole_threshold.len(),
        );
        if ret != 0 {
            dev_err(dev, c"Failed to get 4-pole jack detection threshold\n".as_ptr());
            return ret;
        }

        if fourpole_threshold[0] > fourpole_threshold[1] {
            dev_err(dev, c"Invalid 4-pole jack detection threshold value\n".as_ptr());
            return -EINVAL;
        }

        HEADSET_JACK_ZONES[0].max_mv = fourpole_threshold[0];
        HEADSET_JACK_ZONES[1].min_mv = fourpole_threshold[0].wrapping_add(1);

        HEADSET_JACK_ZONES[1].max_mv = fourpole_threshold[1];
        HEADSET_JACK_ZONES[2].min_mv = fourpole_threshold[1].wrapping_add(1);

        ret = of_property_read_u32_array(
            (*dev).of_node,
            c"samsung,headset-button-threshold-microvolt".as_ptr(),
            button_threshold.as_mut_ptr(),
            button_threshold.len(),
        );
        if ret != 0 {
            dev_err(dev, c"Failed to get headset button detection threshold\n".as_ptr());
            return ret;
        }

        if button_threshold[0] > button_threshold[1] || button_threshold[1] > button_threshold[2] {
            dev_err(dev, c"Invalid headset button detection threshold\u{a0}value\n".as_ptr());
            return -EINVAL;
        }

        i = 0;
        while i < 3 {
            if i != 0 && button_threshold[i] <= 0 {
                dev_err(dev, c"Invalid headset button detection threshold\u{a0}value\n".as_ptr());
                return -EINVAL;
            }

            HEADSET_KEY_ZONES[i].min_mv = button_threshold[i];

            if i == 2 {
                HEADSET_KEY_ZONES[i].max_mv = UINT_MAX;
            } else {
                HEADSET_KEY_ZONES[i].max_mv = button_threshold[i + 1].wrapping_sub(1);
            }
            i += 1;
        }
    }

    ret = snd_soc_of_parse_card_name(card, c"model".as_ptr());
    if ret < 0 {
        dev_err(dev, c"Card name is not specified\n".as_ptr());
        return ret;
    }

    ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
    if ret < 0 {
        /* Backwards compatible way */
        ret = snd_soc_of_parse_audio_routing(card, c"samsung,audio-routing".as_ptr());
        if ret < 0 {
            dev_err(dev, c"Audio routing invalid/unspecified\n".as_ptr());
            return ret;
        }
    }

    cpu = of_get_child_by_name((*dev).of_node, c"cpu".as_ptr());
    if cpu.is_null() {
        return -EINVAL;
    }

    codec = of_get_child_by_name((*dev).of_node, c"codec".as_ptr());
    if codec.is_null() {
        of_node_put(cpu);
        return -EINVAL;
    }

    cpu_dai_node = of_parse_phandle(cpu, c"sound-dai".as_ptr(), 0);
    of_node_put(cpu);
    if cpu_dai_node.is_null() {
        dev_err(dev, c"parsing cpu/sound-dai failed\n".as_ptr());
        of_node_put(codec);
        return -EINVAL;
    }

    codec_dai_node = of_parse_phandle(codec, c"sound-dai".as_ptr(), 0);
    of_node_put(codec);
    if codec_dai_node.is_null() {
        dev_err(dev, c"audio-codec property invalid/missing\n".as_ptr());
        ret = -EINVAL;
        goto_put_cpu_dai_node(ret, cpu_dai_node);
        return ret;
    }

    i = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        (*(*dai_link).codecs).of_node = codec_dai_node;
        (*(*dai_link).cpus).of_node = cpu_dai_node;
        (*(*dai_link).platforms).of_node = cpu_dai_node;
        i += 1;
    }

    ret = devm_snd_soc_register_component(dev, &MIDAS_COMPONENT, MIDAS_EXT_DAI.as_mut_ptr(), MIDAS_EXT_DAI.len() as c_int);
    if ret < 0 {
        dev_err(dev, c"Failed to register component: %d\n".as_ptr(), ret);
        goto_put_codec_dai_node(codec_dai_node);
        goto_put_cpu_dai_node(ret, cpu_dai_node);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret < 0 {
        dev_err(dev, c"Failed to register card: %d\n".as_ptr(), ret);
        goto_put_codec_dai_node(codec_dai_node);
        goto_put_cpu_dai_node(ret, cpu_dai_node);
        return ret;
    }

    0
}

unsafe fn goto_put_codec_dai_node(codec_dai_node: *mut device_node) {
    of_node_put(codec_dai_node);
}

unsafe fn goto_put_cpu_dai_node(ret: c_int, cpu_dai_node: *mut device_node) {
    let _ = ret;
    of_node_put(cpu_dai_node);
}

static MIDAS_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"samsung,midas-audio".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, midas_of_match);

static MIDAS_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"midas-audio".as_ptr(),
        of_match_table: MIDAS_OF_MATCH.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    probe: Some(midas_probe),
};
// module_platform_driver(midas_driver);

// MODULE_AUTHOR("Simon Shields <simon@lineageos.org>");
// MODULE_DESCRIPTION("ASoC support for Midas");
// MODULE_LICENSE("GPL v2");
// MODULE_IMPORT_NS("IIO_CONSUMER");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
