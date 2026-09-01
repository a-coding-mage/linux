// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

use core::ffi::{c_char, c_double, c_int, c_uint, c_ulonglong, c_void};

const DEFAULT_SAMPLE_PERIOD: c_ulonglong = 1000000; /* 1s */
const DEFAULT_SAMPLE_RUNTIME: c_ulonglong = 1000000; /* 1s */
const BUFF_U64_STR_SIZE: usize = 32;
const OSNOISE_TIME_INIT_VAL: c_ulonglong = c_ulonglong::MAX;
const OSNOISE_OPTION_INIT_VAL: c_longlong = -1;
const UINT64_MAX: c_ulonglong = c_ulonglong::MAX;
const FLAG_CONTEXT_NEWLY_CREATED: c_int = 1 << 0;
const FLAG_CONTEXT_DELETED: c_int = 1 << 1;

type c_longlong = i64;

#[repr(C)]
pub struct tracefs_instance {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trace_instance {
    pub inst: *mut tracefs_instance,
    pub missed_events: c_ulonglong,
    pub processed_events: c_ulonglong,
}

#[repr(C)]
pub struct osnoise_context {
    pub curr_cpus: *mut c_char,
    pub orig_cpus: *mut c_char,
    pub runtime_us: c_ulonglong,
    pub orig_runtime_us: c_ulonglong,
    pub period_us: c_ulonglong,
    pub orig_period_us: c_ulonglong,
    pub timerlat_period_us: c_longlong,
    pub orig_timerlat_period_us: c_longlong,
    pub timerlat_align_us: c_longlong,
    pub orig_timerlat_align_us: c_longlong,
    pub stop_us: c_longlong,
    pub orig_stop_us: c_longlong,
    pub stop_total_us: c_longlong,
    pub orig_stop_total_us: c_longlong,
    pub print_stack: c_longlong,
    pub orig_print_stack: c_longlong,
    pub tracing_thresh: c_longlong,
    pub orig_tracing_thresh: c_longlong,
    pub opt_irq_disable: c_int,
    pub orig_opt_irq_disable: c_int,
    pub opt_workload: c_int,
    pub orig_opt_workload: c_int,
    pub opt_timerlat_align: c_int,
    pub orig_opt_timerlat_align: c_int,
    pub flags: c_int,
    pub ref_: c_int,
}

#[repr(C)]
pub struct common_params {
    pub kernel_workload: bool,
    pub warmup: c_uint,
}

#[repr(C)]
pub struct osnoise_params {
    pub common: common_params,
    pub runtime: c_ulonglong,
    pub period: c_ulonglong,
    pub threshold: c_longlong,
}

#[repr(C)]
pub struct osnoise_tool {
    pub trace: trace_instance,
    pub context: *mut osnoise_context,
    pub record: *mut osnoise_tool,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct tool_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stop_tracing: bool;
    static osnoise_top_ops: tool_ops;
    static osnoise_hist_ops: tool_ops;
    static mut stderr: *mut c_void;

    fn tracefs_instance_file_read(
        inst: *mut tracefs_instance,
        file: *const c_char,
        psize: *mut c_void,
    ) -> *mut c_char;
    fn tracefs_instance_file_write(
        inst: *mut tracefs_instance,
        file: *const c_char,
        str_: *const c_char,
    ) -> c_int;
    fn get_llong_from_str(str_: *const c_char) -> c_longlong;
    fn debug_msg(fmt: *const c_char, ...);
    fn err_msg(fmt: *const c_char, ...);
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn calloc_fatal(nmemb: usize, size: usize) -> *mut c_void;
    fn trace_instance_destroy(trace: *mut trace_instance);
    fn trace_instance_init(trace: *mut trace_instance, tool_name: *mut c_char) -> c_int;
    fn tracefs_event_enable(
        inst: *mut tracefs_instance,
        system: *const c_char,
        event: *const c_char,
    ) -> c_int;
    fn enable_tracer_by_name(inst: *mut tracefs_instance, tracer: *const c_char) -> c_int;
    fn tracefs_trace_is_on(inst: *mut tracefs_instance) -> bool;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn common_apply_config(tool: *mut osnoise_tool, params: *mut common_params) -> c_int;
    fn to_osnoise_params(params: *mut c_void) -> *mut osnoise_params;
    fn trace_instance_start(trace: *mut trace_instance);
    fn sleep(seconds: c_uint) -> c_uint;
    fn osn_set_stop(tool: *mut osnoise_tool) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn run_tool(ops: *const tool_ops, argc: c_int, argv: *mut *mut c_char);
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
}

/*
 * osnoise_get_cpus - return the original "osnoise/cpus" content
 *
 * It also saves the value to be restored.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_get_cpus(context: *mut osnoise_context) -> *mut c_char {
    unsafe {
        if !(*context).curr_cpus.is_null() {
            return (*context).curr_cpus;
        }

        if !(*context).orig_cpus.is_null() {
            return (*context).orig_cpus;
        }

        (*context).orig_cpus =
            tracefs_instance_file_read(core::ptr::null_mut(), c"osnoise/cpus".as_ptr(), core::ptr::null_mut());

        /*
         * The error value (NULL) is the same for tracefs_instance_file_read()
         * and this functions, so:
         */
        (*context).orig_cpus
    }
}

/*
 * osnoise_set_cpus - configure osnoise to run on *cpus
 *
 * "osnoise/cpus" file is used to set the cpus in which osnoise/timerlat
 * will run. This function opens this file, saves the current value,
 * and set the cpus passed as argument.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_set_cpus(
    context: *mut osnoise_context,
    cpus: *mut c_char,
) -> c_int {
    unsafe {
        let orig_cpus = osnoise_get_cpus(context);
        let mut buffer = [0 as c_char; 1024];
        let retval: c_int;

        if orig_cpus.is_null() {
            return -1;
        }

        (*context).curr_cpus = strdup(cpus);
        if (*context).curr_cpus.is_null() {
            return -1;
        }

        snprintf(buffer.as_mut_ptr(), buffer.len(), c"%s\n".as_ptr(), cpus);

        debug_msg(
            c"setting cpus to %s from %s".as_ptr(),
            cpus,
            (*context).orig_cpus,
        );

        retval = tracefs_instance_file_write(core::ptr::null_mut(), c"osnoise/cpus".as_ptr(), buffer.as_ptr());
        if retval < 0 {
            free((*context).curr_cpus as *mut c_void);
            (*context).curr_cpus = core::ptr::null_mut();
            return -1;
        }

        0
    }
}

/*
 * osnoise_restore_cpus - restore the original "osnoise/cpus"
 *
 * osnoise_set_cpus() saves the original data for the "osnoise/cpus"
 * file. This function restore the original config it was previously
 * modified.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_restore_cpus(context: *mut osnoise_context) {
    unsafe {
        let retval: c_int;

        if (*context).orig_cpus.is_null() {
            return;
        }

        if (*context).curr_cpus.is_null() {
            return;
        }

        /* nothing to do? */
        if strcmp((*context).orig_cpus, (*context).curr_cpus) == 0 {
            free((*context).curr_cpus as *mut c_void);
            (*context).curr_cpus = core::ptr::null_mut();
            return;
        }

        debug_msg(c"restoring cpus to %s".as_ptr(), (*context).orig_cpus);

        retval = tracefs_instance_file_write(
            core::ptr::null_mut(),
            c"osnoise/cpus".as_ptr(),
            (*context).orig_cpus,
        );
        if retval < 0 {
            err_msg(c"could not restore original osnoise cpus\n".as_ptr());
        }

        free((*context).curr_cpus as *mut c_void);
        (*context).curr_cpus = core::ptr::null_mut();
    }
}

/*
 * osnoise_put_cpus - restore cpus config and cleanup data
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_put_cpus(context: *mut osnoise_context) {
    unsafe {
        osnoise_restore_cpus(context);

        if (*context).orig_cpus.is_null() {
            return;
        }

        free((*context).orig_cpus as *mut c_void);
        (*context).orig_cpus = core::ptr::null_mut();
    }
}

/*
 * osnoise_read_ll_config - read a long long value from a config
 *
 * returns -1 on error.
 */
unsafe extern "C" fn osnoise_read_ll_config(rel_path: *mut c_char) -> c_longlong {
    unsafe {
        let retval: c_longlong;
        let buffer: *mut c_char;

        buffer = tracefs_instance_file_read(core::ptr::null_mut(), rel_path, core::ptr::null_mut());
        if buffer.is_null() {
            return -1;
        }

        /* get_llong_from_str returns -1 on error */
        retval = get_llong_from_str(buffer);

        debug_msg(c"reading %s returned %lld\n".as_ptr(), rel_path, retval);

        free(buffer as *mut c_void);

        retval
    }
}

/*
 * osnoise_write_ll_config - write a long long value to a config in rel_path
 *
 * returns -1 on error.
 */
unsafe extern "C" fn osnoise_write_ll_config(
    rel_path: *mut c_char,
    value: c_longlong,
) -> c_longlong {
    unsafe {
        let mut buffer = [0 as c_char; BUFF_U64_STR_SIZE];
        let retval: c_longlong;

        snprintf(buffer.as_mut_ptr(), buffer.len(), c"%lld\n".as_ptr(), value);

        debug_msg(c"setting %s to %lld\n".as_ptr(), rel_path, value);

        retval = tracefs_instance_file_write(core::ptr::null_mut(), rel_path, buffer.as_ptr()) as c_longlong;
        retval
    }
}

/*
 * osnoise_get_runtime - return the original "osnoise/runtime_us" value
 *
 * It also saves the value to be restored.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_get_runtime(context: *mut osnoise_context) -> c_ulonglong {
    unsafe {
        let runtime_us: c_longlong;

        if (*context).runtime_us != OSNOISE_TIME_INIT_VAL {
            return (*context).runtime_us;
        }

        if (*context).orig_runtime_us != OSNOISE_TIME_INIT_VAL {
            return (*context).orig_runtime_us;
        }

        runtime_us = osnoise_read_ll_config(c"osnoise/runtime_us".as_ptr() as *mut c_char);
        if runtime_us < 0 {
            return OSNOISE_TIME_INIT_VAL;
        }

        (*context).orig_runtime_us = runtime_us as c_ulonglong;
        runtime_us as c_ulonglong
    }
}

/*
 * osnoise_get_period - return the original "osnoise/period_us" value
 *
 * It also saves the value to be restored.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_get_period(context: *mut osnoise_context) -> c_ulonglong {
    unsafe {
        let period_us: c_longlong;

        if (*context).period_us != OSNOISE_TIME_INIT_VAL {
            return (*context).period_us;
        }

        if (*context).orig_period_us != OSNOISE_TIME_INIT_VAL {
            return (*context).orig_period_us;
        }

        period_us = osnoise_read_ll_config(c"osnoise/period_us".as_ptr() as *mut c_char);
        if period_us < 0 {
            return OSNOISE_TIME_INIT_VAL;
        }

        (*context).orig_period_us = period_us as c_ulonglong;
        period_us as c_ulonglong
    }
}

unsafe extern "C" fn __osnoise_write_runtime(
    context: *mut osnoise_context,
    runtime: c_ulonglong,
) -> c_int {
    unsafe {
        let retval: c_int;

        if (*context).orig_runtime_us == OSNOISE_TIME_INIT_VAL {
            return -1;
        }

        retval = osnoise_write_ll_config(c"osnoise/runtime_us".as_ptr() as *mut c_char, runtime as c_longlong) as c_int;
        if retval < 0 {
            return -1;
        }

        (*context).runtime_us = runtime;
        0
    }
}

unsafe extern "C" fn __osnoise_write_period(
    context: *mut osnoise_context,
    period: c_ulonglong,
) -> c_int {
    unsafe {
        let retval: c_int;

        if (*context).orig_period_us == OSNOISE_TIME_INIT_VAL {
            return -1;
        }

        retval = osnoise_write_ll_config(c"osnoise/period_us".as_ptr() as *mut c_char, period as c_longlong) as c_int;
        if retval < 0 {
            return -1;
        }

        (*context).period_us = period;
        0
    }
}

/*
 * osnoise_set_runtime_period - set osnoise runtime and period
 *
 * Osnoise's runtime and period are related as runtime <= period.
 * Thus, this function saves the original values, and then tries
 * to set the runtime and period if they are != 0.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_set_runtime_period(
    context: *mut osnoise_context,
    runtime: c_ulonglong,
    period: c_ulonglong,
) -> c_int {
    unsafe {
        let curr_runtime_us: c_ulonglong;
        let curr_period_us: c_ulonglong;
        let mut retval: c_int;

        if period == 0 && runtime == 0 {
            return 0;
        }

        curr_runtime_us = osnoise_get_runtime(context);
        curr_period_us = osnoise_get_period(context);

        /* error getting any value? */
        if curr_period_us == OSNOISE_TIME_INIT_VAL || curr_runtime_us == OSNOISE_TIME_INIT_VAL {
            return -1;
        }

        if period == 0 {
            if runtime > curr_period_us {
                return -1;
            }
            return __osnoise_write_runtime(context, runtime);
        } else if runtime == 0 {
            if period < curr_runtime_us {
                return -1;
            }
            return __osnoise_write_period(context, period);
        }

        if runtime > curr_period_us {
            retval = __osnoise_write_period(context, period);
            if retval != 0 {
                return -1;
            }
            retval = __osnoise_write_runtime(context, runtime);
            if retval != 0 {
                return -1;
            }
        } else {
            retval = __osnoise_write_runtime(context, runtime);
            if retval != 0 {
                return -1;
            }
            retval = __osnoise_write_period(context, period);
            if retval != 0 {
                return -1;
            }
        }

        0
    }
}

/*
 * osnoise_restore_runtime_period - restore the original runtime and period
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_restore_runtime_period(context: *mut osnoise_context) {
    unsafe {
        let orig_runtime = (*context).orig_runtime_us;
        let orig_period = (*context).orig_period_us;
        let curr_runtime = (*context).runtime_us;
        let curr_period = (*context).period_us;
        let retval: c_int;

        if orig_runtime == OSNOISE_TIME_INIT_VAL && orig_period == OSNOISE_TIME_INIT_VAL {
            return;
        }

        if orig_period == curr_period && orig_runtime == curr_runtime {
            (*context).runtime_us = OSNOISE_TIME_INIT_VAL;
            (*context).period_us = OSNOISE_TIME_INIT_VAL;
            return;
        }

        retval = osnoise_set_runtime_period(context, orig_runtime, orig_period);
        if retval != 0 {
            err_msg(c"Could not restore original osnoise runtime/period\n".as_ptr());
        }

        (*context).runtime_us = OSNOISE_TIME_INIT_VAL;
        (*context).period_us = OSNOISE_TIME_INIT_VAL;
    }
}

/*
 * osnoise_put_runtime_period - restore original values and cleanup data
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_put_runtime_period(context: *mut osnoise_context) {
    unsafe {
        osnoise_restore_runtime_period(context);

        if (*context).orig_runtime_us != OSNOISE_TIME_INIT_VAL {
            (*context).orig_runtime_us = OSNOISE_TIME_INIT_VAL;
        }

        if (*context).orig_period_us != OSNOISE_TIME_INIT_VAL {
            (*context).orig_period_us = OSNOISE_TIME_INIT_VAL;
        }
    }
}

/*
 * osnoise_get_timerlat_period_us - read and save the original "timerlat_period_us"
 */
unsafe extern "C" fn osnoise_get_timerlat_period_us(
    context: *mut osnoise_context,
) -> c_longlong {
    unsafe {
        let timerlat_period_us: c_longlong;

        if (*context).timerlat_period_us != OSNOISE_TIME_INIT_VAL as c_longlong {
            return (*context).timerlat_period_us;
        }

        if (*context).orig_timerlat_period_us != OSNOISE_TIME_INIT_VAL as c_longlong {
            return (*context).orig_timerlat_period_us;
        }

        timerlat_period_us =
            osnoise_read_ll_config(c"osnoise/timerlat_period_us".as_ptr() as *mut c_char);
        if timerlat_period_us < 0 {
            return OSNOISE_TIME_INIT_VAL as c_longlong;
        }

        (*context).orig_timerlat_period_us = timerlat_period_us;
        timerlat_period_us
    }
}

/*
 * osnoise_set_timerlat_period_us - set "timerlat_period_us"
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_set_timerlat_period_us(
    context: *mut osnoise_context,
    timerlat_period_us: c_longlong,
) -> c_int {
    unsafe {
        let curr_timerlat_period_us = osnoise_get_timerlat_period_us(context);
        let retval: c_int;

        if curr_timerlat_period_us == OSNOISE_TIME_INIT_VAL as c_longlong {
            return -1;
        }

        retval = osnoise_write_ll_config(
            c"osnoise/timerlat_period_us".as_ptr() as *mut c_char,
            timerlat_period_us,
        ) as c_int;
        if retval < 0 {
            return -1;
        }

        (*context).timerlat_period_us = timerlat_period_us;

        0
    }
}

/*
 * osnoise_restore_timerlat_period_us - restore "timerlat_period_us"
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_restore_timerlat_period_us(context: *mut osnoise_context) {
    unsafe {
        let retval: c_int;

        if (*context).orig_timerlat_period_us == OSNOISE_TIME_INIT_VAL as c_longlong {
            return;
        }

        if (*context).orig_timerlat_period_us == (*context).timerlat_period_us {
            (*context).timerlat_period_us = OSNOISE_TIME_INIT_VAL as c_longlong;
            return;
        }

        retval = osnoise_write_ll_config(
            c"osnoise/timerlat_period_us".as_ptr() as *mut c_char,
            (*context).orig_timerlat_period_us,
        ) as c_int;
        if retval < 0 {
            err_msg(c"Could not restore original osnoise timerlat_period_us\n".as_ptr());
        }

        (*context).timerlat_period_us = OSNOISE_TIME_INIT_VAL as c_longlong;
    }
}

/*
 * osnoise_put_timerlat_period_us - restore original values and cleanup data
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_put_timerlat_period_us(context: *mut osnoise_context) {
    unsafe {
        osnoise_restore_timerlat_period_us(context);

        if (*context).orig_timerlat_period_us == OSNOISE_TIME_INIT_VAL as c_longlong {
            return;
        }

        (*context).orig_timerlat_period_us = OSNOISE_TIME_INIT_VAL as c_longlong;
    }
}

unsafe extern "C" fn osnoise_get_timerlat_align_us(
    context: *mut osnoise_context,
) -> c_longlong {
    unsafe {
        if (*context).timerlat_align_us != OSNOISE_OPTION_INIT_VAL {
            return (*context).timerlat_align_us;
        }
        if (*context).orig_timerlat_align_us != OSNOISE_OPTION_INIT_VAL {
            return (*context).orig_timerlat_align_us;
        }
        let timerlat_align_us =
            osnoise_read_ll_config(c"osnoise/timerlat_align_us".as_ptr() as *mut c_char);
        if timerlat_align_us < 0 {
            return OSNOISE_OPTION_INIT_VAL;
        }
        (*context).orig_timerlat_align_us = timerlat_align_us;
        timerlat_align_us
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_set_timerlat_align_us(
    context: *mut osnoise_context,
    timerlat_align_us: c_longlong,
) -> c_int {
    unsafe {
        let curr_timerlat_align_us = osnoise_get_timerlat_align_us(context);
        let retval: c_int;
        if curr_timerlat_align_us == OSNOISE_OPTION_INIT_VAL {
            return -1;
        }
        retval = osnoise_write_ll_config(
            c"osnoise/timerlat_align_us".as_ptr() as *mut c_char,
            timerlat_align_us,
        ) as c_int;
        if retval < 0 {
            return -1;
        }
        (*context).timerlat_align_us = timerlat_align_us;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_restore_timerlat_align_us(context: *mut osnoise_context) {
    unsafe {
        let retval: c_int;
        if (*context).orig_timerlat_align_us == OSNOISE_OPTION_INIT_VAL {
            return;
        }
        if (*context).orig_timerlat_align_us == (*context).timerlat_align_us {
            (*context).timerlat_align_us = OSNOISE_OPTION_INIT_VAL;
            return;
        }
        retval = osnoise_write_ll_config(
            c"osnoise/timerlat_align_us".as_ptr() as *mut c_char,
            (*context).orig_timerlat_align_us,
        ) as c_int;
        if retval < 0 {
            err_msg(c"Could not restore original osnoise timerlat_align_us\n".as_ptr());
        }
        (*context).timerlat_align_us = OSNOISE_OPTION_INIT_VAL;
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_put_timerlat_align_us(context: *mut osnoise_context) {
    unsafe {
        osnoise_restore_timerlat_align_us(context);
        if (*context).orig_timerlat_align_us == OSNOISE_OPTION_INIT_VAL {
            return;
        }
        (*context).orig_timerlat_align_us = OSNOISE_OPTION_INIT_VAL;
    }
}

macro_rules! ll_config_accessors {
    ($get_fn:ident, $set_fn:ident, $restore_fn:ident, $put_fn:ident,
     $field:ident, $orig_field:ident, $path:literal, $err:literal) => {
        unsafe extern "C" fn $get_fn(context: *mut osnoise_context) -> c_longlong {
            unsafe {
                if (*context).$field != OSNOISE_OPTION_INIT_VAL {
                    return (*context).$field;
                }
                if (*context).$orig_field != OSNOISE_OPTION_INIT_VAL {
                    return (*context).$orig_field;
                }
                let value = osnoise_read_ll_config(concat!($path, "\0").as_ptr() as *mut c_char);
                if value < 0 {
                    return OSNOISE_OPTION_INIT_VAL;
                }
                (*context).$orig_field = value;
                value
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $set_fn(
            context: *mut osnoise_context,
            value: c_longlong,
        ) -> c_int {
            unsafe {
                let curr_value = $get_fn(context);
                let retval: c_int;
                if curr_value == OSNOISE_OPTION_INIT_VAL {
                    return -1;
                }
                retval =
                    osnoise_write_ll_config(concat!($path, "\0").as_ptr() as *mut c_char, value)
                        as c_int;
                if retval < 0 {
                    return -1;
                }
                (*context).$field = value;
                0
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $restore_fn(context: *mut osnoise_context) {
            unsafe {
                let retval: c_int;
                if (*context).$orig_field == OSNOISE_OPTION_INIT_VAL {
                    return;
                }
                if (*context).$orig_field == (*context).$field {
                    (*context).$field = OSNOISE_OPTION_INIT_VAL;
                    return;
                }
                retval = osnoise_write_ll_config(
                    concat!($path, "\0").as_ptr() as *mut c_char,
                    (*context).$orig_field,
                ) as c_int;
                if retval < 0 {
                    err_msg(concat!($err, "\0").as_ptr() as *const c_char);
                }
                (*context).$field = OSNOISE_OPTION_INIT_VAL;
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $put_fn(context: *mut osnoise_context) {
            unsafe {
                $restore_fn(context);
                if (*context).$orig_field == OSNOISE_OPTION_INIT_VAL {
                    return;
                }
                (*context).$orig_field = OSNOISE_OPTION_INIT_VAL;
            }
        }
    };
}

/*
 * osnoise_get_stop_us - read and save the original "stop_tracing_us"
 */
ll_config_accessors!(
    osnoise_get_stop_us,
    osnoise_set_stop_us,
    osnoise_restore_stop_us,
    osnoise_put_stop_us,
    stop_us,
    orig_stop_us,
    "osnoise/stop_tracing_us",
    "Could not restore original osnoise stop_us\n"
);

/*
 * osnoise_get_stop_total_us - read and save the original "stop_tracing_total_us"
 */
ll_config_accessors!(
    osnoise_get_stop_total_us,
    osnoise_set_stop_total_us,
    osnoise_restore_stop_total_us,
    osnoise_put_stop_total_us,
    stop_total_us,
    orig_stop_total_us,
    "osnoise/stop_tracing_total_us",
    "Could not restore original osnoise stop_total_us\n"
);

/*
 * osnoise_get_print_stack - read and save the original "print_stack"
 */
ll_config_accessors!(
    osnoise_get_print_stack,
    osnoise_set_print_stack,
    osnoise_restore_print_stack,
    osnoise_put_print_stack,
    print_stack,
    orig_print_stack,
    "osnoise/print_stack",
    "Could not restore original osnoise print_stack\n"
);

/*
 * osnoise_get_tracing_thresh - read and save the original "tracing_thresh"
 */
ll_config_accessors!(
    osnoise_get_tracing_thresh,
    osnoise_set_tracing_thresh,
    osnoise_restore_tracing_thresh,
    osnoise_put_tracing_thresh,
    tracing_thresh,
    orig_tracing_thresh,
    "tracing_thresh",
    "Could not restore original tracing_thresh\n"
);

unsafe extern "C" fn osnoise_options_get_option(option: *mut c_char) -> c_int {
    unsafe {
        let options = tracefs_instance_file_read(
            core::ptr::null_mut(),
            c"osnoise/options".as_ptr(),
            core::ptr::null_mut(),
        );
        let mut no_option = [0 as c_char; 128];
        let mut retval: c_int = 0;
        let mut opt: *mut c_char;

        if options.is_null() {
            return OSNOISE_OPTION_INIT_VAL as c_int;
        }

        /*
         * Check first if the option is disabled.
         */
        snprintf(no_option.as_mut_ptr(), no_option.len(), c"NO_%s".as_ptr(), option);

        opt = strstr(options, no_option.as_ptr());
        if opt.is_null() {
            /*
             * Now that it is not disabled, if the string is there, it is
             * enabled. If the string is not there, the option does not exist.
             */
            opt = strstr(options, option);
            if !opt.is_null() {
                retval = 1;
            } else {
                retval = OSNOISE_OPTION_INIT_VAL as c_int;
            }
        }

        free(options as *mut c_void);
        retval
    }
}

unsafe extern "C" fn osnoise_options_set_option(option: *mut c_char, onoff: bool) -> c_int {
    unsafe {
        let mut no_option = [0 as c_char; 128];

        if onoff {
            return tracefs_instance_file_write(core::ptr::null_mut(), c"osnoise/options".as_ptr(), option);
        }

        snprintf(no_option.as_mut_ptr(), no_option.len(), c"NO_%s".as_ptr(), option);

        tracefs_instance_file_write(
            core::ptr::null_mut(),
            c"osnoise/options".as_ptr(),
            no_option.as_ptr(),
        )
    }
}

macro_rules! bool_option_accessors {
    ($get_fn:ident, $set_fn:ident, $restore_fn:ident, $put_fn:ident,
     $field:ident, $orig_field:ident, $option:literal, $set_err:expr, $restore_err:literal) => {
        unsafe extern "C" fn $get_fn(context: *mut osnoise_context) -> c_int {
            unsafe {
                if (*context).$field != OSNOISE_OPTION_INIT_VAL as c_int {
                    return (*context).$field;
                }
                if (*context).$orig_field != OSNOISE_OPTION_INIT_VAL as c_int {
                    return (*context).$orig_field;
                }
                (*context).$orig_field =
                    osnoise_options_get_option(concat!($option, "\0").as_ptr() as *mut c_char);
                (*context).$orig_field
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $set_fn(context: *mut osnoise_context, onoff: bool) -> c_int {
            unsafe {
                let opt_value = $get_fn(context);
                let retval: c_int;
                if opt_value == OSNOISE_OPTION_INIT_VAL as c_int {
                    return -1;
                }
                if opt_value == onoff as c_int {
                    return 0;
                }
                retval = osnoise_options_set_option(
                    concat!($option, "\0").as_ptr() as *mut c_char,
                    onoff,
                );
                if retval < 0 {
                    return $set_err;
                }
                (*context).$field = onoff as c_int;
                0
            }
        }

        unsafe extern "C" fn $restore_fn(context: *mut osnoise_context) {
            unsafe {
                let retval: c_int;
                if (*context).$orig_field == OSNOISE_OPTION_INIT_VAL as c_int {
                    return;
                }
                if (*context).$orig_field == (*context).$field {
                    (*context).$orig_field = OSNOISE_OPTION_INIT_VAL as c_int;
                    return;
                }
                retval = osnoise_options_set_option(
                    concat!($option, "\0").as_ptr() as *mut c_char,
                    (*context).$orig_field != 0,
                );
                if retval < 0 {
                    err_msg(concat!($restore_err, "\0").as_ptr() as *const c_char);
                }
                (*context).$orig_field = OSNOISE_OPTION_INIT_VAL as c_int;
            }
        }

        unsafe extern "C" fn $put_fn(context: *mut osnoise_context) {
            unsafe {
                $restore_fn(context);
                if (*context).$orig_field == OSNOISE_OPTION_INIT_VAL as c_int {
                    return;
                }
                (*context).$orig_field = OSNOISE_OPTION_INIT_VAL as c_int;
            }
        }
    };
}

bool_option_accessors!(
    osnoise_get_irq_disable,
    osnoise_set_irq_disable,
    osnoise_restore_irq_disable,
    osnoise_put_irq_disable,
    opt_irq_disable,
    orig_opt_irq_disable,
    "OSNOISE_IRQ_DISABLE",
    -1,
    "Could not restore original OSNOISE_IRQ_DISABLE option\n"
);

bool_option_accessors!(
    osnoise_get_workload,
    osnoise_set_workload,
    osnoise_restore_workload,
    osnoise_put_workload,
    opt_workload,
    orig_opt_workload,
    "OSNOISE_WORKLOAD",
    -2,
    "Could not restore original OSNOISE_WORKLOAD option\n"
);

bool_option_accessors!(
    osnoise_get_timerlat_align,
    osnoise_set_timerlat_align,
    osnoise_restore_timerlat_align,
    osnoise_put_timerlat_align,
    opt_timerlat_align,
    orig_opt_timerlat_align,
    "TIMERLAT_ALIGN",
    -2,
    "Could not restore original TIMERLAT_ALIGN option\n"
);

/*
 * osnoise_get_context - increase the usage of a context and return it
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_get_context(context: *mut osnoise_context) -> c_int {
    unsafe {
        let ret: c_int;

        if ((*context).flags & FLAG_CONTEXT_DELETED) != 0 {
            ret = -1;
        } else {
            (*context).ref_ += 1;
            ret = 0;
        }

        ret
    }
}

/*
 * osnoise_context_alloc - alloc an osnoise_context
 *
 * The osnoise context contains the information of the "osnoise/" configs.
 * It is used to set and restore the config.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_context_alloc() -> *mut osnoise_context {
    unsafe {
        let context: *mut osnoise_context;

        context = calloc_fatal(1, core::mem::size_of::<osnoise_context>()) as *mut osnoise_context;

        (*context).orig_stop_us = OSNOISE_OPTION_INIT_VAL;
        (*context).stop_us = OSNOISE_OPTION_INIT_VAL;

        (*context).orig_stop_total_us = OSNOISE_OPTION_INIT_VAL;
        (*context).stop_total_us = OSNOISE_OPTION_INIT_VAL;

        (*context).orig_print_stack = OSNOISE_OPTION_INIT_VAL;
        (*context).print_stack = OSNOISE_OPTION_INIT_VAL;

        (*context).orig_tracing_thresh = OSNOISE_OPTION_INIT_VAL;
        (*context).tracing_thresh = OSNOISE_OPTION_INIT_VAL;

        (*context).orig_opt_irq_disable = OSNOISE_OPTION_INIT_VAL as c_int;
        (*context).opt_irq_disable = OSNOISE_OPTION_INIT_VAL as c_int;

        (*context).orig_opt_workload = OSNOISE_OPTION_INIT_VAL as c_int;
        (*context).opt_workload = OSNOISE_OPTION_INIT_VAL as c_int;

        (*context).orig_opt_timerlat_align = OSNOISE_OPTION_INIT_VAL as c_int;
        (*context).opt_timerlat_align = OSNOISE_OPTION_INIT_VAL as c_int;

        (*context).orig_timerlat_align_us = OSNOISE_OPTION_INIT_VAL;
        (*context).timerlat_align_us = OSNOISE_OPTION_INIT_VAL;

        osnoise_get_context(context);

        context
    }
}

/*
 * osnoise_put_context - put the osnoise_put_context
 *
 * If there is no other user for the context, the original data
 * is restored.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_put_context(context: *mut osnoise_context) {
    unsafe {
        (*context).ref_ -= 1;
        if (*context).ref_ < 1 {
            (*context).flags |= FLAG_CONTEXT_DELETED;
        }

        if !(((*context).flags & FLAG_CONTEXT_DELETED) != 0) {
            return;
        }

        osnoise_put_cpus(context);
        osnoise_put_runtime_period(context);
        osnoise_put_stop_us(context);
        osnoise_put_stop_total_us(context);
        osnoise_put_timerlat_period_us(context);
        osnoise_put_print_stack(context);
        osnoise_put_tracing_thresh(context);
        osnoise_put_irq_disable(context);
        osnoise_put_workload(context);
        osnoise_put_timerlat_align(context);
        osnoise_put_timerlat_align_us(context);

        free(context as *mut c_void);
    }
}

/*
 * osnoise_destroy_tool - disable trace, restore configs and free data
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_destroy_tool(top: *mut osnoise_tool) {
    unsafe {
        if top.is_null() {
            return;
        }

        trace_instance_destroy(&mut (*top).trace);

        if !(*top).context.is_null() {
            osnoise_put_context((*top).context);
        }

        free(top as *mut c_void);
    }
}

/*
 * osnoise_init_tool - init an osnoise tool
 *
 * It allocs data, create a context to store data and
 * creates a new trace instance for the tool.
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_init_tool(tool_name: *mut c_char) -> *mut osnoise_tool {
    unsafe {
        let top: *mut osnoise_tool;

        top = calloc_fatal(1, core::mem::size_of::<osnoise_tool>()) as *mut osnoise_tool;
        (*top).context = osnoise_context_alloc();

        if trace_instance_init(&mut (*top).trace, tool_name) != 0 {
            osnoise_destroy_tool(top);
            return core::ptr::null_mut();
        }

        top
    }
}

/*
 * osnoise_init_trace_tool - init a tracer instance to trace osnoise events
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_init_trace_tool(tracer: *const c_char) -> *mut osnoise_tool {
    unsafe {
        let trace: *mut osnoise_tool;
        let mut retval: c_int;

        trace = osnoise_init_tool(c"osnoise_trace".as_ptr() as *mut c_char);
        if trace.is_null() {
            return core::ptr::null_mut();
        }

        retval = tracefs_event_enable((*trace).trace.inst, c"osnoise".as_ptr(), core::ptr::null());
        if retval < 0 && errno == 0 {
            err_msg(c"Could not find osnoise events\n".as_ptr());
            osnoise_destroy_tool(trace);
            return core::ptr::null_mut();
        }

        retval = enable_tracer_by_name((*trace).trace.inst, tracer);
        if retval != 0 {
            err_msg(c"Could not enable %s tracer for tracing\n".as_ptr(), tracer);
            osnoise_destroy_tool(trace);
            return core::ptr::null_mut();
        }

        trace
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_trace_is_off(
    tool: *mut osnoise_tool,
    record: *mut osnoise_tool,
) -> bool {
    unsafe {
        /*
         * The tool instance is always present, it is the one used to collect
         * data.
         */
        if !tracefs_trace_is_on((*tool).trace.inst) {
            return true;
        }

        /*
         * The trace record instance is only enabled when -t is set. IOW, when the system
         * is tracing.
         */
        !record.is_null() && !tracefs_trace_is_on((*record).trace.inst)
    }
}

/*
 * osnoise_report_missed_events - report number of events dropped by trace
 * buffer
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_report_missed_events(tool: *mut osnoise_tool) {
    unsafe {
        let total_events: c_ulonglong;

        if (*tool).trace.missed_events == UINT64_MAX {
            printf(c"unknown number of events missed, results might not be accurate\n".as_ptr());
        } else if (*tool).trace.missed_events > 0 {
            total_events = (*tool).trace.processed_events + (*tool).trace.missed_events;

            printf(
                c"%lld (%.2f%%) events missed, results might not be accurate\n".as_ptr(),
                (*tool).trace.missed_events,
                ((*tool).trace.missed_events as c_double / total_events as c_double) * 100.0,
            );
        }
    }
}

/*
 * osnoise_apply_config - apply osnoise configs to the initialized tool
 */
#[no_mangle]
pub unsafe extern "C" fn osnoise_apply_config(
    tool: *mut osnoise_tool,
    params: *mut osnoise_params,
) -> c_int {
    unsafe {
        let retval: c_int;

        (*params).common.kernel_workload = true;

        if (*params).runtime != 0 || (*params).period != 0 {
            retval = osnoise_set_runtime_period(
                (*tool).context,
                (*params).runtime,
                (*params).period,
            );
        } else {
            retval = osnoise_set_runtime_period(
                (*tool).context,
                DEFAULT_SAMPLE_PERIOD,
                DEFAULT_SAMPLE_RUNTIME,
            );
        }

        if retval != 0 {
            err_msg(c"Failed to set runtime and/or period\n".as_ptr());
            return -1;
        }

        let retval = osnoise_set_tracing_thresh((*tool).context, (*params).threshold);
        if retval != 0 {
            err_msg(c"Failed to set tracing_thresh\n".as_ptr());
            return -1;
        }

        common_apply_config(tool, &mut (*params).common)
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_enable(tool: *mut osnoise_tool) -> c_int {
    unsafe {
        let params = to_osnoise_params((*tool).params);
        let mut retval: c_int;

        /*
         * Start the tracer here, after having set all instances.
         *
         * Let the trace instance start first for the case of hitting a stop
         * tracing while enabling other instances. The trace instance is the
         * one with most valuable information.
         */
        if !(*tool).record.is_null() {
            trace_instance_start(&mut (*(*tool).record).trace);
        }
        trace_instance_start(&mut (*tool).trace);

        if (*params).common.warmup > 0 {
            debug_msg(c"Warming up for %d seconds\n".as_ptr(), (*params).common.warmup);
            sleep((*params).common.warmup);
            if stop_tracing {
                return -1;
            }

            /*
             * Clean up the buffer. The osnoise workload do not run
             * with tracing off to avoid creating a performance penalty
             * when not needed.
             */
            retval = tracefs_instance_file_write((*tool).trace.inst, c"trace".as_ptr(), c"".as_ptr());
            if retval < 0 {
                debug_msg(c"Error cleaning up the buffer".as_ptr());
                return retval;
            }
        }

        retval = osn_set_stop(tool);
        if retval != 0 {
            return retval;
        }

        0
    }
}

unsafe extern "C" fn osnoise_usage(err: c_int) -> ! {
    unsafe {
        let mut i: usize;
        /* C source concatenates the external VERSION macro into this string. */
        let msg: [*const c_char; 11] = [
            c"".as_ptr(),
            c"osnoise version ".as_ptr(),
            c"".as_ptr(),
            c"  usage: [rtla] osnoise [MODE] ...".as_ptr(),
            c"".as_ptr(),
            c"  modes:".as_ptr(),
            c"     top   - prints the summary from osnoise tracer".as_ptr(),
            c"     hist  - prints a histogram of osnoise samples".as_ptr(),
            c"".as_ptr(),
            c"if no MODE is given, the top mode is called, passing the arguments".as_ptr(),
            core::ptr::null(),
        ];

        i = 0;
        while !msg[i].is_null() {
            fprintf(stderr, c"%s\n".as_ptr(), msg[i]);
            i += 1;
        }
        exit(err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn osnoise_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        if argc == 0 {
            osnoise_usage(129);
        }

        /*
         * if osnoise was called without any argument, run the
         * default cmdline.
         */
        if argc == 1 {
            run_tool(&osnoise_top_ops, argc, argv);
            exit(0);
        }

        if strcmp(*argv.add(1), c"-h".as_ptr()) == 0
            || strcmp(*argv.add(1), c"--help".as_ptr()) == 0
        {
            osnoise_usage(129);
        } else if str_has_prefix(*argv.add(1), c"-".as_ptr()) {
            /* the user skipped the tool, call the default one */
            run_tool(&osnoise_top_ops, argc, argv);
            exit(0);
        } else if strcmp(*argv.add(1), c"top".as_ptr()) == 0 {
            run_tool(&osnoise_top_ops, argc - 1, argv.add(1));
            exit(0);
        } else if strcmp(*argv.add(1), c"hist".as_ptr()) == 0 {
            run_tool(&osnoise_hist_ops, argc - 1, argv.add(1));
            exit(0);
        }

        osnoise_usage(129);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hwnoise_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        run_tool(&osnoise_top_ops, argc, argv);
        exit(0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
