// SPDX-License-Identifier: GPL-2.0

/*
 * Stack trace utility functions etc.
 *
 * Copyright 2008 Christoph Hellwig, IBM Corp.
 * Copyright 2018 SUSE Linux GmbH
 * Copyright 2018 Nick Piggin, Michael Ellerman, IBM Corp.
 */

// External declarations and constants are supplied by the corresponding kernel headers.

pub unsafe fn arch_stack_walk(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut core::ffi::c_void,
    task: *mut task_struct,
    regs: *mut pt_regs,
) {
    let mut sp: c_ulong;

    if !regs.is_null() && !consume_entry(cookie, (*regs).nip) {
        return;
    }

    if !regs.is_null() {
        sp = (*regs).gpr[1];
    } else if task == current {
        sp = current_stack_frame();
    } else {
        sp = (*task).thread.ksp;
    }

    loop {
        let stack = sp as *mut c_ulong;
        let newsp: c_ulong;
        let ip: c_ulong;

        if !validate_sp(sp, task) {
            return;
        }

        newsp = *stack.add(0);
        ip = *stack.add(STACK_FRAME_LR_SAVE);

        if !consume_entry(cookie, ip) {
            return;
        }

        sp = newsp;
    }
}

/*
 * This function returns an error if it detects any unreliable features of the
 * stack.  Otherwise it guarantees that the stack trace is reliable.
 *
 * If the task is not 'current', the caller *must* ensure the task is inactive.
 */
pub unsafe fn arch_stack_walk_reliable(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut core::ffi::c_void,
    task: *mut task_struct,
) -> c_int {
    let mut sp: c_ulong;
    let mut newsp: c_ulong = 0;
    let stack_page = task_stack_page(task) as c_ulong;
    let stack_end: c_ulong;
    let mut graph_idx: c_int = 0;
    let mut firstframe = true;

    stack_end = stack_page + THREAD_SIZE;

    // See copy_thread() for details.
    if (*task).flags & PF_KTHREAD != 0 {
        stack_end -= STACK_FRAME_MIN_SIZE;
    } else {
        stack_end -= STACK_USER_INT_FRAME_SIZE;
    }

    if task == current {
        sp = current_stack_frame();
    } else {
        sp = (*task).thread.ksp;
    }

    if sp < stack_page + core::mem::size_of::<thread_struct>() as c_ulong
        || sp > stack_end - STACK_FRAME_MIN_SIZE
    {
        return -EINVAL;
    }

    while sp != stack_end {
        let stack = sp as *mut c_ulong;
        let ip: c_ulong;

        /* sanity check: ABI requires SP to be aligned 16 bytes. */
        if sp & 0xF != 0 {
            return -EINVAL;
        }

        newsp = *stack.add(0);
        /* Stack grows downwards; unwinder may only go up. */
        if newsp <= sp {
            return -EINVAL;
        }

        if newsp != stack_end && newsp > stack_end - STACK_FRAME_MIN_SIZE {
            return -EINVAL; /* invalid backlink, too far up. */
        }

        /*
         * We can only trust the bottom frame's backlink, the
         * rest of the frame may be uninitialized, continue to
         * the next.
         */
        if firstframe {
            firstframe = false;
            sp = newsp;
            continue;
        }

        /* Mark stacktraces with exception frames as unreliable. */
        if sp <= stack_end - STACK_INT_FRAME_SIZE
            && *stack.add(STACK_INT_FRAME_MARKER_LONGS) == STACK_FRAME_REGS_MARKER
        {
            return -EINVAL;
        }

        /* Examine the saved LR: it must point into kernel code. */
        ip = *stack.add(STACK_FRAME_LR_SAVE);
        if !__kernel_text_address(ip) {
            return -EINVAL;
        }

        /*
         * FIXME: IMHO these tests do not belong in
         * arch-dependent code, they are generic.
         */
        ip = ftrace_graph_ret_addr(task, &mut graph_idx, ip, stack);

        /*
         * Mark stacktraces with kretprobed functions on them
         * as unreliable.
         */
        #[cfg(feature = "CONFIG_RETHOOK")]
        if ip == arch_rethook_trampoline as c_ulong {
            return -EINVAL;
        }

        if !consume_entry(cookie, ip) {
            return -EINVAL;
        }

        firstframe = false;
        sp = newsp;
    }
    0
}

#[cfg(all(feature = "CONFIG_PPC_BOOK3S_64", feature = "CONFIG_NMI_IPI"))]
unsafe fn handle_backtrace_ipi(regs: *mut pt_regs) {
    nmi_cpu_backtrace(regs);
}

#[cfg(all(feature = "CONFIG_PPC_BOOK3S_64", feature = "CONFIG_NMI_IPI"))]
unsafe fn raise_backtrace_ipi(mask: *mut cpumask_t) {
    let mut p: *mut paca_struct;
    let mut cpu: c_uint;
    let mut delay_us: u64;

    for_each_cpu!(cpu, mask) {
        if cpu == smp_processor_id() {
            handle_backtrace_ipi(core::ptr::null_mut());
            continue;
        }

        delay_us = 5 * USEC_PER_SEC;

        if smp_send_safe_nmi_ipi(cpu, handle_backtrace_ipi, delay_us) {
            // Now wait up to 5s for the other CPU to do its backtrace
            while cpumask_test_cpu(cpu, mask) && delay_us != 0 {
                udelay(1);
                delay_us -= 1;
            }

            // Other CPU cleared itself from the mask
            if delay_us != 0 {
                continue;
            }
        }

        p = *paca_ptrs.add(cpu as usize);

        cpumask_clear_cpu(cpu, mask);

        pr_warn!("CPU %d didn't respond to backtrace IPI, inspecting paca.\n", cpu);
        if !virt_addr_valid(p) {
            pr_warn!("paca pointer appears corrupt? (%px)\n", p);
            continue;
        }

        pr_warn!("irq_soft_mask: 0x%02x in_mce: %d in_nmi: %d", (*p).irq_soft_mask, (*p).in_mce, (*p).in_nmi);

        if virt_addr_valid((*p).__current) {
            pr_cont!(" current: %d (%s)\n", (*(*p).__current).pid, (*(*p).__current).comm);
        } else {
            pr_cont!(" current pointer corrupt? (%px)\n", (*p).__current);
        }

        pr_warn!("Back trace of paca->saved_r1 (0x%016llx) (possibly stale):\n", (*p).saved_r1);
        show_stack((*p).__current, (*p).saved_r1 as *mut c_ulong, KERN_WARNING);
    }
}

#[cfg(all(feature = "CONFIG_PPC_BOOK3S_64", feature = "CONFIG_NMI_IPI"))]
pub unsafe fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: c_int) {
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, raise_backtrace_ipi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
