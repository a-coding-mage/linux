// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: utpredef - support functions for predefined names
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// Dependencies supplied by the ACPI headers/build environment are intentionally
// referenced here without reproducing their definitions.

static UT_RTYPE_NAMES: [&[u8]; 5] = [
    b"/Integer\0",
    b"/String\0",
    b"/Buffer\0",
    b"/Package\0",
    b"/Reference\0",
];

pub unsafe fn acpi_ut_get_next_predefined_method(
    mut this_name: *const crate::acpi_predefined_info,
) -> *const crate::acpi_predefined_info {
    if ((*this_name).info.expected_btypes & crate::ACPI_RTYPE_PACKAGE) != 0
        && (*this_name).info.expected_btypes != crate::ACPI_RTYPE_ALL
    {
        this_name = this_name.add(1);
    }

    this_name.add(1)
}

pub unsafe fn acpi_ut_match_predefined_method(
    name: *mut core::ffi::c_char,
) -> *const crate::acpi_predefined_info {
    if *(name as *const u8) != b'_' {
        return core::ptr::null();
    }

    let mut this_name = crate::acpi_gbl_predefined_methods;
    while (*this_name).info.name[0] != 0 {
        if crate::ACPI_COMPARE_NAMESEG(name, (*this_name).info.name.as_ptr()) {
            return this_name;
        }
        this_name = acpi_ut_get_next_predefined_method(this_name);
    }

    core::ptr::null()
}

pub unsafe fn acpi_ut_get_expected_return_types(
    buffer: *mut core::ffi::c_char,
    expected_btypes: u32,
) {
    if expected_btypes == 0 {
        crate::strcpy(buffer, b"NONE\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    let mut sub_index = 1usize;
    *buffer = 0;
    let mut this_rtype = crate::ACPI_RTYPE_INTEGER;

    for i in 0..crate::ACPI_NUM_RTYPES as usize {
        if expected_btypes & this_rtype != 0 {
            crate::strcat(buffer, UT_RTYPE_NAMES[i].as_ptr().add(sub_index) as *const _);
            sub_index = 0;
        }
        this_rtype <<= 1;
    }
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
static UT_EXTERNAL_TYPE_NAMES: [&[u8]; 5] = [
    b", Type_ANY\0", b", Integer\0", b", String\0", b", Buffer\0", b", Package\0",
];

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
static UT_RESOURCE_TYPE_NAMES: [&[u8]; 8] = [
    b"/1\0", b"/2\0", b"/3\0", b"/8\0", b"/16\0", b"/32\0", b"/64\0", b"/variable\0",
];

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
pub unsafe fn acpi_ut_match_resource_name(
    name: *mut core::ffi::c_char,
) -> *const crate::acpi_predefined_info {
    if *(name as *const u8) != b'_' {
        return core::ptr::null();
    }

    let mut this_name = crate::acpi_gbl_resource_names;
    while (*this_name).info.name[0] != 0 {
        if crate::ACPI_COMPARE_NAMESEG(name, (*this_name).info.name.as_ptr()) {
            return this_name;
        }
        this_name = this_name.add(1);
    }
    core::ptr::null()
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
pub unsafe fn acpi_ut_display_predefined_method(
    buffer: *mut core::ffi::c_char,
    this_name: *const crate::acpi_predefined_info,
    multi_line: u8,
) {
    let arg_count = acpi_ut_get_argument_types(buffer, (*this_name).info.argument_list);
    if multi_line != 0 { crate::printf(b"      \0".as_ptr() as *const _); }
    crate::printf(
        b"%4.4s    Requires %s%u argument%s\0".as_ptr() as *const _,
        (*this_name).info.name.as_ptr(),
        if (*this_name).info.argument_list & crate::ARG_COUNT_IS_MINIMUM != 0 { b"(at least) \0".as_ptr() } else { b"\0".as_ptr() },
        arg_count,
        if arg_count != 1 { b"s\0".as_ptr() } else { b"\0".as_ptr() },
    );
    if arg_count > 0 { crate::printf(b" (%s)\0".as_ptr() as *const _, buffer); }
    if multi_line != 0 { crate::printf(b"\n    \0".as_ptr() as *const _); }
    if (*this_name).info.expected_btypes != 0 {
        acpi_ut_get_expected_return_types(buffer, (*this_name).info.expected_btypes);
        crate::printf(b"  Return value types: %s\n\0".as_ptr() as *const _, buffer);
    } else { crate::printf(b"  No return value\n\0".as_ptr() as *const _); }
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
unsafe fn acpi_ut_get_argument_types(buffer: *mut core::ffi::c_char, mut argument_types: u16) -> u32 {
    *buffer = 0;
    let mut sub_index = 2usize;
    let arg_count = crate::METHOD_GET_ARG_COUNT(argument_types);
    if arg_count > crate::METHOD_PREDEF_ARGS_MAX { crate::printf(b"**** Invalid argument count (%u) in predefined info structure\n\0".as_ptr() as *const _, arg_count); return arg_count; }
    for _ in 0..arg_count {
        let this_argument_type = crate::METHOD_GET_NEXT_TYPE(&mut argument_types);
        if this_argument_type > crate::METHOD_MAX_ARG_TYPE { crate::printf(b"**** Invalid argument type (%u) in predefined info structure\n\0".as_ptr() as *const _, this_argument_type); return arg_count; }
        crate::strcat(buffer, UT_EXTERNAL_TYPE_NAMES[this_argument_type as usize].as_ptr().add(sub_index) as *const _);
        sub_index = 0;
    }
    arg_count
}

#[cfg(any(feature = "acpi_asl_compiler", feature = "acpi_help_app"))]
pub unsafe fn acpi_ut_get_resource_bit_width(buffer: *mut core::ffi::c_char, mut types: u16) -> u32 {
    *buffer = 0;
    let mut sub_index = 1usize;
    let mut found = 0u32;
    for i in 0..crate::NUM_RESOURCE_WIDTHS as usize {
        if types & 1 != 0 { crate::strcat(buffer, UT_RESOURCE_TYPE_NAMES[i].as_ptr().add(sub_index) as *const _); sub_index = 0; found += 1; }
        types >>= 1;
    }
    found
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
