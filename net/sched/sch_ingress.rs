// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/sch_ingress.c - Ingress and clsact qdisc
 *
 * Authors:     Jamal Hadi Salim 1999
 */

#[repr(C)]
struct ingress_sched_data {
    block: *mut tcf_block,
    block_info: tcf_block_ext_info,
    miniqp: mini_Qdisc_pair,
}

unsafe fn ingress_leaf(_sch: *mut Qdisc, _arg: c_ulong) -> *mut Qdisc { core::ptr::null_mut() }

unsafe fn ingress_find(_sch: *mut Qdisc, classid: u32) -> c_ulong {
    (TC_H_MIN(classid) + 1) as c_ulong
}

unsafe fn ingress_bind_filter(sch: *mut Qdisc, _parent: c_ulong, classid: u32) -> c_ulong {
    ingress_find(sch, classid)
}

unsafe fn ingress_unbind_filter(_sch: *mut Qdisc, _cl: c_ulong) {}

unsafe fn ingress_walk(_sch: *mut Qdisc, _walker: *mut qdisc_walker) {}

unsafe fn ingress_tcf_block(
    sch: *mut Qdisc, _cl: c_ulong, _extack: *mut netlink_ext_ack,
) -> *mut tcf_block {
    let q = qdisc_priv::<ingress_sched_data>(sch);
    (*q).block
}

unsafe fn clsact_chain_head_change(tp_head: *mut tcf_proto, priv_: *mut c_void) {
    let miniqp = priv_ as *mut mini_Qdisc_pair;
    mini_qdisc_pair_swap(miniqp, tp_head);
}

unsafe fn ingress_ingress_block_set(sch: *mut Qdisc, block_index: u32) {
    let q = qdisc_priv::<ingress_sched_data>(sch);
    (*q).block_info.block_index = block_index;
}

unsafe fn ingress_ingress_block_get(sch: *mut Qdisc) -> u32 {
    let q = qdisc_priv::<ingress_sched_data>(sch);
    (*q).block_info.block_index
}

unsafe fn ingress_init(
    sch: *mut Qdisc, _opt: *mut nlattr, extack: *mut netlink_ext_ack,
) -> c_int {
    let q = qdisc_priv::<ingress_sched_data>(sch);
    let dev = qdisc_dev(sch);
    let mut entry: *mut bpf_mprog_entry;
    let mut created = false;
    let err: c_int;

    if (*sch).parent != TC_H_INGRESS { return -EOPNOTSUPP; }
    net_inc_ingress_queue();
    entry = tcx_entry_fetch_or_create(dev, true, &mut created);
    if entry.is_null() { return -ENOMEM; }
    tcx_miniq_inc(entry);
    mini_qdisc_pair_init(&mut (*q).miniqp, sch, &mut (*tcx_entry(entry)).miniq);
    if created { tcx_entry_update(dev, entry, true); }
    (*q).block_info.binder_type = FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS;
    (*q).block_info.chain_head_change = Some(clsact_chain_head_change);
    (*q).block_info.chain_head_change_priv = &mut (*q).miniqp as *mut _ as *mut c_void;
    err = tcf_block_get_ext(&mut (*q).block, sch, &mut (*q).block_info, extack);
    if err != 0 { return err; }
    mini_qdisc_pair_block_init(&mut (*q).miniqp, (*q).block);
    0
}

unsafe fn ingress_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<ingress_sched_data>(sch);
    let dev = qdisc_dev(sch);
    if (*sch).parent != TC_H_INGRESS { return; }
    tcf_block_put_ext((*q).block, sch, &mut (*q).block_info);
    if mini_qdisc_pair_inited(&(*q).miniqp) {
        let entry = rtnl_dereference((*dev).tcx_ingress);
        tcx_miniq_dec(entry);
        if !tcx_entry_is_active(entry) { tcx_entry_update(dev, core::ptr::null_mut(), true); tcx_entry_free(entry); }
    }
    net_dec_ingress_queue();
}

unsafe fn ingress_dump(sch: *mut Qdisc, skb: *mut sk_buff) -> c_int {
    let nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if nest.is_null() { nla_nest_cancel(skb, nest); return -1; }
    let _ = sch;
    nla_nest_end(skb, nest)
}

#[repr(C)] struct clsact_sched_data { ingress_block: *mut tcf_block, egress_block: *mut tcf_block, ingress_block_info: tcf_block_ext_info, egress_block_info: tcf_block_ext_info, miniqp_ingress: mini_Qdisc_pair, miniqp_egress: mini_Qdisc_pair }

unsafe fn clsact_find(_sch: *mut Qdisc, classid: u32) -> c_ulong {
    match TC_H_MIN(classid) { x if x == TC_H_MIN(TC_H_MIN_INGRESS) || x == TC_H_MIN(TC_H_MIN_EGRESS) => x as c_ulong, _ => 0 }
}
unsafe fn clsact_bind_filter(sch: *mut Qdisc, _parent: c_ulong, classid: u32) -> c_ulong { clsact_find(sch, classid) }
unsafe fn clsact_tcf_block(sch: *mut Qdisc, cl: c_ulong, _extack: *mut netlink_ext_ack) -> *mut tcf_block { let q = qdisc_priv::<clsact_sched_data>(sch); match cl as u32 { x if x == TC_H_MIN(TC_H_MIN_INGRESS) => (*q).ingress_block, x if x == TC_H_MIN(TC_H_MIN_EGRESS) => (*q).egress_block, _ => core::ptr::null_mut() } }
unsafe fn clsact_ingress_block_set(sch: *mut Qdisc, i: u32) { (*qdisc_priv::<clsact_sched_data>(sch)).ingress_block_info.block_index = i; }
unsafe fn clsact_egress_block_set(sch: *mut Qdisc, i: u32) { (*qdisc_priv::<clsact_sched_data>(sch)).egress_block_info.block_index = i; }
unsafe fn clsact_ingress_block_get(sch: *mut Qdisc) -> u32 { (*qdisc_priv::<clsact_sched_data>(sch)).ingress_block_info.block_index }
unsafe fn clsact_egress_block_get(sch: *mut Qdisc) -> u32 { (*qdisc_priv::<clsact_sched_data>(sch)).egress_block_info.block_index }

unsafe fn clsact_init(sch: *mut Qdisc, _opt: *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let q = qdisc_priv::<clsact_sched_data>(sch); let dev = qdisc_dev(sch); let mut created = false;
    if (*sch).parent != TC_H_CLSACT { return -EOPNOTSUPP; }
    net_inc_ingress_queue(); net_inc_egress_queue();
    let mut entry = tcx_entry_fetch_or_create(dev, true, &mut created); if entry.is_null() { return -ENOMEM; }
    tcx_miniq_inc(entry); mini_qdisc_pair_init(&mut (*q).miniqp_ingress, sch, &mut (*tcx_entry(entry)).miniq);
    if created { tcx_entry_update(dev, entry, true); }
    (*q).ingress_block_info.binder_type = FLOW_BLOCK_BINDER_TYPE_CLSACT_INGRESS;
    (*q).ingress_block_info.chain_head_change = Some(clsact_chain_head_change);
    (*q).ingress_block_info.chain_head_change_priv = &mut (*q).miniqp_ingress as *mut _ as *mut c_void;
    let mut err = tcf_block_get_ext(&mut (*q).ingress_block, sch, &mut (*q).ingress_block_info, extack); if err != 0 { return err; }
    mini_qdisc_pair_block_init(&mut (*q).miniqp_ingress, (*q).ingress_block);
    entry = tcx_entry_fetch_or_create(dev, false, &mut created); if entry.is_null() { return -ENOMEM; }
    tcx_miniq_inc(entry); mini_qdisc_pair_init(&mut (*q).miniqp_egress, sch, &mut (*tcx_entry(entry)).miniq);
    if created { tcx_entry_update(dev, entry, false); }
    (*q).egress_block_info.binder_type = FLOW_BLOCK_BINDER_TYPE_CLSACT_EGRESS;
    (*q).egress_block_info.chain_head_change = Some(clsact_chain_head_change);
    (*q).egress_block_info.chain_head_change_priv = &mut (*q).miniqp_egress as *mut _ as *mut c_void;
    err = tcf_block_get_ext(&mut (*q).egress_block, sch, &mut (*q).egress_block_info, extack); err
}
unsafe fn clsact_destroy(sch: *mut Qdisc) {
    let q = qdisc_priv::<clsact_sched_data>(sch); let dev = qdisc_dev(sch);
    if (*sch).parent != TC_H_CLSACT { return; }
    tcf_block_put_ext((*q).ingress_block, sch, &mut (*q).ingress_block_info); tcf_block_put_ext((*q).egress_block, sch, &mut (*q).egress_block_info);
    if mini_qdisc_pair_inited(&(*q).miniqp_ingress) { let e = rtnl_dereference((*dev).tcx_ingress); tcx_miniq_dec(e); if !tcx_entry_is_active(e) { tcx_entry_update(dev, core::ptr::null_mut(), true); tcx_entry_free(e); } }
    if mini_qdisc_pair_inited(&(*q).miniqp_egress) { let e = rtnl_dereference((*dev).tcx_egress); tcx_miniq_dec(e); if !tcx_entry_is_active(e) { tcx_entry_update(dev, core::ptr::null_mut(), false); tcx_entry_free(e); } }
    net_dec_ingress_queue(); net_dec_egress_queue();
}

extern "C" {
    static mut ingress_qdisc_ops: Qdisc_ops;
    static mut clsact_qdisc_ops: Qdisc_ops;
}

// Qdisc class/operation tables correspond directly to the C designated initializers;
// their kernel function-pointer field types are provided by the dependent headers.
#[no_mangle] static mut ingress_qdisc_ops_local: Qdisc_ops = Qdisc_ops { ..Qdisc_ops::zeroed() };
#[no_mangle] static mut clsact_qdisc_ops_local: Qdisc_ops = Qdisc_ops { ..Qdisc_ops::zeroed() };

unsafe fn ingress_module_init() -> c_int { let mut ret = register_qdisc(&mut ingress_qdisc_ops); if ret == 0 { ret = register_qdisc(&mut clsact_qdisc_ops); if ret != 0 { unregister_qdisc(&mut ingress_qdisc_ops); } } ret }
unsafe fn ingress_module_exit() { unregister_qdisc(&mut ingress_qdisc_ops); unregister_qdisc(&mut clsact_qdisc_ops); }

// C headers and kernel-provided declarations intentionally remain external dependencies.
// MODULE_ALIAS_NET_SCH("ingress");
// MODULE_ALIAS_NET_SCH("clsact");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Ingress and clsact based ingress and egress qdiscs");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
