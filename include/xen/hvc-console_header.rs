/* SPDX-License-Identifier: GPL-2.0 */

// Translated from xen/hvc-console.h.

/// Opaque declaration of the externally defined C `struct console` type.
#[repr(C)]
pub struct console {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut xenboot_console: console;
}

// The C header uses CONFIG_HVC_XEN as a build-time configuration condition.
#[cfg(CONFIG_HVC_XEN)]
unsafe extern "C" {
    pub fn xen_console_resume();
    pub fn xen_raw_console_write(str_: *const core::ffi::c_char);
    // C declaration carries __printf(1, 2).
    pub fn xen_raw_printk(fmt: *const core::ffi::c_char, ...);
}

#[cfg(not(CONFIG_HVC_XEN))]
#[inline]
pub fn xen_console_resume() {}

#[cfg(not(CONFIG_HVC_XEN))]
#[inline]
pub fn xen_raw_console_write(_str: *const core::ffi::c_char) {}

#[cfg(not(CONFIG_HVC_XEN))]
#[inline]
pub unsafe fn xen_raw_printk(_fmt: *const core::ffi::c_char, ...) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
