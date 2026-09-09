// SPDX-License-Identifier: GPL-2.0-only
/*
 * Runtime PM support code for DaVinci
 *
 * Author: Kevin Hilman
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 */

use core::ffi::c_char;

/* Types and functions supplied by the Linux PM and platform-device code. */
#[repr(C)]
pub struct DevPmDomainOps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DevPmDomain {
    pub ops: DevPmDomainOps,
}

#[repr(C)]
pub struct PmClkNotifierBlock {
    pub pm_domain: *mut DevPmDomain,
    pub con_ids: [*const c_char; 4],
}

#[repr(C)]
pub struct BusType {
    _private: [u8; 0],
}

extern "C" {
    pub static platform_bus_type: BusType;
    pub fn of_have_populated_dt() -> i32;
    pub fn pm_clk_add_notifier(
        bus: *const BusType,
        clknb: *mut PmClkNotifierBlock,
    ) -> i32;
}

/* USE_PM_CLK_RUNTIME_OPS and USE_PLATFORM_PM_SLEEP_OPS expand to the
 * corresponding runtime-PM and platform-PM sleep operation members here. */
static mut davinci_pm_domain: DevPmDomain = DevPmDomain {
    ops: DevPmDomainOps { _private: [] },
};

static mut platform_bus_notifier: PmClkNotifierBlock = PmClkNotifierBlock {
    pm_domain: core::ptr::addr_of_mut!(davinci_pm_domain),
    con_ids: [
        b"fck\0".as_ptr() as *const c_char,
        b"master\0".as_ptr() as *const c_char,
        b"slave\0".as_ptr() as *const c_char,
        core::ptr::null(),
    ],
};

unsafe extern "C" fn davinci_pm_runtime_init() -> i32 {
    if of_have_populated_dt() != 0 {
        return 0;
    }

    /* Use pm_clk as fallback if we're not using genpd. */
    pm_clk_add_notifier(
        core::ptr::addr_of!(platform_bus_type),
        core::ptr::addr_of_mut!(platform_bus_notifier),
    );

    0
}

/* core_initcall(davinci_pm_runtime_init); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
