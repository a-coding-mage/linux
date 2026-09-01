// SPDX-License-Identifier: GPL-2.0-only
//
// ASoC DPCM Machine driver for Baytrail / Cherrytrail platforms with
// CX2072X codec
//

use core::ffi::{c_char, c_int, c_void};

// C dependencies translated from includes:
// linux/acpi.h, linux/device.h, linux/gpio/consumer.h, linux/module.h,
// linux/platform_device.h, linux/slab.h, sound/pcm.h, sound/pcm_params.h,
// sound/jack.h, sound/soc.h, sound/soc-acpi.h, codecs/cx2072x.h,
// atom/sst-atom-controls.h.

const NULL: *mut c_void = core::ptr::null_mut();

static byt_cht_cx2072x_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_HP!("Headphone", NULL),
    SND_SOC_DAPM_MIC!("Headset Mic", NULL),
    SND_SOC_DAPM_MIC!("Int Mic", NULL),
    SND_SOC_DAPM_MIC!("Ext Spk", NULL),
];

static byt_cht_cx2072x_audio_map: [snd_soc_dapm_route; 10] = [
    /* External Speakers: HFL, HFR */
    snd_soc_dapm_route {
        sink: c_str!("Headphone"),
        control: core::ptr::null(),
        source: c_str!("PORTA"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Ext Spk"),
        control: core::ptr::null(),
        source: c_str!("PORTG"),
    },
    snd_soc_dapm_route {
        sink: c_str!("PORTC"),
        control: core::ptr::null(),
        source: c_str!("Int Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("PORTD"),
        control: core::ptr::null(),
        source: c_str!("Headset Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Playback"),
        control: core::ptr::null(),
        source: c_str!("ssp2 Tx"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Tx"),
        control: core::ptr::null(),
        source: c_str!("codec_out0"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Tx"),
        control: core::ptr::null(),
        source: c_str!("codec_out1"),
    },
    snd_soc_dapm_route {
        sink: c_str!("codec_in0"),
        control: core::ptr::null(),
        source: c_str!("ssp2 Rx"),
    },
    snd_soc_dapm_route {
        sink: c_str!("codec_in1"),
        control: core::ptr::null(),
        source: c_str!("ssp2 Rx"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Rx"),
        control: core::ptr::null(),
        source: c_str!("Capture"),
    },
];

static byt_cht_cx2072x_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH!("Headphone"),
    SOC_DAPM_PIN_SWITCH!("Headset Mic"),
    SOC_DAPM_PIN_SWITCH!("Int Mic"),
    SOC_DAPM_PIN_SWITCH!("Ext Spk"),
];

static mut byt_cht_cx2072x_headset: snd_soc_jack = unsafe { core::mem::zeroed() };

/* Headset jack detection DAPM pins */
static mut byt_cht_cx2072x_headset_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c_str!("Headset Mic"),
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: c_str!("Headphone"),
        mask: SND_JACK_HEADPHONE,
    },
];

static byt_cht_cx2072x_headset_gpios: acpi_gpio_params = unsafe { core::mem::zeroed() };
static byt_cht_cx2072x_acpi_gpios: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping {
        name: c_str!("headset-gpios"),
        data: &byt_cht_cx2072x_headset_gpios,
        size: 1,
    },
    acpi_gpio_mapping::zeroed(),
];

unsafe extern "C" fn byt_cht_cx2072x_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let codec: *mut snd_soc_component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let mut ret: c_int;

    if devm_acpi_dev_add_driver_gpios((*codec).dev, byt_cht_cx2072x_acpi_gpios.as_ptr()) != 0 {
        dev_warn((*rtd).dev, c_str!("Unable to add GPIO mapping table\n"));
    }

    snd_soc_dapm_set_idle_bias(dapm, false);

    /* set the default PLL rate, the clock is handled by the codec driver */
    ret = snd_soc_dai_set_sysclk(
        snd_soc_rtd_to_codec(rtd, 0),
        CX2072X_MCLK_EXTERNAL_PLL,
        19200000,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err((*rtd).dev, c_str!("Could not set sysclk\n"));
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c_str!("Headset"),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &raw mut byt_cht_cx2072x_headset,
        byt_cht_cx2072x_headset_pins.as_mut_ptr(),
        byt_cht_cx2072x_headset_pins.len() as c_int,
    );
    if ret != 0 {
        return ret;
    }

    snd_soc_component_set_jack(codec, &raw mut byt_cht_cx2072x_headset, core::ptr::null_mut());

    snd_soc_dai_set_bclk_ratio(snd_soc_rtd_to_codec(rtd, 0), 50);

    0
}

unsafe extern "C" fn byt_cht_cx2072x_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels: *mut snd_interval = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut ret: c_int;

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSP2 to 24-bit */
    params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);

    /*
     * Default mode for SSP configuration is TDM 4 slot, override config
     * with explicit setting to I2S 2ch 24-bit. The word length is set with
     * dai_set_tdm_slot() since there is no other API exposed
     */
    ret = snd_soc_dai_set_fmt(
        snd_soc_rtd_to_cpu(rtd, 0),
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c_str!("can't set format to I2S, err %d\n"), ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, 24);
    if ret < 0 {
        dev_err((*rtd).dev, c_str!("can't set I2S config, err %d\n"), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn byt_cht_cx2072x_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static byt_cht_cx2072x_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(byt_cht_cx2072x_aif1_startup),
};

SND_SOC_DAILINK_DEF!(dummy, DAILINK_COMP_ARRAY!(COMP_DUMMY!()));

SND_SOC_DAILINK_DEF!(media, DAILINK_COMP_ARRAY!(COMP_CPU!("media-cpu-dai")));

SND_SOC_DAILINK_DEF!(deepbuffer, DAILINK_COMP_ARRAY!(COMP_CPU!("deepbuffer-cpu-dai")));

SND_SOC_DAILINK_DEF!(ssp2, DAILINK_COMP_ARRAY!(COMP_CPU!("ssp2-port")));

SND_SOC_DAILINK_DEF!(
    cx2072x,
    DAILINK_COMP_ARRAY!(COMP_CODEC!("i2c-14F10720:00", "cx2072x-hifi"))
);

SND_SOC_DAILINK_DEF!(
    platform,
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!("sst-mfld-platform"))
);

static mut byt_cht_cx2072x_dais: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c_str!("Audio Port"),
        stream_name: c_str!("Audio"),
        nonatomic: true,
        dynamic: 1,
        ops: &byt_cht_cx2072x_aif1_ops,
        SND_SOC_DAILINK_REG!(media, dummy, platform)
    },
    snd_soc_dai_link {
        name: c_str!("Deep-Buffer Audio Port"),
        stream_name: c_str!("Deep-Buffer Audio"),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &byt_cht_cx2072x_aif1_ops,
        SND_SOC_DAILINK_REG!(deepbuffer, dummy, platform)
    },
    /* back ends */
    snd_soc_dai_link {
        name: c_str!("SSP2-Codec"),
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        init: Some(byt_cht_cx2072x_init),
        be_hw_params_fixup: Some(byt_cht_cx2072x_fixup),
        SND_SOC_DAILINK_REG!(ssp2, cx2072x, platform)
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c_str!("bytcht cx2072x"); /* card name will be 'sof-bytcht cx2072x' */
const SOF_DRIVER_NAME: *const c_char = c_str!("SOF");

const CARD_NAME: *const c_char = c_str!("bytcht-cx2072x");
const DRIVER_NAME: *const c_char = core::ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut byt_cht_cx2072x_card: snd_soc_card = snd_soc_card {
    name: CARD_NAME,
    driver_name: DRIVER_NAME,
    owner: THIS_MODULE,
    dai_link: unsafe { byt_cht_cx2072x_dais.as_mut_ptr() },
    num_links: 3,
    dapm_widgets: byt_cht_cx2072x_widgets.as_ptr(),
    num_dapm_widgets: 4,
    dapm_routes: byt_cht_cx2072x_audio_map.as_ptr(),
    num_dapm_routes: 10,
    controls: byt_cht_cx2072x_controls.as_ptr(),
    num_controls: 4,
};

static mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];

unsafe extern "C" fn snd_byt_cht_cx2072x_probe(pdev: *mut platform_device) -> c_int {
    let mut mach: *mut snd_soc_acpi_mach;
    let mut adev: *mut acpi_device;
    let mut dai_index: c_int = 0;
    let sof_parent: bool;
    let mut i: c_int;
    let mut ret: c_int;

    byt_cht_cx2072x_card.dev = &raw mut (*pdev).dev;
    mach = dev_get_platdata(&raw mut (*pdev).dev) as *mut snd_soc_acpi_mach;

    /* fix index of codec dai */
    i = 0;
    while i < byt_cht_cx2072x_dais.len() as c_int {
        if byt_cht_cx2072x_dais[i as usize].num_codecs != 0
            && strcmp(
                (*byt_cht_cx2072x_dais[i as usize].codecs).name,
                c_str!("i2c-14F10720:00"),
            ) == 0
        {
            dai_index = i;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, core::ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            codec_name.as_mut_ptr(),
            codec_name.len(),
            c_str!("i2c-%s"),
            acpi_dev_name(adev),
        );
        (*byt_cht_cx2072x_dais[dai_index as usize].codecs).name = codec_name.as_mut_ptr();
    } else {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Error cannot find '%s' dev\n"),
            (*mach).id,
        );
        return -ENOENT;
    }

    acpi_dev_put(adev);

    /* override platform name, if required */
    ret = snd_soc_fixup_dai_links_platform_name(
        &raw mut byt_cht_cx2072x_card,
        (*mach).mach_params.platform,
    );
    if ret != 0 {
        return ret;
    }

    sof_parent = snd_soc_acpi_sof_parent(&raw mut (*pdev).dev);

    /* set card and driver name */
    if sof_parent {
        byt_cht_cx2072x_card.name = SOF_CARD_NAME;
        byt_cht_cx2072x_card.driver_name = SOF_DRIVER_NAME;
    } else {
        byt_cht_cx2072x_card.name = CARD_NAME;
        byt_cht_cx2072x_card.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*pdev).dev.driver).pm = &snd_soc_pm_ops;
    }

    devm_snd_soc_register_card(&raw mut (*pdev).dev, &raw mut byt_cht_cx2072x_card)
}

static mut snd_byt_cht_cx2072x_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("bytcht_cx2072x"),
    },
    probe: Some(snd_byt_cht_cx2072x_probe),
};

module_platform_driver!(snd_byt_cht_cx2072x_driver);

MODULE_DESCRIPTION!("ASoC Intel(R) Baytrail/Cherrytrail Machine driver");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:bytcht_cx2072x");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
