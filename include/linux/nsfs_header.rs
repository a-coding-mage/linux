/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2025 Christian Brauner <brauner@kernel.org> */

// Declarations supplied by linux/ns_common.h, linux/cred.h, and
// linux/pid_namespace.h are intentionally left as external dependencies.

use core::ffi::c_void;

pub enum path {}
pub enum task_struct {}
pub enum proc_ns_operations {}
pub enum ns_common {}
pub enum nsproxy {}
pub enum cgroup_namespace {}
pub enum ipc_namespace {}
pub enum net {}
pub enum pid_namespace {}
pub enum mnt_namespace {}
pub enum time_namespace {}
pub enum user_namespace {}
pub enum uts_namespace {}

pub type ns_get_path_helper_t = unsafe extern "C" fn(*mut c_void) -> *mut ns_common;

unsafe extern "C" {
    pub fn ns_get_path(
        path: *mut path,
        task: *mut task_struct,
        ns_ops: *const proc_ns_operations,
    ) -> core::ffi::c_int;

    pub fn ns_get_path_cb(
        path: *mut path,
        ns_get_cb: ns_get_path_helper_t,
        private_data: *mut c_void,
    ) -> core::ffi::c_int;

    pub fn ns_match(
        ns: *const ns_common,
        dev: dev_t,
        ino: ino_t,
    ) -> bool;

    pub fn ns_get_name(
        buf: *mut core::ffi::c_char,
        size: usize,
        task: *mut task_struct,
        ns_ops: *const proc_ns_operations,
    ) -> core::ffi::c_int;

    pub fn nsfs_init();

    pub fn nsproxy_ns_active_get(ns: *mut nsproxy);
    pub fn nsproxy_ns_active_put(ns: *mut nsproxy);
}

// C dev_t and ino_t types are provided by the platform headers.
pub type dev_t = u64;
pub type ino_t = u64;

// The C _Generic selection maps each namespace pointer type to the
// corresponding namespace in current->nsproxy (or the active pid/current user
// namespace). Rust has no direct equivalent of C's type-generic selection;
// callers must select the corresponding external expression explicitly.
#[macro_export]
macro_rules! current_in_namespace {
    ($current_namespace:expr, $namespace:expr) => {
        $current_namespace == $namespace
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
