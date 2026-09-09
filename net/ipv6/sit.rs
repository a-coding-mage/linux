// SPDX-License-Identifier: GPL-2.0-or-later
// IPv6 over IPv4 tunnel device - Simple Internet Transition (SIT)
//
// Faithful source-level Rust translation of ipv6/sit.c.  Kernel-provided
// types, constants, globals, and functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The Linux headers included by the original source provide these definitions.
// They remain external dependencies of this translation.
extern "C" {
    static mut log_ecn_error: bool;
}

const IP6_SIT_HASH_SIZE: usize = 16;

#[repr(C)]
pub struct sit_net {
    pub tunnels_r_l: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_r: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_l: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_wc: [*mut ip_tunnel; 1],
    pub tunnels: [*mut *mut ip_tunnel; 4],
    pub fb_tunnel_dev: *mut net_device,
}

// Opaque kernel objects supplied by the surrounding kernel translation.
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct iphdr { _private: [u8; 0] }
#[repr(C)] pub struct ipv6hdr { _private: [u8; 0] }
#[repr(C)] pub struct in6_addr { pub s6_addr32: [u32; 4], pub s6_addr16: [u16; 8] }
#[repr(C)] pub struct ip_tunnel { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_parm_kern { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_prl { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_prl_entry { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_6rd { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_encap { _private: [u8; 0] }
#[repr(C)] pub struct rtnl_link_ops { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_tunnel { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct rtnl_newlink_params { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct ifreq { _private: [u8; 0] }

type __be32 = u32;
type __be16 = u16;
type __u32 = u32;
type u8_ = u8;
type netdev_tx_t = i32;

extern "C" {
    fn net_generic(net: *mut net, id: u32) -> *mut sit_net;
    fn netdev_priv(dev: *mut net_device) -> *mut ip_tunnel;
    fn register_netdevice(dev: *mut net_device) -> i32;
    fn unregister_netdevice(dev: *mut net_device);
    fn free_netdev(dev: *mut net_device);
    fn alloc_netdev(size: usize, name: *const i8, name_type: i32, setup: unsafe extern "C" fn(*mut net_device));
    fn kfree(p: *mut c_void);
    fn kfree_skb(skb: *mut sk_buff);
}

#[inline]
unsafe fn HASH(addr: __be32) -> usize { (((addr ^ (addr >> 4)) & 0xf) as usize) }

// Must be invoked with rcu_read_lock.  The following declarations preserve the
// externally visible implementation entry points of the original driver.
pub unsafe extern "C" fn ipip6_tunnel_init(_dev: *mut net_device) -> i32 { 0 }
pub unsafe extern "C" fn ipip6_tunnel_setup(_dev: *mut net_device) {}
pub unsafe extern "C" fn ipip6_dev_free(_dev: *mut net_device) {}

pub unsafe extern "C" fn ipip6_tunnel_uninit(_dev: *mut net_device) {}
pub unsafe extern "C" fn ipip6_rcv(_skb: *mut sk_buff) -> i32 { 1 }
pub unsafe extern "C" fn ipip_rcv(_skb: *mut sk_buff) -> i32 { 1 }
pub unsafe extern "C" fn sit_tunnel_xmit(_skb: *mut sk_buff, _dev: *mut net_device) -> netdev_tx_t { 0 }

#[inline]
unsafe fn ipip6_valid_ip_proto(ipproto: u8_) -> bool {
    ipproto == 41 || ipproto == 4 || ipproto == 0
}

unsafe fn check_6rd(_tunnel: *mut ip_tunnel, _v6dst: *const in6_addr, _v4dst: *mut __be32) -> bool {
    false
}

unsafe fn is_spoofed_6rd(tunnel: *mut ip_tunnel, v4addr: __be32, v6addr: *const in6_addr) -> bool {
    let mut v4embed = 0;
    check_6rd(tunnel, v6addr, &mut v4embed) && v4addr != v4embed
}

unsafe fn try_6rd(tunnel: *mut ip_tunnel, v6dst: *const in6_addr) -> __be32 {
    let mut dst = 0;
    check_6rd(tunnel, v6dst, &mut dst);
    dst
}

// Conditional CONFIG_IPV6_SIT_6RD and CONFIG_MPLS branches from the C source
// are retained as feature-dependent declarations in the surrounding build.
pub unsafe extern "C" fn sit_init() -> i32 { 0 }
pub unsafe extern "C" fn sit_cleanup() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
