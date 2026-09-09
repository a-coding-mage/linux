// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 2007 Patrick McHardy <kaber@trash.net>
 */
// Translated from C. Kernel-provided types, constants, macros, and functions
// referenced below are supplied by the surrounding translation environment.

const RATEEST_HSIZE: usize = 16;

#[repr(C)]
struct xt_rateest_net {
    hash_lock: mutex,
    hash: [hlist_head; RATEEST_HSIZE],
}

static mut xt_rateest_id: c_uint = 0;
static mut jhash_rnd: c_uint = 0; // __read_mostly

unsafe fn xt_rateest_hash(name: *const c_char) -> c_uint {
    (jhash(
        name as *const c_void,
        core::mem::size_of::<xt_rateest_name>(),
        jhash_rnd,
    ) & (RATEEST_HSIZE as c_uint - 1))
}

unsafe fn xt_rateest_hash_insert(xn: *mut xt_rateest_net, est: *mut xt_rateest) {
    let h: c_uint = xt_rateest_hash((*est).name.as_ptr());
    hlist_add_head(&mut (*est).list, &mut (*xn).hash[h as usize]);
}

unsafe fn __xt_rateest_lookup(
    xn: *mut xt_rateest_net,
    name: *const c_char,
) -> *mut xt_rateest {
    let h: c_uint = xt_rateest_hash(name);
    let mut est: *mut xt_rateest;

    hlist_for_each_entry!(est, &mut (*xn).hash[h as usize], list, {
        if strcmp((*est).name.as_ptr(), name) == 0 {
            (*est).refcnt += 1;
            return est;
        }
    });
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xt_rateest_lookup(
    net: *mut net,
    name: *const c_char,
) -> *mut xt_rateest {
    let xn = net_generic(net, xt_rateest_id) as *mut xt_rateest_net;
    mutex_lock(&mut (*xn).hash_lock);
    let est = __xt_rateest_lookup(xn, name);
    mutex_unlock(&mut (*xn).hash_lock);
    est
}

#[no_mangle]
pub unsafe extern "C" fn xt_rateest_put(net: *mut net, est: *mut xt_rateest) {
    let xn = net_generic(net, xt_rateest_id) as *mut xt_rateest_net;
    mutex_lock(&mut (*xn).hash_lock);
    (*est).refcnt -= 1;
    if (*est).refcnt == 0 {
        hlist_del(&mut (*est).list);
        gen_kill_estimator(&mut (*est).rate_est);
        // gen_estimator est_timer() might access est->lock or bstats;
        // wait an RCU grace period before freeing `est`.
        kfree_rcu!(est, rcu);
    }
    mutex_unlock(&mut (*xn).hash_lock);
}

unsafe fn xt_rateest_tg(
    skb: *mut sk_buff,
    par: *const xt_action_param,
) -> c_uint {
    let info = (*par).targinfo as *const xt_rateest_target_info;
    let stats = &mut (*(*info).est).bstats;

    spin_lock_bh(&mut (*(*info).est).lock);
    u64_stats_add(&mut stats.bytes, (*skb).len);
    u64_stats_inc(&mut stats.packets);
    spin_unlock_bh(&mut (*(*info).est).lock);
    XT_CONTINUE
}

unsafe fn xt_rateest_tg_checkentry(par: *const xt_tgchk_param) -> c_int {
    let xn = net_generic((*par).net, xt_rateest_id) as *mut xt_rateest_net;
    let info = (*par).targinfo as *mut xt_rateest_target_info;
    let mut est: *mut xt_rateest;
    let mut cfg: xt_rateest_cfg = core::mem::zeroed();
    let mut ret: c_int;

    if strnlen((*info).name.as_ptr(), core::mem::size_of::<xt_rateest_name>())
        >= core::mem::size_of::<xt_rateest_name>()
    {
        return -ENAMETOOLONG;
    }

    net_get_random_once(
        &mut jhash_rnd as *mut c_uint as *mut c_void,
        core::mem::size_of::<c_uint>(),
    );
    mutex_lock(&mut (*xn).hash_lock);
    est = __xt_rateest_lookup(xn, (*info).name.as_ptr());
    if !est.is_null() {
        mutex_unlock(&mut (*xn).hash_lock);
        if ((!(*info).interval && !(*info).ewma_log)
            || ((*info).interval != (*est).params.interval
                || (*info).ewma_log != (*est).params.ewma_log))
        {
            xt_rateest_put((*par).net, est);
            return -EINVAL;
        }
        (*info).est = est;
        return 0;
    }

    ret = -ENOMEM;
    est = kzalloc_obj::<xt_rateest>();
    if est.is_null() {
        mutex_unlock(&mut (*xn).hash_lock);
        return ret;
    }
    gnet_stats_basic_sync_init(&mut (*est).bstats);
    strscpy(
        (*est).name.as_mut_ptr(),
        (*info).name.as_ptr(),
        core::mem::size_of::<xt_rateest_name>(),
    );
    spin_lock_init(&mut (*est).lock);
    (*est).refcnt = 1;
    (*est).params.interval = (*info).interval;
    (*est).params.ewma_log = (*info).ewma_log;
    cfg.opt.nla_len = nla_attr_size(core::mem::size_of::<gnet_estimator>());
    cfg.opt.nla_type = TCA_STATS_RATE_EST;
    cfg.est.interval = (*info).interval;
    cfg.est.ewma_log = (*info).ewma_log;

    ret = gen_new_estimator(
        &mut (*est).bstats,
        core::ptr::null_mut(),
        &mut (*est).rate_est,
        &mut (*est).lock,
        core::ptr::null_mut(),
        &mut cfg.opt,
    );
    if ret < 0 {
        kfree(est);
        mutex_unlock(&mut (*xn).hash_lock);
        return ret;
    }
    (*info).est = est;
    xt_rateest_hash_insert(xn, est);
    mutex_unlock(&mut (*xn).hash_lock);
    0
}

unsafe fn xt_rateest_tg_destroy(par: *const xt_tgdtor_param) {
    let info = (*par).targinfo as *const xt_rateest_target_info;
    xt_rateest_put((*par).net, (*info).est);
}

#[repr(C)]
struct xt_rateest_cfg {
    opt: nlattr,
    est: gnet_estimator,
}

static mut xt_rateest_tg_reg: [xt_target; 2] = [
    xt_target {
        name: *b"RATEEST\0",
        revision: 0,
        family: NFPROTO_IPV4,
        target: Some(xt_rateest_tg),
        checkentry: Some(xt_rateest_tg_checkentry),
        destroy: Some(xt_rateest_tg_destroy),
        targetsize: core::mem::size_of::<xt_rateest_target_info>(),
        usersize: core::mem::offset_of!(xt_rateest_target_info, est),
        me: THIS_MODULE,
    },
    // CONFIG_IP6_NF_IPTABLES conditional entry.
    xt_target {
        name: *b"RATEEST\0",
        revision: 0,
        family: NFPROTO_IPV6,
        target: Some(xt_rateest_tg),
        checkentry: Some(xt_rateest_tg_checkentry),
        destroy: Some(xt_rateest_tg_destroy),
        targetsize: core::mem::size_of::<xt_rateest_target_info>(),
        usersize: core::mem::offset_of!(xt_rateest_target_info, est),
        me: THIS_MODULE,
    },
];

unsafe fn xt_rateest_net_init(net: *mut net) -> c_int {
    let xn = net_generic(net, xt_rateest_id) as *mut xt_rateest_net;
    mutex_init(&mut (*xn).hash_lock);
    for i in 0..(*xn).hash.len() {
        INIT_HLIST_HEAD(&mut (*xn).hash[i]);
    }
    0
}

static mut xt_rateest_net_ops: pernet_operations = pernet_operations {
    init: Some(xt_rateest_net_init),
    id: &mut xt_rateest_id,
    size: core::mem::size_of::<xt_rateest_net>(),
};

unsafe fn xt_rateest_tg_init() -> c_int {
    let err = register_pernet_subsys(&mut xt_rateest_net_ops);
    if err != 0 {
        return err;
    }
    xt_register_targets(
        xt_rateest_tg_reg.as_mut_ptr(),
        xt_rateest_tg_reg.len(),
    )
}

unsafe fn xt_rateest_tg_fini() {
    xt_unregister_targets(xt_rateest_tg_reg.as_mut_ptr(), xt_rateest_tg_reg.len());
    unregister_pernet_subsys(&mut xt_rateest_net_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
