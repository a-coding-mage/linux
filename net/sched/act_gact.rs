// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_gact.c        Generic actions
 *
 * copyright     Jamal Hadi Salim (2002-4)
 */

// C kernel includes and build-time configuration are supplied by the surrounding crate.

static mut act_gact_ops: tc_action_ops = tc_action_ops::default();

#[cfg(CONFIG_GACT_PROB)]
unsafe fn gact_net_rand(gact: *mut tcf_gact) -> i32 {
    smp_rmb(); /* coupled with smp_wmb() in tcf_gact_init() */
    if get_random_u32_below((*gact).tcfg_pval) != 0 {
        (*gact).tcf_action
    } else {
        (*gact).tcfg_paction
    }
}

#[cfg(CONFIG_GACT_PROB)]
unsafe fn gact_determ(gact: *mut tcf_gact) -> i32 {
    let pack: u32 = atomic_inc_return(&mut (*gact).packets);

    smp_rmb(); /* coupled with smp_wmb() in tcf_gact_init() */
    if pack % (*gact).tcfg_pval != 0 {
        (*gact).tcf_action
    } else {
        (*gact).tcfg_paction
    }
}

#[cfg(CONFIG_GACT_PROB)]
type g_rand = unsafe fn(*mut tcf_gact) -> i32;
#[cfg(CONFIG_GACT_PROB)]
static mut gact_rand: [Option<g_rand>; MAX_RAND as usize] = [None, Some(gact_net_rand), Some(gact_determ)];

static gact_policy: [nla_policy; (TCA_GACT_MAX + 1) as usize] = [
    /* [TCA_GACT_PARMS] = { .len = sizeof(struct tc_gact) }, */
    /* [TCA_GACT_PROB]  = { .len = sizeof(struct tc_gact_p) }, */
];

unsafe fn tcf_gact_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
                        a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32,
                        extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic(net, (*act_gact_ops).net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut tb: [*mut nlattr; (TCA_GACT_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_GACT_MAX + 1) as usize];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_gact;
    let gact: *mut tcf_gact;
    let mut ret = 0;
    let mut index: u32;
    let mut err: i32;
    #[cfg(CONFIG_GACT_PROB)]
    let mut p_parm: *mut tc_gact_p = core::ptr::null_mut();

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_GACT_MAX, nla, gact_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_GACT_PARMS as usize].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_GACT_PARMS as usize]);
    index = (*parm).index;

    #[cfg(not(CONFIG_GACT_PROB))]
    if !tb[TCA_GACT_PROB as usize].is_null() { return -EOPNOTSUPP; }
    #[cfg(CONFIG_GACT_PROB)]
    if !tb[TCA_GACT_PROB as usize].is_null() {
        p_parm = nla_data(tb[TCA_GACT_PROB as usize]);
        if (*p_parm).ptype >= MAX_RAND { return -EINVAL; }
        if !tcf_action_valid((*p_parm).paction) { NL_SET_ERR_MSG(extack, "invalid fallback control action"); return -EINVAL; }
        if TC_ACT_EXT_CMP((*p_parm).paction, TC_ACT_GOTO_CHAIN) { NL_SET_ERR_MSG(extack, "goto chain not allowed on fallback"); return -EINVAL; }
    }

    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err == 0 {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &act_gact_ops, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if err > 0 {
        if bind { return ACT_P_BOUND; }
        if (flags & TCA_ACT_FLAGS_REPLACE) == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    } else { return err; }

    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); return err; }
    gact = to_gact(*a);
    spin_lock_bh(&mut (*gact).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    #[cfg(CONFIG_GACT_PROB)]
    if !p_parm.is_null() {
        (*gact).tcfg_paction = (*p_parm).paction;
        (*gact).tcfg_pval = core::cmp::max(1u16, (*p_parm).pval);
        smp_wmb();
        (*gact).tcfg_ptype = (*p_parm).ptype;
    }
    spin_unlock_bh(&mut (*gact).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    ret
}

#[inline(never)]
unsafe fn tcf_gact_act(skb: *mut sk_buff, a: *const tc_action, res: *mut tcf_result) -> i32 {
    let gact = to_gact(a);
    let mut action = READ_ONCE((*gact).tcf_action);
    #[cfg(CONFIG_GACT_PROB)]
    { let ptype = READ_ONCE((*gact).tcfg_ptype); if ptype != 0 { action = gact_rand[ptype as usize].unwrap()(gact); } }
    tcf_action_update_bstats(&mut (*gact).common, skb);
    if action == TC_ACT_SHOT { tcf_action_inc_drop_qstats(&mut (*gact).common); }
    tcf_lastuse_update(&mut (*gact).tcf_tm);
    action
}

unsafe fn tcf_gact_stats_update(a: *mut tc_action, bytes: u64, packets: u64, drops: u64, lastuse: u64, hw: bool) {
    let gact = to_gact(a);
    let action = READ_ONCE((*gact).tcf_action);
    tcf_action_update_stats(a, bytes, packets, if action == TC_ACT_SHOT { packets } else { drops }, hw);
    (*gact).tcf_tm.lastuse = core::cmp::max((*gact).tcf_tm.lastuse, lastuse);
}

unsafe fn tcf_gact_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let b = skb_tail_pointer(skb); let gact = to_gact(a);
    let mut opt: tc_gact = core::mem::zeroed(); opt.index = (*gact).tcf_index;
    opt.refcnt = refcount_read(&(*gact).tcf_refcnt) - ref_; opt.bindcnt = atomic_read(&(*gact).tcf_bindcnt) - bind;
    let mut t: tcf_t = core::mem::zeroed(); spin_lock_bh(&mut (*gact).tcf_lock); opt.action = (*gact).tcf_action;
    if nla_put(skb, TCA_GACT_PARMS, core::mem::size_of::<tc_gact>(), &opt) != 0 { spin_unlock_bh(&mut (*gact).tcf_lock); nlmsg_trim(skb,b); return -1; }
    tcf_tm_dump(&mut t, &(*gact).tcf_tm);
    if nla_put_64bit(skb,TCA_GACT_TM,core::mem::size_of::<tcf_t>(),&t,TCA_GACT_PAD) != 0 { spin_unlock_bh(&mut (*gact).tcf_lock); nlmsg_trim(skb,b); return -1; }
    spin_unlock_bh(&mut (*gact).tcf_lock); (*skb).len as i32
}
unsafe fn tcf_gact_get_fill_size(act: *const tc_action) -> usize { let mut sz=nla_total_size(core::mem::size_of::<tc_gact>()); #[cfg(CONFIG_GACT_PROB)] if (*to_gact(act)).tcfg_ptype != 0 { sz += nla_total_size(core::mem::size_of::<tc_gact_p>()); } sz }
unsafe fn tcf_gact_offload_act_setup(act: *mut tc_action, entry_data: *mut core::ffi::c_void, index_inc: *mut u32, bind: bool, extack: *mut netlink_ext_ack) -> i32 {
    if bind { let e=entry_data as *mut flow_action_entry; if is_tcf_gact_ok(act){(*e).id=FLOW_ACTION_ACCEPT}else if is_tcf_gact_shot(act){(*e).id=FLOW_ACTION_DROP}else if is_tcf_gact_trap(act){(*e).id=FLOW_ACTION_TRAP}else if is_tcf_gact_goto_chain(act){(*e).id=FLOW_ACTION_GOTO;(*e).chain_index=tcf_gact_goto_chain_index(act)}else{NL_SET_ERR_MSG_MOD(extack,"Unsupported generic action offload");return -EOPNOTSUPP} *index_inc=1;
    } else { let e=entry_data as *mut flow_offload_action; if is_tcf_gact_ok(act){(*e).id=FLOW_ACTION_ACCEPT}else if is_tcf_gact_shot(act){(*e).id=FLOW_ACTION_DROP}else if is_tcf_gact_trap(act){(*e).id=FLOW_ACTION_TRAP}else if is_tcf_gact_goto_chain(act){(*e).id=FLOW_ACTION_GOTO}else{return -EOPNOTSUPP} } 0
}
unsafe fn gact_init_net(net:*mut net)->i32 { tc_action_net_init(net,net_generic(net,act_gact_ops.net_id),&mut act_gact_ops) }
unsafe fn gact_exit_net(net_list:*mut list_head){tc_action_net_exit(net_list,act_gact_ops.net_id)}
static mut gact_net_ops: pernet_operations = pernet_operations::default();
unsafe fn gact_init_module() -> i32 { tcf_register_action(&mut act_gact_ops, &mut gact_net_ops) }
unsafe fn gact_cleanup_module() { tcf_unregister_action(&mut act_gact_ops, &mut gact_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
