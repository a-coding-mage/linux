// SPDX-License-Identifier: GPL-2.0-only
// core.c:6128..6723. Class selection and SMT cookie coordination stay in Rust.
unsafe fn prev_balance(rq: *mut rq, rf: *mut rq_flags) {
    // SAFETY: The caller supplies a live rq and flags with its lock held;
    // class balance callbacks must return with that lock held.
    unsafe {
        let start = (*lupos_core_rq_donor(rq)).sched_class;
        let mut class = lupos_core_first_active_class_from(start);
        while !class.is_null() && class != addr_of!(idle_sched_class) {
            if let Some(balance) = (*class).balance {
                if balance(rq, rf) != 0 {
                    break;
                }
            }
            class = lupos_core_next_active_class(class);
        }
    }
}
unsafe fn __pick_next_task(rq: *mut rq, rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The caller owns the rq scheduling lock and valid flags;
    // native class callbacks implement the pick/retry and idle-task contracts.
    unsafe {
        (*rq).dl_server = null_mut();
        if !lupos_core_scx_enabled()
            && !lupos_core_sched_class_above(
                (*lupos_core_rq_donor(rq)).sched_class,
                addr_of!(fair_sched_class),
            )
            && (*rq).nr_running == (*rq).cfs.h_nr_queued
        {
            let mut p = pick_task_fair(rq, rf);
            if p != lupos_core_retry_task() {
                if p.is_null() {
                    p = pick_task_idle(rq, rf);
                }
                lupos_core_put_prev_set_next_task(rq, lupos_core_rq_donor(rq), p);
                return p;
            }
        }
        'restart: loop {
            prev_balance(rq, rf);
            let mut class = lupos_core_first_active_class();
            while !class.is_null() {
                let p = (*class).pick_task.unwrap()(rq, rf);
                if p == lupos_core_retry_task() {
                    continue 'restart;
                }
                if !p.is_null() {
                    lupos_core_put_prev_set_next_task(rq, lupos_core_rq_donor(rq), p);
                    return p;
                }
                class = lupos_core_next_active_class(class);
            }
            lupos_core_bug_site_6184();
            core::hint::unreachable_unchecked();
        }
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn is_task_rq_idle(t: *mut task_struct) -> bool {
    // SAFETY: The caller keeps t live and its runqueue association stable
    // under the rq/core scheduling lock.
    unsafe {
        (*lupos_core_task_rq(t)).idle == t
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn cookie_equals(a: *mut task_struct, cookie: c_ulong) -> bool {
    // SAFETY: a is a live selected task protected by the rq/core lock;
    // the idle and cookie fields remain valid for this comparison.
    unsafe {
        is_task_rq_idle(a) || (*a).core_cookie == cookie
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn cookie_match(a: *mut task_struct, b: *mut task_struct) -> bool {
    // SAFETY: Both pointers name live picks protected by the shared rq
    // lock, so their idle identity and cookie fields can be inspected.
    unsafe {
        is_task_rq_idle(a) || is_task_rq_idle(b) || (*a).core_cookie == (*b).core_cookie
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn pick_task(rq: *mut rq, rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The caller holds the rq lock and supplies valid flags. Native
    // class callbacks return with the lock held, including the retry case.
    unsafe {
        (*rq).dl_server = null_mut();
        let mut class = lupos_core_first_active_class();
        while !class.is_null() {
            let p = (*class).pick_task.unwrap()(rq, rf);
            if !p.is_null() {
                return p;
            }
            class = lupos_core_next_active_class(class);
        }
        lupos_core_bug_site_6223();
        core::hint::unreachable_unchecked()
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn pick_next_task(rq: *mut rq, rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The caller holds the rq scheduling lock with matching flags.
    // Class callbacks preserve their native lock/retry contract; rq->core is
    // reloaded after callbacks which may have temporarily dropped the lock.
    unsafe {
        let mut core_clock_updated = rq == (*rq).core;
        if !lupos_core_sched_core_enabled(rq) {
            return __pick_next_task(rq, rf);
        }
        let cpu = lupos_core_cpu_of(rq);
        if lupos_core_cpu_is_offline(cpu) {
            (*rq).core_pick = null_mut();
            (*rq).core_dl_server = null_mut();
            return __pick_next_task(rq, rf);
        }
        // Class balance/pick callbacks may drop the rq lock. CPU hotplug can
        // then move the leader and its in-flight count; reload rq->core as C does.
        (*(*rq).core).core_pick_in_flight = (*(*rq).core).core_pick_in_flight.wrapping_add(1);
        let next;
        if (*(*rq).core).core_pick_seq == (*(*rq).core).core_task_seq
            && (*(*rq).core).core_pick_seq != (*rq).core_sched_seq
            && !(*rq).core_pick.is_null()
        {
            lupos_core_write_once_uint(addr_of_mut!((*rq).core_sched_seq), (*(*rq).core).core_pick_seq);
            next = (*rq).core_pick;
            (*rq).dl_server = (*rq).core_dl_server;
            (*rq).core_pick = null_mut();
            (*rq).core_dl_server = null_mut();
        } else {
            prev_balance(rq, rf);
            let smt_mask = lupos_core_cpu_smt_mask(cpu);
            let mut fi_before = false;
            let mut need_sync = false;
            // C's restart does not reset these state variables, including occupation.
            let mut occ: c_int = 0;
            next = 'restart: loop {
                need_sync |= (*(*rq).core).core_cookie != 0;
                (*(*rq).core).core_cookie = 0;
                if (*(*rq).core).core_forceidle_count != 0 {
                    if !core_clock_updated {
                        update_rq_clock((*rq).core);
                        core_clock_updated = true;
                    }
                    lupos_core_header_sched_core_account_forceidle(rq);
                    (*(*rq).core).core_forceidle_start = 0;
                    (*(*rq).core).core_forceidle_count = 0;
                    (*(*rq).core).core_forceidle_occupation = 0;
                    need_sync = true;
                    fi_before = true;
                }
                (*(*rq).core).core_task_seq = (*(*rq).core).core_task_seq.wrapping_add(1);
                if !need_sync {
                    let picked = pick_task(rq, rf);
                    if picked == lupos_core_retry_task() {
                        core_clock_updated = false;
                        if (*rq).clock_update_flags & LUPOS_CORE_RQCF_UPDATED == 0 {
                            update_rq_clock(rq);
                        }
                        continue 'restart;
                    }
                    if (*picked).core_cookie == 0 {
                        (*rq).core_pick = null_mut();
                        (*rq).core_dl_server = null_mut();
                        lupos_core_warn_once_site_6334(fi_before);
                        task_vruntime_update(rq, picked, false);
                        break 'restart picked;
                    }
                }
                let mut max: *mut task_struct = null_mut();
                let mut i = lupos_core_cpu_next_wrap(-1, smt_mask, cpu);
                while i < nr_cpu_ids as c_int {
                    let rq_i = lupos_core_cpu_rq(i);
                    if i != cpu && (rq_i != (*rq).core || !core_clock_updated) {
                        update_rq_clock(rq_i);
                    }
                    let p = pick_task(rq_i, rf);
                    if p == lupos_core_retry_task() {
                        core_clock_updated = false;
                        if (*rq).clock_update_flags & LUPOS_CORE_RQCF_UPDATED == 0 {
                            update_rq_clock(rq);
                        }
                        continue 'restart;
                    }
                    (*rq_i).core_pick = p;
                    (*rq_i).core_dl_server = (*rq_i).dl_server;
                    if max.is_null() || prio_less(max, p, fi_before) {
                        max = p;
                    }
                    i = lupos_core_cpu_next_wrap(i, smt_mask, cpu);
                }
                let cookie = (*max).core_cookie;
                (*(*rq).core).core_cookie = cookie;
                i = lupos_core_cpu_next(-1, smt_mask);
                while i < nr_cpu_ids as c_int {
                    let rq_i = lupos_core_cpu_rq(i);
                    let mut p = (*rq_i).core_pick;
                    if !cookie_equals(p, cookie) {
                        p = if cookie != 0 {
                            sched_core_find(rq_i, cookie)
                        } else {
                            null_mut()
                        };
                        if p.is_null() {
                            p = idle_sched_class.pick_task.unwrap()(rq_i, rf);
                        }
                    }
                    (*rq_i).core_pick = p;
                    (*rq_i).core_dl_server = null_mut();
                    if p == (*rq_i).idle {
                        if (*rq_i).nr_running != 0 {
                            (*(*rq).core).core_forceidle_count =
                                (*(*rq).core).core_forceidle_count.wrapping_add(1);
                            if !fi_before {
                                (*(*rq).core).core_forceidle_seq =
                                    (*(*rq).core).core_forceidle_seq.wrapping_add(1);
                            }
                        }
                    } else {
                        occ = occ.wrapping_add(1);
                    }
                    i = lupos_core_cpu_next(i, smt_mask);
                }
                if lupos_core_schedstat_enabled() && (*(*rq).core).core_forceidle_count != 0 {
                    (*(*rq).core).core_forceidle_start = lupos_core_rq_clock((*rq).core);
                    (*(*rq).core).core_forceidle_occupation = occ as _;
                }
                (*(*rq).core).core_pick_seq = (*(*rq).core).core_task_seq;
                let picked = (*rq).core_pick;
                (*rq).core_sched_seq = (*(*rq).core).core_pick_seq;
                lupos_core_warn_once_site_6418(picked.is_null());
                // L1TF ordering: publish picks, validate matching cookies, then IPIs.
                i = lupos_core_cpu_next(-1, smt_mask);
                while i < nr_cpu_ids as c_int {
                    let rq_i = lupos_core_cpu_rq(i);
                    let p = (*rq_i).core_pick;
                    if !p.is_null() {
                        if !(fi_before && (*(*rq).core).core_forceidle_count != 0) {
                            task_vruntime_update(rq_i, p, (*(*rq).core).core_forceidle_count != 0);
                        }
                        (*p).core_occupation = occ as _;
                        if i == cpu {
                            (*rq_i).core_pick = null_mut();
                            (*rq_i).core_dl_server = null_mut();
                        } else {
                            lupos_core_warn_once_site_6464(!cookie_match(picked, p));
                            if lupos_core_rq_curr(rq_i) == p {
                                (*rq_i).core_pick = null_mut();
                                (*rq_i).core_dl_server = null_mut();
                            } else {
                                resched_curr(rq_i);
                            }
                        }
                    }
                    i = lupos_core_cpu_next(i, smt_mask);
                }
                break 'restart picked;
            };
        }
        (*(*rq).core).core_pick_in_flight = (*(*rq).core).core_pick_in_flight.wrapping_sub(1);
        lupos_core_put_prev_set_next_task(rq, lupos_core_rq_donor(rq), next);
        if (*(*rq).core).core_forceidle_count != 0 && next == (*rq).idle {
            queue_core_balance(rq);
        }
        next
    }
}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn pick_next_task(rq: *mut rq, rf: *mut rq_flags) -> *mut task_struct {
    // SAFETY: The caller supplies the locked rq and matching flags required
    // by the non-core selection path.
    unsafe {
        __pick_next_task(rq, rf)
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn try_steal_cookie(this_cpu: c_int, that: c_int) -> bool {
    // SAFETY: Valid CPU IDs select permanent runqueues. IRQ exclusion and
    // double-rq locking protect the task accesses; closure exits all reach
    // the matching unlock and IRQ restore below.
    unsafe {
        let dst = lupos_core_cpu_rq(this_cpu);
        let src = lupos_core_cpu_rq(that);
        let flags = lupos_core_local_irq_save();
        double_rq_lock(dst, src);
        let success = (|| {
            let cookie = (*(*dst).core).core_cookie;
            if cookie == 0 || lupos_core_rq_curr(dst) != (*dst).idle {
                return false;
            }
            let mut p = sched_core_find(src, cookie);
            while !p.is_null() {
                if p != (*src).core_pick
                    && p != lupos_core_rq_curr(src)
                    && is_cpu_allowed(p, this_cpu)
                    && (*p).core_occupation <= (*(*dst).idle).core_occupation
                    && sched_task_is_throttled(p, this_cpu) == 0
                {
                    lupos_core_move_queued_task_locked(src, dst, p);
                    resched_curr(dst);
                    return true;
                }
                p = sched_core_next(p, cookie);
            }
            false
        })();
        lupos_core_double_rq_unlock(dst, src);
        lupos_core_local_irq_restore(flags);
        success
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn steal_cookie_task(cpu: c_int, sd: *mut sched_domain) -> bool {
    // SAFETY: The caller pins this CPU and holds RCU over the live domain
    // and its span; try_steal_cookie acquires the participating rq locks.
    unsafe {
        let mask = lupos_core_sched_domain_span(sd);
        let start = cpu.wrapping_add(1);
        let mut i = lupos_core_cpu_next_wrap(-1, mask, start);
        while i < nr_cpu_ids as c_int {
            if i != cpu {
                if lupos_core_need_resched() {
                    break;
                }
                if try_steal_cookie(cpu, i) {
                    return true;
                }
            }
            i = lupos_core_cpu_next_wrap(i, mask, start);
        }
        false
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe extern "C" fn sched_core_balance(rq: *mut rq) {
    // SAFETY: The callback starts with rq locked and IRQs disabled.
    // Preemption and RCU protection span the unlock, domain traversal and
    // relock, preserving the native callback's lifetime and lock contract.
    unsafe {
        let cpu = lupos_core_cpu_of(rq);
        lupos_core_preempt_disable();
        lupos_core_rcu_read_lock();
        lupos_core_raw_spin_rq_unlock_irq(rq);
        let mut sd = lupos_core_sched_domain_first(cpu);
        while !sd.is_null() {
            if lupos_core_need_resched() || steal_cookie_task(cpu, sd) {
                break;
            }
            sd = (*sd).parent;
        }
        lupos_core_raw_spin_rq_lock_irq(rq);
        lupos_core_rcu_read_unlock();
        lupos_core_preempt_enable();
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn queue_core_balance(rq: *mut rq) {
    // SAFETY: The caller holds the live rq's scheduling lock; the native
    // per-CPU callback storage outlives the queued balance callback.
    unsafe {
        if !lupos_core_sched_core_enabled(rq) || (*(*rq).core).core_cookie == 0 || (*rq).nr_running == 0
        {
            return;
        }
        lupos_core_queue_balance_callback(
            rq,
            lupos_core_core_balance_head((*rq).cpu),
            Some(sched_core_balance),
        );
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_cpu_starting(cpu: c_uint) {
    // SAFETY: CPU hotplug keeps the runqueues and topology live.
    // sched_core_lock holds all sibling locks across leader lookup and
    // installation; every closure exit reaches sched_core_unlock.
    unsafe {
        let mask = lupos_core_cpu_smt_mask(cpu as c_int);
        let rq = lupos_core_cpu_rq(cpu as c_int);
        let mut flags: c_ulong = 0;
        sched_core_lock(cpu as c_int, &mut flags);
        (|| {
            lupos_core_warn_once_site_6608((*rq).core != rq);
            if lupos_core_cpumask_weight(mask) == 1 {
                return;
            }
            let mut leader = null_mut();
            let mut t = lupos_core_cpu_next(-1, mask);
            while t < nr_cpu_ids as c_int {
                let trq = lupos_core_cpu_rq(t);
                if t != cpu as c_int && (*trq).core == trq {
                    leader = trq;
                    break;
                }
                t = lupos_core_cpu_next(t, mask);
            }
            if lupos_core_warn_once_site_6624(leader.is_null()) {
                return;
            }
            t = lupos_core_cpu_next(-1, mask);
            while t < nr_cpu_ids as c_int {
                let trq = lupos_core_cpu_rq(t);
                if t == cpu as c_int {
                    (*trq).core = leader;
                }
                lupos_core_warn_once_site_6634((*trq).core != leader);
                t = lupos_core_cpu_next(t, mask);
            }
        })();
        sched_core_unlock(cpu as c_int, &mut flags);
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_cpu_deactivate(cpu: c_uint) {
    // SAFETY: CPU hotplug keeps the runqueues and topology live. All
    // sibling locks cover moving shared state and rebinding the leader;
    // every closure exit reaches sched_core_unlock.
    unsafe {
        let mask = lupos_core_cpu_smt_mask(cpu as c_int);
        let rq = lupos_core_cpu_rq(cpu as c_int);
        let mut flags: c_ulong = 0;
        sched_core_lock(cpu as c_int, &mut flags);
        (|| {
            if lupos_core_cpumask_weight(mask) == 1 {
                lupos_core_warn_once_site_6651((*rq).core != rq);
                return;
            }
            if (*rq).core != rq {
                return;
            }
            let mut leader: *mut rq = null_mut();
            let mut t = lupos_core_cpu_next(-1, mask);
            while t < nr_cpu_ids as c_int {
                if t != cpu as c_int {
                    leader = lupos_core_cpu_rq(t);
                    break;
                }
                t = lupos_core_cpu_next(t, mask);
            }
            if lupos_core_warn_once_site_6667(leader.is_null()) {
                return;
            }
            (*leader).core_task_seq = (*rq).core_task_seq;
            (*leader).core_pick_seq = (*rq).core_pick_seq;
            (*leader).core_cookie = (*rq).core_cookie;
            (*leader).core_forceidle_count = (*rq).core_forceidle_count;
            (*leader).core_forceidle_seq = (*rq).core_forceidle_seq;
            (*leader).core_forceidle_occupation = (*rq).core_forceidle_occupation;
            // e1 authority: move in-flight state; never leave a stale duplicate.
            (*leader).core_pick_in_flight = (*rq).core_pick_in_flight;
            (*rq).core_pick_in_flight = 0;
            (*leader).core_forceidle_start = 0;
            t = lupos_core_cpu_next(-1, mask);
            while t < nr_cpu_ids as c_int {
                (*lupos_core_cpu_rq(t)).core = leader;
                t = lupos_core_cpu_next(t, mask);
            }
        })();
        sched_core_unlock(cpu as c_int, &mut flags);
    }
}
#[cfg(CONFIG_SCHED_CORE)]
unsafe fn sched_core_cpu_dying(cpu: c_uint) {
    // SAFETY: The CPU-hotplug caller serializes the outgoing CPU's
    // runqueue lifetime and leader reset, as required by the native path.
    unsafe {
        let rq = lupos_core_cpu_rq(cpu as c_int);
        if (*rq).core != rq {
            (*rq).core = rq;
        }
    }
}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn sched_core_cpu_starting(_cpu: c_uint) {}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn sched_core_cpu_deactivate(_cpu: c_uint) {}
#[cfg(not(CONFIG_SCHED_CORE))]
unsafe fn sched_core_cpu_dying(_cpu: c_uint) {}
