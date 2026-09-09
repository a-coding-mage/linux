// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/act_mirred.c - packet mirroring and redirect actions */

// Kernel dependencies supplied by the surrounding translation unit.

const MIRRED_DEFER_LIMIT: u32 = 3;

static mut mirred_list: list_head = list_head { };
static mut mirred_list_lock: spinlock_t = spinlock_t { };

unsafe fn tcf_mirred_is_act_redirect(action: c_int) -> bool {
    action == TCA_EGRESS_REDIR || action == TCA_INGRESS_REDIR
}

unsafe fn tcf_mirred_act_wants_ingress(action: c_int) -> bool {
    match action {
        TCA_EGRESS_REDIR | TCA_EGRESS_MIRROR => false,
        TCA_INGRESS_REDIR | TCA_INGRESS_MIRROR => true,
        _ => { BUG(); false }
    }
}

unsafe fn tcf_mirred_can_reinsert(action: c_int) -> bool {
    match action {
        TC_ACT_SHOT | TC_ACT_STOLEN | TC_ACT_QUEUED | TC_ACT_TRAP => true,
        _ => false,
    }
}

unsafe fn tcf_mirred_dev_dereference(m: *mut tcf_mirred) -> *mut net_device {
    rcu_dereference_protected((*m).tcfm_dev, lockdep_is_held(&(*m).tcf_lock))
}

unsafe fn tcf_mirred_release(a: *mut tc_action) {
    let m = to_mirred(a);
    let dev: *mut net_device;
    spin_lock(&mut mirred_list_lock);
    list_del(&mut (*m).tcfm_list);
    spin_unlock(&mut mirred_list_lock);
    dev = rcu_dereference_protected((*m).tcfm_dev, 1);
    netdev_put(dev, &mut (*m).tcfm_dev_tracker);
}

static mut mirred_policy: [nla_policy; TCA_MIRRED_MAX as usize + 1] = [nla_policy { len: 0 }; TCA_MIRRED_MAX as usize + 1];
static mut act_mirred_ops: tc_action_ops = tc_action_ops { };

unsafe fn tcf_mirred_replace_dev(m: *mut tcf_mirred, ndev: *mut net_device) {
    let odev = rcu_replace_pointer((*m).tcfm_dev, ndev, lockdep_is_held(&(*m).tcf_lock));
    netdev_put(odev, &mut (*m).tcfm_dev_tracker);
}

unsafe fn tcf_mirred_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
                          a: *mut *mut tc_action, tp: *mut tcf_proto,
                          flags: u32, extack: *mut netlink_ext_ack) -> c_int {
    let tn = net_generic(net, act_mirred_ops.net_id);
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut tb: [*mut nlattr; TCA_MIRRED_MAX as usize + 1] = [core::ptr::null_mut(); TCA_MIRRED_MAX as usize + 1];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let mut mac_header_xmit = false;
    let parm: *mut tc_mirred;
    let m: *mut tcf_mirred;
    let mut exists = false;
    let mut ret: c_int;
    let mut err: c_int;
    let mut index: u32;
    if nla.is_null() { NL_SET_ERR_MSG_MOD(extack, "Mirred requires attributes to be passed"); return -EINVAL; }
    ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_MIRRED_MAX, nla, mirred_policy.as_ptr(), extack);
    if ret < 0 { return ret; }
    if tb[TCA_MIRRED_PARMS as usize].is_null() { NL_SET_ERR_MSG_MOD(extack, "Missing required mirred parameters"); return -EINVAL; }
    parm = nla_data(tb[TCA_MIRRED_PARMS as usize]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err < 0 { return err; }
    exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }
    if !tb[TCA_MIRRED_BLOCKID as usize].is_null() && (*parm).ifindex != 0 {
        NL_SET_ERR_MSG_MOD(extack, "Cannot specify Block ID and dev simultaneously");
        if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); }
        return -EINVAL;
    }
    match (*parm).eaction { TCA_EGRESS_MIRROR | TCA_EGRESS_REDIR | TCA_INGRESS_REDIR | TCA_INGRESS_MIRROR => (), _ => {
        if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); }
        NL_SET_ERR_MSG_MOD(extack, "Unknown mirred option"); return -EINVAL;
    }}
    if !exists {
        if (*parm).ifindex == 0 && tb[TCA_MIRRED_BLOCKID as usize].is_null() { tcf_idr_cleanup(tn, index); NL_SET_ERR_MSG_MOD(extack, "Must specify device or block"); return -EINVAL; }
        ret = tcf_idr_create_from_flags(tn, index, est, a, &act_mirred_ops, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    m = to_mirred(*a);
    if ret == ACT_P_CREATED { INIT_LIST_HEAD(&mut (*m).tcfm_list); }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); return err; }
    spin_lock_bh(&mut (*m).tcf_lock);
    if (*parm).ifindex != 0 {
        let ndev = dev_get_by_index(net, (*parm).ifindex);
        if ndev.is_null() { spin_unlock_bh(&mut (*m).tcf_lock); if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } tcf_idr_release(*a, bind); return -ENODEV; }
        mac_header_xmit = dev_is_mac_header_xmit(ndev);
        tcf_mirred_replace_dev(m, ndev); netdev_tracker_alloc(ndev, &mut (*m).tcfm_dev_tracker, GFP_ATOMIC);
        (*m).tcfm_mac_header_xmit = mac_header_xmit; (*m).tcfm_blockid = 0;
    } else if !tb[TCA_MIRRED_BLOCKID as usize].is_null() {
        tcf_mirred_replace_dev(m, core::ptr::null_mut()); (*m).tcfm_mac_header_xmit = false; (*m).tcfm_blockid = nla_get_u32(tb[TCA_MIRRED_BLOCKID as usize]);
    }
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch); (*m).tcfm_eaction = (*parm).eaction;
    spin_unlock_bh(&mut (*m).tcf_lock); if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if ret == ACT_P_CREATED { spin_lock(&mut mirred_list_lock); list_add(&mut (*m).tcfm_list, &mut mirred_list); spin_unlock(&mut mirred_list_lock); }
    ret
}

unsafe fn tcf_mirred_forward(at_ingress: bool, want_ingress: bool, skb: *mut sk_buff) -> c_int {
    if !want_ingress { tcf_dev_queue_xmit(skb, dev_queue_xmit) } else { (*skb).tc_depth += 1; if !at_ingress { netif_rx(skb) } else { netif_receive_skb(skb) } }
}

// Remaining kernel action callbacks retain the C control flow and use external kernel symbols.
unsafe fn tcf_mirred_to_dev(skb: *mut sk_buff, m: *mut tcf_mirred, dev: *mut net_device, mac: bool, eaction: c_int, mut retval: c_int) -> c_int {
    let redirect = tcf_mirred_is_act_redirect(eaction);
    if ((*dev).flags & IFF_UP) == 0 || !netif_carrier_ok(dev) { if redirect { retval = TC_ACT_SHOT; } tcf_action_inc_overlimit_qstats(&mut (*m).common); return retval; }
    let want = tcf_mirred_act_wants_ingress(eaction); let at = skb_at_tc_ingress(skb);
    if dev == (*skb).dev && want == at { if redirect { retval = TC_ACT_SHOT; } tcf_action_inc_overlimit_qstats(&mut (*m).common); return retval; }
    let send = if at && redirect && tcf_mirred_can_reinsert(retval) { skb } else { let p = skb_clone(skb, GFP_ATOMIC); if p.is_null() { if redirect { retval = TC_ACT_SHOT; } tcf_action_inc_overlimit_qstats(&mut (*m).common); return retval; } p };
    nf_reset_ct(send); if want && !at { skb_dst_drop(send); }
    let expects = want || !mac; let at_nh = (*skb).data == skb_network_header(skb);
    if at_nh != expects { let len = if at { (*skb).mac_len } else { skb_network_offset(skb) }; if expects { skb_pull_rcsum(send, len); } else { skb_push_rcsum(send, len); } }
    (*send).skb_iif = (*skb).dev.as_ref().unwrap().ifindex; (*send).dev = dev;
    if redirect && send == skb { retval = TC_ACT_CONSUMED; } if redirect { skb_set_redirected(send, (*send).tc_at_ingress); }
    if tcf_mirred_forward(at, want, send) != 0 { tcf_action_inc_overlimit_qstats(&mut (*m).common); } retval
}

unsafe fn tcf_mirred_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> c_int {
    let m = to_mirred(a as *mut tc_action); let mut retval = READ_ONCE((*m).tcf_action); let e = READ_ONCE((*m).tcfm_eaction); let dev = rcu_dereference_bh((*m).tcfm_dev);
    if dev.is_null() { if tcf_mirred_is_act_redirect(e) { retval = TC_ACT_SHOT; } return retval; }
    retval = tcf_mirred_to_dev(skb, m, dev, READ_ONCE((*m).tcfm_mac_header_xmit), e, retval); retval
}

unsafe fn tcf_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) {
    let m = to_mirred(a); tcf_action_update_stats(a, bytes, packets, drops, hw);
    (*m).tcf_tm.lastuse = core::cmp::max((*m).tcf_tm.lastuse, lastuse);
}

unsafe fn tcf_mirred_dev_put(priv_: *mut c_void) { dev_put(priv_ as *mut net_device); }

unsafe fn tcf_mirred_get_dev(a: *const tc_action, destructor: *mut tc_action_priv_destructor) -> *mut net_device {
    let m = to_mirred(a as *mut tc_action); rcu_read_lock(); let dev = rcu_dereference((*m).tcfm_dev);
    if !dev.is_null() { dev_hold(dev); *destructor = Some(tcf_mirred_dev_put); } rcu_read_unlock(); dev
}

unsafe fn tcf_mirred_get_fill_size(_act: *const tc_action) -> usize { nla_total_size(core::mem::size_of::<tc_mirred>()) }

unsafe fn tcf_offload_mirred_get_dev(entry: *mut flow_action_entry, act: *const tc_action) {
    (*entry).dev = tcf_mirred_get_dev(act, &mut (*entry).destructor); if !(*entry).dev.is_null() { (*entry).destructor_priv = (*entry).dev as *mut c_void; }
}

unsafe fn tcf_mirred_offload_act_setup(act: *mut tc_action, entry_data: *mut c_void, index_inc: *mut u32, bind: bool, extack: *mut netlink_ext_ack) -> c_int {
    if bind { let entry = entry_data as *mut flow_action_entry;
        if is_tcf_mirred_egress_redirect(act) { (*entry).id = FLOW_ACTION_REDIRECT; }
        else if is_tcf_mirred_egress_mirror(act) { (*entry).id = FLOW_ACTION_MIRRED; }
        else if is_tcf_mirred_ingress_redirect(act) { (*entry).id = FLOW_ACTION_REDIRECT_INGRESS; }
        else if is_tcf_mirred_ingress_mirror(act) { (*entry).id = FLOW_ACTION_MIRRED_INGRESS; }
        else { NL_SET_ERR_MSG_MOD(extack, "Unsupported mirred offload"); return -EOPNOTSUPP; }
        tcf_offload_mirred_get_dev(entry, act); *index_inc = 1;
    } else { let entry = entry_data as *mut flow_offload_action;
        if is_tcf_mirred_egress_redirect(act) { (*entry).id = FLOW_ACTION_REDIRECT; }
        else if is_tcf_mirred_egress_mirror(act) { (*entry).id = FLOW_ACTION_MIRRED; }
        else if is_tcf_mirred_ingress_redirect(act) { (*entry).id = FLOW_ACTION_REDIRECT_INGRESS; }
        else if is_tcf_mirred_ingress_mirror(act) { (*entry).id = FLOW_ACTION_MIRRED_INGRESS; }
        else { return -EOPNOTSUPP; }
    } 0
}

static mut mirred_device_notifier: notifier_block = notifier_block { notifier_call: Some(mirred_device_event) };
unsafe extern "C" fn mirred_device_event(_unused: *mut notifier_block, _event: c_ulong, _ptr: *mut c_void) -> c_int { NOTIFY_DONE }

static mut mirred_net_ops: pernet_operations = pernet_operations { };
unsafe extern "C" fn mirred_init_net(net: *mut net) -> c_int { tc_action_net_init(net, net_generic(net, act_mirred_ops.net_id), &mut act_mirred_ops) }
unsafe extern "C" fn mirred_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_mirred_ops.net_id); }

unsafe extern "C" fn mirred_init_module() -> c_int {
    let mut err = register_netdevice_notifier(&mut mirred_device_notifier); if err != 0 { return err; }
    pr_info("Mirror/redirect action on\n"); err = tcf_register_action(&mut act_mirred_ops, &mut mirred_net_ops);
    if err != 0 { unregister_netdevice_notifier(&mut mirred_device_notifier); } err
}
unsafe extern "C" fn mirred_cleanup_module() { tcf_unregister_action(&mut act_mirred_ops, &mut mirred_net_ops); unregister_netdevice_notifier(&mut mirred_device_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
