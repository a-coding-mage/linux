/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP2+ Clock Management prototypes
 *
 * Copyright (C) 2007-2009, 2012 Texas Instruments, Inc.
 * Copyright (C) 2007-2009 Nokia Corporation
 *
 * Written by Paul Walmsley
 */

/*
 * MAX_MODULE_READY_TIME: max duration in microseconds to wait for the
 * PRCM to request that a module exit the inactive state in the case of
 * OMAP2 & 3.
 * In the case of OMAP4 this is the max duration in microseconds for the
 * module to reach the functionnal state from an inactive state.
 */
pub const MAX_MODULE_READY_TIME: i32 = 2000;

/* Dependencies supplied by other translation units:
 * linux/clk/ti.h
 * prcm-common.h
 */

pub static mut cm_base: omap_domain_base = unsafe { core::mem::zeroed() };
pub static mut cm2_base: omap_domain_base = unsafe { core::mem::zeroed() };

/*
 * MAX_MODULE_DISABLE_TIME: max duration in microseconds to wait for
 * the PRCM to request that a module enter the inactive state in the
 * case of OMAP2 & 3.  In the case of OMAP4 this is the max duration
 * in microseconds for the module to reach the inactive state from
 * a functional state.
 * XXX FSUSB on OMAP4430 takes ~4ms to idle after reset during
 * kernel init.
 */
pub const MAX_MODULE_DISABLE_TIME: i32 = 5000;

/**
 * struct cm_ll_data - fn ptrs to per-SoC CM function implementations
 * @split_idlest_reg: ptr to the SoC CM-specific split_idlest_reg impl
 * @wait_module_ready: ptr to the SoC CM-specific wait_module_ready impl
 * @wait_module_idle: ptr to the SoC CM-specific wait_module_idle impl
 * @module_enable: ptr to the SoC CM-specific module_enable impl
 * @module_disable: ptr to the SoC CM-specific module_disable impl
 * @xlate_clkctrl: ptr to the SoC CM-specific clkctrl xlate addr impl
 */
#[repr(C)]
pub struct cm_ll_data {
    pub split_idlest_reg: Option<unsafe extern "C" fn(
        idlest_reg: *mut clk_omap_reg,
        prcm_inst: *mut i16,
        idlest_reg_id: *mut u8,
    ) -> i32>,
    pub wait_module_ready: Option<unsafe extern "C" fn(
        part: u8,
        prcm_mod: i16,
        idlest_reg: u16,
        idlest_shift: u8,
    ) -> i32>,
    pub wait_module_idle: Option<unsafe extern "C" fn(
        part: u8,
        prcm_mod: i16,
        idlest_reg: u16,
        idlest_shift: u8,
    ) -> i32>,
    pub module_enable: Option<unsafe extern "C" fn(
        mode: u8,
        part: u8,
        inst: u16,
        clkctrl_offs: u16,
    )>,
    pub module_disable: Option<unsafe extern "C" fn(
        part: u8,
        inst: u16,
        clkctrl_offs: u16,
    )>,
    pub xlate_clkctrl: Option<unsafe extern "C" fn(
        part: u8,
        inst: u16,
        clkctrl_offs: u16,
    ) -> u32>,
}

extern "C" {
    pub fn cm_split_idlest_reg(
        idlest_reg: *mut clk_omap_reg,
        prcm_inst: *mut i16,
        idlest_reg_id: *mut u8,
    ) -> i32;
    pub fn omap_cm_wait_module_ready(
        part: u8,
        prcm_mod: i16,
        idlest_reg: u16,
        idlest_shift: u8,
    ) -> i32;
    pub fn omap_cm_wait_module_idle(
        part: u8,
        prcm_mod: i16,
        idlest_reg: u16,
        idlest_shift: u8,
    ) -> i32;
    pub fn omap_cm_module_enable(mode: u8, part: u8, inst: u16, clkctrl_offs: u16) -> i32;
    pub fn omap_cm_module_disable(part: u8, inst: u16, clkctrl_offs: u16) -> i32;
    pub fn omap_cm_xlate_clkctrl(part: u8, inst: u16, clkctrl_offs: u16) -> u32;
    pub fn cm_register(cld: *const cm_ll_data) -> i32;
    pub fn cm_unregister(cld: *const cm_ll_data) -> i32;
    pub fn omap_cm_init() -> i32;
    pub fn omap2_cm_base_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
