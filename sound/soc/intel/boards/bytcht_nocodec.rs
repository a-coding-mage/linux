// SPDX-License-Identifier: GPL-2.0-only
/*
 *  bytcht_nocodec.c - ASoc Machine driver for MinnowBoard Max and Up
 *  to make I2S signals observable on the Low-Speed connector. Audio codec
 *  is not managed by ASoC/DAPM
 *
 *  Copyright (C) 2015-2017 Intel Corp
 *
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ptr;

// Dependencies from linux/module.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, and ../atom/sst-atom-controls.h are expected to be supplied by
// the surrounding kernel Rust bindings.

static WIDGETS: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_MIC!("Mic", ptr::null()),
    SND_SOC_DAPM_SPK!("Speaker", ptr::null()),
];

static CONTROLS: [snd_kcontrol_new; 2] = [
    SOC_DAPM_PIN_SWITCH!("Mic"),
    SOC_DAPM_PIN_SWITCH!("Speaker"),
];

static AUDIO_MAP: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Tx"),
        control: ptr::null(),
        source: c_str!("codec_out0"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Tx"),
        control: ptr::null(),
        source: c_str!("codec_out1"),
    },
    snd_soc_dapm_route {
        sink: c_str!("codec_in0"),
        control: ptr::null(),
        source: c_str!("ssp2 Rx"),
    },
    snd_soc_dapm_route {
        sink: c_str!("codec_in1"),
        control: ptr::null(),
        source: c_str!("ssp2 Rx"),
    },
    snd_soc_dapm_route {
        sink: c_str!("ssp2 Rx"),
        control: ptr::null(),
        source: c_str!("Mic"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Speaker"),
        control: ptr::null(),
        source: c_str!("ssp2 Tx"),
    },
];

unsafe extern "C" fn codec_fixup(
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
        dev_err(
            (*rtd).dev,
            c_str!("can't set format to I2S, err %d\n"),
            ret,
        );
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, 24);
    if ret < 0 {
        dev_err(
            (*rtd).dev,
            c_str!("can't set I2S config, err %d\n"),
            ret,
        );
        return ret;
    }

    0
}

static RATES_48000: [c_uint; 1] = [48000];

static CONSTRAINTS_48000: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: RATES_48000.len() as c_uint,
    list: RATES_48000.as_ptr(),
};

unsafe extern "C" fn aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &CONSTRAINTS_48000,
    )
}

static AIF1_OPS: snd_soc_ops = snd_soc_ops {
    startup: Some(aif1_startup),
};

SND_SOC_DAILINK_DEF!(
    dummy,
    DAILINK_COMP_ARRAY!(COMP_DUMMY!())
);

SND_SOC_DAILINK_DEF!(
    media,
    DAILINK_COMP_ARRAY!(COMP_CPU!("media-cpu-dai"))
);

SND_SOC_DAILINK_DEF!(
    deepbuffer,
    DAILINK_COMP_ARRAY!(COMP_CPU!("deepbuffer-cpu-dai"))
);

SND_SOC_DAILINK_DEF!(
    ssp2_port,
    DAILINK_COMP_ARRAY!(COMP_CPU!("ssp2-port"))
);

SND_SOC_DAILINK_DEF!(
    platform,
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!("sst-mfld-platform"))
);

static mut DAIS: [snd_soc_dai_link; 3] = {
    let mut dais = [snd_soc_dai_link::default(); 3];

    dais[MERR_DPCM_AUDIO as usize] = snd_soc_dai_link {
        name: c_str!("Audio Port"),
        stream_name: c_str!("Audio"),
        ignore_suspend: 1,
        nonatomic: true,
        dynamic: 1,
        ops: &AIF1_OPS,
        SND_SOC_DAILINK_REG!(media, dummy, platform)
        ..snd_soc_dai_link::default()
    };

    dais[MERR_DPCM_DEEP_BUFFER as usize] = snd_soc_dai_link {
        name: c_str!("Deep-Buffer Audio Port"),
        stream_name: c_str!("Deep-Buffer Audio"),
        ignore_suspend: 1,
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &AIF1_OPS,
        SND_SOC_DAILINK_REG!(deepbuffer, dummy, platform)
        ..snd_soc_dai_link::default()
    };

    /* CODEC<->CODEC link */
    /* back ends */
    dais[2] = snd_soc_dai_link {
        name: c_str!("SSP2-LowSpeed Connector"),
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        be_hw_params_fixup: Some(codec_fixup),
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(ssp2_port, dummy, platform)
        ..snd_soc_dai_link::default()
    };

    dais
};

/* SoC card */
static mut BYTCHT_NOCODEC_CARD: snd_soc_card = snd_soc_card {
    name: c_str!("bytcht-nocodec"),
    owner: THIS_MODULE,
    dai_link: unsafe { DAIS.as_mut_ptr() },
    num_links: unsafe { DAIS.len() as c_int },
    dapm_widgets: WIDGETS.as_ptr(),
    num_dapm_widgets: WIDGETS.len() as c_int,
    dapm_routes: AUDIO_MAP.as_ptr(),
    num_dapm_routes: AUDIO_MAP.len() as c_int,
    controls: CONTROLS.as_ptr(),
    num_controls: CONTROLS.len() as c_int,
    fully_routed: true,
    ..snd_soc_card::default()
};

unsafe extern "C" fn snd_bytcht_nocodec_mc_probe(pdev: *mut platform_device) -> c_int {
    let mut ret_val: c_int = 0;

    /* register the soc card */
    BYTCHT_NOCODEC_CARD.dev = &mut (*pdev).dev;

    ret_val = devm_snd_soc_register_card(&mut (*pdev).dev, &mut BYTCHT_NOCODEC_CARD);

    if ret_val != 0 {
        dev_err(
            &mut (*pdev).dev,
            c_str!("devm_snd_soc_register_card failed %d\n"),
            ret_val,
        );
        return ret_val;
    }
    platform_set_drvdata(pdev, &mut BYTCHT_NOCODEC_CARD as *mut _ as *mut c_void);
    ret_val
}

static mut SND_BYTCHT_NOCODEC_MC_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("bytcht_nocodec"),
        ..device_driver::default()
    },
    probe: Some(snd_bytcht_nocodec_mc_probe),
    ..platform_driver::default()
};

module_platform_driver!(SND_BYTCHT_NOCODEC_MC_DRIVER);

MODULE_DESCRIPTION!("ASoC Intel(R) Baytrail/Cherrytrail Nocodec Machine driver");
MODULE_AUTHOR!("Pierre-Louis Bossart <pierre-louis.bossart at linux.intel.com>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("platform:bytcht_nocodec");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
