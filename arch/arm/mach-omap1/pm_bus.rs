// SPDX-License-Identifier: GPL-2.0-only
/*
 * Runtime PM support code for OMAP1
 *
 * Author: Kevin Hilman, Deep Root Systems, LLC
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/kernel.h, linux/io.h, linux/pm_runtime.h,
// linux/pm_clock.h, linux/platform_device.h, linux/mutex.h, linux/clk.h,
// linux/err.h, and "soc.h".

#[repr(C)]
pub struct dev_pm_ops {
    pub use_pm_clk_runtime_ops: usize,
    pub use_platform_pm_sleep_ops: usize,
}

#[repr(C)]
pub struct dev_pm_domain {
    pub ops: dev_pm_ops,
}

#[repr(C)]
pub struct pm_clk_notifier_block {
    pub pm_domain: *mut dev_pm_domain,
    pub con_ids: [*const core::ffi::c_char; 3],
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cpu_class_is_omap1() -> bool;
    fn pm_clk_add_notifier(
        bus: *mut bus_type,
        notifier: *mut pm_clk_notifier_block,
    );
    static mut platform_bus_type: bus_type;
}

const ENODEV: i32 = 19;

static mut default_pm_domain: dev_pm_domain = dev_pm_domain {
    ops: dev_pm_ops {
        // USE_PM_CLK_RUNTIME_OPS
        use_pm_clk_runtime_ops: 0,
        // USE_PLATFORM_PM_SLEEP_OPS
        use_platform_pm_sleep_ops: 0,
    },
};

static mut platform_bus_notifier: pm_clk_notifier_block = pm_clk_notifier_block {
    pm_domain: core::ptr::addr_of_mut!(default_pm_domain),
    con_ids: [
        b"ick\0".as_ptr() as *const core::ffi::c_char,
        b"fck\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null(),
    ],
};

// __init
unsafe fn omap1_pm_runtime_init() -> i32 {
    if !cpu_class_is_omap1() {
        return -ENODEV;
    }

    pm_clk_add_notifier(
        core::ptr::addr_of_mut!(platform_bus_type),
        core::ptr::addr_of_mut!(platform_bus_notifier),
    );

    0
}

// core_initcall(omap1_pm_runtime_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
