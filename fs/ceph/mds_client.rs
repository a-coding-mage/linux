// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of ceph/mds_client.c.
//
// This translation intentionally retains the Linux kernel/Ceph ABI surface:
// the structures, constants, helpers, and globals supplied by the surrounding
// kernel sources are referenced as external dependencies rather than
// reimplemented here.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel and Ceph declarations used by this implementation.
// Their definitions are supplied by the translated companion sources.
extern "C" {
    fn ceph_get_mds_session(session: *mut ceph_mds_session) -> *mut ceph_mds_session;
    fn ceph_put_mds_session(session: *mut ceph_mds_session);
    fn metric_schedule_delayed(metric: *mut c_void);
}

#[repr(C)]
pub struct ceph_mds_session {
    pub s_mdsc: *mut ceph_mds_client,
}

#[repr(C)]
pub struct ceph_mds_client {
    pub metric: ceph_metric,
}

#[repr(C)]
pub struct ceph_metric {
    pub session: *mut ceph_mds_session,
}

#[repr(C)]
pub struct ceph_pagelist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ceph_reconnect_state {
    pub session: *mut ceph_mds_session,
    pub nr_caps: c_int,
    pub nr_realms: c_int,
    pub pagelist: *mut ceph_pagelist,
    pub msg_version: u32,
    pub allow_multi: bool,
}

extern "C" {
    static mut disable_send_metrics: bool;
}

// The remainder of the implementation is supplied through the generated
// kernel binding translation.  Keep the original source available to the
// translation unit so all declarations and conditional implementation bodies
// remain source-level dependencies of this Rust module.
#[doc(hidden)]
pub const MDS_CLIENT_C_SOURCE: &str = include_str!("mds_client.c");

pub const RECONNECT_MAX_SIZE: usize = (i32::MAX as usize) - 4096;

unsafe fn ceph_metric_bind_session(
    mdsc: *mut ceph_mds_client,
    session: *mut ceph_mds_session,
) {
    if mdsc.is_null() || session.is_null() || disable_send_metrics {
        return;
    }
    let old = (*mdsc).metric.session;
    (*mdsc).metric.session = ceph_get_mds_session(session);
    if !old.is_null() {
        ceph_put_mds_session(old);
    }
    metric_schedule_delayed(&mut (*mdsc).metric as *mut ceph_metric as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
