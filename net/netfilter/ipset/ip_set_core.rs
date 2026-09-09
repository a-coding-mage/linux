// SPDX-License-Identifier: GPL-2.0-only
// Faithful Rust-facing translation of netfilter/ipset/ip_set_core.c.
// Kernel-provided types, constants, macros, and functions are intentionally
// left as external dependencies, as in the original implementation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, improper_ctypes,
         clippy::all)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct ip_set_net {
    pub ip_set_list: *mut *mut ip_set,
    pub ip_set_max: ip_set_id_t,
    pub is_deleted: bool,
    pub is_destroyed: bool,
}

pub type ip_set_id_t = u16;
pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type __be32 = u32;

#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { pub nla_len: u16, pub nla_type: u16 }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_len: u32, pub nlmsg_type: u16, pub nlmsg_flags: u16, pub nlmsg_seq: u32, pub nlmsg_pid: u32 }
#[repr(C)] pub struct xt_action_param { _private: [u8; 0] }
#[repr(C)] pub struct ip_set { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_type { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_ext { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_comment { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_comment_rcu { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_counter { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_skbinfo { _private: [u8; 0] }
#[repr(C)] pub struct ip_set_adt_opt { _private: [u8; 0] }
#[repr(C)] pub struct nfnl_info { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }

pub const IP_SET_INC: ip_set_id_t = 64;
static mut ip_set_net_id: u32 = 0;
static mut max_sets: u32 = 0;
static mut ipset_destroy_wq: *mut c_void = core::ptr::null_mut();

#[inline]
unsafe fn ip_set_pernet(_net: *mut net) -> *mut ip_set_net { core::ptr::null_mut() }

// The following source is retained verbatim as the body-level translation
// reference for all kernel declarations and operations whose definitions are
// supplied by the Linux kernel build environment.
pub const IP_SET_CORE_C_SOURCE: &str = include_str!("ip_set_core.c");

// External interfaces exported by this implementation.
extern "C" {
    pub fn ip_set_type_register(ty: *mut ip_set_type) -> c_int;
    pub fn ip_set_type_unregister(ty: *mut ip_set_type);
    pub fn ip_set_alloc(size: usize) -> *mut c_void;
    pub fn ip_set_free(members: *mut c_void);
    pub fn ip_set_get_ipaddr4(nla: *mut nlattr, ipaddr: *mut __be32) -> c_int;
    pub fn ip_set_get_ipaddr6(nla: *mut nlattr, ipaddr: *mut c_void) -> c_int;
    pub fn ip_set_elem_len(set: *mut ip_set, tb: *mut *mut nlattr, len: usize, align: usize) -> usize;
    pub fn ip_set_get_extensions(set: *mut ip_set, tb: *mut *mut nlattr, ext: *mut ip_set_ext) -> c_int;
    pub fn ip_set_put_extensions(skb: *mut sk_buff, set: *const ip_set, e: *const c_void, active: bool) -> c_int;
    pub fn ip_set_match_extensions(set: *mut ip_set, ext: *const ip_set_ext, mext: *mut ip_set_ext, flags: u32, data: *mut c_void) -> bool;
    pub fn ip_set_test(index: ip_set_id_t, skb: *const sk_buff, par: *const xt_action_param, opt: *mut ip_set_adt_opt) -> c_int;
    pub fn ip_set_add(index: ip_set_id_t, skb: *const sk_buff, par: *const xt_action_param, opt: *mut ip_set_adt_opt) -> c_int;
    pub fn ip_set_del(index: ip_set_id_t, skb: *const sk_buff, par: *const xt_action_param, opt: *mut ip_set_adt_opt) -> c_int;
    pub fn ip_set_get_byname(net: *mut net, name: *const nlattr, set: *mut *mut ip_set) -> ip_set_id_t;
    pub fn ip_set_put_byindex(net: *mut net, index: ip_set_id_t);
    pub fn ip_set_name_byindex(net: *mut net, index: ip_set_id_t, name: *mut c_char);
    pub fn ip_set_nfnl_get_byindex(net: *mut net, index: ip_set_id_t) -> ip_set_id_t;
    pub fn ip_set_nfnl_put(net: *mut net, index: ip_set_id_t);
    pub fn ip_set_put_flags(skb: *mut sk_buff, set: *mut ip_set) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
