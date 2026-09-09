/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for talking to the CUDA.  The CUDA is a microcontroller
 * which controls the ADB, system power, RTC, and various other things.
 *
 * Copyright (C) 1996 Paul Mackerras.
 */

// Dependencies supplied by the corresponding Linux/Rust environment:
// <linux/rtc.h>
// <uapi/linux/cuda.h>

extern "C" {
    pub fn find_via_cuda() -> ::core::ffi::c_int;
    pub fn cuda_request(
        req: *mut adb_request,
        done: Option<unsafe extern "C" fn(*mut adb_request)>,
        nbytes: ::core::ffi::c_int,
        ...,
    ) -> ::core::ffi::c_int;
    pub fn cuda_poll();

    pub fn cuda_get_time() -> time64_t;
    pub fn cuda_set_rtc_time(tm: *mut rtc_time) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
