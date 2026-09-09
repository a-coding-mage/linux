/*
 * PXA250/210 Power Management Routines
 *
 * Original code for the SA11x0:
 * Copyright (c) 2001 Cliff Brake <cbrake@accelent.com>
 *
 * Modified for the PXA250 by Nicolas Pitre:
 * Copyright (c) 2002 Monta Vista Software, Inc.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License.
 */

// Linux kernel headers and "pm.h" provide the declarations referenced below.

use core::ffi::c_void;

type CInt = i32;
type CUInt = u32;
type CULong = usize;
type SuspendStateT = CInt;

const PM_SUSPEND_STANDBY: SuspendStateT = 1;
const EINVAL: CInt = 22;
const ENOMEM: CInt = 12;

#[repr(C)]
pub struct PxaCpuPmFns {
    pub save: Option<unsafe extern "C" fn(*mut CULong)>,
    pub restore: Option<unsafe extern "C" fn(*mut CULong)>,
    pub enter: unsafe extern "C" fn(SuspendStateT),
    pub valid: unsafe extern "C" fn(SuspendStateT) -> CInt,
    pub prepare: Option<unsafe extern "C" fn() -> CInt>,
    pub finish: Option<unsafe extern "C" fn()>,
    pub save_count: usize,
}

extern "C" {
    pub static mut pxa_cpu_pm_fns: *mut PxaCpuPmFns;
    fn kmalloc_array(n: usize, size: usize, flags: CUInt) -> *mut c_void;
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
    fn printk(level: CInt, format: *const u8, ...);
    fn pr_debug(format: *const u8, ...);
}

#[repr(C)]
pub struct PlatformSuspendOps {
    pub valid: Option<unsafe extern "C" fn(SuspendStateT) -> CInt>,
    pub enter: Option<unsafe extern "C" fn(SuspendStateT) -> CInt>,
    pub prepare: Option<unsafe extern "C" fn() -> CInt>,
    pub finish: Option<unsafe extern "C" fn()>,
}

static mut sleep_save: *mut CULong = core::ptr::null_mut();

pub unsafe extern "C" fn pxa_pm_enter(state: SuspendStateT) -> CInt {
    let mut sleep_save_checksum: CULong = 0;
    let mut checksum: CULong = 0;
    let mut i: usize;

    // CONFIG_IWMMXT is a build-time condition from the original source.
    // If enabled, force any iWMMXt context to ram.

    /* skip registers saving for standby */
    if state != PM_SUSPEND_STANDBY && !(*pxa_cpu_pm_fns).save.is_none() {
        ((*pxa_cpu_pm_fns).save.unwrap())(sleep_save);
        /* before sleeping, calculate and save a checksum */
        i = 0;
        while i < (*pxa_cpu_pm_fns).save_count - 1 {
            sleep_save_checksum = sleep_save_checksum.wrapping_add(*sleep_save.add(i));
            i += 1;
        }
    }

    /* *** go zzz *** */
    ((*pxa_cpu_pm_fns).enter)(state);

    if state != PM_SUSPEND_STANDBY && !(*pxa_cpu_pm_fns).restore.is_none() {
        /* after sleeping, validate the checksum */
        i = 0;
        while i < (*pxa_cpu_pm_fns).save_count - 1 {
            checksum = checksum.wrapping_add(*sleep_save.add(i));
            i += 1;
        }

        /* if invalid, display message and wait for a hardware reset */
        if checksum != sleep_save_checksum {
            loop {
                ((*pxa_cpu_pm_fns).enter)(state);
            }
        }
        ((*pxa_cpu_pm_fns).restore.unwrap())(sleep_save);
    }

    pr_debug(b"*** made it back from resume\n\0".as_ptr());

    0
}

pub unsafe extern "C" fn pxa_pm_valid(state: SuspendStateT) -> CInt {
    if !pxa_cpu_pm_fns.is_null() {
        return ((*pxa_cpu_pm_fns).valid)(state);
    }

    -EINVAL
}

pub unsafe extern "C" fn pxa_pm_prepare() -> CInt {
    let mut ret: CInt = 0;

    if !pxa_cpu_pm_fns.is_null() && !(*pxa_cpu_pm_fns).prepare.is_none() {
        ret = ((*pxa_cpu_pm_fns).prepare.unwrap())();
    }

    ret
}

pub unsafe extern "C" fn pxa_pm_finish() {
    if !pxa_cpu_pm_fns.is_null() && !(*pxa_cpu_pm_fns).finish.is_none() {
        ((*pxa_cpu_pm_fns).finish.unwrap())();
    }
}

static pxa_pm_ops: PlatformSuspendOps = PlatformSuspendOps {
    valid: Some(pxa_pm_valid),
    enter: Some(pxa_pm_enter),
    prepare: Some(pxa_pm_prepare),
    finish: Some(pxa_pm_finish),
};

pub unsafe extern "C" fn pxa_pm_init() -> CInt {
    if pxa_cpu_pm_fns.is_null() {
        printk(0, b"no valid pxa_cpu_pm_fns defined\n\0".as_ptr());
        return -EINVAL;
    }

    sleep_save = kmalloc_array((*pxa_cpu_pm_fns).save_count,
                               core::mem::size_of::<CULong>(),
                               0) as *mut CULong;
    if sleep_save.is_null() {
        return -ENOMEM;
    }

    suspend_set_ops(&pxa_pm_ops);
    0
}

// EXPORT_SYMBOL_GPL(pxa_pm_enter);
// device_initcall(pxa_pm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
