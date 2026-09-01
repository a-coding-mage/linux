// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>
// #include <stdbool.h>

unsafe extern "C" {
    // __kconfig __weak
    static CONFIG_X86_KERNEL_IBT: bool;
    // __kconfig __weak
    static CONFIG_PPC_FTRACE_OUT_OF_LINE: bool;
    // __kconfig __weak
    static CONFIG_KPROBES_ON_FTRACE: bool;
    // __kconfig __weak
    static CONFIG_PPC64: bool;
}

/* This function is here to have CONFIG_X86_KERNEL_IBT,
 * CONFIG_PPC_FTRACE_OUT_OF_LINE, CONFIG_KPROBES_ON_FTRACE,
 * CONFIG_PPC64 used and added to object BTF.
 */
#[no_mangle]
pub unsafe extern "C" fn unused() -> i32 {
    if unsafe {
        CONFIG_X86_KERNEL_IBT
            || CONFIG_PPC_FTRACE_OUT_OF_LINE
            || CONFIG_KPROBES_ON_FTRACE
            || CONFIG_PPC64
    } {
        0
    } else {
        1
    }
}

#[no_mangle]
#[link_section = "kprobe"]
pub extern "C" fn kprobe_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "uprobe"]
pub extern "C" fn uprobe_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "tracepoint"]
pub extern "C" fn tp_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "perf_event"]
pub extern "C" fn event_run(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "kprobe.multi"]
pub extern "C" fn kmulti_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "uprobe.multi"]
pub extern "C" fn umulti_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "fentry.multi"]
pub extern "C" fn tmulti_run() -> i32 {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
