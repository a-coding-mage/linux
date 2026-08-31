// SPDX-License-Identifier: GPL-2.0

// C dependencies: check.h, stdio.h, stdlib.h, sched.h, limits.h, unistd.h,
// sys/sysinfo.h, cli_params_assert.h, ../../src/cli.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use std::ffi::c_char;
use std::os::raw::{c_int, c_uint, c_ulong};
use std::ptr;

const SCHED_FIFO: c_int = 1;

#[repr(C)]
pub struct Suite {
	_private: [u8; 0],
}

#[repr(C)]
pub struct TCase {
	_private: [u8; 0],
}

#[repr(C)]
pub struct sched_param {
	pub sched_priority: c_int,
	pub sched_policy: c_int,
}

#[repr(C)]
pub struct hist_params {
	pub bucket_size: c_int,
	pub entries: c_int,
	pub no_header: bool,
	pub no_index: bool,
	pub no_summary: bool,
	pub with_zeros: bool,
}

#[repr(C)]
pub struct action_data_str {
	pub str_: *mut c_char,
}

#[repr(C)]
pub union action_data {
	pub trace_output: action_data_str,
}

#[repr(C)]
pub struct trace_action {
	pub action: c_int,
	pub data: action_data,
	pub next: *mut trace_action,
}

#[repr(C)]
pub struct trace_events {
	pub system: *mut c_char,
	pub event: *mut c_char,
	pub filter: *mut c_char,
	pub trigger: *mut c_char,
	pub next: *mut trace_events,
}

#[repr(C)]
pub struct cpumask {
	pub bits: *mut c_ulong,
}

#[repr(C)]
pub struct common_params {
	pub stop_us: c_int,
	pub stop_total_us: c_int,
	pub duration: c_int,
	pub cpus: *mut c_char,
	pub monitored_cpus: *mut cpumask,
	pub hk_cpu_set: *mut cpumask,
	pub cgroup: bool,
	pub cgroup_name: *mut c_char,
	pub sched_param: sched_param,
	pub hist: hist_params,
	pub buffer_size: c_int,
	pub warmup: c_int,
	pub threshold_actions: *mut trace_action,
	pub end_actions: *mut trace_action,
	pub events: *mut trace_events,
}

#[repr(C)]
pub struct osnoise_params {
	pub period: c_int,
	pub runtime: c_int,
	pub threshold: c_int,
	pub common: common_params,
}

const ACTION_TRACE_OUTPUT: c_int = 0;

unsafe extern "C" {
	static mut nr_cpus: c_int;
	static mut config_debug: bool;

	fn osnoise_hist_parse_args(argc: c_int, argv: *mut *mut c_char) -> *mut common_params;
	fn to_osnoise_params(params: *mut common_params) -> *mut osnoise_params;

	fn suite_create(name: *const c_char) -> *mut Suite;
	fn tcase_create(name: *const c_char) -> *mut TCase;
	fn tcase_add_test(tc: *mut TCase, test: Option<unsafe extern "C" fn(c_int)>);
	fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);

	fn ck_assert(expr: bool);
	fn ck_assert_int_eq(a: c_int, b: c_int);
	fn ck_assert_str_eq(a: *const c_char, b: *const c_char);
	fn ck_assert_ptr_null(ptr: *const std::ffi::c_void);

	fn CLI_ASSERT_SINGLE_ACTION(
		actions: *mut trace_action,
		action: c_int,
		field: *const c_char,
		member: *const c_char,
		value: *const c_char,
	);
	fn CLI_ASSERT_SINGLE_EVENT(system: *const c_char, event: *const c_char);
	fn CLI_ASSERT_SINGLE_FILTER(filter: *const c_char);
	fn CLI_ASSERT_SINGLE_TRIGGER(trigger: *const c_char);
	fn CLI_ASSERT_CPUSET(set: *mut cpumask, cpu0: c_uint, cpu1: c_uint, cpu2: c_uint);
	fn CLI_OSNOISE_ASSERT_AUTO(duration: c_int);
}

macro_rules! cstr {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *mut c_char
	};
}

macro_rules! PARSE_ARGS {
	($($arg:literal),+ $(,)?) => {
		let mut argv: [*mut c_char; [$(stringify!($arg)),+].len() + 1] =
			[$(cstr!($arg)),+, ptr::null_mut()];
		let argc: c_int = (argv.len() - 1) as c_int;
		let params: *mut common_params = osnoise_hist_parse_args(argc, argv.as_mut_ptr());
		let osn_params: *mut osnoise_params = to_osnoise_params(params);
	};
}

/* Tracing Options */

unsafe extern "C" fn test_period_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-p", "100000");

	ck_assert_int_eq((*osn_params).period, 100000);
}

unsafe extern "C" fn test_period_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--period", "100000");

	ck_assert_int_eq((*osn_params).period, 100000);
}

unsafe extern "C" fn test_period_unset_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-p", "100000", "--no-period");

	ck_assert_int_eq((*osn_params).period, 0);
}

unsafe extern "C" fn test_period_unset_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--period", "100000", "--no-period");

	ck_assert_int_eq((*osn_params).period, 0);
}

unsafe extern "C" fn test_runtime_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-r", "95000");

	ck_assert_int_eq((*osn_params).runtime, 95000);
}

unsafe extern "C" fn test_runtime_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--runtime", "95000");

	ck_assert_int_eq((*osn_params).runtime, 95000);
}

unsafe extern "C" fn test_stop_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-s", "20");

	ck_assert_int_eq((*params).stop_us, 20);
}

unsafe extern "C" fn test_stop_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--stop", "20");

	ck_assert_int_eq((*params).stop_us, 20);
}

unsafe extern "C" fn test_stop_total_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-S", "20");

	ck_assert_int_eq((*params).stop_total_us, 20);
}

unsafe extern "C" fn test_stop_total_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--stop-total", "20");

	ck_assert_int_eq((*params).stop_total_us, 20);
}

unsafe extern "C" fn test_threshold_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-T", "5");

	ck_assert_int_eq((*osn_params).threshold, 5);
}

unsafe extern "C" fn test_threshold_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--threshold", "5");

	ck_assert_int_eq((*osn_params).threshold, 5);
}

unsafe extern "C" fn test_trace_short_noarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-t");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
}

unsafe extern "C" fn test_trace_short_followarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-t", "-d", "20");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
	ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
}

unsafe extern "C" fn test_trace_short_space(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-t", "tracefile");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("tracefile"));
}

unsafe extern "C" fn test_trace_short_equals(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-t=tracefile");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("tracefile"));
}

unsafe extern "C" fn test_trace_long_noarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--trace");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
}

unsafe extern "C" fn test_trace_long_followarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--trace", "-d", "20");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
	ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
}

unsafe extern "C" fn test_trace_long_space(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--trace", "tracefile");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("tracefile"));
}

unsafe extern "C" fn test_trace_long_equals(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--trace=tracefile");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("tracefile"));
}

/* Event Configuration */

unsafe extern "C" fn test_event_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-e", "system:event");

	CLI_ASSERT_SINGLE_EVENT(cstr!("system"), cstr!("event"));
}

unsafe extern "C" fn test_event_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--event", "system:event");

	CLI_ASSERT_SINGLE_EVENT(cstr!("system"), cstr!("event"));
}

unsafe extern "C" fn test_filter(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-e", "system:event", "--filter", "filter");

	CLI_ASSERT_SINGLE_FILTER(cstr!("filter"));
}

unsafe extern "C" fn test_trigger(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-e", "system:event", "--trigger", "trigger");

	CLI_ASSERT_SINGLE_TRIGGER(cstr!("trigger"));
}

/* CPU Configuration */

unsafe extern "C" fn test_cpus_short(_i: c_int) {
	nr_cpus = 4;

	PARSE_ARGS!("osnoise", "hist", "-c", "0-1,3");

	ck_assert_str_eq((*params).cpus, cstr!("0-1,3"));
	CLI_ASSERT_CPUSET((*params).monitored_cpus, 0, 1, 3);
}

unsafe extern "C" fn test_cpus_long(_i: c_int) {
	nr_cpus = 4;

	PARSE_ARGS!("osnoise", "hist", "--cpus", "0-1,3");

	ck_assert_str_eq((*params).cpus, cstr!("0-1,3"));
	CLI_ASSERT_CPUSET((*params).monitored_cpus, 0, 1, 3);
}

unsafe extern "C" fn test_housekeeping_short(_i: c_int) {
	nr_cpus = 4;

	PARSE_ARGS!("osnoise", "hist", "-H", "0-1,3");

	CLI_ASSERT_CPUSET((*params).hk_cpu_set, 0, 1, 3);
}

unsafe extern "C" fn test_housekeeping_long(_i: c_int) {
	nr_cpus = 4;

	PARSE_ARGS!("osnoise", "hist", "--house-keeping", "0-1,3");

	CLI_ASSERT_CPUSET((*params).hk_cpu_set, 0, 1, 3);
}

/* Thread Configuration */

unsafe extern "C" fn test_cgroup_short_noarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-C");

	ck_assert((*params).cgroup);
	ck_assert_ptr_null((*params).cgroup_name as *const std::ffi::c_void);
}

unsafe extern "C" fn test_cgroup_short_space(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-C", "cgroup");

	ck_assert((*params).cgroup);
	ck_assert_str_eq((*params).cgroup_name, cstr!("cgroup"));
}

unsafe extern "C" fn test_cgroup_short_equals(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-C=cgroup");

	ck_assert((*params).cgroup);
	ck_assert_str_eq((*params).cgroup_name, cstr!("cgroup"));
}

unsafe extern "C" fn test_cgroup_long_noarg(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--cgroup");

	ck_assert((*params).cgroup);
	ck_assert_ptr_null((*params).cgroup_name as *const std::ffi::c_void);
}

unsafe extern "C" fn test_cgroup_long_space(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--cgroup", "cgroup");

	ck_assert((*params).cgroup);
	ck_assert_str_eq((*params).cgroup_name, cstr!("cgroup"));
}

unsafe extern "C" fn test_cgroup_long_equals(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--cgroup=cgroup");

	ck_assert((*params).cgroup);
	ck_assert_str_eq((*params).cgroup_name, cstr!("cgroup"));
}

unsafe extern "C" fn test_priority_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-P", "f:95");

	ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
	ck_assert_int_eq((*params).sched_param.sched_priority, 95);
}

unsafe extern "C" fn test_priority_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--priority", "f:95");

	ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
	ck_assert_int_eq((*params).sched_param.sched_priority, 95);
}

/* Histogram Options */

unsafe extern "C" fn test_bucket_size_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-b", "2");

	ck_assert_int_eq((*params).hist.bucket_size, 2);
}

unsafe extern "C" fn test_bucket_size_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--bucket-size", "2");

	ck_assert_int_eq((*params).hist.bucket_size, 2);
}

unsafe extern "C" fn test_entries_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-E", "512");

	ck_assert_int_eq((*params).hist.entries, 512);
}

unsafe extern "C" fn test_entries_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--entries", "512");

	ck_assert_int_eq((*params).hist.entries, 512);
}

unsafe extern "C" fn test_no_header(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--no-header");

	ck_assert((*params).hist.no_header);
}

unsafe extern "C" fn test_no_index(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--with-zeros", "--no-index");

	ck_assert((*params).hist.no_index);
}

unsafe extern "C" fn test_no_summary(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--no-summary");

	ck_assert((*params).hist.no_summary);
}

unsafe extern "C" fn test_with_zeros(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--with-zeros");

	ck_assert((*params).hist.with_zeros);
}

/* System Tuning */

unsafe extern "C" fn test_trace_buffer_size(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--trace-buffer-size", "200");

	ck_assert_int_eq((*params).buffer_size, 200);
}

unsafe extern "C" fn test_warm_up(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--warm-up", "5");

	ck_assert_int_eq((*params).warmup, 5);
}

/* Auto Analysis and Actions */

unsafe extern "C" fn test_auto(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-a", "20");

	CLI_OSNOISE_ASSERT_AUTO(20);
}

unsafe extern "C" fn test_on_end(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--on-end", "trace");

	CLI_ASSERT_SINGLE_ACTION((*params).end_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
}

unsafe extern "C" fn test_on_threshold(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--on-threshold", "trace");

	CLI_ASSERT_SINGLE_ACTION((*params).threshold_actions, ACTION_TRACE_OUTPUT, cstr!("trace_output"), cstr!("str"),
				 cstr!("osnoise_trace.txt"));
}

/* General */

unsafe extern "C" fn test_debug_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-D");

	ck_assert(config_debug);
}

unsafe extern "C" fn test_debug_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--debug");

	ck_assert(config_debug);
}

unsafe extern "C" fn test_duration_short(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "-d", "1m");

	ck_assert_int_eq((*params).duration, 60);
}

unsafe extern "C" fn test_duration_long(_i: c_int) {
	PARSE_ARGS!("osnoise", "hist", "--duration", "1m");

	ck_assert_int_eq((*params).duration, 60);
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_hist_cli_suite() -> *mut Suite {
	let s: *mut Suite = suite_create(cstr!("osnoise_hist_cli"));
	let mut tc: *mut TCase;

	tc = tcase_create(cstr!("tracing_options"));
	tcase_add_test(tc, Some(test_period_short));
	tcase_add_test(tc, Some(test_period_long));
	tcase_add_test(tc, Some(test_period_unset_short));
	tcase_add_test(tc, Some(test_period_unset_long));
	tcase_add_test(tc, Some(test_runtime_short));
	tcase_add_test(tc, Some(test_runtime_long));
	tcase_add_test(tc, Some(test_stop_short));
	tcase_add_test(tc, Some(test_stop_long));
	tcase_add_test(tc, Some(test_stop_total_short));
	tcase_add_test(tc, Some(test_stop_total_long));
	tcase_add_test(tc, Some(test_threshold_short));
	tcase_add_test(tc, Some(test_threshold_long));
	tcase_add_test(tc, Some(test_trace_short_noarg));
	tcase_add_test(tc, Some(test_trace_short_followarg));
	tcase_add_test(tc, Some(test_trace_short_space));
	tcase_add_test(tc, Some(test_trace_short_equals));
	tcase_add_test(tc, Some(test_trace_long_noarg));
	tcase_add_test(tc, Some(test_trace_long_followarg));
	tcase_add_test(tc, Some(test_trace_long_space));
	tcase_add_test(tc, Some(test_trace_long_equals));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("event_configuration"));
	tcase_add_test(tc, Some(test_event_short));
	tcase_add_test(tc, Some(test_event_long));
	tcase_add_test(tc, Some(test_filter));
	tcase_add_test(tc, Some(test_trigger));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("cpu_configuration"));
	tcase_add_test(tc, Some(test_cpus_short));
	tcase_add_test(tc, Some(test_cpus_long));
	tcase_add_test(tc, Some(test_housekeeping_short));
	tcase_add_test(tc, Some(test_housekeeping_long));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("thread_configuration"));
	tcase_add_test(tc, Some(test_cgroup_short_noarg));
	tcase_add_test(tc, Some(test_cgroup_short_space));
	tcase_add_test(tc, Some(test_cgroup_short_equals));
	tcase_add_test(tc, Some(test_cgroup_long_noarg));
	tcase_add_test(tc, Some(test_cgroup_long_space));
	tcase_add_test(tc, Some(test_cgroup_long_equals));
	tcase_add_test(tc, Some(test_priority_short));
	tcase_add_test(tc, Some(test_priority_long));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("histogram_options"));
	tcase_add_test(tc, Some(test_bucket_size_short));
	tcase_add_test(tc, Some(test_bucket_size_long));
	tcase_add_test(tc, Some(test_entries_short));
	tcase_add_test(tc, Some(test_entries_long));
	tcase_add_test(tc, Some(test_no_header));
	tcase_add_test(tc, Some(test_no_index));
	tcase_add_test(tc, Some(test_no_summary));
	tcase_add_test(tc, Some(test_with_zeros));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("system_tuning"));
	tcase_add_test(tc, Some(test_trace_buffer_size));
	tcase_add_test(tc, Some(test_warm_up));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("aa_actions"));
	tcase_add_test(tc, Some(test_auto));
	tcase_add_test(tc, Some(test_on_end));
	tcase_add_test(tc, Some(test_on_threshold));
	suite_add_tcase(s, tc);

	tc = tcase_create(cstr!("general"));
	tcase_add_test(tc, Some(test_debug_short));
	tcase_add_test(tc, Some(test_debug_long));
	tcase_add_test(tc, Some(test_duration_short));
	tcase_add_test(tc, Some(test_duration_long));
	suite_add_tcase(s, tc);

	return s;
}
