// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: nsxfobj - Public interfaces to the ACPI subsystem
 *                         ACPI Object oriented interfaces
 *
 ******************************************************************************/

// EXPORT_ACPI_INTERFACES
// Dependencies are provided by the surrounding ACPI translation unit.

// #define _COMPONENT ACPI_NAMESPACE
// ACPI_MODULE_NAME("nsxfobj")

/*******************************************************************************
 *
 * FUNCTION:    acpi_get_type
 *
 ******************************************************************************/
pub unsafe extern "C" fn acpi_get_type(
    handle: acpi_handle,
    ret_type: *mut acpi_object_type,
) -> acpi_status {
    let node: *mut acpi_namespace_node;
    let mut status: acpi_status;

    /* Parameter Validation */
    if ret_type.is_null() {
        return AE_BAD_PARAMETER;
    }

    /* Special case for the predefined Root Node (return type ANY) */
    if handle == ACPI_ROOT_OBJECT {
        *ret_type = ACPI_TYPE_ANY;
        return AE_OK;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Convert and validate the handle */
    node = acpi_ns_validate_handle(handle);
    if node.is_null() {
        let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
        return AE_BAD_PARAMETER;
    }

    *ret_type = (*node).type_;

    status = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

// ACPI_EXPORT_SYMBOL(acpi_get_type)

/*******************************************************************************
 *
 * FUNCTION:    acpi_get_parent
 *
 ******************************************************************************/
pub unsafe extern "C" fn acpi_get_parent(
    handle: acpi_handle,
    ret_handle: *mut acpi_handle,
) -> acpi_status {
    let node: *mut acpi_namespace_node;
    let parent_node: *mut acpi_namespace_node;
    let mut status: acpi_status;

    if ret_handle.is_null() {
        return AE_BAD_PARAMETER;
    }

    /* Special case for the predefined Root Node (no parent) */
    if handle == ACPI_ROOT_OBJECT {
        return AE_NULL_ENTRY;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Convert and validate the handle */
    node = acpi_ns_validate_handle(handle);
    if node.is_null() {
        status = AE_BAD_PARAMETER;
    } else {
        /* Get the parent entry */
        parent_node = (*node).parent;
        *ret_handle = parent_node as acpi_handle;

        /* Return exception if parent is null */
        if parent_node.is_null() {
            status = AE_NULL_ENTRY;
        }
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

// ACPI_EXPORT_SYMBOL(acpi_get_parent)

/*******************************************************************************
 *
 * FUNCTION:    acpi_get_next_object
 *
 ******************************************************************************/
pub unsafe extern "C" fn acpi_get_next_object(
    type_: acpi_object_type,
    parent: acpi_handle,
    child: acpi_handle,
    ret_handle: *mut acpi_handle,
) -> acpi_status {
    let mut status: acpi_status;
    let node: *mut acpi_namespace_node;
    let mut parent_node: *mut acpi_namespace_node = core::ptr::null_mut();
    let mut child_node: *mut acpi_namespace_node = core::ptr::null_mut();

    /* Parameter validation */
    if type_ > ACPI_TYPE_EXTERNAL_MAX {
        return AE_BAD_PARAMETER;
    }

    status = acpi_ut_acquire_mutex(ACPI_MTX_NAMESPACE);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* If null handle, use the parent */
    if child.is_null() {
        /* Start search at the beginning of the specified scope */
        parent_node = acpi_ns_validate_handle(parent);
        if parent_node.is_null() {
            status = AE_BAD_PARAMETER;
            let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
            return status;
        }
    } else {
        /* Non-null handle, ignore the parent */
        /* Convert and validate the handle */
        child_node = acpi_ns_validate_handle(child);
        if child_node.is_null() {
            status = AE_BAD_PARAMETER;
            let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
            return status;
        }
    }

    /* Internal function does the real work */
    node = acpi_ns_get_next_node_typed(type_, parent_node, child_node);
    if node.is_null() {
        status = AE_NOT_FOUND;
    } else {
        if !ret_handle.is_null() {
            *ret_handle = node as acpi_handle;
        }
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_NAMESPACE);
    status
}

// ACPI_EXPORT_SYMBOL(acpi_get_next_object)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
