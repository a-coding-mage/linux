// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Dispatcher method execution callbacks; dispatch to interpreter.

// C headers and build-time configuration are supplied by the surrounding
// ACPI translation unit.

static mut ACPI_GBL_OP_TYPE_DISPATCH: [Option<acpi_execute_op>; 12] = [
    Some(acpi_ex_opcode_0A_0T_1R), Some(acpi_ex_opcode_1A_0T_0R),
    Some(acpi_ex_opcode_1A_0T_1R), None, Some(acpi_ex_opcode_1A_1T_1R),
    Some(acpi_ex_opcode_2A_0T_0R), Some(acpi_ex_opcode_2A_0T_1R),
    Some(acpi_ex_opcode_2A_1T_1R), Some(acpi_ex_opcode_2A_2T_1R),
    Some(acpi_ex_opcode_3A_0T_0R), Some(acpi_ex_opcode_3A_1T_1R),
    Some(acpi_ex_opcode_6A_0T_1R),
];

pub unsafe fn acpi_ds_get_predicate_value(
    walk_state: *mut acpi_walk_state,
    result_obj: *mut acpi_operand_object,
) -> acpi_status {
    let mut status = AE_OK;
    let obj_desc: *mut acpi_operand_object;
    let mut local_obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    (*(*walk_state).control_state).common.state = 0;

    if !result_obj.is_null() {
        status = acpi_ds_result_pop(&mut (obj_desc as *mut _), walk_state);
        if ACPI_FAILURE(status) { return status; }
    } else {
        status = acpi_ds_create_operand(walk_state, (*walk_state).op, 0);
        if ACPI_FAILURE(status) { return status; }
        status = acpi_ex_resolve_to_value(&mut (*walk_state).operands[0], walk_state);
        if ACPI_FAILURE(status) { return status; }
        obj_desc = (*walk_state).operands[0];
    }
    if obj_desc.is_null() { return AE_AML_NO_OPERAND; }
    status = acpi_ex_convert_to_integer(obj_desc, &mut local_obj_desc,
                                        ACPI_IMPLICIT_CONVERSION);
    if ACPI_FAILURE(status) { return acpi_ds_predicate_cleanup(status, walk_state, obj_desc, local_obj_desc); }
    if (*local_obj_desc).common.type_ != ACPI_TYPE_INTEGER {
        status = AE_AML_OPERAND_TYPE;
        return acpi_ds_predicate_cleanup(status, walk_state, obj_desc, local_obj_desc);
    }
    acpi_ex_truncate_for32bit_table(local_obj_desc);
    if (*local_obj_desc).integer.value != 0 {
        (*(*walk_state).control_state).common.value = TRUE;
    } else {
        (*(*walk_state).control_state).common.value = FALSE;
        status = AE_CTRL_FALSE;
    }
    acpi_ds_do_implicit_return(local_obj_desc, walk_state, TRUE);
    acpi_ds_predicate_cleanup(status, walk_state, obj_desc, local_obj_desc)
}

unsafe fn acpi_ds_predicate_cleanup(
    status: acpi_status, walk_state: *mut acpi_walk_state,
    obj_desc: *mut acpi_operand_object, local_obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    acpi_db_display_result_object(local_obj_desc, walk_state);
    if local_obj_desc != obj_desc && !local_obj_desc.is_null() { acpi_ut_remove_reference(local_obj_desc); }
    if !obj_desc.is_null() { acpi_ut_remove_reference(obj_desc); }
    (*(*walk_state).control_state).common.state = ACPI_CONTROL_NORMAL;
    status
}

pub unsafe fn acpi_ds_exec_begin_op(
    walk_state: *mut acpi_walk_state,
    out_op: *mut *mut acpi_parse_object,
) -> acpi_status {
    let mut status = AE_OK;
    let mut op = (*walk_state).op;
    if op.is_null() {
        status = acpi_ds_load2_begin_op(walk_state, out_op);
        if ACPI_FAILURE(status) { return acpi_ds_method_error(status, walk_state); }
        op = *out_op; (*walk_state).op = op;
        (*walk_state).opcode = (*op).common.aml_opcode;
        (*walk_state).op_info = acpi_ps_get_opcode_info((*op).common.aml_opcode);
        if acpi_ns_opens_scope((*(*walk_state).op_info).object_type) {
            status = acpi_ds_scope_stack_pop(walk_state);
            if ACPI_FAILURE(status) { return acpi_ds_method_error(status, walk_state); }
        }
    }
    if op == (*walk_state).origin { if !out_op.is_null() { *out_op = op; } return AE_OK; }
    if !(*walk_state).control_state.is_null() && (*(*walk_state).control_state).common.state == ACPI_CONTROL_CONDITIONAL_EXECUTING {
        (*(*walk_state).control_state).common.state = ACPI_CONTROL_PREDICATE_EXECUTING;
        (*(*walk_state).control_state).control.predicate_op = op;
    }
    let mut opcode_class = (*(*walk_state).op_info).class;
    if (*op).common.aml_opcode == AML_INT_NAMEPATH_OP { opcode_class = AML_CLASS_NAMED_OBJECT; }
    match opcode_class {
        AML_CLASS_CONTROL => status = acpi_ds_exec_begin_control_op(walk_state, op),
        AML_CLASS_NAMED_OBJECT => if (*walk_state).walk_type & ACPI_WALK_METHOD != 0 {
            if (*op).common.aml_opcode != AML_SCOPE_OP { status = acpi_ds_load2_begin_op(walk_state, core::ptr::null_mut()); }
            else { status = acpi_ds_scope_stack_push((*op).named.node, (*op).named.node.type_, walk_state); }
        },
        _ => {}
    }
    if ACPI_FAILURE(status) { acpi_ds_method_error(status, walk_state) } else { status }
}

pub unsafe fn acpi_ds_exec_end_op(walk_state: *mut acpi_walk_state) -> acpi_status {
    let op = (*walk_state).op;
    let op_type = (*(*walk_state).op_info).type_;
    let op_class = (*(*walk_state).op_info).class;
    if op_class == AML_CLASS_UNKNOWN { return AE_NOT_IMPLEMENTED; }
    let first_arg = (*op).common.value.arg;
    (*walk_state).num_operands = 0; (*walk_state).operand_index = 0;
    (*walk_state).return_desc = core::ptr::null_mut(); (*walk_state).result_obj = core::ptr::null_mut();
    let mut status = acpi_db_single_step(walk_state, op, op_class);
    if ACPI_FAILURE(status) { return status; }
    match op_class {
        AML_CLASS_ARGUMENT => if (*walk_state).opcode == AML_INT_NAMEPATH_OP { status = acpi_ds_evaluate_name_path(walk_state); },
        AML_CLASS_EXECUTE => {
            status = acpi_ds_create_operands(walk_state, first_arg);
            if ACPI_SUCCESS(status) && (*(*walk_state).op_info).flags & AML_NO_OPERAND_RESOLVE == 0 && (*(*walk_state).op_info).flags & AML_HAS_ARGS != 0 {
                status = acpi_ex_resolve_operands((*walk_state).opcode, &mut (*walk_state).operands[(*walk_state).num_operands - 1], walk_state);
            }
            if ACPI_SUCCESS(status) { if let Some(f) = ACPI_GBL_OP_TYPE_DISPATCH[op_type as usize] { status = f(walk_state); } }
            acpi_ds_clear_operands(walk_state);
            if ACPI_SUCCESS(status) && !(*walk_state).result_obj.is_null() { status = acpi_ds_result_push((*walk_state).result_obj, walk_state); }
        },
        _ => match op_type {
            AML_TYPE_CONTROL => status = acpi_ds_exec_end_control_op(walk_state, op),
            AML_TYPE_METHOD_CALL => { let mut next_op = (*first_arg).common.next; status = acpi_ds_create_operands(walk_state, next_op); if ACPI_SUCCESS(status) { status = acpi_ds_resolve_operands(walk_state); } if ACPI_SUCCESS(status) { status = AE_CTRL_TRANSFER; return status; } },
            AML_TYPE_CREATE_FIELD => { status = acpi_ds_load2_end_op(walk_state); if ACPI_SUCCESS(status) { status = acpi_ds_eval_buffer_field_operands(walk_state, op); } },
            AML_TYPE_CREATE_OBJECT => { status = acpi_ds_eval_data_object_operands(walk_state, op, core::ptr::null_mut()); if !(*walk_state).result_obj.is_null() { status = acpi_ds_result_push((*walk_state).result_obj, walk_state); } },
            AML_TYPE_NAMED_FIELD | AML_TYPE_NAMED_COMPLEX | AML_TYPE_NAMED_SIMPLE | AML_TYPE_NAMED_NO_OBJ => { status = acpi_ds_load2_end_op(walk_state); },
            AML_TYPE_UNDEFINED => return AE_NOT_IMPLEMENTED,
            _ => status = AE_NOT_IMPLEMENTED,
        }
    }
    acpi_ex_truncate_for32bit_table((*walk_state).result_obj);
    if ACPI_SUCCESS(status) && !(*walk_state).control_state.is_null() && (*(*walk_state).control_state).common.state == ACPI_CONTROL_PREDICATE_EXECUTING && (*(*walk_state).control_state).control.predicate_op == op { status = acpi_ds_get_predicate_value(walk_state, (*walk_state).result_obj); (*walk_state).result_obj = core::ptr::null_mut(); }
    if !(*walk_state).result_obj.is_null() { acpi_db_display_result_object((*walk_state).result_obj, walk_state); acpi_ds_delete_result_if_not_used(op, (*walk_state).result_obj, walk_state); }
    if ACPI_FAILURE(status) { status = acpi_ds_method_error(status, walk_state); }
    (*walk_state).num_operands = 0;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
