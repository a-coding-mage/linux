// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies translated from:
// timerlat.h, timerlat_aa.h, timerlat_bpf.h, cli.h, common.h

use core::ffi::{c_char, c_int, c_longlong, c_ulonglong, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct timerlat_top_cpu {
    pub irq_count: c_ulonglong,
    pub thread_count: c_ulonglong,
    pub user_count: c_ulonglong,

    pub cur_irq: c_ulonglong,
    pub min_irq: c_ulonglong,
    pub sum_irq: c_ulonglong,
    pub max_irq: c_ulonglong,

    pub cur_thread: c_ulonglong,
    pub min_thread: c_ulonglong,
    pub sum_thread: c_ulonglong,
    pub max_thread: c_ulonglong,

    pub cur_user: c_ulonglong,
    pub min_user: c_ulonglong,
    pub sum_user: c_ulonglong,
    pub max_user: c_ulonglong,
}

#[repr(C)]
pub struct timerlat_top_data {
    pub cpu_data: *mut timerlat_top_cpu,
}

#[repr(C)]
pub struct osnoise_tool {
    pub trace: trace_instance,
    pub params: *mut common_params,
    pub data: *mut c_void,
    pub start_time: time_t,
}

#[repr(C)]
pub struct trace_instance {
    pub seq: *mut trace_seq,
    pub tep: *mut tep_handle,
}

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_record {
    pub cpu: c_int,
}

#[repr(C)]
pub struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct common_params {
    pub output_divisor: c_ulonglong,
    pub user_data: bool,
    pub pretty_output: bool,
    pub aa_only: bool,
    pub quiet: bool,
    pub sleep_time: c_int,
    pub user_workload: bool,
    pub user: timerlat_user_params,
}

#[repr(C)]
pub struct timerlat_user_params {
    pub stopped_running: bool,
}

#[repr(C)]
pub struct timerlat_params {
    pub common: common_params,
    pub mode: c_int,
}

pub type time_t = c_longlong;

#[repr(C)]
pub struct tool_ops {
    pub tracer: *const c_char,
    pub comm_prefix: *const c_char,
    pub parse_args: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
    pub init_tool: Option<unsafe extern "C" fn(*mut common_params) -> *mut osnoise_tool>,
    pub apply_config: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    pub main: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    pub print_stats: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
    pub analyze: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    pub free: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;
    static mut config_debug: bool;
    static mut stop_tracing: bool;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn isatty(fd: c_int) -> c_int;

    fn timerlat_free(tool: *mut osnoise_tool);
    fn osnoise_init_tool(name: *const c_char) -> *mut osnoise_tool;
    fn osnoise_destroy_tool(tool: *mut osnoise_tool);
    fn timerlat_apply_config(tool: *mut osnoise_tool, params: *mut timerlat_params) -> c_int;
    fn timerlat_enable(tool: *mut osnoise_tool) -> c_int;
    fn top_main_loop(tool: *mut osnoise_tool) -> c_int;
    fn timerlat_analyze(tool: *mut osnoise_tool) -> c_int;
    fn timerlat_top_parse_args(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn osnoise_report_missed_events(tool: *mut osnoise_tool);

    fn to_timerlat_params(params: *mut common_params) -> *mut timerlat_params;
    fn update_min(dst: *mut c_ulonglong, value: *const c_ulonglong);
    fn update_sum(dst: *mut c_ulonglong, value: *const c_ulonglong);
    fn update_max(dst: *mut c_ulonglong, value: *const c_ulonglong);
    fn get_duration(start_time: time_t, duration: *mut c_char, size: usize);

    fn trace_seq_printf(s: *mut trace_seq, fmt: *const c_char, ...);
    fn trace_seq_do_printf(s: *mut trace_seq);
    fn trace_seq_reset(s: *mut trace_seq);
    fn tep_get_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut c_ulonglong,
        err: c_int,
    ) -> c_int;
    fn tep_register_event_handler(
        tep: *mut tep_handle,
        id: c_int,
        sys_name: *const c_char,
        event_name: *const c_char,
        func: Option<
            unsafe extern "C" fn(
                *mut trace_seq,
                *mut tep_record,
                *mut tep_event,
                *mut c_void,
            ) -> c_int,
        >,
        context: *mut c_void,
    ) -> c_int;

    fn timerlat_bpf_get_summary_value(
        summary: c_int,
        value_irq: *mut c_longlong,
        value_thread: *mut c_longlong,
        value_user: *mut c_longlong,
    ) -> c_int;
    fn timerlat_bpf_wait(timeout: c_int) -> c_int;
    fn timerlat_bpf_restart_tracing() -> c_int;
    fn timerlat_bpf_detach();

    fn common_threshold_handler(tool: *mut osnoise_tool) -> c_int;
    fn should_continue_tracing(params: *mut common_params) -> bool;
    fn err_msg(fmt: *const c_char, ...);
    fn debug_msg(fmt: *const c_char, ...);
}

const STDOUT_FILENO: c_int = 1;
const SUMMARY_CURRENT: c_int = 0;
const SUMMARY_COUNT: c_int = 1;
const SUMMARY_MIN: c_int = 2;
const SUMMARY_MAX: c_int = 3;
const SUMMARY_SUM: c_int = 4;
const TRACING_MODE_TRACEFS: c_int = 0;

/*
 * timerlat_free_top - free runtime data
 */
unsafe extern "C" fn timerlat_free_top(data: *mut timerlat_top_data) {
    unsafe {
        free((*data).cpu_data as *mut c_void);
        free(data as *mut c_void);
    }
}

unsafe extern "C" fn timerlat_free_top_tool(tool: *mut osnoise_tool) {
    unsafe {
        timerlat_free_top((*tool).data as *mut timerlat_top_data);
        timerlat_free(tool);
    }
}

/*
 * timerlat_alloc_histogram - alloc runtime data
 */
unsafe extern "C" fn timerlat_alloc_top() -> *mut timerlat_top_data {
    let data: *mut timerlat_top_data;
    let mut cpu: c_int;

    unsafe {
        data = calloc(1, mem::size_of::<timerlat_top_data>()) as *mut timerlat_top_data;
        if data.is_null() {
            return ptr::null_mut();
        }

        /* one set of histograms per CPU */
        (*data).cpu_data = calloc(
            1,
            mem::size_of::<timerlat_top_cpu>() * nr_cpus as usize,
        ) as *mut timerlat_top_cpu;
        if (*data).cpu_data.is_null() {
            timerlat_free_top(data);
            return ptr::null_mut();
        }

        /* set the min to max */
        cpu = 0;
        while cpu < nr_cpus {
            (*(*data).cpu_data.add(cpu as usize)).min_irq = !0;
            (*(*data).cpu_data.add(cpu as usize)).min_thread = !0;
            (*(*data).cpu_data.add(cpu as usize)).min_user = !0;
            cpu += 1;
        }

        data
    }
}

unsafe extern "C" fn timerlat_top_reset_sum(summary: *mut timerlat_top_cpu) {
    unsafe {
        memset(
            summary as *mut c_void,
            0,
            mem::size_of::<timerlat_top_cpu>(),
        );
        (*summary).min_irq = !0;
        (*summary).min_thread = !0;
        (*summary).min_user = !0;
    }
}

unsafe extern "C" fn timerlat_top_update_sum(
    tool: *mut osnoise_tool,
    cpu: c_int,
    sum: *mut timerlat_top_cpu,
) {
    unsafe {
        let data = (*tool).data as *mut timerlat_top_data;
        let cpu_data = (*data).cpu_data.add(cpu as usize);

        (*sum).irq_count += (*cpu_data).irq_count;
        update_min(&mut (*sum).min_irq, &(*cpu_data).min_irq);
        update_sum(&mut (*sum).sum_irq, &(*cpu_data).sum_irq);
        update_max(&mut (*sum).max_irq, &(*cpu_data).max_irq);

        (*sum).thread_count += (*cpu_data).thread_count;
        update_min(&mut (*sum).min_thread, &(*cpu_data).min_thread);
        update_sum(&mut (*sum).sum_thread, &(*cpu_data).sum_thread);
        update_max(&mut (*sum).max_thread, &(*cpu_data).max_thread);

        (*sum).user_count += (*cpu_data).user_count;
        update_min(&mut (*sum).min_user, &(*cpu_data).min_user);
        update_sum(&mut (*sum).sum_user, &(*cpu_data).sum_user);
        update_max(&mut (*sum).max_user, &(*cpu_data).max_user);
    }
}

/*
 * timerlat_hist_update - record a new timerlat occurent on cpu, updating data
 */
unsafe extern "C" fn timerlat_top_update(
    tool: *mut osnoise_tool,
    cpu: c_int,
    thread: c_ulonglong,
    mut latency: c_ulonglong,
) {
    unsafe {
        let params = to_timerlat_params((*tool).params);
        let data = (*tool).data as *mut timerlat_top_data;
        let cpu_data = (*data).cpu_data.add(cpu as usize);

        if (*params).common.output_divisor != 0 {
            latency = latency / (*params).common.output_divisor;
        }

        if thread == 0 {
            (*cpu_data).irq_count += 1;
            (*cpu_data).cur_irq = latency;
            update_min(&mut (*cpu_data).min_irq, &latency);
            update_sum(&mut (*cpu_data).sum_irq, &latency);
            update_max(&mut (*cpu_data).max_irq, &latency);
        } else if thread == 1 {
            (*cpu_data).thread_count += 1;
            (*cpu_data).cur_thread = latency;
            update_min(&mut (*cpu_data).min_thread, &latency);
            update_sum(&mut (*cpu_data).sum_thread, &latency);
            update_max(&mut (*cpu_data).max_thread, &latency);
        } else {
            (*cpu_data).user_count += 1;
            (*cpu_data).cur_user = latency;
            update_min(&mut (*cpu_data).min_user, &latency);
            update_sum(&mut (*cpu_data).sum_user, &latency);
            update_max(&mut (*cpu_data).max_user, &latency);
        }
    }
}

/*
 * timerlat_top_handler - this is the handler for timerlat tracer events
 */
unsafe extern "C" fn timerlat_top_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    context: *mut c_void,
) -> c_int {
    unsafe {
        let trace = context as *mut trace_instance;
        let mut latency: c_ulonglong = 0;
        let mut thread: c_ulonglong = 0;
        let cpu = (*record).cpu;

        /* container_of(trace, struct osnoise_tool, trace) */
        let top = trace as *mut osnoise_tool;

        if !(*(*top).params).aa_only {
            tep_get_field_val(s, event, c"context".as_ptr(), record, &mut thread, 1);
            tep_get_field_val(
                s,
                event,
                c"timer_latency".as_ptr(),
                record,
                &mut latency,
                1,
            );

            timerlat_top_update(top, cpu, thread, latency);
        }

        0
    }
}

/*
 * timerlat_top_bpf_pull_data - copy data from BPF maps into userspace
 */
unsafe extern "C" fn timerlat_top_bpf_pull_data(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let data = (*tool).data as *mut timerlat_top_data;
        let mut i: c_int;
        let mut err: c_int;
        let mut value_irq = vec![0 as c_longlong; nr_cpus as usize];
        let mut value_thread = vec![0 as c_longlong; nr_cpus as usize];
        let mut value_user = vec![0 as c_longlong; nr_cpus as usize];

        /* Pull summary */
        err = timerlat_bpf_get_summary_value(
            SUMMARY_CURRENT,
            value_irq.as_mut_ptr(),
            value_thread.as_mut_ptr(),
            value_user.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0;
        while i < nr_cpus {
            (*(*data).cpu_data.add(i as usize)).cur_irq = value_irq[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).cur_thread =
                value_thread[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).cur_user = value_user[i as usize] as c_ulonglong;
            i += 1;
        }

        err = timerlat_bpf_get_summary_value(
            SUMMARY_COUNT,
            value_irq.as_mut_ptr(),
            value_thread.as_mut_ptr(),
            value_user.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0;
        while i < nr_cpus {
            (*(*data).cpu_data.add(i as usize)).irq_count = value_irq[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).thread_count =
                value_thread[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).user_count = value_user[i as usize] as c_ulonglong;
            i += 1;
        }

        err = timerlat_bpf_get_summary_value(
            SUMMARY_MIN,
            value_irq.as_mut_ptr(),
            value_thread.as_mut_ptr(),
            value_user.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0;
        while i < nr_cpus {
            (*(*data).cpu_data.add(i as usize)).min_irq = value_irq[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).min_thread =
                value_thread[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).min_user = value_user[i as usize] as c_ulonglong;
            i += 1;
        }

        err = timerlat_bpf_get_summary_value(
            SUMMARY_MAX,
            value_irq.as_mut_ptr(),
            value_thread.as_mut_ptr(),
            value_user.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0;
        while i < nr_cpus {
            (*(*data).cpu_data.add(i as usize)).max_irq = value_irq[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).max_thread =
                value_thread[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).max_user = value_user[i as usize] as c_ulonglong;
            i += 1;
        }

        err = timerlat_bpf_get_summary_value(
            SUMMARY_SUM,
            value_irq.as_mut_ptr(),
            value_thread.as_mut_ptr(),
            value_user.as_mut_ptr(),
        );
        if err != 0 {
            return err;
        }
        i = 0;
        while i < nr_cpus {
            (*(*data).cpu_data.add(i as usize)).sum_irq = value_irq[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).sum_thread =
                value_thread[i as usize] as c_ulonglong;
            (*(*data).cpu_data.add(i as usize)).sum_user = value_user[i as usize] as c_ulonglong;
            i += 1;
        }

        0
    }
}

/*
 * timerlat_top_header - print the header of the tool output
 */
unsafe extern "C" fn timerlat_top_header(params: *mut timerlat_params, top: *mut osnoise_tool) {
    unsafe {
        let s = (*top).trace.seq;
        let pretty = (*params).common.pretty_output;
        let mut duration = [0 as c_char; 26];

        get_duration((*top).start_time, duration.as_mut_ptr(), duration.len());

        if pretty {
            trace_seq_printf(s, c"\x1b[2;37;40m".as_ptr());
        }

        trace_seq_printf(
            s,
            c"                                     Timer Latency                                              ".as_ptr(),
        );
        if (*params).common.user_data {
            trace_seq_printf(s, c"                                         ".as_ptr());
        }

        if pretty {
            trace_seq_printf(s, c"\x1b[0;0;0m".as_ptr());
        }
        trace_seq_printf(s, c"\n".as_ptr());

        trace_seq_printf(
            s,
            c"%-6s   |          IRQ Timer Latency (%s)        |         Thread Timer Latency (%s)".as_ptr(),
            duration.as_ptr(),
            if (*params).common.output_divisor == 1 { c"ns".as_ptr() } else { c"us".as_ptr() },
            if (*params).common.output_divisor == 1 { c"ns".as_ptr() } else { c"us".as_ptr() },
        );

        if (*params).common.user_data {
            trace_seq_printf(
                s,
                c"      |    Ret user Timer Latency (%s)".as_ptr(),
                if (*params).common.output_divisor == 1 { c"ns".as_ptr() } else { c"us".as_ptr() },
            );
        }

        trace_seq_printf(s, c"\n".as_ptr());
        if pretty {
            trace_seq_printf(s, c"\x1b[2;30;47m".as_ptr());
        }

        trace_seq_printf(
            s,
            c"CPU COUNT      |      cur       min       avg       max |      cur       min       avg       max".as_ptr(),
        );
        if (*params).common.user_data {
            trace_seq_printf(s, c" |      cur       min       avg       max".as_ptr());
        }

        if pretty {
            trace_seq_printf(s, c"\x1b[0;0;0m".as_ptr());
        }
        trace_seq_printf(s, c"\n".as_ptr());
    }
}

static NO_VALUE: *const c_char = c"        -".as_ptr();

/*
 * timerlat_top_print - prints the output of a given CPU
 */
unsafe extern "C" fn timerlat_top_print(top: *mut osnoise_tool, cpu: c_int) {
    unsafe {
        let params = to_timerlat_params((*top).params);
        let data = (*top).data as *mut timerlat_top_data;
        let cpu_data = (*data).cpu_data.add(cpu as usize);
        let s = (*top).trace.seq;

        /*
         * Skip if no data is available: is this cpu offline?
         */
        if (*cpu_data).irq_count == 0 && (*cpu_data).thread_count == 0 {
            return;
        }

        /*
         * Unless trace is being lost, IRQ counter is always the max.
         */
        trace_seq_printf(s, c"%3d #%-9llu |".as_ptr(), cpu, (*cpu_data).irq_count);

        if (*cpu_data).irq_count == 0 {
            trace_seq_printf(s, c"%s %s %s %s |".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).cur_irq);
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).min_irq);
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).sum_irq / (*cpu_data).irq_count);
            trace_seq_printf(s, c"%9llu |".as_ptr(), (*cpu_data).max_irq);
        }

        if (*cpu_data).thread_count == 0 {
            trace_seq_printf(s, c"%s %s %s %s".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).cur_thread);
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).min_thread);
            trace_seq_printf(
                s,
                c"%9llu ".as_ptr(),
                (*cpu_data).sum_thread / (*cpu_data).thread_count,
            );
            trace_seq_printf(s, c"%9llu".as_ptr(), (*cpu_data).max_thread);
        }

        if !(*params).common.user_data {
            trace_seq_printf(s, c"\n".as_ptr());
            return;
        }

        trace_seq_printf(s, c" |".as_ptr());

        if (*cpu_data).user_count == 0 {
            trace_seq_printf(s, c"%s %s %s %s\n".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).cur_user);
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*cpu_data).min_user);
            trace_seq_printf(
                s,
                c"%9llu ".as_ptr(),
                (*cpu_data).sum_user / (*cpu_data).user_count,
            );
            trace_seq_printf(s, c"%9llu\n".as_ptr(), (*cpu_data).max_user);
        }
    }
}

/*
 * timerlat_top_print_sum - prints the summary output
 */
unsafe extern "C" fn timerlat_top_print_sum(
    top: *mut osnoise_tool,
    summary: *mut timerlat_top_cpu,
) {
    unsafe {
        let split = c"----------------------------------------".as_ptr();
        let params = to_timerlat_params((*top).params);
        let mut count = (*summary).irq_count;
        let s = (*top).trace.seq;
        let mut e: c_int = 0;

        /*
         * Skip if no data is available: is this cpu offline?
         */
        if (*summary).irq_count == 0 && (*summary).thread_count == 0 {
            return;
        }

        while count > 999999 {
            e += 1;
            count /= 10;
        }

        trace_seq_printf(s, c"%.*s|%.*s|%.*s".as_ptr(), 15, split, 40, split, 39, split);
        if (*params).common.user_data {
            trace_seq_printf(s, c"-|%.*s".as_ptr(), 39, split);
        }
        trace_seq_printf(s, c"\n".as_ptr());

        trace_seq_printf(s, c"ALL #%-6llu e%d |".as_ptr(), count, e);

        if (*summary).irq_count == 0 {
            trace_seq_printf(s, c"          %s %s %s |".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"          ".as_ptr());
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*summary).min_irq);
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*summary).sum_irq / (*summary).irq_count);
            trace_seq_printf(s, c"%9llu |".as_ptr(), (*summary).max_irq);
        }

        if (*summary).thread_count == 0 {
            trace_seq_printf(s, c"%s %s %s %s".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"          ".as_ptr());
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*summary).min_thread);
            trace_seq_printf(
                s,
                c"%9llu ".as_ptr(),
                (*summary).sum_thread / (*summary).thread_count,
            );
            trace_seq_printf(s, c"%9llu".as_ptr(), (*summary).max_thread);
        }

        if !(*params).common.user_data {
            trace_seq_printf(s, c"\n".as_ptr());
            return;
        }

        trace_seq_printf(s, c" |".as_ptr());

        if (*summary).user_count == 0 {
            trace_seq_printf(s, c"          %s %s %s |".as_ptr(), NO_VALUE, NO_VALUE, NO_VALUE);
        } else {
            trace_seq_printf(s, c"          ".as_ptr());
            trace_seq_printf(s, c"%9llu ".as_ptr(), (*summary).min_user);
            trace_seq_printf(
                s,
                c"%9llu ".as_ptr(),
                (*summary).sum_user / (*summary).user_count,
            );
            trace_seq_printf(s, c"%9llu\n".as_ptr(), (*summary).max_user);
        }
    }
}

/*
 * clear_terminal - clears the output terminal
 */
unsafe extern "C" fn clear_terminal(seq: *mut trace_seq) {
    unsafe {
        if !config_debug {
            trace_seq_printf(seq, c"\x1bc".as_ptr());
        }
    }
}

/*
 * timerlat_print_stats - print data for all cpus
 */
unsafe extern "C" fn timerlat_print_stats(top: *mut osnoise_tool) {
    unsafe {
        let params = to_timerlat_params((*top).params);
        let trace = &mut (*top).trace as *mut trace_instance;
        let mut summary: timerlat_top_cpu = mem::zeroed();
        let mut i: c_int;

        if (*params).common.aa_only {
            return;
        }

        if !(*params).common.quiet {
            clear_terminal((*trace).seq);
        }

        timerlat_top_reset_sum(&mut summary);

        timerlat_top_header(params, top);

        /* for_each_monitored_cpu(i, &params->common) */
        i = 0;
        while i < nr_cpus {
            timerlat_top_print(top, i);
            timerlat_top_update_sum(top, i, &mut summary);
            i += 1;
        }

        timerlat_top_print_sum(top, &mut summary);

        trace_seq_do_printf((*trace).seq);
        trace_seq_reset((*trace).seq);
        osnoise_report_missed_events(top);
    }
}

/*
 * timerlat_top_apply_config - apply the top configs to the initialized tool
 */
unsafe extern "C" fn timerlat_top_apply_config(top: *mut osnoise_tool) -> c_int {
    unsafe {
        let params = to_timerlat_params((*top).params);
        let retval: c_int;

        retval = timerlat_apply_config(top, params);
        if retval != 0 {
            return -1;
        }

        if isatty(STDOUT_FILENO) != 0 && !(*params).common.quiet {
            (*params).common.pretty_output = true;
        }

        0
    }
}

/*
 * timerlat_init_top - initialize a timerlat top tool with parameters
 */
unsafe extern "C" fn timerlat_init_top(_params: *mut common_params) -> *mut osnoise_tool {
    unsafe {
        let top: *mut osnoise_tool;

        top = osnoise_init_tool(c"timerlat_top".as_ptr());
        if top.is_null() {
            return ptr::null_mut();
        }

        (*top).data = timerlat_alloc_top() as *mut c_void;
        if (*top).data.is_null() {
            osnoise_destroy_tool(top);
            return ptr::null_mut();
        }

        tep_register_event_handler(
            (*top).trace.tep,
            -1,
            c"ftrace".as_ptr(),
            c"timerlat".as_ptr(),
            Some(timerlat_top_handler),
            top as *mut c_void,
        );

        top
    }
}

/*
 * timerlat_top_bpf_main_loop - main loop to process events (BPF variant)
 */
unsafe extern "C" fn timerlat_top_bpf_main_loop(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params = (*tool).params;
        let mut retval: c_int;
        let mut wait_retval: c_int;

        if (*params).aa_only {
            /* Auto-analysis only, just wait for stop tracing */
            timerlat_bpf_wait(-1);
            return 0;
        }

        /* Pull and display data in a loop */
        while !stop_tracing {
            wait_retval = timerlat_bpf_wait(if (*params).quiet {
                -1
            } else {
                (*params).sleep_time
            });

            retval = timerlat_top_bpf_pull_data(tool);
            if retval != 0 {
                err_msg(c"Error pulling BPF data\n".as_ptr());
                return retval;
            }

            if !(*params).quiet {
                timerlat_print_stats(tool);
            }

            if wait_retval > 0 {
                /* Stopping requested by tracer */
                retval = common_threshold_handler(tool);
                if retval != 0 {
                    return retval;
                }

                if !should_continue_tracing((*tool).params) {
                    break;
                }

                if timerlat_bpf_restart_tracing() != 0 {
                    err_msg(c"Error restarting BPF trace\n".as_ptr());
                    return -1;
                }
            }

            /* is there still any user-threads ? */
            if (*params).user_workload {
                if (*params).user.stopped_running {
                    debug_msg(c"timerlat user space threads stopped!\n".as_ptr());
                    break;
                }
            }
        }

        0
    }
}

unsafe extern "C" fn timerlat_top_main_loop(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params = to_timerlat_params((*tool).params);
        let retval: c_int;

        if (*params).mode == TRACING_MODE_TRACEFS {
            retval = top_main_loop(tool);
        } else {
            retval = timerlat_top_bpf_main_loop(tool);
            timerlat_bpf_detach();
        }

        retval
    }
}

#[unsafe(no_mangle)]
pub static mut timerlat_top_ops: tool_ops = tool_ops {
    tracer: c"timerlat".as_ptr(),
    comm_prefix: c"timerlat/".as_ptr(),
    parse_args: Some(timerlat_top_parse_args),
    init_tool: Some(timerlat_init_top),
    apply_config: Some(timerlat_top_apply_config),
    enable: Some(timerlat_enable),
    main: Some(timerlat_top_main_loop),
    print_stats: Some(timerlat_print_stats),
    analyze: Some(timerlat_analyze),
    free: Some(timerlat_free_top_tool),
};
