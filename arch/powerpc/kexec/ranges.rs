// SPDX-License-Identifier: GPL-2.0-only
/*
 * powerpc code to implement the kexec_file_load syscall
 *
 * Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
 * Copyright (C) 2004  IBM Corp.
 * Copyright (C) 2004,2005  Milton D Miller II, IBM Corporation
 * Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
 * Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
 * Copyright (C) 2020  IBM Corporation
 *
 * Based on kexec-tools' kexec-ppc64.c, fs2dt.c.
 * Heavily modified for the kernel by
 * Hari Bathini, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct range {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct crash_mem {
    pub nr_ranges: u32,
    pub max_nr_ranges: u32,
    pub ranges: [range; 0],
}

extern "C" {
    fn krealloc(ptr: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn sort(
        base: *mut core::ffi::c_void,
        num: usize,
        size: usize,
        cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
        swap: *mut core::ffi::c_void,
    );
}

// Supplied by the kernel build configuration.
extern "C" {
    static MEM_RANGE_CHUNK_SZ: usize;
    static GFP_KERNEL: u32;
}

#[inline]
unsafe fn get_max_nr_ranges(size: usize) -> u32 {
    ((size - core::mem::size_of::<crash_mem>()) / core::mem::size_of::<range>()) as u32
}

#[inline]
unsafe fn get_mem_rngs_size(mem_rngs: *mut crash_mem) -> usize {
    if mem_rngs.is_null() {
        return 0;
    }

    let size = core::mem::size_of::<crash_mem>()
        + ((*mem_rngs).max_nr_ranges as usize * core::mem::size_of::<range>());
    let chunk = MEM_RANGE_CHUNK_SZ;
    (size + chunk - 1) & !(chunk - 1)
}

unsafe fn __add_mem_range(mem_ranges: *mut *mut crash_mem, base: u64, size: u64) -> i32 {
    let mut mem_rngs = *mem_ranges;

    if mem_rngs.is_null() || (*mem_rngs).nr_ranges == (*mem_rngs).max_nr_ranges {
        mem_rngs = realloc_mem_ranges(mem_ranges);
        if mem_rngs.is_null() {
            return -12;
        }
    }

    let ranges = (*mem_rngs).ranges.as_mut_ptr();
    (*ranges.add((*mem_rngs).nr_ranges as usize)).start = base;
    (*ranges.add((*mem_rngs).nr_ranges as usize)).end = base + size - 1;
    (*mem_rngs).nr_ranges += 1;
    0
}

unsafe fn __merge_memory_ranges(mem_rngs: *mut crash_mem) {
    if mem_rngs.is_null() || (*mem_rngs).nr_ranges <= 1 {
        return;
    }

    let ranges = (*mem_rngs).ranges.as_mut_ptr();
    let mut idx = 0usize;
    for i in 1..(*mem_rngs).nr_ranges as usize {
        if (*ranges.add(i)).start <= (*ranges.add(idx)).end + 1 {
            (*ranges.add(idx)).end = core::cmp::max((*ranges.add(idx)).end, (*ranges.add(i)).end);
        } else {
            idx += 1;
            *ranges.add(idx) = *ranges.add(i);
        }
    }
    (*mem_rngs).nr_ranges = (idx + 1) as u32;
}

unsafe extern "C" fn rngcmp(x: *const core::ffi::c_void, y: *const core::ffi::c_void) -> i32 {
    let x = &*(x as *const range);
    let y = &*(y as *const range);
    if x.start > y.start { 1 } else if x.start < y.start { -1 } else { 0 }
}

pub unsafe fn sort_memory_ranges(mem_rngs: *mut crash_mem, merge: bool) {
    if mem_rngs.is_null() {
        return;
    }
    sort(
        (*mem_rngs).ranges.as_mut_ptr() as *mut core::ffi::c_void,
        (*mem_rngs).nr_ranges as usize,
        core::mem::size_of::<range>(),
        rngcmp,
        core::ptr::null_mut(),
    );
    if merge {
        __merge_memory_ranges(mem_rngs);
    }
}

pub unsafe fn realloc_mem_ranges(mem_ranges: *mut *mut crash_mem) -> *mut crash_mem {
    let mut mem_rngs = *mem_ranges;
    let mut size = get_mem_rngs_size(mem_rngs);
    let nr_ranges = if mem_rngs.is_null() { 0 } else { (*mem_rngs).nr_ranges };
    size += MEM_RANGE_CHUNK_SZ;
    mem_rngs = krealloc(*mem_ranges as *mut core::ffi::c_void, size, GFP_KERNEL) as *mut crash_mem;
    if mem_rngs.is_null() {
        kfree(*mem_ranges as *mut core::ffi::c_void);
        *mem_ranges = core::ptr::null_mut();
        return core::ptr::null_mut();
    }
    (*mem_rngs).nr_ranges = nr_ranges;
    (*mem_rngs).max_nr_ranges = get_max_nr_ranges(size);
    *mem_ranges = mem_rngs;
    mem_rngs
}

pub unsafe fn add_mem_range(mem_ranges: *mut *mut crash_mem, base: u64, size: u64) -> i32 {
    let mem_rngs = *mem_ranges;
    if size == 0 { return 0; }
    let end = base + size - 1;
    if mem_rngs.is_null() || (*mem_rngs).nr_ranges == 0 {
        return __add_mem_range(mem_ranges, base, size);
    }
    for i in 0..(*mem_rngs).nr_ranges as usize {
        let r = &mut *(*mem_rngs).ranges.as_mut_ptr().add(i);
        let mstart = r.start;
        let mend = r.end;
        if base < mend && end > mstart {
            if base < mstart { r.start = base; }
            if end > mend { r.end = end; }
            return 0;
        }
    }
    __add_mem_range(mem_ranges, base, size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
