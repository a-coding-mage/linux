/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2024 Intel Corporation
 */

// Dependencies supplied by the Linux kernel headers:
// linux/mm_types.h, linux/shmem_fs.h

/**
 * ttm_backup_handle_to_page_ptr() - Convert handle to struct page pointer
 * @handle: The handle to convert.
 *
 * Converts an opaque handle received from a ttm_backup_backup_*()
 * function to an (invalid) struct page pointer suitable for a struct page array.
 *
 * Return: An (invalid) struct page pointer.
 */
#[inline]
pub fn ttm_backup_handle_to_page_ptr(handle: std::os::raw::c_ulong) -> *mut page {
    (handle.wrapping_shl(1) | 1) as *mut page
}

/**
 * ttm_backup_page_ptr_is_handle() - Whether a struct page pointer is a handle
 * @page: The struct page pointer to check.
 *
 * Return: true if the struct page pointer is a handld returned from
 * ttm_backup_handle_to_page_ptr(). False otherwise.
 */
#[inline]
pub fn ttm_backup_page_ptr_is_handle(page: *const page) -> bool {
    (page as std::os::raw::c_ulong & 1) != 0
}

/**
 * ttm_backup_page_ptr_to_handle() - Convert a struct page pointer to a handle
 * @page: The struct page pointer to convert
 *
 * Return: The handle that was previously used in
 * ttm_backup_handle_to_page_ptr() to obtain a struct page pointer, suitable
 * for use as argument in the struct ttm_backup_drop() or
 * ttm_backup_copy_page() functions.
 */
#[inline]
pub fn ttm_backup_page_ptr_to_handle(page: *const page) -> std::os::raw::c_ulong {
    // WARN_ON is supplied by the kernel dependency environment.
    unsafe {
        WARN_ON(!ttm_backup_page_ptr_is_handle(page));
    }
    (page as std::os::raw::c_ulong) >> 1
}

extern "C" {
    pub fn ttm_backup_drop(backup: *mut file, handle: pgoff_t);

    pub fn ttm_backup_copy_page(
        backup: *mut file,
        dst: *mut page,
        handle: pgoff_t,
        intr: bool,
        additional_gfp: gfp_t,
    ) -> std::os::raw::c_int;

    pub fn ttm_backup_backup_folio(
        backup: *mut file,
        folio: *mut folio,
        order: std::os::raw::c_uint,
        writeback: bool,
        idx: pgoff_t,
        folio_gfp: gfp_t,
        alloc_gfp: gfp_t,
        nr_pages_backed: *mut pgoff_t,
    ) -> i64;

    pub fn ttm_backup_fini(backup: *mut file);

    pub fn ttm_backup_bytes_avail() -> u64;

    pub fn ttm_backup_shmem_create(size: i64) -> *mut file;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
