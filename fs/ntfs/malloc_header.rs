/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * malloc.h - NTFS kernel memory handling. Part of the Linux-NTFS project.
 *
 * Copyright (c) 2001-2005 Anton Altaparmakov
 */

// Dependencies supplied by the surrounding kernel translation.

/**
 * __ntfs_malloc - allocate memory in multiples of pages
 * @size:\tnumber of bytes to allocate
 * @gfp_mask:\textra flags for the allocator
 *
 * Internal function.  You probably want ntfs_malloc_nofs()...
 *
 * Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
 * returns a pointer to the allocated memory.
 *
 * If there was insufficient memory to complete the request, return NULL.
 * Depending on @gfp_mask the allocation may be guaranteed to succeed.
 */
#[inline]
pub unsafe fn __ntfs_malloc(size: libc::c_ulong, gfp_mask: gfp_t) -> *mut libc::c_void {
    if likely(size <= PAGE_SIZE) {
        BUG_ON(!size);
        /* kmalloc() has per-CPU caches so is faster for now. */
        return kmalloc(PAGE_SIZE, gfp_mask & !__GFP_HIGHMEM);
        /* return (void *)__get_free_page(gfp_mask); */
    }
    if likely((size >> PAGE_SHIFT) < totalram_pages()) {
        return __vmalloc(size, gfp_mask);
    }
    core::ptr::null_mut()
}

/**
 * ntfs_malloc_nofs - allocate memory in multiples of pages
 * @size:\tnumber of bytes to allocate
 *
 * Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
 * returns a pointer to the allocated memory.
 *
 * If there was insufficient memory to complete the request, return NULL.
 */
#[inline]
pub unsafe fn ntfs_malloc_nofs(size: libc::c_ulong) -> *mut libc::c_void {
    __ntfs_malloc(size, GFP_NOFS | __GFP_HIGHMEM)
}

/**
 * ntfs_malloc_nofs_nofail - allocate memory in multiples of pages
 * @size:\tnumber of bytes to allocate
 *
 * Allocates @size bytes of memory, rounded up to multiples of PAGE_SIZE and
 * returns a pointer to the allocated memory.
 *
 * This function guarantees that the allocation will succeed.  It will sleep
 * for as long as it takes to complete the allocation.
 *
 * If there was insufficient memory to complete the request, return NULL.
 */
#[inline]
pub unsafe fn ntfs_malloc_nofs_nofail(size: libc::c_ulong) -> *mut libc::c_void {
    __ntfs_malloc(size, GFP_NOFS | __GFP_HIGHMEM | __GFP_NOFAIL)
}

#[inline]
pub unsafe fn ntfs_free(addr: *mut libc::c_void) {
    kvfree(addr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
