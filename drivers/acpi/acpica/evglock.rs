// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*
 * Module Name: evglock - Global Lock support
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 */

// C dependencies supplied by the surrounding ACPICA translation.

#[cfg(not(feature = "acpi_reduced_hardware"))]
const _COMPONENT: u32 = ACPI_EVENTS;

#[cfg(not(feature = "acpi_reduced_hardware"))]
unsafe fn acpi_ev_global_lock_handler(_context: *mut core::ffi::c_void) -> u32 {
    let mut status: acpi_status;
    let flags: acpi_cpu_flags;

    flags = acpi_os_acquire_lock(acpi_gbl_global_lock_pending_lock);

    if !acpi_gbl_global_lock_pending {
        acpi_os_release_lock(acpi_gbl_global_lock_pending_lock, flags);
        return ACPI_INTERRUPT_HANDLED;
    }

    status = acpi_os_signal_semaphore(acpi_gbl_global_lock_semaphore, 1);
    if ACPI_FAILURE(status) {
        ACPI_ERROR((AE_INFO, "Could not signal Global Lock semaphore"));
    }

    acpi_gbl_global_lock_pending = false;
    acpi_os_release_lock(acpi_gbl_global_lock_pending_lock, flags);
    ACPI_INTERRUPT_HANDLED
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_init_global_lock_handler() -> acpi_status {
    let mut status: acpi_status;

    ACPI_FUNCTION_TRACE!(ev_init_global_lock_handler);

    if acpi_gbl_reduced_hardware || !acpi_gbl_use_global_lock {
        return AE_OK;
    }

    status = acpi_install_fixed_event_handler(
        ACPI_EVENT_GLOBAL,
        acpi_ev_global_lock_handler,
        core::ptr::null_mut(),
    );

    acpi_gbl_global_lock_present = false;
    if status == AE_NO_HARDWARE_RESPONSE {
        ACPI_ERROR!((AE_INFO, "No response from Global Lock hardware, disabling lock"));
        return AE_OK;
    }

    status = acpi_os_create_lock(&mut acpi_gbl_global_lock_pending_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_gbl_global_lock_pending = false;
    acpi_gbl_global_lock_present = true;
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_remove_global_lock_handler() -> acpi_status {
    let status: acpi_status;

    ACPI_FUNCTION_TRACE!(ev_remove_global_lock_handler);

    acpi_gbl_global_lock_present = false;
    status = acpi_remove_fixed_event_handler(ACPI_EVENT_GLOBAL, acpi_ev_global_lock_handler);
    acpi_os_delete_lock(acpi_gbl_global_lock_pending_lock);
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_acquire_global_lock(timeout: u16) -> acpi_status {
    let mut flags: acpi_cpu_flags;
    let mut status: acpi_status;
    let mut acquired: u8 = false as u8;

    ACPI_FUNCTION_TRACE!(ev_acquire_global_lock);

    status = acpi_ex_system_wait_mutex(
        (*acpi_gbl_global_lock_mutex).mutex.os_mutex,
        timeout,
    );
    if ACPI_FAILURE(status) {
        return status;
    }

    acpi_gbl_global_lock_handle = acpi_gbl_global_lock_handle.wrapping_add(1);
    if acpi_gbl_global_lock_handle == 0 {
        acpi_gbl_global_lock_handle = 1;
    }

    if !acpi_gbl_global_lock_present {
        acpi_gbl_global_lock_acquired = true;
        return AE_OK;
    }

    flags = acpi_os_acquire_lock(acpi_gbl_global_lock_pending_lock);
    loop {
        // ACPI_ACQUIRE_GLOBAL_LOCK is an external ACPICA macro; preserve its
        // operation here through the surrounding translation's equivalent.
        acpi_acquire_global_lock(acpi_gbl_FACS, &mut acquired);
        if acquired != 0 {
            acpi_gbl_global_lock_acquired = true;
            ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "Acquired hardware Global Lock\n"));
            break;
        }

        acpi_gbl_global_lock_pending = true;
        acpi_os_release_lock(acpi_gbl_global_lock_pending_lock, flags);
        ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "Waiting for hardware Global Lock\n"));
        status = acpi_ex_system_wait_semaphore(acpi_gbl_global_lock_semaphore, ACPI_WAIT_FOREVER);
        flags = acpi_os_acquire_lock(acpi_gbl_global_lock_pending_lock);
        if !ACPI_SUCCESS(status) {
            break;
        }
    }

    acpi_gbl_global_lock_pending = false;
    acpi_os_release_lock(acpi_gbl_global_lock_pending_lock, flags);
    status
}

#[cfg(not(feature = "acpi_reduced_hardware"))]
pub unsafe fn acpi_ev_release_global_lock() -> acpi_status {
    let mut pending: u8 = false as u8;
    let mut status: acpi_status = AE_OK;

    ACPI_FUNCTION_TRACE!(ev_release_global_lock);

    if !acpi_gbl_global_lock_acquired {
        ACPI_WARNING!((AE_INFO, "Cannot release the ACPI Global Lock, it has not been acquired"));
        return AE_NOT_ACQUIRED;
    }

    if acpi_gbl_global_lock_present {
        acpi_release_global_lock(acpi_gbl_FACS, &mut pending);
        if pending != 0 {
            status = acpi_write_bit_register(ACPI_BITREG_GLOBAL_LOCK_RELEASE, ACPI_ENABLE_EVENT);
        }
        ACPI_DEBUG_PRINT!((ACPI_DB_EXEC, "Released hardware Global Lock\n"));
    }

    acpi_gbl_global_lock_acquired = false;
    acpi_os_release_mutex((*acpi_gbl_global_lock_mutex).mutex.os_mutex);
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
