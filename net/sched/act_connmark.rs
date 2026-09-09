// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_connmark.c  netfilter connmark retriever action
 * skb mark is over-written
 *
 * Copyright (c) 2011 Felix Fietkau <nbd@openwrt.org>
 */

// External Linux kernel declarations and build-time configuration are supplied
// by the surrounding kernel translation unit.

static mut act_connmark_ops: tc_action_ops = tc_action_ops {
    kind: "connmark",
    id: TCA_ID_CONNMARK,
    owner: THIS_MODULE,
    act: Some(tcf_connmark_act),
    dump: Some(tcf_connmark_dump),
    init: Some(tcf_connmark_init),
    cleanup: Some(tcf_connmark_cleanup),
    size: core::mem::size_of::<tcf_connmark_info>(),
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn tcf_connmark_act(
    skb: *mut sk_buff,
    a: *const tc_action,
    _res: *mut tcf_result,
) -> i32 {
    let mut thash: *const nf_conntrack_tuple_hash;
    let mut tuple: nf_conntrack_tuple = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ca: *mut tcf_connmark_info = to_connmark(a);
    let mut parms: *mut tcf_connmark_parms;
    let mut zone: nf_conntrack_zone = core::mem::zeroed();
    let mut c: *mut nf_conn;
    let mut proto: i32;

    tcf_lastuse_update(&mut (*ca).tcf_tm);
    tcf_action_update_bstats(&mut (*ca).common, skb);

    parms = rcu_dereference_bh((*ca).parms);

    match skb_protocol(skb, true) {
        x if x == htons(ETH_P_IP) => {
            if (*skb).len < core::mem::size_of::<iphdr>() { return (*parms).action; }
            proto = NFPROTO_IPV4;
        }
        x if x == htons(ETH_P_IPV6) => {
            if (*skb).len < core::mem::size_of::<ipv6hdr>() { return (*parms).action; }
            proto = NFPROTO_IPV6;
        }
        _ => return (*parms).action,
    }

    c = nf_ct_get(skb, &mut ctinfo);
    if !c.is_null() {
        (*skb).mark = core::ptr::read_volatile(&(*c).mark);
        tcf_action_inc_overlimit_qstats(&mut (*ca).common);
        return (*parms).action;
    }

    if !nf_ct_get_tuplepr(skb, skb_network_offset(skb), proto, (*parms).net, &mut tuple) {
        return (*parms).action;
    }

    zone.id = (*parms).zone;
    zone.dir = NF_CT_DEFAULT_ZONE_DIR;

    thash = nf_conntrack_find_get((*parms).net, &zone, &tuple);
    if thash.is_null() { return (*parms).action; }

    c = nf_ct_tuplehash_to_ctrack(thash);
    (*skb).mark = core::ptr::read_volatile(&(*c).mark);
    nf_ct_put(c);
    tcf_action_inc_overlimit_qstats(&mut (*ca).common);
    (*parms).action
}

static connmark_policy: [nla_policy; TCA_CONNMARK_MAX as usize + 1] = {
    let mut p: [nla_policy; TCA_CONNMARK_MAX as usize + 1] = unsafe { core::mem::zeroed() };
    p[TCA_CONNMARK_PARMS as usize].len = core::mem::size_of::<tc_connmark>() as u16;
    p
};

pub unsafe fn tcf_connmark_init(
    net: *mut net, nla: *mut nlattr, est: *mut nlattr, a: *mut *mut tc_action,
    tp: *mut tcf_proto, flags: u32, extack: *mut netlink_ext_ack,
) -> i32 {
    let tn = net_generic(net, act_connmark_ops.net_id);
    let mut nparms: *mut tcf_connmark_parms = kzalloc_obj();
    let mut oparms: *mut tcf_connmark_parms;
    let mut tb: [*mut nlattr; TCA_CONNMARK_MAX as usize + 1] = core::mem::zeroed();
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut ci: *mut tcf_connmark_info;
    let parm: *mut tc_connmark;
    let mut ret: i32 = 0;
    let err: i32;
    let mut index: u32;

    if nla.is_null() { return -EINVAL; }
    ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_CONNMARK_MAX, nla, connmark_policy.as_ptr(), core::ptr::null_mut());
    if ret < 0 { return ret; }
    if tb[TCA_CONNMARK_PARMS as usize].is_null() { return -EINVAL; }
    if nparms.is_null() { return -ENOMEM; }
    parm = nla_data(tb[TCA_CONNMARK_PARMS as usize]);
    index = (*parm).index;
    ret = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if ret == 0 {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &act_connmark_ops, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); kfree(nparms as *mut core::ffi::c_void); return ret; }
        ci = to_connmark(*a);
        (*nparms).net = net; (*nparms).zone = (*parm).zone; ret = ACT_P_CREATED;
    } else if ret > 0 {
        ci = to_connmark(*a);
        if bind { kfree(nparms as *mut core::ffi::c_void); return ACT_P_BOUND; }
        if (flags & TCA_ACT_FLAGS_REPLACE) == 0 { tcf_idr_release(*a, bind); kfree(nparms as *mut core::ffi::c_void); return -EEXIST; }
        (*nparms).net = rtnl_dereference((*ci).parms).net; (*nparms).zone = (*parm).zone; ret = 0;
    } else { kfree(nparms as *mut core::ffi::c_void); return ret; }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); kfree(nparms as *mut core::ffi::c_void); return err; }
    (*nparms).action = (*parm).action;
    spin_lock_bh(&mut (*ci).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    oparms = rcu_replace_pointer((*ci).parms, nparms, lockdep_is_held(&(*ci).tcf_lock));
    spin_unlock_bh(&mut (*ci).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if !oparms.is_null() { kfree_rcu(oparms, rcu); }
    ret
}

pub unsafe fn tcf_connmark_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let ci = to_connmark(a);
    let b = skb_tail_pointer(skb);
    let parms: *const tcf_connmark_parms;
    let mut opt: tc_connmark = core::mem::zeroed();
    let mut t: tcf_t = core::mem::zeroed();
    opt.index = (*ci).tcf_index;
    opt.refcnt = refcount_read(&(*ci).tcf_refcnt) - ref_;
    opt.bindcnt = atomic_read(&(*ci).tcf_bindcnt) - bind;
    rcu_read_lock(); parms = rcu_dereference((*ci).parms);
    opt.action = (*parms).action; opt.zone = (*parms).zone;
    if nla_put(skb, TCA_CONNMARK_PARMS, core::mem::size_of::<tc_connmark>() as u16, &opt) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    tcf_tm_dump(&mut t, &(*ci).tcf_tm);
    if nla_put_64bit(skb, TCA_CONNMARK_TM, core::mem::size_of::<tcf_t>() as u16, &t, TCA_CONNMARK_PAD) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    rcu_read_unlock(); (*skb).len as i32
}

pub unsafe fn tcf_connmark_cleanup(a: *mut tc_action) {
    let ci = to_connmark(a); let parms = rcu_dereference_protected((*ci).parms, 1);
    if !parms.is_null() { kfree_rcu(parms, rcu); }
}

unsafe fn connmark_init_net(net: *mut net) -> i32 {
    let tn = net_generic(net, act_connmark_ops.net_id);
    tc_action_net_init(net, tn, &act_connmark_ops)
}
unsafe fn connmark_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_connmark_ops.net_id); }

static mut connmark_net_ops: pernet_operations = pernet_operations {
    init: Some(connmark_init_net), exit_batch: Some(connmark_exit_net), id: unsafe { &mut act_connmark_ops.net_id },
    size: core::mem::size_of::<tc_action_net>(), ..unsafe { core::mem::zeroed() }
};
unsafe fn connmark_init_module() -> i32 { tcf_register_action(&mut act_connmark_ops, &mut connmark_net_ops) }
unsafe fn connmark_cleanup_module() { tcf_unregister_action(&mut act_connmark_ops, &mut connmark_net_ops); }

// module_init(connmark_init_module);
// module_exit(connmark_cleanup_module);
// MODULE_ALIAS_NET_ACT("connmark");
// MODULE_AUTHOR("Felix Fietkau <nbd@openwrt.org>");
// MODULE_DESCRIPTION("Connection tracking mark restoring");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
