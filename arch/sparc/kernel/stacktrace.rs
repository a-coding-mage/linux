// SPDX-License-Identifier: GPL-2.0-only
// Translated from the C implementation; declarations supplied by kernel dependencies
// are intentionally referenced but not defined here.

unsafe fn __save_stack_trace(
    tp: *mut thread_info,
    trace: *mut stack_trace,
    skip_sched: bool,
) {
    let mut ksp: c_ulong;
    let mut fp: c_ulong;
    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    let t: *mut task_struct;
    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    let mut graph: c_int = 0;

    if tp == current_thread_info() {
        stack_trace_flush();
        // C: __asm__ __volatile__("mov %%fp, %0" : "=r" (ksp));
        core::arch::asm!("mov %fp, {0}", out(reg) ksp);
    } else {
        ksp = (*tp).ksp;
    }

    fp = ksp.wrapping_add(STACK_BIAS);
    #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
    {
        t = (*tp).task;
    }
    loop {
        let sf: *mut sparc_stackf;
        let regs: *mut pt_regs;
        let pc: c_ulong;

        if !kstack_valid(tp, fp) {
            break;
        }

        sf = fp as *mut sparc_stackf;
        regs = sf.add(1) as *mut pt_regs;

        if kstack_is_trap_frame(tp, regs) {
            if ((*regs).tstate & TSTATE_PRIV) == 0 {
                break;
            }
            pc = (*regs).tpc;
            fp = (*regs).u_regs[UREG_I6 as usize].wrapping_add(STACK_BIAS);
        } else {
            pc = (*sf).callers_pc;
            fp = ((*sf).fp as c_ulong).wrapping_add(STACK_BIAS);
        }

        if (*trace).skip > 0 {
            (*trace).skip -= 1;
        } else if !skip_sched || !in_sched_functions(pc) {
            let nr = (*trace).nr_entries as usize;
            (*trace).entries[nr] = pc;
            (*trace).nr_entries += 1;
            #[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
            {
                if pc.wrapping_add(8) == (&return_to_handler as *const _ as c_ulong) {
                    let ret_stack: *mut ftrace_ret_stack =
                        ftrace_graph_get_ret_stack(t, graph);
                    if !ret_stack.is_null() {
                        let ret_pc = (*ret_stack).ret;
                        if (*trace).nr_entries < (*trace).max_entries {
                            let ret_nr = (*trace).nr_entries as usize;
                            (*trace).entries[ret_nr] = ret_pc;
                            (*trace).nr_entries += 1;
                        }
                        graph += 1;
                    }
                }
            }
        }
        if (*trace).nr_entries >= (*trace).max_entries {
            break;
        }
    }
}

pub unsafe fn save_stack_trace(trace: *mut stack_trace) {
    __save_stack_trace(current_thread_info(), trace, false);
}

// EXPORT_SYMBOL_GPL(save_stack_trace);

pub unsafe fn save_stack_trace_tsk(tsk: *mut task_struct, trace: *mut stack_trace) {
    let tp: *mut thread_info = task_thread_info(tsk);
    __save_stack_trace(tp, trace, true);
}

// EXPORT_SYMBOL_GPL(save_stack_trace_tsk);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
