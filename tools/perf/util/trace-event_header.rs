/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type u32 = u32;
pub type u64 = u64;

#[repr(C)]
pub struct list_head {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[repr(C)]
pub union perf_event {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tep_plugin_list {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tep_format_field {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct trace_event {
    pub pevent: *mut tep_handle,
    pub plugin_list: *mut tep_plugin_list,
}

/* Computes a version number comparable with LIBTRACEEVENT_VERSION from Makefile.config. */
#[inline]
pub const fn MAKE_LIBTRACEEVENT_VERSION(a: c_ulong, b: c_ulong, c: c_ulong) -> c_ulong {
    a * 255 * 255 + b * 255 + c
}

pub type tep_func_resolver_t = unsafe extern "C" fn(
    priv_: *mut c_void,
    addrp: *mut u64,
    modp: *mut *mut c_char,
) -> *mut c_char;

unsafe extern "C" {
    pub fn have_tracepoints(evlist: *mut list_head) -> bool;

    pub fn trace_event__init(t: *mut trace_event) -> c_int;
    pub fn trace_event__cleanup(t: *mut trace_event);
    pub fn trace_event__register_resolver(
        machine: *mut machine,
        func: Option<tep_func_resolver_t>,
    ) -> c_int;
    pub fn trace_event__tp_format(sys: *const c_char, name: *const c_char) -> *mut tep_event;

    pub fn trace_event__tp_format_id(id: c_int) -> *mut tep_event;

    pub fn event_format__fprintf(
        event: *const tep_event,
        cpu: c_int,
        data: *mut c_void,
        size: c_int,
        fp: *mut FILE,
    );

    pub fn parse_ftrace_file(pevent: *mut tep_handle, buf: *mut c_char, size: c_ulong) -> c_int;
    pub fn parse_event_file(
        pevent: *mut tep_handle,
        buf: *mut c_char,
        size: c_ulong,
        sys: *mut c_char,
    ) -> c_int;

    pub fn raw_field_value(
        event: *mut tep_event,
        name: *const c_char,
        data: *mut c_void,
    ) -> u64;

    pub fn parse_task_states(state_field: *mut tep_format_field) -> *const c_char;

    pub fn parse_proc_kallsyms(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);
    pub fn parse_ftrace_printk(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);
    pub fn parse_saved_cmdline(pevent: *mut tep_handle, file: *mut c_char, size: c_uint);

    pub fn trace_report(fd: c_int, tevent: *mut trace_event, repipe: bool) -> ssize_t;

    pub fn read_size(event: *mut tep_event, ptr: *mut c_void, size: c_int) -> u64;
    pub fn eval_flag(flag: *const c_char) -> u64;

    pub fn read_tracing_data(fd: c_int, pattrs: *mut list_head) -> c_int;

    /*
     * Return the tracepoint name in the format "subsystem:event_name",
     * callers should free the returned string.
     */
    pub fn tracepoint_id_to_name(config: u64) -> *mut c_char;
}

#[repr(C)]
pub struct tracing_data {
    /* size is only valid if temp is 'true' */
    pub size: ssize_t,
    pub temp: bool,
    pub temp_file: [c_char; 50],
}

unsafe extern "C" {
    pub fn tracing_data_get(pattrs: *mut list_head, fd: c_int, temp: bool) -> *mut tracing_data;
    pub fn tracing_data_put(tdata: *mut tracing_data) -> c_int;
}

#[repr(C)]
pub struct addr_location {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct scripting_ops {
    pub name: *const c_char,
    pub dirname: *const c_char, /* For script path .../scripts/<dirname>/... */
    pub start_script: Option<
        unsafe extern "C" fn(
            script: *const c_char,
            argc: c_int,
            argv: *mut *const c_char,
            session: *mut perf_session,
        ) -> c_int,
    >,
    pub flush_script: Option<unsafe extern "C" fn() -> c_int>,
    pub stop_script: Option<unsafe extern "C" fn() -> c_int>,
    pub process_event: Option<
        unsafe extern "C" fn(
            event: *mut perf_event,
            sample: *mut perf_sample,
            al: *mut addr_location,
            addr_al: *mut addr_location,
        ),
    >,
    pub process_switch: Option<
        unsafe extern "C" fn(
            event: *mut perf_event,
            sample: *mut perf_sample,
            machine: *mut machine,
        ),
    >,
    pub process_auxtrace_error:
        Option<unsafe extern "C" fn(session: *mut perf_session, event: *mut perf_event)>,
    pub process_stat: Option<
        unsafe extern "C" fn(config: *mut perf_stat_config, evsel: *mut evsel, tstamp: u64),
    >,
    pub process_stat_interval: Option<unsafe extern "C" fn(tstamp: u64)>,
    pub process_throttle: Option<
        unsafe extern "C" fn(
            event: *mut perf_event,
            sample: *mut perf_sample,
            machine: *mut machine,
        ),
    >,
    pub generate_script:
        Option<unsafe extern "C" fn(pevent: *mut tep_handle, outfile: *const c_char) -> c_int>,
}

unsafe extern "C" {
    pub static mut scripting_max_stack: c_uint;

    pub fn script_spec__lookup(spec: *const c_char) -> *mut scripting_ops;
    pub fn script_spec__for_each(
        cb: Option<unsafe extern "C" fn(ops: *mut scripting_ops, spec: *const c_char) -> c_int>,
    ) -> c_int;

    pub fn setup_perl_scripting();
    pub fn setup_python_scripting();
}

#[repr(C)]
pub struct scripting_context {
    pub pevent: *mut tep_handle,
    pub event_data: *mut c_void,
    pub event: *mut perf_event,
    pub sample: *mut perf_sample,
    pub al: *mut addr_location,
    pub addr_al: *mut addr_location,
    pub session: *mut perf_session,
}

unsafe extern "C" {
    pub fn scripting_context__update(
        scripting_context: *mut scripting_context,
        event: *mut perf_event,
        sample: *mut perf_sample,
        al: *mut addr_location,
        addr_al: *mut addr_location,
    );

    pub fn common_pc(context: *mut scripting_context) -> c_int;
    pub fn common_flags(context: *mut scripting_context) -> c_int;
    pub fn common_lock_depth(context: *mut scripting_context) -> c_int;
}

pub const SAMPLE_FLAGS_BUF_SIZE: usize = 64;
pub const SAMPLE_FLAGS_STR_ALIGNED_SIZE: usize = 21;

unsafe extern "C" {
    pub fn perf_sample__sprintf_flags(flags: u32, str_: *mut c_char, sz: size_t) -> c_int;
}

/*
 * C conditional:
 * #if defined(LIBTRACEEVENT_VERSION) &&
 *     LIBTRACEEVENT_VERSION >= MAKE_LIBTRACEEVENT_VERSION(1, 5, 0)
 *   include <event-parse.h> and test TEP_FIELD_IS_RELATIVE
 * #else
 *   include <linux/compiler.h> and ignore flags with __maybe_unused, returning false
 * #endif
 *
 * TEP_FIELD_IS_RELATIVE is supplied by the libtraceevent dependency when that
 * version condition is true.
 */
#[inline]
pub unsafe fn tep_field_is_relative(flags: c_ulong) -> bool {
    (flags & TEP_FIELD_IS_RELATIVE) != 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
