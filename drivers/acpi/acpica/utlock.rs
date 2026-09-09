// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utlock - Reader/Writer lock interfaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

// Dependencies supplied by the ACPICA headers:
// #include <acpi/acpi.h>
// #include "accommon.h"

pub const _COMPONENT: u32 = ACPI_UTILITIES;
// ACPI_MODULE_NAME("utlock")

pub const ACPI_WAIT_FOREVER: u32 = 0xffff_ffff;

#[repr(C)]
pub struct acpi_rw_lock {
    pub num_readers: u32,
    pub reader_mutex: *mut core::ffi::c_void,
    pub writer_mutex: *mut core::ffi::c_void,
}

pub type acpi_status = i32;

unsafe extern "C" {
    fn acpi_os_create_mutex(out_handle: *mut *mut core::ffi::c_void) -> acpi_status;
    fn acpi_os_delete_mutex(handle: *mut core::ffi::c_void);
    fn acpi_os_acquire_mutex(handle: *mut core::ffi::c_void, timeout: u32) -> acpi_status;
    fn acpi_os_release_mutex(handle: *mut core::ffi::c_void) -> acpi_status;
}

#[inline]
unsafe fn acpi_failure(status: acpi_status) -> bool {
    status != 0
}

pub unsafe fn acpi_ut_create_rw_lock(lock: *mut acpi_rw_lock) -> acpi_status {
    let mut status: acpi_status;

    (*lock).num_readers = 0;
    status = acpi_os_create_mutex(&mut (*lock).reader_mutex);
    if acpi_failure(status) {
        return status;
    }

    status = acpi_os_create_mutex(&mut (*lock).writer_mutex);
    status
}

pub unsafe fn acpi_ut_delete_rw_lock(lock: *mut acpi_rw_lock) {
    acpi_os_delete_mutex((*lock).reader_mutex);
    acpi_os_delete_mutex((*lock).writer_mutex);

    (*lock).num_readers = 0;
    (*lock).reader_mutex = core::ptr::null_mut();
    (*lock).writer_mutex = core::ptr::null_mut();
}

pub unsafe fn acpi_ut_acquire_read_lock(lock: *mut acpi_rw_lock) -> acpi_status {
    let mut status: acpi_status;

    status = acpi_os_acquire_mutex((*lock).reader_mutex, ACPI_WAIT_FOREVER);
    if acpi_failure(status) {
        return status;
    }

    // Acquire the write lock only for the first reader

    (*lock).num_readers += 1;
    if (*lock).num_readers == 1 {
        status = acpi_os_acquire_mutex((*lock).writer_mutex, ACPI_WAIT_FOREVER);
    }

    acpi_os_release_mutex((*lock).reader_mutex);
    status
}

pub unsafe fn acpi_ut_release_read_lock(lock: *mut acpi_rw_lock) -> acpi_status {
    let status: acpi_status;

    status = acpi_os_acquire_mutex((*lock).reader_mutex, ACPI_WAIT_FOREVER);
    if acpi_failure(status) {
        return status;
    }

    // Release the write lock only for the very last reader

    (*lock).num_readers -= 1;
    if (*lock).num_readers == 0 {
        acpi_os_release_mutex((*lock).writer_mutex);
    }

    acpi_os_release_mutex((*lock).reader_mutex);
    status
}

pub unsafe fn acpi_ut_acquire_write_lock(lock: *mut acpi_rw_lock) -> acpi_status {
    let status: acpi_status;

    status = acpi_os_acquire_mutex((*lock).writer_mutex, ACPI_WAIT_FOREVER);
    status
}

pub unsafe fn acpi_ut_release_write_lock(lock: *mut acpi_rw_lock) {
    acpi_os_release_mutex((*lock).writer_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
