// SPDX-License-Identifier: GPL-2.0-only
// Kernel headers and external symbols supplied by the surrounding build are intentionally not reimplemented here.

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
pub unsafe fn udp_sock_create4(
    net: *mut net,
    cfg: *mut udp_port_cfg,
    sockp: *mut *mut socket,
) -> i32 {
    let mut err: i32;
    let mut sock: *mut socket = core::ptr::null_mut();
    let mut udp_addr: sockaddr_in = core::mem::zeroed();

    err = sock_create_kern(net, AF_INET, SOCK_DGRAM, 0, &mut sock);
    if err < 0 {
        return udp_sock_create4_error(err, sock, sockp);
    }

    if (*cfg).bind_ifindex != 0 {
        err = sock_bindtoindex((*sock).sk, (*cfg).bind_ifindex, true);
        if err < 0 {
            return udp_sock_create4_error(err, sock, sockp);
        }
    }

    udp_addr.sin_family = AF_INET as _;
    udp_addr.sin_addr = (*cfg).local_ip;
    udp_addr.sin_port = (*cfg).local_udp_port;
    err = kernel_bind(sock, &mut udp_addr as *mut _ as *mut sockaddr_unsized,
                      core::mem::size_of::<sockaddr_in>());
    if err < 0 {
        return udp_sock_create4_error(err, sock, sockp);
    }

    if (*cfg).peer_udp_port != 0 {
        udp_addr.sin_family = AF_INET as _;
        udp_addr.sin_addr = (*cfg).peer_ip;
        udp_addr.sin_port = (*cfg).peer_udp_port;
        err = kernel_connect(sock, &mut udp_addr as *mut _ as *mut sockaddr_unsized,
                             core::mem::size_of::<sockaddr_in>(), 0);
        if err < 0 {
            return udp_sock_create4_error(err, sock, sockp);
        }
    }

    (*sock).sk.sk_no_check_tx = !(*cfg).use_udp_checksums;
    *sockp = sock;
    0
}

unsafe fn udp_sock_create4_error(err: i32, sock: *mut socket, sockp: *mut *mut socket) -> i32 {
    if !sock.is_null() {
        kernel_sock_shutdown(sock, SHUT_RDWR);
        sock_release(sock);
    }
    *sockp = core::ptr::null_mut();
    err
}

unsafe fn sk_saddr_any(sk: *mut sock) -> bool {
    // CONFIG_IPV6 conditional preserved from the C source.
    #[cfg(feature = "CONFIG_IPV6")]
    {
        ipv6_addr_any(&(*sk).sk_v6_rcv_saddr)
    }
    #[cfg(not(feature = "CONFIG_IPV6"))]
    {
        (*sk).sk_rcv_saddr == 0
    }
}

pub unsafe fn setup_udp_tunnel_sock(net: *mut net, sk: *mut sock,
                                    cfg: *mut udp_tunnel_sock_cfg) {
    // Disable multicast loopback
    inet_clear_bit(MC_LOOP, sk);

    // Enable CHECKSUM_UNNECESSARY to CHECKSUM_COMPLETE conversion
    inet_inc_convert_csum(sk);

    rcu_assign_sk_user_data(sk, (*cfg).sk_user_data);

    (*udp_sk(sk)).encap_type = (*cfg).encap_type;
    (*udp_sk(sk)).encap_rcv = (*cfg).encap_rcv;
    (*udp_sk(sk)).encap_err_rcv = (*cfg).encap_err_rcv;
    (*udp_sk(sk)).encap_err_lookup = (*cfg).encap_err_lookup;
    (*udp_sk(sk)).encap_destroy = (*cfg).encap_destroy;
    (*udp_sk(sk)).gro_receive = (*cfg).gro_receive;
    (*udp_sk(sk)).gro_complete = (*cfg).gro_complete;

    udp_tunnel_encap_enable(sk);
    udp_tunnel_update_gro_rcv(sk, true);

    if (*sk).sk_dport == 0 && (*sk).sk_bound_dev_if == 0 && sk_saddr_any(sk) && (*sk).sk_kern_sock {
        udp_tunnel_update_gro_lookup(net, sk, true);
    }
}

pub unsafe fn udp_tunnel_push_rx_port(dev: *mut net_device, sk: *mut sock, type_: u16) {
    let mut ti: udp_tunnel_info = core::mem::zeroed();
    ti.type_ = type_;
    ti.sa_family = (*sk).sk_family;
    ti.port = (*inet_sk(sk)).inet_sport;
    udp_tunnel_nic_add_port(dev, &mut ti);
}

pub unsafe fn udp_tunnel_drop_rx_port(dev: *mut net_device, sk: *mut sock, type_: u16) {
    let mut ti: udp_tunnel_info = core::mem::zeroed();
    ti.type_ = type_;
    ti.sa_family = (*sk).sk_family;
    ti.port = (*inet_sk(sk)).inet_sport;
    udp_tunnel_nic_del_port(dev, &mut ti);
}

pub unsafe fn udp_tunnel_notify_add_rx_port(sk: *mut sock, type_: u16) {
    let net = sock_net(sk);
    let mut ti: udp_tunnel_info = core::mem::zeroed();
    let mut dev: *mut net_device;
    ASSERT_RTNL();
    ti.type_ = type_;
    ti.sa_family = (*sk).sk_family;
    ti.port = (*inet_sk(sk)).inet_sport;
    for_each_netdev!(net, dev, {
        udp_tunnel_nic_lock(dev);
        udp_tunnel_nic_add_port(dev, &mut ti);
        udp_tunnel_nic_unlock(dev);
    });
}

pub unsafe fn udp_tunnel_notify_del_rx_port(sk: *mut sock, type_: u16) {
    let net = sock_net(sk);
    let mut ti: udp_tunnel_info = core::mem::zeroed();
    let mut dev: *mut net_device;
    ASSERT_RTNL();
    ti.type_ = type_;
    ti.sa_family = (*sk).sk_family;
    ti.port = (*inet_sk(sk)).inet_sport;
    for_each_netdev!(net, dev, {
        udp_tunnel_nic_lock(dev);
        udp_tunnel_nic_del_port(dev, &mut ti);
        udp_tunnel_nic_unlock(dev);
    });
}

pub unsafe fn udp_tunnel_xmit_skb(
    rt: *mut rtable, sk: *mut sock, skb: *mut sk_buff,
    src: __be32, dst: __be32, tos: u8, ttl: u8, df: __be16,
    src_port: __be16, dst_port: __be16, xnet: bool, nocheck: bool,
    ipcb_flags: u16,
) {
    __skb_push(skb, core::mem::size_of::<udphdr>());
    skb_reset_transport_header(skb);
    let uh = udp_hdr(skb);
    (*uh).dest = dst_port;
    (*uh).source = src_port;
    udp_set_len(uh, (*skb).len);
    memset(&mut (*IPCB(skb)).opt as *mut _, 0, core::mem::size_of_val(&(*IPCB(skb)).opt));
    udp_set_csum(nocheck, skb, src, dst, (*skb).len);
    iptunnel_xmit(sk, rt, skb, src, dst, IPPROTO_UDP, tos, ttl, df, xnet, ipcb_flags);
}

pub unsafe fn udp_tunnel_sock_release(sk: *mut sock) {
    let sock = (*sk).sk_socket;
    rcu_assign_sk_user_data(sk, core::ptr::null_mut());
    kernel_sock_shutdown(sock, SHUT_RDWR);
    sock_release(sock);
}

pub unsafe fn udp_tun_rx_dst(skb: *mut sk_buff, family: i32, flags: *const c_ulong,
                             tunnel_id: __be64, md_size: i32) -> *mut metadata_dst {
    let tun_dst = if family == AF_INET {
        ip_tun_rx_dst(skb, flags, tunnel_id, md_size)
    } else {
        ipv6_tun_rx_dst(skb, flags, tunnel_id, md_size)
    };
    if tun_dst.is_null() { return core::ptr::null_mut(); }
    let info = &mut (*tun_dst).u.tun_info;
    (*info).key.tp_src = (*udp_hdr(skb)).source;
    (*info).key.tp_dst = (*udp_hdr(skb)).dest;
    if (*udp_hdr(skb)).check != 0 {
        __set_bit(IP_TUNNEL_CSUM_BIT, &mut (*info).key.tun_flags);
    }
    tun_dst
}

pub unsafe fn udp_tunnel_dst_lookup(
    skb: *mut sk_buff, dev: *mut net_device, net: *mut net, oif: i32,
    saddr: *mut __be32, key: *const ip_tunnel_key, sport: __be16,
    dport: __be16, tos: u8, dst_cache: *mut dst_cache,
) -> *mut rtable {
    let mut rt: *mut rtable = core::ptr::null_mut();
    let mut fl4: flowi4 = core::mem::zeroed();

    // CONFIG_DST_CACHE conditional preserved from the C source.
    #[cfg(feature = "CONFIG_DST_CACHE")]
    if !dst_cache.is_null() {
        rt = dst_cache_get_ip4(dst_cache, saddr);
        if !rt.is_null() { return rt; }
    }

    fl4.flowi4_mark = (*skb).mark;
    fl4.flowi4_proto = IPPROTO_UDP;
    fl4.flowi4_oif = oif;
    fl4.daddr = (*key).u.ipv4.dst;
    fl4.saddr = (*key).u.ipv4.src;
    fl4.fl4_dport = dport;
    fl4.fl4_sport = sport;
    fl4.flowi4_dscp = inet_dsfield_to_dscp(tos);
    fl4.flowi4_flags = (*key).flow_flags;

    rt = ip_route_output_key(net, &mut fl4);
    if IS_ERR(rt) {
        netdev_dbg(dev, "no route to %pI4\n", &fl4.daddr);
        return ERR_PTR(-ENETUNREACH);
    }
    if (*rt).dst.dev == dev {
        netdev_dbg(dev, "circular route to %pI4\n", &fl4.daddr);
        ip_rt_put(rt);
        return ERR_PTR(-ELOOP);
    }
    #[cfg(feature = "CONFIG_DST_CACHE")]
    if !dst_cache.is_null() {
        dst_cache_set_ip4(dst_cache, &mut (*rt).dst, fl4.saddr);
    }
    *saddr = fl4.saddr;
    rt
}

// C module metadata: MODULE_DESCRIPTION("IPv4 Foo over UDP tunnel driver"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
