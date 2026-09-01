// SPDX-License-Identifier: GPL-2.0

// C dependencies from:
// #include <linux/compiler.h>
// #include <linux/refcount.h>
// #include <linux/rbtree.h>
// #include "util/env.h"

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct cgroup {
    pub node: rb_node,
    pub id: u64,
    pub name: *mut c_char,
    pub fd: c_int,
    pub refcnt: refcount_t,
}

unsafe extern "C" {
    // number of explicit cgroups defined
    pub static mut nr_cgroups: c_int;
    pub static mut cgrp_event_expanded: bool;

    pub fn cgroup__get(cgroup: *mut cgroup) -> *mut cgroup;
    pub fn cgroup__put(cgroup: *mut cgroup);

    pub fn cgroup__new(name: *const c_char, do_open: bool) -> *mut cgroup;
    pub fn evlist__findnew_cgroup(evlist: *mut evlist, name: *const c_char) -> *mut cgroup;
    pub fn evlist__expand_cgroup(
        evlist: *mut evlist,
        cgroups: *const c_char,
        open_cgroup: bool,
    ) -> c_int;

    pub fn evlist__set_default_cgroup(evlist: *mut evlist, cgroup: *mut cgroup);

    pub fn parse_cgroups(opt: *const option, str_: *const c_char, unset: c_int) -> c_int;

    pub fn cgroup__findnew(
        env: *mut perf_env,
        id: u64,
        path: *const c_char,
    ) -> *mut cgroup;
    pub fn cgroup__find(env: *mut perf_env, id: u64) -> *mut cgroup;
    pub fn __cgroup__find(root: *mut rb_root, id: u64) -> *mut cgroup;

    pub fn perf_env__purge_cgroups(env: *mut perf_env);
}

// C conditional: #ifdef HAVE_FILE_HANDLE
#[cfg(HAVE_FILE_HANDLE)]
unsafe extern "C" {
    pub fn read_cgroup_id(cgrp: *mut cgroup) -> c_int;
}

// C conditional: #else for missing HAVE_FILE_HANDLE.
#[cfg(not(HAVE_FILE_HANDLE))]
#[inline]
pub unsafe fn read_cgroup_id(_cgrp: *mut cgroup) -> c_int {
    -1
}

unsafe extern "C" {
    // read all cgroups in the system and save them in the rbtree
    pub fn read_all_cgroups(root: *mut rb_root);

    pub fn cgroup_is_v2(subsys: *const c_char) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
