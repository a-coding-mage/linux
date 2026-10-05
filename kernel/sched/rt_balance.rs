// SPDX-License-Identifier: GPL-2.0
// rt.c push/pull, hotplug, migration retries and IRQ-work root-domain lifetime.
const RT_MAX_TRIES: c_int = 3; // rt.c:1751, not a guessed native constant.
/// Selects a task from an RT pushable list.
///
/// # Safety
/// The runqueue lock must protect the initialized pushable list and every task on it.
/// List nodes must be actual embedded pushable_tasks nodes, and any requested CPU must
/// be a valid native CPU index. The returned task remains protected only while that
/// lifetime/lock protocol is retained.
unsafe fn pick_highest_pushable_task(rq: *mut b::rq, cpu: c_int) -> *mut b::task_struct {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let head = addr_of_mut!((*rq).rt.pushable_tasks);
        if !has_pushable_tasks(rq) { return null_mut(); }
        let sentinel = addr_of_mut!((*head).node_list);
        let mut node = (*sentinel).next;
        while node != sentinel {
            let pn = container_of!(node, b::plist_node, node_list);
            let p = container_of!(pn, b::task_struct, pushable_tasks);
            if b::rust_rt_task_is_pushable(rq, p, cpu) { return p; }
            node = (*node).next;
        }
        null_mut()
    }
}
/// Selects a candidate CPU under the native RT placement protocol.
///
/// # Safety
/// The task must remain live and its affinity and scheduler state must be protected as
/// required by native wakeup or migration selection. CPU IDs must identify initialized
/// runqueues and the current CPU must remain pinned while the per-CPU temporary mask
/// is used. RCU protects topology lifetime; optimistic remote-field reads still
/// require native/Rust memory-model qualification before admission.
unsafe fn find_lowest_rq(task: *mut b::task_struct) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let lowest_mask = b::rust_rt_local_cpu_mask();
        let mut this_cpu = b::rust_rt_smp_processor_id();
        let mut cpu = b::rust_rt_task_cpu(task);
        if lowest_mask.is_null() { return -1; }
        if (*task).nr_cpus_allowed == 1 { return -1; }
        let ret = if b::rust_rt_sched_asym_cpucap_active() {
            b::cpupri_find_fitness(addr_of_mut!((*(*b::rust_rt_task_rq(task)).rd).cpupri), task, lowest_mask, Some(rt_task_fits_capacity))
        } else {
            b::cpupri_find(addr_of_mut!((*(*b::rust_rt_task_rq(task)).rd).cpupri), task, lowest_mask)
        };
        if ret == 0 { return -1; }
        if b::rust_rt_cpumask_test_cpu(cpu as _, lowest_mask) { return cpu; }
        if !b::rust_rt_cpumask_test_cpu(this_cpu as _, lowest_mask) { this_cpu = -1; }
        b::rust_rt_rcu_read_lock();
        let mut sd = b::rust_rt_first_domain(cpu);
        while !sd.is_null() {
            if (*sd).flags & b::RUST_RT_SD_WAKE_AFFINE as c_int != 0 {
                if this_cpu != -1 && b::rust_rt_cpumask_test_cpu(this_cpu as _, b::rust_rt_sched_domain_span(sd)) {
                    b::rust_rt_rcu_read_unlock();
                    return this_cpu;
                }
                let best_cpu = b::rust_rt_cpumask_any_and_distribute(lowest_mask, b::rust_rt_sched_domain_span(sd)) as c_int;
                if (best_cpu as c_uint) < b::rust_rt_nr_cpu_ids() { b::rust_rt_rcu_read_unlock(); return best_cpu; }
            }
            sd = (*sd).parent;
        }
        b::rust_rt_rcu_read_unlock();
        if this_cpu != -1 { return this_cpu; }
        cpu = b::rust_rt_cpumask_any_distribute(lowest_mask) as c_int;
        if (cpu as c_uint) < b::rust_rt_nr_cpu_ids() { return cpu; }
        -1
    }
}
/// Selects a task from an RT pushable list.
///
/// # Safety
/// The runqueue lock must protect the initialized pushable list and every task on it.
/// List nodes must be actual embedded pushable_tasks nodes, and any requested CPU must
/// be a valid native CPU index. The returned task remains protected only while that
/// lifetime/lock protocol is retained.
unsafe fn pick_next_pushable_task(rq: *mut b::rq) -> *mut b::task_struct {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let head = addr_of_mut!((*rq).rt.pushable_tasks);
        if !has_pushable_tasks(rq) { return null_mut(); }
        let sentinel = addr_of_mut!((*head).node_list);
        let mut node = (*sentinel).next;
        let mut p = null_mut();
        while node != sentinel {
            let pn = container_of!(node, b::plist_node, node_list);
            let i = container_of!(pn, b::task_struct, pushable_tasks);
            if !b::rust_rt_task_on_cpu(rq, i) { p = i; break; }
            node = (*node).next;
        }
        if p.is_null() { return null_mut(); }
        b::rust_rt_bug_1885((*rq).cpu != b::rust_rt_task_cpu(p));
        b::rust_rt_bug_1886(b::rust_rt_task_current(rq, p));
        b::rust_rt_bug_1887(b::rust_rt_task_current_donor(rq, p));
        b::rust_rt_bug_1888((*p).nr_cpus_allowed <= 1);
        b::rust_rt_bug_1890(!b::rust_rt_task_on_rq_queued(p));
        b::rust_rt_bug_1891(!b::rust_rt_rt_task(p));
        p
    }
}
#[no_mangle]
/// Finds and locks a lower-priority destination runqueue.
///
/// # Safety
/// The source rq lock must be held in native balancing context and the candidate task
/// must remain referenced across possible lock drops. The caller must honor the
/// returned destination-lock ownership and revalidation rules; rq and topology
/// lifetime must survive each retry.
pub unsafe extern "C" fn find_lock_lowest_rq(task: *mut b::task_struct, rq: *mut b::rq) -> *mut b::rq {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut lowest_rq = null_mut();
        for _tries in 0..RT_MAX_TRIES {
            let cpu = find_lowest_rq(task);
            if cpu == -1 || cpu == (*rq).cpu { break; }
            lowest_rq = b::rust_rt_cpu_rq(cpu);
            if (*lowest_rq).rt.highest_prio.curr <= (*task).prio { lowest_rq = null_mut(); break; }
            if b::rust_rt_double_lock_balance(rq, lowest_rq) != 0 {
                if b::rust_rt_is_migration_disabled(task) ||
                    !b::rust_rt_cpumask_test_cpu((*lowest_rq).cpu as _, addr_of!((*task).cpus_mask)) ||
                    task != pick_next_pushable_task(rq) {
                    b::rust_rt_double_unlock_balance(rq, lowest_rq);
                    lowest_rq = null_mut();
                    break;
                }
            }
            if (*lowest_rq).rt.highest_prio.curr > (*task).prio { break; }
            b::rust_rt_double_unlock_balance(rq, lowest_rq);
            lowest_rq = null_mut();
        }
        lowest_rq
    }
}
/// Migrates eligible RT work between native runqueues.
///
/// # Safety
/// The source/current rq lock must be held with the scheduler interrupt state required
/// by native balancing. Runqueues and their root domains must remain live across lock
/// drops. Task references, both-rq lock acquisition, and stopper work ownership must
/// follow the native migration protocol retained in this body.
unsafe fn push_rt_task(rq: *mut b::rq, pull: bool) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !(*rq).rt.overloaded { return 0; }
        let mut next_task = pick_next_pushable_task(rq);
        if next_task.is_null() { return 0; }
        loop {
            if (*next_task).prio < (*(*rq).donor).prio { b::resched_curr(rq); return 0; }
            if b::rust_rt_is_migration_disabled(next_task) {
                if !pull || b::rust_rt_rq_push_busy(rq) { return 0; }
                if (*(*rq).donor).sched_class != addr_of!(b::rt_sched_class) { return 0; }
                let cpu = find_lowest_rq((*rq).curr);
                if cpu == -1 || cpu == (*rq).cpu { return 0; }
                let push_task = b::rust_rt_get_push_task(rq);
                if !push_task.is_null() {
                    b::rust_rt_preempt_disable();
                    b::rust_rt_raw_spin_rq_unlock(rq);
                    b::rust_rt_stop_one_cpu_nowait((*rq).cpu as _, Some(b::push_cpu_stop), push_task.cast(), addr_of_mut!((*rq).push_work));
                    b::rust_rt_preempt_enable();
                    b::rust_rt_raw_spin_rq_lock(rq);
                }
                return 0;
            }
            if b::rust_rt_warn_2026(next_task == (*rq).curr) { return 0; }
            b::rust_rt_get_task_struct(next_task);
            let lowest_rq = find_lock_lowest_rq(next_task, rq);
            if lowest_rq.is_null() {
                let task = pick_next_pushable_task(rq);
                if task == next_task || task.is_null() {
                    b::rust_rt_put_task_struct(next_task);
                    return 0;
                }
                b::rust_rt_put_task_struct(next_task);
                next_task = task;
                continue;
            }
            b::rust_rt_move_queued_task_locked(rq, lowest_rq, next_task);
            b::resched_curr(lowest_rq);
            b::rust_rt_double_unlock_balance(rq, lowest_rq);
            b::rust_rt_put_task_struct(next_task);
            return 1;
        }
    }
}
/// Migrates eligible RT work between native runqueues.
///
/// # Safety
/// The source/current rq lock must be held with the scheduler interrupt state required
/// by native balancing. Runqueues and their root domains must remain live across lock
/// drops. Task references, both-rq lock acquisition, and stopper work ownership must
/// follow the native migration protocol retained in this body.
unsafe extern "C" fn push_rt_tasks(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        while push_rt_task(rq, false) != 0 {}
    }
}
#[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
/// Advances the root-domain RT push iterator.
///
/// # Safety
/// The root domain must remain live with rto_lock held. The current CPU must remain
/// pinned, and the native atomic and overload-mask publication protocol must protect
/// concurrent overload updates.
unsafe fn rto_next_cpu(rd: *mut b::root_domain) -> c_int {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let this_cpu = b::rust_rt_smp_processor_id();
        loop {
            let cpu = b::rust_rt_cpumask_next((*rd).rto_cpu, b::rust_rt_rd_rto_mask(rd)) as c_int;
            (*rd).rto_cpu = cpu;
            if cpu == this_cpu { continue; }
            if (cpu as c_uint) < b::rust_rt_nr_cpu_ids() { return cpu; }
            (*rd).rto_cpu = -1;
            let next = b::rust_rt_atomic_read_acquire(addr_of!((*rd).rto_loop_next));
            if (*rd).rto_loop == next { break; }
            (*rd).rto_loop = next;
        }
        -1
    }
}
#[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
/// Accesses the native atomic RT push-start lock.
///
/// # Safety
/// The pointer must identify a live, aligned native atomic_t participating in the RT
/// push-start protocol. Unlock requires ownership established by the corresponding
/// successful native acquire operation.
unsafe fn rto_start_trylock(v: *mut b::atomic_t) -> bool {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_atomic_cmpxchg_acquire(v, 0, 1) == 0
    }
}
#[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
/// Accesses the native atomic RT push-start lock.
///
/// # Safety
/// The pointer must identify a live, aligned native atomic_t participating in the RT
/// push-start protocol. Unlock requires ownership established by the corresponding
/// successful native acquire operation.
unsafe fn rto_start_unlock(v: *mut b::atomic_t) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        b::rust_rt_atomic_set_release(v, 0);
    }
}
#[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
/// Starts or extends native IRQ-work RT push traversal.
///
/// # Safety
/// The runqueue and root domain must remain live under the scheduler rq protocol, with
/// the current CPU pinned. The initialized IRQ work and root-domain reference count
/// must remain under the native single-iterator ownership protocol.
unsafe fn tell_cpu_to_push(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let mut cpu = -1;
        b::rust_rt_atomic_inc(addr_of_mut!((*(*rq).rd).rto_loop_next));
        if !rto_start_trylock(addr_of_mut!((*(*rq).rd).rto_loop_start)) { return; }
        b::rust_rt_raw_spin_lock(addr_of_mut!((*(*rq).rd).rto_lock));
        if (*(*rq).rd).rto_cpu < 0 { cpu = rto_next_cpu((*rq).rd); }
        b::rust_rt_raw_spin_unlock(addr_of_mut!((*(*rq).rd).rto_lock));
        rto_start_unlock(addr_of_mut!((*(*rq).rd).rto_loop_start));
        if cpu >= 0 {
            b::sched_get_rd((*rq).rd);
            b::irq_work_queue_on(addr_of_mut!((*(*rq).rd).rto_push_work), cpu);
        }
    }
}
#[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
#[no_mangle]
/// Runs one hop of the RT push IRQ-work traversal.
///
/// # Safety
/// This must be invoked by the native hardirq-work mechanism with work embedded in a
/// live root_domain. The pending traversal must own the sched_get_rd reference that
/// this callback either passes onward or releases, and the current CPU runqueue must
/// be initialized.
pub unsafe extern "C" fn rto_push_irq_work_func(work: *mut b::irq_work) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let rd = container_of!(work, b::root_domain, rto_push_work);
        let rq = b::rust_rt_this_rq();
        if has_pushable_tasks(rq) {
            b::rust_rt_raw_spin_rq_lock(rq);
            while push_rt_task(rq, true) != 0 {}
            b::rust_rt_raw_spin_rq_unlock(rq);
        }
        b::rust_rt_raw_spin_lock(addr_of_mut!((*rd).rto_lock));
        let cpu = rto_next_cpu(rd);
        b::rust_rt_raw_spin_unlock(addr_of_mut!((*rd).rto_lock));
        if cpu < 0 { b::sched_put_rd(rd); return; }
        b::irq_work_queue_on(addr_of_mut!((*rd).rto_push_work), cpu);
    }
}
/// Migrates eligible RT work between native runqueues.
///
/// # Safety
/// The source/current rq lock must be held with the scheduler interrupt state required
/// by native balancing. Runqueues and their root domains must remain live across lock
/// drops. Task references, both-rq lock acquisition, and stopper work ownership must
/// follow the native migration protocol retained in this body.
unsafe extern "C" fn pull_rt_task(this_rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let this_cpu = (*this_rq).cpu;
        let mut resched = false;
        let rt_overload_count = rt_overloaded(this_rq);
        if rt_overload_count == 0 { return; }
        b::rust_rt_smp_rmb();
        if rt_overload_count == 1 && b::rust_rt_cpumask_test_cpu((*this_rq).cpu as _, b::rust_rt_rd_rto_mask((*this_rq).rd)) { return; }
        #[cfg(all(CONFIG_IRQ_WORK, CONFIG_SMP))]
        if b::rust_rt_feat_RT_PUSH_IPI() { tell_cpu_to_push(this_rq); return; }
        each_cpu(b::rust_rt_rd_rto_mask((*this_rq).rd), |cpu| {
            if this_cpu == cpu { return; }
            let src_rq = b::rust_rt_cpu_rq(cpu);
            if (*src_rq).rt.highest_prio.next >= (*this_rq).rt.highest_prio.curr { return; }
            let mut push_task = null_mut();
            b::rust_rt_double_lock_balance(this_rq, src_rq);
            let p = pick_highest_pushable_task(src_rq, this_cpu);
            if !p.is_null() && (*p).prio < (*this_rq).rt.highest_prio.curr {
                b::rust_rt_warn_2326(p == (*src_rq).curr);
                b::rust_rt_warn_2327(!b::rust_rt_task_on_rq_queued(p));
                // C's skip label releases both locks even on the waking-task case.
                if (*p).prio >= (*(*src_rq).donor).prio {
                    if b::rust_rt_is_migration_disabled(p) { push_task = b::rust_rt_get_push_task(src_rq); }
                    else { b::rust_rt_move_queued_task_locked(src_rq, this_rq, p); resched = true; }
                }
            }
            b::rust_rt_double_unlock_balance(this_rq, src_rq);
            if !push_task.is_null() {
                b::rust_rt_preempt_disable();
                b::rust_rt_raw_spin_rq_unlock(this_rq);
                b::rust_rt_stop_one_cpu_nowait((*src_rq).cpu as _, Some(b::push_cpu_stop), push_task.cast(), addr_of_mut!((*src_rq).push_work));
                b::rust_rt_preempt_enable();
                b::rust_rt_raw_spin_rq_lock(this_rq);
            }
        });
        if resched { b::resched_curr(this_rq); }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn task_woken_rt(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        let need_to_push = !b::rust_rt_task_on_cpu(rq, p) && !b::rust_rt_test_tsk_need_resched((*rq).curr)
            && (*p).nr_cpus_allowed > 1 && (b::rust_rt_dl_task((*rq).donor) || b::rust_rt_rt_task((*rq).donor))
            && ((*(*rq).curr).nr_cpus_allowed < 2 || (*(*rq).donor).prio <= (*p).prio);
        if need_to_push { push_rt_tasks(rq); }
    }
}
#[no_mangle]
/// Updates RT state during native runqueue hotplug.
///
/// # Safety
/// The initialized runqueue must be locked and retained by the native hotplug
/// protocol, with its root domain and task-group hierarchy alive. Overload membership
/// and cpupri state must match the corresponding online/offline transition.
pub unsafe extern "C" fn rq_online_rt(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if (*rq).rt.overloaded { rt_set_overload(rq); }
        __enable_runtime(rq);
        b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, (*rq).rt.highest_prio.curr);
    }
}
#[no_mangle]
/// Updates RT state during native runqueue hotplug.
///
/// # Safety
/// The initialized runqueue must be locked and retained by the native hotplug
/// protocol, with its root domain and task-group hierarchy alive. Overload membership
/// and cpupri state must match the corresponding online/offline transition.
pub unsafe extern "C" fn rq_offline_rt(rq: *mut b::rq) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if (*rq).rt.overloaded { rt_clear_overload(rq); }
        __disable_runtime(rq);
        b::cpupri_set(addr_of_mut!((*(*rq).rd).cpupri), (*rq).cpu, b::RUST_RT_CPUPRI_INVALID as c_int);
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn switched_from_rt(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_task_on_rq_queued(p) || (*rq).rt.rt_nr_running != 0 { return; }
        rt_queue_pull_task(rq);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
/// Allocates native per-CPU masks for RT placement.
///
/// # Safety
/// Call only in the scheduler initialization phase, before placement callbacks can use
/// the masks and while GFP_KERNEL node allocation is permitted. Native per-CPU storage
/// and possible-CPU topology must be initialized.
pub unsafe extern "C" fn init_sched_rt_class() {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        each_cpu(b::rust_rt_cpu_possible_mask(), |i| { b::rust_rt_zalloc_local_cpu_mask(i, b::rust_rt_cpu_to_node(i)); });
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn switched_to_rt(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if b::rust_rt_task_current(rq, p) { b::rust_rt_update_rt_rq_load_avg(b::rust_rt_rq_clock_pelt(rq), rq, 0); return; }
        if b::rust_rt_task_on_rq_queued(p) {
            if (*p).nr_cpus_allowed > 1 && (*rq).rt.overloaded { rt_queue_push_tasks(rq); }
            if (*p).prio < (*(*rq).donor).prio && b::rust_rt_cpu_online(b::rust_rt_cpu_of(rq)) { b::resched_curr(rq); }
        }
    }
}
#[no_mangle]
/// Applies an RT scheduling operation under the runqueue protocol.
///
/// # Safety
/// The caller must hold the associated native rq lock with the required scheduler
/// interrupt state. Supplied tasks, entities, queues and their hierarchy must be
/// initialized and live; membership and priority indexes must satisfy the native
/// operation's enqueue, dequeue or accounting preconditions. Task lifetime protection
/// must survive any native callback invoked here.
pub unsafe extern "C" fn prio_changed_rt(rq: *mut b::rq, p: *mut b::task_struct, oldprio: u64) {
    // SAFETY: The caller must supply the native object lifetimes and
    // synchronization described in this function's safety contract.
    unsafe {
        if !b::rust_rt_task_on_rq_queued(p) { return; }
        if (*p).prio as u64 == oldprio { return; }
        if b::rust_rt_task_current_donor(rq, p) {
            if oldprio < (*p).prio as u64 { rt_queue_pull_task(rq); }
            if (*p).prio > (*rq).rt.highest_prio.curr { b::resched_curr(rq); }
        } else if (*p).prio < (*(*rq).donor).prio { b::resched_curr(rq); }
    }
}
