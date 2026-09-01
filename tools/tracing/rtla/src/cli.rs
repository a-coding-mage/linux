// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies removed from executable Rust: stdlib.h, string.h, stdio.h,
// unistd.h, sys/types.h, linux/compiler.h, and cli_p.h.
// The original file defined _GNU_SOURCE and RTLA_ALLOW_CLI_P_H before cli_p.h.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

static mut OSNOISE_TOP_USAGE: [*const c_char; 2] = [
    b"rtla osnoise [top] [<options>] [-h|--help]\0".as_ptr() as *const c_char,
    ptr::null(),
];

static mut OSNOISE_HIST_USAGE: [*const c_char; 2] = [
    b"rtla osnoise hist [<options>] [-h|--help]\0".as_ptr() as *const c_char,
    ptr::null(),
];

static mut TIMERLAT_TOP_USAGE: [*const c_char; 2] = [
    b"rtla timerlat [top] [<options>] [-h|--help]\0".as_ptr() as *const c_char,
    ptr::null(),
];

static mut TIMERLAT_HIST_USAGE: [*const c_char; 2] = [
    b"rtla timerlat hist [<options>] [-h|--help]\0".as_ptr() as *const c_char,
    ptr::null(),
];

static mut HWNOISE_USAGE: [*const c_char; 2] = [
    b"rtla hwnoise [<options>] [-h|--help]\0".as_ptr() as *const c_char,
    ptr::null(),
];

static COMMON_PARSE_OPTIONS_FLAGS: c_int = PARSE_OPT_OPTARG_ALLOW_NEXT;

pub static mut in_unit_test: bool = false;

/*
 * osnoise_top_parse_args - allocs, parse and fill the cmd line parameters
 */
pub unsafe extern "C" fn osnoise_top_parse_args(
    mut argc: c_int,
    argv: *mut *mut c_char,
) -> *mut common_params {
    let params: *mut osnoise_params =
        calloc_fatal(1, core::mem::size_of::<osnoise_params>()) as *mut osnoise_params;
    let mut cb_data: osnoise_cb_data = core::mem::zeroed();
    let usage: *mut *const c_char;

    cb_data.params = params;
    cb_data.trace_output = ptr::null_mut();

    if strcmp(*argv.offset(0), b"hwnoise\0".as_ptr() as *const c_char) == 0 {
        (*params).mode = MODE_HWNOISE;
        /*
         * Reduce CPU usage for 75% to avoid killing the system.
         */
        (*params).runtime = 750000;
        (*params).period = 1000000;
        usage = HWNOISE_USAGE.as_mut_ptr();
    } else {
        usage = OSNOISE_TOP_USAGE.as_mut_ptr();
    }

    let osnoise_top_options = [
        OPT_GROUP!(b"Tracing Options:\0".as_ptr() as *const c_char),
        OSNOISE_OPT_PERIOD!(),
        OSNOISE_OPT_RUNTIME!(),
        RTLA_OPT_STOP!(b's' as c_int, b"stop\0".as_ptr() as *const c_char, b"single sample\0".as_ptr() as *const c_char),
        RTLA_OPT_STOP_TOTAL!(b'S' as c_int, b"stop-total\0".as_ptr() as *const c_char, b"total sample\0".as_ptr() as *const c_char),
        OSNOISE_OPT_THRESHOLD!(),
        RTLA_OPT_TRACE_OUTPUT!(b"osnoise\0".as_ptr() as *const c_char, opt_osnoise_trace_output_cb),
        OPT_GROUP!(b"Event Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_EVENT!(),
        RTLA_OPT_FILTER!(),
        RTLA_OPT_TRIGGER!(),
        OPT_GROUP!(b"CPU Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_CPUS!(),
        RTLA_OPT_HOUSEKEEPING!(),
        OPT_GROUP!(b"Thread Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_PRIORITY!(),
        RTLA_OPT_CGROUP!(),
        OPT_GROUP!(b"Output:\0".as_ptr() as *const c_char),
        RTLA_OPT_QUIET!(),
        OPT_GROUP!(b"System Tuning:\0".as_ptr() as *const c_char),
        RTLA_OPT_TRACE_BUFFER_SIZE!(),
        RTLA_OPT_WARM_UP!(),
        OPT_GROUP!(b"Auto Analysis and Actions:\0".as_ptr() as *const c_char),
        RTLA_OPT_AUTO!(opt_osnoise_auto_cb),
        RTLA_OPT_ON_THRESHOLD!(b"stop-total\0".as_ptr() as *const c_char, opt_osnoise_on_threshold_cb),
        RTLA_OPT_ON_END!(opt_osnoise_on_end_cb),
        OPT_GROUP!(b"General:\0".as_ptr() as *const c_char),
        RTLA_OPT_DURATION!(),
        RTLA_OPT_DEBUG!(),
        OPT_END!(),
    ];

    actions_init(&mut (*params).common.threshold_actions);
    actions_init(&mut (*params).common.end_actions);

    argc = parse_options(
        argc,
        argv as *mut *const c_char,
        osnoise_top_options.as_ptr(),
        usage as *const *const c_char,
        COMMON_PARSE_OPTIONS_FLAGS,
    );
    if argc < 0 {
        return ptr::null_mut();
    }

    if !cb_data.trace_output.is_null() {
        actions_add_trace_output(&mut (*params).common.threshold_actions, cb_data.trace_output);
    }

    if geteuid() != 0 && !in_unit_test {
        fatal(b"osnoise needs root permission\0".as_ptr() as *const c_char);
    }

    &mut (*params).common
}

/*
 * osnoise_hist_parse_args - allocs, parse and fill the cmd line parameters
 */
pub unsafe extern "C" fn osnoise_hist_parse_args(
    mut argc: c_int,
    argv: *mut *mut c_char,
) -> *mut common_params {
    let params: *mut osnoise_params =
        calloc_fatal(1, core::mem::size_of::<osnoise_params>()) as *mut osnoise_params;
    let mut cb_data: osnoise_cb_data = core::mem::zeroed();

    cb_data.params = params;
    cb_data.trace_output = ptr::null_mut();

    let osnoise_hist_options = [
        OPT_GROUP!(b"Tracing Options:\0".as_ptr() as *const c_char),
        OSNOISE_OPT_PERIOD!(),
        OSNOISE_OPT_RUNTIME!(),
        RTLA_OPT_STOP!(b's' as c_int, b"stop\0".as_ptr() as *const c_char, b"single sample\0".as_ptr() as *const c_char),
        RTLA_OPT_STOP_TOTAL!(b'S' as c_int, b"stop-total\0".as_ptr() as *const c_char, b"total sample\0".as_ptr() as *const c_char),
        OSNOISE_OPT_THRESHOLD!(),
        RTLA_OPT_TRACE_OUTPUT!(b"osnoise\0".as_ptr() as *const c_char, opt_osnoise_trace_output_cb),
        OPT_GROUP!(b"Event Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_EVENT!(),
        RTLA_OPT_FILTER!(),
        RTLA_OPT_TRIGGER!(),
        OPT_GROUP!(b"CPU Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_CPUS!(),
        RTLA_OPT_HOUSEKEEPING!(),
        OPT_GROUP!(b"Thread Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_PRIORITY!(),
        RTLA_OPT_CGROUP!(),
        OPT_GROUP!(b"Histogram Options:\0".as_ptr() as *const c_char),
        HIST_OPT_BUCKET_SIZE!(),
        HIST_OPT_ENTRIES!(),
        HIST_OPT_NO_HEADER!(),
        HIST_OPT_NO_SUMMARY!(),
        HIST_OPT_NO_INDEX!(),
        HIST_OPT_WITH_ZEROS!(),
        OPT_GROUP!(b"System Tuning:\0".as_ptr() as *const c_char),
        RTLA_OPT_TRACE_BUFFER_SIZE!(),
        RTLA_OPT_WARM_UP!(),
        OPT_GROUP!(b"Auto Analysis and Actions:\0".as_ptr() as *const c_char),
        RTLA_OPT_AUTO!(opt_osnoise_auto_cb),
        RTLA_OPT_ON_THRESHOLD!(b"stop-total\0".as_ptr() as *const c_char, opt_osnoise_on_threshold_cb),
        RTLA_OPT_ON_END!(opt_osnoise_on_end_cb),
        OPT_GROUP!(b"General:\0".as_ptr() as *const c_char),
        RTLA_OPT_DURATION!(),
        RTLA_OPT_DEBUG!(),
        OPT_END!(),
    ];

    actions_init(&mut (*params).common.threshold_actions);
    actions_init(&mut (*params).common.end_actions);

    /* set default values */
    (*params).common.output_divisor = default_output_divisor;
    (*params).common.hist.bucket_size = default_bucket_size;
    (*params).common.hist.entries = default_entries;

    argc = parse_options(
        argc,
        argv as *mut *const c_char,
        osnoise_hist_options.as_ptr(),
        OSNOISE_HIST_USAGE.as_mut_ptr() as *const *const c_char,
        COMMON_PARSE_OPTIONS_FLAGS,
    );
    if argc < 0 {
        return ptr::null_mut();
    }

    if !cb_data.trace_output.is_null() {
        actions_add_trace_output(&mut (*params).common.threshold_actions, cb_data.trace_output);
    }

    if geteuid() != 0 && !in_unit_test {
        fatal(b"rtla needs root permission\0".as_ptr() as *const c_char);
    }

    if (*params).common.hist.no_index != 0 && (*params).common.hist.with_zeros == 0 {
        fatal(b"no-index set and with-zeros not set - it does not make sense\0".as_ptr() as *const c_char);
    }

    &mut (*params).common
}

pub unsafe extern "C" fn timerlat_top_parse_args(
    mut argc: c_int,
    argv: *mut *mut c_char,
) -> *mut common_params {
    let params: *mut timerlat_params =
        calloc_fatal(1, core::mem::size_of::<timerlat_params>()) as *mut timerlat_params;
    let mut cb_data: timerlat_cb_data = core::mem::zeroed();

    cb_data.params = params;
    cb_data.trace_output = ptr::null_mut();

    let timerlat_top_options = [
        OPT_GROUP!(b"Tracing Options:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_PERIOD!(),
        RTLA_OPT_STOP!(b'i' as c_int, b"irq\0".as_ptr() as *const c_char, b"irq latency\0".as_ptr() as *const c_char),
        RTLA_OPT_STOP_TOTAL!(b'T' as c_int, b"thread\0".as_ptr() as *const c_char, b"thread latency\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_STACK!(),
        RTLA_OPT_TRACE_OUTPUT!(b"timerlat\0".as_ptr() as *const c_char, opt_timerlat_trace_output_cb),
        OPT_GROUP!(b"Event Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_EVENT!(),
        RTLA_OPT_FILTER!(),
        RTLA_OPT_TRIGGER!(),
        OPT_GROUP!(b"CPU Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_CPUS!(),
        RTLA_OPT_HOUSEKEEPING!(),
        OPT_GROUP!(b"Thread Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_PRIORITY!(),
        RTLA_OPT_CGROUP!(),
        RTLA_OPT_USER_THREADS!(),
        RTLA_OPT_KERNEL_THREADS!(),
        RTLA_OPT_USER_LOAD!(),
        TIMERLAT_OPT_ALIGNED!(),
        OPT_GROUP!(b"Output:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_NANO!(),
        RTLA_OPT_QUIET!(),
        OPT_GROUP!(b"System Tuning:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_DMA_LATENCY!(),
        TIMERLAT_OPT_DEEPEST_IDLE_STATE!(),
        RTLA_OPT_TRACE_BUFFER_SIZE!(),
        RTLA_OPT_WARM_UP!(),
        OPT_GROUP!(b"Auto Analysis and Actions:\0".as_ptr() as *const c_char),
        RTLA_OPT_AUTO!(opt_timerlat_auto_cb),
        TIMERLAT_OPT_AA_ONLY!(),
        TIMERLAT_OPT_NO_AA!(),
        TIMERLAT_OPT_DUMPS_TASKS!(),
        RTLA_OPT_ON_THRESHOLD!(b"latency\0".as_ptr() as *const c_char, opt_timerlat_on_threshold_cb),
        RTLA_OPT_ON_END!(opt_timerlat_on_end_cb),
        TIMERLAT_OPT_BPF_ACTION!(),
        TIMERLAT_OPT_STACK_FORMAT!(),
        OPT_GROUP!(b"General:\0".as_ptr() as *const c_char),
        RTLA_OPT_DURATION!(),
        RTLA_OPT_DEBUG!(),
        OPT_END!(),
    ];

    actions_init(&mut (*params).common.threshold_actions);
    actions_init(&mut (*params).common.end_actions);

    /* set default values */
    (*params).dma_latency = default_dma_latency;
    (*params).deepest_idle_state = default_deepest_idle_state;
    (*params).common.output_divisor = default_output_divisor;
    (*params).stack_format = default_stack_format;

    /* default to BPF mode */
    (*params).mode = TRACING_MODE_BPF;

    argc = parse_options(
        argc,
        argv as *mut *const c_char,
        timerlat_top_options.as_ptr(),
        TIMERLAT_TOP_USAGE.as_mut_ptr() as *const *const c_char,
        COMMON_PARSE_OPTIONS_FLAGS,
    );
    if argc < 0 {
        return ptr::null_mut();
    }

    if !cb_data.trace_output.is_null() {
        actions_add_trace_output(&mut (*params).common.threshold_actions, cb_data.trace_output);
    }

    if geteuid() != 0 && !in_unit_test {
        fatal(b"rtla needs root permission\0".as_ptr() as *const c_char);
    }

    /*
     * Auto analysis only happens if stop tracing, thus:
     */
    if (*params).common.stop_us == 0 && (*params).common.stop_total_us == 0 {
        (*params).no_aa = 1;
    }

    if (*params).no_aa != 0 && (*params).common.aa_only != 0 {
        fatal(b"--no-aa and --aa-only are mutually exclusive!\0".as_ptr() as *const c_char);
    }

    if (*params).common.kernel_workload != 0 && (*params).common.user_workload != 0 {
        fatal(b"--kernel-threads and --user-threads are mutually exclusive!\0".as_ptr() as *const c_char);
    }

    /*
     * If auto-analysis or trace output is enabled, switch from BPF mode to
     * mixed mode
     */
    if (*params).mode == TRACING_MODE_BPF
        && ((*params).common.threshold_actions.present[ACTION_TRACE_OUTPUT as usize] != 0
            || (*params).common.end_actions.present[ACTION_TRACE_OUTPUT as usize] != 0
            || (*params).no_aa == 0)
    {
        (*params).mode = TRACING_MODE_MIXED;
    }

    &mut (*params).common
}

pub unsafe extern "C" fn timerlat_hist_parse_args(
    mut argc: c_int,
    argv: *mut *mut c_char,
) -> *mut common_params {
    let params: *mut timerlat_params =
        calloc_fatal(1, core::mem::size_of::<timerlat_params>()) as *mut timerlat_params;
    let mut cb_data: timerlat_cb_data = core::mem::zeroed();

    cb_data.params = params;
    cb_data.trace_output = ptr::null_mut();

    let timerlat_hist_options = [
        OPT_GROUP!(b"Tracing Options:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_PERIOD!(),
        RTLA_OPT_STOP!(b'i' as c_int, b"irq\0".as_ptr() as *const c_char, b"irq latency\0".as_ptr() as *const c_char),
        RTLA_OPT_STOP_TOTAL!(b'T' as c_int, b"thread\0".as_ptr() as *const c_char, b"thread latency\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_STACK!(),
        RTLA_OPT_TRACE_OUTPUT!(b"timerlat\0".as_ptr() as *const c_char, opt_timerlat_trace_output_cb),
        OPT_GROUP!(b"Event Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_EVENT!(),
        RTLA_OPT_FILTER!(),
        RTLA_OPT_TRIGGER!(),
        OPT_GROUP!(b"CPU Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_CPUS!(),
        RTLA_OPT_HOUSEKEEPING!(),
        OPT_GROUP!(b"Thread Configuration:\0".as_ptr() as *const c_char),
        RTLA_OPT_PRIORITY!(),
        RTLA_OPT_CGROUP!(),
        RTLA_OPT_USER_THREADS!(),
        RTLA_OPT_KERNEL_THREADS!(),
        RTLA_OPT_USER_LOAD!(),
        TIMERLAT_OPT_ALIGNED!(),
        OPT_GROUP!(b"Histogram Options:\0".as_ptr() as *const c_char),
        HIST_OPT_BUCKET_SIZE!(),
        HIST_OPT_ENTRIES!(),
        HIST_OPT_NO_IRQ!(),
        HIST_OPT_NO_THREAD!(),
        HIST_OPT_NO_HEADER!(),
        HIST_OPT_NO_SUMMARY!(),
        HIST_OPT_NO_INDEX!(),
        HIST_OPT_WITH_ZEROS!(),
        OPT_GROUP!(b"Output:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_NANO!(),
        OPT_GROUP!(b"System Tuning:\0".as_ptr() as *const c_char),
        TIMERLAT_OPT_DMA_LATENCY!(),
        TIMERLAT_OPT_DEEPEST_IDLE_STATE!(),
        RTLA_OPT_TRACE_BUFFER_SIZE!(),
        RTLA_OPT_WARM_UP!(),
        OPT_GROUP!(b"Auto Analysis and Actions:\0".as_ptr() as *const c_char),
        RTLA_OPT_AUTO!(opt_timerlat_auto_cb),
        TIMERLAT_OPT_NO_AA!(),
        TIMERLAT_OPT_DUMPS_TASKS!(),
        RTLA_OPT_ON_THRESHOLD!(b"latency\0".as_ptr() as *const c_char, opt_timerlat_on_threshold_cb),
        RTLA_OPT_ON_END!(opt_timerlat_on_end_cb),
        TIMERLAT_OPT_BPF_ACTION!(),
        TIMERLAT_OPT_STACK_FORMAT!(),
        OPT_GROUP!(b"General:\0".as_ptr() as *const c_char),
        RTLA_OPT_DURATION!(),
        RTLA_OPT_DEBUG!(),
        OPT_END!(),
    ];

    actions_init(&mut (*params).common.threshold_actions);
    actions_init(&mut (*params).common.end_actions);

    /* set default values */
    (*params).dma_latency = default_dma_latency;
    (*params).deepest_idle_state = default_deepest_idle_state;
    (*params).common.output_divisor = default_output_divisor;
    (*params).common.hist.bucket_size = default_bucket_size;
    (*params).common.hist.entries = default_entries;
    (*params).stack_format = default_stack_format;

    /* default to BPF mode */
    (*params).mode = TRACING_MODE_BPF;

    argc = parse_options(
        argc,
        argv as *mut *const c_char,
        timerlat_hist_options.as_ptr(),
        TIMERLAT_HIST_USAGE.as_mut_ptr() as *const *const c_char,
        COMMON_PARSE_OPTIONS_FLAGS,
    );
    if argc < 0 {
        return ptr::null_mut();
    }

    if !cb_data.trace_output.is_null() {
        actions_add_trace_output(&mut (*params).common.threshold_actions, cb_data.trace_output);
    }

    if geteuid() != 0 && !in_unit_test {
        fatal(b"rtla needs root permission\0".as_ptr() as *const c_char);
    }

    if (*params).common.hist.no_irq != 0 && (*params).common.hist.no_thread != 0 {
        fatal(b"no-irq and no-thread set, there is nothing to do here\0".as_ptr() as *const c_char);
    }

    if (*params).common.hist.no_index != 0 && (*params).common.hist.with_zeros == 0 {
        fatal(b"no-index set with with-zeros is not set - it does not make sense\0".as_ptr() as *const c_char);
    }

    /*
     * Auto analysis only happens if stop tracing, thus:
     */
    if (*params).common.stop_us == 0 && (*params).common.stop_total_us == 0 {
        (*params).no_aa = 1;
    }

    if (*params).common.kernel_workload != 0 && (*params).common.user_workload != 0 {
        fatal(b"--kernel-threads and --user-threads are mutually exclusive!\0".as_ptr() as *const c_char);
    }

    /*
     * If auto-analysis or trace output is enabled, switch from BPF mode to
     * mixed mode
     */
    if (*params).mode == TRACING_MODE_BPF
        && ((*params).common.threshold_actions.present[ACTION_TRACE_OUTPUT as usize] != 0
            || (*params).common.end_actions.present[ACTION_TRACE_OUTPUT as usize] != 0
            || (*params).no_aa == 0)
    {
        (*params).mode = TRACING_MODE_MIXED;
    }

    &mut (*params).common
}

/*
 * rtla_usage - print rtla usage
 */
unsafe extern "C" fn rtla_usage(err: c_int) -> ! {
    let mut i: c_int;

    static mut MSG: [*const c_char; 11] = [
        b"\0".as_ptr() as *const c_char,
        concat!("rtla version ", VERSION, "\0").as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"  usage: rtla COMMAND ...\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        b"  commands:\0".as_ptr() as *const c_char,
        b"     osnoise  - gives information about the operating system noise (osnoise)\0".as_ptr() as *const c_char,
        b"     hwnoise  - gives information about hardware-related noise\0".as_ptr() as *const c_char,
        b"     timerlat - measures the timer irq and thread latency\0".as_ptr() as *const c_char,
        b"\0".as_ptr() as *const c_char,
        ptr::null(),
    ];

    i = 0;
    while !MSG[i as usize].is_null() {
        fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, MSG[i as usize]);
        i += 1;
    }
    exit(err);
}

/*
 * run_tool_command - try to run a rtla tool command
 *
 * It returns 0 if it fails. The tool's main will generally not
 * return as they should call exit().
 */
pub unsafe extern "C" fn run_tool_command(
    argc: c_int,
    argv: *mut *mut c_char,
    start_position: c_int,
) -> c_int {
    if strcmp(
        *argv.offset(start_position as isize),
        b"osnoise\0".as_ptr() as *const c_char,
    ) == 0
    {
        osnoise_main(argc - start_position, argv.offset(start_position as isize));
        return 1;
    } else if strcmp(
        *argv.offset(start_position as isize),
        b"hwnoise\0".as_ptr() as *const c_char,
    ) == 0
    {
        hwnoise_main(argc - start_position, argv.offset(start_position as isize));
        return 1;
    } else if strcmp(
        *argv.offset(start_position as isize),
        b"timerlat\0".as_ptr() as *const c_char,
    ) == 0
    {
        timerlat_main(argc - start_position, argv.offset(start_position as isize));
        return 1;
    }

    0
}

/* Set main as weak to allow overriding it for building unit test binary */
// Original C used: #pragma weak main

pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut retval: c_int;

    /* is it an alias? */
    retval = run_tool_command(argc, argv, 0);
    if retval != 0 {
        exit(0);
    }

    if argc < 2 {
        rtla_usage(129);
    }

    if strcmp(*argv.offset(1), b"-h\0".as_ptr() as *const c_char) == 0 {
        rtla_usage(129);
    } else if strcmp(*argv.offset(1), b"--help\0".as_ptr() as *const c_char) == 0 {
        rtla_usage(129);
    }

    retval = run_tool_command(argc, argv, 1);
    if retval != 0 {
        exit(0);
    }

    rtla_usage(129);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
