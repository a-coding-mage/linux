// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2008 BalaBit IT Ltd.
 * Author: Krisztian Kovacs
 */

// Linux kernel dependencies supplied by other translation units.

unsafe fn extract_icmp4_fields(
    skb: *const sk_buff,
    protocol: *mut u8,
    raddr: *mut __be32,
    laddr: *mut __be32,
    rport: *mut __be16,
    lport: *mut __be16,
) -> i32 {
    let outside_hdrlen: unsigned_int = unsafe { ip_hdrlen(skb) };
    let mut inside_iph: *mut iphdr;
    let mut _inside_iph: iphdr = unsafe { core::mem::zeroed() };
    let mut icmph: *mut icmphdr;
    let mut _icmph: icmphdr = unsafe { core::mem::zeroed() };
    let mut ports: *mut __be16;
    let mut _ports: [__be16; 2] = [0; 2];

    icmph = unsafe {
        skb_header_pointer(
            skb,
            outside_hdrlen,
            core::mem::size_of::<icmphdr>(),
            &mut _icmph as *mut icmphdr as *mut core::ffi::c_void,
        )
    } as *mut icmphdr;
    if icmph.is_null() {
        return 1;
    }

    if unsafe { !icmp_is_err((*icmph).type_) } {
        return 1;
    }

    inside_iph = unsafe {
        skb_header_pointer(
            skb,
            outside_hdrlen + core::mem::size_of::<icmphdr>() as unsigned_int,
            core::mem::size_of::<iphdr>(),
            &mut _inside_iph as *mut iphdr as *mut core::ffi::c_void,
        )
    } as *mut iphdr;
    if inside_iph.is_null() {
        return 1;
    }

    if unsafe { (*inside_iph).protocol != IPPROTO_TCP && (*inside_iph).protocol != IPPROTO_UDP } {
        return 1;
    }

    ports = unsafe {
        skb_header_pointer(
            skb,
            outside_hdrlen
                + core::mem::size_of::<icmphdr>() as unsigned_int
                + ((*inside_iph).ihl as unsigned_int) << 2,
            core::mem::size_of::<[__be16; 2]>(),
            _ports.as_mut_ptr() as *mut core::ffi::c_void,
        )
    } as *mut __be16;
    if ports.is_null() {
        return 1;
    }

    /* the inside IP packet is the one quoted from our side, thus
     * its saddr is the local address */
    unsafe {
        *protocol = (*inside_iph).protocol;
        *laddr = (*inside_iph).saddr;
        *lport = *ports;
        *raddr = (*inside_iph).daddr;
        *rport = *ports.add(1);
    }

    0
}

unsafe fn nf_socket_get_sock_v4(
    net: *mut net,
    skb: *mut sk_buff,
    doff: i32,
    protocol: u8,
    saddr: __be32,
    daddr: __be32,
    sport: __be16,
    dport: __be16,
    input: *const net_device,
) -> *mut sock {
    match protocol {
        IPPROTO_TCP => unsafe { inet_lookup(net, skb, doff, saddr, sport, daddr, dport, (*input).ifindex) },
        IPPROTO_UDP => unsafe { udp4_lib_lookup(net, saddr, sport, daddr, dport, (*input).ifindex) },
        _ => core::ptr::null_mut(),
    }
}

pub unsafe fn nf_sk_lookup_slow_v4(
    net: *mut net,
    skb: *const sk_buff,
    indev: *const net_device,
) -> *mut sock {
    let mut daddr: __be32;
    let mut saddr: __be32;
    let mut dport: __be16;
    let mut sport: __be16;
    let iph: *const iphdr = unsafe { ip_hdr(skb) };
    let mut data_skb: *mut sk_buff = core::ptr::null_mut();
    let mut protocol: u8;
    let mut doff: i32 = 0;

    if unsafe { ntohs((*iph).frag_off) } & IP_OFFSET != 0 {
        return core::ptr::null_mut();
    }

    if unsafe { (*iph).protocol == IPPROTO_UDP || (*iph).protocol == IPPROTO_TCP } {
        let mut _hdr: tcphdr = unsafe { core::mem::zeroed() };
        let hp: *mut udphdr = unsafe {
            skb_header_pointer(
                skb,
                ip_hdrlen(skb),
                if (*iph).protocol == IPPROTO_UDP { core::mem::size_of::<udphdr>() } else { core::mem::size_of::<tcphdr>() },
                &mut _hdr as *mut tcphdr as *mut core::ffi::c_void,
            )
        } as *mut udphdr;
        if hp.is_null() {
            return core::ptr::null_mut();
        }
        unsafe {
            protocol = (*iph).protocol;
            saddr = (*iph).saddr;
            sport = (*hp).source;
            daddr = (*iph).daddr;
            dport = (*hp).dest;
            data_skb = skb as *mut sk_buff;
            doff = if (*iph).protocol == IPPROTO_TCP {
                ip_hdrlen(skb) as i32 + __tcp_hdrlen(hp as *mut tcphdr) as i32
            } else {
                ip_hdrlen(skb) as i32 + core::mem::size_of::<udphdr>() as i32
            };
        }
    } else if unsafe { (*iph).protocol == IPPROTO_ICMP } {
        if unsafe { extract_icmp4_fields(skb, &mut protocol, &mut saddr, &mut daddr, &mut sport, &mut dport) } != 0 {
            return core::ptr::null_mut();
        }
    } else {
        return core::ptr::null_mut();
    }

    unsafe { nf_socket_get_sock_v4(net, data_skb, doff, protocol, saddr, daddr, sport, dport, indev) }
}

// EXPORT_SYMBOL_GPL(nf_sk_lookup_slow_v4);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Krisztian Kovacs, Balazs Scheidler");
// MODULE_DESCRIPTION("Netfilter IPv4 socket lookup infrastructure");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
