// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//	Tomasz Figa <t.figa@samsung.com>
// Copyright (C) 2008 Openmoko, Inc.
// Copyright (C) 2004-2008 Simtec Electronics
//	Ben Dooks <ben@simtec.co.uk>
//	http://armlinux.simtec.co.uk/
//
// Samsung common power management helper functions.

// Dependencies supplied by the surrounding kernel translation:
// linux/io.h, linux/kernel.h, and pm-common.h.

use crate::sleep_save;

/* helper functions to save and restore register state */

/**
 * s3c_pm_do_save() - save a set of registers for restoration on resume.
 * @ptr: Pointer to an array of registers.
 * @count: Size of the ptr array.
 *
 * Run through the list of registers given, saving their contents in the
 * array for later restoration when we wakeup.
 */
pub unsafe fn s3c_pm_do_save(mut ptr: *mut sleep_save, mut count: i32) {
    while count > 0 {
        (*ptr).val = crate::readl_relaxed((*ptr).reg);
        crate::S3C_PMDBG!("saved {:p} value {:08lx}\n", (*ptr).reg, (*ptr).val);
        count -= 1;
        ptr = ptr.add(1);
    }
}

/**
 * s3c_pm_do_restore() - restore register values from the save list.
 * @ptr: Pointer to an array of registers.
 * @count: Size of the ptr array.
 *
 * Restore the register values saved from s3c_pm_do_save().
 *
 * Note, we do not use S3C_PMDBG() in here, as the system may not have
 * restore the UARTs state yet
*/
pub unsafe fn s3c_pm_do_restore(mut ptr: *const sleep_save, mut count: i32) {
    while count > 0 {
        crate::pr_debug!(
            "restore {:p} (restore {:08lx}, was {:08x})\n",
            (*ptr).reg,
            (*ptr).val,
            crate::readl_relaxed((*ptr).reg)
        );

        crate::writel_relaxed((*ptr).val, (*ptr).reg);
        count -= 1;
        ptr = ptr.add(1);
    }
}

/**
 * s3c_pm_do_restore_core() - early restore register values from save list.
 * @ptr: Pointer to an array of registers.
 * @count: Size of the ptr array.
 *
 * This is similar to s3c_pm_do_restore() except we try and minimise the
 * side effects of the function in case registers that hardware might need
 * to work has been restored.
 *
 * WARNING: Do not put any debug in here that may effect memory or use
 * peripherals, as things may be changing!
*/
pub unsafe fn s3c_pm_do_restore_core(mut ptr: *const sleep_save, mut count: i32) {
    while count > 0 {
        crate::writel_relaxed((*ptr).val, (*ptr).reg);
        count -= 1;
        ptr = ptr.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
