// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2018-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Dependency supplied by the surrounding kernel/Rust translation. */

/* u64 bitmap */

#[repr(C)]
pub struct xbitmap64 {
    pub xb_root: rb_root_cached,
}

extern "C" {
    pub fn xbitmap64_init(bitmap: *mut xbitmap64);
    pub fn xbitmap64_destroy(bitmap: *mut xbitmap64);

    pub fn xbitmap64_clear(bitmap: *mut xbitmap64, start: u64, len: u64) -> i32;
    pub fn xbitmap64_set(bitmap: *mut xbitmap64, start: u64, len: u64) -> i32;
    pub fn xbitmap64_disunion(bitmap: *mut xbitmap64, sub: *mut xbitmap64) -> i32;
    pub fn xbitmap64_hweight(bitmap: *mut xbitmap64) -> u64;

    /*
     * Return codes for the bitmap iterator functions are 0 to continue iterating,
     * and non-zero to stop iterating.  Any non-zero value will be passed up to the
     * iteration caller.  The special value -ECANCELED can be used to stop
     * iteration, because neither bitmap iterator ever generates that error code on
     * its own.  Callers must not modify the bitmap while walking it.
     */
    pub fn xbitmap64_walk(
        bitmap: *mut xbitmap64,
        fn_: Option<unsafe extern "C" fn(start: u64, len: u64, priv_: *mut core::ffi::c_void) -> i32>,
        priv_: *mut core::ffi::c_void,
    ) -> i32;

    pub fn xbitmap64_empty(bitmap: *mut xbitmap64) -> bool;
    pub fn xbitmap64_test(bitmap: *mut xbitmap64, start: u64, len: *mut u64) -> bool;
}

/* u32 bitmap */

#[repr(C)]
pub struct xbitmap32 {
    pub xb_root: rb_root_cached,
}

extern "C" {
    pub fn xbitmap32_init(bitmap: *mut xbitmap32);
    pub fn xbitmap32_destroy(bitmap: *mut xbitmap32);

    pub fn xbitmap32_clear(bitmap: *mut xbitmap32, start: u32, len: u32) -> i32;
    pub fn xbitmap32_set(bitmap: *mut xbitmap32, start: u32, len: u32) -> i32;
    pub fn xbitmap32_disunion(bitmap: *mut xbitmap32, sub: *mut xbitmap32) -> i32;
    pub fn xbitmap32_hweight(bitmap: *mut xbitmap32) -> u32;

    /*
     * Return codes for the bitmap iterator functions are 0 to continue iterating,
     * and non-zero to stop iterating.  Any non-zero value will be passed up to the
     * iteration caller.  The special value -ECANCELED can be used to stop
     * iteration, because neither bitmap iterator ever generates that error code on
     * its own.  Callers must not modify the bitmap while walking it.
     */
    pub fn xbitmap32_walk(
        bitmap: *mut xbitmap32,
        fn_: Option<unsafe extern "C" fn(start: u32, len: u32, priv_: *mut core::ffi::c_void) -> i32>,
        priv_: *mut core::ffi::c_void,
    ) -> i32;

    pub fn xbitmap32_empty(bitmap: *mut xbitmap32) -> bool;
    pub fn xbitmap32_test(bitmap: *mut xbitmap32, start: u32, len: *mut u32) -> bool;

    pub fn xbitmap32_count_set_regions(bitmap: *mut xbitmap32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
