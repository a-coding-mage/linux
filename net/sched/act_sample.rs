// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/sched/act_sample.c - Packet sampling tc action
 * Copyright (c) 2017 Yotam Gigi <yotamg@mellanox.com>
 *
 * Direct Rust translation. Kernel-provided types, constants, macros, and
 * functions referenced below are supplied by the surrounding kernel bindings.
 */

static sample_policy: [nla_policy; TCA_SAMPLE_MAX as usize + 1] = [nla_policy { }; TCA_SAMPLE_MAX as usize + 1];

unsafe fn tcf_sample_init(
    net: *mut net,
    nla: *mut nlattr,
    est: *mut nlattr,
    a: *mut *mut tc_action,
    tp: *mut tcf_proto,
    flags: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let tn = net_generic(net, (*act_sample_ops).net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut tb: [*mut nlattr; TCA_SAMPLE_MAX as usize + 1] = [core::ptr::null_mut(); TCA_SAMPLE_MAX as usize + 1];
    let mut psample_group: *mut psample_group;
    let mut psample_group_num: u32;
    let mut rate: u32;
    let mut index: u32;
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_sample;
    let s: *mut tcf_sample;
    let mut exists = false;
    let mut ret: i32;
    let mut err: i32;

    if nla.is_null() { return -EINVAL; }
    ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_SAMPLE_MAX, nla, sample_policy.as_ptr(), core::ptr::null_mut());
    if ret < 0 { return ret; }
    if tb[TCA_SAMPLE_PARMS as usize].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_SAMPLE_PARMS as usize]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err < 0 { return err; }
    exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }
    if !exists {
        ret = tcf_idr_create(tn, index, est, a, &mut act_sample_ops, bind, true, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if (flags & TCA_ACT_FLAGS_REPLACE) == 0 {
        tcf_idr_release(*a, bind); return -EEXIST;
    }
    if tb[TCA_SAMPLE_RATE as usize].is_null() || tb[TCA_SAMPLE_PSAMPLE_GROUP as usize].is_null() {
        NL_SET_ERR_MSG(extack, "sample rate and group are required"); err = -EINVAL; goto release_idr;
    }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { goto release_idr; }
    rate = nla_get_u32(tb[TCA_SAMPLE_RATE as usize]);
    if rate == 0 { NL_SET_ERR_MSG(extack, "invalid sample rate"); err = -EINVAL; goto put_chain; }
    psample_group_num = nla_get_u32(tb[TCA_SAMPLE_PSAMPLE_GROUP as usize]);
    psample_group = psample_group_get(net, psample_group_num);
    if psample_group.is_null() { err = -ENOMEM; goto put_chain; }
    s = to_sample(*a);
    spin_lock_bh(&mut (*s).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    (*s).rate = rate;
    (*s).psample_group_num = psample_group_num;
    psample_group = rcu_replace_pointer(&mut (*s).psample_group, psample_group, lockdep_is_held(&(*s).tcf_lock));
    if !tb[TCA_SAMPLE_TRUNC_SIZE as usize].is_null() { (*s).truncate = true; (*s).trunc_size = nla_get_u32(tb[TCA_SAMPLE_TRUNC_SIZE as usize]); }
    spin_unlock_bh(&mut (*s).tcf_lock);
    if !psample_group.is_null() { psample_group_put(psample_group); }
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    return ret;
put_chain:
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
release_idr:
    tcf_idr_release(*a, bind); return err;
}

unsafe fn tcf_sample_cleanup(a: *mut tc_action) {
    let s = to_sample(a);
    let group = rcu_dereference_protected((*s).psample_group, 1);
    RCU_INIT_POINTER(&mut (*s).psample_group, core::ptr::null_mut());
    if !group.is_null() { psample_group_put(group); }
}

unsafe fn tcf_sample_dev_ok_push(dev: *mut net_device) -> bool {
    match (*dev).type_ { ARPHRD_TUNNEL | ARPHRD_TUNNEL6 | ARPHRD_SIT | ARPHRD_IPGRE | ARPHRD_IP6GRE | ARPHRD_VOID | ARPHRD_NONE => false, _ => true }
}

unsafe fn tcf_sample_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> i32 {
    let s = to_sample(a as *mut tc_action);
    let group = rcu_dereference_bh((*s).psample_group);
    tcf_lastuse_update(&mut (*s).tcf_tm);
    bstats_update(this_cpu_ptr((*s).common.cpu_bstats), skb);
    let retval = READ_ONCE((*s).tcf_action);
    if !group.is_null() && get_random_u32_below((*s).rate) == 0 {
        let mut md: psample_metadata = core::mem::zeroed();
        if !skb_at_tc_ingress(skb) { md.in_ifindex = (*skb).skb_iif; md.out_ifindex = (*(*skb).dev).ifindex; } else { md.in_ifindex = (*(*skb).dev).ifindex; }
        if skb_at_tc_ingress(skb) && tcf_sample_dev_ok_push((*skb).dev) { skb_push(skb, (*skb).mac_len); }
        let mut cookie_data = [0u8; TC_COOKIE_MAX_SIZE as usize];
        rcu_read_lock();
        let user_cookie = rcu_dereference((*a).user_cookie);
        if !user_cookie.is_null() { memcpy(cookie_data.as_mut_ptr(), (*user_cookie).data.as_ptr(), (*user_cookie).len as usize); md.user_cookie = cookie_data.as_mut_ptr(); md.user_cookie_len = (*user_cookie).len; }
        rcu_read_unlock();
        md.trunc_size = if (*s).truncate { (*s).trunc_size } else { (*skb).len };
        psample_sample_packet(group, skb, (*s).rate, &mut md);
        if skb_at_tc_ingress(skb) && tcf_sample_dev_ok_push((*skb).dev) { skb_pull(skb, (*skb).mac_len); }
    }
    retval
}

unsafe fn tcf_sample_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) {
    let s = to_sample(a); tcf_action_update_stats(a, bytes, packets, drops, hw); (*s).tcf_tm.lastuse = core::cmp::max((*s).tcf_tm.lastuse, lastuse);
}

unsafe fn tcf_sample_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let b = skb_tail_pointer(skb);
    let s = to_sample(a);
    let mut opt: tc_sample = core::mem::zeroed();
    opt.index = (*s).tcf_index;
    opt.refcnt = refcount_read(&(*s).tcf_refcnt) - ref_;
    opt.bindcnt = atomic_read(&(*s).tcf_bindcnt) - bind;
    let mut t: tcf_t = core::mem::zeroed();
    spin_lock_bh(&mut (*s).tcf_lock);
    opt.action = (*s).tcf_action;
    if nla_put(skb, TCA_SAMPLE_PARMS, core::mem::size_of::<tc_sample>(), &opt as *const _ as *const core::ffi::c_void) != 0 { goto nla_put_failure; }
    tcf_tm_dump(&mut t, &(*s).tcf_tm);
    if nla_put_64bit(skb, TCA_SAMPLE_TM, core::mem::size_of::<tcf_t>(), &t, TCA_SAMPLE_PAD) != 0 { goto nla_put_failure; }
    if nla_put_u32(skb, TCA_SAMPLE_RATE, (*s).rate) != 0 { goto nla_put_failure; }
    if (*s).truncate && nla_put_u32(skb, TCA_SAMPLE_TRUNC_SIZE, (*s).trunc_size) != 0 { goto nla_put_failure; }
    if nla_put_u32(skb, TCA_SAMPLE_PSAMPLE_GROUP, (*s).psample_group_num) != 0 { goto nla_put_failure; }
    spin_unlock_bh(&mut (*s).tcf_lock); return (*skb).len as i32;
nla_put_failure:
    spin_unlock_bh(&mut (*s).tcf_lock); nlmsg_trim(skb, b); -1
}

unsafe fn tcf_psample_group_put(priv_: *mut core::ffi::c_void) { psample_group_put(priv_ as *mut psample_group); }

unsafe fn tcf_sample_get_group(a: *const tc_action, destructor: *mut tc_action_priv_destructor) -> *mut psample_group {
    let s = to_sample(a as *mut tc_action);
    let group = rcu_dereference_protected((*s).psample_group, lockdep_is_held(&(*s).tcf_lock));
    if !group.is_null() { psample_group_take(group); *destructor = Some(tcf_psample_group_put); }
    group
}

unsafe fn tcf_offload_sample_get_group(entry: *mut flow_action_entry, act: *const tc_action) {
    (*entry).sample.psample_group = tcf_sample_get_group(act, &mut (*entry).destructor);
    (*entry).destructor_priv = (*entry).sample.psample_group as *mut core::ffi::c_void;
}

unsafe fn tcf_sample_offload_act_setup(act: *mut tc_action, entry_data: *mut core::ffi::c_void, index_inc: *mut u32, bind: bool, _extack: *mut netlink_ext_ack) -> i32 {
    if bind {
        let entry = entry_data as *mut flow_action_entry;
        (*entry).id = FLOW_ACTION_SAMPLE;
        (*entry).sample.trunc_size = tcf_sample_trunc_size(act);
        (*entry).sample.truncate = tcf_sample_truncate(act);
        (*entry).sample.rate = tcf_sample_rate(act);
        tcf_offload_sample_get_group(entry, act);
        *index_inc = 1;
    } else { (*(entry_data as *mut flow_offload_action)).id = FLOW_ACTION_SAMPLE; }
    0
}

unsafe fn tcf_sample_get_fill_size(_act: *const tc_action) -> usize {
    nla_total_size(core::mem::size_of::<tc_sample>()) + 3 * nla_total_size(core::mem::size_of::<u32>())
}

static mut act_sample_ops: tc_action_ops = tc_action_ops {
    kind: "sample", id: TCA_ID_SAMPLE, owner: THIS_MODULE, act: Some(tcf_sample_act),
    stats_update: Some(tcf_sample_stats_update), dump: Some(tcf_sample_dump), init: Some(tcf_sample_init),
    cleanup: Some(tcf_sample_cleanup), get_fill_size: Some(tcf_sample_get_fill_size),
    get_psample_group: Some(tcf_sample_get_group), offload_act_setup: Some(tcf_sample_offload_act_setup),
    size: core::mem::size_of::<tcf_sample>(), ..tc_action_ops::zeroed()
};

unsafe fn sample_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, act_sample_ops.net_id), &mut act_sample_ops) }
unsafe fn sample_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_sample_ops.net_id); }
static mut sample_net_ops: pernet_operations = pernet_operations { init: Some(sample_init_net), exit_batch: Some(sample_exit_net), id: &mut act_sample_ops.net_id, size: core::mem::size_of::<tc_action_net>() };
unsafe fn sample_init_module() -> i32 { tcf_register_action(&mut act_sample_ops, &mut sample_net_ops) }
unsafe fn sample_cleanup_module() { tcf_unregister_action(&mut act_sample_ops, &mut sample_net_ops); }

// MODULE_ALIAS_NET_ACT("sample");
// module_init(sample_init_module);
// module_exit(sample_cleanup_module);
// MODULE_AUTHOR("Yotam Gigi <yotam.gi@gmail.com>");
// MODULE_DESCRIPTION("Packet sampling action");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
