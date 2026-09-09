// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2001 Russell King
 */
// Dependencies supplied by xor_impl.h and xor_arch.h.

extern "C" {
    fn kernel_neon_begin();
    fn xor_gen_neon_inner(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: u32,
        bytes: u32,
    );
    fn kernel_neon_end();
}

unsafe fn xor_gen_neon(
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32,
    bytes: u32,
) {
    unsafe {
        kernel_neon_begin();
        xor_gen_neon_inner(dest, srcs, src_cnt, bytes);
        kernel_neon_end();
    }
}

#[no_mangle]
pub static mut xor_block_neon: xor_block_template = xor_block_template {
    name: b"neon\0".as_ptr() as *const core::ffi::c_char,
    xor_gen: xor_gen_neon,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
