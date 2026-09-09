// SPDX-License-Identifier: GPL-2.0-only
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

unsafe fn nf_reject_iphdr_validate(skb: *mut sk_buff) -> c_int {
    let iph: *mut iphdr;
    let len: u32;
    if !pskb_may_pull(skb, core::mem::size_of::<iphdr>()) { return 0; }
    iph = ip_hdr(skb);
    if (*iph).ihl < 5 || (*iph).version != 4 { return 0; }
    len = ntohs((*iph).tot_len) as u32;
    if (*skb).len < len { return 0; }
    else if len < ((*iph).ihl as u32 * 4) { return 0; }
    if !pskb_may_pull(skb, (*iph).ihl as usize * 4) { return 0; }
    1
}

pub unsafe fn nf_reject_skb_v4_tcp_reset(net: *mut net, oldskb: *mut sk_buff,
    dev: *const net_device, hook: c_int) -> *mut sk_buff {
    let oth: *const tcphdr;
    let nskb: *mut sk_buff;
    let niph: *mut iphdr;
    let mut _oth: tcphdr = core::mem::zeroed();
    if nf_reject_iphdr_validate(oldskb) == 0 { return core::ptr::null_mut(); }
    oth = nf_reject_ip_tcphdr_get(oldskb, &mut _oth, hook);
    if oth.is_null() { return core::ptr::null_mut(); }
    nskb = alloc_skb(core::mem::size_of::<iphdr>() + core::mem::size_of::<tcphdr>() + LL_MAX_HEADER, GFP_ATOMIC);
    if nskb.is_null() { return core::ptr::null_mut(); }
    (*nskb).dev = dev as *mut net_device;
    skb_reserve(nskb, LL_MAX_HEADER);
    niph = nf_reject_iphdr_put(nskb, oldskb, IPPROTO_TCP, READ_ONCE((*net).ipv4.sysctl_ip_default_ttl));
    nf_reject_ip_tcphdr_put(nskb, oldskb, oth);
    (*niph).tot_len = htons((*nskb).len as u16);
    ip_send_check(niph);
    nskb
}

unsafe fn nf_skb_is_icmp_unreach(skb: *const sk_buff) -> bool {
    let iph = ip_hdr(skb as *mut sk_buff);
    let mut _type: u8 = 0;
    if (*iph).protocol != IPPROTO_ICMP { return false; }
    let thoff = skb_network_offset(skb) + ip_hdrlen(skb as *mut sk_buff);
    let tp = skb_header_pointer(skb as *mut sk_buff, thoff + core::mem::offset_of!(icmphdr, type_), 1, &mut _type as *mut _ as *mut c_void);
    !tp.is_null() && _type == ICMP_DEST_UNREACH
}

pub unsafe fn nf_reject_skb_v4_unreach(net: *mut net, oldskb: *mut sk_buff,
    dev: *const net_device, hook: c_int, code: u8) -> *mut sk_buff {
    let nskb: *mut sk_buff; let niph: *mut iphdr; let icmph: *mut icmphdr;
    let len: usize; let dataoff: c_int; let mut csum: __wsum; let proto: u8;
    if nf_reject_iphdr_validate(oldskb) == 0 { return core::ptr::null_mut(); }
    if (*ip_hdr(oldskb)).frag_off & htons(IP_OFFSET) != 0 || nf_skb_is_icmp_unreach(oldskb) { return core::ptr::null_mut(); }
    len = core::cmp::min(536usize, (*oldskb).len as usize);
    if !pskb_may_pull(oldskb, len) || pskb_trim_rcsum(oldskb, ntohs((*ip_hdr(oldskb)).tot_len) as usize) != 0 { return core::ptr::null_mut(); }
    dataoff = ip_hdrlen(oldskb); proto = (*ip_hdr(oldskb)).protocol;
    if !skb_csum_unnecessary(oldskb) && nf_reject_verify_csum(oldskb, dataoff, proto) && nf_ip_checksum(oldskb, hook, dataoff, proto) != 0 { return core::ptr::null_mut(); }
    nskb = alloc_skb(core::mem::size_of::<iphdr>() + core::mem::size_of::<icmphdr>() + LL_MAX_HEADER + len, GFP_ATOMIC);
    if nskb.is_null() { return core::ptr::null_mut(); }
    (*nskb).dev = dev as *mut net_device; skb_reserve(nskb, LL_MAX_HEADER);
    niph = nf_reject_iphdr_put(nskb, oldskb, IPPROTO_ICMP, READ_ONCE((*net).ipv4.sysctl_ip_default_ttl));
    skb_reset_transport_header(nskb); icmph = skb_put_zero(nskb, core::mem::size_of::<icmphdr>());
    (*icmph).type_ = ICMP_DEST_UNREACH; (*icmph).code = code;
    skb_put_data(nskb, skb_network_header(oldskb), len);
    csum = csum_partial(icmph as *mut c_void, len + core::mem::size_of::<icmphdr>(), 0); (*icmph).checksum = csum_fold(csum);
    (*niph).tot_len = htons((*nskb).len as u16); ip_send_check(niph); nskb
}

unsafe fn nf_reject_ip_tcphdr_get(oldskb: *mut sk_buff, _oth: *mut tcphdr, hook: c_int) -> *const tcphdr {
    if (*ip_hdr(oldskb)).frag_off & htons(IP_OFFSET) != 0 || (*ip_hdr(oldskb)).protocol != IPPROTO_TCP { return core::ptr::null(); }
    let oth = skb_header_pointer(oldskb, ip_hdrlen(oldskb), core::mem::size_of::<tcphdr>(), _oth as *mut c_void);
    if oth.is_null() || (*oth).rst || nf_ip_checksum(oldskb, hook, ip_hdrlen(oldskb), IPPROTO_TCP) != 0 { core::ptr::null() } else { oth as *const tcphdr }
}

unsafe fn nf_reject_iphdr_put(nskb: *mut sk_buff, oldskb: *mut sk_buff, protocol: u8, ttl: c_int) -> *mut iphdr {
    let oiph = ip_hdr(oldskb); skb_reset_network_header(nskb); let niph = skb_put(nskb, core::mem::size_of::<iphdr>());
    (*niph).version=4; (*niph).ihl=(core::mem::size_of::<iphdr>()/4) as u8; (*niph).tos=0; (*niph).id=0; (*niph).frag_off=htons(IP_DF); (*niph).protocol=protocol; (*niph).check=0; (*niph).saddr=(*oiph).daddr; (*niph).daddr=(*oiph).saddr; (*niph).ttl=ttl as u8; (*nskb).protocol=htons(ETH_P_IP); niph
}

unsafe fn nf_reject_ip_tcphdr_put(nskb: *mut sk_buff, oldskb: *mut sk_buff, oth: *const tcphdr) {
    let niph=ip_hdr(nskb); skb_reset_transport_header(nskb); let tcph=skb_put_zero(nskb, core::mem::size_of::<tcphdr>());
    (*tcph).source=(*oth).dest; (*tcph).dest=(*oth).source; (*tcph).doff=(core::mem::size_of::<tcphdr>()/4) as u8;
    if (*oth).ack { (*tcph).seq=(*oth).ack_seq; } else { (*tcph).ack_seq=htonl(ntohl((*oth).seq).wrapping_add((*oth).syn as u32).wrapping_add((*oth).fin as u32).wrapping_add(((*oldskb).len as i32 - ip_hdrlen(oldskb) - ((*oth).doff as i32 * 4)) as u32)); (*tcph).ack=1; }
    (*tcph).rst=1; (*tcph).check=!tcp_v4_check(core::mem::size_of::<tcphdr>() as u16, (*niph).saddr, (*niph).daddr, 0); (*nskb).ip_summed=CHECKSUM_PARTIAL; (*nskb).csum_start=tcph as *mut u8 as usize - (*nskb).head as usize; (*nskb).csum_offset=core::mem::offset_of!(tcphdr, check);
}

unsafe fn nf_reject_fill_skb_dst(skb_in: *mut sk_buff) -> c_int { let mut dst: *mut dst_entry=core::ptr::null_mut(); let mut fl: flowi=core::mem::zeroed(); (*fl.u.ip4()).daddr=(*ip_hdr(skb_in)).saddr; nf_ip_route(dev_net((*skb_in).dev), &mut dst, &mut fl, false); if dst.is_null(){-1}else{skb_dst_drop(skb_in);skb_dst_set(skb_in,dst);0} }

pub unsafe fn nf_send_reset(net:*mut net, sk:*mut sock, oldskb:*mut sk_buff, hook:c_int){ let mut oth:tcphdr=core::mem::zeroed(); let oth=nf_reject_ip_tcphdr_get(oldskb,&mut oth,hook); if oth.is_null(){return;} if !skb_valid_dst(oldskb)&&nf_reject_fill_skb_dst(oldskb)<0{return;} if skb_rtable(oldskb).rt_flags&(RTCF_BROADCAST|RTCF_MULTICAST)!=0{return;} let nskb=alloc_skb(core::mem::size_of::<iphdr>()+core::mem::size_of::<tcphdr>()+LL_MAX_HEADER,GFP_ATOMIC); if nskb.is_null(){return;} skb_dst_set_noref(nskb,skb_dst(oldskb)); (*nskb).mark=IP4_REPLY_MARK(net,(*oldskb).mark); skb_reserve(nskb,LL_MAX_HEADER); nf_reject_iphdr_put(nskb,oldskb,IPPROTO_TCP,ip4_dst_hoplimit(skb_dst(nskb))); nf_reject_ip_tcphdr_put(nskb,oldskb,oth); if ip_route_me_harder(net,sk,nskb,RTN_UNSPEC)!=0||(*nskb).len>dst4_mtu(skb_dst(nskb)){kfree_skb(nskb);return;} nf_ct_attach(nskb,oldskb); nf_ct_set_closing(skb_nfct(oldskb)); ip_local_out(net,(*nskb).sk,nskb); }

pub unsafe fn nf_send_unreach(skb_in:*mut sk_buff, code:c_int, hook:c_int){let iph=ip_hdr(skb_in);let dataoff=ip_hdrlen(skb_in);let proto=(*iph).protocol;if (*iph).frag_off&htons(IP_OFFSET)!=0{return;}if !skb_valid_dst(skb_in)&&nf_reject_fill_skb_dst(skb_in)<0{return;}if skb_csum_unnecessary(skb_in)||!nf_reject_verify_csum(skb_in,dataoff,proto)||nf_ip_checksum(skb_in,hook,dataoff,proto)==0{icmp_send(skb_in,ICMP_DEST_UNREACH,code,0);}}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
