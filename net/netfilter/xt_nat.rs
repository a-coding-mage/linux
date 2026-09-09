// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2011 Patrick McHardy <kaber@trash.net>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn xt_nat_checkentry_v0(par: *const xt_tgchk_param) -> c_int {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;
    if (*mr).rangesize != 1 {
        pr_info_ratelimited!("multiple ranges no longer supported\n");
        return -EINVAL;
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn xt_nat_checkentry(par: *const xt_tgchk_param) -> c_int {
    match (*par).family {
        NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_INET => {}
        _ => return -EINVAL,
    }
    nf_ct_netns_get((*par).net, (*par).family)
}

unsafe fn xt_nat_destroy(par: *const xt_tgdtor_param) {
    nf_ct_netns_put((*par).net, (*par).family);
}

unsafe fn xt_nat_convert_range(
    dst: *mut nf_nat_range2,
    src: *const nf_nat_ipv4_range,
) {
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*dst).min_addr), 0, 1);
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*dst).max_addr), 0, 1);
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*dst).base_proto), 0, 1);
    (*dst).flags = (*src).flags;
    (*dst).min_addr.ip = (*src).min_ip;
    (*dst).max_addr.ip = (*src).max_ip;
    (*dst).min_proto = (*src).min;
    (*dst).max_proto = (*src).max;
}

unsafe fn xt_snat_target_v0(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED || ctinfo == IP_CT_RELATED_REPLY) ));
    xt_nat_convert_range(&mut range, &(*mr).range[0]);
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_SRC)
}

unsafe fn xt_dnat_target_v0(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let mr = (*par).targinfo as *const nf_nat_ipv4_multi_range_compat;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED) ));
    xt_nat_convert_range(&mut range, &(*mr).range[0]);
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_DST)
}

unsafe fn xt_snat_target_v1(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let range_v1 = (*par).targinfo as *const nf_nat_range;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED || ctinfo == IP_CT_RELATED_REPLY) ));
    core::ptr::copy_nonoverlapping(range_v1 as *const u8, &mut range as *mut _ as *mut u8, core::mem::size_of::<nf_nat_range>());
    core::ptr::write_bytes(core::ptr::addr_of_mut!(range.base_proto), 0, 1);
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_SRC)
}

unsafe fn xt_dnat_target_v1(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let range_v1 = (*par).targinfo as *const nf_nat_range;
    let mut range: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED) ));
    core::ptr::copy_nonoverlapping(range_v1 as *const u8, &mut range as *mut _ as *mut u8, core::mem::size_of::<nf_nat_range>());
    core::ptr::write_bytes(core::ptr::addr_of_mut!(range.base_proto), 0, 1);
    nf_nat_setup_info(ct, &range, NF_NAT_MANIP_DST)
}

unsafe fn xt_snat_target_v2(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let range = (*par).targinfo as *const nf_nat_range2;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED || ctinfo == IP_CT_RELATED_REPLY) ));
    nf_nat_setup_info(ct, range, NF_NAT_MANIP_SRC)
}

unsafe fn xt_dnat_target_v2(skb: *mut sk_buff, par: *const xt_action_param) -> c_uint {
    let range = (*par).targinfo as *const nf_nat_range2;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    WARN_ON!(!( !ct.is_null() && (ctinfo == IP_CT_NEW || ctinfo == IP_CT_RELATED) ));
    nf_nat_setup_info(ct, range, NF_NAT_MANIP_DST)
}

static mut xt_nat_target_reg: [xt_target; 6] = [
    xt_target { name: c"SNAT", revision: 0, checkentry: Some(xt_nat_checkentry_v0), destroy: Some(xt_nat_destroy), target: Some(xt_snat_target_v0), targetsize: core::mem::size_of::<nf_nat_ipv4_multi_range_compat>(), family: NFPROTO_IPV4, table: c"nat", hooks: (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE },
    xt_target { name: c"DNAT", revision: 0, checkentry: Some(xt_nat_checkentry_v0), destroy: Some(xt_nat_destroy), target: Some(xt_dnat_target_v0), targetsize: core::mem::size_of::<nf_nat_ipv4_multi_range_compat>(), family: NFPROTO_IPV4, table: c"nat", hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT), me: THIS_MODULE },
    xt_target { name: c"SNAT", revision: 1, checkentry: Some(xt_nat_checkentry), destroy: Some(xt_nat_destroy), target: Some(xt_snat_target_v1), targetsize: core::mem::size_of::<nf_nat_range>(), table: c"nat", hooks: (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE },
    xt_target { name: c"DNAT", revision: 1, checkentry: Some(xt_nat_checkentry), destroy: Some(xt_nat_destroy), target: Some(xt_dnat_target_v1), targetsize: core::mem::size_of::<nf_nat_range>(), table: c"nat", hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT), me: THIS_MODULE },
    xt_target { name: c"SNAT", revision: 2, checkentry: Some(xt_nat_checkentry), destroy: Some(xt_nat_destroy), target: Some(xt_snat_target_v2), targetsize: core::mem::size_of::<nf_nat_range2>(), table: c"nat", hooks: (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN), me: THIS_MODULE },
    xt_target { name: c"DNAT", revision: 2, checkentry: Some(xt_nat_checkentry), destroy: Some(xt_nat_destroy), target: Some(xt_dnat_target_v2), targetsize: core::mem::size_of::<nf_nat_range2>(), table: c"nat", hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT), me: THIS_MODULE },
];

unsafe fn xt_nat_init() -> c_int {
    xt_register_targets(xt_nat_target_reg.as_mut_ptr(), xt_nat_target_reg.len())
}

unsafe fn xt_nat_exit() {
    xt_unregister_targets(xt_nat_target_reg.as_mut_ptr(), xt_nat_target_reg.len());
}

// module_init(xt_nat_init); module_exit(xt_nat_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Patrick McHardy <kaber@trash.net>");
// MODULE_ALIAS("ipt_SNAT"); MODULE_ALIAS("ipt_DNAT");
// MODULE_ALIAS("ip6t_SNAT"); MODULE_ALIAS("ip6t_DNAT");
// MODULE_DESCRIPTION("SNAT and DNAT targets support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
