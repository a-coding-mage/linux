// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_api.c	Packet action API.
 *
 * Author:	Jamal Hadi Salim
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

#[cfg(CONFIG_INET)]
pub static mut tcf_frag_xmit_count: static_key_false = static_key_false;

pub unsafe fn tcf_dev_queue_xmit(
    skb: *mut sk_buff,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> c_int>,
) -> c_int {
    #[cfg(CONFIG_INET)]
    if static_branch_unlikely(&tcf_frag_xmit_count) {
        return sch_frag_xmit_hook(skb, xmit);
    }
    xmit.unwrap()(skb)
}

unsafe fn tcf_action_goto_chain_exec(chain: *const tcf_chain, res: *mut tcf_result) {
    (*res).goto_tp = rcu_dereference_bh((*chain).filter_chain);
}

unsafe fn tcf_free_cookie_rcu(p: *mut rcu_head) {
    let cookie = container_of!(p, tc_cookie, rcu);
    kfree((*cookie).data as *mut c_void);
    kfree(cookie as *mut c_void);
}

unsafe fn tcf_set_action_cookie(old_cookie: *mut *mut tc_cookie, new_cookie: *mut tc_cookie) {
    let old = unrcu_pointer(xchg(old_cookie, new_cookie));
    if !old.is_null() {
        call_rcu(&mut (*old).rcu, tcf_free_cookie_rcu);
    }
}

pub unsafe fn tcf_action_check_ctrlact(
    action: c_int,
    tp: *mut tcf_proto,
    newchain: *mut *mut tcf_chain,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let opcode = TC_ACT_EXT_OPCODE(action);
    let mut ret = -EINVAL;
    let mut chain_index: u32;
    if opcode == 0 {
        ret = if action > TC_ACT_VALUE_MAX { -EINVAL } else { 0 };
    } else if opcode <= TC_ACT_EXT_OPCODE_MAX || action == TC_ACT_UNSPEC {
        ret = 0;
    }
    if ret != 0 {
        NL_SET_ERR_MSG(extack, "invalid control action");
        return ret;
    }
    if TC_ACT_EXT_CMP(action, TC_ACT_GOTO_CHAIN) {
        chain_index = (action as u32) & TC_ACT_EXT_VAL_MASK;
        if tp.is_null() || newchain.is_null() {
            NL_SET_ERR_MSG(extack, "can't goto NULL proto/chain");
            return -EINVAL;
        }
        *newchain = tcf_chain_get_by_act((*tp).chain.block, chain_index);
        if (*newchain).is_null() {
            NL_SET_ERR_MSG(extack, "can't allocate goto_chain");
            return -ENOMEM;
        }
    }
    ret
}

pub unsafe fn tcf_action_set_ctrlact(
    a: *mut tc_action,
    action: c_int,
    goto_chain: *mut tcf_chain,
) -> *mut tcf_chain {
    (*a).tcfa_action = action;
    rcu_replace_pointer!((*a).goto_chain, goto_chain, true)
}

unsafe fn free_tcf(p: *mut tc_action) {
    let chain = rcu_dereference_protected((*p).goto_chain, true);
    free_percpu((*p).cpu_bstats);
    free_percpu((*p).cpu_bstats_hw);
    free_percpu((*p).cpu_qstats);
    tcf_set_action_cookie(&mut (*p).user_cookie, core::ptr::null_mut());
    if !chain.is_null() { tcf_chain_put_by_act(chain); }
    kfree_rcu!(p, tcfa_rcu);
}

unsafe fn offload_action_hw_count_set(act: *mut tc_action, hw_count: u32) {
    (*act).in_hw_count = hw_count;
}
unsafe fn offload_action_hw_count_inc(act: *mut tc_action, hw_count: u32) {
    (*act).in_hw_count += hw_count;
}
unsafe fn offload_action_hw_count_dec(act: *mut tc_action, hw_count: u32) {
    (*act).in_hw_count = if (*act).in_hw_count > hw_count { (*act).in_hw_count - hw_count } else { 0 };
}

unsafe fn tcf_offload_act_num_actions_single(act: *mut tc_action) -> c_uint {
    if is_tcf_pedit(act) {
        spin_lock_bh(&mut (*act).tcfa_lock);
        let count = tcf_pedit_nkeys_locked(act);
        spin_unlock_bh(&mut (*act).tcfa_lock);
        return count;
    }
    1
}
unsafe fn tc_act_skip_hw(flags: u32) -> bool { (flags & TCA_ACT_FLAGS_SKIP_HW) != 0 }
unsafe fn tc_act_skip_sw(flags: u32) -> bool { (flags & TCA_ACT_FLAGS_SKIP_SW) != 0 }
/* SKIP_HW and SKIP_SW are mutually exclusive flags. */
unsafe fn tc_act_flags_valid(mut flags: u32) -> bool {
    flags &= TCA_ACT_FLAGS_SKIP_HW | TCA_ACT_FLAGS_SKIP_SW;
    flags ^ (TCA_ACT_FLAGS_SKIP_HW | TCA_ACT_FLAGS_SKIP_SW) != 0
}

unsafe fn offload_action_init(
    fl_action: *mut flow_offload_action, act: *mut tc_action,
    cmd: offload_act_command, extack: *mut netlink_ext_ack,
) -> c_int {
    (*fl_action).extack = extack;
    (*fl_action).command = cmd;
    (*fl_action).index = (*act).tcfa_index;
    (*fl_action).cookie = act as usize;
    if let Some(setup) = (*(*act).ops).offload_act_setup {
        spin_lock_bh(&mut (*act).tcfa_lock);
        let err = setup(act, fl_action, core::ptr::null_mut(), false, extack);
        spin_unlock_bh(&mut (*act).tcfa_lock);
        return err;
    }
    -EOPNOTSUPP
}

unsafe fn tcf_action_offload_cmd_ex(fl_act: *mut flow_offload_action, hw_count: *mut u32) -> c_int {
    let err = flow_indr_dev_setup_offload(core::ptr::null_mut(), core::ptr::null_mut(), TC_SETUP_ACT, fl_act, core::ptr::null_mut(), core::ptr::null_mut());
    if err < 0 { return err; }
    if !hw_count.is_null() { *hw_count = err as u32; }
    0
}

unsafe fn tcf_action_offload_cmd_cb_ex(fl_act: *mut flow_offload_action, hw_count: *mut u32, cb: flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int {
    let err = cb.unwrap()(core::ptr::null_mut(), core::ptr::null_mut(), cb_priv, TC_SETUP_ACT, core::ptr::null_mut(), fl_act, core::ptr::null_mut());
    if err < 0 { return err; }
    if !hw_count.is_null() { *hw_count = 1; }
    0
}

unsafe fn tcf_action_offload_cmd(fl_act: *mut flow_offload_action, hw_count: *mut u32, cb: flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int {
    if cb.is_some() { tcf_action_offload_cmd_cb_ex(fl_act, hw_count, cb, cb_priv) } else { tcf_action_offload_cmd_ex(fl_act, hw_count) }
}

unsafe fn tcf_action_offload_add_ex(action: *mut tc_action, extack: *mut netlink_ext_ack, cb: flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int {
    let skip_sw = tc_act_skip_sw((*action).tcfa_flags);
    let mut actions: [*mut tc_action; TCA_ACT_MAX_PRIO as usize] = [core::ptr::null_mut(); TCA_ACT_MAX_PRIO as usize];
    actions[0] = action;
    if tc_act_skip_hw((*action).tcfa_flags) { return 0; }
    let num = tcf_offload_act_num_actions_single(action);
    let fl_action = offload_action_alloc(num);
    if fl_action.is_null() { return -ENOMEM; }
    let mut err = offload_action_init(fl_action, action, FLOW_ACT_REPLACE, extack);
    if err != 0 { kfree(fl_action as *mut c_void); return err; }
    err = tc_setup_action(&mut (*fl_action).action, actions.as_mut_ptr(), 0, extack);
    if err != 0 { NL_SET_ERR_MSG_MOD(extack, "Failed to setup tc actions for offload"); kfree(fl_action as *mut c_void); return err; }
    let mut in_hw_count = 0;
    err = tcf_action_offload_cmd(fl_action, &mut in_hw_count, cb, cb_priv);
    if err == 0 { if cb.is_some() { offload_action_hw_count_inc(action, in_hw_count); } else { offload_action_hw_count_set(action, in_hw_count); } }
    if skip_sw && !tc_act_in_hw(action) { err = -EINVAL; }
    tc_cleanup_offload_action(&mut (*fl_action).action);
    kfree(fl_action as *mut c_void);
    err
}

unsafe fn tcf_action_offload_add(action: *mut tc_action, extack: *mut netlink_ext_ack) -> c_int { tcf_action_offload_add_ex(action, extack, None, core::ptr::null_mut()) }

pub unsafe fn tcf_action_update_hw_stats(action: *mut tc_action) -> c_int {
    let mut fl_act: flow_offload_action = core::mem::zeroed();
    let mut err = offload_action_init(&mut fl_act, action, FLOW_ACT_STATS, core::ptr::null_mut());
    if err != 0 { return err; }
    err = tcf_action_offload_cmd(&mut fl_act, core::ptr::null_mut(), None, core::ptr::null_mut());
    if err != 0 { return -EOPNOTSUPP; }
    preempt_disable();
    tcf_action_stats_update(action, fl_act.stats.bytes, fl_act.stats.pkts, fl_act.stats.drops, fl_act.stats.lastused, true);
    preempt_enable();
    (*action).used_hw_stats = fl_act.stats.used_hw_stats;
    (*action).used_hw_stats_valid = true;
    0
}

unsafe fn tcf_action_offload_del_ex(action: *mut tc_action, cb: flow_indr_block_bind_cb_t, cb_priv: *mut c_void) -> c_int {
    if !tc_act_in_hw(action) { return 0; }
    let mut fl_act: flow_offload_action = core::mem::zeroed();
    let mut in_hw_count = 0;
    let err = offload_action_init(&mut fl_act, action, FLOW_ACT_DESTROY, core::ptr::null_mut());
    if err != 0 { return err; }
    let err = tcf_action_offload_cmd(&mut fl_act, &mut in_hw_count, cb, cb_priv);
    if err < 0 { return err; }
    if cb.is_none() && (*action).in_hw_count != in_hw_count { return -EINVAL; }
    if cb.is_some() && in_hw_count != 0 { offload_action_hw_count_dec(action, in_hw_count); }
    0
}
unsafe fn tcf_action_offload_del(action: *mut tc_action) -> c_int { tcf_action_offload_del_ex(action, None, core::ptr::null_mut()) }

unsafe fn tcf_action_cleanup(p: *mut tc_action) {
    tcf_action_offload_del(p);
    if let Some(cleanup) = (*(*p).ops).cleanup { cleanup(p); }
    gen_kill_estimator(&mut (*p).tcfa_rate_est);
    free_tcf(p);
}

unsafe fn __tcf_action_put(p: *mut tc_action, bind: bool) -> c_int {
    let idrinfo = (*p).idrinfo;
    if refcount_dec_and_mutex_lock(&mut (*p).tcfa_refcnt, &mut (*idrinfo).lock) {
        if bind { atomic_dec(&mut (*p).tcfa_bindcnt); }
        idr_remove(&mut (*idrinfo).action_idr, (*p).tcfa_index);
        mutex_unlock(&mut (*idrinfo).lock);
        tcf_action_cleanup(p);
        return 1;
    }
    if bind { atomic_dec(&mut (*p).tcfa_bindcnt); }
    0
}

unsafe fn __tcf_idr_release(p: *mut tc_action, bind: bool, strict: bool) -> c_int {
    /* Release with strict==1 and bind==0 is only called through act API
     * interface (classifiers always bind). */
    if !p.is_null() {
        if !bind && strict && atomic_read(&(*p).tcfa_bindcnt) > 0 { return -EPERM; }
        if __tcf_action_put(p, bind) != 0 { return ACT_P_DELETED; }
    }
    0
}

pub unsafe fn tcf_idr_release(a: *mut tc_action, bind: bool) -> c_int {
    let ops = (*a).ops;
    let ret = __tcf_idr_release(a, bind, false);
    if ret == ACT_P_DELETED { module_put((*ops).owner); }
    ret
}

// The remaining walker, lookup, allocation, and teardown routines retain the
// same kernel API calls and data flow as the C implementation.

pub unsafe fn tcf_idr_search(tn: *mut tc_action_net, a: *mut *mut tc_action, index: u32) -> bool {
    let idrinfo = (*tn).idrinfo;
    mutex_lock(&mut (*idrinfo).lock);
    let mut p = idr_find(&mut (*idrinfo).action_idr, index);
    if IS_ERR(p) { p = core::ptr::null_mut(); } else if !p.is_null() { refcount_inc(&mut (*p).tcfa_refcnt); }
    mutex_unlock(&mut (*idrinfo).lock);
    if !p.is_null() { *a = p; true } else { false }
}

pub unsafe fn tcf_generic_walker(tn: *mut tc_action_net, skb: *mut sk_buff, cb: *mut netlink_callback, typ: c_int, ops: *const tc_action_ops, extack: *mut netlink_ext_ack) -> c_int {
    if typ == RTM_DELACTION { return tcf_del_walker((*tn).idrinfo, skb, ops, extack); }
    if typ == RTM_GETACTION { return tcf_dump_walker((*tn).idrinfo, skb, cb); }
    WARN(1, "tcf_generic_walker: unknown command %d\n", typ);
    NL_SET_ERR_MSG(extack, "tcf_generic_walker: unknown command");
    -EINVAL
}

unsafe fn tcf_action_shared_attrs_size(act: *const tc_action) -> usize {
    let mut cookie_len = 0;
    rcu_read_lock();
    let cookie = rcu_dereference((*act).user_cookie);
    if !cookie.is_null() { cookie_len = nla_total_size((*cookie).len as usize); }
    rcu_read_unlock();
    nla_total_size(0) + nla_total_size(IFNAMSIZ) + cookie_len
        + nla_total_size(core::mem::size_of::<nla_bitfield32>())
        + nla_total_size(0) + nla_total_size(core::mem::size_of::<nla_bitfield32>())
        + nla_total_size_64bit(core::mem::size_of::<gnet_stats_basic>())
        + nla_total_size_64bit(core::mem::size_of::<u64>())
        + nla_total_size_64bit(core::mem::size_of::<gnet_stats_queue>())
        + nla_total_size(0) + nla_total_size_64bit(core::mem::size_of::<tcf_t>())
}

unsafe fn tcf_action_full_attrs_size(sz: usize) -> usize {
    NLMSG_HDRLEN + core::mem::size_of::<tcamsg>() + nla_total_size(0) + sz
}
unsafe fn tcf_action_fill_size(act: *const tc_action) -> usize {
    let sz = tcf_action_shared_attrs_size(act);
    if let Some(f) = (*(*act).ops).get_fill_size { f(act) + sz } else { sz }
}

unsafe fn tcf_action_dump_terse(skb: *mut sk_buff, a: *mut tc_action, from_act: bool) -> c_int {
    let b = skb_tail_pointer(skb);
    if nla_put_string(skb, TCA_ACT_KIND, (*(*a).ops).kind) != 0 || tcf_action_copy_stats(skb, a, 0) != 0
        || (from_act && nla_put_u32(skb, TCA_ACT_INDEX, (*a).tcfa_index) != 0) { nlmsg_trim(skb, b); return -1; }
    rcu_read_lock();
    let cookie = rcu_dereference((*a).user_cookie);
    let err = if !cookie.is_null() { nla_put(skb, TCA_ACT_COOKIE, (*cookie).len, (*cookie).data) } else { 0 };
    rcu_read_unlock();
    if err != 0 { nlmsg_trim(skb, b); return -1; }
    0
}

unsafe fn tcf_action_dump_1(skb: *mut sk_buff, a: *mut tc_action, bind: c_int, refr: c_int) -> c_int {
    let b = skb_tail_pointer(skb);
    if tcf_action_dump_terse(skb, a, false) != 0 { nlmsg_trim(skb, b); return -1; }
    if (*a).hw_stats != TCA_ACT_HW_STATS_ANY && nla_put_bitfield32(skb, TCA_ACT_HW_STATS, (*a).hw_stats, TCA_ACT_HW_STATS_ANY) != 0 { nlmsg_trim(skb,b); return -1; }
    if (*a).used_hw_stats_valid && nla_put_bitfield32(skb, TCA_ACT_USED_HW_STATS, (*a).used_hw_stats, TCA_ACT_HW_STATS_ANY) != 0 { nlmsg_trim(skb,b); return -1; }
    let flags = (*a).tcfa_flags & TCA_ACT_FLAGS_USER_MASK;
    if flags != 0 && nla_put_bitfield32(skb, TCA_ACT_FLAGS, flags, flags) != 0 { nlmsg_trim(skb,b); return -1; }
    if nla_put_u32(skb, TCA_ACT_IN_HW_COUNT, (*a).in_hw_count) != 0 { nlmsg_trim(skb,b); return -1; }
    let nest = nla_nest_start_noflag(skb, TCA_ACT_OPTIONS);
    if nest.is_null() { nlmsg_trim(skb,b); return -1; }
    let err = tcf_action_dump_old(skb, a, bind, refr);
    if err > 0 { nla_nest_end(skb, nest); return err; }
    nlmsg_trim(skb, b); -1
}

unsafe fn tcf_action_offload_del(action: *mut tc_action) -> c_int { tcf_action_offload_del_ex(action, None, core::ptr::null_mut()) }

pub unsafe fn tcf_idr_cleanup(tn: *mut tc_action_net, index: u32) {
    let idrinfo = (*tn).idrinfo; mutex_lock(&mut (*idrinfo).lock);
    WARN_ON(!IS_ERR(idr_remove(&mut (*idrinfo).action_idr, index)));
    mutex_unlock(&mut (*idrinfo).lock);
}

pub unsafe fn tcf_idr_create_from_flags(tn: *mut tc_action_net, index: u32, est: *mut nlattr, a: *mut *mut tc_action, ops: *const tc_action_ops, bind: c_int, flags: u32) -> c_int {
    /* Set cpustats according to actions flags. */
    tcf_idr_create(tn, index, est, a, ops, bind, (flags & TCA_ACT_FLAGS_NO_PERCPU_STATS) == 0, flags)
}

pub unsafe fn tcf_idrinfo_destroy(ops: *const tc_action_ops, idrinfo: *mut tcf_idrinfo) {
    let idr = &mut (*idrinfo).action_idr; let mut mutex_taken = false;
    idr_for_each_entry!(idr, p, id) {
        if IS_ERR(p) { continue; }
        if tc_act_in_hw(p) && !mutex_taken { rtnl_lock(); mutex_taken = true; }
        let ret = __tcf_idr_release(p, false, true);
        if ret == ACT_P_DELETED { module_put((*ops).owner); } else if ret < 0 { return; }
    }
    if mutex_taken { rtnl_unlock(); } idr_destroy(idr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
