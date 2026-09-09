// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utstrsuppt - Support functions for string-to-integer conversion

// Dependencies supplied by the ACPICA translation environment are referenced
// here by their original names.

extern "C" {
    static mut acpi_gbl_integer_bit_width: u32;
}

unsafe fn acpi_ut_insert_digit(accumulated_value: *mut u64, base: u32, ascii_digit: i32) -> acpi_status {
    let mut product: u64 = 0;
    let status = acpi_ut_strtoul_multiply64(*accumulated_value, base, &mut product);
    if ACPI_FAILURE(status) {
        return status;
    }
    acpi_ut_strtoul_add64(
        product,
        acpi_ut_ascii_char_to_hex(ascii_digit),
        accumulated_value,
    )
}

pub unsafe fn acpi_ut_convert_octal_string(string: *mut i8, return_value_ptr: *mut u64) -> acpi_status {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
    let mut string = string;
    while *string != 0 {
        if !ACPI_IS_OCTAL_DIGIT(*string) {
            // Under ACPI_ASL_COMPILER: status = AE_BAD_OCTAL_CONSTANT;
            break;
        }
        status = acpi_ut_insert_digit(&mut accumulated_value, 8, *string as i32);
        if ACPI_FAILURE(status) {
            status = AE_OCTAL_OVERFLOW;
            break;
        }
        string = string.add(1);
    }
    *return_value_ptr = accumulated_value;
    status
}

pub unsafe fn acpi_ut_convert_decimal_string(string: *mut i8, return_value_ptr: *mut u64) -> acpi_status {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
    let mut string = string;
    while *string != 0 {
        if !isdigit(*string as i32) {
            // Under ACPI_ASL_COMPILER: status = AE_BAD_DECIMAL_CONSTANT;
            break;
        }
        status = acpi_ut_insert_digit(&mut accumulated_value, 10, *string as i32);
        if ACPI_FAILURE(status) {
            status = AE_DECIMAL_OVERFLOW;
            break;
        }
        string = string.add(1);
    }
    *return_value_ptr = accumulated_value;
    status
}

pub unsafe fn acpi_ut_convert_hex_string(string: *mut i8, return_value_ptr: *mut u64) -> acpi_status {
    let mut accumulated_value: u64 = 0;
    let mut status: acpi_status = AE_OK;
    let mut string = string;
    while *string != 0 {
        if !isxdigit(*string as i32) {
            // Under ACPI_ASL_COMPILER: status = AE_BAD_HEX_CONSTANT;
            break;
        }
        status = acpi_ut_insert_digit(&mut accumulated_value, 16, *string as i32);
        if ACPI_FAILURE(status) {
            status = AE_HEX_OVERFLOW;
            break;
        }
        string = string.add(1);
    }
    *return_value_ptr = accumulated_value;
    status
}

pub unsafe fn acpi_ut_remove_leading_zeros(string: *mut *mut i8) -> i8 {
    while **string == ACPI_ASCII_ZERO {
        *string = (*string).add(1);
    }
    **string
}

pub unsafe fn acpi_ut_remove_whitespace(string: *mut *mut i8) -> i8 {
    while isspace(**string as u8 as i32) {
        *string = (*string).add(1);
    }
    **string
}

pub unsafe fn acpi_ut_detect_hex_prefix(string: *mut *mut i8) -> u8 {
    let initial_position = *string;
    acpi_ut_remove_hex_prefix(string);
    if *string != initial_position { TRUE } else { FALSE }
}

pub unsafe fn acpi_ut_remove_hex_prefix(string: *mut *mut i8) {
    if **string == ACPI_ASCII_ZERO && tolower(*(*string).add(1) as i32) == 'x' as i32 {
        *string = (*string).add(2);
    }
}

pub unsafe fn acpi_ut_detect_octal_prefix(string: *mut *mut i8) -> u8 {
    if **string == ACPI_ASCII_ZERO {
        *string = (*string).add(1);
        TRUE
    } else {
        FALSE
    }
}

unsafe fn acpi_ut_strtoul_multiply64(multiplicand: u64, base: u32, out_product: *mut u64) -> acpi_status {
    let mut quotient: u64 = 0;
    *out_product = 0;
    if multiplicand == 0 || base == 0 { return AE_OK; }
    acpi_ut_short_divide(ACPI_UINT64_MAX, base, &mut quotient, core::ptr::null_mut());
    if multiplicand > quotient { return AE_NUMERIC_OVERFLOW; }
    let product = multiplicand * base as u64;
    if acpi_gbl_integer_bit_width == 32 && product > ACPI_UINT32_MAX as u64 { return AE_NUMERIC_OVERFLOW; }
    *out_product = product;
    AE_OK
}

unsafe fn acpi_ut_strtoul_add64(addend1: u64, digit: u32, out_sum: *mut u64) -> acpi_status {
    if addend1 > 0 && digit as u64 > ACPI_UINT64_MAX - addend1 { return AE_NUMERIC_OVERFLOW; }
    let sum = addend1 + digit as u64;
    if acpi_gbl_integer_bit_width == 32 && sum > ACPI_UINT32_MAX as u64 { return AE_NUMERIC_OVERFLOW; }
    *out_sum = sum;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
