/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Module Name: acapps - common include for ACPI applications/tools
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Common info for tool signons */

pub const ACPICA_NAME: &str = "Intel ACPI Component Architecture";
pub const ACPICA_COPYRIGHT: &str = "Copyright (c) 2000 - 2026 Intel Corporation";

/* ACPI_MACHINE_WIDTH is a build-time configuration supplied by the dependencies. */
#[cfg(target_pointer_width = "64")]
pub const ACPI_WIDTH: &str = " (64-bit version)";
#[cfg(target_pointer_width = "32")]
pub const ACPI_WIDTH: &str = " (32-bit version)";
#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
pub const ACPI_WIDTH: &str = " (unknown bit width, not 32 or 64)";

/* Macros for signons and file headers */

#[macro_export]
macro_rules! ACPI_COMMON_SIGNON {
    ($utility_name:expr) => {
        "\n%s\n%s version %8.8X\n%s\n\n"
    };
}

#[macro_export]
macro_rules! ACPI_COMMON_HEADER {
    ($utility_name:expr, $prefix:expr) => {
        "%s%s\n%s%s version %8.8X%s\n%s%s\n%s\n"
    };
}

#[macro_export]
macro_rules! ACPI_COMMON_BUILD_TIME {
    () => { "Build date/time: %s %s\n" };
}

/* Macros for usage messages */

#[macro_export]
macro_rules! ACPI_USAGE_HEADER {
    ($usage:expr) => { unsafe { printf(b"Usage: %s\nOptions:\n\0".as_ptr() as *const i8, $usage); } };
}

#[macro_export]
macro_rules! ACPI_USAGE_TEXT {
    ($description:expr) => { unsafe { printf($description); } };
}

#[macro_export]
macro_rules! ACPI_OPTION {
    ($name:expr, $description:expr) => { unsafe { printf(b"  %-20s%s\n\0".as_ptr() as *const i8, $name, $description); } };
}

/* Check for unexpected exceptions */

#[macro_export]
macro_rules! ACPI_CHECK_STATUS {
    ($name:ident, $status:expr, $expected:expr) => {
        if $status != $expected {
            unsafe {
                acpi_os_printf(
                    b"Unexpected %s from %s (%s-%d)\0".as_ptr() as *const i8,
                    acpi_format_exception($status),
                    stringify!($name),
                    _acpi_module_name,
                    line!() as i32,
                );
            }
        }
    };
}

#[macro_export]
macro_rules! ACPI_CHECK_OK {
    ($name:ident, $status:expr) => { ACPI_CHECK_STATUS!($name, $status, AE_OK); };
}

pub const FILE_SUFFIX_DISASSEMBLY: &str = "dsl";
pub const FILE_SUFFIX_BINARY_TABLE: &str = ".dat"; /* Needs the dot */

/* acfileio */

extern "C" {
    pub fn ac_get_all_tables_from_file(
        filename: *mut i8,
        get_only_aml_tables: u8,
        return_list_head: *mut *mut acpi_new_table_desc,
    ) -> acpi_status;
    pub fn ac_delete_table_list(list_head: *mut acpi_new_table_desc);
    pub fn ac_is_file_binary(file: *mut FILE) -> u8;
    pub fn ac_validate_table_header(file: *mut FILE, table_offset: isize) -> acpi_status;

    pub fn acpi_getopt(argc: i32, argv: *mut *mut i8, opts: *mut i8) -> i32;
    pub fn acpi_getopt_argument(argc: i32, argv: *mut *mut i8) -> i32;

    pub static mut acpi_gbl_optind: i32;
    pub static mut acpi_gbl_opterr: i32;
    pub static mut acpi_gbl_sub_opt_char: i32;
    pub static mut acpi_gbl_optarg: *mut i8;

    pub fn cm_get_file_size(file: ACPI_FILE) -> u32;

    pub fn acpi_dm_cross_reference_namespace(
        parse_tree_root: *mut acpi_parse_object,
        namespace_root: *mut acpi_namespace_node,
        owner_id: acpi_owner_id,
    );
    pub fn acpi_dm_dump_tree(origin: *mut acpi_parse_object);
    pub fn acpi_dm_find_orphan_methods(origin: *mut acpi_parse_object);
    pub fn acpi_dm_finish_namespace_load(
        parse_tree_root: *mut acpi_parse_object,
        namespace_root: *mut acpi_namespace_node,
        owner_id: acpi_owner_id,
    );
    pub fn acpi_dm_convert_parse_objects(
        parse_tree_root: *mut acpi_parse_object,
        namespace_root: *mut acpi_namespace_node,
    );

    pub fn ad_initialize() -> acpi_status;
    pub fn fl_generate_filename(input_filename: *mut i8, suffix: *mut i8) -> *mut i8;
    pub fn fl_split_input_pathname(
        input_path: *mut i8,
        out_directory_path: *mut *mut i8,
        out_filename: *mut *mut i8,
    ) -> acpi_status;
    pub fn fl_get_file_basename(file_pathname: *mut i8) -> *mut i8;
    pub fn ad_generate_filename(prefix: *mut i8, table_id: *mut i8) -> *mut i8;
    pub fn ad_write_table(
        table: *mut acpi_table_header,
        length: u32,
        table_name: *mut i8,
        oem_table_id: *mut i8,
    );
}

/* Values for get_only_aml_tables */
pub const ACPI_GET_ONLY_AML_TABLES: u8 = 1;
pub const ACPI_GET_ALL_TABLES: u8 = 0;

/* External types and functions supplied by other translated headers. */
extern "C" {
    pub fn printf(format: *const i8, ...) -> i32;
    pub fn acpi_os_printf(format: *const i8, ...);
    pub fn acpi_format_exception(status: acpi_status) -> *const i8;
    pub static _acpi_module_name: *const i8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
