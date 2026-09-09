/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OF helpers for network devices.
 */

// C header guard: __LINUX_OF_NET_H
// Dependency: <linux/phy.h>

#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_NET"))]
// Dependency: <linux/of.h>

// Dependency types supplied by <linux/phy.h> and <linux/of.h>.

#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_NET"))]
unsafe extern "C" {
    pub fn of_get_phy_mode(
        np: *mut device_node,
        interface: *mut phy_interface_t,
    ) -> i32;
    pub fn of_get_mac_address(np: *mut device_node, mac: *mut u8) -> i32;
    pub fn of_get_mac_address_nvmem(np: *mut device_node, mac: *mut u8) -> i32;
    pub fn of_get_ethdev_address(np: *mut device_node, dev: *mut net_device) -> i32;
    pub fn of_find_net_device_by_node(np: *mut device_node) -> *mut net_device;
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_NET")))]
pub unsafe fn of_get_phy_mode(
    _np: *mut device_node,
    _interface: *mut phy_interface_t,
) -> i32 {
    -ENODEV
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_NET")))]
pub unsafe fn of_get_mac_address(_np: *mut device_node, _mac: *mut u8) -> i32 {
    -ENODEV
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_NET")))]
pub unsafe fn of_get_mac_address_nvmem(_np: *mut device_node, _mac: *mut u8) -> i32 {
    -ENODEV
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_NET")))]
pub unsafe fn of_get_ethdev_address(_np: *mut device_node, _dev: *mut net_device) -> i32 {
    -ENODEV
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_NET")))]
pub unsafe fn of_find_net_device_by_node(_np: *mut device_node) -> *mut net_device {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
