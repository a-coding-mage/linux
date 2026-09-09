// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: tbfind   - find table
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and other translation units.

// #define _COMPONENT ACPI_TABLES
// ACPI_MODULE_NAME("tbfind")

/*******************************************************************************
 *
 * FUNCTION:    acpi_tb_find_table
 *
 * PARAMETERS:  signature           - String with ACPI table signature
 *              oem_id              - String with the table OEM ID
 *              oem_table_id        - String with the OEM Table ID
 *              table_index         - Where the table index is returned
 *
 * RETURN:      Status and table index
 *
 * DESCRIPTION: Find an ACPI table (in the RSDT/XSDT) that matches the
 *              Signature, OEM ID and OEM Table ID. Returns an index that can
 *              be used to get the table header or entire table.
 *
 ******************************************************************************/
pub unsafe fn acpi_tb_find_table(
    signature: *mut c_char,
    oem_id: *mut c_char,
    oem_table_id: *mut c_char,
    table_index: *mut u32,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut header: acpi_table_header = core::mem::zeroed();
    let mut i: u32;

    ACPI_FUNCTION_TRACE!(tb_find_table);

    /* Validate the input table signature */

    if !acpi_ut_valid_nameseg(signature) {
        return AE_BAD_SIGNATURE;
    }

    /* Don't allow the OEM strings to be too long */

    if (strlen(oem_id) > ACPI_OEM_ID_SIZE) ||
        (strlen(oem_table_id) > ACPI_OEM_TABLE_ID_SIZE)
    {
        return AE_AML_STRING_LIMIT;
    }

    /* Normalize the input strings */

    core::ptr::write_bytes(
        &mut header as *mut acpi_table_header as *mut u8,
        0,
        core::mem::size_of::<acpi_table_header>(),
    );
    ACPI_COPY_NAMESEG!(header.signature, signature);
    memcpy(header.oem_id.as_mut_ptr(), oem_id, ACPI_OEM_ID_SIZE);
    memcpy(
        header.oem_table_id.as_mut_ptr(),
        oem_table_id,
        ACPI_OEM_TABLE_ID_SIZE,
    );

    /* Search for the table */

    acpi_ut_acquire_mutex(ACPI_MTX_TABLES);
    i = 0;
    while i < acpi_gbl_root_table_list.current_table_count {
        if memcmp(
            &acpi_gbl_root_table_list.tables[i as usize].signature as *const _,
            &header.signature as *const _,
            ACPI_NAMESEG_SIZE,
        ) != 0 {
            /* Not the requested table */

            i = i.wrapping_add(1);
            continue;
        }

        /* Table with matching signature has been found */

        if acpi_gbl_root_table_list.tables[i as usize].pointer.is_null() {
            /* Table is not currently mapped, map it */

            status = acpi_tb_validate_table(&mut acpi_gbl_root_table_list.tables[i as usize]);
            if ACPI_FAILURE!(status) {
                break;
            }

            if acpi_gbl_root_table_list.tables[i as usize].pointer.is_null() {
                i = i.wrapping_add(1);
                continue;
            }
        }

        /* Check for table match on all IDs */

        let table = acpi_gbl_root_table_list.tables[i as usize].pointer;
        if memcmp((*table).signature.as_ptr(), header.signature.as_ptr(), ACPI_NAMESEG_SIZE) == 0
            && (*oem_id != 0
                || memcmp((*table).oem_id.as_ptr(), header.oem_id.as_ptr(), ACPI_OEM_ID_SIZE) == 0)
            && (*oem_table_id != 0
                || memcmp(
                    (*table).oem_table_id.as_ptr(),
                    header.oem_table_id.as_ptr(),
                    ACPI_OEM_TABLE_ID_SIZE,
                ) == 0)
        {
            *table_index = i;

            ACPI_DEBUG_PRINT!(
                (ACPI_DB_TABLES, "Found table [%4.4s]\n", header.signature),
            );
            break;
        }

        i = i.wrapping_add(1);
    }
    if i == acpi_gbl_root_table_list.current_table_count {
        status = AE_NOT_FOUND;
    }

    acpi_ut_release_mutex(ACPI_MTX_TABLES);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
