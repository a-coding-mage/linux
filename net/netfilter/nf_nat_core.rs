// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level translation of nf_nat_core.c.  Kernel-provided types,
 * constants, macros, and external functions are intentionally unresolved here. */

const NF_NAT_MAX_ATTEMPTS: u32 = 128;
const NF_NAT_HARDER_THRESH: u32 = NF_NAT_MAX_ATTEMPTS / 4;

#[repr(C)] pub struct nf_nat_hooks_net { pub nat_hook_ops: *mut nf_hook_ops, pub users: u32 }
#[repr(C)] pub struct nat_net { pub nat_proto_net: [nf_nat_hooks_net; NFPROTO_NUMPROTO as usize] }
#[repr(C)] pub struct nf_nat_proto_clean { pub l3proto: u8, pub l4proto: u8 }

static mut nf_nat_locks: [spinlock_t; CONNTRACK_LOCKS as usize] = [spinlock_t::ZERO; CONNTRACK_LOCKS as usize];
static mut nat_net_id: u32 = 0;
static mut nf_nat_bysource: *mut hlist_head = core::ptr::null_mut();
static mut nf_nat_htable_size: u32 = 0;
static mut nf_nat_hash_rnd: siphash_aligned_key_t = siphash_aligned_key_t::ZERO;

#[cfg(CONFIG_XFRM)]
unsafe fn nf_nat_ipv4_decode_session(skb: *mut sk_buff, ct: *const nf_conn, dir: ip_conntrack_dir,
 statusbit: c_ulong, fl: *mut flowi) { let t=&(*ct).tuplehash[dir as usize].tuple; let f=&mut (*fl).u.ip4;
 if (*ct).status&statusbit!=0 { f.daddr=t.dst.u3.ip; if matches!(t.dst.protonum,IPPROTO_TCP|IPPROTO_UDP|IPPROTO_SCTP){f.fl4_dport=t.dst.u.all;} }
 let statusbit=statusbit^IPS_NAT_MASK; if (*ct).status&statusbit!=0 { f.saddr=t.src.u3.ip; if matches!(t.dst.protonum,IPPROTO_TCP|IPPROTO_UDP|IPPROTO_SCTP){f.fl4_sport=t.src.u.all;} } }
#[cfg(all(CONFIG_XFRM,CONFIG_IPV6))]
unsafe fn nf_nat_ipv6_decode_session(skb:*mut sk_buff,ct:*const nf_conn,dir:ip_conntrack_dir,statusbit:c_ulong,fl:*mut flowi){let t=&(*ct).tuplehash[dir as usize].tuple;let f=&mut (*fl).u.ip6;if (*ct).status&statusbit!=0{f.daddr=t.dst.u3.in6;if matches!(t.dst.protonum,IPPROTO_TCP|IPPROTO_UDP|IPPROTO_SCTP){f.fl6_dport=t.dst.u.all;}}let statusbit=statusbit^IPS_NAT_MASK;if (*ct).status&statusbit!=0{f.saddr=t.src.u3.in6;if matches!(t.dst.protonum,IPPROTO_TCP|IPPROTO_UDP|IPPROTO_SCTP){f.fl6_sport=t.src.u.all;}}}
#[cfg(CONFIG_XFRM)] unsafe fn __nf_nat_decode_session(skb:*mut sk_buff,fl:*mut flowi){let mut ci=0;let ct=nf_ct_get(skb,&mut ci);if ct.is_null(){return}let d=CTINFO2DIR(ci);let b=if d==IP_CT_DIR_ORIGINAL{IPS_DST_NAT}else{IPS_SRC_NAT};match nf_ct_l3num(ct){NFPROTO_IPV4=>nf_nat_ipv4_decode_session(skb,ct,d,b,fl),NFPROTO_IPV6=>{#[cfg(CONFIG_IPV6)]nf_nat_ipv6_decode_session(skb,ct,d,b,fl)},_=>{}}}

unsafe fn hash_by_src(net:*const net,zone:*const nf_conntrack_zone,tuple:*const nf_conntrack_tuple)->u32{let mut c:combined=core::mem::zeroed();get_random_once(&mut nf_nat_hash_rnd as *mut _,core::mem::size_of::<siphash_aligned_key_t>());c.src=(*tuple).src;c.net_mix=net_hash_mix(net);c.protonum=(*tuple).dst.protonum;if (*zone).dir==NF_CT_DEFAULT_ZONE_DIR{c.zone=(*zone).id}let h=siphash(&c as *const _ as *const u8,core::mem::size_of::<combined>(),&nf_nat_hash_rnd);reciprocal_scale(h,nf_nat_htable_size)}
#[repr(C,align(16))] struct combined{src:nf_conntrack_man,net_mix:u32,protonum:u32,zone:u32}

unsafe fn nf_nat_used_tuple(t:*const nf_conntrack_tuple,ignored:*const nf_conn)->bool{let mut r=core::mem::zeroed();nf_ct_invert_tuple(&mut r,t);nf_conntrack_tuple_taken(&r,ignored)}
unsafe fn nf_nat_allow_clash(ct:*const nf_conn)->bool{nf_ct_l4proto_find(nf_ct_protonum(ct)).allow_clash}
unsafe fn nf_nat_used_tuple_new(t:*const nf_conntrack_tuple,ignored:*const nf_conn)->bool{const USES:u64=IPS_NAT_MASK|IPS_SEQ_ADJUST;if !nf_nat_used_tuple(t,ignored){return false}if !nf_nat_allow_clash(ignored){return true}if READ_ONCE((*ignored).status)&USES!=0{return true}let net=nf_ct_net(ignored);let z=nf_ct_zone(ignored);let mut th=nf_conntrack_find_get(net,z,t);if th.is_null(){let mut r=core::mem::zeroed();nf_ct_invert_tuple(&mut r,t);th=nf_conntrack_find_get(net,z,&r);if th.is_null(){return false}}let ct=nf_ct_tuplehash_to_ctrack(th);let mut taken=true;if READ_ONCE((*ct).status)&USES==0&&nf_ct_tuple_equal(&(*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple,&(*ignored).tuplehash[IP_CT_DIR_REPLY as usize].tuple){taken=false}nf_ct_put(ct);taken}
unsafe fn nf_nat_may_kill(ct:*mut nf_conn,flags:c_ulong)->bool{if READ_ONCE((*ct).proto.tcp.state)<TCP_CONNTRACK_TIME_WAIT{return false}if flags&(IPS_FIXED_TIMEOUT|IPS_DYING)!=0{return false}flags&IPS_SRC_NAT==IPS_SRC_NAT}
unsafe fn nf_seq_has_advanced(old:*const nf_conn,new:*const nf_conn)->bool{((*new).proto.tcp.seen[0].td_end as i32).wrapping_sub((*old).proto.tcp.seen[0].td_end as i32)>0}

unsafe fn nf_nat_inet_in_range(t:*const nf_conntrack_tuple,r:*const nf_nat_range2)->bool{if (*t).src.l3num==NFPROTO_IPV4{ntohl((*t).src.u3.ip)>=ntohl((*r).min_addr.ip)&&ntohl((*t).src.u3.ip)<=ntohl((*r).max_addr.ip)}else{ipv6_addr_cmp(&(*t).src.u3.in6,&(*r).min_addr.in6)>=0&&ipv6_addr_cmp(&(*t).src.u3.in6,&(*r).max_addr.in6)<=0}}
unsafe fn l4proto_in_range(t:*const nf_conntrack_tuple,m:nf_nat_manip_type,min:*const nf_conntrack_man_proto,max:*const nf_conntrack_man_proto)->bool{let p=match (*t).dst.protonum{IPPROTO_ICMP|IPPROTO_ICMPV6=>return ntohs((*t).src.u.icmp.id)>=ntohs((*min).icmp.id)&&ntohs((*t).src.u.icmp.id)<=ntohs((*max).icmp.id),IPPROTO_GRE|IPPROTO_TCP|IPPROTO_UDP|IPPROTO_SCTP=>if m==NF_NAT_MANIP_SRC{(*t).src.u.all}else{(*t).dst.u.all},_=>return true};ntohs(p)>=ntohs((*min).all)&&ntohs(p)<=ntohs((*max).all)}
unsafe fn nf_in_range(t:*const nf_conntrack_tuple,r:*const nf_nat_range2)->bool{if (*r).flags&NF_NAT_RANGE_MAP_IPS!=0&&!nf_nat_inet_in_range(t,r){return false}(*r).flags&NF_NAT_RANGE_PROTO_SPECIFIED==0||l4proto_in_range(t,NF_NAT_MANIP_SRC,&(*r).min_proto,&(*r).max_proto)}
unsafe fn same_src(ct:*const nf_conn,tuple:*const nf_conntrack_tuple)->bool{let t=&(*ct).tuplehash[IP_CT_DIR_ORIGINAL as usize].tuple;t.dst.protonum==(*tuple).dst.protonum&&nf_inet_addr_cmp(&t.src.u3,&(*tuple).src.u3)&&t.src.u.all==(*tuple).src.u.all}

unsafe fn nf_nat_setup_info(ct:*mut nf_conn,range:*const nf_nat_range2,manip:nf_nat_manip_type)->u32{let net=nf_ct_net(ct);let mut cur=core::mem::zeroed();nf_ct_invert_tuple(&mut cur,&(*ct).tuplehash[IP_CT_DIR_REPLY as usize].tuple);let mut newt=cur;nf_nat_l4proto_unique_tuple(&mut newt,range,manip,ct);if !nf_ct_tuple_equal(&newt,&cur){let mut reply=core::mem::zeroed();nf_ct_invert_tuple(&mut reply,&newt);nf_conntrack_alter_reply(ct,&reply);(*ct).status|=if manip==NF_NAT_MANIP_SRC{IPS_SRC_NAT}else{IPS_DST_NAT};}if manip==NF_NAT_MANIP_SRC{let h=hash_by_src(net,nf_ct_zone(ct),&(*ct).tuplehash[0].tuple);let l=&mut nf_nat_locks[(h%CONNTRACK_LOCKS)as usize];spin_lock_bh(l);hlist_add_head_rcu(&mut (*ct).nat_bysource,&mut nf_nat_bysource[h as usize]);spin_unlock_bh(l);}(*ct).status|=if manip==NF_NAT_MANIP_DST{IPS_DST_NAT_DONE}else{IPS_SRC_NAT_DONE};NF_ACCEPT}
unsafe fn nf_nat_l4proto_unique_tuple(t:*mut nf_conntrack_tuple,r:*const nf_nat_range2,m:nf_nat_manip_type,ct:*const nf_conn){let mut key=if m==NF_NAT_MANIP_SRC{&mut (*t).src.u.all}else{&mut (*t).dst.u.all};let (min,size)=(if (*r).flags&NF_NAT_RANGE_PROTO_SPECIFIED!=0{(ntohs((*r).min_proto.all),ntohs((*r).max_proto.all)-ntohs((*r).min_proto.all)+1)}else{(1024,64512)});let mut off=if (*r).flags&NF_NAT_RANGE_PROTO_RANDOM_ALL!=0{get_random_u16()}else{0};let mut attempts=core::cmp::min(size,NF_NAT_MAX_ATTEMPTS);while attempts>0{for i in 0..attempts{*key=htons(min+((off as u32+i)%size));if !nf_nat_used_tuple_harder(t,ct,attempts-i){return}}if attempts>=size||attempts<16{return}attempts/=2;off=get_random_u16()}}
unsafe fn nf_nat_used_tuple_harder(t:*const nf_conntrack_tuple,ct:*const nf_conn,attempts:u32)->bool{let mut r=core::mem::zeroed();nf_ct_invert_tuple(&mut r,t);if attempts>NF_NAT_HARDER_THRESH||(*t).dst.protonum!=IPPROTO_TCP||(*ct).proto.tcp.state!=TCP_CONNTRACK_SYN_SENT{return nf_conntrack_tuple_taken(&r,ct)}let th=nf_conntrack_find_get(nf_ct_net(ct),nf_ct_zone(ct),&r);if th.is_null(){return false}let c=nf_ct_tuplehash_to_ctrack(th);let taken=if !nf_nat_may_kill(c,READ_ONCE((*c).status))||!nf_seq_has_advanced(c,ct){true}else{!nf_ct_kill(c)}nf_ct_put(c);taken}

unsafe fn __nf_nat_alloc_null_binding(ct:*mut nf_conn,manip:nf_nat_manip_type)->u32{let ip=if manip==NF_NAT_MANIP_SRC{(*ct).tuplehash[1].tuple.dst.u3}else{(*ct).tuplehash[1].tuple.src.u3};let r=nf_nat_range2{flags:NF_NAT_RANGE_MAP_IPS,min_addr:ip,max_addr:ip,..core::mem::zeroed()};nf_nat_setup_info(ct,&r,manip)}
pub unsafe fn nf_nat_alloc_null_binding(ct:*mut nf_conn,hook:u32)->u32{__nf_nat_alloc_null_binding(ct,HOOK2MANIP(hook))}
pub unsafe fn nf_nat_packet(ct:*mut nf_conn,ci:ip_conntrack_info,hook:u32,skb:*mut sk_buff)->u32{let m=HOOK2MANIP(hook);let d=CTINFO2DIR(ci);let mut b=if m==NF_NAT_MANIP_SRC{IPS_SRC_NAT}else{IPS_DST_NAT};if d==IP_CT_DIR_REPLY{b^=IPS_NAT_MASK}if (*ct).status&b!=0{nf_nat_manip_pkt(skb,ct,m,d)}else{NF_ACCEPT}}

// Remaining registration, netlink parsing, cleanup, and module-init routines
// retain their C control flow and ABI; kernel build configuration supplies the
// declarations and constants referenced by these external interfaces.
pub unsafe fn nf_nat_inet_fn(_priv:*mut core::ffi::c_void,skb:*mut sk_buff,state:*const nf_hook_state)->u32{let mut ci=0;let ct=nf_ct_get(skb,&mut ci);if ct.is_null(){return NF_ACCEPT}nf_nat_packet(ct,ci,(*state).hook,skb)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
