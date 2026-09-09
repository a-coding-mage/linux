/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * spear machine family generic header file
 *
 * Copyright (C) 2009-2012 ST Microelectronics
 * Rajeev Kumar <rajeev-dlh.kumar@st.com>
 * Viresh Kumar <vireshk@kernel.org>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/dmaengine.h, linux/amba/pl08x.h, linux/init.h, linux/reboot.h,
// and asm/mach/time.h.

use core::ffi::{c_char, c_int, c_uint};

// Opaque declarations corresponding to externally supplied C types.
pub enum pl022_ssp_controller {}
pub enum pl08x_platform_data {}
pub enum smp_operations {}

// The C enum reboot_mode is represented by its ABI integer type here; its
// concrete definition is supplied by the surrounding dependency.
pub type reboot_mode = c_int;

extern "C" {
    pub static mut spear_pen_release: c_int;

    pub fn spear13xx_timer_init();
    pub fn spear3xx_timer_init();
    pub static mut pl022_plat_data: pl022_ssp_controller;
    pub static mut pl080_plat_data: pl08x_platform_data;

    // C __init annotations have no direct Rust equivalent.
    pub fn spear_setup_of_timer();
    pub fn spear3xx_map_io();
    pub fn spear3xx_dt_init_irq();
    pub fn spear13xx_map_io();
    pub fn spear13xx_l2x0_init();

    pub fn spear_restart(mode: reboot_mode, cmd: *const c_char);

    pub fn spear13xx_secondary_startup();
    pub fn spear13xx_cpu_die(cpu: c_uint);

    pub static spear13xx_smp_ops: smp_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
