/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * auxv definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: ../nolibc.h */

/* C header guard _NOLIBC_SYS_AUXV_H omitted in Rust. */

/* When NOLIBC_NO_RUNTIME is not defined: */
/* C dependency: ../crt.h */

unsafe extern "C" {
    static mut _auxv: *const core::ffi::c_ulong;
}

#[allow(dead_code)]
pub unsafe extern "C" fn getauxval(type_: core::ffi::c_ulong) -> core::ffi::c_ulong {
    let mut auxv: *const core::ffi::c_ulong = unsafe { _auxv };
    let ret: core::ffi::c_ulong;

    if auxv.is_null() {
        return 0;
    }

    loop {
        if unsafe { *auxv.add(0) } == 0 && unsafe { *auxv.add(1) } == 0 {
            ret = 0;
            break;
        }

        if unsafe { *auxv.add(0) } == type_ {
            ret = unsafe { *auxv.add(1) };
            break;
        }

        auxv = unsafe { auxv.add(2) };
    }

    ret
}
