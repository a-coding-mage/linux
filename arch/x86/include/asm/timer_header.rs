/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn native_sched_clock() -> ::core::ffi::c_ulonglong;
pub unsafe fn recalibrate_cpu_khz();

pub static mut no_timer_check: ::core::ffi::c_int;

pub unsafe fn using_native_sched_clock() -> bool;
pub unsafe fn paravirt_set_sched_clock(func: unsafe extern "C" fn(u64) -> u64);

/*
 * We use the full linear equation: f(x) = a + b*x, in order to allow
 * a continuous function in the face of dynamic freq changes.
 *
 * Continuity means that when our frequency changes our slope (b); we want to
 * ensure that: f(t) == f'(t), which gives: a + b*t == a' + b'*t.
 *
 * Without an offset (a) the above would not be possible.
 *
 * See the comment near cycles_2_ns() for details on how we compute (b).
 */
#[repr(C)]
pub struct cyc2ns_data {
    pub cyc2ns_mul: u32,
    pub cyc2ns_shift: u32,
    pub cyc2ns_offset: u64,
} /* 16 bytes */

pub unsafe fn cyc2ns_read_begin(data: *mut cyc2ns_data);
pub unsafe fn cyc2ns_read_end();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
