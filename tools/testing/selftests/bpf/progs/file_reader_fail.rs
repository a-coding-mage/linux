// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies: <vmlinux.h>, <string.h>, <stdbool.h>, <bpf/bpf_tracing.h>,
// and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_dynptr {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_get_task_exe_file(task: *mut task_struct) -> *mut file;
    fn bpf_dynptr_from_file(file: *mut file, flags: u64, ptr: *mut bpf_dynptr) -> c_int;
    fn bpf_dynptr_from_xdp(xdp: *mut xdp_md, flags: u64, ptr: *mut bpf_dynptr) -> c_int;
    fn bpf_dynptr_file_discard(ptr: *mut bpf_dynptr);
    fn bpf_put_file(file: *mut file);
    fn bpf_dynptr_read(
        dst: *mut c_void,
        len: u32,
        src: *const bpf_dynptr,
        offset: u32,
        flags: u64,
    ) -> c_int;
    fn bpf_dynptr_slice(
        ptr: *const bpf_dynptr,
        offset: u32,
        buffer: *mut c_void,
        buffer_sz: u32,
    ) -> *const c_char;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut err: c_int = 0;

#[unsafe(no_mangle)]
pub static mut user_ptr: *mut c_void = core::ptr::null_mut();

// SEC("lsm/file_open")
// __failure
// __msg("Unreleased reference id=")
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm/file_open")]
pub unsafe extern "C" fn on_nanosleep_unreleased_ref(ctx: *mut c_void) -> c_int {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let file: *mut file = unsafe { bpf_get_task_exe_file(task) };
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    if file.is_null() {
        return 0;
    }

    unsafe {
        err = bpf_dynptr_from_file(file, 0, dynptr.as_mut_ptr());
        if err != 0 { 1 } else { 0 }
    }
}

// SEC("xdp")
// __failure
// __msg("Expected a dynptr of type file as R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_wrong_dynptr_type(xdp: *mut xdp_md) -> c_int {
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_from_xdp(xdp, 0, dynptr.as_mut_ptr());
        bpf_dynptr_file_discard(dynptr.as_mut_ptr());
    }
    0
}

// SEC("xdp")
// __failure
// __msg("Expected an initialized dynptr as R1")
#[unsafe(no_mangle)]
#[unsafe(link_section = "xdp")]
pub unsafe extern "C" fn xdp_no_dynptr_type(xdp: *mut xdp_md) -> c_int {
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();

    unsafe {
        bpf_dynptr_file_discard(dynptr.as_mut_ptr());
    }
    0
}

// SEC("lsm/file_open")
// __failure
// __msg("Leaking reference id={{[0-9]+}} alloc_insn={{[0-9]+}}. Release it first.")
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm/file_open")]
pub unsafe extern "C" fn use_file_dynptr_after_put_file(ctx: *mut c_void) -> c_int {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let file: *mut file = unsafe { bpf_get_task_exe_file(task) };
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut buf: [c_char; 64] = [0; 64];

    if file.is_null() {
        return 0;
    }

    if unsafe { bpf_dynptr_from_file(file, 0, dynptr.as_mut_ptr()) } != 0 {
        unsafe {
            bpf_dynptr_file_discard(dynptr.as_mut_ptr());
            bpf_put_file(file);
        }
        return 0;
    }

    /* this should fail - file dynptr should be discarded first to prevent resource leak */
    unsafe {
        bpf_put_file(file);

        bpf_dynptr_read(
            buf.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&buf) as u32,
            dynptr.as_ptr(),
            0,
            0,
        );
    }
    0
}

// SEC("lsm/file_open")
// __failure
// __msg("Leaking reference id={{[0-9]+}} alloc_insn={{[0-9]+}}. Release it first.")
#[unsafe(no_mangle)]
#[unsafe(link_section = "lsm/file_open")]
pub unsafe extern "C" fn use_file_dynptr_slice_after_put_file(ctx: *mut c_void) -> c_int {
    let task: *mut task_struct = unsafe { bpf_get_current_task_btf() };
    let file: *mut file = unsafe { bpf_get_task_exe_file(task) };
    let mut dynptr = core::mem::MaybeUninit::<bpf_dynptr>::uninit();
    let mut buf: [c_char; 1] = [0; 1];
    let data: *const c_char;

    if file.is_null() {
        return 0;
    }

    if unsafe { bpf_dynptr_from_file(file, 0, dynptr.as_mut_ptr()) } != 0 {
        unsafe {
            bpf_dynptr_file_discard(dynptr.as_mut_ptr());
            bpf_put_file(file);
        }
        return 0;
    }

    data = unsafe {
        bpf_dynptr_slice(
            dynptr.as_ptr(),
            0,
            buf.as_mut_ptr() as *mut c_void,
            core::mem::size_of_val(&buf) as u32,
        )
    };
    if data.is_null() {
        unsafe {
            bpf_dynptr_file_discard(dynptr.as_mut_ptr());
            bpf_put_file(file);
        }
        return 0;
    }

    /* this should fail - file dynptr should be discarded first to prevent resource leak */
    unsafe {
        bpf_put_file(file);

        *data as c_int
    }
}
