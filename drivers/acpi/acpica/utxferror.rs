// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utxferror - Various error/warning output functions
 *
 ******************************************************************************/

// C dependencies: acpi/acpi.h and accommon.h
// Build-time condition: the complete module is omitted when ACPI_NO_ERROR_MESSAGES is defined.

use core::ffi::{c_char, c_uint, c_void};

type U32 = c_uint;
type AcpiStatus = U32;
type VaList = *mut c_void;

extern "C" {
    fn acpi_os_printf(format: *const c_char, ...);
    fn acpi_os_vprintf(format: *const c_char, arg_list: VaList);
    fn acpi_format_exception(status: AcpiStatus) -> *const c_char;
}

// The following macros expand to platform-specific message redirection,
// suffix, and message-prefix operations in the ACPICA headers.

pub unsafe extern "C" fn acpi_error(
    module_name: *const c_char,
    line_number: U32,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;

    // ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(b"ACPI Error: %s:%u: \0".as_ptr() as *const c_char, module_name, line_number);
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_exception(
    module_name: *const c_char,
    line_number: U32,
    status: AcpiStatus,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;

    // ACPI_MSG_REDIRECT_BEGIN;
    // For AE_OK, just print the message
    if status == 0 {
        acpi_os_printf(b"ACPI Error: \0".as_ptr() as *const c_char);
    } else {
        acpi_os_printf(b"ACPI Error: %s, \0".as_ptr() as *const c_char,
            acpi_format_exception(status));
    }
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_warning(
    module_name: *const c_char,
    line_number: U32,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;
    // ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(b"ACPI Warning: %s:%u: \0".as_ptr() as *const c_char, module_name, line_number);
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_info(format: *const c_char, ...) {
    let mut arg_list: VaList;
    // ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(b"ACPI: \0".as_ptr() as *const c_char);
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    acpi_os_printf(b"\n\0".as_ptr() as *const c_char);
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_bios_error(
    module_name: *const c_char,
    line_number: U32,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;
    // ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(b"ACPI BIOS Error (bug): %s:%u: \0".as_ptr() as *const c_char, module_name, line_number);
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_bios_exception(
    module_name: *const c_char,
    line_number: U32,
    status: AcpiStatus,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;
    // ACPI_MSG_REDIRECT_BEGIN;
    // For AE_OK, just print the message
    if status == 0 {
        acpi_os_printf(b"ACPI BIOS Error (bug): \0".as_ptr() as *const c_char);
    } else {
        acpi_os_printf(b"ACPI BIOS Error (bug): %s, \0".as_ptr() as *const c_char,
            acpi_format_exception(status));
    }
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

pub unsafe extern "C" fn acpi_bios_warning(
    module_name: *const c_char,
    line_number: U32,
    format: *const c_char,
    ...
) {
    let mut arg_list: VaList;
    // ACPI_MSG_REDIRECT_BEGIN;
    acpi_os_printf(b"ACPI BIOS Warning: %s:%u: \0".as_ptr() as *const c_char, module_name, line_number);
    // va_start(arg_list, format);
    acpi_os_vprintf(format, arg_list);
    // ACPI_MSG_SUFFIX;
    // va_end(arg_list);
    // ACPI_MSG_REDIRECT_END;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
