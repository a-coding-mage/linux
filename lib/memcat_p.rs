// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;
use core::mem::size_of;

// Supplied by the kernel allocator interface included by the original source.
extern "C" {
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut *mut c_void;
    static GFP_KERNEL: u32;
}

/*
 * Merge two NULL-terminated pointer arrays into a newly allocated
 * array, which is also NULL-terminated. Nomenclature is inspired by
 * memset_p() and memcat() found elsewhere in the kernel source tree.
 */
#[no_mangle]
pub unsafe extern "C" fn __memcat_p(a: *mut *mut c_void, b: *mut *mut c_void) -> *mut *mut c_void {
    let mut p = a;
    let mut nr: isize;

    /* count the elements in both arrays */
    nr = 0;
    p = a;
    while !(*p).is_null() {
        nr += 1;
        p = p.add(1);
    }
    while !(*p).is_null() {
        nr += 1;
        p = p.add(1);
    }
    /* one for the NULL-terminator */
    nr += 1;

    let new = kmalloc_array(nr as usize, size_of::<*mut c_void>(), GFP_KERNEL);
    if new.is_null() {
        return core::ptr::null_mut();
    }

    /* nr -> last index; p points to NULL in b[] */
    nr -= 1;
    while nr >= 0 {
        *new.add(nr as usize) = *p;
        nr -= 1;
        p = if p == b {
            a.add(nr as usize)
        } else {
            p.sub(1)
        };
    }

    new
}

// EXPORT_SYMBOL_GPL(__memcat_p);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
