/* SPDX-License-Identifier: GPL-2.0 */

// Translated from pm-trace.h. The C header's linux/types.h and
// asm/pm-trace.h dependencies are supplied externally.

#[cfg(feature = "CONFIG_PM_TRACE")]
extern "C" {
    pub static mut pm_trace_enabled: core::ffi::c_int;
    pub static mut pm_trace_rtc_abused: bool;

    pub fn set_trace_device(dev: *mut device);
    pub fn generate_pm_trace(tracedata: *const core::ffi::c_void, user: core::ffi::c_uint);
    pub fn show_trace_dev_match(
        buf: *mut core::ffi::c_char,
        size: usize,
    ) -> core::ffi::c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_PM_TRACE")]
#[inline]
pub unsafe fn pm_trace_rtc_valid() -> bool {
    !pm_trace_rtc_abused
}

#[cfg(feature = "CONFIG_PM_TRACE")]
#[inline]
pub unsafe fn pm_trace_is_enabled() -> core::ffi::c_int {
    pm_trace_enabled
}

#[cfg(feature = "CONFIG_PM_TRACE")]
#[macro_export]
macro_rules! TRACE_DEVICE {
    ($dev:expr) => {{
        if unsafe { $crate::pm_trace_enabled != 0 } {
            unsafe { $crate::set_trace_device($dev) };
        }
    }};
}

#[cfg(not(feature = "CONFIG_PM_TRACE"))]
#[inline]
pub const fn pm_trace_rtc_valid() -> bool {
    true
}

#[cfg(not(feature = "CONFIG_PM_TRACE"))]
#[inline]
pub const fn pm_trace_is_enabled() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PM_TRACE"))]
#[macro_export]
macro_rules! TRACE_DEVICE {
    ($dev:expr) => {{
        let _ = &$dev;
    }};
}

#[cfg(not(feature = "CONFIG_PM_TRACE"))]
#[macro_export]
macro_rules! TRACE_RESUME {
    ($dev:expr) => {{
        let _ = &$dev;
    }};
}

#[cfg(not(feature = "CONFIG_PM_TRACE"))]
#[macro_export]
macro_rules! TRACE_SUSPEND {
    ($dev:expr) => {{
        let _ = &$dev;
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
