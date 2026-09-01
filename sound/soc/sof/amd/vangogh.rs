// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc.
//
// Authors: Venkata Prasad Potturu <venkataprasad.potturu@amd.com>

/*
 * Hardware interface for Audio DSP on Vangogh platform
 */

// C dependencies:
// #include <linux/delay.h>
// #include <linux/module.h>
// #include "acp.h"

use core::ffi::{c_char, c_int, c_void};

const I2S_HS_INSTANCE: usize = 0;
const I2S_BT_INSTANCE: usize = 1;
const I2S_SP_INSTANCE: usize = 2;
const PDM_DMIC_INSTANCE: usize = 3;
const I2S_HS_VIRTUAL_INSTANCE: usize = 4;

extern "C" {
    static sof_acp_common_ops: snd_sof_dsp_ops;
    static acp_sof_quirk_table: dmi_system_id;

    fn usleep_range(min: u64, max: u64);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn dmi_first_match(list: *const dmi_system_id) -> *const dmi_system_id;
    fn acp_sof_load_signed_firmware(sdev: *mut snd_sof_dev) -> c_int;
}

static mut vangogh_sof_dai: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver {
        id: I2S_HS_INSTANCE as c_int,
        name: c"acp-sof-hs".as_ptr() as *const c_char,
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
        id: I2S_BT_INSTANCE as c_int,
        name: c"acp-sof-bt".as_ptr() as *const c_char,
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
        id: I2S_SP_INSTANCE as c_int,
        name: c"acp-sof-sp".as_ptr() as *const c_char,
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
        id: PDM_DMIC_INSTANCE as c_int,
        name: c"acp-sof-dmic".as_ptr() as *const c_char,
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
        id: I2S_HS_VIRTUAL_INSTANCE as c_int,
        name: c"acp-sof-hs-virtual".as_ptr() as *const c_char,
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
            /* Supporting only stereo for I2S HS-Virtual controller capture */
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn sof_vangogh_post_fw_run_delay(sdev: *mut snd_sof_dev) -> c_int {
    /*
     * Resuming from suspend in some cases my cause the DSP firmware
     * to enter an unrecoverable faulty state.  Delaying a bit any host
     * to DSP transmission right after firmware boot completion seems
     * to resolve the issue.
     */
    if !(*sdev).first_boot {
        usleep_range(100, 150);
    }

    0
}

/* Vangogh ops */
#[no_mangle]
pub static mut sof_vangogh_ops: snd_sof_dsp_ops = unsafe { core::mem::zeroed() };
// EXPORT_SYMBOL_NS(sof_vangogh_ops, "SND_SOC_SOF_AMD_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_vangogh_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    let mut dmi_id: *const dmi_system_id;
    let mut quirks: *mut acp_quirk_entry;

    /* common defaults */
    memcpy(
        &mut sof_vangogh_ops as *mut snd_sof_dsp_ops as *mut c_void,
        &sof_acp_common_ops as *const snd_sof_dsp_ops as *const c_void,
        core::mem::size_of::<snd_sof_dsp_ops>(),
    );

    sof_vangogh_ops.drv = vangogh_sof_dai.as_mut_ptr();
    sof_vangogh_ops.num_drv = vangogh_sof_dai.len() as c_int;

    dmi_id = dmi_first_match(&acp_sof_quirk_table as *const dmi_system_id);
    if !dmi_id.is_null() {
        quirks = (*dmi_id).driver_data as *mut acp_quirk_entry;

        if (*quirks).signed_fw_image {
            sof_vangogh_ops.load_firmware = Some(acp_sof_load_signed_firmware);
        }

        if (*quirks).post_fw_run_delay {
            sof_vangogh_ops.post_fw_run = Some(sof_vangogh_post_fw_run_delay);
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
