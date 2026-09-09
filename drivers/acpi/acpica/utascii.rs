// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utascii - Utility ascii functions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and common utility sources.

extern "C" {
    fn isprint(c: i32) -> i32;
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_valid_nameseg
 *
 * PARAMETERS:  name            - The name or table signature to be examined.
 *                                Four characters, does not have to be a
 *                                NULL terminated string.
 *
 * RETURN:      TRUE if signature is has 4 valid ACPI characters
 *
 * DESCRIPTION: Validate an ACPI table signature.
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_ut_valid_nameseg(name: *mut i8) -> u8 {
    let mut i: u32;

    /* Validate each character in the signature */

    i = 0;
    while i < ACPI_NAMESEG_SIZE {
        if acpi_ut_valid_name_char(*name.add(i as usize), i) == 0 {
            return 0;
        }
        i += 1;
    }

    1
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_valid_name_char
 *
 * PARAMETERS:  char            - The character to be examined
 *              position        - Byte position (0-3)
 *
 * RETURN:      TRUE if the character is valid, FALSE otherwise
 *
 * DESCRIPTION: Check for a valid ACPI character. Must be one of:
 *              1) Upper case alpha
 *              2) numeric
 *              3) underscore
 *
 *              We allow a '!' as the last character because of the ASF! table
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_ut_valid_name_char(character: i8, position: u32) -> u8 {
    if !((character >= b'A' as i8 && character <= b'Z' as i8)
        || (character >= b'0' as i8 && character <= b'9' as i8)
        || character == b'_' as i8)
    {
        /* Allow a '!' in the last position */

        if character == b'!' as i8 && position == 3 {
            return 1;
        }

        return 0;
    }

    1
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_check_and_repair_ascii
 *
 * PARAMETERS:  name                - Ascii string
 *              count               - Number of characters to check
 *
 * RETURN:      None
 *
 * DESCRIPTION: Ensure that the requested number of characters are printable
 *              Ascii characters. Sets non-printable and null chars to <space>.
 *
 ******************************************************************************/

pub unsafe extern "C" fn acpi_ut_check_and_repair_ascii(
    name: *mut u8,
    repaired_name: *mut i8,
    count: u32,
) {
    let mut i: u32 = 0;

    while i < count {
        *repaired_name.add(i as usize) = *name.add(i as usize) as i8;

        if *name.add(i as usize) == 0 {
            return;
        }
        if isprint(*name.add(i as usize) as i32) == 0 {
            *repaired_name.add(i as usize) = b' ' as i8;
        }
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
