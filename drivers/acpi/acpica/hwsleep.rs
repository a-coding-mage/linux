// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Name: hwsleep.c - ACPI Hardware Sleep/Wake Support functions for the
 *                   original/legacy sleep/PM registers.
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies supplied by the surrounding ACPI translation.
// This module is excluded when ACPI_REDUCED_HARDWARE is enabled.

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_legacy_sleep(sleep_state: u8) -> acpi_status {
    let sleep_type_reg_info = acpi_hw_get_bit_register_info(ACPI_BITREG_SLEEP_TYPE);
    let sleep_enable_reg_info = acpi_hw_get_bit_register_info(ACPI_BITREG_SLEEP_ENABLE);
    let mut pm1a_control: u32;
    let mut pm1b_control: u32;
    let mut in_value: u32;
    let mut status: acpi_status;

    status = acpi_write_bit_register(ACPI_BITREG_WAKE_STATUS, ACPI_CLEAR_STATUS);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_disable_all_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_hw_clear_acpi_status();
    if ACPI_FAILURE(status) {
        return status;
    }
    acpi_gbl_system_awake_and_running = FALSE;

    status = acpi_hw_enable_all_wakeup_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_register_read(ACPI_REGISTER_PM1_CONTROL, &mut pm1a_control);
    if ACPI_FAILURE(status) {
        return status;
    }

    pm1a_control &= !(sleep_type_reg_info.access_bit_mask
        | sleep_enable_reg_info.access_bit_mask);
    pm1b_control = pm1a_control;

    pm1a_control |= acpi_gbl_sleep_type_a << sleep_type_reg_info.bit_position;
    pm1b_control |= acpi_gbl_sleep_type_b << sleep_type_reg_info.bit_position;

    status = acpi_hw_write_pm1_control(pm1a_control, pm1b_control);
    if ACPI_FAILURE(status) {
        return status;
    }

    pm1a_control |= sleep_enable_reg_info.access_bit_mask;
    pm1b_control |= sleep_enable_reg_info.access_bit_mask;

    if sleep_state < ACPI_STATE_S4 {
        ACPI_FLUSH_CPU_CACHE!();
    }

    status = acpi_os_enter_sleep(sleep_state, pm1a_control, pm1b_control);
    if status == AE_CTRL_TERMINATE {
        return AE_OK;
    }
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_write_pm1_control(pm1a_control, pm1b_control);
    if ACPI_FAILURE(status) {
        return status;
    }

    if sleep_state > ACPI_STATE_S3 {
        acpi_os_stall(10 * ACPI_USEC_PER_SEC);
        status = acpi_hw_register_write(
            ACPI_REGISTER_PM1_CONTROL,
            sleep_enable_reg_info.access_bit_mask,
        );
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    loop {
        status = acpi_read_bit_register(ACPI_BITREG_WAKE_STATUS, &mut in_value);
        if ACPI_FAILURE(status) {
            return status;
        }
        if in_value != 0 {
            break;
        }
    }

    AE_OK
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_legacy_wake_prep(_sleep_state: u8) -> acpi_status {
    let mut status = AE_OK;
    if acpi_gbl_sleep_type_a_s0 != ACPI_SLEEP_TYPE_INVALID {
        let sleep_type_reg_info = acpi_hw_get_bit_register_info(ACPI_BITREG_SLEEP_TYPE);
        let sleep_enable_reg_info = acpi_hw_get_bit_register_info(ACPI_BITREG_SLEEP_ENABLE);
        let mut pm1a_control: u32 = 0;
        let mut pm1b_control: u32;

        status = acpi_hw_register_read(ACPI_REGISTER_PM1_CONTROL, &mut pm1a_control);
        if ACPI_SUCCESS(status) {
            pm1a_control &= !(sleep_type_reg_info.access_bit_mask
                | sleep_enable_reg_info.access_bit_mask);
            pm1b_control = pm1a_control;
            pm1a_control |= acpi_gbl_sleep_type_a_s0 << sleep_type_reg_info.bit_position;
            pm1b_control |= acpi_gbl_sleep_type_b_s0 << sleep_type_reg_info.bit_position;
            let _ = acpi_hw_write_pm1_control(pm1a_control, pm1b_control);
        }
    }
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_hw_legacy_wake(sleep_state: u8) -> acpi_status {
    let status: acpi_status;

    acpi_gbl_sleep_type_a = ACPI_SLEEP_TYPE_INVALID;
    acpi_hw_execute_sleep_method(METHOD_PATHNAME__SST, ACPI_SST_WAKING);

    status = acpi_hw_disable_all_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }
    status = acpi_hw_enable_all_runtime_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_hw_execute_sleep_method(METHOD_PATHNAME__WAK, sleep_state);
    let _ = acpi_write_bit_register(ACPI_BITREG_WAKE_STATUS, ACPI_CLEAR_STATUS);
    acpi_gbl_system_awake_and_running = TRUE;

    let _ = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[ACPI_EVENT_POWER_BUTTON].enable_register_id,
        ACPI_ENABLE_EVENT,
    );
    let _ = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[ACPI_EVENT_POWER_BUTTON].status_register_id,
        ACPI_CLEAR_STATUS,
    );
    let _ = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[ACPI_EVENT_SLEEP_BUTTON].enable_register_id,
        ACPI_ENABLE_EVENT,
    );
    let _ = acpi_write_bit_register(
        acpi_gbl_fixed_event_info[ACPI_EVENT_SLEEP_BUTTON].status_register_id,
        ACPI_CLEAR_STATUS,
    );

    acpi_hw_execute_sleep_method(METHOD_PATHNAME__SST, ACPI_SST_WORKING);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
