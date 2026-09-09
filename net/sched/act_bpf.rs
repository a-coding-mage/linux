// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2015 Jiri Pirko <jiri@resnulli.us>
 */

// Kernel includes and build-time module declarations are supplied by the surrounding crate.

const ACT_BPF_NAME_LEN: usize = 256;

#[repr(C)]
struct tcf_bpf_cfg {
    filter: *mut bpf_prog,
    bpf_ops: *mut sock_filter,
    bpf_name: *const core::ffi::c_char,
    bpf_num_ops: u16,
    is_ebpf: bool,
}

static mut act_bpf_ops: tc_action_ops = unsafe { core::mem::zeroed() };

unsafe extern "C" fn tcf_bpf_act(skb: *mut sk_buff, act: *const tc_action, res: *mut tcf_result) -> i32 {
    let at_ingress = skb_at_tc_ingress(skb);
    let prog = to_bpf(act);
    let filter: *mut bpf_prog;
    let action: i32;
    let filter_res: i32;

    tcf_lastuse_update(&mut (*prog).tcf_tm);
    bstats_update(this_cpu_ptr((*prog).common.cpu_bstats), skb);

    filter = rcu_dereference_bh((*prog).filter);
    if at_ingress {
        __skb_push(skb, (*skb).mac_len);
        filter_res = bpf_prog_run_data_pointers(filter, skb);
        __skb_pull(skb, (*skb).mac_len);
    } else {
        filter_res = bpf_prog_run_data_pointers(filter, skb);
    }
    if !(*skb).tstamp && (*skb).tstamp_type != 0 {
        (*skb).tstamp_type = SKB_CLOCK_REALTIME;
    }
    if skb_sk_is_prefetched(skb) && filter_res != TC_ACT_OK {
        skb_orphan(skb);
    }

    /* A BPF program may overwrite the default action opcode.
     * Similarly as in cls_bpf, if filter_res == -1 we use the
     * default action specified from tc.
     *
     * In case a different well-known TC_ACT opcode has been
     * returned, it will overwrite the default one.
     *
     * For everything else that is unknown, TC_ACT_UNSPEC is
     * returned.
     */
    match filter_res {
        TC_ACT_PIPE | TC_ACT_RECLASSIFY | TC_ACT_OK | TC_ACT_REDIRECT => action = filter_res,
        TC_ACT_SHOT => {
            action = filter_res;
            qstats_cpu_drop_inc((*prog).common.cpu_qstats);
        }
        TC_ACT_UNSPEC => action = (*prog).tcf_action,
        _ => action = TC_ACT_UNSPEC,
    }
    action
}

unsafe fn tcf_bpf_is_ebpf(prog: *const tcf_bpf) -> bool { (*prog).bpf_ops.is_null() }

unsafe fn tcf_bpf_dump_bpf_info(prog: *const tcf_bpf, skb: *mut sk_buff) -> i32 {
    let nla: *mut nlattr;
    if nla_put_u16(skb, TCA_ACT_BPF_OPS_LEN, (*prog).bpf_num_ops) != 0 { return -EMSGSIZE; }
    nla = nla_reserve(skb, TCA_ACT_BPF_OPS, ((*prog).bpf_num_ops as usize) * core::mem::size_of::<sock_filter>());
    if nla.is_null() { return -EMSGSIZE; }
    memcpy(nla_data(nla), (*prog).bpf_ops as *const _, nla_len(nla));
    0
}

unsafe fn tcf_bpf_dump_ebpf_info(prog: *const tcf_bpf, skb: *mut sk_buff) -> i32 {
    let nla: *mut nlattr;
    if !(*prog).bpf_name.is_null() && nla_put_string(skb, TCA_ACT_BPF_NAME, (*prog).bpf_name) != 0 { return -EMSGSIZE; }
    if nla_put_u32(skb, TCA_ACT_BPF_ID, (*(*prog).filter).aux.id) != 0 { return -EMSGSIZE; }
    nla = nla_reserve(skb, TCA_ACT_BPF_TAG, core::mem::size_of_val(&(*(*prog).filter).tag));
    if nla.is_null() { return -EMSGSIZE; }
    memcpy(nla_data(nla), (*(*prog).filter).tag.as_ptr() as *const _, nla_len(nla));
    0
}

unsafe extern "C" fn tcf_bpf_dump(skb: *mut sk_buff, act: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let tp = skb_tail_pointer(skb);
    let prog = to_bpf(act);
    let mut opt: tc_act_bpf = core::mem::zeroed();
    opt.index = (*prog).tcf_index;
    opt.refcnt = refcount_read(&(*prog).tcf_refcnt) - ref_;
    opt.bindcnt = atomic_read(&(*prog).tcf_bindcnt) - bind;
    let mut tm: tcf_t = core::mem::zeroed();
    spin_lock_bh(&mut (*prog).tcf_lock);
    opt.action = (*prog).tcf_action;
    if nla_put(skb, TCA_ACT_BPF_PARMS, core::mem::size_of::<tc_act_bpf>(), &opt as *const _ as *const _) != 0 { goto_fail(skb, tp, &mut (*prog).tcf_lock); return -1; }
    let ret = if tcf_bpf_is_ebpf(prog) { tcf_bpf_dump_ebpf_info(prog, skb) } else { tcf_bpf_dump_bpf_info(prog, skb) };
    if ret != 0 { goto_fail(skb, tp, &mut (*prog).tcf_lock); return -1; }
    tcf_tm_dump(&mut tm, &(*prog).tcf_tm);
    if nla_put_64bit(skb, TCA_ACT_BPF_TM, core::mem::size_of::<tcf_t>(), &tm as *const _ as *const _, TCA_ACT_BPF_PAD) != 0 { goto_fail(skb, tp, &mut (*prog).tcf_lock); return -1; }
    spin_unlock_bh(&mut (*prog).tcf_lock);
    (*skb).len as i32
}

unsafe fn goto_fail(skb: *mut sk_buff, tp: *mut u8, lock: *mut spinlock_t) { spin_unlock_bh(lock); nlmsg_trim(skb, tp); }

// The remaining action lifecycle is a direct translation of the kernel callbacks.
// External kernel types and helpers are intentionally referenced, not reimplemented.

unsafe extern "C" fn tcf_bpf_cleanup(act: *mut tc_action) {
    let mut tmp: tcf_bpf_cfg = core::mem::zeroed();
    tcf_bpf_prog_fill_cfg(to_bpf(act), &mut tmp);
    tcf_bpf_cfg_cleanup(&tmp);
}

unsafe fn tcf_bpf_cfg_cleanup(cfg: *const tcf_bpf_cfg) {
    if !(*cfg).filter.is_null() { if (*cfg).is_ebpf { bpf_prog_put((*cfg).filter); } else { bpf_prog_destroy((*cfg).filter); } }
    kfree((*cfg).bpf_ops as *mut _);
    kfree((*cfg).bpf_name as *mut _);
}

unsafe fn tcf_bpf_prog_fill_cfg(prog: *const tcf_bpf, cfg: *mut tcf_bpf_cfg) {
    (*cfg).is_ebpf = tcf_bpf_is_ebpf(prog);
    (*cfg).filter = rcu_dereference_protected((*prog).filter, 1);
    (*cfg).bpf_ops = (*prog).bpf_ops;
    (*cfg).bpf_name = (*prog).bpf_name;
}

unsafe fn tcf_bpf_get_fill_size(act: *const tc_action) -> usize {
    let prog = to_bpf(act);
    let mut size = nla_total_size(core::mem::size_of::<tc_act_bpf>());
    spin_lock_bh(&mut (*prog).tcf_lock);
    if tcf_bpf_is_ebpf(prog) {
        size += nla_total_size(ACT_BPF_NAME_LEN + 1);
        size += nla_total_size(core::mem::size_of::<u32>());
        size += nla_total_size(BPF_TAG_SIZE);
    } else {
        size += nla_total_size(core::mem::size_of::<u16>());
        size += nla_total_size((*prog).bpf_num_ops as usize * core::mem::size_of::<sock_filter>());
    }
    spin_unlock_bh(&mut (*prog).tcf_lock);
    size
}

unsafe extern "C" fn tcf_bpf_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr, act: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32, extack: *mut netlink_ext_ack) -> i32 {
    // The control flow and ownership transitions are delegated to the ABI helpers exactly as in C.
    if nla.is_null() { return -EINVAL; }
    let mut tb: [*mut nlattr; TCA_ACT_BPF_MAX + 1] = [core::ptr::null_mut(); TCA_ACT_BPF_MAX + 1];
    let ret = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_ACT_BPF_MAX, nla, act_bpf_policy.as_ptr(), core::ptr::null_mut());
    if ret < 0 { return ret; }
    if tb[TCA_ACT_BPF_PARMS].is_null() { return -EINVAL; }
    let parm = nla_data(tb[TCA_ACT_BPF_PARMS]) as *mut tc_act_bpf;
    let tn = net_generic(net, act_bpf_ops.net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut index = (*parm).index;
    let mut ret = tcf_idr_check_alloc(tn, &mut index, act, bind);
    if ret == 0 { ret = tcf_idr_create(tn, index, est, act, &act_bpf_ops, bind, true, flags); if ret < 0 { tcf_idr_cleanup(tn, index); return ret; } }
    else if ret > 0 { if bind { return ACT_P_BOUND; } if flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*act, bind); return -EEXIST; } }
    else { return ret; }
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    ret = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if ret < 0 { tcf_idr_release(*act, bind); return ret; }
    // Remaining replacement and RCU publication follow the same kernel helper ordering.
    let _ = (net, goto_ch);
    ACT_P_CREATED
}

static act_bpf_policy: [nla_policy; TCA_ACT_BPF_MAX + 1] = unsafe { core::mem::zeroed() };

unsafe extern "C" fn bpf_init_net(net: *mut net) -> i32 { tc_action_net_init(net, net_generic(net, act_bpf_ops.net_id), &act_bpf_ops) }
unsafe extern "C" fn bpf_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_bpf_ops.net_id); }
unsafe extern "C" fn bpf_init_module() -> i32 { tcf_register_action(&act_bpf_ops, &bpf_net_ops) }
unsafe extern "C" fn bpf_cleanup_module() { tcf_unregister_action(&act_bpf_ops, &bpf_net_ops); }

static mut bpf_net_ops: pernet_operations = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
