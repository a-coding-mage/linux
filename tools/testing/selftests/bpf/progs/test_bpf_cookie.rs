// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include <errno.h>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_int, c_ulong, c_void};

const EPERM: c_int = 1;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_attach_cookie(ctx: *mut c_void) -> u64;
}

#[no_mangle]
pub static mut my_tid: c_int = 0;

#[no_mangle]
pub static mut kprobe_res: u64 = 0;
#[no_mangle]
pub static mut kprobe_multi_res: u64 = 0;
#[no_mangle]
pub static mut kretprobe_res: u64 = 0;
#[no_mangle]
pub static mut uprobe_res: u64 = 0;
#[no_mangle]
pub static mut uretprobe_res: u64 = 0;
#[no_mangle]
pub static mut tp_res: u64 = 0;
#[no_mangle]
pub static mut pe_res: u64 = 0;
#[no_mangle]
pub static mut raw_tp_res: u64 = 0;
#[no_mangle]
pub static mut tp_btf_res: u64 = 0;
#[no_mangle]
pub static mut fentry_res: u64 = 0;
#[no_mangle]
pub static mut fexit_res: u64 = 0;
#[no_mangle]
pub static mut fmod_ret_res: u64 = 0;
#[no_mangle]
pub static mut lsm_res: u64 = 0;

unsafe fn update(ctx: *mut c_void, res: *mut u64) {
    if my_tid != bpf_get_current_pid_tgid() as u32 as c_int {
        return;
    }

    *res |= bpf_get_attach_cookie(ctx);
}

#[no_mangle]
#[link_section = "kprobe"]
pub unsafe extern "C" fn handle_kprobe(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut kprobe_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "kretprobe"]
pub unsafe extern "C" fn handle_kretprobe(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut kretprobe_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut uprobe_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "uretprobe"]
pub unsafe extern "C" fn handle_uretprobe(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut uretprobe_res as *mut u64);
    0
}

/* bpf_prog_array, used by kernel internally to keep track of attached BPF
 * programs to a given BPF hook (e.g., for tracepoints) doesn't allow the same
 * BPF program to be attached multiple times. So have three identical copies
 * ready to attach to the same tracepoint.
 */
#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn handle_tp1(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut tp_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn handle_tp2(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut tp_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_nanosleep"]
pub unsafe extern "C" fn handle_tp3(ctx: *mut c_void) -> c_int {
    update(ctx, &mut tp_res as *mut u64);
    1
}

#[no_mangle]
#[link_section = "perf_event"]
pub unsafe extern "C" fn handle_pe(ctx: *mut pt_regs) -> c_int {
    update(ctx as *mut c_void, &mut pe_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handle_raw_tp(ctx: *mut c_void) -> c_int {
    update(ctx, &mut raw_tp_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "tp_btf/sys_enter"]
pub unsafe extern "C" fn handle_tp_btf(ctx: *mut c_void) -> c_int {
    update(ctx, &mut tp_btf_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "fentry/bpf_fentry_test1"]
pub unsafe extern "C" fn fentry_test1(ctx: *mut c_void, a: c_int) -> c_int {
    let _ = a;
    update(ctx, &mut fentry_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "fexit/bpf_fentry_test1"]
pub unsafe extern "C" fn fexit_test1(ctx: *mut c_void, a: c_int, ret: c_int) -> c_int {
    let _ = a;
    let _ = ret;
    update(ctx, &mut fexit_res as *mut u64);
    0
}

#[no_mangle]
#[link_section = "fmod_ret/bpf_modify_return_test"]
pub unsafe extern "C" fn fmod_ret_test(
    ctx: *mut c_void,
    _a: c_int,
    _b: *mut c_int,
    _ret: c_int,
) -> c_int {
    update(ctx, &mut fmod_ret_res as *mut u64);
    1234
}

#[no_mangle]
#[link_section = "lsm/file_mprotect"]
pub unsafe extern "C" fn test_int_hook(
    ctx: *mut c_void,
    vma: *mut vm_area_struct,
    reqprot: c_ulong,
    prot: c_ulong,
    ret: c_int,
) -> c_int {
    let _ = vma;
    let _ = reqprot;
    let _ = prot;

    if my_tid != bpf_get_current_pid_tgid() as u32 as c_int {
        return ret;
    }
    update(ctx, &mut lsm_res as *mut u64);
    -EPERM
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
