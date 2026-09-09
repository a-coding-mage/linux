// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: utcache - local cache allocation routines
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

use core::ffi::{c_char, c_void};

// Dependencies supplied by the ACPI implementation.
pub type acpi_status = i32;
pub const AE_OK: acpi_status = 0;
pub const AE_BAD_PARAMETER: acpi_status = 1;
pub const AE_NO_MEMORY: acpi_status = 2;
pub const ACPI_MTX_CACHES: u32 = 0;
pub const ACPI_DESC_TYPE_CACHED: u8 = 0;

#[repr(C)]
pub struct acpi_memory_list {
    pub list_name: *mut c_char,
    pub list_head: *mut c_void,
    pub object_size: u16,
    pub max_depth: u16,
    pub current_depth: u16,
    pub total_allocated: u32,
    pub total_freed: u32,
    pub max_occupied: u32,
    pub requests: u32,
    pub hits: u32,
}

extern "C" {
    fn acpi_os_allocate(size: usize) -> *mut c_void;
    fn acpi_os_free(object: *mut c_void);
    fn acpi_ut_acquire_mutex(mutex_id: u32) -> acpi_status;
    fn acpi_ut_release_mutex(mutex_id: u32) -> acpi_status;
    fn acpi_get_descriptor_ptr(object: *mut c_void) -> *mut c_void;
    fn acpi_set_descriptor_type(object: *mut c_void, descriptor_type: u8);
    fn acpi_set_descriptor_ptr(object: *mut c_void, next: *mut c_void);
    fn acpi_allocate_zeroed(size: usize) -> *mut c_void;
}

#[inline]
unsafe fn acpi_failure(status: acpi_status) -> bool {
    status != AE_OK
}

#[cfg(feature = "acpi_use_local_cache")]
pub unsafe extern "C" fn acpi_os_create_cache(
    cache_name: *mut c_char,
    object_size: u16,
    max_depth: u16,
    return_cache: *mut *mut acpi_memory_list,
) -> acpi_status {
    if cache_name.is_null() || return_cache.is_null() || object_size == 0 {
        return AE_BAD_PARAMETER;
    }

    /* Create the cache object */
    let cache = acpi_os_allocate(core::mem::size_of::<acpi_memory_list>())
        as *mut acpi_memory_list;
    if cache.is_null() {
        return AE_NO_MEMORY;
    }

    /* Populate the cache object and return it */
    core::ptr::write_bytes(cache, 0, 1);
    (*cache).list_name = cache_name;
    (*cache).object_size = object_size;
    (*cache).max_depth = max_depth;
    *return_cache = cache;
    AE_OK
}

#[cfg(feature = "acpi_use_local_cache")]
pub unsafe extern "C" fn acpi_os_purge_cache(cache: *mut acpi_memory_list) -> acpi_status {
    if cache.is_null() {
        return AE_BAD_PARAMETER;
    }

    let status = acpi_ut_acquire_mutex(ACPI_MTX_CACHES);
    if acpi_failure(status) {
        return status;
    }

    /* Walk the list of objects in this cache */
    while !(*cache).list_head.is_null() {
        /* Delete and unlink one cached state object */
        let next = acpi_get_descriptor_ptr((*cache).list_head);
        acpi_os_free((*cache).list_head);
        (*cache).list_head = next;
        (*cache).current_depth = (*cache).current_depth.wrapping_sub(1);
    }

    let _ = acpi_ut_release_mutex(ACPI_MTX_CACHES);
    AE_OK
}

#[cfg(feature = "acpi_use_local_cache")]
pub unsafe extern "C" fn acpi_os_delete_cache(cache: *mut acpi_memory_list) -> acpi_status {
    let status = acpi_os_purge_cache(cache);
    if acpi_failure(status) {
        return status;
    }
    acpi_os_free(cache as *mut c_void);
    AE_OK
}

#[cfg(feature = "acpi_use_local_cache")]
pub unsafe extern "C" fn acpi_os_release_object(
    cache: *mut acpi_memory_list,
    object: *mut c_void,
) -> acpi_status {
    if cache.is_null() || object.is_null() {
        return AE_BAD_PARAMETER;
    }

    /* If cache is full, just free this object */
    if (*cache).current_depth >= (*cache).max_depth {
        acpi_os_free(object);
        (*cache).total_freed = (*cache).total_freed.wrapping_add(1);
    } else {
        let status = acpi_ut_acquire_mutex(ACPI_MTX_CACHES);
        if acpi_failure(status) {
            return status;
        }

        /* Mark the object as cached */
        core::ptr::write_bytes(object, 0xCA, (*cache).object_size as usize);
        acpi_set_descriptor_type(object, ACPI_DESC_TYPE_CACHED);

        /* Put the object at the head of the cache list */
        acpi_set_descriptor_ptr(object, (*cache).list_head);
        (*cache).list_head = object;
        (*cache).current_depth = (*cache).current_depth.wrapping_add(1);
        let _ = acpi_ut_release_mutex(ACPI_MTX_CACHES);
    }
    AE_OK
}

#[cfg(feature = "acpi_use_local_cache")]
pub unsafe extern "C" fn acpi_os_acquire_object(
    cache: *mut acpi_memory_list,
) -> *mut c_void {
    if cache.is_null() {
        return core::ptr::null_mut();
    }

    let status = acpi_ut_acquire_mutex(ACPI_MTX_CACHES);
    if acpi_failure(status) {
        return core::ptr::null_mut();
    }
    (*cache).requests = (*cache).requests.wrapping_add(1);

    if !(*cache).list_head.is_null() {
        let object = (*cache).list_head;
        (*cache).list_head = acpi_get_descriptor_ptr(object);
        (*cache).current_depth = (*cache).current_depth.wrapping_sub(1);
        (*cache).hits = (*cache).hits.wrapping_add(1);
        let status = acpi_ut_release_mutex(ACPI_MTX_CACHES);
        if acpi_failure(status) {
            return core::ptr::null_mut();
        }
        core::ptr::write_bytes(object, 0, (*cache).object_size as usize);
        object
    } else {
        let status = acpi_ut_release_mutex(ACPI_MTX_CACHES);
        if acpi_failure(status) {
            return core::ptr::null_mut();
        }
        acpi_allocate_zeroed((*cache).object_size as usize)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
