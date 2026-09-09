// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm4_policy.c
 *
 * Changes:
 *	Kazunori MIYAZAWA @USAGI
 * 	YOSHIFUJI Hideaki @USAGI
 *	Split up af-specific portion
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn __xfrm4_dst_lookup(
    fl4: *mut flowi4,
    params: *const xfrm_dst_lookup_params,
) -> *mut dst_entry {
    let mut rt: *mut rtable;

    core::ptr::write_bytes(fl4 as *mut u8, 0, core::mem::size_of::<flowi4>());
    (*fl4).daddr = (*(*params).daddr).a4;
    (*fl4).flowi4_dscp = (*params).dscp;
    (*fl4).flowi4_l3mdev = l3mdev_master_ifindex_by_index((*params).net, (*params).oif);
    (*fl4).flowi4_mark = (*params).mark;
    if !(*params).saddr.is_null() {
        (*fl4).saddr = (*(*params).saddr).a4;
    }
    (*fl4).flowi4_proto = (*params).ipproto;
    (*fl4).uli = (*params).uli;

    rt = __ip_route_output_key((*params).net, fl4);
    if !IS_ERR(rt) {
        return &mut (*rt).dst;
    }

    ERR_CAST(rt)
}

unsafe fn xfrm4_dst_lookup(params: *const xfrm_dst_lookup_params) -> *mut dst_entry {
    let mut fl4: flowi4 = core::mem::zeroed();
    __xfrm4_dst_lookup(&mut fl4, params)
}

unsafe fn xfrm4_get_saddr(
    saddr: *mut xfrm_address_t,
    params: *const xfrm_dst_lookup_params,
) -> i32 {
    let mut fl4: flowi4 = core::mem::zeroed();
    let dst = __xfrm4_dst_lookup(&mut fl4, params);
    if IS_ERR(dst) {
        return -EHOSTUNREACH;
    }

    (*saddr).a4 = fl4.saddr;
    dst_release(dst);
    0
}

unsafe fn xfrm4_fill_dst(xdst: *mut xfrm_dst, dev: *mut net_device, fl: *const flowi) -> i32 {
    let rt = dst_rtable((*xdst).route);
    let fl4 = &(*fl).u.ip4;

    (*xdst).u.rt.rt_iif = fl4.flowi4_iif;
    (*xdst).u.dst.dev = dev;
    netdev_hold(dev, &mut (*xdst).u.dst.dev_tracker, GFP_ATOMIC);

    /* Sheit... I remember I did this right. Apparently,
     * it was magically lost, so this code needs audit */
    (*xdst).u.rt.rt_is_input = (*rt).rt_is_input;
    (*xdst).u.rt.rt_flags = (*rt).rt_flags & (RTCF_BROADCAST | RTCF_MULTICAST | RTCF_LOCAL);
    (*xdst).u.rt.rt_type = (*rt).rt_type;
    (*xdst).u.rt.rt_uses_gateway = (*rt).rt_uses_gateway;
    (*xdst).u.rt.rt_gw_family = (*rt).rt_gw_family;
    if (*rt).rt_gw_family == AF_INET {
        (*xdst).u.rt.rt_gw4 = (*rt).rt_gw4;
    } else if (*rt).rt_gw_family == AF_INET6 {
        (*xdst).u.rt.rt_gw6 = (*rt).rt_gw6;
    }
    (*xdst).u.rt.rt_pmtu = (*rt).rt_pmtu;
    (*xdst).u.rt.rt_mtu_locked = (*rt).rt_mtu_locked;
    rt_add_uncached_list(&mut (*xdst).u.rt);
    0
}

unsafe fn xfrm4_update_pmtu(dst: *mut dst_entry, sk: *mut sock, skb: *mut sk_buff, mtu: u32, confirm_neigh: bool) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;
    ((*(*path).ops).update_pmtu)(path, sk, skb, mtu, confirm_neigh);
}

unsafe fn xfrm4_redirect(dst: *mut dst_entry, sk: *mut sock, skb: *mut sk_buff) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;
    ((*(*path).ops).redirect)(path, sk, skb);
}

unsafe fn xfrm4_dst_destroy(dst: *mut dst_entry) {
    let xdst = dst as *mut xfrm_dst;
    dst_destroy_metrics_generic(dst);
    rt_del_uncached_list(&mut (*xdst).u.rt);
    xfrm_dst_destroy(xdst);
}

static mut xfrm4_dst_ops_template: dst_ops = dst_ops {
    family: AF_INET,
    update_pmtu: Some(xfrm4_update_pmtu),
    redirect: Some(xfrm4_redirect),
    cow_metrics: Some(dst_cow_metrics_generic),
    destroy: Some(xfrm4_dst_destroy),
    ifdown: Some(xfrm_dst_ifdown),
    local_out: Some(__ip_local_out),
    gc_thresh: 32768,
};

static xfrm4_policy_afinfo: xfrm_policy_afinfo = xfrm_policy_afinfo {
    dst_ops: unsafe { &mut xfrm4_dst_ops_template },
    dst_lookup: Some(xfrm4_dst_lookup),
    get_saddr: Some(xfrm4_get_saddr),
    fill_dst: Some(xfrm4_fill_dst),
    blackhole_route: Some(ipv4_blackhole_route),
};

#[cfg(CONFIG_SYSCTL)]
static mut xfrm4_policy_table: [ctl_table; 1] = [ctl_table {
    procname: b"xfrm4_gc_thresh\0".as_ptr() as *mut i8,
    data: unsafe { &mut init_net.xfrm.xfrm4_dst_ops.gc_thresh },
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o644,
    proc_handler: Some(proc_dointvec),
}];

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm4_policy_table_dup(net: *mut net) -> *const ctl_table {
    let table = kmemdup(xfrm4_policy_table.as_ptr(), core::mem::size_of_val(&xfrm4_policy_table), GFP_KERNEL) as *mut ctl_table;
    if table.is_null() { return core::ptr::null(); }
    (*table).data = &mut (*net).xfrm.xfrm4_dst_ops.gc_thresh as *mut _ as *mut core::ffi::c_void;
    table
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm4_net_sysctl_init(net: *mut net) -> i32 {
    let mut table = xfrm4_policy_table.as_ptr();
    let mut owned = false;
    if !net_eq(net, &mut init_net) {
        table = xfrm4_policy_table_dup(net);
        if table.is_null() { return -ENOMEM; }
        owned = true;
    }
    let hdr = register_net_sysctl_sz(net, b"net/ipv4\0".as_ptr() as *const i8, table, ARRAY_SIZE(xfrm4_policy_table));
    if hdr.is_null() {
        if owned { kfree(table as *mut core::ffi::c_void); }
        return -ENOMEM;
    }
    (*net).ipv4.xfrm4_hdr = hdr;
    0
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn xfrm4_net_sysctl_exit(net: *mut net) {
    if (*net).ipv4.xfrm4_hdr.is_null() { return; }
    let table = (*net).ipv4.xfrm4_hdr.ctl_table_arg;
    unregister_net_sysctl_table((*net).ipv4.xfrm4_hdr);
    if !net_eq(net, &mut init_net) { kfree(table as *mut core::ffi::c_void); }
}

#[cfg(not(CONFIG_SYSCTL))]
unsafe fn xfrm4_net_sysctl_init(_net: *mut net) -> i32 { 0 }
#[cfg(not(CONFIG_SYSCTL))]
unsafe fn xfrm4_net_sysctl_exit(_net: *mut net) {}

unsafe fn xfrm4_net_init(net: *mut net) -> i32 {
    core::ptr::copy_nonoverlapping(&xfrm4_dst_ops_template, &mut (*net).xfrm.xfrm4_dst_ops, 1);
    let mut ret = dst_entries_init(&mut (*net).xfrm.xfrm4_dst_ops);
    if ret != 0 { return ret; }
    ret = xfrm4_net_sysctl_init(net);
    if ret != 0 { dst_entries_destroy(&mut (*net).xfrm.xfrm4_dst_ops); }
    ret
}

unsafe fn xfrm4_net_exit(net: *mut net) {
    xfrm4_net_sysctl_exit(net);
    dst_entries_destroy(&mut (*net).xfrm.xfrm4_dst_ops);
}

static mut xfrm4_net_ops: pernet_operations = pernet_operations {
    init: Some(xfrm4_net_init),
    exit: Some(xfrm4_net_exit),
};

unsafe fn xfrm4_policy_init() {
    xfrm_policy_register_afinfo(&xfrm4_policy_afinfo, AF_INET);
}

unsafe fn xfrm4_init() {
    xfrm4_state_init();
    xfrm4_policy_init();
    xfrm4_protocol_init();
    register_pernet_subsys(&mut xfrm4_net_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
