// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Name: hwxfsleep.c - ACPI Hardware Sleep/Wake External Interfaces
 *
 * Rust translation of the implementation source.
 */

// Dependencies supplied by the surrounding ACPI translation.

unsafe fn acpi_hw_set_firmware_waking_vector(
    facs: *mut acpi_table_facs,
    physical_address: acpi_physical_address,
    physical_address64: acpi_physical_address,
) -> acpi_status {
    (*facs).firmware_waking_vector = physical_address as u32;

    if (*facs).length > 32 {
        if (*facs).version >= 1 {
            (*facs).xfirmware_waking_vector = physical_address64;
        } else {
            (*facs).xfirmware_waking_vector = 0;
        }
    }

    AE_OK
}

pub unsafe fn acpi_set_firmware_waking_vector(
    physical_address: acpi_physical_address,
    physical_address64: acpi_physical_address,
) -> acpi_status {
    if !acpi_gbl_FACS.is_null() {
        let _ = acpi_hw_set_firmware_waking_vector(
            acpi_gbl_FACS,
            physical_address,
            physical_address64,
        );
    }

    AE_OK
}

// These functions are removed for the ACPI_REDUCED_HARDWARE case.
// acpi_enter_sleep_state_s4bios
#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_enter_sleep_state_s4bios() -> acpi_status {
    let mut in_value: u32 = 0;
    let mut status: acpi_status;

    status = acpi_write_bit_register(ACPI_BITREG_WAKE_STATUS, ACPI_CLEAR_STATUS);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_clear_acpi_status();
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_disable_all_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }
    acpi_gbl_system_awake_and_running = FALSE;

    status = acpi_hw_enable_all_wakeup_gpes();
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_hw_write_port(
        acpi_gbl_FADT.smi_command,
        acpi_gbl_FADT.s4_bios_request as u32,
        8,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    loop {
        acpi_os_stall(ACPI_USEC_PER_MSEC);
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

pub unsafe fn acpi_enter_sleep_state_prep(sleep_state: u8) -> acpi_status {
    let mut status: acpi_status;
    let mut arg_list: acpi_object_list;
    let mut arg: acpi_object;
    let mut sst_value: u32;

    status = acpi_get_sleep_type_data(
        sleep_state,
        &mut acpi_gbl_sleep_type_a,
        &mut acpi_gbl_sleep_type_b,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_get_sleep_type_data(
        ACPI_STATE_S0,
        &mut acpi_gbl_sleep_type_a_s0,
        &mut acpi_gbl_sleep_type_b_s0,
    );
    if ACPI_FAILURE(status) {
        acpi_gbl_sleep_type_a_s0 = ACPI_SLEEP_TYPE_INVALID;
    }

    arg_list.count = 1;
    arg_list.pointer = &mut arg;
    arg.type_ = ACPI_TYPE_INTEGER;
    arg.integer.value = sleep_state as u64;

    status = acpi_evaluate_object(core::ptr::null_mut(), METHOD_PATHNAME__PTS, &mut arg_list, core::ptr::null_mut());
    if ACPI_FAILURE(status) && status != AE_NOT_FOUND {
        return status;
    }

    sst_value = match sleep_state {
        ACPI_STATE_S0 => ACPI_SST_WORKING,
        ACPI_STATE_S1 | ACPI_STATE_S2 | ACPI_STATE_S3 => ACPI_SST_SLEEPING,
        ACPI_STATE_S4 => ACPI_SST_SLEEP_CONTEXT,
        _ => ACPI_SST_INDICATOR_OFF,
    };
    acpi_hw_execute_sleep_method(METHOD_PATHNAME__SST, sst_value);
    AE_OK
}

pub unsafe fn acpi_enter_sleep_state(sleep_state: u8) -> acpi_status {
    if acpi_gbl_sleep_type_a > ACPI_SLEEP_TYPE_MAX || acpi_gbl_sleep_type_b > ACPI_SLEEP_TYPE_MAX {
        return AE_AML_OPERAND_VALUE;
    }

    #[cfg(not(feature = "acpi_reduced_hardware"))]
    if !acpi_gbl_reduced_hardware {
        return acpi_hw_legacy_sleep(sleep_state);
    }
    acpi_hw_extended_sleep(sleep_state)
}

pub unsafe fn acpi_leave_sleep_state_prep(sleep_state: u8) -> acpi_status {
    #[cfg(not(feature = "acpi_reduced_hardware"))]
    if !acpi_gbl_reduced_hardware {
        return acpi_hw_legacy_wake_prep(sleep_state);
    }
    acpi_hw_extended_wake_prep(sleep_state)
}

pub unsafe fn acpi_leave_sleep_state(sleep_state: u8) -> acpi_status {
    #[cfg(not(feature = "acpi_reduced_hardware"))]
    if !acpi_gbl_reduced_hardware {
        return acpi_hw_legacy_wake(sleep_state);
    }
    acpi_hw_extended_wake(sleep_state)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
