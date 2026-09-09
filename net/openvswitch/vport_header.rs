/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2012 Nicira, Inc.
 */

// Declarations supplied by the Linux/Open vSwitch headers are referenced here
// as external Rust types and functions.

#[repr(C)]
pub struct vport_portids {
    pub rn_ids: reciprocal_value,
    pub rcu: rcu_head,
    pub n_ids: u32,
    pub ids: [u32; 0],
}

#[repr(C)]
pub struct vport {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub dp: *mut datapath,
    pub upcall_portids: *mut vport_portids,
    pub port_no: u16,
    pub hash_node: hlist_node,
    pub dp_hash_node: hlist_node,
    pub ops: *const vport_ops,
    pub upcall_stats: *mut vport_upcall_stats_percpu,
    pub detach_list: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct vport_parms {
    pub name: *const core::ffi::c_char,
    pub type_: ovs_vport_type,
    pub desired_ifindex: core::ffi::c_int,
    pub dp: *mut datapath,
    pub port_no: u16,
    pub upcall_portids: *mut nlattr,
}

#[repr(C)]
pub struct vport_ops {
    pub type_: ovs_vport_type,
    pub create: Option<unsafe extern "C" fn(*const vport_parms) -> *mut vport>,
    pub destroy: Option<unsafe extern "C" fn(*mut vport)>,
    pub send: Option<unsafe extern "C" fn(*mut sk_buff) -> core::ffi::c_int>,
    pub list: list_head,
}

#[repr(C)]
pub struct vport_upcall_stats_percpu {
    pub syncp: u64_stats_sync,
    pub n_success: u64_stats_t,
    pub n_fail: u64_stats_t,
}

extern "C" {
    pub fn ovs_vport_init() -> core::ffi::c_int;
    pub fn ovs_vport_exit();
    pub fn ovs_vport_add(parms: *const vport_parms) -> *mut vport;
    pub fn ovs_vport_del(vport: *mut vport);
    pub fn ovs_vport_locate(net: *const net, name: *const core::ffi::c_char) -> *mut vport;
    pub fn ovs_vport_get_stats(vport: *mut vport, stats: *mut ovs_vport_stats);
    pub fn ovs_vport_get_upcall_stats(vport: *mut vport, skb: *mut sk_buff) -> core::ffi::c_int;
    pub fn ovs_vport_set_upcall_portids(vport: *mut vport, pids: *const nlattr) -> core::ffi::c_int;
    pub fn ovs_vport_get_upcall_portids(vport: *const vport, skb: *mut sk_buff) -> core::ffi::c_int;
    pub fn ovs_vport_find_upcall_portid(vport: *const vport, skb: *mut sk_buff) -> u32;
    pub fn ovs_vport_alloc(priv_size: core::ffi::c_int, ops: *const vport_ops, parms: *const vport_parms) -> *mut vport;
    pub fn ovs_vport_free(vport: *mut vport);
    pub fn ovs_vport_receive(vport: *mut vport, skb: *mut sk_buff, info: *const ip_tunnel_info) -> core::ffi::c_int;
    pub fn ovs_vport_ops_register(ops: *mut vport_ops) -> core::ffi::c_int;
    pub fn ovs_vport_ops_unregister(ops: *mut vport_ops);
    pub fn ovs_vport_send(vport: *mut vport, skb: *mut sk_buff, mac_proto: u8);
}

pub const VPORT_ALIGN: usize = 8;

#[inline]
pub unsafe fn vport_priv(vport: *const vport) -> *mut u8 {
    let size = core::mem::size_of::<vport>();
    (vport as *const u8).add((size + VPORT_ALIGN - 1) & !(VPORT_ALIGN - 1)) as *mut u8
}

#[inline]
pub unsafe fn vport_from_priv(priv_: *mut core::ffi::c_void) -> *mut vport {
    let size = core::mem::size_of::<vport>();
    (priv_ as *mut u8).sub((size + VPORT_ALIGN - 1) & !(VPORT_ALIGN - 1)) as *mut vport
}

#[inline]
pub unsafe fn ovs_vport_name(vport: *mut vport) -> *const core::ffi::c_char {
    (*vport).dev.as_ref().unwrap().name.as_ptr()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
