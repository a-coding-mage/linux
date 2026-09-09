// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: utstate - state object support procedures
 *
 ******************************************************************************/

// Dependencies supplied by the ACPI headers and common implementation.

/*
 * void acpi_ut_push_generic_state(union acpi_generic_state **list_head,
 *                                  union acpi_generic_state *state)
 *
 * Push a state object onto a state stack.
 */
pub unsafe fn acpi_ut_push_generic_state(
    list_head: *mut *mut acpi_generic_state,
    state: *mut acpi_generic_state,
) {
    // ACPI_FUNCTION_ENTRY();

    /* Push the state object onto the front of the list (stack) */

    (*state).common.next = *list_head;
    *list_head = state;
}

/*
 * union acpi_generic_state *acpi_ut_pop_generic_state(
 *     union acpi_generic_state **list_head)
 *
 * Pop a state object from a state stack.
 */
pub unsafe fn acpi_ut_pop_generic_state(
    list_head: *mut *mut acpi_generic_state,
) -> *mut acpi_generic_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    /* Remove the state object at the head of the list (stack) */

    state = *list_head;
    if !state.is_null() {
        /* Update the list head */
        *list_head = (*state).common.next;
    }

    state
}

/*
 * Create a generic state object. Attempt to obtain one from the global state
 * cache; If none available, create a new one.
 */
pub unsafe fn acpi_ut_create_generic_state() -> *mut acpi_generic_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    state = acpi_os_acquire_object(acpi_gbl_state_cache);
    if !state.is_null() {
        /* Initialize */
        (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE;
    }

    state
}

/* Create a "Thread State" used to track per-thread info during method execution. */
pub unsafe fn acpi_ut_create_thread_state() -> *mut acpi_thread_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    /* Create the generic state object */
    state = acpi_ut_create_generic_state();
    if state.is_null() {
        return core::ptr::null_mut();
    }

    /* Init fields specific to the update struct */
    (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE_THREAD;
    (*state).thread.thread_id = acpi_os_get_thread_id();

    /* Check for invalid thread ID - zero is very bad, it will break things */
    if (*state).thread.thread_id == 0 {
        ACPI_ERROR((AE_INFO, "Invalid zero ID from AcpiOsGetThreadId"));
        (*state).thread.thread_id = 1 as acpi_thread_id;
    }

    state as *mut acpi_thread_state
}

/* Create an "Update State" used to update reference counts and delete objects. */
pub unsafe fn acpi_ut_create_update_state(
    object: *mut acpi_operand_object,
    action: u16,
) -> *mut acpi_generic_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    /* Create the generic state object */
    state = acpi_ut_create_generic_state();
    if state.is_null() {
        return core::ptr::null_mut();
    }

    /* Init fields specific to the update struct */
    (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE_UPDATE;
    (*state).update.object = object;
    (*state).update.value = action;
    state
}

/* Create a "Package State". */
pub unsafe fn acpi_ut_create_pkg_state(
    internal_object: *mut core::ffi::c_void,
    external_object: *mut core::ffi::c_void,
    index: u32,
) -> *mut acpi_generic_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    /* Create the generic state object */
    state = acpi_ut_create_generic_state();
    if state.is_null() {
        return core::ptr::null_mut();
    }

    /* Init fields specific to the update struct */
    (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE_PACKAGE;
    (*state).pkg.source_object = internal_object as *mut acpi_operand_object;
    (*state).pkg.dest_object = external_object;
    (*state).pkg.index = index;
    (*state).pkg.num_packages = 1;
    state
}

/* Create a "Control State" for nested IF/WHILE constructs in AML. */
pub unsafe fn acpi_ut_create_control_state() -> *mut acpi_generic_state {
    let state: *mut acpi_generic_state;

    // ACPI_FUNCTION_ENTRY();

    /* Create the generic state object */
    state = acpi_ut_create_generic_state();
    if state.is_null() {
        return core::ptr::null_mut();
    }

    /* Init fields specific to the control struct */
    (*state).common.descriptor_type = ACPI_DESC_TYPE_STATE_CONTROL;
    (*state).common.state = ACPI_CONTROL_CONDITIONAL_EXECUTING;
    state
}

/* Release a state object to the state cache. NULL state objects are ignored. */
pub unsafe fn acpi_ut_delete_generic_state(state: *mut acpi_generic_state) {
    // ACPI_FUNCTION_ENTRY();

    /* Ignore null state */
    if !state.is_null() {
        let _ = acpi_os_release_object(acpi_gbl_state_cache, state);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
