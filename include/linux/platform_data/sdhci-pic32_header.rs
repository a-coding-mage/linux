/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Purna Chandra Mandal, purna.mandal@microchip.com
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

#[repr(C)]
pub struct pic32_sdhci_platform_data {
    /* read & write fifo threshold */
    pub setup_dma: Option<unsafe extern "C" fn(
        rfifo: u32,
        wfifo: u32,
    ) -> core::ffi::c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
