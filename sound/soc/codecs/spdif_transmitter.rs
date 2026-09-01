// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC SPDIF DIT driver
 *
 *  This driver is used by controllers which can operate in DIT (SPDI/F) where
 *  no codec is needed.  This file provides stub codec that can be used
 *  in these configurations. TI DaVinci Audio controller uses this driver.
 *
 * Author:      Steve Chen,  <schen@mvista.com>
 * Copyright:   (C) 2009 MontaVista Software, Inc., <source@mvista.com>
 * Copyright:   (C) 2009  Texas Instruments, India
 */

/* Dependencies from the original C includes:
 * linux/module.h, linux/moduleparam.h, linux/slab.h, sound/soc.h,
 * sound/pcm.h, sound/initval.h, linux/of.h.
 */

const DRV_NAME: &[u8] = b"spdif-dit\0";

const STUB_RATES: u32 = SNDRV_PCM_RATE_8000_768000 | SNDRV_PCM_RATE_128000;
const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

static DIT_WIDGETS: [snd_soc_dapm_widget; 1] = [
    SND_SOC_DAPM_OUTPUT(b"spdif-out\0"),
];

static DIT_ROUTES: [snd_soc_dapm_route; 1] = [
    snd_soc_dapm_route {
        sink: b"spdif-out\0".as_ptr() as *const i8,
        control: core::ptr::null(),
        source: b"Playback\0".as_ptr() as *const i8,
    },
];

static SOC_CODEC_SPDIF_DIT: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: DIT_WIDGETS.as_ptr(),
    num_dapm_widgets: DIT_WIDGETS.len(),
    dapm_routes: DIT_ROUTES.as_ptr(),
    num_dapm_routes: DIT_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static mut DIT_STUB_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"dit-hifi\0".as_ptr() as *const i8,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const i8,
        channels_min: 1,
        channels_max: 384,
        rates: STUB_RATES,
        formats: STUB_FORMATS,
    },
};

unsafe extern "C" fn spdif_dit_probe(pdev: *mut platform_device) -> i32 {
    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &SOC_CODEC_SPDIF_DIT,
            &mut DIT_STUB_DAI,
            1,
        )
    }
}

/* Original C condition: #ifdef CONFIG_OF */
#[cfg(CONFIG_OF)]
static SPDIF_DIT_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"linux,spdif-dit\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
module_device_table!(of, SPDIF_DIT_DT_IDS);

#[cfg(CONFIG_OF)]
const SPDIF_DIT_OF_MATCH_TABLE: *const of_device_id = SPDIF_DIT_DT_IDS.as_ptr();
#[cfg(not(CONFIG_OF))]
const SPDIF_DIT_OF_MATCH_TABLE: *const of_device_id = core::ptr::null();

static mut SPDIF_DIT_DRIVER: platform_driver = platform_driver {
    probe: Some(spdif_dit_probe),
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const i8,
        of_match_table: SPDIF_DIT_OF_MATCH_TABLE,
    },
};

module_platform_driver!(SPDIF_DIT_DRIVER);

module_author!("Steve Chen <schen@mvista.com>");
module_description!("SPDIF dummy codec driver");
module_license!("GPL");
module_alias!("platform:spdif-dit");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
