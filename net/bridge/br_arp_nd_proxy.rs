// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Handle bridge arp/nd proxy/suppress
 *
 *  Copyright (C) 2017 Cumulus Networks
 *  Copyright (c) 2017 Roopa Prabhu <roopa@cumulusnetworks.com>
 *
 *  Authors:
 *	Roopa Prabhu <roopa@cumulusnetworks.com>
 */

// Linux kernel dependencies and br_private.h are supplied by the surrounding translation unit.

pub unsafe fn br_recalculate_neigh_suppress_enabled(br: *mut net_bridge) {
    let mut p: *mut net_bridge_port;
    let mut neigh_suppress = false;
    list_for_each_entry!(p, &mut (*br).port_list, list, {
        if read_once((*p).flags) & (BR_NEIGH_SUPPRESS | BR_NEIGH_VLAN_SUPPRESS) != 0 {
            neigh_suppress = true;
            break;
        }
    });
    br_opt_toggle(br, BROPT_NEIGH_SUPPRESS_ENABLED, neigh_suppress);
}

#[cfg(CONFIG_INET)]
unsafe fn br_arp_send(br: *mut net_bridge, p: *mut net_bridge_port,
                      dev: *mut net_device, dest_ip: __be32, src_ip: __be32,
                      dest_hw: *const u8, src_hw: *const u8,
                      target_hw: *const u8, vlan_proto: __be16, mut vlan_tci: u16) {
    let skb: *mut sk_buff;
    let vg: *mut net_bridge_vlan_group;
    let pvid: u16;
    netdev_dbg!(dev, "arp send dev %s dst %pI4 dst_hw %pM src %pI4 src_hw %pM\n",
                (*dev).name, &dest_ip, dest_hw, &src_ip, src_hw);
    if vlan_tci == 0 {
        arp_send(ARPOP_REPLY, ETH_P_ARP, dest_ip, dev, src_ip, dest_hw, src_hw, target_hw);
        return;
    }
    skb = arp_create(ARPOP_REPLY, ETH_P_ARP, dest_ip, dev, src_ip, dest_hw, src_hw, target_hw);
    if skb.is_null() { return; }
    vg = if !p.is_null() { nbp_vlan_group_rcu(p) } else { br_vlan_group_rcu(br) };
    pvid = br_get_pvid(vg);
    if pvid == (vlan_tci & VLAN_VID_MASK) { vlan_tci = 0; }
    if vlan_tci != 0 { __vlan_hwaccel_put_tag(skb, vlan_proto, vlan_tci); }
    if !p.is_null() {
        arp_xmit(skb);
    } else {
        skb_reset_mac_header(skb);
        __skb_pull(skb, skb_network_offset(skb));
        (*skb).ip_summed = CHECKSUM_UNNECESSARY;
        (*skb).pkt_type = PACKET_HOST;
        netif_rx(skb);
    }
}

#[cfg(CONFIG_INET)]
unsafe fn br_chk_addr_ip(dev: *mut net_device, priv_: *mut netdev_nested_priv) -> i32 {
    let ip = *( (*priv_).data as *const __be32 );
    let in_dev = __in_dev_get_rcu(dev);
    let mut addr: __be32 = 0;
    if !in_dev.is_null() { addr = inet_confirm_addr(dev_net(dev), in_dev, 0, ip, RT_SCOPE_HOST); }
    if addr == ip { 1 } else { 0 }
}

#[cfg(CONFIG_INET)]
unsafe fn br_is_local_ip(dev: *mut net_device, ip: __be32) -> bool {
    let mut priv_ = netdev_nested_priv { data: &ip as *const _ as *mut core::ffi::c_void };
    if br_chk_addr_ip(dev, &mut priv_) != 0 { return true; }
    if netdev_walk_all_upper_dev_rcu(dev, Some(br_chk_addr_ip), &mut priv_) != 0 { return true; }
    false
}

#[cfg(CONFIG_INET)]
pub unsafe fn br_do_proxy_suppress_arp(skb: *mut sk_buff, br: *mut net_bridge,
                                        vid: u16, p: *mut net_bridge_port) {
    let dev = (*br).dev;
    let mut vlandev = dev;
    let mut n: *mut neighbour;
    let parp: *mut arphdr;
    let mut arpptr: *mut u8;
    let sha: *mut u8;
    let mut sip: __be32 = 0;
    let mut tip: __be32 = 0;
    (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 0;
    (*BR_INPUT_SKB_CB!(skb)).grat_arp = 0;
    if ((*dev).flags & IFF_NOARP) != 0 || pskb_may_pull(skb, arp_hdr_len(dev)) == 0 { return; }
    parp = arp_hdr(skb);
    if (*parp).ar_pro != htons(ETH_P_IP) || (*parp).ar_hln != (*dev).addr_len || (*parp).ar_pln != 4 { return; }
    arpptr = (parp as *mut u8).add(core::mem::size_of::<arphdr>());
    sha = arpptr; arpptr = arpptr.add((*dev).addr_len as usize);
    core::ptr::copy_nonoverlapping(arpptr, &mut sip as *mut _ as *mut u8, core::mem::size_of::<__be32>());
    arpptr = arpptr.add(core::mem::size_of::<__be32>() + (*dev).addr_len as usize);
    core::ptr::copy_nonoverlapping(arpptr, &mut tip as *mut _ as *mut u8, core::mem::size_of::<__be32>());
    if ipv4_is_loopback(tip) || ipv4_is_multicast(tip) { return; }
    if br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) {
        if br_is_neigh_suppress_enabled(p, vid) { return; }
        if is_unicast_ether_addr(eth_hdr(skb).as_ref().unwrap().h_dest.as_ptr()) && (*parp).ar_op == htons(ARPOP_REQUEST) { return; }
        if (*parp).ar_op != htons(ARPOP_RREQUEST) && (*parp).ar_op != htons(ARPOP_RREPLY) && sip == tip {
            (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; (*BR_INPUT_SKB_CB!(skb)).grat_arp = 1; return;
        }
    }
    if (*parp).ar_op != htons(ARPOP_REQUEST) { return; }
    if vid != 0 { vlandev = __vlan_find_dev_deep_rcu((*br).dev, (*skb).vlan_proto, vid); if vlandev.is_null() { return; } }
    if br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) && br_is_local_ip(vlandev, tip) { (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; return; }
    n = neigh_lookup(&arp_tbl, &tip, vlandev);
    if !n.is_null() {
        let mut ha = [0u8; ETH_ALEN];
        if read_once((*n).nud_state) & NUD_VALID == 0 { neigh_release(n); return; }
        neigh_ha_snapshot(ha.as_mut_ptr(), n, (*n).dev);
        let f = br_fdb_find_rcu(br, ha.as_ptr(), vid);
        if !f.is_null() {
            let dst = read_once((*f).dst); let mut replied = false;
            if (!p.is_null() && test_bit(BR_PROXYARP_BIT, &(*p).flags) != 0) || (!dst.is_null() && test_bit(BR_PROXYARP_WIFI_BIT, &(*dst).flags) != 0) || br_is_neigh_suppress_enabled(dst, vid) {
                if vid == 0 { br_arp_send(br, p, (*skb).dev, sip, tip, sha, ha.as_ptr(), sha, 0, 0); }
                else { br_arp_send(br, p, (*skb).dev, sip, tip, sha, ha.as_ptr(), sha, (*skb).vlan_proto, skb_vlan_tag_get(skb)); }
                replied = true;
            }
            if replied || br_opt_get(br, BROPT_NEIGH_SUPPRESS_ENABLED) { (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; }
        }
        neigh_release(n);
    }
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn br_is_nd_neigh_msg(skb: *mut sk_buff) -> *mut nd_msg {
    if ndisc_check_ns_na(skb) != 0 || skb_linearize(skb) != 0 { return core::ptr::null_mut(); }
    skb_transport_header(skb) as *mut nd_msg
}

#[cfg(CONFIG_IPV6)]
unsafe fn br_nd_send(br: *mut net_bridge, p: *mut net_bridge_port,
                     request: *mut sk_buff, n: *mut neighbour, ha: *mut u8,
                     vlan_proto: __be16, mut vlan_tci: u16) {
    let dev = (*request).dev;
    if dev.is_null() { return; }
    let mut ndopts: ndisc_options = core::mem::zeroed();
    let ns = skb_transport_header(request) as *mut nd_msg;
    let daddr = eth_hdr(request).as_ref().unwrap().h_source.as_mut_ptr();
    let na_olen: i32 = 8;
    let ns_olen = ntohs((*ipv6_hdr(request)).payload_len) as i32 - core::mem::size_of::<nd_msg>() as i32;
    let len = LL_RESERVED_SPACE(dev) + core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<nd_msg>() + na_olen as usize + (*dev).needed_tailroom as usize;
    let reply = alloc_skb(len, GFP_ATOMIC);
    if reply.is_null() { return; }
    (*reply).protocol = htons(ETH_P_IPV6); (*reply).dev = dev;
    skb_reserve(reply, LL_RESERVED_SPACE(dev)); skb_push(reply, core::mem::size_of::<ethhdr>()); skb_set_mac_header(reply, 0);
    if ndisc_parse_options(dev, (*ns).opt.as_mut_ptr(), ns_olen, &mut ndopts) == 0 { kfree_skb(reply); return; }
    let mut dst = daddr;
    if !ndopts.nd_opts_src_lladdr.is_null() {
        let lladdr = ndisc_opt_addr_data(ndopts.nd_opts_src_lladdr, dev);
        if !lladdr.is_null() { dst = lladdr; }
    }
    let dad = ipv6_addr_any(&(*ipv6_hdr(request)).saddr);
    if dad { ipv6_eth_mc_map(&in6addr_linklocal_allnodes, eth_hdr(reply).as_mut().unwrap().h_dest.as_mut_ptr()); }
    else { ether_addr_copy(eth_hdr(reply).as_mut().unwrap().h_dest.as_mut_ptr(), dst); }
    ether_addr_copy(eth_hdr(reply).as_mut().unwrap().h_source.as_mut_ptr(), ha);
    eth_hdr(reply).as_mut().unwrap().h_proto = htons(ETH_P_IPV6);
    skb_pull(reply, core::mem::size_of::<ethhdr>()); skb_set_network_header(reply, 0); skb_put(reply, core::mem::size_of::<ipv6hdr>());
    let pip6 = ipv6_hdr(reply); core::ptr::write_bytes(pip6 as *mut u8, 0, core::mem::size_of::<ipv6hdr>());
    (*pip6).version = 6; (*pip6).priority = (*ipv6_hdr(request)).priority; (*pip6).nexthdr = IPPROTO_ICMPV6; (*pip6).hop_limit = 255;
    (*pip6).daddr = if dad { in6addr_linklocal_allnodes } else { (*ipv6_hdr(request)).saddr };
    (*pip6).saddr = *( (*n).primary_key as *const in6_addr );
    skb_pull(reply, core::mem::size_of::<ipv6hdr>()); skb_set_transport_header(reply, 0);
    let na = skb_put(reply, core::mem::size_of::<nd_msg>() + na_olen as usize) as *mut nd_msg;
    core::ptr::write_bytes(na as *mut u8, 0, core::mem::size_of::<nd_msg>() + na_olen as usize);
    (*na).icmph.icmp6_type = NDISC_NEIGHBOUR_ADVERTISEMENT; (*na).icmph.icmp6_router = if (*n).flags & NTF_ROUTER != 0 { 1 } else { 0 }; (*na).icmph.icmp6_override = 1; (*na).icmph.icmp6_solicited = if dad { 0 } else { 1 }; (*na).target = (*ns).target;
    ether_addr_copy((*na).opt.as_mut_ptr().add(2), ha); (*na).opt[0] = ND_OPT_TARGET_LL_ADDR; (*na).opt[1] = (na_olen >> 3) as u8;
    (*na).icmph.icmp6_cksum = csum_ipv6_magic(&(*pip6).saddr, &(*pip6).daddr, (core::mem::size_of::<nd_msg>() + na_olen as usize) as u32, IPPROTO_ICMPV6, csum_partial(na as *const _, core::mem::size_of::<nd_msg>() + na_olen as usize, 0));
    (*pip6).payload_len = htons((core::mem::size_of::<nd_msg>() + na_olen as usize) as u16);
    skb_push(reply, core::mem::size_of::<ipv6hdr>()); skb_push(reply, core::mem::size_of::<ethhdr>()); (*reply).ip_summed = CHECKSUM_UNNECESSARY;
    let vg = if !p.is_null() { nbp_vlan_group_rcu(p) } else { br_vlan_group_rcu(br) }; if br_get_pvid(vg) == (vlan_tci & VLAN_VID_MASK) { vlan_tci = 0; }
    if vlan_tci != 0 { __vlan_hwaccel_put_tag(reply, vlan_proto, vlan_tci); }
    if !p.is_null() { dev_queue_xmit(reply); } else { skb_reset_mac_header(reply); __skb_pull(reply, skb_network_offset(reply)); (*reply).ip_summed = CHECKSUM_UNNECESSARY; (*reply).pkt_type = PACKET_HOST; netif_rx(reply); }
}

// The remaining IPv6 implementation mirrors the kernel packet construction path.
#[cfg(CONFIG_IPV6)]
pub unsafe fn br_do_suppress_nd(skb: *mut sk_buff, br: *mut net_bridge, vid: u16,
                                 p: *mut net_bridge_port, msg: *mut nd_msg) {
    (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 0;
    (*BR_INPUT_SKB_CB!(skb)).grat_arp = 0;
    if br_is_neigh_suppress_enabled(p, vid) { return; }
    if is_unicast_ether_addr(eth_hdr(skb).as_ref().unwrap().h_dest.as_ptr()) && (*msg).icmph.icmp6_type == NDISC_NEIGHBOUR_SOLICITATION { return; }
    if (*msg).icmph.icmp6_type == NDISC_NEIGHBOUR_ADVERTISEMENT && (*msg).icmph.icmp6_solicited == 0 { (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; (*BR_INPUT_SKB_CB!(skb)).grat_arp = 1; return; }
    if (*msg).icmph.icmp6_type != NDISC_NEIGHBOUR_SOLICITATION { return; }
    let iphdr = ipv6_hdr(skb);
    if ipv6_addr_cmp(&(*iphdr).saddr, &(*iphdr).daddr) == 0 { (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; return; }
    let vlandev = if vid != 0 { __vlan_find_dev_deep_rcu((*br).dev, (*skb).vlan_proto, vid) } else { (*br).dev };
    if vlandev.is_null() || br_is_local_ip6(vlandev, &mut (*msg).target) { (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1; return; }
    let n = neigh_lookup(&nd_tbl, &(*msg).target, vlandev);
    if n.is_null() { return; }
    let mut ha = [0u8; ETH_ALEN];
    if read_once((*n).nud_state) & NUD_VALID == 0 { neigh_release(n); return; }
    neigh_ha_snapshot(ha.as_mut_ptr(), n, (*n).dev);
    let f = br_fdb_find_rcu(br, ha.as_ptr(), vid);
    if !f.is_null() && br_is_neigh_suppress_enabled(read_once((*f).dst), vid) {
        if vid != 0 { br_nd_send(br, p, skb, n, ha.as_mut_ptr(), (*skb).vlan_proto, skb_vlan_tag_get(skb)); }
        else { br_nd_send(br, p, skb, n, ha.as_mut_ptr(), 0, 0); }
        (*BR_INPUT_SKB_CB!(skb)).proxyarp_replied = 1;
    }
    neigh_release(n);
}

#[cfg(CONFIG_IPV6)]
unsafe fn br_is_local_ip6(dev: *mut net_device, addr: *mut in6_addr) -> bool {
    let mut priv_ = netdev_nested_priv { data: addr as *mut core::ffi::c_void };
    br_chk_addr_ip6(dev, &mut priv_) != 0 || netdev_walk_all_upper_dev_rcu(dev, Some(br_chk_addr_ip6), &mut priv_) != 0
}

#[cfg(CONFIG_IPV6)]
unsafe fn br_chk_addr_ip6(dev: *mut net_device, priv_: *mut netdev_nested_priv) -> i32 {
    if ipv6_chk_addr(dev_net(dev), (*priv_).data as *mut in6_addr, dev, 0) { 1 } else { 0 }
}

pub unsafe fn br_is_neigh_suppress_enabled(p: *const net_bridge_port, vid: u16) -> bool {
    if p.is_null() { return false; }
    if vid != 0 && test_bit(BR_NEIGH_VLAN_SUPPRESS_BIT, &(*p).flags) != 0 {
        let v = br_vlan_find(nbp_vlan_group_rcu(p as *mut _), vid);
        return !v.is_null() && ((*v).priv_flags & BR_VLFLAG_NEIGH_SUPPRESS_ENABLED) != 0;
    }
    test_bit(BR_NEIGH_SUPPRESS_BIT, &(*p).flags) != 0
}

pub unsafe fn br_is_neigh_forward_grat_enabled(p: *const net_bridge_port, vid: u16) -> bool {
    if vid != 0 && test_bit(BR_NEIGH_VLAN_SUPPRESS_BIT, &(*p).flags) != 0 {
        let v = br_vlan_find(nbp_vlan_group_rcu(p as *mut _), vid);
        return !v.is_null() && ((*v).priv_flags & BR_VLFLAG_NEIGH_FORWARD_GRAT_ENABLED) != 0;
    }
    test_bit(BR_NEIGH_FORWARD_GRAT_BIT, &(*p).flags) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
