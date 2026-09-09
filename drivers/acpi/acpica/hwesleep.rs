// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Name: hwesleep.c - ACPI Hardware Sleep/Wake Support functions for the
 *                    extended FADT-V5 sleep registers.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally left external.

/* One argument, integer_argument; No return value expected */
pub unsafe fn acpi_hw_execute_sleep_method(
    method_pathname: *mut core::ffi::c_char,
    integer_argument: u32,
) {
    let mut arg_list: acpi_object_list = core::mem::zeroed();
    let mut arg: acpi_object = core::mem::zeroed();

    arg_list.count = 1;
    arg_list.pointer = &mut arg;
    arg.type_ = ACPI_TYPE_INTEGER;
    arg.integer.value = integer_argument as u64;

    let status = acpi_evaluate_object(core::ptr::null_mut(), method_pathname, &arg_list, core::ptr::null_mut());
    if ACPI_FAILURE(status) && status != AE_NOT_FOUND {
        ACPI_EXCEPTION((AE_INFO, status, "While executing method %s", method_pathname));
    }
}

pub unsafe fn acpi_hw_extended_sleep(sleep_state: u8) -> acpi_status {
    let mut sleep_status: u64 = 0;

    // Extended sleep registers must be valid
    if !acpi_gbl_FADT.sleep_control.address || !acpi_gbl_FADT.sleep_status.address {
        return AE_NOT_EXIST;
    }

    // Clear wake status (WAK_STS)
    let mut status = acpi_write(ACPI_X_WAKE_STATUS as u64, &mut acpi_gbl_FADT.sleep_status);
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_gbl_system_awake_and_running = FALSE;

    /*
     * Set the SLP_TYP and SLP_EN bits.
     *
     * Note: We only use the first value returned by the \_Sx method
     * (acpi_gbl_sleep_type_a) - As per ACPI specification.
     */
    ACPI_DEBUG_PRINT((ACPI_DB_INIT, "Entering sleep state [S%u]\n", sleep_state));
    let sleep_control: u8 = ((acpi_gbl_sleep_type_a << ACPI_X_SLEEP_TYPE_POSITION)
        & ACPI_X_SLEEP_TYPE_MASK) | ACPI_X_SLEEP_ENABLE;

    // Flush caches, as per ACPI specification
    if sleep_state < ACPI_STATE_S4 {
        ACPI_FLUSH_CPU_CACHE!();
    }

    status = acpi_os_enter_sleep(sleep_state, sleep_control, 0);
    if status == AE_CTRL_TERMINATE {
        return AE_OK;
    }
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_write(sleep_control as u64, &mut acpi_gbl_FADT.sleep_control);
    if ACPI_FAILURE(status) {
        return status;
    }

    // Wait for transition back to Working State
    loop {
        status = acpi_read(&mut sleep_status, &mut acpi_gbl_FADT.sleep_status);
        if ACPI_FAILURE(status) {
            return status;
        }
        if (sleep_status as u8 & ACPI_X_WAKE_STATUS) != 0 {
            break;
        }
    }

    AE_OK
}

pub unsafe fn acpi_hw_extended_wake_prep(_sleep_state: u8) -> acpi_status {
    let sleep_type_value: u8;

    if acpi_gbl_sleep_type_a_s0 != ACPI_SLEEP_TYPE_INVALID {
        sleep_type_value = (acpi_gbl_sleep_type_a_s0 << ACPI_X_SLEEP_TYPE_POSITION)
            & ACPI_X_SLEEP_TYPE_MASK;
        let _ = acpi_write(
            (sleep_type_value | ACPI_X_SLEEP_ENABLE) as u64,
            &mut acpi_gbl_FADT.sleep_control,
        );
    }

    AE_OK
}

pub unsafe fn acpi_hw_extended_wake(sleep_state: u8) -> acpi_status {
    // Ensure enter_sleep_state_prep -> enter_sleep_state ordering
    acpi_gbl_sleep_type_a = ACPI_SLEEP_TYPE_INVALID;

    // Execute the wake methods
    acpi_hw_execute_sleep_method(METHOD_PATHNAME__SST, ACPI_SST_WAKING);
    acpi_hw_execute_sleep_method(METHOD_PATHNAME__WAK, sleep_state);

    /*
     * Some BIOS code assumes that WAK_STS will be cleared on resume
     * and use it to determine whether the system is rebooting or
     * resuming. Clear WAK_STS for compatibility.
     */
    let _ = acpi_write(ACPI_X_WAKE_STATUS as u64, &mut acpi_gbl_FADT.sleep_status);
    acpi_gbl_system_awake_and_running = TRUE;

    acpi_hw_execute_sleep_method(METHOD_PATHNAME__SST, ACPI_SST_WORKING);
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
