// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: dsdebug - Parser/Interpreter interface - debugging
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation.

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
unsafe fn acpi_ds_print_node_pathname(
    node: *mut acpi_namespace_node,
    message: *const core::ffi::c_char,
) {
    let mut buffer: acpi_buffer = core::mem::zeroed();
    let status: acpi_status;

    // ACPI_FUNCTION_TRACE(ds_print_node_pathname);

    if node.is_null() {
        // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH, "[NULL NAME]"));
        return;
    }

    /* Convert handle to full pathname and print it (with supplied message) */

    buffer.length = ACPI_ALLOCATE_LOCAL_BUFFER;

    status = acpi_ns_handle_to_pathname(node, &mut buffer, true);
    if ACPI_SUCCESS(status) {
        if !message.is_null() {
            // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH, "%s ", message));
        }

        // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH, "[%s] (Node %p)",
        //                       buffer.pointer as *const c_char, node));
        ACPI_FREE(buffer.pointer);
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ds_dump_method_stack
 *
 * PARAMETERS:  status          - Method execution status
 *              walk_state      - Current state of the parse tree walk
 *              op              - Executing parse op
 *
 * RETURN:      None
 *
 * DESCRIPTION: Called when a method has been aborted because of an error.
 *              Dumps the method execution stack.
 *
 ******************************************************************************/

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_debugger"))]
pub unsafe fn acpi_ds_dump_method_stack(
    status: acpi_status,
    walk_state: *mut acpi_walk_state,
    op: *mut acpi_parse_object,
) {
    let mut next: *mut acpi_parse_object;
    let thread: *mut acpi_thread_state;
    let mut next_walk_state: *mut acpi_walk_state;
    let mut previous_method: *mut acpi_namespace_node = core::ptr::null_mut();
    let method_desc: *mut acpi_operand_object;

    // ACPI_FUNCTION_TRACE(ds_dump_method_stack);

    /* Ignore control codes, they are not errors */

    if ACPI_CNTL_EXCEPTION(status) {
        return;
    }

    /* We may be executing a deferred opcode */

    if (*walk_state).deferred_node {
        // ACPI_DEBUG_PRINT((ACPI_DB_DISPATCH,
        //                   "Executing subtree for Buffer/Package/Region\n"));
        return;
    }

    /*
     * If there is no Thread, we are not actually executing a method.
     * This can happen when the iASL compiler calls the interpreter
     * to perform constant folding.
     */
    thread = (*walk_state).thread;
    if thread.is_null() {
        return;
    }

    /* Display exception and method name */

    // ACPI_DEBUG_PRINT((ACPI_DB_DISPATCH,
    //                   "\n**** Exception %s during execution of method ",
    //                   acpi_format_exception(status)));
    acpi_ds_print_node_pathname((*walk_state).method_node, core::ptr::null());

    // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH,
    //                       "\n\nMethod Execution Stack:\n"));
    next_walk_state = (*thread).walk_state_list;

    /* Walk list of linked walk states */

    while !next_walk_state.is_null() {
        method_desc = (*next_walk_state).method_desc;
        if !method_desc.is_null() {
            acpi_ex_stop_trace_method(
                (*method_desc).method.node as *mut acpi_namespace_node,
                method_desc,
                walk_state,
            );
        }

        // ACPI_DEBUG_PRINT((ACPI_DB_DISPATCH,
        //                   "    Method [%4.4s] executing: ",
        //                   acpi_ut_get_node_name((*next_walk_state).method_node)));

        /* First method is the currently executing method */

        if next_walk_state == walk_state {
            if !op.is_null() {
                /* Display currently executing ASL statement */

                next = (*op).common.next;
                (*op).common.next = core::ptr::null_mut();

                #[cfg(feature = "acpi_disassembler")]
                if (*walk_state).method_node != acpi_gbl_root_node {
                    /* More verbose if not module-level code */
                    acpi_os_printf();
                    acpi_dm_disassemble(next_walk_state, op, ACPI_UINT32_MAX);
                }

                (*op).common.next = next;
            }
        } else {
            /*
             * This method has called another method
             * NOTE: the method call parse subtree is already deleted at
             * this point, so we cannot disassemble the method invocation.
             */
            // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH, "Call to method "));
            acpi_ds_print_node_pathname(previous_method, core::ptr::null());
        }

        previous_method = (*next_walk_state).method_node;
        next_walk_state = (*next_walk_state).next;
        // ACPI_DEBUG_PRINT_RAW((ACPI_DB_DISPATCH, "\n"));
    }
}

#[cfg(not(any(feature = "acpi_debug_output", feature = "acpi_debugger")))]
pub unsafe fn acpi_ds_dump_method_stack(
    _status: acpi_status,
    _walk_state: *mut acpi_walk_state,
    _op: *mut acpi_parse_object,
) {
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
