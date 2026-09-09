// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/stacktrace.c
 *
 * Stack trace management functions
 *
 *  Copyright (C) 2006 - 2008  Paul Mundt
 */

// The declarations below correspond to the types and symbols supplied by the
// Linux scheduler, stacktrace, thread-info, module, unwinder, ptrace, and
// architecture stacktrace headers.

#[repr(C)]
pub struct stack_trace {
    pub entries: *mut ::std::os::raw::c_ulong,
    pub nr_entries: ::std::os::raw::c_uint,
    pub max_entries: ::std::os::raw::c_uint,
    pub skip: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub sp: ::std::os::raw::c_ulong,
}

#[repr(C)]
pub struct stacktrace_ops {
    pub address: Option<unsafe extern "C" fn(
        data: *mut ::std::ffi::c_void,
        addr: ::std::os::raw::c_ulong,
        reliable: ::std::os::raw::c_int,
    )>,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut current_stack_pointer: ::std::os::raw::c_ulong;

    fn unwind_stack(
        task: *mut task_struct,
        regs: *mut ::std::ffi::c_void,
        sp: *mut ::std::os::raw::c_ulong,
        ops: *const stacktrace_ops,
        data: *mut ::std::ffi::c_void,
    );
    fn in_sched_functions(addr: ::std::os::raw::c_ulong) -> bool;
}

/*
 * Save stack-backtrace addresses into a stack_trace buffer.
 */
unsafe extern "C" fn save_stack_address(
    data: *mut ::std::ffi::c_void,
    addr: ::std::os::raw::c_ulong,
    reliable: ::std::os::raw::c_int,
) {
    let trace = &mut *(data as *mut stack_trace);

    if reliable == 0 {
        return;
    }

    if trace.skip > 0 {
        trace.skip -= 1;
        return;
    }

    if trace.nr_entries < trace.max_entries {
        *trace.entries.add(trace.nr_entries as usize) = addr;
        trace.nr_entries += 1;
    }
}

static save_stack_ops: stacktrace_ops = stacktrace_ops {
    address: Some(save_stack_address),
};

pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    let sp = current_stack_pointer as *mut ::std::os::raw::c_ulong;

    unwind_stack(
        current,
        ::std::ptr::null_mut(),
        sp,
        &save_stack_ops,
        trace as *mut ::std::ffi::c_void,
    );
}

// EXPORT_SYMBOL_GPL(save_stack_trace);

unsafe extern "C" fn save_stack_address_nosched(
    data: *mut ::std::ffi::c_void,
    addr: ::std::os::raw::c_ulong,
    reliable: ::std::os::raw::c_int,
) {
    let trace = &mut *(data as *mut stack_trace);

    if reliable == 0 {
        return;
    }

    if in_sched_functions(addr) {
        return;
    }

    if trace.skip > 0 {
        trace.skip -= 1;
        return;
    }

    if trace.nr_entries < trace.max_entries {
        *trace.entries.add(trace.nr_entries as usize) = addr;
        trace.nr_entries += 1;
    }
}

static save_stack_ops_nosched: stacktrace_ops = stacktrace_ops {
    address: Some(save_stack_address_nosched),
};

pub unsafe extern "C" fn save_stack_trace_tsk(
    tsk: *mut task_struct,
    trace: *mut stack_trace,
) {
    let sp = (*tsk).thread.sp as *mut ::std::os::raw::c_ulong;

    unwind_stack(
        current,
        ::std::ptr::null_mut(),
        sp,
        &save_stack_ops_nosched,
        trace as *mut ::std::ffi::c_void,
    );
}

// EXPORT_SYMBOL_GPL(save_stack_trace_tsk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
