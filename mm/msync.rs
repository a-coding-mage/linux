// SPDX-License-Identifier: GPL-2.0
/*
 *	linux/mm/msync.c
 *
 * Copyright (C) 1994-1999  Linus Torvalds
 */

/*
 * The msync() system call.
 */
// Linux kernel headers supplying the following types, constants, and functions
// are dependencies of this translation.

extern "C" {
    static mut current: *mut task_struct;
    fn untagged_addr(addr: usize) -> usize;
    fn offset_in_page(addr: usize) -> usize;
    fn mmap_read_lock(mm: *mut mm_struct);
    fn mmap_read_unlock(mm: *mut mm_struct);
    fn find_vma(mm: *mut mm_struct, addr: usize) -> *mut vm_area_struct;
    fn linear_page_index(vma: *mut vm_area_struct, addr: usize) -> isize;
    fn get_file(file: *mut file);
    fn fput(file: *mut file);
    fn vfs_fsync_range(file: *mut file, start: loff_t, end: loff_t, datasync: i32) -> i32;
}

#[repr(C)]
struct task_struct {
    mm: *mut mm_struct,
}

#[repr(C)]
struct mm_struct;

#[repr(C)]
struct file;

#[repr(C)]
struct vm_area_struct {
    vm_start: usize,
    vm_end: usize,
    vm_flags: usize,
    vm_file: *mut file,
}

type loff_t = i64;

// MS_SYNC syncs the entire file - including mappings.
// MS_ASYNC does not start I/O (it used to, up to 2.5.67).
// Nor does it marks the relevant pages dirty (it used to up to 2.6.17).
// Now it doesn't do anything, since dirty pages are properly tracked.
//
// The application may now run fsync() to write out the dirty pages and wait
// on the writeout and check the result. Or the application may run fadvise()
// against the fd to start async writeout immediately.

// Constants supplied by the kernel headers.
extern "C" {
    static MS_ASYNC: i32;
    static MS_INVALIDATE: i32;
    static MS_SYNC: i32;
    static PAGE_MASK: usize;
    static PAGE_SHIFT: u32;
    static VM_LOCKED: usize;
    static VM_SHARED: usize;
}

#[allow(non_snake_case)]
pub unsafe fn msync(mut start: usize, mut len: usize, flags: i32) -> i32 {
    let mm: *mut mm_struct = (*current).mm;
    let mut vma: *mut vm_area_struct;
    let mut unmapped_error: i32 = 0;
    let mut error: i32 = -EINVAL;

    start = untagged_addr(start);

    if (flags & !(MS_ASYNC | MS_INVALIDATE | MS_SYNC)) != 0 {
        return if error != 0 { error } else { unmapped_error };
    }
    if offset_in_page(start) != 0 {
        return if error != 0 { error } else { unmapped_error };
    }
    if (flags & MS_ASYNC) != 0 && (flags & MS_SYNC) != 0 {
        return if error != 0 { error } else { unmapped_error };
    }
    error = -ENOMEM;
    len = (len + !PAGE_MASK) & PAGE_MASK;
    let end = start + len;
    if end < start {
        return if error != 0 { error } else { unmapped_error };
    }
    error = 0;
    if end == start {
        return if error != 0 { error } else { unmapped_error };
    }

    mmap_read_lock(mm);
    vma = find_vma(mm, start);
    loop {
        let file: *mut file;
        let fstart: loff_t;
        let fend: loff_t;

        error = -ENOMEM;
        if vma.is_null() {
            break;
        }
        if start < (*vma).vm_start {
            if flags == MS_ASYNC {
                break;
            }
            start = (*vma).vm_start;
            if start >= end {
                break;
            }
            unmapped_error = -ENOMEM;
        }
        if (flags & MS_INVALIDATE) != 0 && ((*vma).vm_flags & VM_LOCKED) != 0 {
            error = -EBUSY;
            break;
        }
        file = (*vma).vm_file;
        fstart = (linear_page_index(vma, start) << PAGE_SHIFT) as loff_t;
        let vma_end = (*vma).vm_end;
        let covered_end = if end < vma_end { end } else { vma_end };
        fend = fstart + (covered_end - start) as loff_t - 1;
        start = vma_end;
        if (flags & MS_SYNC) != 0 && !file.is_null() && ((*vma).vm_flags & VM_SHARED) != 0 {
            get_file(file);
            mmap_read_unlock(mm);
            error = vfs_fsync_range(file, fstart, fend, 1);
            fput(file);
            if error != 0 || start >= end {
                return if error != 0 { error } else { unmapped_error };
            }
            mmap_read_lock(mm);
            vma = find_vma(mm, start);
        } else {
            if start >= end {
                error = 0;
                break;
            }
            vma = find_vma(mm, (*vma).vm_end);
        }
    }
    mmap_read_unlock(mm);
    if error != 0 { error } else { unmapped_error }
}

// Error constants supplied by the kernel headers.
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EBUSY: i32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
