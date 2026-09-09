// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Altivec XOR operations
 *
 * Copyright 2017 IBM Corp.
 */

// The declarations below are supplied by the included kernel headers.
extern "C" {
    fn preempt_disable();
    fn enable_kernel_altivec();
    fn xor_gen_altivec_inner(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: u32,
        bytes: u32,
    );
    fn disable_kernel_altivec();
    fn preempt_enable();
}

#[repr(C)]
pub struct xor_block_template {
    pub name: *const core::ffi::c_char,
    pub xor_gen: Option<unsafe extern "C" fn(
        dest: *mut core::ffi::c_void,
        srcs: *mut *mut core::ffi::c_void,
        src_cnt: u32,
        bytes: u32,
    )>,
}

unsafe extern "C" fn xor_gen_altivec(
    dest: *mut core::ffi::c_void,
    srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32,
    bytes: u32,
) {
    unsafe {
        preempt_disable();
        enable_kernel_altivec();
        xor_gen_altivec_inner(dest, srcs, src_cnt, bytes);
        disable_kernel_altivec();
        preempt_enable();
    }
}

#[no_mangle]
pub static mut xor_block_altivec: xor_block_template = xor_block_template {
    name: b"altivec\0".as_ptr() as *const core::ffi::c_char,
    xor_gen: Some(xor_gen_altivec),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
