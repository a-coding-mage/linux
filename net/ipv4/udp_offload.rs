// SPDX-License-Identifier: GPL-2.0-or-later
/* IPV4 GSO/GRO offload support; Linux INET implementation; UDPv4 GSO support. */

/* Kernel types, constants, helpers, and external symbols are supplied by the surrounding
 * translation unit.  This file intentionally retains the pointer-oriented implementation. */

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
unsafe extern "C" {
    fn dummy_gro_rcv(sk: *mut sock, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
}

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
type udp_tunnel_gro_rcv_t = unsafe extern "C" fn(*mut sock, *mut list_head, *mut sk_buff) -> *mut sk_buff;

#[repr(C)]
struct udp_tunnel_type_entry { gro_receive: udp_tunnel_gro_rcv_t, count: refcount_t }

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
static mut udp_tunnel_gro_type_nr: u32 = 0;

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
unsafe fn dummy_gro_rcv_impl(_sk: *mut sock, _head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    (*NAPI_GRO_CB(skb)).flush = 1;
    core::ptr::null_mut()
}

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_update_gro_lookup(net: *mut net, sk: *mut sock, add: bool) {
    let is_ipv6 = (*sk).sk_family == AF_INET6;
    let up = udp_sk(sk);
    spin_lock(&raw mut udp_tunnel_gro_lock);
    let gro = &mut (*net).ipv4.udp_tunnel_gro[is_ipv6 as usize];
    if add { hlist_add_head(&mut (*up).tunnel_list, &mut gro.list); }
    else if !(*up).tunnel_list.pprev.is_null() { hlist_del_init(&mut (*up).tunnel_list); }
    if !gro.list.first.is_null() && (*gro.list.first).next.is_null() {
        let tup = hlist_entry(gro.list.first, core::mem::offset_of!(udp_sock, tunnel_list));
        rcu_assign_pointer(&mut gro.sk, tup as *mut sock);
    } else { RCU_INIT_POINTER(&mut gro.sk, core::ptr::null_mut()); }
    spin_unlock(&raw mut udp_tunnel_gro_lock);
}

#[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_update_gro_rcv(sk: *mut sock, add: bool) {
    let up = udp_sk(sk);
    if UDP_MAX_TUNNEL_TYPES == 0 || (*up).gro_receive.is_none() { return; }
    mutex_lock(&raw mut udp_tunnel_gro_type_lock);
    if udp_tunnel_gro_type_nr > UDP_MAX_TUNNEL_TYPES { mutex_unlock(&raw mut udp_tunnel_gro_type_lock); return; }
    let mut cur: *mut udp_tunnel_type_entry = core::ptr::null_mut();
    for i in 0..udp_tunnel_gro_type_nr as usize {
        if udp_tunnel_gro_types[i].gro_receive == (*up).gro_receive.unwrap() { cur = &raw mut udp_tunnel_gro_types[i]; }
    }
    let old = udp_tunnel_gro_type_nr;
    if add {
        if !cur.is_null() { refcount_inc(&mut (*cur).count); mutex_unlock(&raw mut udp_tunnel_gro_type_lock); return; }
        if udp_tunnel_gro_type_nr == UDP_MAX_TUNNEL_TYPES { pr_err_once!("Too many UDP tunnel types, please increase UDP_MAX_TUNNEL_TYPES\n"); udp_tunnel_gro_type_nr = UDP_MAX_TUNNEL_TYPES + 1; }
        else { cur = &raw mut udp_tunnel_gro_types[udp_tunnel_gro_type_nr as usize]; udp_tunnel_gro_type_nr += 1; refcount_set(&mut (*cur).count, 1); (*cur).gro_receive = (*up).gro_receive.unwrap(); }
    } else {
        if cur.is_null() { mutex_unlock(&raw mut udp_tunnel_gro_type_lock); return; }
        if !refcount_dec_and_test(&mut (*cur).count) { mutex_unlock(&raw mut udp_tunnel_gro_type_lock); return; }
        udp_tunnel_gro_types[(cur as usize - udp_tunnel_gro_types.as_ptr() as usize) / core::mem::size_of::<udp_tunnel_type_entry>()] = udp_tunnel_gro_types[(udp_tunnel_gro_type_nr - 1) as usize];
        udp_tunnel_gro_type_nr -= 1;
    }
    if udp_tunnel_gro_type_nr == 1 { static_call_update!(udp_tunnel_gro_rcv, udp_tunnel_gro_types[0].gro_receive); static_branch_enable(&raw mut udp_tunnel_static_call); }
    else if old == 1 { static_branch_disable(&raw mut udp_tunnel_static_call); static_call_update!(udp_tunnel_gro_rcv, dummy_gro_rcv_impl); }
    mutex_unlock(&raw mut udp_tunnel_gro_type_lock);
}

unsafe fn udp_tunnel_gro_rcv(sk: *mut sock, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff {
    #[cfg(feature = "CONFIG_NET_UDP_TUNNEL")]
    { if static_branch_likely(&raw mut udp_tunnel_static_call) { if gro_recursion_inc_test(skb) { (*NAPI_GRO_CB(skb)).flush |= 1; return core::ptr::null_mut(); } return static_call!(udp_tunnel_gro_rcv)(sk, head, skb); } }
    call_gro_receive_sk((*udp_sk(sk)).gro_receive, sk, head, skb)
}

unsafe fn __skb_udp_tunnel_segment(skb: *mut sk_buff, mut features: netdev_features_t, inner: unsafe extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff, new_protocol: __be16, is_ipv6: bool) -> *mut sk_buff {
    let tnl_hlen = skb_inner_mac_header(skb) - skb_transport_header(skb);
    if !pskb_may_pull(skb, tnl_hlen) { return ERR_PTR(-EINVAL); }
    let uh = udp_hdr(skb);
    let partial = if (*skb_shinfo(skb)).gso_type & SKB_GSO_PARTIAL != 0 { (*uh).len as u32 } else { htonl((*skb).len) };
    let partial = csum_sub(csum_unfold((*uh).check), partial);
    (*skb).encapsulation = 0; (*SKB_GSO_CB(skb)).encap_level = 0; __skb_pull(skb, tnl_hlen); skb_reset_mac_header(skb); skb_set_network_header(skb, skb_inner_network_offset(skb)); skb_set_transport_header(skb, skb_inner_transport_offset(skb)); (*skb).mac_len = skb_inner_network_offset(skb); (*skb).protocol = new_protocol;
    let need_csum = (*skb_shinfo(skb)).gso_type & SKB_GSO_UDP_TUNNEL_CSUM != 0; (*skb).encap_hdr_csum = need_csum;
    let remcsum = (*skb_shinfo(skb)).gso_type & SKB_GSO_TUNNEL_REMCSUM != 0; (*skb).remcsum_offload = remcsum;
    let need_ipsec = (!skb_dst(skb).is_null() && dst_xfrm(skb_dst(skb))) || !skb_sec_path(skb).is_null();
    let offload = need_csum && !need_ipsec && ((*(*skb).dev).features & if is_ipv6 { NETIF_F_HW_CSUM | NETIF_F_IPV6_CSUM } else { NETIF_F_HW_CSUM | NETIF_F_IP_CSUM }) != 0;
    features &= (*(*skb).dev).hw_enc_features; if need_csum { features &= !NETIF_F_SCTP_CRC; }
    if remcsum { features &= !NETIF_F_CSUM_MASK; if !need_csum || offload { features |= NETIF_F_HW_CSUM; } }
    let segs = inner(skb, features); if IS_ERR_OR_NULL(segs) { skb_gso_error_unwind(skb); return segs; }
    let mut p = segs; let outer_hlen = skb_tnl_header_len(skb); let udp_offset = outer_hlen - tnl_hlen;
    while !p.is_null() { if remcsum { (*p).ip_summed = CHECKSUM_NONE; } if (*p).ip_summed == CHECKSUM_PARTIAL { skb_reset_inner_headers(p); (*p).encapsulation = 1; } (*p).mac_len = (*skb).mac_len; (*p).protocol = (*skb).protocol; __skb_push(p, outer_hlen); skb_reset_mac_header(p); skb_set_network_header(p, (*p).mac_len); skb_set_transport_header(p, udp_offset); let len = (*p).len - udp_offset; let h = udp_hdr(p); udp_set_len_short(h, len); if need_csum { (*h).check = !csum_fold(csum_add(partial, htonl(len))); if (*p).encapsulation || !offload { (*h).check = gso_make_checksum(p, !(*h).check); if (*h).check == 0 { (*h).check = CSUM_MANGLED_0; } } else { (*p).ip_summed = CHECKSUM_PARTIAL; (*p).csum_start = skb_transport_header(p) - (*p).head; (*p).csum_offset = core::mem::offset_of!(udphdr, check) as u16; } } p = (*p).next; }
    segs
}

#[no_mangle]
pub unsafe extern "C" fn skb_udp_tunnel_segment(skb: *mut sk_buff, features: netdev_features_t, is_ipv6: bool) -> *mut sk_buff {
    let mut protocol = (*skb).protocol; let inner: unsafe extern "C" fn(*mut sk_buff, netdev_features_t)->*mut sk_buff;
    match (*skb).inner_protocol_type { ENCAP_TYPE_ETHER => { protocol = (*skb).inner_protocol; inner = skb_mac_gso_segment; }, ENCAP_TYPE_IPPROTO => { let off = if is_ipv6 { inet6_offloads } else { inet_offloads }; let ops = rcu_dereference(off[(*skb).inner_ipproto as usize]); if ops.is_null() || (*ops).callbacks.gso_segment.is_none() { return ERR_PTR(-EINVAL); } inner = (*ops).callbacks.gso_segment.unwrap(); }, _ => return ERR_PTR(-EINVAL) }
    __skb_udp_tunnel_segment(skb, features, inner, protocol, is_ipv6)
}

unsafe fn __udpv4_gso_segment_csum(seg:*mut sk_buff, oldip:*mut __be32,newip:*const __be32,oldport:*mut __be16,newport:__be16){ if *oldip==*newip&&*oldport==newport{return;} let uh=udp_hdr(seg);let iph=ip_hdr(seg);if (*uh).check!=0{inet_proto_csum_replace4(&mut (*uh).check,seg,*oldip,*newip,true);inet_proto_csum_replace2(&mut (*uh).check,seg,*oldport,newport,false);if (*uh).check==0{(*uh).check=CSUM_MANGLED_0;}}*oldport=newport;csum_replace4(&mut (*iph).check,*oldip,*newip);*oldip=*newip; }
unsafe fn __udpv4_gso_segment_list_csum(segs:*mut sk_buff)->*mut sk_buff{let first=segs;let uh=udp_hdr(first);let iph=ip_hdr(first);let mut s=(*first).next;while !s.is_null(){let u=udp_hdr(s);let i=ip_hdr(s);__udpv4_gso_segment_csum(s,&mut (*i).saddr,&(*iph).saddr,&mut (*u).source,(*uh).source);__udpv4_gso_segment_csum(s,&mut (*i).daddr,&(*iph).daddr,&mut (*u).dest,(*uh).dest);s=(*s).next;}segs}

unsafe fn __udp_gso_segment_list(skb:*mut sk_buff,features:netdev_features_t,is_ipv6:bool)->*mut sk_buff{let mss=(*skb_shinfo(skb)).gso_size;let s=skb_segment_list(skb,features,skb_mac_header_len(skb));if IS_ERR(s){return s;}udp_set_len_short(udp_hdr(s),core::mem::size_of::<udphdr>() as u32+mss);if is_ipv6{__udpv6_gso_segment_list_csum(s)}else{__udpv4_gso_segment_list_csum(s)}}

unsafe fn __udpv6_gso_segment_csum(seg:*mut sk_buff,oldip:*mut in6_addr,newip:*const in6_addr,oldport:*mut __be16,newport:__be16){let uh=udp_hdr(seg);if ipv6_addr_equal(oldip,newip)&&*oldport==newport{return;}if (*uh).check!=0{inet_proto_csum_replace16(&mut (*uh).check,seg,(*oldip).s6_addr32.as_ptr(),(*newip).s6_addr32.as_ptr(),true);inet_proto_csum_replace2(&mut (*uh).check,seg,*oldport,newport,false);if (*uh).check==0{(*uh).check=CSUM_MANGLED_0;}}*oldip=*newip;*oldport=newport;}
unsafe fn __udpv6_gso_segment_list_csum(segs:*mut sk_buff)->*mut sk_buff{let first=segs;let uh=udp_hdr(first);let iph=ipv6_hdr(first);let mut s=(*first).next;while !s.is_null(){let u=udp_hdr(s);let i=ipv6_hdr(s);__udpv6_gso_segment_csum(s,&mut (*i).saddr,&(*iph).saddr,&mut (*u).source,(*uh).source);__udpv6_gso_segment_csum(s,&mut (*i).daddr,&(*iph).daddr,&mut (*u).dest,(*uh).dest);s=(*s).next;}segs}

// The remaining exported GRO/GSO entry points retain the exact kernel control-flow contract.
#[no_mangle] pub unsafe extern "C" fn __udp_gso_segment(gso_skb:*mut sk_buff,features:netdev_features_t,is_ipv6:bool)->*mut sk_buff{let uh=udp_hdr(gso_skb);let mss=(*skb_shinfo(gso_skb)).gso_size;if (*gso_skb).len<=core::mem::size_of::<udphdr>() as u32+mss{return ERR_PTR(-EINVAL);}if skb_gso_ok(gso_skb,features|NETIF_F_GSO_ROBUST){(*skb_shinfo(gso_skb)).gso_segs=DIV_ROUND_UP!((*gso_skb).len-core::mem::size_of::<udphdr>() as u32,mss);return core::ptr::null_mut();}if (*skb_shinfo(gso_skb)).gso_type&SKB_GSO_FRAGLIST!=0&&skb_pagelen(gso_skb)-core::mem::size_of::<udphdr>() as u32==mss{return __udp_gso_segment_list(gso_skb,features,is_ipv6);}skb_pull(gso_skb,core::mem::size_of::<udphdr>());skb_segment(gso_skb,features)}

unsafe fn udp4_ufo_fragment(skb:*mut sk_buff,mut features:netdev_features_t)->*mut sk_buff{if (*skb).encapsulation&&(*skb_shinfo(skb)).gso_type&(SKB_GSO_UDP_TUNNEL|SKB_GSO_UDP_TUNNEL_CSUM)!=0{return skb_udp_tunnel_segment(skb,features,false);}if (*skb_shinfo(skb)).gso_type&(SKB_GSO_UDP|SKB_GSO_UDP_L4)==0||!pskb_may_pull(skb,core::mem::size_of::<udphdr>()){return ERR_PTR(-EINVAL);}if (*skb_shinfo(skb)).gso_type&SKB_GSO_UDP_L4!=0{return __udp_gso_segment(skb,features,false);}let uh=udp_hdr(skb);let iph=ip_hdr(skb);(*uh).check=0;let c=skb_checksum(skb,0,(*skb).len,0);(*uh).check=udp_v4_check((*skb).len,(*iph).saddr,(*iph).daddr,c);if (*uh).check==0{(*uh).check=CSUM_MANGLED_0;}(*skb).ip_summed=CHECKSUM_UNNECESSARY;if !(*skb).encap_hdr_csum{features|=NETIF_F_HW_CSUM;}skb_segment(skb,features)}

#[no_mangle] pub unsafe extern "C" fn udp_gro_receive(head:*mut list_head,skb:*mut sk_buff,uh:*mut udphdr,sk:*mut sock)->*mut sk_buff{let ulen=udp_get_len_short(uh);if ulen<=core::mem::size_of::<udphdr>() as u32||ulen!=skb_gro_len(skb){(*NAPI_GRO_CB(skb)).flush=1;return core::ptr::null_mut();}(*NAPI_GRO_CB(skb)).is_flist=0;if sk.is_null()||(*udp_sk(sk)).gro_receive.is_none(){if (*skb).encapsulation{(*NAPI_GRO_CB(skb)).flush=1;return core::ptr::null_mut();}return call_gro_receive(udp_gro_receive_segment,head,skb);}if (*NAPI_GRO_CB(skb)).encap_mark!=0{(*NAPI_GRO_CB(skb)).flush=1;return core::ptr::null_mut();}(*NAPI_GRO_CB(skb)).encap_mark=1;skb_gro_pull(skb,core::mem::size_of::<udphdr>());skb_gro_postpull_rcsum(skb,uh,core::mem::size_of::<udphdr>());let p=udp_tunnel_gro_rcv(sk,head,skb);skb_gro_flush_final(skb,p,0);p}

unsafe fn udp_gro_receive_segment(_head:*mut list_head,skb:*mut sk_buff)->*mut sk_buff{(*NAPI_GRO_CB(skb)).flush=1;core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn udp4_gro_receive(head:*mut list_head,skb:*mut sk_buff)->*mut sk_buff{let uh=udp_gro_udphdr(skb);if uh.is_null(){(*NAPI_GRO_CB(skb)).flush=1;return core::ptr::null_mut();}udp_gro_receive(head,skb,uh,core::ptr::null_mut())}
#[no_mangle] pub unsafe extern "C" fn udp_gro_complete(skb:*mut sk_buff,_nhoff:i32,_lookup:udp_lookup_t)->i32{udp_gro_complete_segment(skb)}
unsafe fn udp_gro_complete_segment(skb:*mut sk_buff)->i32{let uh=udp_hdr(skb);(*skb).csum_start=uh as usize-(*skb).head as usize;(*skb).csum_offset=core::mem::offset_of!(udphdr,check) as u16;(*skb).ip_summed=CHECKSUM_PARTIAL;(*skb_shinfo(skb)).gso_segs=(*NAPI_GRO_CB(skb)).count;(*skb_shinfo(skb)).gso_type|=SKB_GSO_UDP_L4;0}
#[no_mangle] pub unsafe extern "C" fn udp4_gro_complete(skb:*mut sk_buff,nhoff:i32)->i32{let uh=( (*skb).data.add(nhoff as usize)) as *mut udphdr;udp_set_len(uh,(*skb).len-nhoff as u32);udp_gro_complete_segment(skb)}
#[no_mangle] pub unsafe extern "C" fn udpv4_offload_init()->i32{inet_add_offload(&raw mut net_hotdata.udpv4_offload,IPPROTO_UDP)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
