/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

use core::ffi::{c_char, c_int};

// C header guard removed: __SDMA_PCM_H__

// #if IS_ENABLED(CONFIG_SND_SOC_TI_SDMA_PCM)
#[cfg(CONFIG_SND_SOC_TI_SDMA_PCM)]
unsafe extern "C" {
    pub fn sdma_pcm_platform_register(
        dev: *mut device,
        txdmachan: *mut c_char,
        rxdmachan: *mut c_char,
    ) -> c_int;
}

// #else
#[cfg(not(CONFIG_SND_SOC_TI_SDMA_PCM))]
pub unsafe fn sdma_pcm_platform_register(
    dev: *mut device,
    txdmachan: *mut c_char,
    rxdmachan: *mut c_char,
) -> c_int {
    let _ = dev;
    let _ = txdmachan;
    let _ = rxdmachan;

    -(ENODEV as c_int)
}
// #endif /* CONFIG_SND_SOC_TI_SDMA_PCM */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
