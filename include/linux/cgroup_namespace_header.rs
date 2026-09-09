/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

// Supplied by the corresponding kernel headers/dependencies.
#[repr(C)]
pub struct ns_common {
    _private: [u8; 0],
}

pub struct user_namespace {
    _private: [u8; 0],
}

pub struct ucounts {
    _private: [u8; 0],
}

pub struct css_set {
    _private: [u8; 0],
}

pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_namespace {
    pub ns: ns_common,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub root_cset: *mut css_set,
}

extern "C" {
    pub static mut init_cgroup_ns: cgroup_namespace;
}

// CONFIG_CGROUPS conditional from the original header.
#[cfg(feature = "CONFIG_CGROUPS")]
pub unsafe fn to_cg_ns(ns: *mut ns_common) -> *mut cgroup_namespace {
    ns as *mut cgroup_namespace
}

#[cfg(feature = "CONFIG_CGROUPS")]
extern "C" {
    pub fn free_cgroup_ns(ns: *mut cgroup_namespace);
    pub fn copy_cgroup_ns(
        flags: u64,
        user_ns: *mut user_namespace,
        old_ns: *mut cgroup_namespace,
    ) -> *mut cgroup_namespace;
    pub fn cgroup_path_ns(
        cgrp: *mut cgroup,
        buf: *mut c_char,
        buflen: usize,
        ns: *mut cgroup_namespace,
    ) -> i32;
    pub fn ns_ref_inc(ns: *mut cgroup_namespace);
    pub fn ns_ref_put(ns: *mut cgroup_namespace) -> bool;
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[inline]
pub unsafe fn get_cgroup_ns(ns: *mut cgroup_namespace) {
    ns_ref_inc(ns);
}

#[cfg(feature = "CONFIG_CGROUPS")]
#[inline]
pub unsafe fn put_cgroup_ns(ns: *mut cgroup_namespace) {
    if ns_ref_put(ns) {
        free_cgroup_ns(ns);
    }
}

// !CONFIG_CGROUPS branch from the original header.
#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline]
pub unsafe fn free_cgroup_ns(_ns: *mut cgroup_namespace) {}

#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline]
pub unsafe fn copy_cgroup_ns(
    _flags: u64,
    _user_ns: *mut user_namespace,
    old_ns: *mut cgroup_namespace,
) -> *mut cgroup_namespace {
    old_ns
}

#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline]
pub unsafe fn get_cgroup_ns(_ns: *mut cgroup_namespace) {}

#[cfg(not(feature = "CONFIG_CGROUPS"))]
#[inline]
pub unsafe fn put_cgroup_ns(_ns: *mut cgroup_namespace) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
