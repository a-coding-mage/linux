// SPDX-License-Identifier: GPL-2.0-only
// Translated from fou6.c. Kernel dependencies are supplied externally.

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn fou6_build_udp(
    skb: *mut sk_buff,
    e: *mut ip_tunnel_encap,
    fl6: *mut flowi6,
    protocol: *mut u8,
    sport: __be16,
) {
    let uh: *mut udphdr;

    skb_push(skb, core::mem::size_of::<udphdr>());
    skb_reset_transport_header(skb);

    uh = udp_hdr(skb);

    (*uh).dest = (*e).dport;
    (*uh).source = sport;
    udp_set_len(uh, (*skb).len);
    udp6_set_csum(
        ((*e).flags & TUNNEL_ENCAP_FLAG_CSUM6) == 0,
        skb,
        &(*fl6).saddr,
        &(*fl6).daddr,
        (*skb).len,
    );

    *protocol = IPPROTO_UDP as u8;
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn fou6_build_header(
    skb: *mut sk_buff,
    e: *mut ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    let mut sport: __be16 = 0;
    let type_: c_int = if (*e).flags & TUNNEL_ENCAP_FLAG_CSUM6 != 0 {
        SKB_GSO_UDP_TUNNEL_CSUM
    } else {
        SKB_GSO_UDP_TUNNEL
    };

    let err = __fou_build_header(skb, e, protocol, &mut sport, type_);
    if err != 0 {
        return err;
    }

    fou6_build_udp(skb, e, fl6, protocol, sport);
    0
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn gue6_build_header(
    skb: *mut sk_buff,
    e: *mut ip_tunnel_encap,
    protocol: *mut u8,
    fl6: *mut flowi6,
) -> c_int {
    let mut sport: __be16 = 0;
    let type_: c_int = if (*e).flags & TUNNEL_ENCAP_FLAG_CSUM6 != 0 {
        SKB_GSO_UDP_TUNNEL_CSUM
    } else {
        SKB_GSO_UDP_TUNNEL
    };

    let err = __gue_build_header(skb, e, protocol, &mut sport, type_);
    if err != 0 {
        return err;
    }

    fou6_build_udp(skb, e, fl6, protocol, sport);
    0
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn gue6_err_proto_handler(
    proto: c_int,
    skb: *mut sk_buff,
    opt: *mut inet6_skb_parm,
    type_: u8,
    code: u8,
    offset: c_int,
    info: __be32,
) -> c_int {
    let ipprot = rcu_dereference(inet6_protos[proto as usize]);
    if !ipprot.is_null() && (*ipprot).err_handler.is_some() {
        if ((*ipprot).err_handler.unwrap())(skb, opt, type_, code, offset, info) == 0 {
            return 0;
        }
    }
    -ENOENT
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn gue6_err(
    skb: *mut sk_buff,
    opt: *mut inet6_skb_parm,
    type_: u8,
    code: u8,
    offset: c_int,
    info: __be32,
) -> c_int {
    let transport_offset = skb_transport_offset(skb);
    let mut guehdr: *mut guehdr;
    let len = core::mem::size_of::<udphdr>() + core::mem::size_of::<guehdr>();
    let mut optlen: usize;
    let ret: c_int;

    if !pskb_may_pull(skb, transport_offset + len) {
        return -EINVAL;
    }

    guehdr = (&mut udp_hdr(skb).add(1) as *mut *mut udphdr) as *mut guehdr;
    match (*guehdr).version {
        0 => {}
        1 => {
            skb_set_transport_header(skb, -(core::mem::size_of::<icmp6hdr>() as c_int));
            match (*(guehdr as *mut iphdr)).version {
                4 => {
                    ret = gue6_err_proto_handler(IPPROTO_IPIP, skb, opt, type_, code, offset, info);
                    skb_set_transport_header(skb, transport_offset);
                    return ret;
                }
                6 => {
                    ret = gue6_err_proto_handler(IPPROTO_IPV6, skb, opt, type_, code, offset, info);
                    skb_set_transport_header(skb, transport_offset);
                    return ret;
                }
                _ => {
                    ret = -EOPNOTSUPP;
                    skb_set_transport_header(skb, transport_offset);
                    return ret;
                }
            }
        }
        _ => return -EOPNOTSUPP,
    }

    if (*guehdr).control != 0 {
        return -ENOENT;
    }
    optlen = ((*guehdr).hlen as usize) << 2;
    if !pskb_may_pull(skb, transport_offset + len + optlen) {
        return -EINVAL;
    }
    guehdr = (&mut udp_hdr(skb).add(1) as *mut *mut udphdr) as *mut guehdr;
    if validate_gue_flags(guehdr, optlen) != 0 {
        return -EINVAL;
    }
    // Direct UDP encapsulation in GUE would recurse and is not configurable.
    if (*guehdr).proto_ctype == IPPROTO_UDP as u8 {
        return -EOPNOTSUPP;
    }
    skb_set_transport_header(skb, -(core::mem::size_of::<icmp6hdr>() as c_int));
    ret = gue6_err_proto_handler((*guehdr).proto_ctype as c_int, skb, opt, type_, code, offset, info);
    skb_set_transport_header(skb, transport_offset);
    ret
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
static fou_ip6tun_ops: ip6_tnl_encap_ops = ip6_tnl_encap_ops {
    encap_hlen: fou_encap_hlen,
    build_header: Some(fou6_build_header),
    err_handler: Some(gue6_err),
};

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
static gue_ip6tun_ops: ip6_tnl_encap_ops = ip6_tnl_encap_ops {
    encap_hlen: gue_encap_hlen,
    build_header: Some(gue6_build_header),
    err_handler: Some(gue6_err),
};

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn ip6_tnl_encap_add_fou_ops() -> c_int {
    let ret = ip6_tnl_encap_add_ops(&fou_ip6tun_ops, TUNNEL_ENCAP_FOU);
    if ret < 0 { pr_err!("can't add fou6 ops\n"); return ret; }
    let ret = ip6_tnl_encap_add_ops(&gue_ip6tun_ops, TUNNEL_ENCAP_GUE);
    if ret < 0 {
        pr_err!("can't add gue6 ops\n");
        ip6_tnl_encap_del_ops(&fou_ip6tun_ops, TUNNEL_ENCAP_FOU);
        return ret;
    }
    0
}

#[cfg(feature = "CONFIG_IPV6_FOU_TUNNEL")]
unsafe fn ip6_tnl_encap_del_fou_ops() {
    ip6_tnl_encap_del_ops(&fou_ip6tun_ops, TUNNEL_ENCAP_FOU);
    ip6_tnl_encap_del_ops(&gue_ip6tun_ops, TUNNEL_ENCAP_GUE);
}

#[cfg(not(feature = "CONFIG_IPV6_FOU_TUNNEL"))]
unsafe fn ip6_tnl_encap_add_fou_ops() -> c_int { 0 }

#[cfg(not(feature = "CONFIG_IPV6_FOU_TUNNEL"))]
unsafe fn ip6_tnl_encap_del_fou_ops() {}

unsafe fn fou6_init() -> c_int {
    ip6_tnl_encap_add_fou_ops()
}

unsafe fn fou6_fini() {
    ip6_tnl_encap_del_fou_ops();
}

module_init!(fou6_init);
module_exit!(fou6_fini);
module_author!("Tom Herbert <therbert@google.com>");
module_license!("GPL");
module_description!("Foo over UDP (IPv6)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
