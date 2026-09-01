// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Hengqi Chen */

// C dependencies removed from executable Rust:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
//
// The BPF helper declarations, SEC/link-section handling, BPF_UPROBE/
// BPF_URETPROBE calling convention, and pt_regs definition are expected to be
// supplied by the surrounding BPF Rust build environment.

type pid_t = i32;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[no_mangle]
pub static mut my_pid: pid_t = 0;

#[no_mangle]
pub static mut test1_result: i32 = 0;
#[no_mangle]
pub static mut test2_result: i32 = 0;
#[no_mangle]
pub static mut test3_result: i32 = 0;
#[no_mangle]
pub static mut test4_result: i32 = 0;

#[link_section = "uprobe/./liburandom_read.so:urandlib_api_sameoffset"]
#[no_mangle]
pub unsafe extern "C" fn test1(_ctx: *mut pt_regs) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    test1_result = 1;
    return 0;
}

#[link_section = "uprobe/./liburandom_read.so:urandlib_api_sameoffset@LIBURANDOM_READ_1.0.0"]
#[no_mangle]
pub unsafe extern "C" fn test2(_ctx: *mut pt_regs) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    test2_result = 1;
    return 0;
}

#[link_section = "uretprobe/./liburandom_read.so:urandlib_api_sameoffset@@LIBURANDOM_READ_2.0.0"]
#[no_mangle]
pub unsafe extern "C" fn test3(_ctx: *mut pt_regs, ret: i32) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    test3_result = ret;
    return 0;
}

#[link_section = "uprobe"]
#[no_mangle]
pub unsafe extern "C" fn test4(_ctx: *mut pt_regs) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    test4_result = 1;
    return 0;
}

// Original C condition: #if defined(__TARGET_ARCH_x86)
#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct pt_regs {
    pub ax: u64,
    pub cx: u64,
    pub dx: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub di: u64,
    pub si: u64,
    pub ip: u64,
}

#[cfg(not(target_arch = "x86_64"))]
pub enum pt_regs {}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub static mut regs: pt_regs = pt_regs {
    ax: 0,
    cx: 0,
    dx: 0,
    r8: 0,
    r9: 0,
    r10: 0,
    r11: 0,
    di: 0,
    si: 0,
    ip: 0,
};

#[cfg(target_arch = "x86_64")]
#[link_section = "uprobe"]
#[no_mangle]
pub unsafe extern "C" fn test_regs_change(ctx: *mut pt_regs) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    (*ctx).ax = regs.ax;
    (*ctx).cx = regs.cx;
    (*ctx).dx = regs.dx;
    (*ctx).r8 = regs.r8;
    (*ctx).r9 = regs.r9;
    (*ctx).r10 = regs.r10;
    (*ctx).r11 = regs.r11;
    (*ctx).di = regs.di;
    (*ctx).si = regs.si;
    return 0;
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub static mut ip: u64 = 0;

#[cfg(target_arch = "x86_64")]
#[link_section = "uprobe"]
#[no_mangle]
pub unsafe extern "C" fn test_regs_change_ip(ctx: *mut pt_regs) -> i32 {
    let pid: pid_t = (bpf_get_current_pid_tgid() >> 32) as pid_t;

    if pid != my_pid {
        return 0;
    }

    (*ctx).ip = ip;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
