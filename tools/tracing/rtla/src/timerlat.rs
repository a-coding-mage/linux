// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Red Hat Inc, Daniel Bristot de Oliveira <bristot@kernel.org>
 */

// C dependencies: sys/types.h, sys/stat.h, pthread.h, stdlib.h, string.h,
// unistd.h, fcntl.h, stdio.h, sched.h, linux/compiler.h, timerlat.h,
// timerlat_aa.h, timerlat_bpf.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const DEFAULT_TIMERLAT_PERIOD: c_int = 1000; /* 1ms */

static mut dma_latency_fd: c_int = -1;

#[repr(C)]
pub struct osnoise_tool {
    pub trace: trace_instance,
    pub record: *mut osnoise_tool,
    pub aa: *mut osnoise_tool,
    pub context: *mut c_void,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct trace_instance {
    pub tep: *mut c_void,
    pub inst: *mut c_void,
}

#[repr(C)]
pub struct timerlat_params {
    pub mode: c_int,
    pub bpf_action_program: *const c_char,
    pub timerlat_period_us: c_int,
    pub print_stack: c_int,
    pub timerlat_align: c_int,
    pub timerlat_align_us: c_int,
    pub common: osnoise_params,
    pub dma_latency: c_int,
    pub deepest_idle_state: c_int,
    pub no_aa: c_int,
    pub dump_tasks: c_int,
    pub stack_format: c_int,
}

#[repr(C)]
pub struct osnoise_params {
    pub kernel_workload: c_int,
    pub user_data: c_int,
    pub user_workload: c_int,
    pub warmup: c_uint,
    pub stop_us: c_int,
    pub stop_total_us: c_int,
    pub aa_only: c_int,
}

#[repr(C)]
pub struct tool_ops {
    _private: [u8; 0],
}

pub const TRACING_MODE_TRACEFS: c_int = 0;
pub const TRACING_MODE_BPF: c_int = 1;

unsafe extern "C" {
    fn getenv(name: *const c_char) -> *const c_char;
    fn sleep(seconds: c_uint) -> c_uint;
    fn close(fd: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    static mut stderr: *mut c_void;
    static mut stop_tracing: c_int;
    static timerlat_top_ops: tool_ops;
    static timerlat_hist_ops: tool_ops;

    fn strncmp_static(a: *const c_char, b: *const c_char) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn str_has_prefix(str_: *const c_char, prefix: *const c_char) -> c_int;

    fn debug_msg(format: *const c_char, ...);
    fn err_msg(format: *const c_char, ...);

    fn to_timerlat_params(params: *mut c_void) -> *mut timerlat_params;
    fn tep_find_event_by_name(tep: *mut c_void, system: *const c_char, event: *const c_char) -> *mut c_void;
    fn timerlat_bpf_init(params: *mut timerlat_params) -> c_int;
    fn timerlat_load_bpf_action_program(program: *const c_char) -> c_int;
    fn osnoise_set_timerlat_period_us(context: *mut c_void, period: c_int) -> c_int;
    fn osnoise_set_print_stack(context: *mut c_void, print_stack: c_int) -> c_int;
    fn osnoise_set_timerlat_align(context: *mut c_void, align: c_int) -> c_int;
    fn osnoise_set_timerlat_align_us(context: *mut c_void, align_us: c_int) -> c_int;
    fn tracefs_file_exists(instance: *mut c_void, file: *const c_char) -> c_int;
    fn common_apply_config(tool: *mut osnoise_tool, params: *mut osnoise_params) -> c_int;

    fn set_cpu_dma_latency(latency: c_int) -> c_int;
    fn have_libcpupower_support() -> c_int;
    fn for_each_monitored_cpu_next(cpu: *mut c_int, common: *mut osnoise_params) -> c_int;
    fn save_cpu_idle_disable_state(cpu: c_int) -> c_int;
    fn set_deepest_cpu_idle_state(cpu: c_int, state: c_int) -> c_int;
    fn osnoise_init_tool(name: *const c_char) -> *mut osnoise_tool;
    fn timerlat_aa_init(tool: *mut osnoise_tool, dump_tasks: c_int, stack_format: c_int) -> c_int;
    fn enable_tracer_by_name(instance: *mut c_void, tracer: *const c_char) -> c_int;
    fn trace_instance_start(trace: *mut trace_instance);
    fn timerlat_bpf_attach() -> c_int;
    fn osn_set_stop(tool: *mut osnoise_tool) -> c_int;

    fn timerlat_auto_analysis(stop_us: c_int, stop_total_us: c_int);
    fn tracefs_instance_file_read(
        instance: *mut c_void,
        file: *const c_char,
        size: *mut c_void,
    ) -> *mut c_char;

    fn timerlat_aa_destroy();
    fn restore_cpu_idle_disable_state(cpu: c_int);
    fn osnoise_destroy_tool(tool: *mut osnoise_tool);
    fn timerlat_bpf_destroy();
    fn free_cpu_idle_disable_states();
    fn run_tool(ops: *const tool_ops, argc: c_int, argv: *mut *mut c_char);
}

/*
 * timerlat_apply_config - apply common configs to the initialized tool
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_apply_config(
    tool: *mut osnoise_tool,
    params: *mut timerlat_params,
) -> c_int {
    let mut retval: c_int;
    let rtla_no_bpf: *const c_char = unsafe { getenv(c"RTLA_NO_BPF".as_ptr()) };

    /*
     * Try to enable BPF, unless disabled explicitly.
     * If BPF enablement fails, fall back to tracefs mode.
     */
    if !rtla_no_bpf.is_null() && unsafe { strncmp_static(rtla_no_bpf, c"1".as_ptr()) } == 0 {
        unsafe { debug_msg(c"RTLA_NO_BPF set, disabling BPF\n".as_ptr()) };
        unsafe { (*params).mode = TRACING_MODE_TRACEFS };
    } else if unsafe {
        tep_find_event_by_name(
            (*tool).trace.tep,
            c"osnoise".as_ptr(),
            c"timerlat_sample".as_ptr(),
        )
        .is_null()
    } {
        unsafe { debug_msg(c"osnoise:timerlat_sample missing, disabling BPF\n".as_ptr()) };
        unsafe { (*params).mode = TRACING_MODE_TRACEFS };
    } else {
        retval = unsafe { timerlat_bpf_init(params) };
        if retval != 0 {
            unsafe { debug_msg(c"Could not enable BPF\n".as_ptr()) };
            unsafe { (*params).mode = TRACING_MODE_TRACEFS };
        }
    }

    /* Check if BPF action program is requested but BPF is not available */
    if unsafe { !(*params).bpf_action_program.is_null() } {
        if unsafe { (*params).mode } == TRACING_MODE_TRACEFS {
            unsafe {
                err_msg(c"BPF actions are not supported in tracefs-only mode\n".as_ptr());
            }
            return -1;
        }

        if unsafe { timerlat_load_bpf_action_program((*params).bpf_action_program) } != 0 {
            return -1;
        }
    }

    retval = unsafe {
        osnoise_set_timerlat_period_us(
            (*tool).context,
            if (*params).timerlat_period_us != 0 {
                (*params).timerlat_period_us
            } else {
                DEFAULT_TIMERLAT_PERIOD
            },
        )
    };
    if retval != 0 {
        unsafe { err_msg(c"Failed to set timerlat period\n".as_ptr()) };
        return -1;
    }

    retval = unsafe { osnoise_set_print_stack((*tool).context, (*params).print_stack) };
    if retval != 0 {
        unsafe { err_msg(c"Failed to set print stack\n".as_ptr()) };
        return -1;
    }

    retval = unsafe { osnoise_set_timerlat_align((*tool).context, (*params).timerlat_align) };
    if retval != 0 && unsafe { (*params).timerlat_align } != 0 {
        /*
         * We might be running on a kernel that does not support timerlat align.
         * Unless user requested it explicitly, ignore the error.
         */
        unsafe { err_msg(c"Failed to enable timerlat align\n".as_ptr()) };
        return -1;
    }

    if unsafe { (*params).timerlat_align } != 0 {
        retval = unsafe { osnoise_set_timerlat_align_us((*tool).context, (*params).timerlat_align_us) };
        if retval != 0 {
            unsafe { err_msg(c"Failed to set timerlat align us\n".as_ptr()) };
            return -1;
        }
    }

    /*
     * If the user did not specify a type of thread, try user-threads first.
     * Fall back to kernel threads otherwise.
     */
    if unsafe { (*params).common.kernel_workload == 0 && (*params).common.user_data == 0 } {
        retval = unsafe {
            tracefs_file_exists(core::ptr::null_mut(), c"osnoise/per_cpu/cpu0/timerlat_fd".as_ptr())
        };
        if retval != 0 {
            unsafe { debug_msg(c"User-space interface detected, setting user-threads\n".as_ptr()) };
            unsafe {
                (*params).common.user_workload = 1;
                (*params).common.user_data = 1;
            }
        } else {
            unsafe { debug_msg(c"User-space interface not detected, setting kernel-threads\n".as_ptr()) };
            unsafe {
                (*params).common.kernel_workload = 1;
            }
        }
    }

    unsafe { common_apply_config(tool, &mut (*params).common) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_enable(tool: *mut osnoise_tool) -> c_int {
    let params: *mut timerlat_params = unsafe { to_timerlat_params((*tool).params) };
    let mut retval: c_int;
    let mut i: c_int = 0;

    if unsafe { (*params).dma_latency } >= 0 {
        unsafe {
            dma_latency_fd = set_cpu_dma_latency((*params).dma_latency);
            if dma_latency_fd < 0 {
                err_msg(c"Could not set /dev/cpu_dma_latency.\n".as_ptr());
                return -1;
            }
        }
    }

    if unsafe { (*params).deepest_idle_state } >= -1 {
        if unsafe { have_libcpupower_support() } == 0 {
            unsafe {
                err_msg(
                    c"rtla built without libcpupower, --deepest-idle-state is not supported\n"
                        .as_ptr(),
                );
            }
            return -1;
        }

        while unsafe { for_each_monitored_cpu_next(&mut i, &mut (*params).common) } != 0 {
            if unsafe { save_cpu_idle_disable_state(i) } < 0 {
                unsafe { err_msg(c"Could not save cpu idle state.\n".as_ptr()) };
                return -1;
            }
            if unsafe { set_deepest_cpu_idle_state(i, (*params).deepest_idle_state) } < 0 {
                unsafe { err_msg(c"Could not set deepest cpu idle state.\n".as_ptr()) };
                return -1;
            }
        }
    }

    if unsafe { (*params).no_aa } == 0 {
        unsafe {
            (*tool).aa = osnoise_init_tool(c"timerlat_aa".as_ptr());
            if (*tool).aa.is_null() {
                return -1;
            }
        }

        retval = unsafe { timerlat_aa_init((*tool).aa, (*params).dump_tasks, (*params).stack_format) };
        if retval != 0 {
            unsafe { err_msg(c"Failed to enable the auto analysis instance\n".as_ptr()) };
            return retval;
        }

        retval = unsafe { enable_tracer_by_name((*(*tool).aa).trace.inst, c"timerlat".as_ptr()) };
        if retval != 0 {
            unsafe { err_msg(c"Failed to enable aa tracer\n".as_ptr()) };
            return retval;
        }
    }

    if unsafe { (*params).common.warmup } > 0 {
        unsafe { debug_msg(c"Warming up for %d seconds\n".as_ptr(), (*params).common.warmup) };
        unsafe { sleep((*params).common.warmup) };
        if unsafe { stop_tracing } != 0 {
            return -1;
        }
    }

    /*
     * Start the tracers here, after having set all instances.
     *
     * Let the trace instance start first for the case of hitting a stop
     * tracing while enabling other instances. The trace instance is the
     * one with most valuable information.
     */
    if unsafe { !(*tool).record.is_null() } {
        unsafe { trace_instance_start(&mut (*(*tool).record).trace) };
    }
    if unsafe { (*params).no_aa } == 0 {
        unsafe { trace_instance_start(&mut (*(*tool).aa).trace) };
    }
    if unsafe { (*params).mode } == TRACING_MODE_TRACEFS {
        unsafe { trace_instance_start(&mut (*tool).trace) };
    } else {
        retval = unsafe { timerlat_bpf_attach() };
        if retval != 0 {
            unsafe { err_msg(c"Error attaching BPF program\n".as_ptr()) };
            return retval;
        }
    }

    /*
     * In tracefs and mixed mode, timerlat tracer handles stopping
     * on threshold
     */
    if unsafe { (*params).mode } != TRACING_MODE_BPF {
        retval = unsafe { osn_set_stop(tool) };
        if retval != 0 {
            return retval;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_analyze(tool: *mut osnoise_tool, stopped: bool) {
    let params: *mut timerlat_params = unsafe { to_timerlat_params((*tool).params) };

    if stopped {
        if unsafe { (*params).no_aa } == 0 {
            unsafe { timerlat_auto_analysis((*params).common.stop_us, (*params).common.stop_total_us) };
        }
    } else if unsafe { (*params).common.aa_only } != 0 {
        let max_lat: *mut c_char;

        /*
         * If the trace did not stop with --aa-only, at least print
         * the max known latency.
         */
        max_lat = unsafe {
            tracefs_instance_file_read(
                (*tool).trace.inst,
                c"tracing_max_latency".as_ptr(),
                core::ptr::null_mut(),
            )
        };
        if !max_lat.is_null() {
            unsafe {
                printf(c"  Max latency was %s\n".as_ptr(), max_lat);
                free(max_lat.cast::<c_void>());
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_free(tool: *mut osnoise_tool) {
    let params: *mut timerlat_params = unsafe { to_timerlat_params((*tool).params) };
    let mut i: c_int = 0;

    unsafe { timerlat_aa_destroy() };
    if unsafe { dma_latency_fd } >= 0 {
        unsafe {
            close(dma_latency_fd);
        }
    }

    if unsafe { (*params).deepest_idle_state } >= -1 {
        while unsafe { for_each_monitored_cpu_next(&mut i, &mut (*params).common) } != 0 {
            unsafe {
                restore_cpu_idle_disable_state(i);
            }
        }
    }

    unsafe { osnoise_destroy_tool((*tool).aa) };

    if unsafe { (*params).mode } != TRACING_MODE_TRACEFS {
        unsafe { timerlat_bpf_destroy() };
    }
    unsafe { free_cpu_idle_disable_states() };
}

unsafe extern "C" fn timerlat_usage(err: c_int) -> ! {
    let mut i: usize;

    static MSG: [*const c_char; 11] = [
        c"".as_ptr(),
        c"timerlat version VERSION".as_ptr(),
        c"".as_ptr(),
        c"  usage: [rtla] timerlat [MODE] ...".as_ptr(),
        c"".as_ptr(),
        c"  modes:".as_ptr(),
        c"     top   - prints the summary from timerlat tracer".as_ptr(),
        c"     hist  - prints a histogram of timer latencies".as_ptr(),
        c"".as_ptr(),
        c"if no MODE is given, the top mode is called, passing the arguments".as_ptr(),
        core::ptr::null(),
    ];

    i = 0;
    while !MSG[i].is_null() {
        unsafe { fprintf(stderr, c"%s\n".as_ptr(), MSG[i]) };
        i += 1;
    }
    unsafe { exit(err) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timerlat_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc == 0 {
        unsafe { timerlat_usage(129) };
    }

    /*
     * if timerlat was called without any argument, run the
     * default cmdline.
     */
    if argc == 1 {
        unsafe {
            run_tool(&timerlat_top_ops, argc, argv);
            exit(0);
        }
    }

    if unsafe { strcmp(*argv.add(1), c"-h".as_ptr()) } == 0
        || unsafe { strcmp(*argv.add(1), c"--help".as_ptr()) } == 0
    {
        unsafe { timerlat_usage(129) };
    } else if unsafe { str_has_prefix(*argv.add(1), c"-".as_ptr()) } != 0 {
        /* the user skipped the tool, call the default one */
        unsafe {
            run_tool(&timerlat_top_ops, argc, argv);
            exit(0);
        }
    } else if unsafe { strcmp(*argv.add(1), c"top".as_ptr()) } == 0 {
        unsafe {
            run_tool(&timerlat_top_ops, argc - 1, argv.add(1));
            exit(0);
        }
    } else if unsafe { strcmp(*argv.add(1), c"hist".as_ptr()) } == 0 {
        unsafe {
            run_tool(&timerlat_hist_ops, argc - 1, argv.add(1));
            exit(0);
        }
    }

    unsafe { timerlat_usage(129) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
