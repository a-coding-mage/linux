// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utuuid -- UUID support functions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies: <acpi/acpi.h> and "accommon.h"

// #if (defined ACPI_ASL_COMPILER || defined ACPI_EXEC_APP || defined ACPI_HELP_APP)
/*
 * UUID support functions.
 *
 * This table is used to convert an input UUID ascii string to a 16 byte
 * buffer and the reverse. The table maps a UUID buffer index 0-15 to
 * the index within the 36-byte UUID string where the associated 2-byte
 * hex value can be found.
 *
 * 36-byte UUID strings are of the form:
 *     aabbccdd-eeff-gghh-iijj-kkllmmnnoopp
 * Where aa-pp are one byte hex numbers, made up of two hex digits
 *
 * Note: This table is basically the inverse of the string-to-offset table
 * found in the ACPI spec in the description of the to_UUID macro.
 */
pub static acpi_gbl_map_to_uuid_offset: [u8; UUID_BUFFER_LENGTH as usize] = [
    6, 4, 2, 0, 11, 9, 16, 14, 19, 21, 24, 26, 28, 30, 32, 34,
];

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_convert_string_to_uuid
 *
 * PARAMETERS:  in_string           - 36-byte formatted UUID string
 *              uuid_buffer         - Where the 16-byte UUID buffer is returned
 *
 * RETURN:      None. Output data is returned in the uuid_buffer
 *
 * DESCRIPTION: Convert a 36-byte formatted UUID string to 16-byte UUID buffer
 *
 ******************************************************************************/

pub unsafe fn acpi_ut_convert_string_to_uuid(
    in_string: *mut core::ffi::c_char,
    uuid_buffer: *mut u8,
) {
    let mut i: u32 = 0;

    while i < UUID_BUFFER_LENGTH {
        *uuid_buffer.add(i as usize) =
            acpi_ut_ascii_char_to_hex(*in_string.add(acpi_gbl_map_to_uuid_offset[i as usize] as usize)) << 4;

        *uuid_buffer.add(i as usize) |= acpi_ut_ascii_char_to_hex(
            *in_string.add((acpi_gbl_map_to_uuid_offset[i as usize] + 1) as usize),
        );
        i += 1;
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_convert_uuid_to_string
 *
 * PARAMETERS:  uuid_buffer         - 16-byte UUID buffer
 *              out_string          - 36-byte formatted UUID string
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Convert 16-byte UUID buffer to 36-byte formatted UUID string
 *              out_string must be 37 bytes to include null terminator.
 *
 ******************************************************************************/

pub unsafe fn acpi_ut_convert_uuid_to_string(
    uuid_buffer: *mut core::ffi::c_char,
    out_string: *mut core::ffi::c_char,
) -> acpi_status {
    let mut i: u32;

    if uuid_buffer.is_null() || out_string.is_null() {
        return AE_BAD_PARAMETER;
    }

    i = 0;
    while i < UUID_BUFFER_LENGTH {
        *out_string.add(acpi_gbl_map_to_uuid_offset[i as usize] as usize) =
            acpi_ut_hex_to_ascii_char(*uuid_buffer.add(i as usize) as u8, 4) as core::ffi::c_char;

        *out_string.add((acpi_gbl_map_to_uuid_offset[i as usize] + 1) as usize) =
            acpi_ut_hex_to_ascii_char(*uuid_buffer.add(i as usize) as u8, 0) as core::ffi::c_char;
        i += 1;
    }

    /* Insert required hyphens (dashes) */

    *out_string.add(UUID_HYPHEN1_OFFSET as usize) = '-' as core::ffi::c_char;
    *out_string.add(UUID_HYPHEN2_OFFSET as usize) = '-' as core::ffi::c_char;
    *out_string.add(UUID_HYPHEN3_OFFSET as usize) = '-' as core::ffi::c_char;
    *out_string.add(UUID_HYPHEN4_OFFSET as usize) = '-' as core::ffi::c_char;

    *out_string.add(UUID_STRING_LENGTH as usize) = 0 as core::ffi::c_char; // Null terminate
    AE_OK
}
// #endif

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
