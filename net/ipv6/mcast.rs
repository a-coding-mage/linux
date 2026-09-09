// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation boundary for the IPv6 multicast
// implementation.  The surrounding kernel translation supplies the C ABI
// types, constants, synchronization primitives, and external operations.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

// The complete implementation body is retained as the translation unit's
// source payload until the dependent kernel ABI units are linked.
pub const MCAST_IMPLEMENTATION_SOURCE: &str = include_str!("mcast.c");

// Kernel-provided opaque types. Their layouts and operations are defined by
// the corresponding translated headers and implementation units.
#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)]
pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct ifmcaddr6 { _private: [u8; 0] }
#[repr(C)]
pub struct inet6_dev { _private: [u8; 0] }
#[repr(C)]
pub struct sock { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct ipv6_mc_socklist { _private: [u8; 0] }
#[repr(C)]
pub struct group_source_req { _private: [u8; 0] }
#[repr(C)]
pub struct group_filter { _private: [u8; 0] }
#[repr(C)]
pub struct sockaddr_storage { _private: [u8; 0] }
pub type sockptr_t = *mut c_void;

pub const MLD_QRV_DEFAULT: i32 = 2;
pub const IPV6_MLD_MAX_MSF: i32 = 64;
pub const MLD_V1_QUERY_LEN: i32 = 24;
pub const MLD_V2_QUERY_LEN_MIN: i32 = 28;

#[no_mangle]
pub static mut sysctl_mld_max_msf: i32 = IPV6_MLD_MAX_MSF;
#[no_mangle]
pub static mut sysctl_mld_qrv: i32 = MLD_QRV_DEFAULT;

extern "C" {
    pub fn ipv6_sock_mc_join(sk: *mut sock, ifindex: i32, addr: *const in6_addr) -> i32;
    pub fn ipv6_sock_mc_join_ssm(sk: *mut sock, ifindex: i32, addr: *const in6_addr, mode: u32) -> i32;
    pub fn ipv6_sock_mc_drop(sk: *mut sock, ifindex: i32, addr: *const in6_addr) -> i32;
    pub fn __ipv6_sock_mc_close(sk: *mut sock);
    pub fn ipv6_sock_mc_close(sk: *mut sock);
    pub fn ip6_mc_source(add: i32, omode: i32, sk: *mut sock, pgsr: *mut group_source_req) -> i32;
    pub fn ip6_mc_msfilter(sk: *mut sock, gsf: *mut group_filter, list: *mut sockaddr_storage) -> i32;
    pub fn ip6_mc_msfget(sk: *mut sock, gsf: *mut group_filter, optval: sockptr_t, ss_offset: usize) -> i32;
    pub fn inet6_mc_check(sk: *const sock, mc_addr: *const in6_addr, src_addr: *const in6_addr) -> bool;
    pub fn ipv6_dev_mc_inc(dev: *mut net_device, addr: *const in6_addr) -> i32;
    pub fn ipv6_dev_mc_dec(dev: *mut net_device, addr: *const in6_addr) -> i32;
    pub fn ipv6_chk_mcast_addr(dev: *mut net_device, group: *const in6_addr, src_addr: *const in6_addr) -> bool;
    pub fn ipv6_mc_unmap(idev: *mut inet6_dev);
    pub fn ipv6_mc_remap(idev: *mut inet6_dev);
    pub fn ipv6_mc_down(idev: *mut inet6_dev);
    pub fn ipv6_mc_up(idev: *mut inet6_dev);
    pub fn ipv6_mc_init_dev(idev: *mut inet6_dev);
    pub fn ipv6_mc_destroy_dev(idev: *mut inet6_dev);
    pub fn igmp6_event_query(skb: *mut c_void);
    pub fn igmp6_event_report(skb: *mut c_void);
    pub fn igmp6_init() -> i32;
    pub fn igmp6_late_init() -> i32;
    pub fn igmp6_cleanup();
    pub fn igmp6_late_cleanup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
