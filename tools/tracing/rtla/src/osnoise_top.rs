// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

/* Translated from osnoise_top.c. C include dependencies:
 * stdlib.h, string.h, signal.h, unistd.h, stdio.h, time.h, osnoise.h, cli.h
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct osnoise_top_cpu {
    sum_runtime: u64,
    sum_noise: u64,
    max_noise: u64,
    max_sample: u64,

    hw_count: u64,
    nmi_count: u64,
    irq_count: u64,
    softirq_count: u64,
    thread_count: u64,

    sum_cycles: c_int,
}

#[repr(C)]
struct osnoise_top_data {
    cpu_data: *mut osnoise_top_cpu,
}

#[repr(C)]
struct trace_seq {
    _private: [u8; 0],
}

#[repr(C)]
struct tep_record {
    cpu: c_int,
}

#[repr(C)]
struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
struct trace_instance {
    seq: *mut trace_seq,
    tep: *mut tep_handle,
}

#[repr(C)]
struct common_params {
    quiet: bool,
    pretty_output: bool,
}

#[repr(C)]
struct osnoise_params {
    common: common_params,
    mode: c_int,
}

#[repr(C)]
struct osnoise_tool {
    trace: trace_instance,
    data: *mut c_void,
    params: *mut c_void,
    context: *mut c_void,
    start_time: time_t,
}

#[repr(C)]
struct tool_ops {
    tracer: *const c_char,
    comm_prefix: *const c_char,
    parse_args: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
    init_tool: Option<unsafe extern "C" fn(*mut common_params) -> *mut osnoise_tool>,
    apply_config: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    enable: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    main: Option<unsafe extern "C" fn(*mut osnoise_tool) -> c_int>,
    print_stats: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
    free: Option<unsafe extern "C" fn(*mut osnoise_tool)>,
}

type time_t = i64;

const MODE_OSNOISE: c_int = 0;
const MODE_HWNOISE: c_int = 1;
const STDOUT_FILENO: c_int = 1;

unsafe extern "C" {
    static nr_cpus: c_int;
    static config_debug: bool;

    fn free(ptr: *mut c_void);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn isatty(fd: c_int) -> c_int;

    fn tep_get_field_val(
        s: *mut trace_seq,
        event: *mut tep_event,
        name: *const c_char,
        record: *mut tep_record,
        val: *mut u64,
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

    fn update_sum(sum: *mut u64, val: *mut u64);
    fn update_max(max: *mut u64, val: *mut u64);
    fn to_osnoise_params(params: *mut c_void) -> *mut osnoise_params;
    fn get_duration(start_time: time_t, duration: *mut c_char, size: usize);
    fn trace_seq_printf(s: *mut trace_seq, fmt: *const c_char, ...);
    fn trace_seq_do_printf(s: *mut trace_seq);
    fn trace_seq_reset(s: *mut trace_seq);
    fn osnoise_report_missed_events(tool: *mut osnoise_tool);
    fn osnoise_apply_config(tool: *mut osnoise_tool, params: *mut osnoise_params) -> c_int;
    fn osnoise_set_irq_disable(context: *mut c_void, disable: c_int) -> c_int;
    fn err_msg(fmt: *const c_char, ...);
    fn osnoise_init_tool(name: *const c_char) -> *mut osnoise_tool;
    fn osnoise_destroy_tool(tool: *mut osnoise_tool);
    fn osnoise_top_parse_args(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn osnoise_enable(tool: *mut osnoise_tool) -> c_int;
    fn top_main_loop(tool: *mut osnoise_tool) -> c_int;
}

/*
 * osnoise_free_top - free runtime data
 */
unsafe fn osnoise_free_top(data: *mut osnoise_top_data) {
    unsafe {
        free((*data).cpu_data as *mut c_void);
        free(data as *mut c_void);
    }
}

unsafe extern "C" fn osnoise_free_top_tool(tool: *mut osnoise_tool) {
    unsafe {
        osnoise_free_top((*tool).data as *mut osnoise_top_data);
    }
}

/*
 * osnoise_alloc_histogram - alloc runtime data
 */
unsafe fn osnoise_alloc_top() -> *mut osnoise_top_data {
    let data: *mut osnoise_top_data;

    unsafe {
        data = calloc(1, size_of::<osnoise_top_data>()) as *mut osnoise_top_data;
        if data.is_null() {
            return ptr::null_mut();
        }

        /* one set of histograms per CPU */
        (*data).cpu_data =
            calloc(1, size_of::<osnoise_top_cpu>() * nr_cpus as usize) as *mut osnoise_top_cpu;
        if (*data).cpu_data.is_null() {
            osnoise_free_top(data);
            return ptr::null_mut();
        }
    }

    data
}

/*
 * osnoise_top_handler - this is the handler for osnoise tracer events
 */
unsafe extern "C" fn osnoise_top_handler(
    s: *mut trace_seq,
    record: *mut tep_record,
    event: *mut tep_event,
    context: *mut c_void,
) -> c_int {
    let trace = context as *mut trace_instance;
    let tool: *mut osnoise_tool;
    let mut val: u64 = 0;
    let cpu_data: *mut osnoise_top_cpu;
    let data: *mut osnoise_top_data;
    let cpu: c_int;

    unsafe {
        cpu = (*record).cpu;

        tool = container_of!(trace, osnoise_tool, trace);

        data = (*tool).data as *mut osnoise_top_data;
        cpu_data = (*data).cpu_data.add(cpu as usize);

        (*cpu_data).sum_cycles += 1;

        tep_get_field_val(s, event, c"runtime".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).sum_runtime, &mut val);

        tep_get_field_val(s, event, c"noise".as_ptr(), record, &mut val, 1);
        update_max(&mut (*cpu_data).max_noise, &mut val);
        update_sum(&mut (*cpu_data).sum_noise, &mut val);

        tep_get_field_val(s, event, c"max_sample".as_ptr(), record, &mut val, 1);
        update_max(&mut (*cpu_data).max_sample, &mut val);

        tep_get_field_val(s, event, c"hw_count".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).hw_count, &mut val);

        tep_get_field_val(s, event, c"nmi_count".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).nmi_count, &mut val);

        tep_get_field_val(s, event, c"irq_count".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).irq_count, &mut val);

        tep_get_field_val(s, event, c"softirq_count".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).softirq_count, &mut val);

        tep_get_field_val(s, event, c"thread_count".as_ptr(), record, &mut val, 1);
        update_sum(&mut (*cpu_data).thread_count, &mut val);
    }

    0
}

/*
 * osnoise_top_header - print the header of the tool output
 */
unsafe fn osnoise_top_header(top: *mut osnoise_tool) {
    let params: *mut osnoise_params;
    let s: *mut trace_seq;
    let pretty: bool;
    let mut duration = [0 as c_char; 26];

    unsafe {
        params = to_osnoise_params((*top).params);
        s = (*top).trace.seq;
        pretty = (*params).common.pretty_output;

        get_duration((*top).start_time, duration.as_mut_ptr(), size_of::<[c_char; 26]>());

        if pretty {
            trace_seq_printf(s, c"\x1b[2;37;40m".as_ptr());
        }

        trace_seq_printf(s, c"                                          ".as_ptr());

        if (*params).mode == MODE_OSNOISE {
            trace_seq_printf(s, c"Operating System Noise".as_ptr());
            trace_seq_printf(s, c"                                       ".as_ptr());
        } else if (*params).mode == MODE_HWNOISE {
            trace_seq_printf(s, c"Hardware-related Noise".as_ptr());
        }

        trace_seq_printf(s, c"                                   ".as_ptr());

        if pretty {
            trace_seq_printf(s, c"\x1b[0;0;0m".as_ptr());
        }
        trace_seq_printf(s, c"\n".as_ptr());

        trace_seq_printf(s, c"duration: %9s | time is in us\n".as_ptr(), duration.as_ptr());

        if pretty {
            trace_seq_printf(s, c"\x1b[2;30;47m".as_ptr());
        }

        trace_seq_printf(s, c"CPU Period       Runtime ".as_ptr());
        trace_seq_printf(s, c"       Noise ".as_ptr());
        trace_seq_printf(s, c" % CPU Aval ".as_ptr());
        trace_seq_printf(s, c"  Max Noise   Max Single ".as_ptr());
        trace_seq_printf(s, c"         HW          NMI".as_ptr());

        if (*params).mode != MODE_HWNOISE {
            trace_seq_printf(s, c"          IRQ      Softirq       Thread".as_ptr());
        }

        if pretty {
            trace_seq_printf(s, c"\x1b[0;0;0m".as_ptr());
        }
        trace_seq_printf(s, c"\n".as_ptr());
    }
}

/*
 * clear_terminal - clears the output terminal
 */
unsafe fn clear_terminal(seq: *mut trace_seq) {
    unsafe {
        if !config_debug {
            trace_seq_printf(seq, c"\x1bc".as_ptr());
        }
    }
}

/*
 * osnoise_top_print - prints the output of a given CPU
 */
unsafe fn osnoise_top_print(tool: *mut osnoise_tool, cpu: c_int) {
    let params: *mut osnoise_params;
    let s: *mut trace_seq;
    let cpu_data: *mut osnoise_top_cpu;
    let data: *mut osnoise_top_data;
    let mut percentage: c_int;
    let decimal: c_int;

    unsafe {
        params = to_osnoise_params((*tool).params);
        s = (*tool).trace.seq;

        data = (*tool).data as *mut osnoise_top_data;
        cpu_data = (*data).cpu_data.add(cpu as usize);

        if (*cpu_data).sum_runtime == 0 {
            return;
        }

        percentage = (((*cpu_data).sum_runtime - (*cpu_data).sum_noise) * 10000000
            / (*cpu_data).sum_runtime) as c_int;
        decimal = percentage % 100000;
        percentage = percentage / 100000;

        trace_seq_printf(
            s,
            c"%3d #%-6d %12llu ".as_ptr(),
            cpu,
            (*cpu_data).sum_cycles,
            (*cpu_data).sum_runtime,
        );
        trace_seq_printf(s, c"%12llu ".as_ptr(), (*cpu_data).sum_noise);
        trace_seq_printf(s, c"  %3d.%05d".as_ptr(), percentage, decimal);
        trace_seq_printf(
            s,
            c"%12llu %12llu".as_ptr(),
            (*cpu_data).max_noise,
            (*cpu_data).max_sample,
        );

        trace_seq_printf(s, c"%12llu ".as_ptr(), (*cpu_data).hw_count);
        trace_seq_printf(s, c"%12llu ".as_ptr(), (*cpu_data).nmi_count);

        if (*params).mode == MODE_HWNOISE {
            trace_seq_printf(s, c"\n".as_ptr());
            return;
        }

        trace_seq_printf(s, c"%12llu ".as_ptr(), (*cpu_data).irq_count);
        trace_seq_printf(s, c"%12llu ".as_ptr(), (*cpu_data).softirq_count);
        trace_seq_printf(s, c"%12llu\n".as_ptr(), (*cpu_data).thread_count);
    }
}

/*
 * osnoise_print_stats - print data for all cpus
 */
unsafe extern "C" fn osnoise_print_stats(top: *mut osnoise_tool) {
    let params: *mut osnoise_params;
    let trace: *mut trace_instance;
    let mut i: c_int;

    unsafe {
        params = to_osnoise_params((*top).params);
        trace = &mut (*top).trace;

        if !(*params).common.quiet {
            clear_terminal((*trace).seq);
        }

        osnoise_top_header(top);

        for_each_monitored_cpu!(i, &mut (*params).common, {
            osnoise_top_print(top, i);
        });

        trace_seq_do_printf((*trace).seq);
        trace_seq_reset((*trace).seq);
        osnoise_report_missed_events(top);
    }
}

/*
 * osnoise_top_apply_config - apply the top configs to the initialized tool
 */
unsafe extern "C" fn osnoise_top_apply_config(tool: *mut osnoise_tool) -> c_int {
    let params: *mut osnoise_params;
    let mut retval: c_int;

    unsafe {
        params = to_osnoise_params((*tool).params);

        retval = osnoise_apply_config(tool, params);
        if retval != 0 {
            return -1;
        }

        if (*params).mode == MODE_HWNOISE {
            retval = osnoise_set_irq_disable((*tool).context, 1);
            if retval != 0 {
                err_msg(c"Failed to set OSNOISE_IRQ_DISABLE option\n".as_ptr());
                return -1;
            }
        }

        if isatty(STDOUT_FILENO) != 0 && !(*params).common.quiet {
            (*params).common.pretty_output = true;
        }
    }

    0
}

/*
 * osnoise_init_top - initialize a osnoise top tool with parameters
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn osnoise_init_top(_params: *mut common_params) -> *mut osnoise_tool {
    let tool: *mut osnoise_tool;

    unsafe {
        tool = osnoise_init_tool(c"osnoise_top".as_ptr());
        if tool.is_null() {
            return ptr::null_mut();
        }

        (*tool).data = osnoise_alloc_top() as *mut c_void;
        if (*tool).data.is_null() {
            osnoise_destroy_tool(tool);
            return ptr::null_mut();
        }

        tep_register_event_handler(
            (*tool).trace.tep,
            -1,
            c"ftrace".as_ptr(),
            c"osnoise".as_ptr(),
            Some(osnoise_top_handler),
            ptr::null_mut(),
        );
    }

    tool
}

#[unsafe(no_mangle)]
pub static mut osnoise_top_ops: tool_ops = tool_ops {
    tracer: c"osnoise".as_ptr(),
    comm_prefix: c"osnoise/".as_ptr(),
    parse_args: Some(osnoise_top_parse_args),
    init_tool: Some(osnoise_init_top),
    apply_config: Some(osnoise_top_apply_config),
    enable: Some(osnoise_enable),
    main: Some(top_main_loop),
    print_stats: Some(osnoise_print_stats),
    free: Some(osnoise_free_top_tool),
};
