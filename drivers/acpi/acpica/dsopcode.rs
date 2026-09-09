// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Dispatcher support for regions and fields.

// C dependencies are supplied by the surrounding ACPI translation unit.

unsafe extern "C" {
    fn acpi_ns_get_attached_object(handle: acpi_handle) -> *mut acpi_operand_object;
    fn acpi_ev_initialize_region(obj_desc: *mut acpi_operand_object) -> acpi_status;
    fn acpi_ut_get_object_type_name(obj_desc: *mut acpi_operand_object) -> *const core::ffi::c_char;
    fn acpi_ps_get_opcode_name(opcode: u16) -> *const core::ffi::c_char;
    fn acpi_ut_get_descriptor_name(desc: *mut acpi_operand_object) -> *const core::ffi::c_char;
    fn acpi_ex_prep_common_field_object(obj: *mut acpi_operand_object, flags: u8, lock_rule: u8, bit_offset: u32, bit_count: u32) -> acpi_status;
    fn acpi_ut_remove_reference(obj: *mut acpi_operand_object);
    fn acpi_ds_create_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status;
    fn acpi_ex_resolve_operands(opcode: u16, operand_flags: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ut_get_node_name(node: *mut acpi_namespace_node) -> *const core::ffi::c_char;
    fn acpi_ut_add_address_range(space_id: acpi_adr_space_type, address: acpi_physical_address, length: u32, node: *mut acpi_namespace_node) -> acpi_status;
    fn acpi_tb_find_table(sig: *const u8, oem_id: *const u8, oem_table_id: *const u8, index: *mut u32) -> acpi_status;
    fn acpi_get_table_by_index(index: u32, table: *mut *mut acpi_table_header) -> acpi_status;
    fn acpi_ds_create_operand(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, arg: u32) -> acpi_status;
    fn acpi_ds_obj_stack_pop(count: u32, walk_state: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ds_build_internal_buffer_obj(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, length: u32, obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ds_build_internal_package_obj(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, length: u32, obj: *mut *mut acpi_operand_object) -> acpi_status;
    fn acpi_ex_resolve_to_value(obj: *mut *mut acpi_operand_object, walk_state: *mut acpi_walk_state) -> acpi_status;
    fn acpi_ps_get_arg(op: *mut acpi_parse_object, arg: u32) -> *mut acpi_parse_object;
}

// Types and constants below are provided by the translated ACPI headers.
extern "C" {
    type acpi_operand_object;
    type acpi_parse_object;
    type acpi_walk_state;
    type acpi_namespace_node;
    type acpi_table_header;
    type acpi_handle;
    type acpi_status;
    type acpi_adr_space_type;
    type acpi_physical_address;
}

#[allow(non_upper_case_globals, non_snake_case)]
pub unsafe fn acpi_ds_initialize_region(obj_handle: acpi_handle) -> acpi_status {
    let obj_desc = acpi_ns_get_attached_object(obj_handle);
    acpi_ev_initialize_region(obj_desc)
}

#[allow(non_snake_case)]
unsafe fn acpi_ds_init_buffer_field(
    aml_opcode: u16,
    obj_desc: *mut acpi_operand_object,
    buffer_desc: *mut acpi_operand_object,
    offset_desc: *mut acpi_operand_object,
    length_desc: *mut acpi_operand_object,
    result_desc: *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status;
    let offset: u32;
    let bit_offset: u32;
    let bit_count: u32;
    let field_flags: u8;

    // Host object must be a Buffer.
    if (*buffer_desc).common.type_ != ACPI_TYPE_BUFFER {
        status = AE_AML_OPERAND_TYPE;
        acpi_ut_remove_reference(offset_desc);
        acpi_ut_remove_reference(buffer_desc);
        if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
        acpi_ut_remove_reference(result_desc);
        return status;
    }

    // The result began as a name string and must now be a namespace node.
    if ACPI_GET_DESCRIPTOR_TYPE(result_desc) != ACPI_DESC_TYPE_NAMED {
        status = AE_AML_OPERAND_TYPE;
        acpi_ut_remove_reference(offset_desc);
        acpi_ut_remove_reference(buffer_desc);
        if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
        acpi_ut_remove_reference(result_desc);
        return status;
    }

    offset = (*offset_desc).integer.value as u32;
    match aml_opcode {
        AML_CREATE_FIELD_OP => {
            field_flags = AML_FIELD_ACCESS_BYTE;
            bit_offset = offset;
            bit_count = (*length_desc).integer.value as u32;
            if bit_count == 0 {
                status = AE_AML_OPERAND_VALUE;
                acpi_ut_remove_reference(offset_desc); acpi_ut_remove_reference(buffer_desc);
                acpi_ut_remove_reference(length_desc); acpi_ut_remove_reference(result_desc);
                return status;
            }
        }
        AML_CREATE_BIT_FIELD_OP => { bit_offset = offset; bit_count = 1; field_flags = AML_FIELD_ACCESS_BYTE; }
        AML_CREATE_BYTE_FIELD_OP => { bit_offset = 8u32.wrapping_mul(offset); bit_count = 8; field_flags = AML_FIELD_ACCESS_BYTE; }
        AML_CREATE_WORD_FIELD_OP => { bit_offset = 8u32.wrapping_mul(offset); bit_count = 16; field_flags = AML_FIELD_ACCESS_WORD; }
        AML_CREATE_DWORD_FIELD_OP => { bit_offset = 8u32.wrapping_mul(offset); bit_count = 32; field_flags = AML_FIELD_ACCESS_DWORD; }
        AML_CREATE_QWORD_FIELD_OP => { bit_offset = 8u32.wrapping_mul(offset); bit_count = 64; field_flags = AML_FIELD_ACCESS_QWORD; }
        _ => {
            status = AE_AML_BAD_OPCODE;
            acpi_ut_remove_reference(offset_desc); acpi_ut_remove_reference(buffer_desc);
            if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
            acpi_ut_remove_reference(result_desc);
            return status;
        }
    }

    if bit_offset.wrapping_add(bit_count) > 8u32.wrapping_mul((*buffer_desc).buffer.length as u32) {
        status = AE_AML_BUFFER_LIMIT;
        acpi_ut_remove_reference(offset_desc); acpi_ut_remove_reference(buffer_desc);
        if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
        acpi_ut_remove_reference(result_desc);
        return status;
    }

    status = acpi_ex_prep_common_field_object(obj_desc, field_flags, 0, bit_offset, bit_count);
    if ACPI_FAILURE(status) {
        acpi_ut_remove_reference(offset_desc); acpi_ut_remove_reference(buffer_desc);
        if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
        acpi_ut_remove_reference(result_desc);
        return status;
    }
    (*obj_desc).buffer_field.buffer_obj = buffer_desc;
    (*obj_desc).buffer_field.is_create_field = (aml_opcode == AML_CREATE_FIELD_OP);
    (*buffer_desc).common.reference_count = (*buffer_desc).common.reference_count.wrapping_add((*obj_desc).common.reference_count);
    acpi_ut_remove_reference(offset_desc);
    acpi_ut_remove_reference(buffer_desc);
    if aml_opcode == AML_CREATE_FIELD_OP { acpi_ut_remove_reference(length_desc); }
    (*obj_desc).buffer_field.flags |= AOPOBJ_DATA_VALID;
    status
}

pub unsafe fn acpi_ds_eval_buffer_field_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status {
    let node = (*op).common.node;
    let next_op = (*op).common.value.arg;
    let mut status = acpi_ds_create_operands(walk_state, next_op);
    if ACPI_FAILURE(status) { return status; }
    let obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() { return AE_NOT_EXIST; }
    status = acpi_ex_resolve_operands((*op).common.aml_opcode, ACPI_WALK_OPERANDS, walk_state);
    if ACPI_FAILURE(status) { return status; }
    if (*op).common.aml_opcode == AML_CREATE_FIELD_OP {
        acpi_ds_init_buffer_field((*op).common.aml_opcode, obj_desc, (*walk_state).operands[0], (*walk_state).operands[1], (*walk_state).operands[2], (*walk_state).operands[3])
    } else {
        acpi_ds_init_buffer_field((*op).common.aml_opcode, obj_desc, (*walk_state).operands[0], (*walk_state).operands[1], core::ptr::null_mut(), (*walk_state).operands[2])
    }
}

pub unsafe fn acpi_ds_eval_region_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status {
    let node = (*op).common.node;
    let mut next_op = (*op).common.value.arg;
    let space_id = (*next_op).common.value.integer as acpi_adr_space_type;
    next_op = (*next_op).common.next;
    let mut status = acpi_ds_create_operands(walk_state, next_op);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ex_resolve_operands((*op).common.aml_opcode, ACPI_WALK_OPERANDS, walk_state);
    if ACPI_FAILURE(status) { return status; }
    let obj_desc = acpi_ns_get_attached_object(node);
    if obj_desc.is_null() { return AE_NOT_EXIST; }
    let operand_desc = (*walk_state).operands[(*walk_state).num_operands - 1];
    (*obj_desc).region.length = (*operand_desc).integer.value as u32;
    acpi_ut_remove_reference(operand_desc);
    let operand_desc = (*walk_state).operands[(*walk_state).num_operands - 2];
    (*obj_desc).region.address = (*operand_desc).integer.value as acpi_physical_address;
    acpi_ut_remove_reference(operand_desc);
    status = acpi_ut_add_address_range((*obj_desc).region.space_id, (*obj_desc).region.address, (*obj_desc).region.length, node);
    (*obj_desc).region.flags |= AOPOBJ_DATA_VALID;
    status
}

pub unsafe fn acpi_ds_eval_table_region_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status {
    let node = (*op).common.node;
    let next_op = (*op).common.value.arg;
    let mut status = acpi_ds_create_operands(walk_state, next_op);
    if ACPI_FAILURE(status) { return status; }
    let operand = (*walk_state).operands.as_mut_ptr();
    status = acpi_ex_resolve_operands((*op).common.aml_opcode, ACPI_WALK_OPERANDS, walk_state);
    if ACPI_FAILURE(status) { for i in 0..3 { acpi_ut_remove_reference(*operand.add(i)); } return status; }
    let mut table_index = 0u32;
    status = acpi_tb_find_table((*operand).string.pointer, (*operand.add(1)).string.pointer, (*operand.add(2)).string.pointer, &mut table_index);
    if ACPI_SUCCESS(status) {
        let mut table = core::ptr::null_mut();
        status = acpi_get_table_by_index(table_index, &mut table);
        if ACPI_SUCCESS(status) {
            let obj_desc = acpi_ns_get_attached_object(node);
            if obj_desc.is_null() { status = AE_NOT_EXIST; } else {
                (*obj_desc).region.address = ACPI_PTR_TO_PHYSADDR(table);
                (*obj_desc).region.length = (*table).length;
                (*obj_desc).region.pointer = table;
                (*obj_desc).region.flags |= AOPOBJ_DATA_VALID;
            }
        }
    }
    for i in 0..3 { acpi_ut_remove_reference(*operand.add(i)); }
    status
}

pub unsafe fn acpi_ds_eval_data_object_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object, obj_desc: *mut acpi_operand_object) -> acpi_status {
    (*walk_state).operand_index = (*walk_state).num_operands;
    if (*op).common.value.arg.is_null() { return AE_OK; }
    let mut status = acpi_ds_create_operand(walk_state, (*op).common.value.arg, 1);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ex_resolve_operands((*walk_state).opcode, (*walk_state).operands.as_mut_ptr().add((*walk_state).num_operands - 1), walk_state);
    if ACPI_FAILURE(status) { return status; }
    let arg_desc = (*walk_state).operands[(*walk_state).num_operands - 1];
    let length = (*arg_desc).integer.value as u32;
    status = acpi_ds_obj_stack_pop(1, walk_state);
    if ACPI_FAILURE(status) { return status; }
    acpi_ut_remove_reference(arg_desc);
    match (*op).common.aml_opcode {
        AML_BUFFER_OP => status = acpi_ds_build_internal_buffer_obj(walk_state, op, length, &mut (obj_desc as *mut _)),
        AML_PACKAGE_OP | AML_VARIABLE_PACKAGE_OP => status = acpi_ds_build_internal_package_obj(walk_state, op, length, &mut (obj_desc as *mut _)),
        _ => return AE_AML_BAD_OPCODE,
    }
    if ACPI_SUCCESS(status) { (*walk_state).result_obj = obj_desc; }
    status
}

pub unsafe fn acpi_ds_eval_bank_field_operands(walk_state: *mut acpi_walk_state, op: *mut acpi_parse_object) -> acpi_status {
    let mut next_op = (*op).common.value.arg;
    next_op = (*next_op).common.next;
    next_op = (*next_op).common.next;
    (*walk_state).operand_index = 0;
    let mut status = acpi_ds_create_operand(walk_state, next_op, 0);
    if ACPI_FAILURE(status) { return status; }
    status = acpi_ex_resolve_to_value(&mut (*walk_state).operands[0], walk_state);
    if ACPI_FAILURE(status) { return status; }
    let operand_desc = (*walk_state).operands[0];
    let mut arg = acpi_ps_get_arg(op, 4);
    while !arg.is_null() {
        if (*arg).common.aml_opcode == AML_INT_NAMEDFIELD_OP {
            let obj_desc = acpi_ns_get_attached_object((*arg).common.node);
            if obj_desc.is_null() { return AE_NOT_EXIST; }
            (*obj_desc).bank_field.value = (*operand_desc).integer.value as u32;
        }
        arg = (*arg).common.next;
    }
    acpi_ut_remove_reference(operand_desc);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
