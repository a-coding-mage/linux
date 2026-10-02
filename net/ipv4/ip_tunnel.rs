// SPDX-License-Identifier: GPL-2.0-only
// Reconstruction of the retained ip_tunnel.c, not recovery of historical work.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ip_tunnel_generated.rs"));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, copy_nonoverlapping, null, null_mut};
use kernel::ffi::{c_char, c_int, c_ulong, c_void};

#[inline] fn neg(e: u32) -> c_int { -(e as c_int) }
#[inline] const fn be16(v: u32) -> u16 { (v as u16).to_be() }
#[inline] unsafe fn priv_tunnel(d: *const net_device) -> *mut ip_tunnel { rust_ipt_netdev_priv(d).cast() }
#[inline] unsafe fn tunnel_net(n: *const net, id: u32) -> *mut ip_tunnel_net { rust_ipt_net_generic(n, id).cast() }
#[inline] unsafe fn src(h: *const iphdr) -> u32 { (*h).__bindgen_anon_1.addrs.saddr }
#[inline] unsafe fn dst(h: *const iphdr) -> u32 { (*h).__bindgen_anon_1.addrs.daddr }
#[inline] unsafe fn multicast(a: u32) -> bool { a & be32(0xf0000000) == be32(0xe0000000) }
#[inline] const fn be32(v: u32) -> u32 { v.to_be() }
#[inline] fn err_ptr<T>(e: c_int) -> *mut T { e as isize as *mut T }
#[inline] unsafe fn ptr_err_or_zero<T>(p: *const T) -> c_int { if rust_ipt_is_err(p.cast()) { p as isize as c_int } else { 0 } }
#[inline] unsafe fn node_tunnel(n: *mut hlist_node) -> *mut ip_tunnel { n.cast::<u8>().sub(offset_of!(ip_tunnel, hash_node)).cast() }
#[inline] unsafe fn key_match(p: *const ip_tunnel_parm_kern, f: *const c_ulong, key: u32) -> bool {
    if !rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, f) { return !rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags.as_ptr()); }
    rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags.as_ptr()) && (*p).i_key == key
}

#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_lookup(itn: *mut ip_tunnel_net, link: c_int,
    flags: *const c_ulong, remote: u32, local: u32, key: u32) -> *mut ip_tunnel {
    let mut cand: *mut ip_tunnel = null_mut();
    // Preserve four ordered passes, including later exact-link matches taking
    // precedence over earlier candidates with a mismatched link.
    for pass in 0..4 {
        let hash = rust_ipt_hash32(key ^ if pass < 2 { remote } else { 0 }, IP_TNL_HASH_BITS);
        let head = addr_of!((*itn).tunnels).cast::<hlist_head>().add(hash as usize);
        let mut n = rust_ipt_hlist_first(head, false);
        while !n.is_null() {
            let t = node_tunnel(n);
            let p = addr_of!((*t).parms);
            let iph = addr_of!((*p).iph);
            let matches = match pass {
                0 => local == src(iph) && remote == dst(iph),
                1 => remote == dst(iph) && src(iph) == 0,
                2 => (local == src(iph) && dst(iph) == 0) || (local == dst(iph) && multicast(local)),
                _ => (rust_ipt_test_bit(IP_TUNNEL_NO_KEY_BIT, flags) || (*p).i_key == key)
                    && src(iph) == 0 && dst(iph) == 0,
            };
            if matches && (*(*t).dev).flags & IFF_UP != 0 && (pass == 3 || key_match(p, flags, key)) {
                if rust_ipt_read_int(addr_of!((*p).link)) == link { return t; }
                if pass == 0 || cand.is_null() { cand = t; }
            }
            n = rust_ipt_hlist_next(n);
        }
    }
    if !cand.is_null() { return cand; }
    let t = rust_ipt_rcu_read(addr_of!((*itn).collect_md_tun));
    if !t.is_null() && (*(*t).dev).flags & IFF_UP != 0 { return t; }
    let d = rust_ipt_read_dev(addr_of!((*itn).fb_tunnel_dev));
    if !d.is_null() && (*d).flags & IFF_UP != 0 { return priv_tunnel(d); }
    null_mut()
}
unsafe fn bucket(itn: *mut ip_tunnel_net, p: *const ip_tunnel_parm_kern) -> *mut hlist_head {
    let d = dst(addr_of!((*p).iph));
    let remote = if d != 0 && !multicast(d) { d } else { 0 };
    let mut key = (*p).i_key;
    if !rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags.as_ptr()) && rust_ipt_test_bit(IP_TUNNEL_VTI_BIT, (*p).i_flags.as_ptr()) { key = 0; }
    addr_of_mut!((*itn).tunnels).cast::<hlist_head>().add(rust_ipt_hash32(key ^ remote, IP_TNL_HASH_BITS) as usize)
}
unsafe fn add(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    let head = bucket(itn, addr_of!((*t).parms));
    if (*t).collect_md { rust_ipt_rcu_assign(addr_of_mut!((*itn).collect_md_tun), t); }
    rust_ipt_hlist_add(addr_of_mut!((*t).hash_node), head);
}
unsafe fn del(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    if (*t).collect_md { rust_ipt_rcu_assign(addr_of_mut!((*itn).collect_md_tun), null_mut()); }
    rust_ipt_hlist_del(addr_of_mut!((*t).hash_node));
}
unsafe fn find(itn: *mut ip_tunnel_net, p: *const ip_tunnel_parm_kern, typ: c_int) -> *mut ip_tunnel {
    let local = src(addr_of!((*p).iph));
    let remote = dst(addr_of!((*p).iph));
    let flags = (*p).i_flags;
    let key = (*p).i_key;
    let link = (*p).link;
    let head = bucket(itn, p);
    let mut n = rust_ipt_hlist_first(head, true);
    while !n.is_null() {
        let t = node_tunnel(n);
        if local == src(addr_of!((*t).parms.iph)) && remote == dst(addr_of!((*t).parms.iph))
            && link == rust_ipt_read_int(addr_of!((*t).parms.link)) && typ == (*(*t).dev).type_ as c_int
            && key_match(addr_of!((*t).parms), flags.as_ptr(), key) { return t; }
        n = rust_ipt_hlist_next(n);
    }
    null_mut()
}
unsafe fn create_dev(n: *mut net, ops: *const rtnl_link_ops, p: *const ip_tunnel_parm_kern) -> *mut net_device {
    let mut name = [0 as c_char; IFNAMSIZ as usize];
    if (*p).name[0] != 0 {
        if !dev_valid_name((*p).name.as_ptr()) { return err_ptr(neg(E2BIG)); }
        rust_ipt_strscpy(name.as_mut_ptr(), (*p).name.as_ptr(), name.len());
    } else {
        let mut len = 0;
        while *(*ops).kind.add(len) != 0 { len += 1; }
        if len > IFNAMSIZ as usize - 3 { return err_ptr(neg(E2BIG)); }
        rust_ipt_strscpy(name.as_mut_ptr(), (*ops).kind, name.len());
        name[len] = b'%' as c_char;
        name[len + 1] = b'd' as c_char;
        name[len + 2] = 0;
    }
    rust_ipt_assert_rtnl();
    let dev = rust_ipt_alloc_netdev((*ops).priv_size as u32, name.as_ptr(), (*ops).setup);
    if dev.is_null() { return err_ptr(neg(ENOMEM)); }
    rust_ipt_dev_net_set(dev, n);
    (*dev).rtnl_link_ops = ops;
    let t = priv_tunnel(dev);
    (*t).parms = *p;
    (*t).net = n;
    let e = register_netdevice(dev);
    if e != 0 { free_netdev(dev); return err_ptr(e); }
    dev
}
unsafe fn bind_dev(dev: *mut net_device) -> c_int {
    let t = priv_tunnel(dev);
    let iph = addr_of!((*t).parms.iph);
    let mut tdev = null_mut();
    let mut hlen = RUST_IPT_LL_MAX_HEADER as c_int;
    let mut mtu = ETH_DATA_LEN as c_int;
    let th = (*t).hlen.wrapping_add(size_of::<iphdr>() as c_int);
    if dst(iph) != 0 {
        let mut fl: flowi4 = zeroed();
        rust_ipt_init_flow(&mut fl, (*iph).protocol as c_int, dst(iph), src(iph), (*t).parms.o_key,
            (*iph).tos & INET_DSCP_MASK as u8, (*t).net, (*t).parms.link, (*t).fwmark, 0, 0);
        let rt = rust_ipt_route_output((*t).net, &mut fl);
        if !rust_ipt_is_err(rt.cast()) { tdev = rust_ipt_dst_dev(addr_of!((*rt).dst)); rust_ipt_rt_put(rt); }
        if (*dev).type_ != ARPHRD_ETHER as u16 { (*dev).flags |= IFF_POINTOPOINT; }
        rust_ipt_cache_reset(addr_of_mut!((*t).dst_cache));
    }
    if tdev.is_null() && (*t).parms.link != 0 { tdev = __dev_get_by_index((*t).net, (*t).parms.link); }
    if !tdev.is_null() {
        hlen = ((*tdev).hard_header_len as c_int).wrapping_add((*tdev).needed_headroom as c_int);
        mtu = core::cmp::min((*tdev).mtu, IP_MAX_MTU) as c_int;
    }
    (*dev).needed_headroom = rust_ipt_limit_headroom(th.wrapping_add(hlen) as u32) as _;
    mtu = mtu.wrapping_sub(th.wrapping_add(if (*dev).type_ == ARPHRD_ETHER as u16 { (*dev).hard_header_len as c_int } else { 0 }));
    core::cmp::max(mtu, IPV4_MIN_MTU as c_int)
}
unsafe fn create(n: *mut net, itn: *mut ip_tunnel_net, p: *const ip_tunnel_parm_kern) -> *mut ip_tunnel {
    let dev = create_dev(n, (*itn).rtnl_link_ops, p);
    if rust_ipt_is_err(dev.cast()) { return dev.cast(); }
    let e = dev_set_mtu(dev, bind_dev(dev));
    if e != 0 { rust_ipt_unregister_netdevice(dev); return err_ptr(e); }
    let t = priv_tunnel(dev);
    let th = (*t).hlen.wrapping_add(size_of::<iphdr>() as c_int);
    (*dev).min_mtu = ETH_MIN_MTU;
    (*dev).max_mtu = (IP_MAX_MTU as c_int).wrapping_sub(th) as u32;
    if (*dev).type_ == ARPHRD_ETHER as u16 { (*dev).max_mtu = (*dev).max_mtu.wrapping_sub((*dev).hard_header_len as u32); }
    add(itn, t);
    t
}

#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_md_udp_encap(skb: *mut sk_buff, info: *mut ip_tunnel_info) {
    let h = rust_ipt_ip_hdr(skb);
    if (*h).protocol != IPPROTO_UDP as u8 { return; }
    let u = h.cast::<u8>().add(((*h).ihl() as usize) << 2).cast::<udphdr>();
    (*info).encap.sport = (*u).source;
    (*info).encap.dport = (*u).dest;
}
unsafe fn receive_drop(skb: *mut sk_buff, md: *mut metadata_dst) -> c_int {
    if !md.is_null() { dst_release(md.cast()); }
    rust_ipt_kfree_skb(skb);
    0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_rcv(t: *mut ip_tunnel, skb: *mut sk_buff,
    p: *const tnl_ptk_info, md: *mut metadata_dst, log_ecn_error: bool) -> c_int {
    let dev = (*t).dev;
    #[cfg(CONFIG_NET_IPGRE_BROADCAST)]
    if multicast(dst(rust_ipt_ip_hdr(skb))) { rust_ipt_multicast(dev); rust_ipt_set_broadcast(skb); }
    if rust_ipt_test_bit(IP_TUNNEL_CSUM_BIT, (*t).parms.i_flags.as_ptr()) != rust_ipt_test_bit(IP_TUNNEL_CSUM_BIT, (*p).flags.as_ptr()) {
        rust_ipt_rx_crc_errors(dev); rust_ipt_rx_errors(dev); return receive_drop(skb, md);
    }
    if rust_ipt_test_bit(IP_TUNNEL_SEQ_BIT, (*t).parms.i_flags.as_ptr()) {
        let seq = u32::from_be((*p).seq);
        if !rust_ipt_test_bit(IP_TUNNEL_SEQ_BIT, (*p).flags.as_ptr())
            || ((*t).i_seqno != 0 && (seq.wrapping_sub((*t).i_seqno) as i32) < 0) {
            rust_ipt_rx_fifo_errors(dev); rust_ipt_rx_errors(dev); return receive_drop(skb, md);
        }
        (*t).i_seqno = seq.wrapping_add(1);
    }
    let nh = rust_ipt_network_header(skb).offset_from((*skb).head) as c_int;
    rust_ipt_set_network_header(skb, if (*dev).type_ == ARPHRD_ETHER as u16 { ETH_HLEN as c_int } else { 0 });
    if !rust_ipt_inet_may_pull(skb) { rust_ipt_rx_length_errors(dev); rust_ipt_rx_errors(dev); return receive_drop(skb, md); }
    let h: *mut iphdr = (*skb).head.offset(nh as isize).cast();
    let e = rust_ipt_ecn_decap(h, skb);
    if e != 0 {
        if log_ecn_error { rust_ipt_log_ecn(h); }
        if e > 1 { rust_ipt_rx_frame_errors(dev); rust_ipt_rx_errors(dev); return receive_drop(skb, md); }
    }
    rust_ipt_stats_rx_add(dev, (*skb).len);
    skb_scrub_packet(skb, !rust_ipt_net_eq((*t).net, rust_ipt_dev_net(dev)));
    if (*dev).type_ == ARPHRD_ETHER as u16 {
        rust_ipt_skb_set_protocol(skb, eth_type_trans(skb, dev));
        rust_ipt_postpull_eth_rcsum(skb);
    } else { rust_ipt_skb_set_dev(skb, dev); }
    if !md.is_null() { rust_ipt_skb_dst_set(skb, md.cast()); }
    gro_cells_receive(addr_of_mut!((*t).gro_cells), skb);
    0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_encap_add_ops(ops: *const ip_tunnel_encap_ops, num: u32) -> c_int {
    if num >= MAX_IPTUN_ENCAP_OPS { return neg(ERANGE); }
    if rust_ipt_cmpxchg_encap(num, null(), ops).is_null() { 0 } else { -1 }
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_encap_del_ops(ops: *const ip_tunnel_encap_ops, num: u32) -> c_int {
    if num >= MAX_IPTUN_ENCAP_OPS { return neg(ERANGE); }
    let e = if rust_ipt_cmpxchg_encap(num, ops, null()) == ops { 0 } else { -1 };
    synchronize_net();
    e
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_encap_setup(t: *mut ip_tunnel, encap: *mut ip_tunnel_encap) -> c_int {
    (*t).encap = zeroed();
    let h = rust_ipt_encap_hlen(encap);
    if h < 0 { return h; }
    (*t).encap.type_ = (*encap).type_;
    (*t).encap.sport = (*encap).sport;
    (*t).encap.dport = (*encap).dport;
    (*t).encap.flags = (*encap).flags;
    (*t).encap_hlen = h;
    (*t).hlen = h.wrapping_add((*t).tun_hlen);
    0
}

unsafe fn update_pmtu(dev: *mut net_device, skb: *mut sk_buff, rt: *mut rtable,
    df: u16, inner: *const iphdr, tunnel_hlen: c_int, _destination: u32, md: bool) -> c_int {
    let t = priv_tunnel(dev);
    let hlen = if md { tunnel_hlen } else { (*t).hlen };
    let l2 = if (*dev).type_ == ARPHRD_ETHER as u16 { (*dev).hard_header_len as c_int } else { 0 };
    let packet_size = ((*skb).len as c_int).wrapping_sub(hlen).wrapping_sub(l2);
    let mtu = if df != 0 {
        (rust_ipt_dst_mtu(addr_of!((*rt).dst)) as c_int).wrapping_sub((size_of::<iphdr>() as c_int).wrapping_add(hlen)).wrapping_sub(l2)
    } else if rust_ipt_skb_valid_dst(skb) { rust_ipt_dst_mtu(rust_ipt_skb_dst(skb)) as c_int } else { (*dev).mtu as c_int };
    if rust_ipt_skb_valid_dst(skb) { rust_ipt_update_pmtu(skb, mtu as u32); }
    if rust_ipt_skb_protocol(skb) == be16(ETH_P_IP) {
        if !rust_ipt_skb_is_gso(skb) && (*inner).frag_off & be16(IP_DF) != 0 && mtu < packet_size {
            rust_ipt_icmp_send(skb, ICMP_DEST_UNREACH as c_int, ICMP_FRAG_NEEDED as c_int, be32(mtu as u32));
            return neg(E2BIG);
        }
    }
    #[cfg(any(CONFIG_IPV6, CONFIG_IPV6_MODULE))]
    if rust_ipt_skb_protocol(skb) == be16(ETH_P_IPV6) {
        let rt6 = if rust_ipt_skb_valid_dst(skb) { rust_ipt_dst_rt6(rust_ipt_skb_dst(skb)) } else { null_mut() };
        let daddr = if md { _destination } else { dst(addr_of!((*t).parms.iph)) };
        // dst_mtu returns unsigned int: retain the original usual conversion.
        if !rt6.is_null() && (mtu as u32) < rust_ipt_dst_mtu(rust_ipt_skb_dst(skb)) && mtu >= IPV6_MIN_MTU as c_int {
            if (daddr != 0 && !multicast(daddr)) || (*rt6).rt6i_dst.plen == 128 {
                (*rt6).rt6i_flags |= RTF_MODIFIED;
                rust_ipt_dst_metric_set(rust_ipt_skb_dst(skb), RTAX_MTU as c_int, mtu as u32);
            }
        }
        if !rust_ipt_skb_is_gso(skb) && mtu >= IPV6_MIN_MTU as c_int && mtu < packet_size {
            rust_ipt_icmpv6_send(skb, ICMPV6_PKT_TOOBIG as u8, 0, mtu as u32);
            return neg(E2BIG);
        }
    }
    0
}
unsafe fn tx_error(skb: *mut sk_buff, dev: *mut net_device) {
    rust_ipt_tx_errors(dev);
    rust_ipt_kfree_skb(skb);
}
unsafe fn tx_drop(skb: *mut sk_buff, dev: *mut net_device) {
    rust_ipt_tx_dropped(dev);
    rust_ipt_kfree_skb(skb);
}
#[no_mangle]
pub unsafe extern "C" fn ip_md_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device,
    mut proto: u8, mut tunnel_hlen: c_int) {
    let t = priv_tunnel(dev);
    let info = rust_ipt_tunnel_info(skb);
    if info.is_null() || (*info).mode & IP_TUNNEL_INFO_TX as u8 == 0 || rust_ipt_info_af(info) != AF_INET as u16 {
        tx_error(skb, dev); return;
    }
    let key = addr_of!((*info).key);
    rust_ipt_clear_ipcb_options(skb);
    let inner = rust_ipt_inner_network_header(skb).cast::<iphdr>();
    let mut tos = (*key).tos;
    if tos == 1 {
        if rust_ipt_skb_protocol(skb) == be16(ETH_P_IP) { tos = (*inner).tos; }
        else if rust_ipt_skb_protocol(skb) == be16(ETH_P_IPV6) { tos = rust_ipt_ipv6_dsfield(inner.cast()); }
    }
    let mut fl: flowi4 = zeroed();
    rust_ipt_init_flow(&mut fl, proto as c_int, (*key).u.ipv4.dst, (*key).u.ipv4.src,
        rust_ipt_id_to_key((*key).tun_id), tos & INET_DSCP_MASK as u8, (*t).net, 0,
        rust_ipt_skb_mark(skb), rust_ipt_skb_hash(skb), (*key).flow_flags);
    if tunnel_hlen == 0 { tunnel_hlen = rust_ipt_encap_hlen(addr_of_mut!((*info).encap)); }
    if rust_ipt_encap(skb, addr_of_mut!((*info).encap), &mut proto, &mut fl) < 0 { tx_error(skb, dev); return; }
    let use_cache = rust_ipt_cache_usable(skb, info);
    let mut rt = if use_cache { dst_cache_get_ip4(addr_of_mut!((*info).dst_cache), &mut fl.saddr) } else { null_mut() };
    if rt.is_null() {
        rt = rust_ipt_route_output((*t).net, &mut fl);
        if rust_ipt_is_err(rt.cast()) { rust_ipt_tx_carrier_errors(dev); tx_error(skb, dev); return; }
        if use_cache { dst_cache_set_ip4(addr_of_mut!((*info).dst_cache), addr_of_mut!((*rt).dst), fl.saddr); }
    }
    if rust_ipt_dst_dev(addr_of!((*rt).dst)) == dev {
        rust_ipt_rt_put(rt); rust_ipt_collisions(dev); tx_error(skb, dev); return;
    }
    let df = if rust_ipt_test_bit(IP_TUNNEL_DONT_FRAGMENT_BIT, (*key).tun_flags.as_ptr()) { be16(IP_DF) } else { 0 };
    if update_pmtu(dev, skb, rt, df, inner, tunnel_hlen, (*key).u.ipv4.dst, true) != 0 {
        rust_ipt_rt_put(rt); tx_error(skb, dev); return;
    }
    tos = rust_ipt_ecn_encap(tos, inner, skb);
    let mut ttl = (*key).ttl;
    if ttl == 0 {
        ttl = if rust_ipt_skb_protocol(skb) == be16(ETH_P_IP) { (*inner).ttl }
            else if rust_ipt_skb_protocol(skb) == be16(ETH_P_IPV6) { (*inner.cast::<ipv6hdr>()).hop_limit }
            else { rust_ipt_hoplimit(addr_of!((*rt).dst)) as u8 };
    }
    let headroom = (size_of::<iphdr>() as u32).wrapping_add(rust_ipt_ll_reserved(rust_ipt_dst_dev(addr_of!((*rt).dst)))).wrapping_add((*rt).dst.header_len as u32);
    if rust_ipt_cow_head(skb, headroom) != 0 { rust_ipt_rt_put(rt); tx_drop(skb, dev); return; }
    rust_ipt_adj_headroom(dev, headroom);
    iptunnel_xmit(null_mut(), rt, skb, fl.saddr, fl.daddr, proto, tos, ttl, df,
        !rust_ipt_net_eq((*t).net, rust_ipt_dev_net(dev)), 0);
}

#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device,
    params: *const iphdr, mut protocol: u8) {
    let t = priv_tunnel(dev);
    let mut info: *mut ip_tunnel_info = null_mut();
    let inner = rust_ipt_inner_network_header(skb).cast::<iphdr>();
    let mut connected = dst(addr_of!((*t).parms.iph)) != 0;
    let payload = rust_ipt_payload_protocol(skb);
    rust_ipt_clear_ipcb_options(skb);
    let mut destination = dst(params);
    let mut md = false;
    if destination == 0 {
        if rust_ipt_skb_dst(skb).is_null() { rust_ipt_tx_fifo_errors(dev); tx_error(skb, dev); return; }
        info = rust_ipt_tunnel_info(skb);
        if !info.is_null() && (*info).mode & IP_TUNNEL_INFO_TX as u8 != 0
            && rust_ipt_info_af(info) == AF_INET as u16 && (*info).key.u.ipv4.dst != 0 {
            destination = (*info).key.u.ipv4.dst;
            md = true;
            connected = true;
        } else if payload == be16(ETH_P_IP) {
            destination = rust_ipt_rt_nexthop(rust_ipt_skb_rtable(skb), dst(inner));
        } else {
            #[cfg(any(CONFIG_IPV6, CONFIG_IPV6_MODULE))]
            {
                if payload != be16(ETH_P_IPV6) { tx_error(skb, dev); return; }
                let neigh = rust_ipt_dst_neigh(rust_ipt_skb_dst(skb), rust_ipt_ipv6_daddr(skb).cast());
                if neigh.is_null() { tx_error(skb, dev); return; }
                let mut a = rust_ipt_neigh_key(neigh);
                let mut typ = rust_ipt_ipv6_addr_type(a);
                if typ == IPV6_ADDR_ANY as c_int { a = rust_ipt_ipv6_daddr(skb); typ = rust_ipt_ipv6_addr_type(a); }
                let error_icmp = typ & IPV6_ADDR_COMPATv4 as c_int == 0;
                if !error_icmp { destination = rust_ipt_ipv6_word3(a); }
                rust_ipt_neigh_release(neigh);
                if error_icmp { rust_ipt_dst_link_failure(skb); tx_error(skb, dev); return; }
            }
            #[cfg(not(any(CONFIG_IPV6, CONFIG_IPV6_MODULE)))]
            { tx_error(skb, dev); return; }
        }
        if !md { connected = false; }
    }
    let mut tos = (*params).tos;
    if tos & 1 != 0 {
        tos &= !1;
        if payload == be16(ETH_P_IP) { tos = (*inner).tos; connected = false; }
        else if payload == be16(ETH_P_IPV6) { tos = rust_ipt_ipv6_dsfield(inner.cast()); connected = false; }
    }
    let mut fl: flowi4 = zeroed();
    rust_ipt_init_flow(&mut fl, protocol as c_int, destination, src(params), (*t).parms.o_key,
        tos & INET_DSCP_MASK as u8, (*t).net, rust_ipt_read_int(addr_of!((*t).parms.link)),
        (*t).fwmark, rust_ipt_skb_hash(skb), 0);
    if rust_ipt_encap(skb, addr_of_mut!((*t).encap), &mut protocol, &mut fl) < 0 { tx_error(skb, dev); return; }
    let mut use_cache = false;
    let mut rt = null_mut();
    if connected && md {
        use_cache = rust_ipt_cache_usable(skb, info);
        if use_cache { rt = dst_cache_get_ip4(addr_of_mut!((*info).dst_cache), &mut fl.saddr); }
    } else if connected { rt = dst_cache_get_ip4(addr_of_mut!((*t).dst_cache), &mut fl.saddr); }
    if rt.is_null() {
        rt = rust_ipt_route_output((*t).net, &mut fl);
        if rust_ipt_is_err(rt.cast()) { rust_ipt_tx_carrier_errors(dev); tx_error(skb, dev); return; }
        if use_cache { dst_cache_set_ip4(addr_of_mut!((*info).dst_cache), addr_of_mut!((*rt).dst), fl.saddr); }
        else if !md && connected { dst_cache_set_ip4(addr_of_mut!((*t).dst_cache), addr_of_mut!((*rt).dst), fl.saddr); }
    }
    if rust_ipt_dst_dev(addr_of!((*rt).dst)) == dev { rust_ipt_rt_put(rt); rust_ipt_collisions(dev); tx_error(skb, dev); return; }
    let mut df = (*params).frag_off;
    if payload == be16(ETH_P_IP) && !(*t).ignore_df { df |= (*inner).frag_off & be16(IP_DF); }
    if update_pmtu(dev, skb, rt, df, inner, 0, 0, false) != 0 { rust_ipt_rt_put(rt); tx_error(skb, dev); return; }
    let errors = rust_ipt_read_int(addr_of!((*t).err_count));
    if errors > 0 {
        let expires = rust_ipt_read_ulong(addr_of!((*t).err_time)).wrapping_add(IPTUNNEL_ERR_TIMEO as c_ulong);
        if (rust_ipt_jiffies().wrapping_sub(expires) as kernel::ffi::c_long) < 0 {
            rust_ipt_write_int(addr_of_mut!((*t).err_count), errors.wrapping_sub(1));
            rust_ipt_dst_link_failure(skb);
        } else { rust_ipt_write_int(addr_of_mut!((*t).err_count), 0); }
    }
    tos = rust_ipt_ecn_encap(tos, inner, skb);
    let mut ttl = (*params).ttl;
    if ttl == 0 {
        if payload == be16(ETH_P_IP) { ttl = (*inner).ttl; }
        else {
            #[cfg(any(CONFIG_IPV6, CONFIG_IPV6_MODULE))]
            if payload == be16(ETH_P_IPV6) { ttl = (*inner.cast::<ipv6hdr>()).hop_limit; }
            else { ttl = rust_ipt_hoplimit(addr_of!((*rt).dst)) as u8; }
            #[cfg(not(any(CONFIG_IPV6, CONFIG_IPV6_MODULE)))]
            { ttl = rust_ipt_hoplimit(addr_of!((*rt).dst)) as u8; }
        }
    }
    let headroom = rust_ipt_ll_reserved(rust_ipt_dst_dev(addr_of!((*rt).dst)))
        .wrapping_add(size_of::<iphdr>() as u32).wrapping_add((*rt).dst.header_len as u32)
        .wrapping_add(rust_ipt_encap_hlen(addr_of_mut!((*t).encap)) as u32);
    if rust_ipt_cow_head(skb, headroom) != 0 { rust_ipt_rt_put(rt); tx_drop(skb, dev); return; }
    rust_ipt_adj_headroom(dev, headroom);
    iptunnel_xmit(null_mut(), rt, skb, fl.saddr, fl.daddr, protocol, tos, ttl, df,
        !rust_ipt_net_eq((*t).net, rust_ipt_dev_net(dev)), 0);
}

unsafe fn update(itn: *mut ip_tunnel_net, t: *mut ip_tunnel, dev: *mut net_device,
    p: *const ip_tunnel_parm_kern, set_mtu: bool, fwmark: u32) {
    del(itn, t);
    (*t).parms.iph.__bindgen_anon_1.addrs.saddr = src(addr_of!((*p).iph));
    (*t).parms.iph.__bindgen_anon_1.addrs.daddr = dst(addr_of!((*p).iph));
    (*t).parms.i_key = (*p).i_key;
    (*t).parms.o_key = (*p).o_key;
    if (*dev).type_ != ARPHRD_ETHER as u16 {
        rust_ipt_dev_addr_set(dev, addr_of!((*p).iph.__bindgen_anon_1.addrs.saddr).cast(), 4);
        copy_nonoverlapping(addr_of!((*p).iph.__bindgen_anon_1.addrs.daddr).cast::<u8>(), (*dev).broadcast.as_mut_ptr(), 4);
    }
    add(itn, t);
    (*t).parms.iph.ttl = (*p).iph.ttl;
    (*t).parms.iph.tos = (*p).iph.tos;
    (*t).parms.iph.frag_off = (*p).iph.frag_off;
    if (*t).parms.link != (*p).link || (*t).fwmark != fwmark {
        rust_ipt_write_int(addr_of_mut!((*t).parms.link), (*p).link);
        (*t).fwmark = fwmark;
        let mtu = bind_dev(dev);
        if set_mtu { rust_ipt_write_uint(addr_of_mut!((*dev).mtu), mtu as u32); }
    }
    rust_ipt_cache_reset(addr_of_mut!((*t).dst_cache));
    netdev_state_change(dev);
}
unsafe fn incompatible_flags(dev: *const net_device, p: *const ip_tunnel_parm_kern) -> bool {
    let d = dst(addr_of!((*p).iph));
    let flags = if multicast(d) { IFF_BROADCAST } else if d != 0 { IFF_POINTOPOINT } else { 0 };
    ((*dev).flags ^ flags) & (IFF_POINTOPOINT | IFF_BROADCAST) != 0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_ctl(mut dev: *mut net_device, p: *mut ip_tunnel_parm_kern, cmd: c_int) -> c_int {
    let mut t = priv_tunnel(dev);
    let n = (*t).net;
    let itn = tunnel_net(n, (*t).ip_tnl_net_id);
    match cmd as u32 {
        SIOCGETTUNNEL => {
            if dev == (*itn).fb_tunnel_dev {
                t = find(itn, p, (*(*itn).fb_tunnel_dev).type_ as c_int);
                if t.is_null() { t = priv_tunnel(dev); }
            }
            copy_nonoverlapping(addr_of!((*t).parms), p, 1);
            0
        }
        SIOCADDTUNNEL | SIOCCHGTUNNEL => {
            if !ns_capable((*n).user_ns, CAP_NET_ADMIN as c_int) { return neg(EPERM); }
            if (*p).iph.ttl != 0 { (*p).iph.frag_off |= be16(IP_DF); }
            if !rust_ipt_test_bit(IP_TUNNEL_VTI_BIT, (*p).i_flags.as_ptr()) {
                if !rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags.as_ptr()) { (*p).i_key = 0; }
                if !rust_ipt_test_bit(IP_TUNNEL_KEY_BIT, (*p).o_flags.as_ptr()) { (*p).o_key = 0; }
            }
            t = find(itn, p, (*itn).type_);
            if cmd as u32 == SIOCADDTUNNEL {
                return if t.is_null() { ptr_err_or_zero(create(n, itn, p)) } else { neg(EEXIST) };
            }
            if dev != (*itn).fb_tunnel_dev && cmd as u32 == SIOCCHGTUNNEL {
                if !t.is_null() {
                    if (*t).dev != dev { return neg(EEXIST); }
                } else {
                    if incompatible_flags(dev, p) { return neg(EINVAL); }
                    t = priv_tunnel(dev);
                }
            }
            if t.is_null() { return neg(ENOENT); }
            update(itn, t, dev, p, true, 0);
            0
        }
        SIOCDELTUNNEL => {
            if !ns_capable((*n).user_ns, CAP_NET_ADMIN as c_int) { return neg(EPERM); }
            if dev == (*itn).fb_tunnel_dev {
                t = find(itn, p, (*(*itn).fb_tunnel_dev).type_ as c_int);
                if t.is_null() { return neg(ENOENT); }
                if t == priv_tunnel((*itn).fb_tunnel_dev) { return neg(EPERM); }
                dev = (*t).dev;
            }
            rust_ipt_unregister_netdevice(dev);
            0
        }
        _ => neg(EINVAL),
    }
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_parm_from_user(kp: *mut ip_tunnel_parm_kern, data: *const c_void) -> bool {
    let mut p: ip_tunnel_parm = zeroed();
    if rust_ipt_copy_from_user(addr_of_mut!(p).cast(), data, size_of::<ip_tunnel_parm>() as c_ulong) != 0 { return false; }
    rust_ipt_strscpy((*kp).name.as_mut_ptr(), p.name.as_ptr(), IFNAMSIZ as usize);
    (*kp).link = p.link;
    rust_ipt_flags_from_be16((*kp).i_flags.as_mut_ptr(), p.i_flags);
    rust_ipt_flags_from_be16((*kp).o_flags.as_mut_ptr(), p.o_flags);
    (*kp).i_key = p.i_key;
    (*kp).o_key = p.o_key;
    copy_nonoverlapping(addr_of!(p.iph), addr_of_mut!((*kp).iph), 1);
    true
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_parm_to_user(data: *mut c_void, kp: *mut ip_tunnel_parm_kern) -> bool {
    if !rust_ipt_flags_compat((*kp).i_flags.as_ptr()) || !rust_ipt_flags_compat((*kp).o_flags.as_ptr()) { return false; }
    let mut p: ip_tunnel_parm = zeroed();
    rust_ipt_strscpy(p.name.as_mut_ptr(), (*kp).name.as_ptr(), IFNAMSIZ as usize);
    p.link = (*kp).link;
    p.i_flags = rust_ipt_flags_to_be16((*kp).i_flags.as_ptr());
    p.o_flags = rust_ipt_flags_to_be16((*kp).o_flags.as_ptr());
    p.i_key = (*kp).i_key;
    p.o_key = (*kp).o_key;
    copy_nonoverlapping(addr_of!((*kp).iph), addr_of_mut!(p.iph), 1);
    rust_ipt_copy_to_user(data, addr_of!(p).cast(), size_of::<ip_tunnel_parm>() as c_ulong) == 0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_siocdevprivate(dev: *mut net_device, _ifr: *mut ifreq, data: *mut c_void, cmd: c_int) -> c_int {
    let mut p: ip_tunnel_parm_kern = zeroed();
    if !ip_tunnel_parm_from_user(&mut p, data) { return neg(EFAULT); }
    let e = ((*(*dev).netdev_ops).ndo_tunnel_ctl.unwrap_unchecked())(dev, &mut p, cmd);
    if e == 0 && !ip_tunnel_parm_to_user(data, &mut p) { return neg(EFAULT); }
    e
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    let t = priv_tunnel(dev);
    let th = (*t).hlen.wrapping_add(size_of::<iphdr>() as c_int);
    let mut max = (IP_MAX_MTU as c_int).wrapping_sub(th);
    if (*dev).type_ == ARPHRD_ETHER as u16 { max = max.wrapping_sub((*dev).hard_header_len as c_int); }
    if new_mtu < ETH_MIN_MTU as c_int || new_mtu > max { return neg(EINVAL); }
    rust_ipt_write_uint(addr_of_mut!((*dev).mtu), new_mtu as u32);
    0
}
unsafe extern "C" fn dev_free(dev: *mut net_device) {
    let t = priv_tunnel(dev);
    gro_cells_destroy(addr_of_mut!((*t).gro_cells));
    dst_cache_destroy(addr_of_mut!((*t).dst_cache));
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_dellink(dev: *mut net_device, head: *mut list_head) {
    let t = priv_tunnel(dev);
    let itn = tunnel_net((*t).net, (*t).ip_tnl_net_id);
    if (*itn).fb_tunnel_dev != dev { del(itn, priv_tunnel(dev)); unregister_netdevice_queue(dev, head); }
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_get_link_net(dev: *const net_device) -> *mut net {
    rust_ipt_read_net(addr_of!((*priv_tunnel(dev)).net))
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_get_iflink(dev: *const net_device) -> c_int {
    rust_ipt_read_int(addr_of!((*priv_tunnel(dev)).parms.link))
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_init_net(n: *mut net, id: u32, ops: *mut rtnl_link_ops, devname: *mut c_char) -> c_int {
    let itn = tunnel_net(n, id);
    (*itn).rtnl_link_ops = ops;
    for h in &mut (*itn).tunnels { h.first = null_mut(); }
    if ops.is_null() || !rust_ipt_has_fallback(n) {
        (*itn).type_ = (*tunnel_net(addr_of_mut!(init_net), id)).type_;
        (*itn).fb_tunnel_dev = null_mut();
        return 0;
    }
    let mut p: ip_tunnel_parm_kern = zeroed();
    if !devname.is_null() { rust_ipt_strscpy(p.name.as_mut_ptr(), devname, IFNAMSIZ as usize); }
    rtnl_lock();
    (*itn).fb_tunnel_dev = create_dev(n, ops, &p);
    let dev = (*itn).fb_tunnel_dev;
    if !rust_ipt_is_err(dev.cast()) {
        rust_ipt_set_netns_immutable(dev);
        (*dev).mtu = bind_dev(dev) as u32;
        add(itn, priv_tunnel(dev));
        (*itn).type_ = (*dev).type_ as c_int;
    }
    rtnl_unlock();
    ptr_err_or_zero((*itn).fb_tunnel_dev)
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_delete_net(n: *mut net, id: u32, ops: *mut rtnl_link_ops, head: *mut list_head) {
    let itn = tunnel_net(n, id);
    rust_ipt_assert_rtnl_net(n);
    let mut dev = rust_ipt_first_netdev(n);
    while !dev.is_null() {
        let next = rust_ipt_next_netdev(dev);
        if (*dev).rtnl_link_ops == ops { unregister_netdevice_queue(dev, head); }
        dev = next;
    }
    for h in &(*itn).tunnels {
        let mut node = h.first;
        while !node.is_null() {
            let next = (*node).next;
            let t = node_tunnel(node);
            if !rust_ipt_net_eq(rust_ipt_dev_net((*t).dev), n) { unregister_netdevice_queue((*t).dev, head); }
            node = next;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_newlink(n: *mut net, dev: *mut net_device,
    tb: *mut *mut nlattr, p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int {
    let t = priv_tunnel(dev);
    let itn = tunnel_net(n, (*t).ip_tnl_net_id);
    if (*t).collect_md {
        if !rust_ipt_rtnl_read(addr_of!((*itn).collect_md_tun)).is_null() { return neg(EEXIST); }
    } else if !find(itn, p, (*dev).type_ as c_int).is_null() { return neg(EEXIST); }
    (*t).net = n;
    (*t).parms = *p;
    (*t).fwmark = fwmark;
    let e = register_netdevice(dev);
    if e != 0 { return e; }
    if (*dev).type_ == ARPHRD_ETHER as u16 && (*tb.add(IFLA_ADDRESS as usize)).is_null() { rust_ipt_hw_addr_random(dev); }
    let mut mtu = bind_dev(dev);
    if !(*tb.add(IFLA_MTU as usize)).is_null() {
        let mut max = IP_MAX_MTU.wrapping_sub((*t).hlen.wrapping_add(size_of::<iphdr>() as c_int) as u32);
        if (*dev).type_ == ARPHRD_ETHER as u16 { max = max.wrapping_sub((*dev).hard_header_len as u32); }
        mtu = core::cmp::min(core::cmp::max((*dev).mtu, ETH_MIN_MTU), max) as c_int;
    }
    let e = dev_set_mtu(dev, mtu);
    if e != 0 { rust_ipt_unregister_netdevice(dev); return e; }
    add(itn, t);
    0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_changelink(dev: *mut net_device, tb: *mut *mut nlattr,
    p: *mut ip_tunnel_parm_kern, fwmark: u32) -> c_int {
    let tunnel = priv_tunnel(dev);
    let itn = tunnel_net((*tunnel).net, (*tunnel).ip_tnl_net_id);
    if dev == (*itn).fb_tunnel_dev { return neg(EINVAL); }
    let mut t = find(itn, p, (*dev).type_ as c_int);
    if !t.is_null() { if (*t).dev != dev { return neg(EEXIST); } }
    else {
        t = tunnel;
        if (*dev).type_ != ARPHRD_ETHER as u16 && incompatible_flags(dev, p) { return neg(EINVAL); }
    }
    update(itn, t, dev, p, (*tb.add(IFLA_MTU as usize)).is_null(), fwmark);
    0
}
#[no_mangle]
pub unsafe extern "C" fn __ip_tunnel_init(dev: *mut net_device) -> c_int {
    let t = priv_tunnel(dev);
    (*dev).needs_free_netdev = true;
    (*dev).priv_destructor = Some(dev_free);
    rust_ipt_set_pcpu_stat_type(dev);
    let e = dst_cache_init(addr_of_mut!((*t).dst_cache), RUST_IPT_GFP_KERNEL);
    if e != 0 { return e; }
    let e = gro_cells_init(addr_of_mut!((*t).gro_cells), dev);
    if e != 0 { dst_cache_destroy(addr_of_mut!((*t).dst_cache)); return e; }
    (*t).dev = dev;
    rust_ipt_strscpy((*t).parms.name.as_mut_ptr(), (*dev).name.as_ptr(), IFNAMSIZ as usize);
    (*t).parms.iph.set_version(4);
    (*t).parms.iph.set_ihl(5);
    if (*t).collect_md { rust_ipt_keep_dst(dev); }
    0
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_uninit(dev: *mut net_device) {
    let t = priv_tunnel(dev);
    let itn = tunnel_net((*t).net, (*t).ip_tnl_net_id);
    del(itn, priv_tunnel(dev));
    if (*itn).fb_tunnel_dev == dev { rust_ipt_write_dev(addr_of_mut!((*itn).fb_tunnel_dev), null_mut()); }
    rust_ipt_cache_reset(addr_of_mut!((*t).dst_cache));
}
#[no_mangle]
pub unsafe extern "C" fn ip_tunnel_setup(dev: *mut net_device, net_id: u32) {
    (*priv_tunnel(dev)).ip_tnl_net_id = net_id;
}
