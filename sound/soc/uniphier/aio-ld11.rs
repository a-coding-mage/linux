// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO ALSA driver for LD11/LD20.
//
// Copyright (c) 2016-2018 Socionext Inc.

// C dependencies: <linux/module.h>, "aio.h"

static UNIPHIER_AIO_LD11: [uniphier_aio_spec; 12] = [
    /* for HDMI PCM In, Pin:AI1Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMIN1,
        gname: AUD_GNAME_HDMI,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_INPUT,
            rb: [21, 14],
            ch: [21, 14],
            iif: [5, 3],
            iport: [0, AUD_HW_PCMIN1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for SIF In, Pin:AI2Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMIN2,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_INPUT,
            rb: [22, 15],
            ch: [22, 15],
            iif: [6, 4],
            iport: [1, AUD_HW_PCMIN2],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Line In, Pin:AI3Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMIN3,
        gname: AUD_GNAME_LINE,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_EVE,
            dir: PORT_DIR_INPUT,
            rb: [23, 16],
            ch: [23, 16],
            iif: [7, 5],
            iport: [2, AUD_HW_PCMIN3],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for S/PDIF In, Pin:AI1IEC */
    uniphier_aio_spec {
        name: AUD_NAME_IECIN1,
        gname: AUD_GNAME_IEC,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_INPUT,
            rb: [26, 17],
            ch: [26, 17],
            iif: [10, 6],
            iport: [3, AUD_HW_IECIN1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Speaker, Pin:AO1Dx */
    uniphier_aio_spec {
        name: AUD_NAME_HPCMOUT1,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_OUTPUT,
            rb: [0, 0],
            ch: [0, 0],
            oif: [0, 0],
            oport: [0, AUD_HW_HPCMOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HDMI PCM, Pin:AO2Dx */
    uniphier_aio_spec {
        name: AUD_NAME_PCMOUT1,
        gname: AUD_GNAME_HDMI,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_I2S,
            dir: PORT_DIR_OUTPUT,
            rb: [0, 0],
            ch: [0, 0],
            oif: [0, 0],
            oport: [3, AUD_HW_PCMOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Line Out, Pin:LO2_x */
    uniphier_aio_spec {
        name: AUD_NAME_PCMOUT2,
        gname: AUD_GNAME_LINE,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_EVE,
            dir: PORT_DIR_OUTPUT,
            rb: [2, 2],
            ch: [2, 2],
            oif: [2, 2],
            oport: [1, AUD_HW_PCMOUT2],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for Headphone, Pin:HP1_x */
    uniphier_aio_spec {
        name: AUD_NAME_PCMOUT3,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_EVE,
            dir: PORT_DIR_OUTPUT,
            rb: [3, 3],
            ch: [3, 3],
            oif: [3, 3],
            oport: [2, AUD_HW_PCMOUT3],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HW Sampling Rate Converter */
    uniphier_aio_spec {
        name: AUD_NAME_EPCMOUT2,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_CONV,
            dir: PORT_DIR_OUTPUT,
            rb: [7, 5],
            ch: [7, 5],
            oif: [7, 5],
            oport: [6, AUD_HW_EPCMOUT2],
            och: [17, 12],
            iif: [1, 1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for HW Sampling Rate Converter 2 */
    uniphier_aio_spec {
        name: AUD_NAME_EPCMOUT3,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_CONV,
            dir: PORT_DIR_OUTPUT,
            rb: [8, 6],
            ch: [8, 6],
            oif: [8, 6],
            oport: [7, AUD_HW_EPCMOUT3],
            och: [18, 13],
            iif: [2, 2],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for S/PDIF Out, Pin:AO1IEC */
    uniphier_aio_spec {
        name: AUD_NAME_HIECOUT1,
        gname: AUD_GNAME_IEC,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [1, 1],
            ch: [1, 1],
            oif: [1, 1],
            oport: [12, AUD_HW_HIECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },

    /* for S/PDIF Out, Pin:AO1IEC, Compress */
    uniphier_aio_spec {
        name: AUD_NAME_HIECCOMPOUT1,
        gname: AUD_GNAME_IEC,
        swm: uniphier_aio_swm {
            type_: PORT_TYPE_SPDIF,
            dir: PORT_DIR_OUTPUT,
            rb: [1, 1],
            ch: [1, 1],
            oif: [1, 1],
            oport: [12, AUD_HW_HIECOUT1],
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

static UNIPHIER_AIO_PLL_LD11: [uniphier_aio_pll; AUD_PLL_HSC0 + 1] = {
    let mut plls: [uniphier_aio_pll; AUD_PLL_HSC0 + 1] =
        unsafe { core::mem::zeroed() };

    plls[AUD_PLL_A1] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_F1] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_A2] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_F2] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_APLL] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_RX0] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_USB0] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };
    plls[AUD_PLL_HSC0] = uniphier_aio_pll {
        enable: true,
        ..unsafe { core::mem::zeroed() }
    };

    plls
};

static mut UNIPHIER_AIO_DAI_LD11: [snd_soc_dai_driver; 9] = [
    snd_soc_dai_driver {
        name: AUD_GNAME_HDMI,
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
            rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_PCMIN2,
        capture: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMIN2,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_GNAME_LINE,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMOUT2,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMIN3,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_HPCMOUT1,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_HPCMOUT1,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 8,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_PCMOUT3,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_PCMOUT3,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
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
        ops: unsafe { &uniphier_aio_spdif_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_EPCMOUT2,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_EPCMOUT2,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: AUD_NAME_EPCMOUT3,
        playback: snd_soc_pcm_stream {
            stream_name: AUD_NAME_EPCMOUT3,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            rates: SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_32000,
            channels_min: 2,
            channels_max: 2,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &uniphier_aio_i2s_ld11_ops },
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
        ops: unsafe { &uniphier_aio_spdif_ld11_ops2 },
        ..unsafe { core::mem::zeroed() }
    },
];

static UNIPHIER_AIO_LD11_SPEC: uniphier_aio_chip_spec = uniphier_aio_chip_spec {
    specs: UNIPHIER_AIO_LD11.as_ptr(),
    num_specs: UNIPHIER_AIO_LD11.len(),
    dais: unsafe { UNIPHIER_AIO_DAI_LD11.as_ptr() },
    num_dais: 9,
    plls: UNIPHIER_AIO_PLL_LD11.as_ptr(),
    num_plls: UNIPHIER_AIO_PLL_LD11.len(),
    addr_ext: 0,
    ..unsafe { core::mem::zeroed() }
};

static UNIPHIER_AIO_LD20_SPEC: uniphier_aio_chip_spec = uniphier_aio_chip_spec {
    specs: UNIPHIER_AIO_LD11.as_ptr(),
    num_specs: UNIPHIER_AIO_LD11.len(),
    dais: unsafe { UNIPHIER_AIO_DAI_LD11.as_ptr() },
    num_dais: 9,
    plls: UNIPHIER_AIO_PLL_LD11.as_ptr(),
    num_plls: UNIPHIER_AIO_PLL_LD11.len(),
    addr_ext: 1,
    ..unsafe { core::mem::zeroed() }
};

#[used]
static UNIPHIER_AIO_OF_MATCH: [of_device_id; 3] = [
    of_device_id {
        compatible: b"socionext,uniphier-ld11-aio\0".as_ptr() as *const _,
        data: &UNIPHIER_AIO_LD11_SPEC as *const _ as *const _,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: b"socionext,uniphier-ld20-aio\0".as_ptr() as *const _,
        data: &UNIPHIER_AIO_LD20_SPEC as *const _ as *const _,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
// MODULE_DEVICE_TABLE(of, uniphier_aio_of_match);

static mut UNIPHIER_AIO_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"snd-uniphier-aio-ld11\0".as_ptr() as *const _,
        of_match_table: UNIPHIER_AIO_OF_MATCH.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(uniphier_aio_probe),
    remove: Some(uniphier_aio_remove),
    ..unsafe { core::mem::zeroed() }
};
// module_platform_driver(uniphier_aio_driver);

// MODULE_AUTHOR("Katsuhiro Suzuki <suzuki.katsuhiro@socionext.com>");
// MODULE_DESCRIPTION("UniPhier LD11/LD20 AIO driver.");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
