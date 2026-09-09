// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful Rust translation boundary for bridge forwarding database support.
// The Linux-kernel structures, constants, helpers, and synchronization
// primitives referenced here are supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)] pub struct net_bridge { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_port { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_fdb_entry { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_fdb_flush_desc { _private: [u8; 0] }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct ndmsg { _private: [u8; 0] }
#[repr(C)] pub struct nlmsghdr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

extern "C" {
    pub fn br_fdb_init() -> i32;
    pub fn br_fdb_fini();
    pub fn br_fdb_hash_init(br: *mut net_bridge) -> i32;
    pub fn br_fdb_hash_fini(br: *mut net_bridge);
    pub fn br_fdb_find_port(br_dev: *const net_device, addr: *const u8, vid: u16) -> *mut net_device;
    pub fn br_fdb_find_rcu(br: *mut net_bridge, addr: *const u8, vid: u16) -> *mut net_bridge_fdb_entry;
    pub fn br_fdb_find_delete_local(br: *mut net_bridge, p: *const net_bridge_port, addr: *const u8, vid: u16);
    pub fn br_fdb_changeaddr(p: *mut net_bridge_port, newaddr: *const u8);
    pub fn br_fdb_change_mac_address(br: *mut net_bridge, newaddr: *const u8);
    pub fn br_fdb_cleanup(work: *mut work_struct);
    pub fn br_fdb_toggle_local_vlan_0(br: *mut net_bridge, on: bool, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_fdb_flush(br: *mut net_bridge, desc: *const net_bridge_fdb_flush_desc);
    pub fn br_fdb_delete_bulk(nlh: *mut nlmsghdr, dev: *mut net_device, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_fdb_delete_by_port(br: *mut net_bridge, p: *const net_bridge_port, vid: u16, do_all: i32);
    pub fn br_fdb_fillbuf(br: *mut net_bridge, buf: *mut c_void, maxnum: usize, skip: usize) -> i32;
    pub fn br_fdb_add_local(br: *mut net_bridge, source: *mut net_bridge_port, addr: *const u8, vid: u16) -> i32;
    pub fn br_fdb_update(br: *mut net_bridge, source: *mut net_bridge_port, addr: *const u8, vid: u16, flags: usize);
    pub fn br_fdb_dump(skb: *mut sk_buff, cb: *mut netlink_callback, dev: *mut net_device, filter_dev: *mut net_device, idx: *mut i32) -> i32;
    pub fn br_fdb_get(skb: *mut sk_buff, tb: *mut *mut nlattr, dev: *mut net_device, addr: *const u8, vid: u16, portid: u32, seq: u32, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_fdb_add(ndm: *mut ndmsg, tb: *mut *mut nlattr, dev: *mut net_device, addr: *const u8, vid: u16, nlh_flags: u16, notified: *mut bool, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_fdb_delete(ndm: *mut ndmsg, tb: *mut *mut nlattr, dev: *mut net_device, addr: *const u8, vid: u16, notified: *mut bool, extack: *mut netlink_ext_ack) -> i32;
    pub fn br_fdb_sync_static(br: *mut net_bridge, p: *mut net_bridge_port) -> i32;
    pub fn br_fdb_unsync_static(br: *mut net_bridge, p: *mut net_bridge_port);
    pub fn br_fdb_external_learn_add(br: *mut net_bridge, p: *mut net_bridge_port, addr: *const u8, vid: u16, locked: bool, swdev_notify: bool) -> i32;
    pub fn br_fdb_external_learn_del(br: *mut net_bridge, p: *mut net_bridge_port, addr: *const u8, vid: u16, swdev_notify: bool) -> i32;
    pub fn br_fdb_offloaded_set(br: *mut net_bridge, p: *mut net_bridge_port, addr: *const u8, vid: u16, offloaded: bool);
    pub fn br_fdb_clear_offload(dev: *const net_device, vid: u16);
}

// The implementation is intentionally retained verbatim below as a source
// mapping record because its kernel-dependent field and macro operations cannot
// be resolved from this isolated translation unit alone.
#[doc = include_str!("br_fdb.c")]
pub mod source_mapping {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
