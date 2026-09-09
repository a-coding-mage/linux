// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Main AML parse loop. C headers and build-time configuration are supplied by
// the surrounding ACPICA translation unit.

static unsafe fn acpi_ps_get_arguments(
    walk_state: *mut acpi_walk_state,
    aml_op_start: *mut u8,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut arg: *mut acpi_parse_object = core::ptr::null_mut();

    ACPI_FUNCTION_TRACE_PTR!(ps_get_arguments, walk_state);
    ACPI_DEBUG_PRINT!((ACPI_DB_PARSE, "Get arguments for opcode [%s]\n", (*op).common.aml_op_name));

    match (*op).common.aml_opcode {
        AML_BYTE_OP | AML_WORD_OP | AML_DWORD_OP | AML_QWORD_OP | AML_STRING_OP => {
            acpi_ps_get_next_simple_arg(
                &mut (*walk_state).parser_state,
                GET_CURRENT_ARG_TYPE!((*walk_state).arg_types),
                op,
            );
        }
        AML_INT_NAMEPATH_OP => {
            status = acpi_ps_get_next_namepath(
                walk_state,
                &mut (*walk_state).parser_state,
                op,
                ACPI_POSSIBLE_METHOD_CALL,
            );
            if ACPI_FAILURE!(status) {
                return_ACPI_STATUS!(status);
            }
            (*walk_state).arg_types = 0;
        }
        _ => {
            while GET_CURRENT_ARG_TYPE!((*walk_state).arg_types) != 0
                && (*walk_state).arg_count == 0
            {
                (*walk_state).aml = (*walk_state).parser_state.aml;
                match (*op).common.aml_opcode {
                    AML_METHOD_OP | AML_BUFFER_OP | AML_PACKAGE_OP |
                    AML_VARIABLE_PACKAGE_OP | AML_WHILE_OP => {}
                    _ => ASL_CV_CAPTURE_COMMENTS!(walk_state),
                }
                status = acpi_ps_get_next_arg(
                    walk_state,
                    &mut (*walk_state).parser_state,
                    GET_CURRENT_ARG_TYPE!((*walk_state).arg_types),
                    &mut arg,
                );
                if ACPI_FAILURE!(status) {
                    return_ACPI_STATUS!(status);
                }
                if !arg.is_null() {
                    acpi_ps_append_arg(op, arg);
                }
                INCREMENT_ARG_LIST!((*walk_state).arg_types);
            }

            ACPI_DEBUG_PRINT!((ACPI_DB_PARSE, "Final argument count: %8.8X pass %u\n",
                (*walk_state).arg_count, (*walk_state).pass_number));

            match (*op).common.aml_opcode {
                AML_METHOD_OP => {
                    (*op).named.data = (*walk_state).parser_state.aml;
                    (*op).named.length = ((*walk_state).parser_state.pkg_end as usize -
                        (*walk_state).parser_state.aml as usize) as u32;
                    (*walk_state).parser_state.aml = (*walk_state).parser_state.pkg_end;
                    (*walk_state).arg_count = 0;
                }
                AML_BUFFER_OP | AML_PACKAGE_OP | AML_VARIABLE_PACKAGE_OP => {
                    if !(*op).common.parent.is_null()
                        && (*(*op).common.parent).common.aml_opcode == AML_NAME_OP
                        && (*walk_state).pass_number <= ACPI_IMODE_LOAD_PASS2
                    {
                        ACPI_DEBUG_PRINT!((ACPI_DB_PARSE,
                            "Setup Package/Buffer: Pass %u, AML Ptr: %p\n",
                            (*walk_state).pass_number, aml_op_start));
                        (*op).named.data = aml_op_start;
                        (*op).named.length = ((*walk_state).parser_state.pkg_end as usize -
                            aml_op_start as usize) as u32;
                        (*walk_state).parser_state.aml = (*walk_state).parser_state.pkg_end;
                        (*walk_state).arg_count = 0;
                    }
                }
                AML_WHILE_OP => {
                    if !(*walk_state).control_state.is_null() {
                        (*(*walk_state).control_state).control.package_end =
                            (*walk_state).parser_state.pkg_end;
                    }
                }
                _ => {}
            }
        }
    }
    return_ACPI_STATUS!(AE_OK);
}

pub unsafe fn acpi_ps_parse_loop(walk_state: *mut acpi_walk_state) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut op: *mut acpi_parse_object = core::ptr::null_mut();
    let parser_state: *mut acpi_parse_state = &mut (*walk_state).parser_state;
    let mut aml_op_start: *mut u8 = core::ptr::null_mut();
    let mut opcode_length: u8;

    ACPI_FUNCTION_TRACE_PTR!(ps_parse_loop, walk_state);
    if (*walk_state).descending_callback.is_none() {
        return_ACPI_STATUS!(AE_BAD_PARAMETER);
    }
    (*walk_state).arg_types = 0;

    // #ifndef ACPI_CONSTANT_EVAL_ONLY
    if (*walk_state).walk_type & ACPI_WALK_METHOD_RESTART != 0 {
        if acpi_ps_has_completed_scope(parser_state) {
            if !(*parser_state).scope.is_null()
                && !(*(*parser_state).scope).parse_scope.op.is_null()
                && ((*(*(*parser_state).scope).parse_scope.op).common.aml_opcode == AML_IF_OP
                    || (*(*(*parser_state).scope).parse_scope.op).common.aml_opcode == AML_WHILE_OP)
                && !(*walk_state).control_state.is_null()
                && (*(*walk_state).control_state).common.state == ACPI_CONTROL_PREDICATE_EXECUTING
            {
                (*walk_state).op = core::ptr::null_mut();
                status = acpi_ds_get_predicate_value(walk_state, ACPI_TO_POINTER!(TRUE));
                if ACPI_FAILURE!(status) && !ACPI_CNTL_EXCEPTION!(status) {
                    if status == AE_AML_NO_RETURN_VALUE {
                        ACPI_EXCEPTION!((AE_INFO, status, "Invoked method did not return a value"));
                    }
                    ACPI_EXCEPTION!((AE_INFO, status, "GetPredicate Failed"));
                    return_ACPI_STATUS!(status);
                }
                status = acpi_ps_next_parse_state(walk_state, op, status);
            }
            acpi_ps_pop_scope(parser_state, &mut op, &mut (*walk_state).arg_types,
                &mut (*walk_state).arg_count);
        } else if !(*walk_state).prev_op.is_null() {
            op = (*walk_state).prev_op;
            (*walk_state).arg_types = (*walk_state).prev_arg_types;
        }
    }

    while (*parser_state).aml < (*parser_state).aml_end || !op.is_null() {
        ASL_CV_CAPTURE_COMMENTS!(walk_state);
        aml_op_start = (*parser_state).aml;
        if op.is_null() {
            status = acpi_ps_create_op(walk_state, aml_op_start, &mut op);
            if ACPI_FAILURE!(status) {
                if (*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL != 0
                    && (status == AE_ALREADY_EXISTS || status == AE_NOT_FOUND) { status = AE_OK; }
                if status == AE_CTRL_PARSE_CONTINUE { continue; }
                if status == AE_CTRL_PARSE_PENDING { status = AE_OK; }
                if status == AE_CTRL_TERMINATE { return_ACPI_STATUS!(status); }
                status = acpi_ps_complete_op(walk_state, &mut op, status);
                if ACPI_FAILURE!(status) { return_ACPI_STATUS!(status); }
                if acpi_ns_opens_scope(acpi_ps_get_opcode_info((*walk_state).opcode).object_type) {
                    ACPI_INFO!(("Skipping parse of AML opcode: %s (0x%4.4X)",
                        acpi_ps_get_opcode_name((*walk_state).opcode), (*walk_state).opcode));
                    opcode_length = if (*walk_state).opcode & 0xFF00 == AML_EXTENDED_OPCODE { 2 } else { 1 };
                    (*walk_state).parser_state.aml = (*walk_state).aml.add(opcode_length as usize);
                    (*walk_state).parser_state.aml = acpi_ps_get_next_package_end(&mut (*walk_state).parser_state);
                    if (*walk_state).parser_state.aml > (*walk_state).parser_state.aml_end
                        || (*walk_state).parser_state.aml < (*walk_state).aml { return_ACPI_STATUS!(AE_AML_PACKAGE_LIMIT); }
                    (*walk_state).aml = (*walk_state).parser_state.aml;
                }
                continue;
            }
            acpi_ex_start_trace_opcode(op, walk_state);
        }
        (*walk_state).arg_count = 0;
        match (*op).common.aml_opcode { AML_BYTE_OP | AML_WORD_OP | AML_DWORD_OP | AML_QWORD_OP => {}, _ => ASL_CV_CAPTURE_COMMENTS!(walk_state) }

        if (*walk_state).arg_types != 0 {
            status = acpi_ps_get_arguments(walk_state, aml_op_start, op);
            if ACPI_FAILURE!(status) {
                status = acpi_ps_complete_op(walk_state, &mut op, status);
                if ACPI_FAILURE!(status) { return_ACPI_STATUS!(status); }
                if !(*walk_state).control_state.is_null()
                    && ((*(*walk_state).control_state).control.opcode == AML_IF_OP
                        || (*(*walk_state).control_state).control.opcode == AML_WHILE_OP)
                {
                    (*parser_state).aml = (*(*walk_state).control_state).control.aml_predicate_start.add(1);
                    (*parser_state).aml = acpi_ps_get_next_package_end(parser_state);
                    if (*parser_state).aml > (*parser_state).aml_end
                        || (*parser_state).aml < (*(*walk_state).control_state).control.aml_predicate_start { return_ACPI_STATUS!(AE_AML_PACKAGE_LIMIT); }
                    (*walk_state).aml = (*parser_state).aml;
                    ACPI_ERROR!((AE_INFO, "Skipping While/If block"));
                    if (*walk_state).aml < (*parser_state).aml_end && *(*walk_state).aml == AML_ELSE_OP {
                        ACPI_ERROR!((AE_INFO, "Skipping Else block"));
                        (*parser_state).aml = (*walk_state).aml.add(1);
                        (*parser_state).aml = acpi_ps_get_next_package_end(parser_state);
                        if (*parser_state).aml > (*parser_state).aml_end || (*parser_state).aml < (*walk_state).aml { return_ACPI_STATUS!(AE_AML_PACKAGE_LIMIT); }
                        (*walk_state).aml = (*parser_state).aml;
                    }
                    ACPI_FREE!(acpi_ut_pop_generic_state(&mut (*walk_state).control_state));
                }
                op = core::ptr::null_mut();
                continue;
            }
        }
        if (*walk_state).arg_count != 0 {
            status = acpi_ps_push_scope(parser_state, op, (*walk_state).arg_types, (*walk_state).arg_count);
            if ACPI_FAILURE!(status) {
                status = acpi_ps_complete_op(walk_state, &mut op, status);
                if ACPI_FAILURE!(status) { return_ACPI_STATUS!(status); }
                continue;
            }
            op = core::ptr::null_mut();
            continue;
        }

        (*walk_state).op_info = acpi_ps_get_opcode_info((*op).common.aml_opcode);
        if (*walk_state).op_info.flags & AML_NAMED != 0
            && ((*op).common.aml_opcode == AML_REGION_OP || (*op).common.aml_opcode == AML_DATA_REGION_OP) {
            (*op).named.length = ((*parser_state).aml as usize - (*op).named.data as usize) as u32;
        }
        if (*walk_state).op_info.flags & AML_CREATE != 0 {
            (*op).named.length = ((*parser_state).aml as usize - (*op).named.data as usize) as u32;
        }
        if (*op).common.aml_opcode == AML_BANK_FIELD_OP {
            (*op).named.length = ((*parser_state).aml as usize - (*op).named.data as usize) as u32;
        }
        if let Some(callback) = (*walk_state).ascending_callback {
            (*walk_state).op = op;
            (*walk_state).opcode = (*op).common.aml_opcode;
            status = callback(walk_state);
            status = acpi_ps_next_parse_state(walk_state, op, status);
            if status == AE_CTRL_PENDING { status = AE_OK; }
            else if (*walk_state).parse_flags & ACPI_PARSE_MODULE_LEVEL != 0
                && (ACPI_AML_EXCEPTION!(status) || status == AE_ALREADY_EXISTS || status == AE_NOT_FOUND) { status = AE_OK; }
        }
        status = acpi_ps_complete_op(walk_state, &mut op, status);
        if ACPI_FAILURE!(status) { return_ACPI_STATUS!(status); }
    }
    status = acpi_ps_complete_final_op(walk_state, op, status);
    return_ACPI_STATUS!(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
