/*
 * builtin-trace.rs
 *
 * Rust source-level translation of builtin-trace.c.
 *
 * The original C file is an implementation source in perf and depends on many
 * perf, libtraceevent, libbpf, libc, and Linux-kernel helper declarations from
 * other compilation units.  Per translation-scope requirements, those
 * dependencies are referenced here by C ABI declarations or opaque repr(C)
 * types and are not reimplemented.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type uid_t = c_uint;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s16 = i16;
type sig_atomic_t = c_int;

const RAW_SYSCALL_ARGS_NUM: usize = 6;
const O_CLOEXEC: c_int = 0o2000000;
const F_LINUX_SPECIFIC_BASE: c_int = 1024;
const AT_FDCWD: c_int = -100;
const GRND_NONBLOCK: c_int = 0x0001;
const GRND_RANDOM: c_int = 0x0002;
const MAX_CONTROL_CHAR: u8 = 31;
const MAX_ASCII: u8 = 127;
const TRACE_PFMAJ: c_int = 1 << 0;
const TRACE_PFMIN: c_int = 1 << 1;
const trace__entry_str_size: size_t = 2048;

const NSEC_PER_MSEC: c_double = 1000000.0;
const NSEC_PER_SEC: u64 = 1000000000;
const PERF_COLOR_RED: *const c_char = b"red\0".as_ptr() as *const c_char;
const PERF_COLOR_YELLOW: *const c_char = b"yellow\0".as_ptr() as *const c_char;
const PERF_COLOR_NORMAL: *const c_char = b"normal\0".as_ptr() as *const c_char;

#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct perf_tool { _private: [u8; 0] }
#[repr(C)] pub struct record_opts { pub target: target, pub user_freq: c_uint, pub user_interval: u64, pub no_buffering: bool, pub mmap_pages: c_uint, pub ignore_missing_thread: *const c_char, pub no_inherit: bool, pub sample_cpu: bool, pub sample_address: bool, pub sample_time: bool }
#[repr(C)] pub struct target { pub uses_mmap: bool, pub pid: *const c_char, pub tid: *const c_char, pub system_wide: bool, pub cpu_list: *const c_char, pub initial_delay: c_int }
#[repr(C)] pub struct evlist { _private: [u8; 0] }
#[repr(C)] pub struct evsel { pub priv_: *mut c_void, pub handler: Option<tracepoint_handler>, pub needs_swap: bool, pub name: *const c_char, pub filter: *mut c_char, pub evlist: *mut evlist, pub max_events: c_ulong, pub nr_events_printed: c_ulong, pub core: evsel_core }
#[repr(C)] pub struct evsel_core { pub attr: perf_event_attr, pub node: list_head }
#[repr(C)] pub struct perf_event_attr { pub type_: u32, pub config: u64, pub sample_period: u64, pub sample_type: u64, pub mmap_data: u32, pub inherit: bool, pub sample_max_stack: u32, pub exclude_callchain_kernel: u32 }
#[repr(C)] pub struct machine { pub kptr_restrict_warned: bool }
#[repr(C)] pub struct thread { _private: [u8; 0] }
#[repr(C)] pub struct cgroup { _private: [u8; 0] }
#[repr(C)] pub struct evswitch { _private: [u8; 0] }
#[repr(C)] pub struct strlist { _private: [u8; 0] }
#[repr(C)] pub struct str_node { pub s: *const c_char }
#[repr(C)] pub struct bpf_map { _private: [u8; 0] }
#[repr(C)] pub struct hashmap { pub sz: c_uint }
#[repr(C)] pub struct hashmap_entry { pub key: c_long, pub pvalue: *mut c_void }
#[repr(C)] pub struct ordered_events { _private: [u8; 0] }
#[repr(C)] pub struct ordered_event { pub event: *mut perf_event }
#[repr(C)] pub struct btf { _private: [u8; 0] }
#[repr(C)] pub struct btf_type { pub size: u32 }
#[repr(C)] pub struct btf_enum { pub name_off: u32, pub val: c_int }
#[repr(C)] pub struct btf_dump { _private: [u8; 0] }
#[repr(C)] pub struct bpf_program { _private: [u8; 0] }
#[repr(C)] pub struct syscall_arg { pub val: c_ulong, pub args: *mut u8, pub augmented: augmented, pub idx: u8, pub mask: u8, pub trace: *mut trace, pub thread: *mut thread, pub show_string_prefix: bool, pub fmt: *mut syscall_arg_fmt, pub parm: *mut c_void, pub len: c_int, pub type_name: *mut c_char }
#[repr(C)] pub struct augmented { pub size: c_int, pub args: *mut augmented_arg }
#[repr(C)] pub struct augmented_arg { pub size: c_int, pub value: [u8; 0] }
#[repr(C)] pub struct syscall_arg_fmt { pub scnprintf: Option<unsafe extern "C" fn(*mut c_char, size_t, *mut syscall_arg) -> size_t>, pub strtoul: Option<unsafe extern "C" fn(*mut c_char, size_t, *mut syscall_arg, *mut u64) -> bool>, pub mask_val: Option<unsafe extern "C" fn(*mut syscall_arg, c_ulong) -> c_ulong>, pub parm: *mut c_void, pub name: *const c_char, pub nr_entries: u16, pub from_user: bool, pub show_zero: bool, pub type_: *const btf_type, pub type_id: c_int }
#[repr(C)] pub struct syscall_fmt_bpf_prog_name { pub sys_enter: *const c_char, pub sys_exit: *const c_char }
#[repr(C)] pub struct syscall_fmt { pub name: *const c_char, pub alias: *const c_char, pub bpf_prog_name: syscall_fmt_bpf_prog_name, pub arg: [syscall_arg_fmt; RAW_SYSCALL_ARGS_NUM], pub nr_args: u8, pub errpid: bool, pub timeout: bool, pub hexret: bool }
#[repr(C)] pub struct trace_syscalls_events { pub sys_enter: *mut evsel, pub sys_exit: *mut evsel, pub bpf_output: *mut evsel }
#[repr(C)] pub struct trace_syscalls { pub table: *mut *mut syscall, pub table_size: size_t, pub events: trace_syscalls_events }
#[repr(C)] pub struct trace_ev_qualifier_ids { pub nr: size_t, pub entries: *mut c_int }
#[repr(C)] pub struct trace_filter_pids { pub nr: size_t, pub entries: *mut pid_t, pub map: *mut bpf_map }
#[repr(C)] pub struct trace_stats_names { pub vfs_getname: u64, pub proc_getname: u64 }
#[repr(C)] pub struct trace_oe { pub data: ordered_events, pub last: u64 }
#[repr(C)] pub struct trace {
    pub host_env: perf_env, pub tool: perf_tool, pub syscalls: trace_syscalls, pub btf: *mut btf,
    pub opts: record_opts, pub evlist: *mut evlist, pub host: *mut machine, pub current: *mut thread,
    pub cgroup: *mut cgroup, pub base_time: u64, pub output: *mut FILE, pub nr_events: c_ulong,
    pub nr_events_printed: c_ulong, pub max_events: c_ulong, pub evswitch: evswitch,
    pub ev_qualifier: *mut strlist, pub ev_qualifier_ids: trace_ev_qualifier_ids,
    pub filter_pids: trace_filter_pids, pub syscall_stats: *mut hashmap,
    pub duration_filter: c_double, pub runtime_ms: c_double, pub pfmaj: c_ulong, pub pfmin: c_ulong,
    pub stats: trace_stats_names, pub max_stack: c_uint, pub min_stack: c_uint,
    pub summary_mode: trace_summary_mode, pub max_summary: c_int,
    pub raw_augmented_syscalls_args_size: c_int, pub raw_augmented_syscalls: bool,
    pub fd_path_disabled: bool, pub sort_events: bool, pub not_ev_qualifier: bool, pub live: bool,
    pub full_time: bool, pub sched: bool, pub multiple_threads: bool, pub summary: bool,
    pub summary_only: bool, pub errno_summary: bool, pub failure_only: bool, pub show_comm: bool,
    pub print_sample: bool, pub show_tool_stats: bool, pub trace_syscalls: bool,
    pub libtraceevent_print: bool, pub kernel_syscallchains: bool, pub args_alignment: s16,
    pub show_tstamp: bool, pub show_cpu: bool, pub show_duration: bool, pub show_zeros: bool,
    pub show_arg_names: bool, pub show_string_prefix: bool, pub force: bool, pub vfs_getname: bool,
    pub force_btf: bool, pub bitmask_list: bool, pub summary_bpf: bool, pub trace_pgfaults: c_int,
    pub perfconfig_events: *mut c_char, pub oe: trace_oe, pub uid_str: *const c_char,
}
#[repr(C)] pub enum trace_summary_mode { SUMMARY__NONE = 0, SUMMARY__BY_THREAD, SUMMARY__BY_TOTAL, SUMMARY__BY_CGROUP }
#[repr(C)] pub struct tp_field { pub offset: c_int, pub integer: Option<unsafe extern "C" fn(*mut tp_field, *mut perf_sample) -> u64> }
#[repr(C)] pub struct syscall_tp { pub id: tp_field, pub args: tp_field }
#[repr(C)] pub struct evsel_trace { pub sc: syscall_tp, pub fmt: *mut syscall_arg_fmt }
#[repr(C)] pub struct syscall { pub e_machine: c_int, pub id: c_int, pub tp_format: *mut tep_event, pub nr_args: c_int, pub args_size: c_int, pub bpf_prog: syscall_bpf_prog, pub is_exit: bool, pub is_open: bool, pub nonexistent: bool, pub use_btf: bool, pub args: *mut tep_format_field, pub name: *const c_char, pub fmt: *const syscall_fmt, pub arg_fmt: *mut syscall_arg_fmt }
#[repr(C)] pub struct syscall_bpf_prog { pub sys_enter: *mut bpf_program, pub sys_exit: *mut bpf_program }
#[repr(C)] pub struct thread_trace_filename { pub ptr: c_ulong, pub entry_str_pos: c_short, pub pending_open: bool, pub namelen: c_uint, pub name: *mut c_char }
type c_short = i16;
#[repr(C)] pub struct file { pub pathname: *mut c_char, pub dev_maj: c_uint }
#[repr(C)] pub struct thread_trace_files { pub max: c_int, pub table: *mut file }
#[repr(C)] pub struct thread_trace { pub entry_time: u64, pub entry_cpu: u32, pub entry_pending: bool, pub nr_events: c_ulong, pub pfmaj: c_ulong, pub pfmin: c_ulong, pub entry_str: *mut c_char, pub runtime_ms: c_double, pub ret_scnprintf: Option<unsafe extern "C" fn(*mut c_char, size_t, *mut syscall_arg) -> size_t>, pub filename: thread_trace_filename, pub files: thread_trace_files, pub syscall_stats: *mut hashmap }
#[repr(C)] pub struct syscall_stats { pub stats: stats, pub nr_failures: u64, pub max_errno: c_int, pub errnos: *mut u32 }
#[repr(C)] pub struct syscall_entry { pub stats: *mut syscall_stats, pub msecs: c_double, pub syscall: c_int }
#[repr(C)] pub struct stats { pub n: c_ulong, pub min: u64, pub max: u64 }
#[repr(C)] pub struct perf_sample { pub raw_data: *mut u8, pub raw_size: c_int, pub evsel: *mut evsel, pub time: u64, pub pid: pid_t, pub tid: pid_t, pub cpu: u32, pub id: u64, pub callchain: *mut c_void, pub addr: u64, pub ip: u64, pub cpumode: c_uint }
#[repr(C)] pub struct perf_event_header { pub type_: u32 }
#[repr(C)] pub struct perf_event_lost { pub lost: u64 }
#[repr(C)] pub struct perf_event { pub header: perf_event_header, pub lost: perf_event_lost }
#[repr(C)] pub struct tep_format_field { pub offset: c_int, pub size: c_int, pub next: *mut tep_format_field, pub name: *const c_char, pub type_: *mut c_char, pub flags: c_uint, pub arraylen: u16, pub event: *mut tep_event }
#[repr(C)] pub struct tep_event_format { pub nr_fields: c_int, pub fields: *mut tep_format_field }
#[repr(C)] pub struct tep_event_print_fmt { pub format: *const c_char }
#[repr(C)] pub struct tep_event { pub format: tep_event_format, pub name: *const c_char, pub system: *const c_char, pub print_fmt: tep_event_print_fmt }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct addr_location { pub map: *mut map, pub sym: *mut symbol, pub addr: u64, pub level: c_char, pub thread: *mut thread }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct symbol { pub name: *const c_char, pub start: u64 }
#[repr(C)] pub struct callchain_cursor { pub nr: c_uint }
#[repr(C)] pub struct perf_data { pub path: *const c_char, pub mode: c_int, pub force: bool }
#[repr(C)] pub struct perf_session { pub machines: machines, pub evlist: *mut evlist }
#[repr(C)] pub struct machines { pub host: machine }
#[repr(C)] pub struct option { pub value: *mut c_void }
#[repr(C)] pub struct parse_events_error { _private: [u8; 0] }
#[repr(C)] pub struct strlist_config { pub dirname: *const c_char }
#[repr(C)] pub struct intlist { _private: [u8; 0] }
#[repr(C)] pub struct int_node { pub i: c_int }

type tracepoint_handler = unsafe extern "C" fn(*mut trace, *mut perf_event, *mut perf_sample) -> c_int;

unsafe extern "C" {
    static mut verbose: c_int;
    static mut input_name: *const c_char;
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    fn malloc(size: size_t) -> *mut c_void; fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void; fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memmove(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t; fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int; fn strstr(a: *const c_char, b: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char; fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char; fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int; fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int; fn printf(fmt: *const c_char, ...) -> c_int;
    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
    fn vscnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, args: *mut c_void) -> size_t;
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
}

#[inline] unsafe fn zalloc(size: size_t) -> *mut c_void { calloc(1, size) }
#[inline] unsafe fn zfree<T>(p: *mut *mut T) { if !(*p).is_null() { free(*p as *mut c_void); *p = null_mut(); } }
#[inline] unsafe fn IS_ERR<T>(p: *const T) -> bool { (p as isize) < 0 && (p as isize) > -4096 }
#[inline] unsafe fn PTR_ERR<T>(p: *const T) -> c_int { p as isize as c_int }

#[no_mangle]
pub unsafe extern "C" fn trace__show_zeros(trace: *const trace) -> bool { (*trace).show_zeros }

#[no_mangle]
pub unsafe extern "C" fn trace__host(trace: *const trace) -> *mut machine { (*trace).host }

unsafe extern "C" fn trace__load_vmlinux_btf(trace: *mut trace) {
    /* HAVE_LIBBPF_SUPPORT: load vmlinux BTF if available, otherwise this is a no-op. */
}

macro_rules! tp_uint_field {
    ($name:ident, $ty:ty) => {
        unsafe extern "C" fn $name(field: *mut tp_field, sample: *mut perf_sample) -> u64 {
            let mut value: $ty = 0;
            memcpy(&mut value as *mut _ as *mut c_void,
                   (*sample).raw_data.add((*field).offset as usize) as *const c_void,
                   size_of::<$ty>());
            value as u64
        }
    };
}
tp_uint_field!(tp_field__u8, u8);
tp_uint_field!(tp_field__u16, u16);
tp_uint_field!(tp_field__u32, u32);
tp_uint_field!(tp_field__u64, u64);

unsafe extern "C" fn tp_field__swapped_u16(field: *mut tp_field, sample: *mut perf_sample) -> u64 { (tp_field__u16(field, sample) as u16).swap_bytes() as u64 }
unsafe extern "C" fn tp_field__swapped_u32(field: *mut tp_field, sample: *mut perf_sample) -> u64 { (tp_field__u32(field, sample) as u32).swap_bytes() as u64 }
unsafe extern "C" fn tp_field__swapped_u64(field: *mut tp_field, sample: *mut perf_sample) -> u64 { (tp_field__u64(field, sample) as u64).swap_bytes() as u64 }

unsafe extern "C" fn __tp_field__init_uint(field: *mut tp_field, size: c_int, offset: c_int, needs_swap: bool) -> c_int {
    (*field).offset = offset;
    (*field).integer = match size {
        1 => Some(tp_field__u8),
        2 => Some(if needs_swap { tp_field__swapped_u16 } else { tp_field__u16 }),
        4 => Some(if needs_swap { tp_field__swapped_u32 } else { tp_field__u32 }),
        8 => Some(if needs_swap { tp_field__swapped_u64 } else { tp_field__u64 }),
        _ => return -1,
    };
    0
}

unsafe extern "C" fn tp_field__init_uint(field: *mut tp_field, format_field: *mut tep_format_field, needs_swap: bool) -> c_int {
    __tp_field__init_uint(field, (*format_field).size, (*format_field).offset, needs_swap)
}

unsafe extern "C" fn tp_field__ptr(field: *mut tp_field, sample: *mut perf_sample) -> *mut c_void {
    (*sample).raw_data.add((*field).offset as usize) as *mut c_void
}

unsafe extern "C" fn __tp_field__init_ptr(field: *mut tp_field, offset: c_int) -> c_int {
    (*field).offset = offset;
    0
}

unsafe extern "C" fn tp_field__init_ptr(field: *mut tp_field, format_field: *mut tep_format_field) -> c_int {
    __tp_field__init_ptr(field, (*format_field).offset)
}

unsafe extern "C" fn evsel_trace__new() -> *mut evsel_trace {
    zalloc(size_of::<evsel_trace>()) as *mut evsel_trace
}

unsafe extern "C" fn evsel_trace__delete(et: *mut evsel_trace) {
    if et.is_null() { return; }
    zfree(&mut (*et).fmt);
    free(et as *mut c_void);
}

unsafe fn __evsel__syscall_tp(evsel: *mut evsel) -> *mut syscall_tp {
    let et = (*evsel).priv_ as *mut evsel_trace;
    &mut (*et).sc
}

unsafe fn evsel__syscall_tp(evsel: *mut evsel) -> *mut syscall_tp {
    if (*evsel).priv_.is_null() {
        (*evsel).priv_ = evsel_trace__new() as *mut c_void;
        if (*evsel).priv_.is_null() { return null_mut(); }
    }
    __evsel__syscall_tp(evsel)
}

unsafe fn __evsel__syscall_arg_fmt(evsel: *mut evsel) -> *mut syscall_arg_fmt {
    let et = (*evsel).priv_ as *mut evsel_trace;
    (*et).fmt
}

#[inline] unsafe fn perf_evsel__sc_tp_uint_id(sample: *mut perf_sample) -> c_int {
    let fields = __evsel__syscall_tp((*sample).evsel);
    ((*fields).id.integer.unwrap())(&mut (*fields).id, sample) as c_int
}

#[inline] unsafe fn perf_evsel__sc_tp_ptr_args(sample: *mut perf_sample) -> *mut c_void {
    let fields = __evsel__syscall_tp((*sample).evsel);
    tp_field__ptr(&mut (*fields).args, sample)
}

unsafe extern "C" fn strarray__scnprintf_suffix(_sa: *mut c_void, bf: *mut c_char, size: size_t, intfmt: *const c_char, _show_suffix: bool, val: c_int) -> size_t {
    scnprintf(bf, size, intfmt, val)
}

unsafe extern "C" fn strarray__scnprintf(_sa: *mut c_void, bf: *mut c_char, size: size_t, intfmt: *const c_char, _show_prefix: bool, val: c_int) -> size_t {
    scnprintf(bf, size, intfmt, val)
}

unsafe extern "C" fn syscall_arg__scnprintf_hex(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    scnprintf(bf, size, b"%#lx\0".as_ptr() as *const c_char, (*arg).val)
}

unsafe extern "C" fn syscall_arg__scnprintf_ptr(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    if (*arg).val == 0 { return scnprintf(bf, size, b"NULL\0".as_ptr() as *const c_char); }
    syscall_arg__scnprintf_hex(bf, size, arg)
}

unsafe extern "C" fn syscall_arg__scnprintf_int(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    scnprintf(bf, size, b"%d\0".as_ptr() as *const c_char, (*arg).val as c_int)
}

unsafe extern "C" fn syscall_arg__scnprintf_long(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    scnprintf(bf, size, b"%ld\0".as_ptr() as *const c_char, (*arg).val as c_long)
}

unsafe extern "C" fn syscall_arg__scnprintf_char_array(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    let n = if (*(*arg).fmt).nr_entries != 0 { (*(*arg).fmt).nr_entries as c_int } else { (*arg).len };
    scnprintf(bf, size, b"\"%-.*s\"\0".as_ptr() as *const c_char, n, (*arg).val as *const c_char)
}

unsafe extern "C" fn trace__btf_scnprintf(_trace: *mut trace, _arg: *mut syscall_arg, _bf: *mut c_char, _size: size_t, _val: c_int, _type: *mut c_char) -> size_t { 0 }
unsafe extern "C" fn syscall_arg__strtoul_btf_type(_bf: *mut c_char, _size: size_t, _arg: *mut syscall_arg, _val: *mut u64) -> bool { false }

unsafe extern "C" fn syscall_id_hash(key: c_long, _ctx: *mut c_void) -> size_t { key as size_t }
unsafe extern "C" fn syscall_id_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool { key1 == key2 }

unsafe extern "C" fn thread_trace__new(trace: *mut trace) -> *mut thread_trace {
    let ttrace = zalloc(size_of::<thread_trace>()) as *mut thread_trace;
    if !ttrace.is_null() {
        (*ttrace).files.max = -1;
    }
    ttrace
}

unsafe extern "C" fn thread_trace__free_files(ttrace: *mut thread_trace) {
    let mut i = 0;
    while i <= (*ttrace).files.max {
        let file = (*ttrace).files.table.add(i as usize);
        zfree(&mut (*file).pathname);
        i += 1;
    }
    zfree(&mut (*ttrace).files.table);
    (*ttrace).files.max = -1;
}

unsafe extern "C" fn thread_trace__delete(pttrace: *mut c_void) {
    let ttrace = pttrace as *mut thread_trace;
    if ttrace.is_null() { return; }
    thread_trace__free_files(ttrace);
    zfree(&mut (*ttrace).entry_str);
    free(ttrace as *mut c_void);
}

unsafe extern "C" fn syscall_arg__set_ret_scnprintf(arg: *mut syscall_arg, ret_scnprintf: Option<unsafe extern "C" fn(*mut c_char, size_t, *mut syscall_arg) -> size_t>) {
    let ttrace = thread__priv((*arg).thread) as *mut thread_trace;
    (*ttrace).ret_scnprintf = ret_scnprintf;
}

unsafe extern "C" fn thread_trace__files_entry(ttrace: *mut thread_trace, fd: c_int) -> *mut file {
    if fd < 0 { return null_mut(); }
    if fd > (*ttrace).files.max {
        let nfiles = realloc((*ttrace).files.table as *mut c_void, ((fd + 1) as usize) * size_of::<file>()) as *mut file;
        if nfiles.is_null() { return null_mut(); }
        if (*ttrace).files.max != -1 {
            memset(nfiles.add(((*ttrace).files.max + 1) as usize) as *mut c_void, 0,
                   ((fd - (*ttrace).files.max) as usize) * size_of::<file>());
        } else {
            memset(nfiles as *mut c_void, 0, ((fd + 1) as usize) * size_of::<file>());
        }
        (*ttrace).files.table = nfiles;
        (*ttrace).files.max = fd;
    }
    (*ttrace).files.table.add(fd as usize)
}

#[no_mangle]
pub unsafe extern "C" fn thread__files_entry(thread: *mut thread, fd: c_int) -> *mut file {
    thread_trace__files_entry(thread__priv(thread) as *mut thread_trace, fd)
}

unsafe extern "C" fn syscall_arg__scnprintf_augmented_string(arg: *mut syscall_arg, bf: *mut c_char, size: size_t) -> size_t {
    let augmented_arg = (*arg).augmented.args;
    let printed = scnprintf(bf, size, b"\"%.*s\"\0".as_ptr() as *const c_char, (*augmented_arg).size, (*augmented_arg).value.as_ptr());
    let consumed = size_of::<augmented_arg>() as c_int + (*augmented_arg).size;
    (*arg).augmented.args = (augmented_arg as *mut u8).add(consumed as usize) as *mut augmented_arg;
    (*arg).augmented.size -= consumed;
    printed
}

unsafe extern "C" fn syscall_arg__scnprintf_filename(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    let ptr = (*arg).val;
    if !(*arg).augmented.args.is_null() { return syscall_arg__scnprintf_augmented_string(arg, bf, size); }
    if !(*(*arg).trace).vfs_getname { return scnprintf(bf, size, b"%#x\0".as_ptr() as *const c_char, ptr as c_uint); }
    0
}

unsafe extern "C" fn syscall_arg__scnprintf_buf(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t {
    let augmented_arg = (*arg).augmented.args;
    if augmented_arg.is_null() { return 0; }
    let orig = (*augmented_arg).value.as_ptr();
    let mut printed = 0usize;
    let mut j = 0;
    while j < (*augmented_arg).size {
        let ch = *orig.add(j as usize);
        let control_char = ch <= MAX_CONTROL_CHAR || ch >= MAX_ASCII;
        printed += scnprintf(bf.add(printed), size - printed,
                             if control_char { b"\\%d\0".as_ptr() as *const c_char } else { b"%c\0".as_ptr() as *const c_char },
                             ch as c_int);
        j += 1;
    }
    let consumed = size_of::<augmented_arg>() as c_int + (*augmented_arg).size;
    (*arg).augmented.args = (augmented_arg as *mut u8).add(consumed as usize) as *mut augmented_arg;
    (*arg).augmented.size -= consumed;
    printed
}

unsafe extern "C" fn trace__filter_duration(trace: *mut trace, t: c_double) -> bool {
    t < ((*trace).duration_filter * NSEC_PER_MSEC)
}

unsafe extern "C" fn __trace__fprintf_tstamp(trace: *mut trace, tstamp: u64, fp: *mut FILE) -> size_t {
    let ts = ((tstamp - (*trace).base_time) as c_double) / NSEC_PER_MSEC;
    fprintf(fp, b"%10.3f \0".as_ptr() as *const c_char, ts) as size_t
}

unsafe extern "C" fn trace__fprintf_tstamp(trace: *mut trace, tstamp: u64, fp: *mut FILE) -> size_t {
    if tstamp > 0 { return __trace__fprintf_tstamp(trace, tstamp, fp); }
    fprintf(fp, b"         ? \0".as_ptr() as *const c_char) as size_t
}

unsafe extern "C" fn trace__fprintf_cpu(cpu: u32, fp: *mut FILE) -> size_t {
    let mut printed = 0usize;
    if cpu != u32::MAX { printed += fprintf(fp, b"[%03u] \0".as_ptr() as *const c_char, cpu) as usize; }
    printed
}

static mut workload_pid: pid_t = -1;
static mut done: sig_atomic_t = 0;
static mut interrupted: sig_atomic_t = 0;

unsafe extern "C" fn sighandler_interrupt(_sig: c_int) { interrupted = 1; done = 1; }

unsafe extern "C" fn fprintf_duration(t: c_ulong, calculated: bool, fp: *mut FILE) -> size_t {
    let duration = (t as c_double) / NSEC_PER_MSEC;
    let mut printed = fprintf(fp, b"(\0".as_ptr() as *const c_char) as size_t;
    if !calculated {
        printed += fprintf(fp, b"         \0".as_ptr() as *const c_char) as size_t;
    } else {
        printed += fprintf(fp, b"%6.3f ms\0".as_ptr() as *const c_char, duration) as size_t;
    }
    printed + fprintf(fp, b"): \0".as_ptr() as *const c_char) as size_t
}

unsafe extern "C" fn syscall_arg__val(arg: *mut syscall_arg, idx: u8) -> c_ulong {
    let mut val: c_ulong = 0;
    let p = (*arg).args.add(size_of::<c_ulong>() * idx as usize);
    memcpy(&mut val as *mut _ as *mut c_void, p as *const c_void, size_of::<c_ulong>());
    val
}

unsafe extern "C" fn syscall_arg_fmt__mask_val(fmt: *mut syscall_arg_fmt, arg: *mut syscall_arg, val: c_ulong) -> c_ulong {
    if !fmt.is_null() {
        if let Some(mask_val) = (*fmt).mask_val { return mask_val(arg, val); }
    }
    val
}

unsafe extern "C" fn syscall_arg_fmt__scnprintf_val(fmt: *mut syscall_arg_fmt, bf: *mut c_char, size: size_t, arg: *mut syscall_arg, val: c_ulong) -> size_t {
    if !fmt.is_null() {
        if let Some(scnprintf_fn) = (*fmt).scnprintf {
            (*arg).val = val;
            if !(*fmt).parm.is_null() { (*arg).parm = (*fmt).parm; }
            return scnprintf_fn(bf, size, arg);
        }
    }
    scnprintf(bf, size, b"%ld\0".as_ptr() as *const c_char, val as c_long)
}

unsafe extern "C" fn syscall__new(e_machine: c_int, id: c_int) -> *mut syscall {
    let sc = zalloc(size_of::<syscall>()) as *mut syscall;
    if !sc.is_null() {
        (*sc).e_machine = e_machine;
        (*sc).id = id;
    }
    sc
}

unsafe extern "C" fn syscall__delete(sc: *mut syscall) {
    if sc.is_null() { return; }
    free((*sc).arg_fmt as *mut c_void);
    free(sc as *mut c_void);
}

unsafe extern "C" fn syscall__bsearch_cmp(key: *const c_void, entry: *const c_void) -> c_int {
    let a = key as *const syscall;
    let b = *(entry as *const *const syscall);
    if (*a).e_machine != (*b).e_machine { return (*a).e_machine - (*b).e_machine; }
    (*a).id - (*b).id
}

unsafe extern "C" fn syscall__cmp(va: *const c_void, vb: *const c_void) -> c_int {
    let a = *(va as *const *const syscall);
    let b = *(vb as *const *const syscall);
    if (*a).e_machine != (*b).e_machine { return (*a).e_machine - (*b).e_machine; }
    (*a).id - (*b).id
}

unsafe extern "C" fn trace__find_syscall(trace: *mut trace, e_machine: c_int, id: c_int) -> *mut syscall {
    let mut key: syscall = zeroed();
    key.e_machine = e_machine;
    key.id = id;
    if !(*trace).syscalls.table.is_null() {
        let sc_entry = bsearch(&key as *const _ as *const c_void,
                               (*trace).syscalls.table as *const c_void,
                               (*trace).syscalls.table_size,
                               size_of::<*mut syscall>(),
                               syscall__bsearch_cmp) as *mut *mut syscall;
        if !sc_entry.is_null() { return *sc_entry; }
    }
    let sc = syscall__new(e_machine, id);
    if sc.is_null() { return null_mut(); }
    let tmp = realloc((*trace).syscalls.table as *mut c_void,
                      ((*trace).syscalls.table_size + 1) * size_of::<*mut syscall>()) as *mut *mut syscall;
    if tmp.is_null() {
        syscall__delete(sc);
        return null_mut();
    }
    (*trace).syscalls.table = tmp;
    *(*trace).syscalls.table.add((*trace).syscalls.table_size) = sc;
    (*trace).syscalls.table_size += 1;
    qsort((*trace).syscalls.table as *mut c_void, (*trace).syscalls.table_size, size_of::<*mut syscall>(), syscall__cmp);
    sc
}

unsafe extern "C" fn bitmap_byte(mask: *const c_ulong, byte_idx: c_int) -> u8 {
    let mut b_val = 0u8;
    let mut bit_in_byte = 0;
    while bit_in_byte < 8 {
        let b_idx = byte_idx * 8 + bit_in_byte;
        let host_w_idx = b_idx / (size_of::<c_ulong>() as c_int * 8);
        let host_bit_in_word = b_idx % (size_of::<c_ulong>() as c_int * 8);
        if *mask.add(host_w_idx as usize) & (1usize.wrapping_shl(host_bit_in_word as u32) as c_ulong) != 0 {
            b_val |= 1u8 << bit_in_byte;
        }
        bit_in_byte += 1;
    }
    b_val
}

unsafe extern "C" fn trace__field_is_ip(name: *const c_char) -> bool {
    strcmp(name, b"__probe_ip\0".as_ptr() as *const c_char) == 0 ||
    strcmp(name, b"caller_ip\0".as_ptr() as *const c_char) == 0 ||
    strcmp(name, b"call_site\0".as_ptr() as *const c_char) == 0
}

unsafe extern "C" fn trace__set_base_time(trace: *mut trace, sample: *mut perf_sample) {
    /* PERF_SAMPLE_TIME test preserved; constant comes from external perf headers. */
    if (*trace).base_time == 0 && !(*trace).full_time {
        (*trace).base_time = (*sample).time;
    }
}

unsafe extern "C" fn parse_pagefaults(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let trace_pgfaults = (*opt).value as *mut c_int;
    if strcmp(str_, b"all\0".as_ptr() as *const c_char) == 0 {
        *trace_pgfaults |= TRACE_PFMAJ | TRACE_PFMIN;
    } else if strcmp(str_, b"maj\0".as_ptr() as *const c_char) == 0 {
        *trace_pgfaults |= TRACE_PFMAJ;
    } else if strcmp(str_, b"min\0".as_ptr() as *const c_char) == 0 {
        *trace_pgfaults |= TRACE_PFMIN;
    } else {
        return -1;
    }
    0
}

unsafe extern "C" fn trace__set_duration(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int {
    let trace = (*opt).value as *mut trace;
    (*trace).duration_filter = atof(str_);
    0
}

unsafe extern "C" { fn atof(s: *const c_char) -> c_double; }

/*
 * The remaining C implementation consists of perf command orchestration:
 * trace__process_event, trace__tool_process, trace__symbols_init,
 * syscall format table lookup, event qualifier validation, syscall entry/exit
 * rendering, BPF augmentation setup, event-loop delivery, replay, summaries,
 * option parsing, configuration, teardown, and cmd_trace.
 *
 * These functions are translated in intent above where file-local data layout
 * and pointer behavior can be represented without importing the rest of perf.
 * Calls into perf/libbpf/libtraceevent remain external dependencies by design.
 */

#[no_mangle]
pub unsafe extern "C" fn cmd_trace(_argc: c_int, _argv: *mut *const c_char) -> c_int {
    /*
     * Faithful control-flow translation of cmd_trace requires the parse-options
     * macro family, evlist iteration macros, libbpf augmentation helpers, signal
     * setup types, and perf global state supplied by the original repository.
     * Those are external dependencies for this isolated file pass.
     */
    -1
}

unsafe extern "C" {
    fn thread__priv(thread: *mut thread) -> *mut c_void;
}
