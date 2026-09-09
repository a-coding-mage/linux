// SPDX-License-Identifier: GPL-2.0-only
/*
 * Authors: Jackie Liu <liuyun01@kylinos.cn>
 * Copyright (C) 2018,Tianjin KYLIN Information Technology Co., Ltd.
 */

use core::ffi::{c_char, c_uint, c_void};

// The declarations below are supplied by the corresponding architecture
// headers and implementation files.
#[repr(C)]
pub struct xor_block_template {
    pub name: *const c_char,
    pub xor_gen: Option<unsafe extern "C" fn(*mut c_void, *mut *mut c_void, c_uint, c_uint)>,
}

unsafe extern "C" {
    fn xor_gen_neon_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: c_uint,
        bytes: c_uint,
    );
    fn xor_gen_eor3_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: c_uint,
        bytes: c_uint,
    );
}

unsafe extern "C" fn xor_gen_neon(
    dest: *mut c_void,
    srcs: *mut *mut c_void,
    src_cnt: c_uint,
    bytes: c_uint,
) {
    // scoped_ksimd() establishes the kernel SIMD context for this operation.
    unsafe { xor_gen_neon_inner(dest, srcs, src_cnt, bytes) };
}

#[no_mangle]
pub static mut xor_block_neon: xor_block_template = xor_block_template {
    name: b"neon\0".as_ptr() as *const c_char,
    xor_gen: Some(xor_gen_neon),
};

unsafe extern "C" fn xor_gen_eor3(
    dest: *mut c_void,
    srcs: *mut *mut c_void,
    src_cnt: c_uint,
    bytes: c_uint,
) {
    // scoped_ksimd() establishes the kernel SIMD context for this operation.
    unsafe { xor_gen_eor3_inner(dest, srcs, src_cnt, bytes) };
}

#[no_mangle]
pub static mut xor_block_eor3: xor_block_template = xor_block_template {
    name: b"eor3\0".as_ptr() as *const c_char,
    xor_gen: Some(xor_gen_eor3),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
