/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Core functions for Marvell System On Chip
 *
 * Copyright (C) 2012 Marvell
 *
 * Lior Amsalem <alior@marvell.com>
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

use core::ffi::{c_char, c_void};

// `enum reboot_mode` is supplied by linux/reboot.h; its C ABI representation
// is an integer.
pub type reboot_mode = i32;

unsafe extern "C" {
    pub fn mvebu_restart(mode: reboot_mode, cmd: *const c_char);
    pub fn mvebu_cpu_reset_deassert(cpu: i32) -> i32;
    pub fn mvebu_pmsu_set_cpu_boot_addr(hw_cpu: i32, boot_addr: *mut c_void);
    pub fn mvebu_system_controller_set_cpu_boot_addr(boot_addr: *mut c_void);
    pub fn mvebu_system_controller_get_soc_id(dev: *mut u32, rev: *mut u32) -> i32;

    pub fn mvebu_get_scu_base() -> *mut c_void;

    pub fn mvebu_pm_suspend_init(
        board_pm_enter: Option<unsafe extern "C" fn(sdram_reg: *mut c_void, srcmd: u32)>,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
