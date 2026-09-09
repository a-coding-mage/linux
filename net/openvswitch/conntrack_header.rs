/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2015 Nicira, Inc.
 */

// Translated from conntrack.h.  Symbols from flow.h and the kernel are
// supplied by other translation units.

pub struct ovs_conntrack_info;
pub struct ovs_ct_limit_info;
pub enum ovs_key_attr {}

// The following declarations are active when CONFIG_NF_CONNTRACK is enabled.
#[cfg(feature = "CONFIG_NF_CONNTRACK")]
extern "C" {
    pub fn ovs_ct_init(net: *mut net) -> ::std::os::raw::c_int;
    pub fn ovs_ct_exit_start(net: *mut net);
    pub fn ovs_ct_exit_finish(net: *mut net);
    pub fn ovs_ct_verify(net: *mut net, attr: ovs_key_attr) -> bool;
    pub fn ovs_ct_copy_action(
        net: *mut net,
        nla: *const nlattr,
        key: *const sw_flow_key,
        acts: *mut *mut sw_flow_actions,
        log: bool,
    ) -> ::std::os::raw::c_int;
    pub fn ovs_ct_action_to_attr(
        info: *const ovs_conntrack_info,
        skb: *mut sk_buff,
    ) -> ::std::os::raw::c_int;
    pub fn ovs_ct_execute(
        net: *mut net,
        skb: *mut sk_buff,
        key: *mut sw_flow_key,
        info: *const ovs_conntrack_info,
    ) -> ::std::os::raw::c_int;
    pub fn ovs_ct_clear(skb: *mut sk_buff, key: *mut sw_flow_key) -> ::std::os::raw::c_int;
    pub fn ovs_ct_fill_key(skb: *const sk_buff, key: *mut sw_flow_key, post_ct: bool);
    pub fn ovs_ct_put_key(
        swkey: *const sw_flow_key,
        output: *const sw_flow_key,
        skb: *mut sk_buff,
    ) -> ::std::os::raw::c_int;
    pub fn ovs_ct_free_action(a: *const nlattr);
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
pub const CT_SUPPORTED_MASK: u32 = OVS_CS_F_NEW
    | OVS_CS_F_ESTABLISHED
    | OVS_CS_F_RELATED
    | OVS_CS_F_REPLY_DIR
    | OVS_CS_F_INVALID
    | OVS_CS_F_TRACKED
    | OVS_CS_F_SRC_NAT
    | OVS_CS_F_DST_NAT;

// Stubs corresponding to the CONFIG_NF_CONNTRACK-disabled branch.
#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_init(_net: *mut net) -> ::std::os::raw::c_int { 0 }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_exit_start(_net: *mut net) {}
#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_exit_finish(_net: *mut net) {}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_verify(_net: *mut net, _attr: ::std::os::raw::c_int) -> bool { false }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_copy_action(
    _net: *mut net,
    _nla: *const nlattr,
    _key: *const sw_flow_key,
    _acts: *mut *mut sw_flow_actions,
    _log: bool,
) -> ::std::os::raw::c_int { -ENOTSUPP }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_action_to_attr(
    _info: *const ovs_conntrack_info,
    _skb: *mut sk_buff,
) -> ::std::os::raw::c_int { -ENOTSUPP }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_execute(
    _net: *mut net,
    skb: *mut sk_buff,
    _key: *mut sw_flow_key,
    _info: *const ovs_conntrack_info,
) -> ::std::os::raw::c_int {
    kfree_skb(skb);
    -ENOTSUPP
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_clear(_skb: *mut sk_buff, _key: *mut sw_flow_key) -> ::std::os::raw::c_int { -ENOTSUPP }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_fill_key(_skb: *const sk_buff, key: *mut sw_flow_key, _post_ct: bool) {
    (*key).ct_state = 0;
    (*key).ct_zone = 0;
    (*key).ct.mark = 0;
    memset(&mut (*key).ct.labels, 0, ::std::mem::size_of_val(&(*key).ct.labels));
    // Clear 'ct_orig_proto' to mark the non-existence of original
    // direction key fields.
    (*key).ct_orig_proto = 0;
}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_put_key(
    _swkey: *const sw_flow_key,
    _output: *const sw_flow_key,
    _skb: *mut sk_buff,
) -> ::std::os::raw::c_int { 0 }

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub unsafe fn ovs_ct_free_action(_a: *const nlattr) {}

#[cfg(not(feature = "CONFIG_NF_CONNTRACK"))]
pub const CT_SUPPORTED_MASK: u32 = 0;

#[cfg(feature = "CONFIG_NETFILTER_CONNCOUNT")]
extern "C" {
    pub static mut dp_ct_limit_genl_family: genl_family;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
