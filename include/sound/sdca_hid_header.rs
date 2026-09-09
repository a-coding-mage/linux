/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 */

pub enum device {}
pub enum sdw_slave {}

pub enum sdca_entity {}
pub enum sdca_interrupt {}

/* Corresponds to IS_ENABLED(CONFIG_SND_SOC_SDCA_HID). */
#[cfg(feature = "CONFIG_SND_SOC_SDCA_HID")]
unsafe extern "C" {
    pub fn sdca_add_hid_device(interrupt: *mut sdca_interrupt) -> ::core::ffi::c_int;
    pub fn sdca_destroy_hid_device(interrupt: *mut sdca_interrupt);
    pub fn sdca_hid_process_report(interrupt: *mut sdca_interrupt) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_HID"))]
#[inline]
pub unsafe fn sdca_add_hid_device(_interrupt: *mut sdca_interrupt) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_HID"))]
#[inline]
pub unsafe fn sdca_destroy_hid_device(_interrupt: *mut sdca_interrupt) {}

#[cfg(not(feature = "CONFIG_SND_SOC_SDCA_HID"))]
#[inline]
pub unsafe fn sdca_hid_process_report(_interrupt: *mut sdca_interrupt) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
