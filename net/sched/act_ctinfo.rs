// SPDX-License-Identifier: GPL-2.0+
/* net/sched/act_ctinfo.c  netfilter ctinfo connmark actions
 *
 * Copyright (c) 2019 Kevin Darbyshire-Bryant <ldir@darbyshire-bryant.me.uk>
 */

// C includes are supplied by the surrounding kernel translation environment.

static mut ACT_CTINFO_OPS: tc_action_ops = tc_action_ops { };

unsafe fn tcf_ctinfo_dscp_set(
    ct: *mut nf_conn,
    ca: *mut tcf_ctinfo,
    cp: *mut tcf_ctinfo_params,
    skb: *mut sk_buff,
    wlen: i32,
    proto: i32,
) {
    let mut dscp: u8;
    let newdscp: u8 = ((((READ_ONCE((*ct).mark) & (*cp).dscpmask) >> (*cp).dscpmaskshift) << 2)
        & !INET_ECN_MASK) as u8;

    match proto {
        NFPROTO_IPV4 => {
            dscp = (ipv4_get_dsfield(ip_hdr(skb)) & !INET_ECN_MASK) as u8;
            if dscp != newdscp {
                if likely(skb_try_make_writable(skb, wlen) == 0) {
                    ipv4_change_dsfield(ip_hdr(skb), INET_ECN_MASK, newdscp);
                    atomic64_inc(&mut (*ca).stats_dscp_set);
                } else {
                    atomic64_inc(&mut (*ca).stats_dscp_error);
                }
            }
        }
        NFPROTO_IPV6 => {
            dscp = (ipv6_get_dsfield(ipv6_hdr(skb)) & !INET_ECN_MASK) as u8;
            if dscp != newdscp {
                if likely(skb_try_make_writable(skb, wlen) == 0) {
                    ipv6_change_dsfield(ipv6_hdr(skb), INET_ECN_MASK, newdscp);
                    atomic64_inc(&mut (*ca).stats_dscp_set);
                } else {
                    atomic64_inc(&mut (*ca).stats_dscp_error);
                }
            }
        }
        _ => {}
    }
}

unsafe fn tcf_ctinfo_cpmark_set(
    ct: *mut nf_conn,
    ca: *mut tcf_ctinfo,
    cp: *mut tcf_ctinfo_params,
    skb: *mut sk_buff,
) {
    atomic64_inc(&mut (*ca).stats_cpmark_set);
    (*skb).mark = READ_ONCE((*ct).mark) & (*cp).cpmarkmask;
}

unsafe fn tcf_ctinfo_act(
    skb: *mut sk_buff,
    a: *const tc_action,
    res: *mut tcf_result,
) -> i32 {
    let mut thash: *const nf_conntrack_tuple_hash = core::ptr::null();
    let ca: *mut tcf_ctinfo = to_ctinfo(a);
    let mut tuple: nf_conntrack_tuple = core::mem::zeroed();
    let mut zone: nf_conntrack_zone = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let cp: *mut tcf_ctinfo_params = rcu_dereference_bh((*ca).params);
    let mut ct: *mut nf_conn;
    let mut proto: i32;
    let mut wlen: i32;

    tcf_lastuse_update(&mut (*ca).tcf_tm);
    tcf_action_update_bstats(&mut (*ca).common, skb);

    wlen = skb_network_offset(skb);
    match skb_protocol(skb, true) {
        x if x == htons(ETH_P_IP) => {
            wlen += core::mem::size_of::<iphdr>() as i32;
            if !pskb_may_pull(skb, wlen) { return (*cp).action; }
            proto = NFPROTO_IPV4;
        }
        x if x == htons(ETH_P_IPV6) => {
            wlen += core::mem::size_of::<ipv6hdr>() as i32;
            if !pskb_may_pull(skb, wlen) { return (*cp).action; }
            proto = NFPROTO_IPV6;
        }
        _ => return (*cp).action,
    }

    ct = nf_ct_get(skb, &mut ctinfo);
    if ct.is_null() {
        if !nf_ct_get_tuplepr(skb, skb_network_offset(skb), proto, (*cp).net, &mut tuple) {
            return (*cp).action;
        }
        (*(&mut zone)).id = (*cp).zone;
        (*(&mut zone)).dir = NF_CT_DEFAULT_ZONE_DIR;
        thash = nf_conntrack_find_get((*cp).net, &zone, &tuple);
        if thash.is_null() { return (*cp).action; }
        ct = nf_ct_tuplehash_to_ctrack(thash);
    }

    if ((*cp).mode & CTINFO_MODE_DSCP) != 0
        && ((*cp).dscpstatemask == 0 || (READ_ONCE((*ct).mark) & (*cp).dscpstatemask) != 0) {
        tcf_ctinfo_dscp_set(ct, ca, cp, skb, wlen, proto);
    }
    if ((*cp).mode & CTINFO_MODE_CPMARK) != 0 {
        tcf_ctinfo_cpmark_set(ct, ca, cp, skb);
    }
    if !thash.is_null() { nf_ct_put(ct); }
    (*cp).action
}

static ctinfo_policy: [nla_policy; TCA_CTINFO_MAX as usize + 1] = [nla_policy { }; TCA_CTINFO_MAX as usize + 1];

unsafe fn tcf_ctinfo_init(
    net: *mut net, nla: *mut nlattr, est: *mut nlattr, a: *mut *mut tc_action,
    tp: *mut tcf_proto, flags: u32, extack: *mut netlink_ext_ack,
) -> i32 {
    let tn = net_generic(net, ACT_CTINFO_OPS.net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut dscpmask: u32 = 0;
    let mut dscpstatemask: u32 = 0;
    let mut index: u32;
    let mut tb: [*mut nlattr; TCA_CTINFO_MAX as usize + 1] = core::mem::zeroed();
    let mut cp_new: *mut tcf_ctinfo_params;
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let actparm: *mut tc_ctinfo;
    let ci: *mut tcf_ctinfo;
    let mut dscpmaskshift: u8 = 0;
    let mut ret: i32 = 0;
    let mut err: i32;

    if nla.is_null() { NL_SET_ERR_MSG_MOD(extack, "ctinfo requires attributes to be passed"); return -EINVAL; }
    err = nla_parse_nested(tb.as_mut_ptr(), TCA_CTINFO_MAX, nla, &ctinfo_policy, extack);
    if err < 0 { return err; }
    if tb[TCA_CTINFO_ACT as usize].is_null() {
        NL_SET_ERR_MSG_MOD(extack, "Missing required TCA_CTINFO_ACT attribute"); return -EINVAL;
    }
    actparm = nla_data(tb[TCA_CTINFO_ACT as usize]);
    if !tb[TCA_CTINFO_PARMS_DSCP_MASK as usize].is_null() {
        dscpmask = nla_get_u32(tb[TCA_CTINFO_PARMS_DSCP_MASK as usize]);
        dscpmaskshift = if dscpmask != 0 { __ffs(dscpmask) as u8 } else { 0 };
        if ((!0u32 & (dscpmask >> dscpmaskshift)) != 0x3f) {
            NL_SET_ERR_MSG_ATTR(extack, tb[TCA_CTINFO_PARMS_DSCP_MASK as usize], "dscp mask must be 6 contiguous bits"); return -EINVAL;
        }
        dscpstatemask = nla_get_u32_default(tb[TCA_CTINFO_PARMS_DSCP_STATEMASK as usize], 0);
        if (dscpmask & dscpstatemask) != 0 {
            NL_SET_ERR_MSG_ATTR(extack, tb[TCA_CTINFO_PARMS_DSCP_STATEMASK as usize], "dscp statemask must not overlap dscp mask"); return -EINVAL;
        }
    }
    index = (*actparm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err == 0 {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &mut ACT_CTINFO_OPS, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if err > 0 {
        if bind { return ACT_P_BOUND; }
        if (flags & TCA_ACT_FLAGS_REPLACE) == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    } else { return err; }
    err = tcf_action_check_ctrlact((*actparm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); return err; }
    ci = to_ctinfo(*a);
    cp_new = kzalloc_obj();
    if cp_new.is_null() { err = -ENOMEM; if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } tcf_idr_release(*a, bind); return err; }
    (*cp_new).net = net;
    (*cp_new).zone = nla_get_u16_default(tb[TCA_CTINFO_ZONE as usize], 0);
    if dscpmask != 0 { (*cp_new).dscpmask = dscpmask; (*cp_new).dscpmaskshift = dscpmaskshift; (*cp_new).dscpstatemask = dscpstatemask; (*cp_new).mode |= CTINFO_MODE_DSCP; }
    if !tb[TCA_CTINFO_PARMS_CPMARK_MASK as usize].is_null() { (*cp_new).cpmarkmask = nla_get_u32(tb[TCA_CTINFO_PARMS_CPMARK_MASK as usize]); (*cp_new).mode |= CTINFO_MODE_CPMARK; }
    (*cp_new).action = (*actparm).action;
    spin_lock_bh(&mut (*ci).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*actparm).action, goto_ch);
    cp_new = rcu_replace_pointer((*ci).params, cp_new, lockdep_is_held(&(*ci).tcf_lock));
    spin_unlock_bh(&mut (*ci).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if !cp_new.is_null() { kfree_rcu(cp_new, rcu); }
    ret
}

unsafe fn tcf_ctinfo_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let ci = to_ctinfo(a);
    let b = skb_tail_pointer(skb);
    let cp: *const tcf_ctinfo_params;
    let mut opt: tc_ctinfo = core::mem::zeroed();
    let mut t: tcf_t = core::mem::zeroed();

    (*(&mut opt)).index = (*ci).tcf_index;
    (*(&mut opt)).refcnt = refcount_read(&(*ci).tcf_refcnt) - ref_;
    (*(&mut opt)).bindcnt = atomic_read(&(*ci).tcf_bindcnt) - bind;
    rcu_read_lock();
    cp = rcu_dereference((*ci).params);
    tcf_tm_dump(&mut t, &(*ci).tcf_tm);
    if nla_put_64bit(skb, TCA_CTINFO_TM, core::mem::size_of::<tcf_t>(), &t, TCA_CTINFO_PAD) != 0 {
        rcu_read_unlock(); nlmsg_trim(skb, b); return -1;
    }
    (*(&mut opt)).action = (*cp).action;
    if nla_put(skb, TCA_CTINFO_ACT, core::mem::size_of::<tc_ctinfo>(), &opt) != 0 {
        rcu_read_unlock(); nlmsg_trim(skb, b); return -1;
    }
    if nla_put_u16(skb, TCA_CTINFO_ZONE, (*cp).zone) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    if ((*cp).mode & CTINFO_MODE_DSCP) != 0 {
        if nla_put_u32(skb, TCA_CTINFO_PARMS_DSCP_MASK, (*cp).dscpmask) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
        if nla_put_u32(skb, TCA_CTINFO_PARMS_DSCP_STATEMASK, (*cp).dscpstatemask) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    }
    if ((*cp).mode & CTINFO_MODE_CPMARK) != 0 && nla_put_u32(skb, TCA_CTINFO_PARMS_CPMARK_MASK, (*cp).cpmarkmask) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    if nla_put_u64_64bit(skb, TCA_CTINFO_STATS_DSCP_SET, atomic64_read(&(*ci).stats_dscp_set), TCA_CTINFO_PAD) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    if nla_put_u64_64bit(skb, TCA_CTINFO_STATS_DSCP_ERROR, atomic64_read(&(*ci).stats_dscp_error), TCA_CTINFO_PAD) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    if nla_put_u64_64bit(skb, TCA_CTINFO_STATS_CPMARK_SET, atomic64_read(&(*ci).stats_cpmark_set), TCA_CTINFO_PAD) != 0 { rcu_read_unlock(); nlmsg_trim(skb, b); return -1; }
    rcu_read_unlock(); (*skb).len
}

unsafe fn tcf_ctinfo_cleanup(a: *mut tc_action) {
    let ci = to_ctinfo(a);
    let cp = rcu_dereference_protected((*ci).params, 1);
    if !cp.is_null() { kfree_rcu(cp, rcu); }
}

unsafe fn tcf_ctinfo_get_fill_size(act: *const tc_action) -> usize {
    let _ = act;
    nla_total_size(core::mem::size_of::<tc_ctinfo>()) + nla_total_size(core::mem::size_of::<u16>())
        + 3 * nla_total_size(core::mem::size_of::<u32>())
        + 3 * nla_total_size_64bit(core::mem::size_of::<u64>())
}

static mut ACT_CTINFO_OPS_INIT: tc_action_ops = tc_action_ops {
    kind: "ctinfo", id: TCA_ID_CTINFO, owner: THIS_MODULE, act: tcf_ctinfo_act,
    dump: tcf_ctinfo_dump, init: tcf_ctinfo_init, cleanup: tcf_ctinfo_cleanup,
    get_fill_size: tcf_ctinfo_get_fill_size, size: core::mem::size_of::<tcf_ctinfo>(),
};

unsafe fn ctinfo_init_net(net: *mut net) -> i32 {
    let tn = net_generic(net, ACT_CTINFO_OPS_INIT.net_id);
    tc_action_net_init(net, tn, &mut ACT_CTINFO_OPS_INIT)
}

unsafe fn ctinfo_exit_net(net_list: *mut list_head) {
    tc_action_net_exit(net_list, ACT_CTINFO_OPS_INIT.net_id);
}

static mut CTINFO_NET_OPS: pernet_operations = pernet_operations {
    init: ctinfo_init_net, exit_batch: ctinfo_exit_net,
    id: &mut ACT_CTINFO_OPS_INIT.net_id, size: core::mem::size_of::<tc_action_net>(),
};

unsafe fn ctinfo_init_module() -> i32 {
    tcf_register_action(&mut ACT_CTINFO_OPS_INIT, &mut CTINFO_NET_OPS)
}

unsafe fn ctinfo_cleanup_module() {
    tcf_unregister_action(&mut ACT_CTINFO_OPS_INIT, &mut CTINFO_NET_OPS);
}

// module_init(ctinfo_init_module);
// module_exit(ctinfo_cleanup_module);
// MODULE_ALIAS_NET_ACT("ctinfo");
// MODULE_AUTHOR("Kevin Darbyshire-Bryant <ldir@darbyshire-bryant.me.uk>");
// MODULE_DESCRIPTION("Connection tracking mark actions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
