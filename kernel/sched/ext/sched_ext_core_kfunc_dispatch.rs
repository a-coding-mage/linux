// SPDX-License-Identifier: GPL-2.0
// F14 dispatch-kfunc owners, ext.c:8712-8957 and 8972-9288, pinned at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Extend the existing Rust baseline;
// F13 owns all interleaved BTF sets. Native leaves remain unqualified C runtime.
compile_error!("SOURCE ONLY HOLD: sched_ext dispatch kfunc ABI and protection qualification incomplete");

use super::*;
use core::ptr;
use kernel::ffi::c_ulong;

/// Validate public enqueue bits before any internal PRIQ bit is added.
///
/// # Safety
/// sch is live under the caller's scheduler protection; enq_flags points to
/// the caller's writable local value, not concurrently shared BPF storage.
unsafe fn scx_vet_enq_flags(sch: *mut scx_sched, dsq_id: u64, enq_flags: *mut u64) -> bool {
    // SAFETY: Native constants retain their configured values. The automatic
    // IMMED addition precedes the separate RESCUE check, exactly as in C.
    unsafe {
        let is_local = dsq_id == SCX_DSQ_LOCAL as u64
            || (dsq_id & SCX_DSQ_LOCAL_ON as u64) == SCX_DSQ_LOCAL_ON as u64;
        if lupos_scx_core_kd_unlikely_internal_flags(*enq_flags) {
            lupos_scx_core_kd_error_enq_flags(sch, *enq_flags);
            return false;
        }
        if *enq_flags & SCX_ENQ_IMMED as u64 != 0 {
            if lupos_scx_core_kd_unlikely_immed_nonlocal(is_local) {
                lupos_scx_core_kd_error_immed_nonlocal(sch, dsq_id);
                return false;
            }
        } else if (*sch).ops.flags & SCX_OPS_ALWAYS_ENQ_IMMED as u64 != 0 && is_local {
            *enq_flags |= SCX_ENQ_IMMED as u64;
        }
        if lupos_scx_core_kd_unlikely_rescue_nonlocal(*enq_flags, is_local) {
            lupos_scx_core_kd_error_rescue_nonlocal(sch, dsq_id);
            return false;
        }
        true
    }
}

/// Check task ownership before accepting an insertion verdict.
///
/// # Safety
/// Caller holds the original RCU lifetime and IRQ-disabled dispatch context.
/// p may be NULL; otherwise the verifier supplies its original task lifetime.
unsafe fn scx_dsq_insert_preamble(
    sch: *mut scx_sched, p: *mut task_struct, dsq_id: u64, enq_flags: *mut u64,
) -> bool {
    // SAFETY: No task read precedes NULL rejection. A foreign task increments
    // INSERT_NOT_OWNED without generating a scheduler error at this site.
    unsafe {
        lupos_scx_core_kd_assert_insert_irqs_disabled();
        if lupos_scx_core_kd_unlikely_null_insert_task(p) {
            lupos_scx_core_kd_error_null_task(sch);
            return false;
        }
        if lupos_scx_core_kd_unlikely_insert_not_owned(sch, p) {
            lupos_scx_core_kd_event_insert_not_owned(sch);
            return false;
        }
        if !scx_vet_enq_flags(sch, dsq_id, enq_flags) {
            return false;
        }
        true
    }
}

/// Record direct dispatch or append one buffered verdict with its ops-state qseq.
///
/// # Safety
/// Preamble succeeded. IRQ-disabled current-CPU dispatch context serializes
/// dspc and direct_dispatch_task. sch/p remain live, and dspc has the native
/// dsp_max_batch allocation. This records a verdict, not task custody transfer.
unsafe fn scx_dsq_insert_commit(
    sch: *mut scx_sched, p: *mut task_struct, dsq_id: u64,
    slice: u64, vtime: u64, enq_flags: u64,
) {
    // SAFETY: Preserve this_cpu_ptr before __this_cpu_read, direct dispatch
    // before overflow, and ordinary atomic_long_read (not acquire) for qseq.
    unsafe {
        let dspc = lupos_scx_core_kd_this_dsp_ctx(sch);
        let ddsp_task = lupos_scx_core_direct_dispatch_task_read();
        if !ddsp_task.is_null() {
            mark_direct_dispatch(sch, ddsp_task, p, dsq_id, slice, vtime, enq_flags);
            return;
        }
        if lupos_scx_core_kd_unlikely_buffer_overflow(dspc, sch) {
            lupos_scx_core_kd_error_buffer_overflow(sch);
            return;
        }
        let index = (*dspc).cursor;
        let ent = scx_dsp_buf_ent {
            task: p,
            qseq: lupos_scx_core_kd_ops_state_read(p) & SCX_OPSS_QSEQ_MASK as c_ulong,
            dsq_id,
            slice,
            vtime,
            enq_flags,
        };
        (*dspc).cursor = index.wrapping_add(1);
        // The flexible-array address stays native; ptr::write does not read an
        // uninitialized old slot or fabricate a Rust array bound/reference.
        ptr::write(lupos_scx_core_kd_buf_entry(dspc, index), ent);
    }
}

/// FIFO insertion result after preamble, including direct-dispatch/overflow errors.
///
/// # Safety
/// Only the exact native kfunc wrapper calls this body, within guard(rcu).
/// aux is the implicit verifier argument; p is NULL or verifier-lifetime-pinned.
#[export_name = "lupos_scx_core_kd_insert_v2_body"]
pub unsafe extern "C" fn scx_bpf_dsq_insert___v2(
    p: *mut task_struct, dsq_id: u64, slice: u64, mut enq_flags: u64,
    aux: *const bpf_prog_aux,
) -> bool {
    // SAFETY: Commit is void in the authority; its diagnostic does not change
    // the already-accepted preamble's true return. Do not invent a new result.
    unsafe {
        let sch = lupos_scx_core_kd_prog_sched(aux);
        if lupos_scx_core_kd_unlikely_no_insert_sched(sch) {
            return false;
        }
        if !scx_dsq_insert_preamble(sch, p, dsq_id, &mut enq_flags) {
            return false;
        }
        scx_dsq_insert_commit(sch, p, dsq_id, slice, 0, enq_flags);
        true
    }
}

/// Compatibility FIFO name retains the v2 callee's original RCU guard.
///
/// # Safety
/// Exact native kfunc arguments and verifier context are required. This body
/// has no guard of its own; the native v2 entry is invoked synchronously.
#[export_name = "lupos_scx_core_kd_insert_body"]
pub unsafe extern "C" fn scx_bpf_dsq_insert(
    p: *mut task_struct, dsq_id: u64, slice: u64, enq_flags: u64,
    aux: *const bpf_prog_aux,
) {
    // SAFETY: The called native entry is this lane's guarded Rust-owner shell,
    // never the old C owner. The compatibility ABI deliberately discards bool.
    unsafe { lupos_scx_core_kd_call_insert_v2(p, dsq_id, slice, enq_flags, aux); }
}

/// Vtime insertion adds its internal flag only after public flag validation.
///
/// # Safety
/// Same scheduler/task lifetimes, IRQ context and local flags as FIFO insertion.
unsafe fn scx_dsq_insert_vtime(
    sch: *mut scx_sched, p: *mut task_struct, dsq_id: u64,
    slice: u64, vtime: u64, mut enq_flags: u64,
) -> bool {
    // SAFETY: Preserve preamble result and void commit behavior of the source.
    unsafe {
        if !scx_dsq_insert_preamble(sch, p, dsq_id, &mut enq_flags) {
            return false;
        }
        scx_dsq_insert_commit(sch, p, dsq_id, slice, vtime, enq_flags | SCX_ENQ_DSQ_PRIQ as u64);
        true
    }
}

/// Arg-wrapped vtime entry without pre-evaluating args before scheduler lookup.
///
/// # Safety
/// Native guard(rcu) spans this body. args has the exact native four-u64 layout
/// and remains readable; p retains its separate verifier KF_RCU association.
#[export_name = "lupos_scx_core_kd_insert_vtime_args_body"]
pub unsafe extern "C" fn __scx_bpf_dsq_insert_vtime(
    p: *mut task_struct, args: *mut scx_bpf_dsq_insert_vtime_args,
    aux: *const bpf_prog_aux,
) -> bool {
    // SAFETY: A failed scheduler lookup returns without reading any args field.
    unsafe {
        let sch = lupos_scx_core_kd_prog_sched(aux);
        if lupos_scx_core_kd_unlikely_no_vtime_sched(sch) {
            return false;
        }
        scx_dsq_insert_vtime(sch, p, (*args).dsq_id, (*args).slice, (*args).vtime, (*args).enq_flags)
    }
}

/// Legacy vtime entry refuses ambiguity while any sub-scheduler is attached.
///
/// # Safety
/// Native guard(rcu) spans root lookup and this entire body; p's original
/// kfunc lifetime/context is preserved, including the source's protected
/// scx_task_sched(p) diagnostic when CONFIG_EXT_SUB_SCHED is enabled.
#[export_name = "lupos_scx_core_kd_insert_vtime_compat_body"]
pub unsafe extern "C" fn scx_bpf_dsq_insert_vtime(
    p: *mut task_struct, dsq_id: u64, slice: u64, vtime: u64, enq_flags: u64,
) {
    // SAFETY: Legacy lookup is rcu_dereference(scx_root), not scx_prog_sched,
    // and its sub-scheduler rejection diagnoses p's scheduler, not root.
    unsafe {
        let sch = lupos_scx_core_kd_root_vtime_rcu();
        if lupos_scx_core_kd_unlikely_no_compat_root(sch) {
            return;
        }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if lupos_scx_core_kd_unlikely_compat_children(sch) {
            lupos_scx_core_kd_error_compat_vtime(p);
            return;
        }
        scx_dsq_insert_vtime(sch, p, dsq_id, slice, vtime, enq_flags);
    }
}

/// Move with source DSQ/rq custody revalidation and exact rq restoration.
///
/// # Safety
/// kit is a verifier-initialized iterator slot, possibly with dsq NULL after a
/// failed iterator constructor. p and iterator DSQ remain lifetime-pinned by
/// the kfunc caller. Entry is either a dispatch context holding its tracked rq
/// lock, or an unlocked BPF context; no other rq lock may be held. Native IRQ
/// save/restore and lock leaves retain architecture widths and lock tracking.
unsafe fn scx_dsq_move(
    kit: *mut bpf_iter_scx_dsq_kern, p: *mut task_struct,
    dsq_id: u64, mut enq_flags: u64, priq: bool,
) -> bool {
    // SAFETY: Early validation failures leave iterator overrides unchanged.
    // Once locks are taken, every outcome restores original rq custody and
    // clears overrides; no reference into p or DSQ survives a lock switch.
    unsafe {
        let src_dsq = (*kit).dsq;
        if lupos_scx_core_kd_unlikely_no_move_dsq(src_dsq) {
            return false;
        }
        let sch = (*src_dsq).sched;
        if !scx_vet_enq_flags(sch, dsq_id, &mut enq_flags) {
            return false;
        }
        if priq {
            enq_flags |= SCX_ENQ_DSQ_PRIQ as u64;
        }
        if lupos_scx_core_kd_unlikely_move_aborting(sch) {
            return false;
        }
        if lupos_scx_core_kd_unlikely_move_not_owned(sch, p) {
            lupos_scx_core_kd_error_move_not_owned(sch, p);
            return false;
        }
        let src_rq = lupos_scx_core_kd_task_rq(p);
        let flags = lupos_scx_core_kd_irq_save();
        let locked_rq = lupos_scx_core_locked_rq();
        if !locked_rq.is_null() {
            if locked_rq != src_rq {
                switch_rq_lock(locked_rq, src_rq);
            }
        } else {
            lupos_scx_core_raw_spin_rq_lock(src_rq);
        }
        let mut p_rq = src_rq;
        lupos_scx_core_kd_dsq_lock(src_dsq);
        let dispatched;
        if nldsq_cursor_lost_task(ptr::addr_of_mut!((*kit).cursor), src_rq, src_dsq, p) {
            lupos_scx_core_kd_dsq_unlock(src_dsq);
            dispatched = false;
        } else {
            let rq = if !locked_rq.is_null() { locked_rq } else { lupos_scx_core_kd_this_rq() };
            let dst_dsq = find_dsq_for_dispatch(sch, rq, dsq_id, lupos_scx_core_kd_task_cpu(p));
            if (*kit).cursor.flags & __SCX_DSQ_ITER_HAS_VTIME as u32 != 0 {
                scx_shared_vtime_write(ptr::addr_of_mut!((*p).scx), (*kit).vtime);
            }
            if (*kit).cursor.flags & __SCX_DSQ_ITER_HAS_SLICE as u32 != 0 {
                scx_set_task_slice(p, (*kit).slice);
            }
            p_rq = move_task_between_dsqs(sch, p, enq_flags, src_dsq, dst_dsq);
            dispatched = true;
        }
        if !locked_rq.is_null() {
            if locked_rq != p_rq {
                switch_rq_lock(p_rq, locked_rq);
            }
            // In dispatch, IRQs were already disabled. C intentionally has no
            // local_irq_restore here and keeps the incoming rq lock held.
        } else {
            scx_rq_lock_drop(p_rq);
            lupos_scx_core_kd_rq_unlock_irqrestore(p_rq, flags);
        }
        (*kit).cursor.flags &= !(__SCX_DSQ_ITER_HAS_SLICE as u32 | __SCX_DSQ_ITER_HAS_VTIME as u32);
        dispatched
    }
}

/// Remaining dispatch slots, preserving native __this_cpu_read semantics.
///
/// # Safety
/// Exact native kfunc RCU guard and implicit aux context; only ops.dispatch
/// may call this interface, with current CPU and its dispatch storage pinned.
#[export_name = "lupos_scx_core_kd_dispatch_nr_slots_body"]
pub unsafe extern "C" fn scx_bpf_dispatch_nr_slots(aux: *const bpf_prog_aux) -> u32 {
    // SAFETY: Use unsigned subtraction just as C; no invented saturation.
    unsafe {
        let sch = lupos_scx_core_kd_prog_sched(aux);
        if lupos_scx_core_kd_unlikely_no_slots_sched(sch) {
            return 0;
        }
        (*sch).dsp_max_batch.wrapping_sub(lupos_scx_core_kd_this_cursor_read(sch))
    }
}

/// Cancel exactly the latest pending slot or report native underflow.
///
/// # Safety
/// Native RCU guard pins sch; original dispatch context serializes its per-CPU
/// buffer. Cancelling does not acquire task custody or clear old slot bytes.
#[export_name = "lupos_scx_core_kd_dispatch_cancel_body"]
pub unsafe extern "C" fn scx_bpf_dispatch_cancel(aux: *const bpf_prog_aux) {
    // SAFETY: No per-CPU context is obtained until scheduler lookup succeeds.
    unsafe {
        let sch = lupos_scx_core_kd_prog_sched(aux);
        if lupos_scx_core_kd_unlikely_no_cancel_sched(sch) {
            return;
        }
        let dspc = lupos_scx_core_kd_this_dsp_ctx(sch);
        if (*dspc).cursor > 0 {
            (*dspc).cursor = (*dspc).cursor.wrapping_sub(1);
        } else {
            lupos_scx_core_kd_error_buffer_underflow(sch);
        }
    }
}

/// Flush pending insertions before consuming the selected non-local user DSQ.
///
/// # Safety
/// Exact native RCU guard and dispatch-only verifier context. The dispatched
/// rq is locked; F03 may transfer locks while consuming and restores its
/// promised rq custody. Caller must not hold a BPF lock across this operation.
#[export_name = "lupos_scx_core_kd_move_to_local_v2_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move_to_local___v2(
    dsq_id: u64, mut enq_flags: u64, aux: *const bpf_prog_aux,
) -> bool {
    // SAFETY: Flag vetting uses LOCAL regardless of source ID. Flush precedes
    // lookup/error and successful consumption bumps nr_tasks even if the task
    // is dequeued before it can run, preserving dispatch_one's retry signal.
    unsafe {
        let sch = lupos_scx_core_kd_prog_sched(aux);
        if lupos_scx_core_kd_unlikely_no_local_sched(sch) {
            return false;
        }
        if !scx_vet_enq_flags(sch, SCX_DSQ_LOCAL as u64, &mut enq_flags) {
            return false;
        }
        let dspc = lupos_scx_core_kd_this_dsp_ctx(sch);
        scx_flush_dispatch_buf(sch, (*dspc).rq);
        let dsq = find_user_dsq(sch, dsq_id);
        if lupos_scx_core_kd_unlikely_missing_local_dsq(dsq) {
            lupos_scx_core_kd_error_invalid_dsq(sch, dsq_id);
            return false;
        }
        if scx_consume_dispatch_q(sch, (*dspc).rq, dsq, enq_flags) {
            (*dspc).nr_tasks = (*dspc).nr_tasks.wrapping_add(1);
            true
        } else {
            false
        }
    }
}

/// Compatibility move entry supplies zero flags to the exact guarded v2 entry.
///
/// # Safety
/// Native dispatch-only kfunc context and arguments; no additional guard is
/// added here, and the v2 wrapper's guard cannot end before its body returns.
#[export_name = "lupos_scx_core_kd_move_to_local_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move_to_local(dsq_id: u64, aux: *const bpf_prog_aux) -> bool {
    // SAFETY: This lane supplies the destination native shell and Rust owner.
    unsafe { lupos_scx_core_kd_call_move_to_local_v2(dsq_id, 0, aux) }
}

/// Arm an iterator slice override without writing any task's slice yet.
///
/// # Safety
/// it__iter is an initialized verifier-owned iterator slot with F01's exact
/// size/alignment. It is exclusively used by this BPF call, under KF_RCU.
#[export_name = "lupos_scx_core_kd_move_set_slice_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move_set_slice(it__iter: *mut bpf_iter_scx_dsq, slice: u64) {
    // SAFETY: Native cast is the original iterator ABI, not a lookalike type.
    unsafe {
        let kit = lupos_scx_core_kd_iter_kern(it__iter);
        (*kit).slice = slice;
        (*kit).cursor.flags |= __SCX_DSQ_ITER_HAS_SLICE as u32;
    }
}

/// Arm an iterator vtime override, consumed/cleared by the next locked attempt.
///
/// # Safety
/// Same live native iterator layout and verifier ownership as the slice setter.
#[export_name = "lupos_scx_core_kd_move_set_vtime_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move_set_vtime(it__iter: *mut bpf_iter_scx_dsq, vtime: u64) {
    // SAFETY: This changes iterator-private state, not shared task vtime.
    unsafe {
        let kit = lupos_scx_core_kd_iter_kern(it__iter);
        (*kit).vtime = vtime;
        (*kit).cursor.flags |= __SCX_DSQ_ITER_HAS_VTIME as u32;
    }
}

/// FIFO move entry retaining exact iterator and task parameter identities.
///
/// # Safety
/// Native KF_RCU iterator/task context and scx_dsq_move entry locking contract.
#[export_name = "lupos_scx_core_kd_move_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move(
    it__iter: *mut bpf_iter_scx_dsq, p: *mut task_struct, dsq_id: u64, enq_flags: u64,
) -> bool {
    // SAFETY: No extra guard is added to the original KF_RCU-only interface.
    unsafe { scx_dsq_move(lupos_scx_core_kd_iter_kern(it__iter), p, dsq_id, enq_flags, false) }
}

/// PRIQ move entry adds its internal bit only after vetting public flags.
///
/// # Safety
/// Native KF_RCU iterator/task context and scx_dsq_move entry locking contract.
#[export_name = "lupos_scx_core_kd_move_vtime_body"]
pub unsafe extern "C" fn scx_bpf_dsq_move_vtime(
    it__iter: *mut bpf_iter_scx_dsq, p: *mut task_struct, dsq_id: u64, enq_flags: u64,
) -> bool {
    // SAFETY: The same locked move owner handles vtime/slice updates and cleanup.
    unsafe { scx_dsq_move(lupos_scx_core_kd_iter_kern(it__iter), p, dsq_id, enq_flags, true) }
}
