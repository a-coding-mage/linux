/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the C header:
// linux/types.h and net/act_api.h provide __be32, u32, rcu_head, and tc_action.

#[repr(C)]
pub struct tcf_nat_parms {
    pub action: i32,
    pub old_addr: __be32,
    pub new_addr: __be32,
    pub mask: __be32,
    pub flags: u32,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct tcf_nat {
    pub common: tc_action,
    // C __rcu annotation is an address-space/access-discipline qualifier;
    // the underlying field is a raw pointer.
    pub parms: *mut tcf_nat_parms,
}

#[inline]
pub unsafe fn to_tcf_nat(a: *mut tc_action) -> *mut tcf_nat {
    a as *mut tcf_nat
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
