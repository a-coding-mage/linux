// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for generic Bluetooth SCO link
 * Copyright 2011 Lars-Peter Clausen <lars@metafoo.de>
 */

// C dependencies:
// #include <linux/init.h>
// #include <linux/module.h>
// #include <linux/platform_device.h>
// #include <sound/soc.h>

static BT_SCO_WIDGETS: &[snd_soc_dapm_widget] = &[
    SND_SOC_DAPM_INPUT!("RX"),
    SND_SOC_DAPM_OUTPUT!("TX"),
    SND_SOC_DAPM_AIF_IN!("BT_SCO_RX", "Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("BT_SCO_TX", "Capture", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("BT_SCO_RX_WB", "WB Playback", 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("BT_SCO_TX_WB", "WB Capture", 0, SND_SOC_NOPM, 0, 0),
];

static BT_SCO_ROUTES: &[snd_soc_dapm_route] = &[
    snd_soc_dapm_route {
        sink: c"BT_SCO_TX".as_ptr(),
        control: core::ptr::null(),
        source: c"RX".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"TX".as_ptr(),
        control: core::ptr::null(),
        source: c"BT_SCO_RX".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"BT_SCO_TX_WB".as_ptr(),
        control: core::ptr::null(),
        source: c"RX".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"TX".as_ptr(),
        control: core::ptr::null(),
        source: c"BT_SCO_RX_WB".as_ptr(),
    },
];

static mut BT_SCO_DAI: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"bt-sco-pcm".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"Playback".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"Capture".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c"bt-sco-pcm-wb".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"WB Playback".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"WB Capture".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

static SOC_COMPONENT_DEV_BT_SCO: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: BT_SCO_WIDGETS.as_ptr(),
    num_dapm_widgets: BT_SCO_WIDGETS.len() as _,
    dapm_routes: BT_SCO_ROUTES.as_ptr(),
    num_dapm_routes: BT_SCO_ROUTES.len() as _,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn bt_sco_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    devm_snd_soc_register_component(
        unsafe { &raw mut (*pdev).dev },
        &SOC_COMPONENT_DEV_BT_SCO,
        unsafe { BT_SCO_DAI.as_mut_ptr() },
        BT_SCO_DAI.len() as _,
    )
}

static BT_SCO_DRIVER_IDS: &[platform_device_id] = &[
    platform_device_id {
        name: *b"dfbmcs320\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        name: *b"bt-sco\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ..unsafe { core::mem::zeroed() }
    },
    platform_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE!(platform, BT_SCO_DRIVER_IDS);

// C conditional preserved: #if defined(CONFIG_OF)
#[cfg(CONFIG_OF)]
static BT_SCO_CODEC_OF_MATCH: &[of_device_id] = &[
    of_device_id {
        compatible: *b"delta,dfbmcs320\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: *b"linux,bt-sco\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, BT_SCO_CODEC_OF_MATCH);

static mut BT_SCO_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"bt-sco".as_ptr(),
        of_match_table: of_match_ptr!(BT_SCO_CODEC_OF_MATCH),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(bt_sco_probe),
    id_table: BT_SCO_DRIVER_IDS.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(BT_SCO_DRIVER);

MODULE_AUTHOR!("Lars-Peter Clausen <lars@metafoo.de>");
MODULE_DESCRIPTION!("ASoC generic bluetooth sco link driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
