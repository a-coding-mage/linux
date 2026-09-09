// SPDX-License-Identifier: GPL-2.0-only
/* Flow Queue PIE discipline
 *
 * Copyright (C) 2019 Mohit P. Tahiliani <tahiliani@nitk.edu.in>
 * Copyright (C) 2019 Sachin D. Patil <sdp.sachin@gmail.com>
 * Copyright (C) 2019 V. Saicharan <vsaicharan1998@gmail.com>
 * Copyright (C) 2019 Mohit Bhasi <mohitbhasi1998@gmail.com>
 * Copyright (C) 2019 Leslie Monis <lesliemonis@gmail.com>
 * Copyright (C) 2019 Gautam Ramakrishnan <gautamramk@gmail.com>
 */

// Kernel dependencies supplied by the surrounding crate.

#[repr(C)]
pub struct fq_pie_flow {
    pub vars: pie_vars,
    pub deficit: i32,
    pub backlog: u32,
    pub qlen: u32,
    pub flowchain: list_head,
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
}

#[repr(C)]
pub struct fq_pie_sched_data {
    pub filter_list: *mut tcf_proto,
    pub block: *mut tcf_block,
    pub flows: *mut fq_pie_flow,
    pub sch: *mut Qdisc,
    pub old_flows: list_head,
    pub new_flows: list_head,
    pub p_params: pie_params,
    pub ecn_prob: u32,
    pub flows_cnt: u32,
    pub flows_cursor: u32,
    pub quantum: u32,
    pub memory_limit: u32,
    pub new_flow_count: u32,
    pub memory_usage: u32,
    pub overmemory: u32,
    pub stats: pie_stats,
    pub adapt_timer: timer_list,
}

unsafe fn fq_pie_hash(q: *const fq_pie_sched_data, skb: *mut sk_buff) -> u32 {
    reciprocal_scale(skb_get_hash(skb), (*q).flows_cnt)
}

unsafe fn fq_pie_classify(skb: *mut sk_buff, sch: *mut Qdisc, qerr: *mut i32) -> u32 {
    let q = qdisc_priv::<fq_pie_sched_data>(sch);
    let mut res: tcf_result = core::mem::zeroed();
    let priority = (*skb).priority;
    if TC_H_MAJ(priority) == (*sch).handle && TC_H_MIN(priority) > 0 && TC_H_MIN(priority) <= (*q).flows_cnt {
        return TC_H_MIN(priority);
    }
    let filter = rcu_dereference_bh((*q).filter_list);
    if filter.is_null() { return fq_pie_hash(q, skb).wrapping_add(1); }
    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    let result = tcf_classify_qdisc(skb, filter, &mut res, false);
    if result >= 0 {
        // CONFIG_NET_CLS_ACT conditional is supplied by the build configuration.
        if result == TC_ACT_STOLEN || result == TC_ACT_QUEUED || result == TC_ACT_TRAP {
            *qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
            return 0;
        }
        if result == TC_ACT_SHOT { return 0; }
        if TC_H_MIN(res.classid) <= (*q).flows_cnt { return TC_H_MIN(res.classid); }
    }
    0
}

unsafe fn flow_queue_add(flow: *mut fq_pie_flow, skb: *mut sk_buff) {
    if (*flow).head.is_null() { (*flow).head = skb; } else { (*(*flow).tail).next = skb; }
    (*flow).tail = skb;
    (*skb).next = core::ptr::null_mut();
}

unsafe fn fq_pie_qdisc_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let mut reason = QDISC_DROP_OVERLIMIT;
    let q = qdisc_priv::<fq_pie_sched_data>(sch);
    let mut ret = 0;
    let mut enqueue = false;
    let idx = fq_pie_classify(skb, sch, &mut ret);
    if idx == 0 {
        if ret & __NET_XMIT_BYPASS != 0 { qdisc_qstats_drop(sch); }
        __qdisc_drop(skb, to_free); return ret;
    }
    let flow = (*q).flows.add((idx - 1) as usize);
    (*get_pie_cb(skb)).mem_usage = (*skb).truesize;
    let memory_limited = (*q).memory_usage > (*q).memory_limit + (*skb).truesize;
    if qdisc_qlen(sch) >= (*sch).limit { (*q).stats.overlimit += 1; goto_out(q, flow, skb, sch, to_free, reason); return NET_XMIT_CN; }
    if memory_limited { (*q).overmemory += 1; }
    reason = QDISC_DROP_CONGESTED;
    if !pie_drop_early(sch, &(*q).p_params, &(*flow).vars, (*flow).backlog, (*skb).len) {
        enqueue = true;
    } else if (*q).p_params.ecn && (*flow).vars.prob <= (MAX_PROB / 100) * (*q).ecn_prob && INET_ECN_set_ce(skb) {
        (*q).stats.ecn_mark += 1; enqueue = true;
    }
    if enqueue {
        if !(*q).p_params.dq_rate_estimator { pie_set_enqueue_time(skb); }
        let pkt_len = qdisc_pkt_len(skb);
        (*q).stats.packets_in += 1; (*q).memory_usage += (*skb).truesize;
        qstats_backlog_add(sch, pkt_len); qdisc_qlen_inc(sch); flow_queue_add(flow, skb);
        if list_empty(&(*flow).flowchain) { list_add_tail(&mut (*flow).flowchain, &mut (*q).new_flows); (*q).new_flow_count += 1; (*flow).deficit = (*q).quantum as i32; (*flow).qlen = 0; (*flow).backlog = 0; }
        (*flow).qlen += 1; (*flow).backlog += pkt_len; return NET_XMIT_SUCCESS;
    }
    goto_out(q, flow, skb, sch, to_free, reason); NET_XMIT_CN
}

unsafe fn goto_out(q: *mut fq_pie_sched_data, flow: *mut fq_pie_flow, skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff, reason: qdisc_drop_reason) {
    (*q).stats.dropped += 1; (*flow).vars.accu_prob = 0; qdisc_drop_reason(skb, sch, to_free, reason);
}

unsafe fn dequeue_head(flow: *mut fq_pie_flow) -> *mut sk_buff {
    let skb = (*flow).head; (*flow).head = (*skb).next; (*skb).next = core::ptr::null_mut(); skb
}

unsafe fn fq_pie_qdisc_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<fq_pie_sched_data>(sch);
    loop {
        let mut head = &mut (*q).new_flows as *mut list_head;
        if list_empty(head) { head = &mut (*q).old_flows; if list_empty(head) { return core::ptr::null_mut(); } }
        let flow = list_first_entry::<fq_pie_flow>(head);
        if (*flow).deficit <= 0 { (*flow).deficit += (*q).quantum as i32; list_move_tail(&mut (*flow).flowchain, &mut (*q).old_flows); continue; }
        let mut skb = core::ptr::null_mut();
        if !(*flow).head.is_null() { skb = dequeue_head(flow); let len = qdisc_pkt_len(skb); qstats_backlog_sub(sch, len); qdisc_qlen_dec(sch); qdisc_bstats_update(sch, skb); }
        if skb.is_null() { if head == &mut (*q).new_flows && !list_empty(&(*q).old_flows) { list_move_tail(&mut (*flow).flowchain, &mut (*q).old_flows); } else { list_del_init(&mut (*flow).flowchain); } continue; }
        let len = qdisc_pkt_len(skb); (*flow).qlen -= 1; (*flow).deficit -= len as i32; (*flow).backlog -= len; (*q).memory_usage -= (*get_pie_cb(skb)).mem_usage; pie_process_dequeue(skb, &(*q).p_params, &mut (*flow).vars, (*flow).backlog); return skb;
    }
}

#[repr(C)] pub struct netlink_range_validation { pub min: u32, pub max: u32 }
pub static fq_pie_q_range: netlink_range_validation = netlink_range_validation { min: 1, max: 1 << 20 };
unsafe fn fq_pie_policy() -> *const nla_policy { core::ptr::null() }

unsafe fn fq_pie_timer(t: *mut timer_list) { let q=timer_container_of::<fq_pie_sched_data>(t, "adapt_timer"); let sch=(*q).sch; rcu_read_lock(); let root_lock=qdisc_lock(qdisc_root_sleeping(sch)); spin_lock(root_lock); let remaining=(*q).flows_cnt.wrapping_sub((*q).flows_cursor); let max_cnt=if remaining<2048{remaining}else{2048}; for _ in 0..max_cnt { let f=(*q).flows.add((*q).flows_cursor as usize); pie_calculate_probability(&(*q).p_params,&mut (*f).vars,(*f).backlog); (*q).flows_cursor+=1; } let tupdate=(*q).p_params.tupdate; let mut next=0; if (*q).flows_cursor>=(*q).flows_cnt {(*q).flows_cursor=0;next=tupdate;} if tupdate!=0 {mod_timer(&mut (*q).adapt_timer,jiffies+next);} spin_unlock(root_lock);rcu_read_unlock(); }

unsafe fn fq_pie_change(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv::<fq_pie_sched_data>(sch); let mut tb: [*mut nlattr; TCA_FQ_PIE_MAX as usize + 1] = [core::ptr::null_mut(); TCA_FQ_PIE_MAX as usize + 1];
    let err = nla_parse_nested(tb.as_mut_ptr(), TCA_FQ_PIE_MAX, opt, fq_pie_policy(), extack); if err < 0 { return err; }
    sch_tree_lock(sch);
    if !tb[TCA_FQ_PIE_LIMIT as usize].is_null() { let v = nla_get_u32(tb[TCA_FQ_PIE_LIMIT as usize]); WRITE_ONCE!((*q).p_params.limit, v); WRITE_ONCE!((*sch).limit, v); }
    if !tb[TCA_FQ_PIE_FLOWS as usize].is_null() { if !(*q).flows.is_null() { NL_SET_ERR_MSG_MOD(extack, "Number of flows cannot be changed"); sch_tree_unlock(sch); return -EINVAL; } let v=nla_get_u32(tb[TCA_FQ_PIE_FLOWS as usize]); if v==0 || v>65536 { NL_SET_ERR_MSG_MOD(extack, "Number of flows must range in [1..65536]"); sch_tree_unlock(sch); return -EINVAL; } (*q).flows_cnt=v; }
    if !tb[TCA_FQ_PIE_TARGET as usize].is_null() { let v=nla_get_u32(tb[TCA_FQ_PIE_TARGET as usize]); WRITE_ONCE!((*q).p_params.target, PSCHED_NS2TICKS((v as u64)*NSEC_PER_USEC)); }
    if !tb[TCA_FQ_PIE_TUPDATE as usize].is_null() { WRITE_ONCE!((*q).p_params.tupdate, usecs_to_jiffies(nla_get_u32(tb[TCA_FQ_PIE_TUPDATE as usize]))); }
    if !tb[TCA_FQ_PIE_ALPHA as usize].is_null() { WRITE_ONCE!((*q).p_params.alpha, nla_get_u32(tb[TCA_FQ_PIE_ALPHA as usize])); }
    if !tb[TCA_FQ_PIE_BETA as usize].is_null() { WRITE_ONCE!((*q).p_params.beta, nla_get_u32(tb[TCA_FQ_PIE_BETA as usize])); }
    if !tb[TCA_FQ_PIE_QUANTUM as usize].is_null() { WRITE_ONCE!((*q).quantum, nla_get_u32(tb[TCA_FQ_PIE_QUANTUM as usize])); }
    if !tb[TCA_FQ_PIE_MEMORY_LIMIT as usize].is_null() { WRITE_ONCE!((*q).memory_limit, nla_get_u32(tb[TCA_FQ_PIE_MEMORY_LIMIT as usize])); }
    if !tb[TCA_FQ_PIE_ECN_PROB as usize].is_null() { WRITE_ONCE!((*q).ecn_prob, nla_get_u32(tb[TCA_FQ_PIE_ECN_PROB as usize])); }
    if !tb[TCA_FQ_PIE_ECN as usize].is_null() { WRITE_ONCE!((*q).p_params.ecn, nla_get_u32(tb[TCA_FQ_PIE_ECN as usize])); }
    if !tb[TCA_FQ_PIE_BYTEMODE as usize].is_null() { WRITE_ONCE!((*q).p_params.bytemode, nla_get_u32(tb[TCA_FQ_PIE_BYTEMODE as usize])); }
    if !tb[TCA_FQ_PIE_DQ_RATE_ESTIMATOR as usize].is_null() { WRITE_ONCE!((*q).p_params.dq_rate_estimator, nla_get_u32(tb[TCA_FQ_PIE_DQ_RATE_ESTIMATOR as usize])); }
    while (*sch).q.qlen > (*sch).limit { let skb=qdisc_dequeue_internal(sch,false); if skb.is_null(){break;} rtnl_kfree_skbs(skb,skb); }
    sch_tree_unlock(sch); 0
}

unsafe fn fq_pie_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 { let q=qdisc_priv::<fq_pie_sched_data>(sch); pie_params_init(&mut (*q).p_params); (*sch).limit=10*1024; (*q).p_params.limit=(*sch).limit; (*q).quantum=clamp_t(psched_mtu(qdisc_dev(sch)),256,1<<20); (*q).sch=sch; (*q).ecn_prob=10; (*q).flows_cnt=1024; (*q).memory_limit=SZ_32M; INIT_LIST_HEAD(&mut (*q).new_flows); INIT_LIST_HEAD(&mut (*q).old_flows); timer_setup(&mut (*q).adapt_timer,fq_pie_timer,0); if !opt.is_null(){let e=fq_pie_change(sch,opt,extack);if e!=0{return e;}} (*q).flows=kvzalloc_objs::<fq_pie_flow>((*q).flows_cnt); if (*q).flows.is_null(){return -ENOMEM;} for i in 0..(*q).flows_cnt { let f=(*q).flows.add(i as usize); INIT_LIST_HEAD(&mut (*f).flowchain); pie_vars_init(&mut (*f).vars); } mod_timer(&mut (*q).adapt_timer,jiffies+HZ/2); 0 }

unsafe fn fq_pie_reset(sch: *mut Qdisc) { let q=qdisc_priv::<fq_pie_sched_data>(sch); INIT_LIST_HEAD(&mut (*q).new_flows); INIT_LIST_HEAD(&mut (*q).old_flows); for i in 0..(*q).flows_cnt { let f=(*q).flows.add(i as usize); rtnl_kfree_skbs((*f).head,(*f).tail); (*f).head=core::ptr::null_mut(); INIT_LIST_HEAD(&mut (*f).flowchain); pie_vars_init(&mut (*f).vars); } }
unsafe fn fq_pie_destroy(sch:*mut Qdisc){let q=qdisc_priv::<fq_pie_sched_data>(sch);tcf_block_put((*q).block);(*q).p_params.tupdate=0;timer_delete_sync(&mut (*q).adapt_timer);kvfree((*q).flows);}
unsafe fn fq_pie_module_init()->i32{register_qdisc(&fq_pie_qdisc_ops)} unsafe fn fq_pie_module_exit(){unregister_qdisc(&fq_pie_qdisc_ops)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
