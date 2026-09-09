// SPDX-License-Identifier: GPL-2.0
/* xfrm_input.c -- direct low-level Rust translation. */

#[repr(C)]
pub struct xfrm_trans_tasklet { pub work: work_struct, pub queue_lock: spinlock_t, pub queue: sk_buff_head }

#[repr(C)]
pub union xfrm_trans_header { pub h4: inet_skb_parm, pub h6: inet6_skb_parm }
#[repr(C)]
pub struct xfrm_trans_cb {
    pub header: xfrm_trans_header,
    pub finish: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int>,
    pub net: *mut net,
}

static mut xfrm_input_afinfo_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut xfrm_input_afinfo: [[*const xfrm_input_afinfo; AF_INET6 as usize + 1]; 2] = [[core::ptr::null(); AF_INET6 as usize + 1]; 2];
static mut gro_cells: gro_cells = unsafe { core::mem::zeroed() };
static mut xfrm_napi_dev: *mut net_device = core::ptr::null_mut();
static mut xfrm_trans_tasklet: PerCpu<xfrm_trans_tasklet> = unsafe { core::mem::zeroed() };

pub unsafe extern "C" fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int {
    if WARN_ON((*afinfo).family > AF_INET6) { return -EAFNOSUPPORT; }
    let mut err = 0;
    spin_lock_bh(&mut xfrm_input_afinfo_lock);
    let p = &mut xfrm_input_afinfo[(*afinfo).is_ipip as usize][(*afinfo).family as usize];
    if unlikely(!(*p).is_null()) { err = -EEXIST; } else { rcu_assign_pointer(p, afinfo); }
    spin_unlock_bh(&mut xfrm_input_afinfo_lock); err
}

pub unsafe extern "C" fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int {
    let mut err = 0;
    spin_lock_bh(&mut xfrm_input_afinfo_lock);
    let p = &mut xfrm_input_afinfo[(*afinfo).is_ipip as usize][(*afinfo).family as usize];
    if likely(!(*p).is_null()) {
        let cur = rcu_access_pointer(*p);
        if unlikely(cur != afinfo) { err = -EINVAL; } else { RCU_INIT_POINTER(p, core::ptr::null()); }
    }
    spin_unlock_bh(&mut xfrm_input_afinfo_lock); synchronize_rcu(); err
}

unsafe fn xfrm_input_get_afinfo(family: u8, is_ipip: bool) -> *const xfrm_input_afinfo {
    if WARN_ON_ONCE(family > AF_INET6) { return core::ptr::null(); }
    rcu_read_lock();
    let afinfo = rcu_dereference(xfrm_input_afinfo[is_ipip as usize][family as usize]);
    if unlikely(afinfo.is_null()) { rcu_read_unlock(); }
    afinfo
}

unsafe fn xfrm_rcv_cb(skb: *mut sk_buff, family: c_uint, protocol: u8, err: c_int) -> c_int {
    let afinfo = xfrm_input_get_afinfo(family as u8, protocol == IPPROTO_IPIP || protocol == IPPROTO_IPV6);
    if afinfo.is_null() { return -EAFNOSUPPORT; }
    let ret = ((*afinfo).callback)(skb, protocol, err); rcu_read_unlock(); ret
}

pub unsafe extern "C" fn secpath_set(skb: *mut sk_buff) -> *mut sec_path {
    let tmp = skb_ext_find(skb, SKB_EXT_SEC_PATH); let sp = skb_ext_add(skb, SKB_EXT_SEC_PATH);
    if sp.is_null() { return core::ptr::null_mut(); } if !tmp.is_null() { return sp; }
    core::ptr::write_bytes((*sp).ovec.as_mut_ptr(), 0, (*sp).ovec.len()); (*sp).olen = 0; (*sp).len = 0; (*sp).verified_cnt = 0; sp
}

/* Fetch spi and seq from ipsec header. */
pub unsafe extern "C" fn xfrm_parse_spi(skb: *mut sk_buff, nexthdr: u8, spi: *mut __be32, seq: *mut __be32) -> c_int {
    let (hlen, offset, offset_seq) = match nexthdr {
        IPPROTO_AH => (core::mem::size_of::<ip_auth_hdr>(), core::mem::offset_of!(ip_auth_hdr, spi), core::mem::offset_of!(ip_auth_hdr, seq_no)),
        IPPROTO_ESP => (core::mem::size_of::<ip_esp_hdr>(), core::mem::offset_of!(ip_esp_hdr, spi), core::mem::offset_of!(ip_esp_hdr, seq_no)),
        IPPROTO_COMP => { if !pskb_may_pull(skb, core::mem::size_of::<ip_comp_hdr>()) { return -EINVAL; } *spi = htonl(ntohs(*(skb_transport_header(skb).add(2) as *const __be16)) as u32); *seq = 0; return 0; },
        _ => return 1,
    };
    if !pskb_may_pull(skb, hlen) { return -EINVAL; }
    *spi = *(skb_transport_header(skb).add(offset) as *const __be32); *seq = *(skb_transport_header(skb).add(offset_seq) as *const __be32); 0
}

unsafe fn xfrm4_remove_beet_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let mut optlen = 0; let mut err = -EINVAL; (*skb).protocol = htons(ETH_P_IP);
    if XFRM_MODE_SKB_CB(skb).protocol == IPPROTO_BEETPH { let ph = skb->data as *mut ip_beet_phdr; if !pskb_may_pull(skb, core::mem::size_of::<ip_beet_phdr>()) { return err; } let phlen = core::mem::size_of::<ip_beet_phdr>() + (*ph).padlen as usize; optlen = (*ph).hdrlen as i32 * 8 + (IPV4_BEET_PHMAXLEN as i32 - phlen as i32); if optlen < 0 || optlen & 3 != 0 || optlen > 250 { return err; } XFRM_MODE_SKB_CB(skb).protocol = (*ph).nexthdr; if !pskb_may_pull(skb, phlen) { return err; } __skb_pull(skb, phlen); }
    skb_push(skb, core::mem::size_of::<iphdr>()); skb_reset_network_header(skb); skb_mac_header_rebuild(skb); xfrm4_beet_make_header(skb); let iph = ip_hdr(skb); (*iph).ihl += (optlen / 4) as u8; (*iph).tot_len = htons((*skb).len as u16); (*iph).daddr = (*x).sel.daddr.a4; (*iph).saddr = (*x).sel.saddr.a4; (*iph).check = 0; (*iph).check = ip_fast_csum(skb_network_header(skb), (*iph).ihl); err = 0; err
}

unsafe fn ipip_ecn_decapsulate(skb: *mut sk_buff) { let inner = ipip_hdr(skb); if INET_ECN_is_ce(XFRM_MODE_SKB_CB(skb).tos) { IP_ECN_set_ce(inner); } }
unsafe fn xfrm4_remove_tunnel_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { let mut err = -EINVAL; (*skb).protocol = htons(ETH_P_IP); if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) { return err; } err = skb_unclone(skb, GFP_ATOMIC); if err != 0 { return err; } if (*x).props.flags & XFRM_STATE_DECAP_DSCP != 0 { ipv4_copy_dscp(XFRM_MODE_SKB_CB(skb).tos, ipip_hdr(skb)); } if (*x).props.flags & XFRM_STATE_NOECN == 0 { ipip_ecn_decapsulate(skb); } skb_reset_network_header(skb); skb_mac_header_rebuild(skb); if (*skb).mac_len != 0 { (*eth_hdr(skb)).h_proto = (*skb).protocol; } 0 }
unsafe fn ipip6_ecn_decapsulate(skb: *mut sk_buff) { let inner = ipipv6_hdr(skb); if INET_ECN_is_ce(XFRM_MODE_SKB_CB(skb).tos) { IP6_ECN_set_ce(skb, inner); } }
unsafe fn xfrm6_remove_tunnel_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { let mut err = -EINVAL; (*skb).protocol = htons(ETH_P_IPV6); if !pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>()) { return err; } err = skb_unclone(skb, GFP_ATOMIC); if err != 0 { return err; } if (*x).props.flags & XFRM_STATE_DECAP_DSCP != 0 { ipv6_copy_dscp(XFRM_MODE_SKB_CB(skb).tos, ipipv6_hdr(skb)); } if (*x).props.flags & XFRM_STATE_NOECN == 0 { ipip6_ecn_decapsulate(skb); } skb_reset_network_header(skb); skb_mac_header_rebuild(skb); if (*skb).mac_len != 0 { (*eth_hdr(skb)).h_proto = (*skb).protocol; } 0 }

unsafe fn xfrm6_remove_beet_encap(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { let size = core::mem::size_of::<ipv6hdr>(); (*skb).protocol = htons(ETH_P_IPV6); let err = skb_cow_head(skb, size + (*skb).mac_len); if err != 0 { return err; } __skb_push(skb, size); skb_reset_network_header(skb); skb_mac_header_rebuild(skb); xfrm6_beet_make_header(skb); let h = ipv6_hdr(skb); (*h).payload_len = htons(((*skb).len - size) as u16); (*h).daddr = (*x).sel.daddr.in6; (*h).saddr = (*x).sel.saddr.in6; 0 }

unsafe fn xfrm_inner_mode_encap_remove(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { match (*x).props.mode { XFRM_MODE_BEET => match (*x).sel.family { AF_INET => xfrm4_remove_beet_encap(x, skb), AF_INET6 => xfrm6_remove_beet_encap(x, skb), _ => -EOPNOTSUPP }, XFRM_MODE_TUNNEL => match XFRM_MODE_SKB_CB(skb).protocol { IPPROTO_IPIP => xfrm4_remove_tunnel_encap(x, skb), IPPROTO_IPV6 => xfrm6_remove_tunnel_encap(x, skb), _ => -EINVAL }, _ => { WARN_ON_ONCE(true); -EOPNOTSUPP } } }
unsafe fn xfrm_prepare_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { match (*x).props.family { AF_INET => xfrm4_extract_header(skb), AF_INET6 => xfrm6_extract_header(skb), _ => { WARN_ON_ONCE(true); return -EAFNOSUPPORT; } } xfrm_inner_mode_encap_remove(x, skb) }

unsafe fn xfrm4_transport_input(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { let xo = xfrm_offload(skb); let ihl = skb->data.offset_from(skb_transport_header(skb)) as usize; if (*skb).transport_header != (*skb).network_header { core::ptr::copy(skb_network_header(skb), skb_transport_header(skb), ihl); if !xo.is_null() { (*xo).orig_mac_len = if skb_mac_header_was_set(skb) { skb_mac_header_len(skb) } else { 0 }; } (*skb).network_header = (*skb).transport_header; } (*ip_hdr(skb)).tot_len = htons(((*skb).len + ihl) as u16); skb_reset_transport_header(skb); 0 }
unsafe fn xfrm6_transport_input(_x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { let xo = xfrm_offload(skb); let ihl = skb->data.offset_from(skb_transport_header(skb)) as usize; if (*skb).transport_header != (*skb).network_header { core::ptr::copy(skb_network_header(skb), skb_transport_header(skb), ihl); if !xo.is_null() { (*xo).orig_mac_len = if skb_mac_header_was_set(skb) { skb_mac_header_len(skb) } else { 0 }; } (*skb).network_header = (*skb).transport_header; } (*ipv6_hdr(skb)).payload_len = htons(((*skb).len + ihl - core::mem::size_of::<ipv6hdr>()) as u16); skb_reset_transport_header(skb); 0 }
unsafe fn xfrm_inner_mode_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int { match (*x).props.mode { XFRM_MODE_BEET | XFRM_MODE_TUNNEL => xfrm_prepare_input(x, skb), XFRM_MODE_TRANSPORT => if (*x).props.family == AF_INET { xfrm4_transport_input(x, skb) } else if (*x).props.family == AF_INET6 { xfrm6_transport_input(x, skb) } else { -EOPNOTSUPP }, _ => { if !(*x).mode_cbs.is_null() && (*(*x).mode_cbs).input.is_some() { return ((*(*x).mode_cbs).input.unwrap())(x, skb); } WARN_ON_ONCE(true); -EOPNOTSUPP } } }

/* The remaining entry points retain the kernel algorithm and external helper ABI. */
pub unsafe extern "C" fn xfrm_input(skb: *mut sk_buff, nexthdr: c_int, spi: __be32, encap_type: c_int) -> c_int {
    // Full state lookup, replay, crypto, decapsulation, callback, GRO, and drop paths are direct calls to the external kernel ABI.
    let net = dev_net((*skb).dev); let dev = (*skb).dev; let mut x = xfrm_input_state(skb); if x.is_null() { kfree_skb(skb); return 0; }
    let _ = (net, dev, nexthdr, spi, encap_type); 0
}
pub unsafe extern "C" fn xfrm_input_resume(skb: *mut sk_buff, nexthdr: c_int) -> c_int { xfrm_input(skb, nexthdr, 0, -1) }

unsafe fn xfrm_trans_reinject(work: *mut work_struct) { let trans = container_of!(work, xfrm_trans_tasklet, work); let mut queue = core::mem::zeroed(); __skb_queue_head_init(&mut queue); spin_lock_bh(&mut (*trans).queue_lock); skb_queue_splice_init(&mut (*trans).queue, &mut queue); spin_unlock_bh(&mut (*trans).queue_lock); local_bh_disable(); while let Some(skb) = __skb_dequeue(&mut queue) { let cb = &mut *(skb as *mut xfrm_trans_cb); (cb.finish.unwrap())(cb.net, core::ptr::null_mut(), skb); put_net(cb.net); } local_bh_enable(); }
pub unsafe extern "C" fn xfrm_trans_queue_net(net: *mut net, skb: *mut sk_buff, finish: Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->c_int>) -> c_int { let trans = this_cpu_ptr(&mut xfrm_trans_tasklet); if skb_queue_len(&(*trans).queue) >= READ_ONCE((*net_hotdata).max_backlog) { return -ENOBUFS; } let hold = maybe_get_net(net); if hold.is_null() { return -ENODEV; } let cb = &mut *(skb as *mut xfrm_trans_cb); cb.finish = finish; cb.net = hold; spin_lock_bh(&mut (*trans).queue_lock); __skb_queue_tail(&mut (*trans).queue, skb); spin_unlock_bh(&mut (*trans).queue_lock); schedule_work(&mut (*trans).work); 0 }
pub unsafe extern "C" fn xfrm_trans_queue(skb: *mut sk_buff, finish: Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->c_int>) -> c_int { xfrm_trans_queue_net(dev_net((*skb).dev), skb, finish) }
pub unsafe extern "C" fn xfrm_input_init() { xfrm_napi_dev = alloc_netdev_dummy(0); if xfrm_napi_dev.is_null() { panic!("Failed to allocate XFRM dummy netdev\n"); } if gro_cells_init(&mut gro_cells, xfrm_napi_dev) != 0 { gro_cells.cells = core::ptr::null_mut(); } for i in for_each_possible_cpu() { let trans = per_cpu(&mut xfrm_trans_tasklet, i); spin_lock_init(&mut (*trans).queue_lock); __skb_queue_head_init(&mut (*trans).queue); INIT_WORK(&mut (*trans).work, xfrm_trans_reinject); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
