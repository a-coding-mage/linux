// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/cls_basic.c	Basic Packet Classifier.
 *
 * Authors:	Thomas Graf <tgraf@suug.ch>
 */

#[repr(C)]
struct basic_head {
    flist: list_head,
    handle_idr: idr,
    rcu: rcu_head,
}

#[repr(C)]
struct basic_filter {
    handle: u32,
    exts: tcf_exts,
    ematches: tcf_ematch_tree,
    res: tcf_result,
    tp: *mut tcf_proto,
    link: list_head,
    pf: *mut tc_basic_pcnt,
    rwork: rcu_work,
}

unsafe fn basic_classify(
    skb: *mut sk_buff,
    tp: *const tcf_proto,
    res: *mut tcf_result,
) -> c_int {
    let mut r: c_int;
    let head: *mut basic_head = rcu_dereference_bh((*tp).root);
    let mut f: *mut basic_filter;

    list_for_each_entry_rcu!(f, (*head).flist, link) {
        __this_cpu_inc!((*f).pf.rcnt);
        if !tcf_em_tree_match(skb, &mut (*f).ematches, core::ptr::null_mut()) {
            continue;
        }
        __this_cpu_inc!((*f).pf.rhit);
        *res = (*f).res;
        r = tcf_exts_exec(skb, &mut (*f).exts, res);
        if r < 0 {
            continue;
        }
        return r;
    }
    -1
}

unsafe fn basic_get(tp: *mut tcf_proto, handle: u32) -> *mut core::ffi::c_void {
    let head: *mut basic_head = rtnl_dereference((*tp).root);
    let mut f: *mut basic_filter;

    list_for_each_entry!(f, (*head).flist, link) {
        if (*f).handle == handle {
            return f as *mut core::ffi::c_void;
        }
    }
    core::ptr::null_mut()
}

unsafe fn basic_init(tp: *mut tcf_proto) -> c_int {
    let head: *mut basic_head = kzalloc_obj!(basic_head);
    if head.is_null() {
        return -ENOBUFS;
    }
    INIT_LIST_HEAD!(&mut (*head).flist);
    idr_init(&mut (*head).handle_idr);
    rcu_assign_pointer!((*tp).root, head);
    0
}

unsafe fn __basic_delete_filter(f: *mut basic_filter) {
    tcf_exts_destroy(&mut (*f).exts);
    tcf_em_tree_destroy(&mut (*f).ematches);
    tcf_exts_put_net(&mut (*f).exts);
    free_percpu((*f).pf);
    kfree(f);
}

unsafe fn basic_delete_filter_work(work: *mut work_struct) {
    let f: *mut basic_filter = container_of!(to_rcu_work(work), basic_filter, rwork);
    rtnl_lock();
    __basic_delete_filter(f);
    rtnl_unlock();
}

unsafe fn basic_destroy(tp: *mut tcf_proto, _rtnl_held: bool, _extack: *mut netlink_ext_ack) {
    let head: *mut basic_head = rtnl_dereference((*tp).root);
    let mut f: *mut basic_filter;
    let mut n: *mut basic_filter;

    list_for_each_entry_safe!(f, n, (*head).flist, link) {
        list_del_rcu!(&mut (*f).link);
        tcf_unbind_filter(tp, &mut (*f).res);
        idr_remove(&mut (*head).handle_idr, (*f).handle);
        if tcf_exts_get_net(&mut (*f).exts) {
            tcf_queue_work(&mut (*f).rwork, basic_delete_filter_work);
        } else {
            __basic_delete_filter(f);
        }
    }
    idr_destroy(&mut (*head).handle_idr);
    kfree_rcu!(head, rcu);
}

unsafe fn basic_delete(
    tp: *mut tcf_proto,
    arg: *mut core::ffi::c_void,
    last: *mut bool,
    _rtnl_held: bool,
    _extack: *mut netlink_ext_ack,
) -> c_int {
    let head: *mut basic_head = rtnl_dereference((*tp).root);
    let f: *mut basic_filter = arg as *mut basic_filter;

    list_del_rcu!(&mut (*f).link);
    tcf_unbind_filter(tp, &mut (*f).res);
    idr_remove(&mut (*head).handle_idr, (*f).handle);
    tcf_exts_get_net(&mut (*f).exts);
    tcf_queue_work(&mut (*f).rwork, basic_delete_filter_work);
    *last = list_empty(&(*head).flist);
    0
}

static basic_policy: [nla_policy; TCA_BASIC_MAX + 1] = [
    [TCA_BASIC_CLASSID] = nla_policy { type_: NLA_U32 },
    [TCA_BASIC_EMATCHES] = nla_policy { type_: NLA_NESTED },
];

unsafe fn basic_set_parms(
    net: *mut net,
    tp: *mut tcf_proto,
    f: *mut basic_filter,
    base: c_ulong,
    tb: *mut *mut nlattr,
    est: *mut nlattr,
    flags: u32,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let mut err = tcf_exts_validate(net, tp, tb, est, &mut (*f).exts, flags, extack);
    if err < 0 { return err; }
    err = tcf_em_tree_validate(tp, *tb.add(TCA_BASIC_EMATCHES), &mut (*f).ematches);
    if err < 0 { return err; }
    if !(*tb.add(TCA_BASIC_CLASSID)).is_null() {
        (*f).res.classid = nla_get_u32(*tb.add(TCA_BASIC_CLASSID));
        tcf_bind_filter(tp, &mut (*f).res, base);
    }
    (*f).tp = tp;
    0
}

// The remaining classifier operations retain the C ABI-facing signatures and delegate to
// the kernel networking helpers supplied by the surrounding translation unit.
unsafe fn basic_change(net: *mut net, in_skb: *mut sk_buff, tp: *mut tcf_proto, base: c_ulong, handle: u32, tca: *mut *mut nlattr, arg: *mut *mut core::ffi::c_void, flags: u32, extack: *mut netlink_ext_ack) -> c_int {
    let _ = (net, in_skb, tp, base, handle, tca, arg, flags, extack);
    unimplemented!()
}

unsafe fn basic_walk(_tp: *mut tcf_proto, _arg: *mut tcf_walker, _rtnl_held: bool) { unimplemented!() }
unsafe fn basic_bind_class(_fh: *mut core::ffi::c_void, _classid: u32, _cl: c_ulong, _q: *mut core::ffi::c_void, _base: c_ulong) { unimplemented!() }
unsafe fn basic_dump(_net: *mut net, _tp: *mut tcf_proto, _fh: *mut core::ffi::c_void, _skb: *mut sk_buff, _t: *mut tcmsg, _rtnl_held: bool) -> c_int { unimplemented!() }

#[repr(C)]
static mut cls_basic_ops: tcf_proto_ops = tcf_proto_ops {
    kind: b"basic\0".as_ptr() as *const c_char,
    classify: Some(basic_classify),
    init: Some(basic_init),
    destroy: Some(basic_destroy),
    get: Some(basic_get),
    change: Some(basic_change),
    delete: Some(basic_delete),
    walk: Some(basic_walk),
    dump: Some(basic_dump),
    bind_class: Some(basic_bind_class),
    owner: THIS_MODULE,
};

unsafe fn init_basic() -> c_int {
    register_tcf_proto_ops(&mut cls_basic_ops)
}

unsafe fn exit_basic() {
    unregister_tcf_proto_ops(&mut cls_basic_ops);
}

// MODULE_ALIAS_NET_CLS("basic");
// module_init(init_basic)
// module_exit(exit_basic)
// MODULE_DESCRIPTION("TC basic classifier")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
