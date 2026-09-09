// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Module Name: dbxface - AML Debugger external interfaces

// C includes and build-time ACPI configuration are supplied by the surrounding
// translation unit.

unsafe fn acpi_db_start_command(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let mut status: acpi_status;

    // TBD: [Investigate] are there namespace locking issues here?
    // acpi_ut_release_mutex (ACPI_MTX_NAMESPACE);

    acpi_gbl_method_executing = TRUE;
    status = AE_CTRL_TRUE;

    while status == AE_CTRL_TRUE {
        status = acpi_os_notify_command_complete();
        if ACPI_FAILURE(status) { break; }

        status = acpi_os_wait_command_ready();
        if ACPI_FAILURE(status) { break; }

        status = acpi_db_command_dispatch(acpi_gbl_db_line_buf, walk_state, op);
    }

    if ACPI_FAILURE(status) && status != AE_CTRL_TERMINATE {
        ACPI_EXCEPTION((AE_INFO, status, "While parsing/handling command line"));
    }
    status
}

pub unsafe fn acpi_db_signal_break_point(walk_state: *mut acpi_walk_state) {
    // C condition: compiled out for ACPI_APPLICATION.
    #[cfg(not(feature = "acpi_application"))]
    if acpi_gbl_db_thread_id != acpi_os_get_thread_id() { return; }

    acpi_gbl_cm_single_step = TRUE;
    acpi_os_printf("**break** Executed AML BreakPoint opcode\n");
}

// C condition: this helper is present only when ACPI_DISASSEMBLER is enabled.
#[cfg(feature = "acpi_disassembler")]
unsafe fn acpi_db_get_display_op(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) -> *mut acpi_parse_object {
    let mut display_op = op;
    let mut parent_op = (*op).common.parent;
    if !parent_op.is_null() {
        if !(*walk_state).control_state.is_null()
            && (*(*walk_state).control_state).common.state == ACPI_CONTROL_PREDICATE_EXECUTING
        {
            while !parent_op.is_null() {
                if (*parent_op).common.aml_opcode == AML_IF_OP
                    || (*parent_op).common.aml_opcode == AML_WHILE_OP {
                    display_op = parent_op;
                    break;
                }
                parent_op = (*parent_op).common.parent;
            }
        } else {
            while !parent_op.is_null() {
                if (*parent_op).common.aml_opcode == AML_IF_OP
                    || (*parent_op).common.aml_opcode == AML_ELSE_OP
                    || (*parent_op).common.aml_opcode == AML_SCOPE_OP
                    || (*parent_op).common.aml_opcode == AML_METHOD_OP
                    || (*parent_op).common.aml_opcode == AML_WHILE_OP { break; }
                display_op = parent_op;
                parent_op = (*parent_op).common.parent;
            }
        }
    }
    display_op
}

pub unsafe fn acpi_db_single_step(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
    opcode_class: u32,
) -> acpi_status {
    let mut status = AE_OK;
    let mut original_debug_level: u32;
    let aml_offset: u32;

    ACPI_FUNCTION_ENTRY();

    #[cfg(not(feature = "acpi_application"))]
    if acpi_gbl_db_thread_id != acpi_os_get_thread_id() { return AE_OK; }

    if acpi_gbl_abort_method {
        acpi_gbl_abort_method = FALSE;
        return AE_ABORT_METHOD;
    }

    aml_offset = ACPI_PTR_DIFF((*op).common.aml, (*walk_state).parser_state.aml_start) as u32;

    if (*walk_state).method_breakpoint != 0 && (*walk_state).method_breakpoint <= aml_offset {
        acpi_os_printf("***Break*** at AML offset %X\n", aml_offset);
        acpi_gbl_cm_single_step = TRUE;
        acpi_gbl_step_to_next_call = FALSE;
        (*walk_state).method_breakpoint = 0;
    } else if (*walk_state).user_breakpoint != 0 && (*walk_state).user_breakpoint == aml_offset {
        acpi_os_printf("***UserBreakpoint*** at AML offset %X\n", aml_offset);
        acpi_gbl_cm_single_step = TRUE;
        acpi_gbl_step_to_next_call = FALSE;
        (*walk_state).method_breakpoint = 0;
    }

    if (*op).common.aml_opcode == AML_INT_NAMEDFIELD_OP { return AE_OK; }
    match opcode_class {
        AML_CLASS_UNKNOWN | AML_CLASS_ARGUMENT => return AE_OK,
        _ => {}
    }

    if acpi_gbl_db_output_to_file || acpi_gbl_cm_single_step || (acpi_dbg_level & ACPI_LV_PARSE) != 0 {
        if acpi_gbl_db_output_to_file || (acpi_dbg_level & ACPI_LV_PARSE) != 0 {
            acpi_os_printf("\nAML Debug: Next AML Opcode to execute:\n");
        }
        original_debug_level = acpi_dbg_level;
        acpi_dbg_level &= !(ACPI_LV_PARSE | ACPI_LV_FUNCTIONS);
        let next = (*op).common.next;
        (*op).common.next = core::ptr::null_mut();
        #[cfg(feature = "acpi_disassembler")]
        acpi_dm_disassemble(walk_state, acpi_db_get_display_op(walk_state, op), ACPI_UINT32_MAX);
        #[cfg(not(feature = "acpi_disassembler"))]
        acpi_os_printf("AML Opcode: %4.4X %s\n", (*op).common.aml_opcode,
            acpi_ps_get_opcode_name((*op).common.aml_opcode));
        if (*op).common.aml_opcode == AML_IF_OP || (*op).common.aml_opcode == AML_WHILE_OP {
            if (*(*walk_state).control_state).common.value != 0 {
                acpi_os_printf("Predicate = [True], IF block was executed\n");
            } else { acpi_os_printf("Predicate = [False], Skipping IF block\n"); }
        } else if (*op).common.aml_opcode == AML_ELSE_OP {
            acpi_os_printf("Predicate = [False], ELSE block was executed\n");
        }
        (*op).common.next = next;
        acpi_os_printf("\n");
        if acpi_gbl_db_output_to_file || (acpi_dbg_level & ACPI_LV_PARSE) != 0 { acpi_os_printf("\n"); }
        acpi_dbg_level = original_debug_level;
    }
    if !acpi_gbl_cm_single_step { return AE_OK; }
    if acpi_gbl_step_to_next_call {
        if (*op).common.aml_opcode != AML_INT_METHODCALL_OP { return AE_OK; }
        acpi_gbl_step_to_next_call = FALSE;
    }
    if (*op).common.aml_opcode == AML_INT_METHODCALL_OP {
        acpi_gbl_cm_single_step = FALSE;
        (*walk_state).method_breakpoint = 1;
    }
    acpi_ex_exit_interpreter();
    status = acpi_db_start_command(walk_state, op);
    acpi_ex_enter_interpreter();
    status
}

pub unsafe fn acpi_initialize_debugger() -> acpi_status {
    ACPI_FUNCTION_TRACE(acpi_initialize_debugger);
    acpi_gbl_db_buffer = core::ptr::null_mut();
    acpi_gbl_db_filename = core::ptr::null_mut();
    acpi_gbl_db_output_to_file = FALSE;
    acpi_gbl_db_debug_level = ACPI_LV_VERBOSITY2;
    acpi_gbl_db_console_debug_level = ACPI_NORMAL_DEFAULT | ACPI_LV_TABLES;
    acpi_gbl_db_output_flags = ACPI_DB_CONSOLE_OUTPUT;
    acpi_gbl_db_opt_no_ini_methods = FALSE;
    acpi_gbl_db_opt_no_region_support = FALSE;
    acpi_gbl_db_buffer = acpi_os_allocate(ACPI_DEBUG_BUFFER_SIZE);
    if acpi_gbl_db_buffer.is_null() { return AE_NO_MEMORY; }
    core::ptr::write_bytes(acpi_gbl_db_buffer, 0, ACPI_DEBUG_BUFFER_SIZE as usize);
    acpi_gbl_db_scope_buf[0] = AML_ROOT_PREFIX;
    acpi_gbl_db_scope_buf[1] = 0;
    acpi_gbl_db_scope_node = acpi_gbl_root_node;
    acpi_gbl_db_terminate_loop = FALSE;
    if acpi_gbl_debugger_configuration & DEBUGGER_MULTI_THREADED != 0 {
        let mut status = acpi_os_initialize_debugger();
        if ACPI_FAILURE(status) { acpi_os_printf("Could not get debugger mutex\n"); return status; }
        acpi_gbl_db_threads_terminated = FALSE;
        status = acpi_os_execute(OSL_DEBUGGER_MAIN_THREAD, acpi_db_execute_thread, core::ptr::null_mut());
        if ACPI_FAILURE(status) { acpi_gbl_db_threads_terminated = TRUE; return status; }
    } else { acpi_gbl_db_thread_id = acpi_os_get_thread_id(); }
    AE_OK
}

pub unsafe fn acpi_terminate_debugger() {
    acpi_gbl_db_terminate_loop = TRUE;
    if acpi_gbl_debugger_configuration & DEBUGGER_MULTI_THREADED != 0 {
        while !acpi_gbl_db_threads_terminated { acpi_os_sleep(100); }
        acpi_os_terminate_debugger();
    }
    if !acpi_gbl_db_buffer.is_null() { acpi_os_free(acpi_gbl_db_buffer); acpi_gbl_db_buffer = core::ptr::null_mut(); }
    acpi_gbl_db_output_flags = ACPI_DB_DISABLE_OUTPUT;
}

pub unsafe fn acpi_set_debugger_thread_id(thread_id: acpi_thread_id) {
    acpi_gbl_db_thread_id = thread_id;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
