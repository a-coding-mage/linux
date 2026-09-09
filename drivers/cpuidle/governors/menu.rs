// SPDX-License-Identifier: GPL-2.0-only
/*
 * menu.c - the menu idle governor
 *
 * Copyright (C) 2006-2007 Adam Belay <abelay@novell.com>
 * Copyright (C) 2009 Intel Corporation
 * Author:
 *        Arjan van de Ven <arjan@linux.intel.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const BUCKETS: usize = 6;
const INTERVAL_SHIFT: usize = 3;
const INTERVALS: usize = 1usize << INTERVAL_SHIFT;
const RESOLUTION: u64 = 1024;
const DECAY: u64 = 8;
const MAX_INTERESTING: u64 = 50000 * NSEC_PER_USEC;

#[repr(C)]
struct menu_device {
    needs_update: i32,
    tick_wakeup: i32,
    next_timer_ns: u64,
    bucket: u32,
    correction_factor: [u32; BUCKETS],
    intervals: [u32; INTERVALS],
    interval_ptr: i32,
}

static mut menu_devices: /* DEFINE_PER_CPU(struct menu_device, menu_devices) */ menu_device = menu_device {
    needs_update: 0,
    tick_wakeup: 0,
    next_timer_ns: 0,
    bucket: 0,
    correction_factor: [0; BUCKETS],
    intervals: [0; INTERVALS],
    interval_ptr: 0,
};

#[inline]
unsafe fn which_bucket(duration_ns: u64) -> i32 {
    if duration_ns < 10 * NSEC_PER_USEC { return 0; }
    if duration_ns < 100 * NSEC_PER_USEC { return 1; }
    if duration_ns < 1000 * NSEC_PER_USEC { return 2; }
    if duration_ns < 10000 * NSEC_PER_USEC { return 3; }
    if duration_ns < 100000 * NSEC_PER_USEC { return 4; }
    5
}

unsafe fn menu_update_intervals(data: *mut menu_device, interval_us: u32) {
    (*data).intervals[(*data).interval_ptr as usize] = interval_us;
    (*data).interval_ptr += 1;
    if (*data).interval_ptr >= INTERVALS as i32 { (*data).interval_ptr = 0; }
}

unsafe fn get_typical_interval(data: *mut menu_device) -> u32 {
    let mut min_thresh: i64 = -1;
    let mut max_thresh: i64 = u32::MAX as i64;
    loop {
        let mut max: u32 = 0;
        let mut min: u32 = u32::MAX;
        let mut avg: u64 = 0;
        let mut variance: u64 = 0;
        let mut divisor: u32 = 0;
        for i in 0..INTERVALS {
            let value = (*data).intervals[i] as i64;
            if value <= min_thresh || value >= max_thresh { continue; }
            divisor += 1;
            avg += value as u64;
            variance += (value as u64) * (value as u64);
            if value as u32 > max { max = value as u32; }
            if value as u32 < min { min = value as u32; }
        }
        if max == 0 { return u32::MAX; }
        if divisor == INTERVALS as u32 {
            avg >>= INTERVAL_SHIFT;
            variance >>= INTERVAL_SHIFT;
        } else {
            avg /= divisor as u64;
            variance /= divisor as u64;
        }
        let avg_sq = avg * avg;
        variance -= avg_sq;
        if variance <= u64::MAX / 36 &&
            ((avg_sq > variance * 36 && divisor * 4 >= INTERVALS as u32 * 3) || variance <= 400) {
            return avg as u32;
        }
        if divisor * 4 <= INTERVALS as u32 * 3 { return u32::MAX; }
        if avg - min as u64 > max as u64 - avg { min_thresh = min as i64; }
        else { max_thresh = max as i64; }
    }
}

unsafe fn menu_select(drv: *mut cpuidle_driver, dev: *mut cpuidle_device, stop_tick: *mut bool) -> i32 {
    let data = &mut menu_devices as *mut menu_device;
    let latency_req = cpuidle_governor_latency_req((*dev).cpu);
    let mut predicted_ns = get_typical_interval(data) as u64 * NSEC_PER_USEC;
    let mut delta_tick: ktime_t;
    if predicted_ns > RESIDENCY_THRESHOLD_NS || tick_nohz_tick_stopped() {
        let mut delta: ktime_t;
        (delta, delta_tick) = tick_nohz_get_sleep_length();
        if delta < 0 { delta = 0; delta_tick = 0; }
        (*data).next_timer_ns = delta as u64;
        (*data).bucket = which_bucket((*data).next_timer_ns) as u32;
        let timer_us = ((RESOLUTION * DECAY * NSEC_PER_USEC) / 2 +
            (*data).next_timer_ns * (*data).correction_factor[(*data).bucket as usize]) /
            (RESOLUTION * DECAY * NSEC_PER_USEC);
        predicted_ns = core::cmp::min(timer_us * NSEC_PER_USEC, predicted_ns);
        if tick_nohz_tick_stopped() && predicted_ns < TICK_NSEC && (*data).next_timer_ns > SAFE_TIMER_RANGE_NS {
            predicted_ns = (*data).next_timer_ns;
        }
    } else {
        (*data).next_timer_ns = KTIME_MAX;
        delta_tick = TICK_NSEC / 2;
        (*data).bucket = (BUCKETS - 1) as u32;
    }
    if latency_req == 0 || (((*data).next_timer_ns < (*drv).states[1].target_residency_ns ||
        latency_req < (*drv).states[1].exit_latency_ns) && !(*dev).states_usage[0].disable) {
        *stop_tick = ((*drv).states[0].flags & CPUIDLE_FLAG_POLLING) == 0;
        return 0;
    }
    let mut idx: i32 = -1;
    let mut i = 0;
    while i < (*drv).state_count as usize {
        let s = &(*drv).states[i];
        if (*dev).states_usage[i].disable { i += 1; continue; }
        if idx == -1 { idx = i as i32; }
        if s.exit_latency_ns > latency_req { break; }
        if s.target_residency_ns <= predicted_ns { idx = i as i32; i += 1; continue; }
        if ((*drv).states[idx as usize].flags & CPUIDLE_FLAG_POLLING) != 0 &&
            s.target_residency_ns < RESIDENCY_THRESHOLD_NS && s.target_residency_ns <= (*data).next_timer_ns &&
            s.exit_latency_ns <= predicted_ns { predicted_ns = s.target_residency_ns; idx = i as i32; break; }
        if predicted_ns < TICK_NSEC { break; }
        if !tick_nohz_tick_stopped() { predicted_ns = (*drv).states[idx as usize].target_residency_ns; break; }
        if (*drv).states[idx as usize].target_residency_ns < TICK_NSEC && s.target_residency_ns <= delta_tick as u64 { idx = i as i32; }
        return idx;
    }
    if idx == -1 { idx = 0; }
    if (((*drv).states[idx as usize].flags & CPUIDLE_FLAG_POLLING) != 0 || predicted_ns < TICK_NSEC) && !tick_nohz_tick_stopped() {
        *stop_tick = false;
        if idx > 0 && (*drv).states[idx as usize].target_residency_ns > delta_tick as u64 {
            let mut j = idx - 1;
            while j >= 0 {
                if !(*dev).states_usage[j as usize].disable { idx = j; if (*drv).states[j as usize].target_residency_ns <= delta_tick as u64 { break; } }
                j -= 1;
            }
        }
    }
    idx
}

unsafe fn menu_reflect(dev: *mut cpuidle_device, index: i32) {
    (*dev).last_state_idx = index;
    menu_devices.needs_update = 1;
    menu_devices.tick_wakeup = tick_nohz_idle_got_tick();
}

unsafe fn menu_update(drv: *mut cpuidle_driver, dev: *mut cpuidle_device) {
    let data = &mut menu_devices as *mut menu_device;
    let last_idx = (*dev).last_state_idx as usize;
    let target = &(*drv).states[last_idx];
    let mut measured_ns: u64;
    if (*data).tick_wakeup != 0 && (*data).next_timer_ns > TICK_NSEC { measured_ns = 9 * MAX_INTERESTING / 10; }
    else if ((*drv).states[last_idx].flags & CPUIDLE_FLAG_POLLING) != 0 && (*dev).poll_time_limit != 0 { measured_ns = (*data).next_timer_ns; }
    else {
        measured_ns = (*dev).last_residency_ns;
        if measured_ns > 2 * target.exit_latency_ns { measured_ns -= target.exit_latency_ns; } else { measured_ns /= 2; }
    }
    if measured_ns > (*data).next_timer_ns { measured_ns = (*data).next_timer_ns; }
    let b = (*data).bucket as usize;
    let mut new_factor = (*data).correction_factor[b] as u64;
    new_factor -= new_factor / DECAY;
    if (*data).next_timer_ns > 0 && measured_ns < MAX_INTERESTING { new_factor += RESOLUTION * measured_ns / (*data).next_timer_ns; }
    else { new_factor += RESOLUTION; }
    if DECAY == 1 && new_factor == 0 { new_factor = 1; }
    (*data).correction_factor[b] = new_factor as u32;
    menu_update_intervals(data, ktime_to_us(measured_ns));
}

unsafe fn menu_enable_device(_drv: *mut cpuidle_driver, dev: *mut cpuidle_device) -> i32 {
    core::ptr::write_bytes(&mut menu_devices as *mut menu_device as *mut u8, 0, core::mem::size_of::<menu_device>());
    for i in 0..BUCKETS { menu_devices.correction_factor[i] = (RESOLUTION * DECAY) as u32; }
    let _ = dev;
    0
}

// The governor registration and initcall use external kernel definitions.
static mut menu_governor: cpuidle_governor = cpuidle_governor {
    name: "menu",
    rating: 20,
    enable: Some(menu_enable_device),
    select: Some(menu_select),
    reflect: Some(menu_reflect),
};

unsafe fn init_menu() -> i32 { cpuidle_register_governor(&mut menu_governor) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
