// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm6_policy.c: based on xfrm4_policy.c
 *
 * Authors:
 *\tMitsuru KANDA @USAGI
 *\tKazunori MIYAZAWA @USAGI
 *\tKunihiro Ishiguro <kunihiro@ipinfusion.com>
 *\t\tIPv6 support
 *\tYOSHIFUJI Hideaki
 *\tSplit up af-specific portion
 */

// Linux kernel headers and symbols are supplied by the surrounding translation.

unsafe fn xfrm6_dst_lookup(params: *const xfrm_dst_lookup_params) -> *mut dst_entry {
    let mut fl6: flowi6 = core::mem::zeroed();
    let mut dst: *mut dst_entry;
    let err: i32;

    fl6.flowi6_l3mdev = l3mdev_master_ifindex_by_index((*params).net, (*params).oif);
    fl6.flowi6_mark = (*params).mark;
    core::ptr::copy_nonoverlapping(
        (*params).daddr as *const u8,
        &mut fl6.daddr as *mut _ as *mut u8,
        core::mem::size_of_val(&fl6.daddr),
    );
    if !(*params).saddr.is_null() {
        core::ptr::copy_nonoverlapping(
            (*params).saddr as *const u8,
            &mut fl6.saddr as *mut _ as *mut u8,
            core::mem::size_of_val(&fl6.saddr),
        );
    }

    fl6.flowi4_proto = (*params).ipproto;
    fl6.uli = (*params).uli;

    dst = ip6_route_output((*params).net, core::ptr::null_mut(), &mut fl6);

    err = (*dst).error;
    if (*dst).error != 0 {
        dst_release(dst);
        dst = ERR_PTR(err);
    }

    dst
}

unsafe fn xfrm6_get_saddr(
    saddr: *mut xfrm_address_t,
    params: *const xfrm_dst_lookup_params,
) -> i32 {
    let dst: *mut dst_entry = xfrm6_dst_lookup(params);
    let idev: *mut inet6_dev;
    let dev: *mut net_device;
    let err: i32;

    if IS_ERR(dst) {
        return -EHOSTUNREACH;
    }

    idev = ip6_dst_idev(dst);
    if idev.is_null() {
        dst_release(dst);
        return -EHOSTUNREACH;
    }
    dev = (*idev).dev;
    err = ipv6_dev_get_saddr(
        dev_net(dev),
        dev,
        &(*params).daddr->in6,
        0,
        &mut (*saddr).in6,
    );
    dst_release(dst);
    if err != 0 {
        return -EHOSTUNREACH;
    }
    0
}

unsafe fn xfrm6_fill_dst(
    xdst: *mut xfrm_dst,
    dev: *mut net_device,
    _fl: *const flowi,
) -> i32 {
    let rt: *mut rt6_info = dst_rt6_info((*xdst).route);

    (*xdst).u.dst.dev = dev;
    netdev_hold(dev, &mut (*xdst).u.dst.dev_tracker, GFP_ATOMIC);

    (*xdst).u.rt6.rt6i_idev = in6_dev_get(dev);
    if (*xdst).u.rt6.rt6i_idev.is_null() {
        netdev_put(dev, &mut (*xdst).u.dst.dev_tracker);
        (*xdst).u.dst.dev = core::ptr::null_mut();
        return -ENODEV;
    }

    /* Sheit... I remember I did this right. Apparently,
     * it was magically lost, so this code needs audit */
    (*xdst).u.rt6.rt6i_flags = (*rt).rt6i_flags & (RTF_ANYCAST | RTF_LOCAL);
    (*xdst).route_cookie = rt6_get_cookie(rt);
    (*xdst).u.rt6.rt6i_gateway = (*rt).rt6i_gateway;
    (*xdst).u.rt6.rt6i_dst = (*rt).rt6i_dst;
    (*xdst).u.rt6.rt6i_src = (*rt).rt6i_src;
    rt6_uncached_list_add(&mut (*xdst).u.rt6);

    0
}

unsafe fn xfrm6_update_pmtu(
    dst: *mut dst_entry,
    sk: *mut sock,
    skb: *mut sk_buff,
    mtu: u32,
    confirm_neigh: bool,
) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;
    ((*(*path).ops).update_pmtu)(path, sk, skb, mtu, confirm_neigh);
}

unsafe fn xfrm6_redirect(dst: *mut dst_entry, sk: *mut sock, skb: *mut sk_buff) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;
    ((*(*path).ops).redirect)(path, sk, skb);
}

unsafe fn xfrm6_dst_destroy(dst: *mut dst_entry) {
    let xdst = dst as *mut xfrm_dst;

    dst_destroy_metrics_generic(dst);
    rt6_uncached_list_del(&mut (*xdst).u.rt6);
    if likely(!(*xdst).u.rt6.rt6i_idev.is_null()) {
        in6_dev_put((*xdst).u.rt6.rt6i_idev);
    }
    xfrm_dst_destroy(xdst);
}

unsafe fn xfrm6_dst_ifdown(dst: *mut dst_entry, dev: *mut net_device) {
    let mut xdst = dst as *mut xfrm_dst;
    if (*(*xdst).u.rt6.rt6i_idev).dev == dev {
        let loopback_idev = in6_dev_get((*dev_net(dev)).loopback_dev);

        loop {
            in6_dev_put((*xdst).u.rt6.rt6i_idev);
            (*xdst).u.rt6.rt6i_idev = loopback_idev;
            in6_dev_hold(loopback_idev);
            let next = xfrm_dst_child(&mut (*xdst).u.dst);
            xdst = next as *mut xfrm_dst;
            if !(*xdst).u.dst.xfrm {
                break;
            }
        }

        __in6_dev_put(loopback_idev);
    }

    xfrm_dst_ifdown(dst, dev);
}

static mut xfrm6_dst_ops_template: dst_ops = dst_ops {
    family: AF_INET6,
    update_pmtu: Some(xfrm6_update_pmtu),
    redirect: Some(xfrm6_redirect),
    cow_metrics: Some(dst_cow_metrics_generic),
    destroy: Some(xfrm6_dst_destroy),
    ifdown: Some(xfrm6_dst_ifdown),
    local_out: Some(__ip6_local_out),
    gc_thresh: 32768,
};

static xfrm6_policy_afinfo: xfrm_policy_afinfo = xfrm_policy_afinfo {
    dst_ops: unsafe { &raw mut xfrm6_dst_ops_template },
    dst_lookup: Some(xfrm6_dst_lookup),
    get_saddr: Some(xfrm6_get_saddr),
    fill_dst: Some(xfrm6_fill_dst),
    blackhole_route: Some(ip6_blackhole_route),
};

unsafe fn xfrm6_policy_init() -> i32 {
    xfrm_policy_register_afinfo(&xfrm6_policy_afinfo, AF_INET6)
}

unsafe fn xfrm6_policy_fini() {
    xfrm_policy_unregister_afinfo(&xfrm6_policy_afinfo);
}

// CONFIG_SYSCTL preserves the source's conditional build-time behavior.
#[cfg(CONFIG_SYSCTL)]
static xfrm6_policy_table: [ctl_table; 1] = [ctl_table {
    procname: "xfrm6_gc_thresh",
    data: unsafe { &raw mut init_net.xfrm.xfrm6_dst_ops.gc_thresh },
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec),
}];

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm6_policy_table_dup(net: *mut net) -> *const ctl_table {
    let table = kmemdup(
        xfrm6_policy_table.as_ptr() as *const _,
        core::mem::size_of_val(&xfrm6_policy_table),
        GFP_KERNEL,
    ) as *mut ctl_table;
    if table.is_null() {
        return core::ptr::null();
    }
    (*table).data = &mut (*net).xfrm.xfrm6_dst_ops.gc_thresh;
    table
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm6_net_sysctl_init(net: *mut net) -> i32 {
    let mut table = xfrm6_policy_table.as_ptr();
    if !net_eq(net, &raw const init_net) {
        table = xfrm6_policy_table_dup(net);
        if table.is_null() {
            return -ENOMEM;
        }
    }
    let hdr = register_net_sysctl_sz(net, "net/ipv6", table, xfrm6_policy_table.len());
    if hdr.is_null() {
        if !net_eq(net, &raw const init_net) {
            kfree(table as *mut _);
        }
        return -ENOMEM;
    }
    (*net).ipv6.sysctl.xfrm6_hdr = hdr;
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm6_net_sysctl_exit(net: *mut net) {
    if (*net).ipv6.sysctl.xfrm6_hdr.is_null() {
        return;
    }
    let table = (*(*net).ipv6.sysctl.xfrm6_hdr).ctl_table_arg;
    unregister_net_sysctl_table((*net).ipv6.sysctl.xfrm6_hdr);
    if !net_eq(net, &raw const init_net) {
        kfree(table as *mut _);
    }
}

#[cfg(not(CONFIG_SYSCTL))]
unsafe fn xfrm6_net_sysctl_init(_net: *mut net) -> i32 { 0 }
#[cfg(not(CONFIG_SYSCTL))]
unsafe fn xfrm6_net_sysctl_exit(_net: *mut net) {}

unsafe fn xfrm6_net_init(net: *mut net) -> i32 {
    core::ptr::copy_nonoverlapping(
        &xfrm6_dst_ops_template,
        &mut (*net).xfrm.xfrm6_dst_ops,
        1,
    );
    let mut ret = dst_entries_init(&mut (*net).xfrm.xfrm6_dst_ops);
    if ret != 0 { return ret; }
    ret = xfrm6_net_sysctl_init(net);
    if ret != 0 { dst_entries_destroy(&mut (*net).xfrm.xfrm6_dst_ops); }
    ret
}

unsafe fn xfrm6_net_exit(net: *mut net) {
    xfrm6_net_sysctl_exit(net);
    dst_entries_destroy(&mut (*net).xfrm.xfrm6_dst_ops);
}

static mut xfrm6_net_ops: pernet_operations = pernet_operations {
    init: Some(xfrm6_net_init),
    exit: Some(xfrm6_net_exit),
};

unsafe fn xfrm6_init() -> i32 {
    let mut ret = xfrm6_policy_init();
    if ret != 0 { return ret; }
    ret = xfrm6_state_init();
    if ret != 0 { xfrm6_policy_fini(); return ret; }
    ret = xfrm6_protocol_init();
    if ret != 0 { xfrm6_state_fini(); xfrm6_policy_fini(); return ret; }
    ret = register_pernet_subsys(&mut xfrm6_net_ops);
    if ret != 0 { xfrm6_protocol_fini(); xfrm6_state_fini(); xfrm6_policy_fini(); return ret; }
    ret = xfrm_nat_keepalive_init(AF_INET6);
    if ret != 0 {
        unregister_pernet_subsys(&mut xfrm6_net_ops);
        xfrm6_protocol_fini();
        xfrm6_state_fini();
        xfrm6_policy_fini();
    }
    ret
}

unsafe fn xfrm6_fini() {
    xfrm_nat_keepalive_fini(AF_INET6);
    unregister_pernet_subsys(&mut xfrm6_net_ops);
    xfrm6_protocol_fini();
    xfrm6_policy_fini();
    xfrm6_state_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
