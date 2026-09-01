// SPDX-License-Identifier: GPL-2.0-only
/*
 *  bytcht_es8316.c - ASoc Machine driver for Intel Baytrail/Cherrytrail
 *                    platforms with Everest ES8316 SoC
 *
 *  Copyright (C) 2017 Endless Mobile, Inc.
 *  Authors: David Yang <yangxiaohua@everest-semi.com>,
 *           Daniel Drake <drake@endlessm.com>
 *
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

/* Dependencies from Linux, ALSA SoC, ES83xx DSM, and Intel SST headers. */

const MAX_NO_PROPS: usize = 2; /* jd-inv + terminating entry */

#[repr(C)]
struct byt_cht_es8316_private {
    mclk: *mut clk,
    jack: snd_soc_jack,
    speaker_en_gpio: *mut gpio_desc,
    codec_dev: *mut device,
    speaker_en: bool,
    mclk_enabled: bool,
}

const BYT_CHT_ES8316_INTMIC_IN1_MAP: c_ulong = 0;
const BYT_CHT_ES8316_INTMIC_IN2_MAP: c_ulong = 1;

const BYT_CHT_ES8316_MAP_MASK: c_ulong = genmask(3, 0);
const BYT_CHT_ES8316_SSP0: c_ulong = bit(16);
const BYT_CHT_ES8316_MONO_SPEAKER: c_ulong = bit(17);
const BYT_CHT_ES8316_JD_INVERTED: c_ulong = bit(18);

const fn bit(n: u32) -> c_ulong {
    1usize.wrapping_shl(n) as c_ulong
}

const fn genmask(h: u32, l: u32) -> c_ulong {
    let width = h - l + 1;
    (((1usize.wrapping_shl(width)) - 1).wrapping_shl(l)) as c_ulong
}

const fn BYT_CHT_ES8316_MAP(quirk: c_ulong) -> c_ulong {
    quirk & BYT_CHT_ES8316_MAP_MASK
}

static mut quirk: c_ulong = 0;

static mut quirk_override: c_int = -1;
/* module_param_named(quirk, quirk_override, int, 0444); */
/* MODULE_PARM_DESC(quirk, "Board-specific quirk override"); */

unsafe fn log_quirks(dev: *mut device) {
    let map: c_int = BYT_CHT_ES8316_MAP(quirk) as c_int;

    match map as c_ulong {
        BYT_CHT_ES8316_INTMIC_IN1_MAP => {
            dev_info(dev, c"quirk IN1_MAP enabled".as_ptr());
        }
        BYT_CHT_ES8316_INTMIC_IN2_MAP => {
            dev_info(dev, c"quirk IN2_MAP enabled".as_ptr());
        }
        _ => {
            dev_warn_once(
                dev,
                c"quirk sets invalid input map: 0x%x, default to INTMIC_IN1_MAP\n".as_ptr(),
                map,
            );
            quirk &= !BYT_CHT_ES8316_MAP_MASK;
            quirk |= BYT_CHT_ES8316_INTMIC_IN1_MAP;
        }
    }

    if (quirk & BYT_CHT_ES8316_SSP0) != 0 {
        dev_info(dev, c"quirk SSP0 enabled".as_ptr());
    }
    if (quirk & BYT_CHT_ES8316_MONO_SPEAKER) != 0 {
        dev_info(dev, c"quirk MONO_SPEAKER enabled\n".as_ptr());
    }
    if (quirk & BYT_CHT_ES8316_JD_INVERTED) != 0 {
        dev_info(dev, c"quirk JD_INVERTED enabled\n".as_ptr());
    }
}

unsafe extern "C" fn byt_cht_es8316_speaker_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_cht_es8316_private;

    if SND_SOC_DAPM_EVENT_ON(event) {
        (*priv_).speaker_en = true;
    } else {
        (*priv_).speaker_en = false;
    }

    gpiod_set_value_cansleep((*priv_).speaker_en_gpio, (*priv_).speaker_en as c_int);

    0
}

static byt_cht_es8316_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_SPK(c"Speaker".as_ptr(), ptr::null()),
    SND_SOC_DAPM_HP(c"Headphone".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Headset Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_MIC(c"Internal Mic".as_ptr(), ptr::null()),
    SND_SOC_DAPM_SUPPLY(
        c"Speaker Power".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(byt_cht_es8316_speaker_power_event),
        SND_SOC_DAPM_PRE_PMD | SND_SOC_DAPM_POST_PMU,
    ),
];

static byt_cht_es8316_audio_map: [snd_soc_dapm_route; 5] = [
    route(c"Headphone".as_ptr(), ptr::null(), c"HPOL".as_ptr()),
    route(c"Headphone".as_ptr(), ptr::null(), c"HPOR".as_ptr()),
    /*
     * There is no separate speaker output instead the speakers are muxed to
     * the HP outputs. The mux is controlled by the "Speaker Power" supply.
     */
    route(c"Speaker".as_ptr(), ptr::null(), c"HPOL".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"HPOR".as_ptr()),
    route(c"Speaker".as_ptr(), ptr::null(), c"Speaker Power".as_ptr()),
];

static byt_cht_es8316_intmic_in1_map: [snd_soc_dapm_route; 2] = [
    route(c"MIC1".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"MIC2".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_cht_es8316_intmic_in2_map: [snd_soc_dapm_route; 2] = [
    route(c"MIC2".as_ptr(), ptr::null(), c"Internal Mic".as_ptr()),
    route(c"MIC1".as_ptr(), ptr::null(), c"Headset Mic".as_ptr()),
];

static byt_cht_es8316_ssp0_map: [snd_soc_dapm_route; 4] = [
    route(c"Playback".as_ptr(), ptr::null(), c"ssp0 Tx".as_ptr()),
    route(c"ssp0 Tx".as_ptr(), ptr::null(), c"modem_out".as_ptr()),
    route(c"modem_in".as_ptr(), ptr::null(), c"ssp0 Rx".as_ptr()),
    route(c"ssp0 Rx".as_ptr(), ptr::null(), c"Capture".as_ptr()),
];

static byt_cht_es8316_ssp2_map: [snd_soc_dapm_route; 6] = [
    route(c"Playback".as_ptr(), ptr::null(), c"ssp2 Tx".as_ptr()),
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out0".as_ptr()),
    route(c"ssp2 Tx".as_ptr(), ptr::null(), c"codec_out1".as_ptr()),
    route(c"codec_in0".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"codec_in1".as_ptr(), ptr::null(), c"ssp2 Rx".as_ptr()),
    route(c"ssp2 Rx".as_ptr(), ptr::null(), c"Capture".as_ptr()),
];

static byt_cht_es8316_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_PIN_SWITCH(c"Speaker".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headphone".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Headset Mic".as_ptr()),
    SOC_DAPM_PIN_SWITCH(c"Internal Mic".as_ptr()),
];

static mut byt_cht_es8316_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: c"Headphone".as_ptr(),
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: c"Headset Mic".as_ptr(),
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe fn byt_cht_es8316_disable_mclk(priv_: *mut byt_cht_es8316_private) {
    if !(*priv_).mclk_enabled {
        return;
    }

    clk_disable_unprepare((*priv_).mclk);
    (*priv_).mclk_enabled = false;
}

unsafe extern "C" fn byt_cht_es8316_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let codec = (*snd_soc_rtd_to_codec(runtime, 0)).component;
    let card = (*runtime).card;
    let dapm = snd_soc_card_to_dapm(card);
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_cht_es8316_private;
    let mut custom_map: *const snd_soc_dapm_route;
    let mut num_routes: c_int;
    let mut ret: c_int;

    snd_soc_dapm_set_idle_bias(dapm, false);

    match BYT_CHT_ES8316_MAP(quirk) {
        BYT_CHT_ES8316_INTMIC_IN1_MAP => {
            custom_map = byt_cht_es8316_intmic_in1_map.as_ptr();
            num_routes = byt_cht_es8316_intmic_in1_map.len() as c_int;
        }
        BYT_CHT_ES8316_INTMIC_IN2_MAP => {
            custom_map = byt_cht_es8316_intmic_in2_map.as_ptr();
            num_routes = byt_cht_es8316_intmic_in2_map.len() as c_int;
        }
        _ => {
            custom_map = byt_cht_es8316_intmic_in1_map.as_ptr();
            num_routes = byt_cht_es8316_intmic_in1_map.len() as c_int;
        }
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    if (quirk & BYT_CHT_ES8316_SSP0) != 0 {
        custom_map = byt_cht_es8316_ssp0_map.as_ptr();
        num_routes = byt_cht_es8316_ssp0_map.len() as c_int;
    } else {
        custom_map = byt_cht_es8316_ssp2_map.as_ptr();
        num_routes = byt_cht_es8316_ssp2_map.len() as c_int;
    }
    ret = snd_soc_dapm_add_routes(dapm, custom_map, num_routes);
    if ret != 0 {
        return ret;
    }

    /*
     * The firmware might enable the clock at boot (this information
     * may or may not be reflected in the enable clock register).
     * To change the rate we must disable the clock first to cover these
     * cases. Due to common clock framework restrictions that do not allow
     * to disable a clock that has not been enabled, we need to enable
     * the clock first.
     */
    ret = clk_prepare_enable((*priv_).mclk);
    if ret == 0 {
        clk_disable_unprepare((*priv_).mclk);
    }

    ret = clk_set_rate((*priv_).mclk, 19200000);
    if ret != 0 {
        dev_err((*card).dev, c"unable to set MCLK rate\n".as_ptr());
    }

    ret = clk_prepare_enable((*priv_).mclk);
    if ret != 0 {
        dev_err((*card).dev, c"unable to enable MCLK\n".as_ptr());
    } else {
        (*priv_).mclk_enabled = true;
    }

    ret = snd_soc_dai_set_sysclk(
        snd_soc_rtd_to_codec(runtime, 0),
        0,
        19200000,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*card).dev, c"can't set codec clock %d\n".as_ptr(), ret);
        byt_cht_es8316_disable_mclk(priv_);
        return ret;
    }

    ret = snd_soc_card_jack_new_pins(
        card,
        c"Headset".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0,
        &mut (*priv_).jack,
        byt_cht_es8316_jack_pins.as_mut_ptr(),
        byt_cht_es8316_jack_pins.len() as c_int,
    );
    if ret != 0 {
        dev_err((*card).dev, c"jack creation failed %d\n".as_ptr(), ret);
        byt_cht_es8316_disable_mclk(priv_);
        return ret;
    }

    snd_jack_set_key((*(*priv_).jack.jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_soc_component_set_jack(codec, &mut (*priv_).jack, ptr::null_mut());

    0
}

unsafe extern "C" fn byt_cht_es8316_exit(runtime: *mut snd_soc_pcm_runtime) {
    let card = (*runtime).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_cht_es8316_private;

    byt_cht_es8316_disable_mclk(priv_);
}

unsafe extern "C" fn byt_cht_es8316_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut ret: c_int;
    let bits: c_int;

    /* The DSP will convert the FE rate to 48k, stereo */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    if (quirk & BYT_CHT_ES8316_SSP0) != 0 {
        /* set SSP0 to 16-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
        bits = 16;
    } else {
        /* set SSP2 to 24-bit */
        params_set_format(params, SNDRV_PCM_FORMAT_S24_LE);
        bits = 24;
    }

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
        dev_err((*rtd).dev, c"can't set format to I2S, err %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, bits);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set I2S config, err %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn byt_cht_es8316_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

static byt_cht_es8316_aif1_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(byt_cht_es8316_aif1_startup),
};

/* SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY())); */
/* SND_SOC_DAILINK_DEF(media, DAILINK_COMP_ARRAY(COMP_CPU("media-cpu-dai"))); */
/* SND_SOC_DAILINK_DEF(deepbuffer, DAILINK_COMP_ARRAY(COMP_CPU("deepbuffer-cpu-dai"))); */
/* SND_SOC_DAILINK_DEF(ssp2_port, DAILINK_COMP_ARRAY(COMP_CPU("ssp2-port"))); */
/* SND_SOC_DAILINK_DEF(ssp2_codec, DAILINK_COMP_ARRAY(COMP_CODEC("i2c-ESSX8316:00", "ES8316 HiFi"))); */
/* SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sst-mfld-platform"))); */

static mut dummy: [snd_soc_dai_link_component; 1] = [COMP_DUMMY()];
static mut media: [snd_soc_dai_link_component; 1] = [COMP_CPU(c"media-cpu-dai".as_ptr())];
static mut deepbuffer: [snd_soc_dai_link_component; 1] =
    [COMP_CPU(c"deepbuffer-cpu-dai".as_ptr())];
static mut ssp2_port: [snd_soc_dai_link_component; 1] = [COMP_CPU(c"ssp2-port".as_ptr())];
static mut ssp2_codec: [snd_soc_dai_link_component; 1] =
    [COMP_CODEC(c"i2c-ESSX8316:00".as_ptr(), c"ES8316 HiFi".as_ptr())];
static mut platform: [snd_soc_dai_link_component; 1] =
    [COMP_PLATFORM(c"sst-mfld-platform".as_ptr())];

static mut byt_cht_es8316_dais: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"Audio Port".as_ptr(),
        stream_name: c"Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        ops: &byt_cht_es8316_aif1_ops,
        cpus: unsafe { media.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
    snd_soc_dai_link {
        name: c"Deep-Buffer Audio Port".as_ptr(),
        stream_name: c"Deep-Buffer Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &byt_cht_es8316_aif1_ops,
        cpus: unsafe { deepbuffer.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { dummy.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
    /* back ends */
    snd_soc_dai_link {
        name: c"SSP2-Codec".as_ptr(),
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        be_hw_params_fixup: Some(byt_cht_es8316_codec_fixup),
        init: Some(byt_cht_es8316_init),
        exit: Some(byt_cht_es8316_exit),
        cpus: unsafe { ssp2_port.as_mut_ptr() },
        num_cpus: 1,
        codecs: unsafe { ssp2_codec.as_mut_ptr() },
        num_codecs: 1,
        platforms: unsafe { platform.as_mut_ptr() },
        num_platforms: 1,
        ..snd_soc_dai_link::zeroed()
    },
];

/* SoC card */
static mut codec_name: [c_char; SND_ACPI_I2C_ID_LEN] = [0; SND_ACPI_I2C_ID_LEN];
/* if !IS_ENABLED(CONFIG_SND_SOC_INTEL_USER_FRIENDLY_LONG_NAMES) */
static mut long_name: [c_char; 50] = [0; 50]; /* = "bytcht-es8316-*-spk-*-mic" */
static mut components_string: [c_char; 32] = [0; 32]; /* = "cfg-spk:* cfg-mic:*" */

unsafe extern "C" fn byt_cht_es8316_suspend(card: *mut snd_soc_card) -> c_int {
    let mut component: *mut snd_soc_component = ptr::null_mut();

    for_each_card_components(card, &mut component, |component| {
        if strcmp((*component).name, codec_name.as_ptr()) == 0 {
            dev_dbg(
                (*component).dev,
                c"disabling jack detect before suspend\n".as_ptr(),
            );
            snd_soc_component_set_jack(component, ptr::null_mut(), ptr::null_mut());
            return false;
        }
        true
    });

    0
}

unsafe extern "C" fn byt_cht_es8316_resume(card: *mut snd_soc_card) -> c_int {
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_cht_es8316_private;
    let mut component: *mut snd_soc_component = ptr::null_mut();

    for_each_card_components(card, &mut component, |component| {
        if strcmp((*component).name, codec_name.as_ptr()) == 0 {
            dev_dbg(
                (*component).dev,
                c"re-enabling jack detect after resume\n".as_ptr(),
            );
            snd_soc_component_set_jack(component, &mut (*priv_).jack, ptr::null_mut());
            return false;
        }
        true
    });

    /*
     * Some Cherry Trail boards with an ES8316 codec have a bug in their
     * ACPI tables where the MSSL1680 touchscreen's _PS0 and _PS3 methods
     * wrongly also set the speaker-enable GPIO to 1/0. Testing has shown
     * that this really is a bug and the GPIO has no influence on the
     * touchscreen at all.
     *
     * The silead.c touchscreen driver does not support runtime suspend, so
     * the GPIO can only be changed underneath us during a system suspend.
     * This resume() function runs from a pm complete() callback, and thus
     * is guaranteed to run after the touchscreen driver/ACPI-subsys has
     * brought the touchscreen back up again (and thus changed the GPIO).
     *
     * So to work around this we pass GPIOD_FLAGS_BIT_NONEXCLUSIVE when
     * requesting the GPIO and we set its value here to undo any changes
     * done by the touchscreen's broken _PS0 ACPI method.
     */
    gpiod_set_value_cansleep((*priv_).speaker_en_gpio, (*priv_).speaker_en as c_int);

    0
}

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bytcht es8316".as_ptr(); /* card name will be 'sof-bytcht es8316' */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"bytcht-es8316".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

static mut byt_cht_es8316_card: snd_soc_card = snd_soc_card {
    owner: THIS_MODULE,
    dai_link: unsafe { byt_cht_es8316_dais.as_mut_ptr() },
    num_links: unsafe { byt_cht_es8316_dais.len() as c_int },
    dapm_widgets: byt_cht_es8316_widgets.as_ptr(),
    num_dapm_widgets: byt_cht_es8316_widgets.len() as c_int,
    dapm_routes: byt_cht_es8316_audio_map.as_ptr(),
    num_dapm_routes: byt_cht_es8316_audio_map.len() as c_int,
    controls: byt_cht_es8316_controls.as_ptr(),
    num_controls: byt_cht_es8316_controls.len() as c_int,
    fully_routed: true,
    suspend_pre: Some(byt_cht_es8316_suspend),
    resume_post: Some(byt_cht_es8316_resume),
    ..snd_soc_card::zeroed()
};

static first_gpio: acpi_gpio_params = acpi_gpio_params {
    crs_entry_index: 0,
    line_index: 0,
    active_low: false,
};

static byt_cht_es8316_gpios: [acpi_gpio_mapping; 2] = [
    acpi_gpio_mapping {
        name: c"speaker-enable-gpios".as_ptr(),
        data: &first_gpio,
        size: 1,
    },
    acpi_gpio_mapping::zeroed(),
];

/* Please keep this list alphabetically sorted */
static byt_cht_es8316_quirk_table: [dmi_system_id; 4] = [
    dmi_system_id {
        /* Irbis NB41 */
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"IRBIS".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_NAME, c"NB41".as_ptr()),
            dmi_strmatch::zeroed(),
            dmi_strmatch::zeroed(),
        ],
        driver_data: (BYT_CHT_ES8316_SSP0
            | BYT_CHT_ES8316_INTMIC_IN2_MAP
            | BYT_CHT_ES8316_JD_INVERTED) as *mut c_void,
        ..dmi_system_id::zeroed()
    },
    dmi_system_id {
        /* Nanote UMPC-01 */
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"RWC CO.,LTD".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_NAME, c"UMPC-01".as_ptr()),
            dmi_strmatch::zeroed(),
            dmi_strmatch::zeroed(),
        ],
        driver_data: BYT_CHT_ES8316_INTMIC_IN1_MAP as *mut c_void,
        ..dmi_system_id::zeroed()
    },
    dmi_system_id {
        /* Teclast X98 Plus II */
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"TECLAST".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_NAME, c"X98 Plus II".as_ptr()),
            dmi_strmatch::zeroed(),
            dmi_strmatch::zeroed(),
        ],
        driver_data: (BYT_CHT_ES8316_INTMIC_IN1_MAP | BYT_CHT_ES8316_JD_INVERTED)
            as *mut c_void,
        ..dmi_system_id::zeroed()
    },
    dmi_system_id::zeroed(),
];

unsafe fn byt_cht_es8316_get_quirks_from_dsm(
    priv_: *mut byt_cht_es8316_private,
    is_bytcr: bool,
) -> c_int {
    let mut ret: c_int;
    let mut val1: c_int = 0;
    let mut val2: c_int = 0;
    let mut dsm_quirk: c_int = 0;

    if is_bytcr {
        dsm_quirk |= BYT_CHT_ES8316_SSP0 as c_int;
    }

    ret = es83xx_dsm((*priv_).codec_dev, PLATFORM_MAINMIC_TYPE_ARG, &mut val1);
    if ret < 0 {
        return ret;
    }

    ret = es83xx_dsm((*priv_).codec_dev, PLATFORM_HPMIC_TYPE_ARG, &mut val2);
    if ret < 0 {
        return ret;
    }

    if val1 == PLATFORM_MIC_AMIC_LIN1RIN1 && val2 == PLATFORM_MIC_AMIC_LIN2RIN2 {
        dsm_quirk |= BYT_CHT_ES8316_INTMIC_IN1_MAP as c_int;
    } else if val1 == PLATFORM_MIC_AMIC_LIN2RIN2 && val2 == PLATFORM_MIC_AMIC_LIN1RIN1 {
        dsm_quirk |= BYT_CHT_ES8316_INTMIC_IN2_MAP as c_int;
    } else {
        dev_warn(
            (*priv_).codec_dev,
            c"Unknown mic settings mainmic 0x%02x hpmic 0x%02x\n".as_ptr(),
            val1,
            val2,
        );
        return -EINVAL;
    }

    ret = es83xx_dsm((*priv_).codec_dev, PLATFORM_SPK_TYPE_ARG, &mut val1);
    if ret < 0 {
        return ret;
    }

    match val1 {
        PLATFORM_SPK_MONO => {
            dsm_quirk |= BYT_CHT_ES8316_MONO_SPEAKER as c_int;
        }
        PLATFORM_SPK_STEREO => {}
        _ => {
            dev_warn((*priv_).codec_dev, c"Unknown speaker setting 0x%02x\n".as_ptr(), val1);
            return -EINVAL;
        }
    }

    ret = es83xx_dsm((*priv_).codec_dev, PLATFORM_HPDET_INV_ARG, &mut val1);
    if ret < 0 {
        return ret;
    }

    match val1 {
        PLATFORM_HPDET_NORMAL => {}
        PLATFORM_HPDET_INVERTED => {
            dsm_quirk |= BYT_CHT_ES8316_JD_INVERTED as c_int;
        }
        _ => {
            dev_warn((*priv_).codec_dev, c"Unknown hpdet-inv setting 0x%02x\n".as_ptr(), val1);
            return -EINVAL;
        }
    }

    quirk = dsm_quirk as c_ulong;
    0
}

unsafe extern "C" fn snd_byt_cht_es8316_mc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    static mic_name: [*const c_char; 2] = [c"in1".as_ptr(), c"in2".as_ptr()];
    let mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    let mut props: [property_entry; MAX_NO_PROPS] = [property_entry::zeroed(); MAX_NO_PROPS];
    let mut priv_: *mut byt_cht_es8316_private;
    let mut dmi_id: *const dmi_system_id;
    let mut fwnode: *mut fwnode_handle;
    let sof_parent: bool;
    let is_bytcr: bool;
    let platform_name: *const c_char;
    let mut adev: *mut acpi_device;
    let codec_dev: *mut device;
    let mut cnt: c_uint = 0;
    let mut dai_index: c_int = 0;
    let mut i: c_int;
    let mut ret: c_int = 0;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<byt_cht_es8316_private>(), GFP_KERNEL)
        as *mut byt_cht_es8316_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    /* fix index of codec dai */
    i = 0;
    while (i as usize) < byt_cht_es8316_dais.len() {
        if byt_cht_es8316_dais[i as usize].num_codecs != 0
            && strcmp(
                (*byt_cht_es8316_dais[i as usize].codecs).name,
                c"i2c-ESSX8316:00".as_ptr(),
            ) == 0
        {
            dai_index = i;
            break;
        }
        i += 1;
    }

    /* fixup codec name based on HID */
    adev = acpi_dev_get_first_match_dev((*mach).id, ptr::null(), -1);
    if !adev.is_null() {
        snprintf(
            codec_name.as_mut_ptr(),
            codec_name.len(),
            c"i2c-%s".as_ptr(),
            acpi_dev_name(adev),
        );
        (*byt_cht_es8316_dais[dai_index as usize].codecs).name = codec_name.as_ptr();
    } else {
        dev_err(dev, c"Error cannot find '%s' dev\n".as_ptr(), (*mach).id);
        return -ENOENT;
    }

    codec_dev = acpi_get_first_physical_node(adev);
    acpi_dev_put(adev);
    if codec_dev.is_null() {
        return -EPROBE_DEFER;
    }
    (*priv_).codec_dev = get_device(codec_dev);

    /* override platform name, if required */
    byt_cht_es8316_card.dev = dev;
    platform_name = (*mach).mach_params.platform;

    ret = snd_soc_fixup_dai_links_platform_name(&mut byt_cht_es8316_card, platform_name);
    if ret != 0 {
        put_device(codec_dev);
        return ret;
    }

    es83xx_dsm_dump((*priv_).codec_dev);

    /* Check for BYTCR or other platform and setup quirks */
    is_bytcr = soc_intel_is_byt() && (*mach).mach_params.acpi_ipc_irq_index == 0;
    dmi_id = dmi_first_match(byt_cht_es8316_quirk_table.as_ptr());
    if !dmi_id.is_null() {
        quirk = (*dmi_id).driver_data as c_ulong;
    } else if byt_cht_es8316_get_quirks_from_dsm(priv_, is_bytcr) == 0 {
        dev_info(dev, c"Using ACPI DSM info for quirks\n".as_ptr());
    } else if is_bytcr {
        /* On BYTCR default to SSP0, internal-mic-in2-map, mono-spk */
        quirk = BYT_CHT_ES8316_SSP0
            | BYT_CHT_ES8316_INTMIC_IN2_MAP
            | BYT_CHT_ES8316_MONO_SPEAKER;
    } else {
        /* Others default to internal-mic-in1-map, mono-speaker */
        quirk = BYT_CHT_ES8316_INTMIC_IN1_MAP | BYT_CHT_ES8316_MONO_SPEAKER;
    }
    if quirk_override != -1 {
        dev_info(
            dev,
            c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(),
            quirk,
            quirk_override,
        );
        quirk = quirk_override as c_ulong;
    }
    log_quirks(dev);

    if (quirk & BYT_CHT_ES8316_SSP0) != 0 {
        (*byt_cht_es8316_dais[dai_index as usize].cpus).dai_name = c"ssp0-port".as_ptr();
    }

    /* get the clock */
    (*priv_).mclk = devm_clk_get(dev, c"pmc_plt_clk_3".as_ptr());
    if IS_ERR((*priv_).mclk as *const c_void) {
        put_device(codec_dev);
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).mclk as *const c_void),
            c"clk_get pmc_plt_clk_3 failed\n".as_ptr(),
        );
    }

    if (quirk & BYT_CHT_ES8316_JD_INVERTED) != 0 {
        props[cnt as usize] = PROPERTY_ENTRY_BOOL(c"everest,jack-detect-inverted".as_ptr());
        cnt += 1;
    }

    if cnt != 0 {
        fwnode = fwnode_create_software_node(props.as_ptr(), ptr::null());
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
    devm_acpi_dev_add_driver_gpios(codec_dev, byt_cht_es8316_gpios.as_ptr());
    (*priv_).speaker_en_gpio = gpiod_get_optional(
        codec_dev,
        c"speaker-enable".as_ptr(),
        /* see comment in byt_cht_es8316_resume() */
        GPIOD_OUT_LOW | GPIOD_FLAGS_BIT_NONEXCLUSIVE,
    );
    if IS_ERR((*priv_).speaker_en_gpio as *const c_void) {
        ret = dev_err_probe(
            dev,
            PTR_ERR((*priv_).speaker_en_gpio as *const c_void),
            c"get speaker GPIO failed\n".as_ptr(),
        );
        device_remove_software_node((*priv_).codec_dev);
        put_device((*priv_).codec_dev);
        return ret;
    }

    snprintf(
        components_string.as_mut_ptr(),
        components_string.len(),
        c"cfg-spk:%s cfg-mic:%s".as_ptr(),
        if (quirk & BYT_CHT_ES8316_MONO_SPEAKER) != 0 {
            c"1".as_ptr()
        } else {
            c"2".as_ptr()
        },
        mic_name[BYT_CHT_ES8316_MAP(quirk) as usize],
    );
    byt_cht_es8316_card.components = components_string.as_ptr();
    /* if !IS_ENABLED(CONFIG_SND_SOC_INTEL_USER_FRIENDLY_LONG_NAMES) */
    snprintf(
        long_name.as_mut_ptr(),
        long_name.len(),
        c"bytcht-es8316-%s-spk-%s-mic".as_ptr(),
        if (quirk & BYT_CHT_ES8316_MONO_SPEAKER) != 0 {
            c"mono".as_ptr()
        } else {
            c"stereo".as_ptr()
        },
        mic_name[BYT_CHT_ES8316_MAP(quirk) as usize],
    );
    byt_cht_es8316_card.long_name = long_name.as_ptr();

    sof_parent = snd_soc_acpi_sof_parent(dev);

    /* set card and driver name */
    if sof_parent {
        byt_cht_es8316_card.name = SOF_CARD_NAME;
        byt_cht_es8316_card.driver_name = SOF_DRIVER_NAME;
    } else {
        byt_cht_es8316_card.name = CARD_NAME;
        byt_cht_es8316_card.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*dev).driver).pm = &snd_soc_pm_ops;
    }

    /* register the soc card */
    snd_soc_card_set_drvdata(&mut byt_cht_es8316_card, priv_ as *mut c_void);

    ret = devm_snd_soc_register_card(dev, &mut byt_cht_es8316_card);
    if ret != 0 {
        gpiod_put((*priv_).speaker_en_gpio);
        dev_err(dev, c"snd_soc_register_card failed: %d\n".as_ptr(), ret);
        device_remove_software_node((*priv_).codec_dev);
        put_device((*priv_).codec_dev);
        return ret;
    }
    platform_set_drvdata(pdev, &mut byt_cht_es8316_card as *mut _ as *mut c_void);
    0
}

unsafe extern "C" fn snd_byt_cht_es8316_mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut byt_cht_es8316_private;

    gpiod_put((*priv_).speaker_en_gpio);
    device_remove_software_node((*priv_).codec_dev);
    put_device((*priv_).codec_dev);
}

static mut snd_byt_cht_es8316_mc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"bytcht_es8316".as_ptr(),
        ..device_driver::zeroed()
    },
    probe: Some(snd_byt_cht_es8316_mc_probe),
    remove: Some(snd_byt_cht_es8316_mc_remove),
};

/* module_platform_driver(snd_byt_cht_es8316_mc_driver); */
/* MODULE_DESCRIPTION("ASoC Intel(R) Baytrail/Cherrytrail Machine driver"); */
/* MODULE_AUTHOR("David Yang <yangxiaohua@everest-semi.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:bytcht_es8316"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
