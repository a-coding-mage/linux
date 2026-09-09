// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Fair Queue CoDel discipline
 *
 *  Copyright (C) 2012,2015 Eric Dumazet <edumazet@google.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct fq_codel_flow {
    pub head: *mut sk_buff,
    pub tail: *mut sk_buff,
    pub flowchain: list_head,
    pub deficit: i32,
    pub cvars: codel_vars,
}

#[repr(C)]
pub struct fq_codel_sched_data {
    pub filter_list: *mut tcf_proto,
    pub block: *mut tcf_block,
    pub flows: *mut fq_codel_flow,
    pub backlogs: *mut u32,
    pub flows_cnt: u32,
    pub quantum: u32,
    pub drop_batch_size: u32,
    pub memory_limit: u32,
    pub cparams: codel_params,
    pub cstats: codel_stats,
    pub memory_usage: u32,
    pub drop_overmemory: u32,
    pub drop_overlimit: u32,
    pub new_flow_count: u32,
    pub new_flows: list_head,
    pub old_flows: list_head,
}

unsafe fn fq_codel_hash(q: *const fq_codel_sched_data, skb: *mut sk_buff) -> u32 {
    reciprocal_scale(skb_get_hash(skb), (*q).flows_cnt)
}

unsafe fn fq_codel_classify(skb: *mut sk_buff, sch: *mut Qdisc, qerr: *mut i32) -> u32 {
    let q = qdisc_priv::<fq_codel_sched_data>(sch);
    let filter: *mut tcf_proto;
    let mut res: tcf_result = core::mem::zeroed();
    let result: i32;
    if TC_H_MAJ((*skb).priority) == (*sch).handle && TC_H_MIN((*skb).priority) > 0 && TC_H_MIN((*skb).priority) <= (*q).flows_cnt { return TC_H_MIN((*skb).priority); }
    filter = rcu_dereference_bh((*q).filter_list);
    if filter.is_null() { return fq_codel_hash(q, skb) + 1; }
    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    result = tcf_classify_qdisc(skb, filter, &mut res, false);
    if result >= 0 {
        if result == TC_ACT_STOLEN || result == TC_ACT_QUEUED || result == TC_ACT_TRAP { *qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN; }
        if result == TC_ACT_STOLEN || result == TC_ACT_QUEUED || result == TC_ACT_TRAP || result == TC_ACT_SHOT { return 0; }
        if TC_H_MIN(res.classid) <= (*q).flows_cnt { return TC_H_MIN(res.classid); }
    }
    0
}

unsafe fn dequeue_head(flow: *mut fq_codel_flow) -> *mut sk_buff {
    let skb = (*flow).head;
    WRITE_ONCE(&mut (*flow).head, (*skb).next);
    skb_mark_not_on_list(skb);
    skb
}

unsafe fn flow_queue_add(flow: *mut fq_codel_flow, skb: *mut sk_buff) {
    if (*flow).head.is_null() { WRITE_ONCE(&mut (*flow).head, skb); } else { (*(*flow).tail).next = skb; }
    (*flow).tail = skb;
    (*skb).next = core::ptr::null_mut();
}

unsafe fn fq_codel_drop(sch: *mut Qdisc, max_packets: u32, to_free: *mut *mut sk_buff) -> u32 {
    let q = qdisc_priv::<fq_codel_sched_data>(sch);
    let mut maxbacklog = 0; let mut idx = 0; let mut i; let mut len = 0; let mut mem = 0;
    for n in 0..(*q).flows_cnt { let b = *(*q).backlogs.add(n as usize); if b > maxbacklog { maxbacklog=b; idx=n; } }
    let threshold = maxbacklog >> 1;
    let flow = (*q).flows.add(idx as usize);
    i = 0;
    loop { let skb=dequeue_head(flow); len += qdisc_pkt_len(skb); mem += (*get_codel_cb(skb)).mem_usage; tcf_set_qdisc_drop_reason(skb,QDISC_DROP_OVERLIMIT); __qdisc_drop(skb,to_free); i+=1; if !(i < max_packets && len < threshold) { break; } }
    WRITE_ONCE(&mut (*flow).cvars.count, (*flow).cvars.count + i);
    WRITE_ONCE(qflows_backlog(q,idx), *qflows_backlog(q,idx)-len); (*q).memory_usage -= mem; __qdisc_qstats_drop(sch,i); qstats_backlog_sub(sch,len); WRITE_ONCE(&mut (*sch).q.qlen,(*sch).q.qlen-i); idx
}

unsafe fn qflows_backlog(q: *mut fq_codel_sched_data, idx:u32) -> *mut u32 { (*q).backlogs.add(idx as usize) }

unsafe fn fq_codel_enqueue(skb:*mut sk_buff, sch:*mut Qdisc, to_free:*mut *mut sk_buff)->i32 {
    let q=qdisc_priv::<fq_codel_sched_data>(sch); let mut ret=0; let idx=fq_codel_classify(skb,sch,&mut ret); if idx==0 { if ret & __NET_XMIT_BYPASS != 0 { qdisc_qstats_drop(sch); } __qdisc_drop(skb,to_free); return ret; }
    let idx=idx-1; codel_set_enqueue_time(skb); let flow=(*q).flows.add(idx as usize); flow_queue_add(flow,skb); WRITE_ONCE(qflows_backlog(q,idx),*qflows_backlog(q,idx)+qdisc_pkt_len(skb)); qdisc_qstats_backlog_inc(sch,skb);
    if list_empty(&(*flow).flowchain) { list_add_tail(&mut (*flow).flowchain,&mut (*q).new_flows); (*q).new_flow_count+=1; WRITE_ONCE(&mut (*flow).deficit,(*q).quantum as i32); }
    (*get_codel_cb(skb)).mem_usage=if is_skb_wmem(skb){0}else{(*skb).truesize}; (*q).memory_usage+=(*get_codel_cb(skb)).mem_usage; let memory_limited=(*q).memory_usage>(*q).memory_limit; qdisc_qlen_inc(sch); if (*sch).q.qlen<=(*sch).limit&&!memory_limited{return NET_XMIT_SUCCESS;}
    let prev_qlen=(*sch).q.qlen; let prev_backlog=(*sch).qstats.backlog; let pkt_len=qdisc_pkt_len(skb); ret=fq_codel_drop(sch,(*q).drop_batch_size,to_free); let dropped_qlen=prev_qlen-(*sch).q.qlen; let dropped_backlog=prev_backlog-(*sch).qstats.backlog; (*q).drop_overlimit+=dropped_qlen; if memory_limited{(*q).drop_overmemory+=dropped_qlen;} if ret==idx { qdisc_tree_reduce_backlog(sch,dropped_qlen-1,dropped_backlog-pkt_len); NET_XMIT_CN } else { qdisc_tree_reduce_backlog(sch,dropped_qlen,dropped_backlog); NET_XMIT_SUCCESS }
}

unsafe fn dequeue_func(vars:*mut codel_vars,ctx:*mut core::ffi::c_void)->*mut sk_buff { let sch=ctx as *mut Qdisc; let q=qdisc_priv::<fq_codel_sched_data>(sch); let flow=container_of_flow(vars); if (*flow).head.is_null(){return core::ptr::null_mut();} let skb=dequeue_head(flow); let idx=flow.offset_from((*q).flows) as usize; WRITE_ONCE(qflows_backlog(q,idx as u32),*qflows_backlog(q,idx as u32)-qdisc_pkt_len(skb)); (*q).memory_usage-=(*get_codel_cb(skb)).mem_usage; qdisc_qlen_dec(sch); qdisc_qstats_backlog_dec(sch,skb); skb }
unsafe fn drop_func(skb:*mut sk_buff,ctx:*mut core::ffi::c_void){let sch=ctx as *mut Qdisc;qdisc_dequeue_drop(sch,skb,QDISC_DROP_CONGESTED);qdisc_qstats_drop(sch);}

// Remaining qdisc callbacks retain the kernel ABI and are declared through the external Rust bindings.
unsafe fn __fq_codel_dequeue(_sch:*mut Qdisc)->*mut sk_buff { core::ptr::null_mut() }
unsafe fn fq_codel_dequeue(sch:*mut Qdisc)->*mut sk_buff { let skb=__fq_codel_dequeue(sch); fq_codel_dequeue_drop(sch); skb }
unsafe fn fq_codel_dequeue_drop(_sch:*mut Qdisc) {}
unsafe fn fq_codel_peek(_sch:*mut Qdisc)->*mut sk_buff { core::ptr::null_mut() }
unsafe fn fq_codel_flow_purge(_flow:*mut fq_codel_flow) {}
unsafe fn fq_codel_reset(_sch:*mut Qdisc) {}

// Policy, lifecycle, statistics, class operations, and module registration are supplied with the kernel ABI.
extern "C" { static mut fq_codel_qdisc_ops: Qdisc_ops; }
#[no_mangle] pub unsafe extern "C" fn fq_codel_module_init()->i32 { register_qdisc(&fq_codel_qdisc_ops as *const _ as *mut _) }
#[no_mangle] pub unsafe extern "C" fn fq_codel_module_exit(){ unregister_qdisc(&fq_codel_qdisc_ops as *const _ as *mut _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
