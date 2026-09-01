// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_core_read.h>
// #include <errno.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type u32 = u32;
pub type u64 = u64;

const EINVAL: i32 = 22;
const BPF_F_PAD_ZEROS: u64 = 1 << 0;

#[repr(C)]
pub struct __kernel_timespec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_probe_read_user(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: u32, user_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_strncmp(s1: *const i8, s1_sz: u32, s2: *const i8) -> i32;

    // int bpf_copy_from_user_str(void *dst, u32, const void *, u64) __weak __ksym;
    fn bpf_copy_from_user_str(
        dst: *mut core::ffi::c_void,
        sz: u32,
        unsafe_ptr: *const core::ffi::c_void,
        flags: u64,
    ) -> i32;
}

#[no_mangle]
pub static mut dynamic_sz: u32 = 1;
#[no_mangle]
pub static mut kprobe2_res: i32 = 0;
#[no_mangle]
pub static mut kretprobe2_res: i32 = 0;
#[no_mangle]
pub static mut uprobe_byname_res: i32 = 0;
#[no_mangle]
pub static mut uretprobe_byname_res: i32 = 0;
#[no_mangle]
pub static mut uprobe_byname2_res: i32 = 0;
#[no_mangle]
pub static mut uretprobe_byname2_res: i32 = 0;
#[no_mangle]
pub static mut uprobe_byname3_sleepable_res: i32 = 0;
#[no_mangle]
pub static mut uprobe_byname3_str_sleepable_res: i32 = 0;
#[no_mangle]
pub static mut uprobe_byname3_res: i32 = 0;
#[no_mangle]
pub static mut uretprobe_byname3_sleepable_res: i32 = 0;
#[no_mangle]
pub static mut uretprobe_byname3_str_sleepable_res: i32 = 0;
#[no_mangle]
pub static mut uretprobe_byname3_res: i32 = 0;
#[no_mangle]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

#[no_mangle]
#[link_section = "ksyscall/nanosleep"]
pub unsafe extern "C" fn handle_kprobe_auto(
    _req: *mut __kernel_timespec,
    _rem: *mut __kernel_timespec,
) -> i32 {
    kprobe2_res = 11;
    0
}

#[no_mangle]
#[link_section = "kretsyscall/nanosleep"]
pub unsafe extern "C" fn handle_kretprobe_auto(ret: i32) -> i32 {
    kretprobe2_res = 22;
    ret
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe_ref_ctr(_ctx: *mut pt_regs) -> i32 {
    0
}

#[no_mangle]
#[link_section = "uretprobe"]
pub unsafe extern "C" fn handle_uretprobe_ref_ctr(_ctx: *mut pt_regs) -> i32 {
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe_byname(_ctx: *mut pt_regs) -> i32 {
    uprobe_byname_res = 5;
    0
}

/* use auto-attach format for section definition. */
#[no_mangle]
#[link_section = "uretprobe//proc/self/exe:trigger_func2"]
pub unsafe extern "C" fn handle_uretprobe_byname(_ctx: *mut pt_regs) -> i32 {
    uretprobe_byname_res = 6;
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe_byname2(pathname: *const i8, mode: *const i8) -> i32 {
    let mut mode_buf: [i8; 2] = [0; 2];

    /* verify fopen mode */
    let _ = bpf_probe_read_user(
        mode_buf.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&mode_buf) as u32,
        mode as *const core::ffi::c_void,
    );
    if mode_buf[0] == b'r' as i8 && mode_buf[1] == 0 {
        uprobe_byname2_res = 7;
    }
    let _ = pathname;
    0
}

#[no_mangle]
#[link_section = "uretprobe"]
pub unsafe extern "C" fn handle_uretprobe_byname2(ret: *mut core::ffi::c_void) -> i32 {
    uretprobe_byname2_res = 8;
    let _ = ret;
    0
}

#[inline(always)]
unsafe fn verify_sleepable_user_copy() -> bool {
    let mut data: [i8; 9] = [0; 9];

    let _ = bpf_copy_from_user(
        data.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data) as u32,
        user_ptr as *const core::ffi::c_void,
    );
    bpf_strncmp(
        data.as_ptr(),
        core::mem::size_of_val(&data) as u32,
        c"test_data".as_ptr(),
    ) == 0
}

#[inline(always)]
unsafe fn verify_sleepable_user_copy_str() -> bool {
    let mut ret: i32;
    let mut data_long: [i8; 20] = [0; 20];
    let mut data_long_pad: [i8; 20] = [0; 20];
    let mut data_long_err: [i8; 20] = [0; 20];
    let mut data_short: [i8; 4] = [0; 4];
    let mut data_short_pad: [i8; 4] = [0; 4];

    ret = bpf_copy_from_user_str(
        data_short.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_short) as u32,
        user_ptr as *const core::ffi::c_void,
        0,
    );

    if bpf_strncmp(data_short.as_ptr(), 4, c"tes".as_ptr()) != 0 || ret != 4 {
        return false;
    }

    ret = bpf_copy_from_user_str(
        data_short_pad.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_short_pad) as u32,
        user_ptr as *const core::ffi::c_void,
        BPF_F_PAD_ZEROS,
    );

    if bpf_strncmp(data_short.as_ptr(), 4, c"tes".as_ptr()) != 0 || ret != 4 {
        return false;
    }

    /* Make sure this passes the verifier */
    ret = bpf_copy_from_user_str(
        data_long.as_mut_ptr() as *mut core::ffi::c_void,
        (dynamic_sz & core::mem::size_of_val(&data_long) as u32) as u32,
        user_ptr as *const core::ffi::c_void,
        0,
    );

    if ret != 0 {
        return false;
    }

    ret = bpf_copy_from_user_str(
        data_long.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_long) as u32,
        user_ptr as *const core::ffi::c_void,
        0,
    );

    if bpf_strncmp(data_long.as_ptr(), 10, c"test_data".as_ptr()) != 0 || ret != 10 {
        return false;
    }

    ret = bpf_copy_from_user_str(
        data_long_pad.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_long_pad) as u32,
        user_ptr as *const core::ffi::c_void,
        BPF_F_PAD_ZEROS,
    );

    if bpf_strncmp(data_long_pad.as_ptr(), 10, c"test_data".as_ptr()) != 0
        || ret != 10
        || data_long_pad[19] != b'\0' as i8
    {
        return false;
    }

    ret = bpf_copy_from_user_str(
        data_long_err.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_long_err) as u32,
        data_long.as_mut_ptr() as *mut core::ffi::c_void,
        BPF_F_PAD_ZEROS,
    );

    if ret > 0 || data_long_err[19] != b'\0' as i8 {
        return false;
    }

    ret = bpf_copy_from_user_str(
        data_long.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&data_long) as u32,
        user_ptr as *const core::ffi::c_void,
        2,
    );

    if ret != -EINVAL {
        return false;
    }

    true
}

#[no_mangle]
#[link_section = "uprobe.s//proc/self/exe:trigger_func3"]
pub unsafe extern "C" fn handle_uprobe_byname3_sleepable(_ctx: *mut pt_regs) -> i32 {
    if verify_sleepable_user_copy() {
        uprobe_byname3_sleepable_res = 9;
    }
    if verify_sleepable_user_copy_str() {
        uprobe_byname3_str_sleepable_res = 10;
    }
    0
}

/**
 * same target as the uprobe.s above to force sleepable and non-sleepable
 * programs in the same bpf_prog_array
 */
#[no_mangle]
#[link_section = "uprobe//proc/self/exe:trigger_func3"]
pub unsafe extern "C" fn handle_uprobe_byname3(_ctx: *mut pt_regs) -> i32 {
    uprobe_byname3_res = 11;
    0
}

#[no_mangle]
#[link_section = "uretprobe.s//proc/self/exe:trigger_func3"]
pub unsafe extern "C" fn handle_uretprobe_byname3_sleepable(_ctx: *mut pt_regs) -> i32 {
    if verify_sleepable_user_copy() {
        uretprobe_byname3_sleepable_res = 12;
    }
    if verify_sleepable_user_copy_str() {
        uretprobe_byname3_str_sleepable_res = 13;
    }
    0
}

#[no_mangle]
#[link_section = "uretprobe//proc/self/exe:trigger_func3"]
pub unsafe extern "C" fn handle_uretprobe_byname3(_ctx: *mut pt_regs) -> i32 {
    uretprobe_byname3_res = 14;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
