/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2013 Samsung Electronics Co., Ltd.
 *	Tomasz Figa <t.figa@samsung.com>
 * Copyright (c) 2004 Simtec Electronics
 *	http://armlinux.simtec.co.uk/
 *	Written by Ben Dooks, <ben@simtec.com>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/irq.h and linux/soc/samsung/s3c-pm.h

/* sleep save info */

/**
 * struct sleep_save - save information for shared peripherals.
 * @reg: Pointer to the register to save.
 * @val: Holder for the value saved from reg.
 *
 * This describes a list of registers which is used by the pm core and
 * other subsystem to save and restore register values over suspend.
 */
#[repr(C)]
pub struct sleep_save {
    pub reg: *mut core::ffi::c_void,
    pub val: usize,
}

#[macro_export]
macro_rules! SAVE_ITEM {
    ($x:expr) => {
        $crate::sleep_save {
            reg: $x as *mut core::ffi::c_void,
            val: 0,
        }
    };
}

/* helper functions to save/restore lists of registers. */

extern "C" {
    pub fn s3c_pm_do_save(ptr: *mut sleep_save, count: core::ffi::c_int);
    pub fn s3c_pm_do_restore(ptr: *const sleep_save, count: core::ffi::c_int);
    pub fn s3c_pm_do_restore_core(ptr: *const sleep_save, count: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
