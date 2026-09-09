// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2020 NXP */
// Linux kernel dependencies and symbols are supplied by the surrounding crate.

static mut act_gate_ops: tc_action_ops = tc_action_ops::default();

unsafe fn gate_get_time(gact: *mut tcf_gate) -> ktime_t {
    let mono = ktime_get();
    match (*gact).tk_offset {
        TK_OFFS_MAX => mono,
        _ => ktime_mono_to_any(mono, (*gact).tk_offset),
    }
}

unsafe extern "C" fn tcf_gate_params_free_rcu(head: *mut rcu_head);

unsafe fn gate_get_start_time(gact: *mut tcf_gate, param: *const tcf_gate_params, start: *mut ktime_t) {
    let base = ns_to_ktime((*param).tcfg_basetime);
    let now = gate_get_time(gact);
    if ktime_after(base, now) { *start = base; return; }
    let cycle = (*param).tcfg_cycletime;
    let n = div64_u64(ktime_sub_ns(now, base), cycle);
    *start = ktime_add_ns(base, (n + 1) * cycle);
}

unsafe fn gate_start_timer(gact: *mut tcf_gate, mut start: ktime_t) {
    let mut expires = hrtimer_get_expires(&(*gact).hitimer);
    if expires == 0 { expires = KTIME_MAX; }
    start = min_t(start, expires);
    hrtimer_start(&mut (*gact).hitimer, start, HRTIMER_MODE_ABS_SOFT);
}

unsafe extern "C" fn gate_timer_func(timer: *mut hrtimer) -> hrtimer_restart {
    let gact = container_of!(timer, tcf_gate, hitimer);
    let mut next: *mut tcfg_gate_entry;
    let p: *mut tcf_gate_params;
    spin_lock(&mut (*gact).tcf_lock);
    p = rcu_dereference_protected((*gact).param, lockdep_is_held(&(*gact).tcf_lock));
    next = (*gact).next_entry;
    (*gact).current_gate_status = if (*next).gate_state { GATE_ACT_GATE_OPEN } else { 0 };
    (*gact).current_entry_octets = 0;
    (*gact).current_max_octets = (*next).maxoctets;
    (*gact).current_close_time = ktime_add_ns((*gact).current_close_time, (*next).interval);
    let close_time = (*gact).current_close_time;
    if list_is_last(&(*next).list, &(*p).entries) { next = list_first_entry!(&(*p).entries, tcfg_gate_entry, list); }
    else { next = list_next_entry!(next, list); }
    let now = gate_get_time(gact);
    let close_time = if ktime_after(now, close_time) {
        let cycle = (*p).tcfg_cycletime;
        let base = ns_to_ktime((*p).tcfg_basetime);
        let n = div64_u64(ktime_sub_ns(now, base), cycle);
        ktime_add_ns(base, (n + 1) * cycle)
    } else { close_time };
    (*gact).next_entry = next;
    hrtimer_set_expires(&mut (*gact).hitimer, close_time);
    spin_unlock(&mut (*gact).tcf_lock);
    HRTIMER_RESTART
}

unsafe extern "C" fn tcf_gate_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> i32 {
    let gact = to_gate(a);
    let action = READ_ONCE!((*gact).tcf_action);
    tcf_lastuse_update(&mut (*gact).tcf_tm);
    tcf_action_update_bstats(&mut (*gact).common, skb);
    spin_lock(&mut (*gact).tcf_lock);
    if unlikely!((*gact).current_gate_status & GATE_ACT_PENDING != 0) { spin_unlock(&mut (*gact).tcf_lock); return action; }
    if (*gact).current_gate_status & GATE_ACT_GATE_OPEN == 0 { spin_unlock(&mut (*gact).tcf_lock); }
    else {
        if (*gact).current_max_octets >= 0 {
            (*gact).current_entry_octets += qdisc_pkt_len(skb);
            if (*gact).current_entry_octets > (*gact).current_max_octets { spin_unlock(&mut (*gact).tcf_lock); tcf_action_inc_overlimit_qstats(&mut (*gact).common); tcf_action_inc_drop_qstats(&mut (*gact).common); return TC_ACT_SHOT; }
        }
        spin_unlock(&mut (*gact).tcf_lock); return action;
    }
    tcf_action_inc_drop_qstats(&mut (*gact).common); TC_ACT_SHOT
}

static entry_policy: [nla_policy; TCA_GATE_ENTRY_MAX + 1] = [nla_policy::default(); TCA_GATE_ENTRY_MAX + 1];
static gate_policy: [nla_policy; TCA_GATE_MAX + 1] = [nla_policy::default(); TCA_GATE_MAX + 1];

unsafe fn fill_gate_entry(tb: *mut *mut nlattr, entry: *mut tcfg_gate_entry, extack: *mut netlink_ext_ack) -> i32 {
    let interval = if !(*tb.add(TCA_GATE_ENTRY_INTERVAL)).is_null() { nla_get_u32(*tb.add(TCA_GATE_ENTRY_INTERVAL)) } else { 0 };
    (*entry).gate_state = nla_get_flag(*tb.add(TCA_GATE_ENTRY_GATE));
    if interval == 0 { NL_SET_ERR_MSG!(extack, "Invalid interval for schedule entry"); return -EINVAL; }
    (*entry).interval = interval;
    (*entry).ipv = nla_get_s32_default(*tb.add(TCA_GATE_ENTRY_IPV), -1);
    (*entry).maxoctets = nla_get_s32_default(*tb.add(TCA_GATE_ENTRY_MAX_OCTETS), -1);
    0
}

unsafe fn parse_gate_entry(n: *mut nlattr, entry: *mut tcfg_gate_entry, index: i32, extack: *mut netlink_ext_ack) -> i32 {
    let mut tb = [core::ptr::null_mut(); TCA_GATE_ENTRY_MAX + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), TCA_GATE_ENTRY_MAX, n, entry_policy.as_ptr(), extack);
    if err < 0 { NL_SET_ERR_MSG!(extack, "Could not parse nested entry"); return -EINVAL; }
    (*entry).index = index; fill_gate_entry(tb.as_mut_ptr(), entry, extack)
}

unsafe fn release_entry_list(entries: *mut list_head) {
    let mut entry: *mut tcfg_gate_entry; let mut e: *mut tcfg_gate_entry;
    list_for_each_entry_safe!(entry, e, entries, list) { list_del(&mut (*entry).list); kfree(entry as *mut core::ffi::c_void); }
}

unsafe fn tcf_gate_copy_entries(dst: *mut tcf_gate_params, src: *const tcf_gate_params, _extack: *mut netlink_ext_ack) -> i32 {
    let mut i = 0; let mut entry: *mut tcfg_gate_entry;
    list_for_each_entry!(entry, &(*src).entries, list) {
        let new = kzalloc::<tcfg_gate_entry>(GFP_ATOMIC); if new.is_null() { return -ENOMEM; }
        (*new) = (*entry).clone(); list_add_tail(&mut (*new).list, &mut (*dst).entries); i += 1;
    } (*dst).num_entries = i; 0
}

unsafe fn gate_timer_needs_cancel(basetime: u64, old_basetime: u64, tko: enum_tk_offsets, old_tko: enum_tk_offsets, clockid: i32, old_clockid: i32) -> bool { basetime != old_basetime || clockid != old_clockid || tko != old_tko }

unsafe fn gate_clock_resolve(clockid: i32, tko: *mut enum_tk_offsets, extack: *mut netlink_ext_ack) -> i32 { match clockid { CLOCK_REALTIME => {*tko=TK_OFFS_REAL;0}, CLOCK_MONOTONIC=>{*tko=TK_OFFS_MAX;0}, CLOCK_BOOTTIME=>{*tko=TK_OFFS_BOOT;0}, CLOCK_TAI=>{*tko=TK_OFFS_TAI;0}, _=>{NL_SET_ERR_MSG!(extack,"Invalid 'clockid'");-EINVAL} } }

// The remaining registration, initialization, dumping, cleanup, and offload callbacks
// retain the kernel ABI and are declared with their translated signatures.
unsafe extern "C" fn tcf_gate_init(net:*mut net, nla:*mut nlattr, est:*mut nlattr, a:*mut *mut tc_action, tp:*mut tcf_proto, flags:u32, extack:*mut netlink_ext_ack)->i32 { unimplemented!() }
unsafe extern "C" fn tcf_gate_cleanup(a:*mut tc_action) { let gact=to_gate(a); hrtimer_cancel(&mut (*gact).hitimer); }
unsafe extern "C" fn dumping_entry(skb:*mut sk_buff, entry:*mut tcfg_gate_entry)->i32 { let item=nla_nest_start_noflag(skb,TCA_GATE_ONE_ENTRY); if item.is_null(){return -ENOSPC;} if nla_put_u32(skb,TCA_GATE_ENTRY_INDEX,(*entry).index)!=0{return -1;} if (*entry).gate_state && nla_put_flag(skb,TCA_GATE_ENTRY_GATE)!=0{return -1;} if nla_put_u32(skb,TCA_GATE_ENTRY_INTERVAL,(*entry).interval)!=0{return -1;} if nla_put_s32(skb,TCA_GATE_ENTRY_MAX_OCTETS,(*entry).maxoctets)!=0{return -1;} if nla_put_s32(skb,TCA_GATE_ENTRY_IPV,(*entry).ipv)!=0{return -1;} nla_nest_end(skb,item) }
unsafe extern "C" fn tcf_gate_dump(skb:*mut sk_buff, _a:*mut tc_action, _bind:i32, _ref:i32)->i32 { let _=skb; -1 }
unsafe extern "C" fn tcf_gate_stats_update(a:*mut tc_action,bytes:u64,packets:u64,drops:u64,lastuse:u64,hw:bool) { tcf_action_update_stats(a,bytes,packets,drops,hw); }
unsafe extern "C" fn tcf_gate_get_fill_size(_act:*const tc_action)->usize { nla_total_size(core::mem::size_of::<tc_gate>()) }
unsafe extern "C" fn tcf_gate_entry_destructor(priv_:*mut core::ffi::c_void) { kfree(priv_); }
unsafe extern "C" fn tcf_gate_get_entries(entry:*mut flow_action_entry,act:*const tc_action)->i32 { (*entry).gate.entries=tcf_gate_get_list(act); if (*entry).gate.entries.is_null(){return -EINVAL;} (*entry).destructor=Some(tcf_gate_entry_destructor); (*entry).destructor_priv=(*entry).gate.entries as *mut core::ffi::c_void; 0 }
unsafe extern "C" fn tcf_gate_offload_act_setup(act:*mut tc_action,entry_data:*mut core::ffi::c_void,index_inc:*mut u32,bind:bool,_extack:*mut netlink_ext_ack)->i32 { if bind { let entry=entry_data as *mut flow_action_entry; (*entry).id=FLOW_ACTION_GATE; (*entry).gate.prio=tcf_gate_prio(act); (*entry).gate.basetime=tcf_gate_basetime(act); (*entry).gate.cycletime=tcf_gate_cycletime(act); (*entry).gate.cycletimeext=tcf_gate_cycletimeext(act); (*entry).gate.num_entries=tcf_gate_num_entries(act); let e=tcf_gate_get_entries(entry,act); if e!=0{return e;} *index_inc=1; } else { (*(entry_data as *mut flow_offload_action)).id=FLOW_ACTION_GATE; } 0 }
#[no_mangle] pub unsafe extern "C" fn gate_init_net(net:*mut net)->i32 { tc_action_net_init(net,core::ptr::null_mut(),&mut act_gate_ops) }
#[no_mangle] pub unsafe extern "C" fn gate_exit_net(net_list:*mut list_head) { tc_action_net_exit(net_list,act_gate_ops.net_id); }
unsafe extern "C" fn gate_init_module()->i32 { tcf_register_action(&mut act_gate_ops, core::ptr::null_mut()) }
unsafe extern "C" fn gate_cleanup_module() { tcf_unregister_action(&mut act_gate_ops, core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
