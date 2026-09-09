// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.

use core::ffi::c_void;

// Supplied by the kernel headers/build environment.
extern "C" {
    static mut current: *mut c_void;
    static THREAD_SIZE: usize;
    static EPERM: i32;

    fn task_stack_page(task: *mut c_void) -> *mut c_void;
    fn kstack_end(addr: *const c_void) -> bool;
    fn kernel_text_address(addr: usize) -> bool;
    fn ftrace_graph_ret_addr(
        task: *mut c_void,
        graph: *mut i32,
        ret: usize,
        err: *mut c_void,
    ) -> usize;
    fn access_ok(addr: *const c_void, size: usize) -> bool;
    fn __copy_from_user_inatomic(to: *mut c_void, from: *const c_void, size: usize) -> usize;
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, ip: usize);
}

/* Kernel callchain */
#[repr(C)]
struct stackframe {
    fp: usize,
    lr: usize,
}

#[repr(C)]
struct perf_callchain_entry_ctx {
    nr: usize,
    max_stack: usize,
}

#[repr(C)]
struct pt_regs {
    regs: [usize; 16],
    pc: usize,
    lr: usize,
}

unsafe fn unwind_frame_kernel(frame: *mut stackframe) -> i32 {
    let low = task_stack_page(current) as usize;
    let high = low.wrapping_add(THREAD_SIZE);

    if (*frame).fp < low || (*frame).fp > high {
        return -EPERM;
    }

    if kstack_end((*frame).fp as *const c_void) || ((*frame).fp & 0x3) != 0 {
        return -EPERM;
    }

    *frame = *((*frame).fp as *const stackframe);

    if kernel_text_address((*frame).lr) {
        let mut graph = 0;
        (*frame).lr = ftrace_graph_ret_addr(
            core::ptr::null_mut(),
            &mut graph,
            (*frame).lr,
            core::ptr::null_mut(),
        );
    }
    0
}

unsafe fn walk_stackframe(fr: *mut stackframe, entry: *mut perf_callchain_entry_ctx) {
    loop {
        perf_callchain_store(entry, (*fr).lr);
        if unwind_frame_kernel(fr) < 0 {
            break;
        }
    }
}

/*
 * Get the return address for a single stackframe and return a pointer to the
 * next frame tail.
 */
unsafe fn user_backtrace(
    entry: *mut perf_callchain_entry_ctx,
    mut fp: usize,
    reg_lr: usize,
) -> usize {
    let mut buftail: stackframe = core::mem::zeroed();
    let mut lr = 0;
    let user_frame_tail = fp as *mut usize;

    /* Check accessibility of one struct frame_tail beyond */
    if !access_ok(user_frame_tail as *const c_void, core::mem::size_of::<stackframe>()) {
        return 0;
    }
    if __copy_from_user_inatomic(
        &mut buftail as *mut stackframe as *mut c_void,
        user_frame_tail as *const c_void,
        core::mem::size_of::<stackframe>(),
    ) != 0 {
        return 0;
    }

    if reg_lr != 0 {
        lr = reg_lr;
    } else {
        lr = buftail.lr;
    }

    fp = buftail.fp;
    perf_callchain_store(entry, lr);

    fp
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
 * On C-SKY platform, the program being sampled and the C library
 * need to be compiled with * -mbacktrace, otherwise the user
 * stack will not contain function frame.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_callchain_user(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut fp = 0;

    fp = (*regs).regs[4];
    perf_callchain_store(entry, (*regs).pc);

    /*
     * While backtrace from leaf function, lr is normally
     * not saved inside frame on C-SKY, so get lr from pt_regs
     * at the sample point. However, lr value can be incorrect if
     * lr is used as temp register
     */
    fp = user_backtrace(entry, fp, (*regs).lr);

    while fp != 0 && (fp & 0x3) == 0 && (*entry).nr < (*entry).max_stack {
        fp = user_backtrace(entry, fp, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut fr = stackframe {
        fp: (*regs).regs[4],
        lr: (*regs).lr,
    };

    walk_stackframe(&mut fr, entry);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
