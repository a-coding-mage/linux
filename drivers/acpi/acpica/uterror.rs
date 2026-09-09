// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: uterror - Various internal error/warning output functions
 *
 ******************************************************************************/

// The declarations below are supplied by the ACPI implementation and its headers.

#[allow(non_camel_case_types)]
pub type u32 = std::ffi::c_uint;
#[allow(non_camel_case_types)]
pub type u16 = std::ffi::c_ushort;
#[allow(non_camel_case_types)]
pub type acpi_status = u32;

#[repr(C)]
pub struct acpi_generic_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_namespace_node {
    _private: [u8; 0],
}

extern "C" {
    fn acpi_os_printf(format: *const std::ffi::c_char, ...);
    fn acpi_os_vprintf(format: *const std::ffi::c_char, args: *mut std::ffi::c_void);
    fn acpi_ns_build_prefixed_pathname(
        prefix_scope: *mut acpi_generic_state,
        internal_path: *const std::ffi::c_char,
    ) -> *mut std::ffi::c_char;
    fn acpi_format_exception(status: acpi_status) -> *const std::ffi::c_char;
    fn acpi_ns_get_node(
        prefix_node: *mut acpi_namespace_node,
        path: *const std::ffi::c_char,
        flags: u32,
        node: *mut *mut acpi_namespace_node,
    ) -> acpi_status;
    fn acpi_ns_print_node_pathname(
        node: *mut acpi_namespace_node,
        message: *const std::ffi::c_char,
    );
    fn acpi_ns_externalize_name(
        name_length: u32,
        internal_name: *const std::ffi::c_char,
        converted_name: *mut std::ffi::c_char,
        buffer: *mut *mut std::ffi::c_char,
    ) -> acpi_status;
    fn acpi_free(pointer: *mut std::ffi::c_void);
}

const ANOBJ_EVALUATED: u16 = 0x0800;
const AE_ALREADY_EXISTS: acpi_status = 0x0005;
const AE_NOT_FOUND: acpi_status = 0x0006;
const AE_BAD_CHARACTER: acpi_status = 0x0007;
const ACPI_NS_NO_UPSEARCH: u32 = 0x0001;
const ACPI_UINT32_MAX: u32 = u32::MAX;

// ACPI_MSG_* and ACPI_MSG_SUFFIX/REDIRECT macros are supplied by the ACPI headers.
const ACPI_MSG_WARNING: &[u8] = b"WARNING: \0";
const ACPI_MSG_INFO: &[u8] = b"INFO: \0";
const ACPI_MSG_BIOS_ERROR: &[u8] = b"BIOS Error: \0";
const ACPI_MSG_ERROR: &[u8] = b"Error: \0";

/*
 * This module contains internal error functions that may
 * be configured out.
 */
#[cfg(not(feature = "ACPI_NO_ERROR_MESSAGES"))]
pub unsafe extern "C" fn acpi_ut_predefined_warning(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    pathname: *mut std::ffi::c_char,
    node_flags: u16,
    format: *const std::ffi::c_char,
    ...
) {
    if node_flags & ANOBJ_EVALUATED != 0 {
        return;
    }
    acpi_os_printf(ACPI_MSG_WARNING.as_ptr() as *const _, pathname);
    // C va_list forwarding is an ABI operation supplied by the target runtime.
    let _ = format;
    // ACPI_MSG_SUFFIX;
}

#[cfg(not(feature = "ACPI_NO_ERROR_MESSAGES"))]
pub unsafe extern "C" fn acpi_ut_predefined_info(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    pathname: *mut std::ffi::c_char,
    node_flags: u16,
    format: *const std::ffi::c_char,
    ...
) {
    if node_flags & ANOBJ_EVALUATED != 0 {
        return;
    }
    acpi_os_printf(ACPI_MSG_INFO.as_ptr() as *const _, pathname);
    let _ = format;
    // ACPI_MSG_SUFFIX;
}

#[cfg(not(feature = "ACPI_NO_ERROR_MESSAGES"))]
pub unsafe extern "C" fn acpi_ut_predefined_bios_error(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    pathname: *mut std::ffi::c_char,
    node_flags: u16,
    format: *const std::ffi::c_char,
    ...
) {
    if node_flags & ANOBJ_EVALUATED != 0 {
        return;
    }
    acpi_os_printf(ACPI_MSG_BIOS_ERROR.as_ptr() as *const _, pathname);
    let _ = format;
    // ACPI_MSG_SUFFIX;
}

#[cfg(not(feature = "ACPI_NO_ERROR_MESSAGES"))]
pub unsafe extern "C" fn acpi_ut_prefixed_namespace_error(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    prefix_scope: *mut acpi_generic_state,
    internal_path: *const std::ffi::c_char,
    lookup_status: acpi_status,
) {
    let message: *const std::ffi::c_char;
    match lookup_status {
        AE_ALREADY_EXISTS => {
            acpi_os_printf(ACPI_MSG_BIOS_ERROR.as_ptr() as *const _);
            message = b"Failure creating named object\0".as_ptr() as *const _;
        }
        AE_NOT_FOUND => {
            acpi_os_printf(ACPI_MSG_BIOS_ERROR.as_ptr() as *const _);
            message = b"Could not resolve symbol\0".as_ptr() as *const _;
        }
        _ => {
            acpi_os_printf(ACPI_MSG_ERROR.as_ptr() as *const _);
            message = b"Failure resolving symbol\0".as_ptr() as *const _;
        }
    }
    let full_path = acpi_ns_build_prefixed_pathname(prefix_scope, internal_path);
    let fallback = b"Could not get pathname\0";
    acpi_os_printf(
        b"%s [%s], %s\0".as_ptr() as *const _,
        message,
        if full_path.is_null() { fallback.as_ptr() } else { full_path as *const _ },
        acpi_format_exception(lookup_status),
    );
    if !full_path.is_null() {
        acpi_free(full_path as *mut _);
    }
    // ACPI_MSG_SUFFIX;
}

#[cfg(not(feature = "ACPI_NO_ERROR_MESSAGES"))]
pub unsafe extern "C" fn acpi_ut_method_error(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    message: *const std::ffi::c_char,
    prefix_node: *mut acpi_namespace_node,
    path: *const std::ffi::c_char,
    method_status: acpi_status,
) {
    let mut node = prefix_node;
    acpi_os_printf(ACPI_MSG_ERROR.as_ptr() as *const _);
    if !path.is_null() {
        let status = acpi_ns_get_node(prefix_node, path, ACPI_NS_NO_UPSEARCH, &mut node);
        if status != 0 {
            acpi_os_printf(b"[Could not get node by pathname]\0".as_ptr() as *const _);
        }
    }
    acpi_ns_print_node_pathname(node, message);
    acpi_os_printf(
        b" due to previous error (%s)\0".as_ptr() as *const _,
        acpi_format_exception(method_status),
    );
    // ACPI_MSG_SUFFIX;
}

#[cfg(all(not(feature = "ACPI_NO_ERROR_MESSAGES"), feature = "__OBSOLETE_FUNCTION"))]
pub unsafe extern "C" fn acpi_ut_namespace_error(
    _module_name: *const std::ffi::c_char,
    _line_number: u32,
    internal_name: *const std::ffi::c_char,
    lookup_status: acpi_status,
) {
    let mut name: *mut std::ffi::c_char = std::ptr::null_mut();
    acpi_os_printf(ACPI_MSG_ERROR.as_ptr() as *const _);
    if lookup_status == AE_BAD_CHARACTER {
        let bad_name = *(internal_name as *const u32);
        acpi_os_printf(b"[0x%.8X] (NON-ASCII)\0".as_ptr() as *const _, bad_name);
    } else {
        let status = acpi_ns_externalize_name(
            ACPI_UINT32_MAX,
            internal_name,
            std::ptr::null_mut(),
            &mut name,
        );
        if status == 0 {
            acpi_os_printf(b"[%s]\0".as_ptr() as *const _, name);
        } else {
            acpi_os_printf(b"[COULD NOT EXTERNALIZE NAME]\0".as_ptr() as *const _);
        }
        if !name.is_null() {
            acpi_free(name as *mut _);
        }
    }
    acpi_os_printf(
        b" Namespace lookup failure, %s\0".as_ptr() as *const _,
        acpi_format_exception(lookup_status),
    );
    // ACPI_MSG_SUFFIX;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
