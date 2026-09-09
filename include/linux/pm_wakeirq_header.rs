/* SPDX-License-Identifier: GPL-2.0-only */
/* pm_wakeirq.h - Device wakeirq helper functions */

/* The CONFIG_PM build-time condition is preserved as a Rust cfg condition. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(CONFIG_PM)]
extern "C" {
    pub fn dev_pm_set_wake_irq(dev: *mut device, irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn dev_pm_set_dedicated_wake_irq(
        dev: *mut device,
        irq: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn dev_pm_set_dedicated_wake_irq_reverse(
        dev: *mut device,
        irq: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn dev_pm_clear_wake_irq(dev: *mut device);
    pub fn devm_pm_set_wake_irq(dev: *mut device, irq: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn dev_pm_set_wake_irq(
    _dev: *mut device,
    _irq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn dev_pm_set_dedicated_wake_irq(
    _dev: *mut device,
    _irq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn dev_pm_set_dedicated_wake_irq_reverse(
    _dev: *mut device,
    _irq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn dev_pm_clear_wake_irq(_dev: *mut device) {}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn devm_pm_set_wake_irq(
    _dev: *mut device,
    _irq: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
