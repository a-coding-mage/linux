// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*
 * Hardware interface for Audio DSP on ACP6.3 version based platform
 */

// C dependencies:
// #include <linux/platform_device.h>
// #include <linux/module.h>
// #include "../ops.h"
// #include "../sof-audio.h"
// #include "acp.h"
// #include "acp-dsp-offset.h"

pub const I2S_HS_INSTANCE: usize = 0;
pub const I2S_BT_INSTANCE: usize = 1;
pub const I2S_SP_INSTANCE: usize = 2;
pub const PDM_DMIC_INSTANCE: usize = 3;
pub const I2S_HS_VIRTUAL_INSTANCE: usize = 4;

unsafe extern "C" {
    fn memcpy(dest: *mut core::ffi::c_void,
              src: *const core::ffi::c_void,
              n: usize) -> *mut core::ffi::c_void;

    static sof_acp_common_ops: snd_sof_dsp_ops;
}

static mut acp63_sof_dai: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver {
        id: I2S_HS_INSTANCE as i32,
        name: b"acp-sof-hs\0".as_ptr() as *const core::ffi::c_char,
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
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
        id: I2S_BT_INSTANCE as i32,
        name: b"acp-sof-bt\0".as_ptr() as *const core::ffi::c_char,
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
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
        id: I2S_SP_INSTANCE as i32,
        name: b"acp-sof-sp\0".as_ptr() as *const core::ffi::c_char,
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 8,
            rate_min: 8000,
            rate_max: 96000,
            ..unsafe { core::mem::zeroed() }
        },
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
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
        id: PDM_DMIC_INSTANCE as i32,
        name: b"acp-sof-dmic\0".as_ptr() as *const core::ffi::c_char,
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
        id: I2S_HS_VIRTUAL_INSTANCE as i32,
        name: b"acp-sof-hs-virtual\0".as_ptr() as *const core::ffi::c_char,
        playback: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 |
                SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
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
pub static mut sof_acp63_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };
// EXPORT_SYMBOL_NS(sof_acp63_ops, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_acp63_ops_init(sdev: *mut snd_sof_dev) -> i32 {
    /* common defaults */
    memcpy(&raw mut sof_acp63_ops as *mut core::ffi::c_void,
           &raw const sof_acp_common_ops as *const core::ffi::c_void,
           core::mem::size_of::<snd_sof_dsp_ops>());

    sof_acp63_ops.drv = (&raw mut acp63_sof_dai) as *mut snd_soc_dai_driver;
    sof_acp63_ops.num_drv = acp63_sof_dai.len() as _;

    let _ = sdev;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
