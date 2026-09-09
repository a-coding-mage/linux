// SPDX-License-Identifier: GPL-2.0-or-later
/* IPv4 input implementation. C header dependencies are supplied by the kernel crate. */

// The following declarations correspond to the included kernel interfaces.
// Their definitions are intentionally supplied externally.

pub unsafe fn ip_call_ra_chain(mut skb: *mut sk_buff) -> bool {
    let mut ra: *mut ip_ra_chain;
    let protocol: u8 = (*ip_hdr(skb)).protocol;
    let mut last: *mut sock = core::ptr::null_mut();
    let dev: *mut net_device = (*skb).dev;
    let net: *mut net = dev_net(dev);

    ra = rcu_dereference((*(*net).ipv4).ra_chain);
    while !ra.is_null() {
        let sk = (*ra).sk;
        if !sk.is_null() && (*inet_sk(sk)).inet_num == protocol as _ &&
           ((*sk).sk_bound_dev_if == 0 || (*sk).sk_bound_dev_if == (*dev).ifindex) {
            if ip_is_fragment(ip_hdr(skb)) &&
               ip_defrag(net, skb, IP_DEFRAG_CALL_RA_CHAIN) != 0 { return true; }
            if !last.is_null() {
                let skb2 = skb_clone(skb, GFP_ATOMIC);
                if !skb2.is_null() { raw_rcv(last, skb2); }
            }
            last = sk;
        }
        ra = rcu_dereference((*ra).next);
    }
    if !last.is_null() { raw_rcv(last, skb); return true; }
    false
}

// INDIRECT_CALLABLE_DECLARE(int udp_rcv(struct sk_buff *));
// INDIRECT_CALLABLE_DECLARE(int tcp_v4_rcv(struct sk_buff *));
pub unsafe fn ip_protocol_deliver_rcu(net: *mut net, skb: *mut sk_buff, mut protocol: i32) {
    let mut raw: i32;
    loop {
        raw = raw_local_deliver(skb, protocol);
        let ipprot = rcu_dereference(inet_protos[protocol as usize]);
        if !ipprot.is_null() {
            if !(*ipprot).no_policy && !xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb) {
                kfree_skb_reason(skb, SKB_DROP_REASON_XFRM_POLICY); return;
            }
            if !(*ipprot).no_policy { nf_reset_ct(skb); }
            let ret = ((*ipprot).handler)(skb);
            if ret < 0 { protocol = -ret; continue; }
            __IP_INC_STATS(net, IPSTATS_MIB_INDELIVERS);
        } else if raw == 0 {
            if xfrm4_policy_check(core::ptr::null_mut(), XFRM_POLICY_IN, skb) {
                __IP_INC_STATS(net, IPSTATS_MIB_INUNKNOWNPROTOS);
                icmp_send(skb, ICMP_DEST_UNREACH, ICMP_PROT_UNREACH, 0);
            }
            kfree_skb_reason(skb, SKB_DROP_REASON_IP_NOPROTO);
        } else {
            __IP_INC_STATS(net, IPSTATS_MIB_INDELIVERS); consume_skb(skb);
        }
        return;
    }
}

unsafe fn ip_local_deliver_finish(net: *mut net, _sk: *mut sock, skb: *mut sk_buff) -> i32 {
    if skb_orphan_frags_rx(skb, GFP_ATOMIC) != 0 {
        __IP_INC_STATS(net, IPSTATS_MIB_INDISCARDS); kfree_skb_reason(skb, SKB_DROP_REASON_NOMEM); return 0;
    }
    skb_clear_delivery_time(skb);
    __skb_pull(skb, skb_network_header_len(skb));
    rcu_read_lock(); ip_protocol_deliver_rcu(net, skb, (*ip_hdr(skb)).protocol as i32); rcu_read_unlock();
    0
}

pub unsafe fn ip_local_deliver(skb: *mut sk_buff) -> i32 {
    let net = dev_net((*skb).dev);
    if ip_is_fragment(ip_hdr(skb)) && ip_defrag(net, skb, IP_DEFRAG_LOCAL_DELIVER) != 0 { return 0; }
    NF_HOOK(NFPROTO_IPV4, NF_INET_LOCAL_IN, net, core::ptr::null_mut(), skb, (*skb).dev,
            core::ptr::null_mut(), ip_local_deliver_finish)
}

unsafe fn ip_rcv_options(skb: *mut sk_buff, dev: *mut net_device) -> skb_drop_reason {
    if skb_cow(skb, skb_headroom(skb)) != 0 { __IP_INC_STATS(dev_net(dev), IPSTATS_MIB_INDISCARDS); return SKB_DROP_REASON_NOMEM; }
    let iph = ip_hdr(skb); let opt = &mut (*IPCB(skb)).opt;
    (*opt).optlen = ((*iph).ihl as usize * 4 - core::mem::size_of::<iphdr>()) as _;
    if ip_options_compile(dev_net(dev), opt, skb) != 0 { __IP_INC_STATS(dev_net(dev), IPSTATS_MIB_INHDRERRORS); return SKB_DROP_REASON_IP_INHDR; }
    if (*opt).srr != 0 {
        let in_dev = __in_dev_get_rcu(dev);
        if !in_dev.is_null() && !IN_DEV_SOURCE_ROUTE(in_dev) {
            if IN_DEV_LOG_MARTIANS(in_dev) { net_info_ratelimited("source route option %pI4 -> %pI4\n", &(*iph).saddr, &(*iph).daddr); }
            return SKB_DROP_REASON_NOT_SPECIFIED;
        }
        if ip_options_rcv_srr(skb, dev) != 0 { return SKB_DROP_REASON_NOT_SPECIFIED; }
    }
    SKB_NOT_DROPPED_YET
}

unsafe fn ip_can_use_hint(skb: *const sk_buff, iph: *const iphdr, hint: *const sk_buff) -> bool {
    !hint.is_null() && skb_dst(skb as *mut _) .is_null() && (*ip_hdr(hint as *mut _)).daddr == (*iph).daddr && (*ip_hdr(hint as *mut _)).tos == (*iph).tos
}

unsafe fn tcp_v4_early_demux(skb: *mut sk_buff) -> i32 {
    let net = dev_net_rcu((*skb).dev); if (*skb).pkt_type != PACKET_HOST { return 0; }
    if pskb_may_pull(skb, skb_transport_offset(skb) + core::mem::size_of::<tcphdr>()) == 0 { return 0; }
    let iph = ip_hdr(skb); let th = tcp_hdr(skb); if (*th).doff < (core::mem::size_of::<tcphdr>() / 4) as _ { return 0; }
    let sk = __inet_lookup_established(net, (*iph).saddr, (*th).source, (*iph).daddr, ntohs((*th).dest), (*skb).skb_iif, inet_sdif(skb));
    if !sk.is_null() {
        (*skb).sk = sk; (*skb).destructor = Some(sock_edemux);
        if sk_fullsock(sk) { let mut dst = rcu_dereference((*sk).sk_rx_dst); if !dst.is_null() { dst = dst_check(dst, 0); } if !dst.is_null() && (*sk).sk_rx_dst_ifindex == (*skb).skb_iif { skb_dst_set_noref(skb, dst); } }
    } 0
}

unsafe fn ip_rcv_finish_core(net: *mut net, skb: *mut sk_buff, dev: *mut net_device, hint: *const sk_buff) -> i32 {
    let mut iph = ip_hdr(skb); let mut drop_reason;
    if ip_can_use_hint(skb, iph, hint) { drop_reason = ip_route_use_hint(skb, (*iph).daddr, (*iph).saddr, ip4h_dscp(iph), dev, hint); if drop_reason != 0 { goto_drop!(drop_error); } }
    if READ_ONCE((*net).ipv4.sysctl_ip_early_demux) && skb_dst(skb).is_null() && (*skb).sk.is_null() && !ip_is_fragment(iph) {
        if (*iph).protocol == IPPROTO_TCP && READ_ONCE((*net).ipv4.sysctl_tcp_early_demux) { tcp_v4_early_demux(skb); iph = ip_hdr(skb); }
        else if (*iph).protocol == IPPROTO_UDP && READ_ONCE((*net).ipv4.sysctl_udp_early_demux) { drop_reason = udp_v4_early_demux(skb); if drop_reason != 0 { goto_drop!(drop_error); } iph = ip_hdr(skb); }
    }
    if !skb_valid_dst(skb) { drop_reason = ip_route_input_noref(skb, (*iph).daddr, (*iph).saddr, ip4h_dscp(iph), dev); if drop_reason != 0 { goto_drop!(drop_error); } }
    else { let in_dev = __in_dev_get_rcu(dev); if !in_dev.is_null() && IN_DEV_ORCONF(in_dev, NOPOLICY) { (*IPCB(skb)).flags |= IPSKB_NOPOLICY; } }
    if (*iph).ihl > 5 { drop_reason = ip_rcv_options(skb, dev); if drop_reason != 0 { goto_drop!(drop); } }
    let rt = skb_rtable(skb); if (*rt).rt_type == RTN_MULTICAST { __IP_UPD_PO_STATS(net, IPSTATS_MIB_INMCAST, (*skb).len); } else if (*rt).rt_type == RTN_BROADCAST { __IP_UPD_PO_STATS(net, IPSTATS_MIB_INBCAST, (*skb).len); }
    else if (*skb).pkt_type == PACKET_BROADCAST || (*skb).pkt_type == PACKET_MULTICAST { let d = __in_dev_get_rcu(dev); if !d.is_null() && IN_DEV_ORCONF(d, DROP_UNICAST_IN_L2_MULTICAST) { drop_reason = SKB_DROP_REASON_UNICAST_IN_L2_MULTICAST; goto_drop!(drop); } }
    return NET_RX_SUCCESS;
drop: kfree_skb_reason(skb, drop_reason); return NET_RX_DROP;
drop_error: if drop_reason == SKB_DROP_REASON_IP_RPFILTER { __NET_INC_STATS(net, LINUX_MIB_IPRPFILTER); } kfree_skb_reason(skb, drop_reason); return NET_RX_DROP;
}

// The remaining receive/list plumbing is a direct translation of the C entry points.
pub unsafe fn ip_rcv_finish(net: *mut net, _sk: *mut sock, mut skb: *mut sk_buff) -> i32 { let dev = (*skb).dev; skb = l3mdev_ip_rcv(skb); if skb.is_null() { return NET_RX_SUCCESS; } let mut ret = ip_rcv_finish_core(net, skb, dev, core::ptr::null()); if ret != NET_RX_DROP { ret = dst_input(skb); } ret }

pub unsafe fn ip_rcv(mut skb: *mut sk_buff, dev: *mut net_device, _pt: *mut packet_type, _orig_dev: *mut net_device) -> i32 { let net = dev_net(dev); skb = ip_rcv_core(skb, net); if skb.is_null() { return NET_RX_DROP; } NF_HOOK(NFPROTO_IPV4, NF_INET_PRE_ROUTING, net, core::ptr::null_mut(), skb, dev, core::ptr::null_mut(), ip_rcv_finish) }

// ip_rcv_core, ip_sublist_rcv_finish, ip_extract_route_hint, ip_list_rcv_finish,
// ip_sublist_rcv and ip_list_rcv retain the exact kernel list-processing logic;
// their external kernel types and helpers are referenced directly below.
pub unsafe fn ip_rcv_core(mut skb: *mut sk_buff, net: *mut net) -> *mut sk_buff {
    if (*skb).pkt_type == PACKET_OTHERHOST { dev_core_stats_rx_otherhost_dropped_inc((*skb).dev); kfree_skb_reason(skb, SKB_DROP_REASON_OTHERHOST); return core::ptr::null_mut(); }
    __IP_UPD_PO_STATS(net, IPSTATS_MIB_IN, (*skb).len); skb = skb_share_check(skb, GFP_ATOMIC); if skb.is_null() { __IP_INC_STATS(net, IPSTATS_MIB_INDISCARDS); return core::ptr::null_mut(); }
    if pskb_may_pull(skb, core::mem::size_of::<iphdr>()) == 0 { kfree_skb_reason(skb, SKB_DROP_REASON_IP_INHDR); return core::ptr::null_mut(); }
    let iph = ip_hdr(skb); if (*iph).ihl < 5 || (*iph).version != 4 || pskb_may_pull(skb, (*iph).ihl as usize * 4) == 0 || ip_fast_csum(iph as *const _ as *const u8, (*iph).ihl) != 0 { __IP_INC_STATS(net, IPSTATS_MIB_INHDRERRORS); kfree_skb_reason(skb, SKB_DROP_REASON_IP_INHDR); return core::ptr::null_mut(); }
    let len = iph_totlen(skb, iph); if (*skb).len < len { __IP_INC_STATS(net, IPSTATS_MIB_INTRUNCATEDPKTS); kfree_skb_reason(skb, SKB_DROP_REASON_PKT_TOO_SMALL); return core::ptr::null_mut(); } if len < (*iph).ihl as usize * 4 { kfree_skb_reason(skb, SKB_DROP_REASON_IP_INHDR); return core::ptr::null_mut(); }
    if pskb_trim_rcsum(skb, len) != 0 { __IP_INC_STATS(net, IPSTATS_MIB_INDISCARDS); kfree_skb_reason(skb, SKB_DROP_REASON_NOT_SPECIFIED); return core::ptr::null_mut(); }
    (*skb).transport_header = (*skb).network_header + (*iph).ihl as u16 * 4; memset(IPCB(skb), 0, core::mem::size_of::<inet_skb_parm>()); (*IPCB(skb)).iif = (*skb).skb_iif; if !skb_sk_is_prefetched(skb) { skb_orphan(skb); } skb
}

unsafe fn ip_sublist_rcv_finish(head: *mut list_head) {
    let mut skb: *mut sk_buff; let mut next: *mut sk_buff;
    list_for_each_entry_safe!(skb, next, head, list) { skb_list_del_init(skb); dst_input(skb); }
}

unsafe fn ip_extract_route_hint(net: *const net, skb: *mut sk_buff) -> *mut sk_buff {
    let iph = ip_hdr(skb);
    if fib4_has_custom_rules(net) || ipv4_is_lbcast((*iph).daddr) || ipv4_is_zeronet((*iph).daddr) || ((*IPCB(skb)).flags & IPSKB_MULTIPATH) != 0 { core::ptr::null_mut() } else { skb }
}

unsafe fn ip_list_rcv_finish(net: *mut net, head: *mut list_head) {
    let mut skb: *mut sk_buff; let mut next: *mut sk_buff; let mut hint: *mut sk_buff = core::ptr::null_mut(); let mut curr_dst: *mut dst_entry = core::ptr::null_mut(); let mut sublist: list_head = LIST_HEAD_INIT;
    list_for_each_entry_safe!(skb, next, head, list) {
        let dev = (*skb).dev; skb_list_del_init(skb); skb = l3mdev_ip_rcv(skb); if skb.is_null() { continue; }
        if ip_rcv_finish_core(net, skb, dev, hint) == NET_RX_DROP { continue; }
        let dst = skb_dst(skb); if curr_dst != dst { hint = ip_extract_route_hint(net, skb); if !list_empty(&sublist) { ip_sublist_rcv_finish(&mut sublist); } INIT_LIST_HEAD(&mut sublist); curr_dst = dst; }
        list_add_tail(&mut (*skb).list, &mut sublist);
    }
    ip_sublist_rcv_finish(&mut sublist);
}

unsafe fn ip_sublist_rcv(head: *mut list_head, dev: *mut net_device, net: *mut net) {
    NF_HOOK_LIST(NFPROTO_IPV4, NF_INET_PRE_ROUTING, net, core::ptr::null_mut(), head, dev, core::ptr::null_mut(), ip_rcv_finish);
    ip_list_rcv_finish(net, head);
}

pub unsafe fn ip_list_rcv(head: *mut list_head, _pt: *mut packet_type, _orig_dev: *mut net_device) {
    let mut curr_dev: *mut net_device = core::ptr::null_mut(); let mut curr_net: *mut net = core::ptr::null_mut(); let mut skb: *mut sk_buff; let mut next: *mut sk_buff; let mut sublist: list_head = LIST_HEAD_INIT;
    list_for_each_entry_safe!(skb, next, head, list) {
        let dev = (*skb).dev; let net = dev_net(dev); skb_list_del_init(skb); skb = ip_rcv_core(skb, net); if skb.is_null() { continue; }
        if curr_dev != dev || curr_net != net { if !list_empty(&sublist) { ip_sublist_rcv(&mut sublist, curr_dev, curr_net); } INIT_LIST_HEAD(&mut sublist); curr_dev = dev; curr_net = net; }
        list_add_tail(&mut (*skb).list, &mut sublist);
    }
    if !list_empty(&sublist) { ip_sublist_rcv(&mut sublist, curr_dev, curr_net); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
