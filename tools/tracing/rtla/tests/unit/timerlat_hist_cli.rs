// SPDX-License-Identifier: GPL-2.0

// Translated from timerlat_hist_cli.c. C includes:
// <check.h>, <stdio.h>, <stdlib.h>, <sched.h>, <limits.h>, <unistd.h>,
// <sys/sysinfo.h>, <linux/container_of.h>, "cli_params_assert.h",
// "../../src/cli.h"

use core::ffi::{c_char, c_int};

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
pub struct sched_param_compat {
    pub sched_policy: c_int,
    pub sched_priority: c_int,
}

#[repr(C)]
pub struct hist_params {
    pub bucket_size: c_int,
    pub entries: c_int,
    pub no_header: bool,
    pub no_index: bool,
    pub no_irq: bool,
    pub no_summary: bool,
    pub no_thread: bool,
    pub with_zeros: bool,
}

#[repr(C)]
pub struct common_params {
    pub stop_us: c_int,
    pub stop_total_us: c_int,
    pub duration: c_int,
    pub cpus: *const c_char,
    pub cgroup: bool,
    pub cgroup_name: *const c_char,
    pub kernel_workload: bool,
    pub user_workload: bool,
    pub user_data: bool,
    pub sched_param: sched_param_compat,
    pub hist: hist_params,
    pub output_divisor: c_int,
    pub buffer_size: c_int,
    pub warmup: c_int,
}

#[repr(C)]
pub struct timerlat_params {
    pub timerlat_period_us: c_int,
    pub print_stack: c_int,
    pub timerlat_align: bool,
    pub timerlat_align_us: c_int,
    pub deepest_idle_state: c_int,
    pub dma_latency: c_int,
    pub bpf_action_program: *const c_char,
    pub dump_tasks: bool,
    pub no_aa: bool,
    pub stack_format: c_int,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static mut config_debug: bool;

    fn timerlat_hist_parse_args(argc: c_int, argv: *mut *mut c_char) -> *mut common_params;
    fn to_timerlat_params(params: *mut common_params) -> *mut timerlat_params;

    fn suite_create(name: *const c_char) -> *mut Suite;
    fn tcase_create(name: *const c_char) -> *mut TCase;
    fn tcase_add_test(tc: *mut TCase, test: unsafe extern "C" fn(c_int));
    fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);

    fn ck_assert(expr: bool);
    fn ck_assert_int_eq(actual: c_int, expected: c_int);
    fn ck_assert_str_eq(actual: *const c_char, expected: *const c_char);
    fn ck_assert_ptr_null(ptr: *const c_char);

    fn CLI_ASSERT_SINGLE_ACTION(
        actions_name: *const c_char,
        action: c_int,
        field_name: *const c_char,
        kind_name: *const c_char,
        expected: *const c_char,
    );
    fn CLI_ASSERT_SINGLE_EVENT(system: *const c_char, event: *const c_char);
    fn CLI_ASSERT_SINGLE_FILTER(filter: *const c_char);
    fn CLI_ASSERT_SINGLE_TRIGGER(trigger: *const c_char);
    fn CLI_ASSERT_CPUSET(set_name: *const c_char, cpu0: c_int, cpu1: c_int, cpu2: c_int);
    fn CLI_TIMERLAT_ASSERT_AUTO(value: c_int);
}

const ACTION_TRACE_OUTPUT: c_int = 0;
const STACK_FORMAT_TRUNCATE: c_int = 0;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! parse_args {
    ($($arg:expr),+ $(,)?) => {{
        let mut argv = [$(c_str!($arg) as *mut c_char,)+ core::ptr::null_mut()];
        let argc = (argv.len() - 1) as c_int;
        let params = timerlat_hist_parse_args(argc, argv.as_mut_ptr());
        let tlat_params = to_timerlat_params(params);
        (params, tlat_params)
    }};
}

/* Tracing Options */

unsafe extern "C" fn test_irq_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-i", "20");
    ck_assert_int_eq((*params).stop_us, 20);
}

unsafe extern "C" fn test_irq_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--irq", "20");
    ck_assert_int_eq((*params).stop_us, 20);
}

unsafe extern "C" fn test_period_short(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "-p", "200");
    ck_assert_int_eq((*tlat_params).timerlat_period_us, 200);
}

unsafe extern "C" fn test_period_long(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--period", "200");
    ck_assert_int_eq((*tlat_params).timerlat_period_us, 200);
}

unsafe extern "C" fn test_period_unset_short(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "-p", "200", "--no-period");
    ck_assert_int_eq((*tlat_params).timerlat_period_us, 0);
}

unsafe extern "C" fn test_period_unset_long(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--period", "200", "--no-period");
    ck_assert_int_eq((*tlat_params).timerlat_period_us, 0);
}

unsafe extern "C" fn test_stack_short(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "-s", "20");
    ck_assert_int_eq((*tlat_params).print_stack, 20);
}

unsafe extern "C" fn test_stack_long(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--stack", "20");
    ck_assert_int_eq((*tlat_params).print_stack, 20);
}

unsafe extern "C" fn test_thread_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-T", "20");
    ck_assert_int_eq((*params).stop_total_us, 20);
}

unsafe extern "C" fn test_thread_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--thread", "20");
    ck_assert_int_eq((*params).stop_total_us, 20);
}

unsafe extern "C" fn test_trace_short_noarg(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-t");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
}

unsafe extern "C" fn test_trace_short_followarg(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-t", "-d", "20");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
    ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
}

unsafe extern "C" fn test_trace_short_space(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-t", "tracefile");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("tracefile"));
}

unsafe extern "C" fn test_trace_short_equals(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-t=tracefile");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("tracefile"));
}

unsafe extern "C" fn test_trace_long_noarg(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--trace");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
}

unsafe extern "C" fn test_trace_long_followarg(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--trace", "-d", "20");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
    ck_assert_int_eq((*params).duration, 20); /* check if next argument is read correctly */
}

unsafe extern "C" fn test_trace_long_space(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--trace", "tracefile");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("tracefile"));
}

unsafe extern "C" fn test_trace_long_equals(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--trace=tracefile");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("tracefile"));
}

/* Event Configuration */

unsafe extern "C" fn test_event_short(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-e", "system:event");
    CLI_ASSERT_SINGLE_EVENT(c_str!("system"), c_str!("event"));
}

unsafe extern "C" fn test_event_long(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--event", "system:event");
    CLI_ASSERT_SINGLE_EVENT(c_str!("system"), c_str!("event"));
}

unsafe extern "C" fn test_filter(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-e", "system:event", "--filter", "filter");
    CLI_ASSERT_SINGLE_FILTER(c_str!("filter"));
}

unsafe extern "C" fn test_trigger(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-e", "system:event", "--trigger", "trigger");
    CLI_ASSERT_SINGLE_TRIGGER(c_str!("trigger"));
}

/* CPU Configuration */

unsafe extern "C" fn test_cpus_short(_: c_int) {
    nr_cpus = 4;
    let (params, _) = parse_args!("timerlat", "hist", "-c", "0-1,3");
    ck_assert_str_eq((*params).cpus, c_str!("0-1,3"));
    CLI_ASSERT_CPUSET(c_str!("monitored_cpus"), 0, 1, 3);
}

unsafe extern "C" fn test_cpus_long(_: c_int) {
    nr_cpus = 4;
    let (params, _) = parse_args!("timerlat", "hist", "--cpus", "0-1,3");
    ck_assert_str_eq((*params).cpus, c_str!("0-1,3"));
    CLI_ASSERT_CPUSET(c_str!("monitored_cpus"), 0, 1, 3);
}

unsafe extern "C" fn test_housekeeping_short(_: c_int) {
    nr_cpus = 4;
    let _ = parse_args!("timerlat", "hist", "-H", "0-1,3");
    CLI_ASSERT_CPUSET(c_str!("hk_cpu_set"), 0, 1, 3);
}

unsafe extern "C" fn test_housekeeping_long(_: c_int) {
    nr_cpus = 4;
    let _ = parse_args!("timerlat", "hist", "--house-keeping", "0-1,3");
    CLI_ASSERT_CPUSET(c_str!("hk_cpu_set"), 0, 1, 3);
}

/* Thread Configuration */

unsafe extern "C" fn test_cgroup_short_noarg(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-C");
    ck_assert((*params).cgroup);
    ck_assert_ptr_null((*params).cgroup_name);
}

unsafe extern "C" fn test_cgroup_short_space(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-C", "cgroup");
    ck_assert((*params).cgroup);
    ck_assert_str_eq((*params).cgroup_name, c_str!("cgroup"));
}

unsafe extern "C" fn test_cgroup_short_equals(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-C=cgroup");
    ck_assert((*params).cgroup);
    ck_assert_str_eq((*params).cgroup_name, c_str!("cgroup"));
}

unsafe extern "C" fn test_cgroup_long_noarg(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--cgroup");
    ck_assert((*params).cgroup);
    ck_assert_ptr_null((*params).cgroup_name);
}

unsafe extern "C" fn test_cgroup_long_space(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--cgroup", "cgroup");
    ck_assert((*params).cgroup);
    ck_assert_str_eq((*params).cgroup_name, c_str!("cgroup"));
}

unsafe extern "C" fn test_cgroup_long_equals(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--cgroup=cgroup");
    ck_assert((*params).cgroup);
    ck_assert_str_eq((*params).cgroup_name, c_str!("cgroup"));
}

unsafe extern "C" fn test_kernel_threads_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-k");
    ck_assert((*params).kernel_workload);
    ck_assert(!(*params).user_workload);
    ck_assert(!(*params).user_data);
}

unsafe extern "C" fn test_kernel_threads_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--kernel-threads");
    ck_assert((*params).kernel_workload);
    ck_assert(!(*params).user_workload);
    ck_assert(!(*params).user_data);
}

unsafe extern "C" fn test_priority_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-P", "f:95");
    ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
    ck_assert_int_eq((*params).sched_param.sched_priority, 95);
}

unsafe extern "C" fn test_priority_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--priority", "f:95");
    ck_assert_int_eq((*params).sched_param.sched_policy, SCHED_FIFO);
    ck_assert_int_eq((*params).sched_param.sched_priority, 95);
}

unsafe extern "C" fn test_user_load_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-U");
    ck_assert(!(*params).kernel_workload);
    ck_assert(!(*params).user_workload);
    ck_assert((*params).user_data);
}

unsafe extern "C" fn test_user_load_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--user-load");
    ck_assert(!(*params).kernel_workload);
    ck_assert(!(*params).user_workload);
    ck_assert((*params).user_data);
}

unsafe extern "C" fn test_user_threads_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-u");
    ck_assert(!(*params).kernel_workload);
    ck_assert((*params).user_workload);
    ck_assert((*params).user_data);
}

unsafe extern "C" fn test_user_threads_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--user-threads");
    ck_assert(!(*params).kernel_workload);
    ck_assert((*params).user_workload);
    ck_assert((*params).user_data);
}

unsafe extern "C" fn test_aligned_short(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "-A", "500");
    ck_assert((*tlat_params).timerlat_align);
    ck_assert_int_eq((*tlat_params).timerlat_align_us, 500);
}

unsafe extern "C" fn test_aligned_long(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--aligned", "500");
    ck_assert((*tlat_params).timerlat_align);
    ck_assert_int_eq((*tlat_params).timerlat_align_us, 500);
}

/* Histogram Options */

unsafe extern "C" fn test_bucket_size_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-b", "2");
    ck_assert_int_eq((*params).hist.bucket_size, 2);
}

unsafe extern "C" fn test_bucket_size_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--bucket-size", "2");
    ck_assert_int_eq((*params).hist.bucket_size, 2);
}

unsafe extern "C" fn test_entries_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-E", "512");
    ck_assert_int_eq((*params).hist.entries, 512);
}

unsafe extern "C" fn test_entries_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--entries", "512");
    ck_assert_int_eq((*params).hist.entries, 512);
}

unsafe extern "C" fn test_no_header(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--no-header");
    ck_assert((*params).hist.no_header);
}

unsafe extern "C" fn test_no_index(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--with-zeros", "--no-index");
    ck_assert((*params).hist.no_index);
}

unsafe extern "C" fn test_no_irq(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--no-irq");
    ck_assert((*params).hist.no_irq);
}

unsafe extern "C" fn test_no_summary(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--no-summary");
    ck_assert((*params).hist.no_summary);
}

unsafe extern "C" fn test_no_thread(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--no-thread");
    ck_assert((*params).hist.no_thread);
}

unsafe extern "C" fn test_with_zeros(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--with-zeros");
    ck_assert((*params).hist.with_zeros);
}

/* Output */

unsafe extern "C" fn test_nano_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-n");
    ck_assert_int_eq((*params).output_divisor, 1);
}

unsafe extern "C" fn test_nano_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--nano");
    ck_assert_int_eq((*params).output_divisor, 1);
}

/* System Tuning */

unsafe extern "C" fn test_deepest_idle_state(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--deepest-idle-state", "1");
    ck_assert_int_eq((*tlat_params).deepest_idle_state, 1);
}

unsafe extern "C" fn test_dma_latency(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--dma-latency", "10");
    ck_assert_int_eq((*tlat_params).dma_latency, 10);
}

unsafe extern "C" fn test_trace_buffer_size(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--trace-buffer-size", "200");
    ck_assert_int_eq((*params).buffer_size, 200);
}

unsafe extern "C" fn test_warm_up(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--warm-up", "5");
    ck_assert_int_eq((*params).warmup, 5);
}

/* Auto Analysis and Actions */

unsafe extern "C" fn test_auto(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-a", "20");
    CLI_TIMERLAT_ASSERT_AUTO(20);
}

unsafe extern "C" fn test_bpf_action(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--bpf-action", "program");
    ck_assert_str_eq((*tlat_params).bpf_action_program, c_str!("program"));
}

unsafe extern "C" fn test_dump_tasks(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--dump-tasks");
    ck_assert((*tlat_params).dump_tasks);
}

unsafe extern "C" fn test_no_aa(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--no-aa");
    ck_assert((*tlat_params).no_aa);
}

unsafe extern "C" fn test_on_end(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--on-end", "trace");
    CLI_ASSERT_SINGLE_ACTION(c_str!("end_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
}

unsafe extern "C" fn test_on_threshold(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--on-threshold", "trace");
    CLI_ASSERT_SINGLE_ACTION(c_str!("threshold_actions"), ACTION_TRACE_OUTPUT, c_str!("trace_output"), c_str!("str"), c_str!("timerlat_trace.txt"));
}

unsafe extern "C" fn test_stack_format(_: c_int) {
    let (_, tlat_params) = parse_args!("timerlat", "hist", "--stack-format", "truncate");
    ck_assert_int_eq((*tlat_params).stack_format, STACK_FORMAT_TRUNCATE);
}

/* General */

unsafe extern "C" fn test_debug_short(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "-D");
    ck_assert(config_debug);
}

unsafe extern "C" fn test_debug_long(_: c_int) {
    let _ = parse_args!("timerlat", "hist", "--debug");
    ck_assert(config_debug);
}

unsafe extern "C" fn test_duration_short(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "-d", "1m");
    ck_assert_int_eq((*params).duration, 60);
}

unsafe extern "C" fn test_duration_long(_: c_int) {
    let (params, _) = parse_args!("timerlat", "hist", "--duration", "1m");
    ck_assert_int_eq((*params).duration, 60);
}

#[no_mangle]
pub unsafe extern "C" fn timerlat_hist_cli_suite() -> *mut Suite {
    let s = suite_create(c_str!("timerlat_hist_cli"));
    let mut tc: *mut TCase;

    tc = tcase_create(c_str!("tracing_options"));
    tcase_add_test(tc, test_irq_short);
    tcase_add_test(tc, test_irq_long);
    tcase_add_test(tc, test_period_short);
    tcase_add_test(tc, test_period_long);
    tcase_add_test(tc, test_period_unset_short);
    tcase_add_test(tc, test_period_unset_long);
    tcase_add_test(tc, test_stack_short);
    tcase_add_test(tc, test_stack_long);
    tcase_add_test(tc, test_thread_short);
    tcase_add_test(tc, test_thread_long);
    tcase_add_test(tc, test_trace_short_noarg);
    tcase_add_test(tc, test_trace_short_followarg);
    tcase_add_test(tc, test_trace_short_space);
    tcase_add_test(tc, test_trace_short_equals);
    tcase_add_test(tc, test_trace_long_noarg);
    tcase_add_test(tc, test_trace_long_followarg);
    tcase_add_test(tc, test_trace_long_space);
    tcase_add_test(tc, test_trace_long_equals);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("event_configuration"));
    tcase_add_test(tc, test_event_short);
    tcase_add_test(tc, test_event_long);
    tcase_add_test(tc, test_filter);
    tcase_add_test(tc, test_trigger);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("cpu_configuration"));
    tcase_add_test(tc, test_cpus_short);
    tcase_add_test(tc, test_cpus_long);
    tcase_add_test(tc, test_housekeeping_short);
    tcase_add_test(tc, test_housekeeping_long);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("thread_configuration"));
    tcase_add_test(tc, test_cgroup_short_noarg);
    tcase_add_test(tc, test_cgroup_short_space);
    tcase_add_test(tc, test_cgroup_short_equals);
    tcase_add_test(tc, test_cgroup_long_noarg);
    tcase_add_test(tc, test_cgroup_long_space);
    tcase_add_test(tc, test_cgroup_long_equals);
    tcase_add_test(tc, test_kernel_threads_short);
    tcase_add_test(tc, test_kernel_threads_long);
    tcase_add_test(tc, test_priority_short);
    tcase_add_test(tc, test_priority_long);
    tcase_add_test(tc, test_user_load_short);
    tcase_add_test(tc, test_user_load_long);
    tcase_add_test(tc, test_user_threads_short);
    tcase_add_test(tc, test_user_threads_long);
    tcase_add_test(tc, test_aligned_short);
    tcase_add_test(tc, test_aligned_long);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("histogram_options"));
    tcase_add_test(tc, test_bucket_size_short);
    tcase_add_test(tc, test_bucket_size_long);
    tcase_add_test(tc, test_entries_short);
    tcase_add_test(tc, test_entries_long);
    tcase_add_test(tc, test_no_header);
    tcase_add_test(tc, test_no_index);
    tcase_add_test(tc, test_no_irq);
    tcase_add_test(tc, test_no_summary);
    tcase_add_test(tc, test_no_thread);
    tcase_add_test(tc, test_with_zeros);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("output"));
    tcase_add_test(tc, test_nano_short);
    tcase_add_test(tc, test_nano_long);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("system_tuning"));
    tcase_add_test(tc, test_deepest_idle_state);
    tcase_add_test(tc, test_dma_latency);
    tcase_add_test(tc, test_trace_buffer_size);
    tcase_add_test(tc, test_warm_up);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("aa_actions"));
    tcase_add_test(tc, test_auto);
    tcase_add_test(tc, test_bpf_action);
    tcase_add_test(tc, test_dump_tasks);
    tcase_add_test(tc, test_no_aa);
    tcase_add_test(tc, test_on_end);
    tcase_add_test(tc, test_on_threshold);
    tcase_add_test(tc, test_stack_format);
    suite_add_tcase(s, tc);

    tc = tcase_create(c_str!("general"));
    tcase_add_test(tc, test_debug_short);
    tcase_add_test(tc, test_debug_long);
    tcase_add_test(tc, test_duration_short);
    tcase_add_test(tc, test_duration_long);
    suite_add_tcase(s, tc);

    s
}
