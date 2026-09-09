/*
 * OpenRISC unwinder.c
 *
 * Reusable arch specific api for unwinding stacks.
 *
 * Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// C dependencies: linux/sched/task_stack.h, linux/kernel.h, asm/unwinder.h

extern "C" {
    fn kstack_end(addr: *mut ::core::ffi::c_ulong) -> bool;
    fn __kernel_text_address(addr: ::core::ffi::c_ulong) -> bool;
}

#[cfg(CONFIG_FRAME_POINTER)]
#[repr(C)]
struct or1k_frameinfo {
    fp: *mut ::core::ffi::c_ulong,
    ra: ::core::ffi::c_ulong,
    top: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_FRAME_POINTER)]
#[inline]
unsafe fn or1k_frameinfo_valid(frameinfo: *mut or1k_frameinfo) -> i32 {
    ((*frameinfo).fp.is_null()
        || (!kstack_end((*frameinfo).fp)
            && (*frameinfo).fp > &mut (*frameinfo).top as *mut _))
        as i32
        & __kernel_text_address((*frameinfo).ra) as i32
}

#[cfg(CONFIG_FRAME_POINTER)]
pub unsafe fn unwind_stack(
    data: *mut ::core::ffi::c_void,
    mut stack: *mut ::core::ffi::c_ulong,
    trace: Option<unsafe extern "C" fn(
        data: *mut ::core::ffi::c_void,
        addr: ::core::ffi::c_ulong,
        reliable: i32,
    )>,
) {
    let mut next_fp: *mut ::core::ffi::c_ulong = core::ptr::null_mut();
    let mut frameinfo: *mut or1k_frameinfo;
    let mut reliable: i32 = 0;

    while !kstack_end(stack) {
        // container_of(stack, struct or1k_frameinfo, top)
        frameinfo = (stack as *mut u8)
            .offset(-(core::mem::offset_of!(or1k_frameinfo, top) as isize))
            as *mut or1k_frameinfo;

        if __kernel_text_address((*frameinfo).ra) {
            if or1k_frameinfo_valid(frameinfo) != 0
                && (next_fp.is_null()
                    || next_fp == &mut (*frameinfo).top as *mut _) {
                reliable = 1;
                next_fp = (*frameinfo).fp;
            } else {
                reliable = 0;
            }

            if let Some(trace_fn) = trace {
                trace_fn(data, (*frameinfo).ra, reliable);
            }
        }
        stack = stack.add(1);
    }
}

#[cfg(not(CONFIG_FRAME_POINTER))]
pub unsafe fn unwind_stack(
    data: *mut ::core::ffi::c_void,
    mut stack: *mut ::core::ffi::c_ulong,
    trace: Option<unsafe extern "C" fn(
        data: *mut ::core::ffi::c_void,
        addr: ::core::ffi::c_ulong,
        reliable: i32,
    )>,
) {
    let mut addr: ::core::ffi::c_ulong;

    while !kstack_end(stack) {
        addr = *stack;
        stack = stack.add(1);
        if __kernel_text_address(addr) {
            if let Some(trace_fn) = trace {
                trace_fn(data, addr, 0);
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
