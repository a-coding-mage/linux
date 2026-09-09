/* SPDX-License-Identifier: GPL-2.0 */
/* Multipath TCP */

use core::ffi::{c_int, c_void};

/* C headers supplied by the surrounding kernel translation. */
#[repr(C)] pub struct mptcp_info { _private: [u8; 0] }
#[repr(C)] pub struct mptcp_sock { _private: [u8; 0] }
#[repr(C)] pub struct mptcp_pm_addr_entry { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct request_sock { _private: [u8; 0] }
#[repr(C)] pub struct request_sock_ops { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct tcphdr { _private: [u8; 0] }
#[repr(C)] pub struct tcp_sock { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct in_addr { _private: [u8; 0] }
#[repr(C)] pub struct in6_addr { _private: [u8; 0] }

pub type sa_family_t = u16;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __sum16 = u16;

pub const MPTCPOPT_HMAC_LEN: usize = 20;
pub const MPTCP_RM_IDS_MAX: usize = 8;

#[repr(C)]
pub struct mptcp_ext {
    pub data_seq: u64,
    pub subflow_seq: u32,
    pub data_len: u16,
    pub csum: __sum16,
    pub use_map: u8,
    pub dsn64: u8,
    pub data_fin: u8,
    pub use_ack: u8,
    pub ack64: u8,
    pub mpc_map: u8,
    pub frozen: u8,
    pub reset_transient: u8,
    pub reset_reason: u8,
    pub csum_reqd: u8,
    pub infinite_map: u8,
}

#[repr(C)] pub struct mptcp_rm_list { pub ids: [u8; MPTCP_RM_IDS_MAX], pub nr: u8 }

#[repr(C)]
pub union mptcp_addr_union { pub addr: in_addr, pub addr6: in6_addr }
#[repr(C)]
pub struct mptcp_addr_info { pub id: u8, pub family: sa_family_t, pub port: __be16, pub addr: mptcp_addr_union }

#[repr(C)]
pub struct mptcp_out_options {
    #[cfg(CONFIG_MPTCP)] pub suboptions: u16,
    #[cfg(CONFIG_MPTCP)] pub rm_list: mptcp_rm_list,
    #[cfg(CONFIG_MPTCP)] pub join_id: u8,
    #[cfg(CONFIG_MPTCP)] pub backup: u8,
    #[cfg(CONFIG_MPTCP)] pub reset_reason: u8,
    #[cfg(CONFIG_MPTCP)] pub reset_transient: u8,
    #[cfg(CONFIG_MPTCP)] pub csum_reqd: u8,
    #[cfg(CONFIG_MPTCP)] pub allow_join_id0: u8,
    #[cfg(CONFIG_MPTCP)] pub drop_ts: u8,
    #[cfg(CONFIG_MPTCP)] pub data: mptcp_out_options_union,
}
#[repr(C)] pub union mptcp_out_options_union {
    pub keys: mptcp_out_options_keys,
    pub addr: mptcp_out_options_addr,
    pub ext: mptcp_out_options_ext,
    pub join: mptcp_out_options_join,
}
#[repr(C)] pub struct mptcp_out_options_keys { pub sndr_key:u64, pub rcvr_key:u64, pub data_seq:u64, pub subflow_seq:u32, pub data_len:u16, pub csum:__sum16 }
#[repr(C)] pub struct mptcp_out_options_addr { pub addr:mptcp_addr_info, pub ahmac:u64 }
#[repr(C)] pub struct mptcp_out_options_ext { pub ext_copy:mptcp_ext, pub fail_seq:u64 }
#[repr(C)] pub struct mptcp_out_options_join { pub nonce:u32, pub token:u32, pub thmac:u64, pub hmac:[u8; MPTCPOPT_HMAC_LEN] }

pub const MPTCP_SCHED_NAME_MAX: usize = 16;
pub const MPTCP_SCHED_MAX: usize = 128;
pub const MPTCP_SCHED_BUF_MAX: usize = MPTCP_SCHED_NAME_MAX * MPTCP_SCHED_MAX;
pub const MPTCP_PM_NAME_MAX: usize = 16;
pub const MPTCP_PM_MAX: usize = 128;
pub const MPTCP_PM_BUF_MAX: usize = MPTCP_PM_NAME_MAX * MPTCP_PM_MAX;

#[repr(C)] pub struct mptcp_sched_ops { pub get_send: Option<unsafe extern "C" fn(*mut mptcp_sock)->c_int>, pub get_retrans: Option<unsafe extern "C" fn(*mut mptcp_sock)->c_int>, pub name:[u8;16], pub owner:*mut module, pub list:list_head, pub init:Option<unsafe extern "C" fn(*mut mptcp_sock)>, pub release:Option<unsafe extern "C" fn(*mut mptcp_sock)> }
#[repr(C)] pub struct mptcp_pm_ops { pub name:[u8;16], pub owner:*mut module, pub list:list_head, pub init:Option<unsafe extern "C" fn(*mut mptcp_sock)>, pub release:Option<unsafe extern "C" fn(*mut mptcp_sock)> }

#[cfg(CONFIG_MPTCP)]
extern "C" {
    pub fn mptcp_init();
    pub fn mptcp_space(ssk:*const sock, space:*mut c_int, full_space:*mut c_int);
    pub fn mptcp_syn_options(sk:*mut sock, skb:*const sk_buff, size:*mut u32, opts:*mut mptcp_out_options)->bool;
    pub fn mptcp_synack_options(req:*const request_sock, size:*mut u32, opts:*mut mptcp_out_options)->bool;
    pub fn mptcp_established_options(sk:*mut sock, skb:*mut sk_buff, remaining:u32, has_ts:bool, opts:*mut mptcp_out_options)->c_int;
    pub fn mptcp_incoming_options(sk:*mut sock, skb:*mut sk_buff)->bool;
    pub fn mptcp_write_options(th:*mut tcphdr, ptr:*mut __be32, tp:*mut tcp_sock, opts:*mut mptcp_out_options);
    pub fn mptcp_diag_fill_info(msk:*mut mptcp_sock, info:*mut mptcp_info);
    pub fn mptcp_seq_show(seq:*mut seq_file);
    pub fn mptcp_subflow_init_cookie_req(req:*mut request_sock, sk_listener:*const sock, skb:*mut sk_buff)->c_int;
    pub fn mptcp_subflow_reqsk_alloc(ops:*const request_sock_ops, sk_listener:*mut sock, attach_listener:bool)->*mut request_sock;
    pub fn mptcp_get_reset_option(skb:*const sk_buff)->__be32;
    pub fn mptcp_active_detect_blackhole(sk:*mut sock, expired:bool);
}

#[cfg(not(CONFIG_MPTCP))]
pub unsafe fn mptcp_init() {}
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn sk_is_mptcp(_: *const sock)->bool { false }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn rsk_is_mptcp(_: *const request_sock)->bool { false }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn rsk_drop_req(_: *const request_sock)->bool { false }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_syn_options(_: *mut sock, _: *const sk_buff, _: *mut u32, _: *mut mptcp_out_options)->bool { false }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_synack_options(_: *const request_sock, _: *mut u32, _: *mut mptcp_out_options)->bool { false }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_incoming_options(_: *mut sock, _: *mut sk_buff)->bool { true }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_skb_ext_move(_: *mut sk_buff, _: *const sk_buff) {}
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_skb_ext_copy(_: *mut sk_buff, _: *mut sk_buff) {}
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_skb_can_collapse(_: *const sk_buff, _: *const sk_buff)->bool { true }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_space(_: *const sock, _: *mut c_int, _: *mut c_int) {}
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_seq_show(_: *mut seq_file) {}
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_subflow_init_cookie_req(_: *mut request_sock, _: *const sock, _: *mut sk_buff)->c_int { 0 }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_subflow_reqsk_alloc(_: *const request_sock_ops, _: *mut sock, _: bool)->*mut request_sock { core::ptr::null_mut() }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_reset_option(_: *const sk_buff)->__be32 { 0u32.to_be() }
#[cfg(not(CONFIG_MPTCP))] pub unsafe fn mptcp_active_detect_blackhole(_: *mut sock, _: bool) {}

#[cfg(IS_ENABLED_CONFIG_MPTCP_IPV6)] pub fn mptcpv6_init() -> c_int;
#[cfg(IS_ENABLED_CONFIG_MPTCP_IPV6)] pub fn mptcpv6_handle_mapped(sk:*mut sock, mapped:bool);
#[cfg(all(not(IS_ENABLED_CONFIG_MPTCP_IPV6), IS_ENABLED_CONFIG_IPV6))] pub fn mptcpv6_init()->c_int { 0 }
#[cfg(all(not(IS_ENABLED_CONFIG_MPTCP_IPV6), IS_ENABLED_CONFIG_IPV6))] pub fn mptcpv6_handle_mapped(_: *mut sock, _: bool) {}

#[cfg(all(CONFIG_MPTCP, CONFIG_BPF_SYSCALL))] extern "C" { pub fn bpf_mptcp_sock_from_subflow(sk:*mut sock)->*mut mptcp_sock; }
#[cfg(not(all(CONFIG_MPTCP, CONFIG_BPF_SYSCALL)))] pub unsafe fn bpf_mptcp_sock_from_subflow(_: *mut sock)->*mut mptcp_sock { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
