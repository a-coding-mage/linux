// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LoongArch SIMD XOR operations
 *
 * Copyright (C) 2023 WANG Xuerui <git@xen0n.name>
 */

use core::ffi::c_void;

// Dependencies supplied by the kernel and the XOR implementation headers.
extern "C" {
    fn kernel_fpu_begin();
    fn kernel_fpu_end();
}

#[repr(C)]
pub struct xor_block_template {
    pub name: *const u8,
    pub xor_gen: Option<unsafe extern "C" fn(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: u32,
        bytes: u32,
    )>,
}

// DO_XOR_BLOCKS is defined by xor_impl.h.  Its generated inner routines are
// represented here as external declarations because that macro is not local
// to this translation unit.
extern "C" {
    fn xor_gen_lsx_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: u32,
        bytes: u32,
    );
    fn xor_gen_lasx_inner(
        dest: *mut c_void,
        srcs: *mut *mut c_void,
        src_cnt: u32,
        bytes: u32,
    );
}

// #ifdef CONFIG_CPU_HAS_LSX
#[cfg(CONFIG_CPU_HAS_LSX)]
#[no_mangle]
pub unsafe extern "C" fn xor_gen_lsx(
    dest: *mut c_void,
    srcs: *mut *mut c_void,
    src_cnt: u32,
    bytes: u32,
) {
    kernel_fpu_begin();
    xor_gen_lsx_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
}

#[cfg(CONFIG_CPU_HAS_LSX)]
#[no_mangle]
pub static mut xor_block_lsx: xor_block_template = xor_block_template {
    name: b"lsx\0".as_ptr(),
    xor_gen: Some(xor_gen_lsx),
};
// #endif /* CONFIG_CPU_HAS_LSX */

// #ifdef CONFIG_CPU_HAS_LASX
#[cfg(CONFIG_CPU_HAS_LASX)]
#[no_mangle]
pub unsafe extern "C" fn xor_gen_lasx(
    dest: *mut c_void,
    srcs: *mut *mut c_void,
    src_cnt: u32,
    bytes: u32,
) {
    kernel_fpu_begin();
    xor_gen_lasx_inner(dest, srcs, src_cnt, bytes);
    kernel_fpu_end();
}

#[cfg(CONFIG_CPU_HAS_LASX)]
#[no_mangle]
pub static mut xor_block_lasx: xor_block_template = xor_block_template {
    name: b"lasx\0".as_ptr(),
    xor_gen: Some(xor_gen_lasx),
};
// #endif /* CONFIG_CPU_HAS_LASX */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
