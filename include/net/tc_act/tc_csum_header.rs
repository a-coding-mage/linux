/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// linux/types.h, net/act_api.h, and linux/tc_act/tc_csum.h.

#[repr(C)]
pub struct tcf_csum_params {
    pub update_flags: u32,
    pub action: i32,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_csum {
    pub common: tc_action,
    pub params: *mut tcf_csum_params,
}

#[inline]
pub unsafe fn to_tcf_csum(a: *const tc_action) -> *mut tcf_csum {
    a as *mut tcf_csum
}

extern "C" {
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
}

#[inline]
pub unsafe fn tcf_csum_update_flags(a: *const tc_action) -> u32 {
    let update_flags: u32;

    rcu_read_lock();
    update_flags = (*(*to_tcf_csum(a)).params).update_flags;
    rcu_read_unlock();

    update_flags
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
