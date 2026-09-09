// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: pstree - Parser op tree manipulation/traversal/search
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by ACPICA headers/modules.

/* Local prototypes */
#[cfg(feature = "ACPI_OBSOLETE_FUNCTIONS")]
extern "C" {
    fn acpi_ps_get_child(op: *mut acpi_parse_object) -> *mut acpi_parse_object;
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_get_arg
 *
 * PARAMETERS:  op              - Get an argument for this op
 *              argn            - Nth argument to get
 *
 * RETURN:      The argument (as an Op object). NULL if argument does not exist
 *
 * DESCRIPTION: Get the specified op's argument.
 *
 ******************************************************************************/

pub unsafe fn acpi_ps_get_arg(
    op: *mut acpi_parse_object,
    mut argn: u32,
) -> *mut acpi_parse_object {
    let mut arg: *mut acpi_parse_object = core::ptr::null_mut();
    let op_info: *const acpi_opcode_info;

    // ACPI_FUNCTION_ENTRY();

    /*
    if (*op).common.aml_opcode == AML_INT_CONNECTION_OP {
        return (*op).common.value.arg;
    }
    */
    /* Get the info structure for this opcode */

    op_info = acpi_ps_get_opcode_info((*op).common.aml_opcode);
    if (*op_info).class_ == AML_CLASS_UNKNOWN {

        /* Invalid opcode or ASCII character */

        return core::ptr::null_mut();
    }

    /* Check if this opcode requires argument sub-objects */

    if (*op_info).flags & AML_HAS_ARGS == 0 {

        /* Has no linked argument objects */

        return core::ptr::null_mut();
    }

    /* Get the requested argument object */

    arg = (*op).common.value.arg;
    while !arg.is_null() && argn != 0 {
        argn -= 1;
        arg = (*arg).common.next;
    }

    arg
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_append_arg
 *
 * PARAMETERS:  op              - Append an argument to this Op.
 *              arg             - Argument Op to append
 *
 * RETURN:      None.
 *
 * DESCRIPTION: Append an argument to an op's argument list (a NULL arg is OK)
 *
 ******************************************************************************/

pub unsafe fn acpi_ps_append_arg(
    op: *mut acpi_parse_object,
    mut arg: *mut acpi_parse_object,
) {
    let mut prev_arg: *mut acpi_parse_object;
    let op_info: *const acpi_opcode_info;

    // ACPI_FUNCTION_TRACE(ps_append_arg);

    if op.is_null() {
        return;
    }

    /* Get the info structure for this opcode */

    op_info = acpi_ps_get_opcode_info((*op).common.aml_opcode);
    if (*op_info).class_ == AML_CLASS_UNKNOWN {

        /* Invalid opcode */

        // ACPI_ERROR((AE_INFO, "Invalid AML Opcode: 0x%2.2X", (*op).common.aml_opcode));
        return;
    }

    /* Check if this opcode requires argument sub-objects */

    if (*op_info).flags & AML_HAS_ARGS == 0 {

        /* Has no linked argument objects */

        return;
    }

    /* Append the argument to the linked argument list */

    if !(*op).common.value.arg.is_null() {

        /* Append to existing argument list */

        prev_arg = (*op).common.value.arg;
        while !(*prev_arg).common.next.is_null() {
            prev_arg = (*prev_arg).common.next;
        }
        (*prev_arg).common.next = arg;
    } else {
        /* No argument list, this will be the first argument */

        (*op).common.value.arg = arg;
    }

    /* Set the parent in this arg and any args linked after it */

    while !arg.is_null() {
        (*arg).common.parent = op;
        arg = (*arg).common.next;
        (*op).common.arg_list_length += 1;
    }
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ps_get_depth_next
 *
 * PARAMETERS:  origin          - Root of subtree to search
 *              op              - Last (previous) Op that was found
 *
 * RETURN:      Next Op found in the search.
 *
 * DESCRIPTION: Get next op in tree (walking the tree in depth-first order)
 *              Return NULL when reaching "origin" or when walking up from root
 *
 ******************************************************************************/

pub unsafe fn acpi_ps_get_depth_next(
    origin: *mut acpi_parse_object,
    mut op: *mut acpi_parse_object,
) -> *mut acpi_parse_object {
    let mut next: *mut acpi_parse_object = core::ptr::null_mut();
    let mut parent: *mut acpi_parse_object;
    let mut arg: *mut acpi_parse_object;

    // ACPI_FUNCTION_ENTRY();

    if op.is_null() {
        return core::ptr::null_mut();
    }

    /* Look for an argument or child */

    next = acpi_ps_get_arg(op, 0);
    if !next.is_null() {
        // ASL_CV_LABEL_FILENODE(next);
        return next;
    }

    /* Look for a sibling */

    next = (*op).common.next;
    if !next.is_null() {
        // ASL_CV_LABEL_FILENODE(next);
        return next;
    }

    /* Look for a sibling of parent */

    parent = (*op).common.parent;

    while !parent.is_null() {
        arg = acpi_ps_get_arg(parent, 0);
        while !arg.is_null() && arg != origin && arg != op {
            // ASL_CV_LABEL_FILENODE(arg);
            arg = (*arg).common.next;
        }

        if arg == origin {
            /* Reached parent of origin, end search */
            return core::ptr::null_mut();
        }

        if !(*parent).common.next.is_null() {
            /* Found sibling of parent */
            // ASL_CV_LABEL_FILENODE((*parent).common.next);
            return (*parent).common.next;
        }

        op = parent;
        parent = (*parent).common.parent;
    }

    // ASL_CV_LABEL_FILENODE(next);
    next
}

#[cfg(feature = "ACPI_OBSOLETE_FUNCTIONS")]
pub unsafe fn acpi_ps_get_child(op: *mut acpi_parse_object) -> *mut acpi_parse_object {
    let mut child: *mut acpi_parse_object = core::ptr::null_mut();

    // ACPI_FUNCTION_ENTRY();

    match (*op).common.aml_opcode {
        AML_SCOPE_OP | AML_ELSE_OP | AML_DEVICE_OP | AML_THERMAL_ZONE_OP
        | AML_INT_METHODCALL_OP => {
            child = acpi_ps_get_arg(op, 0);
        }
        AML_BUFFER_OP | AML_PACKAGE_OP | AML_VARIABLE_PACKAGE_OP | AML_METHOD_OP
        | AML_IF_OP | AML_WHILE_OP | AML_FIELD_OP => {
            child = acpi_ps_get_arg(op, 1);
        }
        AML_POWER_RESOURCE_OP | AML_INDEX_FIELD_OP => {
            child = acpi_ps_get_arg(op, 2);
        }
        AML_PROCESSOR_OP | AML_BANK_FIELD_OP => {
            child = acpi_ps_get_arg(op, 3);
        }
        _ => {
            /* All others have no children */
        }
    }

    child
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
