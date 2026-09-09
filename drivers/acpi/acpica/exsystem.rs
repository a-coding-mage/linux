// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: exsystem - Interface to OS services
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPI headers and neighboring translation units.

/* #define _COMPONENT ACPI_EXECUTER */
/* ACPI_MODULE_NAME("exsystem") */

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_wait_semaphore
 *
 * PARAMETERS:  semaphore       - Semaphore to wait on
 *              timeout         - Max time to wait
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Implements a semaphore wait with a check to see if the
 *              semaphore is available immediately. If it is not, the
 *              interpreter is released before waiting.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_wait_semaphore(semaphore: acpi_semaphore, timeout: u16) -> acpi_status {
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(ex_system_wait_semaphore);

    status = acpi_os_wait_semaphore(semaphore, 1, ACPI_DO_NOT_WAIT);
    if ACPI_SUCCESS(status) {
        return status;
    }

    if status == AE_TIME {
        /* We must wait, so unlock the interpreter */
        acpi_ex_exit_interpreter();
        status = acpi_os_wait_semaphore(semaphore, 1, timeout);

        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "*** Thread awake after blocking, %s\n",
        //                    acpi_format_exception(status)));

        /* Reacquire the interpreter */
        acpi_ex_enter_interpreter();
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_wait_mutex
 *
 * PARAMETERS:  mutex           - Mutex to wait on
 *              timeout         - Max time to wait
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Implements a mutex wait with a check to see if the
 *              mutex is available immediately. If it is not, the
 *              interpreter is released before waiting.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_wait_mutex(mutex: acpi_mutex, timeout: u16) -> acpi_status {
    let mut status: acpi_status;

    // ACPI_FUNCTION_TRACE(ex_system_wait_mutex);

    status = acpi_os_acquire_mutex(mutex, ACPI_DO_NOT_WAIT);
    if ACPI_SUCCESS(status) {
        return status;
    }

    if status == AE_TIME {
        /* We must wait, so unlock the interpreter */
        acpi_ex_exit_interpreter();
        status = acpi_os_acquire_mutex(mutex, timeout);

        // ACPI_DEBUG_PRINT((ACPI_DB_EXEC, "*** Thread awake after blocking, %s\n",
        //                    acpi_format_exception(status)));

        /* Reacquire the interpreter */
        acpi_ex_enter_interpreter();
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_do_stall
 *
 * PARAMETERS:  how_long_us     - The amount of time to stall,
 *                                in microseconds
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Suspend running thread for specified amount of time.
 *              Note: ACPI specification requires that Stall() does not
 *              relinquish the processor, and delays longer than 100 usec
 *              should use Sleep() instead. We allow stalls up to 255 usec
 *              for compatibility with other interpreters and existing BIOSs.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_do_stall(how_long_us: u32) -> acpi_status {
    let mut status: acpi_status = AE_OK;

    // ACPI_FUNCTION_ENTRY();

    if how_long_us > 255 {
        /* Longer than 255 microseconds, this is an error */
        // ACPI_ERROR_ONCE((AE_INFO, "Time parameter is too large (%u)", how_long_us));
        status = AE_AML_OPERAND_VALUE;
    } else {
        if how_long_us > 100 {
            // ACPI_WARNING_ONCE((AE_INFO,
            //     "Time parameter %u us > 100 us violating ACPI spec, please fix the firmware.",
            //     how_long_us));
        }
        acpi_os_stall(how_long_us);
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_do_sleep
 *
 * PARAMETERS:  how_long_ms     - The amount of time to sleep,
 *                                in milliseconds
 *
 * RETURN:      None
 *
 * DESCRIPTION: Sleep the running thread for specified amount of time.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_do_sleep(mut how_long_ms: u64) -> acpi_status {
    // ACPI_FUNCTION_ENTRY();

    /* Since this thread will sleep, we must release the interpreter */
    acpi_ex_exit_interpreter();

    /* For compatibility with other ACPI implementations and to prevent
     * accidental deep sleeps, limit the sleep time to something reasonable. */
    if how_long_ms > ACPI_MAX_SLEEP {
        how_long_ms = ACPI_MAX_SLEEP;
    }

    acpi_os_sleep(how_long_ms);

    /* And now we must get the interpreter again */
    acpi_ex_enter_interpreter();
    AE_OK
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_signal_event
 *
 * PARAMETERS:  obj_desc        - The object descriptor for this op
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Provides an access point to perform synchronization operations
 *              within the AML.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_signal_event(obj_desc: *mut acpi_operand_object) -> acpi_status {
    let mut status: acpi_status = AE_OK;

    // ACPI_FUNCTION_TRACE(ex_system_signal_event);

    if !obj_desc.is_null() {
        status = acpi_os_signal_semaphore((*obj_desc).event.os_semaphore, 1);
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_wait_event
 *
 * PARAMETERS:  time_desc       - The 'time to delay' object descriptor
 *              obj_desc        - The object descriptor for this op
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Provides an access point to perform synchronization operations
 *              within the AML. This operation is a request to wait for an
 *              event.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_wait_event(
    time_desc: *mut acpi_operand_object,
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;

    // ACPI_FUNCTION_TRACE(ex_system_wait_event);

    if !obj_desc.is_null() {
        status = acpi_ex_system_wait_semaphore(
            (*obj_desc).event.os_semaphore,
            (*time_desc).integer.value as u16,
        );
    }

    status
}

/*******************************************************************************
 *
 * FUNCTION:    acpi_ex_system_reset_event
 *
 * PARAMETERS:  obj_desc        - The object descriptor for this op
 *
 * RETURN:      Status
 *
 * DESCRIPTION: Reset an event to a known state.
 *
 ******************************************************************************/
pub unsafe fn acpi_ex_system_reset_event(obj_desc: *mut acpi_operand_object) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let mut temp_semaphore: acpi_semaphore;

    // ACPI_FUNCTION_ENTRY();

    /* We are going to simply delete the existing semaphore and
     * create a new one! */
    status = acpi_os_create_semaphore(ACPI_NO_UNIT_LIMIT, 0, &mut temp_semaphore);
    if ACPI_SUCCESS(status) {
        let _ = acpi_os_delete_semaphore((*obj_desc).event.os_semaphore);
        (*obj_desc).event.os_semaphore = temp_semaphore;
    }

    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
