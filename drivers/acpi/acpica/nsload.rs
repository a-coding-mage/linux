// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nsload - namespace loading/expanding/contracting procedures
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally not
// reproduced here.

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_load_table
 *
 * PARAMETERS:  table_index     - Index for table to be loaded
 *              node            - Owning NS node
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Load one ACPI table into the namespace
 *
 ******************************************************************************/

pub unsafe fn acpi_ns_load_table(
    table_index: u32,
    node: *mut acpi_namespace_node,
) -> acpi_status {
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(ns_load_table);

    /* If table already loaded into namespace, just return */
    if acpi_tb_is_table_loaded(table_index) {
        status = AE_ALREADY_EXISTS;
    } else {
        // ACPI_DEBUG_PRINT((ACPI_DB_INFO, "**** Loading table into namespace ****\n"));

        status = acpi_tb_allocate_owner_id(table_index);
        if ACPI_FAILURE(status) {
            return status;
        }

        /*
     * Parse the table and load the namespace with all named
     * objects found within. Control methods are NOT parsed
     * at this time. In fact, the control methods cannot be
     * parsed until the entire namespace is loaded, because
     * if a control method makes a forward reference (call)
     * to another control method, we can't continue parsing
     * because we don't know how many arguments to parse next!
         */
        status = acpi_ns_parse_table(table_index, node);
        if ACPI_SUCCESS(status) {
            acpi_tb_set_table_loaded_flag(table_index, TRUE);
        } else {
        /* On error, delete any namespace objects created by this table. */
            acpi_ns_delete_namespace_by_owner(
                acpi_gbl_root_table_list.tables[table_index as usize].owner_id,
            );
            acpi_tb_release_owner_id(table_index);
            return status;
        }
    }

    if ACPI_FAILURE(status) {
        return status;
    }

    /* Parse and initialize the control methods. */
    // ACPI_DEBUG_PRINT((ACPI_DB_INFO, "**** Begin Table Object Initialization\n"));
    acpi_ex_enter_interpreter();
    status = acpi_ds_initialize_objects(table_index, node);
    acpi_ex_exit_interpreter();
    // ACPI_DEBUG_PRINT((ACPI_DB_INFO, "**** Completed Table Object Initialization\n"));

    status
}

#[cfg(feature = "acpi_obsolete_functions")]
pub unsafe fn acpi_ns_load_namespace() -> acpi_status {
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(acpi_load_name_space);

    /* There must be at least a DSDT installed */
    if acpi_gbl_DSDT.is_null() {
        // ACPI_ERROR((AE_INFO, "DSDT is not in memory"));
        return AE_NO_ACPI_TABLES;
    }

    /* Load the namespace. The DSDT is required, but SSDT and PSDT are optional. */
    status = acpi_ns_load_table_by_type(ACPI_TABLE_ID_DSDT);
    if ACPI_FAILURE(status) {
        return status;
    }

    let _ = acpi_ns_load_table_by_type(ACPI_TABLE_ID_SSDT);
    let _ = acpi_ns_load_table_by_type(ACPI_TABLE_ID_PSDT);
    // ACPI_DEBUG_PRINT_RAW((ACPI_DB_INIT, "ACPI Namespace successfully loaded at root %p\n", acpi_gbl_root_node));
    status
}

#[cfg(feature = "acpi_future_implementation")]
unsafe fn acpi_ns_delete_subtree(start_handle: acpi_handle) -> acpi_status {
    let mut status: acpi_status;
    let mut child_handle: acpi_handle = core::ptr::null_mut();
    let mut parent_handle = start_handle;
    let mut next_child_handle: acpi_handle;
    let mut dummy: acpi_handle;
    let mut level: u32 = 1;

    // ACPI_FUNCTION_TRACE(ns_delete_subtree);

    while level > 0 {
        status = acpi_get_next_object(
            ACPI_TYPE_ANY,
            parent_handle,
            child_handle,
            &mut next_child_handle,
        );
        child_handle = next_child_handle;

        if ACPI_SUCCESS(status) {
            if ACPI_SUCCESS(acpi_get_next_object(
                ACPI_TYPE_ANY,
                child_handle,
                core::ptr::null_mut(),
                &mut dummy,
            )) {
                level += 1;
                parent_handle = child_handle;
                child_handle = core::ptr::null_mut();
            }
        } else {
            level -= 1;
            acpi_ns_delete_children(child_handle);
            child_handle = parent_handle;
            status = acpi_get_parent(parent_handle, &mut parent_handle);
            if ACPI_FAILURE(status) {
                return status;
            }
        }
    }

    acpi_ns_remove_node(child_handle);
    AE_OK
}

#[cfg(feature = "acpi_future_implementation")]
pub unsafe fn acpi_ns_unload_namespace(handle: acpi_handle) -> acpi_status {
    // ACPI_FUNCTION_TRACE(ns_unload_name_space);
    if acpi_gbl_root_node.is_null() {
        return AE_NO_NAMESPACE;
    }
    if handle.is_null() {
        return AE_BAD_PARAMETER;
    }
    acpi_ns_delete_subtree(handle)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
