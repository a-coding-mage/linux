/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	Bridge per vlan tunnels
 *
 *	Authors:
 *	Roopa Prabhu		<roopa@cumulusnetworks.com>
 */

#[repr(C)]
pub struct vtunnel_info {
    pub tunid: u32,
    pub vid: u16,
    pub flags: u16,
}

/* br_netlink_tunnel.c */
extern "C" {
    pub fn br_parse_vlan_tunnel_info(
        attr: *mut nlattr,
        tinfo: *mut vtunnel_info,
    ) -> core::ffi::c_int;
    pub fn br_process_vlan_tunnel_info(
        br: *const net_bridge,
        p: *const net_bridge_port,
        cmd: core::ffi::c_int,
        tinfo_curr: *mut vtunnel_info,
        tinfo_last: *mut vtunnel_info,
        changed: *mut bool,
    ) -> core::ffi::c_int;
    pub fn br_get_vlan_tunnel_info_size(vg: *mut net_bridge_vlan_group) -> core::ffi::c_int;
    pub fn br_fill_vlan_tunnel_info(
        skb: *mut sk_buff,
        vg: *mut net_bridge_vlan_group,
    ) -> core::ffi::c_int;
    pub fn vlan_tunid_inrange(
        v_curr: *const net_bridge_vlan,
        v_last: *const net_bridge_vlan,
    ) -> bool;
    pub fn br_vlan_tunnel_info(
        p: *const net_bridge_port,
        cmd: core::ffi::c_int,
        vid: u16,
        tun_id: u32,
        changed: *mut bool,
    ) -> core::ffi::c_int;
}

/* CONFIG_BRIDGE_VLAN_FILTERING is supplied by the build configuration. */
#[cfg(feature = "CONFIG_BRIDGE_VLAN_FILTERING")]
extern "C" {
    pub fn vlan_tunnel_init(vg: *mut net_bridge_vlan_group) -> core::ffi::c_int;
    pub fn vlan_tunnel_deinit(vg: *mut net_bridge_vlan_group);
    pub fn nbp_vlan_tunnel_info_delete(
        port: *const net_bridge_port,
        vid: u16,
    ) -> core::ffi::c_int;
    pub fn nbp_vlan_tunnel_info_add(
        port: *const net_bridge_port,
        vid: u16,
        tun_id: u32,
    ) -> core::ffi::c_int;
    pub fn nbp_vlan_tunnel_info_flush(port: *mut net_bridge_port);
    pub fn vlan_tunnel_info_del(
        vg: *mut net_bridge_vlan_group,
        vlan: *mut net_bridge_vlan,
    );
    pub fn br_handle_ingress_vlan_tunnel(
        skb: *mut sk_buff,
        p: *mut net_bridge_port,
        vg: *mut net_bridge_vlan_group,
    );
    pub fn br_handle_egress_vlan_tunnel(
        skb: *mut sk_buff,
        vlan: *mut net_bridge_vlan,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn vlan_tunnel_init(_vg: *mut net_bridge_vlan_group) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn nbp_vlan_tunnel_info_delete(
    _port: *const net_bridge_port,
    _vid: u16,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn nbp_vlan_tunnel_info_add(
    _port: *const net_bridge_port,
    _vid: u16,
    _tun_id: u32,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn nbp_vlan_tunnel_info_flush(_port: *mut net_bridge_port) {}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn vlan_tunnel_info_del(
    _vg: *mut net_bridge_vlan_group,
    _vlan: *mut net_bridge_vlan,
) {
}

#[cfg(not(feature = "CONFIG_BRIDGE_VLAN_FILTERING"))]
pub unsafe fn br_handle_ingress_vlan_tunnel(
    _skb: *mut sk_buff,
    _p: *mut net_bridge_port,
    _vg: *mut net_bridge_vlan_group,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
