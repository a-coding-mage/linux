// SPDX-License-Identifier: GPL-2.0-only
// C dependencies are supplied by the surrounding kernel translation.

#[cfg(all(CONFIG_FRAME_POINTER, not(CONFIG_ARM_UNWIND)))]
extern "C" {
    static mut call_with_stack_end: ::core::ffi::c_ulong;
    static mut call_with_stack: ::core::ffi::c_void;
}

#[cfg(all(CONFIG_FRAME_POINTER, not(CONFIG_ARM_UNWIND)))]
unsafe fn frame_pointer_check(frame: *mut stackframe) -> i32 {
    let fp = (*frame).fp;
    let pc = (*frame).pc;

    if pc >= (&call_with_stack as *const _ as usize) &&
        pc < (call_with_stack_end as usize)
    {
        return 0;
    }

    let low = (*frame).sp;
    let high = (low + (THREAD_SIZE - 1)) & !(THREAD_SIZE - 1);

    #[cfg(CONFIG_CC_IS_CLANG)]
    if fp < low + 4 || fp > high - 4 {
        return -EINVAL;
    }
    #[cfg(not(CONFIG_CC_IS_CLANG))]
    if fp < low + 12 || fp > high - 4 {
        return -EINVAL;
    }

    0
}

#[cfg(all(CONFIG_FRAME_POINTER, not(CONFIG_ARM_UNWIND)))]
pub unsafe fn unwind_frame(frame: *mut stackframe) -> i32 {
    let fp = (*frame).fp;

    if frame_pointer_check(frame) != 0 {
        return -EINVAL;
    }

    if (*frame).ex_frame {
        let regs = (*frame).sp as *mut pt_regs;
        if (regs.add(1) as usize) > (((*frame).sp + (THREAD_SIZE - 1)) & !(THREAD_SIZE - 1)) {
            return -EINVAL;
        }
        (*frame).pc = (*regs).ARM_pc;
        (*frame).ex_frame = false;
        return 0;
    }

    #[cfg(CONFIG_CC_IS_CLANG)]
    {
        (*frame).sp = (*frame).fp;
        (*frame).fp = core::ptr::read_volatile(fp as *const ::core::ffi::c_ulong);
        (*frame).pc = core::ptr::read_volatile((fp + 4) as *const ::core::ffi::c_ulong);
    }
    #[cfg(not(CONFIG_CC_IS_CLANG))]
    {
        (*frame).fp = core::ptr::read_volatile((fp - 12) as *const ::core::ffi::c_ulong);
        (*frame).sp = core::ptr::read_volatile((fp - 8) as *const ::core::ffi::c_ulong);
        (*frame).pc = core::ptr::read_volatile((fp - 4) as *const ::core::ffi::c_ulong);
    }

    #[cfg(CONFIG_KRETPROBES)]
    if is_kretprobe_trampoline((*frame).pc) {
        (*frame).pc = kretprobe_find_ret_addr((*frame).tsk, (*frame).fp as *mut _, &mut (*frame).kr_cur);
    }

    if in_entry_text((*frame).pc) {
        (*frame).ex_frame = true;
    }
    0
}

pub unsafe fn walk_stackframe(
    frame: *mut stackframe,
    fn_: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, usize) -> bool>,
    data: *mut ::core::ffi::c_void,
) {
    loop {
        if !fn_.expect("callback is non-null")(data, (*frame).pc) {
            break;
        }
        if unwind_frame(frame) < 0 {
            break;
        }
    }
}

#[cfg(CONFIG_STACKTRACE)]
unsafe fn start_stack_trace(
    frame: *mut stackframe,
    task: *mut task_struct,
    fp: usize,
    sp: usize,
    lr: usize,
    pc: usize,
) {
    (*frame).fp = fp;
    (*frame).sp = sp;
    (*frame).lr = lr;
    (*frame).pc = pc;
    #[cfg(CONFIG_KRETPROBES)]
    {
        (*frame).kr_cur = core::ptr::null_mut();
        (*frame).tsk = task;
    }
    #[cfg(CONFIG_UNWINDER_FRAME_POINTER)]
    {
        (*frame).ex_frame = in_entry_text((*frame).pc);
    }
}

#[cfg(CONFIG_STACKTRACE)]
pub unsafe fn arch_stack_walk(
    consume_entry: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, usize) -> bool>,
    cookie: *mut ::core::ffi::c_void,
    task: *mut task_struct,
    regs: *mut pt_regs,
) {
    let mut frame: stackframe = core::mem::zeroed();
    if !regs.is_null() {
        start_stack_trace(&mut frame, core::ptr::null_mut(), (*regs).ARM_fp,
                          (*regs).ARM_sp, (*regs).ARM_lr, (*regs).ARM_pc);
    } else if task != current {
        #[cfg(CONFIG_SMP)]
        return;
        #[cfg(not(CONFIG_SMP))]
        start_stack_trace(&mut frame, task, thread_saved_fp(task), thread_saved_sp(task),
                          0, thread_saved_pc(task));
    } else {
        start_stack_trace(&mut frame, task, __builtin_frame_address(0), current_stack_pointer,
                          __builtin_return_address(0), stacktrace_here as usize);
        if unwind_frame(&mut frame) != 0 {
            return;
        }
    }
    walk_stackframe(&mut frame, consume_entry, cookie);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
