// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dswscope - Scope stack manipulation
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPICA headers and other translation units.

/*
 * FUNCTION:    acpi_ds_scope_stack_clear
 *
 * DESCRIPTION: Pop (and free) everything on the scope stack except the
 *              root scope object (which remains at the stack top.)
 */
pub unsafe fn acpi_ds_scope_stack_clear(walk_state: *mut acpi_walk_state) {
    let mut scope_info: *mut acpi_generic_state;

    acpi_function_name!(ds_scope_stack_clear);

    while !(*walk_state).scope_info.is_null() {
        /* Pop a scope off the stack */
        scope_info = (*walk_state).scope_info;
        (*walk_state).scope_info = (*scope_info).scope.next;

        acpi_debug_print!(
            ACPI_DB_EXEC,
            "Popped object type ({})\n",
            acpi_ut_get_type_name((*scope_info).common.value)
        );

        acpi_ut_delete_generic_state(scope_info);
    }
}

/*
 * FUNCTION:    acpi_ds_scope_stack_push
 *
 * DESCRIPTION: Push the current scope on the scope stack, and make the
 *              passed Node current.
 */
pub unsafe fn acpi_ds_scope_stack_push(
    node: *mut acpi_namespace_node,
    type_: acpi_object_type,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let scope_info: *mut acpi_generic_state;
    let old_scope_info: *mut acpi_generic_state;

    acpi_function_trace!(ds_scope_stack_push);

    if node.is_null() {
        /* Invalid scope */
        acpi_error!(AE_INFO, "Null scope parameter");
        return AE_BAD_PARAMETER;
    }

    /* Make sure object type is valid */
    if !acpi_ut_valid_object_type(type_) {
        acpi_warning!(AE_INFO, "Invalid object type: 0x{:X}", type_);
    }

    /* Allocate a new scope object */
    scope_info = acpi_ut_create_generic_state();
    if scope_info.is_null() {
        return AE_NO_MEMORY;
    }

    /* Init new scope object */
    (*scope_info).common.descriptor_type = ACPI_DESC_TYPE_STATE_WSCOPE;
    (*scope_info).scope.node = node;
    (*scope_info).common.value = type_ as u16;

    (*walk_state).scope_depth += 1;

    acpi_debug_print!(
        ACPI_DB_EXEC,
        "[{:.2}] Pushed scope ",
        (*walk_state).scope_depth as u32
    );

    old_scope_info = (*walk_state).scope_info;
    if !old_scope_info.is_null() {
        acpi_debug_print_raw!(
            ACPI_DB_EXEC,
            "[{:.4}] ({})",
            acpi_ut_get_node_name((*old_scope_info).scope.node),
            acpi_ut_get_type_name((*old_scope_info).common.value)
        );
    } else {
        acpi_debug_print_raw!(ACPI_DB_EXEC, ACPI_NAMESPACE_ROOT);
    }

    acpi_debug_print_raw!(
        ACPI_DB_EXEC,
        ", New scope -> [{:.4}] ({})\n",
        acpi_ut_get_node_name((*scope_info).scope.node),
        acpi_ut_get_type_name((*scope_info).common.value)
    );

    /* Push new scope object onto stack */
    acpi_ut_push_generic_state(&mut (*walk_state).scope_info, scope_info);
    AE_OK
}

/*
 * FUNCTION:    acpi_ds_scope_stack_pop
 *
 * DESCRIPTION: Pop the scope stack once.
 */
pub unsafe fn acpi_ds_scope_stack_pop(walk_state: *mut acpi_walk_state) -> acpi_status {
    let scope_info: *mut acpi_generic_state;
    let new_scope_info: *mut acpi_generic_state;

    acpi_function_trace!(ds_scope_stack_pop);

    /* Pop scope info object off the stack. */
    scope_info = acpi_ut_pop_generic_state(&mut (*walk_state).scope_info);
    if scope_info.is_null() {
        return AE_STACK_UNDERFLOW;
    }

    (*walk_state).scope_depth -= 1;

    acpi_debug_print!(
        ACPI_DB_EXEC,
        "[{:.2}] Popped scope [{:.4}] ({}), New scope -> ",
        (*walk_state).scope_depth as u32,
        acpi_ut_get_node_name((*scope_info).scope.node),
        acpi_ut_get_type_name((*scope_info).common.value)
    );

    new_scope_info = (*walk_state).scope_info;
    if !new_scope_info.is_null() {
        acpi_debug_print_raw!(
            ACPI_DB_EXEC,
            "[{:.4}] ({})\n",
            acpi_ut_get_node_name((*new_scope_info).scope.node),
            acpi_ut_get_type_name((*new_scope_info).common.value)
        );
    } else {
        acpi_debug_print_raw!(ACPI_DB_EXEC, "{}\n", ACPI_NAMESPACE_ROOT);
    }

    acpi_ut_delete_generic_state(scope_info);
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
