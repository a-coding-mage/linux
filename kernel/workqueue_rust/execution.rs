// SPDX-License-Identifier: GPL-2.0-only
//! Worker lifetime, management, and execution from native workqueue.c.
//!
//! Authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b, lines 2751–3882.
//! This is a source-only overlay. Configured native types, primitive leaves,
//! callback ABI/KCFI, symbol metadata, and integration remain unadmitted.

use super::common::*;
use super::dispatch::{
    assign_work,
    keep_working,
    kick_pool_pick,
    kick_pool,
    put_pwq_unlocked,
    may_start_working,
    need_more_worker,
    pwq_dec_nr_in_flight,
    worker_clr_flags,
    worker_leave_idle,
    worker_set_flags,
    get_pwq,
    need_to_create_worker,
    queue_work_on,
    too_many_workers,
    is_percpu_pool,
    worker_enter_idle, //
};
use super::diagnostics::debug_work_deactivate;
use core::mem::{
    size_of,
    MaybeUninit, //
};
use core::ptr::{
    addr_of,
    addr_of_mut,
    null_mut, //
};
use kernel::ffi::{
    c_char,
    c_int,
    c_long,
    c_uint,
    c_ulong,
    c_void, //
};

/// Allocate a detached worker in PREP, native lines 2751–2764.
///
/// # Safety
/// The caller must be permitted to sleep and perform GFP_KERNEL allocation.
pub(super) unsafe fn alloc_worker(node: c_int) -> *mut worker {
    // SAFETY: The native allocation leaf returns suitably aligned, zeroed
    // configured worker storage. No reference is formed to its union members.
    unsafe {
        let worker = lupos_wq_kzalloc_node(size_of::<worker>(), GFP_KERNEL, node).cast::<worker>();
        if !worker.is_null() {
            lupos_wq_init_list_head(lupos_wq_worker_entry(worker));
            lupos_wq_init_list_head(addr_of_mut!((*worker).scheduled));
            lupos_wq_init_list_head(addr_of_mut!((*worker).node));
            (*worker).flags = WORKER_PREP;
        }
        worker
    }
}

/// Select the native pool affinity mask, lines 2766–2772.
///
/// # Safety
/// The pool and its immutable attributes must remain alive through use of the result.
pub(super) unsafe fn pool_allowed_cpus(pool: *mut worker_pool) -> *mut cpumask {
    // SAFETY: The caller pins the native pool/attrs; accessors preserve the
    // configured cpumask_var_t representation rather than projecting a layout.
    unsafe {
        if !is_percpu_pool(pool) && (*(*pool).attrs).affn_strict {
            lupos_wq_attrs_pod_cpumask((*pool).attrs)
        } else {
            lupos_wq_attrs_cpumask((*pool).attrs)
        }
    }
}

/// Attach a worker while serializing CPU disassociation, lines 2783–2807.
///
/// # Safety
/// The caller owns an unattached worker and pins the pool; this may sleep.
pub(super) unsafe fn worker_attach_to_pool(worker: *mut worker, pool: *mut worker_pool) {
    // SAFETY: The attach mutex protects membership and stabilizes DISASSOCIATED.
    // The native BH branch deliberately never dereferences its null task.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
        if (*pool).flags & POOL_DISASSOCIATED != 0 {
            (*worker).flags |= WORKER_UNBOUND;
        } else {
            lupos_wq_warn_worker_attach_to_pool_bh((*pool).flags & POOL_BH != 0);
            kthread_set_per_cpu((*worker).task, (*pool).cpu);
        }
        if !(*worker).rescue_wq.is_null() {
            set_cpus_allowed_ptr((*worker).task, pool_allowed_cpus(pool));
        }
        lupos_wq_list_add_tail(addr_of_mut!((*worker).node), addr_of_mut!((*pool).workers));
        (*worker).pool = pool;
        mutex_unlock(lupos_wq_pool_attach_mutex());
    }
}

/// Remove per-CPU binding, native lines 2809–2818.
///
/// # Safety
/// The worker/task must be live and wq_pool_attach_mutex must be held.
pub(super) unsafe fn unbind_worker(worker: *mut worker) {
    // SAFETY: The attach mutex excludes hotplug binding updates. The masks and
    // task remain alive; affinity failures retain the native warning behavior.
    unsafe {
        lupos_wq_assert_unbind_worker_attach_mutex();
        kthread_set_per_cpu((*worker).task, -1);
        if lupos_wq_cpumask_intersects(lupos_wq_unbound_cpumask(), lupos_wq_cpu_active_mask()) {
            let result = set_cpus_allowed_ptr((*worker).task, lupos_wq_unbound_cpumask());
            lupos_wq_warn_unbind_worker_unbound_affinity(result < 0);
        } else {
            let result = set_cpus_allowed_ptr((*worker).task, lupos_wq_cpu_possible_mask());
            lupos_wq_warn_unbind_worker_possible_affinity(result < 0);
        }
    }
}

/// Unbind and remove pool membership, native lines 2821–2827.
///
/// # Safety
/// The attached worker and its task are live; wq_pool_attach_mutex is held.
unsafe fn detach_worker(worker: *mut worker) {
    // SAFETY: The caller's attach mutex protects the native membership link.
    unsafe {
        lupos_wq_assert_detach_worker_attach_mutex();
        unbind_worker(worker);
        lupos_wq_list_del(addr_of_mut!((*worker).node));
    }
}

/// Detach a rescuer and discard association flags, lines 2837–2851.
///
/// # Safety
/// The caller owns the attached worker; this may sleep. The pool may not be
/// accessed after this function without an independent lifetime guarantee.
pub(super) unsafe fn worker_detach_from_pool(worker: *mut worker) {
    // SAFETY: Pool use ends before dropping the attach mutex. After detach only
    // the caller-owned worker is accessed; association flags are now private.
    unsafe {
        let pool = (*worker).pool;
        lupos_wq_warn_worker_detach_from_pool_bh((*pool).flags & POOL_BH != 0);
        lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
        detach_worker(worker);
        (*worker).pool = null_mut();
        mutex_unlock(lupos_wq_pool_attach_mutex());
        (*worker).flags &= !(WORKER_UNBOUND | WORKER_REBOUND);
    }
}

/// Format the native identity without transferring worker ownership, 2853–2871.
///
/// # Safety
/// The writable buffer has `size` bytes; worker and any nonnull pool/rescue WQ
/// remain live. Native name fields contain their initialized C strings.
pub(super) unsafe fn format_worker_id(
    buf: *mut c_char,
    size: usize,
    worker: *mut worker,
    pool: *mut worker_pool,
) -> c_int {
    // SAFETY: All native varargs have their promoted C types and formats are
    // fixed NUL-terminated strings. The caller guarantees buffer/object lifetime.
    unsafe {
        if !(*worker).rescue_wq.is_null() {
            scnprintf(buf, size, c"kworker/R-%s".as_ptr().cast::<c_char>(), addr_of!((*(*worker).rescue_wq).name).cast::<c_char>())
        } else if !pool.is_null() {
            if (*pool).cpu >= 0 {
                scnprintf(
                    buf,
                    size,
                    c"kworker/%d:%d%s".as_ptr().cast::<c_char>(),
                    (*pool).cpu,
                    (*worker).id,
                    if (*(*pool).attrs).nice < 0 { c"H".as_ptr().cast::<c_char>() } else { c"".as_ptr().cast::<c_char>() },
                )
            } else {
                scnprintf(buf, size, c"kworker/u%d:%d".as_ptr().cast::<c_char>(), (*pool).id, (*worker).id)
            }
        } else {
            scnprintf(buf, size, c"kworker/dying".as_ptr().cast::<c_char>())
        }
    }
}

/// Allocate, attach, and start a pool worker, native lines 2885–2952.
///
/// # Safety
/// The caller pins the pool across allocation/attachment and may sleep.
pub(super) unsafe fn create_worker(pool: *mut worker_pool) -> *mut worker {
    // SAFETY: Worker storage is private until attachment. Each failure path
    // releases the ID before freeing storage, including native ERR_PTR task
    // failures. Pool statistics/idle state are changed under its IRQ lock.
    unsafe {
        let id = lupos_wq_ida_alloc(addr_of_mut!((*pool).worker_ida), GFP_KERNEL);
        if id < 0 {
            lupos_wq_log_create_worker_id_failed(id);
            return null_mut();
        }
        let worker = alloc_worker((*pool).node);
        if worker.is_null() {
            lupos_wq_log_create_worker_alloc_failed();
            ida_free(addr_of_mut!((*pool).worker_ida), id as c_uint);
            kfree(worker.cast());
            return null_mut();
        }
        (*worker).id = id;
        if (*pool).flags & POOL_BH == 0 {
            let mut id_buf = [0 as c_char; WORKER_ID_LEN as usize];
            format_worker_id(id_buf.as_mut_ptr(), id_buf.len(), worker, pool);
            (*worker).task = kthread_create_on_node(
                Some(worker_thread),
                worker.cast::<c_void>(),
                (*pool).node,
                c"%s".as_ptr().cast::<c_char>(),
                id_buf.as_ptr(),
            );
            if lupos_wq_is_err((*worker).task.cast()) {
                if lupos_wq_ptr_err((*worker).task.cast()) == -(EINTR as c_long) {
                    lupos_wq_log_create_worker_interrupted(id_buf.as_ptr());
                } else {
                    lupos_wq_log_create_worker_thread_failed((*worker).task);
                }
                ida_free(addr_of_mut!((*pool).worker_ida), id as c_uint);
                kfree(worker.cast());
                return null_mut();
            }
            set_user_nice((*worker).task, (*(*pool).attrs).nice as c_long);
            kthread_bind_mask((*worker).task, pool_allowed_cpus(pool));
        }
        worker_attach_to_pool(worker, pool);
        lupos_wq_pool_lock_irq(pool);
        (*(*worker).pool).nr_workers = (*(*worker).pool).nr_workers.wrapping_add(1);
        worker_enter_idle(worker);
        if !(*worker).task.is_null() {
            wake_up_process((*worker).task);
        }
        lupos_wq_pool_unlock_irq(pool);
        worker
    }
}

/// Detach all culled workers before allowing self-destruction, 2954–2960.
///
/// # Safety
/// The caller owns the live cull list and holds wq_pool_attach_mutex.
pub(super) unsafe fn detach_dying_workers(cull_list: *mut list_head) {
    // SAFETY: Detachment removes `node`, not the cull-list `entry`, so advancing
    // through the still-owned list after each detach is valid.
    unsafe {
        let mut entry = (*cull_list).next;
        while entry != cull_list {
            let worker = lupos_wq_worker_from_entry(entry);
            detach_worker(worker);
            entry = (*entry).next;
        }
    }
}

/// Stop detached dying tasks and release their storage, lines 2962–2971.
///
/// # Safety
/// The caller owns the cull list; each task has the extra reference acquired
/// by set_worker_dying, and has already been detached under the attach mutex.
pub(super) unsafe fn reap_dying_workers(cull_list: *mut list_head) {
    // SAFETY: Cache the successor before stop/free; after kthread_stop_put the
    // thread cannot use worker storage and its extra task reference is released.
    unsafe {
        let mut entry = (*cull_list).next;
        while entry != cull_list {
            let next = (*entry).next;
            let worker = lupos_wq_worker_from_entry(entry);
            lupos_wq_list_del_init(entry);
            kthread_stop_put((*worker).task);
            kfree(worker.cast());
            entry = next;
        }
    }
}

/// Mark an idle worker dying and transfer its lifetime to a cull list, 2984–3013.
///
/// # Safety
/// Both the pool IRQ lock and attach mutex are held; worker, task and list are
/// live, and the worker is idle with no current or scheduled work.
pub(super) unsafe fn set_worker_dying(worker: *mut worker, list: *mut list_head) {
    // SAFETY: Both locks serialize the union entry and self-destruction. The
    // watchdog pointer is cleared before the worker can become unobservable.
    unsafe {
        let pool = (*worker).pool;
        lupos_wq_assert_set_worker_dying_pool_lock(pool);
        lupos_wq_assert_set_worker_dying_attach_mutex();
        if lupos_wq_warn_set_worker_dying_current(!(*worker).current_work.is_null())
            || lupos_wq_warn_set_worker_dying_scheduled(!lupos_wq_list_empty(addr_of!((*worker).scheduled)))
            || lupos_wq_warn_set_worker_dying_not_idle((*worker).flags & WORKER_IDLE == 0)
        {
            return;
        }
        (*pool).nr_workers = (*pool).nr_workers.wrapping_sub(1);
        (*pool).nr_idle = (*pool).nr_idle.wrapping_sub(1);
        if (*pool).last_woken_worker == worker {
            (*pool).last_woken_worker = null_mut();
        }
        (*worker).flags |= WORKER_DIE;
        lupos_wq_list_move(lupos_wq_worker_entry(worker), list);
        lupos_wq_get_task_struct((*worker).task);
    }
}

/// Schedule sleepable culling after the oldest idle worker expires, 3025–3051.
///
/// # Safety
/// Invoked for the live pool's native idle timer with native timer lifetime.
pub(super) unsafe extern "C" fn idle_worker_timeout(timer: *mut timer_list) {
    // SAFETY: The callback's timer is embedded in a live native pool; list access
    // occurs under the pool IRQ lock and no worker pointer escapes that lock.
    unsafe {
        let pool = kernel::container_of!(timer, worker_pool, idle_timer);
        let mut do_cull = false;
        if lupos_wq_work_pending(addr_of!((*pool).idle_cull_work)) {
            return;
        }
        lupos_wq_pool_lock_irq(pool);
        if too_many_workers(pool) {
            let worker = lupos_wq_worker_from_entry((*pool).idle_list.prev);
            let expires = (*worker).last_active.wrapping_add(IDLE_WORKER_TIMEOUT as c_ulong);
            do_cull = !lupos_wq_time_before(lupos_wq_read_jiffies(), expires);
            if !do_cull {
                mod_timer(addr_of_mut!((*pool).idle_timer), expires);
            }
        }
        lupos_wq_pool_unlock_irq(pool);
        if do_cull {
            queue_work_on(WORK_CPU_UNBOUND as c_int, system_dfl_wq, addr_of_mut!((*pool).idle_cull_work));
        }
    }
}

/// Detach expired workers under attach exclusion and then reap, 3064–3098.
///
/// # Safety
/// Invoked for the live pool's idle_cull_work in sleepable workqueue context.
pub(super) unsafe extern "C" fn idle_cull_fn(work: *mut work_struct) {
    // SAFETY: This work pins its containing pool for the callback. The local
    // native list is initialized at its final address, never copied/moved, and
    // is drained before returning. Attach mutex precedes pool IRQ locking.
    unsafe {
        let pool = kernel::container_of!(work, worker_pool, idle_cull_work);
        let mut cull_storage = MaybeUninit::<list_head>::uninit();
        let cull_list = cull_storage.as_mut_ptr();
        lupos_wq_init_list_head(cull_list);
        lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
        lupos_wq_pool_lock_irq(pool);
        while too_many_workers(pool) {
            let worker = lupos_wq_worker_from_entry((*pool).idle_list.prev);
            let expires = (*worker).last_active.wrapping_add(IDLE_WORKER_TIMEOUT as c_ulong);
            if lupos_wq_time_before(lupos_wq_read_jiffies(), expires) {
                mod_timer(addr_of_mut!((*pool).idle_timer), expires);
                break;
            }
            set_worker_dying(worker, cull_list);
        }
        lupos_wq_pool_unlock_irq(pool);
        detach_dying_workers(cull_list);
        mutex_unlock(lupos_wq_pool_attach_mutex());
        reap_dying_workers(cull_list);
    }
}

/// Pin a PWQ exactly once while requesting its rescuer, 3100–3121.
///
/// # Safety
/// The caller holds the pool lock and wq_mayday_lock; PWQ and WQ are live.
pub(super) unsafe fn send_mayday(pwq: *mut pool_workqueue) {
    // SAFETY: Mayday membership and wakeup are serialized by wq_mayday_lock;
    // the pool lock protects the reference count and mayday statistic.
    unsafe {
        let wq = (*pwq).wq;
        lupos_wq_assert_send_mayday_mayday_lock();
        if (*wq).rescuer.is_null() {
            return;
        }
        if lupos_wq_list_empty(addr_of!((*pwq).mayday_node)) {
            get_pwq(pwq);
            lupos_wq_list_add_tail(addr_of_mut!((*pwq).mayday_node), addr_of_mut!((*wq).maydays));
            wake_up_process((*(*wq).rescuer).task);
            (*pwq).stats[PWQ_STAT_MAYDAY as usize] = (*pwq).stats[PWQ_STAT_MAYDAY as usize].wrapping_add(1);
        }
    }
}

/// Request rescue for queued PWQs and rearm the SOS timer, 3123–3146.
///
/// # Safety
/// Invoked for a live pool's native mayday timer; the timer owner pins the pool.
pub(super) unsafe extern "C" fn pool_mayday_timeout(timer: *mut timer_list) {
    // SAFETY: Pool lock precedes mayday lock. Worklist nodes remain live and in
    // place while mayday messages only link PWQ nodes on a separate list.
    unsafe {
        let pool = kernel::container_of!(timer, worker_pool, mayday_timer);
        lupos_wq_pool_lock_irq(pool);
        lupos_wq_raw_spin_lock(lupos_wq_mayday_lock());
        if need_to_create_worker(pool) {
            let head = addr_of_mut!((*pool).worklist);
            let mut entry = (*head).next;
            while entry != head {
                let work = kernel::container_of!(entry, work_struct, entry);
                send_mayday(get_work_pwq(work));
                entry = (*entry).next;
            }
        }
        lupos_wq_raw_spin_unlock(lupos_wq_mayday_lock());
        lupos_wq_pool_unlock_irq(pool);
        mod_timer(
            addr_of_mut!((*pool).mayday_timer),
            lupos_wq_read_jiffies().wrapping_add(MAYDAY_INTERVAL as c_ulong),
        );
    }
}

/// Ensure the manager leaves at least one idle worker, 3166–3195.
///
/// # Safety
/// Called by the unique pool manager with pool IRQ lock held and the pool
/// pinned. Drops/reacquires the lock and may sleep; returns with it held.
unsafe fn maybe_create_worker(pool: *mut worker_pool) {
    // SAFETY: Manager ownership pins the pool across allocation and sleep. The
    // unlocked policy reads are the native advisory checks; the final locked
    // recheck is authoritative. Timer deletion is synchronous before relocking.
    unsafe {
        loop {
            lupos_wq_pool_unlock_irq(pool);
            mod_timer(
                addr_of_mut!((*pool).mayday_timer),
                lupos_wq_read_jiffies().wrapping_add(MAYDAY_INITIAL_TIMEOUT as c_ulong),
            );
            loop {
                if !create_worker(pool).is_null() || !need_to_create_worker(pool) {
                    break;
                }
                schedule_timeout_interruptible(CREATE_COOLDOWN as c_long);
                if !need_to_create_worker(pool) {
                    break;
                }
            }
            timer_delete_sync(addr_of_mut!((*pool).mayday_timer));
            lupos_wq_pool_lock_irq(pool);
            if !need_to_create_worker(pool) {
                return;
            }
        }
    }
}

/// Serialize RT BH callbacks, native lines 3198–3201 / 3216.
///
/// # Safety
/// The pool is live; PREEMPT_RT callers may acquire the configured spinlock.
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn worker_lock_callback(pool: *mut worker_pool) {
    // SAFETY: The configured native pool owns cb_lock, used for BH serialization.
    unsafe { lupos_wq_spin_lock(addr_of_mut!((*pool).cb_lock)) };
}

/// Native non-RT callback-lock definition at line 3216 is intentionally empty.
///
/// # Safety
/// The pool pointer obeys the callback owner's lifetime contract.
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn worker_lock_callback(_pool: *mut worker_pool) {}

/// Release RT BH callback serialization, native lines 3203–3206 / 3217.
///
/// # Safety
/// The caller holds this live pool's cb_lock.
#[cfg(CONFIG_PREEMPT_RT)]
unsafe fn worker_unlock_callback(pool: *mut worker_pool) {
    // SAFETY: This releases the exact native cb_lock acquired for the callback.
    unsafe { lupos_wq_spin_unlock(addr_of_mut!((*pool).cb_lock)) };
}

/// Native non-RT callback-unlock definition at line 3217 is intentionally empty.
///
/// # Safety
/// The pool pointer obeys the callback owner's lifetime contract.
#[cfg(not(CONFIG_PREEMPT_RT))]
unsafe fn worker_unlock_callback(_pool: *mut worker_pool) {}

/// Wait for RT BH callback completion, native lines 3208–3212 / 3218.
///
/// # Safety
/// The caller pins the live pool and may acquire its RT callback spinlock.
#[cfg(CONFIG_PREEMPT_RT)]
pub(super) unsafe fn workqueue_callback_cancel_wait_running(pool: *mut worker_pool) {
    // SAFETY: Acquiring and releasing the same native lock waits for an active
    // callback, without replacing completion or cancellation state machinery.
    unsafe {
        lupos_wq_spin_lock(addr_of_mut!((*pool).cb_lock));
        lupos_wq_spin_unlock(addr_of_mut!((*pool).cb_lock));
    }
}

/// Native non-RT cancellation-wait definition at line 3218 is intentionally empty.
///
/// # Safety
/// The pool pointer obeys the cancellation owner's lifetime contract.
#[cfg(not(CONFIG_PREEMPT_RT))]
pub(super) unsafe fn workqueue_callback_cancel_wait_running(_pool: *mut worker_pool) {}

/// Temporarily assume unique management and notify destroyers, 3244–3260.
///
/// # Safety
/// The caller owns the live worker and holds its pool IRQ lock; this may drop
/// the lock and sleep before reacquiring it.
unsafe fn manage_workers(worker: *mut worker) -> bool {
    // SAFETY: POOL_MANAGER_ACTIVE under pool lock elects a unique manager.
    // Native destruction waits on manager_wait before releasing pool lifetime.
    unsafe {
        let pool = (*worker).pool;
        if (*pool).flags & POOL_MANAGER_ACTIVE != 0 {
            return false;
        }
        (*pool).flags |= POOL_MANAGER_ACTIVE;
        (*pool).manager = worker;
        maybe_create_worker(pool);
        (*pool).manager = null_mut();
        (*pool).flags &= !POOL_MANAGER_ACTIVE;
        rcuwait_wake_up(lupos_wq_manager_wait());
        true
    }
}

/// Run one callback and retire its exact queued generation, native 3276–3445.
///
/// # Safety
/// Caller owns this worker, pins its pool and the queued work/PWQ, and holds
/// the pool IRQ lock. Returns holding that lock, possibly after retirement
/// temporarily dropped it. The callback may free its work allocation.
unsafe fn process_one_work(worker: *mut worker, work: *mut work_struct) {
    // SAFETY: Until PENDING is cleared, the pool lock owns the queued item.
    // After the callback, `work` is only an address token for execute_end; no
    // pointer to any work field, Rust reference, or ownership guard survives
    // invocation. The PWQ's in-flight reference pins queue/pool state through
    // final retirement, and the lockdep map is copied before the callback.
    unsafe {
        let pwq = get_work_pwq(work);
        let pool = (*worker).pool;
        let mut wake_task = null_mut();
        let bh_draining = (*pool).flags & POOL_BH_DRAINING != 0;
        #[cfg(CONFIG_LOCKDEP)]
        let mut lockdep_storage = MaybeUninit::<lockdep_map>::uninit();
        #[cfg(CONFIG_LOCKDEP)]
        let lockdep_map = lockdep_storage.as_mut_ptr();
        #[cfg(CONFIG_LOCKDEP)]
        lupos_wq_lockdep_copy_map(lockdep_map, addr_of!((*work).lockdep_map));

        lupos_wq_warn_process_one_work_wrong_cpu(
            (*pool).flags & POOL_DISASSOCIATED == 0
                && lupos_wq_raw_smp_processor_id() != (*pool).cpu,
        );
        debug_work_deactivate(work);
        lupos_wq_hlist_add_head(
            lupos_wq_worker_hentry(worker),
            lupos_wq_busy_worker_hash_head(pool, work),
        );
        (*worker).current_work = work;
        (*worker).current_func = (*work).func;
        (*worker).current_pwq = pwq;
        if !(*worker).task.is_null() {
            (*worker).current_at = lupos_wq_read_task_sum_exec_runtime((*worker).task);
        }
        (*worker).current_start = lupos_wq_read_jiffies();
        let work_data = *work_data_bits(work);
        (*worker).current_color = get_work_color(work_data) as c_uint;
        sized_strscpy(
            addr_of_mut!((*worker).desc).cast::<c_char>(),
            addr_of!((*(*pwq).wq).name).cast::<c_char>(),
            WORKER_DESC_LEN as usize,
        );
        lupos_wq_list_del_init(addr_of_mut!((*work).entry));
        if (*(*pwq).wq).flags & WQ_CPU_INTENSIVE != 0 {
            worker_set_flags(worker, WORKER_CPU_INTENSIVE);
        }
        kick_pool_pick(pool, addr_of_mut!(wake_task));
        set_work_pool_and_clear_pending(work, (*pool).id, pool_offq_flags(pool));
        (*pwq).stats[PWQ_STAT_STARTED as usize] = (*pwq).stats[PWQ_STAT_STARTED as usize].wrapping_add(1);

        lupos_wq_rcu_read_lock();
        lupos_wq_pool_unlock_irq(pool);
        if !wake_task.is_null() {
            wake_up_process(wake_task);
        }
        lupos_wq_rcu_read_unlock();

        let rcu_start_depth = lupos_wq_rcu_preempt_depth();
        let lockdep_start_depth = lupos_wq_lockdep_depth(lupos_wq_current());
        if !bh_draining {
            #[cfg(CONFIG_LOCKDEP)]
            lupos_wq_lockmap_acquire_process_one_work_wq((*(*pwq).wq).lockdep_map);
        }
        #[cfg(CONFIG_LOCKDEP)]
        lupos_wq_lockmap_acquire_process_one_work_work(lockdep_map);
        lupos_wq_lockdep_invariant_state(true);
        lupos_wq_trace_process_one_work_execute_start(work);
        // A valid queued native work item has an initialized, nonnull callback.
        // Copy only the callback value; do not borrow work across invocation.
        let callback = (*worker).current_func.unwrap_unchecked();
        callback(work);
        // Source trace header's execute_end stores only this address and the
        // supplied callback value; unlike execute_start it never reads ->func.
        lupos_wq_trace_process_one_work_execute_end(work, (*worker).current_func);
        #[cfg(CONFIG_LOCKDEP)]
        lupos_wq_lockmap_release_process_one_work_work(lockdep_map);
        if !bh_draining {
            #[cfg(CONFIG_LOCKDEP)]
            lupos_wq_lockmap_release_process_one_work_wq((*(*pwq).wq).lockdep_map);
        }
        if (!(*worker).task.is_null() && lupos_wq_in_atomic())
            || lupos_wq_lockdep_depth(lupos_wq_current()) != lockdep_start_depth
            || lupos_wq_rcu_preempt_depth() != rcu_start_depth
        {
            lupos_wq_log_process_one_work_leak(
                lupos_wq_current(),
                lockdep_start_depth,
                rcu_start_depth,
                (*worker).current_func,
            );
            #[cfg(CONFIG_LOCKDEP)]
            debug_show_held_locks(lupos_wq_current());
            #[cfg(CONFIG_PRINTK)]
            dump_stack();
        }
        if !(*worker).task.is_null() {
            lupos_wq_cond_resched();
        }
        lupos_wq_pool_lock_irq(pool);
        (*pwq).stats[PWQ_STAT_COMPLETED as usize] = (*pwq).stats[PWQ_STAT_COMPLETED as usize].wrapping_add(1);
        worker_clr_flags(worker, WORKER_CPU_INTENSIVE);
        (*worker).last_func = (*worker).current_func;
        lupos_wq_hash_del(lupos_wq_worker_hentry(worker));
        (*worker).current_work = null_mut();
        (*worker).current_func = None;
        (*worker).current_pwq = null_mut();
        (*worker).current_color = INT_MAX as c_uint;
        // Last operation: retirement can release PWQ lifetime and unlock/relock.
        pwq_dec_nr_in_flight(pwq, work_data);
    }
}

/// Re-read the scheduled list after every callback, native lines 3459–3472.
///
/// # Safety
/// The caller owns the worker and holds its pool IRQ lock; queued items carry
/// their normal lifetime. Each callback may mutate/free its own list item.
unsafe fn process_scheduled_works(worker: *mut worker) {
    // SAFETY: Every iteration obtains the current list head under the re-held
    // pool lock. No successor pointer is retained across callback execution.
    unsafe {
        let mut first = true;
        loop {
            let head = addr_of_mut!((*worker).scheduled);
            if lupos_wq_list_empty(head) {
                break;
            }
            let work = kernel::container_of!((*head).next, work_struct, entry);
            if first {
                (*(*worker).pool).last_progress_ts = lupos_wq_read_jiffies();
                first = false;
            }
            process_one_work(worker, work);
        }
    }
}

/// Coordinate scheduler worker identity with attach/cull, native 3474–3482.
///
/// # Safety
/// Called in the current worker/rescuer task in sleepable context.
unsafe fn set_pf_worker(value: bool) {
    // SAFETY: Only this task changes its own flags here. Attach exclusion stops
    // self-destruction at this point until a culler has detached its membership.
    unsafe {
        lupos_wq_mutex_lock(lupos_wq_pool_attach_mutex());
        let task = lupos_wq_current();
        if value {
            (*task).flags |= PF_WQ_WORKER;
        } else {
            (*task).flags &= !PF_WQ_WORKER;
        }
        mutex_unlock(lupos_wq_pool_attach_mutex());
    }
}

/// Native kthread worker loop, lines 3496–3568.
///
/// # Safety
/// Invoked by kthread for the initialized, attached worker supplied to
/// kthread_create_on_node. The owning culler waits for termination before free.
pub(super) unsafe extern "C" fn worker_thread(data: *mut c_void) -> c_int {
    // SAFETY: The initialized worker is owned by this kthread. Pool locks guard
    // shared state. The culler retains the pool through stop/reap; after DIE,
    // worker->pool is nulled before releasing its ID and returning to kthread.
    unsafe {
        let worker = data.cast::<worker>();
        let pool = (*worker).pool;
        set_pf_worker(true);
        loop {
            lupos_wq_pool_lock_irq(pool);
            if (*worker).flags & WORKER_DIE != 0 {
                lupos_wq_pool_unlock_irq(pool);
                set_pf_worker(false);
                (*worker).pool = null_mut();
                ida_free(addr_of_mut!((*pool).worker_ida), (*worker).id as c_uint);
                return 0;
            }
            worker_leave_idle(worker);
            loop {
                if !need_more_worker(pool) {
                    break;
                }
                if !may_start_working(pool) && manage_workers(worker) {
                    continue;
                }
                lupos_wq_warn_worker_thread_scheduled(!lupos_wq_list_empty(addr_of!((*worker).scheduled)));
                worker_clr_flags(worker, WORKER_PREP | WORKER_REBOUND);
                loop {
                    let work = kernel::container_of!((*pool).worklist.next, work_struct, entry);
                    if assign_work(work, worker, null_mut()) {
                        process_scheduled_works(worker);
                    }
                    if !keep_working(pool) {
                        break;
                    }
                }
                worker_set_flags(worker, WORKER_PREP);
                break;
            }
            worker_enter_idle(worker);
            lupos_wq_set_state_worker_thread_idle();
            lupos_wq_pool_unlock_irq(pool);
            schedule();
        }
    }
}

/// Assign the next eligible rescue chain while maintaining its cursor, 3570–3624.
///
/// # Safety
/// PWQ and rescuer are live, attached to the same pool, and its IRQ lock is held.
/// Native advisory mayday-list observations do not grant ownership of that list.
unsafe fn assign_rescuer_work(pwq: *mut pool_workqueue, rescuer: *mut worker) -> bool {
    // SAFETY: Pool locking protects worklist/cursor membership. Convert only
    // real entries to Rust-visible containers; native address-only conversion
    // leaves transport and recover the C list-head pseudo-container sentinel.
    unsafe {
        let pool = (*pwq).pool;
        let cursor_entry = addr_of_mut!((*pwq).mayday_cursor.entry);
        if (*pwq).nr_active == 0 {
            return false;
        }
        if !need_to_create_worker(pool) {
            if (*pool).flags & POOL_MANAGER_ACTIVE == 0
                || !lupos_wq_list_empty(addr_of!((*(*pwq).wq).maydays))
            {
                return false;
            }
        }
        let head = addr_of_mut!((*pool).worklist);
        let mut entry = if lupos_wq_list_empty(cursor_entry) {
            (*head).next
        } else {
            (*cursor_entry).next
        };
        while entry != head {
            let work = kernel::container_of!(entry, work_struct, entry);
            let mut next = lupos_wq_work_from_entry((*entry).next);
            if get_work_pwq(work) == pwq && assign_work(work, rescuer, addr_of_mut!(next)) {
                (*pwq).stats[PWQ_STAT_RESCUED as usize] = (*pwq).stats[PWQ_STAT_RESCUED as usize].wrapping_add(1);
                lupos_wq_list_move_tail(cursor_entry, lupos_wq_work_entry(next));
                return true;
            }
            // `assign_work` can move a whole colliding chain and replace next.
            entry = lupos_wq_work_entry(next);
        }
        false
    }
}

/// Drain requested PWQs fairly before honoring rescuer stop, 3647–3745.
///
/// # Safety
/// Invoked by kthread for the initialized rescuer. Its WQ is live until this
/// thread exits; each queued mayday owns a PWQ reference.
pub(super) unsafe extern "C" fn rescuer_thread(data: *mut c_void) -> c_int {
    // SAFETY: The rescuer owns its worker storage and retains PREP throughout,
    // so scheduler concurrency accounting excludes it. Each pool is pinned by
    // a mayday PWQ reference until detach followed by put_pwq_unlocked.
    unsafe {
        let rescuer = data.cast::<worker>();
        let wq = (*rescuer).rescue_wq;
        set_user_nice(lupos_wq_current(), RESCUER_NICE_LEVEL as c_long);
        set_pf_worker(true);
        loop {
            lupos_wq_set_state_rescuer_thread_idle();
            let should_stop = kthread_should_stop();
            lupos_wq_raw_spin_lock_irq(lupos_wq_mayday_lock());
            while !lupos_wq_list_empty(addr_of!((*wq).maydays)) {
                let pwq = kernel::container_of!((*wq).maydays.next, pool_workqueue, mayday_node);
                let pool = (*pwq).pool;
                let mut count: c_uint = 0;
                lupos_wq_set_state_rescuer_thread_running();
                lupos_wq_list_del_init(addr_of_mut!((*pwq).mayday_node));
                lupos_wq_raw_spin_unlock_irq(lupos_wq_mayday_lock());
                worker_attach_to_pool(rescuer, pool);
                lupos_wq_pool_lock_irq(pool);
                lupos_wq_warn_rescuer_thread_scheduled(!lupos_wq_list_empty(addr_of!((*rescuer).scheduled)));
                while assign_rescuer_work(pwq, rescuer) {
                    process_scheduled_works(rescuer);
                    count = count.wrapping_add(1);
                    if count > RESCUER_BATCH as c_uint
                        && !lupos_wq_list_empty(addr_of!((*(*pwq).wq).maydays))
                        && (*pwq).nr_active != 0
                        && need_to_create_worker(pool)
                    {
                        lupos_wq_raw_spin_lock(lupos_wq_mayday_lock());
                        send_mayday(pwq);
                        lupos_wq_raw_spin_unlock(lupos_wq_mayday_lock());
                        break;
                    }
                }
                if !lupos_wq_list_empty(addr_of!((*pwq).mayday_cursor.entry))
                    && lupos_wq_list_empty(addr_of!((*pwq).mayday_node))
                {
                    lupos_wq_list_del_init(addr_of_mut!((*pwq).mayday_cursor.entry));
                }
                kick_pool(pool);
                lupos_wq_pool_unlock_irq(pool);
                worker_detach_from_pool(rescuer);
                // Last use of pool/PWQ before reacquiring only the mayday lock.
                put_pwq_unlocked(pwq);
                lupos_wq_raw_spin_lock_irq(lupos_wq_mayday_lock());
            }
            lupos_wq_raw_spin_unlock_irq(lupos_wq_mayday_lock());
            if should_stop {
                lupos_wq_set_state_rescuer_thread_stopping();
                set_pf_worker(false);
                return 0;
            }
            lupos_wq_warn_rescuer_thread_running((*rescuer).flags & WORKER_NOT_RUNNING == 0);
            schedule();
        }
    }
}

/// Execute bounded BH batches under configured callback serialization, 3747–3783.
///
/// # Safety
/// The caller owns the CPU's permanent BH worker execution context, including
/// the supported dead-CPU-drain exception, and enters with IRQs enabled.
unsafe fn bh_worker(worker: *mut worker) {
    // SAFETY: Native per-CPU/RT callback exclusion prevents competing execution.
    // Pool IRQ lock protects lists/flags, is reacquired by each work callback,
    // and is released before callback serialization is released.
    unsafe {
        let pool = (*worker).pool;
        let mut nr_restarts = BH_WORKER_RESTARTS as c_int;
        let end = lupos_wq_read_jiffies().wrapping_add(lupos_wq_bh_worker_jiffies());
        worker_lock_callback(pool);
        lupos_wq_pool_lock_irq(pool);
        worker_leave_idle(worker);
        if need_more_worker(pool) {
            lupos_wq_warn_bh_worker_scheduled(!lupos_wq_list_empty(addr_of!((*worker).scheduled)));
            worker_clr_flags(worker, WORKER_PREP | WORKER_REBOUND);
            loop {
                let work = kernel::container_of!((*pool).worklist.next, work_struct, entry);
                if assign_work(work, worker, null_mut()) {
                    process_scheduled_works(worker);
                }
                if !keep_working(pool) {
                    break;
                }
                nr_restarts = nr_restarts.wrapping_sub(1);
                if nr_restarts == 0 || !lupos_wq_time_before(lupos_wq_read_jiffies(), end) {
                    break;
                }
            }
            worker_set_flags(worker, WORKER_PREP);
        }
        worker_enter_idle(worker);
        kick_pool(pool);
        lupos_wq_pool_unlock_irq(pool);
        worker_unlock_callback(pool);
    }
}

/// Handle the normal or high-priority local BH work pool, native 3796–3802.
///
/// # Safety
/// Called in the corresponding CPU's tasklet/softirq action with initialized
/// permanent BH pools and native softirq exclusion.
pub(super) unsafe extern "C" fn workqueue_softirq_action(highpri: bool) {
    // SAFETY: The native per-CPU leaf resolves the selected local pool. Its
    // permanent worker cannot detach; the advisory predicate is only a fast exit.
    unsafe {
        let pool = lupos_wq_bh_pool_at(lupos_wq_smp_processor_id() as c_int, highpri as c_int);
        if need_more_worker(pool) {
            let worker = kernel::container_of!((*pool).workers.next, worker, node);
            bh_worker(worker);
        }
    }
}

/// Drain a dead CPU's BH pool without monopolizing the live CPU, 3810–3848.
///
/// # Safety
/// This callback belongs to a live, stationary native wq_drain_dead_softirq_work
/// on its waiting caller's stack. The target CPU is dead until completion.
unsafe extern "C" fn drain_dead_softirq_workfn(work: *mut work_struct) {
    // SAFETY: The stack object stays alive until complete and is only resubmitted
    // while its caller waits. Permanent target pool/worker lifetime is protected
    // by CPU hotplug; DRAINING encloses the nested BH execution exactly.
    unsafe {
        let dead_work = kernel::container_of!(work, wq_drain_dead_softirq_work, work);
        let pool = (*dead_work).pool;
        lupos_wq_pool_lock_irq(pool);
        (*pool).flags |= POOL_BH_DRAINING;
        lupos_wq_pool_unlock_irq(pool);
        let worker = kernel::container_of!((*pool).workers.next, worker, node);
        bh_worker(worker);
        lupos_wq_pool_lock_irq(pool);
        (*pool).flags &= !POOL_BH_DRAINING;
        let repeat = need_more_worker(pool);
        lupos_wq_pool_unlock_irq(pool);
        if repeat {
            if (*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL {
                queue_work_on(WORK_CPU_UNBOUND as c_int, system_bh_highpri_wq, work);
            } else {
                queue_work_on(WORK_CPU_UNBOUND as c_int, system_bh_wq, work);
            }
        } else {
            complete(addr_of_mut!((*dead_work).done));
        }
    }
}

/// Synchronously drain both BH pools after their CPU dies, native 3858–3881.
///
/// # Safety
/// CPU is a valid, dead CPU under native hotplug serialization. The caller may
/// sleep and holds hotplug lifetime until both stack-backed drains complete.
pub(super) unsafe extern "C" fn workqueue_softirq_dead(cpu: c_uint) {
    // SAFETY: Each native stack allocation is initialized in place, published
    // only after work/pool/completion initialization, and never moved. Waiting
    // precedes debug-object destruction and the end of its storage lifetime.
    unsafe {
        let mut index: c_uint = 0;
        while index < NR_STD_WORKER_POOLS as c_uint {
            let pool = lupos_wq_bh_pool_at(cpu as c_int, index as c_int);
            index = index.wrapping_add(1);
            if !need_more_worker(pool) {
                continue;
            }
            let mut storage = MaybeUninit::<wq_drain_dead_softirq_work>::uninit();
            let dead_work = storage.as_mut_ptr();
            let work = addr_of_mut!((*dead_work).work);
            // Reuse the genuine native Rust helper, with the original expanded
            // INIT_WORK_ONSTACK lockdep name and a unique native static key.
            kernel::bindings::init_work_with_key(
                work,
                Some(drain_dead_softirq_workfn),
                true,
                c"(work_completion)(&dead_work.work)".as_ptr().cast::<c_char>(),
                lupos_wq_dead_softirq_work_key(),
            );
            (*dead_work).pool = pool;
            lupos_wq_init_completion_c3871(addr_of_mut!((*dead_work).done));
            if (*(*pool).attrs).nice == HIGHPRI_NICE_LEVEL {
                queue_work_on(WORK_CPU_UNBOUND as c_int, system_bh_highpri_wq, work);
            } else {
                queue_work_on(WORK_CPU_UNBOUND as c_int, system_bh_wq, work);
            }
            wait_for_completion(addr_of_mut!((*dead_work).done));
            lupos_wq_destroy_work_on_stack(work);
        }
    }
}
