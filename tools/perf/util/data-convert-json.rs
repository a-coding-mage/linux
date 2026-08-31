// SPDX-License-Identifier: GPL-2.0-only
/*
 * JSON export.
 *
 * Copyright (C) 2021, CodeWeavers Inc. <nfraser@codeweavers.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type FILE = c_void;
type time_t = i64;
type u8 = u8;
type u64 = u64;
type s64 = i64;

const PERF_RECORD_MISC_USER: u8 = 2;
const PERF_RECORD_MISC_HYPERVISOR: u8 = 3;
const PERF_RECORD_MISC_KERNEL: u8 = 1;
const PERF_CONTEXT_MAX: u64 = !0u64 - 4095;
const PERF_CONTEXT_HV: u64 = !0u64 - 32;
const PERF_CONTEXT_KERNEL: u64 = !0u64 - 128;
const PERF_CONTEXT_USER: u64 = !0u64 - 512;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_DATA_MODE_READ: c_int = 0;
const O_CREAT: c_int = 0o100;
const O_WRONLY: c_int = 0o1;
const O_TRUNC: c_int = 0o1000;
const O_EXCL: c_int = 0o200;
const EEXIST: c_int = 17;

#[repr(C)]
pub struct perf_tool {
    pub sample: Option<
        unsafe extern "C" fn(
            *const perf_tool,
            *mut perf_event,
            *mut perf_sample,
            *mut machine,
        ) -> c_int,
    >,
    pub mmap: Option<unsafe extern "C" fn()>,
    pub mmap2: Option<unsafe extern "C" fn()>,
    pub comm: Option<unsafe extern "C" fn()>,
    pub namespaces: Option<unsafe extern "C" fn()>,
    pub cgroup: Option<unsafe extern "C" fn()>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub fork: Option<unsafe extern "C" fn()>,
    pub lost: Option<unsafe extern "C" fn()>,
    /* HAVE_LIBTRACEEVENT: tracing_data is present when libtraceevent support is enabled. */
    pub tracing_data: Option<unsafe extern "C" fn()>,
    pub build_id: Option<unsafe extern "C" fn()>,
    pub id_index: Option<unsafe extern "C" fn()>,
    pub auxtrace_info: Option<unsafe extern "C" fn()>,
    pub auxtrace: Option<unsafe extern "C" fn()>,
    pub event_update: Option<unsafe extern "C" fn()>,
    pub attr: Option<unsafe extern "C" fn()>,
    pub feature: Option<unsafe extern "C" fn()>,
    pub ordering_requires_timestamps: bool,
}

#[repr(C)]
pub struct convert_json {
    pub tool: perf_tool,
    pub out: *mut FILE,
    pub first: bool,
    pub ptime_range: *mut perf_time_interval,
    pub range_size: c_int,
    pub range_num: c_int,

    pub events_count: u64,
    pub skipped: u64,
}

#[repr(C)]
pub struct perf_time_interval {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
    pub namelen: c_uint,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct callchain {
    pub nr: c_uint,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    pub evlist: *mut evlist,
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub time: u64,
    pub cpu: c_int,
    pub callchain: *mut callchain,
    pub ip: u64,
    pub raw_data: *mut c_void,
}

#[repr(C)]
pub struct perf_header {
    pub version: c_uint,
    pub data_offset: u64,
    pub data_size: u64,
    pub feat_offset: u64,
}

#[repr(C)]
pub struct perf_env_clock {
    pub enabled: bool,
    pub clockid: c_uint,
    pub clockid_ns: u64,
    pub tod_ns: u64,
}

#[repr(C)]
pub struct perf_env {
    pub hostname: *const c_char,
    pub arch: *const c_char,
    pub cpu_desc: *const c_char,
    pub cpuid: *const c_char,
    pub nr_cpus_online: c_uint,
    pub nr_cpus_avail: c_uint,
    pub clock: perf_env_clock,
    pub version: *const c_char,
    pub nr_cmdline: c_int,
    pub cmdline_argv: *mut *const c_char,
}

#[repr(C)]
pub struct perf_data {
    pub mode: c_int,
    pub path: *const c_char,
    pub force: bool,
}

#[repr(C)]
pub struct perf_session {
    pub header: perf_header,
    pub data: *mut perf_data,
}

#[repr(C)]
pub struct perf_data_convert_opts {
    pub force: bool,
    pub all: bool,
    pub tod: bool,
    pub time_str: *const c_char,
}

#[repr(C)]
pub struct stat {
    pub st_mtime: time_t,
}

#[repr(C)]
pub struct tm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_format_field {
    pub name: *const c_char,
}

#[repr(C)]
pub struct trace_seq {
    pub buffer: *const c_char,
}

extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut FILE;

    fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, format: *const c_char, args: *mut c_void) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn ftell(stream: *mut FILE) -> c_long;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn gmtime(timep: *const time_t) -> *mut tm;
    fn strftime(s: *mut c_char, max: usize, format: *const c_char, tm: *const tm) -> usize;
    fn free(ptr: *mut c_void);

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample)
        -> c_int;
    fn perf_time__ranges_skip_sample(
        ptime_range: *mut perf_time_interval,
        range_num: c_int,
        time: u64,
    ) -> bool;
    fn __evlist__combined_sample_type(evlist: *mut evlist) -> u64;
    fn thread__pid(thread: *mut thread) -> c_int;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn thread__cpu(thread: *mut thread) -> c_int;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__find_symbol(
        thread: *mut thread,
        cpumode: u8,
        ip: u64,
        al: *mut addr_location,
    ) -> bool;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn pr_err(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);

    fn perf_session__env(session: *mut perf_session) -> *mut perf_env;
    fn perf_data__fd(data: *mut perf_data) -> c_int;
    fn perf_env__os_release(env: *mut perf_env) -> *const c_char;
    fn perf_tool__init(tool: *mut perf_tool, ordered_events: bool);
    fn perf_session__new(data: *mut perf_data, tool: *mut perf_tool) -> *mut perf_session;
    fn perf_session__delete(session: *mut perf_session);
    fn perf_session__process_events(session: *mut perf_session) -> c_int;
    fn symbol__init(env: *mut perf_env) -> c_int;
    fn perf_time__parse_for_ranges(
        time_str: *const c_char,
        session: *mut perf_session,
        ptime_range: *mut *mut perf_time_interval,
        range_size: *mut c_int,
        range_num: *mut c_int,
    ) -> c_int;
    fn zfree(ptr: *mut *mut perf_time_interval);
    fn IS_ERR(ptr: *const c_void) -> bool;

    fn perf_event__process_mmap();
    fn perf_event__process_mmap2();
    fn perf_event__process_comm();
    fn perf_event__process_namespaces();
    fn perf_event__process_cgroup();
    fn perf_event__process_exit();
    fn perf_event__process_fork();
    fn perf_event__process_lost();
    /* HAVE_LIBTRACEEVENT */
    fn perf_event__process_tracing_data();
    fn perf_event__process_build_id();
    fn perf_event__process_id_index();
    fn perf_event__process_auxtrace_info();
    fn perf_event__process_auxtrace();
    fn perf_event__process_event_update();
    fn perf_event__process_attr();
    fn perf_event__process_feature();

    /* HAVE_LIBTRACEEVENT */
    fn evsel__tp_format(evsel: *mut evsel) -> *mut tep_event;
    fn tep_event_fields(event: *mut tep_event) -> *mut *mut tep_format_field;
    fn trace_seq_init(s: *mut trace_seq);
    fn tep_print_field(s: *mut trace_seq, data: *mut c_void, field: *mut tep_format_field);
    fn trace_seq_destroy(s: *mut trace_seq);
}

type c_long = i64;

unsafe fn convert_json_from_tool(tool: *const perf_tool) -> *mut convert_json {
    tool as *mut convert_json
}

// Outputs a JSON-encoded string surrounded by quotes with characters escaped.
unsafe fn output_json_string(out: *mut FILE, mut s: *const c_char) {
    fputc(b'"' as c_int, out);
    if s.is_null() {
        goto_out(out);
        return;
    }

    while *s != 0 {
        match *s {
            // required escapes with special forms as per RFC 8259
            x if x == b'"' as c_char => {
                fputs(c"\\\"".as_ptr(), out);
            }
            x if x == b'\\' as c_char => {
                fputs(c"\\\\".as_ptr(), out);
            }
            x if x == b'\x08' as c_char => {
                fputs(c"\\b".as_ptr(), out);
            }
            x if x == b'\x0c' as c_char => {
                fputs(c"\\f".as_ptr(), out);
            }
            x if x == b'\n' as c_char => {
                fputs(c"\\n".as_ptr(), out);
            }
            x if x == b'\r' as c_char => {
                fputs(c"\\r".as_ptr(), out);
            }
            x if x == b'\t' as c_char => {
                fputs(c"\\t".as_ptr(), out);
            }
            _ => {
                // all other control characters must be escaped by hex code
                if (*s as u8) <= 0x1f {
                    fprintf(out, c"\\u%04x".as_ptr(), *s as c_int);
                } else {
                    fputc(*s as c_int, out);
                }
            }
        }

        s = s.add(1);
    }
    goto_out(out);
}

unsafe fn goto_out(out: *mut FILE) {
    fputc(b'"' as c_int, out);
}

// Outputs an optional comma, newline and indentation to delimit a new value
// from the previous one in a JSON object or array.
unsafe fn output_json_delimiters(out: *mut FILE, comma: bool, depth: c_int) {
    let mut i: c_int;

    if comma {
        fputc(b',' as c_int, out);
    }
    fputc(b'\n' as c_int, out);
    i = 0;
    while i < depth {
        fputc(b'\t' as c_int, out);
        i += 1;
    }
}

macro_rules! output_json_format {
    ($out:expr, $comma:expr, $depth:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        output_json_delimiters($out, $comma, $depth);
        fprintf($out, $format.as_ptr() $(, $arg)*);
    }};
}

// Outputs a JSON key-value pair where the value is a string.
unsafe fn output_json_key_string(
    out: *mut FILE,
    comma: bool,
    depth: c_int,
    key: *const c_char,
    value: *const c_char,
) {
    output_json_delimiters(out, comma, depth);
    output_json_string(out, key);
    fputs(c": ".as_ptr(), out);
    output_json_string(out, value);
}

macro_rules! output_json_key_format {
    ($out:expr, $comma:expr, $depth:expr, $key:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        output_json_delimiters($out, $comma, $depth);
        output_json_string($out, $key.as_ptr());
        fputs(c": ".as_ptr(), $out);
        fprintf($out, $format.as_ptr() $(, $arg)*);
    }};
}

unsafe fn output_sample_callchain_entry(
    tool: *const perf_tool,
    ip: u64,
    al: *mut addr_location,
) {
    let c = convert_json_from_tool(tool);
    let out = (*c).out;

    output_json_format!(out, false, 4, c"{");
    output_json_key_format!(out, false, 5, c"ip", c"\"0x%lx\"", ip);

    if !al.is_null() && !(*al).sym.is_null() && (*(*al).sym).namelen != 0 {
        let dso = if !(*al).map.is_null() {
            map__dso((*al).map)
        } else {
            ptr::null_mut()
        };

        fputc(b',' as c_int, out);
        output_json_key_string(out, false, 5, c"symbol".as_ptr(), (*(*al).sym).name);

        if !dso.is_null() {
            let dso_name = dso__short_name(dso);

            if !dso_name.is_null() && strlen(dso_name) > 0 {
                fputc(b',' as c_int, out);
                output_json_key_string(out, false, 5, c"dso".as_ptr(), dso_name);
            }
        }
    }

    output_json_format!(out, false, 4, c"}");
}

unsafe extern "C" fn process_sample_event(
    tool: *const perf_tool,
    _event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let c = convert_json_from_tool(tool);
    let out = (*c).out;
    let mut al = MaybeUninit::<addr_location>::uninit();
    let sample_type = __evlist__combined_sample_type((*(*sample).evsel).evlist);
    let mut cpumode: u8 = PERF_RECORD_MISC_USER;

    addr_location__init(al.as_mut_ptr());
    let al = al.as_mut_ptr();
    if machine__resolve(machine, al, sample) < 0 {
        pr_err(c"Sample resolution failed!\n".as_ptr());
        addr_location__exit(al);
        return -1;
    }

    if perf_time__ranges_skip_sample((*c).ptime_range, (*c).range_num, (*sample).time) {
        (*c).skipped += 1;
        addr_location__exit(al);
        return 0;
    }

    (*c).events_count += 1;

    if (*c).first {
        (*c).first = false;
    } else {
        fputc(b',' as c_int, out);
    }
    output_json_format!(out, false, 2, c"{");

    output_json_key_format!(out, false, 3, c"timestamp", c"%ld", (*sample).time as i64);
    output_json_key_format!(out, true, 3, c"pid", c"%i", thread__pid((*al).thread));
    output_json_key_format!(out, true, 3, c"tid", c"%i", thread__tid((*al).thread));

    if (sample_type & PERF_SAMPLE_CPU) != 0 {
        output_json_key_format!(out, true, 3, c"cpu", c"%i", (*sample).cpu);
    } else if thread__cpu((*al).thread) >= 0 {
        output_json_key_format!(out, true, 3, c"cpu", c"%i", thread__cpu((*al).thread));
    }

    output_json_key_string(out, true, 3, c"comm".as_ptr(), thread__comm_str((*al).thread));

    output_json_key_format!(out, true, 3, c"callchain", c"[");
    if !(*sample).callchain.is_null() {
        let mut i: c_uint;
        let mut ok: bool;
        let mut first_callchain = true;

        i = 0;
        while i < (*(*sample).callchain).nr {
            let ip = *(*sample).callchain.cast::<u8>().add(core::mem::size_of::<c_uint>()).cast::<u64>().add(i as usize);
            let mut tal = MaybeUninit::<addr_location>::uninit();

            if ip >= PERF_CONTEXT_MAX {
                match ip {
                    PERF_CONTEXT_HV => {
                        cpumode = PERF_RECORD_MISC_HYPERVISOR;
                    }
                    PERF_CONTEXT_KERNEL => {
                        cpumode = PERF_RECORD_MISC_KERNEL;
                    }
                    PERF_CONTEXT_USER => {
                        cpumode = PERF_RECORD_MISC_USER;
                    }
                    _ => {
                        pr_debug(c"invalid callchain context: %ld\n".as_ptr(), ip as s64);
                    }
                }
                i += 1;
                continue;
            }

            if first_callchain {
                first_callchain = false;
            } else {
                fputc(b',' as c_int, out);
            }

            addr_location__init(tal.as_mut_ptr());
            let tal = tal.as_mut_ptr();
            ok = thread__find_symbol((*al).thread, cpumode, ip, tal);
            output_sample_callchain_entry(tool, ip, if ok { tal } else { ptr::null_mut() });
            addr_location__exit(tal);
            i += 1;
        }
    } else {
        output_sample_callchain_entry(tool, (*sample).ip, al);
    }
    output_json_format!(out, false, 3, c"]");

    /* HAVE_LIBTRACEEVENT */
    if !(*sample).raw_data.is_null() {
        let tp_format = evsel__tp_format((*sample).evsel);
        let fields = if !tp_format.is_null() {
            tep_event_fields(tp_format)
        } else {
            ptr::null_mut()
        };

        if !fields.is_null() {
            let mut i: c_int = 0;

            while !(*fields.add(i as usize)).is_null() {
                let mut s = MaybeUninit::<trace_seq>::uninit();

                trace_seq_init(s.as_mut_ptr());
                let s_ptr = s.as_mut_ptr();
                tep_print_field(s_ptr, (*sample).raw_data, *fields.add(i as usize));
                output_json_key_string(
                    out,
                    true,
                    3,
                    (**fields.add(i as usize)).name,
                    (*s_ptr).buffer,
                );
                trace_seq_destroy(s_ptr);

                i += 1;
            }
            free(fields as *mut c_void);
        }
    }

    output_json_format!(out, false, 2, c"}");
    addr_location__exit(al);
    0
}

unsafe fn output_headers(session: *mut perf_session, c: *mut convert_json) {
    let mut st = MaybeUninit::<stat>::uninit();
    let header = &(*session).header as *const perf_header;
    let env = perf_session__env(session);
    let mut ret: c_int;
    let fd = perf_data__fd((*session).data);
    let mut i: c_int;
    let out = (*c).out;

    output_json_key_format!(out, false, 2, c"header-version", c"%u", (*header).version);

    ret = fstat(fd, st.as_mut_ptr());
    if ret >= 0 {
        let st = st.assume_init();
        let stctime: time_t = st.st_mtime;
        let mut buf = [0 as c_char; 256];

        strftime(
            buf.as_mut_ptr(),
            buf.len(),
            c"%FT%TZ".as_ptr(),
            gmtime(&stctime),
        );
        output_json_key_string(out, true, 2, c"captured-on".as_ptr(), buf.as_ptr());
    } else {
        pr_debug(c"Failed to get mtime of source file, not writing captured-on".as_ptr());
    }

    output_json_key_format!(out, true, 2, c"data-offset", c"%lu", (*header).data_offset);
    output_json_key_format!(out, true, 2, c"data-size", c"%lu", (*header).data_size);
    output_json_key_format!(out, true, 2, c"feat-offset", c"%lu", (*header).feat_offset);

    output_json_key_string(out, true, 2, c"hostname".as_ptr(), (*env).hostname);
    output_json_key_string(out, true, 2, c"os-release".as_ptr(), perf_env__os_release(env));
    output_json_key_string(out, true, 2, c"arch".as_ptr(), (*env).arch);

    if !(*env).cpu_desc.is_null() {
        output_json_key_string(out, true, 2, c"cpu-desc".as_ptr(), (*env).cpu_desc);
    }

    output_json_key_string(out, true, 2, c"cpuid".as_ptr(), (*env).cpuid);
    output_json_key_format!(out, true, 2, c"nrcpus-online", c"%u", (*env).nr_cpus_online);
    output_json_key_format!(out, true, 2, c"nrcpus-avail", c"%u", (*env).nr_cpus_avail);

    if (*env).clock.enabled {
        output_json_key_format!(out, true, 2, c"clockid", c"%u", (*env).clock.clockid);
        output_json_key_format!(out, true, 2, c"clock-time", c"%lu", (*env).clock.clockid_ns);
        output_json_key_format!(out, true, 2, c"real-time", c"%lu", (*env).clock.tod_ns);
    }

    output_json_key_string(out, true, 2, c"perf-version".as_ptr(), (*env).version);

    output_json_key_format!(out, true, 2, c"cmdline", c"[");
    i = 0;
    while i < (*env).nr_cmdline {
        output_json_delimiters(out, i != 0, 3);
        output_json_string((*c).out, *(*env).cmdline_argv.add(i as usize));
        i += 1;
    }
    output_json_format!(out, false, 2, c"]");
}

#[no_mangle]
pub unsafe extern "C" fn bt_convert__perf2json(
    _input_name: *const c_char,
    output_name: *const c_char,
    opts: *mut perf_data_convert_opts,
) -> c_int {
    let mut session: *mut perf_session;
    let mut fd: c_int;
    let mut ret: c_int = -1;
    let mut c = convert_json {
        tool: MaybeUninit::<perf_tool>::zeroed().assume_init(),
        out: ptr::null_mut(),
        first: true,
        events_count: 0,
        ptime_range: ptr::null_mut(),
        range_size: 0,
        range_num: 0,
        skipped: 0,
    };
    let mut data = perf_data {
        mode: PERF_DATA_MODE_READ,
        path: _input_name,
        force: (*opts).force,
    };

    perf_tool__init(&mut c.tool, true);
    c.tool.sample = Some(process_sample_event);
    c.tool.mmap = Some(perf_event__process_mmap);
    c.tool.mmap2 = Some(perf_event__process_mmap2);
    c.tool.comm = Some(perf_event__process_comm);
    c.tool.namespaces = Some(perf_event__process_namespaces);
    c.tool.cgroup = Some(perf_event__process_cgroup);
    c.tool.exit = Some(perf_event__process_exit);
    c.tool.fork = Some(perf_event__process_fork);
    c.tool.lost = Some(perf_event__process_lost);
    /* HAVE_LIBTRACEEVENT */
    c.tool.tracing_data = Some(perf_event__process_tracing_data);
    c.tool.build_id = Some(perf_event__process_build_id);
    c.tool.id_index = Some(perf_event__process_id_index);
    c.tool.auxtrace_info = Some(perf_event__process_auxtrace_info);
    c.tool.auxtrace = Some(perf_event__process_auxtrace);
    c.tool.event_update = Some(perf_event__process_event_update);
    c.tool.attr = Some(perf_event__process_attr);
    c.tool.feature = Some(perf_event__process_feature);
    c.tool.ordering_requires_timestamps = true;

    if (*opts).all {
        pr_err(c"--all is currently unsupported for JSON output.\n".as_ptr());
        return ret;
    }
    if (*opts).tod {
        pr_err(c"--tod is currently unsupported for JSON output.\n".as_ptr());
        return ret;
    }

    fd = open(
        output_name,
        O_CREAT | O_WRONLY | if (*opts).force { O_TRUNC } else { O_EXCL },
        0o666,
    );
    if fd == -1 {
        if errno == EEXIST {
            pr_err(c"Output file exists. Use --force to overwrite it.\n".as_ptr());
        } else {
            pr_err(c"Error opening output file!\n".as_ptr());
        }
        return ret;
    }

    c.out = fdopen(fd, c"w".as_ptr());
    if c.out.is_null() {
        fprintf(stderr, c"Error opening output file!\n".as_ptr());
        close(fd);
        return ret;
    }

    session = perf_session__new(&mut data, &mut c.tool);
    if IS_ERR(session as *const c_void) {
        fprintf(stderr, c"Error creating perf session!\n".as_ptr());
        goto_err_fclose(&mut c);
        return ret;
    }
    if symbol__init(perf_session__env(session)) < 0 {
        fprintf(stderr, c"Symbol init error!\n".as_ptr());
        goto_err_session_delete(session, &mut c);
        return ret;
    }

    if !(*opts).time_str.is_null() {
        ret = perf_time__parse_for_ranges(
            (*opts).time_str,
            session,
            &mut c.ptime_range,
            &mut c.range_size,
            &mut c.range_num,
        );
        if ret < 0 {
            goto_err_session_delete(session, &mut c);
            return ret;
        }
    }

    // The opening brace is printed manually because it isn't delimited from a
    // previous value (i.e. we don't want a leading newline)
    fputc(b'{' as c_int, c.out);

    // Version number for future-proofing. Most additions should be able to be
    // done in a backwards-compatible way so this should only need to be bumped
    // if some major breaking change must be made.
    output_json_format!(c.out, false, 1, c"\"linux-perf-json-version\": 1");

    // Output headers
    output_json_format!(c.out, true, 1, c"\"headers\": {");
    output_headers(session, &mut c);
    output_json_format!(c.out, false, 1, c"}");

    // Output samples
    output_json_format!(c.out, true, 1, c"\"samples\": [");
    perf_session__process_events(session);
    output_json_format!(c.out, false, 1, c"]");
    output_json_format!(c.out, false, 0, c"}");
    fputc(b'\n' as c_int, c.out);

    fprintf(
        stderr,
        c"[ perf data convert: Converted '%s' into JSON data '%s' ]\n".as_ptr(),
        data.path,
        output_name,
    );

    fprintf(
        stderr,
        c"[ perf data convert: Converted and wrote %.3f MB (%lu samples) ]\n".as_ptr(),
        (ftell(c.out) as f64) / 1024.0 / 1024.0,
        c.events_count,
    );

    if c.skipped != 0 {
        fprintf(
            stderr,
            c"[ perf data convert: Skipped %lu samples ]\n".as_ptr(),
            c.skipped,
        );
    }

    ret = 0;

    if !c.ptime_range.is_null() {
        zfree(&mut c.ptime_range);
    }

    perf_session__delete(session);
    fclose(c.out);
    ret
}

unsafe fn goto_err_session_delete(session: *mut perf_session, c: *mut convert_json) {
    perf_session__delete(session);
    goto_err_fclose(c);
}

unsafe fn goto_err_fclose(c: *mut convert_json) {
    fclose((*c).out);
}
