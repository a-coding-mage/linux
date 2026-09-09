// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd. */

use core::ffi::c_void;

// Declarations supplied by the Linux perf and RISC-V stacktrace dependencies.
#[repr(C)]
pub struct perf_callchain_entry_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    fn perf_callchain_store(entry: *mut c_void, pc: usize) -> i32;
    fn perf_guest_state() -> bool;
    fn arch_stack_walk(
        callback: unsafe fn(*mut c_void, usize) -> bool,
        entry: *mut c_void,
        regs: *mut pt_regs,
    );
    fn walk_stackframe(
        task: *mut c_void,
        regs: *mut pt_regs,
        callback: unsafe fn(*mut c_void, usize) -> bool,
        entry: *mut c_void,
    );
}

unsafe fn fill_callchain(entry: *mut c_void, pc: usize) -> bool {
    unsafe { perf_callchain_store(entry, pc) == 0 }
}

/*
 * This will be called when the target is in user mode
 * This function will only be called when we use
 * "PERF_SAMPLE_CALLCHAIN" in
 * kernel/events/core.c:perf_prepare_sample()
 *
 * How to trigger perf_callchain_[user/kernel] :
 * $ perf record -e cpu-clock --call-graph fp ./program
 * $ perf report --call-graph
 *
 * On RISC-V platform, the program being sampled and the C library
 * need to be compiled with -fno-omit-frame-pointer, otherwise
 * the user stack will not contain function frame.
 */
pub unsafe fn perf_callchain_user(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    if unsafe { perf_guest_state() } {
        /* TODO: We don't support guest os callchain now */
        return;
    }

    unsafe {
        arch_stack_walk(
            fill_callchain,
            entry.cast::<c_void>(),
            regs,
        );
    }
}

pub unsafe fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    if unsafe { perf_guest_state() } {
        /* TODO: We don't support guest os callchain now */
        return;
    }

    unsafe {
        walk_stackframe(
            core::ptr::null_mut(),
            regs,
            fill_callchain,
            entry.cast::<c_void>(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
