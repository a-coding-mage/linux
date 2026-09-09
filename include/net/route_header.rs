/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from route.h. C includes and external dependencies are intentionally omitted. */

#[inline]
pub unsafe fn ip_sock_rt_scope(sk: *const sock) -> u8 {
    if sock_flag(sk, SOCK_LOCALROUTE) { RT_SCOPE_LINK } else { RT_SCOPE_UNIVERSE }
}

#[inline]
pub unsafe fn ip_sock_rt_tos(sk: *const sock) -> u8 {
    read_once((*inet_sk(sk)).tos) & INET_DSCP_MASK
}

pub enum ip_tunnel_info {}
pub enum fib_nh {}
pub enum fib_info {}
pub enum uncached_list {}

#[repr(C)]
pub union RtableGateway {
    pub rt_gw4: __be32,
    pub rt_gw6: in6_addr,
}

#[repr(C)]
pub struct rtable {
    pub dst: dst_entry,
    pub rt_genid: i32,
    pub rt_flags: u32,
    pub rt_type: u16,
    pub rt_is_input: u8,
    pub rt_uses_gateway: u8,
    pub rt_iif: i32,
    pub rt_gw_family: u8,
    pub gateway: RtableGateway,
    pub rt_mtu_locked: u32,
    pub rt_pmtu: u32,
}

#[inline]
pub unsafe fn skb_rtable(skb: *const sk_buff) -> *mut rtable {
    dst_rtable(skb_dst(skb))
}

#[inline]
pub unsafe fn rt_is_input_route(rt: *const rtable) -> bool { (*rt).rt_is_input != 0 }

#[inline]
pub unsafe fn rt_is_output_route(rt: *const rtable) -> bool { (*rt).rt_is_input == 0 }

#[inline]
pub unsafe fn rt_nexthop(rt: *const rtable, daddr: __be32) -> __be32 {
    if (*rt).rt_gw_family == AF_INET { (*rt).gateway.rt_gw4 } else { daddr }
}

#[repr(C)]
pub struct ip_rt_acct { pub o_bytes: u32, pub o_packets: u32, pub i_bytes: u32, pub i_packets: u32 }

#[repr(C)]
pub struct rt_cache_stat {
    pub in_slow_tot: u32, pub in_slow_mc: u32, pub in_no_route: u32, pub in_brd: u32,
    pub in_martian_dst: u32, pub in_martian_src: u32, pub out_slow_tot: u32, pub out_slow_mc: u32,
}

extern "C" {
    pub static mut ip_rt_acct: *mut ip_rt_acct;
    pub fn ip_rt_init() -> i32;
    pub fn rt_cache_flush(net: *mut net);
    pub fn rt_flush_dev(dev: *mut net_device);
}

#[inline]
pub unsafe fn inet_sk_init_flowi4(inet: *const inet_sock, fl4: *mut flowi4) {
    let ip4_opt: *const ip_options_rcu;
    let sk: *const sock;
    let daddr: __be32;
    rcu_read_lock();
    ip4_opt = rcu_dereference((*inet).inet_opt);
    daddr = if !ip4_opt.is_null() && (*ip4_opt).opt.srr { (*ip4_opt).opt.faddr } else { (*inet).inet_daddr };
    rcu_read_unlock();
    sk = &(*inet).sk;
    flowi4_init_output(fl4, (*sk).sk_bound_dev_if, read_once((*sk).sk_mark), ip_sock_rt_tos(sk),
                       ip_sock_rt_scope(sk), (*sk).sk_protocol, inet_sk_flowi_flags(sk), daddr,
                       (*inet).inet_saddr, (*inet).inet_dport, (*inet).inet_sport, sk_uid(sk));
    security_sk_classify_flow(sk, flowi4_to_flowi_common(fl4));
}

extern "C" {
    pub fn ip_route_output_key_hash(net: *mut net, flp: *mut flowi4, skb: *const sk_buff) -> *mut rtable;
    pub fn ip_route_output_key_hash_rcu(net: *mut net, flp: *mut flowi4, res: *mut fib_result, skb: *const sk_buff) -> *mut rtable;
}

#[inline]
pub unsafe fn __ip_route_output_key(net: *mut net, flp: *mut flowi4) -> *mut rtable { ip_route_output_key_hash(net, flp, core::ptr::null()) }

extern "C" {
    pub fn ip_route_output_flow(net: *mut net, flp: *mut flowi4, sk: *const sock) -> *mut rtable;
    pub fn ipv4_blackhole_route(net: *mut net, dst_orig: *mut dst_entry) -> *mut dst_entry;
}

#[inline]
pub unsafe fn ip_route_output_key(net: *mut net, flp: *mut flowi4) -> *mut rtable { ip_route_output_flow(net, flp, core::ptr::null()) }

#[inline]
pub unsafe fn ip_route_output(net: *mut net, daddr: __be32, saddr: __be32, dscp: dscp_t, oif: i32, scope: u8) -> *mut rtable {
    let mut fl4 = flowi4 { flowi4_oif: oif, flowi4_dscp: dscp, flowi4_scope: scope, daddr, saddr };
    ip_route_output_key(net, &mut fl4)
}

#[inline]
pub unsafe fn ip_route_output_ports(net: *mut net, fl4: *mut flowi4, sk: *const sock, daddr: __be32, saddr: __be32, dport: __be16, sport: __be16, proto: u8, tos: u8, oif: i32) -> *mut rtable {
    flowi4_init_output(fl4, oif, if !sk.is_null() { read_once((*sk).sk_mark) } else { 0 }, tos,
        if !sk.is_null() { ip_sock_rt_scope(sk) } else { RT_SCOPE_UNIVERSE }, proto,
        if !sk.is_null() { inet_sk_flowi_flags(sk) } else { 0 }, daddr, saddr, dport, sport, sock_net_uid(net, sk));
    if !sk.is_null() { security_sk_classify_flow(sk, flowi4_to_flowi_common(fl4)); }
    ip_route_output_flow(net, fl4, sk)
}

extern "C" {
    pub fn ip_mc_validate_source(skb: *mut sk_buff, daddr: __be32, saddr: __be32, dscp: dscp_t, dev: *mut net_device, in_dev: *mut in_device, itag: *mut u32) -> skb_drop_reason;
    pub fn ip_route_input_noref(skb: *mut sk_buff, daddr: __be32, saddr: __be32, dscp: dscp_t, dev: *mut net_device) -> skb_drop_reason;
    pub fn ip_route_use_hint(skb: *mut sk_buff, daddr: __be32, saddr: __be32, dscp: dscp_t, dev: *mut net_device, hint: *const sk_buff) -> skb_drop_reason;
}

#[inline]
pub unsafe fn ip_route_input(skb: *mut sk_buff, dst: __be32, src: __be32, dscp: dscp_t, devin: *mut net_device) -> skb_drop_reason {
    rcu_read_lock();
    let mut reason = ip_route_input_noref(skb, dst, src, dscp, devin);
    if reason == 0 { skb_dst_force(skb); if skb_dst(skb).is_null() { reason = SKB_DROP_REASON_NOT_SPECIFIED; } }
    rcu_read_unlock(); reason
}

extern "C" {
    pub fn ipv4_update_pmtu(skb: *mut sk_buff, net: *mut net, mtu: u32, oif: i32, protocol: u8);
    pub fn ipv4_sk_update_pmtu(skb: *mut sk_buff, sk: *mut sock, mtu: u32);
    pub fn ipv4_redirect(skb: *mut sk_buff, net: *mut net, oif: i32, protocol: u8);
    pub fn ipv4_sk_redirect(skb: *mut sk_buff, sk: *mut sock);
    pub fn ip_rt_send_redirect(skb: *mut sk_buff);
    pub fn inet_addr_type(net: *mut net, addr: __be32) -> u32;
    pub fn inet_addr_type_table(net: *mut net, addr: __be32, tb_id: u32) -> u32;
    pub fn inet_dev_addr_type(net: *mut net, dev: *const net_device, addr: __be32) -> u32;
    pub fn inet_addr_type_dev_table(net: *mut net, dev: *const net_device, addr: __be32) -> u32;
    pub fn ip_rt_multicast_event(dev: *mut in_device);
    pub fn ip_rt_ioctl(net: *mut net, cmd: u32, rt: *mut rtentry) -> i32;
    pub fn ip_rt_get_source(src: *mut u8, skb: *mut sk_buff, rt: *mut rtable);
    pub fn rt_dst_alloc(dev: *mut net_device, flags: u32, type_: u16, noxfrm: bool) -> *mut rtable;
    pub fn rt_dst_clone(dev: *mut net_device, rt: *mut rtable) -> *mut rtable;
    pub fn fib_add_ifaddr(ifa: *mut in_ifaddr);
    pub fn fib_del_ifaddr(ifa: *mut in_ifaddr, ifa2: *mut in_ifaddr);
    pub fn fib_modify_prefix_metric(ifa: *mut in_ifaddr, new_metric: u32);
    pub fn rt_add_uncached_list(rt: *mut rtable);
    pub fn rt_del_uncached_list(rt: *mut rtable);
    pub fn fib_dump_info_fnhe(skb: *mut sk_buff, cb: *mut netlink_callback, table_id: u32, fi: *mut fib_info, fa_index: *mut i32, fa_start: i32, flags: u32) -> i32;
    pub fn fnhe_update_pmtu(fnhe: *mut fib_nh_exception, new_: u32, orig: u32);
}

#[inline]
pub unsafe fn ip_rt_put(rt: *mut rtable) { dst_release(&mut (*rt).dst); }

extern "C" { pub static ip_tos2prio: [u8; 16]; }

#[inline]
pub unsafe fn rt_tos2priority(tos: u8) -> i8 { ip_tos2prio[(IPTOS_TOS(tos) >> 1) as usize] as i8 }

#[inline]
pub unsafe fn inet_iif(skb: *const sk_buff) -> i32 {
    let rt = skb_rtable(skb); if !rt.is_null() && (*rt).rt_iif != 0 { (*rt).rt_iif } else { (*skb).skb_iif }
}

#[inline]
pub unsafe fn ip4_dst_hoplimit(dst: *const dst_entry) -> i32 {
    let mut hoplimit = dst_metric_raw(dst, RTAX_HOPLIMIT);
    if hoplimit == 0 { rcu_read_lock(); let net = dst_dev_net_rcu(dst); hoplimit = read_once((*net).ipv4.sysctl_ip_default_ttl); rcu_read_unlock(); }
    hoplimit
}

#[inline]
pub unsafe fn ip_neigh_gw4(dev: *mut net_device, daddr: __be32) -> *mut neighbour {
    let mut neigh = __ipv4_neigh_lookup_noref(dev, daddr as u32);
    if neigh.is_null() { neigh = __neigh_create(&mut arp_tbl, &daddr, dev, false); }
    neigh
}

#[inline]
pub unsafe fn ip_neigh_for_gw(rt: *mut rtable, skb: *mut sk_buff, is_v6gw: *mut bool) -> *mut neighbour {
    let dev = (*rt).dst.dev;
    if (*rt).rt_gw_family == AF_INET { ip_neigh_gw4(dev, (*rt).gateway.rt_gw4) }
    else if (*rt).rt_gw_family == AF_INET6 { *is_v6gw = true; ip_neigh_gw6(dev, &(*rt).gateway.rt_gw6) }
    else { ip_neigh_gw4(dev, (*ip_hdr(skb)).daddr) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
