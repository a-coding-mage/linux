// SPDX-License-Identifier: GPL-2.0
/*
 * xfrm6_input.c: based on net/ipv4/xfrm4_input.c
 *
 * Authors:
 *	Mitsuru KANDA @USAGI
 *	Kazunori MIYAZAWA @USAGI
 *	Kunihiro Ishiguro <kunihiro@ipinfusion.com>
 *	YOSHIFUJI Hideaki @USAGI
 *		IPv6 support
 */

// Kernel headers and configuration-provided declarations are external dependencies.

pub unsafe fn xfrm6_rcv_spi(
    skb: *mut sk_buff,
    nexthdr: i32,
    spi: __be32,
    t: *mut ip6_tnl,
) -> i32 {
    xfrm_tunnel_skb_cb(skb).tunnel.ip6 = t;
    xfrm_spi_skb_cb(skb).family = AF_INET6;
    xfrm_spi_skb_cb(skb).daddroff = core::mem::offset_of!(ipv6hdr, daddr);
    xfrm_input(skb, nexthdr, spi, 0)
}

unsafe fn xfrm6_transport_finish2(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    if xfrm_trans_queue(skb, Some(ip6_rcv_finish)) != 0 {
        kfree_skb(skb);
        return NET_RX_DROP;
    }
    0
}

pub unsafe fn xfrm6_transport_finish(skb: *mut sk_buff, async_: i32) -> i32 {
    let xo = xfrm_offload(skb);
    let dev = (*skb).dev;
    let nhlen = -skb_network_offset(skb);

    *skb_network_header(skb).add((*ip6cb(skb)).nhoff as usize) =
        xfrm_mode_skb_cb(skb).protocol;

    // CONFIG_NETFILTER-disabled builds return here for synchronous processing.
    #[cfg(not(feature = "CONFIG_NETFILTER"))]
    if async_ == 0 { return 1; }

    __skb_push(skb, nhlen as usize);
    (*ipv6_hdr(skb)).payload_len = htons((*skb).len.wrapping_sub(core::mem::size_of::<ipv6hdr>()) as u16);
    skb_postpush_rcsum(skb, skb_network_header(skb), nhlen as usize);

    if !xo.is_null() && ((*xo).flags & XFRM_GRO) != 0 {
        skb_mac_header_rebuild_full(skb, (*xo).orig_mac_len);
        skb_reset_network_header(skb);
        skb_reset_transport_header(skb);
        return 0;
    }

    nf_hook(
        NFPROTO_IPV6, NF_INET_PRE_ROUTING, dev_net(dev), core::ptr::null_mut(),
        skb, dev, core::ptr::null_mut(), Some(xfrm6_transport_finish2),
    );
    0
}

unsafe fn __xfrm6_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff, pull: bool) -> i32 {
    let up = udp_sk(sk);
    let mut uh: *mut udphdr;
    let ip6h: *mut ipv6hdr;
    let mut len = (*skb).len.wrapping_sub(core::mem::size_of::<udphdr>());
    let ip6hlen = core::mem::size_of::<ipv6hdr>();
    let udpdata: *mut u8;
    let udpdata32: *mut __be32;
    let encap_type = read_once(&(*up).encap_type);

    if encap_type == 0 { return 1; }
    if pskb_may_pull(skb, core::mem::size_of::<udphdr>() + core::cmp::min(len, 8)) == 0 { return 1; }

    uh = udp_hdr(skb);
    udpdata = (uh as *mut u8).add(core::mem::size_of::<udphdr>());
    udpdata32 = udpdata as *mut __be32;

    match encap_type {
        UDP_ENCAP_ESPINUDP => {
            if len == 1 && *udpdata == 0xff { return -EINVAL; }
            else if len > core::mem::size_of::<ip_esp_hdr>() && *udpdata32 != 0 { len = core::mem::size_of::<udphdr>(); }
            else { return 1; }
        }
        _ => {
            if len == 1 && *udpdata == 0xff { return -EINVAL; }
            else if len > core::mem::size_of::<ip_esp_hdr>() && *udpdata32 != 0 { len = core::mem::size_of::<udphdr>(); }
            else { return 1; }
        }
    }

    if skb_unclone(skb, GFP_ATOMIC) != 0 { return -EINVAL; }
    ip6h = ipv6_hdr(skb);
    (*ip6h).payload_len = htons(ntohs((*ip6h).payload_len).wrapping_sub(len as u16));
    if (*skb).len < ip6hlen + len { return -EINVAL; }
    if pull {
        __skb_pull(skb, len);
        skb_reset_transport_header(skb);
    } else { skb_set_transport_header(skb, len); }
    0
}

pub unsafe fn xfrm6_udp_encap_rcv(sk: *mut sock, skb: *mut sk_buff) -> i32 {
    if (*skb).protocol == htons(ETH_P_IP) { return xfrm4_udp_encap_rcv(sk, skb); }
    let ret = __xfrm6_udp_encap_rcv(sk, skb, true);
    if ret == 0 { return xfrm6_rcv_encap(skb, IPPROTO_ESP, 0, (*udp_sk(sk)).encap_type); }
    if ret < 0 { kfree_skb(skb); return 0; }
    ret
}

pub unsafe fn xfrm6_gro_udp_encap_rcv(sk: *mut sock, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let offset = skb_gro_offset(skb);
    let len = (*skb).len - offset;
    let dlen = offset + core::cmp::min(len, 8);
    if (*skb).protocol == htons(ETH_P_IP) { return xfrm4_gro_udp_encap_rcv(sk, head, skb); }
    let udpdata = skb_gro_header(skb, dlen, offset);
    if udpdata.is_null() { return core::ptr::null_mut(); }
    rcu_read_lock();
    let ops = rcu_dereference(inet6_offloads[IPPROTO_ESP as usize]);
    if ops.is_null() || (*ops).callbacks.gro_receive.is_none() || len <= core::mem::size_of::<ip_esp_hdr>() || *(udpdata as *mut __be32) == 0 {
        rcu_read_unlock(); napi_gro_cb(skb).same_flow = 0; napi_gro_cb(skb).flush = 1; return core::ptr::null_mut();
    }
    skb_set_transport_header(skb, offset);
    napi_gro_cb(skb).proto = IPPROTO_UDP;
    let pp = call_gro_receive((*ops).callbacks.gro_receive, head, skb);
    rcu_read_unlock(); pp
}

pub unsafe fn xfrm6_rcv_tnl(skb: *mut sk_buff, t: *mut ip6_tnl) -> i32 {
    xfrm6_rcv_spi(skb, *skb_network_header(skb).add((*ip6cb(skb)).nhoff as usize) as i32, 0, t)
}

pub unsafe fn xfrm6_rcv(skb: *mut sk_buff) -> i32 { xfrm6_rcv_tnl(skb, core::ptr::null_mut()) }

pub unsafe fn xfrm6_input_addr(skb: *mut sk_buff, daddr: *mut xfrm_address_t, saddr: *mut xfrm_address_t, proto: u8) -> i32 {
    let net = dev_net((*skb).dev);
    let mut x: *mut xfrm_state = core::ptr::null_mut();
    let sp = secpath_set(skb);
    if sp.is_null() { xfrm_inc_stats(net, LINUX_MIB_XFRMINERROR); return -1; }
    if (*sp).len >= XFRM_MAX_DEPTH { xfrm_inc_stats(net, LINUX_MIB_XFRMINBUFFERERROR); return -1; }
    for i in 0..3 {
        let (dst, src) = match i { 0 => (daddr, saddr), 1 => (daddr, &in6addr_any as *const _ as *mut _), _ => (&in6addr_any as *const _ as *mut _, &in6addr_any as *const _ as *mut _) };
        x = xfrm_state_lookup_byaddr(net, (*skb).mark, dst, src, proto, AF_INET6);
        if x.is_null() { continue; }
        if (*x).dir != 0 && (*x).dir != XFRM_SA_DIR_IN { xfrm_inc_stats(net, LINUX_MIB_XFRMINSTATEDIRERROR); xfrm_state_put(x); x = core::ptr::null_mut(); continue; }
        spin_lock(&mut (*x).lock);
        if (i == 0 || ((*x).props.flags & XFRM_STATE_WILDRECV) != 0) && (*x).km.state == XFRM_STATE_VALID && xfrm_state_check_expire(x) == 0 {
            spin_unlock(&mut (*x).lock);
            if ((*(*x).type_).input)(x, skb) > 0 { break; }
        } else { spin_unlock(&mut (*x).lock); }
        xfrm_state_put(x); x = core::ptr::null_mut();
    }
    if x.is_null() { xfrm_inc_stats(net, LINUX_MIB_XFRMINNOSTATES); xfrm_audit_state_notfound_simple(skb, AF_INET6); return -1; }
    (*sp).xvec[(*sp).len] = x; (*sp).len += 1;
    spin_lock(&mut (*x).lock); (*x).curlft.bytes += (*skb).len as u64; (*x).curlft.packets += 1; spin_unlock(&mut (*x).lock); 1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
