/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2013 Nicira, Inc.
 */

/*
 * C dependencies supplied by the surrounding translation unit:
 * linux kernel, netlink, Open vSwitch, networking, and flow declarations.
 */

extern "C" {
    pub fn ovs_tun_key_attr_size() -> usize;
    pub fn ovs_key_attr_size() -> usize;

    pub fn ovs_match_init(
        r#match: *mut sw_flow_match,
        key: *mut sw_flow_key,
        reset_key: bool,
        mask: *mut sw_flow_mask,
    );

    pub fn ovs_nla_put_key(
        key: *const sw_flow_key,
        mask: *const sw_flow_key,
        attr: core::ffi::c_int,
        is_mask: bool,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;
    pub fn parse_flow_nlattrs(
        attr: *const nlattr,
        a: *const *const nlattr,
        attrsp: *mut u64,
        log: bool,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_get_flow_metadata(
        net: *mut net,
        a: *const *const nlattr,
        attrs: u64,
        key: *mut sw_flow_key,
        log: bool,
    ) -> core::ffi::c_int;

    pub fn ovs_nla_put_identifier(
        flow: *const sw_flow,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_put_masked_key(
        flow: *const sw_flow,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_put_mask(
        flow: *const sw_flow,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;

    pub fn ovs_nla_get_match(
        net: *mut net,
        r#match: *mut sw_flow_match,
        key: *const nlattr,
        mask: *const nlattr,
        log: bool,
    ) -> core::ffi::c_int;

    pub fn ovs_nla_put_tunnel_info(
        skb: *mut sk_buff,
        tun_info: *mut ip_tunnel_info,
    ) -> core::ffi::c_int;

    pub fn ovs_nla_get_ufid(
        sfid: *mut sw_flow_id,
        attr: *const nlattr,
        log: bool,
    ) -> bool;
    pub fn ovs_nla_get_identifier(
        sfid: *mut sw_flow_id,
        ufid: *const nlattr,
        key: *const sw_flow_key,
        log: bool,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_get_ufid_flags(attr: *const nlattr) -> u32;

    pub fn ovs_nla_copy_actions(
        net: *mut net,
        attr: *const nlattr,
        key: *const sw_flow_key,
        sfa: *mut *mut sw_flow_actions,
        log: bool,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_add_action(
        sfa: *mut *mut sw_flow_actions,
        attrtype: core::ffi::c_int,
        data: *mut core::ffi::c_void,
        len: core::ffi::c_int,
        log: bool,
    ) -> core::ffi::c_int;
    pub fn ovs_nla_put_actions(
        attr: *const nlattr,
        len: core::ffi::c_int,
        skb: *mut sk_buff,
    ) -> core::ffi::c_int;

    pub fn ovs_nla_free_flow_actions(sfa: *mut sw_flow_actions);
    pub fn ovs_nla_free_flow_actions_rcu(sfa: *mut sw_flow_actions);

    pub fn nsh_hdr_from_nlattr(
        attr: *const nlattr,
        nh: *mut nshhdr,
        size: usize,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
