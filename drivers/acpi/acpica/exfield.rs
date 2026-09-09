// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: exfield - AML execution - field_unit read/write
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

use core::ffi::c_void;

// ACPICA headers and externally supplied symbols are dependencies of this translation.

const ACPI_INVALID_PROTOCOL_ID: u8 = 0x80;
const ACPI_MAX_PROTOCOL_ID: u32 = 0x0F;
static ACPI_PROTOCOL_LENGTHS: [u8; 16] = [
    ACPI_INVALID_PROTOCOL_ID, ACPI_INVALID_PROTOCOL_ID, 0x00,
    ACPI_INVALID_PROTOCOL_ID, 0x01, ACPI_INVALID_PROTOCOL_ID, 0x01,
    ACPI_INVALID_PROTOCOL_ID, 0x02, ACPI_INVALID_PROTOCOL_ID, 0xFF, 0xFF,
    0x02, 0xFF, 0xFF, 0xFF,
];

const PCC_MASTER_SUBSPACE: u32 = 3;

#[inline]
const fn generic_subspace_command(a: u32) -> bool { 4 == a || a == 5 }
#[inline]
const fn master_subspace_command(a: u32) -> bool { 12 <= a && a <= 15 }

pub unsafe fn acpi_ex_get_protocol_buffer_length(
    protocol_id: u32,
    return_length: *mut u32,
) -> acpi_status {
    if protocol_id > ACPI_MAX_PROTOCOL_ID
        || ACPI_PROTOCOL_LENGTHS[protocol_id as usize] == ACPI_INVALID_PROTOCOL_ID
    {
        ACPI_ERROR!(AE_INFO, "Invalid Field/AccessAs protocol ID: 0x{:4.4X}", protocol_id);
        return AE_AML_PROTOCOL;
    }

    *return_length = ACPI_PROTOCOL_LENGTHS[protocol_id as usize] as u32;
    AE_OK
}

pub unsafe fn acpi_ex_read_data_from_field(
    walk_state: *mut acpi_walk_state,
    obj_desc: *mut acpi_operand_object,
    ret_buffer_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status;
    let buffer_desc: *mut acpi_operand_object;
    let buffer: *mut c_void;
    let mut buffer_length: u32;

    ACPI_FUNCTION_TRACE_PTR!(ex_read_data_from_field, obj_desc);

    if obj_desc.is_null() { return AE_AML_NO_OPERAND; }
    if ret_buffer_desc.is_null() { return AE_BAD_PARAMETER; }

    if (*obj_desc).common.type_ == ACPI_TYPE_BUFFER_FIELD {
        if (*obj_desc).common.flags & AOPOBJ_DATA_VALID == 0 {
            status = acpi_ds_get_buffer_field_arguments(obj_desc);
            if ACPI_FAILURE!(status) { return status; }
        }
    } else if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && ((*obj_desc).field.region_obj).as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_SMBUS
        || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
            && ((*obj_desc).field.region_obj).as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_GSBUS
        || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
            && ((*obj_desc).field.region_obj).as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_IPMI
        || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
            && ((*obj_desc).field.region_obj).as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_PLATFORM_RT
        || (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
            && ((*obj_desc).field.region_obj).as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_FIXED_HARDWARE {
        status = acpi_ex_read_serial_bus(obj_desc, ret_buffer_desc);
        return status;
    }

    buffer_length = ACPI_ROUND_BITS_UP_TO_BYTES!((*obj_desc).field.bit_length);
    if buffer_length > acpi_gbl_integer_byte_width
        || ((*obj_desc).common.type_ == ACPI_TYPE_BUFFER_FIELD
            && (*obj_desc).buffer_field.is_create_field != 0)
    {
        buffer_desc = acpi_ut_create_buffer_object(buffer_length);
        if buffer_desc.is_null() { return AE_NO_MEMORY; }
        buffer = (*buffer_desc).buffer.pointer as *mut c_void;
    } else {
        buffer_desc = acpi_ut_create_integer_object(0);
        if buffer_desc.is_null() { return AE_NO_MEMORY; }
        buffer_length = acpi_gbl_integer_byte_width;
        buffer = &mut (*buffer_desc).integer.value as *mut u64 as *mut c_void;
    }

    if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && (*obj_desc).field.region_obj.as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_GPIO {
        status = acpi_ex_read_gpio(obj_desc, buffer);
    } else if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && (*obj_desc).field.region_obj.as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_PLATFORM_COMM {
        ACPI_DEBUG_PRINT!(ACPI_DB_BFIELD, "PCC FieldRead bits {}\n", (*obj_desc).field.bit_length);
        core::ptr::copy_nonoverlapping(
            (*obj_desc).field.region_obj.as_ref().unwrap().field.internal_pcc_buffer
                .add((*obj_desc).field.base_byte_offset as usize),
            buffer as *mut u8,
            ACPI_ROUND_BITS_UP_TO_BYTES!((*obj_desc).field.bit_length) as usize,
        );
        *ret_buffer_desc = buffer_desc;
        return AE_OK;
    } else {
        ACPI_DEBUG_PRINT!(ACPI_DB_BFIELD, "FieldRead [TO]: Obj {:?}, Type {:X}, Buf {:?}, ByteLen {:X}\n", obj_desc, (*obj_desc).common.type_, buffer, buffer_length);
        ACPI_DEBUG_PRINT!(ACPI_DB_BFIELD, "FieldRead [FROM]: BitLen {:X}, BitOff {:X}, ByteOff {:X}\n", (*obj_desc).common_field.bit_length, (*obj_desc).common_field.start_field_bit_offset, (*obj_desc).common_field.base_byte_offset);
        acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
        status = acpi_ex_extract_from_field(obj_desc, buffer, buffer_length);
        acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    }

    if ACPI_FAILURE!(status) { acpi_ut_remove_reference(buffer_desc); }
    else { *ret_buffer_desc = buffer_desc; }
    status
}

pub unsafe fn acpi_ex_write_data_to_field(
    source_desc: *mut acpi_operand_object,
    obj_desc: *mut acpi_operand_object,
    result_desc: *mut *mut acpi_operand_object,
) -> acpi_status {
    let status: acpi_status;
    let mut buffer_length: u32;
    let mut data_length: u32;
    let buffer: *mut c_void;

    ACPI_FUNCTION_TRACE_PTR!(ex_write_data_to_field, obj_desc);
    if source_desc.is_null() || obj_desc.is_null() { return AE_AML_NO_OPERAND; }

    if (*obj_desc).common.type_ == ACPI_TYPE_BUFFER_FIELD {
        if (*obj_desc).common.flags & AOPOBJ_DATA_VALID == 0 {
            let s = acpi_ds_get_buffer_field_arguments(obj_desc);
            if ACPI_FAILURE!(s) { return s; }
        }
    } else if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && (*obj_desc).field.region_obj.as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_GPIO {
        status = acpi_ex_write_gpio(source_desc, obj_desc, result_desc);
        return status;
    } else if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && matches!((*obj_desc).field.region_obj.as_ref().unwrap().region.space_id, ACPI_ADR_SPACE_SMBUS | ACPI_ADR_SPACE_GSBUS | ACPI_ADR_SPACE_IPMI | ACPI_ADR_SPACE_PLATFORM_RT | ACPI_ADR_SPACE_FIXED_HARDWARE) {
        status = acpi_ex_write_serial_bus(source_desc, obj_desc, result_desc);
        return status;
    } else if (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_REGION_FIELD
        && (*obj_desc).field.region_obj.as_ref().unwrap().region.space_id == ACPI_ADR_SPACE_PLATFORM_COMM {
        data_length = ACPI_ROUND_BITS_UP_TO_BYTES!((*obj_desc).field.bit_length);
        core::ptr::copy_nonoverlapping((*source_desc).buffer.pointer, (*obj_desc).field.region_obj.as_ref().unwrap().field.internal_pcc_buffer.add((*obj_desc).field.base_byte_offset as usize), data_length as usize);
        if master_subspace_command((*obj_desc).field.base_byte_offset) {
            return acpi_ex_access_region(obj_desc, 0, (*obj_desc).field.region_obj.as_ref().unwrap().field.internal_pcc_buffer as *mut u64, ACPI_WRITE);
        }
        return AE_OK;
    }

    buffer_length = match (*source_desc).common.type_ {
        ACPI_TYPE_INTEGER => { buffer = &mut (*source_desc).integer.value as *mut u64 as *mut c_void; core::mem::size_of::<u64>() as u32 }
        ACPI_TYPE_BUFFER => { buffer = (*source_desc).buffer.pointer as *mut c_void; (*source_desc).buffer.length }
        ACPI_TYPE_STRING => { buffer = (*source_desc).string.pointer as *mut c_void; (*source_desc).string.length }
        _ => return AE_AML_OPERAND_TYPE,
    };
    acpi_ex_acquire_global_lock((*obj_desc).common_field.field_flags);
    status = acpi_ex_insert_into_field(obj_desc, buffer, buffer_length);
    acpi_ex_release_global_lock((*obj_desc).common_field.field_flags);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
