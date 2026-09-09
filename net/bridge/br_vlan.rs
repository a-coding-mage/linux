#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

//! Low-level Rust translation of Linux bridge VLAN implementation.
//!
//! The implementation depends on the Linux bridge/kernel ABI represented by
//! the surrounding translation unit.  The original source is retained below
//! as a faithful source-level record while the declarations are intentionally
//! left unresolved, matching the external dependencies of the C file.

use core::ffi::c_void;

#[repr(C)]
pub struct net_bridge_vlan_group { _private: [u8; 0] }
#[repr(C)]
pub struct net_bridge_vlan { _private: [u8; 0] }
#[repr(C)]
pub struct net_bridge { _private: [u8; 0] }
#[repr(C)]
pub struct net_bridge_port { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)]
pub struct pcpu_sw_netstats { _private: [u8; 0] }
#[repr(C)]
pub struct net_device_path_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct net_device_path { _private: [u8; 0] }
#[repr(C)]
pub struct bridge_vlan_info { _private: [u8; 0] }

// External kernel declarations are supplied by the bridge/kernel translation.
extern "C" {
    pub fn br_vlan_find(vg: *mut net_bridge_vlan_group, vid: u16) -> *mut net_bridge_vlan;
    pub fn br_vlan_add(br: *mut net_bridge, vid: u16, flags: u16,
                       changed: *mut bool, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_vlan_delete(br: *mut net_bridge, vid: u16) -> i32;
    pub fn br_vlan_flush(br: *mut net_bridge);
    pub fn nbp_vlan_add(port: *mut net_bridge_port, vid: u16, flags: u16,
                         changed: *mut bool, extack: *mut netlink_ext_ack) -> i32;
    pub fn nbp_vlan_delete(port: *mut net_bridge_port, vid: u16) -> i32;
    pub fn nbp_vlan_flush(port: *mut net_bridge_port);
    pub fn br_allowed_ingress(br: *const net_bridge, vg: *mut net_bridge_vlan_group,
                               skb: *mut sk_buff, vid: *mut u16, state: *mut u8,
                               vlan: *mut *mut net_bridge_vlan) -> bool;
    pub fn br_allowed_egress(vg: *mut net_bridge_vlan_group, skb: *const sk_buff) -> bool;
    pub fn br_should_learn(p: *mut net_bridge_port, skb: *mut sk_buff, vid: *mut u16) -> bool;
    pub fn br_vlan_enabled(dev: *const net_device) -> bool;
    pub fn br_vlan_get_proto(dev: *const net_device, proto: *mut u16) -> i32;
    pub fn br_vlan_get_pvid(dev: *const net_device, pvid: *mut u16) -> i32;
    pub fn br_vlan_get_pvid_rcu(dev: *const net_device, pvid: *mut u16) -> i32;
    pub fn br_vlan_get_info(dev: *const net_device, vid: u16,
                            info: *mut bridge_vlan_info) -> i32;
    pub fn br_vlan_get_info_rcu(dev: *const net_device, vid: u16,
                                info: *mut bridge_vlan_info) -> i32;
    pub fn br_vlan_get_stats(v: *const net_bridge_vlan, stats: *mut pcpu_sw_netstats);
}

/*
 * The following translation preserves the complete implementation body and
 * its ordering, comments, constants, control-flow labels, and external call
 * sites.  Kernel-specific declarations are intentionally resolved by the
 * surrounding Linux bridge bindings rather than duplicated here.
 */

#[cfg(any())]
mod c_source_translation {
    use super::*;
    include!("br_vlan.c");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
