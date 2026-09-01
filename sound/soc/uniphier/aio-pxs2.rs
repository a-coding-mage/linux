// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO ALSA driver for PXs2.
//
// Copyright (c) 2018 Socionext Inc.

// C dependencies: <linux/module.h>, "aio.h"

static uniphier_aio_pxs2: [uniphier_aio_spec; 9] = [
    /* for Line PCM In, Pin:AI1Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMIN1,
        gname: AUD_GNAME_LINE,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_INPUT,
            rb: [16, 11],
            ch: [16, 11],
            iif: [0, 0],
            iport: [0, AUD_HW_PCMIN1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Speaker/Headphone/Mic PCM In, Pin:AI2Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMIN2,
        gname: AUD_GNAME_AUX,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_INPUT,
            rb: [17, 12],
            ch: [17, 12],
            iif: [1, 1],
            iport: [1, AUD_HW_PCMIN2],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HDMI PCM Out, Pin:AO1Dx (inner) */
    uniphier_aio_spec {
        name: AUD_NAME_HPCMOUT1,
        gname: AUD_GNAME_HDMI,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_OUTPUT,
            rb: [0, 0],
            ch: [0, 0],
            oif: [0, 0],
            oport: [3, AUD_HW_HPCMOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Line PCM Out, Pin:AO2Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMOUT1,
        gname: AUD_GNAME_LINE,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_OUTPUT,
            rb: [1, 1],
            ch: [1, 1],
            oif: [1, 1],
            oport: [0, AUD_HW_PCMOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Speaker/Headphone/Mic PCM Out, Pin:AO3Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMOUT2,
        gname: AUD_GNAME_AUX,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_OUTPUT,
            rb: [2, 2],
            ch: [2, 2],
            oif: [2, 2],
            oport: [1, AUD_HW_PCMOUT2],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HDMI Out, Pin:AO1IEC */
    uniphier_aio_spec {
        name: AUD_NAME_HIECOUT1,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [6, 4],
            ch: [6, 4],
            oif: [6, 4],
            oport: [12, AUD_HW_HIECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HDMI Out, Pin:AO1IEC, Compress */
    uniphier_aio_spec {
        name: AUD_NAME_HIECCOMPOUT1,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [6, 4],
            ch: [6, 4],
            oif: [6, 4],
            oport: [12, AUD_HW_HIECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for S/PDIF Out, Pin:AO2IEC */
    uniphier_aio_spec {
        name: AUD_NAME_IECOUT1,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [7, 5],
            ch: [7, 5],
            oif: [7, 5],
            oport: [13, AUD_HW_IECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for S/PDIF Out, Pin:AO2IEC */
    uniphier_aio_spec {
        name: AUD_NAME_IECCOMPOUT1,
        swm: uniphier_aio_spec_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [7, 5],
            ch: [7, 5],
            oif: [7, 5],
            oport: [13, AUD_HW_IECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

static uniphier_aio_pll_pxs2: [uniphier_aio_pll; 6] = [
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_A1] */
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_F1] */
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_A2] */
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_F2] */
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_APLL] */
    uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    }, /* [AUD_PLL_HSC0] */
];

static mut uniphier_aio_dai_pxs2: [snd_soc_dai_driver; 7] = [
    snd_soc_dai_driver {
        name: AUD_GNAME_HDMI,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_HPCMOUT1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_pxs2_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_GNAME_LINE,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMOUT1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMIN1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_pxs2_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_GNAME_AUX,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMOUT2,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMIN2,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_pxs2_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_HIECOUT1,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_HIECOUT1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_spdif_pxs2_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_IECOUT1,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_IECOUT1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_spdif_pxs2_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_HIECCOMPOUT1,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_HIECCOMPOUT1,
            channels_min: 1,
            channels_max: 1,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_spdif_pxs2_ops2 },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_IECCOMPOUT1,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_IECCOMPOUT1,
            channels_min: 1,
            channels_max: 1,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_spdif_pxs2_ops2 },
        ..unsafe { core::mem::zeroed() }
    },
];

static uniphier_aio_pxs2_spec: uniphier_aio_chip_spec = uniphier_aio_chip_spec {
    specs: uniphier_aio_pxs2.as_ptr(),
    num_specs: uniphier_aio_pxs2.len(),
    dais: unsafe { uniphier_aio_dai_pxs2.as_ptr() as *mut snd_soc_dai_driver },
    num_dais: 7,
    plls: uniphier_aio_pll_pxs2.as_ptr(),
    num_plls: uniphier_aio_pll_pxs2.len(),
    addr_ext: 0,
    ..unsafe { core::mem::zeroed() }
};

static uniphier_aio_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"socionext,uniphier-pxs2-aio\0".as_ptr() as *const core::ffi::c_char,
        data: &uniphier_aio_pxs2_spec as *const uniphier_aio_chip_spec as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, uniphier_aio_of_match);

static mut uniphier_aio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"snd-uniphier-aio-pxs2\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: uniphier_aio_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(uniphier_aio_probe),
    remove: Some(uniphier_aio_remove),
    ..unsafe { core::mem::zeroed() }
};
// module_platform_driver(uniphier_aio_driver);

// MODULE_AUTHOR("Katsuhiro Suzuki <suzuki.katsuhiro@socionext.com>");
// MODULE_DESCRIPTION("UniPhier PXs2 AIO driver.");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
