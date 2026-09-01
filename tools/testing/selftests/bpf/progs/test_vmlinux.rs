// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies:
// #include "vmlinux.h"
// #include <asm/unistd.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_core_read.h>

pub const MY_TV_NSEC: i32 = 1337;

extern "C" {
    pub static __NR_nanosleep: ::core::ffi::c_long;

    pub fn bpf_probe_read_user(
        dst: *mut ::core::ffi::c_void,
        size: u32,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;

    // From BPF tracing helpers/macros.
    pub fn PT_REGS_PARM1_CORE_SYSCALL(regs: *mut pt_regs) -> *mut ::core::ffi::c_void;
}

#[repr(C)]
pub struct syscall_trace_enter {
    pub nr: ::core::ffi::c_long,
    pub args: [u64; 6],
}

// Types supplied by vmlinux.h/BTF in the original C source.
#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: ::core::ffi::c_long,
    pub tv_nsec: ::core::ffi::c_long,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hrtimer {
    _private: [u8; 0],
}

pub type ktime_t = i64;

#[repr(C)]
pub enum hrtimer_mode {
    // Variants are provided by vmlinux.h in the original C source.
}

#[no_mangle]
pub static mut tp_called: bool = false;
#[no_mangle]
pub static mut raw_tp_called: bool = false;
#[no_mangle]
pub static mut tp_btf_called: bool = false;
#[no_mangle]
pub static mut kprobe_called: bool = false;
#[no_mangle]
pub static mut fentry_called: bool = false;

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn handle__tp(args: *mut syscall_trace_enter) -> i32 {
    let ts: *mut __kernel_timespec;
    let mut tv_nsec: ::core::ffi::c_long = 0;

    if (*args).nr != __NR_nanosleep {
        return 0;
    }

    ts = (*args).args[0] as *mut __kernel_timespec;
    if bpf_probe_read_user(
        &mut tv_nsec as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&(*ts).tv_nsec) as u32,
        &(*ts).tv_nsec as *const _ as *const ::core::ffi::c_void,
    ) != 0
        || tv_nsec != MY_TV_NSEC as ::core::ffi::c_long
    {
        return 0;
    }

    tp_called = true;
    0
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handle__raw_tp(regs: *mut pt_regs, id: ::core::ffi::c_long) -> i32 {
    let ts: *mut __kernel_timespec;
    let mut tv_nsec: ::core::ffi::c_long = 0;

    if id != __NR_nanosleep {
        return 0;
    }

    ts = PT_REGS_PARM1_CORE_SYSCALL(regs) as *mut __kernel_timespec;
    if bpf_probe_read_user(
        &mut tv_nsec as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&(*ts).tv_nsec) as u32,
        &(*ts).tv_nsec as *const _ as *const ::core::ffi::c_void,
    ) != 0
        || tv_nsec != MY_TV_NSEC as ::core::ffi::c_long
    {
        return 0;
    }

    raw_tp_called = true;
    0
}

#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn handle__tp_btf(regs: *mut pt_regs, id: ::core::ffi::c_long) -> i32 {
    let ts: *mut __kernel_timespec;
    let mut tv_nsec: ::core::ffi::c_long = 0;

    if id != __NR_nanosleep {
        return 0;
    }

    ts = PT_REGS_PARM1_CORE_SYSCALL(regs) as *mut __kernel_timespec;
    if bpf_probe_read_user(
        &mut tv_nsec as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&(*ts).tv_nsec) as u32,
        &(*ts).tv_nsec as *const _ as *const ::core::ffi::c_void,
    ) != 0
        || tv_nsec != MY_TV_NSEC as ::core::ffi::c_long
    {
        return 0;
    }

    tp_btf_called = true;
    0
}

#[no_mangle]
#[link_section = "kprobe"]
pub unsafe extern "C" fn handle__kprobe(
    _timer: *mut hrtimer,
    tim: ktime_t,
    _delta_ns: u64,
    _mode: hrtimer_mode,
) -> i32 {
    if tim == MY_TV_NSEC as ktime_t {
        kprobe_called = true;
    }
    0
}

#[no_mangle]
#[link_section = "fentry"]
pub unsafe extern "C" fn handle__fentry(
    _timer: *mut hrtimer,
    tim: ktime_t,
    _delta_ns: u64,
    _mode: hrtimer_mode,
) -> i32 {
    if tim == MY_TV_NSEC as ktime_t {
        fentry_called = true;
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
