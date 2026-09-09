/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Wifi Band Exclusion Interface (AMD ACPI Implementation)
 * Copyright (C) 2023 Advanced Micro Devices
 */

/* The maximum number of frequency band ranges */
pub const MAX_NUM_OF_WBRF_RANGES: usize = 11;

/* Record actions */
pub const WBRF_RECORD_ADD: u32 = 0x0;
pub const WBRF_RECORD_REMOVE: u32 = 0x1;

/**
 * struct freq_band_range - Wifi frequency band range definition
 * @start: start frequency point (in Hz)
 * @end: end frequency point (in Hz)
 */
#[repr(C)]
pub struct freq_band_range {
    pub start: u64,
    pub end: u64,
}

/**
 * struct wbrf_ranges_in_out - wbrf ranges info
 * @num_of_ranges: total number of band ranges in this struct
 * @band_list: array of Wifi band ranges
 */
#[repr(C)]
pub struct wbrf_ranges_in_out {
    pub num_of_ranges: u64,
    pub band_list: [freq_band_range; MAX_NUM_OF_WBRF_RANGES],
}

/**
 * enum wbrf_notifier_actions - wbrf notifier actions index
 * @WBRF_CHANGED: there was some frequency band updates. The consumers
 *               should retrieve the latest active frequency bands.
 */
#[repr(C)]
#[derive Copy, Clone, PartialEq, Eq)]
pub enum wbrf_notifier_actions {
    WBRF_CHANGED = 0,
}

/* CONFIG_AMD_WBRF is a build-time condition; select the corresponding API. */
#[cfg(feature = "CONFIG_AMD_WBRF")]
extern "C" {
    pub fn acpi_amd_wbrf_supported_producer(dev: *mut crate::device::device) -> bool;
    pub fn acpi_amd_wbrf_add_remove(
        dev: *mut crate::device::device,
        action: u8,
        input: *mut wbrf_ranges_in_out,
    ) -> i32;
    pub fn acpi_amd_wbrf_supported_consumer(dev: *mut crate::device::device) -> bool;
    pub fn amd_wbrf_retrieve_freq_band(
        dev: *mut crate::device::device,
        output: *mut wbrf_ranges_in_out,
    ) -> i32;
    pub fn amd_wbrf_register_notifier(nb: *mut crate::notifier::notifier_block) -> i32;
    pub fn amd_wbrf_unregister_notifier(nb: *mut crate::notifier::notifier_block) -> i32;
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn acpi_amd_wbrf_supported_consumer(_dev: *mut crate::device::device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn acpi_amd_wbrf_add_remove(
    _dev: *mut crate::device::device,
    _action: u8,
    _input: *mut wbrf_ranges_in_out,
) -> i32 {
    -crate::ENODEV
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn acpi_amd_wbrf_supported_producer(_dev: *mut crate::device::device) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn amd_wbrf_retrieve_freq_band(
    _dev: *mut crate::device::device,
    _output: *mut wbrf_ranges_in_out,
) -> i32 {
    -crate::ENODEV
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn amd_wbrf_register_notifier(_nb: *mut crate::notifier::notifier_block) -> i32 {
    -crate::ENODEV
}

#[cfg(not(feature = "CONFIG_AMD_WBRF"))]
#[inline]
pub unsafe fn amd_wbrf_unregister_notifier(_nb: *mut crate::notifier::notifier_block) -> i32 {
    -crate::ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
