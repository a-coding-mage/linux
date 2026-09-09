/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2021-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* xfile array index type, along with cursor initialization */
pub type xfarray_idx_t = u64;
pub const XFARRAY_NULLIDX: xfarray_idx_t = u64::MAX;
pub const XFARRAY_CURSOR_INIT: xfarray_idx_t = 0;

/* Iterate each index of an xfile array. */
#[macro_export]
macro_rules! foreach_xfarray_idx {
    ($array:expr, $idx:ident) => {
        for $idx in $crate::XFARRAY_CURSOR_INIT..unsafe { $crate::xfarray_length($array) } {
        }
    };
}

#[repr(C)]
pub struct xfarray {
    /* Underlying file that backs the array. */
    pub xfile: *mut xfile,

    /* Number of array elements. */
    pub nr: xfarray_idx_t,

    /* Maximum possible array size. */
    pub max_nr: xfarray_idx_t,

    /* Number of unset slots in the array below @nr. */
    pub unset_slots: u64,

    /* Size of an array element. */
    pub obj_size: usize,

    /* log2 of array element size, if possible. */
    pub obj_size_log: i32,
}

extern "C" {
    pub fn xfarray_create(
        descr: *const core::ffi::c_char,
        required_capacity: u64,
        obj_size: usize,
        arrayp: *mut *mut xfarray,
    ) -> i32;
    pub fn xfarray_destroy(array: *mut xfarray);
    pub fn xfarray_load(array: *mut xfarray, idx: xfarray_idx_t, ptr: *mut core::ffi::c_void) -> i32;
    pub fn xfarray_unset(array: *mut xfarray, idx: xfarray_idx_t) -> i32;
    pub fn xfarray_store(array: *mut xfarray, idx: xfarray_idx_t, ptr: *const core::ffi::c_void) -> i32;
    pub fn xfarray_store_anywhere(array: *mut xfarray, ptr: *const core::ffi::c_void) -> i32;
    pub fn xfarray_element_is_null(array: *mut xfarray, ptr: *const core::ffi::c_void) -> bool;
    pub fn xfarray_truncate(array: *mut xfarray);
    pub fn xfarray_bytes(array: *mut xfarray) -> u64;
    pub fn xfarray_length(array: *mut xfarray) -> u64;
    pub fn xfarray_load_next(array: *mut xfarray, idx: *mut xfarray_idx_t, rec: *mut core::ffi::c_void) -> i32;
    pub fn xfarray_sort(array: *mut xfarray, cmp_fn: xfarray_cmp_fn, flags: u32) -> i32;
}

/* Load an array element, but zero the buffer if there's no data because we
 * haven't stored to that array element yet.
 */
#[inline]
pub unsafe fn xfarray_load_sparse(array: *mut xfarray, idx: u64, rec: *mut core::ffi::c_void) -> i32 {
    let error = xfarray_load(array, idx, rec);
    if error == -ENODATA {
        core::ptr::write_bytes(rec, 0, (*array).obj_size);
        return 0;
    }
    error
}

/* Append an element to the array. */
#[inline]
pub unsafe fn xfarray_append(array: *mut xfarray, ptr: *const core::ffi::c_void) -> i32 {
    xfarray_store(array, (*array).nr, ptr)
}

/* Iterate the non-null elements in a sparse xfarray. */
#[inline]
pub unsafe fn xfarray_iter(array: *mut xfarray, idx: *mut xfarray_idx_t, rec: *mut core::ffi::c_void) -> i32 {
    let ret = xfarray_load_next(array, idx, rec);
    if ret == -ENODATA { return 0; }
    if ret == 0 { return 1; }
    ret
}

pub type xfarray_cmp_fn = cmp_func_t;
pub const XFARRAY_ISORT_SHIFT: u32 = 4;
pub const XFARRAY_ISORT_NR: u32 = 1u32 << XFARRAY_ISORT_SHIFT;
pub const XFARRAY_QSORT_PIVOT_NR: u32 = 9;

#[repr(C)]
pub struct xfarray_sortinfo {
    pub array: *mut xfarray,
    pub cmp_fn: xfarray_cmp_fn,
    pub max_stack_depth: u8,
    pub stack_depth: i8,
    pub max_stack_used: u8,
    pub flags: u32,
    pub relax: xchk_relax,
    pub folio: *mut folio,
    pub first_folio_idx: xfarray_idx_t,
    pub last_folio_idx: xfarray_idx_t,
    #[cfg(debug_assertions)]
    pub loads: u64,
    #[cfg(debug_assertions)]
    pub stores: u64,
    #[cfg(debug_assertions)]
    pub compares: u64,
    #[cfg(debug_assertions)]
    pub heapsorts: u64,
    /* Extra quicksort storage is allocated beyond this structure. */
}

pub const XFARRAY_SORT_KILLABLE: u32 = 1u32 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
