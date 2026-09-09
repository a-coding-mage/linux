// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Handle firewalling
 *	Linux ethernet bridge
 *
 *	Authors:
 *	Lennert Buytenhek		<buytenh@gnu.org>
 *	Bart De Schuymer		<bdschuym@pandora.be>
 *
 *	Lennert dedicates this file to Kerstin Wurdinger.
 */

// Linux kernel headers and local dependencies from the original translation unit.

pub unsafe fn br_validate_ipv6(net: *mut net, skb: *mut sk_buff) -> i32 {
    let hdr: *const ipv6hdr;
    let idev: *mut inet6_dev = __in6_dev_get((*skb).dev);
    let pkt_len: u32;
    let ip6h_len: u8 = core::mem::size_of::<ipv6hdr>() as u8;

    if !pskb_may_pull(skb, ip6h_len) {
        goto_inhdr_error!();
    }

    if (*skb).len < ip6h_len {
        goto_drop!();
    }

    hdr = ipv6_hdr(skb);

    if (*hdr).version != 6 {
        goto_inhdr_error!();
    }

    pkt_len = ipv6_payload_len(skb, hdr);
    if (*hdr).nexthdr == NEXTHDR_HOP && nf_ip6_check_hbh_len(skb, &mut pkt_len) {
        goto_drop!();
    }

    if pkt_len + ip6h_len as u32 > (*skb).len {
        __IP6_INC_STATS(net, idev, IPSTATS_MIB_INTRUNCATEDPKTS);
        goto_drop!();
    }
    if pskb_trim_rcsum(skb, pkt_len + ip6h_len as u32) != 0 {
        __IP6_INC_STATS(net, idev, IPSTATS_MIB_INDISCARDS);
        goto_drop!();
    }

    memset(IP6CB(skb), 0, core::mem::size_of::<inet6_skb_parm>());
    /* No IP options in IPv6 header; however it should be
     * checked if some next headers need special treatment
     */
    return 0;

    // C goto targets are represented by the macros below.
    #[allow(unreachable_code)]
    {
        return -1;
    }
}

#[inline]
unsafe fn br_nf_ipv6_daddr_was_changed(
    skb: *const sk_buff,
    nf_bridge: *const nf_bridge_info,
) -> bool {
    memcmp(
        &(*nf_bridge).ipv6_daddr as *const _ as *const core::ffi::c_void,
        &ipv6_hdr(skb as *mut sk_buff).read().daddr as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&(*ipv6_hdr(skb as *mut sk_buff)).daddr),
    ) != 0
}

/* PF_BRIDGE/PRE_ROUTING: Undo the changes made for ip6tables
 * PREROUTING and continue the bridge PRE_ROUTING hook. See comment
 * for br_nf_pre_routing_finish(), same logic is used here.
 */
unsafe fn br_nf_pre_routing_finish_ipv6(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    let nf_bridge: *mut nf_bridge_info = nf_bridge_info_get(skb);
    let mut rt: *mut rtable;
    let dev: *mut net_device = (*skb).dev;
    let br_indev: *mut net_device;

    br_indev = nf_bridge_get_physindev(skb, net);
    if br_indev.is_null() {
        kfree_skb(skb);
        return 0;
    }

    (*nf_bridge).frag_max_size = (*IP6CB(skb)).frag_max_size;

    if (*nf_bridge).pkt_otherhost {
        (*skb).pkt_type = PACKET_OTHERHOST;
        (*nf_bridge).pkt_otherhost = false;
    }
    (*nf_bridge).in_prerouting = 0;
    if br_nf_ipv6_daddr_was_changed(skb, nf_bridge) {
        skb_dst_drop(skb);
        ip6_route_input(skb);

        if (*skb_dst(skb)).error != 0 {
            kfree_skb(skb);
            return 0;
        }

        if (*skb_dst(skb)).dev == dev {
            (*skb).dev = br_indev;
            nf_bridge_update_protocol(skb);
            nf_bridge_push_encap_header(skb);
            br_nf_hook_thresh(NF_BR_PRE_ROUTING, net, sk, skb, (*skb).dev, core::ptr::null_mut(), br_nf_pre_routing_finish_bridge);
            return 0;
        }
        ether_addr_copy((*eth_hdr(skb)).h_dest.as_mut_ptr(), (*dev).dev_addr.as_ptr());
        (*skb).pkt_type = PACKET_HOST;
    } else {
        rt = bridge_parent_rtable(br_indev);
        if rt.is_null() {
            kfree_skb(skb);
            return 0;
        }
        skb_dst_drop(skb);
        skb_dst_set_noref(skb, &mut (*rt).dst);
    }

    (*skb).dev = br_indev;
    nf_bridge_update_protocol(skb);
    nf_bridge_push_encap_header(skb);
    br_nf_hook_thresh(NF_BR_PRE_ROUTING, net, sk, skb, (*skb).dev, core::ptr::null_mut(), br_handle_frame_finish);

    0
}

/* Replicate the checks that IPv6 does on packet reception and pass the packet
 * to ip6tables.
 */
pub unsafe fn br_nf_pre_routing_ipv6(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    let mut nf_bridge: *mut nf_bridge_info;

    if br_validate_ipv6((*state).net, skb) != 0 {
        return NF_DROP_REASON(skb, SKB_DROP_REASON_IP_INHDR, 0);
    }

    nf_bridge = nf_bridge_alloc(skb);
    if nf_bridge.is_null() {
        return NF_DROP_REASON(skb, SKB_DROP_REASON_NOMEM, 0);
    }
    if !setup_pre_routing(skb, (*state).net) {
        return NF_DROP_REASON(skb, SKB_DROP_REASON_DEV_READY, 0);
    }

    nf_bridge = nf_bridge_info_get(skb);
    (*nf_bridge).ipv6_daddr = (*ipv6_hdr(skb)).daddr;

    (*skb).protocol = htons(ETH_P_IPV6);
    (*skb).transport_header = (*skb).network_header + core::mem::size_of::<ipv6hdr>();

    NF_HOOK(NFPROTO_IPV6, NF_INET_PRE_ROUTING, (*state).net, (*state).sk, skb,
        (*skb).dev, core::ptr::null_mut(), br_nf_pre_routing_finish_ipv6);

    NF_STOLEN
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
