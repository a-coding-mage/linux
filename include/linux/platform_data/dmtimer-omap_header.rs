/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * DMTIMER platform data for TI OMAP platforms
 *
 * Copyright (C) 2012 Texas Instruments
 * Author: Jon Hunter <jon-hunter@ti.com>
 */

use core::ffi::c_int;

// Opaque types supplied by the surrounding kernel interfaces.
pub enum omap_dm_timer {}
pub enum device_node {}
pub enum device {}
pub enum platform_device {}
pub enum clk {}

#[repr(C)]
pub struct omap_dm_timer_ops {
    pub request_by_node: Option<unsafe extern "C" fn(np: *mut device_node) -> *mut omap_dm_timer>,
    pub request_specific: Option<unsafe extern "C" fn(timer_id: c_int) -> *mut omap_dm_timer>,
    pub request: Option<unsafe extern "C" fn() -> *mut omap_dm_timer>,

    pub free: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,

    pub enable: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer)>,
    pub disable: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer)>,

    pub get_irq: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,
    pub set_int_enable:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, value: u32) -> c_int>,
    pub set_int_disable:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, mask: u32) -> c_int>,

    pub get_fclk:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> *mut clk>,

    pub start: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,
    pub set_source:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, source: c_int) -> c_int>,

    pub set_load:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, value: u32) -> c_int>,
    pub set_match:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, enable: c_int, r#match: u32) -> c_int>,
    pub set_pwm: Option<unsafe extern "C" fn(
        timer: *mut omap_dm_timer,
        def_on: c_int,
        toggle: c_int,
        trigger: c_int,
        autoreload: c_int,
    ) -> c_int>,
    pub get_pwm_status: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,
    pub set_cap: Option<unsafe extern "C" fn(
        timer: *mut omap_dm_timer,
        autoreload: c_int,
        config_period: bool,
    ) -> c_int>,
    pub get_cap_status: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> c_int>,
    pub set_prescaler:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, prescaler: c_int) -> c_int>,

    pub read_counter: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> u32>,
    pub read_cap:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, is_period: bool) -> u32>,
    pub write_counter:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, value: u32) -> c_int>,
    pub read_status: Option<unsafe extern "C" fn(timer: *mut omap_dm_timer) -> u32>,
    pub write_status:
        Option<unsafe extern "C" fn(timer: *mut omap_dm_timer, value: u32) -> c_int>,
}

#[repr(C)]
pub struct dmtimer_platform_data {
    /* set_timer_src - Only used for OMAP1 devices */
    pub set_timer_src:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, source: c_int) -> c_int>,
    pub timer_capability: u32,
    pub timer_errata: u32,
    pub get_context_loss_count: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub timer_ops: *const omap_dm_timer_ops,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
