// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>

/*
 * Hardware interface for Audio DSP on Renoir platform
 */

// C includes translated as external dependencies:
// linux/platform_device.h, linux/module.h, ../ops.h, ../sof-audio.h,
// acp.h, acp-dsp-offset.h

pub const I2S_BT_INSTANCE: usize = 0;
pub const I2S_SP_INSTANCE: usize = 1;
pub const PDM_DMIC_INSTANCE: usize = 2;
pub const I2S_SP_VIRTUAL_INSTANCE: usize = 3;

unsafe extern "C" {
    static sof_acp_common_ops: snd_sof_dsp_ops;
}

unsafe extern "C" {
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub rates: u32,
    pub formats: u64,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub id: i32,
    pub name: *const core::ffi::c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: i32,
}

#[repr(C)]
pub struct snd_sof_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_RATE_8000_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
}

#[unsafe(no_mangle)]
pub static mut renoir_sof_dai: [snd_soc_dai_driver; 4] = unsafe {
    [
        snd_soc_dai_driver {
            id: I2S_BT_INSTANCE as i32,
            name: c"acp-sof-bt".as_ptr(),
            playback: snd_soc_pcm_stream {
                rates: SNDRV_PCM_RATE_8000_96000,
                formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
                channels_min: 2,
                channels_max: 8,
                rate_min: 8000,
                rate_max: 96000,
            },
            capture: snd_soc_pcm_stream {
                rates: SNDRV_PCM_RATE_8000_48000,
                formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
                /* Supporting only stereo for I2S BT controller capture */
                channels_min: 2,
                channels_max: 2,
                rate_min: 8000,
                rate_max: 48000,
            },
        },
        snd_soc_dai_driver {
            id: I2S_SP_INSTANCE as i32,
            name: c"acp-sof-sp".as_ptr(),
            playback: snd_soc_pcm_stream {
                rates: SNDRV_PCM_RATE_8000_96000,
                formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
                channels_min: 2,
                channels_max: 8,
                rate_min: 8000,
                rate_max: 96000,
            },
            capture: snd_soc_pcm_stream {
                rates: SNDRV_PCM_RATE_8000_48000,
                formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
                /* Supporting only stereo for I2S SP controller capture */
                channels_min: 2,
                channels_max: 2,
                rate_min: 8000,
                rate_max: 48000,
            },
        },
        snd_soc_dai_driver {
            id: PDM_DMIC_INSTANCE as i32,
            name: c"acp-sof-dmic".as_ptr(),
            playback: snd_soc_pcm_stream {
                rates: 0,
                formats: 0,
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
            },
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
            id: I2S_SP_VIRTUAL_INSTANCE as i32,
            name: c"acp-sof-sp-virtual".as_ptr(),
            playback: snd_soc_pcm_stream {
                rates: SNDRV_PCM_RATE_8000_96000,
                formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_LE,
                channels_min: 2,
                channels_max: 8,
                rate_min: 8000,
                rate_max: 96000,
            },
            capture: snd_soc_pcm_stream {
                rates: 0,
                formats: 0,
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
            },
        },
    ]
};

/* Renoir ops */
#[unsafe(no_mangle)]
pub static mut sof_renoir_ops: snd_sof_dsp_ops = snd_sof_dsp_ops {
    drv: core::ptr::null_mut(),
    num_drv: 0,
};

// EXPORT_SYMBOL_NS(sof_renoir_ops, "SND_SOC_SOF_AMD_COMMON");

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sof_renoir_ops_init(sdev: *mut snd_sof_dev) -> core::ffi::c_int {
    let _ = sdev;

    /* common defaults */
    unsafe {
        memcpy(
            (&raw mut sof_renoir_ops).cast::<core::ffi::c_void>(),
            (&raw const sof_acp_common_ops).cast::<core::ffi::c_void>(),
            core::mem::size_of::<snd_sof_dsp_ops>(),
        );

        sof_renoir_ops.drv = (&raw mut renoir_sof_dai).cast::<snd_soc_dai_driver>();
        sof_renoir_ops.num_drv = renoir_sof_dai.len() as i32;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
