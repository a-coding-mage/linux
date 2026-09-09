// SPDX-License-Identifier: GPL-2.0
// Translated from sched/pelt.h. C header dependencies are supplied elsewhere.

extern "C" {
    pub fn __update_load_avg_blocked_se(now: u64, se: *mut sched_entity) -> i32;
    pub fn __update_load_avg_se(now: u64, cfs_rq: *mut cfs_rq, se: *mut sched_entity) -> i32;
    pub fn __update_load_avg_cfs_rq(now: u64, cfs_rq: *mut cfs_rq) -> i32;
    pub fn update_rt_rq_load_avg(now: u64, rq: *mut rq, running: i32) -> i32;
    pub fn update_dl_rq_load_avg(now: u64, rq: *mut rq, running: i32) -> i32;
    pub fn update_other_load_avgs(rq: *mut rq) -> bool;
}

// Under CONFIG_SCHED_HW_PRESSURE, update_hw_load_avg is an external function.
#[cfg(feature = "CONFIG_SCHED_HW_PRESSURE")]
extern "C" {
    pub fn update_hw_load_avg(now: u64, rq: *mut rq, capacity: u64) -> i32;
}

#[cfg(not(feature = "CONFIG_SCHED_HW_PRESSURE"))]
#[inline]
pub unsafe fn update_hw_load_avg(_now: u64, _rq: *mut rq, _capacity: u64) -> i32 { 0 }

#[cfg(feature = "CONFIG_SCHED_HW_PRESSURE")]
#[inline]
pub unsafe fn hw_load_avg(rq: *mut rq) -> u64 {
    core::ptr::read_volatile(&(*rq).avg_hw.load_avg)
}

#[cfg(not(feature = "CONFIG_SCHED_HW_PRESSURE"))]
#[inline]
pub unsafe fn hw_load_avg(_rq: *mut rq) -> u64 { 0 }

// Under CONFIG_HAVE_SCHED_AVG_IRQ, update_irq_load_avg is an external function.
#[cfg(feature = "CONFIG_HAVE_SCHED_AVG_IRQ")]
extern "C" {
    pub fn update_irq_load_avg(rq: *mut rq, running: u64) -> i32;
}

#[cfg(not(feature = "CONFIG_HAVE_SCHED_AVG_IRQ"))]
#[inline]
pub unsafe fn update_irq_load_avg(_rq: *mut rq, _running: u64) -> i32 { 0 }

pub const PELT_MIN_DIVIDER: u32 = LOAD_AVG_MAX - 1024;

#[inline]
pub unsafe fn get_pelt_divider(avg: *mut sched_avg) -> u32 {
    PELT_MIN_DIVIDER + (*avg).period_contrib
}

#[inline]
pub unsafe fn cfs_se_util_change(avg: *mut sched_avg) {
    let mut enqueued: u32;
    if !sched_feat(UTIL_EST) { return; }
    enqueued = (*avg).util_est;
    if (enqueued & UTIL_AVG_UNCHANGED) == 0 { return; }
    enqueued &= !UTIL_AVG_UNCHANGED;
    core::ptr::write_volatile(&mut (*avg).util_est, enqueued);
}

#[inline]
pub unsafe fn rq_clock_pelt(rq: *mut rq) -> u64 {
    lockdep_assert_rq_held(rq);
    assert_clock_updated(rq);
    (*rq).clock_pelt - (*rq).lost_idle_time
}

#[inline]
pub unsafe fn _update_idle_rq_clock_pelt(rq: *mut rq) {
    (*rq).clock_pelt = rq_clock_task(rq);
    u64_u32_store(&mut (*rq).clock_idle, rq_clock(rq));
    // Paired with smp_rmb in migrate_se_pelt_lag().
    smp_wmb();
    u64_u32_store(&mut (*rq).clock_pelt_idle, rq_clock_pelt(rq));
}

/*
 * The clock_pelt scales the time to reflect the effective amount of
 * computation done during the running delta time but then sync back to
 * clock_task when rq is idle.
 */
#[inline]
pub unsafe fn update_rq_clock_pelt(rq: *mut rq, mut delta: i64) {
    if unlikely(is_idle_task((*rq).curr)) {
        _update_idle_rq_clock_pelt(rq);
        return;
    }
    delta = cap_scale(delta, arch_scale_cpu_capacity(cpu_of(rq)));
    delta = cap_scale(delta, arch_scale_freq_capacity(cpu_of(rq)));
    (*rq).clock_pelt = (*rq).clock_pelt.wrapping_add(delta as u64);
}

#[inline]
pub unsafe fn update_idle_rq_clock_pelt(rq: *mut rq) {
    let divider: u32 = ((LOAD_AVG_MAX - 1024) << SCHED_CAPACITY_SHIFT) - LOAD_AVG_MAX;
    let mut util_sum = (*rq).cfs.avg.util_sum;
    util_sum += (*rq).avg_rt.util_sum;
    util_sum += (*rq).avg_dl.util_sum;
    if util_sum >= divider {
        (*rq).lost_idle_time += rq_clock_task(rq) - (*rq).clock_pelt;
    }
    _update_idle_rq_clock_pelt(rq);
}

#[cfg(feature = "CONFIG_CFS_BANDWIDTH")]
#[inline]
pub unsafe fn update_idle_cfs_rq_clock_pelt(cfs_rq: *mut cfs_rq) {
    let throttled = if unlikely((*cfs_rq).pelt_clock_throttled) {
        u64::MAX
    } else {
        (*cfs_rq).throttled_clock_pelt_time
    };
    u64_u32_store(&mut (*cfs_rq).throttled_pelt_idle, throttled);
}

#[cfg(feature = "CONFIG_CFS_BANDWIDTH")]
#[inline]
pub unsafe fn cfs_rq_clock_pelt(cfs_rq: *mut cfs_rq) -> u64 {
    if unlikely((*cfs_rq).pelt_clock_throttled) {
        return (*cfs_rq).throttled_clock_pelt - (*cfs_rq).throttled_clock_pelt_time;
    }
    rq_clock_pelt(rq_of(cfs_rq)) - (*cfs_rq).throttled_clock_pelt_time
}

#[cfg(not(feature = "CONFIG_CFS_BANDWIDTH"))]
#[inline]
pub unsafe fn update_idle_cfs_rq_clock_pelt(_cfs_rq: *mut cfs_rq) {}

#[cfg(not(feature = "CONFIG_CFS_BANDWIDTH"))]
#[inline]
pub unsafe fn cfs_rq_clock_pelt(cfs_rq: *mut cfs_rq) -> u64 {
    rq_clock_pelt(rq_of(cfs_rq))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
