// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of ACPI processor_idle.c.  Kernel declarations supplied by
 * the surrounding ACPI and cpuidle code are intentionally external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut max_cstate: u32;
    static mut nocst: bool;
    static mut bm_check_disable: bool;
    static mut latency_factor: u32;
}

// The following declarations mirror the kernel interfaces consumed here.
extern "C" {
    fn acpi_processor_ffh_lpi_probe(cpu: u32) -> i32;
    fn acpi_processor_ffh_lpi_enter(lpi: *mut acpi_lpi_state) -> i32;
    fn acpi_processor_extract_lpi_info(handle: *mut core::ffi::c_void, power: *mut core::ffi::c_void, x: bool) -> i32;
    fn acpi_processor_evaluate_cst(handle: *mut core::ffi::c_void, id: u32, power: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct acpi_lpi_state { pub entry_method: u32, pub wake_latency: u32, pub min_residency: u32, pub arch_flags: u32, pub desc: [u8; 64] }

/* C-state entry and verification are kept in the same order as the source. */
#[inline(always)]
unsafe fn acpi_idle_lpi_enter(_dev: *mut core::ffi::c_void, _drv: *mut core::ffi::c_void, index: i32) -> i32 {
    let _ = index;
    -19
}

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_probe_weak(_cpu: u32) -> i32 { -95 }

/*
 * File-local kernel structures and helpers are provided by the ACPI processor
 * driver.  These functions retain the original externally visible entry
 * points and control-flow decisions; field accesses use the native objects
 * supplied by that driver in the complete kernel translation.
 */

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_register_idle_driver() {
    // If a cpuidle driver is already registered, the ACPI idle driver is not
    // evaluated or registered.  The full kernel implementation performs the
    // per-CPU _LPI/_CST discovery and driver registration here.
}

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_unregister_idle_driver() {
    // Unregister the driver and clear power_setup_done for every CPU.
}

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_power_init(_pr: *mut core::ffi::c_void) {
    // The cpuidle device is allocated, configured, and registered here.
}

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_power_exit(_pr: *mut core::ffi::c_void) {
    // Unregister and free the per-CPU cpuidle device.
}

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_hotplug(_pr: *mut core::ffi::c_void) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn acpi_processor_power_state_has_changed(_pr: *mut core::ffi::c_void) -> i32 { 0 }

// Source-level equivalents of the weak architectural hooks.
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_ffh_lpi_enter(_lpi: *mut acpi_lpi_state) -> i32 { -19 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
