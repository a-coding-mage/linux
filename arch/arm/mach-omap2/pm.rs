// SPDX-License-Identifier: GPL-2.0-only
/*
 * pm.c - Common OMAP2+ power management-related code
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Copyright (C) 2010 Nokia Corporation
 */

// Linux and OMAP dependencies supplied by the surrounding translation.

use core::ffi::c_void;

pub type u32 = core::primitive::u32;
pub type suspend_state_t = i32;

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_suspend_ops {
    pub begin: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    pub end: Option<unsafe extern "C" fn()>,
    pub enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    pub wake: Option<unsafe extern "C" fn()>,
    pub valid: Option<unsafe extern "C" fn(suspend_state_t) -> bool>,
}

pub const ULONG_MAX: u32 = u32::MAX;
pub const ENOENT: i32 = 2;
pub const EINVAL: i32 = 22;
pub const PM_SUSPEND_MEM: suspend_state_t = 3;

extern "C" {
    fn clkdm_allow_idle(clkdm: *mut clockdomain);
    fn cpu_idle_poll_ctrl(enable: bool);
    fn soc_is_omap34xx() -> bool;
    fn omap_prcm_irq_prepare();
    fn omap_prcm_irq_complete();
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn suspend_valid_only_mem(state: suspend_state_t) -> bool;
    fn omap3_twl_init();
    fn omap4_twl_init();
    fn omap4_cpcap_init();
    fn omap_voltage_late_init();
    fn omap_devinit_smartreflex();
    fn omap2_clk_enable_autoidle_all();
    fn pr_warn(fmt: *const u8, ...);
    fn omap_late_initcall(init: unsafe extern "C" fn() -> i32);
}

pub static mut enable_off_mode: u32 = 0;

// CONFIG_SUSPEND
/*
 * omap_pm_suspend: points to a function that does the SoC-specific
 * suspend work
 */
#[cfg(any())]
static mut omap_pm_suspend: Option<unsafe extern "C" fn() -> i32> = None;

// CONFIG_PM
/**
 * struct omap2_oscillator - Describe the board main oscillator latencies
 * @startup_time: oscillator startup latency
 * @shutdown_time: oscillator shutdown latency
 */
#[repr(C)]
pub struct omap2_oscillator {
    pub startup_time: u32,
    pub shutdown_time: u32,
}

#[cfg(any())]
static mut oscillator: omap2_oscillator = omap2_oscillator {
    startup_time: ULONG_MAX,
    shutdown_time: ULONG_MAX,
};

#[cfg(any())]
pub unsafe extern "C" fn omap_pm_get_oscillator(tstart: *mut u32, tshut: *mut u32) {
    if tstart.is_null() || tshut.is_null() {
        return;
    }

    (*tstart) = oscillator.startup_time;
    (*tshut) = oscillator.shutdown_time;
}

pub unsafe extern "C" fn omap_pm_clkdms_setup(
    clkdm: *mut clockdomain,
    _unused: *mut c_void,
) -> i32 {
    clkdm_allow_idle(clkdm);
    0
}

// CONFIG_SUSPEND
#[cfg(any())]
unsafe extern "C" fn omap_pm_enter(_suspend_state: suspend_state_t) -> i32 {
    let mut ret: i32 = 0;

    if omap_pm_suspend.is_none() {
        return -ENOENT; /* XXX doublecheck */
    }

    match _suspend_state {
        PM_SUSPEND_MEM => {
            ret = omap_pm_suspend.unwrap()();
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

#[cfg(any())]
unsafe extern "C" fn omap_pm_begin(_state: suspend_state_t) -> i32 {
    cpu_idle_poll_ctrl(true);
    if soc_is_omap34xx() {
        omap_prcm_irq_prepare();
    }
    0
}

#[cfg(any())]
unsafe extern "C" fn omap_pm_end() {
    cpu_idle_poll_ctrl(false);
}

#[cfg(any())]
unsafe extern "C" fn omap_pm_wake() {
    if soc_is_omap34xx() {
        omap_prcm_irq_complete();
    }
}

#[cfg(any())]
static omap_pm_ops: platform_suspend_ops = platform_suspend_ops {
    begin: Some(omap_pm_begin),
    end: Some(omap_pm_end),
    enter: Some(omap_pm_enter),
    wake: Some(omap_pm_wake),
    valid: Some(suspend_valid_only_mem),
};

/**
 * omap_common_suspend_init - Set common suspend routines for OMAP SoCs
 * @pm_suspend: function pointer to SoC specific suspend function
 */
#[cfg(any())]
pub unsafe extern "C" fn omap_common_suspend_init(pm_suspend: *mut c_void) {
    omap_pm_suspend = core::mem::transmute(pm_suspend);
    suspend_set_ops(&omap_pm_ops);
}

pub unsafe extern "C" fn omap_pm_nop_init() -> i32 {
    0
}

pub static mut omap_pm_soc_init: Option<unsafe extern "C" fn() -> i32> = None;

unsafe extern "C" fn omap2_common_pm_late_init() -> i32 {
    let error: i32;

    if omap_pm_soc_init.is_none() {
        return 0;
    }

    /* Init the voltage layer */
    omap3_twl_init();
    omap4_twl_init();
    omap4_cpcap_init();
    omap_voltage_late_init();

    /* Smartreflex device init */
    omap_devinit_smartreflex();

    error = omap_pm_soc_init.unwrap()();
    if error != 0 {
        // pr_warn("%s: pm soc init failed: %i\n", __func__, error);
        pr_warn(core::ptr::null(), error);
    }

    omap2_clk_enable_autoidle_all();

    0
}

// omap_late_initcall(omap2_common_pm_late_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
