// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Maxime Coquelin 2015
 * Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
 */

// Translated from armv7m_systick.c. Kernel-provided types, functions, and
// registration macros are intentionally left as external dependencies.

const SYST_CSR: usize = 0x00;
const SYST_RVR: usize = 0x04;
const SYST_CVR: usize = 0x08;
const SYST_CALIB: usize = 0x0c;

const SYST_CSR_ENABLE: u32 = 1u32 << 0;

const SYSTICK_LOAD_RELOAD_MASK: u32 = 0x00ff_ffff;

extern "C" {
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const core::ffi::c_char,
        out_value: *mut u32,
    ) -> i32;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn writel_relaxed(value: u32, address: *mut core::ffi::c_void);
    fn clocksource_mmio_init(
        address: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        clock_rate: u32,
        rating: u32,
        bits: u32,
        read: unsafe extern "C" fn(*mut core::ffi::c_void) -> u64,
    ) -> i32;
    fn clocksource_mmio_readl_down(address: *mut core::ffi::c_void) -> u64;
    fn iounmap(address: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

unsafe fn system_timer_of_register(np: *mut device_node) -> i32 {
    let mut clk: *mut clk = core::ptr::null_mut();
    let base: *mut core::ffi::c_void;
    let mut rate: u32 = 0;
    let mut ret: i32;

    base = of_iomap(np, 0);
    if base.is_null() {
        // pr_warn("system-timer: invalid base address\n");
        return -6; // -ENXIO
    }

    ret = of_property_read_u32(
        np,
        b"clock-frequency\0".as_ptr() as *const core::ffi::c_char,
        &mut rate,
    );
    if ret != 0 {
        clk = of_clk_get(np, 0);
        // IS_ERR(clk), PTR_ERR(clk), and kernel logging are external kernel
        // facilities; preserve the original error-path ordering here.
        if (clk as usize) >= (usize::MAX - 4095) {
            ret = clk as isize as i32;
            iounmap(base);
            // pr_warn("ARM System timer register failed (%d)\n", ret);
            return ret;
        }

        ret = clk_prepare_enable(clk);
        if ret != 0 {
            clk_put(clk);
            iounmap(base);
            // pr_warn("ARM System timer register failed (%d)\n", ret);
            return ret;
        }

        rate = clk_get_rate(clk);
        if rate == 0 {
            ret = -22; // -EINVAL
            clk_disable_unprepare(clk);
            clk_put(clk);
            iounmap(base);
            // pr_warn("ARM System timer register failed (%d)\n", ret);
            return ret;
        }
    }

    writel_relaxed(SYSTICK_LOAD_RELOAD_MASK, base.add(SYST_RVR));
    writel_relaxed(SYST_CSR_ENABLE, base.add(SYST_CSR));

    ret = clocksource_mmio_init(
        base.add(SYST_CVR),
        b"arm_system_timer\0".as_ptr() as *const core::ffi::c_char,
        rate,
        200,
        24,
        clocksource_mmio_readl_down,
    );
    if ret != 0 {
        // pr_err("failed to init clocksource (%d)\n", ret);
        if !clk.is_null() {
            clk_disable_unprepare(clk);
            clk_put(clk);
        }
        iounmap(base);
        // pr_warn("ARM System timer register failed (%d)\n", ret);
        return ret;
    }

    // pr_info("ARM System timer initialized as clocksource\n");
    0
}

// TIMER_OF_DECLARE(arm_systick, "arm,armv7m-systick", system_timer_of_register);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
