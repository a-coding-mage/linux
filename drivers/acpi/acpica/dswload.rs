// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Dispatcher first pass namespace load callbacks.
// C dependencies are supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ds_init_callbacks(walk_state: *mut acpi_walk_state, pass_number: u32) -> acpi_status {
    match pass_number {
        0 => {
            (*walk_state).parse_flags = ACPI_PARSE_LOAD_PASS1 | ACPI_PARSE_DELETE_TREE | ACPI_PARSE_DISASSEMBLE;
            (*walk_state).descending_callback = None;
            (*walk_state).ascending_callback = None;
        }
        1 => {
            (*walk_state).parse_flags = ACPI_PARSE_LOAD_PASS1 | ACPI_PARSE_DELETE_TREE;
            (*walk_state).descending_callback = Some(acpi_ds_load1_begin_op);
            (*walk_state).ascending_callback = Some(acpi_ds_load1_end_op);
        }
        2 => {
            (*walk_state).parse_flags = ACPI_PARSE_LOAD_PASS1 | ACPI_PARSE_DELETE_TREE;
            (*walk_state).descending_callback = Some(acpi_ds_load2_begin_op);
            (*walk_state).ascending_callback = Some(acpi_ds_load2_end_op);
        }
        3 => {
            (*walk_state).parse_flags |= ACPI_PARSE_EXECUTE | ACPI_PARSE_DELETE_TREE;
            (*walk_state).descending_callback = Some(acpi_ds_exec_begin_op);
            (*walk_state).ascending_callback = Some(acpi_ds_exec_end_op);
        }
        _ => return AE_BAD_PARAMETER,
    }
    AE_OK
}

pub unsafe fn acpi_ds_load1_begin_op(
    walk_state: *mut acpi_walk_state,
    out_op: *mut *mut acpi_parse_object,
) -> acpi_status {
    let mut op = (*walk_state).op;
    let mut node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut status: acpi_status;
    let object_type: acpi_object_type;
    let path: *mut i8;
    let mut flags: u32;

    if !op.is_null() {
        if (*(*walk_state).op_info).flags & AML_NAMED == 0 {
            *out_op = op;
            return AE_OK;
        }
        if !(*op).common.node.is_null() {
            *out_op = op;
            return AE_OK;
        }
    }

    path = acpi_ps_get_next_namestring(&mut (*walk_state).parser_state);
    object_type = (*(*walk_state).op_info).object_type;

    match (*walk_state).opcode {
        AML_SCOPE_OP => {
            status = acpi_ns_lookup((*walk_state).scope_info, path, object_type,
                ACPI_IMODE_EXECUTE, ACPI_NS_SEARCH_PARENT, walk_state, &mut node);
            #[cfg(feature = "acpi_asl_compiler")]
            if status == AE_NOT_FOUND {
                acpi_dm_add_op_to_external_list(op, path, ACPI_TYPE_DEVICE, 0, 0);
                status = acpi_ns_lookup((*walk_state).scope_info, path, object_type,
                    ACPI_IMODE_LOAD_PASS1, ACPI_NS_SEARCH_PARENT, walk_state, &mut node);
            }
            if ACPI_FAILURE(status) { return status; }
            match (*node).type_ {
                ACPI_TYPE_ANY | ACPI_TYPE_LOCAL_SCOPE | ACPI_TYPE_DEVICE |
                ACPI_TYPE_POWER | ACPI_TYPE_PROCESSOR | ACPI_TYPE_THERMAL => {}
                ACPI_TYPE_INTEGER | ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => {
                    (*node).type_ = ACPI_TYPE_ANY;
                    (*(*walk_state).scope_info).common.value = ACPI_TYPE_ANY;
                }
                ACPI_TYPE_METHOD => {
                    if !(node == acpi_gbl_root_node && (*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL != 0) {
                        return AE_AML_OPERAND_TYPE;
                    }
                }
                _ => return AE_AML_OPERAND_TYPE,
            }
        }
        _ => {
            if !(*walk_state).deferred_node.is_null() {
                node = (*walk_state).deferred_node;
                status = AE_OK;
            } else if !(*walk_state).method_node.is_null() {
                status = AE_OK;
            } else {
                flags = ACPI_NS_NO_UPSEARCH;
                if (*walk_state).opcode != AML_SCOPE_OP && (*walk_state).parse_flags & ACPI_PARSE_DEFERRED_OP == 0 {
                    flags |= if (*walk_state).namespace_override { ACPI_NS_OVERRIDE_IF_FOUND } else { ACPI_NS_ERROR_IF_FOUND };
                }
                status = acpi_ns_lookup((*walk_state).scope_info, path, object_type,
                    ACPI_IMODE_LOAD_PASS1, flags, walk_state, &mut node);
                if ACPI_FAILURE(status) && status == AE_ALREADY_EXISTS {
                    if (*node).flags & ANOBJ_IS_EXTERNAL != 0 {
                        (*node).flags &= !ANOBJ_IS_EXTERNAL;
                        (*node).type_ = object_type as u8;
                        if acpi_ns_opens_scope(object_type) {
                            status = acpi_ds_scope_stack_push(node, object_type, walk_state);
                            if ACPI_FAILURE(status) { return status; }
                        }
                        status = AE_OK;
                    }
                }
                if ACPI_FAILURE(status) { return status; }
            }
        }
    }

    if op.is_null() {
        op = acpi_ps_alloc_op((*walk_state).opcode, (*walk_state).aml);
        if op.is_null() { return AE_NO_MEMORY; }
    }
    #[cfg(feature = "acpi_constant_eval_only")]
    { (*op).named.path = path; }
    if !node.is_null() {
        (*op).common.node = node;
        (*op).named.name = (*node).name.integer;
    }
    acpi_ps_append_arg(acpi_ps_get_parent_scope(&mut (*walk_state).parser_state), op);
    *out_op = op;
    status
}

pub unsafe fn acpi_ds_load1_end_op(walk_state: *mut acpi_walk_state) -> acpi_status {
    let op = (*walk_state).op;
    let mut object_type: acpi_object_type;
    let mut status = AE_OK;

    #[cfg(feature = "acpi_asl_compiler")]
    if (*walk_state).parse_flags & ACPI_PARSE_DISASSEMBLE != 0 && (*(*walk_state).op_info).flags & AML_CREATE != 0 {
        return acpi_ds_create_buffer_field(op, walk_state);
    }
    if (*(*walk_state).op_info).flags & (AML_NAMED | AML_FIELD) == 0 { return AE_OK; }
    object_type = (*(*walk_state).op_info).object_type;
    if (*(*walk_state).op_info).flags & AML_FIELD != 0 {
        if (*walk_state).method_node.is_null() && ((*walk_state).opcode == AML_FIELD_OP ||
            (*walk_state).opcode == AML_BANK_FIELD_OP || (*walk_state).opcode == AML_INDEX_FIELD_OP) {
            status = acpi_ds_init_field_objects(op, walk_state);
        }
        return status;
    }
    if (*walk_state).method_node.is_null() {
        if (*op).common.aml_opcode == AML_REGION_OP {
            status = acpi_ex_create_region((*op).named.data, (*op).named.length,
                (*(*op).common.value.arg).common.value.integer as acpi_adr_space_type, walk_state);
        } else if (*op).common.aml_opcode == AML_DATA_REGION_OP {
            status = acpi_ex_create_region((*op).named.data, (*op).named.length,
                ACPI_ADR_SPACE_DATA_TABLE, walk_state);
        }
        if ACPI_FAILURE(status) { return status; }
    }
    if (*op).common.aml_opcode == AML_NAME_OP && !(*op).common.value.arg.is_null() {
        object_type = (*acpi_ps_get_opcode_info((*(*op).common.value.arg).common.aml_opcode)).object_type;
        if !(*op).common.node.is_null() { (*(*op).common.node).type_ = object_type as u8; }
    }
    #[cfg(feature = "acpi_asl_compiler")]
    if acpi_gbl_disasm_flag && !(*op).common.node.is_null() && (*op).common.aml_opcode == AML_EXTERNAL_OP {
        let param_count = (*(*(*op).common.value.arg).common.next).common.value.integer as u8;
        object_type = (*(*op).common.value.arg).common.value.integer as u8;
        (*(*op).common.node).flags |= ANOBJ_IS_EXTERNAL;
        (*(*op).common.node).type_ = object_type as u8;
        acpi_dm_create_subobject_for_external(object_type as u8, &mut (*op).common.node, param_count);
        acpi_dm_add_op_to_external_list(op, (*op).named.path, object_type as u8, param_count,
            ACPI_EXT_ORIGIN_FROM_OPCODE | ACPI_EXT_RESOLVED_REFERENCE);
    }
    if (*walk_state).method_node.is_null() && (*op).common.aml_opcode == AML_METHOD_OP &&
        acpi_ns_get_attached_object((*op).named.node).is_null() {
        (*walk_state).operands[0] = (*op).named.node as *mut core::ffi::c_void;
        (*walk_state).num_operands = 1;
        status = acpi_ds_create_operands(walk_state, (*op).common.value.arg);
        if ACPI_SUCCESS(status) { status = acpi_ex_create_method((*op).named.data, (*op).named.length, walk_state); }
        (*walk_state).operands[0] = core::ptr::null_mut();
        (*walk_state).num_operands = 0;
        if ACPI_FAILURE(status) { return status; }
    }
    if (*walk_state).method_node.is_null() && (*op).common.aml_opcode != AML_EXTERNAL_OP && acpi_ns_opens_scope(object_type) {
        status = acpi_ds_scope_stack_pop(walk_state);
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
