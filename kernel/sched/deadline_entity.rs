// SPDX-License-Identifier: GPL-2.0
// Newly reconstructed from the unchanged deadline.c oracle, not restored bodies.
unsafe fn task_non_contending(se: *mut b::sched_dl_entity, is_dl_task: bool) {
    // SAFETY: The live entity's rq is locked; task lifetime and root-domain state survive timer/accounting changes.
    unsafe {
        let timer = addr_of_mut!((*se).inactive_timer);
        let rq = rq_of_dl_se(se);
        let dl_rq = addr_of_mut!((*rq).dl);
        if (*se).dl_runtime == 0 || b::rust_dl_entity_is_special(se) { return; }
        b::rust_dl_warn_non_contending((*se).dl_non_contending() != 0);
        let product = ((*se).runtime as u64).wrapping_mul((*se).dl_period) as i64;
        let fraction = b::rust_dl_div64_long(product, (*se).dl_runtime as _);
        let zerolag = (*se).deadline.wrapping_sub(fraction as u64)
            .wrapping_sub(b::rust_dl_rq_clock(rq)) as i64;
        if zerolag < 0 || b::rust_dl_hrtimer_active(timer) {
            if b::rust_dl_server(se) { sub_running_bw(se, dl_rq); }
            else {
                let p = b::rust_dl_task_of(se);
                if is_dl_task { sub_running_bw(se, dl_rq); }
                if !is_dl_task || b::rust_dl_task_state(p) == b::TASK_DEAD as c_uint {
                    let bw = dl_bw_of(b::rust_dl_task_cpu(p));
                    if b::rust_dl_task_state(p) == b::TASK_DEAD as c_uint { sub_rq_bw(se, dl_rq); }
                    b::rust_dl_raw_spin_lock(addr_of_mut!((*bw).lock));
                    __dl_sub(bw, (*se).dl_bw, dl_bw_cpus(b::rust_dl_task_cpu(p)));
                    b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
                    __dl_clear_params(se);
                }
            }
            return;
        }
        (*se).set_dl_non_contending(1);
        if !b::rust_dl_server(se) { b::rust_dl_get_task_struct(b::rust_dl_task_of(se)); }
        b::rust_dl_hrtimer_start_relative_hard(timer, zerolag);
    }
}
unsafe fn task_contending(se: *mut b::sched_dl_entity, flags: c_int) {
    // SAFETY: The live entity's destination rq is locked across timer cancellation and utilization changes.
    unsafe {
        let rq = dl_rq_of_se(se);
        if (*se).dl_runtime == 0 { return; }
        if flags & b::ENQUEUE_MIGRATED as c_int != 0 { add_rq_bw(se, rq); }
        if (*se).dl_non_contending() != 0 {
            (*se).set_dl_non_contending(0);
            cancel_inactive_timer(se);
        } else { add_running_bw(se, rq); }
    }
}
#[inline]
unsafe fn is_leftmost(se: *mut b::sched_dl_entity, rq: *mut b::dl_rq) -> c_int {
    // SAFETY: Both pointers are live and the dl runqueue's tree is serialized by its rq lock.
    unsafe {
        (b::rust_dl_rb_first_cached(addr_of!((*rq).root)) == addr_of_mut!((*se).rb_node)) as c_int
    }
}
#[inline]
unsafe fn replenish_dl_new_period(se: *mut b::sched_dl_entity, rq: *mut b::rq) {
    // SAFETY: The live entity, its PI source and rq clock are protected by the owning rq lock.
    unsafe {
        (*se).deadline = b::rust_dl_rq_clock(rq).wrapping_add((*pi_of(se)).dl_deadline);
        (*se).runtime = (*pi_of(se)).dl_runtime as i64;
        if (*se).dl_defer() != 0 && (*se).dl_defer_running() == 0 {
            (*se).set_dl_throttled(1);
            (*se).set_dl_defer_armed(1);
        }
        b::rust_dl_trace_replenish(se, b::rust_dl_cpu_of(rq), dl_get_type(se, rq));
    }
}
#[inline]
unsafe fn setup_new_dl_entity(se: *mut b::sched_dl_entity) {
    // SAFETY: The live entity's rq lock protects the clock, timer state and PI reservation.
    unsafe {
        let rq = rq_of_dl_rq(dl_rq_of_se(se));
        b::rust_dl_warn_setup_boosted(is_dl_boosted(se));
        b::rust_dl_warn_setup_future(b::rust_dl_time_before(b::rust_dl_rq_clock(rq), (*se).deadline));
        if (*se).dl_throttled() != 0 { return; }
        replenish_dl_new_period(se, rq);
    }
}
unsafe fn dl_entity_overflow(se: *mut b::sched_dl_entity, t: u64) -> bool {
    // SAFETY: The live entity and its priority-inheritance reservation are serialized by its rq lock.
    unsafe {
        // The signed remaining runtime is shifted before C converts it to u64.
        let left = ((*pi_of(se)).dl_deadline >> b::DL_SCALE)
            .wrapping_mul(((*se).runtime >> b::DL_SCALE) as u64);
        let right = ((*se).deadline.wrapping_sub(t) >> b::DL_SCALE)
            .wrapping_mul((*pi_of(se)).dl_runtime >> b::DL_SCALE);
        b::rust_dl_time_before(right, left)
    }
}
unsafe fn update_dl_revised_wakeup(se: *mut b::sched_dl_entity, rq: *mut b::rq) {
    // SAFETY: The locked rq owns the live entity and provides a valid serialized rq clock.
    unsafe {
        let laxity = (*se).deadline.wrapping_sub(b::rust_dl_rq_clock(rq));
        b::rust_dl_warn_revised_past(b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq)));
        (*se).runtime = ((*se).dl_density.wrapping_mul(laxity) >> b::BW_SHIFT) as i64;
    }
}
unsafe fn update_dl_entity(se: *mut b::sched_dl_entity) {
    // SAFETY: The entity and its PI source remain live under the owning rq lock throughout wakeup.
    unsafe {
        let rq = rq_of_dl_se(se);
        if b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq))
            || dl_entity_overflow(se, b::rust_dl_rq_clock(rq)) {
            if b::rust_dl_unlikely_revised_wakeup((!b::rust_dl_is_implicit(se)
                    || ((*se).dl_defer() != 0 && (*se).dl_defer_running() != 0))
                && !b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq))
                && !is_dl_boosted(se)) {
                update_dl_revised_wakeup(se, rq);
                return;
            }
            (*se).set_dl_defer_running(0);
            replenish_dl_new_period(se, rq);
        } else if b::rust_dl_server(se) && (*se).dl_defer() != 0 {
            if (*se).dl_defer_running() == 0 {
                (*se).set_dl_defer_armed(1);
                (*se).set_dl_throttled(1);
            }
        }
    }
}
#[inline]
unsafe fn dl_next_period(se: *mut b::sched_dl_entity) -> u64 {
    // SAFETY: The entity is live and the caller serializes its deadline and period fields.
    unsafe {
        (*se).deadline.wrapping_sub((*se).dl_deadline).wrapping_add((*se).dl_period)
    }
}
unsafe fn start_dl_timer(se: *mut b::sched_dl_entity) -> c_int {
    // SAFETY: The entity's rq lock is held; its initialized timer and task/server lifetime are valid.
    unsafe {
        let timer = addr_of_mut!((*se).dl_timer);
        let rq = rq_of_dl_rq(dl_rq_of_se(se));
        b::rust_dl_lockdep_assert_rq_held(rq);
        let act_ns = if (*se).dl_defer_armed() != 0 {
            b::rust_dl_warn_timer_not_throttled((*se).dl_throttled() == 0);
            (*se).deadline.wrapping_sub((*se).runtime as u64)
        } else { dl_next_period(se) };
        let now = b::rust_dl_ktime_get();
        let delta = (b::rust_dl_ktime_to_ns(now) as u64).wrapping_sub(b::rust_dl_rq_clock(rq)) as i64;
        let act = b::rust_dl_ktime_add_ns(b::rust_dl_ns_to_ktime(act_ns), delta as u64);
        if b::rust_dl_ktime_us_delta(act, now) < 0 { return 0; }
        if !b::rust_dl_hrtimer_is_queued(timer) {
            if !b::rust_dl_server(se) { b::rust_dl_get_task_struct(b::rust_dl_task_of(se)); }
            b::rust_dl_hrtimer_start_absolute_hard(timer, act);
        }
        1
    }
}
unsafe fn replenish_dl_entity(se: *mut b::sched_dl_entity) {
    // SAFETY: The live entity and its PI source are protected by the rq lock across replenishment and timer changes.
    unsafe {
        let rq = rq_of_dl_rq(dl_rq_of_se(se));
        b::rust_dl_warn_replenish_zero((*pi_of(se)).dl_runtime == 0);
        if (*se).dl_deadline == 0
            || ((*se).dl_defer_armed() != 0 && dl_entity_overflow(se, b::rust_dl_rq_clock(rq))) {
            (*se).deadline = b::rust_dl_rq_clock(rq).wrapping_add((*pi_of(se)).dl_deadline);
            (*se).runtime = (*pi_of(se)).dl_runtime as i64;
        }
        if (*se).dl_yielded() != 0 && (*se).runtime > 0 { (*se).runtime = 0; }
        while (*se).runtime <= 0 {
            (*se).deadline = (*se).deadline.wrapping_add((*pi_of(se)).dl_period);
            (*se).runtime = ((*se).runtime as u64).wrapping_add((*pi_of(se)).dl_runtime) as i64;
        }
        if b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq)) {
            b::rust_dl_print_replenish_lagged();
            replenish_dl_new_period(se, rq);
        }
        if (*se).dl_yielded() != 0 { (*se).set_dl_yielded(0); }
        if (*se).dl_throttled() != 0 { (*se).set_dl_throttled(0); }
        b::rust_dl_trace_replenish(se, b::rust_dl_cpu_of(rq), dl_get_type(se, rq));
        if (*se).dl_defer_armed() != 0 {
            (*se).set_dl_defer_armed(0);
            return;
        }
        if (*se).dl_defer() != 0 && (*se).dl_defer_running() == 0
            && b::rust_dl_time_before(b::rust_dl_rq_clock((*se).rq), (*se).deadline.wrapping_sub((*se).runtime as u64)) {
            if !is_dl_boosted(se) {
                (*se).set_dl_defer_armed(1);
                (*se).set_dl_throttled(1);
                if start_dl_timer(se) == 0 {
                    b::hrtimer_try_to_cancel(addr_of_mut!((*se).dl_timer));
                    (*se).set_dl_defer_armed(0);
                    (*se).set_dl_throttled(0);
                }
            }
        }
    }
}
#[inline]
unsafe fn dl_check_constrained_dl(se: *mut b::sched_dl_entity) {
    // SAFETY: The live entity's rq lock protects its clock comparison, timer and throttling state.
    unsafe {
        let rq = rq_of_dl_se(se);
        if b::rust_dl_time_before((*se).deadline, b::rust_dl_rq_clock(rq))
            && b::rust_dl_time_before(b::rust_dl_rq_clock(rq), dl_next_period(se)) {
            if b::rust_dl_unlikely_constrained_boost(is_dl_boosted(se) || start_dl_timer(se) == 0) { return; }
            b::rust_dl_trace_throttle(se, b::rust_dl_cpu_of(rq), dl_get_type(se, rq));
            (*se).set_dl_throttled(1);
            if (*se).runtime > 0 { (*se).runtime = 0; }
        }
    }
}
unsafe fn dl_runtime_exceeded(se: *mut b::sched_dl_entity) -> c_int {
    // SAFETY: The caller holds the live entity's runqueue lock while reading its remaining runtime.
    unsafe { ((*se).runtime <= 0) as c_int }
}
unsafe fn grub_reclaim(delta: u64, rq: *mut b::rq, se: *mut b::sched_dl_entity) -> u64 {
    // SAFETY: The rq lock protects the live reservation and the runqueue's running/assigned bandwidth.
    unsafe {
        let inactive = (*rq).dl.this_bw.wrapping_sub((*rq).dl.running_bw);
        let active = if inactive.wrapping_add((*rq).dl.extra_bw) > (*rq).dl.max_bw.wrapping_sub((*se).dl_bw) {
            (*se).dl_bw
        } else { (*rq).dl.max_bw.wrapping_sub(inactive).wrapping_sub((*rq).dl.extra_bw) };
        let active = active.wrapping_mul((*rq).dl.bw_ratio) >> b::RATIO_SHIFT;
        delta.wrapping_mul(active) >> b::BW_SHIFT
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_scaled_delta_exec(rq: *mut b::rq, se: *mut b::sched_dl_entity, delta: i64) -> i64 {
    // SAFETY: The live rq and entity are locked for accounting; CPU capacity primitives receive a valid CPU.
    unsafe {
        if b::rust_dl_unlikely_reclaim((*se).flags & b::SCHED_FLAG_RECLAIM as u32 != 0) { grub_reclaim(delta as u64, rq, se) as i64 }
        else {
            let cpu = b::rust_dl_cpu_of(rq);
            let scale_freq = b::rust_dl_arch_scale_freq_capacity(cpu);
            let scale_cpu = b::rust_dl_arch_scale_cpu_capacity(cpu);
            let scaled = b::rust_dl_cap_scale(delta as u64, scale_freq);
            b::rust_dl_cap_scale(scaled, scale_cpu) as i64
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __setparam_dl(p: *mut b::task_struct, attr: *const b::sched_attr) {
    // SAFETY: The policy-change caller owns the live task's scheduler state and a readable attribute record.
    unsafe {
        let se = addr_of_mut!((*p).dl);
        (*se).dl_runtime = (*attr).sched_runtime;
        (*se).dl_deadline = (*attr).sched_deadline;
        (*se).dl_period = if (*attr).sched_period != 0 { (*attr).sched_period } else { (*se).dl_deadline };
        (*se).flags = ((*attr).sched_flags & b::SCHED_DL_FLAGS as u64) as c_uint;
        (*se).dl_bw = b::to_ratio((*se).dl_period, (*se).dl_runtime);
        (*se).dl_density = b::to_ratio((*se).dl_deadline, (*se).dl_runtime);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __checkparam_dl(attr: *const b::sched_attr) -> bool {
    // SAFETY: The attribute record is readable; concurrent period-limit reads use native READ_ONCE leaves.
    unsafe {
        if (*attr).sched_flags & b::SCHED_FLAG_SUGOV as u64 != 0 { return true; }
        if (*attr).sched_deadline == 0 || (*attr).sched_runtime < (1u64 << b::DL_SCALE) { return false; }
        if (*attr).sched_deadline & (1u64 << 63) != 0 || (*attr).sched_period & (1u64 << 63) != 0 { return false; }
        let period = if (*attr).sched_period == 0 { (*attr).sched_deadline } else { (*attr).sched_period };
        if period < (*attr).sched_deadline || (*attr).sched_deadline < (*attr).sched_runtime { return false; }
        let max = b::rust_dl_read_period_max() as u64 * b::NSEC_PER_USEC as u64;
        let min = b::rust_dl_read_period_min() as u64 * b::NSEC_PER_USEC as u64;
        period >= min && period <= max
    }
}
unsafe fn __dl_clear_params(se: *mut b::sched_dl_entity) {
    // SAFETY: The caller exclusively owns or locks the live entity and serializes its timer/PI state.
    unsafe {
        (*se).dl_runtime = 0;
        (*se).dl_deadline = 0;
        (*se).dl_period = 0;
        (*se).flags = 0;
        (*se).dl_bw = 0;
        (*se).dl_density = 0;
        (*se).set_dl_throttled(0);
        (*se).set_dl_yielded(0);
        (*se).set_dl_non_contending(0);
        (*se).set_dl_overrun(0);
        (*se).set_dl_server(0);
        (*se).set_dl_defer(0);
        (*se).set_dl_defer_running(0);
        (*se).set_dl_defer_armed(0);
        // Do not clear other bitfields: the original owner does not clear them here.
        #[cfg(CONFIG_RT_MUTEXES)]
        { (*se).pi_se = se; }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_param_changed(p: *mut b::task_struct, attr: *const b::sched_attr) -> bool {
    // SAFETY: The task and attribute record are live and the caller serializes reservation parameter changes.
    unsafe {
        let se = addr_of!((*p).dl);
        (*se).dl_runtime != (*attr).sched_runtime || (*se).dl_deadline != (*attr).sched_deadline
            || (*se).dl_period != (*attr).sched_period
            || (*se).flags as u64 != (*attr).sched_flags & b::SCHED_DL_FLAGS as u64
    }
}
#[no_mangle]
pub unsafe extern "C" fn dl_server_apply_params(se: *mut b::sched_dl_entity, runtime: u64, period: u64, init: bool) -> c_int {
    // SAFETY: The caller locks the server rq and stabilizes its root domain; the nested bandwidth lock protects totals.
    unsafe {
        let old_bw = if init || (*se).dl_bw_attached() == 0 { 0 }
            else { b::to_ratio((*se).dl_period, (*se).dl_runtime) };
        let new_bw = b::to_ratio(period, runtime);
        let rq = (*se).rq;
        let cpu = b::rust_dl_cpu_of(rq);
        let bw = dl_bw_of(cpu);
        b::rust_dl_raw_spin_lock(addr_of_mut!((*bw).lock));
        let cpus = dl_bw_cpus(cpu);
        let cap = dl_bw_capacity(cpu);
        if __dl_overflow(bw, cap, old_bw, new_bw) {
            b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
            return -(b::EBUSY as c_int);
        }
        if init {
            __add_rq_bw(new_bw, addr_of_mut!((*rq).dl));
            __dl_add(bw, new_bw, cpus);
            (*se).set_dl_bw_attached(1);
        } else if (*se).dl_bw_attached() != 0 {
            __dl_sub(bw, (*se).dl_bw, cpus);
            __dl_add(bw, new_bw, cpus);
            dl_rq_change_utilization(rq, se, new_bw);
        }
        (*se).dl_runtime = runtime;
        (*se).dl_deadline = period;
        (*se).dl_period = period;
        (*se).runtime = 0;
        (*se).deadline = 0;
        (*se).dl_bw = b::to_ratio((*se).dl_period, (*se).dl_runtime);
        (*se).dl_density = b::to_ratio((*se).dl_deadline, (*se).dl_runtime);
        b::rust_dl_raw_spin_unlock(addr_of_mut!((*bw).lock));
        0
    }
}
