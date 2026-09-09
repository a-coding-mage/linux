/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * sched_clock.h: support for extending counters to full 64-bit ns counter
 */

// C build condition: CONFIG_GENERIC_SCHED_CLOCK.
#[cfg(feature = "CONFIG_GENERIC_SCHED_CLOCK")]
#[repr(C)]
pub struct clock_read_data {
    pub epoch_ns: u64,
    pub epoch_cyc: u64,
    pub sched_clock_mask: u64,
    pub read_sched_clock: Option<unsafe extern "C" fn() -> u64>,
    pub mult: u32,
    pub shift: u32,
}

#[cfg(feature = "CONFIG_GENERIC_SCHED_CLOCK")]
unsafe extern "C" {
    pub fn sched_clock_read_begin(seq: *mut core::ffi::c_uint) -> *mut clock_read_data;
    pub fn sched_clock_read_retry(seq: core::ffi::c_uint) -> core::ffi::c_int;
    pub fn generic_sched_clock_init();
    pub fn sched_clock_register(
        read: Option<unsafe extern "C" fn() -> u64>,
        bits: core::ffi::c_int,
        rate: core::ffi::c_ulong,
    );
}

#[cfg(not(feature = "CONFIG_GENERIC_SCHED_CLOCK"))]
#[inline]
pub const fn generic_sched_clock_init() {}

#[cfg(not(feature = "CONFIG_GENERIC_SCHED_CLOCK"))]
#[inline]
pub fn sched_clock_register(
    _read: Option<unsafe extern "C" fn() -> u64>,
    _bits: core::ffi::c_int,
    _rate: core::ffi::c_ulong,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
