/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2008, Intel Corporation.
 *
 * Author: Alexander Duyck <alexander.h.duyck@intel.com>
 */

// Dependencies supplied by the surrounding translation unit:
// net/act_api.h and linux/tc_act/tc_skbedit.h

#[repr(C)]
pub struct tcf_skbedit_params {
    pub action: ::core::ffi::c_int,
    pub flags: u32,
    pub priority: u32,
    pub mark: u32,
    pub mask: u32,
    pub queue_mapping: u16,
    pub mapping_mod: u16,
    pub ptype: u16,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_skbedit {
    pub common: tc_action,
    pub params: *mut tcf_skbedit_params,
}

#[inline]
pub unsafe fn to_skbedit(a: *const tc_action) -> *const tcf_skbedit {
    a as *const tcf_skbedit
}

/* Return true iff action is the one identified by FLAG. */
#[inline]
pub unsafe fn is_tcf_skbedit_with_flag(a: *const tc_action, flag: u32) -> bool {
    // C conditional: CONFIG_NET_CLS_ACT
    #[cfg(feature = "CONFIG_NET_CLS_ACT")]
    {
        let mut flags: u32;

        if !(*a).ops.is_null() && (*(*a).ops).id == TCA_ID_SKBEDIT {
            rcu_read_lock();
            flags = (*rcu_dereference((*to_skbedit(a)).params)).flags;
            rcu_read_unlock();
            return flags == flag;
        }
    }
    false
}

/* Return true iff action is mark */
#[inline]
pub unsafe fn is_tcf_skbedit_mark(a: *const tc_action) -> bool {
    is_tcf_skbedit_with_flag(a, SKBEDIT_F_MARK)
}

#[inline]
pub unsafe fn tcf_skbedit_mark(a: *const tc_action) -> u32 {
    let mark: u32;

    rcu_read_lock();
    mark = (*rcu_dereference((*to_skbedit(a)).params)).mark;
    rcu_read_unlock();

    mark
}

/* Return true iff action is ptype */
#[inline]
pub unsafe fn is_tcf_skbedit_ptype(a: *const tc_action) -> bool {
    is_tcf_skbedit_with_flag(a, SKBEDIT_F_PTYPE)
}

#[inline]
pub unsafe fn tcf_skbedit_ptype(a: *const tc_action) -> u32 {
    let ptype: u16;

    rcu_read_lock();
    ptype = (*rcu_dereference((*to_skbedit(a)).params)).ptype;
    rcu_read_unlock();

    ptype as u32
}

/* Return true iff action is priority */
#[inline]
pub unsafe fn is_tcf_skbedit_priority(a: *const tc_action) -> bool {
    is_tcf_skbedit_with_flag(a, SKBEDIT_F_PRIORITY)
}

#[inline]
pub unsafe fn tcf_skbedit_priority(a: *const tc_action) -> u32 {
    let priority: u32;

    rcu_read_lock();
    priority = (*rcu_dereference((*to_skbedit(a)).params)).priority;
    rcu_read_unlock();

    priority
}

#[inline]
pub unsafe fn tcf_skbedit_rx_queue_mapping(a: *const tc_action) -> u16 {
    let rx_queue: u16;

    rcu_read_lock();
    rx_queue = (*rcu_dereference((*to_skbedit(a)).params)).queue_mapping;
    rcu_read_unlock();

    rx_queue
}

/* Return true iff action is queue_mapping */
#[inline]
pub unsafe fn is_tcf_skbedit_queue_mapping(a: *const tc_action) -> bool {
    is_tcf_skbedit_with_flag(a, SKBEDIT_F_QUEUE_MAPPING)
}

/* Return true if action is on ingress traffic */
#[inline]
pub unsafe fn is_tcf_skbedit_ingress(flags: u32) -> bool {
    (flags & TCA_ACT_FLAGS_AT_INGRESS) != 0
}

#[inline]
pub unsafe fn is_tcf_skbedit_tx_queue_mapping(a: *const tc_action) -> bool {
    is_tcf_skbedit_queue_mapping(a) &&
        !is_tcf_skbedit_ingress((*a).tcfa_flags)
}

#[inline]
pub unsafe fn is_tcf_skbedit_rx_queue_mapping(a: *const tc_action) -> bool {
    is_tcf_skbedit_queue_mapping(a) &&
        is_tcf_skbedit_ingress((*a).tcfa_flags)
}

/* Return true iff action is inheritdsfield */
#[inline]
pub unsafe fn is_tcf_skbedit_inheritdsfield(a: *const tc_action) -> bool {
    is_tcf_skbedit_with_flag(a, SKBEDIT_F_INHERITDSFIELD)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
