// SPDX-License-Identifier: GPL-2.0
// Newly reconstructed SMP owner behavior; native leaves preserve header primitives.
unsafe fn dl_overloaded(rq: *mut b::rq) -> c_int {
    // SAFETY: The live rq and root domain are protected by the scheduling caller; the count read is atomic.
    unsafe { b::rust_dl_atomic_read(addr_of!((*(*rq).rd).dlo_count)) }
}
unsafe fn dl_set_overload(rq: *mut b::rq) {
    // SAFETY: The rq lock stabilizes the live root domain while its overload mask/count are published.
    unsafe {
        if (*rq).online == 0 { return; }
        b::rust_dl_cpumask_set_cpu((*rq).cpu, b::rust_dl_dlo_mask((*rq).rd));
        b::rust_dl_smp_wmb();
        b::rust_dl_atomic_inc(addr_of_mut!((*(*rq).rd).dlo_count));
    }
}
unsafe fn dl_clear_overload(rq: *mut b::rq) {
    // SAFETY: The rq lock stabilizes the live root domain while its overload count/mask are withdrawn.
    unsafe {
        if (*rq).online == 0 { return; }
        b::rust_dl_atomic_dec(addr_of_mut!((*(*rq).rd).dlo_count));
        b::rust_dl_cpumask_clear_cpu((*rq).cpu, b::rust_dl_dlo_mask((*rq).rd));
    }
}
unsafe extern "C" fn __pushable_less(a: *mut b::rb_node, other: *const b::rb_node) -> bool {
    // SAFETY: Both rb nodes belong to live tasks in the rq-locked pushable tree insertion.
    unsafe {
        b::rust_dl_entity_preempt(addr_of!((*b::rust_dl_pushable_from_node(a)).dl),
            addr_of!((*b::rust_dl_pushable_from_node(other.cast_mut())).dl))
    }
}
unsafe fn has_pushable_dl_tasks(rq: *mut b::rq) -> c_int {
    // SAFETY: The scheduler keeps the rq live and serializes its pushable-tree access.
    unsafe { (!b::rust_dl_rb_empty_root(addr_of!((*rq).dl.pushable_dl_tasks_root.rb_root))) as c_int }
}
unsafe fn enqueue_pushable_dl_task(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The rq lock protects the live task's detached pushable node and overload state.
    unsafe {
        b::rust_dl_warn_push_enqueue(!b::rust_dl_rb_empty_node(addr_of!((*p).pushable_dl_tasks)));
        let left = b::rust_dl_rb_add_cached(addr_of_mut!((*p).pushable_dl_tasks),
            addr_of_mut!((*rq).dl.pushable_dl_tasks_root), Some(__pushable_less));
        if !left.is_null() { (*rq).dl.earliest_dl.next = (*p).dl.deadline; }
        if !(*rq).dl.overloaded {
            dl_set_overload(rq);
            (*rq).dl.overloaded = true;
        }
    }
}
unsafe fn dequeue_pushable_dl_task(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The rq lock protects the live task's pushable node and overload state.
    unsafe {
        let dl_rq = addr_of_mut!((*rq).dl);
        if b::rust_dl_rb_empty_node(addr_of!((*p).pushable_dl_tasks)) { return; }
        let left = b::rust_dl_rb_erase_cached(addr_of_mut!((*p).pushable_dl_tasks),
            addr_of_mut!((*dl_rq).pushable_dl_tasks_root));
        if !left.is_null() { (*dl_rq).earliest_dl.next = (*b::rust_dl_pushable_from_node(left)).dl.deadline; }
        b::rust_dl_rb_clear_node(addr_of_mut!((*p).pushable_dl_tasks));
        if has_pushable_dl_tasks(rq) == 0 && (*rq).dl.overloaded {
            dl_clear_overload(rq);
            (*rq).dl.overloaded = false;
        }
    }
}
unsafe fn need_pull_dl_task(rq: *mut b::rq, prev: *mut b::task_struct) -> bool {
    // SAFETY: The scheduler holds rq and keeps its previous task live while deciding whether to pull.
    unsafe { (*rq).online != 0 && b::rust_dl_task(prev) }
}
unsafe fn deadline_queue_push_tasks(rq: *mut b::rq) {
    // SAFETY: The rq lock and CPU lifetime protect the per-CPU callback head and task tree.
    unsafe {
        if has_pushable_dl_tasks(rq) == 0 { return; }
        b::rust_dl_queue_balance_callback(rq, b::rust_dl_push_head((*rq).cpu), Some(push_dl_tasks));
    }
}
unsafe fn deadline_queue_pull_task(rq: *mut b::rq) {
    // SAFETY: The rq lock and CPU lifetime protect the per-CPU pull callback head.
    unsafe { b::rust_dl_queue_balance_callback(rq, b::rust_dl_pull_head((*rq).cpu), Some(pull_dl_task)); }
}
unsafe fn dl_task_offline_migration(rq: *mut b::rq, p: *mut b::task_struct) -> *mut b::rq {
    // SAFETY: The timer caller holds the task PI lock and source rq; double locking transfers rq ownership.
    unsafe {
        let mut later = find_lock_later_rq(p, rq);
        if later.is_null() {
            let mut cpu = b::rust_dl_cpumask_any_and(b::rust_dl_active_mask(), (*p).cpus_ptr);
            if cpu >= b::nr_cpu_ids {
                b::rust_dl_warn_offline_no_cpu(b::rust_dl_bandwidth_enabled());
                cpu = b::rust_dl_cpumask_any(b::rust_dl_active_mask());
            }
            later = b::rust_dl_cpu_rq(cpu as c_int);
            b::rust_dl_double_lock_balance(rq, later);
        }
        let se = addr_of_mut!((*p).dl);
        if (*se).dl_non_contending() != 0 || (*se).dl_throttled() != 0 {
            sub_running_bw(se, addr_of_mut!((*rq).dl));
            sub_rq_bw(se, addr_of_mut!((*rq).dl));
            add_rq_bw(se, addr_of_mut!((*later).dl));
            add_running_bw(se, addr_of_mut!((*later).dl));
        } else {
            sub_rq_bw(se, addr_of_mut!((*rq).dl));
            add_rq_bw(se, addr_of_mut!((*later).dl));
        }
        let old_bw = addr_of_mut!((*(*rq).rd).dl_bw);
        b::rust_dl_raw_spin_lock(addr_of_mut!((*old_bw).lock));
        __dl_sub(old_bw, (*se).dl_bw, b::rust_dl_cpumask_weight(b::rust_dl_rd_span((*rq).rd)) as c_int);
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*old_bw).lock));
        let new_bw = addr_of_mut!((*(*later).rd).dl_bw);
        b::rust_dl_raw_spin_lock(addr_of_mut!((*new_bw).lock));
        __dl_add(new_bw, (*se).dl_bw, b::rust_dl_cpumask_weight(b::rust_dl_rd_span((*later).rd)) as c_int);
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*new_bw).lock));
        b::set_task_cpu(p, (*later).cpu as c_uint);
        b::rust_dl_double_unlock_balance(later, rq);
        later
    }
}
unsafe fn dl_task_is_earliest_deadline(p: *mut b::task_struct, rq: *mut b::rq) -> bool {
    // SAFETY: The live task and rq have the scheduling caller's synchronization for deadline comparison.
    unsafe { (*rq).dl.dl_nr_running == 0 || b::rust_dl_time_before((*p).dl.deadline, (*rq).dl.earliest_dl.curr) }
}
#[export_name = "rust_dl_select_task_rq_dl"]
unsafe extern "C" fn select_task_rq_dl(p: *mut b::task_struct, mut cpu: c_int, flags: c_int) -> c_int {
    // SAFETY: The waking task's PI lock is held; RCU and READ_ONCE protect unlocked current/donor reads.
    unsafe {
        if flags & b::WF_TTWU as c_int == 0 { return cpu; }
        let rq = b::rust_dl_cpu_rq(cpu);
        b::rust_dl_rcu_read_lock();
        let curr = b::rust_dl_read_rq_curr(rq);
        let donor = b::rust_dl_read_rq_donor(rq);
        let mut select = b::rust_dl_unlikely_select_donor(b::rust_dl_task(donor))
            && ((*curr).nr_cpus_allowed < 2 || !b::rust_dl_entity_preempt(addr_of!((*p).dl), addr_of!((*donor).dl)))
            && (*p).nr_cpus_allowed > 1;
        if b::rust_dl_sched_asym_cpucap_active() { select |= !b::rust_dl_task_fits_capacity(p, cpu); }
        if select {
            let target = find_later_rq(p);
            if target != -1 && dl_task_is_earliest_deadline(p, b::rust_dl_cpu_rq(target)) { cpu = target; }
        }
        b::rust_dl_rcu_read_unlock();
        cpu
    }
}
#[export_name = "rust_dl_migrate_task_rq_dl"]
unsafe extern "C" fn migrate_task_rq_dl(p: *mut b::task_struct, _new_cpu: c_int) {
    // SAFETY: The waking caller holds the task PI lock; the acquired rq lock protects accounting and timers.
    unsafe {
        if b::rust_dl_task_state(p) != b::TASK_WAKING as c_uint { return; }
        let rq = b::rust_dl_task_rq(p);
        let mut rf = core::mem::MaybeUninit::<b::rq_flags>::uninit();
        b::rust_dl_rq_lock(rq, rf.as_mut_ptr());
        if (*p).dl.dl_non_contending() != 0 {
            b::update_rq_clock(rq);
            sub_running_bw(addr_of_mut!((*p).dl), addr_of_mut!((*rq).dl));
            (*p).dl.set_dl_non_contending(0);
            cancel_inactive_timer(addr_of_mut!((*p).dl));
        }
        sub_rq_bw(addr_of_mut!((*p).dl), addr_of_mut!((*rq).dl));
        b::rust_dl_rq_unlock(rq, rf.as_mut_ptr());
    }
}
unsafe fn check_preempt_equal_dl(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The caller holds rq and keeps its current, donor and waking task live.
    unsafe {
        if (*b::rust_dl_rq_curr(rq)).nr_cpus_allowed == 1
            || b::cpudl_find(addr_of_mut!((*(*rq).rd).cpudl), b::rust_dl_rq_donor(rq), core::ptr::null_mut()) == 0 { return; }
        if (*p).nr_cpus_allowed != 1
            && b::cpudl_find(addr_of_mut!((*(*rq).rd).cpudl), p, core::ptr::null_mut()) != 0 { return; }
        b::resched_curr(rq);
    }
}
#[export_name = "rust_dl_balance_dl"]
unsafe extern "C" fn balance_dl(rq: *mut b::rq, rf: *mut b::rq_flags) -> c_int {
    // SAFETY: The scheduler holds rq and provides the live lock-pin context across pull lock drops.
    unsafe {
        let p = b::rust_dl_rq_donor(rq);
        if on_dl_rq(addr_of_mut!((*p).dl)) == 0 && need_pull_dl_task(rq, p) {
            b::rust_dl_rq_unpin_lock(rq, rf);
            pull_dl_task(rq);
            b::rust_dl_rq_repin_lock(rq, rf);
        }
        (b::rust_dl_stop_runnable(rq) || b::rust_dl_runnable(rq)) as c_int
    }
}
unsafe fn pick_earliest_pushable_dl_task(rq: *mut b::rq, cpu: c_int) -> *mut b::task_struct {
    // SAFETY: The source rq lock keeps pushable nodes and their owning tasks live during iteration.
    unsafe {
        if has_pushable_dl_tasks(rq) == 0 { return core::ptr::null_mut(); }
        let mut node = b::rust_dl_rb_first_cached(addr_of!((*rq).dl.pushable_dl_tasks_root));
        while !node.is_null() {
            let p = b::rust_dl_pushable_from_node(node);
            if b::rust_dl_task_is_pushable(rq, p, cpu) { return p; }
            node = b::rb_next(node);
        }
        core::ptr::null_mut()
    }
}
unsafe fn find_later_rq(task: *mut b::task_struct) -> c_int {
    // SAFETY: The local caller disables preemption and keeps task live; RCU protects traversed domains.
    unsafe {
        let mask = b::rust_dl_local_cpu_mask();
        let mut this_cpu = b::rust_dl_smp_processor_id();
        let mut cpu = b::rust_dl_task_cpu(task);
        if b::rust_dl_unlikely_missing_mask(mask.is_null()) { return -1; }
        if (*task).nr_cpus_allowed == 1 { return -1; }
        let rq = b::rust_dl_task_rq(task);
        if b::cpudl_find(addr_of_mut!((*(*rq).rd).cpudl), task, mask) == 0 { return -1; }
        if b::rust_dl_cpumask_test_cpu(cpu, mask) { return cpu; }
        if !b::rust_dl_cpumask_test_cpu(this_cpu, mask) { this_cpu = -1; }
        b::rust_dl_rcu_read_lock();
        let mut sd = b::rust_dl_first_domain(cpu);
        while !sd.is_null() {
            if (*sd).flags & b::SD_WAKE_AFFINE as c_int != 0 {
                let span = b::rust_dl_sched_domain_span(sd);
                if this_cpu != -1 && b::rust_dl_cpumask_test_cpu(this_cpu, span) {
                    b::rust_dl_rcu_read_unlock();
                    return this_cpu;
                }
                let best = b::rust_dl_cpumask_any_and_distribute(mask, span) as c_int;
                if best < b::nr_cpu_ids as c_int {
                    b::rust_dl_rcu_read_unlock();
                    return best;
                }
            }
            sd = (*sd).parent;
        }
        b::rust_dl_rcu_read_unlock();
        if this_cpu != -1 { return this_cpu; }
        cpu = b::rust_dl_cpumask_any_distribute(mask) as c_int;
        if cpu < b::nr_cpu_ids as c_int { cpu } else { -1 }
    }
}
unsafe fn pick_next_pushable_dl_task(rq: *mut b::rq) -> *mut b::task_struct {
    // SAFETY: The source rq lock protects pushable nodes and the selected task's scheduler state.
    unsafe {
        if has_pushable_dl_tasks(rq) == 0 { return core::ptr::null_mut(); }
        let mut node = b::rust_dl_rb_first_cached(addr_of!((*rq).dl.pushable_dl_tasks_root));
        let mut p: *mut b::task_struct = core::ptr::null_mut();
        while !node.is_null() {
            let task = b::rust_dl_pushable_from_node(node);
            if !b::rust_dl_task_on_cpu(rq, task) { p = task; break; }
            node = b::rb_next(node);
        }
        if p.is_null() { return p; }
        b::rust_dl_warn_push_wrong_cpu((*rq).cpu != b::rust_dl_task_cpu(p));
        b::rust_dl_warn_push_current(b::rust_dl_task_current(rq, p));
        b::rust_dl_warn_push_affinity((*p).nr_cpus_allowed <= 1);
        b::rust_dl_warn_push_not_queued(!b::rust_dl_task_on_rq_queued(p));
        b::rust_dl_warn_push_not_dl(!b::rust_dl_task(p));
        p
    }
}
const DL_MAX_TRIES: c_int = 3; // Exact owner constant from deadline.c.
#[export_name = "rust_dl_find_lock_later_rq"]
unsafe extern "C" fn find_lock_later_rq(task: *mut b::task_struct, rq: *mut b::rq) -> *mut b::rq {
    // SAFETY: The caller holds source rq and keeps task live; revalidation follows each possible lock drop.
    unsafe {
        let mut later: *mut b::rq = core::ptr::null_mut();
        for _ in 0..DL_MAX_TRIES {
            let cpu = find_later_rq(task);
            if cpu == -1 || cpu == (*rq).cpu { break; }
            later = b::rust_dl_cpu_rq(cpu);
            if !dl_task_is_earliest_deadline(task, later) { later = core::ptr::null_mut(); break; }
            if b::rust_dl_double_lock_balance(rq, later) != 0 {
                if b::rust_dl_unlikely_migration_changed(b::rust_dl_is_migration_disabled(task)
                    || !b::rust_dl_cpumask_test_cpu((*later).cpu, addr_of!((*task).cpus_mask))
                    || ((*task).dl.dl_throttled() != 0
                        && (b::rust_dl_task_rq(task) != rq || b::rust_dl_task_on_cpu(rq, task)
                            || !b::rust_dl_task(task) || !b::rust_dl_task_on_rq_queued(task)))
                    || ((*task).dl.dl_throttled() == 0 && task != pick_next_pushable_dl_task(rq))) {
                    b::rust_dl_double_unlock_balance(rq, later);
                    later = core::ptr::null_mut();
                    break;
                }
            }
            if dl_task_is_earliest_deadline(task, later) { break; }
            b::rust_dl_double_unlock_balance(rq, later);
            later = core::ptr::null_mut();
        }
        later
    }
}
unsafe fn push_dl_task(rq: *mut b::rq) -> c_int {
    // SAFETY: The source rq is locked; task references bridge lock drops and the destination is double-locked.
    unsafe {
        let mut next = pick_next_pushable_dl_task(rq);
        if next.is_null() { return 0; }
        loop {
            let donor = b::rust_dl_rq_donor(rq);
            if b::rust_dl_task(donor) && b::rust_dl_time_before((*next).dl.deadline, (*donor).dl.deadline)
                && (*b::rust_dl_rq_curr(rq)).nr_cpus_allowed > 1 { b::resched_curr(rq); return 0; }
            if b::rust_dl_is_migration_disabled(next) { return 0; }
            if b::rust_dl_warn_push_self(next == b::rust_dl_rq_curr(rq)) { return 0; }
            b::rust_dl_get_task_struct(next);
            let later = find_lock_later_rq(next, rq);
            if later.is_null() {
                let task = pick_next_pushable_dl_task(rq);
                if task == next || task.is_null() { b::rust_dl_put_task_struct(next); return 0; }
                b::rust_dl_put_task_struct(next);
                next = task;
                continue;
            }
            b::rust_dl_move_queued_task_locked(rq, later, next);
            b::resched_curr(later);
            b::rust_dl_double_unlock_balance(rq, later);
            b::rust_dl_put_task_struct(next);
            return 1;
        }
    }
}
unsafe extern "C" fn push_dl_tasks(rq: *mut b::rq) {
    // SAFETY: The balance callback enters with its live source rq locked.
    unsafe { while push_dl_task(rq) != 0 {} }
}
unsafe extern "C" fn pull_dl_task(this_rq: *mut b::rq) {
    // SAFETY: The balance caller holds the live rq with scheduling/preemption exclusion across explicit lock drops.
    unsafe {
        let this_cpu = (*this_rq).cpu;
        let mut resched = false;
        let mut dmin = b::rust_dl_long_max();
        if b::rust_dl_likely_not_overloaded(dl_overloaded(this_rq) == 0) { return; }
        b::rust_dl_smp_rmb();
        let mut cpu = b::rust_dl_cpumask_first(b::rust_dl_dlo_mask((*this_rq).rd));
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            'next_cpu: {
                if this_cpu == cpu as c_int { break 'next_cpu; }
                let src = b::rust_dl_cpu_rq(cpu as c_int);
                if (*this_rq).dl.dl_nr_running != 0
                    && b::rust_dl_time_before((*this_rq).dl.earliest_dl.curr, (*src).dl.earliest_dl.next) { break 'next_cpu; }
                let mut push_task: *mut b::task_struct = core::ptr::null_mut();
                b::rust_dl_double_lock_balance(this_rq, src);
                'skip: {
                    if (*src).dl.dl_nr_running <= 1 { break 'skip; }
                    let p = pick_earliest_pushable_dl_task(src, this_cpu);
                    if !p.is_null() && b::rust_dl_time_before((*p).dl.deadline, dmin)
                        && dl_task_is_earliest_deadline(p, this_rq) {
                        b::rust_dl_warn_pull_current(p == b::rust_dl_rq_curr(src));
                        b::rust_dl_warn_pull_not_queued(!b::rust_dl_task_on_rq_queued(p));
                        if b::rust_dl_time_before((*p).dl.deadline, (*b::rust_dl_rq_donor(src)).dl.deadline) { break 'skip; }
                        if b::rust_dl_is_migration_disabled(p) { push_task = b::rust_dl_get_push_task(src); }
                        else {
                            b::rust_dl_move_queued_task_locked(src, this_rq, p);
                            dmin = (*p).dl.deadline;
                            resched = true;
                        }
                    }
                }
                b::rust_dl_double_unlock_balance(this_rq, src);
                if !push_task.is_null() {
                    b::rust_dl_preempt_disable();
                    b::rust_dl_raw_spin_rq_unlock(this_rq);
                    b::stop_one_cpu_nowait((*src).cpu as c_uint, Some(b::push_cpu_stop), push_task.cast(), addr_of_mut!((*src).push_work));
                    b::rust_dl_preempt_enable();
                    b::rust_dl_raw_spin_rq_lock(this_rq);
                }
            }
            cpu = b::rust_dl_cpumask_next(cpu as c_int, b::rust_dl_dlo_mask((*this_rq).rd));
        }
        if resched { b::resched_curr(this_rq); }
    }
}
#[export_name = "rust_dl_task_woken_dl"]
unsafe extern "C" fn task_woken_dl(rq: *mut b::rq, p: *mut b::task_struct) {
    // SAFETY: The wakeup caller holds rq and keeps the waking task, current and donor live.
    unsafe {
        let curr = b::rust_dl_rq_curr(rq);
        let donor = b::rust_dl_rq_donor(rq);
        if !b::rust_dl_task_on_cpu(rq, p) && !b::rust_dl_test_tsk_need_resched(curr)
            && (*p).nr_cpus_allowed > 1 && b::rust_dl_task(donor)
            && ((*curr).nr_cpus_allowed < 2 || !b::rust_dl_entity_preempt(addr_of!((*p).dl), addr_of!((*donor).dl))) {
            push_dl_tasks(rq);
        }
    }
}
#[export_name = "rust_dl_set_cpus_allowed_dl"]
unsafe extern "C" fn set_cpus_allowed_dl(p: *mut b::task_struct, ctx: *mut b::affinity_context) {
    // SAFETY: The affinity caller locks task/rq and supplies a live context with destination bandwidth reserved.
    unsafe {
        b::rust_dl_warn_affinity_not_dl(!b::rust_dl_task(p));
        let rq = b::rust_dl_task_rq(p);
        if dl_task_needs_bw_move(p, (*ctx).new_mask) {
            let bw = dl_bw_of(b::rust_dl_cpu_of(rq));
            b::rust_dl_raw_spin_lock(addr_of_mut!((*bw).lock));
            __dl_sub(bw, (*p).dl.dl_bw, dl_bw_cpus(b::rust_dl_task_cpu(p)));
            b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
        }
        b::set_cpus_allowed_common(p, ctx);
    }
}
#[export_name = "rust_dl_rq_online_dl"]
unsafe extern "C" fn rq_online_dl(rq: *mut b::rq) {
    // SAFETY: The online transition caller holds the live rq lock and its attached root domain is valid.
    unsafe {
        if (*rq).dl.overloaded { dl_set_overload(rq); }
        if (*rq).dl.dl_nr_running > 0 { b::cpudl_set(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, (*rq).dl.earliest_dl.curr); }
        else { b::cpudl_clear(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, true); }
    }
}
#[export_name = "rust_dl_rq_offline_dl"]
unsafe extern "C" fn rq_offline_dl(rq: *mut b::rq) {
    // SAFETY: The offline transition caller holds the live rq lock and its attached root domain is valid.
    unsafe {
        if (*rq).dl.overloaded { dl_clear_overload(rq); }
        b::cpudl_clear(addr_of_mut!((*(*rq).rd).cpudl), (*rq).cpu, false);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn init_sched_dl_class() {
    // SAFETY: Scheduler initialization keeps possible CPUs stable while initializing their per-CPU masks.
    unsafe {
        let mask = b::rust_dl_possible_mask();
        let mut cpu = b::rust_dl_cpumask_first(mask);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            b::rust_dl_alloc_local_cpu_mask(cpu as c_int);
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
    }
}
unsafe fn dl_get_task_effective_cpus(p: *mut b::task_struct, cpus: *mut b::cpumask) {
    // SAFETY: The caller holds cpuset_mutex and supplies a live task and writable CPU mask.
    unsafe {
        let hk = b::rust_dl_housekeeping_domain_mask();
        if b::rust_dl_housekeeping_domain_enabled() && !b::rust_dl_cpumask_intersects((*p).cpus_ptr, hk) {
            b::rust_dl_cpumask_andnot(cpus, b::rust_dl_active_mask(), hk);
            return;
        }
        b::rust_dl_cpuset_cpus_allowed_locked(p, cpus);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_add_task_root_domain(p: *mut b::task_struct) {
    // SAFETY: The caller holds cpuset_mutex; the acquired PI lock stabilizes the task during domain admission.
    unsafe {
        let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*p).pi_lock));
        if !b::rust_dl_task(p) || b::rust_dl_entity_is_special(addr_of_mut!((*p).dl)) {
            b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), flags);
            return;
        }
        let mask = b::rust_dl_local_cpu_mask();
        dl_get_task_effective_cpus(p, mask);
        let cpu = b::rust_dl_cpumask_first_and(b::rust_dl_active_mask(), mask);
        b::rust_dl_bug_no_effective_cpu(cpu >= b::nr_cpu_ids);
        let rq = b::rust_dl_cpu_rq(cpu as c_int);
        let bw = addr_of_mut!((*(*rq).rd).dl_bw);
        b::rust_dl_raw_spin_lock(addr_of_mut!((*bw).lock));
        __dl_add(bw, (*p).dl.dl_bw, b::rust_dl_cpumask_weight(b::rust_dl_rd_span((*rq).rd)) as c_int);
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
        b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*p).pi_lock), flags);
    }
}
