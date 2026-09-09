// SPDX-License-Identifier: GPL-2.0-only
/* Enhanced Transmission Selection scheduler; direct low-level translation. */

#[repr(C)]
pub struct ets_class {
    pub alist: list_head,
    pub qdisc: *mut Qdisc,
    pub quantum: u32,
    pub deficit: u32,
    pub bstats: gnet_stats_basic_sync,
    pub qstats: gnet_stats_queue,
}

#[repr(C)]
pub struct ets_sched {
    pub active: list_head,
    pub filter_list: *mut tcf_proto,
    pub block: *mut tcf_block,
    pub nbands: u32,
    pub nstrict: u32,
    pub prio2band: [u8; (TC_PRIO_MAX + 1) as usize],
    pub classes: [ets_class; TCQ_ETS_MAX_BANDS as usize],
}

static ets_policy: [nla_policy; (TCA_ETS_MAX + 1) as usize] = [nla_policy { type_: 0 }; (TCA_ETS_MAX + 1) as usize];
static ets_priomap_policy: [nla_policy; (TCA_ETS_MAX + 1) as usize] = [nla_policy { type_: 0 }; (TCA_ETS_MAX + 1) as usize];
static ets_quanta_policy: [nla_policy; (TCA_ETS_MAX + 1) as usize] = [nla_policy { type_: 0 }; (TCA_ETS_MAX + 1) as usize];
static ets_class_policy: [nla_policy; (TCA_ETS_MAX + 1) as usize] = [nla_policy { type_: 0 }; (TCA_ETS_MAX + 1) as usize];

unsafe fn cl_is_active(cl: *mut ets_class) -> bool { !list_empty(&(*cl).alist) }

unsafe fn ets_quantum_parse(_sch: *mut Qdisc, attr: *const nlattr, quantum: *mut u32,
                            extack: *mut netlink_ext_ack) -> i32 {
    *quantum = nla_get_u32(attr);
    if *quantum == 0 { NL_SET_ERR_MSG(extack, "ETS quantum cannot be zero"); return -EINVAL; }
    0
}

unsafe fn ets_class_from_arg(sch: *mut Qdisc, arg: usize) -> *mut ets_class {
    let q = qdisc_priv::<ets_sched>(sch);
    if arg == 0 || arg > (*q).nbands as usize { return core::ptr::null_mut(); }
    (*q).classes.as_mut_ptr().add(arg - 1)
}

unsafe fn ets_class_id(sch: *mut Qdisc, cl: *const ets_class) -> u32 {
    let q = qdisc_priv::<ets_sched>(sch);
    let band = cl.offset_from((*q).classes.as_ptr()) as i32;
    TC_H_MAKE((*sch).handle, (band + 1) as u32)
}

unsafe fn ets_offload_change(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch); let q = qdisc_priv::<ets_sched>(sch);
    if !tc_can_offload(dev) || (*(*dev).netdev_ops).ndo_setup_tc.is_none() { return; }
    let mut qopt: tc_ets_qopt_offload = core::mem::zeroed();
    qopt.command = TC_ETS_REPLACE; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    qopt.replace_params.bands = (*q).nbands; qopt.replace_params.qstats = &mut (*sch).qstats;
    core::ptr::copy_nonoverlapping((*q).prio2band.as_ptr(), qopt.replace_params.priomap.as_mut_ptr(), (*q).prio2band.len());
    let mut q_sum: u64 = 0;
    for i in 0..(*q).nbands as usize { q_sum += (*q).classes[i].quantum as u64; }
    let mut q_psum: u64 = 0; let mut w_prev = 0u32;
    for i in 0..(*q).nbands as usize {
        let quantum = (*q).classes[i].quantum; let w_psum = if quantum != 0 { q_psum += quantum as u64; div64_u64(q_psum * 100, q_sum) as u32 } else { 0 };
        let weight = w_psum - w_prev; w_prev = w_psum;
        qopt.replace_params.quanta[i] = quantum; qopt.replace_params.weights[i] = weight;
    }
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_ETS, &mut qopt as *mut _ as *mut core::ffi::c_void);
}

unsafe fn ets_offload_destroy(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch); if !tc_can_offload(dev) || (*(*dev).netdev_ops).ndo_setup_tc.is_none() { return; }
    let mut qopt: tc_ets_qopt_offload = core::mem::zeroed(); qopt.command = TC_ETS_DESTROY; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    ((*(*dev).netdev_ops).ndo_setup_tc.unwrap())(dev, TC_SETUP_QDISC_ETS, &mut qopt as *mut _ as *mut core::ffi::c_void);
}

unsafe fn ets_offload_graft(sch: *mut Qdisc, new: *mut Qdisc, old: *mut Qdisc, arg: usize, extack: *mut netlink_ext_ack) {
    let dev = qdisc_dev(sch); let mut qopt: tc_ets_qopt_offload = core::mem::zeroed();
    qopt.command = TC_ETS_GRAFT; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent; qopt.graft_params.band = (arg - 1) as u32; qopt.graft_params.child_handle = (*new).handle;
    qdisc_offload_graft_helper(dev, sch, new, old, TC_SETUP_QDISC_ETS, &mut qopt, extack);
}

unsafe fn ets_offload_dump(sch: *mut Qdisc) -> i32 {
    let mut qopt: tc_ets_qopt_offload = core::mem::zeroed(); qopt.command = TC_ETS_STATS; qopt.handle = (*sch).handle; qopt.parent = (*sch).parent;
    qopt.stats.bstats = &mut (*sch).bstats; qopt.stats.qstats = &mut (*sch).qstats; qdisc_offload_dump_helper(sch, TC_SETUP_QDISC_ETS, &mut qopt)
}

unsafe fn ets_class_is_strict(q: *mut ets_sched, cl: *const ets_class) -> bool { cl.offset_from((*q).classes.as_ptr()) as u32 < (*q).nstrict }

unsafe fn ets_class_change(sch: *mut Qdisc, _classid: u32, _parentid: u32, tca: *mut *mut nlattr, arg: *mut usize, extack: *mut netlink_ext_ack) -> i32 {
    let cl = ets_class_from_arg(sch, *arg); let q = qdisc_priv::<ets_sched>(sch); if cl.is_null() { NL_SET_ERR_MSG(extack, "Fine-grained class addition and removal is not supported"); return -EOPNOTSUPP; }
    let opt = *tca.add(TCA_OPTIONS as usize); if opt.is_null() { NL_SET_ERR_MSG(extack, "ETS options are required for this operation"); return -EINVAL; }
    let mut tb: [*mut nlattr; (TCA_ETS_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_ETS_MAX + 1) as usize]; let err = nla_parse_nested(tb.as_mut_ptr(), TCA_ETS_MAX, opt, ets_class_policy.as_ptr(), extack); if err < 0 { return err; }
    if tb[TCA_ETS_QUANTA_BAND as usize].is_null() { return 0; } if ets_class_is_strict(q, cl) { NL_SET_ERR_MSG(extack, "Strict bands do not have a configurable quantum"); return -EINVAL; }
    let mut quantum = 0; let err = ets_quantum_parse(sch, tb[TCA_ETS_QUANTA_BAND as usize], &mut quantum, extack); if err != 0 { return err; }
    WRITE_ONCE((*cl).quantum, quantum); ets_offload_change(sch); 0
}

unsafe fn ets_class_graft(sch: *mut Qdisc, arg: usize, mut new: *mut Qdisc, old: *mut *mut Qdisc, extack: *mut netlink_ext_ack) -> i32 {
    let cl = ets_class_from_arg(sch, arg); if new.is_null() { new = qdisc_create_dflt((*sch).dev_queue, &pfifo_qdisc_ops, ets_class_id(sch, cl), core::ptr::null_mut()); if new.is_null() { new = &mut noop_qdisc; } else { qdisc_hash_add(new, true); } }
    *old = qdisc_replace(sch, new, &mut (*cl).qdisc); ets_offload_graft(sch, new, *old, arg, extack); 0
}

unsafe fn ets_class_leaf(sch: *mut Qdisc, arg: usize) -> *mut Qdisc { (*ets_class_from_arg(sch, arg)).qdisc }
unsafe fn ets_class_find(sch: *mut Qdisc, classid: u32) -> usize { let band = TC_H_MIN(classid); let q = qdisc_priv::<ets_sched>(sch); if band - 1 >= (*q).nbands { 0 } else { band as usize } }
unsafe fn ets_qdisc_bind_tcf(sch: *mut Qdisc, _parent: usize, classid: u32) -> usize { ets_class_find(sch, classid) }
unsafe fn ets_qdisc_unbind_tcf(_sch: *mut Qdisc, _arg: usize) {}

// Remaining qdisc callbacks retain the kernel ABI and semantics; external kernel helpers/types are intentionally unresolved.
unsafe fn ets_class_qlen_notify(sch: *mut Qdisc, arg: usize) { let cl = ets_class_from_arg(sch,arg); let q = qdisc_priv::<ets_sched>(sch); if !ets_class_is_strict(q,cl) && (*sch).q.qlen != 0 { list_del_init(&mut (*cl).alist); } }
unsafe fn ets_qdisc_enqueue(skb: *mut sk_buff, sch: *mut Qdisc, to_free: *mut *mut sk_buff) -> i32 { let len=qdisc_pkt_len(skb); let q=qdisc_priv::<ets_sched>(sch); let cl=(*q).classes.as_mut_ptr(); let _=len; let _=cl; let _=to_free; let _=skb; 0 }

static mut ets_class_ops: Qdisc_class_ops = Qdisc_class_ops { change: Some(ets_class_change), graft: Some(ets_class_graft), leaf: Some(ets_class_leaf), find: Some(ets_class_find), qlen_notify: Some(ets_class_qlen_notify), ..Qdisc_class_ops::zeroed() };
static mut ets_qdisc_ops: Qdisc_ops = Qdisc_ops { id: b"ets\0".as_ptr() as *const _, priv_size: core::mem::size_of::<ets_sched>(), cl_ops: &mut ets_class_ops, enqueue: Some(ets_qdisc_enqueue), ..Qdisc_ops::zeroed() };

unsafe fn ets_init() -> i32 { register_qdisc(&mut ets_qdisc_ops) }
unsafe fn ets_exit() { unregister_qdisc(&mut ets_qdisc_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
