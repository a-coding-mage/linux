// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utxfmutex - external AML mutex access functions
 *
 ******************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation unit.

// #define _COMPONENT ACPI_UTILITIES
// ACPI_MODULE_NAME("utxfmutex")

/* Local prototypes */
unsafe fn acpi_ut_get_mutex_object(
    handle: acpi_handle,
    pathname: acpi_string,
    ret_obj: *mut *mut acpi_operand_object,
) -> acpi_status;

/*******************************************************************************
 *
 * FUNCTION:    acpi_ut_get_mutex_object
 *
 * PARAMETERS:  handle              - Mutex or prefix handle (optional)
 *              pathname            - Mutex pathname (optional)
 *              ret_obj             - Where the mutex object is returned
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Get an AML mutex object. The mutex node is pointed to by
 *              Handle:Pathname. Either Handle or Pathname can be NULL, but
 *              not both.
 *
 ******************************************************************************/

unsafe fn acpi_ut_get_mutex_object(
    handle: acpi_handle,
    pathname: acpi_string,
    ret_obj: *mut *mut acpi_operand_object,
) -> acpi_status {
    let mut mutex_node: *mut acpi_namespace_node;
    let mutex_obj: *mut acpi_operand_object;
    let mut status: acpi_status;

    /* Parameter validation */

    if ret_obj.is_null() || (handle.is_null() && pathname.is_null()) {
        return AE_BAD_PARAMETER;
    }

    /* Get a the namespace node for the mutex */

    mutex_node = handle as *mut acpi_namespace_node;
    if !pathname.is_null() {
        status = acpi_get_handle(
            handle,
            pathname,
            (&mut mutex_node as *mut *mut acpi_namespace_node).cast::<acpi_handle>(),
        );
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    /* Ensure that we actually have a Mutex object */

    if mutex_node.is_null() || (*mutex_node).type_ != ACPI_TYPE_MUTEX {
        return AE_TYPE;
    }

    /* Get the low-level mutex object */

    mutex_obj = acpi_ns_get_attached_object(mutex_node);
    if mutex_obj.is_null() {
        return AE_NULL_OBJECT;
    }

    *ret_obj = mutex_obj;
    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_acquire_mutex
 *
 * PARAMETERS:  handle              - Mutex or prefix handle (optional)
 *              pathname            - Mutex pathname (optional)
 *              timeout             - Max time to wait for the lock (millisec)
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Acquire an AML mutex. This is a device driver interface to
 *              AML mutex objects, and allows for transaction locking between
 *              drivers and AML code. The mutex node is pointed to by
 *              Handle:Pathname. Either Handle or Pathname can be NULL, but
 *              not both.
 *
 ******************************************************************************/

pub unsafe fn acpi_acquire_mutex(
    handle: acpi_handle,
    pathname: acpi_string,
    timeout: u16,
) -> acpi_status {
    let mut mutex_obj: *mut acpi_operand_object = core::ptr::null_mut();

    /* Get the low-level mutex associated with Handle:Pathname */

    let mut status = acpi_ut_get_mutex_object(handle, pathname, &mut mutex_obj);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Acquire the OS mutex */

    status = acpi_os_acquire_mutex((*mutex_obj).mutex.os_mutex, timeout);
    status
}

// ACPI_EXPORT_SYMBOL(acpi_acquire_mutex)

/*******************************************************************************
 *
 * FUNCTION:    acpi_release_mutex
 *
 * PARAMETERS:  handle              - Mutex or prefix handle (optional)
 *              pathname            - Mutex pathname (optional)
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Release an AML mutex. This is a device driver interface to
 *              AML mutex objects, and allows for transaction locking between
 *              drivers and AML code. The mutex node is pointed to by
 *              Handle:Pathname. Either Handle or Pathname can be NULL, but
 *              not both.
 *
 ******************************************************************************/

pub unsafe fn acpi_release_mutex(handle: acpi_handle, pathname: acpi_string) -> acpi_status {
    let mut mutex_obj: *mut acpi_operand_object = core::ptr::null_mut();

    /* Get the low-level mutex associated with Handle:Pathname */

    let status = acpi_ut_get_mutex_object(handle, pathname, &mut mutex_obj);
    if ACPI_FAILURE(status) {
        return status;
    }

    /* Release the OS mutex */

    acpi_os_release_mutex((*mutex_obj).mutex.os_mutex);
    AE_OK
}

// ACPI_EXPORT_SYMBOL(acpi_release_mutex)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
