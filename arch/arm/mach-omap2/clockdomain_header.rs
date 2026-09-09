/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3 clockdomain framework functions
 *
 * Copyright (C) 2008, 2012 Texas Instruments, Inc.
 * Copyright (C) 2008-2011 Nokia Corporation
 *
 * Paul Walmsley
 */

// Linux and platform dependencies are supplied by the surrounding translation.

/* Clockdomain flags */
pub const CLKDM_CAN_FORCE_SLEEP: u32 = 1 << 0;
pub const CLKDM_CAN_FORCE_WAKEUP: u32 = 1 << 1;
pub const CLKDM_CAN_ENABLE_AUTO: u32 = 1 << 2;
pub const CLKDM_CAN_DISABLE_AUTO: u32 = 1 << 3;
pub const CLKDM_NO_AUTODEPS: u32 = 1 << 4;
pub const CLKDM_ACTIVE_WITH_MPU: u32 = 1 << 5;
pub const CLKDM_MISSING_IDLE_REPORTING: u32 = 1 << 6;
pub const CLKDM_STANDBY_FORCE_WAKEUP: u32 = 1 << 7;

pub const CLKDM_CAN_HWSUP: u32 = CLKDM_CAN_ENABLE_AUTO | CLKDM_CAN_DISABLE_AUTO;
pub const CLKDM_CAN_SWSUP: u32 = CLKDM_CAN_FORCE_SLEEP | CLKDM_CAN_FORCE_WAKEUP;
pub const CLKDM_CAN_HWSUP_SWSUP: u32 = CLKDM_CAN_SWSUP | CLKDM_CAN_HWSUP;

#[repr(C)]
#[derive(Copy, Clone)]
pub union clkdm_autodep_clkdm {
    pub name: *const ::core::ffi::c_char,
    pub ptr: *mut clockdomain,
}

#[repr(C)]
pub struct clkdm_autodep {
    pub clkdm: clkdm_autodep_clkdm,
}

#[repr(C)]
pub struct clkdm_dep {
    pub clkdm_name: *const ::core::ffi::c_char,
    pub clkdm: *mut clockdomain,
    pub wkdep_usecount: i16,
    pub sleepdep_usecount: i16,
}

pub struct omap_hwmod;

#[repr(C)]
#[derive(Copy, Clone)]
pub union clockdomain_pwrdm {
    pub name: *const ::core::ffi::c_char,
    pub ptr: *mut powerdomain,
}

#[repr(C)]
pub struct clockdomain {
    pub name: *const ::core::ffi::c_char,
    pub pwrdm: clockdomain_pwrdm,
    pub clktrctrl_mask: u16,
    pub flags: u8,
    pub _flags: u8,
    pub dep_bit: u8,
    pub prcm_partition: u8,
    pub cm_inst: u16,
    pub clkdm_offs: u16,
    pub wkdep_srcs: *mut clkdm_dep,
    pub sleepdep_srcs: *mut clkdm_dep,
    pub usecount: ::core::ffi::c_int,
    pub forcewake_count: ::core::ffi::c_int,
    pub node: list_head,
    pub context: u32,
}

#[repr(C)]
pub struct clkdm_ops {
    pub clkdm_add_wkdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_del_wkdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_read_wkdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_clear_all_wkdeps: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_add_sleepdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_del_sleepdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_read_sleepdep: Option<unsafe extern "C" fn(*mut clockdomain, *mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_clear_all_sleepdeps: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_sleep: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_wakeup: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_allow_idle: Option<unsafe extern "C" fn(*mut clockdomain)>,
    pub clkdm_deny_idle: Option<unsafe extern "C" fn(*mut clockdomain)>,
    pub clkdm_clk_enable: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_clk_disable: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_save_context: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
    pub clkdm_restore_context: Option<unsafe extern "C" fn(*mut clockdomain) -> ::core::ffi::c_int>,
}

extern "C" {
    pub fn clkdm_register_platform_funcs(co: *mut clkdm_ops) -> ::core::ffi::c_int;
    pub fn clkdm_register_autodeps(ia: *mut clkdm_autodep) -> ::core::ffi::c_int;
    pub fn clkdm_register_clkdms(c: *mut *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_complete_init() -> ::core::ffi::c_int;
    pub fn clkdm_lookup(name: *const ::core::ffi::c_char) -> *mut clockdomain;
    pub fn clkdm_for_each(fn_: Option<unsafe extern "C" fn(*mut clockdomain, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, user: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn clkdm_get_pwrdm(clkdm: *mut clockdomain) -> *mut powerdomain;
    pub fn clkdm_add_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_del_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_read_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_clear_all_wkdeps(clkdm: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_add_sleepdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_del_sleepdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_read_sleepdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_clear_all_sleepdeps(clkdm: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_allow_idle_nolock(clkdm: *mut clockdomain);
    pub fn clkdm_allow_idle(clkdm: *mut clockdomain);
    pub fn clkdm_deny_idle_nolock(clkdm: *mut clockdomain);
    pub fn clkdm_deny_idle(clkdm: *mut clockdomain);
    pub fn clkdm_wakeup(clkdm: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_sleep(clkdm: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn clkdm_clk_enable(clkdm: *mut clockdomain, clk: *mut clk) -> ::core::ffi::c_int;
    pub fn clkdm_clk_disable(clkdm: *mut clockdomain, clk: *mut clk) -> ::core::ffi::c_int;
    pub fn clkdm_hwmod_enable(clkdm: *mut clockdomain, oh: *mut omap_hwmod) -> ::core::ffi::c_int;
    pub fn clkdm_hwmod_disable(clkdm: *mut clockdomain, oh: *mut omap_hwmod) -> ::core::ffi::c_int;
    pub fn clkdm_save_context();
    pub fn clkdm_restore_context();
    pub fn omap242x_clockdomains_init();
    pub fn omap243x_clockdomains_init();
    pub fn omap3xxx_clockdomains_init();
    pub fn am33xx_clockdomains_init();
    pub fn ti814x_clockdomains_init();
    pub fn ti816x_clockdomains_init();
    pub fn omap44xx_clockdomains_init();
    pub fn omap54xx_clockdomains_init();
    pub fn dra7xx_clockdomains_init();
    pub fn am43xx_clockdomains_init();
    pub fn clkdm_add_autodeps(clkdm: *mut clockdomain);
    pub fn clkdm_del_autodeps(clkdm: *mut clockdomain);
    pub static mut omap2_clkdm_operations: clkdm_ops;
    pub static mut omap3_clkdm_operations: clkdm_ops;
    pub static mut omap4_clkdm_operations: clkdm_ops;
    pub static mut am33xx_clkdm_operations: clkdm_ops;
    pub static mut am43xx_clkdm_operations: clkdm_ops;
    pub static mut gfx_24xx_wkdeps: [clkdm_dep; 0];
    pub static mut dsp_24xx_wkdeps: [clkdm_dep; 0];
    pub static mut wkup_common_clkdm: clockdomain;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
