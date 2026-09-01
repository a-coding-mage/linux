// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

// Dependencies from the original C includes:
// linux/module.h, linux/platform_data/x86/soc.h, linux/platform_device.h,
// sound/jack.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-acpi.h, sound/soc-dapm.h, uapi/linux/input-event-codes.h,
// ../../../codecs/da7219.h, ../utils.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const DA7219_DAI_NAME: *const c_char = b"da7219-hifi\0".as_ptr() as *const c_char;

extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    static DA7219_SYSCLK_MCLK: c_int;
    static DA7219_SYSCLK_PLL_SRM: c_int;
    static DA7219_PLL_FREQ_OUT_98304: c_uint;
    static DA7219_CLKSRC_MCLK: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_JACK_HEADPHONE: c_uint;
    static SND_JACK_MICROPHONE: c_uint;
    static SND_JACK_LINEOUT: c_uint;
    static SND_JACK_HEADSET: c_uint;
    static SND_JACK_BTN_0: c_uint;
    static SND_JACK_BTN_1: c_uint;
    static SND_JACK_BTN_2: c_uint;
    static SND_JACK_BTN_3: c_uint;
    static KEY_PLAYPAUSE: c_uint;
    static KEY_VOLUMEUP: c_uint;
    static KEY_VOLUMEDOWN: c_uint;
    static KEY_VOICECOMMAND: c_uint;
    static SNDRV_PCM_HW_PARAM_RATE: c_int;
    static SNDRV_PCM_HW_PARAM_CHANNELS: c_int;
    static SNDRV_PCM_HW_PARAM_FORMAT: c_int;
    static SNDRV_PCM_FORMAT_S24_LE: c_int;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
    static EIO: c_int;
    static SND_SOC_NOPM: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;

    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_card_get_codec_dai(
        card: *mut snd_soc_card,
        dai_name: *const c_char,
    ) -> *mut snd_soc_dai;
    fn SND_SOC_DAPM_EVENT_OFF(event: c_int) -> bool;
    fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_rtd_to_codec(runtime: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut snd_soc_jack;
    fn soc_intel_is_apl() -> bool;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
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
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_uint, keytype: c_uint);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, format: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut c_int,
        tdm_slot: *mut c_int,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *mut c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub id: c_int,
    pub dai_fmt: c_uint,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub nonatomic: c_uint,
    pub no_pcm: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
    pub id_table: *const platform_device_id,
}

// Original C macro initializers from sound/soc.h are preserved as macro calls.
static card_controls: [snd_kcontrol_new; 3] = [
    SOC_DAPM_PIN_SWITCH!("Headphone Jack"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
    SOC_DAPM_PIN_SWITCH!("Line Out"),
];

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card: *mut snd_soc_card = snd_soc_dapm_to_card((*w).dapm);
    let codec_dai: *mut snd_soc_dai;
    let mut ret: c_int = 0;

    codec_dai = snd_soc_card_get_codec_dai(card, DA7219_DAI_NAME);
    if codec_dai.is_null() {
        dev_err(
            (*card).dev,
            b"Codec dai not found. Unable to set/unset codec pll\n\0".as_ptr()
                as *const c_char,
        );
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_OFF(event) {
        ret = snd_soc_dai_set_pll(codec_dai, 0, DA7219_SYSCLK_MCLK, 0, 0);
        if ret != 0 {
            dev_err(
                (*card).dev,
                b"failed to stop PLL: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    } else if SND_SOC_DAPM_EVENT_ON(event) {
        ret = snd_soc_dai_set_pll(
            codec_dai,
            0,
            DA7219_SYSCLK_PLL_SRM,
            0,
            DA7219_PLL_FREQ_OUT_98304,
        );
        if ret != 0 {
            dev_err(
                (*card).dev,
                b"failed to start PLL: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    ret
}

static card_widgets: [snd_soc_dapm_widget_def; 4] = [
    SND_SOC_DAPM_HP!("Headphone Jack", ptr::null()),
    SND_SOC_DAPM_MIC!("Headset Mic", ptr::null()),
    SND_SOC_DAPM_LINE!("Line Out", ptr::null()),
    SND_SOC_DAPM_SUPPLY!(
        "Platform Clock",
        SND_SOC_NOPM,
        0,
        0,
        platform_clock_control,
        SND_SOC_DAPM_POST_PMD | SND_SOC_DAPM_PRE_PMU
    ),
];

static card_base_routes: [snd_soc_dapm_route; 6] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HPR\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Platform Clock\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headset Mic\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Platform Clock\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Line Out\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Platform Clock\0".as_ptr() as *const c_char,
    },
];

static card_headset_pins: [snd_soc_jack_pin; 3] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Line Out\0".as_ptr() as *const c_char,
        mask: SND_JACK_LINEOUT,
    },
];

unsafe extern "C" fn avs_da7219_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(runtime, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;
    let card: *mut snd_soc_card = (*runtime).card;
    let pins: *mut snd_soc_jack_pin;
    let jack: *mut snd_soc_jack;
    let num_pins: c_int;
    let clk_freq: c_int;
    let mut ret: c_int;

    jack = snd_soc_card_get_drvdata(card);
    if soc_intel_is_apl() {
        clk_freq = 19200000;
    } else {
        /* kbl */
        clk_freq = 24576000;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, DA7219_CLKSRC_MCLK, clk_freq as c_uint, SND_SOC_CLOCK_IN);
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"can't set codec sysclk configuration\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    num_pins = card_headset_pins.len() as c_int;
    pins = devm_kmemdup_array(
        (*card).dev,
        card_headset_pins.as_ptr() as *const c_void,
        num_pins as usize,
        core::mem::size_of::<snd_soc_jack_pin>(),
        GFP_KERNEL,
    ) as *mut snd_soc_jack_pin;
    if pins.is_null() {
        return -ENOMEM;
    }

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET
            | SND_JACK_BTN_0
            | SND_JACK_BTN_1
            | SND_JACK_BTN_2
            | SND_JACK_BTN_3
            | SND_JACK_LINEOUT,
        jack,
        pins,
        num_pins as c_uint,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    snd_soc_component_set_jack(component, jack, ptr::null_mut())
}

unsafe extern "C" fn avs_da7219_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    snd_soc_component_set_jack(
        (*snd_soc_rtd_to_codec(rtd, 0)).component,
        ptr::null_mut(),
        ptr::null_mut(),
    );
}

unsafe extern "C" fn avs_da7219_be_fixup(
    _runrime: *mut snd_soc_pcm_runtime,
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

    /* set SSP0 to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
    0
}

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: c_int,
    tdm_slot: c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT!("SSP", "-Codec", ssp_port, tdm_slot),
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        AVS_STRING_FMT!("SSP", " Pin", ssp_port, tdm_slot),
    );
    (*(*dl).codecs).name = devm_kasprintf(dev, GFP_KERNEL, b"i2c-DLGS7219:00\0".as_ptr() as *const c_char);
    (*(*dl).codecs).dai_name = devm_kasprintf(dev, GFP_KERNEL, DA7219_DAI_NAME);
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
    (*dl).be_hw_params_fixup = Some(avs_da7219_be_fixup);
    (*dl).init = Some(avs_da7219_codec_init);
    (*dl).exit = Some(avs_da7219_codec_exit);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_da7219_probe(pdev: *mut platform_device) -> c_int {
    let mut dai_link: *mut snd_soc_dai_link = ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let jack: *mut snd_soc_jack;
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

    jack = devm_kzalloc(dev, core::mem::size_of::<snd_soc_jack>(), GFP_KERNEL)
        as *mut snd_soc_jack;
    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL)
        as *mut snd_soc_card;
    if jack.is_null() || card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = b"avs_da7219\0".as_ptr() as *const c_char;
    } else {
        (*card).driver_name = b"avs_da7219\0".as_ptr() as *const c_char;
        (*card).name = b"AVS I2S DA7219\0".as_ptr() as *const c_char;
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as c_uint;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as c_uint;
    (*card).fully_routed = true;
    snd_soc_card_set_drvdata(card, jack as *mut c_void);

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_da7219_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'7' as c_char,
            b'2' as c_char,
            b'1' as c_char,
            b'9' as c_char,
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
// MODULE_DEVICE_TABLE(platform, avs_da7219_driver_ids);

static mut avs_da7219_driver: platform_driver = platform_driver {
    probe: Some(avs_da7219_probe),
    driver: platform_driver_driver {
        name: b"avs_da7219\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    id_table: avs_da7219_driver_ids.as_ptr(),
};

// module_platform_driver(avs_da7219_driver);
// MODULE_DESCRIPTION("Intel da7219 machine driver");
// MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
