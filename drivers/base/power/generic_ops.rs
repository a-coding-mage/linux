// SPDX-License-Identifier: GPL-2.0
/*
 * drivers/base/power/generic_ops.c - Generic PM callbacks for subsystems
 *
 * Copyright (c) 2010 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
 */

// The declarations below are supplied by the Linux PM headers in the target.
#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub prepare: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend_late: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub freeze_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff_late: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub thaw_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume_early: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore_noirq: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore_early: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> i32>,
    pub complete: Option<unsafe extern "C" fn(*mut device)>,
}

#[repr(C)]
pub struct device_driver { pub pm: *const dev_pm_ops }
#[repr(C)]
pub struct device { pub driver: *mut device_driver }

unsafe fn call_pm_op(dev: *mut device, op: unsafe extern "C" fn(&dev_pm_ops) -> Option<unsafe extern "C" fn(*mut device) -> i32>) -> i32 {
    let driver = (*dev).driver;
    if !driver.is_null() && !(*driver).pm.is_null() {
        if let Some(callback) = op(&*(*driver).pm) { return callback(dev); }
    }
    0
}

#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn pm_generic_runtime_suspend(dev: *mut device) -> i32 {
    call_pm_op(dev, |pm| pm.runtime_suspend)
}

#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn pm_generic_runtime_resume(dev: *mut device) -> i32 {
    call_pm_op(dev, |pm| pm.runtime_resume)
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_prepare(dev: *mut device) -> i32 {
    let drv = (*dev).driver;
    let mut ret = 0;
    if !drv.is_null() && !(*drv).pm.is_null() {
        if let Some(prepare) = (*(*drv).pm).prepare { ret = prepare(dev); }
    }
    ret
}

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_suspend_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.suspend_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_suspend_late(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.suspend_late) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_suspend(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.suspend) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_freeze_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.freeze_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_freeze(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.freeze) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_poweroff_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.poweroff_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_poweroff_late(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.poweroff_late) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_poweroff(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.poweroff) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_thaw_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.thaw_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_thaw(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.thaw) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_resume_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.resume_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_resume_early(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.resume_early) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_resume(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.resume) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_restore_noirq(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.restore_noirq) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_restore_early(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.restore_early) }
#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_restore(dev: *mut device) -> i32 { call_pm_op(dev, |pm| pm.restore) }

#[cfg(CONFIG_PM_SLEEP)]
pub unsafe extern "C" fn pm_generic_complete(dev: *mut device) {
    let drv = (*dev).driver;
    if !drv.is_null() && !(*drv).pm.is_null() {
        if let Some(complete) = (*(*drv).pm).complete { complete(dev); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
