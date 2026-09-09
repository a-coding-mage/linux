// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/* Dispatcher second pass namespace load callbacks. */

pub unsafe fn acpi_ds_load2_begin_op(
    walk_state: *mut acpi_walk_state,
    out_op: *mut *mut acpi_parse_object,
) -> acpi_status {
    let mut op = (*walk_state).op;
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut status: acpi_status;
    let object_type: acpi_object_type;
    let buffer_ptr: *mut i8;
    let mut flags: u32;

    acpi_function_trace!(ds_load2_begin_op);
    acpi_debug_print!(ACPI_DB_DISPATCH, "Op={:?} State={:?}\n", op, walk_state);

    if !op.is_null() {
        if !(*walk_state).control_state.is_null()
            && (*(*walk_state).control_state).common.state == ACPI_CONTROL_CONDITIONAL_EXECUTING
        {
            return acpi_ds_exec_begin_op(walk_state, out_op);
        }
        if ((!((*walk_state).op_info.flags & AML_NSOPCODE) != 0
            && (*walk_state).opcode != AML_INT_NAMEPATH_OP)
            || ((*walk_state).op_info.flags & AML_NAMED) == 0)
        {
            return AE_OK;
        }
        if (*walk_state).opcode == AML_INT_NAMEPATH_OP {
            buffer_ptr = (*op).common.value.string;
            if buffer_ptr.is_null() { return AE_OK; }
        } else {
            buffer_ptr = &mut (*op).named.name as *mut _ as *mut i8;
        }
    } else {
        buffer_ptr = acpi_ps_get_next_namestring(&mut (*walk_state).parser_state);
    }

    object_type = (*walk_state).op_info.object_type;
    acpi_debug_print!(ACPI_DB_DISPATCH, "State={:?} Op={:?} Type={:?}\n", walk_state, op, object_type);

    match (*walk_state).opcode {
        AML_FIELD_OP | AML_BANK_FIELD_OP | AML_INDEX_FIELD_OP => { status = AE_OK; }
        AML_INT_NAMEPATH_OP => {
            status = acpi_ns_lookup((*walk_state).scope_info, buffer_ptr, object_type,
                ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut node);
        }
        AML_SCOPE_OP => {
            if !op.is_null() && (*op).named.node == acpi_gbl_root_node {
                node = (*op).named.node;
                status = acpi_ds_scope_stack_push(node, object_type, walk_state);
                if ACPI_FAILURE(status) { return status; }
            } else {
                status = acpi_ns_lookup((*walk_state).scope_info, buffer_ptr, object_type,
                    ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut node);
                if ACPI_FAILURE(status) {
                    #[cfg(feature = "acpi_asl_compiler")]
                    if status == AE_NOT_FOUND { status = AE_OK; }
                    else { acpi_error_namespace((*walk_state).scope_info, buffer_ptr, status); }
                    #[cfg(not(feature = "acpi_asl_compiler"))]
                    acpi_error_namespace((*walk_state).scope_info, buffer_ptr, status);
                    return status;
                }
            }
            match (*node).type_ {
                ACPI_TYPE_ANY | ACPI_TYPE_LOCAL_SCOPE | ACPI_TYPE_DEVICE | ACPI_TYPE_POWER |
                ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => {}
                ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => {
                    acpi_warning!(AE_INFO, "Type override for Scope operator");
                    (*node).type_ = ACPI_TYPE_ANY;
                    (*(*walk_state).scope_info).common.value = ACPI_TYPE_ANY;
                }
                ACPI_TYPE_METHOD => {
                    if node == acpi_gbl_root_node && ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) != 0 {}
                    else { acpi_error!(AE_INFO, "Invalid type for Scope operator"); return AE_AML_OPERAND_TYPE; }
                }
                _ => { acpi_error!(AE_INFO, "Invalid type for Scope operator"); return AE_AML_OPERAND_TYPE; }
            }
        }
        _ => {
            if !op.is_null() && !(*op).common.node.is_null() {
                node = (*op).common.node;
                if acpi_ns_opens_scope(object_type) {
                    status = acpi_ds_scope_stack_push(node, object_type, walk_state);
                    if ACPI_FAILURE(status) { return status; }
                }
                return AE_OK;
            }
            if !(*walk_state).deferred_node.is_null() { node = (*walk_state).deferred_node; status = AE_OK; }
            else {
                flags = ACPI_NS_NO_UPSEARCH;
                if (*walk_state).pass_number == ACPI_IMODE_EXECUTE {
                    flags |= ACPI_NS_ERROR_IF_FOUND;
                    if ((*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL) == 0 { flags |= ACPI_NS_TEMPORARY; }
                }
                #[cfg(feature = "acpi_asl_compiler")]
                if (*walk_state).opcode == AML_EXTERNAL_OP { flags |= ACPI_NS_DONT_OPEN_SCOPE; }
                if ((*walk_state).op_info.flags & AML_NAMED) != 0 { flags |= ACPI_NS_PREFIX_MUST_EXIST; }
                status = acpi_ns_lookup((*walk_state).scope_info, buffer_ptr, object_type,
                    ACPI_IMODE_LOAD_PASS2, flags, walk_state, &mut node);
            }
        }
    }
    if ACPI_FAILURE(status) { acpi_error_namespace((*walk_state).scope_info, buffer_ptr, status); return status; }
    if op.is_null() {
        op = acpi_ps_alloc_op((*walk_state).opcode, (*walk_state).aml);
        if op.is_null() { return AE_NO_MEMORY; }
        if !node.is_null() { (*op).named.name = (*node).name.integer; }
        *out_op = op;
    }
    (*op).common.node = node;
    status
}

pub unsafe fn acpi_ds_load2_end_op(walk_state: *mut acpi_walk_state) -> acpi_status {
    let op = (*walk_state).op;
    let mut status = AE_OK;
    if ((*walk_state).op_info.flags & AML_NSOBJECT) == 0 { return AE_OK; }
    let object_type = (*walk_state).op_info.object_type;
    let node = (*op).common.node;
    (*walk_state).operands[0] = node as *mut core::ffi::c_void;
    (*walk_state).num_operands = 1;
    if acpi_ns_opens_scope(object_type) && (*op).common.aml_opcode != AML_INT_METHODCALL_OP {
        status = acpi_ds_scope_stack_pop(walk_state);
        if ACPI_FAILURE(status) { return acpi_ds_load2_cleanup(walk_state, status); }
    }
    let arg = (*op).common.value.arg;
    match (*walk_state).op_info.type_ {
        AML_TYPE_CREATE_FIELD => { status = acpi_ds_create_buffer_field(op, walk_state); }
        AML_TYPE_NAMED_FIELD => {
            if !(*walk_state).method_node.is_null() { status = acpi_ds_init_field_objects(op, walk_state); }
            match (*op).common.aml_opcode {
                AML_INDEX_FIELD_OP => status = acpi_ds_create_index_field(op, (*arg).common.node, walk_state),
                AML_BANK_FIELD_OP => status = acpi_ds_create_bank_field(op, (*arg).common.node, walk_state),
                AML_FIELD_OP => status = acpi_ds_create_field(op, (*arg).common.node, walk_state),
                _ => {}
            }
        }
        AML_TYPE_NAMED_SIMPLE => {
            status = acpi_ds_create_operands(walk_state, arg);
            if ACPI_FAILURE(status) { return acpi_ds_load2_cleanup(walk_state, status); }
            status = match (*op).common.aml_opcode {
                AML_PROCESSOR_OP => acpi_ex_create_processor(walk_state),
                AML_POWER_RESOURCE_OP => acpi_ex_create_power_resource(walk_state),
                AML_MUTEX_OP => acpi_ex_create_mutex(walk_state), AML_EVENT_OP => acpi_ex_create_event(walk_state),
                AML_ALIAS_OP => acpi_ex_create_alias(walk_state), _ => AE_OK,
            };
            let mut i = 1; while i < (*walk_state).num_operands { acpi_ut_remove_reference((*walk_state).operands[i]); (*walk_state).operands[i] = core::ptr::null_mut(); i += 1; }
        }
        AML_TYPE_NAMED_COMPLEX => match (*op).common.aml_opcode {
            AML_REGION_OP | AML_DATA_REGION_OP => { let space = if (*op).common.aml_opcode == AML_REGION_OP { (*(*op).common.value.arg).common.value.integer as u8 } else { ACPI_ADR_SPACE_DATA_TABLE }; if !(*walk_state).method_node.is_null() { status = acpi_ex_create_region((*op).named.data, (*op).named.length, space, walk_state); if ACPI_FAILURE(status) { return status; } } status = acpi_ev_initialize_region(acpi_ns_get_attached_object(node)); }
            AML_NAME_OP => status = acpi_ds_create_node(walk_state, node, op),
            AML_METHOD_OP => { if acpi_ns_get_attached_object((*op).named.node).is_null() { (*walk_state).operands[0] = (*op).named.node as *mut _; (*walk_state).num_operands = 1; status = acpi_ds_create_operands(walk_state, (*op).common.value.arg); if ACPI_SUCCESS(status) { status = acpi_ex_create_method((*op).named.data, (*op).named.length, walk_state); } (*walk_state).operands[0] = core::ptr::null_mut(); (*walk_state).num_operands = 0; } }
            _ => {}
        },
        AML_CLASS_METHOD_CALL => { let mut new_node = core::ptr::null_mut(); status = acpi_ns_lookup((*walk_state).scope_info, (*arg).common.value.string, ACPI_TYPE_ANY, ACPI_IMODE_LOAD_PASS2, ACPI_NS_SEARCH_PARENT | ACPI_NS_DONT_OPEN_SCOPE, walk_state, &mut new_node); if ACPI_SUCCESS(status) && (*new_node).type_ != ACPI_TYPE_METHOD { status = AE_AML_OPERAND_TYPE; } (*op).common.node = new_node; }
        _ => {}
    }
    acpi_ds_load2_cleanup(walk_state, status)
}

unsafe fn acpi_ds_load2_cleanup(walk_state: *mut acpi_walk_state, status: acpi_status) -> acpi_status { (*walk_state).operands[0] = core::ptr::null_mut(); (*walk_state).num_operands = 0; status }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
