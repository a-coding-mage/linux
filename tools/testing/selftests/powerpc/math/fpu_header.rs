/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2023, Michael Ellerman, IBM Corporation.
 */

unsafe extern "C" {
    fn random() -> ::std::os::raw::c_long;
}

pub unsafe fn randomise_darray(darray: *mut f64, num: ::std::os::raw::c_int) {
    let mut val: ::std::os::raw::c_long;

    let mut i: ::std::os::raw::c_int = 0;
    while i < num {
        val = unsafe { random() };
        if val & 1 != 0 {
            val *= -1;
        }

        if val & 2 != 0 {
            unsafe {
                *darray.offset(i as isize) = 1.0 / val as f64;
            }
        } else {
            unsafe {
                *darray.offset(i as isize) = (val * val) as f64;
            }
        }

        i += 1;
    }
}
