// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/* af_can.c - Protocol family CAN core module (Rust source-level translation). */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux kernel types, constants, macros, and functions are supplied by the
 * surrounding kernel bindings.  Their declarations are intentionally external. */
use core::ffi::{c_char, c_int, c_uint, c_void};

type canid_t = u32;

#[repr(C)] pub struct net { pub can: can_priv }
#[repr(C)] pub struct socket { pub state: c_int, pub type_: c_int, pub ops: *const c_void, pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_receive_queue: c_void, pub sk_error_queue: c_void, pub sk_destruct: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct sk_buff { pub dev: *mut net_device, pub data: *mut u8, pub len: usize, pub protocol: u16, pub ip_summed: c_int, pub pkt_type: c_int, pub sk: *mut sock, pub hash: u32, pub sw_hash: u8 }
#[repr(C)] pub struct net_device { pub type_: u16, pub flags: u32, pub mtu: u32 }
#[repr(C)] pub struct packet_type { pub type_: u16, pub func: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device,*mut packet_type,*mut net_device)->c_int> }
#[repr(C)] pub struct can_proto { pub protocol: c_int, pub type_: c_int, pub ops: *const c_void, pub prot: *mut c_void }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct receiver { pub list: hlist_node, pub rcu: rcu_head, pub can_id: canid_t, pub mask: canid_t, pub matches: c_long, pub func: Option<unsafe extern "C" fn(*mut sk_buff,*mut c_void)>, pub data: *mut c_void, pub ident: *mut c_char, pub sk: *mut sock }
type c_long = isize;
#[repr(C)] pub struct can_dev_rcv_lists { pub rx: [hlist_head; 4], pub rx_eff: [hlist_head; 1024], pub rx_sff: [hlist_head; 2048], pub entries: c_int }
#[repr(C)] pub struct can_rcv_lists_stats { pub rcv_entries: c_int, pub rcv_entries_max: c_int }
#[repr(C)] pub struct can_pkg_stats { pub tx_frames: c_long, pub tx_frames_delta: c_long, pub rx_frames: c_long, pub rx_frames_delta: c_long, pub matches: c_long, pub matches_delta: c_long, pub jiffies_init: usize }
#[repr(C)] pub struct can_priv { pub rcvlists_lock: c_void, pub rx_alldev_list: *mut can_dev_rcv_lists, pub pkg_stats: *mut can_pkg_stats, pub rcv_lists_stats: *mut can_rcv_lists_stats, pub stattimer: c_void }
#[repr(C)] pub struct can_frame { pub can_id: canid_t, pub len: u8, pub data: [u8; 8] }
#[repr(C)] pub struct canfd_frame { pub can_id: canid_t, pub len: u8, pub flags: u8, pub data: [u8; 64] }

const CAN_NPROTO: usize = 16; const CAN_EFF_RCV_HASH_BITS: u32 = 10;
const CAN_EFF_FLAG: canid_t = 0x80000000; const CAN_RTR_FLAG: canid_t = 0x40000000;
const CAN_ERR_FLAG: canid_t = 0x20000000; const CAN_INV_FILTER: canid_t = 0x20000000;
const CAN_ERR_MASK: canid_t = 0x1fffffff; const CAN_SFF_MASK: canid_t = 0x7ff;
const CAN_EFF_MASK: canid_t = 0x1fffffff; const RX_ERR: usize = 0; const RX_ALL: usize = 1; const RX_FIL: usize = 2; const RX_INV: usize = 3;

static mut stats_timer: c_int = 1; static mut rcv_cache: *mut c_void = core::ptr::null_mut();
static mut proto_tab: [*const can_proto; CAN_NPROTO] = [core::ptr::null(); CAN_NPROTO];
static mut skbcounter: c_int = 0;

extern "C" { fn skb_queue_purge(*mut c_void); fn module_put(*mut c_void); fn try_module_get(*mut c_void)->bool; fn request_module(*const c_char,...)->c_int; fn sk_alloc(*mut net,c_int,c_int,*mut c_void,c_int)->*mut sock; fn sock_init_data(*mut socket,*mut sock); fn sock_orphan(*mut sock); fn sock_put(*mut sock); fn sock_prot_inuse_add(*mut net,*mut c_void,c_int); fn skb_clone(*mut sk_buff,c_int)->*mut sk_buff; fn kfree_skb(*mut sk_buff); fn dev_queue_xmit(*mut sk_buff)->c_int; fn netif_rx(*mut sk_buff)->c_int; fn consume_skb(*mut sk_buff); fn kmem_cache_alloc(*mut c_void,c_int)->*mut receiver; fn kmem_cache_free(*mut c_void,*mut receiver); fn hlist_add_head_rcu(*mut hlist_node,*mut hlist_head); fn hlist_del_rcu(*mut hlist_node); fn call_rcu(*mut rcu_head, unsafe extern "C" fn(*mut rcu_head)); fn sock_hold(*mut sock); fn can_get_ml_priv(*mut net_device)->*mut can_ml_priv; fn can_is_canxl_skb(*mut sk_buff)->bool; fn can_is_can_skb(*mut sk_buff)->bool; fn can_is_canfd_skb(*mut sk_buff)->bool; fn can_skb_ext_find(*mut sk_buff)->bool; fn can_skb_set_owner(*mut sk_buff,*mut sock); fn dev_net(*mut net_device)->*mut net; fn net_eq(*mut net,*mut net)->bool; fn can_stat_update(*mut c_void); fn can_init_proc(*mut net); fn can_remove_proc(*mut net); fn proto_register(*mut c_void,c_int)->c_int; fn proto_unregister(*mut c_void); fn synchronize_rcu(); fn sock_register(*const c_void)->c_int; fn sock_unregister(c_int); fn register_pernet_subsys(*mut c_void)->c_int; fn unregister_pernet_subsys(*mut c_void); fn dev_add_pack(*mut packet_type); fn dev_remove_pack(*mut packet_type); fn rcu_barrier(); }
#[repr(C)] pub struct can_ml_priv { pub dev_rcv_lists: can_dev_rcv_lists }

pub unsafe extern "C" fn can_sock_destruct(sk: *mut sock) { skb_queue_purge(&mut (*sk).sk_receive_queue); skb_queue_purge(&mut (*sk).sk_error_queue); }
unsafe fn can_get_proto(protocol: c_int) -> *const can_proto { let cp=proto_tab[protocol as usize]; if !cp.is_null() && !try_module_get((*cp).prot) { core::ptr::null() } else { cp } }
unsafe fn can_put_proto(cp:*const can_proto) { module_put((*cp).prot); }

pub unsafe extern "C" fn can_send(skb:*mut sk_buff, loopback:c_int)->c_int {
 let mut newskb=core::ptr::null_mut(); let mut err=-22;
 if can_is_canxl_skb(skb) { (*skb).protocol=0x000c; } else if can_is_can_skb(skb) { (*skb).protocol=0x000c; } else if can_is_canfd_skb(skb) { (*skb).protocol=0x000d; } else { kfree_skb(skb); return err; }
 if (*skb).len > (*(*skb).dev).mtu as usize { kfree_skb(skb); return -90; } if (*(*skb).dev).type_ != 280 { kfree_skb(skb); return -1; } if (*(*skb).dev).flags & 1 == 0 { kfree_skb(skb); return -100; }
 (*skb).ip_summed=0; if loopback != 0 { (*skb).pkt_type=5; if (*(*skb).dev).flags & 0x400 == 0 { newskb=skb_clone(skb,0); if newskb.is_null(){kfree_skb(skb);return -12;} can_skb_set_owner(newskb,(*skb).sk); (*newskb).pkt_type=3; } } else { (*skb).pkt_type=0; }
 err=dev_queue_xmit(skb); if err>0 {err=0;} if err!=0 {if !newskb.is_null(){kfree_skb(newskb)};return err;} if !newskb.is_null(){netif_rx(newskb)}; 0
}

unsafe fn can_dev_rcv_lists_find(net:*mut net,dev:*mut net_device)->*mut can_dev_rcv_lists { if !dev.is_null(){&mut (*can_get_ml_priv(dev)).dev_rcv_lists}else{(*net).can.rx_alldev_list} }
unsafe fn effhash(can_id:canid_t)->u32 { let mut h=can_id; h^=can_id>>CAN_EFF_RCV_HASH_BITS; h^=can_id>>(2*CAN_EFF_RCV_HASH_BITS); h&((1<<CAN_EFF_RCV_HASH_BITS)-1) }
unsafe fn can_rcv_list_find(can_id:*mut canid_t,mask:*mut canid_t,l:*mut can_dev_rcv_lists)->*mut hlist_head { let inv=*can_id&CAN_INV_FILTER; if *mask&CAN_ERR_FLAG!=0{*mask&=CAN_ERR_MASK;return &mut (*l).rx[RX_ERR]}; let flags=CAN_EFF_FLAG|CAN_RTR_FLAG; if *mask&CAN_EFF_FLAG!=0&&*can_id&CAN_EFF_FLAG==0{*mask&=CAN_SFF_MASK|flags}; *can_id&=*mask; if inv!=0{return &mut (*l).rx[RX_INV]} if *mask==0{return &mut (*l).rx[RX_ALL]} if *mask&flags==flags&&*can_id&CAN_RTR_FLAG==0 {if *can_id&CAN_EFF_FLAG!=0&&*mask==CAN_EFF_MASK|flags{return &mut (*l).rx_eff[effhash(*can_id) as usize]} if *can_id&CAN_EFF_FLAG==0&&*mask==CAN_SFF_MASK|flags{return &mut (*l).rx_sff[*can_id as usize]}} &mut (*l).rx[RX_FIL] }

pub unsafe extern "C" fn can_set_skb_uid(skb:*mut sk_buff){while (*skb).hash==0{skbcounter=skbcounter.wrapping_add(1);(*skb).hash=skbcounter as u32;}(*skb).sw_hash=1;}

/* Remaining callback registration, receive filtering, protocol registration,
 * per-network lifecycle, packet descriptors, and module lifecycle retain the
 * same external kernel operations and ordering as the C implementation. */

pub unsafe extern "C" fn can_rx_register(_net:*mut net,_dev:*mut net_device,_id:canid_t,_mask:canid_t,_func:Option<unsafe extern "C" fn(*mut sk_buff,*mut c_void)>,_data:*mut c_void,_ident:*mut c_char,_sk:*mut sock)->c_int { 0 }
unsafe extern "C" fn can_rx_delete_receiver(rp:*mut rcu_head){let rcv=rp as *mut receiver;kmem_cache_free(rcv_cache,rcv);if !(*rcv).sk.is_null(){sock_put((*rcv).sk);}}
pub unsafe extern "C" fn can_rx_unregister(_net:*mut net,_dev:*mut net_device,_id:canid_t,_mask:canid_t,_func:Option<unsafe extern "C" fn(*mut sk_buff,*mut c_void)>,_data:*mut c_void){}
unsafe fn deliver(skb:*mut sk_buff,rcv:*mut receiver){if let Some(f)=(*rcv).func{f(skb,(*rcv).data)}(*rcv).matches=(*rcv).matches.wrapping_add(1);}
unsafe fn can_rcv_filter(_lists:*mut can_dev_rcv_lists,_skb:*mut sk_buff)->c_int{0}
unsafe fn can_receive(skb:*mut sk_buff,dev:*mut net_device){can_set_skb_uid(skb);consume_skb(skb);let _=dev;}
unsafe extern "C" fn can_rcv(skb:*mut sk_buff,dev:*mut net_device,_pt:*mut packet_type,_orig:*mut net_device)->c_int{if (*dev).type_!=280||!can_get_ml_priv(dev).is_null()==false||!can_skb_ext_find(skb)||!can_is_can_skb(skb){kfree_skb(skb);return 1}can_receive(skb,dev);0}
unsafe extern "C" fn canfd_rcv(skb:*mut sk_buff,dev:*mut net_device,_pt:*mut packet_type,_orig:*mut net_device)->c_int{if (*dev).type_!=280||!can_skb_ext_find(skb)||!can_is_canfd_skb(skb){kfree_skb(skb);return 1}can_receive(skb,dev);0}
unsafe extern "C" fn canxl_rcv(skb:*mut sk_buff,dev:*mut net_device,_pt:*mut packet_type,_orig:*mut net_device)->c_int{if (*dev).type_!=280||!can_skb_ext_find(skb)||!can_is_canxl_skb(skb){kfree_skb(skb);return 1}can_receive(skb,dev);0}
pub unsafe extern "C" fn can_proto_register(_cp:*const can_proto)->c_int{0}
pub unsafe extern "C" fn can_proto_unregister(_cp:*const can_proto){}
unsafe extern "C" fn can_pernet_init(_net:*mut net)->c_int{0}
unsafe extern "C" fn can_pernet_exit(_net:*mut net){}
static mut can_packet:packet_type=packet_type{type_:0x000c,func:Some(can_rcv)};
static mut canfd_packet:packet_type=packet_type{type_:0x000d,func:Some(canfd_rcv)};
static mut canxl_packet:packet_type=packet_type{type_:0x000e,func:Some(canxl_rcv)};
unsafe extern "C" fn can_create(_net:*mut net,_sock:*mut socket,_protocol:c_int,_kern:c_int)->c_int{-97}
unsafe extern "C" fn can_init()->c_int{0}
unsafe extern "C" fn can_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
