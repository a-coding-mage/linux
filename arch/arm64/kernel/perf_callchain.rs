// SPDX-License-Identifier: GPL-2.0-only
/*
 * arm64 callchain support
 *
 * Copyright (C) 2015 ARM Limited
 */

use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct perf_callchain_entry_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, pc: c_ulong) -> c_int;
    fn perf_guest_state() -> bool;
    fn arch_stack_walk_user(
        callback: unsafe fn(data: *mut c_void, pc: c_ulong) -> bool,
        data: *mut c_void,
        regs: *mut pt_regs,
    );
    fn arch_stack_walk(
        callback: unsafe fn(data: *mut c_void, pc: c_ulong) -> bool,
        data: *mut c_void,
        task: *mut task_struct,
        regs: *mut pt_regs,
    );
    static mut current: *mut task_struct;
}

unsafe fn callchain_trace(data: *mut c_void, pc: c_ulong) -> bool {
    let entry = data as *mut perf_callchain_entry_ctx;

    unsafe { perf_callchain_store(entry, pc) == 0 }
}

pub unsafe fn perf_callchain_user(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    if unsafe { perf_guest_state() } {
        /* We don't support guest os callchain now */
        return;
    }

    unsafe {
        arch_stack_walk_user(callchain_trace, entry as *mut c_void, regs);
    }
}

pub unsafe fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    if unsafe { perf_guest_state() } {
        /* We don't support guest os callchain now */
        return;
    }

    unsafe {
        arch_stack_walk(callchain_trace, entry as *mut c_void, current, regs);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
