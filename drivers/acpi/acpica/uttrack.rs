// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: uttrack - Memory allocation tracking routines (debug only)
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 ******************************************************************************/

/*
 * These procedures are used for tracking memory leaks in the subsystem, and
 * they get compiled out when the ACPI_DBG_TRACK_ALLOCATIONS is not set.
 *
 * Each memory allocation is tracked via a doubly linked list. Each
 * element contains the caller's component, module name, function name, and
 * line number. acpi_ut_allocate and acpi_ut_allocate_zeroed call
 * acpi_ut_track_allocation to add an element to the list; deletion
 * occurs in the body of acpi_ut_free.
 */

/* C includes omitted; symbols supplied by the ACPICA Rust environment. */

#[cfg(feature = "ACPI_DBG_TRACK_ALLOCATIONS")]
mod acpi_dbg_track_allocations {
    use super::*;

    // #define _COMPONENT ACPI_UTILITIES
    // ACPI_MODULE_NAME("uttrack")

    unsafe fn acpi_ut_find_allocation(
        allocation: *mut acpi_debug_mem_block,
    ) -> *mut acpi_debug_mem_block {
        let mut element = (*acpi_gbl_global_list).list_head;
        if element.is_null() { return core::ptr::null_mut(); }

        /* List is ordered by larger-to-smaller addresses. */
        while (element as usize) > (allocation as usize) {
            if (*element).next.is_null() { return element; }
            element = (*element).next;
        }
        if element == allocation { element } else { (*element).previous }
    }

    unsafe fn acpi_ut_track_allocation(
        allocation: *mut acpi_debug_mem_block,
        size: acpi_size,
        alloc_type: u8,
        component: u32,
        module: *const core::ffi::c_char,
        line: u32,
    ) -> acpi_status {
        if acpi_gbl_disable_mem_tracking { return AE_OK; }

        let mem_list = acpi_gbl_global_list;
        let mut status = acpi_ut_acquire_mutex(ACPI_MTX_MEMORY);
        if ACPI_FAILURE(status) { return status; }

        let element = acpi_ut_find_allocation(allocation);
        if element == allocation {
            ACPI_ERROR!((AE_INFO, "UtTrackAllocation: Allocation (%p) already present in global list!", allocation));
            status = acpi_ut_release_mutex(ACPI_MTX_MEMORY);
            return status;
        }

        (*allocation).size = size as u32;
        (*allocation).alloc_type = alloc_type;
        (*allocation).component = component;
        (*allocation).line = line;
        acpi_ut_safe_strncpy((*allocation).module.as_mut_ptr(), module as *mut core::ffi::c_char, ACPI_MAX_MODULE_NAME);

        if element.is_null() {
            if !(*mem_list).list_head.is_null() { (*(*mem_list).list_head).previous = allocation; }
            (*allocation).next = (*mem_list).list_head;
            (*allocation).previous = core::ptr::null_mut();
            (*mem_list).list_head = allocation;
        } else {
            (*allocation).next = (*element).next;
            (*allocation).previous = element;
            if !(*element).next.is_null() { (*(*element).next).previous = allocation; }
            (*element).next = allocation;
        }

        status = acpi_ut_release_mutex(ACPI_MTX_MEMORY);
        status
    }

    unsafe fn acpi_ut_remove_allocation(
        allocation: *mut acpi_debug_mem_block,
        component: u32,
        module: *const core::ffi::c_char,
        line: u32,
    ) -> acpi_status {
        if acpi_gbl_disable_mem_tracking { return AE_OK; }
        let mem_list = acpi_gbl_global_list;
        if (*mem_list).list_head.is_null() {
            ACPI_ERROR!((module, line, "Empty allocation list, nothing to free!"));
            return AE_OK;
        }
        let status = acpi_ut_acquire_mutex(ACPI_MTX_MEMORY);
        if ACPI_FAILURE(status) { return status; }

        if !(*allocation).previous.is_null() { (*(*allocation).previous).next = (*allocation).next; }
        else { (*mem_list).list_head = (*allocation).next; }
        if !(*allocation).next.is_null() { (*(*allocation).next).previous = (*allocation).previous; }

        core::ptr::write_bytes((*allocation).user_space.as_mut_ptr(), 0xEA, (*allocation).size as usize);
        acpi_ut_release_mutex(ACPI_MTX_MEMORY)
    }

    pub unsafe fn acpi_ut_create_list(list_name: *const core::ffi::c_char, object_size: u16, return_cache: *mut *mut acpi_memory_list) -> acpi_status {
        let cache = acpi_os_allocate_zeroed(core::mem::size_of::<acpi_memory_list>()) as *mut acpi_memory_list;
        if cache.is_null() { return AE_NO_MEMORY; }
        (*cache).list_name = list_name;
        (*cache).object_size = object_size;
        *return_cache = cache;
        AE_OK
    }

    pub unsafe fn acpi_ut_allocate_and_track(size: acpi_size, component: u32, module: *const core::ffi::c_char, line: u32) -> *mut core::ffi::c_void {
        let mut size = size;
        if size == 0 { ACPI_WARNING!((module, line, "Attempt to allocate zero bytes, allocating 1 byte")); size = 1; }
        let allocation = acpi_os_allocate(size + core::mem::size_of::<acpi_debug_mem_header>()) as *mut acpi_debug_mem_block;
        if allocation.is_null() { ACPI_WARNING!((module, line, "Could not allocate size %u", size as u32)); return core::ptr::null_mut(); }
        if ACPI_FAILURE(acpi_ut_track_allocation(allocation, size, ACPI_MEM_MALLOC, component, module, line)) { acpi_os_free(allocation as *mut core::ffi::c_void); return core::ptr::null_mut(); }
        (*acpi_gbl_global_list).total_allocated += 1;
        (*acpi_gbl_global_list).total_size += size as u32;
        (*acpi_gbl_global_list).current_total_size += size as u32;
        if (*acpi_gbl_global_list).current_total_size > (*acpi_gbl_global_list).max_occupied { (*acpi_gbl_global_list).max_occupied = (*acpi_gbl_global_list).current_total_size; }
        (*allocation).user_space.as_mut_ptr() as *mut core::ffi::c_void
    }

    pub unsafe fn acpi_ut_allocate_zeroed_and_track(size: acpi_size, component: u32, module: *const core::ffi::c_char, line: u32) -> *mut core::ffi::c_void {
        let mut size = size;
        if size == 0 { ACPI_WARNING!((module, line, "Attempt to allocate zero bytes, allocating 1 byte")); size = 1; }
        let allocation = acpi_os_allocate_zeroed(size + core::mem::size_of::<acpi_debug_mem_header>()) as *mut acpi_debug_mem_block;
        if allocation.is_null() { ACPI_ERROR!((module, line, "Could not allocate size %u", size as u32)); return core::ptr::null_mut(); }
        if ACPI_FAILURE(acpi_ut_track_allocation(allocation, size, ACPI_MEM_CALLOC, component, module, line)) { acpi_os_free(allocation as *mut core::ffi::c_void); return core::ptr::null_mut(); }
        (*acpi_gbl_global_list).total_allocated += 1;
        (*acpi_gbl_global_list).total_size += size as u32;
        (*acpi_gbl_global_list).current_total_size += size as u32;
        if (*acpi_gbl_global_list).current_total_size > (*acpi_gbl_global_list).max_occupied { (*acpi_gbl_global_list).max_occupied = (*acpi_gbl_global_list).current_total_size; }
        (*allocation).user_space.as_mut_ptr() as *mut core::ffi::c_void
    }

    pub unsafe fn acpi_ut_free_and_track(allocation: *mut core::ffi::c_void, component: u32, module: *const core::ffi::c_char, line: u32) {
        if allocation.is_null() { ACPI_ERROR!((module, line, "Attempt to delete a NULL address")); return; }
        let debug_block = (allocation as *mut u8).sub(core::mem::size_of::<acpi_debug_mem_header>()) as *mut acpi_debug_mem_block;
        (*acpi_gbl_global_list).total_freed += 1;
        (*acpi_gbl_global_list).current_total_size -= (*debug_block).size;
        let status = acpi_ut_remove_allocation(debug_block, component, module, line);
        if ACPI_FAILURE(status) { ACPI_EXCEPTION!((AE_INFO, status, "Could not free memory")); }
        acpi_os_free(debug_block as *mut core::ffi::c_void);
    }

    pub unsafe fn acpi_ut_dump_allocation_info() { /* Diagnostic body is intentionally empty in the source. */ }

    pub unsafe fn acpi_ut_dump_allocations(component: u32, module: *const core::ffi::c_char) {
        if acpi_gbl_disable_mem_tracking { return; }
        if ACPI_FAILURE(acpi_ut_acquire_mutex(ACPI_MTX_MEMORY)) { return; }
        let mut element = if acpi_gbl_global_list.is_null() { core::ptr::null_mut() } else { (*acpi_gbl_global_list).list_head };
        let mut num_outstanding: u32 = 0;
        while !element.is_null() {
            if ((*element).component & component) != 0 && (module.is_null() || strcmp(module, (*element).module.as_ptr()) == 0) {
                let descriptor = (*element).user_space.as_mut_ptr() as *mut acpi_descriptor;
                if (*element).size >= core::mem::size_of::<acpi_common_descriptor>() && ACPI_GET_DESCRIPTOR_TYPE(descriptor) != ACPI_DESC_TYPE_CACHED {
                    acpi_os_printf("%p Length 0x%04X %9.9s-%4.4u\n", descriptor, (*element).size, (*element).module.as_ptr(), (*element).line);
                    if acpi_gbl_verbose_leak_dump { acpi_ut_dump_buffer(descriptor as *const u8, (*element).size, DB_BYTE_DISPLAY, 0); }
                }
                num_outstanding += 1;
            }
            element = (*element).next;
        }
        let _ = acpi_ut_release_mutex(ACPI_MTX_MEMORY);
        if num_outstanding == 0 { ACPI_INFO!(("No outstanding allocations")); }
        else { ACPI_ERROR!((AE_INFO, "%u (0x%X) Outstanding cache allocations", num_outstanding, num_outstanding)); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
