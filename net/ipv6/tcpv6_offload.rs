// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	IPV6 GSO/GRO offload support
 *	Linux INET6 implementation
 *
 *      TCPv6 GSO/GRO support
 */
// C dependencies supplied by the surrounding kernel translation.

unsafe fn tcp6_check_fraglist_gro(head: *mut list_head, skb: *mut sk_buff,
                                  th: *mut tcphdr) {
    // #if IS_ENABLED(CONFIG_IPV6)
    let hdr: *const ipv6hdr;
    let mut p: *mut sk_buff;
    let mut sk: *mut sock;
    let net: *mut net;
    let (mut iif, mut sdif): (i32, i32);

    p = tcp_gro_lookup(head, th);
    if !p.is_null() {
        (*NAPI_GRO_CB(skb)).is_flist = (*NAPI_GRO_CB(p)).is_flist;
        return;
    }

    inet6_get_iif_sdif(skb, &mut iif, &mut sdif);
    hdr = skb_gro_network_header(skb);
    net = dev_net_rcu((*skb).dev);
    sk = __inet6_lookup_established(net, &(*hdr).saddr, (*th).source,
                                    &(*hdr).daddr, ntohs((*th).dest),
                                    iif, sdif);
    (*NAPI_GRO_CB(skb)).is_flist = sk.is_null();
    if !sk.is_null() {
        sock_gen_put(sk);
    }
    // #endif /* IS_ENABLED(CONFIG_IPV6) */
}

unsafe fn tcp6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let th: *mut tcphdr;

    /* Don't bother verifying checksum if we're going to flush anyway. */
    if !(*NAPI_GRO_CB(skb)).flush
        && skb_gro_checksum_validate(skb, IPPROTO_TCP, ip6_gro_compute_pseudo) != 0
    {
        (*NAPI_GRO_CB(skb)).flush = 1;
        return core::ptr::null_mut();
    }

    th = tcp_gro_pull_header(skb);
    if th.is_null() {
        (*NAPI_GRO_CB(skb)).flush = 1;
        return core::ptr::null_mut();
    }

    if unlikely((*(*skb).dev).features & NETIF_F_GRO_FRAGLIST) != 0 {
        tcp6_check_fraglist_gro(head, skb, th);
    }

    tcp_gro_receive(head, skb, th)
}

unsafe fn tcp6_gro_complete(skb: *mut sk_buff, thoff: i32) -> i32 {
    let offset: u16 = (*NAPI_GRO_CB(skb)).network_offsets[(*NAPI_GRO_CB(skb)).encapsulation as usize];
    let iph: *const ipv6hdr = ((*skb).data.add(offset as usize)) as *const ipv6hdr;
    let th: *mut tcphdr = tcp_hdr(skb);

    if unlikely((*NAPI_GRO_CB(skb)).is_flist) {
        (*skb_shinfo(skb)).gso_type |= SKB_GSO_FRAGLIST | SKB_GSO_TCPV6;
        (*skb_shinfo(skb)).gso_segs = (*NAPI_GRO_CB(skb)).count;
        __skb_incr_checksum_unnecessary(skb);
        return 0;
    }

    (*th).check = !tcp_v6_check((*skb).len - thoff as usize, &(*iph).saddr,
                                &(*iph).daddr, 0);
    (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV6;
    tcp_gro_complete(skb);
    0
}

unsafe fn __tcpv6_gso_segment_csum(seg: *mut sk_buff, oldip: *mut in6_addr,
                                   newip: *const in6_addr, oldport: *mut __be16,
                                   newport: __be16) {
    let th: *mut tcphdr = tcp_hdr(seg);
    if !ipv6_addr_equal(oldip, newip) {
        inet_proto_csum_replace16(&mut (*th).check, seg, (*oldip).s6_addr32,
                                  (*newip).s6_addr32, true);
        *oldip = *newip;
    }
    if *oldport == newport { return; }
    inet_proto_csum_replace2(&mut (*th).check, seg, *oldport, newport, false);
    *oldport = newport;
}

unsafe fn __tcpv6_gso_segment_list_csum(segs: *mut sk_buff) -> *mut sk_buff {
    let seg = segs;
    let th = tcp_hdr(seg);
    let iph = ipv6_hdr(seg);
    let th2 = tcp_hdr((*seg).next);
    let iph2 = ipv6_hdr((*seg).next);
    if (*th).source as u32 ^ (*th2).source as u32 == 0
        && ipv6_addr_equal(&(*iph).saddr, &(*iph2).saddr)
        && ipv6_addr_equal(&(*iph).daddr, &(*iph2).daddr) { return segs; }
    let mut seg = (*seg).next;
    while !seg.is_null() {
        let th2 = tcp_hdr(seg);
        let iph2 = ipv6_hdr(seg);
        __tcpv6_gso_segment_csum(seg, &mut (*iph2).saddr, &(*iph).saddr,
                                 &mut (*th2).source, (*th).source);
        __tcpv6_gso_segment_csum(seg, &mut (*iph2).daddr, &(*iph).daddr,
                                 &mut (*th2).dest, (*th).dest);
        seg = (*seg).next;
    }
    segs
}

unsafe fn __tcp6_gso_segment_list(mut skb: *mut sk_buff,
                                  features: netdev_features_t) -> *mut sk_buff {
    skb = skb_segment_list(skb, features, skb_mac_header_len(skb));
    if IS_ERR(skb) { return skb; }
    __tcpv6_gso_segment_list_csum(skb)
}

unsafe fn tcp6_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff {
    if (*skb_shinfo(skb)).gso_type & SKB_GSO_TCPV6 == 0 { return ERR_PTR(-EINVAL); }
    if !pskb_may_pull(skb, core::mem::size_of::<tcphdr>()) { return ERR_PTR(-EINVAL); }
    if (*skb_shinfo(skb)).gso_type & SKB_GSO_FRAGLIST != 0 {
        let th = tcp_hdr(skb);
        if skb_pagelen(skb) - (*th).doff as usize * 4 == (*skb_shinfo(skb)).gso_size
            && (*skb_shinfo(skb)).gso_type & SKB_GSO_DODGY == 0 {
            return __tcp6_gso_segment_list(skb, features);
        }
        (*skb).ip_summed = CHECKSUM_NONE;
    }
    if unlikely((*skb).ip_summed != CHECKSUM_PARTIAL) {
        let ipv6h = ipv6_hdr(skb);
        let th = tcp_hdr(skb);
        (*th).check = 0;
        (*skb).ip_summed = CHECKSUM_PARTIAL;
        __tcp_v6_send_check(skb, &(*ipv6h).saddr, &(*ipv6h).daddr);
    }
    tcp_gso_segment(skb, features)
}

pub unsafe fn tcpv6_offload_init() -> i32 {
    net_hotdata.tcpv6_offload = net_offload {
        callbacks: net_offload_callbacks {
            gso_segment: Some(tcp6_gso_segment),
            gro_receive: Some(tcp6_gro_receive),
            gro_complete: Some(tcp6_gro_complete),
        },
    };
    inet6_add_offload(&mut net_hotdata.tcpv6_offload, IPPROTO_TCP)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
