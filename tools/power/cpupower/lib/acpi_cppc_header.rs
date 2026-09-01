/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub enum acpi_cppc_value {
    HIGHEST_PERF,
    LOWEST_PERF,
    NOMINAL_PERF,
    LOWEST_NONLINEAR_PERF,
    LOWEST_FREQ,
    NOMINAL_FREQ,
    REFERENCE_PERF,
    WRAPAROUND_TIME,
    MAX_CPPC_VALUE_FILES,
}

unsafe extern "C" {
    pub fn acpi_cppc_get_data(
        cpu: core::ffi::c_uint,
        which: acpi_cppc_value,
    ) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
