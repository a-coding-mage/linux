// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utcksum - Support generating table checksums
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// C dependencies: acpi/acpi.h, accommon.h, and acutils.h.
// This module is used for application-level code only.

/*
 * FUNCTION:    acpi_ut_verify_checksum
 *
 * PARAMETERS:  table               - ACPI table to verify
 *              length              - Length of entire table
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Verifies that the table checksums to zero. Optionally returns
 *              exception on bad checksum.
 *              Note: We don't have to check for a CDAT here, since CDAT is
 *              not in the RSDT/XSDT, and the CDAT table is never installed
 *              via ACPICA.
 */
pub unsafe fn acpi_ut_verify_checksum(
    table: *mut acpi_table_header,
    mut length: u32,
) -> acpi_status {
    let mut checksum: u8;

    /*
     * FACS/S3PT:
     * They are the odd tables, have no standard ACPI header and no checksum
     */
    if ACPI_COMPARE_NAMESEG((*table).signature, ACPI_SIG_S3PT)
        || ACPI_COMPARE_NAMESEG((*table).signature, ACPI_SIG_FACS)
    {
        return AE_OK;
    }

    /* Compute the checksum on the table */

    length = (*table).length;
    checksum = acpi_ut_generate_checksum(
        table as *mut u8 as *mut core::ffi::c_void,
        length,
        (*table).checksum,
    );

    /* Computed checksum matches table? */

    if checksum != (*table).checksum {
        ACPI_BIOS_WARNING((
            AE_INFO,
            "Incorrect checksum in table [%4.4s] - 0x%2.2X, should be 0x%2.2X",
            (*table).signature,
            (*table).checksum,
            (*table).checksum.wrapping_sub(checksum),
        ));

        // Preserves the ACPI_CHECKSUM_ABORT build-time condition.
        #[cfg(ACPI_CHECKSUM_ABORT)]
        return AE_BAD_CHECKSUM;
    }

    AE_OK
}

/*
 * FUNCTION:    acpi_ut_verify_cdat_checksum
 *
 * PARAMETERS:  table               - CDAT ACPI table to verify
 *              length              - Length of entire table
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Verifies that the CDAT table checksums to zero. Optionally
 *              returns an exception on bad checksum.
 */
pub unsafe fn acpi_ut_verify_cdat_checksum(
    cdat_table: *mut acpi_table_cdat,
    _length: u32,
) -> acpi_status {
    let mut checksum: u8;

    /* Compute the checksum on the table */

    checksum = acpi_ut_generate_checksum(
        cdat_table as *mut u8 as *mut core::ffi::c_void,
        (*cdat_table).length,
        (*cdat_table).checksum,
    );

    /* Computed checksum matches table? */

    if checksum != (*cdat_table).checksum {
        ACPI_BIOS_WARNING((
            AE_INFO,
            "Incorrect checksum in table [%4.4s] - 0x%2.2X, should be 0x%2.2X",
            acpi_gbl_CDAT,
            (*cdat_table).checksum,
            checksum,
        ));

        // Preserves the ACPI_CHECKSUM_ABORT build-time condition.
        #[cfg(ACPI_CHECKSUM_ABORT)]
        return AE_BAD_CHECKSUM;
    }

    (*cdat_table).checksum = checksum;
    AE_OK
}

/*
 * FUNCTION:    acpi_ut_generate_checksum
 *
 * PARAMETERS:  table              - Pointer to table to be checksummed
 *              length             - Length of the table
 *              original_checksum - Value of the checksum field
 *
 * RETURN:      8 bit checksum of buffer
 *
 * DESCRIPTION: Computes an 8 bit checksum of the table.
 */
pub unsafe fn acpi_ut_generate_checksum(
    table: *mut core::ffi::c_void,
    length: u32,
    original_checksum: u8,
) -> u8 {
    /* Sum the entire table as-is */

    let mut checksum = acpi_ut_checksum(table as *mut u8, length);

    /* Subtract off the existing checksum value in the table */

    checksum = checksum.wrapping_sub(original_checksum);

    /* Compute and return the final checksum */

    checksum = 0u8.wrapping_sub(checksum);
    checksum
}

/*
 * FUNCTION:    acpi_ut_checksum
 *
 * PARAMETERS:  buffer          - Pointer to memory region to be checked
 *              length          - Length of this memory region
 *
 * RETURN:      Checksum (u8)
 *
 * DESCRIPTION: Calculates circular checksum of memory region.
 */
pub unsafe fn acpi_ut_checksum(mut buffer: *mut u8, length: u32) -> u8 {
    let mut sum: u8 = 0;
    let end = buffer.add(length as usize);

    while buffer < end {
        sum = sum.wrapping_add(*buffer);
        buffer = buffer.add(1);
    }

    sum
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
