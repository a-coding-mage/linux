/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Inline definitions layered on top of internal.h and cid.h.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

// Dependencies supplied by internal.h and cid.h are intentionally external.

/* what dispatch concluded, consumed by the pick that follows */
#[repr(C)]
pub enum scx_dsp_verdict {
    SCX_DSP_NONE,
    SCX_DSP_LOCAL,
    SCX_DSP_PREV,
    SCX_DSP_RETRY,
}

/*
 * One user of this function is scx_bpf_sub_dispatch() which can be called
 * recursively as sub-sched dispatches nest. Always inline to reduce stack usage
 * from the call frame.
 */
#[inline(always)]
pub unsafe fn scx_dispatch_sched(
    sch: *mut scx_sched,
    rq: *mut rq,
    prev: *mut task_struct,
    nested: bool,
) -> scx_dsp_verdict {
    let dspc: *mut scx_dsp_ctx = &mut (*this_cpu_ptr((*sch).pcpu)).dsp_ctx;
    let mut nr_loops: i32 = SCX_DSP_MAX_LOOPS;
    let cpu: i32 = cpu_of(rq);
    let prev_on_sch: bool = ((*prev).sched_class == &ext_sched_class)
        && scx_task_on_sched(sch, prev);

    if scx_consume_global_dsq(sch, rq) {
        return scx_dsp_verdict::SCX_DSP_LOCAL;
    }

    if scx_bypass_dsp_enabled(sch) {
        /* if @sch is bypassing, only the bypass DSQs are active */
        if scx_bypassing(sch, cpu) {
            if scx_consume_dispatch_q(sch, rq, scx_bypass_dsq(sch, cpu), 0) {
                return scx_dsp_verdict::SCX_DSP_LOCAL;
            }
            return scx_dsp_verdict::SCX_DSP_NONE;
        }

        // CONFIG_EXT_SUB_SCHED: host-side automatic bypass DSQ consumption.
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        {
            /*
             * If @sch isn't bypassing but its children are, @sch is
             * responsible for making forward progress for both its own
             * tasks that aren't bypassing and the bypassing descendants'
             * tasks. The following implements a simple built-in behavior -
             * let each CPU try to run the bypass DSQ every Nth time.
             *
             * Later, if necessary, we can add an ops flag to suppress the
             * auto-consumption and a kfunc to consume the bypass DSQ and,
             * so that the BPF scheduler can fully control scheduling of
             * bypassed tasks.
             */
            let pcpu: *mut scx_sched_pcpu = per_cpu_ptr((*sch).pcpu, cpu);
            let seq = (*pcpu).bypass_host_seq;
            (*pcpu).bypass_host_seq = seq.wrapping_add(1);
            if seq % SCX_BYPASS_HOST_NTH == 0
                && scx_consume_dispatch_q(sch, rq, scx_bypass_dsq(sch, cpu), 0)
            {
                __scx_add_event(sch, SCX_EV_SUB_BYPASS_DISPATCH, 1);
                return scx_dsp_verdict::SCX_DSP_LOCAL;
            }
        }
    }

    if (!SCX_HAS_OP(sch, dispatch)) || !scx_rq_online(rq) {
        return scx_dsp_verdict::SCX_DSP_NONE;
    }

    (*dspc).rq = rq;

    /*
     * The dispatch loop. Because scx_flush_dispatch_buf() may drop the rq
     * lock, the local DSQ might still end up empty after a successful
     * ops.dispatch(). If the local DSQ is empty even after ops.dispatch()
     * produced some tasks, retry. The BPF scheduler may depend on this
     * looping behavior to simplify its implementation.
     */
    loop {
        (*dspc).nr_tasks = 0;

        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !nested {
            (*rq).scx.sub_dispatch_prev = prev;
        }

        SCX_CALL_OP(sch, dispatch, rq, scx_cpu_arg(cpu),
                    if prev_on_sch { prev } else { core::ptr::null_mut() });

        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !nested {
            (*rq).scx.sub_dispatch_prev = core::ptr::null_mut();
        }

        scx_flush_dispatch_buf(sch, rq);

        if ((*prev).scx.flags & SCX_TASK_QUEUED) != 0 && (*prev).scx.slice != 0 {
            return scx_dsp_verdict::SCX_DSP_PREV;
        }
        if (*rq).scx.local_dsq.nr != 0 {
            return scx_dsp_verdict::SCX_DSP_LOCAL;
        }
        if scx_consume_global_dsq(sch, rq) {
            return scx_dsp_verdict::SCX_DSP_LOCAL;
        }

        if {
            nr_loops -= 1;
            nr_loops == 0
        } {
            scx_kick_cpu(sch, cpu, 0);
            break;
        }
        if (*dspc).nr_tasks == 0 {
            break;
        }
    }

    /*
     * Prevent the CPU from going idle while bypassed descendants have tasks
     * queued. Without this fallback, bypassed tasks could stall if the host
     * scheduler's ops.dispatch() doesn't yield any tasks.
     */
    if scx_bypass_dsp_enabled(sch)
        && scx_consume_dispatch_q(sch, rq, scx_bypass_dsq(sch, cpu), 0)
    {
        return scx_dsp_verdict::SCX_DSP_LOCAL;
    }

    scx_dsp_verdict::SCX_DSP_NONE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
