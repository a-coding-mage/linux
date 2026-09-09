// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Support for parse objects. Direct translation of psobject.c.

// External ACPICA types, constants, macros, and functions are supplied by the
// surrounding translation unit.

unsafe fn acpi_ps_get_aml_opcode(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut aml_offset: u32;
    ACPI_FUNCTION_TRACE_PTR!(ps_get_aml_opcode, walk_state);

    (*walk_state).aml = (*walk_state).parser_state.aml;
    (*walk_state).opcode = acpi_ps_peek_opcode(&mut (*walk_state).parser_state);
    (*walk_state).op_info = acpi_ps_get_opcode_info((*walk_state).opcode);

    match (*(*walk_state).op_info).class {
        AML_CLASS_ASCII | AML_CLASS_PREFIX => {
            (*walk_state).opcode = AML_INT_NAMEPATH_OP;
            (*walk_state).arg_types = ARGP_NAMESTRING;
        }
        AML_CLASS_UNKNOWN => {
            if (*walk_state).pass_number == 2 {
                aml_offset = ACPI_PTR_DIFF((*walk_state).aml,
                    (*walk_state).parser_state.aml_start) as u32;
                ACPI_ERROR!((AE_INFO, "Unknown opcode 0x{:02X} at table offset 0x{:04X}, ignoring",
                    (*walk_state).opcode, aml_offset + core::mem::size_of::<acpi_table_header>() as u32));
                ACPI_DUMP_BUFFER!(((*walk_state).parser_state.aml).offset(-16), 48);
            }
            (*walk_state).parser_state.aml = (*walk_state).parser_state.aml.add(1);
            if (*walk_state).opcode > 0xFF {
                (*walk_state).parser_state.aml = (*walk_state).parser_state.aml.add(1);
            }
            return AE_CTRL_PARSE_CONTINUE;
        }
        _ => {
            (*walk_state).parser_state.aml = (*walk_state).parser_state.aml.add(
                acpi_ps_get_opcode_size((*walk_state).opcode) as usize);
            (*walk_state).arg_types = (*(*walk_state).op_info).parse_args;
        }
    }
    AE_OK
}

pub unsafe fn acpi_ps_build_named_op(
    walk_state: *mut acpi_walk_state, aml_op_start: *mut u8,
    unnamed_op: *mut acpi_parse_object_union,
    op: *mut *mut acpi_parse_object_union) -> acpi_status {
    let mut status = AE_OK;
    let mut arg: *mut acpi_parse_object_union = core::ptr::null_mut();
    ACPI_FUNCTION_TRACE_PTR!(ps_build_named_op, walk_state);
    (*unnamed_op).common.value.arg = core::ptr::null_mut();
    (*unnamed_op).common.arg_list_length = 0;
    (*unnamed_op).common.aml_opcode = (*walk_state).opcode;

    while GET_CURRENT_ARG_TYPE!((*walk_state).arg_types) != 0 &&
          GET_CURRENT_ARG_TYPE!((*walk_state).arg_types) != ARGP_NAME {
        ASL_CV_CAPTURE_COMMENTS!(walk_state);
        status = acpi_ps_get_next_arg(walk_state, &mut (*walk_state).parser_state,
            GET_CURRENT_ARG_TYPE!((*walk_state).arg_types), &mut arg);
        if ACPI_FAILURE!(status) { return status; }
        acpi_ps_append_arg(unnamed_op, arg);
        INCREMENT_ARG_LIST!((*walk_state).arg_types);
    }
    ASL_CV_CAPTURE_COMMENTS!(walk_state);
    if GET_CURRENT_ARG_TYPE!((*walk_state).arg_types) == 0 { return AE_AML_NO_OPERAND; }
    INCREMENT_ARG_LIST!((*walk_state).arg_types);
    (*walk_state).op = core::ptr::null_mut();
    status = ((*walk_state).descending_callback.unwrap())(walk_state, op);
    if ACPI_FAILURE!(status) { return status; }
    if (*op).is_null() { return AE_CTRL_PARSE_CONTINUE; }
    status = acpi_ps_next_parse_state(walk_state, *op, status);
    if ACPI_FAILURE!(status) { if status == AE_CTRL_PENDING { status = AE_CTRL_PARSE_PENDING; } return status; }
    acpi_ps_append_arg(*op, (*unnamed_op).common.value.arg);
    if (*op).as_ref().unwrap().common.aml_opcode == AML_REGION_OP ||
       (*op).as_ref().unwrap().common.aml_opcode == AML_DATA_REGION_OP {
        (*op).as_mut().unwrap().named.data = aml_op_start;
        (*op).as_mut().unwrap().named.length = 0;
    }
    AE_OK
}

pub unsafe fn acpi_ps_create_op(walk_state: *mut acpi_walk_state,
    aml_op_start: *mut u8, new_op: *mut *mut acpi_parse_object_union) -> acpi_status {
    let mut status = acpi_ps_get_aml_opcode(walk_state);
    if status == AE_CTRL_PARSE_CONTINUE || ACPI_FAILURE!(status) { return status; }
    (*walk_state).op_info = acpi_ps_get_opcode_info((*walk_state).opcode);
    let op = acpi_ps_alloc_op((*walk_state).opcode, aml_op_start);
    if op.is_null() { return AE_NO_MEMORY; }
    if (*(*walk_state).op_info).flags & AML_NAMED != 0 {
        let mut named_op: *mut acpi_parse_object_union = core::ptr::null_mut();
        status = acpi_ps_build_named_op(walk_state, aml_op_start, op, &mut named_op);
        acpi_ps_free_op(op);
        if ACPI_FAILURE!(status) { return status; }
        *new_op = named_op;
        return AE_OK;
    }
    if (*(*walk_state).op_info).flags & AML_CREATE != 0 || (*walk_state).opcode == AML_BANK_FIELD_OP {
        (*op).named.data = aml_op_start; (*op).named.length = 0;
    }
    let parent_scope = acpi_ps_get_parent_scope(&mut (*walk_state).parser_state);
    acpi_ps_append_arg(parent_scope, op);
    if !parent_scope.is_null() { let info = acpi_ps_get_opcode_info((*parent_scope).common.aml_opcode);
        if (*info).flags & AML_HAS_TARGET != 0 && (*parent_scope).common.arg_list_length > acpi_ps_get_argument_count((*info).type_) { (*op).common.flags |= ACPI_PARSEOP_TARGET; }
        else if (*parent_scope).common.aml_opcode == AML_INCREMENT_OP || (*parent_scope).common.aml_opcode == AML_DECREMENT_OP { (*op).common.flags |= ACPI_PARSEOP_TARGET; }
    }
    if let Some(cb) = (*walk_state).descending_callback { (*walk_state).op = {*new_op = op; op}; status = cb(walk_state, &mut (*walk_state).op); status = acpi_ps_next_parse_state(walk_state, op, status); if status == AE_CTRL_PENDING { status = AE_CTRL_PARSE_PENDING; } }
    status
}

pub unsafe fn acpi_ps_complete_op(walk_state: *mut acpi_walk_state, op: *mut *mut acpi_parse_object_union, mut status: acpi_status) -> acpi_status {
    let status2 = acpi_ps_complete_this_op(walk_state, *op); if ACPI_FAILURE!(status2) { return status2; }
    (*walk_state).parser_state.scope.as_mut().unwrap().parse_scope.arg_count -= 1; *op = core::ptr::null_mut();
    match status { AE_OK => {}, AE_CTRL_TRANSFER => { (*walk_state).prev_op = core::ptr::null_mut(); (*walk_state).prev_arg_types = (*walk_state).arg_types; }, AE_CTRL_END => { acpi_ps_pop_scope(&mut (*walk_state).parser_state, op, &mut (*walk_state).arg_types, &mut (*walk_state).arg_count); }, _ => { while !(*op).is_null() { acpi_ps_complete_this_op(walk_state, *op)?; acpi_ps_pop_scope(&mut (*walk_state).parser_state, op, &mut (*walk_state).arg_types, &mut (*walk_state).arg_count); } if (*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL != 0 { return AE_OK; } } }
    if acpi_ps_has_completed_scope(&mut (*walk_state).parser_state) { acpi_ps_pop_scope(&mut (*walk_state).parser_state, op, &mut (*walk_state).arg_types, &mut (*walk_state).arg_count); } else { *op = core::ptr::null_mut(); } AE_OK
}

pub unsafe fn acpi_ps_complete_final_op(walk_state: *mut acpi_walk_state, mut op: *mut acpi_parse_object_union, mut status: acpi_status) -> acpi_status {
    let return_status = status; while !op.is_null() { status = acpi_ps_complete_this_op(walk_state, op); if ACPI_FAILURE!(status) { return status; } acpi_ps_pop_scope(&mut (*walk_state).parser_state, &mut op, &mut (*walk_state).arg_types, &mut (*walk_state).arg_count); } return_status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
