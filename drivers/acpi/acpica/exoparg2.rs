// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// AML execution - opcodes with 2 arguments.

pub unsafe fn acpi_ex_opcode_2A_0T_0R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut status = AE_OK;

    match (*walk_state).opcode {
        AML_NOTIFY_OP => {
            let node = operand as *mut acpi_namespace_node;
            let value = (*(*operand.add(1))).integer.value as u32;
            if !acpi_ev_is_notify_object(node) {
                ACPI_ERROR!((AE_INFO, "Unexpected notify object type [{}]", acpi_ut_get_type_name((*node).type_)));
                status = AE_AML_OPERAND_TYPE;
            } else {
                status = acpi_ev_queue_notify_request(node, value);
            }
        }
        _ => {
            ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x{:X}", (*walk_state).opcode));
            status = AE_AML_BAD_OPCODE;
        }
    }
    status
}

pub unsafe fn acpi_ex_opcode_2A_2T_1R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut return_desc1: *mut acpi_operand_object = core::ptr::null_mut();
    let mut return_desc2: *mut acpi_operand_object = core::ptr::null_mut();
    let mut status;
    match (*walk_state).opcode {
        AML_DIVIDE_OP => {
            return_desc1 = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
            if return_desc1.is_null() { status = AE_NO_MEMORY; } else {
                return_desc2 = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
                if return_desc2.is_null() { status = AE_NO_MEMORY; } else {
                    status = acpi_ut_divide((*(*operand)).integer.value, (*(*operand.add(1))).integer.value,
                        &mut (*return_desc1).integer.value, &mut (*return_desc2).integer.value);
                }
            }
        }
        _ => { ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x{:X}", (*walk_state).opcode)); status = AE_AML_BAD_OPCODE; }
    }
    if ACPI_SUCCESS(status) {
        status = acpi_ex_store(return_desc2, *operand.add(2), walk_state);
        if ACPI_SUCCESS(status) { status = acpi_ex_store(return_desc1, *operand.add(3), walk_state); }
    }
    acpi_ut_remove_reference(return_desc2);
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(return_desc1); }
    else { (*walk_state).result_obj = return_desc1; }
    status
}

pub unsafe fn acpi_ex_opcode_2A_1T_1R(
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut return_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut index: u64 = 0;
    let mut status = AE_OK;
    let mut length: usize = 0;

    if (*(*walk_state).op_info).flags & AML_MATH != 0 {
        return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
        if return_desc.is_null() { status = AE_NO_MEMORY; }
        else { (*return_desc).integer.value = acpi_ex_do_math_op((*walk_state).opcode, (*(*operand)).integer.value, (*(*operand.add(1))).integer.value); }
    } else {
        match (*walk_state).opcode {
            AML_MOD_OP => {
                return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
                if return_desc.is_null() { status = AE_NO_MEMORY; }
                else { status = acpi_ut_divide((*(*operand)).integer.value, (*(*operand.add(1))).integer.value, core::ptr::null_mut(), &mut (*return_desc).integer.value); }
            }
            AML_CONCATENATE_OP => { status = acpi_ex_do_concatenate(*operand, *operand.add(1), &mut return_desc, walk_state); }
            AML_TO_STRING_OP => {
                while length < (*(*operand)).buffer.length && length < (*(*operand.add(1))).integer.value as usize && (*(*operand)).buffer.pointer.add(length).read() != 0 { length += 1; }
                return_desc = acpi_ut_create_string_object(length);
                if return_desc.is_null() { status = AE_NO_MEMORY; }
                else { core::ptr::copy_nonoverlapping((*(*operand)).buffer.pointer, (*return_desc).string.pointer, length); }
            }
            AML_CONCATENATE_TEMPLATE_OP => { status = acpi_ex_concat_template(*operand, *operand.add(1), &mut return_desc, walk_state); }
            AML_INDEX_OP => {
                return_desc = acpi_ut_create_internal_object(ACPI_TYPE_LOCAL_REFERENCE);
                if return_desc.is_null() { status = AE_NO_MEMORY; }
                else {
                    index = (*(*operand.add(1))).integer.value;
                    (*return_desc).reference.value = index as u32;
                    (*return_desc).reference.class_ = ACPI_REFCLASS_INDEX;
                    match (*(*operand)).common.type_ {
                        ACPI_TYPE_STRING => { if index >= (*(*operand)).string.length as u64 { length = (*(*operand)).string.length as usize; status = AE_AML_STRING_LIMIT; } (*return_desc).reference.target_type = ACPI_TYPE_BUFFER_FIELD; (*return_desc).reference.index_pointer = (*operand).buffer.pointer.add(index as usize); }
                        ACPI_TYPE_BUFFER => { if index >= (*(*operand)).buffer.length as u64 { length = (*(*operand)).buffer.length; status = AE_AML_BUFFER_LIMIT; } (*return_desc).reference.target_type = ACPI_TYPE_BUFFER_FIELD; (*return_desc).reference.index_pointer = (*operand).buffer.pointer.add(index as usize); }
                        ACPI_TYPE_PACKAGE => { if index >= (*(*operand)).package.count as u64 { length = (*(*operand)).package.count; status = AE_AML_PACKAGE_LIMIT; } (*return_desc).reference.target_type = ACPI_TYPE_PACKAGE; (*return_desc).reference.where_ = (*operand).package.elements.add(index as usize); }
                        _ => { ACPI_ERROR!((AE_INFO, "Invalid object type: {:X}", (*(*operand)).common.type_)); status = AE_AML_INTERNAL; }
                    }
                    if ACPI_SUCCESS(status) { (*return_desc).reference.object = *operand; acpi_ut_add_reference(*operand); status = acpi_ex_store(return_desc, *operand.add(2), walk_state); (*walk_state).result_obj = return_desc; return status; }
                }
            }
            _ => { ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x{:X}", (*walk_state).opcode)); status = AE_AML_BAD_OPCODE; }
        }
    }
    if ACPI_SUCCESS(status) { status = acpi_ex_store(return_desc, *operand.add(2), walk_state); if ACPI_SUCCESS(status) && (*walk_state).result_obj.is_null() { (*walk_state).result_obj = return_desc; } }
    if ACPI_FAILURE(status) { acpi_ut_remove_reference(return_desc); (*walk_state).result_obj = core::ptr::null_mut(); }
    status
}

pub unsafe fn acpi_ex_opcode_2A_0T_1R(walk_state: *mut acpi_walk_state) -> acpi_status {
    let operand = (*walk_state).operands.as_mut_ptr();
    let mut return_desc = acpi_ut_create_internal_object(ACPI_TYPE_INTEGER);
    let mut status = AE_OK;
    let mut logical_result: u8 = FALSE;
    if return_desc.is_null() { status = AE_NO_MEMORY; }
    else if (*(*walk_state).op_info).flags & AML_LOGICAL_NUMERIC != 0 { status = acpi_ex_do_logical_numeric_op((*walk_state).opcode, (*(*operand)).integer.value, (*(*operand.add(1))).integer.value, &mut logical_result); }
    else if (*(*walk_state).op_info).flags & AML_LOGICAL != 0 { status = acpi_ex_do_logical_op((*walk_state).opcode, *operand, *operand.add(1), &mut logical_result); }
    else { match (*walk_state).opcode { AML_ACQUIRE_OP => { status = acpi_ex_acquire_mutex(*operand.add(1), *operand, walk_state); if status == AE_TIME { logical_result = TRUE; status = AE_OK; } } AML_WAIT_OP => { status = acpi_ex_system_wait_event(*operand.add(1), *operand); if status == AE_TIME { logical_result = TRUE; status = AE_OK; } } _ => { ACPI_ERROR!((AE_INFO, "Unknown AML opcode 0x{:X}", (*walk_state).opcode)); status = AE_AML_BAD_OPCODE; } } }
    if ACPI_SUCCESS(status) { if logical_result != FALSE { (*return_desc).integer.value = ACPI_UINT64_MAX; } (*walk_state).result_obj = return_desc; } else { acpi_ut_remove_reference(return_desc); }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
