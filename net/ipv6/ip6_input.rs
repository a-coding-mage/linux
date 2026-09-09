// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv6 input; translated from ip6_input.c. */

unsafe fn tcp_v6_early_demux(skb: *mut sk_buff) {
    let net = dev_net_rcu((*skb).dev);
    let hdr: *const ipv6hdr;
    let th: *const tcphdr;
    let sk: *mut sock;

    if (*skb).pkt_type != PACKET_HOST { return; }
    if !pskb_may_pull(skb, skb_transport_offset(skb) + core::mem::size_of::<tcphdr>()) { return; }
    hdr = ipv6_hdr(skb);
    th = tcp_hdr(skb);
    if (*th).doff < core::mem::size_of::<tcphdr>() / 4 { return; }
    sk = __inet6_lookup_established(net, &(*hdr).saddr, (*th).source,
        &(*hdr).daddr, ntohs((*th).dest), inet6_iif(skb), inet6_sdif(skb));
    if !sk.is_null() {
        (*skb).sk = sk;
        (*skb).destructor = Some(sock_edemux);
        if sk_fullsock(sk) {
            let mut dst = rcu_dereference((*sk).sk_rx_dst);
            if !dst.is_null() { dst = dst_check(dst, (*sk).sk_rx_dst_cookie); }
            if !dst.is_null() && (*sk).sk_rx_dst_ifindex == (*skb).skb_iif { skb_dst_set_noref(skb, dst); }
        }
    }
}

unsafe fn ip6_rcv_finish_core(net: *mut net, sk: *mut sock, skb: *mut sk_buff) {
    if READ_ONCE((*net).ipv4.sysctl_ip_early_demux) && skb_dst(skb).is_null() && (*skb).sk.is_null() {
        match (*ipv6_hdr(skb)).nexthdr {
            IPPROTO_TCP => if READ_ONCE((*net).ipv4.sysctl_tcp_early_demux) { tcp_v6_early_demux(skb); },
            IPPROTO_UDP => if READ_ONCE((*net).ipv4.sysctl_udp_early_demux) { udp_v6_early_demux(skb); },
            _ => {}
        }
    }
    if !skb_valid_dst(skb) { ip6_route_input(skb); }
}

pub unsafe fn ip6_rcv_finish(net: *mut net, sk: *mut sock, mut skb: *mut sk_buff) -> c_int {
    skb = l3mdev_ip6_rcv(skb);
    if skb.is_null() { return NET_RX_SUCCESS; }
    ip6_rcv_finish_core(net, sk, skb);
    dst_input(skb)
}

unsafe fn ip6_sublist_rcv_finish(head: *mut list_head) {
    let mut skb: *mut sk_buff;
    let mut next: *mut sk_buff;
    list_for_each_entry_safe!(skb, next, head, list, { skb_list_del_init(skb); dst_input(skb); });
}

unsafe fn ip6_can_use_hint(skb: *const sk_buff, hint: *const sk_buff) -> bool {
    !hint.is_null() && skb_dst(skb as *mut sk_buff).is_null() &&
        ipv6_addr_equal(&(*ipv6_hdr(hint as *mut sk_buff)).daddr, &(*ipv6_hdr(skb as *mut sk_buff)).daddr)
}

unsafe fn ip6_extract_route_hint(net: *const net, skb: *mut sk_buff) -> *mut sk_buff {
    if fib6_routes_require_src(net) || fib6_has_custom_rules(net) || ((*IP6CB(skb)).flags & IP6SKB_MULTIPATH) != 0 { core::ptr::null_mut() } else { skb }
}

unsafe fn ip6_list_rcv_finish(net: *mut net, sk: *mut sock, head: *mut list_head) {
    let mut skb: *mut sk_buff;
    let mut next: *mut sk_buff;
    let mut hint: *mut sk_buff = core::ptr::null_mut();
    let mut curr_dst: *mut dst_entry = core::ptr::null_mut();
    let mut sublist: list_head = LIST_HEAD_INIT!();
    list_for_each_entry_safe!(skb, next, head, list, {
        let mut dst: *mut dst_entry;
        skb_list_del_init(skb);
        skb = l3mdev_ip6_rcv(skb);
        if skb.is_null() { continue; }
        if ip6_can_use_hint(skb, hint) { skb_dst_copy(skb, hint); } else { ip6_rcv_finish_core(net, sk, skb); }
        dst = skb_dst(skb);
        if curr_dst != dst {
            hint = ip6_extract_route_hint(net, skb);
            if !list_empty(&sublist) { ip6_sublist_rcv_finish(&mut sublist); }
            INIT_LIST_HEAD!(&mut sublist);
            curr_dst = dst;
        }
        list_add_tail(&mut (*skb).list, &mut sublist);
    });
    ip6_sublist_rcv_finish(&mut sublist);
}

unsafe fn ip6_rcv_core(mut skb: *mut sk_buff, dev: *mut net_device, net: *mut net) -> *mut sk_buff {
    let mut reason: skb_drop_reason = NOT_SPECIFIED;
    let hdr: *const ipv6hdr;
    let mut pkt_len: u32;
    let idev: *mut inet6_dev;
    if (*skb).pkt_type == PACKET_OTHERHOST { dev_core_stats_rx_otherhost_dropped_inc((*skb).dev); kfree_skb_reason(skb, SKB_DROP_REASON_OTHERHOST); return core::ptr::null_mut(); }
    rcu_read_lock();
    idev = __in6_dev_get((*skb).dev);
    __IP6_UPD_PO_STATS!(net, idev, IPSTATS_MIB_IN, (*skb).len);
    if { skb = skb_share_check(skb, GFP_ATOMIC); skb.is_null() } || idev.is_null() || unlikely(READ_ONCE((*idev).cnf.disable_ipv6)) {
        __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INDISCARDS);
        if !idev.is_null() && unlikely(READ_ONCE((*idev).cnf.disable_ipv6)) { reason = IPV6DISABLED; }
        rcu_read_unlock(); kfree_skb_reason(skb, reason); return core::ptr::null_mut();
    }
    core::ptr::write_bytes(IP6CB(skb) as *mut u8, 0, core::mem::size_of::<inet6_skb_parm>());
    (*IP6CB(skb)).iif = if skb_valid_dst(skb) { (*ip6_dst_idev(skb_dst(skb))).dev.ifindex } else { (*dev).ifindex };
    if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    hdr = ipv6_hdr(skb);
    if (*hdr).version != 6 { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    __IP6_ADD_STATS!(net, idev, IPSTATS_MIB_NOECTPKTS + (ipv6_get_dsfield(hdr) & INET_ECN_MASK), core::cmp::max(1, (*skb_shinfo(skb)).gso_segs));
    if (ipv6_addr_loopback(&(*hdr).saddr) || ipv6_addr_loopback(&(*hdr).daddr)) && ((*dev).flags & IFF_LOOPBACK) == 0 && !netif_is_l3_master(dev) { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    if ((*skb).pkt_type != PACKET_LOOPBACK && ((*dev).flags & IFF_LOOPBACK) == 0) && ipv6_addr_is_multicast(&(*hdr).daddr) && IPV6_ADDR_MC_SCOPE!(&(*hdr).daddr) == 1 { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    if !ipv6_addr_is_multicast(&(*hdr).daddr) && ((*skb).pkt_type == PACKET_BROADCAST || (*skb).pkt_type == PACKET_MULTICAST) && READ_ONCE((*idev).cnf.drop_unicast_in_l2_multicast) { rcu_read_unlock(); kfree_skb_reason(skb, UNICAST_IN_L2_MULTICAST); return core::ptr::null_mut(); }
    if ipv6_addr_is_multicast(&(*hdr).daddr) && IPV6_ADDR_MC_SCOPE!(&(*hdr).daddr) == 0 { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    if ipv6_addr_is_multicast(&(*hdr).saddr) { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    (*skb).transport_header = (*skb).network_header + core::mem::size_of::<ipv6hdr>();
    (*IP6CB(skb)).nhoff = core::mem::offset_of!(ipv6hdr, nexthdr);
    pkt_len = ipv6_payload_len(skb, hdr);
    if pkt_len != 0 || (*hdr).nexthdr != NEXTHDR_HOP {
        if pkt_len + core::mem::size_of::<ipv6hdr>() > (*skb).len { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INTRUNCATEDPKTS); rcu_read_unlock(); kfree_skb_reason(skb, PKT_TOO_SMALL); return core::ptr::null_mut(); }
        if pskb_trim_rcsum(skb, pkt_len + core::mem::size_of::<ipv6hdr>()) != 0 { rcu_read_unlock(); kfree_skb_reason(skb, reason | IP_INHDR); return core::ptr::null_mut(); }
    }
    if (*hdr).nexthdr == NEXTHDR_HOP && ipv6_parse_hopopts(skb) < 0 { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INHDRERRORS); rcu_read_unlock(); return core::ptr::null_mut(); }
    rcu_read_unlock();
    if !skb_sk_is_prefetched(skb) { skb_orphan(skb); }
    skb
}

pub unsafe fn ipv6_rcv(skb: *mut sk_buff, dev: *mut net_device, pt: *mut packet_type, orig_dev: *mut net_device) -> c_int {
    let net = dev_net((*skb).dev); let skb = ip6_rcv_core(skb, dev, net); if skb.is_null() { return NET_RX_DROP; }
    NF_HOOK(NFPROTO_IPV6, NF_INET_PRE_ROUTING, net, core::ptr::null_mut(), skb, dev, core::ptr::null_mut(), ip6_rcv_finish)
}

unsafe fn ip6_sublist_rcv(head: *mut list_head, dev: *mut net_device, net: *mut net) {
    NF_HOOK_LIST(NFPROTO_IPV6, NF_INET_PRE_ROUTING, net, core::ptr::null_mut(), head, dev, core::ptr::null_mut(), ip6_rcv_finish);
    ip6_list_rcv_finish(net, core::ptr::null_mut(), head);
}

pub unsafe fn ipv6_list_rcv(head: *mut list_head, pt: *mut packet_type, orig_dev: *mut net_device) {
    let mut curr_dev = core::ptr::null_mut(); let mut curr_net = core::ptr::null_mut();
    let mut skb: *mut sk_buff; let mut next: *mut sk_buff; let mut sublist: list_head = LIST_HEAD_INIT!();
    list_for_each_entry_safe!(skb, next, head, list, {
        let dev = (*skb).dev; let net = dev_net(dev); skb_list_del_init(skb); skb = ip6_rcv_core(skb, dev, net); if skb.is_null() { continue; }
        if curr_dev != dev || curr_net != net { if !list_empty(&sublist) { ip6_sublist_rcv(&mut sublist, curr_dev, curr_net); } INIT_LIST_HEAD!(&mut sublist); curr_dev = dev; curr_net = net; }
        list_add_tail(&mut (*skb).list, &mut sublist);
    });
    if !list_empty(&sublist) { ip6_sublist_rcv(&mut sublist, curr_dev, curr_net); }
}

pub unsafe fn ip6_protocol_deliver_rcu(net: *mut net, skb: *mut sk_buff, mut nexthdr: c_int, mut have_final: bool) {
    let mut exthdr_cnt = if ((*IP6CB(skb)).flags & IP6SKB_HOPBYHOP) != 0 { 1 } else { 0 };
    let mut reason: skb_drop_reason = NOT_SPECIFIED;
    let mut idev: *mut inet6_dev; let mut nhoff: usize; let mut raw: bool;
    'resubmit: loop {
        idev = ip6_dst_idev(skb_dst(skb)); nhoff = (*IP6CB(skb)).nhoff;
        if !have_final { if !pskb_pull(skb, skb_transport_offset(skb)) { break 'discard; } nexthdr = (*skb_network_header(skb).add(nhoff)) as c_int; }
        'resubmit_final: loop {
            raw = raw6_local_deliver(skb, nexthdr); let ipprot = rcu_dereference(inet6_protos[nexthdr as usize]);
            if !ipprot.is_null() {
                let ret = INDIRECT_CALL_2!((*ipprot).handler, tcp_v6_rcv, udpv6_rcv, skb);
                if ret > 0 { if ((*ipprot).flags & INET6_PROTO_FINAL) != 0 { nexthdr = ret; continue 'resubmit_final; } else { if exthdr_cnt >= IP6_MAX_EXT_HDRS_CNT { reason = IPV6_TOO_MANY_EXTHDRS; break 'discard; } exthdr_cnt += 1; continue 'resubmit; } }
                if ret == 0 { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INDELIVERS); }
            } else if !raw { if xfrm6_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb) { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INUNKNOWNPROTOS); icmpv6_send(skb, ICMPV6_PARAMPROB, ICMPV6_UNK_NEXTHDR, nhoff); reason = IP_NOPROTO; } else { reason = XFRM_POLICY; } kfree_skb_reason(skb, reason); } else { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INDELIVERS); consume_skb(skb); }
            return;
        }
    }
    'discard: { __IP6_INC_STATS!(net, idev, IPSTATS_MIB_INDISCARDS); kfree_skb_reason(skb, reason); }
}

unsafe fn ip6_input_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int {
    if unlikely(skb_orphan_frags_rx(skb, GFP_ATOMIC)) { __IP6_INC_STATS!(net, ip6_dst_idev(skb_dst(skb)), IPSTATS_MIB_INDISCARDS); kfree_skb_reason(skb, SKB_DROP_REASON_NOMEM); return 0; }
    skb_clear_delivery_time(skb); ip6_protocol_deliver_rcu(net, skb, 0, false); 0
}

pub unsafe fn ip6_input(skb: *mut sk_buff) -> c_int {
    rcu_read_lock(); let res = NF_HOOK(NFPROTO_IPV6, NF_INET_LOCAL_IN, dev_net_rcu((*skb).dev), core::ptr::null_mut(), skb, (*skb).dev, core::ptr::null_mut(), ip6_input_finish); rcu_read_unlock(); res
}

pub unsafe fn ip6_mc_input(mut skb: *mut sk_buff) -> c_int {
    let mut dev = (*skb).dev; let sdif = inet6_sdif(skb);
    __IP6_UPD_PO_STATS!(skb_dst_dev_net_rcu(skb), __in6_dev_get_safely(dev), IPSTATS_MIB_INMCAST, (*skb).len);
    if sdif != 0 { dev = dev_get_by_index_rcu(dev_net_rcu(dev), sdif); if dev.is_null() { kfree_skb(skb); return -ENODEV; } }
    let hdr = ipv6_hdr(skb); let mut deliver = ipv6_chk_mcast_addr(dev, &(*hdr).daddr, core::ptr::null());
    /* CONFIG_IPV6_MROUTE is preserved by the external build configuration. */
    if atomic_read(&(*dev_net_rcu((*skb).dev)).ipv6.devconf_all.mc_forwarding) && (ipv6_addr_type(&(*hdr).daddr) & (IPV6_ADDR_LOOPBACK | IPV6_ADDR_LINKLOCAL)) == 0 && likely(((*IP6CB(skb)).flags & IP6SKB_FORWARDED) == 0) {
        let mut skb2: *mut sk_buff; let opt = IP6CB(skb);
        if unlikely(((*opt).flags & IP6SKB_ROUTERALERT) != 0) && (*opt).ra == htons(IPV6_OPT_ROUTERALERT_MLD) { deliver = false; let mut nexthdr = (*hdr).nexthdr; let mut frag_off: __be16 = 0; if !ipv6_ext_hdr(nexthdr) { return 0; } let offset = ipv6_skip_exthdr(skb, core::mem::size_of::<ipv6hdr>(), &mut nexthdr, &mut frag_off); if offset >= 0 && ipv6_is_mld(skb, nexthdr, offset) { deliver = true; } } else { if deliver { skb2 = skb_clone(skb, GFP_ATOMIC); } else { skb_orphan(skb); skb2 = skb; skb = core::ptr::null_mut(); } if !skb2.is_null() { ip6_mr_input(skb2); } }
    }
    if likely(deliver) { ip6_input(skb); } else { kfree_skb(skb); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
