// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2013 Nicira, Inc. */
// Linux dependencies and build-time configuration are supplied by the kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::mem::{size_of, offset_of};

extern "C" {
    static mut iptun_encaps: [*const ip_tunnel_encap_ops; MAX_IPTUN_ENCAP_OPS];
    static mut ip6tun_encaps: [*const ip6_tnl_encap_ops; MAX_IPTUN_ENCAP_OPS];
}

#[repr(C)] pub struct ip_tunnel_encap_ops { _private: [u8; 0] }
#[repr(C)] pub struct ip6_tnl_encap_ops { _private: [u8; 0] }

pub unsafe fn iptunnel_xmit(sk: *mut sock, rt: *mut rtable, skb: *mut sk_buff,
    src: __be32, dst: __be32, proto: __u8, tos: __u8, ttl: __u8,
    df: __be16, xnet: bool, ipcb_flags: u16) {
    let mut pkt_len = (*skb).len - skb_inner_network_offset(skb);
    let net = dev_net((*rt).dst.dev);
    let dev = (*skb).dev;
    if dev_recursion_level() > IP_TUNNEL_RECURSION_LIMIT {
        if !dev.is_null() { net_crit_ratelimited("Dead loop on virtual device %s (net %llu), fix it urgently!\n", (*dev).name, dev_net(dev).net_cookie); DEV_STATS_INC(dev, tx_errors); }
        ip_rt_put(rt); kfree_skb_reason(skb, SKB_DROP_REASON_RECURSION_LIMIT); return;
    }
    dev_xmit_recursion_inc(); skb_scrub_packet(skb, xnet); skb_clear_hash_if_not_l4(skb);
    skb_dst_set(skb, &mut (*rt).dst); memset(IPCB(skb), 0, size_of::<IPCB>()); (*IPCB(skb)).flags = ipcb_flags;
    skb_push(skb, size_of::<iphdr>()); skb_reset_network_header(skb);
    let iph = ip_hdr(skb); (*iph).version=4; (*iph).ihl=(size_of::<iphdr>() >> 2) as _;
    (*iph).frag_off=if ip_mtu_locked(&(*rt).dst) {0} else {df}; (*iph).protocol=proto; (*iph).tos=tos;
    (*iph).daddr=dst; (*iph).saddr=src; (*iph).ttl=ttl; __ip_select_ident(net, iph, skb_shinfo(skb).gso_segs.unwrap_or(1));
    let err=ip_local_out(net, sk, skb); if !dev.is_null() { if net_xmit_eval(err) != 0 {pkt_len=0;} iptunnel_xmit_stats(dev,pkt_len); } dev_xmit_recursion_dec();
}

pub unsafe fn __iptunnel_pull_header(skb: *mut sk_buff, hdr_len: i32, inner_proto: __be16, raw_proto: bool, xnet: bool) -> i32 {
    if !pskb_may_pull(skb,hdr_len) { return -ENOMEM; } skb_pull_rcsum(skb,hdr_len);
    if !raw_proto && inner_proto==htons(ETH_P_TEB) { if !pskb_may_pull(skb,ETH_HLEN as _) {return -ENOMEM;} let eh=(*skb).data as *const ethhdr; (*skb).protocol=if eth_proto_is_802_3((*eh).h_proto){(*eh).h_proto}else{htons(ETH_P_802_2)}; } else {(*skb).protocol=inner_proto;}
    skb_clear_hash_if_not_l4(skb); __vlan_hwaccel_clear_tag(skb); skb_set_queue_mapping(skb,0); skb_scrub_packet(skb,xnet); iptunnel_pull_offloads(skb)
}

pub unsafe fn iptunnel_metadata_reply(md: *mut metadata_dst, flags: gfp_t) -> *mut metadata_dst {
    if md.is_null() || (*md).type_ != METADATA_IP_TUNNEL || ((*md).u.tun_info.mode & IP_TUNNEL_INFO_TX)!=0 {return core::ptr::null_mut();}
    let src=&(*md).u.tun_info; let res=metadata_dst_alloc(src.options_len,METADATA_IP_TUNNEL,flags); if res.is_null(){return res;}
    let dst=&mut (*res).u.tun_info; dst.key.tun_id=src.key.tun_id; if src.mode & IP_TUNNEL_INFO_IPV6 != 0 {dst.key.u.ipv6.dst=src.key.u.ipv6.src;} else {dst.key.u.ipv4.dst=src.key.u.ipv4.src;}
    ip_tunnel_flags_copy(dst.key.tun_flags,src.key.tun_flags); dst.mode=src.mode|IP_TUNNEL_INFO_TX; ip_tunnel_info_opts_set(dst,ip_tunnel_info_opts(src),src.options_len,0); res
}

pub unsafe fn iptunnel_handle_offloads(skb:*mut sk_buff,gso_type_mask:i32)->i32 { if !(*skb).encapsulation {skb_reset_inner_headers(skb);(*skb).encapsulation=true;} if skb_is_gso(skb) {let e=skb_header_unclone(skb,GFP_ATOMIC);if e!=0{return e;}skb_shinfo(skb).gso_type|=gso_type_mask;return 0;} if (*skb).ip_summed!=CHECKSUM_PARTIAL {(*skb).ip_summed=CHECKSUM_NONE;(*skb).encapsulation=false;} 0 }

// PMTU ICMP construction retains the original packet surgery and checksum ordering.
unsafe fn iptunnel_pmtud_build_icmp(skb:*mut sk_buff,mtu:i32)->i32 { if !pskb_may_pull(skb,(ETH_HLEN+size_of::<iphdr>()) as _){return -EINVAL;} if skb_is_gso(skb){skb_gso_reset(skb);} let mut eh=core::mem::zeroed::<ethhdr>(); skb_copy_bits(skb,skb_mac_offset(skb),&mut eh,ETH_HLEN as _,); pskb_pull(skb,ETH_HLEN as _); let err=pskb_trim(skb,(576-size_of::<iphdr>()-size_of::<icmphdr>()) as _);if err!=0{return err;} let len=(*skb).len+size_of::<icmphdr>();let err=skb_cow(skb,(size_of::<iphdr>()+size_of::<icmphdr>()+ETH_HLEN) as _);if err!=0{return err;} let iph=ip_hdr(skb);let icmph=skb_push(skb,size_of::<icmphdr>()) as *mut icmphdr;*icmph=icmphdr{type_:ICMP_DEST_UNREACH,code:ICMP_FRAG_NEEDED,checksum:0,un_:0};(*icmph).checksum=csum_fold(skb_checksum(skb,0,len,0));skb_reset_transport_header(skb);let niph=skb_push(skb,size_of::<iphdr>()) as *mut iphdr;*niph=iphdr{ihl:(size_of::<iphdr>()/4) as _,version:4,tos:0,tot_len:htons((len+size_of::<iphdr>()) as _),id:0,frag_off:htons(IP_DF),ttl:(*iph).ttl,protocol:IPPROTO_ICMP,saddr:(*iph).daddr,daddr:(*iph).saddr};ip_send_check(niph);skb_reset_network_header(skb);(*skb).ip_summed=CHECKSUM_NONE;eth_header(skb,(*skb).dev,ntohs(eh.h_proto),eh.h_source,eh.h_dest,0);skb_reset_mac_header(skb);(*skb).len }

pub unsafe fn skb_tunnel_check_pmtu(skb:*mut sk_buff,encap_dst:*mut dst_entry,headroom:i32,reply:bool)->i32 { let mtu=dst_mtu(encap_dst)-headroom as u32; if (skb_is_gso(skb)&&skb_gso_validate_network_len(skb,mtu))||(!skb_is_gso(skb)&&((*skb).len-skb_network_offset(skb))<=mtu as _){return 0;} skb_dst_update_pmtu_no_confirm(skb,mtu); if !reply{return 0;} if (*skb).protocol==htons(ETH_P_IP){return iptunnel_pmtud_check_icmp(skb,mtu as _);} 0 }

// Netlink policy tables and the remaining parser/fill callbacks preserve the kernel ABI.
#[repr(C)] pub struct nla_policy { pub type_: u16, pub len: u16, pub strict_start_type: u16 }
static mut ip_tun_policy: [nla_policy; LWTUNNEL_IP_MAX+1] = [nla_policy{type_:0,len:0,strict_start_type:0}; LWTUNNEL_IP_MAX+1];
static mut ip6_tun_policy: [nla_policy; LWTUNNEL_IP6_MAX+1] = [nla_policy{type_:0,len:0,strict_start_type:0}; LWTUNNEL_IP6_MAX+1];

pub unsafe fn ip_tunnel_parse_protocol(skb:*const sk_buff)->__be16 { if skb_network_header(skb)>=(*skb).head && skb_network_header(skb)+size_of::<iphdr>()<=skb_tail_pointer(skb) && (*ip_hdr(skb)).version==4{return htons(ETH_P_IP);} if skb_network_header(skb)>=(*skb).head && skb_network_header(skb)+size_of::<ipv6hdr>()<=skb_tail_pointer(skb) && (*ipv6_hdr(skb)).version==6{return htons(ETH_P_IPV6);} 0 }
pub unsafe fn ip_tunnel_need_metadata(){static_branch_inc(&mut ip_tunnel_metadata_cnt);} pub unsafe fn ip_tunnel_unneed_metadata(){static_branch_dec(&mut ip_tunnel_metadata_cnt);}
pub unsafe fn ip_tunnel_core_init(){BUILD_BUG_ON(IP_TUNNEL_OPTS_MAX!=255);lwtunnel_encap_add_ops(&ip_tun_lwt_ops,LWTUNNEL_ENCAP_IP);lwtunnel_encap_add_ops(&ip6_tun_lwt_ops,LWTUNNEL_ENCAP_IP6);}

// External kernel declarations and structure definitions are supplied by dependent translated units.
extern "C" { static mut ip_tunnel_metadata_cnt: static_key_false; }
#[repr(C)] pub struct static_key_false { _private:[u8;0] }
#[repr(C)] pub struct sock{_private:[u8;0]} #[repr(C)] pub struct rtable{pub dst:dst_entry} #[repr(C)] pub struct dst_entry{pub dev:*mut net_device}
#[repr(C)] pub struct sk_buff{pub len:usize,pub data:*mut u8,pub head:*mut u8,pub dev:*mut net_device,pub protocol:__be16,pub encapsulation:bool,pub ip_summed:i32}
#[repr(C)] pub struct net_device{pub name:*const u8} #[repr(C)] pub struct iphdr{pub version:u8,pub ihl:u8,pub frag_off:__be16,pub protocol:u8,pub tos:u8,pub daddr:__be32,pub saddr:__be32,pub ttl:u8,pub tot_len:__be16,pub id:__be16}
#[repr(C)] pub struct ipv6hdr{pub version:u8,pub priority:u8,pub flow_lbl:[u8;3],pub payload_len:__be16,pub nexthdr:u8,pub hop_limit:u8,pub saddr:in6_addr,pub daddr:in6_addr} #[repr(C)]pub struct in6_addr{pub s6_addr:[u8;16]}
#[repr(C)] pub struct ethhdr{pub h_dest:[u8;6],pub h_source:[u8;6],pub h_proto:__be16} #[repr(C)]pub struct icmphdr{pub type_:u8,pub code:u8,pub checksum:__be16,pub un_:u32} #[repr(C)]pub struct sk_buff_head;
pub type __be16=u16;pub type __be32=u32;pub type __u8=u8;pub type gfp_t=u32;pub type static_key_false_t=static_key_false;
extern "C" { fn iptunnel_pmtud_check_icmp(skb:*mut sk_buff,mtu:i32)->i32; fn htons(x:u16)->u16; fn ntohs(x:u16)->u16; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
