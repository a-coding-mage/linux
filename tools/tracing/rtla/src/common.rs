// SPDX-License-Identifier: GPL-2.0
// Translated from common.c. C includes and _GNU_SOURCE are build-context only.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type size_t = usize;
pub type time_t = isize;
pub type pthread_t = usize;
pub type sighandler_t = Option<unsafe extern "C" fn(c_int)>;
pub type result = c_int;

pub const ERROR: result = 1;
pub const PASSED: result = 0;
pub const FAILED: result = 2;
pub const SIGINT: c_int = 2;
pub const SIGALRM: c_int = 14;
pub const SIG_DFL: sighandler_t = None;
pub const EXIT_SUCCESS: c_int = 0;
pub const ACTION_TRACE_OUTPUT: usize = 0;

#[repr(C)]
pub struct cpu_set_t {
    _private: [usize; 16],
}

#[repr(C)]
pub struct sched_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut c_void,
    pub tep: *mut c_void,
}

#[repr(C)]
pub struct trace_context {
    pub inst: *mut c_void,
}

#[repr(C)]
pub struct actions {
    pub present: *mut bool,
    pub trace_output_inst: *mut c_void,
}

#[repr(C)]
pub struct timerlat_user_params {
    pub should_run: c_int,
    pub stopped_running: c_int,
    pub set: *mut cpu_set_t,
    pub sched_param: *mut sched_attr,
    pub cgroup_name: *mut c_char,
}

#[repr(C)]
pub struct common_params {
    pub sleep_time: c_uint,
    pub duration: c_uint,
    pub cpus: *const c_char,
    pub monitored_cpus: cpu_set_t,
    pub hk_cpus: *const c_char,
    pub hk_cpu_set: cpu_set_t,
    pub kernel_workload: c_int,
    pub threshold_actions: actions,
    pub end_actions: actions,
    pub events: *mut c_void,
    pub buffer_size: c_int,
    pub user_workload: bool,
    pub user: timerlat_user_params,
    pub set_sched: bool,
    pub sched_param: sched_attr,
    pub cgroup: bool,
    pub user_data: *mut c_void,
    pub cgroup_name: *mut c_char,
    pub aa_only: bool,
    pub quiet: bool,
    pub stop_us: c_long,
    pub stop_total_us: c_long,
}

pub type c_long = isize;

#[repr(C)]
pub struct osnoise_tool {
    pub trace: trace_context,
    pub record: *mut osnoise_tool,
    pub aa: *mut osnoise_tool,
    pub params: *mut common_params,
    pub ops: *mut tool_ops,
    pub context: *mut c_void,
    pub start_time: time_t,
}

#[repr(C)]
pub struct tool_ops {
    pub parse_args: unsafe extern "C" fn(c_int, *mut *mut c_char) -> *mut common_params,
    pub init_tool: unsafe extern "C" fn(*mut common_params) -> *mut osnoise_tool,
    pub apply_config: unsafe extern "C" fn(*mut osnoise_tool) -> c_int,
    pub tracer: *const c_char,
    pub comm_prefix: *const c_char,
    pub enable: unsafe extern "C" fn(*mut osnoise_tool) -> c_int,
    pub main: unsafe extern "C" fn(*mut osnoise_tool) -> c_int,
    pub print_stats: unsafe extern "C" fn(*mut osnoise_tool),
    pub analyze: Option<unsafe extern "C" fn(*mut osnoise_tool, bool)>,
    pub free: unsafe extern "C" fn(*mut osnoise_tool),
}

unsafe extern "C" {
    static VERSION: *const c_char;
    static mut stderr: *mut c_void;

    fn tracefs_iterate_stop(inst: *mut c_void);
    fn trace_instance_stop(trace: *mut trace_context);
    fn trace_instance_start(trace: *mut trace_context) -> c_int;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn alarm(seconds: c_uint) -> c_uint;
    fn osnoise_set_cpus(context: *mut c_void, cpus: *const c_char) -> c_int;
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn getpid() -> c_int;
    fn auto_house_keeping(set: *mut cpu_set_t);
    fn osnoise_set_workload(context: *mut c_void, kernel_workload: c_int) -> c_int;
    fn err_msg(fmt: *const c_char, ...);
    fn debug_msg(fmt: *const c_char, ...);
    fn actions_perform(actions: *mut actions);
    fn should_continue_tracing(params: *mut common_params) -> bool;
    fn get_nprocs_conf() -> c_int;
    fn exit(status: c_int) -> !;
    fn enable_tracer_by_name(inst: *mut c_void, tracer: *const c_char) -> c_int;
    fn set_comm_sched_attr(comm_prefix: *const c_char, sched_param: *mut sched_attr) -> c_int;
    fn set_comm_cgroup(comm_prefix: *const c_char, cgroup_name: *mut c_char) -> c_int;
    fn osnoise_init_trace_tool(tracer: *const c_char) -> *mut osnoise_tool;
    fn trace_events_enable(trace: *mut trace_context, events: *mut c_void) -> c_int;
    fn trace_set_buffer_size(trace: *mut trace_context, size: c_int) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn timerlat_u_dispatcher(arg: *mut c_void) -> *mut c_void;
    fn time(tloc: *mut time_t) -> time_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn printf(fmt: *const c_char, ...);
    fn trace_events_destroy(trace: *mut trace_context, events: *mut c_void);
    fn osnoise_destroy_tool(tool: *mut osnoise_tool);
    fn actions_destroy(actions: *mut actions);
    fn osnoise_trace_is_off(tool: *mut osnoise_tool, record: *mut osnoise_tool) -> bool;
    fn tracefs_iterate_raw_events(
        tep: *mut c_void,
        inst: *mut c_void,
        cpu: *mut c_void,
        flags: c_int,
        callback: unsafe extern "C" fn(),
        context: *mut c_void,
    ) -> c_int;
    fn collect_registered_events();
    fn osnoise_set_stop_us(context: *mut c_void, stop_us: c_long) -> c_int;
    fn osnoise_set_stop_total_us(context: *mut c_void, stop_total_us: c_long) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub static mut trace_tool: *mut osnoise_tool = ptr::null_mut();
#[no_mangle]
pub static mut stop_tracing: c_int = 0;
#[no_mangle]
pub static mut nr_cpus: c_int = 0;

unsafe extern "C" fn stop_trace(_sig: c_int) {
    unsafe {
        if stop_tracing != 0 {
            /*
             * Stop requested twice in a row; abort event processing and
             * exit immediately
             */
            if !trace_tool.is_null() {
                tracefs_iterate_stop((*trace_tool).trace.inst);
            }
            return;
        }
        stop_tracing = 1;
        if !trace_tool.is_null() {
            trace_instance_stop(&mut (*trace_tool).trace);
            if !(*trace_tool).record.is_null() {
                trace_instance_stop(&mut (*(*trace_tool).record).trace);
            }
        }
    }
}

/*
 * set_signals - handles the signal to stop the tool
 */
unsafe fn set_signals(params: *mut common_params) {
    unsafe {
        signal(SIGINT, Some(stop_trace));
        if (*params).duration != 0 {
            signal(SIGALRM, Some(stop_trace));
            alarm((*params).duration);
        }
    }
}

/*
 * unset_signals - unsets the signals to stop the tool
 */
unsafe fn unset_signals(params: *mut common_params) {
    unsafe {
        signal(SIGINT, SIG_DFL);
        if (*params).duration != 0 {
            alarm(0);
            signal(SIGALRM, SIG_DFL);
        }
    }
}

/*
 * common_apply_config - apply common configs to the initialized tool
 */
#[no_mangle]
pub unsafe extern "C" fn common_apply_config(
    tool: *mut osnoise_tool,
    params: *mut common_params,
) -> c_int {
    unsafe {
        let mut retval: c_int;
        let mut i: c_int;

        if (*params).sleep_time == 0 {
            (*params).sleep_time = 1;
        }

        retval = osnoise_set_cpus(
            (*tool).context,
            if !(*params).cpus.is_null() {
                (*params).cpus
            } else {
                b"all\0".as_ptr() as *const c_char
            },
        );
        if retval != 0 {
            err_msg(b"Failed to apply CPUs config\n\0".as_ptr() as *const c_char);
            return -1;
        }

        if (*params).cpus.is_null() {
            i = 0;
            while i < nr_cpus {
                CPU_SET(i, &mut (*params).monitored_cpus);
                i += 1;
            }
        }

        if !(*params).hk_cpus.is_null() {
            retval = sched_setaffinity(
                getpid(),
                core::mem::size_of_val(&(*params).hk_cpu_set),
                &(*params).hk_cpu_set,
            );
            if retval == -1 {
                err_msg(
                    b"Failed to set rtla to the house keeping CPUs\n\0".as_ptr()
                        as *const c_char,
                );
                return -1;
            }
        } else if !(*params).cpus.is_null() {
            /*
             * Even if the user do not set a house-keeping CPU, try to
             * move rtla to a CPU set different to the one where the user
             * set the workload to run.
             *
             * No need to check results as this is an automatic attempt.
             */
            auto_house_keeping(&mut (*params).monitored_cpus);
        }

        /*
         * Set workload according to type of thread if the kernel supports it.
         * On kernels without support, user threads will have already failed
         * on missing fd, and kernel threads do not need it.
         */
        retval = osnoise_set_workload((*tool).context, (*params).kernel_workload);
        if retval < -1 {
            err_msg(b"Failed to set OSNOISE_WORKLOAD option\n\0".as_ptr() as *const c_char);
            return -1;
        }

        0
    }
}

/**
 * common_threshold_handler - handle latency threshold overflow
 * @tool: pointer to the osnoise_tool instance containing trace contexts
 *
 * Executes the configured threshold actions (e.g., saving trace, printing,
 * sending signals). If the continue flag is set (--on-threshold continue),
 * restarts the auxiliary trace instances to continue monitoring.
 *
 * Return: 0 for success, -1 for error.
 */
#[no_mangle]
pub unsafe extern "C" fn common_threshold_handler(tool: *const osnoise_tool) -> c_int {
    unsafe {
        actions_perform(&mut (*(*tool).params).threshold_actions);

        if !should_continue_tracing((*tool).params) {
            /* continue flag not set, break */
            return 0;
        }

        /* continue action reached, re-enable tracing */
        if !(*tool).record.is_null() && trace_instance_start(&mut (*(*tool).record).trace) != 0 {
            err_msg(b"Error restarting trace\n\0".as_ptr() as *const c_char);
            return -1;
        }
        if !(*tool).aa.is_null() && trace_instance_start(&mut (*(*tool).aa).trace) != 0 {
            err_msg(b"Error restarting trace\n\0".as_ptr() as *const c_char);
            return -1;
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn run_tool(
    ops: *mut tool_ops,
    argc: c_int,
    argv: *mut *mut c_char,
) -> c_int {
    unsafe {
        let params: *mut common_params;
        let mut return_value: result = ERROR;
        let tool: *mut osnoise_tool;
        let stopped: bool;
        let mut retval: c_int;

        nr_cpus = get_nprocs_conf();
        params = ((*ops).parse_args)(argc, argv);
        if params.is_null() {
            exit(1);
        }

        tool = ((*ops).init_tool)(params);
        if tool.is_null() {
            err_msg(b"Could not init osnoise tool\n\0".as_ptr() as *const c_char);
            exit(return_value);
        }
        (*tool).ops = ops;
        (*tool).params = params;

        /*
         * Expose the tool to signal handlers so they can stop the trace.
         * Otherwise, rtla could loop indefinitely when overloaded.
         */
        trace_tool = tool;

        retval = ((*ops).apply_config)(tool);
        if retval != 0 {
            err_msg(b"Could not apply config\n\0".as_ptr() as *const c_char);
            goto_out_free(tool, params, return_value);
        }

        retval = enable_tracer_by_name((*tool).trace.inst, (*ops).tracer);
        if retval != 0 {
            err_msg(b"Failed to enable %s tracer\n\0".as_ptr() as *const c_char, (*ops).tracer);
            goto_out_free(tool, params, return_value);
        }

        if (*params).set_sched {
            retval = set_comm_sched_attr((*ops).comm_prefix, &mut (*params).sched_param);
            if retval != 0 {
                err_msg(b"Failed to set sched parameters\n\0".as_ptr() as *const c_char);
                goto_out_free(tool, params, return_value);
            }
        }

        if (*params).cgroup && (*params).user_data.is_null() {
            retval = set_comm_cgroup((*ops).comm_prefix, (*params).cgroup_name);
            if retval == 0 {
                err_msg(b"Failed to move threads to cgroup\n\0".as_ptr() as *const c_char);
                goto_out_free(tool, params, return_value);
            }
        }

        if *(*params).threshold_actions.present.add(ACTION_TRACE_OUTPUT)
            || *(*params).end_actions.present.add(ACTION_TRACE_OUTPUT)
        {
            (*tool).record = osnoise_init_trace_tool((*ops).tracer);
            if (*tool).record.is_null() {
                err_msg(b"Failed to enable the trace instance\n\0".as_ptr() as *const c_char);
                goto_out_free(tool, params, return_value);
            }
            (*params).threshold_actions.trace_output_inst = (*(*tool).record).trace.inst;
            (*params).end_actions.trace_output_inst = (*(*tool).record).trace.inst;

            if !(*params).events.is_null() {
                retval = trace_events_enable(&mut (*(*tool).record).trace, (*params).events);
                if retval != 0 {
                    goto_out_trace(tool, params, return_value);
                }
            }

            if (*params).buffer_size > 0 {
                retval =
                    trace_set_buffer_size(&mut (*(*tool).record).trace, (*params).buffer_size);
                if retval != 0 {
                    goto_out_trace(tool, params, return_value);
                }
            }
        }

        if (*params).user_workload {
            let mut user_thread: pthread_t = 0;

            /* rtla asked to stop */
            (*params).user.should_run = 1;
            /* all threads left */
            (*params).user.stopped_running = 0;

            (*params).user.set = &mut (*params).monitored_cpus;
            if (*params).set_sched {
                (*params).user.sched_param = &mut (*params).sched_param;
            } else {
                (*params).user.sched_param = ptr::null_mut();
            }

            (*params).user.cgroup_name = (*params).cgroup_name;

            retval = pthread_create(
                &mut user_thread,
                ptr::null(),
                timerlat_u_dispatcher,
                &mut (*params).user as *mut timerlat_user_params as *mut c_void,
            );
            if retval != 0 {
                err_msg(
                    b"Error creating timerlat user-space threads\n\0".as_ptr() as *const c_char,
                );
                goto_out_trace(tool, params, return_value);
            }
        }

        retval = ((*ops).enable)(tool);
        if retval != 0 {
            goto_out_trace(tool, params, return_value);
        }

        (*tool).start_time = time(ptr::null_mut());
        set_signals(params);

        retval = ((*ops).main)(tool);
        if retval != 0 {
            unset_signals(params);
            goto_out_trace(tool, params, return_value);
        }

        if (*params).user_workload && (*params).user.stopped_running == 0 {
            (*params).user.should_run = 0;
            sleep(1);
        }

        ((*ops).print_stats)(tool);

        actions_perform(&mut (*params).end_actions);

        return_value = PASSED;

        stopped = osnoise_trace_is_off(tool, (*tool).record) && stop_tracing == 0;
        if stopped {
            printf(b"%s hit stop tracing\n\0".as_ptr() as *const c_char, (*ops).tracer);
            return_value = FAILED;
        }

        if let Some(analyze) = (*ops).analyze {
            analyze(tool, stopped);
        }

        unset_signals(params);
        goto_out_trace(tool, params, return_value);
    }
}

unsafe fn goto_out_trace(tool: *mut osnoise_tool, params: *mut common_params, return_value: result) -> ! {
    unsafe {
        trace_events_destroy(&mut (*(*tool).record).trace, (*params).events);
        (*params).events = ptr::null_mut();
        goto_out_free(tool, params, return_value);
    }
}

unsafe fn goto_out_free(tool: *mut osnoise_tool, params: *mut common_params, return_value: result) -> ! {
    unsafe {
        ((*(*tool).ops).free)(tool);
        osnoise_destroy_tool((*tool).record);
        osnoise_destroy_tool(tool);
        actions_destroy(&mut (*params).threshold_actions);
        actions_destroy(&mut (*params).end_actions);
        free(params as *mut c_void);
        exit(return_value);
    }
}

#[no_mangle]
pub unsafe extern "C" fn top_main_loop(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params: *mut common_params = (*tool).params;
        let trace: *mut trace_context = &mut (*tool).trace;
        let record: *mut osnoise_tool = (*tool).record;
        let mut retval: c_int;

        while stop_tracing == 0 {
            sleep((*params).sleep_time);

            if (*params).aa_only && !osnoise_trace_is_off(tool, record) {
                continue;
            }

            retval = tracefs_iterate_raw_events(
                (*trace).tep,
                (*trace).inst,
                ptr::null_mut(),
                0,
                collect_registered_events,
                trace as *mut c_void,
            );
            if retval < 0 {
                err_msg(b"Error iterating on events\n\0".as_ptr() as *const c_char);
                return retval;
            }

            if !(*params).quiet {
                ((*(*tool).ops).print_stats)(tool);
            }

            if osnoise_trace_is_off(tool, record) {
                if stop_tracing != 0 {
                    /* stop tracing requested, do not perform actions */
                    return 0;
                }

                retval = common_threshold_handler(tool);
                if retval != 0 {
                    return retval;
                }

                if !should_continue_tracing(params) {
                    return 0;
                }

                trace_instance_start(trace);
            }

            /* is there still any user-threads ? */
            if (*params).user_workload {
                if (*params).user.stopped_running != 0 {
                    debug_msg(b"timerlat user space threads stopped!\n\0".as_ptr() as *const c_char);
                    break;
                }
            }
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn hist_main_loop(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params: *mut common_params = (*tool).params;
        let trace: *mut trace_context = &mut (*tool).trace;
        let mut retval: c_int = 0;

        while stop_tracing == 0 {
            sleep((*params).sleep_time);

            retval = tracefs_iterate_raw_events(
                (*trace).tep,
                (*trace).inst,
                ptr::null_mut(),
                0,
                collect_registered_events,
                trace as *mut c_void,
            );
            if retval < 0 {
                err_msg(b"Error iterating on events\n\0".as_ptr() as *const c_char);
                break;
            }

            if osnoise_trace_is_off(tool, (*tool).record) {
                if stop_tracing != 0 {
                    /* stop tracing requested, do not perform actions */
                    break;
                }

                retval = common_threshold_handler(tool);
                if retval != 0 {
                    return retval;
                }

                if !should_continue_tracing(params) {
                    return 0;
                }

                trace_instance_start(trace);
            }

            /* is there still any user-threads ? */
            if (*params).user_workload {
                if (*params).user.stopped_running != 0 {
                    debug_msg(b"user-space threads stopped!\n\0".as_ptr() as *const c_char);
                    break;
                }
            }
        }

        retval
    }
}

#[no_mangle]
pub unsafe extern "C" fn osn_set_stop(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params: *mut common_params = (*tool).params;
        let mut retval: c_int;

        retval = osnoise_set_stop_us((*tool).context, (*params).stop_us);
        if retval != 0 {
            err_msg(b"Failed to set stop us\n\0".as_ptr() as *const c_char);
            return retval;
        }

        retval = osnoise_set_stop_total_us((*tool).context, (*params).stop_total_us);
        if retval != 0 {
            err_msg(b"Failed to set stop total us\n\0".as_ptr() as *const c_char);
            return retval;
        }

        0
    }
}

unsafe fn print_msg_array(msgs: *const *const c_char) {
    unsafe {
        if msgs.is_null() {
            return;
        }

        let mut i: isize = 0;
        while !(*msgs.offset(i)).is_null() {
            fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, *msgs.offset(i));
            i += 1;
        }
    }
}

/*
 * common_usage - print complete usage information
 */
#[no_mangle]
pub unsafe extern "C" fn common_usage(
    tool: *const c_char,
    mode: *const c_char,
    desc: *const c_char,
    start_msgs: *const *const c_char,
    opt_msgs: *const *const c_char,
) {
    unsafe {
        static common_options: [*const c_char; 2] = [
            b"\t  -h/--help: print this menu\0".as_ptr() as *const c_char,
            ptr::null(),
        ];
        fprintf(stderr, b"rtla %s\0".as_ptr() as *const c_char, tool);
        if strcmp(mode, b"\0".as_ptr() as *const c_char) != 0 {
            fprintf(stderr, b" %s\0".as_ptr() as *const c_char, mode);
        }
        fprintf(
            stderr,
            b": %s (version %s)\n\n\0".as_ptr() as *const c_char,
            desc,
            VERSION,
        );
        fprintf(stderr, b"  usage: [rtla] %s \0".as_ptr() as *const c_char, tool);

        if strcmp(mode, b"top\0".as_ptr() as *const c_char) == 0 {
            fprintf(stderr, b"[top] [-h] \0".as_ptr() as *const c_char);
        } else {
            fprintf(stderr, b"%s [-h] \0".as_ptr() as *const c_char, mode);
        }

        print_msg_array(start_msgs);
        fprintf(stderr, b"\n\0".as_ptr() as *const c_char);
        print_msg_array(common_options.as_ptr());
        print_msg_array(opt_msgs);

        exit(EXIT_SUCCESS);
    }
}
