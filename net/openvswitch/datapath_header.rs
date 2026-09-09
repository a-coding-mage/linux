/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2014 Nicira, Inc.
 */

/* Translated from datapath.h. Kernel and Open vSwitch dependencies are external. */

pub const DP_MAX_PORTS: u16 = u16::MAX;
pub const DP_VPORT_HASH_BUCKETS: usize = 1024;
pub const DP_MASKS_REBALANCE_INTERVAL: u32 = 4000;

#[repr(C)]
pub struct dp_stats_percpu {
    pub n_hit: u64,
    pub n_missed: u64,
    pub n_lost: u64,
    pub n_mask_hit: u64,
    pub n_cache_hit: u64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
pub struct dp_nlsk_pids {
    pub rcu: rcu_head,
    pub n_pids: u32,
    pub pids: [u32; 0],
}

#[repr(C)]
pub struct datapath {
    pub rcu: rcu_head,
    pub list_node: list_head,
    pub table: flow_table,
    pub ports: *mut hlist_head,
    pub stats_percpu: *mut dp_stats_percpu,
    pub net: possible_net_t,
    pub user_features: u32,
    pub max_headroom: u32,
    pub meter_tbl: dp_meter_table,
    pub upcall_portids: *mut dp_nlsk_pids,
}

#[repr(C)]
pub struct ovs_skb_cb {
    pub input_vport: *mut vport,
    pub mru: u16,
    pub acts_origlen: u16,
    pub cutlen: u32,
    pub probability: u32,
    pub upcall_pid: u32,
}

#[inline]
pub unsafe fn ovs_cb(skb: *mut sk_buff) -> *mut ovs_skb_cb {
    (*skb).cb.as_mut_ptr() as *mut ovs_skb_cb
}

#[repr(C)]
pub struct dp_upcall_info {
    pub egress_tun_info: *mut ip_tunnel_info,
    pub userdata: *const nlattr,
    pub actions: *const nlattr,
    pub actions_len: i32,
    pub portid: u32,
    pub cmd: u8,
    pub mru: u16,
}

#[repr(C)]
pub struct ovs_net {
    pub dps: list_head,
    pub dp_notify_work: work_struct,
    pub masks_rebalance: delayed_work,
    #[cfg(feature = "CONFIG_NETFILTER_CONNCOUNT")]
    pub ct_limit_info: *mut ovs_ct_limit_info,
    #[cfg(feature = "CONFIG_NETFILTER_CONNCOUNT")]
    pub ct_limit_exit_data: *mut ovs_ct_limit_info,
    pub xt_label: bool,
}

pub const MAX_L2_LEN: usize = VLAN_ETH_HLEN + 3 * MPLS_HLEN;

#[repr(C)]
pub struct ovs_frag_data {
    pub dst: usize,
    pub vport: *mut vport,
    pub cb: ovs_skb_cb,
    pub inner_protocol: __be16,
    pub network_offset: u16,
    pub vlan_tci: u16,
    pub vlan_proto: __be16,
    pub l2_len: c_uint,
    pub mac_proto: u8,
    pub l2_data: [u8; MAX_L2_LEN],
}

#[repr(C)]
pub struct deferred_action {
    pub skb: *mut sk_buff,
    pub actions: *const nlattr,
    pub actions_len: i32,
    pub pkt_key: sw_flow_key,
}

pub const DEFERRED_ACTION_FIFO_SIZE: usize = 10;
pub const OVS_RECURSION_LIMIT: i32 = 5;
pub const OVS_DEFERRED_ACTION_THRESHOLD: usize = (OVS_RECURSION_LIMIT - 2) as usize;

#[repr(C)]
pub struct action_fifo {
    pub head: i32,
    pub tail: i32,
    pub fifo: [deferred_action; DEFERRED_ACTION_FIFO_SIZE],
}

#[repr(C)]
pub struct action_flow_keys {
    pub key: [sw_flow_key; OVS_DEFERRED_ACTION_THRESHOLD],
}

#[repr(C)]
pub struct ovs_pcpu_storage {
    pub action_fifos: action_fifo,
    pub flow_keys: action_flow_keys,
    pub frag_data: ovs_frag_data,
    pub exec_level: i32,
    pub owner: *mut task_struct,
    pub bh_lock: local_lock_t,
}

extern "C" {
    pub static mut ovs_pcpu_storage: *mut ovs_pcpu_storage;
}

#[repr(u64)]
pub enum ovs_pkt_hash_types {
    OVS_PACKET_HASH_SW_BIT = 1u64 << 32,
    OVS_PACKET_HASH_L4_BIT = 1u64 << 33,
}

extern "C" {
    pub static mut ovs_net_id: c_uint;
    pub fn ovs_lock();
    pub fn ovs_unlock();
    #[cfg(feature = "CONFIG_LOCKDEP")]
    pub fn lockdep_ovsl_is_held() -> i32;
    pub fn ovs_lookup_vport(dp: *const datapath, port_no: u16) -> *mut vport;
    pub fn dev_get_by_index_rcu(net: *mut net, dp_ifindex: i32) -> *mut net_device;
    pub fn ovs_internal_dev_get_vport(dev: *mut net_device) -> *mut vport;
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline]
pub fn lockdep_ovsl_is_held() -> i32 { 1 }

#[inline]
pub unsafe fn ovs_vport_rcu(dp: *const datapath, port_no: i32) -> *mut vport {
    ovs_lookup_vport(dp, port_no as u16)
}

#[inline]
pub unsafe fn ovs_vport_ovsl_rcu(dp: *const datapath, port_no: i32) -> *mut vport {
    ovs_lookup_vport(dp, port_no as u16)
}

#[inline]
pub unsafe fn ovs_vport_ovsl(dp: *const datapath, port_no: i32) -> *mut vport {
    ovs_lookup_vport(dp, port_no as u16)
}

#[inline]
pub unsafe fn get_dp_rcu(net: *mut net, dp_ifindex: i32) -> *mut datapath {
    let dev = dev_get_by_index_rcu(net, dp_ifindex);
    if !dev.is_null() {
        let vport = ovs_internal_dev_get_vport(dev);
        if !vport.is_null() { return (*vport).dp; }
    }
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn get_dp(net: *mut net, dp_ifindex: i32) -> *mut datapath {
    get_dp_rcu(net, dp_ifindex)
}

extern "C" {
    pub static mut ovs_dp_device_notifier: notifier_block;
    pub static mut dp_vport_genl_family: genl_family;
    pub fn ovs_dp_process_packet(skb: *mut sk_buff, key: *mut sw_flow_key);
    pub fn ovs_dp_detach_port(vport: *mut vport);
    pub fn ovs_dp_upcall(dp: *mut datapath, skb: *mut sk_buff, key: *const sw_flow_key, info: *const dp_upcall_info, cutlen: u32) -> i32;
    pub fn ovs_dp_get_upcall_portid(dp: *const datapath, cpu_id: u32) -> u32;
    pub fn ovs_dp_name(dp: *const datapath) -> *const c_char;
    pub fn ovs_execute_actions(dp: *mut datapath, skb: *mut sk_buff, actions: *const sw_flow_actions, key: *mut sw_flow_key) -> i32;
    pub fn ovs_dp_notify_wq(work: *mut work_struct);
}

#[inline]
pub fn ovs_masked(old: u64, key: u64, mask: u64) -> u64 { key | (old & !mask) }

#[inline]
pub fn ovs_set_masked(old: &mut u64, key: u64, mask: u64) { *old = ovs_masked(*old, key, mask); }

/* OVS_NLERR and locking/RCU diagnostics are retained as external-kernel intent. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
