/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <linux/usb/otg.h>

// Sometimes transceivers are accessed only through e.g. ULPI.
// The CONFIG_NOP_USB_XCEIV condition is a build-time kernel configuration
// condition; the enabled declarations are preserved here under a Rust cfg.
#[cfg(feature = "CONFIG_NOP_USB_XCEIV")]
extern "C" {
    pub fn usb_phy_generic_register() -> *mut platform_device;
    pub fn usb_phy_generic_unregister(pdev: *mut platform_device);
}

#[cfg(not(feature = "CONFIG_NOP_USB_XCEIV"))]
#[inline]
pub unsafe fn usb_phy_generic_register() -> *mut platform_device {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NOP_USB_XCEIV"))]
#[inline]
pub unsafe fn usb_phy_generic_unregister(_pdev: *mut platform_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
