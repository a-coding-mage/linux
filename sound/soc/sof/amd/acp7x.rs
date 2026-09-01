// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2025 Advanced Micro Devices, Inc.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>

/*
 * Hardware interface for Audio DSP on ACP7.B/7.F platforms
 */

// C dependencies: linux/acpi.h, linux/module.h, linux/platform_device.h,
// linux/pci.h, sound/soc-acpi.h, ../ops.h, ../sof-audio.h, acp.h,
// acp-dsp-offset.h.

use core::ffi::{c_char, c_int};
use core::mem::size_of;
use core::ptr;

const I2S_TDM0_INSTANCE: usize = 0;
const I2S_TDM1_INSTANCE: usize = 1;
const I2S_TDM2_INSTANCE: usize = 2;
const PDM0_DMIC_INSTANCE: usize = 3;
const PDM1_DMIC_INSTANCE: usize = 4;

unsafe extern "C" {
    static sof_acp_common_ops: snd_sof_dsp_ops;

    fn usleep_range(min: u64, max: u64);
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn acpi_dev_get_property(
        adev: *mut acpi_device,
        name: *const c_char,
        r#type: c_int,
        obj: *mut *const acpi_object,
    ) -> c_int;

    fn amd_sof_acp7x_probe(sdev: *mut snd_sof_dev) -> c_int;
    fn amd_sof_acp7x_remove(sdev: *mut snd_sof_dev) -> c_int;
    fn amd_sof_acp7x_suspend(sdev: *mut snd_sof_dev) -> c_int;
    fn amd_sof_acp7x_resume(sdev: *mut snd_sof_dev) -> c_int;
    fn amd_sof_acp7x_suspend_runtime(sdev: *mut snd_sof_dev) -> c_int;
    fn amd_sof_acp7x_resume_runtime(sdev: *mut snd_sof_dev) -> c_int;
    fn acp_sof_load_signed_firmware(sdev: *mut snd_sof_dev) -> c_int;
}

static mut acp7x_sof_dai: [snd_soc_dai_driver; 5] = [
    snd_soc_dai_driver {
        id: I2S_TDM0_INSTANCE as c_int,
        name: b"acp-sof-i2s0\0".as_ptr() as *const c_char,
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
        },
    },
    snd_soc_dai_driver {
        id: I2S_TDM1_INSTANCE as c_int,
        name: b"acp-sof-i2s1\0".as_ptr() as *const c_char,
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
        },
    },
    snd_soc_dai_driver {
        id: I2S_TDM2_INSTANCE as c_int,
        name: b"acp-sof-i2s2\0".as_ptr() as *const c_char,
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
        },
    },
    snd_soc_dai_driver {
        id: PDM0_DMIC_INSTANCE as c_int,
        name: b"acp-sof-dmic0\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream::default(),
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 48000,
        },
    },
    snd_soc_dai_driver {
        id: PDM1_DMIC_INSTANCE as c_int,
        name: b"acp-sof-dmic1\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream::default(),
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 4,
            rate_min: 8000,
            rate_max: 96000,
        },
    },
];

unsafe extern "C" fn sof_acp7x_post_fw_run_delay(sdev: *mut snd_sof_dev) -> c_int {
    /*
     * Resuming from suspend in some cases may cause the DSP firmware
     * to enter an unrecoverable faulty state. Delaying a bit any host
     * to DSP transmission right after firmware boot completion seems
     * to resolve the issue.
     */
    if !(*sdev).first_boot {
        usleep_range(100, 150);
    }

    0
}

#[unsafe(no_mangle)]
pub static mut sof_acp7x_ops: snd_sof_dsp_ops = snd_sof_dsp_ops::default();
// EXPORT_SYMBOL_NS(sof_acp7x_ops, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_acp7x_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    let adev: *mut acpi_device = ACPI_COMPANION(&mut (*to_pci_dev((*sdev).dev)).dev);
    let mut obj: *const acpi_object = ptr::null();
    let mut acp_sof_signed_firmware_image: c_int = 0;
    let mut acp_sof_post_fw_run_delay: c_int = 0;

    /* common defaults */
    ptr::copy_nonoverlapping(
        &sof_acp_common_ops as *const snd_sof_dsp_ops,
        &mut sof_acp7x_ops as *mut snd_sof_dsp_ops,
        1,
    );
    let _ = size_of::<snd_sof_dsp_ops>();

    sof_acp7x_ops.drv = acp7x_sof_dai.as_mut_ptr();
    sof_acp7x_ops.num_drv = acp7x_sof_dai.len();
    sof_acp7x_ops.probe = Some(amd_sof_acp7x_probe);
    sof_acp7x_ops.remove = Some(amd_sof_acp7x_remove);

    if !adev.is_null() {
        if acpi_dev_get_property(
            adev,
            b"acp-sof-signed-firmware-image\0".as_ptr() as *const c_char,
            ACPI_TYPE_INTEGER,
            &mut obj,
        ) == 0
        {
            acp_sof_signed_firmware_image = (*obj).integer.value as c_int;
        }

        if acpi_dev_get_property(
            adev,
            b"acp-sof-post_fw_run_delay\0".as_ptr() as *const c_char,
            ACPI_TYPE_INTEGER,
            &mut obj,
        ) == 0
        {
            acp_sof_post_fw_run_delay = (*obj).integer.value as c_int;
        }
    }

    if acp_sof_signed_firmware_image != 0 {
        sof_acp7x_ops.load_firmware = Some(acp_sof_load_signed_firmware);
    }

    if acp_sof_post_fw_run_delay != 0 {
        sof_acp7x_ops.post_fw_run = Some(sof_acp7x_post_fw_run_delay);
    }

    sof_acp7x_ops.suspend = Some(amd_sof_acp7x_suspend);
    sof_acp7x_ops.resume = Some(amd_sof_acp7x_resume);
    sof_acp7x_ops.runtime_suspend = Some(amd_sof_acp7x_suspend_runtime);
    sof_acp7x_ops.runtime_resume = Some(amd_sof_acp7x_resume_runtime);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
