/*
 * Copyright (C) 2013 Altera Corporation
 * Copyright (C) 2011-2012 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel are intentionally external.

use core::ffi::c_int;

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: usize,
    pub vm_end: usize,
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
}

extern "C" {
    pub static mut current: *mut task_struct;

    pub fn mmap_read_lock_killable(mm: *mut mm_struct) -> c_int;
    pub fn mmap_read_unlock(mm: *mut mm_struct);
    pub fn find_vma(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    pub fn flush_cache_range(vma: *mut vm_area_struct, start: usize, end: usize);
}

// Supplied by the kernel's architecture and errno definitions.
const PAGE_SIZE: c_int = 4096;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EINTR: c_int = 4;

/* sys_cacheflush -- flush the processor cache. */
pub unsafe fn sys_cacheflush(addr: usize, len: usize, op: u32) -> c_int {
    let vma: *mut vm_area_struct;
    let mm: *mut mm_struct = (*current).mm;

    if len == 0 {
        return 0;
    }

    /* We only support op 0 now, return error if op is non-zero.*/
    if op != 0 {
        return -EINVAL;
    }

    /* Check for overflow */
    if addr.wrapping_add(len) < addr {
        return -EFAULT;
    }

    if mmap_read_lock_killable(mm) != 0 {
        return -EINTR;
    }

    /*
     * Verify that the specified address region actually belongs
     * to this process.
     */
    vma = find_vma(mm, addr);
    if vma.is_null()
        || addr < (*vma).vm_start
        || addr.wrapping_add(len) > (*vma).vm_end
    {
        mmap_read_unlock(mm);
        return -EFAULT;
    }

    flush_cache_range(vma, addr, addr.wrapping_add(len));

    mmap_read_unlock(mm);
    0
}

pub unsafe fn sys_getpagesize() -> c_int {
    PAGE_SIZE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
