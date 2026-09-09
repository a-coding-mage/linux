// SPDX-License-Identifier: GPL-2.0
/*
 * Implement primitive realloc(3) functionality.
 *
 * Author: Mark A. Greer <mgreer@mvista.com>
 *
 * 2006 (c) MontaVista, Software, Inc.
 */

use core::ffi::c_void;
use core::ptr;

// Types and platform operations are supplied by the surrounding bootwrapper.

const ENTRY_BEEN_USED: ::core::ffi::c_ulong = 0x01;
const ENTRY_IN_USE: ::core::ffi::c_ulong = 0x02;

#[repr(C)]
struct AllocInfo {
    flags: ::core::ffi::c_ulong,
    base: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
}

static mut ALLOC_TBL: *mut AllocInfo = ptr::null_mut();
static mut TBL_ENTRIES: ::core::ffi::c_ulong = 0;
static mut ALLOC_MIN: ::core::ffi::c_ulong = 0;
static mut NEXT_BASE: ::core::ffi::c_ulong = 0;
static mut SPACE_LEFT: ::core::ffi::c_ulong = 0;

#[inline]
unsafe fn align_up(value: ::core::ffi::c_ulong, alignment: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    (value + alignment - 1) & !(alignment - 1)
}

extern "C" {
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: i32, n: usize) -> *mut c_void;
    static mut platform_ops: PlatformOps;
}

#[repr(C)]
struct PlatformOps {
    malloc: Option<unsafe extern "C" fn(::core::ffi::c_ulong) -> *mut c_void>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
    realloc: Option<unsafe extern "C" fn(*mut c_void, ::core::ffi::c_ulong) -> *mut c_void>,
}

unsafe extern "C" fn simple_malloc(mut size: ::core::ffi::c_ulong) -> *mut c_void {
    let mut p = ALLOC_TBL;

    if size == 0 {
        return ptr::null_mut();
    }

    size = align_up(size, ALLOC_MIN);

    for _ in 0..TBL_ENTRIES {
        if (*p).flags & ENTRY_BEEN_USED == 0 {
            if size <= SPACE_LEFT {
                (*p).base = NEXT_BASE;
                (*p).size = size;
                (*p).flags = ENTRY_BEEN_USED | ENTRY_IN_USE;
                NEXT_BASE += size;
                SPACE_LEFT -= size;
                return (*p).base as *mut c_void;
            }
            return ptr::null_mut();
        } else if (*p).flags & ENTRY_IN_USE == 0 && size <= (*p).size {
            (*p).flags |= ENTRY_IN_USE;
            return (*p).base as *mut c_void;
        }
        p = p.add(1);
    }
    ptr::null_mut()
}

unsafe fn simple_find_entry(ptr_value: *mut c_void) -> *mut AllocInfo {
    let mut p = ALLOC_TBL;

    for _ in 0..TBL_ENTRIES {
        if (*p).flags & ENTRY_BEEN_USED == 0 {
            break;
        }
        if (*p).flags & ENTRY_IN_USE != 0 && (*p).base == ptr_value as ::core::ffi::c_ulong {
            return p;
        }
        p = p.add(1);
    }
    ptr::null_mut()
}

unsafe extern "C" fn simple_free(ptr_value: *mut c_void) {
    let p = simple_find_entry(ptr_value);
    if !p.is_null() {
        (*p).flags &= !ENTRY_IN_USE;
    }
}

unsafe extern "C" fn simple_realloc(ptr_value: *mut c_void, size: ::core::ffi::c_ulong) -> *mut c_void {
    if size == 0 {
        simple_free(ptr_value);
        return ptr::null_mut();
    }

    if ptr_value.is_null() {
        return simple_malloc(size);
    }

    let p = simple_find_entry(ptr_value);
    if p.is_null() {
        return ptr::null_mut();
    }
    if size <= (*p).size {
        return ptr_value;
    }

    let new_ptr = simple_malloc(size);
    if !new_ptr.is_null() {
        memcpy(new_ptr, ptr_value, (*p).size as usize);
        simple_free(ptr_value);
    }
    new_ptr
}

#[no_mangle]
pub unsafe extern "C" fn simple_alloc_init(
    base: *mut i8,
    mut heap_size: ::core::ffi::c_ulong,
    granularity: ::core::ffi::c_ulong,
    max_allocs: ::core::ffi::c_ulong,
) -> *mut c_void {
    heap_size = align_up(heap_size, granularity);
    ALLOC_MIN = granularity;
    TBL_ENTRIES = max_allocs;

    let tbl_size = TBL_ENTRIES * core::mem::size_of::<AllocInfo>() as ::core::ffi::c_ulong;

    ALLOC_TBL = align_up(base as ::core::ffi::c_ulong, 8) as *mut AllocInfo;
    memset(ALLOC_TBL as *mut c_void, 0, tbl_size as usize);

    let heap_base = align_up(ALLOC_TBL as ::core::ffi::c_ulong + tbl_size, ALLOC_MIN);

    NEXT_BASE = heap_base;
    SPACE_LEFT = heap_size;

    platform_ops.malloc = Some(simple_malloc);
    platform_ops.free = Some(simple_free);
    platform_ops.realloc = Some(simple_realloc);

    (heap_base + heap_size) as *mut c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
