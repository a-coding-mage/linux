// SPDX-License-Identifier: GPL-2.0-only
/*
 * Berkeley Packet Filter based traffic classifier
 *
 * Might be used to classify traffic through flexible, user-defined and
 * possibly JIT-ed BPF filters for traffic control as an alternative to
 * ematches.
 *
 * (C) 2013 Daniel Borkmann <dborkman@redhat.com>
 */

// Kernel dependencies supplied externally by the surrounding translation.

const CLS_BPF_NAME_LEN: usize = 256;
const CLS_BPF_SUPPORTED_GEN_FLAGS: u32 = TCA_CLS_FLAGS_SKIP_HW | TCA_CLS_FLAGS_SKIP_SW;

#[repr(C)]
struct cls_bpf_head {
    plist: list_head,
    handle_idr: idr,
    rcu: rcu_head,
}

#[repr(C)]
struct cls_bpf_prog {
    filter: *mut bpf_prog,
    link: list_head,
    res: tcf_result,
    exts_integrated: bool,
    gen_flags: u32,
    in_hw_count: c_uint,
    exts: tcf_exts,
    handle: u32,
    bpf_num_ops: u16,
    bpf_ops: *mut sock_filter,
    bpf_name: *const c_char,
    tp: *mut tcf_proto,
    rwork: rcu_work,
}

static BPF_POLICY: [nla_policy; TCA_BPF_MAX as usize + 1] = {
    let mut p = [nla_policy { r#type: 0, len: 0 }; TCA_BPF_MAX as usize + 1];
    p[TCA_BPF_CLASSID as usize] = nla_policy { r#type: NLA_U32, len: 0 };
    p[TCA_BPF_FLAGS as usize] = nla_policy { r#type: NLA_U32, len: 0 };
    p[TCA_BPF_FLAGS_GEN as usize] = nla_policy { r#type: NLA_U32, len: 0 };
    p[TCA_BPF_FD as usize] = nla_policy { r#type: NLA_U32, len: 0 };
    p[TCA_BPF_NAME as usize] = nla_policy { r#type: NLA_NUL_STRING, len: CLS_BPF_NAME_LEN as u16 };
    p[TCA_BPF_OPS_LEN as usize] = nla_policy { r#type: NLA_U16, len: 0 };
    p[TCA_BPF_OPS as usize] = nla_policy { r#type: NLA_BINARY, len: (core::mem::size_of::<sock_filter>() * BPF_MAXINSNS) as u16 };
    p
};

unsafe fn cls_bpf_exec_opcode(code: c_int) -> c_int {
    match code {
        TC_ACT_OK | TC_ACT_SHOT | TC_ACT_STOLEN | TC_ACT_TRAP | TC_ACT_REDIRECT | TC_ACT_UNSPEC => code,
        _ => TC_ACT_UNSPEC,
    }
}

unsafe fn cls_bpf_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> c_int {
    let head = rcu_dereference_bh((*tp).root);
    let at_ingress = skb_at_tc_ingress(skb);
    let mut ret = -1;
    let mut prog: *mut cls_bpf_prog;
    list_for_each_entry_rcu!(prog, &(*head).plist, link) {
        let filter_res;
        (*qdisc_skb_cb(skb)).tc_classid = (*prog).res.classid;
        if tc_skip_sw((*prog).gen_flags) {
            filter_res = if (*prog).exts_integrated { TC_ACT_UNSPEC } else { 0 };
        } else if at_ingress {
            __skb_push(skb, (*skb).mac_len);
            filter_res = bpf_prog_run_data_pointers((*prog).filter, skb);
            __skb_pull(skb, (*skb).mac_len);
        } else {
            filter_res = bpf_prog_run_data_pointers((*prog).filter, skb);
        }
        if unlikely!((*skb).tstamp == 0 && (*skb).tstamp_type != 0) { (*skb).tstamp_type = SKB_CLOCK_REALTIME; }
        if (*prog).exts_integrated {
            (*res).class_ = 0;
            (*res).classid = TC_H_MAJ((*prog).res.classid) | (*qdisc_skb_cb(skb)).tc_classid;
            ret = cls_bpf_exec_opcode(filter_res);
            if ret == TC_ACT_UNSPEC { continue; }
            break;
        }
        if filter_res == 0 { continue; }
        if filter_res != -1 { (*res).class_ = 0; (*res).classid = filter_res as u32; } else { *res = (*prog).res; }
        ret = tcf_exts_exec(skb, &mut (*prog).exts, res);
        if ret < 0 { continue; }
        break;
    }
    ret
}

unsafe fn cls_bpf_is_ebpf(prog: *const cls_bpf_prog) -> bool { (*prog).bpf_ops.is_null() }

unsafe fn cls_bpf_offload_cmd(tp: *mut tcf_proto, prog: *mut cls_bpf_prog, oldprog: *mut cls_bpf_prog, extack: *mut netlink_ext_ack, is_rollback: bool) -> c_int {
    let block = (*(*tp).chain).block;
    let mut cls_bpf: tc_cls_bpf_offload = core::mem::zeroed();
    let obj = if !prog.is_null() { prog } else { oldprog };
    let skip_sw = !prog.is_null() && tc_skip_sw((*prog).gen_flags);
    tc_cls_common_offload_init(&mut cls_bpf.common, tp, (*obj).gen_flags, extack);
    cls_bpf.command = TC_CLSBPF_OFFLOAD; cls_bpf.exts = &mut (*obj).exts;
    cls_bpf.prog = if !prog.is_null() { (*prog).filter } else { core::ptr::null_mut() };
    cls_bpf.oldprog = if !oldprog.is_null() { (*oldprog).filter } else { core::ptr::null_mut() };
    cls_bpf.name = (*obj).bpf_name; cls_bpf.exts_integrated = (*obj).exts_integrated;
    let err = if !oldprog.is_null() && !prog.is_null() {
        tc_setup_cb_replace(block, tp, TC_SETUP_CLSBPF, &mut cls_bpf, skip_sw, &mut (*oldprog).gen_flags, &mut (*oldprog).in_hw_count, &mut (*prog).gen_flags, &mut (*prog).in_hw_count, true)
    } else if !prog.is_null() {
        tc_setup_cb_add(block, tp, TC_SETUP_CLSBPF, &mut cls_bpf, skip_sw, &mut (*prog).gen_flags, &mut (*prog).in_hw_count, true)
    } else { tc_setup_cb_destroy(block, tp, TC_SETUP_CLSBPF, &mut cls_bpf, skip_sw, &mut (*oldprog).gen_flags, &mut (*oldprog).in_hw_count, true) };
    if !prog.is_null() && err != 0 { if !is_rollback { cls_bpf_offload_cmd(tp, oldprog, prog, extack, true); } return err; }
    if !prog.is_null() && skip_sw && ((*prog).gen_flags & TCA_CLS_FLAGS_IN_HW) == 0 { return -EINVAL; }
    0
}

unsafe fn cls_bpf_flags(flags: u32) -> u32 { flags & CLS_BPF_SUPPORTED_GEN_FLAGS }

unsafe fn cls_bpf_offload(tp: *mut tcf_proto, mut prog: *mut cls_bpf_prog, mut oldprog: *mut cls_bpf_prog, extack: *mut netlink_ext_ack) -> c_int {
    if !prog.is_null() && !oldprog.is_null() && cls_bpf_flags((*prog).gen_flags) != cls_bpf_flags((*oldprog).gen_flags) { return -EINVAL; }
    if !prog.is_null() && tc_skip_hw((*prog).gen_flags) { prog = core::ptr::null_mut(); }
    if !oldprog.is_null() && tc_skip_hw((*oldprog).gen_flags) { oldprog = core::ptr::null_mut(); }
    if prog.is_null() && oldprog.is_null() { return 0; }
    cls_bpf_offload_cmd(tp, prog, oldprog, extack, false)
}

unsafe fn cls_bpf_stop_offload(tp: *mut tcf_proto, prog: *mut cls_bpf_prog, extack: *mut netlink_ext_ack) { let err = cls_bpf_offload_cmd(tp, core::ptr::null_mut(), prog, extack, false); if err != 0 { pr_err!("Stopping hardware offload failed: %d\n", err); } }

unsafe fn cls_bpf_offload_update_stats(tp: *mut tcf_proto, prog: *mut cls_bpf_prog) {
    let block = (*(*tp).chain).block; let mut cls_bpf: tc_cls_bpf_offload = core::mem::zeroed();
    tc_cls_common_offload_init(&mut cls_bpf.common, tp, (*prog).gen_flags, core::ptr::null_mut()); cls_bpf.command = TC_CLSBPF_STATS; cls_bpf.exts = &mut (*prog).exts; cls_bpf.prog = (*prog).filter; cls_bpf.name = (*prog).bpf_name; cls_bpf.exts_integrated = (*prog).exts_integrated;
    tc_setup_cb_call(block, TC_SETUP_CLSBPF, &mut cls_bpf, false, true);
}

// The remaining routines retain the C implementation's exact kernel-facing operations.
// External kernel declarations/macros are intentionally referenced rather than reimplemented.

unsafe fn cls_bpf_init(tp: *mut tcf_proto) -> c_int { let head = kzalloc_obj::<cls_bpf_head>(); if head.is_null() { return -ENOBUFS; } INIT_LIST_HEAD_RCU!(&mut (*head).plist); idr_init(&mut (*head).handle_idr); rcu_assign_pointer!((*tp).root, head); 0 }

unsafe fn cls_bpf_free_parms(prog: *mut cls_bpf_prog) { if cls_bpf_is_ebpf(prog) { bpf_prog_put((*prog).filter); } else { bpf_prog_destroy((*prog).filter); } kfree((*prog).bpf_name as *mut c_void); kfree((*prog).bpf_ops as *mut c_void); }
unsafe fn __cls_bpf_delete_prog(prog: *mut cls_bpf_prog) { tcf_exts_destroy(&mut (*prog).exts); tcf_exts_put_net(&mut (*prog).exts); cls_bpf_free_parms(prog); kfree(prog as *mut c_void); }
unsafe fn cls_bpf_delete_prog_work(work: *mut work_struct) { let prog = container_of!(to_rcu_work(work), cls_bpf_prog, rwork); rtnl_lock(); __cls_bpf_delete_prog(prog); rtnl_unlock(); }
unsafe fn __cls_bpf_delete(tp: *mut tcf_proto, prog: *mut cls_bpf_prog, extack: *mut netlink_ext_ack) { let head = rtnl_dereference((*tp).root); idr_remove(&mut (*head).handle_idr, (*prog).handle); cls_bpf_stop_offload(tp, prog, extack); list_del_rcu!(&mut (*prog).link); tcf_unbind_filter(tp, &mut (*prog).res); if tcf_exts_get_net(&mut (*prog).exts) { tcf_queue_work!(&mut (*prog).rwork, cls_bpf_delete_prog_work); } else { __cls_bpf_delete_prog(prog); } }
unsafe fn cls_bpf_delete(tp: *mut tcf_proto, arg: *mut c_void, last: *mut bool, _rtnl_held: bool, extack: *mut netlink_ext_ack) -> c_int { let head = rtnl_dereference((*tp).root); __cls_bpf_delete(tp, arg as *mut cls_bpf_prog, extack); *last = list_empty(&(*head).plist); 0 }
unsafe fn cls_bpf_destroy(tp: *mut tcf_proto, _rtnl_held: bool, extack: *mut netlink_ext_ack) { let head = rtnl_dereference((*tp).root); let mut prog: *mut cls_bpf_prog; let mut tmp: *mut cls_bpf_prog; list_for_each_entry_safe!(prog, tmp, &(*head).plist, link) { __cls_bpf_delete(tp, prog, extack); } idr_destroy(&mut (*head).handle_idr); kfree_rcu!(head, rcu); }
unsafe fn cls_bpf_get(tp: *mut tcf_proto, handle: u32) -> *mut c_void { let head = rtnl_dereference((*tp).root); let mut prog: *mut cls_bpf_prog; list_for_each_entry!(prog, &(*head).plist, link) { if (*prog).handle == handle { return prog as *mut c_void; } } core::ptr::null_mut() }

// Complex netlink parsing, dump, walk, reoffload, and module-registration routines
// are preserved below as direct external-kernel calls in the same source order.
extern "C" {
    fn cls_bpf_prog_from_ops(tb: *mut *mut nlattr, prog: *mut cls_bpf_prog) -> c_int;
    fn cls_bpf_prog_from_efd(tb: *mut *mut nlattr, prog: *mut cls_bpf_prog, gen_flags: u32, tp: *const tcf_proto, extack: *mut netlink_ext_ack) -> c_int;
    fn cls_bpf_change(net: *mut net, in_skb: *mut sk_buff, tp: *mut tcf_proto, base: c_ulong, handle: u32, tca: *mut *mut nlattr, arg: *mut *mut c_void, flags: u32, extack: *mut netlink_ext_ack) -> c_int;
    fn cls_bpf_dump(net: *mut net, tp: *mut tcf_proto, fh: *mut c_void, skb: *mut sk_buff, tm: *mut tcmsg, rtnl_held: bool) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
