/* SPDX-License-Identifier: GPL-2.0 */
/*
 * You SHOULD NOT be including this unless you're vsyscall
 * handling code or timekeeping internal code!
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/clocksource.h, linux/jiffies.h, linux/time.h

/**
 * timekeeper_ids - IDs for various time keepers in the kernel
 * @TIMEKEEPER_CORE:                The central core timekeeper managing system time
 * @TIMEKEEPER_AUX_FIRST:           The first AUX timekeeper
 * @TIMEKEEPER_AUX_LAST:            The last AUX timekeeper
 * @TIMEKEEPERS_MAX:                The maximum number of timekeepers managed
 */
#[repr(C)]
pub enum timekeeper_ids {
    TIMEKEEPER_CORE,
    #[cfg(feature = "CONFIG_POSIX_AUX_CLOCKS")]
    TIMEKEEPER_AUX_FIRST,
    #[cfg(feature = "CONFIG_POSIX_AUX_CLOCKS")]
    TIMEKEEPER_AUX_LAST = TIMEKEEPER_AUX_FIRST as isize + MAX_AUX_CLOCKS as isize - 1,
    TIMEKEEPERS_MAX,
}

/** Base structure for timekeeping readout. */
#[repr(C)]
pub struct tk_read_base {
    pub clock: *mut clocksource,
    pub mask: u64,
    pub cycle_last: u64,
    pub mult: u32,
    pub shift: u32,
    pub xtime_nsec: u64,
    pub base: ktime_t,
    pub base_real: u64,
}

/** Structure holding internal timekeeping values. */
#[repr(C)]
pub struct timekeeper {
    /* Cacheline 0 (together with prepended seqcount of timekeeper core): */
    pub tkr_mono: tk_read_base,

    /* Cacheline 1: */
    pub xtime_sec: u64,
    pub ktime_sec: usize,
    pub wall_to_monotonic: timespec64,
    pub offs_real: ktime_t,
    pub offs_boot: ktime_t,
    pub offs_tai_aux: timekeeper_offs_tai_aux,
    pub coarse_nsec: u32,
    pub id: timekeeper_ids,

    /* Cacheline 2: */
    pub tkr_raw: tk_read_base,
    pub raw_sec: u64,

    /* Cachline 3 and 4 (timekeeping internal variables): */
    pub cs_id: clocksource_ids,
    pub cs_ns_to_cyc_mult: u32,
    pub cs_ns_to_cyc_shift: u32,
    pub cs_ns_to_cyc_maxns: u64,
    pub clock_was_set_seq: u32,
    pub cs_was_changed_seq: u8,
    pub clock_valid: u8,
    pub monotonic_to_boot_aux: timekeeper_monotonic_to_boot_aux,
    pub cycle_interval: u64,
    pub xtime_interval: u64,
    pub raw_interval: u64,
    pub next_leap_ktime: ktime_t,
    pub ntp_tick: u64,
    pub ntp_error: i64,
    pub ntp_error_shift: u32,
    pub ntp_err_mult: u32,
    pub cs_tick_adj: i64,
    pub skip_second_overflow: u32,
    pub skew_delta: i64,
    pub tai_offset: i32,
}

#[repr(C)]
pub union timekeeper_offs_tai_aux {
    pub offs_tai: ktime_t,
    pub offs_aux: ktime_t,
}

#[repr(C)]
pub union timekeeper_monotonic_to_boot_aux {
    pub monotonic_to_boot: timespec64,
    pub monotonic_to_aux: timespec64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
