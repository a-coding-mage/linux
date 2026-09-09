/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Standalone EHCI usb debug driver
 *
 * Originally written by:
 *  Eric W. Biederman" <ebiederm@xmission.com> and
 *  Yinghai Lu <yhlu.kernel@gmail.com>
 *
 * Changes for early/late printk and HW errata:
 *  Jason Wessel <jason.wessel@windriver.com>
 *  Copyright (C) 2009 Wind River Systems, Inc.
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* Appendix C, Debug port ... intended for use with special "debug devices"
 * that can help if there's no serial console.  (nonstandard enumeration.)
 */
#[repr(C)]
pub struct ehci_dbg_port {
    pub control: u32,
    pub pids: u32,
    pub data03: u32,
    pub data47: u32,
    pub address: u32,
}

pub const DBGP_OWNER: u32 = 1 << 30;
pub const DBGP_ENABLED: u32 = 1 << 28;
pub const DBGP_DONE: u32 = 1 << 16;
pub const DBGP_INUSE: u32 = 1 << 10;
#[inline]
pub const fn DBGP_ERRCODE(x: u32) -> u32 { (x >> 7) & 0x07 }
pub const DBGP_ERR_BAD: u32 = 1;
pub const DBGP_ERR_SIGNAL: u32 = 2;
pub const DBGP_ERROR: u32 = 1 << 6;
pub const DBGP_GO: u32 = 1 << 5;
pub const DBGP_OUT: u32 = 1 << 4;
#[inline]
pub const fn DBGP_LEN(x: u32) -> u32 { (x >> 0) & 0x0f }
#[inline]
pub const fn DBGP_PID_GET(x: u32) -> u32 { (x >> 16) & 0xff }
#[inline]
pub const fn DBGP_PID_SET(data: u32, tok: u32) -> u32 { (data << 8) | tok }
#[inline]
pub const fn DBGP_EPADDR(dev: u32, ep: u32) -> u32 { (dev << 8) | ep }

/* CONFIG_EARLY_PRINTK_DBGP declarations. */
#[cfg(feature = "CONFIG_EARLY_PRINTK_DBGP")]
extern "C" {
    pub fn early_dbgp_init(s: *mut core::ffi::c_char) -> i32;
    pub static mut early_dbgp_console: console;
}

#[repr(C)]
pub struct console {
    _private: [u8; 0],
}

#[repr(C)]
pub struct usb_hcd {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_XEN_DOM0")]
extern "C" {
    pub fn xen_dbgp_reset_prep(hcd: *mut usb_hcd) -> i32;
    pub fn xen_dbgp_external_startup(hcd: *mut usb_hcd) -> i32;
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe fn xen_dbgp_reset_prep(_hcd: *mut usb_hcd) -> i32 {
    1 /* Shouldn't this be 0? */
}

#[cfg(not(feature = "CONFIG_XEN_DOM0"))]
#[inline]
pub unsafe fn xen_dbgp_external_startup(_hcd: *mut usb_hcd) -> i32 {
    -1
}

/* Call backs from ehci host driver to ehci debug driver */
#[cfg(feature = "CONFIG_EARLY_PRINTK_DBGP")]
extern "C" {
    pub fn dbgp_external_startup(hcd: *mut usb_hcd) -> i32;
    pub fn dbgp_reset_prep(hcd: *mut usb_hcd) -> i32;
}

#[cfg(not(feature = "CONFIG_EARLY_PRINTK_DBGP"))]
#[inline]
pub unsafe fn dbgp_reset_prep(hcd: *mut usb_hcd) -> i32 {
    xen_dbgp_reset_prep(hcd)
}

#[cfg(not(feature = "CONFIG_EARLY_PRINTK_DBGP"))]
#[inline]
pub unsafe fn dbgp_external_startup(hcd: *mut usb_hcd) -> i32 {
    xen_dbgp_external_startup(hcd)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
