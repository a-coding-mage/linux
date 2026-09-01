// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies in the original C source:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 */

#[no_mangle]
pub static mut a: [i32; 4] = [0; 4];

#[no_mangle]
pub static off: i32 = 4000;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn good_prog(ctx: *const core::ffi::c_void) -> i32 {
    a[0] = ctx as isize as i32;
    a[1]
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn bad_prog(_ctx: *const core::ffi::c_void) -> i32 {
    /* out of bounds access */
    core::ptr::read_volatile(a.as_ptr().offset(core::ptr::read_volatile(&off) as isize))
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
