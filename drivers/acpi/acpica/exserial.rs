// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exserial - field_unit support for serial address spaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ex_read_gpio(
    obj_desc: *mut acpi_operand_object,
    buffer: *mut core::ffi::c_void,
) -> acpi_status {
    // For GPIO, Address is the bit offset from the previous Connection(),
    // effectively a pin number index. bit_length is the number of pins.
    acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
    let status = acpi_ex_access_region(obj_desc, 0, buffer as *mut u64, ACPI_READ);
    acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    status
}

pub unsafe fn acpi_ex_write_gpio(
    source_desc: *mut acpi_operand_object,
    obj_desc: *mut acpi_operand_object,
    _return_buffer: *mut *mut acpi_operand_object,
) -> acpi_status {
    // For GPIO, bypass the field mechanism and pass the bit address and width
    // directly to the handler.
    if (*source_desc).common.type_ != ACPI_TYPE_INTEGER {
        return AE_AML_OPERAND_TYPE;
    }

    let buffer = &mut (*source_desc).integer.value as *mut u64;
    acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
    let status = acpi_ex_access_region(obj_desc, 0, buffer, ACPI_WRITE);
    acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    status
}

pub unsafe fn acpi_ex_read_serial_bus(
    obj_desc: *mut acpi_operand_object,
    return_buffer: *mut *mut acpi_operand_object,
) -> acpi_status {
    let buffer_length: u32;
    let function: u32;
    let accessor_type: u16;

    match (*obj_desc).field.region_obj.region.space_id {
        ACPI_ADR_SPACE_SMBUS => {
            buffer_length = ACPI_SMBUS_BUFFER_SIZE;
            function = ACPI_READ | ((*obj_desc).field.attribute << 16);
        }
        ACPI_ADR_SPACE_IPMI => {
            buffer_length = ACPI_IPMI_BUFFER_SIZE;
            function = ACPI_READ;
        }
        ACPI_ADR_SPACE_GSBUS => {
            accessor_type = (*obj_desc).field.attribute;
            if accessor_type == AML_FIELD_ATTRIB_RAW_PROCESS_BYTES {
                return AE_AML_PROTOCOL;
            }
            let mut length = 0u32;
            let status = acpi_ex_get_protocol_buffer_length(accessor_type, &mut length);
            if ACPI_FAILURE(status) {
                return status;
            }
            buffer_length = length + ACPI_SERIAL_HEADER_SIZE;
            function = ACPI_READ | ((accessor_type as u32) << 16);
        }
        ACPI_ADR_SPACE_PLATFORM_RT => {
            buffer_length = ACPI_PRM_INPUT_BUFFER_SIZE;
            function = ACPI_READ;
        }
        ACPI_ADR_SPACE_FIXED_HARDWARE => {
            buffer_length = ACPI_FFH_INPUT_BUFFER_SIZE;
            function = ACPI_READ;
        }
        _ => return AE_AML_INVALID_SPACE_ID,
    }

    let buffer_desc = acpi_ut_create_buffer_object(buffer_length);
    if buffer_desc.is_null() {
        return AE_NO_MEMORY;
    }
    acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
    let status = acpi_ex_access_region(
        obj_desc,
        0,
        (*buffer_desc).buffer.pointer as *mut u64,
        function,
    );
    acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    *return_buffer = buffer_desc;
    status
}

pub unsafe fn acpi_ex_write_serial_bus(
    source_desc: *mut acpi_operand_object,
    obj_desc: *mut acpi_operand_object,
    return_buffer: *mut *mut acpi_operand_object,
) -> acpi_status {
    if (*source_desc).common.type_ != ACPI_TYPE_BUFFER {
        return AE_AML_OPERAND_TYPE;
    }

    let buffer_length: u32;
    let function: u32;
    let accessor_type: u16;
    match (*obj_desc).field.region_obj.region.space_id {
        ACPI_ADR_SPACE_SMBUS => {
            buffer_length = ACPI_SMBUS_BUFFER_SIZE;
            function = ACPI_WRITE | ((*obj_desc).field.attribute << 16);
        }
        ACPI_ADR_SPACE_IPMI => {
            buffer_length = ACPI_IPMI_BUFFER_SIZE;
            function = ACPI_WRITE;
        }
        ACPI_ADR_SPACE_GSBUS => {
            accessor_type = (*obj_desc).field.attribute;
            let mut length = 0u32;
            let status = acpi_ex_get_protocol_buffer_length(accessor_type, &mut length);
            if ACPI_FAILURE(status) {
                return status;
            }
            buffer_length = length + ACPI_SERIAL_HEADER_SIZE;
            function = ACPI_WRITE | ((accessor_type as u32) << 16);
        }
        ACPI_ADR_SPACE_PLATFORM_RT => {
            buffer_length = ACPI_PRM_INPUT_BUFFER_SIZE;
            function = ACPI_WRITE;
        }
        ACPI_ADR_SPACE_FIXED_HARDWARE => {
            buffer_length = ACPI_FFH_INPUT_BUFFER_SIZE;
            function = ACPI_WRITE;
        }
        _ => return AE_AML_INVALID_SPACE_ID,
    }

    let buffer_desc = acpi_ut_create_buffer_object(buffer_length);
    if buffer_desc.is_null() {
        return AE_NO_MEMORY;
    }
    let buffer = (*buffer_desc).buffer.pointer;
    let data_length = core::cmp::min(buffer_length, (*source_desc).buffer.length);
    core::ptr::copy_nonoverlapping(
        (*source_desc).buffer.pointer,
        buffer,
        data_length as usize,
    );
    acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
    let status = acpi_ex_access_region(obj_desc, 0, buffer as *mut u64, function);
    acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    *return_buffer = buffer_desc;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
