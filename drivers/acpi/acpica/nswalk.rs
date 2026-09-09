// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: nswalk - Functions for walking the ACPI namespace
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// C dependencies: acpi/acpi.h, accommon.h, and acnamesp.h.

/* _COMPONENT ACPI_NAMESPACE; ACPI_MODULE_NAME("nswalk") */

pub unsafe fn acpi_ns_get_next_node(
    parent_node: *mut acpi_namespace_node,
    child_node: *mut acpi_namespace_node,
) -> *mut acpi_namespace_node {
    if child_node.is_null() {
        /* It's really the parent's _scope_ that we want */
        return (*parent_node).child;
    }

    /* Otherwise just return the next peer */
    (*child_node).peer
}

pub unsafe fn acpi_ns_get_next_node_typed(
    type_: acpi_object_type,
    parent_node: *mut acpi_namespace_node,
    child_node: *mut acpi_namespace_node,
) -> *mut acpi_namespace_node {
    let mut next_node = acpi_ns_get_next_node(parent_node, child_node);

    /* If any type is OK, we are done */
    if type_ == ACPI_TYPE_ANY {
        /* next_node is NULL if we are at the end-of-list */
        return next_node;
    }

    /* Must search for the node -- but within this scope only */
    while !next_node.is_null() {
        /* If type matches, we are done */
        if (*next_node).type_ == type_ {
            return next_node;
        }

        /* Otherwise, move on to the next peer node */
        next_node = (*next_node).peer;
    }

    /* Not found */
    core::ptr::null_mut()
}

pub unsafe fn acpi_ns_walk_namespace(
    type_: acpi_object_type,
    mut start_node: acpi_handle,
    max_depth: u32,
    flags: u32,
    descending_callback: acpi_walk_callback,
    ascending_callback: acpi_walk_callback,
    context: *mut core::ffi::c_void,
    return_value: *mut *mut core::ffi::c_void,
) -> acpi_status {
    let mut status: acpi_status;
    let mut mutex_status: acpi_status;
    let mut child_node: *mut acpi_namespace_node;
    let mut parent_node: *mut acpi_namespace_node;
    let mut child_type: acpi_object_type;
    let mut level: u32;
    let mut node_previously_visited: u8 = FALSE;

    /* Special case for the namespace Root Node */
    if start_node == ACPI_ROOT_OBJECT {
        start_node = acpi_gbl_root_node;
    }

    /* Avoid walking the namespace if the StartNode is NULL */
    if start_node.is_null() {
        return AE_NO_NAMESPACE;
    }

    /* Null child means "get first node" */
    parent_node = start_node;
    child_node = acpi_ns_get_next_node(parent_node, core::ptr::null_mut());
    child_type = ACPI_TYPE_ANY;
    level = 1;

    /* Traverse the tree until we bubble back up to where we started. */
    while level > 0 && !child_node.is_null() {
        status = AE_OK;

        /* Found next child, get the type if we are not searching for ANY */
        if type_ != ACPI_TYPE_ANY {
            child_type = (*child_node).type_;
        }

        /* Ignore temporary namespace nodes unless told otherwise. */
        if ((*child_node).flags & ANOBJ_TEMPORARY) != 0
            && (flags & ACPI_NS_WALK_TEMP_NODES) == 0
        {
            status = AE_CTRL_DEPTH;
        } else if child_type == type_ {
            /* Found a matching node, invoke the user callback function. */
            if (flags & ACPI_NS_WALK_UNLOCK) != 0 {
                mutex_status = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
                if ACPI_FAILURE(mutex_status) {
                    return mutex_status;
                }
            }

            if node_previously_visited == FALSE {
                if let Some(callback) = descending_callback {
                    status = callback(child_node, level, context, return_value);
                }
            } else if let Some(callback) = ascending_callback {
                status = callback(child_node, level, context, return_value);
            }

            if (flags & ACPI_NS_WALK_UNLOCK) != 0 {
                mutex_status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
                if ACPI_FAILURE(mutex_status) {
                    return mutex_status;
                }
            }

            match status {
                AE_OK | AE_CTRL_DEPTH => {}
                AE_CTRL_TERMINATE => return AE_OK,
                _ => return status,
            }
        }

        /* Depth first search: attempt to go down another level. */
        if node_previously_visited == FALSE
            && level < max_depth
            && status != AE_CTRL_DEPTH
            && !(*child_node).child.is_null()
        {
            level += 1;
            parent_node = child_node;
            child_node = acpi_ns_get_next_node(parent_node, core::ptr::null_mut());
            continue;
        }

        /* No more children, re-visit this node */
        if node_previously_visited == FALSE {
            node_previously_visited = TRUE;
            continue;
        }

        /* No more children, visit peers */
        child_node = acpi_ns_get_next_node(parent_node, child_node);
        if !child_node.is_null() {
            node_previously_visited = FALSE;
        } else {
            /* No peers, re-visit parent */
            level -= 1;
            child_node = parent_node;
            parent_node = (*parent_node).parent;
            node_previously_visited = TRUE;
        }
    }

    /* Complete walk, not terminated by user function */
    AE_OK
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
