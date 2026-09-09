// SPDX-License-Identifier: GPL-2.0
/*
 * System bus type for containers.
 *
 * Copyright (C) 2013, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependency supplied by linux/container.h.
// Dependency supplied by base.h.

const CONTAINER_BUS_NAME: &str = "container";

unsafe fn trivial_online(_dev: *mut crate::device) -> i32 {
    0
}

unsafe fn container_offline(dev: *mut crate::device) -> i32 {
    let cdev: *mut crate::container_dev = crate::to_container_dev(dev);

    if !(*cdev).offline.is_null() {
        ((*cdev).offline)(cdev)
    } else {
        0
    }
}

pub static container_subsys: crate::bus_type = crate::bus_type {
    name: CONTAINER_BUS_NAME,
    dev_name: CONTAINER_BUS_NAME,
    online: Some(trivial_online),
    offline: Some(container_offline),
};

pub unsafe fn container_dev_init() {
    let ret: i32 = crate::subsys_system_register(&container_subsys, core::ptr::null());
    if ret != 0 {
        crate::pr_err!("{}() failed: {}\\n", "container_dev_init", ret);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
