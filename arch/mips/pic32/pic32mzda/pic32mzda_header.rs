/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

/* early clock */
extern "C" {
    pub fn pic32_get_pbclk(bus: core::ffi::c_int) -> u32;
    pub fn pic32_get_sysclk() -> u32;

    /* Device configuration */
    // __init
    pub fn pic32_config_init();
    pub fn pic32_set_lcd_mode(mode: core::ffi::c_int) -> core::ffi::c_int;
    pub fn pic32_set_sdhci_adma_fifo_threshold(rthrs: u32, wthrs: u32) -> core::ffi::c_int;
    pub fn pic32_get_boot_status() -> u32;
    pub fn pic32_disable_lcd() -> core::ffi::c_int;
    pub fn pic32_enable_lcd() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
