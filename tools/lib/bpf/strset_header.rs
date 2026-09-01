/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/* Copyright (c) 2021 Facebook */

// C header dependencies: <stdbool.h>, <stddef.h>

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;

// Opaque C type: struct strset;
pub type strset = c_void;

unsafe extern "C" {
    pub fn strset__new(
        max_data_sz: size_t,
        init_data: *const c_char,
        init_data_sz: size_t,
    ) -> *mut strset;
    pub fn strset__free(set: *mut strset);

    pub fn strset__data(set: *const strset) -> *const c_char;
    pub fn strset__data_size(set: *const strset) -> size_t;

    pub fn strset__find_str(set: *mut strset, s: *const c_char) -> c_int;
    pub fn strset__add_str(set: *mut strset, s: *const c_char) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
