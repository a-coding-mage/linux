// SPDX-License-Identifier: GPL-2.0-only
/*
 * Abstract code for CPUFreq governor tunable sysfs attributes.
 *
 * Copyright (C) 2016, Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

// Dependency declarations and definitions are supplied by cpufreq_governor.h.

#[inline]
unsafe fn to_gov_attr(attr: *mut attribute) -> *mut governor_attr {
    container_of!(attr, governor_attr, attr)
}

unsafe fn governor_show(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let gattr: *mut governor_attr = to_gov_attr(attr);

    ((*gattr).show)(to_gov_attr_set(kobj), buf)
}

unsafe fn governor_store(
    kobj: *mut kobject,
    attr: *mut attribute,
    buf: *const core::ffi::c_char,
    count: usize,
) -> isize {
    let attr_set: *mut gov_attr_set = to_gov_attr_set(kobj);
    let gattr: *mut governor_attr = to_gov_attr(attr);
    let ret: i32;

    mutex_lock(&mut (*attr_set).update_lock);
    ret = if (*attr_set).usage_count != 0 {
        ((*gattr).store)(attr_set, buf, count)
    } else {
        -EBUSY
    };
    mutex_unlock(&mut (*attr_set).update_lock);
    ret as isize
}

const GOVERNOR_SYSFS_OPS: sysfs_ops = sysfs_ops {
    show: Some(governor_show),
    store: Some(governor_store),
};

pub static governor_sysfs_ops: sysfs_ops = GOVERNOR_SYSFS_OPS;

pub unsafe fn gov_attr_set_init(
    attr_set: *mut gov_attr_set,
    list_node: *mut list_head,
) {
    INIT_LIST_HEAD(&mut (*attr_set).policy_list);
    mutex_init(&mut (*attr_set).update_lock);
    (*attr_set).usage_count = 1;
    list_add(list_node, &mut (*attr_set).policy_list);
}

pub unsafe fn gov_attr_set_get(
    attr_set: *mut gov_attr_set,
    list_node: *mut list_head,
) {
    mutex_lock(&mut (*attr_set).update_lock);
    (*attr_set).usage_count += 1;
    list_add(list_node, &mut (*attr_set).policy_list);
    mutex_unlock(&mut (*attr_set).update_lock);
}

pub unsafe fn gov_attr_set_put(
    attr_set: *mut gov_attr_set,
    list_node: *mut list_head,
) -> u32 {
    let count: u32;

    mutex_lock(&mut (*attr_set).update_lock);
    list_del(list_node);
    (*attr_set).usage_count -= 1;
    count = (*attr_set).usage_count;
    mutex_unlock(&mut (*attr_set).update_lock);
    if count != 0 {
        return count;
    }

    mutex_destroy(&mut (*attr_set).update_lock);
    kobject_put(&mut (*attr_set).kobj);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
