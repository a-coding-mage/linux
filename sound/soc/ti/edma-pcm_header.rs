/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * edma-pcm.h - eDMA PCM driver using dmaengine for AM3xxx, AM4xxx
 *
 * Copyright (C) 2014 Texas Instruments, Inc.
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 *
 * Based on: sound/soc/tegra/tegra_pcm.h
 */

/* Header guard __EDMA_PCM_H__ omitted in Rust. */

/* External dependency corresponding to C's `struct device`. */
#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

/*
 * C conditional:
 * #if IS_ENABLED(CONFIG_SND_SOC_TI_EDMA_PCM)
 */
#[cfg(CONFIG_SND_SOC_TI_EDMA_PCM)]
extern "C" {
    pub fn edma_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int;
}

/*
 * C fallback:
 * #else
 * static inline int edma_pcm_platform_register(struct device *dev) { return 0; }
 */
#[cfg(not(CONFIG_SND_SOC_TI_EDMA_PCM))]
#[inline]
pub unsafe fn edma_pcm_platform_register(dev: *mut device) -> ::core::ffi::c_int {
    let _ = dev;
    0
}

/* #endif CONFIG_SND_SOC_TI_EDMA_PCM */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
