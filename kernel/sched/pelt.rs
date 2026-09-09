// SPDX-License-Identifier: GPL-2.0
/*
 * Per Entity Load Tracking (PELT)
 *
 * This file is a source-level Rust translation of pelt.c.
 * Symbols supplied by pelt.h and the scheduler remain external dependencies.
 */

unsafe fn decay_load(mut val: u64, n: u64) -> u64 {
    if n > LOAD_AVG_PERIOD * 63 {
        return 0;
    }

    let mut local_n = n as u32;
    if local_n >= LOAD_AVG_PERIOD {
        val >>= local_n / LOAD_AVG_PERIOD;
        local_n %= LOAD_AVG_PERIOD;
    }

    mul_u64_u32_shr(val, runnable_avg_yN_inv[local_n as usize], 32)
}

unsafe fn __accumulate_pelt_segments(periods: u64, d1: u32, d3: u32) -> u32 {
    let c1 = decay_load(d1 as u64, periods) as u32;
    let c2 = LOAD_AVG_MAX
        .wrapping_sub(decay_load(LOAD_AVG_MAX as u64, periods) as u32)
        .wrapping_sub(1024);
    c1.wrapping_add(c2).wrapping_add(d3)
}

unsafe fn accumulate_sum(
    mut delta: u64,
    sa: &mut sched_avg,
    load: usize,
    runnable: usize,
    running: i32,
) -> u32 {
    let mut contrib = delta as u32;
    delta = delta.wrapping_add(sa.period_contrib);
    let periods = delta / 1024;

    if periods != 0 {
        sa.load_sum = decay_load(sa.load_sum, periods);
        sa.runnable_sum = decay_load(sa.runnable_sum, periods);
        sa.util_sum = decay_load(sa.util_sum as u64, periods) as u32;

        delta %= 1024;
        if load != 0 {
            contrib = __accumulate_pelt_segments(
                periods,
                1024u32.wrapping_sub(sa.period_contrib as u32),
                delta as u32,
            );
        }
    }
    sa.period_contrib = delta;

    if load != 0 {
        sa.load_sum = sa.load_sum.wrapping_add((load as u64).wrapping_mul(contrib as u64));
    }
    if runnable != 0 {
        sa.runnable_sum = sa.runnable_sum.wrapping_add(
            (runnable as u64).wrapping_mul(contrib as u64) << SCHED_CAPACITY_SHIFT,
        );
    }
    if running != 0 {
        sa.util_sum = sa
            .util_sum
            .wrapping_add(contrib << SCHED_CAPACITY_SHIFT);
    }

    periods as u32
}

unsafe fn ___update_load_sum(
    now: u64,
    sa: &mut sched_avg,
    load: usize,
    mut runnable: usize,
    mut running: i32,
) -> i32 {
    let mut delta = now.wrapping_sub(sa.last_update_time);
    if (delta as i64) < 0 {
        sa.last_update_time = now;
        return 0;
    }

    delta >>= 10;
    if delta == 0 {
        return 0;
    }
    sa.last_update_time = sa.last_update_time.wrapping_add(delta << 10);

    if load == 0 {
        runnable = 0;
        running = 0;
    }

    if accumulate_sum(delta, sa, load, runnable, running) == 0 {
        return 0;
    }
    1
}

unsafe fn ___update_load_avg(sa: &mut sched_avg, load: usize) {
    let divider = get_pelt_divider(sa);
    sa.load_avg = div_u64((load as u64).wrapping_mul(sa.load_sum), divider);
    sa.runnable_avg = div_u64(sa.runnable_sum, divider);
    WRITE_ONCE(sa.util_avg, sa.util_sum / divider);
}

pub unsafe fn __update_load_avg_blocked_se(now: u64, se: &mut sched_entity) -> i32 {
    if ___update_load_sum(now, &mut se.avg, 0, 0, 0) != 0 {
        ___update_load_avg(&mut se.avg, se_weight(se));
        trace_pelt_se_tp(se);
        return 1;
    }
    0
}

pub unsafe fn __update_load_avg_se(now: u64, cfs_rq: &mut cfs_rq, se: &mut sched_entity) -> i32 {
    if ___update_load_sum(
        now,
        &mut se.avg,
        if se.on_rq { 1 } else { 0 },
        se_runnable(se),
        if cfs_rq.h_curr == se as *mut sched_entity { 1 } else { 0 },
    ) != 0 {
        ___update_load_avg(&mut se.avg, se_weight(se));
        cfs_se_util_change(&mut se.avg);
        trace_pelt_se_tp(se);
        return 1;
    }
    0
}

pub unsafe fn __update_load_avg_cfs_rq(now: u64, cfs_rq: &mut cfs_rq) -> i32 {
    if ___update_load_sum(
        now,
        &mut cfs_rq.avg,
        scale_load_down(cfs_rq.load.weight),
        cfs_rq.h_nr_runnable,
        if !cfs_rq.h_curr.is_null() { 1 } else { 0 },
    ) != 0 {
        ___update_load_avg(&mut cfs_rq.avg, 1);
        trace_pelt_cfs_tp(cfs_rq);
        return 1;
    }
    0
}

pub unsafe fn update_rt_rq_load_avg(now: u64, rq: &mut rq, running: i32) -> i32 {
    if ___update_load_sum(now, &mut rq.avg_rt, running as usize, running as usize, running) != 0 {
        ___update_load_avg(&mut rq.avg_rt, 1);
        trace_pelt_rt_tp(rq);
        return 1;
    }
    0
}

pub unsafe fn update_dl_rq_load_avg(now: u64, rq: &mut rq, running: i32) -> i32 {
    if ___update_load_sum(now, &mut rq.avg_dl, running as usize, running as usize, running) != 0 {
        ___update_load_avg(&mut rq.avg_dl, 1);
        trace_pelt_dl_tp(rq);
        return 1;
    }
    0
}

#[cfg(CONFIG_SCHED_HW_PRESSURE)]
pub unsafe fn update_hw_load_avg(now: u64, rq: &mut rq, capacity: u64) -> i32 {
    if ___update_load_sum(now, &mut rq.avg_hw, capacity as usize, capacity as usize, capacity as i32) != 0 {
        ___update_load_avg(&mut rq.avg_hw, 1);
        trace_pelt_hw_tp(rq);
        return 1;
    }
    0
}

#[cfg(CONFIG_HAVE_SCHED_AVG_IRQ)]
pub unsafe fn update_irq_load_avg(rq: &mut rq, mut running: u64) -> i32 {
    let mut ret = 0;
    running = cap_scale(running, arch_scale_freq_capacity(cpu_of(rq)));
    running = cap_scale(running, arch_scale_cpu_capacity(cpu_of(rq)));
    ret += ___update_load_sum(rq.clock.wrapping_sub(running), &mut rq.avg_irq, 0, 0, 0);
    ret += ___update_load_sum(rq.clock, &mut rq.avg_irq, 1, 1, 1);
    if ret != 0 {
        ___update_load_avg(&mut rq.avg_irq, 1);
        trace_pelt_irq_tp(rq);
    }
    ret
}

pub unsafe fn update_other_load_avgs(rq: &mut rq) -> bool {
    let now = rq_clock_pelt(rq);
    let curr_class = (*rq.donor).sched_class;
    let hw_pressure = arch_scale_hw_pressure(cpu_of(rq));
    lockdep_assert_rq_held(rq);

    (update_rt_rq_load_avg(now, rq, curr_class == &rt_sched_class) != 0)
        || (update_dl_rq_load_avg(now, rq, curr_class == &dl_sched_class) != 0)
        || (update_hw_load_avg(rq_clock_task(rq), rq, hw_pressure) != 0)
        || (update_irq_load_avg(rq, 0) != 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
