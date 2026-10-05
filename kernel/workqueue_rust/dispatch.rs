// SPDX-License-Identifier: GPL-2.0-only
//! Workqueue dispatch, policy and PWQ accounting.
//!
//! Source authority: kernel/workqueue.c, commit
//! 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native private types, constants,
//! state and primitive operations are supplied exclusively by `common`.
//! This is a source-only implementation; configured binding/build integration
//! has not been performed. No alternative C workqueue algorithm is called.

use super::common::*;
use super::diagnostics::{
    debug_work_activate,
    debug_work_deactivate,
    wq_cpu_intensive_report, //
};
use core::mem::MaybeUninit;
use kernel::sync::barrier::{
    smp_mb,
    Full, //
};
use core::ptr::{
    addr_of,
    addr_of_mut,
    null_mut, //
};

/// Whether an idle worker should be woken (C 952–955).
///
/// # Safety
/// `pool` is live and its lifetime is pinned. Caller holds its lock or uses
/// a native unlocked advisory context: pool-manager retry, the owning CPU's
/// BH softirq entry, or serialized dead-CPU BH draining. The observation
/// itself does not acquire a lock or confer a lifetime guarantee.
pub(super) unsafe fn need_more_worker(pool: *mut worker_pool) -> bool {
    // SAFETY: Caller pins the pool. Native list_empty and scalar-read leaves
    // retain native advisory unlocked load semantics without Rust references.
    unsafe { !lupos_wq_list_empty(addr_of!((*pool).worklist)) && lupos_wq_pool_nr_running_plain(pool) == 0 }
}

/// Whether a non-running busy worker may start (C 958–961).
///
/// # Safety
/// `pool` is live; its lock is held or the native manager owns the pool
/// lifetime while making an advisory unlocked retry observation.
pub(super) unsafe fn may_start_working(pool: *mut worker_pool) -> bool {
    // SAFETY: Caller pins the pool; the native leaf preserves its plain scalar
    // read in both locked and native manager advisory-unlocked contexts.
    unsafe { lupos_wq_pool_nr_idle_plain(pool) != 0 }
}

/// Whether a running worker should continue (C 964–967).
///
/// # Safety
/// `pool` is live and its lock is held.
pub(super) unsafe fn keep_working(pool: *mut worker_pool) -> bool {
    // SAFETY: Both fields are protected by the caller's pool lock.
    unsafe { !lupos_wq_list_empty(addr_of!((*pool).worklist)) && (*pool).nr_running <= 1 }
}

/// Whether management needs another worker (C 970–973).
///
/// # Safety
/// `pool` is live and locked, or the native manager owns its lifetime and
/// treats this result as advisory before its mandatory locked revalidation.
pub(super) unsafe fn need_to_create_worker(pool: *mut worker_pool) -> bool {
    // SAFETY: The caller meets the same lifetime/observation contract for both policies.
    unsafe { need_more_worker(pool) && !may_start_working(pool) }
}

/// Whether idle workers exceed the retention policy (C 976–983).
///
/// # Safety
/// `pool` is live and its lock is held.
pub(super) unsafe fn too_many_workers(pool: *mut worker_pool) -> bool {
    // SAFETY: The caller holds the pool lock; counters have native C int width.
    unsafe {
        let managing = ((*pool).flags & POOL_MANAGER_ACTIVE) != 0;
        let nr_idle = (*pool).nr_idle.wrapping_add(managing as c_int);
        let nr_busy = (*pool).nr_workers.wrapping_sub(nr_idle);
        nr_idle > 2
            && nr_idle.wrapping_sub(2).wrapping_mul(MAX_IDLE_WORKERS_RATIO as c_int)
                >= nr_busy
    }
}

/// Set flags and retire a running count on the first NOT_RUNNING bit (C 992–1005).
///
/// # Safety
/// `worker` and its pool are live; the matching pool lock is held.
pub(super) unsafe fn worker_set_flags(worker: *mut worker, flags: c_uint) {
    // SAFETY: The caller serializes flags and nr_running through the pool lock.
    unsafe {
        let pool = (*worker).pool;
        lupos_wq_assert_pool_locked(pool);
        if flags & WORKER_NOT_RUNNING != 0 && (*worker).flags & WORKER_NOT_RUNNING == 0 {
            (*pool).nr_running = (*pool).nr_running.wrapping_sub(1);
        }
        (*worker).flags |= flags;
    }
}

/// Clear flags and restore running only after the last NOT_RUNNING bit (C 1014–1031).
///
/// # Safety
/// `worker` and its pool are live; the matching pool lock is held.
pub(super) unsafe fn worker_clr_flags(worker: *mut worker, flags: c_uint) {
    // SAFETY: Flags and running count are protected by the caller's pool lock.
    unsafe {
        let pool = (*worker).pool;
        let old_flags = (*worker).flags;
        lupos_wq_assert_pool_locked(pool);
        (*worker).flags &= !flags;
        if flags & WORKER_NOT_RUNNING != 0 && old_flags & WORKER_NOT_RUNNING != 0 {
            if (*worker).flags & WORKER_NOT_RUNNING == 0 {
                (*pool).nr_running = (*pool).nr_running.wrapping_add(1);
            }
        }
    }
}

/// Return the most recently idled worker, or NULL (C 1034–1040).
///
/// # Safety
/// `pool` is live and its lock is held.
pub(super) unsafe fn first_idle_worker(pool: *mut worker_pool) -> *mut worker {
    // SAFETY: The lock stabilizes list membership; conversion uses native layout.
    unsafe {
        let head = addr_of!((*pool).idle_list);
        if lupos_wq_list_empty(head) {
            return null_mut();
        }
        lupos_wq_worker_from_entry((*head).next)
    }
}

/// Enter idle state and maintain the LIFO idle list (C 1052–1074).
///
/// # Safety
/// `worker` and its pool are live, with pool lock held and IRQs disabled, or
/// this is the native create-worker initialization context excluding observers.
pub(super) unsafe fn worker_enter_idle(worker: *mut worker) {
    // SAFETY: Caller owns the idle/busy union and pool state. The native union
    // accessors do not create references or assume an independently defined ABI.
    unsafe {
        let pool = (*worker).pool;
        if lupos_wq_warn_worker_enter_idle_already_idle((*worker).flags & WORKER_IDLE != 0)
            || lupos_wq_warn_worker_enter_idle_linked_busy(
                !lupos_wq_list_empty(lupos_wq_worker_entry(worker))
                    && (!(*lupos_wq_worker_hentry(worker)).next.is_null()
                        || !(*lupos_wq_worker_hentry(worker)).pprev.is_null())
            )
        {
            return;
        }
        // Do not call worker_set_flags: creation also enters through this path.
        (*worker).flags |= WORKER_IDLE;
        (*pool).nr_idle = (*pool).nr_idle.wrapping_add(1);
        (*worker).last_active = lupos_wq_read_jiffies();
        lupos_wq_list_add(lupos_wq_worker_entry(worker), addr_of_mut!((*pool).idle_list));
        if too_many_workers(pool) && !lupos_wq_timer_pending(addr_of!((*pool).idle_timer)) {
            mod_timer(
                addr_of_mut!((*pool).idle_timer),
                lupos_wq_read_jiffies().wrapping_add(IDLE_WORKER_TIMEOUT as c_ulong),
            );
        }
        lupos_wq_warn_worker_enter_idle_running((*pool).nr_workers == (*pool).nr_idle && (*pool).nr_running != 0);
    }
}

/// Leave idle state (C 1085–1094).
///
/// # Safety
/// `worker` and its pool are live; pool lock is held with IRQs disabled.
pub(super) unsafe fn worker_leave_idle(worker: *mut worker) {
    // SAFETY: Caller serializes idle flags, the union list node and pool counters.
    unsafe {
        let pool = (*worker).pool;
        if lupos_wq_warn_worker_leave_idle_not_idle((*worker).flags & WORKER_IDLE == 0) {
            return;
        }
        worker_clr_flags(worker, WORKER_IDLE);
        (*pool).nr_idle = (*pool).nr_idle.wrapping_sub(1);
        lupos_wq_list_del_init(lupos_wq_worker_entry(worker));
    }
}

/// Find an execution with both matching address and callback (C 1129–1141).
///
/// # Safety
/// `pool` is live and locked; `work` is a live item whose callback can be read.
/// An executing worker's saved `current_work` is compared only, never dereferenced.
pub(super) unsafe fn find_worker_executing_work(
    pool: *mut worker_pool,
    work: *mut work_struct,
) -> *mut worker {
    // SAFETY: Pool locking pins busy hash members. The native hash-head leaf
    // supplies exactly hash_min((unsigned long)work, BUSY_WORKER_HASH_ORDER).
    unsafe {
        let mut node = (*lupos_wq_busy_worker_hash_head(pool, work)).first;
        while !node.is_null() {
            let worker = lupos_wq_worker_from_hentry(node);
            if (*worker).current_work == work
                && lupos_wq_work_func_eq((*worker).current_func, (*work).func)
            {
                return worker;
            }
            node = (*node).next;
        }
        null_mut()
    }
}

/// Sentinel callback; executing a mayday cursor is a fatal invariant failure (C 1143–1147).
///
/// # Safety
/// This is an ABI-compatible callback identity, never a runnable work item.
pub(super) unsafe extern "C" fn mayday_cursor_func(_work: *mut work_struct) {
    // SAFETY: Native BUG unconditionally terminates the invalid execution path.
    unsafe { lupos_wq_bug_mayday_cursor_func() }
}

/// Move one complete linked chain, preserving the outer safe-iterator cursor (C 1163–1185).
///
/// # Safety
/// `work` starts a well-formed native linked work chain which terminates before
/// its list head, both lists are owned under their pool lock, and non-NULL
/// `nextp` is writable. The returned cursor may denote a list-head container
/// sentinel and must not be dereferenced until the outer iterator checks it.
pub(super) unsafe fn move_linked_works(
    mut work: *mut work_struct,
    head: *mut list_head,
    nextp: *mut *mut work_struct,
) {
    // SAFETY: Caller supplies an owned chain ending before the head. Compute the
    // next container before moving, as native list_for_each_entry_safe_from does.
    unsafe {
        loop {
            let next = lupos_wq_work_from_entry((*addr_of!((*work).entry)).next);
            lupos_wq_list_move_tail(addr_of_mut!((*work).entry), head);
            if *work_data_bits(work) & (WORK_STRUCT_LINKED as c_ulong) == 0 {
                if !nextp.is_null() {
                    *nextp = next;
                }
                break;
            }
            work = next;
        }
    }
}

/// Assign a chain, diverting a collision to its currently executing worker (C 1203–1237).
///
/// # Safety
/// Worker, work chain and matching pool are live and locked. `nextp` obeys the
/// move_linked_works contract; cursor work is only supplied to worker_thread.
pub(super) unsafe fn assign_work(
    work: *mut work_struct,
    worker: *mut worker,
    nextp: *mut *mut work_struct,
) -> bool {
    // SAFETY: All work/worker list changes occur under the caller's pool lock.
    unsafe {
        let pool = (*worker).pool;
        lupos_wq_assert_pool_locked(pool);
        if lupos_wq_work_func_eq((*work).func, Some(mayday_cursor_func)) {
            lupos_wq_warn_assign_work_rescuer(!(*worker).rescue_wq.is_null());
            if !nextp.is_null() {
                *nextp = lupos_wq_work_from_entry((*addr_of!((*work).entry)).next);
            }
            lupos_wq_list_del_init(addr_of_mut!((*work).entry));
            return false;
        }
        let collision = find_worker_executing_work(pool, work);
        if !collision.is_null() {
            move_linked_works(work, addr_of_mut!((*collision).scheduled), nextp);
            return false;
        }
        move_linked_works(work, addr_of_mut!((*worker).scheduled), nextp);
        true
    }
}

/// Select the normal/high-priority per-CPU BH irq-work object (C 1239–1244).
///
/// # Safety
/// `pool` is a live BH pool with valid CPU and immutable attrs.
pub(super) unsafe fn bh_pool_irq_work(pool: *mut worker_pool) -> *mut irq_work {
    // SAFETY: Native per-CPU address access handles configured storage layout.
    unsafe {
        let high = ((*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL) as c_int;
        lupos_wq_bh_pool_irq_work_at((*pool).cpu, high)
    }
}

/// Kick local softirq, or remote irq-work for an SMP BH pool (C 1246–1260).
///
/// # Safety
/// `pool` is a live BH pool; pool lock is held and local IRQs are disabled.
pub(super) unsafe fn kick_bh_pool(pool: *mut worker_pool) {
    // SAFETY: IRQ-disabled pool context satisfies softirq and irq-work contracts.
    unsafe {
        #[cfg(CONFIG_SMP)]
        if (*pool).cpu != lupos_wq_smp_processor_id() as c_int && (*pool).flags & POOL_BH_DRAINING == 0 {
            irq_work_queue_on(bh_pool_irq_work(pool), (*pool).cpu);
            return;
        }
        if (*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL {
            lupos_wq_raise_softirq_irqoff(HI_SOFTIRQ);
        } else {
            lupos_wq_raise_softirq_irqoff(TASKLET_SOFTIRQ);
        }
    }
}

/// Pick a task under pool lock, leaving its wakeup to the caller (C 1275–1329).
///
/// # Safety
/// `pool` is live, locked with IRQs disabled and `wakep` is writable. A returned
/// task must remain alive through the deferred wake using the caller's native
/// RCU/lifetime window. This function does not acquire a task reference.
pub(super) unsafe fn kick_pool_pick(
    pool: *mut worker_pool,
    wakep: *mut *mut task_struct,
) -> bool {
    // SAFETY: Pool lock stabilizes idle worker selection and worklist. Only
    // native READ/WRITE_ONCE leaves access scheduler-owned wake_cpu concurrently.
    unsafe {
        let worker = first_idle_worker(pool);
        lupos_wq_assert_pool_locked(pool);
        *wakep = null_mut();
        if !need_more_worker(pool) || worker.is_null() {
            return false;
        }
        if (*pool).flags & POOL_BH != 0 {
            kick_bh_pool(pool);
            return true;
        }
        let task = (*worker).task;
        #[cfg(CONFIG_SMP)]
        {
            let attrs = (*pool).attrs;
            if !(*attrs).affn_strict
                && !lupos_wq_cpumask_test_cpu(lupos_wq_read_task_wake_cpu(task), lupos_wq_attrs_pod_cpumask(attrs))
            {
                let work = lupos_wq_work_from_entry((*addr_of!((*pool).worklist)).next);
                let wake_cpu = lupos_wq_cpumask_any_and_distribute(lupos_wq_attrs_pod_cpumask(attrs), lupos_wq_cpu_online_mask()) as c_int;
                // C compares signed wake_cpu with unsigned nr_cpu_ids.
                if (wake_cpu as c_uint) < lupos_wq_nr_cpu_ids() {
                    lupos_wq_write_task_wake_cpu(task, wake_cpu);
                    let pwq = get_work_pwq(work);
                    let stat = addr_of_mut!((*pwq).stats[PWQ_STAT_REPATRIATED as usize]);
                    *stat = (*stat).wrapping_add(1);
                }
            }
        }
        (*pool).last_woken_worker = worker;
        *wakep = task;
        true
    }
}

/// Select and immediately wake the idle worker, as native callers require (C 1338–1346).
///
/// # Safety
/// `pool` is live and locked with IRQs disabled. Callers requiring an unlocked
/// wake must call kick_pool_pick instead and retain its task lifetime guarantee.
pub(super) unsafe fn kick_pool(pool: *mut worker_pool) -> bool {
    // SAFETY: The caller keeps the pool lock across selection and immediate wake.
    unsafe {
        let mut task = null_mut();
        let kicked = kick_pool_pick(pool, &mut task);
        if !task.is_null() {
            wake_up_process(task);
        }
        kicked
    }
}

/// Identify pools which enforce active limits per PWQ (C 1609–1612).
///
/// # Safety
/// `pool` is live; its CPU association is immutable during the access.
pub(super) unsafe fn is_percpu_pool(pool: *mut worker_pool) -> bool {
    // SAFETY: The live pool's immutable CPU field can be read by this caller.
    unsafe { (*pool).cpu >= 0 }
}

/// Resolve a NUMA active-count slot, including the no-node fallback (C 1626–1635).
///
/// # Safety
/// `wq` is a live unbound workqueue; node is NUMA_NO_NODE or a valid node index.
pub(super) unsafe fn wq_node_nr_active(
    wq: *mut workqueue_struct,
    mut node: c_int,
) -> *mut wq_node_nr_active {
    // SAFETY: The caller ensures the native flexible array has the requested
    // configured slot. Address calculation is provided by the native layout leaf.
    unsafe {
        if (*wq).flags & WQ_UNBOUND == 0 {
            lupos_wq_bug_wq_node_nr_active_bound();
        }
        if node == NUMA_NO_NODE {
            node = lupos_wq_nr_node_ids() as c_int;
        }
        *lupos_wq_node_nr_active_slot(wq, node)
    }
}

/// Redistribute unbound active limits across possible NUMA nodes (C 1646–1687).
///
/// # Safety
/// `wq` is live and unbound, its mutex is held and native topology/hotplug
/// exclusion stabilizes masks and node slots. off_cpu is -1 or the CPU going down.
pub(super) unsafe fn wq_update_node_max_active(wq: *mut workqueue_struct, mut off_cpu: c_int) {
    // SAFETY: Caller owns workqueue configuration and topology. READ_ONCE leaves
    // preserve native concurrent scalar access; all node pointers are live slots.
    unsafe {
        let effective = unbound_effective_cpumask(wq);
        let min_active = lupos_wq_read_once_int(addr_of!((*wq).min_active));
        let max_active = lupos_wq_read_once_int(addr_of!((*wq).max_active));
        lupos_wq_assert_wq_mutex_held(wq);
        if !lupos_wq_topo_initialized() {
            return;
        }
        if off_cpu >= 0 && !lupos_wq_cpumask_test_cpu(off_cpu, effective) {
            off_cpu = -1;
        }
        let mut total_cpus =
            lupos_wq_cpumask_weight_and(effective, lupos_wq_cpu_online_mask()) as c_int;
        if off_cpu >= 0 {
            total_cpus = total_cpus.wrapping_sub(1);
        }
        if total_cpus == 0 {
            let mut node = lupos_wq_first_possible_node() as c_int;
            while node < MAX_NUMNODES as c_int {
                (*wq_node_nr_active(wq, node)).max = min_active;
                node = lupos_wq_next_possible_node(node) as c_int;
            }
            (*wq_node_nr_active(wq, NUMA_NO_NODE)).max = max_active;
            return;
        }
        let mut node = lupos_wq_first_possible_node() as c_int;
        while node < MAX_NUMNODES as c_int {
            let mut node_cpus =
                lupos_wq_cpumask_weight_and(effective, lupos_wq_cpumask_of_node(node)) as c_int;
            if off_cpu >= 0 && lupos_wq_cpu_to_node(off_cpu) == node {
                node_cpus = node_cpus.wrapping_sub(1);
            }
            // C DIV_ROUND_UP is signed-int arithmetic here. Kernel C disables
            // strict signed-overflow optimization; retain width and wrapping.
            let share = max_active
                .wrapping_mul(node_cpus)
                .wrapping_add(total_cpus)
                .wrapping_sub(1)
                / total_cpus;
            (*wq_node_nr_active(wq, node)).max = share.max(min_active).min(max_active);
            node = lupos_wq_next_possible_node(node) as c_int;
        }
        (*wq_node_nr_active(wq, NUMA_NO_NODE)).max = max_active;
    }
}

/// Acquire an additional positive PWQ reference (C 1696–1701).
///
/// # Safety
/// `pwq` is live, has a positive refcount and its matching pool lock is held.
pub(super) unsafe fn get_pwq(pwq: *mut pool_workqueue) {
    // SAFETY: Caller serializes the refcount and guarantees its preexisting owner.
    unsafe {
        lupos_wq_assert_pool_locked((*pwq).pool);
        lupos_wq_warn_get_pwq_nonpositive((*pwq).refcnt <= 0);
        (*pwq).refcnt = (*pwq).refcnt.wrapping_add(1);
    }
}

/// Drop a PWQ reference, deferring last-reference destruction (C 1710–1720).
///
/// # Safety
/// `pwq` is live with an owned reference and matching pool lock held. No caller
/// may keep using it after the last reference without native RCU protection.
pub(super) unsafe fn put_pwq(pwq: *mut pool_workqueue) {
    // SAFETY: Caller serializes the refcount; dedicated kthread work owns the
    // deferred destruction, so release never runs synchronously under pool lock.
    unsafe {
        lupos_wq_assert_pool_locked((*pwq).pool);
        (*pwq).refcnt = (*pwq).refcnt.wrapping_sub(1);
        if (*pwq).refcnt != 0 {
            return;
        }
        kthread_queue_work(lupos_wq_pwq_release_worker(), addr_of_mut!((*pwq).release_work));
    }
}

/// Drop an optional PWQ reference with native IRQ-off locking (C 1728–1739).
///
/// # Safety
/// A non-NULL pwq carries an owned reference; caller permits IRQ disable/enable.
/// Native RCU protection of PWQ and pool covers the final put/unlock sequence.
pub(super) unsafe fn put_pwq_unlocked(pwq: *mut pool_workqueue) {
    // SAFETY: The reference protects lock acquisition. The native deferred RCU
    // release and IRQ-off locked interval protect both objects through unlock.
    unsafe {
        if !pwq.is_null() {
            lupos_wq_pool_lock_irq((*pwq).pool);
            put_pwq(pwq);
            lupos_wq_pool_unlock_irq((*pwq).pool);
        }
    }
}

/// Whether a PWQ has neither active nor inactive work (C 1741–1744).
///
/// # Safety
/// `pwq` is live and its pool lock is held.
pub(super) unsafe fn pwq_is_empty(pwq: *mut pool_workqueue) -> bool {
    // SAFETY: Both fields are stable under the caller's pool lock.
    unsafe { (*pwq).nr_active == 0 && lupos_wq_list_empty(addr_of!((*pwq).inactive_works)) }
}

/// Move an inactive linked chain to the runnable list (C 1746–1757).
///
/// # Safety
/// `pwq`, its pool and work chain are live and locked; the caller has acquired
/// the needed active count. work is the first non-barrier inactive item.
pub(super) unsafe fn __pwq_activate_work(pwq: *mut pool_workqueue, work: *mut work_struct) {
    // SAFETY: Caller owns this chain and its work data under matching pool lock.
    unsafe {
        let bits = work_data_bits(work);
        lupos_wq_warn_pwq_activate_work_not_inactive(*bits & (WORK_STRUCT_INACTIVE as c_ulong) == 0);
        lupos_wq_trace_workqueue_activate_work(work);
        let pool = (*pwq).pool;
        if lupos_wq_list_empty(addr_of!((*pool).worklist)) {
            (*pool).last_progress_ts = lupos_wq_read_jiffies();
        }
        move_linked_works(work, addr_of_mut!((*pool).worklist), null_mut());
        lupos_wq___clear_bit(WORK_STRUCT_INACTIVE_BIT as c_uint, bits);
    }
}

/// Acquire one NUMA active slot using relaxed compare-exchange (C 1759–1770).
///
/// # Safety
/// `nna` is live for the operation; its scalar/atomic native accessors are valid.
pub(super) unsafe fn tryinc_node_nr_active(nna: *mut wq_node_nr_active) -> bool {
    // SAFETY: max is read once and nr is accessed only by native atomic leaves.
    unsafe {
        let max = lupos_wq_read_once_int(addr_of!((*nna).max));
        let mut old = lupos_wq_atomic_read(addr_of!((*nna).nr));
        loop {
            if old >= max {
                return false;
            }
            let new = old.wrapping_add(1);
            if lupos_wq_atomic_try_cmpxchg_relaxed(addr_of_mut!((*nna).nr), &mut old, new) {
                return true;
            }
        }
    }
}

/// Try to acquire an active count while preserving pending-PWQ fairness (C 1780–1850).
///
/// # Safety
/// `pwq`, its workqueue and pool are live; matching pool lock is held with IRQs
/// disabled. fill is reserved for restoring concurrency after a limit increase.
pub(super) unsafe fn pwq_tryinc_nr_active(pwq: *mut pool_workqueue, fill: bool) -> bool {
    // SAFETY: Pool lock owns PWQ fields; nna lock nests inside it and serializes
    // pending membership. Native atomics/barrier implement the publication pair.
    unsafe {
        let wq = (*pwq).wq;
        let pool = (*pwq).pool;
        lupos_wq_assert_pool_locked(pool);
        let obtained;
        if is_percpu_pool(pool) {
            obtained = (*pwq).nr_active < lupos_wq_read_once_int(addr_of!((*wq).max_active));
        } else {
            if (*pwq).plugged {
                return false;
            }
            let nna = wq_node_nr_active(wq, (*pool).node);
            if !lupos_wq_list_empty(addr_of!((*pwq).pending_node)) && !fill {
                return false;
            }
            if tryinc_node_nr_active(nna) {
                obtained = true;
            } else {
                lupos_wq_nna_lock(nna);
                if lupos_wq_list_empty(addr_of!((*pwq).pending_node)) {
                    lupos_wq_list_add_tail(
                        addr_of_mut!((*pwq).pending_node),
                        addr_of_mut!((*nna).pending_pwqs),
                    );
                } else if !fill {
                    lupos_wq_nna_unlock(nna);
                    return false;
                }
                // Pairs with the full barrier in atomic_dec_return below.
                smp_mb(Full);
                obtained = tryinc_node_nr_active(nna);
                if obtained && !fill {
                    lupos_wq_list_del_init(addr_of_mut!((*pwq).pending_node));
                }
                lupos_wq_nna_unlock(nna);
            }
        }
        if obtained {
            (*pwq).nr_active = (*pwq).nr_active.wrapping_add(1);
        }
        obtained
    }
}

/// Activate the oldest inactive item if a count is available (C 1863–1875).
///
/// # Safety
/// `pwq` and its pool are live; pool lock held with IRQs disabled.
pub(super) unsafe fn pwq_activate_first_inactive(pwq: *mut pool_workqueue, fill: bool) -> bool {
    // SAFETY: Caller stabilizes the inactive list and meets active-count contract.
    unsafe {
        let head = addr_of!((*pwq).inactive_works);
        if lupos_wq_list_empty(head) {
            return false;
        }
        let work = lupos_wq_work_from_entry((*head).next);
        if pwq_tryinc_nr_active(pwq, fill) {
            __pwq_activate_work(pwq, work);
            true
        } else {
            false
        }
    }
}

/// Unplug the oldest ordered PWQ and restore activation/pending accounting (C 1899–1927).
///
/// # Safety
/// `wq` is live and ordered, mutex is held, and its PWQ list is nonempty. Caller
/// permits IRQ disable/enable and the native immediate kick under pool lock.
pub(super) unsafe fn unplug_oldest_pwq(wq: *mut workqueue_struct) {
    // SAFETY: Mutex stabilizes the nonempty list; pool lock protects unplugging
    // and activation. The second activation attempt restores pending membership.
    unsafe {
        lupos_wq_assert_wq_mutex_held(wq);
        let head = addr_of!((*wq).pwqs);
        let pwq = if lupos_wq_list_empty(head) {
            null_mut()
        } else {
            lupos_wq_pwq_from_pwqs_node((*head).next)
        };
        // Native requires nonempty pwqs; do not invent a success-on-empty path.
        lupos_wq_pool_lock_irq((*pwq).pool);
        if (*pwq).plugged {
            (*pwq).plugged = false;
            if pwq_activate_first_inactive(pwq, true) {
                pwq_activate_first_inactive(pwq, false);
                kick_pool((*pwq).pool);
            }
        }
        lupos_wq_pool_unlock_irq((*pwq).pool);
    }
}

/// Activate a pending PWQ, changing locked pools when necessary (C 1937–2008).
///
/// # Safety
/// nna and caller_pool are live under native IRQ-off/RCU lifetime protection;
/// caller_pool is locked with IRQs disabled. All caller state changes must be
/// complete because its lock can be dropped. Returns with caller_pool locked.
pub(super) unsafe fn node_activate_pending_pwq(
    nna: *mut wq_node_nr_active,
    caller_pool: *mut worker_pool,
) {
    // SAFETY: IRQ-off native lifetime protection spans pool lock handoffs. The
    // nna lock is dropped before blocking for a foreign pool and list selection
    // is restarted after reacquiring it; stale PWQ selections are never reused.
    unsafe {
        let mut locked_pool = caller_pool;
        lupos_wq_assert_pool_locked(caller_pool);
        lupos_wq_nna_lock(nna);
        loop {
            let pending = addr_of!((*nna).pending_pwqs);
            if lupos_wq_list_empty(pending) {
                break;
            }
            let pwq = lupos_wq_pwq_from_pending_node((*pending).next);
            if (*pwq).pool != locked_pool {
                lupos_wq_pool_unlock(locked_pool);
                locked_pool = (*pwq).pool;
                if !lupos_wq_pool_trylock(locked_pool) {
                    lupos_wq_nna_unlock(nna);
                    lupos_wq_pool_lock(locked_pool);
                    lupos_wq_nna_lock(nna);
                    continue;
                }
            }
            let inactive = addr_of!((*pwq).inactive_works);
            if lupos_wq_list_empty(inactive) {
                lupos_wq_list_del_init(addr_of_mut!((*pwq).pending_node));
                continue;
            }
            let work = lupos_wq_work_from_entry((*inactive).next);
            if tryinc_node_nr_active(nna) {
                (*pwq).nr_active = (*pwq).nr_active.wrapping_add(1);
                __pwq_activate_work(pwq, work);
                if lupos_wq_list_empty(addr_of!((*pwq).inactive_works)) {
                    lupos_wq_list_del_init(addr_of_mut!((*pwq).pending_node));
                } else {
                    lupos_wq_list_move_tail(
                        addr_of_mut!((*pwq).pending_node),
                        addr_of_mut!((*nna).pending_pwqs),
                    );
                }
                if (*pwq).pool != caller_pool {
                    kick_pool((*pwq).pool);
                }
            }
            break;
        }
        lupos_wq_nna_unlock(nna);
        if locked_pool != caller_pool {
            lupos_wq_pool_unlock(locked_pool);
            lupos_wq_pool_lock(caller_pool);
        }
    }
}

/// Retire one active count, possibly handing it to another PWQ (C 2017–2060).
///
/// # Safety
/// `pwq` carries an owned active count and is live with pool locked, IRQs off.
/// Caller state changes are complete before this may release the pool lock.
pub(super) unsafe fn pwq_dec_nr_active(pwq: *mut pool_workqueue) {
    // SAFETY: Caller owns PWQ active count; shared counts use native atomics.
    // native list_empty supplies the lockless READ_ONCE pending observation.
    unsafe {
        let pool = (*pwq).pool;
        lupos_wq_assert_pool_locked(pool);
        (*pwq).nr_active = (*pwq).nr_active.wrapping_sub(1);
        if is_percpu_pool(pool) {
            pwq_activate_first_inactive(pwq, false);
            return;
        }
        let nna = wq_node_nr_active((*pwq).wq, (*pool).node);
        // Full native atomic ordering pairs with publication smp_mb above.
        if lupos_wq_atomic_dec_return(addr_of_mut!((*nna).nr))
            >= lupos_wq_read_once_int(addr_of!((*nna).max))
        {
            return;
        }
        if !lupos_wq_list_empty(addr_of!((*nna).pending_pwqs)) {
            node_activate_pending_pwq(nna, pool);
        }
    }
}

/// Retire a work's color and reference, completing the first flusher if last (C 2078–2106).
///
/// # Safety
/// `pwq` has an owned in-flight count/reference described by work_data; its pool
/// is locked with IRQs disabled. All other work state changes must be complete
/// before this call, which may drop the lock and consume the last PWQ reference.
pub(super) unsafe fn pwq_dec_nr_in_flight(pwq: *mut pool_workqueue, work_data: c_ulong) {
    // SAFETY: Caller owns the in-flight reference. Color is decoded by common
    // from native encoded bits and indexes the configured native color array.
    unsafe {
        let color = get_work_color(work_data);
        if work_data & (WORK_STRUCT_INACTIVE as c_ulong) == 0 {
            pwq_dec_nr_active(pwq);
        }
        let count = addr_of_mut!((*pwq).nr_in_flight[color as usize]);
        *count = (*count).wrapping_sub(1);
        if (*pwq).flush_color == color && *count == 0 {
            (*pwq).flush_color = -1;
            let wq = (*pwq).wq;
            if lupos_wq_atomic_dec_and_test(addr_of_mut!((*wq).nr_pwqs_to_flush)) {
                // Native single completion, never complete_all or polling.
                complete(addr_of_mut!((*(*wq).first_flusher).done));
            }
        }
        put_pwq(pwq);
    }
}

/// Claim PENDING or steal a queued/timer work, retaining IRQ ownership on success (C 2136–2223).
///
/// # Safety
/// `work` is live and cancelable, not a linked flush barrier or inner RCU work.
/// WORK_CANCEL_DELAYED requires an enclosing live delayed_work with IRQ-safe
/// timer. irq_flags is writable. Caller restores saved IRQ state after success.
pub(super) unsafe fn try_to_grab_pending(
    work: *mut work_struct,
    cflags: u32,
    irq_flags: *mut c_ulong,
) -> c_int {
    // SAFETY: Caller provides a stable cancelable work lifetime. IRQ-off plus
    // IRQ-safe timer excludes local incomplete ownership transitions; RCU pins
    // pool/PWQ observations until after pool unlock on every non-timer path.
    unsafe {
        *irq_flags = lupos_wq_local_irq_save();
        if cflags & WORK_CANCEL_DELAYED != 0 {
            let dwork = lupos_wq_to_delayed_work(work);
            if timer_delete(addr_of_mut!((*dwork).timer)) != 0 {
                return 1;
            }
        }
        if !lupos_wq_test_and_set_bit(WORK_STRUCT_PENDING_BIT as c_long, work_data_bits(work)) {
            return 0;
        }
        lupos_wq_rcu_read_lock();
        let pool = get_work_pool(work);
        if !pool.is_null() {
            lupos_wq_pool_lock(pool);
            let pwq = get_work_pwq(work);
            if !pwq.is_null() && (*pwq).pool == pool {
                let work_data = *work_data_bits(work);
                debug_work_deactivate(work);
                if work_data & (WORK_STRUCT_INACTIVE as c_ulong) != 0 {
                    // Preserve linked barriers and keep INACTIVE in the saved
                    // data, so retirement does not consume an active count.
                    move_linked_works(work, addr_of_mut!((*(*pwq).pool).worklist), null_mut());
                }
                lupos_wq_list_del_init(addr_of_mut!((*work).entry));
                set_work_pool_and_keep_pending(work, (*pool).id, pool_offq_flags(pool));
                // This is last: retirement may drop and retake the pool lock.
                pwq_dec_nr_in_flight(pwq, work_data);
                lupos_wq_pool_unlock(pool);
                lupos_wq_rcu_read_unlock();
                return 1;
            }
            lupos_wq_pool_unlock(pool);
        }
        lupos_wq_rcu_read_unlock();
        lupos_wq_local_irq_restore(*irq_flags);
        -(EAGAIN as c_int)
    }
}

/// Retry a transient PENDING owner until the caller owns it (C 2240–2251).
///
/// # Safety
/// Same requirements as try_to_grab_pending. Returns with IRQs disabled and the
/// saved state in irq_flags; the caller owns PENDING and must eventually restore.
pub(super) unsafe fn work_grab_pending(
    work: *mut work_struct,
    cflags: u32,
    irq_flags: *mut c_ulong,
) -> bool {
    // SAFETY: Caller meets the stable work/timer lifetime contract for retries.
    unsafe {
        loop {
            let ret = try_to_grab_pending(work, cflags, irq_flags);
            if ret >= 0 {
                return ret != 0;
            }
            lupos_wq_cpu_relax();
        }
    }
}

/// Publish work data, append the item, then take its PWQ reference (C 2266–2278).
///
/// # Safety
/// `work` is live, unlinked and PENDING-owned. pwq is live with positive refcount,
/// its pool lock is held with IRQs disabled, and head is the chosen native list.
pub(super) unsafe fn insert_work(
    pwq: *mut pool_workqueue,
    work: *mut work_struct,
    head: *mut list_head,
    extra_flags: c_uint,
) {
    // SAFETY: Caller owns work/PWQ list state and supplies required instrumentation
    // context; no reference to work survives a future execution callback.
    unsafe {
        debug_work_activate(work);
        lupos_wq_kasan_record_aux_stack(work);
        set_work_pwq(work, pwq, extra_flags as c_ulong);
        lupos_wq_list_add_tail(addr_of_mut!((*work).entry), head);
        get_pwq(pwq);
    }
}

/// Whether current is executing a work on the same queue (C 2284–2294).
///
/// # Safety
/// wq is live; native current_wq_worker semantics apply to the calling context.
pub(super) unsafe fn is_chained_work(wq: *mut workqueue_struct) -> bool {
    // SAFETY: Only the current worker is observed; its current PWQ is self-owned
    // in an execution context, so no pool lock is needed for this identity test.
    unsafe {
        let worker = lupos_wq_current_wq_worker();
        !worker.is_null() && (*(*worker).current_pwq).wq == wq
    }
}

/// Prefer the supplied CPU or rotate among online allowed unbound CPUs (C 2301–2319).
///
/// # Safety
/// Caller prevents migration, as the native queue path does with IRQs disabled;
/// cpu is a valid CPU and native unbound-mask publication rules hold.
pub(super) unsafe fn wq_select_unbound_cpu(cpu: c_int) -> c_int {
    // SAFETY: IRQ-off caller owns its per-CPU round-robin cursor; native mask
    // primitives and configured accessors retain current CPU/mask semantics.
    unsafe {
        if !lupos_wq_debug_force_rr_cpu() {
            if lupos_wq_cpumask_test_cpu(cpu, lupos_wq_unbound_cpumask()) {
                return cpu;
            }
        } else {
            lupos_wq_warn_unbound_cpu_force_rr();
        }
        let last = lupos_wq_this_cpu_rr_read();
        let new_cpu = lupos_wq_cpumask_next_and_wrap(
            last,
            lupos_wq_unbound_cpumask(),
            lupos_wq_cpu_online_mask(),
        ) as c_int;
        if new_cpu as c_uint >= lupos_wq_nr_cpu_ids() {
            return cpu;
        }
        lupos_wq_this_cpu_rr_write(new_cpu);
        new_cpu
    }
}

/// Queue a PENDING-owned work with non-reentrance and deferred wake ordering (C 2321–2460).
///
/// # Safety
/// `wq` and unlinked `work` are live; caller owns PENDING with local IRQs off.
/// cpu is WORK_CPU_UNBOUND or a CPU valid for the native queue API. Workqueue
/// lifetime, CPU hotplug and caller publication constraints are those of C API.
pub(super) unsafe fn __queue_work(
    mut cpu: c_int,
    wq: *mut workqueue_struct,
    work: *mut work_struct,
) {
    // SAFETY: PENDING and IRQ-off protect submission; explicit RCU spans PWQ
    // selection, pool unlock AND deferred task wake. Pool locks serialize all
    // data/list/refcount changes, with no Rust references across lock handoffs.
    unsafe {
        let mut wake_task = null_mut();
        let req_cpu = cpu as c_uint;
        if (*wq).flags & __WQ_DEPRECATED != 0 {
            lupos_wq_warn_queue_work_deprecated((*work).func);
        }
        lupos_wq_assert_queue_work_irqs_disabled();
        if (*wq).flags & (__WQ_DESTROYING | __WQ_DRAINING) != 0
            && lupos_wq_warn_queue_work_draining(!is_chained_work(wq), wq, work)
        {
            let mut offqd = MaybeUninit::<work_offq_data>::uninit();
            let data = offqd.as_mut_ptr();
            work_offqd_unpack(data, *work_data_bits(work));
            set_work_pool_and_clear_pending(
                work,
                (*data).pool_id as c_int,
                work_offqd_pack_flags(data),
            );
            return;
        }
        lupos_wq_rcu_read_lock();
        loop {
            if req_cpu == WORK_CPU_UNBOUND as c_uint {
                cpu = if (*wq).flags & WQ_UNBOUND != 0 {
                    wq_select_unbound_cpu(lupos_wq_raw_smp_processor_id() as c_int)
                } else {
                    lupos_wq_raw_smp_processor_id() as c_int
                };
            }
            let mut pwq = lupos_wq_cpu_pwq_dereference(wq, cpu);
            let mut pool = (*pwq).pool;
            let last_pool = get_work_pool(work);
            if !last_pool.is_null() && last_pool != pool && (*wq).flags & __WQ_ORDERED == 0 {
                lupos_wq_pool_lock(last_pool);
                let worker = find_worker_executing_work(last_pool, work);
                if !worker.is_null() && (*(*worker).current_pwq).wq == wq {
                    pwq = (*worker).current_pwq;
                    pool = (*pwq).pool;
                    lupos_wq_warn_queue_work_wrong_pool(pool != last_pool);
                } else {
                    lupos_wq_pool_unlock(last_pool);
                    lupos_wq_pool_lock(pool);
                }
            } else {
                lupos_wq_pool_lock(pool);
            }
            if (*pwq).refcnt == 0 {
                if (*wq).flags & WQ_UNBOUND != 0 {
                    lupos_wq_pool_unlock(pool);
                    lupos_wq_cpu_relax();
                    continue;
                }
                lupos_wq_warn_queue_work_percpu_zero_ref(wq, cpu);
            }
            lupos_wq_trace_workqueue_queue_work(req_cpu as c_int, pwq, work);
            if !lupos_wq_warn_queue_work_linked(!lupos_wq_list_empty(addr_of!((*work).entry))) {
                let color = (*pwq).work_color;
                let count = addr_of_mut!((*pwq).nr_in_flight[color as usize]);
                *count = (*count).wrapping_add(1);
                let mut work_flags = work_color_to_flags(color);
                if lupos_wq_list_empty(addr_of!((*pwq).inactive_works))
                    && pwq_tryinc_nr_active(pwq, false)
                {
                    if lupos_wq_list_empty(addr_of!((*pool).worklist)) {
                        (*pool).last_progress_ts = lupos_wq_read_jiffies();
                    }
                    lupos_wq_trace_workqueue_activate_work(work);
                    insert_work(pwq, work, addr_of_mut!((*pool).worklist), work_flags);
                    kick_pool_pick(pool, &mut wake_task);
                } else {
                    work_flags |= WORK_STRUCT_INACTIVE;
                    insert_work(pwq, work, addr_of_mut!((*pwq).inactive_works), work_flags);
                }
            }
            lupos_wq_pool_unlock(pool);
            if !wake_task.is_null() {
                wake_up_process(wake_task);
            }
            lupos_wq_rcu_read_unlock();
            return;
        }
    }
}

/// Release PENDING when an off-queue work is disabled (C 2462–2475).
///
/// # Safety
/// `work` is live and the caller owns PENDING. For ordinary cancelable work,
/// local IRQs remain disabled until ownership is transferred or released.
/// The native queue_rcu_work path may call with IRQs enabled: its inner work
/// must not be independently canceled or disabled, as required by that API.
pub(super) unsafe fn clear_pending_if_disabled(work: *mut work_struct) -> bool {
    // SAFETY: PENDING ownership makes encoded off-queue data stable. Unpack
    // initializes every field before pool ID and disable flags are read.
    unsafe {
        let data = *work_data_bits(work);
        if data & (WORK_STRUCT_PWQ as c_ulong) != 0 || data & (WORK_OFFQ_DISABLE_MASK as c_ulong) == 0 {
            return false;
        }
        let mut offqd = MaybeUninit::<work_offq_data>::uninit();
        let unpacked = offqd.as_mut_ptr();
        work_offqd_unpack(unpacked, data);
        set_work_pool_and_clear_pending(
            work,
            (*unpacked).pool_id as c_int,
            work_offqd_pack_flags(unpacked),
        );
        true
    }
}

/// Attempt immediate enqueue on a selected CPU (C 2491–2507).
///
/// # Safety
/// Caller meets the native queue_work_on work/queue lifetime and CPU conditions.
/// The callback may start and free the item immediately after IRQ restoration.
#[no_mangle]
pub unsafe extern "C" fn queue_work_on(
    cpu: c_int,
    wq: *mut workqueue_struct,
    work: *mut work_struct,
) -> bool {
    // SAFETY: Native atomic PENDING claim serializes competing submissions;
    // disabled-work cleanup and queue publication run in one IRQ-off interval.
    unsafe {
        let flags = lupos_wq_local_irq_save();
        let mut ret = false;
        if !lupos_wq_test_and_set_bit(WORK_STRUCT_PENDING_BIT as c_long, work_data_bits(work))
            && !clear_pending_if_disabled(work)
        {
            __queue_work(cpu, wq, work);
            ret = true;
        }
        lupos_wq_local_irq_restore(flags);
        ret
    }
}

/// Choose current or first online CPU on a requested node (C 2519–2537).
///
/// # Safety
/// Caller disables migration and satisfies native topology read requirements.
pub(super) unsafe fn select_numa_node_cpu(node: c_int) -> c_int {
    // SAFETY: Native mask accessors preserve topology representation; CPU
    // selection is advisory and uses the native WORK_CPU_UNBOUND fallback.
    unsafe {
        if node < 0 || node >= MAX_NUMNODES as c_int || !lupos_wq_node_online(node) {
            return WORK_CPU_UNBOUND as c_int;
        }
        let cpu = lupos_wq_raw_smp_processor_id() as c_int;
        if node == lupos_wq_cpu_to_node(cpu) {
            return cpu;
        }
        let cpu = lupos_wq_cpumask_any_and(
            lupos_wq_cpumask_of_node(node),
            lupos_wq_cpu_online_mask(),
        ) as c_int;
        if (cpu as c_uint) < lupos_wq_nr_cpu_ids() {
            cpu
        } else {
            WORK_CPU_UNBOUND as c_int
        }
    }
}

/// Attempt immediate enqueue on a CPU near a requested NUMA node (C 2559–2588).
///
/// # Safety
/// Caller meets native queue_work_node work/queue lifetime requirements.
#[no_mangle]
pub unsafe extern "C" fn queue_work_node(
    node: c_int,
    wq: *mut workqueue_struct,
    work: *mut work_struct,
) -> bool {
    // SAFETY: Atomic PENDING and local IRQ disable protect node selection and
    // queue insertion; the native warning does not turn bound usage into success.
    unsafe {
        lupos_wq_warn_queue_work_node_bound((*wq).flags & WQ_UNBOUND == 0);
        let flags = lupos_wq_local_irq_save();
        let mut ret = false;
        if !lupos_wq_test_and_set_bit(WORK_STRUCT_PENDING_BIT as c_long, work_data_bits(work))
            && !clear_pending_if_disabled(work)
        {
            let cpu = select_numa_node_cpu(node);
            __queue_work(cpu, wq, work);
            ret = true;
        }
        lupos_wq_local_irq_restore(flags);
        ret
    }
}

/// Queue the inner work when its IRQ-safe delayed timer expires (C 2591–2597).
///
/// # Safety
/// t is the timer embedded in a live delayed_work initialized for this callback;
/// native IRQ-safe timer invocation guarantees local IRQs are already disabled.
#[no_mangle]
pub unsafe extern "C" fn delayed_work_timer_fn(t: *mut timer_list) {
    // SAFETY: Native timer container conversion uses configured layout; timer
    // ownership retains the pending work and its queue until submission.
    unsafe {
        let dwork = lupos_wq_delayed_work_from_timer(t);
        __queue_work((*dwork).cpu, (*dwork).wq, addr_of_mut!((*dwork).work));
    }
}

/// Immediately enqueue zero delay, otherwise arm the native IRQ-safe timer (C 2600–2638).
///
/// # Safety
/// dwork/wq are live; caller owns inner work's PENDING with local IRQs disabled.
/// Timer uses delayed_work_timer_fn and is not pending; work is unlinked. Native
/// requested-CPU lifetime conditions must hold for a nonzero delay.
pub(super) unsafe fn __queue_delayed_work(
    mut cpu: c_int,
    wq: *mut workqueue_struct,
    dwork: *mut delayed_work,
    delay: c_ulong,
) {
    // SAFETY: Caller owns both timer and work until arming. The timer target can
    // differ from saved dwork.cpu under housekeeping, exactly as the native API.
    unsafe {
        let timer = addr_of_mut!((*dwork).timer);
        let work = addr_of_mut!((*dwork).work);
        lupos_wq_warn_queue_delayed_work_callback(
            !lupos_wq_timer_func_eq((*timer).function, Some(delayed_work_timer_fn)),
        );
        lupos_wq_warn_queue_delayed_work_timer_pending(lupos_wq_timer_pending(timer));
        lupos_wq_warn_queue_delayed_work_linked(!lupos_wq_list_empty(addr_of!((*work).entry)));
        if delay == 0 {
            __queue_work(cpu, wq, work);
            return;
        }
        lupos_wq_warn_queue_delayed_work_cpu_offline(
            (cpu as c_uint) != WORK_CPU_UNBOUND as c_uint && !lupos_wq_cpu_online(cpu),
        );
        (*dwork).wq = wq;
        (*dwork).cpu = cpu;
        (*timer).expires = lupos_wq_read_jiffies().wrapping_add(delay);
        if lupos_wq_housekeeping_enabled(HK_TYPE_TIMER) {
            cpu = lupos_wq_smp_processor_id() as c_int;
            if !lupos_wq_housekeeping_test_cpu(cpu, HK_TYPE_TIMER) {
                cpu = lupos_wq_housekeeping_any_cpu(HK_TYPE_TIMER);
            }
            add_timer_on(timer, cpu);
        } else if (cpu as c_uint) == WORK_CPU_UNBOUND as c_uint {
            add_timer_global(timer);
        } else {
            add_timer_on(timer, cpu);
        }
    }
}

/// Attempt enqueue after a delay on a requested CPU (C 2657–2675).
///
/// # Safety
/// Caller meets native delayed-work lifetime/initialization requirements. For
/// nonzero delay a specified CPU is online and cannot disappear before expiry.
#[no_mangle]
pub unsafe extern "C" fn queue_delayed_work_on(
    cpu: c_int,
    wq: *mut workqueue_struct,
    dwork: *mut delayed_work,
    delay: c_ulong,
) -> bool {
    // SAFETY: Atomic PENDING ownership and IRQ disable cover disabled-work
    // cleanup, immediate enqueue or timer publication without an off-queue gap.
    unsafe {
        let work = addr_of_mut!((*dwork).work);
        let flags = lupos_wq_local_irq_save();
        let mut ret = false;
        if !lupos_wq_test_and_set_bit(WORK_STRUCT_PENDING_BIT as c_long, work_data_bits(work))
            && !clear_pending_if_disabled(work)
        {
            __queue_delayed_work(cpu, wq, dwork, delay);
            ret = true;
        }
        lupos_wq_local_irq_restore(flags);
        ret
    }
}

/// Modify or queue delayed work, reporting whether it had been pending (C 2696–2709).
///
/// # Safety
/// Caller meets native delayed-work lifetime, IRQ-safe timer and CPU contracts.
#[no_mangle]
pub unsafe extern "C" fn mod_delayed_work_on(
    cpu: c_int,
    wq: *mut workqueue_struct,
    dwork: *mut delayed_work,
    delay: c_ulong,
) -> bool {
    // SAFETY: Grab owns PENDING and returns IRQ-off state. The same saved state
    // is restored after disabled cleanup or re-arming; ret describes prior state.
    unsafe {
        let mut flags = 0;
        let work = addr_of_mut!((*dwork).work);
        let ret = work_grab_pending(work, WORK_CANCEL_DELAYED, &mut flags);
        if !clear_pending_if_disabled(work) {
            __queue_delayed_work(cpu, wq, dwork, delay);
        }
        lupos_wq_local_irq_restore(flags);
        ret
    }
}

/// Transfer an RCU work to its queue after grace-period callback (C 2712–2720).
///
/// # Safety
/// rcu is the embedded head of a live rcu_work registered with call_rcu_hurry;
/// its work remains PENDING and native callback context permits IRQ enable/disable.
pub(super) unsafe extern "C" fn rcu_work_rcufn(rcu: *mut rcu_head) {
    // SAFETY: The callback registration retains the enclosing rcu_work. Native
    // local_irq_disable/enable, not save/restore, matches this callback contract.
    unsafe {
        let rwork = lupos_wq_rcu_work_from_rcu(rcu);
        lupos_wq_local_irq_disable();
        __queue_work(WORK_CPU_UNBOUND as c_int, (*rwork).wq, addr_of_mut!((*rwork).work));
        lupos_wq_local_irq_enable();
    }
}

/// Attempt a fresh RCU enqueue, guaranteeing a grace period only on true (C 2732–2748).
///
/// # Safety
/// wq/rwork are live and initialized; caller retains their native RCU-work
/// lifetime. Inner work must never be independently canceled or disabled.
#[no_mangle]
pub unsafe extern "C" fn queue_rcu_work(
    wq: *mut workqueue_struct,
    rwork: *mut rcu_work,
) -> bool {
    // SAFETY: Atomic ownership serializes publication. Unlike ordinary work,
    // the inner work cannot be independently canceled or disabled, so the
    // native path needs no IRQ-off interval. Disabled-inner-work cleanup is
    // deliberately warned and prevents registering a new callback.
    unsafe {
        let work = addr_of_mut!((*rwork).work);
        if !lupos_wq_test_and_set_bit(WORK_STRUCT_PENDING_BIT as c_long, work_data_bits(work))
            && !lupos_wq_warn_queue_rcu_work_disabled(clear_pending_if_disabled(work))
        {
            (*rwork).wq = wq;
            call_rcu_hurry(addr_of_mut!((*rwork).rcu), Some(rcu_work_rcufn));
            return true;
        }
        false
    }
}

/// Restore scheduler concurrency accounting when a worker runs again (C 1450–1475).
///
/// # Safety
/// task is the current live kworker in native scheduler wake/run context; native
/// preemption exclusion and worker field access rules are in force.
#[no_mangle]
pub unsafe extern "C" fn wq_worker_running(task: *mut task_struct) {
    // SAFETY: kthread_data denotes this current worker. Preemption is disabled
    // across NOT_RUNNING test and increment to exclude local hotplug reset.
    unsafe {
        let worker = kthread_data(task).cast::<worker>();
        if lupos_wq_read_once_int(addr_of!((*worker).sleeping)) == 0 {
            return;
        }
        lupos_wq_preempt_disable();
        if (*worker).flags & WORKER_NOT_RUNNING == 0 {
            let pool = (*worker).pool;
            (*pool).nr_running = (*pool).nr_running.wrapping_add(1);
        }
        lupos_wq_preempt_enable();
        (*worker).current_at = lupos_wq_read_task_sum_exec_runtime((*worker).task);
        lupos_wq_write_once_int(addr_of_mut!((*worker).sleeping), 0);
    }
}

/// Retire a sleeping worker's concurrency count, rechecking after locking (C 1484–1521).
///
/// # Safety
/// task is the current live kworker entering schedule; the native scheduler
/// context permits raw_spin_lock_irq/unlock_irq. Rescuers may have partial state.
#[no_mangle]
pub unsafe extern "C" fn wq_worker_sleeping(task: *mut task_struct) {
    // SAFETY: Native early NOT_RUNNING check precedes all pool/current-PWQ
    // access for rescuers. Lock then rechecks against hotplug preemption.
    unsafe {
        let worker = kthread_data(task).cast::<worker>();
        if (*worker).flags & WORKER_NOT_RUNNING != 0 {
            return;
        }
        let pool = (*worker).pool;
        if lupos_wq_read_once_int(addr_of!((*worker).sleeping)) != 0 {
            return;
        }
        lupos_wq_write_once_int(addr_of_mut!((*worker).sleeping), 1);
        lupos_wq_pool_lock_irq(pool);
        if (*worker).flags & WORKER_NOT_RUNNING != 0 {
            lupos_wq_pool_unlock_irq(pool);
            return;
        }
        (*pool).nr_running = (*pool).nr_running.wrapping_sub(1);
        if kick_pool(pool) {
            let stat = addr_of_mut!((*(*worker).current_pwq).stats[PWQ_STAT_CM_WAKEUP as usize]);
            *stat = (*stat).wrapping_add(1);
        }
        lupos_wq_pool_unlock_irq(pool);
    }
}

/// Account worker CPU time and detect a concurrency-managed CPU hog (C 1530–1575).
///
/// # Safety
/// task is current kworker in scheduler tick IRQ context. Worker K-rule fields
/// are stable here; the advisory PWQ CPU-time statistic is intentionally racy.
#[no_mangle]
pub unsafe extern "C" fn wq_worker_tick(task: *mut task_struct) {
    // SAFETY: IRQ context stabilizes this worker's K fields. The native data_race
    // primitive preserves advisory-stat access; accounting changes take pool lock.
    unsafe {
        let worker = kthread_data(task).cast::<worker>();
        let pwq = (*worker).current_pwq;
        let pool = (*worker).pool;
        if pwq.is_null() {
            return;
        }
        lupos_wq_pwq_cpu_time_add_data_race(pwq, TICK_USEC as u64);
        if lupos_wq_cpu_intensive_thresh_us() == 0 {
            return;
        }
        if (*worker).flags & WORKER_NOT_RUNNING != 0
            || lupos_wq_read_once_int(addr_of!((*worker).sleeping)) != 0
            || lupos_wq_read_task_sum_exec_runtime((*worker).task)
                .wrapping_sub((*worker).current_at)
                < lupos_wq_cpu_intensive_thresh_us().wrapping_mul(NSEC_PER_USEC as c_ulong) as u64
        {
            return;
        }
        lupos_wq_pool_lock(pool);
        worker_set_flags(worker, WORKER_CPU_INTENSIVE);
        wq_cpu_intensive_report((*worker).current_func);
        let intensive = addr_of_mut!((*pwq).stats[PWQ_STAT_CPU_INTENSIVE as usize]);
        *intensive = (*intensive).wrapping_add(1);
        if kick_pool(pool) {
            let wakeups = addr_of_mut!((*pwq).stats[PWQ_STAT_CM_WAKEUP as usize]);
            *wakeups = (*wakeups).wrapping_add(1);
        }
        lupos_wq_pool_unlock(pool);
    }
}

/// Read the scheduler-stable last callback without workqueue locking (C 1601–1606).
///
/// # Safety
/// task is a kworker stopped from processing work by native scheduler queueing
/// or dequeueing context, with rq lock held and IRQs disabled. No pool lock added.
#[no_mangle]
pub unsafe extern "C" fn wq_worker_last_func(task: *mut task_struct) -> work_func_t {
    // SAFETY: Native rq-locked scheduler context stabilizes the worker identity.
    unsafe { (*kthread_data(task).cast::<worker>()).last_func }
}
