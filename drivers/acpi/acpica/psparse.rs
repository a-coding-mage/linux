// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Parser top level AML parse routines.
//
// Dependencies are supplied by the surrounding ACPICA translation.

// #define _COMPONENT ACPI_PARSER
// ACPI_MODULE_NAME("psparse")

pub unsafe fn acpi_ps_get_opcode_size(opcode: u32) -> u32 {
    if opcode > 0x00ff { 2 } else { 1 }
}

pub unsafe fn acpi_ps_peek_opcode(parser_state: *mut acpi_parse_state) -> u16 {
    let mut aml = (*parser_state).aml;
    if aml >= (*parser_state).aml_end { return 0xffff; }
    let mut opcode = *aml as u16;
    if opcode == AML_EXTENDED_PREFIX as u16 {
        aml = aml.add(1);
        if aml >= (*parser_state).aml_end { return 0xffff; }
        opcode = (opcode << 8) | (*aml as u16);
    }
    opcode
}

pub unsafe fn acpi_ps_complete_this_op(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let mut replacement_op: *mut acpi_parse_object = core::ptr::null_mut();
    let mut status = AE_OK;
    if op.is_null() { return AE_OK; }

    acpi_ex_stop_trace_opcode(op, walk_state);
    if (((*walk_state).parse_flags & ACPI_PARSE_TREE_MASK) != ACPI_PARSE_DELETE_TREE)
        || ((*(*walk_state).op_info).class == AML_CLASS_ARGUMENT) { return AE_OK; }

    if !(*op).common.parent.is_null() {
        let parent = (*op).common.parent;
        let mut prev = (*parent).common.value.arg;
        if prev.is_null() { acpi_ps_delete_parse_tree(op); return status; }
        let parent_info = acpi_ps_get_opcode_info((*parent).common.aml_opcode);
        match (*parent_info).class {
            AML_CLASS_CONTROL => {}
            AML_CLASS_CREATE => {
                replacement_op = acpi_ps_alloc_op(AML_INT_RETURN_VALUE_OP, (*op).common.aml);
                if replacement_op.is_null() { status = AE_NO_MEMORY; }
            }
            AML_CLASS_NAMED_OBJECT => {
                let code = (*parent).common.aml_opcode;
                if code == AML_REGION_OP || code == AML_DATA_REGION_OP || code == AML_BUFFER_OP
                    || code == AML_PACKAGE_OP || code == AML_BANK_FIELD_OP
                    || code == AML_VARIABLE_PACKAGE_OP {
                    replacement_op = acpi_ps_alloc_op(AML_INT_RETURN_VALUE_OP, (*op).common.aml);
                    if replacement_op.is_null() { status = AE_NO_MEMORY; }
                } else if code == AML_NAME_OP && (*walk_state).pass_number <= ACPI_IMODE_LOAD_PASS2 {
                    let opc = (*op).common.aml_opcode;
                    if opc == AML_BUFFER_OP || opc == AML_PACKAGE_OP || opc == AML_VARIABLE_PACKAGE_OP {
                        replacement_op = acpi_ps_alloc_op(opc, (*op).common.aml);
                        if replacement_op.is_null() { status = AE_NO_MEMORY; }
                        else {
                            (*replacement_op).named.data = (*op).named.data;
                            (*replacement_op).named.length = (*op).named.length;
                        }
                    }
                }
            }
            _ => {
                replacement_op = acpi_ps_alloc_op(AML_INT_RETURN_VALUE_OP, (*op).common.aml);
                if replacement_op.is_null() { status = AE_NO_MEMORY; }
            }
        }
        if prev == op {
            if !replacement_op.is_null() {
                (*replacement_op).common.parent = parent;
                (*replacement_op).common.value.arg = core::ptr::null_mut();
                (*replacement_op).common.node = (*op).common.node;
                (*parent).common.value.arg = replacement_op;
                (*replacement_op).common.next = (*op).common.next;
            } else { (*parent).common.value.arg = (*op).common.next; }
        } else {
            while !prev.is_null() {
                let next = (*prev).common.next;
                if next == op {
                    if !replacement_op.is_null() {
                        (*replacement_op).common.parent = parent;
                        (*replacement_op).common.value.arg = core::ptr::null_mut();
                        (*replacement_op).common.node = (*op).common.node;
                        (*prev).common.next = replacement_op;
                        (*replacement_op).common.next = (*op).common.next;
                    } else { (*prev).common.next = (*op).common.next; }
                    prev = core::ptr::null_mut();
                } else { prev = next; }
            }
        }
    }
    acpi_ps_delete_parse_tree(op);
    status
}

pub unsafe fn acpi_ps_next_parse_state(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    callback_status: acpi_status,
) -> acpi_status {
    let parser_state = &mut (*walk_state).parser_state;
    let mut status = AE_CTRL_PENDING;
    match callback_status {
        AE_CTRL_TERMINATE => { parser_state.aml = parser_state.aml_end; status = AE_CTRL_TERMINATE; }
        AE_CTRL_BREAK => { parser_state.aml = (*walk_state).aml_last_while; (*(*walk_state).control_state).common.value = FALSE; status = AE_CTRL_BREAK; }
        AE_CTRL_CONTINUE => { parser_state.aml = (*walk_state).aml_last_while; status = AE_CTRL_CONTINUE; }
        AE_CTRL_PENDING => { parser_state.aml = (*walk_state).aml_last_while; }
        AE_CTRL_TRUE => {
            let aml = parser_state.aml;
            parser_state.aml = acpi_ps_get_next_package_end(parser_state);
            if parser_state.aml > parser_state.aml_end || parser_state.aml < aml { status = AE_AML_PACKAGE_LIMIT; }
        }
        AE_CTRL_FALSE => { parser_state.aml = (*parser_state.scope).parse_scope.pkg_end; (*(*walk_state).control_state).common.value = FALSE; status = AE_CTRL_END; }
        AE_CTRL_TRANSFER => {
            status = AE_CTRL_TRANSFER;
            (*walk_state).prev_op = op;
            (*walk_state).method_call_op = op;
            (*walk_state).method_call_node = (*(*op).common.value.arg).common.node;
            (*walk_state).return_used = acpi_ds_is_result_used(op, walk_state);
        }
        _ => { status = callback_status; if ACPI_CNTL_EXCEPTION(callback_status) { status = AE_OK; } }
    }
    status
}

pub unsafe fn acpi_ps_parse_aml(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut status;
    let thread;
    let prev_walk_list = acpi_gbl_current_walk_list;
    let mut previous_walk_state;
    if (*walk_state).parser_state.aml.is_null() { return AE_BAD_ADDRESS; }
    thread = acpi_ut_create_thread_state();
    if thread.is_null() {
        if !(*walk_state).method_desc.is_null() { acpi_ds_terminate_control_method((*walk_state).method_desc, walk_state); }
        acpi_ds_delete_walk_state(walk_state); return AE_NO_MEMORY;
    }
    (*walk_state).thread = thread;
    if !(*walk_state).method_desc.is_null() { (*thread).current_sync_level = (*(*walk_state).method_desc).method.sync_level; }
    acpi_ds_push_walk_state(walk_state, thread);
    acpi_gbl_current_walk_list = thread;
    status = AE_OK;
    while !walk_state.is_null() {
        if ACPI_SUCCESS(status) { status = acpi_ps_parse_loop(walk_state); }
        if (*walk_state).method_pathname != core::ptr::null_mut() && (*walk_state).method_is_nested {
            acpi_free((*walk_state).method_pathname);
            (*walk_state).method_is_nested = FALSE;
        }
        if status == AE_CTRL_TRANSFER {
            status = acpi_ds_call_control_method(thread, walk_state, core::ptr::null_mut());
            if ACPI_FAILURE(status) { status = acpi_ds_method_error(status, walk_state); }
            walk_state = acpi_ds_get_current_walk_state(thread); continue;
        } else if status == AE_CTRL_TERMINATE { status = AE_OK; }
        else if status != AE_OK && !(*walk_state).method_desc.is_null() {
            acpi_ex_exit_interpreter();
            if status == AE_ABORT_METHOD { acpi_ns_print_node_pathname((*walk_state).method_node, "Aborting method"); }
            else { acpi_error_method("Aborting method", (*walk_state).method_node, core::ptr::null_mut(), status); }
            acpi_ex_enter_interpreter();
            if status == AE_ALREADY_EXISTS && ((*(*walk_state).method_desc).method.info_flags & ACPI_METHOD_SERIALIZED) == 0 { (*(*walk_state).method_desc).method.info_flags |= ACPI_METHOD_SERIALIZED_PENDING; }
        }
        walk_state = acpi_ds_pop_walk_state(thread);
        acpi_ds_scope_stack_clear(walk_state);
        if (((*walk_state).parse_flags & ACPI_PARSE_MODE_MASK) == ACPI_PARSE_EXECUTE && ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) == 0) || ACPI_FAILURE(status) { acpi_ds_terminate_control_method((*walk_state).method_desc, walk_state); }
        acpi_ps_cleanup_scope(&mut (*walk_state).parser_state);
        previous_walk_state = walk_state;
        walk_state = acpi_ds_get_current_walk_state(thread);
        if !walk_state.is_null() {
            if ACPI_SUCCESS(status) {
                if (*previous_walk_state).return_desc.is_null() {
                    if acpi_gbl_enable_interpreter_slack && (*previous_walk_state).implicit_return_obj.is_null() { (*previous_walk_state).implicit_return_obj = acpi_ut_create_integer_object(0); if (*previous_walk_state).implicit_return_obj.is_null() { return AE_NO_MEMORY; } }
                    status = acpi_ds_restart_control_method(walk_state, (*previous_walk_state).implicit_return_obj);
                } else { acpi_ds_clear_implicit_return(previous_walk_state); status = acpi_ds_restart_control_method(walk_state, (*previous_walk_state).return_desc); }
                if ACPI_SUCCESS(status) { (*walk_state).walk_type |= ACPI_WALK_METHOD_RESTART; }
            } else { acpi_ut_remove_reference((*previous_walk_state).return_desc); acpi_ds_clear_implicit_return(previous_walk_state); }
        } else if !(*previous_walk_state).caller_return_desc.is_null() {
            *(*previous_walk_state).caller_return_desc = if !(*previous_walk_state).implicit_return_obj.is_null() { (*previous_walk_state).implicit_return_obj } else { (*previous_walk_state).return_desc };
        } else {
            if !(*previous_walk_state).return_desc.is_null() { acpi_ut_remove_reference((*previous_walk_state).return_desc); }
            if !(*previous_walk_state).implicit_return_obj.is_null() { acpi_ut_remove_reference((*previous_walk_state).implicit_return_obj); }
        }
        acpi_ds_delete_walk_state(previous_walk_state);
    }
    acpi_ex_release_all_mutexes(thread);
    acpi_ut_delete_generic_state(thread as *mut acpi_generic_state);
    acpi_gbl_current_walk_list = prev_walk_list;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
