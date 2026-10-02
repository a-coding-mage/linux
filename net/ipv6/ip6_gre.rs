// SPDX-License-Identifier: GPL-2.0-or-later
// New reconstruction from retained ip6_gre.c; not an exact historical recovery.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ip6_gre_generated.rs"));
}
use bindings::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, null, null_mut, write_bytes};
use kernel::ffi::{c_int, c_void, c_char};

#[link_section = ".data..read_mostly"]
static mut ip6gre_net_id: u32 = 0;
#[inline] fn neg(errno: u32) -> c_int { -(errno as c_int) }
#[inline] const fn be16(value: u32) -> u16 { (value as u16).to_be() }
#[inline] const fn be32(value: u32) -> u32 { value.to_be() }
#[inline] fn HASH_KEY(key: u32) -> usize { ((key ^ (key >> 4)) & 31) as usize }
type TunnelFlags = [kernel::ffi::c_ulong; RUST_IP6GRE_FLAG_WORDS as usize];
#[inline]
unsafe fn tunnel_priv(dev: *const net_device) -> *mut ip6_tnl {
    rust_ip6gre_netdev_priv(dev).cast()
}
#[inline]
unsafe fn gre_net(net: *const net) -> *mut ip6gre_net {
    rust_ip6gre_net_generic(net, ip6gre_net_id).cast()
}

unsafe fn ip6gre_tunnel_match(t: *mut ip6_tnl, dev_type: c_int, link: c_int,
    cand_score: *mut c_int, ret: *mut *mut ip6_tnl) -> bool {
    let actual = (*(*t).dev).type_ as c_int;
    if actual != ARPHRD_IP6GRE as c_int && actual != dev_type { return false; }
    let mut score = 0;
    if (*t).parms.link != link { score |= 1; }
    if actual != dev_type { score |= 2; }
    if score == 0 { *ret = t; return true; }
    if score < *cand_score { *ret = t; *cand_score = score; }
    false
}

unsafe fn ip6gre_tunnel_lookup(dev: *mut net_device, remote: *const in6_addr,
    local: *const in6_addr, key: u32, gre_proto: u16) -> *mut ip6_tnl {
    let ign = gre_net(rust_ip6gre_dev_net(dev));
    let h0 = rust_ip6gre_hash_addr(remote) as usize;
    let h1 = HASH_KEY(key);
    let link = (*dev).ifindex;
    let dev_type = if gre_proto == be16(ETH_P_TEB) || gre_proto == be16(ETH_P_ERSPAN)
        || gre_proto == be16(ETH_P_ERSPAN2) { ARPHRD_ETHER } else { ARPHRD_IP6GRE } as c_int;
    let mut cand = null_mut();
    let mut cand_score = 4;
    // Each table keeps the original selection priority; a less-specific exact
    // device/link match still supersedes an earlier imperfect candidate.
    for table in (0..4usize).rev() {
        let hash = if table >= 2 { h0 ^ h1 } else { h1 };
        let mut t = rust_ip6gre_rcu_read(addr_of!((*ign).tunnels).cast::<*mut ip6_tnl>().add(table * 32).add(hash));
        while !t.is_null() {
            let p = addr_of!((*t).parms);
            let addr_ok = match table {
                3 => rust_ip6gre_addr_equal(local, addr_of!((*p).laddr))
                    && rust_ip6gre_addr_equal(remote, addr_of!((*p).raddr)),
                2 => rust_ip6gre_addr_equal(remote, addr_of!((*p).raddr)),
                1 => rust_ip6gre_addr_equal(local, addr_of!((*p).laddr))
                    || (rust_ip6gre_addr_equal(local, addr_of!((*p).raddr))
                        && rust_ip6gre_addr_multicast(local)),
                _ => true,
            };
            if addr_ok && key == (*p).i_key && (*(*t).dev).flags & IFF_UP != 0
                && ip6gre_tunnel_match(t, dev_type, link, &mut cand_score, &mut cand) {
                return cand;
            }
            t = rust_ip6gre_rcu_read(addr_of!((*t).next));
        }
    }
    if !cand.is_null() { return cand; }
    let t = if gre_proto == be16(ETH_P_ERSPAN) || gre_proto == be16(ETH_P_ERSPAN2) {
        rust_ip6gre_rcu_read(addr_of!((*ign).collect_md_tun_erspan))
    } else { rust_ip6gre_rcu_read(addr_of!((*ign).collect_md_tun)) };
    if !t.is_null() && (*(*t).dev).flags & IFF_UP != 0 { return t; }
    let ndev = rust_ip6gre_read_dev(addr_of!((*ign).fb_tunnel_dev));
    if !ndev.is_null() && (*ndev).flags & IFF_UP != 0 { return tunnel_priv(ndev); }
    null_mut()
}

unsafe fn __ip6gre_bucket(ign: *mut ip6gre_net, p: *const __ip6_tnl_parm) -> *mut *mut ip6_tnl {
    let mut h = HASH_KEY((*p).i_key);
    let mut prio = 0;
    if !rust_ip6gre_addr_any(addr_of!((*p).laddr)) { prio |= 1; }
    if !rust_ip6gre_addr_any(addr_of!((*p).raddr))
        && !rust_ip6gre_addr_multicast(addr_of!((*p).raddr)) {
        prio |= 2;
        h ^= rust_ip6gre_hash_addr(addr_of!((*p).raddr)) as usize;
    }
    addr_of_mut!((*ign).tunnels).cast::<*mut ip6_tnl>().add(prio * 32).add(h)
}
unsafe fn ip6gre_bucket(ign: *mut ip6gre_net, t: *const ip6_tnl) -> *mut *mut ip6_tnl {
    __ip6gre_bucket(ign, addr_of!((*t).parms))
}
unsafe fn ip6gre_tunnel_link_md(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    if (*t).parms.collect_md { rust_ip6gre_rcu_assign(addr_of_mut!((*ign).collect_md_tun), t); }
}
unsafe fn ip6erspan_tunnel_link_md(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    if (*t).parms.collect_md { rust_ip6gre_rcu_assign(addr_of_mut!((*ign).collect_md_tun_erspan), t); }
}
unsafe fn ip6gre_tunnel_unlink_md(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    if (*t).parms.collect_md { rust_ip6gre_rcu_assign(addr_of_mut!((*ign).collect_md_tun), null_mut()); }
}
unsafe fn ip6erspan_tunnel_unlink_md(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    if (*t).parms.collect_md { rust_ip6gre_rcu_assign(addr_of_mut!((*ign).collect_md_tun_erspan), null_mut()); }
}
unsafe fn ip6gre_tunnel_link(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    let tp = ip6gre_bucket(ign, t);
    rust_ip6gre_rcu_assign(addr_of_mut!((*t).next), rust_ip6gre_rtnl_read(tp));
    rust_ip6gre_rcu_assign(tp, t);
}
unsafe fn ip6gre_tunnel_unlink(ign: *mut ip6gre_net, t: *mut ip6_tnl) {
    let mut tp = ip6gre_bucket(ign, t);
    loop {
        let iter = rust_ip6gre_rtnl_read(tp);
        if iter.is_null() { break; }
        if t == iter { rust_ip6gre_rcu_assign(tp, (*t).next); break; }
        tp = addr_of_mut!((*iter).next);
    }
}
unsafe fn ip6gre_tunnel_find(net: *mut net, parms: *const __ip6_tnl_parm,
    type_: c_int) -> *mut ip6_tnl {
    let mut tp = __ip6gre_bucket(gre_net(net), parms);
    loop {
        let t = rust_ip6gre_rtnl_read(tp);
        if t.is_null() { return t; }
        if rust_ip6gre_addr_equal(addr_of!((*parms).laddr), addr_of!((*t).parms.laddr))
            && rust_ip6gre_addr_equal(addr_of!((*parms).raddr), addr_of!((*t).parms.raddr))
            && (*parms).i_key == (*t).parms.i_key && (*parms).link == (*t).parms.link
            && type_ == (*(*t).dev).type_ as c_int { return t; }
        tp = addr_of_mut!((*t).next);
    }
}
unsafe fn ip6gre_tunnel_locate(net: *mut net, parms: *const __ip6_tnl_parm,
    create: c_int) -> *mut ip6_tnl {
    let t = ip6gre_tunnel_find(net, parms, ARPHRD_IP6GRE as c_int);
    if !t.is_null() && create != 0 { return null_mut(); }
    if !t.is_null() || create == 0 { return t; }
    let mut name = [0 as c_char; IFNAMSIZ as usize];
    if (*parms).name[0] != 0 {
        if !dev_valid_name((*parms).name.as_ptr()) { return null_mut(); }
        rust_ip6gre_strscpy(name.as_mut_ptr(), (*parms).name.as_ptr(), name.len());
    } else {
        rust_ip6gre_strscpy(name.as_mut_ptr(), c"ip6gre%d".as_ptr().cast(), name.len());
    }
    let dev = rust_ip6gre_alloc_netdev(name.as_ptr(), Some(ip6gre_tunnel_setup));
    if dev.is_null() { return null_mut(); }
    rust_ip6gre_dev_net_set(dev, net);
    let nt = tunnel_priv(dev);
    (*nt).parms = *parms;
    (*dev).rtnl_link_ops = addr_of_mut!(ip6gre_link_ops);
    (*nt).dev = dev;
    (*nt).net = rust_ip6gre_dev_net(dev);
    if register_netdevice(dev) < 0 { free_netdev(dev); return null_mut(); }
    ip6gre_tnl_link_config(nt, 1);
    ip6gre_tunnel_link(gre_net(net), nt);
    nt
}
unsafe extern "C" fn ip6erspan_tunnel_uninit(dev: *mut net_device) {
    let t = tunnel_priv(dev);
    let ign = gre_net((*t).net);
    ip6erspan_tunnel_unlink_md(ign, t);
    ip6gre_tunnel_unlink(ign, t);
    rust_ip6gre_dst_cache_reset(addr_of_mut!((*t).dst_cache));
    rust_ip6gre_netdev_put(dev, addr_of_mut!((*t).dev_tracker));
}
unsafe extern "C" fn ip6gre_tunnel_uninit(dev: *mut net_device) {
    let t = tunnel_priv(dev);
    let ign = gre_net((*t).net);
    ip6gre_tunnel_unlink_md(ign, t);
    ip6gre_tunnel_unlink(ign, t);
    if (*ign).fb_tunnel_dev == dev { rust_ip6gre_write_dev(addr_of_mut!((*ign).fb_tunnel_dev), null_mut()); }
    rust_ip6gre_dst_cache_reset(addr_of_mut!((*t).dst_cache));
    rust_ip6gre_netdev_put(dev, addr_of_mut!((*t).dev_tracker));
}

unsafe extern "C" fn ip6gre_err(skb: *mut sk_buff, _opt: *mut inet6_skb_parm,
    type_: u8, code: u8, offset: c_int, info: u32) -> c_int {
    let dev = rust_ip6gre_skb_dev(skb);
    let net = rust_ip6gre_dev_net(dev);
    let mut tpi: tnl_ptk_info = zeroed();
    if gre_parse_header(skb, &mut tpi, null_mut(), be16(ETH_P_IPV6), offset) < 0 { return neg(EINVAL); }
    let ipv6h = (*skb).data.cast::<ipv6hdr>();
    let t = ip6gre_tunnel_lookup(dev, rust_ip6gre_daddr(ipv6h), rust_ip6gre_saddr(ipv6h), tpi.key, tpi.proto);
    if t.is_null() { return neg(ENOENT); }
    match type_ as u32 {
        ICMPV6_DEST_UNREACH => {
            rust_ip6gre_debug_error(0, (*t).parms.name.as_ptr());
            if code as u32 == ICMPV6_PORT_UNREACH { return 0; }
        }
        ICMPV6_TIME_EXCEED => {
            if code as u32 != ICMPV6_EXC_HOPLIMIT { return 0; }
            rust_ip6gre_debug_error(1, (*t).parms.name.as_ptr());
        }
        ICMPV6_PARAMPROB => {
            let teli = if code as u32 == ICMPV6_HDR_FIELD { ip6_tnl_parse_tlv_enc_lim(skb, (*skb).data) as u32 } else { 0 };
            if teli != 0 && teli == u32::from_be(info).wrapping_sub(2) {
                let tel = (*skb).data.add(teli as usize).cast::<ipv6_tlv_tnl_enc_lim>();
                if (*tel).encap_limit == 0 { rust_ip6gre_debug_error(2, (*t).parms.name.as_ptr()); }
            } else { rust_ip6gre_debug_error(3, (*t).parms.name.as_ptr()); }
            return 0;
        }
        ICMPV6_PKT_TOOBIG => { ip6_update_pmtu(skb, net, info, 0, 0, rust_ip6gre_sock_net_uid(net)); return 0; }
        NDISC_REDIRECT => { ip6_redirect(skb, net, (*dev).ifindex, 0, rust_ip6gre_sock_net_uid(net)); return 0; }
        _ => {}
    }
    // Linux time_before compares the signed difference of wrapping ulong ticks.
    let now = rust_ip6gre_jiffies();
    let until = rust_ip6gre_read_ulong(addr_of!((*t).err_time)).wrapping_add(IP6TUNNEL_ERR_TIMEO as _);
    let count = if (now.wrapping_sub(until) as kernel::ffi::c_long) < 0 {
        rust_ip6gre_read_int(addr_of!((*t).err_count)).wrapping_add(1)
    } else { 1 };
    rust_ip6gre_write_int(addr_of_mut!((*t).err_count), count);
    rust_ip6gre_write_ulong(addr_of_mut!((*t).err_time), rust_ip6gre_jiffies());
    0
}

unsafe fn ip6gre_rcv(skb: *mut sk_buff, tpi: *const tnl_ptk_info) -> c_int {
    let ipv6h = rust_ip6gre_ipv6_hdr(skb);
    let tunnel = ip6gre_tunnel_lookup(rust_ip6gre_skb_dev(skb), rust_ip6gre_saddr(ipv6h),
        rust_ip6gre_daddr(ipv6h), (*tpi).key, (*tpi).proto);
    if tunnel.is_null() { return PACKET_REJECT as c_int; }
    let mut tun_dst = null_mut();
    if (*tunnel).parms.collect_md {
        let mut flags: TunnelFlags = zeroed();
        rust_ip6gre_flags_copy(flags.as_mut_ptr(), (*tpi).flags.as_ptr());
        tun_dst = rust_ip6gre_rx_dst(skb, flags.as_ptr(), rust_ip6gre_key_to_id((*tpi).key), 0);
        if tun_dst.is_null() { return PACKET_REJECT as c_int; }
    }
    ip6_tnl_rcv(tunnel, skb, tpi, tun_dst, rust_ip6gre_log_ecn_error());
    PACKET_RCVD as c_int
}

unsafe fn ip6erspan_rcv(skb: *mut sk_buff, tpi: *mut tnl_ptk_info, gre_hdr_len: c_int) -> c_int {
    if !rust_ip6gre_may_pull(skb, size_of::<erspan_base_hdr>() as u32) { return PACKET_REJECT as c_int; }
    let ipv6h = rust_ip6gre_ipv6_hdr(skb);
    let ver = rust_ip6gre_erspan_ver((*skb).data.cast());
    let tunnel = ip6gre_tunnel_lookup(rust_ip6gre_skb_dev(skb), rust_ip6gre_saddr(ipv6h),
        rust_ip6gre_daddr(ipv6h), (*tpi).key, (*tpi).proto);
    if tunnel.is_null() { return PACKET_REJECT as c_int; }
    let len = rust_ip6gre_erspan_hlen(ver as c_int);
    if !rust_ip6gre_may_pull(skb, len as u32)
        || __iptunnel_pull_header(skb, len, be16(ETH_P_TEB), false, false) < 0 {
        return PACKET_REJECT as c_int;
    }
    let mut tun_dst = null_mut();
    if (*tunnel).parms.collect_md {
        let mut flags: TunnelFlags = zeroed();
        rust_ip6gre_set_bit(IP_TUNNEL_KEY_BIT, (*tpi).flags.as_mut_ptr());
        rust_ip6gre_flags_copy(flags.as_mut_ptr(), (*tpi).flags.as_ptr());
        tun_dst = rust_ip6gre_rx_dst(skb, flags.as_ptr(), rust_ip6gre_key_to_id((*tpi).key), size_of::<erspan_metadata>() as c_int);
        if tun_dst.is_null() { return PACKET_REJECT as c_int; }
        let info = rust_ip6gre_md_info(tun_dst);
        (*info).options_len = size_of::<erspan_metadata>() as u8;
        // Pulling may have unshared/reallocated skb; reconstruct the pointer now.
        let gh = rust_ip6gre_network_header(skb).add(rust_ip6gre_network_header_len(skb) as usize);
        let pkt_md = gh.offset(gre_hdr_len as isize).add(size_of::<erspan_base_hdr>());
        let md = rust_ip6gre_info_opts(info).cast::<erspan_metadata>();
        (*md).version = ver as c_int;
        copy_nonoverlapping(pkt_md, rust_ip6gre_md2(md).cast::<u8>(),
            if ver == 1 { ERSPAN_V1_MDSIZE as usize } else { ERSPAN_V2_MDSIZE as usize });
        rust_ip6gre_set_bit(IP_TUNNEL_ERSPAN_OPT_BIT, (*info).key.tun_flags.as_mut_ptr());
    }
    ip6_tnl_rcv(tunnel, skb, tpi, tun_dst, rust_ip6gre_log_ecn_error());
    PACKET_RCVD as c_int
}

unsafe extern "C" fn gre_rcv(skb: *mut sk_buff) -> c_int {
    let mut tpi: tnl_ptk_info = zeroed();
    let mut csum_err = false;
    let hdr_len = gre_parse_header(skb, &mut tpi, &mut csum_err, be16(ETH_P_IPV6), 0);
    if hdr_len >= 0 && rust_ip6gre_pull_header(skb, hdr_len, tpi.proto) == 0 {
        let received = if tpi.proto == be16(ETH_P_ERSPAN) || tpi.proto == be16(ETH_P_ERSPAN2) {
            ip6erspan_rcv(skb, &mut tpi, hdr_len)
        } else { ip6gre_rcv(skb, &tpi) };
        if received == PACKET_RCVD as c_int { return 0; }
        rust_ip6gre_icmp6_send(skb, ICMPV6_DEST_UNREACH as u8, ICMPV6_PORT_UNREACH as u8, 0);
    }
    rust_ip6gre_rx_dropped(rust_ip6gre_skb_dev(skb));
    rust_ip6gre_kfree_skb(skb);
    0
}

unsafe fn gre_handle_offloads(skb: *mut sk_buff, csum: bool) -> c_int {
    iptunnel_handle_offloads(skb, if csum { SKB_GSO_GRE_CSUM } else { SKB_GSO_GRE } as c_int)
}
unsafe fn prepare_ip6gre_xmit_ipv4(skb: *mut sk_buff, dev: *mut net_device,
    fl6: *mut flowi6, dsfield: *mut u8, encap_limit: *mut c_int) {
    let iph = rust_ip6gre_ip_hdr(skb);
    let t = tunnel_priv(dev);
    if (*t).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { *encap_limit = (*t).parms.encap_limit as c_int; }
    *fl6 = (*t).fl.u.ip6;
    *dsfield = if (*t).parms.flags & IP6_TNL_F_USE_ORIG_TCLASS != 0 { rust_ip6gre_ipv4_dsfield(iph) }
        else { rust_ip6gre_tclass((*t).parms.flowinfo) };
    rust_ip6gre_flow_mark(fl6, if (*t).parms.flags & IP6_TNL_F_USE_ORIG_FWMARK != 0 { rust_ip6gre_skb_mark(skb) } else { (*t).parms.fwmark });
    rust_ip6gre_flow_uid(fl6, rust_ip6gre_sock_net_uid(rust_ip6gre_dev_net(dev)));
}
unsafe fn prepare_ip6gre_xmit_ipv6(skb: *mut sk_buff, dev: *mut net_device,
    fl6: *mut flowi6, dsfield: *mut u8, encap_limit: *mut c_int) -> c_int {
    let t = tunnel_priv(dev);
    let offset = ip6_tnl_parse_tlv_enc_lim(skb, rust_ip6gre_network_header(skb));
    let ipv6h = rust_ip6gre_ipv6_hdr(skb);
    if offset > 0 {
        let tel = rust_ip6gre_network_header(skb).add(offset as usize).cast::<ipv6_tlv_tnl_enc_lim>();
        if (*tel).encap_limit == 0 {
            icmpv6_ndo_send(skb, ICMPV6_PARAMPROB as u8, ICMPV6_HDR_FIELD as u8, offset as u32 + 2);
            return -1;
        }
        *encap_limit = (*tel).encap_limit as c_int - 1;
    } else if (*t).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { *encap_limit = (*t).parms.encap_limit as c_int; }
    *fl6 = (*t).fl.u.ip6;
    *dsfield = if (*t).parms.flags & IP6_TNL_F_USE_ORIG_TCLASS != 0 { rust_ip6gre_ipv6_dsfield(ipv6h) }
        else { rust_ip6gre_tclass((*t).parms.flowinfo) };
    if (*t).parms.flags & IP6_TNL_F_USE_ORIG_FLOWLABEL != 0 { (*fl6).flowlabel |= rust_ip6gre_flowlabel(ipv6h); }
    rust_ip6gre_flow_mark(fl6, if (*t).parms.flags & IP6_TNL_F_USE_ORIG_FWMARK != 0 { rust_ip6gre_skb_mark(skb) } else { (*t).parms.fwmark });
    rust_ip6gre_flow_uid(fl6, rust_ip6gre_sock_net_uid(rust_ip6gre_dev_net(dev)));
    0
}
unsafe fn prepare_ip6gre_xmit_other(skb: *mut sk_buff, dev: *mut net_device,
    fl6: *mut flowi6, dsfield: *mut u8, encap_limit: *mut c_int) -> c_int {
    let t = tunnel_priv(dev);
    if (*t).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { *encap_limit = (*t).parms.encap_limit as c_int; }
    *fl6 = (*t).fl.u.ip6;
    *dsfield = if (*t).parms.flags & IP6_TNL_F_USE_ORIG_TCLASS != 0 { 0 } else { rust_ip6gre_tclass((*t).parms.flowinfo) };
    rust_ip6gre_flow_mark(fl6, if (*t).parms.flags & IP6_TNL_F_USE_ORIG_FWMARK != 0 { rust_ip6gre_skb_mark(skb) } else { (*t).parms.fwmark });
    rust_ip6gre_flow_uid(fl6, rust_ip6gre_sock_net_uid(rust_ip6gre_dev_net(dev)));
    0
}
unsafe fn skb_tunnel_info_txcheck(skb: *mut sk_buff) -> *mut ip_tunnel_info {
    let tun_info = rust_ip6gre_skb_tunnel_info(skb);
    if tun_info.is_null() || (*tun_info).mode as u32 & IP_TUNNEL_INFO_TX == 0 {
        return neg(EINVAL) as isize as *mut ip_tunnel_info;
    }
    tun_info
}

unsafe fn __gre6_xmit(skb: *mut sk_buff, dev: *mut net_device, mut dsfield: u8,
    fl6: *mut flowi6, encap_limit: c_int, pmtu: *mut u32, proto: u16) -> c_int {
    let tunnel = tunnel_priv(dev);
    let mut flags: TunnelFlags = zeroed();
    if (*dev).type_ as u32 == ARPHRD_ETHER { rust_ip6gre_ipcb_clear_flags(skb); }
    (*fl6).daddr = if !(*dev).header_ops.is_null() && (*dev).type_ as u32 == ARPHRD_IP6GRE {
        *rust_ip6gre_daddr((*skb).data.cast())
    } else { (*tunnel).parms.raddr };
    let protocol = if (*dev).type_ as u32 == ARPHRD_ETHER { be16(ETH_P_TEB) } else { proto };
    if (*tunnel).parms.collect_md {
        let tun_info = skb_tunnel_info_txcheck(skb);
        if rust_ip6gre_is_err(tun_info.cast()) || rust_ip6gre_info_af(tun_info) as u32 != AF_INET6 { return neg(EINVAL); }
        let key = addr_of!((*tun_info).key);
        *fl6 = zeroed();
        rust_ip6gre_flow_proto(fl6, IPPROTO_GRE as u8);
        (*fl6).daddr = (*key).u.ipv6.dst;
        (*fl6).flowlabel = (*key).label;
        rust_ip6gre_flow_uid(fl6, rust_ip6gre_sock_net_uid(rust_ip6gre_dev_net(dev)));
        rust_ip6gre_flow_key(fl6, rust_ip6gre_id_to_key((*key).tun_id));
        dsfield = (*key).tos;
        rust_ip6gre_set_bit(IP_TUNNEL_CSUM_BIT, flags.as_mut_ptr());
        rust_ip6gre_set_bit(IP_TUNNEL_KEY_BIT, flags.as_mut_ptr());
        rust_ip6gre_set_bit(IP_TUNNEL_SEQ_BIT, flags.as_mut_ptr());
        rust_ip6gre_flags_and(flags.as_mut_ptr(), flags.as_ptr(), (*key).tun_flags.as_ptr());
        let tun_hlen = rust_ip6gre_gre_hlen(flags.as_ptr());
        let headroom = if (*dev).needed_headroom != 0 { (*dev).needed_headroom as u32 }
            else { tun_hlen.wrapping_add((*tunnel).encap_hlen) as u32 };
        if rust_ip6gre_cow_head(skb, headroom) != 0 { return neg(ENOMEM); }
        let seq = if rust_ip6gre_test_bit(IP_TUNNEL_SEQ_BIT, flags.as_ptr()) {
            be32(rust_ip6gre_atomic_fetch_inc(addr_of_mut!((*tunnel).o_seqno)) as u32)
        } else { 0 };
        rust_ip6gre_gre_build(skb, tun_hlen, flags.as_ptr(), protocol, rust_ip6gre_id_to_key((*tun_info).key.tun_id), seq);
    } else {
        let headroom = if (*dev).needed_headroom != 0 { (*dev).needed_headroom as u32 } else { (*tunnel).hlen as u32 };
        if rust_ip6gre_cow_head(skb, headroom) != 0 { return neg(ENOMEM); }
        rust_ip6gre_flags_copy(flags.as_mut_ptr(), (*tunnel).parms.o_flags.as_ptr());
        let seq = if rust_ip6gre_test_bit(IP_TUNNEL_SEQ_BIT, flags.as_ptr()) {
            be32(rust_ip6gre_atomic_fetch_inc(addr_of_mut!((*tunnel).o_seqno)) as u32)
        } else { 0 };
        rust_ip6gre_gre_build(skb, (*tunnel).tun_hlen, flags.as_ptr(), protocol, (*tunnel).parms.o_key, seq);
    }
    ip6_tnl_xmit(skb, dev, dsfield, fl6, encap_limit, pmtu, NEXTHDR_GRE as u8)
}

unsafe fn ip6gre_xmit_ipv4(skb: *mut sk_buff, dev: *mut net_device) -> c_int {
    let t = tunnel_priv(dev);
    let mut encap_limit = -1;
    let mut fl6: flowi6 = zeroed();
    let mut dsfield = 0;
    let mut mtu = 0;
    rust_ip6gre_ipcb_clear_options(skb);
    if !(*t).parms.collect_md { prepare_ip6gre_xmit_ipv4(skb, dev, &mut fl6, &mut dsfield, &mut encap_limit); }
    if gre_handle_offloads(skb, rust_ip6gre_test_bit(IP_TUNNEL_CSUM_BIT, (*t).parms.o_flags.as_ptr())) != 0 { return -1; }
    let err = __gre6_xmit(skb, dev, dsfield, &mut fl6, encap_limit, &mut mtu, rust_ip6gre_protocol(skb));
    if err != 0 {
        if err == neg(EMSGSIZE) { icmp_ndo_send(skb, ICMP_DEST_UNREACH as c_int, ICMP_FRAG_NEEDED as c_int, be32(mtu)); }
        return -1;
    }
    0
}
unsafe fn ip6gre_xmit_ipv6(skb: *mut sk_buff, dev: *mut net_device) -> c_int {
    let t = tunnel_priv(dev);
    let ipv6h = rust_ip6gre_ipv6_hdr(skb);
    let mut encap_limit = -1;
    let mut fl6: flowi6 = zeroed();
    let mut dsfield = 0;
    let mut mtu = 0;
    if rust_ip6gre_addr_equal(addr_of!((*t).parms.raddr), rust_ip6gre_saddr(ipv6h)) { return -1; }
    if !(*t).parms.collect_md && prepare_ip6gre_xmit_ipv6(skb, dev, &mut fl6, &mut dsfield, &mut encap_limit) != 0 { return -1; }
    if gre_handle_offloads(skb, rust_ip6gre_test_bit(IP_TUNNEL_CSUM_BIT, (*t).parms.o_flags.as_ptr())) != 0 { return -1; }
    let err = __gre6_xmit(skb, dev, dsfield, &mut fl6, encap_limit, &mut mtu, rust_ip6gre_protocol(skb));
    if err != 0 {
        if err == neg(EMSGSIZE) { icmpv6_ndo_send(skb, ICMPV6_PKT_TOOBIG as u8, 0, mtu); }
        return -1;
    }
    0
}
unsafe fn ip6gre_xmit_other(skb: *mut sk_buff, dev: *mut net_device) -> c_int {
    let t = tunnel_priv(dev);
    let mut encap_limit = -1;
    let mut fl6: flowi6 = zeroed();
    let mut dsfield = 0;
    let mut mtu = 0;
    if !(*t).parms.collect_md && prepare_ip6gre_xmit_other(skb, dev, &mut fl6, &mut dsfield, &mut encap_limit) != 0 { return -1; }
    let err = gre_handle_offloads(skb, rust_ip6gre_test_bit(IP_TUNNEL_CSUM_BIT, (*t).parms.o_flags.as_ptr()));
    if err != 0 { return err; }
    __gre6_xmit(skb, dev, dsfield, &mut fl6, encap_limit, &mut mtu, rust_ip6gre_protocol(skb))
}
unsafe extern "C" fn ip6gre_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let t = tunnel_priv(dev);
    if rust_ip6gre_inet_may_pull(skb) && ip6_tnl_xmit_ctl(t, addr_of!((*t).parms.laddr), addr_of!((*t).parms.raddr)) != 0 {
        let protocol = rust_ip6gre_skb_protocol(skb, true);
        let ret = if protocol == be16(ETH_P_IP) { ip6gre_xmit_ipv4(skb, dev) }
            else if protocol == be16(ETH_P_IPV6) { ip6gre_xmit_ipv6(skb, dev) }
            else { ip6gre_xmit_other(skb, dev) };
        if ret >= 0 { return NETDEV_TX_OK; }
    }
    if !(*t).parms.collect_md || !rust_ip6gre_is_err(skb_tunnel_info_txcheck(skb).cast()) { rust_ip6gre_tx_errors(dev); }
    rust_ip6gre_tx_dropped(dev);
    rust_ip6gre_kfree_skb(skb);
    NETDEV_TX_OK
}

unsafe extern "C" fn ip6erspan_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    let mut tun_info: *mut ip_tunnel_info = null_mut();
    let t = tunnel_priv(dev);
    let dst = rust_ip6gre_skb_dst(skb);
    let mut flags: TunnelFlags = zeroed();
    let mut truncate = false;
    let mut encap_limit = -1;
    let mut dsfield = 0;
    let mut fl6: flowi6 = zeroed();
    let mut mtu = 0;
    // This labeled block preserves the single original error/free path, including
    // its special ERR_PTR metadata accounting and transmit ownership transfer.
    'tx: {
        if !rust_ip6gre_inet_may_pull(skb)
            || ip6_tnl_xmit_ctl(t, addr_of!((*t).parms.laddr), addr_of!((*t).parms.raddr)) == 0
            || gre_handle_offloads(skb, false) != 0 { break 'tx; }
        let frame_mtu = (*dev).mtu.wrapping_add((*dev).hard_header_len as u32);
        if (*skb).len > frame_mtu {
            if rust_ip6gre_trim(skb, frame_mtu) != 0 { break 'tx; }
            truncate = true;
        }
        let nhoff = rust_ip6gre_network_offset(skb);
        if rust_ip6gre_protocol(skb) == be16(ETH_P_IP)
            && u16::from_be((*rust_ip6gre_ip_hdr(skb)).tot_len) as u32 > (*skb).len.wrapping_sub(nhoff as u32) { truncate = true; }
        if rust_ip6gre_protocol(skb) == be16(ETH_P_IPV6) {
            let thoff = if rust_ip6gre_transport_header_was_set(skb) { rust_ip6gre_transport_offset(skb) }
                else { (nhoff as usize).wrapping_add(size_of::<ipv6hdr>()) as c_int };
            if u16::from_be((*rust_ip6gre_ipv6_hdr(skb)).payload_len) as u32 > (*skb).len.wrapping_sub(thoff as u32) { truncate = true; }
        }
        let headroom = if (*dev).needed_headroom != 0 { (*dev).needed_headroom as u32 } else { (*t).hlen as u32 };
        if rust_ip6gre_cow_head(skb, headroom) != 0 { break 'tx; }
        rust_ip6gre_ipcb_clear_flags(skb);
        let proto;
        if (*t).parms.collect_md {
            tun_info = skb_tunnel_info_txcheck(skb);
            if rust_ip6gre_is_err(tun_info.cast()) || rust_ip6gre_info_af(tun_info) as u32 != AF_INET6 { break 'tx; }
            let key = addr_of!((*tun_info).key);
            fl6 = zeroed();
            rust_ip6gre_flow_proto(&mut fl6, IPPROTO_GRE as u8);
            fl6.daddr = (*key).u.ipv6.dst;
            fl6.flowlabel = (*key).label;
            rust_ip6gre_flow_uid(&mut fl6, rust_ip6gre_sock_net_uid(rust_ip6gre_dev_net(dev)));
            rust_ip6gre_flow_key(&mut fl6, rust_ip6gre_id_to_key((*key).tun_id));
            dsfield = (*key).tos;
            if !rust_ip6gre_test_bit(IP_TUNNEL_ERSPAN_OPT_BIT, (*tun_info).key.tun_flags.as_ptr())
                || ((*tun_info).options_len as usize) < size_of::<erspan_metadata>() { break 'tx; }
            let md = rust_ip6gre_info_opts(tun_info).cast::<erspan_metadata>();
            let tun_id = u32::from_be(rust_ip6gre_id_to_key((*key).tun_id));
            if (*md).version == 1 {
                rust_ip6gre_erspan_build(skb, tun_id, u32::from_be(rust_ip6gre_md_index(md)), truncate);
                proto = be16(ETH_P_ERSPAN);
            } else if (*md).version == 2 {
                let md2 = rust_ip6gre_md2(md).cast::<erspan_md2>();
                rust_ip6gre_erspan_build_v2(skb, tun_id, rust_ip6gre_md2_dir(md2), rust_ip6gre_md2_hwid(md2) as u16, truncate);
                proto = be16(ETH_P_ERSPAN2);
            } else { break 'tx; }
        } else {
            if rust_ip6gre_protocol(skb) == be16(ETH_P_IP) {
                rust_ip6gre_ipcb_clear_options(skb);
                prepare_ip6gre_xmit_ipv4(skb, dev, &mut fl6, &mut dsfield, &mut encap_limit);
            } else if rust_ip6gre_protocol(skb) == be16(ETH_P_IPV6) {
                if rust_ip6gre_addr_equal(addr_of!((*t).parms.raddr), rust_ip6gre_saddr(rust_ip6gre_ipv6_hdr(skb)))
                    || prepare_ip6gre_xmit_ipv6(skb, dev, &mut fl6, &mut dsfield, &mut encap_limit) != 0 { break 'tx; }
            } else { fl6 = (*t).fl.u.ip6; }
            if (*t).parms.erspan_ver == 1 {
                rust_ip6gre_erspan_build(skb, u32::from_be((*t).parms.o_key), (*t).parms.index, truncate);
                proto = be16(ETH_P_ERSPAN);
            } else if (*t).parms.erspan_ver == 2 {
                rust_ip6gre_erspan_build_v2(skb, u32::from_be((*t).parms.o_key), (*t).parms.dir, (*t).parms.hwid, truncate);
                proto = be16(ETH_P_ERSPAN2);
            } else { break 'tx; }
            fl6.daddr = (*t).parms.raddr;
        }
        rust_ip6gre_set_bit(IP_TUNNEL_SEQ_BIT, flags.as_mut_ptr());
        rust_ip6gre_gre_build(skb, 8, flags.as_ptr(), proto, 0, be32(rust_ip6gre_atomic_fetch_inc(addr_of_mut!((*t).o_seqno)) as u32));
        if !(*t).parms.collect_md && !dst.is_null() {
            mtu = rust_ip6gre_read_uint(addr_of!((*rust_ip6gre_dst_dev(dst)).mtu));
            if rust_ip6gre_dst_mtu(dst) > mtu {
                ((*(*dst).ops).update_pmtu.unwrap_unchecked())(dst, null_mut(), skb, mtu, false);
            }
        }
        let err = ip6_tnl_xmit(skb, dev, dsfield, &mut fl6, encap_limit, &mut mtu, NEXTHDR_GRE as u8);
        if err != 0 {
            if err == neg(EMSGSIZE) {
                if rust_ip6gre_protocol(skb) == be16(ETH_P_IP) { icmp_ndo_send(skb, ICMP_DEST_UNREACH as c_int, ICMP_FRAG_NEEDED as c_int, be32(mtu)); }
                else { icmpv6_ndo_send(skb, ICMPV6_PKT_TOOBIG as u8, 0, mtu); }
            }
            break 'tx;
        }
        return NETDEV_TX_OK;
    }
    if !rust_ip6gre_is_err(tun_info.cast()) { rust_ip6gre_tx_errors(dev); }
    rust_ip6gre_tx_dropped(dev);
    rust_ip6gre_kfree_skb(skb);
    NETDEV_TX_OK
}
// Control-plane translation of ip6_gre.c, from link configuration onward.
unsafe fn ip6gre_tnl_link_config_common(t: *mut ip6_tnl) {
    let dev = (*t).dev;
    let p = addr_of_mut!((*t).parms);
    let fl6 = addr_of_mut!((*t).fl.u.ip6);
    if (*dev).type_ as u32 != ARPHRD_ETHER {
        rust_ip6gre_dev_addr_set(dev, addr_of!((*p).laddr).cast(), size_of::<in6_addr>());
        copy_nonoverlapping(addr_of!((*p).raddr).cast::<u8>(), addr_of_mut!((*dev).broadcast).cast(), size_of::<in6_addr>());
    }
    (*fl6).saddr = (*p).laddr;
    (*fl6).daddr = (*p).raddr;
    rust_ip6gre_flow_oif(fl6, (*p).link);
    (*fl6).flowlabel = 0;
    rust_ip6gre_flow_proto(fl6, IPPROTO_GRE as u8);
    rust_ip6gre_flow_key(fl6, (*p).o_key);
    if (*p).flags & IP6_TNL_F_USE_ORIG_TCLASS == 0 {
        (*fl6).flowlabel |= be32(0x0ff0_0000) & (*p).flowinfo;
    }
    if (*p).flags & IP6_TNL_F_USE_ORIG_FLOWLABEL == 0 {
        (*fl6).flowlabel |= be32(0x000f_ffff) & (*p).flowinfo;
    }
    (*p).flags &= !(IP6_TNL_F_CAP_XMIT | IP6_TNL_F_CAP_RCV | IP6_TNL_F_CAP_PER_PACKET);
    (*p).flags |= ip6_tnl_get_cap(t, addr_of!((*p).laddr), addr_of!((*p).raddr));
    if (*p).flags & IP6_TNL_F_CAP_XMIT != 0 && (*p).flags & IP6_TNL_F_CAP_RCV != 0 && (*dev).type_ as u32 != ARPHRD_ETHER {
        (*dev).flags |= IFF_POINTOPOINT;
    } else {
        (*dev).flags &= !IFF_POINTOPOINT;
    }
}

unsafe fn ip6gre_tnl_link_config_route(t: *mut ip6_tnl, set_mtu: c_int, t_hlen: c_int) {
    let p = addr_of!((*t).parms);
    let dev = (*t).dev;
    if (*p).flags & IP6_TNL_F_CAP_XMIT == 0 { return; }
    let strict = rust_ip6gre_addr_type(addr_of!((*p).raddr)) & (IPV6_ADDR_MULTICAST | IPV6_ADDR_LINKLOCAL) as c_int;
    let rt = rt6_lookup((*t).net, addr_of!((*p).raddr), addr_of!((*p).laddr), (*p).link, null(), strict);
    if rt.is_null() { return; }
    let route_dev = rust_ip6gre_dst_dev(addr_of!((*rt).dst));
    if !route_dev.is_null() {
        let headroom = rust_ip6gre_limit_headroom(((*route_dev).hard_header_len as u32).wrapping_add(t_hlen as u32));
        (*dev).needed_headroom = headroom as _;
        if set_mtu != 0 {
            let mut mtu = (*route_dev).mtu.wrapping_sub(t_hlen as u32) as c_int;
            if (*t).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { mtu = mtu.wrapping_sub(8); }
            if (*dev).type_ as u32 == ARPHRD_ETHER { mtu = mtu.wrapping_sub(ETH_HLEN as c_int); }
            if mtu < IPV6_MIN_MTU as c_int { mtu = IPV6_MIN_MTU as c_int; }
            rust_ip6gre_write_uint(addr_of_mut!((*dev).mtu), mtu as u32);
        }
    }
    rust_ip6gre_rt_put(rt);
}

unsafe fn ip6gre_calc_hlen(t: *mut ip6_tnl) -> c_int {
    (*t).tun_hlen = rust_ip6gre_gre_hlen(addr_of!((*t).parms.o_flags).cast());
    (*t).hlen = (*t).tun_hlen.wrapping_add((*t).encap_hlen);
    let t_hlen = (*t).hlen.wrapping_add(size_of::<ipv6hdr>() as c_int);
    let dev = (*t).dev;
    if !(*dev).header_ops.is_null() && (*dev).type_ as u32 == ARPHRD_IP6GRE {
        (*dev).hard_header_len = t_hlen as _;
    } else {
        (*dev).needed_headroom = (RUST_IP6GRE_LL_MAX_HEADER as c_int).wrapping_add(t_hlen) as _;
    }
    t_hlen
}

unsafe fn ip6gre_tnl_link_config(t: *mut ip6_tnl, set_mtu: c_int) {
    ip6gre_tnl_link_config_common(t);
    ip6gre_tnl_link_config_route(t, set_mtu, ip6gre_calc_hlen(t));
}

unsafe fn ip6gre_tnl_copy_tnl_parm(t: *mut ip6_tnl, p: *const __ip6_tnl_parm) {
    let out = addr_of_mut!((*t).parms);
    (*out).laddr = (*p).laddr;
    (*out).raddr = (*p).raddr;
    (*out).flags = (*p).flags;
    (*out).hop_limit = (*p).hop_limit;
    (*out).encap_limit = (*p).encap_limit;
    (*out).flowinfo = (*p).flowinfo;
    (*out).link = (*p).link;
    (*out).proto = (*p).proto;
    (*out).i_key = (*p).i_key;
    (*out).o_key = (*p).o_key;
    rust_ip6gre_flags_copy(addr_of_mut!((*out).i_flags).cast(), addr_of!((*p).i_flags).cast());
    rust_ip6gre_flags_copy(addr_of_mut!((*out).o_flags).cast(), addr_of!((*p).o_flags).cast());
    (*out).fwmark = (*p).fwmark;
    (*out).erspan_ver = (*p).erspan_ver;
    (*out).index = (*p).index;
    (*out).dir = (*p).dir;
    (*out).hwid = (*p).hwid;
    // collect_md and name are deliberately not copied by the original update.
    rust_ip6gre_dst_cache_reset(addr_of_mut!((*t).dst_cache));
}

unsafe fn ip6gre_tnl_change(t: *mut ip6_tnl, p: *const __ip6_tnl_parm, set_mtu: c_int) -> c_int {
    ip6gre_tnl_copy_tnl_parm(t, p);
    ip6gre_tnl_link_config(t, set_mtu);
    0
}

unsafe fn ip6gre_tnl_parm_from_user(p: *mut __ip6_tnl_parm, u: *const ip6_tnl_parm2) {
    (*p).laddr = (*u).laddr;
    (*p).raddr = (*u).raddr;
    (*p).flags = (*u).flags;
    (*p).hop_limit = (*u).hop_limit;
    (*p).encap_limit = (*u).encap_limit;
    (*p).flowinfo = (*u).flowinfo;
    (*p).link = (*u).link;
    (*p).i_key = (*u).i_key;
    (*p).o_key = (*u).o_key;
    rust_ip6gre_flags_from_gre(addr_of_mut!((*p).i_flags).cast(), (*u).i_flags);
    rust_ip6gre_flags_from_gre(addr_of_mut!((*p).o_flags).cast(), (*u).o_flags);
    copy_nonoverlapping(addr_of!((*u).name).cast::<c_char>(), addr_of_mut!((*p).name).cast(), IFNAMSIZ as usize);
}

unsafe fn ip6gre_tnl_parm_to_user(u: *mut ip6_tnl_parm2, p: *const __ip6_tnl_parm) {
    (*u).proto = IPPROTO_GRE as u8;
    (*u).laddr = (*p).laddr;
    (*u).raddr = (*p).raddr;
    (*u).flags = (*p).flags;
    (*u).hop_limit = (*p).hop_limit;
    (*u).encap_limit = (*p).encap_limit;
    (*u).flowinfo = (*p).flowinfo;
    (*u).link = (*p).link;
    (*u).i_key = (*p).i_key;
    (*u).o_key = (*p).o_key;
    (*u).i_flags = rust_ip6gre_flags_to_gre(addr_of!((*p).i_flags).cast());
    (*u).o_flags = rust_ip6gre_flags_to_gre(addr_of!((*p).o_flags).cast());
    copy_nonoverlapping(addr_of!((*p).name).cast::<c_char>(), addr_of_mut!((*u).name).cast(), IFNAMSIZ as usize);
}

unsafe extern "C" fn ip6gre_tunnel_siocdevprivate(mut dev: *mut net_device, _ifr: *mut ifreq, data: *mut c_void, cmd: c_int) -> c_int {
    let mut p: ip6_tnl_parm2 = zeroed();
    let mut p1: __ip6_tnl_parm = zeroed();
    let mut t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let net = (*t).net;
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    match cmd as u32 {
        SIOCGETTUNNEL => {
            if dev == (*ign).fb_tunnel_dev {
                if rust_ip6gre_copy_from_user(addr_of_mut!(p).cast(), data, size_of::<ip6_tnl_parm2>() as _) != 0 { return neg(EFAULT); }
                ip6gre_tnl_parm_from_user(addr_of_mut!(p1), addr_of!(p));
                t = ip6gre_tunnel_locate(net, addr_of_mut!(p1), 0);
                if t.is_null() { t = rust_ip6gre_netdev_priv(dev).cast(); }
            }
            p = zeroed();
            ip6gre_tnl_parm_to_user(addr_of_mut!(p), addr_of!((*t).parms));
            if rust_ip6gre_copy_to_user(data, addr_of!(p).cast(), size_of::<ip6_tnl_parm2>() as _) != 0 { return neg(EFAULT); }
            0
        }
        SIOCADDTUNNEL | SIOCCHGTUNNEL => {
            if !ns_capable((*net).user_ns, CAP_NET_ADMIN as c_int) { return neg(EPERM); }
            if rust_ip6gre_copy_from_user(addr_of_mut!(p).cast(), data, size_of::<ip6_tnl_parm2>() as _) != 0 { return neg(EFAULT); }
            if (p.i_flags | p.o_flags) & be16(0x0007 | 0x4000) != 0 { return neg(EINVAL); }
            if p.i_flags & be16(0x2000) == 0 { p.i_key = 0; }
            if p.o_flags & be16(0x2000) == 0 { p.o_key = 0; }
            ip6gre_tnl_parm_from_user(addr_of_mut!(p1), addr_of!(p));
            t = ip6gre_tunnel_locate(net, addr_of_mut!(p1), (cmd as u32 == SIOCADDTUNNEL) as c_int);
            if dev != (*ign).fb_tunnel_dev && cmd as u32 == SIOCCHGTUNNEL {
                if !t.is_null() {
                    if (*t).dev != dev { return neg(EEXIST); }
                } else {
                    t = rust_ip6gre_netdev_priv(dev).cast();
                    ip6gre_tunnel_unlink(ign, t);
                    synchronize_net();
                    ip6gre_tnl_change(t, addr_of!(p1), 1);
                    ip6gre_tunnel_link(ign, t);
                    netdev_state_change(dev);
                }
            }
            if t.is_null() { return if cmd as u32 == SIOCADDTUNNEL { neg(ENOBUFS) } else { neg(ENOENT) }; }
            p = zeroed();
            ip6gre_tnl_parm_to_user(addr_of_mut!(p), addr_of!((*t).parms));
            if rust_ip6gre_copy_to_user(data, addr_of!(p).cast(), size_of::<ip6_tnl_parm2>() as _) != 0 { return neg(EFAULT); }
            0
        }
        SIOCDELTUNNEL => {
            if !ns_capable((*net).user_ns, CAP_NET_ADMIN as c_int) { return neg(EPERM); }
            if dev == (*ign).fb_tunnel_dev {
                if rust_ip6gre_copy_from_user(addr_of_mut!(p).cast(), data, size_of::<ip6_tnl_parm2>() as _) != 0 { return neg(EFAULT); }
                ip6gre_tnl_parm_from_user(addr_of_mut!(p1), addr_of!(p));
                t = ip6gre_tunnel_locate(net, addr_of_mut!(p1), 0);
                if t.is_null() { return neg(ENOENT); }
                if t == rust_ip6gre_netdev_priv((*ign).fb_tunnel_dev).cast() { return neg(EPERM); }
                dev = (*t).dev;
            }
            rust_ip6gre_unregister_netdevice(dev);
            0
        }
        _ => neg(EINVAL),
    }
}

unsafe extern "C" fn ip6gre_header(skb: *mut sk_buff, dev: *mut net_device, type_: u16, daddr: *const c_void, saddr: *const c_void, _len: u32) -> c_int {
    let t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let needed = (*t).hlen.wrapping_add(size_of::<ipv6hdr>() as c_int);
    if rust_ip6gre_headroom(skb) < needed as u32 && pskb_expand_head(skb,
        rust_ip6gre_hh_data_align(needed.wrapping_sub(rust_ip6gre_headroom(skb) as c_int)), 0, RUST_IP6GRE_GFP_ATOMIC) != 0 {
        return needed.wrapping_neg();
    }
    let ipv6h = skb_push(skb, needed as u32).cast::<ipv6hdr>();
    rust_ip6gre_flow_hdr(ipv6h, 0, rust_ip6gre_make_flowlabel(rust_ip6gre_dev_net(dev), skb, (*t).fl.u.ip6.flowlabel, addr_of_mut!((*t).fl.u.ip6)));
    (*ipv6h).hop_limit = (*t).parms.hop_limit;
    (*ipv6h).nexthdr = NEXTHDR_GRE as u8;
    *rust_ip6gre_saddr(ipv6h) = (*t).parms.laddr;
    *rust_ip6gre_daddr(ipv6h) = (*t).parms.raddr;
    let p = ipv6h.add(1).cast::<u16>();
    *p = rust_ip6gre_flags_to_be16(addr_of!((*t).parms.o_flags).cast());
    *p.add(1) = be16(type_ as u32);
    if !saddr.is_null() { copy_nonoverlapping(saddr.cast::<u8>(), rust_ip6gre_saddr(ipv6h).cast(), size_of::<in6_addr>()); }
    if !daddr.is_null() { copy_nonoverlapping(daddr.cast::<u8>(), rust_ip6gre_daddr(ipv6h).cast(), size_of::<in6_addr>()); }
    if !rust_ip6gre_addr_any(rust_ip6gre_daddr(ipv6h)) { (*t).hlen } else { (*t).hlen.wrapping_neg() }
}

const fn ip6gre_make_header_ops() -> header_ops {
    let mut ops: header_ops = unsafe { zeroed() };
    ops.create = Some(ip6gre_header);
    ops
}
static ip6gre_header_ops: header_ops = ip6gre_make_header_ops();

const fn ip6gre_make_netdev_ops() -> net_device_ops {
    let mut ops: net_device_ops = unsafe { zeroed() };
    ops.ndo_init = Some(ip6gre_tunnel_init);
    ops.ndo_uninit = Some(ip6gre_tunnel_uninit);
    ops.ndo_start_xmit = Some(ip6gre_tunnel_xmit);
    ops.ndo_siocdevprivate = Some(ip6gre_tunnel_siocdevprivate);
    ops.ndo_change_mtu = Some(ip6_tnl_change_mtu);
    ops.ndo_get_iflink = Some(ip6_tnl_get_iflink);
    ops
}
static ip6gre_netdev_ops: net_device_ops = ip6gre_make_netdev_ops();

unsafe extern "C" fn ip6gre_dev_free(dev: *mut net_device) {
    let t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    gro_cells_destroy(addr_of_mut!((*t).gro_cells));
    dst_cache_destroy(addr_of_mut!((*t).dst_cache));
}

unsafe extern "C" fn ip6gre_tunnel_setup(dev: *mut net_device) {
    (*dev).netdev_ops = addr_of!(ip6gre_netdev_ops);
    (*dev).needs_free_netdev = true;
    (*dev).priv_destructor = Some(ip6gre_dev_free);
    rust_ip6gre_set_pcpu_stat_type(dev, NETDEV_PCPU_STAT_TSTATS);
    (*dev).type_ = ARPHRD_IP6GRE as u16;
    (*dev).flags |= IFF_NOARP;
    (*dev).addr_len = size_of::<in6_addr>() as u8;
    rust_ip6gre_keep_dst(dev);
    (*dev).addr_assign_type = NET_ADDR_RANDOM as u8;
    rust_ip6gre_random_addr(addr_of_mut!((*dev).perm_addr).cast());
}

unsafe fn ip6gre_tnl_init_features(dev: *mut net_device) {
    let nt = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    (*dev).features |= rust_ip6gre_gre6_features();
    (*dev).hw_features |= rust_ip6gre_gre6_features();
    rust_ip6gre_set_lltx(dev, true);
    if rust_ip6gre_test_bit(IP_TUNNEL_SEQ_BIT, addr_of!((*nt).parms.o_flags).cast()) { return; }
    if rust_ip6gre_test_bit(IP_TUNNEL_CSUM_BIT, addr_of!((*nt).parms.o_flags).cast()) && (*nt).encap.type_ as u32 != TUNNEL_ENCAP_NONE { return; }
    (*dev).features |= rust_ip6gre_gso_features();
    (*dev).hw_features |= rust_ip6gre_gso_features();
}

unsafe fn ip6gre_tunnel_init_common(dev: *mut net_device) -> c_int {
    let tunnel = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    (*tunnel).dev = dev;
    rust_ip6gre_strscpy(addr_of_mut!((*tunnel).parms.name).cast(), addr_of!((*dev).name).cast(), IFNAMSIZ as usize);
    let ret = dst_cache_init(addr_of_mut!((*tunnel).dst_cache), RUST_IP6GRE_GFP_KERNEL);
    if ret != 0 { return ret; }
    let ret = gro_cells_init(addr_of_mut!((*tunnel).gro_cells), dev);
    if ret != 0 { dst_cache_destroy(addr_of_mut!((*tunnel).dst_cache)); return ret; }
    let t_hlen = ip6gre_calc_hlen(tunnel);
    (*dev).mtu = (ETH_DATA_LEN as u32).wrapping_sub(t_hlen as u32);
    if (*dev).type_ as u32 == ARPHRD_ETHER { (*dev).mtu = (*dev).mtu.wrapping_sub(ETH_HLEN); }
    if (*tunnel).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { (*dev).mtu = (*dev).mtu.wrapping_sub(8); }
    if (*tunnel).parms.collect_md { rust_ip6gre_keep_dst(dev); }
    ip6gre_tnl_init_features(dev);
    rust_ip6gre_netdev_hold(dev, addr_of_mut!((*tunnel).dev_tracker), RUST_IP6GRE_GFP_KERNEL);
    rust_ip6gre_lockdep_classes(dev);
    0
}

unsafe extern "C" fn ip6gre_tunnel_init(dev: *mut net_device) -> c_int {
    let ret = ip6gre_tunnel_init_common(dev);
    if ret != 0 { return ret; }
    let tunnel = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    if (*tunnel).parms.collect_md { return 0; }
    rust_ip6gre_dev_addr_set(dev, addr_of!((*tunnel).parms.laddr).cast(), size_of::<in6_addr>());
    copy_nonoverlapping(addr_of!((*tunnel).parms.raddr).cast::<u8>(), addr_of_mut!((*dev).broadcast).cast(), size_of::<in6_addr>());
    if rust_ip6gre_addr_any(addr_of!((*tunnel).parms.raddr)) { (*dev).header_ops = addr_of!(ip6gre_header_ops); }
    0
}

unsafe fn ip6gre_fb_tunnel_init(dev: *mut net_device) {
    let tunnel = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    (*tunnel).dev = dev;
    (*tunnel).net = rust_ip6gre_dev_net(dev);
    rust_ip6gre_strscpy(addr_of_mut!((*tunnel).parms.name).cast(), addr_of!((*dev).name).cast(), IFNAMSIZ as usize);
    (*tunnel).hlen = (size_of::<ipv6hdr>() + 4) as c_int;
}

const fn ip6gre_make_protocol() -> inet6_protocol {
    let mut protocol: inet6_protocol = unsafe { zeroed() };
    protocol.handler = Some(gre_rcv);
    protocol.err_handler = Some(ip6gre_err);
    protocol.flags = INET6_PROTO_FINAL as _;
    protocol
}
#[link_section = ".data..read_mostly"]
static mut ip6gre_protocol: inet6_protocol = ip6gre_make_protocol();

unsafe extern "C" fn ip6gre_exit_rtnl_net(net: *mut net, head: *mut list_head) {
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    let mut dev = rust_ip6gre_first_netdev(net);
    while !dev.is_null() {
        let next = rust_ip6gre_next_netdev(dev);
        if (*dev).rtnl_link_ops == addr_of!(ip6gre_link_ops) || (*dev).rtnl_link_ops == addr_of!(ip6gre_tap_ops) || (*dev).rtnl_link_ops == addr_of!(ip6erspan_tap_ops) {
            unregister_netdevice_queue(dev, head);
        }
        dev = next;
    }
    for prio in 0..4 {
        for h in 0..IP6_GRE_HASH_SIZE as usize {
            let mut t = rust_ip6gre_rtnl_net_read(net, addr_of!((*ign).tunnels[prio][h]));
            while !t.is_null() {
                if !rust_ip6gre_net_eq(rust_ip6gre_dev_net((*t).dev), net) { unregister_netdevice_queue((*t).dev, head); }
                t = rust_ip6gre_rtnl_net_read(net, addr_of!((*t).next));
            }
        }
    }
}

unsafe extern "C" fn ip6gre_init_net(net: *mut net) -> c_int {
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    if !rust_ip6gre_has_fallback(net) { return 0; }
    let ndev = rust_ip6gre_alloc_netdev(b"ip6gre0\0".as_ptr().cast(), Some(ip6gre_tunnel_setup));
    if ndev.is_null() { return neg(ENOMEM); }
    (*ign).fb_tunnel_dev = ndev;
    rust_ip6gre_dev_net_set(ndev, net);
    rust_ip6gre_set_netns_immutable(ndev, true);
    ip6gre_fb_tunnel_init(ndev);
    (*ndev).rtnl_link_ops = addr_of!(ip6gre_link_ops);
    let err = register_netdev(ndev);
    if err != 0 { free_netdev(ndev); return err; }
    rust_ip6gre_rcu_assign(addr_of_mut!((*ign).tunnels[0][0]), rust_ip6gre_netdev_priv(ndev).cast());
    0
}

const fn ip6gre_make_net_ops() -> pernet_operations {
    let mut ops: pernet_operations = unsafe { zeroed() };
    ops.init = Some(ip6gre_init_net);
    ops.exit_rtnl = Some(ip6gre_exit_rtnl_net);
    ops.id = addr_of_mut!(ip6gre_net_id);
    ops.size = size_of::<ip6gre_net>();
    ops
}
static mut ip6gre_net_ops: pernet_operations = ip6gre_make_net_ops();

unsafe fn gre_attr(attrs: *mut *mut nlattr, index: u32) -> *mut nlattr { *attrs.add(index as usize) }

unsafe extern "C" fn ip6gre_tunnel_validate(_tb: *mut *mut nlattr, data: *mut *mut nlattr, _extack: *mut netlink_ext_ack) -> c_int {
    if data.is_null() { return 0; }
    let mut flags = 0u16;
    let a = gre_attr(data, IFLA_GRE_IFLAGS);
    if !a.is_null() { flags |= rust_ip6gre_nla_u16(a); }
    let a = gre_attr(data, IFLA_GRE_OFLAGS);
    if !a.is_null() { flags |= rust_ip6gre_nla_u16(a); }
    if flags & be16(0x0007 | 0x4000) != 0 { neg(EINVAL) } else { 0 }
}

unsafe extern "C" fn ip6gre_tap_validate(tb: *mut *mut nlattr, data: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let a = gre_attr(tb, IFLA_ADDRESS);
    if !a.is_null() {
        if rust_ip6gre_nla_len(a) != ETH_ALEN as c_int { return neg(EINVAL); }
        if !rust_ip6gre_valid_ether_addr(rust_ip6gre_nla_data(a).cast()) { return neg(EADDRNOTAVAIL); }
    }
    if !data.is_null() {
        let a = gre_attr(data, IFLA_GRE_REMOTE);
        if !a.is_null() {
            let daddr = rust_ip6gre_nla_in6(a);
            if rust_ip6gre_addr_any(addr_of!(daddr)) { return neg(EINVAL); }
        }
    }
    ip6gre_tunnel_validate(tb, data, extack)
}

unsafe extern "C" fn ip6erspan_tap_validate(tb: *mut *mut nlattr, data: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    if data.is_null() { return 0; }
    let ret = ip6gre_tap_validate(tb, data, extack);
    if ret != 0 { return ret; }
    let mut flags = 0u16;
    let mut ver = 0u8;
    let a = gre_attr(data, IFLA_GRE_OFLAGS);
    if !a.is_null() { flags |= rust_ip6gre_nla_u16(a); }
    let a = gre_attr(data, IFLA_GRE_IFLAGS);
    if !a.is_null() { flags |= rust_ip6gre_nla_u16(a); }
    if gre_attr(data, IFLA_GRE_COLLECT_METADATA).is_null() && flags != be16(0x1000 | 0x2000) { return neg(EINVAL); }
    let a = gre_attr(data, IFLA_GRE_IKEY);
    if !a.is_null() && u32::from_be(rust_ip6gre_nla_u32(a)) & !(ID_MASK as u32) != 0 { return neg(EINVAL); }
    let a = gre_attr(data, IFLA_GRE_OKEY);
    if !a.is_null() && u32::from_be(rust_ip6gre_nla_u32(a)) & !(ID_MASK as u32) != 0 { return neg(EINVAL); }
    let a = gre_attr(data, IFLA_GRE_ERSPAN_VER);
    if !a.is_null() {
        ver = rust_ip6gre_nla_u8(a);
        if ver != 1 && ver != 2 { return neg(EINVAL); }
    }
    if ver == 1 {
        let a = gre_attr(data, IFLA_GRE_ERSPAN_INDEX);
        if !a.is_null() && rust_ip6gre_nla_u32(a) & !(INDEX_MASK as u32) != 0 { return neg(EINVAL); }
    } else if ver == 2 {
        let a = gre_attr(data, IFLA_GRE_ERSPAN_DIR);
        if !a.is_null() && (rust_ip6gre_nla_u8(a) as u32) & !((DIR_MASK as u32) >> DIR_OFFSET) != 0 { return neg(EINVAL); }
        let a = gre_attr(data, IFLA_GRE_ERSPAN_HWID);
        if !a.is_null() && (rust_ip6gre_nla_u16(a) as u32) & !((HWID_MASK as u32) >> HWID_OFFSET) != 0 { return neg(EINVAL); }
    }
    0
}

unsafe fn ip6erspan_set_version(data: *mut *mut nlattr, parms: *mut __ip6_tnl_parm) {
    if data.is_null() { return; }
    (*parms).erspan_ver = 1;
    let a = gre_attr(data, IFLA_GRE_ERSPAN_VER);
    if !a.is_null() { (*parms).erspan_ver = rust_ip6gre_nla_u8(a); }
    if (*parms).erspan_ver == 1 {
        let a = gre_attr(data, IFLA_GRE_ERSPAN_INDEX);
        if !a.is_null() { (*parms).index = rust_ip6gre_nla_u32(a); }
    } else if (*parms).erspan_ver == 2 {
        let a = gre_attr(data, IFLA_GRE_ERSPAN_DIR);
        if !a.is_null() { (*parms).dir = rust_ip6gre_nla_u8(a); }
        let a = gre_attr(data, IFLA_GRE_ERSPAN_HWID);
        if !a.is_null() { (*parms).hwid = rust_ip6gre_nla_u16(a); }
    }
}

unsafe fn ip6gre_netlink_parms(data: *mut *mut nlattr, parms: *mut __ip6_tnl_parm) {
    write_bytes(parms, 0, 1);
    if data.is_null() { return; }
    let a = gre_attr(data, IFLA_GRE_LINK);
    if !a.is_null() { (*parms).link = rust_ip6gre_nla_u32(a) as c_int; }
    let a = gre_attr(data, IFLA_GRE_IFLAGS);
    if !a.is_null() { rust_ip6gre_flags_from_gre(addr_of_mut!((*parms).i_flags).cast(), rust_ip6gre_nla_u16(a)); }
    let a = gre_attr(data, IFLA_GRE_OFLAGS);
    if !a.is_null() { rust_ip6gre_flags_from_gre(addr_of_mut!((*parms).o_flags).cast(), rust_ip6gre_nla_u16(a)); }
    let a = gre_attr(data, IFLA_GRE_IKEY);
    if !a.is_null() { (*parms).i_key = rust_ip6gre_nla_u32(a); }
    let a = gre_attr(data, IFLA_GRE_OKEY);
    if !a.is_null() { (*parms).o_key = rust_ip6gre_nla_u32(a); }
    let a = gre_attr(data, IFLA_GRE_LOCAL);
    if !a.is_null() { (*parms).laddr = rust_ip6gre_nla_in6(a); }
    let a = gre_attr(data, IFLA_GRE_REMOTE);
    if !a.is_null() { (*parms).raddr = rust_ip6gre_nla_in6(a); }
    let a = gre_attr(data, IFLA_GRE_TTL);
    if !a.is_null() { (*parms).hop_limit = rust_ip6gre_nla_u8(a); }
    let a = gre_attr(data, IFLA_GRE_ENCAP_LIMIT);
    if !a.is_null() { (*parms).encap_limit = rust_ip6gre_nla_u8(a); }
    let a = gre_attr(data, IFLA_GRE_FLOWINFO);
    if !a.is_null() { (*parms).flowinfo = rust_ip6gre_nla_u32(a); }
    let a = gre_attr(data, IFLA_GRE_FLAGS);
    if !a.is_null() { (*parms).flags = rust_ip6gre_nla_u32(a); }
    let a = gre_attr(data, IFLA_GRE_FWMARK);
    if !a.is_null() { (*parms).fwmark = rust_ip6gre_nla_u32(a); }
    if !gre_attr(data, IFLA_GRE_COLLECT_METADATA).is_null() { (*parms).collect_md = true; }
}

unsafe extern "C" fn ip6gre_tap_init(dev: *mut net_device) -> c_int {
    let ret = ip6gre_tunnel_init_common(dev);
    if ret != 0 { return ret; }
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) | IFF_LIVE_ADDR_CHANGE as kernel::ffi::c_ulong);
    0
}

const fn ip6gre_make_tap_netdev_ops() -> net_device_ops {
    let mut ops: net_device_ops = unsafe { zeroed() };
    ops.ndo_init = Some(ip6gre_tap_init);
    ops.ndo_uninit = Some(ip6gre_tunnel_uninit);
    ops.ndo_start_xmit = Some(ip6gre_tunnel_xmit);
    ops.ndo_set_mac_address = Some(eth_mac_addr);
    ops.ndo_validate_addr = Some(eth_validate_addr);
    ops.ndo_change_mtu = Some(ip6_tnl_change_mtu);
    ops.ndo_get_iflink = Some(ip6_tnl_get_iflink);
    ops
}
static ip6gre_tap_netdev_ops: net_device_ops = ip6gre_make_tap_netdev_ops();

unsafe fn ip6erspan_calc_hlen(tunnel: *mut ip6_tnl) -> c_int {
    (*tunnel).tun_hlen = 8;
    (*tunnel).hlen = (*tunnel).tun_hlen.wrapping_add((*tunnel).encap_hlen).wrapping_add(rust_ip6gre_erspan_hlen((*tunnel).parms.erspan_ver as c_int));
    let t_hlen = (*tunnel).hlen.wrapping_add(size_of::<ipv6hdr>() as c_int);
    (*(*tunnel).dev).needed_headroom = (RUST_IP6GRE_LL_MAX_HEADER as c_int).wrapping_add(t_hlen) as _;
    t_hlen
}

unsafe extern "C" fn ip6erspan_tap_init(dev: *mut net_device) -> c_int {
    let tunnel = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    (*tunnel).dev = dev;
    rust_ip6gre_strscpy(addr_of_mut!((*tunnel).parms.name).cast(), addr_of!((*dev).name).cast(), IFNAMSIZ as usize);
    let ret = dst_cache_init(addr_of_mut!((*tunnel).dst_cache), RUST_IP6GRE_GFP_KERNEL);
    if ret != 0 { return ret; }
    let ret = gro_cells_init(addr_of_mut!((*tunnel).gro_cells), dev);
    if ret != 0 { dst_cache_destroy(addr_of_mut!((*tunnel).dst_cache)); return ret; }
    let t_hlen = ip6erspan_calc_hlen(tunnel);
    (*dev).mtu = ETH_DATA_LEN.wrapping_sub(t_hlen as u32);
    if (*dev).type_ as u32 == ARPHRD_ETHER { (*dev).mtu = (*dev).mtu.wrapping_sub(ETH_HLEN); }
    if (*tunnel).parms.flags & IP6_TNL_F_IGN_ENCAP_LIMIT == 0 { (*dev).mtu = (*dev).mtu.wrapping_sub(8); }
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) | IFF_LIVE_ADDR_CHANGE as kernel::ffi::c_ulong);
    ip6erspan_tnl_link_config(tunnel, 1);
    rust_ip6gre_netdev_hold(dev, addr_of_mut!((*tunnel).dev_tracker), RUST_IP6GRE_GFP_KERNEL);
    rust_ip6gre_lockdep_classes(dev);
    0
}

const fn ip6erspan_make_netdev_ops() -> net_device_ops {
    let mut ops: net_device_ops = unsafe { zeroed() };
    ops.ndo_init = Some(ip6erspan_tap_init);
    ops.ndo_uninit = Some(ip6erspan_tunnel_uninit);
    ops.ndo_start_xmit = Some(ip6erspan_tunnel_xmit);
    ops.ndo_set_mac_address = Some(eth_mac_addr);
    ops.ndo_validate_addr = Some(eth_validate_addr);
    ops.ndo_change_mtu = Some(ip6_tnl_change_mtu);
    ops.ndo_get_iflink = Some(ip6_tnl_get_iflink);
    ops
}
static ip6erspan_netdev_ops: net_device_ops = ip6erspan_make_netdev_ops();

unsafe extern "C" fn ip6gre_tap_setup(dev: *mut net_device) {
    ether_setup(dev);
    (*dev).max_mtu = 0;
    (*dev).netdev_ops = addr_of!(ip6gre_tap_netdev_ops);
    (*dev).needs_free_netdev = true;
    (*dev).priv_destructor = Some(ip6gre_dev_free);
    rust_ip6gre_set_pcpu_stat_type(dev, NETDEV_PCPU_STAT_TSTATS);
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) & !(IFF_TX_SKB_SHARING as kernel::ffi::c_ulong));
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) | IFF_LIVE_ADDR_CHANGE as kernel::ffi::c_ulong);
    rust_ip6gre_keep_dst(dev);
}

unsafe fn ip6gre_netlink_encap_parms(data: *mut *mut nlattr, ipencap: *mut ip_tunnel_encap) -> bool {
    write_bytes(ipencap, 0, 1);
    if data.is_null() { return false; }
    let mut ret = false;
    let a = gre_attr(data, IFLA_GRE_ENCAP_TYPE);
    if !a.is_null() { ret = true; (*ipencap).type_ = rust_ip6gre_nla_u16(a); }
    let a = gre_attr(data, IFLA_GRE_ENCAP_FLAGS);
    if !a.is_null() { ret = true; (*ipencap).flags = rust_ip6gre_nla_u16(a); }
    let a = gre_attr(data, IFLA_GRE_ENCAP_SPORT);
    if !a.is_null() { ret = true; (*ipencap).sport = rust_ip6gre_nla_u16(a); }
    let a = gre_attr(data, IFLA_GRE_ENCAP_DPORT);
    if !a.is_null() { ret = true; (*ipencap).dport = rust_ip6gre_nla_u16(a); }
    ret
}

unsafe fn ip6gre_newlink_common(link_net: *mut net, dev: *mut net_device, tb: *mut *mut nlattr, data: *mut *mut nlattr, _extack: *mut netlink_ext_ack) -> c_int {
    let nt = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let mut ipencap: ip_tunnel_encap = zeroed();
    if ip6gre_netlink_encap_parms(data, addr_of_mut!(ipencap)) {
        let err = ip6_tnl_encap_setup(nt, addr_of_mut!(ipencap));
        if err < 0 { return err; }
    }
    if (*dev).type_ as u32 == ARPHRD_ETHER && gre_attr(tb, IFLA_ADDRESS).is_null() { rust_ip6gre_hw_addr_random(dev); }
    (*nt).dev = dev;
    (*nt).net = link_net;
    let err = register_netdevice(dev);
    if err != 0 { return err; }
    let a = gre_attr(tb, IFLA_MTU);
    if !a.is_null() { ip6_tnl_change_mtu(dev, rust_ip6gre_nla_u32(a) as c_int); }
    err
}

unsafe extern "C" fn ip6gre_newlink(dev: *mut net_device, params: *mut rtnl_newlink_params, extack: *mut netlink_ext_ack) -> c_int {
    let net = if (*params).link_net.is_null() { rust_ip6gre_dev_net(dev) } else { (*params).link_net };
    let nt = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let data = (*params).data;
    let tb = (*params).tb;
    ip6gre_netlink_parms(data, addr_of_mut!((*nt).parms));
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    if (*nt).parms.collect_md {
        if !rust_ip6gre_rtnl_read(addr_of!((*ign).collect_md_tun)).is_null() { return neg(EEXIST); }
    } else if !ip6gre_tunnel_find(net, addr_of!((*nt).parms), (*dev).type_ as c_int).is_null() { return neg(EEXIST); }
    let err = ip6gre_newlink_common(net, dev, tb, data, extack);
    if err == 0 {
        ip6gre_tnl_link_config(nt, gre_attr(tb, IFLA_MTU).is_null() as c_int);
        ip6gre_tunnel_link_md(ign, nt);
        ip6gre_tunnel_link(rust_ip6gre_net_generic(net, ip6gre_net_id).cast(), nt);
    }
    err
}

unsafe fn ip6gre_changelink_common(dev: *mut net_device, _tb: *mut *mut nlattr, data: *mut *mut nlattr, p: *mut __ip6_tnl_parm, _extack: *mut netlink_ext_ack) -> *mut ip6_tnl {
    let nt = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let net = (*nt).net;
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    let mut ipencap: ip_tunnel_encap = zeroed();
    if dev == (*ign).fb_tunnel_dev { return neg(EINVAL) as isize as *mut ip6_tnl; }
    if ip6gre_netlink_encap_parms(data, addr_of_mut!(ipencap)) {
        let err = ip6_tnl_encap_setup(nt, addr_of_mut!(ipencap));
        if err < 0 { return err as isize as *mut ip6_tnl; }
    }
    ip6gre_netlink_parms(data, p);
    let t = ip6gre_tunnel_locate(net, p, 0);
    if !t.is_null() {
        if (*t).dev != dev { return neg(EEXIST) as isize as *mut ip6_tnl; }
        t
    } else { nt }
}

unsafe extern "C" fn ip6gre_changelink(dev: *mut net_device, tb: *mut *mut nlattr, data: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let mut t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let ign = rust_ip6gre_net_generic((*t).net, ip6gre_net_id).cast::<ip6gre_net>();
    let mut p: __ip6_tnl_parm = zeroed();
    if !rtnl_dev_link_net_capable(dev, (*t).net) { return neg(EPERM); }
    t = ip6gre_changelink_common(dev, tb, data, addr_of_mut!(p), extack);
    if rust_ip6gre_is_err(t.cast()) { return t as isize as c_int; }
    ip6gre_tunnel_unlink_md(ign, t);
    ip6gre_tunnel_unlink(ign, t);
    ip6gre_tnl_change(t, addr_of!(p), gre_attr(tb, IFLA_MTU).is_null() as c_int);
    ip6gre_tunnel_link_md(ign, t);
    ip6gre_tunnel_link(ign, t);
    0
}

unsafe extern "C" fn ip6gre_dellink(dev: *mut net_device, head: *mut list_head) {
    let net = rust_ip6gre_dev_net(dev);
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    if dev != (*ign).fb_tunnel_dev { unregister_netdevice_queue(dev, head); }
}

unsafe extern "C" fn ip6gre_get_size(_dev: *const net_device) -> usize {
    // Keep the original estimator, including its lack of separate ERSPAN-v2 sizes.
    let lengths = [4, 2, 2, 4, 4, size_of::<in6_addr>() as c_int, size_of::<in6_addr>() as c_int,
        1, 1, 4, 4, 2, 2, 2, 2, 0, 4, 4];
    let mut total = 0usize;
    for payload in lengths { total = total.wrapping_add(rust_ip6gre_nla_total_size(payload) as usize); }
    total
}

unsafe fn gre_nla_put<T>(skb: *mut sk_buff, attr: u32, value: &T) -> c_int {
    nla_put(skb, attr as c_int, size_of::<T>() as c_int, (value as *const T).cast())
}

unsafe extern "C" fn ip6gre_fill_info(skb: *mut sk_buff, dev: *const net_device) -> c_int {
    let t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let p = addr_of!((*t).parms);
    let mut o_flags: TunnelFlags = zeroed();
    rust_ip6gre_flags_copy(o_flags.as_mut_ptr(), addr_of!((*p).o_flags).cast());
    if (*p).erspan_ver == 1 || (*p).erspan_ver == 2 {
        if !(*p).collect_md { rust_ip6gre_set_bit(IP_TUNNEL_KEY_BIT, o_flags.as_mut_ptr()); }
        if gre_nla_put(skb, IFLA_GRE_ERSPAN_VER, &(*p).erspan_ver) != 0 { return neg(EMSGSIZE); }
        if (*p).erspan_ver == 1 {
            if gre_nla_put(skb, IFLA_GRE_ERSPAN_INDEX, &(*p).index) != 0 { return neg(EMSGSIZE); }
        } else {
            if gre_nla_put(skb, IFLA_GRE_ERSPAN_DIR, &(*p).dir) != 0 { return neg(EMSGSIZE); }
            if gre_nla_put(skb, IFLA_GRE_ERSPAN_HWID, &(*p).hwid) != 0 { return neg(EMSGSIZE); }
        }
    }
    if gre_nla_put(skb, IFLA_GRE_LINK, &((*p).link as u32)) != 0
        || gre_nla_put(skb, IFLA_GRE_IFLAGS, &rust_ip6gre_flags_to_gre(addr_of!((*p).i_flags).cast())) != 0
        || gre_nla_put(skb, IFLA_GRE_OFLAGS, &rust_ip6gre_flags_to_gre(o_flags.as_ptr())) != 0
        || gre_nla_put(skb, IFLA_GRE_IKEY, &(*p).i_key) != 0
        || gre_nla_put(skb, IFLA_GRE_OKEY, &(*p).o_key) != 0
        || gre_nla_put(skb, IFLA_GRE_LOCAL, &(*p).laddr) != 0
        || gre_nla_put(skb, IFLA_GRE_REMOTE, &(*p).raddr) != 0
        || gre_nla_put(skb, IFLA_GRE_TTL, &(*p).hop_limit) != 0
        || gre_nla_put(skb, IFLA_GRE_ENCAP_LIMIT, &(*p).encap_limit) != 0
        || gre_nla_put(skb, IFLA_GRE_FLOWINFO, &(*p).flowinfo) != 0
        || gre_nla_put(skb, IFLA_GRE_FLAGS, &(*p).flags) != 0
        || gre_nla_put(skb, IFLA_GRE_FWMARK, &(*p).fwmark) != 0 { return neg(EMSGSIZE); }
    if gre_nla_put(skb, IFLA_GRE_ENCAP_TYPE, &(*t).encap.type_) != 0
        || gre_nla_put(skb, IFLA_GRE_ENCAP_SPORT, &(*t).encap.sport) != 0
        || gre_nla_put(skb, IFLA_GRE_ENCAP_DPORT, &(*t).encap.dport) != 0
        || gre_nla_put(skb, IFLA_GRE_ENCAP_FLAGS, &(*t).encap.flags) != 0 { return neg(EMSGSIZE); }
    if (*p).collect_md && nla_put(skb, IFLA_GRE_COLLECT_METADATA as c_int, 0, null()) != 0 { return neg(EMSGSIZE); }
    0
}

const fn ip6gre_make_policy() -> [nla_policy; RUST_IP6GRE_IFLA_GRE_MAX as usize + 1] {
    let mut p: [nla_policy; RUST_IP6GRE_IFLA_GRE_MAX as usize + 1] = unsafe { zeroed() };
    p[IFLA_GRE_LINK as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_IFLAGS as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_OFLAGS as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_IKEY as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_OKEY as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_LOCAL as usize].len = size_of::<in6_addr>() as u16;
    p[IFLA_GRE_REMOTE as usize].len = size_of::<in6_addr>() as u16;
    p[IFLA_GRE_TTL as usize].type_ = NLA_U8 as u8;
    p[IFLA_GRE_ENCAP_LIMIT as usize].type_ = NLA_U8 as u8;
    p[IFLA_GRE_FLOWINFO as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_FLAGS as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_ENCAP_TYPE as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_ENCAP_FLAGS as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_ENCAP_SPORT as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_ENCAP_DPORT as usize].type_ = NLA_U16 as u8;
    p[IFLA_GRE_COLLECT_METADATA as usize].type_ = NLA_FLAG as u8;
    p[IFLA_GRE_FWMARK as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_ERSPAN_INDEX as usize].type_ = NLA_U32 as u8;
    p[IFLA_GRE_ERSPAN_VER as usize].type_ = NLA_U8 as u8;
    p[IFLA_GRE_ERSPAN_DIR as usize].type_ = NLA_U8 as u8;
    p[IFLA_GRE_ERSPAN_HWID as usize].type_ = NLA_U16 as u8;
    p
}
// The original const policy array is immutable after construction. Any raw
// validation pointers are used only under the kernel netlink contracts.
unsafe impl Sync for nla_policy {}

static ip6gre_policy: [nla_policy; RUST_IP6GRE_IFLA_GRE_MAX as usize + 1] = ip6gre_make_policy();

unsafe extern "C" fn ip6erspan_tap_setup(dev: *mut net_device) {
    ether_setup(dev);
    (*dev).max_mtu = 0;
    (*dev).netdev_ops = addr_of!(ip6erspan_netdev_ops);
    (*dev).needs_free_netdev = true;
    (*dev).priv_destructor = Some(ip6gre_dev_free);
    rust_ip6gre_set_pcpu_stat_type(dev, NETDEV_PCPU_STAT_TSTATS);
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) & !(IFF_TX_SKB_SHARING as kernel::ffi::c_ulong));
    rust_ip6gre_set_priv_flags(dev, rust_ip6gre_priv_flags(dev) | IFF_LIVE_ADDR_CHANGE as kernel::ffi::c_ulong);
    rust_ip6gre_keep_dst(dev);
}

unsafe extern "C" fn ip6erspan_newlink(dev: *mut net_device, params: *mut rtnl_newlink_params, extack: *mut netlink_ext_ack) -> c_int {
    let net = if (*params).link_net.is_null() { rust_ip6gre_dev_net(dev) } else { (*params).link_net };
    let nt = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let data = (*params).data;
    let tb = (*params).tb;
    ip6gre_netlink_parms(data, addr_of_mut!((*nt).parms));
    ip6erspan_set_version(data, addr_of_mut!((*nt).parms));
    let ign = rust_ip6gre_net_generic(net, ip6gre_net_id).cast::<ip6gre_net>();
    if (*nt).parms.collect_md {
        if !rust_ip6gre_rtnl_read(addr_of!((*ign).collect_md_tun_erspan)).is_null() { return neg(EEXIST); }
    } else if !ip6gre_tunnel_find(net, addr_of!((*nt).parms), (*dev).type_ as c_int).is_null() { return neg(EEXIST); }
    let err = ip6gre_newlink_common(net, dev, tb, data, extack);
    if err == 0 {
        ip6erspan_tnl_link_config(nt, gre_attr(tb, IFLA_MTU).is_null() as c_int);
        ip6erspan_tunnel_link_md(ign, nt);
        ip6gre_tunnel_link(rust_ip6gre_net_generic(net, ip6gre_net_id).cast(), nt);
    }
    err
}

unsafe fn ip6erspan_tnl_link_config(t: *mut ip6_tnl, set_mtu: c_int) {
    ip6gre_tnl_link_config_common(t);
    ip6gre_tnl_link_config_route(t, set_mtu, ip6erspan_calc_hlen(t));
}

unsafe fn ip6erspan_tnl_change(t: *mut ip6_tnl, p: *const __ip6_tnl_parm, set_mtu: c_int) -> c_int {
    ip6gre_tnl_copy_tnl_parm(t, p);
    ip6erspan_tnl_link_config(t, set_mtu);
    0
}

unsafe extern "C" fn ip6erspan_changelink(dev: *mut net_device, tb: *mut *mut nlattr, data: *mut *mut nlattr, extack: *mut netlink_ext_ack) -> c_int {
    let mut t = rust_ip6gre_netdev_priv(dev).cast::<ip6_tnl>();
    let mut p: __ip6_tnl_parm = zeroed();
    if !rtnl_dev_link_net_capable(dev, (*t).net) { return neg(EPERM); }
    let ign = rust_ip6gre_net_generic((*t).net, ip6gre_net_id).cast::<ip6gre_net>();
    t = ip6gre_changelink_common(dev, tb, data, addr_of_mut!(p), extack);
    if rust_ip6gre_is_err(t.cast()) { return t as isize as c_int; }
    ip6erspan_set_version(data, addr_of_mut!(p));
    // The C ERSPAN change path intentionally uses the GRE metadata unlink.
    ip6gre_tunnel_unlink_md(ign, t);
    ip6gre_tunnel_unlink(ign, t);
    ip6erspan_tnl_change(t, addr_of!(p), gre_attr(tb, IFLA_MTU).is_null() as c_int);
    ip6erspan_tunnel_link_md(ign, t);
    ip6gre_tunnel_link(ign, t);
    0
}

const fn ip6gre_make_link_ops() -> rtnl_link_ops {
    let mut ops: rtnl_link_ops = unsafe { zeroed() };
    ops.kind = b"ip6gre\0".as_ptr().cast();
    ops.maxtype = RUST_IP6GRE_IFLA_GRE_MAX;
    ops.policy = addr_of!(ip6gre_policy).cast();
    ops.priv_size = size_of::<ip6_tnl>();
    ops.setup = Some(ip6gre_tunnel_setup);
    ops.validate = Some(ip6gre_tunnel_validate);
    ops.newlink = Some(ip6gre_newlink);
    ops.changelink = Some(ip6gre_changelink);
    ops.dellink = Some(ip6gre_dellink);
    ops.get_size = Some(ip6gre_get_size);
    ops.fill_info = Some(ip6gre_fill_info);
    ops.get_link_net = Some(ip6_tnl_get_link_net);
    ops
}
#[link_section = ".data..read_mostly"]
static mut ip6gre_link_ops: rtnl_link_ops = ip6gre_make_link_ops();

const fn ip6gre_make_tap_ops() -> rtnl_link_ops {
    let mut ops: rtnl_link_ops = unsafe { zeroed() };
    ops.kind = b"ip6gretap\0".as_ptr().cast();
    ops.maxtype = RUST_IP6GRE_IFLA_GRE_MAX;
    ops.policy = addr_of!(ip6gre_policy).cast();
    ops.priv_size = size_of::<ip6_tnl>();
    ops.setup = Some(ip6gre_tap_setup);
    ops.validate = Some(ip6gre_tap_validate);
    ops.newlink = Some(ip6gre_newlink);
    ops.changelink = Some(ip6gre_changelink);
    ops.get_size = Some(ip6gre_get_size);
    ops.fill_info = Some(ip6gre_fill_info);
    ops.get_link_net = Some(ip6_tnl_get_link_net);
    ops
}
#[link_section = ".data..read_mostly"]
static mut ip6gre_tap_ops: rtnl_link_ops = ip6gre_make_tap_ops();

const fn ip6erspan_make_tap_ops() -> rtnl_link_ops {
    let mut ops: rtnl_link_ops = unsafe { zeroed() };
    ops.kind = b"ip6erspan\0".as_ptr().cast();
    ops.maxtype = RUST_IP6GRE_IFLA_GRE_MAX;
    ops.policy = addr_of!(ip6gre_policy).cast();
    ops.priv_size = size_of::<ip6_tnl>();
    ops.setup = Some(ip6erspan_tap_setup);
    ops.validate = Some(ip6erspan_tap_validate);
    ops.newlink = Some(ip6erspan_newlink);
    ops.changelink = Some(ip6erspan_changelink);
    ops.get_size = Some(ip6gre_get_size);
    ops.fill_info = Some(ip6gre_fill_info);
    ops.get_link_net = Some(ip6_tnl_get_link_net);
    ops
}
#[link_section = ".data..read_mostly"]
static mut ip6erspan_tap_ops: rtnl_link_ops = ip6erspan_make_tap_ops();

#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_ip6gre_init() -> c_int {
    rust_ip6gre_log_driver();
    let err = register_pernet_device(addr_of_mut!(ip6gre_net_ops));
    if err < 0 { return err; }
    let err = inet6_add_protocol(addr_of!(ip6gre_protocol), IPPROTO_GRE as u8);
    if err < 0 {
        rust_ip6gre_log_add_protocol();
        unregister_pernet_device(addr_of_mut!(ip6gre_net_ops));
        return err;
    }
    let err = rtnl_link_register(addr_of_mut!(ip6gre_link_ops));
    if err < 0 {
        inet6_del_protocol(addr_of!(ip6gre_protocol), IPPROTO_GRE as u8);
        unregister_pernet_device(addr_of_mut!(ip6gre_net_ops));
        return err;
    }
    let err = rtnl_link_register(addr_of_mut!(ip6gre_tap_ops));
    if err < 0 {
        rtnl_link_unregister(addr_of_mut!(ip6gre_link_ops));
        inet6_del_protocol(addr_of!(ip6gre_protocol), IPPROTO_GRE as u8);
        unregister_pernet_device(addr_of_mut!(ip6gre_net_ops));
        return err;
    }
    let err = rtnl_link_register(addr_of_mut!(ip6erspan_tap_ops));
    if err < 0 {
        rtnl_link_unregister(addr_of_mut!(ip6gre_tap_ops));
        rtnl_link_unregister(addr_of_mut!(ip6gre_link_ops));
        inet6_del_protocol(addr_of!(ip6gre_protocol), IPPROTO_GRE as u8);
        unregister_pernet_device(addr_of_mut!(ip6gre_net_ops));
    }
    err
}

#[no_mangle]
#[link_section = ".exit.text"]
pub unsafe extern "C" fn rust_ip6gre_fini() {
    rtnl_link_unregister(addr_of_mut!(ip6gre_tap_ops));
    rtnl_link_unregister(addr_of_mut!(ip6gre_link_ops));
    rtnl_link_unregister(addr_of_mut!(ip6erspan_tap_ops));
    inet6_del_protocol(addr_of!(ip6gre_protocol), IPPROTO_GRE as u8);
    unregister_pernet_device(addr_of_mut!(ip6gre_net_ops));
}
