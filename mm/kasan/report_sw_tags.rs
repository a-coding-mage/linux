// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains software tag-based KASAN specific error reporting code.
 *
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 *
 * Some code borrowed from https://github.com/xairy/kasan-prototype by
 *        Andrey Konovalov <andreyknvl@gmail.com>
 */

use core::ffi::{c_char, c_void};

// The following names are supplied by the surrounding kernel translation.
extern "C" {
    fn get_tag(addr: *const c_void) -> u8;
    fn kasan_reset_tag(addr: *const c_void) -> *mut c_void;
    fn addr_has_metadata(addr: *const c_void) -> bool;
    fn kasan_mem_to_shadow(addr: *const c_void) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn object_is_on_stack(addr: *const c_void) -> bool;
    fn task_pid_nr(task: *mut task_struct) -> i32;
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct kmem_cache {
    pub object_size: usize,
}

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
}

// Supplied by the surrounding kernel translation.
const KASAN_GRANULE_SIZE: usize = 0;
const KASAN_TAG_INVALID: u8 = 0;
const META_BYTES_PER_ROW: usize = 0;

unsafe extern "C" {
    fn pr_err(format: *const c_char, ...);
    fn WARN_ON(condition: bool) -> bool;
}

pub unsafe fn kasan_find_first_bad_addr(addr: *const c_void, size: usize) -> *const c_void {
    let tag: u8 = get_tag(addr);
    let mut p: *mut c_void = kasan_reset_tag(addr);
    let end = (p as *mut u8).add(size) as *mut c_void;

    if !addr_has_metadata(p) {
        return p;
    }

    while (p as usize) < (end as usize)
        && tag == *((kasan_mem_to_shadow(p)) as *const u8)
    {
        p = (p as *mut u8).add(KASAN_GRANULE_SIZE) as *mut c_void;
    }

    p
}

pub unsafe fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize {
    let mut size: usize = 0;
    let mut shadow: *mut u8 = kasan_mem_to_shadow(object) as *mut u8;

    /*
     * Skip the addr_has_metadata check, as this function only operates on
     * slab memory, which must have metadata.
     */

    /*
     * The loop below returns 0 for freed objects, for which KASAN cannot
     * calculate the allocation size based on the metadata.
     */
    while size < (*cache).object_size {
        if *shadow != KASAN_TAG_INVALID {
            size += KASAN_GRANULE_SIZE;
        } else {
            return size;
        }
        shadow = shadow.add(1);
    }

    (*cache).object_size
}

pub unsafe fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void) {
    memcpy(
        buffer as *mut c_void,
        kasan_mem_to_shadow(row) as *const c_void,
        META_BYTES_PER_ROW,
    );
}

pub unsafe fn kasan_print_tags(addr_tag: u8, addr: *const c_void) {
    let shadow: *mut u8 = kasan_mem_to_shadow(addr) as *mut u8;

    pr_err(
        b"Pointer tag: [%02x], memory tag: [%02x]\n\0".as_ptr() as *const c_char,
        addr_tag as i32,
        *shadow as i32,
    );
}

#[cfg(CONFIG_KASAN_STACK)]
pub unsafe fn kasan_print_address_stack_frame(addr: *const c_void) {
    if WARN_ON(!object_is_on_stack(addr)) {
        return;
    }

    pr_err(
        b"The buggy address belongs to stack of task %s/%d\n\0".as_ptr()
            as *const c_char,
        (*current).comm.as_ptr(),
        task_pid_nr(current),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
