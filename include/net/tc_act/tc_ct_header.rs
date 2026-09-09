/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <net/act_api.h>
// #include <uapi/linux/tc_act/tc_ct.h>

// This block is conditionally present when CONFIG_NF_CONNTRACK is enabled.
#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[repr(C)]
pub struct tcf_ct_params {
    pub helper: *mut nf_conntrack_helper,
    pub tmpl: *mut nf_conn,
    pub zone: u16,
    pub action: i32,
    pub mark: u32,
    pub mark_mask: u32,

    pub labels: [u32; NF_CT_LABELS_MAX_SIZE / core::mem::size_of::<u32>()],
    pub labels_mask: [u32; NF_CT_LABELS_MAX_SIZE / core::mem::size_of::<u32>()],

    pub range: nf_nat_range2,
    pub ipv4_range: bool,
    pub put_labels: bool,

    pub ct_action: u16,

    pub rcu: rcu_head,

    pub ct_ft: *mut tcf_ct_flow_table,
    pub nf_ft: *mut nf_flowtable,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[repr(C)]
pub struct tcf_ct {
    pub common: tc_action,
    pub params: *mut tcf_ct_params,
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn to_ct(a: *mut tc_action) -> *mut tcf_ct {
    a as *mut tcf_ct
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn to_ct_params(a: *mut tc_action) -> *mut tcf_ct_params {
    // Equivalent to rcu_dereference_protected(to_ct(a)->params,
    // lockdep_is_held(&a->tcfa_lock)).
    (*to_ct(a)).params
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn tcf_ct_zone(a: *const tc_action) -> u16 {
    (*to_ct_params(a as *mut tc_action)).zone
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn tcf_ct_action(a: *const tc_action) -> i32 {
    (*to_ct_params(a as *mut tc_action)).ct_action as i32
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn tcf_ct_ft(a: *const tc_action) -> *mut nf_flowtable {
    (*to_ct_params(a as *mut tc_action)).nf_ft
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[inline]
pub unsafe fn tcf_ct_helper(a: *const tc_action) -> *mut nf_conntrack_helper {
    (*to_ct_params(a as *mut tc_action)).helper
}

// Fallbacks when CONFIG_NF_CONNTRACK is disabled.
#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
#[inline]
pub unsafe fn tcf_ct_zone(_a: *const tc_action) -> u16 { 0 }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
#[inline]
pub unsafe fn tcf_ct_action(_a: *const tc_action) -> i32 { 0 }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
#[inline]
pub unsafe fn tcf_ct_ft(_a: *const tc_action) -> *mut nf_flowtable { core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
#[inline]
pub unsafe fn tcf_ct_helper(_a: *const tc_action) -> *mut nf_conntrack_helper {
    core::ptr::null_mut()
}

// This block is conditionally present when CONFIG_NET_ACT_CT is enabled.
#[cfg(feature = "CONFIG_NET_ACT_CT")]
#[inline]
pub unsafe fn tcf_ct_flow_table_restore_skb(skb: *mut sk_buff, cookie: usize) {
    let ctinfo: ip_conntrack_info = (cookie & NFCT_INFOMASK) as ip_conntrack_info;
    let ct: *mut nf_conn = (cookie & NFCT_PTRMASK) as *mut nf_conn;

    nf_conntrack_get(&mut (*ct).ct_general);
    nf_ct_set(skb, ct, ctinfo);
}

#[cfg(not(feature = "CONFIG_NET_ACT_CT"))]
#[inline]
pub unsafe fn tcf_ct_flow_table_restore_skb(_skb: *mut sk_buff, _cookie: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
