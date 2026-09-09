// SPDX-License-Identifier: GPL-2.0+
/*
 * This file contains the jiffies based clocksource.
 *
 * Copyright (C) 2004, 2005 IBM, John Stultz (johnstul@us.ibm.com)
 */

// Kernel dependencies supplied by other translation units.

unsafe extern "C" {
    static mut jiffies: c_ulong;
    static mut jiffies_64: u64;
    static HZ: c_ulong;
    static NSEC_PER_SEC: u64;

    fn __clocksource_register(cs: *mut clocksource) -> c_int;
    fn read_seqcount_begin(seq: *const seqcount_raw_spinlock_t) -> c_uint;
    fn read_seqcount_retry(seq: *const seqcount_raw_spinlock_t, start: c_uint) -> bool;
    fn proc_int_u2k_conv_uop(
        u_ptr: *const c_ulong,
        k_ptr: *mut c_int,
        negp: *const bool,
        op: unsafe extern "C" fn(c_ulong) -> c_ulong,
    ) -> c_int;
    fn proc_int_k2u_conv_kop(
        u_ptr: *mut c_ulong,
        k_ptr: *const c_int,
        negp: *mut bool,
        op: unsafe extern "C" fn(c_ulong) -> c_ulong,
    ) -> c_int;
    fn proc_int_conv(
        negp: *mut bool,
        u_ptr: *mut c_ulong,
        k_ptr: *mut c_int,
        dir: c_int,
        tbl: *const ctl_table,
        conv: unsafe extern "C" fn(*mut bool, *mut c_ulong, *mut c_int, c_int, *const ctl_table) -> c_int,
    ) -> c_int;
    fn proc_ulong_u2k_conv_uop(u_ptr: *const c_ulong, k_ptr: *mut c_ulong,
                               op: unsafe extern "C" fn(c_ulong) -> c_ulong) -> c_int;
    fn proc_ulong_k2u_conv_kop(u_ptr: *mut c_ulong, k_ptr: *const c_ulong,
                               op: unsafe extern "C" fn(c_ulong) -> c_ulong) -> c_int;
    fn proc_ulong_conv(u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int,
                       tbl: *const ctl_table, write: bool,
                       u2k: unsafe extern "C" fn(*const c_ulong, *mut c_ulong) -> c_int,
                       k2u: unsafe extern "C" fn(*mut c_ulong, *const c_ulong) -> c_int) -> c_int;
    fn proc_dointvec_conv(table: *const ctl_table, dir: c_int, buffer: *mut c_void,
                          lenp: *mut usize, ppos: *mut loff_t,
                          conv: unsafe extern "C" fn(*mut bool, *mut c_ulong, *mut c_int, c_int, *const ctl_table) -> c_int) -> c_int;
    fn proc_doulongvec_conv(table: *const ctl_table, dir: c_int, buffer: *mut c_void,
                            lenp: *mut usize, ppos: *mut loff_t,
                            conv: unsafe extern "C" fn(*mut bool, *mut c_ulong, *mut c_ulong, c_int, *const ctl_table) -> c_int) -> c_int;
    fn clock_t_to_jiffies(val: c_ulong) -> c_ulong;
    fn jiffies_to_clock_t(val: c_ulong) -> c_ulong;
    fn msecs_to_jiffies(val: c_ulong) -> c_ulong;
    fn jiffies_to_msecs(val: c_ulong) -> c_ulong;
}

type c_ulong = usize;
type c_long = isize;
type c_uint = u32;
type c_int = i32;
type c_void = core::ffi::c_void;
type loff_t = i64;

unsafe extern "C" fn jiffies_read(_cs: *mut clocksource) -> u64 {
    jiffies as u64
}

static mut clocksource_jiffies: clocksource = clocksource {
    name: "jiffies",
    rating: 1,
    read: Some(jiffies_read),
    mask: CLOCKSOURCE_MASK(32),
    mult: TICK_NSEC << JIFFIES_SHIFT,
    shift: JIFFIES_SHIFT,
    max_cycles: 10,
};

static mut jiffies_lock: raw_spinlock_t = raw_spinlock_t::ZERO;
static mut jiffies_seq: seqcount_raw_spinlock_t = SEQCNT_RAW_SPINLOCK_ZERO;

#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub unsafe extern "C" fn get_jiffies_64() -> u64 {
    let mut seq: c_uint;
    let mut ret: u64;
    loop {
        seq = read_seqcount_begin(&jiffies_seq);
        ret = jiffies_64;
        if !read_seqcount_retry(&jiffies_seq, seq) {
            return ret;
        }
    }
}

static mut cs_jiffies_registered: bool = false;

#[no_mangle]
pub unsafe extern "C" fn clocksource_default_clock() -> *mut clocksource {
    if !cs_jiffies_registered {
        __clocksource_register(&raw mut clocksource_jiffies);
        cs_jiffies_registered = true;
    }
    &raw mut clocksource_jiffies
}

static mut refined_jiffies: clocksource = clocksource {
    name: "",
    rating: 0,
    read: None,
    mask: 0,
    mult: 0,
    shift: 0,
    max_cycles: 0,
};

#[no_mangle]
pub unsafe extern "C" fn register_refined_jiffies(cycles_per_second: c_long) {
    let mut nsec_per_tick: u64;
    let mut shift_hz: u64;
    let cycles_per_tick: c_long;

    refined_jiffies = clocksource_jiffies;
    refined_jiffies.name = "refined-jiffies";
    refined_jiffies.rating += 1;

    cycles_per_tick = (cycles_per_second + (HZ as c_long) / 2) / HZ as c_long;
    shift_hz = (cycles_per_second as u64) << 8;
    shift_hz += (cycles_per_tick / 2) as u64;
    shift_hz /= cycles_per_tick as u64;
    nsec_per_tick = NSEC_PER_SEC << 8;
    nsec_per_tick += (shift_hz as u32 / 2) as u64;
    nsec_per_tick /= shift_hz as u32 as u64;
    refined_jiffies.mult = (nsec_per_tick as u32) << JIFFIES_SHIFT;
    __clocksource_register(&raw mut refined_jiffies);
}

// The CONFIG_SYSCTL branch is retained as a feature conditional.
#[cfg(feature = "CONFIG_SYSCTL")]
mod sysctl {
    use super::*;

    unsafe extern "C" fn mult_hz(val: c_ulong) -> c_ulong { val * HZ as c_ulong }
    unsafe extern "C" fn div_hz(val: c_ulong) -> c_ulong { val / HZ as c_ulong }
    unsafe extern "C" fn sysctl_jiffies_to_clock_t(val: c_ulong) -> c_ulong { jiffies_to_clock_t(val) }
    unsafe extern "C" fn sysctl_msecs_to_jiffies(val: c_ulong) -> c_ulong { msecs_to_jiffies(val) }
    unsafe extern "C" fn sysctl_jiffies_to_msecs(val: c_ulong) -> c_ulong { jiffies_to_msecs(val) }

    unsafe extern "C" fn sysctl_u2k_int_conv_hz(n: *const bool, u: *const c_ulong, k: *mut c_int) -> c_int { proc_int_u2k_conv_uop(u, k, n, mult_hz) }
    unsafe extern "C" fn sysctl_k2u_int_conv_hz(n: *mut bool, u: *mut c_ulong, k: *const c_int) -> c_int { proc_int_k2u_conv_kop(u, k, n, div_hz) }
    unsafe extern "C" fn sysctl_u2k_int_conv_userhz(n: *const bool, u: *const c_ulong, k: *mut c_int) -> c_int { proc_int_u2k_conv_uop(u, k, n, clock_t_to_jiffies) }
    unsafe extern "C" fn sysctl_k2u_int_conv_userhz(n: *mut bool, u: *mut c_ulong, k: *const c_int) -> c_int { proc_int_k2u_conv_kop(u, k, n, sysctl_jiffies_to_clock_t) }
    unsafe extern "C" fn sysctl_u2k_int_conv_ms(n: *const bool, u: *const c_ulong, k: *mut c_int) -> c_int { proc_int_u2k_conv_uop(u, k, n, sysctl_msecs_to_jiffies) }
    unsafe extern "C" fn sysctl_k2u_int_conv_ms(n: *mut bool, u: *mut c_ulong, k: *const c_int) -> c_int { proc_int_k2u_conv_kop(u, k, n, sysctl_jiffies_to_msecs) }
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe extern "C" fn do_proc_int_conv_jiffies(_n: *mut bool, _u: *mut c_ulong, _k: *mut c_int, _d: c_int, _t: *const ctl_table) -> c_int { -38 }

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn do_proc_int_conv_jiffies(n: *mut bool, u: *mut c_ulong, k: *mut c_int, d: c_int, t: *const ctl_table) -> c_int {
    proc_int_conv(n, u, k, d, t, false, sysctl::sysctl_u2k_int_conv_hz, sysctl::sysctl_k2u_int_conv_hz)
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn do_proc_int_conv_userhz_jiffies(n: *mut bool, u: *mut c_ulong, k: *mut c_int, d: c_int, t: *const ctl_table) -> c_int {
    proc_int_conv(n, u, k, d, t, false, sysctl::sysctl_u2k_int_conv_userhz, sysctl::sysctl_k2u_int_conv_userhz)
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn do_proc_int_conv_ms_jiffies(n: *mut bool, u: *mut c_ulong, k: *mut c_int, d: c_int, t: *const ctl_table) -> c_int {
    proc_int_conv(n, u, k, d, t, false, sysctl::sysctl_u2k_int_conv_ms, sysctl::sysctl_k2u_int_conv_ms)
}

#[cfg(feature = "CONFIG_SYSCTL")]
unsafe extern "C" fn do_proc_ulong_conv_ms_jiffies(_n: *mut bool, u: *mut c_ulong, k: *mut c_ulong, d: c_int, t: *const ctl_table) -> c_int {
    proc_ulong_conv(u, k, d, t, false, sysctl::sysctl_u2k_ulong_conv_ms, sysctl::sysctl_k2u_ulong_conv_ms)
}

#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe extern "C" fn do_proc_int_conv_userhz_jiffies(_n: *mut bool, _u: *mut c_ulong, _k: *mut c_int, _d: c_int, _t: *const ctl_table) -> c_int { -38 }
#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe extern "C" fn do_proc_int_conv_ms_jiffies(_n: *mut bool, _u: *mut c_ulong, _k: *mut c_int, _d: c_int, _t: *const ctl_table) -> c_int { -38 }
#[cfg(not(feature = "CONFIG_SYSCTL"))]
unsafe extern "C" fn do_proc_ulong_conv_ms_jiffies(_n: *mut bool, _u: *mut c_ulong, _k: *mut c_ulong, _d: c_int, _t: *const ctl_table) -> c_int { -38 }

#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_jiffies(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    proc_dointvec_conv(table, dir, buffer, lenp, ppos, do_proc_int_conv_jiffies)
}

#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_userhz_jiffies(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    proc_dointvec_conv(table, dir, buffer, lenp, ppos, do_proc_int_conv_userhz_jiffies)
}

#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_ms_jiffies(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    proc_dointvec_conv(table, dir, buffer, lenp, ppos, do_proc_int_conv_ms_jiffies)
}

#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_ms_jiffies_minmax(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    proc_dointvec_conv(table, dir, buffer, lenp, ppos, do_proc_int_conv_ms_jiffies)
}

#[no_mangle]
pub unsafe extern "C" fn proc_doulongvec_ms_jiffies_minmax(table: *const ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut usize, ppos: *mut loff_t) -> c_int {
    proc_doulongvec_conv(table, dir, buffer, lenp, ppos, do_proc_ulong_conv_ms_jiffies)
}

// External kernel types and constants are supplied by the translated headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
