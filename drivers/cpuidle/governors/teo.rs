// SPDX-License-Identifier: GPL-2.0
/*
 * Timer events oriented CPU idle governor
 *
 * Copyright (C) 2018 - 2021 Intel Corporation
 * Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 */

/* The documentation comment from teo.c is retained as source commentary. */

// C headers and build-time kernel dependencies are supplied by the surrounding kernel translation.

const LATENCY_THRESHOLD_NS: s64 = RESIDENCY_THRESHOLD_NS / 2;
const PULSE: u32 = 1024;
const DECAY_SHIFT: u32 = 3;

#[repr(C)]
pub struct teo_bin {
    pub intercepts: u32,
    pub hits: u32,
}

#[repr(C)]
pub struct teo_cpu {
    pub sleep_length_ns: s64,
    pub state_bins: [teo_bin; CPUIDLE_STATE_MAX as usize],
    pub total: u32,
    pub total_tick: u32,
    pub tick_intercepts: u32,
    pub short_idles: u32,
    pub tick_wakeup: bool,
}

// DEFINE_PER_CPU(struct teo_cpu, teo_cpus);
extern "C" {
    static mut teo_cpus: teo_cpu;
}

unsafe fn this_teo_cpu() -> *mut teo_cpu {
    &raw mut teo_cpus
}

unsafe fn teo_decay(metric: *mut u32) {
    let delta = *metric >> DECAY_SHIFT;
    if delta != 0 {
        *metric = (*metric).wrapping_sub(delta);
    } else {
        *metric = 0;
    }
}

unsafe fn teo_update(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) {
    let lat_ns = (*drv).states[(*dev).last_state_idx as usize].exit_latency_ns;
    let cpu_data = this_teo_cpu();
    let mut idx_timer: usize = 0;
    let mut idx_duration: usize = 0;
    let mut measured_ns: s64;
    let mut total: u32 = 0;

    teo_decay(&raw mut (*cpu_data).short_idles);

    if (*dev).poll_time_limit {
        (*dev).poll_time_limit = false;
        measured_ns = S64_MAX;
    } else {
        measured_ns = (*dev).last_residency_ns;
        if measured_ns >= lat_ns {
            measured_ns -= lat_ns / 2;
            if measured_ns < RESIDENCY_THRESHOLD_NS {
                (*cpu_data).short_idles = (*cpu_data).short_idles.wrapping_add(PULSE);
            }
        } else {
            measured_ns /= 2;
            (*cpu_data).short_idles = (*cpu_data).short_idles.wrapping_add(PULSE);
        }
    }

    for i in 0..(*drv).state_count as usize {
        let bin = &raw mut (*cpu_data).state_bins[i];
        teo_decay(&raw mut (*bin).hits);
        total = total.wrapping_add((*bin).hits);
        teo_decay(&raw mut (*bin).intercepts);
        total = total.wrapping_add((*bin).intercepts);
        let target_residency_ns = (*drv).states[i].target_residency_ns;
        if target_residency_ns <= (*cpu_data).sleep_length_ns {
            idx_timer = i;
            if target_residency_ns <= measured_ns {
                idx_duration = i;
            }
        }
    }

    (*cpu_data).total = total.wrapping_add(PULSE);
    teo_decay(&raw mut (*cpu_data).tick_intercepts);
    teo_decay(&raw mut (*cpu_data).total_tick);
    if (*cpu_data).tick_wakeup {
        (*cpu_data).total_tick = (*cpu_data).total_tick.wrapping_add(PULSE);
        if 3 * (*cpu_data).total_tick > 2 * (*cpu_data).total {
            (*cpu_data).state_bins[(*drv).state_count as usize - 1].hits =
                (*cpu_data).state_bins[(*drv).state_count as usize - 1].hits.wrapping_add(PULSE);
            return;
        }
        if 3 * (*cpu_data).tick_intercepts < 2 * total {
            (*cpu_data).state_bins[idx_timer].hits = (*cpu_data).state_bins[idx_timer].hits.wrapping_add(PULSE);
            return;
        }
    }

    if idx_timer == idx_duration && (*cpu_data).sleep_length_ns - measured_ns < lat_ns / 2 {
        (*cpu_data).state_bins[idx_timer].hits = (*cpu_data).state_bins[idx_timer].hits.wrapping_add(PULSE);
    } else {
        (*cpu_data).state_bins[idx_duration].intercepts = (*cpu_data).state_bins[idx_duration].intercepts.wrapping_add(PULSE);
        if measured_ns <= TICK_NSEC {
            (*cpu_data).tick_intercepts = (*cpu_data).tick_intercepts.wrapping_add(PULSE);
        }
    }
}

unsafe fn teo_find_shallower_state(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, mut state_idx: i32, duration_ns: s64) -> i32 {
    let mut i = state_idx - 1;
    while i >= 0 {
        if (*dev).states_usage[i as usize].disable {
            i -= 1;
            continue;
        }
        state_idx = i;
        if (*drv).states[i as usize].target_residency_ns <= duration_ns { break; }
        i -= 1;
    }
    state_idx
}

unsafe fn teo_select(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, stop_tick: *mut bool) -> i32 {
    let cpu_data = this_teo_cpu();
    let latency_req = cpuidle_governor_latency_req((*dev).cpu);
    let mut delta_tick: ktime_t = TICK_NSEC / 2;
    let mut idx_intercept_sum = 0u32;
    let mut intercept_sum = 0u32;
    let mut intercept_max = 0u32;
    let mut idx_hit_sum = 0u32;
    let mut hit_sum = 0u32;
    let mut intercept_max_idx = -1i32;
    let mut constraint_idx = 0i32;
    let mut idx0 = 0i32;
    let mut idx = -1i32;
    let mut duration_ns: s64;

    if (*dev).last_state_idx >= 0 {
        teo_update(drv, dev);
        (*dev).last_state_idx = -1;
    }
    (*cpu_data).sleep_length_ns = KTIME_MAX;
    if !(*dev).states_usage[0].disable { idx = 0; }

    for i in 1..(*drv).state_count as usize {
        let prev = &(*cpu_data).state_bins[i - 1];
        let prev_intercepts = prev.intercepts;
        hit_sum += prev.hits;
        intercept_sum += prev_intercepts;
        if prev_intercepts >= intercept_max { intercept_max = prev_intercepts; intercept_max_idx = i as i32 - 1; }
        if (*dev).states_usage[i].disable { continue; }
        if idx < 0 { idx0 = i as i32; }
        idx = i as i32;
        if (*drv).states[i].exit_latency_ns <= latency_req { constraint_idx = i as i32; }
        idx_intercept_sum = intercept_sum;
        idx_hit_sum = hit_sum;
    }
    if idx < 0 { idx = 0; *stop_tick = false; return idx; }
    if idx == idx0 { duration_ns = (*drv).states[idx as usize].target_residency_ns; return teo_finish(drv, dev, stop_tick, idx, idx0, delta_tick, duration_ns); }
    if 2 * idx_intercept_sum > (*cpu_data).total - idx_hit_sum {
        intercept_sum = 0;
        let mut i = idx - 1;
        while i >= idx0 {
            intercept_sum += (*cpu_data).state_bins[i as usize].intercepts;
            if !(*dev).states_usage[i as usize].disable {
                idx = i;
                if 2 * intercept_sum > idx_intercept_sum && i <= intercept_max_idx { break; }
            }
            i -= 1;
        }
    }
    if idx > constraint_idx { idx = constraint_idx; }
    if !tick_nohz_tick_stopped() && (idx == 0 || (*drv).states[idx as usize].target_residency_ns < RESIDENCY_THRESHOLD_NS) && (2 * (*cpu_data).short_idles >= (*cpu_data).total || latency_req < LATENCY_THRESHOLD_NS) {
        *stop_tick = false; return idx;
    }
    duration_ns = tick_nohz_get_sleep_length(&mut delta_tick);
    (*cpu_data).sleep_length_ns = duration_ns;
    if tick_nohz_tick_stopped() && duration_ns > SAFE_TIMER_RANGE_NS && (*drv).states[idx as usize].target_residency_ns < TICK_NSEC {
        let mut i = constraint_idx;
        while i > idx { if !(*dev).states_usage[i as usize].disable && (*drv).states[i as usize].target_residency_ns <= duration_ns { idx = i; break; } i -= 1; }
        return idx;
    }
    if idx == 0 { *stop_tick = false; return idx; }
    if (*drv).states[idx as usize].target_residency_ns > duration_ns { idx = teo_find_shallower_state(drv, dev, idx, duration_ns); }
    if (*drv).states[idx as usize].target_residency_ns < TICK_NSEC && 3 * (*cpu_data).tick_intercepts >= 2 * (*cpu_data).total { duration_ns = TICK_NSEC / 2; }
    teo_finish(drv, dev, stop_tick, idx, idx0, delta_tick, duration_ns)
}

unsafe fn teo_finish(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, stop_tick: *mut bool, mut idx: i32, idx0: i32, delta_tick: ktime_t, duration_ns: s64) -> i32 {
    if (((*drv).states[idx as usize].flags & CPUIDLE_FLAG_POLLING) == 0 && duration_ns >= TICK_NSEC) || tick_nohz_tick_stopped() { return idx; }
    if idx > idx0 && (*drv).states[idx as usize].target_residency_ns > delta_tick { idx = teo_find_shallower_state(drv, dev, idx, delta_tick); }
    *stop_tick = false;
    idx
}

unsafe fn teo_reflect(dev: *mut cpuidle_device, state: i32) {
    let cpu_data = this_teo_cpu();
    (*cpu_data).tick_wakeup = tick_nohz_idle_got_tick();
    (*dev).last_state_idx = state;
}

unsafe fn teo_enable_device(_drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> i32 {
    let cpu_data = per_cpu_ptr(&raw mut teo_cpus, (*dev).cpu);
    core::ptr::write_bytes(cpu_data, 0, 1);
    0
}

#[repr(C)]
static mut teo_governor: cpuidle_governor = cpuidle_governor {
    name: "teo",
    rating: 19,
    enable: teo_enable_device,
    select: teo_select,
    reflect: teo_reflect,
};

unsafe fn teo_governor_init() -> i32 {
    cpuidle_register_governor(&raw mut teo_governor)
}

// postcore_initcall(teo_governor_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
