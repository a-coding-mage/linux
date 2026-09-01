/* SPDX-License-Identifier: LGPL-2.1-only OR MIT */
/*
 * rseq-generic-thread-pointer.h
 *
 * (C) Copyright 2021 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/* Use gcc builtin thread pointer. */
unsafe extern "C" {
    fn __builtin_thread_pointer() -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn rseq_thread_pointer() -> *mut core::ffi::c_void {
    unsafe { __builtin_thread_pointer() }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
