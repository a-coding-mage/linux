/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2014 Jiri Pirko <jiri@resnulli.us>
 */

// C dependencies: <net/act_api.h> and <linux/tc_act/tc_vlan.h>.

#[repr(C)]
pub struct tcf_vlan_params {
    pub action: ::core::ffi::c_int,
    pub tcfv_action: ::core::ffi::c_int,
    pub tcfv_push_dst: [u8; ETH_ALEN],
    pub tcfv_push_src: [u8; ETH_ALEN],
    pub tcfv_push_vid: u16,
    pub tcfv_push_proto: __be16,
    pub tcfv_push_prio: u8,
    pub tcfv_push_prio_exists: bool,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_vlan {
    pub common: tc_action,
    pub vlan_p: *mut tcf_vlan_params,
}

#[inline]
pub unsafe fn to_vlan(a: *mut tc_action) -> *mut tcf_vlan {
    a as *mut tcf_vlan
}

#[inline]
pub unsafe fn tcf_vlan_action(a: *const tc_action) -> u32 {
    let tcfv_action: u32;

    rcu_read_lock();
    tcfv_action = (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_action as u32;
    rcu_read_unlock();

    tcfv_action
}

#[inline]
pub unsafe fn tcf_vlan_push_vid(a: *const tc_action) -> u16 {
    let tcfv_push_vid: u16;

    rcu_read_lock();
    tcfv_push_vid = (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_push_vid;
    rcu_read_unlock();

    tcfv_push_vid
}

#[inline]
pub unsafe fn tcf_vlan_push_proto(a: *const tc_action) -> __be16 {
    let tcfv_push_proto: __be16;

    rcu_read_lock();
    tcfv_push_proto = (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_push_proto;
    rcu_read_unlock();

    tcfv_push_proto
}

#[inline]
pub unsafe fn tcf_vlan_push_prio(a: *const tc_action) -> u8 {
    let tcfv_push_prio: u8;

    rcu_read_lock();
    tcfv_push_prio = (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_push_prio;
    rcu_read_unlock();

    tcfv_push_prio
}

#[inline]
pub unsafe fn tcf_vlan_push_eth(
    src: *mut u8,
    dest: *mut u8,
    a: *const tc_action,
) {
    rcu_read_lock();
    ::core::ptr::copy_nonoverlapping(
        (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_push_dst.as_ptr(),
        dest,
        ETH_ALEN,
    );
    ::core::ptr::copy_nonoverlapping(
        (*rcu_dereference((*to_vlan(a as *mut tc_action)).vlan_p)).tcfv_push_src.as_ptr(),
        src,
        ETH_ALEN,
    );
    rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
