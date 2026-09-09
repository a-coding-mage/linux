/*
 * Kernel and userspace stack tracing.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 * Copyright (C) 2015 Cadence Design Systems Inc.
 */

// Linux and Xtensa declarations supplied by the surrounding kernel.

#[cfg(feature = "perf_events")]
extern "C" {
    static mut common_exception_return: ::core::ffi::c_int;
}

#[cfg(feature = "perf_events")]
pub unsafe extern "C" fn xtensa_backtrace_user(
    regs: *mut pt_regs,
    mut depth: ::core::ffi::c_uint,
    ufn: Option<unsafe extern "C" fn(*mut stackframe, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    data: *mut ::core::ffi::c_void,
) {
    let mut windowstart = (*regs).windowstart;
    let windowbase = (*regs).windowbase;
    let mut a0 = (*regs).areg[0];
    let mut a1 = (*regs).areg[1];
    let mut pc = (*regs).pc;
    let mut frame = stackframe { pc: 0, sp: 0 };
    let mut index: ::core::ffi::c_int;

    if depth == 0 {
        return;
    }
    depth -= 1;

    frame.pc = pc;
    frame.sp = a1;

    if pc == 0 || pc >= TASK_SIZE || ufn.unwrap()(&mut frame, data) != 0 {
        return;
    }

    if cfg!(feature = "user_abi_call0_only")
        || (cfg!(feature = "user_abi_call0_probe") && ((*regs).ps & PS_WOE_MASK) == 0)
    {
        return;
    }

    /* Two steps:
     *
     * 1. Look through the register window for the
     * previous PCs in the call trace.
     *
     * 2. Look on the stack.
     */

    /* Step 1.  */
    /* Rotate WINDOWSTART to move the bit corresponding to
     * the current window to the bit #0.
     */
    windowstart = (windowstart << WSBITS | windowstart) >> windowbase;

    /* Look for bits that are set, they correspond to
     * valid windows.
     */
    index = WSBITS - 1;
    while index > 0 && depth != 0 {
        depth -= 1;
        if windowstart & (1 << index) != 0 {
            /* Get the PC from a0 and a1. */
            pc = MAKE_PC_FROM_RA(a0, pc);
            /* Read a0 and a1 from the
             * corresponding position in AREGs.
             */
            a0 = (*regs).areg[(index * 4) as usize];
            a1 = (*regs).areg[(index * 4 + 1) as usize];

            frame.pc = pc;
            frame.sp = a1;

            if pc == 0 || pc >= TASK_SIZE || ufn.unwrap()(&mut frame, data) != 0 {
                return;
            }
        }
        index -= 1;
    }

    /* Step 2. */
    /* We are done with the register window, we need to
     * look through the stack.
     */
    if depth == 0 {
        return;
    }

    /* Start from the a1 register. */
    /* a1 = regs->areg[1]; */
    while a0 != 0 && depth != 0 {
        depth -= 1;
        pc = MAKE_PC_FROM_RA(a0, pc);

        /* Check if the region is OK to access. */
        if !access_ok(&SPILL_SLOT(a1, 0), 8) {
            return;
        }
        /* Copy a1, a0 from user space stack frame. */
        if __get_user(&mut a0, &SPILL_SLOT(a1, 0)) != 0
            || __get_user(&mut a1, &SPILL_SLOT(a1, 1)) != 0
        {
            return;
        }

        frame.pc = pc;
        frame.sp = a1;

        if pc == 0 || pc >= TASK_SIZE || ufn.unwrap()(&mut frame, data) != 0 {
            return;
        }
    }
}

#[cfg(feature = "perf_events")]
pub unsafe extern "C" fn xtensa_backtrace_kernel(
    mut regs: *mut pt_regs,
    mut depth: ::core::ffi::c_uint,
    kfn: Option<unsafe extern "C" fn(*mut stackframe, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    ufn: Option<unsafe extern "C" fn(*mut stackframe, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    data: *mut ::core::ffi::c_void,
) {
    let mut pc = if (*regs).depc > VALID_DOUBLE_EXCEPTION_ADDRESS { (*regs).depc } else { (*regs).pc };
    let mut a0 = (*regs).areg[0];
    let mut a1 = (*regs).areg[1];
    let mut sp_start = a1 & !(THREAD_SIZE - 1);
    let sp_end = sp_start + THREAD_SIZE;

    spill_registers();

    while a1 > sp_start && a1 < sp_end && depth != 0 {
        depth -= 1;
        let mut frame = stackframe { pc: 0, sp: 0 };
        frame.pc = pc;
        frame.sp = a1;

        if kernel_text_address(pc) && kfn.unwrap()(&mut frame, data) != 0 {
            return;
        }

        if pc == &common_exception_return as *const _ as usize {
            regs = a1 as *mut pt_regs;
            if user_mode(regs) {
                if ufn.is_none() {
                    return;
                }
                xtensa_backtrace_user(regs, depth, ufn, data);
                return;
            }
            a0 = (*regs).areg[0];
            a1 = (*regs).areg[1];
            continue;
        }

        sp_start = a1;
        pc = MAKE_PC_FROM_RA(a0, pc);
        a0 = SPILL_SLOT(a1, 0);
        a1 = SPILL_SLOT(a1, 1);
    }
}

pub unsafe extern "C" fn walk_stackframe(
    mut sp: *mut usize,
    fn_: Option<unsafe extern "C" fn(*mut stackframe, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    data: *mut ::core::ffi::c_void,
) {
    let mut a1 = sp as usize;
    let sp_end = ALIGN(a1, THREAD_SIZE);

    spill_registers();

    while a1 < sp_end {
        let mut frame = stackframe { pc: 0, sp: 0 };
        sp = a1 as *mut usize;
        let a0 = SPILL_SLOT(a1, 0);
        a1 = SPILL_SLOT(a1, 1);

        if a1 <= sp as usize {
            break;
        }

        frame.pc = MAKE_PC_FROM_RA(a0, _text);
        frame.sp = a1;

        if fn_.unwrap()(&mut frame, data) != 0 {
            return;
        }
    }
}

#[cfg(feature = "stacktrace")]
struct stack_trace_data {
    trace: *mut stack_trace,
    skip: usize,
}

#[cfg(feature = "stacktrace")]
unsafe extern "C" fn stack_trace_cb(frame: *mut stackframe, data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let trace_data = data as *mut stack_trace_data;
    let trace = (*trace_data).trace;

    if (*trace_data).skip != 0 {
        (*trace_data).skip -= 1;
        return 0;
    }
    if !kernel_text_address((*frame).pc) {
        return 0;
    }

    (*trace).entries[(*trace).nr_entries] = (*frame).pc;
    (*trace).nr_entries += 1;
    ((*trace).nr_entries >= (*trace).max_entries) as ::core::ffi::c_int
}

#[cfg(feature = "stacktrace")]
pub unsafe extern "C" fn save_stack_trace_tsk(task: *mut task_struct, trace: *mut stack_trace) {
    let mut trace_data = stack_trace_data {
        trace,
        skip: (*trace).skip,
    };
    walk_stackframe(stack_pointer(task), Some(stack_trace_cb), &mut trace_data as *mut _ as *mut _);
}

#[cfg(feature = "stacktrace")]
pub unsafe extern "C" fn save_stack_trace(trace: *mut stack_trace) {
    save_stack_trace_tsk(current, trace);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
