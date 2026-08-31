/* SPDX-License-Identifier: GPL-2.0-only */

use std::ffi::{c_char, c_int, c_ulong};

unsafe extern "C" {
    pub fn is_xtheadvector_supported() -> bool;

    pub fn is_vector_supported() -> bool;

    pub fn get_vr_len() -> c_ulong;

    pub fn launch_test(next_program: *mut c_char, test_inherit: c_int, xtheadvector: c_int) -> c_int;
}
