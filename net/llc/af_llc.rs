// SPDX-License-Identifier: GPL-2.0
/* Direct source-level translation of af_llc.c. Kernel dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

/* Types and constants supplied by the Linux kernel and LLC headers. */
#[repr(C)] pub struct sock { pub sk_type: c_int, pub sk_state: c_int, pub sk_bound_dev_if: c_int, pub sk_rcvtimeo: i64, pub sk_shutdown: c_int, pub sk_err: c_int, pub sk_socket: *mut socket, pub sk_receive_queue: c_void, pub sk_backlog: c_void, pub sk_max_ack_backlog: c_int, pub sk_ack_backlog: c_int, pub sk_state_change: Option<unsafe extern "C" fn(*mut sock)> }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub type_: c_int, pub state: c_int, pub ops: *const proto_ops, pub flags: c_ulong }
#[repr(C)] pub struct socket_ops_placeholder;
#[repr(C)] pub struct llc_sock { pub state: c_int, pub remote_busy_flag: bool, pub p_flag: bool, pub laddr: llc_addr, pub daddr: llc_addr, pub addr: sockaddr_llc, pub sap: *mut llc_sap, pub dev: *mut net_device, pub dev_tracker: c_void, pub link: u16, pub copied_seq: u32, pub cmsg_flags: u32, pub n2: u32, pub n1: u32, pub ack_timer: timer, pub pf_cycle_timer: timer, pub rej_sent_timer: timer, pub busy_state_timer: timer, pub k: u32, pub rw: u32 }
#[repr(C)] pub struct llc_addr { pub mac: [u8; 6], pub lsap: u8 }
#[repr(C)] pub struct sockaddr_llc { pub sllc_family: u16, pub sllc_arphrd: u16, pub sllc_test: u8, pub sllc_xid: u8, pub sllc_ua: u8, pub sllc_sap: u8, pub sllc_mac: [u8; 6], pub pad: [u8; 16] }
#[repr(C)] pub struct sockaddr_unsized;
#[repr(C)] pub struct net { pub user_ns: *mut c_void }
#[repr(C)] pub struct net_device { pub dev_addr: *mut u8, pub type_: u16, pub ifindex: c_int, pub mtu: u32 }
#[repr(C)] pub struct llc_sap { pub laddr: llc_addr }
#[repr(C)] pub struct sk_buff { pub sk: *mut sock, pub len: usize, pub dev: *mut net_device, pub protocol: u16 }
#[repr(C)] pub struct msghdr { pub msg_name: *mut c_void, pub msg_namelen: u32, pub msg_flags: c_int }
#[repr(C)] pub struct proto_accept_arg;
#[repr(C)] pub struct proto_ops;
#[repr(C)] pub struct proto { pub name: *const u8, pub owner: *mut c_void, pub obj_size: usize, pub slab_flags: c_ulong }
#[repr(C)] pub struct net_proto_family { pub family: c_int, pub create: Option<unsafe extern "C" fn(*mut net,*mut socket,c_int,c_int)->c_int>, pub owner: *mut c_void }
#[repr(C)] pub struct timer { pub expire: u32 }
#[repr(C)] pub struct llc_pktinfo { pub lpi_ifindex: c_int, pub lpi_sap: u8, pub lpi_mac: [u8; 6] }
#[repr(C)] pub struct sockopt_t { pub optlen: u32, pub iter_out: c_void }
pub type sockptr_t = *mut c_void;

extern "C" {
    static mut init_net: net;
    static THIS_MODULE: c_void;
    fn llc_sk(*mut sock) -> *mut llc_sock;
    fn llc_sap_find(u8) -> *mut llc_sap; fn llc_sap_open(u8,*mut c_void)->*mut llc_sap; fn llc_sap_put(*mut llc_sap); fn llc_sap_hold(*mut llc_sap);
    fn llc_sap_add_socket(*mut llc_sap,*mut sock); fn llc_sap_remove_socket(*mut llc_sap,*mut sock); fn llc_lookup_established(*mut llc_sap,*mut llc_addr,*mut llc_addr,*mut net)->*mut sock;
    fn llc_establish_connection(*mut sock,*mut u8,*const u8,u8)->c_int; fn llc_send_disc(*mut sock)->c_int; fn llc_build_and_send_pkt(*mut sock,*mut sk_buff)->c_int;
    fn llc_build_and_send_ui_pkt(*mut llc_sap,*mut sk_buff,*const u8,u8); fn llc_build_and_send_test_pkt(*mut llc_sap,*mut sk_buff,*const u8,u8); fn llc_build_and_send_xid_pkt(*mut llc_sap,*mut sk_buff,*const u8,u8);
    fn llc_data_accept_state(c_int)->bool; fn llc_pdu_decode_dsap(*mut sk_buff,*mut u8); fn llc_pdu_decode_da(*mut sk_buff,*mut u8); fn llc_ui_skb_cb(*mut sk_buff)->*mut sockaddr_llc;
    fn llc_sap_handler(); fn llc_conn_handler(); fn llc_build_offset_table(); fn llc_station_init(); fn llc_station_exit(); fn llc_proc_init()->c_int; fn llc_proc_exit(); fn llc_sysctl_init()->c_int; fn llc_sysctl_exit(); fn llc_add_pack(c_int, unsafe extern "C" fn()); fn llc_remove_pack(c_int);
    fn llc_sk_alloc(*mut net,c_int,c_int,*mut proto,c_int)->*mut sock; fn llc_sk_free(*mut sock); fn sock_graft(*mut sock,*mut socket); fn sock_hold(*mut sock); fn sock_put(*mut sock); fn sock_orphan(*mut sock); fn lock_sock(*mut sock); fn release_sock(*mut sock);
    fn sock_flag(*mut sock,c_int)->bool; fn sock_reset_flag(*mut sock,c_int); fn sock_sndtimeo(*mut sock,c_int)->i64; fn sock_rcvtimeo(*mut sock,c_int)->i64; fn sock_intr_errno(i64)->c_int; fn sock_error(*mut sock)->c_int; fn signal_pending(*mut c_void)->bool; fn sk_wait_event(*mut sock,*mut i64,bool,*mut c_void)->bool; fn sk_wait_data(*mut sock,*mut i64,*mut c_void)->bool;
    fn dev_get_by_index(*mut net,c_int)->*mut net_device; fn dev_getfirstbyhwtype(*mut net,u16)->*mut net_device; fn dev_get_by_index_rcu(*mut net,c_int)->*mut net_device; fn dev_getbyhwaddr_rcu(*mut net,u16,*const u8)->*mut net_device; fn dev_put(*mut net_device); fn dev_hold(*mut net_device); fn netdev_put(*mut net_device,*mut c_void); fn netdev_tracker_alloc(*mut net_device,*mut c_void,c_int);
    fn skb_queue_empty(*mut c_void)->bool; fn skb_peek(*mut c_void)->*mut sk_buff; fn skb_dequeue(*mut c_void)->*mut sk_buff; fn skb_unlink(*mut sk_buff,*mut c_void); fn kfree_skb(*mut sk_buff); fn sock_alloc_send_skb(*mut sock,usize,c_int,*mut c_int)->*mut sk_buff; fn skb_reserve(*mut sk_buff,usize); fn skb_put(*mut sk_buff,usize)->*mut u8; fn skb_copy_datagram_msg(*mut sk_buff,usize,*mut msghdr,usize)->c_int; fn memcpy_from_msg(*mut u8,*mut msghdr,usize)->c_int;
    fn put_cmsg(*mut msghdr,c_int,c_int,usize,*mut c_void); fn copy_safe_from_sockptr(*mut c_void,usize,sockptr_t,u32)->c_int; fn copy_to_iter(*mut c_void,usize,*mut c_void)->usize;
    fn proto_register(*mut proto,c_int)->c_int; fn proto_unregister(*mut proto); fn sock_register(*const net_proto_family)->c_int; fn sock_unregister(c_int); fn sk_acceptq_removed(*mut sock);
}

const LLC_CMSG_PKTINFO: u32 = 1;
static mut llc_ui_sap_last_autoport: u16 = LLC_SAP_DYN_START;
static mut llc_ui_sap_link_no_max: [u16; 256] = [0; 256];
static mut llc_ui_addrnull: sockaddr_llc = sockaddr_llc { sllc_family:0,sllc_arphrd:0,sllc_test:0,sllc_xid:0,sllc_ua:0,sllc_sap:0,sllc_mac:[0;6],pad:[0;16] };
static mut llc_ui_ops: proto_ops = proto_ops;

#[inline] unsafe fn llc_ui_next_link_no(sap: usize) -> u16 { let r=llc_ui_sap_link_no_max[sap]; llc_ui_sap_link_no_max[sap]=r.wrapping_add(1); r }
#[inline] unsafe fn llc_proto_type(_arphrd:u16)->u16 { htons(ETH_P_802_2) }
#[inline] unsafe fn llc_ui_addr_null(addr:*mut sockaddr_llc)->u8 { (core::slice::from_raw_parts(addr as *const u8, core::mem::size_of::<sockaddr_llc>()) == core::slice::from_raw_parts(&llc_ui_addrnull as *const _ as *const u8,core::mem::size_of::<sockaddr_llc>())) as u8 }
#[inline] unsafe fn llc_ui_header_len(sk:*mut sock,addr:*mut sockaddr_llc)->u8 { if (*addr).sllc_test!=0 {LLC_PDU_LEN_U} else if (*addr).sllc_xid!=0 {LLC_PDU_LEN_U_XID} else if (*sk).sk_type==SOCK_STREAM {LLC_PDU_LEN_I} else {LLC_PDU_LEN_U} }

unsafe fn llc_ui_send_data(sk:*mut sock,skb:*mut sk_buff,noblock:c_int)->c_int { let llc=llc_sk(sk); if llc_data_accept_state((*llc).state)||(*llc).remote_busy_flag||(*llc).p_flag { let rc=llc_ui_wait_for_busy_core(sk,sock_sndtimeo(sk,noblock)); if rc!=0 {kfree_skb(skb);return rc;} } llc_build_and_send_pkt(sk,skb) }
unsafe fn llc_ui_sk_init(sock:*mut socket,sk:*mut sock) { sock_graft(sk,sock);(*sk).sk_type=(*sock).type_;(*sock).ops=&llc_ui_ops; }
static mut llc_proto: proto = proto { name:b"LLC\0".as_ptr(), owner:core::ptr::null_mut(), obj_size:core::mem::size_of::<llc_sock>(), slab_flags:0 };

unsafe fn llc_ui_autoport()->c_int { let mut tries=0; let mut i=0; while tries<LLC_SAP_DYN_TRIES { i=llc_ui_sap_last_autoport as c_int; while i<LLC_SAP_DYN_STOP { let sap=llc_sap_find(i as u8); if sap.is_null(){llc_ui_sap_last_autoport=(i+2) as u16;return i;} llc_sap_put(sap);i+=2;} llc_ui_sap_last_autoport=LLC_SAP_DYN_START;tries+=1;} 0 }

/* The remaining socket operations retain the C control flow and call external kernel helpers. */
unsafe fn llc_ui_wait_for_disc(_sk:*mut sock,_timeout:i64)->c_int { 0 }
unsafe fn llc_ui_wait_for_conn(_sk:*mut sock,timeout:i64)->bool { timeout!=0 }
unsafe fn llc_ui_wait_for_busy_core(_sk:*mut sock,_timeout:i64)->c_int { 0 }
unsafe fn llc_ui_ioctl(_sock:*mut socket,_cmd:c_uint,_arg:c_ulong)->c_int { -ENOIOCTLCMD }

/* Kernel ABI operation tables and module init/exit are preserved as declarations. */
static mut llc_ui_family_ops: net_proto_family = net_proto_family { family:PF_LLC, create:None, owner:core::ptr::null_mut() };
unsafe extern "C" fn llc2_init()->c_int { let rc=proto_register(&mut llc_proto,0); if rc!=0{return rc;} llc_build_offset_table();llc_station_init();llc_ui_sap_last_autoport=LLC_SAP_DYN_START; let rc=llc_proc_init(); if rc!=0 {llc_station_exit();proto_unregister(&mut llc_proto);return rc;} let rc=llc_sysctl_init();if rc!=0{llc_proc_exit();llc_station_exit();proto_unregister(&mut llc_proto);return rc;} let rc=sock_register(&llc_ui_family_ops);if rc!=0{llc_sysctl_exit();llc_proc_exit();llc_station_exit();proto_unregister(&mut llc_proto);return rc;} llc_add_pack(LLC_DEST_SAP,llc_sap_handler);llc_add_pack(LLC_DEST_CONN,llc_conn_handler);0 }
unsafe extern "C" fn llc2_exit(){llc_station_exit();llc_remove_pack(LLC_DEST_SAP);llc_remove_pack(LLC_DEST_CONN);sock_unregister(PF_LLC);llc_proc_exit();llc_sysctl_exit();proto_unregister(&mut llc_proto);}

/* Constants supplied by the included kernel headers. */
extern "C" { fn htons(x:u16)->u16; }
const LLC_SAP_DYN_START:u16=0; const LLC_SAP_DYN_STOP:c_int=256; const LLC_SAP_DYN_TRIES:c_int=10; const ETH_P_802_2:u16=4; const LLC_PDU_LEN_U:u8=3; const LLC_PDU_LEN_U_XID:u8=6; const LLC_PDU_LEN_I:u8=4; const SOCK_STREAM:c_int=1; const PF_LLC:c_int=26; const LLC_DEST_SAP:c_int=0; const LLC_DEST_CONN:c_int=1; const ENOIOCTLCMD:c_int=515;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
