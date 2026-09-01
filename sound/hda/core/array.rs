// SPDX-License-Identifier: GPL-2.0-only
/*
 * generic arrays
 */

use core::ffi::c_void;
use core::ptr;

pub const GFP_KERNEL: gfp_t = 0;

#[allow(non_camel_case_types)]
pub type gfp_t = u32;

#[repr(C)]
pub struct snd_array {
    pub list: *mut c_void,
    pub used: i32,
    pub alloced: i32,
    pub elem_size: i32,
    pub alloc_align: i32,
}

extern "C" {
    fn krealloc(p: *const c_void, new_size: usize, flags: gfp_t) -> *mut c_void;
    fn kfree(p: *const c_void);
    fn snd_BUG_ON(condition: bool) -> bool;
}

#[inline]
unsafe fn snd_array_elem(array: *mut snd_array, idx: i32) -> *mut c_void {
    unsafe {
        ((*array).list as *mut u8)
            .add((idx * (*array).elem_size) as usize)
            as *mut c_void
    }
}

/**
 * snd_array_new - get a new element from the given array
 * @array: the array object
 *
 * Get a new element from the given array.  If it exceeds the
 * pre-allocated array size, re-allocate the array.
 *
 * Returns NULL if allocation failed.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_array_new(array: *mut snd_array) -> *mut c_void {
    unsafe {
        if snd_BUG_ON((*array).elem_size == 0) {
            return ptr::null_mut();
        }
        if (*array).used >= (*array).alloced {
            let num: i32 = (*array).alloced + (*array).alloc_align;
            let oldsize: i32 = (*array).alloced * (*array).elem_size;
            let size: i32 = (num + 1) * (*array).elem_size;
            let nlist: *mut c_void;
            if snd_BUG_ON(num >= 4096) {
                return ptr::null_mut();
            }
            nlist = krealloc((*array).list, size as usize, GFP_KERNEL);
            if nlist.is_null() {
                return ptr::null_mut();
            }
            ptr::write_bytes(
                (nlist as *mut u8).add(oldsize as usize),
                0,
                (size - oldsize) as usize,
            );
            (*array).list = nlist;
            (*array).alloced = num;
        }
        let used = (*array).used;
        (*array).used += 1;
        snd_array_elem(array, used)
    }
}

/* EXPORT_SYMBOL_GPL(snd_array_new); */

/**
 * snd_array_free - free the given array elements
 * @array: the array object
 */
#[no_mangle]
pub unsafe extern "C" fn snd_array_free(array: *mut snd_array) {
    unsafe {
        kfree((*array).list);
        (*array).used = 0;
        (*array).alloced = 0;
        (*array).list = ptr::null_mut();
    }
}

/* EXPORT_SYMBOL_GPL(snd_array_free); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
