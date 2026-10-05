// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn __task_prio(p: *const task_struct) -> c_int {
    // SAFETY: The caller keeps p and any deadline server live under the
    // scheduler/core lock while deriving its native class priority.
    unsafe {
        if (*p).sched_class == addr_of!(stop_sched_class) {
            return -2;
        }
        if !(*p).dl_server.is_null() {
            return -1;
        }
        if lupos_core_rt_or_dl_prio((*p).prio) {
            return (*p).prio;
        }
        if (*p).sched_class == addr_of!(idle_sched_class) {
            return LUPOS_CORE_MAX_RT_PRIO + LUPOS_CORE_NICE_WIDTH;
        }
        if lupos_core_task_on_scx(p) {
            return LUPOS_CORE_MAX_RT_PRIO + LUPOS_CORE_MAX_NICE + 1;
        }
        LUPOS_CORE_MAX_RT_PRIO + LUPOS_CORE_MAX_NICE
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn prio_less(a: *const task_struct, b: *const task_struct, in_fi: bool) -> bool {
    // SAFETY: Both tasks and their deadline servers stay live under the
    // core/rq lock; class-specific comparators receive native ordering state.
    unsafe {
        let pa = __task_prio(a);
        let pb = __task_prio(b);
        if pa.wrapping_neg() < pb.wrapping_neg() {
            return true;
        }
        if pb.wrapping_neg() < pa.wrapping_neg() {
            return false;
        }
        if pa == -1 {
            let a_dl = if !(*a).dl_server.is_null() {
                (*a).dl_server.cast_const()
            } else {
                addr_of!((*a).dl)
            };
            let b_dl = if !(*b).dl_server.is_null() {
                (*b).dl_server.cast_const()
            } else {
                addr_of!((*b).dl)
            };
            return !lupos_core_dl_time_before((*a_dl).deadline, (*b_dl).deadline);
        }
        if pa == LUPOS_CORE_MAX_RT_PRIO + LUPOS_CORE_MAX_NICE {
            return cfs_prio_less(a, b, in_fi);
        }
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        if pa == LUPOS_CORE_MAX_RT_PRIO + LUPOS_CORE_MAX_NICE + 1 {
            return scx_prio_less(a, b, in_fi);
        }
        false
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn __sched_core_less(a: *const task_struct, b: *const task_struct) -> bool {
    // SAFETY: The caller holds the relevant core/rq lock and keeps both
    // tasks live, protecting cookies and the force-idle priority comparison.
    unsafe {
        if (*a).core_cookie < (*b).core_cookie {
            return true;
        }
        if (*a).core_cookie > (*b).core_cookie {
            return false;
        }
        prio_less(
            b,
            a,
            (*(*lupos_core_task_rq(a)).core).core_forceidle_count != 0,
        )
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn node_2_sc(node: *const rb_node) -> *mut task_struct {
    // SAFETY: node points to the embedded core_node of a live task; its
    // container address uses the configured native task layout.
    unsafe {
        node.cast::<u8>()
            .sub(offset_of!(task_struct, core_node))
            .cast_mut()
            .cast()
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe extern "C" fn rb_sched_core_less(a: *mut rb_node, b: *const rb_node) -> bool {
    // SAFETY: The native tree callback receives embedded nodes of live
    // tasks protected by the rq/core lock throughout comparison.
    unsafe {
        __sched_core_less(node_2_sc(a), node_2_sc(b))
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe extern "C" fn rb_sched_core_cmp(key: *const c_void, node: *const rb_node) -> c_int {
    // SAFETY: node belongs to a live rq-locked task. key is an encoded
    // cookie used only as an integer, never dereferenced as an object pointer.
    unsafe {
        let p = node_2_sc(node);
        let cookie = key as c_ulong;
        if cookie < (*p).core_cookie {
            -1
        } else if cookie > (*p).core_cookie {
            1
        } else {
            0
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn sched_core_enqueue(rq: *mut rq, p: *mut task_struct) {
    // SAFETY: The caller holds rq/core locking and owns p's enqueue
    // transition; its embedded node is inserted only for a nonzero cookie.
    unsafe {
        if (*p).se.sched_delayed != 0 {
            return;
        }
        (*(*rq).core).core_task_seq = (*(*rq).core).core_task_seq.wrapping_add(1);
        if (*p).core_cookie == 0 {
            return;
        }
        lupos_core_rb_add(
            addr_of_mut!((*p).core_node),
            addr_of_mut!((*rq).core_tree),
            Some(rb_sched_core_less),
        );
    }
}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn sched_core_enqueue(_rq: *mut rq, _p: *mut task_struct) {}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn sched_core_dequeue(rq: *mut rq, p: *mut task_struct, flags: c_int) {
    // SAFETY: The caller holds rq/core locking and keeps p live while
    // removing its embedded node and updating force-idle scheduling state.
    unsafe {
        if (*p).se.sched_delayed != 0 {
            return;
        }
        (*(*rq).core).core_task_seq = (*(*rq).core).core_task_seq.wrapping_add(1);
        if lupos_core_sched_core_enqueued(p) {
            rb_erase(addr_of_mut!((*p).core_node), addr_of_mut!((*rq).core_tree));
            lupos_core_rb_clear_node(addr_of_mut!((*p).core_node));
        }
        if flags & LUPOS_CORE_DEQUEUE_SAVE == 0
            && (*rq).nr_running == 1
            && (*(*rq).core).core_forceidle_count != 0
            && lupos_core_rq_curr(rq) == (*rq).idle
        {
            resched_curr(rq);
        }
    }
}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn sched_core_dequeue(_rq: *mut rq, _p: *mut task_struct, _flags: c_int) {}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_task_is_throttled(p: *mut task_struct, cpu: c_int) -> c_int {
    // SAFETY: The caller keeps p/class state live under rq/core locking
    // and supplies a valid CPU for the optional native throttle callback.
    unsafe {
        if let Some(throttled) = (*(*p).sched_class).task_is_throttled {
            throttled(p, cpu)
        } else {
            0
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_next(mut p: *mut task_struct, cookie: c_ulong) -> *mut task_struct {
    // SAFETY: The caller holds the task's rq/core lock; tree nodes and
    // their tasks remain live during same-cookie and throttling traversal.
    unsafe {
        let mut node = addr_of_mut!((*p).core_node);
        let cpu = lupos_core_task_cpu(p);
        loop {
            node = rb_next(node);
            if node.is_null() {
                return null_mut();
            }
            p = node_2_sc(node);
            if (*p).core_cookie != cookie {
                return null_mut();
            }
            if sched_task_is_throttled(p, cpu) == 0 {
                return p;
            }
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_find(rq: *mut rq, cookie: c_ulong) -> *mut task_struct {
    // SAFETY: The caller holds the live rq/core lock, protecting tree
    // lookup and task lifetime through subsequent throttle filtering.
    unsafe {
        let node = lupos_core_rb_find_first(
            cookie as *const c_void,
            addr_of!((*rq).core_tree),
            Some(rb_sched_core_cmp),
        );
        if node.is_null() {
            return null_mut();
        }
        let p = node_2_sc(node);
        if sched_task_is_throttled(p, (*rq).cpu) == 0 {
            return p;
        }
        sched_core_next(p, cookie)
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_lock(cpu: c_int, flags: *mut c_ulong) {
    // SAFETY: The caller stabilizes the valid CPU's sibling topology and
    // provides writable flags; native IRQ exclusion spans nested sibling locks.
    unsafe {
        let mask = lupos_core_cpu_smt_mask(cpu);
        *flags = lupos_core_local_irq_save();
        let mut t = lupos_core_cpu_next(-1, mask);
        let mut depth = 0;
        while t < lupos_core_nr_cpu_ids() {
            lupos_core_raw_spin_lock_nested(addr_of_mut!((*lupos_core_cpu_rq(t)).__lock), depth);
            depth += 1;
            t = lupos_core_cpu_next(t, mask);
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_unlock(cpu: c_int, flags: *const c_ulong) {
    // SAFETY: The caller holds the sibling locks acquired by sched_core_lock
    // with stable topology and the matching saved IRQ flags.
    unsafe {
        let mask = lupos_core_cpu_smt_mask(cpu);
        let mut t = lupos_core_cpu_next(-1, mask);
        while t < lupos_core_nr_cpu_ids() {
            lupos_core_raw_spin_unlock(addr_of_mut!((*lupos_core_cpu_rq(t)).__lock));
            t = lupos_core_cpu_next(t, mask);
        }
        lupos_core_local_irq_restore(*flags);
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn __sched_core_flip(enabled: bool) {
    // SAFETY: The caller holds the core-mode mutex. CPU-hotplug protection
    // stabilizes topology; sibling locks and the in-flight wait serialize rebinding.
    unsafe {
        lupos_core_cpus_read_lock();
        let mask = addr_of_mut!(lupos_core_sched_core_mask);
        lupos_core_cpumask_copy(mask, lupos_core_cpu_online_mask());
        let mut cpu = lupos_core_cpu_next(-1, mask);
        while cpu < lupos_core_nr_cpu_ids() {
            let smt = lupos_core_cpu_smt_mask(cpu);
            let mut flags = 0;
            sched_core_lock(cpu, addr_of_mut!(flags));
            while (*(*lupos_core_cpu_rq(cpu)).core).core_pick_in_flight != 0 {
                sched_core_unlock(cpu, addr_of!(flags));
                lupos_core_cpu_relax();
                sched_core_lock(cpu, addr_of_mut!(flags));
            }
            let mut t = lupos_core_cpu_next(-1, smt);
            while t < lupos_core_nr_cpu_ids() {
                (*lupos_core_cpu_rq(t)).core_enabled = enabled as c_uint;
                t = lupos_core_cpu_next(t, smt);
            }
            (*(*lupos_core_cpu_rq(cpu)).core).core_forceidle_start = 0;
            sched_core_unlock(cpu, addr_of!(flags));
            lupos_core_cpumask_andnot(mask, mask, smt);
            cpu = lupos_core_cpu_next(cpu, mask);
        }
        let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
        while cpu < lupos_core_nr_cpu_ids() {
            if !lupos_core_cpumask_test_cpu(cpu, lupos_core_cpu_online_mask()) {
                (*lupos_core_cpu_rq(cpu)).core_enabled = enabled as c_uint;
            }
            cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
        }
        lupos_core_cpus_read_unlock();
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_assert_empty() {
    // SAFETY: The caller serializes the core-mode transition; native
    // possible-CPU iteration selects permanent runqueue tree roots.
    unsafe {
        let mut cpu = lupos_core_cpu_next(-1, lupos_core_cpu_possible_mask());
        while cpu < lupos_core_nr_cpu_ids() {
            lupos_core_warn_core_tree(!(*lupos_core_cpu_rq(cpu)).core_tree.rb_node.is_null());
            cpu = lupos_core_cpu_next(cpu, lupos_core_cpu_possible_mask());
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn __sched_core_enable() {
    // SAFETY: The caller holds the core-mode mutex. Native key publication
    // and an RCU grace period precede sibling lock-mode rebinding.
    unsafe {
        lupos_core_core_key_enable();
        synchronize_rcu();
        __sched_core_flip(true);
        sched_core_assert_empty();
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn __sched_core_disable() {
    // SAFETY: The caller holds the core-mode mutex with no remaining
    // users; sibling lock mode changes before the native key is disabled.
    unsafe {
        sched_core_assert_empty();
        __sched_core_flip(false);
        lupos_core_core_key_disable();
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn sched_core_get() {
    // SAFETY: The native atomic fast path or core mutex serializes mode
    // reference acquisition against the deferred last-reference release.
    unsafe {
        if lupos_core_atomic_inc_not_zero(addr_of_mut!(lupos_core_sched_core_count)) {
            return;
        }
        lupos_core_mutex_lock(addr_of_mut!(lupos_core_sched_core_mutex));
        if lupos_core_atomic_read(addr_of!(lupos_core_sched_core_count)) == 0 {
            __sched_core_enable();
        }
        lupos_core_smp_mb_before_atomic();
        lupos_core_atomic_inc(addr_of_mut!(lupos_core_sched_core_count));
        lupos_core_mutex_unlock(addr_of_mut!(lupos_core_sched_core_mutex));
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn lupos_core_core_put_workfn(_work: *mut work_struct) {
    // SAFETY: The work item owns the deferred core reference. Native
    // atomic-dec/mutex acquisition serializes the last disable transition.
    unsafe {
        if atomic_dec_and_mutex_lock(
            addr_of_mut!(lupos_core_sched_core_count),
            addr_of_mut!(lupos_core_sched_core_mutex),
        ) != 0
        {
            __sched_core_disable();
            lupos_core_mutex_unlock(addr_of_mut!(lupos_core_sched_core_mutex));
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
#[no_mangle]
pub unsafe extern "C" fn sched_core_put() {
    // SAFETY: The caller releases a held core-mode reference. Native
    // atomic/workqueue rules serialize the potentially deferred final decrement.
    unsafe {
        if !lupos_core_atomic_add_unless(addr_of_mut!(lupos_core_sched_core_count), -1, 1) {
            lupos_core_schedule_work(addr_of_mut!(lupos_core_sched_core_put_work));
        }
    }
}
