// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014 Red Hat, Inc.
 * All Rights Reserved.
 */

// Translated from xfs_sysfs.h. Required kernel and XFS types/functions are
// supplied by other translation units.

extern "C" {
    pub static xfs_dbg_ktype: kobj_type; // debug
    pub static xfs_log_ktype: kobj_type; // xlog
    pub static xfs_stats_ktype: kobj_type; // stats

    pub fn init_completion(completion: *mut completion);
    pub fn kobject_init_and_add(
        kobject: *mut kobject,
        ktype: *const kobj_type,
        parent: *mut kobject,
        format: *const ::std::os::raw::c_char,
        ...,
    ) -> ::std::os::raw::c_int;
    pub fn kobject_put(kobject: *mut kobject);
    pub fn kobject_del(kobject: *mut kobject);
    pub fn wait_for_completion(completion: *mut completion);
    pub fn complete(completion: *mut completion);
}

#[inline]
pub unsafe fn to_kobj(kobject: *mut kobject) -> *mut xfs_kobj {
    // Equivalent to container_of(kobject, struct xfs_kobj, kobject).
    (kobject as *mut u8).sub(::core::mem::offset_of!(xfs_kobj, kobject))
        as *mut xfs_kobj
}

#[inline]
pub unsafe fn xfs_sysfs_release(kobject: *mut kobject) {
    let kobj: *mut xfs_kobj = to_kobj(kobject);
    complete(&mut (*kobj).complete);
}

#[inline]
pub unsafe fn xfs_sysfs_init(
    kobj: *mut xfs_kobj,
    ktype: *const kobj_type,
    parent_kobj: *mut xfs_kobj,
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let parent: *mut kobject = if !parent_kobj.is_null() {
        &mut (*parent_kobj).kobject
    } else {
        ::core::ptr::null_mut()
    };
    let mut err: ::std::os::raw::c_int;

    init_completion(&mut (*kobj).complete);
    err = kobject_init_and_add(
        &mut (*kobj).kobject,
        ktype,
        parent,
        b"%s\0".as_ptr() as *const ::std::os::raw::c_char,
        name,
    );
    if err != 0 {
        kobject_put(&mut (*kobj).kobject);
    }

    err
}

#[inline]
pub unsafe fn xfs_sysfs_del(kobj: *mut xfs_kobj) {
    kobject_del(&mut (*kobj).kobject);
    kobject_put(&mut (*kobj).kobject);
    wait_for_completion(&mut (*kobj).complete);
}

extern "C" {
    pub fn xfs_mount_sysfs_init(mp: *mut xfs_mount) -> ::std::os::raw::c_int;
    pub fn xfs_zoned_sysfs_init(mp: *mut xfs_mount) -> ::std::os::raw::c_int;
    pub fn xfs_zoned_sysfs_del(mp: *mut xfs_mount);
    pub fn xfs_mount_sysfs_del(mp: *mut xfs_mount);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
