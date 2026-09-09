/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2016, Jamal Hadi Salim
 */

// Dependencies supplied by the surrounding kernel translation:
// <net/act_api.h>
// <linux/tc_act/tc_skbmod.h>

#[repr(C)]
pub struct tcf_skbmod_params {
    pub rcu: rcu_head,
    /// up to 64 types of operations; extend if needed
    pub flags: u64,
    pub action: i32,
    pub eth_dst: [u8; ETH_ALEN],
    pub eth_type: u16,
    pub eth_src: [u8; ETH_ALEN],
}

#[repr(C)]
pub struct tcf_skbmod {
    pub common: tc_action,
    pub skbmod_p: *mut tcf_skbmod_params,
}

#[inline]
pub unsafe fn to_skbmod<T>(a: *mut T) -> *mut tcf_skbmod {
    a as *mut tcf_skbmod
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
