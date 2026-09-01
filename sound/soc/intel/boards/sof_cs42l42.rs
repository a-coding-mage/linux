// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2021 Intel Corporation.

/*
 * Intel SOF Machine Driver with Cirrus Logic CS42L42 Codec
 * and speaker codec MAX98357A
 */

// C includes translated as external dependencies:
// linux/i2c.h, linux/input.h, linux/module.h, linux/platform_device.h,
// linux/regulator/consumer.h, linux/dmi.h, sound/core.h, sound/jack.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/sof.h,
// sound/soc-acpi.h, dt-bindings/sound/cs42l42.h,
// ../common/soc-intel-quirks.h, sof_board_helpers.h, sof_maxim_common.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

type kernel_ulong_t = c_ulong;

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id_entry: *const platform_device_id,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: *const c_char,
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub codec_mask: c_ulong,
    pub platform: *const c_char,
}

#[repr(C)]
pub struct sof_card_private {
    pub headset_jack: snd_soc_jack,
    pub codec_link: *mut snd_soc_dai_link,
    pub amp_type: c_int,
    pub amp_link: *mut snd_soc_dai_link,
    pub dmic_be_num: c_int,
    pub link_order_overwrite: c_ulong,
    pub hdmi: sof_hdmi_private,
}

#[repr(C)]
pub struct sof_hdmi_private {
    pub idisp_codec: bool,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_int,
        dir: c_int,
    ) -> c_int;
    fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
    fn sof_intel_board_set_dai_link(
        dev: *mut device,
        card: *mut snd_soc_card,
        ctx: *mut sof_card_private,
    ) -> c_int;
    fn max_98357a_dai_link(link: *mut snd_soc_dai_link);
    fn max_98360a_dai_link(link: *mut snd_soc_dai_link);
    fn sof_intel_board_get_ctx(dev: *mut device, quirk: c_ulong) -> *mut sof_card_private;
    fn soc_intel_is_glk() -> bool;
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

extern "C" {
    fn SOF_SSP_PORT_CODEC(port: c_int) -> c_ulong;
    fn SOF_SSP_PORT_AMP(port: c_int) -> c_ulong;
    fn SOF_NUM_IDISP_HDMI(num: c_int) -> c_ulong;
    fn SOF_SSP_PORT_BT_OFFLOAD(port: c_int) -> c_ulong;
    fn SOF_LINK_ORDER(
        a: c_ulong,
        b: c_ulong,
        c: c_ulong,
        d: c_ulong,
        e: c_ulong,
        f: c_ulong,
        g: c_ulong,
    ) -> c_ulong;
    fn SOC_DAPM_PIN_SWITCH(name: *const c_char) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_HP(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_MIC(name: *const c_char, event: *const c_void) -> snd_soc_dapm_widget;
}

extern "C" {
    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static KEY_PLAYPAUSE: c_int;
    static KEY_VOLUMEUP: c_int;
    static KEY_VOLUMEDOWN: c_int;
    static KEY_VOICECOMMAND: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static CODEC_NONE: c_int;
    static CODEC_MAX98357A: c_int;
    static CODEC_MAX98360A: c_int;
    static IDISP_CODEC_MASK: c_ulong;
    static SOF_LINK_AMP: c_ulong;
    static SOF_LINK_CODEC: c_ulong;
    static SOF_LINK_DMIC01: c_ulong;
    static SOF_LINK_IDISP_HDMI: c_ulong;
    static SOF_LINK_NONE: c_ulong;
    static SOF_BT_OFFLOAD_PRESENT: c_ulong;
}

static mut jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone Jack\0".as_ptr() as *const c_char,
        mask: unsafe { SND_JACK_HEADPHONE },
    },
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: unsafe { SND_JACK_MICROPHONE },
    },
];

/* Default: SSP2 */
static mut sof_cs42l42_quirk: c_ulong = unsafe { SOF_SSP_PORT_CODEC(2) };

unsafe extern "C" fn sof_cs42l42_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*rtd).card) as *mut sof_card_private;
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let jack = &mut (*ctx).headset_jack as *mut snd_soc_jack;
    let mut ret: c_int;

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        jack_pins.as_mut_ptr(),
        jack_pins.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"Headset Jack creation failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEDOWN);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOICECOMMAND);

    ret = snd_soc_component_set_jack(component, jack, ptr::null_mut());
    if ret != 0 {
        dev_err(
            (*rtd).dev,
            b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret
}

unsafe extern "C" fn sof_cs42l42_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
}

unsafe extern "C" fn sof_cs42l42_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let clk_freq: c_int;
    let ret: c_int;

    clk_freq = sof_dai_get_bclk(rtd); /* BCLK freq */

    if clk_freq <= 0 {
        dev_err(
            (*rtd).dev,
            b"get bclk freq failed: %d\n\0".as_ptr() as *const c_char,
            clk_freq,
        );
        return -EINVAL;
    }

    /* Configure sysclk for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, clk_freq, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err(
            (*rtd).dev,
            b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

static sof_cs42l42_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sof_cs42l42_hw_params),
};

unsafe extern "C" fn sof_card_late_probe(card: *mut snd_soc_card) -> c_int {
    sof_intel_board_card_late_probe(card)
}

static sof_controls: [snd_kcontrol_new; 2] = [
    unsafe { SOC_DAPM_PIN_SWITCH(b"Headphone Jack\0".as_ptr() as *const c_char) },
    unsafe { SOC_DAPM_PIN_SWITCH(b"Headset Mic\0".as_ptr() as *const c_char) },
];

static sof_widgets: [snd_soc_dapm_widget; 2] = [
    unsafe { SND_SOC_DAPM_HP(b"Headphone Jack\0".as_ptr() as *const c_char, ptr::null()) },
    unsafe { SND_SOC_DAPM_MIC(b"Headset Mic\0".as_ptr() as *const c_char, ptr::null()) },
];

static sof_map: [snd_soc_dapm_route; 2] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"HP\0".as_ptr() as *const c_char,
    },
    /* other jacks */
    snd_soc_dapm_route {
        sink: b"HS\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headset Mic\0".as_ptr() as *const c_char,
    },
];

/* sof audio machine driver for cs42l42 codec */
static mut sof_audio_card_cs42l42: snd_soc_card = snd_soc_card {
    name: b"cs42l42\0".as_ptr() as *const c_char, /* the sof- prefix is added by the core */
    owner: unsafe { THIS_MODULE },
    controls: sof_controls.as_ptr(),
    num_controls: sof_controls.len() as c_int,
    dapm_widgets: sof_widgets.as_ptr(),
    num_dapm_widgets: sof_widgets.len() as c_int,
    dapm_routes: sof_map.as_ptr(),
    num_dapm_routes: sof_map.len() as c_int,
    fully_routed: true,
    late_probe: Some(sof_card_late_probe),
    dev: ptr::null_mut(),
};

static mut cs42l42_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-10134242:00\0".as_ptr() as *const c_char,
    dai_name: b"cs42l42\0".as_ptr() as *const c_char,
}];

unsafe extern "C" fn sof_card_dai_links_create(
    dev: *mut device,
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
) -> c_int {
    let ret: c_int;

    ret = sof_intel_board_set_dai_link(dev, card, ctx);
    if ret != 0 {
        return ret;
    }

    if (*ctx).codec_link.is_null() {
        dev_err(dev, b"codec link not available\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* codec-specific fields for headphone codec */
    (*(*ctx).codec_link).codecs = cs42l42_component.as_mut_ptr();
    (*(*ctx).codec_link).num_codecs = cs42l42_component.len() as c_int;
    (*(*ctx).codec_link).init = Some(sof_cs42l42_init);
    (*(*ctx).codec_link).exit = Some(sof_cs42l42_exit);
    (*(*ctx).codec_link).ops = &sof_cs42l42_ops;

    if (*ctx).amp_type == CODEC_NONE {
        return 0;
    }

    if (*ctx).amp_link.is_null() {
        dev_err(dev, b"amp link not available\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* codec-specific fields for speaker amplifier */
    match (*ctx).amp_type {
        x if x == CODEC_MAX98357A => {
            max_98357a_dai_link((*ctx).amp_link);
        }
        x if x == CODEC_MAX98360A => {
            max_98360a_dai_link((*ctx).amp_link);
        }
        _ => {
            dev_err(
                dev,
                b"invalid amp type %d\n\0".as_ptr() as *const c_char,
                (*ctx).amp_type,
            );
            return -EINVAL;
        }
    }

    0
}

static GLK_LINK_ORDER: c_ulong = unsafe {
    SOF_LINK_ORDER(
        SOF_LINK_AMP,
        SOF_LINK_CODEC,
        SOF_LINK_DMIC01,
        SOF_LINK_IDISP_HDMI,
        SOF_LINK_NONE,
        SOF_LINK_NONE,
        SOF_LINK_NONE,
    )
};

unsafe extern "C" fn sof_audio_probe(pdev: *mut platform_device) -> c_int {
    let mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        sof_cs42l42_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dev_dbg(
        &mut (*pdev).dev,
        b"sof_cs42l42_quirk = %lx\n\0".as_ptr() as *const c_char,
        sof_cs42l42_quirk,
    );

    /* initialize ctx with board quirk */
    ctx = sof_intel_board_get_ctx(&mut (*pdev).dev, sof_cs42l42_quirk);
    if ctx.is_null() {
        return -ENOMEM;
    }

    if soc_intel_is_glk() {
        (*ctx).dmic_be_num = 1;

        /* overwrite the DAI link order for GLK boards */
        (*ctx).link_order_overwrite = GLK_LINK_ORDER;
    }

    if (*mach).mach_params.codec_mask & IDISP_CODEC_MASK != 0 {
        (*ctx).hdmi.idisp_codec = true;
    }

    /* update dai_link */
    ret = sof_card_dai_links_create(&mut (*pdev).dev, &mut sof_audio_card_cs42l42, ctx);
    if ret != 0 {
        return ret;
    }

    sof_audio_card_cs42l42.dev = &mut (*pdev).dev;

    /* set platform name for each dailink */
    ret = snd_soc_fixup_dai_links_platform_name(
        &mut sof_audio_card_cs42l42,
        (*mach).mach_params.platform,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut sof_audio_card_cs42l42, ctx as *mut c_void);

    devm_snd_soc_register_card(&mut (*pdev).dev, &mut sof_audio_card_cs42l42)
}

static board_ids: [platform_device_id; 6] = [
    platform_device_id {
        name: b"glk_cs4242_mx98357a\0".as_ptr() as *const c_char,
        driver_data: unsafe { (SOF_SSP_PORT_CODEC(2) | SOF_SSP_PORT_AMP(1)) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"jsl_cs4242_mx98360a\0".as_ptr() as *const c_char,
        driver_data: unsafe { (SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1)) as kernel_ulong_t },
    },
    platform_device_id {
        name: b"adl_cs42l42_def\0".as_ptr() as *const c_char,
        driver_data: unsafe {
            (SOF_SSP_PORT_CODEC(0)
                | SOF_SSP_PORT_AMP(1)
                | SOF_NUM_IDISP_HDMI(4)
                | SOF_BT_OFFLOAD_PRESENT
                | SOF_SSP_PORT_BT_OFFLOAD(2)) as kernel_ulong_t
        },
    },
    platform_device_id {
        name: b"rpl_cs42l42_def\0".as_ptr() as *const c_char,
        driver_data: unsafe {
            (SOF_SSP_PORT_CODEC(0)
                | SOF_SSP_PORT_AMP(1)
                | SOF_NUM_IDISP_HDMI(4)
                | SOF_BT_OFFLOAD_PRESENT
                | SOF_SSP_PORT_BT_OFFLOAD(2)) as kernel_ulong_t
        },
    },
    platform_device_id {
        name: b"mtl_cs42l42_def\0".as_ptr() as *const c_char,
        driver_data: unsafe {
            (SOF_SSP_PORT_CODEC(2)
                | SOF_SSP_PORT_AMP(0)
                | SOF_BT_OFFLOAD_PRESENT
                | SOF_SSP_PORT_BT_OFFLOAD(1)) as kernel_ulong_t
        },
    },
    platform_device_id {
        name: ptr::null(),
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(platform, board_ids);

static mut sof_audio: platform_driver = platform_driver {
    probe: Some(sof_audio_probe),
    driver: device_driver {
        name: b"sof_cs42l42\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
    },
    id_table: board_ids.as_ptr(),
};
// module_platform_driver(sof_audio)

/* Module information */
// MODULE_DESCRIPTION("SOF Audio Machine driver for CS42L42");
// MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS");
// MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_MAXIM_COMMON");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
