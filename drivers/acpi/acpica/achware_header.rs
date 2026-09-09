/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: achware.h -- hardware specific interfaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Values for the _SST predefined method */
pub const ACPI_SST_INDICATOR_OFF: u32 = 0;
pub const ACPI_SST_WORKING: u32 = 1;
pub const ACPI_SST_WAKING: u32 = 2;
pub const ACPI_SST_SLEEPING: u32 = 3;
pub const ACPI_SST_SLEEP_CONTEXT: u32 = 4;

/*
 * hwacpi - high level functions
 */
unsafe extern "C" {
    pub fn acpi_hw_set_mode(mode: u32) -> acpi_status;
    pub fn acpi_hw_get_mode() -> u32;

    /*
     * hwregs - ACPI Register I/O
     */
    pub fn acpi_hw_validate_register(
        reg: *mut acpi_generic_address,
        max_bit_width: u8,
        address: *mut u64,
    ) -> acpi_status;
    pub fn acpi_hw_read(value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status;
    pub fn acpi_hw_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status;
    pub fn acpi_hw_get_bit_register_info(register_id: u32) -> *mut acpi_bit_register_info;
    pub fn acpi_hw_write_pm1_control(pm1a_control: u32, pm1b_control: u32) -> acpi_status;
    pub fn acpi_hw_register_read(register_id: u32, return_value: *mut u32) -> acpi_status;
    pub fn acpi_hw_register_write(register_id: u32, value: u32) -> acpi_status;
    pub fn acpi_hw_clear_acpi_status() -> acpi_status;

    /*
     * hwsleep - sleep/wake support (Legacy sleep registers)
     */
    pub fn acpi_hw_legacy_sleep(sleep_state: u8) -> acpi_status;
    pub fn acpi_hw_legacy_wake_prep(sleep_state: u8) -> acpi_status;
    pub fn acpi_hw_legacy_wake(sleep_state: u8) -> acpi_status;

    /*
     * hwesleep - sleep/wake support (Extended FADT-V5 sleep registers)
     */
    pub fn acpi_hw_execute_sleep_method(method_name: *mut u8, integer_argument: u32);
    pub fn acpi_hw_extended_sleep(sleep_state: u8) -> acpi_status;
    pub fn acpi_hw_extended_wake_prep(sleep_state: u8) -> acpi_status;
    pub fn acpi_hw_extended_wake(sleep_state: u8) -> acpi_status;

    /*
     * hwvalid - Port I/O with validation
     */
    pub fn acpi_hw_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
    pub fn acpi_hw_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
    pub fn acpi_hw_validate_io_block(address: u64, bit_width: u32, count: u32) -> acpi_status;

    /*
     * hwgpe - GPE support
     */
    pub fn acpi_hw_gpe_read(value: *mut u64, reg: *mut acpi_gpe_address) -> acpi_status;
    pub fn acpi_hw_gpe_write(value: u64, reg: *mut acpi_gpe_address) -> acpi_status;
    pub fn acpi_hw_get_gpe_register_bit(gpe_event_info: *mut acpi_gpe_event_info) -> u32;
    pub fn acpi_hw_low_set_gpe(gpe_event_info: *mut acpi_gpe_event_info, action: u32) -> acpi_status;
    pub fn acpi_hw_disable_gpe_block(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_hw_clear_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
    pub fn acpi_hw_clear_gpe_block(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
    pub fn acpi_hw_get_gpe_status(
        gpe_event_info: *mut acpi_gpe_event_info,
        event_status: *mut acpi_event_status,
    ) -> acpi_status;
    pub fn acpi_hw_enable_all_runtime_gpes() -> acpi_status;
    pub fn acpi_hw_check_all_gpes(gpe_skip_device: acpi_handle, gpe_skip_number: u32) -> u8;
    pub fn acpi_hw_enable_runtime_gpe_block(
        gpe_xrupt_info: *mut acpi_gpe_xrupt_info,
        gpe_block: *mut acpi_gpe_block_info,
        context: *mut core::ffi::c_void,
    ) -> acpi_status;
}

/* ACPI_PCI_CONFIGURED condition preserved; map this cfg to the build system's PCI setting. */
#[cfg(feature = "ACPI_PCI_CONFIGURED")]
unsafe extern "C" {
    pub fn acpi_hw_derive_pci_id(
        pci_id: *mut acpi_pci_id,
        root_pci_device: acpi_handle,
        pci_region: acpi_handle,
    ) -> acpi_status;
}

#[cfg(not(feature = "ACPI_PCI_CONFIGURED"))]
pub unsafe fn acpi_hw_derive_pci_id(
    _pci_id: *mut acpi_pci_id,
    _root_pci_device: acpi_handle,
    _pci_region: acpi_handle,
) -> acpi_status {
    AE_SUPPORT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
