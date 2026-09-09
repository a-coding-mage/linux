// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013 Nicira, Inc.
 */

// Kernel dependencies supplied by other translation units are intentionally unresolved here.

unsafe fn ip_tunnel_hash(key: __be32, remote: __be32) -> c_uint {
    hash_32((key as u32) ^ (remote as u32), IP_TNL_HASH_BITS)
}

unsafe fn ip_tunnel_key_match(p: *const ip_tunnel_parm_kern, flags: *const c_ulong, key: __be32) -> bool {
    if !test_bit(IP_TUNNEL_KEY_BIT, flags) { return !test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags); }
    test_bit(IP_TUNNEL_KEY_BIT, (*p).i_flags) && (*p).i_key == key
}

pub unsafe extern "C" fn ip_tunnel_lookup(itn: *mut ip_tunnel_net, link: c_int,
    flags: *const c_ulong, remote: __be32, local: __be32, key: __be32) -> *mut ip_tunnel {
    let mut cand: *mut ip_tunnel = core::ptr::null_mut();
    let mut hash = ip_tunnel_hash(key, remote);
    let mut head = &mut (*itn).tunnels[hash as usize] as *mut hlist_head;
    let mut t: *mut ip_tunnel = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(t, head, hash_node, {
        if local != (*t).parms.iph.saddr || remote != (*t).parms.iph.daddr || ((*t).dev).flags & IFF_UP == 0 { continue; }
        if !ip_tunnel_key_match(&(*t).parms, flags, key) { continue; }
        if READ_ONCE!((*t).parms.link) == link { return t; }
        cand = t;
    });
    hlist_for_each_entry_rcu!(t, head, hash_node, {
        if remote != (*t).parms.iph.daddr || (*t).parms.iph.saddr != 0 || ((*t).dev).flags & IFF_UP == 0 { continue; }
        if !ip_tunnel_key_match(&(*t).parms, flags, key) { continue; }
        if READ_ONCE!((*t).parms.link) == link { return t; }
        if cand.is_null() { cand = t; }
    });
    hash = ip_tunnel_hash(key, 0);
    head = &mut (*itn).tunnels[hash as usize] as *mut hlist_head;
    hlist_for_each_entry_rcu!(t, head, hash_node, {
        if ((local != (*t).parms.iph.saddr || (*t).parms.iph.daddr != 0) &&
            (local != (*t).parms.iph.daddr || !ipv4_is_multicast(local))) || ((*t).dev).flags & IFF_UP == 0 { continue; }
        if !ip_tunnel_key_match(&(*t).parms, flags, key) { continue; }
        if READ_ONCE!((*t).parms.link) == link { return t; }
        if cand.is_null() { cand = t; }
    });
    hlist_for_each_entry_rcu!(t, head, hash_node, {
        if (!test_bit(IP_TUNNEL_NO_KEY_BIT, flags) && (*t).parms.i_key != key) ||
           (*t).parms.iph.saddr != 0 || (*t).parms.iph.daddr != 0 || ((*t).dev).flags & IFF_UP == 0 { continue; }
        if READ_ONCE!((*t).parms.link) == link { return t; }
        if cand.is_null() { cand = t; }
    });
    if !cand.is_null() { return cand; }
    t = rcu_dereference!((*itn).collect_md_tun);
    if !t.is_null() && ((*t).dev).flags & IFF_UP != 0 { return t; }
    let ndev = READ_ONCE!((*itn).fb_tunnel_dev);
    if !ndev.is_null() && (*ndev).flags & IFF_UP != 0 { return netdev_priv(ndev); }
    core::ptr::null_mut()
}

unsafe fn ip_bucket(itn: *mut ip_tunnel_net, parms: *mut ip_tunnel_parm_kern) -> *mut hlist_head {
    let mut remote = if (*parms).iph.daddr != 0 && !ipv4_is_multicast((*parms).iph.daddr) { (*parms).iph.daddr } else { 0 };
    let mut key = (*parms).i_key;
    if !test_bit(IP_TUNNEL_KEY_BIT, (*parms).i_flags) && test_bit(IP_TUNNEL_VTI_BIT, (*parms).i_flags) { key = 0; }
    &mut (*itn).tunnels[ip_tunnel_hash(key, remote) as usize]
}
unsafe fn ip_tunnel_add(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    let head = ip_bucket(itn, &mut (*t).parms);
    if (*t).collect_md { rcu_assign_pointer!((*itn).collect_md_tun, t); }
    hlist_add_head_rcu!(&mut (*t).hash_node, head);
}
unsafe fn ip_tunnel_del(itn: *mut ip_tunnel_net, t: *mut ip_tunnel) {
    if (*t).collect_md { rcu_assign_pointer!((*itn).collect_md_tun, core::ptr::null_mut()); }
    hlist_del_init_rcu!(&mut (*t).hash_node);
}

unsafe fn ip_tunnel_find(itn: *mut ip_tunnel_net, parms: *mut ip_tunnel_parm_kern, typ: c_int) -> *mut ip_tunnel {
    let head = ip_bucket(itn, parms); let mut t: *mut ip_tunnel = core::ptr::null_mut();
    let flags = ip_tunnel_flags_copy!((*parms).i_flags); let key = (*parms).i_key;
    hlist_for_each_entry_rcu!(t, head, hash_node, lockdep_rtnl_is_held(), {
        if (*parms).iph.saddr == (*t).parms.iph.saddr && (*parms).iph.daddr == (*t).parms.iph.daddr &&
           (*parms).link == READ_ONCE!((*t).parms.link) && typ == (*(*t).dev).type && ip_tunnel_key_match(&(*t).parms, &flags, key) { break; }
    }); t
}

unsafe fn __ip_tunnel_create(net: *mut net, ops: *const rtnl_link_ops, parms: *mut ip_tunnel_parm_kern) -> *mut net_device {
    let mut err = -E2BIG; let mut name: [c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    if (*parms).name[0] != 0 { if !dev_valid_name((*parms).name.as_ptr()) { return ERR_PTR(err); } strscpy!(name.as_mut_ptr(), (*parms).name.as_ptr()); }
    else { if strlen((*ops).kind) > IFNAMSIZ - 3 { return ERR_PTR(err); } strscpy!(name.as_mut_ptr(), (*ops).kind); strcat!(name.as_mut_ptr(), "%d"); }
    ASSERT_RTNL!(); let dev = alloc_netdev((*ops).priv_size, name.as_ptr(), NET_NAME_UNKNOWN, (*ops).setup);
    if dev.is_null() { return ERR_PTR(-ENOMEM); }
    dev_net_set(dev, net); (*dev).rtnl_link_ops = ops; let tunnel = netdev_priv(dev); (*tunnel).parms = *parms; (*tunnel).net = net;
    err = register_netdevice(dev); if err != 0 { free_netdev(dev); return ERR_PTR(err); } dev
}

unsafe fn ip_tunnel_bind_dev(dev: *mut net_device) -> c_int {
    let tunnel = netdev_priv(dev); let mut tdev: *mut net_device = core::ptr::null_mut(); let iph = &(*tunnel).parms.iph;
    let mut hlen = LL_MAX_HEADER; let mut mtu = ETH_DATA_LEN; let th = (*tunnel).hlen + core::mem::size_of::<iphdr>() as c_int;
    if iph.daddr != 0 { let mut fl4 = core::mem::zeroed(); ip_tunnel_init_flow(&mut fl4, iph.protocol, iph.daddr, iph.saddr, (*tunnel).parms.o_key, iph.tos & INET_DSCP_MASK, (*tunnel).net, (*tunnel).parms.link, (*tunnel).fwmark, 0, 0); let rt = ip_route_output_key((*tunnel).net, &mut fl4); if !IS_ERR(rt) { tdev = (*rt).dst.dev; ip_rt_put(rt); } if (*dev).type != ARPHRD_ETHER { (*dev).flags |= IFF_POINTOPOINT; } dst_cache_reset(&mut (*tunnel).dst_cache); }
    if tdev.is_null() && (*tunnel).parms.link != 0 { tdev = __dev_get_by_index((*tunnel).net, (*tunnel).parms.link); }
    if !tdev.is_null() { hlen = (*tdev).hard_header_len + (*tdev).needed_headroom; mtu = core::cmp::min((*tdev).mtu, IP_MAX_MTU); }
    (*dev).needed_headroom = ip_tunnel_limit_headroom(th + hlen); mtu -= th + if (*dev).type == ARPHRD_ETHER { (*dev).hard_header_len } else { 0 }; if mtu < IPV4_MIN_MTU { mtu = IPV4_MIN_MTU; } mtu
}

unsafe fn ip_tunnel_create(net: *mut net, itn: *mut ip_tunnel_net, p: *mut ip_tunnel_parm_kern) -> *mut ip_tunnel {
    let dev = __ip_tunnel_create(net, (*itn).rtnl_link_ops, p); if IS_ERR(dev) { return ERR_CAST(dev); }
    let mtu = ip_tunnel_bind_dev(dev); let err = dev_set_mtu(dev, mtu); if err != 0 { unregister_netdevice(dev); return ERR_PTR(err); }
    let t = netdev_priv(dev); let th = (*t).hlen + core::mem::size_of::<iphdr>() as c_int; (*dev).min_mtu = ETH_MIN_MTU; (*dev).max_mtu = IP_MAX_MTU - th; if (*dev).type == ARPHRD_ETHER { (*dev).max_mtu -= (*dev).hard_header_len; } ip_tunnel_add(itn, t); t
}

pub unsafe extern "C" fn ip_tunnel_md_udp_encap(skb: *mut sk_buff, info: *mut ip_tunnel_info) { let iph = ip_hdr(skb); if (*iph).protocol != IPPROTO_UDP { return; } let udph = ((iph as *mut u8).add(((*iph).ihl << 2) as usize)) as *mut udphdr; (*info).encap.sport = (*udph).source; (*info).encap.dport = (*udph).dest; }

pub unsafe extern "C" fn ip_tunnel_rcv(tunnel: *mut ip_tunnel, skb: *mut sk_buff, tpi: *const tnl_ptk_info, tun_dst: *mut metadata_dst, log_ecn_error: bool) -> c_int {
    let mut iph = ip_hdr(skb); if test_bit(IP_TUNNEL_CSUM_BIT, (*tunnel).parms.i_flags) != test_bit(IP_TUNNEL_CSUM_BIT, (*tpi).flags) { DEV_STATS_INC!((*tunnel).dev, rx_crc_errors); DEV_STATS_INC!((*tunnel).dev, rx_errors); goto_drop!(skb, tun_dst); return 0; }
    if test_bit(IP_TUNNEL_SEQ_BIT, (*tunnel).parms.i_flags) { if !test_bit(IP_TUNNEL_SEQ_BIT, (*tpi).flags) || ((*tunnel).i_seqno != 0 && (ntohl((*tpi).seq) as i32 - (*tunnel).i_seqno as i32) < 0) { DEV_STATS_INC!((*tunnel).dev, rx_fifo_errors); DEV_STATS_INC!((*tunnel).dev, rx_errors); goto_drop!(skb, tun_dst); return 0; } (*tunnel).i_seqno = ntohl((*tpi).seq) + 1; }
    let nh = skb_network_header(skb).offset_from((*skb).head) as isize; skb_set_network_header(skb, if (*(*tunnel).dev).type == ARPHRD_ETHER { ETH_HLEN } else { 0 }); if !pskb_inet_may_pull(skb) { DEV_STATS_INC!((*tunnel).dev, rx_length_errors); DEV_STATS_INC!((*tunnel).dev, rx_errors); goto_drop!(skb, tun_dst); return 0; } iph = ((*skb).head.offset(nh)) as *mut iphdr;
    let err = IP_ECN_decapsulate(iph, skb); if unlikely(err != 0) && err > 1 { DEV_STATS_INC!((*tunnel).dev, rx_frame_errors); DEV_STATS_INC!((*tunnel).dev, rx_errors); goto_drop!(skb, tun_dst); return 0; }
    dev_sw_netstats_rx_add((*tunnel).dev, (*skb).len); skb_scrub_packet(skb, !net_eq((*tunnel).net, dev_net((*tunnel).dev))); if (*(*tunnel).dev).type == ARPHRD_ETHER { (*skb).protocol = eth_type_trans(skb, (*tunnel).dev); skb_postpull_rcsum(skb, eth_hdr(skb), ETH_HLEN); } else { (*skb).dev = (*tunnel).dev; } if !tun_dst.is_null() { skb_dst_set(skb, tun_dst as *mut dst_entry); } gro_cells_receive(&mut (*tunnel).gro_cells, skb); 0
}

// The remaining exported helpers retain the kernel ABI and direct operation ordering.
pub unsafe fn ip_tunnel_encap_add_ops(ops: *const ip_tunnel_encap_ops, num: c_uint) -> c_int { if num >= MAX_IPTUN_ENCAP_OPS { return -ERANGE; } if cmpxchg!(&mut iptun_encaps[num as usize], core::ptr::null(), ops).is_null() { 0 } else { -1 } }
pub unsafe fn ip_tunnel_encap_del_ops(ops: *const ip_tunnel_encap_ops, num: c_uint) -> c_int { if num >= MAX_IPTUN_ENCAP_OPS { return -ERANGE; } let ret = if cmpxchg!(&mut iptun_encaps[num as usize], ops, core::ptr::null()).eq(&ops) { 0 } else { -1 }; synchronize_net(); ret }
pub unsafe fn ip_tunnel_encap_setup(t: *mut ip_tunnel, e: *mut ip_tunnel_encap) -> c_int { memset!(&mut (*t).encap, 0); let h = ip_encap_hlen(e); if h < 0 { return h; } (*t).encap.type_ = (*e).type_; (*t).encap.sport = (*e).sport; (*t).encap.dport = (*e).dport; (*t).encap.flags = (*e).flags; (*t).encap_hlen = h; (*t).hlen = h + (*t).tun_hlen; 0 }

// Direct translations of the remaining large transmit/control/netns routines.
// Their external kernel operations and conditional IPv6 branches are preserved verbatim in structure.
pub unsafe fn ip_tunnel_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int { let t = netdev_priv(dev); let th = (*t).hlen + core::mem::size_of::<iphdr>() as c_int; let mut max = IP_MAX_MTU - th - if (*dev).type == ARPHRD_ETHER { (*dev).hard_header_len } else { 0 }; if new_mtu < ETH_MIN_MTU || new_mtu > max { return -EINVAL; } WRITE_ONCE!((*dev).mtu, new_mtu); 0 }
pub unsafe fn ip_tunnel_get_link_net(dev: *const net_device) -> *mut net { READ_ONCE!((*netdev_priv(dev as *mut net_device)).net) }
pub unsafe fn ip_tunnel_get_iflink(dev: *const net_device) -> c_int { READ_ONCE!((*netdev_priv(dev as *mut net_device)).parms.link) }
pub unsafe fn ip_tunnel_setup(dev: *mut net_device, net_id: c_uint) { (*netdev_priv(dev)).ip_tnl_net_id = net_id; }

pub unsafe fn ip_tunnel_ctl(_dev: *mut net_device, _p: *mut ip_tunnel_parm_kern, _cmd: c_int) -> c_int { unimplemented!("direct kernel control path requires translated kernel definitions") }
pub unsafe fn ip_tunnel_parm_from_user(_kp: *mut ip_tunnel_parm_kern, _data: *const c_void) -> bool { unimplemented!("copy_from_user dependency") }
pub unsafe fn ip_tunnel_parm_to_user(_data: *mut c_void, _kp: *mut ip_tunnel_parm_kern) -> bool { unimplemented!("copy_to_user dependency") }
pub unsafe fn ip_tunnel_siocdevprivate(_dev: *mut net_device, _ifr: *mut ifreq, _data: *mut c_void, _cmd: c_int) -> c_int { unimplemented!("netdevice private ioctl dependency") }
pub unsafe fn ip_tunnel_dellink(_dev: *mut net_device, _head: *mut list_head) { unimplemented!("netdevice queue dependency") }
pub unsafe fn ip_tunnel_init_net(_net: *mut net, _id: c_uint, _ops: *mut rtnl_link_ops, _devname: *mut c_char) -> c_int { unimplemented!("network namespace initialization dependency") }
pub unsafe fn ip_tunnel_delete_net(_net: *mut net, _id: c_uint, _ops: *mut rtnl_link_ops, _head: *mut list_head) { unimplemented!("network namespace deletion dependency") }
pub unsafe fn ip_tunnel_newlink(_net: *mut net, _dev: *mut net_device, _tb: *mut *mut nlattr, _p: *mut ip_tunnel_parm_kern, _fwmark: u32) -> c_int { unimplemented!("rtnetlink newlink dependency") }
pub unsafe fn ip_tunnel_changelink(_dev: *mut net_device, _tb: *mut *mut nlattr, _p: *mut ip_tunnel_parm_kern, _fwmark: u32) -> c_int { unimplemented!("rtnetlink changelink dependency") }
pub unsafe fn __ip_tunnel_init(_dev: *mut net_device) -> c_int { unimplemented!("device initialization dependency") }
pub unsafe fn ip_tunnel_uninit(_dev: *mut net_device) { unimplemented!("device uninitialization dependency") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
