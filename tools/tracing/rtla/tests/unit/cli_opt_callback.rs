// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_longlong, c_void};
use core::ptr;

#[repr(C)]
pub struct option {
    pub short_name: c_int,
    pub long_name: *const c_char,
    pub value: *mut c_void,
    pub arg_name: *const c_char,
    pub help: *const c_char,
    pub callback: Option<unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int>,
    pub data: *const c_void,
}

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TCase {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_set_t {
    _private: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sched_attr {
    pub sched_policy: c_int,
    pub sched_priority: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_params {
    pub cpus: *const c_char,
    pub cgroup: c_int,
    pub cgroup_name: *const c_char,
    pub hk_cpus: c_int,
    pub hk_cpu_set: cpu_set_t,
    pub sched_param: sched_attr,
    pub duration: c_longlong,
    pub stop_us: c_longlong,
    pub stop_total_us: c_longlong,
    pub aa_only: c_int,
    pub user_workload: c_int,
    pub user_data: c_int,
    pub output_divisor: c_int,
}

#[repr(C)]
pub struct trace_events {
    pub system: *const c_char,
    pub event: *const c_char,
    pub trigger: *const c_char,
    pub filter: *const c_char,
    pub next: *mut trace_events,
}

#[repr(C)]
pub struct osnoise_params {
    pub common: common_params,
    pub threshold: c_int,
}

#[repr(C)]
pub struct osnoise_cb_data {
    pub params: *mut osnoise_params,
    pub trace_output: *const c_char,
}

#[repr(C)]
pub struct timerlat_params {
    pub common: common_params,
    pub print_stack: c_longlong,
    pub timerlat_align: c_int,
    pub timerlat_align_us: c_longlong,
}

#[repr(C)]
pub struct timerlat_cb_data {
    pub params: *mut timerlat_params,
    pub trace_output: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct action {
    pub type_: c_int,
    pub trace_output: *const c_char,
}

#[repr(C)]
pub struct actions {
    pub len: c_int,
    pub list: [action; 16],
}

#[repr(C)]
pub struct llong_range {
    pub lo: c_longlong,
    pub hi: c_longlong,
}

#[repr(C)]
pub struct int_range {
    pub lo: c_int,
    pub hi: c_int,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static default_output_divisor: c_int;
    static default_stack_format: c_int;
    static stderr: *mut c_void;

    static SCHED_FIFO: c_int;
    static ACTION_TRACE_OUTPUT: c_int;
    static STACK_FORMAT_FULL: c_int;

    fn opt_llong_callback(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_int_callback(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_cpus_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_cgroup_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_duration_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_event_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_housekeeping_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_priority_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_trigger_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_filter_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_osnoise_auto_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_osnoise_trace_output_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_osnoise_on_threshold_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_osnoise_on_end_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_timerlat_auto_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_aa_only_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_timerlat_trace_output_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_timerlat_on_threshold_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_timerlat_on_end_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_user_threads_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_nano_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_timerlat_align_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn opt_stack_format_cb(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
    fn trace_event_alloc(event: *const c_char) -> *mut trace_events;

    fn suite_create(name: *const c_char) -> *mut Suite;
    fn tcase_create(name: *const c_char) -> *mut TCase;
    fn tcase_add_test(tc: *mut TCase, test: unsafe extern "C" fn(c_int));
    fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);
    fn freopen(path: *const c_char, mode: *const c_char, stream: *mut c_void) -> *mut c_void;
    fn CPU_COUNT(set: *const cpu_set_t) -> c_int;
}

const fn LLONG_RANGE(lo: c_longlong, hi: c_longlong) -> llong_range {
    llong_range { lo, hi }
}

const fn INT_RANGE(lo: c_int, hi: c_int) -> int_range {
    int_range { lo, hi }
}

fn TEST_CALLBACK(
    value: *mut c_void,
    cb: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
) -> option {
    option {
        short_name: b't' as c_int,
        long_name: c"test".as_ptr(),
        value,
        arg_name: c"test value".as_ptr(),
        help: c"test help".as_ptr(),
        callback: Some(cb),
        data: ptr::null(),
    }
}

fn TEST_LLONG_RANGE(value: *mut c_void, lo: c_longlong, hi: c_longlong) -> option {
    let range = Box::leak(Box::new(LLONG_RANGE(lo, hi)));
    let mut opt = TEST_CALLBACK(value, opt_llong_callback);
    opt.data = range as *const llong_range as *const c_void;
    opt
}

fn TEST_INT_RANGE(value: *mut c_void, lo: c_int, hi: c_int) -> option {
    let range = Box::leak(Box::new(INT_RANGE(lo, hi)));
    let mut opt = TEST_CALLBACK(value, opt_int_callback);
    opt.data = range as *const int_range as *const c_void;
    opt
}

fn RTLA_OPT_LLONG_DEFVAL(
    short_name: c_int,
    long_name: *const c_char,
    value: *mut c_longlong,
    arg_name: *const c_char,
    help: *const c_char,
    default_value: *const c_longlong,
) -> option {
    option {
        short_name,
        long_name,
        value: value as *mut c_void,
        arg_name,
        help,
        callback: Some(opt_llong_callback),
        data: default_value as *const c_void,
    }
}

fn RTLA_OPT_INT_DEFVAL(
    short_name: c_int,
    long_name: *const c_char,
    value: *mut c_int,
    arg_name: *const c_char,
    help: *const c_char,
    default_value: c_int,
) -> option {
    let default_value = Box::leak(Box::new(default_value));
    option {
        short_name,
        long_name,
        value: value as *mut c_void,
        arg_name,
        help,
        callback: Some(opt_int_callback),
        data: default_value as *const c_int as *const c_void,
    }
}

fn RTLA_OPT_CALLBACK_DATA(
    short_name: c_int,
    long_name: *const c_char,
    value: *mut c_void,
    arg_name: *const c_char,
    help: *const c_char,
    cb: unsafe extern "C" fn(*const option, *const c_char, c_int) -> c_int,
    data: *const c_void,
) -> option {
    option {
        short_name,
        long_name,
        value,
        arg_name,
        help,
        callback: Some(cb),
        data,
    }
}

macro_rules! ck_assert_int_eq {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b)
    };
}

macro_rules! ck_assert_ptr_null {
    ($p:expr) => {
        assert!(($p).is_null())
    };
}

macro_rules! ck_assert_ptr_eq {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b)
    };
}

macro_rules! ck_assert_str_eq {
    ($a:expr, $b:expr) => {
        assert_eq!(std::ffi::CStr::from_ptr($a).to_bytes(), $b)
    };
}

macro_rules! redirect_stderr_to_null {
    () => {
        assert!(!freopen(c"/dev/null".as_ptr(), c"w".as_ptr(), stderr).is_null())
    };
}

macro_rules! CLI_ASSERT_CPUSET {
    ($set:expr, $($cpu:expr),+ $(,)?) => {
        {
            let expected = [$( $cpu ),+];
            ck_assert_int_eq!(CPU_COUNT(&$set), expected.len() as c_int);
        }
    };
}

unsafe extern "C" fn test_opt_llong_callback_simple(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 1234567890);
}

unsafe extern "C" fn test_opt_llong_callback_max(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"9223372036854775807".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 9223372036854775807_i64);
}

unsafe extern "C" fn test_opt_llong_callback_min(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"-9223372036854775808".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, !9223372036854775807_i64);
}

unsafe extern "C" fn test_opt_llong_callback_non_numeric(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_llong_callback(&opt, c"abc".as_ptr(), 0), -1);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_llong_callback_non_numeric_suffix(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_llong_callback(&opt, c"1234567890abc".as_ptr(), 0), -1);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_llong_callback_unset(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_llong_callback);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_llong_callback(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_llong_callback_unset_defval(_: c_int) {
    let mut test_value: c_longlong = 0;
    let default_value: c_longlong = 42;
    let opt = RTLA_OPT_LLONG_DEFVAL(
        b't' as c_int,
        c"test".as_ptr(),
        &mut test_value,
        c"test value".as_ptr(),
        c"test help".as_ptr(),
        &default_value,
    );

    ck_assert_int_eq!(opt_llong_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_llong_callback(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(test_value, default_value);
}

unsafe extern "C" fn test_opt_int_callback_simple(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    ck_assert_int_eq!(opt_int_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 1234567890);
}

unsafe extern "C" fn test_opt_int_callback_max(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    ck_assert_int_eq!(opt_int_callback(&opt, c"2147483647".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 2147483647);
}

unsafe extern "C" fn test_opt_int_callback_min(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    ck_assert_int_eq!(opt_int_callback(&opt, c"-2147483648".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, -2147483648);
}

unsafe extern "C" fn test_opt_int_callback_non_numeric(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_int_callback(&opt, c"abc".as_ptr(), 0), -1);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_int_callback_non_numeric_suffix(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_int_callback(&opt, c"1234567890abc".as_ptr(), 0), -1);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_int_callback_unset(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_CALLBACK(&mut test_value as *mut _ as *mut c_void, opt_int_callback);

    ck_assert_int_eq!(opt_int_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_int_callback(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(test_value, 0);
}

unsafe extern "C" fn test_opt_int_callback_unset_defval(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = RTLA_OPT_INT_DEFVAL(
        b't' as c_int,
        c"test".as_ptr(),
        &mut test_value,
        c"test value".as_ptr(),
        c"test help".as_ptr(),
        42,
    );

    ck_assert_int_eq!(opt_int_callback(&opt, c"1234567890".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_int_callback(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(test_value, 42);
}

unsafe extern "C" fn test_opt_llong_callback_range_in(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_LLONG_RANGE(&mut test_value as *mut _ as *mut c_void, 10, 100);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"50".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 50);
}

unsafe extern "C" fn test_opt_llong_callback_range_below(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_LLONG_RANGE(&mut test_value as *mut _ as *mut c_void, 10, 100);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_llong_callback(&opt, c"9".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_llong_callback_range_above(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_LLONG_RANGE(&mut test_value as *mut _ as *mut c_void, 10, 100);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_llong_callback(&opt, c"101".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_llong_callback_range_boundary(_: c_int) {
    let mut test_value: c_longlong = 0;
    let opt = TEST_LLONG_RANGE(&mut test_value as *mut _ as *mut c_void, 10, 100);

    ck_assert_int_eq!(opt_llong_callback(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 10);
    ck_assert_int_eq!(opt_llong_callback(&opt, c"100".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 100);
}

unsafe extern "C" fn test_opt_int_callback_range_in(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_INT_RANGE(&mut test_value as *mut _ as *mut c_void, 0, 10000);

    ck_assert_int_eq!(opt_int_callback(&opt, c"5000".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 5000);
}

unsafe extern "C" fn test_opt_int_callback_range_below(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_INT_RANGE(&mut test_value as *mut _ as *mut c_void, 0, 10000);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_int_callback(&opt, c"-1".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_int_callback_range_above(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_INT_RANGE(&mut test_value as *mut _ as *mut c_void, 0, 10000);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_int_callback(&opt, c"10001".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_int_callback_range_boundary(_: c_int) {
    let mut test_value: c_int = 0;
    let opt = TEST_INT_RANGE(&mut test_value as *mut _ as *mut c_void, 0, 10000);

    ck_assert_int_eq!(opt_int_callback(&opt, c"0".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 0);
    ck_assert_int_eq!(opt_int_callback(&opt, c"10000".as_ptr(), 0), 0);
    ck_assert_int_eq!(test_value, 10000);
}

unsafe extern "C" fn test_opt_cpus_cb(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_cpus_cb);

    nr_cpus = 4;
    ck_assert_int_eq!(opt_cpus_cb(&opt, c"0-3".as_ptr(), 0), 0);
    ck_assert_str_eq!(params.cpus, b"0-3");
}

unsafe extern "C" fn test_opt_cpus_cb_invalid(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_cpus_cb);

    nr_cpus = 4;
    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_cpus_cb(&opt, c"0-3,5".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_cgroup_cb(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_cgroup_cb);

    ck_assert_int_eq!(opt_cgroup_cb(&opt, c"cgroup".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.cgroup, 1);
    ck_assert_str_eq!(params.cgroup_name, b"cgroup");
}

unsafe extern "C" fn test_opt_cgroup_cb_equals(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_cgroup_cb);

    ck_assert_int_eq!(opt_cgroup_cb(&opt, c"=cgroup".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.cgroup, 1);
    ck_assert_str_eq!(params.cgroup_name, b"cgroup");
}

unsafe extern "C" fn test_opt_cgroup_cb_unset(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_cgroup_cb);

    ck_assert_int_eq!(opt_cgroup_cb(&opt, c"cgroup".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_cgroup_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.cgroup, 0);
    ck_assert_ptr_null!(params.cgroup_name);
}

unsafe extern "C" fn test_opt_duration_cb(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_duration_cb);

    ck_assert_int_eq!(opt_duration_cb(&opt, c"1m".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.duration, 60);
}

unsafe extern "C" fn test_opt_duration_cb_invalid(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_duration_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_duration_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_duration_cb_unset(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_duration_cb);

    ck_assert_int_eq!(opt_duration_cb(&opt, c"1m".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_duration_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.duration, 0);
}

unsafe extern "C" fn test_opt_event_cb(_: c_int) {
    let mut events: *mut trace_events = ptr::null_mut();
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_event_cb);

    ck_assert_int_eq!(opt_event_cb(&opt, c"sched:sched_switch".as_ptr(), 0), 0);
    ck_assert_str_eq!((*events).system, b"sched");
    ck_assert_str_eq!((*events).event, b"sched_switch");
    ck_assert_ptr_eq!((*events).next, ptr::null_mut());
}

unsafe extern "C" fn test_opt_event_cb_multiple(_: c_int) {
    let mut events: *mut trace_events = ptr::null_mut();
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_event_cb);

    ck_assert_int_eq!(opt_event_cb(&opt, c"sched:sched_switch".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_event_cb(&opt, c"sched:sched_wakeup".as_ptr(), 0), 0);
    ck_assert_str_eq!((*events).system, b"sched");
    ck_assert_str_eq!((*events).event, b"sched_wakeup");
    ck_assert_str_eq!((*(*events).next).system, b"sched");
    ck_assert_str_eq!((*(*events).next).event, b"sched_switch");
    ck_assert_ptr_eq!((*(*events).next).next, ptr::null_mut());
}

unsafe extern "C" fn test_opt_housekeeping_cb(_: c_int) {
    let mut __params: common_params = core::mem::zeroed();
    let params: *mut common_params = &mut __params;
    let opt = TEST_CALLBACK(params as *mut c_void, opt_housekeeping_cb);

    nr_cpus = 4;
    ck_assert_int_eq!(opt_housekeeping_cb(&opt, c"0-3".as_ptr(), 0), 0);
    ck_assert_int_eq!((*params).hk_cpus, 1);
    CLI_ASSERT_CPUSET!((*params).hk_cpu_set, 0, 1, 2, 3);
}

unsafe extern "C" fn test_opt_housekeeping_cb_invalid(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_housekeeping_cb);

    nr_cpus = 4;
    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_housekeeping_cb(&opt, c"0-3,5".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_housekeeping_cb_unset(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_housekeeping_cb);

    nr_cpus = 4;
    ck_assert_int_eq!(opt_housekeeping_cb(&opt, c"0-3".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_housekeeping_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.hk_cpus, 0);
    ck_assert_int_eq!(CPU_COUNT(&params.hk_cpu_set), 0);
}

unsafe extern "C" fn test_opt_priority_cb(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_priority_cb);

    ck_assert_int_eq!(opt_priority_cb(&opt, c"f:95".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.sched_param.sched_policy, SCHED_FIFO);
    ck_assert_int_eq!(params.sched_param.sched_priority, 95);
}

unsafe extern "C" fn test_opt_priority_cb_invalid(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_priority_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_priority_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_priority_cb_unset(_: c_int) {
    let mut params: common_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_priority_cb);

    ck_assert_int_eq!(opt_priority_cb(&opt, c"f:95".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_priority_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.sched_param.sched_policy, 0);
    ck_assert_int_eq!(params.sched_param.sched_priority, 0);
}

unsafe extern "C" fn test_opt_trigger_cb(_: c_int) {
    let mut events = trace_event_alloc(c"sched:sched_switch".as_ptr());
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_trigger_cb);

    ck_assert_int_eq!(opt_trigger_cb(&opt, c"stacktrace".as_ptr(), 0), 0);
    ck_assert_str_eq!((*events).trigger, b"stacktrace");
}

unsafe extern "C" fn test_opt_trigger_cb_no_event(_: c_int) {
    let mut events: *mut trace_events = ptr::null_mut();
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_trigger_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_trigger_cb(&opt, c"stacktrace".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_filter_cb(_: c_int) {
    let mut events = trace_event_alloc(c"sched:sched_switch".as_ptr());
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_filter_cb);

    ck_assert_int_eq!(opt_filter_cb(&opt, c"comm ~ \"rtla\"".as_ptr(), 0), 0);
    ck_assert_str_eq!((*events).filter, b"comm ~ \"rtla\"");
}

unsafe extern "C" fn test_opt_filter_cb_no_event(_: c_int) {
    let mut events: *mut trace_events = ptr::null_mut();
    let opt = TEST_CALLBACK(&mut events as *mut _ as *mut c_void, opt_filter_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_filter_cb(&opt, c"comm ~ \"rtla\"".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_osnoise_auto_cb(_: c_int) {
    let mut params: osnoise_params = core::mem::zeroed();
    let mut cb_data = osnoise_cb_data { params: &mut params, trace_output: ptr::null() };
    let opt = TEST_CALLBACK(&mut cb_data as *mut _ as *mut c_void, opt_osnoise_auto_cb);

    ck_assert_int_eq!(opt_osnoise_auto_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.common.stop_us, 10);
    ck_assert_int_eq!(params.threshold, 1);
    ck_assert_str_eq!(cb_data.trace_output, b"osnoise_trace.txt");
}

unsafe extern "C" fn test_opt_osnoise_auto_cb_unset(_: c_int) {
    let mut params: osnoise_params = core::mem::zeroed();
    let mut cb_data = osnoise_cb_data { params: &mut params, trace_output: ptr::null() };
    let opt = TEST_CALLBACK(&mut cb_data as *mut _ as *mut c_void, opt_osnoise_auto_cb);

    ck_assert_int_eq!(opt_osnoise_auto_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_osnoise_auto_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.common.stop_us, 0);
    ck_assert_int_eq!(params.threshold, 0);
    ck_assert_ptr_null!(cb_data.trace_output);
}

unsafe extern "C" fn test_opt_osnoise_trace_output_cb(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_osnoise_trace_output_cb);

    ck_assert_int_eq!(opt_osnoise_trace_output_cb(&opt, c"trace.txt".as_ptr(), 0), 0);
    ck_assert_str_eq!(trace_output, b"trace.txt");
}

unsafe extern "C" fn test_opt_osnoise_trace_output_cb_noarg(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_osnoise_trace_output_cb);

    ck_assert_int_eq!(opt_osnoise_trace_output_cb(&opt, ptr::null(), 0), 0);
    ck_assert_str_eq!(trace_output, b"osnoise_trace.txt");
}

unsafe extern "C" fn test_opt_osnoise_trace_output_cb_unset(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_osnoise_trace_output_cb);

    ck_assert_int_eq!(opt_osnoise_trace_output_cb(&opt, c"trace.txt".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_osnoise_trace_output_cb(&opt, ptr::null(), 1), 0);
    ck_assert_ptr_null!(trace_output);
}

unsafe extern "C" fn test_opt_osnoise_on_threshold_cb(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_osnoise_on_threshold_cb);

    ck_assert_int_eq!(opt_osnoise_on_threshold_cb(&opt, c"trace".as_ptr(), 0), 0);
    ck_assert_int_eq!(actions.len, 1);
    ck_assert_int_eq!(actions.list[0].type_, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq!(actions.list[0].trace_output, b"osnoise_trace.txt");
}

unsafe extern "C" fn test_opt_osnoise_on_threshold_cb_invalid(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_osnoise_on_threshold_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_osnoise_on_threshold_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_osnoise_on_end_cb(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_osnoise_on_end_cb);

    ck_assert_int_eq!(opt_osnoise_on_end_cb(&opt, c"trace".as_ptr(), 0), 0);
    ck_assert_int_eq!(actions.len, 1);
    ck_assert_int_eq!(actions.list[0].type_, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq!(actions.list[0].trace_output, b"osnoise_trace.txt");
}

unsafe extern "C" fn test_opt_osnoise_on_end_cb_invalid(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_osnoise_on_end_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_osnoise_on_end_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_timerlat_auto_cb(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let mut cb_data = timerlat_cb_data { params: &mut params, trace_output: ptr::null() };
    let opt = TEST_CALLBACK(&mut cb_data as *mut _ as *mut c_void, opt_timerlat_auto_cb);

    ck_assert_int_eq!(opt_timerlat_auto_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.common.stop_us, 10);
    ck_assert_int_eq!(params.common.stop_total_us, 10);
    ck_assert_int_eq!(params.print_stack, 10);
    ck_assert_str_eq!(cb_data.trace_output, b"timerlat_trace.txt");
}

unsafe extern "C" fn test_opt_timerlat_auto_cb_unset(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let mut cb_data = timerlat_cb_data { params: &mut params, trace_output: ptr::null() };
    let opt = TEST_CALLBACK(&mut cb_data as *mut _ as *mut c_void, opt_timerlat_auto_cb);

    ck_assert_int_eq!(opt_timerlat_auto_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_timerlat_auto_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.common.stop_us, 0);
    ck_assert_int_eq!(params.common.stop_total_us, 0);
    ck_assert_int_eq!(params.print_stack, 0);
    ck_assert_ptr_null!(cb_data.trace_output);
}

unsafe extern "C" fn test_opt_aa_only_cb(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_aa_only_cb);

    ck_assert_int_eq!(opt_aa_only_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(params.common.stop_us, 10);
    ck_assert_int_eq!(params.common.stop_total_us, 10);
    ck_assert_int_eq!(params.print_stack, 10);
    ck_assert_int_eq!(params.common.aa_only, 1);
}

unsafe extern "C" fn test_opt_aa_only_cb_unset(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_aa_only_cb);

    ck_assert_int_eq!(opt_aa_only_cb(&opt, c"10".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_aa_only_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.common.stop_us, 0);
    ck_assert_int_eq!(params.common.stop_total_us, 0);
    ck_assert_int_eq!(params.print_stack, 0);
    ck_assert_int_eq!(params.common.aa_only, 0);
}

unsafe extern "C" fn test_opt_timerlat_trace_output_cb(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_timerlat_trace_output_cb);

    ck_assert_int_eq!(opt_timerlat_trace_output_cb(&opt, c"trace.txt".as_ptr(), 0), 0);
    ck_assert_str_eq!(trace_output, b"trace.txt");
}

unsafe extern "C" fn test_opt_timerlat_trace_output_cb_noarg(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_timerlat_trace_output_cb);

    ck_assert_int_eq!(opt_timerlat_trace_output_cb(&opt, ptr::null(), 0), 0);
    ck_assert_str_eq!(trace_output, b"timerlat_trace.txt");
}

unsafe extern "C" fn test_opt_timerlat_trace_output_cb_unset(_: c_int) {
    let mut trace_output: *const c_char = ptr::null();
    let opt = TEST_CALLBACK(&mut trace_output as *mut _ as *mut c_void, opt_timerlat_trace_output_cb);

    ck_assert_int_eq!(opt_timerlat_trace_output_cb(&opt, c"trace.txt".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_timerlat_trace_output_cb(&opt, ptr::null(), 1), 0);
    ck_assert_ptr_null!(trace_output);
}

unsafe extern "C" fn test_opt_timerlat_on_threshold_cb(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_timerlat_on_threshold_cb);

    ck_assert_int_eq!(opt_timerlat_on_threshold_cb(&opt, c"trace".as_ptr(), 0), 0);
    ck_assert_int_eq!(actions.len, 1);
    ck_assert_int_eq!(actions.list[0].type_, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq!(actions.list[0].trace_output, b"timerlat_trace.txt");
}

unsafe extern "C" fn test_opt_timerlat_on_threshold_cb_invalid(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_timerlat_on_threshold_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_timerlat_on_threshold_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_timerlat_on_end_cb(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_timerlat_on_end_cb);

    ck_assert_int_eq!(opt_timerlat_on_end_cb(&opt, c"trace".as_ptr(), 0), 0);
    ck_assert_int_eq!(actions.len, 1);
    ck_assert_int_eq!(actions.list[0].type_, ACTION_TRACE_OUTPUT);
    ck_assert_str_eq!(actions.list[0].trace_output, b"timerlat_trace.txt");
}

unsafe extern "C" fn test_opt_timerlat_on_end_cb_invalid(_: c_int) {
    let mut actions: actions = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut actions as *mut _ as *mut c_void, opt_timerlat_on_end_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_timerlat_on_end_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_user_threads_cb(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_user_threads_cb);

    ck_assert_int_eq!(opt_user_threads_cb(&opt, ptr::null(), 0), 0);
    ck_assert_int_eq!(params.common.user_workload, 1);
    ck_assert_int_eq!(params.common.user_data, 1);
}

unsafe extern "C" fn test_opt_user_threads_cb_unset(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_user_threads_cb);

    ck_assert_int_eq!(opt_user_threads_cb(&opt, ptr::null(), 0), 0);
    ck_assert_int_eq!(opt_user_threads_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.common.user_workload, 0);
    ck_assert_int_eq!(params.common.user_data, 0);
}

unsafe extern "C" fn test_opt_nano_cb(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_nano_cb);

    ck_assert_int_eq!(opt_nano_cb(&opt, ptr::null(), 0), 0);
    ck_assert_int_eq!(params.common.output_divisor, 1);
}

unsafe extern "C" fn test_opt_nano_cb_unset(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let opt = TEST_CALLBACK(&mut params as *mut _ as *mut c_void, opt_nano_cb);

    ck_assert_int_eq!(opt_nano_cb(&opt, ptr::null(), 0), 0);
    ck_assert_int_eq!(opt_nano_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.common.output_divisor, default_output_divisor);
}

unsafe extern "C" fn test_opt_timerlat_align_cb(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let range = Box::leak(Box::new(LLONG_RANGE(0, c_longlong::MAX)));
    let opt = RTLA_OPT_CALLBACK_DATA(
        b'A' as c_int,
        c"aligned".as_ptr(),
        &mut params as *mut _ as *mut c_void,
        c"us".as_ptr(),
        c"test".as_ptr(),
        opt_timerlat_align_cb,
        range as *const llong_range as *const c_void,
    );

    ck_assert_int_eq!(opt_timerlat_align_cb(&opt, c"500".as_ptr(), 0), 0);
    assert!(params.timerlat_align != 0);
    ck_assert_int_eq!(params.timerlat_align_us, 500);
}

unsafe extern "C" fn test_opt_timerlat_align_cb_invalid(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let range = Box::leak(Box::new(LLONG_RANGE(0, c_longlong::MAX)));
    let opt = RTLA_OPT_CALLBACK_DATA(
        b'A' as c_int,
        c"aligned".as_ptr(),
        &mut params as *mut _ as *mut c_void,
        c"us".as_ptr(),
        c"test".as_ptr(),
        opt_timerlat_align_cb,
        range as *const llong_range as *const c_void,
    );

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_timerlat_align_cb(&opt, c"-1".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_timerlat_align_cb_unset(_: c_int) {
    let mut params: timerlat_params = core::mem::zeroed();
    let range = Box::leak(Box::new(LLONG_RANGE(0, c_longlong::MAX)));
    let opt = RTLA_OPT_CALLBACK_DATA(
        b'A' as c_int,
        c"aligned".as_ptr(),
        &mut params as *mut _ as *mut c_void,
        c"us".as_ptr(),
        c"test".as_ptr(),
        opt_timerlat_align_cb,
        range as *const llong_range as *const c_void,
    );

    ck_assert_int_eq!(opt_timerlat_align_cb(&opt, c"500".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_timerlat_align_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(params.timerlat_align, 0);
    ck_assert_int_eq!(params.timerlat_align_us, 0);
}

unsafe extern "C" fn test_opt_stack_format_cb(_: c_int) {
    let mut stack_format: c_int = 0;
    let opt = TEST_CALLBACK(&mut stack_format as *mut _ as *mut c_void, opt_stack_format_cb);

    ck_assert_int_eq!(opt_stack_format_cb(&opt, c"full".as_ptr(), 0), 0);
    ck_assert_int_eq!(stack_format, STACK_FORMAT_FULL);
}

unsafe extern "C" fn test_opt_stack_format_cb_invalid(_: c_int) {
    let mut stack_format: c_int = 0;
    let opt = TEST_CALLBACK(&mut stack_format as *mut _ as *mut c_void, opt_stack_format_cb);

    redirect_stderr_to_null!();
    ck_assert_int_eq!(opt_stack_format_cb(&opt, c"abc".as_ptr(), 0), -1);
}

unsafe extern "C" fn test_opt_stack_format_cb_unset(_: c_int) {
    let mut stack_format: c_int = 0;
    let opt = TEST_CALLBACK(&mut stack_format as *mut _ as *mut c_void, opt_stack_format_cb);

    ck_assert_int_eq!(opt_stack_format_cb(&opt, c"full".as_ptr(), 0), 0);
    ck_assert_int_eq!(opt_stack_format_cb(&opt, ptr::null(), 1), 0);
    ck_assert_int_eq!(stack_format, default_stack_format);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cli_opt_callback_suite() -> *mut Suite {
    let s = suite_create(c"cli_opt_callback".as_ptr());
    let mut tc: *mut TCase;

    tc = tcase_create(c"common".as_ptr());
    tcase_add_test(tc, test_opt_llong_callback_simple);
    tcase_add_test(tc, test_opt_llong_callback_max);
    tcase_add_test(tc, test_opt_llong_callback_min);
    tcase_add_test(tc, test_opt_llong_callback_non_numeric);
    tcase_add_test(tc, test_opt_llong_callback_non_numeric_suffix);
    tcase_add_test(tc, test_opt_llong_callback_unset);
    tcase_add_test(tc, test_opt_llong_callback_unset_defval);
    tcase_add_test(tc, test_opt_llong_callback_range_in);
    tcase_add_test(tc, test_opt_llong_callback_range_below);
    tcase_add_test(tc, test_opt_llong_callback_range_above);
    tcase_add_test(tc, test_opt_llong_callback_range_boundary);
    tcase_add_test(tc, test_opt_int_callback_simple);
    tcase_add_test(tc, test_opt_int_callback_max);
    tcase_add_test(tc, test_opt_int_callback_min);
    tcase_add_test(tc, test_opt_int_callback_non_numeric);
    tcase_add_test(tc, test_opt_int_callback_non_numeric_suffix);
    tcase_add_test(tc, test_opt_int_callback_unset);
    tcase_add_test(tc, test_opt_int_callback_unset_defval);
    tcase_add_test(tc, test_opt_int_callback_range_in);
    tcase_add_test(tc, test_opt_int_callback_range_below);
    tcase_add_test(tc, test_opt_int_callback_range_above);
    tcase_add_test(tc, test_opt_int_callback_range_boundary);
    tcase_add_test(tc, test_opt_cpus_cb);
    tcase_add_test(tc, test_opt_cpus_cb_invalid);
    tcase_add_test(tc, test_opt_cgroup_cb);
    tcase_add_test(tc, test_opt_cgroup_cb_equals);
    tcase_add_test(tc, test_opt_cgroup_cb_unset);
    tcase_add_test(tc, test_opt_duration_cb);
    tcase_add_test(tc, test_opt_duration_cb_unset);
    tcase_add_test(tc, test_opt_duration_cb_invalid);
    tcase_add_test(tc, test_opt_event_cb);
    tcase_add_test(tc, test_opt_event_cb_multiple);
    tcase_add_test(tc, test_opt_housekeeping_cb);
    tcase_add_test(tc, test_opt_housekeeping_cb_invalid);
    tcase_add_test(tc, test_opt_housekeeping_cb_unset);
    tcase_add_test(tc, test_opt_priority_cb);
    tcase_add_test(tc, test_opt_priority_cb_invalid);
    tcase_add_test(tc, test_opt_priority_cb_unset);
    tcase_add_test(tc, test_opt_trigger_cb);
    tcase_add_test(tc, test_opt_trigger_cb_no_event);
    tcase_add_test(tc, test_opt_filter_cb);
    tcase_add_test(tc, test_opt_filter_cb_no_event);
    suite_add_tcase(s, tc);

    tc = tcase_create(c"osnoise".as_ptr());
    tcase_add_test(tc, test_opt_osnoise_auto_cb);
    tcase_add_test(tc, test_opt_osnoise_auto_cb_unset);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb_noarg);
    tcase_add_test(tc, test_opt_osnoise_trace_output_cb_unset);
    tcase_add_test(tc, test_opt_osnoise_on_threshold_cb);
    tcase_add_test(tc, test_opt_osnoise_on_threshold_cb_invalid);
    tcase_add_test(tc, test_opt_osnoise_on_end_cb);
    tcase_add_test(tc, test_opt_osnoise_on_end_cb_invalid);
    suite_add_tcase(s, tc);

    tc = tcase_create(c"timerlat".as_ptr());
    tcase_add_test(tc, test_opt_timerlat_auto_cb);
    tcase_add_test(tc, test_opt_timerlat_auto_cb_unset);
    tcase_add_test(tc, test_opt_aa_only_cb);
    tcase_add_test(tc, test_opt_aa_only_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb_noarg);
    tcase_add_test(tc, test_opt_timerlat_trace_output_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_on_threshold_cb);
    tcase_add_test(tc, test_opt_timerlat_on_threshold_cb_invalid);
    tcase_add_test(tc, test_opt_timerlat_on_end_cb);
    tcase_add_test(tc, test_opt_timerlat_on_end_cb_invalid);
    tcase_add_test(tc, test_opt_user_threads_cb);
    tcase_add_test(tc, test_opt_user_threads_cb_unset);
    tcase_add_test(tc, test_opt_nano_cb);
    tcase_add_test(tc, test_opt_nano_cb_unset);
    tcase_add_test(tc, test_opt_stack_format_cb);
    tcase_add_test(tc, test_opt_stack_format_cb_invalid);
    tcase_add_test(tc, test_opt_stack_format_cb_unset);
    tcase_add_test(tc, test_opt_timerlat_align_cb);
    tcase_add_test(tc, test_opt_timerlat_align_cb_invalid);
    tcase_add_test(tc, test_opt_timerlat_align_cb_unset);
    suite_add_tcase(s, tc);

    s
}
