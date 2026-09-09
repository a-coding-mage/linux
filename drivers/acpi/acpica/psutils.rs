// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: psutils - Parser miscellaneous utilities (Parser only)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation.

pub unsafe fn acpi_ps_create_scope_op(aml: *mut u8) -> *mut acpi_parse_object {
    let scope_op = acpi_ps_alloc_op(AML_SCOPE_OP, aml);
    if scope_op.is_null() {
        return core::ptr::null_mut();
    }

    (*scope_op).named.name = ACPI_ROOT_NAME;
    scope_op
}

pub unsafe fn acpi_ps_init_op(op: *mut acpi_parse_object, opcode: u16) {
    ACPI_FUNCTION_ENTRY!();

    (*op).common.descriptor_type = ACPI_DESC_TYPE_PARSER;
    (*op).common.aml_opcode = opcode;

    // ACPI_DISASM_ONLY_MEMBERS(acpi_ut_safe_strncpy(
    //     (*op).common.aml_op_name,
    //     acpi_ps_get_opcode_info(opcode).name,
    //     core::mem::size_of_val(&(*op).common.aml_op_name)));
}

pub unsafe fn acpi_ps_alloc_op(opcode: u16, aml: *mut u8) -> *mut acpi_parse_object {
    let mut op: *mut acpi_parse_object;
    let op_info: *const acpi_opcode_info;
    let mut flags: u8 = ACPI_PARSEOP_GENERIC;

    ACPI_FUNCTION_ENTRY!();

    op_info = acpi_ps_get_opcode_info(opcode);

    /* Determine type of parse_op required */
    if (*op_info).flags & AML_DEFER != 0 {
        flags = ACPI_PARSEOP_DEFERRED;
    } else if (*op_info).flags & AML_NAMED != 0 {
        flags = ACPI_PARSEOP_NAMED_OBJECT;
    } else if opcode == AML_INT_BYTELIST_OP {
        flags = ACPI_PARSEOP_BYTELIST;
    }

    /* Allocate the minimum required size object */
    if flags == ACPI_PARSEOP_GENERIC {
        /* The generic op (default) is by far the most common (16 to 1) */
        op = acpi_os_acquire_object(acpi_gbl_ps_node_cache);
    } else {
        /* Extended parseop */
        op = acpi_os_acquire_object(acpi_gbl_ps_node_ext_cache);
    }

    /* Initialize the Op */
    if !op.is_null() {
        acpi_ps_init_op(op, opcode);
        (*op).common.aml = aml;
        (*op).common.flags = flags;
        ASL_CV_CLEAR_OP_COMMENTS!(op);

        if opcode == AML_SCOPE_OP {
            acpi_gbl_current_scope = op;
        }

        if acpi_gbl_capture_comments {
            ASL_CV_TRANSFER_COMMENTS!(op);
        }
    }

    op
}

pub unsafe fn acpi_ps_free_op(op: *mut acpi_parse_object) {
    ACPI_FUNCTION_NAME!(ps_free_op);

    ASL_CV_CLEAR_OP_COMMENTS!(op);
    if (*op).common.aml_opcode == AML_INT_RETURN_VALUE_OP {
        ACPI_DEBUG_PRINT!((ACPI_DB_ALLOCATIONS, "Free retval op: %p\n", op));
    }

    if (*op).common.flags & ACPI_PARSEOP_GENERIC != 0 {
        let _ = acpi_os_release_object(acpi_gbl_ps_node_cache, op);
    } else {
        let _ = acpi_os_release_object(acpi_gbl_ps_node_ext_cache, op);
    }
}

/* Is "c" a namestring lead character? */
pub fn acpi_ps_is_leading_char(c: u32) -> u8 {
    (c == b'_' as u32 || (c >= b'A' as u32 && c <= b'Z' as u32)) as u8
}

/* Get op's name (4-byte name segment) or 0 if unnamed */
pub unsafe fn acpi_ps_get_name(op: *mut acpi_parse_object) -> u32 {
    /* The "generic" object has no name associated with it */
    if (*op).common.flags & ACPI_PARSEOP_GENERIC != 0 {
        return 0;
    }

    /* Only the "Extended" parse objects have a name */
    (*op).named.name
}

/* Set op's name */
pub unsafe fn acpi_ps_set_name(op: *mut acpi_parse_object, name: u32) {
    /* The "generic" object has no name associated with it */
    if (*op).common.flags & ACPI_PARSEOP_GENERIC != 0 {
        return;
    }

    (*op).named.name = name;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
