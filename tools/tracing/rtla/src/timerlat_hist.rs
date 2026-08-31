// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct trace_seq {
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
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_instance {
    pub seq: *mut trace_seq,
    pub tep: *mut tep_handle,
}

#[repr(C)]
pub struct hist_params {
    pub no_header: bool,
    pub no_index: bool,
    pub no_irq: bool,
    pub no_thread: bool,
    pub no_summary: bool,
    pub with_zeros: bool,
    pub entries: c_int,
    pub bucket_size: c_int,
}

#[repr(C)]
pub struct common_params {
    pub hist: hist_params,
    pub output_divisor: u64,
    pub user_data: bool,
}

#[repr(C)]
pub struct timerlat_params {
    pub common: common_params,
    pub mode: c_int,
}

#[repr(C)]
pub struct osnoise_tool {
    pub data: *mut timerlat_hist_data,
    pub params: *mut c_void,
    pub trace: trace_instance,
    pub start_time: libc::time_t,
}

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

#[repr(C)]
pub struct timerlat_hist_cpu {
    pub irq: *mut c_int,
    pub thread: *mut c_int,
    pub user: *mut c_int,

    pub irq_count: u64,
    pub thread_count: u64,
    pub user_count: u64,

    pub min_irq: u64,
    pub sum_irq: u64,
    pub max_irq: u64,

    pub min_thread: u64,
    pub sum_thread: u64,
    pub max_thread: u64,

    pub min_user: u64,
    pub sum_user: u64,
    pub max_user: u64,
}

#[repr(C)]
pub struct timerlat_hist_data {
    pub hist: *mut timerlat_hist_cpu,
    pub entries: c_int,
    pub bucket_size: c_int,
}

const TRACING_MODE_TRACEFS: c_int = 0;
const SUMMARY_COUNT: c_int = 0;
const SUMMARY_MIN: c_int = 1;
const SUMMARY_MAX: c_int = 2;
const SUMMARY_SUM: c_int = 3;
const SUMMARY_OVERFLOW: c_int = 4;

unsafe extern "C" {
    static nr_cpus: c_int;
    static mut stop_tracing: bool;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn to_timerlat_params(params: *mut c_void) -> *mut timerlat_params;
    fn timerlat_free(tool: *mut osnoise_tool);
    fn timerlat_apply_config(tool: *mut osnoise_tool, params: *mut timerlat_params) -> c_int;
    fn osnoise_init_tool(name: *const c_char) -> *mut osnoise_tool;
    fn osnoise_destroy_tool(tool: *mut osnoise_tool);
    fn tep_register_event_handler(
        tep: *mut tep_handle,
        id: c_int,
        sys_name: *const c_char,
        event_name: *const c_char,
        handler: Option<
            unsafe extern "C" fn(*mut trace_seq, *mut tep_record, *mut tep_event, *mut c_void) -> c_int,
        >,
        context: *mut c_void,
    ) -> c_int;
    fn tep_get_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut u64,
        err: c_int,
    ) -> c_int;
    fn timerlat_bpf_get_hist_value(
        i: c_int,
        value_irq: *mut i64,
        value_thread: *mut i64,
        value_user: *mut i64,
    ) -> c_int;
    fn timerlat_bpf_get_summary_value(
        summary: c_int,
        value_irq: *mut i64,
        value_thread: *mut i64,
        value_user: *mut i64,
    ) -> c_int;
    fn get_duration(start_time: libc::time_t, duration: *mut c_char, size: usize);
    fn trace_seq_printf(seq: *mut trace_seq, fmt: *const c_char, ...);
    fn trace_seq_do_printf(seq: *mut trace_seq);
    fn trace_seq_reset(seq: *mut trace_seq);
    fn update_min(current: *mut u64, val: *const u64);
    fn update_sum(current: *mut u64, val: *const u64);
    fn update_max(current: *mut u64, val: *const u64);
    fn timerlat_bpf_wait(timeout: c_int);
    fn common_threshold_handler(tool: *mut osnoise_tool) -> c_int;
    fn should_continue_tracing(params: *mut c_void) -> bool;
    fn timerlat_bpf_restart_tracing() -> c_int;
    fn err_msg(fmt: *const c_char, ...);
    fn timerlat_bpf_detach();
    fn hist_main_loop(tool: *mut osnoise_tool) -> c_int;
    fn osnoise_report_missed_events(tool: *mut osnoise_tool);
    fn timerlat_hist_parse_args(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn timerlat_enable(tool: *mut osnoise_tool) -> c_int;
    fn timerlat_analyze(tool: *mut osnoise_tool) -> c_int;

    /* C macro dependency: for_each_monitored_cpu(cpu, &params->common). */
    fn cpu_is_monitored(cpu: c_int, params: *mut common_params) -> bool;
}

unsafe fn timerlat_free_histogram(data: *mut timerlat_hist_data) {
    let mut cpu: c_int;

    /* one histogram for IRQ and one for thread, per CPU */
    cpu = 0;
    while cpu < nr_cpus {
        let hist = (*data).hist.add(cpu as usize);
        if !(*hist).irq.is_null() {
            free((*hist).irq as *mut c_void);
        }

        if !(*hist).thread.is_null() {
            free((*hist).thread as *mut c_void);
        }

        if !(*hist).user.is_null() {
            free((*hist).user as *mut c_void);
        }

        cpu += 1;
    }

    /* one set of histograms per CPU */
    if !(*data).hist.is_null() {
        free((*data).hist as *mut c_void);
    }
}

unsafe extern "C" fn timerlat_free_histogram_tool(tool: *mut osnoise_tool) {
    timerlat_free_histogram((*tool).data);
    timerlat_free(tool);
}

unsafe fn timerlat_alloc_histogram(entries: c_int, bucket_size: c_int) -> *mut timerlat_hist_data {
    let data: *mut timerlat_hist_data;
    let mut cpu: c_int;

    data = calloc(1, mem::size_of::<timerlat_hist_data>()) as *mut timerlat_hist_data;
    if data.is_null() {
        return ptr::null_mut();
    }

    (*data).entries = entries;
    (*data).bucket_size = bucket_size;

    /* one set of histograms per CPU */
    (*data).hist = calloc(1, mem::size_of::<timerlat_hist_cpu>() * nr_cpus as usize) as *mut timerlat_hist_cpu;
    if (*data).hist.is_null() {
        timerlat_free_histogram(data);
        return ptr::null_mut();
    }

    /* one histogram for IRQ and one for thread, per cpu */
    cpu = 0;
    while cpu < nr_cpus {
        let hist = (*data).hist.add(cpu as usize);
        (*hist).irq = calloc(1, mem::size_of::<c_int>() * (entries + 1) as usize) as *mut c_int;
        if (*hist).irq.is_null() {
            timerlat_free_histogram(data);
            return ptr::null_mut();
        }

        (*hist).thread = calloc(1, mem::size_of::<c_int>() * (entries + 1) as usize) as *mut c_int;
        if (*hist).thread.is_null() {
            timerlat_free_histogram(data);
            return ptr::null_mut();
        }

        (*hist).user = calloc(1, mem::size_of::<c_int>() * (entries + 1) as usize) as *mut c_int;
        if (*hist).user.is_null() {
            timerlat_free_histogram(data);
            return ptr::null_mut();
        }
        cpu += 1;
    }

    /* set the min to max */
    cpu = 0;
    while cpu < nr_cpus {
        let hist = (*data).hist.add(cpu as usize);
        (*hist).min_irq = !0;
        (*hist).min_thread = !0;
        (*hist).min_user = !0;
        cpu += 1;
    }

    data
}

unsafe fn timerlat_hist_update(
    tool: *mut osnoise_tool,
    cpu: c_int,
    context: u64,
    mut latency: u64,
) {
    let params = to_timerlat_params((*tool).params);
    let data = (*tool).data;
    let entries = (*data).entries;
    let bucket: c_int;
    let hist: *mut c_int;

    if (*params).common.output_divisor != 0 {
        latency = latency / (*params).common.output_divisor;
    }

    bucket = (latency / (*data).bucket_size as u64) as c_int;

    if context == 0 {
        hist = (*(*data).hist.add(cpu as usize)).irq;
        (*(*data).hist.add(cpu as usize)).irq_count += 1;
        update_min(&mut (*(*data).hist.add(cpu as usize)).min_irq, &latency);
        update_sum(&mut (*(*data).hist.add(cpu as usize)).sum_irq, &latency);
        update_max(&mut (*(*data).hist.add(cpu as usize)).max_irq, &latency);
    } else if context == 1 {
        hist = (*(*data).hist.add(cpu as usize)).thread;
        (*(*data).hist.add(cpu as usize)).thread_count += 1;
        update_min(&mut (*(*data).hist.add(cpu as usize)).min_thread, &latency);
        update_sum(&mut (*(*data).hist.add(cpu as usize)).sum_thread, &latency);
        update_max(&mut (*(*data).hist.add(cpu as usize)).max_thread, &latency);
    } else {
        /* user */
        hist = (*(*data).hist.add(cpu as usize)).user;
        (*(*data).hist.add(cpu as usize)).user_count += 1;
        update_min(&mut (*(*data).hist.add(cpu as usize)).min_user, &latency);
        update_sum(&mut (*(*data).hist.add(cpu as usize)).sum_user, &latency);
        update_max(&mut (*(*data).hist.add(cpu as usize)).max_user, &latency);
    }

    if bucket < entries {
        *hist.add(bucket as usize) += 1;
    } else {
        *hist.add(entries as usize) += 1;
    }
}

unsafe extern "C" fn timerlat_hist_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    data: *mut c_void,
) -> c_int {
    let trace = data as *mut trace_instance;
    let mut context: u64 = 0;
    let mut latency: u64 = 0;
    let tool = (trace as *mut u8).sub(mem::offset_of!(osnoise_tool, trace)) as *mut osnoise_tool;
    let cpu = (*record).cpu;

    tep_get_field_val(s, event, c"context".as_ptr(), record, &mut context, 1);
    tep_get_field_val(s, event, c"timer_latency".as_ptr(), record, &mut latency, 1);

    timerlat_hist_update(tool, cpu, context, latency);

    0
}

unsafe fn timerlat_hist_bpf_pull_data(tool: *mut osnoise_tool) -> c_int {
    let data = (*tool).data;
    let mut i: c_int;
    let mut j: c_int;
    let mut err: c_int;
    let mut value_irq = vec![0i64; nr_cpus as usize];
    let mut value_thread = vec![0i64; nr_cpus as usize];
    let mut value_user = vec![0i64; nr_cpus as usize];

    /* Pull histogram */
    i = 0;
    while i < (*data).entries {
        err = timerlat_bpf_get_hist_value(i, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
        if err != 0 {
            return err;
        }
        j = 0;
        while j < nr_cpus {
            *(*(*data).hist.add(j as usize)).irq.add(i as usize) = value_irq[j as usize] as c_int;
            *(*(*data).hist.add(j as usize)).thread.add(i as usize) = value_thread[j as usize] as c_int;
            *(*(*data).hist.add(j as usize)).user.add(i as usize) = value_user[j as usize] as c_int;
            j += 1;
        }
        i += 1;
    }

    /* Pull summary */
    err = timerlat_bpf_get_summary_value(SUMMARY_COUNT, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
    if err != 0 {
        return err;
    }
    i = 0;
    while i < nr_cpus {
        (*(*data).hist.add(i as usize)).irq_count = value_irq[i as usize] as u64;
        (*(*data).hist.add(i as usize)).thread_count = value_thread[i as usize] as u64;
        (*(*data).hist.add(i as usize)).user_count = value_user[i as usize] as u64;
        i += 1;
    }

    err = timerlat_bpf_get_summary_value(SUMMARY_MIN, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
    if err != 0 {
        return err;
    }
    i = 0;
    while i < nr_cpus {
        (*(*data).hist.add(i as usize)).min_irq = value_irq[i as usize] as u64;
        (*(*data).hist.add(i as usize)).min_thread = value_thread[i as usize] as u64;
        (*(*data).hist.add(i as usize)).min_user = value_user[i as usize] as u64;
        i += 1;
    }

    err = timerlat_bpf_get_summary_value(SUMMARY_MAX, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
    if err != 0 {
        return err;
    }
    i = 0;
    while i < nr_cpus {
        (*(*data).hist.add(i as usize)).max_irq = value_irq[i as usize] as u64;
        (*(*data).hist.add(i as usize)).max_thread = value_thread[i as usize] as u64;
        (*(*data).hist.add(i as usize)).max_user = value_user[i as usize] as u64;
        i += 1;
    }

    err = timerlat_bpf_get_summary_value(SUMMARY_SUM, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
    if err != 0 {
        return err;
    }
    i = 0;
    while i < nr_cpus {
        (*(*data).hist.add(i as usize)).sum_irq = value_irq[i as usize] as u64;
        (*(*data).hist.add(i as usize)).sum_thread = value_thread[i as usize] as u64;
        (*(*data).hist.add(i as usize)).sum_user = value_user[i as usize] as u64;
        i += 1;
    }

    err = timerlat_bpf_get_summary_value(SUMMARY_OVERFLOW, value_irq.as_mut_ptr(), value_thread.as_mut_ptr(), value_user.as_mut_ptr());
    if err != 0 {
        return err;
    }
    i = 0;
    while i < nr_cpus {
        *(*(*data).hist.add(i as usize)).irq.add((*data).entries as usize) = value_irq[i as usize] as c_int;
        *(*(*data).hist.add(i as usize)).thread.add((*data).entries as usize) = value_thread[i as usize] as c_int;
        *(*(*data).hist.add(i as usize)).user.add((*data).entries as usize) = value_user[i as usize] as c_int;
        i += 1;
    }

    0
}

unsafe fn for_each_monitored_cpu_body<F: FnMut(c_int)>(common: *mut common_params, mut f: F) {
    let mut cpu = 0;
    while cpu < nr_cpus {
        if cpu_is_monitored(cpu, common) {
            f(cpu);
        }
        cpu += 1;
    }
}

unsafe fn timerlat_hist_header(tool: *mut osnoise_tool) {
    let params = to_timerlat_params((*tool).params);
    let data = (*tool).data;
    let s = (*tool).trace.seq;
    let mut duration = [0 as c_char; 26];

    if (*params).common.hist.no_header {
        return;
    }

    get_duration((*tool).start_time, duration.as_mut_ptr(), duration.len());
    trace_seq_printf(s, c"# RTLA timerlat histogram\n".as_ptr());
    trace_seq_printf(
        s,
        c"# Time unit is %s (%s)\n".as_ptr(),
        if (*params).common.output_divisor == 1 { c"nanoseconds".as_ptr() } else { c"microseconds".as_ptr() },
        if (*params).common.output_divisor == 1 { c"ns".as_ptr() } else { c"us".as_ptr() },
    );

    trace_seq_printf(s, c"# Duration: %s\n".as_ptr(), duration.as_ptr());

    if !(*params).common.hist.no_index {
        trace_seq_printf(s, c"Index".as_ptr());
    }

    for_each_monitored_cpu_body(&mut (*params).common, |cpu| {
        unsafe {
            if (*(*data).hist.add(cpu as usize)).irq_count == 0 && (*(*data).hist.add(cpu as usize)).thread_count == 0 {
                return;
            }

            if !(*params).common.hist.no_irq {
                trace_seq_printf(s, c"   IRQ-%03d".as_ptr(), cpu);
            }

            if !(*params).common.hist.no_thread {
                trace_seq_printf(s, c"   Thr-%03d".as_ptr(), cpu);
            }

            if (*params).common.user_data {
                trace_seq_printf(s, c"   Usr-%03d".as_ptr(), cpu);
            }
        }
    });
    trace_seq_printf(s, c"\n".as_ptr());

    trace_seq_do_printf(s);
    trace_seq_reset(s);
}

unsafe fn format_summary_value(seq: *mut trace_seq, count: c_int, val: u64, avg: bool) {
    if count != 0 {
        trace_seq_printf(seq, c"%9llu ".as_ptr(), if avg { val / count as u64 } else { val });
    } else {
        trace_seq_printf(seq, c"%9c ".as_ptr(), '-' as c_int);
    }
}

unsafe fn timerlat_print_summary(params: *mut timerlat_params, trace: *mut trace_instance, data: *mut timerlat_hist_data) {
    if (*params).common.hist.no_summary {
        return;
    }

    if !(*params).common.hist.no_index {
        trace_seq_printf((*trace).seq, c"count:".as_ptr());
    }

    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let h = (*data).hist.add(cpu as usize);
        if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
        if !(*params).common.hist.no_irq { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), (*h).irq_count); }
        if !(*params).common.hist.no_thread { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), (*h).thread_count); }
        if (*params).common.user_data { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), (*h).user_count); }
    });
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"min:  ".as_ptr()); }
    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let h = (*data).hist.add(cpu as usize);
        if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
        if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, (*h).irq_count as c_int, (*h).min_irq, false); }
        if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, (*h).thread_count as c_int, (*h).min_thread, false); }
        if (*params).common.user_data { format_summary_value((*trace).seq, (*h).user_count as c_int, (*h).min_user, false); }
    });
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"avg:  ".as_ptr()); }
    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let h = (*data).hist.add(cpu as usize);
        if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
        if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, (*h).irq_count as c_int, (*h).sum_irq, true); }
        if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, (*h).thread_count as c_int, (*h).sum_thread, true); }
        if (*params).common.user_data { format_summary_value((*trace).seq, (*h).user_count as c_int, (*h).sum_user, true); }
    });
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"max:  ".as_ptr()); }
    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let h = (*data).hist.add(cpu as usize);
        if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
        if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, (*h).irq_count as c_int, (*h).max_irq, false); }
        if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, (*h).thread_count as c_int, (*h).max_thread, false); }
        if (*params).common.user_data { format_summary_value((*trace).seq, (*h).user_count as c_int, (*h).max_user, false); }
    });
    trace_seq_printf((*trace).seq, c"\n".as_ptr());
    trace_seq_do_printf((*trace).seq);
    trace_seq_reset((*trace).seq);
}

unsafe fn timerlat_print_stats_all(params: *mut timerlat_params, trace: *mut trace_instance, data: *mut timerlat_hist_data) {
    let mut sum: timerlat_hist_cpu = mem::zeroed();

    if (*params).common.hist.no_summary {
        return;
    }

    sum.min_irq = !0;
    sum.min_thread = !0;
    sum.min_user = !0;

    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let cpu_data = (*data).hist.add(cpu as usize);
        if (*cpu_data).irq_count == 0 && (*cpu_data).thread_count == 0 { return; }

        sum.irq_count += (*cpu_data).irq_count;
        update_min(&mut sum.min_irq, &(*cpu_data).min_irq);
        update_sum(&mut sum.sum_irq, &(*cpu_data).sum_irq);
        update_max(&mut sum.max_irq, &(*cpu_data).max_irq);

        sum.thread_count += (*cpu_data).thread_count;
        update_min(&mut sum.min_thread, &(*cpu_data).min_thread);
        update_sum(&mut sum.sum_thread, &(*cpu_data).sum_thread);
        update_max(&mut sum.max_thread, &(*cpu_data).max_thread);

        sum.user_count += (*cpu_data).user_count;
        update_min(&mut sum.min_user, &(*cpu_data).min_user);
        update_sum(&mut sum.sum_user, &(*cpu_data).sum_user);
        update_max(&mut sum.max_user, &(*cpu_data).max_user);
    });

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"ALL:  ".as_ptr()); }
    if !(*params).common.hist.no_irq { trace_seq_printf((*trace).seq, c"      IRQ".as_ptr()); }
    if !(*params).common.hist.no_thread { trace_seq_printf((*trace).seq, c"       Thr".as_ptr()); }
    if (*params).common.user_data { trace_seq_printf((*trace).seq, c"       Usr".as_ptr()); }
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"count:".as_ptr()); }
    if !(*params).common.hist.no_irq { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), sum.irq_count); }
    if !(*params).common.hist.no_thread { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), sum.thread_count); }
    if (*params).common.user_data { trace_seq_printf((*trace).seq, c"%9llu ".as_ptr(), sum.user_count); }
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"min:  ".as_ptr()); }
    if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, sum.irq_count as c_int, sum.min_irq, false); }
    if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, sum.thread_count as c_int, sum.min_thread, false); }
    if (*params).common.user_data { format_summary_value((*trace).seq, sum.user_count as c_int, sum.min_user, false); }
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"avg:  ".as_ptr()); }
    if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, sum.irq_count as c_int, sum.sum_irq, true); }
    if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, sum.thread_count as c_int, sum.sum_thread, true); }
    if (*params).common.user_data { format_summary_value((*trace).seq, sum.user_count as c_int, sum.sum_user, true); }
    trace_seq_printf((*trace).seq, c"\n".as_ptr());

    if !(*params).common.hist.no_index { trace_seq_printf((*trace).seq, c"max:  ".as_ptr()); }
    if !(*params).common.hist.no_irq { format_summary_value((*trace).seq, sum.irq_count as c_int, sum.max_irq, false); }
    if !(*params).common.hist.no_thread { format_summary_value((*trace).seq, sum.thread_count as c_int, sum.max_thread, false); }
    if (*params).common.user_data { format_summary_value((*trace).seq, sum.user_count as c_int, sum.max_user, false); }
    trace_seq_printf((*trace).seq, c"\n".as_ptr());
    trace_seq_do_printf((*trace).seq);
    trace_seq_reset((*trace).seq);
}

unsafe extern "C" fn timerlat_print_stats(tool: *mut osnoise_tool) {
    let params = to_timerlat_params((*tool).params);
    let data = (*tool).data;
    let trace = &mut (*tool).trace as *mut trace_instance;
    let mut bucket: c_int;
    let mut total: c_int;

    timerlat_hist_header(tool);

    bucket = 0;
    while bucket < (*data).entries {
        total = 0;

        if !(*params).common.hist.no_index {
            trace_seq_printf((*trace).seq, c"%-6d".as_ptr(), bucket * (*data).bucket_size);
        }

        for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
            let h = (*data).hist.add(cpu as usize);
            if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
            if !(*params).common.hist.no_irq {
                total += *(*h).irq.add(bucket as usize);
                trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).irq.add(bucket as usize));
            }
            if !(*params).common.hist.no_thread {
                total += *(*h).thread.add(bucket as usize);
                trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).thread.add(bucket as usize));
            }
            if (*params).common.user_data {
                total += *(*h).user.add(bucket as usize);
                trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).user.add(bucket as usize));
            }
        });

        if total == 0 && !(*params).common.hist.with_zeros {
            trace_seq_reset((*trace).seq);
            bucket += 1;
            continue;
        }

        trace_seq_printf((*trace).seq, c"\n".as_ptr());
        trace_seq_do_printf((*trace).seq);
        trace_seq_reset((*trace).seq);
        bucket += 1;
    }

    if !(*params).common.hist.no_index {
        trace_seq_printf((*trace).seq, c"over: ".as_ptr());
    }

    for_each_monitored_cpu_body(&mut (*params).common, |cpu| unsafe {
        let h = (*data).hist.add(cpu as usize);
        if (*h).irq_count == 0 && (*h).thread_count == 0 { return; }
        if !(*params).common.hist.no_irq {
            trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).irq.add((*data).entries as usize));
        }
        if !(*params).common.hist.no_thread {
            trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).thread.add((*data).entries as usize));
        }
        if (*params).common.user_data {
            trace_seq_printf((*trace).seq, c"%9d ".as_ptr(), *(*h).user.add((*data).entries as usize));
        }
    });
    trace_seq_printf((*trace).seq, c"\n".as_ptr());
    trace_seq_do_printf((*trace).seq);
    trace_seq_reset((*trace).seq);

    timerlat_print_summary(params, trace, data);
    timerlat_print_stats_all(params, trace, data);
    osnoise_report_missed_events(tool);
}

unsafe extern "C" fn timerlat_hist_apply_config(tool: *mut osnoise_tool) -> c_int {
    let params = to_timerlat_params((*tool).params);
    let retval: c_int;

    retval = timerlat_apply_config(tool, params);
    if retval != 0 {
        return -1;
    }

    0
}

unsafe extern "C" fn timerlat_init_hist(params: *mut common_params) -> *mut osnoise_tool {
    let tool: *mut osnoise_tool;

    tool = osnoise_init_tool(c"timerlat_hist".as_ptr());
    if tool.is_null() {
        return ptr::null_mut();
    }

    (*tool).data = timerlat_alloc_histogram((*params).hist.entries, (*params).hist.bucket_size);
    if (*tool).data.is_null() {
        osnoise_destroy_tool(tool);
        return ptr::null_mut();
    }

    tep_register_event_handler(
        (*tool).trace.tep,
        -1,
        c"ftrace".as_ptr(),
        c"timerlat".as_ptr(),
        Some(timerlat_hist_handler),
        tool as *mut c_void,
    );

    tool
}

unsafe fn timerlat_hist_bpf_main_loop(tool: *mut osnoise_tool) -> c_int {
    let mut retval: c_int;

    while !stop_tracing {
        timerlat_bpf_wait(-1);

        if !stop_tracing {
            /* Threshold overflow, perform actions on threshold */
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
    }
    timerlat_bpf_detach();

    retval = timerlat_hist_bpf_pull_data(tool);
    if retval != 0 {
        err_msg(c"Error pulling BPF data\n".as_ptr());
    }

    retval
}

unsafe extern "C" fn timerlat_hist_main(tool: *mut osnoise_tool) -> c_int {
    let params = to_timerlat_params((*tool).params);
    let retval: c_int;

    if (*params).mode == TRACING_MODE_TRACEFS {
        retval = hist_main_loop(tool);
    } else {
        retval = timerlat_hist_bpf_main_loop(tool);
    }

    retval
}

#[unsafe(no_mangle)]
pub static mut timerlat_hist_ops: tool_ops = tool_ops {
    tracer: c"timerlat".as_ptr(),
    comm_prefix: c"timerlat/".as_ptr(),
    parse_args: Some(timerlat_hist_parse_args),
    init_tool: Some(timerlat_init_hist),
    apply_config: Some(timerlat_hist_apply_config),
    enable: Some(timerlat_enable),
    main: Some(timerlat_hist_main),
    print_stats: Some(timerlat_print_stats),
    analyze: Some(timerlat_analyze),
    free: Some(timerlat_free_histogram_tool),
};
