/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2015 ARM Limited
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not reproduced here: linux/arm-smccc.h, linux/init.h, and linux/types.h.

pub const PSCI_POWER_STATE_TYPE_STANDBY: u32 = 0;
pub const PSCI_POWER_STATE_TYPE_POWER_DOWN: u32 = 1;

extern "C" {
    pub fn psci_tos_resident_on(cpu: core::ffi::c_int) -> bool;
    pub fn psci_cpu_suspend_enter(state: u32) -> core::ffi::c_int;
    pub fn psci_power_state_is_valid(state: u32) -> bool;
    pub fn psci_set_osi_mode(enable: bool) -> core::ffi::c_int;
    pub fn psci_has_osi_support() -> bool;
}

#[repr(C)]
pub struct psci_operations {
    pub get_version: Option<unsafe extern "C" fn() -> u32>,
    pub cpu_suspend:
        Option<unsafe extern "C" fn(state: u32, entry_point: core::ffi::c_ulong) -> core::ffi::c_int>,
    pub cpu_off: Option<unsafe extern "C" fn(state: u32) -> core::ffi::c_int>,
    pub cpu_on: Option<
        unsafe extern "C" fn(cpuid: core::ffi::c_ulong, entry_point: core::ffi::c_ulong)
            -> core::ffi::c_int,
    >,
    pub migrate: Option<unsafe extern "C" fn(cpuid: core::ffi::c_ulong) -> core::ffi::c_int>,
    pub affinity_info: Option<
        unsafe extern "C" fn(
            target_affinity: core::ffi::c_ulong,
            lowest_affinity_level: core::ffi::c_ulong,
        ) -> core::ffi::c_int,
    >,
    pub migrate_info_type: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
}

extern "C" {
    pub static mut psci_ops: psci_operations;
}

#[repr(C)]
pub struct psci_0_1_function_ids {
    pub cpu_suspend: u32,
    pub cpu_on: u32,
    pub cpu_off: u32,
    pub migrate: u32,
}

extern "C" {
    pub fn get_psci_0_1_function_ids() -> psci_0_1_function_ids;
}

// The __init annotation is a kernel build/linker attribute and is preserved by
// the surrounding integration rather than represented as a Rust ABI property.
#[cfg(feature = "CONFIG_ARM_PSCI_FW")]
extern "C" {
    pub fn psci_dt_init() -> core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_ARM_PSCI_FW"))]
#[inline]
pub fn psci_dt_init() -> core::ffi::c_int {
    0
}

#[cfg(all(feature = "CONFIG_ARM_PSCI_FW", feature = "CONFIG_ACPI"))]
extern "C" {
    pub fn psci_acpi_init() -> core::ffi::c_int;
    pub fn acpi_psci_present() -> bool;
    pub fn acpi_psci_use_hvc() -> bool;
}

#[cfg(not(all(feature = "CONFIG_ARM_PSCI_FW", feature = "CONFIG_ACPI")))]
#[inline]
pub fn psci_acpi_init() -> core::ffi::c_int {
    0
}

#[cfg(not(all(feature = "CONFIG_ARM_PSCI_FW", feature = "CONFIG_ACPI")))]
#[inline]
pub fn acpi_psci_present() -> bool {
    false
}

#[cfg(not(all(feature = "CONFIG_ARM_PSCI_FW", feature = "CONFIG_ACPI")))]
#[inline]
pub fn acpi_psci_use_hvc() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
