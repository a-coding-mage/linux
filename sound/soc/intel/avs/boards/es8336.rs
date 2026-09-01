// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2023 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const ES8336_CODEC_DAI: *const c_char = b"ES8316 HiFi\0".as_ptr() as *const c_char;

const true_: bool = true;
const false_: bool = false;

const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const ACPI_GPIO_QUIRK_ONLY_GPIOIO: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_JACK_HEADPHONE: c_uint = 0;
const SND_JACK_MICROPHONE: c_uint = 0;
const SND_JACK_BTN_0: c_uint = 0;
const SND_JACK_HEADSET: c_uint = 0;
const KEY_PLAYPAUSE: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const INTEL_KABYLAKE_L: c_uint = 0;
const INTEL_KABYLAKE: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 0;
const SNDRV_PCM_FORMAT_S24_3LE: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct avs_card_drvdata {
    jack: snd_soc_jack,
    gpiod: *mut gpio_desc,
}

#[repr(C)]
struct acpi_gpio_params {
    crs_entry_index: c_uint,
    line_index: c_uint,
    active_low: bool,
}

#[repr(C)]
struct acpi_gpio_mapping {
    name: *const c_char,
    data: *const acpi_gpio_params,
    size: c_uint,
    quirks: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct snd_kcontrol;

#[repr(C)]
struct snd_soc_card {
    name: *const c_char,
    driver_name: *const c_char,
    long_name: *const c_char,
    dev: *mut device,
    owner: *mut module,
    suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dai_link: *mut snd_soc_dai_link,
    num_links: c_int,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    fully_routed: bool,
}

#[repr(C)]
struct gpio_desc;

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const c_char,
    mask: c_uint,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_component;

#[repr(C)]
struct snd_soc_dapm_context;

#[repr(C)]
struct snd_jack;

#[repr(C)]
struct snd_pcm_substream;

#[repr(C)]
struct snd_pcm_hw_params;

#[repr(C)]
struct snd_interval {
    min: c_uint,
    max: c_uint,
}

#[repr(C)]
struct snd_mask;

#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const c_char,
    dai_name: *const c_char,
}

#[repr(C)]
struct snd_soc_dai_link {
    name: *const c_char,
    cpus: *mut snd_soc_dai_link_component,
    codecs: *mut snd_soc_dai_link_component,
    num_cpus: c_uint,
    num_codecs: c_uint,
    platforms: *mut snd_soc_dai_link_component,
    num_platforms: c_uint,
    id: c_int,
    dai_fmt: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    ops: *const snd_soc_ops,
    nonatomic: c_uint,
    no_pcm: c_uint,
}

#[repr(C)]
struct device;

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_soc_acpi_mach {
    pdata: *mut avs_mach_pdata,
}

#[repr(C)]
struct avs_mach_pdata {
    obsolete_card_names: bool,
}

#[repr(C)]
struct platform_device_id {
    name: [c_char; 20],
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const c_void,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
    id_table: *const platform_device_id,
}

#[repr(C)]
struct module;

#[repr(C)]
struct cpuinfo_x86 {
    x86_vfm: c_uint,
}

unsafe extern "C" {
    static mut boot_cpu_data: cpuinfo_x86;
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: bool);
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
    fn snd_soc_rtd_to_codec(runtime: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn devm_kmemdup_array(
        dev: *mut device,
        src: *const c_void,
        n: usize,
        size: usize,
        flags: c_uint,
    ) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn devm_acpi_dev_add_driver_gpios(dev: *mut device, gpios: *const acpi_gpio_mapping) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_dapm_set_idle_bias(dapm: *mut snd_soc_dapm_context, idle_bias_on: bool);
    fn gpiod_put(desc: *mut gpio_desc);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

static enable_gpio: acpi_gpio_params = acpi_gpio_params {
    crs_entry_index: 0,
    line_index: 0,
    active_low: true_,
};

static speaker_gpios: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping {
        name: b"speaker-enable-gpios\0".as_ptr() as *const c_char,
        data: &enable_gpio,
        size: 1,
        quirks: ACPI_GPIO_QUIRK_ONLY_GPIOIO,
    },
    acpi_gpio_mapping {
        name: ptr::null(),
        data: ptr::null(),
        size: 0,
        quirks: 0,
    },
];

unsafe extern "C" fn avs_es8336_speaker_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let data: *mut avs_card_drvdata;
    let speaker_en: bool;

    data = snd_soc_card_get_drvdata(card) as *mut avs_card_drvdata;
    /* As enable_gpio has active_low=true, logic is inverted. */
    speaker_en = !SND_SOC_DAPM_EVENT_ON(event);

    gpiod_set_value_cansleep((*data).gpiod, speaker_en);
    0
}

// SND_SOC_DAPM_* macro initializers from the C source are preserved as dependency-created items.
static card_widgets: [snd_soc_dapm_widget; 5] = unsafe {
    [
        SND_SOC_DAPM_SPK(b"Speaker\0".as_ptr() as *const c_char, ptr::null()),
        SND_SOC_DAPM_HP(b"Headphone\0".as_ptr() as *const c_char, ptr::null()),
        SND_SOC_DAPM_MIC(b"Headset Mic\0".as_ptr() as *const c_char, ptr::null()),
        SND_SOC_DAPM_MIC(b"Internal Mic\0".as_ptr() as *const c_char, ptr::null()),
        SND_SOC_DAPM_SUPPLY(
            b"Speaker Power\0".as_ptr() as *const c_char,
            SND_SOC_NOPM,
            0,
            0,
            Some(avs_es8336_speaker_power_event),
            SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
        ),
    ]
};

unsafe extern "C" {
    fn SND_SOC_DAPM_SPK(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_HP(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIC(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_SUPPLY(
        name: *const c_char,
        reg: c_int,
        shift: c_int,
        invert: c_int,
        event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
        flags: c_int,
    ) -> snd_soc_dapm_widget;
    fn SOC_DAPM_PIN_SWITCH(name: *const c_char) -> snd_kcontrol_new;
}

static card_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOR\0".as_ptr() as *const c_char,
    },

    /*
     * There is no separate speaker output instead the speakers are muxed to
     * the HP outputs. The mux is controlled by the "Speaker Power" widget.
     */
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPOR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Speaker\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Speaker Power\0".as_ptr() as *const c_char,
    },

    /* Mic route map */
    snd_soc_dapm_route {
        sink: b"MIC1\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Internal Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC2\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

static card_controls: [snd_kcontrol_new; 4] = unsafe {
    [
        SOC_DAPM_PIN_SWITCH(b"Speaker\0".as_ptr() as *const c_char),
        SOC_DAPM_PIN_SWITCH(b"Headphone\0".as_ptr() as *const c_char),
        SOC_DAPM_PIN_SWITCH(b"Headset Mic\0".as_ptr() as *const c_char),
        SOC_DAPM_PIN_SWITCH(b"Internal Mic\0".as_ptr() as *const c_char),
    ]
};

static card_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn avs_es8336_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(runtime, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;
    let card: *mut snd_soc_card = (*runtime).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let pins: *mut snd_soc_jack_pin;
    let data: *mut avs_card_drvdata;
    let gpiod: *mut gpio_desc;
    let num_pins: c_int;
    let mut ret: c_int;

    data = snd_soc_card_get_drvdata(card) as *mut avs_card_drvdata;
    num_pins = card_headset_pins.len() as c_int;

    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &mut (*data).jack,
        pins,
        num_pins,
    );
    if ret != 0 {
        return ret;
    }

    ret = devm_acpi_dev_add_driver_gpios((*codec_dai).dev, speaker_gpios.as_ptr());
    if ret != 0 {
        dev_warn(
            (*codec_dai).dev,
            b"Unable to add GPIO mapping table\n\0".as_ptr() as *const c_char,
        );
    }

    gpiod = gpiod_get_optional(
        (*codec_dai).dev,
        b"speaker-enable\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR(gpiod as *const c_void) {
        return dev_err_probe(
            (*codec_dai).dev,
            PTR_ERR(gpiod as *const c_void),
            b"Get gpiod failed: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR(gpiod as *const c_void),
        );
    }

    (*data).gpiod = gpiod;
    snd_jack_set_key((*data).jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_soc_component_set_jack(component, &mut (*data).jack, ptr::null_mut());

    snd_soc_dapm_set_idle_bias(dapm, false_);

    0
}

unsafe extern "C" fn avs_es8336_codec_exit(runtime: *mut snd_soc_pcm_runtime) {
    let data: *mut avs_card_drvdata =
        snd_soc_card_get_drvdata((*runtime).card) as *mut avs_card_drvdata;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(runtime, 0);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut());
    gpiod_put((*data).gpiod);
}

unsafe extern "C" fn avs_es8336_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(runtime, 0);
    let clk_freq: c_uint;
    let ret: c_int;

    match boot_cpu_data.x86_vfm {
        INTEL_KABYLAKE_L | INTEL_KABYLAKE => {
            clk_freq = 24000000;
        }
        _ => {
            clk_freq = 19200000;
        }
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 1, clk_freq, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        dev_err(
            (*runtime).dev,
            b"Set codec sysclk failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

static avs_es8336_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(avs_es8336_hw_params),
};

unsafe extern "C" fn avs_es8336_be_fixup(
    _runtime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval;
    let channels: *mut snd_interval;
    let fmt: *mut snd_mask;

    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    /* The ADSP will convert the FE rate to 48k, stereo */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSPN to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_3LE);

    0
}

unsafe fn AVS_STRING_FMT(
    prefix: *const c_char,
    suffix: *const c_char,
    ssp_port: c_int,
    tdm_slot: c_int,
) -> *const c_char {
    let _ = (prefix, suffix, ssp_port, tdm_slot);
    b"%s%d%s%d\0".as_ptr() as *const c_char
}

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    platform = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT(
            b"SSP\0".as_ptr() as *const c_char,
            b"-Codec\0".as_ptr() as *const c_char,
            ssp_port,
            tdm_slot,
        ),
    );
    (*dl).cpus = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
        as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT(
            b"SSP\0".as_ptr() as *const c_char,
            b" Pin\0".as_ptr() as *const c_char,
            ssp_port,
            tdm_slot,
        ),
    );
    (*(*dl).codecs).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        b"i2c-ESSX8336:00\0".as_ptr() as *const c_char,
    );
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, ES8336_CODEC_DAI);
    if (*(*dl).cpus).dai_name.is_null()
        || (*(*dl).codecs).name.is_null()
        || (*(*dl).codecs).dai_name.is_null()
    {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl).num_cpus = 1;
    (*dl).num_codecs = 1;
    (*dl).platforms = platform;
    (*dl).num_platforms = 1;
    (*dl).id = 0;
    (*dl).dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_es8336_codec_init);
    (*dl).exit = Some(avs_es8336_codec_exit);
    (*dl).be_hw_params_fixup = Some(avs_es8336_be_fixup);
    (*dl).ops = &avs_es8336_ops;
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, ES8336_CODEC_DAI);

    snd_soc_component_set_jack((*codec_dai).component, ptr::null_mut(), ptr::null_mut())
}

unsafe extern "C" fn avs_card_resume_post(card: *mut snd_soc_card) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_card_get_codec_dai(card, ES8336_CODEC_DAI);
    let data: *mut avs_card_drvdata = snd_soc_card_get_drvdata(card) as *mut avs_card_drvdata;

    snd_soc_component_set_jack((*codec_dai).component, &mut (*data).jack, ptr::null_mut())
}

unsafe extern "C" fn avs_es8336_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let data: *mut avs_card_drvdata;
    let card: *mut snd_soc_card;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ssp_port: c_int = 0;
    let mut tdm_slot: c_int = 0;
    let mut ret: c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to create dai link: %d\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    data = devm_kzalloc(dev, size_of::<avs_card_drvdata>(), GFP_KERNEL) as *mut avs_card_drvdata;
    card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if data.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_es8336\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_es8336\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S ES8336\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).suspend_pre = Some(avs_card_suspend_pre);
    (*card).resume_post = Some(avs_card_resume_post);
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_uint;
    (*card).dapm_routes = card_routes.as_ptr();
    (*card).num_dapm_routes = card_routes.len() as c_uint;
    (*card).fully_routed = true_;
    snd_soc_card_set_drvdata(card, data as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_es8336_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'e' as c_char,
            b's' as c_char,
            b'8' as c_char,
            b'3' as c_char,
            b'3' as c_char,
            b'6' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(platform, avs_es8336_driver_ids);

static mut avs_es8336_driver: platform_driver = platform_driver {
    probe: Some(avs_es8336_probe),
    driver: device_driver {
        name: b"avs_es8336\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    id_table: avs_es8336_driver_ids.as_ptr(),
};

// module_platform_driver(avs_es8336_driver);
// MODULE_DESCRIPTION("Intel es8336 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
