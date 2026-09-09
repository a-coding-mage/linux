// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2021 SiFive
 */

// C dependencies supplied by the corresponding architecture and XOR headers.
use core::ffi::c_void;
use core::ffi::c_uint;

// DO_XOR_BLOCKS(vector_inner, xor_regs_2_, xor_regs_3_, xor_regs_4_, xor_regs_5_);
// The macro expansion is provided by xor_impl.h and remains an external
// dependency of this translation.
extern "C" {
    fn kernel_vector_begin();
    fn kernel_vector_end();
    fn xor_gen_vector_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: c_uint,
        bytes: c_uint,
    );
}

unsafe extern "C" fn xor_gen_vector(
    dest: *mut c_void,
    srcs: *mut *mut c_void,
    src_cnt: c_uint,
    bytes: c_uint,
) {
    kernel_vector_begin();
    xor_gen_vector_inner(dest, srcs, src_cnt, bytes);
    kernel_vector_end();
}

pub static mut xor_block_rvv: xor_block_template = xor_block_template {
    name: b"rvv\0".as_ptr() as *const _,
    xor_gen: xor_gen_vector,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
