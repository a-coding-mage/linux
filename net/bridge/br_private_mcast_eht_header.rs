/* SPDX-License-Identifier: GPL-2.0-or-later
 * Copyright (c) 2020, Nikolay Aleksandrov <nikolay@nvidia.com>
 */

pub const BR_MCAST_DEFAULT_EHT_HOSTS_LIMIT: u32 = 512;

#[repr(C)]
pub union net_bridge_eht_addr {
    pub ip4: __be32,
    // Preserved from: #if IS_ENABLED(CONFIG_IPV6)
    #[cfg(feature = "CONFIG_IPV6")]
    pub ip6: in6_addr,
}

#[repr(C)]
pub struct net_bridge_group_eht_host {
    pub rb_node: rb_node,
    pub h_addr: net_bridge_eht_addr,
    pub set_entries: hlist_head,
    pub num_entries: c_uint,
    pub filter_mode: c_uchar,
    pub pg: *mut net_bridge_port_group,
}

#[repr(C)]
pub struct net_bridge_group_eht_set_entry {
    pub rb_node: rb_node,
    pub host_list: hlist_node,
    pub h_addr: net_bridge_eht_addr,
    pub timer: timer_list,
    pub br: *mut net_bridge,
    pub eht_set: *mut net_bridge_group_eht_set,
    pub h_parent: *mut net_bridge_group_eht_host,
    pub mcast_gc: net_bridge_mcast_gc,
}

#[repr(C)]
pub struct net_bridge_group_eht_set {
    pub rb_node: rb_node,
    pub src_addr: net_bridge_eht_addr,
    pub entry_tree: rb_root,
    pub timer: timer_list,
    pub pg: *mut net_bridge_port_group,
    pub br: *mut net_bridge,
    pub mcast_gc: net_bridge_mcast_gc,
}

// Preserved from: #ifdef CONFIG_BRIDGE_IGMP_SNOOPING
#[cfg(feature = "CONFIG_BRIDGE_IGMP_SNOOPING")]
extern "C" {
    pub fn br_multicast_eht_clean_sets(pg: *mut net_bridge_port_group);
    pub fn br_multicast_eht_handle(
        brmctx: *const net_bridge_mcast,
        pg: *mut net_bridge_port_group,
        h_addr: *mut c_void,
        srcs: *mut c_void,
        nsrcs: u32,
        addr_size: usize,
        grec_type: c_int,
    ) -> bool;
    pub fn br_multicast_eht_set_hosts_limit(
        p: *mut net_bridge_port,
        eht_hosts_limit: u32,
    ) -> c_int;
}

#[cfg(feature = "CONFIG_BRIDGE_IGMP_SNOOPING")]
#[inline]
pub unsafe fn br_multicast_eht_should_del_pg(
    pg: *const net_bridge_port_group,
) -> bool {
    ((*pg).key.port.as_ref().unwrap().flags & BR_MULTICAST_FAST_LEAVE != 0)
        && RB_EMPTY_ROOT!(&(*pg).eht_host_tree)
}

#[cfg(feature = "CONFIG_BRIDGE_IGMP_SNOOPING")]
#[inline]
pub unsafe fn br_multicast_eht_hosts_over_limit(
    pg: *const net_bridge_port_group,
) -> bool {
    let p = (*pg).key.port.as_ref().unwrap();

    p.multicast_eht_hosts_cnt >= p.multicast_eht_hosts_limit
}

#[cfg(feature = "CONFIG_BRIDGE_IGMP_SNOOPING")]
#[inline]
pub unsafe fn br_multicast_eht_hosts_inc(pg: *mut net_bridge_port_group) {
    let p = (*pg).key.port.as_mut().unwrap();

    p.multicast_eht_hosts_cnt += 1;
}

#[cfg(feature = "CONFIG_BRIDGE_IGMP_SNOOPING")]
#[inline]
pub unsafe fn br_multicast_eht_hosts_dec(pg: *mut net_bridge_port_group) {
    let p = (*pg).key.port.as_mut().unwrap();

    p.multicast_eht_hosts_cnt -= 1;
}
// Preserved from: #endif /* CONFIG_BRIDGE_IGMP_SNOOPING */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
