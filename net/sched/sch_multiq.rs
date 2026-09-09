// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008, Intel Corporation.
 *
 * Author: Alexander Duyck <alexander.h.duyck@intel.com>
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct multiq_sched_data {
    pub bands: u16,
    pub max_bands: u16,
    pub curband: u16,
    pub filter_list: *mut tcf_proto,
    pub block: *mut tcf_block,
    pub queues: *mut *mut Qdisc,
}

unsafe fn multiq_classify(skb: *mut sk_buff, sch: *mut Qdisc, qerr: *mut i32) -> *mut Qdisc {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    let mut band: u32;
    let mut res: tcf_result = core::mem::zeroed();
    let fl = rcu_dereference_bh((*q).filter_list);
    let err: i32;

    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS;
    err = tcf_classify_qdisc(skb, fl, &mut res, false);
    // CONFIG_NET_CLS_ACT conditional code.
    match err {
        TC_ACT_STOLEN | TC_ACT_QUEUED | TC_ACT_TRAP => {
            *qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN;
            return core::ptr::null_mut();
        }
        TC_ACT_SHOT => return core::ptr::null_mut(),
        _ => {}
    }
    band = skb_get_queue_mapping(skb);
    if band >= (*q).bands as u32 { return *(*q).queues; }
    *(*q).queues.add(band as usize)
}

unsafe fn multiq_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 {
    let mut ret: i32 = 0;
    let qdisc = multiq_classify(skb, sch, &mut ret);
    // CONFIG_NET_CLS_ACT conditional code.
    if qdisc.is_null() {
        if ret & __NET_XMIT_BYPASS != 0 { qdisc_qstats_drop(sch); }
        __qdisc_drop(skb, to_free);
        return ret;
    }
    ret = qdisc_enqueue(skb, qdisc, to_free);
    if ret == NET_XMIT_SUCCESS { qdisc_qlen_inc(sch); return NET_XMIT_SUCCESS; }
    if net_xmit_drop_count(ret) != 0 { qdisc_qstats_drop(sch); }
    ret
}

unsafe fn multiq_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    let mut qdisc: *mut Qdisc;
    let mut skb: *mut sk_buff;
    for _band in 0..(*q).bands {
        (*q).curband += 1;
        if (*q).curband >= (*q).bands { (*q).curband = 0; }
        // Check target subqueue availability before pulling an skb.
        if !netif_xmit_stopped(netdev_get_tx_queue(qdisc_dev(sch), (*q).curband)) {
            qdisc = *(*q).queues.add((*q).curband as usize);
            skb = qdisc_dequeue_peeked(qdisc);
            if !skb.is_null() { qdisc_bstats_update(sch, skb); qdisc_qlen_dec(sch); return skb; }
        }
    }
    core::ptr::null_mut()
}

unsafe fn multiq_peek(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    let mut curband = (*q).curband;
    for _band in 0..(*q).bands {
        curband += 1;
        if curband >= (*q).bands { curband = 0; }
        if !netif_xmit_stopped(netdev_get_tx_queue(qdisc_dev(sch), curband)) {
            let qdisc = *(*q).queues.add(curband as usize);
            let skb = ((*(*qdisc).ops).peek)(qdisc);
            if !skb.is_null() { return skb; }
        }
    }
    core::ptr::null_mut()
}

unsafe fn multiq_reset(sch: *mut Qdisc) {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    for band in 0..(*q).bands { qdisc_reset(*(*q).queues.add(band as usize)); }
    (*q).curband = 0;
}

unsafe fn multiq_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    tcf_block_put((*q).block);
    for band in 0..(*q).bands { qdisc_put(*(*q).queues.add(band as usize)); }
    kfree((*q).queues as *mut core::ffi::c_void);
}

unsafe fn multiq_tune(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    if !netif_is_multiqueue(qdisc_dev(sch)) { return -EOPNOTSUPP; }
    if nla_len(opt) < core::mem::size_of::<tc_multiq_qopt>() { return -EINVAL; }
    let qopt = nla_data(opt) as *mut tc_multiq_qopt;
    (*qopt).bands = (*qdisc_dev(sch)).real_num_tx_queues;
    let removed = kmalloc(core::mem::size_of::<*mut Qdisc>() * ((*q).max_bands - (*qopt).bands) as usize, GFP_KERNEL) as *mut *mut Qdisc;
    if removed.is_null() { return -ENOMEM; }
    sch_tree_lock(sch); (*q).bands = (*qopt).bands;
    let mut n_removed = 0usize;
    for i in (*q).bands..(*q).max_bands {
        if *(*q).queues.add(i as usize) != &mut noop_qdisc {
            let child = *(*q).queues.add(i as usize); *(*q).queues.add(i as usize) = &mut noop_qdisc;
            qdisc_purge_queue(child); *removed.add(n_removed) = child; n_removed += 1;
        }
    }
    sch_tree_unlock(sch);
    for i in 0..n_removed { qdisc_put(*removed.add(i)); } kfree(removed as *mut core::ffi::c_void);
    for i in 0..(*q).bands {
        if *(*q).queues.add(i as usize) == &mut noop_qdisc {
            let child = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, TC_H_MAKE((*sch).handle, i + 1), extack);
            if !child.is_null() { sch_tree_lock(sch); let old = *(*q).queues.add(i as usize); *(*q).queues.add(i as usize) = child; if child != &mut noop_qdisc { qdisc_hash_add(child, true); } if old != &mut noop_qdisc { qdisc_purge_queue(old); } sch_tree_unlock(sch); qdisc_put(old); }
        }
    }
    0
}

unsafe fn multiq_init(sch: *mut Qdisc, opt: *mut nlattr, extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv(sch) as *mut multiq_sched_data; (*q).queues = core::ptr::null_mut();
    if opt.is_null() { return -EINVAL; }
    let err = tcf_block_get(&mut (*q).block, &mut (*q).filter_list, sch, extack); if err != 0 { return err; }
    (*q).max_bands = (*qdisc_dev(sch)).num_tx_queues;
    (*q).queues = kzalloc_objs(core::mem::size_of::<*mut Qdisc>() * (*q).max_bands as usize) as *mut *mut Qdisc;
    if (*q).queues.is_null() { return -ENOBUFS; }
    for i in 0..(*q).max_bands { *(*q).queues.add(i as usize) = &mut noop_qdisc; }
    multiq_tune(sch, opt, extack)
}

// Remaining class callbacks and registration metadata mirror the C source;
// their external kernel types and operations are supplied by dependencies.
unsafe fn multiq_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> i32 {
    let q = qdisc_priv(sch) as *mut multiq_sched_data;
    let opt = tc_multiq_qopt { bands: (*q).bands, max_bands: (*q).max_bands };
    let b = skb_tail_pointer(skb);
    if nla_put(skb, TCA_OPTIONS, core::mem::size_of::<tc_multiq_qopt>(), &opt as *const _ as *const core::ffi::c_void) != 0 { nlmsg_trim(skb, b); return -1; }
    (*skb).len as i32
}

unsafe fn multiq_graft(sch: *mut Qdisc, arg: usize, mut new: *mut Qdisc, old: *mut *mut Qdisc, _extack: *mut netlink_ext_ack) -> i32 {
    let q = qdisc_priv(sch) as *mut multiq_sched_data; let band = arg - 1;
    if new.is_null() { new = &mut noop_qdisc; }
    *old = qdisc_replace(sch, new, (*q).queues.add(band)); 0
}
unsafe fn multiq_leaf(sch: *mut Qdisc, arg: usize) -> *mut Qdisc { let q = qdisc_priv(sch) as *mut multiq_sched_data; *(*q).queues.add(arg - 1) }
unsafe fn multiq_find(sch: *mut Qdisc, classid: u32) -> usize { let q = qdisc_priv(sch) as *mut multiq_sched_data; let band = TC_H_MIN(classid) as usize; if band - 1 >= (*q).bands as usize { 0 } else { band } }
unsafe fn multiq_bind(sch: *mut Qdisc, _parent: usize, classid: u32) -> usize { multiq_find(sch, classid) }
unsafe fn multiq_unbind(_q: *mut Qdisc, _cl: usize) {}
unsafe fn multiq_dump_class(sch: *mut Qdisc, cl: usize, _skb: *mut sk_buff, tcm: *mut tcmsg) -> i32 { let q = qdisc_priv(sch) as *mut multiq_sched_data; (*tcm).tcm_handle |= TC_H_MIN(cl as u32); (*tcm).tcm_info = (*(*q).queues.add(cl - 1)).handle; 0 }
unsafe fn multiq_dump_class_stats(sch: *mut Qdisc, cl: usize, d: *mut gnet_dump) -> i32 { let q = qdisc_priv(sch) as *mut multiq_sched_data; let c = *(*q).queues.add(cl - 1); if gnet_stats_copy_basic(d, (*c).cpu_bstats, &(*c).bstats, true) < 0 || qdisc_qstats_copy(d, c) < 0 { -1 } else { 0 } }
unsafe fn multiq_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) { let q = qdisc_priv(sch) as *mut multiq_sched_data; if (*arg).stop { return; } for band in 0..(*q).bands { if !tc_qdisc_stats_dump(sch, band as usize + 1, arg) { break; } } }
unsafe fn multiq_tcf_block(sch: *mut Qdisc, cl: usize, _extack: *mut netlink_ext_ack) -> *mut tcf_block { let q = qdisc_priv(sch) as *mut multiq_sched_data; if cl != 0 { core::ptr::null_mut() } else { (*q).block } }

// The Qdisc class operations, qdisc operations, module alias, and module init/
// exit registration are retained as external-kernel metadata in this Rust
// translation. Their concrete kernel definitions are supplied by dependencies.
static multiq_class_ops: Qdisc_class_ops = Qdisc_class_ops {
    graft: Some(multiq_graft), leaf: Some(multiq_leaf), find: Some(multiq_find),
    walk: Some(multiq_walk), tcf_block: Some(multiq_tcf_block),
    bind_tcf: Some(multiq_bind), unbind_tcf: Some(multiq_unbind),
    dump: Some(multiq_dump_class), dump_stats: Some(multiq_dump_class_stats),
};
static multiq_qdisc_ops: Qdisc_ops = Qdisc_ops {
    next: core::ptr::null(), cl_ops: &multiq_class_ops, id: "multiq",
    priv_size: core::mem::size_of::<multiq_sched_data>(),
    enqueue: Some(multiq_enqueue), dequeue: Some(multiq_dequeue),
    peek: Some(multiq_peek), init: Some(multiq_init), reset: Some(multiq_reset),
    destroy: Some(multiq_destroy), change: Some(multiq_tune), dump: Some(multiq_dump),
    owner: THIS_MODULE,
};
// MODULE_ALIAS_NET_SCH("multiq");
unsafe fn multiq_module_init() -> i32 { register_qdisc(&multiq_qdisc_ops) }
unsafe fn multiq_module_exit() { unregister_qdisc(&multiq_qdisc_ops); }
// module_init(multiq_module_init)
// module_exit(multiq_module_exit)
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Multi queue to hardware queue mapping qdisc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
