// SPDX-License-Identifier: GPL-2.0-only
// Translated from ip6_udp_tunnel.c. Kernel dependencies are supplied externally.

pub unsafe fn udp_sock_create6(
    net: *mut net,
    cfg: *mut udp_port_cfg,
    sockp: *mut *mut socket,
) -> i32 {
    let mut udp6_addr: sockaddr_in6 = core::mem::zeroed();
    let mut err: i32;
    let mut sock: *mut socket = core::ptr::null_mut();

    err = sock_create_kern(net, AF_INET6, SOCK_DGRAM, 0, &mut sock);
    if err < 0 {
        goto_error(err, sock, sockp);
        return err;
    }

    if (*cfg).ipv6_v6only {
        err = ip6_sock_set_v6only((*sock).sk);
        if err < 0 {
            goto_error(err, sock, sockp);
            return err;
        }
    }
    if (*cfg).bind_ifindex != 0 {
        err = sock_bindtoindex((*sock).sk, (*cfg).bind_ifindex, true);
        if err < 0 {
            goto_error(err, sock, sockp);
            return err;
        }
    }

    udp6_addr.sin6_family = AF_INET6;
    core::ptr::copy_nonoverlapping(
        &(*cfg).local_ip6 as *const _ as *const u8,
        &mut udp6_addr.sin6_addr as *mut _ as *mut u8,
        core::mem::size_of_val(&udp6_addr.sin6_addr),
    );
    udp6_addr.sin6_port = (*cfg).local_udp_port;
    err = kernel_bind(sock, &mut udp6_addr as *mut _ as *mut sockaddr_unsized,
                      core::mem::size_of::<sockaddr_in6>());
    if err < 0 {
        goto_error(err, sock, sockp);
        return err;
    }

    if (*cfg).peer_udp_port != 0 {
        core::ptr::write_bytes(&mut udp6_addr as *mut _, 0, 1);
        udp6_addr.sin6_family = AF_INET6;
        core::ptr::copy_nonoverlapping(
            &(*cfg).peer_ip6 as *const _ as *const u8,
            &mut udp6_addr.sin6_addr as *mut _ as *mut u8,
            core::mem::size_of_val(&udp6_addr.sin6_addr),
        );
        udp6_addr.sin6_port = (*cfg).peer_udp_port;
        err = kernel_connect(sock, &mut udp6_addr as *mut _ as *mut sockaddr_unsized,
                             core::mem::size_of::<sockaddr_in6>(), 0);
    }
    if err < 0 {
        goto_error(err, sock, sockp);
        return err;
    }

    udp_set_no_check6_tx((*sock).sk, !(*cfg).use_udp6_tx_checksums);
    udp_set_no_check6_rx((*sock).sk, !(*cfg).use_udp6_rx_checksums);
    *sockp = sock;
    0
}

unsafe fn goto_error(err: i32, sock: *mut socket, sockp: *mut *mut socket) {
    if !sock.is_null() {
        kernel_sock_shutdown(sock, SHUT_RDWR);
        sock_release(sock);
    }
    *sockp = core::ptr::null_mut();
    let _ = err;
}

pub unsafe fn udp_tunnel6_xmit_skb(
    dst: *mut dst_entry, sk: *mut sock, skb: *mut sk_buff, dev: *mut net_device,
    saddr: *const in6_addr, daddr: *const in6_addr, prio: u8, ttl: u8,
    label: __be32, src_port: __be16, dst_port: __be16, nocheck: bool,
    ip6cb_flags: u16,
) {
    __skb_push(skb, core::mem::size_of::<udphdr>());
    skb_reset_transport_header(skb);
    let uh = udp_hdr(skb);
    (*uh).dest = dst_port;
    (*uh).source = src_port;
    udp_set_len(uh, (*skb).len);
    skb_dst_set(skb, dst);
    udp6_set_csum(nocheck, skb, saddr, daddr, (*skb).len);

    __skb_push(skb, core::mem::size_of::<ipv6hdr>());
    skb_reset_network_header(skb);
    let ip6h = ipv6_hdr(skb);
    ip6_flow_hdr(ip6h, prio, label);
    (*ip6h).payload_len = htons((*skb).len);
    (*ip6h).nexthdr = IPPROTO_UDP;
    (*ip6h).hop_limit = ttl;
    (*ip6h).daddr = *daddr;
    (*ip6h).saddr = *saddr;
    ip6tunnel_xmit(sk, skb, dev, ip6cb_flags);
}

pub unsafe fn udp_tunnel6_dst_lookup(
    skb: *mut sk_buff, dev: *mut net_device, net: *mut net, sk: *mut sock,
    oif: i32, saddr: *mut in6_addr, key: *const ip_tunnel_key,
    sport: __be16, dport: __be16, dsfield: u8, dst_cache: *mut dst_cache,
) -> *mut dst_entry {
    let mut dst: *mut dst_entry = core::ptr::null_mut();
    let mut fl6: flowi6 = core::mem::zeroed();

    // CONFIG_DST_CACHE: preserve the conditional cache lookup when enabled.
    #[cfg(CONFIG_DST_CACHE)]
    if !dst_cache.is_null() {
        dst = dst_cache_get_ip6(dst_cache, saddr);
        if !dst.is_null() { return dst; }
    }
    core::ptr::write_bytes(&mut fl6 as *mut _, 0, 1);
    fl6.flowi6_mark = (*skb).mark;
    fl6.flowi6_proto = IPPROTO_UDP;
    fl6.flowi6_oif = oif;
    fl6.daddr = (*key).u.ipv6.dst;
    fl6.saddr = (*key).u.ipv6.src;
    fl6.fl6_sport = sport;
    fl6.fl6_dport = dport;
    fl6.flowlabel = ip6_make_flowinfo(dsfield, (*key).label);
    dst = ip6_dst_lookup_flow(net, sk, &mut fl6, core::ptr::null_mut());
    if IS_ERR(dst) { return ERR_PTR(-ENETUNREACH); }
    if dst_dev(dst) == dev {
        dst_release(dst);
        return ERR_PTR(-ELOOP);
    }
    #[cfg(CONFIG_DST_CACHE)]
    if !dst_cache.is_null() { dst_cache_set_ip6(dst_cache, dst, &fl6.saddr); }
    *saddr = fl6.saddr;
    dst
}

// MODULE_DESCRIPTION("IPv6 Foo over UDP tunnel driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
