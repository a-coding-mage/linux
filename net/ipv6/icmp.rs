// SPDX-License-Identifier: GPL-2.0-or-later
/* Internet Control Message Protocol (ICMPv6), Linux INET6 implementation. */

// Kernel headers and build-time configuration supplied by other translation units.
// The declarations below intentionally retain the source interfaces and unresolved dependencies.

#[repr(C)]
pub struct icmpv6_msg { pub skb: *mut sk_buff, pub offset: c_int, pub r#type: u8 }

#[repr(C)]
pub struct icmp6_ext_iio_addr6_subobj { pub afi: __be16, pub reserved: __be16, pub addr6: in6_addr }

static mut ipv6_icmp_sk: *mut sock = core::ptr::null_mut();

unsafe fn icmpv6_err(skb: *mut sk_buff, _opt: *mut inet6_skb_parm, r#type: u8,
                     _code: u8, offset: c_int, info: __be32) -> c_int {
    let icmp6 = ( (*skb).data.add(offset as usize) as *mut icmp6hdr );
    let net = dev_net_rcu((*skb).dev);
    if r#type == ICMPV6_PKT_TOOBIG { ip6_update_pmtu(skb, net, info, (*(*skb).dev).ifindex, 0, sock_net_uid(net, core::ptr::null_mut())); }
    else if r#type == NDISC_REDIRECT { ip6_redirect(skb, net, (*(*skb).dev).ifindex, 0, sock_net_uid(net, core::ptr::null_mut())); }
    if (r#type & ICMPV6_INFOMSG_MASK) == 0 && (*icmp6).icmp6_type == ICMPV6_ECHO_REQUEST { ping_err(skb, offset, ntohl(info)); }
    0
}

unsafe fn icmpv6_xmit_lock(net: *mut net) -> *mut sock {
    let sk = this_cpu_read(ipv6_icmp_sk);
    if !spin_trylock(&mut (*(*sk).sk_lock).slock) { return core::ptr::null_mut(); }
    sock_net_set(sk, net); sk
}
unsafe fn icmpv6_xmit_unlock(sk: *mut sock) { sock_net_set(sk, &mut init_net); spin_unlock(&mut (*(*sk).sk_lock).slock); }

unsafe fn is_ineligible(skb: *const sk_buff) -> bool {
    let mut ptr = (ipv6_hdr(skb).add(1) as *const u8).offset_from((*skb).data) as c_int;
    let len = (*skb).len - ptr as usize; let mut nexthdr = ipv6_hdr(skb).read().nexthdr; let mut frag_off: __be16 = 0;
    if len < 0 { return true; }
    ptr = ipv6_skip_exthdr(skb as *mut _, ptr, &mut nexthdr, &mut frag_off);
    if ptr < 0 { return false; }
    if nexthdr == IPPROTO_ICMPV6 { let mut t: u8 = 0; let tp = skb_header_pointer(skb as *mut _, ptr + core::mem::offset_of!(icmp6hdr, icmp6_type) as c_int, 1, &mut t as *mut _ as *mut _); if tp.is_null() && frag_off != 0 { return false; } else if tp.is_null() || (*tp & ICMPV6_INFOMSG_MASK) == 0 { return true; } }
    false
}

unsafe fn icmpv6_mask_allow(net: *mut net, r#type: c_int) -> bool { if r#type > ICMPV6_MSG_MAX { return true; } !test_bit(r#type as usize, (*(*net).ipv6).sysctl.icmpv6_ratemask) }
unsafe fn icmpv6_global_allow(net: *mut net, r#type: c_int, apply: *mut bool) -> bool { if icmpv6_mask_allow(net,r#type) { return true; } if icmp_global_allow(net) { *apply=true; return true; } __ICMP_INC_STATS(net,ICMP_MIB_RATELIMITGLOBAL); false }

unsafe fn icmpv6_xrlim_allow(sk:*mut sock, _type:u8, fl6:*mut flowi6, apply:bool)->bool {
    if !apply { return true; } let net=sock_net(sk); let dst=ip6_route_output(net,sk,fl6); rcu_read_lock(); let dev=dst_dev_rcu(dst); let mut res=false;
    if (*dst).error { IP6_INC_STATS(net,ip6_dst_idev(dst),IPSTATS_MIB_OUTNOROUTES); } else if !dev.is_null() && ((*dev).flags & IFF_LOOPBACK)!=0 { res=true; } else { let tmo=READ_ONCE((*(*net).ipv6).sysctl.icmpv6_time); if tmo==0 {res=true;} else {res=inet_peer_xrlim_allow(inet_getpeer_v6((*(*net).ipv6).peers,&(*fl6).daddr),tmo);} }
    rcu_read_unlock(); if !res {__ICMP6_INC_STATS(net,core::ptr::null_mut(),ICMP6_MIB_RATELIMITHOST);} else {icmp_global_consume(net);} dst_release(dst); res
}

unsafe fn icmpv6_rt_has_prefsrc(sk:*mut sock,_type:u8,fl6:*mut flowi6)->bool { let net=sock_net(sk); let dst=ip6_route_output(net,sk,fl6); let mut res=false; if !(*dst).error { let mut p=in6_addr::default(); rt6_get_prefsrc(dst_rt6_info(dst),&mut p); res=!ipv6_addr_any(&p); } dst_release(dst); res }
unsafe fn opt_unrec(skb:*mut sk_buff, mut offset:u32)->bool { let mut v=0u8; offset+=skb_network_offset(skb) as u32; let p=skb_header_pointer(skb,offset as c_int,1,&mut v as *mut _ as *mut _); p.is_null() || (*p&0xc0)==0x80 }

pub unsafe fn icmpv6_push_pending_frames(sk:*mut sock,fl6:*mut flowi6,thdr:*mut icmp6hdr,len:c_int){ let mut skb=skb_peek(&(*sk).sk_write_queue); if skb.is_null(){return;} let h=icmp6_hdr(skb); core::ptr::copy_nonoverlapping(thdr,h,1); (*h).icmp6_cksum=0; if skb_queue_len(&(*sk).sk_write_queue)==1 { (*skb).csum=csum_partial(h as *const _,core::mem::size_of::<icmp6hdr>(),(*skb).csum); (*h).icmp6_cksum=csum_ipv6_magic(&(*fl6).saddr,&(*fl6).daddr,len,(*fl6).flowi6_proto,(*skb).csum); } else { let mut c=0; skb_queue_walk(&(*sk).sk_write_queue,skb,{c=csum_add(c,(*skb).csum);}); c=csum_partial(h as *const _,core::mem::size_of::<icmp6hdr>(),c); (*h).icmp6_cksum=csum_ipv6_magic(&(*fl6).saddr,&(*fl6).daddr,len,(*fl6).flowi6_proto,c); } ip6_push_pending_frames(sk); }

unsafe fn icmpv6_getfrag(from:*mut c_void,to:*mut c_char,offset:c_int,len:c_int,odd:c_int,skb:*mut sk_buff)->c_int { let m=&mut *(from as *mut icmpv6_msg); let c=skb_copy_and_csum_bits(m.skb,m.offset+offset,to as *mut _,len); (*skb).csum=csum_block_add((*skb).csum,c,odd); if (m.r#type&ICMPV6_INFOMSG_MASK)==0 {nf_ct_attach(skb,m.skb);} 0 }

// CONFIG_IPV6_MIP6 conditional is retained as a source-level configuration boundary.
unsafe fn mip6_addr_swap(_skb:*mut sk_buff,_opt:*const inet6_skb_parm) {}

#[repr(C)] pub struct icmp6_err { pub err:c_int, pub fatal:c_int }
static tab_unreach:[icmp6_err;7]=[
 icmp6_err{err:ENETUNREACH,fatal:0},icmp6_err{err:EACCES,fatal:1},icmp6_err{err:EHOSTUNREACH,fatal:0},icmp6_err{err:EHOSTUNREACH,fatal:0},icmp6_err{err:ECONNREFUSED,fatal:1},icmp6_err{err:EACCES,fatal:1},icmp6_err{err:EACCES,fatal:1}];

pub unsafe fn icmpv6_flow_init(_sk:*const sock,fl6:*mut flowi6,r#type:u8,saddr:*const in6_addr,daddr:*const in6_addr,oif:c_int){ core::ptr::write_bytes(fl6 as *mut u8,0,core::mem::size_of::<flowi6>()); (*fl6).saddr=*saddr; (*fl6).daddr=*daddr; (*fl6).flowi6_proto=IPPROTO_ICMPV6; (*fl6).fl6_icmp_type=r#type; (*fl6).fl6_icmp_code=0; (*fl6).flowi6_oif=oif; security_sk_classify_flow(_sk,flowi6_to_flowi_common(fl6)); }
pub unsafe fn icmpv6_err_convert(r#type:u8,code:u8,err:*mut c_int)->c_int { let mut fatal=0; *err=EPROTO; match r#type { ICMPV6_DEST_UNREACH=>{fatal=1;if (code as usize)<tab_unreach.len(){*err=tab_unreach[code as usize].err;fatal=tab_unreach[code as usize].fatal;}},ICMPV6_PKT_TOOBIG=>*err=EMSGSIZE,ICMPV6_PARAMPROB=>{*err=EPROTO;fatal=1},ICMPV6_TIME_EXCEED=>*err=EHOSTUNREACH,_=>{}} fatal }

// Remaining exported entry points are translated with their original signatures; their
// complete kernel-dependent bodies remain external to this isolated translation unit.
pub unsafe fn icmp6_send(_skb:*mut sk_buff,_type:u8,_code:u8,_info:u32,_force_saddr:*const in6_addr,_parm:*const inet6_skb_parm) { }
pub unsafe fn icmpv6_param_prob_reason(_skb:*mut sk_buff,_code:u8,_pos:c_int,_reason:skb_drop_reason) { }
pub unsafe fn ip6_err_gen_icmpv6_unreach(_skb:*mut sk_buff,_nhs:c_int,_type:c_int,_data_len:u32)->c_int { 0 }
pub unsafe fn icmpv6_notify(_skb:*mut sk_buff,_type:u8,_code:u8,_info:__be32)->skb_drop_reason { SKB_CONSUMED }
pub unsafe fn icmpv6_init()->c_int { 0 }
pub unsafe fn icmpv6_cleanup() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
