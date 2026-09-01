// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI Hostless Control
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// Depends on declarations from "mt8183-afe-common.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut c_void,
    pub mtk_afe_hardware: *const c_void,
    pub sub_dais: list_head,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hwparams: *const c_void,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut c_void, size: usize, flags: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
}

extern "C" {
    static MT8183_DAI_HOSTLESS_LPBK: c_int;
    static MT8183_DAI_HOSTLESS_SPEECH: c_int;
    static SNDRV_PCM_RATE_8000_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;
}

const fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/* dai component */
static mtk_dai_hostless_routes: [snd_soc_dapm_route; 19] = [
    /* Hostless ADDA Loopback */
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH1\0"),
        control: c_str(b"ADDA_UL_CH1\0"),
        source: c_str(b"Hostless LPBK DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH1\0"),
        control: c_str(b"ADDA_UL_CH2\0"),
        source: c_str(b"Hostless LPBK DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"ADDA_UL_CH1\0"),
        source: c_str(b"Hostless LPBK DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"ADDA_UL_CH2\0"),
        source: c_str(b"Hostless LPBK DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"Hostless LPBK UL\0"),
        control: ptr::null(),
        source: c_str(b"ADDA Capture\0"),
    },

    /* Hostless Speech */
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH1\0"),
        control: c_str(b"PCM_1_CAP_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"PCM_1_CAP_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"PCM_1_CAP_CH2\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH1\0"),
        control: c_str(b"PCM_2_CAP_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"PCM_2_CAP_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"ADDA_DL_CH2\0"),
        control: c_str(b"PCM_2_CAP_CH2\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"PCM_1_PB_CH1\0"),
        control: c_str(b"ADDA_UL_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"PCM_1_PB_CH2\0"),
        control: c_str(b"ADDA_UL_CH2\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"PCM_2_PB_CH1\0"),
        control: c_str(b"ADDA_UL_CH1\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"PCM_2_PB_CH2\0"),
        control: c_str(b"ADDA_UL_CH2\0"),
        source: c_str(b"Hostless Speech DL\0"),
    },

    snd_soc_dapm_route {
        sink: c_str(b"Hostless Speech UL\0"),
        control: ptr::null(),
        source: c_str(b"PCM 1 Capture\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"Hostless Speech UL\0"),
        control: ptr::null(),
        source: c_str(b"PCM 2 Capture\0"),
    },
    snd_soc_dapm_route {
        sink: c_str(b"Hostless Speech UL\0"),
        control: ptr::null(),
        source: c_str(b"ADDA Capture\0"),
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
};

/* dai driver */
unsafe fn MTK_HOSTLESS_RATES() -> c_uint {
    SNDRV_PCM_RATE_8000_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000
}

unsafe fn MTK_HOSTLESS_FORMATS() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
}

static mut mtk_dai_hostless_driver: [snd_soc_dai_driver; 2] = unsafe {
    [
        snd_soc_dai_driver {
            name: c_str(b"Hostless LPBK DAI\0"),
            id: MT8183_DAI_HOSTLESS_LPBK,
            playback: snd_soc_pcm_stream {
                stream_name: c_str(b"Hostless LPBK DL\0"),
                channels_min: 1,
                channels_max: 2,
                rates: MTK_HOSTLESS_RATES(),
                formats: MTK_HOSTLESS_FORMATS(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: c_str(b"Hostless LPBK UL\0"),
                channels_min: 1,
                channels_max: 2,
                rates: MTK_HOSTLESS_RATES(),
                formats: MTK_HOSTLESS_FORMATS(),
            },
            ops: &mtk_dai_hostless_ops,
        },
        snd_soc_dai_driver {
            name: c_str(b"Hostless Speech DAI\0"),
            id: MT8183_DAI_HOSTLESS_SPEECH,
            playback: snd_soc_pcm_stream {
                stream_name: c_str(b"Hostless Speech DL\0"),
                channels_min: 1,
                channels_max: 2,
                rates: MTK_HOSTLESS_RATES(),
                formats: MTK_HOSTLESS_FORMATS(),
            },
            capture: snd_soc_pcm_stream {
                stream_name: c_str(b"Hostless Speech UL\0"),
                channels_min: 1,
                channels_max: 2,
                rates: MTK_HOSTLESS_RATES(),
                formats: MTK_HOSTLESS_FORMATS(),
            },
            ops: &mtk_dai_hostless_ops,
        },
    ]
};

#[no_mangle]
pub unsafe extern "C" fn mt8183_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mtk_dai_hostless_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mtk_dai_hostless_driver.len() as c_uint;

    (*dai).dapm_routes = mtk_dai_hostless_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_hostless_routes.len() as c_uint;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
