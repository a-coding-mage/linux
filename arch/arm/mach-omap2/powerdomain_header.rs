/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2/3/4 powerdomain control
 *
 * Copyright (C) 2007-2008, 2010 Texas Instruments, Inc.
 * Copyright (C) 2007-2011 Nokia Corporation
 *
 * Paul Walmsley
 *
 * XXX This should be moved to the mach-omap2/ directory at the earliest
 * opportunity.
 */

// Dependencies supplied by other translation units: linux types, list, and spinlock definitions.

/* Powerdomain basic power states */
pub const PWRDM_POWER_OFF: u32 = 0x0;
pub const PWRDM_POWER_RET: u32 = 0x1;
pub const PWRDM_POWER_INACTIVE: u32 = 0x2;
pub const PWRDM_POWER_ON: u32 = 0x3;

pub const PWRDM_MAX_PWRSTS: usize = 4;

/* Powerdomain allowable state bitfields */
pub const PWRSTS_ON: u32 = 1 << PWRDM_POWER_ON;
pub const PWRSTS_INACTIVE: u32 = 1 << PWRDM_POWER_INACTIVE;
pub const PWRSTS_RET: u32 = 1 << PWRDM_POWER_RET;
pub const PWRSTS_OFF: u32 = 1 << PWRDM_POWER_OFF;
pub const PWRSTS_OFF_ON: u32 = PWRSTS_OFF | PWRSTS_ON;
pub const PWRSTS_OFF_RET: u32 = PWRSTS_OFF | PWRSTS_RET;
pub const PWRSTS_RET_ON: u32 = PWRSTS_RET | PWRSTS_ON;
pub const PWRSTS_OFF_RET_ON: u32 = PWRSTS_OFF_RET | PWRSTS_ON;
pub const PWRSTS_INA_ON: u32 = PWRSTS_INACTIVE | PWRSTS_ON;

/* Powerdomain flags (struct powerdomain.flags) */
pub const PWRDM_HAS_HDWR_SAR: u32 = 1 << 0;
pub const PWRDM_HAS_MPU_QUIRK: u32 = 1 << 1;
pub const PWRDM_HAS_LOWPOWERSTATECHANGE: u32 = 1 << 2;

pub const PWRDM_MAX_MEM_BANKS: usize = 5;
pub const PWRDM_MAX_CLKDMS: usize = 11;
pub const PWRDM_TRANSITION_BAILOUT: u32 = 100000;

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct voltagedomain {
    _private: [u8; 0],
}

#[repr(C)]
pub union powerdomain_voltdm {
    pub name: *const ::core::ffi::c_char,
    pub ptr: *mut voltagedomain,
}

#[repr(C)]
pub struct powerdomain {
    pub name: *const ::core::ffi::c_char,
    pub voltdm: powerdomain_voltdm,
    pub prcm_offs: i16,
    pub pwrsts: u8,
    pub pwrsts_logic_ret: u8,
    pub flags: u8,
    pub banks: u8,
    pub pwrsts_mem_ret: [u8; PWRDM_MAX_MEM_BANKS],
    pub pwrsts_mem_on: [u8; PWRDM_MAX_MEM_BANKS],
    pub prcm_partition: u8,
    pub pwrdm_clkdms: [*mut clockdomain; PWRDM_MAX_CLKDMS],
    pub node: list_head,
    pub voltdm_node: list_head,
    pub state: ::core::ffi::c_int,
    pub state_counter: [u32; PWRDM_MAX_PWRSTS],
    pub ret_logic_off_counter: u32,
    pub ret_mem_off_counter: [u32; PWRDM_MAX_MEM_BANKS],
    pub _lock: spinlock_t,
    pub _lock_flags: ::core::ffi::c_ulong,
    pub pwrstctrl_offs: u8,
    pub pwrstst_offs: u8,
    pub logicretstate_mask: u32,
    pub mem_on_mask: [u32; PWRDM_MAX_MEM_BANKS],
    pub mem_ret_mask: [u32; PWRDM_MAX_MEM_BANKS],
    pub mem_pwrst_mask: [u32; PWRDM_MAX_MEM_BANKS],
    pub mem_retst_mask: [u32; PWRDM_MAX_MEM_BANKS],
    // CONFIG_PM_DEBUG fields are build-time conditional in the C header.
    #[cfg(CONFIG_PM_DEBUG)]
    pub timer: i64,
    #[cfg(CONFIG_PM_DEBUG)]
    pub state_timer: [i64; PWRDM_MAX_PWRSTS],
    pub context: u32,
}

#[repr(C)]
pub struct pwrdm_ops {
    pub pwrdm_set_next_pwrst: Option<unsafe extern "C" fn(*mut powerdomain, u8) -> ::core::ffi::c_int>,
    pub pwrdm_read_next_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_read_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_read_prev_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_set_logic_retst: Option<unsafe extern "C" fn(*mut powerdomain, u8) -> ::core::ffi::c_int>,
    pub pwrdm_set_mem_onst: Option<unsafe extern "C" fn(*mut powerdomain, u8, u8) -> ::core::ffi::c_int>,
    pub pwrdm_set_mem_retst: Option<unsafe extern "C" fn(*mut powerdomain, u8, u8) -> ::core::ffi::c_int>,
    pub pwrdm_read_logic_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_read_prev_logic_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_read_logic_retst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_read_mem_pwrst: Option<unsafe extern "C" fn(*mut powerdomain, u8) -> ::core::ffi::c_int>,
    pub pwrdm_read_prev_mem_pwrst: Option<unsafe extern "C" fn(*mut powerdomain, u8) -> ::core::ffi::c_int>,
    pub pwrdm_read_mem_retst: Option<unsafe extern "C" fn(*mut powerdomain, u8) -> ::core::ffi::c_int>,
    pub pwrdm_clear_all_prev_pwrst: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_enable_hdwr_sar: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_disable_hdwr_sar: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_set_lowpwrstchange: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_wait_transition: Option<unsafe extern "C" fn(*mut powerdomain) -> ::core::ffi::c_int>,
    pub pwrdm_has_voltdm: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub pwrdm_save_context: Option<unsafe extern "C" fn(*mut powerdomain)>,
    pub pwrdm_restore_context: Option<unsafe extern "C" fn(*mut powerdomain)>,
}

extern "C" {
    pub fn pwrdm_register_platform_funcs(custom_funcs: *mut pwrdm_ops) -> ::core::ffi::c_int;
    pub fn pwrdm_register_pwrdms(pwrdm_list: *mut *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_complete_init() -> ::core::ffi::c_int;
    pub fn pwrdm_lookup(name: *const ::core::ffi::c_char) -> *mut powerdomain;
    pub fn pwrdm_for_each(fn_: Option<unsafe extern "C" fn(*mut powerdomain, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>, user: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn pwrdm_add_clkdm(pwrdm: *mut powerdomain, clkdm: *mut clockdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_get_mem_bank_count(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_get_valid_lp_state(pwrdm: *mut powerdomain, is_logic_state: bool, req_state: u8) -> u8;
    pub fn pwrdm_set_next_pwrst(pwrdm: *mut powerdomain, pwrst: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_read_next_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_read_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_read_prev_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_clear_all_prev_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_set_logic_retst(pwrdm: *mut powerdomain, pwrst: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_set_mem_onst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_set_mem_retst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_read_logic_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_read_prev_logic_pwrst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_read_logic_retst(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_read_mem_pwrst(pwrdm: *mut powerdomain, bank: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_read_prev_mem_pwrst(pwrdm: *mut powerdomain, bank: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_read_mem_retst(pwrdm: *mut powerdomain, bank: u8) -> ::core::ffi::c_int;
    pub fn pwrdm_enable_hdwr_sar(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_disable_hdwr_sar(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_has_hdwr_sar(pwrdm: *mut powerdomain) -> bool;
    pub fn pwrdm_state_switch_nolock(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_state_switch(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_pre_transition(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn pwrdm_post_transition(pwrdm: *mut powerdomain) -> ::core::ffi::c_int;
    pub fn omap_set_pwrdm_state(pwrdm: *mut powerdomain, state: u8) -> ::core::ffi::c_int;
    pub fn omap242x_powerdomains_init();
    pub fn omap243x_powerdomains_init();
    pub fn omap3xxx_powerdomains_init();
    pub fn am33xx_powerdomains_init();
    pub fn omap44xx_powerdomains_init();
    pub fn omap54xx_powerdomains_init();
    pub fn dra7xx_powerdomains_init();
    pub fn am43xx_powerdomains_init();
    pub static mut omap2_pwrdm_operations: pwrdm_ops;
    pub static mut omap3_pwrdm_operations: pwrdm_ops;
    pub static mut am33xx_pwrdm_operations: pwrdm_ops;
    pub static mut omap4_pwrdm_operations: pwrdm_ops;
    pub fn omap2_pwrdm_get_mem_bank_onstate_mask(bank: u8) -> u32;
    pub fn omap2_pwrdm_get_mem_bank_retst_mask(bank: u8) -> u32;
    pub fn omap2_pwrdm_get_mem_bank_stst_mask(bank: u8) -> u32;
    pub static mut wkup_omap2_pwrdm: powerdomain;
    pub static mut gfx_omap2_pwrdm: powerdomain;
    pub fn pwrdm_lock(pwrdm: *mut powerdomain);
    pub fn pwrdm_unlock(pwrdm: *mut powerdomain);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
