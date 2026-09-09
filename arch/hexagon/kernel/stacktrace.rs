// SPDX-License-Identifier: GPL-2.0-only
/*
 * Stacktrace support for Hexagon
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// Declarations supplied by the kernel headers and other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct stackframe {
    pub fp: c_ulong,
    pub rets: c_ulong,
}

#[repr(C)]
pub struct stack_trace {
    pub entries: *mut c_ulong,
    pub nr_entries: c_uint,
    pub max_entries: c_uint,
    pub skip: c_int,
}

pub type c_ulong = usize;
pub type c_uint = u32;
pub type c_int = i32;

extern "C" {
    static mut current: *mut c_void;
    fn task_stack_page(task: *mut c_void) -> *mut c_void;
}

// THREAD_SIZE is supplied by the target kernel configuration.
pub const THREAD_SIZE: c_ulong = 0;

extern "C" {
    fn __builtin_frame_address(level: c_int) -> *mut c_void;
}

/*
 * Save stack-backtrace addresses into a stack_trace buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    let mut low: c_ulong;
    let high: c_ulong;
    let mut fp: c_ulong;
    let mut frame: *mut stackframe;
    let mut skip: c_int = (*trace).skip;

    low = task_stack_page(current) as c_ulong;
    high = low.wrapping_add(THREAD_SIZE);
    fp = __builtin_frame_address(0) as c_ulong;

    while fp >= low
        && fp <= high.wrapping_sub(core::mem::size_of::<stackframe>())
    {
        frame = fp as *mut stackframe;

        if skip != 0 {
            skip -= 1;
        } else {
            let entry = (*trace).entries.add((*trace).nr_entries as usize);
            *entry = (*frame).rets;
            (*trace).nr_entries = (*trace).nr_entries.wrapping_add(1);
            if (*trace).nr_entries >= (*trace).max_entries {
                break;
            }
        }

        /*
         * The next frame must be at a higher address than the
         * current frame.
         */
        low = fp.wrapping_add(core::mem::size_of::<stackframe>());
        fp = (*frame).fp;
    }
}

// EXPORT_SYMBOL_GPL(save_stack_trace);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
