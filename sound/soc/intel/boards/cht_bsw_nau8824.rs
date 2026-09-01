// SPDX-License-Identifier: GPL-2.0-only
/*
 *  cht-bsw-nau8824.c - ASoc Machine driver for Intel Cherryview-based
 *          platforms Cherrytrail and Braswell, with nau8824 codec.
 *
 *  Copyright (C) 2018 Intel Corp
 *  Copyright (C) 2018 Nuvoton Technology Corp
 *
 *  Author: Wang, Joseph C <joequant@gmail.com>
 *  Co-author: John Hsu <KCHSU0@nuvoton.com>
 *  This file is based on cht_bsw_rt5672.c and cht-bsw-max98090.c
 */

// Dependencies from the original C includes:
// linux/module.h, linux/platform_device.h, linux/slab.h, linux/input.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h,
// sound/jack.h, ../atom/sst-atom-controls.h, ../../codecs/nau8824.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct cht_mc_private {
    pub jack: snd_soc_jack,
}

unsafe extern "C" {
    static mut snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        source: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        ty: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, ty: c_int, keytype: c_int) -> c_int;
    fn nau8824_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: snd_pcm_hw_param,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn params_set_format(params: *mut snd_pcm_hw_params, val: snd_pcm_format);
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: snd_pcm_hw_param,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn nau8824_components() -> *const c_char;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
}

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: c_int = 12;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const NAU8824_CLK_FLL_FS: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_DAIFMT_DSP_B: c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const MERR_DPCM_AUDIO: usize = 0;
const MERR_DPCM_DEEP_BUFFER: usize = 1;
const NAU8824_CODEC_DAI: *const c_char = c"nau8824-hifi".as_ptr();
const SOF_CARD_NAME: *const c_char = c"bytcht nau8824".as_ptr();
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();
const CARD_NAME: *const c_char = c"chtnau8824".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null();

type gfp_t = c_uint;
type snd_pcm_format = c_int;

const SNDRV_PCM_HW_PARAM_RATE: snd_pcm_hw_param = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: snd_pcm_hw_param = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: snd_pcm_hw_param = 2;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format = 0;

type snd_pcm_hw_param = c_int;

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
    pub mask: c_int,
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

#[repr(C)]
pub struct snd_kcontrol_new {
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
    pub dev: *mut device,
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
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub nonatomic: bool,
    pub dynamic: c_uint,
    pub playback_only: c_uint,
    pub id: c_int,
    pub no_pcm: c_uint,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub ops: *const snd_soc_ops,
    // SND_SOC_DAILINK_REG(...) fields are supplied by the ASoC macros in C.
}

#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dev: *mut device,
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub components: *const c_char,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub platform: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

static mut cht_bsw_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

// SND_SOC_DAPM_HP/MIC/SPK macro initializers are provided by the ASoC framework.
static cht_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
    snd_soc_dapm_widget { _private: [] },
];

static cht_audio_map: [snd_soc_dapm_route; 14] = [
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: ptr::null(), source: c"SPKOUTL".as_ptr() },
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: ptr::null(), source: c"SPKOUTR".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPOR".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC1".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"MIC2".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"HSMIC1".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"HSMIC2".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Playback".as_ptr(), control: ptr::null(), source: c"ssp2 Tx".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Tx".as_ptr(), control: ptr::null(), source: c"codec_out0".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Tx".as_ptr(), control: ptr::null(), source: c"codec_out1".as_ptr() },
    snd_soc_dapm_route { sink: c"codec_in0".as_ptr(), control: ptr::null(), source: c"ssp2 Rx".as_ptr() },
    snd_soc_dapm_route { sink: c"codec_in1".as_ptr(), control: ptr::null(), source: c"ssp2 Rx".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Rx".as_ptr(), control: ptr::null(), source: c"Capture".as_ptr() },
];

// SOC_DAPM_PIN_SWITCH macro initializers are provided by the ASoC framework.
static cht_mc_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn cht_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let codec_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };
    let mut ret: c_int;

    ret = unsafe { snd_soc_dai_set_sysclk(codec_dai, NAU8824_CLK_FLL_FS, 0, SND_SOC_CLOCK_IN) };
    if ret < 0 {
        unsafe { dev_err((*codec_dai).dev, c"can't set FS clock %d\n".as_ptr(), ret) };
        return ret;
    }
    ret = unsafe {
        snd_soc_dai_set_pll(
            codec_dai,
            0,
            0,
            params_rate(params),
            params_rate(params).wrapping_mul(256),
        )
    };
    if ret < 0 {
        unsafe { dev_err((*codec_dai).dev, c"can't set FLL: %d\n".as_ptr(), ret) };
        return ret;
    }

    0
}

unsafe extern "C" fn cht_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = unsafe { snd_soc_card_get_drvdata((*runtime).card) } as *mut cht_mc_private;
    let jack = unsafe { &mut (*ctx).jack as *mut snd_soc_jack };
    let codec_dai = unsafe { snd_soc_rtd_to_codec(runtime, 0) };
    let component = unsafe { (*codec_dai).component };
    let mut ret: c_int;
    let jack_type: c_int;

    /* NAU88L24 supports 4 buttons headset detection
     * KEY_PLAYPAUSE
     * KEY_VOICECOMMAND
     * KEY_VOLUMEUP
     * KEY_VOLUMEDOWN
     */
    jack_type = SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3;
    ret = unsafe {
        snd_soc_card_jack_new_pins(
            (*runtime).card,
            c"Headset".as_ptr(),
            jack_type,
            jack,
            cht_bsw_jack_pins.as_mut_ptr(),
            cht_bsw_jack_pins.len() as c_uint,
        )
    };
    if ret != 0 {
        unsafe { dev_err((*runtime).dev, c"Headset Jack creation failed %d\n".as_ptr(), ret) };
        return ret;
    }
    unsafe {
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
        snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

        nau8824_enable_jack_detect(component, jack);
    }

    ret
}

unsafe extern "C" fn cht_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE) };
    let channels = unsafe { hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS) };
    let fmt = unsafe { hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT) };
    let ret: c_int;

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    unsafe {
        (*rate).max = 48000;
        (*rate).min = (*rate).max;
        (*channels).max = 2;
        (*channels).min = (*channels).max;
    }

    /* set SSP2 to 24-bit */
    unsafe {
        snd_mask_none(fmt);
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
    }

    /* TDM 4 slots 24 bit, set Rx & Tx bitmask to 4 active slots */
    ret = unsafe { snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_codec(rtd, 0), 0xf, 0x1, 4, 24) };
    if ret < 0 {
        unsafe { dev_err((*rtd).dev, c"can't set codec TDM slot %d\n".as_ptr(), ret) };
        return ret;
    }

    0
}

unsafe extern "C" fn cht_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    unsafe { snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000) }
}

static cht_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(cht_aif1_startup),
    hw_params: None,
};

static cht_be_ssp2_ops: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(cht_aif1_hw_params),
};

// SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY()));
// SND_SOC_DAILINK_DEF(media, DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai")));
// SND_SOC_DAILINK_DEF(deepbuffer, DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai")));
// SND_SOC_DAILINK_DEF(ssp2_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp2-port")));
// SND_SOC_DAILINK_DEF(ssp2_codec, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10508824:00", NAU8824_CODEC_DAI)));
// SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform")));

static mut cht_dailink: [snd_soc_dai_link; 3] = [
    /* Front End DAI links */
    snd_soc_dai_link {
        name: c"Audio Port".as_ptr(),
        stream_name: c"Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 0,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        init: None,
        be_hw_params_fixup: None,
        ops: &cht_aif1_ops,
    },
    snd_soc_dai_link {
        name: c"Deep-Buffer Audio Port".as_ptr(),
        stream_name: c"Deep-Buffer Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        init: None,
        be_hw_params_fixup: None,
        ops: &cht_aif1_ops,
    },
    /* Back End DAI links */
    snd_soc_dai_link {
        /* SSP2 - Codec */
        name: c"SSP2-Codec".as_ptr(),
        stream_name: ptr::null(),
        nonatomic: false,
        dynamic: 0,
        playback_only: 0,
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_DSP_B | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBC_CFC,
        init: Some(cht_codec_init),
        be_hw_params_fixup: Some(cht_codec_fixup),
        ops: &cht_be_ssp2_ops,
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
/* SOF_CARD_NAME card name will be 'sof-bytcht nau8824' */
/* DRIVER_NAME: card name will be used for driver name */

/* SoC card */
#[unsafe(no_mangle)]
static mut snd_soc_card_cht: snd_soc_card = snd_soc_card {
    owner: ptr::null_mut(),
    dai_link: unsafe { cht_dailink.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: cht_dapm_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: cht_audio_map.as_ptr(),
    num_dapm_routes: 14,
    controls: cht_mc_controls.as_ptr(),
    num_controls: 4,
    dev: ptr::null_mut(),
    name: ptr::null(),
    driver_name: ptr::null(),
    components: ptr::null(),
};

unsafe extern "C" fn snd_cht_mc_probe(pdev: *mut platform_device) -> c_int {
    let drv: *mut cht_mc_private;
    let mach: *mut snd_soc_acpi_mach;
    let platform_name: *const c_char;
    let sof_parent: bool;
    let mut ret_val: c_int;

    drv = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<cht_mc_private>(),
            GFP_KERNEL,
        )
    } as *mut cht_mc_private;
    if drv.is_null() {
        return -ENOMEM;
    }
    unsafe { snd_soc_card_set_drvdata(&mut snd_soc_card_cht, drv as *mut c_void) };

    /* override platform name, if required */
    unsafe {
        snd_soc_card_cht.dev = &mut (*pdev).dev;
        mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
        platform_name = (*mach).mach_params.platform;
    }

    ret_val = unsafe { snd_soc_fixup_dai_links_platform_name(&mut snd_soc_card_cht, platform_name) };
    if ret_val != 0 {
        return ret_val;
    }

    sof_parent = unsafe { snd_soc_acpi_sof_parent(&mut (*pdev).dev) };

    /* set card and driver name */
    unsafe {
        if sof_parent {
            snd_soc_card_cht.name = SOF_CARD_NAME;
            snd_soc_card_cht.driver_name = SOF_DRIVER_NAME;
        } else {
            snd_soc_card_cht.name = CARD_NAME;
            snd_soc_card_cht.driver_name = DRIVER_NAME;
        }

        snd_soc_card_cht.components = nau8824_components();
    }

    /* set pm ops */
    if sof_parent {
        unsafe {
            (*(*pdev).dev.driver).pm = &snd_soc_pm_ops;
        }
    }

    /* register the soc card */
    ret_val = unsafe { devm_snd_soc_register_card(&mut (*pdev).dev, &mut snd_soc_card_cht) };
    if ret_val != 0 {
        unsafe { dev_err(&mut (*pdev).dev, c"snd_soc_register_card failed %d\n".as_ptr(), ret_val) };
        return ret_val;
    }
    unsafe { platform_set_drvdata(pdev, &mut snd_soc_card_cht as *mut snd_soc_card as *mut c_void) };

    ret_val
}

static mut snd_cht_mc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"cht-bsw-nau8824".as_ptr(),
        pm: ptr::null(),
    },
    probe: Some(snd_cht_mc_probe),
};

// module_platform_driver(snd_cht_mc_driver);
// MODULE_DESCRIPTION("ASoC Intel(R) Baytrail CR Machine driver");
// MODULE_AUTHOR("Wang, Joseph C <joequant@gmail.com>");
// MODULE_AUTHOR("John Hsu <KCHSU0@nuvoton.com>");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:cht-bsw-nau8824");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
