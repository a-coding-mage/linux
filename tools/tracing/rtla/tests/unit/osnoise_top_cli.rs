// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// check.h, stdio.h, stdlib.h, sched.h, limits.h, unistd.h, sys/sysinfo.h,
// "cli_params_assert.h", and "../../src/cli.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use std::ffi::CString;

const SCHED_FIFO: c_int = 1;
const ACTION_TRACE_OUTPUT: c_int = 0;

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TCase {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_param_cli {
    pub sched_policy: c_int,
    pub sched_priority: c_int,
}

#[repr(C)]
pub union action_data {
    pub str_: *mut c_char,
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct trace_action {
    pub action: c_int,
    pub trace_output: action_data,
    pub next: *mut trace_action,
}

#[repr(C)]
pub struct common_params {
    pub stop_us: c_int,
    pub stop_total_us: c_int,
    pub duration: c_int,
    pub cpus: *mut c_char,
    pub monitored_cpus: *mut c_void,
    pub hk_cpu_set: *mut c_void,
    pub cgroup: bool,
    pub cgroup_name: *mut c_char,
    pub sched_param: sched_param_cli,
    pub quiet: bool,
    pub end_actions: *mut trace_action,
    pub threshold_actions: *mut trace_action,
    pub buffer_size: c_int,
    pub warmup: c_int,
}

#[repr(C)]
pub struct osnoise_params {
    pub period: c_int,
    pub runtime: c_int,
    pub threshold: c_int,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static mut config_debug: bool;

    fn osnoise_top_parse_args(argc: c_int, argv: *mut *mut c_char) -> *mut common_params;
    fn to_osnoise_params(params: *mut common_params) -> *mut osnoise_params;

    fn suite_create(name: *const c_char) -> *mut Suite;
    fn tcase_create(name: *const c_char) -> *mut TCase;
    fn tcase_add_test(tc: *mut TCase, test: unsafe extern "C" fn(c_int));
    fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);

    fn ck_assert(expr: bool);
    fn ck_assert_int_eq(left: c_int, right: c_int);
    fn ck_assert_str_eq(left: *const c_char, right: *const c_char);
    fn ck_assert_ptr_null(ptr: *const c_void);

    fn cli_assert_single_event(system: *const c_char, event: *const c_char);
    fn cli_assert_single_filter(filter: *const c_char);
    fn cli_assert_single_trigger(trigger: *const c_char);
    fn cli_assert_cpuset(set: *mut c_void, cpu0: c_int, cpu1: c_int, cpu2: c_int);
    fn cli_osnoise_assert_auto(usec: c_int);
}

unsafe fn parse_args(args: &[&str]) -> (*mut common_params, *mut osnoise_params) {
    let cstrings: Vec<CString> = args.iter().map(|arg| CString::new(*arg).unwrap()).collect();
    let mut argv: Vec<*mut c_char> = cstrings.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
    argv.push(core::ptr::null_mut());
    let argc = (argv.len() - 1) as c_int;
    let params = unsafe { osnoise_top_parse_args(argc, argv.as_mut_ptr()) };
    let osn_params = unsafe { to_osnoise_params(params) };

    (params, osn_params)
}

unsafe fn cli_assert_single_action(
    actions: *mut trace_action,
    action: c_int,
    trace_output: *const c_char,
) {
    unsafe {
        ck_assert(!actions.is_null());
        ck_assert_int_eq((*actions).action, action);
        ck_assert_str_eq((*actions).trace_output.str_ as *const c_char, trace_output);
        ck_assert_ptr_null((*actions).next as *const c_void);
    }
}

/* Tracing Options */

unsafe extern "C" fn test_period_short(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "-p", "100000"]);

        ck_assert_int_eq((*osn_params).period, 100000);
    }
}

unsafe extern "C" fn test_period_long(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "--period", "100000"]);

        ck_assert_int_eq((*osn_params).period, 100000);
    }
}

unsafe extern "C" fn test_period_unset_short(_i: c_int) {
    unsafe {
        let (_params, osn_params) =
            parse_args(&["osnoise", "top", "-p", "100000", "--no-period"]);

        ck_assert_int_eq((*osn_params).period, 0);
    }
}

unsafe extern "C" fn test_period_unset_long(_i: c_int) {
    unsafe {
        let (_params, osn_params) =
            parse_args(&["osnoise", "top", "--period", "100000", "--no-period"]);

        ck_assert_int_eq((*osn_params).period, 0);
    }
}

unsafe extern "C" fn test_runtime_short(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "-r", "95000"]);

        ck_assert_int_eq((*osn_params).runtime, 95000);
    }
}

unsafe extern "C" fn test_runtime_long(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "--runtime", "95000"]);

        ck_assert_int_eq((*osn_params).runtime, 95000);
    }
}

unsafe extern "C" fn test_stop_short(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-s", "20"]);

        ck_assert_int_eq((*params).stop_us, 20);
    }
}

unsafe extern "C" fn test_stop_long(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--stop", "20"]);

        ck_assert_int_eq((*params).stop_us, 20);
    }
}

unsafe extern "C" fn test_stop_total_short(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-S", "20"]);

        ck_assert_int_eq((*params).stop_total_us, 20);
    }
}

unsafe extern "C" fn test_stop_total_long(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--stop-total", "20"]);

        ck_assert_int_eq((*params).stop_total_us, 20);
    }
}

unsafe extern "C" fn test_threshold_short(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "-T", "5"]);

        ck_assert_int_eq((*osn_params).threshold, 5);
    }
}

unsafe extern "C" fn test_threshold_long(_i: c_int) {
    unsafe {
        let (_params, osn_params) = parse_args(&["osnoise", "top", "--threshold", "5"]);

        ck_assert_int_eq((*osn_params).threshold, 5);
    }
}

unsafe extern "C" fn test_trace_short_noarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-t"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_trace_short_followarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-t", "-d", "20"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
        ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
    }
}

unsafe extern "C" fn test_trace_short_space(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-t", "tracefile"]);
        let trace = CString::new("tracefile").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_trace_short_equals(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-t=tracefile"]);
        let trace = CString::new("tracefile").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_trace_long_noarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--trace"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_trace_long_followarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--trace", "-d", "20"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
        ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
    }
}

unsafe extern "C" fn test_trace_long_space(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--trace", "tracefile"]);
        let trace = CString::new("tracefile").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_trace_long_equals(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--trace=tracefile"]);
        let trace = CString::new("tracefile").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

/* Event Configuration */

unsafe extern "C" fn test_event_short(_i: c_int) {
    unsafe {
        let (_params, _osn_params) = parse_args(&["osnoise", "top", "-e", "system:event"]);
        let system = CString::new("system").unwrap();
        let event = CString::new("event").unwrap();

        cli_assert_single_event(system.as_ptr(), event.as_ptr());
    }
}

unsafe extern "C" fn test_event_long(_i: c_int) {
    unsafe {
        let (_params, _osn_params) = parse_args(&["osnoise", "top", "--event", "system:event"]);
        let system = CString::new("system").unwrap();
        let event = CString::new("event").unwrap();

        cli_assert_single_event(system.as_ptr(), event.as_ptr());
    }
}

unsafe extern "C" fn test_filter(_i: c_int) {
    unsafe {
        let (_params, _osn_params) =
            parse_args(&["osnoise", "top", "-e", "system:event", "--filter", "filter"]);
        let filter = CString::new("filter").unwrap();

        cli_assert_single_filter(filter.as_ptr());
    }
}

unsafe extern "C" fn test_trigger(_i: c_int) {
    unsafe {
        let (_params, _osn_params) =
            parse_args(&["osnoise", "top", "-e", "system:event", "--trigger", "trigger"]);
        let trigger = CString::new("trigger").unwrap();

        cli_assert_single_trigger(trigger.as_ptr());
    }
}

/* CPU Configuration */

unsafe extern "C" fn test_cpus_short(_i: c_int) {
    unsafe {
        nr_cpus = 4;

        let (params, _osn_params) = parse_args(&["osnoise", "top", "-c", "0-1,3"]);
        let cpus = CString::new("0-1,3").unwrap();

        ck_assert_str_eq((*params).cpus, cpus.as_ptr());
        cli_assert_cpuset((*params).monitored_cpus, 0, 1, 3);
    }
}

unsafe extern "C" fn test_cpus_long(_i: c_int) {
    unsafe {
        nr_cpus = 4;

        let (params, _osn_params) = parse_args(&["osnoise", "top", "--cpus", "0-1,3"]);
        let cpus = CString::new("0-1,3").unwrap();

        ck_assert_str_eq((*params).cpus, cpus.as_ptr());
        cli_assert_cpuset((*params).monitored_cpus, 0, 1, 3);
    }
}

unsafe extern "C" fn test_housekeeping_short(_i: c_int) {
    unsafe {
        nr_cpus = 4;

        let (params, _osn_params) = parse_args(&["osnoise", "top", "-H", "0-1,3"]);

        cli_assert_cpuset((*params).hk_cpu_set, 0, 1, 3);
    }
}

unsafe extern "C" fn test_housekeeping_long(_i: c_int) {
    unsafe {
        nr_cpus = 4;

        let (params, _osn_params) =
            parse_args(&["osnoise", "top", "--house-keeping", "0-1,3"]);

        cli_assert_cpuset((*params).hk_cpu_set, 0, 1, 3);
    }
}

/* Thread Configuration */

unsafe extern "C" fn test_cgroup_short_noarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-C"]);

        ck_assert((*params).cgroup);
        ck_assert_ptr_null((*params).cgroup_name as *const c_void);
    }
}

unsafe extern "C" fn test_cgroup_short_space(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-C", "cgroup"]);
        let cgroup = CString::new("cgroup").unwrap();

        ck_assert((*params).cgroup);
        ck_assert_str_eq((*params).cgroup_name, cgroup.as_ptr());
    }
}

unsafe extern "C" fn test_cgroup_short_equals(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-C=cgroup"]);
        let cgroup = CString::new("cgroup").unwrap();

        ck_assert((*params).cgroup);
        ck_assert_str_eq((*params).cgroup_name, cgroup.as_ptr());
    }
}

unsafe extern "C" fn test_cgroup_long_noarg(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--cgroup"]);

        ck_assert((*params).cgroup);
        ck_assert_ptr_null((*params).cgroup_name as *const c_void);
    }
}

unsafe extern "C" fn test_cgroup_long_space(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--cgroup", "cgroup"]);
        let cgroup = CString::new("cgroup").unwrap();

        ck_assert((*params).cgroup);
        ck_assert_str_eq((*params).cgroup_name, cgroup.as_ptr());
    }
}

unsafe extern "C" fn test_cgroup_long_equals(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--cgroup=cgroup"]);
        let cgroup = CString::new("cgroup").unwrap();

        ck_assert((*params).cgroup);
        ck_assert_str_eq((*params).cgroup_name, cgroup.as_ptr());
    }
}

unsafe extern "C" fn test_priority_short(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-P", "f:95"]);

        ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
        ck_assert_int_eq((*params).sched_param.sched_priority, 95);
    }
}

unsafe extern "C" fn test_priority_long(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--priority", "f:95"]);

        ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
        ck_assert_int_eq((*params).sched_param.sched_priority, 95);
    }
}

/* Output */

unsafe extern "C" fn test_quiet_short(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-q"]);

        ck_assert((*params).quiet);
    }
}

unsafe extern "C" fn test_quiet_long(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--quiet"]);

        ck_assert((*params).quiet);
    }
}

/* Auto Analysis and Actions */

unsafe extern "C" fn test_auto(_i: c_int) {
    unsafe {
        let (_params, _osn_params) = parse_args(&["osnoise", "top", "-a", "20"]);

        cli_osnoise_assert_auto(20);
    }
}

unsafe extern "C" fn test_on_end(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--on-end", "trace"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).end_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

unsafe extern "C" fn test_on_threshold(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--on-threshold", "trace"]);
        let trace = CString::new("osnoise_trace.txt").unwrap();

        cli_assert_single_action((*params).threshold_actions, ACTION_TRACE_OUTPUT, trace.as_ptr());
    }
}

/* System Tuning */

unsafe extern "C" fn test_trace_buffer_size(_i: c_int) {
    unsafe {
        let (params, _osn_params) =
            parse_args(&["osnoise", "top", "--trace-buffer-size", "200"]);

        ck_assert_int_eq((*params).buffer_size, 200);
    }
}

unsafe extern "C" fn test_warm_up(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--warm-up", "5"]);

        ck_assert_int_eq((*params).warmup, 5);
    }
}

/* General */

unsafe extern "C" fn test_debug_short(_i: c_int) {
    unsafe {
        let (_params, _osn_params) = parse_args(&["osnoise", "top", "-D"]);

        ck_assert(config_debug);
    }
}

unsafe extern "C" fn test_debug_long(_i: c_int) {
    unsafe {
        let (_params, _osn_params) = parse_args(&["osnoise", "top", "--debug"]);

        ck_assert(config_debug);
    }
}

unsafe extern "C" fn test_duration_short(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "-d", "1m"]);

        ck_assert_int_eq((*params).duration, 60);
    }
}

unsafe extern "C" fn test_duration_long(_i: c_int) {
    unsafe {
        let (params, _osn_params) = parse_args(&["osnoise", "top", "--duration", "1m"]);

        ck_assert_int_eq((*params).duration, 60);
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_top_cli_suite() -> *mut Suite {
    unsafe {
        let suite_name = CString::new("osnoise_top_cli").unwrap();
        let mut s = suite_create(suite_name.as_ptr());
        let mut tc: *mut TCase;

        let name = CString::new("tracing_options").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_period_short);
        tcase_add_test(tc, test_period_long);
        tcase_add_test(tc, test_period_unset_short);
        tcase_add_test(tc, test_period_unset_long);
        tcase_add_test(tc, test_runtime_short);
        tcase_add_test(tc, test_runtime_long);
        tcase_add_test(tc, test_stop_short);
        tcase_add_test(tc, test_stop_long);
        tcase_add_test(tc, test_stop_total_short);
        tcase_add_test(tc, test_stop_total_long);
        tcase_add_test(tc, test_threshold_short);
        tcase_add_test(tc, test_threshold_long);
        tcase_add_test(tc, test_trace_short_noarg);
        tcase_add_test(tc, test_trace_short_followarg);
        tcase_add_test(tc, test_trace_short_space);
        tcase_add_test(tc, test_trace_short_equals);
        tcase_add_test(tc, test_trace_long_noarg);
        tcase_add_test(tc, test_trace_long_followarg);
        tcase_add_test(tc, test_trace_long_space);
        tcase_add_test(tc, test_trace_long_equals);
        suite_add_tcase(s, tc);

        let name = CString::new("event_configuration").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_event_short);
        tcase_add_test(tc, test_event_long);
        tcase_add_test(tc, test_filter);
        tcase_add_test(tc, test_trigger);
        suite_add_tcase(s, tc);

        let name = CString::new("cpu_configuration").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_cpus_short);
        tcase_add_test(tc, test_cpus_long);
        tcase_add_test(tc, test_housekeeping_short);
        tcase_add_test(tc, test_housekeeping_long);
        suite_add_tcase(s, tc);

        let name = CString::new("thread_configuration").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_cgroup_short_noarg);
        tcase_add_test(tc, test_cgroup_short_space);
        tcase_add_test(tc, test_cgroup_short_equals);
        tcase_add_test(tc, test_cgroup_long_noarg);
        tcase_add_test(tc, test_cgroup_long_space);
        tcase_add_test(tc, test_cgroup_long_equals);
        tcase_add_test(tc, test_priority_short);
        tcase_add_test(tc, test_priority_long);
        suite_add_tcase(s, tc);

        let name = CString::new("output").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_quiet_short);
        tcase_add_test(tc, test_quiet_long);
        suite_add_tcase(s, tc);

        let name = CString::new("system_tuning").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_trace_buffer_size);
        tcase_add_test(tc, test_warm_up);
        suite_add_tcase(s, tc);

        let name = CString::new("aa_actions").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_auto);
        tcase_add_test(tc, test_on_end);
        tcase_add_test(tc, test_on_threshold);
        suite_add_tcase(s, tc);

        let name = CString::new("general").unwrap();
        tc = tcase_create(name.as_ptr());
        tcase_add_test(tc, test_debug_short);
        tcase_add_test(tc, test_debug_long);
        tcase_add_test(tc, test_duration_short);
        tcase_add_test(tc, test_duration_long);
        suite_add_tcase(s, tc);

        s
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
