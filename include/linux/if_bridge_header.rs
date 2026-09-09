/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux ethernet bridge */

// Dependencies supplied by the surrounding kernel translation are referenced here.

#[repr(C)]
pub struct br_ip {
    pub src: br_ip_src,
    pub dst: br_ip_dst,
    pub proto: __be16,
    pub vid: __u16,
}

#[repr(C)]
pub union br_ip_src {
    pub ip4: __be32,
    #[cfg(feature = "CONFIG_IPV6")]
    pub ip6: in6_addr,
}

#[repr(C)]
pub union br_ip_dst {
    pub ip4: __be32,
    #[cfg(feature = "CONFIG_IPV6")]
    pub ip6: in6_addr,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
pub struct br_ip_list {
    pub list: list_head,
    pub addr: br_ip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bridge_flags_bit {
    BR_HAIRPIN_MODE_BIT,
    BR_BPDU_GUARD_BIT,
    BR_ROOT_BLOCK_BIT,
    BR_MULTICAST_FAST_LEAVE_BIT,
    BR_ADMIN_COST_BIT,
    BR_LEARNING_BIT,
    BR_FLOOD_BIT,
    BR_PROMISC_BIT,
    BR_PROXYARP_BIT,
    BR_LEARNING_SYNC_BIT,
    BR_PROXYARP_WIFI_BIT,
    BR_MCAST_FLOOD_BIT,
    BR_MULTICAST_TO_UNICAST_BIT,
    BR_VLAN_TUNNEL_BIT,
    BR_BCAST_FLOOD_BIT,
    BR_NEIGH_SUPPRESS_BIT,
    BR_ISOLATED_BIT,
    BR_MRP_AWARE_BIT,
    BR_MRP_LOST_CONT_BIT,
    BR_MRP_LOST_IN_CONT_BIT,
    BR_TX_FWD_OFFLOAD_BIT,
    BR_PORT_LOCKED_BIT,
    BR_PORT_MAB_BIT,
    BR_NEIGH_VLAN_SUPPRESS_BIT,
    BR_NEIGH_FORWARD_GRAT_BIT,
}

pub const BR_HAIRPIN_MODE: usize = 1usize << bridge_flags_bit::BR_HAIRPIN_MODE_BIT as usize;
pub const BR_BPDU_GUARD: usize = 1usize << bridge_flags_bit::BR_BPDU_GUARD_BIT as usize;
pub const BR_ROOT_BLOCK: usize = 1usize << bridge_flags_bit::BR_ROOT_BLOCK_BIT as usize;
pub const BR_MULTICAST_FAST_LEAVE: usize = 1usize << bridge_flags_bit::BR_MULTICAST_FAST_LEAVE_BIT as usize;
pub const BR_ADMIN_COST: usize = 1usize << bridge_flags_bit::BR_ADMIN_COST_BIT as usize;
pub const BR_LEARNING: usize = 1usize << bridge_flags_bit::BR_LEARNING_BIT as usize;
pub const BR_FLOOD: usize = 1usize << bridge_flags_bit::BR_FLOOD_BIT as usize;
pub const BR_AUTO_MASK: usize = BR_FLOOD | BR_LEARNING;
pub const BR_PROMISC: usize = 1usize << bridge_flags_bit::BR_PROMISC_BIT as usize;
pub const BR_PROXYARP: usize = 1usize << bridge_flags_bit::BR_PROXYARP_BIT as usize;
pub const BR_LEARNING_SYNC: usize = 1usize << bridge_flags_bit::BR_LEARNING_SYNC_BIT as usize;
pub const BR_PROXYARP_WIFI: usize = 1usize << bridge_flags_bit::BR_PROXYARP_WIFI_BIT as usize;
pub const BR_MCAST_FLOOD: usize = 1usize << bridge_flags_bit::BR_MCAST_FLOOD_BIT as usize;
pub const BR_MULTICAST_TO_UNICAST: usize = 1usize << bridge_flags_bit::BR_MULTICAST_TO_UNICAST_BIT as usize;
pub const BR_VLAN_TUNNEL: usize = 1usize << bridge_flags_bit::BR_VLAN_TUNNEL_BIT as usize;
pub const BR_BCAST_FLOOD: usize = 1usize << bridge_flags_bit::BR_BCAST_FLOOD_BIT as usize;
pub const BR_NEIGH_SUPPRESS: usize = 1usize << bridge_flags_bit::BR_NEIGH_SUPPRESS_BIT as usize;
pub const BR_ISOLATED: usize = 1usize << bridge_flags_bit::BR_ISOLATED_BIT as usize;
pub const BR_MRP_AWARE: usize = 1usize << bridge_flags_bit::BR_MRP_AWARE_BIT as usize;
pub const BR_MRP_LOST_CONT: usize = 1usize << bridge_flags_bit::BR_MRP_LOST_CONT_BIT as usize;
pub const BR_MRP_LOST_IN_CONT: usize = 1usize << bridge_flags_bit::BR_MRP_LOST_IN_CONT_BIT as usize;
pub const BR_TX_FWD_OFFLOAD: usize = 1usize << bridge_flags_bit::BR_TX_FWD_OFFLOAD_BIT as usize;
pub const BR_PORT_LOCKED: usize = 1usize << bridge_flags_bit::BR_PORT_LOCKED_BIT as usize;
pub const BR_PORT_MAB: usize = 1usize << bridge_flags_bit::BR_PORT_MAB_BIT as usize;
pub const BR_NEIGH_VLAN_SUPPRESS: usize = 1usize << bridge_flags_bit::BR_NEIGH_VLAN_SUPPRESS_BIT as usize;
pub const BR_NEIGH_FORWARD_GRAT: usize = 1usize << bridge_flags_bit::BR_NEIGH_FORWARD_GRAT_BIT as usize;

pub const BR_DEFAULT_AGEING_TIME: usize = 300 * HZ;

pub struct net_bridge;

extern "C" {
    pub fn brioctl_set(hook: Option<unsafe extern "C" fn(*mut net, c_uint, *mut c_void) -> c_int>);
    pub fn br_ioctl_call(net: *mut net, cmd: c_uint, uarg: *mut c_void) -> c_int;
}

#[cfg(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING"))]
extern "C" {
    pub fn br_multicast_list_adjacent(dev: *mut net_device, br_ip_list: *mut list_head) -> c_int;
    pub fn br_multicast_has_querier_anywhere(dev: *mut net_device, proto: c_int) -> bool;
    pub fn br_multicast_has_querier_adjacent(dev: *mut net_device, proto: c_int) -> bool;
    pub fn br_multicast_has_router_adjacent(dev: *mut net_device, proto: c_int) -> bool;
    pub fn br_multicast_enabled(dev: *const net_device) -> bool;
    pub fn br_multicast_router(dev: *const net_device) -> bool;
}

#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_list_adjacent(_: *mut net_device, _: *mut list_head) -> c_int { 0 }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_has_querier_anywhere(_: *mut net_device, _: c_int) -> bool { false }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_has_querier_adjacent(_: *mut net_device, _: c_int) -> bool { false }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_has_router_adjacent(_: *mut net_device, _: c_int) -> bool { true }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_enabled(_: *const net_device) -> bool { false }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_IGMP_SNOOPING")))]
pub unsafe fn br_multicast_router(_: *const net_device) -> bool { false }

#[cfg(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
extern "C" {
    pub fn br_vlan_enabled(dev: *const net_device) -> bool;
    pub fn br_vlan_get_pvid(dev: *const net_device, p_pvid: *mut u16) -> c_int;
    pub fn br_vlan_get_pvid_rcu(dev: *const net_device, p_pvid: *mut u16) -> c_int;
    pub fn br_vlan_get_proto(dev: *const net_device, p_proto: *mut u16) -> c_int;
    pub fn br_vlan_get_info(dev: *const net_device, vid: u16, p_vinfo: *mut bridge_vlan_info) -> c_int;
    pub fn br_vlan_get_info_rcu(dev: *const net_device, vid: u16, p_vinfo: *mut bridge_vlan_info) -> c_int;
    pub fn br_mst_enabled(dev: *const net_device) -> bool;
    pub fn br_mst_get_info(dev: *const net_device, msti: u16, vids: *mut c_ulong) -> c_int;
    pub fn br_mst_get_state(dev: *const net_device, msti: u16, state: *mut u8) -> c_int;
}

#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_enabled(_: *const net_device) -> bool { false }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_get_pvid(_: *const net_device, _: *mut u16) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_get_pvid_rcu(_: *const net_device, _: *mut u16) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_get_proto(_: *const net_device, _: *mut u16) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_get_info(_: *const net_device, _: u16, _: *mut bridge_vlan_info) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_vlan_get_info_rcu(_: *const net_device, _: u16, _: *mut bridge_vlan_info) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_mst_enabled(_: *const net_device) -> bool { false }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_mst_get_info(_: *const net_device, _: u16, _: *mut c_ulong) -> c_int { -EINVAL }
#[cfg(not(all(feature = "CONFIG_BRIDGE", feature = "CONFIG_BRIDGE_VLAN_FILTERING")))]
pub unsafe fn br_mst_get_state(_: *const net_device, _: u16, _: *mut u8) -> c_int { -EINVAL }

#[cfg(feature = "CONFIG_BRIDGE")]
extern "C" {
    pub fn br_fdb_find_port(br_dev: *const net_device, addr: *const u8, vid: __u16) -> *mut net_device;
    pub fn br_fdb_clear_offload(dev: *const net_device, vid: u16);
    pub fn br_port_flag_is_set(dev: *const net_device, flag: c_ulong) -> bool;
    pub fn br_port_get_stp_state(dev: *const net_device) -> u8;
    pub fn br_get_ageing_time(br_dev: *const net_device) -> clock_t;
}

#[cfg(not(feature = "CONFIG_BRIDGE"))]
pub unsafe fn br_fdb_find_port(_: *const net_device, _: *const u8, _: __u16) -> *mut net_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_BRIDGE"))]
pub unsafe fn br_fdb_clear_offload(_: *const net_device, _: u16) {}
#[cfg(not(feature = "CONFIG_BRIDGE"))]
pub unsafe fn br_port_flag_is_set(_: *const net_device, _: c_ulong) -> bool { false }
#[cfg(not(feature = "CONFIG_BRIDGE"))]
pub unsafe fn br_port_get_stp_state(_: *const net_device) -> u8 { BR_STATE_DISABLED }
#[cfg(not(feature = "CONFIG_BRIDGE"))]
pub unsafe fn br_get_ageing_time(_: *const net_device) -> clock_t { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
