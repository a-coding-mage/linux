// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sam9g20_wm8731  --  SoC audio for AT91SAM9G20-based
 * 			ATMEL AT91SAM9G20ek board.
 *
 *  Copyright (C) 2005 SAN People
 *  Copyright (C) 2008 Atmel
 *
 * Authors: Sedji Gaouaou <sedji.gaouaou@atmel.com>
 *
 * Based on ati_b1_wm8731.c by:
 * Frank Mandarino <fmandarino@endrelia.com>
 * Copyright 2006 Endrelia Technologies Inc.
 * Based on corgi.c by:
 * Copyright 2005 Wolfson Microelectronics PLC.
 * Copyright 2005 Openedhand Ltd.
 */

// Dependencies in the original C source:
// linux/module.h, linux/moduleparam.h, linux/kernel.h, linux/clk.h,
// linux/timer.h, linux/interrupt.h, linux/platform_device.h, linux/of.h,
// linux/atmel-ssc.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, ../codecs/wm8731.h, atmel-pcm.h, atmel_ssc_dai.h.

const MCLK_RATE: u32 = 12000000;

/*
 * As shipped the board does not have inputs.  However, it is relatively
 * straightforward to modify the board to hook them up so support is left
 * in the driver.
 */
// Original C has: #undef ENABLE_MIC_INPUT

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const core::ffi::c_char,
    pub control: *const core::ffi::c_char,
    pub source: *const core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const core::ffi::c_char,
    pub dai_name: *const core::ffi::c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const core::ffi::c_char,
    pub stream_name: *const core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> core::ffi::c_int>,
    pub dai_fmt: core::ffi::c_uint,
    pub playback_only: bool,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: core::ffi::c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: core::ffi::c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const core::ffi::c_char,
    pub owner: *mut core::ffi::c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: core::ffi::c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: core::ffi::c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: core::ffi::c_int,
    pub fully_routed: bool,
    pub dev: *mut device,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const ENODEV: core::ffi::c_int = 19;
const EINVAL: core::ffi::c_int = 22;

extern "C" {
    static mut THIS_MODULE: *mut core::ffi::c_void;

    static WM8731_SYSCLK_MCLK: core::ffi::c_int;
    static SND_SOC_CLOCK_IN: core::ffi::c_int;
    static SND_SOC_DAIFMT_I2S: core::ffi::c_uint;
    static SND_SOC_DAIFMT_NB_NF: core::ffi::c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: core::ffi::c_uint;

    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: core::ffi::c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: core::ffi::c_int,
        freq: core::ffi::c_uint,
        dir: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn snd_soc_dapm_disable_pin(
        dapm: *mut core::ffi::c_void,
        pin: *const core::ffi::c_char,
    ) -> core::ffi::c_int;

    fn atmel_ssc_set_audio(id: core::ffi::c_int) -> core::ffi::c_int;
    fn atmel_ssc_put_audio(id: core::ffi::c_int);
    fn snd_soc_of_parse_card_name(
        card: *mut snd_soc_card,
        propname: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn snd_soc_of_parse_audio_routing(
        card: *mut snd_soc_card,
        propname: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const core::ffi::c_char,
        index: core::ffi::c_int,
    ) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn snd_soc_register_card(card: *mut snd_soc_card) -> core::ffi::c_int;
    fn snd_soc_unregister_card(card: *mut snd_soc_card);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut snd_soc_card;

    fn dev_dbg(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_err_probe(
        dev: *mut device,
        err: core::ffi::c_int,
        fmt: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
}

static AT91SAM9G20EK_DAPM_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        name: b"Int Mic\0".as_ptr() as *const core::ffi::c_char,
    },
    snd_soc_dapm_widget {
        name: b"Ext Spk\0".as_ptr() as *const core::ffi::c_char,
    },
];

static INTERCON: [snd_soc_dapm_route; 4] = [
    /*
     * speaker connected to LHPOUT/RHPOUT
     */
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const core::ffi::c_char,
        control: core::ptr::null(),
        source: b"LHPOUT\0".as_ptr() as *const core::ffi::c_char,
    },
    snd_soc_dapm_route {
        sink: b"Ext Spk\0".as_ptr() as *const core::ffi::c_char,
        control: core::ptr::null(),
        source: b"RHPOUT\0".as_ptr() as *const core::ffi::c_char,
    },
    /*
     * mic is connected to Mic Jack, with WM8731 Mic Bias
     */
    snd_soc_dapm_route {
        sink: b"MICIN\0".as_ptr() as *const core::ffi::c_char,
        control: core::ptr::null(),
        source: b"Mic Bias\0".as_ptr() as *const core::ffi::c_char,
    },
    snd_soc_dapm_route {
        sink: b"Mic Bias\0".as_ptr() as *const core::ffi::c_char,
        control: core::ptr::null(),
        source: b"Int Mic\0".as_ptr() as *const core::ffi::c_char,
    },
];

/*
 * Logic for a wm8731 as connected on a at91sam9g20ek board.
 */
unsafe extern "C" fn at91sam9g20ek_wm8731_init(
    rtd: *mut snd_soc_pcm_runtime,
) -> core::ffi::c_int {
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let dev: *mut device = (*rtd).dev;
    let mut ret: core::ffi::c_int;

    dev_dbg(
        dev,
        b"%s called\n\0".as_ptr() as *const core::ffi::c_char,
        b"at91sam9g20ek_wm8731_init\0".as_ptr() as *const core::ffi::c_char,
    );

    ret = snd_soc_dai_set_sysclk(
        codec_dai,
        WM8731_SYSCLK_MCLK,
        MCLK_RATE,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err(
            dev,
            b"Failed to set WM8731 SYSCLK: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }

    // Original C condition: #ifndef ENABLE_MIC_INPUT
    snd_soc_dapm_disable_pin(
        snd_soc_card_to_dapm((*rtd).card),
        b"Int Mic\0".as_ptr() as *const core::ffi::c_char,
    );

    0
}

// SND_SOC_DAILINK_DEFS(pcm,
//     DAILINK_COMP_ARRAY(COMP_CPU("at91rm9200_ssc.0")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("wm8731.0-001b", "wm8731-hifi")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("at91rm9200_ssc.0")));
static mut PCM_CPUS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"at91rm9200_ssc.0\0".as_ptr() as *const core::ffi::c_char,
    dai_name: core::ptr::null(),
    of_node: core::ptr::null_mut(),
}];

static mut PCM_CODECS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"wm8731.0-001b\0".as_ptr() as *const core::ffi::c_char,
    dai_name: b"wm8731-hifi\0".as_ptr() as *const core::ffi::c_char,
    of_node: core::ptr::null_mut(),
}];

static mut PCM_PLATFORMS: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"at91rm9200_ssc.0\0".as_ptr() as *const core::ffi::c_char,
    dai_name: core::ptr::null(),
    of_node: core::ptr::null_mut(),
}];

static mut AT91SAM9G20EK_DAI: snd_soc_dai_link = snd_soc_dai_link {
    name: b"WM8731\0".as_ptr() as *const core::ffi::c_char,
    stream_name: b"WM8731 PCM\0".as_ptr() as *const core::ffi::c_char,
    init: Some(at91sam9g20ek_wm8731_init),
    dai_fmt: unsafe { SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP },
    // Original C condition: #ifndef ENABLE_MIC_INPUT
    playback_only: true,
    cpus: unsafe { PCM_CPUS.as_mut_ptr() },
    num_cpus: 1,
    codecs: unsafe { PCM_CODECS.as_mut_ptr() },
    num_codecs: 1,
    platforms: unsafe { PCM_PLATFORMS.as_mut_ptr() },
    num_platforms: 1,
};

static mut SND_SOC_AT91SAM9G20EK: snd_soc_card = snd_soc_card {
    name: b"AT91SAMG20-EK\0".as_ptr() as *const core::ffi::c_char,
    owner: unsafe { THIS_MODULE },
    dai_link: unsafe { &mut AT91SAM9G20EK_DAI },
    num_links: 1,

    dapm_widgets: AT91SAM9G20EK_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: AT91SAM9G20EK_DAPM_WIDGETS.len() as core::ffi::c_int,
    dapm_routes: INTERCON.as_ptr(),
    num_dapm_routes: INTERCON.len() as core::ffi::c_int,
    fully_routed: true,
    dev: core::ptr::null_mut(),
};

unsafe extern "C" fn at91sam9g20ek_audio_probe(
    pdev: *mut platform_device,
) -> core::ffi::c_int {
    let np: *mut device_node = (*pdev).dev.of_node;
    let mut codec_np: *mut device_node;
    let cpu_np: *mut device_node;
    let card: *mut snd_soc_card = &mut SND_SOC_AT91SAM9G20EK;
    let mut ret: core::ffi::c_int;

    if np.is_null() {
        return -ENODEV;
    }

    ret = atmel_ssc_set_audio(0);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"ssc channel is not valid: %d\n\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }

    (*card).dev = &mut (*pdev).dev;

    /* Parse device node info */
    ret = snd_soc_of_parse_card_name(
        card,
        b"atmel,model\0".as_ptr() as *const core::ffi::c_char,
    );
    if ret != 0 {
        atmel_ssc_put_audio(0);
        return ret;
    }

    ret = snd_soc_of_parse_audio_routing(
        card,
        b"atmel,audio-routing\0".as_ptr() as *const core::ffi::c_char,
    );
    if ret != 0 {
        atmel_ssc_put_audio(0);
        return ret;
    }

    /* Parse codec info */
    (*AT91SAM9G20EK_DAI.codecs).name = core::ptr::null();
    codec_np = of_parse_phandle(
        np,
        b"atmel,audio-codec\0".as_ptr() as *const core::ffi::c_char,
        0,
    );
    if codec_np.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"codec info missing\n\0".as_ptr() as *const core::ffi::c_char,
        );
        ret = -EINVAL;
        atmel_ssc_put_audio(0);
        return ret;
    }
    (*AT91SAM9G20EK_DAI.codecs).of_node = codec_np;

    /* Parse dai and platform info */
    (*AT91SAM9G20EK_DAI.cpus).dai_name = core::ptr::null();
    (*AT91SAM9G20EK_DAI.platforms).name = core::ptr::null();
    cpu_np = of_parse_phandle(
        np,
        b"atmel,ssc-controller\0".as_ptr() as *const core::ffi::c_char,
        0,
    );
    if cpu_np.is_null() {
        dev_err(
            &mut (*pdev).dev,
            b"dai and pcm info missing\n\0".as_ptr() as *const core::ffi::c_char,
        );
        of_node_put(codec_np);
        ret = -EINVAL;
        atmel_ssc_put_audio(0);
        return ret;
    }
    (*AT91SAM9G20EK_DAI.cpus).of_node = cpu_np;
    (*AT91SAM9G20EK_DAI.platforms).of_node = cpu_np;

    of_node_put(codec_np);
    of_node_put(cpu_np);

    ret = snd_soc_register_card(card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"snd_soc_register_card() failed\n\0".as_ptr() as *const core::ffi::c_char,
        );
        atmel_ssc_put_audio(0);
        return ret;
    }

    0
}

unsafe extern "C" fn at91sam9g20ek_audio_remove(pdev: *mut platform_device) {
    let card: *mut snd_soc_card = platform_get_drvdata(pdev);

    snd_soc_unregister_card(card);
    atmel_ssc_put_audio(0);
}

// Original C condition: #ifdef CONFIG_OF
static AT91SAM9G20EK_WM8731_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"atmel,at91sam9g20ek-wm8731-audio\0".as_ptr()
            as *const core::ffi::c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, at91sam9g20ek_wm8731_dt_ids);

static mut AT91SAM9G20EK_AUDIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"at91sam9g20ek-audio\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: AT91SAM9G20EK_WM8731_DT_IDS.as_ptr(),
    },
    probe: Some(at91sam9g20ek_audio_probe),
    remove: Some(at91sam9g20ek_audio_remove),
};

// module_platform_driver(at91sam9g20ek_audio_driver);

/* Module information */
// MODULE_AUTHOR("Sedji Gaouaou <sedji.gaouaou@atmel.com>");
// MODULE_DESCRIPTION("ALSA SoC AT91SAM9G20EK_WM8731");
// MODULE_ALIAS("platform:at91sam9g20ek-audio");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
