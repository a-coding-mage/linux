// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: pswalk - Parser routines to walk parsed op tree(s)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// External ACPI declarations and build-time configuration are supplied by the
// surrounding translation unit.

pub const _COMPONENT: u32 = ACPI_PARSER;

extern "C" {
    fn acpi_ps_get_opcode_name(opcode: u16) -> *const core::ffi::c_char;
    fn acpi_os_printf(format: *const core::ffi::c_char, ...);
    fn acpi_ps_get_arg(op: *mut AcpiParseObject, arg: u32) -> *mut AcpiParseObject;
    fn acpi_ps_free_op(op: *mut AcpiParseObject);
}

// These types and constants are provided by acpi/acpi.h, accommon.h,
// acparser.h, and amlcode.h in the complete translation.
#[repr(C)]
pub union AcpiParseObject {
    pub common: AcpiParseObjectCommon,
    pub named: AcpiParseObjectNamed,
}

#[repr(C)]
pub struct AcpiParseObjectCommon {
    pub aml_opcode: u16,
    pub value: AcpiParseValue,
    pub next: *mut AcpiParseObject,
    pub parent: *mut AcpiParseObject,
}

#[repr(C)]
pub struct AcpiParseObjectNamed {
    pub aml_opcode: u16,
    pub value: AcpiParseValue,
}

#[repr(C)]
pub union AcpiParseValue {
    pub string: *const core::ffi::c_char,
}

pub const ACPI_PARSER: u32 = 0;
pub const ACPI_LV_PARSE_TREES: u32 = 0;
pub const ACPI_DB_PARSE_TREES: u32 = 0;
pub const AML_INT_NAMEPATH_OP: u16 = 0;
pub const AML_STRING_OP: u16 = 0;

/// Delete a portion of or an entire parse tree.
pub unsafe extern "C" fn acpi_ps_delete_parse_tree(
    subtree_root: *mut AcpiParseObject,
) {
    let mut op = subtree_root;
    let mut next: *mut AcpiParseObject = core::ptr::null_mut();
    let mut parent: *mut AcpiParseObject = core::ptr::null_mut();
    let mut level: u32 = 0;

    // ACPI_FUNCTION_TRACE_PTR(ps_delete_parse_tree, subtree_root);
    // ACPI_DEBUG_PRINT((ACPI_DB_PARSE_TREES, " root %p\n", subtree_root));

    /* Visit all nodes in the subtree */
    while !op.is_null() {
        if op != parent {
            /* This is the descending case */
            // ACPI_IS_DEBUG_ENABLED(ACPI_LV_PARSE_TREES, _COMPONENT) controls
            // this diagnostic tree dump in the source build.
            if false {
                /* This debug option will print the entire parse tree */
                acpi_os_printf(
                    b"      %*s%s %p\0".as_ptr() as *const core::ffi::c_char,
                );
                let opcode_name = acpi_ps_get_opcode_name((*op).common.aml_opcode);
                let _ = opcode_name;
                if (*op).named.aml_opcode == AML_INT_NAMEPATH_OP {
                    acpi_os_printf((*op).common.value.string);
                }
                if (*op).named.aml_opcode == AML_STRING_OP {
                    acpi_os_printf((*op).common.value.string);
                }
                acpi_os_printf(b"\n\0".as_ptr() as *const core::ffi::c_char);
            }

            /* Look for an argument or child of the current op */
            next = acpi_ps_get_arg(op, 0);
            if !next.is_null() {
                /* Still going downward in tree (Op is not completed yet) */
                op = next;
                level = level.wrapping_add(1);
                continue;
            }
        }

        /* No more children, this Op is complete. */
        next = (*op).common.next;
        parent = (*op).common.parent;
        acpi_ps_free_op(op);

        /* If we are back to the starting point, the walk is complete. */
        if op == subtree_root {
            return;
        }
        if !next.is_null() {
            op = next;
        } else {
            level = level.wrapping_sub(1);
            op = parent;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
