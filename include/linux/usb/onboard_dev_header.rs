/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/usb/onboard_dev.h.

#[repr(C)]
pub struct usb_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

// Equivalent build-time condition for IS_ENABLED(CONFIG_USB_ONBOARD_DEV).
#[cfg(feature = "CONFIG_USB_ONBOARD_DEV")]
extern "C" {
    pub fn onboard_dev_create_pdevs(
        parent_dev: *mut usb_device,
        pdev_list: *mut list_head,
    );
    pub fn onboard_dev_destroy_pdevs(pdev_list: *mut list_head);
}

#[cfg(not(feature = "CONFIG_USB_ONBOARD_DEV"))]
#[inline]
pub unsafe fn onboard_dev_create_pdevs(
    _parent_dev: *mut usb_device,
    _pdev_list: *mut list_head,
) {
}

#[cfg(not(feature = "CONFIG_USB_ONBOARD_DEV"))]
#[inline]
pub unsafe fn onboard_dev_destroy_pdevs(_pdev_list: *mut list_head) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
