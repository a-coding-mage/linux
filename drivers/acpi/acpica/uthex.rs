// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: uthex -- Hex/ASCII support functions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and other translation units.
use core::ffi::c_char;

// #define _COMPONENT ACPI_COMPILER
// ACPI_MODULE_NAME("uthex")

/* Hex to ASCII conversion table */
static ACPI_GBL_HEX_TO_ASCII: [c_char; 16] = [
    b'0' as c_char,
    b'1' as c_char,
    b'2' as c_char,
    b'3' as c_char,
    b'4' as c_char,
    b'5' as c_char,
    b'6' as c_char,
    b'7' as c_char,
    b'8' as c_char,
    b'9' as c_char,
    b'A' as c_char,
    b'B' as c_char,
    b'C' as c_char,
    b'D' as c_char,
    b'E' as c_char,
    b'F' as c_char,
];

extern "C" {
    fn acpi_ut_short_shift_right(integer: u64, position: u32, result: *mut u64);
    fn isxdigit(c: i32) -> i32;
    fn acpi_ut_ascii_char_to_hex(hex_char: i32) -> u8;
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_hex_to_ascii_char
 *
 * PARAMETERS:  integer             - Contains the hex digit
 *              position            - bit position of the digit within the
 *                                    integer (multiple of 4)
 *
 * RETURN:      The converted Ascii character
 *
 * DESCRIPTION: Convert a hex digit to an Ascii character
 *
 ******************************************************************************/

pub unsafe fn acpi_ut_hex_to_ascii_char(integer: u64, position: u32) -> c_char {
    let mut index: u64 = 0;

    acpi_ut_short_shift_right(integer, position, &mut index);
    ACPI_GBL_HEX_TO_ASCII[(index & 0xF) as usize]
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_ascii_to_hex_byte
 *
 * PARAMETERS:  two_ascii_chars             - Pointer to two ASCII characters
 *              return_byte                 - Where converted byte is returned
 *
 * RETURN:      Status and converted hex byte
 *
 * DESCRIPTION: Perform ascii-to-hex translation, exactly two ASCII characters
 *              to a single converted byte value.
 *
 ******************************************************************************/

pub unsafe fn acpi_ut_ascii_to_hex_byte(
    two_ascii_chars: *mut c_char,
    return_byte: *mut u8,
) -> acpi_status {
    /* Both ASCII characters must be valid hex digits */

    if isxdigit((*two_ascii_chars).into()) == 0 || isxdigit((*two_ascii_chars.add(1)).into()) == 0 {
        return AE_BAD_HEX_CONSTANT;
    }

    *return_byte = acpi_ut_ascii_char_to_hex(*two_ascii_chars.add(1) as i32)
        | (acpi_ut_ascii_char_to_hex(*two_ascii_chars as i32) << 4);

    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_ascii_char_to_hex
 *
 * PARAMETERS:  hex_char                - Hex character in Ascii. Must be:
 *                                        0-9 or A-F or a-f
 *
 * RETURN:      The binary value of the ascii/hex character
 *
 * DESCRIPTION: Perform ascii-to-hex translation
 *
 ******************************************************************************/

pub fn acpi_ut_ascii_char_to_hex(hex_char: i32) -> u8 {
    /* Values 0-9 */

    if hex_char <= b'9' as i32 {
        return (hex_char - b'0' as i32) as u8;
    }

    /* Upper case A-F */

    if hex_char <= b'F' as i32 {
        return (hex_char - 0x37) as u8;
    }

    /* Lower case a-f */

    (hex_char - 0x57) as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
