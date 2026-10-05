// SPDX-License-Identifier: GPL-2.0
// F01 source repair, ext.c:483-634 and 1159-1410, pinned at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves are unqualified C runtime.
// The six pre-existing constants and scx_set_task_slice remain root identities;
// their exact replacements are supplied separately for F00 to compose.

compile_error!("SOURCE ONLY HOLD: sched_ext slice/cursor ABI and protection qualification incomplete");

use super::*;
use core::ptr;

/// Advance from a real list node, skipping cursors without inventing a task.
///
/// # Safety
/// dsq is a live non-local DSQ with its lock held. position is its sentinel or
/// a live node currently on its list. The DSQ lock pins all traversed links and
/// task embeddings until this function returns. rev selects dispatch direction.
pub(crate) unsafe fn nldsq_next_task_after_node(
    dsq: *mut scx_dispatch_q,
    mut position: *mut list_head,
    rev: bool,
) -> *mut task_struct {
    // SAFETY: Every next/prev link belongs to the locked list. The sentinel is
    // rejected before container_of, and cursor flags are rejected before the
    // task conversion. No synthetic task pointer or Rust reference is formed.
    unsafe {
        let head = ptr::addr_of_mut!((*dsq).list);
        loop {
            position = if rev { (*position).prev } else { (*position).next };
            if position == head {
                return ptr::null_mut();
            }
            let node = lupos_scx_core_slice_node_lnode(position);
            if (*node).flags & SCX_DSQ_LNODE_ITER_CURSOR == 0 {
                return lupos_scx_core_slice_lnode_task(node);
            }
        }
    }
}

/// Iterate in either direction through tasks on a non-local DSQ.
///
/// # Safety
/// dsq is live and locked. cur is NULL to start, or a live task whose DSQ node
/// is currently on dsq. The returned task is borrowed under that same DSQ lock.
pub(crate) unsafe fn nldsq_next_task(
    dsq: *mut scx_dispatch_q,
    cur: *mut task_struct,
    rev: bool,
) -> *mut task_struct {
    // SAFETY: cur is a genuine task or NULL. addr_of_mut! neither dereferences
    // a fabricated task nor produces an exclusive reference to a shared task.
    unsafe {
        lupos_scx_core_slice_assert_next_dsq(dsq);
        let position = if cur.is_null() {
            ptr::addr_of_mut!((*dsq).list)
        } else {
            ptr::addr_of_mut!((*cur).scx.dsq_list.node)
        };
        nldsq_next_task_after_node(dsq, position, rev)
    }
}

// Rust form of ext.c:534-537. The caller declares p and supplies the unsafe
// context and nldsq_next_task Safety obligations. Advancing at loop entry keeps
// C for-loop continue semantics; break leaves p at its current task, and normal
// exhaustion leaves it NULL. The root must re-export nldsq_next_task once.
macro_rules! nldsq_for_each_task {
    ($p:ident, $dsq:expr, $body:block) => {{
        $p = core::ptr::null_mut();
        loop {
            $p = crate::nldsq_next_task($dsq, $p, false);
            if $p.is_null() {
                break;
            }
            $body
        }
    }};
}
pub(crate) use nldsq_for_each_task;

/// Advance a stable cursor, excluding tasks queued after cursor initialization.
///
/// # Safety
/// dsq is live, non-local and locked. cursor is live and initialized by the
/// native INIT_DSQ_LIST_CURSOR for dsq, with its node empty or on dsq's list.
/// Its address stays stable between calls; no competing caller uses it. The
/// DSQ lock may be dropped between calls, but not during this operation.
pub(crate) unsafe fn nldsq_cursor_next_task(
    cursor: *mut scx_dsq_list_node,
    dsq: *mut scx_dispatch_q,
) -> *mut task_struct {
    // SAFETY: The native BUG retains the invalid-cursor diagnostic. Traversal
    // starts at cursor's actual node, avoiding C's fabricated container task.
    // u32_before preserves wrap-aware queue-sequence eligibility unchanged.
    unsafe {
        let rev = (*cursor).flags & SCX_DSQ_ITER_REV != 0;
        lupos_scx_core_slice_assert_cursor_dsq(dsq);
        lupos_scx_core_slice_bug_invalid_cursor(cursor);
        let cursor_node = ptr::addr_of_mut!((*cursor).node);
        let mut position = if lupos_scx_core_slice_list_empty(cursor_node) {
            ptr::addr_of_mut!((*dsq).list)
        } else {
            cursor_node
        };
        let p = loop {
            // This is the assertion originally performed by each nested
            // nldsq_next_task call, in addition to the outer cursor assertion.
            lupos_scx_core_slice_assert_next_dsq(dsq);
            let next = nldsq_next_task_after_node(dsq, position, rev);
            if next.is_null()
                || !lupos_scx_core_slice_unlikely_cursor_newer(u32_before(
                    lupos_scx_core_slice_cursor_seq(cursor), (*next).scx.dsq_seq,
                ))
            {
                break next;
            }
            position = ptr::addr_of_mut!((*next).scx.dsq_list.node);
        };
        if p.is_null() {
            lupos_scx_core_slice_list_del_init(cursor_node);
        } else {
            let task_node = ptr::addr_of_mut!((*p).scx.dsq_list.node);
            if rev {
                lupos_scx_core_slice_list_move_tail(cursor_node, task_node);
            } else {
                lupos_scx_core_slice_list_move(cursor_node, task_node);
            }
        }
        p
    }
}

/// Revalidate a cursor-returned task after dropping and reacquiring its locks.
///
/// # Safety
/// cursor, rq, dsq and p remain live. Caller holds rq's lock and dsq's lock;
/// dsq is the non-local queue and rq the runqueue on which p was observed.
/// A task lifetime reference must survive the intervening lock release.
pub(crate) unsafe fn nldsq_cursor_lost_task(
    cursor: *mut scx_dsq_list_node,
    rq: *mut rq,
    dsq: *mut scx_dispatch_q,
    p: *mut task_struct,
) -> bool {
    // SAFETY: Both locks serialize DSQ ownership, sequence and holding_cpu.
    // Only after all three tests does the original task_rq WARN execute.
    unsafe {
        lupos_scx_core_slice_assert_lost_rq(rq);
        lupos_scx_core_slice_assert_lost_dsq(dsq);
        if lupos_scx_core_slice_unlikely_lost_task((*p).scx.dsq != dsq
            || u32_before(lupos_scx_core_slice_cursor_seq(cursor), (*p).scx.dsq_seq)
            || (*p).scx.holding_cpu >= 0)
        {
            return true;
        }
        if lupos_scx_core_slice_warn_lost_rq(rq, p) {
            return true;
        }
        false
    }
}

/// Clear a pending request using the original atomic64 read-then-set sequence.
///
/// # Safety
/// p is live and its slice_oob atomic is initialized. Caller has authority to
/// supersede this task's request at a slice update/dispatch commit boundary;
/// concurrent atomic writers remain possible exactly as in the original C.
pub(crate) unsafe fn clear_task_slice_oob(p: *mut task_struct) {
    // SAFETY: Native atomic64 operations retain their original ordering and
    // widths. This deliberately is not a swap or unconditional write.
    unsafe {
        if lupos_scx_core_slice_unlikely_clear_pending(lupos_scx_core_slice_oob_read(p) != 0) {
            lupos_scx_core_slice_oob_set(p, 0);
        }
    }
}

/// Insert at FIFO head after any leading protected tasks.
///
/// # Safety
/// dsq and p are live. Caller owns dsq's synchronization: its containing rq
/// lock for an rq-owned DSQ, otherwise dsq->lock. p's node is unlinked and
/// exclusively insertable. The rq lock stabilizes protected flags while read.
pub(crate) unsafe fn dsq_insert_head(dsq: *mut scx_dispatch_q, p: *mut task_struct) -> bool {
    // SAFETY: Native list primitives preserve link checks and update ordering.
    // Task container conversion follows cursor rejection, including the
    // original once-only diagnostic on an impossible rq-owned cursor.
    unsafe {
        let head = ptr::addr_of_mut!((*dsq).list);
        let task_node = ptr::addr_of_mut!((*p).scx.dsq_list.node);
        if !dsq_is_rq_owned(dsq) {
            lupos_scx_core_slice_list_add(task_node, head);
            return true;
        }
        let mut pos = head;
        let mut entry = (*head).next;
        while entry != head {
            let node = lupos_scx_core_slice_node_lnode(entry);
            if lupos_scx_core_slice_warn_head_cursor(node) {
                entry = (*entry).next;
                continue;
            }
            let q = lupos_scx_core_slice_lnode_task(node);
            if (*q).scx.flags & SCX_TASK_PROTECTED == 0 {
                break;
            }
            pos = entry;
            entry = (*entry).next;
        }
        lupos_scx_core_slice_list_add(task_node, pos);
        pos == head
    }
}

/// Set an unprotected slice without consuming any pending out-of-band request.
///
/// # Safety
/// p is live and the caller holds its current rq lock, stabilizing flags and
/// slice ownership. slice is the requested native u64 duration or infinity.
pub(crate) unsafe fn set_task_slice_keep_oob(p: *mut task_struct, slice: u64) -> bool {
    // SAFETY: The native rq lookup/assertion precedes the protection check.
    // Rejection leaves both slice and the OOB atomic unchanged.
    unsafe {
        lupos_scx_core_slice_assert_set_slice(p);
        if lupos_scx_core_slice_unlikely_protected((*p).scx.flags & SCX_TASK_PROTECTED != 0) {
            return false;
        }
        (*p).scx.slice = slice;
        true
    }
}

/// End slice protection and, if this task is the rescuee, its rescue session.
///
/// # Safety
/// rq and p are live; caller holds rq's lock and p belongs to it. The caller
/// supplies an actual slice-ending transition (including the original dequeue
/// save/restore exception), not an arbitrary task-attribute update.
#[no_mangle]
pub unsafe extern "C" fn scx_task_slice_ended(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: Clearing protection precedes the native rescue identity check.
    // Rescue callbacks keep the native configured ABI and original rq context.
    unsafe {
        lupos_scx_core_slice_assert_ended_rq(rq);
        (*p).scx.flags &= !SCX_TASK_PROTECTED;
        if lupos_scx_core_slice_unlikely_ended_rescue(p == lupos_scx_core_slice_rescuee(rq)) {
            lupos_scx_core_slice_rescue_end(rq);
        }
    }
}

/// Pack a deferred slice request with the issuing scheduler's owner ID.
///
/// # Safety
/// sch and p are live, their ID/atomic initialized, and the caller is authorized
/// to request p's slice for sch. Caller pins sch using the original scheduler
/// lifetime protection. No rq lock is required and preemption may be enabled.
pub(crate) unsafe fn set_task_slice_oob(sch: *mut scx_sched, p: *mut task_struct, slice: u64) {
    // SAFETY: Only the atomic is written. Ordinary scx_add_event, not its
    // preemption-disabled counterpart, accounts finite saturation. Infinity
    // consumes the reserved duration value without producing a clamp event.
    unsafe {
        let dur = if slice == SCX_SLICE_INF {
            SCX_SLICE_OOB_DUR_MASK
        } else if lupos_scx_core_slice_unlikely_clamp(slice >= SCX_SLICE_OOB_DUR_MASK) {
            lupos_scx_core_slice_event_clamped(sch);
            SCX_SLICE_OOB_DUR_MASK - 1
        } else {
            slice
        };
        lupos_scx_core_slice_oob_set(
            p,
            SCX_SLICE_OOB_PENDING
                | (((*sch).id & SCX_SLICE_OOB_ID_MASK) << SCX_SLICE_OOB_ID_SHIFT)
                | dur,
        );
    }
}

/// Apply one pending request under the task's rq lock, dropping stale owners.
///
/// # Safety
/// rq and p are live, p belongs to rq, and caller holds rq's lock with
/// preemption disabled. That lock pins the current owner and the slice cap
/// decision. Native capability/rescue helpers retain their configured behavior.
pub(crate) unsafe fn apply_task_slice_oob(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: Read avoids an unnecessary exchange; exchange consumes exactly
    // the observed winner, with the native full atomic ordering. All ownership,
    // cap and protection decisions follow consumption as in ext.c:1339-1366.
    unsafe {
        lupos_scx_core_slice_assert_apply_rq(rq);
        if lupos_scx_core_slice_likely_no_pending(lupos_scx_core_slice_oob_read(p) == 0) {
            return;
        }
        let oob = lupos_scx_core_slice_oob_xchg_zero(p);
        if lupos_scx_core_slice_unlikely_empty_exchange(oob == 0) {
            return;
        }
        if lupos_scx_core_slice_unlikely_stale_owner(
            ((oob >> SCX_SLICE_OOB_ID_SHIFT) & SCX_SLICE_OOB_ID_MASK)
                != ((*lupos_scx_core_slice_task_sched(p)).id & SCX_SLICE_OOB_ID_MASK),
        )
        {
            return;
        }
        let dur = oob & SCX_SLICE_OOB_DUR_MASK;
        let slice = if dur == SCX_SLICE_OOB_DUR_MASK { SCX_SLICE_INF } else { dur };
        if slice > (*p).scx.slice
            && lupos_scx_core_slice_unlikely_missing_caps(
                lupos_scx_core_slice_missing_base_caps(lupos_scx_core_slice_task_sched(p), rq) != 0,
            )
        {
            lupos_scx_core_slice_event_denied(lupos_scx_core_slice_task_sched(p));
            return;
        }
        if lupos_scx_core_slice_unlikely_apply_protected(!set_task_slice_keep_oob(p, slice)) {
            lupos_scx_core_slice_event_denied(lupos_scx_core_slice_task_sched(p));
        }
    }
}

/// Commit insertion verdict slice/vtime, preserving OOB on a default refill.
///
/// # Safety
/// p is live and caller has already validated and owns this insertion verdict.
/// Running, sleeping and rq-DSQ fields require p's rq lock. For a user-DSQ or
/// BPF-owned task, the BPF scheduler is responsible for writer synchronization;
/// the native source also permits a race with an rq-locked setter and specifies
/// last-writer-wins behavior. That permitted case has no established Rust
/// data-race-safe implementation here and remains an unresolved source
/// dependency. Raw-pointer syntax does not supply synchronization or prove it.
/// This hard-held candidate may only be reasoned about as executable Rust when
/// all overlapping slice/vtime accesses are synchronized; that restriction is
/// not an accepted replacement for the original supported concurrency contract.
/// Caller must enforce the original protected-task insertion restrictions.
pub(crate) unsafe fn apply_slice_vtime(
    p: *mut task_struct,
    slice: u64,
    vtime: u64,
    enq_flags: u64,
) {
    // SAFETY: For the synchronized cases described above, p's live native
    // fields are readable/writable for this commit. The allowed native racing
    // case is unresolved and cannot be justified by this unsafe block.
    // Calling the rq-locked setter here would change protection/refill/race
    // semantics; no extra protection check or request clearing is introduced.
    unsafe {
        if slice != 0 {
            (*p).scx.slice = slice;
            if enq_flags & SCX_ENQ_SLICE_DFL == 0 {
                clear_task_slice_oob(p);
            }
        } else if (*p).scx.slice == 0 {
            (*p).scx.slice = 1;
        }
        if enq_flags & SCX_ENQ_DSQ_PRIQ != 0 {
            (*p).scx.dsq_vtime = vtime;
        }
    }
}

/// Account current execution after first applying a deferred slice request.
///
/// # Safety
/// rq is live, locked and in the original update_curr_scx scheduler context.
/// rq->curr is a live ext task on this rq, with initialized accounting/server
/// state. The rq lock and disabled preemption pin all task/scheduler state.
pub(crate) unsafe fn update_curr_scx(rq: *mut rq) {
    // SAFETY: apply runs even for zero/negative execution deltas. Native common
    // accounting runs first, finite slice consumption saturates at zero, rescue
    // charge follows, then the DL server is updated with the same signed delta.
    unsafe {
        // Snapshot the original plain native field before applying OOB; the
        // configured curr/donor union is not a guessed Rust binding field.
        let curr = lupos_scx_core_rq_curr(rq);
        apply_task_slice_oob(rq, curr);
        let delta_exec = update_curr_common(rq);
        if lupos_scx_core_slice_unlikely_nonpositive_delta(delta_exec <= 0) {
            return;
        }
        if (*curr).scx.slice != SCX_SLICE_INF {
            let consumed = (*curr).scx.slice.min(delta_exec as u64);
            (*curr).scx.slice -= consumed;
        }
        if lupos_scx_core_slice_unlikely_current_rescue(curr == lupos_scx_core_slice_rescuee(rq)) {
            lupos_scx_core_slice_rescue_charge(rq, delta_exec);
        }
        dl_server_update(ptr::addr_of_mut!((*rq).ext_server), delta_exec);
    }
}
