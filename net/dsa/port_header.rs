/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the Linux and DSA headers are referenced here.

#[repr(C)]
pub struct ifreq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_lag_lower_state_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netdev_lag_upper_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct switchdev_mst_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct switchdev_obj_port_mdb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct switchdev_vlan_msti {
    _private: [u8; 0],
}

#[repr(C)]
pub struct phy_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn dsa_port_supports_hwtstamp(dp: *mut dsa_port) -> bool;
    pub fn dsa_port_set_tag_protocol(
        cpu_dp: *mut dsa_port,
        tag_ops: *const dsa_device_ops,
    );
    pub fn dsa_port_set_state(dp: *mut dsa_port, state: u8, do_fast_age: bool) -> i32;
    pub fn dsa_port_set_mst_state(
        dp: *mut dsa_port,
        state: *const switchdev_mst_state,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_port_enable_rt(dp: *mut dsa_port, phy: *mut phy_device) -> i32;
    pub fn dsa_port_enable(dp: *mut dsa_port, phy: *mut phy_device) -> i32;
    pub fn dsa_port_disable_rt(dp: *mut dsa_port);
    pub fn dsa_port_disable(dp: *mut dsa_port);
    pub fn dsa_port_bridge_join(
        dp: *mut dsa_port,
        br: *mut net_device,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_port_pre_bridge_leave(dp: *mut dsa_port, br: *mut net_device);
    pub fn dsa_port_bridge_leave(dp: *mut dsa_port, br: *mut net_device);
    pub fn dsa_port_lag_change(
        dp: *mut dsa_port,
        linfo: *mut netdev_lag_lower_state_info,
    ) -> i32;
    pub fn dsa_port_lag_join(
        dp: *mut dsa_port,
        lag_dev: *mut net_device,
        uinfo: *mut netdev_lag_upper_info,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_port_pre_lag_leave(dp: *mut dsa_port, lag_dev: *mut net_device);
    pub fn dsa_port_lag_leave(dp: *mut dsa_port, lag_dev: *mut net_device);
    pub fn dsa_port_vlan_filtering(
        dp: *mut dsa_port,
        vlan_filtering: bool,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_port_skip_vlan_configuration(dp: *mut dsa_port) -> bool;
    pub fn dsa_port_ageing_time(dp: *mut dsa_port, ageing_clock: clock_t) -> i32;
    pub fn dsa_port_mst_enable(
        dp: *mut dsa_port,
        on: bool,
        extack: *mut netlink_ext_ack,
    ) -> i32;
    pub fn dsa_port_vlan_msti(dp: *mut dsa_port, msti: *const switchdev_vlan_msti) -> i32;
    pub fn dsa_port_mtu_change(dp: *mut dsa_port, new_mtu: i32) -> i32;
    pub fn dsa_port_fdb_add(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_fdb_del(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_standalone_host_fdb_add(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_standalone_host_fdb_del(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_bridge_host_fdb_add(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_bridge_host_fdb_del(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_lag_fdb_add(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_lag_fdb_del(dp: *mut dsa_port, addr: *const u8, vid: u16) -> i32;
    pub fn dsa_port_fdb_dump(dp: *mut dsa_port, cb: *mut dsa_fdb_dump_cb_t, data: *mut core::ffi::c_void) -> i32;
    pub fn dsa_port_mdb_add(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_mdb_del(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_standalone_host_mdb_add(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_standalone_host_mdb_del(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_bridge_host_mdb_add(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_bridge_host_mdb_del(dp: *const dsa_port, mdb: *const switchdev_obj_port_mdb) -> i32;
    pub fn dsa_port_pre_bridge_flags(dp: *const dsa_port, flags: switchdev_brport_flags, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_port_bridge_flags(dp: *mut dsa_port, flags: switchdev_brport_flags, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_port_vlan_add(dp: *mut dsa_port, vlan: *const switchdev_obj_port_vlan, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_port_vlan_del(dp: *mut dsa_port, vlan: *const switchdev_obj_port_vlan) -> i32;
    pub fn dsa_port_host_vlan_add(dp: *mut dsa_port, vlan: *const switchdev_obj_port_vlan, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_port_host_vlan_del(dp: *mut dsa_port, vlan: *const switchdev_obj_port_vlan) -> i32;
    pub fn dsa_port_mrp_add(dp: *const dsa_port, mrp: *const switchdev_obj_mrp) -> i32;
    pub fn dsa_port_mrp_del(dp: *const dsa_port, mrp: *const switchdev_obj_mrp) -> i32;
    pub fn dsa_port_mrp_add_ring_role(dp: *const dsa_port, mrp: *const switchdev_obj_ring_role_mrp) -> i32;
    pub fn dsa_port_mrp_del_ring_role(dp: *const dsa_port, mrp: *const switchdev_obj_ring_role_mrp) -> i32;
    pub fn dsa_port_phylink_create(dp: *mut dsa_port) -> i32;
    pub fn dsa_port_phylink_destroy(dp: *mut dsa_port);
    pub fn dsa_shared_port_link_register_of(dp: *mut dsa_port) -> i32;
    pub fn dsa_shared_port_link_unregister_of(dp: *mut dsa_port);
    pub fn dsa_port_hsr_join(dp: *mut dsa_port, hsr: *mut net_device, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_port_hsr_leave(dp: *mut dsa_port, hsr: *mut net_device);
    pub fn dsa_port_tag_8021q_vlan_add(dp: *mut dsa_port, vid: u16, broadcast: bool) -> i32;
    pub fn dsa_port_tag_8021q_vlan_del(dp: *mut dsa_port, vid: u16, broadcast: bool);
    pub fn dsa_port_set_host_flood(dp: *mut dsa_port, uc: bool, mc: bool);
    pub fn dsa_port_change_conduit(dp: *mut dsa_port, conduit: *mut net_device, extack: *mut netlink_ext_ack) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
