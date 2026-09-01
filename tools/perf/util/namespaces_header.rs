/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2017 Hari Bathini, IBM Corporation
 */

// Translated from perf/util/namespaces.h.
// C includes removed; the referenced C/kernel types are expected to be supplied
// by surrounding translated bindings.

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

// Original conditional declaration:
// #ifndef HAVE_SETNS_SUPPORT
unsafe extern "C" {
    pub fn setns(fd: c_int, nstype: c_int) -> c_int;
}
// #endif

#[repr(C)]
pub struct namespaces {
    pub list: list_head,
    pub end_time: u64,
    pub link_info: [perf_ns_link_info; 0],
}

unsafe extern "C" {
    pub fn namespaces__new(event: *mut perf_record_namespaces) -> *mut namespaces;
    pub fn namespaces__free(namespaces: *mut namespaces);
}

// DECLARE_RC_STRUCT(nsinfo)
#[repr(C)]
pub struct nsinfo {
    pub pid: pid_t,
    pub tgid: pid_t,
    pub nstgid: pid_t,
    pub need_setns: bool,
    pub in_pidns: bool,
    pub mntns_path: *mut c_char,
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct nscookie {
    pub oldns: c_int,
    pub newns: c_int,
    pub oldcwd: *mut c_char,
}

unsafe extern "C" {
    pub fn nsinfo__init(nsi: *mut nsinfo) -> c_int;
    pub fn nsinfo__new(pid: pid_t) -> *mut nsinfo;
    pub fn nsinfo__copy(nsi: *const nsinfo) -> *mut nsinfo;

    pub fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    pub fn nsinfo__put(nsi: *mut nsinfo);

    pub fn nsinfo__need_setns(nsi: *const nsinfo) -> bool;
    pub fn nsinfo__clear_need_setns(nsi: *mut nsinfo);
    pub fn nsinfo__tgid(nsi: *const nsinfo) -> pid_t;
    pub fn nsinfo__nstgid(nsi: *const nsinfo) -> pid_t;
    pub fn nsinfo__pid(nsi: *const nsinfo) -> pid_t;
    pub fn nsinfo__in_pidns(nsi: *const nsinfo) -> bool;
    pub fn nsinfo__set_in_pidns(nsi: *mut nsinfo);

    pub fn nsinfo__mountns_enter(nsi: *mut nsinfo, nc: *mut nscookie);
    pub fn nsinfo__mountns_exit(nc: *mut nscookie);

    pub fn nsinfo__realpath(path: *const c_char, nsi: *mut nsinfo) -> *mut c_char;
    pub fn nsinfo__stat(filename: *const c_char, st: *mut stat, nsi: *mut nsinfo) -> c_int;

    pub fn nsinfo__is_in_root_namespace() -> bool;
}

#[inline]
pub unsafe fn __nsinfo__zput(nsip: *mut *mut nsinfo) {
    if !nsip.is_null() {
        unsafe {
            nsinfo__put(*nsip);
            *nsip = ptr::null_mut();
        }
    }
}

// C macro:
// #define nsinfo__zput(nsi) __nsinfo__zput(&nsi)
#[inline]
pub unsafe fn nsinfo__zput(nsi: *mut *mut nsinfo) {
    unsafe {
        __nsinfo__zput(nsi);
    }
}

unsafe extern "C" {
    pub fn perf_ns__name(id: c_uint) -> *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
