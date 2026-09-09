// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: tbprint - Table output utilities
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPI translation unit.

const _COMPONENT: u32 = ACPI_TABLES;
ACPI_MODULE_NAME!("ACPI_TABLES");

/* Local prototypes */
unsafe fn acpi_tb_fix_string(string: *mut c_char, length: acpi_size);

unsafe fn acpi_tb_cleanup_table_header(
    out_header: *mut acpi_table_header,
    header: *const acpi_table_header,
);

/*******************************************************************************
 *
 * FUNCTION:    acpi_tb_fix_string
 *
 * PARAMETERS:  string              - String to be repaired
 *              length              - Maximum length
 *
 * RETURN:      None
 *
 * DESCRIPTION: Replace every non-printable or non-ascii byte in the string
 *              with a question mark '?'.
 *
 ******************************************************************************/

unsafe fn acpi_tb_fix_string(string: *mut c_char, mut length: acpi_size) {
    let mut string = string;

    while length != 0 && *string != 0 {
        if libc::isprint((*string as u8) as c_int) == 0 {
            *string = b'?' as c_char;
        }

        string = string.add(1);
        length -= 1;
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_tb_cleanup_table_header
 *
 * PARAMETERS:  out_header          - Where the cleaned header is returned
 *              header              - Input ACPI table header
 *
 * RETURN:      Returns the cleaned header in out_header
 *
 * DESCRIPTION: Copy the table header and ensure that all "string" fields in
 *              the header consist of printable characters.
 *
 ******************************************************************************/

unsafe fn acpi_tb_cleanup_table_header(
    out_header: *mut acpi_table_header,
    header: *const acpi_table_header,
) {
    core::ptr::copy_nonoverlapping(
        header as *const u8,
        out_header as *mut u8,
        core::mem::size_of::<acpi_table_header>(),
    );

    acpi_tb_fix_string((*out_header).signature.as_mut_ptr(), ACPI_NAMESEG_SIZE);
    acpi_tb_fix_string((*out_header).oem_id.as_mut_ptr(), ACPI_OEM_ID_SIZE);
    acpi_tb_fix_string((*out_header).oem_table_id.as_mut_ptr(), ACPI_OEM_TABLE_ID_SIZE);
    acpi_tb_fix_string((*out_header).asl_compiler_id.as_mut_ptr(), ACPI_NAMESEG_SIZE);
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_tb_print_table_header
 *
 * PARAMETERS:  address             - Table physical address
 *              header              - Table header
 *
 * RETURN:      None
 *
 * DESCRIPTION: Print an ACPI table header. Special cases for FACS and RSDP.
 *
 ******************************************************************************/

pub unsafe fn acpi_tb_print_table_header(
    address: acpi_physical_address,
    header: *mut acpi_table_header,
) {
    let mut local_header: acpi_table_header = core::mem::zeroed();

    if ACPI_COMPARE_NAMESEG!((*header).signature.as_ptr(), ACPI_SIG_FACS) {
        /* FACS only has signature and length fields */

        ACPI_INFO!(
            "%-4.4s 0x%8.8X%8.8X %06X",
            (*header).signature.as_ptr(),
            ACPI_FORMAT_UINT64!(address),
            (*header).length
        );
    } else if ACPI_VALIDATE_RSDP_SIG!((*((header as *mut acpi_table_rsdp))).signature.as_ptr()) {
        /* RSDP has no common fields */

        core::ptr::copy_nonoverlapping(
            (*((header as *mut acpi_table_rsdp))).oem_id.as_ptr(),
            local_header.oem_id.as_mut_ptr(),
            ACPI_OEM_ID_SIZE,
        );
        acpi_tb_fix_string(local_header.oem_id.as_mut_ptr(), ACPI_OEM_ID_SIZE);

        ACPI_INFO!(
            "RSDP 0x%8.8X%8.8X %06X (v%.2d %-6.6s)",
            ACPI_FORMAT_UINT64!(address),
            if (*((header as *mut acpi_table_rsdp))).revision > 0 {
                (*((header as *mut acpi_table_rsdp))).length
            } else {
                20
            },
            (*((header as *mut acpi_table_rsdp))).revision,
            local_header.oem_id.as_ptr()
        );
    } else if acpi_gbl_CDAT && !acpi_ut_valid_nameseg((*header).signature.as_ptr()) {
        /* CDAT does not use the common ACPI table header */

        ACPI_INFO!(
            "%-4.4s 0x%8.8X%8.8X %06X",
            ACPI_SIG_CDAT,
            ACPI_FORMAT_UINT64!(address),
            (*((header as *mut acpi_table_cdat))).length
        );
    } else {
        /* Standard ACPI table with full common header */

        acpi_tb_cleanup_table_header(&mut local_header, header);

        ACPI_INFO!(
            "%-4.4s 0x%8.8X%8.8X %06X (v%.2d %-6.6s %-8.8s %08X %-4.4s %08X)",
            local_header.signature.as_ptr(),
            ACPI_FORMAT_UINT64!(address),
            local_header.length,
            local_header.revision,
            local_header.oem_id.as_ptr(),
            local_header.oem_table_id.as_ptr(),
            local_header.oem_revision,
            local_header.asl_compiler_id.as_ptr(),
            local_header.asl_compiler_revision
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
