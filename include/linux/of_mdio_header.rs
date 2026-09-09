/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OF helpers for the MDIO (Ethernet PHY) API
 *
 * Copyright (c) 2009 Secret Lab Technologies, Ltd.
 */

/* Dependencies supplied by the surrounding Linux Rust translation. */

#[cfg(feature = "CONFIG_OF_MDIO")]
extern "C" {
    pub fn of_mdiobus_child_is_phy(child: *mut device_node) -> bool;
    pub fn __of_mdiobus_register(
        mdio: *mut mii_bus,
        np: *mut device_node,
        owner: *mut module,
    ) -> i32;
    pub fn __devm_of_mdiobus_register(
        dev: *mut device,
        mdio: *mut mii_bus,
        np: *mut device_node,
        owner: *mut module,
    ) -> i32;
    pub fn of_mdio_find_device(np: *mut device_node) -> *mut mdio_device;
    pub fn of_phy_find_device(phy_np: *mut device_node) -> *mut phy_device;
    pub fn of_phy_connect(
        dev: *mut net_device,
        phy_np: *mut device_node,
        hndlr: Option<unsafe extern "C" fn(*mut net_device)>,
        flags: u32,
        iface: phy_interface_t,
    ) -> *mut phy_device;
    pub fn of_phy_get_and_connect(
        dev: *mut net_device,
        np: *mut device_node,
        hndlr: Option<unsafe extern "C" fn(*mut net_device)>,
    ) -> *mut phy_device;
    pub fn of_mdio_find_bus(mdio_np: *mut device_node) -> *mut mii_bus;
    pub fn of_phy_register_fixed_link(np: *mut device_node) -> i32;
    pub fn of_phy_deregister_fixed_link(np: *mut device_node);
    pub fn of_phy_is_fixed_link(np: *mut device_node) -> bool;
    pub fn of_mdiobus_phy_device_register(
        mdio: *mut mii_bus,
        phy: *mut phy_device,
        child: *mut device_node,
        addr: u32,
    ) -> i32;
}

#[cfg(feature = "CONFIG_OF_MDIO")]
#[inline]
pub unsafe fn of_mdiobus_register(mdio: *mut mii_bus, np: *mut device_node) -> i32 {
    __of_mdiobus_register(mdio, np, THIS_MODULE)
}

#[cfg(feature = "CONFIG_OF_MDIO")]
#[inline]
pub unsafe fn devm_of_mdiobus_register(
    dev: *mut device,
    mdio: *mut mii_bus,
    np: *mut device_node,
) -> i32 {
    __devm_of_mdiobus_register(dev, mdio, np, THIS_MODULE)
}

#[cfg(feature = "CONFIG_OF_MDIO")]
#[inline]
pub unsafe fn of_mdio_parse_addr(dev: *mut device, np: *const device_node) -> i32 {
    let mut addr: u32 = 0;
    let ret = of_property_read_u32(np, b"reg\0".as_ptr() as *const i8, &mut addr);
    if ret < 0 {
        dev_err(dev, b"%s has invalid PHY address\n\0".as_ptr() as *const i8, (*np).full_name);
        return ret;
    }

    /* A PHY must have a reg property in the range [0-31] */
    if addr >= PHY_MAX_ADDR {
        dev_err(
            dev,
            b"%s PHY address %i is too large\n\0".as_ptr() as *const i8,
            (*np).full_name,
            addr,
        );
        return -EINVAL;
    }

    addr as i32
}

/* CONFIG_OF_MDIO disabled: provide the non-DT fallbacks. */
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdiobus_child_is_phy(_child: *mut device_node) -> bool { false }

#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdiobus_register(mdio: *mut mii_bus, _np: *mut device_node) -> i32 {
    /* Fall back to the non-DT function to register a bus. */
    mdiobus_register(mdio)
}

#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn devm_of_mdiobus_register(
    dev: *mut device,
    mdio: *mut mii_bus,
    _np: *mut device_node,
) -> i32 { devm_mdiobus_register(dev, mdio) }

#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdio_find_device(_np: *mut device_node) -> *mut mdio_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_find_device(_phy_np: *mut device_node) -> *mut phy_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_connect(
    _dev: *mut net_device, _phy_np: *mut device_node,
    _hndlr: Option<unsafe extern "C" fn(*mut net_device)>,
    _flags: u32, _iface: phy_interface_t,
) -> *mut phy_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_get_and_connect(
    _dev: *mut net_device, _np: *mut device_node,
    _hndlr: Option<unsafe extern "C" fn(*mut net_device)>,
) -> *mut phy_device { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdio_find_bus(_mdio_np: *mut device_node) -> *mut mii_bus { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdio_parse_addr(_dev: *mut device, _np: *const device_node) -> i32 { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_register_fixed_link(_np: *mut device_node) -> i32 { -ENOSYS }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_deregister_fixed_link(_np: *mut device_node) {}
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_phy_is_fixed_link(_np: *mut device_node) -> bool { false }
#[cfg(not(feature = "CONFIG_OF_MDIO"))]
#[inline]
pub unsafe fn of_mdiobus_phy_device_register(
    _mdio: *mut mii_bus, _phy: *mut phy_device,
    _child: *mut device_node, _addr: u32,
) -> i32 { -ENOSYS }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
