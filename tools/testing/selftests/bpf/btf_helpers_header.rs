/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2020 Facebook */

// C dependencies from the original header:
// #include <stdio.h>
// #include <bpf/btf.h>

use std::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn fprintf_btf_type_raw(out: *mut FILE, btf: *const btf, id: __u32) -> c_int;
    pub fn btf_type_raw_dump(btf: *const btf, type_id: c_int) -> *const c_char;
    pub fn btf_validate_raw(
        btf: *mut btf,
        nr_types: c_int,
        exp_types: *const *const c_char,
    ) -> c_int;
    pub fn btf_type_c_dump(btf: *const btf) -> *const c_char;
}

macro_rules! VALIDATE_RAW_BTF {
    ($btf:expr $(, $raw_types:expr)* $(,)?) => {{
        let raw_types = [$($raw_types),*];
        unsafe {
            btf_validate_raw(
                $btf,
                (raw_types.len()) as c_int,
                raw_types.as_ptr(),
            )
        }
    }};
}

pub(crate) use VALIDATE_RAW_BTF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
