// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/sched/sch_drr.c         Deficit Round Robin scheduler
 *
 * Copyright (c) 2008 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel headers and build-time configuration supplied by dependencies.

#[repr(C)]
struct drr_class {
    common: Qdisc_class_common,
    bstats: gnet_stats_basic_sync,
    qstats: gnet_stats_queue,
    rate_est: *mut net_rate_estimator,
    alist: list_head,
    qdisc: *mut Qdisc,
    quantum: u32,
    deficit: u32,
}

#[repr(C)]
struct drr_sched {
    active: list_head,
    filter_list: *mut tcf_proto,
    block: *mut tcf_block,
    clhash: Qdisc_class_hash,
}

unsafe fn cl_is_active(cl: *mut drr_class) -> bool {
    !list_empty(&mut (*cl).alist)
}

unsafe fn drr_find_class(sch: *mut Qdisc, classid: u32) -> *mut drr_class {
    let q = qdisc_priv::<drr_sched>(sch);
    let clc = qdisc_class_find(&mut (*q).clhash, classid);
    if clc.is_null() {
        return core::ptr::null_mut();
    }
    container_of!(clc, drr_class, common)
}

static drr_policy: [nla_policy; TCA_DRR_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; TCA_DRR_MAX as usize + 1];
    policy[TCA_DRR_QUANTUM as usize] = nla_policy { type_: NLA_U32 };
    policy
};

unsafe fn drr_change_class(
    sch: *mut Qdisc, classid: u32, _parentid: u32, tca: *mut *mut nlattr,
    arg: *mut c_ulong, extack: *mut netlink_ext_ack,
) -> c_int {
    let q = qdisc_priv::<drr_sched>(sch);
    let mut cl = *arg as *mut drr_class;
    let opt = *tca.add(TCA_OPTIONS as usize);
    let mut tb = [core::ptr::null_mut(); TCA_DRR_MAX as usize + 1];
    let quantum: u32;
    let mut err: c_int;

    if opt.is_null() {
        NL_SET_ERR_MSG!(extack, "DRR options are required for this operation");
        return -EINVAL;
    }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_DRR_MAX, opt, drr_policy.as_ptr(), extack);
    if err < 0 { return err; }

    if !tb[TCA_DRR_QUANTUM as usize].is_null() {
        quantum = nla_get_u32(tb[TCA_DRR_QUANTUM as usize]);
        if quantum == 0 {
            NL_SET_ERR_MSG!(extack, "Specified DRR quantum cannot be zero");
            return -EINVAL;
        }
    } else {
        quantum = psched_mtu(qdisc_dev(sch));
    }

    if !cl.is_null() {
        if !(*tca.add(TCA_RATE as usize)).is_null() {
            err = gen_replace_estimator(&mut (*cl).bstats, core::ptr::null_mut(), &mut (*cl).rate_est,
                                         core::ptr::null_mut(), true, *tca.add(TCA_RATE as usize));
            if err != 0 {
                NL_SET_ERR_MSG!(extack, "Failed to replace estimator");
                return err;
            }
        }
        if !tb[TCA_DRR_QUANTUM as usize].is_null() { WRITE_ONCE!((*cl).quantum, quantum); }
        return 0;
    }

    cl = kzalloc_obj::<drr_class>();
    if cl.is_null() { return -ENOBUFS; }
    gnet_stats_basic_sync_init(&mut (*cl).bstats);
    INIT_LIST_HEAD(&mut (*cl).alist);
    (*cl).common.classid = classid;
    (*cl).quantum = quantum;
    (*cl).qdisc = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, classid, core::ptr::null_mut());
    if (*cl).qdisc.is_null() { (*cl).qdisc = &mut noop_qdisc; }
    else { qdisc_hash_add((*cl).qdisc, true); }
    if !(*tca.add(TCA_RATE as usize)).is_null() {
        err = gen_replace_estimator(&mut (*cl).bstats, core::ptr::null_mut(), &mut (*cl).rate_est,
                                     core::ptr::null_mut(), true, *tca.add(TCA_RATE as usize));
        if err != 0 {
            NL_SET_ERR_MSG!(extack, "Failed to replace estimator");
            qdisc_put((*cl).qdisc); kfree(cl); return err;
        }
    }
    sch_tree_lock(sch);
    qdisc_class_hash_insert(&mut (*q).clhash, &mut (*cl).common);
    sch_tree_unlock(sch);
    qdisc_class_hash_grow(sch, &mut (*q).clhash);
    *arg = cl as c_ulong;
    0
}

unsafe fn drr_destroy_class(_sch: *mut Qdisc, cl: *mut drr_class) {
    gen_kill_estimator(&mut (*cl).rate_est); qdisc_put((*cl).qdisc); kfree(cl);
}

unsafe fn drr_delete_class(sch: *mut Qdisc, arg: c_ulong, extack: *mut netlink_ext_ack) -> c_int {
    let q = qdisc_priv::<drr_sched>(sch); let cl = arg as *mut drr_class;
    if qdisc_class_in_use(&(*cl).common) { NL_SET_ERR_MSG!(extack, "DRR class is in use"); return -EBUSY; }
    sch_tree_lock(sch); qdisc_purge_queue((*cl).qdisc); qdisc_class_hash_remove(&mut (*q).clhash, &mut (*cl).common); sch_tree_unlock(sch);
    drr_destroy_class(sch, cl); 0
}

unsafe fn drr_search_class(sch: *mut Qdisc, classid: u32) -> c_ulong { drr_find_class(sch, classid) as c_ulong }

unsafe fn drr_tcf_block(sch: *mut Qdisc, cl: c_ulong, extack: *mut netlink_ext_ack) -> *mut tcf_block {
    let q = qdisc_priv::<drr_sched>(sch); if cl != 0 { NL_SET_ERR_MSG!(extack, "DRR classid must be zero"); return core::ptr::null_mut(); } (*q).block
}

unsafe fn drr_bind_tcf(sch: *mut Qdisc, _parent: c_ulong, classid: u32) -> c_ulong { let cl = drr_find_class(sch, classid); if !cl.is_null() { qdisc_class_get(&mut (*cl).common); } cl as c_ulong }
unsafe fn drr_unbind_tcf(_sch: *mut Qdisc, arg: c_ulong) { qdisc_class_put(&mut (*(arg as *mut drr_class)).common); }

unsafe fn drr_graft_class(sch: *mut Qdisc, arg: c_ulong, mut new: *mut Qdisc, old: *mut *mut Qdisc, _extack: *mut netlink_ext_ack) -> c_int {
    let cl = arg as *mut drr_class;
    if new.is_null() { new = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, (*cl).common.classid, core::ptr::null_mut()); if new.is_null() { new = &mut noop_qdisc; } }
    *old = qdisc_replace(sch, new, &mut (*cl).qdisc); 0
}
unsafe fn drr_class_leaf(_sch: *mut Qdisc, arg: c_ulong) -> *mut Qdisc { (*(arg as *mut drr_class)).qdisc }
unsafe fn drr_qlen_notify(_csh: *mut Qdisc, arg: c_ulong) { list_del_init(&mut (*(arg as *mut drr_class)).alist); }

unsafe fn drr_dump_class(_sch: *mut Qdisc, arg: c_ulong, skb: *mut sk_buff, tcm: *mut tcmsg) -> c_int {
    let cl = arg as *mut drr_class; (*tcm).tcm_parent = TC_H_ROOT; (*tcm).tcm_handle = (*cl).common.classid; (*tcm).tcm_info = (*(*cl).qdisc).handle;
    let nest = nla_nest_start_noflag(skb, TCA_OPTIONS); if nest.is_null() { return -EMSGSIZE; }
    if nla_put_u32(skb, TCA_DRR_QUANTUM, READ_ONCE!((*cl).quantum)) != 0 { nla_nest_cancel(skb, nest); return -EMSGSIZE; }
    nla_nest_end(skb, nest)
}

unsafe fn drr_dump_class_stats(_sch: *mut Qdisc, arg: c_ulong, d: *mut gnet_dump) -> c_int {
    let cl = arg as *mut drr_class; let qlen = qdisc_qlen_sum((*cl).qdisc); let mut xstats = tc_drr_stats { deficit: 0 };
    if qlen != 0 { xstats.deficit = READ_ONCE!((*cl).deficit); }
    if gnet_stats_copy_basic(d, core::ptr::null_mut(), &mut (*cl).bstats, true) < 0 || gnet_stats_copy_rate_est(d, &mut (*cl).rate_est) < 0 || gnet_stats_copy_queue(d, (*(*cl).qdisc).cpu_qstats, &mut (*(*cl).qdisc).qstats, qlen) < 0 { return -1; }
    gnet_stats_copy_app(d, &xstats as *const _ as *mut _, core::mem::size_of::<tc_drr_stats>())
}

unsafe fn drr_walk(sch: *mut Qdisc, arg: *mut qdisc_walker) {
    let q = qdisc_priv::<drr_sched>(sch); if (*arg).stop { return; }
    for i in 0..(*q).clhash.hashsize { hlist_for_each_entry!(_cl: *mut drr_class, (*q).clhash.hash.add(i), common.hnode, { if !tc_qdisc_stats_dump(sch, _cl as c_ulong, arg) { return; } }); }
}

unsafe fn drr_classify(skb: *mut sk_buff, sch: *mut Qdisc, qerr: *mut c_int) -> *mut drr_class {
    let q = qdisc_priv::<drr_sched>(sch); if TC_H_MAJ((*skb).priority ^ (*sch).handle) == 0 { let cl = drr_find_class(sch, (*skb).priority); if !cl.is_null() { return cl; } }
    *qerr = NET_XMIT_SUCCESS | __NET_XMIT_BYPASS; let fl = rcu_dereference_bh!((*q).filter_list); let mut res = tcf_result { class: 0, classid: 0 }; let result = tcf_classify_qdisc(skb, fl, &mut res, false);
    if result >= 0 { #[cfg(CONFIG_NET_CLS_ACT)] { match result { TC_ACT_QUEUED | TC_ACT_STOLEN | TC_ACT_TRAP => { *qerr = NET_XMIT_SUCCESS | __NET_XMIT_STOLEN; }, TC_ACT_SHOT => return core::ptr::null_mut(), _ => {} } } let mut cl = res.class as *mut drr_class; if cl.is_null() { cl = drr_find_class(sch, res.classid); } return cl; } core::ptr::null_mut()
}

unsafe fn drr_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> c_int {
    let len = qdisc_pkt_len(skb); let q = qdisc_priv::<drr_sched>(sch); let mut err = 0; let cl = drr_classify(skb, sch, &mut err);
    if cl.is_null() { if err & __NET_XMIT_BYPASS != 0 { qdisc_qstats_drop(sch); } __qdisc_drop(skb, to_free); return err; }
    err = qdisc_enqueue(skb, (*cl).qdisc, to_free); if unlikely!(err != NET_XMIT_SUCCESS) { if net_xmit_drop_count(err) { (*cl).qstats.drops += 1; qdisc_qstats_drop(sch); } return err; }
    if !cl_is_active(cl) { list_add_tail(&mut (*cl).alist, &mut (*q).active); WRITE_ONCE!((*cl).deficit, READ_ONCE!((*cl).quantum)); }
    qstats_backlog_add(sch, len); qdisc_qlen_inc(sch); err
}

unsafe fn drr_dequeue(sch: *mut Qdisc) -> *mut sk_buff {
    let q = qdisc_priv::<drr_sched>(sch); if list_empty(&mut (*q).active) { return core::ptr::null_mut(); }
    loop { let cl = list_first_entry!(&mut (*q).active, drr_class, alist); let mut skb = ((*(*cl).qdisc).ops).peek((*cl).qdisc); if skb.is_null() { qdisc_warn_nonwc!("drr_dequeue", (*cl).qdisc); return core::ptr::null_mut(); }
        let len = qdisc_pkt_len(skb); if len <= (*cl).deficit { WRITE_ONCE!((*cl).deficit, (*cl).deficit - len); skb = qdisc_dequeue_peeked((*cl).qdisc); if skb.is_null() { return core::ptr::null_mut(); } if (*cl).qdisc.q.qlen == 0 { list_del_init(&mut (*cl).alist); } bstats_update(&mut (*cl).bstats, skb); qdisc_bstats_update(sch, skb); qdisc_qstats_backlog_dec(sch, skb); qdisc_qlen_dec(sch); return skb; }
        WRITE_ONCE!((*cl).deficit, (*cl).deficit + READ_ONCE!((*cl).quantum)); list_move_tail(&mut (*cl).alist, &mut (*q).active);
    }
}

unsafe fn drr_init_qdisc(sch: *mut Qdisc, _opt: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int { let q = qdisc_priv::<drr_sched>(sch); let mut err = tcf_block_get(&mut (*q).block, &mut (*q).filter_list, sch, extack); if err != 0 { return err; } err = qdisc_class_hash_init(&mut (*q).clhash); if err < 0 { return err; } INIT_LIST_HEAD(&mut (*q).active); 0 }
unsafe fn drr_reset_qdisc(sch: *mut Qdisc) { let q = qdisc_priv::<drr_sched>(sch); hlist_for_each_class!((*q).clhash, cl, { if (*cl).qdisc.q.qlen != 0 { list_del_init(&mut (*cl).alist); } qdisc_reset((*cl).qdisc); }); }
unsafe fn drr_destroy_qdisc(sch: *mut Qdisc) { let q = qdisc_priv::<drr_sched>(sch); tcf_block_put((*q).block); hlist_for_each_class_safe!((*q).clhash, cl, next, { drr_destroy_class(sch, cl); }); qdisc_class_hash_destroy(&mut (*q).clhash); }

static drr_class_ops: Qdisc_class_ops = Qdisc_class_ops { change: Some(drr_change_class), delete: Some(drr_delete_class), find: Some(drr_search_class), tcf_block: Some(drr_tcf_block), bind_tcf: Some(drr_bind_tcf), unbind_tcf: Some(drr_unbind_tcf), graft: Some(drr_graft_class), leaf: Some(drr_class_leaf), qlen_notify: Some(drr_qlen_notify), dump: Some(drr_dump_class), dump_stats: Some(drr_dump_class_stats), walk: Some(drr_walk) };
static mut drr_qdisc_ops: Qdisc_ops = Qdisc_ops { cl_ops: &drr_class_ops, id: "drr", priv_size: core::mem::size_of::<drr_sched>(), enqueue: Some(drr_enqueue), dequeue: Some(drr_dequeue), peek: Some(qdisc_peek_dequeued), init: Some(drr_init_qdisc), reset: Some(drr_reset_qdisc), destroy: Some(drr_destroy_qdisc), owner: THIS_MODULE };

unsafe fn drr_init() -> c_int { register_qdisc(&mut drr_qdisc_ops) }
unsafe fn drr_exit() { unregister_qdisc(&mut drr_qdisc_ops); }

// module_init(drr_init); module_exit(drr_exit);
// MODULE_ALIAS_NET_SCH("drr"); MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Deficit Round Robin scheduler");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
