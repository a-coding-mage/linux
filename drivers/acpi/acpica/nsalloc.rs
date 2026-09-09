// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: nsalloc - Namespace allocation and deletion utilities
 *
 ******************************************************************************/

// C dependencies: <acpi/acpi.h>, "accommon.h", and "acnamesp.h".
// Build-time ACPI tracing, debugging, and memory-tracking macros are retained
// below as comments where they have no direct Rust equivalent.

/* #define _COMPONENT ACPI_NAMESPACE */
/* ACPI_MODULE_NAME("nsalloc") */

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_create_node
 *
 * PARAMETERS:  name            - Name of the new node (4 char ACPI name)
 *
 * RETURN:      New namespace node (Null on failure)
 *
 * DESCRIPTION: Create a namespace node
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_create_node(name: u32) -> *mut acpi_namespace_node {
    // ACPI_FUNCTION_TRACE(ns_create_node);
    let node = acpi_os_acquire_object(acpi_gbl_namespace_cache);
    if node.is_null() {
        return core::ptr::null_mut();
    }

    // ACPI_MEM_TRACKING(acpi_gbl_ns_node_list->total_allocated++);
    // #ifdef ACPI_DBG_TRACK_ALLOCATIONS
    // let temp = acpi_gbl_ns_node_list->total_allocated -
    //     acpi_gbl_ns_node_list->total_freed;
    // if temp > acpi_gbl_ns_node_list->max_occupied {
    //     acpi_gbl_ns_node_list->max_occupied = temp;
    // }
    // #endif

    (*node).name.integer = name;
    // ACPI_SET_DESCRIPTOR_TYPE(node, ACPI_DESC_TYPE_NAMED);
    (*node).descriptor_type = ACPI_DESC_TYPE_NAMED;
    node
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_delete_node
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_delete_node(node: *mut acpi_namespace_node) {
    if node.is_null() {
        return;
    }

    acpi_ns_detach_object(node);

    let mut obj_desc = (*node).object;
    while !obj_desc.is_null() && (*obj_desc).common.type_ == ACPI_TYPE_LOCAL_DATA {
        if let Some(handler) = (*obj_desc).data.handler {
            handler(node, (*obj_desc).data.pointer);
        }

        let next_desc = (*obj_desc).common.next_object;
        acpi_ut_remove_reference(obj_desc);
        obj_desc = next_desc;
    }

    if node == acpi_gbl_root_node {
        return;
    }

    let _ = acpi_os_release_object(acpi_gbl_namespace_cache, node);
    // ACPI_MEM_TRACKING(acpi_gbl_ns_node_list->total_freed++);
    // ACPI_DEBUG_PRINT((ACPI_DB_ALLOCATIONS, "Node %p, Remaining %X\n",
    //                   node, acpi_gbl_current_node_count));
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_remove_node
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_remove_node(node: *mut acpi_namespace_node) {
    // ACPI_FUNCTION_TRACE_PTR(ns_remove_node, node);
    let parent_node = (*node).parent;
    let mut prev_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut next_node = (*parent_node).child;

    while next_node != node {
        prev_node = next_node;
        next_node = (*next_node).peer;
    }

    if !prev_node.is_null() {
        (*prev_node).peer = (*node).peer;
    } else {
        (*parent_node).child = (*node).peer;
    }

    acpi_ns_delete_node(node);
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_install_node
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_install_node(
    walk_state: *mut acpi_walk_state,
    parent_node: *mut acpi_namespace_node,
    node: *mut acpi_namespace_node,
    type_: acpi_object_type,
) {
    // ACPI_FUNCTION_TRACE(ns_install_node);
    let mut owner_id: acpi_owner_id = 0;

    if !walk_state.is_null() {
        owner_id = (*walk_state).owner_id;
        if !(*walk_state).method_desc.is_null()
            && parent_node != (*walk_state).method_node
        {
            (*(*walk_state).method_desc).method.info_flags |= ACPI_METHOD_MODIFIED_NAMESPACE;
        }
    }

    (*node).peer = core::ptr::null_mut();
    (*node).parent = parent_node;
    let mut child_node = (*parent_node).child;
    if child_node.is_null() {
        (*parent_node).child = node;
    } else {
        while !(*child_node).peer.is_null() {
            child_node = (*child_node).peer;
        }
        (*child_node).peer = node;
    }

    (*node).owner_id = owner_id;
    (*node).type_ = type_ as u8;
    // ACPI_DEBUG_PRINT((ACPI_DB_NAMES, ...));
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_delete_children
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_delete_children(parent_node: *mut acpi_namespace_node) {
    // ACPI_FUNCTION_TRACE_PTR(ns_delete_children, parent_node);
    if parent_node.is_null() {
        return;
    }

    let mut next_node = (*parent_node).child;
    while !next_node.is_null() {
        if !(*next_node).child.is_null() {
            // ACPI_ERROR((AE_INFO, "Found a grandchild! P=%p C=%p", parent_node, next_node));
        }
        let node_to_delete = next_node;
        next_node = (*next_node).peer;
        acpi_ns_delete_node(node_to_delete);
    }

    (*parent_node).child = core::ptr::null_mut();
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_delete_namespace_subtree
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_delete_namespace_subtree(parent_node: *mut acpi_namespace_node) {
    let mut child_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut level: u32 = 1;
    // ACPI_FUNCTION_TRACE(ns_delete_namespace_subtree);
    if parent_node.is_null() {
        return;
    }

    let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return;
    }

    let mut current_parent = parent_node;
    while level > 0 {
        child_node = acpi_ns_get_next_node(current_parent, child_node);
        if !child_node.is_null() {
            acpi_ns_detach_object(child_node);
            if !(*child_node).child.is_null() {
                level += 1;
                current_parent = child_node;
                child_node = core::ptr::null_mut();
            }
        } else {
            level -= 1;
            acpi_ns_delete_children(current_parent);
            child_node = current_parent;
            current_parent = (*current_parent).parent;
        }
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ns_delete_namespace_by_owner
 *
 ******************************************************************************/
pub unsafe fn acpi_ns_delete_namespace_by_owner(owner_id: acpi_owner_id) {
    // ACPI_FUNCTION_TRACE_U32(ns_delete_namespace_by_owner, owner_id);
    if owner_id == 0 {
        return;
    }

    let status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return;
    }

    let mut deletion_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut parent_node = acpi_gbl_root_node;
    let mut child_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut level: u32 = 1;

    while level > 0 {
        child_node = acpi_ns_get_next_node(parent_node, child_node);

        if !deletion_node.is_null() {
            acpi_ns_delete_children(deletion_node);
            acpi_ns_remove_node(deletion_node);
            deletion_node = core::ptr::null_mut();
        }

        if !child_node.is_null() {
            if (*child_node).owner_id == owner_id {
                acpi_ns_detach_object(child_node);
            }
            if !(*child_node).child.is_null() {
                level += 1;
                parent_node = child_node;
                child_node = core::ptr::null_mut();
            } else if (*child_node).owner_id == owner_id {
                deletion_node = child_node;
            }
        } else {
            level -= 1;
            if level != 0 && (*parent_node).owner_id == owner_id {
                deletion_node = parent_node;
            }
            child_node = parent_node;
            parent_node = (*parent_node).parent;
        }
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
