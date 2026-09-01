/* SPDX-License-Identifier: GPL-2.0 */

use core::mem;
use std::os::raw::{c_char, c_int, c_long};

pub type size_t = usize;
pub type ssize_t = isize;

unsafe extern "C" {
    pub fn libbpf_num_possible_cpus() -> c_int;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn exit(status: c_int) -> !;
    pub fn syscall(number: c_long, ...) -> c_long;
}

pub unsafe fn bpf_num_possible_cpus() -> c_uint {
    let possible_cpus: c_int = unsafe { libbpf_num_possible_cpus() };

    if possible_cpus < 0 {
        unsafe {
            printf(
                b"Failed to get # of possible cpus: '%s'!\n\0".as_ptr() as *const c_char,
                strerror(-possible_cpus),
            );
            exit(1);
        }
    }
    possible_cpus as c_uint
}

/*
 * Simplified strscpy() implementation. The kernel one is in lib/string.c
 */
pub unsafe fn sized_strscpy(dest: *mut c_char, src: *const c_char, mut count: size_t) -> ssize_t {
    let mut res: c_long = 0;

    if count == 0 {
        return -E2BIG;
    }

    while count > 1 {
        let c: c_char;

        c = unsafe { *src.offset(res as isize) };
        unsafe {
            *dest.offset(res as isize) = c;
        }
        if c == 0 {
            return res as ssize_t;
        }
        res += 1;
        count -= 1;
    }

    /* Force NUL-termination. */
    unsafe {
        *dest.offset(res as isize) = b'\0' as c_char;
    }

    /* Return E2BIG if the source didn't stop */
    if unsafe { *src.offset(res as isize) } != 0 {
        -E2BIG
    } else {
        res as ssize_t
    }
}

pub unsafe fn __strscpy0<const N: usize>(dst: *mut c_char, src: *const c_char) -> ssize_t {
    unsafe { sized_strscpy(dst, src, N) }
}

pub unsafe fn __strscpy1(dst: *mut c_char, src: *const c_char, size: size_t) -> ssize_t {
    unsafe { sized_strscpy(dst, src, size) }
}

/*
 * C macro:
 *   strscpy(dst, src, ...)
 * dispatches to __strscpy0(dst, src) using sizeof(dst), or to
 * __strscpy1(dst, src, size) when an explicit size is supplied.
 */

/*
 * C macro:
 *   #define __bpf_percpu_val_align __attribute__((__aligned__(8)))
 *
 * Use #[repr(C, align(8))] on the per-cpu value wrapper in Rust.
 */
#[repr(C, align(8))]
pub struct bpf_percpu_val<T> {
    pub v: T,
    /* padding */
}

/*
 * C macro:
 *   BPF_DECLARE_PERCPU(type, name)
 * declares an aligned array sized by bpf_num_possible_cpus().
 *
 * Rust cannot declare a local variable-length array through a macro-equivalent
 * item here; use bpf_percpu_val<T> for the element layout.
 */

pub unsafe fn bpf_percpu<T>(name: *mut bpf_percpu_val<T>, cpu: size_t) -> *mut T {
    unsafe { &mut (*name.add(cpu)).v as *mut T }
}

pub const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> size_t {
    N
}

pub const fn sizeof_field<T, F>(field: fn(*const T) -> *const F) -> size_t {
    let base = core::ptr::null::<T>();
    let ptr = field(base);
    let _ = ptr;
    mem::size_of::<F>()
}

/*
 * C macro:
 *   offsetofend(TYPE, MEMBER) = offsetof(TYPE, MEMBER) + sizeof_field(TYPE, MEMBER)
 *
 * Rust needs the member offset from an external offset_of-style facility.
 */
pub const fn offsetofend(offset: size_t, field_size: size_t) -> size_t {
    offset + field_size
}

/*
 * Availability of gettid across glibc versions is hit-and-miss, therefore
 * fallback to syscall in this macro and use it everywhere.
 */
/* Requires SYS_gettid from <syscall.h>. */
pub unsafe fn sys_gettid() -> c_long {
    unsafe { syscall(SYS_gettid) }
}

/* and poison usage to ensure it does not creep back in. */
/* C used: #pragma GCC poison gettid */

pub const ENOTSUPP: c_int = 524;

pub const E2BIG: ssize_t = 7;

pub type c_uint = u32;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
