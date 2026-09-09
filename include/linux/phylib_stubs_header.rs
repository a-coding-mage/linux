/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Stubs for the Network PHY library
 */

// Dependency supplied by the surrounding kernel translation: rtnetlink.

pub struct EthtoolEthPhyStats;
pub struct EthtoolLinkExtStats;
pub struct EthtoolPhyStats;
pub struct KernelHwtstampConfig;
pub struct NetlinkExtAck;
pub struct PhyDevice;

// This condition corresponds to CONFIG_PHYLIB.
#[cfg(feature = "CONFIG_PHYLIB")]
extern "C" {
    pub static phylib_stubs: *const PhylibStubs;
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[repr(C)]
pub struct PhylibStubs {
    pub hwtstamp_get: Option<unsafe extern "C" fn(
        phydev: *mut PhyDevice,
        config: *mut KernelHwtstampConfig,
    ) -> i32>,
    pub hwtstamp_set: Option<unsafe extern "C" fn(
        phydev: *mut PhyDevice,
        config: *mut KernelHwtstampConfig,
        extack: *mut NetlinkExtAck,
    ) -> i32>,
    pub get_phy_stats: Option<unsafe extern "C" fn(
        phydev: *mut PhyDevice,
        phy_stats: *mut EthtoolEthPhyStats,
        phydev_stats: *mut EthtoolPhyStats,
    )>,
    pub get_link_ext_stats: Option<unsafe extern "C" fn(
        phydev: *mut PhyDevice,
        link_stats: *mut EthtoolLinkExtStats,
    )>,
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[inline]
pub unsafe fn phy_hwtstamp_get(
    phydev: *mut PhyDevice,
    config: *mut KernelHwtstampConfig,
) -> i32 {
    // phylib_register_stubs() and phylib_unregister_stubs() also run under
    // rtnl_lock().
    // ASSERT_RTNL();
    if phylib_stubs.is_null() {
        return -EOPNOTSUPP;
    }
    ((*phylib_stubs).hwtstamp_get.unwrap())(phydev, config)
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[inline]
pub unsafe fn phy_hwtstamp_set(
    phydev: *mut PhyDevice,
    config: *mut KernelHwtstampConfig,
    extack: *mut NetlinkExtAck,
) -> i32 {
    // phylib_register_stubs() and phylib_unregister_stubs() also run under
    // rtnl_lock().
    // ASSERT_RTNL();
    if phylib_stubs.is_null() {
        return -EOPNOTSUPP;
    }
    ((*phylib_stubs).hwtstamp_set.unwrap())(phydev, config, extack)
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[inline]
pub unsafe fn phy_ethtool_get_phy_stats(
    phydev: *mut PhyDevice,
    phy_stats: *mut EthtoolEthPhyStats,
    phydev_stats: *mut EthtoolPhyStats,
) {
    // ASSERT_RTNL();
    if phylib_stubs.is_null() {
        return;
    }
    ((*phylib_stubs).get_phy_stats.unwrap())(phydev, phy_stats, phydev_stats);
}

#[cfg(feature = "CONFIG_PHYLIB")]
#[inline]
pub unsafe fn phy_ethtool_get_link_ext_stats(
    phydev: *mut PhyDevice,
    link_stats: *mut EthtoolLinkExtStats,
) {
    // ASSERT_RTNL();
    if phylib_stubs.is_null() {
        return;
    }
    ((*phylib_stubs).get_link_ext_stats.unwrap())(phydev, link_stats);
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_hwtstamp_get(
    _phydev: *mut PhyDevice,
    _config: *mut KernelHwtstampConfig,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_hwtstamp_set(
    _phydev: *mut PhyDevice,
    _config: *mut KernelHwtstampConfig,
    _extack: *mut NetlinkExtAck,
) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_ethtool_get_phy_stats(
    _phydev: *mut PhyDevice,
    _phy_stats: *mut EthtoolEthPhyStats,
    _phydev_stats: *mut EthtoolPhyStats,
) {
}

#[cfg(not(feature = "CONFIG_PHYLIB"))]
#[inline]
pub unsafe fn phy_ethtool_get_link_ext_stats(
    _phydev: *mut PhyDevice,
    _link_stats: *mut EthtoolLinkExtStats,
) {
}

// Supplied by the surrounding kernel translation.
extern "C" {
    static EOPNOTSUPP: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
