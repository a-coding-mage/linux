// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: hwacpi - ACPI Hardware Initialization/Mode Interface
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies are supplied by the surrounding ACPI translation unit.

// Entire module is excluded when ACPI_REDUCED_HARDWARE is enabled.

/******************************************************************************
 *
 * FUNCTION:    acpi_hw_set_mode
 *
 * PARAMETERS:  mode            - SYS_MODE_ACPI or SYS_MODE_LEGACY
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Transitions the system into the requested mode.
 *
 ******************************************************************************/
pub unsafe fn acpi_hw_set_mode(mode: u32) -> acpi_status {
    let status: acpi_status;

    // ACPI_FUNCTION_TRACE(hw_set_mode);

    /* If the Hardware Reduced flag is set, machine is always in acpi mode */
    if acpi_gbl_reduced_hardware {
        return AE_OK;
    }

    /*
     * ACPI 2.0 clarified that if SMI_CMD in FADT is zero,
     * system does not support mode transition.
     */
    if acpi_gbl_FADT.smi_command == 0 {
        // ACPI_ERROR((AE_INFO, "No SMI_CMD in FADT, mode transition failed"));
        return AE_NO_HARDWARE_RESPONSE;
    }

    /*
     * ACPI 2.0 clarified the meaning of ACPI_ENABLE and ACPI_DISABLE
     * in FADT: If it is zero, enabling or disabling is not supported.
     * As old systems may have used zero for mode transition,
     * we make sure both the numbers are zero to determine these
     * transitions are not supported.
     */
    if acpi_gbl_FADT.acpi_enable == 0 && acpi_gbl_FADT.acpi_disable == 0 {
        // ACPI_ERROR((AE_INFO,
        //     "No ACPI mode transition supported in this system (enable/disable both zero)"));
        return AE_OK;
    }

    match mode {
        ACPI_SYS_MODE_ACPI => {
            /* BIOS should have disabled ALL fixed and GP events */
            status = acpi_hw_write_port(
                acpi_gbl_FADT.smi_command,
                acpi_gbl_FADT.acpi_enable as u32,
                8,
            );
            // ACPI_DEBUG_PRINT((ACPI_DB_INFO, "Attempting to enable ACPI mode\n"));
        }

        ACPI_SYS_MODE_LEGACY => {
            /*
             * BIOS should clear all fixed status bits and restore fixed event
             * enable bits to default
             */
            status = acpi_hw_write_port(
                acpi_gbl_FADT.smi_command,
                acpi_gbl_FADT.acpi_disable as u32,
                8,
            );
            // ACPI_DEBUG_PRINT((ACPI_DB_INFO,
            //     "Attempting to enable Legacy (non-ACPI) mode\n"));
        }

        _ => return AE_BAD_PARAMETER,
    }

    if ACPI_FAILURE(status) {
        // ACPI_EXCEPTION((AE_INFO, status, "Could not write ACPI mode change"));
        return status;
    }

    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_hw_get_mode
 *
 * PARAMETERS:  none
 *
 * RETURN:      SYS_MODE_ACPI or SYS_MODE_LEGACY
 *
 * DESCRIPTION: Return current operating state of system. Determined by
 *              querying the SCI_EN bit.
 *
 ******************************************************************************/
pub unsafe fn acpi_hw_get_mode() -> u32 {
    let mut value: u32 = 0;

    // ACPI_FUNCTION_TRACE(hw_get_mode);

    /* If the Hardware Reduced flag is set, machine is always in acpi mode */
    if acpi_gbl_reduced_hardware {
        return ACPI_SYS_MODE_ACPI;
    }

    /*
     * ACPI 2.0 clarified that if SMI_CMD in FADT is zero,
     * system does not support mode transition.
     */
    if acpi_gbl_FADT.smi_command == 0 {
        return ACPI_SYS_MODE_ACPI;
    }

    let status = acpi_read_bit_register(ACPI_BITREG_SCI_ENABLE, &mut value);
    if ACPI_FAILURE(status) {
        return ACPI_SYS_MODE_LEGACY;
    }

    if value != 0 {
        ACPI_SYS_MODE_ACPI
    } else {
        ACPI_SYS_MODE_LEGACY
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
