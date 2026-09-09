// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dscontrol - Support for execution control opcodes -
 *                          if/else/while/return
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// C dependencies supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ds_exec_begin_control_op(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut control_state: *mut acpi_generic_state;

    match (*op).common.aml_opcode {
        AML_WHILE_OP => {
            /* If this is an additional iteration of a while loop, continue. */
            if !(*walk_state).control_state.is_null()
                && (*(*walk_state).control_state).control.aml_predicate_start
                    == (*walk_state).parser_state.aml.offset(-1)
            {
                (*(*walk_state).control_state).common.state =
                    ACPI_CONTROL_CONDITIONAL_EXECUTING;
                return status;
            }
            // Fall through to AML_IF_OP.
            control_state = acpi_ut_create_control_state();
            if control_state.is_null() {
                status = AE_NO_MEMORY;
            } else {
                (*control_state).control.aml_predicate_start =
                    (*walk_state).parser_state.aml.offset(-1);
                (*control_state).control.package_end = (*walk_state).parser_state.pkg_end;
                (*control_state).control.opcode = (*op).common.aml_opcode;
                (*control_state).control.loop_timeout = acpi_os_get_timer()
                    .wrapping_add((acpi_gbl_max_loop_iterations as u64)
                        .wrapping_mul(ACPI_100NSEC_PER_SEC));
                acpi_ut_push_generic_state(&mut (*walk_state).control_state, control_state);
            }
        }
        AML_IF_OP => {
            control_state = acpi_ut_create_control_state();
            if control_state.is_null() {
                status = AE_NO_MEMORY;
            } else {
                (*control_state).control.aml_predicate_start =
                    (*walk_state).parser_state.aml.offset(-1);
                (*control_state).control.package_end = (*walk_state).parser_state.pkg_end;
                (*control_state).control.opcode = (*op).common.aml_opcode;
                (*control_state).control.loop_timeout = acpi_os_get_timer()
                    .wrapping_add((acpi_gbl_max_loop_iterations as u64)
                        .wrapping_mul(ACPI_100NSEC_PER_SEC));
                acpi_ut_push_generic_state(&mut (*walk_state).control_state, control_state);
            }
        }
        AML_ELSE_OP => {
            if (*walk_state).last_predicate != 0 { status = AE_CTRL_TRUE; }
        }
        AML_RETURN_OP => {}
        _ => {}
    }
    status
}

pub unsafe fn acpi_ds_exec_end_control_op(
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut control_state: *mut acpi_generic_state;

    match (*op).common.aml_opcode {
        AML_IF_OP => {
            (*walk_state).last_predicate = (*(*walk_state).control_state).common.value as u8;
            control_state = acpi_ut_pop_generic_state(&mut (*walk_state).control_state);
            acpi_ut_delete_generic_state(control_state);
        }
        AML_ELSE_OP => {}
        AML_WHILE_OP => {
            control_state = (*walk_state).control_state;
            if (*control_state).common.value != 0 {
                if acpi_os_get_timer().wrapping_after((*control_state).control.loop_timeout) {
                    status = AE_AML_LOOP_TIMEOUT;
                } else {
                    status = AE_CTRL_PENDING;
                    (*walk_state).aml_last_while =
                        (*control_state).control.aml_predicate_start;
                }
            } else {
                control_state = acpi_ut_pop_generic_state(&mut (*walk_state).control_state);
                acpi_ut_delete_generic_state(control_state);
            }
        }
        AML_RETURN_OP => {
            if !(*op).common.value.arg.is_null() {
                acpi_ds_clear_implicit_return(walk_state);
                status = acpi_ds_create_operands(walk_state, (*op).common.value.arg);
                if ACPI_FAILURE(status) { return status; }
                status = acpi_ex_resolve_to_value(&mut (*walk_state).operands[0], walk_state);
                if ACPI_FAILURE(status) { return status; }
                (*walk_state).return_desc = (*walk_state).operands[0];
            } else if (*walk_state).result_count != 0 {
                acpi_ds_clear_implicit_return(walk_state);
                let desc = (*walk_state).results.results.obj_desc[0];
                if ACPI_GET_DESCRIPTOR_TYPE(desc) == ACPI_DESC_TYPE_OPERAND
                    && (*desc).common.type_ == ACPI_TYPE_LOCAL_REFERENCE
                    && (*desc).reference.class_ != ACPI_REFCLASS_INDEX
                {
                    status = acpi_ex_resolve_to_value(
                        &mut (*walk_state).results.results.obj_desc[0], walk_state);
                    if ACPI_FAILURE(status) { return status; }
                }
                (*walk_state).return_desc = (*walk_state).results.results.obj_desc[0];
            } else {
                if (*walk_state).num_operands != 0 {
                    acpi_ut_remove_reference((*walk_state).operands[0]);
                }
                (*walk_state).operands[0] = core::ptr::null_mut();
                (*walk_state).num_operands = 0;
                (*walk_state).return_desc = core::ptr::null_mut();
            }
            status = AE_CTRL_TERMINATE;
        }
        AML_NOOP_OP => {}
        AML_BREAKPOINT_OP => {
            acpi_db_signal_break_point(walk_state);
            status = acpi_os_signal(ACPI_SIGNAL_BREAKPOINT, b"Executed AML Breakpoint opcode\0".as_ptr() as *const i8);
        }
        AML_BREAK_OP | AML_CONTINUE_OP => {
            while !(*walk_state).control_state.is_null()
                && (*(*walk_state).control_state).control.opcode != AML_WHILE_OP
            {
                control_state = acpi_ut_pop_generic_state(&mut (*walk_state).control_state);
                acpi_ut_delete_generic_state(control_state);
            }
            if (*walk_state).control_state.is_null() { return AE_AML_NO_WHILE; }
            (*walk_state).aml_last_while = (*(*walk_state).control_state).control.package_end;
            status = if (*op).common.aml_opcode == AML_BREAK_OP { AE_CTRL_BREAK } else { AE_CTRL_CONTINUE };
        }
        _ => { status = AE_AML_BAD_OPCODE; }
    }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
