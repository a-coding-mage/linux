/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const VEC_MAX: c_int = 128;
pub const VSX_MAX: c_int = 32;
pub const VMX_MAX: c_int = 32;

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;

    static TEST_FAIL: c_int;
    static TEST_PASS: c_int;

    pub fn loadvsx(p: *mut c_void, tmp: c_int);
    pub fn storevsx(p: *mut c_void, tmp: c_int);
}

/*
 * unsigned long vsx[32]
 * unsigned long load[128]
 */
pub unsafe extern "C" fn validate_vsx(vsx: *mut c_ulong, load: *mut c_ulong) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < VSX_MAX {
        unsafe {
            if *vsx.add(i as usize) != *load.add((2 * i + 1) as usize) {
                printf(
                    c"vsx[%d]: %lx load[%d] %lx\n".as_ptr(),
                    i,
                    *vsx.add(i as usize),
                    2 * i + 1,
                    *load.add((2 * i + 1) as usize),
                );
                return TEST_FAIL;
            }
        }
        i += 1;
    }
    unsafe { TEST_PASS }
}

/*
 * unsigned long vmx[32][2]
 * unsigned long load[128]
 */
pub unsafe extern "C" fn validate_vmx(vmx: *mut [c_ulong; 2], load: *mut c_ulong) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < VMX_MAX {
        #[cfg(target_endian = "big")]
        unsafe {
            if (*vmx.add(i as usize))[0] != *load.add((64 + 2 * i) as usize)
                || (*vmx.add(i as usize))[1] != *load.add((65 + 2 * i) as usize)
            {
                printf(
                    c"vmx[%d][0]: %lx load[%d] %lx\n".as_ptr(),
                    i,
                    (*vmx.add(i as usize))[0],
                    64 + 2 * i,
                    *load.add((64 + 2 * i) as usize),
                );
                printf(
                    c"vmx[%d][1]: %lx load[%d] %lx\n".as_ptr(),
                    i,
                    (*vmx.add(i as usize))[1],
                    65 + 2 * i,
                    *load.add((65 + 2 * i) as usize),
                );
                return TEST_FAIL;
            }
        }

        #[cfg(not(target_endian = "big"))]
        unsafe {
            /*
             * In LE each value pair is stored in an
             * alternate manner.
             */
            if (*vmx.add(i as usize))[0] != *load.add((65 + 2 * i) as usize)
                || (*vmx.add(i as usize))[1] != *load.add((64 + 2 * i) as usize)
            {
                printf(
                    c"vmx[%d][0]: %lx load[%d] %lx\n".as_ptr(),
                    i,
                    (*vmx.add(i as usize))[0],
                    65 + 2 * i,
                    *load.add((65 + 2 * i) as usize),
                );
                printf(
                    c"vmx[%d][1]: %lx load[%d] %lx\n".as_ptr(),
                    i,
                    (*vmx.add(i as usize))[1],
                    64 + 2 * i,
                    *load.add((64 + 2 * i) as usize),
                );
                return TEST_FAIL;
            }
        }

        i += 1;
    }
    unsafe { TEST_PASS }
}

/*
 * unsigned long store[128]
 * unsigned long load[128]
 */
pub unsafe extern "C" fn compare_vsx_vmx(store: *mut c_ulong, load: *mut c_ulong) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < VSX_MAX {
        unsafe {
            if *store.add((1 + 2 * i) as usize) != *load.add((1 + 2 * i) as usize) {
                printf(
                    c"store[%d]: %lx load[%d] %lx\n".as_ptr(),
                    1 + 2 * i,
                    *store.add(i as usize),
                    1 + 2 * i,
                    *load.add(i as usize),
                );
                return TEST_FAIL;
            }
        }
        i += 1;
    }

    #[cfg(target_endian = "big")]
    {
        i = 64;
        while i < VEC_MAX {
            unsafe {
                if *store.add(i as usize) != *load.add(i as usize) {
                    printf(
                        c"store[%d]: %lx load[%d] %lx\n".as_ptr(),
                        i,
                        *store.add(i as usize),
                        i,
                        *load.add(i as usize),
                    );
                    return TEST_FAIL;
                }
            }
            i += 1;
        }
    }

    #[cfg(not(target_endian = "big"))]
    {
        /* In LE each value pair is stored in an alternate manner */
        i = 64;
        while i < VEC_MAX {
            unsafe {
                if !(i % 2 != 0) && *store.add(i as usize) != *load.add((i + 1) as usize) {
                    printf(
                        c"store[%d]: %lx load[%d] %lx\n".as_ptr(),
                        i,
                        *store.add(i as usize),
                        i + 1,
                        *load.add((i + 1) as usize),
                    );
                    return TEST_FAIL;
                }
                if i % 2 != 0 && *store.add(i as usize) != *load.add((i - 1) as usize) {
                    printf(
                        c"here store[%d]: %lx load[%d] %lx\n".as_ptr(),
                        i,
                        *store.add(i as usize),
                        i - 1,
                        *load.add((i - 1) as usize),
                    );
                    return TEST_FAIL;
                }
            }
            i += 1;
        }
    }

    unsafe { TEST_PASS }
}

pub unsafe extern "C" fn load_vsx_vmx(
    load: *mut c_ulong,
    vsx: *mut c_ulong,
    vmx: *mut [c_ulong; 2],
) {
    let mut i: c_int;

    i = 0;
    while i < VSX_MAX {
        unsafe {
            *vsx.add(i as usize) = *load.add((1 + 2 * i) as usize);
        }
        i += 1;
    }

    i = 0;
    while i < VMX_MAX {
        unsafe {
            (*vmx.add(i as usize))[0] = *load.add((64 + 2 * i) as usize);
            (*vmx.add(i as usize))[1] = *load.add((65 + 2 * i) as usize);
        }
        i += 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
