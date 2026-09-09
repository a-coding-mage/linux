/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Purna Chandra Mandal,<purna.mandal@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

// Dependency: <linux/clk-provider.h>

/* PIC32 clock data */
#[repr(C)]
pub struct pic32_clk_common {
    pub dev: *mut device,
    pub iobase: *mut core::ffi::c_void,
    pub reg_lock: spinlock_t, /* clock lock */
}

/* System PLL clock */
#[repr(C)]
pub struct pic32_sys_pll_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
    pub status_reg: u32,
    pub lock_mask: u32,
}

/* System clock */
#[repr(C)]
pub struct pic32_sys_clk_data {
    pub init_data: clk_init_data,
    pub mux_reg: u32,
    pub slew_reg: u32,
    pub parent_map: *const u32,
    pub slew_div: u32,
}

/* Reference Oscillator clock */
#[repr(C)]
pub struct pic32_ref_osc_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
    pub parent_map: *const u32,
}

/* Peripheral Bus clock */
#[repr(C)]
pub struct pic32_periph_clk_data {
    pub init_data: clk_init_data,
    pub ctrl_reg: u32,
}

/* External Secondary Oscillator clock  */
#[repr(C)]
pub struct pic32_sec_osc_data {
    pub init_data: clk_init_data,
    pub enable_reg: u32,
    pub status_reg: u32,
    pub enable_mask: u32,
    pub status_mask: u32,
    pub fixed_rate: core::ffi::c_ulong,
}

extern "C" {
    pub static pic32_pbclk_ops: clk_ops;
    pub static pic32_sclk_ops: clk_ops;
    pub static pic32_sclk_no_div_ops: clk_ops;
    pub static pic32_spll_ops: clk_ops;
    pub static pic32_roclk_ops: clk_ops;
    pub static pic32_sosc_ops: clk_ops;

    pub fn pic32_periph_clk_register(
        data: *const pic32_periph_clk_data,
        core: *mut pic32_clk_common,
    ) -> *mut clk;
    pub fn pic32_refo_clk_register(
        data: *const pic32_ref_osc_data,
        core: *mut pic32_clk_common,
    ) -> *mut clk;
    pub fn pic32_sys_clk_register(
        data: *const pic32_sys_clk_data,
        core: *mut pic32_clk_common,
    ) -> *mut clk;
    pub fn pic32_spll_clk_register(
        data: *const pic32_sys_pll_data,
        core: *mut pic32_clk_common,
    ) -> *mut clk;
    pub fn pic32_sosc_clk_register(
        data: *const pic32_sec_osc_data,
        core: *mut pic32_clk_common,
    ) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
