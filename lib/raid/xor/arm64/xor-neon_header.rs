/* SPDX-License-Identifier: GPL-2.0-only */

unsafe extern "C" {
    pub fn xor_gen_neon_inner(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: core::ffi::c_uint,
        bytes: core::ffi::c_uint,
    );
    pub fn xor_gen_eor3_inner(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: core::ffi::c_uint,
        bytes: core::ffi::c_uint,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
