/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2005-2007 Takahiro Hirofuchi
 */

/* C header dependencies: <libudev.h>, <stdint.h>, "usbip_common.h" */

pub const USBIP_VHCI_BUS_TYPE: &str = "platform";
pub const USBIP_VHCI_DEVICE_NAME: &str = "vhci_hcd.0";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hub_speed {
    HUB_SPEED_HIGH = 0,
    HUB_SPEED_SUPER = 1,
}

#[repr(C)]
pub struct usbip_imported_device {
    pub hub: hub_speed,
    pub port: u8,
    pub status: u32,

    pub devid: u32,

    pub busnum: u8,
    pub devnum: u8,

    /* usbip_class_device list */
    pub udev: usbip_usb_device,
}

#[repr(C)]
pub struct usbip_vhci_driver {
    /* /sys/devices/platform/vhci_hcd */
    pub hc_device: *mut udev_device,

    pub ncontrollers: core::ffi::c_int,
    pub nports: core::ffi::c_int,
    pub idev: [usbip_imported_device; 0],
}

unsafe extern "C" {
    pub static mut vhci_driver: *mut usbip_vhci_driver;

    pub fn usbip_vhci_driver_open() -> core::ffi::c_int;
    pub fn usbip_vhci_driver_close();

    pub fn usbip_vhci_refresh_device_list() -> core::ffi::c_int;

    pub fn usbip_vhci_get_free_port(speed: u32) -> core::ffi::c_int;
    pub fn usbip_vhci_attach_device2(
        port: u8,
        sockfd: core::ffi::c_int,
        devid: u32,
        speed: u32,
    ) -> core::ffi::c_int;

    /* will be removed */
    pub fn usbip_vhci_attach_device(
        port: u8,
        sockfd: core::ffi::c_int,
        busnum: u8,
        devnum: u8,
        speed: u32,
    ) -> core::ffi::c_int;

    pub fn usbip_vhci_detach_device(port: u8) -> core::ffi::c_int;

    pub fn usbip_vhci_imported_device_dump(idev: *mut usbip_imported_device) -> core::ffi::c_int;
}
