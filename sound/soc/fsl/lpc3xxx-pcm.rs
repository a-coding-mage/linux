// SPDX-License-Identifier: GPL-2.0-or-later
//
// Author: Kevin Wells <kevin.wells@nxp.com>
//
// Copyright (C) 2008 NXP Semiconductors
// Copyright 2023 Timesys Corporation <piotr.wojtaszczyk@timesys.com>

// C dependencies:
// linux/module.h, linux/init.h, linux/platform_device.h, linux/slab.h,
// linux/dma-mapping.h, linux/amba/pl08x.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/dmaengine_pcm.h, sound/soc.h, "lpc3xxx-i2s.h"

use core::ffi::c_int;
use core::ptr;

const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_U32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

static lpc3xxx_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: STUB_FORMATS,
    period_bytes_min: 128,
    period_bytes_max: 2048,
    periods_min: 2,
    periods_max: 1024,
    buffer_bytes_max: 128 * 1024,
};

static lpc3xxx_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &lpc3xxx_pcm_hardware,
    prepare_slave_config: Some(snd_dmaengine_pcm_prepare_slave_config),
    compat_filter_fn: Some(pl08x_filter_id),
    prealloc_buffer_size: 128 * 1024,
};

static lpc3xxx_soc_platform_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: b"lpc32xx-pcm\0".as_ptr() as *const _,
};

pub unsafe extern "C" fn lpc3xxx_pcm_register(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;

    ret = devm_snd_dmaengine_pcm_register(
        &mut (*pdev).dev,
        &lpc3xxx_dmaengine_pcm_config,
        0,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to register dmaengine: %d\n\0".as_ptr() as *const _,
            ret,
        );
        return ret;
    }

    return devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &lpc3xxx_soc_platform_driver,
        ptr::null_mut(),
        0,
    );
}

// EXPORT_SYMBOL(lpc3xxx_pcm_register);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
