/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Definitions for container bus type.
 *
 * Copyright (C) 2013, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

/* Dependency supplied by linux/device.h. */

/* drivers/base/power/container.c */
unsafe extern "C" {
    pub static container_subsys: bus_type;
}

#[repr(C)]
pub struct container_dev {
    pub dev: device,
    pub offline: Option<unsafe extern "C" fn(cdev: *mut container_dev) -> i32>,
}

pub unsafe fn to_container_dev(dev: *mut device) -> *mut container_dev {
    (dev as *mut u8).sub(std::mem::offset_of!(container_dev, dev)) as *mut container_dev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
