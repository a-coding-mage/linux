// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011 Calxeda, Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/cpu_pm.h, linux/init.h, linux/psci.h, linux/suspend.h,
// asm/suspend.h, uapi/linux/psci.h, and "core.h".

const HIGHBANK_SUSPEND_PARAM: u32 =
    ((0u32 << PSCI_0_2_POWER_STATE_ID_SHIFT)
        | (1u32 << PSCI_0_2_POWER_STATE_AFFL_SHIFT)
        | (PSCI_POWER_STATE_TYPE_POWER_DOWN << PSCI_0_2_POWER_STATE_TYPE_SHIFT));

unsafe extern "C" {
    static mut psci_ops: PsciOps;

    fn cpu_pm_enter();
    fn cpu_pm_exit();
    fn cpu_cluster_pm_enter();
    fn cpu_cluster_pm_exit();
    fn cpu_suspend(arg: u32, fn_ptr: Option<unsafe extern "C" fn(usize) -> i32>) -> i32;
    fn __pa(addr: unsafe extern "C" fn());
    fn cpu_resume();
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
    fn suspend_valid_only_mem(state: SuspendState) -> bool;
}

#[repr(C)]
struct PsciOps {
    cpu_suspend: Option<unsafe extern "C" fn(power_state: u32, entry: usize) -> i32>,
}

#[repr(C)]
struct PlatformSuspendOps {
    enter: Option<unsafe extern "C" fn(state: SuspendState) -> i32>,
    valid: Option<unsafe extern "C" fn(state: SuspendState) -> bool>,
}

// Translation of the kernel's suspend_state_t.
type SuspendState = u32;

unsafe extern "C" fn highbank_suspend_finish(_val: usize) -> i32 {
    match (*core::ptr::addr_of!(psci_ops)).cpu_suspend {
        Some(cpu_suspend_op) => cpu_suspend_op(HIGHBANK_SUSPEND_PARAM, __pa(cpu_resume) as usize),
        None => 0,
    }
}

unsafe extern "C" fn highbank_pm_enter(_state: SuspendState) -> i32 {
    cpu_pm_enter();
    cpu_cluster_pm_enter();

    cpu_suspend(0, Some(highbank_suspend_finish));

    cpu_cluster_pm_exit();
    cpu_pm_exit();

    0
}

static HIGHBANK_PM_OPS: PlatformSuspendOps = PlatformSuspendOps {
    enter: Some(highbank_pm_enter),
    valid: Some(suspend_valid_only_mem),
};

#[no_mangle]
pub unsafe extern "C" fn highbank_pm_init() {
    if (*core::ptr::addr_of!(psci_ops)).cpu_suspend.is_none() {
        return;
    }

    suspend_set_ops(&HIGHBANK_PM_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
