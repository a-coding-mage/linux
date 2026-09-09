/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP WakeupGen header file
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 */

/* OMAP4 and OMAP5 has same base address */
pub const OMAP_WKUPGEN_BASE: u32 = 0x4828_1000;

pub const OMAP_WKG_CONTROL_0: u32 = 0x00;
pub const OMAP_WKG_ENB_A_0: u32 = 0x10;
pub const OMAP_WKG_ENB_B_0: u32 = 0x14;
pub const OMAP_WKG_ENB_C_0: u32 = 0x18;
pub const OMAP_WKG_ENB_D_0: u32 = 0x1c;
pub const OMAP_WKG_ENB_E_0: u32 = 0x20;
pub const OMAP_WKG_ENB_A_1: u32 = 0x410;
pub const OMAP_WKG_ENB_B_1: u32 = 0x414;
pub const OMAP_WKG_ENB_C_1: u32 = 0x418;
pub const OMAP_WKG_ENB_D_1: u32 = 0x41c;
pub const OMAP_WKG_ENB_E_1: u32 = 0x420;
pub const OMAP_AUX_CORE_BOOT_0: u32 = 0x800;
pub const OMAP_AUX_CORE_BOOT_1: u32 = 0x804;
pub const OMAP_AMBA_IF_MODE: u32 = 0x80c;
pub const OMAP_PTMSYNCREQ_MASK: u32 = 0xc00;
pub const OMAP_PTMSYNCREQ_EN: u32 = 0xc04;
pub const OMAP_TIMESTAMPCYCLELO: u32 = 0xc08;
pub const OMAP_TIMESTAMPCYCLEHI: u32 = 0xc0c;

extern "C" {
    pub fn omap_get_wakeupgen_base() -> *mut core::ffi::c_void;
    pub fn omap_secure_apis_support() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
