// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/cls_cgroup.c	Control Group Classifier
 *
 * Authors:	Thomas Graf <tgraf@suug.ch>
 */

#[repr(C)]
pub struct cls_cgroup_head {
    pub handle: u32,
    pub exts: tcf_exts,
    pub ematches: tcf_ematch_tree,
    pub tp: *mut tcf_proto,
    pub rwork: rcu_work,
}

// TC_INDIRECT_SCOPE
pub unsafe fn cls_cgroup_classify(
    skb: *mut sk_buff,
    tp: *const tcf_proto,
    res: *mut tcf_result,
) -> i32 {
    let head = rcu_dereference_bh((*tp).root) as *mut cls_cgroup_head;
    let classid: u32 = task_get_classid(skb);

    if unlikely(head.is_null()) {
        return -1;
    }
    if classid == 0 {
        return -1;
    }
    if !tcf_em_tree_match(skb, &mut (*head).ematches, core::ptr::null_mut()) {
        return -1;
    }

    (*res).classid = classid;
    (*res).class = 0;

    tcf_exts_exec(skb, &mut (*head).exts, res)
}

unsafe fn cls_cgroup_get(_tp: *mut tcf_proto, _handle: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

unsafe fn cls_cgroup_init(_tp: *mut tcf_proto) -> i32 {
    0
}

static CGROUP_POLICY: [nla_policy; TCA_CGROUP_MAX as usize + 1] = {
    let mut policy = [nla_policy { type_: 0 }; TCA_CGROUP_MAX as usize + 1];
    policy[TCA_CGROUP_EMATCHES as usize] = nla_policy { type_: NLA_NESTED };
    policy
};

unsafe fn __cls_cgroup_destroy(head: *mut cls_cgroup_head) {
    tcf_exts_destroy(&mut (*head).exts);
    tcf_em_tree_destroy(&mut (*head).ematches);
    tcf_exts_put_net(&mut (*head).exts);
    kfree(head as *mut core::ffi::c_void);
}

unsafe fn cls_cgroup_destroy_work(work: *mut work_struct) {
    let head = container_of(
        to_rcu_work(work),
        core::mem::offset_of!(cls_cgroup_head, rwork),
    );
    rtnl_lock();
    __cls_cgroup_destroy(head);
    rtnl_unlock();
}

unsafe fn cls_cgroup_change(
    net: *mut net,
    in_skb: *mut sk_buff,
    tp: *mut tcf_proto,
    base: core::ffi::c_ulong,
    handle: u32,
    tca: *mut *mut nlattr,
    arg: *mut *mut core::ffi::c_void,
    flags: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut tb: [*mut nlattr; TCA_CGROUP_MAX as usize + 1] =
        [core::ptr::null_mut(); TCA_CGROUP_MAX as usize + 1];
    let head = rtnl_dereference((*tp).root) as *mut cls_cgroup_head;
    let mut new: *mut cls_cgroup_head;
    let mut err: i32;

    if (*tca.add(TCA_OPTIONS as usize)).is_null() {
        return -EINVAL;
    }

    if head.is_null() && handle == 0 {
        return -EINVAL;
    }

    if !head.is_null() && handle != (*head).handle {
        return -ENOENT;
    }

    new = kzalloc_obj::<cls_cgroup_head>(GFP_KERNEL_ACCOUNT);
    if new.is_null() {
        return -ENOBUFS;
    }

    err = tcf_exts_init(&mut (*new).exts, net, TCA_CGROUP_ACT, TCA_CGROUP_POLICE);
    if err < 0 {
        goto_errout(new, err);
    }
    (*new).handle = handle;
    (*new).tp = tp;
    err = nla_parse_nested_deprecated(
        tb.as_mut_ptr(), TCA_CGROUP_MAX, *tca.add(TCA_OPTIONS as usize),
        CGROUP_POLICY.as_ptr(), core::ptr::null_mut(),
    );
    if err < 0 {
        goto_errout(new, err);
    }

    err = tcf_exts_validate(net, tp, tb.as_mut_ptr(), *tca.add(TCA_RATE as usize),
                            &mut (*new).exts, flags, extack);
    if err < 0 {
        goto_errout(new, err);
    }

    err = tcf_em_tree_validate(tp, tb[TCA_CGROUP_EMATCHES as usize], &mut (*new).ematches);
    if err < 0 {
        goto_errout(new, err);
    }

    rcu_assign_pointer((*tp).root, new);
    if !head.is_null() {
        tcf_exts_get_net(&mut (*head).exts);
        tcf_queue_work(&mut (*head).rwork, cls_cgroup_destroy_work);
    }
    return 0;
}

unsafe fn goto_errout(new: *mut cls_cgroup_head, err: i32) -> ! {
    tcf_exts_destroy(&mut (*new).exts);
    kfree(new as *mut core::ffi::c_void);
    panic!("C goto errout: {}", err);
}

unsafe fn cls_cgroup_destroy(
    tp: *mut tcf_proto,
    _rtnl_held: bool,
    _extack: *mut netlink_ext_ack,
) {
    let head = rtnl_dereference((*tp).root) as *mut cls_cgroup_head;
    // Head can still be NULL due to cls_cgroup_init().
    if !head.is_null() {
        if tcf_exts_get_net(&mut (*head).exts) {
            tcf_queue_work(&mut (*head).rwork, cls_cgroup_destroy_work);
        } else {
            __cls_cgroup_destroy(head);
        }
    }
}

unsafe fn cls_cgroup_delete(
    _tp: *mut tcf_proto, _arg: *mut core::ffi::c_void, _last: *mut bool,
    _rtnl_held: bool, _extack: *mut netlink_ext_ack,
) -> i32 {
    -EOPNOTSUPP
}

unsafe fn cls_cgroup_walk(tp: *mut tcf_proto, arg: *mut tcf_walker, _rtnl_held: bool) {
    let head = rtnl_dereference((*tp).root) as *mut cls_cgroup_head;
    if (*arg).count < (*arg).skip {
        (*arg).count += 1;
        return;
    }
    if head.is_null() {
        return;
    }
    if ((*arg).fn_)(tp, head as *mut core::ffi::c_void, arg) < 0 {
        (*arg).stop = 1;
        return;
    }
    (*arg).count += 1;
}

unsafe fn cls_cgroup_dump(
    _net: *mut net, tp: *mut tcf_proto, _fh: *mut core::ffi::c_void,
    skb: *mut sk_buff, t: *mut tcmsg, _rtnl_held: bool,
) -> i32 {
    let head = rtnl_dereference((*tp).root) as *mut cls_cgroup_head;
    let nest: *mut nlattr;
    (*t).tcm_handle = (*head).handle;
    nest = nla_nest_start_noflag(skb, TCA_OPTIONS);
    if nest.is_null() {
        return -1;
    }
    if tcf_exts_dump(skb, &mut (*head).exts) < 0
        || tcf_em_tree_dump(skb, &mut (*head).ematches, TCA_CGROUP_EMATCHES) < 0
    {
        nla_nest_cancel(skb, nest);
        return -1;
    }
    nla_nest_end(skb, nest);
    if tcf_exts_dump_stats(skb, &mut (*head).exts) < 0 {
        nla_nest_cancel(skb, nest);
        return -1;
    }
    (*skb).len as i32
}

// Equivalent to the C module registration and metadata declarations.
static mut CLS_CGROUP_OPS: tcf_proto_ops = tcf_proto_ops {
    kind: "cgroup",
    init: Some(cls_cgroup_init),
    change: Some(cls_cgroup_change),
    classify: Some(cls_cgroup_classify),
    destroy: Some(cls_cgroup_destroy),
    get: Some(cls_cgroup_get),
    delete: Some(cls_cgroup_delete),
    walk: Some(cls_cgroup_walk),
    dump: Some(cls_cgroup_dump),
    owner: THIS_MODULE,
};

unsafe fn init_cgroup_cls() -> i32 {
    register_tcf_proto_ops(&mut CLS_CGROUP_OPS)
}

unsafe fn exit_cgroup_cls() {
    unregister_tcf_proto_ops(&mut CLS_CGROUP_OPS);
}

// MODULE_ALIAS_NET_CLS("cgroup");
// module_init(init_cgroup_cls);
// module_exit(exit_cgroup_cls);
// MODULE_DESCRIPTION("TC cgroup classifier");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
