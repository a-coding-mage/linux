// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*!
 * Module Name: utmutex - local mutex support
 */

/* Dependencies are supplied by the surrounding ACPICA translation. */

pub unsafe fn acpi_ut_mutex_initialize() -> acpi_status {
    let mut status: acpi_status;

    for i in 0..ACPI_NUM_MUTEX {
        status = acpi_ut_create_mutex(i);
        if ACPI_FAILURE(status) {
            return status;
        }
    }

    status = acpi_os_create_lock(&mut acpi_gbl_gpe_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_os_create_raw_lock(&mut acpi_gbl_hardware_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_os_create_lock(&mut acpi_gbl_reference_count_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_os_create_mutex(&mut acpi_gbl_osi_mutex);
    if ACPI_FAILURE(status) {
        return status;
    }

    status = acpi_ut_create_rw_lock(&mut acpi_gbl_namespace_rw_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    status
}

pub unsafe fn acpi_ut_mutex_terminate() {
    for i in 0..ACPI_NUM_MUTEX {
        acpi_ut_delete_mutex(i);
    }

    acpi_os_delete_mutex(acpi_gbl_osi_mutex);
    acpi_os_delete_lock(acpi_gbl_gpe_lock);
    acpi_os_delete_raw_lock(acpi_gbl_hardware_lock);
    acpi_os_delete_lock(acpi_gbl_reference_count_lock);
    acpi_ut_delete_rw_lock(&mut acpi_gbl_namespace_rw_lock);
}

unsafe fn acpi_ut_create_mutex(mutex_id: acpi_mutex_handle) -> acpi_status {
    let mut status = AE_OK;

    if !acpi_gbl_mutex_info[mutex_id].mutex.is_null() {
        return status;
    }

    status = acpi_os_create_mutex(&mut acpi_gbl_mutex_info[mutex_id].mutex);
    acpi_gbl_mutex_info[mutex_id].thread_id = ACPI_MUTEX_NOT_ACQUIRED;
    acpi_gbl_mutex_info[mutex_id].use_count = 0;
    status
}

unsafe fn acpi_ut_delete_mutex(mutex_id: acpi_mutex_handle) {
    acpi_os_delete_mutex(acpi_gbl_mutex_info[mutex_id].mutex);
    acpi_gbl_mutex_info[mutex_id].mutex = core::ptr::null_mut();
    acpi_gbl_mutex_info[mutex_id].thread_id = ACPI_MUTEX_NOT_ACQUIRED;
}

pub unsafe fn acpi_ut_acquire_mutex(mutex_id: acpi_mutex_handle) -> acpi_status {
    let this_thread_id = acpi_os_get_thread_id();

    if mutex_id > ACPI_MAX_MUTEX {
        return AE_BAD_PARAMETER;
    }

    #[cfg(feature = "ACPI_MUTEX_DEBUG")]
    {
        for i in mutex_id..ACPI_NUM_MUTEX {
            if acpi_gbl_mutex_info[i].thread_id == this_thread_id {
                if i == mutex_id {
                    return AE_ALREADY_ACQUIRED;
                }
                return AE_ACQUIRE_DEADLOCK;
            }
        }
    }

    let status = acpi_os_acquire_mutex(
        acpi_gbl_mutex_info[mutex_id].mutex,
        ACPI_WAIT_FOREVER,
    );
    if ACPI_SUCCESS(status) {
        acpi_gbl_mutex_info[mutex_id].use_count += 1;
        acpi_gbl_mutex_info[mutex_id].thread_id = this_thread_id;
    }
    status
}

pub unsafe fn acpi_ut_release_mutex(mutex_id: acpi_mutex_handle) -> acpi_status {
    if mutex_id > ACPI_MAX_MUTEX {
        return AE_BAD_PARAMETER;
    }

    if acpi_gbl_mutex_info[mutex_id].thread_id == ACPI_MUTEX_NOT_ACQUIRED {
        return AE_NOT_ACQUIRED;
    }

    #[cfg(feature = "ACPI_MUTEX_DEBUG")]
    {
        for i in mutex_id..ACPI_NUM_MUTEX {
            if acpi_gbl_mutex_info[i].thread_id == acpi_os_get_thread_id() {
                if i == mutex_id {
                    continue;
                }
                return AE_RELEASE_DEADLOCK;
            }
        }
    }

    acpi_gbl_mutex_info[mutex_id].thread_id = ACPI_MUTEX_NOT_ACQUIRED;
    acpi_os_release_mutex(acpi_gbl_mutex_info[mutex_id].mutex);
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
