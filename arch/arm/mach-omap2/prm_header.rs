/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3/4 Power/Reset Management (PRM) bitfield definitions
 *
 * Copyright (C) 2007-2009, 2012 Texas Instruments, Inc.
 * Copyright (C) 2010 Nokia Corporation
 *
 * Paul Walmsley
 */

// Dependency supplied by prcm-common.h in the original C header.

extern "C" {
    pub static mut prm_base: omap_domain_base;
    pub static mut prm_features: u16;
    pub static mut prm_reboot_mode: reboot_mode;
    pub fn omap_prcm_init() -> i32;
    pub fn omap2_prcm_base_init() -> i32;
}

/*
 * prm_features flag values
 *
 * PRM_HAS_IO_WAKEUP: has IO wakeup capability
 * PRM_HAS_VOLTAGE: has voltage domains
 */
pub const PRM_HAS_IO_WAKEUP: u32 = 1 << 0;
pub const PRM_HAS_VOLTAGE: u32 = 1 << 1;

/*
 * MAX_MODULE_SOFTRESET_WAIT: Maximum microseconds to wait for OMAP
 * module to softreset
 */
pub const MAX_MODULE_SOFTRESET_WAIT: u32 = 10000;

/*
 * MAX_MODULE_HARDRESET_WAIT: Maximum microseconds to wait for an OMAP
 * submodule to exit hardreset
 */
pub const MAX_MODULE_HARDRESET_WAIT: u32 = 10000;

/* Register bitfields */

/*
 * 24XX: PM_PWSTST_CORE, PM_PWSTST_GFX, PM_PWSTST_MPU, PM_PWSTST_DSP
 *
 * 2430: PM_PWSTST_MDM
 *
 * 3430: PM_PWSTST_IVA2, PM_PWSTST_MPU, PM_PWSTST_CORE, PM_PWSTST_GFX,
 *       PM_PWSTST_DSS, PM_PWSTST_CAM, PM_PWSTST_PER, PM_PWSTST_EMU,
 *       PM_PWSTST_NEON
 */
pub const OMAP_INTRANSITION_MASK: u32 = 1 << 20;

/* Power state status/control fields. */
pub const OMAP_POWERSTATEST_SHIFT: u32 = 0;
pub const OMAP_POWERSTATEST_MASK: u32 = 0x3 << 0;
pub const OMAP_POWERSTATE_SHIFT: u32 = 0;
pub const OMAP_POWERSTATE_MASK: u32 = 0x3 << 0;

/*
 * Standardized OMAP reset source bits
 *
 * To the extent these happen to match the hardware register bit
 * shifts, it's purely coincidental. Used by omap-wdt.c.
 * OMAP_UNKNOWN_RST_SRC_ID_SHIFT is a special value, used whenever
 * there are any bits remaining in the global PRM_RSTST register that
 * haven't been identified, or when the PRM code for the current SoC
 * doesn't know how to interpret the register.
 */
pub const OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT: u32 = 0;
pub const OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT: u32 = 1;
pub const OMAP_SECU_VIOL_RST_SRC_ID_SHIFT: u32 = 2;
pub const OMAP_MPU_WD_RST_SRC_ID_SHIFT: u32 = 3;
pub const OMAP_SECU_WD_RST_SRC_ID_SHIFT: u32 = 4;
pub const OMAP_EXTWARM_RST_SRC_ID_SHIFT: u32 = 5;
pub const OMAP_VDD_MPU_VM_RST_SRC_ID_SHIFT: u32 = 6;
pub const OMAP_VDD_IVA_VM_RST_SRC_ID_SHIFT: u32 = 7;
pub const OMAP_VDD_CORE_VM_RST_SRC_ID_SHIFT: u32 = 8;
pub const OMAP_ICEPICK_RST_SRC_ID_SHIFT: u32 = 9;
pub const OMAP_ICECRUSHER_RST_SRC_ID_SHIFT: u32 = 10;
pub const OMAP_C2C_RST_SRC_ID_SHIFT: u32 = 11;
pub const OMAP_UNKNOWN_RST_SRC_ID_SHIFT: u32 = 12;

#[repr(C)]
pub struct prm_reset_src_map {
    pub reg_shift: i8,
    pub std_shift: i8,
}

#[repr(C)]
pub struct prm_ll_data {
    pub read_reset_sources: Option<unsafe extern "C" fn() -> u32>,
    pub was_any_context_lost_old: Option<unsafe extern "C" fn(u8, i16, u16) -> bool>,
    pub clear_context_loss_flags_old: Option<unsafe extern "C" fn(u8, i16, u16)>,
    pub late_init: Option<unsafe extern "C" fn() -> i32>,
    pub assert_hardreset: Option<unsafe extern "C" fn(u8, u8, i16, u16) -> i32>,
    pub deassert_hardreset: Option<unsafe extern "C" fn(u8, u8, u8, i16, u16, u16) -> i32>,
    pub is_hardreset_asserted: Option<unsafe extern "C" fn(u8, u8, i16, u16) -> i32>,
    pub reset_system: Option<unsafe extern "C" fn()>,
    pub clear_mod_irqs: Option<unsafe extern "C" fn(i16, u8, u32) -> i32>,
    pub vp_check_txdone: Option<unsafe extern "C" fn(u8) -> u32>,
    pub vp_clear_txdone: Option<unsafe extern "C" fn(u8)>,
}

extern "C" {
    pub fn prm_register(pld: *mut prm_ll_data) -> i32;
    pub fn prm_unregister(pld: *mut prm_ll_data) -> i32;
    pub fn omap_prm_assert_hardreset(shift: u8, part: u8, prm_mod: i16, offset: u16) -> i32;
    pub fn omap_prm_deassert_hardreset(shift: u8, st_shift: u8, part: u8, prm_mod: i16, offset: u16, st_offset: u16) -> i32;
    pub fn omap_prm_is_hardreset_asserted(shift: u8, part: u8, prm_mod: i16, offset: u16) -> i32;
    pub fn prm_was_any_context_lost_old(part: u8, inst: i16, idx: u16) -> bool;
    pub fn prm_clear_context_loss_flags_old(part: u8, inst: i16, idx: u16);
    pub fn omap_prm_reset_system();
    pub fn omap_prm_clear_mod_irqs(module: i16, regs: u8, wkst_mask: u32) -> i32;
}

/* Voltage Processor (VP) identifiers */
pub const OMAP3_VP_VDD_MPU_ID: u32 = 0;
pub const OMAP3_VP_VDD_CORE_ID: u32 = 1;
pub const OMAP4_VP_VDD_CORE_ID: u32 = 0;
pub const OMAP4_VP_VDD_IVA_ID: u32 = 1;
pub const OMAP4_VP_VDD_MPU_ID: u32 = 2;

extern "C" {
    pub fn omap_prm_vp_check_txdone(vp_id: u8) -> u32;
    pub fn omap_prm_vp_clear_txdone(vp_id: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
