// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2024. Huawei Technologies Co., Ltd */

use core::ffi::c_void;

type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_kernel_str(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_str(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_probe_read_user_str(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_copy_from_user_task(
        dst: *mut c_void,
        size: u32,
        unsafe_ptr: *const c_void,
        task: *mut task_struct,
        flags: u64,
    ) -> i32;
    fn bpf_get_current_task_btf() -> *mut task_struct;
}

#[no_mangle]
pub static mut target_pid: i32 = 0;
#[no_mangle]
pub static mut user_ptr: *mut c_void = 0 as *mut c_void;
#[no_mangle]
pub static mut read_ret: [i32; 10] = [0; 10];

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/*
 * These are the kfuncs, the others are helpers
 */
extern "C" {
    #[link_name = "bpf_copy_from_user_str"]
    fn bpf_copy_from_user_str(dst: *mut c_void, arg1: u32, arg2: *const c_void, arg3: u64) -> i32;
    #[link_name = "bpf_copy_from_user_task_str"]
    fn bpf_copy_from_user_task_str(
        dst: *mut c_void,
        arg1: u32,
        arg2: *const c_void,
        arg3: *mut task_struct,
        arg4: u64,
    ) -> i32;
}

/* SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn do_probe_read(ctx: *mut c_void) -> i32 {
    let mut buf: [i8; 8] = [0; 8];

    if (bpf_get_current_pid_tgid() >> 32) as i32 != target_pid {
        return 0;
    }

    read_ret[0] = bpf_probe_read_kernel(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[1] = bpf_probe_read_kernel_str(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[2] = bpf_probe_read(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[3] = bpf_probe_read_str(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[4] = bpf_probe_read_user(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[5] = bpf_probe_read_user_str(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );

    let _ = ctx;
    0
}

/* SEC("fentry.s/" SYS_PREFIX "sys_nanosleep") */
#[no_mangle]
pub unsafe extern "C" fn do_copy_from_user(ctx: *mut c_void) -> i32 {
    let mut buf: [i8; 8] = [0; 8];

    if (bpf_get_current_pid_tgid() >> 32) as i32 != target_pid {
        return 0;
    }

    read_ret[6] = bpf_copy_from_user(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
    );
    read_ret[7] = bpf_copy_from_user_task(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
        bpf_get_current_task_btf(),
        0,
    );
    read_ret[8] = bpf_copy_from_user_str(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
        0,
    );
    read_ret[9] = bpf_copy_from_user_task_str(
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const c_void,
        bpf_get_current_task_btf(),
        0,
    );

    let _ = ctx;
    0
}
