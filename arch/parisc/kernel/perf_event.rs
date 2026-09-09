// SPDX-License-Identifier: GPL-2.0
/*
 * Performance event support for parisc
 *
 * Copyright (C) 2025 by Helge Deller <deller@gmx.de>
 */

use core::ffi::c_void;

// Supplied by the Linux kernel headers and other translation units.
#[repr(C)]
pub struct perf_callchain_entry_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unwind_frame_info {
    pub ip: usize,
}

extern "C" {
    static mut current: *mut c_void;

    fn unwind_frame_init_task(
        info: *mut unwind_frame_info,
        task: *mut c_void,
        stack: *mut c_void,
    );
    fn unwind_once(info: *mut unwind_frame_info) -> i32;
    fn __kernel_text_address(addr: usize) -> bool;
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, ip: usize) -> bool;
}

pub unsafe fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut info: unwind_frame_info = core::mem::zeroed();

    unwind_frame_init_task(&mut info, current, core::ptr::null_mut());
    let _ = regs;
    loop {
        if unwind_once(&mut info) < 0 || info.ip == 0 {
            break;
        }

        if !__kernel_text_address(info.ip) || perf_callchain_store(entry, info.ip) {
            return;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
