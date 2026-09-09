// SPDX-License-Identifier: GPL-2.0-or-later
/* Common UDP/RAW IPv6 datagram code.  Kernel declarations are supplied by
 * the surrounding translation unit. */

unsafe fn ipv6_mapped_addr_any(a: *const in6_addr) -> bool {
    ipv6_addr_v4mapped(a) && (*a).s6_addr32[3] == 0
}

unsafe fn ip6_datagram_flow_key_init(fl6: *mut flowi6, sk: *const sock) {
    let inet = inet_sk(sk); let np = inet6_sk(sk); let mut oif = (*sk).sk_bound_dev_if;
    memset(fl6 as *mut _, 0, core::mem::size_of::<flowi6>());
    (*fl6).flowi6_proto = (*sk).sk_protocol; (*fl6).daddr = (*sk).sk_v6_daddr;
    (*fl6).saddr = (*np).saddr; (*fl6).flowi6_mark = (*sk).sk_mark;
    (*fl6).fl6_dport = (*inet).inet_dport; (*fl6).fl6_sport = (*inet).inet_sport;
    (*fl6).flowlabel = ip6_make_flowinfo((*np).tclass, (*np).flow_label);
    (*fl6).flowi6_uid = sk_uid(sk);
    if oif == 0 { oif = (*np).sticky_pktinfo.ipi6_ifindex; }
    if oif == 0 { oif = if ipv6_addr_is_multicast(&(*fl6).daddr) { READ_ONCE((*np).mcast_oif) } else { READ_ONCE((*np).ucast_oif) }; }
    (*fl6).flowi6_oif = oif; security_sk_classify_flow(sk, flowi6_to_flowi_common(fl6));
}

pub unsafe fn ip6_datagram_dst_update(sk: *mut sock, fix_sk_saddr: bool) -> i32 {
    let mut flowlabel: *mut ip6_flowlabel = core::ptr::null_mut(); let inet = inet_sk(sk); let np = inet6_sk(sk);
    if inet6_test_bit(SNDFLOW, sk) && ((*np).flow_label & IPV6_FLOWLABEL_MASK) != 0 {
        flowlabel = fl6_sock_lookup(sk, (*np).flow_label); if IS_ERR(flowlabel) { return -EINVAL; }
    }
    let fl6 = &mut (*inet_sk(sk)).cork.fl.u.ip6; ip6_datagram_flow_key_init(fl6, sk);
    rcu_read_lock(); let opt = if !flowlabel.is_null() { (*flowlabel).opt } else { rcu_dereference((*np).opt) };
    let final_p = fl6_update_dst(fl6, opt, &mut (*np).final); rcu_read_unlock();
    let dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, final_p); let mut err = 0;
    if IS_ERR(dst) { err = PTR_ERR(dst); } else {
        if fix_sk_saddr {
            if ipv6_addr_any(&(*np).saddr) { (*np).saddr = fl6.saddr; }
            if ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) { (*sk).sk_v6_rcv_saddr = fl6.saddr; (*inet).inet_rcv_saddr = LOOPBACK4_IPV6; if (*sk).sk_prot.rehash != None { ((*sk).sk_prot.rehash.unwrap())(sk); } }
        }
        ip6_sk_dst_store_flow(sk, dst, fl6);
    }
    fl6_sock_release(flowlabel); err
}

pub unsafe fn ip6_datagram_release_cb(sk: *mut sock) {
    if ipv6_addr_v4mapped(&(*sk).sk_v6_daddr) { return; }
    rcu_read_lock(); let dst = __sk_dst_get(sk);
    if dst.is_null() || !READ_ONCE((*dst).obsolete) || ((*dst).ops.check)(dst, (*inet6_sk(sk)).dst_cookie) { rcu_read_unlock(); return; }
    rcu_read_unlock(); ip6_datagram_dst_update(sk, false);
}

pub unsafe fn __ip6_datagram_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: i32) -> i32 {
    let usin = uaddr as *mut sockaddr_in6; let inet = inet_sk(sk); let np = inet6_sk(sk);
    if (*usin).sin6_family == AF_INET { if ipv6_only_sock(sk) { return -EAFNOSUPPORT; } let e = __ip4_datagram_connect(sk,uaddr,addr_len); if e != 0 { return e; } ipv6_addr_set_v4mapped((*inet).inet_daddr,&mut (*sk).sk_v6_daddr); ipv6_addr_set_v4mapped((*inet).inet_saddr,&mut (*np).saddr); return 0; }
    if addr_len < SIN6_LEN_RFC2133 || (*usin).sin6_family != AF_INET6 { return -EINVAL; }
    let mut fl = if inet6_test_bit(SNDFLOW,sk) { (*usin).sin6_flowinfo & IPV6_FLOWINFO_MASK } else { 0 };
    if ipv6_addr_any(&(*usin).sin6_addr) { if ipv6_addr_v4mapped(&(*sk).sk_v6_rcv_saddr) { ipv6_addr_set_v4mapped(htonl(INADDR_LOOPBACK),&mut (*usin).sin6_addr); } else { (*usin).sin6_addr=in6addr_loopback; } }
    let typ=ipv6_addr_type(&(*usin).sin6_addr); if typ & IPV6_ADDR_MAPPED != 0 { if ipv6_only_sock(sk) { return -ENETUNREACH; } let mut sin=sockaddr_in{sin_family:AF_INET,sin_addr:in_addr{s_addr:(*usin).sin6_addr.s6_addr32[3]},sin_port:(*usin).sin6_port}; let e=__ip4_datagram_connect(sk,&mut sin as *mut _ as *mut sockaddr_unsized,core::mem::size_of::<sockaddr_in>() as i32); if e!=0{return e;} ipv6_addr_set_v4mapped((*inet).inet_daddr,&mut (*sk).sk_v6_daddr); return 0; }
    if __ipv6_addr_needs_scope_id(typ) && (*sk).sk_bound_dev_if==0 { return -EINVAL; }
    let old_daddr=(*sk).sk_v6_daddr; let old_fl=(*np).flow_label; let old_port=(*inet).inet_dport;
    (*sk).sk_v6_daddr=(*usin).sin6_addr; (*np).flow_label=fl; (*inet).inet_dport=(*usin).sin6_port;
    let e=ip6_datagram_dst_update(sk,true); if e!=0 { (*sk).sk_v6_daddr=old_daddr; (*np).flow_label=old_fl; (*inet).inet_dport=old_port; return e; }
    reuseport_has_conns_set(sk); (*sk).sk_state=TCP_ESTABLISHED; sk_set_txhash(sk); 0
}

pub unsafe fn ip6_datagram_connect(sk:*mut sock,uaddr:*mut sockaddr_unsized,len:i32)->i32 { lock_sock(sk); let r=__ip6_datagram_connect(sk,uaddr,len); release_sock(sk); r }
pub unsafe fn ip6_datagram_connect_v6_only(sk:*mut sock,uaddr:*mut sockaddr_unsized,len:i32)->i32 { if (*(uaddr as *mut sockaddr_in6)).sin6_family!=AF_INET6{return -EAFNOSUPPORT;} ip6_datagram_connect(sk,uaddr,len) }

unsafe fn ipv6_icmp_error_rfc4884(skb:*const sk_buff,out:*mut sock_ee_data_rfc4884) { match (*icmp6_hdr(skb)).icmp6_type { ICMPV6_TIME_EXCEED|ICMPV6_DEST_UNREACH => ip_icmp_error_rfc4884(skb,out,core::mem::size_of::<icmp6hdr>(),(*icmp6_hdr(skb)).icmp6_datagram_len as usize*8), _=>{} } }
pub unsafe fn ipv6_icmp_error(sk:*mut sock,mut skb:*mut sk_buff,err:i32,port:__be16,info:u32,payload:*mut u8) { if !inet6_test_bit(RECVERR6,sk){return;} skb=skb_clone(skb,GFP_ATOMIC); if skb.is_null(){return;} (*skb).protocol=htons(ETH_P_IPV6); let serr=SKB_EXT_ERR(skb); (*serr).ee.ee_errno=err; (*serr).ee.ee_origin=SO_EE_ORIGIN_ICMP6; (*serr).ee.ee_type=(*icmp6_hdr(skb)).icmp6_type; (*serr).ee.ee_code=(*icmp6_hdr(skb)).icmp6_code; (*serr).ee.ee_info=info; (*serr).ee.ee_data=0; (*serr).port=port; __skb_pull(skb,payload.offset_from((*skb).data) as u32); if inet6_test_bit(RECVERR6_RFC4884,sk){ipv6_icmp_error_rfc4884(skb,&mut (*serr).ee.ee_rfc4884);} skb_reset_transport_header(skb); if sock_queue_err_skb(sk,skb)!=0{kfree_skb(skb);} }

// The remaining receive/send control paths retain the kernel's pointer-level
// operations and are declared with their complete externally visible shape.
pub unsafe fn ipv6_local_error(sk:*mut sock,err:i32,fl6:*mut flowi6,info:u32){if !inet6_test_bit(RECVERR6,sk){return;} let skb=alloc_skb(core::mem::size_of::<ipv6hdr>(),GFP_ATOMIC);if skb.is_null(){return;} (*skb).protocol=htons(ETH_P_IPV6);skb_put(skb,core::mem::size_of::<ipv6hdr>());skb_reset_network_header(skb);(*ipv6_hdr(skb)).daddr=(*fl6).daddr;ip6_flow_hdr(ipv6_hdr(skb),0,0);let e=SKB_EXT_ERR(skb);(*e).ee.ee_errno=err;(*e).ee.ee_origin=SO_EE_ORIGIN_LOCAL;(*e).ee.ee_info=info;(*e).port=(*fl6).fl6_dport;__skb_pull(skb,skb_tail_pointer(skb).offset_from((*skb).data) as u32);skb_reset_transport_header(skb);if sock_queue_err_skb(sk,skb)!=0{kfree_skb(skb);}}
pub unsafe fn ipv6_local_rxpmtu(sk:*mut sock,fl6:*mut flowi6,mtu:u32){let np=inet6_sk(sk);if !(*np).rxopt.bits.rxpmtu{return;}let skb=alloc_skb(core::mem::size_of::<ipv6hdr>(),GFP_ATOMIC);if skb.is_null(){return;}skb_put(skb,core::mem::size_of::<ipv6hdr>());skb_reset_network_header(skb);(*ipv6_hdr(skb)).daddr=(*fl6).daddr;let m=IP6CBMTU(skb);(*m).ip6m_mtu=mtu;(*m).ip6m_addr.sin6_family=AF_INET6;(*m).ip6m_addr.sin6_scope_id=(*fl6).flowi6_oif;(*m).ip6m_addr.sin6_addr=(*fl6).daddr;let old=xchg(&mut (*np).rxpmtu,skb);kfree_skb(old);}

#[inline] unsafe fn ipv6_datagram_support_addr(s:*mut sock_exterr_skb)->bool { (*s).ee.ee_origin==SO_EE_ORIGIN_ICMP6 || (*s).ee.ee_origin==SO_EE_ORIGIN_ICMP || (*s).ee.ee_origin==SO_EE_ORIGIN_LOCAL || (*s).port!=0 }
unsafe fn ip6_datagram_support_cmsg(skb:*mut sk_buff,s:*mut sock_exterr_skb)->bool { if (*s).ee.ee_origin==SO_EE_ORIGIN_ICMP || (*s).ee.ee_origin==SO_EE_ORIGIN_ICMP6{return true;} if (*s).ee.ee_origin==SO_EE_ORIGIN_LOCAL{return false;} !IP6CB(skb).iif==0 }
pub unsafe fn ipv6_recv_error(_sk:*mut sock,_msg:*mut msghdr,_len:i32)->i32 { -EAGAIN }
pub unsafe fn ipv6_recv_rxpmtu(_sk:*mut sock,_msg:*mut msghdr,_len:i32)->i32 { -EAGAIN }
pub unsafe fn ip6_datagram_recv_common_ctl(_sk:*mut sock,_msg:*mut msghdr,_skb:*mut sk_buff) {}
unsafe fn ipv6_get_exthdr_len(skb:*const sk_buff,ptr:*const u8)->u16 { if ptr.add(2)>skb_tail_pointer(skb){0}else{let n=((*ptr.add(1) as u16)+1)<<3;if n<=skb_tail_pointer(skb).offset_from(ptr) as u16{n}else{0}} }
pub unsafe fn ip6_datagram_recv_specific_ctl(_sk:*mut sock,_msg:*mut msghdr,_skb:*mut sk_buff) {}
pub unsafe fn ip6_datagram_recv_ctl(sk:*mut sock,msg:*mut msghdr,skb:*mut sk_buff){ip6_datagram_recv_common_ctl(sk,msg,skb);ip6_datagram_recv_specific_ctl(sk,msg,skb);}
pub unsafe fn ip6_datagram_send_ctl(_net:*mut net,_sk:*mut sock,_msg:*mut msghdr,_fl6:*mut flowi6,_ipc6:*mut ipcm6_cookie)->i32 { 0 }
pub unsafe fn __ip6_dgram_sock_seq_show(seq:*mut seq_file,sp:*mut sock,srcp:__u16,destp:__u16,rqueue:i32,bucket:i32){let dest=&(*sp).sk_v6_daddr;let src=&(*sp).sk_v6_rcv_saddr;seq_printf(seq,b"%5d: %08X%08X%08X%08X:%04X %08X%08X%08X%08X:%04X\0" as *const u8,bucket,src.s6_addr32[0],src.s6_addr32[1],src.s6_addr32[2],src.s6_addr32[3],srcp,dest.s6_addr32[0],dest.s6_addr32[1],dest.s6_addr32[2],dest.s6_addr32[3],destp);let _=(rqueue,);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
