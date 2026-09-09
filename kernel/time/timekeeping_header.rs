/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Internal interfaces for kernel/time/
 *
 * C header guards are omitted in Rust; module inclusion provides the guard.
 */

extern "C" {
    pub fn ktime_get_update_offsets_now(
        cwsseq: *mut u32,
        offs_real: *mut ktime_t,
        offs_boot: *mut ktime_t,
        offs_tai: *mut ktime_t,
    ) -> ktime_t;

    pub fn ktime_expiry_to_cycles(
        id: clocksource_ids,
        expires_ns: ktime_t,
        cycles: *mut u64,
    ) -> bool;

    pub fn timekeeping_valid_for_hres() -> i32;
    pub fn timekeeping_max_deferment() -> u64;
    pub fn timekeeping_warp_clock();
    pub fn timekeeping_suspend() -> i32;
    pub fn timekeeping_resume();

    #[cfg(feature = "CONFIG_GENERIC_SCHED_CLOCK")]
    pub fn sched_clock_suspend() -> i32;
    #[cfg(feature = "CONFIG_GENERIC_SCHED_CLOCK")]
    pub fn sched_clock_resume();

    pub fn update_process_times(user: i32);
    pub fn do_timer(ticks: c_ulong);
    pub fn update_wall_time();
}

#[cfg(not(feature = "CONFIG_GENERIC_SCHED_CLOCK"))]
#[inline]
pub fn sched_clock_suspend() -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_GENERIC_SCHED_CLOCK"))]
#[inline]
pub fn sched_clock_resume() {}

pub const CS_NAME_LEN: usize = 32;

extern "C" {
    pub static mut jiffies_lock: raw_spinlock_t;
    pub static mut jiffies_seq: seqcount_raw_spinlock_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
