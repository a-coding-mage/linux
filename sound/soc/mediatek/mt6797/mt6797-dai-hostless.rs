// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI Hostless Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// C dependency: "mt6797-afe-common.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

/* dai component */
static mtk_dai_hostless_routes: [snd_soc_dapm_route; 19] = [
    /* Hostless ADDA Loopback */
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless LPBK DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless LPBK DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless LPBK DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless LPBK DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Hostless LPBK UL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ADDA Capture\0".as_ptr() as *const c_char,
    },

    /* Hostless Speech */
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
        control: b"PCM_1_CAP_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"PCM_1_CAP_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"PCM_1_CAP_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH1\0".as_ptr() as *const c_char,
        control: b"PCM_2_CAP_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"PCM_2_CAP_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ADDA_DL_CH2\0".as_ptr() as *const c_char,
        control: b"PCM_2_CAP_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"PCM_1_PB_CH1\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"PCM_1_PB_CH2\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"PCM_2_PB_CH1\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH1\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"PCM_2_PB_CH2\0".as_ptr() as *const c_char,
        control: b"ADDA_UL_CH2\0".as_ptr() as *const c_char,
        source: b"Hostless Speech DL\0".as_ptr() as *const c_char,
    },

    snd_soc_dapm_route {
        sink: b"Hostless Speech UL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"PCM 1 Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Hostless Speech UL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"PCM 2 Capture\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Hostless Speech UL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ADDA Capture\0".as_ptr() as *const c_char,
    },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_hostless_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe: *mut mtk_base_afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;

    snd_soc_set_runtime_hwparams(substream, (*afe).mtk_afe_hardware)
}

static mtk_dai_hostless_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mtk_dai_hostless_startup),
    ..unsafe { core::mem::zeroed() }
};

/* dai driver */
const MTK_HOSTLESS_RATES: u32 = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_HOSTLESS_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_hostless_driver: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"Hostless LPBK DAI\0".as_ptr() as *const c_char,
        id: MT6797_DAI_HOSTLESS_LPBK,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless LPBK DL\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless LPBK UL\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless Speech DAI\0".as_ptr() as *const c_char,
        id: MT6797_DAI_HOSTLESS_SPEECH,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless Speech DL\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless Speech UL\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

pub unsafe extern "C" fn mt6797_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_hostless_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_hostless_driver.len();

    (*dai).dapm_routes = mtk_dai_hostless_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_hostless_routes.len();

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
