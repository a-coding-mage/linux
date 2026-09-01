// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2017 Facebook

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <bpf/bpf_core_read.h>
// #include "bpf_misc.h"

use core::ffi::c_void;

pub static mut kprobe_res: i32 = 0;
pub static mut kretprobe_res: i32 = 0;
pub static mut uprobe_res: i32 = 0;
pub static mut uretprobe_res: i32 = 0;
pub static mut uprobe_byname_res: i32 = 0;
pub static mut user_ptr: *mut c_void = core::ptr::null_mut();

#[no_mangle]
#[link_section = "kprobe"]
pub unsafe extern "C" fn handle_kprobe(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    kprobe_res = 1;
    0
}

#[no_mangle]
#[link_section = "kretprobe"]
pub unsafe extern "C" fn handle_kretprobe(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    kretprobe_res = 2;
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    uprobe_res = 3;
    0
}

#[no_mangle]
#[link_section = "uretprobe"]
pub unsafe extern "C" fn handle_uretprobe(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    uretprobe_res = 4;
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn handle_uprobe_byname(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    uprobe_byname_res = 5;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
