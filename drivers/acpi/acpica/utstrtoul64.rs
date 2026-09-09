// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utstrtoul64 - String-to-integer conversion support for both
 *                            64-bit and 32-bit integers
 *
 ******************************************************************************/

use core::ffi::c_char;

// The declarations below are supplied by the surrounding ACPICA translation.
extern "C" {
    static mut acpi_gbl_integer_bit_width: u8;

    fn acpi_ut_remove_whitespace(string: *mut *mut c_char) -> bool;
    fn acpi_ut_detect_hex_prefix(string: *mut *mut c_char) -> bool;
    fn acpi_ut_detect_octal_prefix(string: *mut *mut c_char) -> bool;
    fn acpi_ut_remove_hex_prefix(string: *mut *mut c_char);
    fn acpi_ut_remove_leading_zeros(string: *mut *mut c_char) -> bool;
    fn acpi_ut_convert_octal_string(string: *mut c_char, return_value: *mut u64) -> i32;
    fn acpi_ut_convert_decimal_string(string: *mut c_char, return_value: *mut u64) -> i32;
    fn acpi_ut_convert_hex_string(string: *mut c_char, return_value: *mut u64) -> i32;
}

pub type AcpiStatus = i32;
pub const AE_OK: AcpiStatus = 0;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_strtoul64
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_strtoul64(string: *mut c_char, return_value: *mut u64) -> AcpiStatus {
    let mut status: AcpiStatus = AE_OK;
    let mut original_bit_width: u8;
    let mut base: u32 = 10; // Default is decimal

    *return_value = 0;

    // A NULL return string returns a value of zero
    if *string == 0 {
        return AE_OK;
    }

    if !acpi_ut_remove_whitespace(&mut (string as *mut c_char)) {
        return AE_OK;
    }

    // 1) Check for a hex constant. A "0x" prefix indicates base 16.
    if acpi_ut_detect_hex_prefix(&mut (string as *mut c_char)) {
        base = 16;
    } else if acpi_ut_detect_octal_prefix(&mut (string as *mut c_char)) {
        // 2) Check for an octal constant, defined to be a leading zero
        // followed by sequence of octal digits (0-7)
        base = 8;
    }

    if !acpi_ut_remove_leading_zeros(&mut (string as *mut c_char)) {
        return AE_OK; // Return value 0
    }

    // Force a full 64-bit conversion.
    original_bit_width = acpi_gbl_integer_bit_width;
    acpi_gbl_integer_bit_width = 64;

    // Perform the base 8, 10, or 16 conversion.
    status = match base {
        8 => acpi_ut_convert_octal_string(string, return_value),
        10 => acpi_ut_convert_decimal_string(string, return_value),
        _ => acpi_ut_convert_hex_string(string, return_value),
    };

    acpi_gbl_integer_bit_width = original_bit_width;
    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_implicit_strtoul64
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_implicit_strtoul64(mut string: *mut c_char) -> u64 {
    let mut converted_integer: u64 = 0;

    if !acpi_ut_remove_whitespace(&mut string) {
        return 0;
    }

    // Only hexadecimal is supported; allow a "0x" prefix as an extension.
    acpi_ut_remove_hex_prefix(&mut string);

    if !acpi_ut_remove_leading_zeros(&mut string) {
        return 0;
    }

    // Ignore overflow; the input string is simply truncated.
    acpi_ut_convert_hex_string(string, &mut converted_integer);
    converted_integer
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_explicit_strtoul64
 *
 ******************************************************************************/
pub unsafe fn acpi_ut_explicit_strtoul64(mut string: *mut c_char) -> u64 {
    let mut converted_integer: u64 = 0;
    let mut base: u32 = 10; // Default is decimal

    if !acpi_ut_remove_whitespace(&mut string) {
        return 0;
    }

    // A "0x" prefix indicates hex; otherwise decimal is assumed.
    if acpi_ut_detect_hex_prefix(&mut string) {
        base = 16;
    }

    if !acpi_ut_remove_leading_zeros(&mut string) {
        return 0;
    }

    // Ignore overflow; the input string is simply truncated.
    match base {
        16 => {
            acpi_ut_convert_hex_string(string, &mut converted_integer);
        }
        10 | _ => {
            acpi_ut_convert_decimal_string(string, &mut converted_integer);
        }
    }

    converted_integer
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
