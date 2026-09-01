/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: actions.h, timerlat_u.h, trace.h, utils.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct osnoise_context {
    pub flags: c_int,
    pub ref_: c_int,

    pub curr_cpus: *mut c_char,
    pub orig_cpus: *mut c_char,

    /* 0 as init value */
    pub orig_runtime_us: u64,
    pub runtime_us: u64,

    /* 0 as init value */
    pub orig_period_us: u64,
    pub period_us: u64,

    /* 0 as init value */
    pub orig_timerlat_period_us: i64,
    pub timerlat_period_us: i64,

    /* 0 as init value */
    pub orig_tracing_thresh: i64,
    pub tracing_thresh: i64,

    /* -1 as init value because 0 is disabled */
    pub orig_stop_us: i64,
    pub stop_us: i64,

    /* -1 as init value because 0 is disabled */
    pub orig_stop_total_us: i64,
    pub stop_total_us: i64,

    /* -1 as init value because 0 is disabled */
    pub orig_print_stack: i64,
    pub print_stack: i64,

    /* -1 as init value because 0 is off */
    pub orig_opt_irq_disable: c_int,
    pub opt_irq_disable: c_int,

    /* -1 as init value because 0 is off */
    pub orig_opt_workload: c_int,
    pub opt_workload: c_int,

    /* -1 as init value because 0 is off */
    pub orig_opt_timerlat_align: c_int,
    pub opt_timerlat_align: c_int,

    /* 0 as init value */
    pub orig_timerlat_align_us: u64,
    pub timerlat_align_us: u64,
}

unsafe extern "C" {
    pub static mut stop_tracing: c_int;
}

#[repr(C)]
pub struct hist_params {
    pub no_irq: bool,
    pub no_thread: bool,
    pub no_header: bool,
    pub no_summary: bool,
    pub no_index: bool,
    pub with_zeros: bool,
    pub bucket_size: c_int,
    pub entries: c_int,
}

/*
 * common_params - Parameters shared between timerlat_params and osnoise_params
 */
#[repr(C)]
pub struct common_params {
    /* trace configuration */
    pub cpus: *mut c_char,
    pub monitored_cpus: cpu_set_t,
    pub events: *mut trace_events,
    pub buffer_size: c_int,

    /* Timing parameters */
    pub warmup: c_int,
    pub stop_us: i64,
    pub stop_total_us: i64,
    pub sleep_time: c_int,
    pub duration: c_int,

    /* Scheduling parameters */
    pub set_sched: c_int,
    pub sched_param: sched_attr,
    pub cgroup: c_int,
    pub cgroup_name: *mut c_char,
    pub hk_cpus: c_int,
    pub hk_cpu_set: cpu_set_t,

    /* Other parameters */
    pub hist: hist_params,
    pub output_divisor: c_int,
    pub pretty_output: bool,
    pub quiet: bool,
    pub user_workload: bool,
    pub kernel_workload: bool,
    pub user_data: bool,
    pub aa_only: bool,

    pub threshold_actions: actions,
    pub end_actions: actions,
    pub user: timerlat_u_params,
}

unsafe extern "C" {
    pub static mut nr_cpus: c_int;
}

// C macro:
// #define for_each_monitored_cpu(cpu, common) \
//     for (cpu = 0; cpu < nr_cpus; cpu++) \
//         if (!(common)->cpus || CPU_ISSET(cpu, &(common)->monitored_cpus))
//
// The CPU_ISSET dependency is provided externally by the translated environment.
#[macro_export]
macro_rules! for_each_monitored_cpu {
    ($cpu:ident, $common:expr, $body:block) => {{
        $cpu = 0;
        while $cpu < unsafe { nr_cpus } {
            if unsafe { (*$common).cpus.is_null() || CPU_ISSET($cpu, &(*$common).monitored_cpus) } {
                $body
            }
            $cpu += 1;
        }
    }};
}

#[repr(C)]
pub struct tool_ops {
    pub tracer: *const c_char,
    pub comm_prefix: *const c_char,
    pub parse_args: Option<unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> *mut common_params>,
    pub init_tool: Option<unsafe extern "C" fn(params: *mut common_params) -> *mut osnoise_tool>,
    pub apply_config: Option<unsafe extern "C" fn(tool: *mut osnoise_tool) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(tool: *mut osnoise_tool) -> c_int>,
    pub main: Option<unsafe extern "C" fn(tool: *mut osnoise_tool) -> c_int>,
    pub print_stats: Option<unsafe extern "C" fn(tool: *mut osnoise_tool)>,
    pub analyze: Option<unsafe extern "C" fn(tool: *mut osnoise_tool, stopped: bool)>,
    pub free: Option<unsafe extern "C" fn(tool: *mut osnoise_tool)>,
}

/*
 * osnoise_tool -  osnoise based tool definition.
 *
 * Only the "trace" and "context" fields are used for
 * the additional trace instances (record and aa).
 */
#[repr(C)]
pub struct osnoise_tool {
    pub ops: *mut tool_ops,
    pub trace: trace_instance,
    pub context: *mut osnoise_context,
    pub data: *mut c_void,
    pub params: *mut common_params,
    pub start_time: time_t,
    pub record: *mut osnoise_tool,
    pub aa: *mut osnoise_tool,
}

/**
 * should_continue_tracing - check if tracing should continue after threshold
 * @params: pointer to the common parameters structure
 *
 * Returns true if the continue action was configured (--on-threshold continue),
 * indicating that tracing should be restarted after handling the threshold event.
 *
 * Return: 1 if tracing should continue, 0 otherwise.
 */
#[inline]
pub unsafe extern "C" fn should_continue_tracing(params: *const common_params) -> c_int {
    unsafe { (*params).threshold_actions.continue_flag }
}

unsafe extern "C" {
    pub fn common_threshold_handler(tool: *const osnoise_tool) -> c_int;

    pub fn osnoise_set_cpus(context: *mut osnoise_context, cpus: *mut c_char) -> c_int;
    pub fn osnoise_restore_cpus(context: *mut osnoise_context);

    pub fn osnoise_set_workload(context: *mut osnoise_context, onoff: bool) -> c_int;

    pub fn osnoise_destroy_tool(top: *mut osnoise_tool);
    pub fn osnoise_init_tool(tool_name: *mut c_char) -> *mut osnoise_tool;
    pub fn osnoise_init_trace_tool(tracer: *const c_char) -> *mut osnoise_tool;
    pub fn osnoise_trace_is_off(tool: *mut osnoise_tool, record: *mut osnoise_tool) -> bool;
    pub fn osnoise_set_stop_us(context: *mut osnoise_context, stop_us: i64) -> c_int;
    pub fn osnoise_set_stop_total_us(
        context: *mut osnoise_context,
        stop_total_us: i64,
    ) -> c_int;

    pub fn common_apply_config(tool: *mut osnoise_tool, params: *mut common_params) -> c_int;
    pub fn top_main_loop(tool: *mut osnoise_tool) -> c_int;
    pub fn hist_main_loop(tool: *mut osnoise_tool) -> c_int;
    pub fn osn_set_stop(tool: *mut osnoise_tool) -> c_int;

    pub fn common_usage(
        tool: *const c_char,
        mode: *const c_char,
        desc: *const c_char,
        start_msgs: *const *const c_char,
        opt_msgs: *const *const c_char,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
