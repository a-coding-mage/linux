// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: nssearch - Namespace search
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI implementation are intentionally external.

/* Local prototypes */
unsafe fn acpi_ns_search_parent_tree(
    target_name: u32,
    node: *mut acpi_namespace_node,
    ty: acpi_object_type,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_search_one_scope
 *
 ******************************************************************************/

pub unsafe fn acpi_ns_search_one_scope(
    target_name: u32,
    parent_node: *mut acpi_namespace_node,
    ty: acpi_object_type,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    // Search for name at this namespace level, among the children of this object.
    let mut node = (*parent_node).child;
    while !node.is_null() {
        if (*node).name.integer == target_name {
            // Resolve a control method alias if any.
            if acpi_ns_get_type(node) == ACPI_TYPE_LOCAL_METHOD_ALIAS {
                node = (*node).object as *mut acpi_namespace_node;
            }

            *return_node = node;
            return AE_OK;
        }

        // Didn't match name, move on to the next peer object.
        node = (*node).peer;
    }

    AE_NOT_FOUND
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_search_parent_tree
 *
 ******************************************************************************/

unsafe fn acpi_ns_search_parent_tree(
    target_name: u32,
    node: *mut acpi_namespace_node,
    ty: acpi_object_type,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    let mut parent_node = (*node).parent;

    // If there is no parent (we are at the root), we won't search the parent tree.
    if parent_node.is_null() {
        return AE_NOT_FOUND;
    }

    if acpi_ns_local(ty) {
        return AE_NOT_FOUND;
    }

    // Search parents until target is found or we have backed up to the root.
    while !parent_node.is_null() {
        let status = acpi_ns_search_one_scope(
            target_name,
            parent_node,
            ACPI_TYPE_ANY,
            return_node,
        );
        if ACPI_SUCCESS(status) {
            return status;
        }
        parent_node = (*parent_node).parent;
    }

    AE_NOT_FOUND
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_search_and_enter
 *
 ******************************************************************************/

pub unsafe fn acpi_ns_search_and_enter(
    target_name: u32,
    walk_state: *mut acpi_walk_state,
    node: *mut acpi_namespace_node,
    interpreter_mode: acpi_interpreter_mode,
    ty: acpi_object_type,
    flags: u32,
    return_node: *mut *mut acpi_namespace_node,
) -> acpi_status {
    if node.is_null() || target_name == 0 || return_node.is_null() {
        return AE_BAD_PARAMETER;
    }

    let mut target_name = target_name;
    acpi_ut_repair_name((&mut target_name as *mut u32).cast::<i8>());

    *return_node = ACPI_ENTRY_NOT_FOUND;
    let mut status = acpi_ns_search_one_scope(target_name, node, ty, return_node);
    if status != AE_NOT_FOUND {
        if status == AE_OK {
            if flags & ACPI_NS_OVERRIDE_IF_FOUND != 0 {
                acpi_ns_delete_children(*return_node);
                if acpi_gbl_runtime_namespace_override {
                    acpi_ut_remove_reference((**return_node).object);
                    (**return_node).object = core::ptr::null_mut();
                    (**return_node).owner_id = (*walk_state).owner_id;
                } else {
                    acpi_ns_remove_node(*return_node);
                    *return_node = ACPI_ENTRY_NOT_FOUND;
                }
            } else if flags & ACPI_NS_ERROR_IF_FOUND != 0 {
                status = AE_ALREADY_EXISTS;
            }
        }

        #[cfg(feature = "acpi_asl_compiler")]
        if !(*return_node).is_null() && (**return_node).r#type == ACPI_TYPE_ANY {
            (**return_node).flags |= ANOBJ_IS_EXTERNAL;
        }

        return status;
    }

    if interpreter_mode != ACPI_IMODE_LOAD_PASS1
        && flags & ACPI_NS_SEARCH_PARENT != 0
    {
        status = acpi_ns_search_parent_tree(target_name, node, ty, return_node);
        if ACPI_SUCCESS(status) {
            return status;
        }
    }

    // In execute mode, just search, never add names.
    if interpreter_mode == ACPI_IMODE_EXECUTE {
        return AE_NOT_FOUND;
    }

    let new_node = acpi_ns_create_node(target_name);
    if new_node.is_null() {
        return AE_NO_MEMORY;
    }

    #[cfg(feature = "acpi_asl_compiler")]
    if flags & ACPI_NS_EXTERNAL != 0
        || (!walk_state.is_null() && (*walk_state).opcode == AML_SCOPE_OP)
    {
        (*new_node).flags |= ANOBJ_IS_EXTERNAL;
    }

    if flags & ACPI_NS_TEMPORARY != 0 {
        (*new_node).flags |= ANOBJ_TEMPORARY;
    }

    acpi_ns_install_node(walk_state, node, new_node, ty);
    *return_node = new_node;
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
