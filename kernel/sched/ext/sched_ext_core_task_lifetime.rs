// SPDX-License-Identifier: GPL-2.0
// F06 task-lifetime source repair against ext.c:636-919,3809-4303,5516-5567
// at 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native leaves are unqualified
// C runtime boundaries, not Rust algorithm coverage or an admission claim.

compile_error!("SOURCE ONLY HOLD: sched_ext task lifetime ABI and protection qualification incomplete");

use super::*;
use core::{mem::MaybeUninit, ptr};
use kernel::ffi::{c_int, c_ulong};

/// Read the native task-state bits without changing their synchronization.
///
/// # Safety
/// p is live and readable under the original rq lock or exclusive lifecycle
/// ownership; this plain load is not an independent synchronization primitive.
/// The original sched_ext_dead pre-rq read after list removal is a separate
/// supported case: an in-flight initializer can still complete under its own
/// rq acquisition. Its Rust race treatment remains unqualified; the native
/// call sequence is not permission to assume this Rust load is synchronized.
#[no_mangle]
pub unsafe extern "C" fn scx_get_task_state(p: *const task_struct) -> u32 {
    // SAFETY: Caller pins the native entity and its flags for this plain load.
    unsafe { (*p).scx.flags & SCX_TASK_STATE_MASK }
}

/// Apply the original state-transition checks and task-state mask update.
///
/// # Safety
/// p is live with the original rq lock or exclusive lifecycle ownership. Its
/// comm/pid remain readable for diagnostics. Invalid states still warn and
/// return; invalid transitions still warn and perform the original mutation.
#[no_mangle]
pub unsafe extern "C" fn scx_set_task_state(p: *mut task_struct, state: u32) {
    // SAFETY: Only the caller-owned state word is written. Each warning retains
    // its distinct native WARN_ONCE site, format and task identity.
    unsafe {
        let prev = scx_get_task_state(p);
        let warn = match state {
            SCX_TASK_NONE => prev == SCX_TASK_DEAD,
            SCX_TASK_INIT_BEGIN => prev != SCX_TASK_NONE,
            SCX_TASK_INIT => {
                (*p).scx.flags |= SCX_TASK_RESET_RUNNABLE_AT;
                prev != SCX_TASK_INIT_BEGIN
            }
            SCX_TASK_READY => !(prev == SCX_TASK_INIT || prev == SCX_TASK_ENABLED),
            SCX_TASK_ENABLED => prev != SCX_TASK_READY,
            SCX_TASK_DEAD => !(prev == SCX_TASK_NONE || prev == SCX_TASK_INIT_BEGIN),
            _ => {
                lupos_scx_core_task_warn_state(p, prev, state);
                return;
            }
        };
        lupos_scx_core_task_warn_transition(p, prev, state, warn);
        (*p).scx.flags &= !SCX_TASK_STATE_MASK;
        (*p).scx.flags |= state;
    }
}

/// Start the original global-list or cgroup-subtree task iteration.
///
/// # Safety
/// iter is exclusively writable native storage whose address remains stable
/// until stop. It is not already active. With CONFIG_EXT_SUB_SCHED and non-NULL
/// cgrp, cgroup_lock() is held for the walk, pinning cgrp and task membership.
/// Otherwise entry permits raw_spin_lock_irq and exit holds scx_tasks_lock.
/// Every successful start must eventually stop. Between lock-dropping calls,
/// any returned task that the caller still uses needs RCU or a task reference.
#[no_mangle]
pub unsafe extern "C" fn scx_task_iter_start(iter: *mut scx_task_iter, cgrp: *mut cgroup) {
    // SAFETY: Native memset initializes the entire authoritative layout. List
    // publication happens only after initialization and under the single lock.
    unsafe {
        lupos_scx_core_task_zero_iter(iter);
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !cgrp.is_null() {
            lupos_scx_core_task_assert_cgroup_mutex();
            (*iter).cgrp = cgrp;
            (*iter).css_pos = lupos_scx_core_task_css_next(ptr::null_mut(), cgrp);
            lupos_scx_core_task_css_start((*iter).css_pos, ptr::addr_of_mut!((*iter).css_iter));
            return;
        }
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        let _ = cgrp;
        lupos_scx_core_task_list_lock_irq();
        (*iter).cursor.flags = SCX_TASK_CURSOR;
        lupos_scx_core_task_list_add(
            ptr::addr_of_mut!((*iter).cursor.tasks_node), lupos_scx_core_tasks_head());
        (*iter).list_locked = true;
    }
}

/// Drop a task rq acquired by this iterator after running balance callbacks.
///
/// # Safety
/// iter is active and exclusively managed by this caller; locked_task, rq and
/// rf describe the same outstanding task_rq_lock acquisition, or task is NULL.
pub(crate) unsafe fn __scx_task_iter_rq_unlock(iter: *mut scx_task_iter) {
    // SAFETY: Callback processing precedes unlock exactly as in the source;
    // the task pointer is cleared only after the lock release has completed.
    unsafe {
        if !(*iter).locked_task.is_null() {
            lupos_scx_core_task_balance_callbacks((*iter).rq, ptr::addr_of_mut!((*iter).rf));
            lupos_scx_core_task_rq_unlock((*iter).rq, (*iter).locked_task,
                                          ptr::addr_of_mut!((*iter).rf));
            (*iter).locked_task = ptr::null_mut();
        }
    }
}

/// Release every rq/list lock currently recorded by an active iterator.
///
/// # Safety
/// iter obeys start's stable-address/lifetime contract and owns any recorded
/// locks on this CPU. This operation does not stop or unpublish its cursor.
#[no_mangle]
pub unsafe extern "C" fn scx_task_iter_unlock(iter: *mut scx_task_iter) {
    // SAFETY: Only recorded acquisitions are released, rq before list lock.
    unsafe {
        __scx_task_iter_rq_unlock(iter);
        if (*iter).list_locked {
            (*iter).list_locked = false;
            lupos_scx_core_task_list_unlock_irq();
        }
    }
}

/// Reacquire the original global task-list lock when the iterator lacks it.
///
/// # Safety
/// iter is active, exclusively managed here, and its recorded lock state is
/// accurate. The caller may acquire the raw IRQ-disabling lock in this context.
pub(crate) unsafe fn __scx_task_iter_maybe_relock(iter: *mut scx_task_iter) {
    // SAFETY: The flag is set only after successful acquisition of the one
    // F00-owned task-list lock, never a per-family replica.
    unsafe {
        if !(*iter).list_locked {
            lupos_scx_core_task_list_lock_irq();
            (*iter).list_locked = true;
        }
    }
}

/// Restore task-list locking and optionally a selected task's rq lock.
///
/// # Safety
/// iter is active and has no outstanding rq acquisition. p is NULL or pinned
/// under RCU/a task reference across the prior unlock. Original lock ordering
/// and context apply, including cgroup_mutex when using cgroup iteration.
pub(crate) unsafe fn scx_task_iter_relock(iter: *mut scx_task_iter, p: *mut task_struct) {
    // SAFETY: The native rq_flags storage belongs to this stable iterator;
    // both the rq and task are recorded only after task_rq_lock completes.
    unsafe {
        __scx_task_iter_maybe_relock(iter);
        if !p.is_null() {
            (*iter).rq = lupos_scx_core_task_rq_lock(p, ptr::addr_of_mut!((*iter).rf));
            (*iter).locked_task = p;
        }
    }
}

/// End a started iterator, preserving the two native iteration modes.
///
/// # Safety
/// iter was started, not stopped, and stays exclusively accessible with its
/// original cgroup/list ownership. No caller may use it as active after return.
#[no_mangle]
pub unsafe extern "C" fn scx_task_iter_stop(iter: *mut scx_task_iter) {
    // SAFETY: CSS iteration is ended only for a current css_pos; global-list
    // cursor deletion is serialized and precedes the final lock release.
    unsafe {
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !(*iter).cgrp.is_null() {
            if !(*iter).css_pos.is_null() {
                lupos_scx_core_task_css_end(ptr::addr_of_mut!((*iter).css_iter));
            }
            __scx_task_iter_rq_unlock(iter);
            return;
        }
        __scx_task_iter_maybe_relock(iter);
        lupos_scx_core_task_list_del_init(ptr::addr_of_mut!((*iter).cursor.tasks_node));
        scx_task_iter_unlock(iter);
    }
}

/// Advance the original task walk, yielding every native batch interval.
///
/// # Safety
/// iter is active under start's contract. Caller tolerates dropping its locks
/// and sleeping at batch boundaries and pins previously returned tasks itself
/// if it needs them across this call. No overlapping iterator operation runs.
/// The original global walk reads task flags without the task's rq lock while
/// other bits may change under that lock. Its Rust memory-model treatment is
/// unresolved; list protection and raw pointers do not prove race safety or
/// authorize excluding this supported native case from later qualification.
pub(crate) unsafe fn scx_task_iter_next(iter: *mut scx_task_iter) -> *mut task_struct {
    // SAFETY: All list traversal occurs under scx_tasks_lock and tests the real
    // sentinel before container conversion. Cursor entities never become tasks.
    // The flags load below still needs the concurrent-reader qualification
    // described above; this source-only body does not establish that safety.
    unsafe {
        (*iter).cnt = (*iter).cnt.wrapping_add(1);
        if (*iter).cnt % SCX_TASK_ITER_BATCH == 0 {
            scx_task_iter_unlock(iter);
            lupos_scx_core_task_cond_resched();
        }
        #[cfg(CONFIG_EXT_SUB_SCHED)]
        if !(*iter).cgrp.is_null() {
            while !(*iter).css_pos.is_null() {
                let p = lupos_scx_core_task_css_next_task(ptr::addr_of_mut!((*iter).css_iter));
                if !p.is_null() {
                    return p;
                }
                lupos_scx_core_task_css_end(ptr::addr_of_mut!((*iter).css_iter));
                (*iter).css_pos = lupos_scx_core_task_css_next((*iter).css_pos, (*iter).cgrp);
                if !(*iter).css_pos.is_null() {
                    lupos_scx_core_task_css_start((*iter).css_pos,
                                                  ptr::addr_of_mut!((*iter).css_iter));
                }
            }
            return ptr::null_mut();
        }
        __scx_task_iter_maybe_relock(iter);
        let cursor = ptr::addr_of_mut!((*iter).cursor.tasks_node);
        let head = lupos_scx_core_tasks_head();
        let mut node = (*cursor).next;
        while node != cursor {
            if node == head {
                return ptr::null_mut();
            }
            let pos = lupos_scx_core_task_node_entity(node);
            if (*pos).flags & SCX_TASK_CURSOR == 0 {
                lupos_scx_core_task_list_move(cursor, node);
                return lupos_scx_core_task_entity_task(pos);
            }
            node = (*node).next;
        }
        lupos_scx_core_task_iter_bug();
    }
}

/// Return the next non-idle, non-dead task with its rq locked.
///
/// # Safety
/// iter satisfies next's contract. The borrowed result remains valid only
/// under iterator ownership, unless the caller separately pins it across an
/// unlock. Every returned rq lock must be released by an iterator operation.
/// The native idle-class filter reads sched_class before rq acquisition; a
/// concurrent class change is not excluded by task-list/cgroup protection.
/// Like next's flags read, this raw Rust read remains an unresolved supported
/// concurrency case, not a requirement for callers to add a new lock.
#[no_mangle]
pub unsafe extern "C" fn scx_task_iter_next_locked(iter: *mut scx_task_iter) -> *mut task_struct {
    // SAFETY: The previous rq is balanced/unlocked first. Idle class is checked
    // by native identity (not PF_IDLE). DEAD is tested only after rq locking,
    // synchronizing with sched_ext_dead even during cgroup task iteration.
    // That later lock does not qualify the earlier sched_class read's race.
    unsafe {
        __scx_task_iter_rq_unlock(iter);
        loop {
            let p = scx_task_iter_next(iter);
            if p.is_null() {
                return ptr::null_mut();
            }
            if (*p).sched_class == lupos_scx_core_task_idle_class() {
                continue;
            }
            (*iter).rq = lupos_scx_core_task_rq_lock(p, ptr::addr_of_mut!((*iter).rf));
            (*iter).locked_task = p;
            if scx_get_task_state(p) == SCX_TASK_DEAD {
                __scx_task_iter_rq_unlock(iter);
                continue;
            }
            return p;
        }
    }
}

/// Resolve the original task-group cgroup with the default-root fallback.
///
/// # Safety
/// tg is NULL or a live native task_group protected by the caller's task/cgroup
/// lifecycle locks; the selected cgroup remains pinned through its use.
#[cfg(CONFIG_EXT_GROUP_SCHED)]
pub(crate) unsafe fn tg_cgrp(tg: *mut task_group) -> *mut cgroup {
    // SAFETY: NULL and autogroup cases use the authoritative native root;
    // no task_group field is read before checking tg.
    unsafe {
        if !tg.is_null() && !(*tg).css.cgroup.is_null() {
            (*tg).css.cgroup
        } else {
            lupos_scx_core_task_default_cgroup()
        }
    }
}

/// Run init_task and enforce the original disallow restrictions.
///
/// # Safety
/// sch and p are pinned by the original enable/fork/cgroup migration protocol.
/// cgrp is NULL for current task-group selection or the pinned target cgroup.
/// The context permits init_task and task_rq_lock; caller performs the original
/// post-callback state/lifetime recheck before associating or enabling p.
#[no_mangle]
pub unsafe extern "C" fn __scx_init_task(
    sch: *mut scx_sched, p: *mut task_struct, cgrp: *mut cgroup, fork: bool,
) -> c_int {
    // SAFETY: The zeroed native argument object lives only through the callback.
    // CONFIG_EXT_GROUP_SCHED alone contributes its cgroup field, matching the
    // source SCX_INIT_TASK_ARGS_CGROUP expansion. No task guard is added to the
    // original SCX_CALL_OP_RET init call. Rejection policy writes hold p's rq.
    unsafe {
        (*p).scx.disallow = false;
        if lupos_scx_core_task_has_init(sch) {
            let mut args = MaybeUninit::<scx_init_task_args>::zeroed();
            let args = args.as_mut_ptr();
            #[cfg(CONFIG_EXT_GROUP_SCHED)]
            {
                (*args).cgroup = if cgrp.is_null() {
                    tg_cgrp(lupos_scx_core_task_group(p))
                } else {
                    cgrp
                };
            }
            #[cfg(not(CONFIG_EXT_GROUP_SCHED))]
            let _ = cgrp;
            (*args).fork = fork;
            let ret = lupos_scx_core_task_call_init(sch, p, args);
            if lupos_scx_core_task_unlikely_init_error(ret != 0) {
                return lupos_scx_core_task_sanitize_init_err(sch, ret);
            }
        }
        if (*p).scx.disallow {
            if lupos_scx_core_task_unlikely_disallow_parent(!lupos_scx_core_parent(sch).is_null()) {
                lupos_scx_core_task_error_disallow_parent(sch, p);
            } else if lupos_scx_core_task_unlikely_disallow_fork(fork) {
                lupos_scx_core_task_error_disallow_fork(sch, p);
            } else if lupos_scx_core_task_unlikely_disallow_enable(scx_enable_state() != SCX_ENABLING) {
                lupos_scx_core_task_error_disallow_enable(sch, p);
            } else {
                let mut rf = MaybeUninit::<rq_flags>::uninit();
                let rq = lupos_scx_core_task_rq_lock(p, rf.as_mut_ptr());
                if (*p).policy == SCHED_EXT as _ {
                    (*p).policy = SCHED_NORMAL as _;
                    lupos_scx_core_rejected_inc();
                }
                lupos_scx_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
            }
        }
        0
    }
}

/// Refresh task weight and deliver enable/set_weight before changing state.
///
/// # Safety
/// p and its associated sch are live, p's rq is held, and native static_prio
/// is in sched_prio_to_weight's original range when p lacks idle policy. Caller
/// obeys non-nesting task callback guards and the original enable protocol.
pub(crate) unsafe fn __scx_enable_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: Native macro calls retain custody warnings and task/rq guards;
    // weight is published before either callback observes the task.
    unsafe {
        let rq = lupos_scx_core_task_rq(p);
        lupos_scx_core_task_assert_enable_rq(rq);
        lupos_scx_core_task_warn_enable_custody(p);
        let weight = if lupos_scx_core_task_idle_policy(p) {
            WEIGHT_IDLEPRIO as u32
        } else {
            lupos_scx_core_task_prio_weight((*p).static_prio - MAX_RT_PRIO as c_int)
        };
        (*p).scx.weight = lupos_scx_core_task_weight_to_cgroup(weight as c_ulong) as u32;
        if lupos_scx_core_task_has_enable(sch) {
            lupos_scx_core_task_call_enable(sch, rq, p);
        }
        if lupos_scx_core_task_has_enable_weight(sch) {
            lupos_scx_core_task_call_enable_weight(sch, rq, p);
        }
    }
}

/// Enable a READY task on its associated scheduler.
///
/// # Safety
/// All __scx_enable_task requirements apply; p participates in the original
/// READY-to-ENABLED transition under its rq lock and scheduler lifetime pin.
#[no_mangle]
pub unsafe extern "C" fn scx_enable_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: Enable callbacks precede the state transition as in the source.
    unsafe {
        __scx_enable_task(sch, p);
        scx_set_task_state(p, SCX_TASK_ENABLED);
    }
}

/// Disable an enabled task and reset SCX-owned fields after its callback.
///
/// # Safety
/// sch is p's associated live scheduler; p's rq is held and the task is leaving
/// BPF custody under the original class/lifetime protocol. Caller keeps p pinned
/// while direct-dispatch, slice/rescue and callback dependencies execute.
pub(crate) unsafe fn scx_disable_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: The original warning does not skip cleanup. F02 clears direct
    // dispatch; F01 owns slice/rescue transitions. These are owner dependencies,
    // not calls to old ext.c implementations or local success substitutes.
    unsafe {
        let rq = lupos_scx_core_task_rq(p);
        lupos_scx_core_task_assert_disable_rq(rq);
        lupos_scx_core_task_warn_disable_state(p);
        super::clear_direct_dispatch(p);
        if lupos_scx_core_task_has_disable(sch) {
            lupos_scx_core_task_call_disable(sch, rq, p);
        }
        scx_set_task_state(p, SCX_TASK_READY);
        (*p).scx.dsq_vtime = 0;
        super::scx_task_slice_ended(rq, p);
        super::scx_set_task_slice(p, 0);
        (*p).scx.reenq_cnt = 0;
        lupos_scx_core_task_warn_disable_custody(p);
    }
}

/// Deliver disable/exit_task for the original eligible task states.
///
/// # Safety
/// p's pi_lock and rq lock are held. In INIT, READY or ENABLED, sch is its
/// pinned associated scheduler. NONE and the warning-only default return before
/// using sch and permit NULL, including failed-fork unwind. Caller owns the
/// lifetime transition and callback non-nesting requirements.
#[no_mangle]
pub unsafe extern "C" fn __scx_disable_and_exit_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: The native argument object is zeroed and synchronously borrowed.
    // NONE/default return without callback; only INIT is marked cancelled.
    unsafe {
        let mut args = MaybeUninit::<scx_exit_task_args>::zeroed();
        let args = args.as_mut_ptr();
        (*args).cancelled = false;
        lupos_scx_core_task_assert_exit_pi(p);
        lupos_scx_core_task_assert_exit_rq(p);
        match scx_get_task_state(p) {
            SCX_TASK_NONE => return,
            SCX_TASK_INIT => (*args).cancelled = true,
            SCX_TASK_READY => {},
            SCX_TASK_ENABLED => scx_disable_task(sch, p),
            _ => {
                lupos_scx_core_task_warn_exit_state();
                return;
            }
        }
        if lupos_scx_core_task_has_exit(sch) {
            lupos_scx_core_task_call_exit(sch, p, args);
        }
    }
}

/// Undo completed init_task on an explicit, not-yet-associated sub scheduler.
///
/// # Safety
/// sch and p remain pinned; p's pi_lock and rq lock are held. init_task on sch
/// completed and enable never ran. This is the approved cross-owner callback
/// case; it must use __SCX_CALL_OP_TASK without the associated-owner assertion.
#[no_mangle]
pub unsafe extern "C" fn scx_sub_init_cancel_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: The native cancelled argument is scoped to this synchronous call;
    // the explicit-sch callback still retains native task guards/rq tracking.
    unsafe {
        let mut args = MaybeUninit::<scx_exit_task_args>::zeroed();
        let args = args.as_mut_ptr();
        (*args).cancelled = true;
        lupos_scx_core_task_assert_cancel_pi(p);
        lupos_scx_core_task_assert_cancel_rq(p);
        if lupos_scx_core_task_has_cancel_exit(sch) {
            lupos_scx_core_task_call_cancel_exit(sch, p, args);
        }
    }
}

/// Exit both associated and pending sub-init ownership, then clear association.
///
/// # Safety
/// p is pinned with pi_lock and rq lock held. sch follows the state-dependent
/// requirements of __scx_disable_and_exit_task; NONE may have no association
/// and permit NULL. The original globally serialized sub-enable protocol pins
/// enabling_sub_sched while SCX_TASK_SUB_INIT is set. Caller must not race
/// scheduler association updates.
#[no_mangle]
pub unsafe extern "C" fn scx_disable_and_exit_task(sch: *mut scx_sched, p: *mut task_struct) {
    // SAFETY: Pending sub init is cancelled before clearing its bit, scheduler
    // association and task state. The missing-sub warning suppresses only that
    // callback, preserving the source's remaining cleanup.
    unsafe {
        __scx_disable_and_exit_task(sch, p);
        if (*p).scx.flags & SCX_TASK_SUB_INIT != 0 {
            if !lupos_scx_core_task_warn_no_sub() {
                scx_sub_init_cancel_task(lupos_scx_core_enabling_sub_sched(), p);
            }
            (*p).scx.flags &= !SCX_TASK_SUB_INIT;
        }
        lupos_scx_core_task_set_sched(p, ptr::null_mut());
        scx_set_task_state(p, SCX_TASK_NONE);
    }
}

/// Initialize an unpublished native sched_ext_entity using original defaults.
///
/// # Safety
/// scx is exclusively writable, properly aligned native storage with no live
/// list membership, rb linkage, scheduler association or outstanding observers.
#[no_mangle]
pub unsafe extern "C" fn init_scx_entity(scx: *mut sched_ext_entity) {
    // SAFETY: Native memset/list/rb operations use authoritative layout. All
    // original nonzero/sentinel defaults are written before publication.
    unsafe {
        lupos_scx_core_task_zero_entity(scx);
        lupos_scx_core_task_init_list(ptr::addr_of_mut!((*scx).dsq_list.node));
        lupos_scx_core_task_clear_rb(ptr::addr_of_mut!((*scx).dsq_priq));
        (*scx).sticky_cpu = -1;
        (*scx).holding_cpu = -1;
        (*scx).runnable_cpu = -1;
        lupos_scx_core_task_init_list(ptr::addr_of_mut!((*scx).runnable_node));
        (*scx).runnable_at = lupos_scx_core_task_jiffies();
        (*scx).ddsp_dsq_id = LUPOS_SCX_CORE_TASK_DSQ_INVALID as u64;
        (*scx).slice = LUPOS_SCX_CORE_TASK_SLICE_DFL as u64;
    }
}

/// Allocate an ID from the one native per-CPU chunk allocator.
///
/// # Safety
/// Caller may disable/re-enable preemption and obeys the original allocator
/// lifetime. F00 has initialized the shared per-CPU storage and atomic cursor.
pub(crate) unsafe fn scx_alloc_tid() -> u64 {
    // SAFETY: The per-CPU pointer is acquired and used only while preemption
    // is disabled. Unsigned additions wrap as in C; no pointer escapes after
    // preempt_enable, which matches the original guard(preempt) cleanup.
    unsafe {
        lupos_scx_core_task_preempt_disable();
        let ta = lupos_scx_core_this_tid_alloc();
        if lupos_scx_core_task_unlikely_tid_refill((*ta).next >= (*ta).end) {
            (*ta).next = lupos_scx_core_tid_cursor_fetch_add(SCX_TID_CHUNK as u64);
            (*ta).end = (*ta).next.wrapping_add(SCX_TID_CHUNK as u64);
        }
        let tid = (*ta).next;
        (*ta).next = (*ta).next.wrapping_add(1);
        lupos_scx_core_task_preempt_enable();
        tid
    }
}

/// Insert a task into F00's one TID hash with its original warning behavior.
///
/// # Safety
/// p is live with a freshly allocated tid, scx_tasks_lock is held, and the hash
/// is initialized. Caller owns publication and keeps the entity alive until
/// removal and the original reader-lifetime obligations have completed.
pub(crate) unsafe fn scx_tid_hash_insert(p: *mut task_struct) {
    // SAFETY: Original native hash parameters target p's true tid_hash_node;
    // insertion errors warn without inventing a new rollback policy.
    unsafe {
        lupos_scx_core_task_assert_tid_lock();
        let ret = lupos_scx_core_task_tid_insert(p);
        lupos_scx_core_task_warn_tid_insert(ret);
    }
}

/// Start the fork exclusion interval against scheduler enable/disable.
///
/// # Safety
/// Called in the original fork context permitting percpu_down_read, and exactly
/// one later post_fork or cancel_fork must release this acquisition.
#[no_mangle]
pub unsafe extern "C" fn scx_pre_fork(_p: *mut task_struct) {
    // SAFETY: The single native semaphore owns the original exclusion protocol.
    unsafe { lupos_scx_core_task_fork_down_read(); }
}

/// Allocate task identity and initialize scheduler ownership during fork.
///
/// # Safety
/// pre_fork's semaphore read hold remains active; p is an exclusively initialized
/// fork task, kargs is live, and its cset is pinned when sub scheduling is built.
/// Caller invokes cancel_fork on error or post_fork on success, retaining the
/// task and scheduler references until that terminal step.
#[no_mangle]
pub unsafe extern "C" fn scx_fork(p: *mut task_struct, kargs: *mut kernel_clone_args) -> c_int {
    // SAFETY: Native configured scheduler selection uses the same semaphore
    // protection. Failed init resets state before returning its exact error;
    // scheduler association is published only after successful INIT transition.
    unsafe {
        lupos_scx_core_task_assert_fork_sem();
        (*p).scx.tid = scx_alloc_tid();
        if lupos_scx_core_init_task_enabled() {
            #[cfg(CONFIG_EXT_SUB_SCHED)]
            let sch = lupos_scx_core_task_cgroup_sched(lupos_scx_core_task_fork_cgroup(kargs));
            #[cfg(not(CONFIG_EXT_SUB_SCHED))]
            let sch = lupos_scx_core_task_root_protected_live();
            scx_set_task_state(p, SCX_TASK_INIT_BEGIN);
            let ret = __scx_init_task(sch, p, ptr::null_mut(), true);
            if lupos_scx_core_task_unlikely_fork_error(ret != 0) {
                scx_set_task_state(p, SCX_TASK_NONE);
                return ret;
            }
            scx_set_task_state(p, SCX_TASK_INIT);
            lupos_scx_core_task_set_sched(p, sch);
        }
        #[cfg(not(CONFIG_EXT_SUB_SCHED))]
        let _ = kargs;
        0
    }
}

/// Finish task initialization, publish on the task list/hash and end fork hold.
///
/// # Safety
/// pre_fork and successful fork ran for this pinned p; this is their unique
/// terminal post_fork. Native class selection has completed, IRQ locking is
/// permitted, and task list/hash membership is not yet published. The native
/// scoped guard and its synchronous Rust body must return normally; no unwind
/// may cross either C ABI boundary or bypass guard destruction.
#[no_mangle]
pub unsafe extern "C" fn scx_post_fork(p: *mut task_struct) {
    // SAFETY: READY precedes optional rq-locked enable. Task publication and
    // optional hash insertion share the exact native raw_spinlock_irq guard,
    // which uses refcounted interrupt disabling in the pinned headers. Its
    // destructor completes before the fork semaphore is released.
    unsafe {
        if lupos_scx_core_init_task_enabled() {
            scx_set_task_state(p, SCX_TASK_READY);
            if (*p).sched_class == lupos_scx_core_ext_class() {
                let mut rf = MaybeUninit::<rq_flags>::uninit();
                let rq = lupos_scx_core_task_rq_lock(p, rf.as_mut_ptr());
                scx_enable_task(lupos_scx_core_task_sched(p), p);
                lupos_scx_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
            }
        }
        lupos_scx_core_task_post_fork_publish(p);
        lupos_scx_core_task_fork_up_read();
    }
}

/// Publish a forked task inside the original native scoped guard.
///
/// # Safety
/// Only the synchronous post_fork native guard adapter calls this body. It
/// holds F00's task-list lock under the native raw_spinlock_irq guard; p is
/// pinned, unpublished, and still covered by pre_fork's semaphore read hold.
/// Return normally without unwinding, sleeping or releasing the adapter's lock.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_post_fork_publish_body(p: *mut task_struct) {
    // SAFETY: The native envelope owns lock acquisition/destruction. This Rust
    // body retains the original list-first, optional-hash publication decisions.
    unsafe {
        lupos_scx_core_task_list_add_tail(ptr::addr_of_mut!((*p).scx.tasks_node),
                                          lupos_scx_core_tasks_head());
        if lupos_scx_core_tid_to_task_enabled() {
            scx_tid_hash_insert(p);
        }
    }
}

/// Cancel fork initialization and release its original semaphore read hold.
///
/// # Safety
/// This is the unique cancellation terminal for pre_fork's hold; p and any
/// initialized associated scheduler remain live and task_rq_lock is permitted.
#[no_mangle]
pub unsafe extern "C" fn scx_cancel_fork(p: *mut task_struct) {
    // SAFETY: The state warning is observational; exit still runs under the rq
    // and pi locks acquired by task_rq_lock, then the fork hold is released.
    unsafe {
        if lupos_scx_core_init_task_enabled() {
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rq = lupos_scx_core_task_rq_lock(p, rf.as_mut_ptr());
            lupos_scx_core_task_warn_cancel_state(p);
            scx_disable_and_exit_task(lupos_scx_core_task_sched(p), p);
            lupos_scx_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
        }
        lupos_scx_core_task_fork_up_read();
    }
}

/// Recognize tasks that have completed their final schedule-out.
///
/// # Safety
/// p is pinned and its rq is held; original task_on_cpu and READ_ONCE semantics
/// apply. This does not itself remove membership or release any task lifetime.
pub(crate) unsafe fn task_dead_and_done(p: *mut task_struct) -> bool {
    // SAFETY: Read state once under rq protection, preserving short-circuit:
    // task_on_cpu is queried only if TASK_DEAD was observed.
    unsafe {
        let rq = lupos_scx_core_task_rq(p);
        lupos_scx_core_task_assert_dead_rq(rq);
        lupos_scx_core_task_unlikely_dead(lupos_scx_core_task_state_read_once(p) == TASK_DEAD)
            && !lupos_scx_core_task_on_cpu(rq, p)
    }
}

/// Remove the dead task from global enumeration and terminate scheduler state.
///
/// # Safety
/// p is live but TASK_DEAD, permanently off CPU after its final switch, and this
/// is its unique sched_ext_dead call. Caller permits the original native
/// raw_spinlock_irqsave scoped guard and later task_rq_lock; task/hash reader
/// lifetime protection remains. The guard/body C ABI calls must return normally
/// without unwinding or bypassing the native guard destructor.
/// The original state read before rq locking is retained, including concurrent
/// init completion; see scx_get_task_state's unresolved race qualification.
#[no_mangle]
pub unsafe extern "C" fn sched_ext_dead(p: *mut task_struct) {
    // SAFETY: List/hash removal completes before rq acquisition. INIT_BEGIN
    // skips ops because init is still running, then marks DEAD for its owner's
    // post-init cancellation. NONE tasks retain the original no-mark behavior.
    unsafe {
        lupos_scx_core_task_dead_unpublish(p);
        if scx_get_task_state(p) != SCX_TASK_NONE {
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rq = lupos_scx_core_task_rq_lock(p, rf.as_mut_ptr());
            if scx_get_task_state(p) != SCX_TASK_INIT_BEGIN {
                scx_disable_and_exit_task(lupos_scx_core_task_sched(p), p);
            }
            scx_set_task_state(p, SCX_TASK_DEAD);
            lupos_scx_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
        }
    }
}

/// Unpublish a dead task inside the original native scoped guard.
///
/// # Safety
/// Only the synchronous dead-task native guard adapter calls this body. It
/// holds F00's task-list lock under the native raw_spinlock_irqsave guard; p
/// remains pinned through removal and every original hash/RCU reader lifetime.
/// Return normally without unwinding, sleeping or releasing the adapter's lock.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_dead_unpublish_body(p: *mut task_struct) {
    // SAFETY: List/hash removal stays in Rust under the exact native guard.
    // Its destructor runs before sched_ext_dead tests state or takes p's rq.
    unsafe {
        lupos_scx_core_task_list_del_init(ptr::addr_of_mut!((*p).scx.tasks_node));
        if lupos_scx_core_tid_to_task_enabled() {
            lupos_scx_core_task_tid_remove(p);
        }
    }
}

/// Refresh an enabled live task's weight and notify its scheduler.
///
/// # Safety
/// rq is the task's held rq; p, its associated scheduler and lw are live. This
/// runs under the original class callback protocol and native task-op guards.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_reweight_body(
    rq: *mut rq, p: *mut task_struct, lw: *const load_weight,
) {
    // SAFETY: Dead and already-disabled tasks return before reading load weight
    // or invoking ops. Native-width scaling precedes the u32 destination cast.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        lupos_scx_core_task_assert_reweight_rq(p);
        if task_dead_and_done(p) || scx_get_task_state(p) != SCX_TASK_ENABLED {
            return;
        }
        (*p).scx.weight = lupos_scx_core_task_weight_to_cgroup(
            lupos_scx_core_task_scale_load_down((*lw).weight)) as u32;
        if lupos_scx_core_task_has_reweight(sch) {
            lupos_scx_core_task_call_reweight(sch, rq, p);
        }
    }
}

/// Preserve ext.c's genuinely empty prio_changed_scx callback.
///
/// # Safety
/// Called only through the original native sched_class callback contract. No
/// argument is dereferenced; this deliberate no-op is not missing behavior.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_prio_changed_body(
    _rq: *mut rq, _p: *mut task_struct, _oldprio: u64,
) {}

/// Enable a live task entering SCX and refresh its CPU-mask callback state.
///
/// # Safety
/// rq is p's held rq, p and associated scheduler are pinned, and the original
/// sched_class transition owns its READY-to-ENABLED state change and mask.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_switching_to_body(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The dead-task check precedes enable. The native typed cpumask/CID
    // dispatch helper retains all callback guards and configured form checks.
    unsafe {
        let sch = lupos_scx_core_task_sched(p);
        if task_dead_and_done(p) {
            return;
        }
        scx_enable_task(sch, p);
        if lupos_scx_core_task_has_cpumask(sch) {
            scx_call_op_set_cpumask(sch, rq, p, (*p).cpus_ptr.cast_mut());
        }
    }
}

/// Disable a live tracked task when it leaves SCX class.
///
/// # Safety
/// The original class-switch protocol holds p's rq and associated scheduler
/// lifetime; any SCX task state still present belongs to this transition.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_switched_from_body(_rq: *mut rq, p: *mut task_struct) {
    // SAFETY: NONE is intentionally skipped after the dead-task test, avoiding
    // a bogus NONE-to-READY transition during parent teardown handback.
    unsafe {
        if task_dead_and_done(p) {
            return;
        }
        if scx_get_task_state(p) == SCX_TASK_NONE {
            return;
        }
        scx_disable_task(lupos_scx_core_task_sched(p), p);
    }
}

/// Preserve ext.c's genuinely empty switched_to_scx callback.
///
/// # Safety
/// Called under the native sched_class callback contract. No argument is read;
/// the source itself intentionally performs no operation in this callback.
#[no_mangle]
pub unsafe extern "C" fn lupos_scx_core_task_switched_to_body(_rq: *mut rq, _p: *mut task_struct) {}

/// Reject policy changes into SCHED_EXT when init_task disallowed the task.
///
/// # Safety
/// p is pinned and its rq lock held under the original setscheduler protocol;
/// policy is the requested native policy, with native signed comparison rules.
#[no_mangle]
pub unsafe extern "C" fn scx_check_setscheduler(p: *mut task_struct, policy: c_int) -> c_int {
    // SAFETY: disallow uses native READ_ONCE; short-circuit order avoids even
    // that read while disabled. The original policy comparison is unsigned.
    unsafe {
        lupos_scx_core_task_assert_setscheduler_rq(p);
        if lupos_scx_core_task_enabled()
            && lupos_scx_core_task_disallow_read_once(p)
            && (*p).policy != policy as _
            && policy == SCHED_EXT as c_int
        {
            return -(EACCES as c_int);
        }
        0
    }
}

/// Choose SCX after the caller has already handled deadline/realtime policy.
///
/// # Safety
/// Caller obeys the original sched_fork/setscheduler class-selection protocol;
/// the native static key, READ_ONCE flag and atomic state have their original
/// synchronization strength and must not be used as independent lifetime pins.
#[no_mangle]
pub unsafe extern "C" fn task_should_scx(policy: c_int) -> bool {
    // SAFETY: switching_all is tested before DISABLING. Reversing these tests
    // can route forks to skipped fair class and deadlock teardown/helper spawn.
    unsafe {
        if !lupos_scx_core_task_enabled() {
            return false;
        }
        if lupos_scx_core_switching_all_read_once() {
            return true;
        }
        if lupos_scx_core_task_unlikely_disabling(scx_enable_state() == SCX_DISABLING) {
            return false;
        }
        policy == SCHED_EXT as c_int
    }
}

/// Preserve scheduler policy for remote queued task wakeups.
///
/// # Safety
/// p remains pinned. While SCX is enabled, its pi_lock or rq lock must be held
/// for native scx_task_sched, and the caller keeps the selected scheduler/root
/// alive through the ops-flags and class reads. The native enabled key does not
/// itself pin either lifetime; configured ops-form layout also applies.
#[no_mangle]
pub unsafe extern "C" fn scx_allow_ttwu_queue(p: *const task_struct) -> bool {
    // SAFETY: Disabled, unassociated, opted-in and non-ext cases return in the
    // exact source order. Native ops access avoids inventing anonymous-union
    // layout or assuming a full cpu-form allocation for cid-form schedulers.
    unsafe {
        if !lupos_scx_core_task_enabled() {
            return true;
        }
        let sch = lupos_scx_core_task_sched(p);
        if lupos_scx_core_task_unlikely_no_sched(sch.is_null()) {
            return true;
        }
        if lupos_scx_core_task_ops_flags(sch) & LUPOS_SCX_CORE_TASK_ALLOW_QUEUED_WAKEUP as u64 != 0 {
            return true;
        }
        if lupos_scx_core_task_unlikely_non_ext((*p).sched_class != lupos_scx_core_ext_class()) {
            return true;
        }
        false
    }
}
