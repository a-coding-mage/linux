// SPDX-License-Identifier: GPL-2.0+
// Translated from soc/samsung/aries_wm8994.c.
// Kernel includes from the C source are external dependencies in Rust form.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type irqreturn_t = c_int;
type gfp_t = c_uint;
type c_uint = u32;

const ARIES_MCLK1_FREQ: c_uint = 24000000;

const UINT_MAX: c_uint = c_uint::MAX;
const NOTIFY_DONE: c_int = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: gfp_t = 0;
const GPIOD_OUT_LOW: c_int = 0;
const GPIOD_IN: c_int = 1;
const IIO_VOLTAGE: iio_chan_type = 0;
const EXTCON_JACK_LINE_OUT: c_uint = 0;
const IRQF_TRIGGER_RISING: c_ulong = 0x00000001;
const IRQF_TRIGGER_FALLING: c_ulong = 0x00000002;
const IRQF_ONESHOT: c_ulong = 0x00002000;
const KEY_MEDIA: c_uint = 226;
const SND_SOC_CLOCK_IN: c_int = 0;

const SND_JACK_LINEOUT: c_uint = 0x000020;
const SND_JACK_HEADPHONE: c_uint = 0x000001;
const SND_JACK_MICROPHONE: c_uint = 0x000004;
const SND_JACK_HEADSET: c_uint = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_uint = 0x4000;

const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x4;
const SND_SOC_DAPM_POST_PMD: c_int = 0x8;

const WM8994_AIF1_DAC1_FILTERS_1: c_uint = 0;
const WM8994_AIF1DAC1_MONO: c_uint = 0;
const WM8994_FLL1: c_int = 0;
const WM8994_FLL2: c_int = 1;
const WM8994_FLL_SRC_MCLK1: c_int = 0;
const WM8994_SYSCLK_FLL1: c_int = 0;
const WM8994_SYSCLK_FLL2: c_int = 1;
const WM8994_SYSCLK_MCLK1: c_int = 2;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1;
const SNDRV_PCM_RATE_8000: c_uint = 1;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;

#[repr(C)] pub struct extcon_dev { _private: [u8; 0] }
#[repr(C)] pub struct regulator { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct iio_channel { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_jack_ptr { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { dev: device }
#[repr(C)] pub struct platform_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int> }
#[repr(C)] pub struct device_driver { name: *const c_char, of_match_table: *const of_device_id, pm: *const c_void }
#[repr(C)] pub struct device { of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
type iio_chan_type = c_int;

#[repr(C)]
struct notifier_block {
    notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_soc_jack_ptr,
    status: c_uint,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
struct snd_soc_jack_zone {
    min_mv: c_uint,
    max_mv: c_uint,
    jack_type: c_uint,
}

#[repr(C)]
struct snd_soc_jack_gpio {
    name: *const c_char,
    report: c_uint,
    debounce_time: c_int,
    jack_status_check: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    data: *mut c_void,
    desc: *mut gpio_desc,
}

#[repr(C)] struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] struct snd_soc_dapm_widget { dapm: *mut snd_soc_dapm_context }
#[repr(C)] struct snd_kcontrol_new { _private: [u8; 0] }

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    formats: u64,
    rate_min: c_uint,
    rate_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
}

#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)] struct snd_soc_component { _private: [u8; 0] }
#[repr(C)] struct snd_soc_codec { component: *mut snd_soc_component }

#[repr(C)]
struct snd_soc_dai_link_component {
    of_node: *mut device_node,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    stream_name: *const c_char,
    dai_fmt: c_uint,
    ops: *const snd_soc_ops,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    c2c_params: *const snd_soc_pcm_stream,
    num_c2c_params: c_uint,
    ignore_suspend: c_uint,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    platforms: *mut snd_soc_dai_link_component,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    dev: *mut device,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct aries_wm8994_variant {
    modem_dai_fmt: c_uint,
    has_fm_radio: bool_,
}

#[repr(C)]
struct aries_wm8994_data {
    usb_extcon: *mut extcon_dev,
    reg_main_micbias: *mut regulator,
    reg_headset_micbias: *mut regulator,
    gpio_headset_detect: *mut gpio_desc,
    gpio_headset_key: *mut gpio_desc,
    gpio_earpath_sel: *mut gpio_desc,
    adc: *mut iio_channel,
    variant: *const aries_wm8994_variant,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_uint, mask: c_uint);
    fn gpiod_get_value(desc: *mut gpio_desc) -> c_int;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn iio_read_channel_processed(chan: *mut iio_channel, val: *mut c_int) -> c_int;
    fn snd_soc_jack_get_type(jack: *mut snd_soc_jack, micbias_voltage: c_int) -> c_uint;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_codec;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_uint, freq_out: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_uint, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn devm_extcon_register_notifier(dev: *mut device, edev: *mut extcon_dev, id: c_uint, nb: *mut notifier_block) -> c_int;
    fn extcon_get_state(edev: *mut extcon_dev, id: c_uint) -> c_int;
    fn snd_soc_jack_add_zones(jack: *mut snd_soc_jack, count: c_int, zones: *mut snd_soc_jack_zone) -> c_int;
    fn gpiod_to_irq(desc: *mut gpio_desc) -> c_int;
    fn devm_request_threaded_irq(dev: *mut device, irq: c_int, handler: *const c_void, thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_soc_jack_ptr, type_: c_uint, keytype: c_uint);
    fn snd_soc_jack_add_gpios(jack: *mut snd_soc_jack, count: c_int, gpios: *mut snd_soc_jack_gpio) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn of_match_node(matches: *const of_device_id, node: *mut device_node) -> *const of_device_id;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int) -> *mut device_node;
    fn extcon_find_edev_by_node(node: *mut device_node) -> *mut extcon_dev;
    fn of_node_put(node: *mut device_node);
    fn devm_iio_channel_get(dev: *mut device, name: *const c_char) -> *mut iio_channel;
    fn iio_get_channel_type(chan: *mut iio_channel, type_: *mut iio_chan_type) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_node_get(node: *mut device_node) -> *mut device_node;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char) -> c_int;
    fn PTR_ERR<T>(ptr: *mut T) -> c_int;
    fn IS_ERR<T>(ptr: *mut T) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

/* USB dock */
static mut aries_dock: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut(), status: 0 };

static mut dock_pins: [snd_soc_jack_pin; 1] = [
    snd_soc_jack_pin { pin: b"LINE\0".as_ptr() as *const c_char, mask: SND_JACK_LINEOUT },
];

unsafe extern "C" fn aries_extcon_notifier(
    _this: *mut notifier_block,
    connected: c_ulong,
    _cmd: *mut c_void,
) -> c_int {
    if connected != 0 {
        snd_soc_jack_report(&raw mut aries_dock, SND_JACK_LINEOUT, SND_JACK_LINEOUT);
    } else {
        snd_soc_jack_report(&raw mut aries_dock, 0, SND_JACK_LINEOUT);
    }
    NOTIFY_DONE
}

static mut aries_extcon_notifier_block: notifier_block = notifier_block {
    notifier_call: Some(aries_extcon_notifier),
};

/* Headset jack */
static mut aries_headset: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut(), status: 0 };

static mut jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"HP\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];

static mut headset_zones: [snd_soc_jack_zone; 3] = [
    snd_soc_jack_zone { min_mv: 0, max_mv: 241, jack_type: SND_JACK_HEADPHONE },
    snd_soc_jack_zone { min_mv: 242, max_mv: 2980, jack_type: SND_JACK_HEADSET },
    snd_soc_jack_zone { min_mv: 2981, max_mv: UINT_MAX, jack_type: SND_JACK_HEADPHONE },
];

unsafe extern "C" fn headset_det_irq_thread(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let priv_ = data as *mut aries_wm8994_data;
    let mut ret: c_int;
    let mut time_left_ms: c_int = 300;
    let mut adc: c_int = 0;

    while time_left_ms > 0 {
        if gpiod_get_value((*priv_).gpio_headset_detect) == 0 {
            snd_soc_jack_report(&raw mut aries_headset, 0, SND_JACK_HEADSET);
            gpiod_set_value((*priv_).gpio_earpath_sel, 0);
            return IRQ_HANDLED;
        }
        msleep(20);
        time_left_ms -= 20;
    }

    /* Temporarily enable micbias and earpath selector */
    ret = regulator_enable((*priv_).reg_headset_micbias);
    if ret != 0 {
        pr_err(c"%s failed to enable micbias: %d".as_ptr(), c"headset_det_irq_thread".as_ptr(), ret);
    }

    gpiod_set_value((*priv_).gpio_earpath_sel, 1);

    ret = iio_read_channel_processed((*priv_).adc, &mut adc);
    if ret < 0 {
        /* failed to read ADC, so assume headphone */
        pr_err(c"%s failed to read ADC, assuming headphones".as_ptr(), c"headset_det_irq_thread".as_ptr());
        snd_soc_jack_report(&raw mut aries_headset, SND_JACK_HEADPHONE, SND_JACK_HEADSET);
    } else {
        let jack_type = snd_soc_jack_get_type(&raw mut aries_headset, adc);
        snd_soc_jack_report(&raw mut aries_headset, jack_type, SND_JACK_HEADSET);
    }

    ret = regulator_disable((*priv_).reg_headset_micbias);
    if ret != 0 {
        pr_err(c"%s failed disable micbias: %d".as_ptr(), c"headset_det_irq_thread".as_ptr(), ret);
    }

    /* Disable earpath selector when no mic connected */
    if (aries_headset.status & SND_JACK_MICROPHONE) == 0 {
        gpiod_set_value((*priv_).gpio_earpath_sel, 0);
    }

    IRQ_HANDLED
}

unsafe extern "C" fn headset_button_check(data: *mut c_void) -> c_int {
    let priv_ = data as *mut aries_wm8994_data;

    /* Filter out keypresses when 4 pole jack not detected */
    if gpiod_get_value_cansleep((*priv_).gpio_headset_key) != 0
        && (aries_headset.status & SND_JACK_MICROPHONE) != 0
    {
        return SND_JACK_BTN_0 as c_int;
    }

    0
}

static mut headset_button_gpio: [snd_soc_jack_gpio; 1] = [
    snd_soc_jack_gpio {
        name: b"Media Button\0".as_ptr() as *const c_char,
        report: SND_JACK_BTN_0,
        debounce_time: 30,
        jack_status_check: Some(headset_button_check),
        data: ptr::null_mut(),
        desc: ptr::null_mut(),
    },
];

unsafe extern "C" fn aries_spk_cfg(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let rtd = snd_soc_get_pcm_runtime(card, &mut *(*card).dai_link.add(0));
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int = 0;

    /**
     * We have an odd setup - the SPKMODE pin is pulled up so
     * we only have access to the left side SPK configs,
     * but SPKOUTR isn't bridged so when playing back in
     * stereo, we only get the left hand channel.  The only
     * option we're left with is to force the AIF into mono
     * mode.
     */
    match event {
        SND_SOC_DAPM_POST_PMU => {
            ret = snd_soc_component_update_bits(
                component,
                WM8994_AIF1_DAC1_FILTERS_1,
                WM8994_AIF1DAC1_MONO,
                WM8994_AIF1DAC1_MONO,
            );
        }
        SND_SOC_DAPM_PRE_PMD => {
            ret = snd_soc_component_update_bits(
                component,
                WM8994_AIF1_DAC1_FILTERS_1,
                WM8994_AIF1DAC1_MONO,
                0,
            );
        }
        _ => {}
    }

    ret
}

unsafe extern "C" fn aries_main_bias(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut aries_wm8994_data;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => ret = regulator_enable((*priv_).reg_main_micbias),
        SND_SOC_DAPM_POST_PMD => ret = regulator_disable((*priv_).reg_main_micbias),
        _ => {}
    }

    ret
}

unsafe extern "C" fn aries_headset_bias(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut aries_wm8994_data;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_PRE_PMU => ret = regulator_enable((*priv_).reg_headset_micbias),
        SND_SOC_DAPM_POST_PMD => ret = regulator_disable((*priv_).reg_headset_micbias),
        _ => {}
    }

    ret
}

/* static const struct snd_kcontrol_new aries_controls[] = {
 *     SOC_DAPM_PIN_SWITCH("Modem In"),
 *     SOC_DAPM_PIN_SWITCH("Modem Out"),
 * };
 */
static aries_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

/* DAPM widget macro initializers from the C source are preserved as entries. */
static aries_dapm_widgets: [snd_soc_dapm_widget; 11] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_HP("HP", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_SPK("SPK", aries_spk_cfg) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_SPK("RCV", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_LINE("LINE", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_MIC("Main Mic", aries_main_bias) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_MIC("Headset Mic", aries_headset_bias) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_MIC("Bluetooth Mic", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_SPK("Bluetooth SPK", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_LINE("Modem In", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_LINE("Modem Out", NULL) */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* This must be last as it is conditionally not used: SND_SOC_DAPM_LINE("FM In", NULL) */
];

unsafe extern "C" fn aries_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_dai;
    let pll_out: c_uint;
    let mut ret: c_int;

    /* AIF1CLK should be >=3MHz for optimal performance */
    if params_width(params) == 24 {
        pll_out = params_rate(params).wrapping_mul(384);
    } else if params_rate(params) == 8000 || params_rate(params) == 11025 {
        pll_out = params_rate(params).wrapping_mul(512);
    } else {
        pll_out = params_rate(params).wrapping_mul(256);
    }

    ret = snd_soc_dai_set_pll(codec_dai, WM8994_FLL1, WM8994_FLL_SRC_MCLK1, ARIES_MCLK1_FREQ, pll_out);
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, WM8994_SYSCLK_FLL1, pll_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn aries_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_dai;
    let mut ret: c_int;

    /* Switch sysclk to MCLK1 */
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8994_SYSCLK_MCLK1, ARIES_MCLK1_FREQ, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    /* Stop PLL */
    ret = snd_soc_dai_set_pll(codec_dai, WM8994_FLL1, WM8994_FLL_SRC_MCLK1, ARIES_MCLK1_FREQ, 0);
    if ret < 0 {
        return ret;
    }

    0
}

/*
 * Main DAI operations
 */
static aries_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(aries_hw_params),
    hw_free: Some(aries_hw_free),
};

unsafe extern "C" fn aries_baseband_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0) as *mut snd_soc_dai;
    let pll_out: c_uint = 8000 * 512;
    let mut ret: c_int;

    /* Set the codec FLL */
    ret = snd_soc_dai_set_pll(codec_dai, WM8994_FLL2, WM8994_FLL_SRC_MCLK1, ARIES_MCLK1_FREQ, pll_out);
    if ret < 0 {
        return ret;
    }

    /* Set the codec system clock */
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8994_SYSCLK_FLL2, pll_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn aries_late_probe(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut aries_wm8994_data;
    let mut ret: c_int;
    let irq: c_int;

    ret = snd_soc_card_jack_new_pins(card, c"Dock".as_ptr(), SND_JACK_LINEOUT, &raw mut aries_dock, dock_pins.as_mut_ptr(), dock_pins.len() as c_uint);
    if ret != 0 {
        return ret;
    }

    ret = devm_extcon_register_notifier((*card).dev, (*priv_).usb_extcon, EXTCON_JACK_LINE_OUT, &raw mut aries_extcon_notifier_block);
    if ret != 0 {
        return ret;
    }

    if extcon_get_state((*priv_).usb_extcon, EXTCON_JACK_LINE_OUT) > 0 {
        snd_soc_jack_report(&raw mut aries_dock, SND_JACK_LINEOUT, SND_JACK_LINEOUT);
    } else {
        snd_soc_jack_report(&raw mut aries_dock, 0, SND_JACK_LINEOUT);
    }

    ret = snd_soc_card_jack_new_pins(card, c"Headset".as_ptr(), SND_JACK_HEADSET | SND_JACK_BTN_0, &raw mut aries_headset, jack_pins.as_mut_ptr(), jack_pins.len() as c_uint);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_jack_add_zones(&raw mut aries_headset, headset_zones.len() as c_int, headset_zones.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    irq = gpiod_to_irq((*priv_).gpio_headset_detect);
    if irq < 0 {
        dev_err((*card).dev, c"Failed to map headset detect gpio to irq".as_ptr());
        return -EINVAL;
    }

    ret = devm_request_threaded_irq(
        (*card).dev,
        irq,
        ptr::null(),
        Some(headset_det_irq_thread),
        IRQF_TRIGGER_RISING | IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
        c"headset_detect".as_ptr(),
        priv_ as *mut c_void,
    );
    if ret != 0 {
        dev_err((*card).dev, c"Failed to request headset detect irq".as_ptr());
        return ret;
    }

    headset_button_gpio[0].data = priv_ as *mut c_void;
    headset_button_gpio[0].desc = (*priv_).gpio_headset_key;

    snd_jack_set_key(aries_headset.jack, SND_JACK_BTN_0, KEY_MEDIA);

    snd_soc_jack_add_gpios(&raw mut aries_headset, headset_button_gpio.len() as c_int, headset_button_gpio.as_mut_ptr())
}

static baseband_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    stream_name: ptr::null(),
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rate_min: 8000,
    rate_max: 8000,
    channels_min: 1,
    channels_max: 1,
    rates: 0,
};

static bluetooth_params: snd_soc_pcm_stream = snd_soc_pcm_stream {
    stream_name: ptr::null(),
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    rate_min: 8000,
    rate_max: 8000,
    channels_min: 1,
    channels_max: 2,
    rates: 0,
};

static aries_modem_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_INPUT("Modem RX") */
    snd_soc_dapm_widget { dapm: ptr::null_mut() }, /* SND_SOC_DAPM_OUTPUT("Modem TX") */
];

static aries_modem_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Modem Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Modem RX\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Modem TX\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Modem Playback\0".as_ptr() as *const c_char },
];

static aries_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"aries-audio\0".as_ptr() as *const c_char,
    dapm_widgets: aries_modem_widgets.as_ptr(),
    num_dapm_widgets: aries_modem_widgets.len() as c_uint,
    dapm_routes: aries_modem_routes.as_ptr(),
    num_dapm_routes: aries_modem_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut aries_ext_dai: [snd_soc_dai_driver; 1] = [
    snd_soc_dai_driver {
        name: b"Voice call\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Modem Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 1,
            rate_min: 8000,
            rate_max: 8000,
            rates: SNDRV_PCM_RATE_8000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Modem Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 1,
            rate_min: 8000,
            rate_max: 8000,
            rates: SNDRV_PCM_RATE_8000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
    },
];

/* SND_SOC_DAILINK_DEFS(aif1/baseband/bluetooth, ...) */
static mut aif1_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut aif1_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut aif1_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut baseband_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut baseband_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut bluetooth_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];
static mut bluetooth_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { of_node: ptr::null_mut() }];

static mut aries_dai: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: b"WM8994 AIF1\0".as_ptr() as *const c_char,
        stream_name: b"HiFi\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP,
        ops: &aries_ops,
        init: None,
        c2c_params: ptr::null(),
        num_c2c_params: 0,
        ignore_suspend: 0,
        cpus: aif1_cpus.as_mut_ptr(),
        codecs: aif1_codecs.as_mut_ptr(),
        platforms: aif1_platforms.as_mut_ptr(),
    },
    snd_soc_dai_link {
        name: b"WM8994 AIF2\0".as_ptr() as *const c_char,
        stream_name: b"Baseband\0".as_ptr() as *const c_char,
        dai_fmt: 0,
        ops: ptr::null(),
        init: Some(aries_baseband_init),
        c2c_params: &baseband_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
        cpus: baseband_cpus.as_mut_ptr(),
        codecs: baseband_codecs.as_mut_ptr(),
        platforms: ptr::null_mut(),
    },
    snd_soc_dai_link {
        name: b"WM8994 AIF3\0".as_ptr() as *const c_char,
        stream_name: b"Bluetooth\0".as_ptr() as *const c_char,
        dai_fmt: 0,
        ops: ptr::null(),
        init: None,
        c2c_params: &bluetooth_params,
        num_c2c_params: 1,
        ignore_suspend: 1,
        cpus: bluetooth_cpus.as_mut_ptr(),
        codecs: bluetooth_codecs.as_mut_ptr(),
        platforms: ptr::null_mut(),
    },
];

static mut aries_card: snd_soc_card = snd_soc_card {
    name: b"ARIES\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dev: ptr::null_mut(),
    dai_link: aries_dai.as_mut_ptr(),
    num_links: aries_dai.len() as c_uint,
    controls: aries_controls.as_ptr(),
    num_controls: aries_controls.len() as c_uint,
    dapm_widgets: aries_dapm_widgets.as_ptr(),
    num_dapm_widgets: aries_dapm_widgets.len() as c_uint,
    late_probe: Some(aries_late_probe),
};

static fascinate4g_variant: aries_wm8994_variant = aries_wm8994_variant {
    modem_dai_fmt: SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_IB_NF,
    has_fm_radio: false,
};

static aries_variant: aries_wm8994_variant = aries_wm8994_variant {
    modem_dai_fmt: SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_CBP_CFP | SND_SOC_DAIFMT_IB_NF,
    has_fm_radio: true,
};

static samsung_wm8994_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"samsung,fascinate4g-wm8994\0".as_ptr() as *const c_char,
        data: &fascinate4g_variant as *const _ as *const c_void,
    },
    of_device_id {
        compatible: b"samsung,aries-wm8994\0".as_ptr() as *const c_char,
        data: &aries_variant as *const _ as *const c_void,
    },
    of_device_id { compatible: ptr::null(), data: ptr::null() }, /* sentinel */
];
/* MODULE_DEVICE_TABLE(of, samsung_wm8994_of_match); */

unsafe extern "C" fn aries_audio_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut cpu: *mut device_node;
    let mut codec: *mut device_node = ptr::null_mut();
    let extcon_np: *mut device_node;
    let dev = &raw mut (*pdev).dev;
    let card = &raw mut aries_card;
    let priv_: *mut aries_wm8994_data;
    let mut dai_link: *mut snd_soc_dai_link;
    let match_: *const of_device_id;
    let mut channel_type: iio_chan_type = 0;
    let mut ret: c_int;

    if np.is_null() {
        return -EINVAL;
    }

    (*card).dev = dev;
    (*card).owner = THIS_MODULE;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<aries_wm8994_data>(), GFP_KERNEL) as *mut aries_wm8994_data;
    if priv_.is_null() {
        return -ENOMEM;
    }

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    match_ = of_match_node(samsung_wm8994_of_match.as_ptr(), np);
    (*priv_).variant = (*match_).data as *const aries_wm8994_variant;

    /* Remove FM widget if not present */
    if !(*(*priv_).variant).has_fm_radio {
        (*card).num_dapm_widgets -= 1;
    }

    (*priv_).reg_main_micbias = devm_regulator_get(dev, c"main-micbias".as_ptr());
    if IS_ERR((*priv_).reg_main_micbias) {
        dev_err(dev, c"Failed to get main micbias regulator\n".as_ptr());
        return PTR_ERR((*priv_).reg_main_micbias);
    }

    (*priv_).reg_headset_micbias = devm_regulator_get(dev, c"headset-micbias".as_ptr());
    if IS_ERR((*priv_).reg_headset_micbias) {
        dev_err(dev, c"Failed to get headset micbias regulator\n".as_ptr());
        return PTR_ERR((*priv_).reg_headset_micbias);
    }

    (*priv_).gpio_earpath_sel = devm_gpiod_get(dev, c"earpath-sel".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).gpio_earpath_sel) {
        dev_err(dev, c"Failed to get earpath selector gpio".as_ptr());
        return PTR_ERR((*priv_).gpio_earpath_sel);
    }

    extcon_np = of_parse_phandle(np, c"extcon".as_ptr(), 0);
    (*priv_).usb_extcon = extcon_find_edev_by_node(extcon_np);
    of_node_put(extcon_np);
    if IS_ERR((*priv_).usb_extcon) {
        return dev_err_probe(dev, PTR_ERR((*priv_).usb_extcon), c"Failed to get extcon device".as_ptr());
    }

    (*priv_).adc = devm_iio_channel_get(dev, c"headset-detect".as_ptr());
    if IS_ERR((*priv_).adc) {
        return dev_err_probe(dev, PTR_ERR((*priv_).adc), c"Failed to get ADC channel".as_ptr());
    }

    ret = iio_get_channel_type((*priv_).adc, &mut channel_type);
    if ret != 0 {
        return dev_err_probe(dev, ret, c"Failed to get ADC channel type".as_ptr());
    }
    if channel_type != IIO_VOLTAGE {
        return -EINVAL;
    }

    (*priv_).gpio_headset_key = devm_gpiod_get(dev, c"headset-key".as_ptr(), GPIOD_IN);
    if IS_ERR((*priv_).gpio_headset_key) {
        dev_err(dev, c"Failed to get headset key gpio".as_ptr());
        return PTR_ERR((*priv_).gpio_headset_key);
    }

    (*priv_).gpio_headset_detect = devm_gpiod_get(dev, c"headset-detect".as_ptr(), GPIOD_IN);
    if IS_ERR((*priv_).gpio_headset_detect) {
        dev_err(dev, c"Failed to get headset detect gpio".as_ptr());
        return PTR_ERR((*priv_).gpio_headset_detect);
    }

    /* Update card-name if provided through DT, else use default name */
    snd_soc_of_parse_card_name(card, c"model".as_ptr());

    ret = snd_soc_of_parse_audio_routing(card, c"audio-routing".as_ptr());
    if ret < 0 {
        /* Backwards compatible way */
        ret = snd_soc_of_parse_audio_routing(card, c"samsung,audio-routing".as_ptr());
        if ret < 0 {
            dev_err(dev, c"Audio routing invalid/unspecified\n".as_ptr());
            return ret;
        }
    }

    aries_dai[1].dai_fmt = (*(*priv_).variant).modem_dai_fmt;

    cpu = of_get_child_by_name((*dev).of_node, c"cpu".as_ptr());
    if cpu.is_null() {
        return -EINVAL;
    }

    codec = of_get_child_by_name((*dev).of_node, c"codec".as_ptr());
    if codec.is_null() {
        ret = -EINVAL;
        goto_out(cpu, codec);
        return ret;
    }

    let mut i: usize = 0;
    while i < (*card).num_links as usize {
        dai_link = (*card).dai_link.add(i);
        (*(*dai_link).codecs).of_node = of_parse_phandle(codec, c"sound-dai".as_ptr(), 0);
        if (*(*dai_link).codecs).of_node.is_null() {
            ret = -EINVAL;
            goto_out(cpu, codec);
            return ret;
        }
        i += 1;
    }

    /* Set CPU and platform of_node for main DAI */
    (*aries_dai[0].cpus).of_node = of_parse_phandle(cpu, c"sound-dai".as_ptr(), 0);
    if (*aries_dai[0].cpus).of_node.is_null() {
        ret = -EINVAL;
        goto_out(cpu, codec);
        return ret;
    }

    of_node_get((*aries_dai[0].cpus).of_node);
    (*aries_dai[0].platforms).of_node = (*aries_dai[0].cpus).of_node;

    /* Set CPU of_node for BT DAI */
    (*aries_dai[2].cpus).of_node = of_parse_phandle(cpu, c"sound-dai".as_ptr(), 1);
    if (*aries_dai[2].cpus).of_node.is_null() {
        ret = -EINVAL;
        goto_out(cpu, codec);
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &aries_component, aries_ext_dai.as_mut_ptr(), aries_ext_dai.len() as c_int);
    if ret < 0 {
        dev_err(dev, c"Failed to register component: %d\n".as_ptr(), ret);
        goto_out(cpu, codec);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        dev_err(dev, c"snd_soc_register_card() failed:%d\n".as_ptr(), ret);
    }

    goto_out(cpu, codec);
    ret
}

unsafe fn goto_out(cpu: *mut device_node, codec: *mut device_node) {
    of_node_put(cpu);
    of_node_put(codec);
}

static mut aries_audio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"aries-audio-wm8994\0".as_ptr() as *const c_char,
        of_match_table: samsung_wm8994_of_match.as_ptr(),
        pm: &snd_soc_pm_ops as *const _ as *const c_void,
    },
    probe: Some(aries_audio_probe),
};

/* module_platform_driver(aries_audio_driver); */
/* MODULE_DESCRIPTION("ALSA SoC ARIES WM8994"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:aries-audio-wm8994"); */
/* MODULE_IMPORT_NS("IIO_CONSUMER"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
