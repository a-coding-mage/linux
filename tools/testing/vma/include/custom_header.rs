/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * Contains declarations that exist in the kernel which have been CUSTOMISED for
 * testing purposes to faciliate userland VMA testing.
 */

use core::ffi::{c_int, c_ulong, c_void};

/*
 * C conditional:
 *   #ifdef CONFIG_MMU
 * declares mmap_min_addr and dac_mmap_min_addr as extern globals.
 *   #else
 * defines both as 0UL.
 */
unsafe extern "C" {
    pub static mut mmap_min_addr: c_ulong;
    pub static mut dac_mmap_min_addr: c_ulong;

    pub fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn refcount_set(r: *mut refcount_t, n: c_int);
}

pub const TASK_SIZE: c_ulong = ((1 as c_ulong) << 47).wrapping_sub(PAGE_SIZE as c_ulong);

/*
 * The shared stubs do not implement this, it amounts to an fprintf(STDERR,...)
 * either way :)
 */
pub use pr_err as pr_warn_once;

#[repr(C)]
pub struct anon_vma {
    pub root: *mut anon_vma,
    pub rb_root: rb_root_cached,

    /* Test fields. */
    pub was_cloned: bool,
    pub was_unlinked: bool,
}

#[inline]
pub unsafe fn unlink_anon_vmas(vma: *mut vm_area_struct) {
    /* For testing purposes, indicate that the anon_vma was unlinked. */
    unsafe {
        (*(*vma).anon_vma).was_unlinked = true;
    }
}

#[inline]
pub unsafe fn vma_start_write(vma: *mut vm_area_struct) {
    /* Used to indicate to tests that a write operation has begun. */
    unsafe {
        (*vma).vm_lock_seq += 1;
    }
}

#[inline]
#[must_use]
pub unsafe fn vma_start_write_killable(vma: *mut vm_area_struct) -> c_int {
    /* Used to indicate to tests that a write operation has begun. */
    unsafe {
        (*vma).vm_lock_seq += 1;
    }
    0
}

#[inline]
pub unsafe fn anon_vma_clone(
    dst: *mut vm_area_struct,
    src: *mut vm_area_struct,
    operation: vma_operation,
) -> c_int {
    let _ = operation;

    /* For testing purposes. We indicate that an anon_vma has been cloned. */
    unsafe {
        if !(*src).anon_vma.is_null() {
            (*dst).anon_vma = (*src).anon_vma;
            (*(*dst).anon_vma).was_cloned = true;
        }
    }

    0
}

#[inline]
pub unsafe fn __anon_vma_prepare(vma: *mut vm_area_struct) -> c_int {
    let anon_vma = unsafe { calloc(1, core::mem::size_of::<anon_vma>()) as *mut anon_vma };

    if anon_vma.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*anon_vma).root = anon_vma;
        (*vma).anon_vma = anon_vma;
    }

    0
}

#[inline]
pub unsafe fn anon_vma_prepare(vma: *mut vm_area_struct) -> c_int {
    unsafe {
        if likely((*vma).anon_vma as c_ulong) != 0 {
            return 0;
        }
    }

    unsafe { __anon_vma_prepare(vma) }
}

#[inline]
pub unsafe fn vma_lock_init(vma: *mut vm_area_struct, reset_refcnt: bool) {
    unsafe {
        if reset_refcnt {
            refcount_set(&mut (*vma).vm_refcnt, 0);
        }
    }
}

#[inline]
pub unsafe fn vma_kernel_pagesize(vma: *mut vm_area_struct) -> c_ulong {
    let _ = vma;
    PAGE_SIZE as c_ulong
}
