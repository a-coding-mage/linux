/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file holds USB constants and structures that are needed for
 * USB device APIs.  These are used by the USB device model, which is
 * defined in chapter 9 of the USB 2.0 specification and in the
 * Wireless USB 1.0 spec (now defunct).  Linux has several APIs in C that
 * need these:
 *
 * - the host side Linux-USB kernel driver API;
 * - the "usbfs" user space API; and
 * - the Linux "gadget" device/peripheral side driver API.
 *
 * USB 2.0 adds an additional "On The Go" (OTG) mode, which lets systems
 * act either as a USB host or as a USB device.  That means the host and
 * device side APIs benefit from working well together.
 *
 * Note all descriptors are declared '__attribute__((packed))' so that:
 *
 * [a] they never get padded, either internally (USB spec writers
 *     probably handled that) or externally;
 *
 * [b] so that accessing bigger-than-a-bytes fields will never
 *     generate bus errors on any platform, even when the location of
 *     its descriptor inside a bundle isn't "naturally aligned", and
 *
 * [c] for consistency, removing all doubt even when it appears to
 *     someone that the two other points are non-issues for that
 *     particular descriptor type.
 */

/* The declarations below use types supplied by the corresponding USB API. */

/* USB 3.2 SuperSpeed Plus phy signaling rate generation and lane count */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum usb_ssp_rate {
    USB_SSP_GEN_UNKNOWN = 0,
    USB_SSP_GEN_2x1,
    USB_SSP_GEN_1x2,
    USB_SSP_GEN_2x2,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn usb_ep_type_string(ep_type: core::ffi::c_int) -> *const core::ffi::c_char;
    pub fn usb_speed_string(speed: crate::usb_device_speed) -> *const core::ffi::c_char;
    pub fn usb_get_maximum_speed(dev: *mut device) -> crate::usb_device_speed;
    pub fn usb_get_maximum_ssp_rate(dev: *mut device) -> usb_ssp_rate;
    pub fn usb_state_string(state: crate::usb_device_state) -> *const core::ffi::c_char;
    pub fn usb_decode_interval(
        epd: *const crate::usb_endpoint_descriptor,
        speed: crate::usb_device_speed,
    ) -> core::ffi::c_uint;

    /* CONFIG_TRACING */
    pub fn usb_decode_ctrl(
        str_: *mut core::ffi::c_char,
        size: usize,
        bRequestType: u8,
        bRequest: u8,
        wValue: u16,
        wIndex: u16,
        wLength: u16,
    ) -> *const core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
