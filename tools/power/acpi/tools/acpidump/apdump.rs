// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: apdump - Dump routines for ACPI tables (acpidump)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Dependency intent from C: #include "acpidump.h" */

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type acpi_status = u32;
pub type acpi_physical_address = u64;

pub const FALSE: u8 = 0;
pub const TRUE: u8 = 1;

#[repr(C)]
pub struct acpi_table_header {
    pub signature: [c_char; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [c_char; 6],
    pub oem_table_id: [c_char; 8],
    pub oem_revision: u32,
    pub asl_compiler_id: [c_char; 4],
    pub asl_compiler_revision: u32,
}

#[repr(C)]
pub struct acpi_table_rsdp {
    pub signature: [c_char; 8],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut gbl_summary_mode: bool;
    static mut gbl_binary_mode: bool;
    static mut gbl_verbose_mode: bool;
    static mut gbl_output_file: *mut FILE;

    static ACPI_SIG_FADT: *const c_char;
    static ACPI_SIG_MADT: *const c_char;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;

    fn ACPI_VALIDATE_RSDP_SIG(signature: *const c_char) -> bool;
    fn ACPI_COMPARE_NAMESEG(name1: *const c_char, name2: *const c_char) -> bool;
    fn ACPI_FAILURE(status: acpi_status) -> bool;
    fn ACPI_FREE(ptr: *mut c_void);

    fn acpi_ut_valid_nameseg(name: *const c_char) -> bool;
    fn acpi_tb_validate_rsdp(rsdp: *mut acpi_table_rsdp) -> acpi_status;
    fn acpi_ut_verify_checksum(table: *mut acpi_table_header, length: u32) -> acpi_status;
    fn acpi_tb_get_rsdp_length(rsdp: *mut acpi_table_rsdp) -> u32;
    fn acpi_tb_print_table_header(address: acpi_physical_address, table: *mut acpi_table_header);
    fn ap_write_to_binary_file(table: *mut acpi_table_header, instance: u32) -> c_int;
    fn acpi_ut_dump_buffer_to_file(
        file: *mut FILE,
        buffer: *mut u8,
        count: u32,
        display: u32,
        base_offset: u32,
    );
    fn acpi_os_get_table_by_index(
        index: u32,
        table: *mut *mut acpi_table_header,
        instance: *mut u32,
        address: *mut acpi_physical_address,
    ) -> acpi_status;
    fn acpi_format_exception(status: acpi_status) -> *const c_char;
    fn acpi_ut_strtoul64(string: *mut c_char, ret_integer: *mut u64) -> acpi_status;
    fn acpi_os_get_table_by_address(
        address: acpi_physical_address,
        table: *mut *mut acpi_table_header,
    ) -> acpi_status;
    fn acpi_ut_strupr(src_string: *mut c_char);
    fn acpi_os_get_table_by_name(
        signature: *mut c_char,
        instance: u32,
        table: *mut *mut acpi_table_header,
        address: *mut acpi_physical_address,
    ) -> acpi_status;
    fn ap_get_table_from_file(pathname: *mut c_char, file_size: *mut u32) -> *mut acpi_table_header;
}

pub const ACPI_NAMESEG_SIZE: usize = 4;
pub const AP_MAX_ACPI_FILES: u32 = 256;
pub const AE_LIMIT: acpi_status = 0x0000_100B;
pub const DB_BYTE_DISPLAY: u32 = 0;

unsafe fn acpi_format_uint64_high(value: acpi_physical_address) -> c_uint {
    ((value >> 32) & 0xFFFF_FFFF) as c_uint
}

unsafe fn acpi_format_uint64_low(value: acpi_physical_address) -> c_uint {
    (value & 0xFFFF_FFFF) as c_uint
}

/* Local prototypes */

unsafe fn ap_dump_table_buffer(
    table: *mut acpi_table_header,
    instance: u32,
    address: acpi_physical_address,
) -> c_int {
    let table_length: u32;

    table_length = ap_get_table_length(table);

    /* Print only the header if requested */

    if gbl_summary_mode {
        acpi_tb_print_table_header(address, table);
        return 0;
    }

    /* Dump to binary file if requested */

    if gbl_binary_mode {
        return ap_write_to_binary_file(table, instance);
    }

    /*
     * Dump the table with header for use with acpixtract utility.
     * Note: simplest to just always emit a 64-bit address. acpi_xtract
     * utility can handle this.
     */
    fprintf(
        gbl_output_file,
        c"%4.4s @ 0x%8.8X%8.8X\n".as_ptr(),
        (*table).signature.as_ptr(),
        acpi_format_uint64_high(address),
        acpi_format_uint64_low(address),
    );

    acpi_ut_dump_buffer_to_file(
        gbl_output_file,
        table as *mut u8,
        table_length,
        DB_BYTE_DISPLAY,
        0,
    );
    fprintf(gbl_output_file, c"\n".as_ptr());
    0
}

/******************************************************************************
 *
 * FUNCTION:    ap_is_valid_header
 *
 * PARAMETERS:  table               - Pointer to table to be validated
 *
 * RETURN:      TRUE if the header appears to be valid. FALSE otherwise
 *
 * DESCRIPTION: Check for a valid ACPI table header
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_is_valid_header(table: *mut acpi_table_header) -> u8 {
    if !ACPI_VALIDATE_RSDP_SIG((*table).signature.as_ptr()) {
        /* Make sure signature is all ASCII and a valid ACPI name */

        if !acpi_ut_valid_nameseg((*table).signature.as_ptr()) {
            fprintf(
                stderr,
                c"Table signature (0x%8.8X) is invalid\n".as_ptr(),
                *((*table).signature.as_ptr() as *mut u32),
            );
            return FALSE;
        }

        /* Check for minimum table length */

        if (*table).length < core::mem::size_of::<acpi_table_header>() as u32 {
            fprintf(
                stderr,
                c"Table length (0x%8.8X) is invalid\n".as_ptr(),
                (*table).length,
            );
            return FALSE;
        }
    }

    TRUE
}

/******************************************************************************
 *
 * FUNCTION:    ap_is_valid_checksum
 *
 * PARAMETERS:  table               - Pointer to table to be validated
 *
 * RETURN:      TRUE if the checksum appears to be valid. FALSE otherwise.
 *
 * DESCRIPTION: Check for a valid ACPI table checksum.
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_is_valid_checksum(table: *mut acpi_table_header) -> u8 {
    let status: acpi_status;
    let rsdp: *mut acpi_table_rsdp;

    if ACPI_VALIDATE_RSDP_SIG((*table).signature.as_ptr()) {
        /*
         * Checksum for RSDP.
         * Note: Other checksums are computed during the table dump.
         */
        rsdp = table as *mut acpi_table_rsdp;
        status = acpi_tb_validate_rsdp(rsdp);
    } else {
        /* We don't have to check for a CDAT here, since CDAT is not in the RSDT/XSDT */

        status = acpi_ut_verify_checksum(table, (*table).length);
    }

    if ACPI_FAILURE(status) {
        fprintf(
            stderr,
            c"%4.4s: Warning: wrong checksum in table\n".as_ptr(),
            (*table).signature.as_ptr(),
        );
        return FALSE;
    }

    TRUE
}

/******************************************************************************
 *
 * FUNCTION:    ap_get_table_length
 *
 * PARAMETERS:  table               - Pointer to the table
 *
 * RETURN:      Table length
 *
 * DESCRIPTION: Obtain table length according to table signature.
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_get_table_length(table: *mut acpi_table_header) -> u32 {
    let rsdp: *mut acpi_table_rsdp;

    /* Check if table is valid */

    if ap_is_valid_header(table) == 0 {
        return 0;
    }

    if ACPI_VALIDATE_RSDP_SIG((*table).signature.as_ptr()) {
        rsdp = table as *mut acpi_table_rsdp;
        return acpi_tb_get_rsdp_length(rsdp);
    }

    /* Normal ACPI table */

    (*table).length
}

/******************************************************************************
 *
 * FUNCTION:    ap_dump_all_tables
 *
 * PARAMETERS:  None
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Get all tables from the RSDT/XSDT (or at least all of the
 *              tables that we can possibly get).
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_dump_all_tables() -> c_int {
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut instance: u32 = 0;
    let mut address: acpi_physical_address = 0;
    let mut status: acpi_status;
    let mut table_status: c_int;
    let mut i: u32;

    /* Get and dump all available ACPI tables */

    i = 0;
    while i < AP_MAX_ACPI_FILES {
        status = acpi_os_get_table_by_index(i, &mut table, &mut instance, &mut address);
        if ACPI_FAILURE(status) {
            /* AE_LIMIT means that no more tables are available */

            if status == AE_LIMIT {
                return 0;
            } else if i == 0 {
                fprintf(
                    stderr,
                    c"Could not get ACPI tables, %s\n".as_ptr(),
                    acpi_format_exception(status),
                );
                return -1;
            } else {
                fprintf(
                    stderr,
                    c"Could not get ACPI table at index %u, %s\n".as_ptr(),
                    i,
                    acpi_format_exception(status),
                );
                i = i.wrapping_add(1);
                continue;
            }
        }

        table_status = ap_dump_table_buffer(table, instance, address);
        ACPI_FREE(table as *mut c_void);

        if table_status != 0 {
            break;
        }

        i = i.wrapping_add(1);
    }

    /* Something seriously bad happened if the loop terminates here */

    -1
}

/******************************************************************************
 *
 * FUNCTION:    ap_dump_table_by_address
 *
 * PARAMETERS:  ascii_address       - Address for requested ACPI table
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Get an ACPI table via a physical address and dump it.
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_dump_table_by_address(ascii_address: *mut c_char) -> c_int {
    let mut address: acpi_physical_address;
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut status: acpi_status;
    let table_status: c_int;
    let mut long_address: u64 = 0;

    /* Convert argument to an integer physical address */

    status = acpi_ut_strtoul64(ascii_address, &mut long_address);
    if ACPI_FAILURE(status) {
        fprintf(
            stderr,
            c"%s: Could not convert to a physical address\n".as_ptr(),
            ascii_address,
        );
        return -1;
    }

    address = long_address as acpi_physical_address;
    status = acpi_os_get_table_by_address(address, &mut table);
    if ACPI_FAILURE(status) {
        fprintf(
            stderr,
            c"Could not get table at 0x%8.8X%8.8X, %s\n".as_ptr(),
            acpi_format_uint64_high(address),
            acpi_format_uint64_low(address),
            acpi_format_exception(status),
        );
        return -1;
    }

    table_status = ap_dump_table_buffer(table, 0, address);
    ACPI_FREE(table as *mut c_void);
    table_status
}

/******************************************************************************
 *
 * FUNCTION:    ap_dump_table_by_name
 *
 * PARAMETERS:  signature           - Requested ACPI table signature
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Get an ACPI table via a signature and dump it. Handles
 *              multiple tables with the same signature (SSDTs).
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_dump_table_by_name(signature: *mut c_char) -> c_int {
    let mut local_signature: [c_char; ACPI_NAMESEG_SIZE + 1] = [0; ACPI_NAMESEG_SIZE + 1];
    let mut instance: u32;
    let mut table: *mut acpi_table_header = core::ptr::null_mut();
    let mut address: acpi_physical_address = 0;
    let mut status: acpi_status;
    let mut table_status: c_int;

    if strlen(signature) != ACPI_NAMESEG_SIZE {
        fprintf(
            stderr,
            c"Invalid table signature [%s]: must be exactly 4 characters\n".as_ptr(),
            signature,
        );
        return -1;
    }

    /* Table signatures are expected to be uppercase */

    strcpy(local_signature.as_mut_ptr(), signature);
    acpi_ut_strupr(local_signature.as_mut_ptr());

    /* To be friendly, handle tables whose signatures do not match the name */

    if ACPI_COMPARE_NAMESEG(local_signature.as_ptr(), c"FADT".as_ptr()) {
        strcpy(local_signature.as_mut_ptr(), ACPI_SIG_FADT);
    } else if ACPI_COMPARE_NAMESEG(local_signature.as_ptr(), c"MADT".as_ptr()) {
        strcpy(local_signature.as_mut_ptr(), ACPI_SIG_MADT);
    }

    /* Dump all instances of this signature (to handle multiple SSDTs) */

    instance = 0;
    while instance < AP_MAX_ACPI_FILES {
        status = acpi_os_get_table_by_name(
            local_signature.as_mut_ptr(),
            instance,
            &mut table,
            &mut address,
        );
        if ACPI_FAILURE(status) {
            /* AE_LIMIT means that no more tables are available */

            if status == AE_LIMIT {
                return 0;
            }

            fprintf(
                stderr,
                c"Could not get ACPI table with signature [%s], %s\n".as_ptr(),
                local_signature.as_ptr(),
                acpi_format_exception(status),
            );
            return -1;
        }

        table_status = ap_dump_table_buffer(table, instance, address);
        ACPI_FREE(table as *mut c_void);

        if table_status != 0 {
            break;
        }

        instance = instance.wrapping_add(1);
    }

    /* Something seriously bad happened if the loop terminates here */

    -1
}

/******************************************************************************
 *
 * FUNCTION:    ap_dump_table_from_file
 *
 * PARAMETERS:  pathname            - File containing the binary ACPI table
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Dump an ACPI table from a binary file
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn ap_dump_table_from_file(pathname: *mut c_char) -> c_int {
    let table: *mut acpi_table_header;
    let mut file_size: u32 = 0;
    let mut table_status: c_int = -1;

    /* Get the entire ACPI table from the file */

    table = ap_get_table_from_file(pathname, &mut file_size);
    if table.is_null() {
        return -1;
    }

    if !acpi_ut_valid_nameseg((*table).signature.as_ptr()) {
        fprintf(
            stderr,
            c"No valid ACPI signature was found in input file %s\n".as_ptr(),
            pathname,
        );
    }

    /* File must be at least as long as the table length */

    if (*table).length > file_size {
        fprintf(
            stderr,
            c"Table length (0x%X) is too large for input file (0x%X) %s\n".as_ptr(),
            (*table).length,
            file_size,
            pathname,
        );
        ACPI_FREE(table as *mut c_void);
        return table_status;
    }

    if gbl_verbose_mode {
        fprintf(
            stderr,
            c"Input file:  %s contains table [%4.4s], 0x%X (%u) bytes\n".as_ptr(),
            pathname,
            (*table).signature.as_ptr(),
            file_size,
            file_size,
        );
    }

    table_status = ap_dump_table_buffer(table, 0, 0);

    ACPI_FREE(table as *mut c_void);
    table_status
}
