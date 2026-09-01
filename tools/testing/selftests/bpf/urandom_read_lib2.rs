// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
// C dependency: #include "sdt.h"

extern "C" {
    fn STAP_PROBE3(
        provider: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn urandlib_read_without_sema(
    iter_num: ::core::ffi::c_int,
    iter_cnt: ::core::ffi::c_int,
    read_sz: ::core::ffi::c_int,
) {
    unsafe {
        STAP_PROBE3(
            b"urandlib\0".as_ptr() as *const ::core::ffi::c_char,
            b"read_without_sema\0".as_ptr() as *const ::core::ffi::c_char,
            iter_num,
            iter_cnt,
            read_sz,
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
