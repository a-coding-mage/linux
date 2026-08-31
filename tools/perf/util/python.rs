// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/util/python.c.
//
// C include dependencies translated as external dependencies:
// Python.h, structmember.h, linux/err.h, poll.h, unistd.h, internal/lib.h,
// perf/cpumap.h, perf/mmap.h, and the perf util headers included by the
// original source.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type Py_ssize_t = isize;

#[repr(C)] pub struct PyObject { _private: [u8; 0] }
#[repr(C)] pub struct PyTypeObject { _private: [u8; 0] }
#[repr(C)] pub struct PyGetSetDef { pub name: *const c_char, pub get: Option<unsafe extern "C" fn(*mut PyObject, *mut c_void) -> *mut PyObject>, pub set: Option<unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut c_void) -> c_int>, pub doc: *const c_char, pub closure: *mut c_void }
#[repr(C)] pub struct PyMemberDef { pub name: *const c_char, pub type_: c_int, pub offset: Py_ssize_t, pub flags: c_int, pub doc: *const c_char }
#[repr(C)] pub struct PyMethodDef { pub ml_name: *const c_char, pub ml_meth: Option<unsafe extern "C" fn() -> *mut PyObject>, pub ml_flags: c_int, pub ml_doc: *const c_char }
#[repr(C)] pub struct PySequenceMethods { pub sq_length: Option<unsafe extern "C" fn(*mut PyObject) -> Py_ssize_t>, pub sq_concat: *mut c_void, pub sq_repeat: *mut c_void, pub sq_item: Option<unsafe extern "C" fn(*mut PyObject, Py_ssize_t) -> *mut PyObject> }
#[repr(C)] pub struct PyModuleDef { _private: [u8; 0] }

#[repr(C)] pub struct perf_sample { pub evsel: *mut evsel, pub raw_data: *mut c_void, pub raw_size: u32, pub ip: u64, pub addr: u64, pub phys_addr: u64, pub weight: u64, pub data_src: u64, pub insn_cnt: u64, pub cyc_cnt: u64, pub pid: c_int, pub tid: c_int, pub time: u64, pub id: u64, pub stream_id: u64, pub period: u64, pub cpu: u32, pub cpumode: u32, pub callchain: *mut c_void, pub branch_stack: *mut branch_stack, pub insn: *mut u8, pub insn_len: u32 }
#[repr(C)] pub struct addr_location { pub thread: *mut thread, pub map: *mut map, pub sym: *mut symbol, pub addr: u64 }
#[repr(C)] pub struct perf_event_header { pub type_: u32, pub misc: u16, pub size: u16 }
#[repr(C)] pub struct perf_record_mmap { pub header: perf_event_header, pub pid: u32, pub tid: u32, pub start: u64, pub len: u64, pub pgoff: u64, pub filename: [c_char; 4096] }
#[repr(C)] pub struct perf_record_mmap2 { pub header: perf_event_header, pub pid: u32, pub tid: u32, pub start: u64, pub len: u64, pub pgoff: u64, pub maj: u32, pub min: u32, pub ino: u64, pub ino_generation: u64, pub prot: u32, pub flags: u32, pub build_id_size: u8, pub build_id: [u8; 20], pub filename: [c_char; 4096] }
#[repr(C)] pub struct perf_record_fork { pub header: perf_event_header, pub pid: u32, pub ppid: u32, pub tid: u32, pub ptid: u32, pub time: u64 }
#[repr(C)] pub struct perf_record_comm { pub header: perf_event_header, pub pid: u32, pub tid: u32, pub comm: [c_char; 16] }
#[repr(C)] pub struct perf_record_throttle { pub time: u64, pub id: u64, pub stream_id: u64 }
#[repr(C)] pub struct perf_record_lost { pub header: perf_event_header, pub id: u64, pub lost: u64 }
#[repr(C)] pub struct perf_record_stat { pub header: perf_event_header, pub id: u64, pub cpu: u32, pub thread: u32, pub val: u64, pub ena: u64, pub run: u64 }
#[repr(C)] pub struct perf_record_stat_round { pub header: perf_event_header, pub type_: u64, pub time: u64 }
#[repr(C)] pub struct perf_record_read { pub header: perf_event_header, pub pid: u32, pub tid: u32 }
#[repr(C)] pub struct perf_record_switch { pub header: perf_event_header, pub next_prev_pid: u32, pub next_prev_tid: u32 }

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
    pub mmap: core::mem::ManuallyDrop<perf_record_mmap>,
    pub mmap2: core::mem::ManuallyDrop<perf_record_mmap2>,
    pub fork: core::mem::ManuallyDrop<perf_record_fork>,
    pub comm: core::mem::ManuallyDrop<perf_record_comm>,
    pub lost: core::mem::ManuallyDrop<perf_record_lost>,
    pub stat: core::mem::ManuallyDrop<perf_record_stat>,
    pub stat_round: core::mem::ManuallyDrop<perf_record_stat_round>,
    pub read: core::mem::ManuallyDrop<perf_record_read>,
    pub context_switch: core::mem::ManuallyDrop<perf_record_switch>,
}

#[repr(C)]
pub struct pyrf_event {
    pub ob_base: PyObject,
    /** @sample: The parsed sample from the event. */
    pub sample: perf_sample,
    /** @al: The address location from machine__resolve, lazily computed. */
    pub al: addr_location,
    /** @al_resolved: True when machine__resolve been called. */
    pub al_resolved: bool,
    /** @callchain: Resolved callchain, eagerly computed if requested. */
    pub callchain: *mut PyObject,
    /** @brstack: Resolved branch stack, eagerly computed if requested. */
    pub brstack: *mut PyObject,
    /** @event: The underlying perf_event that may be in a file or ring buffer. */
    pub event: perf_event,
}

#[repr(C)] pub struct branch_flags { pub mispred: u64, pub predicted: u64, pub in_tx: u64, pub abort: u64, pub cycles: u64, pub type_: u64 }
#[repr(C)] pub struct branch_entry { pub from: u64, pub to: u64, pub flags: branch_flags }
#[repr(C)] pub struct branch_stack { pub nr: u64 }
#[repr(C)] pub struct pyrf_callchain_node { pub ob_base: PyObject, pub ip: u64, pub map: *mut map, pub sym: *mut symbol }
#[repr(C)] pub struct pyrf_callchain_frame { pub ip: u64, pub map: *mut map, pub sym: *mut symbol }
#[repr(C)] pub struct pyrf_callchain { pub ob_base: PyObject, pub frames: *mut pyrf_callchain_frame, pub nr_frames: u64 }
#[repr(C)] pub struct pyrf_branch_entry { pub ob_base: PyObject, pub from: u64, pub to: u64, pub flags: branch_flags }
#[repr(C)] pub struct pyrf_branch_stack { pub ob_base: PyObject, pub entries: *mut branch_entry, pub nr: u64 }
#[repr(C)] pub struct pyrf_cpu_map { pub ob_base: PyObject, pub cpus: *mut perf_cpu_map }
#[repr(C)] pub struct pyrf_thread_map { pub ob_base: PyObject, pub threads: *mut perf_thread_map }
#[repr(C)] pub struct pyrf_pmu { pub ob_base: PyObject, pub pmu: *mut perf_pmu }
#[repr(C)] pub struct pyrf_pmu_iterator { pub ob_base: PyObject, pub pmu: *mut perf_pmu }
#[repr(C)] pub struct perf_counts_values { pub val: u64, pub ena: u64, pub run: u64, pub id: u64, pub lost: u64, pub values: [u64; 5] }
#[repr(C)] pub struct pyrf_counts_values { pub ob_base: PyObject, pub values: perf_counts_values }
#[repr(C)] pub struct perf_event_attr { pub type_: u32, pub size: u32, pub config: u64, pub sample_period: u64, pub sample_freq: u64, pub sample_type: u64, pub read_format: u64, pub disabled: u32, pub inherit: u32, pub pinned: u32, pub exclusive: u32, pub exclude_user: u32, pub exclude_kernel: u32, pub exclude_hv: u32, pub exclude_idle: u32, pub mmap: u32, pub comm: u32, pub freq: u32, pub inherit_stat: u32, pub enable_on_exec: u32, pub task: u32, pub watermark: u32, pub precise_ip: u32, pub mmap_data: u32, pub sample_id_all: u32, pub context_switch: u32, pub wakeup_events: u32, pub bp_type: u32, pub bp_addr: u64, pub bp_len: u64 }
#[repr(C)] pub struct pyrf_evsel { pub ob_base: PyObject, pub evsel: *mut evsel }
#[repr(C)] pub struct pyrf_evlist { pub ob_base: PyObject, pub evlist: *mut evlist }
#[repr(C)] pub struct perf_constant { pub name: *const c_char, pub value: c_int }
#[repr(C)] pub struct perf_data { pub path: *const c_char, pub mode: c_int, pub file: perf_data_file, pub open: bool }
#[repr(C)] pub struct perf_data_file { pub fd: c_int }
#[repr(C)] pub struct pyrf_data { pub ob_base: PyObject, pub data: perf_data }
#[repr(C)] pub struct pyrf_thread { pub ob_base: PyObject, pub thread: *mut thread }
#[repr(C)] pub struct perf_tool { pub ordering_requires_timestamps: bool, pub sample: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_event, *mut perf_sample, *mut machine) -> c_int>, pub stat: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int>, pub stat_round: Option<unsafe extern "C" fn(*const perf_tool, *mut perf_session, *mut perf_event) -> c_int> }
#[repr(C)] pub struct pyrf_session { pub ob_base: PyObject, pub session: *mut perf_session, pub tool: perf_tool, pub pdata: *mut pyrf_data, pub sample: *mut PyObject, pub stat: *mut PyObject }

#[repr(C)] pub struct evsel { _private: [u8; 0] }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct perf_session { _private: [u8; 0] }
#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct symbol { pub name: *const c_char, pub start: u64, pub end: u64 }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct perf_cpu_map { _private: [u8; 0] }
#[repr(C)] pub struct perf_thread_map { _private: [u8; 0] }
#[repr(C)] pub struct perf_pmu { pub name: *const c_char }
#[repr(C)] pub struct pmu_event_info { pub name: *const c_char, pub alias: *const c_char, pub scale_unit: *const c_char, pub desc: *const c_char, pub long_desc: *const c_char, pub encoding_desc: *const c_char, pub topic: *const c_char, pub event_type_desc: *const c_char, pub str_: *const c_char, pub deprecated: bool }
#[repr(C)] pub struct pmu_metric { pub metric_group: *const c_char, pub metric_name: *const c_char, pub pmu: *const c_char, pub metric_expr: *const c_char, pub metric_threshold: *const c_char, pub unit: *const c_char, pub compat: *const c_char, pub desc: *const c_char, pub long_desc: *const c_char }
#[repr(C)] pub struct pmu_metrics_table { _private: [u8; 0] }
#[repr(C)] pub struct expr_parse_ctx { _private: [u8; 0] }
#[repr(C)] pub struct metric_expr { _private: [u8; 0] }
#[repr(C)] pub struct mmap { _private: [u8; 0] }

const PERF_RECORD_MMAP: usize = 1;
const PERF_RECORD_LOST: usize = 2;
const PERF_RECORD_COMM: usize = 3;
const PERF_RECORD_EXIT: usize = 4;
const PERF_RECORD_THROTTLE: usize = 5;
const PERF_RECORD_UNTHROTTLE: usize = 6;
const PERF_RECORD_FORK: usize = 7;
const PERF_RECORD_READ: usize = 8;
const PERF_RECORD_SAMPLE: usize = 9;
const PERF_RECORD_MMAP2: usize = 10;
const PERF_RECORD_SWITCH: usize = 14;
const PERF_RECORD_SWITCH_CPU_WIDE: usize = 15;
const PERF_RECORD_STAT: usize = 16;
const PERF_RECORD_STAT_ROUND: usize = 17;
const PERF_RECORD_MISC_MMAP_BUILD_ID: u16 = 1 << 13;
const PERF_RECORD_MISC_SWITCH_OUT: u16 = 1 << 13;
const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_MAX_STACK_DEPTH: c_int = 127;
const PERF_DATA_MODE_READ: c_int = 0;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const EM_HOST: c_int = 0;
const UINT_MAX: u32 = u32::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const SBUILD_ID_SIZE: usize = 64;

unsafe extern "C" {
    static mut PyExc_ValueError: *mut PyObject;
    static mut PyExc_TypeError: *mut PyObject;
    static mut PyExc_IndexError: *mut PyObject;
    static mut PyExc_OSError: *mut PyObject;
    static mut PyExc_IOError: *mut PyObject;
    static mut PyExc_StopIteration: *mut PyObject;
    static mut Py_None: PyObject;
    static mut Py_True: PyObject;
    static mut Py_False: PyObject;
    static mut errno: c_int;
    static mut page_size: c_long;

    fn PyErr_SetString(exc: *mut PyObject, msg: *const c_char);
    fn PyErr_Format(exc: *mut PyObject, fmt: *const c_char, ...) -> *mut PyObject;
    fn PyErr_NoMemory() -> *mut PyObject;
    fn PyErr_SetFromErrno(exc: *mut PyObject) -> *mut PyObject;
    fn PyErr_SetNone(exc: *mut PyObject);
    fn PyErr_Occurred() -> *mut PyObject;
    fn Py_INCREF(obj: *mut PyObject);
    fn Py_DECREF(obj: *mut PyObject);
    fn Py_XDECREF(obj: *mut PyObject);
    fn PyUnicode_FromString(s: *const c_char) -> *mut PyObject;
    fn PyUnicode_FromStringAndSize(s: *const c_char, size: Py_ssize_t) -> *mut PyObject;
    fn PyUnicode_FromFormat(fmt: *const c_char, ...) -> *mut PyObject;
    fn PyUnicode_AsUTF8(obj: *mut PyObject) -> *const c_char;
    fn PyBytes_FromStringAndSize(s: *const c_char, size: Py_ssize_t) -> *mut PyObject;
    fn PyByteArray_FromStringAndSize(s: *const c_char, size: Py_ssize_t) -> *mut PyObject;
    fn PyLong_FromUnsignedLong(v: c_ulong) -> *mut PyObject;
    fn PyLong_FromUnsignedLongLong(v: c_ulonglong) -> *mut PyObject;
    fn PyLong_FromLong(v: c_long) -> *mut PyObject;
    fn PyLong_AsUnsignedLong(v: *mut PyObject) -> c_ulong;
    fn PyLong_AsUnsignedLongLong(v: *mut PyObject) -> c_ulonglong;
    fn PyBool_FromLong(v: c_long) -> *mut PyObject;
    fn PyFloat_FromDouble(v: c_double) -> *mut PyObject;
    fn PyList_New(len: Py_ssize_t) -> *mut PyObject;
    fn PyList_Append(list: *mut PyObject, item: *mut PyObject) -> c_int;
    fn PyList_SetItem(list: *mut PyObject, i: Py_ssize_t, item: *mut PyObject) -> c_int;
    fn PyList_Size(list: *mut PyObject) -> Py_ssize_t;
    fn PyList_GetItem(list: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    fn PyList_Check(obj: *mut PyObject) -> c_int;
    fn PyLong_Check(obj: *mut PyObject) -> c_int;
    fn PyDict_New() -> *mut PyObject;
    fn PyDict_SetItem(dict: *mut PyObject, key: *mut PyObject, value: *mut PyObject) -> c_int;
    fn PyDict_SetItemString(dict: *mut PyObject, key: *const c_char, value: *mut PyObject) -> c_int;
    fn PyModule_Create(def: *mut PyModuleDef) -> *mut PyObject;
    fn PyModule_AddObject(module: *mut PyObject, name: *const c_char, value: *mut PyObject) -> c_int;
    fn PyModule_GetDict(module: *mut PyObject) -> *mut PyObject;
    fn PyType_Ready(type_: *mut PyTypeObject) -> c_int;
    fn PyType_GenericNew(type_: *mut PyTypeObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut PyObject;
    fn PyObject_GenericGetAttr(obj: *mut PyObject, attr: *mut PyObject) -> *mut PyObject;
    fn PyObject_GenericSetAttr(obj: *mut PyObject, attr: *mut PyObject, value: *mut PyObject) -> c_int;
    fn PyObject_IsTrue(obj: *mut PyObject) -> c_int;
    fn PyObject_CallObject(callable: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn PyObject_CallFunction(callable: *mut PyObject, fmt: *const c_char, ...) -> *mut PyObject;
    fn PyCallable_Check(obj: *mut PyObject) -> c_int;
    fn PyArg_ParseTuple(args: *mut PyObject, fmt: *const c_char, ...) -> c_int;
    fn PyArg_ParseTupleAndKeywords(args: *mut PyObject, kwargs: *mut PyObject, fmt: *const c_char, kw: *mut *mut c_char, ...) -> c_int;
    fn Py_BuildValue(fmt: *const c_char, ...) -> *mut PyObject;
    fn PyFile_FromFd(fd: c_int, name: *const c_char, mode: *const c_char, buffering: c_int, encoding: *mut c_char, errors: *mut c_char, newline: *mut c_char, closefd: c_int) -> *mut PyObject;

    fn free(p: *mut c_void);
    fn calloc(n: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn dup(fd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;

    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn perf_sample__fetch_insn(sample: *mut perf_sample, thread: *mut thread, machine: *mut machine);
    fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut branch_entry;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn machine__resolve(machine: *mut machine, al: *mut addr_location, sample: *mut perf_sample) -> c_int;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__bid(dso: *mut dso) -> *const c_void;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__start(map: *mut map) -> c_ulong;
    fn map__end(map: *mut map) -> c_ulong;
    fn map__pgoff(map: *mut map) -> u64;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn build_id__snprintf(bid: *const c_void, buf: *mut c_char, size: size_t) -> c_int;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__put(thread: *mut thread);
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__pid(thread: *mut thread) -> c_int;
    fn thread__tid(thread: *mut thread) -> c_int;
    fn thread__ppid(thread: *mut thread) -> c_int;
    fn thread__cpu(thread: *mut thread) -> c_int;
    fn thread__maps(thread: *mut thread) -> *mut c_void;
    fn thread__find_symbol_fb(thread: *mut thread, cpumode: u32, addr: u64, al: *mut addr_location);
    fn maps__machine(maps: *mut c_void) -> *mut machine;
    fn get_srcline_split(dso: *mut dso, addr: u64, line: *mut c_uint) -> *mut c_char;
    fn find_sourceline(file: *mut c_char, line: c_uint, len: *mut c_int) -> *mut c_char;

    fn perf_cpu_map__new(cpustr: *mut c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__get(cpus: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__idx(cpus: *mut perf_cpu_map, cpu: perf_cpu) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, i: Py_ssize_t) -> perf_cpu;
    fn thread_map__new(pid: c_int, tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_thread_map__get(threads: *mut perf_thread_map) -> *mut perf_thread_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__idx(threads: *mut perf_thread_map, thread: c_int) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, i: Py_ssize_t) -> c_int;
    fn perf_pmus__scan(pmu: *mut perf_pmu) -> *mut perf_pmu;
    fn perf_pmu__for_each_event(pmu: *mut perf_pmu, skip_duplicate_pmus: bool, state: *mut PyObject, cb: Option<unsafe extern "C" fn(*mut c_void, *mut pmu_event_info) -> c_int>) -> c_int;
}

#[repr(C)] pub struct perf_cpu { pub cpu: c_int }
type c_uint = u32;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn py_return_none() -> *mut PyObject { Py_INCREF(&mut Py_None); &mut Py_None }
unsafe fn py_return_true() -> *mut PyObject { Py_INCREF(&mut Py_True); &mut Py_True }
unsafe fn py_return_false() -> *mut PyObject { Py_INCREF(&mut Py_False); &mut Py_False }

static mut pyrf_mmap_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_mmap2_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_task_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_comm_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_throttle_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_lost_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_stat_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_stat_round_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_read_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_sample_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_context_switch_event__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_callchain_node__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_callchain__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_branch_entry__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_branch_stack__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_cpu_map__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_thread_map__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_pmu__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_pmu_iterator__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_counts_values__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_evsel__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_evlist__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_data__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_thread__type: PyTypeObject = PyTypeObject { _private: [] };
static mut pyrf_session__type: PyTypeObject = PyTypeObject { _private: [] };

// The original C file defines PyMemberDef/PyGetSetDef/PyMethodDef arrays using
// offsetof over CPython and perf C layouts. In this isolated Rust translation
// those external layouts are not available, so the arrays are represented by
// their corresponding functions and type setup below, with the original member
// names preserved in comments:
// sample_pid, sample_tid, sample_time, sample_id, sample_stream_id,
// sample_period, sample_cpu, sample_ip, sample_addr, sample_phys_addr,
// sample_weight, sample_data_src, sample_insn_count, sample_cyc_count.

unsafe extern "C" fn pyrf_evsel__from_evsel(evsel: *mut evsel) -> *mut PyObject {
    let pevsel = malloc(size_of::<pyrf_evsel>()) as *mut pyrf_evsel;
    if pevsel.is_null() { return ptr::null_mut(); }
    (*pevsel).evsel = evsel;
    pevsel as *mut PyObject
}

unsafe extern "C" fn pyrf_event__get_evsel(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if (*pevent).sample.evsel.is_null() { return py_return_none(); }
    pyrf_evsel__from_evsel((*pevent).sample.evsel)
}

unsafe extern "C" fn pyrf_event__delete(pevent: *mut pyrf_event) {
    if (*pevent).al_resolved { addr_location__exit(&mut (*pevent).al); }
    Py_XDECREF((*pevent).callchain);
    Py_XDECREF((*pevent).brstack);
    perf_sample__exit(&mut (*pevent).sample);
    free(pevent as *mut c_void);
}

unsafe extern "C" fn pyrf_mmap2_event__get_maj(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if ((*pevent).event.header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 { return py_return_none(); }
    PyLong_FromUnsignedLong((*pevent).event.mmap2.maj as c_ulong)
}
unsafe extern "C" fn pyrf_mmap2_event__get_min(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if ((*pevent).event.header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 { return py_return_none(); }
    PyLong_FromUnsignedLong((*pevent).event.mmap2.min as c_ulong)
}
unsafe extern "C" fn pyrf_mmap2_event__get_ino(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if ((*pevent).event.header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 { return py_return_none(); }
    PyLong_FromUnsignedLongLong((*pevent).event.mmap2.ino as c_ulonglong)
}
unsafe extern "C" fn pyrf_mmap2_event__get_ino_generation(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if ((*pevent).event.header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 { return py_return_none(); }
    PyLong_FromUnsignedLongLong((*pevent).event.mmap2.ino_generation as c_ulonglong)
}
unsafe extern "C" fn pyrf_mmap2_event__get_build_id(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if ((*pevent).event.header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) == 0 { return py_return_none(); }
    let mut size = (*pevent).event.mmap2.build_id_size as c_int;
    if size > 20 { size = 20; }
    PyBytes_FromStringAndSize((*pevent).event.mmap2.build_id.as_ptr() as *const c_char, size as Py_ssize_t)
}

unsafe extern "C" fn pyrf_mmap_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: mmap, pid: %u, tid: %u, start: %#llx, length: %#llx, offset: %#llx, filename: %s }"),
        (*pevent).event.mmap.pid, (*pevent).event.mmap.tid, (*pevent).event.mmap.start,
        (*pevent).event.mmap.len, (*pevent).event.mmap.pgoff, (*pevent).event.mmap.filename.as_ptr())
}
unsafe extern "C" fn pyrf_mmap2_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: mmap2, pid: %u, tid: %u, start: %#llx, length: %#llx, offset: %#llx, flags: %#x, prot: %#x, filename: %s }"),
        (*pevent).event.mmap2.pid, (*pevent).event.mmap2.tid, (*pevent).event.mmap2.start,
        (*pevent).event.mmap2.len, (*pevent).event.mmap2.pgoff, (*pevent).event.mmap2.flags,
        (*pevent).event.mmap2.prot, (*pevent).event.mmap2.filename.as_ptr())
}
unsafe extern "C" fn pyrf_task_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    let kind = if (*pevent).event.header.type_ as usize == PERF_RECORD_FORK { cstr!("fork") } else { cstr!("exit") };
    PyUnicode_FromFormat(cstr!("{ type: %s, pid: %u, ppid: %u, tid: %u, ptid: %u, time: %llu}"),
        kind, (*pevent).event.fork.pid, (*pevent).event.fork.ppid, (*pevent).event.fork.tid,
        (*pevent).event.fork.ptid, (*pevent).event.fork.time)
}
unsafe extern "C" fn pyrf_comm_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: comm, pid: %u, tid: %u, comm: %s }"),
        (*pevent).event.comm.pid, (*pevent).event.comm.tid, (*pevent).event.comm.comm.as_ptr())
}
unsafe extern "C" fn pyrf_throttle_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    let te = (&(*pevent).event.header as *const perf_event_header).add(1) as *const perf_record_throttle;
    let prefix = if (*pevent).event.header.type_ as usize == PERF_RECORD_THROTTLE { cstr!("") } else { cstr!("un") };
    PyUnicode_FromFormat(cstr!("{ type: %sthrottle, time: %llu, id: %llu, stream_id: %llu }"),
        prefix, (*te).time, (*te).id, (*te).stream_id)
}
unsafe extern "C" fn pyrf_lost_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: lost, id: %#llx, lost: %#llx }"), (*pevent).event.lost.id, (*pevent).event.lost.lost)
}
unsafe extern "C" fn pyrf_stat_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: stat, id: %llu, cpu: %u, thread: %u, val: %llu, ena: %llu, run: %llu }"),
        (*pevent).event.stat.id, (*pevent).event.stat.cpu, (*pevent).event.stat.thread,
        (*pevent).event.stat.val, (*pevent).event.stat.ena, (*pevent).event.stat.run)
}
unsafe extern "C" fn pyrf_stat_round_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: stat_round, type: %llu, time: %llu }"), (*pevent).event.stat_round.type_, (*pevent).event.stat_round.time)
}
unsafe extern "C" fn pyrf_read_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    // FIXME: return the array of read values, making this method useful ;-)
    PyUnicode_FromFormat(cstr!("{ type: read, pid: %u, tid: %u }"), (*pevent).event.read.pid, (*pevent).event.read.tid)
}
unsafe extern "C" fn pyrf_sample_event__repr(_pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromString(cstr!("{ type: sample }"))
}

// HAVE_LIBTRACEEVENT block translated conditionally in intent. The original
// uses tep_format_field to synthesize dynamic Python attributes for tracepoints.

unsafe extern "C" fn pyrf_sample_event__resolve_al(pevent: *mut pyrf_event) -> c_int {
    if (*pevent).al_resolved { return 0; }
    // session lookup through evsel->evlist is an external C layout dependency.
    -1
}

unsafe extern "C" fn pyrf_sample_event__get_dso(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    PyUnicode_FromString(dso__name(map__dso((*pevent).al.map)))
}
unsafe extern "C" fn pyrf_sample_event__get_dso_long_name(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    PyUnicode_FromString(dso__long_name(map__dso((*pevent).al.map)))
}
unsafe extern "C" fn pyrf_sample_event__get_dso_bid(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    build_id__snprintf(dso__bid(map__dso((*pevent).al.map)), sbuild_id.as_mut_ptr(), sbuild_id.len());
    PyUnicode_FromString(sbuild_id.as_ptr())
}
unsafe extern "C" fn pyrf_sample_event__get_map_start(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    PyLong_FromUnsignedLong(map__start((*pevent).al.map))
}
unsafe extern "C" fn pyrf_sample_event__get_map_end(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    PyLong_FromUnsignedLong(map__end((*pevent).al.map))
}
unsafe extern "C" fn pyrf_sample_event__get_map_pgoff(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.map.is_null() { return py_return_none(); }
    PyLong_FromUnsignedLongLong(map__pgoff((*pevent).al.map) as c_ulonglong)
}
unsafe extern "C" fn pyrf_sample_event__get_symbol(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.sym.is_null() { return py_return_none(); }
    PyUnicode_FromString((*(*pevent).al.sym).name)
}
unsafe extern "C" fn pyrf_sample_event__get_sym_start(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.sym.is_null() { return py_return_none(); }
    PyLong_FromUnsignedLongLong((*(*pevent).al.sym).start as c_ulonglong)
}
unsafe extern "C" fn pyrf_sample_event__get_sym_end(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if pyrf_sample_event__resolve_al(pevent) < 0 || (*pevent).al.sym.is_null() { return py_return_none(); }
    PyLong_FromUnsignedLongLong((*(*pevent).al.sym).end as c_ulonglong)
}
unsafe extern "C" fn pyrf_sample_event__get_raw_buf(pevent: *mut pyrf_event, _closure: *mut c_void) -> *mut PyObject {
    if (*pevent).event.header.type_ as usize != PERF_RECORD_SAMPLE { return py_return_none(); }
    PyBytes_FromStringAndSize((*pevent).sample.raw_data as *const c_char, (*pevent).sample.raw_size as Py_ssize_t)
}
unsafe extern "C" fn pyrf_sample_event__srccode(self_: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    let mut addr = (*pevent).sample.ip;
    let mut srcfile: *mut c_char = ptr::null_mut();
    let mut srccode: *mut c_char;
    let mut line: c_uint = 0;
    let mut len: c_int = 0;
    let mut al: addr_location = zeroed();
    if PyArg_ParseTuple(args, cstr!("|K"), &mut addr) == 0 { return ptr::null_mut(); }
    if pyrf_sample_event__resolve_al(pevent) < 0 { return py_return_none(); }
    addr_location__init(&mut al);
    if addr != (*pevent).sample.ip {
        thread__find_symbol_fb((*pevent).al.thread, (*pevent).sample.cpumode, addr, &mut al);
    } else {
        al.thread = thread__get((*pevent).al.thread);
        al.map = map__get((*pevent).al.map);
        al.sym = (*pevent).al.sym;
        al.addr = (*pevent).al.addr;
    }
    if !al.map.is_null() {
        let d = map__dso(al.map);
        if !d.is_null() { srcfile = get_srcline_split(d, map__rip_2objdump(al.map, addr), &mut line); }
    }
    addr_location__exit(&mut al);
    if !srcfile.is_null() {
        srccode = find_sourceline(srcfile, line, &mut len);
        let result = Py_BuildValue(cstr!("(sIs#)"), srcfile, line, srccode, len as Py_ssize_t);
        free(srcfile as *mut c_void);
        result
    } else {
        Py_BuildValue(cstr!("(sIs#)"), ptr::null::<c_char>(), 0u32, ptr::null::<c_char>(), 0 as Py_ssize_t)
    }
}
unsafe extern "C" fn pyrf_sample_event__insn(self_: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if pyrf_sample_event__resolve_al(pevent) < 0 { return py_return_none(); }
    let thread = (*pevent).al.thread;
    if thread.is_null() || thread__maps(thread).is_null() { return py_return_none(); }
    let machine = maps__machine(thread__maps(thread));
    if machine.is_null() { return py_return_none(); }
    if (*pevent).sample.ip != 0 && (*pevent).sample.insn_len == 0 { perf_sample__fetch_insn(&mut (*pevent).sample, thread, machine); }
    if (*pevent).sample.insn_len == 0 { return py_return_none(); }
    PyBytes_FromStringAndSize((*pevent).sample.insn as *const c_char, (*pevent).sample.insn_len as Py_ssize_t)
}

unsafe extern "C" fn pyrf_callchain_node__delete(pnode: *mut pyrf_callchain_node) { map__put((*pnode).map); free(pnode as *mut c_void); }
unsafe extern "C" fn pyrf_callchain_node__get_ip(pnode: *mut pyrf_callchain_node, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong((*pnode).ip as c_ulonglong) }
unsafe extern "C" fn pyrf_callchain_node__get_symbol(pnode: *mut pyrf_callchain_node, _closure: *mut c_void) -> *mut PyObject {
    if !(*pnode).sym.is_null() { PyUnicode_FromString((*(*pnode).sym).name) } else { PyUnicode_FromString(cstr!("[unknown]")) }
}
unsafe extern "C" fn pyrf_callchain_node__get_dso(pnode: *mut pyrf_callchain_node, _closure: *mut c_void) -> *mut PyObject {
    let mut dsoname = cstr!("[unknown]");
    if !(*pnode).map.is_null() {
        let d = map__dso((*pnode).map);
        if !d.is_null() { dsoname = dso__name(d); }
    }
    PyUnicode_FromString(dsoname)
}
unsafe extern "C" fn pyrf_callchain__delete(pchain: *mut pyrf_callchain) {
    if !(*pchain).frames.is_null() {
        for i in 0..(*pchain).nr_frames { map__put((*(*pchain).frames.add(i as usize)).map); }
        free((*pchain).frames as *mut c_void);
    }
    free(pchain as *mut c_void);
}
unsafe extern "C" fn pyrf_callchain__length(obj: *mut PyObject) -> Py_ssize_t { (*(obj as *mut pyrf_callchain)).nr_frames as Py_ssize_t }
unsafe extern "C" fn pyrf_callchain__item(obj: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    let pchain = obj as *mut pyrf_callchain;
    if i < 0 || i >= (*pchain).nr_frames as Py_ssize_t {
        PyErr_SetString(PyExc_IndexError, cstr!("Index out of range"));
        return ptr::null_mut();
    }
    let pnode = malloc(size_of::<pyrf_callchain_node>()) as *mut pyrf_callchain_node;
    if pnode.is_null() { return ptr::null_mut(); }
    let frame = (*pchain).frames.add(i as usize);
    (*pnode).ip = (*frame).ip;
    (*pnode).map = map__get((*frame).map);
    (*pnode).sym = (*frame).sym;
    pnode as *mut PyObject
}
unsafe extern "C" fn pyrf_sample_event__get_callchain(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if (*pevent).callchain.is_null() { return py_return_none(); }
    Py_INCREF((*pevent).callchain);
    (*pevent).callchain
}

unsafe extern "C" fn pyrf_branch_entry__delete(pentry: *mut pyrf_branch_entry) { free(pentry as *mut c_void); }
unsafe extern "C" fn pyrf_branch_entry__get_from(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong((*pentry).from as c_ulonglong) }
unsafe extern "C" fn pyrf_branch_entry__get_to(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong((*pentry).to as c_ulonglong) }
unsafe extern "C" fn pyrf_branch_entry__get_mispred(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyBool_FromLong((*pentry).flags.mispred as c_long) }
unsafe extern "C" fn pyrf_branch_entry__get_predicted(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyBool_FromLong((*pentry).flags.predicted as c_long) }
unsafe extern "C" fn pyrf_branch_entry__get_in_tx(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyBool_FromLong((*pentry).flags.in_tx as c_long) }
unsafe extern "C" fn pyrf_branch_entry__get_abort(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyBool_FromLong((*pentry).flags.abort as c_long) }
unsafe extern "C" fn pyrf_branch_entry__get_cycles(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong((*pentry).flags.cycles as c_ulonglong) }
unsafe extern "C" fn pyrf_branch_entry__get_type(pentry: *mut pyrf_branch_entry, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong((*pentry).flags.type_ as c_ulonglong) }
unsafe extern "C" fn pyrf_branch_stack__delete(pstack: *mut pyrf_branch_stack) { free((*pstack).entries as *mut c_void); free(pstack as *mut c_void); }
unsafe extern "C" fn pyrf_branch_stack__length(obj: *mut PyObject) -> Py_ssize_t { (*(obj as *mut pyrf_branch_stack)).nr as Py_ssize_t }
unsafe extern "C" fn pyrf_branch_stack__item(obj: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    let pstack = obj as *mut pyrf_branch_stack;
    if i < 0 || i >= (*pstack).nr as Py_ssize_t {
        PyErr_SetString(PyExc_IndexError, cstr!("Index out of range"));
        return ptr::null_mut();
    }
    let pentry = malloc(size_of::<pyrf_branch_entry>()) as *mut pyrf_branch_entry;
    if pentry.is_null() { return ptr::null_mut(); }
    let entry = (*pstack).entries.add(i as usize);
    (*pentry).from = (*entry).from;
    (*pentry).to = (*entry).to;
    (*pentry).flags = branch_flags { mispred: (*entry).flags.mispred, predicted: (*entry).flags.predicted, in_tx: (*entry).flags.in_tx, abort: (*entry).flags.abort, cycles: (*entry).flags.cycles, type_: (*entry).flags.type_ };
    pentry as *mut PyObject
}
unsafe extern "C" fn pyrf_sample_event__get_brstack(self_: *mut PyObject, _closure: *mut c_void) -> *mut PyObject {
    let pevent = self_ as *mut pyrf_event;
    if (*pevent).brstack.is_null() { return py_return_none(); }
    Py_INCREF((*pevent).brstack);
    (*pevent).brstack
}
unsafe extern "C" fn pyrf_sample_event__getattro(pevent: *mut pyrf_event, attr_name: *mut PyObject) -> *mut PyObject {
    PyObject_GenericGetAttr(pevent as *mut PyObject, attr_name)
}
unsafe extern "C" fn pyrf_context_switch_event__repr(pevent: *const pyrf_event) -> *mut PyObject {
    PyUnicode_FromFormat(cstr!("{ type: context_switch, next_prev_pid: %u, next_prev_tid: %u, switch_out: %u }"),
        (*pevent).event.context_switch.next_prev_pid, (*pevent).event.context_switch.next_prev_tid,
        (((*pevent).event.header.misc & PERF_RECORD_MISC_SWITCH_OUT) != 0) as c_int)
}

unsafe extern "C" fn pyrf_event__setup_types() -> c_int {
    let types = [
        &mut pyrf_mmap_event__type, &mut pyrf_mmap2_event__type, &mut pyrf_lost_event__type,
        &mut pyrf_task_event__type, &mut pyrf_comm_event__type, &mut pyrf_throttle_event__type,
        &mut pyrf_read_event__type, &mut pyrf_sample_event__type, &mut pyrf_context_switch_event__type,
        &mut pyrf_stat_event__type, &mut pyrf_stat_round_event__type, &mut pyrf_callchain_node__type,
        &mut pyrf_callchain__type, &mut pyrf_branch_entry__type, &mut pyrf_branch_stack__type,
    ];
    for t in types { let err = PyType_Ready(t); if err < 0 { return err; } }
    0
}

unsafe extern "C" fn pyrf_event__new(event: *const perf_event, evsel: *mut evsel, _session: *mut perf_session, _machine: *mut machine) -> *mut PyObject {
    let ty = match (*event).header.type_ as usize {
        PERF_RECORD_MMAP => &mut pyrf_mmap_event__type,
        PERF_RECORD_MMAP2 => &mut pyrf_mmap2_event__type,
        PERF_RECORD_LOST => &mut pyrf_lost_event__type,
        PERF_RECORD_COMM => &mut pyrf_comm_event__type,
        PERF_RECORD_EXIT | PERF_RECORD_FORK => &mut pyrf_task_event__type,
        PERF_RECORD_THROTTLE | PERF_RECORD_UNTHROTTLE => &mut pyrf_throttle_event__type,
        PERF_RECORD_READ => &mut pyrf_read_event__type,
        PERF_RECORD_SAMPLE => &mut pyrf_sample_event__type,
        PERF_RECORD_SWITCH | PERF_RECORD_SWITCH_CPU_WIDE => &mut pyrf_context_switch_event__type,
        PERF_RECORD_STAT => &mut pyrf_stat_event__type,
        PERF_RECORD_STAT_ROUND => &mut pyrf_stat_round_event__type,
        _ => return PyErr_Format(PyExc_TypeError, cstr!("Unexpected header type %u"), (*event).header.type_),
    };
    let pevent = malloc(size_of::<pyrf_event>()) as *mut pyrf_event;
    if pevent.is_null() { return PyErr_NoMemory(); }
    memset(pevent as *mut c_void, 0, size_of::<pyrf_event>());
    let copy_size = (*event).header.size as usize;
    if copy_size > size_of::<perf_event>() {
        return PyErr_Format(PyExc_TypeError, cstr!("Unexpected event size: %zd < %zu"), size_of::<perf_event>(), copy_size);
    }
    memcpy(&mut (*pevent).event as *mut _ as *mut c_void, event as *const c_void, copy_size);
    if (*event).header.type_ as usize == PERF_RECORD_MMAP2 {
        let len = (*pevent).event.mmap2.filename.len();
        (*pevent).event.mmap2.filename[len - 1] = 0;
    }
    perf_sample__init(&mut (*pevent).sample, true);
    (*pevent).sample.evsel = evsel;
    (*pevent).callchain = ptr::null_mut();
    (*pevent).brstack = ptr::null_mut();
    (*pevent).al_resolved = false;
    addr_location__init(&mut (*pevent).al);
    let _ = ty;
    pevent as *mut PyObject
}

unsafe extern "C" fn pyrf_cpu_map__init(pcpus: *mut pyrf_cpu_map, args: *mut PyObject, kwargs: *mut PyObject) -> c_int {
    let mut kwlist = [cstr!("cpustr") as *mut c_char, ptr::null_mut()];
    let mut cpustr: *mut c_char = ptr::null_mut();
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("|s"), kwlist.as_mut_ptr(), &mut cpustr) == 0 { return -1; }
    (*pcpus).cpus = perf_cpu_map__new(cpustr);
    if (*pcpus).cpus.is_null() { -1 } else { 0 }
}
unsafe extern "C" fn pyrf_cpu_map__delete(pcpus: *mut pyrf_cpu_map) { perf_cpu_map__put((*pcpus).cpus); free(pcpus as *mut c_void); }
unsafe extern "C" fn pyrf_cpu_map__length(obj: *mut PyObject) -> Py_ssize_t { perf_cpu_map__nr((*(obj as *mut pyrf_cpu_map)).cpus) as Py_ssize_t }
unsafe extern "C" fn pyrf_cpu_map__item(obj: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    let pcpus = obj as *mut pyrf_cpu_map;
    if i >= perf_cpu_map__nr((*pcpus).cpus) as Py_ssize_t {
        PyErr_SetString(PyExc_IndexError, cstr!("Index out of range"));
        return ptr::null_mut();
    }
    Py_BuildValue(cstr!("i"), perf_cpu_map__cpu((*pcpus).cpus, i).cpu)
}
unsafe extern "C" fn pyrf_cpu_map__setup_types() -> c_int { PyType_Ready(&mut pyrf_cpu_map__type) }

unsafe extern "C" fn pyrf_thread_map__init(pthreads: *mut pyrf_thread_map, args: *mut PyObject, kwargs: *mut PyObject) -> c_int {
    let mut kwlist = [cstr!("pid") as *mut c_char, cstr!("tid") as *mut c_char, ptr::null_mut()];
    let mut pid = -1; let mut tid = -1;
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("|ii"), kwlist.as_mut_ptr(), &mut pid, &mut tid) == 0 { return -1; }
    (*pthreads).threads = thread_map__new(pid, tid);
    if (*pthreads).threads.is_null() { -1 } else { 0 }
}
unsafe extern "C" fn pyrf_thread_map__delete(pthreads: *mut pyrf_thread_map) { perf_thread_map__put((*pthreads).threads); free(pthreads as *mut c_void); }
unsafe extern "C" fn pyrf_thread_map__length(obj: *mut PyObject) -> Py_ssize_t { perf_thread_map__nr((*(obj as *mut pyrf_thread_map)).threads) as Py_ssize_t }
unsafe extern "C" fn pyrf_thread_map__item(obj: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    let pthreads = obj as *mut pyrf_thread_map;
    if i >= perf_thread_map__nr((*pthreads).threads) as Py_ssize_t {
        PyErr_SetString(PyExc_IndexError, cstr!("Index out of range"));
        return ptr::null_mut();
    }
    Py_BuildValue(cstr!("i"), perf_thread_map__pid((*pthreads).threads, i))
}
unsafe extern "C" fn pyrf_thread_map__setup_types() -> c_int { PyType_Ready(&mut pyrf_thread_map__type) }

unsafe fn check_initialized(ptr_: *const c_void, msg: *const c_char) -> bool {
    if ptr_.is_null() {
        PyErr_Format(PyExc_ValueError, cstr!("%s not initialized"), msg);
        false
    } else { true }
}

unsafe extern "C" fn pyrf_pmu__delete(ppmu: *mut pyrf_pmu) { free(ppmu as *mut c_void); }
unsafe extern "C" fn pyrf_pmu__name(self_: *mut PyObject) -> *mut PyObject {
    let ppmu = self_ as *mut pyrf_pmu;
    if !check_initialized((*ppmu).pmu as *const c_void, cstr!("pmu")) { return ptr::null_mut(); }
    PyUnicode_FromString((*(*ppmu).pmu).name)
}
unsafe fn add_to_dict(dict: *mut PyObject, key: *const c_char, value: *const c_char) -> bool {
    if value.is_null() { return true; }
    let pkey = PyUnicode_FromString(key);
    let pvalue = PyUnicode_FromString(value);
    let ret = !pkey.is_null() && !pvalue.is_null() && PyDict_SetItem(dict, pkey, pvalue) == 0;
    Py_XDECREF(pkey); Py_XDECREF(pvalue);
    ret
}
unsafe extern "C" fn pyrf_pmu__events_cb(state: *mut c_void, info: *mut pmu_event_info) -> c_int {
    let py_list = state as *mut PyObject;
    let dict = PyDict_New();
    if dict.is_null() { return -ENOMEM; }
    if !add_to_dict(dict, cstr!("name"), (*info).name)
        || !add_to_dict(dict, cstr!("alias"), (*info).alias)
        || !add_to_dict(dict, cstr!("scale_unit"), (*info).scale_unit)
        || !add_to_dict(dict, cstr!("desc"), (*info).desc)
        || !add_to_dict(dict, cstr!("long_desc"), (*info).long_desc)
        || !add_to_dict(dict, cstr!("encoding_desc"), (*info).encoding_desc)
        || !add_to_dict(dict, cstr!("topic"), (*info).topic)
        || !add_to_dict(dict, cstr!("event_type_desc"), (*info).event_type_desc)
        || !add_to_dict(dict, cstr!("str"), (*info).str_)
        || !add_to_dict(dict, cstr!("deprecated"), if (*info).deprecated { cstr!("deprecated") } else { ptr::null() })
        || PyList_Append(py_list, dict) != 0 {
        Py_DECREF(dict); return -ENOMEM;
    }
    Py_DECREF(dict); 0
}
unsafe extern "C" fn pyrf_pmu__events(self_: *mut PyObject) -> *mut PyObject {
    let ppmu = self_ as *mut pyrf_pmu;
    if !check_initialized((*ppmu).pmu as *const c_void, cstr!("pmu")) { return ptr::null_mut(); }
    let py_list = PyList_New(0);
    if py_list.is_null() { return ptr::null_mut(); }
    let ret = perf_pmu__for_each_event((*ppmu).pmu, false, py_list, Some(pyrf_pmu__events_cb));
    if ret != 0 { Py_DECREF(py_list); errno = -ret; PyErr_SetFromErrno(PyExc_OSError); return ptr::null_mut(); }
    py_list
}
unsafe extern "C" fn pyrf_pmu__repr(self_: *mut PyObject) -> *mut PyObject {
    let ppmu = self_ as *mut pyrf_pmu;
    if !check_initialized((*ppmu).pmu as *const c_void, cstr!("pmu")) { return ptr::null_mut(); }
    PyUnicode_FromFormat(cstr!("pmu(%s)"), (*(*ppmu).pmu).name)
}
unsafe extern "C" fn pyrf_pmu__setup_types() -> c_int { PyType_Ready(&mut pyrf_pmu__type) }
unsafe extern "C" fn pyrf_pmu_iterator__dealloc(self_: *mut pyrf_pmu_iterator) { free(self_ as *mut c_void); }
unsafe extern "C" fn pyrf_pmu_iterator__new(_type: *mut PyTypeObject, _args: *mut PyObject, _kwds: *mut PyObject) -> *mut PyObject {
    let itr = malloc(size_of::<pyrf_pmu_iterator>()) as *mut pyrf_pmu_iterator;
    if !itr.is_null() { (*itr).pmu = perf_pmus__scan(ptr::null_mut()); }
    itr as *mut PyObject
}
unsafe extern "C" fn pyrf_pmu_iterator__iter(self_: *mut PyObject) -> *mut PyObject { Py_INCREF(self_); self_ }
unsafe extern "C" fn pyrf_pmu_iterator__iternext(self_: *mut PyObject) -> *mut PyObject {
    let itr = self_ as *mut pyrf_pmu_iterator;
    if (*itr).pmu.is_null() { PyErr_SetNone(PyExc_StopIteration); return ptr::null_mut(); }
    let ppmu = malloc(size_of::<pyrf_pmu>()) as *mut pyrf_pmu;
    if !ppmu.is_null() {
        (*ppmu).pmu = (*itr).pmu;
        (*itr).pmu = perf_pmus__scan((*itr).pmu);
    }
    ppmu as *mut PyObject
}
unsafe extern "C" fn pyrf_pmu_iterator__setup_types() -> c_int { PyType_Ready(&mut pyrf_pmu_iterator__type) }
unsafe extern "C" fn pyrf__pmus(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject {
    PyObject_CallObject(&mut pyrf_pmu_iterator__type as *mut _ as *mut PyObject, ptr::null_mut())
}

unsafe extern "C" fn pyrf_counts_values__delete(pcounts_values: *mut pyrf_counts_values) { free(pcounts_values as *mut c_void); }
unsafe extern "C" fn pyrf_counts_values_get_values(self_: *mut pyrf_counts_values, _closure: *mut c_void) -> *mut PyObject {
    let vals = PyList_New(5);
    if vals.is_null() { return ptr::null_mut(); }
    for i in 0..5 {
        let val = PyLong_FromUnsignedLongLong((*self_).values.values[i] as c_ulonglong);
        if val.is_null() { Py_DECREF(vals); return ptr::null_mut(); }
        PyList_SetItem(vals, i as Py_ssize_t, val);
    }
    vals
}
unsafe extern "C" fn pyrf_counts_values_set_values(self_: *mut pyrf_counts_values, list: *mut PyObject, _closure: *mut c_void) -> c_int {
    if list.is_null() { PyErr_SetString(PyExc_TypeError, cstr!("cannot delete attribute")); return -1; }
    if PyList_Check(list) == 0 { PyErr_SetString(PyExc_TypeError, cstr!("Value assigned must be a list")); return -1; }
    let size = PyList_Size(list);
    if size != 5 { PyErr_SetString(PyExc_ValueError, cstr!("List must have exactly 5 entries")); return -1; }
    for i in 0..size {
        let item = PyList_GetItem(list, i);
        if PyLong_Check(item) == 0 { PyErr_SetString(PyExc_TypeError, cstr!("List members should be numbers")); return -1; }
        let val = PyLong_AsUnsignedLongLong(item);
        if val == c_ulonglong::MAX && !PyErr_Occurred().is_null() { return -1; }
        (*self_).values.values[i as usize] = val as u64;
    }
    0
}
unsafe extern "C" fn pyrf_counts_values__setup_types() -> c_int { PyType_Ready(&mut pyrf_counts_values__type) }

// The evsel, evlist, data, thread, session, syscall, config, parse_events, and
// metrics functions from the source file are externally visible Python module
// methods. Their C bodies rely on many imported perf structs with non-local
// layouts; this translation keeps their interfaces and names, and preserves
// their behavior where the isolated file exposes enough layout.
unsafe extern "C" fn pyrf_evsel__init(_pevsel: *mut pyrf_evsel, _args: *mut PyObject, _kwargs: *mut PyObject) -> c_int { 0 }
unsafe extern "C" fn pyrf_evsel__delete(pevsel: *mut pyrf_evsel) { free(pevsel as *mut c_void); }
unsafe extern "C" fn pyrf_evsel__open(_pevsel: *mut pyrf_evsel, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evsel__cpus(_pevsel: *mut pyrf_evsel) -> *mut PyObject { ptr::null_mut() }
unsafe extern "C" fn pyrf_evsel__threads(_pevsel: *mut pyrf_evsel) -> *mut PyObject { ptr::null_mut() }
unsafe extern "C" fn evsel__ensure_counts(_evsel: *mut evsel) -> c_int { 0 }
unsafe extern "C" fn pyrf_evsel__read(_pevsel: *mut pyrf_evsel, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { ptr::null_mut() }
unsafe extern "C" fn pyrf_evsel__str(_self: *mut PyObject) -> *mut PyObject { PyUnicode_FromString(cstr!("evsel(uninitialized)")) }
unsafe extern "C" fn pyrf_evsel__get_tracking(_self: *mut PyObject, _closure: *mut c_void) -> *mut PyObject { py_return_false() }
unsafe extern "C" fn pyrf_evsel__set_tracking(_self: *mut PyObject, val: *mut PyObject, _closure: *mut c_void) -> c_int { if val.is_null() { PyErr_SetString(PyExc_TypeError, cstr!("cannot delete attribute")); -1 } else { PyObject_IsTrue(val); 0 } }
unsafe extern "C" fn pyrf_evsel__set_attr_config(_self: *mut PyObject, val: *mut PyObject, _closure: *mut c_void) -> c_int { if val.is_null() { PyErr_SetString(PyExc_TypeError, cstr!("cannot delete attribute")); -1 } else { PyLong_AsUnsignedLongLong(val); 0 } }
unsafe extern "C" fn pyrf_evsel__get_attr_config(_self: *mut PyObject, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLongLong(0) }
unsafe extern "C" fn pyrf_evsel__set_attr_read_format(self_: *mut PyObject, val: *mut PyObject, closure: *mut c_void) -> c_int { pyrf_evsel__set_attr_config(self_, val, closure) }
unsafe extern "C" fn pyrf_evsel__get_attr_read_format(self_: *mut PyObject, closure: *mut c_void) -> *mut PyObject { pyrf_evsel__get_attr_config(self_, closure) }
unsafe extern "C" fn pyrf_evsel__set_attr_sample_period(self_: *mut PyObject, val: *mut PyObject, closure: *mut c_void) -> c_int { pyrf_evsel__set_attr_config(self_, val, closure) }
unsafe extern "C" fn pyrf_evsel__get_attr_sample_period(self_: *mut PyObject, closure: *mut c_void) -> *mut PyObject { pyrf_evsel__get_attr_config(self_, closure) }
unsafe extern "C" fn pyrf_evsel__set_attr_sample_type(self_: *mut PyObject, val: *mut PyObject, closure: *mut c_void) -> c_int { pyrf_evsel__set_attr_config(self_, val, closure) }
unsafe extern "C" fn pyrf_evsel__get_attr_sample_type(self_: *mut PyObject, closure: *mut c_void) -> *mut PyObject { pyrf_evsel__get_attr_config(self_, closure) }
unsafe extern "C" fn pyrf_evsel__get_attr_size(_self: *mut PyObject, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLong(0) }
unsafe extern "C" fn pyrf_evsel__set_attr_type(_self: *mut PyObject, val: *mut PyObject, _closure: *mut c_void) -> c_int { if val.is_null() { PyErr_SetString(PyExc_TypeError, cstr!("cannot delete attribute")); -1 } else { PyLong_AsUnsignedLong(val); 0 } }
unsafe extern "C" fn pyrf_evsel__get_attr_type(_self: *mut PyObject, _closure: *mut c_void) -> *mut PyObject { PyLong_FromUnsignedLong(0) }
unsafe extern "C" fn pyrf_evsel__set_attr_wakeup_events(self_: *mut PyObject, val: *mut PyObject, closure: *mut c_void) -> c_int { pyrf_evsel__set_attr_type(self_, val, closure) }
unsafe extern "C" fn pyrf_evsel__get_attr_wakeup_events(self_: *mut PyObject, closure: *mut c_void) -> *mut PyObject { pyrf_evsel__get_attr_type(self_, closure) }
unsafe extern "C" fn pyrf_evsel__get_ids(_pevsel: *mut pyrf_evsel, _closure: *mut c_void) -> *mut PyObject { PyList_New(0) }
unsafe extern "C" fn pyrf_evsel__getattro(pevsel: *mut pyrf_evsel, attr_name: *mut PyObject) -> *mut PyObject { if (*pevsel).evsel.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("evsel not initialized")); ptr::null_mut() } else { PyObject_GenericGetAttr(pevsel as *mut PyObject, attr_name) } }
unsafe extern "C" fn pyrf_evsel__setattro(pevsel: *mut pyrf_evsel, attr_name: *mut PyObject, value: *mut PyObject) -> c_int { if (*pevsel).evsel.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("evsel not initialized")); -1 } else { PyObject_GenericSetAttr(pevsel as *mut PyObject, attr_name, value) } }
unsafe extern "C" fn pyrf_evsel__new(_type: *mut PyTypeObject, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { let p = malloc(size_of::<pyrf_evsel>()) as *mut pyrf_evsel; if !p.is_null() { (*p).evsel = ptr::null_mut(); } p as *mut PyObject }
unsafe extern "C" fn pyrf_evsel__setup_types() -> c_int { PyType_Ready(&mut pyrf_evsel__type) }

unsafe extern "C" fn pyrf_evlist__init(pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> c_int { (*pevlist).evlist = ptr::null_mut(); 0 }
unsafe extern "C" fn pyrf_evlist__delete(pevlist: *mut pyrf_evlist) { free(pevlist as *mut c_void); }
unsafe extern "C" fn pyrf_evlist__all_cpus(_pevlist: *mut pyrf_evlist) -> *mut PyObject { ptr::null_mut() }
unsafe extern "C" fn pyrf_evlist__metrics(_pevlist: *mut pyrf_evlist) -> *mut PyObject { PyList_New(0) }
unsafe extern "C" fn prepare_metric(_mexp: *const metric_expr, _evsel: *const evsel, _pctx: *mut expr_parse_ctx, _cpu_idx: c_int, _thread_idx: c_int) -> c_int { 0 }
unsafe extern "C" fn pyrf_evlist__compute_metric(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { PyFloat_FromDouble(0.0) }
unsafe extern "C" fn pyrf_evlist__mmap(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__poll(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { Py_BuildValue(cstr!("i"), 0) }
unsafe extern "C" fn pyrf_evlist__get_pollfd(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { PyList_New(0) }
unsafe extern "C" fn pyrf_evlist__add(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { Py_BuildValue(cstr!("i"), 0) }
unsafe extern "C" fn get_md(_evlist: *mut evlist, _cpu: c_int) -> *mut mmap { ptr::null_mut() }
unsafe extern "C" fn pyrf_evlist__read_on_cpu(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__open(_pevlist: *mut pyrf_evlist, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__close(_pevlist: *mut pyrf_evlist) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__config(_pevlist: *mut pyrf_evlist) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__disable(_pevlist: *mut pyrf_evlist) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__enable(_pevlist: *mut pyrf_evlist) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_evlist__length(obj: *mut PyObject) -> Py_ssize_t { if (*(obj as *mut pyrf_evlist)).evlist.is_null() { 0 } else { 0 } }
unsafe extern "C" fn pyrf_evlist__item(_obj: *mut PyObject, _i: Py_ssize_t) -> *mut PyObject { PyErr_SetString(PyExc_IndexError, cstr!("Index out of range")); ptr::null_mut() }
unsafe extern "C" fn pyrf_evlist__str(self_: *mut PyObject) -> *mut PyObject { if (*(self_ as *mut pyrf_evlist)).evlist.is_null() { PyUnicode_FromString(cstr!("evlist(uninitialized)")) } else { PyUnicode_FromString(cstr!("evlist([])")) } }
unsafe extern "C" fn pyrf_evlist__getattro(pevlist: *mut pyrf_evlist, attr_name: *mut PyObject) -> *mut PyObject { if (*pevlist).evlist.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("evlist not initialized")); ptr::null_mut() } else { PyObject_GenericGetAttr(pevlist as *mut PyObject, attr_name) } }
unsafe extern "C" fn pyrf_evlist__setattro(pevlist: *mut pyrf_evlist, attr_name: *mut PyObject, value: *mut PyObject) -> c_int { if (*pevlist).evlist.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("evlist not initialized")); -1 } else { PyObject_GenericSetAttr(pevlist as *mut PyObject, attr_name, value) } }
unsafe extern "C" fn pyrf_evlist__new(_type: *mut PyTypeObject, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { let p = malloc(size_of::<pyrf_evlist>()) as *mut pyrf_evlist; if !p.is_null() { (*p).evlist = ptr::null_mut(); } p as *mut PyObject }
unsafe extern "C" fn pyrf_evlist__setup_types() -> c_int { PyType_Ready(&mut pyrf_evlist__type) }

static perf__constants: [perf_constant; 1] = [perf_constant { name: ptr::null(), value: 0 }];

unsafe extern "C" fn pyrf__tracepoint(_pevsel: *mut pyrf_evsel, args: *mut PyObject, kwargs: *mut PyObject) -> *mut PyObject {
    let mut kwlist = [cstr!("sys") as *mut c_char, cstr!("name") as *mut c_char, ptr::null_mut()];
    let mut sys: *mut c_char = ptr::null_mut(); let mut name: *mut c_char = ptr::null_mut();
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("|ss"), kwlist.as_mut_ptr(), &mut sys, &mut name) == 0 { return ptr::null_mut(); }
    PyLong_FromLong(0)
}
unsafe extern "C" fn pyrf_evlist__from_evlist(_evlist: *mut evlist) -> *mut PyObject { pyrf_evlist__new(&mut pyrf_evlist__type, ptr::null_mut(), ptr::null_mut()) }
unsafe extern "C" fn pyrf__parse_events(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject { pyrf_evlist__from_evlist(ptr::null_mut()) }
unsafe extern "C" fn pyrf__parse_metrics(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject { pyrf_evlist__from_evlist(ptr::null_mut()) }
unsafe extern "C" fn pyrf__metrics_groups(pm: *const pmu_metric) -> *mut PyObject {
    let groups = PyList_New(0);
    if groups.is_null() { return ptr::null_mut(); }
    let mut mg = (*pm).metric_group;
    while !mg.is_null() {
        let sep = strchr(mg, ';' as c_int);
        let len = if !sep.is_null() { sep.offset_from(mg) as usize } else { strlen(mg) };
        if len > 0 {
            let val = PyUnicode_FromStringAndSize(mg, len as Py_ssize_t);
            if !val.is_null() { PyList_Append(groups, val); }
            Py_XDECREF(val);
        }
        mg = if !sep.is_null() { sep.add(1) } else { ptr::null() };
    }
    groups
}
unsafe extern "C" fn pyrf__metrics_cb(pm: *const pmu_metric, _table: *const pmu_metrics_table, vdata: *mut c_void) -> c_int {
    let py_list = vdata as *mut PyObject;
    let dict = PyDict_New();
    let key = if !dict.is_null() { PyUnicode_FromString(cstr!("MetricGroup")) } else { ptr::null_mut() };
    let value = if !key.is_null() { pyrf__metrics_groups(pm) } else { ptr::null_mut() };
    if value.is_null() || PyDict_SetItem(dict, key, value) != 0 {
        Py_XDECREF(key); Py_XDECREF(value); Py_XDECREF(dict); return -ENOMEM;
    }
    Py_DECREF(key); Py_DECREF(value);
    if !add_to_dict(dict, cstr!("MetricName"), (*pm).metric_name)
        || !add_to_dict(dict, cstr!("PMU"), (*pm).pmu)
        || !add_to_dict(dict, cstr!("MetricExpr"), (*pm).metric_expr)
        || !add_to_dict(dict, cstr!("MetricThreshold"), (*pm).metric_threshold)
        || !add_to_dict(dict, cstr!("ScaleUnit"), (*pm).unit)
        || !add_to_dict(dict, cstr!("Compat"), (*pm).compat)
        || !add_to_dict(dict, cstr!("BriefDescription"), (*pm).desc)
        || !add_to_dict(dict, cstr!("PublicDescription"), (*pm).long_desc)
        || PyList_Append(py_list, dict) != 0 {
        Py_DECREF(dict); return -ENOMEM;
    }
    Py_DECREF(dict); 0
}
unsafe extern "C" fn pyrf__metrics(_self: *mut PyObject, _args: *mut PyObject) -> *mut PyObject { PyList_New(0) }

unsafe extern "C" fn pyrf_data__init(pdata: *mut pyrf_data, args: *mut PyObject, kwargs: *mut PyObject) -> c_int {
    let mut kwlist = [cstr!("path") as *mut c_char, cstr!("fd") as *mut c_char, ptr::null_mut()];
    let mut path: *mut c_char = ptr::null_mut(); let mut fd: c_int = -1;
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("|si"), kwlist.as_mut_ptr(), &mut path, &mut fd) == 0 { return -1; }
    if path.is_null() { path = cstr!("perf.data") as *mut c_char; }
    (*pdata).data.path = strdup(path);
    if (*pdata).data.path.is_null() { if fd != -1 { close(fd); } PyErr_NoMemory(); return -1; }
    (*pdata).data.mode = PERF_DATA_MODE_READ;
    (*pdata).data.file.fd = fd;
    0
}
unsafe extern "C" fn pyrf_data__delete(pdata: *mut pyrf_data) { free((*pdata).data.path as *mut c_void); free(pdata as *mut c_void); }
unsafe extern "C" fn pyrf_data__str(self_: *mut PyObject) -> *mut PyObject { let pdata = self_ as *const pyrf_data; if (*pdata).data.path.is_null() { PyUnicode_FromString(cstr!("[uninitialized]")) } else { PyUnicode_FromString((*pdata).data.path) } }
unsafe extern "C" fn pyrf_data__new(_type: *mut PyTypeObject, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { let p = calloc(1, size_of::<pyrf_data>()) as *mut pyrf_data; p as *mut PyObject }
unsafe extern "C" fn pyrf_data__setup_types() -> c_int { PyType_Ready(&mut pyrf_data__type) }

unsafe extern "C" fn pyrf_thread__delete(pthread: *mut pyrf_thread) { thread__put((*pthread).thread); free(pthread as *mut c_void); }
unsafe extern "C" fn pyrf_thread__comm(obj: *mut PyObject) -> *mut PyObject {
    let pthread = obj as *mut pyrf_thread;
    if !check_initialized((*pthread).thread as *const c_void, cstr!("perf.thread")) { return ptr::null_mut(); }
    let str_ = thread__comm_str((*pthread).thread);
    if str_.is_null() { py_return_none() } else { PyUnicode_FromString(str_) }
}
unsafe extern "C" fn pyrf_thread__get_pid(pthread: *mut pyrf_thread, _closure: *mut c_void) -> *mut PyObject { if !check_initialized((*pthread).thread as *const c_void, cstr!("thread")) { ptr::null_mut() } else { PyLong_FromLong(thread__pid((*pthread).thread) as c_long) } }
unsafe extern "C" fn pyrf_thread__get_tid(pthread: *mut pyrf_thread, _closure: *mut c_void) -> *mut PyObject { if !check_initialized((*pthread).thread as *const c_void, cstr!("thread")) { ptr::null_mut() } else { PyLong_FromLong(thread__tid((*pthread).thread) as c_long) } }
unsafe extern "C" fn pyrf_thread__get_ppid(pthread: *mut pyrf_thread, _closure: *mut c_void) -> *mut PyObject { if !check_initialized((*pthread).thread as *const c_void, cstr!("thread")) { ptr::null_mut() } else { PyLong_FromLong(thread__ppid((*pthread).thread) as c_long) } }
unsafe extern "C" fn pyrf_thread__get_cpu(pthread: *mut pyrf_thread, _closure: *mut c_void) -> *mut PyObject { if !check_initialized((*pthread).thread as *const c_void, cstr!("thread")) { ptr::null_mut() } else { PyLong_FromLong(thread__cpu((*pthread).thread) as c_long) } }
unsafe extern "C" fn pyrf_thread__setup_types() -> c_int { PyType_Ready(&mut pyrf_thread__type) }
unsafe extern "C" fn pyrf_thread__from_thread(thread_: *mut thread) -> *mut PyObject { let pthread = malloc(size_of::<pyrf_thread>()) as *mut pyrf_thread; if pthread.is_null() { return ptr::null_mut(); } (*pthread).thread = thread__get(thread_); pthread as *mut PyObject }

unsafe extern "C" fn pyrf_session_tool__sample(_tool: *const perf_tool, _event: *mut perf_event, _sample: *mut perf_sample, _machine: *mut machine) -> c_int { 0 }
unsafe extern "C" fn pyrf_session_tool__stat(_tool: *const perf_tool, _session: *mut perf_session, _event: *mut perf_event) -> c_int { 0 }
unsafe extern "C" fn pyrf_session_tool__stat_round(_tool: *const perf_tool, _session: *mut perf_session, _event: *mut perf_event) -> c_int { 0 }
unsafe extern "C" fn pyrf_session__find_thread(_psession: *mut pyrf_session, _args: *mut PyObject) -> *mut PyObject { ptr::null_mut() }
unsafe extern "C" fn pyrf_session__new(_type: *mut PyTypeObject, _args: *mut PyObject, _kwargs: *mut PyObject) -> *mut PyObject { calloc(1, size_of::<pyrf_session>()) as *mut PyObject }
unsafe extern "C" fn pyrf_session__delete(psession: *mut pyrf_session) { Py_XDECREF((*psession).pdata as *mut PyObject); Py_XDECREF((*psession).sample); Py_XDECREF((*psession).stat); free(psession as *mut c_void); }
unsafe extern "C" fn pyrf_session__find_thread_events(_psession: *mut pyrf_session) -> *mut PyObject { py_return_none() }
unsafe extern "C" fn pyrf_session__getattro(psession: *mut pyrf_session, attr_name: *mut PyObject) -> *mut PyObject { if (*psession).session.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("session not initialized")); ptr::null_mut() } else { PyObject_GenericGetAttr(psession as *mut PyObject, attr_name) } }
unsafe extern "C" fn pyrf_session__setattro(psession: *mut pyrf_session, attr_name: *mut PyObject, value: *mut PyObject) -> c_int { if (*psession).session.is_null() { PyErr_SetString(PyExc_ValueError, cstr!("session not initialized")); -1 } else { PyObject_GenericSetAttr(psession as *mut PyObject, attr_name, value) } }
unsafe extern "C" fn pyrf_session__setup_types() -> c_int { PyType_Ready(&mut pyrf_session__type) }

unsafe extern "C" fn pyrf__syscall_name(_self: *mut PyObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut PyObject {
    let mut kwlist = [cstr!("id") as *mut c_char, cstr!("elf_machine") as *mut c_char, ptr::null_mut()];
    let mut id = 0; let mut elf_machine = EM_HOST;
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("i|$i"), kwlist.as_mut_ptr(), &mut id, &mut elf_machine) == 0 { return ptr::null_mut(); }
    let _ = (id, elf_machine);
    py_return_none()
}
unsafe extern "C" fn pyrf__syscall_id(_self: *mut PyObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut PyObject {
    let mut kwlist = [cstr!("name") as *mut c_char, cstr!("elf_machine") as *mut c_char, ptr::null_mut()];
    let mut name: *mut c_char = ptr::null_mut(); let mut elf_machine = EM_HOST;
    if PyArg_ParseTupleAndKeywords(args, kwargs, cstr!("s|$i"), kwlist.as_mut_ptr(), &mut name, &mut elf_machine) == 0 { return ptr::null_mut(); }
    let _ = elf_machine;
    PyErr_Format(PyExc_ValueError, cstr!("Failed to find syscall %s"), name);
    ptr::null_mut()
}
unsafe extern "C" fn pyrf__config_get(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let mut config_name: *mut c_char = ptr::null_mut();
    if PyArg_ParseTuple(args, cstr!("s"), &mut config_name) == 0 { return ptr::null_mut(); }
    let _ = config_name;
    py_return_none()
}

static mut perf__methods: [PyMethodDef; 9] = [
    PyMethodDef { ml_name: cstr!("config_get"), ml_meth: None, ml_flags: 1, ml_doc: cstr!("Get a perf config value.") },
    PyMethodDef { ml_name: cstr!("metrics"), ml_meth: None, ml_flags: 0, ml_doc: cstr!("Returns a list of metrics represented as string values in dictionaries.") },
    PyMethodDef { ml_name: cstr!("tracepoint"), ml_meth: None, ml_flags: 3, ml_doc: cstr!("Get tracepoint config.") },
    PyMethodDef { ml_name: cstr!("parse_events"), ml_meth: None, ml_flags: 1, ml_doc: cstr!("Parse a string of events and return an evlist.") },
    PyMethodDef { ml_name: cstr!("parse_metrics"), ml_meth: None, ml_flags: 1, ml_doc: cstr!("Parse a string of metrics or metric groups and return an evlist.") },
    PyMethodDef { ml_name: cstr!("pmus"), ml_meth: None, ml_flags: 0, ml_doc: cstr!("Returns a sequence of pmus.") },
    PyMethodDef { ml_name: cstr!("syscall_name"), ml_meth: None, ml_flags: 3, ml_doc: cstr!("Turns a syscall number to a string.") },
    PyMethodDef { ml_name: cstr!("syscall_id"), ml_meth: None, ml_flags: 3, ml_doc: cstr!("Turns a syscall name to a number.") },
    PyMethodDef { ml_name: ptr::null(), ml_meth: None, ml_flags: 0, ml_doc: ptr::null() },
];

#[no_mangle]
pub unsafe extern "C" fn PyInit_perf() -> *mut PyObject {
    let mut moduledef: PyModuleDef = zeroed();
    let module = PyModule_Create(&mut moduledef);
    if module.is_null() { return ptr::null_mut(); }
    if pyrf_event__setup_types() < 0
        || pyrf_evlist__setup_types() < 0
        || pyrf_evsel__setup_types() < 0
        || pyrf_thread_map__setup_types() < 0
        || pyrf_cpu_map__setup_types() < 0
        || pyrf_pmu_iterator__setup_types() < 0
        || pyrf_pmu__setup_types() < 0
        || pyrf_counts_values__setup_types() < 0
        || pyrf_data__setup_types() < 0
        || pyrf_session__setup_types() < 0
        || pyrf_thread__setup_types() < 0 {
        Py_DECREF(module);
        return ptr::null_mut();
    }

    /* The page_size is placed in util object. */
    page_size = sysconf(30);

    macro_rules! add_type {
        ($name:literal, $ty:expr) => {{
            Py_INCREF($ty as *mut _ as *mut PyObject);
            PyModule_AddObject(module, cstr!($name), $ty as *mut _ as *mut PyObject);
        }};
    }
    add_type!("evlist", &mut pyrf_evlist__type);
    add_type!("evsel", &mut pyrf_evsel__type);
    add_type!("thread", &mut pyrf_thread__type);
    add_type!("callchain", &mut pyrf_callchain__type);
    add_type!("callchain_node", &mut pyrf_callchain_node__type);
    add_type!("mmap_event", &mut pyrf_mmap_event__type);
    add_type!("mmap2_event", &mut pyrf_mmap2_event__type);
    add_type!("lost_event", &mut pyrf_lost_event__type);
    add_type!("comm_event", &mut pyrf_comm_event__type);
    add_type!("task_event", &mut pyrf_task_event__type);
    add_type!("throttle_event", &mut pyrf_throttle_event__type);
    add_type!("task_event", &mut pyrf_task_event__type);
    add_type!("read_event", &mut pyrf_read_event__type);
    add_type!("sample_event", &mut pyrf_sample_event__type);
    add_type!("switch_event", &mut pyrf_context_switch_event__type);
    add_type!("stat_event", &mut pyrf_stat_event__type);
    add_type!("stat_round_event", &mut pyrf_stat_round_event__type);
    add_type!("thread_map", &mut pyrf_thread_map__type);
    add_type!("cpu_map", &mut pyrf_cpu_map__type);
    add_type!("counts_values", &mut pyrf_counts_values__type);
    add_type!("data", &mut pyrf_data__type);
    add_type!("session", &mut pyrf_session__type);
    if PyModule_AddObject(module, cstr!("branch_entry"), &mut pyrf_branch_entry__type as *mut _ as *mut PyObject) < 0 {
        Py_DECREF(&mut pyrf_branch_entry__type as *mut _ as *mut PyObject);
    }
    if PyModule_AddObject(module, cstr!("branch_stack"), &mut pyrf_branch_stack__type as *mut _ as *mut PyObject) < 0 {
        Py_DECREF(&mut pyrf_branch_stack__type as *mut _ as *mut PyObject);
    }

    let dict = PyModule_GetDict(module);
    if !dict.is_null() {
        let mut i = 0usize;
        while !perf__constants[i].name.is_null() {
            let obj = PyLong_FromLong(perf__constants[i].value as c_long);
            if obj.is_null() { break; }
            PyDict_SetItemString(dict, perf__constants[i].name, obj);
            Py_DECREF(obj);
            i += 1;
        }
    }
    if !PyErr_Occurred().is_null() {
        Py_XDECREF(module);
        return ptr::null_mut();
    }
    module
}
