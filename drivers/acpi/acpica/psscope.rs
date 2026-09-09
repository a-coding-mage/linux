// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: psscope - Parser scope stack management routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation unit.

/* Get parent of current op being parsed. */
pub unsafe fn acpi_ps_get_parent_scope(
    parser_state: *mut acpi_parse_state,
) -> *mut acpi_parse_object {
    (*(*parser_state).scope).parse_scope.op
}

/* Is parsing of current argument complete? */
pub unsafe fn acpi_ps_has_completed_scope(parser_state: *mut acpi_parse_state) -> u8 {
    ((*parser_state).aml >= (*(*parser_state).scope).parse_scope.arg_end
        || (*(*parser_state).scope).parse_scope.arg_count == 0) as u8
}

/* Allocate and initialize a new scope object. */
pub unsafe fn acpi_ps_init_scope(
    parser_state: *mut acpi_parse_state,
    root_op: *mut acpi_parse_object,
) -> acpi_status {
    let scope: *mut acpi_generic_state = acpi_ut_create_generic_state();
    if scope.is_null() {
        return AE_NO_MEMORY;
    }

    (*scope).common.descriptor_type = ACPI_DESC_TYPE_STATE_RPSCOPE;
    (*scope).parse_scope.op = root_op;
    (*scope).parse_scope.arg_count = ACPI_VAR_ARGS;
    (*scope).parse_scope.arg_end = (*parser_state).aml_end;
    (*scope).parse_scope.pkg_end = (*parser_state).aml_end;

    (*parser_state).scope = scope;
    (*parser_state).start_op = root_op;

    AE_OK
}

/* Push current op to begin parsing its argument. */
pub unsafe fn acpi_ps_push_scope(
    parser_state: *mut acpi_parse_state,
    op: *mut acpi_parse_object,
    remaining_args: u32,
    arg_count: u32,
) -> acpi_status {
    let scope: *mut acpi_generic_state = acpi_ut_create_generic_state();
    if scope.is_null() {
        return AE_NO_MEMORY;
    }

    (*scope).common.descriptor_type = ACPI_DESC_TYPE_STATE_PSCOPE;
    (*scope).parse_scope.op = op;
    (*scope).parse_scope.arg_list = remaining_args;
    (*scope).parse_scope.arg_count = arg_count;
    (*scope).parse_scope.pkg_end = (*parser_state).pkg_end;

    /* Push onto scope stack */
    acpi_ut_push_generic_state(&mut (*parser_state).scope, scope);

    if arg_count == ACPI_VAR_ARGS {
        /* Multiple arguments */
        (*scope).parse_scope.arg_end = (*parser_state).pkg_end;
    } else {
        /* Single argument */
        (*scope).parse_scope.arg_end = usize::MAX as *mut _;
    }

    AE_OK
}

/* Return to parsing a previous op. */
pub unsafe fn acpi_ps_pop_scope(
    parser_state: *mut acpi_parse_state,
    op: *mut *mut acpi_parse_object,
    arg_list: *mut u32,
    arg_count: *mut u32,
) {
    let mut scope: *mut acpi_generic_state = (*parser_state).scope;

    /* Only pop the scope if there is in fact a next scope */
    if !(*scope).common.next.is_null() {
        scope = acpi_ut_pop_generic_state(&mut (*parser_state).scope);

        /* Return to parsing previous op */
        *op = (*scope).parse_scope.op;
        *arg_list = (*scope).parse_scope.arg_list;
        *arg_count = (*scope).parse_scope.arg_count;
        (*parser_state).pkg_end = (*scope).parse_scope.pkg_end;

        /* All done with this scope state structure */
        acpi_ut_delete_generic_state(scope);
    } else {
        /* Empty parse stack, prepare to fetch next opcode */
        *op = core::ptr::null_mut();
        *arg_list = 0;
        *arg_count = 0;
    }
}

/* Destroy available list, remaining stack levels, and return root scope. */
pub unsafe fn acpi_ps_cleanup_scope(parser_state: *mut acpi_parse_state) {
    if parser_state.is_null() {
        return;
    }

    /* Delete anything on the scope stack */
    while !(*parser_state).scope.is_null() {
        let scope = acpi_ut_pop_generic_state(&mut (*parser_state).scope);
        acpi_ut_delete_generic_state(scope);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
