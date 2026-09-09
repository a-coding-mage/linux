/*
 * Stack trace utility for OpenRISC
 *
 * Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 *
 * Losely based on work from sh and powerpc.
 */

// Kernel and architecture declarations are supplied by the surrounding crate.

extern "C" {
    fn unwind_stack(
        trace: *mut stack_trace,
        sp: *mut ::core::ffi::c_ulong,
        callback: unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_ulong, ::core::ffi::c_int),
    );
    fn in_sched_functions(addr: ::core::ffi::c_ulong) -> bool;
    fn try_get_task_stack(tsk: *mut task_struct) -> bool;
    fn put_task_stack(tsk: *mut task_struct);
    fn task_thread_info(tsk: *mut task_struct) -> *mut thread_info;
    static mut current: *mut task_struct;
}

/*
 * Save stack-backtrace addresses into a stack_trace buffer.
 */
unsafe extern "C" fn save_stack_address(
    data: *mut ::core::ffi::c_void,
    addr: ::core::ffi::c_ulong,
    reliable: ::core::ffi::c_int,
) {
    let trace = data as *mut stack_trace;

    if reliable == 0 {
        return;
    }

    if (*trace).skip > 0 {
        (*trace).skip -= 1;
        return;
    }

    if (*trace).nr_entries < (*trace).max_entries {
        let index = (*trace).nr_entries as usize;
        (*trace).entries[index] = addr;
        (*trace).nr_entries += 1;
    }
}

pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    unwind_stack(
        trace,
        (&trace as *const *mut stack_trace) as *mut ::core::ffi::c_ulong,
        save_stack_address,
    );
}

unsafe extern "C" fn save_stack_address_nosched(
    data: *mut ::core::ffi::c_void,
    addr: ::core::ffi::c_ulong,
    reliable: ::core::ffi::c_int,
) {
    let trace = data as *mut stack_trace;

    if reliable == 0 {
        return;
    }

    if in_sched_functions(addr) {
        return;
    }

    if (*trace).skip > 0 {
        (*trace).skip -= 1;
        return;
    }

    if (*trace).nr_entries < (*trace).max_entries {
        let index = (*trace).nr_entries as usize;
        (*trace).entries[index] = addr;
        (*trace).nr_entries += 1;
    }
}

pub unsafe extern "C" fn save_stack_trace_tsk(
    tsk: *mut task_struct,
    trace: *mut stack_trace,
) {
    let mut sp: *mut ::core::ffi::c_ulong = ::core::ptr::null_mut();

    if !try_get_task_stack(tsk) {
        return;
    }

    if tsk == current {
        sp = (&sp as *const *mut ::core::ffi::c_ulong) as *mut ::core::ffi::c_ulong;
    } else {
        let mut ksp: ::core::ffi::c_ulong;

        /* Locate stack from kernel context */
        ksp = (*task_thread_info(tsk)).ksp;
        ksp += STACK_FRAME_OVERHEAD;
        ksp += ::core::mem::size_of::<pt_regs>() as ::core::ffi::c_ulong;

        sp = ksp as *mut ::core::ffi::c_ulong;
    }

    unwind_stack(trace, sp, save_stack_address_nosched);

    put_task_stack(tsk);
}

pub unsafe extern "C" fn save_stack_trace_regs(
    regs: *mut pt_regs,
    trace: *mut stack_trace,
) {
    unwind_stack(
        trace,
        (*regs).sp as *mut ::core::ffi::c_ulong,
        save_stack_address_nosched,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
