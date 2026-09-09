// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: exmutex - ASL Mutex Acquire/Release functions
//
// Copyright (C) 2000 - 2026, Intel Corp.

// Dependencies supplied by the ACPI implementation are intentionally external.

unsafe fn acpi_ex_link_mutex(
    obj_desc: *mut acpi_operand_object,
    thread: *mut acpi_thread_state,
) {
    let list_head: *mut acpi_operand_object = (*thread).acquired_mutex_list;

    /* This object will be the first object in the list */
    (*obj_desc).mutex.prev = core::ptr::null_mut();
    (*obj_desc).mutex.next = list_head;

    /* Update old first object to point back to this object */
    if !list_head.is_null() {
        (*list_head).mutex.prev = obj_desc;
    }

    /* Update list head */
    (*thread).acquired_mutex_list = obj_desc;
}

pub unsafe fn acpi_ex_unlink_mutex(obj_desc: *mut acpi_operand_object) {
    let thread: *mut acpi_thread_state = (*obj_desc).mutex.owner_thread;

    if thread.is_null() {
        return;
    }

    /* Doubly linked list */
    if !(*obj_desc).mutex.next.is_null() {
        (*(*obj_desc).mutex.next).mutex.prev = (*obj_desc).mutex.prev;
    }

    if !(*obj_desc).mutex.prev.is_null() {
        (*(*obj_desc).mutex.prev).mutex.next = (*obj_desc).mutex.next;
        /* Migrate the previous sync level associated with this mutex to the
         * previous mutex on the list so that it may be preserved. */
        (*(*obj_desc).mutex.prev).mutex.original_sync_level =
            (*obj_desc).mutex.original_sync_level;
    } else {
        (*thread).acquired_mutex_list = (*obj_desc).mutex.next;
    }
}

pub unsafe fn acpi_ex_acquire_mutex_object(
    timeout: u16,
    obj_desc: *mut acpi_operand_object,
    thread_id: acpi_thread_id,
) -> acpi_status {
    if obj_desc.is_null() {
        return AE_BAD_PARAMETER;
    }

    /* Support for multiple acquires by the owning thread */
    if (*obj_desc).mutex.thread_id == thread_id {
        (*obj_desc).mutex.acquisition_depth += 1;
        return AE_OK;
    }

    let status: acpi_status;
    if obj_desc == acpi_gbl_global_lock_mutex {
        status = acpi_ev_acquire_global_lock(timeout);
    } else {
        status = acpi_ex_system_wait_mutex((*obj_desc).mutex.os_mutex, timeout);
    }

    if ACPI_FAILURE(status) {
        return status;
    }

    (*obj_desc).mutex.thread_id = thread_id;
    (*obj_desc).mutex.acquisition_depth = 1;
    (*obj_desc).mutex.original_sync_level = 0;
    (*obj_desc).mutex.owner_thread = core::ptr::null_mut();
    AE_OK
}

pub unsafe fn acpi_ex_acquire_mutex(
    time_desc: *mut acpi_operand_object,
    obj_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    if obj_desc.is_null() {
        return AE_BAD_PARAMETER;
    }
    if (*walk_state).thread.is_null() {
        return AE_AML_INTERNAL;
    }

    let thread = (*walk_state).thread;
    if (*thread).current_sync_level > (*obj_desc).mutex.sync_level {
        return AE_AML_MUTEX_ORDER;
    }

    let status = acpi_ex_acquire_mutex_object(
        (*time_desc).integer.value as u16,
        obj_desc,
        (*thread).thread_id,
    );

    if ACPI_SUCCESS(status) && (*obj_desc).mutex.acquisition_depth == 1 {
        (*obj_desc).mutex.owner_thread = thread;
        (*obj_desc).mutex.original_sync_level = (*thread).current_sync_level;
        (*thread).current_sync_level = (*obj_desc).mutex.sync_level;
        acpi_ex_link_mutex(obj_desc, thread);
    }
    status
}

pub unsafe fn acpi_ex_release_mutex_object(
    obj_desc: *mut acpi_operand_object,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    if (*obj_desc).mutex.acquisition_depth == 0 {
        return AE_NOT_ACQUIRED;
    }

    (*obj_desc).mutex.acquisition_depth -= 1;
    if (*obj_desc).mutex.acquisition_depth != 0 {
        return AE_OK;
    }

    if !(*obj_desc).mutex.owner_thread.is_null() {
        acpi_ex_unlink_mutex(obj_desc);
        (*obj_desc).mutex.owner_thread = core::ptr::null_mut();
    }

    if obj_desc == acpi_gbl_global_lock_mutex {
        status = acpi_ev_release_global_lock();
    } else {
        acpi_os_release_mutex((*obj_desc).mutex.os_mutex);
    }

    (*obj_desc).mutex.thread_id = 0;
    status
}

pub unsafe fn acpi_ex_release_mutex(
    obj_desc: *mut acpi_operand_object,
    walk_state: *mut acpi_walk_state,
) -> acpi_status {
    if obj_desc.is_null() {
        return AE_BAD_PARAMETER;
    }

    let owner_thread = (*obj_desc).mutex.owner_thread;
    if owner_thread.is_null() {
        return AE_AML_MUTEX_NOT_ACQUIRED;
    }
    if (*walk_state).thread.is_null() {
        return AE_AML_INTERNAL;
    }

    let thread = (*walk_state).thread;
    if (*owner_thread).thread_id != (*thread).thread_id
        && obj_desc != acpi_gbl_global_lock_mutex
    {
        return AE_AML_NOT_OWNER;
    }

    if (*obj_desc).mutex.sync_level != (*owner_thread).current_sync_level {
        return AE_AML_MUTEX_ORDER;
    }

    let previous_sync_level =
        (*(*owner_thread).acquired_mutex_list).mutex.original_sync_level;
    let status = acpi_ex_release_mutex_object(obj_desc);
    if ACPI_FAILURE(status) {
        return status;
    }

    if (*obj_desc).mutex.acquisition_depth == 0 {
        (*owner_thread).current_sync_level = previous_sync_level;
    }
    status
}

pub unsafe fn acpi_ex_release_all_mutexes(thread: *mut acpi_thread_state) {
    let mut next = (*thread).acquired_mutex_list;

    while !next.is_null() {
        let obj_desc = next;
        if obj_desc == acpi_gbl_global_lock_mutex {
            let _ = acpi_ev_release_global_lock();
        } else {
            acpi_os_release_mutex((*obj_desc).mutex.os_mutex);
        }

        (*thread).current_sync_level = (*obj_desc).mutex.original_sync_level;
        next = (*obj_desc).mutex.next;
        (*obj_desc).mutex.prev = core::ptr::null_mut();
        (*obj_desc).mutex.next = core::ptr::null_mut();
        (*obj_desc).mutex.acquisition_depth = 0;
        (*obj_desc).mutex.owner_thread = core::ptr::null_mut();
        (*obj_desc).mutex.thread_id = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
