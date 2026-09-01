// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: getopt
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * ACPICA getopt() implementation
 *
 * Option strings:
 *    "f"       - Option has no arguments
 *    "f:"      - Option requires an argument
 *    "f+"      - Option has an optional argument
 *    "f^"      - Option has optional single-char sub-options
 *    "f|"      - Option has required single-char sub-options
 */

use core::ffi::{c_char, c_int, c_void};

// Dependencies from acpi/acpi.h, accommon.h, and acapps.h.
extern "C" {
    static ACPI_OPT_END: c_int;

    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[no_mangle]
pub static mut acpi_gbl_opterr: c_int = 1;
#[no_mangle]
pub static mut acpi_gbl_optind: c_int = 1;
#[no_mangle]
pub static mut acpi_gbl_sub_opt_char: c_int = 0;
#[no_mangle]
pub static mut acpi_gbl_optarg: *mut c_char = core::ptr::null_mut();

static mut current_char_ptr: c_int = 1;

unsafe fn acpi_option_error(msg: *const c_char, badchar: c_int) {
    if acpi_gbl_opterr != 0 {
        fprintf(stderr, b"%s%c\n\0".as_ptr() as *const c_char, msg, badchar);
    }
}

unsafe fn argv_entry(argv: *mut *mut c_char, index: c_int) -> *mut c_char {
    *argv.offset(index as isize)
}

unsafe fn argv_char(argv: *mut *mut c_char, index: c_int, char_index: c_int) -> c_char {
    *argv_entry(argv, index).offset(char_index as isize)
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_getopt_argument
 *
 * PARAMETERS:  argc, argv          - from main
 *
 * RETURN:      0 if an argument was found, -1 otherwise. Sets acpi_gbl_Optarg
 *              to point to the next argument.
 *
 * DESCRIPTION: Get the next argument. Used to obtain arguments for the
 *              two-character options after the original call to acpi_getopt.
 *              Note: Either the argument starts at the next character after
 *              the option, or it is pointed to by the next argv entry.
 *              (After call to acpi_getopt, we need to backup to the previous
 *              argv entry).
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_getopt_argument(argc: c_int, argv: *mut *mut c_char) -> c_int {
    acpi_gbl_optind -= 1;
    current_char_ptr += 1;

    if argv_char(argv, acpi_gbl_optind, current_char_ptr + 1) != b'\0' as c_char {
        acpi_gbl_optarg = argv_entry(argv, acpi_gbl_optind).offset((current_char_ptr + 1) as isize);
        acpi_gbl_optind += 1;
    } else {
        acpi_gbl_optind += 1;
        if acpi_gbl_optind >= argc {
            acpi_option_error(
                b"\nOption requires an argument\0".as_ptr() as *const c_char,
                0,
            );

            current_char_ptr = 1;
            return -1;
        } else {
            acpi_gbl_optarg = argv_entry(argv, acpi_gbl_optind);
            acpi_gbl_optind += 1;
        }
    }

    current_char_ptr = 1;
    0
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_getopt
 *
 * PARAMETERS:  argc, argv          - from main
 *              opts                - options info list
 *
 * RETURN:      Option character or ACPI_OPT_END
 *
 * DESCRIPTION: Get the next option
 *
 ******************************************************************************/

#[no_mangle]
pub unsafe extern "C" fn acpi_getopt(
    argc: c_int,
    argv: *mut *mut c_char,
    opts: *mut c_char,
) -> c_int {
    let current_char: c_int;
    let mut opts_ptr: *mut c_char;

    if current_char_ptr == 1 {
        if acpi_gbl_optind >= argc
            || argv_char(argv, acpi_gbl_optind, 0) != b'-' as c_char
            || argv_char(argv, acpi_gbl_optind, 1) == b'\0' as c_char
        {
            return ACPI_OPT_END;
        } else if strcmp(
            argv_entry(argv, acpi_gbl_optind),
            b"--\0".as_ptr() as *const c_char,
        ) == 0
        {
            acpi_gbl_optind += 1;
            return ACPI_OPT_END;
        }
    }

    /* Get the option */

    current_char = argv_char(argv, acpi_gbl_optind, current_char_ptr) as c_int;

    /* Make sure that the option is legal */

    opts_ptr = strchr(opts, current_char);
    if current_char == b':' as c_int || opts_ptr.is_null() {
        acpi_option_error(b"Illegal option: -\0".as_ptr() as *const c_char, current_char);

        current_char_ptr += 1;
        if argv_char(argv, acpi_gbl_optind, current_char_ptr) == b'\0' as c_char {
            acpi_gbl_optind += 1;
            current_char_ptr = 1;
        }

        return b'?' as c_int;
    }

    /* Option requires an argument? */

    opts_ptr = opts_ptr.offset(1);
    if *opts_ptr == b':' as c_char {
        if argv_char(argv, acpi_gbl_optind, current_char_ptr + 1) != b'\0' as c_char {
            acpi_gbl_optarg =
                argv_entry(argv, acpi_gbl_optind).offset((current_char_ptr + 1) as isize);
            acpi_gbl_optind += 1;
        } else {
            acpi_gbl_optind += 1;
            if acpi_gbl_optind >= argc {
                acpi_option_error(
                    b"Option requires an argument: -\0".as_ptr() as *const c_char,
                    current_char,
                );

                current_char_ptr = 1;
                return b'?' as c_int;
            } else {
                acpi_gbl_optarg = argv_entry(argv, acpi_gbl_optind);
                acpi_gbl_optind += 1;
            }
        }

        current_char_ptr = 1;
    }

    /* Option has an optional argument? */

    else if *opts_ptr == b'+' as c_char {
        if argv_char(argv, acpi_gbl_optind, current_char_ptr + 1) != b'\0' as c_char {
            acpi_gbl_optarg =
                argv_entry(argv, acpi_gbl_optind).offset((current_char_ptr + 1) as isize);
            acpi_gbl_optind += 1;
        } else {
            acpi_gbl_optind += 1;
            if acpi_gbl_optind >= argc {
                acpi_gbl_optarg = core::ptr::null_mut();
            } else {
                acpi_gbl_optarg = argv_entry(argv, acpi_gbl_optind);
                acpi_gbl_optind += 1;
            }
        }

        current_char_ptr = 1;
    }

    /* Option has optional single-char arguments? */

    else if *opts_ptr == b'^' as c_char {
        if argv_char(argv, acpi_gbl_optind, current_char_ptr + 1) != b'\0' as c_char {
            acpi_gbl_optarg =
                argv_entry(argv, acpi_gbl_optind).offset((current_char_ptr + 1) as isize);
        } else {
            acpi_gbl_optarg = b"^\0".as_ptr() as *mut c_char;
        }

        acpi_gbl_sub_opt_char = *acpi_gbl_optarg as c_int;
        acpi_gbl_optind += 1;
        current_char_ptr = 1;
    }

    /* Option has a required single-char argument? */

    else if *opts_ptr == b'|' as c_char {
        if argv_char(argv, acpi_gbl_optind, current_char_ptr + 1) != b'\0' as c_char {
            acpi_gbl_optarg =
                argv_entry(argv, acpi_gbl_optind).offset((current_char_ptr + 1) as isize);
        } else {
            acpi_option_error(
                b"Option requires a single-character suboption: -\0".as_ptr() as *const c_char,
                current_char,
            );

            current_char_ptr = 1;
            return b'?' as c_int;
        }

        acpi_gbl_sub_opt_char = *acpi_gbl_optarg as c_int;
        acpi_gbl_optind += 1;
        current_char_ptr = 1;
    }

    /* Option with no arguments */

    else {
        current_char_ptr += 1;
        if argv_char(argv, acpi_gbl_optind, current_char_ptr) == b'\0' as c_char {
            current_char_ptr = 1;
            acpi_gbl_optind += 1;
        }

        acpi_gbl_optarg = core::ptr::null_mut();
    }

    current_char
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
