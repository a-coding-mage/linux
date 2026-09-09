// SPDX-License-Identifier: GPL-2.0
/*
 * kernel/sched/loadavg.c
 *
 * This file contains the magic bits required to compute the global loadavg
 * figure. Its a silly number but people think its important. We go through
 * great pains to make it work on big machines and tickless kernels.
 */

use core::ffi::{c_int, c_long, c_uint, c_ulong};

// External kernel declarations supplied by other translation units.
#[repr(C)]
pub struct atomic_long_t {
    pub counter: c_long,
}

#[repr(C)]
pub struct rq {
    pub nr_running: c_long,
    pub nr_uninterruptible: c_long,
    pub calc_load_active: c_long,
    pub calc_load_update: c_ulong,
}

extern "C" {
    static mut jiffies: c_ulong;
    static mut FSHIFT: c_uint;
    static mut FIXED_1: c_long;
    static mut EXP_1: c_ulong;
    static mut EXP_5: c_ulong;
    static mut EXP_15: c_ulong;
    static mut LOAD_FREQ: c_ulong;

    fn this_rq() -> *mut rq;
    fn calc_load(load: c_ulong, exp: c_ulong, active: c_long) -> c_ulong;
    fn time_before(a: c_ulong, b: c_ulong) -> bool;
    fn smp_rmb();
    fn smp_wmb();
    fn atomic_long_add(value: c_long, ptr: *mut atomic_long_t);
    fn atomic_long_read(ptr: *const atomic_long_t) -> c_long;
    fn atomic_long_xchg(ptr: *mut atomic_long_t, value: c_long) -> c_long;
}

pub static mut calc_load_tasks: atomic_long_t = atomic_long_t { counter: 0 };
pub static mut calc_load_update: c_ulong = 0;
pub static mut avenrun: [c_ulong; 3] = [0; 3];

pub unsafe fn get_avenrun(loads: *mut c_ulong, offset: c_ulong, shift: c_int) {
    *loads.add(0) = (avenrun[0].wrapping_add(offset)) << shift;
    *loads.add(1) = (avenrun[1].wrapping_add(offset)) << shift;
    *loads.add(2) = (avenrun[2].wrapping_add(offset)) << shift;
}

pub unsafe fn calc_load_fold_active(this_rq: *mut rq, adjust: c_long) -> c_long {
    let mut nr_active: c_long;
    let mut delta: c_long = 0;

    nr_active = (*this_rq).nr_running.wrapping_sub(adjust);
    nr_active = nr_active.wrapping_add((*this_rq).nr_uninterruptible as c_long);

    if nr_active != (*this_rq).calc_load_active {
        delta = nr_active.wrapping_sub((*this_rq).calc_load_active);
        (*this_rq).calc_load_active = nr_active;
    }

    delta
}

unsafe fn fixed_power_int(mut x: c_ulong, frac_bits: c_uint, mut n: c_uint) -> c_ulong {
    let mut result = 1u64.wrapping_shl(frac_bits) as c_ulong;

    if n != 0 {
        loop {
            if n & 1 != 0 {
                result = result.wrapping_mul(x);
                result = result.wrapping_add((1u64 << (frac_bits - 1)) as c_ulong);
                result >>= frac_bits;
            }
            n >>= 1;
            if n == 0 {
                break;
            }
            x = x.wrapping_mul(x);
            x = x.wrapping_add((1u64 << (frac_bits - 1)) as c_ulong);
            x >>= frac_bits;
        }
    }

    result
}

pub unsafe fn calc_load_n(load: c_ulong, exp: c_ulong, active: c_long, n: c_uint) -> c_ulong {
    calc_load(load, fixed_power_int(exp, FSHIFT, n), active)
}

#[cfg(CONFIG_NO_HZ_COMMON)]
mod no_hz {
    use super::*;

    static mut calc_load_nohz: [atomic_long_t; 2] = [
        atomic_long_t { counter: 0 },
        atomic_long_t { counter: 0 },
    ];
    static mut calc_load_idx: c_int = 0;

    unsafe fn calc_load_write_idx() -> c_int {
        let mut idx = calc_load_idx;
        smp_rmb();
        if !time_before(jiffies, calc_load_update) {
            idx += 1;
        }
        idx & 1
    }

    unsafe fn calc_load_read_idx() -> c_int { calc_load_idx & 1 }

    unsafe fn calc_load_nohz_fold(rq: *mut rq) {
        let delta = calc_load_fold_active(rq, 0);
        if delta != 0 {
            let idx = calc_load_write_idx();
            atomic_long_add(delta, &mut calc_load_nohz[idx as usize]);
        }
    }

    pub unsafe fn calc_load_nohz_start() { calc_load_nohz_fold(this_rq()); }

    pub unsafe fn calc_load_nohz_remote(rq: *mut rq) { calc_load_nohz_fold(rq); }

    pub unsafe fn calc_load_nohz_stop() {
        let this_rq = this_rq();
        (*this_rq).calc_load_update = calc_load_update;
        if time_before(jiffies, (*this_rq).calc_load_update) { return; }
        if time_before(jiffies, (*this_rq).calc_load_update.wrapping_add(10)) {
            (*this_rq).calc_load_update = (*this_rq).calc_load_update.wrapping_add(LOAD_FREQ);
        }
    }

    unsafe fn calc_load_nohz_read() -> c_long {
        let idx = calc_load_read_idx() as usize;
        if atomic_long_read(&calc_load_nohz[idx]) != 0 {
            atomic_long_xchg(&mut calc_load_nohz[idx], 0)
        } else { 0 }
    }

    pub unsafe fn calc_global_nohz() {
        let sample_window = calc_load_update;
        if !time_before(jiffies, sample_window.wrapping_add(10)) {
            let delta = jiffies.wrapping_sub(sample_window).wrapping_sub(10);
            let n = 1 + delta / LOAD_FREQ;
            let mut active = atomic_long_read(&calc_load_tasks);
            active = if active > 0 { active.wrapping_mul(FIXED_1) } else { 0 };
            avenrun[0] = calc_load_n(avenrun[0], EXP_1, active, n as c_uint);
            avenrun[1] = calc_load_n(avenrun[1], EXP_5, active, n as c_uint);
            avenrun[2] = calc_load_n(avenrun[2], EXP_15, active, n as c_uint);
            calc_load_update = sample_window.wrapping_add(n.wrapping_mul(LOAD_FREQ));
        }
        smp_wmb();
        calc_load_idx += 1;
    }

    pub unsafe fn calc_load_nohz_delta() -> c_long { calc_load_nohz_read() }
}

#[cfg(not(CONFIG_NO_HZ_COMMON))]
mod no_hz {
    use super::*;
    pub unsafe fn calc_global_nohz() {}
    pub unsafe fn calc_load_nohz_delta() -> c_long { 0 }
}

#[cfg(CONFIG_NO_HZ_COMMON)]
pub use no_hz::{calc_load_nohz_remote, calc_load_nohz_start, calc_load_nohz_stop};

pub unsafe fn calc_global_load() {
    let sample_window = calc_load_update;
    if time_before(jiffies, sample_window.wrapping_add(10)) { return; }
    let delta = no_hz::calc_load_nohz_delta();
    if delta != 0 { atomic_long_add(delta, &mut calc_load_tasks); }
    let mut active = atomic_long_read(&calc_load_tasks);
    active = if active > 0 { active.wrapping_mul(FIXED_1) } else { 0 };
    avenrun[0] = calc_load(avenrun[0], EXP_1, active);
    avenrun[1] = calc_load(avenrun[1], EXP_5, active);
    avenrun[2] = calc_load(avenrun[2], EXP_15, active);
    calc_load_update = sample_window.wrapping_add(LOAD_FREQ);
    no_hz::calc_global_nohz();
}

pub unsafe fn calc_global_load_tick(this_rq: *mut rq) {
    if time_before(jiffies, (*this_rq).calc_load_update) { return; }
    let delta = calc_load_fold_active(this_rq, 0);
    if delta != 0 { atomic_long_add(delta, &mut calc_load_tasks); }
    (*this_rq).calc_load_update = (*this_rq).calc_load_update.wrapping_add(LOAD_FREQ);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
