// SPDX-License-Identifier: GPL-2.0-only
unsafe fn update_rq_clock_task(rq: *mut rq, mut delta: i64) {
    let mut steal: i64 = 0;
    let mut irq_delta: i64 = 0;
    #[cfg(CONFIG_IRQ_TIME_ACCOUNTING)]
    if lupos_core_irqtime_enabled() {
        irq_delta = lupos_core_irq_time_read(lupos_core_cpu_of(rq))
            .wrapping_sub((*rq).prev_irq_time) as i64;
        if irq_delta > delta {
            irq_delta = delta;
        }
        (*rq).prev_irq_time = (*rq).prev_irq_time.wrapping_add(irq_delta as u64);
        delta = delta.wrapping_sub(irq_delta);
        lupos_core_delayacct_irq(lupos_core_rq_curr(rq), irq_delta as u64);
    }
    #[cfg(CONFIG_PARAVIRT_TIME_ACCOUNTING)]
    if lupos_core_paravirt_steal_enabled() {
        let prev_steal = lupos_core_paravirt_steal_clock(lupos_core_cpu_of(rq));
        steal = prev_steal.wrapping_sub((*rq).prev_steal_time_rq) as i64;
        if steal > delta {
            steal = delta;
        }
        (*rq).prev_steal_time_rq = prev_steal;
        delta = delta.wrapping_sub(steal);
    }
    (*rq).clock_task = (*rq).clock_task.wrapping_add(delta as u64);
    #[cfg(CONFIG_HAVE_SCHED_AVG_IRQ)]
    if irq_delta.wrapping_add(steal) != 0 && lupos_core_feat_nontask_capacity() {
        lupos_core_update_irq_load_avg(rq, irq_delta.wrapping_add(steal) as u64);
    }
    lupos_core_update_rq_clock_pelt(rq, delta);
}
#[no_mangle]
pub unsafe extern "C" fn update_rq_clock(rq: *mut rq) {
    lupos_core_assert_rq_held(rq);
    if (*rq).clock_update_flags & LUPOS_CORE_RQCF_ACT_SKIP != 0 {
        return;
    }
    if lupos_core_feat_warn_double_clock() {
        lupos_core_warn_double_clock((*rq).clock_update_flags & LUPOS_CORE_RQCF_UPDATED != 0);
    }
    (*rq).clock_update_flags |= LUPOS_CORE_RQCF_UPDATED;
    let clock = sched_clock_cpu(lupos_core_cpu_of(rq));
    lupos_core_scx_rq_clock_update(rq, clock);
    let delta = clock.wrapping_sub((*rq).clock) as i64;
    if delta < 0 {
        return;
    }
    (*rq).clock = (*rq).clock.wrapping_add(delta as u64);
    update_rq_clock_task(rq, delta);
}
