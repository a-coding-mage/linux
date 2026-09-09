/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the surrounding kernel translation: linux/uuid.h

#[cfg(CONFIG_ACPI_PRMT)]
extern "C" {
    pub fn init_prmt();
    pub fn acpi_prm_handler_available(handler_guid: *const guid_t) -> bool;
    pub fn acpi_call_prm_handler(handler_guid: guid_t, param_buffer: *mut core::ffi::c_void) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_ACPI_PRMT))]
#[inline]
pub fn init_prmt() {}

#[cfg(not(CONFIG_ACPI_PRMT))]
#[inline]
pub fn acpi_prm_handler_available(_handler_guid: *const guid_t) -> bool {
    false
}

#[cfg(not(CONFIG_ACPI_PRMT))]
#[inline]
pub fn acpi_call_prm_handler(
    _handler_guid: guid_t,
    _param_buffer: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
