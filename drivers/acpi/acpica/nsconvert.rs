// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nsconvert - Object conversions for objects returned by
 *                          predefined methods
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPICA Rust translation.

pub unsafe fn acpi_ns_convert_to_integer(
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut new_object: *mut acpi_operand_object;
    let status: acpi_status;
    let mut value: u64 = 0;
    let mut i: u32;

    match (*original_object).common.type_ {
        ACPI_TYPE_STRING => {
            status = acpi_ut_strtoul64((*original_object).string.pointer, &mut value);
            if ACPI_FAILURE(status) {
                return status;
            }
        }
        ACPI_TYPE_BUFFER => {
            if (*original_object).buffer.length > 8 {
                return AE_AML_OPERAND_TYPE;
            }
            i = 0;
            while i < (*original_object).buffer.length {
                value |= ((*original_object).buffer.pointer.add(i as usize) as u64)
                    << (i * 8);
                i += 1;
            }
        }
        _ => return AE_AML_OPERAND_TYPE,
    }

    new_object = acpi_ut_create_integer_object(value);
    if new_object.is_null() {
        return AE_NO_MEMORY;
    }
    *return_object = new_object;
    AE_OK
}

pub unsafe fn acpi_ns_convert_to_string(
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let new_object: *mut acpi_operand_object;
    let mut length: acpi_size;
    let status: acpi_status;

    match (*original_object).common.type_ {
        ACPI_TYPE_INTEGER => {
            if (*original_object).integer.value == 0 {
                new_object = acpi_ut_create_string_object(0);
                if new_object.is_null() {
                    return AE_NO_MEMORY;
                }
            } else {
                status = acpi_ex_convert_to_string(
                    original_object,
                    &mut (new_object as *mut acpi_operand_object),
                    ACPI_IMPLICIT_CONVERT_HEX,
                );
                if ACPI_FAILURE(status) {
                    return status;
                }
            }
        }
        ACPI_TYPE_BUFFER => {
            length = 0;
            while length < (*original_object).buffer.length
                && *(*original_object).buffer.pointer.add(length as usize) != 0
            {
                length += 1;
            }
            new_object = acpi_ut_create_string_object(length);
            if new_object.is_null() {
                return AE_NO_MEMORY;
            }
            core::ptr::copy_nonoverlapping(
                (*original_object).buffer.pointer,
                (*new_object).string.pointer,
                length as usize,
            );
        }
        _ => return AE_AML_OPERAND_TYPE,
    }
    *return_object = new_object;
    AE_OK
}

pub unsafe fn acpi_ns_convert_to_buffer(
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let new_object: *mut acpi_operand_object;
    let status: acpi_status;
    let mut elements: *mut *mut acpi_operand_object;
    let mut dword_buffer: *mut u32;
    let count: u32;
    let mut i: u32;

    match (*original_object).common.type_ {
        ACPI_TYPE_INTEGER => {
            status = acpi_ex_convert_to_buffer(original_object, &mut (new_object as *mut _));
            if ACPI_FAILURE(status) { return status; }
        }
        ACPI_TYPE_STRING => {
            new_object = acpi_ut_create_buffer_object((*original_object).string.length);
            if new_object.is_null() { return AE_NO_MEMORY; }
            core::ptr::copy_nonoverlapping(
                (*original_object).string.pointer,
                (*new_object).buffer.pointer,
                (*original_object).string.length as usize,
            );
        }
        ACPI_TYPE_PACKAGE => {
            elements = (*original_object).package.elements;
            count = (*original_object).package.count;
            i = 0;
            while i < count {
                if (*elements).is_null() || (**elements).common.type_ != ACPI_TYPE_INTEGER {
                    return AE_AML_OPERAND_TYPE;
                }
                elements = elements.add(1);
                i += 1;
            }
            new_object = acpi_ut_create_buffer_object(ACPI_MUL_4(count));
            if new_object.is_null() { return AE_NO_MEMORY; }
            elements = (*original_object).package.elements;
            dword_buffer = (*new_object).buffer.pointer as *mut u32;
            i = 0;
            while i < count {
                *dword_buffer = (**elements).integer.value as u32;
                dword_buffer = dword_buffer.add(1);
                elements = elements.add(1);
                i += 1;
            }
        }
        _ => return AE_AML_OPERAND_TYPE,
    }
    *return_object = new_object;
    AE_OK
}

pub unsafe fn acpi_ns_convert_to_unicode(
    _scope: *mut acpi_namespace_node,
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let new_object: *mut acpi_operand_object;
    let ascii_string: *mut i8;
    let unicode_buffer: *mut u16;
    let unicode_length: u32;
    let mut i: u32;
    if original_object.is_null() { return AE_OK; }
    if (*original_object).common.type_ == ACPI_TYPE_BUFFER {
        if (*original_object).buffer.length < 2 { return AE_AML_OPERAND_VALUE; }
        *return_object = core::ptr::null_mut();
        return AE_OK;
    }
    ascii_string = (*original_object).string.pointer;
    unicode_length = (*original_object).string.length * 2 + 2;
    new_object = acpi_ut_create_buffer_object(unicode_length);
    if new_object.is_null() { return AE_NO_MEMORY; }
    unicode_buffer = (*new_object).buffer.pointer as *mut u16;
    i = 0;
    while i < (*original_object).string.length {
        *unicode_buffer.add(i as usize) = *ascii_string.add(i as usize) as u16;
        i += 1;
    }
    *return_object = new_object;
    AE_OK
}

pub unsafe fn acpi_ns_convert_to_resource(
    _scope: *mut acpi_namespace_node,
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let new_object: *mut acpi_operand_object;
    let buffer: *mut u8;
    if !original_object.is_null() {
        match (*original_object).common.type_ {
            ACPI_TYPE_INTEGER => if (*original_object).integer.value != 0 { return AE_AML_OPERAND_TYPE; },
            ACPI_TYPE_BUFFER => if (*original_object).buffer.length != 0 {
                *return_object = core::ptr::null_mut(); return AE_OK;
            },
            _ => return AE_AML_OPERAND_TYPE,
        }
    }
    new_object = acpi_ut_create_buffer_object(2);
    if new_object.is_null() { return AE_NO_MEMORY; }
    buffer = (*new_object).buffer.pointer;
    *buffer = (ACPI_RESOURCE_NAME_END_TAG | ASL_RDESC_END_TAG_SIZE) as u8;
    *buffer.add(1) = 0;
    *return_object = new_object;
    AE_OK
}

pub unsafe fn acpi_ns_convert_to_reference(
    scope: *mut acpi_namespace_node,
    original_object: *mut acpi_operand_object,
    return_object: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut new_object: *mut acpi_operand_object = core::ptr::null_mut();
    let status: acpi_status;
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut scope_info: acpi_generic_state = core::mem::zeroed();
    let mut name: *mut i8 = core::ptr::null_mut();

    status = acpi_ns_internalize_name((*original_object).string.pointer, &mut name);
    if ACPI_FAILURE(status) { return status; }
    scope_info.scope.node = scope;
    status = acpi_ns_lookup(
        &mut scope_info, name, ACPI_TYPE_ANY, ACPI_IMODE_EXECUTE,
        ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE,
        core::ptr::null_mut(), &mut node,
    );
    if ACPI_FAILURE(status) {
        ACPI_ERROR_NAMESPACE(&mut scope_info, (*original_object).string.pointer, status);
        ACPI_FREE(name);
        *return_object = new_object;
        return status;
    }
    new_object = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_REFERENCE);
    if new_object.is_null() {
        ACPI_FREE(name);
        *return_object = new_object;
        return AE_NO_MEMORY;
    }
    (*new_object).reference.node = node;
    (*new_object).reference.object = (*node).object;
    (*new_object).reference.class = ACPI_REFCLASS_NAME;
    acpi_ut_add_reference((*node).object);
    ACPI_FREE(name);
    *return_object = new_object;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
