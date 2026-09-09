/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic GPIO card-detect helper header
 *
 * Copyright (C) 2011, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

// Translated from the C header. The declarations below depend on the
// corresponding Linux kernel types and implementations supplied elsewhere.

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct mmc_host {
    _private: [u8; 0],
}

extern "C" {
    pub fn mmc_gpio_get_ro(host: *mut mmc_host) -> i32;
    pub fn mmc_gpio_get_cd(host: *mut mmc_host) -> i32;
    pub fn mmc_gpio_set_cd_irq(host: *mut mmc_host, irq: i32);
    pub fn mmc_gpiod_request_cd(
        host: *mut mmc_host,
        con_id: *const c_char,
        idx: u32,
        override_active_level: bool,
        debounce: u32,
    ) -> i32;
    pub fn mmc_gpiod_request_ro(
        host: *mut mmc_host,
        con_id: *const c_char,
        idx: u32,
        debounce: u32,
    ) -> i32;
    pub fn mmc_gpiod_set_cd_config(host: *mut mmc_host, config: c_ulong) -> i32;
    pub fn mmc_gpio_set_cd_wake(host: *mut mmc_host, on: bool) -> i32;
    pub fn mmc_gpiod_request_cd_irq(host: *mut mmc_host);
    pub fn mmc_host_can_gpio_cd(host: *mut mmc_host) -> bool;
    pub fn mmc_host_can_gpio_ro(host: *mut mmc_host) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
