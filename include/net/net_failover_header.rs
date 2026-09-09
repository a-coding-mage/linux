/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018, Intel Corporation. */

// C dependency: <net/failover.h>

/* failover state */
#[repr(C)]
pub struct net_failover_info {
	/* primary netdev with same MAC */
	pub primary_dev: *mut net_device,

	/* standby netdev */
	pub standby_dev: *mut net_device,

	/* primary netdev stats */
	pub primary_stats: rtnl_link_stats64,

	/* standby netdev stats */
	pub standby_stats: rtnl_link_stats64,

	/* aggregated stats */
	pub failover_stats: rtnl_link_stats64,

	/* spinlock while updating stats */
	pub stats_lock: spinlock_t,
}

extern "C" {
	pub fn net_failover_create(standby_dev: *mut net_device) -> *mut failover;
	pub fn net_failover_destroy(failover: *mut failover);
}

pub const FAILOVER_VLAN_FEATURES: _ = NETIF_F_HW_CSUM
	| NETIF_F_SG
	| NETIF_F_FRAGLIST
	| NETIF_F_ALL_TSO
	| NETIF_F_HIGHDMA
	| NETIF_F_LRO;

pub const FAILOVER_ENC_FEATURES: _ = NETIF_F_HW_CSUM
	| NETIF_F_SG
	| NETIF_F_RXCSUM
	| NETIF_F_ALL_TSO;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
