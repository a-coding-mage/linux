// SPDX-License-Identifier: GPL-2.0
//
// MediaTek ALSA SoC Audio DAI Hostless Control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// Original C dependency: #include "mt8186-afe-common.h"

use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null};

static mt8186_hostless_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    period_bytes_min: 256,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 4 * 48 * 1024,
    fifo_size: 0,
};

/* dai component */
static mtk_dai_hostless_routes: [snd_soc_dapm_route; 36] = [
    /* Hostless ADDA Loopback */
    snd_soc_dapm_route { sink: b"ADDA_DL_CH1\0".as_ptr().cast(), control: b"ADDA_UL_CH1 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"ADDA_DL_CH1\0".as_ptr().cast(), control: b"ADDA_UL_CH2 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"ADDA_DL_CH2\0".as_ptr().cast(), control: b"ADDA_UL_CH1 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"ADDA_DL_CH2\0".as_ptr().cast(), control: b"ADDA_UL_CH2 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S1_CH1\0".as_ptr().cast(), control: b"ADDA_UL_CH1 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S1_CH2\0".as_ptr().cast(), control: b"ADDA_UL_CH2 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH1\0".as_ptr().cast(), control: b"ADDA_UL_CH1 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH1\0".as_ptr().cast(), control: b"ADDA_UL_CH2 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH2\0".as_ptr().cast(), control: b"ADDA_UL_CH1 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH2\0".as_ptr().cast(), control: b"ADDA_UL_CH2 Switch\0".as_ptr().cast(), source: b"Hostless LPBK DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Hostless LPBK UL\0".as_ptr().cast(), control: null(), source: b"ADDA_UL_Mux\0".as_ptr().cast() },

    /* Hostelss FM */
    /* connsys_i2s to hw gain 1*/
    snd_soc_dapm_route { sink: b"Hostless FM UL\0".as_ptr().cast(), control: null(), source: b"Connsys I2S\0".as_ptr().cast() },

    snd_soc_dapm_route { sink: b"HW_GAIN1_IN_CH1\0".as_ptr().cast(), control: b"CONNSYS_I2S_CH1 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HW_GAIN1_IN_CH2\0".as_ptr().cast(), control: b"CONNSYS_I2S_CH2 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    /* hw gain to adda dl */
    snd_soc_dapm_route { sink: b"Hostless FM UL\0".as_ptr().cast(), control: null(), source: b"HW Gain 1 Out\0".as_ptr().cast() },

    snd_soc_dapm_route { sink: b"ADDA_DL_CH1\0".as_ptr().cast(), control: b"GAIN1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"ADDA_DL_CH2\0".as_ptr().cast(), control: b"GAIN1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    /* hw gain to i2s3 */
    snd_soc_dapm_route { sink: b"I2S3_CH1\0".as_ptr().cast(), control: b"GAIN1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH2\0".as_ptr().cast(), control: b"GAIN1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    /* hw gain to i2s1 */
    snd_soc_dapm_route { sink: b"I2S1_CH1\0".as_ptr().cast(), control: b"GAIN1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S1_CH2\0".as_ptr().cast(), control: b"GAIN1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless FM DL\0".as_ptr().cast() },

    /* Hostless_SRC */
    snd_soc_dapm_route { sink: b"ADDA_DL_CH1\0".as_ptr().cast(), control: b"SRC_1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"ADDA_DL_CH2\0".as_ptr().cast(), control: b"SRC_1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S1_CH1\0".as_ptr().cast(), control: b"SRC_1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S1_CH2\0".as_ptr().cast(), control: b"SRC_1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH1\0".as_ptr().cast(), control: b"SRC_1_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"I2S3_CH2\0".as_ptr().cast(), control: b"SRC_1_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_1_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Hostless_SRC_1_UL\0".as_ptr().cast(), control: null(), source: b"HW_SRC_1_Out\0".as_ptr().cast() },

    /* Hostless_SRC_bargein */
    snd_soc_dapm_route { sink: b"HW_SRC_1_IN_CH1\0".as_ptr().cast(), control: b"I2S0_CH1 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_Bargein_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HW_SRC_1_IN_CH2\0".as_ptr().cast(), control: b"I2S0_CH2 Switch\0".as_ptr().cast(), source: b"Hostless_SRC_Bargein_DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Hostless_SRC_Bargein_UL\0".as_ptr().cast(), control: null(), source: b"I2S0\0".as_ptr().cast() },

    /* Hostless AAudio */
    snd_soc_dapm_route { sink: b"Hostless HW Gain AAudio In\0".as_ptr().cast(), control: null(), source: b"HW Gain 2 In\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"Hostless SRC AAudio UL\0".as_ptr().cast(), control: null(), source: b"HW Gain 2 Out\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HW_SRC_2_IN_CH1\0".as_ptr().cast(), control: b"HW_GAIN2_OUT_CH1 Switch\0".as_ptr().cast(), source: b"Hostless SRC AAudio DL\0".as_ptr().cast() },
    snd_soc_dapm_route { sink: b"HW_SRC_2_IN_CH2\0".as_ptr().cast(), control: b"HW_GAIN2_OUT_CH2 Switch\0".as_ptr().cast(), source: b"Hostless SRC AAudio DL\0".as_ptr().cast() },
];

/* dai ops */
unsafe extern "C" fn mtk_dai_hostless_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let runtime = (*substream).runtime;
    let mut ret: c_int;

    snd_soc_set_runtime_hwparams(substream, &mt8186_hostless_hardware);

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_err((*afe).dev, b"snd_pcm_hw_constraint_integer failed\n\0".as_ptr().cast());
        return ret;
    }

    0
}

static mtk_dai_hostless_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mtk_dai_hostless_startup),
};

/* dai driver */
const MTK_HOSTLESS_RATES: u32 = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_HOSTLESS_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mtk_dai_hostless_driver: [snd_soc_dai_driver; 11] = [
    snd_soc_dai_driver {
        name: b"Hostless LPBK DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_LPBK,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless LPBK DL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless LPBK UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
    },
    snd_soc_dai_driver {
        name: b"Hostless FM DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_FM,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless FM DL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless FM UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
    },
    snd_soc_dai_driver {
        name: b"Hostless_SRC_1_DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_SRC_1,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless_SRC_1_DL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_SRC_1_UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
    },
    snd_soc_dai_driver {
        name: b"Hostless_SRC_Bargein_DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_SRC_BARGEIN,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless_SRC_Bargein_DL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_SRC_Bargein_UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
    },
    /* BE dai */
    snd_soc_dai_driver {
        name: b"Hostless_UL1 DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_UL1,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_UL1 UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 4,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless_UL2 DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_UL2,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_UL2 UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 4,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless_UL3 DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_UL3,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_UL3 UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless_UL5 DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_UL5,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_UL5 UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 12,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless_UL6 DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_UL6,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless_UL6 UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless HW Gain AAudio DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_HW_GAIN_AAUDIO,
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless HW Gain AAudio In\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"Hostless SRC AAudio DAI\0".as_ptr().cast(),
        id: MT8186_DAI_HOSTLESS_SRC_AAUDIO,
        playback: snd_soc_pcm_stream {
            stream_name: b"Hostless SRC AAudio DL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Hostless SRC AAudio UL\0".as_ptr().cast(),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_HOSTLESS_RATES,
            formats: MTK_HOSTLESS_FORMATS,
        },
        ops: &mtk_dai_hostless_ops,
    },
];

#[no_mangle]
pub unsafe extern "C" fn mt8186_dai_hostless_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(addr_of_mut!((*dai).list), addr_of_mut!((*afe).sub_dais));

    (*dai).dai_drivers = addr_of_mut!(mtk_dai_hostless_driver[0]);
    (*dai).num_dai_drivers = mtk_dai_hostless_driver.len() as c_int;

    (*dai).dapm_routes = mtk_dai_hostless_routes.as_ptr();
    (*dai).num_dapm_routes = mtk_dai_hostless_routes.len() as c_int;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
