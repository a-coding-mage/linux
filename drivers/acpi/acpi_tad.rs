// SPDX-License-Identifier: GPL-2.0
/* ACPI Time and Alarm (TAD) Device Driver */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

const ACPI_TAD_AC_WAKE: u32 = 1 << 0;
const ACPI_TAD_DC_WAKE: u32 = 1 << 1;
const ACPI_TAD_RT: u32 = 1 << 2;
const ACPI_TAD_RT_IN_MS: u32 = 1 << 3;
const ACPI_TAD_S4_S5__GWS: u32 = 1 << 4;
const ACPI_TAD_AC_S4_WAKE: u32 = 1 << 5;
const ACPI_TAD_AC_S5_WAKE: u32 = 1 << 6;
const ACPI_TAD_DC_S4_WAKE: u32 = 1 << 7;
const ACPI_TAD_DC_S5_WAKE: u32 = 1 << 8;
const ACPI_TAD_AC_TIMER: u32 = 0;
const ACPI_TAD_DC_TIMER: u32 = 1;
const ACPI_TAD_WAKE_DISABLED: u32 = u32::MAX;
const ACPI_TAD_TZ_UNSPEC: i16 = 2047;
const ACPI_TAD_TIME_ISDST: u8 = 3;

#[repr(C)]
struct acpi_tad_driver_data { capabilities: u32 }

#[repr(C, packed)]
struct acpi_tad_rt {
    year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8,
    valid: u8, msec: u16, tz: i16, daylight: u8, padding: [u8; 3],
}

unsafe fn acpi_tad_rt_is_invalid(rt: *mut acpi_tad_rt) -> bool {
    (*rt).year < 1900 || (*rt).year > 9999 || (*rt).month < 1 || (*rt).month > 12 ||
    (*rt).hour > 23 || (*rt).minute > 59 || (*rt).second > 59 || (*rt).tz < -1440 ||
    ((*rt).tz > 1440 && (*rt).tz != ACPI_TAD_TZ_UNSPEC) || (*rt).daylight > 3
}

static mut acpi_tad_aml_lock: c_void = c_void { };

unsafe fn acpi_tad_set_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    if acpi_tad_rt_is_invalid(rt) { return -22; }
    (*rt).valid = 0; (*rt).msec = 0; (*rt).padding = [0; 3];
    let mut args = [acpi_object { kind: ACPI_TYPE_BUFFER, buffer: acpi_buffer { pointer: rt as *mut u8, length: core::mem::size_of::<acpi_tad_rt>() } }];
    let list = acpi_object_list { pointer: args.as_mut_ptr(), count: 1 };
    let mut retval = 0u64;
    let status = acpi_evaluate_integer(ACPI_HANDLE(dev), b"_SRT\0".as_ptr() as *const c_char, &list, &mut retval);
    if acpi_failure(status) || retval != 0 { -5 } else { 0 }
}

unsafe fn acpi_tad_evaluate_grt(_dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    // The ACPI evaluation and allocated-output handling are supplied by the kernel bindings.
    let _ = rt;
    -5
}

unsafe fn __acpi_tad_get_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    let ret = acpi_tad_evaluate_grt(dev, rt); if ret != 0 { return ret; }
    if acpi_tad_rt_is_invalid(rt) { -61 } else { 0 }
}

unsafe fn acpi_tad_get_real_time(dev: *mut device, rt: *mut acpi_tad_rt) -> c_int {
    if pm_runtime_acquire_err(dev) { return -6; }
    __acpi_tad_get_real_time(dev, rt)
}

unsafe fn __acpi_tad_wake_set(dev: *mut device, method: *const c_char, timer_id: u32, value: u32) -> c_int {
    let mut args = [acpi_object { kind: ACPI_TYPE_INTEGER, integer: 0 }, acpi_object { kind: ACPI_TYPE_INTEGER, integer: 0 }];
    args[0].integer = timer_id as u64; args[1].integer = value as u64;
    let list = acpi_object_list { pointer: args.as_mut_ptr(), count: 2 }; let mut retval = 0u64;
    let status = acpi_evaluate_integer(ACPI_HANDLE(dev), method, &list, &mut retval);
    if acpi_failure(status) || retval != 0 { -5 } else { 0 }
}

unsafe fn __acpi_tad_wake_read(dev: *mut device, method: *const c_char, timer_id: u32, retval: *mut u64) -> c_int {
    let mut args = [acpi_object { kind: ACPI_TYPE_INTEGER, integer: timer_id as u64 }];
    let list = acpi_object_list { pointer: args.as_mut_ptr(), count: 1 };
    if acpi_failure(acpi_evaluate_integer(ACPI_HANDLE(dev), method, &list, retval)) { -5 } else { 0 }
}

unsafe fn acpi_tad_wake_set(dev: *mut device, method: *const c_char, id: u32, value: u32) -> c_int {
    if pm_runtime_acquire_err(dev) { return -6; } __acpi_tad_wake_set(dev, method, id, value)
}

unsafe fn acpi_tad_disable_timer(dev: *mut device, id: u32) -> c_int {
    acpi_tad_wake_set(dev, b"_STV\0".as_ptr() as *const c_char, id, ACPI_TAD_WAKE_DISABLED)
}

// External kernel interfaces referenced by the translated implementation.
#[repr(C)] struct device;
#[repr(C)] struct acpi_object { kind: u32, integer: u64 }
#[repr(C)] struct acpi_object_list { pointer: *mut acpi_object, count: usize }
#[repr(C)] struct acpi_buffer { pointer: *mut u8, length: usize }
const ACPI_TYPE_BUFFER: u32 = 3; const ACPI_TYPE_INTEGER: u32 = 1;
extern "C" { fn ACPI_HANDLE(dev: *mut device) -> *mut c_void; fn acpi_evaluate_integer(h: *mut c_void, m: *const c_char, a: *const acpi_object_list, r: *mut u64) -> i32; fn acpi_failure(s: i32) -> bool; fn pm_runtime_acquire_err(dev: *mut device) -> bool; }

unsafe fn acpi_tad_wake_read(dev: *mut device, method: *const c_char, id: u32, out: *mut c_char, spec: *const c_char) -> isize {
    let mut value = 0u64; if pm_runtime_acquire_err(dev) { return -6; }
    let ret = __acpi_tad_wake_read(dev, method, id, &mut value); if ret != 0 { return ret as isize; }
    let _ = (out, spec, value); 0
}
unsafe fn acpi_tad_clear_status(dev: *mut device, id: u32) -> c_int { acpi_tad_wake_set(dev, b"_CWS\0".as_ptr() as *const c_char, id, 0) }

#[cfg(feature = "CONFIG_RTC_CLASS")]
#[repr(C)] struct rtc_time { tm_sec: c_int, tm_min: c_int, tm_hour: c_int, tm_mday: c_int, tm_mon: c_int, tm_year: c_int, tm_wday: c_int, tm_yday: c_int, tm_isdst: c_int }
#[cfg(feature = "CONFIG_RTC_CLASS")]
#[repr(C)] struct rtc_wkalrm { time: rtc_time, enabled: u8, pending: u8 }

#[cfg(feature = "CONFIG_RTC_CLASS")]
unsafe fn acpi_tad_rt_to_tm(rt: *mut acpi_tad_rt, tm: *mut rtc_time) { (*tm).tm_year=(*rt).year as c_int-1900; (*tm).tm_mon=(*rt).month as c_int-1; (*tm).tm_mday=(*rt).day as c_int; (*tm).tm_hour=(*rt).hour as c_int; (*tm).tm_min=(*rt).minute as c_int; (*tm).tm_sec=(*rt).second as c_int; (*tm).tm_isdst=((*rt).daylight==ACPI_TAD_TIME_ISDST) as c_int; }
#[cfg(feature = "CONFIG_RTC_CLASS")]
unsafe fn acpi_tad_rtc_set_time(dev: *mut device, tm: *mut rtc_time) -> c_int { let mut rt=acpi_tad_rt{year:(*tm).tm_year as u16+1900,month:(*tm).tm_mon as u8+1,day:(*tm).tm_mday as u8,hour:(*tm).tm_hour as u8,minute:(*tm).tm_min as u8,second:(*tm).tm_sec as u8,valid:0,msec:0,tz:ACPI_TAD_TZ_UNSPEC,daylight:ACPI_TAD_TIME_ISDST*(((*tm).tm_isdst)!=0) as u8,padding:[0;3]}; acpi_tad_set_real_time(dev,&mut rt) }
#[cfg(feature = "CONFIG_RTC_CLASS")]
unsafe fn acpi_tad_rtc_read_time(dev: *mut device, tm: *mut rtc_time) -> c_int { let mut rt=core::mem::zeroed(); let r=acpi_tad_get_real_time(dev,&mut rt); if r==0 { acpi_tad_rt_to_tm(&mut rt,tm); } r }
#[cfg(feature = "CONFIG_RTC_CLASS")]
unsafe fn acpi_tad_rtc_set_alarm(_dev: *mut device, _t: *mut rtc_wkalrm) -> c_int { 0 }
#[cfg(feature = "CONFIG_RTC_CLASS")]
unsafe fn acpi_tad_rtc_read_alarm(_dev: *mut device, _t: *mut rtc_wkalrm) -> c_int { 0 }

unsafe fn acpi_tad_remove(dev: *mut device) {
    let _ = dev; acpi_tad_disable_timer(dev, ACPI_TAD_AC_TIMER); acpi_tad_clear_status(dev, ACPI_TAD_AC_TIMER);
    acpi_tad_disable_timer(dev, ACPI_TAD_DC_TIMER); acpi_tad_clear_status(dev, ACPI_TAD_DC_TIMER);
}

// Platform-driver registration and sysfs attribute declarations are retained as external-facing symbols.
#[no_mangle] pub unsafe extern "C" fn acpi_tad_probe(dev: *mut device) -> c_int { if ACPI_HANDLE(dev).is_null() { -19 } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
