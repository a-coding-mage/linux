// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: exprep - ACPI AML field prep utilities

// Dependencies are supplied by the surrounding ACPI translation unit.

// Local prototypes
unsafe fn acpi_ex_decode_field_access(
    obj_desc: *mut acpi_operand_object,
    field_flags: u8,
    return_byte_alignment: *mut u32,
) -> u32;

// ACPI_UNDER_DEVELOPMENT is a build-time condition in the original source.
#[cfg(feature = "ACPI_UNDER_DEVELOPMENT")]
unsafe fn acpi_ex_generate_access(
    field_bit_offset: u32,
    field_bit_length: u32,
    region_length: u32,
) -> u32 {
    let field_byte_offset = (field_bit_offset & !7) / 8;
    let field_byte_end_offset = (field_bit_length + field_bit_offset + 7) / 8;
    let field_byte_length = field_byte_end_offset - field_byte_offset;
    let mut minimum_access_width = 0xffff_ffffu32;
    let mut minimum_accesses = 0xffff_ffffu32;
    let mut access_byte_width = 1u32;

    while access_byte_width <= 8 {
        let rounded_end = ((field_byte_end_offset + access_byte_width - 1)
            / access_byte_width)
            * access_byte_width;
        if rounded_end <= region_length {
            let field_start_offset =
                (field_byte_offset / access_byte_width);
            let field_end_offset =
                ((field_byte_length + field_byte_offset + access_byte_width - 1)
                    / access_byte_width);
            let accesses = field_end_offset - field_start_offset;
            if accesses <= 1 {
                return access_byte_width;
            }
            if accesses < minimum_accesses {
                minimum_accesses = accesses;
                minimum_access_width = access_byte_width;
            }
        } else {
            if access_byte_width == 1 {
                return 0;
            }
            return minimum_access_width;
        }
        access_byte_width <<= 1;
    }
    8
}

unsafe fn acpi_ex_decode_field_access(
    obj_desc: *mut acpi_operand_object,
    field_flags: u8,
    return_byte_alignment: *mut u32,
) -> u32 {
    let access = (field_flags as u32) & AML_FIELD_ACCESS_TYPE_MASK;
    let (mut byte_alignment, bit_length) = match access {
        AML_FIELD_ACCESS_ANY => {
            #[cfg(feature = "ACPI_UNDER_DEVELOPMENT")]
            {
                let _ = acpi_ex_generate_access(
                    (*obj_desc).common_field.start_field_bit_offset as u32,
                    (*obj_desc).common_field.bit_length,
                    0xffff_ffff,
                );
            }
            (1, 8)
        }
        AML_FIELD_ACCESS_BYTE | AML_FIELD_ACCESS_BUFFER => (1, 8),
        AML_FIELD_ACCESS_WORD => (2, 16),
        AML_FIELD_ACCESS_DWORD => (4, 32),
        AML_FIELD_ACCESS_QWORD => (8, 64),
        _ => return 0,
    };

    if (*obj_desc).common.type_ == ACPI_TYPE_BUFFER_FIELD {
        byte_alignment = 1;
    }
    *return_byte_alignment = byte_alignment;
    bit_length
}

pub unsafe fn acpi_ex_prep_common_field_object(
    obj_desc: *mut acpi_operand_object,
    field_flags: u8,
    field_attribute: u8,
    field_bit_position: u32,
    field_bit_length: u32,
) -> acpi_status {
    let mut byte_alignment = 0u32;
    (*obj_desc).common_field.field_flags = field_flags;
    (*obj_desc).common_field.attribute = field_attribute;
    (*obj_desc).common_field.bit_length = field_bit_length;
    let access_bit_width =
        acpi_ex_decode_field_access(obj_desc, field_flags, &mut byte_alignment);
    if access_bit_width == 0 {
        return AE_AML_OPERAND_VALUE;
    }
    (*obj_desc).common_field.access_byte_width = (access_bit_width / 8) as u8;
    let nearest_byte_address = field_bit_position / 8;
    (*obj_desc).common_field.base_byte_offset =
        (nearest_byte_address / byte_alignment) * byte_alignment;
    (*obj_desc).common_field.start_field_bit_offset =
        (field_bit_position - 8 * (*obj_desc).common_field.base_byte_offset) as u8;
    AE_OK
}

pub unsafe fn acpi_ex_prep_field_value(info: *mut acpi_create_field_info) -> acpi_status {
    let mut obj_desc: *mut acpi_operand_object;
    let mut second_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut status: acpi_status;
    let type_: u32;

    if (*info).field_type != ACPI_TYPE_LOCAL_INDEX_FIELD {
        if (*info).region_node.is_null() {
            return AE_AML_NO_OPERAND;
        }
        type_ = acpi_ns_get_type((*info).region_node);
        if type_ != ACPI_TYPE_REGION {
            return AE_AML_OPERAND_TYPE;
        }
    }

    obj_desc = acpi_ut_create_internal_object((*info).field_type);
    if obj_desc.is_null() {
        return AE_NO_MEMORY;
    }
    (*obj_desc).common_field.node = (*info).field_node;
    status = acpi_ex_prep_common_field_object(
        obj_desc, (*info).field_flags, (*info).attribute,
        (*info).field_bit_position, (*info).field_bit_length,
    );
    if ACPI_FAILURE(status) {
        acpi_ut_delete_object_desc(obj_desc);
        return status;
    }

    match (*info).field_type {
        ACPI_TYPE_LOCAL_REGION_FIELD => {
            (*obj_desc).field.region_obj = acpi_ns_get_attached_object((*info).region_node);
            (*obj_desc).field.access_length = (*info).access_length;
            if !(*info).connection_node.is_null() {
                second_desc = (*(*info).connection_node).object;
                if !second_desc.is_null() {
                    if ((*second_desc).common.flags & AOPOBJ_DATA_VALID) == 0 {
                        status = acpi_ds_get_buffer_arguments(second_desc);
                        if ACPI_FAILURE(status) {
                            acpi_ut_delete_object_desc(obj_desc);
                            return status;
                        }
                    }
                    (*obj_desc).field.resource_buffer = (*second_desc).buffer.pointer;
                    (*obj_desc).field.resource_length = (*second_desc).buffer.length as u16;
                }
            } else if !(*info).resource_buffer.is_null() {
                (*obj_desc).field.resource_buffer = (*info).resource_buffer;
                (*obj_desc).field.resource_length = (*info).resource_length;
            }
            (*obj_desc).field.pin_number_index = (*info).pin_number_index;
            if (*obj_desc).field.region_obj.space_id == ACPI_ADR_SPACE_EC
                && (*obj_desc).common_field.bit_length > 8
            {
                let width = ((*obj_desc).common_field.bit_length + 7) / 8;
                if width < 256 { (*obj_desc).common_field.access_byte_width = width as u8; }
            }
        }
        ACPI_TYPE_LOCAL_BANK_FIELD => {
            (*obj_desc).bank_field.value = (*info).bank_value;
            (*obj_desc).bank_field.region_obj = acpi_ns_get_attached_object((*info).region_node);
            (*obj_desc).bank_field.bank_obj = acpi_ns_get_attached_object((*info).register_node);
            acpi_ut_add_reference((*obj_desc).bank_field.region_obj);
            acpi_ut_add_reference((*obj_desc).bank_field.bank_obj);
            second_desc = (*obj_desc).common.next_object;
            (*second_desc).extra.aml_start = (*info).data_register_node;
        }
        ACPI_TYPE_LOCAL_INDEX_FIELD => {
            (*obj_desc).index_field.index_obj = acpi_ns_get_attached_object((*info).register_node);
            (*obj_desc).index_field.data_obj = acpi_ns_get_attached_object((*info).data_register_node);
            if (*obj_desc).index_field.data_obj.is_null() || (*obj_desc).index_field.index_obj.is_null() {
                acpi_ut_delete_object_desc(obj_desc);
                return AE_AML_INTERNAL;
            }
            acpi_ut_add_reference((*obj_desc).index_field.data_obj);
            acpi_ut_add_reference((*obj_desc).index_field.index_obj);
            (*obj_desc).index_field.value =
                (((*info).field_bit_position / 8) / (*obj_desc).index_field.access_byte_width as u32)
                    * (*obj_desc).index_field.access_byte_width as u32;
        }
        _ => {}
    }
    status = acpi_ns_attach_object((*info).field_node, obj_desc, acpi_ns_get_type((*info).field_node));
    acpi_ut_remove_reference(obj_desc);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
