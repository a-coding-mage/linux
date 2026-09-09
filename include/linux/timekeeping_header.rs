/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers.

extern "C" {
    pub fn timekeeping_init();
    pub static mut timekeeping_suspended: ::core::ffi::c_int;
    pub fn legacy_timer_tick(ticks: ::core::ffi::c_ulong);
    pub fn do_settimeofday64(ts: *const timespec64) -> ::core::ffi::c_int;
    pub fn do_sys_settimeofday64(tv: *const timespec64, tz: *const timezone) -> ::core::ffi::c_int;
    pub fn ktime_get_raw_ts64(ts: *mut timespec64);
    pub fn ktime_get_ts64(ts: *mut timespec64);
    pub fn ktime_get_real_ts64(tv: *mut timespec64);
    pub fn ktime_get_coarse_ts64(ts: *mut timespec64);
    pub fn ktime_get_coarse_real_ts64(ts: *mut timespec64);
    pub fn ktime_get_coarse_real_ts64_mg(ts: *mut timespec64);
    pub fn ktime_get_real_ts64_mg(ts: *mut timespec64);
    pub fn timekeeping_get_mg_floor_swaps() -> ::core::ffi::c_ulong;
    pub fn getboottime64(ts: *mut timespec64);
    pub fn ktime_get_seconds() -> time64_t;
    pub fn __ktime_get_real_seconds() -> time64_t;
    pub fn ktime_get_real_seconds() -> time64_t;
    pub fn ktime_get() -> ktime_t;
    pub fn ktime_get_with_offset(offs: tk_offsets) -> ktime_t;
    pub fn ktime_get_coarse_with_offset(offs: tk_offsets) -> ktime_t;
    pub fn ktime_mono_to_any(tmono: ktime_t, offs: tk_offsets) -> ktime_t;
    pub fn ktime_get_raw() -> ktime_t;
    pub fn ktime_get_resolution_ns() -> u32;
    pub fn ktime_get_mono_fast_ns() -> u64;
    pub fn ktime_get_raw_fast_ns() -> u64;
    pub fn ktime_get_boot_fast_ns() -> u64;
    pub fn ktime_get_tai_fast_ns() -> u64;
    pub fn ktime_get_real_fast_ns() -> u64;
    pub fn timekeeping_rtc_skipsuspend() -> bool;
    pub fn timekeeping_rtc_skipresume() -> bool;
    pub fn timekeeping_inject_sleeptime64(delta: *const timespec64);
    pub fn ktime_real_to_base_clock(treal: ktime_t, base_id: clocksource_ids, cycles: *mut u64) -> bool;
    pub fn timekeeping_clocksource_has_base(id: clocksource_ids) -> bool;
    pub fn get_device_system_crosststamp(
        get_time_fn: Option<unsafe extern "C" fn(*mut ktime_t, *mut system_counterval_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        ctx: *mut ::core::ffi::c_void,
        history: *mut system_time_snapshot,
        xtstamp: *mut system_device_crosststamp,
    ) -> ::core::ffi::c_int;
    pub fn ktime_get_snapshot_id(clock_id: clockid_t, systime_snapshot: *mut system_time_snapshot);
    pub static mut persistent_clock_is_local: ::core::ffi::c_int;
    pub fn read_persistent_clock64(ts: *mut timespec64);
    pub fn read_persistent_wall_and_boot_offset(wall_clock: *mut timespec64, boot_offset: *mut timespec64);
    #[cfg(feature = "CONFIG_GENERIC_CMOS_UPDATE")]
    pub fn update_persistent_clock64(now: timespec64) -> ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum tk_offsets {
    TK_OFFS_REAL,
    TK_OFFS_BOOT,
    TK_OFFS_TAI,
    TK_OFFS_MAX,
}

pub unsafe fn ktime_get_real() -> ktime_t { ktime_get_with_offset(tk_offsets::TK_OFFS_REAL) }
pub unsafe fn ktime_get_coarse_real() -> ktime_t { ktime_get_coarse_with_offset(tk_offsets::TK_OFFS_REAL) }
pub unsafe fn ktime_get_boottime() -> ktime_t { ktime_get_with_offset(tk_offsets::TK_OFFS_BOOT) }
pub unsafe fn ktime_get_coarse_boottime() -> ktime_t { ktime_get_coarse_with_offset(tk_offsets::TK_OFFS_BOOT) }
pub unsafe fn ktime_get_clocktai() -> ktime_t { ktime_get_with_offset(tk_offsets::TK_OFFS_TAI) }
pub unsafe fn ktime_get_coarse_clocktai() -> ktime_t { ktime_get_coarse_with_offset(tk_offsets::TK_OFFS_TAI) }

pub unsafe fn ktime_get_coarse() -> ktime_t {
    let mut ts: timespec64 = ::core::mem::zeroed();
    ktime_get_coarse_ts64(&mut ts);
    timespec64_to_ktime(ts)
}
pub unsafe fn ktime_get_coarse_ns() -> u64 { ktime_to_ns(ktime_get_coarse()) }
pub unsafe fn ktime_get_coarse_real_ns() -> u64 { ktime_to_ns(ktime_get_coarse_real()) }
pub unsafe fn ktime_get_coarse_boottime_ns() -> u64 { ktime_to_ns(ktime_get_coarse_boottime()) }
pub unsafe fn ktime_get_coarse_clocktai_ns() -> u64 { ktime_to_ns(ktime_get_coarse_clocktai()) }
pub unsafe fn ktime_mono_to_real(mono: ktime_t) -> ktime_t { ktime_mono_to_any(mono, tk_offsets::TK_OFFS_REAL) }
pub unsafe fn ktime_get_ns() -> u64 { ktime_to_ns(ktime_get()) }
pub unsafe fn ktime_get_real_ns() -> u64 { ktime_to_ns(ktime_get_real()) }
pub unsafe fn ktime_get_boottime_ns() -> u64 { ktime_to_ns(ktime_get_boottime()) }
pub unsafe fn ktime_get_clocktai_ns() -> u64 { ktime_to_ns(ktime_get_clocktai()) }
pub unsafe fn ktime_get_raw_ns() -> u64 { ktime_to_ns(ktime_get_raw()) }

pub unsafe fn ktime_get_boottime_ts64(ts: *mut timespec64) { *ts = ktime_to_timespec64(ktime_get_boottime()); }
pub unsafe fn ktime_get_coarse_boottime_ts64(ts: *mut timespec64) { *ts = ktime_to_timespec64(ktime_get_coarse_boottime()); }
pub unsafe fn ktime_get_boottime_seconds() -> time64_t { ktime_divns(ktime_get_coarse_boottime(), NSEC_PER_SEC) }
pub unsafe fn ktime_get_clocktai_ts64(ts: *mut timespec64) { *ts = ktime_to_timespec64(ktime_get_clocktai()); }
pub unsafe fn ktime_get_coarse_clocktai_ts64(ts: *mut timespec64) { *ts = ktime_to_timespec64(ktime_get_coarse_clocktai()); }
pub unsafe fn ktime_get_clocktai_seconds() -> time64_t { ktime_divns(ktime_get_coarse_clocktai(), NSEC_PER_SEC) }

#[cfg(feature = "CONFIG_POSIX_AUX_CLOCKS")]
extern "C" {
    pub fn ktime_get_aux(id: clockid_t, kt: *mut ktime_t) -> bool;
    pub fn ktime_get_aux_ts64(id: clockid_t, kt: *mut timespec64) -> bool;
}
#[cfg(not(feature = "CONFIG_POSIX_AUX_CLOCKS"))]
pub unsafe fn ktime_get_aux(_id: clockid_t, _kt: *mut ktime_t) -> bool { false }
#[cfg(not(feature = "CONFIG_POSIX_AUX_CLOCKS"))]
pub unsafe fn ktime_get_aux_ts64(_id: clockid_t, _kt: *mut timespec64) -> bool { false }

#[repr(C)]
pub struct system_time_snapshot {
    pub cycles: u64, pub hw_cycles: u64, pub systime: ktime_t, pub monoraw: ktime_t,
    pub cs_id: clocksource_ids, pub hw_csid: clocksource_ids,
    pub clock_was_set_seq: u32, pub cs_was_changed_seq: u8, pub valid: u8,
}
#[repr(C)]
pub struct system_counterval_t { pub cycles: u64, pub cs_id: clocksource_ids, pub use_nsecs: bool }
#[repr(C)]
pub struct system_device_crosststamp {
    pub clock_id: clockid_t, pub device: ktime_t, pub sys_counter: system_counterval_t,
    pub sys_systime: ktime_t, pub sys_monoraw: ktime_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
