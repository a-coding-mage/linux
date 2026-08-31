// SPDX-License-Identifier: GPL-2.0-only
/*
 * CTF writing support via babeltrace.
 *
 * Copyright (C) 2014, Jiri Olsa <jolsa@redhat.com>
 * Copyright (C) 2014, Sebastian Andrzej Siewior <bigeasy@linutronix.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_longlong, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type u32 = ::core::primitive::u32;
type u64 = ::core::primitive::u64;
type size_t = usize;
type bool_t = bool;

const MAX_CPUS: c_int = 4096;
const STREAM_FLUSH_COUNT: u32 = 100000;

const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_READ: u64 = 1 << 4;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_ID: u64 = 1 << 6;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
const PERF_SAMPLE_RAW: u64 = 1 << 10;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_WEIGHT: u64 = 1 << 14;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;

const PERF_TYPE_TRACEPOINT: u32 = 2;
const PERF_DATA_MODE_READ: c_int = 0;
const BT_CTF_INTEGER_BASE_HEXADECIMAL: c_int = 16;
const BT_CTF_BYTE_ORDER_BIG_ENDIAN: c_int = 4321;
const BT_CTF_BYTE_ORDER_LITTLE_ENDIAN: c_int = 1234;

const TEP_FIELD_IS_ARRAY: c_ulong = 1 << 0;
const TEP_FIELD_IS_POINTER: c_ulong = 1 << 1;
const TEP_FIELD_IS_SIGNED: c_ulong = 1 << 2;
const TEP_FIELD_IS_STRING: c_ulong = 1 << 3;
const TEP_FIELD_IS_DYNAMIC: c_ulong = 1 << 4;
const TEP_FIELD_IS_LONG: c_ulong = 1 << 5;

const HEADER_EVENT_DESC: c_int = 12;
const HEADER_HOSTNAME: c_int = 1;
const HEADER_OSRELEASE: c_int = 2;
const HEADER_VERSION: c_int = 3;
const HEADER_ARCH: c_int = 4;

#[repr(C)]
pub struct bt_ctf_event_class {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_event {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_field_type {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_field {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_writer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_stream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_stream_class {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bt_ctf_clock {
    _private: [u8; 0],
}
#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_priv {
    event_class: *mut bt_ctf_event_class,
}

#[repr(C)]
pub struct ctf_stream {
    stream: *mut bt_ctf_stream,
    cpu: c_int,
    count: u32,
}

#[repr(C)]
pub struct ctf_writer_data_named {
    s64: *mut bt_ctf_field_type,
    u64: *mut bt_ctf_field_type,
    s32: *mut bt_ctf_field_type,
    u32: *mut bt_ctf_field_type,
    string: *mut bt_ctf_field_type,
    u32_hex: *mut bt_ctf_field_type,
    u64_hex: *mut bt_ctf_field_type,
}

#[repr(C)]
pub union ctf_writer_data {
    named: core::mem::ManuallyDrop<ctf_writer_data_named>,
    array: [*mut bt_ctf_field_type; 6],
}

#[repr(C)]
pub struct ctf_writer {
    writer: *mut bt_ctf_writer,
    stream: *mut *mut ctf_stream,
    stream_cnt: c_int,
    stream_class: *mut bt_ctf_stream_class,
    clock: *mut bt_ctf_clock,
    data: ctf_writer_data,
    comm_class: *mut bt_ctf_event_class,
    exit_class: *mut bt_ctf_event_class,
    fork_class: *mut bt_ctf_event_class,
    mmap_class: *mut bt_ctf_event_class,
    mmap2_class: *mut bt_ctf_event_class,
}

#[repr(C)]
pub struct convert {
    tool: perf_tool,
    writer: ctf_writer,
    ptime_range: *mut perf_time_interval,
    range_size: c_int,
    range_num: c_int,
    events_size: u64,
    events_count: u64,
    non_sample_count: u64,
    skipped: u64,
    /* Ordered events configured queue size. */
    queue_size: u64,
}

#[repr(C)]
pub struct perf_tool {
    sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    mmap: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    mmap2: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    comm: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    exit: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    fork: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    lost: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    tracing_data: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    build_id: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    namespaces: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    finished_round: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    attr: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>,
    feature: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>,
    ordering_requires_timestamps: bool_t,
}

#[repr(C)]
pub struct perf_event_header {
    size: u16,
}
type u16 = ::core::primitive::u16;

#[repr(C)]
pub struct perf_record_header_feature {
    feat_id: c_int,
}

#[repr(C)]
pub union perf_event {
    header: core::mem::ManuallyDrop<perf_event_header>,
    feat: core::mem::ManuallyDrop<perf_record_header_feature>,
    comm: core::mem::ManuallyDrop<perf_record_comm>,
    fork: core::mem::ManuallyDrop<perf_record_fork>,
    mmap: core::mem::ManuallyDrop<perf_record_mmap>,
    mmap2: core::mem::ManuallyDrop<perf_record_mmap>,
}

#[repr(C)]
pub struct perf_record_comm {
    pid: u32,
    tid: u32,
    comm: *const c_char,
}
#[repr(C)]
pub struct perf_record_fork {
    pid: u32,
    ppid: u32,
    tid: u32,
    ptid: u32,
    time: u64,
}
#[repr(C)]
pub struct perf_record_mmap {
    pid: u32,
    tid: u32,
    start: u64,
    filename: *const c_char,
}

#[repr(C)]
pub struct perf_sample {
    evsel: *mut evsel,
    raw_data: *mut c_void,
    raw_size: u32,
    time: u64,
    ip: u64,
    tid: i32,
    pid: i32,
    id: u64,
    stream_id: u64,
    period: u64,
    weight: u64,
    data_src: u64,
    transaction: u64,
    cpu: c_int,
    callchain: *mut ip_callchain,
}

#[repr(C)]
pub struct ip_callchain {
    nr: u64,
    ips: [u64; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    type_: u32,
    sample_type: u64,
}
#[repr(C)]
pub struct evsel_core {
    attr: perf_event_attr,
}
#[repr(C)]
pub struct evsel {
    core: evsel_core,
    priv_: *mut c_void,
    next: *mut evsel,
}

#[repr(C)]
pub struct evlist {
    entries: *mut evsel,
}

#[repr(C)]
pub struct perf_clock {
    enabled: bool_t,
    clockid: c_int,
    tod_ns: i64,
    clockid_ns: i64,
}
#[repr(C)]
pub struct perf_env {
    hostname: *const c_char,
    version: *const c_char,
    arch: *const c_char,
    os_release: *const c_char,
    nr_cpus_avail: c_int,
    clock: perf_clock,
}
#[repr(C)]
pub struct perf_header {
    env: perf_env,
}
#[repr(C)]
pub struct ordered_events {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_session {
    evlist: *mut evlist,
    header: perf_header,
    ordered_events: ordered_events,
}
#[repr(C)]
pub struct perf_data {
    path: *const c_char,
    mode: c_int,
    force: bool_t,
}
#[repr(C)]
pub struct perf_data_convert_opts {
    force: bool_t,
    all: bool_t,
    tod: bool_t,
    time_str: *const c_char,
}
#[repr(C)]
pub struct perf_time_interval {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event_format {
    common_fields: *mut tep_format_field,
    fields: *mut tep_format_field,
}
#[repr(C)]
pub struct tep_event {
    tep: *mut tep_handle,
    format: tep_event_format,
}
#[repr(C)]
pub struct tep_format_field {
    next: *mut tep_format_field,
    name: *mut c_char,
    alias: *mut c_char,
    flags: c_ulong,
    size: c_int,
    offset: c_int,
    arraylen: c_int,
    event: *mut tep_event,
}

unsafe extern "C" {
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut c_void);
    fn isprint(c: c_int) -> c_int;

    fn bt_ctf_field_type_integer_get_signed(type_: *mut bt_ctf_field_type) -> bool_t;
    fn bt_ctf_field_create(type_: *mut bt_ctf_field_type) -> *mut bt_ctf_field;
    fn bt_ctf_field_integer_signed_set_value(field: *mut bt_ctf_field, val: c_ulonglong) -> c_int;
    fn bt_ctf_field_integer_unsigned_set_value(field: *mut bt_ctf_field, val: c_ulonglong) -> c_int;
    fn bt_ctf_event_set_payload(event: *mut bt_ctf_event, name: *const c_char, field: *mut bt_ctf_field) -> c_int;
    fn bt_ctf_field_put(field: *mut bt_ctf_field);
    fn bt_ctf_field_string_set_value(field: *mut bt_ctf_field, string: *const c_char) -> c_int;
    fn bt_ctf_event_class_get_field_by_name(class: *mut bt_ctf_event_class, name: *const c_char) -> *mut bt_ctf_field_type;
    fn bt_ctf_field_type_put(type_: *mut bt_ctf_field_type);
    fn bt_ctf_field_array_get_field(array: *mut bt_ctf_field, i: u32) -> *mut bt_ctf_field;
    fn bt_ctf_field_sequence_set_length(seq: *mut bt_ctf_field, len: *mut bt_ctf_field) -> c_int;
    fn bt_ctf_field_sequence_get_field(seq: *mut bt_ctf_field, i: u32) -> *mut bt_ctf_field;
    fn bt_ctf_stream_flush(stream: *mut bt_ctf_stream) -> c_int;
    fn bt_ctf_writer_create_stream(writer: *mut bt_ctf_writer, class: *mut bt_ctf_stream_class) -> *mut bt_ctf_stream;
    fn bt_ctf_stream_get_packet_context(stream: *mut bt_ctf_stream) -> *mut bt_ctf_field;
    fn bt_ctf_field_structure_get_field(field: *mut bt_ctf_field, name: *const c_char) -> *mut bt_ctf_field;
    fn bt_ctf_stream_put(stream: *mut bt_ctf_stream);
    fn bt_ctf_event_create(class: *mut bt_ctf_event_class) -> *mut bt_ctf_event;
    fn bt_ctf_clock_set_time(clock: *mut bt_ctf_clock, time: u64) -> c_int;
    fn bt_ctf_stream_append_event(stream: *mut bt_ctf_stream, event: *mut bt_ctf_event) -> c_int;
    fn bt_ctf_event_put(event: *mut bt_ctf_event);
    fn bt_ctf_validate_identifier(name: *const c_char) -> c_int;
    fn bt_ctf_event_class_add_field(class: *mut bt_ctf_event_class, type_: *mut bt_ctf_field_type, name: *const c_char) -> c_int;
    fn bt_ctf_field_type_array_create(type_: *mut bt_ctf_field_type, len: c_int) -> *mut bt_ctf_field_type;
    fn bt_ctf_field_type_sequence_create(type_: *mut bt_ctf_field_type, len_name: *const c_char) -> *mut bt_ctf_field_type;
    fn bt_ctf_event_class_create(name: *const c_char) -> *mut bt_ctf_event_class;
    fn bt_ctf_stream_class_add_event_class(stream_class: *mut bt_ctf_stream_class, event_class: *mut bt_ctf_event_class) -> c_int;
    fn bt_ctf_event_class_put(class: *mut bt_ctf_event_class);
    fn bt_ctf_writer_add_environment_field(writer: *mut bt_ctf_writer, name: *const c_char, val: *const c_char) -> c_int;
    fn bt_ctf_clock_set_frequency(clock: *mut bt_ctf_clock, v: u64) -> c_int;
    fn bt_ctf_clock_set_offset(clock: *mut bt_ctf_clock, v: i64) -> c_int;
    fn bt_ctf_clock_set_description(clock: *mut bt_ctf_clock, v: *const c_char) -> c_int;
    fn bt_ctf_clock_set_precision(clock: *mut bt_ctf_clock, v: u64) -> c_int;
    fn bt_ctf_clock_set_is_absolute(clock: *mut bt_ctf_clock, v: c_int) -> c_int;
    fn bt_ctf_field_type_integer_create(size: c_int) -> *mut bt_ctf_field_type;
    fn bt_ctf_field_type_integer_set_signed(type_: *mut bt_ctf_field_type, v: c_int) -> c_int;
    fn bt_ctf_field_type_integer_set_base(type_: *mut bt_ctf_field_type, base: c_int) -> c_int;
    fn bt_ctf_field_type_set_byte_order(type_: *mut bt_ctf_field_type, order: c_int) -> c_int;
    fn bt_ctf_field_type_string_create() -> *mut bt_ctf_field_type;
    fn bt_ctf_clock_put(clock: *mut bt_ctf_clock);
    fn bt_ctf_stream_class_put(stream_class: *mut bt_ctf_stream_class);
    fn bt_ctf_writer_put(writer: *mut bt_ctf_writer);
    fn bt_ctf_writer_create(path: *const c_char) -> *mut bt_ctf_writer;
    fn bt_ctf_clock_create(name: *const c_char) -> *mut bt_ctf_clock;
    fn bt_ctf_stream_class_create(name: *const c_char) -> *mut bt_ctf_stream_class;
    fn bt_ctf_stream_class_set_clock(stream_class: *mut bt_ctf_stream_class, clock: *mut bt_ctf_clock) -> c_int;
    fn bt_ctf_stream_class_get_packet_context_type(stream_class: *mut bt_ctf_stream_class) -> *mut bt_ctf_field_type;
    fn bt_ctf_field_type_structure_add_field(type_: *mut bt_ctf_field_type, field: *mut bt_ctf_field_type, name: *const c_char) -> c_int;
    fn bt_ctf_writer_add_clock(writer: *mut bt_ctf_writer, clock: *mut bt_ctf_clock) -> c_int;

    fn tep_read_number(tep: *mut tep_handle, data: *mut c_void, size: c_int) -> c_ulonglong;
    fn tep_field_is_relative(flags: c_ulong) -> c_int;
    fn evsel__tp_format(evsel: *mut evsel) -> *const tep_event;
    fn evsel__is_bpf_output(evsel: *mut evsel) -> bool_t;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn perf_time__ranges_skip_sample(range: *mut perf_time_interval, nr: c_int, time: u64) -> bool_t;
    fn perf_event__process_comm(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_mmap2(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_exit(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_fork(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_lost(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_build_id(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_namespaces(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_finished_round(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_attr(tool: *const perf_tool, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn perf_event__process_feature(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_event__process_tracing_data(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool_t);
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_config_u64(var: *mut u64, name: *const c_char, value: *const c_char) -> c_int;
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_env__os_release(env: *mut perf_env) -> *const c_char;
    fn perf_time__parse_for_ranges(time: *const c_char, session: *mut perf_session, range: *mut *mut perf_time_interval, size: *mut c_int, num: *mut c_int) -> c_int;
    fn ordered_events__set_alloc_size(events: *mut ordered_events, size: u64);
    fn evlist__put(evlist: *mut evlist);
    fn clockid_name(clockid: c_int) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool_t;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn pr_N(_n: c_int, fmt: *const c_char) {
    pr_err(fmt);
}

unsafe fn pr(fmt: *const c_char) {
    pr_N(1, fmt);
}

unsafe fn pr2(fmt: *const c_char) {
    pr_N(2, fmt);
}

unsafe fn value_set(type_: *mut bt_ctf_field_type, event: *mut bt_ctf_event, name: *const c_char, val: u64) -> c_int {
    let field: *mut bt_ctf_field;
    let sign = bt_ctf_field_type_integer_get_signed(type_);
    let mut ret: c_int;

    field = bt_ctf_field_create(type_);
    if field.is_null() {
        pr_err(cstr!("failed to create a field %s\n"), name);
        return -1;
    }

    if sign {
        ret = bt_ctf_field_integer_signed_set_value(field, val as c_ulonglong);
        if ret != 0 {
            pr_err(cstr!("failed to set field value %s\n"), name);
            bt_ctf_field_put(field);
            return ret;
        }
    } else {
        ret = bt_ctf_field_integer_unsigned_set_value(field, val as c_ulonglong);
        if ret != 0 {
            pr_err(cstr!("failed to set field value %s\n"), name);
            bt_ctf_field_put(field);
            return ret;
        }
    }

    ret = bt_ctf_event_set_payload(event, name, field);
    if ret != 0 {
        pr_err(cstr!("failed to set payload %s\n"), name);
    }

    bt_ctf_field_put(field);
    ret
}

unsafe fn value_set_s32(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, val: i32) -> c_int {
    value_set((*cw).data.named.s32, event, name, val as u64)
}
unsafe fn value_set_u32(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, val: u32) -> c_int {
    value_set((*cw).data.named.u32, event, name, val as u64)
}
unsafe fn value_set_s64(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, val: i64) -> c_int {
    value_set((*cw).data.named.s64, event, name, val as u64)
}
unsafe fn value_set_u64(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, val: u64) -> c_int {
    value_set((*cw).data.named.u64, event, name, val)
}
unsafe fn value_set_u64_hex(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, val: u64) -> c_int {
    value_set((*cw).data.named.u64_hex, event, name, val)
}

unsafe fn value_set_string(cw: *mut ctf_writer, event: *mut bt_ctf_event, name: *const c_char, string: *const c_char) -> c_int {
    let type_ = (*cw).data.named.string;
    let field = bt_ctf_field_create(type_);
    let mut ret = 0;

    if field.is_null() {
        pr_err(cstr!("failed to create a field %s\n"), name);
        return -1;
    }
    ret = string_set_value(field, string);
    if ret != 0 {
        pr_err(cstr!("failed to set value %s\n"), name);
        bt_ctf_field_put(field);
        return ret;
    }
    ret = bt_ctf_event_set_payload(event, name, field);
    if ret != 0 {
        pr_err(cstr!("failed to set payload %s\n"), name);
    }
    bt_ctf_field_put(field);
    ret
}

unsafe fn get_tracepoint_field_type(cw: *mut ctf_writer, field: *mut tep_format_field) -> *mut bt_ctf_field_type {
    let flags = (*field).flags;
    if flags & TEP_FIELD_IS_STRING != 0 {
        return (*cw).data.named.string;
    }
    if flags & TEP_FIELD_IS_SIGNED == 0 {
        /* unsigned long are mostly pointers */
        if flags & TEP_FIELD_IS_LONG != 0 || flags & TEP_FIELD_IS_POINTER != 0 {
            return (*cw).data.named.u64_hex;
        }
    }
    if flags & TEP_FIELD_IS_SIGNED != 0 {
        if (*field).size == 8 {
            return (*cw).data.named.s64;
        } else {
            return (*cw).data.named.s32;
        }
    }
    if (*field).size == 8 {
        (*cw).data.named.u64
    } else {
        (*cw).data.named.u32
    }
}

unsafe fn adjust_signedness(value_int: c_ulonglong, size: c_int) -> c_ulonglong {
    let value_mask: c_ulonglong;
    /*
     * value_mask = (1 << (size * 8 - 1)) - 1.
     * Directly set value_mask for code readers.
     */
    match size {
        1 => value_mask = 0x7f,
        2 => value_mask = 0x7fff,
        4 => value_mask = 0x7fffffff,
        8 => {
            /*
             * For 64 bit value, return it self. There is no need
             * to fill high bit.
             */
            return value_int;
        }
        _ => {
            /* BUG! */
            return value_int;
        }
    }
    /* If it is a positive value, don't adjust. */
    if value_int & (!0u64 - value_mask) == 0 {
        return value_int;
    }
    /* Fill upper part of value_int with 1 to make it a negative long long. */
    (value_int & value_mask) | !value_mask
}

unsafe fn string_set_value(field: *mut bt_ctf_field, string: *const c_char) -> c_int {
    let mut buffer: *mut c_char = ptr::null_mut();
    let len = strlen(string);
    let mut i: size_t = 0;
    let mut p: size_t = 0;
    let err: c_int;

    while i < len {
        if isprint(*string.add(i) as c_int) != 0 {
            if !buffer.is_null() {
                *buffer.add(p) = *string.add(i);
            }
        } else {
            let mut numstr = [0 as c_char; 5];
            snprintf(numstr.as_mut_ptr(), numstr.len(), cstr!("\\x%02x"), (*string.add(i) as u32) & 0xff);
            if buffer.is_null() {
                buffer = zalloc(i + (len - i) * 4 + 2) as *mut c_char;
                if buffer.is_null() {
                    pr_err(cstr!("failed to set unprintable string '%s'\n"), string);
                    return bt_ctf_field_string_set_value(field, cstr!("UNPRINTABLE-STRING"));
                }
                if i > 0 {
                    strncpy(buffer, string, i);
                }
            }
            memcpy(buffer.add(p) as *mut c_void, numstr.as_ptr() as *const c_void, 4);
            p += 3;
        }
        i += 1;
        p += 1;
    }

    if buffer.is_null() {
        return bt_ctf_field_string_set_value(field, string);
    }
    err = bt_ctf_field_string_set_value(field, buffer);
    free(buffer as *mut c_void);
    err
}

unsafe fn add_tracepoint_field_value(cw: *mut ctf_writer, event_class: *mut bt_ctf_event_class, event: *mut bt_ctf_event, sample: *mut perf_sample, fmtf: *mut tep_format_field) -> c_int {
    let mut type_: *mut bt_ctf_field_type;
    let mut array_field: *mut bt_ctf_field;
    let mut field: *mut bt_ctf_field = ptr::null_mut();
    let name = (*fmtf).alias;
    let data = (*sample).raw_data;
    let mut flags = (*fmtf).flags;
    let mut offset = (*fmtf).offset as u32;
    let mut len = (*fmtf).size as u32;
    let mut ret: c_int = 0;

    if flags & TEP_FIELD_IS_STRING != 0 {
        flags &= !TEP_FIELD_IS_ARRAY;
    }
    if flags & TEP_FIELD_IS_DYNAMIC != 0 {
        let tmp_val = tep_read_number((*(*fmtf).event).tep, (data as *mut u8).add(offset as usize) as *mut c_void, len as c_int);
        offset = tmp_val as u32;
        len = offset >> 16;
        offset &= 0xffff;
        if tep_field_is_relative(flags) != 0 {
            offset += ((*fmtf).offset + (*fmtf).size) as u32;
        }
    }

    let n_items: u32;
    if flags & TEP_FIELD_IS_ARRAY != 0 {
        type_ = bt_ctf_event_class_get_field_by_name(event_class, name);
        array_field = bt_ctf_field_create(type_);
        bt_ctf_field_type_put(type_);
        if array_field.is_null() {
            pr_err(cstr!("Failed to create array type %s\n"), name);
            return -1;
        }
        len = ((*fmtf).size / (*fmtf).arraylen) as u32;
        n_items = (*fmtf).arraylen as u32;
    } else {
        n_items = 1;
        array_field = ptr::null_mut();
    }

    type_ = get_tracepoint_field_type(cw, fmtf);
    let mut i = 0;
    while i < n_items {
        if flags & TEP_FIELD_IS_ARRAY != 0 {
            field = bt_ctf_field_array_get_field(array_field, i);
        } else {
            field = bt_ctf_field_create(type_);
        }
        if field.is_null() {
            pr_err(cstr!("failed to create a field %s\n"), name);
            return -1;
        }

        if flags & TEP_FIELD_IS_STRING != 0 {
            ret = string_set_value(field, (data as *mut u8).add((offset + i * len) as usize) as *const c_char);
        } else {
            let value_int = tep_read_number((*(*fmtf).event).tep, (data as *mut u8).add((offset + i * len) as usize) as *mut c_void, len as c_int);
            if flags & TEP_FIELD_IS_SIGNED == 0 {
                ret = bt_ctf_field_integer_unsigned_set_value(field, value_int);
            } else {
                ret = bt_ctf_field_integer_signed_set_value(field, adjust_signedness(value_int, len as c_int));
            }
        }
        if ret != 0 {
            pr_err(cstr!("failed to set file value %s\n"), name);
            bt_ctf_field_put(field);
            return -1;
        }
        if flags & TEP_FIELD_IS_ARRAY == 0 {
            ret = bt_ctf_event_set_payload(event, name, field);
            if ret != 0 {
                pr_err(cstr!("failed to set payload %s\n"), name);
                bt_ctf_field_put(field);
                return -1;
            }
        }
        bt_ctf_field_put(field);
        i += 1;
    }
    if flags & TEP_FIELD_IS_ARRAY != 0 {
        ret = bt_ctf_event_set_payload(event, name, array_field);
        if ret != 0 {
            pr_err(cstr!("Failed add payload array %s\n"), name);
            return -1;
        }
        bt_ctf_field_put(array_field);
    }
    0
}

unsafe fn add_tracepoint_fields_values(cw: *mut ctf_writer, event_class: *mut bt_ctf_event_class, event: *mut bt_ctf_event, fields: *mut tep_format_field, sample: *mut perf_sample) -> c_int {
    let mut field = fields;
    while !field.is_null() {
        if add_tracepoint_field_value(cw, event_class, event, sample, field) != 0 {
            return -1;
        }
        field = (*field).next;
    }
    0
}

unsafe fn add_tracepoint_values(cw: *mut ctf_writer, event_class: *mut bt_ctf_event_class, event: *mut bt_ctf_event, evsel: *mut evsel, sample: *mut perf_sample) -> c_int {
    let tp_format = evsel__tp_format(evsel);
    let common_fields = (*tp_format).format.common_fields;
    let fields = (*tp_format).format.fields;
    let mut ret = add_tracepoint_fields_values(cw, event_class, event, common_fields, sample);
    if ret == 0 {
        ret = add_tracepoint_fields_values(cw, event_class, event, fields, sample);
    }
    ret
}

unsafe fn add_bpf_output_values(event_class: *mut bt_ctf_event_class, event: *mut bt_ctf_event, sample: *mut perf_sample) -> c_int {
    let raw_size = (*sample).raw_size;
    let nr_elements = raw_size / size_of::<u32>() as u32;
    let mut ret: c_int;

    if nr_elements * size_of::<u32>() as u32 != raw_size {
        pr_warning(cstr!("Incorrect raw_size (%u) in bpf output event, skip %zu bytes\n"), raw_size, nr_elements * size_of::<u32>() as u32 - raw_size);
    }
    let len_type = bt_ctf_event_class_get_field_by_name(event_class, cstr!("raw_len"));
    let len_field = bt_ctf_field_create(len_type);
    if len_field.is_null() {
        pr_err(cstr!("failed to create 'raw_len' for bpf output event\n"));
        bt_ctf_field_type_put(len_type);
        return -1;
    }
    ret = bt_ctf_field_integer_unsigned_set_value(len_field, nr_elements as c_ulonglong);
    if ret == 0 {
        ret = bt_ctf_event_set_payload(event, cstr!("raw_len"), len_field);
    }
    if ret != 0 {
        bt_ctf_field_put(len_field);
        bt_ctf_field_type_put(len_type);
        return ret;
    }
    let seq_type = bt_ctf_event_class_get_field_by_name(event_class, cstr!("raw_data"));
    let seq_field = bt_ctf_field_create(seq_type);
    if seq_field.is_null() {
        pr_err(cstr!("failed to create 'raw_data' for bpf output event\n"));
        bt_ctf_field_type_put(seq_type);
        bt_ctf_field_put(len_field);
        bt_ctf_field_type_put(len_type);
        return -1;
    }
    ret = bt_ctf_field_sequence_set_length(seq_field, len_field);
    if ret == 0 {
        let mut i = 0;
        while i < nr_elements {
            let elem_field = bt_ctf_field_sequence_get_field(seq_field, i);
            ret = bt_ctf_field_integer_unsigned_set_value(elem_field, *((*sample).raw_data as *mut u32).add(i as usize) as c_ulonglong);
            bt_ctf_field_put(elem_field);
            if ret != 0 {
                pr_err(cstr!("failed to set raw_data[%d]\n"), i);
                break;
            }
            i += 1;
        }
    }
    if ret == 0 {
        ret = bt_ctf_event_set_payload(event, cstr!("raw_data"), seq_field);
        if ret != 0 {
            pr_err(cstr!("failed to set payload for raw_data\n"));
        }
    }
    bt_ctf_field_put(seq_field);
    bt_ctf_field_type_put(seq_type);
    bt_ctf_field_put(len_field);
    bt_ctf_field_type_put(len_type);
    ret
}

unsafe fn add_callchain_output_values(event_class: *mut bt_ctf_event_class, event: *mut bt_ctf_event, callchain: *mut ip_callchain) -> c_int {
    let nr_elements = (*callchain).nr as u32;
    let len_type = bt_ctf_event_class_get_field_by_name(event_class, cstr!("perf_callchain_size"));
    let len_field = bt_ctf_field_create(len_type);
    let mut ret: c_int;
    if len_field.is_null() {
        pr_err(cstr!("failed to create 'perf_callchain_size' for callchain output event\n"));
        bt_ctf_field_type_put(len_type);
        return -1;
    }
    ret = bt_ctf_field_integer_unsigned_set_value(len_field, nr_elements as c_ulonglong);
    if ret == 0 {
        ret = bt_ctf_event_set_payload(event, cstr!("perf_callchain_size"), len_field);
    }
    if ret != 0 {
        bt_ctf_field_put(len_field);
        bt_ctf_field_type_put(len_type);
        return ret;
    }
    let seq_type = bt_ctf_event_class_get_field_by_name(event_class, cstr!("perf_callchain"));
    let seq_field = bt_ctf_field_create(seq_type);
    if seq_field.is_null() {
        pr_err(cstr!("failed to create 'perf_callchain' for callchain output event\n"));
        bt_ctf_field_type_put(seq_type);
        bt_ctf_field_put(len_field);
        bt_ctf_field_type_put(len_type);
        return -1;
    }
    ret = bt_ctf_field_sequence_set_length(seq_field, len_field);
    let mut i = 0;
    while ret == 0 && i < nr_elements {
        let elem_field = bt_ctf_field_sequence_get_field(seq_field, i);
        ret = bt_ctf_field_integer_unsigned_set_value(elem_field, *(*callchain).ips.as_ptr().add(i as usize) as c_ulonglong);
        bt_ctf_field_put(elem_field);
        if ret != 0 {
            pr_err(cstr!("failed to set callchain[%d]\n"), i);
        }
        i += 1;
    }
    if ret == 0 {
        ret = bt_ctf_event_set_payload(event, cstr!("perf_callchain"), seq_field);
        if ret != 0 {
            pr_err(cstr!("failed to set payload for raw_data\n"));
        }
    }
    bt_ctf_field_put(seq_field);
    bt_ctf_field_type_put(seq_type);
    bt_ctf_field_put(len_field);
    bt_ctf_field_type_put(len_type);
    ret
}

unsafe fn add_generic_values(cw: *mut ctf_writer, event: *mut bt_ctf_event, evsel: *mut evsel, sample: *mut perf_sample) -> c_int {
    let type_ = (*evsel).core.attr.sample_type;
    if type_ & PERF_SAMPLE_IP != 0 && value_set_u64_hex(cw, event, cstr!("perf_ip"), (*sample).ip) != 0 { return -1; }
    if type_ & PERF_SAMPLE_TID != 0 {
        if value_set_s32(cw, event, cstr!("perf_tid"), (*sample).tid) != 0 { return -1; }
        if value_set_s32(cw, event, cstr!("perf_pid"), (*sample).pid) != 0 { return -1; }
    }
    if (type_ & PERF_SAMPLE_ID != 0 || type_ & PERF_SAMPLE_IDENTIFIER != 0) && value_set_u64(cw, event, cstr!("perf_id"), (*sample).id) != 0 { return -1; }
    if type_ & PERF_SAMPLE_STREAM_ID != 0 && value_set_u64(cw, event, cstr!("perf_stream_id"), (*sample).stream_id) != 0 { return -1; }
    if type_ & PERF_SAMPLE_PERIOD != 0 && value_set_u64(cw, event, cstr!("perf_period"), (*sample).period) != 0 { return -1; }
    if type_ & PERF_SAMPLE_WEIGHT != 0 && value_set_u64(cw, event, cstr!("perf_weight"), (*sample).weight) != 0 { return -1; }
    if type_ & PERF_SAMPLE_DATA_SRC != 0 && value_set_u64(cw, event, cstr!("perf_data_src"), (*sample).data_src) != 0 { return -1; }
    if type_ & PERF_SAMPLE_TRANSACTION != 0 && value_set_u64(cw, event, cstr!("perf_transaction"), (*sample).transaction) != 0 { return -1; }
    0
}

unsafe fn ctf_stream__flush(cs: *mut ctf_stream) -> c_int {
    let mut err = 0;
    if !cs.is_null() {
        err = bt_ctf_stream_flush((*cs).stream);
        if err != 0 {
            pr_err(cstr!("CTF stream %d flush failed\n"), (*cs).cpu);
        }
        (*cs).count = 0;
    }
    err
}

unsafe fn ctf_stream__create(cw: *mut ctf_writer, cpu: c_int) -> *mut ctf_stream {
    let cs = zalloc(size_of::<ctf_stream>()) as *mut ctf_stream;
    if cs.is_null() {
        pr_err(cstr!("Failed to allocate ctf stream\n"));
        return ptr::null_mut();
    }
    let stream = bt_ctf_writer_create_stream((*cw).writer, (*cw).stream_class);
    if stream.is_null() {
        free(cs as *mut c_void);
        return ptr::null_mut();
    }
    let pkt_ctx = bt_ctf_stream_get_packet_context(stream);
    if pkt_ctx.is_null() {
        bt_ctf_stream_put(stream);
        free(cs as *mut c_void);
        return ptr::null_mut();
    }
    let cpu_field = bt_ctf_field_structure_get_field(pkt_ctx, cstr!("cpu_id"));
    bt_ctf_field_put(pkt_ctx);
    if cpu_field.is_null() {
        bt_ctf_stream_put(stream);
        free(cs as *mut c_void);
        return ptr::null_mut();
    }
    if bt_ctf_field_integer_unsigned_set_value(cpu_field, cpu as u32 as c_ulonglong) != 0 {
        bt_ctf_field_put(cpu_field);
        bt_ctf_stream_put(stream);
        free(cs as *mut c_void);
        return ptr::null_mut();
    }
    bt_ctf_field_put(cpu_field);
    (*cs).cpu = cpu;
    (*cs).stream = stream;
    cs
}

unsafe fn ctf_stream__delete(cs: *mut ctf_stream) {
    if !cs.is_null() {
        bt_ctf_stream_put((*cs).stream);
        free(cs as *mut c_void);
    }
}

unsafe fn ctf_stream(cw: *mut ctf_writer, cpu: c_int) -> *mut ctf_stream {
    let slot = (*cw).stream.add(cpu as usize);
    let mut cs = *slot;
    if cs.is_null() {
        cs = ctf_stream__create(cw, cpu);
        *slot = cs;
    }
    cs
}

unsafe fn get_sample_cpu(cw: *mut ctf_writer, sample: *mut perf_sample, evsel: *mut evsel) -> c_int {
    let mut cpu = 0;
    if (*evsel).core.attr.sample_type & PERF_SAMPLE_CPU != 0 {
        cpu = (*sample).cpu;
    }
    if cpu > (*cw).stream_cnt {
        pr_err(cstr!("Event was recorded for CPU %d, limit is at %d.\n"), cpu, (*cw).stream_cnt);
        cpu = 0;
    }
    cpu
}

unsafe fn is_flush_needed(cs: *mut ctf_stream) -> bool_t {
    (*cs).count >= STREAM_FLUSH_COUNT
}

unsafe extern "C" fn process_sample_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    let evsel = (*sample).evsel;
    let priv_ = (*evsel).priv_ as *mut evsel_priv;
    let cw = &mut (*c).writer as *mut ctf_writer;
    if priv_.is_null() {
        pr_err(cstr!("Failed to setup all events.\n"));
        return 0;
    }
    if perf_time__ranges_skip_sample((*c).ptime_range, (*c).range_num, (*sample).time) {
        (*c).skipped += 1;
        return 0;
    }
    let event_class = (*priv_).event_class;
    (*c).events_count += 1;
    (*c).events_size += (*_event).header.size as u64;
    let event = bt_ctf_event_create(event_class);
    if event.is_null() {
        pr_err(cstr!("Failed to create an CTF event\n"));
        return -1;
    }
    bt_ctf_clock_set_time((*cw).clock, (*sample).time);
    if add_generic_values(cw, event, evsel, sample) != 0 { return -1; }
    if (*evsel).core.attr.type_ == PERF_TYPE_TRACEPOINT {
        if add_tracepoint_values(cw, event_class, event, evsel, sample) != 0 { return -1; }
    }
    if (*evsel).core.attr.sample_type & PERF_SAMPLE_CALLCHAIN != 0 {
        if add_callchain_output_values(event_class, event, (*sample).callchain) != 0 { return -1; }
    }
    if evsel__is_bpf_output(evsel) {
        if add_bpf_output_values(event_class, event, sample) != 0 { return -1; }
    }
    let cs = ctf_stream(cw, get_sample_cpu(cw, sample, evsel));
    if !cs.is_null() {
        if is_flush_needed(cs) {
            ctf_stream__flush(cs);
        }
        (*cs).count += 1;
        bt_ctf_stream_append_event((*cs).stream, event);
    }
    bt_ctf_event_put(event);
    if !cs.is_null() { 0 } else { -1 }
}

unsafe fn process_non_sample_common(c: *mut convert, event_class: *mut bt_ctf_event_class, sample: *mut perf_sample) -> (*mut ctf_writer, *mut bt_ctf_event) {
    let cw = &mut (*c).writer as *mut ctf_writer;
    (*c).non_sample_count += 1;
    let event = bt_ctf_event_create(event_class);
    if !event.is_null() {
        bt_ctf_clock_set_time((*cw).clock, (*sample).time);
    }
    (cw, event)
}

unsafe fn finish_non_sample(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine, cw: *mut ctf_writer, event: *mut bt_ctf_event, process: unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int) -> c_int {
    let cs = ctf_stream(cw, 0);
    if !cs.is_null() {
        if is_flush_needed(cs) {
            ctf_stream__flush(cs);
        }
        (*cs).count += 1;
        bt_ctf_stream_append_event((*cs).stream, event);
    }
    bt_ctf_event_put(event);
    process(tool, _event, sample, machine)
}

unsafe extern "C" fn process_comm_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    (*c).events_size += (*_event).header.size as u64;
    let (cw, event) = process_non_sample_common(c, (*c).writer.comm_class, sample);
    if event.is_null() { return -1; }
    if value_set_u32(cw, event, cstr!("pid"), (*_event).comm.pid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("tid"), (*_event).comm.tid) != 0 { return -1; }
    if value_set_string(cw, event, cstr!("comm"), (*_event).comm.comm) != 0 { return -1; }
    finish_non_sample(tool, _event, sample, machine, cw, event, perf_event__process_comm)
}

unsafe extern "C" fn process_fork_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    (*c).events_size += (*_event).header.size as u64;
    let (cw, event) = process_non_sample_common(c, (*c).writer.fork_class, sample);
    if event.is_null() { return -1; }
    if value_set_u32(cw, event, cstr!("pid"), (*_event).fork.pid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("ppid"), (*_event).fork.ppid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("tid"), (*_event).fork.tid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("ptid"), (*_event).fork.ptid) != 0 { return -1; }
    if value_set_u64(cw, event, cstr!("time"), (*_event).fork.time) != 0 { return -1; }
    finish_non_sample(tool, _event, sample, machine, cw, event, perf_event__process_fork)
}

unsafe extern "C" fn process_exit_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    (*c).events_size += (*_event).header.size as u64;
    let (cw, event) = process_non_sample_common(c, (*c).writer.exit_class, sample);
    if event.is_null() { return -1; }
    if value_set_u32(cw, event, cstr!("pid"), (*_event).fork.pid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("ppid"), (*_event).fork.ppid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("tid"), (*_event).fork.tid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("ptid"), (*_event).fork.ptid) != 0 { return -1; }
    if value_set_u64(cw, event, cstr!("time"), (*_event).fork.time) != 0 { return -1; }
    finish_non_sample(tool, _event, sample, machine, cw, event, perf_event__process_exit)
}

unsafe extern "C" fn process_mmap_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    (*c).events_size += (*_event).header.size as u64;
    let (cw, event) = process_non_sample_common(c, (*c).writer.mmap_class, sample);
    if event.is_null() { return -1; }
    if value_set_u32(cw, event, cstr!("pid"), (*_event).mmap.pid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("tid"), (*_event).mmap.tid) != 0 { return -1; }
    if value_set_u64_hex(cw, event, cstr!("start"), (*_event).mmap.start) != 0 { return -1; }
    if value_set_string(cw, event, cstr!("filename"), (*_event).mmap.filename) != 0 { return -1; }
    finish_non_sample(tool, _event, sample, machine, cw, event, perf_event__process_mmap)
}

unsafe extern "C" fn process_mmap2_event(tool: *const perf_tool, _event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int {
    let c = tool as *mut convert;
    (*c).events_size += (*_event).header.size as u64;
    let (cw, event) = process_non_sample_common(c, (*c).writer.mmap2_class, sample);
    if event.is_null() { return -1; }
    if value_set_u32(cw, event, cstr!("pid"), (*_event).mmap2.pid) != 0 { return -1; }
    if value_set_u32(cw, event, cstr!("tid"), (*_event).mmap2.tid) != 0 { return -1; }
    if value_set_u64_hex(cw, event, cstr!("start"), (*_event).mmap2.start) != 0 { return -1; }
    if value_set_string(cw, event, cstr!("filename"), (*_event).mmap2.filename) != 0 { return -1; }
    finish_non_sample(tool, _event, sample, machine, cw, event, perf_event__process_mmap2)
}

/* If dup < 0, add a prefix. Else, add _dupl_X suffix. */
unsafe fn change_name(mut name: *mut c_char, orig_name: *mut c_char, dup: c_int) -> *mut c_char {
    let mut new_name: *mut c_char = ptr::null_mut();
    let len: size_t;
    if name.is_null() {
        name = orig_name;
    }
    if dup >= 10 {
        if name != orig_name { free(name as *mut c_void); }
        return ptr::null_mut();
    }
    /*
     * Add '_' prefix to potential keywork.  According to
     * Mathieu Desnoyers (https://lore.kernel.org/lkml/1074266107.40857.1422045946295.JavaMail.zimbra@efficios.com),
     * further CTF spec updating may require us to use '$'.
     */
    if dup < 0 {
        len = strlen(name) + size_of::<[c_char; 2]>();
    } else {
        len = strlen(orig_name) + size_of::<[c_char; 8]>();
    }
    new_name = malloc(len) as *mut c_char;
    if !new_name.is_null() {
        if dup < 0 {
            snprintf(new_name, len, cstr!("_%s"), name);
        } else {
            snprintf(new_name, len, cstr!("%s_dupl_%d"), orig_name, dup);
        }
    }
    if name != orig_name {
        free(name as *mut c_void);
    }
    new_name
}

unsafe fn event_class_add_field(event_class: *mut bt_ctf_event_class, type_: *mut bt_ctf_field_type, field: *mut tep_format_field) -> c_int {
    let mut t: *mut bt_ctf_field_type;
    let mut name: *mut c_char;
    let mut dup = 1;
    if (*field).alias != (*field).name {
        return bt_ctf_event_class_add_field(event_class, type_, (*field).alias);
    }
    name = (*field).name;
    /* If 'name' is a keywork, add prefix. */
    if bt_ctf_validate_identifier(name) != 0 {
        name = change_name(name, (*field).name, -1);
    }
    if name.is_null() {
        pr_err(cstr!("Failed to fix invalid identifier."));
        return -1;
    }
    loop {
        t = bt_ctf_event_class_get_field_by_name(event_class, name);
        if t.is_null() {
            break;
        }
        bt_ctf_field_type_put(t);
        name = change_name(name, (*field).name, dup);
        dup += 1;
        if name.is_null() {
            pr_err(cstr!("Failed to create dup name for '%s'\n"), (*field).name);
            return -1;
        }
    }
    let ret = bt_ctf_event_class_add_field(event_class, type_, name);
    if ret == 0 {
        (*field).alias = name;
    }
    ret
}

unsafe fn add_tracepoint_fields_types(cw: *mut ctf_writer, fields: *mut tep_format_field, event_class: *mut bt_ctf_event_class) -> c_int {
    let mut field = fields;
    while !field.is_null() {
        let mut flags = (*field).flags;
        let mut type_ = get_tracepoint_field_type(cw, field);
        if type_.is_null() { return -1; }
        if flags & TEP_FIELD_IS_STRING != 0 {
            flags &= !TEP_FIELD_IS_ARRAY;
        }
        if flags & TEP_FIELD_IS_ARRAY != 0 {
            type_ = bt_ctf_field_type_array_create(type_, (*field).arraylen);
        }
        let ret = event_class_add_field(event_class, type_, field);
        if flags & TEP_FIELD_IS_ARRAY != 0 {
            bt_ctf_field_type_put(type_);
        }
        if ret != 0 {
            pr_err(cstr!("Failed to add field '%s': %d\n"), (*field).name, ret);
            return -1;
        }
        field = (*field).next;
    }
    0
}

unsafe fn add_tracepoint_types(cw: *mut ctf_writer, evsel: *mut evsel, class: *mut bt_ctf_event_class) -> c_int {
    let tp_format = evsel__tp_format(evsel);
    let common_fields = if !tp_format.is_null() { (*tp_format).format.common_fields } else { ptr::null_mut() };
    let fields = if !tp_format.is_null() { (*tp_format).format.fields } else { ptr::null_mut() };
    let mut ret = add_tracepoint_fields_types(cw, common_fields, class);
    if ret == 0 {
        ret = add_tracepoint_fields_types(cw, fields, class);
    }
    ret
}

unsafe fn add_bpf_output_types(cw: *mut ctf_writer, class: *mut bt_ctf_event_class) -> c_int {
    let len_type = (*cw).data.named.u32;
    let seq_base_type = (*cw).data.named.u32_hex;
    let ret = bt_ctf_event_class_add_field(class, len_type, cstr!("raw_len"));
    if ret != 0 { return ret; }
    let seq_type = bt_ctf_field_type_sequence_create(seq_base_type, cstr!("raw_len"));
    if seq_type.is_null() { return -1; }
    bt_ctf_event_class_add_field(class, seq_type, cstr!("raw_data"))
}

unsafe fn add_field(event_class: *mut bt_ctf_event_class, type_: *mut bt_ctf_field_type, name: *const c_char) -> c_int {
    if bt_ctf_event_class_add_field(event_class, type_, name) != 0 {
        pr_err(cstr!("Failed to add field '%s';\n"), name);
        return -1;
    }
    0
}

unsafe fn add_generic_types(cw: *mut ctf_writer, evsel: *mut evsel, event_class: *mut bt_ctf_event_class) -> c_int {
    let type_ = (*evsel).core.attr.sample_type;
    if type_ & PERF_SAMPLE_IP != 0 && add_field(event_class, (*cw).data.named.u64_hex, cstr!("perf_ip")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_TID != 0 {
        if add_field(event_class, (*cw).data.named.s32, cstr!("perf_tid")) != 0 { return -1; }
        if add_field(event_class, (*cw).data.named.s32, cstr!("perf_pid")) != 0 { return -1; }
    }
    if (type_ & PERF_SAMPLE_ID != 0 || type_ & PERF_SAMPLE_IDENTIFIER != 0) && add_field(event_class, (*cw).data.named.u64, cstr!("perf_id")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_STREAM_ID != 0 && add_field(event_class, (*cw).data.named.u64, cstr!("perf_stream_id")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_PERIOD != 0 && add_field(event_class, (*cw).data.named.u64, cstr!("perf_period")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_WEIGHT != 0 && add_field(event_class, (*cw).data.named.u64, cstr!("perf_weight")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_DATA_SRC != 0 && add_field(event_class, (*cw).data.named.u64, cstr!("perf_data_src")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_TRANSACTION != 0 && add_field(event_class, (*cw).data.named.u64, cstr!("perf_transaction")) != 0 { return -1; }
    if type_ & PERF_SAMPLE_CALLCHAIN != 0 {
        if add_field(event_class, (*cw).data.named.u32, cstr!("perf_callchain_size")) != 0 { return -1; }
        if add_field(event_class, bt_ctf_field_type_sequence_create((*cw).data.named.u64_hex, cstr!("perf_callchain_size")), cstr!("perf_callchain")) != 0 { return -1; }
    }
    0
}

unsafe fn add_event(cw: *mut ctf_writer, evsel: *mut evsel) -> c_int {
    let name = evsel__name(evsel);
    if !(*evsel).priv_.is_null() {
        pr_err(cstr!("Error: attempt to add already added event %s\n"), name);
        return -1;
    }
    let event_class = bt_ctf_event_class_create(name);
    if event_class.is_null() { return -1; }
    let mut ret = add_generic_types(cw, evsel, event_class);
    if ret == 0 && (*evsel).core.attr.type_ == PERF_TYPE_TRACEPOINT {
        ret = add_tracepoint_types(cw, evsel, event_class);
    }
    if ret == 0 && evsel__is_bpf_output(evsel) {
        ret = add_bpf_output_types(cw, event_class);
    }
    if ret == 0 {
        ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    }
    if ret != 0 {
        bt_ctf_event_class_put(event_class);
        pr_err(cstr!("Failed to add event '%s'.\n"), name);
        return -1;
    }
    let priv_ = malloc(size_of::<evsel_priv>()) as *mut evsel_priv;
    if priv_.is_null() {
        bt_ctf_event_class_put(event_class);
        return -1;
    }
    (*priv_).event_class = event_class;
    (*evsel).priv_ = priv_ as *mut c_void;
    0
}

#[repr(C)]
enum setup_events_type {
    SETUP_EVENTS_ALL,
    SETUP_EVENTS_NOT_TRACEPOINT,
    SETUP_EVENTS_TRACEPOINT_ONLY,
}

unsafe fn setup_events(cw: *mut ctf_writer, session: *mut perf_session, type_: setup_events_type) -> c_int {
    let evlist = (*session).evlist;
    let mut evsel = if evlist.is_null() { ptr::null_mut() } else { (*evlist).entries };
    while !evsel.is_null() {
        let is_tracepoint = (*evsel).core.attr.type_ == PERF_TYPE_TRACEPOINT;
        if is_tracepoint && matches!(type_, setup_events_type::SETUP_EVENTS_NOT_TRACEPOINT) {
            evsel = (*evsel).next;
            continue;
        }
        if !is_tracepoint && matches!(type_, setup_events_type::SETUP_EVENTS_TRACEPOINT_ONLY) {
            evsel = (*evsel).next;
            continue;
        }
        let ret = add_event(cw, evsel);
        if ret != 0 { return ret; }
        evsel = (*evsel).next;
    }
    0
}

unsafe fn add_non_sample_field(cw: *mut ctf_writer, event_class: *mut bt_ctf_event_class, type_: *mut bt_ctf_field_type, name: *const c_char) -> c_int {
    if bt_ctf_event_class_add_field(event_class, type_, name) != 0 {
        pr_err(cstr!("Failed to add field '%s';\n"), name);
        return -1;
    }
    0
}

unsafe fn add_comm_event(cw: *mut ctf_writer) -> c_int {
    let event_class = bt_ctf_event_class_create(cstr!("perf_comm"));
    if event_class.is_null() { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("pid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("tid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.string, cstr!("comm")) != 0 { return -1; }
    let ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    if ret != 0 { return ret; }
    (*cw).comm_class = event_class;
    bt_ctf_event_class_put(event_class);
    0
}

unsafe fn add_fork_event(cw: *mut ctf_writer) -> c_int {
    let event_class = bt_ctf_event_class_create(cstr!("perf_fork"));
    if event_class.is_null() { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("pid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("ppid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("tid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("ptid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u64, cstr!("time")) != 0 { return -1; }
    let ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    if ret != 0 { return ret; }
    (*cw).fork_class = event_class;
    bt_ctf_event_class_put(event_class);
    0
}

unsafe fn add_exit_event(cw: *mut ctf_writer) -> c_int {
    let event_class = bt_ctf_event_class_create(cstr!("perf_exit"));
    if event_class.is_null() { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("pid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("ppid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("tid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("ptid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u64, cstr!("time")) != 0 { return -1; }
    let ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    if ret != 0 { return ret; }
    (*cw).exit_class = event_class;
    bt_ctf_event_class_put(event_class);
    0
}

unsafe fn add_mmap_event(cw: *mut ctf_writer) -> c_int {
    let event_class = bt_ctf_event_class_create(cstr!("perf_mmap"));
    if event_class.is_null() { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("pid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("tid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u64_hex, cstr!("start")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.string, cstr!("filename")) != 0 { return -1; }
    let ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    if ret != 0 { return ret; }
    (*cw).mmap_class = event_class;
    bt_ctf_event_class_put(event_class);
    0
}

unsafe fn add_mmap2_event(cw: *mut ctf_writer) -> c_int {
    let event_class = bt_ctf_event_class_create(cstr!("perf_mmap2"));
    if event_class.is_null() { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("pid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u32, cstr!("tid")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.u64_hex, cstr!("start")) != 0 { return -1; }
    if add_non_sample_field(cw, event_class, (*cw).data.named.string, cstr!("filename")) != 0 { return -1; }
    let ret = bt_ctf_stream_class_add_event_class((*cw).stream_class, event_class);
    if ret != 0 { return ret; }
    (*cw).mmap2_class = event_class;
    bt_ctf_event_class_put(event_class);
    0
}

unsafe fn setup_non_sample_events(cw: *mut ctf_writer, _session: *mut perf_session) -> c_int {
    let mut ret = add_comm_event(cw);
    if ret != 0 { return ret; }
    ret = add_exit_event(cw);
    if ret != 0 { return ret; }
    ret = add_fork_event(cw);
    if ret != 0 { return ret; }
    ret = add_mmap_event(cw);
    if ret != 0 { return ret; }
    ret = add_mmap2_event(cw);
    if ret != 0 { return ret; }
    0
}

unsafe fn cleanup_events(session: *mut perf_session) {
    let evlist = (*session).evlist;
    let mut evsel = if evlist.is_null() { ptr::null_mut() } else { (*evlist).entries };
    while !evsel.is_null() {
        let priv_ = (*evsel).priv_ as *mut evsel_priv;
        if !priv_.is_null() {
            bt_ctf_event_class_put((*priv_).event_class);
        }
        zfree(&mut (*evsel).priv_ as *mut *mut c_void as *mut c_void);
        evsel = (*evsel).next;
    }
    evlist__put(evlist);
    (*session).evlist = ptr::null_mut();
}

unsafe fn setup_streams(cw: *mut ctf_writer, session: *mut perf_session) -> c_int {
    let env = perf_session__env(session);
    /*
     * Try to get the number of cpus used in the data file,
     * if not present fallback to the MAX_CPUS.
     */
    let ncpus = if (*env).nr_cpus_avail != 0 { (*env).nr_cpus_avail } else { MAX_CPUS };
    let stream = calloc(ncpus as size_t, size_of::<*mut ctf_stream>()) as *mut *mut ctf_stream;
    if stream.is_null() {
        pr_err(cstr!("Failed to allocate streams.\n"));
        return -12;
    }
    (*cw).stream = stream;
    (*cw).stream_cnt = ncpus;
    0
}

unsafe fn free_streams(cw: *mut ctf_writer) {
    let mut cpu = 0;
    while cpu < (*cw).stream_cnt {
        ctf_stream__delete(*(*cw).stream.add(cpu as usize));
        cpu += 1;
    }
    zfree(&mut (*cw).stream as *mut *mut *mut ctf_stream as *mut c_void);
}

unsafe fn ctf_writer__setup_env(cw: *mut ctf_writer, session: *mut perf_session) -> c_int {
    let env = perf_session__env(session);
    let writer = (*cw).writer;
    if !(*env).hostname.is_null() && bt_ctf_writer_add_environment_field(writer, cstr!("host"), (*env).hostname) != 0 { return -1; }
    if bt_ctf_writer_add_environment_field(writer, cstr!("sysname"), cstr!("Linux")) != 0 { return -1; }
    let release = perf_env__os_release(env);
    if !release.is_null() && bt_ctf_writer_add_environment_field(writer, cstr!("release"), release) != 0 { return -1; }
    if !(*env).version.is_null() && bt_ctf_writer_add_environment_field(writer, cstr!("version"), (*env).version) != 0 { return -1; }
    if !(*env).arch.is_null() && bt_ctf_writer_add_environment_field(writer, cstr!("machine"), (*env).arch) != 0 { return -1; }
    if bt_ctf_writer_add_environment_field(writer, cstr!("domain"), cstr!("kernel")) != 0 { return -1; }
    if bt_ctf_writer_add_environment_field(writer, cstr!("tracer_name"), cstr!("perf")) != 0 { return -1; }
    0
}

unsafe extern "C" fn process_feature_event(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    let c = tool as *mut convert;
    let cw = &mut (*c).writer as *mut ctf_writer;
    let fe = &mut (*event).feat as *mut perf_record_header_feature;
    let ret = perf_event__process_feature(tool, session, event);
    if ret != 0 { return ret; }
    match (*fe).feat_id {
        HEADER_EVENT_DESC => return setup_events(cw, session, setup_events_type::SETUP_EVENTS_NOT_TRACEPOINT),
        HEADER_HOSTNAME => {
            if !(*session).header.env.hostname.is_null() {
                return bt_ctf_writer_add_environment_field((*cw).writer, cstr!("host"), (*session).header.env.hostname);
            }
        }
        HEADER_OSRELEASE => {
            if !(*session).header.env.os_release.is_null() {
                return bt_ctf_writer_add_environment_field((*cw).writer, cstr!("release"), (*session).header.env.os_release);
            }
        }
        HEADER_VERSION => {
            if !(*session).header.env.version.is_null() {
                return bt_ctf_writer_add_environment_field((*cw).writer, cstr!("version"), (*session).header.env.version);
            }
        }
        HEADER_ARCH => {
            if !(*session).header.env.arch.is_null() {
                return bt_ctf_writer_add_environment_field((*cw).writer, cstr!("machine"), (*session).header.env.arch);
            }
        }
        _ => {}
    }
    0
}

unsafe extern "C" fn process_tracing_data(tool: *const perf_tool, session: *mut perf_session, event: *mut perf_event) -> c_int {
    let c = tool as *mut convert;
    let cw = &mut (*c).writer as *mut ctf_writer;
    let ret = perf_event__process_tracing_data(tool, session, event);
    if ret < 0 { return ret; }
    /*
     * Now the attr was set up by the attr event, the name by the feature
     * event desc event and the tracepoint data set up above, the tracepoint
     * babeltrace events can be added.
     */
    setup_events(cw, session, setup_events_type::SETUP_EVENTS_TRACEPOINT_ONLY)
}

unsafe fn ctf_writer__setup_clock(cw: *mut ctf_writer, session: *mut perf_session, tod: bool_t) -> c_int {
    let clock = (*cw).clock;
    let mut desc = cstr!("perf clock");
    let mut offset: i64 = 0;
    if tod {
        let env = perf_session__env(session);
        if !(*env).clock.enabled {
            pr_err(cstr!("Can't provide --tod time, missing clock data. Please record with -k/--clockid option.\n"));
            return -1;
        }
        desc = clockid_name((*env).clock.clockid);
        offset = (*env).clock.tod_ns - (*env).clock.clockid_ns;
    }
    if bt_ctf_clock_set_frequency(clock, 1000000000) != 0 { return -1; }
    if bt_ctf_clock_set_offset(clock, offset) != 0 { return -1; }
    if bt_ctf_clock_set_description(clock, desc) != 0 { return -1; }
    if bt_ctf_clock_set_precision(clock, 10) != 0 { return -1; }
    if bt_ctf_clock_set_is_absolute(clock, 0) != 0 { return -1; }
    0
}

unsafe fn create_int_type(size: c_int, sign: bool_t, hex: bool_t) -> *mut bt_ctf_field_type {
    let type_ = bt_ctf_field_type_integer_create(size);
    if type_.is_null() { return ptr::null_mut(); }
    if sign && bt_ctf_field_type_integer_set_signed(type_, 1) != 0 {
        bt_ctf_field_type_put(type_);
        return ptr::null_mut();
    }
    if hex && bt_ctf_field_type_integer_set_base(type_, BT_CTF_INTEGER_BASE_HEXADECIMAL) != 0 {
        bt_ctf_field_type_put(type_);
        return ptr::null_mut();
    }
    /* C preprocessor selected byte order from __BYTE_ORDER__. */
    #[cfg(target_endian = "big")]
    {
        bt_ctf_field_type_set_byte_order(type_, BT_CTF_BYTE_ORDER_BIG_ENDIAN);
    }
    #[cfg(not(target_endian = "big"))]
    {
        bt_ctf_field_type_set_byte_order(type_, BT_CTF_BYTE_ORDER_LITTLE_ENDIAN);
    }
    type_
}

unsafe fn ctf_writer__cleanup_data(cw: *mut ctf_writer) {
    let mut i = 0;
    while i < (*cw).data.array.len() {
        bt_ctf_field_type_put((*cw).data.array[i]);
        i += 1;
    }
}

unsafe fn ctf_writer__init_data(cw: *mut ctf_writer) -> c_int {
    (*cw).data.named.s64 = create_int_type(64, true, false);
    if (*cw).data.named.s64.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.u64 = create_int_type(64, false, false);
    if (*cw).data.named.u64.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.s32 = create_int_type(32, true, false);
    if (*cw).data.named.s32.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.u32 = create_int_type(32, false, false);
    if (*cw).data.named.u32.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.u32_hex = create_int_type(32, false, true);
    if (*cw).data.named.u32_hex.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.u64_hex = create_int_type(64, false, true);
    if (*cw).data.named.u64_hex.is_null() { ctf_writer__cleanup_data(cw); return -1; }
    (*cw).data.named.string = bt_ctf_field_type_string_create();
    if !(*cw).data.named.string.is_null() { return 0; }
    ctf_writer__cleanup_data(cw);
    pr_err(cstr!("Failed to create data types.\n"));
    -1
}

unsafe fn ctf_writer__cleanup(cw: *mut ctf_writer) {
    ctf_writer__cleanup_data(cw);
    bt_ctf_clock_put((*cw).clock);
    free_streams(cw);
    bt_ctf_stream_class_put((*cw).stream_class);
    bt_ctf_writer_put((*cw).writer);
    /* and NULL all the pointers */
    memset(cw as *mut c_void, 0, size_of::<ctf_writer>());
}

unsafe fn ctf_writer__init(cw: *mut ctf_writer, path: *const c_char, session: *mut perf_session, tod: bool_t) -> c_int {
    let writer = bt_ctf_writer_create(path);
    if writer.is_null() {
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    (*cw).writer = writer;
    let clock = bt_ctf_clock_create(cstr!("perf_clock"));
    if clock.is_null() {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    (*cw).clock = clock;
    if ctf_writer__setup_clock(cw, session, tod) != 0 {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    let stream_class = bt_ctf_stream_class_create(cstr!("perf_stream"));
    if stream_class.is_null() {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    (*cw).stream_class = stream_class;
    if bt_ctf_stream_class_set_clock(stream_class, clock) != 0 {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    if ctf_writer__init_data(cw) != 0 {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    let pkt_ctx_type = bt_ctf_stream_class_get_packet_context_type(stream_class);
    if pkt_ctx_type.is_null() {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    let ret = bt_ctf_field_type_structure_add_field(pkt_ctx_type, (*cw).data.named.u32, cstr!("cpu_id"));
    bt_ctf_field_type_put(pkt_ctx_type);
    if ret != 0 {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    if bt_ctf_writer_add_clock(writer, clock) != 0 {
        ctf_writer__cleanup(cw);
        pr_err(cstr!("Failed to setup CTF writer.\n"));
        return -1;
    }
    0
}

unsafe fn ctf_writer__flush_streams(cw: *mut ctf_writer) -> c_int {
    let mut cpu = 0;
    let mut ret = 0;
    while cpu < (*cw).stream_cnt && ret == 0 {
        ret = ctf_stream__flush(*(*cw).stream.add(cpu as usize));
        cpu += 1;
    }
    ret
}

unsafe extern "C" fn convert__config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    let c = cb as *mut convert;
    if strcmp(var, cstr!("convert.queue-size")) == 0 {
        return perf_config_u64(&mut (*c).queue_size, var, value);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bt_convert__perf2ctf(input: *const c_char, path: *const c_char, opts: *mut perf_data_convert_opts) -> c_int {
    let mut data = perf_data {
        path: input,
        mode: PERF_DATA_MODE_READ,
        force: (*opts).force,
    };
    let mut c: convert = zeroed();
    let cw = &mut c.writer as *mut ctf_writer;
    let mut err: c_int;

    perf_tool__init(&mut c.tool, true);
    c.tool.sample = Some(process_sample_event);
    c.tool.mmap = Some(perf_event__process_mmap);
    c.tool.mmap2 = Some(perf_event__process_mmap2);
    c.tool.comm = Some(perf_event__process_comm);
    c.tool.exit = Some(perf_event__process_exit);
    c.tool.fork = Some(perf_event__process_fork);
    c.tool.lost = Some(perf_event__process_lost);
    c.tool.tracing_data = Some(process_tracing_data);
    c.tool.build_id = Some(perf_event__process_build_id);
    c.tool.namespaces = Some(perf_event__process_namespaces);
    c.tool.finished_round = Some(perf_event__process_finished_round);
    c.tool.attr = Some(perf_event__process_attr);
    c.tool.feature = Some(process_feature_event);
    c.tool.ordering_requires_timestamps = true;

    if (*opts).all {
        c.tool.comm = Some(process_comm_event);
        c.tool.exit = Some(process_exit_event);
        c.tool.fork = Some(process_fork_event);
        c.tool.mmap = Some(process_mmap_event);
        c.tool.mmap2 = Some(process_mmap2_event);
    }

    err = perf_config(convert__config, &mut c as *mut convert as *mut c_void);
    if err != 0 { return err; }
    err = -1;
    let session = perf_session__new(&mut data, &mut c.tool);
    if IS_ERR(session as *const c_void) {
        return PTR_ERR(session as *const c_void);
    }
    if !(*opts).time_str.is_null() {
        err = perf_time__parse_for_ranges((*opts).time_str, session, &mut c.ptime_range, &mut c.range_size, &mut c.range_num);
        if err < 0 {
            perf_session__delete(session);
            pr_err(cstr!("Error during conversion setup.\n"));
            return err;
        }
    }
    if ctf_writer__init(cw, path, session, (*opts).tod) != 0 {
        if !c.ptime_range.is_null() { zfree(&mut c.ptime_range as *mut *mut perf_time_interval as *mut c_void); }
        perf_session__delete(session);
        pr_err(cstr!("Error during conversion setup.\n"));
        return err;
    }
    if c.queue_size != 0 {
        ordered_events__set_alloc_size(&mut (*session).ordered_events, c.queue_size);
    }
    if ctf_writer__setup_env(cw, session) != 0 { ctf_writer__cleanup(cw); perf_session__delete(session); return err; }
    if setup_events(cw, session, setup_events_type::SETUP_EVENTS_ALL) != 0 { ctf_writer__cleanup(cw); perf_session__delete(session); return err; }
    if (*opts).all && setup_non_sample_events(cw, session) != 0 { ctf_writer__cleanup(cw); perf_session__delete(session); return err; }
    if setup_streams(cw, session) != 0 { ctf_writer__cleanup(cw); perf_session__delete(session); return err; }

    err = perf_session__process_events(session);
    if err == 0 {
        err = ctf_writer__flush_streams(cw);
    } else {
        pr_err(cstr!("Error during conversion.\n"));
    }

    fprintf(stderr, cstr!("[ perf data convert: Converted '%s' into CTF data '%s' ]\n"), data.path, path);
    fprintf(stderr, cstr!("[ perf data convert: Converted and wrote %.3f MB (%llu samples"), c.events_size as f64 / 1024.0 / 1024.0, c.events_count);
    if c.non_sample_count == 0 {
        fprintf(stderr, cstr!(") ]\n"));
    } else {
        fprintf(stderr, cstr!(", %llu non-samples) ]\n"), c.non_sample_count);
    }
    if c.skipped != 0 {
        fprintf(stderr, cstr!("[ perf data convert: Skipped %llu samples ]\n"), c.skipped);
    }
    if !c.ptime_range.is_null() {
        zfree(&mut c.ptime_range as *mut *mut perf_time_interval as *mut c_void);
    }
    cleanup_events(session);
    perf_session__delete(session);
    ctf_writer__cleanup(cw);
    err
}
