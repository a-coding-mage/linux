// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2020 Texas Instruments Incorporated - https://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// C dependencies translated from:
// <linux/module.h>, <sound/core.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, <sound/soc.h>, <sound/dmaengine_pcm.h>,
// and "udma-pcm.h".

use core::ffi::{c_int, c_uint};

extern "C" {
    static snd_dmaengine_pcm_prepare_slave_config: unsafe extern "C" fn();

    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: usize,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prepare_slave_config: unsafe extern "C" fn(),
}

const SNDRV_PCM_INFO_MMAP: usize = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: usize = 1 << 1;
const SNDRV_PCM_INFO_PAUSE: usize = 1 << 2;
const SNDRV_PCM_INFO_RESUME: usize = 1 << 3;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: usize = 1 << 4;
const SNDRV_PCM_INFO_INTERLEAVED: usize = 1 << 5;
const SIZE_MAX: usize = usize::MAX;
const SZ_64K: usize = 64 * 1024;
const UINT_MAX: c_uint = c_uint::MAX;

static udma_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP
        | SNDRV_PCM_INFO_INTERLEAVED,
    buffer_bytes_max: SIZE_MAX,
    period_bytes_min: 32,
    period_bytes_max: SZ_64K,
    periods_min: 2,
    periods_max: UINT_MAX,
};

static udma_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    pcm_hardware: &udma_pcm_hardware,
    prepare_slave_config: snd_dmaengine_pcm_prepare_slave_config,
};

#[no_mangle]
pub unsafe extern "C" fn udma_pcm_platform_register(dev: *mut device) -> c_int {
    unsafe { devm_snd_dmaengine_pcm_register(dev, &udma_dmaengine_pcm_config, 0) }
}

// EXPORT_SYMBOL_GPL(udma_pcm_platform_register);

// MODULE_AUTHOR("Peter Ujfalusi <peter.ujfalusi@ti.com>");
// MODULE_DESCRIPTION("UDMA PCM ASoC platform driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
