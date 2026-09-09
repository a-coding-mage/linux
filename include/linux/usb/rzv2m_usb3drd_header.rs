/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

// External types supplied by the Linux kernel headers.
pub enum device {}
pub enum reset_control {}

#[repr(C)]
pub struct rzv2m_usb3drd {
    pub reg: *mut c_void,
    pub drd_irq: i32,
    pub dev: *mut device,
    pub drd_rstc: *mut reset_control,
}

// Equivalent to IS_ENABLED(CONFIG_USB_RZV2M_USB3DRD); map the kernel
// configuration symbol to the Rust build configuration when integrating.
#[cfg(feature = "CONFIG_USB_RZV2M_USB3DRD")]
unsafe extern "C" {
    pub fn rzv2m_usb3drd_reset(dev: *mut device, host: bool);
}

#[cfg(not(feature = "CONFIG_USB_RZV2M_USB3DRD"))]
#[inline]
pub unsafe fn rzv2m_usb3drd_reset(_dev: *mut device, _host: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
