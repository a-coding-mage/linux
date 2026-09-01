// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// Dependencies from the original C includes:
// "vmlinux.h"
// <bpf/bpf_helpers.h>
// <bpf/bpf_tracing.h>
// <bpf/bpf_core_read.h>
// "../test_kmods/bpf_testmod.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u32 = u32;
pub type loff_t = i64;
pub type size_t = usize;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    pub f_mode: i32,
}

#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bin_attribute {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_test_read_ctx {
    pub len: __u32,
}

#[repr(C)]
pub struct bpf_testmod_test_write_ctx {
    pub len: __u32,
}

#[repr(C)]
pub struct bpf_testmod_test_writable_ctx {
    pub val: i32,
    pub early_ret: i32,
}

unsafe extern "C" {
    pub fn bpf_probe_read_kernel(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[unsafe(no_mangle)]
pub static mut sz: __u32 = 0;

#[unsafe(link_section = "?raw_tp/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_raw_tp(
    task: *mut task_struct,
    read_ctx: *mut bpf_testmod_test_read_ctx,
) -> i32 {
    let _ = task;
    unsafe {
        sz = (*read_ctx).len;
    }
    0
}

#[unsafe(link_section = "?raw_tp/bpf_testmod_test_write_bare_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_raw_tp_bare(
    task: *mut task_struct,
    write_ctx: *mut bpf_testmod_test_write_ctx,
) -> i32 {
    let _ = task;
    unsafe {
        sz = (*write_ctx).len;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut raw_tp_writable_bare_in_val: i32 = 0;
#[unsafe(no_mangle)]
pub static mut raw_tp_writable_bare_early_ret: i32 = 0;
#[unsafe(no_mangle)]
pub static mut raw_tp_writable_bare_out_val: i32 = 0;

#[unsafe(link_section = "?raw_tp.w/bpf_testmod_test_writable_bare_tp")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_raw_tp_writable_bare(
    writable: *mut bpf_testmod_test_writable_ctx,
) -> i32 {
    unsafe {
        raw_tp_writable_bare_in_val = (*writable).val;
        (*writable).early_ret = raw_tp_writable_bare_early_ret;
        (*writable).val = raw_tp_writable_bare_out_val;
    }
    0
}

#[unsafe(link_section = "?tp_btf/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_tp_btf(
    task: *mut task_struct,
    read_ctx: *mut bpf_testmod_test_read_ctx,
) -> i32 {
    let _ = task;
    unsafe {
        sz = (*read_ctx).len;
    }
    0
}

#[unsafe(link_section = "?fentry/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fentry(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
    }
    0
}

#[unsafe(link_section = "?fentry")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fentry_manual(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
    }
    0
}

#[unsafe(link_section = "?fentry/bpf_testmod:bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fentry_explicit(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
    }
    0
}

#[unsafe(link_section = "?fentry")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fentry_explicit_manual(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut retval: i32 = 0;

#[unsafe(link_section = "?fexit/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fexit(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
    ret: i32,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
        retval = ret;
    }
    0
}

#[unsafe(link_section = "?fexit/bpf_testmod_return_ptr")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fexit_ret(arg: i32, ret: *mut file) -> i32 {
    let _ = arg;
    let mut buf: i64 = 0;

    unsafe {
        bpf_probe_read_kernel(
            (&mut buf as *mut i64).cast::<core::ffi::c_void>(),
            8,
            ret.cast::<core::ffi::c_void>(),
        );
        bpf_probe_read_kernel(
            (&mut buf as *mut i64).cast::<core::ffi::c_void>(),
            8,
            (ret.cast::<u8>()).add(256).cast::<core::ffi::c_void>(),
        );
        core::ptr::read_volatile(ret.cast::<i32>());
        core::ptr::read_volatile(core::ptr::addr_of!((*ret).f_mode));
    }
    0
}

#[unsafe(link_section = "?fmod_ret/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn handle_fmod_ret(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let _ = (file, kobj, bin_attr, buf, off);
    unsafe {
        sz = len as __u32;
    }
    0 // don't override the exit code
}

#[unsafe(link_section = "?kprobe.multi/bpf_testmod_test_read")]
#[unsafe(no_mangle)]
pub extern "C" fn kprobe_multi() -> i32 {
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
