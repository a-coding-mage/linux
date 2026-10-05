// SPDX-License-Identifier: GPL-2.0
// Newly reconstructed from deadline.original.c; not recovered private Rust bodies.
#[inline]
unsafe fn dl_bw_of(cpu: c_int) -> *mut b::dl_bw {
    // SAFETY: The CPU is valid and scheduler RCU protects its rq root-domain pointer.
    unsafe {
        b::rust_dl_rcu_warn_bw_of();
        addr_of_mut!((*(*b::rust_dl_cpu_rq(cpu)).rd).dl_bw)
    }
}
#[inline]
unsafe fn dl_bw_cpus(cpu: c_int) -> c_int {
    // SAFETY: The valid CPU's root-domain span is held live under scheduler RCU.
    unsafe {
        let rd = (*b::rust_dl_cpu_rq(cpu)).rd;
        b::rust_dl_rcu_warn_bw_cpus();
        b::rust_dl_cpumask_weight_and(b::rust_dl_rd_span(rd), b::rust_dl_active_mask()) as c_int
    }
}
#[inline]
unsafe fn dl_bw_capacity(cpu: c_int) -> c_ulong {
    // SAFETY: The CPU and its domain span are valid under the caller's scheduler-RCU exclusion.
    unsafe {
        if !b::rust_dl_sched_asym_cpucap_active()
            && b::rust_dl_arch_scale_cpu_capacity(cpu) == b::SCHED_CAPACITY_SCALE as c_ulong {
            (dl_bw_cpus(cpu) as c_ulong) << b::SCHED_CAPACITY_SHIFT
        } else {
            b::rust_dl_rcu_warn_bw_capacity();
            __dl_bw_capacity(b::rust_dl_rd_span((*b::rust_dl_cpu_rq(cpu)).rd))
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_bw_visited(cpu: c_int, cookie: u64) -> bool {
    // SAFETY: The caller serializes domain rebuilding or global validation and supplies a valid CPU.
    unsafe {
        let rd = (*b::rust_dl_cpu_rq(cpu)).rd;
        if (*rd).visit_cookie == cookie { return true; }
        (*rd).visit_cookie = cookie;
        false
    }
}
#[inline]
unsafe fn __dl_update(dl_b: *mut b::dl_bw, bw: i64) {
    // SAFETY: The live bandwidth owner and its span are protected by its lock and scheduler RCU.
    unsafe {
        let rd = b::rust_dl_rd_from_bw(dl_b);
        b::rust_dl_rcu_warn_update();
        let mask = b::rust_dl_rd_span(rd);
        let active = b::rust_dl_active_mask();
        let mut cpu = b::rust_dl_cpumask_first_and(mask, active);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            let rq = b::rust_dl_cpu_rq(cpu as c_int);
            (*rq).dl.extra_bw = (*rq).dl.extra_bw.wrapping_add(bw as u64);
            cpu = b::rust_dl_cpumask_next_and(cpu as c_int, mask, active);
        }
    }
}
#[inline]
unsafe fn __dl_sub(dl_b: *mut b::dl_bw, tsk_bw: u64, cpus: c_int) {
    // SAFETY: The bandwidth lock and scheduler RCU are held; cpus is the nonzero admission count.
    unsafe {
        (*dl_b).total_bw = (*dl_b).total_bw.wrapping_sub(tsk_bw);
        // The signed 32-bit truncation precedes division in the C authority.
        __dl_update(dl_b, ((tsk_bw as i32) / cpus) as i64);
    }
}
#[inline]
unsafe fn __dl_add(dl_b: *mut b::dl_bw, tsk_bw: u64, cpus: c_int) {
    // SAFETY: The bandwidth lock and scheduler RCU are held; cpus is the nonzero admission count.
    unsafe {
        (*dl_b).total_bw = (*dl_b).total_bw.wrapping_add(tsk_bw);
        __dl_update(dl_b, ((tsk_bw as i32) / cpus).wrapping_neg() as i64);
    }
}
#[inline]
unsafe fn __dl_overflow(dl_b: *mut b::dl_bw, cap: c_ulong, old_bw: u64, new_bw: u64) -> bool {
    // SAFETY: The live bandwidth record is protected by its lock while totals are inspected.
    unsafe {
        (*dl_b).bw != u64::MAX && b::rust_dl_cap_scale((*dl_b).bw, cap)
            < (*dl_b).total_bw.wrapping_sub(old_bw).wrapping_add(new_bw)
    }
}
#[inline]
unsafe fn __add_running_bw(bw: u64, dl_rq: *mut b::dl_rq) {
    // SAFETY: The dl runqueue is live and its owning rq lock is held throughout accounting.
    unsafe {
        let old = (*dl_rq).running_bw;
        b::rust_dl_lockdep_assert_rq_held(rq_of_dl_rq(dl_rq));
        (*dl_rq).running_bw = old.wrapping_add(bw);
        b::rust_dl_warn_add_running_overflow((*dl_rq).running_bw < old);
        b::rust_dl_warn_add_running_total((*dl_rq).running_bw > (*dl_rq).this_bw);
        b::rust_dl_cpufreq_update_util(rq_of_dl_rq(dl_rq), 0);
    }
}
#[inline]
unsafe fn __sub_running_bw(bw: u64, dl_rq: *mut b::dl_rq) {
    // SAFETY: The dl runqueue is live and its owning rq lock is held throughout accounting.
    unsafe {
        let old = (*dl_rq).running_bw;
        b::rust_dl_lockdep_assert_rq_held(rq_of_dl_rq(dl_rq));
        (*dl_rq).running_bw = old.wrapping_sub(bw);
        b::rust_dl_warn_sub_running_underflow((*dl_rq).running_bw > old);
        if (*dl_rq).running_bw > old { (*dl_rq).running_bw = 0; }
        b::rust_dl_cpufreq_update_util(rq_of_dl_rq(dl_rq), 0);
    }
}
#[inline]
unsafe fn __add_rq_bw(bw: u64, dl_rq: *mut b::dl_rq) {
    // SAFETY: The dl runqueue is live and its owning rq lock serializes assigned bandwidth.
    unsafe {
        let old = (*dl_rq).this_bw;
        b::rust_dl_lockdep_assert_rq_held(rq_of_dl_rq(dl_rq));
        (*dl_rq).this_bw = old.wrapping_add(bw);
        b::rust_dl_warn_add_rq_overflow((*dl_rq).this_bw < old);
    }
}
#[inline]
unsafe fn __sub_rq_bw(bw: u64, dl_rq: *mut b::dl_rq) {
    // SAFETY: The dl runqueue is live and its owning rq lock serializes assigned bandwidth.
    unsafe {
        let old = (*dl_rq).this_bw;
        b::rust_dl_lockdep_assert_rq_held(rq_of_dl_rq(dl_rq));
        (*dl_rq).this_bw = old.wrapping_sub(bw);
        b::rust_dl_warn_sub_rq_underflow((*dl_rq).this_bw > old);
        if (*dl_rq).this_bw > old { (*dl_rq).this_bw = 0; }
        b::rust_dl_warn_sub_rq_running((*dl_rq).running_bw > (*dl_rq).this_bw);
    }
}
#[inline]
unsafe fn add_rq_bw(se: *mut b::sched_dl_entity, rq: *mut b::dl_rq) {
    // SAFETY: The live entity belongs to the locked destination dl runqueue for this accounting step.
    unsafe {
        if !b::rust_dl_entity_is_special(se) { __add_rq_bw((*se).dl_bw, rq); }
    }
}
#[inline]
unsafe fn sub_rq_bw(se: *mut b::sched_dl_entity, rq: *mut b::dl_rq) {
    // SAFETY: The live entity belongs to the locked source dl runqueue for this accounting step.
    unsafe {
        if !b::rust_dl_entity_is_special(se) { __sub_rq_bw((*se).dl_bw, rq); }
    }
}
#[inline]
unsafe fn add_running_bw(se: *mut b::sched_dl_entity, rq: *mut b::dl_rq) {
    // SAFETY: The live entity and running bandwidth are serialized by the supplied rq lock.
    unsafe {
        if !b::rust_dl_entity_is_special(se) { __add_running_bw((*se).dl_bw, rq); }
    }
}
#[inline]
unsafe fn sub_running_bw(se: *mut b::sched_dl_entity, rq: *mut b::dl_rq) {
    // SAFETY: The live entity and running bandwidth are serialized by the supplied rq lock.
    unsafe {
        if !b::rust_dl_entity_is_special(se) { __sub_running_bw((*se).dl_bw, rq); }
    }
}
unsafe fn dl_rq_change_utilization(rq: *mut b::rq, se: *mut b::sched_dl_entity, new_bw: u64) {
    // SAFETY: The rq lock protects the live entity, its timer state and bandwidth transition.
    unsafe {
        if (*se).dl_non_contending() != 0 {
            sub_running_bw(se, addr_of_mut!((*rq).dl));
            (*se).set_dl_non_contending(0);
            if b::hrtimer_try_to_cancel(addr_of_mut!((*se).inactive_timer)) == 1
                && !b::rust_dl_server(se) {
                b::rust_dl_put_task_struct(b::rust_dl_task_of(se));
            }
        }
        __sub_rq_bw((*se).dl_bw, addr_of_mut!((*rq).dl));
        __add_rq_bw(new_bw, addr_of_mut!((*rq).dl));
    }
}
#[inline(always)]
unsafe fn cancel_dl_timer(se: *mut b::sched_dl_entity, timer: *mut b::hrtimer) {
    // SAFETY: The live entity owns this initialized timer and the caller serializes cancellation.
    unsafe {
        if b::hrtimer_try_to_cancel(timer) == 1 && !b::rust_dl_server(se) {
            b::rust_dl_put_task_struct(b::rust_dl_task_of(se));
        }
    }
}
#[inline(always)]
unsafe fn cancel_replenish_timer(se: *mut b::sched_dl_entity) {
    // SAFETY: The entity is live with an initialized replenishment timer and serialized state.
    unsafe {
        cancel_dl_timer(se, addr_of_mut!((*se).dl_timer));
    }
}
#[inline(always)]
unsafe fn cancel_inactive_timer(se: *mut b::sched_dl_entity) {
    // SAFETY: The entity is live with an initialized inactive timer and serialized state.
    unsafe {
        cancel_dl_timer(se, addr_of_mut!((*se).inactive_timer));
    }
}
unsafe fn dl_change_utilization(p: *mut b::task_struct, new_bw: u64) {
    // SAFETY: The live task's scheduler state and rq bandwidth are locked by the policy-change caller.
    unsafe {
        b::rust_dl_warn_change_sugov((*p).dl.flags & b::SCHED_FLAG_SUGOV as u32 != 0);
        if b::rust_dl_task_on_rq_queued(p) { return; }
        dl_rq_change_utilization(b::rust_dl_task_rq(p), addr_of_mut!((*p).dl), new_bw);
    }
}
unsafe fn init_dl_rq_bw_ratio(rq: *mut b::dl_rq) {
    // SAFETY: The caller exclusively initializes rq or holds global bandwidth-update exclusion.
    unsafe {
        if b::rust_dl_global_rt_runtime() == b::RUNTIME_INF as u64 {
            (*rq).bw_ratio = 1 << b::RATIO_SHIFT;
            (*rq).max_bw = 1 << b::BW_SHIFT;
            (*rq).extra_bw = (*rq).max_bw;
        } else {
            (*rq).bw_ratio = b::to_ratio(b::rust_dl_global_rt_runtime(), b::rust_dl_global_rt_period())
                >> (b::BW_SHIFT - b::RATIO_SHIFT);
            (*rq).max_bw = b::to_ratio(b::rust_dl_global_rt_period(), b::rust_dl_global_rt_runtime());
            (*rq).extra_bw = (*rq).max_bw;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_dl_bw(bw: *mut b::dl_bw) {
    // SAFETY: The bandwidth object is writable and unpublished during initialization.
    unsafe {
        b::rust_dl_init_bw_lock(addr_of_mut!((*bw).lock));
        (*bw).bw = if b::rust_dl_global_rt_runtime() == b::RUNTIME_INF as u64 { u64::MAX }
            else { b::to_ratio(b::rust_dl_global_rt_period(), b::rust_dl_global_rt_runtime()) };
        (*bw).total_bw = 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_dl_rq(rq: *mut b::dl_rq) {
    // SAFETY: The native dl runqueue is writable and exclusively owned during initialization.
    unsafe {
        b::rust_dl_init_rb_root_cached(addr_of_mut!((*rq).root));
        (*rq).earliest_dl.curr = 0;
        (*rq).earliest_dl.next = 0;
        (*rq).overloaded = false;
        b::rust_dl_init_rb_root_cached(addr_of_mut!((*rq).pushable_dl_tasks_root));
        (*rq).running_bw = 0;
        (*rq).this_bw = 0;
        init_dl_rq_bw_ratio(rq);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_dl_global_validate() -> c_int {
    // SAFETY: The caller holds the RT-handler mutex and sched_domains_mutex; local locks protect totals.
    unsafe {
        let new_bw = b::to_ratio(b::rust_dl_global_rt_period(), b::rust_dl_global_rt_runtime());
        dl_cookie = dl_cookie.wrapping_add(1);
        let cookie = dl_cookie;
        let mask = b::rust_dl_online_mask();
        let mut cpu = b::rust_dl_cpumask_first(mask);
        let mut ret = 0;
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            b::rust_dl_rcu_read_lock_sched();
            if !dl_bw_visited(cpu as c_int, cookie) {
                let bw = dl_bw_of(cpu as c_int);
                let cpus = dl_bw_cpus(cpu as c_int);
                let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*bw).lock));
                if new_bw.wrapping_mul(cpus as u64) < (*bw).total_bw { ret = -(b::EBUSY as c_int); }
                b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*bw).lock), flags);
            }
            b::rust_dl_rcu_read_unlock_sched();
            if ret != 0 { break; }
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
        ret
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_dl_do_global() {
    // SAFETY: The caller holds the RT-handler mutex and sched_domains_mutex while replacing global limits.
    unsafe {
        let new_bw = if b::rust_dl_global_rt_runtime() == b::RUNTIME_INF as u64 { u64::MAX }
            else { b::to_ratio(b::rust_dl_global_rt_period(), b::rust_dl_global_rt_runtime()) };
        dl_cookie = dl_cookie.wrapping_add(1);
        let cookie = dl_cookie;
        let mask = b::rust_dl_possible_mask();
        let mut cpu = b::rust_dl_cpumask_first(mask);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            init_dl_rq_bw_ratio(addr_of_mut!((*b::rust_dl_cpu_rq(cpu as c_int)).dl));
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
        cpu = b::rust_dl_cpumask_first(mask);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            b::rust_dl_rcu_read_lock_sched();
            if !dl_bw_visited(cpu as c_int, cookie) {
                let bw = dl_bw_of(cpu as c_int);
                let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*bw).lock));
                (*bw).bw = new_bw;
                b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*bw).lock), flags);
            }
            b::rust_dl_rcu_read_unlock_sched();
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sched_dl_overflow(p: *mut b::task_struct, policy: c_int, attr: *const b::sched_attr) -> c_int {
    // SAFETY: The policy-change caller locks the live task and rq, supplies live attrs and holds scheduler RCU.
    unsafe {
        let period = if (*attr).sched_period != 0 { (*attr).sched_period } else { (*attr).sched_deadline };
        let new_bw = if b::rust_dl_policy(policy) { b::to_ratio(period, (*attr).sched_runtime) } else { 0 };
        let cpu = b::rust_dl_task_cpu(p);
        let bw = dl_bw_of(cpu);
        if (*attr).sched_flags & b::SCHED_FLAG_SUGOV as u64 != 0 { return 0; }
        if new_bw == (*p).dl.dl_bw && b::rust_dl_task_has_dl_policy(p) { return 0; }
        b::rust_dl_raw_spin_lock(addr_of_mut!((*bw).lock));
        let cpus = dl_bw_cpus(cpu);
        let cap = dl_bw_capacity(cpu);
        let mut err = -1;
        if b::rust_dl_policy(policy) && !b::rust_dl_task_has_dl_policy(p) && !__dl_overflow(bw, cap, 0, new_bw) {
            if b::rust_dl_hrtimer_active(addr_of!((*p).dl.inactive_timer)) { __dl_sub(bw, (*p).dl.dl_bw, cpus); }
            __dl_add(bw, new_bw, cpus);
            err = 0;
        } else if b::rust_dl_policy(policy) && b::rust_dl_task_has_dl_policy(p) && !__dl_overflow(bw, cap, (*p).dl.dl_bw, new_bw) {
            __dl_sub(bw, (*p).dl.dl_bw, cpus);
            __dl_add(bw, new_bw, cpus);
            dl_change_utilization(p, new_bw);
            err = 0;
        } else if !b::rust_dl_policy(policy) && b::rust_dl_task_has_dl_policy(p) { err = 0; }
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
        err
    }
}

unsafe fn dl_server_add_bw(rd: *mut b::root_domain, cpu: c_int) {
    // SAFETY: The rebuilding caller locks the live domain bandwidth and stabilizes configured CPU servers.
    unsafe {
        let se = addr_of_mut!((*b::rust_dl_cpu_rq(cpu)).fair_server);
        if b::rust_dl_server(se) && (*se).dl_bw_attached() != 0 && b::rust_dl_cpu_active(cpu) {
            __dl_add(addr_of_mut!((*rd).dl_bw), (*se).dl_bw, dl_bw_cpus(cpu));
        }
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        {
            let se = addr_of_mut!((*b::rust_dl_cpu_rq(cpu)).ext_server);
            if b::rust_dl_server(se) && (*se).dl_bw_attached() != 0 && b::rust_dl_cpu_active(cpu) {
                __dl_add(addr_of_mut!((*rd).dl_bw), (*se).dl_bw, dl_bw_cpus(cpu));
            }
        }
    }
}
unsafe fn dl_server_read_bw(cpu: c_int) -> u64 {
    // SAFETY: The valid CPU's server reservations remain stable under the bandwidth-management caller.
    unsafe {
        let mut bw: u64 = 0;
        let se = addr_of!((*b::rust_dl_cpu_rq(cpu)).fair_server);
        if (*se).dl_server() != 0 && (*se).dl_bw_attached() != 0 {
            bw = bw.wrapping_add((*se).dl_bw);
        }
        #[cfg(CONFIG_SCHED_CLASS_EXT)]
        {
            let se = addr_of!((*b::rust_dl_cpu_rq(cpu)).ext_server);
            if (*se).dl_server() != 0 && (*se).dl_bw_attached() != 0 {
                bw = bw.wrapping_add((*se).dl_bw);
            }
        }
        bw
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_clear_root_domain(rd: *mut b::root_domain) {
    // SAFETY: The caller serializes domain rebuilding; the domain is live through its acquired bandwidth lock.
    unsafe {
        let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*rd).dl_bw.lock));
        (*rd).dl_bw.total_bw = 0;
        let mask = b::rust_dl_rd_span(rd);
        let mut cpu = b::rust_dl_cpumask_first(mask);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            let rq = b::rust_dl_cpu_rq(cpu as c_int);
            (*rq).dl.extra_bw = (*rq).dl.max_bw;
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
        cpu = b::rust_dl_cpumask_first(mask);
        while cpu < b::rust_dl_cpumask_iteration_limit() {
            dl_server_add_bw(rd, cpu as c_int);
            cpu = b::rust_dl_cpumask_next(cpu as c_int, mask);
        }
        b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*rd).dl_bw.lock), flags);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_clear_root_domain_cpu(cpu: c_int) {
    // SAFETY: The CPU is valid and domain rebuilding exclusion keeps its root domain live.
    unsafe {
        dl_clear_root_domain((*b::rust_dl_cpu_rq(cpu)).rd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_task_needs_bw_move(p: *mut b::task_struct, new_mask: *const b::cpumask) -> bool {
    // SAFETY: The task and new affinity mask are live; the affinity caller stabilizes its root-domain span.
    unsafe {
        if !b::rust_dl_task(p) { return false; }
        !b::rust_dl_cpumask_intersects(b::rust_dl_rd_span((*b::rust_dl_task_rq(p)).rd), new_mask)
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_cpuset_cpumask_can_shrink(cur: *const b::cpumask, trial: *const b::cpumask) -> c_int {
    // SAFETY: The current and trial masks are live and current is nonempty; local RCU and lock protect totals.
    unsafe {
        b::rust_dl_rcu_read_lock_sched();
        let bw = dl_bw_of(b::rust_dl_cpumask_any(cur) as c_int);
        let cap = __dl_bw_capacity(trial);
        let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*bw).lock));
        let ret = (!__dl_overflow(bw, cap, 0, 0)) as c_int;
        b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*bw).lock), flags);
        b::rust_dl_rcu_read_unlock_sched();
        ret
    }
}
// Private owner control flow, not an FFI enum or a surrogate header layout.
enum DlBwRequest { Deactivate, Alloc, Free }
unsafe fn dl_bw_manage(req: DlBwRequest, cpu: c_int, task_bw: u64) -> c_int {
    // SAFETY: The CPU is valid and caller serializes the relevant CPU/cpuset change; local RCU and lock protect totals.
    unsafe {
        b::rust_dl_rcu_read_lock_sched();
        let bw = dl_bw_of(cpu);
        let flags = b::rust_dl_raw_spin_lock_irqsave(addr_of_mut!((*bw).lock));
        let mut cap = dl_bw_capacity(cpu);
        let mut overflow = false;
        match req {
            DlBwRequest::Free => __dl_sub(bw, task_bw, dl_bw_cpus(cpu)),
            DlBwRequest::Alloc => {
                overflow = __dl_overflow(bw, cap, 0, task_bw);
                if !overflow { __dl_add(bw, task_bw, dl_bw_cpus(cpu)); }
            }
            DlBwRequest::Deactivate => {
                cap = cap.wrapping_sub(b::rust_dl_arch_scale_cpu_capacity(cpu));
                let server_bw = dl_server_read_bw(cpu);
                if (*bw).total_bw.wrapping_sub(server_bw) > 0 {
                    overflow = if dl_bw_cpus(cpu) - 1 != 0 { __dl_overflow(bw, cap, server_bw, 0) }
                        else { true };
                }
            }
        }
        b::rust_dl_raw_spin_unlock_irqrestore(addr_of_mut!((*bw).lock), flags);
        b::rust_dl_rcu_read_unlock_sched();
        if overflow { -(b::EBUSY as c_int) } else { 0 }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_bw_deactivate(cpu: c_int) -> c_int {
    // SAFETY: The CPU-hotplug caller supplies a valid CPU and serializes its deactivation admission.
    unsafe {
        dl_bw_manage(DlBwRequest::Deactivate, cpu, 0)
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_bw_alloc(cpu: c_int, bw: u64) -> c_int {
    // SAFETY: The caller supplies a valid destination CPU and serializes its bandwidth reservation.
    unsafe {
        dl_bw_manage(DlBwRequest::Alloc, cpu, bw)
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_bw_free(cpu: c_int, bw: u64) {
    // SAFETY: The caller supplies a valid source CPU and serializes releasing its bandwidth reservation.
    unsafe {
        dl_bw_manage(DlBwRequest::Free, cpu, bw);
    }
}
