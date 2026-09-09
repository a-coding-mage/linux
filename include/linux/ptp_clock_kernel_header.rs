/* SPDX-License-Identifier: GPL-2.0-or-later */
/* PTP 1588 clock support */

// Dependencies supplied by the surrounding kernel bindings.

pub const PTP_CLOCK_NAME_LEN: usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PtpClockRequestType {
    PTP_CLK_REQ_EXTTS,
    PTP_CLK_REQ_PEROUT,
    PTP_CLK_REQ_PPS,
}

#[repr(C)]
pub union PtpClockRequestData {
    pub extts: ptp_extts_request,
    pub perout: ptp_perout_request,
}

#[repr(C)]
pub struct ptp_clock_request {
    pub type_: PtpClockRequestType,
    pub data: PtpClockRequestData,
}

pub struct system_device_crosststamp;

#[repr(C)]
pub struct ptp_system_timestamp {
    pub pre_sts: system_time_snapshot,
    pub post_sts: system_time_snapshot,
    pub clockid: clockid_t,
}

#[repr(C)]
pub struct ptp_clock_info {
    pub owner: *mut module,
    pub name: [::core::ffi::c_char; PTP_CLOCK_NAME_LEN],
    pub max_adj: s32,
    pub n_alarm: ::core::ffi::c_int,
    pub n_ext_ts: ::core::ffi::c_int,
    pub n_per_out: ::core::ffi::c_int,
    pub n_pins: ::core::ffi::c_int,
    pub n_per_lp: ::core::ffi::c_int,
    pub pps: ::core::ffi::c_int,
    pub supported_perout_flags: ::core::ffi::c_uint,
    pub supported_extts_flags: ::core::ffi::c_uint,
    pub pin_config: *mut ptp_pin_desc,
    pub adjfine: Option<unsafe extern "C" fn(*mut ptp_clock_info, ::core::ffi::c_long) -> ::core::ffi::c_int>,
    pub adjphase: Option<unsafe extern "C" fn(*mut ptp_clock_info, s32) -> ::core::ffi::c_int>,
    pub getmaxphase: Option<unsafe extern "C" fn(*mut ptp_clock_info) -> s32>,
    pub adjtime: Option<unsafe extern "C" fn(*mut ptp_clock_info, s64) -> ::core::ffi::c_int>,
    pub gettime64: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut timespec64) -> ::core::ffi::c_int>,
    pub gettimex64: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut timespec64, *mut ptp_system_timestamp) -> ::core::ffi::c_int>,
    pub getcrosststamp: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut system_device_crosststamp) -> ::core::ffi::c_int>,
    pub settime64: Option<unsafe extern "C" fn(*mut ptp_clock_info, *const timespec64) -> ::core::ffi::c_int>,
    pub getcycles64: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut timespec64) -> ::core::ffi::c_int>,
    pub getcyclesx64: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut timespec64, *mut ptp_system_timestamp) -> ::core::ffi::c_int>,
    pub getcrosscycles: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut system_device_crosststamp) -> ::core::ffi::c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut ptp_clock_info, *mut ptp_clock_request, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub verify: Option<unsafe extern "C" fn(*mut ptp_clock_info, ::core::ffi::c_uint, ptp_pin_function, ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub do_aux_work: Option<unsafe extern "C" fn(*mut ptp_clock_info) -> ::core::ffi::c_long>,
    pub perout_loopback: Option<unsafe extern "C" fn(*mut ptp_clock_info, ::core::ffi::c_uint, ::core::ffi::c_int) -> ::core::ffi::c_int>,
}

pub struct ptp_clock;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ptp_clock_events {
    PTP_CLOCK_ALARM,
    PTP_CLOCK_EXTTS,
    PTP_CLOCK_EXTOFF,
    PTP_CLOCK_PPS,
    PTP_CLOCK_PPSUSR,
}

#[repr(C)]
pub union ptp_clock_event_data {
    pub timestamp: u64,
    pub offset: s64,
    pub pps_times: pps_event_time,
}

#[repr(C)]
pub struct ptp_clock_event {
    pub type_: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub data: ptp_clock_event_data,
}

#[inline]
pub unsafe fn scaled_ppm_to_ppb(ppm: ::core::ffi::c_long) -> ::core::ffi::c_long {
    let mut ppb: s64 = 1i64.wrapping_add(ppm as s64);
    ppb = ppb.wrapping_mul(125);
    ppb >>= 13;
    ppb as ::core::ffi::c_long
}

#[inline]
pub unsafe fn diff_by_scaled_ppm(base: u64, mut scaled_ppm: ::core::ffi::c_long, diff: *mut u64) -> bool {
    let mut negative = false;
    if scaled_ppm < 0 {
        negative = true;
        scaled_ppm = -scaled_ppm;
    }
    *diff = mul_u64_u64_div_u64(base, scaled_ppm as u64, 1000000u64 << 16);
    negative
}

#[inline]
pub unsafe fn adjust_by_scaled_ppm(base: u64, scaled_ppm: ::core::ffi::c_long) -> u64 {
    let mut diff = 0u64;
    if diff_by_scaled_ppm(base, scaled_ppm, &mut diff) { base.wrapping_sub(diff) } else { base.wrapping_add(diff) }
}

// The following declarations are enabled by CONFIG_PTP_1588_CLOCK in the C header.
#[cfg(feature = "CONFIG_PTP_1588_CLOCK")]
extern "C" {
    pub fn ptp_clock_register(info: *mut ptp_clock_info, parent: *mut device) -> *mut ptp_clock;
    pub fn ptp_clock_unregister(ptp: *mut ptp_clock) -> ::core::ffi::c_int;
    pub fn ptp_clock_event(ptp: *mut ptp_clock, event: *mut ptp_clock_event);
    pub fn ptp_clock_index(ptp: *mut ptp_clock) -> ::core::ffi::c_int;
    pub fn ptp_clock_index_by_of_node(np: *mut device_node) -> ::core::ffi::c_int;
    pub fn ptp_clock_index_by_dev(parent: *mut device) -> ::core::ffi::c_int;
    pub fn ptp_find_pin(ptp: *mut ptp_clock, func: ptp_pin_function, chan: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ptp_find_pin_unlocked(ptp: *mut ptp_clock, func: ptp_pin_function, chan: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn ptp_schedule_worker(ptp: *mut ptp_clock, delay: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn ptp_cancel_worker_sync(ptp: *mut ptp_clock);
}

#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_register(_: *mut ptp_clock_info, _: *mut device) -> *mut ptp_clock { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_unregister(_: *mut ptp_clock) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_event(_: *mut ptp_clock, _: *mut ptp_clock_event) {}
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_index(_: *mut ptp_clock) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_index_by_of_node(_: *mut device_node) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_clock_index_by_dev(_: *mut device) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_find_pin(_: *mut ptp_clock, _: ptp_pin_function, _: ::core::ffi::c_uint) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_find_pin_unlocked(_: *mut ptp_clock, _: ptp_pin_function, _: ::core::ffi::c_uint) -> ::core::ffi::c_int { -1 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_schedule_worker(_: *mut ptp_clock, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int { -95 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK"))]
#[inline] pub unsafe fn ptp_cancel_worker_sync(_: *mut ptp_clock) {}

#[cfg(feature = "CONFIG_PTP_1588_CLOCK_BUILTIN")]
extern "C" {
    pub fn ptp_get_vclocks_index(pclock_index: ::core::ffi::c_int, vclock_index: *mut *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ptp_convert_timestamp(hwtstamp: *const ktime_t, vclock_index: ::core::ffi::c_int) -> ktime_t;
}
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK_BUILTIN"))]
#[inline] pub unsafe fn ptp_get_vclocks_index(_: ::core::ffi::c_int, _: *mut *mut ::core::ffi::c_int) -> ::core::ffi::c_int { 0 }
#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK_BUILTIN"))]
#[inline] pub unsafe fn ptp_convert_timestamp(_: *const ktime_t, _: ::core::ffi::c_int) -> ktime_t { 0 }

#[inline]
pub unsafe fn ptp_read_system_prets(sts: *mut ptp_system_timestamp) {
    if !sts.is_null() { ktime_get_snapshot_id((*sts).clockid, &mut (*sts).pre_sts); }
}

#[inline]
pub unsafe fn ptp_read_system_postts(sts: *mut ptp_system_timestamp) {
    if !sts.is_null() { ktime_get_snapshot_id((*sts).clockid, &mut (*sts).post_sts); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
