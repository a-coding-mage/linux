/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/include/linux/clocksource.h. */

#[repr(C)]
pub struct clocksource_hw_snapshot {
    pub hw_cycles: u64,
    pub hw_csid: clocksource_ids,
}

#[repr(C)]
pub struct clocksource {
    pub read: Option<unsafe extern "C" fn(cs: *mut clocksource) -> u64>,
    pub mask: u64,
    pub mult: u32,
    pub shift: u32,
    pub max_idle_ns: u64,
    pub maxadj: u32,
    pub max_cycles: u64,
    pub max_raw_delta: u64,
    pub name: *const core::ffi::c_char,
    pub list: list_head,
    pub freq_khz: u32,
    pub rating: core::ffi::c_int,
    pub id: clocksource_ids,
    pub vdso_clock_mode: vdso_clock_mode,
    pub flags: core::ffi::c_ulong,
    pub base: *mut clocksource_base,
    pub read_snapshot: Option<unsafe extern "C" fn(cs: *mut clocksource, chs: *mut clocksource_hw_snapshot) -> u64>,
    pub enable: Option<unsafe extern "C" fn(cs: *mut clocksource) -> core::ffi::c_int>,
    pub disable: Option<unsafe extern "C" fn(cs: *mut clocksource)>,
    pub suspend: Option<unsafe extern "C" fn(cs: *mut clocksource)>,
    pub resume: Option<unsafe extern "C" fn(cs: *mut clocksource)>,
    pub mark_unstable: Option<unsafe extern "C" fn(cs: *mut clocksource)>,
    pub tick_stable: Option<unsafe extern "C" fn(cs: *mut clocksource)>,
    #[cfg(CONFIG_CLOCKSOURCE_WATCHDOG)]
    pub wd_list: list_head,
    #[cfg(CONFIG_CLOCKSOURCE_WATCHDOG)]
    pub cs_last: u64,
    #[cfg(CONFIG_CLOCKSOURCE_WATCHDOG)]
    pub wd_last: u64,
    #[cfg(CONFIG_CLOCKSOURCE_WATCHDOG)]
    pub wd_cpu: core::ffi::c_uint,
    pub owner: *mut module,
}

pub const CLOCK_SOURCE_IS_CONTINUOUS: core::ffi::c_ulong = 0x01;
pub const CLOCK_SOURCE_MUST_VERIFY: core::ffi::c_ulong = 0x02;
pub const CLOCK_SOURCE_CALIBRATED: core::ffi::c_ulong = 0x04;
pub const CLOCK_SOURCE_WATCHDOG: core::ffi::c_ulong = 0x10;
pub const CLOCK_SOURCE_VALID_FOR_HRES: core::ffi::c_ulong = 0x20;
pub const CLOCK_SOURCE_UNSTABLE: core::ffi::c_ulong = 0x40;
pub const CLOCK_SOURCE_SUSPEND_NONSTOP: core::ffi::c_ulong = 0x80;
pub const CLOCK_SOURCE_RESELECT: core::ffi::c_ulong = 0x100;
pub const CLOCK_SOURCE_CAN_INLINE_READ: core::ffi::c_ulong = 0x200;
pub const CLOCK_SOURCE_HAS_COUPLED_CLOCK_EVENT: core::ffi::c_ulong = 0x400;
pub const CLOCK_SOURCE_WDTEST: core::ffi::c_ulong = 0x800;
pub const CLOCK_SOURCE_WDTEST_PERCPU: core::ffi::c_ulong = 0x1000;

#[inline]
pub unsafe fn clocksource_freq2mult(freq: u32, shift_constant: u32, from: u64) -> u32 {
    let mut tmp = from << shift_constant;
    tmp += (freq / 2) as u64;
    tmp /= freq as u64;
    tmp as u32
}

#[inline]
pub unsafe fn clocksource_khz2mult(khz: u32, shift_constant: u32) -> u32 {
    clocksource_freq2mult(khz, shift_constant, NSEC_PER_MSEC as u64)
}

#[inline]
pub unsafe fn clocksource_hz2mult(hz: u32, shift_constant: u32) -> u32 {
    clocksource_freq2mult(hz, shift_constant, NSEC_PER_SEC as u64)
}

#[inline]
pub fn clocksource_cyc2ns(cycles: u64, mult: u32, shift: u32) -> i64 {
    ((cycles * mult as u64) >> shift) as i64
}

extern "C" {
    pub fn clocksource_unregister(cs: *mut clocksource) -> core::ffi::c_int;
    pub fn clocksource_touch_watchdog();
    pub fn clocksource_suspend();
    pub fn clocksource_resume();
    pub fn clocksource_default_clock() -> *mut clocksource;
    pub fn clocksource_mark_unstable(cs: *mut clocksource);
    pub fn clocksource_start_suspend_timing(cs: *mut clocksource, start_cycles: u64);
    pub fn clocksource_stop_suspend_timing(cs: *mut clocksource, now: u64) -> u64;
    pub fn clocks_calc_max_nsecs(mult: u32, shift: u32, maxadj: u32, mask: u64, max_cycles: *mut u64) -> u64;
    pub fn clocks_calc_mult_shift(mult: *mut u32, shift: *mut u32, from: u32, to: u32, minsec: u32);
    pub fn __clocksource_register_scale(cs: *mut clocksource, scale: u32, freq: u32) -> core::ffi::c_int;
    pub fn __devm_clocksource_register_scale(dev: *mut device, cs: *mut clocksource, scale: u32, freq: u32) -> core::ffi::c_int;
    pub fn timekeeping_notify(clock: *mut clocksource) -> core::ffi::c_int;
    pub fn clocksource_mmio_readl_up(cs: *mut clocksource) -> u64;
    pub fn clocksource_mmio_readl_down(cs: *mut clocksource) -> u64;
    pub fn clocksource_mmio_readw_up(cs: *mut clocksource) -> u64;
    pub fn clocksource_mmio_readw_down(cs: *mut clocksource) -> u64;
    pub fn clocksource_mmio_init(addr: *mut core::ffi::c_void, name: *const core::ffi::c_char, rating: core::ffi::c_ulong, type_: core::ffi::c_int, bits: core::ffi::c_uint, read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>) -> core::ffi::c_int;
    pub fn clocksource_i8253_init() -> core::ffi::c_int;
    #[cfg(CONFIG_TIMER_PROBE)]
    pub fn timer_probe();
}

#[inline]
pub unsafe fn __clocksource_register(cs: *mut clocksource) -> core::ffi::c_int {
    __clocksource_register_scale(cs, 1, 0)
}

#[inline]
pub unsafe fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> core::ffi::c_int {
    __clocksource_register_scale(cs, 1, hz)
}

#[inline]
pub unsafe fn clocksource_register_khz(cs: *mut clocksource, khz: u32) -> core::ffi::c_int {
    __clocksource_register_scale(cs, 1000, khz)
}

#[inline]
pub unsafe fn devm_clocksource_register_hz(dev: *mut device, cs: *mut clocksource, hz: u32) -> core::ffi::c_int {
    __devm_clocksource_register_scale(dev, cs, 1, hz)
}

#[inline]
pub unsafe fn devm_clocksource_register_khz(dev: *mut device, cs: *mut clocksource, khz: u32) -> core::ffi::c_int {
    __devm_clocksource_register_scale(dev, cs, 1000, khz)
}

#[cfg(CONFIG_ARCH_CLOCKSOURCE_INIT)]
extern "C" { pub fn clocksource_arch_init(cs: *mut clocksource); }

#[cfg(not(CONFIG_TIMER_PROBE))]
#[inline]
pub unsafe fn timer_probe() {}

#[repr(C)]
pub struct clocksource_base {
    pub id: clocksource_ids,
    pub freq_khz: u32,
    pub offset: u64,
    pub numerator: u32,
    pub denominator: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
