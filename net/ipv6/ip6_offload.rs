// SPDX-License-Identifier: GPL-2.0-or-later
/* IPV6 GSO/GRO offload support; Linux INET6 implementation */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ipv6_gro_pull_exthdrs(skb: *mut sk_buff, mut off: c_int, mut proto: c_int) -> c_int {
    let mut ops: *const net_offload = core::ptr::null();
    loop {
        ops = rcu_dereference(inet6_offloads[proto as usize]);
        if ops.is_null() || (*ops).flags & INET6_PROTO_GSO_EXTHDR == 0 { break; }
        let mut opth = skb_gro_header(skb, off + core::mem::size_of::<ipv6_opt_hdr>() as c_int, off);
        if opth.is_null() { break; }
        let len = ipv6_optlen(opth);
        opth = skb_gro_header(skb, off + len, off);
        if opth.is_null() { break; }
        proto = (*opth).nexthdr as c_int;
        off += len;
    }
    skb_gro_pull(skb, off - skb_gro_receive_network_offset(skb));
    proto
}

unsafe fn ipv6_gso_pull_exthdrs(skb: *mut sk_buff, mut proto: c_int) -> c_int {
    loop {
        let ops = rcu_dereference(inet6_offloads[proto as usize]);
        if ops.is_null() || (*ops).flags & INET6_PROTO_GSO_EXTHDR == 0 { break; }
        if pskb_may_pull(skb, 8) == 0 { break; }
        let mut opth = (*skb).data as *mut ipv6_opt_hdr;
        let len = ipv6_optlen(opth);
        if pskb_may_pull(skb, len) == 0 { break; }
        opth = (*skb).data as *mut ipv6_opt_hdr;
        proto = (*opth).nexthdr as c_int;
        __skb_pull(skb, len);
    }
    proto
}

unsafe fn ipv6_gso_segment(skb: *mut sk_buff, mut features: netdev_features_t) -> *mut sk_buff {
    let mut segs = ERR_PTR(-EINVAL);
    skb_reset_network_header(skb);
    let nhoff = skb_network_header(skb) - skb_mac_header(skb);
    if pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() as c_int) == 0 { return segs; }
    let encap = (*SKB_GSO_CB(skb)).encap_level > 0;
    if encap { features &= (*skb).dev.hw_enc_features; }
    (*SKB_GSO_CB(skb)).encap_level += core::mem::size_of::<ipv6hdr>() as c_int;
    let ipv6h = ipv6_hdr(skb);
    let next = (*ipv6h).nexthdr;
    __skb_pull(skb, core::mem::size_of::<ipv6hdr>() as c_int);
    segs = ERR_PTR(-EPROTONOSUPPORT);
    let proto = ipv6_gso_pull_exthdrs(skb, next as c_int);
    let udpfrag = if (*skb).encapsulation && (*skb_shinfo(skb)).gso_type & (SKB_GSO_IPXIP4|SKB_GSO_IPXIP6) != 0 {
        proto == IPPROTO_UDP && encap && (*skb_shinfo(skb)).gso_type & SKB_GSO_UDP != 0
    } else { proto == IPPROTO_UDP && !(*skb).encapsulation && (*skb_shinfo(skb)).gso_type & SKB_GSO_UDP != 0 };
    let ops = rcu_dereference(inet6_offloads[proto as usize]);
    if !ops.is_null() && (*ops).callbacks.gso_segment.is_some() {
        if skb_reset_transport_header_careful(skb) == 0 { return segs; }
        segs = ((*ops).callbacks.gso_segment.unwrap())(skb, features);
        if segs.is_null() { (*skb).network_header = skb_mac_header(skb) + nhoff - (*skb).head; }
    }
    if IS_ERR_OR_NULL(segs) { return segs; }
    let partial = (*skb_shinfo(segs)).gso_type & SKB_GSO_PARTIAL != 0;
    let mut offset = 0;
    let mut cur = segs;
    while !cur.is_null() {
        let h = (skb_mac_header(cur) + nhoff) as *mut ipv6hdr;
        let payload = if partial && skb_is_gso(cur) { (*skb_shinfo(cur)).gso_size + (*SKB_GSO_CB(cur)).data_offset + (*cur).head - (h.add(1) as *mut u8) as usize } else { (*cur).len - nhoff as usize - core::mem::size_of::<ipv6hdr>() };
        (*h).payload_len = htons(payload as u16);
        (*cur).network_header = h as *mut u8 as usize - (*cur).head as usize;
        skb_reset_mac_len(cur);
        if udpfrag {
            let mut prev: *mut u8 = core::ptr::null_mut(); let err = ip6_find_1stfragopt(cur, &mut prev);
            if err < 0 { kfree_skb_list(segs); return ERR_PTR(err); }
            let f = (h as *mut u8).add(err as usize) as *mut frag_hdr;
            (*f).frag_off = htons(offset as u16); if !(*cur).next.is_null() { (*f).frag_off |= htons(IP6_MF); }
            offset += ntohs((*h).payload_len) as i32 - core::mem::size_of::<frag_hdr>() as i32;
        }
        if encap { skb_reset_inner_headers(cur); }
        cur = (*cur).next;
    }
    segs
}

unsafe fn ipv6_exthdrs_len(iph: *mut ipv6hdr, opps: *mut *const net_offload) -> c_int {
    let mut opth = iph as *mut ipv6_opt_hdr; let mut len = 0; let mut optlen = core::mem::size_of::<ipv6hdr>() as c_int; let mut proto = (*iph).nexthdr as c_int;
    loop { *opps = rcu_dereference(inet6_offloads[proto as usize]); if (*opps).is_null() || (**opps).flags & INET6_PROTO_GSO_EXTHDR == 0 { break; } opth = (opth as *mut u8).add(optlen as usize) as *mut ipv6_opt_hdr; optlen = ipv6_optlen(opth); len += optlen; proto = (*opth).nexthdr as c_int; } len
}

// The remaining GRO callbacks retain the C control flow and call the translated kernel interfaces.
unsafe fn sit_gso_segment(skb: *mut sk_buff, f: netdev_features_t) -> *mut sk_buff { if (*skb_shinfo(skb)).gso_type & SKB_GSO_IPXIP4 == 0 { return ERR_PTR(-EINVAL); } ipv6_gso_segment(skb,f) }
unsafe fn ip6ip6_gso_segment(skb: *mut sk_buff, f: netdev_features_t) -> *mut sk_buff { if (*skb_shinfo(skb)).gso_type & SKB_GSO_IPXIP6 == 0 { return ERR_PTR(-EINVAL); } ipv6_gso_segment(skb,f) }
unsafe fn ip4ip6_gso_segment(skb: *mut sk_buff, f: netdev_features_t) -> *mut sk_buff { if (*skb_shinfo(skb)).gso_type & SKB_GSO_IPXIP6 == 0 { return ERR_PTR(-EINVAL); } inet_gso_segment(skb,f) }

unsafe fn sit_ip6ip6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    if (*NAPI_GRO_CB(skb)).encap_mark != 0 { (*NAPI_GRO_CB(skb)).flush = 1; return core::ptr::null_mut(); }
    (*NAPI_GRO_CB(skb)).encap_mark = 1; ipv6_gro_receive(head, skb)
}
unsafe fn ip4ip6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    if (*NAPI_GRO_CB(skb)).encap_mark != 0 { (*NAPI_GRO_CB(skb)).flush = 1; return core::ptr::null_mut(); }
    (*NAPI_GRO_CB(skb)).encap_mark = 1; inet_gro_receive(head, skb)
}

unsafe fn ipv6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    let off = skb_gro_offset(skb); let hlen = off + core::mem::size_of::<ipv6hdr>() as c_int;
    let mut iph = skb_gro_header(skb, hlen, off); if iph.is_null() { skb_gro_flush_final(skb, core::ptr::null_mut(), 1); return core::ptr::null_mut(); }
    (*NAPI_GRO_CB(skb)).network_offsets[(*NAPI_GRO_CB(skb)).encap_mark as usize] = off;
    let mut flush: u16 = 1 + (ntohs((*iph).payload_len) as usize != (*skb).len - hlen as usize) as u16;
    let mut proto = (*iph).nexthdr as c_int; let mut ops = rcu_dereference(inet6_offloads[proto as usize]);
    if ops.is_null() || (*ops).callbacks.gro_receive.is_none() { proto = ipv6_gro_pull_exthdrs(skb,hlen,proto); ops = rcu_dereference(inet6_offloads[proto as usize]); if ops.is_null() || (*ops).callbacks.gro_receive.is_none() { skb_gro_flush_final(skb,core::ptr::null_mut(),flush); return core::ptr::null_mut(); } iph = skb_gro_network_header(skb); } else { skb_gro_pull(skb, core::mem::size_of::<ipv6hdr>() as c_int); }
    skb_set_transport_header(skb, skb_gro_offset(skb)); (*NAPI_GRO_CB(skb)).proto = proto as u16; flush -= 1;
    let nlen = skb_gro_offset(skb) - off;
    let mut p = (*head).next as *mut sk_buff; while p != head as *mut sk_buff { if (*NAPI_GRO_CB(p)).same_flow != 0 { let iph2 = ((*p).data.add(off as usize)) as *const ipv6hdr; let first = *(iph as *const u32) ^ *(iph2 as *const u32); if first & htonl(0xF00FFFFF) != 0 || !ipv6_addr_equal(&(*iph).saddr,&(*iph2).saddr) || !ipv6_addr_equal(&(*iph).daddr,&(*iph2).daddr) || (*iph).nexthdr != (*iph2).nexthdr { (*NAPI_GRO_CB(p)).same_flow=0; } else if nlen as usize > core::mem::size_of::<ipv6hdr>() && memcmp(iph.add(1) as *const _,iph2.add(1) as *const _,(nlen as usize)-core::mem::size_of::<ipv6hdr>()) != 0 { (*NAPI_GRO_CB(p)).same_flow=0; } } p=(*p).next; }
    (*NAPI_GRO_CB(skb)).flush |= flush; skb_gro_postpull_rcsum(skb,iph,nlen); let mut pp=core::ptr::null_mut(); if gro_recursion_inc_test(skb)!=0 { flush=1; } else if proto==IPPROTO_TCP { pp=tcp6_gro_receive(head,skb); } else if proto==IPPROTO_UDP { pp=udp6_gro_receive(head,skb); } else { pp=((*ops).callbacks.gro_receive.unwrap())(head,skb); } skb_gro_flush_final(skb,pp,flush); pp
}

unsafe fn ipv6_gro_complete(skb: *mut sk_buff, mut nhoff: c_int) -> c_int { let mut ops: *const net_offload=core::ptr::null(); if (*skb).encapsulation { skb_set_inner_protocol(skb,cpu_to_be16(ETH_P_IPV6)); skb_set_inner_network_header(skb,nhoff); } let iph=( (*skb).data.add(nhoff as usize)) as *mut ipv6hdr; ipv6_set_payload_len(iph,(*skb).len-nhoff as usize-core::mem::size_of::<ipv6hdr>()); nhoff+=core::mem::size_of::<ipv6hdr>() as c_int+ipv6_exthdrs_len(iph,&mut ops); if ops==&net_hotdata.tcpv6_offload { return tcp6_gro_complete(skb,nhoff); } if ops==&net_hotdata.udpv6_offload { return udp6_gro_complete(skb,nhoff); } if ops.is_null() || (*ops).callbacks.gro_complete.is_none() { return -ENOSYS; } ((*ops).callbacks.gro_complete.unwrap())(skb,nhoff) }
unsafe fn sit_gro_complete(skb:*mut sk_buff,n:c_int)->c_int{(*skb).encapsulation=true;(*skb_shinfo(skb)).gso_type|=SKB_GSO_IPXIP4;ipv6_gro_complete(skb,n)}
unsafe fn ip6ip6_gro_complete(skb:*mut sk_buff,n:c_int)->c_int{(*skb).encapsulation=true;(*skb_shinfo(skb)).gso_type|=SKB_GSO_IPXIP6;ipv6_gro_complete(skb,n)}
unsafe fn ip4ip6_gro_complete(skb:*mut sk_buff,n:c_int)->c_int{(*skb).encapsulation=true;(*skb_shinfo(skb)).gso_type|=SKB_GSO_IPXIP6;inet_gro_complete(skb,n)}

static sit_offload: net_offload = net_offload { callbacks: net_offload_callbacks { gso_segment: Some(sit_gso_segment), gro_receive: Some(sit_ip6ip6_gro_receive), gro_complete: Some(sit_gro_complete) }, flags: 0 };
static ip4ip6_offload: net_offload = net_offload { callbacks: net_offload_callbacks { gso_segment: Some(ip4ip6_gso_segment), gro_receive: Some(ip4ip6_gro_receive), gro_complete: Some(ip4ip6_gro_complete) }, flags: 0 };
static ip6ip6_offload: net_offload = net_offload { callbacks: net_offload_callbacks { gso_segment: Some(ip6ip6_gso_segment), gro_receive: Some(sit_ip6ip6_gro_receive), gro_complete: Some(ip6ip6_gro_complete) }, flags: 0 };

unsafe fn ipv6_offload_init() -> c_int {
    if tcpv6_offload_init() < 0 { pr_crit!("{}: Cannot add TCP protocol offload\n", "ipv6_offload_init"); }
    if ipv6_exthdrs_offload_init() < 0 { pr_crit!("{}: Cannot add EXTHDRS protocol offload\n", "ipv6_offload_init"); }
    net_hotdata.ipv6_packet_offload = packet_offload { type_: cpu_to_be16(ETH_P_IPV6), callbacks: packet_offload_callbacks { gso_segment: Some(ipv6_gso_segment), gro_receive: Some(ipv6_gro_receive), gro_complete: Some(ipv6_gro_complete) } };
    dev_add_offload(&mut net_hotdata.ipv6_packet_offload);
    inet_add_offload(&sit_offload, IPPROTO_IPV6); inet6_add_offload(&ip6ip6_offload, IPPROTO_IPV6); inet6_add_offload(&ip4ip6_offload, IPPROTO_IPIP); 0
}

// fs_initcall(ipv6_offload_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
