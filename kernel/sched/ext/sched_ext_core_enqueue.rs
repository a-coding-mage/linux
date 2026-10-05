// SPDX-License-Identifier: GPL-2.0
// F02 continuation of retained ext.rs: pinned ext.c:1412-2374 at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves are unqualified C
// runtime. No native layouts, storage replicas or C algorithm fallback.
compile_error!("SOURCE ONLY HOLD: sched_ext enqueue ABI, concurrency and protection qualification incomplete");

use super::*;
use core::ptr;

/// Native rb comparator's Rust body (1412-1421).
///
/// # Safety
/// Both nodes are genuine live task dsq_priq embeddings on the DSQ protected
/// by its lock. The original BPF-mutability of dsq_vtime still needs a Rust
/// concurrency solution; raw pointers do not establish race freedom.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_enq_priq_less_body(
    node_a: *mut rb_node, node_b: *const rb_node,
) -> bool {
    // SAFETY: The native conversions preserve the actual embedding/constness;
    // time_before64 retains native signed wrap-aware comparison semantics.
    unsafe {
        let a = lupos_scx_core_enq_priq_task(node_a);
        let b = lupos_scx_core_enq_priq_task_const(node_b);
        lupos_scx_core_enq_time_before64(scx_shared_vtime_read(ptr::addr_of!((*a).scx)), scx_shared_vtime_read(ptr::addr_of!((*b).scx)))
    }
}

/// Account insertion and persistent IMMED custody (1423-1459).
///
/// # Safety
/// dsq/p are live; caller holds the DSQ lock, or containing rq lock for local
/// queues. Task flags are serialized by original rq/ops_state custody. An
/// invalid DSQ still runs the native diagnostic without dereferencing its
/// container as an rq. No new admission restrictions replace native behavior.
pub(crate) unsafe fn dsq_inc_nr(dsq: *mut scx_dispatch_q, p: *mut task_struct, enq_flags: u64) {
    // SAFETY: Preserve plain nr read plus WRITE_ONCE store, not an atomic RMW.
    unsafe {
        lupos_scx_core_enq_nr_write_once(dsq, (*dsq).nr.wrapping_add(1));
        if enq_flags & SCX_ENQ_IMMED as u64 != 0 {
            if lupos_scx_core_enq_unlikely_immed_nonlocal(dsq) {
                lupos_scx_core_enq_warn_immed_fallback(enq_flags);
                return;
            }
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_IMMED as u32);
        }
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_IMMED as u32 != 0 {
            let rq = lupos_scx_core_enq_local_dsq_rq(dsq);
            if lupos_scx_core_enq_warn_inc_nonlocal(dsq) {
                return;
            }
            (*rq).scx.nr_immed = (*rq).scx.nr_immed.wrapping_add(1);
            if lupos_scx_core_enq_unlikely_immed_wait(
                (*dsq).nr > 1 || !rq_is_open(rq, enq_flags),
            ) {
                lupos_scx_core_enq_schedule_reenq_local(rq, 0);
            }
        }
    }
}

/// Account removal, retaining independent short-circuited diagnostics (1461-1475).
///
/// # Safety
/// Same DSQ/task synchronization and lifetime contract as dsq_inc_nr.
pub(crate) unsafe fn dsq_dec_nr(dsq: *mut scx_dispatch_q, p: *mut task_struct) {
    // SAFETY: The native rq container is not accessed on the nonlocal warning
    // path. Each WARN has distinct original once-only state.
    unsafe {
        lupos_scx_core_enq_nr_write_once(dsq, (*dsq).nr.wrapping_sub(1));
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_IMMED as u32 != 0 {
            let rq = lupos_scx_core_enq_local_dsq_rq(dsq);
            if lupos_scx_core_enq_warn_dec_nonlocal(dsq)
                || lupos_scx_core_enq_warn_dec_no_immed(rq)
            {
                return;
            }
            (*rq).scx.nr_immed = (*rq).scx.nr_immed.wrapping_sub(1);
        }
    }
}

/// Refill without discarding a pending OOB request (1477-1485).
///
/// # Safety
/// Caller holds p's rq lock, pins sch and disables preemption for accounting.
pub(crate) unsafe fn refill_task_slice_dfl(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: Native READ_ONCE and event preserve original width/order. The
    // ignored setter result is intentional: accounting follows either result.
    unsafe {
        set_task_slice_keep_oob(p, lupos_scx_core_enq_slice_dfl_read_once(sch));
        lupos_scx_core_enq_event_refill(sch);
    }
}

/// Recognize only internal SCX migration (1491-1500).
///
/// # Safety
/// p is live and sticky_cpu is stable under the original migration protocol.
pub(crate) unsafe fn task_scx_migrating(p: *mut task_struct) -> bool {
    // SAFETY: No task_on_rq_migrating substitution is made.
    unsafe { (*p).scx.sticky_cpu >= 0 }
}

/// End BPF custody unless an internal migration is underway (1506-1516).
///
/// # Safety
/// sch/rq/p are live under rq locking or the original DISPATCHING custody
/// protocol; that protocol must exclude concurrent flag RMW until release.
/// Callback runs in the original pinned CPU/rq context, possibly the dispatch
/// rq rather than p's rq. Its task guards and typed callback remain native.
pub(crate) unsafe fn call_task_dequeue(
    sch: *mut scx_sched, rq: *mut rq, p: *mut task_struct, deq_flags: u64,
) {
    // SAFETY: Guard short-circuiting and clear-after-callback remain ordered.
    unsafe {
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_IN_CUSTODY as u32 == 0 || task_scx_migrating(p) {
            return;
        }
        if lupos_scx_core_enq_has_dequeue(sch) {
            lupos_scx_core_enq_call_dequeue(sch, rq, p, deq_flags);
        }
        scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_IN_CUSTODY as u32));
    }
}

/// Finish rq-owned insertion before ops_state release (1518-1587).
///
/// # Safety
/// rq is locked and pinned; p is inserted in its local/reject/rescue queue.
/// Scheduler/task/callback lifetimes and original custody protocol remain live.
pub(crate) unsafe fn rq_owned_post_enq(
    sch: *mut scx_sched, rq: *mut rq, dsq: *mut scx_dispatch_q,
    p: *mut task_struct, enq_flags: u64,
) {
    // SAFETY: Repeated plain curr reads stay behind a native union accessor;
    // no READ_ONCE strengthening or early caching alters callback observations.
    unsafe {
        call_task_dequeue(sch, rq, p, 0);
        if lupos_scx_core_enq_unlikely_post_nonlocal(dsq) {
            if scx_shared_dsq_id_read(dsq) == SCX_DSQ_REJECT as u64 {
                super::schedule_deferred_locked(rq);
            }
            return;
        }
        if lupos_scx_core_class_above(scx_shared_class_read(p), (*rq).next_class) {
            lupos_scx_core_enq_wakeup_preempt(rq, p, 0);
        }
        if (*rq).scx.flags & SCX_RQ_IN_DISPATCH as u32 != 0 {
            return;
        }
        if enq_flags & SCX_ENQ_PREEMPT as u64 != 0
            && p != lupos_scx_core_rq_curr(rq)
            && scx_shared_class_read(lupos_scx_core_rq_curr(rq)) == lupos_scx_core_ext_class()
        {
            if lupos_scx_core_enq_likely_set_preempt_slice(scx_set_task_slice(
                lupos_scx_core_rq_curr(rq), 0,
            )) {
                resched_curr(rq);
            } else {
                lupos_scx_core_enq_event_slice_denied(sch);
            }
        }
    }
}

/// Commit a DSQ insertion, then transfer custody (1589-1730).
///
/// # Safety
/// rq and sch are live; caller holds the original rq/dispatch ownership and
/// task/DSQ lifetime protection. For nonlocal queues this function acquires the
/// DSQ lock, but does not acquire p's rq lock. The caller's ops_state protocol
/// must serialize task flag RMW; waiters cannot proceed before the final release.
/// F01's apply_slice_vtime permitted native racing case is unresolved in Rust,
/// as are externally mutable dsq_vtime reads; hard holds remain mandatory.
pub(crate) unsafe fn scx_dispatch_enqueue(
    sch: *mut scx_sched, rq: *mut rq, mut dsq: *mut scx_dispatch_q,
    p: *mut task_struct, slice: u64, vtime: u64, mut enq_flags: u64,
) {
    // SAFETY: List/tree primitives use actual native nodes and layouts; all
    // queue decisions remain here. is_rq_owned records original LOCAL entry,
    // including a subsequent capability resolution to REJECT or RESCUE.
    unsafe {
        let mut is_rq_owned = false;
        if scx_shared_dsq_id_read(dsq) == SCX_DSQ_LOCAL as u64 {
            dsq = lupos_scx_core_enq_resolve_local(sch, rq, p, &mut enq_flags);
            is_rq_owned = true;
        }
        lupos_scx_core_enq_warn_linked_dispatch(p);
        lupos_scx_core_enq_warn_priq_dispatch(p);
        if !is_rq_owned {
            lupos_scx_core_enq_lock_nested(dsq, enq_flags);
            if lupos_scx_core_enq_unlikely_destroyed(dsq) {
                lupos_scx_core_enq_error_destroyed(sch);
                lupos_scx_core_enq_unlock(dsq);
                dsq = find_global_dsq(sch, lupos_scx_core_enq_task_cpu(p));
                lupos_scx_core_enq_lock(dsq);
            }
        }
        if lupos_scx_core_enq_unlikely_builtin_priq(dsq, enq_flags) {
            lupos_scx_core_enq_error_builtin_priq(sch);
            enq_flags &= !(SCX_ENQ_DSQ_PRIQ as u64);
        }
        if enq_flags & SCX_ENQ_APPLY_SLICE as u64 != 0 {
            apply_slice_vtime(p, slice, vtime, enq_flags);
        }
        if enq_flags & SCX_ENQ_DSQ_PRIQ as u64 != 0 {
            if lupos_scx_core_enq_unlikely_existing_fifo(
                lupos_scx_core_enq_priq_empty(dsq)
                    && !nldsq_next_task(dsq, ptr::null_mut(), false).is_null(),
            ) {
                lupos_scx_core_enq_error_existing_fifo(sch, dsq);
            }
            (*p).scx.dsq_flags |= SCX_TASK_DSQ_ON_PRIQ as u32;
            lupos_scx_core_enq_rb_add(p, dsq);
            let rbp = lupos_scx_core_enq_rb_prev(p);
            if !rbp.is_null() {
                let prev = lupos_scx_core_enq_priq_task(rbp);
                lupos_scx_core_slice_list_add(
                    ptr::addr_of_mut!((*p).scx.dsq_list.node),
                    ptr::addr_of_mut!((*prev).scx.dsq_list.node),
                );
            } else {
                lupos_scx_core_slice_list_add(
                    ptr::addr_of_mut!((*p).scx.dsq_list.node), ptr::addr_of_mut!((*dsq).list),
                );
                lupos_scx_core_enq_first_assign(dsq, p);
            }
        } else {
            if lupos_scx_core_enq_unlikely_existing_priq(dsq) {
                lupos_scx_core_enq_error_existing_priq(sch, dsq);
            }
            if enq_flags & (SCX_ENQ_HEAD as u64 | SCX_ENQ_PREEMPT as u64) != 0 {
                if dsq_insert_head(dsq, p) && scx_shared_dsq_id_read(dsq) & SCX_DSQ_FLAG_BUILTIN as u64 == 0 {
                    lupos_scx_core_enq_first_assign(dsq, p);
                }
            } else {
                lupos_scx_core_enq_list_add_tail(
                    ptr::addr_of_mut!((*p).scx.dsq_list.node), ptr::addr_of_mut!((*dsq).list),
                );
                // Deliberately plain first_task, not rcu_access_pointer.
                if lupos_scx_core_enq_first_plain(dsq).is_null()
                    && scx_shared_dsq_id_read(dsq) & SCX_DSQ_FLAG_BUILTIN as u64 == 0
                {
                    lupos_scx_core_enq_first_assign(dsq, p);
                }
            }
        }
        lupos_scx_core_enq_seq_write_once(dsq, (*dsq).seq.wrapping_add(1));
        (*p).scx.dsq_seq = (*dsq).seq;
        dsq_inc_nr(dsq, p, enq_flags);
        (*p).scx.dsq = dsq;
        if is_rq_owned {
            rq_owned_post_enq(sch, rq, dsq, p, enq_flags);
        } else {
            if scx_shared_dsq_id_read(dsq) == SCX_DSQ_GLOBAL as u64 || scx_shared_dsq_id_read(dsq) == SCX_DSQ_BYPASS as u64 {
                call_task_dequeue(sch, rq, p, 0);
            } else {
                scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_IN_CUSTODY as u32);
            }
            lupos_scx_core_enq_unlock(dsq);
        }
        if enq_flags & SCX_ENQ_CLEAR_OPSS as u64 != 0 {
            lupos_scx_core_enq_opss_set_release(p, SCX_OPSS_NONE as c_ulong);
        }
    }
}

/// Unlink both tree/list representations and repair lockless head (1732-1751).
///
/// # Safety
/// p belongs to live dsq; caller owns its DSQ/rq lock and task lifetime. Queue
/// links, dsq flags and rq-owned IMMED accounting are stable under that lock.
#[no_mangle]
pub unsafe extern "C" fn scx_task_unlink_from_dsq(p: *mut task_struct, dsq: *mut scx_dispatch_q) {
    // SAFETY: The RCU pointer is compared only, then replaced with the next
    // real task after unlinking; parked iterator nodes are skipped by F01.
    unsafe {
        lupos_scx_core_enq_warn_unlink_empty(p);
        if (*p).scx.dsq_flags & SCX_TASK_DSQ_ON_PRIQ as u32 != 0 {
            lupos_scx_core_enq_rb_erase(p, dsq);
            lupos_scx_core_enq_rb_clear(p);
            (*p).scx.dsq_flags &= !(SCX_TASK_DSQ_ON_PRIQ as u32);
        }
        lupos_scx_core_slice_list_del_init(ptr::addr_of_mut!((*p).scx.dsq_list.node));
        dsq_dec_nr(dsq, p);
        if scx_shared_dsq_id_read(dsq) & SCX_DSQ_FLAG_BUILTIN as u64 == 0
            && lupos_scx_core_enq_first_access(dsq) == p
        {
            let first_task = nldsq_next_task(dsq, ptr::null_mut(), false);
            lupos_scx_core_enq_first_assign(dsq, first_task);
        }
    }
}

/// Cancel deferred or in-flight dispatch while holding p's rq lock (1753-1804).
///
/// # Safety
/// rq/p and any p->scx.dsq remain live. rq is locked; ops_dequeue or the
/// caller's equivalent original synchronization has resolved ops_state first.
#[no_mangle]
pub unsafe extern "C" fn scx_dispatch_dequeue(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: Lock nonlocal dsq before testing holding_cpu. No-dsq deferred
    // cancellation requires only unlinking and relinquishing holding_cpu.
    unsafe {
        let dsq = (*p).scx.dsq;
        let is_rq_owned = !dsq.is_null() && dsq_is_rq_owned(dsq);
        lupos_scx_core_enq_assert_dequeue_rq(rq);
        if dsq.is_null() {
            if lupos_scx_core_enq_unlikely_deferred_linked(p) {
                lupos_scx_core_slice_list_del_init(ptr::addr_of_mut!((*p).scx.dsq_list.node));
            }
            if (*p).scx.holding_cpu >= 0 {
                (*p).scx.holding_cpu = -1;
            }
            return;
        }
        if !is_rq_owned {
            lupos_scx_core_enq_lock(dsq);
        }
        if (*p).scx.holding_cpu < 0 {
            scx_task_unlink_from_dsq(p, dsq);
        } else {
            lupos_scx_core_enq_warn_holding_linked(p);
            (*p).scx.holding_cpu = -1;
        }
        (*p).scx.dsq = ptr::null_mut();
        if !is_rq_owned {
            lupos_scx_core_enq_unlock(dsq);
        }
    }
}

/// Abbreviated removal with rq and DSQ already locked (1810-1818).
///
/// # Safety
/// Both native locks are held, p belongs to dsq, and objects remain live.
pub(crate) unsafe fn dispatch_dequeue_locked(p: *mut task_struct, dsq: *mut scx_dispatch_q) {
    // SAFETY: Native assertions precede all owner operations.
    unsafe {
        lupos_scx_core_enq_assert_locked_task_rq(p);
        lupos_scx_core_enq_assert_locked_dsq(dsq);
        scx_task_unlink_from_dsq(p, dsq);
        (*p).scx.dsq = ptr::null_mut();
    }
}

/// Resolve an insertion verdict, falling back only on the original errors (1820-1853).
///
/// # Safety
/// Scheduler hash/per-node queues, rq and CPU topology remain pinned under the
/// original rq/RCU protection. tcpu is valid; dsq_id may be invalid and is checked.
pub(crate) unsafe fn find_dsq_for_dispatch(
    sch: *mut scx_sched, rq: *mut rq, dsq_id: u64, tcpu: i32,
) -> *mut scx_dispatch_q {
    // SAFETY: CPU/CID return conversion stays with its native owner. Reject
    // and rescue are not manually exposed: their IDs take the user hash path.
    unsafe {
        if dsq_id == SCX_DSQ_LOCAL as u64 {
            return ptr::addr_of_mut!((*rq).scx.local_dsq);
        }
        if dsq_id & SCX_DSQ_LOCAL_ON as u64 == SCX_DSQ_LOCAL_ON as u64 {
            let cpu = lupos_scx_core_enq_cpu_ret(sch, (dsq_id & SCX_DSQ_LOCAL_CPU_MASK as u64) as i32);
            if !scx_cpu_valid(sch, cpu, b"in SCX_DSQ_LOCAL_ON dispatch verdict\0".as_ptr().cast()) {
                return find_global_dsq(sch, tcpu);
            }
            return ptr::addr_of_mut!((*lupos_scx_core_enq_cpu_rq(cpu)).scx.local_dsq);
        }
        let dsq = if dsq_id == SCX_DSQ_GLOBAL as u64 {
            find_global_dsq(sch, tcpu)
        } else {
            find_user_dsq(sch, dsq_id)
        };
        if lupos_scx_core_enq_unlikely_missing_dsq(dsq) {
            lupos_scx_core_enq_error_missing_dsq(sch, dsq_id);
            return find_global_dsq(sch, tcpu);
        }
        dsq
    }
}

/// Record a single direct-dispatch verdict (1855-1886).
///
/// # Safety
/// Current CPU is pinned and owns its direct_dispatch_task slot. p is live;
/// ddsp_task is its live expected task or a native error pointer previously
/// stored in that slot. sch and task attributes remain pinned for diagnostics.
pub(crate) unsafe fn mark_direct_dispatch(
    sch: *mut scx_sched, ddsp_task: *mut task_struct, p: *mut task_struct,
    dsq_id: u64, slice: u64, vtime: u64, enq_flags: u64,
) {
    // SAFETY: Spoil always precedes validation. IS_ERR is native and error
    // pointers are never dereferenced by Rust or the already-dispatched leaf.
    unsafe {
        lupos_scx_core_spoil_direct_dispatch_task();
        if lupos_scx_core_enq_unlikely_wrong_ddsp_task(p, ddsp_task) {
            if lupos_scx_core_enq_is_err_task(ddsp_task) {
                lupos_scx_core_enq_error_already_dispatched(sch, p);
            } else {
                lupos_scx_core_enq_error_wrong_dispatched(sch, ddsp_task, p);
            }
            return;
        }
        lupos_scx_core_enq_warn_ddsp_id(p);
        lupos_scx_core_enq_warn_ddsp_flags(p);
        (*p).scx.ddsp_slice = slice;
        (*p).scx.ddsp_vtime = vtime;
        (*p).scx.ddsp_dsq_id = dsq_id;
        (*p).scx.ddsp_enq_flags = enq_flags;
    }
}

/// Clear only the direct-dispatch verdict fields (1903-1907).
///
/// # Safety
/// p is live and caller owns its enqueue/dequeue/disable transition. Deferred
/// dispatch must retain these fields until consumed or cancelled.
pub(crate) unsafe fn clear_direct_dispatch(p: *mut task_struct) {
    // SAFETY: Slice/vtime deliberately survive; invalid id makes them inactive.
    unsafe {
        (*p).scx.ddsp_dsq_id = SCX_DSQ_INVALID as u64;
        (*p).scx.ddsp_enq_flags = 0;
    }
}

/// Deliver direct dispatch or defer a remote-local verdict (1909-1961).
///
/// # Safety
/// p's rq is locked/pinned and sch is live; recorded verdict and DSQ lifetimes
/// are protected by the enqueue path. This path cannot double-lock a remote rq.
pub(crate) unsafe fn direct_dispatch(sch: *mut scx_sched, p: *mut task_struct, enq_flags: u64) {
    // SAFETY: Remote deferral clears ops_state but keeps direct verdict fields;
    // synchronous delivery snapshots and clears them before enqueueing.
    unsafe {
        let rq = lupos_scx_core_task_rq(p);
        let dsq = find_dsq_for_dispatch(sch, rq, (*p).scx.ddsp_dsq_id, lupos_scx_core_enq_task_cpu(p));
        (*p).scx.ddsp_enq_flags |= enq_flags;
        if scx_shared_dsq_id_read(dsq) == SCX_DSQ_LOCAL as u64 && dsq != ptr::addr_of_mut!((*rq).scx.local_dsq) {
            let opss = lupos_scx_core_enq_opss_read(p) & SCX_OPSS_STATE_MASK as c_ulong;
            match opss & SCX_OPSS_STATE_MASK as c_ulong {
                state if state == SCX_OPSS_NONE as c_ulong => {},
                state if state == SCX_OPSS_QUEUEING as c_ulong => {
                    lupos_scx_core_enq_opss_set_release(p, SCX_OPSS_NONE as c_ulong);
                },
                _ => {
                    lupos_scx_core_enq_warn_direct_opss(p, opss);
                    lupos_scx_core_enq_opss_set_release(p, SCX_OPSS_NONE as c_ulong);
                },
            }
            lupos_scx_core_enq_warn_linked_direct(p);
            lupos_scx_core_enq_list_add_tail(
                ptr::addr_of_mut!((*p).scx.dsq_list.node),
                ptr::addr_of_mut!((*rq).scx.ddsp_deferred_locals),
            );
            super::schedule_deferred_locked(rq);
            return;
        }
        let ddsp_enq_flags = (*p).scx.ddsp_enq_flags;
        let slice = (*p).scx.ddsp_slice;
        let vtime = (*p).scx.ddsp_vtime;
        clear_direct_dispatch(p);
        scx_dispatch_enqueue(sch, rq, dsq, p, slice, vtime,
            ddsp_enq_flags | SCX_ENQ_APPLY_SLICE as u64 | SCX_ENQ_CLEAR_OPSS as u64);
    }
}

/// Test both kernel and scheduler-visible online state (1963-1973).
///
/// # Safety
/// rq and topology remain live in the original scheduling/hotplug context;
/// that context stabilizes cpu_active through this scheduling operation.
#[no_mangle]
pub unsafe extern "C" fn scx_rq_online(rq: *mut rq) -> bool {
    // SAFETY: Short-circuit CPU test and original likely marker are retained.
    unsafe {
        lupos_scx_core_enq_likely_online(rq)
    }
}

/// Enqueue in BPF custody or the selected fallback queue (1975-2098).
///
/// # Safety
/// Caller holds rq's lock and pins CPU/preemption, sch/p and DSQ lifetimes.
/// The queued-bit diagnostic remains reachable; it is not a new safety
/// precondition suppressing native warning/continuation behavior. The original
/// ops_state and direct-dispatch protocol applies. Callback guards stay native. F01's
/// unresolved slice/vtime concurrency obligation also applies to dispatch.
#[no_mangle]
pub unsafe extern "C" fn scx_do_enqueue_task(
    rq: *mut rq, p: *mut task_struct, mut enq_flags: u64, sticky_cpu: c_int,
) {
    // SAFETY: The labeled block expresses only the original fallback gotos.
    // Sticky internal movement returns before IMMED clearing or slice refill.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        lupos_scx_core_enq_warn_not_queued(p);
        if sticky_cpu == lupos_scx_core_enq_cpu_of(rq) {
            scx_dispatch_enqueue(sch, rq, ptr::addr_of_mut!((*rq).scx.local_dsq), p, 0, 0, enq_flags);
            return;
        }
        scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_IMMED as u32));
        if enq_flags & SCX_ENQ_REENQ as u64 != 0 {
            (*p).scx.reenq_cnt = (*p).scx.reenq_cnt.wrapping_add(1);
            if (*p).scx.reenq_cnt > 1 {
                lupos_scx_core_enq_event_reenq_repeat(sch);
            }
            if lupos_scx_core_enq_unlikely_reenq_limit(p) {
                lupos_scx_core_enq_exit_reenq(sch, rq, p);
                return;
            }
        }
        let dsq = 'fallback: {
            if !scx_rq_online(rq) {
                break 'fallback ptr::addr_of_mut!((*rq).scx.local_dsq);
            }
            if lupos_scx_core_bypassing(sch, lupos_scx_core_enq_cpu_of(rq)) {
                lupos_scx_core_enq_event_bypass(sch);
                break 'fallback bypass_enq_target_dsq(sch, lupos_scx_core_enq_task_cpu(p));
            }
            if (*p).scx.ddsp_dsq_id != SCX_DSQ_INVALID as u64 {
                direct_dispatch(sch, p, enq_flags);
                return;
            }
            if lupos_scx_core_task_ops_flags(sch) & SCX_OPS_ENQ_EXITING as u64 == 0
                && lupos_scx_core_enq_unlikely_exiting(p)
            {
                lupos_scx_core_enq_event_skip_exiting(sch);
                enq_flags |= SCX_ENQ_RESCUE as u64;
                break 'fallback ptr::addr_of_mut!((*rq).scx.local_dsq);
            }
            if lupos_scx_core_task_ops_flags(sch) & SCX_OPS_ENQ_MIGRATION_DISABLED as u64 == 0
                && lupos_scx_core_enq_migration_disabled(p)
            {
                lupos_scx_core_enq_event_skip_migration(sch);
                break 'fallback ptr::addr_of_mut!((*rq).scx.local_dsq);
            }
            if lupos_scx_core_enq_unlikely_no_enqueue(sch) {
                break 'fallback find_global_dsq(sch, lupos_scx_core_enq_task_cpu(p));
            }
            let qseq = (*rq).scx.ops_qseq << SCX_OPSS_QSEQ_SHIFT;
            (*rq).scx.ops_qseq = (*rq).scx.ops_qseq.wrapping_add(1);
            lupos_scx_core_enq_warn_enqueue_opss(p);
            lupos_scx_core_enq_opss_set(p, SCX_OPSS_QUEUEING as c_ulong | qseq);
            let ddsp_taskp = lupos_scx_core_this_direct_dispatch_task();
            lupos_scx_core_enq_warn_enqueue_ddsp(ddsp_taskp);
            *ddsp_taskp = p;
            lupos_scx_core_enq_call_enqueue(sch, rq, p, enq_flags);
            *ddsp_taskp = ptr::null_mut();
            if (*p).scx.ddsp_dsq_id != SCX_DSQ_INVALID as u64 {
                direct_dispatch(sch, p, enq_flags);
                return;
            }
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_IN_CUSTODY as u32);
            lupos_scx_core_enq_opss_set_release(p, SCX_OPSS_QUEUED as c_ulong | qseq);
            return;
        };
        refill_task_slice_dfl(sch, p);
        clear_direct_dispatch(p);
        scx_dispatch_enqueue(sch, rq, dsq, p, 0, 0, enq_flags);
    }
}

/// Check runnable-list membership (2100-2103).
///
/// # Safety
/// p is live and runnable_node is stable under its rq synchronization.
/// Native warning adapters call synchronously with that same protection;
/// this body neither transfers ownership nor permits unwind across C ABI.
#[export_name = "lupos_scx_core_enq_task_runnable_body"]
pub(crate) unsafe extern "C" fn task_runnable(p: *const task_struct) -> bool {
    // SAFETY: The native list_empty sees the actual native node.
    unsafe { !lupos_scx_core_slice_list_empty(ptr::addr_of!((*p).scx.runnable_node)) }
}

/// Append runnable membership and record its rq (2105-2126).
///
/// # Safety
/// Caller holds rq's lock, p is live and insertable, and rq is p's runnable rq.
pub(crate) unsafe fn set_task_runnable(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: Preserve append order relied on by bypass, and publish CPU only
    // after list insertion. Reset timestamp uses original plain jiffies read.
    unsafe {
        lupos_scx_core_enq_assert_runnable_rq(rq);
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_RESET_RUNNABLE_AT as u32 != 0 {
            (*p).scx.runnable_at = lupos_scx_core_task_jiffies();
            scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_RESET_RUNNABLE_AT as u32));
        }
        lupos_scx_core_enq_list_add_tail(
            ptr::addr_of_mut!((*p).scx.runnable_node), ptr::addr_of_mut!((*rq).scx.runnable_list),
        );
        lupos_scx_core_enq_runnable_cpu_write_once(p, lupos_scx_core_enq_cpu_of(rq));
    }
}

/// Remove runnable membership, optionally restarting runnable accounting (2128-2136).
///
/// # Safety
/// Caller holds p's rq synchronization and pins p and its list membership.
pub(crate) unsafe fn clr_task_runnable(p: *mut task_struct, reset_runnable_at: bool) {
    // SAFETY: Timestamp flag and reenq count change only on the reset path.
    unsafe {
        lupos_scx_core_slice_list_del_init(ptr::addr_of_mut!((*p).scx.runnable_node));
        lupos_scx_core_enq_runnable_cpu_write_once(p, -1);
        if reset_runnable_at {
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_RESET_RUNNABLE_AT as u32);
            (*p).scx.reenq_cnt = 0;
        }
    }
}

/// Class enqueue body, reached through the native typed callback (2138-2186).
///
/// # Safety
/// Original scheduler class context holds rq's lock and task/owner lifetimes.
/// Core enqueue flags and restoration/current-task relationship are native.
#[export_name = "lupos_scx_core_enq_enqueue_task_body"]
pub unsafe extern "C" fn enqueue_task_scx(rq: *mut rq, p: *mut task_struct, core_enq_flags: c_int) {
    // SAFETY: Signed int converts to u64 exactly as the native usual arithmetic
    // conversion; no u32 intermediate discards high sign-extension bits.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        let mut sticky_cpu = (*p).scx.sticky_cpu;
        let mut enq_flags = core_enq_flags as u64 | (*rq).scx.remote_activate_enq_flags;
        if enq_flags & ENQUEUE_WAKEUP as u64 != 0 {
            (*rq).scx.flags |= SCX_RQ_IN_WAKEUP as u32;
        }
        if lupos_scx_core_enq_unlikely_restore(enq_flags)
            && lupos_scx_core_enq_task_current(rq, p)
        {
            sticky_cpu = lupos_scx_core_enq_cpu_of(rq);
            enq_flags |= SCX_ENQ_IGNORE_CAPS as u64;
        }
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 != 0 {
            lupos_scx_core_enq_warn_queued_not_runnable(p);
        } else {
            set_task_runnable(rq, p);
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_QUEUED as u32);
            (*rq).scx.nr_running = (*rq).scx.nr_running.wrapping_add(1);
            lupos_scx_core_enq_add_nr_running(rq);
            if lupos_scx_core_enq_has_runnable(sch) && !lupos_scx_core_enq_on_rq_migrating(p) {
                lupos_scx_core_enq_call_runnable(sch, rq, p, enq_flags);
            }
            if (*rq).scx.nr_running == 1 {
                dl_server_start(ptr::addr_of_mut!((*rq).ext_server));
            }
            scx_do_enqueue_task(rq, p, enq_flags, sticky_cpu);
            if sticky_cpu >= 0 {
                (*p).scx.sticky_cpu = -1;
            }
        }
        (*rq).scx.flags &= !(SCX_RQ_IN_WAKEUP as u32);
        if enq_flags & SCX_ENQ_CPU_SELECTED as u64 != 0
            && lupos_scx_core_enq_unlikely_selected_fallback(rq, p)
        {
            lupos_scx_core_enq_event_select_fallback(sch);
        }
    }
}

/// Finish BPF ownership transfer before dequeue (2188-2262).
///
/// # Safety
/// rq's lock is held and p/sch remain live. Dispatching side must obey original
/// no-rq-lock-while-DISPATCHING protocol, allowing this acquire-spin to finish.
/// READ_ONCE flags native races remain an explicit Rust memory-model obligation.
pub(crate) unsafe fn ops_dequeue(rq: *mut rq, p: *mut task_struct, deq_flags: u64) {
    // SAFETY: A failed queued cmpxchg falls through to waiting for DISPATCHING
    // specifically, even when it updates opss to NONE. Unknown states retain
    // the native switch's no-default behavior rather than inventing an error.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        clr_task_runnable(p, false);
        loop {
            let mut opss = lupos_scx_core_ops_state_read_acquire(p) as c_ulong;
            let state = opss & SCX_OPSS_STATE_MASK as c_ulong;
            if state == SCX_OPSS_QUEUEING as c_ulong {
                lupos_scx_core_enq_bug_queueing();
            }
            if state == SCX_OPSS_QUEUED as c_ulong {
                if lupos_scx_core_enq_unlikely_lost_custody(p) {
                    lupos_scx_core_cpu_relax();
                    continue;
                }
                if lupos_scx_core_enq_opss_try_none(p, &mut opss) {
                    break;
                }
            }
            if state == SCX_OPSS_QUEUED as c_ulong || state == SCX_OPSS_DISPATCHING as c_ulong {
                wait_ops_state(p, SCX_OPSS_DISPATCHING as c_ulong);
                lupos_scx_core_enq_bug_after_dispatch(p);
            }
            break;
        }
        call_task_dequeue(sch, rq, p, deq_flags);
    }
}

/// Class dequeue body, preserving stopping inside runnable lifetime (2264-2322).
///
/// # Safety
/// Scheduler class context pins rq, p and sch and holds rq's lock. The ops
/// ownership protocol must allow waiting as described by ops_dequeue.
#[export_name = "lupos_scx_core_enq_dequeue_task_body"]
pub unsafe extern "C" fn dequeue_task_scx(rq: *mut rq, p: *mut task_struct, core_deq_flags: c_int) -> bool {
    // SAFETY: Original SAVE/current exception retains slice protection. Direct
    // state is cleared only after dispatch cancellation and slice-end handling.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        let mut deq_flags = core_deq_flags as u64;
        if deq_flags & DEQUEUE_SLEEP as u64 == 0 {
            deq_flags |= SCX_DEQ_SCHED_CHANGE as u64;
        }
        if scx_shared_flags_read(ptr::addr_of!((*p).scx)) & SCX_TASK_QUEUED as u32 == 0 {
            lupos_scx_core_enq_warn_unqueued_runnable(p);
            return true;
        }
        ops_dequeue(rq, p, deq_flags);
        if lupos_scx_core_enq_task_current(rq, p)
            && (lupos_scx_core_enq_has_stopping_outer(sch)
                || lupos_scx_core_enq_unlikely_stopping_rescue(p, rq))
        {
            update_curr_scx(rq);
            if lupos_scx_core_enq_has_stopping_inner(sch) {
                lupos_scx_core_enq_call_stopping(sch, rq, p);
            }
        }
        if lupos_scx_core_enq_has_quiescent(sch) && !lupos_scx_core_enq_on_rq_migrating(p) {
            lupos_scx_core_enq_call_quiescent(sch, rq, p, deq_flags);
        }
        if deq_flags & SCX_DEQ_SLEEP as u64 != 0 {
            scx_shared_flags_or(ptr::addr_of_mut!((*p).scx), SCX_TASK_DEQD_FOR_SLEEP as u32);
        } else {
            scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_DEQD_FOR_SLEEP as u32));
        }
        scx_shared_flags_and(ptr::addr_of_mut!((*p).scx), !(SCX_TASK_QUEUED as u32));
        (*rq).scx.nr_running = (*rq).scx.nr_running.wrapping_sub(1);
        lupos_scx_core_enq_sub_nr_running(rq);
        scx_dispatch_dequeue(rq, p);
        if !(deq_flags & DEQUEUE_SAVE as u64 != 0 && lupos_scx_core_enq_task_current(rq, p)) {
            scx_task_slice_ended(rq, p);
        }
        clear_direct_dispatch(p);
        true
    }
}

/// Class yield body using the scheduling donor, not execution current (2324-2336).
///
/// # Safety
/// Caller holds rq's lock and original class context; donor and owner are live.
#[export_name = "lupos_scx_core_enq_yield_task_body"]
pub unsafe extern "C" fn yield_task_scx(rq: *mut rq) {
    // SAFETY: Native plain donor access handles CONFIG_SCHED_PROXY_EXEC's
    // union alternatives. Protection ends even if the callback refuses yield.
    unsafe {
        let p = lupos_scx_core_enq_rq_donor(rq);
        let sch = lupos_scx_core_task_sched(p);
        scx_task_slice_ended(rq, p);
        if lupos_scx_core_enq_has_yield(sch) {
            lupos_scx_core_enq_call_yield(sch, rq, p);
        } else {
            scx_set_task_slice(p, 0);
        }
    }
}

/// Class directed yield body with original same-owner restriction (2338-2350).
///
/// # Safety
/// rq/from/to and their schedulers are live and stable under original rq and
/// directed-yield synchronization; native two-task callback guards may be used.
#[export_name = "lupos_scx_core_enq_yield_to_task_body"]
pub unsafe extern "C" fn yield_to_task_scx(rq: *mut rq, to: *mut task_struct) -> bool {
    // SAFETY: Slice protection ends before testing callback and target owner.
    unsafe {
        let from = lupos_scx_core_enq_rq_donor(rq);
        let sch = lupos_scx_core_task_sched(from);
        scx_task_slice_ended(rq, from);
        if lupos_scx_core_enq_has_yield_to(sch) && sch == lupos_scx_core_task_sched(to) {
            lupos_scx_core_enq_call_yield_to(sch, rq, from, to)
        } else {
            false
        }
    }
}

/// Reenqueue IMMED tasks on preemption by another class (2352-2374).
///
/// # Safety
/// rq is locked, task/class fields are live and stable, and the caller supplies
/// the scheduler's original wakeup-preempt context. wake_flags is unused in C.
#[export_name = "lupos_scx_core_enq_wakeup_preempt_body"]
pub unsafe extern "C" fn wakeup_preempt_scx(rq: *mut rq, p: *mut task_struct, _wake_flags: c_int) {
    // SAFETY: SCX-to-SCX path is deliberately empty; native primitive preserves
    // the shared header's RCU/root/deferred-queue lifetime contract.
    unsafe {
        if scx_shared_class_read(p) == lupos_scx_core_ext_class() {
            return;
        }
        if (*rq).scx.nr_immed != 0 {
            lupos_scx_core_enq_schedule_reenq_local(rq, 0);
        }
    }
}
