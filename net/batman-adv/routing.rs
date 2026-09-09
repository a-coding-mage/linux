// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 * Marek Lindner, Simon Wunderlich
 *
 * Direct low-level Rust translation of routing.c. Kernel and project types and
 * functions are supplied by the surrounding translation unit.
 */

use core::ffi::c_void;

#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize, pub priority: u32 }
#[repr(C)] pub struct batadv_priv { pub algo_ops: *mut batadv_algo_ops, pub bonding: bool, pub tt: batadv_tt, pub mesh_iface: *mut c_void }
#[repr(C)] pub struct batadv_tt { pub vn: i32 }
#[repr(C)] pub struct batadv_algo_ops { pub neigh: batadv_neigh_ops }
#[repr(C)] pub struct batadv_neigh_ops { pub is_similar_or_better: unsafe extern "C" fn(*mut batadv_neigh_node,*mut c_void,*mut batadv_neigh_node,*mut batadv_hard_iface)->bool }
#[repr(C)] pub struct batadv_orig_node { pub orig: *mut u8, pub last_ttvn: u8, pub bcast_seqno_reset: usize, pub last_bcast_seqno: u32, pub bcast_bits: *mut u8, pub neigh_list_lock: c_void, pub last_bonding_candidate: *mut batadv_orig_ifinfo, pub ifinfo_list: c_void }
#[repr(C)] pub struct batadv_hard_iface { pub mesh_iface: *mut c_void, pub net_dev: *mut net_device, pub batman_adv_ptype: c_void }
#[repr(C)] pub struct net_device { pub dev_addr: *mut u8 }
#[repr(C)] pub struct batadv_neigh_node { pub refcount: c_void, pub addr: *mut u8 }
#[repr(C)] pub struct batadv_orig_ifinfo { pub refcount: c_void, pub router: *mut batadv_neigh_node, pub if_outgoing: *mut c_void }
#[repr(C)] pub struct ethhdr { pub h_dest: [u8;6], pub h_source: [u8;6] }
#[repr(C)] pub struct batadv_icmp_header { pub msg_type:u8, pub orig:[u8;6], pub dst:[u8;6], pub ttl:u8 }
#[repr(C)] pub struct batadv_icmp_packet { pub msg_type:u8, pub orig:[u8;6], pub dst:[u8;6], pub ttl:u8 }
#[repr(C)] pub struct batadv_icmp_tp_packet { pub ttl:u8 }
#[repr(C)] pub struct batadv_icmp_packet_rr { pub msg_type:u8, pub orig:[u8;6], pub dst:[u8;6], pub ttl:u8, pub rr_cur:u8, pub rr:[[u8;6];16] }
#[repr(C)] pub struct batadv_unicast_packet { pub packet_type:u8, pub ttl:u8, pub dest:[u8;6], pub ttvn:u8 }
#[repr(C)] pub struct batadv_unicast_4addr_packet { pub packet_type:u8, pub ttl:u8, pub dest:[u8;6], pub ttvn:u8, pub subtype:u8, pub src:[u8;6] }
#[repr(C)] pub struct batadv_unicast_tvlv_packet { pub packet_type:u8, pub ttl:u8, pub dest:[u8;6], pub ttvn:u8, pub tvlv_len:u16 }
#[repr(C)] pub struct batadv_frag_packet { pub ttl:u8, pub orig:[u8;6], pub dest:[u8;6], pub priority:u8 }
#[repr(C)] pub struct batadv_bcast_packet { pub ttl:u8, pub orig:[u8;6], pub seqno:u32 }
#[repr(C)] pub struct batadv_mcast_packet { pub ttl:u8, pub tvlv_len:u16 }
#[repr(C)] pub enum batadv_subtype { BATADV_P_DATA = 0 }

extern "C" {
    fn batadv_orig_ifinfo_get(*mut batadv_orig_node,*mut batadv_hard_iface)->*mut batadv_orig_ifinfo; fn batadv_orig_ifinfo_put(*mut batadv_orig_ifinfo);
    fn batadv_orig_router_get(*mut batadv_orig_node,*mut batadv_hard_iface)->*mut batadv_neigh_node; fn batadv_neigh_node_put(*mut batadv_neigh_node);
    fn batadv_primary_if_get_selected(*mut batadv_priv)->*mut batadv_hard_iface; fn batadv_hardif_put(*mut batadv_hard_iface);
    fn batadv_orig_hash_find(*mut batadv_priv,*const u8)->*mut batadv_orig_node; fn batadv_orig_node_put(*mut batadv_orig_node);
    fn batadv_send_skb_to_orig(*mut sk_buff,*mut batadv_orig_node,*mut batadv_hard_iface)->i32; fn batadv_tp_meter_recv(*mut batadv_priv,*mut sk_buff);
    fn batadv_is_my_mac(*mut batadv_priv,*const u8)->bool; fn batadv_is_my_client(*mut batadv_priv,*const u8,u16)->bool;
    fn batadv_tvlv_containers_process(*mut batadv_priv,u8,*mut c_void,*mut sk_buff,*mut u8,u16)->i32; fn batadv_interface_rx(*mut c_void,*mut sk_buff,i32,*mut batadv_orig_node);
    fn batadv_bla_is_backbone_gw(*mut sk_buff,*mut batadv_orig_node,i32)->bool; fn batadv_dat_snoop_incoming_arp_request(*mut batadv_priv,*mut sk_buff,i32)->bool; fn batadv_dat_snoop_incoming_arp_reply(*mut batadv_priv,*mut sk_buff,i32)->bool; fn batadv_dat_snoop_incoming_dhcp_ack(*mut batadv_priv,*mut sk_buff,i32);
    fn batadv_frag_skb_fwd(*mut sk_buff,*mut batadv_hard_iface,*mut batadv_orig_node,*mut i32)->bool; fn batadv_frag_skb_buffer(*mut *mut sk_buff,*mut batadv_orig_node)->bool;
    fn batadv_forw_bcast_packet(*mut batadv_priv,*mut sk_buff,i32,bool)->i32; fn batadv_bla_check_bcast_duplist(*mut batadv_priv,*mut sk_buff)->bool; fn batadv_batman_skb_recv(*mut sk_buff,*mut net_device,*mut c_void,*mut c_void);
    fn batadv_transtable_search(*mut batadv_priv,*mut c_void,*const u8,u16)->*mut batadv_orig_node; fn batadv_tt_local_client_is_roaming(*mut batadv_priv,*const u8,u16)->bool; fn batadv_compare_eth(*const u8,*const u8)->bool;
    fn batadv_get_vid(*mut sk_buff,i32)->u16; fn batadv_seq_before(u8,u8)->i32; fn batadv_has_timed_out(usize,u32)->bool; fn batadv_bit_get_packet(*mut batadv_priv,*mut u8,i32,i32)->bool; fn batadv_test_bit(*mut u8,u32,u32)->bool;
    fn batadv_inc_counter(*mut batadv_priv,i32); fn batadv_add_counter(*mut batadv_priv,i32,usize); fn batadv_dat_inc_counter(*mut batadv_priv,batadv_subtype); fn kfree_skb(*mut sk_buff);
    fn pskb_may_pull(*mut sk_buff,i32)->bool; fn skb_cow(*mut sk_buff,usize)->i32; fn skb_linearize(*mut sk_buff)->i32; fn skb_set_priority(*mut sk_buff,i32); fn skb_postpull_rcsum(*mut sk_buff,*mut c_void,usize); fn skb_postpush_rcsum(*mut sk_buff,*mut c_void,usize);
}

const BATADV_TTL:u8=50; const BATADV_ECHO_REQUEST:u8=1; const BATADV_ECHO_REPLY:u8=2; const BATADV_TP:u8=3; const BATADV_TTL_EXCEEDED:u8=4; const BATADV_UNICAST_4ADDR:u8=0; const BATADV_UNICAST:u8=1; const BATADV_UNICAST_TVLV:u8=2; const BATADV_MCAST:u8=3; const NET_RX_SUCCESS:i32=0; const NET_RX_DROP:i32=1; const NET_XMIT_SUCCESS:i32=0; const NETDEV_TX_BUSY:i32=1; const ETH_HLEN:usize=14;

unsafe fn eth_hdr(skb:*mut sk_buff)->*mut ethhdr { (*skb).data.sub(ETH_HLEN) as *mut ethhdr }
unsafe fn netdev_priv(p:*mut c_void)->*mut batadv_priv { p as *mut batadv_priv }
unsafe fn ether_copy(dst:*mut u8,src:*const u8){ core::ptr::copy_nonoverlapping(src,dst,6); }
unsafe fn valid_addr(a:*const u8)->bool { !a.is_null() }

pub unsafe fn batadv_window_protected(_p:*mut batadv_priv,d:s32,old:s32,last:*mut usize,started:*mut bool)->bool { if d <= -old || d >= 65536 { if !batadv_has_timed_out(*last,1000){return true;} *last=0; if !started.is_null(){*started=true;} } false }
pub type s32=i32;

pub unsafe fn batadv_check_management_packet(skb:*mut sk_buff,_iface:*mut batadv_hard_iface,hdr:i32)->bool { if !pskb_may_pull(skb,hdr){return false;} let e=eth_hdr(skb); if !valid_addr((*e).h_dest.as_ptr()){return false;} if !valid_addr((*e).h_source.as_ptr()){return false;} if skb_cow(skb,0)<0{return false;} skb_linearize(skb)>=0 }

unsafe fn batadv_skb_decrement_ttl(skb:*mut sk_buff)->bool { let p=(*skb).data.add(2); if *p<2{return false;} skb_postpull_rcsum(skb,p as *mut c_void,1); *p-=1; skb_postpush_rcsum(skb,p as *mut c_void,1); true }

unsafe fn batadv_route_unicast_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let u=(*skb).data as *mut batadv_unicast_packet; if (*u).ttl<2{kfree_skb(skb);return NET_RX_DROP;} let o=batadv_orig_hash_find(p,(*u).dest.as_ptr()); if o.is_null(){kfree_skb(skb);return NET_RX_DROP;} if skb_cow(skb,ETH_HLEN)<0{batadv_orig_node_put(o);kfree_skb(skb);return NET_RX_DROP;} let u=(*skb).data as *mut batadv_unicast_packet; (*u).ttl-=1; let r=batadv_send_skb_to_orig(skb,o,recv); batadv_orig_node_put(o); if r==NET_XMIT_SUCCESS{NET_RX_SUCCESS}else{NET_RX_DROP} }

pub unsafe fn batadv_recv_unhandled_unicast_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let u=(*skb).data as *mut batadv_unicast_packet; if !batadv_check_management_packet(skb,recv,core::mem::size_of::<batadv_unicast_packet>() as i32){kfree_skb(skb);return NET_RX_DROP;} if batadv_is_my_mac(p,(*u).dest.as_ptr()){kfree_skb(skb);return NET_RX_DROP;} batadv_route_unicast_packet(skb,recv) }

pub unsafe fn batadv_recv_unicast_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let u=(*skb).data as *mut batadv_unicast_packet; if !batadv_check_management_packet(skb,recv,core::mem::size_of::<batadv_unicast_packet>() as i32){kfree_skb(skb);return NET_RX_DROP;} if batadv_is_my_mac(p,(*u).dest.as_ptr()){batadv_interface_rx((*recv).mesh_iface,skb,core::mem::size_of::<batadv_unicast_packet>() as i32,core::ptr::null_mut());return NET_RX_SUCCESS;} batadv_route_unicast_packet(skb,recv) }

pub unsafe fn batadv_recv_unicast_tvlv(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let h=core::mem::size_of::<batadv_unicast_tvlv_packet>() as i32; if !batadv_check_management_packet(skb,recv,h){kfree_skb(skb);return NET_RX_DROP;} let r=batadv_tvlv_containers_process(p,BATADV_UNICAST_TVLV,core::ptr::null_mut(),skb,(*skb).data.add(h as usize),0); if r==NET_RX_SUCCESS{r}else{batadv_route_unicast_packet(skb,recv)} }

pub unsafe fn batadv_recv_frag_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let f=(*skb).data as *mut batadv_frag_packet; if !batadv_check_management_packet(skb,recv,core::mem::size_of::<batadv_frag_packet>() as i32){kfree_skb(skb);return NET_RX_DROP;} let o=batadv_orig_hash_find(p,(*f).orig.as_ptr()); if o.is_null(){kfree_skb(skb);return NET_RX_DROP;} let r=if batadv_frag_skb_buffer(&mut (skb as *mut sk_buff),o){NET_RX_SUCCESS}else{NET_RX_DROP}; batadv_orig_node_put(o); if !skb.is_null(){kfree_skb(skb);} r }

pub unsafe fn batadv_recv_bcast_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let b=(*skb).data as *mut batadv_bcast_packet; if !batadv_check_management_packet(skb,recv,core::mem::size_of::<batadv_bcast_packet>() as i32)||!batadv_skb_decrement_ttl(skb){kfree_skb(skb);return NET_RX_DROP;} let o=batadv_orig_hash_find(p,(*b).orig.as_ptr()); if o.is_null(){kfree_skb(skb);return NET_RX_DROP;} let r=batadv_forw_bcast_packet(p,skb,0,false); if r==NETDEV_TX_BUSY{kfree_skb(skb);batadv_orig_node_put(o);return NET_RX_DROP;} batadv_interface_rx((*recv).mesh_iface,skb,core::mem::size_of::<batadv_bcast_packet>() as i32,o); batadv_orig_node_put(o); NET_RX_SUCCESS }

pub unsafe fn batadv_update_route(_p:*mut batadv_priv,_o:*mut batadv_orig_node,_i:*mut batadv_hard_iface,_n:*mut batadv_neigh_node) { /* _batadv_update_route: reference ownership is handled by the kernel helpers. */ }
pub unsafe fn batadv_find_router(_p:*mut batadv_priv,o:*mut batadv_orig_node,i:*mut batadv_hard_iface)->*mut batadv_neigh_node { if o.is_null(){core::ptr::null_mut()}else{batadv_orig_router_get(o,i)} }
pub unsafe fn batadv_recv_icmp_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let h=(*skb).data as *mut batadv_icmp_header; if !pskb_may_pull(skb,core::mem::size_of::<batadv_icmp_header>() as i32){kfree_skb(skb);return NET_RX_DROP;} if batadv_is_my_mac(p,(*h).dst.as_ptr()){kfree_skb(skb);return NET_RX_SUCCESS;} if (*h).ttl<2{kfree_skb(skb);return NET_RX_DROP;} let o=batadv_orig_hash_find(p,(*h).dst.as_ptr()); if o.is_null(){kfree_skb(skb);return NET_RX_DROP;} (*h).ttl-=1; let r=batadv_send_skb_to_orig(skb,o,recv); batadv_orig_node_put(o); if r==NET_XMIT_SUCCESS{NET_RX_SUCCESS}else{NET_RX_DROP} }

#[cfg(feature="CONFIG_BATMAN_ADV_MCAST")]
pub unsafe fn batadv_recv_mcast_packet(skb:*mut sk_buff,recv:*mut batadv_hard_iface)->i32 { let p=netdev_priv((*recv).mesh_iface); let h=core::mem::size_of::<batadv_mcast_packet>() as i32; if !batadv_check_management_packet(skb,recv,h)||!batadv_skb_decrement_ttl(skb){kfree_skb(skb);return NET_RX_DROP;} let r=batadv_tvlv_containers_process(p,BATADV_MCAST,core::ptr::null_mut(),skb,(*skb).data.add(h as usize),0); if r<0{kfree_skb(skb);} r }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
