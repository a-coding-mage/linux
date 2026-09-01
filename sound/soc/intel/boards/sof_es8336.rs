// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation.

/*
 * Intel SOF Machine Driver with es8336 Codec
 *
 * Dependencies originally supplied by Linux and local headers:
 * linux/device.h, linux/dmi.h, linux/gpio/consumer.h, linux/gpio/machine.h,
 * linux/i2c.h, linux/input.h, linux/module.h, linux/platform_device.h,
 * linux/slab.h, sound/jack.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
 * sound/soc-acpi.h, hda_dsp_common.h.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

type bool_ = bool;
type kernel_ulong_t = c_ulong;

const fn BIT(n: c_uint) -> c_ulong {
    1_c_ulong << n
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_ulong {
    if h >= (usize::BITS - 1) {
        (!0_c_ulong) << l
    } else {
        (((1_c_ulong << (h + 1)) - 1) & (!((1_c_ulong << l) - 1)))
    }
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

/* jd-inv + terminating entry */
const MAX_NO_PROPS: usize = 2;

const fn SOF_ES8336_SSP_CODEC(quirk: c_ulong) -> c_ulong {
    quirk & GENMASK(3, 0)
}
const SOF_ES8336_SSP_CODEC_MASK: c_ulong = GENMASK(3, 0);

const SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK: c_ulong = BIT(4);

/* HDMI capture*/
const SOF_SSP_HDMI_CAPTURE_PRESENT: c_ulong = BIT(14);
const SOF_NO_OF_HDMI_CAPTURE_SSP_SHIFT: c_uint = 15;
const SOF_NO_OF_HDMI_CAPTURE_SSP_MASK: c_ulong = GENMASK(16, 15);
const fn SOF_NO_OF_HDMI_CAPTURE_SSP(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_NO_OF_HDMI_CAPTURE_SSP_SHIFT) & SOF_NO_OF_HDMI_CAPTURE_SSP_MASK
}

const SOF_HDMI_CAPTURE_1_SSP_SHIFT: c_uint = 7;
const SOF_HDMI_CAPTURE_1_SSP_MASK: c_ulong = GENMASK(9, 7);
const fn SOF_HDMI_CAPTURE_1_SSP(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_HDMI_CAPTURE_1_SSP_SHIFT) & SOF_HDMI_CAPTURE_1_SSP_MASK
}

const SOF_HDMI_CAPTURE_2_SSP_SHIFT: c_uint = 10;
const SOF_HDMI_CAPTURE_2_SSP_MASK: c_ulong = GENMASK(12, 10);
const fn SOF_HDMI_CAPTURE_2_SSP(quirk: c_ulong) -> c_ulong {
    (quirk << SOF_HDMI_CAPTURE_2_SSP_SHIFT) & SOF_HDMI_CAPTURE_2_SSP_MASK
}

const SOF_ES8336_ENABLE_DMIC: c_ulong = BIT(5);
const SOF_ES8336_JD_INVERTED: c_ulong = BIT(6);
const SOF_ES8336_HEADPHONE_GPIO: c_ulong = BIT(7);
const SOC_ES8336_HEADSET_MIC1: c_ulong = BIT(8);

static mut quirk: c_ulong = 0;

static mut quirk_override: c_int = -1;
/* module_param_named(quirk, quirk_override, int, 0444); */
/* MODULE_PARM_DESC(quirk, "Board-specific quirk override"); */

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}
#[repr(C)]
struct snd_jack {
    _private: [u8; 0],
}
#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}
#[repr(C)]
struct delayed_work {
    work: work_struct,
}
#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_substream {
    stream: c_int,
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    owner: *mut c_void,
    dev: *mut device,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_int,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_int,
    fully_routed: bool_,
    late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    num_links: c_int,
    dai_link: *mut snd_soc_dai_link,
    components: *mut c_char,
}
#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}
#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_int,
}
#[repr(C)]
struct dmi_system_id {
    callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    matches: [dmi_strmatch; 4],
    driver_data: *mut c_void,
}
#[repr(C)]
struct dmi_strmatch {
    slot: c_int,
    substr: *const c_char,
}
#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
}
#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}
#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    id: c_int,
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_int,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_int,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    ops: *const snd_soc_ops,
    nonatomic: bool_,
    no_pcm: c_int,
    cpus: *mut snd_soc_dai_link_component,
    num_cpus: c_int,
    ignore_suspend: c_int,
    capture_only: c_int,
    playback_only: c_int,
}
#[repr(C)]
struct acpi_gpio_params {
    crs_entry_index: c_uint,
    line_index: c_uint,
    active_low: bool_,
}
#[repr(C)]
struct acpi_gpio_mapping {
    name: *const c_char,
    data: *const acpi_gpio_params,
    size: c_uint,
    quirks: c_uint,
}
#[repr(C)]
struct platform_device {
    dev: device_with_data,
    id_entry: *const platform_device_id,
}
#[repr(C)]
struct device_with_data {
    platform_data: *mut c_void,
}
#[repr(C)]
struct platform_device_id {
    name: [c_char; 32],
    driver_data: kernel_ulong_t,
}
#[repr(C)]
struct snd_soc_acpi_mach {
    id: *const c_char,
    tplg_quirk_mask: c_ulong,
    mach_params: snd_soc_acpi_mach_params,
}
#[repr(C)]
struct snd_soc_acpi_mach_params {
    i2s_link_mask: c_ulong,
    dmic_num: c_int,
    platform: *const c_char,
}
#[repr(C)]
struct property_entry {
    _private: [u8; 0],
}
#[repr(C)]
struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
struct acpi_device {
    _private: [u8; 0],
}
#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    id_table: *const platform_device_id,
}
#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const c_void,
}

#[repr(C)]
struct sof_es8336_private {
    codec_dev: *mut device,
    gpio_speakers: *mut gpio_desc,
    gpio_headphone: *mut gpio_desc,
    jack: snd_soc_jack,
    hdmi_pcm_list: list_head,
    speaker_en: bool_,
    pcm_pop_work: delayed_work,
}

#[repr(C)]
struct sof_hdmi_pcm {
    head: list_head,
    codec_dai: *mut snd_soc_dai,
    device: c_int,
}

unsafe extern "C" {
    static mut system_dfl_wq: *mut c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static mut snd_soc_pm_ops: c_void;
    static mut THIS_MODULE: *mut c_void;

    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;
    fn gpiod_put(desc: *mut gpio_desc);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn cancel_delayed_work(work: *mut delayed_work) -> bool_;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn queue_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_ulong) -> bool_;
    fn msecs_to_jiffies(m: c_uint) -> c_ulong;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool_;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias_on: bool_);
    fn snd_soc_card_jack_new_pins(card: *mut snd_soc_card, id: *const c_char, type_: c_int, jack: *mut snd_soc_jack, pins: *mut snd_soc_jack_pin, num_pins: c_uint) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(component: *mut snd_soc_component, jack: *mut snd_soc_jack, data: *mut c_void) -> c_int;
    fn dmi_check_system(list: *const dmi_system_id) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn hda_dsp_hdmi_build_controls(card: *mut snd_soc_card, component: *mut snd_soc_component) -> c_int;
    fn fls(x: c_ulong) -> c_int;
    fn acpi_dev_get_first_match_dev(hid: *const c_char, uid: *const c_char, hrv: c_int) -> *mut acpi_device;
    fn acpi_dev_name(adev: *mut acpi_device) -> *const c_char;
    fn acpi_get_first_physical_node(adev: *mut acpi_device) -> *mut device;
    fn acpi_dev_put(adev: *mut acpi_device);
    fn get_device(dev: *mut device) -> *mut device;
    fn put_device(dev: *mut device);
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform: *const c_char) -> c_int;
    fn fwnode_create_software_node(props: *const property_entry, parent: *const fwnode_handle) -> *mut fwnode_handle;
    fn to_software_node(fwnode: *mut fwnode_handle) -> *mut c_void;
    fn device_add_software_node(dev: *mut device, swnode: *mut c_void) -> c_int;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn device_remove_software_node(dev: *mut device);
    fn devm_acpi_dev_add_driver_gpios(dev: *mut device, mapping: *const acpi_gpio_mapping) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const EPROBE_DEFER: c_int = 517;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const KEY_PLAYPAUSE: c_int = 164;
const SND_ACPI_I2C_ID_LEN: usize = 9;
const SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER: c_ulong = 1;
const GPIOD_OUT_LOW: c_int = 0;
const ACPI_GPIO_QUIRK_ONLY_GPIOIO: c_uint = 1;
const DMI_SYS_VENDOR: c_int = 1;
const DMI_PRODUCT_NAME: c_int = 2;
const DMI_BOARD_NAME: c_int = 3;

const fn cstr<const N: usize>(s: &[u8; N]) -> *const c_char {
    s.as_ptr() as *const c_char
}

static enable_gpio0: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: true };
static enable_gpio1: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 1, line_index: 0, active_low: true };

static acpi_speakers_enable_gpio0: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping { name: cstr(b"speakers-enable-gpios\0"), data: &enable_gpio0, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping { name: null(), data: null(), size: 0, quirks: 0 },
];

static acpi_speakers_enable_gpio1: [acpi_gpio_mapping; 1] = [
    acpi_gpio_mapping { name: cstr(b"speakers-enable-gpios\0"), data: &enable_gpio1, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
];

static acpi_enable_both_gpios: [acpi_gpio_mapping; 3] = [
    acpi_gpio_mapping { name: cstr(b"speakers-enable-gpios\0"), data: &enable_gpio0, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping { name: cstr(b"headphone-enable-gpios\0"), data: &enable_gpio1, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping { name: null(), data: null(), size: 0, quirks: 0 },
];

static acpi_enable_both_gpios_rev_order: [acpi_gpio_mapping; 3] = [
    acpi_gpio_mapping { name: cstr(b"speakers-enable-gpios\0"), data: &enable_gpio1, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping { name: cstr(b"headphone-enable-gpios\0"), data: &enable_gpio0, size: 1, quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO },
    acpi_gpio_mapping { name: null(), data: null(), size: 0, quirks: 0 },
];

unsafe extern "C" fn log_quirks(dev: *mut device) {
    dev_info(dev, cstr(b"quirk mask %#lx\n\0"), quirk);
    dev_info(dev, cstr(b"quirk SSP%ld\n\0"), SOF_ES8336_SSP_CODEC(quirk));
    if quirk & SOF_ES8336_ENABLE_DMIC != 0 {
        dev_info(dev, cstr(b"quirk DMIC enabled\n\0"));
    }
    if quirk & SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK != 0 {
        dev_info(dev, cstr(b"Speakers GPIO1 quirk enabled\n\0"));
    }
    if quirk & SOF_ES8336_HEADPHONE_GPIO != 0 {
        dev_info(dev, cstr(b"quirk headphone GPIO enabled\n\0"));
    }
    if quirk & SOF_ES8336_JD_INVERTED != 0 {
        dev_info(dev, cstr(b"quirk JD inverted enabled\n\0"));
    }
    if quirk & SOC_ES8336_HEADSET_MIC1 != 0 {
        dev_info(dev, cstr(b"quirk headset at mic1 port enabled\n\0"));
    }
}

unsafe extern "C" fn pcm_pop_work_events(work: *mut work_struct) {
    let priv_ = (work as *mut u8).offset(-(0 as isize)) as *mut sof_es8336_private;

    gpiod_set_value_cansleep((*priv_).gpio_speakers, (*priv_).speaker_en as c_int);

    if quirk & SOF_ES8336_HEADPHONE_GPIO != 0 {
        gpiod_set_value_cansleep((*priv_).gpio_headphone, (!(*priv_).speaker_en) as c_int);
    }
}

unsafe extern "C" fn sof_8336_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut sof_es8336_private;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE | SNDRV_PCM_TRIGGER_RESUME => {}
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            if (*priv_).speaker_en == false {
                if (*substream).stream == 0 {
                    cancel_delayed_work(addr_of_mut!((*priv_).pcm_pop_work));
                    gpiod_set_value_cansleep((*priv_).gpio_speakers, true as c_int);
                }
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn sof_es8316_speaker_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card(w as *mut snd_soc_dapm_context);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut sof_es8336_private;

    if (*priv_).speaker_en == !SND_SOC_DAPM_EVENT_ON(event) {
        return 0;
    }

    (*priv_).speaker_en = !SND_SOC_DAPM_EVENT_ON(event);

    queue_delayed_work(system_dfl_wq, addr_of_mut!((*priv_).pcm_pop_work), msecs_to_jiffies(70));
    0
}

/* SND_SOC_DAPM_* and SOC_DAPM_PIN_SWITCH initializers are external macro data in C. */
static sof_es8316_widgets: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static dmic_widgets: [snd_soc_dapm_widget; 1] = [
    snd_soc_dapm_widget { _private: [] },
];

static sof_es8316_audio_map: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: cstr(b"Headphone\0"), control: null(), source: cstr(b"HPOL\0") },
    snd_soc_dapm_route { sink: cstr(b"Headphone\0"), control: null(), source: cstr(b"HPOR\0") },
    /*
     * There is no separate speaker output instead the speakers are muxed to
     * the HP outputs. The mux is controlled Speaker and/or headphone switch.
     */
    snd_soc_dapm_route { sink: cstr(b"Speaker\0"), control: null(), source: cstr(b"HPOL\0") },
    snd_soc_dapm_route { sink: cstr(b"Speaker\0"), control: null(), source: cstr(b"HPOR\0") },
    snd_soc_dapm_route { sink: cstr(b"Speaker\0"), control: null(), source: cstr(b"Speaker Power\0") },
];

static sof_es8316_headset_mic2_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: cstr(b"MIC1\0"), control: null(), source: cstr(b"Internal Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"MIC2\0"), control: null(), source: cstr(b"Headset Mic\0") },
];

static sof_es8316_headset_mic1_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: cstr(b"MIC2\0"), control: null(), source: cstr(b"Internal Mic\0") },
    snd_soc_dapm_route { sink: cstr(b"MIC1\0"), control: null(), source: cstr(b"Headset Mic\0") },
];

static dmic_map: [snd_soc_dapm_route; 1] = [
    /* digital mics */
    snd_soc_dapm_route { sink: cstr(b"DMic\0"), control: null(), source: cstr(b"SoC DMIC\0") },
];

static sof_es8316_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

static mut sof_es8316_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: cstr(b"Headphone\0"), mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: cstr(b"Headset Mic\0"), mask: SND_JACK_MICROPHONE },
];

unsafe extern "C" fn dmic_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, sof_es8316_widgets.as_ptr().add(0).cast(), ARRAY_SIZE(&dmic_widgets) as c_int);
    if ret != 0 {
        dev_err((*card).dev, cstr(b"DMic widget addition failed: %d\n\0"), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, dmic_map.as_ptr(), ARRAY_SIZE(&dmic_map) as c_int);
    if ret != 0 {
        dev_err((*card).dev, cstr(b"DMic map addition failed: %d\n\0"), ret);
    }

    ret
}

unsafe extern "C" fn sof_hdmi_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let priv_ = snd_soc_card_get_drvdata((*runtime).card) as *mut sof_es8336_private;
    let dai = snd_soc_rtd_to_codec(runtime, 0);
    let pcm: *mut sof_hdmi_pcm;

    pcm = devm_kzalloc((*(*runtime).card).dev, size_of::<sof_hdmi_pcm>(), GFP_KERNEL) as *mut sof_hdmi_pcm;
    if pcm.is_null() {
        return -ENOMEM;
    }

    /* dai_link id is 1:1 mapped to the PCM device */
    (*pcm).device = (*(*runtime).dai_link).id;
    (*pcm).codec_dai = dai;

    list_add_tail(addr_of_mut!((*pcm).head), addr_of_mut!((*priv_).hdmi_pcm_list));

    0
}

unsafe extern "C" fn sof_es8316_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut sof_es8336_private;
    let custom_map: *const snd_soc_dapm_route;
    let num_routes: c_int;
    let mut ret: c_int;

    snd_soc_dapm_set_idle_bias(dapm, false);

    if quirk & SOC_ES8336_HEADSET_MIC1 != 0 {
        custom_map = sof_es8316_headset_mic1_map.as_ptr();
        num_routes = ARRAY_SIZE(&sof_es8316_headset_mic1_map) as c_int;
    } else {
        custom_map = sof_es8316_headset_mic2_map.as_ptr();
        num_routes = ARRAY_SIZE(&sof_es8316_headset_mic2_map) as c_int;
    }

    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        cstr(b"Headset\0"),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        addr_of_mut!((*priv_).jack),
        sof_es8316_jack_pins.as_mut_ptr(),
        ARRAY_SIZE(&sof_es8316_jack_pins) as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, cstr(b"jack creation failed %d\n\0"), ret);
        return ret;
    }

    snd_jack_set_key((*priv_).jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);

    snd_soc_component_set_jack(codec, addr_of_mut!((*priv_).jack), null_mut());

    0
}

unsafe extern "C" fn sof_es8316_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, null_mut(), null_mut());
}

unsafe extern "C" fn sof_es8336_quirk_cb(id: *const dmi_system_id) -> c_int {
    quirk = (*id).driver_data as c_ulong;

    1
}

/*
 * this table should only be used to add GPIO or jack-detection quirks
 * that cannot be detected from ACPI tables. The SSP and DMIC
 * information are providing by the platform driver and are aligned
 * with the topology used.
 *
 * If the GPIO support is missing, the quirk parameter can be used to
 * enable speakers. In that case it's recommended to keep the SSP and DMIC
 * information consistent, overriding the SSP and DMIC can only be done
 * if the topology file is modified as well.
 */
static sof_es8336_quirk_table: [dmi_system_id; 4] = [
    dmi_system_id {
        callback: Some(sof_es8336_quirk_cb),
        matches: [
            dmi_strmatch { slot: DMI_SYS_VENDOR, substr: cstr(b"HUAWEI\0") },
            dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: cstr(b"BOD-WXX9\0") },
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
        driver_data: (SOF_ES8336_HEADPHONE_GPIO | SOF_ES8336_ENABLE_DMIC) as *mut c_void,
    },
    dmi_system_id {
        callback: Some(sof_es8336_quirk_cb),
        matches: [
            dmi_strmatch { slot: DMI_SYS_VENDOR, substr: cstr(b"IP3 tech\0") },
            dmi_strmatch { slot: DMI_BOARD_NAME, substr: cstr(b"WN1\0") },
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
        driver_data: SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK as *mut c_void,
    },
    dmi_system_id {
        callback: Some(sof_es8336_quirk_cb),
        matches: [
            dmi_strmatch { slot: DMI_SYS_VENDOR, substr: cstr(b"HUAWEI\0") },
            dmi_strmatch { slot: DMI_BOARD_NAME, substr: cstr(b"BOHB-WAX9-PCB-B2\0") },
            dmi_strmatch { slot: 0, substr: null() },
            dmi_strmatch { slot: 0, substr: null() },
        ],
        driver_data: (SOF_ES8336_HEADPHONE_GPIO | SOC_ES8336_HEADSET_MIC1) as *mut c_void,
    },
    dmi_system_id { callback: None, matches: [dmi_strmatch { slot: 0, substr: null() }, dmi_strmatch { slot: 0, substr: null() }, dmi_strmatch { slot: 0, substr: null() }, dmi_strmatch { slot: 0, substr: null() }], driver_data: null_mut() },
];

unsafe extern "C" fn sof_es8336_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let sysclk: c_int = 19200000;
    let ret: c_int;

    ret = snd_soc_dai_set_sysclk(codec_dai, 1, sysclk as c_uint, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err((*rtd).dev, cstr(b"%s, Failed to set ES8336 SYSCLK: %d\n\0"), cstr(b"sof_es8336_hw_params\0"), ret);
        return ret;
    }

    0
}

/* machine stream operations */
static sof_es8336_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sof_es8336_hw_params),
    trigger: Some(sof_8336_trigger),
};

static mut platform_component: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component {
        /* name might be overridden during probe */
        name: cstr(b"0000:00:1f.3\0"),
        dai_name: null(),
    },
];

static mut es8336_codec: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: cstr(b"i2c-ESSX8336:00\0"), dai_name: cstr(b"ES8316 HiFi\0") },
];

static mut dmic_component: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: cstr(b"dmic-codec\0"), dai_name: cstr(b"dmic-hifi\0") },
];

unsafe extern "C" fn sof_es8336_late_probe(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut sof_es8336_private;
    let pcm: *mut sof_hdmi_pcm;

    if list_empty(addr_of_mut!((*priv_).hdmi_pcm_list)) != 0 {
        return -ENOENT;
    }

    pcm = (*priv_).hdmi_pcm_list.next as *mut sof_hdmi_pcm;

    hda_dsp_hdmi_build_controls(card, (*(*pcm).codec_dai).component)
}

/* SoC card */
static mut sof_es8336_card: snd_soc_card = snd_soc_card {
    name: cstr(b"essx8336\0"), /* sof- prefix added automatically */
    owner: null_mut(),
    dev: null_mut(),
    dapm_widgets: sof_es8316_widgets.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: sof_es8316_audio_map.as_ptr(),
    num_dapm_routes: 5,
    controls: sof_es8316_controls.as_ptr(),
    num_controls: 4,
    fully_routed: true,
    late_probe: Some(sof_es8336_late_probe),
    num_links: 1,
    dai_link: null_mut(),
    components: null_mut(),
};

unsafe extern "C" fn sof_card_dai_links_create(
    dev: *mut device,
    ssp_codec: c_int,
    mut dmic_be_num: c_int,
    hdmi_num: c_int,
) -> *mut snd_soc_dai_link {
    let cpus: *mut snd_soc_dai_link_component;
    let links: *mut snd_soc_dai_link;
    let mut idisp_components: *mut snd_soc_dai_link_component = null_mut();
    let mut hdmi_id_offset: c_int = 0;
    let mut id: c_int = 0;
    let mut i: c_int;

    links = devm_kcalloc(dev, sof_es8336_card.num_links as usize, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    cpus = devm_kcalloc(dev, sof_es8336_card.num_links as usize, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
    if links.is_null() || cpus.is_null() {
        return null_mut();
    }

    /* codec SSP */
    (*links.add(id as usize)).name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"SSP%d-Codec\0"), ssp_codec);
    if (*links.add(id as usize)).name.is_null() {
        return null_mut();
    }

    (*links.add(id as usize)).id = id;
    (*links.add(id as usize)).codecs = es8336_codec.as_mut_ptr();
    (*links.add(id as usize)).num_codecs = ARRAY_SIZE(&es8336_codec) as c_int;
    (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
    (*links.add(id as usize)).num_platforms = ARRAY_SIZE(&platform_component) as c_int;
    (*links.add(id as usize)).init = Some(sof_es8316_init);
    (*links.add(id as usize)).exit = Some(sof_es8316_exit);
    (*links.add(id as usize)).ops = &sof_es8336_ops;
    (*links.add(id as usize)).nonatomic = true;
    (*links.add(id as usize)).no_pcm = 1;
    (*links.add(id as usize)).cpus = cpus.add(id as usize);
    (*links.add(id as usize)).num_cpus = 1;

    (*(*links.add(id as usize)).cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"SSP%d Pin\0"), ssp_codec);
    if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
        return null_mut();
    }

    id += 1;

    /* dmic */
    if dmic_be_num > 0 {
        /* at least we have dmic01 */
        (*links.add(id as usize)).name = cstr(b"dmic01\0");
        (*links.add(id as usize)).cpus = cpus.add(id as usize);
        (*(*links.add(id as usize)).cpus).dai_name = cstr(b"DMIC01 Pin\0");
        (*links.add(id as usize)).init = Some(dmic_init);
        if dmic_be_num > 1 {
            /* set up 2 BE links at most */
            (*links.add((id + 1) as usize)).name = cstr(b"dmic16k\0");
            (*links.add((id + 1) as usize)).cpus = cpus.add((id + 1) as usize);
            (*(*links.add((id + 1) as usize)).cpus).dai_name = cstr(b"DMIC16k Pin\0");
            dmic_be_num = 2;
        }
    } else {
        /* HDMI dai link starts at 3 according to current topology settings */
        hdmi_id_offset = 2;
    }

    i = 0;
    while i < dmic_be_num {
        (*links.add(id as usize)).id = id;
        (*links.add(id as usize)).num_cpus = 1;
        (*links.add(id as usize)).codecs = dmic_component.as_mut_ptr();
        (*links.add(id as usize)).num_codecs = ARRAY_SIZE(&dmic_component) as c_int;
        (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
        (*links.add(id as usize)).num_platforms = ARRAY_SIZE(&platform_component) as c_int;
        (*links.add(id as usize)).ignore_suspend = 1;
        (*links.add(id as usize)).capture_only = 1;
        (*links.add(id as usize)).no_pcm = 1;

        id += 1;
        i += 1;
    }

    /* HDMI */
    if hdmi_num > 0 {
        idisp_components = devm_kcalloc(dev, hdmi_num as usize, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL) as *mut snd_soc_dai_link_component;
        if idisp_components.is_null() {
            return null_mut();
        }
    }

    i = 1;
    while i <= hdmi_num {
        (*links.add(id as usize)).name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"iDisp%d\0"), i);
        if (*links.add(id as usize)).name.is_null() {
            return null_mut();
        }

        (*links.add(id as usize)).id = id + hdmi_id_offset;
        (*links.add(id as usize)).cpus = cpus.add(id as usize);
        (*links.add(id as usize)).num_cpus = 1;
        (*(*links.add(id as usize)).cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"iDisp%d Pin\0"), i);
        if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
            return null_mut();
        }

        (*idisp_components.add((i - 1) as usize)).name = cstr(b"ehdaudio0D2\0");
        (*idisp_components.add((i - 1) as usize)).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"intel-hdmi-hifi%d\0"), i);
        if (*idisp_components.add((i - 1) as usize)).dai_name.is_null() {
            return null_mut();
        }

        (*links.add(id as usize)).codecs = idisp_components.add((i - 1) as usize);
        (*links.add(id as usize)).num_codecs = 1;
        (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
        (*links.add(id as usize)).num_platforms = ARRAY_SIZE(&platform_component) as c_int;
        (*links.add(id as usize)).init = Some(sof_hdmi_init);
        (*links.add(id as usize)).playback_only = 1;
        (*links.add(id as usize)).no_pcm = 1;

        id += 1;
        i += 1;
    }

    /* HDMI-In SSP */
    if quirk & SOF_SSP_HDMI_CAPTURE_PRESENT != 0 {
        let num_of_hdmi_ssp = ((quirk & SOF_NO_OF_HDMI_CAPTURE_SSP_MASK) >> SOF_NO_OF_HDMI_CAPTURE_SSP_SHIFT) as c_int;

        i = 1;
        while i <= num_of_hdmi_ssp {
            let port = if i == 1 {
                ((quirk & SOF_HDMI_CAPTURE_1_SSP_MASK) >> SOF_HDMI_CAPTURE_1_SSP_SHIFT) as c_int
            } else {
                ((quirk & SOF_HDMI_CAPTURE_2_SSP_MASK) >> SOF_HDMI_CAPTURE_2_SSP_SHIFT) as c_int
            };

            (*links.add(id as usize)).cpus = cpus.add(id as usize);
            (*(*links.add(id as usize)).cpus).dai_name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"SSP%d Pin\0"), port);
            if (*(*links.add(id as usize)).cpus).dai_name.is_null() {
                return null_mut();
            }
            (*links.add(id as usize)).name = devm_kasprintf(dev, GFP_KERNEL, cstr(b"SSP%d-HDMI\0"), port);
            if (*links.add(id as usize)).name.is_null() {
                return null_mut();
            }
            (*links.add(id as usize)).id = id + hdmi_id_offset;
            (*links.add(id as usize)).codecs = addr_of_mut!(snd_soc_dummy_dlc);
            (*links.add(id as usize)).num_codecs = 1;
            (*links.add(id as usize)).platforms = platform_component.as_mut_ptr();
            (*links.add(id as usize)).num_platforms = ARRAY_SIZE(&platform_component) as c_int;
            (*links.add(id as usize)).capture_only = 1;
            (*links.add(id as usize)).no_pcm = 1;
            (*links.add(id as usize)).num_cpus = 1;
            id += 1;
            i += 1;
        }
    }

    links
}

static mut soc_components: [c_char; 30] = [0; 30];

 /* i2c-<HID>:00 with HID being 8 chars */
static mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];

unsafe extern "C" fn sof_es8336_probe(pdev: *mut platform_device) -> c_int {
    let dev = addr_of_mut!((*pdev).dev) as *mut device;
    let card: *mut snd_soc_card;
    let mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let mut props: [property_entry; MAX_NO_PROPS] = [property_entry { _private: [] }, property_entry { _private: [] }];
    let priv_: *mut sof_es8336_private;
    let mut fwnode: *mut fwnode_handle;
    let mut adev: *mut acpi_device;
    let dai_links: *mut snd_soc_dai_link;
    let codec_dev: *mut device;
    let gpio_mapping: *const acpi_gpio_mapping;
    let mut cnt: c_uint = 0;
    let mut dmic_be_num: c_int = 0;
    let hdmi_num: c_int = 3;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<sof_es8336_private>(), GFP_KERNEL) as *mut sof_es8336_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = addr_of_mut!(sof_es8336_card);
    (*card).dev = dev;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    /* check GPIO DMI quirks */
    dmi_check_system(sof_es8336_quirk_table.as_ptr());

    /* Use NHLT configuration only for Non-HDMI capture use case.
     * Because more than one SSP will be enabled for HDMI capture hence wrong codec
     * SSP will be set.
     */
    if (*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER != 0 {
        if (*mach).mach_params.i2s_link_mask == 0 {
            dev_warn(dev, cstr(b"No I2S link information provided, using SSP0. This may need to be modified with the quirk module parameter\n\0"));
        } else {
            /*
             * Set configuration based on platform NHLT.
             * In this machine driver, we can only support one SSP for the
             * ES8336 link.
             * In some cases multiple SSPs can be reported by NHLT, starting MSB-first
             * seems to pick the right connection.
             */
            let ssp: c_ulong;

            /* fls returns 1-based results, SSPs indices are 0-based */
            ssp = (fls((*mach).mach_params.i2s_link_mask) - 1) as c_ulong;

            quirk |= ssp;
        }
    }

    if (*mach).mach_params.dmic_num != 0 {
        quirk |= SOF_ES8336_ENABLE_DMIC;
    }

    if quirk_override != -1 {
        dev_info(dev, cstr(b"Overriding quirk 0x%lx => 0x%x\n\0"), quirk, quirk_override);
        quirk = quirk_override as c_ulong;
    }
    log_quirks(dev);

    if quirk & SOF_ES8336_ENABLE_DMIC != 0 {
        dmic_be_num = 2;
    }

    /* compute number of dai links */
    sof_es8336_card.num_links = 1 + dmic_be_num + hdmi_num;

    if quirk & SOF_SSP_HDMI_CAPTURE_PRESENT != 0 {
        sof_es8336_card.num_links += ((quirk & SOF_NO_OF_HDMI_CAPTURE_SSP_MASK) >> SOF_NO_OF_HDMI_CAPTURE_SSP_SHIFT) as c_int;
    }

    dai_links = sof_card_dai_links_create(dev, SOF_ES8336_SSP_CODEC(quirk) as c_int, dmic_be_num, hdmi_num);
    if dai_links.is_null() {
        return -ENOMEM;
    }

    sof_es8336_card.dai_link = dai_links;

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, null(), -1);
    if !adev.is_null() {
        snprintf(codec_name.as_mut_ptr(), codec_name.len(), cstr(b"i2c-%s\0"), acpi_dev_name(adev));
        (*(*dai_links.add(0)).codecs).name = codec_name.as_ptr();

        /* also fixup codec dai name if relevant */
        if strncmp((*mach).id, cstr(b"ESSX8326\0"), SND_ACPI_I2C_ID_LEN) == 0 {
            (*(*dai_links.add(0)).codecs).dai_name = cstr(b"ES8326 HiFi\0");
        }
    } else {
        dev_err(dev, cstr(b"Error cannot find '%s' dev\n\0"), (*mach).id);
        return -ENOENT;
    }

    codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if codec_dev.is_null() {
        return -EPROBE_DEFER;
    }
    (*priv_).codec_dev = get_device(codec_dev);

    ret = snd_soc_fixup_dai_links_platform_name(addr_of_mut!(sof_es8336_card), (*mach).mach_params.platform);
    if ret != 0 {
        put_device(codec_dev);
        return ret;
    }

    if quirk & SOF_ES8336_JD_INVERTED != 0 {
        /* PROPERTY_ENTRY_BOOL("everest,jack-detect-inverted") */
        cnt += 1;
    }

    if cnt != 0 {
        fwnode = fwnode_create_software_node(props.as_ptr(), null());
        if IS_ERR(fwnode as *const c_void) {
            put_device(codec_dev);
            return PTR_ERR(fwnode as *const c_void);
        }

        ret = device_add_software_node(codec_dev, to_software_node(fwnode));

        fwnode_handle_put(fwnode);

        if ret != 0 {
            put_device(codec_dev);
            return ret;
        }
    }

    /* get speaker enable GPIO */
    if quirk & SOF_ES8336_HEADPHONE_GPIO != 0 {
        if quirk & SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK != 0 {
            gpio_mapping = acpi_enable_both_gpios.as_ptr();
        } else {
            gpio_mapping = acpi_enable_both_gpios_rev_order.as_ptr();
        }
    } else if quirk & SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK != 0 {
        gpio_mapping = acpi_speakers_enable_gpio1.as_ptr();
    } else {
        gpio_mapping = acpi_speakers_enable_gpio0.as_ptr();
    }

    ret = devm_acpi_dev_add_driver_gpios(codec_dev, gpio_mapping);
    if ret != 0 {
        dev_warn(codec_dev, cstr(b"unable to add GPIO mapping table\n\0"));
    }

    (*priv_).gpio_speakers = gpiod_get_optional(codec_dev, cstr(b"speakers-enable\0"), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).gpio_speakers as *const c_void) {
        ret = dev_err_probe(dev, PTR_ERR((*priv_).gpio_speakers as *const c_void), cstr(b"could not get speakers-enable GPIO\n\0"));
        return ret;
    }

    (*priv_).gpio_headphone = gpiod_get_optional(codec_dev, cstr(b"headphone-enable\0"), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).gpio_headphone as *const c_void) {
        ret = dev_err_probe(dev, PTR_ERR((*priv_).gpio_headphone as *const c_void), cstr(b"could not get headphone-enable GPIO\n\0"));
        return ret;
    }

    INIT_LIST_HEAD(addr_of_mut!((*priv_).hdmi_pcm_list));
    INIT_DELAYED_WORK(addr_of_mut!((*priv_).pcm_pop_work), Some(pcm_pop_work_events));
    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    if (*mach).mach_params.dmic_num > 0 {
        snprintf(soc_components.as_mut_ptr(), soc_components.len(), cstr(b"cfg-dmics:%d\0"), (*mach).mach_params.dmic_num);
        (*card).components = soc_components.as_mut_ptr();
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        gpiod_put((*priv_).gpio_speakers);
        dev_err(dev, cstr(b"snd_soc_register_card failed: %d\n\0"), ret);
        device_remove_software_node((*priv_).codec_dev);
        put_device(codec_dev);
        return ret;
    }
    platform_set_drvdata(pdev, addr_of_mut!(sof_es8336_card) as *mut c_void);
    0
}

unsafe extern "C" fn sof_es8336_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut sof_es8336_private;

    cancel_delayed_work_sync(addr_of_mut!((*priv_).pcm_pop_work));
    gpiod_put((*priv_).gpio_speakers);
    device_remove_software_node((*priv_).codec_dev);
    put_device((*priv_).codec_dev);
}

const fn platform_id_name(bytes: &[u8]) -> [c_char; 32] {
    let mut out = [0 as c_char; 32];
    let mut i = 0;
    while i < bytes.len() && i < 32 {
        out[i] = bytes[i] as c_char;
        i += 1;
    }
    out
}

static board_ids: [platform_device_id; 8] = [
    platform_device_id {
        name: platform_id_name(b"sof-essx8336"), /* default quirk == 0 */
        driver_data: 0,
    },
    platform_device_id {
        name: platform_id_name(b"adl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: platform_id_name(b"rpl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: platform_id_name(b"mtl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: platform_id_name(b"arl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: platform_id_name(b"ptl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: platform_id_name(b"nvl_es83x6_c1_h02"),
        driver_data: (SOF_ES8336_SSP_CODEC(1)
            | SOF_NO_OF_HDMI_CAPTURE_SSP(2)
            | SOF_HDMI_CAPTURE_1_SSP(0)
            | SOF_HDMI_CAPTURE_2_SSP(2)
            | SOF_SSP_HDMI_CAPTURE_PRESENT
            | SOF_ES8336_SPEAKERS_EN_GPIO1_QUIRK
            | SOF_ES8336_JD_INVERTED) as kernel_ulong_t,
    },
    platform_device_id {
        name: [0; 32],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(platform, board_ids); */

static mut sof_es8336_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: cstr(b"sof-essx8336\0"),
        pm: unsafe { addr_of_mut!(snd_soc_pm_ops) as *const c_void },
    },
    probe: Some(sof_es8336_probe),
    remove: Some(sof_es8336_remove),
    id_table: board_ids.as_ptr(),
};
/* module_platform_driver(sof_es8336_driver); */

/* MODULE_DESCRIPTION("ASoC Intel(R) SOF + ES8336 Machine driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_IMPORT_NS("SND_SOC_INTEL_HDA_DSP_COMMON"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
