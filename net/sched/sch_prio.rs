// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/sch_prio.c - Simple 3-band priority scheduler. */

// External kernel types, constants, functions, and macros are supplied by the
// surrounding kernel bindings.

#[repr(C)]
struct PrioSchedData {
    bands: c_int,
    filter_list: *mut TcfProto,
    block: *mut TcfBlock,
    prio2band: [u8; TC_PRIO_MAX + 1],
    queues: [*mut Qdisc; TCQ_PRIO_BANDS],
}

unsafe fn prio_classify(skb: *mut SkBuff, sch: *mut Qdisc, qerr: *mut c_int) -> *mut Qdisc {
    let q = qdisc_priv::<PrioSchedData>(sch);
    let mut band: u32 = (*skb).priority;
    let mut res: TcfResult = core::mem::zeroed();
    let fl = rcu_dereference_bh((*q).filter_list);
    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    if tc_h_maj((*skb).priority) != (*sch).handle {
        let err = tcf_classify_qdisc(skb, fl, &mut res, false);
        #[cfg(CONFIG_NET_CLS_ACT)]
        {
            match err {
                TC_ACT_STOLEN | TC_ACT_QUEUED | TC_ACT_TRAP => {
                    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
                }
                TC_ACT_SHOT => return core::ptr::null_mut(),
                _ => {}
            }
        }
        if fl.is_null() || err < 0 {
            if tc_h_maj(band) != 0 { band = 0; }
            return (*q).queues[((*q).prio2band[(band & TC_PRIO_MAX as u32) as usize]) as usize];
        }
        band = res.classid;
    }
    band = tc_h_min(band).wrapping_sub(1);
    if band >= (*q).bands as u32 { return (*q).queues[(*q).prio2band[0] as usize]; }
    (*q).queues[band as usize]
}

unsafe fn prio_enqueue(skb: *mut SkBuff, sch: *mut Qdisc, to_free: *mut *mut SkBuff) -> c_int {
    let len = qdisc_pkt_len(skb);
    let mut ret = 0;
    let qdisc = prio_classify(skb, sch, &mut ret);
    #[cfg(CONFIG_NET_CLS_ACT)]
    if qdisc.is_null() {
        if ret & __NET_XMIT_BYPASS != 0 { qdisc_qstats_drop(sch); }
        __qdisc_drop(skb, to_free);
        return ret;
    }
    ret = qdisc_enqueue(skb, qdisc, to_free);
    if ret == NET_XMIT_SUCCESS {
        qstats_backlog_add(sch, len); qdisc_qlen_inc(sch); return NET_XMIT_SUCCESS;
    }
    if net_xmit_drop_count(ret) != 0 { qdisc_qstats_drop(sch); }
    ret
}

unsafe fn prio_peek(sch: *mut Qdisc) -> *mut SkBuff {
    let q = qdisc_priv::<PrioSchedData>(sch);
    for prio in 0..(*q).bands {
        let qdisc = (*q).queues[prio as usize];
        let skb = ((*(*qdisc).ops).peek.unwrap())(qdisc);
        if !skb.is_null() { return skb; }
    }
    core::ptr::null_mut()
}

unsafe fn prio_dequeue(sch: *mut Qdisc) -> *mut SkBuff {
    let q = qdisc_priv::<PrioSchedData>(sch);
    for prio in 0..(*q).bands {
        let qdisc = (*q).queues[prio as usize];
        let skb = qdisc_dequeue_peeked(qdisc);
        if !skb.is_null() {
            qdisc_bstats_update(sch, skb); qdisc_qstats_backlog_dec(sch, skb); qdisc_qlen_dec(sch); return skb;
        }
    }
    core::ptr::null_mut()
}

unsafe fn prio_reset(sch: *mut Qdisc) {
    let q = qdisc_priv::<PrioSchedData>(sch);
    for prio in 0..(*q).bands { qdisc_reset((*q).queues[prio as usize]); }
}

unsafe fn prio_offload(sch: *mut Qdisc, qopt: *mut TcPrioQopt) -> c_int {
    let dev = qdisc_dev(sch);
    let mut opt: TcPrioQoptOffload = core::mem::zeroed();
    opt.handle = (*sch).handle; opt.parent = (*sch).parent;
    if !tc_can_offload(dev) || (*(*dev).netdev_ops).ndo_setup_tc.is_none() { return -EOPNOTSUPP; }
    if !qopt.is_null() {
        opt.command = TC_PRIO_REPLACE; opt.replace_params.bands = (*qopt).bands;
        core::ptr::copy_nonoverlapping((*qopt).priomap.as_ptr(), opt.replace_params.priomap.as_mut_ptr(), TC_PRIO_MAX + 1);
        opt.replace_params.qstats = &mut (*sch).qstats;
    } else { opt.command = TC_PRIO_DESTROY; }
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_PRIO, &mut opt)
}

unsafe fn prio_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<PrioSchedData>(sch); tcf_block_put((*q).block); prio_offload(sch, core::ptr::null_mut());
    for prio in 0..(*q).bands { qdisc_put((*q).queues[prio as usize]); }
}

unsafe fn prio_tune(sch: *mut Qdisc, opt: *mut Nlattr, extack: *mut NetlinkExtAck) -> c_int {
    let q = qdisc_priv::<PrioSchedData>(sch); let mut queues: [*mut Qdisc; TCQ_PRIO_BANDS] = [core::ptr::null_mut(); TCQ_PRIO_BANDS];
    let oldbands = (*q).bands; if nla_len(opt) < core::mem::size_of::<TcPrioQopt>() { return -EINVAL; }
    let qopt = nla_data::<TcPrioQopt>(opt); if (*qopt).bands > TCQ_PRIO_BANDS as u8 || (*qopt).bands < TCQ_MIN_PRIO_BANDS as u8 { return -EINVAL; }
    for i in 0..=TC_PRIO_MAX { if (*qopt).priomap[i] >= (*qopt).bands { return -EINVAL; } }
    for i in oldbands..(*qopt).bands as c_int { queues[i as usize] = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, tc_h_make((*sch).handle, (i + 1) as u32), extack); if queues[i as usize].is_null() { let mut j=i; while j > oldbands { j-=1; qdisc_put(queues[j as usize]); } return -ENOMEM; } }
    prio_offload(sch, qopt); sch_tree_lock(sch); (*q).bands = (*qopt).bands as c_int; core::ptr::copy_nonoverlapping((*qopt).priomap.as_ptr(), (*q).prio2band.as_mut_ptr(), TC_PRIO_MAX+1);
    for i in (*q).bands..oldbands { qdisc_purge_queue((*q).queues[i as usize]); }
    for i in oldbands..(*q).bands { (*q).queues[i as usize] = queues[i as usize]; if (*q).queues[i as usize] != &noop_qdisc { qdisc_hash_add((*q).queues[i as usize], true); } }
    sch_tree_unlock(sch); for i in (*q).bands..oldbands { qdisc_put((*q).queues[i as usize]); } 0
}

unsafe fn prio_init(sch: *mut Qdisc, opt: *mut Nlattr, extack: *mut NetlinkExtAck) -> c_int {
    if opt.is_null() { return -EINVAL; } let q = qdisc_priv::<PrioSchedData>(sch); let err = tcf_block_get(&mut (*q).block, &mut (*q).filter_list, sch, extack); if err != 0 { return err; } prio_tune(sch, opt, extack)
}

unsafe fn prio_dump_offload(sch: *mut Qdisc) -> c_int { let mut hw_stats: TcPrioQoptOffload = core::mem::zeroed(); hw_stats.command=TC_PRIO_STATS; hw_stats.handle=(*sch).handle; hw_stats.parent=(*sch).parent; hw_stats.stats.bstats=&mut (*sch).bstats; hw_stats.stats.qstats=&mut (*sch).qstats; qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_PRIO, &mut hw_stats) }
unsafe fn prio_dump(sch: *mut Qdisc, skb: *mut SkBuff) -> c_int { let q=qdisc_priv::<PrioSchedData>(sch); let b=skb_tail_pointer(skb); let mut opt: TcPrioQopt=core::mem::zeroed(); opt.bands=(*q).bands as u8; core::ptr::copy_nonoverlapping((*q).prio2band.as_ptr(),opt.priomap.as_mut_ptr(),TC_PRIO_MAX+1); if prio_dump_offload(sch)!=0 || nla_put(skb,TCA_OPTIONS,core::mem::size_of::<TcPrioQopt>(),&opt)!=0 { nlmsg_trim(skb,b); return -1; } (*skb).len as c_int }

unsafe fn prio_graft(sch:*mut Qdisc,arg:usize,new:*mut Qdisc,old:*mut *mut Qdisc,extack:*mut NetlinkExtAck)->c_int { let q=qdisc_priv::<PrioSchedData>(sch); let band=arg-1; let mut child=new; if child.is_null(){child=qdisc_create_dflt((*sch).dev_queue,&pfifo_qdisc_ops,tc_h_make((*sch).handle,arg as u32),extack);if child.is_null(){child=&raw mut noop_qdisc}else{qdisc_hash_add(child,true);}} *old=qdisc_replace(sch,child,&mut (*q).queues[band]); let mut o:TcPrioQoptOffload=core::mem::zeroed();o.handle=(*sch).handle;o.parent=(*sch).parent;o.graft_params.band=band;o.graft_params.child_handle=(*child).handle;o.command=TC_PRIO_GRAFT;qdisc_offload_graft_helper(qdisc_dev(sch),sch,child,*old,TC_SETUP_QDISC_PRIO,&mut o,extack);0 }
unsafe fn prio_leaf(sch:*mut Qdisc,arg:usize)->*mut Qdisc { let q=qdisc_priv::<PrioSchedData>(sch);(*q).queues[arg-1] }
unsafe fn prio_find(sch:*mut Qdisc,classid:u32)->usize { let q=qdisc_priv::<PrioSchedData>(sch);let band=tc_h_min(classid) as usize;if band-1>=(*q).bands as usize{0}else{band} }
unsafe fn prio_bind(sch:*mut Qdisc,_parent:usize,classid:u32)->usize{prio_find(sch,classid)}
unsafe fn prio_unbind(_q:*mut Qdisc,_cl:usize){}
unsafe fn prio_dump_class(sch:*mut Qdisc,cl:usize,_skb:*mut SkBuff,tcm:*mut Tcmsg)->c_int{let q=qdisc_priv::<PrioSchedData>(sch);(*tcm).tcm_handle|=tc_h_min(cl as u32);(*tcm).tcm_info=(*q).queues[cl-1].as_ref().unwrap().handle;0}
unsafe fn prio_dump_class_stats(sch:*mut Qdisc,cl:usize,d:*mut GnetDump)->c_int{let q=qdisc_priv::<PrioSchedData>(sch);let c=(*q).queues[cl-1];if gnet_stats_copy_basic(d,(*c).cpu_bstats,&mut (*c).bstats,true)<0||qdisc_qstats_copy(d,c)<0{-1}else{0}}
unsafe fn prio_walk(sch:*mut Qdisc,arg:*mut QdiscWalker){let q=qdisc_priv::<PrioSchedData>(sch);if (*arg).stop{return;}for i in 0..(*q).bands{if !tc_qdisc_stats_dump(sch,i+1,arg){break;}}}
unsafe fn prio_tcf_block(sch:*mut Qdisc,cl:usize,_extack:*mut NetlinkExtAck)->*mut TcfBlock{let q=qdisc_priv::<PrioSchedData>(sch);if cl!=0{core::ptr::null_mut()}else{(*q).block}}
static mut PRIO_CLASS_OPS: QdiscClassOps = QdiscClassOps { graft: Some(prio_graft), leaf: Some(prio_leaf), find: Some(prio_find), walk: Some(prio_walk), tcf_block: Some(prio_tcf_block), bind_tcf: Some(prio_bind), unbind_tcf: Some(prio_unbind), dump: Some(prio_dump_class), dump_stats: Some(prio_dump_class_stats) };
static mut PRIO_QDISC_OPS: QdiscOps = QdiscOps { next: core::ptr::null_mut(), cl_ops: &raw mut PRIO_CLASS_OPS, id: b"prio\0".as_ptr() as *const _, priv_size: core::mem::size_of::<PrioSchedData>(), enqueue: Some(prio_enqueue), dequeue: Some(prio_dequeue), peek: Some(prio_peek), init: Some(prio_init), reset: Some(prio_reset), destroy: Some(prio_destroy), change: Some(prio_tune), dump: Some(prio_dump), owner: THIS_MODULE };
unsafe fn prio_module_init() -> c_int { register_qdisc(&raw mut PRIO_QDISC_OPS) }
unsafe fn prio_module_exit() { unregister_qdisc(&raw mut PRIO_QDISC_OPS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
