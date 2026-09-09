/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from net/ip6_route.h. C dependencies are supplied externally. */

#[repr(C)]
pub struct route_info {
    pub type_: u8,
    pub length: u8,
    pub prefix_len: u8,
    pub reserved_h: u8,
    pub route_pref: u8,
    pub reserved_l: u8,
    pub lifetime: u32,
    pub prefix: [u8; 0], /* 0, 8 or 16 */
}

pub const RT6_LOOKUP_F_IFACE: u32 = 0x00000001;
pub const RT6_LOOKUP_F_REACHABLE: u32 = 0x00000002;
pub const RT6_LOOKUP_F_HAS_SADDR: u32 = 0x00000004;
pub const RT6_LOOKUP_F_SRCPREF_TMP: u32 = 0x00000008;
pub const RT6_LOOKUP_F_SRCPREF_PUBLIC: u32 = 0x00000010;
pub const RT6_LOOKUP_F_SRCPREF_COA: u32 = 0x00000020;
pub const RT6_LOOKUP_F_IGNORE_LINKSTATE: u32 = 0x00000040;
pub const RT6_LOOKUP_F_DST_NOREF: u32 = 0x00000080;

/* IPv6 jumbograms are not supported (RFC 2675). */
pub const IP6_MAX_MTU: usize = 0xFFFF + core::mem::size_of::<ipv6hdr>();

#[inline]
pub unsafe fn rt6_srcprefs2flags(srcprefs: u32) -> i32 {
    ((srcprefs & IPV6_PREFER_SRC_MASK) << 3) as i32
}
#[inline]
pub unsafe fn rt6_flags2srcprefs(flags: i32) -> u32 {
    ((flags >> 3) as u32) & IPV6_PREFER_SRC_MASK
}
#[inline]
pub unsafe fn rt6_need_strict(daddr: *const in6_addr) -> bool {
    (ipv6_addr_type(daddr) & (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL | IPV6_ADDR_LOOPBACK)) != 0
}
#[inline]
pub unsafe fn rt6_qualify_for_ecmp(f6i: *const fib6_info) -> bool {
    ((*f6i).fib6_flags & RTF_ADDRCONF) == 0 && (*f6i).nh.is_null() && (*(*f6i).fib6_nh).fib_nh_gw_family != 0
}

#[cfg(feature = "CONFIG_IPV6")]
extern "C" { pub fn ip6_route_input(skb: *mut sk_buff); }
#[cfg(not(feature = "CONFIG_IPV6"))]
#[inline] pub unsafe fn ip6_route_input(_skb: *mut sk_buff) {}

extern "C" {
    pub fn ip6_route_input_lookup(net: *mut net, dev: *mut net_device, fl6: *mut flowi6, skb: *const sk_buff, flags: i32) -> *mut dst_entry;
    pub fn ip6_route_output_flags(net: *mut net, sk: *const sock, fl6: *mut flowi6, flags: i32) -> *mut dst_entry;
    pub fn ip6_route_lookup(net: *mut net, fl6: *mut flowi6, skb: *const sk_buff, flags: i32) -> *mut dst_entry;
    pub fn ip6_pol_route(net: *mut net, table: *mut fib6_table, ifindex: i32, fl6: *mut flowi6, skb: *const sk_buff, flags: i32) -> *mut rt6_info;
    pub fn ip6_route_init_special_entries();
    pub fn ip6_route_init() -> i32;
    pub fn ip6_route_cleanup();
    pub fn ipv6_route_ioctl(net: *mut net, cmd: u32, rtmsg: *mut in6_rtmsg) -> i32;
    pub fn ip6_route_add(cfg: *mut fib6_config, gfp_flags: gfp_t, extack: *mut netlink_ext_ack) -> i32;
    pub fn ip6_ins_rt(net: *mut net, f6i: *mut fib6_info) -> i32;
    pub fn ip6_del_rt_reason(net: *mut net, f6i: *mut fib6_info, del_reason: rt_del_reason) -> i32;
    pub fn rt6_flush_exceptions(f6i: *mut fib6_info);
    pub fn rt6_age_exceptions(f6i: *mut fib6_info, gc_args: *mut fib6_gc_args, now: c_ulong);
    pub fn rt6_lookup(net: *mut net, daddr: *const in6_addr, saddr: *const in6_addr, oif: i32, skb: *const sk_buff, flags: i32) -> *mut rt6_info;
    pub fn rt6_multipath_hash(net: *const net, fl6: *const flowi6, skb: *const sk_buff, hkeys: *mut flow_keys) -> u32;
    pub fn icmp6_dst_alloc(dev: *mut net_device, fl6: *mut flowi6) -> *mut dst_entry;
    pub fn fib6_force_start_gc(net: *mut net);
    pub fn addrconf_f6i_alloc(net: *mut net, idev: *mut inet6_dev, addr: *const in6_addr, anycast: bool, gfp_flags: gfp_t, extack: *mut netlink_ext_ack) -> *mut fib6_info;
    pub fn ip6_dst_alloc(net: *mut net, dev: *mut net_device, flags: i32) -> *mut rt6_info;
    pub fn rt6_get_dflt_router(net: *mut net, addr: *const in6_addr, dev: *mut net_device) -> *mut fib6_info;
    pub fn rt6_add_dflt_router(net: *mut net, gwaddr: *const in6_addr, dev: *mut net_device, pref: u32, defrtr_usr_metric: u32, lifetime: i32) -> *mut fib6_info;
    pub fn rt6_purge_dflt_routers(net: *mut net);
    pub fn rt6_route_rcv(dev: *mut net_device, opt: *mut u8, len: i32, gwaddr: *const in6_addr) -> i32;
    pub fn ip6_update_pmtu(skb: *mut sk_buff, net: *mut net, mtu: u32, oif: i32, mark: u32, uid: kuid_t);
    pub fn ip6_sk_update_pmtu(skb: *mut sk_buff, sk: *mut sock, mtu: u32);
    pub fn ip6_redirect(skb: *mut sk_buff, net: *mut net, oif: i32, mark: u32, uid: kuid_t);
    pub fn ip6_redirect_no_header(skb: *mut sk_buff, net: *mut net, oif: i32);
    pub fn ip6_sk_redirect(skb: *mut sk_buff, sk: *mut sock);
    pub fn rt6_dump_route(f6i: *mut fib6_info, p_arg: *mut core::ffi::c_void, skip: u32) -> i32;
    pub fn rt6_mtu_change(dev: *mut net_device, mtu: u32);
    pub fn rt6_remove_prefsrc(ifp: *mut inet6_ifaddr);
    pub fn rt6_clean_tohost(net: *mut net, gateway: *mut in6_addr);
    pub fn rt6_sync_up(dev: *mut net_device, nh_flags: u8);
    pub fn rt6_disable_ip(dev: *mut net_device, event: c_ulong);
    pub fn rt6_sync_down_dev(dev: *mut net_device, event: c_ulong);
    pub fn rt6_multipath_rebalance(f6i: *mut fib6_info);
    pub fn rt6_uncached_list_add(rt: *mut rt6_info);
    pub fn rt6_uncached_list_del(rt: *mut rt6_info);
    pub fn ip6_sk_dst_store_flow(sk: *mut sock, dst: *mut dst_entry, fl6: *const flowi6);
    pub fn ip6_mtu(dst: *const dst_entry) -> u32;
    pub fn ip6_mtu_from_fib6(res: *const fib6_result, daddr: *const in6_addr, saddr: *const in6_addr) -> u32;
    pub fn ip6_neigh_lookup(gw: *const in6_addr, dev: *mut net_device, skb: *mut sk_buff, daddr: *const core::ffi::c_void) -> *mut neighbour;
    pub fn ip6_del_rt_reason(net: *mut net, f6i: *mut fib6_info, del_reason: rt_del_reason) -> i32;
    pub fn ip6_del_rt(net: *mut net, f6i: *mut fib6_info, skip_notify: bool) -> i32;
    pub fn ip6_fragment(net: *mut net, sk: *mut sock, skb: *mut sk_buff, output: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> i32>) -> i32;
}

#[inline] pub unsafe fn ip6_route_output(net: *mut net, sk: *const sock, fl6: *mut flowi6) -> *mut dst_entry { ip6_route_output_flags(net, sk, fl6, 0) }
#[inline] pub unsafe fn ip6_rt_put_flags(rt: *mut rt6_info, flags: i32) { if (flags & RT6_LOOKUP_F_DST_NOREF as i32) == 0 || !list_empty(&(*rt).dst.rt_uncached) { ip6_rt_put(rt); } }

#[inline] pub unsafe fn ip6_route_get_saddr(net: *mut net, f6i: *mut fib6_info, daddr: *const in6_addr, prefs: u32, l3mdev_index: i32, saddr: *mut in6_addr) -> i32 {
    rcu_read_lock(); let l3mdev = dev_get_by_index_rcu(net, l3mdev_index); let dev = if f6i.is_null() { core::ptr::null_mut() } else { fib6_info_nh_dev(f6i) }; let same_vrf = l3mdev.is_null() || l3mdev_master_dev_rcu(dev) == l3mdev; let err = if !f6i.is_null() && (*f6i).fib6_prefsrc.plen != 0 && same_vrf { *saddr = (*f6i).fib6_prefsrc.addr; 0 } else { ipv6_dev_get_saddr(net, if same_vrf { dev } else { l3mdev }, daddr, prefs, saddr) }; rcu_read_unlock(); err
}

#[repr(C)] pub struct rt6_rtnl_dump_arg { pub skb: *mut sk_buff, pub cb: *mut netlink_callback, pub net: *mut net, pub filter: fib_dump_filter }

#[inline] pub unsafe fn ip6_dst_store(sk: *mut sock, dst: *mut dst_entry, daddr_set: bool, saddr_set: bool) { let np = inet6_sk(sk); (*np).dst_cookie = rt6_get_cookie(dst_rt6_info(dst)); sk_setup_caps(sk, dst); (*np).daddr_cache = daddr_set; #[cfg(feature = "CONFIG_IPV6_SUBTREES")] { (*np).saddr_cache = saddr_set; } }

#[inline] pub unsafe fn skb_rt6_info(skb: *const sk_buff) -> *const rt6_info { let dst = skb_dst(skb); if !dst.is_null() { dst_rt6_info(dst) } else { core::ptr::null() } }

#[inline] pub unsafe fn ipv6_unicast_destination(skb: *const sk_buff) -> bool { ((*dst_rt6_info(skb_dst(skb))).rt6i_flags & RTF_LOCAL) != 0 }
#[inline] pub unsafe fn __ipv6_anycast_destination(rt6i_dst: *const rt6key, rt6i_flags: u32, daddr: *const in6_addr) -> bool { (rt6i_flags & RTF_ANYCAST) != 0 || ((*rt6i_dst).plen < 127 && (rt6i_flags & (RTF_GATEWAY | RTF_NONEXTHOP)) == 0 && ipv6_addr_equal(&(*rt6i_dst).addr, daddr)) }
#[inline] pub unsafe fn ipv6_anycast_destination(dst: *const dst_entry, daddr: *const in6_addr) -> bool { let rt = dst_rt6_info(dst); __ipv6_anycast_destination(&(*rt).rt6i_dst, (*rt).rt6i_flags, daddr) }

#[inline] pub unsafe fn rt6_nexthop(rt: *const rt6_info, daddr: *const in6_addr) -> *const in6_addr { if ((*rt).rt6i_flags & RTF_GATEWAY) != 0 { &(*rt).rt6i_gateway } else if ((*rt).rt6i_flags & RTF_CACHE) != 0 { &(*rt).rt6i_dst.addr } else { daddr } }

#[inline] pub unsafe fn rt6_duplicate_nexthop(a: *mut fib6_info, b: *mut fib6_info) -> bool {
    if !(*a).nh.is_null() || !(*b).nh.is_null() { return nexthop_cmp((*a).nh, (*b).nh); }
    let nha = (*a).fib6_nh; let nhb = (*b).fib6_nh;
    (*nha).fib_nh_dev == (*nhb).fib_nh_dev && ipv6_addr_equal(&(*nha).fib_nh_gw6, &(*nhb).fib_nh_gw6) && !lwtunnel_cmp_encap((*nha).fib_nh_lws, (*nhb).fib_nh_lws)
}

#[inline] pub unsafe fn ip6_sk_accept_pmtu(sk: *const sock) -> bool { let p = READ_ONCE(inet6_sk(sk).pmtudisc); p != IPV6_PMTUDISC_INTERFACE && p != IPV6_PMTUDISC_OMIT }
#[inline] pub unsafe fn ip6_sk_ignore_df(sk: *const sock) -> bool { let p = READ_ONCE(inet6_sk(sk).pmtudisc); p < IPV6_PMTUDISC_DO || p == IPV6_PMTUDISC_OMIT }

/* The remaining inline helpers retain their C dependency calls directly. */
#[inline] pub unsafe fn dst6_mtu(dst: *const dst_entry) -> u32 { ip6_mtu(dst) }
#[inline] pub unsafe fn ip6_skb_dst_mtu(skb: *const sk_buff) -> u32 { dst_mtu(skb_dst(skb)) }

#[inline] pub unsafe fn ip6_dst_mtu_maybe_forward(dst: *const dst_entry, forwarding: bool) -> u32 {
    let mut mtu = if !forwarding || dst_metric_locked(dst, RTAX_MTU) { dst_metric_raw(dst, RTAX_MTU) } else { 0 };
    if mtu == 0 { mtu = IPV6_MIN_MTU; rcu_read_lock(); let idev = __in6_dev_get(dst_dev_rcu(dst)); if !idev.is_null() { mtu = READ_ONCE((*idev).cnf.mtu6); } rcu_read_unlock(); }
    mtu - lwtunnel_headroom((*dst).lwtstate, mtu)
}

#[inline] pub unsafe fn ip6_dst_mtu_configured(dst: *const dst_entry) -> u32 {
    let rt = dst_rt6_info(dst); let mut mtu = 0; rcu_read_lock(); let from = rcu_dereference((*rt).from); if !from.is_null() { mtu = (*from).fib6_pmtu; } if mtu == 0 { mtu = IPV6_MIN_MTU; let idev = __in6_dev_get(dst_dev_rcu(dst)); if !idev.is_null() { mtu = max_t(mtu, READ_ONCE((*idev).cnf.mtu6)); } } rcu_read_unlock(); mtu = min_t(mtu, IP6_MAX_MTU as u32); mtu - lwtunnel_headroom((*dst).lwtstate, mtu)
}

/* CONFIG_IPV6-gated declarations and inline fallbacks are preserved by cfg above. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
