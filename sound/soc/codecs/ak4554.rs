// SPDX-License-Identifier: GPL-2.0
// ak4554.c
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

// C dependencies: <linux/module.h>, <sound/soc.h>

use core::ffi::{c_char, c_int};
use core::ptr::null;

extern "C" {
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

/*
 * ak4554 is very simple DA/AD converter which has no setting register.
 *
 * CAUTION
 *
 * ak4554 playback format is SND_SOC_DAIFMT_RIGHT_J,
 * and,   capture  format is SND_SOC_DAIFMT_LEFT_J
 * on same bit clock, LR clock.
 * But, this driver doesn't have snd_soc_dai_ops :: set_fmt
 *
 * CPU/Codec DAI image
 *
 * CPU-DAI1 (plaback only fmt = RIGHT_J) --+-- ak4554
 *					   |
 * CPU-DAI2 (capture only fmt = LEFT_J) ---+
 */

static ak4554_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_INPUT!(c"AINL"),
    SND_SOC_DAPM_INPUT!(c"AINR"),
    SND_SOC_DAPM_OUTPUT!(c"AOUTL"),
    SND_SOC_DAPM_OUTPUT!(c"AOUTR"),
];

static ak4554_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: null(),
        source: c"AINL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Capture".as_ptr(),
        control: null(),
        source: c"AINR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"AOUTL".as_ptr(),
        control: null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"AOUTR".as_ptr(),
        control: null(),
        source: c"Playback".as_ptr(),
    },
];

static mut ak4554_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"ak4554-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    symmetric_rate: 1,
};

static soc_component_dev_ak4554: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: ak4554_dapm_widgets.as_ptr(),
    num_dapm_widgets: ak4554_dapm_widgets.len(),
    dapm_routes: ak4554_dapm_routes.as_ptr(),
    num_dapm_routes: ak4554_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ak4554_soc_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_component_dev_ak4554,
        &mut ak4554_dai,
        1,
    )
}

static ak4554_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"asahi-kasei,ak4554".as_ptr(),
    },
    of_device_id {
        compatible: null::<c_char>(),
    },
];
MODULE_DEVICE_TABLE!(of, ak4554_of_match);

static mut ak4554_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"ak4554-adc-dac".as_ptr(),
        of_match_table: ak4554_of_match.as_ptr(),
    },
    probe: Some(ak4554_soc_probe),
};
module_platform_driver!(ak4554_driver);

MODULE_LICENSE!(c"GPL v2");
MODULE_DESCRIPTION!(c"SoC AK4554 driver");
MODULE_AUTHOR!(c"Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
