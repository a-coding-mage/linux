// SPDX-License-Identifier: GPL-2.0
//
// mt6797-mt6351.c  --  MT6797 MT6351 ALSA SoC machine driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// C dependencies:
// #include <linux/module.h>
// #include <sound/soc.h>
// #include "mt6797-afe-common.h"

SND_SOC_DAILINK_DEFS!(
    playback_1,
    DAILINK_COMP_ARRAY!(COMP_CPU!("DL1")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    playback_2,
    DAILINK_COMP_ARRAY!(COMP_CPU!("DL2")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    playback_3,
    DAILINK_COMP_ARRAY!(COMP_CPU!("DL3")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    capture_1,
    DAILINK_COMP_ARRAY!(COMP_CPU!("UL1")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    capture_2,
    DAILINK_COMP_ARRAY!(COMP_CPU!("UL2")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    capture_3,
    DAILINK_COMP_ARRAY!(COMP_CPU!("UL3")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    capture_mono_1,
    DAILINK_COMP_ARRAY!(COMP_CPU!("UL_MONO_1")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    hostless_lpbk,
    DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless LPBK DAI")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    hostless_speech,
    DAILINK_COMP_ARRAY!(COMP_CPU!("Hostless Speech DAI")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    primary_codec,
    DAILINK_COMP_ARRAY!(COMP_CPU!("ADDA")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(core::ptr::null(), "mt6351-snd-codec-aif1")),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    pcm1,
    DAILINK_COMP_ARRAY!(COMP_CPU!("PCM 1")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

SND_SOC_DAILINK_DEFS!(
    pcm2,
    DAILINK_COMP_ARRAY!(COMP_CPU!("PCM 2")),
    DAILINK_COMP_ARRAY!(COMP_DUMMY!()),
    DAILINK_COMP_ARRAY!(COMP_EMPTY!())
);

static mut mt6797_mt6351_dai_links: [snd_soc_dai_link; 12] = [
    /* FE */
    snd_soc_dai_link {
        name: c_str!("Playback_1"),
        stream_name: c_str!("Playback_1"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        playback_only: 1,
        SND_SOC_DAILINK_REG!(playback_1)
    },
    snd_soc_dai_link {
        name: c_str!("Playback_2"),
        stream_name: c_str!("Playback_2"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        playback_only: 1,
        SND_SOC_DAILINK_REG!(playback_2)
    },
    snd_soc_dai_link {
        name: c_str!("Playback_3"),
        stream_name: c_str!("Playback_3"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        playback_only: 1,
        SND_SOC_DAILINK_REG!(playback_3)
    },
    snd_soc_dai_link {
        name: c_str!("Capture_1"),
        stream_name: c_str!("Capture_1"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        capture_only: 1,
        SND_SOC_DAILINK_REG!(capture_1)
    },
    snd_soc_dai_link {
        name: c_str!("Capture_2"),
        stream_name: c_str!("Capture_2"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        capture_only: 1,
        SND_SOC_DAILINK_REG!(capture_2)
    },
    snd_soc_dai_link {
        name: c_str!("Capture_3"),
        stream_name: c_str!("Capture_3"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        capture_only: 1,
        SND_SOC_DAILINK_REG!(capture_3)
    },
    snd_soc_dai_link {
        name: c_str!("Capture_Mono_1"),
        stream_name: c_str!("Capture_Mono_1"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        capture_only: 1,
        SND_SOC_DAILINK_REG!(capture_mono_1)
    },
    snd_soc_dai_link {
        name: c_str!("Hostless_LPBK"),
        stream_name: c_str!("Hostless_LPBK"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(hostless_lpbk)
    },
    snd_soc_dai_link {
        name: c_str!("Hostless_Speech"),
        stream_name: c_str!("Hostless_Speech"),
        trigger: [SND_SOC_DPCM_TRIGGER_PRE, SND_SOC_DPCM_TRIGGER_PRE],
        dynamic: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(hostless_speech)
    },
    /* BE */
    snd_soc_dai_link {
        name: c_str!("Primary Codec"),
        no_pcm: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(primary_codec)
    },
    snd_soc_dai_link {
        name: c_str!("PCM 1"),
        no_pcm: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(pcm1)
    },
    snd_soc_dai_link {
        name: c_str!("PCM 2"),
        no_pcm: 1,
        ignore_suspend: 1,
        SND_SOC_DAILINK_REG!(pcm2)
    },
];

static mut mt6797_mt6351_card: snd_soc_card = snd_soc_card {
    name: c_str!("mt6797-mt6351"),
    owner: THIS_MODULE,
    dai_link: unsafe { mt6797_mt6351_dai_links.as_mut_ptr() },
    num_links: ARRAY_SIZE!(mt6797_mt6351_dai_links),
};

unsafe extern "C" fn mt6797_mt6351_dev_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let card: *mut snd_soc_card = &raw mut mt6797_mt6351_card;
    let mut platform_node: *mut device_node;
    let mut codec_node: *mut device_node;
    let mut dai_link: *mut snd_soc_dai_link;
    let mut ret: core::ffi::c_int;
    let mut i: core::ffi::c_int;

    (*card).dev = &raw mut (*pdev).dev;

    platform_node = of_parse_phandle((*pdev).dev.of_node, c_str!("mediatek,platform"), 0);
    if platform_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Property 'platform' missing or invalid\n"),
        );
        return -EINVAL;
    }

    for_each_card_prelinks!(card, i, dai_link, {
        if !(*(*dai_link).platforms).name.is_null() {
            continue;
        }
        (*(*dai_link).platforms).of_node = platform_node;
    });

    codec_node = of_parse_phandle((*pdev).dev.of_node, c_str!("mediatek,audio-codec"), 0);
    if codec_node.is_null() {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("Property 'audio-codec' missing or invalid\n"),
        );
        ret = -EINVAL;
        goto_put_platform_node!(ret, platform_node);
        return ret;
    }

    for_each_card_prelinks!(card, i, dai_link, {
        if !(*(*dai_link).codecs).name.is_null() {
            continue;
        }
        (*(*dai_link).codecs).of_node = codec_node;
    });

    ret = devm_snd_soc_register_card(&raw mut (*pdev).dev, card);
    if ret != 0 {
        dev_err(
            &raw mut (*pdev).dev,
            c_str!("%s snd_soc_register_card fail %d\n"),
            c_str!("mt6797_mt6351_dev_probe"),
            ret,
        );
    }

    of_node_put(codec_node);

    /* put_platform_node: */
    of_node_put(platform_node);
    ret
}

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static mt6797_mt6351_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("mediatek,mt6797-mt6351-sound"),
    },
    of_device_id {},
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, mt6797_mt6351_dt_match);

static mut mt6797_mt6351_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("mt6797-mt6351"),
        #[cfg(CONFIG_OF)]
        of_match_table: mt6797_mt6351_dt_match.as_ptr(),
    },
    probe: Some(mt6797_mt6351_dev_probe),
};

module_platform_driver!(mt6797_mt6351_driver);

/* Module information */
MODULE_DESCRIPTION!("MT6797 MT6351 ALSA SoC machine driver");
MODULE_AUTHOR!("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
MODULE_LICENSE!("GPL v2");
MODULE_ALIAS!("mt6797 mt6351 soc card");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
