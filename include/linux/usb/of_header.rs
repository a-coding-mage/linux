// SPDX-License-Identifier: GPL-2.0
/*
 * OF helpers for usb devices.
 */

// C dependencies: linux/usb.h, linux/usb/ch9.h, linux/usb/otg.h,
// linux/usb/phy.h

pub struct usb_device;

// Equivalent to: #if IS_ENABLED(CONFIG_OF)
#[cfg(feature = "CONFIG_OF")]
extern "C" {
    pub fn of_usb_get_dr_mode_by_phy(np: *mut device_node, arg0: ::core::ffi::c_int) -> usb_dr_mode;
    pub fn of_usb_host_tpl_support(np: *mut device_node) -> bool;
    pub fn of_usb_update_otg_caps(np: *mut device_node, otg_caps: *mut usb_otg_caps) -> ::core::ffi::c_int;
    pub fn usb_of_get_connect_type(hub: *const usb_device, port1: ::core::ffi::c_int) -> usb_port_connect_type;
    pub fn usb_of_get_device_node(hub: *mut usb_device, port1: ::core::ffi::c_int) -> *mut device_node;
    pub fn usb_of_has_combined_node(udev: *mut usb_device) -> bool;
    pub fn usb_of_get_interface_node(
        udev: *mut usb_device,
        config: u8,
        ifnum: u8,
    ) -> *mut device_node;
    pub fn usb_of_get_companion_dev(dev: *mut device) -> *mut device;
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_usb_get_dr_mode_by_phy(_np: *mut device_node, _arg0: ::core::ffi::c_int) -> usb_dr_mode {
    USB_DR_MODE_UNKNOWN
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_usb_host_tpl_support(_np: *mut device_node) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn of_usb_update_otg_caps(_np: *mut device_node, _otg_caps: *mut usb_otg_caps) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn usb_of_get_connect_type(_hub: *const usb_device, _port1: ::core::ffi::c_int) -> usb_port_connect_type {
    USB_PORT_CONNECT_TYPE_UNKNOWN
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn usb_of_get_device_node(_hub: *mut usb_device, _port1: ::core::ffi::c_int) -> *mut device_node {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn usb_of_has_combined_node(_udev: *mut usb_device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn usb_of_get_interface_node(_udev: *mut usb_device, _config: u8, _ifnum: u8) -> *mut device_node {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_OF"))]
#[inline]
pub unsafe fn usb_of_get_companion_dev(_dev: *mut device) -> *mut device {
    core::ptr::null_mut()
}

// Equivalent to: #if IS_ENABLED(CONFIG_OF) && IS_ENABLED(CONFIG_USB_SUPPORT)
#[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_USB_SUPPORT"))]
extern "C" {
    pub fn of_usb_get_phy_mode(np: *mut device_node) -> usb_phy_interface;
}

#[cfg(not(all(feature = "CONFIG_OF", feature = "CONFIG_USB_SUPPORT")))]
#[inline]
pub unsafe fn of_usb_get_phy_mode(_np: *mut device_node) -> usb_phy_interface {
    USBPHY_INTERFACE_MODE_UNKNOWN
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
