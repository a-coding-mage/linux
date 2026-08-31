// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// "vmlinux.h", <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
}

#[no_mangle]
pub static mut user_ptr: *mut c_void = core::ptr::null_mut();

#[inline(always)]
unsafe fn barrier_var_i32(mut var: i32) {
    core::arch::asm!("", inout(reg) var, options(nostack, preserves_flags));
}

#[no_mangle]
#[link_section = "kprobe.multi"]
pub unsafe extern "C" fn handle_kprobe_multi_sleepable(ctx: *mut pt_regs) -> i32 {
    let mut a: i32 = 0;
    let err: i32;

    let _ = ctx;

    err = bpf_copy_from_user(
        &mut a as *mut i32 as *mut c_void,
        core::mem::size_of_val(&a) as u32,
        user_ptr as *const c_void,
    );
    barrier_var_i32(a);
    return err;
}

#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn fentry() -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
