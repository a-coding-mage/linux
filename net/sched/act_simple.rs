// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * net/sched/act_simple.c\tSimple example of an action
 *
 * Authors:\tJamal Hadi Salim (2005-8)
 */

// Linux kernel dependencies supplied by other translation units.

static mut act_simp_ops: tc_action_ops = tc_action_ops {
    kind: "simple", id: TCA_ID_SIMP, owner: THIS_MODULE,
    act: Some(tcf_simp_act), dump: Some(tcf_simp_dump), cleanup: Some(tcf_simp_release),
    init: Some(tcf_simp_init), size: core::mem::size_of::<tcf_defact>(), ..tc_action_ops::default()
};

const SIMP_MAX_DATA: usize = 32;

unsafe fn tcf_simp_act(
    skb: *mut sk_buff,
    a: *const tc_action,
    res: *mut tcf_result,
) -> i32 {
    let d: *mut tcf_defact = to_defact(a);

    spin_lock(&mut (*d).tcf_lock);
    tcf_lastuse_update(&mut (*d).tcf_tm);
    bstats_update(&mut (*d).tcf_bstats, skb);

    /* print policy string followed by _ then packet count
     * Example if this was the 3rd packet and the string was "hello"
     * then it would look like "hello_3" (without quotes)
     */
    pr_info(
        "simple: %s_%llu\n",
        (*d).tcfd_defdata as *const i8,
        u64_stats_read(&(*d).tcf_bstats.packets),
    );
    spin_unlock(&mut (*d).tcf_lock);
    (*d).tcf_action
}

unsafe fn tcf_simp_release(a: *mut tc_action) {
    let d: *mut tcf_defact = to_defact(a);
    kfree((*d).tcfd_defdata);
}

unsafe fn alloc_defdata(d: *mut tcf_defact, defdata: *const nlattr) -> i32 {
    (*d).tcfd_defdata = kzalloc(SIMP_MAX_DATA, GFP_KERNEL);
    if unlikely((*d).tcfd_defdata.is_null()) {
        return -ENOMEM;
    }
    nla_strscpy((*d).tcfd_defdata, defdata, SIMP_MAX_DATA);
    0
}

unsafe fn reset_policy(
    a: *mut tc_action,
    defdata: *const nlattr,
    p: *const tc_defact,
    tp: *mut tcf_proto,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let d: *mut tcf_defact;
    let err: i32;

    err = tcf_action_check_ctrlact((*p).action, tp, &mut goto_ch, extack);
    if err < 0 {
        return err;
    }
    d = to_defact(a);
    spin_lock_bh(&mut (*d).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(a, (*p).action, goto_ch);
    nla_strscpy((*d).tcfd_defdata, defdata, SIMP_MAX_DATA);
    spin_unlock_bh(&mut (*d).tcf_lock);
    if !goto_ch.is_null() {
        tcf_chain_put_by_act(goto_ch);
    }
    0
}

static simple_policy: [nla_policy; (TCA_DEF_MAX + 1) as usize] = [
    nla_policy { len: core::mem::size_of::<tc_defact>() as u16, ..nla_policy::default() },
    nla_policy { type_: NLA_STRING, len: SIMP_MAX_DATA as u16, ..nla_policy::default() },
];

unsafe fn tcf_simp_init(
    net: *mut net,
    nla: *mut nlattr,
    est: *mut nlattr,
    a: *mut *mut tc_action,
    tp: *mut tcf_proto,
    flags: u32,
    extack: *mut netlink_ext_ack,
) -> i32 {
    let tn: *mut tc_action_net = net_generic(net, (*(&raw const act_simp_ops)).net_id);
    let bind = flags & TCA_ACT_FLAGS_BIND != 0;
    let mut tb: [*mut nlattr; (TCA_DEF_MAX + 1) as usize] = [core::ptr::null_mut(); (TCA_DEF_MAX + 1) as usize];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_defact;
    let mut d: *mut tcf_defact;
    let mut exists = false;
    let mut ret = 0;
    let err: i32;
    let index: u32;

    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_DEF_MAX, nla, &raw const simple_policy, core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_DEF_PARMS as usize].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_DEF_PARMS as usize]);
    index = (*parm).index;
    err = tcf_idr_check_alloc(tn, &index as *const u32 as *mut u32, a, bind);
    if err < 0 { return err; }
    exists = err != 0;
    if exists && bind { return ACT_P_BOUND; }
    if tb[TCA_DEF_DATA as usize].is_null() {
        if exists { tcf_idr_release(*a, bind); } else { tcf_idr_cleanup(tn, index); }
        return -EINVAL;
    }
    if !exists {
        ret = tcf_idr_create(tn, index, est, a, &raw const act_simp_ops, bind, false, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        d = to_defact(*a);
        err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
        if err < 0 { tcf_idr_release(*a, bind); return err; }
        err = alloc_defdata(d, tb[TCA_DEF_DATA as usize]);
        if err < 0 { if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } tcf_idr_release(*a, bind); return err; }
        tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
        ret = ACT_P_CREATED;
    } else {
        if flags & TCA_ACT_FLAGS_REPLACE == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
        err = reset_policy(*a, tb[TCA_DEF_DATA as usize], parm, tp, extack);
        if err != 0 { tcf_idr_release(*a, bind); return err; }
    }
    ret
}

unsafe fn tcf_simp_dump(skb: *mut sk_buff, a: *mut tc_action, bind: i32, ref_: i32) -> i32 {
    let b = skb_tail_pointer(skb);
    let d = to_defact(a);
    let mut opt = tc_defact {
        index: (*d).tcf_index,
        refcnt: refcount_read(&(*d).tcf_refcnt) - ref_,
        bindcnt: atomic_read(&(*d).tcf_bindcnt) - bind,
        ..tc_defact::default()
    };
    let mut t: tcf_t = core::mem::zeroed();
    spin_lock_bh(&mut (*d).tcf_lock);
    opt.action = (*d).tcf_action;
    if nla_put(skb, TCA_DEF_PARMS, core::mem::size_of::<tc_defact>(), &opt as *const _ as *const core::ffi::c_void) != 0
        || nla_put_string(skb, TCA_DEF_DATA, (*d).tcfd_defdata) != 0 { goto_nla_put_failure(skb, &mut (*d).tcf_lock, b); return -1; }
    tcf_tm_dump(&mut t, &(*d).tcf_tm);
    if nla_put_64bit(skb, TCA_DEF_TM, core::mem::size_of::<tcf_t>(), &t as *const _ as *const core::ffi::c_void, TCA_DEF_PAD) != 0 { goto_nla_put_failure(skb, &mut (*d).tcf_lock, b); return -1; }
    spin_unlock_bh(&mut (*d).tcf_lock);
    (*skb).len as i32
}

unsafe fn goto_nla_put_failure(skb: *mut sk_buff, lock: *mut spinlock_t, b: *mut u8) {
    spin_unlock_bh(lock);
    nlmsg_trim(skb, b);
}

static mut simp_net_ops: pernet_operations = pernet_operations {
    init: Some(simp_init_net), exit_batch: Some(simp_exit_net), id: unsafe { &raw mut act_simp_ops.net_id }, size: core::mem::size_of::<tc_action_net>(),
};

unsafe fn simp_init_net(net: *mut net) -> i32 {
    let tn = net_generic(net, act_simp_ops.net_id);
    tc_action_net_init(net, tn, &raw const act_simp_ops)
}

unsafe fn simp_exit_net(net_list: *mut list_head) { tc_action_net_exit(net_list, act_simp_ops.net_id); }

// MODULE_ALIAS_NET_ACT("simple");
// MODULE_AUTHOR("Jamal Hadi Salim(2005)");
// MODULE_DESCRIPTION("Simple example action");
// MODULE_LICENSE("GPL");
// module_init(simp_init_module);
// module_exit(simp_cleanup_module);

unsafe fn simp_init_module() -> i32 {
    let ret = tcf_register_action(&raw const act_simp_ops, &raw const simp_net_ops);
    if ret == 0 { pr_info("Simple TC action Loaded\n"); }
    ret
}

unsafe fn simp_cleanup_module() { tcf_unregister_action(&raw const act_simp_ops, &raw const simp_net_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
