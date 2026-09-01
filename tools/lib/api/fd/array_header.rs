/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from lib/api/fd/array.h. */

use core::ffi::{c_int, c_short, c_uint, c_void};

/* From <stdio.h>. */
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

/* Forward declaration: struct pollfd. */
#[repr(C)]
pub struct pollfd {
    _private: [u8; 0],
}

/**
 * struct fdarray: Array of file descriptors
 *
 * @priv: Per array entry priv area, users should access just its contents,
 *        not set it to anything, as it is kept in synch with @entries, being
 *        realloc'ed, * for instance, in fdarray__{grow,filter}.
 *
 *        I.e. using 'fda->priv[N].idx = * value' where N < fda->nr is ok,
 *        but doing 'fda->priv = malloc(M)' is not allowed.
 */
#[repr(C)]
pub struct fdarray {
    pub nr: c_int,
    pub nr_alloc: c_int,
    pub nr_autogrow: c_int,
    pub entries: *mut pollfd,
    pub priv_: *mut priv_,
}

#[repr(C)]
pub struct priv_ {
    pub u: priv__bindgen_ty_1,
    pub flags: c_uint,
}

#[repr(C)]
pub union priv__bindgen_ty_1 {
    pub idx: c_int,
    pub ptr: *mut c_void,
}

pub type fdarray_flags = c_uint;

pub const fdarray_flag__default: fdarray_flags = 0x00000000;
pub const fdarray_flag__nonfilterable: fdarray_flags = 0x00000001;
pub const fdarray_flag__non_perf_event: fdarray_flags = 0x00000002;

unsafe extern "C" {
    pub fn fdarray__init(fda: *mut fdarray, nr_autogrow: c_int);
    pub fn fdarray__exit(fda: *mut fdarray);

    pub fn fdarray__new(nr_alloc: c_int, nr_autogrow: c_int) -> *mut fdarray;
    pub fn fdarray__delete(fda: *mut fdarray);

    pub fn fdarray__add(
        fda: *mut fdarray,
        fd: c_int,
        revents: c_short,
        flags: fdarray_flags,
    ) -> c_int;
    pub fn fdarray__dup_entry_from(fda: *mut fdarray, pos: c_int, from: *mut fdarray) -> c_int;
    pub fn fdarray__poll(fda: *mut fdarray, timeout: c_int) -> c_int;
    pub fn fdarray__filter(
        fda: *mut fdarray,
        revents: c_short,
        entry_destructor: Option<unsafe extern "C" fn(fda: *mut fdarray, fd: c_int, arg: *mut c_void)>,
        arg: *mut c_void,
    ) -> c_int;
    pub fn fdarray__grow(fda: *mut fdarray, extra: c_int) -> c_int;
    pub fn fdarray__fprintf(fda: *mut fdarray, fp: *mut FILE) -> c_int;
}

#[inline]
pub unsafe fn fdarray__available_entries(fda: *mut fdarray) -> c_int {
    unsafe { (*fda).nr_alloc - (*fda).nr }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
