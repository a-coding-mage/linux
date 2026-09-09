// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/cls_matchll.c        Match-all classifier
 *
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

// C dependencies: linux/kernel.h, linux/init.h, linux/module.h,
// linux/percpu.h, net/sch_generic.h, net/pkt_cls.h, net/tc_wrapper.h

#[repr(C)]
pub struct cls_mall_head {
    pub exts: tcf_exts,
    pub res: tcf_result,
    pub handle: u32,
    pub flags: u32,
    pub in_hw_count: c_uint,
    pub pf: *mut tc_matchall_pcnt,
    pub rwork: rcu_work,
    pub deleting: bool,
}

pub unsafe fn mall_classify(
    skb: *mut sk_buff,
    tp: *const tcf_proto,
    res: *mut tcf_result,
) -> c_int {
    let head: *mut cls_mall_head = rcu_dereference_bh((*tp).root);

    if head.is_null() {
        return -1;
    }

    if tc_skip_sw((*head).flags) {
        return -1;
    }

    *res = (*head).res;
    __this_cpu_inc((*head).pf.as_mut().unwrap().rhit);
    tcf_exts_exec(skb, &mut (*head).exts, res)
}

unsafe fn mall_init(_tp: *mut tcf_proto) -> c_int {
    0
}

unsafe fn __mall_destroy(head: *mut cls_mall_head) {
    tcf_exts_destroy(&mut (*head).exts);
    tcf_exts_put_net(&mut (*head).exts);
    free_percpu(head.cast());
    kfree(head.cast());
}

unsafe fn mall_destroy_work(work: *mut work_struct) {
    let head: *mut cls_mall_head = container_of(to_rcu_work(work), |x: *mut rcu_work| {
        x.cast::<cls_mall_head>()
    });
    rtnl_lock();
    __mall_destroy(head);
    rtnl_unlock();
}

unsafe fn mall_destroy_hw_filter(
    tp: *mut tcf_proto,
    head: *mut cls_mall_head,
    cookie: c_ulong,
    extack: *mut netlink_ext_ack,
) {
    let mut cls_mall: tc_cls_matchall_offload = core::mem::zeroed();
    let block: *mut tcf_block = (*(*tp).chain).block;

    tc_cls_common_offload_init(&mut cls_mall.common, tp, (*head).flags, extack);
    cls_mall.command = TC_CLSMATCHALL_DESTROY;
    cls_mall.cookie = cookie;

    tc_setup_cb_destroy(
        block, tp, TC_SETUP_CLSMATCHALL, &mut cls_mall, false,
        &mut (*head).flags, &mut (*head).in_hw_count, true,
    );
}

unsafe fn mall_replace_hw_filter(
    tp: *mut tcf_proto,
    head: *mut cls_mall_head,
    cookie: c_ulong,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let mut cls_mall: tc_cls_matchall_offload = core::mem::zeroed();
    let block: *mut tcf_block = (*(*tp).chain).block;
    let skip_sw = tc_skip_sw((*head).flags);
    let mut err: c_int;

    cls_mall.rule = flow_rule_alloc(tcf_exts_num_actions(&(*head).exts));
    if cls_mall.rule.is_null() {
        return -ENOMEM;
    }

    tc_cls_common_offload_init(&mut cls_mall.common, tp, (*head).flags, extack);
    cls_mall.command = TC_CLSMATCHALL_REPLACE;
    cls_mall.cookie = cookie;

    err = tc_setup_offload_action(&mut (*cls_mall.rule).action, &(*head).exts,
                                   cls_mall.common.extack);
    if err != 0 {
        kfree(cls_mall.rule.cast());
        mall_destroy_hw_filter(tp, head, cookie, core::ptr::null_mut());
        return if skip_sw { err } else { 0 };
    }

    err = tc_setup_cb_add(block, tp, TC_SETUP_CLSMATCHALL, &mut cls_mall,
                          skip_sw, &mut (*head).flags, &mut (*head).in_hw_count, true);
    tc_cleanup_offload_action(&mut (*cls_mall.rule).action);
    kfree(cls_mall.rule.cast());

    if err != 0 {
        mall_destroy_hw_filter(tp, head, cookie, core::ptr::null_mut());
        return err;
    }

    if skip_sw && ((*head).flags & TCA_CLS_FLAGS_IN_HW) == 0 {
        return -EINVAL;
    }
    0
}

unsafe fn mall_destroy(tp: *mut tcf_proto, _rtnl_held: bool, extack: *mut netlink_ext_ack) {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    if head.is_null() { return; }
    tcf_unbind_filter(tp, &mut (*head).res);
    if !tc_skip_hw((*head).flags) {
        mall_destroy_hw_filter(tp, head, head as c_ulong, extack);
    }
    if tcf_exts_get_net(&mut (*head).exts) != 0 {
        tcf_queue_work(&mut (*head).rwork, mall_destroy_work);
    } else {
        __mall_destroy(head);
    }
}

unsafe fn mall_get(tp: *mut tcf_proto, handle: u32) -> *mut core::ffi::c_void {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    if !head.is_null() && (*head).handle == handle { head.cast() } else { core::ptr::null_mut() }
}

static mall_policy: [nla_policy; TCA_MATCHALL_MAX + 1] = [
    nla_policy { type_: NLA_UNSPEC },
    nla_policy { type_: NLA_U32 },
    nla_policy { type_: NLA_U32 },
];

unsafe fn mall_change(
    net: *mut net, in_skb: *mut sk_buff, tp: *mut tcf_proto, base: c_ulong,
    mut handle: u32, tca: *mut *mut nlattr, arg: *mut *mut core::ffi::c_void,
    flags: u32, extack: *mut netlink_ext_ack,
) -> c_int {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    let mut tb: [*mut nlattr; TCA_MATCHALL_MAX + 1] = [core::ptr::null_mut(); TCA_MATCHALL_MAX + 1];
    let mut bound_to_filter = false;
    let new: *mut cls_mall_head;
    let mut userflags = 0u32;
    let mut err: c_int;

    if (*tca.add(TCA_OPTIONS)).is_null() { return -EINVAL; }
    if !head.is_null() { return -EEXIST; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_MATCHALL_MAX,
        *tca.add(TCA_OPTIONS), mall_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if !tb[TCA_MATCHALL_FLAGS].is_null() {
        userflags = nla_get_u32(tb[TCA_MATCHALL_FLAGS]);
        if !tc_flags_valid(userflags) { return -EINVAL; }
    }
    new = kzalloc_obj::<cls_mall_head>(GFP_KERNEL_ACCOUNT);
    if new.is_null() { return -ENOBUFS; }
    err = tcf_exts_init(&mut (*new).exts, net, TCA_MATCHALL_ACT, 0);
    if err != 0 { kfree(new.cast()); return err; }
    if handle == 0 { handle = 1; }
    (*new).handle = handle;
    (*new).flags = userflags;
    (*new).pf = alloc_percpu_gfp::<tc_matchall_pcnt>(GFP_KERNEL_ACCOUNT);
    if (*new).pf.is_null() { err = -ENOMEM; tcf_exts_destroy(&mut (*new).exts); kfree(new.cast()); return err; }
    err = tcf_exts_validate_ex(net, tp, tb.as_mut_ptr(), *tca.add(TCA_RATE),
        &mut (*new).exts, flags, (*new).flags, extack);
    if err < 0 { free_percpu((*new).pf.cast()); tcf_exts_destroy(&mut (*new).exts); kfree(new.cast()); return err; }
    if !tb[TCA_MATCHALL_CLASSID].is_null() {
        (*new).res.classid = nla_get_u32(tb[TCA_MATCHALL_CLASSID]);
        tcf_bind_filter(tp, &mut (*new).res, base);
        bound_to_filter = true;
    }
    if !tc_skip_hw((*new).flags) {
        err = mall_replace_hw_filter(tp, new, new as c_ulong, extack);
        if err != 0 { if bound_to_filter { tcf_unbind_filter(tp, &mut (*new).res); } free_percpu((*new).pf.cast()); tcf_exts_destroy(&mut (*new).exts); kfree(new.cast()); return err; }
    }
    if !tc_in_hw((*new).flags) { (*new).flags |= TCA_CLS_FLAGS_NOT_IN_HW; }
    tcf_proto_update_usesw(tp, (*new).flags);
    *arg = head.cast();
    rcu_assign_pointer((*tp).root, new);
    0
}

unsafe fn mall_delete(tp: *mut tcf_proto, _arg: *mut core::ffi::c_void, last: *mut bool, _rtnl_held: bool, _extack: *mut netlink_ext_ack) -> c_int {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    (*head).deleting = true;
    *last = true;
    0
}

unsafe fn mall_walk(tp: *mut tcf_proto, arg: *mut tcf_walker, _rtnl_held: bool) {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    if (*arg).count < (*arg).skip { (*arg).count += 1; return; }
    if head.is_null() || (*head).deleting { return; }
    if ((*arg).fn_)(tp, head.cast(), arg) < 0 { (*arg).stop = 1; }
    (*arg).count += 1;
}

unsafe fn mall_reoffload(tp: *mut tcf_proto, add: bool, cb: *mut flow_setup_cb_t, cb_priv: *mut core::ffi::c_void, extack: *mut netlink_ext_ack) -> c_int {
    let head: *mut cls_mall_head = rtnl_dereference((*tp).root);
    let mut cls_mall: tc_cls_matchall_offload = core::mem::zeroed();
    let block: *mut tcf_block = (*(*tp).chain).block;
    if tc_skip_hw((*head).flags) { return 0; }
    cls_mall.rule = flow_rule_alloc(tcf_exts_num_actions(&(*head).exts));
    if cls_mall.rule.is_null() { return -ENOMEM; }
    tc_cls_common_offload_init(&mut cls_mall.common, tp, (*head).flags, extack);
    cls_mall.command = if add { TC_CLSMATCHALL_REPLACE } else { TC_CLSMATCHALL_DESTROY };
    cls_mall.cookie = head as c_ulong;
    let err = tc_setup_offload_action(&mut (*cls_mall.rule).action, &(*head).exts, cls_mall.common.extack);
    if err != 0 { kfree(cls_mall.rule.cast()); return if add && tc_skip_sw((*head).flags) { err } else { 0 }; }
    let err = tc_setup_cb_reoffload(block, tp, add, cb, TC_SETUP_CLSMATCHALL, &mut cls_mall, cb_priv, &mut (*head).flags, &mut (*head).in_hw_count);
    tc_cleanup_offload_action(&mut (*cls_mall.rule).action);
    kfree(cls_mall.rule.cast());
    err
}

unsafe fn mall_stats_hw_filter(tp: *mut tcf_proto, head: *mut cls_mall_head, cookie: c_ulong) {
    let mut cls_mall: tc_cls_matchall_offload = core::mem::zeroed();
    let block: *mut tcf_block = (*(*tp).chain).block;
    tc_cls_common_offload_init(&mut cls_mall.common, tp, (*head).flags, core::ptr::null_mut());
    cls_mall.command = TC_CLSMATCHALL_STATS;
    cls_mall.cookie = cookie;
    tc_setup_cb_call(block, TC_SETUP_CLSMATCHALL, &mut cls_mall, false, true);
    tcf_exts_hw_stats_update(&mut (*head).exts, &mut cls_mall.stats, cls_mall.use_act_stats);
}

unsafe fn mall_dump(net: *mut net, tp: *mut tcf_proto, fh: *mut core::ffi::c_void, skb: *mut sk_buff, t: *mut tcmsg, rtnl_held: bool) -> c_int {
    let _ = (net, rtnl_held);
    let head = fh as *mut cls_mall_head;
    if head.is_null() { return (*skb).len; }
    if !tc_skip_hw((*head).flags) { mall_stats_hw_filter(tp, head, head as c_ulong); }
    (*t).tcm_handle = (*head).handle;
    (*skb).len
}

unsafe fn mall_bind_class(fh: *mut core::ffi::c_void, classid: u32, cl: c_ulong, q: *mut core::ffi::c_void, base: c_ulong) {
    let head = fh as *mut cls_mall_head;
    tc_cls_bind_class(classid, cl, q, &mut (*head).res, base);
}

// module_init(cls_mall_init); module_exit(cls_mall_exit);
// MODULE_ALIAS_NET_CLS("matchall");
// MODULE_AUTHOR("Jiri Pirko <jiri@mellanox.com>");
// MODULE_DESCRIPTION("Match-all classifier");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
