// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/slab.h, linux/export.h.

use core::ffi::c_void;
use core::ptr::null_mut;
use kernel::ffi::c_int;

extern "C" {
    fn lupos_memcat_p_alloc(nr: c_int) -> *mut *mut c_void;
}

/*
 * Merge two NULL-terminated pointer arrays into a newly allocated
 * array, which is also NULL-terminated. Nomenclature is inspired by
 * memset_p() and memcat() found elsewhere in the kernel source tree.
 */
/// Returns a new NULL-terminated array holding `a` followed by `b`.
///
/// # Safety
/// `a` and `b` point to valid NULL-terminated pointer arrays.
#[no_mangle]
pub unsafe extern "C" fn __memcat_p(a: *mut *mut c_void, b: *mut *mut c_void) -> *mut *mut c_void {
    let mut nr: c_int = 0;
    let mut p: *mut *mut c_void;

    // SAFETY: both arrays are valid up to and including their NULL
    // terminators, which bound every read below.
    unsafe {
        /* count the elements in both arrays */
        p = a;
        // The int counter wraps as under the kernel's -fno-strict-overflow.
        while !(*p).is_null() {
            nr = nr.wrapping_add(1);
            p = p.add(1);
        }
        p = b;
        while !(*p).is_null() {
            nr = nr.wrapping_add(1);
            p = p.add(1);
        }
        /* one for the NULL-terminator */
        nr = nr.wrapping_add(1);

        let new = lupos_memcat_p_alloc(nr);
        if new.is_null() {
            return null_mut();
        }

        /* nr -> last index; p points to NULL in b[] */
        nr = nr.wrapping_sub(1);
        while nr >= 0 {
            *new.add(nr as usize) = *p;
            nr = nr.wrapping_sub(1);
            // C forms &a[nr] even after the final element (nr == -1), so
            // keep that pointer step free of in-bounds requirements.
            p = if p == b { a.wrapping_offset(nr as isize) } else { p.wrapping_sub(1) };
        }

        new
    }
}
// EXPORT_SYMBOL_GPL(__memcat_p);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
