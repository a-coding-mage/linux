// SPDX-License-Identifier: GPL-2.0-only
// UP uses the configured native header alternative; SMP keeps the Rust owner.
#[cfg(not(CONFIG_SMP))]
use kernel::bindings::sched_core_native::lupos_core_header_set_task_cpu as set_task_cpu;

unsafe fn is_cpu_allowed(p: *mut task_struct, cpu: c_int) -> bool {
    // SAFETY: The caller keeps p live and stabilizes its allowed mask with
    // pi/rq locking; cpu names native CPU-mask and hotplug state.
    unsafe {
        if !lupos_core_task_allowed_on_cpu(p, cpu) {
            return false;
        }
        if lupos_core_is_migration_disabled(p) {
            return lupos_core_cpu_online(cpu);
        }
        if (*p).flags & LUPOS_CORE_PF_KTHREAD == 0 {
            return lupos_core_cpu_active(cpu);
        }
        if lupos_core_kthread_is_per_cpu(p) {
            return lupos_core_cpu_online(cpu);
        }
        if lupos_core_cpu_dying(cpu) {
            return false;
        }
        lupos_core_cpu_online(cpu)
    }
}
unsafe fn rq_has_pinned_tasks(rq: *mut rq) -> bool {
    // SAFETY: The caller supplies a live runqueue and the scheduler/hotplug
    // synchronization required to inspect its pinned-task count.
    unsafe {
        (*rq).nr_pinned != 0
    }
}
unsafe fn move_queued_task(
    mut rq: *mut rq,
    rf: *mut rq_flags,
    p: *mut task_struct,
    cpu: c_int,
) -> *mut rq {
    // SAFETY: The caller keeps p live with pi serialization and holds rq's
    // lock with matching flags. Migration transfers that lock to the destination.
    unsafe {
        lupos_core_assert_rq_held(rq);
        deactivate_task(rq, p, LUPOS_CORE_DEQUEUE_NOCLOCK);
        set_task_cpu(p, cpu as c_uint);
        lupos_core_rq_unlock(rq, rf);
        rq = lupos_core_cpu_rq(cpu);
        lupos_core_rq_lock(rq, rf);
        lupos_core_warn_moved_cpu(lupos_core_task_cpu(p) != cpu);
        activate_task(rq, p, 0);
        wakeup_preempt(rq, p, 0);
        rq
    }
}
unsafe fn __migrate_task(
    rq: *mut rq,
    rf: *mut rq_flags,
    p: *mut task_struct,
    cpu: c_int,
) -> *mut rq {
    // SAFETY: The caller holds the live task's source rq lock and migration
    // serialization; move_queued_task preserves the locked-runqueue return contract.
    unsafe {
        if !is_cpu_allowed(p, cpu) {
            return rq;
        }
        move_queued_task(rq, rf, p, cpu)
    }
}
unsafe extern "C" fn migration_cpu_stop(data: *mut c_void) -> c_int {
    // SAFETY: The stopper owns a live migration_arg and task. pi/rq locking
    // protects migration; a pending request stays live through completion/requeue.
    unsafe {
        let arg = data.cast::<migration_arg>();
        let pending = (*arg).pending;
        let p = (*arg).task;
        let mut rq = lupos_core_this_rq();
        let mut complete = false;
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        (*rf.as_mut_ptr()).flags = lupos_core_local_irq_save();
        lupos_core_header_flush_smp_call_function_queue();
        lupos_core_raw_spin_lock_nested(addr_of_mut!((*p).pi_lock), 0);
        lupos_core_rq_lock(rq, rf.as_mut_ptr());
        lupos_core_warn_migration_pending(
            !pending.is_null() && pending != (*p).migration_pending.cast::<set_affinity_pending>(),
        );
        'locked: {
            if lupos_core_task_rq(p) == rq {
                if lupos_core_is_migration_disabled(p) {
                    break 'locked;
                }
                if !pending.is_null() {
                    (*p).migration_pending = null_mut();
                    complete = true;
                    if lupos_core_cpumask_test_cpu(lupos_core_task_cpu(p), addr_of!((*p).cpus_mask)) {
                        break 'locked;
                    }
                }
                if lupos_core_task_on_rq_queued(p) {
                    update_rq_clock(rq);
                    rq = __migrate_task(rq, rf.as_mut_ptr(), p, (*arg).dest_cpu);
                } else {
                    (*p).wake_cpu = (*arg).dest_cpu;
                }
            } else if !pending.is_null() {
                if lupos_core_cpumask_test_cpu(lupos_core_task_cpu(p), (*p).cpus_ptr) {
                    (*p).migration_pending = null_mut();
                    complete = true;
                    break 'locked;
                }
                lupos_core_warn_stop_pending((*pending).stop_pending == 0);
                lupos_core_preempt_disable();
                lupos_core_rq_unlock(rq, rf.as_mut_ptr());
                lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), (*rf.as_ptr()).flags);
                lupos_core_header_stop_one_cpu_nowait(
                    lupos_core_task_cpu(p) as c_uint,
                    Some(migration_cpu_stop),
                    addr_of_mut!((*pending).arg).cast(),
                    addr_of_mut!((*pending).stop_work),
                );
                lupos_core_preempt_enable();
                return 0;
            }
        }
        if !pending.is_null() {
            (*pending).stop_pending = 0;
        }
        lupos_core_rq_unlock(rq, rf.as_mut_ptr());
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), (*rf.as_ptr()).flags);
        if complete {
            complete_all(addr_of_mut!((*pending).done));
        }
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn push_cpu_stop(arg: *mut c_void) -> c_int {
    // SAFETY: The queued stopper owns p's task reference. pi/rq locks protect
    // class callbacks and migration; that reference is dropped after unlocking.
    unsafe {
        let rq = lupos_core_this_rq();
        let p = arg.cast::<task_struct>();
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*p).pi_lock));
        raw_spin_rq_lock_nested(rq, 0);
        'locked: {
            if lupos_core_task_rq(p) != rq {
                break 'locked;
            }
            if lupos_core_is_migration_disabled(p) {
                (*p).migration_flags |= LUPOS_CORE_MDF_PUSH;
                break 'locked;
            }
            (*p).migration_flags &= !LUPOS_CORE_MDF_PUSH;
            let lowest = if let Some(find) = (*(*p).sched_class).find_lock_rq {
                find(p, rq)
            } else {
                null_mut()
            };
            if lowest.is_null() {
                break 'locked;
            }
            lupos_core_assert_rq_held(lowest);
            if lupos_core_task_rq(p) == rq {
                lupos_core_move_queued_task_locked(rq, lowest, p);
                resched_curr(lowest);
            }
            lupos_core_double_unlock_balance(rq, lowest);
        }
        (*rq).push_busy = 0;
        lupos_core_raw_spin_rq_unlock(rq);
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*p).pi_lock));
        lupos_core_put_task_struct(p);
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_cpus_allowed_common(p: *mut task_struct, ctx: *mut affinity_context) {
    // SAFETY: The caller serializes p's affinity and supplies a live context
    // and masks. Class-change locking protects task and MM affinity updates.
    unsafe {
        if (*ctx).flags & (LUPOS_CORE_SCA_MIGRATE_ENABLE | LUPOS_CORE_SCA_MIGRATE_DISABLE) != 0 {
            (*p).cpus_ptr = (*ctx).new_mask;
            return;
        }
        lupos_core_cpumask_copy(addr_of_mut!((*p).cpus_mask), (*ctx).new_mask);
        (*p).nr_cpus_allowed = lupos_core_cpumask_weight((*ctx).new_mask) as c_int;
        mm_update_cpus_allowed((*p).mm, (*ctx).new_mask);
        if (*ctx).flags & LUPOS_CORE_SCA_USER != 0 {
            core::ptr::swap(
                addr_of_mut!((*p).user_cpus_ptr),
                addr_of_mut!((*ctx).user_mask),
            );
        }
    }
}
unsafe fn do_set_cpus_allowed(p: *mut task_struct, ctx: *mut affinity_context) {
    // SAFETY: The caller holds p's rq lock and affinity serialization. The
    // class callback receives the live context between matched sched_change calls.
    unsafe {
        let change = sched_change_begin(p, LUPOS_CORE_DEQUEUE_SAVE as c_uint);
        ((*(*p).sched_class).set_cpus_allowed.unwrap_unchecked())(p, ctx);
        sched_change_end(change);
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_cpus_allowed_force(p: *mut task_struct, mask: *const cpumask) {
    // SAFETY: The caller keeps p and mask live and holds p's pi lock. The
    // rq lock covers the class update; the old mask uses native RCU reclamation.
    unsafe {
        let mut ac = affinity_context {
            new_mask: mask,
            user_mask: null_mut(),
            flags: LUPOS_CORE_SCA_USER,
        };
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = ___task_rq_lock(p, rf.as_mut_ptr());
        do_set_cpus_allowed(p, addr_of_mut!(ac));
        lupos_core_task_rq_unlock_only(rq, p, rf.as_mut_ptr());
        lupos_core_kfree_cpumask_rcu(ac.user_mask);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dup_user_cpus_ptr(
    dst: *mut task_struct,
    src: *mut task_struct,
    node: c_int,
) -> c_int {
    // SAFETY: The fork caller owns dst and keeps src live. The source pi lock
    // protects the copied mask; allocation ownership follows the native swap/free.
    unsafe {
        (*dst).user_cpus_ptr = null_mut();
        if lupos_core_data_race_user_cpus_ptr(src).is_null() {
            return 0;
        }
        let mut user_mask = lupos_core_alloc_user_cpus_ptr(node);
        if user_mask.is_null() {
            return -LUPOS_CORE_ENOMEM;
        }
        let flags = lupos_core_raw_spin_lock_irqsave(addr_of_mut!((*src).pi_lock));
        if !(*src).user_cpus_ptr.is_null() {
            core::ptr::swap(addr_of_mut!((*dst).user_cpus_ptr), addr_of_mut!(user_mask));
            lupos_core_cpumask_copy((*dst).user_cpus_ptr, (*src).user_cpus_ptr);
        }
        lupos_core_raw_spin_unlock_irqrestore(addr_of_mut!((*src).pi_lock), flags);
        if !user_mask.is_null() {
            kfree(user_mask.cast());
        }
        0
    }
}
unsafe fn clear_user_cpus_ptr(p: *mut task_struct) -> *mut cpumask {
    // SAFETY: The caller keeps p live and exclusively owns or serializes its
    // user-mask pointer while detaching that allocation.
    unsafe {
        let mask = (*p).user_cpus_ptr;
        (*p).user_cpus_ptr = null_mut();
        mask
    }
}
#[no_mangle]
pub unsafe extern "C" fn release_user_cpus_ptr(p: *mut task_struct) {
    // SAFETY: The task teardown caller owns p's user-mask allocation and
    // serializes its detachment before passing it to native kfree.
    unsafe {
        kfree(clear_user_cpus_ptr(p).cast());
    }
}
unsafe fn affine_move_task(
    mut rq: *mut rq,
    p: *mut task_struct,
    rf: *mut rq_flags,
    dest: c_int,
    flags: c_uint,
) -> c_int {
    // SAFETY: The caller holds p's pi/rq locks and keeps p live. Request refs
    // keep shared pending storage live; its owner waits before leaving the stack.
    unsafe {
        // C uses a zero-initialized, stack-owned native set_affinity_pending.
        let mut my_pending: set_affinity_pending = core::mem::zeroed();
        let mut complete = false;
        if lupos_core_cpumask_test_cpu(lupos_core_task_cpu(p), addr_of!((*p).cpus_mask))
            || (lupos_core_task_current_donor(rq, p) && !lupos_core_task_current(rq, p))
        {
            let mut push_task: *mut task_struct = null_mut();
            if flags & LUPOS_CORE_SCA_MIGRATE_ENABLE != 0
                && (*p).migration_flags & LUPOS_CORE_MDF_PUSH != 0
                && (*rq).push_busy == 0
            {
                (*rq).push_busy = 1;
                push_task = lupos_core_get_task_struct(p);
            }
            let pending = (*p).migration_pending.cast::<set_affinity_pending>();
            if !pending.is_null() && (*pending).stop_pending == 0 {
                (*p).migration_pending = null_mut();
                complete = true;
            }
            lupos_core_preempt_disable();
            lupos_core_task_rq_unlock(rq, p, rf);
            if !push_task.is_null() {
                lupos_core_header_stop_one_cpu_nowait(
                    (*rq).cpu as c_uint,
                    Some(push_cpu_stop),
                    p.cast(),
                    addr_of_mut!((*rq).push_work),
                );
            }
            lupos_core_preempt_enable();
            if complete {
                complete_all(addr_of_mut!((*pending).done));
            }
            return 0;
        }
        if flags & LUPOS_CORE_SCA_MIGRATE_ENABLE == 0 {
            if (*p)
                .migration_pending
                .cast::<set_affinity_pending>()
                .is_null()
            {
                lupos_core_refcount_set(addr_of_mut!(my_pending.refs), 1);
                lupos_core_init_completion(addr_of_mut!(my_pending.done));
                my_pending.arg = migration_arg {
                    task: p,
                    dest_cpu: dest,
                    pending: addr_of_mut!(my_pending),
                };
                (*p).migration_pending = addr_of_mut!(my_pending).cast();
            } else {
                let pending = (*p).migration_pending.cast::<set_affinity_pending>();
                lupos_core_refcount_inc(addr_of_mut!((*pending).refs));
                (*pending).arg.dest_cpu = dest;
            }
        }
        let pending = (*p).migration_pending.cast::<set_affinity_pending>();
        if lupos_core_warn_missing_pending(pending.is_null()) {
            lupos_core_task_rq_unlock(rq, p, rf);
            return -LUPOS_CORE_EINVAL;
        }
        if lupos_core_task_on_cpu(rq, p)
            || lupos_core_read_once_uint(addr_of!((*p).__state)) == LUPOS_CORE_TASK_WAKING
        {
            let stop_pending = (*pending).stop_pending;
            if stop_pending == 0 {
                (*pending).stop_pending = 1;
            }
            if flags & LUPOS_CORE_SCA_MIGRATE_ENABLE != 0 {
                (*p).migration_flags &= !LUPOS_CORE_MDF_PUSH;
            }
            lupos_core_preempt_disable();
            lupos_core_task_rq_unlock(rq, p, rf);
            if stop_pending == 0 {
                lupos_core_header_stop_one_cpu_nowait(
                    lupos_core_cpu_of(rq) as c_uint,
                    Some(migration_cpu_stop),
                    addr_of_mut!((*pending).arg).cast(),
                    addr_of_mut!((*pending).stop_work),
                );
            }
            lupos_core_preempt_enable();
            if flags & LUPOS_CORE_SCA_MIGRATE_ENABLE != 0 {
                return 0;
            }
        } else {
            if !lupos_core_is_migration_disabled(p) {
                if lupos_core_task_on_rq_queued(p) {
                    rq = move_queued_task(rq, rf, p, dest);
                }
                if (*pending).stop_pending == 0 {
                    (*p).migration_pending = null_mut();
                    complete = true;
                }
            }
            lupos_core_task_rq_unlock(rq, p, rf);
            if complete {
                complete_all(addr_of_mut!((*pending).done));
            }
        }
        wait_for_completion(addr_of_mut!((*pending).done));
        // Capture the field address while our reference keeps pending live. The
        // final decrement may let its stack-owning caller return immediately.
        let refs = addr_of_mut!((*pending).refs);
        if lupos_core_refcount_dec_and_test(refs) {
            // Native wake_up_var only hashes/compares this saved address.
            lupos_core_wake_up_var(refs.cast());
        }
        lupos_core_wait_pending_refs(addr_of_mut!(my_pending.refs));
        lupos_core_warn_owner_stop_pending(my_pending.stop_pending != 0);
        0
    }
}
unsafe fn __set_cpus_allowed_ptr_locked(
    p: *mut task_struct,
    ctx: *mut affinity_context,
    rq: *mut rq,
    rf: *mut rq_flags,
) -> c_int {
    // SAFETY: The caller supplies live p/context/masks and holds both pi/rq
    // locks with matching flags; all return paths consume that lock ownership.
    unsafe {
        let allowed = lupos_core_task_cpu_possible_mask(p);
        let kthread = (*p).flags & LUPOS_CORE_PF_KTHREAD != 0;
        let valid = if kthread || lupos_core_is_migration_disabled(p) {
            lupos_core_cpu_online_mask()
        } else {
            lupos_core_cpu_active_mask()
        };
        let ret = 'locked: {
            if !kthread && !lupos_core_cpumask_subset((*ctx).new_mask, allowed) {
                break 'locked -LUPOS_CORE_EINVAL;
            }
            if (*ctx).flags & LUPOS_CORE_SCA_CHECK != 0
                && (*p).flags & LUPOS_CORE_PF_NO_SETAFFINITY != 0
            {
                break 'locked -LUPOS_CORE_EINVAL;
            }
            if (*ctx).flags & LUPOS_CORE_SCA_MIGRATE_ENABLE == 0 {
                if lupos_core_cpumask_equal(addr_of!((*p).cpus_mask), (*ctx).new_mask) {
                    if (*ctx).flags & LUPOS_CORE_SCA_USER != 0 {
                        core::ptr::swap(
                            addr_of_mut!((*p).user_cpus_ptr),
                            addr_of_mut!((*ctx).user_mask),
                        );
                    }
                    break 'locked 0;
                }
                if lupos_core_warn_affinity_busy(
                    p == lupos_core_current()
                        && lupos_core_is_migration_disabled(p)
                        && !lupos_core_cpumask_test_cpu(lupos_core_task_cpu(p), (*ctx).new_mask),
                ) {
                    break 'locked -LUPOS_CORE_EBUSY;
                }
            }
            let dest = lupos_core_header_cpumask_any_and_distribute(valid, (*ctx).new_mask);
            if dest >= lupos_core_nr_cpu_ids() as c_uint {
                break 'locked -LUPOS_CORE_EINVAL;
            }
            do_set_cpus_allowed(p, ctx);
            return affine_move_task(rq, p, rf, dest as c_int, (*ctx).flags);
        };
        lupos_core_task_rq_unlock(rq, p, rf);
        ret
    }
}
#[no_mangle]
pub unsafe extern "C" fn __set_cpus_allowed_ptr(
    p: *mut task_struct,
    ctx: *mut affinity_context,
) -> c_int {
    // SAFETY: The caller keeps p and the context/masks live through any wait.
    // The acquired pi/rq locks protect mask selection and are consumed downstream.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = _task_rq_lock(p, rf.as_mut_ptr());
        if !(*p).user_cpus_ptr.is_null()
            && (*ctx).flags
                & (LUPOS_CORE_SCA_USER | LUPOS_CORE_SCA_MIGRATE_ENABLE | LUPOS_CORE_SCA_MIGRATE_DISABLE)
                == 0
            && lupos_core_cpumask_and(
                lupos_core_rq_scratch_mask(rq),
                (*ctx).new_mask,
                (*p).user_cpus_ptr,
            )
        {
            (*ctx).new_mask = lupos_core_rq_scratch_mask(rq);
        }
        __set_cpus_allowed_ptr_locked(p, ctx, rq, rf.as_mut_ptr())
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_cpus_allowed_ptr(p: *mut task_struct, mask: *const cpumask) -> c_int {
    // SAFETY: The caller keeps p and mask live through the potentially
    // blocking affinity operation; the stack context lasts until it returns.
    unsafe {
        let mut ac = affinity_context {
            new_mask: mask,
            user_mask: null_mut(),
            flags: 0,
        };
        __set_cpus_allowed_ptr(p, addr_of_mut!(ac))
    }
}
unsafe fn restrict_cpus_allowed_ptr(
    p: *mut task_struct,
    mask: *mut cpumask,
    subset: *const cpumask,
) -> c_int {
    // SAFETY: The caller owns writable mask storage and keeps p/subset live.
    // The acquired pi/rq locks protect mask intersection and affinity changes.
    unsafe {
        let mut ac = affinity_context {
            new_mask: mask,
            user_mask: null_mut(),
            flags: 0,
        };
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = _task_rq_lock(p, rf.as_mut_ptr());
        let err = if lupos_core_task_has_dl_policy(p) && lupos_core_dl_bandwidth_enabled() {
            -LUPOS_CORE_EPERM
        } else if !lupos_core_cpumask_and(mask, lupos_core_task_user_cpus(p), subset) {
            -LUPOS_CORE_EINVAL
        } else {
            return __set_cpus_allowed_ptr_locked(p, addr_of_mut!(ac), rq, rf.as_mut_ptr());
        };
        lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
        err
    }
}
#[no_mangle]
pub unsafe extern "C" fn relax_compatible_cpus_allowed_ptr(p: *mut task_struct) {
    // SAFETY: The caller keeps p live and serializes this restoration with
    // forced restriction; native setaffinity handles cpuset masking and locking.
    unsafe {
        let mut ac = affinity_context {
            new_mask: lupos_core_task_user_cpus(p),
            user_mask: null_mut(),
            flags: 0,
        };
        let ret = __sched_setaffinity(p, addr_of_mut!(ac));
        lupos_core_warn_relax_affinity(ret != 0);
    }
}
unsafe fn migrate_disable_switch(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The scheduler caller keeps rq and p live. The task-rq guard
    // serializes the class callback that installs the per-CPU migration mask.
    unsafe {
        let mut ac = affinity_context {
            new_mask: lupos_core_cpumask_of((*rq).cpu),
            user_mask: null_mut(),
            flags: LUPOS_CORE_SCA_MIGRATE_DISABLE,
        };
        if (*p).migration_disabled == 0 || (*p).cpus_ptr != addr_of!((*p).cpus_mask) {
            return;
        }
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rq = _task_rq_lock(p, rf.as_mut_ptr());
        do_set_cpus_allowed(p, addr_of_mut!(ac));
        lupos_core_task_rq_unlock(rq, p, rf.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn ___migrate_enable() {
    // SAFETY: The migration-enable caller keeps current and its cpus_mask
    // live, with the native migrate-enable preconditions for affinity restoration.
    unsafe {
        let p = lupos_core_current();
        let mut ac = affinity_context {
            new_mask: addr_of!((*p).cpus_mask),
            user_mask: null_mut(),
            flags: LUPOS_CORE_SCA_MIGRATE_ENABLE,
        };
        __set_cpus_allowed_ptr(p, addr_of_mut!(ac));
    }
}
#[no_mangle]
pub unsafe extern "C" fn migrate_disable() {
    // SAFETY: The task-context caller meets the native migration-disable
    // contract; the configured header operation owns its per-task bookkeeping.
    unsafe {
        lupos_core_migrate_disable_inline();
    }
}
#[no_mangle]
pub unsafe extern "C" fn migrate_enable() {
    // SAFETY: The caller pairs a prior migration disable in task context;
    // the configured header operation owns the matching enable bookkeeping.
    unsafe {
        lupos_core_migrate_enable_inline();
    }
}
#[no_mangle]
pub unsafe extern "C" fn kick_process(p: *mut task_struct) {
    // SAFETY: The caller keeps p live. Preemption is disabled around CPU
    // selection and the native current-task check; a racing migration is allowed.
    unsafe {
        lupos_core_preempt_disable();
        let cpu = lupos_core_task_cpu(p);
        if cpu != lupos_core_smp_processor_id() && task_curr(p) != 0 {
            lupos_core_smp_send_reschedule(cpu);
        }
        lupos_core_preempt_enable();
    }
}
unsafe fn select_fallback_rq(cpu: c_int, p: *mut task_struct) -> c_int {
    // SAFETY: The caller keeps p live and holds its pi lock, stabilizing
    // affinity while native topology/hotplug masks and class callbacks are used.
    unsafe {
        let nid = lupos_core_cpu_to_node(cpu);
        // Original function-local enum is represented by native constants.
        let mut state = LUPOS_CORE_FALLBACK_CPUSET;
        if nid != -1 {
            let mask = lupos_core_cpumask_of_node(nid);
            let mut dest = lupos_core_cpu_next(-1, mask);
            while dest < lupos_core_nr_cpu_ids() {
                if is_cpu_allowed(p, dest) {
                    return dest;
                }
                dest = lupos_core_cpu_next(dest, mask);
            }
        }
        loop {
            let mut dest = lupos_core_cpu_next(-1, (*p).cpus_ptr);
            while dest < lupos_core_nr_cpu_ids() {
                if is_cpu_allowed(p, dest) {
                    if state != LUPOS_CORE_FALLBACK_CPUSET
                        && !(*p).mm.is_null()
                        && lupos_core_printk_ratelimit()
                    {
                        lupos_core_print_affinity_lost(p, cpu);
                    }
                    return dest;
                }
                dest = lupos_core_cpu_next(dest, (*p).cpus_ptr);
            }
            if state == LUPOS_CORE_FALLBACK_CPUSET && lupos_core_cpuset_cpus_allowed_fallback(p) {
                state = LUPOS_CORE_FALLBACK_POSSIBLE;
            } else if state == LUPOS_CORE_FALLBACK_FAIL {
                lupos_core_bug_site_3599();
            } else {
                set_cpus_allowed_force(p, lupos_core_task_cpu_fallback_mask(p));
                state = LUPOS_CORE_FALLBACK_FAIL;
            }
        }
    }
}
unsafe fn select_task_rq(p: *mut task_struct, mut cpu: c_int, flags: *mut c_int) -> c_int {
    // SAFETY: The caller holds p's pi lock and supplies writable flags, so
    // the class selector and fallback operate on a stable task affinity.
    unsafe {
        lupos_core_assert_pi_lock(p);
        if (*p).nr_cpus_allowed > 1 && !lupos_core_is_migration_disabled(p) {
            cpu = ((*(*p).sched_class).select_task_rq.unwrap_unchecked())(p, cpu, *flags);
            *flags |= LUPOS_CORE_WF_RQ_SELECTED;
        } else {
            cpu = lupos_core_cpumask_any((*p).cpus_ptr);
        }
        if !is_cpu_allowed(p, cpu) {
            cpu = select_fallback_rq(lupos_core_task_cpu(p), p);
        }
        cpu
    }
}
#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn set_task_cpu(p: *mut task_struct, cpu: c_uint) {
    // SAFETY: The caller keeps p live and holds its pi or rq lock as required
    // for waking or runnable migration; native callbacks update that live task.
    unsafe {
        let state = lupos_core_read_once_uint(addr_of!((*p).__state));
        lupos_core_warn_set_cpu_blocked(
            state != LUPOS_CORE_TASK_RUNNING && state != LUPOS_CORE_TASK_WAKING && (*p).on_rq == 0,
        );
        lupos_core_warn_set_cpu_fair(
            state == LUPOS_CORE_TASK_RUNNING
                && (*p).sched_class == addr_of!(fair_sched_class)
                && (*p).on_rq != 0
                && !lupos_core_task_on_rq_migrating(p),
        );
        #[cfg(CONFIG_LOCKDEP)]
        lupos_core_warn_set_cpu_lockdep(p);
        lupos_core_warn_set_cpu_offline(!lupos_core_cpu_online(cpu as c_int));
        lupos_core_warn_set_cpu_disabled(lupos_core_is_migration_disabled(p));
        lupos_core_trace_migrate_task(p, cpu as c_int);
        if lupos_core_task_cpu(p) as c_uint != cpu {
            if let Some(migrate) = (*(*p).sched_class).migrate_task_rq {
                migrate(p, cpu as c_int);
            }
            (*p).se.nr_migrations = (*p).se.nr_migrations.wrapping_add(1);
            lupos_core_perf_task_migrate(p);
        }
        lupos_core___set_task_cpu(p, cpu);
    }
}
