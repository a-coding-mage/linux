// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utprint - Formatted printing routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation.

const ACPI_FORMAT_SIGN: u8 = 0x01;
const ACPI_FORMAT_SIGN_PLUS: u8 = 0x02;
const ACPI_FORMAT_SIGN_PLUS_SPACE: u8 = 0x04;
const ACPI_FORMAT_ZERO: u8 = 0x08;
const ACPI_FORMAT_LEFT: u8 = 0x10;
const ACPI_FORMAT_UPPER: u8 = 0x20;
const ACPI_FORMAT_PREFIX: u8 = 0x40;

extern "C" {
    static acpi_gbl_upper_hex_digits: *const u8;
    static acpi_gbl_lower_hex_digits: *const u8;
    fn acpi_ut_divide(dividend: u64, divisor: u8, quotient: *mut u64, remainder: *mut u64) -> u32;
    fn acpi_ut_short_multiply(multiplicand: u64, multiplier: u32, product: *mut u64);
}

unsafe fn acpi_ut_bound_string_length(mut string: *const u8, mut count: usize) -> u32 {
    let mut length: u32 = 0;
    while *string != 0 && count != 0 {
        length += 1;
        string = string.add(1);
        count -= 1;
    }
    length
}

unsafe fn acpi_ut_bound_string_output(mut string: *mut u8, end: *const u8, c: u8) -> *mut u8 {
    if (string as usize) < (end as usize) {
        *string = c;
    }
    string = string.add(1);
    string
}

unsafe fn acpi_ut_put_number(mut string: *mut u8, mut number: u64, base: u8, upper: u8) -> *mut u8 {
    let digits = if upper != 0 { acpi_gbl_upper_hex_digits } else { acpi_gbl_lower_hex_digits };
    let mut pos = string;
    if number == 0 {
        *pos = b'0';
        pos = pos.add(1);
    } else {
        while number != 0 {
            let mut digit_index = 0u64;
            acpi_ut_divide(number, base, &mut number, &mut digit_index);
            *pos = *digits.add(digit_index as usize);
            pos = pos.add(1);
        }
    }
    pos
}

pub unsafe fn acpi_ut_scan_number(mut string: *const u8, number_ptr: *mut u64) -> *const u8 {
    let mut number = 0u64;
    while (*string as u8).is_ascii_digit() {
        acpi_ut_short_multiply(number, 10, &mut number);
        number = number.wrapping_add((*string - b'0') as u64);
        string = string.add(1);
    }
    *number_ptr = number;
    string
}

pub unsafe fn acpi_ut_print_number(string: *mut u8, number: u64) -> *const u8 {
    let mut ascii_string = [0u8; 20];
    let mut pos1 = acpi_ut_put_number(ascii_string.as_mut_ptr(), number, 10, 0);
    let mut pos2 = string;
    while pos1 != ascii_string.as_mut_ptr() {
        pos1 = pos1.sub(1);
        *pos2 = *pos1;
        pos2 = pos2.add(1);
    }
    *pos2 = 0;
    string
}

unsafe fn acpi_ut_format_number(mut string: *mut u8, end: *mut u8, mut number: u64, base: u8, mut width: i32, mut precision: i32, mut type_: u8) -> *mut u8 {
    if base < 2 || base > 16 { return core::ptr::null_mut(); }
    if type_ & ACPI_FORMAT_LEFT != 0 { type_ &= !ACPI_FORMAT_ZERO; }
    let need_prefix = if type_ & ACPI_FORMAT_PREFIX != 0 && base != 10 { 1 } else { 0 };
    let upper = if type_ & ACPI_FORMAT_UPPER != 0 { 1 } else { 0 };
    let zero = if type_ & ACPI_FORMAT_ZERO != 0 { b'0' } else { b' ' };
    let mut sign = 0u8;
    if type_ & ACPI_FORMAT_SIGN != 0 {
        if (number as i64) < 0 { sign = b'-'; number = (-(number as i64)) as u64; width -= 1; }
        else if type_ & ACPI_FORMAT_SIGN_PLUS != 0 { sign = b'+'; width -= 1; }
        else if type_ & ACPI_FORMAT_SIGN_PLUS_SPACE != 0 { sign = b' '; width -= 1; }
    }
    if need_prefix != 0 { width -= 1; if base == 16 { width -= 1; } }
    let mut reversed_string = [0u8; 66];
    let pos = acpi_ut_put_number(reversed_string.as_mut_ptr(), number, base, upper);
    let mut i = (pos as usize - reversed_string.as_mut_ptr() as usize) as i32;
    if i > precision { precision = i; }
    width -= precision;
    if type_ & (ACPI_FORMAT_ZERO | ACPI_FORMAT_LEFT) == 0 { while { width -= 1; width >= 0 } { string = acpi_ut_bound_string_output(string, end, b' '); } }
    if sign != 0 { string = acpi_ut_bound_string_output(string, end, sign); }
    if need_prefix != 0 { string = acpi_ut_bound_string_output(string, end, b'0'); if base == 16 { string = acpi_ut_bound_string_output(string, end, if upper != 0 { b'X' } else { b'x' }); } }
    if type_ & ACPI_FORMAT_LEFT == 0 { while { width -= 1; width >= 0 } { string = acpi_ut_bound_string_output(string, end, zero); } }
    while { precision -= 1; i <= precision } { string = acpi_ut_bound_string_output(string, end, b'0'); }
    while { i -= 1; i >= 0 } { string = acpi_ut_bound_string_output(string, end, *reversed_string.get_unchecked(i as usize)); }
    while { width -= 1; width >= 0 } { string = acpi_ut_bound_string_output(string, end, b' '); }
    string
}

// The following entry points use the target's native C va_list ABI. Their
// definitions are supplied by the ACPICA variadic compatibility layer.
extern "C" {
    pub fn vsnprintf(string: *mut u8, size: usize, format: *const u8, args: *mut core::ffi::c_void) -> i32;
    pub fn snprintf(string: *mut u8, size: usize, format: *const u8, ...) -> i32;
    pub fn sprintf(string: *mut u8, format: *const u8, ...) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
