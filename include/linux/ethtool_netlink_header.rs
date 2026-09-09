/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from <uapi/linux/ethtool_netlink.h>, <linux/ethtool.h>,
// and <linux/netdevice.h>. Their declarations are supplied externally.

pub const ETHTOOL_PAUSE_STAT_CNT: u32 =
    (__ETHTOOL_A_PAUSE_STAT_CNT - ETHTOOL_A_PAUSE_STAT_TX_FRAMES);

#[repr(C)]
pub enum ethtool_multicast_groups {
    ETHNL_MCGRP_MONITOR,
}

// The following types and constants are supplied by the corresponding kernel headers.

#[cfg(CONFIG_ETHTOOL_NETLINK)]
extern "C" {
    pub fn ethnl_cable_test_alloc(phydev: *mut phy_device, cmd: u8) -> ::core::ffi::c_int;
    pub fn ethnl_cable_test_free(phydev: *mut phy_device);
    pub fn ethnl_cable_test_finished(phydev: *mut phy_device);
    pub fn ethnl_cable_test_result_with_src(
        phydev: *mut phy_device, pair: u8, result: u8, src: u32,
    ) -> ::core::ffi::c_int;
    pub fn ethnl_cable_test_fault_length_with_src(
        phydev: *mut phy_device, pair: u8, cm: u32, src: u32,
    ) -> ::core::ffi::c_int;
    pub fn ethnl_cable_test_amplitude(
        phydev: *mut phy_device, pair: u8, mV: s16,
    ) -> ::core::ffi::c_int;
    pub fn ethnl_cable_test_pulse(phydev: *mut phy_device, mV: u16) -> ::core::ffi::c_int;
    pub fn ethnl_cable_test_step(
        phydev: *mut phy_device, first: u32, last: u32, step: u32,
    ) -> ::core::ffi::c_int;
    pub fn ethtool_aggregate_mac_stats(
        dev: *mut net_device, mac_stats: *mut ethtool_eth_mac_stats,
    );
    pub fn ethtool_aggregate_phy_stats(
        dev: *mut net_device, phy_stats: *mut ethtool_eth_phy_stats,
    );
    pub fn ethtool_aggregate_ctrl_stats(
        dev: *mut net_device, ctrl_stats: *mut ethtool_eth_ctrl_stats,
    );
    pub fn ethtool_aggregate_pause_stats(
        dev: *mut net_device, pause_stats: *mut ethtool_pause_stats,
    );
    pub fn ethtool_aggregate_rmon_stats(
        dev: *mut net_device, rmon_stats: *mut ethtool_rmon_stats,
    );
    pub fn ethtool_dev_mm_supported(dev: *mut net_device) -> bool;
    pub fn ethnl_pse_send_ntf(dev: *mut net_device, notif: ::core::ffi::c_ulong);
}

// Fallback definitions for the disabled CONFIG_ETHTOOL_NETLINK build.
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_alloc(_phydev: *mut phy_device, _cmd: u8) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_free(_phydev: *mut phy_device) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_finished(_phydev: *mut phy_device) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_result_with_src(_phydev: *mut phy_device, _pair: u8, _result: u8, _src: u32) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_fault_length_with_src(_phydev: *mut phy_device, _pair: u8, _cm: u32, _src: u32) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_amplitude(_phydev: *mut phy_device, _pair: u8, _mV: s16) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_pulse(_phydev: *mut phy_device, _mV: u16) -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_cable_test_step(_phydev: *mut phy_device, _first: u32, _last: u32, _step: u32) -> ::core::ffi::c_int { -EOPNOTSUPP }

#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_aggregate_mac_stats(_dev: *mut net_device, _stats: *mut ethtool_eth_mac_stats) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_aggregate_phy_stats(_dev: *mut net_device, _stats: *mut ethtool_eth_phy_stats) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_aggregate_ctrl_stats(_dev: *mut net_device, _stats: *mut ethtool_eth_ctrl_stats) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_aggregate_pause_stats(_dev: *mut net_device, _stats: *mut ethtool_pause_stats) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_aggregate_rmon_stats(_dev: *mut net_device, _stats: *mut ethtool_rmon_stats) {}
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethtool_dev_mm_supported(_dev: *mut net_device) -> bool { false }
#[cfg(not(CONFIG_ETHTOOL_NETLINK))]
pub unsafe fn ethnl_pse_send_ntf(_netdev: *mut net_device, _notif: ::core::ffi::c_ulong) {}

pub unsafe fn ethnl_cable_test_result(
    phydev: *mut phy_device, pair: u8, result: u8,
) -> ::core::ffi::c_int {
    ethnl_cable_test_result_with_src(phydev, pair, result, ETHTOOL_A_CABLE_INF_SRC_TDR)
}

pub unsafe fn ethnl_cable_test_fault_length(
    phydev: *mut phy_device, pair: u8, cm: u32,
) -> ::core::ffi::c_int {
    ethnl_cable_test_fault_length_with_src(phydev, pair, cm, ETHTOOL_A_CABLE_INF_SRC_TDR)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
