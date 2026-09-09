// SPDX-License-Identifier: GPL-2.0-only
/*
 * Power Management driver for Marvell Kirkwood SoCs
 *
 * Copyright (C) 2013 Ezequiel Garcia <ezequiel@free-electrons.com>
 * Copyright (C) 2010 Simon Guinot <sguinot@lacie.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/suspend.h, linux/io.h, kirkwood.h, kirkwood-pm.h

use core::ffi::c_void;

extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn cpu_do_idle();
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn suspend_set_ops(ops: *const platform_suspend_ops);
}

// Supplied by the surrounding kernel translation unit.
type suspend_state_t = i32;
const PM_SUSPEND_STANDBY: suspend_state_t = 1;
const EINVAL: i32 = 22;

// Supplied by kirkwood.h / kirkwood-pm.h.
extern "C" {
    static DDR_OPERATION_BASE: usize;
    static MEMORY_PM_CTRL_PHYS: usize;
}

#[repr(C)]
struct platform_suspend_ops {
    enter: Option<unsafe extern "C" fn(state: suspend_state_t) -> i32>,
    valid: Option<unsafe extern "C" fn(state: suspend_state_t) -> bool>,
}

static mut ddr_operation_base: *mut c_void = core::ptr::null_mut();
static mut memory_pm_ctrl: *mut c_void = core::ptr::null_mut();

unsafe fn kirkwood_low_power() {
    let mem_pm_ctrl: u32;

    mem_pm_ctrl = readl(memory_pm_ctrl);

    /* Set peripherals to low-power mode */
    writel_relaxed(!0u32, memory_pm_ctrl);

    /* Set DDR in self-refresh */
    writel_relaxed(0x7, ddr_operation_base);

    /*
     * Set CPU in wait-for-interrupt state.
     * This disables the CPU core clocks,
     * the array clocks, and also the L2 controller.
     */
    cpu_do_idle();

    writel_relaxed(mem_pm_ctrl, memory_pm_ctrl);
}

unsafe extern "C" fn kirkwood_suspend_enter(state: suspend_state_t) -> i32 {
    match state {
        PM_SUSPEND_STANDBY => {
            kirkwood_low_power();
        }
        _ => {
            return -EINVAL;
        }
    }
    0
}

unsafe extern "C" fn kirkwood_pm_valid_standby(state: suspend_state_t) -> bool {
    state == PM_SUSPEND_STANDBY
}

static kirkwood_suspend_ops: platform_suspend_ops = platform_suspend_ops {
    enter: Some(kirkwood_suspend_enter),
    valid: Some(kirkwood_pm_valid_standby),
};

// __init
#[no_mangle]
pub unsafe extern "C" fn kirkwood_pm_init() {
    ddr_operation_base = ioremap(DDR_OPERATION_BASE, 4);
    memory_pm_ctrl = ioremap(MEMORY_PM_CTRL_PHYS, 4);

    suspend_set_ops(&kirkwood_suspend_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
