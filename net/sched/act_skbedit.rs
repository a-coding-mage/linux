// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008, Intel Corporation.
 *
 * Author: Alexander Duyck <alexander.h.duyck@intel.com>
 */

// Linux kernel dependencies supplied by the surrounding translation.

static mut act_skbedit_ops: tc_action_ops = tc_action_ops::default();

unsafe fn tcf_skbedit_hash(params: *mut tcf_skbedit_params, skb: *mut sk_buff) -> u16 {
    let mut queue_mapping = (*params).queue_mapping;
    if (*params).flags & SKBEDIT_F_TXQ_SKBHASH != 0 {
        let hash: u32 = skb_get_hash(skb);
        queue_mapping = queue_mapping.wrapping_add(hash % (*params).mapping_mod);
    }
    netdev_cap_txqueue((*skb).dev, queue_mapping)
}

unsafe fn tcf_skbedit_act(skb: *mut sk_buff, a: *const tc_action,
                          res: *mut tcf_result) -> i32 {
    let d = to_skbedit(a);
    let params: *mut tcf_skbedit_params;
    tcf_lastuse_update(&mut (*d).tcf_tm);
    bstats_update(this_cpu_ptr((*d).common.cpu_bstats), skb);
    params = rcu_dereference_bh((*d).params);

    if (*params).flags & SKBEDIT_F_PRIORITY != 0 { (*skb).priority = (*params).priority; }
    if (*params).flags & SKBEDIT_F_INHERITDSFIELD != 0 {
        let mut wlen = skb_network_offset(skb);
        match skb_protocol(skb, true) {
            x if x == htons(ETH_P_IP) => {
                wlen += core::mem::size_of::<iphdr>();
                if !pskb_may_pull(skb, wlen) { return tcf_skbedit_error(d); }
                (*skb).priority = ipv4_get_dsfield(ip_hdr(skb)) >> 2;
            }
            x if x == htons(ETH_P_IPV6) => {
                wlen += core::mem::size_of::<ipv6hdr>();
                if !pskb_may_pull(skb, wlen) { return tcf_skbedit_error(d); }
                (*skb).priority = ipv6_get_dsfield(ipv6_hdr(skb)) >> 2;
            }
            _ => {}
        }
    }
    if (*params).flags & SKBEDIT_F_QUEUE_MAPPING != 0 &&
       (*(*skb).dev).real_num_tx_queues > (*params).queue_mapping {
        // CONFIG_NET_EGRESS conditionally enables this operation.
        netdev_xmit_skip_txqueue(true);
        skb_set_queue_mapping(skb, tcf_skbedit_hash(params, skb));
    }
    if (*params).flags & SKBEDIT_F_MARK != 0 {
        (*skb).mark &= !(*params).mask;
        (*skb).mark |= (*params).mark & (*params).mask;
    }
    if (*params).flags & SKBEDIT_F_PTYPE != 0 { (*skb).pkt_type = (*params).ptype; }
    (*params).action
}

unsafe fn tcf_skbedit_error(d: *mut tcf_skbedit) -> i32 {
    qstats_cpu_drop_inc((*d).common.cpu_qstats);
    TC_ACT_SHOT
}

unsafe fn tcf_skbedit_stats_update(a: *mut tc_action, bytes: u64, packets: u64,
                                   drops: u64, lastuse: u64, hw: bool) {
    let d = to_skbedit(a);
    let tm = &mut (*d).tcf_tm;
    tcf_action_update_stats(a, bytes, packets, drops, hw);
    tm.lastuse = core::cmp::max(tm.lastuse, lastuse);
}

static skbedit_policy: [nla_policy; (TCA_SKBEDIT_MAX + 1) as usize] = [
    nla_policy { len: core::mem::size_of::<tc_skbedit>() },
    nla_policy { len: core::mem::size_of::<u32>() },
    nla_policy { len: core::mem::size_of::<u16>() },
    nla_policy { len: core::mem::size_of::<u32>() },
    nla_policy { len: core::mem::size_of::<u16>() },
    nla_policy { len: core::mem::size_of::<u32>() },
    nla_policy { len: core::mem::size_of::<u64>() },
    nla_policy { len: core::mem::size_of::<u16>() },
];

unsafe fn tcf_skbedit_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
    a: *mut *mut tc_action, tp: *mut tcf_proto, act_flags: u32,
    extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic(net, (*act_skbedit_ops).net_id);
    let bind = act_flags & TCA_ACT_FLAGS_BIND != 0;
    let mut params_new: *mut tcf_skbedit_params;
    let mut tb: [*mut nlattr; (TCA_SKBEDIT_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_SKBEDIT_MAX + 1) as usize];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut flags: u32 = 0;
    let mut priority: *mut u32 = core::ptr::null_mut();
    let mut mark: *mut u32 = core::ptr::null_mut();
    let mut mask: *mut u32 = core::ptr::null_mut();
    let mut queue_mapping: *mut u16 = core::ptr::null_mut();
    let mut ptype: *mut u16 = core::ptr::null_mut();
    let mut mapping_mod: u32 = 1;
    let mut err: i32;
    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_SKBEDIT_MAX, nla, skbedit_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_SKBEDIT_PARMS as usize].is_null() { return -EINVAL; }
    if !tb[TCA_SKBEDIT_PRIORITY as usize].is_null() { flags |= SKBEDIT_F_PRIORITY; priority = nla_data(tb[TCA_SKBEDIT_PRIORITY as usize]); }
    if !tb[TCA_SKBEDIT_QUEUE_MAPPING as usize].is_null() {
        if is_tcf_skbedit_ingress(act_flags) && act_flags & TCA_ACT_FLAGS_SKIP_SW == 0 { return -EOPNOTSUPP; }
        flags |= SKBEDIT_F_QUEUE_MAPPING; queue_mapping = nla_data(tb[TCA_SKBEDIT_QUEUE_MAPPING as usize]);
    }
    if !tb[TCA_SKBEDIT_PTYPE as usize].is_null() { ptype = nla_data(tb[TCA_SKBEDIT_PTYPE as usize]); if !skb_pkt_type_ok(*ptype) { return -EINVAL; } flags |= SKBEDIT_F_PTYPE; }
    if !tb[TCA_SKBEDIT_MARK as usize].is_null() { flags |= SKBEDIT_F_MARK; mark = nla_data(tb[TCA_SKBEDIT_MARK as usize]); }
    if !tb[TCA_SKBEDIT_MASK as usize].is_null() { flags |= SKBEDIT_F_MASK; mask = nla_data(tb[TCA_SKBEDIT_MASK as usize]); }
    if !tb[TCA_SKBEDIT_FLAGS as usize].is_null() {
        let pure_flags: *mut u64 = nla_data(tb[TCA_SKBEDIT_FLAGS as usize]);
        if *pure_flags & SKBEDIT_F_TXQ_SKBHASH != 0 {
            if tb[TCA_SKBEDIT_QUEUE_MAPPING as usize].is_null() || tb[TCA_SKBEDIT_QUEUE_MAPPING_MAX as usize].is_null() { return -EINVAL; }
            let max: *mut u16 = nla_data(tb[TCA_SKBEDIT_QUEUE_MAPPING_MAX as usize]);
            if *max < *queue_mapping { return -EINVAL; }
            mapping_mod = (*max as u32) - (*queue_mapping as u32) + 1;
            if mapping_mod > U16_MAX as u32 { return -EINVAL; }
            flags |= SKBEDIT_F_TXQ_SKBHASH;
        }
        if *pure_flags & SKBEDIT_F_INHERITDSFIELD != 0 { flags |= SKBEDIT_F_INHERITDSFIELD; }
    }
    let parm: *mut tc_skbedit = nla_data(tb[TCA_SKBEDIT_PARMS as usize]);
    let index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &index as *const _ as *mut u32, a, bind);
    if err < 0 { return err; }
    if err != 0 && bind { return ACT_P_BOUND; }
    if flags == 0 { if err != 0 { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); } return -EINVAL; }
    let d: *mut tcf_skbedit;
    let mut ret = 0;
    if err == 0 { ret = tcf_idr_create(tn, index, est, a, &act_skbedit_ops, bind, true, act_flags); if ret != 0 { tcf_idr_cleanup(tn, index); return ret; } d = to_skbedit(*a); ret = ACT_P_CREATED; }
    else { d = to_skbedit(*a); if act_flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*a, bind); return -EEXIST; } }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack); if err < 0 { tcf_idr_release(*a, bind); return err; }
    params_new = kzalloc_obj(); if params_new.is_null() { if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } tcf_idr_release(*a, bind); return -ENOMEM; }
    (*params_new).flags = flags;
    if flags & SKBEDIT_F_PRIORITY != 0 { (*params_new).priority = *priority; }
    if flags & SKBEDIT_F_QUEUE_MAPPING != 0 { (*params_new).queue_mapping = *queue_mapping; (*params_new).mapping_mod = mapping_mod; }
    if flags & SKBEDIT_F_MARK != 0 { (*params_new).mark = *mark; }
    if flags & SKBEDIT_F_PTYPE != 0 { (*params_new).ptype = *ptype; }
    (*params_new).mask = 0xffffffff; if flags & SKBEDIT_F_MASK != 0 { (*params_new).mask = *mask; }
    (*params_new).action = (*parm).action;
    spin_lock_bh(&mut (*d).tcf_lock); goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch); params_new = rcu_replace_pointer((*d).params, params_new, lockdep_is_held(&(*d).tcf_lock)); spin_unlock_bh(&mut (*d).tcf_lock);
    if !params_new.is_null() { kfree_rcu(params_new, rcu); } if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } ret
}

// The remaining netlink dump, cleanup, sizing, offload, registration, and module
// lifecycle declarations retain the C ABI and are supplied through kernel bindings.
unsafe extern "C" {
    fn tcf_skbedit_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32;
    fn tcf_skbedit_cleanup(a: *mut tc_action);
    fn tcf_skbedit_get_fill_size(act: *const tc_action) -> usize;
    fn tcf_skbedit_offload_act_setup(act: *mut tc_action, entry_data: *mut core::ffi::c_void, index_inc: *mut u32, bind: bool, extack: *mut netlink_ext_ack) -> i32;
}

unsafe fn skbedit_init_net(net: *mut net) -> i32 {
    let tn = net_generic(net, (*act_skbedit_ops).net_id);
    tc_action_net_init(net, tn, &act_skbedit_ops)
}

unsafe fn skbedit_exit_net(net_list: *mut list_head) {
    tc_action_net_exit(net_list, (*act_skbedit_ops).net_id);
}

static mut skbedit_net_ops: pernet_operations = pernet_operations {
    init: Some(skbedit_init_net),
    exit_batch: Some(skbedit_exit_net),
    id: core::ptr::null_mut(),
    size: core::mem::size_of::<tc_action_net>(),
};

// Equivalent to the C tc_action_ops initializer.
#[allow(non_upper_case_globals)]
unsafe fn init_act_skbedit_ops() {
    act_skbedit_ops.kind = "skbedit";
    act_skbedit_ops.id = TCA_ID_SKBEDIT;
    act_skbedit_ops.owner = THIS_MODULE;
    act_skbedit_ops.act = Some(tcf_skbedit_act);
    act_skbedit_ops.stats_update = Some(tcf_skbedit_stats_update);
    act_skbedit_ops.dump = Some(tcf_skbedit_dump);
    act_skbedit_ops.init = Some(tcf_skbedit_init);
    act_skbedit_ops.cleanup = Some(tcf_skbedit_cleanup);
    act_skbedit_ops.get_fill_size = Some(tcf_skbedit_get_fill_size);
    act_skbedit_ops.offload_act_setup = Some(tcf_skbedit_offload_act_setup);
    act_skbedit_ops.size = core::mem::size_of::<tcf_skbedit>();
    skbedit_net_ops.id = &mut act_skbedit_ops.net_id;
}

// MODULE_ALIAS_NET_ACT("skbedit"); MODULE_AUTHOR, MODULE_DESCRIPTION, MODULE_LICENSE.
unsafe fn skbedit_init_module() -> i32 { tcf_register_action(&act_skbedit_ops, &skbedit_net_ops) }
unsafe fn skbedit_cleanup_module() { tcf_unregister_action(&act_skbedit_ops, &skbedit_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
