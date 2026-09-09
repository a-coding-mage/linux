// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exresnte - AML Interpreter object resolution
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPICA Rust translation environment.

pub unsafe fn acpi_ex_resolve_node_to_value(
    object_ptr: *mut *mut acpi_namespace_node,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut source_desc: *mut acpi_operand_object;
    let mut obj_desc: *mut acpi_operand_object = core::ptr::null_mut();
    let mut node: *mut acpi_namespace_node;
    let mut entry_type: acpi_object_type;

    // ACPI_FUNCTION_TRACE(ex_resolve_node_to_value);

    /*
     * The stack pointer points to a struct acpi_namespace_node (Node). Get the
     * object that is attached to the Node.
     */
    node = *object_ptr;
    source_desc = acpi_ns_get_attached_object(node);
    entry_type = acpi_ns_get_type(node as acpi_handle);

    // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "Entry=%p SourceDesc=%p [%s]\n",
    //                   node, source_desc, acpi_ut_get_type_name(entry_type)));

    if entry_type == ACPI_TYPE_LOCAL_ALIAS || entry_type == ACPI_TYPE_LOCAL_METHOD_ALIAS {
        /* There is always exactly one level of indirection */
        node = (*node).object as *mut acpi_namespace_node;
        source_desc = acpi_ns_get_attached_object(node);
        entry_type = acpi_ns_get_type(node as acpi_handle);
        *object_ptr = node;
    }

    /*
     * Several object types require no further processing:
     * 1) Device/Thermal objects don't have a "real" subobject, return Node
     * 2) Method locals and arguments have a pseudo-Node
     * 3) 10/2007: Added method type to assist with Package construction.
     */
    if entry_type == ACPI_TYPE_DEVICE
        || entry_type == ACPI_TYPE_THERMAL
        || entry_type == ACPI_TYPE_METHOD
        || ((*node).flags & (ANOBJ_METHOD_ARG | ANOBJ_METHOD_LOCAL)) != 0
    {
        return AE_OK;
    }

    if source_desc.is_null() {
        // ACPI_ERROR((AE_INFO, "No object attached to node [%4.4s] %p",
        //              (*node).name.ascii, node));
        return AE_AML_UNINITIALIZED_NODE;
    }

    /* Action is based on the type of the Node. */
    match entry_type {
        ACPI_TYPE_PACKAGE => {
            if (*source_desc).common.type_ != ACPI_TYPE_PACKAGE {
                // ACPI_ERROR((AE_INFO, "Object not a Package, type %s", ...));
                return AE_AML_OPERAND_TYPE;
            }
            status = acpi_ds_get_package_arguments(source_desc);
            if ACPI_SUCCESS(status) {
                obj_desc = source_desc;
                acpi_ut_add_reference(obj_desc);
            }
        }
        ACPI_TYPE_BUFFER => {
            if (*source_desc).common.type_ != ACPI_TYPE_BUFFER {
                return AE_AML_OPERAND_TYPE;
            }
            status = acpi_ds_get_buffer_arguments(source_desc);
            if ACPI_SUCCESS(status) {
                obj_desc = source_desc;
                acpi_ut_add_reference(obj_desc);
            }
        }
        ACPI_TYPE_STRING => {
            if (*source_desc).common.type_ != ACPI_TYPE_STRING {
                return AE_AML_OPERAND_TYPE;
            }
            obj_desc = source_desc;
            acpi_ut_add_reference(obj_desc);
        }
        ACPI_TYPE_INTEGER => {
            if (*source_desc).common.type_ != ACPI_TYPE_INTEGER {
                return AE_AML_OPERAND_TYPE;
            }
            obj_desc = source_desc;
            acpi_ut_add_reference(obj_desc);
        }
        ACPI_TYPE_BUFFER_FIELD
        | ACPI_TYPE_LOCAL_REGION_FIELD
        | ACPI_TYPE_LOCAL_BANK_FIELD
        | ACPI_TYPE_LOCAL_INDEX_FIELD => {
            status = acpi_ex_read_data_from_field(walk_state, source_desc, &mut obj_desc);
        }
        ACPI_TYPE_MUTEX
        | ACPI_TYPE_POWER
        | ACPI_TYPE_PROCESSOR
        | ACPI_TYPE_EVENT
        | ACPI_TYPE_REGION => {
            obj_desc = source_desc;
            acpi_ut_add_reference(obj_desc);
        }
        ACPI_TYPE_ANY => {
            return AE_AML_OPERAND_TYPE; // Cannot be AE_TYPE
        }
        ACPI_TYPE_LOCAL_REFERENCE => {
            match (*source_desc).reference.class_ {
                ACPI_REFCLASS_TABLE | ACPI_REFCLASS_REFOF | ACPI_REFCLASS_INDEX => {
                    obj_desc = source_desc;
                    acpi_ut_add_reference(obj_desc);
                }
                _ => return AE_AML_OPERAND_TYPE,
            }
        }
        _ => return AE_AML_OPERAND_TYPE,
    }

    /* Return the object descriptor */
    *object_ptr = obj_desc as *mut acpi_namespace_node;
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
