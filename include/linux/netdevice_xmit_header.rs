/* SPDX-License-Identifier: GPL-2.0-or-later */

// The following configuration-gated items correspond to IS_ENABLED(CONFIG_NET_ACT_MIRRED).
#[cfg(feature = "CONFIG_NET_ACT_MIRRED")]
pub const MIRRED_NEST_LIMIT: usize = 4;

// Declaration supplied by another translated dependency.
pub struct net_device;

#[repr(C)]
pub struct netdev_xmit {
    pub recursion: u16,
    pub more: u8,
    // Corresponds to CONFIG_NET_EGRESS.
    #[cfg(feature = "CONFIG_NET_EGRESS")]
    pub skip_txqueue: u8,
    // Corresponds to IS_ENABLED(CONFIG_NET_ACT_MIRRED).
    #[cfg(feature = "CONFIG_NET_ACT_MIRRED")]
    pub sched_mirred_nest: u8,
    #[cfg(feature = "CONFIG_NET_ACT_MIRRED")]
    pub sched_mirred_dev: [*mut net_device; MIRRED_NEST_LIMIT],
    // Corresponds to IS_ENABLED(CONFIG_NF_DUP_NETDEV).
    #[cfg(feature = "CONFIG_NF_DUP_NETDEV")]
    pub nf_dup_skb_recursion: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
