// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2008 BalaBit IT Ltd.
 * Author: Krisztian Kovacs
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn extract_icmp6_fields(
    skb: *const sk_buff,
    outside_hdrlen: c_uint,
    protocol: *mut c_int,
    raddr: *mut *const in6_addr,
    laddr: *mut *const in6_addr,
    rport: *mut __be16,
    lport: *mut __be16,
    ipv6_var: *mut ipv6hdr,
) -> c_int {
    let mut _icmph: icmp6hdr = core::mem::zeroed();
    let mut _ports: [__be16; 2] = [0; 2];
    let icmph = skb_header_pointer(
        skb,
        outside_hdrlen,
        core::mem::size_of::<icmp6hdr>(),
        &mut _icmph as *mut _ as *mut c_void,
    ) as *mut icmp6hdr;
    if icmph.is_null() {
        return 1;
    }

    if (*icmph).icmp6_type & ICMPV6_INFOMSG_MASK != 0 {
        return 1;
    }

    let inside_iph = skb_header_pointer(
        skb,
        outside_hdrlen + core::mem::size_of::<icmp6hdr>() as c_uint,
        core::mem::size_of::<ipv6hdr>(),
        ipv6_var as *mut c_void,
    ) as *const ipv6hdr;
    if inside_iph.is_null() {
        return 1;
    }
    let mut inside_nexthdr = (*inside_iph).nexthdr;
    let mut inside_fragoff: __be16 = 0;
    let inside_hdrlen = ipv6_skip_exthdr(
        skb,
        outside_hdrlen + core::mem::size_of::<icmp6hdr>() as c_uint
            + core::mem::size_of::<ipv6hdr>() as c_uint,
        &mut inside_nexthdr,
        &mut inside_fragoff,
    );
    if inside_hdrlen < 0 {
        return 1; // Packet has no/incomplete transport layer headers.
    }

    if inside_nexthdr != IPPROTO_TCP && inside_nexthdr != IPPROTO_UDP {
        return 1;
    }

    let ports = skb_header_pointer(
        skb,
        inside_hdrlen as c_uint,
        core::mem::size_of::<[__be16; 2]>(),
        _ports.as_mut_ptr() as *mut c_void,
    ) as *const __be16;
    if ports.is_null() {
        return 1;
    }

    // The inside IP packet is the one quoted from our side, thus its saddr is
    // the local address.
    *protocol = inside_nexthdr as c_int;
    *laddr = &(*inside_iph).saddr;
    *lport = *ports;
    *raddr = &(*inside_iph).daddr;
    *rport = *ports.add(1);
    0
}

unsafe fn nf_socket_get_sock_v6(
    net: *mut net,
    skb: *mut sk_buff,
    doff: c_int,
    protocol: u8,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: __be16,
    dport: __be16,
    input: *const net_device,
) -> *mut sock {
    match protocol as c_int {
        IPPROTO_TCP => inet6_lookup(net, skb, doff, saddr, sport, daddr, dport, (*input).ifindex),
        IPPROTO_UDP => udp6_lib_lookup(net, saddr, sport, daddr, dport, (*input).ifindex),
        _ => core::ptr::null_mut(),
    }
}

pub unsafe fn nf_sk_lookup_slow_v6(
    net: *mut net,
    skb: *const sk_buff,
    indev: *const net_device,
) -> *mut sock {
    let mut dport: __be16 = 0;
    let mut sport: __be16 = 0;
    let mut daddr: *const in6_addr = core::ptr::null();
    let mut saddr: *const in6_addr = core::ptr::null();
    let iph = ipv6_hdr(skb);
    let mut ipv6_var: ipv6hdr = core::mem::zeroed();
    let mut data_skb: *mut sk_buff = core::ptr::null_mut();
    let mut fragoff: c_ushort = 0;
    let mut doff: c_int = 0;
    let mut thoff: c_int = 0;
    let mut tproto: c_int;

    tproto = ipv6_find_hdr(skb, &mut thoff, -1, &mut fragoff, core::ptr::null_mut());
    if tproto < 0 || fragoff != 0 {
        pr_debug!("unable to find transport header in IPv6 packet, dropping\n");
        return core::ptr::null_mut();
    }

    if tproto == IPPROTO_UDP || tproto == IPPROTO_TCP {
        let mut _hdr: tcphdr = core::mem::zeroed();
        let hp = skb_header_pointer(
            skb,
            thoff as c_uint,
            if tproto == IPPROTO_UDP { core::mem::size_of::<udphdr>() } else { core::mem::size_of::<tcphdr>() },
            &mut _hdr as *mut _ as *mut c_void,
        ) as *mut udphdr;
        if hp.is_null() {
            return core::ptr::null_mut();
        }
        saddr = &(*iph).saddr;
        sport = (*hp).source;
        daddr = &(*iph).daddr;
        dport = (*hp).dest;
        data_skb = skb as *mut sk_buff;
        doff = if tproto == IPPROTO_TCP {
            thoff + __tcp_hdrlen(hp as *mut tcphdr)
        } else {
            thoff + core::mem::size_of::<udphdr>() as c_int
        };
    } else if tproto == IPPROTO_ICMPV6 {
        if extract_icmp6_fields(skb, thoff as c_uint, &mut tproto, &mut saddr, &mut daddr, &mut sport, &mut dport, &mut ipv6_var) != 0 {
            return core::ptr::null_mut();
        }
    } else {
        return core::ptr::null_mut();
    }

    nf_socket_get_sock_v6(net, data_skb, doff, tproto as u8, saddr, daddr, sport, dport, indev)
}

// EXPORT_SYMBOL_GPL(nf_sk_lookup_slow_v6);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Krisztian Kovacs, Balazs Scheidler");
// MODULE_DESCRIPTION("Netfilter IPv6 socket lookup infrastructure");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
