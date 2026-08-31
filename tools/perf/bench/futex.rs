// SPDX-License-Identifier: GPL-2.0
//
// C dependencies removed from executable Rust:
// <err.h>, <errno.h>, <stdio.h>, <stdlib.h>, <sys/prctl.h>, and "futex.h".

use std::os::raw::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct bench_futex_parameters {
    pub nbuckets: c_int,
}

// #ifndef PR_FUTEX_HASH
const PR_FUTEX_HASH: c_int = 78;
const PR_FUTEX_HASH_SET_SLOTS: c_ulong = 1;
const PR_FUTEX_HASH_GET_SLOTS: c_ulong = 2;
// #endif // PR_FUTEX_HASH

const EXIT_FAILURE: c_int = 1;

unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn free(ptr: *mut std::ffi::c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn futex_set_nbuckets_param(params: *mut bench_futex_parameters) {
    let mut ret: c_int;

    if unsafe { (*params).nbuckets } < 0 {
        return;
    }

    ret = unsafe {
        prctl(
            PR_FUTEX_HASH,
            PR_FUTEX_HASH_SET_SLOTS,
            (*params).nbuckets as c_ulong,
            0 as c_ulong,
        )
    };
    if ret != 0 {
        unsafe {
            printf(
                b"Requesting %d hash buckets failed: %d/%m\n\0".as_ptr() as *const c_char,
                (*params).nbuckets,
                ret,
            );
            err(
                EXIT_FAILURE,
                b"prctl(PR_FUTEX_HASH)\0".as_ptr() as *const c_char,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn futex_print_nbuckets(params: *mut bench_futex_parameters) {
    let mut futex_hash_mode: *mut c_char = std::ptr::null_mut();
    let mut ret: c_int;

    ret = unsafe { prctl(PR_FUTEX_HASH, PR_FUTEX_HASH_GET_SLOTS) };
    if unsafe { (*params).nbuckets } >= 0 {
        if ret != unsafe { (*params).nbuckets } {
            if ret < 0 {
                unsafe {
                    printf(b"Can't query number of buckets: %m\n\0".as_ptr() as *const c_char);
                    err(
                        EXIT_FAILURE,
                        b"prctl(PR_FUTEX_HASH)\0".as_ptr() as *const c_char,
                    );
                }
            }
            unsafe {
                printf(
                    b"Requested number of hash buckets does not currently used.\n\0".as_ptr()
                        as *const c_char,
                );
                printf(
                    b"Requested: %d in usage: %d\n\0".as_ptr() as *const c_char,
                    (*params).nbuckets,
                    ret,
                );
                err(
                    EXIT_FAILURE,
                    b"prctl(PR_FUTEX_HASH)\0".as_ptr() as *const c_char,
                );
            }
        }
        if unsafe { (*params).nbuckets } == 0 {
            ret = unsafe {
                asprintf(
                    &mut futex_hash_mode,
                    b"Futex hashing: global hash\0".as_ptr() as *const c_char,
                )
            };
        } else {
            ret = unsafe {
                asprintf(
                    &mut futex_hash_mode,
                    b"Futex hashing: %d hash buckets\0".as_ptr() as *const c_char,
                    (*params).nbuckets,
                )
            };
        }
    } else {
        if ret <= 0 {
            ret = unsafe {
                asprintf(
                    &mut futex_hash_mode,
                    b"Futex hashing: global hash\0".as_ptr() as *const c_char,
                )
            };
        } else {
            ret = unsafe {
                asprintf(
                    &mut futex_hash_mode,
                    b"Futex hashing: auto resized to %d buckets\0".as_ptr() as *const c_char,
                    ret,
                )
            };
        }
    }
    if ret < 0 {
        unsafe {
            err(
                EXIT_FAILURE,
                b"ENOMEM, futex_hash_mode\0".as_ptr() as *const c_char,
            );
        }
    }
    unsafe {
        printf(b"%s\n\0".as_ptr() as *const c_char, futex_hash_mode);
        free(futex_hash_mode as *mut std::ffi::c_void);
    }
}
