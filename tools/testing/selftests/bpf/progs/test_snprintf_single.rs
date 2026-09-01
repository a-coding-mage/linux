// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Google LLC. */

use core::ffi::c_void;
use core::ptr;

unsafe extern "C" {
    fn bpf_snprintf(
        str: *mut i8,
        str_size: u32,
        fmt: *const i8,
        data: *const c_void,
        data_len: u32,
    ) -> i64;
}

/* The format string is filled from the userspace such that loading fails */
#[no_mangle]
pub static fmt: [i8; 10] = [0; 10];

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler(ctx: *const c_void) -> i32 {
    let arg: u64 = 42;

    unsafe {
        bpf_snprintf(
            ptr::null_mut(),
            0,
            fmt.as_ptr(),
            &arg as *const u64 as *const c_void,
            core::mem::size_of_val(&arg) as u32,
        );
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [i8; 4] = [b'G' as i8, b'P' as i8, b'L' as i8, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
