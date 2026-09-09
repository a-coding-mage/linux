// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains hardware tag-based KASAN specific error reporting code.
 *
 * Copyright (c) 2020 Google, Inc.
 * Author: Andrey Konovalov <andreyknvl@google.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct kmem_cache {
    pub object_size: usize,
}

extern "C" {
    fn kasan_reset_tag(addr: *const core::ffi::c_void) -> *const core::ffi::c_void;
    fn hw_get_mem_tag(addr: *mut core::ffi::c_void) -> u8;
    fn pr_err(format: *const core::ffi::c_char, ...);
}

extern "C" {
    static KASAN_GRANULE_SIZE: usize;
    static KASAN_TAG_INVALID: u8;
    static META_BYTES_PER_ROW: i32;
}

pub unsafe fn kasan_find_first_bad_addr(
    addr: *const core::ffi::c_void,
    _size: usize,
) -> *const core::ffi::c_void {
    /*
     * Hardware Tag-Based KASAN only calls this function for normal memory
     * accesses, and thus addr points precisely to the first bad address
     * with an invalid (and present) memory tag. Therefore:
     * 1. Return the address as is without walking memory tags.
     * 2. Skip the addr_has_metadata check.
     */
    kasan_reset_tag(addr)
}

pub unsafe fn kasan_get_alloc_size(object: *mut core::ffi::c_void, cache: *const kmem_cache) -> usize {
    let mut size: usize = 0;
    let mut i: i32 = 0;
    let mut memory_tag: u8;

    /*
     * Skip the addr_has_metadata check, as this function only operates on
     * slab memory, which must have metadata.
     */

    /*
     * The loop below returns 0 for freed objects, for which KASAN cannot
     * calculate the allocation size based on the metadata.
     */
    while size < (*cache).object_size {
        memory_tag = hw_get_mem_tag(object.add((i as usize).wrapping_mul(KASAN_GRANULE_SIZE)));
        if memory_tag != KASAN_TAG_INVALID {
            size = size.wrapping_add(KASAN_GRANULE_SIZE);
        } else {
            return size;
        }
        i += 1;
    }

    (*cache).object_size
}

pub unsafe fn kasan_metadata_fetch_row(buffer: *mut core::ffi::c_char, row: *mut core::ffi::c_void) {
    let mut i: i32;

    i = 0;
    while i < META_BYTES_PER_ROW {
        *buffer.add(i as usize) = hw_get_mem_tag(
            row.add((i as usize).wrapping_mul(KASAN_GRANULE_SIZE)),
        ) as core::ffi::c_char;
        i += 1;
    }
}

pub unsafe fn kasan_print_tags(addr_tag: u8, addr: *const core::ffi::c_void) {
    let memory_tag: u8 = hw_get_mem_tag(addr as *mut core::ffi::c_void);

    pr_err(
        b"Pointer tag: [%02x], memory tag: [%02x]\n\0".as_ptr() as *const core::ffi::c_char,
        addr_tag as core::ffi::c_int,
        memory_tag as core::ffi::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
