/* SPDX-License-Identifier: GPL-2.0 */

/* Private header equivalent for cli.c module.
 *
 * C dependencies intentionally left external:
 * errno.h, limits.h, linux/kernel.h, subcmd/parse-options.h,
 * cli.h, osnoise.h, timerlat.h.
 */

use core::ffi::{c_char, c_int, c_longlong, c_void};

#[repr(C)]
pub struct osnoise_cb_data {
    pub params: *mut osnoise_params,
    pub trace_output: *mut c_char,
}

#[repr(C)]
pub struct timerlat_cb_data {
    pub params: *mut timerlat_params,
    pub trace_output: *mut c_char,
}

/*
 * Non-zero default values for parameters
 */
pub const default_dma_latency: c_int = -1; /* -1 = unset */
pub const default_deepest_idle_state: c_int = -2; /* -1 = disable all, -2 = unset */
pub const default_output_divisor: c_int = 1000;
pub const default_bucket_size: c_int = 1;
pub const default_entries: c_int = 256;
pub const default_stack_format: stack_format = STACK_FORMAT_TRUNCATE;

/*
 * Range checking for long long and int option callbacks.
 *
 * Pass a pointer to a const struct as opt->data to enable range checking.
 * If opt->data is NULL, no range check is performed.
 */
#[repr(C)]
pub struct llong_range {
    pub min: c_longlong,
    pub max: c_longlong,
}

#[repr(C)]
pub struct int_range {
    pub min: c_int,
    pub max: c_int,
}

macro_rules! LLONG_RANGE {
    ($lo:expr, $hi:expr) => {
        &llong_range {
            min: $lo,
            max: $hi,
        } as *const llong_range
    };
}

macro_rules! INT_RANGE {
    ($lo:expr, $hi:expr) => {
        &int_range { min: $lo, max: $hi } as *const int_range
    };
}

pub(crate) use INT_RANGE;
pub(crate) use LLONG_RANGE;

pub unsafe extern "C" fn check_llong_range(
    opt: *const option,
    value: c_longlong,
) -> c_int {
    let range = unsafe { (*opt).data as *const llong_range };

    if range.is_null() {
        return 0;
    }
    if value < unsafe { (*range).min } || value > unsafe { (*range).max } {
        unsafe {
            fprintf(
                stderr,
                c" Error: --%s value %lld is out of range [%lld, %lld]\n".as_ptr(),
                (*opt).long_name,
                value,
                (*range).min,
                (*range).max,
            );
        }
        return -1;
    }
    0
}

pub unsafe extern "C" fn check_int_range(opt: *const option, value: c_int) -> c_int {
    let range = unsafe { (*opt).data as *const int_range };

    if range.is_null() {
        return 0;
    }
    if value < unsafe { (*range).min } || value > unsafe { (*range).max } {
        unsafe {
            fprintf(
                stderr,
                c" Error: --%s value %d is out of range [%d, %d]\n".as_ptr(),
                (*opt).long_name,
                value,
                (*range).min,
                (*range).max,
            );
        }
        return -1;
    }
    0
}

/*
 * OPT_CALLBACK variant that populates .data (for range checking).
 */
macro_rules! RTLA_OPT_CALLBACK_DATA {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr, $d:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some($f),
            data: $d as *mut c_void,
            ..Default::default()
        }
    };
}

macro_rules! RTLA_OPT_CALLBACK_DATA_DEFVAL {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $f:expr, $d:expr, $dv:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some($f),
            data: $d as *mut c_void,
            defval: $dv as intptr_t,
            ..Default::default()
        }
    };
}

/*
 * Shorthand macros for integer/long long command line options using
 * opt_int_callback/opt_llong_callback, with variants that set defval
 * and/or data (for range checking).
 *
 * Note: defval's type is intptr_t. opt_int_callback interprets it directly as
 * an int, opt_llong_callback interprets it as a pointer to a long long, as
 * long long does not fit into intptr_t on 32-bit architectures.
 */
macro_rules! RTLA_OPT_LLONG {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr) => {
        OPT_CALLBACK!($s, $l, $v, $a, $h, opt_llong_callback)
    };
}

macro_rules! RTLA_OPT_LLONG_DEFVAL {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $d:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some(opt_llong_callback),
            defval: $d as intptr_t,
            ..Default::default()
        }
    };
}

macro_rules! RTLA_OPT_LLONG_DATA {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $d:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some(opt_llong_callback),
            data: $d as *mut c_void,
            ..Default::default()
        }
    };
}

macro_rules! RTLA_OPT_INT {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr) => {
        OPT_CALLBACK!($s, $l, $v, $a, $h, opt_int_callback)
    };
}

macro_rules! RTLA_OPT_INT_DEFVAL {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $d:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some(opt_int_callback),
            defval: $d as intptr_t,
            ..Default::default()
        }
    };
}

macro_rules! RTLA_OPT_INT_DATA_DEFVAL {
    ($s:expr, $l:expr, $v:expr, $a:expr, $h:expr, $d:expr, $dv:expr) => {
        option {
            type_: OPTION_CALLBACK,
            short_name: $s,
            long_name: $l,
            value: $v as *mut c_void,
            argh: $a,
            help: $h,
            callback: Some(opt_int_callback),
            data: $d as *mut c_void,
            defval: $dv as intptr_t,
            ..Default::default()
        }
    };
}

pub(crate) use RTLA_OPT_CALLBACK_DATA;
pub(crate) use RTLA_OPT_CALLBACK_DATA_DEFVAL;
pub(crate) use RTLA_OPT_INT;
pub(crate) use RTLA_OPT_INT_DATA_DEFVAL;
pub(crate) use RTLA_OPT_INT_DEFVAL;
pub(crate) use RTLA_OPT_LLONG;
pub(crate) use RTLA_OPT_LLONG_DATA;
pub(crate) use RTLA_OPT_LLONG_DEFVAL;

/*
 * Macros for command line options common to all tools
 *
 * Note: Some of the options are common to both timerlat and osnoise, but
 * have a slightly different meaning. Such options take additional arguments
 * that have to be provided by the *_parse_args() function of the corresponding
 * tool.
 *
 * All macros defined here assume the presence of a params variable of
 * the corresponding tool type (i.e struct timerlat_params or struct osnoise_params)
 * and a cb_data variable of the matching type.
 */

macro_rules! RTLA_OPT_STOP {
    ($short:expr, $long:expr, $name:expr) => {
        OPT_CALLBACK_FLAG!(
            $short,
            $long,
            &mut (*params).common.stop_us,
            c"us".as_ptr(),
            concat!("stop trace if ", $name, " is higher than the argument in us\0").as_ptr()
                as *const c_char,
            opt_llong_callback,
            PARSE_OPT_NOAUTONEG
        )
    };
}

macro_rules! RTLA_OPT_STOP_TOTAL {
    ($short:expr, $long:expr, $name:expr) => {
        OPT_CALLBACK_FLAG!(
            $short,
            $long,
            &mut (*params).common.stop_total_us,
            c"us".as_ptr(),
            concat!("stop trace if ", $name, " is higher than the argument in us\0").as_ptr()
                as *const c_char,
            opt_llong_callback,
            PARSE_OPT_NOAUTONEG
        )
    };
}

macro_rules! RTLA_OPT_TRACE_OUTPUT {
    ($tracer:expr, $cb:expr) => {
        OPT_CALLBACK_OPTARG!(
            b't' as c_int,
            c"trace".as_ptr(),
            &mut cb_data.trace_output as *mut *mut c_char as *mut *const c_char,
            concat!($tracer, "_trace.txt\0").as_ptr() as *const c_char,
            c"[file]".as_ptr(),
            concat!("save the stopped trace to [file|", $tracer, "_trace.txt]\0").as_ptr()
                as *const c_char,
            $cb
        )
    };
}

macro_rules! RTLA_OPT_CPUS {
    () => {
        OPT_CALLBACK!(
            b'c' as c_int,
            c"cpus".as_ptr(),
            &mut (*params).common,
            c"cpu-list".as_ptr(),
            c"run the tracer only on the given cpus".as_ptr(),
            opt_cpus_cb
        )
    };
}

macro_rules! RTLA_OPT_CGROUP {
    () => {
        OPT_CALLBACK_OPTARG!(
            b'C' as c_int,
            c"cgroup".as_ptr(),
            &mut (*params).common,
            c"[cgroup_name]".as_ptr(),
            core::ptr::null(),
            c"set cgroup, no argument means rtla's cgroup will be inherited".as_ptr(),
            opt_cgroup_cb
        )
    };
}

macro_rules! RTLA_OPT_USER_THREADS {
    () => {
        OPT_CALLBACK_NOOPT!(
            b'u' as c_int,
            c"user-threads".as_ptr(),
            params,
            core::ptr::null(),
            c"use rtla user-space threads instead of kernel-space timerlat threads".as_ptr(),
            opt_user_threads_cb
        )
    };
}

macro_rules! RTLA_OPT_KERNEL_THREADS {
    () => {
        OPT_BOOLEAN!(
            b'k' as c_int,
            c"kernel-threads".as_ptr(),
            &mut (*params).common.kernel_workload,
            c"use timerlat kernel-space threads instead of rtla user-space threads".as_ptr()
        )
    };
}

macro_rules! RTLA_OPT_USER_LOAD {
    () => {
        OPT_BOOLEAN!(
            b'U' as c_int,
            c"user-load".as_ptr(),
            &mut (*params).common.user_data,
            c"enable timerlat for user-defined user-space workload".as_ptr()
        )
    };
}

macro_rules! RTLA_OPT_DURATION {
    () => {
        OPT_CALLBACK!(
            b'd' as c_int,
            c"duration".as_ptr(),
            &mut (*params).common,
            c"time[s|m|h|d]".as_ptr(),
            c"set the duration of the session".as_ptr(),
            opt_duration_cb
        )
    };
}

macro_rules! RTLA_OPT_EVENT {
    () => {
        OPT_CALLBACK!(
            b'e' as c_int,
            c"event".as_ptr(),
            &mut (*params).common.events,
            c"sys:event".as_ptr(),
            c"enable the <sys:event> in the trace instance, multiple -e are allowed".as_ptr(),
            opt_event_cb
        )
    };
}

macro_rules! RTLA_OPT_HOUSEKEEPING {
    () => {
        OPT_CALLBACK!(
            b'H' as c_int,
            c"house-keeping".as_ptr(),
            &mut (*params).common,
            c"cpu-list".as_ptr(),
            c"run rtla control threads only on the given cpus".as_ptr(),
            opt_housekeeping_cb
        )
    };
}

macro_rules! RTLA_OPT_PRIORITY {
    () => {
        OPT_CALLBACK!(
            b'P' as c_int,
            c"priority".as_ptr(),
            &mut (*params).common,
            c"o:prio|r:prio|f:prio|d:runtime:period".as_ptr(),
            c"set scheduling parameters".as_ptr(),
            opt_priority_cb
        )
    };
}

macro_rules! RTLA_OPT_TRIGGER {
    () => {
        OPT_CALLBACK!(
            0,
            c"trigger".as_ptr(),
            &mut (*params).common.events,
            c"trigger".as_ptr(),
            c"enable a trace event trigger to the previous -e event".as_ptr(),
            opt_trigger_cb
        )
    };
}

macro_rules! RTLA_OPT_FILTER {
    () => {
        OPT_CALLBACK!(
            0,
            c"filter".as_ptr(),
            &mut (*params).common.events,
            c"filter".as_ptr(),
            c"enable a trace event filter to the previous -e event".as_ptr(),
            opt_filter_cb
        )
    };
}

macro_rules! RTLA_OPT_QUIET {
    () => {
        OPT_BOOLEAN!(
            b'q' as c_int,
            c"quiet".as_ptr(),
            &mut (*params).common.quiet,
            c"print only a summary at the end".as_ptr()
        )
    };
}

macro_rules! RTLA_OPT_TRACE_BUFFER_SIZE {
    () => {
        RTLA_OPT_INT!(
            0,
            c"trace-buffer-size".as_ptr(),
            &mut (*params).common.buffer_size,
            c"kB".as_ptr(),
            c"set the per-cpu trace buffer size in kB".as_ptr()
        )
    };
}

macro_rules! RTLA_OPT_WARM_UP {
    () => {
        RTLA_OPT_INT!(
            0,
            c"warm-up".as_ptr(),
            &mut (*params).common.warmup,
            c"s".as_ptr(),
            c"let the workload run for s seconds before collecting data".as_ptr()
        )
    };
}

macro_rules! RTLA_OPT_AUTO {
    ($cb:expr) => {
        OPT_CALLBACK!(
            b'a' as c_int,
            c"auto".as_ptr(),
            &mut cb_data,
            c"us".as_ptr(),
            c"set automatic trace mode, stopping the session if argument in us sample is hit".as_ptr(),
            $cb
        )
    };
}

macro_rules! RTLA_OPT_ON_THRESHOLD {
    ($threshold:expr, $cb:expr) => {
        OPT_CALLBACK!(
            0,
            c"on-threshold".as_ptr(),
            &mut (*params).common.threshold_actions,
            c"action".as_ptr(),
            concat!("define action to be executed at ", $threshold, " threshold, multiple are allowed\0")
                .as_ptr() as *const c_char,
            $cb
        )
    };
}

macro_rules! RTLA_OPT_ON_END {
    ($cb:expr) => {
        OPT_CALLBACK!(
            0,
            c"on-end".as_ptr(),
            &mut (*params).common.end_actions,
            c"action".as_ptr(),
            c"define action to be executed at measurement end, multiple are allowed".as_ptr(),
            $cb
        )
    };
}

macro_rules! RTLA_OPT_DEBUG {
    () => {
        OPT_BOOLEAN!(
            b'D' as c_int,
            c"debug".as_ptr(),
            &mut config_debug,
            c"print debug info".as_ptr()
        )
    };
}

pub(crate) use RTLA_OPT_AUTO;
pub(crate) use RTLA_OPT_CGROUP;
pub(crate) use RTLA_OPT_CPUS;
pub(crate) use RTLA_OPT_DEBUG;
pub(crate) use RTLA_OPT_DURATION;
pub(crate) use RTLA_OPT_EVENT;
pub(crate) use RTLA_OPT_FILTER;
pub(crate) use RTLA_OPT_HOUSEKEEPING;
pub(crate) use RTLA_OPT_KERNEL_THREADS;
pub(crate) use RTLA_OPT_ON_END;
pub(crate) use RTLA_OPT_ON_THRESHOLD;
pub(crate) use RTLA_OPT_PRIORITY;
pub(crate) use RTLA_OPT_QUIET;
pub(crate) use RTLA_OPT_STOP;
pub(crate) use RTLA_OPT_STOP_TOTAL;
pub(crate) use RTLA_OPT_TRACE_BUFFER_SIZE;
pub(crate) use RTLA_OPT_TRACE_OUTPUT;
pub(crate) use RTLA_OPT_TRIGGER;
pub(crate) use RTLA_OPT_USER_LOAD;
pub(crate) use RTLA_OPT_USER_THREADS;
pub(crate) use RTLA_OPT_WARM_UP;

/*
 * Helper functions for parsing numeric option arguments.
 */
pub unsafe extern "C" fn opt_err(opt: *const option, arg: *const c_char, msg: *const c_char) {
    unsafe {
        fprintf(
            stderr,
            c" Error: --%s: '%s' %s\n".as_ptr(),
            (*opt).long_name,
            arg,
            msg,
        );
    }
}

pub unsafe extern "C" fn strtoll_safe(
    opt: *const option,
    arg: *const c_char,
    value: *mut c_longlong,
) -> c_int {
    let mut tmp: c_longlong;
    let mut end: *mut c_char = core::ptr::null_mut();

    unsafe {
        *__errno_location() = 0;
        tmp = strtoll(arg, &mut end, 10);
        if *__errno_location() != 0 || *end != 0 || end == arg as *mut c_char {
            opt_err(opt, arg, c"is not a valid number".as_ptr());
            return -1;
        }
        *value = tmp;
    }
    0
}

pub unsafe extern "C" fn strtoi_safe(
    opt: *const option,
    arg: *const c_char,
    value: *mut c_int,
) -> c_int {
    let mut tmp: c_int = 0;

    unsafe {
        if strtoi(arg, &mut tmp) != 0 {
            opt_err(opt, arg, c"is not a valid number".as_ptr());
            return -1;
        }
        *value = tmp;
    }
    0
}

/*
 * Common callback functions for command line options
 */

pub unsafe extern "C" fn opt_llong_callback(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let value = unsafe { (*opt).value as *mut c_longlong };

    if unset != 0 {
        unsafe {
            *value = if (*opt).defval != 0 {
                *((*opt).defval as *mut c_longlong)
            } else {
                0
            };
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoll_safe(opt, arg, value) != 0 {
            return -1;
        }
        if check_llong_range(opt, *value) != 0 {
            return -1;
        }
    }
    0
}

pub unsafe extern "C" fn opt_int_callback(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let value = unsafe { (*opt).value as *mut c_int };

    if unset != 0 {
        unsafe {
            *value = (*opt).defval as c_int;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoi_safe(opt, arg, value) != 0 {
            return -1;
        }
        if check_int_range(opt, *value) != 0 {
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_cpus_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut common_params };
    let retval: c_int;

    if unset != 0 {
        unsafe {
            CPU_ZERO(&mut (*params).monitored_cpus);
            (*params).cpus = core::ptr::null_mut();
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        retval = parse_cpu_set(arg as *mut c_char, &mut (*params).monitored_cpus);
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid cpu set".as_ptr());
            return -1;
        }
        (*params).cpus = arg as *mut c_char;
    }

    0
}

pub unsafe extern "C" fn opt_cgroup_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut common_params };

    if unset != 0 {
        unsafe {
            (*params).cgroup = 0;
            (*params).cgroup_name = core::ptr::null_mut();
        }
        return 0;
    }

    unsafe {
        (*params).cgroup = 1;
        (*params).cgroup_name = arg as *mut c_char;
        if !(*params).cgroup_name.is_null() && *(*params).cgroup_name == b'=' as c_char {
            /* Allow -C=<cgroup_name> next to -C[ ]<cgroup_name> */
            (*params).cgroup_name = (*params).cgroup_name.add(1);
        }
    }

    0
}

pub unsafe extern "C" fn opt_duration_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut common_params };

    if unset != 0 {
        unsafe {
            (*params).duration = 0;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        (*params).duration = parse_seconds_duration(arg as *mut c_char);
        if (*params).duration == 0 {
            opt_err(opt, arg, c"is not a valid duration".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_event_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let events = unsafe { (*opt).value as *mut *mut trace_events };
    let tevent: *mut trace_events;

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        tevent = trace_event_alloc(arg as *mut c_char);
        if tevent.is_null() {
            fatal(c"Error alloc trace event".as_ptr());
        }

        if !(*events).is_null() {
            (*tevent).next = *events;
        }
        *events = tevent;
    }

    0
}

pub unsafe extern "C" fn opt_housekeeping_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut common_params };
    let retval: c_int;

    if unset != 0 {
        unsafe {
            (*params).hk_cpus = 0;
            CPU_ZERO(&mut (*params).hk_cpu_set);
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        (*params).hk_cpus = 1;
        retval = parse_cpu_set(arg as *mut c_char, &mut (*params).hk_cpu_set);
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid cpu set".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_priority_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut common_params };
    let retval: c_int;

    if unset != 0 {
        unsafe {
            memset(
                &mut (*params).sched_param as *mut _ as *mut c_void,
                0,
                core::mem::size_of_val(&(*params).sched_param),
            );
            (*params).set_sched = 0;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        retval = parse_prio(arg as *mut c_char, &mut (*params).sched_param);
        if retval == -1 {
            opt_err(opt, arg, c"is not a valid priority".as_ptr());
            return -1;
        }
        (*params).set_sched = 1;
    }

    0
}

pub unsafe extern "C" fn opt_trigger_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let events = unsafe { (*opt).value as *mut *mut trace_events };

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        if (*events).is_null() {
            opt_err(opt, arg, c"has no previous event to apply to".as_ptr());
            return -1;
        }

        trace_event_add_trigger(*events, arg as *mut c_char);
    }

    0
}

pub unsafe extern "C" fn opt_filter_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let events = unsafe { (*opt).value as *mut *mut trace_events };

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        if (*events).is_null() {
            opt_err(opt, arg, c"has no previous event to apply to".as_ptr());
            return -1;
        }

        trace_event_add_filter(*events, arg as *mut c_char);
    }

    0
}

/*
 * Macros for command line options specific to osnoise
 */
macro_rules! OSNOISE_OPT_PERIOD {
    () => {
        RTLA_OPT_LLONG_DATA!(
            b'p' as c_int,
            c"period".as_ptr(),
            &mut (*params).period,
            c"us".as_ptr(),
            c"osnoise period in us".as_ptr(),
            LLONG_RANGE!(1, 10000000)
        )
    };
}

macro_rules! OSNOISE_OPT_RUNTIME {
    () => {
        RTLA_OPT_LLONG_DATA!(
            b'r' as c_int,
            c"runtime".as_ptr(),
            &mut (*params).runtime,
            c"us".as_ptr(),
            c"osnoise runtime in us".as_ptr(),
            LLONG_RANGE!(100, LLONG_MAX)
        )
    };
}

macro_rules! OSNOISE_OPT_THRESHOLD {
    () => {
        RTLA_OPT_LLONG!(
            b'T' as c_int,
            c"threshold".as_ptr(),
            &mut (*params).threshold,
            c"us".as_ptr(),
            c"the minimum delta to be considered a noise".as_ptr()
        )
    };
}

pub(crate) use OSNOISE_OPT_PERIOD;
pub(crate) use OSNOISE_OPT_RUNTIME;
pub(crate) use OSNOISE_OPT_THRESHOLD;

/*
 * Callback functions for command line options for osnoise tools
 */

pub unsafe extern "C" fn opt_osnoise_auto_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let cb_data = unsafe { (*opt).value as *mut osnoise_cb_data };
    let params = unsafe { (*cb_data).params };
    let mut auto_thresh: c_longlong = 0;

    if unset != 0 {
        unsafe {
            (*params).common.stop_us = 0;
            (*params).threshold = 0;
            (*cb_data).trace_output = core::ptr::null_mut();
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoll_safe(opt, arg, &mut auto_thresh) != 0 {
            return -1;
        }
        (*params).common.stop_us = auto_thresh;
        (*params).threshold = 1;

        if (*cb_data).trace_output.is_null() {
            (*cb_data).trace_output = c"osnoise_trace.txt".as_ptr() as *mut c_char;
        }
    }

    0
}

pub unsafe extern "C" fn opt_osnoise_trace_output_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let trace_output = unsafe { (*opt).value as *mut *const c_char };

    if unset != 0 {
        unsafe {
            *trace_output = core::ptr::null();
        }
        return 0;
    }

    unsafe {
        if arg.is_null() {
            *trace_output = c"osnoise_trace.txt".as_ptr();
        } else {
            *trace_output = arg;
            if !(*trace_output).is_null() && **trace_output == b'=' as c_char {
                /* Allow -t=<trace_output> next to -t[ ]<trace_output> */
                *trace_output = (*trace_output).add(1);
            }
        }
    }

    0
}

pub unsafe extern "C" fn opt_osnoise_on_threshold_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let actions = unsafe { (*opt).value as *mut actions };
    let retval: c_int;

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        retval = actions_parse(actions, arg as *mut c_char, c"osnoise_trace.txt".as_ptr());
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid action".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_osnoise_on_end_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let actions = unsafe { (*opt).value as *mut actions };
    let retval: c_int;

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        retval = actions_parse(actions, arg as *mut c_char, c"osnoise_trace.txt".as_ptr());
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid action".as_ptr());
            return -1;
        }
    }

    0
}

/*
 * Macros for command line options specific to timerlat
 */
macro_rules! TIMERLAT_OPT_PERIOD {
    () => {
        RTLA_OPT_LLONG_DATA!(
            b'p' as c_int,
            c"period".as_ptr(),
            &mut (*params).timerlat_period_us,
            c"us".as_ptr(),
            c"timerlat period in us".as_ptr(),
            LLONG_RANGE!(100, 1000000)
        )
    };
}

macro_rules! TIMERLAT_OPT_STACK {
    () => {
        RTLA_OPT_LLONG!(
            b's' as c_int,
            c"stack".as_ptr(),
            &mut (*params).print_stack,
            c"us".as_ptr(),
            c"save the stack trace at the IRQ if a thread latency is higher than the argument in us".as_ptr()
        )
    };
}

macro_rules! TIMERLAT_OPT_NANO {
    () => {
        OPT_CALLBACK_NOOPT!(
            b'n' as c_int,
            c"nano".as_ptr(),
            params,
            core::ptr::null(),
            c"display data in nanoseconds".as_ptr(),
            opt_nano_cb
        )
    };
}

macro_rules! TIMERLAT_OPT_DMA_LATENCY {
    () => {
        RTLA_OPT_INT_DATA_DEFVAL!(
            0,
            c"dma-latency".as_ptr(),
            &mut (*params).dma_latency,
            c"us".as_ptr(),
            c"set /dev/cpu_dma_latency latency <us> to reduce exit from idle latency".as_ptr(),
            INT_RANGE!(0, 10000),
            default_dma_latency
        )
    };
}

macro_rules! TIMERLAT_OPT_DEEPEST_IDLE_STATE {
    () => {
        RTLA_OPT_INT_DATA_DEFVAL!(
            0,
            c"deepest-idle-state".as_ptr(),
            &mut (*params).deepest_idle_state,
            c"n".as_ptr(),
            c"only go down to idle state n on cpus used by timerlat to reduce exit from idle latency".as_ptr(),
            INT_RANGE!(-1, INT_MAX),
            default_deepest_idle_state
        )
    };
}

macro_rules! TIMERLAT_OPT_AA_ONLY {
    () => {
        OPT_CALLBACK!(
            0,
            c"aa-only".as_ptr(),
            params,
            c"us".as_ptr(),
            c"stop if <us> latency is hit, only printing the auto analysis (reduces CPU usage)".as_ptr(),
            opt_aa_only_cb
        )
    };
}

macro_rules! TIMERLAT_OPT_NO_AA {
    () => {
        OPT_BOOLEAN!(
            0,
            c"no-aa".as_ptr(),
            &mut (*params).no_aa,
            c"disable auto-analysis, reducing rtla timerlat cpu usage".as_ptr()
        )
    };
}

macro_rules! TIMERLAT_OPT_DUMPS_TASKS {
    () => {
        OPT_BOOLEAN!(
            0,
            c"dump-tasks".as_ptr(),
            &mut (*params).dump_tasks,
            c"prints the task running on all CPUs if stop conditions are met (depends on !--no-aa)".as_ptr()
        )
    };
}

macro_rules! TIMERLAT_OPT_BPF_ACTION {
    () => {
        OPT_STRING!(
            0,
            c"bpf-action".as_ptr(),
            &mut (*params).bpf_action_program,
            c"program".as_ptr(),
            c"load and execute BPF program when latency threshold is exceeded".as_ptr()
        )
    };
}

macro_rules! TIMERLAT_OPT_STACK_FORMAT {
    () => {
        OPT_CALLBACK!(
            0,
            c"stack-format".as_ptr(),
            &mut (*params).stack_format,
            c"format".as_ptr(),
            c"set the stack format (truncate, skip, full)".as_ptr(),
            opt_stack_format_cb
        )
    };
}

macro_rules! TIMERLAT_OPT_ALIGNED {
    () => {
        RTLA_OPT_CALLBACK_DATA!(
            b'A' as c_int,
            c"aligned".as_ptr(),
            params,
            c"us".as_ptr(),
            c"align thread wakeups to a specific offset".as_ptr(),
            opt_timerlat_align_cb,
            LLONG_RANGE!(0, LLONG_MAX)
        )
    };
}

pub(crate) use TIMERLAT_OPT_AA_ONLY;
pub(crate) use TIMERLAT_OPT_ALIGNED;
pub(crate) use TIMERLAT_OPT_BPF_ACTION;
pub(crate) use TIMERLAT_OPT_DEEPEST_IDLE_STATE;
pub(crate) use TIMERLAT_OPT_DMA_LATENCY;
pub(crate) use TIMERLAT_OPT_DUMPS_TASKS;
pub(crate) use TIMERLAT_OPT_NANO;
pub(crate) use TIMERLAT_OPT_NO_AA;
pub(crate) use TIMERLAT_OPT_PERIOD;
pub(crate) use TIMERLAT_OPT_STACK;
pub(crate) use TIMERLAT_OPT_STACK_FORMAT;

/*
 * Callback functions for command line options for timerlat tools
 */

pub unsafe extern "C" fn opt_timerlat_auto_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let cb_data = unsafe { (*opt).value as *mut timerlat_cb_data };
    let params = unsafe { (*cb_data).params };
    let mut auto_thresh: c_longlong = 0;

    if unset != 0 {
        unsafe {
            (*params).common.stop_total_us = 0;
            (*params).common.stop_us = 0;
            (*params).print_stack = 0;
            (*cb_data).trace_output = core::ptr::null_mut();
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoll_safe(opt, arg, &mut auto_thresh) != 0 {
            return -1;
        }
        (*params).common.stop_total_us = auto_thresh;
        (*params).common.stop_us = auto_thresh;
        (*params).print_stack = auto_thresh;

        if (*cb_data).trace_output.is_null() {
            (*cb_data).trace_output = c"timerlat_trace.txt".as_ptr() as *mut c_char;
        }
    }

    0
}

pub unsafe extern "C" fn opt_aa_only_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut timerlat_params };
    let mut auto_thresh: c_longlong = 0;

    if unset != 0 {
        unsafe {
            (*params).common.stop_total_us = 0;
            (*params).common.stop_us = 0;
            (*params).print_stack = 0;
            (*params).common.aa_only = 0;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoll_safe(opt, arg, &mut auto_thresh) != 0 {
            return -1;
        }
        (*params).common.stop_total_us = auto_thresh;
        (*params).common.stop_us = auto_thresh;
        (*params).print_stack = auto_thresh;
        (*params).common.aa_only = 1;
    }

    0
}

pub unsafe extern "C" fn opt_timerlat_trace_output_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let trace_output = unsafe { (*opt).value as *mut *const c_char };

    if unset != 0 {
        unsafe {
            *trace_output = core::ptr::null();
        }
        return 0;
    }

    unsafe {
        if arg.is_null() {
            *trace_output = c"timerlat_trace.txt".as_ptr();
        } else {
            *trace_output = arg;
            if !(*trace_output).is_null() && **trace_output == b'=' as c_char {
                /* Allow -t=<trace_output> next to -t[ ]<trace_output> */
                *trace_output = (*trace_output).add(1);
            }
        }
    }

    0
}

pub unsafe extern "C" fn opt_timerlat_on_threshold_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let actions = unsafe { (*opt).value as *mut actions };
    let retval: c_int;

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        retval = actions_parse(actions, arg as *mut c_char, c"timerlat_trace.txt".as_ptr());
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid action".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_timerlat_on_end_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let actions = unsafe { (*opt).value as *mut actions };
    let retval: c_int;

    if unset != 0 || arg.is_null() {
        return -1;
    }

    unsafe {
        retval = actions_parse(actions, arg as *mut c_char, c"timerlat_trace.txt".as_ptr());
        if retval != 0 {
            opt_err(opt, arg, c"is not a valid action".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_user_threads_cb(
    opt: *const option,
    _arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut timerlat_params };

    if unset != 0 {
        unsafe {
            (*params).common.user_workload = false;
            (*params).common.user_data = false;
        }
        return 0;
    }

    unsafe {
        (*params).common.user_workload = true;
        (*params).common.user_data = true;
    }

    0
}

pub unsafe extern "C" fn opt_nano_cb(
    opt: *const option,
    _arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut timerlat_params };

    if unset != 0 {
        unsafe {
            (*params).common.output_divisor = default_output_divisor;
        }
        return 0;
    }

    unsafe {
        (*params).common.output_divisor = 1;
    }

    0
}

pub unsafe extern "C" fn opt_stack_format_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let format = unsafe { (*opt).value as *mut c_int };

    if unset != 0 {
        unsafe {
            *format = default_stack_format as c_int;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        *format = parse_stack_format(arg as *mut c_char);

        if *format == -1 {
            opt_err(opt, arg, c"is not a valid stack format".as_ptr());
            return -1;
        }
    }

    0
}

pub unsafe extern "C" fn opt_timerlat_align_cb(
    opt: *const option,
    arg: *const c_char,
    unset: c_int,
) -> c_int {
    let params = unsafe { (*opt).value as *mut timerlat_params };
    let mut val: c_longlong = 0;

    if unset != 0 {
        unsafe {
            (*params).timerlat_align = false;
            (*params).timerlat_align_us = 0;
        }
        return 0;
    }

    if arg.is_null() {
        return -1;
    }

    unsafe {
        if strtoll_safe(opt, arg, &mut val) != 0 {
            return -1;
        }
        if check_llong_range(opt, val) != 0 {
            return -1;
        }

        (*params).timerlat_align = true;
        (*params).timerlat_align_us = val;
    }

    0
}

/*
 * Macros for command line options specific to histogram-based tools
 */

macro_rules! HIST_OPT_BUCKET_SIZE {
    () => {
        RTLA_OPT_INT_DATA_DEFVAL!(
            b'b' as c_int,
            c"bucket-size".as_ptr(),
            &mut (*params).common.hist.bucket_size,
            c"N".as_ptr(),
            c"set the histogram bucket size (default 1)".as_ptr(),
            INT_RANGE!(1, 999999),
            default_bucket_size
        )
    };
}

macro_rules! HIST_OPT_ENTRIES {
    () => {
        RTLA_OPT_INT_DATA_DEFVAL!(
            b'E' as c_int,
            c"entries".as_ptr(),
            &mut (*params).common.hist.entries,
            c"N".as_ptr(),
            c"set the number of entries of the histogram (default 256)".as_ptr(),
            INT_RANGE!(10, 9999999),
            default_entries
        )
    };
}

macro_rules! HIST_OPT_NO_IRQ {
    () => {
        OPT_BOOLEAN_FLAG!(
            0,
            c"no-irq".as_ptr(),
            &mut (*params).common.hist.no_irq,
            c"ignore IRQ latencies".as_ptr(),
            PARSE_OPT_NOAUTONEG
        )
    };
}

macro_rules! HIST_OPT_NO_THREAD {
    () => {
        OPT_BOOLEAN_FLAG!(
            0,
            c"no-thread".as_ptr(),
            &mut (*params).common.hist.no_thread,
            c"ignore thread latencies".as_ptr(),
            PARSE_OPT_NOAUTONEG
        )
    };
}

macro_rules! HIST_OPT_NO_HEADER {
    () => {
        OPT_BOOLEAN!(
            0,
            c"no-header".as_ptr(),
            &mut (*params).common.hist.no_header,
            c"do not print header".as_ptr()
        )
    };
}

macro_rules! HIST_OPT_NO_SUMMARY {
    () => {
        OPT_BOOLEAN!(
            0,
            c"no-summary".as_ptr(),
            &mut (*params).common.hist.no_summary,
            c"do not print summary".as_ptr()
        )
    };
}

macro_rules! HIST_OPT_NO_INDEX {
    () => {
        OPT_BOOLEAN!(
            0,
            c"no-index".as_ptr(),
            &mut (*params).common.hist.no_index,
            c"do not print index".as_ptr()
        )
    };
}

macro_rules! HIST_OPT_WITH_ZEROS {
    () => {
        OPT_BOOLEAN!(
            0,
            c"with-zeros".as_ptr(),
            &mut (*params).common.hist.with_zeros,
            c"print zero only entries".as_ptr()
        )
    };
}

pub(crate) use HIST_OPT_BUCKET_SIZE;
pub(crate) use HIST_OPT_ENTRIES;
pub(crate) use HIST_OPT_NO_HEADER;
pub(crate) use HIST_OPT_NO_INDEX;
pub(crate) use HIST_OPT_NO_IRQ;
pub(crate) use HIST_OPT_NO_SUMMARY;
pub(crate) use HIST_OPT_NO_THREAD;
pub(crate) use HIST_OPT_WITH_ZEROS;
