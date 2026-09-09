/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2024 Google LLC
 *
 * dbitmap - dynamically sized bitmap library.
 *
 * Used by the binder driver to optimize the allocation of the smallest
 * available descriptor ID. Each bit in the bitmap represents the state
 * of an ID.
 *
 * A dbitmap can grow or shrink as needed. This part has been designed
 * considering that users might need to briefly release their locks in
 * order to allocate memory for the new bitmap. These operations then,
 * are verified to determine if the grow or shrink is sill valid.
 *
 * This library does not provide protection against concurrent access
 * by itself. Binder uses the proc->outer_lock for this purpose.
 */

use core::ffi::c_void;

// Supplied by the Linux bitmap and allocator APIs.
unsafe extern "C" {
    fn find_last_bit(addr: *const usize, size: u32) -> usize;
    fn find_next_zero_bit(addr: *const usize, size: u32, offset: usize) -> usize;
    fn bitmap_copy(dst: *mut usize, src: *const usize, nbits: u32);
    fn bitmap_zalloc(nbits: u32, flags: u32) -> *mut usize;
    fn set_bit(nr: usize, addr: *mut usize);
    fn clear_bit(nr: usize, addr: *mut usize);
    fn kfree(ptr: *mut c_void);
}

// GFP_KERNEL is a build-time allocator flag supplied by the kernel headers.
unsafe extern "C" {
    static GFP_KERNEL: u32;
}

pub const NBITS_MIN: u32 = usize::BITS;

#[repr(C)]
pub struct dbitmap {
    pub nbits: u32,
    pub map: *mut usize,
}

#[inline]
pub unsafe fn dbitmap_enabled(dmap: *mut dbitmap) -> bool {
    (*dmap).nbits != 0
}

#[inline]
pub unsafe fn dbitmap_free(dmap: *mut dbitmap) {
    (*dmap).nbits = 0;
    kfree((*dmap).map.cast::<c_void>());
    (*dmap).map = core::ptr::null_mut();
}

/* Returns the nbits that a dbitmap can shrink to, 0 if not possible. */
#[inline]
pub unsafe fn dbitmap_shrink_nbits(dmap: *mut dbitmap) -> u32 {
    let bit: usize;

    if (*dmap).nbits <= NBITS_MIN {
        return 0;
    }

    /*
     * Determine if the bitmap can shrink based on the position of
     * its last set bit. If the bit is within the first quarter of the
     * bitmap then shrinking is possible. In this case, the
     * bitmap should shrink to half its current size.
     */
    bit = find_last_bit((*dmap).map, (*dmap).nbits);
    if bit < ((*dmap).nbits >> 2) as usize {
        return (*dmap).nbits >> 1;
    }

    /* find_last_bit() returns dmap->nbits when no bits are set. */
    if bit == (*dmap).nbits as usize {
        return NBITS_MIN;
    }

    0
}

/* Replace the internal bitmap with a new one of different size */
#[inline]
pub unsafe fn dbitmap_replace(dmap: *mut dbitmap, new: *mut usize, nbits: u32) {
    let copy_nbits = if (*dmap).nbits < nbits { (*dmap).nbits } else { nbits };
    bitmap_copy(new, (*dmap).map, copy_nbits);
    kfree((*dmap).map.cast::<c_void>());
    (*dmap).map = new;
    (*dmap).nbits = nbits;
}

#[inline]
pub unsafe fn dbitmap_shrink(dmap: *mut dbitmap, new: *mut usize, nbits: u32) {
    if new.is_null() {
        return;
    }

    /*
     * Verify that shrinking to @nbits is still possible. The @new
     * bitmap might have been allocated without locks, so this call
     * could now be outdated. In this case, free @new and move on.
     */
    if !dbitmap_enabled(dmap) || dbitmap_shrink_nbits(dmap) != nbits {
        kfree(new.cast::<c_void>());
        return;
    }

    dbitmap_replace(dmap, new, nbits);
}

/* Returns the nbits that a dbitmap can grow to. */
#[inline]
pub unsafe fn dbitmap_grow_nbits(dmap: *mut dbitmap) -> u32 {
    (*dmap).nbits << 1
}

#[inline]
pub unsafe fn dbitmap_grow(dmap: *mut dbitmap, new: *mut usize, nbits: u32) {
    /*
     * Verify that growing to @nbits is still possible. The @new
     * bitmap might have been allocated without locks, so this call
     * could now be outdated. In this case, free @new and move on.
     */
    if !dbitmap_enabled(dmap) || nbits <= (*dmap).nbits {
        kfree(new.cast::<c_void>());
        return;
    }

    /*
     * Check for ENOMEM after confirming the grow operation is still
     * required. This ensures we only disable the dbitmap when it's
     * necessary. Once the dbitmap is disabled, binder will fallback
     * to slow_desc_lookup_olocked().
     */
    if new.is_null() {
        dbitmap_free(dmap);
        return;
    }

    dbitmap_replace(dmap, new, nbits);
}

/*
 * Finds and sets the next zero bit in the bitmap. Upon success @bit
 * is populated with the index and 0 is returned. Otherwise, -ENOSPC
 * is returned to indicate that a dbitmap_grow() is needed.
 */
#[inline]
pub unsafe fn dbitmap_acquire_next_zero_bit(
    dmap: *mut dbitmap,
    offset: usize,
    bit: *mut usize,
) -> i32 {
    let n = find_next_zero_bit((*dmap).map, (*dmap).nbits, offset);
    if n == (*dmap).nbits as usize {
        return -28; // -ENOSPC
    }

    *bit = n;
    set_bit(n, (*dmap).map);

    0
}

#[inline]
pub unsafe fn dbitmap_clear_bit(dmap: *mut dbitmap, bit: usize) {
    clear_bit(bit, (*dmap).map);
}

#[inline]
pub unsafe fn dbitmap_init(dmap: *mut dbitmap) -> i32 {
    (*dmap).map = bitmap_zalloc(NBITS_MIN, GFP_KERNEL);
    if (*dmap).map.is_null() {
        (*dmap).nbits = 0;
        return -12; // -ENOMEM
    }

    (*dmap).nbits = NBITS_MIN;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
