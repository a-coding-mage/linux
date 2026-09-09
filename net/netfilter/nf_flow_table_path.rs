// SPDX-License-Identifier: GPL-2.0-only
// External Linux kernel declarations and build-time configuration are supplied by other files.

unsafe fn nft_xmit_type(dst: *mut dst_entry) -> flow_offload_xmit_type {
    if dst_xfrm(dst) {
        return FLOW_OFFLOAD_XMIT_XFRM;
    }
    FLOW_OFFLOAD_XMIT_NEIGH
}

unsafe fn nft_default_forward_path(
    route: *mut nf_flow_route,
    dst_cache: *mut dst_entry,
    dir: ip_conntrack_dir,
) {
    (*route).tuple[!dir].in_.ifindex = (*(*dst_cache).dev).ifindex;
    (*route).tuple[dir].dst = dst_cache;
    (*route).tuple[dir].xmit_type = nft_xmit_type(dst_cache);
}

unsafe fn nft_is_valid_ether_device(dev: *const net_device) -> bool {
    if dev.is_null()
        || ((*dev).flags & IFF_LOOPBACK) != 0
        || (*dev).type_ != ARPHRD_ETHER
        || (*dev).addr_len != ETH_ALEN
        || !is_valid_ether_addr((*dev).dev_addr.as_ptr())
    {
        return false;
    }
    true
}

unsafe fn nft_dev_fill_forward_path(
    dst_cache: *const dst_entry,
    ct: *const nf_conn,
    dir: ip_conntrack_dir,
    ha: *mut u8,
    ether_type: __be16,
    stack: *mut net_device_path_stack,
) -> i32 {
    let daddr = &(*ct).tuplehash[!dir].tuple.src.u3 as *const _ as *const c_void;
    let dev = (*dst_cache).dev;
    let mut ctx = net_device_path_ctx { dev, ether_type, ..core::mem::zeroed() };
    let n: *mut neighbour;
    let nud_state: u8;

    if !nft_is_valid_ether_device(dev) {
        eth_zero_addr(ha);
        return dev_fill_forward_path(&mut ctx, stack);
    }

    n = dst_neigh_lookup(dst_cache, daddr);
    if n.is_null() {
        return -1;
    }
    read_lock_bh(&mut (*n).lock);
    nud_state = (*n).nud_state;
    ether_addr_copy(ha, (*n).ha.as_ptr());
    read_unlock_bh(&mut (*n).lock);
    neigh_release(n);

    if (nud_state & NUD_VALID) == 0 {
        return -1;
    }
    ether_addr_copy((*ctx.daddr).as_mut_ptr(), ha);
    dev_fill_forward_path(&mut ctx, stack)
}

#[repr(C)]
struct nft_forward_info {
    dev: *const net_device,
    encap: [nft_forward_info_id; NF_FLOW_TABLE_ENCAP_MAX],
    num_encaps: u8,
    tun: flow_offload_tunnel,
    tun_dst: *mut dst_entry,
    num_tuns: u8,
    ingress_vlans: u8,
    h_source: [u8; ETH_ALEN],
    h_dest: [u8; ETH_ALEN],
    needs_gso_segment: bool,
    xmit_type: flow_offload_xmit_type,
}

#[repr(C)]
struct nft_forward_info_id {
    id: u16,
    proto: __be16,
}

unsafe fn nft_flowtable_find_dev(dev: *const net_device, ft: *mut nft_flowtable) -> bool;

unsafe fn nft_dev_path_info(
    stack: *mut net_device_path_stack,
    info: *mut nft_forward_info,
    ha: *const u8,
    ft: *mut nft_flowtable,
) -> i32 {
    (*info).h_dest.copy_from_slice(core::slice::from_raw_parts(ha, ETH_ALEN));
    let mut i = 0;
    while i < (*stack).num_paths {
        let path = &(*stack).path[i as usize];
        match path.type_ {
            DEV_PATH_ETHERNET | DEV_PATH_DSA | DEV_PATH_VLAN | DEV_PATH_PPPOE | DEV_PATH_TUN => {
                (*info).dev = path.dev;
                if is_zero_ether_addr((*info).h_source.as_ptr()) {
                    (*info).h_source.copy_from_slice(&(*path.dev).dev_addr[..ETH_ALEN]);
                }
                if path.type_ == DEV_PATH_ETHERNET || path.type_ == DEV_PATH_DSA {
                    i += 1;
                    continue;
                }
                if path.type_ == DEV_PATH_TUN {
                    if (*info).num_tuns != 0 { dev_fill_forward_path_release(stack); return -1; }
                    (*info).tun.src_v6 = path.tun.src_v6;
                    (*info).tun.dst_v6 = path.tun.dst_v6;
                    (*info).tun.inner_proto = path.tun.inner_proto;
                    (*info).tun_dst = path.tun.dst;
                    (*info).num_tuns += 1;
                } else {
                    if (*info).num_encaps >= NF_FLOW_TABLE_ENCAP_MAX { dev_fill_forward_path_release(stack); return -1; }
                    (*info).encap[(*info).num_encaps as usize].id = path.encap.id;
                    (*info).encap[(*info).num_encaps as usize].proto = path.encap.proto;
                    (*info).num_encaps += 1;
                }
                if path.type_ == DEV_PATH_PPPOE {
                    (*info).h_dest.copy_from_slice(&path.encap.h_dest[..ETH_ALEN]);
                    (*info).xmit_type = FLOW_OFFLOAD_XMIT_DIRECT;
                    (*info).needs_gso_segment = true;
                }
            }
            DEV_PATH_BRIDGE => {
                if is_zero_ether_addr((*info).h_source.as_ptr()) { (*info).h_source.copy_from_slice(&(*path.dev).dev_addr[..ETH_ALEN]); }
                match path.bridge.vlan_mode {
                    DEV_PATH_BR_VLAN_UNTAG_HW => { if (*info).num_encaps == 0 { dev_fill_forward_path_release(stack); return -1; } (*info).ingress_vlans |= BIT((*info).num_encaps - 1); }
                    DEV_PATH_BR_VLAN_TAG => { if (*info).num_encaps >= NF_FLOW_TABLE_ENCAP_MAX { dev_fill_forward_path_release(stack); return -1; } (*info).encap[(*info).num_encaps as usize].id = path.bridge.vlan_id; (*info).encap[(*info).num_encaps as usize].proto = path.bridge.vlan_proto; (*info).num_encaps += 1; }
                    DEV_PATH_BR_VLAN_UNTAG => { if (*info).num_encaps == 0 { dev_fill_forward_path_release(stack); return -1; } (*info).num_encaps -= 1; }
                    DEV_PATH_BR_VLAN_KEEP => {}
                }
                (*info).xmit_type = FLOW_OFFLOAD_XMIT_DIRECT;
            }
            _ => { dev_fill_forward_path_release(stack); return -1; }
        }
        i += 1;
    }
    if nf_flowtable_hw_offload(&mut (*ft).data) && nft_is_valid_ether_device((*info).dev) { (*info).xmit_type = FLOW_OFFLOAD_XMIT_DIRECT; }
    if !nft_flowtable_find_dev((*info).dev, ft) { dev_fill_forward_path_release(stack); return -1; }
    0
}

unsafe fn nft_dev_forward_path(pkt: *const nft_pktinfo, route: *mut nf_flow_route, ct: *const nf_conn, dir: ip_conntrack_dir, ft: *mut nft_flowtable) -> i32 {
    let dst = (*route).tuple[dir].dst;
    let mut stack: net_device_path_stack = core::mem::zeroed();
    let mut info: nft_forward_info = core::mem::zeroed();
    let mut ha = [0u8; ETH_ALEN];
    if nft_dev_fill_forward_path(dst, ct, dir, ha.as_mut_ptr(), (*pkt).ethertype, &mut stack) < 0 || nft_dev_path_info(&mut stack, &mut info, ha.as_ptr(), ft) < 0 { return -ENOENT; }
    (*route).tuple[!dir].in_.ifindex = (*info.dev).ifindex;
    (*route).tuple[dir].out.ifindex = (*info.dev).ifindex;
    for i in 0..info.num_encaps as usize { (*route).tuple[!dir].in_.encap[i].id = info.encap[i].id; (*route).tuple[!dir].in_.encap[i].proto = info.encap[i].proto; }
    if info.num_tuns != 0 { (*route).tuple[!dir].in_.tun.src_v6 = info.tun.dst_v6; (*route).tuple[!dir].in_.tun.dst_v6 = info.tun.src_v6; (*route).tuple[!dir].in_.tun.inner_proto = info.tun.inner_proto; (*route).tuple[!dir].in_.num_tuns = info.num_tuns; dst_release((*route).tuple[dir].dst); (*route).tuple[dir].dst = info.tun_dst; }
    (*route).tuple[!dir].in_.num_encaps = info.num_encaps;
    (*route).tuple[!dir].in_.ingress_vlans = info.ingress_vlans;
    if info.xmit_type == FLOW_OFFLOAD_XMIT_DIRECT { (*route).tuple[dir].out.h_source.copy_from_slice(&info.h_source); (*route).tuple[dir].out.h_dest.copy_from_slice(&info.h_dest); (*route).tuple[dir].xmit_type = info.xmit_type; }
    (*route).tuple[dir].out.needs_gso_segment = info.needs_gso_segment;
    0
}

pub unsafe fn nft_flow_route(pkt: *const nft_pktinfo, ct: *const nf_conn, route: *mut nf_flow_route, dir: ip_conntrack_dir, ft: *mut nft_flowtable) -> i32 {
    let this_dst = skb_dst((*pkt).skb);
    let mut other_dst: *mut dst_entry = core::ptr::null_mut();
    let mut fl: flowi = core::mem::zeroed();
    match nft_pf(pkt) {
        NFPROTO_IPV4 => { fl.u.ip4.daddr = (*ct).tuplehash[dir].tuple.src.u3.ip; fl.u.ip4.saddr = (*ct).tuplehash[!dir].tuple.src.u3.ip; fl.u.ip4.flowi4_oif = nft_in(pkt).ifindex; fl.u.ip4.flowi4_iif = (*(*this_dst).dev).ifindex; fl.u.ip4.flowi4_dscp = ip4h_dscp(ip_hdr((*pkt).skb)); fl.u.ip4.flowi4_mark = (*(*pkt).skb).mark; fl.u.ip4.flowi4_flags = FLOWI_FLAG_ANYSRC; }
        NFPROTO_IPV6 => { fl.u.ip6.daddr = (*ct).tuplehash[dir].tuple.src.u3.in6; fl.u.ip6.saddr = (*ct).tuplehash[!dir].tuple.src.u3.in6; fl.u.ip6.flowi6_oif = nft_in(pkt).ifindex; fl.u.ip6.flowi6_iif = (*(*this_dst).dev).ifindex; fl.u.ip6.flowlabel = ip6_flowinfo(ipv6_hdr((*pkt).skb)); fl.u.ip6.flowi6_mark = (*(*pkt).skb).mark; fl.u.ip6.flowi6_flags = FLOWI_FLAG_ANYSRC; }
    }
    if !dst_hold_safe(this_dst) { return -ENOENT; }
    nf_route(nft_net(pkt), &mut other_dst, &mut fl, false, nft_pf(pkt));
    if other_dst.is_null() { dst_release(this_dst); return -ENOENT; }
    nft_default_forward_path(route, this_dst, dir);
    nft_default_forward_path(route, other_dst, !dir);
    if ((*route).tuple[dir].xmit_type == FLOW_OFFLOAD_XMIT_NEIGH && nft_dev_forward_path(pkt, route, ct, dir, ft) < 0) || ((*route).tuple[!dir].xmit_type == FLOW_OFFLOAD_XMIT_NEIGH && nft_dev_forward_path(pkt, route, ct, !dir, ft) < 0) { dst_release((*route).tuple[dir].dst); dst_release((*route).tuple[!dir].dst); return -ENOENT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
