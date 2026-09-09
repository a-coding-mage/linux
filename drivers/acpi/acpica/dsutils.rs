// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Dispatcher utilities. C headers and build-time tracing macros are supplied by
// the surrounding ACPICA translation.

pub unsafe fn acpi_ds_clear_implicit_return(walk_state: *mut acpi_walk_state) {
    if !acpi_gbl_enable_interpreter_slack { return; }
    if !(*walk_state).implicit_return_obj.is_null() {
        acpi_ut_remove_reference((*walk_state).implicit_return_obj);
        (*walk_state).implicit_return_obj = core::ptr::null_mut();
    }
}

pub unsafe fn acpi_ds_do_implicit_return(
    return_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
    add_reference: u8,
) -> u8 {
    if !acpi_gbl_enable_interpreter_slack || return_desc.is_null() { return FALSE; }
    if !(*walk_state).implicit_return_obj.is_null() {
        if (*walk_state).implicit_return_obj == return_desc { return TRUE; }
        acpi_ds_clear_implicit_return(walk_state);
    }
    (*walk_state).implicit_return_obj = return_desc;
    if add_reference != 0 { acpi_ut_add_reference(return_desc); }
    TRUE
}

pub unsafe fn acpi_ds_is_result_used(
    op: *mut acpi_parse_object,
    walk_state: *mut acpi_walk_state,
) -> u8 {
    if op.is_null() { return TRUE; }
    acpi_ds_do_implicit_return((*walk_state).result_obj, walk_state, TRUE);
    if (*op).common.parent.is_null() ||
       (*(*op).common.parent).common.aml_opcode == AML_SCOPE_OP { return FALSE; }
    let parent_info = acpi_ps_get_opcode_info((*(*op).common.parent).common.aml_opcode);
    if (*parent_info).class == AML_CLASS_UNKNOWN { return FALSE; }
    match (*parent_info).class {
        AML_CLASS_CONTROL => {
            match (*(*op).common.parent).common.aml_opcode {
                AML_RETURN_OP => TRUE,
                AML_IF_OP | AML_WHILE_OP => {
                    if !(*walk_state).control_state.is_null() &&
                       (*(*walk_state).control_state).common.state == ACPI_CONTROL_PREDICATE_EXECUTING &&
                       (*(*walk_state).control_state).control.predicate_op == op { TRUE } else { FALSE }
                }
                _ => FALSE,
            }
        }
        AML_CLASS_CREATE => TRUE,
        AML_CLASS_NAMED_OBJECT => {
            match (*(*op).common.parent).common.aml_opcode {
                AML_REGION_OP | AML_DATA_REGION_OP | AML_PACKAGE_OP | AML_BUFFER_OP |
                AML_VARIABLE_PACKAGE_OP | AML_INT_EVAL_SUBTREE_OP | AML_BANK_FIELD_OP => TRUE,
                _ => FALSE,
            }
        }
        _ => TRUE,
    }
}

pub unsafe fn acpi_ds_delete_result_if_not_used(
    op: *mut acpi_parse_object,
    result_obj: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) {
    if op.is_null() || result_obj.is_null() { return; }
    if acpi_ds_is_result_used(op, walk_state) == 0 {
        let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
        let status = acpi_ds_result_pop(&mut obj_desc, walk_state);
        if ACPI_SUCCESS(status) { acpi_ut_remove_reference(result_obj); }
    }
}

pub unsafe fn acpi_ds_resolve_operands(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut status = AE_OK;
    for i in 0..(*walk_state).num_operands as usize {
        status = acpi_ex_resolve_to_value(&mut (*walk_state).operands[i], walk_state);
        if ACPI_FAILURE(status) { break; }
    }
    status
}

pub unsafe fn acpi_ds_clear_operands(walk_state: *mut acpi_walk_state) {
    for i in 0..(*walk_state).num_operands as usize {
        acpi_ut_remove_reference((*walk_state).operands[i]);
        (*walk_state).operands[i] = core::ptr::null_mut();
    }
    (*walk_state).num_operands = 0;
}

pub unsafe fn acpi_ds_create_operand(
    walk_state: *mut acpi_walk_state,
    arg: *mut acpi_parse_object,
    arg_index: u32,
) -> acpi_status {
    let mut status = AE_OK;
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut opcode: u16;
    if (*arg).common.aml_opcode == AML_INT_NAMEPATH_OP &&
       !(*arg).common.value.string.is_null() &&
       (*arg).common.flags & ACPI_PARSEOP_IN_STACK == 0 {
        let mut name_string: *mut i8 = core::ptr::null_mut();
        let mut name_length = 0u32;
        status = acpi_ex_get_name_string(ACPI_TYPE_ANY, (*arg).common.value.buffer,
                                         &mut name_string, &mut name_length);
        if ACPI_FAILURE(status) { return status; }
        if !(*walk_state).deferred_node.is_null() &&
           (*(*walk_state).deferred_node).type_ == ACPI_TYPE_BUFFER_FIELD &&
           arg_index == if (*walk_state).opcode == AML_CREATE_FIELD_OP { 3 } else { 2 } {
            obj_desc = (*walk_state).deferred_node as *mut acpi_operand_object;
        } else {
            let parent = (*arg).common.parent;
            let info = acpi_ps_get_opcode_info((*parent).common.aml_opcode);
            let mode = if (*info).flags & AML_NSNODE != 0 &&
                (*parent).common.aml_opcode != AML_INT_METHODCALL_OP &&
                (*parent).common.aml_opcode != AML_REGION_OP &&
                (*parent).common.aml_opcode != AML_INT_NAMEPATH_OP {
                ACPI_IMODE_LOAD_PASS2
            } else { ACPI_IMODE_EXECUTE };
            status = acpi_ns_lookup((*walk_state).scope_info, name_string, ACPI_TYPE_ANY,
                mode, ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE, walk_state,
                &mut obj_desc as *mut _ as *mut *mut acpi_namespace_node);
            if status == AE_NOT_FOUND {
                status = if (*parent).common.aml_opcode == AML_CONDITIONAL_REF_OF_OP {
                    obj_desc = acpi_gbl_root_node as *mut acpi_operand_object; AE_OK
                } else if (*parent).common.aml_opcode == AML_EXTERNAL_OP {
                    AE_AML_BAD_OPCODE
                } else { AE_AML_NAME_NOT_FOUND };
            }
        }
        ACPI_FREE(name_string);
        if ACPI_FAILURE(status) { return status; }
        status = acpi_ds_obj_stack_push(obj_desc, walk_state);
        if ACPI_FAILURE(status) { return status; }
    } else {
        if (*arg).common.aml_opcode == AML_INT_NAMEPATH_OP &&
           (*arg).common.flags & ACPI_PARSEOP_IN_STACK == 0 { opcode = AML_ZERO_OP; }
        else { opcode = (*arg).common.aml_opcode; }
        let info = acpi_ps_get_opcode_info(opcode);
        if (*info).object_type == ACPI_TYPE_INVALID { return AE_NOT_IMPLEMENTED; }
        if (*info).flags & AML_HAS_RETVAL != 0 || (*arg).common.flags & ACPI_PARSEOP_IN_STACK != 0 {
            status = acpi_ds_result_pop(&mut obj_desc, walk_state);
            if ACPI_FAILURE(status) { return status; }
        } else {
            obj_desc = acpi_ut_create_internal_object((*info).object_type);
            if obj_desc.is_null() { return AE_NO_MEMORY; }
            status = acpi_ds_init_object_from_op(walk_state, arg, opcode, &mut obj_desc);
            if ACPI_FAILURE(status) { acpi_ut_delete_object_desc(obj_desc); return status; }
        }
        status = acpi_ds_obj_stack_push(obj_desc, walk_state);
        if ACPI_FAILURE(status) { return status; }
    }
    acpi_db_display_argument_object(obj_desc, walk_state);
    AE_OK
}

pub unsafe fn acpi_ds_create_operands(walk_state: *mut acpi_walk_state, first_arg: *mut acpi_parse_object) -> acpi_status {
    let mut args: [*mut acpi_parse_object; ACPI_OBJ_NUM_OPERANDS as usize] = [core::ptr::null_mut(); ACPI_OBJ_NUM_OPERANDS as usize];
    let mut arg = first_arg; let mut count = 0usize; let mut index = (*walk_state).num_operands as usize;
    let previous = (*walk_state).num_operands as u8;
    while !arg.is_null() {
        if index >= ACPI_OBJ_NUM_OPERANDS as usize { return AE_BAD_DATA; }
        args[index] = arg; (*walk_state).operands[index] = core::ptr::null_mut();
        arg = (*arg).common.next; count += 1; index += 1;
    }
    let new_count = index; index -= 1;
    for i in 0..count {
        arg = args[index]; (*walk_state).operand_index = index as u8;
        let status = acpi_ds_create_operand(walk_state, arg, index as u32);
        if ACPI_FAILURE(status) {
            (*walk_state).num_operands = i as u8;
            acpi_ds_obj_stack_pop_and_delete(new_count as u32, walk_state);
            (*walk_state).num_operands = previous;
            return status;
        }
        index -= 1;
    }
    AE_OK
}

pub unsafe fn acpi_ds_evaluate_name_path(walk_state: *mut acpi_walk_state) -> acpi_status {
    let op = (*walk_state).op;
    if (*op).common.parent.is_null() { return AE_OK; }
    match (*(*op).common.parent).common.aml_opcode {
        AML_PACKAGE_OP | AML_VARIABLE_PACKAGE_OP | AML_REF_OF_OP => return AE_OK,
        _ => {}
    }
    let mut operand = &mut (*walk_state).operands[0] as *mut *mut acpi_operand_object;
    let mut status = acpi_ds_create_operand(walk_state, op, 0);
    if ACPI_FAILURE(status) { return status; }
    let mut new_obj_desc: *mut acpi_operand_object;
    if (*op).common.flags & ACPI_PARSEOP_TARGET != 0 { new_obj_desc = *operand; }
    else {
        let typ = (**operand).common.type_;
        status = acpi_ex_resolve_to_value(operand, walk_state);
        if ACPI_FAILURE(status) { return status; }
        if typ == ACPI_TYPE_INTEGER {
            acpi_ut_remove_reference(*operand);
            status = acpi_ut_copy_iobject_to_iobject(*operand, &mut new_obj_desc, walk_state);
            if ACPI_FAILURE(status) { return status; }
        } else { new_obj_desc = *operand; }
    }
    status = acpi_ds_obj_stack_pop(1, walk_state);
    if ACPI_FAILURE(status) { (*walk_state).result_obj = new_obj_desc; return status; }
    (*walk_state).result_obj = new_obj_desc;
    status = acpi_ds_result_push((*walk_state).result_obj, walk_state);
    if ACPI_SUCCESS(status) { (*op).common.flags |= ACPI_PARSEOP_IN_STACK; }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
