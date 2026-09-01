/*
 * trace-event-python.  Feed trace events to an embedded Python interpreter.
 *
 * Copyright (C) 2010 Tom Zanussi <tzanussi@gmail.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type s64 = i64;
type pid_t = i32;
type size_t = usize;
type wchar_t = i32;
type FILE = c_void;
type PyObject = c_void;
type PyMODINIT_FUNC = *mut PyObject;

const MAX_FIELDS: c_uint = 64;
const MAX_REG_SIZE: c_int = 128;

/* Constants, structs, unions, and helper macros supplied by the perf, Python,
 * Linux, and libtraceevent headers included by the original C file.
 */
const PATH_MAX: usize = 4096;
const SBUILD_ID_SIZE: usize = 64;
const SAMPLE_FLAGS_BUF_SIZE: usize = 64;
const MAX_AUXTRACE_ERROR_MSG: usize = 64;
const NSEC_PER_SEC: c_ulong = 1_000_000_000;
const LONG_MAX: u64 = c_long::MAX as u64;
const LONG_MIN: i64 = c_long::MIN as i64;
const BITS_PER_LONG: c_int = (size_of::<c_ulong>() * 8) as c_int;
const PERF_FORMAT_LOST: u64 = 1 << 4;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_TYPE_TRACEPOINT: u32 = 2;
const PERF_TYPE_SYNTH: u32 = 1 << 31;
const PERF_RECORD_THROTTLE: u32 = 5;
const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;
const PERF_RECORD_MISC_SWITCH_OUT: u16 = 1 << 13;
const PERF_RECORD_MISC_SWITCH_OUT_PREEMPT: u16 = 1 << 14;
const PERF_RECORD_MISC_CPUMODE_MASK: u16 = 7;
const PERF_BRANCH_MASK: u64 = 0xff;
const PERF_IP_FLAG_IN_TX: u64 = 1 << 3;
const EM_HOST: u16 = 0;
const EF_HOST: u32 = 0;

const TEP_FIELD_IS_ARRAY: c_uint = 1 << 0;
const TEP_FIELD_IS_DYNAMIC: c_uint = 1 << 1;
const TEP_FIELD_IS_STRING: c_uint = 1 << 2;
const TEP_FIELD_IS_SIGNED: c_uint = 1 << 3;
const TEP_FIELD_IS_FLAG: c_uint = 1 << 4;
const TEP_FIELD_IS_SYMBOLIC: c_uint = 1 << 5;

const TEP_PRINT_NULL: tep_print_arg_type = 0;
const TEP_PRINT_ATOM: tep_print_arg_type = 1;
const TEP_PRINT_FIELD: tep_print_arg_type = 2;
const TEP_PRINT_FLAGS: tep_print_arg_type = 3;
const TEP_PRINT_SYMBOL: tep_print_arg_type = 4;
const TEP_PRINT_HEX: tep_print_arg_type = 5;
const TEP_PRINT_HEX_STR: tep_print_arg_type = 6;
const TEP_PRINT_INT_ARRAY: tep_print_arg_type = 7;
const TEP_PRINT_STRING: tep_print_arg_type = 8;
const TEP_PRINT_TYPE: tep_print_arg_type = 9;
const TEP_PRINT_OP: tep_print_arg_type = 10;
const TEP_PRINT_BSTRING: tep_print_arg_type = 11;
const TEP_PRINT_DYNAMIC_ARRAY: tep_print_arg_type = 12;
const TEP_PRINT_DYNAMIC_ARRAY_LEN: tep_print_arg_type = 13;
const TEP_PRINT_FUNC: tep_print_arg_type = 14;
const TEP_PRINT_BITMASK: tep_print_arg_type = 15;
type tep_print_arg_type = c_uint;

const TRACE_EVENT_TYPE_MAX: usize = (1usize << (size_of::<c_ushort>() * 8)) - 1;
const N_COMMON_FIELDS: c_int = 7;
type c_ushort = u16;

#[repr(C)]
pub struct scripting_context {
    session: *mut perf_session,
    pevent: *mut tep_handle,
}
#[repr(C)] pub struct db_export { _private: [u8; 0] }
#[repr(C)] pub struct evsel { core: evsel_core, db_id: u64, counts: *mut c_void }
#[repr(C)] pub struct evsel_core { attr: perf_event_attr, threads: *mut perf_thread_map, cpus: *mut perf_cpu_map }
#[repr(C)] pub struct perf_event_attr { type_: u32, config: u64, read_format: u64, sample_regs_intr: u64, sample_regs_user: u64 }
#[repr(C)] pub struct perf_session { _private: [u8; 0] }
#[repr(C)] pub struct machine { db_id: u64, pid: s32, root_dir: *const c_char }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct comm { db_id: u64, start: u64, exec: s32 }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct symbol { start: u64, end: u64, binding: u8, namelen: u16, name: *const c_char }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct maps { _private: [u8; 0] }
#[repr(C)] pub struct perf_thread_map { _private: [u8; 0] }
#[repr(C)] pub struct perf_cpu_map { _private: [u8; 0] }
#[repr(C)] pub struct perf_cpu { cpu: c_int }
#[repr(C)] pub struct perf_counts_values { val: u64, ena: u64, run: u64 }
#[repr(C)] pub struct perf_stat_config { _private: [u8; 0] }
#[repr(C)] pub struct mem_info { _private: [u8; 0] }
#[repr(C)] pub struct data_src { val: u64 }
#[repr(C)] pub struct regs_dump { regs: *mut u64 }
#[repr(C)] pub struct callchain_cursor { _private: [u8; 0] }
#[repr(C)] pub struct callchain { _private: [u8; 0] }
#[repr(C)] pub struct callchain_cursor_node { ip: u64, ms: map_symbol, srcline: *const c_char }
#[repr(C)] pub struct map_symbol { map: *mut map, sym: *mut symbol }
#[repr(C)] pub struct addr_location { addr: u64, map: *mut map, sym: *mut symbol, thread: *mut thread }
#[repr(C)] pub struct sample_read_value { id: u64, value: u64, lost: u64 }
#[repr(C)] pub struct sample_read_group { nr: c_uint, values: *mut sample_read_value }
#[repr(C)] pub union sample_read {
    one: sample_read_value,
    group: sample_read_group,
    time_enabled: u64,
}
#[repr(C)] pub struct perf_sample {
    evsel: *mut evsel,
    id: u64,
    stream_id: u64,
    pid: s32,
    tid: s32,
    cpu: s32,
    ip: u64,
    time: u64,
    period: u64,
    phys_addr: u64,
    addr: u64,
    read: perf_sample_read,
    weight: u64,
    ins_lat: u32,
    transaction: u64,
    data_src: u64,
    raw_data: *mut c_void,
    raw_size: c_uint,
    callchain: *mut callchain,
    branch_stack: *mut branch_stack,
    cpumode: u8,
    machine_pid: s32,
    vcpu: s32,
    flags: u64,
    insn_cnt: u64,
    cyc_cnt: u64,
    intr_regs: *mut regs_dump,
    user_regs: *mut regs_dump,
}
#[repr(C)] pub struct perf_sample_read {
    time_enabled: u64,
    time_running: u64,
    one: sample_read_value,
    group: sample_read_group,
}
#[repr(C)] pub struct branch_stack { nr: u64 }
#[repr(C)] pub struct branch_entry { from: u64, to: u64, flags: branch_flags }
#[repr(C)] pub struct branch_flags { mispred: u64, predicted: u64, in_tx: u64, abort: u64, cycles: u64 }
#[repr(C)] pub struct call_path { db_id: u64, parent: *mut call_path, sym: *mut symbol, ip: u64 }
#[repr(C)] pub struct call_return {
    db_id: u64, thread: *mut thread, comm: *mut comm, cp: *mut call_path,
    call_time: u64, return_time: u64, branch_count: u64, call_ref: u64,
    return_ref: u64, flags: s32, parent_db_id: u64, insn_count: u64, cyc_count: u64,
}
#[repr(C)] pub struct export_sample {
    db_id: u64, sample: *mut perf_sample, al: *mut addr_location, comm_db_id: u64,
    dso_db_id: u64, sym_db_id: u64, offset: u64, addr_dso_db_id: u64,
    addr_sym_db_id: u64, addr_offset: u64, call_path_id: u64,
}
#[repr(C)] pub struct perf_event_header { type_: u32, misc: u16, size: u16 }
#[repr(C)] pub struct perf_record_throttle { time: u64, id: u64, stream_id: u64 }
#[repr(C)] pub struct perf_record_context_switch { next_prev_pid: pid_t, next_prev_tid: pid_t }
#[repr(C)] pub struct perf_record_auxtrace_error {
    header: perf_event_header, type_: u32, code: u32, cpu: s32, pid: s32, tid: s32,
    ip: u64, time: u64, msg: [c_char; MAX_AUXTRACE_ERROR_MSG], fmt: u32,
    machine_pid: s32, vcpu: s32,
}
#[repr(C)] pub union perf_event {
    header: perf_event_header,
    throttle: perf_record_throttle,
    context_switch: perf_record_context_switch,
    auxtrace_error: perf_record_auxtrace_error,
}
#[repr(C)] pub struct tep_handle { _private: [u8; 0] }
#[repr(C)] pub struct tep_event {
    id: c_int, system: *const c_char, name: *const c_char,
    format: tep_format, print_fmt: tep_print_fmt,
}
#[repr(C)] pub struct tep_format { fields: *mut tep_format_field }
#[repr(C)] pub struct tep_format_field {
    next: *mut tep_format_field, name: *const c_char, flags: c_uint,
    offset: c_uint, size: c_uint, arraylen: c_uint,
}
#[repr(C)] pub struct tep_print_fmt { args: *mut tep_print_arg }
#[repr(C)] pub struct tep_print_flag_sym { value: *const c_char, str_: *const c_char, next: *mut tep_print_flag_sym }
#[repr(C)] pub struct tep_print_arg { type_: tep_print_arg_type, next: *mut tep_print_arg, u: tep_print_arg_union }
#[repr(C)] pub union tep_print_arg_union {
    atom: tep_print_arg_atom,
    field: tep_print_arg_field,
    flags: tep_print_arg_flags,
    symbol: tep_print_arg_symbol,
    hex: tep_print_arg_hex,
    int_array: tep_print_arg_int_array,
    typecast: tep_print_arg_typecast,
    op: tep_print_arg_op,
}
#[repr(C)] pub struct tep_print_arg_atom { atom: *const c_char }
#[repr(C)] pub struct tep_print_arg_field { name: *const c_char }
#[repr(C)] pub struct tep_print_arg_flags { field: *mut tep_print_arg, delim: *const c_char, flags: *mut tep_print_flag_sym }
#[repr(C)] pub struct tep_print_arg_symbol { field: *mut tep_print_arg, symbols: *mut tep_print_flag_sym }
#[repr(C)] pub struct tep_print_arg_hex { field: *mut tep_print_arg, size: *mut tep_print_arg }
#[repr(C)] pub struct tep_print_arg_int_array { field: *mut tep_print_arg, count: *mut tep_print_arg, el_size: *mut tep_print_arg }
#[repr(C)] pub struct tep_print_arg_typecast { item: *mut tep_print_arg }
#[repr(C)] pub struct tep_print_arg_op { op: *const c_char, left: *mut tep_print_arg, right: *mut tep_print_arg }

#[repr(C)]
pub struct scripting_ops {
    name: *const c_char,
    dirname: *const c_char,
    start_script: Option<unsafe extern "C" fn(*const c_char, c_int, *mut *const c_char, *mut perf_session) -> c_int>,
    flush_script: Option<unsafe extern "C" fn() -> c_int>,
    stop_script: Option<unsafe extern "C" fn() -> c_int>,
    process_event: Option<unsafe extern "C" fn(*mut perf_event, *mut perf_sample, *mut addr_location, *mut addr_location)>,
    process_switch: Option<unsafe extern "C" fn(*mut perf_event, *mut perf_sample, *mut machine)>,
    process_auxtrace_error: Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event)>,
    process_stat: Option<unsafe extern "C" fn(*mut perf_stat_config, *mut evsel, u64)>,
    process_stat_interval: Option<unsafe extern "C" fn(u64)>,
    process_throttle: Option<unsafe extern "C" fn(*mut perf_event, *mut perf_sample, *mut machine)>,
    generate_script: Option<unsafe extern "C" fn(*mut tep_handle, *const c_char) -> c_int>,
}

#[repr(C)]
struct tables {
    dbe: db_export,
    evsel_handler: *mut PyObject,
    machine_handler: *mut PyObject,
    thread_handler: *mut PyObject,
    comm_handler: *mut PyObject,
    comm_thread_handler: *mut PyObject,
    dso_handler: *mut PyObject,
    symbol_handler: *mut PyObject,
    branch_type_handler: *mut PyObject,
    sample_handler: *mut PyObject,
    call_path_handler: *mut PyObject,
    call_return_handler: *mut PyObject,
    synth_handler: *mut PyObject,
    context_switch_handler: *mut PyObject,
    db_export_mode: bool,
}

extern "C" {
    fn PyInit_perf_trace_context() -> PyMODINIT_FUNC;
    static mut scripting_context: *mut scripting_context;
    static mut symbol_conf: symbol_conf_t;
    static mut scripting_max_stack: c_int;
    static PERF_IP_FLAG_CHARS: *const c_char;
    fn PyErr_Print();
    fn Py_FatalError(msg: *const c_char) -> !;
    fn abort() -> !;
    fn PyDict_SetItemString(dict: *mut PyObject, key: *const c_char, val: *mut PyObject) -> c_int;
    fn Py_DECREF(obj: *mut PyObject);
    fn Py_XDECREF(obj: *mut PyObject);
    fn Py_INCREF(obj: *mut PyObject);
    fn PyDict_GetItemString(dict: *mut PyObject, key: *const c_char) -> *mut PyObject;
    fn PyCallable_Check(obj: *mut PyObject) -> c_int;
    fn PyObject_CallObject(callable: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn PyObject_GetAttrString(obj: *mut PyObject, attr: *const c_char) -> *mut PyObject;
    fn PyErr_Clear();
    fn PyLong_AsLong(obj: *mut PyObject) -> c_long;
    fn PyTuple_New(size: isize) -> *mut PyObject;
    fn PyTuple_SetItem(tuple: *mut PyObject, pos: isize, item: *mut PyObject) -> c_int;
    fn _PyTuple_Resize(tuple: *mut *mut PyObject, newsize: isize) -> c_int;
    fn PyUnicode_FromString(s: *const c_char) -> *mut PyObject;
    fn PyUnicode_FromStringAndSize(s: *const c_char, size: isize) -> *mut PyObject;
    fn PyBytes_FromStringAndSize(s: *const c_char, size: isize) -> *mut PyObject;
    fn PyLong_FromLong(v: c_long) -> *mut PyObject;
    fn PyLong_FromLongLong(v: i64) -> *mut PyObject;
    fn PyLong_FromUnsignedLong(v: c_ulong) -> *mut PyObject;
    fn PyLong_FromUnsignedLongLong(v: u64) -> *mut PyObject;
    fn PyBool_FromLong(v: c_long) -> *mut PyObject;
    fn PyList_New(size: isize) -> *mut PyObject;
    fn PyList_SET_ITEM(list: *mut PyObject, pos: isize, item: *mut PyObject);
    fn PyList_Append(list: *mut PyObject, item: *mut PyObject) -> c_int;
    fn PyDict_New() -> *mut PyObject;
    fn PyByteArray_FromStringAndSize(s: *const c_char, len: isize) -> *mut PyObject;
    fn PyCapsule_New(pointer: *mut c_void, name: *const c_char, destructor: *mut c_void) -> *mut PyObject;
    fn PyImport_AddModule(name: *const c_char) -> *mut PyObject;
    fn PyModule_GetDict(module: *mut PyObject) -> *mut PyObject;
    fn PyImport_AppendInittab(name: *const c_char, initfunc: unsafe extern "C" fn() -> PyMODINIT_FUNC) -> c_int;
    fn Py_Initialize();
    fn PySys_SetArgv(argc: c_int, argv: *mut *mut wchar_t);
    fn PyRun_SimpleFile(fp: *mut FILE, filename: *const c_char) -> c_int;
    fn Py_Finalize();
    fn Py_DecodeLocale(arg: *const c_char, size: *mut size_t) -> *mut wchar_t;
    fn PyMem_RawFree(ptr: *mut c_void);
    fn PyObject_IsTrue(obj: *mut PyObject) -> c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    static mut stderr: *mut FILE;

    fn eval_flag(str_: *const c_char) -> u64;
    fn read_size(event: *mut tep_event, data: *mut c_void, size: c_uint) -> u64;
    fn tep_read_number(pevent: *mut tep_handle, data: *mut c_void, size: c_uint) -> u64;
    fn tep_field_is_relative(flags: c_uint) -> c_int;
    fn is_printable_array(data: *mut c_void, len: c_uint) -> c_int;
    fn evsel__tp_format(evsel: *mut evsel) -> *mut tep_event;
    fn tep_get_events_count(pevent: *mut tep_handle) -> c_int;
    fn tep_list_events(pevent: *mut tep_handle, sort_type: c_int) -> *mut *mut tep_event;

    fn map__dso(map: *mut map) -> *mut dso;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__pgoff(map: *mut map) -> u64;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__get(map: *mut map) -> *mut map;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__db_id(dso: *mut dso) -> u64;
    fn dso__bid(dso: *mut dso) -> *mut c_void;
    fn build_id__snprintf(bid: *mut c_void, bf: *mut c_char, size: size_t) -> c_int;
    fn symbol__binding(sym: *mut symbol) -> c_int;
    fn symbol__priv(sym: *mut symbol) -> *mut c_void;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__resolve_callchain(thread: *mut thread, cursor: *mut callchain_cursor, sample: *mut perf_sample, a: *mut c_void, b: *mut c_void, max_stack: c_int) -> c_int;
    fn thread__find_map_fb(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location);
    fn thread__find_symbol_fb(thread: *mut thread, cpumode: u8, addr: u64, al: *mut addr_location);
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut u32) -> u16;
    fn thread__db_id(thread: *mut thread) -> u64;
    fn thread__pid(thread: *mut thread) -> s32;
    fn thread__tid(thread: *mut thread) -> s32;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn get_tls_callchain_cursor() -> *mut callchain_cursor;
    fn callchain_cursor_commit(cursor: *mut callchain_cursor);
    fn callchain_cursor_current(cursor: *mut callchain_cursor) -> *mut callchain_cursor_node;
    fn callchain_cursor_advance(cursor: *mut callchain_cursor);
    fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut branch_entry;
    fn perf_sample__sprintf_flags(flags: u64, bf: *mut c_char, size: size_t) -> c_int;
    fn mem_info__new() -> *mut mem_info;
    fn mem_info__data_src(mi: *mut mem_info) -> *mut data_src;
    fn mem_info__put(mi: *mut mem_info);
    fn perf_script__meminfo_scnprintf(bf: *mut c_char, size: size_t, mi: *mut mem_info) -> c_int;
    fn perf_reg_name(r: c_uint, e_machine: u16, e_flags: u32) -> *const c_char;
    fn __sw_hweight64(w: u64) -> c_uint;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn scripting_context__update(ctx: *mut scripting_context, event: *mut perf_event, sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location);
    fn db_export__init(dbe: *mut db_export) -> c_int;
    fn db_export__exit(dbe: *mut db_export);
    fn db_export__sample(dbe: *mut db_export, event: *mut perf_event, sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location) -> c_int;
    fn db_export__switch(dbe: *mut db_export, event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) -> c_int;
    fn db_export__branch_types(dbe: *mut db_export) -> c_int;
    fn db_export__call_return(dbe: *mut db_export, cr: *mut call_return, parent_db_id: *mut u64) -> c_int;
    fn call_return_processor__new(cb: unsafe extern "C" fn(*mut call_return, *mut u64, *mut c_void) -> c_int, data: *mut c_void) -> *mut call_return_processor;
    fn call_path_root__new() -> *mut call_path_root;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, thread: c_int) -> pid_t;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_uint) -> perf_cpu;
    fn perf_counts(counts: *mut c_void, idx: c_uint, thread: c_int) -> *mut perf_counts_values;
}

#[repr(C)] pub struct call_return_processor { cpr: *mut call_path_root }
#[repr(C)] pub struct call_path_root { _private: [u8; 0] }
#[repr(C)] pub struct symbol_conf_t { show_kernel_path: bool, use_callchain: bool, priv_size: size_t }

static mut main_module: *mut PyObject = ptr::null_mut();
static mut main_dict: *mut PyObject = ptr::null_mut();
static mut tables_global: tables = tables {
    dbe: db_export { _private: [] },
    evsel_handler: ptr::null_mut(), machine_handler: ptr::null_mut(), thread_handler: ptr::null_mut(),
    comm_handler: ptr::null_mut(), comm_thread_handler: ptr::null_mut(), dso_handler: ptr::null_mut(),
    symbol_handler: ptr::null_mut(), branch_type_handler: ptr::null_mut(), sample_handler: ptr::null_mut(),
    call_path_handler: ptr::null_mut(), call_return_handler: ptr::null_mut(), synth_handler: ptr::null_mut(),
    context_switch_handler: ptr::null_mut(), db_export_mode: false,
};
static mut cur_field_name: *mut c_char = ptr::null_mut();
static mut zero_flag_atom: c_int = 0;

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char { bytes.as_ptr() as *const c_char }

unsafe extern "C" fn handler_call_die(_handler_name: *const c_char) -> ! {
    PyErr_Print();
    Py_FatalError(cstr(b"problem in Python trace event handler\0"));
}

unsafe fn pydict_set_item_string_decref(dict: *mut PyObject, key: *const c_char, val: *mut PyObject) {
    PyDict_SetItemString(dict, key, val);
    Py_DECREF(val);
}

unsafe fn get_handler(handler_name: *const c_char) -> *mut PyObject {
    let handler = PyDict_GetItemString(main_dict, handler_name);
    if !handler.is_null() && PyCallable_Check(handler) == 0 {
        return ptr::null_mut();
    }
    handler
}

unsafe fn call_object(handler: *mut PyObject, args: *mut PyObject, die_msg: *const c_char) {
    let retval = PyObject_CallObject(handler, args);
    if retval.is_null() {
        handler_call_die(die_msg);
    }
    Py_DECREF(retval);
}

unsafe fn try_call_object(handler_name: *const c_char, args: *mut PyObject) {
    let handler = get_handler(handler_name);
    if !handler.is_null() {
        call_object(handler, args, handler_name);
    }
}

unsafe fn get_argument_count(handler: *mut PyObject) -> c_int {
    let mut arg_count = 0;
    let code_obj = PyObject_GetAttrString(handler, cstr(b"__code__\0"));
    PyErr_Clear();
    if !code_obj.is_null() {
        let arg_count_obj = PyObject_GetAttrString(code_obj, cstr(b"co_argcount\0"));
        if !arg_count_obj.is_null() {
            arg_count = PyLong_AsLong(arg_count_obj) as c_int;
            Py_DECREF(arg_count_obj);
        }
        Py_DECREF(code_obj);
    }
    arg_count
}

unsafe fn define_value(field_type: tep_print_arg_type, ev_name: *const c_char, field_name: *const c_char, field_value: *const c_char, field_str: *const c_char) {
    let mut handler_name = cstr(b"define_flag_value\0");
    let mut n: isize = 0;
    if field_type == TEP_PRINT_SYMBOL {
        handler_name = cstr(b"define_symbolic_value\0");
    }
    let t = PyTuple_New(4);
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    let value = eval_flag(field_value);
    PyTuple_SetItem(t, n, PyUnicode_FromString(ev_name)); n += 1;
    PyTuple_SetItem(t, n, PyUnicode_FromString(field_name)); n += 1;
    PyTuple_SetItem(t, n, PyLong_FromLong(value as c_long)); n += 1;
    PyTuple_SetItem(t, n, PyUnicode_FromString(field_str)); n += 1;
    let _ = n;
    try_call_object(handler_name, t);
    Py_DECREF(t);
}

unsafe fn define_values(field_type: tep_print_arg_type, field: *mut tep_print_flag_sym, ev_name: *const c_char, field_name: *const c_char) {
    define_value(field_type, ev_name, field_name, (*field).value, (*field).str_);
    if !(*field).next.is_null() {
        define_values(field_type, (*field).next, ev_name, field_name);
    }
}

unsafe fn define_field(field_type: tep_print_arg_type, ev_name: *const c_char, field_name: *const c_char, delim: *const c_char) {
    let mut handler_name = cstr(b"define_flag_field\0");
    let mut n: isize = 0;
    if field_type == TEP_PRINT_SYMBOL {
        handler_name = cstr(b"define_symbolic_field\0");
    }
    let t = if field_type == TEP_PRINT_FLAGS { PyTuple_New(3) } else { PyTuple_New(2) };
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    PyTuple_SetItem(t, n, PyUnicode_FromString(ev_name)); n += 1;
    PyTuple_SetItem(t, n, PyUnicode_FromString(field_name)); n += 1;
    if field_type == TEP_PRINT_FLAGS {
        PyTuple_SetItem(t, n, PyUnicode_FromString(delim)); n += 1;
    }
    let _ = n;
    try_call_object(handler_name, t);
    Py_DECREF(t);
}

unsafe fn define_event_symbols(event: *mut tep_event, ev_name: *const c_char, args: *mut tep_print_arg) {
    if args.is_null() { return; }
    match (*args).type_ {
        TEP_PRINT_NULL => {}
        TEP_PRINT_ATOM => {
            define_value(TEP_PRINT_FLAGS, ev_name, cur_field_name, cstr(b"0\0"), (*args).u.atom.atom);
            zero_flag_atom = 0;
        }
        TEP_PRINT_FIELD => {
            free(cur_field_name as *mut c_void);
            cur_field_name = strdup((*args).u.field.name);
        }
        TEP_PRINT_FLAGS => {
            define_event_symbols(event, ev_name, (*args).u.flags.field);
            define_field(TEP_PRINT_FLAGS, ev_name, cur_field_name, (*args).u.flags.delim);
            define_values(TEP_PRINT_FLAGS, (*args).u.flags.flags, ev_name, cur_field_name);
        }
        TEP_PRINT_SYMBOL => {
            define_event_symbols(event, ev_name, (*args).u.symbol.field);
            define_field(TEP_PRINT_SYMBOL, ev_name, cur_field_name, ptr::null());
            define_values(TEP_PRINT_SYMBOL, (*args).u.symbol.symbols, ev_name, cur_field_name);
        }
        TEP_PRINT_HEX | TEP_PRINT_HEX_STR => {
            define_event_symbols(event, ev_name, (*args).u.hex.field);
            define_event_symbols(event, ev_name, (*args).u.hex.size);
        }
        TEP_PRINT_INT_ARRAY => {
            define_event_symbols(event, ev_name, (*args).u.int_array.field);
            define_event_symbols(event, ev_name, (*args).u.int_array.count);
            define_event_symbols(event, ev_name, (*args).u.int_array.el_size);
        }
        TEP_PRINT_STRING => {}
        TEP_PRINT_TYPE => define_event_symbols(event, ev_name, (*args).u.typecast.item),
        TEP_PRINT_OP => {
            if strcmp((*args).u.op.op, cstr(b":\0")) == 0 { zero_flag_atom = 1; }
            define_event_symbols(event, ev_name, (*args).u.op.left);
            define_event_symbols(event, ev_name, (*args).u.op.right);
        }
        _ => return,
    }
    if !(*args).next.is_null() {
        define_event_symbols(event, ev_name, (*args).next);
    }
}

unsafe fn get_field_numeric_entry(event: *mut tep_event, field: *mut tep_format_field, data: *mut c_void) -> *mut PyObject {
    let is_array = ((*field).flags & TEP_FIELD_IS_ARRAY) != 0;
    let mut obj: *mut PyObject = ptr::null_mut();
    let mut list: *mut PyObject = ptr::null_mut();
    let (item_size, n_items);
    if is_array {
        list = PyList_New((*field).arraylen as isize);
        if list.is_null() { Py_FatalError(cstr(b"couldn't create Python list\0")); }
        item_size = (*field).size / (*field).arraylen;
        n_items = (*field).arraylen;
    } else {
        item_size = (*field).size;
        n_items = 1;
    }
    for i in 0..n_items {
        let val = read_size(event, (data as *mut u8).add(((*field).offset + i * item_size) as usize) as *mut c_void, item_size);
        if ((*field).flags & TEP_FIELD_IS_SIGNED) != 0 {
            if (val as i64) >= LONG_MIN && (val as i64) <= c_long::MAX as i64 {
                obj = PyLong_FromLong(val as c_long);
            } else {
                obj = PyLong_FromLongLong(val as i64);
            }
        } else if val <= LONG_MAX {
            obj = PyLong_FromLong(val as c_long);
        } else {
            obj = PyLong_FromUnsignedLongLong(val);
        }
        if is_array {
            PyList_SET_ITEM(list, i as isize, obj);
        }
    }
    if is_array { obj = list; }
    obj
}

unsafe fn get_dsoname(map_: *mut map) -> *const c_char {
    let mut dsoname = cstr(b"[unknown]\0");
    let dso = if !map_.is_null() { map__dso(map_) } else { ptr::null_mut() };
    if !dso.is_null() {
        if symbol_conf.show_kernel_path && !dso__long_name(dso).is_null() {
            dsoname = dso__long_name(dso);
        } else {
            dsoname = dso__name(dso);
        }
    }
    dsoname
}

unsafe fn get_offset(sym: *mut symbol, al: *mut addr_location) -> c_ulong {
    if (*al).addr < (*sym).end {
        ((*al).addr - (*sym).start) as c_ulong
    } else {
        ((*al).addr - map__start((*al).map) - (*sym).start) as c_ulong
    }
}

unsafe fn python_process_callchain(sample: *mut perf_sample, al: *mut addr_location) -> *mut PyObject {
    let pylist = PyList_New(0);
    if pylist.is_null() { Py_FatalError(cstr(b"couldn't create Python list\0")); }
    if !symbol_conf.use_callchain || (*sample).callchain.is_null() {
        return pylist;
    }
    let cursor = get_tls_callchain_cursor();
    if thread__resolve_callchain((*al).thread, cursor, sample, ptr::null_mut(), ptr::null_mut(), scripting_max_stack) != 0 {
        pr_err(cstr(b"Failed to resolve callchain. Skipping\n\0"));
        return pylist;
    }
    callchain_cursor_commit(cursor);
    loop {
        let node = callchain_cursor_current(cursor);
        if node.is_null() { break; }
        let pyelem = PyDict_New();
        if pyelem.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
        pydict_set_item_string_decref(pyelem, cstr(b"ip\0"), PyLong_FromUnsignedLongLong((*node).ip));
        if !(*node).ms.sym.is_null() {
            let pysym = PyDict_New();
            if pysym.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
            pydict_set_item_string_decref(pysym, cstr(b"start\0"), PyLong_FromUnsignedLongLong((*(*node).ms.sym).start));
            pydict_set_item_string_decref(pysym, cstr(b"end\0"), PyLong_FromUnsignedLongLong((*(*node).ms.sym).end));
            pydict_set_item_string_decref(pysym, cstr(b"binding\0"), PyLong_FromLong(symbol__binding((*node).ms.sym) as c_long));
            pydict_set_item_string_decref(pysym, cstr(b"name\0"), PyUnicode_FromStringAndSize((*(*node).ms.sym).name, (*(*node).ms.sym).namelen as isize));
            pydict_set_item_string_decref(pyelem, cstr(b"sym\0"), pysym);
            if !(*node).ms.map.is_null() {
                let mut node_al: addr_location = core::mem::zeroed();
                addr_location__init(&mut node_al);
                node_al.addr = map__map_ip((*node).ms.map, (*node).ip);
                node_al.map = map__get((*node).ms.map);
                let offset = get_offset((*node).ms.sym, &mut node_al);
                addr_location__exit(&mut node_al);
                pydict_set_item_string_decref(pyelem, cstr(b"sym_off\0"), PyLong_FromUnsignedLongLong(offset as u64));
            }
            if !(*node).srcline.is_null() && strcmp(cstr(b":0\0"), (*node).srcline) != 0 {
                pydict_set_item_string_decref(pyelem, cstr(b"sym_srcline\0"), PyUnicode_FromString((*node).srcline));
            }
        }
        if !(*node).ms.map.is_null() {
            pydict_set_item_string_decref(pyelem, cstr(b"dso\0"), PyUnicode_FromString(get_dsoname((*node).ms.map)));
        }
        callchain_cursor_advance(cursor);
        PyList_Append(pylist, pyelem);
        Py_DECREF(pyelem);
    }
    pylist
}

unsafe fn python_process_brstack(sample: *mut perf_sample, thread: *mut thread) -> *mut PyObject {
    let br = (*sample).branch_stack;
    let entries = perf_sample__branch_entries(sample);
    let pylist = PyList_New(0);
    if pylist.is_null() { Py_FatalError(cstr(b"couldn't create Python list\0")); }
    if br.is_null() || (*br).nr == 0 { return pylist; }
    for i in 0..(*br).nr {
        let e = entries.add(i as usize);
        let pyelem = PyDict_New();
        if pyelem.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
        pydict_set_item_string_decref(pyelem, cstr(b"from\0"), PyLong_FromUnsignedLongLong((*e).from));
        pydict_set_item_string_decref(pyelem, cstr(b"to\0"), PyLong_FromUnsignedLongLong((*e).to));
        pydict_set_item_string_decref(pyelem, cstr(b"mispred\0"), PyBool_FromLong((*e).flags.mispred as c_long));
        pydict_set_item_string_decref(pyelem, cstr(b"predicted\0"), PyBool_FromLong((*e).flags.predicted as c_long));
        pydict_set_item_string_decref(pyelem, cstr(b"in_tx\0"), PyBool_FromLong((*e).flags.in_tx as c_long));
        pydict_set_item_string_decref(pyelem, cstr(b"abort\0"), PyBool_FromLong((*e).flags.abort as c_long));
        pydict_set_item_string_decref(pyelem, cstr(b"cycles\0"), PyLong_FromUnsignedLongLong((*e).flags.cycles));
        let mut al: addr_location = core::mem::zeroed();
        addr_location__init(&mut al);
        thread__find_map_fb(thread, (*sample).cpumode, (*e).from, &mut al);
        pydict_set_item_string_decref(pyelem, cstr(b"from_dsoname\0"), PyUnicode_FromString(get_dsoname(al.map)));
        thread__find_map_fb(thread, (*sample).cpumode, (*e).to, &mut al);
        pydict_set_item_string_decref(pyelem, cstr(b"to_dsoname\0"), PyUnicode_FromString(get_dsoname(al.map)));
        addr_location__exit(&mut al);
        PyList_Append(pylist, pyelem);
        Py_DECREF(pyelem);
    }
    pylist
}

unsafe fn get_symoff(sym: *mut symbol, al: *mut addr_location, print_off: bool, bf: *mut c_char, size: c_int) -> c_int {
    if sym.is_null() || *(*sym).name == 0 {
        return scnprintf(bf, size as size_t, cstr(b"%s\0"), cstr(b"[unknown]\0"));
    }
    if !print_off {
        return scnprintf(bf, size as size_t, cstr(b"%s\0"), (*sym).name);
    }
    let offset = get_offset(sym, al);
    scnprintf(bf, size as size_t, cstr(b"%s+0x%x\0"), (*sym).name, offset)
}

unsafe fn get_br_mspred(flags: *mut branch_flags, bf: *mut c_char, size: c_int) -> c_int {
    if (*flags).mispred == 0 && (*flags).predicted == 0 {
        return scnprintf(bf, size as size_t, cstr(b"%s\0"), cstr(b"-\0"));
    }
    if (*flags).mispred != 0 {
        return scnprintf(bf, size as size_t, cstr(b"%s\0"), cstr(b"M\0"));
    }
    scnprintf(bf, size as size_t, cstr(b"%s\0"), cstr(b"P\0"))
}

unsafe fn python_process_brstacksym(sample: *mut perf_sample, thread: *mut thread) -> *mut PyObject {
    let br = (*sample).branch_stack;
    let entries = perf_sample__branch_entries(sample);
    let pylist = PyList_New(0);
    let mut bf = [0 as c_char; 512];
    if pylist.is_null() { Py_FatalError(cstr(b"couldn't create Python list\0")); }
    if br.is_null() || (*br).nr == 0 { return pylist; }
    for i in 0..(*br).nr {
        let e = entries.add(i as usize);
        let mut al: addr_location = core::mem::zeroed();
        addr_location__init(&mut al);
        let pyelem = PyDict_New();
        if pyelem.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
        thread__find_symbol_fb(thread, (*sample).cpumode, (*e).from, &mut al);
        get_symoff(al.sym, &mut al, true, bf.as_mut_ptr(), bf.len() as c_int);
        pydict_set_item_string_decref(pyelem, cstr(b"from\0"), PyUnicode_FromString(bf.as_ptr()));
        thread__find_symbol_fb(thread, (*sample).cpumode, (*e).to, &mut al);
        get_symoff(al.sym, &mut al, true, bf.as_mut_ptr(), bf.len() as c_int);
        pydict_set_item_string_decref(pyelem, cstr(b"to\0"), PyUnicode_FromString(bf.as_ptr()));
        get_br_mspred(&mut (*e).flags, bf.as_mut_ptr(), bf.len() as c_int);
        pydict_set_item_string_decref(pyelem, cstr(b"pred\0"), PyUnicode_FromString(bf.as_ptr()));
        pydict_set_item_string_decref(pyelem, cstr(b"in_tx\0"), PyUnicode_FromString(if (*e).flags.in_tx != 0 { cstr(b"X\0") } else { cstr(b"-\0") }));
        pydict_set_item_string_decref(pyelem, cstr(b"abort\0"), PyUnicode_FromString(if (*e).flags.abort != 0 { cstr(b"A\0") } else { cstr(b"-\0") }));
        PyList_Append(pylist, pyelem);
        Py_DECREF(pyelem);
        addr_location__exit(&mut al);
    }
    pylist
}

unsafe fn get_sample_value_as_tuple(value: *mut sample_read_value, read_format: u64) -> *mut PyObject {
    let t = PyTuple_New(3);
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    PyTuple_SetItem(t, 0, PyLong_FromUnsignedLongLong((*value).id));
    PyTuple_SetItem(t, 1, PyLong_FromUnsignedLongLong((*value).value));
    if (read_format & PERF_FORMAT_LOST) != 0 {
        PyTuple_SetItem(t, 2, PyLong_FromUnsignedLongLong((*value).lost));
    }
    t
}

unsafe fn set_sample_read_in_dict(dict_sample: *mut PyObject, sample: *mut perf_sample) {
    let read_format = (*(*sample).evsel).core.attr.read_format;
    if (read_format & PERF_FORMAT_TOTAL_TIME_ENABLED) != 0 {
        pydict_set_item_string_decref(dict_sample, cstr(b"time_enabled\0"), PyLong_FromUnsignedLongLong((*sample).read.time_enabled));
    }
    if (read_format & PERF_FORMAT_TOTAL_TIME_RUNNING) != 0 {
        pydict_set_item_string_decref(dict_sample, cstr(b"time_running\0"), PyLong_FromUnsignedLongLong((*sample).read.time_running));
    }
    let values = if (read_format & PERF_FORMAT_GROUP) != 0 { PyList_New((*sample).read.group.nr as isize) } else { PyList_New(1) };
    if values.is_null() { Py_FatalError(cstr(b"couldn't create Python list\0")); }
    if (read_format & PERF_FORMAT_GROUP) != 0 {
        let mut v = (*sample).read.group.values;
        for i in 0..(*sample).read.group.nr {
            let t = get_sample_value_as_tuple(v, read_format);
            PyList_SET_ITEM(values, i as isize, t);
            v = v.add(1);
        }
    } else {
        let t = get_sample_value_as_tuple(&mut (*sample).read.one, read_format);
        PyList_SET_ITEM(values, 0, t);
    }
    pydict_set_item_string_decref(dict_sample, cstr(b"values\0"), values);
}

unsafe fn set_sample_datasrc_in_dict(dict: *mut PyObject, sample: *mut perf_sample) {
    let mi = mem_info__new();
    let mut decode = [0 as c_char; 100];
    if mi.is_null() { Py_FatalError(cstr(b"couldn't create mem-info\0")); }
    pydict_set_item_string_decref(dict, cstr(b"datasrc\0"), PyLong_FromUnsignedLongLong((*sample).data_src));
    (*mem_info__data_src(mi)).val = (*sample).data_src;
    perf_script__meminfo_scnprintf(decode.as_mut_ptr(), 100, mi);
    mem_info__put(mi);
    pydict_set_item_string_decref(dict, cstr(b"datasrc_decode\0"), PyUnicode_FromString(decode.as_ptr()));
}

unsafe fn regs_map(regs: *mut regs_dump, mask: u64, e_machine: u16, e_flags: u32, bf: *mut c_char, size: c_int) {
    let mut i = 0usize;
    let mut printed = 0;
    *bf = 0;
    if size <= 0 || regs.is_null() || (*regs).regs.is_null() { return; }
    for r in 0..64u32 {
        if (mask & (1u64 << r)) != 0 {
            let val = *(*regs).regs.add(i);
            i += 1;
            printed += scnprintf(bf.add(printed as usize), (size - printed) as size_t, cstr(b"%5s:0x%llx \0"), perf_reg_name(r, e_machine, e_flags), val);
        }
    }
}

unsafe fn set_regs_in_dict(dict: *mut PyObject, sample: *mut perf_sample, e_machine: u16, e_flags: u32) -> c_int {
    let attr = &mut (*(*sample).evsel).core.attr as *mut perf_event_attr;
    let size = (__sw_hweight64((*attr).sample_regs_intr) as c_int * MAX_REG_SIZE) + 1;
    let mut bf: *mut c_char = ptr::null_mut();
    if !(*sample).intr_regs.is_null() {
        bf = malloc(size as size_t) as *mut c_char;
        if bf.is_null() { return -1; }
        regs_map((*sample).intr_regs, (*attr).sample_regs_intr, e_machine, e_flags, bf, size);
        pydict_set_item_string_decref(dict, cstr(b"iregs\0"), PyUnicode_FromString(bf));
    }
    if !(*sample).user_regs.is_null() {
        if bf.is_null() {
            bf = malloc(size as size_t) as *mut c_char;
            if bf.is_null() { return -1; }
        }
        regs_map((*sample).user_regs, (*attr).sample_regs_user, e_machine, e_flags, bf, size);
        pydict_set_item_string_decref(dict, cstr(b"uregs\0"), PyUnicode_FromString(bf));
    }
    free(bf as *mut c_void);
    0
}

unsafe fn set_sym_in_dict(dict: *mut PyObject, al: *mut addr_location, dso_field: *const c_char, dso_bid_field: *const c_char, dso_map_start: *const c_char, dso_map_end: *const c_char, sym_field: *const c_char, symoff_field: *const c_char, map_pgoff: *const c_char) {
    if !(*al).map.is_null() {
        let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
        let dso = map__dso((*al).map);
        pydict_set_item_string_decref(dict, dso_field, PyUnicode_FromString(dso__name(dso)));
        build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
        pydict_set_item_string_decref(dict, dso_bid_field, PyUnicode_FromString(sbuild_id.as_ptr()));
        pydict_set_item_string_decref(dict, dso_map_start, PyLong_FromUnsignedLong(map__start((*al).map) as c_ulong));
        pydict_set_item_string_decref(dict, dso_map_end, PyLong_FromUnsignedLong(map__end((*al).map) as c_ulong));
        pydict_set_item_string_decref(dict, map_pgoff, PyLong_FromUnsignedLongLong(map__pgoff((*al).map)));
    }
    if !(*al).sym.is_null() {
        pydict_set_item_string_decref(dict, sym_field, PyUnicode_FromString((*(*al).sym).name));
        pydict_set_item_string_decref(dict, symoff_field, PyLong_FromUnsignedLong(get_offset((*al).sym, al)));
    }
}

unsafe fn set_sample_flags(dict: *mut PyObject, mut flags: u32) {
    let mut ch = PERF_IP_FLAG_CHARS;
    let mut str_ = [0 as c_char; 33];
    let mut p = str_.as_mut_ptr();
    while *ch != 0 {
        if (flags & 1) != 0 {
            *p = *ch;
            p = p.add(1);
        }
        ch = ch.add(1);
        flags >>= 1;
    }
    *p = 0;
    pydict_set_item_string_decref(dict, cstr(b"flags\0"), PyUnicode_FromString(str_.as_ptr()));
}

unsafe fn python_process_sample_flags(sample: *mut perf_sample, dict_sample: *mut PyObject) {
    let mut flags_disp = [0 as c_char; SAMPLE_FLAGS_BUF_SIZE];
    set_sample_flags(dict_sample, (*sample).flags as u32);
    perf_sample__sprintf_flags((*sample).flags, flags_disp.as_mut_ptr(), flags_disp.len());
    pydict_set_item_string_decref(dict_sample, cstr(b"flags_disp\0"), PyUnicode_FromString(flags_disp.as_ptr()));
}

unsafe fn get_perf_sample_dict(sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location, callchain: *mut PyObject) -> *mut PyObject {
    let mut e_machine = EM_HOST;
    let mut e_flags = EF_HOST;
    let evsel = (*sample).evsel;
    let dict = PyDict_New();
    if dict.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
    let dict_sample = PyDict_New();
    if dict_sample.is_null() { Py_FatalError(cstr(b"couldn't create Python dictionary\0")); }
    pydict_set_item_string_decref(dict, cstr(b"ev_name\0"), PyUnicode_FromString(evsel__name(evsel)));
    pydict_set_item_string_decref(dict, cstr(b"attr\0"), PyBytes_FromStringAndSize(&(*evsel).core.attr as *const _ as *const c_char, size_of::<perf_event_attr>() as isize));
    pydict_set_item_string_decref(dict_sample, cstr(b"id\0"), PyLong_FromUnsignedLongLong((*sample).id));
    pydict_set_item_string_decref(dict_sample, cstr(b"stream_id\0"), PyLong_FromUnsignedLongLong((*sample).stream_id));
    pydict_set_item_string_decref(dict_sample, cstr(b"pid\0"), PyLong_FromLong((*sample).pid as c_long));
    pydict_set_item_string_decref(dict_sample, cstr(b"tid\0"), PyLong_FromLong((*sample).tid as c_long));
    pydict_set_item_string_decref(dict_sample, cstr(b"cpu\0"), PyLong_FromLong((*sample).cpu as c_long));
    pydict_set_item_string_decref(dict_sample, cstr(b"ip\0"), PyLong_FromUnsignedLongLong((*sample).ip));
    pydict_set_item_string_decref(dict_sample, cstr(b"time\0"), PyLong_FromUnsignedLongLong((*sample).time));
    pydict_set_item_string_decref(dict_sample, cstr(b"period\0"), PyLong_FromUnsignedLongLong((*sample).period));
    pydict_set_item_string_decref(dict_sample, cstr(b"phys_addr\0"), PyLong_FromUnsignedLongLong((*sample).phys_addr));
    pydict_set_item_string_decref(dict_sample, cstr(b"addr\0"), PyLong_FromUnsignedLongLong((*sample).addr));
    set_sample_read_in_dict(dict_sample, sample);
    pydict_set_item_string_decref(dict_sample, cstr(b"weight\0"), PyLong_FromUnsignedLongLong((*sample).weight));
    pydict_set_item_string_decref(dict_sample, cstr(b"ins_lat\0"), PyLong_FromUnsignedLong((*sample).ins_lat as c_ulong));
    pydict_set_item_string_decref(dict_sample, cstr(b"transaction\0"), PyLong_FromUnsignedLongLong((*sample).transaction));
    set_sample_datasrc_in_dict(dict_sample, sample);
    pydict_set_item_string_decref(dict, cstr(b"sample\0"), dict_sample);
    pydict_set_item_string_decref(dict, cstr(b"raw_buf\0"), PyBytes_FromStringAndSize((*sample).raw_data as *const c_char, (*sample).raw_size as isize));
    pydict_set_item_string_decref(dict, cstr(b"comm\0"), PyUnicode_FromString(thread__comm_str((*al).thread)));
    set_sym_in_dict(dict, al, cstr(b"dso\0"), cstr(b"dso_bid\0"), cstr(b"dso_map_start\0"), cstr(b"dso_map_end\0"), cstr(b"symbol\0"), cstr(b"symoff\0"), cstr(b"map_pgoff\0"));
    pydict_set_item_string_decref(dict, cstr(b"callchain\0"), callchain);
    pydict_set_item_string_decref(dict, cstr(b"brstack\0"), python_process_brstack(sample, (*al).thread));
    pydict_set_item_string_decref(dict, cstr(b"brstacksym\0"), python_process_brstacksym(sample, (*al).thread));
    if (*sample).machine_pid != 0 {
        pydict_set_item_string_decref(dict_sample, cstr(b"machine_pid\0"), PyLong_FromLong((*sample).machine_pid as c_long));
        pydict_set_item_string_decref(dict_sample, cstr(b"vcpu\0"), PyLong_FromLong((*sample).vcpu as c_long));
    }
    pydict_set_item_string_decref(dict_sample, cstr(b"cpumode\0"), PyLong_FromLong((*sample).cpumode as c_long));
    if !addr_al.is_null() {
        pydict_set_item_string_decref(dict_sample, cstr(b"addr_correlates_sym\0"), PyBool_FromLong(1));
        set_sym_in_dict(dict_sample, addr_al, cstr(b"addr_dso\0"), cstr(b"addr_dso_bid\0"), cstr(b"addr_dso_map_start\0"), cstr(b"addr_dso_map_end\0"), cstr(b"addr_symbol\0"), cstr(b"addr_symoff\0"), cstr(b"addr_map_pgoff\0"));
    }
    if (*sample).flags != 0 { python_process_sample_flags(sample, dict_sample); }
    /* Instructions per cycle (IPC) */
    if (*sample).insn_cnt != 0 && (*sample).cyc_cnt != 0 {
        pydict_set_item_string_decref(dict_sample, cstr(b"insn_cnt\0"), PyLong_FromUnsignedLongLong((*sample).insn_cnt));
        pydict_set_item_string_decref(dict_sample, cstr(b"cyc_cnt\0"), PyLong_FromUnsignedLongLong((*sample).cyc_cnt));
    }
    if !(*al).thread.is_null() {
        e_machine = thread__e_machine((*al).thread, ptr::null_mut(), &mut e_flags);
    }
    if set_regs_in_dict(dict, sample, e_machine, e_flags) != 0 {
        Py_FatalError(cstr(b"Failed to setting regs in dict\0"));
    }
    dict
}

unsafe fn python_process_tracepoint(sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location) {
    let mut handler_name = [0 as c_char; 256];
    let mut n: isize = 0;
    let evsel = (*sample).evsel;
    let event = evsel__tp_format(evsel);
    if event.is_null() {
        snprintf(handler_name.as_mut_ptr(), handler_name.len(), cstr(b"ug! no event found for type %llu\0"), (*evsel).core.attr.config);
        Py_FatalError(handler_name.as_ptr());
    }
    let data = (*sample).raw_data;
    let pid = raw_field_value(event, cstr(b"common_pid\0"), data);
    sprintf(handler_name.as_mut_ptr(), cstr(b"%s__%s\0"), (*event).system, (*event).name);
    define_event_symbols(event, handler_name.as_ptr(), (*event).print_fmt.args);
    let default_handler_name = cstr(b"trace_unhandled\0");
    let mut handler = get_handler(handler_name.as_ptr());
    let mut dict: *mut PyObject = ptr::null_mut();
    if handler.is_null() {
        handler = get_handler(default_handler_name);
        if handler.is_null() { return; }
        dict = PyDict_New();
        if dict.is_null() { Py_FatalError(cstr(b"couldn't create Python dict\0")); }
    }
    let mut t = PyTuple_New(MAX_FIELDS as isize);
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    let s = (*sample).time / NSEC_PER_SEC as u64;
    let ns = (*sample).time - s * NSEC_PER_SEC as u64;
    let context = PyCapsule_New(scripting_context as *mut c_void, ptr::null(), ptr::null_mut());
    PyTuple_SetItem(t, n, PyUnicode_FromString(handler_name.as_ptr())); n += 1;
    PyTuple_SetItem(t, n, context); n += 1;
    let callchain = python_process_callchain(sample, al);
    Py_INCREF(callchain);
    if dict.is_null() {
        PyTuple_SetItem(t, n, PyLong_FromLong((*sample).cpu as c_long)); n += 1;
        PyTuple_SetItem(t, n, PyLong_FromLong(s as c_long)); n += 1;
        PyTuple_SetItem(t, n, PyLong_FromLong(ns as c_long)); n += 1;
        PyTuple_SetItem(t, n, PyLong_FromLong(pid as c_long)); n += 1;
        PyTuple_SetItem(t, n, PyUnicode_FromString(thread__comm_str((*al).thread))); n += 1;
        PyTuple_SetItem(t, n, callchain); n += 1;
    } else {
        pydict_set_item_string_decref(dict, cstr(b"common_cpu\0"), PyLong_FromLong((*sample).cpu as c_long));
        pydict_set_item_string_decref(dict, cstr(b"common_s\0"), PyLong_FromLong(s as c_long));
        pydict_set_item_string_decref(dict, cstr(b"common_ns\0"), PyLong_FromLong(ns as c_long));
        pydict_set_item_string_decref(dict, cstr(b"common_pid\0"), PyLong_FromLong(pid as c_long));
        pydict_set_item_string_decref(dict, cstr(b"common_comm\0"), PyUnicode_FromString(thread__comm_str((*al).thread)));
        pydict_set_item_string_decref(dict, cstr(b"common_callchain\0"), callchain);
    }
    let mut field = (*event).format.fields;
    while !field.is_null() {
        let obj;
        if ((*field).flags & TEP_FIELD_IS_ARRAY) != 0 {
            let mut offset = (*field).offset;
            let mut len = (*field).size;
            if ((*field).flags & TEP_FIELD_IS_DYNAMIC) != 0 {
                let val = tep_read_number((*scripting_context).pevent, (data as *mut u8).add(offset as usize) as *mut c_void, len);
                offset = val as c_uint;
                len = offset >> 16;
                offset &= 0xffff;
                if tep_field_is_relative((*field).flags) != 0 {
                    offset += (*field).offset + (*field).size;
                }
            }
            if ((*field).flags & TEP_FIELD_IS_STRING) != 0 && is_printable_array((data as *mut u8).add(offset as usize) as *mut c_void, len) != 0 {
                obj = PyUnicode_FromString((data as *mut u8).add(offset as usize) as *const c_char);
            } else {
                obj = PyByteArray_FromStringAndSize((data as *mut u8).add(offset as usize) as *const c_char, len as isize);
                (*field).flags &= !TEP_FIELD_IS_STRING;
            }
        } else {
            obj = get_field_numeric_entry(event, field, data);
        }
        if dict.is_null() { PyTuple_SetItem(t, n, obj); n += 1; } else { pydict_set_item_string_decref(dict, (*field).name, obj); }
        field = (*field).next;
    }
    if !dict.is_null() { PyTuple_SetItem(t, n, dict); n += 1; }
    if get_argument_count(handler) == n as c_int + 1 {
        let all_entries_dict = get_perf_sample_dict(sample, al, addr_al, callchain);
        PyTuple_SetItem(t, n, all_entries_dict); n += 1;
    } else {
        Py_DECREF(callchain);
    }
    if _PyTuple_Resize(&mut t, n) == -1 { Py_FatalError(cstr(b"error resizing Python tuple\0")); }
    if dict.is_null() { call_object(handler, t, handler_name.as_ptr()); } else { call_object(handler, t, default_handler_name); }
    Py_DECREF(t);
}

extern "C" { fn raw_field_value(event: *mut tep_event, name: *const c_char, data: *mut c_void) -> c_int; }

unsafe fn tuple_new(sz: c_uint) -> *mut PyObject {
    let t = PyTuple_New(sz as isize);
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    t
}

unsafe fn tuple_set_s64(t: *mut PyObject, pos: c_uint, val: s64) -> c_int {
    if BITS_PER_LONG == 64 { PyTuple_SetItem(t, pos as isize, PyLong_FromLong(val as c_long)) } else { PyTuple_SetItem(t, pos as isize, PyLong_FromLongLong(val)) }
}
unsafe fn tuple_set_d64(t: *mut PyObject, pos: c_uint, val: u64) -> c_int { tuple_set_s64(t, pos, val as s64) }
unsafe fn tuple_set_u64(t: *mut PyObject, pos: c_uint, val: u64) -> c_int {
    if BITS_PER_LONG == 64 { PyTuple_SetItem(t, pos as isize, PyLong_FromUnsignedLong(val as c_ulong)) } else { PyTuple_SetItem(t, pos as isize, PyLong_FromUnsignedLongLong(val)) }
}
unsafe fn tuple_set_u32(t: *mut PyObject, pos: c_uint, val: u32) -> c_int { PyTuple_SetItem(t, pos as isize, PyLong_FromUnsignedLong(val as c_ulong)) }
unsafe fn tuple_set_s32(t: *mut PyObject, pos: c_uint, val: s32) -> c_int { PyTuple_SetItem(t, pos as isize, PyLong_FromLong(val as c_long)) }
unsafe fn tuple_set_bool(t: *mut PyObject, pos: c_uint, val: bool) -> c_int { PyTuple_SetItem(t, pos as isize, PyBool_FromLong(val as c_long)) }
unsafe fn tuple_set_string(t: *mut PyObject, pos: c_uint, s: *const c_char) -> c_int { PyTuple_SetItem(t, pos as isize, PyUnicode_FromString(s)) }
unsafe fn tuple_set_bytes(t: *mut PyObject, pos: c_uint, bytes: *mut c_void, sz: c_uint) -> c_int { PyTuple_SetItem(t, pos as isize, PyBytes_FromStringAndSize(bytes as *const c_char, sz as isize)) }

unsafe fn tables_from_dbe(dbe: *mut db_export) -> *mut tables { dbe as *mut tables }

unsafe extern "C" fn python_export_evsel(dbe: *mut db_export, evsel: *mut evsel) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(2);
    tuple_set_d64(t, 0, (*evsel).db_id); tuple_set_string(t, 1, evsel__name(evsel));
    call_object((*tables).evsel_handler, t, cstr(b"evsel_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_machine(dbe: *mut db_export, machine: *mut machine) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(3);
    tuple_set_d64(t, 0, (*machine).db_id); tuple_set_s32(t, 1, (*machine).pid);
    tuple_set_string(t, 2, if !(*machine).root_dir.is_null() { (*machine).root_dir } else { cstr(b"\0") });
    call_object((*tables).machine_handler, t, cstr(b"machine_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_thread(dbe: *mut db_export, thread: *mut thread, main_thread_db_id: u64, machine: *mut machine) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(5);
    tuple_set_d64(t, 0, thread__db_id(thread)); tuple_set_d64(t, 1, (*machine).db_id); tuple_set_d64(t, 2, main_thread_db_id);
    tuple_set_s32(t, 3, thread__pid(thread)); tuple_set_s32(t, 4, thread__tid(thread));
    call_object((*tables).thread_handler, t, cstr(b"thread_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_comm(dbe: *mut db_export, comm: *mut comm, thread: *mut thread) -> c_int {
    extern "C" { fn comm__str(comm: *mut comm) -> *const c_char; }
    let tables = tables_from_dbe(dbe); let t = tuple_new(5);
    tuple_set_d64(t, 0, (*comm).db_id); tuple_set_string(t, 1, comm__str(comm)); tuple_set_d64(t, 2, thread__db_id(thread));
    tuple_set_d64(t, 3, (*comm).start); tuple_set_s32(t, 4, (*comm).exec);
    call_object((*tables).comm_handler, t, cstr(b"comm_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_comm_thread(dbe: *mut db_export, db_id: u64, comm: *mut comm, thread: *mut thread) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(3);
    tuple_set_d64(t, 0, db_id); tuple_set_d64(t, 1, (*comm).db_id); tuple_set_d64(t, 2, thread__db_id(thread));
    call_object((*tables).comm_thread_handler, t, cstr(b"comm_thread_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_dso(dbe: *mut db_export, dso: *mut dso, machine: *mut machine) -> c_int {
    let tables = tables_from_dbe(dbe); let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE]; build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
    let t = tuple_new(5); tuple_set_d64(t, 0, dso__db_id(dso)); tuple_set_d64(t, 1, (*machine).db_id);
    tuple_set_string(t, 2, dso__short_name(dso)); tuple_set_string(t, 3, dso__long_name(dso)); tuple_set_string(t, 4, sbuild_id.as_ptr());
    call_object((*tables).dso_handler, t, cstr(b"dso_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_symbol(dbe: *mut db_export, sym: *mut symbol, dso: *mut dso) -> c_int {
    let tables = tables_from_dbe(dbe); let sym_db_id = symbol__priv(sym) as *mut u64; let t = tuple_new(6);
    tuple_set_d64(t, 0, *sym_db_id); tuple_set_d64(t, 1, dso__db_id(dso)); tuple_set_d64(t, 2, (*sym).start); tuple_set_d64(t, 3, (*sym).end);
    tuple_set_s32(t, 4, symbol__binding(sym)); tuple_set_string(t, 5, (*sym).name);
    call_object((*tables).symbol_handler, t, cstr(b"symbol_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_branch_type(dbe: *mut db_export, branch_type: u32, name: *const c_char) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(2);
    tuple_set_s32(t, 0, branch_type as s32); tuple_set_string(t, 1, name);
    call_object((*tables).branch_type_handler, t, cstr(b"branch_type_table\0")); Py_DECREF(t); 0
}

unsafe fn python_export_sample_table(dbe: *mut db_export, es: *mut export_sample) {
    let tables = tables_from_dbe(dbe); let t = tuple_new(28); let sample = (*es).sample;
    tuple_set_d64(t, 0, (*es).db_id); tuple_set_d64(t, 1, (*(*sample).evsel).db_id);
    tuple_set_d64(t, 2, (*maps__machine(thread__maps((*(*es).al).thread))).db_id);
    tuple_set_d64(t, 3, thread__db_id((*(*es).al).thread)); tuple_set_d64(t, 4, (*es).comm_db_id);
    tuple_set_d64(t, 5, (*es).dso_db_id); tuple_set_d64(t, 6, (*es).sym_db_id); tuple_set_d64(t, 7, (*es).offset);
    tuple_set_d64(t, 8, (*sample).ip); tuple_set_d64(t, 9, (*sample).time); tuple_set_s32(t, 10, (*sample).cpu);
    tuple_set_d64(t, 11, (*es).addr_dso_db_id); tuple_set_d64(t, 12, (*es).addr_sym_db_id); tuple_set_d64(t, 13, (*es).addr_offset);
    tuple_set_d64(t, 14, (*sample).addr); tuple_set_d64(t, 15, (*sample).period); tuple_set_d64(t, 16, (*sample).weight);
    tuple_set_d64(t, 17, (*sample).transaction); tuple_set_d64(t, 18, (*sample).data_src); tuple_set_s32(t, 19, ((*sample).flags & PERF_BRANCH_MASK) as s32);
    tuple_set_s32(t, 20, ((*sample).flags & PERF_IP_FLAG_IN_TX != 0) as s32); tuple_set_d64(t, 21, (*es).call_path_id);
    tuple_set_d64(t, 22, (*sample).insn_cnt); tuple_set_d64(t, 23, (*sample).cyc_cnt); tuple_set_s32(t, 24, (*sample).flags as s32);
    tuple_set_d64(t, 25, (*sample).id); tuple_set_d64(t, 26, (*sample).stream_id); tuple_set_u32(t, 27, (*sample).ins_lat);
    call_object((*tables).sample_handler, t, cstr(b"sample_table\0")); Py_DECREF(t);
}
unsafe fn python_export_synth(dbe: *mut db_export, es: *mut export_sample) {
    let tables = tables_from_dbe(dbe); let t = tuple_new(3);
    tuple_set_d64(t, 0, (*es).db_id); tuple_set_d64(t, 1, (*(*(*es).sample).evsel).core.attr.config);
    tuple_set_bytes(t, 2, (*(*es).sample).raw_data, (*(*es).sample).raw_size);
    call_object((*tables).synth_handler, t, cstr(b"synth_data\0")); Py_DECREF(t);
}
unsafe extern "C" fn python_export_sample(dbe: *mut db_export, es: *mut export_sample) -> c_int {
    let tables = tables_from_dbe(dbe); python_export_sample_table(dbe, es);
    if (*(*(*es).sample).evsel).core.attr.type_ == PERF_TYPE_SYNTH && !(*tables).synth_handler.is_null() { python_export_synth(dbe, es); }
    0
}
unsafe extern "C" fn python_export_call_path(dbe: *mut db_export, cp: *mut call_path) -> c_int {
    let tables = tables_from_dbe(dbe); let parent_db_id = if !(*cp).parent.is_null() { (*(*cp).parent).db_id } else { 0 };
    let sym_db_id = if !(*cp).sym.is_null() { *(symbol__priv((*cp).sym) as *mut u64) } else { 0 };
    let t = tuple_new(4); tuple_set_d64(t, 0, (*cp).db_id); tuple_set_d64(t, 1, parent_db_id); tuple_set_d64(t, 2, sym_db_id); tuple_set_d64(t, 3, (*cp).ip);
    call_object((*tables).call_path_handler, t, cstr(b"call_path_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_call_return(dbe: *mut db_export, cr: *mut call_return) -> c_int {
    let tables = tables_from_dbe(dbe); let comm_db_id = if !(*cr).comm.is_null() { (*(*cr).comm).db_id } else { 0 }; let t = tuple_new(14);
    tuple_set_d64(t, 0, (*cr).db_id); tuple_set_d64(t, 1, thread__db_id((*cr).thread)); tuple_set_d64(t, 2, comm_db_id); tuple_set_d64(t, 3, (*(*cr).cp).db_id);
    tuple_set_d64(t, 4, (*cr).call_time); tuple_set_d64(t, 5, (*cr).return_time); tuple_set_d64(t, 6, (*cr).branch_count); tuple_set_d64(t, 7, (*cr).call_ref);
    tuple_set_d64(t, 8, (*cr).return_ref); tuple_set_d64(t, 9, (*(*(*cr).cp).parent).db_id); tuple_set_s32(t, 10, (*cr).flags); tuple_set_d64(t, 11, (*cr).parent_db_id);
    tuple_set_d64(t, 12, (*cr).insn_count); tuple_set_d64(t, 13, (*cr).cyc_count);
    call_object((*tables).call_return_handler, t, cstr(b"call_return_table\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_export_context_switch(dbe: *mut db_export, db_id: u64, machine: *mut machine, sample: *mut perf_sample, th_out_id: u64, comm_out_id: u64, th_in_id: u64, comm_in_id: u64, flags: c_int) -> c_int {
    let tables = tables_from_dbe(dbe); let t = tuple_new(9);
    tuple_set_d64(t, 0, db_id); tuple_set_d64(t, 1, (*machine).db_id); tuple_set_d64(t, 2, (*sample).time); tuple_set_s32(t, 3, (*sample).cpu);
    tuple_set_d64(t, 4, th_out_id); tuple_set_d64(t, 5, comm_out_id); tuple_set_d64(t, 6, th_in_id); tuple_set_d64(t, 7, comm_in_id); tuple_set_s32(t, 8, flags);
    call_object((*tables).context_switch_handler, t, cstr(b"context_switch\0")); Py_DECREF(t); 0
}
unsafe extern "C" fn python_process_call_return(cr: *mut call_return, parent_db_id: *mut u64, data: *mut c_void) -> c_int {
    db_export__call_return(data as *mut db_export, cr, parent_db_id)
}

unsafe fn python_process_general_event(sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location) {
    let handler_name = cstr(b"process_event\0");
    let handler = get_handler(handler_name);
    if handler.is_null() { return; }
    let mut n: isize = 0;
    let mut t = PyTuple_New(MAX_FIELDS as isize);
    if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    let callchain = python_process_callchain(sample, al);
    let dict = get_perf_sample_dict(sample, al, addr_al, callchain);
    PyTuple_SetItem(t, n, dict); n += 1;
    if _PyTuple_Resize(&mut t, n) == -1 { Py_FatalError(cstr(b"error resizing Python tuple\0")); }
    call_object(handler, t, handler_name);
    Py_DECREF(t);
}

unsafe extern "C" fn python_process_event(event: *mut perf_event, sample: *mut perf_sample, al: *mut addr_location, addr_al: *mut addr_location) {
    let tables = &mut tables_global as *mut tables;
    scripting_context__update(scripting_context, event, sample, al, addr_al);
    match (*(*sample).evsel).core.attr.type_ {
        PERF_TYPE_TRACEPOINT => python_process_tracepoint(sample, al, addr_al),
        _ => {
            if (*tables).db_export_mode { db_export__sample(&mut (*tables).dbe, event, sample, al, addr_al); }
            else { python_process_general_event(sample, al, addr_al); }
        }
    }
}

unsafe extern "C" fn python_process_throttle(event: *mut perf_event, sample: *mut perf_sample, _machine: *mut machine) {
    let handler_name = if (*event).header.type_ == PERF_RECORD_THROTTLE { cstr(b"throttle\0") } else { cstr(b"unthrottle\0") };
    let handler = get_handler(handler_name); if handler.is_null() { return; }
    let t = tuple_new(6);
    tuple_set_u64(t, 0, (*event).throttle.time); tuple_set_u64(t, 1, (*event).throttle.id); tuple_set_u64(t, 2, (*event).throttle.stream_id);
    tuple_set_s32(t, 3, (*sample).cpu); tuple_set_s32(t, 4, (*sample).pid); tuple_set_s32(t, 5, (*sample).tid);
    call_object(handler, t, handler_name); Py_DECREF(t);
}

unsafe fn python_do_process_switch(event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) {
    let handler_name = cstr(b"context_switch\0");
    let handler = get_handler(handler_name); if handler.is_null() { return; }
    let out = ((*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT) != 0;
    let out_preempt = out && ((*event).header.misc & PERF_RECORD_MISC_SWITCH_OUT_PREEMPT) != 0;
    let mut np_pid: pid_t = -1; let mut np_tid: pid_t = -1;
    if (*event).header.type_ == PERF_RECORD_SWITCH_CPU_WIDE {
        np_pid = (*event).context_switch.next_prev_pid; np_tid = (*event).context_switch.next_prev_tid;
    }
    let t = tuple_new(11);
    tuple_set_u64(t, 0, (*sample).time); tuple_set_s32(t, 1, (*sample).cpu); tuple_set_s32(t, 2, (*sample).pid); tuple_set_s32(t, 3, (*sample).tid);
    tuple_set_s32(t, 4, np_pid); tuple_set_s32(t, 5, np_tid); tuple_set_s32(t, 6, (*machine).pid); tuple_set_bool(t, 7, out); tuple_set_bool(t, 8, out_preempt);
    tuple_set_s32(t, 9, (*sample).machine_pid); tuple_set_s32(t, 10, (*sample).vcpu);
    call_object(handler, t, handler_name); Py_DECREF(t);
}

unsafe extern "C" fn python_process_switch(event: *mut perf_event, sample: *mut perf_sample, machine: *mut machine) {
    let tables = &mut tables_global as *mut tables;
    if (*tables).db_export_mode { db_export__switch(&mut (*tables).dbe, event, sample, machine); } else { python_do_process_switch(event, sample, machine); }
}

unsafe extern "C" fn python_process_auxtrace_error(_session: *mut perf_session, event: *mut perf_event) {
    let e = &mut (*event).auxtrace_error as *mut perf_record_auxtrace_error;
    let cpumode = ((*e).header.misc & PERF_RECORD_MISC_CPUMODE_MASK) as u8;
    let handler_name = cstr(b"auxtrace_error\0");
    let mut tm = (*e).time;
    let mut msg = (*e).msg.as_ptr();
    let mut machine_pid: s32 = 0; let mut vcpu: s32 = 0;
    let mut msg_buf = [0 as c_char; MAX_AUXTRACE_ERROR_MSG + 1];
    let handler = get_handler(handler_name); if handler.is_null() { return; }
    if (*e).fmt == 0 { tm = 0; msg = &(*e).time as *const _ as *const c_char; }
    let mut msg_max = (event as *mut u8).add((*event).header.size as usize).offset_from(msg as *mut u8) as c_int;
    if msg_max <= 0 { msg_buf[0] = 0; } else {
        if msg_max > msg_buf.len() as c_int - 1 { msg_max = msg_buf.len() as c_int - 1; }
        memcpy(msg_buf.as_mut_ptr() as *mut c_void, msg as *const c_void, msg_max as size_t);
        msg_buf[msg_max as usize] = 0;
    }
    if (*e).fmt >= 2 && (*event).header.size as usize >= offset_of!(perf_record_auxtrace_error, vcpu) + size_of::<s32>() {
        machine_pid = (*e).machine_pid; vcpu = (*e).vcpu;
    }
    let t = tuple_new(11);
    tuple_set_u32(t, 0, (*e).type_); tuple_set_u32(t, 1, (*e).code); tuple_set_s32(t, 2, (*e).cpu); tuple_set_s32(t, 3, (*e).pid); tuple_set_s32(t, 4, (*e).tid);
    tuple_set_u64(t, 5, (*e).ip); tuple_set_u64(t, 6, tm); tuple_set_string(t, 7, msg_buf.as_ptr()); tuple_set_u32(t, 8, cpumode as u32);
    tuple_set_s32(t, 9, machine_pid); tuple_set_s32(t, 10, vcpu);
    call_object(handler, t, handler_name); Py_DECREF(t);
}

unsafe fn get_handler_name(str_: *mut c_char, size: size_t, evsel: *mut evsel) {
    let mut p = str_;
    scnprintf(str_, size, cstr(b"stat__%s\0"), evsel__name(evsel));
    loop {
        p = strchr(p, ':' as c_int);
        if p.is_null() { break; }
        *p = '_' as c_char;
        p = p.add(1);
    }
}

unsafe fn process_stat(counter: *mut evsel, cpu: perf_cpu, thread: c_int, tstamp: u64, count: *mut perf_counts_values) {
    let mut handler_name = [0 as c_char; 256]; let mut n: isize = 0;
    let mut t = PyTuple_New(MAX_FIELDS as isize); if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    get_handler_name(handler_name.as_mut_ptr(), handler_name.len(), counter);
    let handler = get_handler(handler_name.as_ptr());
    if handler.is_null() { pr_debug(cstr(b"can't find python handler %s\n\0"), handler_name.as_ptr()); return; }
    PyTuple_SetItem(t, n, PyLong_FromLong(cpu.cpu as c_long)); n += 1;
    PyTuple_SetItem(t, n, PyLong_FromLong(thread as c_long)); n += 1;
    tuple_set_u64(t, n as c_uint, tstamp); n += 1; tuple_set_u64(t, n as c_uint, (*count).val); n += 1;
    tuple_set_u64(t, n as c_uint, (*count).ena); n += 1; tuple_set_u64(t, n as c_uint, (*count).run); n += 1;
    if _PyTuple_Resize(&mut t, n) == -1 { Py_FatalError(cstr(b"error resizing Python tuple\0")); }
    call_object(handler, t, handler_name.as_ptr()); Py_DECREF(t);
}

unsafe extern "C" fn python_process_stat(_config: *mut perf_stat_config, counter: *mut evsel, tstamp: u64) {
    let threads = (*counter).core.threads; let cpus = (*counter).core.cpus;
    for thread_idx in 0..perf_thread_map__nr(threads) {
        for idx in 0..perf_cpu_map__nr(cpus) as c_uint {
            let cpu = perf_cpu_map__cpu(cpus, idx);
            process_stat(counter, cpu, perf_thread_map__pid(threads, thread_idx), tstamp, perf_counts((*counter).counts, idx, thread_idx));
        }
    }
}

unsafe extern "C" fn python_process_stat_interval(tstamp: u64) {
    let handler_name = cstr(b"stat__interval\0"); let mut n: isize = 0;
    let mut t = PyTuple_New(MAX_FIELDS as isize); if t.is_null() { Py_FatalError(cstr(b"couldn't create Python tuple\0")); }
    let handler = get_handler(handler_name);
    if handler.is_null() { pr_debug(cstr(b"can't find python handler %s\n\0"), handler_name); return; }
    tuple_set_u64(t, n as c_uint, tstamp); n += 1;
    if _PyTuple_Resize(&mut t, n) == -1 { Py_FatalError(cstr(b"error resizing Python tuple\0")); }
    call_object(handler, t, handler_name); Py_DECREF(t);
}

unsafe fn perf_script_context_init() -> c_int {
    let perf_trace_context = PyImport_AddModule(cstr(b"perf_trace_context\0"));
    if perf_trace_context.is_null() { return -1; }
    let dict = PyModule_GetDict(perf_trace_context);
    if dict.is_null() { return -1; }
    let perf_script_context = PyCapsule_New(scripting_context as *mut c_void, ptr::null(), ptr::null_mut());
    if perf_script_context.is_null() { return -1; }
    let mut ret = PyDict_SetItemString(dict, cstr(b"perf_script_context\0"), perf_script_context);
    if ret == 0 { ret = PyDict_SetItemString(main_dict, cstr(b"perf_script_context\0"), perf_script_context); }
    Py_DECREF(perf_script_context);
    ret
}

unsafe fn run_start_sub() -> c_int {
    main_module = PyImport_AddModule(cstr(b"__main__\0"));
    if main_module.is_null() { return -1; }
    Py_INCREF(main_module);
    main_dict = PyModule_GetDict(main_module);
    if main_dict.is_null() { Py_XDECREF(main_module); return -1; }
    Py_INCREF(main_dict);
    if perf_script_context_init() != 0 {
        Py_XDECREF(main_dict); Py_XDECREF(main_module); return -1;
    }
    try_call_object(cstr(b"trace_begin\0"), ptr::null_mut());
    0
}

unsafe fn set_table_handlers(tables: *mut tables) {
    memset(tables as *mut c_void, 0, size_of::<tables>());
    if db_export__init(&mut (*tables).dbe) != 0 { Py_FatalError(cstr(b"failed to initialize export\0")); }
    let db_export_mode = PyDict_GetItemString(main_dict, cstr(b"perf_db_export_mode\0"));
    if db_export_mode.is_null() { return; }
    let mut ret = PyObject_IsTrue(db_export_mode);
    if ret == -1 { handler_call_die(cstr(b"perf_db_export_mode\0")); }
    if ret == 0 { return; }
    let db_export_calls = PyDict_GetItemString(main_dict, cstr(b"perf_db_export_calls\0"));
    let mut export_calls = false;
    if !db_export_calls.is_null() {
        ret = PyObject_IsTrue(db_export_calls); if ret == -1 { handler_call_die(cstr(b"perf_db_export_calls\0")); }
        export_calls = ret != 0;
    }
    /* The db_export callback field assignments from SET_TABLE_HANDLER are
     * represented by handler lookups here; concrete callback fields are
     * supplied by the external db_export layout.
     */
    if export_calls {
        let crp = call_return_processor__new(python_process_call_return, &mut (*tables).dbe as *mut _ as *mut c_void);
        if crp.is_null() { Py_FatalError(cstr(b"failed to create calls processor\0")); }
    }
    let db_export_callchains = PyDict_GetItemString(main_dict, cstr(b"perf_db_export_callchains\0"));
    let mut export_callchains = false;
    if !db_export_callchains.is_null() {
        ret = PyObject_IsTrue(db_export_callchains); if ret == -1 { handler_call_die(cstr(b"perf_db_export_callchains\0")); }
        export_callchains = ret != 0;
    }
    if export_callchains && call_path_root__new().is_null() {
        Py_FatalError(cstr(b"failed to create call path root\0"));
    }
    (*tables).db_export_mode = true;
    symbol_conf.priv_size = size_of::<u64>();
    (*tables).evsel_handler = get_handler(cstr(b"evsel_table\0"));
    (*tables).machine_handler = get_handler(cstr(b"machine_table\0"));
    (*tables).thread_handler = get_handler(cstr(b"thread_table\0"));
    (*tables).comm_handler = get_handler(cstr(b"comm_table\0"));
    (*tables).comm_thread_handler = get_handler(cstr(b"comm_thread_table\0"));
    (*tables).dso_handler = get_handler(cstr(b"dso_table\0"));
    (*tables).symbol_handler = get_handler(cstr(b"symbol_table\0"));
    (*tables).branch_type_handler = get_handler(cstr(b"branch_type_table\0"));
    (*tables).sample_handler = get_handler(cstr(b"sample_table\0"));
    (*tables).call_path_handler = get_handler(cstr(b"call_path_table\0"));
    (*tables).call_return_handler = get_handler(cstr(b"call_return_table\0"));
    (*tables).context_switch_handler = get_handler(cstr(b"context_switch_table\0"));
    (*tables).synth_handler = get_handler(cstr(b"synth_data\0"));
}

unsafe fn _free_command_line(command_line: *mut *mut wchar_t, num: c_int) {
    for i in 0..num { PyMem_RawFree(*command_line.add(i as usize) as *mut c_void); }
    free(command_line as *mut c_void);
}

unsafe extern "C" fn python_start_script(script: *const c_char, argc: c_int, argv: *mut *const c_char, session: *mut perf_session) -> c_int {
    let tables = &mut tables_global as *mut tables;
    let mut buf = [0 as c_char; PATH_MAX];
    let mut err = 0;
    (*scripting_context).session = session;
    let command_line = malloc(((argc + 1) as usize) * size_of::<*mut wchar_t>()) as *mut *mut wchar_t;
    if command_line.is_null() { return -1; }
    *command_line = Py_DecodeLocale(script, ptr::null_mut());
    for i in 1..(argc + 1) { *command_line.add(i as usize) = Py_DecodeLocale(*argv.add((i - 1) as usize), ptr::null_mut()); }
    PyImport_AppendInittab(cstr(b"perf_trace_context\0"), PyInit_perf_trace_context);
    Py_Initialize();
    PySys_SetArgv(argc + 1, command_line);
    let fp = fopen(script, cstr(b"r\0"));
    if fp.is_null() {
        sprintf(buf.as_mut_ptr(), cstr(b"Can't open python script \"%s\"\0"), script);
        perror(buf.as_ptr()); err = -1; Py_Finalize(); _free_command_line(command_line, argc + 1); return err;
    }
    err = PyRun_SimpleFile(fp, script);
    if err != 0 {
        fprintf(stderr, cstr(b"Error running python script %s\n\0"), script);
        Py_Finalize(); _free_command_line(command_line, argc + 1); return err;
    }
    err = run_start_sub();
    if err != 0 {
        fprintf(stderr, cstr(b"Error starting python script %s\n\0"), script);
        Py_Finalize(); _free_command_line(command_line, argc + 1); return err;
    }
    set_table_handlers(tables);
    if (*tables).db_export_mode {
        err = db_export__branch_types(&mut (*tables).dbe);
        if err != 0 { Py_Finalize(); _free_command_line(command_line, argc + 1); return err; }
    }
    _free_command_line(command_line, argc + 1);
    err
}

unsafe extern "C" fn python_flush_script() -> c_int { 0 }

unsafe extern "C" fn python_stop_script() -> c_int {
    let tables = &mut tables_global as *mut tables;
    try_call_object(cstr(b"trace_end\0"), ptr::null_mut());
    db_export__exit(&mut (*tables).dbe);
    Py_XDECREF(main_dict);
    Py_XDECREF(main_module);
    Py_Finalize();
    0
}

unsafe extern "C" fn python_generate_script(pevent: *mut tep_handle, outfile: *const c_char) -> c_int {
    let mut fname = [0 as c_char; PATH_MAX];
    sprintf(fname.as_mut_ptr(), cstr(b"%s.py\0"), outfile);
    let ofp = fopen(fname.as_ptr(), cstr(b"w\0"));
    if ofp.is_null() {
        fprintf(stderr, cstr(b"couldn't open %s\n\0"), fname.as_ptr());
        return -1;
    }
    fprintf(ofp, cstr(b"# perf script event handlers, generated by perf script -g python\n\0"));
    fprintf(ofp, cstr(b"# Licensed under the terms of the GNU GPL License version 2\n\n\0"));
    fprintf(ofp, cstr(b"# The common_* event handler fields are the most useful fields common to\n\0"));
    fprintf(ofp, cstr(b"# all events.  They don't necessarily correspond to the 'common_*' fields\n\0"));
    fprintf(ofp, cstr(b"# in the format files.  Those fields not available as handler params can\n\0"));
    fprintf(ofp, cstr(b"# be retrieved using Python functions of the form common_*(context).\n\0"));
    fprintf(ofp, cstr(b"# See the perf-script-python Documentation for the list of available functions.\n\n\0"));
    fprintf(ofp, cstr(b"from __future__ import print_function\n\nimport os\nimport sys\n\n\0"));
    fprintf(ofp, cstr(b"sys.path.append(os.environ['PERF_EXEC_PATH'] + \\\n\t'/scripts/python/Perf-Trace-Util/lib/Perf/Trace')\n\0"));
    fprintf(ofp, cstr(b"\nfrom perf_trace_context import *\nfrom Core import *\n\n\n\0"));
    fprintf(ofp, cstr(b"def trace_begin():\n\tprint(\"in trace_begin\")\n\n\0"));
    fprintf(ofp, cstr(b"def trace_end():\n\tprint(\"in trace_end\")\n\n\0"));
    let nr_events = tep_get_events_count(pevent);
    let all_events = tep_list_events(pevent, 0);
    for i in 0..nr_events {
        if all_events.is_null() { break; }
        let event = *all_events.add(i as usize);
        fprintf(ofp, cstr(b"def %s__%s(\0"), (*event).system, (*event).name);
        fprintf(ofp, cstr(b"event_name, context, common_cpu,\n\tcommon_secs, common_nsecs, common_pid, common_comm,\n\tcommon_callchain, \0"));
        let mut not_first = 0; let mut count = 0; let mut f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 { fprintf(ofp, cstr(b", \0")); } not_first += 1;
            count += 1; if count % 5 == 0 { fprintf(ofp, cstr(b"\n\t\0")); }
            fprintf(ofp, cstr(b"%s\0"), (*f).name); f = (*f).next;
        }
        if not_first != 0 { fprintf(ofp, cstr(b", \0")); }
        count += 1; if count % 5 == 0 { fprintf(ofp, cstr(b"\n\t\t\0")); }
        fprintf(ofp, cstr(b"perf_sample_dict):\n\0"));
        fprintf(ofp, cstr(b"\t\tprint_header(event_name, common_cpu, common_secs, common_nsecs,\n\t\t\tcommon_pid, common_comm)\n\n\0"));
        fprintf(ofp, cstr(b"\t\tprint(\"\0"));
        not_first = 0; count = 0; f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 { fprintf(ofp, cstr(b", \0")); } not_first += 1;
            if count != 0 && count % 3 == 0 { fprintf(ofp, cstr(b"\" \\\n\t\t\"\0")); }
            count += 1; fprintf(ofp, cstr(b"%s=\0"), (*f).name);
            if ((*f).flags & (TEP_FIELD_IS_STRING | TEP_FIELD_IS_FLAG | TEP_FIELD_IS_ARRAY | TEP_FIELD_IS_SYMBOLIC)) != 0 { fprintf(ofp, cstr(b"%%s\0")); }
            else if ((*f).flags & TEP_FIELD_IS_SIGNED) != 0 { fprintf(ofp, cstr(b"%%d\0")); }
            else { fprintf(ofp, cstr(b"%%u\0")); }
            f = (*f).next;
        }
        fprintf(ofp, cstr(b"\" %% \\\n\t\t(\0"));
        not_first = 0; count = 0; f = (*event).format.fields;
        while !f.is_null() {
            if not_first != 0 { fprintf(ofp, cstr(b", \0")); } not_first += 1;
            count += 1; if count % 5 == 0 { fprintf(ofp, cstr(b"\n\t\t\0")); }
            if ((*f).flags & TEP_FIELD_IS_FLAG) != 0 {
                if (count - 1) % 5 != 0 { fprintf(ofp, cstr(b"\n\t\t\0")); count = 4; }
                fprintf(ofp, cstr(b"flag_str(\"%s__%s\", \"%s\", %s)\0"), (*event).system, (*event).name, (*f).name, (*f).name);
            } else if ((*f).flags & TEP_FIELD_IS_SYMBOLIC) != 0 {
                if (count - 1) % 5 != 0 { fprintf(ofp, cstr(b"\n\t\t\0")); count = 4; }
                fprintf(ofp, cstr(b"symbol_str(\"%s__%s\", \"%s\", %s)\0"), (*event).system, (*event).name, (*f).name, (*f).name);
            } else {
                fprintf(ofp, cstr(b"%s\0"), (*f).name);
            }
            f = (*f).next;
        }
        fprintf(ofp, cstr(b"))\n\n\0"));
        fprintf(ofp, cstr(b"\t\tprint('Sample: {'+get_dict_as_string(perf_sample_dict['sample'], ', ')+'}')\n\n\0"));
        fprintf(ofp, cstr(b"\t\tfor node in common_callchain:\n\t\t\tif 'sym' in node:\n\t\t\t\tprint(\"\\t[%%x] %%s%%s%%s%%s\" %% (\n\t\t\t\t\tnode['ip'], node['sym']['name'],\n\t\t\t\t\t\"+0x{:x}\".format(node['sym_off']) if 'sym_off' in node else \"\",\n\t\t\t\t\t\" ({})\".format(node['dso'])  if 'dso' in node else \"\",\n\t\t\t\t\t\" \" + node['sym_srcline'] if 'sym_srcline' in node else \"\"))\n\t\t\telse:\n\t\t\t\tprint(\"\\t[%%x]\" %% (node['ip']))\n\n\t\tprint()\n\n\0"));
    }
    fprintf(ofp, cstr(b"def trace_unhandled(event_name, context, event_fields_dict, perf_sample_dict):\n\0"));
    fprintf(ofp, cstr(b"\t\tprint(get_dict_as_string(event_fields_dict))\n\t\tprint('Sample: {'+get_dict_as_string(perf_sample_dict['sample'], ', ')+'}')\n\n\0"));
    fprintf(ofp, cstr(b"def print_header(event_name, cpu, secs, nsecs, pid, comm):\n\tprint(\"%%-20s %%5u %%05u.%%09u %%8u %%-20s \" %% \\\n\t(event_name, cpu, secs, nsecs, pid, comm), end=\"\")\n\n\0"));
    fprintf(ofp, cstr(b"def get_dict_as_string(a_dict, delimiter=' '):\n\treturn delimiter.join(['%%s=%%s'%%(k,str(v))for k,v in sorted(a_dict.items())])\n\0"));
    fclose(ofp);
    fprintf(stderr, cstr(b"generated Python script: %s\n\0"), fname.as_ptr());
    0
}

#[no_mangle]
pub static mut python_scripting_ops: scripting_ops = scripting_ops {
    name: b"Python\0".as_ptr() as *const c_char,
    dirname: b"python\0".as_ptr() as *const c_char,
    start_script: Some(python_start_script),
    flush_script: Some(python_flush_script),
    stop_script: Some(python_stop_script),
    process_event: Some(python_process_event),
    process_switch: Some(python_process_switch),
    process_auxtrace_error: Some(python_process_auxtrace_error),
    process_stat: Some(python_process_stat),
    process_stat_interval: Some(python_process_stat_interval),
    process_throttle: Some(python_process_throttle),
    generate_script: Some(python_generate_script),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
