/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (C) 2019 Netronome Systems, Inc. */

// Dependencies supplied by the Linux networking headers:
// <linux/tc_act/tc_mpls.h> and <net/act_api.h>

#[repr(C)]
pub struct tcf_mpls_params {
    pub tcfm_action: ::core::ffi::c_int,
    pub tcfm_label: u32,
    pub action: ::core::ffi::c_int, /* tcf_action */
    pub tcfm_tc: u8,
    pub tcfm_ttl: u8,
    pub tcfm_bos: u8,
    pub tcfm_proto: u16, // __be16
    pub rcu: rcu_head,
}

pub const ACT_MPLS_TC_NOT_SET: u8 = 0xff;
pub const ACT_MPLS_BOS_NOT_SET: u8 = 0xff;
pub const ACT_MPLS_LABEL_NOT_SET: u32 = 0xffff_ffff;

#[repr(C)]
pub struct tcf_mpls {
    pub common: tc_action,
    pub mpls_p: *mut tcf_mpls_params, // struct tcf_mpls_params __rcu *
}

#[inline]
pub unsafe fn to_mpls(a: *const tc_action) -> *const tcf_mpls {
    a as *const tcf_mpls
}

#[inline]
pub unsafe fn tcf_mpls_action(a: *const tc_action) -> u32 {
    let tcfm_action: u32;

    rcu_read_lock();
    tcfm_action = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_action as u32;
    rcu_read_unlock();

    tcfm_action
}

#[inline]
pub unsafe fn tcf_mpls_proto(a: *const tc_action) -> u16 {
    let tcfm_proto: u16;

    rcu_read_lock();
    tcfm_proto = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_proto;
    rcu_read_unlock();

    tcfm_proto
}

#[inline]
pub unsafe fn tcf_mpls_label(a: *const tc_action) -> u32 {
    let tcfm_label: u32;

    rcu_read_lock();
    tcfm_label = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_label;
    rcu_read_unlock();

    tcfm_label
}

#[inline]
pub unsafe fn tcf_mpls_tc(a: *const tc_action) -> u8 {
    let tcfm_tc: u8;

    rcu_read_lock();
    tcfm_tc = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_tc;
    rcu_read_unlock();

    tcfm_tc
}

#[inline]
pub unsafe fn tcf_mpls_bos(a: *const tc_action) -> u8 {
    let tcfm_bos: u8;

    rcu_read_lock();
    tcfm_bos = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_bos;
    rcu_read_unlock();

    tcfm_bos
}

#[inline]
pub unsafe fn tcf_mpls_ttl(a: *const tc_action) -> u8 {
    let tcfm_ttl: u8;

    rcu_read_lock();
    tcfm_ttl = (*rcu_dereference((*to_mpls(a)).mpls_p)).tcfm_ttl;
    rcu_read_unlock();

    tcfm_ttl
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
