// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2024 Advanced Micro Devices, Inc.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*
 * Hardware interface for Audio DSP on ACP7.0 version based platform
 */

// C dependencies: linux/platform_device.h, linux/module.h, ../ops.h,
// ../sof-audio.h, acp.h, acp-dsp-offset.h

pub const I2S_HS_INSTANCE: usize = 0;
pub const I2S_BT_INSTANCE: usize = 1;
pub const I2S_SP_INSTANCE: usize = 2;
pub const PDM_DMIC_INSTANCE: usize = 3;
pub const I2S_HS_VIRTUAL_INSTANCE: usize = 4;

static mut acp70_sof_dai: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver {
        id: I2S_HS_INSTANCE as _,
        name: c"acp-sof-hs".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            /* Supporting only stereo for I2S HS controller capture */
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: I2S_BT_INSTANCE as _,
        name: c"acp-sof-bt".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            /* Supporting only stereo for I2S BT controller capture */
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: I2S_SP_INSTANCE as _,
        name: c"acp-sof-sp".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            /* Supporting only stereo for I2S SP controller capture */
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: PDM_DMIC_INSTANCE as _,
        name: c"acp-sof-dmic".as_ptr(),
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        id: I2S_HS_VIRTUAL_INSTANCE as _,
        name: c"acp-sof-hs-virtual".as_ptr(),
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

/* Phoenix ops */
#[no_mangle]
pub static mut sof_acp70_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };

// EXPORT_SYMBOL_NS(sof_acp70_ops, "SND_SOC_SOF_AMD_COMMON");

extern "C" {
    static sof_acp_common_ops: snd_sof_dsp_ops;
}

#[no_mangle]
pub unsafe extern "C" fn sof_acp70_ops_init(sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    let _ = sdev;

    /* common defaults */
    core::ptr::copy_nonoverlapping(
        &sof_acp_common_ops,
        &mut sof_acp70_ops,
        1,
    );

    sof_acp70_ops.drv = acp70_sof_dai.as_mut_ptr();
    sof_acp70_ops.num_drv = acp70_sof_dai.len() as _;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
