// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/core/dst.c	Protocol independent destination cache.
 *
 * Authors:		Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
 */

// Dependencies are supplied by the surrounding kernel translation.

pub unsafe extern "C" fn dst_discard_out(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    kfree_skb(skb);
    0
}

pub static mut dst_default_metrics: dst_metrics = dst_metrics {
    // This initializer forces placement in the const section in the C source.
    refcnt: REFCOUNT_INIT!(1),
    ..unsafe { core::mem::zeroed() }
};

pub unsafe extern "C" fn dst_init(
    dst: *mut dst_entry,
    ops: *mut dst_ops,
    dev: *mut net_device,
    initial_obsolete: i32,
    flags: u16,
) {
    (*dst).dev = dev;
    netdev_hold(dev, &mut (*dst).dev_tracker, GFP_ATOMIC);
    (*dst).ops = ops;
    dst_init_metrics(dst, dst_default_metrics.metrics, true);
    (*dst).expires = 0;
    #[cfg(CONFIG_XFRM)]
    { (*dst).xfrm = core::ptr::null_mut(); }
    (*dst).input = Some(dst_discard);
    (*dst).output = Some(dst_discard_out);
    (*dst).error = 0;
    (*dst).obsolete = initial_obsolete;
    (*dst).header_len = 0;
    (*dst).trailer_len = 0;
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    { (*dst).tclassid = 0; }
    (*dst).lwtstate = core::ptr::null_mut();
    rcuref_init(&mut (*dst).__rcuref, 1);
    INIT_LIST_HEAD!(&mut (*dst).rt_uncached);
    (*dst).rt_uncached_list = core::ptr::null_mut();
    (*dst).__use = 0;
    (*dst).lastuse = jiffies;
    (*dst).flags = flags;
    if flags & DST_NOCOUNT == 0 {
        dst_entries_add(ops, 1);
    }
}

pub unsafe extern "C" fn dst_alloc(
    ops: *mut dst_ops,
    dev: *mut net_device,
    initial_obsolete: i32,
    flags: u16,
) -> *mut core::ffi::c_void {
    if !(*ops).gc.is_none() && flags & DST_NOCOUNT == 0
        && dst_entries_get_fast(ops) > (*ops).gc_thresh
    {
        ((*ops).gc.unwrap())(ops);
    }
    let dst = kmem_cache_alloc((*ops).kmem_cachep, GFP_ATOMIC);
    if dst.is_null() { return core::ptr::null_mut(); }
    dst_init(dst as *mut dst_entry, ops, dev, initial_obsolete, flags);
    dst as *mut core::ffi::c_void
}

unsafe fn dst_destroy(dst: *mut dst_entry) {
    let mut child: *mut dst_entry = core::ptr::null_mut();
    smp_rmb();
    #[cfg(CONFIG_XFRM)]
    if !(*dst).xfrm.is_null() {
        let xdst = dst as *mut xfrm_dst;
        child = (*xdst).child;
    }
    if let Some(destroy) = (*(*dst).ops).destroy { destroy(dst); }
    netdev_put((*dst).dev, &mut (*dst).dev_tracker);
    lwtstate_put((*dst).lwtstate);
    if (*dst).flags & DST_METADATA != 0 {
        metadata_dst_free(dst as *mut metadata_dst);
    } else {
        kmem_cache_free((*(*dst).ops).kmem_cachep, dst);
    }
    if !child.is_null() { dst_release_immediate(child); }
}

unsafe fn dst_destroy_rcu(head: *mut rcu_head) {
    let dst = container_of!(head, dst_entry, rcu_head);
    dst_destroy(dst);
}

pub unsafe extern "C" fn dst_dev_put(dst: *mut dst_entry) {
    let dev = (*dst).dev;
    WRITE_ONCE!((*dst).obsolete, DST_OBSOLETE_DEAD);
    if let Some(ifdown) = (*(*dst).ops).ifdown { ifdown(dst, dev); }
    WRITE_ONCE!((*dst).input, Some(dst_discard));
    WRITE_ONCE!((*dst).output, Some(dst_discard_out));
    rcu_assign_pointer!((*dst).dev_rcu, blackhole_netdev);
    netdev_ref_replace(dev, blackhole_netdev, &mut (*dst).dev_tracker, GFP_ATOMIC);
}

unsafe fn dst_count_dec(dst: *mut dst_entry) {
    if (*dst).flags & DST_NOCOUNT == 0 { dst_entries_add((*dst).ops, -1); }
}

pub unsafe extern "C" fn dst_release(dst: *mut dst_entry) {
    if !dst.is_null() && rcuref_put(&mut (*dst).__rcuref) {
        #[cfg(CONFIG_DST_CACHE)]
        if (*dst).flags & DST_METADATA != 0 {
            let md = dst as *mut metadata_dst;
            if (*md).type_ == METADATA_IP_TUNNEL { dst_cache_reset_now(&mut (*md).u.tun_info.dst_cache); }
        }
        dst_count_dec(dst);
        call_rcu_hurry(&mut (*dst).rcu_head, dst_destroy_rcu);
    }
}

pub unsafe extern "C" fn dst_release_immediate(dst: *mut dst_entry) {
    if !dst.is_null() && rcuref_put(&mut (*dst).__rcuref) {
        dst_count_dec(dst);
        dst_destroy(dst);
    }
}

pub unsafe extern "C" fn dst_cow_metrics_generic(dst: *mut dst_entry, old: usize) -> *mut u32 {
    let mut p = kmalloc_obj::<dst_metrics>(GFP_ATOMIC);
    if !p.is_null() {
        let old_p = __DST_METRICS_PTR(old) as *mut dst_metrics;
        (*p).refcnt = refcount_set_value(1);
        core::ptr::copy_nonoverlapping((*old_p).metrics.as_ptr(), (*p).metrics.as_mut_ptr(), (*p).metrics.len());
        let new = p as usize;
        let prev = cmpxchg(&mut (*dst)._metrics, old, new);
        if prev != old {
            kfree(p as *mut core::ffi::c_void);
            p = __DST_METRICS_PTR(prev) as *mut dst_metrics;
            if prev & DST_METRICS_READ_ONLY != 0 { p = core::ptr::null_mut(); }
        } else if prev & DST_METRICS_REFCOUNTED != 0 && refcount_dec_and_test(&mut (*old_p).refcnt) {
            kfree(old_p as *mut core::ffi::c_void);
        }
    }
    BUILD_BUG_ON!(core::mem::offset_of!(dst_metrics, metrics) != 0);
    p as *mut u32
}

pub unsafe extern "C" fn __dst_destroy_metrics_generic(dst: *mut dst_entry, old: usize) {
    let new = (&raw const dst_default_metrics as usize) | DST_METRICS_READ_ONLY;
    let prev = cmpxchg(&mut (*dst)._metrics, old, new);
    if prev == old { kfree(__DST_METRICS_PTR(old)); }
}

pub unsafe extern "C" fn dst_blackhole_check(_dst: *mut dst_entry, _cookie: u32) -> *mut dst_entry { core::ptr::null_mut() }
pub unsafe extern "C" fn dst_blackhole_cow_metrics(_dst: *mut dst_entry, _old: usize) -> *mut u32 { core::ptr::null_mut() }
pub unsafe extern "C" fn dst_blackhole_neigh_lookup(_dst: *const dst_entry, _skb: *mut sk_buff, _daddr: *const core::ffi::c_void) -> *mut neighbour { core::ptr::null_mut() }
pub unsafe extern "C" fn dst_blackhole_update_pmtu(_dst: *mut dst_entry, _sk: *mut sock, _skb: *mut sk_buff, _mtu: u32, _confirm_neigh: bool) {}
pub unsafe extern "C" fn dst_blackhole_redirect(_dst: *mut dst_entry, _sk: *mut sock, _skb: *mut sk_buff) {}

pub unsafe extern "C" fn dst_blackhole_mtu(dst: *const dst_entry) -> u32 {
    let mtu = dst_metric_raw(dst, RTAX_MTU);
    if mtu != 0 { mtu } else { (*dst_dev(dst)).mtu }
}

static mut dst_blackhole_ops: dst_ops = dst_ops {
    family: AF_UNSPEC,
    neigh_lookup: Some(dst_blackhole_neigh_lookup), check: Some(dst_blackhole_check),
    cow_metrics: Some(dst_blackhole_cow_metrics), update_pmtu: Some(dst_blackhole_update_pmtu),
    redirect: Some(dst_blackhole_redirect), mtu: Some(dst_blackhole_mtu),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn __metadata_dst_init(md_dst: *mut metadata_dst, type_: metadata_type, optslen: u8) {
    let dst = &mut (*md_dst).dst;
    dst_init(dst, &raw mut dst_blackhole_ops, core::ptr::null_mut(), DST_OBSOLETE_NONE, DST_METADATA | DST_NOCOUNT);
    core::ptr::write_bytes(dst.add(1) as *mut u8, 0, core::mem::size_of::<metadata_dst>() + optslen as usize - core::mem::size_of::<dst_entry>());
    (*md_dst).type_ = type_;
}

pub unsafe extern "C" fn metadata_dst_alloc(optslen: u8, type_: metadata_type, flags: gfp_t) -> *mut metadata_dst {
    let md = kmalloc_flex::<metadata_dst>(optslen, flags);
    if md.is_null() { return core::ptr::null_mut(); }
    __metadata_dst_init(md, type_, optslen); md
}

pub unsafe extern "C" fn metadata_dst_free(md_dst: *mut metadata_dst) {
    #[cfg(CONFIG_DST_CACHE)]
    if (*md_dst).type_ == METADATA_IP_TUNNEL { dst_cache_destroy(&mut (*md_dst).u.tun_info.dst_cache); }
    if (*md_dst).type_ == METADATA_XFRM { dst_release((*md_dst).u.xfrm_info.dst_orig); }
    kfree(md_dst as *mut core::ffi::c_void);
}

pub unsafe extern "C" fn metadata_dst_alloc_percpu(optslen: u8, type_: metadata_type, flags: gfp_t) -> *mut metadata_dst {
    let md = __alloc_percpu_gfp(core::mem::size_of::<metadata_dst>() + optslen as usize, core::mem::align_of::<metadata_dst>(), flags);
    if md.is_null() { return core::ptr::null_mut(); }
    for_each_possible_cpu!(cpu) { __metadata_dst_init(per_cpu_ptr(md, cpu), type_, optslen); }
    md
}

pub unsafe extern "C" fn metadata_dst_free_percpu(md_dst: *mut metadata_dst) {
    for_each_possible_cpu!(cpu) {
        let one = per_cpu_ptr(md_dst, cpu);
        #[cfg(CONFIG_DST_CACHE)]
        if (*one).type_ == METADATA_IP_TUNNEL { dst_cache_destroy(&mut (*one).u.tun_info.dst_cache); }
        if (*one).type_ == METADATA_XFRM { dst_release((*one).u.xfrm_info.dst_orig); }
    }
    free_percpu(md_dst);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
