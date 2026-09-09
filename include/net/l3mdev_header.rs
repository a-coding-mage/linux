/* SPDX-License-Identifier: GPL-2.0-or-later */
/* include/net/l3mdev.h - L3 master device API */

use core::ffi::c_int;

/* Dependencies supplied by the surrounding kernel translation. */
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct dst_entry { _private: [u8; 0] }
#[repr(C)] pub struct flowi { _private: [u8; 0] }
#[repr(C)] pub struct flowi6 { _private: [u8; 0] }
#[repr(C)] pub struct fib_lookup_arg { _private: [u8; 0] }

pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum l3mdev_type {
    L3MDEV_TYPE_UNSPEC,
    L3MDEV_TYPE_VRF,
    __L3MDEV_TYPE_MAX,
}
pub const L3MDEV_TYPE_MAX: l3mdev_type = l3mdev_type::__L3MDEV_TYPE_MAX;

pub type lookup_by_table_id_t = Option<unsafe extern "C" fn(*mut net, u32) -> c_int>;

#[repr(C)]
pub struct l3mdev_ops {
    pub l3mdev_fib_table: Option<unsafe extern "C" fn(*const net_device) -> u32>,
    pub l3mdev_l3_rcv: Option<unsafe extern "C" fn(*mut net_device, *mut sk_buff, u16) -> *mut sk_buff>,
    pub l3mdev_l3_out: Option<unsafe extern "C" fn(*mut net_device, *mut sock, *mut sk_buff, u16) -> *mut sk_buff>,
    /* IPv6 ops */
    pub l3mdev_link_scope_lookup: Option<unsafe extern "C" fn(*const net_device, *mut flowi6) -> *mut dst_entry>,
}

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
extern "C" {
    pub fn l3mdev_table_lookup_register(l3type: l3mdev_type, func: lookup_by_table_id_t) -> c_int;
    pub fn l3mdev_table_lookup_unregister(l3type: l3mdev_type, func: lookup_by_table_id_t);
    pub fn l3mdev_ifindex_lookup_by_table_id(l3type: l3mdev_type, net: *mut net, table_id: u32) -> c_int;
    pub fn l3mdev_fib_rule_match(net: *mut net, fl: *mut flowi, arg: *mut fib_lookup_arg) -> c_int;
    pub fn l3mdev_update_flow(net: *mut net, fl: *mut flowi);
    pub fn l3mdev_master_ifindex_rcu(dev: *const net_device) -> c_int;
    pub fn l3mdev_master_upper_ifindex_by_index_rcu(net: *mut net, ifindex: c_int) -> c_int;
    pub fn l3mdev_fib_table_rcu(dev: *const net_device) -> u32;
    pub fn l3mdev_fib_table_by_index(net: *mut net, ifindex: c_int) -> u32;
    pub fn l3mdev_link_scope_lookup(net: *mut net, fl6: *mut flowi6) -> *mut dst_entry;
}

/* The CONFIG_NET_L3_MASTER_DEV-disabled branch is retained as inline fallbacks. */
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_ifindex_rcu(_: *const net_device) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_upper_ifindex_by_index_rcu(_: *mut net, _: c_int) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_table_rcu(_: *const net_device) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_table_by_index(_: *mut net, _: c_int) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_link_scope_lookup(_: *mut net, _: *mut flowi6) -> *mut dst_entry { core::ptr::null_mut() }

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_fib_rule_iif_match(_: *const flowi, _: c_int) -> bool { todo!("flowi fields supplied by dependency") }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_fib_rule_oif_match(_: *const flowi, _: c_int) -> bool { todo!("flowi fields supplied by dependency") }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_rule_iif_match(_: *const flowi, _: c_int) -> bool { false }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_rule_oif_match(_: *const flowi, _: c_int) -> bool { false }

/* Remaining inline definitions preserve the source interfaces; kernel helpers are external dependencies. */
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_master_ifindex(dev: *mut net_device) -> c_int {
    /* rcu_read_lock(); */ let ret = l3mdev_master_ifindex_rcu(dev); /* rcu_read_unlock(); */ ret
}
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_ifindex(_: *mut net_device) -> c_int { 0 }

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_master_upper_ifindex_by_index(net: *mut net, ifindex: c_int) -> c_int {
    /* rcu_read_lock(); */ let ret = l3mdev_master_upper_ifindex_by_index_rcu(net, ifindex); /* rcu_read_unlock(); */ ret
}
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_upper_ifindex_by_index(_: *mut net, _: c_int) -> c_int { 0 }

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_fib_table(dev: *const net_device) -> u32 { l3mdev_fib_table_rcu(dev) }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_table(_: *const net_device) -> u32 { 0 }

#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_dev_rcu(_: *const net_device) -> *mut net_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_master_ifindex_by_index(_: *mut net, _: c_int) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn netif_index_is_l3_master(_: *mut net, _: c_int) -> bool { false }

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_ip_rcv(skb: *mut sk_buff) -> *mut sk_buff { l3mdev_l3_rcv(skb, 2) }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_ip6_rcv(skb: *mut sk_buff) -> *mut sk_buff { l3mdev_l3_rcv(skb, 10) }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_l3_rcv(skb: *mut sk_buff, _proto: u16) -> *mut sk_buff { skb }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_l3_out(_sk: *mut sock, skb: *mut sk_buff, _proto: u16) -> *mut sk_buff { skb }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_ip_out(sk: *mut sock, skb: *mut sk_buff) -> *mut sk_buff { l3mdev_l3_out(sk, skb, 2) }
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_ip6_out(sk: *mut sock, skb: *mut sk_buff) -> *mut sk_buff { l3mdev_l3_out(sk, skb, 10) }

#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_ip_rcv(skb: *mut sk_buff) -> *mut sk_buff { skb }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_ip6_rcv(skb: *mut sk_buff) -> *mut sk_buff { skb }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_ip_out(_: *mut sock, skb: *mut sk_buff) -> *mut sk_buff { skb }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_ip6_out(_: *mut sock, skb: *mut sk_buff) -> *mut sk_buff { skb }

#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_table_lookup_register(_: l3mdev_type, _: lookup_by_table_id_t) -> c_int { -95 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_table_lookup_unregister(_: l3mdev_type, _: lookup_by_table_id_t) {}
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_ifindex_lookup_by_table_id(_: l3mdev_type, _: *mut net, _: u32) -> c_int { -19 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_fib_rule_match(_: *mut net, _: *mut flowi, _: *mut fib_lookup_arg) -> c_int { 1 }
#[cfg(not(feature = "CONFIG_NET_L3_MASTER_DEV"))]
pub unsafe fn l3mdev_update_flow(_: *mut net, _: *mut flowi) {}

#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
extern "C" {
    pub fn l3mdev_master_dev_rcu(dev: *const net_device) -> *mut net_device;
    pub fn netif_index_is_l3_master(net: *mut net, ifindex: c_int) -> bool;
}
#[cfg(feature = "CONFIG_NET_L3_MASTER_DEV")]
pub unsafe fn l3mdev_master_ifindex_by_index(net: *mut net, ifindex: c_int) -> c_int {
    l3mdev_master_ifindex_rcu(net as *const net_device).wrapping_add(ifindex)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
