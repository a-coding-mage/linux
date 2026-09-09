/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright(c) 1999 - 2004 Intel Corporation. All rights reserved.
 */

// Translated from the C header. The original include supplies ETH_ALEN and
// other external kernel declarations.

use core::ffi::c_void;

#[repr(C)]
pub struct bonding {
    _private: [u8; 0],
}

#[repr(C)]
pub struct slave {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

pub type __be32 = u32;
pub type netdev_tx_t = c_int;
pub type c_int = i32;

// External kernel types supplied by other translated headers.
#[repr(C)]
pub struct atomic_t {
    pub counter: c_int,
}

// The source uses ETH_ALEN from <linux/if_ether.h>.
pub const ETH_ALEN: usize = 6;

// C macro equivalents; field access and argument typing are supplied by the
// surrounding bonding translation.
#[macro_export]
macro_rules! BOND_ALB_INFO { ($bond:expr) => { ($bond).alb_info }; }
#[macro_export]
macro_rules! SLAVE_TLB_INFO { ($slave:expr) => { ($slave).tlb_info }; }

pub const ALB_TIMER_TICKS_PER_SEC: u32 = 10; // should be a divisor of HZ
pub const BOND_TLB_REBALANCE_INTERVAL: u32 = 10; // In seconds, periodic re-balancing.
// Used for division - never set to zero !!!
pub const BOND_ALB_DEFAULT_LP_INTERVAL: u32 = 1;

#[macro_export]
macro_rules! BOND_ALB_LP_INTERVAL { ($bond:expr) => { ($bond).params.lp_interval }; }
#[macro_export]
macro_rules! BOND_TLB_REBALANCE_TICKS {
    () => { BOND_TLB_REBALANCE_INTERVAL * ALB_TIMER_TICKS_PER_SEC };
}
#[macro_export]
macro_rules! BOND_ALB_LP_TICKS {
    ($bond:expr) => { BOND_ALB_LP_INTERVAL!($bond) * ALB_TIMER_TICKS_PER_SEC };
}

pub const TLB_HASH_TABLE_SIZE: u32 = 256; // The size of the clients hash table.
// Note that this value MUST NOT be smaller because the key hash table is BYTE wide !
pub const TLB_NULL_INDEX: u32 = 0xffff_ffff;

// rlb defs
pub const RLB_HASH_TABLE_SIZE: u32 = 256;
pub const RLB_NULL_INDEX: u32 = 0xffff_ffff;
pub const RLB_UPDATE_DELAY: u32 = 2 * ALB_TIMER_TICKS_PER_SEC;
pub const RLB_ARP_BURST_SIZE: u32 = 2;
pub const RLB_UPDATE_RETRY: u32 = 3;
// RLB_PROMISC_TIMEOUT = 10 sec equals the time that the current slave is
// promiscuous after failover
pub const RLB_PROMISC_TIMEOUT: u32 = 10 * ALB_TIMER_TICKS_PER_SEC;

#[repr(C)]
pub struct tlb_client_info {
    pub tx_slave: *mut slave,
    pub tx_bytes: u32,
    pub load_history: u32,
    pub next: u32,
    pub prev: u32,
}

#[repr(C)]
pub struct rlb_client_info {
    pub ip_src: __be32,
    pub ip_dst: __be32,
    pub mac_src: [u8; ETH_ALEN],
    pub mac_dst: [u8; ETH_ALEN],
    pub used_next: u32,
    pub used_prev: u32,
    pub src_next: u32,
    pub src_prev: u32,
    pub src_first: u32,
    pub assigned: u8,
    pub ntt: u8,
    pub slave: *mut slave,
    pub vlan_id: u16,
}

#[repr(C)]
pub struct tlb_slave_info {
    pub head: u32,
    pub load: u32,
}

#[repr(C)]
pub struct alb_bond_info {
    pub tx_hashtbl: *mut tlb_client_info,
    pub unbalanced_load: u32,
    pub tx_rebalance_counter: atomic_t,
    pub lp_counter: c_int,
    // -------- rlb parameters --------
    pub rlb_enabled: c_int,
    pub rx_hashtbl: *mut rlb_client_info,
    pub rx_hashtbl_used_head: u32,
    pub rx_ntt: u8,
    pub rx_slave: *mut slave,
    pub primary_is_promisc: u8,
    pub rlb_promisc_timeout_counter: u32,
    pub rlb_update_delay_counter: u32,
    pub rlb_update_retry_counter: u32,
    pub rlb_rebalance: u8,
}

unsafe extern "C" {
    pub fn bond_alb_initialize(bond: *mut bonding, rlb_enabled: c_int) -> c_int;
    pub fn bond_alb_deinitialize(bond: *mut bonding);
    pub fn bond_alb_init_slave(bond: *mut bonding, slave: *mut slave) -> c_int;
    pub fn bond_alb_deinit_slave(bond: *mut bonding, slave: *mut slave);
    pub fn bond_alb_handle_link_change(bond: *mut bonding, slave: *mut slave, link: i8);
    pub fn bond_alb_handle_active_change(bond: *mut bonding, new_slave: *mut slave);
    pub fn bond_alb_xmit(skb: *mut sk_buff, bond_dev: *mut net_device) -> netdev_tx_t;
    pub fn bond_tlb_xmit(skb: *mut sk_buff, bond_dev: *mut net_device) -> netdev_tx_t;
    pub fn bond_xmit_alb_slave_get(bond: *mut bonding, skb: *mut sk_buff) -> *mut slave;
    pub fn bond_xmit_tlb_slave_get(bond: *mut bonding, skb: *mut sk_buff) -> *mut slave;
    pub fn bond_alb_monitor(work: *mut work_struct);
    pub fn bond_alb_set_mac_address(bond_dev: *mut net_device, addr: *mut c_void) -> c_int;
    pub fn bond_alb_clear_vlan(bond: *mut bonding, vlan_id: u16);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
