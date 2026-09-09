// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_FRAME_POINTER)]
#[repr(C)]
struct stackframe {
    fp: ::core::ffi::c_ulong,
    ra: ::core::ffi::c_ulong,
}

#[cfg(CONFIG_FRAME_POINTER)]
pub unsafe fn walk_stackframe(
    task: *mut task_struct,
    regs: *mut pt_regs,
    fn_: Option<unsafe extern "C" fn(::core::ffi::c_ulong, *mut ::core::ffi::c_void) -> bool>,
    arg: *mut ::core::ffi::c_void,
) {
    let (mut fp, mut sp, mut pc): (
        ::core::ffi::c_ulong,
        ::core::ffi::c_ulong,
        ::core::ffi::c_ulong,
    );

    if !regs.is_null() {
        fp = frame_pointer(regs);
        sp = user_stack_pointer(regs);
        pc = instruction_pointer(regs);
    } else if task.is_null() || task == current {
        let current_fp: ::core::ffi::c_ulong;
        core::arch::asm!("", out("r8") current_fp);
        fp = current_fp;
        sp = current_stack_pointer;
        pc = walk_stackframe as usize as ::core::ffi::c_ulong;
    } else {
        // task blocked in __switch_to
        fp = thread_saved_fp(task);
        sp = thread_saved_sp(task);
        pc = thread_saved_lr(task);
    }

    loop {
        let low: ::core::ffi::c_ulong;
        let high: ::core::ffi::c_ulong;
        let frame: *mut stackframe;

        if !__kernel_text_address(pc) || fn_.unwrap()(pc, arg) {
            break;
        }

        // Validate frame pointer
        low = sp;
        high = (sp + (THREAD_SIZE - 1)) & !(THREAD_SIZE - 1);
        if fp < low || fp > high || (fp & 0x3) != 0 {
            break;
        }
        // Unwind stack frame
        frame = fp as *mut stackframe;
        sp = fp;
        fp = (*frame).fp;
        pc = ftrace_graph_ret_addr(
            current,
            core::ptr::null_mut(),
            (*frame).ra,
            (fp.wrapping_sub(8)) as *mut ::core::ffi::c_ulong,
        );
    }
}

// !CONFIG_FRAME_POINTER: the frame-pointer implementation above is omitted.
#[cfg(not(CONFIG_FRAME_POINTER))]
unsafe fn walk_stackframe(
    task: *mut task_struct,
    regs: *mut pt_regs,
    fn_: Option<unsafe extern "C" fn(::core::ffi::c_ulong, *mut ::core::ffi::c_void) -> bool>,
    arg: *mut ::core::ffi::c_void,
) {
    let (mut sp, mut pc): (::core::ffi::c_ulong, ::core::ffi::c_ulong);
    let mut ksp: *mut ::core::ffi::c_ulong;

    if !regs.is_null() {
        sp = user_stack_pointer(regs);
        pc = instruction_pointer(regs);
    } else if task.is_null() || task == current {
        sp = current_stack_pointer;
        pc = walk_stackframe as usize as ::core::ffi::c_ulong;
    } else {
        // task blocked in __switch_to
        sp = thread_saved_sp(task);
        pc = thread_saved_lr(task);
    }

    if (sp & 0x3) != 0 {
        return;
    }

    ksp = sp as *mut ::core::ffi::c_ulong;
    while !kstack_end(ksp) {
        if __kernel_text_address(pc) && fn_.unwrap()(pc, arg) {
            break;
        }
        pc = (*ksp) - 0x4;
        ksp = ksp.add(1);
    }
}

unsafe extern "C" fn print_trace_address(
    pc: ::core::ffi::c_ulong,
    arg: *mut ::core::ffi::c_void,
) -> bool {
    print_ip_sym(arg as *const ::core::ffi::c_char, pc);
    false
}

pub unsafe fn show_stack(
    task: *mut task_struct,
    _sp: *mut ::core::ffi::c_ulong,
    loglvl: *const ::core::ffi::c_char,
) {
    pr_cont("Call Trace:\n");
    walk_stackframe(task, core::ptr::null_mut(), Some(print_trace_address), loglvl as *mut _);
}

unsafe extern "C" fn save_wchan(
    pc: ::core::ffi::c_ulong,
    arg: *mut ::core::ffi::c_void,
) -> bool {
    if !in_sched_functions(pc) {
        *(arg as *mut ::core::ffi::c_ulong) = pc;
        return true;
    }
    false
}

pub unsafe fn __get_wchan(task: *mut task_struct) -> ::core::ffi::c_ulong {
    let mut pc: ::core::ffi::c_ulong = 0;
    walk_stackframe(task, core::ptr::null_mut(), Some(save_wchan), &mut pc as *mut _ as *mut _);
    pc
}

#[cfg(CONFIG_STACKTRACE)]
unsafe fn __save_trace(
    pc: ::core::ffi::c_ulong,
    arg: *mut ::core::ffi::c_void,
    nosched: bool,
) -> bool {
    let trace = &mut *(arg as *mut stack_trace);

    if nosched && in_sched_functions(pc) {
        return false;
    }
    if trace.skip > 0 {
        trace.skip -= 1;
        return false;
    }

    *trace.entries.add(trace.nr_entries as usize) = pc;
    trace.nr_entries += 1;
    trace.nr_entries >= trace.max_entries
}

#[cfg(CONFIG_STACKTRACE)]
unsafe extern "C" fn save_trace(
    pc: ::core::ffi::c_ulong,
    arg: *mut ::core::ffi::c_void,
) -> bool {
    __save_trace(pc, arg, false)
}

// Save stack-backtrace addresses into a stack_trace buffer.
#[cfg(CONFIG_STACKTRACE)]
pub unsafe fn save_stack_trace_tsk(task: *mut task_struct, trace: *mut stack_trace) {
    walk_stackframe(task, core::ptr::null_mut(), Some(save_trace), trace as *mut _);
}

#[cfg(CONFIG_STACKTRACE)]
pub unsafe fn save_stack_trace(trace: *mut stack_trace) {
    save_stack_trace_tsk(core::ptr::null_mut(), trace);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
