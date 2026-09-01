// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: apmain - Main module for the acpidump utility
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// #define _DECLARE_GLOBALS
// #include "acpidump.h"

/*
 * acpidump - A portable utility for obtaining system ACPI tables and dumping
 * them in an ASCII hex format suitable for binary extraction via acpixtract.
 *
 * Obtaining the system ACPI tables is an OS-specific operation.
 *
 * This utility can be ported to any host operating system by providing a
 * module containing system-specific versions of these interfaces:
 *
 *      acpi_os_get_table_by_address
 *      acpi_os_get_table_by_index
 *      acpi_os_get_table_by_name
 *
 * See the ACPICA Reference Guide for the exact definitions of these
 * interfaces. Also, see these ACPICA source code modules for example
 * implementations:
 *
 *      source/os_specific/service_layers/oswintbl.c
 *      source/os_specific/service_layers/oslinuxtbl.c
 */

use core::ffi::{c_char, c_int, c_void};

type u32 = u32;
type u64 = u64;
type acpi_status = u32;
type FILE = c_void;

#[repr(C)]
pub struct ap_dump_action {
    pub argument: *mut c_char,
    pub to_be_done: u32,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut acpi_gbl_optarg: *mut c_char;
    static mut acpi_gbl_do_not_use_xsdt: u8;
    static mut acpi_gbl_integer_byte_width: u8;

    static mut gbl_binary_mode: u8;
    static mut gbl_dump_customized_tables: u8;
    static mut gbl_do_not_dump_xsdt: u8;
    static mut gbl_output_file: *mut FILE;
    static mut gbl_output_filename: *mut c_char;
    static mut gbl_rsdp_base: u64;
    static mut gbl_summary_mode: u8;
    static mut gbl_verbose_mode: u8;

    static mut ACPI_FILE_OUT: *mut FILE;

    fn acpi_getopt(argc: c_int, argv: *mut *mut c_char, opts: *const c_char) -> c_int;
    fn acpi_os_initialize();
    fn acpi_ut_strtoul64(string: *mut c_char, ret_integer: *mut u64) -> acpi_status;
    fn ap_open_output_file(pathname: *mut c_char) -> c_int;
    fn ap_dump_all_tables() -> c_int;
    fn ap_dump_table_by_address(argument: *mut c_char) -> c_int;
    fn ap_dump_table_by_name(argument: *mut c_char) -> c_int;
    fn ap_dump_table_from_file(argument: *mut c_char) -> c_int;
    fn cm_get_file_size(file: *mut FILE) -> u32;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

// Local prototypes

unsafe fn ap_display_usage();

unsafe fn ap_do_options(argc: c_int, argv: *mut *mut c_char) -> c_int;

unsafe fn ap_insert_action(argument: *mut c_char, to_be_done: u32) -> c_int;

// Table for deferred actions from command line options

pub static mut action_table: [ap_dump_action; AP_MAX_ACTIONS as usize] =
    [ap_dump_action {
        argument: core::ptr::null_mut(),
        to_be_done: 0,
    }; AP_MAX_ACTIONS as usize];
pub static mut current_action: u32 = 0;

const AP_UTILITY_NAME: &[u8] = b"ACPI Binary Table Dump Utility\0";
const AP_SUPPORTED_OPTIONS: &[u8] = b"?a:bc:f:hn:o:r:sv^xz\0";

/******************************************************************************
 *
 * FUNCTION:    ap_display_usage
 *
 * DESCRIPTION: Usage message for the acpi_dump utility
 *
 ******************************************************************************/

unsafe fn ap_display_usage() {
    ACPI_USAGE_HEADER!("acpidump [options]\0");

    ACPI_OPTION!("-b\0", "Dump tables to binary files\0");
    ACPI_OPTION!("-h -?\0", "This help message\0");
    ACPI_OPTION!("-o <File>\0", "Redirect output to file\0");
    ACPI_OPTION!("-r <Address>\0", "Dump tables from specified RSDP\0");
    ACPI_OPTION!("-s\0", "Print table summaries only\0");
    ACPI_OPTION!("-v\0", "Display version information\0");
    ACPI_OPTION!("-vd\0", "Display build date and time\0");
    ACPI_OPTION!("-z\0", "Verbose mode\0");

    ACPI_USAGE_TEXT!("\nTable Options:\n\0");

    ACPI_OPTION!("-a <Address>\0", "Get table via a physical address\0");
    ACPI_OPTION!("-c <on|off>\0", "Turning on/off customized table dumping\0");
    ACPI_OPTION!("-f <BinaryFile>\0", "Get table via a binary file\0");
    ACPI_OPTION!("-n <Signature>\0", "Get table via a name/signature\0");
    ACPI_OPTION!("-x\0", "Do not use but dump XSDT\0");
    ACPI_OPTION!("-x -x\0", "Do not use or dump XSDT\0");

    ACPI_USAGE_TEXT!(
        "\nInvocation without parameters dumps all available tables\nMultiple mixed instances of -a, -f, and -n are supported\n\n\0"
    );
}

/******************************************************************************
 *
 * FUNCTION:    ap_insert_action
 *
 * PARAMETERS:  argument            - Pointer to the argument for this action
 *              to_be_done          - What to do to process this action
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Add an action item to the action table
 *
 ******************************************************************************/

unsafe fn ap_insert_action(argument: *mut c_char, to_be_done: u32) -> c_int {
    // Insert action and check for table overflow

    action_table[current_action as usize].argument = argument;
    action_table[current_action as usize].to_be_done = to_be_done;

    current_action = current_action.wrapping_add(1);
    if current_action > AP_MAX_ACTIONS {
        fprintf(
            stderr,
            b"Too many table options (max %d)\n\0".as_ptr() as *const c_char,
            AP_MAX_ACTIONS,
        );
        return -1;
    }

    return 0;
}

/******************************************************************************
 *
 * FUNCTION:    ap_do_options
 *
 * PARAMETERS:  argc/argv           - Standard argc/argv
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Command line option processing. The main actions for getting
 *              and dumping tables are deferred via the action table.
 *
 *****************************************************************************/

unsafe fn ap_do_options(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut j: c_int;
    let mut status: acpi_status;

    // Command line options

    loop {
        j = acpi_getopt(argc, argv, AP_SUPPORTED_OPTIONS.as_ptr() as *const c_char);
        if j == ACPI_OPT_END {
            break;
        }

        match j {
            // Global options
            98 => {
                // 'b': Dump all input tables to binary files
                gbl_binary_mode = TRUE;
                continue;
            }

            99 => {
                // 'c': Dump customized tables
                if strcmp(acpi_gbl_optarg, b"on\0".as_ptr() as *const c_char) == 0 {
                    gbl_dump_customized_tables = TRUE;
                } else if strcmp(acpi_gbl_optarg, b"off\0".as_ptr() as *const c_char) == 0 {
                    gbl_dump_customized_tables = FALSE;
                } else {
                    fprintf(
                        stderr,
                        b"%s: Cannot handle this switch, please use on|off\n\0".as_ptr()
                            as *const c_char,
                        acpi_gbl_optarg,
                    );
                    return -1;
                }
                continue;
            }

            104 | 63 => {
                // 'h' or '?'
                ap_display_usage();
                return 1;
            }

            111 => {
                // 'o': Redirect output to a single file
                if ap_open_output_file(acpi_gbl_optarg) != 0 {
                    return -1;
                }
                continue;
            }

            114 => {
                // 'r': Dump tables from specified RSDP
                status = acpi_ut_strtoul64(acpi_gbl_optarg, &mut gbl_rsdp_base);
                if ACPI_FAILURE!(status) {
                    fprintf(
                        stderr,
                        b"%s: Could not convert to a physical address\n\0".as_ptr()
                            as *const c_char,
                        acpi_gbl_optarg,
                    );
                    return -1;
                }
                continue;
            }

            115 => {
                // 's': Print table summaries only
                gbl_summary_mode = TRUE;
                continue;
            }

            120 => {
                // 'x': Do not use XSDT
                if acpi_gbl_do_not_use_xsdt == 0 {
                    acpi_gbl_do_not_use_xsdt = TRUE;
                } else {
                    gbl_do_not_dump_xsdt = TRUE;
                }
                continue;
            }

            118 => {
                // 'v': -v: (Version): signon already emitted, just exit
                match *acpi_gbl_optarg {
                    94 => {
                        // '^': -v: (Version)
                        fprintf(stderr, ACPI_COMMON_SIGNON!(AP_UTILITY_NAME.as_ptr()));
                        return 1;
                    }

                    100 => {
                        // 'd'
                        fprintf(stderr, ACPI_COMMON_SIGNON!(AP_UTILITY_NAME.as_ptr()));
                        printf(ACPI_COMMON_BUILD_TIME!());
                        return 1;
                    }

                    _ => {
                        printf(
                            b"Unknown option: -v%s\n\0".as_ptr() as *const c_char,
                            acpi_gbl_optarg,
                        );
                        return -1;
                    }
                }
            }

            122 => {
                // 'z': Verbose mode
                gbl_verbose_mode = TRUE;
                fprintf(stderr, ACPI_COMMON_SIGNON!(AP_UTILITY_NAME.as_ptr()));
                continue;
            }

            // Table options
            97 => {
                // 'a': Get table by physical address
                if ap_insert_action(acpi_gbl_optarg, AP_DUMP_TABLE_BY_ADDRESS) != 0 {
                    return -1;
                }
            }

            102 => {
                // 'f': Get table from a file
                if ap_insert_action(acpi_gbl_optarg, AP_DUMP_TABLE_BY_FILE) != 0 {
                    return -1;
                }
            }

            110 => {
                // 'n': Get table by input name (signature)
                if ap_insert_action(acpi_gbl_optarg, AP_DUMP_TABLE_BY_NAME) != 0 {
                    return -1;
                }
            }

            _ => {
                ap_display_usage();
                return -1;
            }
        }
    }

    // If there are no actions, this means "get/dump all tables"

    if current_action == 0 {
        if ap_insert_action(core::ptr::null_mut(), AP_DUMP_ALL_TABLES) != 0 {
            return -1;
        }
    }

    return 0;
}

/******************************************************************************
 *
 * FUNCTION:    main
 *
 * PARAMETERS:  argc/argv           - Standard argc/argv
 *
 * RETURN:      Status
 *
 * DESCRIPTION: C main function for acpidump utility
 *
 ******************************************************************************/

// C conditional:
// #if !defined(_GNU_EFI) && !defined(_EDK2_EFI)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut status: c_int = 0;
    let mut action: *mut ap_dump_action;
    let mut file_size: u32;
    let mut i: u32;

    ACPI_DEBUG_INITIALIZE!(); // For debug version only
    acpi_os_initialize();
    gbl_output_file = ACPI_FILE_OUT;
    acpi_gbl_integer_byte_width = 8;

    // Process command line options

    status = ap_do_options(argc, argv);
    if status > 0 {
        return 0;
    }
    if status < 0 {
        return status;
    }

    // Get/dump ACPI table(s) as requested

    i = 0;
    while i < current_action {
        action = &mut action_table[i as usize];
        match (*action).to_be_done {
            AP_DUMP_ALL_TABLES => {
                status = ap_dump_all_tables();
            }

            AP_DUMP_TABLE_BY_ADDRESS => {
                status = ap_dump_table_by_address((*action).argument);
            }

            AP_DUMP_TABLE_BY_NAME => {
                status = ap_dump_table_by_name((*action).argument);
            }

            AP_DUMP_TABLE_BY_FILE => {
                status = ap_dump_table_from_file((*action).argument);
            }

            _ => {
                fprintf(
                    stderr,
                    b"Internal error, invalid action: 0x%X\n\0".as_ptr() as *const c_char,
                    (*action).to_be_done,
                );
                return -1;
            }
        }

        if status != 0 {
            return status;
        }

        i = i.wrapping_add(1);
    }

    if !gbl_output_filename.is_null() {
        if gbl_verbose_mode != 0 {
            // Summary for the output file

            file_size = cm_get_file_size(gbl_output_file);
            fprintf(
                stderr,
                b"Output file %s contains 0x%X (%u) bytes\n\n\0".as_ptr() as *const c_char,
                gbl_output_filename,
                file_size,
                file_size,
            );
        }

        fclose(gbl_output_file);
    }

    return status;
}
// #else
// int ACPI_SYSTEM_XFACE acpi_main(int argc, char *argv[])
// The EFI entry-point variant maps to the same translated body when those
// build-time conditions are selected by the surrounding build.
// #endif

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
