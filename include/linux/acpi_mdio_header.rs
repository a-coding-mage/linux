/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ACPI helper for the MDIO (Ethernet PHY) API
 */

/* C header guard: __LINUX_ACPI_MDIO_H */

/* Dependency supplied by linux/phy.h. */

#[cfg(feature = "CONFIG_ACPI_MDIO")]
extern "C" {
    pub fn __acpi_mdiobus_register(
        mdio: *mut mii_bus,
        fwnode: *mut fwnode_handle,
        owner: *mut module,
    ) -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_ACPI_MDIO")]
#[inline]
pub unsafe fn acpi_mdiobus_register(
    mdio: *mut mii_bus,
    handle: *mut fwnode_handle,
) -> ::core::ffi::c_int {
    __acpi_mdiobus_register(mdio, handle, THIS_MODULE)
}

#[cfg(not(feature = "CONFIG_ACPI_MDIO"))]
#[inline]
pub unsafe fn acpi_mdiobus_register(
    mdio: *mut mii_bus,
    _fwnode: *mut fwnode_handle,
) -> ::core::ffi::c_int {
    /*
     * Fall back to mdiobus_register() function to register a bus.
     * This way, we don't have to keep compat bits around in drivers.
     */
    mdiobus_register(mdio)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
