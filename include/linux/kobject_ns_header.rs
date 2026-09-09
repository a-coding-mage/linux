// SPDX-License-Identifier: GPL-2.0
/* Kernel object name space definitions
 *
 * Copyright (c) 2002-2003 Patrick Mochel
 * Copyright (c) 2002-2003 Open Source Development Labs
 * Copyright (c) 2006-2008 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (c) 2006-2008 Novell Inc.
 *
 * Split from kobject.h by David Howells (dhowells@redhat.com)
 *
 * Please read Documentation/core-api/kobject.rst before using the kobject
 * interface, ESPECIALLY the parts about reference counts and object
 * destructors.
 */

// C forward declarations.
pub struct ns_common;
pub struct sock;
pub struct kobject;

/*
 * Namespace types which are used to tag kobjects and sysfs entries.
 * Network namespace will likely be the first.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum kobj_ns_type {
    KOBJ_NS_TYPE_NONE = 0,
    KOBJ_NS_TYPE_NET,
    KOBJ_NS_TYPES,
}

/*
 * Callbacks so sysfs can determine namespaces
 *   @grab_current_ns: return a new reference to calling task's namespace
 *   @netlink_ns: return namespace to which a sock belongs (right?)
 *   @initial_ns: return the initial namespace (i.e. init_net_ns)
 *   @drop_ns: drops a reference to namespace
 */
#[repr(C)]
pub struct kobj_ns_type_operations {
    pub type_: kobj_ns_type,
    pub current_may_mount: Option<unsafe extern "C" fn() -> bool>,
    pub grab_current_ns: Option<unsafe extern "C" fn() -> *mut ns_common>,
    pub netlink_ns:
        Option<unsafe extern "C" fn(sk: *mut sock) -> *const ns_common>,
    pub initial_ns: Option<unsafe extern "C" fn() -> *const ns_common>,
    pub drop_ns: Option<unsafe extern "C" fn(ns: *mut ns_common)>,
}

unsafe extern "C" {
    pub fn kobj_ns_type_register(ops: *const kobj_ns_type_operations) -> core::ffi::c_int;
    pub fn kobj_ns_type_registered(type_: kobj_ns_type) -> core::ffi::c_int;
    pub fn kobj_child_ns_ops(
        parent: *const kobject,
    ) -> *const kobj_ns_type_operations;
    pub fn kobj_ns_ops(kobj: *const kobject) -> *const kobj_ns_type_operations;

    pub fn kobj_ns_current_may_mount(type_: kobj_ns_type) -> bool;
    pub fn kobj_ns_grab_current(type_: kobj_ns_type) -> *mut ns_common;
    pub fn kobj_ns_drop(type_: kobj_ns_type, ns: *mut ns_common);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
