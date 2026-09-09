/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2012 Pavel Machek <pavel@denx.de>
 * Copyright (C) 2012-2015 Altera Corporation
 */

// Translated from the C header; include dependencies are supplied externally.

pub const SOCFPGA_RSTMGR_CTRL: u32 = 0x04;
pub const SOCFPGA_RSTMGR_MODMPURST: u32 = 0x10;
pub const SOCFPGA_RSTMGR_MODPERRST: u32 = 0x14;
pub const SOCFPGA_RSTMGR_BRGMODRST: u32 = 0x1c;

pub const SOCFPGA_A10_RSTMGR_CTRL: u32 = 0xC;
pub const SOCFPGA_A10_RSTMGR_MODMPURST: u32 = 0x20;

/* System Manager bits */
pub const RSTMGR_CTRL_SWCOLDRSTREQ: u32 = 0x1; // Cold Reset
pub const RSTMGR_CTRL_SWWARMRSTREQ: u32 = 0x2; // Warm Reset

pub const RSTMGR_MPUMODRST_CPU1: u32 = 0x2; // CPU1 Reset

extern "C" {
    pub fn socfpga_init_l2_ecc();
    pub fn socfpga_init_ocram_ecc();
    pub fn socfpga_init_arria10_l2_ecc();
    pub fn socfpga_init_arria10_ocram_ecc();

    pub static mut sys_manager_base_addr: *mut core::ffi::c_void;
    pub static mut rst_manager_base_addr: *mut core::ffi::c_void;
    pub static mut sdr_ctl_base_addr: *mut core::ffi::c_void;

    pub fn socfpga_sdram_self_refresh(sdr_base: u32) -> u32;
    pub static mut socfpga_sdram_self_refresh_sz: core::ffi::c_uint;

    pub static mut secondary_trampoline: [core::ffi::c_char; 0];
    pub static mut secondary_trampoline_end: [core::ffi::c_char; 0];

    pub static mut socfpga_cpu1start_addr: core::ffi::c_ulong;
}

pub const SOCFPGA_SCU_VIRT_BASE: u32 = 0xfee00000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
