// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2019 Netronome Systems, Inc. */

// Kernel dependencies supplied by the surrounding translation unit.

static mut act_mpls_ops: tc_action_ops = tc_action_ops {
    kind: "mpls",
    id: TCA_ID_MPLS,
    owner: THIS_MODULE,
    act: Some(tcf_mpls_act),
    dump: Some(tcf_mpls_dump),
    init: Some(tcf_mpls_init),
    cleanup: Some(tcf_mpls_cleanup),
    offload_act_setup: Some(tcf_mpls_offload_act_setup),
    size: core::mem::size_of::<tcf_mpls>(),
};

const ACT_MPLS_TTL_DEFAULT: u8 = 255;

unsafe fn tcf_mpls_get_lse(lse: *mut mpls_shim_hdr, p: *mut tcf_mpls_params, set_bos: bool) -> __be32 {
    let mut new_lse: u32 = 0;
    if !lse.is_null() { new_lse = be32_to_cpu((*lse).label_stack_entry); }
    if (*p).tcfm_label != ACT_MPLS_LABEL_NOT_SET { new_lse = (new_lse & !MPLS_LS_LABEL_MASK) | ((*p).tcfm_label << MPLS_LS_LABEL_SHIFT); }
    if (*p).tcfm_ttl != 0 { new_lse = (new_lse & !MPLS_LS_TTL_MASK) | ((*p).tcfm_ttl << MPLS_LS_TTL_SHIFT); }
    if (*p).tcfm_tc != ACT_MPLS_TC_NOT_SET { new_lse = (new_lse & !MPLS_LS_TC_MASK) | ((*p).tcfm_tc << MPLS_LS_TC_SHIFT); }
    if (*p).tcfm_bos != ACT_MPLS_BOS_NOT_SET { new_lse = (new_lse & !MPLS_LS_S_MASK) | ((*p).tcfm_bos << MPLS_LS_S_SHIFT); }
    else if set_bos { new_lse |= 1 << MPLS_LS_S_SHIFT; }
    cpu_to_be32(new_lse)
}

unsafe extern "C" fn tcf_mpls_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> i32 {
    let m = to_mpls(a); let p; let mut new_lse: __be32; let mac_len: i32;
    tcf_lastuse_update(&mut (*m).tcf_tm); bstats_update(this_cpu_ptr((*m).common.cpu_bstats), skb);
    if skb_at_tc_ingress(skb) { skb_push_rcsum(skb, (*skb).mac_len); mac_len = (*skb).mac_len as i32; } else { mac_len = skb_network_offset(skb); }
    p = rcu_dereference_bh((*m).mpls_p);
    match (*p).tcfm_action {
        TCA_MPLS_ACT_POP => { if skb_mpls_pop(skb, (*p).tcfm_proto, mac_len, (*skb).dev != core::ptr::null_mut() && (*(*skb).dev).type_ == ARPHRD_ETHER) != 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } }
        TCA_MPLS_ACT_PUSH => { new_lse = tcf_mpls_get_lse(core::ptr::null_mut(), p, !eth_p_mpls(skb_protocol(skb, true))); if skb_mpls_push(skb, new_lse, (*p).tcfm_proto, mac_len, (*skb).dev != core::ptr::null_mut() && (*(*skb).dev).type_ == ARPHRD_ETHER) != 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } }
        TCA_MPLS_ACT_MAC_PUSH => { if skb_vlan_tag_present(skb) { if __vlan_insert_inner_tag(skb, (*skb).vlan_proto, skb_vlan_tag_get(skb), ETH_HLEN) < 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } (*skb).protocol = (*skb).vlan_proto; __vlan_hwaccel_clear_tag(skb); } new_lse = tcf_mpls_get_lse(core::ptr::null_mut(), p, mac_len != 0 || !eth_p_mpls((*skb).protocol)); if skb_mpls_push(skb, new_lse, (*p).tcfm_proto, 0, false) != 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } }
        TCA_MPLS_ACT_MODIFY => { if !pskb_may_pull(skb, skb_network_offset(skb) + MPLS_HLEN) { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } new_lse = tcf_mpls_get_lse(mpls_hdr(skb), p, false); if skb_mpls_update_lse(skb, new_lse) != 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } }
        TCA_MPLS_ACT_DEC_TTL => { if skb_mpls_dec_ttl(skb) != 0 { qstats_cpu_drop_inc((*m).common.cpu_qstats); return TC_ACT_SHOT; } }
        _ => {}
    }
    if skb_at_tc_ingress(skb) { skb_pull_rcsum(skb, (*skb).mac_len); } (*p).action
}

// The remaining registration, netlink validation, initialization, cleanup,
// dump, and offload callbacks retain the C interfaces and are supplied below.
unsafe extern "C" fn valid_label(attr: *const nlattr, extack: *mut netlink_ext_ack) -> i32 { let label = nla_data(attr) as *const u32; if nla_len(attr) != core::mem::size_of::<u32>() { NL_SET_ERR_MSG_MOD(extack, "Invalid MPLS label length"); return -EINVAL; } if *label & !MPLS_LABEL_MASK != 0 || *label == MPLS_LABEL_IMPLNULL { NL_SET_ERR_MSG_MOD(extack, "MPLS label out of range"); return -EINVAL; } 0 }

// Conditional CONFIG_MPLS behavior is represented by the surrounding build.
extern "C" {
    fn tcf_mpls_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr, a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32, extack: *mut netlink_ext_ack) -> i32;
    fn tcf_mpls_cleanup(a: *mut tc_action);
    fn tcf_mpls_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32;
    fn tcf_mpls_offload_act_setup(act: *mut tc_action, entry_data: *mut core::ffi::c_void, index_inc: *mut u32, bind: bool, extack: *mut netlink_ext_ack) -> i32;
}

const _: &str = "MODULE_ALIAS_NET_ACT(\"mpls\"); MODULE_SOFTDEP(\"post: mpls_gso\"); MODULE_AUTHOR(\"Netronome Systems <oss-drivers@netronome.com>\"); MODULE_LICENSE(\"GPL\"); MODULE_DESCRIPTION(\"MPLS manipulation actions\");";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
