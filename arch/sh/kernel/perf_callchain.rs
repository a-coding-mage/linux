// SPDX-License-Identifier: GPL-2.0
/*
 * Performance event callchain support - SuperH architecture code
 *
 * Copyright (C) 2009  Paul Mundt
 */

use core::ffi::{c_int, c_ulong, c_void};

// Declarations supplied by the Linux kernel and SuperH architecture headers.
#[repr(C)]
pub struct perf_callchain_entry_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    pub pc: c_ulong,
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(data: *mut c_void, addr: c_ulong, reliable: c_int)>,
}

unsafe extern "C" {
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, addr: c_ulong);
    fn unwind_stack(
        task: *mut c_void,
        regs: *mut pt_regs,
        stack: *mut c_void,
        ops: *const stacktrace_ops,
        data: *mut c_void,
    );
}

unsafe extern "C" fn callchain_address(data: *mut c_void, addr: c_ulong, reliable: c_int) {
    let entry = data as *mut perf_callchain_entry_ctx;

    if reliable != 0 {
        unsafe {
            perf_callchain_store(entry, addr);
        }
    }
}

static callchain_ops: stacktrace_ops = stacktrace_ops {
    address: Some(callchain_address),
};

pub unsafe extern "C" fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    unsafe {
        perf_callchain_store(entry, (*regs).pc);

        unwind_stack(
            core::ptr::null_mut(),
            regs,
            core::ptr::null_mut(),
            &callchain_ops,
            entry as *mut c_void,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
