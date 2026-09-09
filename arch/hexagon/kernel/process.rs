// SPDX-License-Identifier: GPL-2.0-only
/*
 * Process creation support for Hexagon
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

/* Dependencies are supplied by the surrounding kernel translation. */

/*
 * Program thread launch.  Often defined as a macro in processor.h,
 * but we're shooting for a small footprint and it's not an inner-loop
 * performance-critical operation.
 *
 * The Hexagon ABI specifies that R28 is zero'ed before program launch,
 * so that gets automatically done here.  If we ever stop doing that here,
 * we'll probably want to define the ELF_PLAT_INIT macro.
 */
pub unsafe fn start_thread(regs: *mut pt_regs, pc: ::core::ffi::c_ulong, sp: ::core::ffi::c_ulong) {
    /* We want to zero all data-containing registers. Is this overkill? */
    memset(regs.cast(), 0, core::mem::size_of::<pt_regs>());
    /* We might want to also zero all Processor registers here */
    pt_set_usermode(regs);
    pt_set_elr(regs, pc);
    pt_set_rte_sp(regs, sp);
}

/*
 *  Spin, or better still, do a hardware or VM wait instruction
 *  If hardware or VM offer wait termination even though interrupts
 *  are disabled.
 */
pub unsafe fn arch_cpu_idle() {
    __vmwait();
    /*  interrupts wake us up, but irqs are still disabled */
}

/* Copy architecture-specific thread state */
pub unsafe fn copy_thread(
    p: *mut task_struct,
    args: *const kernel_clone_args,
) -> ::core::ffi::c_int {
    let clone_flags: u64 = (*args).flags;
    let usp: ::core::ffi::c_ulong = (*args).stack;
    let tls: ::core::ffi::c_ulong = (*args).tls;
    let ti: *mut thread_info = task_thread_info(p);
    let ss: *mut hexagon_switch_stack;
    let childregs: *mut pt_regs;
    unsafe extern "C" {
        fn ret_from_fork();
    }

    childregs = (((ti as ::core::ffi::c_ulong) + THREAD_SIZE)
        - core::mem::size_of::<pt_regs>() as ::core::ffi::c_ulong) as *mut pt_regs;

    (*ti).regs = childregs;

    /*
     * Establish kernel stack pointer and initial PC for new thread
     * Note that unlike the usual situation, we do not copy the
     * parent's callee-saved here; those are in pt_regs and whatever
     * we leave here will be overridden on return to userland.
     */
    ss = ((childregs as ::core::ffi::c_ulong)
        - core::mem::size_of::<hexagon_switch_stack>() as ::core::ffi::c_ulong)
        as *mut hexagon_switch_stack;
    (*ss).lr = ret_from_fork as usize as ::core::ffi::c_ulong;
    (*p).thread.switch_sp = ss;
    if unlikely((*args).fn_.is_some()) {
        memset(childregs.cast(), 0, core::mem::size_of::<pt_regs>());
        /* r24 <- fn, r25 <- arg */
        (*ss).r24 = (*args).fn_.map_or(0, |f| f as usize as ::core::ffi::c_ulong);
        (*ss).r25 = (*args).fn_arg;
        pt_set_kmode(childregs);
        return 0;
    }
    memcpy(childregs.cast(), current_pt_regs().cast(), core::mem::size_of::<pt_regs>());
    (*ss).r2524 = 0;

    if usp != 0 {
        pt_set_rte_sp(childregs, usp);
    }

    /* Child sees zero return value */
    (*childregs).r00 = 0;

    /* The clone syscall has the C signature; ugp is used to provide TLS support. */
    if clone_flags & CLONE_SETTLS != 0 {
        (*childregs).ugp = tls;
    }

    /* Parent sees new pid -- not necessary, not even possible at this point in the fork process */
    0
}

/* Some archs flush debug and FPU info here */
pub unsafe fn flush_thread() {}

/*
 * The "wait channel" terminology is archaic, but what we want
 * is an identification of the point at which the scheduler
 * was invoked by a blocked thread.
 */
pub unsafe fn __get_wchan(p: *mut task_struct) -> ::core::ffi::c_ulong {
    let mut fp: ::core::ffi::c_ulong;
    let mut pc: ::core::ffi::c_ulong;
    let stack_page: ::core::ffi::c_ulong;
    let mut count = 0;

    stack_page = task_stack_page(p) as ::core::ffi::c_ulong;
    fp = (*( (*p).thread.switch_sp as *mut hexagon_switch_stack)).fp;
    loop {
        if fp < stack_page + core::mem::size_of::<thread_info>() as ::core::ffi::c_ulong
            || fp >= THREAD_SIZE - 8 + stack_page
        {
            return 0;
        }
        pc = *((fp as *mut ::core::ffi::c_ulong).add(1));
        if !in_sched_functions(pc) {
            return pc;
        }
        fp = *(fp as *mut ::core::ffi::c_ulong);
        count += 1;
        if count >= 16 {
            break;
        }
    }
    0
}

/* Called on the exit path of event entry; see vm_entry.S. Interrupts are disabled. */
pub unsafe fn do_work_pending(regs: *mut pt_regs, thread_info_flags: u32) -> ::core::ffi::c_int {
    if thread_info_flags & _TIF_WORK_MASK == 0 {
        return 0;
    } /* shortcut -- no work to be done */

    local_irq_enable();

    if thread_info_flags & _TIF_NEED_RESCHED != 0 {
        schedule();
        return 1;
    }
    if thread_info_flags & (_TIF_SIGPENDING | _TIF_NOTIFY_SIGNAL) != 0 {
        do_signal(regs);
        return 1;
    }
    if thread_info_flags & _TIF_NOTIFY_RESUME != 0 {
        resume_user_mode_work(regs);
        return 1;
    }

    /* Should not even reach here */
    panic("%s: bad thread_info flags 0x%08x\n", "do_work_pending", thread_info_flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
