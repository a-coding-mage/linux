// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2014 Jiri Pirko <jiri@resnulli.us>
 */

// Kernel headers and build-time configuration supplied by the surrounding tree.

static mut act_vlan_ops: tc_action_ops = tc_action_ops::default();

unsafe fn tcf_vlan_act(skb: *mut sk_buff, a: *const tc_action,
                       res: *mut tcf_result) -> i32 {
    let v = to_vlan(a);
    let mut p: *mut tcf_vlan_params;
    let mut err: i32;
    let mut tci: u16;

    tcf_lastuse_update(&mut (*v).tcf_tm);
    tcf_action_update_bstats(&mut (*v).common, skb);

    /* Ensure 'data' points at mac_header prior calling vlan manipulating
     * functions.
     */
    if skb_at_tc_ingress(skb) {
        skb_push_rcsum(skb, (*skb).mac_len);
    }

    p = rcu_dereference_bh((*v).vlan_p);

    match (*p).tcfv_action {
        TCA_VLAN_ACT_POP => {
            err = skb_vlan_pop(skb);
            if err != 0 { goto drop; }
        }
        TCA_VLAN_ACT_PUSH => {
            err = skb_vlan_push(skb, (*p).tcfv_push_proto,
                (*p).tcfv_push_vid | ((*p).tcfv_push_prio << VLAN_PRIO_SHIFT));
            if err != 0 { goto drop; }
        }
        TCA_VLAN_ACT_MODIFY => {
            /* No-op if no vlan tag (either hw-accel or in-payload) */
            if !skb_vlan_tagged(skb) { goto out; }
            /* extract existing tag (and guarantee no hw-accel tag) */
            if skb_vlan_tag_present(skb) {
                tci = skb_vlan_tag_get(skb);
                __vlan_hwaccel_clear_tag(skb);
            } else {
                /* in-payload vlan tag, pop it */
                err = __skb_vlan_pop(skb, &mut tci);
                if err != 0 { goto drop; }
            }
            /* replace the vid */
            tci = (tci & !VLAN_VID_MASK) | (*p).tcfv_push_vid;
            /* replace prio bits, if tcfv_push_prio specified */
            if (*p).tcfv_push_prio_exists {
                tci &= !VLAN_PRIO_MASK;
                tci |= (*p).tcfv_push_prio << VLAN_PRIO_SHIFT;
            }
            /* put updated tci as hwaccel tag */
            __vlan_hwaccel_put_tag(skb, (*p).tcfv_push_proto, tci);
        }
        TCA_VLAN_ACT_POP_ETH => {
            err = skb_eth_pop(skb);
            if err != 0 { goto drop; }
        }
        TCA_VLAN_ACT_PUSH_ETH => {
            err = skb_eth_push(skb, (*p).tcfv_push_dst, (*p).tcfv_push_src);
            if err != 0 { goto drop; }
        }
        _ => BUG!(),
    }

out:
    if skb_at_tc_ingress(skb) {
        skb_pull_rcsum(skb, (*skb).mac_len);
    }
    skb_reset_mac_len(skb);
    return (*p).action;

drop:
    tcf_action_inc_drop_qstats(&mut (*v).common);
    TC_ACT_SHOT
}

static vlan_policy: [nla_policy; TCA_VLAN_MAX + 1] = [
    nla_policy::strict_start_type(TCA_VLAN_PUSH_ETH_DST),
    nla_policy::len(TCA_VLAN_PARMS, size_of::<tc_vlan>()),
    nla_policy::kind(TCA_VLAN_PUSH_VLAN_ID, NLA_U16),
    nla_policy::kind(TCA_VLAN_PUSH_VLAN_PROTOCOL, NLA_U16),
    nla_policy::kind(TCA_VLAN_PUSH_VLAN_PRIORITY, NLA_U8),
    nla_policy::eth_addr(TCA_VLAN_PUSH_ETH_DST),
    nla_policy::eth_addr(TCA_VLAN_PUSH_ETH_SRC),
];

unsafe fn tcf_vlan_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
                        a: *mut *mut tc_action, tp: *mut tcf_proto,
                        flags: u32, extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic(net, (*(&raw const act_vlan_ops)).net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut tb: [*mut nlattr; TCA_VLAN_MAX + 1] = [core::ptr::null_mut(); TCA_VLAN_MAX + 1];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut push_prio_exists = false;
    let mut p: *mut tcf_vlan_params;
    let mut parm: *mut tc_vlan;
    let mut v: *mut tcf_vlan;
    let mut action: i32;
    let mut push_vid: u16 = 0;
    let mut push_proto: __be16 = 0;
    let mut push_prio: u8 = 0;
    let mut exists = false;
    let mut ret: i32 = 0;
    let mut err: i32;
    let mut index: u32;

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_VLAN_MAX, nla, &vlan_policy, core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_VLAN_PARMS].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_VLAN_PARMS]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err < 0 { return err; }
    exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }

    match (*parm).v_action {
        TCA_VLAN_ACT_POP | TCA_VLAN_ACT_POP_ETH => {}
        TCA_VLAN_ACT_PUSH | TCA_VLAN_ACT_MODIFY => {
            if tb[TCA_VLAN_PUSH_VLAN_ID].is_null() { goto invalid; }
            push_vid = nla_get_u16(tb[TCA_VLAN_PUSH_VLAN_ID]);
            if push_vid >= VLAN_VID_MASK { goto range; }
            if !tb[TCA_VLAN_PUSH_VLAN_PROTOCOL].is_null() {
                push_proto = nla_get_be16(tb[TCA_VLAN_PUSH_VLAN_PROTOCOL]);
                if push_proto != htons(ETH_P_8021Q) && push_proto != htons(ETH_P_8021AD) { goto proto; }
            } else { push_proto = htons(ETH_P_8021Q); }
            push_prio_exists = !tb[TCA_VLAN_PUSH_VLAN_PRIORITY].is_null();
            if push_prio_exists { push_prio = nla_get_u8(tb[TCA_VLAN_PUSH_VLAN_PRIORITY]); }
        }
        TCA_VLAN_ACT_PUSH_ETH => {
            if tb[TCA_VLAN_PUSH_ETH_DST].is_null() || tb[TCA_VLAN_PUSH_ETH_SRC].is_null() { goto invalid; }
        }
        _ => goto invalid,
    }
    action = (*parm).v_action;
    if !exists {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &raw const act_vlan_ops, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if flags & TCA_ACT_FLAGS_REPLACE == 0 {
        tcf_idr_release(*a, bind); return -EEXIST;
    }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { goto release_idr; }
    v = to_vlan(*a);
    p = kzalloc_obj::<tcf_vlan_params>();
    if p.is_null() { err = -ENOMEM; goto put_chain; }
    (*p).tcfv_action = action;
    (*p).tcfv_push_vid = push_vid;
    (*p).tcfv_push_prio = push_prio;
    (*p).tcfv_push_prio_exists = push_prio_exists || action == TCA_VLAN_ACT_PUSH;
    (*p).tcfv_push_proto = push_proto;
    if action == TCA_VLAN_ACT_PUSH_ETH {
        nla_memcpy((*p).tcfv_push_dst.as_mut_ptr(), tb[TCA_VLAN_PUSH_ETH_DST], ETH_ALEN);
        nla_memcpy((*p).tcfv_push_src.as_mut_ptr(), tb[TCA_VLAN_PUSH_ETH_SRC], ETH_ALEN);
    }
    (*p).action = (*parm).action;
    spin_lock_bh(&mut (*v).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    p = rcu_replace_pointer(&mut (*v).vlan_p, p, lockdep_is_held(&(*v).tcf_lock));
    spin_unlock_bh(&mut (*v).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if !p.is_null() { kfree_rcu(p, rcu); }
    return ret;
put_chain:
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
release_idr:
    tcf_idr_release(*a, bind); return err;
invalid:
    if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); } return -EINVAL;
range:
    if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); } return -ERANGE;
proto:
    if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); } return -EPROTONOSUPPORT;
}

unsafe fn tcf_vlan_cleanup(a: *mut tc_action) {
    let v = to_vlan(a);
    let p = rcu_dereference_protected((*v).vlan_p, 1);
    if !p.is_null() { kfree_rcu(p, rcu); }
}

unsafe fn tcf_vlan_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let b = skb_tail_pointer(skb);
    let v = to_vlan(a);
    let p;
    let mut opt: tc_vlan = core::mem::zeroed();
    opt.index = (*v).tcf_index;
    opt.refcnt = refcount_read(&(*v).tcf_refcnt) - ref_;
    opt.bindcnt = atomic_read(&(*v).tcf_bindcnt) - bind;
    let mut t: tcf_t = core::mem::zeroed();
    rcu_read_lock();
    p = rcu_dereference((*v).vlan_p);
    opt.action = (*p).action; opt.v_action = (*p).tcfv_action;
    if nla_put(skb, TCA_VLAN_PARMS, size_of::<tc_vlan>(), &opt) { goto nla_put_failure; }
    if ((*p).tcfv_action == TCA_VLAN_ACT_PUSH || (*p).tcfv_action == TCA_VLAN_ACT_MODIFY) &&
       (nla_put_u16(skb, TCA_VLAN_PUSH_VLAN_ID, (*p).tcfv_push_vid) ||
        nla_put_be16(skb, TCA_VLAN_PUSH_VLAN_PROTOCOL, (*p).tcfv_push_proto) ||
        ((*p).tcfv_push_prio_exists && nla_put_u8(skb, TCA_VLAN_PUSH_VLAN_PRIORITY, (*p).tcfv_push_prio))) { goto nla_put_failure; }
    if (*p).tcfv_action == TCA_VLAN_ACT_PUSH_ETH {
        if nla_put(skb, TCA_VLAN_PUSH_ETH_DST, ETH_ALEN, (*p).tcfv_push_dst.as_ptr()) { goto nla_put_failure; }
        if nla_put(skb, TCA_VLAN_PUSH_ETH_SRC, ETH_ALEN, (*p).tcfv_push_src.as_ptr()) { goto nla_put_failure; }
    }
    tcf_tm_dump(&mut t, &(*v).tcf_tm);
    if nla_put_64bit(skb, TCA_VLAN_TM, size_of::<tcf_t>(), &t, TCA_VLAN_PAD) { goto nla_put_failure; }
    rcu_read_unlock(); return (*skb).len;
nla_put_failure:
    rcu_read_unlock(); nlmsg_trim(skb, b); -1
}

unsafe fn tcf_vlan_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) {
    let v = to_vlan(a); let tm = &mut (*v).tcf_tm;
    tcf_action_update_stats(a, bytes, packets, drops, hw);
    tm.lastuse = core::cmp::max(tm.lastuse, lastuse);
}

unsafe fn tcf_vlan_get_fill_size(_act: *const tc_action) -> usize {
    nla_total_size(size_of::<tc_vlan>()) + nla_total_size(size_of::<u16>()) + nla_total_size(size_of::<u16>()) + nla_total_size(size_of::<u8>())
}

unsafe fn tcf_vlan_offload_act_setup(act: *mut tc_action, entry_data: *mut core::ffi::c_void, index_inc: *mut u32, bind: bool, extack: *mut netlink_ext_ack) -> i32 {
    if bind {
        let entry = entry_data as *mut flow_action_entry;
        match tcf_vlan_action(act) {
            TCA_VLAN_ACT_PUSH => { (*entry).id = FLOW_ACTION_VLAN_PUSH; (*entry).vlan.vid = tcf_vlan_push_vid(act); (*entry).vlan.proto = tcf_vlan_push_proto(act); (*entry).vlan.prio = tcf_vlan_push_prio(act); }
            TCA_VLAN_ACT_POP => (*entry).id = FLOW_ACTION_VLAN_POP,
            TCA_VLAN_ACT_MODIFY => { (*entry).id = FLOW_ACTION_VLAN_MANGLE; (*entry).vlan.vid = tcf_vlan_push_vid(act); (*entry).vlan.proto = tcf_vlan_push_proto(act); (*entry).vlan.prio = tcf_vlan_push_prio(act); }
            TCA_VLAN_ACT_POP_ETH => (*entry).id = FLOW_ACTION_VLAN_POP_ETH,
            TCA_VLAN_ACT_PUSH_ETH => { (*entry).id = FLOW_ACTION_VLAN_PUSH_ETH; tcf_vlan_push_eth((*entry).vlan_push_eth.src.as_mut_ptr(), (*entry).vlan_push_eth.dst.as_mut_ptr(), act); }
            _ => { NL_SET_ERR_MSG_MOD(extack, "Unsupported vlan action mode offload"); return -EOPNOTSUPP; }
        }
        *index_inc = 1;
    } else {
        let fl_action = entry_data as *mut flow_offload_action;
        (*fl_action).id = match tcf_vlan_action(act) { TCA_VLAN_ACT_PUSH => FLOW_ACTION_VLAN_PUSH, TCA_VLAN_ACT_POP => FLOW_ACTION_VLAN_POP, TCA_VLAN_ACT_MODIFY => FLOW_ACTION_VLAN_MANGLE, TCA_VLAN_ACT_POP_ETH => FLOW_ACTION_VLAN_POP_ETH, TCA_VLAN_ACT_PUSH_ETH => FLOW_ACTION_VLAN_PUSH_ETH, _ => return -EOPNOTSUPP };
    }
    0
}

static mut act_vlan_ops: tc_action_ops = tc_action_ops {
    kind: "vlan", id: TCA_ID_VLAN, owner: THIS_MODULE, act: Some(tcf_vlan_act), dump: Some(tcf_vlan_dump), init: Some(tcf_vlan_init), cleanup: Some(tcf_vlan_cleanup), stats_update: Some(tcf_vlan_stats_update), get_fill_size: Some(tcf_vlan_get_fill_size), offload_act_setup: Some(tcf_vlan_offload_act_setup), size: size_of::<tcf_vlan>(), ..tc_action_ops::default()
};

unsafe fn vlan_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, act_vlan_ops.net_id), &act_vlan_ops) }
unsafe fn vlan_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_vlan_ops.net_id); }
static mut vlan_net_ops: pernet_operations = pernet_operations { init: Some(vlan_init_net), exit_batch: Some(vlan_exit_net), id: &raw mut act_vlan_ops.net_id, size: size_of::<tc_action_net>(), ..pernet_operations::default() };
unsafe fn vlan_init_module() -> i32 { tcf_register_action(&act_vlan_ops, &vlan_net_ops) }
unsafe fn vlan_cleanup_module() { tcf_unregister_action(&act_vlan_ops, &vlan_net_ops); }

// module_init(vlan_init_module);
// module_exit(vlan_cleanup_module);
// MODULE_ALIAS_NET_ACT("vlan");
// MODULE_AUTHOR("Jiri Pirko <jiri@resnulli.us>");
// MODULE_DESCRIPTION("vlan manipulation actions");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
