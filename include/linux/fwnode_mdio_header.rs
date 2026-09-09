/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * FWNODE helper for the MDIO (Ethernet PHY) API
 */

// Dependency supplied by the Linux PHY API.

#[cfg(feature = "CONFIG_FWNODE_MDIO")]
extern "C" {
    pub fn fwnode_mdiobus_phy_device_register(
        mdio: *mut mii_bus,
        phy: *mut phy_device,
        child: *mut fwnode_handle,
        addr: u32,
    ) -> i32;

    pub fn fwnode_mdiobus_register_phy(
        bus: *mut mii_bus,
        child: *mut fwnode_handle,
        addr: u32,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_FWNODE_MDIO"))]
pub unsafe extern "C" fn fwnode_mdiobus_phy_device_register(
    _mdio: *mut mii_bus,
    _phy: *mut phy_device,
    _child: *mut fwnode_handle,
    _addr: u32,
) -> i32 {
    -EINVAL
}

#[cfg(not(feature = "CONFIG_FWNODE_MDIO"))]
pub unsafe extern "C" fn fwnode_mdiobus_register_phy(
    _bus: *mut mii_bus,
    _child: *mut fwnode_handle,
    _addr: u32,
) -> i32 {
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
