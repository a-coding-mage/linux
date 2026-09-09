// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utnonansi - Non-ansi C library functions
 *
 ******************************************************************************/

// Dependencies supplied by the ACPICA headers and C library.
use core::ffi::{c_char, c_int, c_void};

type u8 = u8;
type acpi_size = usize;

const TRUE: u8 = 1;
const FALSE: u8 = 0;

unsafe extern "C" {
    fn tolower(c: c_int) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn strlen(s: *const c_char) -> acpi_size;
    fn strcpy(dest: *mut c_char, source: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, source: *const c_char) -> *mut c_char;
    fn strncat(
        dest: *mut c_char,
        source: *const c_char,
        max_transfer_length: acpi_size,
    ) -> *mut c_char;
}

/*
 * Non-ANSI C library functions - strlwr, strupr, stricmp, and "safe"
 * string functions.
 */

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_strlwr (strlwr)
 *
 * PARAMETERS:  src_string      - The source string to convert
 *
 * RETURN:      None
 *
 * DESCRIPTION: Convert a string to lowercase
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_strlwr(src_string: *mut c_char) {
    if src_string.is_null() {
        return;
    }

    /* Walk entire string, lowercasing the letters */
    let mut string = src_string;
    while unsafe { *string } != 0 {
        unsafe {
            *string = tolower(*string as c_int) as c_char;
            string = string.add(1);
        }
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_strupr (strupr)
 *
 * PARAMETERS:  src_string      - The source string to convert
 *
 * RETURN:      None
 *
 * DESCRIPTION: Convert a string to uppercase
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_strupr(src_string: *mut c_char) {
    if src_string.is_null() {
        return;
    }

    /* Walk entire string, uppercasing the letters */
    let mut string = src_string;
    while unsafe { *string } != 0 {
        unsafe {
            *string = toupper(*string as c_int) as c_char;
            string = string.add(1);
        }
    }
}

/******************************************************************************
 *
 * FUNCTION:    acpi_ut_stricmp (stricmp)
 *
 * PARAMETERS:  string1             - first string to compare
 *              string2             - second string to compare
 *
 * RETURN:      int that signifies string relationship. Zero means strings
 *              are equal.
 *
 * DESCRIPTION: Case-insensitive string compare. Implementation of the
 *              non-ANSI stricmp function.
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_stricmp(mut string1: *mut c_char, mut string2: *mut c_char) -> c_int {
    let (c1, c2);
    loop {
        c1 = unsafe { tolower(*string1 as c_int) };
        c2 = unsafe { tolower(*string2 as c_int) };

        unsafe {
            string1 = string1.add(1);
            string2 = string2.add(1);
        }

        if c1 != c2 || c1 == 0 {
            break;
        }
    }

    c1 - c2
}

// These safe string functions are conditionally compiled in ACPICA builds
// with ACPI_DEBUGGER, ACPI_APPLICATION, or ACPI_DEBUG_OUTPUT defined.

#[cfg(any(feature = "ACPI_DEBUGGER", feature = "ACPI_APPLICATION", feature = "ACPI_DEBUG_OUTPUT"))]
pub unsafe fn acpi_ut_safe_strcpy(
    dest: *mut c_char,
    dest_size: acpi_size,
    source: *mut c_char,
) -> u8 {
    if unsafe { strlen(source) } >= dest_size {
        return TRUE;
    }

    unsafe { strcpy(dest, source) };
    FALSE
}

#[cfg(any(feature = "ACPI_DEBUGGER", feature = "ACPI_APPLICATION", feature = "ACPI_DEBUG_OUTPUT"))]
pub unsafe fn acpi_ut_safe_strcat(
    dest: *mut c_char,
    dest_size: acpi_size,
    source: *mut c_char,
) -> u8 {
    if unsafe { strlen(dest) + strlen(source) } >= dest_size {
        return TRUE;
    }

    unsafe { strcat(dest, source) };
    FALSE
}

#[cfg(any(feature = "ACPI_DEBUGGER", feature = "ACPI_APPLICATION", feature = "ACPI_DEBUG_OUTPUT"))]
pub unsafe fn acpi_ut_safe_strncat(
    dest: *mut c_char,
    dest_size: acpi_size,
    source: *mut c_char,
    max_transfer_length: acpi_size,
) -> u8 {
    let actual_transfer_length = core::cmp::min(max_transfer_length, unsafe { strlen(source) });

    if unsafe { strlen(dest) + actual_transfer_length } >= dest_size {
        return TRUE;
    }

    unsafe { strncat(dest, source, max_transfer_length) };
    FALSE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
