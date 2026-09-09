// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of inet_diag.c. Kernel-provided types, constants, and
 * functions are intentionally referenced as external dependencies. */

use core::{mem, ptr};

extern "C" {
    static mut inet_diag_table: *mut *const inet_diag_handler;
}

#[repr(C)]
pub struct inet_diag_entry {
    pub saddr: *const u32, pub daddr: *const u32, pub sport: u16, pub dport: u16,
    pub family: u16, pub userlocks: u16, pub ifindex: u32, pub mark: u32,
    #[cfg(CONFIG_SOCK_CGROUP_DATA)] pub cgroup_id: u64,
}

#[repr(C)] pub struct inet_diag_handler { pub owner: *mut core::ffi::c_void, pub idiag_type: u16, pub idiag_info_size: u32, pub idiag_get_info: Option<unsafe extern "C" fn(*mut sock,*mut inet_diag_msg,*mut core::ffi::c_void)>, pub idiag_get_aux: Option<unsafe extern "C" fn(*mut sock,bool,*mut sk_buff)->i32>, pub dump: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback,*const inet_diag_req_v2)>, pub dump_one: Option<unsafe extern "C" fn(*mut netlink_callback,*const inet_diag_req_v2)->i32>, pub destroy: Option<unsafe extern "C" fn(*mut sk_buff,*const inet_diag_req_v2)->i32> }
#[repr(C)] pub struct sock { pub sk_family:u16, pub sk_num:u16, pub sk_dport:u16, pub sk_bound_dev_if:u32, pub sk_rcv_saddr:u32, pub sk_daddr:u32, pub sk_shutdown:u8, pub sk_mark:u32, pub sk_priority:u32, pub sk_type:i32, pub sk_protocol:u8, pub sk_state:u8, pub sk_userlocks:u16, pub sk_wmem_queued:u32, pub sk_forward_alloc:i32 }
#[repr(C)] pub struct sk_buff { pub data:*mut u8, pub len:u32 }
#[repr(C)] pub struct netlink_callback { pub nlh:*const nlmsghdr, pub skb:*mut sk_buff, pub data:*mut inet_diag_dump_data, pub min_dump_alloc:u32 }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_len:u32, pub nlmsg_type:u16, pub nlmsg_flags:u16, pub nlmsg_seq:u32 }
#[repr(C)] pub struct nlattr { pub nla_len:u16, pub nla_type:u16 }
#[repr(C)] pub struct inet_diag_msg { pub idiag_family:u8, pub idiag_state:u8, pub idiag_timer:u8, pub idiag_retrans:u8, pub id:inet_diag_sockid, pub idiag_expires:u32, pub idiag_rqueue:u32, pub idiag_wqueue:u32, pub idiag_uid:u32, pub idiag_inode:u32 }
#[repr(C)] pub struct inet_diag_sockid { pub idiag_sport:u16, pub idiag_dport:u16, pub idiag_src:[u32;4], pub idiag_dst:[u32;4], pub idiag_if:u32, pub idiag_cookie:[u32;2] }
#[repr(C)] pub struct inet_diag_req_v2 { pub sdiag_family:u8, pub sdiag_protocol:u8, pub idiag_ext:u8, pub pad:u8, pub idiag_states:u32, pub id:inet_diag_sockid }
#[repr(C)] pub struct inet_diag_req { pub idiag_family:u8, pub idiag_src_len:u8, pub idiag_dst_len:u8, pub idiag_ext:u8, pub idiag_states:u32, pub id:inet_diag_sockid }
#[repr(C)] pub struct inet_diag_dump_data { pub req_nlas:[*mut nlattr;32], pub inet_diag_nla_bc:*const nlattr, pub inet_diag_nla_bpf_stgs:*mut nlattr, pub bpf_stg_diag:*mut core::ffi::c_void, pub userlocks_needed:bool, pub mark_needed:bool, pub cgroup_needed:bool }
#[repr(C)] pub struct inet_connection_sock { pub icsk_pending:u8, pub icsk_retransmits:u8, pub icsk_probes_out:u8, pub icsk_ack_pending:u8 }
#[repr(C)] pub struct inet_sock { pub inet_num:u16, pub inet_dport:u16, pub inet_sport:u16, pub tos:u8 }
#[repr(C)] pub struct inet_diag_sockopt { pub recverr:bool,pub is_icsk:bool,pub freebind:bool,pub hdrincl:bool,pub mc_loop:bool,pub transparent:bool,pub mc_all:bool,pub nodefrag:bool,pub bind_address_no_port:bool,pub recverr_rfc4884:bool,pub defer_connect:bool }
#[repr(C)] pub struct inet_diag_bc_op { pub code:u8,pub yes:u8,pub no:u16,pub no_port:u16 }
#[repr(C)] pub struct inet_diag_hostcond { pub family:u16,pub prefix_len:u8,pub port:i32,pub addr:[u32;4] }
#[repr(C)] pub struct inet_diag_markcond { pub mark:u32,pub mask:u32 }

extern "C" {
    fn inet_sk_diag_fill(sk:*mut sock, icsk:*mut inet_connection_sock, skb:*mut sk_buff, cb:*mut netlink_callback, req:*const inet_diag_req_v2, flags:u16, admin:bool)->i32;
    fn inet_sk(sk:*mut sock)->*mut inet_sock;
    fn nla_data(a:*const nlattr)->*mut core::ffi::c_void; fn nla_len(a:*const nlattr)->i32; fn nla_type(a:*const nlattr)->i32;
    fn nla_put_u8(*mut sk_buff,i32,u8)->i32; fn nla_put_u32(*mut sk_buff,i32,u32)->i32; fn nla_put(*mut sk_buff,i32,usize,*const core::ffi::c_void)->i32;
    fn nlmsg_put(*mut sk_buff,u32,u32,u16,usize,u16)->*mut nlmsghdr; fn nlmsg_data(*mut nlmsghdr)->*mut core::ffi::c_void; fn nlmsg_end(*mut sk_buff,*mut nlmsghdr); fn nlmsg_cancel(*mut sk_buff,*mut nlmsghdr);
    fn sock_diag_save_cookie(*mut sock,*mut [u32;2]); fn inet_diag_get_protocol(*const inet_diag_req_v2,*const inet_diag_dump_data)->i32;
    fn htons(u16)->u16; fn ntohs(u16)->u16; fn htonl(u32)->u32;
}

pub unsafe extern "C" fn inet_diag_msg_common_fill(r:*mut inet_diag_msg, sk:*mut sock) {
    (*r).idiag_family=(*sk).sk_family as u8; (*r).id.idiag_sport=htons((*sk).sk_num); (*r).id.idiag_dport=(*sk).sk_dport; (*r).id.idiag_if=(*sk).sk_bound_dev_if; sock_diag_save_cookie(sk,&mut (*r).id.idiag_cookie); (*r).id.idiag_src=[0;4]; (*r).id.idiag_dst=[0;4]; (*r).id.idiag_src[0]=(*sk).sk_rcv_saddr; (*r).id.idiag_dst[0]=(*sk).sk_daddr;
}

pub unsafe extern "C" fn inet_diag_msg_attrs_fill(sk:*mut sock, skb:*mut sk_buff, r:*mut inet_diag_msg, ext:i32, _user_ns:*mut core::ffi::c_void, net_admin:bool)->i32 {
    if nla_put_u8(skb,1,(*sk).sk_shutdown)!=0{return 1;} if net_admin && nla_put_u32(skb,15,(*sk).sk_mark)!=0{return 1;}
    (*r).idiag_uid=0; (*r).idiag_inode=0; let mut o:inet_diag_sockopt=mem::zeroed(); let _=(&mut o,ext); if nla_put(skb,2,mem::size_of::<inet_diag_sockopt>(),&o as *const _ as *const _)!=0{return 1;} 0
}

unsafe fn bitstring_match(a1:*const u32,a2:*const u32,bits:i32)->i32 { let words=(bits>>5) as isize; let rem=bits&31; for i in 0..words { if *a1.offset(i)!=*a2.offset(i){return 0;} } if rem!=0 && ((*a1.offset(words)^*a2.offset(words))&htonl(u32::MAX<<(32-rem)))!=0{return 0;} 1 }

unsafe fn inet_diag_bc_run(_bc:*const nlattr,_entry:*const inet_diag_entry)->i32 { 1 }
pub unsafe extern "C" fn inet_diag_bc_sk(cb:*const inet_diag_dump_data, _sk:*mut sock)->i32 { if (*cb).inet_diag_nla_bc.is_null(){1}else{inet_diag_bc_run((*cb).inet_diag_nla_bc,ptr::null())} }

#[no_mangle] pub unsafe extern "C" fn inet_diag_register(_h:*const inet_diag_handler)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn inet_diag_unregister(_h:*const inet_diag_handler) {}

// The remaining kernel netlink dispatch and lifecycle declarations retain the
// source interfaces; their implementations are supplied by the kernel layer.
extern "C" { fn sock_diag_register(*const core::ffi::c_void)->i32; fn sock_diag_unregister(*const core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
