// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_police.c Input police filter
 *
 * Authors: Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 *          J Hadi Salim (action changes)
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

/* Each policer is serialized by its individual spinlock */
static mut act_police_ops: tc_action_ops = tc_action_ops { };

static police_policy: [nla_policy; TCA_POLICE_MAX as usize + 1] = [nla_policy { }; TCA_POLICE_MAX as usize + 1];

unsafe fn tcf_police_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
    a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32,
    extack: *mut netlink_ext_ack) -> c_int {
    let mut ret: c_int = 0;
    let mut tcfp_result: c_int = TC_ACT_OK;
    let mut err: c_int;
    let mut size: c_int;
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut tb: [*mut nlattr; TCA_POLICE_MAX as usize + 1] = [core::ptr::null_mut(); TCA_POLICE_MAX as usize + 1];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_police;
    let police: *mut tcf_police;
    let mut r_tab: *mut qdisc_rate_table = core::ptr::null_mut();
    let mut p_tab: *mut qdisc_rate_table = core::ptr::null_mut();
    let tn = net_generic(net, (*(&raw const act_police_ops)).net_id);
    let mut new: *mut tcf_police_params;
    let mut exists = false;
    let mut index: u32;
    let mut rate64: u64;
    let mut prate64: u64;
    let mut pps: u64;
    let mut ppsburst: u64;

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_POLICE_MAX, nla, police_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_POLICE_TBF as usize].is_null() { return -EINVAL; }
    size = nla_len(tb[TCA_POLICE_TBF as usize]);
    if size as usize != core::mem::size_of::<tc_police>() && size as usize != core::mem::size_of::<tc_police_compat>() { return -EINVAL; }
    parm = nla_data(tb[TCA_POLICE_TBF as usize]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &mut index, a, bind);
    if err < 0 { return err; }
    exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }
    if !exists {
        ret = tcf_idr_create(tn, index, core::ptr::null_mut(), a, &raw mut act_police_ops, bind, true, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
        spin_lock_init(&mut (*to_police(*a)).tcfp_lock);
    } else if flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { goto release_idr; }
    police = to_police(*a);
    if (*parm).rate.rate != 0 {
        err = -ENOMEM;
        r_tab = qdisc_get_rtab(&(*parm).rate, tb[TCA_POLICE_RATE as usize], core::ptr::null_mut());
        if r_tab.is_null() { goto failure; }
        if (*parm).peakrate.rate != 0 {
            p_tab = qdisc_get_rtab(&(*parm).peakrate, tb[TCA_POLICE_PEAKRATE as usize], core::ptr::null_mut());
            if p_tab.is_null() { goto failure; }
        }
    }
    if !est.is_null() { err = gen_replace_estimator(&mut (*police).tcf_bstats, (*police).common.cpu_bstats, &mut (*police).tcf_rate_est, &mut (*police).tcf_lock, false, est); if err != 0 { goto failure; } }
    else if !tb[TCA_POLICE_AVRATE as usize].is_null() && (ret == ACT_P_CREATED || !gen_estimator_active(&(*police).tcf_rate_est)) { err = -EINVAL; goto failure; }
    if !tb[TCA_POLICE_RESULT as usize].is_null() { tcfp_result = nla_get_u32(tb[TCA_POLICE_RESULT as usize]) as c_int; if !tcf_action_valid(tcfp_result) || TC_ACT_EXT_CMP(tcfp_result, TC_ACT_GOTO_CHAIN) { err = -EINVAL; goto failure; } }
    if (!tb[TCA_POLICE_PKTRATE64 as usize].is_null()) != (!tb[TCA_POLICE_PKTBURST64 as usize].is_null()) { err = -EINVAL; goto failure; }
    if !tb[TCA_POLICE_PKTRATE64 as usize].is_null() && !r_tab.is_null() { err = -EINVAL; goto failure; }
    new = kzalloc_obj::<tcf_police_params>();
    if new.is_null() { err = -ENOMEM; goto failure; }
    (*new).tcfp_result = tcfp_result as u32;
    (*new).tcfp_mtu = (*parm).mtu;
    if (*new).tcfp_mtu == 0 { (*new).tcfp_mtu = !0; if !r_tab.is_null() { (*new).tcfp_mtu = 255 << (*r_tab).rate.cell_log; } }
    if !r_tab.is_null() { (*new).rate_present = true; rate64 = nla_get_u64_default(tb[TCA_POLICE_RATE64 as usize], 0); psched_ratecfg_precompute(&mut (*new).rate, &(*r_tab).rate, rate64); qdisc_put_rtab(r_tab); r_tab = core::ptr::null_mut(); } else { (*new).rate_present = false; }
    if !p_tab.is_null() { (*new).peak_present = true; prate64 = nla_get_u64_default(tb[TCA_POLICE_PEAKRATE64 as usize], 0); psched_ratecfg_precompute(&mut (*new).peak, &(*p_tab).rate, prate64); qdisc_put_rtab(p_tab); p_tab = core::ptr::null_mut(); } else { (*new).peak_present = false; }
    (*new).tcfp_burst = PSCHED_TICKS2NS((*parm).burst);
    if (*new).peak_present { (*new).tcfp_mtu_ptoks = psched_l2t_ns(&(*new).peak, (*new).tcfp_mtu) as i64; }
    if !tb[TCA_POLICE_AVRATE as usize].is_null() { (*new).tcfp_ewma_rate = nla_get_u32(tb[TCA_POLICE_AVRATE as usize]); }
    if !tb[TCA_POLICE_PKTRATE64 as usize].is_null() { pps = nla_get_u64(tb[TCA_POLICE_PKTRATE64 as usize]); ppsburst = nla_get_u64(tb[TCA_POLICE_PKTBURST64 as usize]); (*new).pps_present = true; (*new).tcfp_pkt_burst = PSCHED_TICKS2NS(ppsburst); psched_ppscfg_precompute(&mut (*new).ppsrate, pps); }
    (*new).action = (*parm).action;
    spin_lock_bh(&mut (*police).tcf_lock); spin_lock_bh(&mut (*police).tcfp_lock);
    (*police).tcfp_t_c = ktime_get_ns(); (*police).tcfp_toks = (*new).tcfp_burst; if (*new).peak_present { (*police).tcfp_ptoks = (*new).tcfp_mtu_ptoks; }
    spin_unlock_bh(&mut (*police).tcfp_lock); goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch); new = rcu_replace_pointer(&mut (*police).params, new, lockdep_is_held(&(*police).tcf_lock)); spin_unlock_bh(&mut (*police).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } if !new.is_null() { kfree_rcu(new, rcu); } return ret;
failure: qdisc_put_rtab(p_tab); qdisc_put_rtab(r_tab); if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
release_idr: tcf_idr_release(*a, bind); err
}

unsafe fn tcf_police_mtu_check(skb: *mut sk_buff, limit: u32) -> bool {
    if skb_is_gso(skb) { return skb_gso_validate_mac_len(skb, limit); }
    let mut len = qdisc_pkt_len(skb); if skb_at_tc_ingress(skb) { len += (*skb).mac_len as u32; } len <= limit
}

// Remaining callbacks preserve the C interfaces and delegate to the corresponding kernel helpers.
unsafe fn tcf_police_cleanup(a: *mut tc_action) { let p = (*to_police(a)).params; if !p.is_null() { kfree_rcu(p, rcu); } }

unsafe fn tcf_police_act(skb: *mut sk_buff, a: *const tc_action, _res: *mut tcf_result) -> c_int {
    let police = to_police(a); tcf_lastuse_update(&mut (*police).tcf_tm); bstats_update(this_cpu_ptr((*police).common.cpu_bstats), skb);
    let p = rcu_dereference_bh((*police).params); let mut ret = (*p).action;
    if (*p).tcfp_ewma_rate != 0 { let mut sample = gnet_stats_rate_est64 { }; if !gen_estimator_read(&(*police).tcf_rate_est, &mut sample) || sample.bps >= (*p).tcfp_ewma_rate as u64 { qstats_cpu_overlimit_inc((*police).common.cpu_qstats); if ret == TC_ACT_SHOT { qstats_cpu_drop_inc((*police).common.cpu_qstats); } return ret; } }
    if tcf_police_mtu_check(skb, (*p).tcfp_mtu) {
        if !(*p).rate_present && !(*p).pps_present { return (*p).tcfp_result as c_int; }
        let now = ktime_get_ns(); spin_lock_bh(&mut (*police).tcfp_lock);
        let mut toks = core::cmp::min(now - (*police).tcfp_t_c, (*p).tcfp_burst as i64); let mut ptoks = 0i64; let mut ppstoks = 0i64;
        if (*p).peak_present { ptoks = toks + (*police).tcfp_ptoks; if ptoks > (*p).tcfp_mtu_ptoks { ptoks = (*p).tcfp_mtu_ptoks; } ptoks -= psched_l2t_ns(&(*p).peak, qdisc_pkt_len(skb)) as i64; }
        if (*p).rate_present { toks += (*police).tcfp_toks; if toks > (*p).tcfp_burst as i64 { toks = (*p).tcfp_burst as i64; } toks -= psched_l2t_ns(&(*p).rate, qdisc_pkt_len(skb)) as i64; } else if (*p).pps_present { ppstoks = core::cmp::min(now - (*police).tcfp_t_c, (*p).tcfp_pkt_burst as i64) + (*police).tcfp_pkttoks; if ppstoks > (*p).tcfp_pkt_burst as i64 { ppstoks = (*p).tcfp_pkt_burst as i64; } ppstoks -= psched_pkt2t_ns(&(*p).ppsrate, 1) as i64; }
        if (toks | ptoks | ppstoks) >= 0 { (*police).tcfp_t_c=now; (*police).tcfp_toks=toks; (*police).tcfp_ptoks=ptoks; (*police).tcfp_pkttoks=ppstoks; spin_unlock_bh(&mut (*police).tcfp_lock); ret=(*p).tcfp_result as c_int; qstats_cpu_overlimit_inc((*police).common.cpu_qstats); if ret==TC_ACT_SHOT { qstats_cpu_drop_inc((*police).common.cpu_qstats); } return ret; }
        spin_unlock_bh(&mut (*police).tcfp_lock);
    }
    qstats_cpu_overlimit_inc((*police).common.cpu_qstats); if ret == TC_ACT_SHOT { qstats_cpu_drop_inc((*police).common.cpu_qstats); } ret
}

unsafe fn tcf_police_stats_update(a:*mut tc_action, bytes:u64, packets:u64, drops:u64, lastuse:u64, hw:bool) { let p=to_police(a); tcf_action_update_stats(a,bytes,packets,drops,hw); (*p).tcf_tm.lastuse=core::cmp::max((*p).tcf_tm.lastuse,lastuse); }

unsafe fn tcf_police_get_fill_size(_a:*const tc_action)->usize { nla_total_size(core::mem::size_of::<tc_police>()) + 4*nla_total_size_64bit(core::mem::size_of::<u64>()) + 2*nla_total_size(core::mem::size_of::<u32>()) }

MODULE_AUTHOR!("Alexey Kuznetsov");
MODULE_DESCRIPTION!("Policing actions");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
