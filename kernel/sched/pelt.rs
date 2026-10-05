// SPDX-License-Identifier: GPL-2.0
/*
 * Per Entity Load Tracking (PELT)
 *
 * This file is a source-level Rust translation of pelt.c.
 * Continued from the existing translation at the pinned baseline
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Native configured headers own
 * layouts, constants, lookup data and primitive semantics.
 */

compile_error!("SOURCE ONLY HOLD: PELT native ABI/accounting admission is pending");

use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings::sched_pelt_native as native;
use kernel::ffi::{
    c_int,
    c_ulong, //
};
use native::{cfs_rq, rq, sched_avg, sched_entity};

unsafe fn decay_load(mut val: u64, n: u64) -> u64 {
    // SAFETY: The factor index is reduced into the native decay table's period.
    unsafe {
        let period = native::LUPOS_PELT_LOAD_AVG_PERIOD;
        if native::lupos_pelt_unlikely_decay_zero(n > (period as u64) * 63) {
            return 0;
        }

        let mut local_n = n as u32;
        if native::lupos_pelt_unlikely_decay_period(local_n >= period) {
            val >>= local_n / period;
            local_n %= period;
        }

        native::lupos_pelt_mul_u64_u32_shr(
            val,
            native::lupos_pelt_decay_factor(local_n),
            32,
        )
    }
}

unsafe fn __accumulate_pelt_segments(periods: u64, d1: u32, d3: u32) -> u32 {
    // SAFETY: decay_load bounds and reduces its own table index.
    unsafe {
        let c1 = decay_load(d1 as u64, periods) as u32;
        let max = native::LUPOS_PELT_LOAD_AVG_MAX as u64;
        let c2 = max.wrapping_sub(decay_load(max, periods)).wrapping_sub(1024) as u32;
        c1.wrapping_add(c2).wrapping_add(d3)
    }
}

unsafe fn accumulate_sum(
    mut delta: u64,
    sa: *mut sched_avg,
    load: c_ulong,
    runnable: c_ulong,
    running: c_int,
) -> u32 {
    // SAFETY: The caller supplies a live average and native PELT serialization.
    // Keep shared scheduler objects as raw pointers, without exclusive references.
    unsafe {
        let mut contrib = delta as u32;
        delta = delta.wrapping_add((*sa).period_contrib as u64);
        let periods = delta / 1024;

        if periods != 0 {
            (*sa).load_sum = decay_load((*sa).load_sum, periods);
            (*sa).runnable_sum = decay_load((*sa).runnable_sum, periods);
            (*sa).util_sum = decay_load((*sa).util_sum as u64, periods) as u32;

            delta %= 1024;
            if load != 0 {
                contrib = __accumulate_pelt_segments(
                    periods,
                    1024u32.wrapping_sub((*sa).period_contrib),
                    delta as u32,
                );
            }
        }
        (*sa).period_contrib = delta as u32;

        // C evaluates these products and the runnable shift in unsigned long
        // before adding to the u64 sums. Widening first changes 32-bit overflow.
        if load != 0 {
            (*sa).load_sum = (*sa).load_sum
                .wrapping_add(load.wrapping_mul(contrib as c_ulong) as u64);
        }
        if runnable != 0 {
            (*sa).runnable_sum = (*sa).runnable_sum.wrapping_add(
                (runnable.wrapping_mul(contrib as c_ulong)
                    << native::LUPOS_PELT_SCHED_CAPACITY_SHIFT) as u64,
            );
        }
        if running != 0 {
            (*sa).util_sum = (*sa).util_sum
                .wrapping_add(contrib << native::LUPOS_PELT_SCHED_CAPACITY_SHIFT);
        }

        periods as u32
    }
}

unsafe fn ___update_load_sum(
    now: u64,
    sa: *mut sched_avg,
    load: c_ulong,
    mut runnable: c_ulong,
    mut running: c_int,
) -> c_int {
    // SAFETY: The caller supplies a live average and native PELT serialization.
    unsafe {
        let mut delta = now.wrapping_sub((*sa).last_update_time);
        if (delta as i64) < 0 {
            (*sa).last_update_time = now;
            return 0;
        }

        delta >>= 10;
        if delta == 0 {
            return 0;
        }
        (*sa).last_update_time = (*sa).last_update_time.wrapping_add(delta << 10);

        if load == 0 {
            runnable = 0;
            running = 0;
        }

        if accumulate_sum(delta, sa, load, runnable, running) == 0 {
            return 0;
        }
        1
    }
}

unsafe fn ___update_load_avg(sa: *mut sched_avg, load: c_ulong) {
    // SAFETY: The caller supplies a live average with a valid period contribution
    // and native PELT serialization. WRITE_ONCE retains the native shared-store
    // semantics for util_avg; a Rust volatile substitute is not used.
    unsafe {
        let divider = native::LUPOS_PELT_MIN_DIVIDER.wrapping_add((*sa).period_contrib);
        (*sa).load_avg = native::lupos_pelt_div_u64(
            (load as u64).wrapping_mul((*sa).load_sum), divider,
        ) as c_ulong;
        (*sa).runnable_avg = native::lupos_pelt_div_u64(
            (*sa).runnable_sum, divider,
        ) as c_ulong;
        native::lupos_pelt_write_util_avg(sa, ((*sa).util_sum / divider) as c_ulong);
    }
}

unsafe fn cfs_se_util_change(sa: *mut sched_avg) {
    // SAFETY: The caller supplies the live average and the native update context.
    unsafe {
        if !native::lupos_pelt_util_est_enabled() {
            return;
        }
        let enqueued = (*sa).util_est;
        if enqueued & native::LUPOS_PELT_UTIL_AVG_UNCHANGED == 0 {
            return;
        }
        native::lupos_pelt_write_util_est(
            sa, enqueued & !native::LUPOS_PELT_UTIL_AVG_UNCHANGED,
        );
    }
}

unsafe fn rq_clock_pelt(rq: *mut rq) -> u64 {
    // SAFETY: The caller holds the runqueue lock and has updated its clock.
    unsafe {
        native::lupos_pelt_lockdep_assert_rq_held(rq);
        native::lupos_pelt_assert_clock_updated(rq);
        (*rq).clock_pelt.wrapping_sub((*rq).lost_idle_time as u64)
    }
}

/// Update a blocked entity's PELT averages.
///
/// # Safety
/// The caller must keep the entity live and serialize its averages according to
/// the native scheduler's blocked-load and entity ownership rules.
#[no_mangle]
pub unsafe extern "C" fn __update_load_avg_blocked_se(
    now: u64, se: *mut sched_entity,
) -> c_int {
    // SAFETY: The caller supplies native entity lifetime and update serialization.
    unsafe {
        let avg = addr_of_mut!((*se).avg);
        if ___update_load_sum(now, avg, 0, 0, 0) != 0 {
            ___update_load_avg(avg, native::lupos_pelt_se_weight(se) as c_ulong);
            native::lupos_pelt_trace_se(se);
            return 1;
        }
        0
    }
}

/// Update an entity's PELT sums and averages on its CFS runqueue.
///
/// # Safety
/// Both pointers must be live and satisfy the native entity/runqueue ownership,
/// locking, clock and serialization requirements of __update_load_avg_se.
#[no_mangle]
pub unsafe extern "C" fn __update_load_avg_se(
    now: u64, cfs_rq: *mut cfs_rq, se: *mut sched_entity,
) -> c_int {
    // SAFETY: The caller supplies native lifetimes and PELT update serialization.
    unsafe {
        let avg = addr_of_mut!((*se).avg);
        if ___update_load_sum(
            now,
            avg,
            ((*se).on_rq != 0) as c_ulong,
            native::lupos_pelt_se_runnable(se) as c_ulong,
            ((*cfs_rq).h_curr == se) as c_int,
        ) != 0 {
            ___update_load_avg(avg, native::lupos_pelt_se_weight(se) as c_ulong);
            cfs_se_util_change(avg);
            native::lupos_pelt_trace_se(se);
            return 1;
        }
        0
    }
}

/// Update a CFS runqueue's PELT sums and averages.
///
/// # Safety
/// The caller must keep the runqueue live and satisfy its native PELT locking,
/// clock and update serialization requirements.
#[no_mangle]
pub unsafe extern "C" fn __update_load_avg_cfs_rq(
    now: u64, cfs_rq: *mut cfs_rq,
) -> c_int {
    // SAFETY: The caller supplies native runqueue lifetime and serialization.
    unsafe {
        let avg = addr_of_mut!((*cfs_rq).avg);
        if ___update_load_sum(
            now,
            avg,
            native::lupos_pelt_scale_load_down((*cfs_rq).load.weight),
            (*cfs_rq).h_nr_runnable as c_ulong,
            (!(*cfs_rq).h_curr.is_null()) as c_int,
        ) != 0 {
            ___update_load_avg(avg, 1);
            native::lupos_pelt_trace_cfs(cfs_rq);
            return 1;
        }
        0
    }
}

/// Update the runqueue's realtime PELT accounting.
///
/// # Safety
/// The caller must supply a live, locked runqueue and the native clock/update
/// context used by update_rt_rq_load_avg.
#[no_mangle]
pub unsafe extern "C" fn update_rt_rq_load_avg(
    now: u64, rq: *mut rq, running: c_int,
) -> c_int {
    // SAFETY: The caller supplies native runqueue lifetime and serialization.
    unsafe {
        let avg = addr_of_mut!((*rq).avg_rt);
        if ___update_load_sum(now, avg, running as c_ulong, running as c_ulong, running) != 0 {
            ___update_load_avg(avg, 1);
            native::lupos_pelt_trace_rt(rq);
            return 1;
        }
        0
    }
}

/// Update the runqueue's deadline PELT accounting.
///
/// # Safety
/// The caller must supply a live, locked runqueue and the native clock/update
/// context used by update_dl_rq_load_avg.
#[no_mangle]
pub unsafe extern "C" fn update_dl_rq_load_avg(
    now: u64, rq: *mut rq, running: c_int,
) -> c_int {
    // SAFETY: The caller supplies native runqueue lifetime and serialization.
    unsafe {
        let avg = addr_of_mut!((*rq).avg_dl);
        if ___update_load_sum(now, avg, running as c_ulong, running as c_ulong, running) != 0 {
            ___update_load_avg(avg, 1);
            native::lupos_pelt_trace_dl(rq);
            return 1;
        }
        0
    }
}

#[cfg(CONFIG_SCHED_HW_PRESSURE)]
/// Update the runqueue's hardware-pressure PELT accounting.
///
/// # Safety
/// The caller must supply a live, locked runqueue and the native clock/update
/// context used by update_hw_load_avg.
#[no_mangle]
pub unsafe extern "C" fn update_hw_load_avg(
    now: u64, rq: *mut rq, capacity: u64,
) -> c_int {
    // SAFETY: The caller supplies native runqueue lifetime and serialization.
    unsafe {
        let avg = addr_of_mut!((*rq).avg_hw);
        if ___update_load_sum(now, avg, capacity as c_ulong, capacity as c_ulong, capacity as c_int) != 0 {
            ___update_load_avg(avg, 1);
            native::lupos_pelt_trace_hw(rq);
            return 1;
        }
        0
    }
}

#[cfg(CONFIG_HAVE_SCHED_AVG_IRQ)]
/// Update IRQ accounting, decaying non-IRQ time before adding IRQ time.
///
/// # Safety
/// The caller must supply a live, locked runqueue with its clock updated and the
/// IRQ-duration invariants required by the native update_irq_load_avg path.
#[no_mangle]
pub unsafe extern "C" fn update_irq_load_avg(rq: *mut rq, mut running: u64) -> c_int {
    // SAFETY: The caller supplies native runqueue lifetime and serialization.
    unsafe {
        running = running.wrapping_mul(native::lupos_pelt_freq_capacity((*rq).cpu))
            >> native::LUPOS_PELT_SCHED_CAPACITY_SHIFT;
        running = running.wrapping_mul(native::lupos_pelt_cpu_capacity((*rq).cpu))
            >> native::LUPOS_PELT_SCHED_CAPACITY_SHIFT;
        let avg = addr_of_mut!((*rq).avg_irq);
        let mut ret = ___update_load_sum((*rq).clock.wrapping_sub(running), avg, 0, 0, 0);
        ret += ___update_load_sum((*rq).clock, avg, 1, 1, 1);
        if ret != 0 {
            ___update_load_avg(avg, 1);
            native::lupos_pelt_trace_irq(rq);
        }
        ret
    }
}

/// Update every non-fair PELT signal, even when an earlier update changed it.
///
/// # Safety
/// The runqueue and donor must remain live, the runqueue lock must be held and
/// its clock must be updated, as required by the native entry point.
#[no_mangle]
pub unsafe extern "C" fn update_other_load_avgs(rq: *mut rq) -> bool {
    // SAFETY: The caller supplies the native runqueue/donor lifetimes, lock and
    // clock contract; the lock also serializes the nested accounting updates.
    unsafe {
        let now = rq_clock_pelt(rq);
        let curr_class = native::lupos_pelt_donor_class(rq);
        let hw_pressure = native::lupos_pelt_hw_pressure((*rq).cpu);
        native::lupos_pelt_lockdep_assert_rq_held(rq);

        let mut changed = update_rt_rq_load_avg(
            now, rq, (curr_class == addr_of!(native::rt_sched_class)) as c_int,
        );
        changed |= update_dl_rq_load_avg(
            now, rq, (curr_class == addr_of!(native::dl_sched_class)) as c_int,
        );
        // Even the !CONFIG_SCHED_HW_PRESSURE inline in pelt.h evaluates this
        // argument. Keep its clock assertions, without inventing a stub symbol.
        let hw_now = native::lupos_pelt_rq_clock_task(rq);
        #[cfg(CONFIG_SCHED_HW_PRESSURE)]
        {
            changed |= update_hw_load_avg(hw_now, rq, hw_pressure as u64);
        }
        #[cfg(not(CONFIG_SCHED_HW_PRESSURE))]
        let _ = (hw_now, hw_pressure);
        #[cfg(CONFIG_HAVE_SCHED_AVG_IRQ)]
        {
            changed |= update_irq_load_avg(rq, 0);
        }
        changed != 0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
