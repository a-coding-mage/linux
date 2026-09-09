// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: exconcat - Concatenate-type AML operators
//
// Copyright (C) 2000 - 2026, Intel Corp.

// ACPICA dependencies supplied by other translation units.

static unsafe fn acpi_ex_convert_to_object_type_string(
    obj_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut return_desc: *mut acpi_operand_object;
    let type_string: *const core::ffi::c_char;

    type_string = acpi_ut_get_type_name((*obj_desc).common.type_);

    return_desc = acpi_ut_create_string_object(
        (strlen(type_string) + 9) as acpi_size,
    ); // 9 For "[ Object]"
    if return_desc.is_null() {
        return AE_NO_MEMORY;
    }

    strcpy((*return_desc).string.pointer, c"[".as_ptr());
    strcat((*return_desc).string.pointer, type_string);
    strcat((*return_desc).string.pointer, c" Object]".as_ptr());

    *result_desc = return_desc;
    AE_OK
}

pub unsafe fn acpi_ex_do_concatenate(
    operand0: *mut acpi_operand_object,
    operand1: *mut acpi_operand_object,
    actual_return_desc: *mut *mut acpi_operand_object,
    _walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut local_operand0 = operand0;
    let mut local_operand1 = operand1;
    let mut temp_operand1: *mut acpi_operand_object = core::ptr::null_mut();
    let return_desc: *mut acpi_operand_object;
    let buffer: *mut core::ffi::c_char;
    let operand0_type: acpi_object_type;
    let operand1_type: acpi_object_type;
    let mut status: acpi_status;

    match (*operand0).common.type_ {
        ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => {
            operand0_type = (*operand0).common.type_;
        }
        _ => {
            status = acpi_ex_convert_to_object_type_string(operand0, &mut local_operand0);
            if ACPI_FAILURE(status) { return cleanup_concat(status, operand0, operand1, local_operand0, local_operand1); }
            operand0_type = ACPI_TYPE_STRING;
        }
    }

    match (*operand1).common.type_ {
        ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => {
            operand1_type = (*operand1).common.type_;
        }
        _ => {
            status = acpi_ex_convert_to_object_type_string(operand1, &mut local_operand1);
            if ACPI_FAILURE(status) { return cleanup_concat(status, operand0, operand1, local_operand0, local_operand1); }
            operand1_type = ACPI_TYPE_STRING;
        }
    }

    status = match operand0_type {
        ACPI_TYPE_INTEGER => acpi_ex_convert_to_integer(local_operand1, &mut temp_operand1, ACPI_IMPLICIT_CONVERSION),
        ACPI_TYPE_BUFFER => acpi_ex_convert_to_buffer(local_operand1, &mut temp_operand1),
        ACPI_TYPE_STRING => acpi_ex_convert_to_string(local_operand1, &mut temp_operand1, ACPI_IMPLICIT_CONVERT_HEX),
        _ => AE_AML_INTERNAL,
    };
    if ACPI_FAILURE(status) { return cleanup_concat(status, operand0, operand1, local_operand0, local_operand1); }

    if local_operand1 != operand1 && local_operand1 != temp_operand1 { acpi_ut_remove_reference(local_operand1); }
    local_operand1 = temp_operand1;

    return_desc = match operand0_type {
        ACPI_TYPE_INTEGER => {
            let p = acpi_ut_create_buffer_object((2 * acpi_gbl_integer_byte_width) as acpi_size);
            if p.is_null() { return cleanup_concat(AE_NO_MEMORY, operand0, operand1, local_operand0, local_operand1); }
            core::ptr::copy_nonoverlapping(&(*operand0).integer.value as *const _, (*p).buffer.pointer, acpi_gbl_integer_byte_width);
            core::ptr::copy_nonoverlapping(&(*local_operand1).integer.value as *const _, (*p).buffer.pointer.add(acpi_gbl_integer_byte_width), acpi_gbl_integer_byte_width);
            p
        }
        ACPI_TYPE_STRING => {
            let p = acpi_ut_create_string_object(((*local_operand0).string.length + (*local_operand1).string.length) as acpi_size);
            if p.is_null() { return cleanup_concat(AE_NO_MEMORY, operand0, operand1, local_operand0, local_operand1); }
            strcpy((*p).string.pointer, (*local_operand0).string.pointer);
            strcat((*p).string.pointer, (*local_operand1).string.pointer);
            p
        }
        ACPI_TYPE_BUFFER => {
            let p = acpi_ut_create_buffer_object(((*operand0).buffer.length + (*local_operand1).buffer.length) as acpi_size);
            if p.is_null() { return cleanup_concat(AE_NO_MEMORY, operand0, operand1, local_operand0, local_operand1); }
            core::ptr::copy_nonoverlapping((*operand0).buffer.pointer, (*p).buffer.pointer, (*operand0).buffer.length);
            core::ptr::copy_nonoverlapping((*local_operand1).buffer.pointer, (*p).buffer.pointer.add((*operand0).buffer.length), (*local_operand1).buffer.length);
            p
        }
        _ => return cleanup_concat(AE_AML_INTERNAL, operand0, operand1, local_operand0, local_operand1),
    };

    *actual_return_desc = return_desc;
    cleanup_concat(AE_OK, operand0, operand1, local_operand0, local_operand1)
}

unsafe fn cleanup_concat(status: acpi_status, operand0: *mut acpi_operand_object, operand1: *mut acpi_operand_object, local_operand0: *mut acpi_operand_object, local_operand1: *mut acpi_operand_object) -> acpi_status {
    if local_operand0 != operand0 { acpi_ut_remove_reference(local_operand0); }
    if local_operand1 != operand1 { acpi_ut_remove_reference(local_operand1); }
    status
}

pub unsafe fn acpi_ex_concat_template(operand0: *mut acpi_operand_object, operand1: *mut acpi_operand_object, actual_return_desc: *mut *mut acpi_operand_object, _walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut end_tag: *mut u8 = core::ptr::null_mut();
    let status = acpi_ut_get_resource_end_tag(operand0, &mut end_tag);
    if ACPI_FAILURE(status) { return status; }
    let length0 = end_tag.offset_from((*operand0).buffer.pointer) as acpi_size;
    let status = acpi_ut_get_resource_end_tag(operand1, &mut end_tag);
    if ACPI_FAILURE(status) { return status; }
    let length1 = end_tag.offset_from((*operand1).buffer.pointer) as acpi_size;
    let new_length = length0 + length1 + core::mem::size_of::<aml_resource_end_tag>();
    let return_desc = acpi_ut_create_buffer_object(new_length);
    if return_desc.is_null() { return AE_NO_MEMORY; }
    let new_buf = (*return_desc).buffer.pointer;
    core::ptr::copy_nonoverlapping((*operand0).buffer.pointer, new_buf, length0);
    core::ptr::copy_nonoverlapping((*operand1).buffer.pointer, new_buf.add(length0), length1);
    *new_buf.add(new_length - 1) = 0;
    *new_buf.add(new_length - 2) = ACPI_RESOURCE_NAME_END_TAG | 1;
    *actual_return_desc = return_desc;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
