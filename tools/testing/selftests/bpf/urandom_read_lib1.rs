// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependency intent:
// #define _SDT_HAS_SEMAPHORES 1
// #include "sdt.h"
//
// #define SHARED 1
// #include "bpf/libbpf_internal.h"

pub const _SDT_HAS_SEMAPHORES: i32 = 1;
pub const SHARED: i32 = 1;

// C macro equivalent: #define SEC(name) __attribute__((section(name), used))
#[no_mangle]
#[link_section = ".probes"]
pub static mut urandlib_read_with_sema_semaphore: u16 = 0;

unsafe extern "C" {
    // Supplied by the translated or linked equivalent of "sdt.h".
    fn STAP_PROBE3(provider: *const u8, name: *const u8, arg1: i32, arg2: i32, arg3: i32);
}

#[no_mangle]
pub extern "C" fn urandlib_read_with_sema(iter_num: i32, iter_cnt: i32, read_sz: i32) {
    unsafe {
        STAP_PROBE3(
            c"urandlib".as_ptr() as *const u8,
            c"read_with_sema".as_ptr() as *const u8,
            iter_num,
            iter_cnt,
            read_sz,
        );
    }
}

// C symbol versioning intent:
// COMPAT_VERSION(urandlib_api_v1, urandlib_api, LIBURANDOM_READ_1.0.0)
#[no_mangle]
pub extern "C" fn urandlib_api_v1() -> i32 {
    1
}

// C symbol versioning intent:
// DEFAULT_VERSION(urandlib_api_v2, urandlib_api, LIBURANDOM_READ_2.0.0)
#[no_mangle]
pub extern "C" fn urandlib_api_v2() -> i32 {
    2
}

// C symbol versioning intent:
// COMPAT_VERSION(urandlib_api_sameoffset, urandlib_api_sameoffset, LIBURANDOM_READ_1.0.0)
// DEFAULT_VERSION(urandlib_api_sameoffset, urandlib_api_sameoffset, LIBURANDOM_READ_2.0.0)
#[no_mangle]
pub extern "C" fn urandlib_api_sameoffset() -> i32 {
    3
}
