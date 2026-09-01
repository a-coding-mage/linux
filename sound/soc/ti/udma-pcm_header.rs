/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
 */

// C header dependency: struct device is declared by included kernel headers.
// Rust translation references the corresponding external type name.

#[cfg(CONFIG_SND_SOC_TI_UDMA_PCM)]
extern "C" {
    pub fn udma_pcm_platform_register(dev: *mut device) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_SND_SOC_TI_UDMA_PCM))]
#[inline]
pub unsafe fn udma_pcm_platform_register(dev: *mut device) -> core::ffi::c_int {
    let _ = dev;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
