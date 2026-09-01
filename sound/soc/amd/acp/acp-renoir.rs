// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//

/*
 * Hardware interface for Renoir ACP block
 */

/* Translated from Linux C includes:
 * <linux/platform_device.h>, <linux/module.h>, <linux/err.h>, <linux/io.h>,
 * <sound/pcm_params.h>, <sound/soc.h>, <sound/soc-dai.h>,
 * <linux/dma-mapping.h>, <linux/pm_runtime.h>, "amd.h", "acp-mach.h".
 */

const DRV_NAME: *const core::ffi::c_char = b"acp_asoc_renoir\0".as_ptr() as *const core::ffi::c_char;

static mut acp_renoir_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: b"acp-i2s-sp\0".as_ptr() as *const core::ffi::c_char,
        id: I2S_SP_INSTANCE,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S SP Playback\0".as_ptr() as *const core::ffi::c_char,
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
            stream_name: b"I2S SP Capture\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops as *const _ },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"acp-i2s-bt\0".as_ptr() as *const core::ffi::c_char,
        id: I2S_BT_INSTANCE,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S BT Playback\0".as_ptr() as *const core::ffi::c_char,
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
            stream_name: b"I2S BT Capture\0".as_ptr() as *const core::ffi::c_char,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE
                | SNDRV_PCM_FMTBIT_S8
                | SNDRV_PCM_FMTBIT_U8
                | SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &asoc_acp_cpu_dai_ops as *const _ },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: b"acp-pdm-dmic\0".as_ptr() as *const core::ffi::c_char,
        id: DMIC_INSTANCE,
        capture: snd_soc_pcm_stream {
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S32_LE,
            channels_min: 2,
            channels_max: 2,
            rate_min: 8000,
            rate_max: 48000,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &acp_dmic_dai_ops as *const _ },
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn renoir_audio_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let chip: *mut acp_chip_info;
    let ret: core::ffi::c_int;

    chip = unsafe { dev_get_platdata(&mut (*pdev).dev) as *mut acp_chip_info };
    if chip.is_null() || unsafe { (*chip).base.is_null() } {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"ACP chip data is NULL\n\0".as_ptr() as *const core::ffi::c_char,
            );
        }
        return -ENODEV;
    }

    if unsafe { (*chip).acp_rev } != ACP_RN_PCI_ID {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"Un-supported ACP Revision %d\n\0".as_ptr() as *const core::ffi::c_char,
                (*chip).acp_rev,
            );
        }
        return -ENODEV;
    }

    unsafe {
        (*chip).dev = dev;
        (*chip).dai_driver = acp_renoir_dai.as_mut_ptr();
        (*chip).num_dai = ARRAY_SIZE!(acp_renoir_dai);
    }

    ret = unsafe { acp_hw_en_interrupts(chip) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                b"ACP en-interrupts failed\n\0".as_ptr() as *const core::ffi::c_char,
            );
        }
        return ret;
    }

    unsafe {
        acp_platform_register(dev);

        pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, ACP_SUSPEND_DELAY_MS);
        pm_runtime_use_autosuspend(&mut (*pdev).dev);
        pm_runtime_mark_last_busy(&mut (*pdev).dev);
        pm_runtime_set_active(&mut (*pdev).dev);
        pm_runtime_enable(&mut (*pdev).dev);
    }
    0
}

unsafe extern "C" fn renoir_audio_remove(pdev: *mut platform_device) {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let chip: *mut acp_chip_info = unsafe { dev_get_platdata(dev) as *mut acp_chip_info };
    let ret: core::ffi::c_int;

    ret = unsafe { acp_hw_dis_interrupts(chip) };
    if ret != 0 {
        unsafe {
            dev_err(
                dev,
                b"ACP dis-interrupts failed\n\0".as_ptr() as *const core::ffi::c_char,
            );
        }
    }

    unsafe {
        acp_platform_unregister(dev);
    }
}

unsafe extern "C" fn rn_pcm_resume(dev: *mut device) -> core::ffi::c_int {
    let chip: *mut acp_chip_info = unsafe { dev_get_drvdata((*dev).parent) as *mut acp_chip_info };
    let mut stream: *mut acp_stream;
    let mut substream: *mut snd_pcm_substream;
    let mut buf_in_frames: snd_pcm_uframes_t;
    let mut buf_size: u64;

    unsafe {
        spin_lock(&mut (*chip).acp_lock);
        list_for_each_entry!(stream, &mut (*chip).stream_list, list, {
            substream = (*stream).substream;
            if !substream.is_null() && !(*substream).runtime.is_null() {
                buf_in_frames = (*(*substream).runtime).buffer_size;
                buf_size = frames_to_bytes((*substream).runtime, buf_in_frames);
                config_pte_for_stream(chip, stream);
                config_acp_dma(chip, stream, buf_size);
                if (*stream).dai_id != 0 {
                    restore_acp_i2s_params(substream, chip, stream);
                } else {
                    restore_acp_pdm_params(substream, chip);
                }
            }
        });
        spin_unlock(&mut (*chip).acp_lock);
    }
    0
}

static rn_dma_pm_ops: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(NULL, rn_pcm_resume) */
    suspend: None,
    resume: Some(rn_pcm_resume),
    ..unsafe { core::mem::zeroed() }
};

static mut renoir_driver: platform_driver = platform_driver {
    probe: Some(renoir_audio_probe),
    remove: Some(renoir_audio_remove),
    driver: device_driver {
        name: b"acp_asoc_renoir\0".as_ptr() as *const core::ffi::c_char,
        pm: pm_ptr!(&rn_dma_pm_ops),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(renoir_driver);

MODULE_DESCRIPTION!("AMD ACP Renoir Driver");
MODULE_IMPORT_NS!("SND_SOC_ACP_COMMON");
MODULE_LICENSE!("Dual BSD/GPL");
MODULE_ALIAS!("platform:acp_asoc_renoir");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
