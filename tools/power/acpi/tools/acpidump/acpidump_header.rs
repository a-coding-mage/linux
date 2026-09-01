/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Module Name: acpidump.h - Include file for acpi_dump utility
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Global variables. Defined in main.c only, externed in all other files
 *
 * C preprocessor behavior:
 * If _DECLARE_GLOBALS is defined:
 *   EXTERN is empty and INIT_GLOBAL(a,b) expands to a=b.
 * Otherwise:
 *   EXTERN expands to extern and INIT_GLOBAL(a,b) expands to a.
 */

/* Dependencies from C includes:
 * <acpi/acpi.h>
 * "accommon.h"
 * "actables.h"
 * "acapps.h"
 */

use core::ffi::{c_char, c_int};

/* Globals */

unsafe extern "C" {
    /* Initialized to FALSE when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_summary_mode: u8;
    /* Initialized to FALSE when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_verbose_mode: u8;
    /* Initialized to FALSE when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_binary_mode: u8;
    /* Initialized to TRUE when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_dump_customized_tables: u8;
    /* Initialized to FALSE when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_do_not_dump_xsdt: u8;
    /* Initialized to NULL when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_output_file: ACPI_FILE;
    /* Initialized to NULL when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_output_filename: *mut c_char;
    /* Initialized to 0 when _DECLARE_GLOBALS is defined. */
    pub static mut gbl_rsdp_base: u64;
}

/* Action table used to defer requested options */

#[repr(C)]
pub struct ap_dump_action {
    pub argument: *mut c_char,
    pub to_be_done: u32,
}

pub const AP_MAX_ACTIONS: u32 = 32;

pub const AP_DUMP_ALL_TABLES: u32 = 0;
pub const AP_DUMP_TABLE_BY_ADDRESS: u32 = 1;
pub const AP_DUMP_TABLE_BY_NAME: u32 = 2;
pub const AP_DUMP_TABLE_BY_FILE: u32 = 3;

pub const AP_MAX_ACPI_FILES: u32 = 256; /* Prevent infinite loops */

/* Minimum FADT sizes for various table addresses */

/*
 * C macros preserved from acpidump.h. These depend on the external
 * ACPI_FADT_OFFSET(field) macro and the C field tokens from struct acpi_table_fadt:
 *
 * #define MIN_FADT_FOR_DSDT   (ACPI_FADT_OFFSET (dsdt) + sizeof (u32))
 * #define MIN_FADT_FOR_FACS   (ACPI_FADT_OFFSET (facs) + sizeof (u32))
 * #define MIN_FADT_FOR_XDSDT  (ACPI_FADT_OFFSET (Xdsdt) + sizeof (u64))
 * #define MIN_FADT_FOR_XFACS  (ACPI_FADT_OFFSET (Xfacs) + sizeof (u64))
 */

/*
 * apdump - Table get/dump routines
 */
unsafe extern "C" {
    pub fn ap_dump_table_from_file(pathname: *mut c_char) -> c_int;

    pub fn ap_dump_table_by_name(signature: *mut c_char) -> c_int;

    pub fn ap_dump_table_by_address(ascii_address: *mut c_char) -> c_int;

    pub fn ap_dump_all_tables() -> c_int;

    pub fn ap_is_valid_header(table: *mut acpi_table_header) -> u8;

    pub fn ap_is_valid_checksum(table: *mut acpi_table_header) -> u8;

    pub fn ap_get_table_length(table: *mut acpi_table_header) -> u32;

    /*
     * apfiles - File I/O utilities
     */
    pub fn ap_open_output_file(pathname: *mut c_char) -> c_int;

    pub fn ap_write_to_binary_file(table: *mut acpi_table_header, instance: u32) -> c_int;

    pub fn ap_get_table_from_file(
        pathname: *mut c_char,
        file_size: *mut u32,
    ) -> *mut acpi_table_header;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
