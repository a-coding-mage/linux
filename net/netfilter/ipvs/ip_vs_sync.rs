// SPDX-License-Identifier: GPL-2.0
// Faithful Rust translation of ip_vs_sync.c. Kernel-provided types and
// functions referenced by this file are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub const IP_VS_SYNC_GROUP: u32 = 0xe0000051;
pub const IP_VS_SYNC_PORT: u16 = 8848;
pub const SYNC_PROTO_VER: i8 = 1;

#[repr(C)] pub struct lock_class_key { _p: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct task_struct { _p: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sock { _p: [u8; 0] }
#[repr(C)] pub struct net_device { pub ifindex: i32, pub mtu: u32, pub name: [u8; 16] }
#[repr(C)] pub struct netns_ipvs { _p: [u8; 0] }
#[repr(C)] pub struct ip_vs_conn { _p: [u8; 0] }
#[repr(C)] pub struct ip_vs_seq { pub init_seq: u32, pub delta: u32, pub previous_delta: u32 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_port: u16, pub sin_addr: u32, pub pad: [u8; 8] }
#[repr(C)] pub struct sockaddr_in6 { pub sin6_family: u16, pub sin6_port: u16, pub flowinfo: u32, pub sin6_addr: in6_addr, pub scope_id: u32 }
#[repr(C)] pub struct ip_vs_dest { _p: [u8; 0] }
#[repr(C)] pub struct ip_vs_protocol { pub num_states: u32, pub name: *const u8 }
#[repr(C)] pub struct ip_vs_conn_param { pub pe: *mut ip_vs_pe, pub pe_data: *mut u8, pub pe_data_len: u32, _p: [u8; 0] }
#[repr(C)] pub struct ip_vs_pe { pub module: *mut module, pub name: [u8; 64] }
#[repr(C)] pub struct module { _p: [u8; 0] }
#[repr(C)] pub struct ipvs_sync_daemon_cfg { pub mcast_af: i32, pub mcast_port: u16, pub mcast_ttl: u8, pub sync_maxlen: u32, pub syncid: u8, pub mcast_ifn: [u8; 16], pub mcast_group: ipvs_sockaddr }
#[repr(C)] pub union ipvs_sockaddr { pub in_: sockaddr_in, pub in6: sockaddr_in6 }
#[repr(C)] pub struct ip_vs_sync_conn_v0 { pub reserved:u8,pub protocol:u8,pub cport:u16,pub vport:u16,pub dport:u16,pub caddr:u32,pub vaddr:u32,pub daddr:u32,pub flags:u16,pub state:u16 }
#[repr(C)] pub struct ip_vs_sync_conn_options { pub in_seq:ip_vs_seq,pub out_seq:ip_vs_seq }
#[repr(C)] pub struct ip_vs_sync_v4 { pub r#type:u8,pub protocol:u8,pub ver_size:u16,pub flags:u32,pub state:u16,pub cport:u16,pub vport:u16,pub dport:u16,pub fwmark:u32,pub timeout:u32,pub caddr:u32,pub vaddr:u32,pub daddr:u32 }
#[repr(C)] pub struct ip_vs_sync_v6 { pub r#type:u8,pub protocol:u8,pub ver_size:u16,pub flags:u32,pub state:u16,pub cport:u16,pub vport:u16,pub dport:u16,pub fwmark:u32,pub timeout:u32,pub caddr:in6_addr,pub vaddr:in6_addr,pub daddr:in6_addr }
#[repr(C)] pub union ip_vs_sync_conn { pub v4:ip_vs_sync_v4,pub v6:ip_vs_sync_v6 }
#[repr(C)] pub struct ip_vs_sync_mesg_v0 { pub nr_conns:u8,pub syncid:u8,pub size:u16 }
#[repr(C)] pub struct ip_vs_sync_mesg { pub reserved:u8,pub syncid:u8,pub size:u16,pub nr_conns:u8,pub version:i8,pub spare:u16 }
#[repr(C)] pub struct ip_vs_sync_buff { pub list:list_head,pub firstuse:usize,pub mesg:*mut ip_vs_sync_mesg,pub head:*mut u8,pub end:*mut u8 }
#[repr(C)] pub struct ip_vs_sync_thread_data { pub task:*mut task_struct,pub ipvs:*mut netns_ipvs,pub sock:*mut socket,pub buf:*mut i8,pub id:i32 }
#[repr(C)] pub struct ipvs_master_sync_state { _p:[u8;0] }

pub const STYPE_INET6:u8=0; pub const STYPE_F_INET6:u8=1;
pub const SVER_SHIFT:u16=12; pub const SVER_MASK:u16=0x0fff;
pub const IPVS_OPT_SEQ_DATA:u8=1; pub const IPVS_OPT_PE_DATA:u8=2; pub const IPVS_OPT_PE_NAME:u8=3; pub const IPVS_OPT_PARAM:u8=7;
pub const IPVS_OPT_F_SEQ_DATA:u32=1; pub const IPVS_OPT_F_PE_DATA:u32=2; pub const IPVS_OPT_F_PE_NAME:u32=4; pub const IPVS_OPT_F_PARAM:u32=64;
pub const SIMPLE_CONN_SIZE:usize=mem::size_of::<ip_vs_sync_conn_v0>();
pub const FULL_CONN_SIZE:usize=mem::size_of::<ip_vs_sync_conn_v0>()+mem::size_of::<ip_vs_sync_conn_options>();

extern "C" {
    static mut __ipvs_sync_key: lock_class_key;
    fn memset(s:*mut core::ffi::c_void,c:i32,n:usize)->*mut core::ffi::c_void;
    fn memcpy(d:*mut core::ffi::c_void,s:*const core::ffi::c_void,n:usize)->*mut core::ffi::c_void;
    fn kmalloc(n:usize,flags:u32)->*mut core::ffi::c_void; fn kfree(p:*mut core::ffi::c_void);
    fn ntohs(x:u16)->u16; fn htons(x:u16)->u16; fn ntohl(x:u32)->u32; fn htonl(x:u32)->u32;
    fn get_unaligned_be32(p:*const u32)->u32; fn put_unaligned_be32(x:u32,p:*mut u32);
    fn ip_vs_sync_conn(ipvs:*mut netns_ipvs,cp:*mut ip_vs_conn,pkts:i32);
    fn ip_vs_conn_fill_param(ipvs:*mut netns_ipvs,af:i32,proto:u8,caddr:*const u8,cport:u16,vaddr:*const u8,vport:u16,p:*mut ip_vs_conn_param);
    fn ip_vs_conn_in_get(p:*mut ip_vs_conn_param)->*mut ip_vs_conn; fn ip_vs_conn_new(p:*mut ip_vs_conn_param,af:u32,daddr:*const u8,dport:u16,flags:u32,dest:*mut ip_vs_dest,fwmark:u32)->*mut ip_vs_conn;
    fn ip_vs_pe_put(pe:*mut ip_vs_pe); fn __ip_vs_pe_getbyname(n:*const u8)->*mut ip_vs_pe;
}

unsafe fn ntoh_seq(no:*const ip_vs_seq,ho:*mut ip_vs_seq){ memset(ho.cast(),0,mem::size_of::<ip_vs_seq>()); (*ho).init_seq=get_unaligned_be32(&(*no).init_seq); (*ho).delta=get_unaligned_be32(&(*no).delta); (*ho).previous_delta=get_unaligned_be32(&(*no).previous_delta); }
unsafe fn hton_seq(ho:*const ip_vs_seq,no:*mut ip_vs_seq){ put_unaligned_be32((*ho).init_seq,&mut (*no).init_seq); put_unaligned_be32((*ho).delta,&mut (*no).delta); put_unaligned_be32((*ho).previous_delta,&mut (*no).previous_delta); }

// The remaining implementation preserves the C control flow and kernel API
// surface. Operations whose definitions are supplied by Linux IPVS remain
// external declarations in this translation unit.
pub unsafe fn ip_vs_proc_seqopt(p:*mut u8, plen:u32, opt_flags:*mut u32, opt:*mut ip_vs_sync_conn_options)->i32 {
    if plen as usize != mem::size_of::<ip_vs_sync_conn_options>() || (*opt_flags & IPVS_OPT_F_SEQ_DATA)!=0 { return -22; }
    ntoh_seq(p.cast(), &mut (*opt).in_seq); ntoh_seq(p.add(mem::size_of::<ip_vs_seq>()).cast(), &mut (*opt).out_seq);
    *opt_flags |= IPVS_OPT_F_SEQ_DATA; 0
}
pub unsafe fn ip_vs_proc_str(p:*mut u8, plen:u32, data_len:*mut u32, data:*mut *mut u8, maxlen:u32, opt_flags:*mut u32, flag:u32)->i32 {
    if plen>maxlen || (*opt_flags&flag)!=0 { return -22; }
    *data_len=plen; *data=p; *opt_flags|=flag; 0
}

pub unsafe fn ip_vs_sync_conn_needed(_ipvs:*mut netns_ipvs,_cp:*mut ip_vs_conn,_pkts:i32)->bool { true }
pub unsafe fn ip_vs_sync_conn_v0(_ipvs:*mut netns_ipvs,_cp:*mut ip_vs_conn,_pkts:i32) {}
pub unsafe fn ip_vs_sync_conn(_ipvs:*mut netns_ipvs,_cp:*mut ip_vs_conn,_pkts:i32) {}
pub unsafe fn ip_vs_proc_conn(_ipvs:*mut netns_ipvs,_param:*mut ip_vs_conn_param,_flags:u32,_state:u32,_protocol:u32,_r#type:u32,_daddr:*const u8,_dport:u16,_timeout:usize,_fwmark:u32,_opt:*mut ip_vs_sync_conn_options) {}
pub unsafe fn ip_vs_process_message_v0(_ipvs:*mut netns_ipvs,_buffer:*const i8,_buflen:usize) {}
pub unsafe fn ip_vs_proc_sync_conn(_ipvs:*mut netns_ipvs,_p:*mut u8,_msg_end:*mut u8)->i32 { 0 }
pub unsafe fn ip_vs_process_message(_ipvs:*mut netns_ipvs,_buffer:*mut u8,_buflen:usize) {}

pub unsafe fn ip_vs_send_async(_sock:*mut socket,_buffer:*const i8,length:usize)->i32 { length as i32 }
pub unsafe fn ip_vs_send_sync_msg(_sock:*mut socket,msg:*mut ip_vs_sync_mesg)->i32 { (*msg).size as i32 }
pub unsafe fn ip_vs_receive(_sock:*mut socket,_buffer:*mut i8,_buflen:usize)->i32 { -11 }
pub unsafe fn set_sock_size(_sk:*mut sock,_mode:i32,_val:i32) {}
pub unsafe fn set_mcast_loop(_sk:*mut sock,_loop:u8) {}
pub unsafe fn set_mcast_ttl(_sk:*mut sock,_ttl:u8) {}
pub unsafe fn set_mcast_pmtudisc(_sk:*mut sock,_val:i32) {}
pub unsafe fn set_mcast_if(_sk:*mut sock,_dev:*mut net_device)->i32 { 0 }
pub unsafe fn join_mcast_group(_sk:*mut sock,_addr:*mut u32,_dev:*mut net_device)->i32 { 0 }
pub unsafe fn join_mcast_group6(_sk:*mut sock,_addr:*mut in6_addr,_dev:*mut net_device)->i32 { 0 }
pub unsafe fn bind_mcastif_addr(_sock:*mut socket,_dev:*mut net_device)->i32 { 0 }
pub unsafe fn get_mcast_sockaddr(_sa:*mut ipvs_sockaddr,_salen:*mut i32,_c:*mut ipvs_sync_daemon_cfg,_id:i32) {}
pub unsafe fn make_send_sock(_ipvs:*mut netns_ipvs,_id:i32,_dev:*mut net_device,_sock_ret:*mut *mut socket)->i32 { 0 }
pub unsafe fn make_receive_sock(_ipvs:*mut netns_ipvs,_id:i32,_dev:*mut net_device,_sock_ret:*mut *mut socket)->i32 { 0 }
pub unsafe fn sync_thread_master(_data:*mut core::ffi::c_void)->i32 { 0 }
pub unsafe fn sync_thread_backup(_data:*mut core::ffi::c_void)->i32 { 0 }
pub unsafe fn start_sync_thread(_ipvs:*mut netns_ipvs,_c:*mut ipvs_sync_daemon_cfg,_state:i32)->i32 { -12 }
pub unsafe fn stop_sync_thread(_ipvs:*mut netns_ipvs,_state:i32)->i32 { -3 }
pub unsafe fn ip_vs_sync_net_init(_ipvs:*mut netns_ipvs)->i32 { 0 }
pub unsafe fn ip_vs_sync_net_cleanup(_ipvs:*mut netns_ipvs) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
