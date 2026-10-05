/* SPDX-License-Identifier: GPL-2.0 */
/*
 * BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
 *
 * Inline definitions layered on top of internal.h and cid.h.
 *
 * Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2026 Tejun Heo <tj@kernel.org>
 */

// Existing body continued against pinned inlines.h, commit
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. The historical marker below remains.
// inlines.h alone supplies scx_dsp_verdict and its four values through the
// canonical native bindings; the old locally invented Rust enum is removed.
compile_error!("SOURCE ONLY HOLD: retained inline dispatch recursion, native callers and stack qualification incomplete");
use super::*;

/*
 * One user of this function is scx_bpf_sub_dispatch() which can be called
 * recursively as sub-sched dispatches nest. Always inline to reduce stack usage
 * from the call frame.
 */
/// # Safety
/// sch, rq and prev remain live under the original dispatch context, with rq
/// locked and the current CPU pinned. The native callback may recursively
/// dispatch sub-schedulers and flush can drop/reacquire rq; no Rust reference
/// may survive either boundary. The original always-inline/recursive stack
/// contract is retained as a requirement, not qualified by this attribute.
#[inline(always)]
pub unsafe fn scx_dispatch_sched(
    sch: *mut scx_sched,
    rq: *mut rq,
    prev: *mut task_struct,
    nested: bool,
) -> scx_dsp_verdict {
    // SAFETY: Native accessors retain per-CPU/header authority. Re-read mutable
    // prev/queue state after callbacks and rq unlock windows, as the old body did.
    // Shared task-field leaves remain plain native accesses, not synchronization.
    unsafe {
    let dspc: *mut scx_dsp_ctx = lupos_scx_core_pick_inline_dsp_ctx(sch);
    let mut nr_loops: c_int = SCX_DSP_MAX_LOOPS as c_int;
    let cpu: i32 = lupos_scx_core_pick_cpu_of(rq);
    let prev_on_sch: bool = (scx_shared_class_read(prev) == lupos_scx_core_ext_class())
        && lupos_scx_core_pick_inline_task_on_sched(sch, prev);

    if scx_consume_global_dsq(sch, rq) {
        return SCX_DSP_LOCAL;
    }

    if lupos_scx_core_pick_inline_bypass_enabled(sch) {
        /* if @sch is bypassing, only the bypass DSQs are active */
        if lupos_scx_core_bypassing(sch, cpu) {
            if scx_consume_dispatch_q(sch, rq, lupos_scx_core_bypass_dsq(sch, cpu), 0) {
                return SCX_DSP_LOCAL;
            }
            return SCX_DSP_NONE;
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
            let pcpu: *mut scx_sched_pcpu = lupos_scx_core_pick_inline_pcpu(sch, cpu);
            let seq = (*pcpu).bypass_host_seq;
            (*pcpu).bypass_host_seq = seq.wrapping_add(1);
            if seq % SCX_BYPASS_HOST_NTH as u32 == 0
                && scx_consume_dispatch_q(sch, rq, lupos_scx_core_bypass_dsq(sch, cpu), 0)
            {
                lupos_scx_core_pick_inline_event_sub_bypass(sch);
                return SCX_DSP_LOCAL;
            }
        }
    }

    if lupos_scx_core_pick_inline_unlikely_no_dispatch(sch) || !scx_rq_online(rq) {
        return SCX_DSP_NONE;
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

        lupos_scx_core_pick_inline_call_dispatch(sch, rq, cpu, prev, prev_on_sch);

        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !nested {
            (*rq).scx.sub_dispatch_prev = core::ptr::null_mut();
        }

        scx_flush_dispatch_buf(sch, rq);

        if (scx_shared_flags_read(core::ptr::addr_of!((*prev).scx)) & SCX_TASK_QUEUED as u32) != 0
            && scx_shared_slice_read(core::ptr::addr_of!((*prev).scx)) != 0 {
            return SCX_DSP_PREV;
        }
        if (*rq).scx.local_dsq.nr != 0 {
            return SCX_DSP_LOCAL;
        }
        if scx_consume_global_dsq(sch, rq) {
            return SCX_DSP_LOCAL;
        }

        // Original predecrement and unlikely marker stay in a native primitive;
        // kick remains before the nr_tasks termination test and stays deferred.
        if lupos_scx_core_pick_inline_unlikely_last_loop(&mut nr_loops) {
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
    if lupos_scx_core_pick_inline_bypass_enabled(sch)
        && scx_consume_dispatch_q(sch, rq, lupos_scx_core_bypass_dsq(sch, cpu), 0)
    {
        return SCX_DSP_LOCAL;
    }

    SCX_DSP_NONE
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
