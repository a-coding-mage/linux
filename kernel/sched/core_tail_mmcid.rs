// SPDX-License-Identifier: GPL-2.0-only
// core.c:10818..11215. All CID constants/layouts are configured native authority.
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn __mm_update_max_cids(mc: *mut mm_mm_cid) {
    // SAFETY: The caller holds the live MM-CID lock; native unsigned
    // constraints and ordered publication determine the configured allocation limit.
    unsafe {
        let opt = (*mc).nr_cpus_allowed.min((*mc).users);
        let max = opt
            .wrapping_add(opt / 4)
            .min(lupos_core_num_possible_cpus());
        lupos_core_write_once_uint(addr_of_mut!((*mc).max_cids), max);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
#[inline(always)]
unsafe fn mm_cid_calc_pcpu_thrs(mc: *mut mm_mm_cid) -> c_uint {
    // SAFETY: The caller holds the live MM-CID lock while reading its
    // user/affinity constraints and the native possible-CPU count.
    unsafe {
        let opt = (*mc).nr_cpus_allowed.min((*mc).users);
        opt.wrapping_sub(opt / 4)
            .min(lupos_core_num_possible_cpus() / 2)
            .max(1)
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_update_max_cids(mm: *mut mm_struct) -> bool {
    // SAFETY: The caller owns the live MM-CID lock. Mode publication is
    // followed by the native barrier before any runqueue fixup can acquire a lock.
    unsafe {
        let mc = addr_of_mut!((*mm).mm_cid);
        let percpu = lupos_core_cid_on_cpu((*mc).mode);
        lupos_core_assert_raw_spin_held(addr_of!((*mc).lock));
        (*mc).update_deferred = 0;
        __mm_update_max_cids(mc);
        if !percpu {
            if (*mc).users > (*mc).max_cids {
                (*mc).pcpu_thrs = mm_cid_calc_pcpu_thrs(mc);
            }
        } else if (*mc).users < (*mc).pcpu_thrs {
            (*mc).pcpu_thrs = 0;
        }
        if percpu == ((*mc).pcpu_thrs != 0) {
            return false;
        }
        let mode = (*mc).mode ^ (LUPOS_CORE_MM_CID_TRANSIT | LUPOS_CORE_MM_CID_ONCPU);
        lupos_core_write_once_uint(addr_of_mut!((*mc).mode), mode);
        // Publish mode before any subsequent rq lock acquire in a fixup.
        lupos_core_smp_mb();
        true
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_update_cpus_allowed(mm: *mut mm_struct, affmsk: *const cpumask) {
    // SAFETY: A non-null mm and affinity mask remain live under the caller's
    // task/rq contract; MM-CID locking protects mask growth and deferred-work state.
    unsafe {
        if mm.is_null() || lupos_core_read_once_uint(addr_of!((*mm).mm_cid.users)) == 0 {
            return;
        }
        let mc = addr_of_mut!((*mm).mm_cid);
        lupos_core_raw_spin_lock(addr_of_mut!((*mc).lock));
        // Keep early exits inside this scope so every exit releases exactly once.
        (|| {
            let allowed = lupos_core_mm_cpus_allowed(mm);
            let weight = lupos_core_cpumask_weighted_or(allowed, allowed, affmsk);
            if weight == (*mc).nr_cpus_allowed {
                return;
            }
            lupos_core_write_once_uint(addr_of_mut!((*mc).nr_cpus_allowed), weight);
            __mm_update_max_cids(mc);
            if !lupos_core_cid_on_cpu((*mc).mode) {
                return;
            }
            (*mc).pcpu_thrs = mm_cid_calc_pcpu_thrs(mc);
            if (*mc).users >= (*mc).pcpu_thrs || (*mc).update_deferred != 0 {
                return;
            }
            (*mc).update_deferred = 1;
            irq_work_queue(addr_of_mut!((*mc).irq_work));
        })();
        lupos_core_raw_spin_unlock(addr_of_mut!((*mc).lock));
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_complete_transit(mm: *mut mm_struct, mode: c_uint) {
    // SAFETY: The caller serializes this live MM's mode transition and has
    // completed fixups; the native barrier precedes stable-mode publication.
    unsafe {
        // Complete all fixups before publishing stable ownership.
        lupos_core_smp_mb();
        lupos_core_write_once_uint(addr_of_mut!((*mm).mm_cid.mode), mode);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_transit_to_task(t: *mut task_struct, pcp: *mut mm_cid_pcpu) {
    // SAFETY: The caller owns the live task's rq or MM-CID IRQ lock and
    // keeps its per-CPU CID slot valid through the ownership transition.
    unsafe {
        if lupos_core_cid_on_cpu((*t).mm_cid.cid) {
            let cid = lupos_core_cpu_cid_to_cid((*t).mm_cid.cid);
            (*t).mm_cid.cid = lupos_core_cid_to_transit_cid(cid);
            (*pcp).cid = (*t).mm_cid.cid;
        }
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_fixup_cpus_to_tasks(mm: *mut mm_struct) {
    // SAFETY: The caller holds the live MM's CID mutex. Each acquired rq
    // lock protects remote per-CPU storage and current-task CID access; unset IDs
    // remain excluded from the transition conversion.
    unsafe {
        let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
        while cpu < nr_cpu_ids as c_int {
            let pcp = lupos_core_mm_cid_pcpu(mm, cpu);
            let rq = lupos_core_cpu_rq(cpu);
            let mut rf = MaybeUninit::<rq_flags>::uninit();
            let rf = rf.as_mut_ptr();
            lupos_core_rq_lock_irq(rq, rf);
            let curr = lupos_core_rq_curr(rq);
            if lupos_core_cid_on_cpu((*pcp).cid) {
                if (*curr).mm == mm && (*curr).mm_cid.active != 0 {
                    mm_cid_transit_to_task(curr, pcp);
                } else {
                    lupos_core_mm_drop_cid_on_cpu(mm, pcp);
                }
            } else if (*curr).mm == mm && (*curr).mm_cid.active != 0 {
                let cid = (*curr).mm_cid.cid;
                // e1d84f5 fix: never convert MM_CID_UNSET to an out-of-range bitmap index.
                if cid != LUPOS_CORE_MM_CID_UNSET && !lupos_core_cid_in_transit(cid) {
                    let cid = lupos_core_cid_to_transit_cid(cid);
                    (*curr).mm_cid.cid = cid;
                    (*pcp).cid = cid;
                }
            }
            lupos_core_rq_unlock_irq(rq, rf);
            cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
        }
        mm_cid_complete_transit(mm, 0);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_transit_to_cpu(t: *mut task_struct, pcp: *mut mm_cid_pcpu) {
    // SAFETY: The caller owns the live task's rq or MM-CID IRQ lock and
    // keeps the CPU's CID slot valid through its task-to-CPU transition.
    unsafe {
        if lupos_core_cid_on_task((*t).mm_cid.cid) {
            (*t).mm_cid.cid = lupos_core_cid_to_transit_cid((*t).mm_cid.cid);
            (*pcp).cid = (*t).mm_cid.cid;
        }
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_fixup_task_to_cpu(t: *mut task_struct, mm: *mut mm_struct) {
    // SAFETY: The caller's MM-CID mutex keeps t/mm membership live; task-rq
    // locking protects the running-task check and remote per-CPU CID transition.
    unsafe {
        let mut rf = MaybeUninit::<rq_flags>::uninit();
        let rf = rf.as_mut_ptr();
        let rq = _task_rq_lock(t, rf);
        if lupos_core_cid_on_task((*t).mm_cid.cid) {
            if lupos_core_rq_curr(lupos_core_task_rq(t)) == t {
                mm_cid_transit_to_cpu(t, lupos_core_mm_cid_pcpu(mm, lupos_core_task_cpu(t)));
            } else {
                lupos_core_mm_unset_cid_on_task(t);
            }
        }
        lupos_core_task_rq_unlock(rq, t, rf);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn mm_cid_fixup_tasks_to_cpus() {
    // SAFETY: Current has a live MM whose CID mutex the caller holds;
    // that mutex protects user-list task lifetimes across per-task rq locking.
    unsafe {
        let current = lupos_core_current();
        let mm = (*current).mm;
        lupos_core_assert_mutex_held(addr_of!((*mm).mm_cid.mutex));
        let mut node = (*mm).mm_cid.user_list.first;
        while !node.is_null() {
            let t = node
                .cast::<u8>()
                .sub(offset_of!(task_struct, mm_cid) + offset_of!(sched_mm_cid, node))
                .cast::<task_struct>();
            if t != current {
                mm_cid_fixup_task_to_cpu(t, mm);
            }
            node = (*node).next;
        }
        mm_cid_complete_transit(mm, LUPOS_CORE_MM_CID_ONCPU);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn sched_mm_cid_add_user(t: *mut task_struct, mm: *mut mm_struct) -> bool {
    // SAFETY: The caller holds the live MM-CID lock and serializes user-list
    // changes with its mutex; t remains live as its CID membership is installed.
    unsafe {
        lupos_core_assert_raw_spin_held(addr_of!((*mm).mm_cid.lock));
        (*t).mm_cid.active = 1;
        lupos_core_hlist_add_head(
            addr_of_mut!((*t).mm_cid.node),
            addr_of_mut!((*mm).mm_cid.user_list),
        );
        (*mm).mm_cid.users = (*mm).mm_cid.users.wrapping_add(1);
        mm_update_max_cids(mm)
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn sched_mm_cid_fork(t: *mut task_struct) {
    // SAFETY: The caller keeps the child/exec task and MM live. CID mutex
    // and IRQ spinlock protect membership, allocation and mode handoff; every
    // closure return reaches the matching unlocks.
    unsafe {
        let mm = (*t).mm;
        if mm.is_null() {
            return;
        }
        lupos_core_warn_once_site_11057((*t).mm_cid.cid != LUPOS_CORE_MM_CID_UNSET);
        lupos_core_mutex_lock(addr_of_mut!((*mm).mm_cid.mutex));
        (|| {
            let mut switched = false;
            let mut percpu = false;
            lupos_core_raw_spin_lock_irq(addr_of_mut!((*mm).mm_cid.lock));
            (|| {
                let pcp = lupos_core_mm_cid_this_cpu(mm);
                if (*mm).mm_cid.users == 0 {
                    sched_mm_cid_add_user(t, mm);
                    (*t).mm_cid.cid = lupos_core_mm_get_cid(mm);
                    (*pcp).cid = (*t).mm_cid.cid;
                    return;
                }
                if !sched_mm_cid_add_user(t, mm) {
                    if !lupos_core_cid_on_cpu((*mm).mm_cid.mode) {
                        (*t).mm_cid.cid = lupos_core_mm_get_cid(mm);
                    }
                    return;
                }
                switched = true;
                percpu = lupos_core_cid_on_cpu((*mm).mm_cid.mode);
                if !percpu {
                    mm_cid_transit_to_task(lupos_core_current(), pcp);
                } else {
                    mm_cid_transit_to_cpu(lupos_core_current(), pcp);
                }
            })();
            lupos_core_raw_spin_unlock_irq(addr_of_mut!((*mm).mm_cid.lock));
            if !switched {
                return;
            }
            if percpu {
                mm_cid_fixup_tasks_to_cpus();
            } else {
                mm_cid_fixup_cpus_to_tasks(mm);
                (*t).mm_cid.cid = lupos_core_mm_get_cid(mm);
            }
        })();
        lupos_core_mutex_unlock(addr_of_mut!((*mm).mm_cid.mutex));
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn sched_mm_cid_remove_user(t: *mut task_struct) -> bool {
    // SAFETY: The caller holds the task MM's CID lock and membership mutex;
    // t's live membership owns the CID/list reference being removed.
    unsafe {
        let mm = (*t).mm;
        lupos_core_assert_raw_spin_held(addr_of!((*mm).mm_cid.lock));
        (*t).mm_cid.active = 0;
        (*t).mm_cid.cid = lupos_core_cid_from_transit_cid((*t).mm_cid.cid);
        lupos_core_mm_unset_cid_on_task(t);
        lupos_core_hlist_del_init(addr_of_mut!((*t).mm_cid.node));
        (*mm).mm_cid.users = (*mm).mm_cid.users.wrapping_sub(1);
        mm_update_max_cids(mm)
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe fn __sched_mm_cid_exit(t: *mut task_struct) -> bool {
    // SAFETY: The caller owns this live task's MM-CID mutex/IRQ lock and
    // registered membership; native mode/current-MM invariants gate fixup.
    unsafe {
        let mm = (*t).mm;
        if !sched_mm_cid_remove_user(t) {
            return false;
        }
        if lupos_core_warn_once_site_11115(lupos_core_cid_on_cpu((*mm).mm_cid.mode)) {
            return false;
        }
        if lupos_core_warn_once_site_11122((*lupos_core_current()).mm != mm) {
            return false;
        }
        true
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
#[no_mangle]
pub unsafe extern "C" fn sched_mm_cid_exit(t: *mut task_struct) {
    // SAFETY: The exiting/exec caller keeps t/mm live. CID locks serialize
    // removal and mode fixup; last-user work synchronization occurs after both
    // locks are released and prevents callbacks outliving the MM.
    unsafe {
        let mm = (*t).mm;
        if mm.is_null() || (*t).mm_cid.active == 0 {
            return;
        }
        lupos_core_mutex_lock(addr_of_mut!((*mm).mm_cid.mutex));
        if (*mm).mm_cid.users > 1 {
            lupos_core_raw_spin_lock_irq(addr_of_mut!((*mm).mm_cid.lock));
            let switched = __sched_mm_cid_exit(t);
            if switched {
                lupos_core_mm_drop_cid_on_cpu(mm, lupos_core_mm_cid_this_cpu(mm));
            }
            lupos_core_raw_spin_unlock_irq(addr_of_mut!((*mm).mm_cid.lock));
            if switched {
                mm_cid_fixup_cpus_to_tasks(mm);
            }
            lupos_core_mutex_unlock(addr_of_mut!((*mm).mm_cid.mutex));
            return;
        }
        lupos_core_raw_spin_lock_irq(addr_of_mut!((*mm).mm_cid.lock));
        if t == lupos_core_current() {
            mm_cid_transit_to_task(t, lupos_core_mm_cid_this_cpu(mm));
        }
        sched_mm_cid_remove_user(t);
        lupos_core_raw_spin_unlock_irq(addr_of_mut!((*mm).mm_cid.lock));
        lupos_core_mutex_unlock(addr_of_mut!((*mm).mm_cid.mutex));
        // Last-user synchronization is outside both locks; no new work can queue.
        irq_work_sync(addr_of_mut!((*mm).mm_cid.irq_work));
        cancel_work_sync(addr_of_mut!((*mm).mm_cid.work));
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
#[no_mangle]
pub unsafe extern "C" fn sched_mm_cid_before_execve(t: *mut task_struct) {
    // SAFETY: The exec caller keeps t and its old MM live through native
    // CID membership removal and any pending-work synchronization.
    unsafe {
        sched_mm_cid_exit(t);
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
#[no_mangle]
pub unsafe extern "C" fn sched_mm_cid_after_execve(t: *mut task_struct) {
    // SAFETY: The exec caller keeps t and its installed MM live through
    // reactivation of the task's native CID membership.
    unsafe {
        if !(*t).mm.is_null() {
            sched_mm_cid_fork(t);
        }
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe extern "C" fn mm_cid_work_fn(work: *mut work_struct) {
    // SAFETY: The native workqueue supplies work embedded in a live MM;
    // last-user exit synchronizes it. CID mutex/IRQ locking serialize deferred fixup.
    unsafe {
        let mm = work
            .cast::<u8>()
            .sub(offset_of!(mm_struct, mm_cid) + offset_of!(mm_mm_cid, work))
            .cast::<mm_struct>();
        lupos_core_mutex_lock(addr_of_mut!((*mm).mm_cid.mutex));
        if (*mm).mm_cid.users != 0 {
            lupos_core_raw_spin_lock_irq(addr_of_mut!((*mm).mm_cid.lock));
            let fixup = (*mm).mm_cid.update_deferred != 0
                && mm_update_max_cids(mm)
                && !lupos_core_warn_once_site_11203(lupos_core_cid_on_cpu((*mm).mm_cid.mode));
            lupos_core_raw_spin_unlock_irq(addr_of_mut!((*mm).mm_cid.lock));
            if fixup {
                mm_cid_fixup_cpus_to_tasks(mm);
            }
        }
        lupos_core_mutex_unlock(addr_of_mut!((*mm).mm_cid.mutex));
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
unsafe extern "C" fn mm_cid_irq_work(work: *mut irq_work) {
    // SAFETY: The native IRQ-work item is embedded in a live MM; its last
    // user synchronizes IRQ and regular work before lifetime can end.
    unsafe {
        let mm = work
            .cast::<u8>()
            .sub(offset_of!(mm_struct, mm_cid) + offset_of!(mm_mm_cid, irq_work))
            .cast::<mm_struct>();
        lupos_core_schedule_work(addr_of_mut!((*mm).mm_cid.work));
    }
}
#[cfg(CONFIG_SCHED_MM_CID)]
#[no_mangle]
pub unsafe extern "C" fn mm_init_cid(mm: *mut mm_struct, p: *mut task_struct) {
    // SAFETY: The MM-creation caller exclusively owns mm and keeps p live;
    // configured native CID storage and bitmap allocation precede initialization.
    unsafe {
        let mc = addr_of_mut!((*mm).mm_cid);
        (*mc).max_cids = 0;
        (*mc).mode = 0;
        (*mc).nr_cpus_allowed = (*p).nr_cpus_allowed as c_uint;
        (*mc).users = 0;
        (*mc).pcpu_thrs = 0;
        (*mc).update_deferred = 0;
        lupos_core_raw_spin_lock_init_mmcid(addr_of_mut!((*mc).lock));
        lupos_core_mutex_init(addr_of_mut!((*mc).mutex));
        lupos_core_irq_work_init_hard(addr_of_mut!((*mc).irq_work), Some(mm_cid_irq_work));
        lupos_core_init_work(addr_of_mut!((*mc).work), Some(mm_cid_work_fn));
        lupos_core_init_hlist_head(addr_of_mut!((*mc).user_list));
        lupos_core_cpumask_copy(lupos_core_mm_cpus_allowed(mm), addr_of!((*p).cpus_mask));
        lupos_core_bitmap_zero(lupos_core_mm_cidmask(mm), lupos_core_num_possible_cpus());
    }
}
#[cfg(not(CONFIG_SCHED_MM_CID))]
unsafe fn mm_update_cpus_allowed(_mm: *mut mm_struct, _affmsk: *const cpumask) {}
#[cfg(not(CONFIG_SCHED_MM_CID))]
unsafe fn sched_mm_cid_fork(_t: *mut task_struct) {}
