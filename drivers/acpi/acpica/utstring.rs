// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: utstring - Common functions for strings and characters

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn acpi_os_printf(format: *const c_char, ...);
    fn acpi_ut_valid_name_char(character: u8, position: u32) -> u8;
    static acpi_gbl_enable_interpreter_slack: u8;
}

pub unsafe fn acpi_ut_print_string(string: *mut c_char, max_length: u16) {
    let mut i: u32 = 0;

    if string.is_null() {
        acpi_os_printf(b"<\"NULL STRING PTR\">\0".as_ptr() as *const c_char);
        return;
    }

    acpi_os_printf(b"\"\0".as_ptr() as *const c_char);
    while i < max_length as u32 && *string.add(i as usize) != 0 {
        let ch = *string.add(i as usize) as u8;
        match ch {
            0x07 => acpi_os_printf(b"\\a\0".as_ptr() as *const c_char),
            0x08 => acpi_os_printf(b"\\b\0".as_ptr() as *const c_char),
            0x0c => acpi_os_printf(b"\\f\0".as_ptr() as *const c_char),
            0x0a => acpi_os_printf(b"\\n\0".as_ptr() as *const c_char),
            0x0d => acpi_os_printf(b"\\r\0".as_ptr() as *const c_char),
            0x09 => acpi_os_printf(b"\\t\0".as_ptr() as *const c_char),
            0x0b => acpi_os_printf(b"\\v\0".as_ptr() as *const c_char),
            b'\'' | b'"' | b'\\' => {
                acpi_os_printf(b"\\%c\0".as_ptr() as *const c_char, ch as c_int);
            }
            _ => {
                if (ch as c_int >= 0x20 && ch as c_int <= 0x7e) {
                    acpi_os_printf(b"%c\0".as_ptr() as *const c_char, ch as c_int);
                } else {
                    acpi_os_printf(b"\\x%2.2X\0".as_ptr() as *const c_char, ch as c_int);
                }
            }
        }
        i += 1;
    }

    acpi_os_printf(b"\"\0".as_ptr() as *const c_char);
    if i == max_length as u32 && *string.add(i as usize) != 0 {
        acpi_os_printf(b"...\0".as_ptr() as *const c_char);
    }
}

pub unsafe fn acpi_ut_repair_name(name: *mut c_char) {
    let mut found_bad_char: u8 = 0;
    let original_name = *(name as *const u32);

    // Special handling for the root node is retained by the caller's ACPI
    // name comparison facilities.
    if acpi_compare_nameseg(name, ACPI_ROOT_PATHNAME) != 0 {
        return;
    }

    for i in 0..ACPI_NAMESEG_SIZE as u32 {
        if acpi_ut_valid_name_char(*name.add(i as usize) as u8, i) != 0 {
            continue;
        }
        *name.add(i as usize) = b'_' as c_char;
        found_bad_char = 1;
    }

    if found_bad_char != 0 {
        if acpi_gbl_enable_interpreter_slack == 0 {
            acpi_warning(original_name, name);
        } else {
            acpi_debug_print(original_name, name);
        }
    }
}

// Build-time ACPI utility macros and constants supplied by the surrounding
// ACPICA translation unit.
extern "C" {
    fn acpi_compare_nameseg(name: *const c_char, root: *const c_char) -> c_int;
    fn acpi_warning(original_name: u32, name: *const c_char);
    fn acpi_debug_print(original_name: u32, name: *const c_char);
}

extern "C" {
    static ACPI_ROOT_PATHNAME: *const c_char;
    static ACPI_NAMESEG_SIZE: u32;
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_exec_app"))]
pub unsafe fn ut_convert_backslashes(mut pathname: *mut c_char) {
    if pathname.is_null() {
        return;
    }
    while *pathname != 0 {
        if *pathname == b'\\' as c_char {
            *pathname = b'/' as c_char;
        }
        pathname = pathname.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
