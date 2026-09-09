// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: evxfevnt - External Interfaces, ACPI event disable/enable
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// This module is excluded when ACPI_REDUCED_HARDWARE is enabled.

/*
 * The types, constants, globals, and external functions referenced here are
 * supplied by the ACPI implementation.
 */

/// Transfers the system into ACPI mode.
pub unsafe fn acpi_enable() -> acpi_status {
    let mut retry: i32;

    if acpi_gbl_fadt_index == ACPI_INVALID_TABLE_INDEX {
        return AE_NO_ACPI_TABLES;
    }

    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }

    if acpi_hw_get_mode() == ACPI_SYS_MODE_ACPI {
        return AE_OK;
    }

    let status = acpi_hw_set_mode(ACPI_SYS_MODE_ACPI);
    if ACPI_FAILURE(status) {
        return status;
    }

    retry = 0;
    while retry < 30000 {
        if acpi_hw_get_mode() == ACPI_SYS_MODE_ACPI {
            return AE_OK;
        }
        acpi_os_stall(100);
        retry += 1;
    }

    AE_NO_HARDWARE_RESPONSE
}

/// Transfers the system into LEGACY (non-ACPI) mode.
pub unsafe fn acpi_disable() -> acpi_status {
    let mut status: acpi_status = AE_OK;

    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }

    if acpi_hw_get_mode() != ACPI_SYS_MODE_LEGACY {
        status = acpi_hw_set_mode(ACPI_SYS_MODE_LEGACY);
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    status
}

/// Enable an ACPI event (fixed).
pub unsafe fn acpi_enable_event(event: u32, _flags: u32) -> acpi_status {
    let mut value: u32 = 0;

    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }
    if event > ACPI_EVENT_MAX {
        return AE_BAD_PARAMETER;
    }

    let status = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[event as usize].enable_register_id,
        ACPI_ENABLE_EVENT,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    let status = acpi_read_bit_register(
        acpi_gbl_fixed_event_info[event as usize].enable_register_id,
        &mut value,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    if value != 1 {
        return AE_NO_HARDWARE_RESPONSE;
    }
    status
}

/// Disable an ACPI event (fixed).
pub unsafe fn acpi_disable_event(event: u32, _flags: u32) -> acpi_status {
    let mut value: u32 = 0;

    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }
    if event > ACPI_EVENT_MAX {
        return AE_BAD_PARAMETER;
    }

    let status = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[event as usize].enable_register_id,
        ACPI_DISABLE_EVENT,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    let status = acpi_read_bit_register(
        acpi_gbl_fixed_event_info[event as usize].enable_register_id,
        &mut value,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    if value != 0 {
        return AE_NO_HARDWARE_RESPONSE;
    }
    status
}

/// Clear an ACPI event (fixed).
pub unsafe fn acpi_clear_event(event: u32) -> acpi_status {
    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }
    if event > ACPI_EVENT_MAX {
        return AE_BAD_PARAMETER;
    }

    acpi_write_bit_register(
        acpi_gbl_fixed_event_info[event as usize].status_register_id,
        ACPI_CLEAR_STATUS,
    )
}

/// Obtains and returns the current status of the event.
pub unsafe fn acpi_get_event_status(
    event: u32,
    event_status: *mut acpi_event_status,
) -> acpi_status {
    let mut local_event_status: acpi_event_status = 0;
    let mut in_byte: u32 = 0;

    if event_status.is_null() {
        return AE_BAD_PARAMETER;
    }
    if event > ACPI_EVENT_MAX {
        return AE_BAD_PARAMETER;
    }

    if acpi_gbl_fixed_event_handlers[event as usize].handler.is_some() {
        local_event_status |= ACPI_EVENT_FLAG_HAS_HANDLER;
    }

    let status = acpi_read_bit_register(
        acpi_gbl_fixed_event_info[event as usize].enable_register_id,
        &mut in_byte,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    if in_byte != 0 {
        local_event_status |= ACPI_EVENT_FLAG_ENABLED | ACPI_EVENT_FLAG_ENABLE_SET;
    }

    let status = acpi_read_bit_register(
        acpi_gbl_fixed_event_info[event as usize].status_register_id,
        &mut in_byte,
    );
    if ACPI_FAILURE(status) {
        return status;
    }
    if in_byte != 0 {
        local_event_status |= ACPI_EVENT_FLAG_STATUS_SET;
    }

    *event_status = local_event_status;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
