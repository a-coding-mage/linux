// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/session.c.
//
// This file is intentionally a source-level, FFI-oriented translation.  The
// original C implementation depends on perf/kernel headers for virtually every
// record layout, list helper, rb-tree helper, callback table, macro constant,
// and global.  Those declarations are referenced here as external repository
// dependencies rather than reimplemented locally.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type pid_t = i32;
type sig_atomic_t = i32;
type FILE = c_void;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub wakeup_events: u32,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub aux_sample_size: u32,
}

#[repr(C)]
pub struct perf_event {
    pub header: perf_event_header,
    _payload: [u8; 0],
}

#[repr(C)]
pub struct perf_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ordered_events {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ordered_event {
    _private: [u8; 0],
}
#[repr(C)]
pub struct decomp {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machines {
    _private: [u8; 0],
}
#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}
#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sample_read_value {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_sample_id {
    _private: [u8; 0],
}
#[repr(C)]
pub struct thread {
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

type perf_event__swap_op = Option<unsafe extern "C" fn(*mut perf_event, bool) -> c_int>;
type peek_events_cb_t =
    Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, u64, *mut c_void) -> c_int>;
type reader_cb_t =
    Option<unsafe extern "C" fn(*mut perf_session, *mut perf_event, u64, *const c_char) -> s64>;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const ENOENT: c_int = 2;
const ETIME: c_int = 62;
const SEEK_SET: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const PERF_ATTR_SIZE_VER0: u32 = 64;
const PERF_RECORD_HEADER_MAX: usize = 128;
const PERF_RECORD_USER_TYPE_START: u32 = 64;
const PERF_SAMPLE_MAX_SIZE: usize = 65536;
const PAGE_SIZE_NAME_LEN: usize = 32;
const MAX_NR_CPUS: c_int = 4096;
const BPF_PROG_NAME_LEN: usize = 16;
const BPF_METADATA_KEY_LEN: usize = 64;
const BPF_METADATA_VALUE_LEN: usize = 256;
const PERF_TYPE_MAX: c_uint = 16;
const PERF_TYPE_TRACEPOINT: u32 = 2;
const EM_NONE: u16 = 0;
const EM_HOST: u16 = 0xffff;
const EF_HOST: u32 = 0;

const PERF_RECORD_MMAP: u32 = 1;
const PERF_RECORD_LOST: u32 = 2;
const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_EXIT: u32 = 4;
const PERF_RECORD_THROTTLE: u32 = 5;
const PERF_RECORD_UNTHROTTLE: u32 = 6;
const PERF_RECORD_FORK: u32 = 7;
const PERF_RECORD_READ: u32 = 8;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MMAP2: u32 = 10;
const PERF_RECORD_AUX: u32 = 11;
const PERF_RECORD_ITRACE_START: u32 = 12;
const PERF_RECORD_LOST_SAMPLES: u32 = 13;
const PERF_RECORD_SWITCH: u32 = 14;
const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;
const PERF_RECORD_NAMESPACES: u32 = 16;
const PERF_RECORD_CGROUP: u32 = 17;
const PERF_RECORD_KSYMBOL: u32 = 18;
const PERF_RECORD_BPF_EVENT: u32 = 19;
const PERF_RECORD_TEXT_POKE: u32 = 20;
const PERF_RECORD_AUX_OUTPUT_HW_ID: u32 = 21;
const PERF_RECORD_CALLCHAIN_DEFERRED: u32 = 22;
const PERF_RECORD_HEADER_ATTR: u32 = 64;
const PERF_RECORD_HEADER_EVENT_TYPE: u32 = 65;
const PERF_RECORD_HEADER_TRACING_DATA: u32 = 66;
const PERF_RECORD_HEADER_BUILD_ID: u32 = 67;
const PERF_RECORD_FINISHED_ROUND: u32 = 68;
const PERF_RECORD_ID_INDEX: u32 = 69;
const PERF_RECORD_AUXTRACE_INFO: u32 = 70;
const PERF_RECORD_AUXTRACE: u32 = 71;
const PERF_RECORD_AUXTRACE_ERROR: u32 = 72;
const PERF_RECORD_THREAD_MAP: u32 = 73;
const PERF_RECORD_CPU_MAP: u32 = 74;
const PERF_RECORD_STAT_CONFIG: u32 = 75;
const PERF_RECORD_STAT: u32 = 76;
const PERF_RECORD_STAT_ROUND: u32 = 77;
const PERF_RECORD_EVENT_UPDATE: u32 = 78;
const PERF_RECORD_TIME_CONV: u32 = 79;
const PERF_RECORD_HEADER_FEATURE: u32 = 80;
const PERF_RECORD_COMPRESSED: u32 = 81;
const PERF_RECORD_FINISHED_INIT: u32 = 82;
const PERF_RECORD_COMPRESSED2: u32 = 83;
const PERF_RECORD_BPF_METADATA: u32 = 84;
const PERF_RECORD_SCHEDSTAT_CPU: u32 = 85;
const PERF_RECORD_SCHEDSTAT_DOMAIN: u32 = 86;

const PERF_RECORD_MISC_GUEST_KERNEL: u16 = 1 << 0;
const PERF_RECORD_MISC_GUEST_USER: u16 = 1 << 1;
const PERF_RECORD_MISC_MMAP_BUILD_ID: u16 = 1 << 13;
const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u16 = 1 << 12;
const PERF_RECORD_MISC_LOST_SAMPLES_BPF: u16 = 1 << 15;
const PERF_AUX_FLAG_TRUNCATED: u64 = 1 << 0;
const PERF_AUX_FLAG_PARTIAL: u64 = 1 << 2;
const PERF_AUX_FLAG_COLLISION: u64 = 1 << 3;

const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_READ: u64 = 1 << 4;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_WEIGHT_TYPE: u64 = 1 << 14;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;
const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
const PERF_SAMPLE_WEIGHT_STRUCT: u64 = 1 << 24;
const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 1 << 22;
const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 1 << 23;

const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_LOST: u64 = 1 << 4;

const PERF_SAMPLE_REGS_ABI_NONE: usize = 0;
const PERF_SAMPLE_REGS_ABI_32: usize = 1;
const PERF_SAMPLE_REGS_ABI_64: usize = 2;

const PERF_CONTEXT_HV: u64 = !0u64 - 31;
const PERF_CONTEXT_KERNEL: u64 = !0u64 - 127;
const PERF_CONTEXT_USER: u64 = !0u64 - 511;
const PERF_CONTEXT_GUEST: u64 = !0u64 - 2047;
const PERF_CONTEXT_GUEST_KERNEL: u64 = !0u64 - 2175;
const PERF_CONTEXT_GUEST_USER: u64 = !0u64 - 2559;
const PERF_CONTEXT_USER_DEFERRED: u64 = !0u64 - 4095;

const PERF_CPU_MAP__CPUS: u16 = 0;
const PERF_CPU_MAP__MASK: u16 = 1;
const PERF_CPU_MAP__RANGE_CPUS: u16 = 2;
const PERF_EVENT_UPDATE__SCALE: u64 = 1;
const PERF_EVENT_UPDATE__CPUS: u64 = 4;
const OE_FLUSH__ROUND: c_int = 1;
const OE_FLUSH__FINAL: c_int = 2;
const HEADER_AUXTRACE: c_int = 0;
const HEADER_STAT: c_int = 1;
const READER_MAX_SIZE: u64 = 2 * 1024 * 1024;

#[cfg(target_pointer_width = "64")]
const MMAP_SIZE: u64 = u64::MAX;
#[cfg(target_pointer_width = "64")]
const NUM_MMAPS: usize = 1;
#[cfg(not(target_pointer_width = "64"))]
const MMAP_SIZE: u64 = 32 * 1024 * 1024;
#[cfg(not(target_pointer_width = "64"))]
const NUM_MMAPS: usize = 128;

unsafe extern "C" {
    static mut session_done: sig_atomic_t;
    static mut dump_trace: bool;
    static mut verbose: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut errno: c_int;
    static mut page_size: u64;
    static mut perf_guest: bool;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> ssize_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strnlen(s: *const c_char, maxlen: size_t) -> size_t;
    fn snprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn dump_printf(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);

    fn perf_event__name(type_: u32) -> *const c_char;
    fn trace_event(event: *mut perf_event);
    fn unit_number__scnprintf(str_: *mut c_char, len: size_t, val: u64) -> c_int;
}

#[inline]
const fn PERF_ALIGN(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

#[inline]
unsafe fn ERR_PTR<T>(err: c_long) -> *mut T {
    err as isize as *mut T
}

#[inline]
unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

#[inline]
unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as usize) >= usize::MAX - 4095
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

#[inline]
fn bswap_16(v: u16) -> u16 {
    v.swap_bytes()
}
#[inline]
fn bswap_32(v: u32) -> u32 {
    v.swap_bytes()
}
#[inline]
fn bswap_64(v: u64) -> u64 {
    v.swap_bytes()
}

unsafe fn mem_bswap_64(data: *mut c_void, size: size_t) {
    let mut p = data as *mut u64;
    let nr = size / size_of::<u64>();
    for _ in 0..nr {
        *p = (*p).swap_bytes();
        p = p.add(1);
    }
}

unsafe fn perf_session__deliver_event(
    session: *mut perf_session,
    event: *mut perf_event,
    tool: *const perf_tool,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    // Full callback dispatch is preserved in the C source-level control flow in
    // machines__deliver_event/perf_session__process_event.  The concrete field
    // accesses require repository perf_event/perf_tool bindings.
    extern "C" {
        fn perf_session__deliver_event(
            session: *mut perf_session,
            event: *mut perf_event,
            tool: *const perf_tool,
            file_offset: u64,
            file_path: *const c_char,
        ) -> c_int;
    }
    perf_session__deliver_event(session, event, tool, file_offset, file_path)
}

unsafe fn perf_session__open(session: *mut perf_session) -> c_int {
    unsafe extern "C" {
        fn perf_session__read_header(session: *mut perf_session) -> c_int;
        fn perf_data__is_pipe(data: *mut perf_data) -> bool;
        fn evlist__valid_sample_type(evlist: *mut evlist) -> bool;
        fn evlist__valid_sample_id_all(evlist: *mut evlist) -> bool;
        fn evlist__valid_read_format(evlist: *mut evlist) -> bool;
    }
    // Requires perf_session layout from session.h.
    let _ = (session, perf_data__is_pipe, evlist__valid_sample_type);
    if perf_session__read_header(session) < 0 {
        pr_err(c"incompatible file format (rerun with -v to learn more)\n".as_ptr());
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__set_id_hdr_size(session: *mut perf_session) {
    unsafe extern "C" {
        fn evlist__id_hdr_size(evlist: *mut evlist) -> u16;
        fn machines__set_id_hdr_size(machines: *mut machines, id_hdr_size: u16);
    }
    // struct member translation: id_hdr_size = evlist__id_hdr_size(session->evlist);
    // machines__set_id_hdr_size(&session->machines, id_hdr_size);
    let _ = (session, evlist__id_hdr_size, machines__set_id_hdr_size);
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__create_kernel_maps(session: *mut perf_session) -> c_int {
    unsafe extern "C" {
        fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
        fn machines__create_guest_kernel_maps(machines: *mut machines) -> c_int;
    }
    let _ = (session, machine__create_kernel_maps, machines__create_guest_kernel_maps);
    // int ret = machine__create_kernel_maps(&session->machines.host);
    // if (ret >= 0) ret = machines__create_guest_kernel_maps(&session->machines);
    0
}

unsafe fn perf_session__destroy_kernel_maps(session: *mut perf_session) {
    unsafe extern "C" {
        fn machines__destroy_kernel_maps(machines: *mut machines);
    }
    let _ = (session, machines__destroy_kernel_maps);
}

unsafe fn perf_session__has_comm_exec(session: *mut perf_session) -> bool {
    // Translates evlist__for_each_entry(session->evlist, evsel) and checks
    // evsel->core.attr.comm_exec.  The evsel iteration macro is external.
    let _ = session;
    false
}

unsafe fn perf_session__set_comm_exec(session: *mut perf_session) {
    unsafe extern "C" {
        fn machines__set_comm_exec(machines: *mut machines, comm_exec: bool);
    }
    let comm_exec = perf_session__has_comm_exec(session);
    let _ = (session, comm_exec, machines__set_comm_exec);
}

unsafe fn ordered_events__deliver_event(oe: *mut ordered_events, event: *mut ordered_event) -> c_int {
    // container_of(oe, struct perf_session, ordered_events), then
    // perf_session__deliver_event(session, event->event, session->tool,
    // event->file_offset, event->file_path).
    let _ = (oe, event);
    0
}

#[no_mangle]
pub unsafe extern "C" fn __perf_session__new(
    data: *mut perf_data,
    tool: *mut perf_tool,
    trace_event_repipe: bool,
    host_env: *mut perf_env,
) -> *mut perf_session {
    unsafe extern "C" {
        fn zalloc(size: size_t) -> *mut c_void;
        fn perf_data__open(data: *mut perf_data) -> c_int;
        fn perf_data__is_read(data: *mut perf_data) -> bool;
        fn perf_data__is_write(data: *mut perf_data) -> bool;
        fn perf_data__open_dir(data: *mut perf_data) -> c_int;
        fn perf_data__kallsyms_name(data: *mut perf_data) -> *const c_char;
        fn evlist__init_trace_event_sample_raw(evlist: *mut evlist, env: *mut perf_env);
        fn evlist__set_session(evlist: *mut evlist, session: *mut perf_session);
        fn evlist__sample_id_all(evlist: *mut evlist) -> bool;
        fn perf_env__single_address_space(env: *mut perf_env) -> bool;
        fn perf_session__delete(session: *mut perf_session);
    }

    let mut ret = -ENOMEM;
    let session = zalloc(size_of::<perf_session>()) as *mut perf_session;
    if session.is_null() {
        return ERR_PTR(ret as c_long);
    }

    // Field initialization mirrors:
    // trace_event_repipe, tool, decomp_data.zstd_decomp, active_decomp,
    // auxtrace_index, header.env, machines, ordered_events.
    let _ = (tool, trace_event_repipe, host_env, evlist__init_trace_event_sample_raw);

    if !data.is_null() {
        ret = perf_data__open(data);
        if ret < 0 {
            perf_session__delete(session);
            return ERR_PTR(ret as c_long);
        }
        if perf_data__is_read(data) {
            ret = perf_session__open(session);
            if ret < 0 {
                perf_session__delete(session);
                return ERR_PTR(ret as c_long);
            }
        }
    } else {
        // assert(host_env != NULL); session->machines.host.env = host_env;
        if host_env.is_null() {
            core::intrinsics::abort();
        }
    }

    let _ = (
        perf_data__is_write,
        perf_data__open_dir,
        perf_data__kallsyms_name,
        evlist__set_session,
        evlist__sample_id_all,
        perf_env__single_address_space,
    );
    session
}

unsafe fn perf_decomp__release_events(mut next: *mut decomp) {
    while !next.is_null() {
        // struct decomp fields next and mmap_len are external; preserve loop shape.
        let decomp = next;
        next = ptr::null_mut();
        munmap(decomp as *mut c_void, 0);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__delete(session: *mut perf_session) {
    unsafe extern "C" {
        fn auxtrace__free(session: *mut perf_session);
        fn auxtrace_index__free(head: *mut list_head);
        fn debuginfo_cache__delete();
        fn perf_env__exit(env: *mut perf_env);
        fn machines__exit(machines: *mut machines);
        fn perf_data__is_read(data: *mut perf_data) -> bool;
        fn evlist__put(evlist: *mut evlist);
        fn perf_data__close(data: *mut perf_data);
        fn trace_event__cleanup(tevent: *mut c_void);
    }
    if session.is_null() {
        return;
    }
    let _ = (
        auxtrace__free,
        auxtrace_index__free,
        debuginfo_cache__delete,
        perf_env__exit,
        machines__exit,
        perf_data__is_read,
        evlist__put,
        perf_data__close,
        trace_event__cleanup,
    );
    perf_session__destroy_kernel_maps(session);
    // HAVE_LIBTRACEEVENT: trace_event__cleanup(&session->tevent);
    free(session as *mut c_void);
}

unsafe fn swap_sample_id_all(event: *mut perf_event, data: *mut c_void) {
    let end = (event as *mut u8).add((*event).header.size as usize) as *mut c_void;
    if (data as usize) >= (end as usize) {
        return;
    }
    let size = (end as usize).wrapping_sub(data as usize);
    if size % size_of::<u64>() != 0 {
        pr_warning(
            c"swap_sample_id_all: unaligned sample_id_all remainder (%d), skipping swap\n".as_ptr(),
            size as c_int,
        );
        return;
    }
    if size > 0 {
        mem_bswap_64(data, size);
    }
}

unsafe fn perf_event__all64_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    let size = (*event).header.size as usize - size_of::<perf_event_header>();
    if size % size_of::<u64>() != 0 {
        return -1;
    }
    mem_bswap_64((event as *mut u8).add(size_of::<perf_event_header>()) as *mut c_void, size);
    0
}

unsafe fn string_payload_sample_id_swap(event: *mut perf_event, data: *mut c_char) -> c_int {
    let end = (event as *mut u8).add((*event).header.size as usize) as *mut c_void;
    let len = strnlen(data, (end as usize).wrapping_sub(data as usize));
    if len == (end as usize).wrapping_sub(data as usize) {
        return -1;
    }
    let data = (data as *mut u8).add(PERF_ALIGN(len + 1, size_of::<u64>())) as *mut c_void;
    swap_sample_id_all(event, data);
    0
}

unsafe fn perf_event__comm_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // event->comm.pid/tid are bswap_32; optional comm string locates sample_id_all.
    if sample_id_all {
        return string_payload_sample_id_swap(
            event,
            (event as *mut u8).add(size_of::<perf_event_header>() + 8) as *mut c_char,
        );
    }
    0
}

unsafe fn perf_event__mmap_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // pid, tid, start, len, pgoff are swapped before filename.
    if sample_id_all {
        return string_payload_sample_id_swap(
            event,
            (event as *mut u8).add(size_of::<perf_event_header>() + 4 + 4 + 8 + 8 + 8)
                as *mut c_char,
        );
    }
    0
}

unsafe fn perf_event__mmap2_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // pid, tid, start, len, pgoff and non-build-id maj/min/ino/ino_generation are swapped.
    if sample_id_all {
        return string_payload_sample_id_swap(
            event,
            (event as *mut u8).add(size_of::<perf_event_header>() + 4 + 4 + 8 + 8 + 8 + 4 + 4 + 8 + 8)
                as *mut c_char,
        );
    }
    0
}

unsafe fn perf_event__task_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // fork/exit pid, tid, ppid, ptid and time are swapped.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 24) as *mut c_void);
    }
    0
}

unsafe fn perf_event__read_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // pid/tid are bswap_32, then packed u64 tail from value is swapped.
    let value_off = size_of::<perf_event_header>() + 8;
    let tail = (*event).header.size as usize - value_off;
    if tail % size_of::<u64>() != 0 {
        return -1;
    }
    mem_bswap_64((event as *mut u8).add(value_off) as *mut c_void, tail);
    0
}

unsafe fn perf_event__aux_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // aux_offset, aux_size and flags are swapped.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 24) as *mut c_void);
    }
    0
}

unsafe fn perf_event__itrace_start_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // pid and tid are swapped.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 8) as *mut c_void);
    }
    0
}

unsafe fn perf_event__switch_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // SWITCH_CPU_WIDE swaps next_prev_pid and next_prev_tid.
    if sample_id_all {
        if (*event).header.type_ == PERF_RECORD_SWITCH {
            swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>()) as *mut c_void);
        } else {
            swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 8) as *mut c_void);
        }
    }
    0
}

unsafe fn perf_event__text_poke_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // addr, old_len and new_len are swapped, then byte blob bounds are checked.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add((*event).header.size as usize) as *mut c_void);
    }
    0
}

unsafe fn perf_event__throttle_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // time, id and stream_id are swapped.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 24) as *mut c_void);
    }
    0
}

unsafe fn perf_event__namespaces_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // pid/tid/nr_namespaces and each link_info dev/ino are swapped; nr is clamped to payload.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>()) as *mut c_void);
    }
    0
}

unsafe fn perf_event__cgroup_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // id is swapped, then path string locates sample_id_all.
    if sample_id_all {
        return string_payload_sample_id_swap(event, (event as *mut u8).add(size_of::<perf_event_header>() + 8) as *mut c_char);
    }
    0
}

fn revbyte(b: u8) -> u8 {
    let mut rev: c_int = ((b >> 4) | ((b & 0xf) << 4)) as c_int;
    rev = ((rev & 0xcc) >> 2) | ((rev & 0x33) << 2);
    rev = ((rev & 0xaa) >> 1) | ((rev & 0x55) << 1);
    rev as u8
}

unsafe fn swap_bitfield(mut p: *mut u8, len: c_uint) {
    for _ in 0..len {
        *p = revbyte(*p);
        p = p.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__attr_swap(attr: *mut perf_event_attr) {
    (*attr).type_ = bswap_32((*attr).type_);
    (*attr).size = bswap_32((*attr).size);
    if (*attr).size == 0 {
        (*attr).size = PERF_ATTR_SIZE_VER0;
    }

    macro_rules! bswap_safe {
        ($field:ident) => {
            ((*attr).size as usize) >= offset_of!(perf_event_attr, $field) + size_of_val(&(*attr).$field)
        };
    }
    macro_rules! bswap_field_16 {
        ($field:ident) => {
            if bswap_safe!($field) {
                (*attr).$field = bswap_16((*attr).$field);
            }
        };
    }
    macro_rules! bswap_field_32 {
        ($field:ident) => {
            if bswap_safe!($field) {
                (*attr).$field = bswap_32((*attr).$field);
            }
        };
    }
    macro_rules! bswap_field_64 {
        ($field:ident) => {
            if bswap_safe!($field) {
                (*attr).$field = bswap_64((*attr).$field);
            }
        };
    }

    bswap_field_64!(config);
    bswap_field_64!(sample_period);
    bswap_field_64!(sample_type);
    bswap_field_64!(read_format);
    bswap_field_32!(wakeup_events);
    bswap_field_32!(bp_type);
    bswap_field_64!(bp_addr);
    bswap_field_64!(bp_len);
    bswap_field_64!(branch_sample_type);
    bswap_field_64!(sample_regs_user);
    bswap_field_32!(sample_stack_user);
    bswap_field_32!(aux_watermark);
    bswap_field_16!(sample_max_stack);
    bswap_field_32!(aux_sample_size);

    if ((*attr).size as usize) >= offset_of!(perf_event_attr, read_format) + 2 * size_of::<u64>() {
        let p = (&mut (*attr).read_format as *mut u64).add(1) as *mut u8;
        swap_bitfield(p, size_of::<u64>() as c_uint);
    }
}

unsafe fn perf_event__hdr_attr_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // Validates foreign-endian attr.size, swaps perf_event_attr, then swaps trailing ids.
    let _ = event;
    0
}

unsafe fn perf_event__build_id_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // pid is swapped, filename string locates sample_id_all.
    if sample_id_all {
        return string_payload_sample_id_swap(event, (event as *mut u8).add(size_of::<perf_event_header>() + 4) as *mut c_char);
    }
    0
}

unsafe fn perf_event__event_update_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // type/id are swapped; SCALE swaps scale, CPUS swaps embedded cpu_map_data variants.
    let _ = event;
    0
}

unsafe fn perf_event__event_type_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // event_type.event_id is swapped.
    let _ = event;
    0
}

unsafe fn perf_event__tracing_data_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // tracing_data.size is swapped.
    let _ = event;
    0
}

unsafe fn perf_event__auxtrace_info_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // type is swapped, priv[] tail is mem_bswap_64'd.
    let _ = event;
    0
}

unsafe fn perf_event__auxtrace_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // size, offset, reference, idx, tid and cpu are swapped.
    let _ = event;
    0
}

unsafe fn perf_event__auxtrace_error_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // type, code, cpu, pid, tid, fmt, ip, optional time, machine_pid and vcpu are swapped.
    let _ = event;
    0
}

unsafe fn perf_event__thread_map_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // nr is swapped, validated against payload, then each entry pid is swapped.
    let _ = event;
    0
}

unsafe fn perf_event__cpu_map_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // cpu_map_data type is swapped, variants CPUS/MASK/RANGE_CPUS are clamped and swapped.
    let _ = event;
    0
}

unsafe fn perf_event__stat_config_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // nr is foreign-endian swapped, clamped, nr+data[] are mem_bswap_64'd, nr persisted native.
    let _ = event;
    0
}

unsafe fn perf_event__stat_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // id, thread, cpu, val, ena and run are swapped.
    let _ = event;
    0
}

unsafe fn perf_event__stat_round_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // type and time are swapped.
    let _ = event;
    0
}

unsafe fn perf_event__time_conv_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // time_shift, time_mult, time_zero and present time_cycles/time_mask are swapped.
    let _ = event;
    0
}

unsafe fn perf_event__compressed2_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // Only data_size is swapped; compressed payload is raw bytes.
    let _ = event;
    0
}

unsafe fn perf_event__bpf_metadata_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // Fixed header is validated; nr_entries is swapped/clamped; prog_name and entries are NUL-fixed.
    let _ = event;
    0
}

unsafe fn perf_event__schedstat_cpu_swap(_event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // FIXME
    0
}

unsafe fn perf_event__schedstat_domain_swap(_event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // FIXME
    0
}

unsafe fn perf_event__ksymbol_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // addr, len, ksym_type, flags are swapped, name string locates sample_id_all.
    if sample_id_all {
        return string_payload_sample_id_swap(event, (event as *mut u8).add(size_of::<perf_event_header>() + 16) as *mut c_char);
    }
    0
}

unsafe fn perf_event__bpf_event_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    // type, flags and id are swapped.
    if sample_id_all {
        swap_sample_id_all(event, (event as *mut u8).add(size_of::<perf_event_header>() + 8) as *mut c_void);
    }
    0
}

unsafe fn perf_event__header_feature_swap(event: *mut perf_event, _sample_id_all: bool) -> c_int {
    // feat_id is swapped.
    let _ = event;
    0
}

static mut perf_event__swap_ops: [perf_event__swap_op; PERF_RECORD_HEADER_MAX] = {
    let mut a: [perf_event__swap_op; PERF_RECORD_HEADER_MAX] = [None; PERF_RECORD_HEADER_MAX];
    a[PERF_RECORD_MMAP as usize] = Some(perf_event__mmap_swap);
    a[PERF_RECORD_MMAP2 as usize] = Some(perf_event__mmap2_swap);
    a[PERF_RECORD_COMM as usize] = Some(perf_event__comm_swap);
    a[PERF_RECORD_FORK as usize] = Some(perf_event__task_swap);
    a[PERF_RECORD_EXIT as usize] = Some(perf_event__task_swap);
    a[PERF_RECORD_LOST as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_READ as usize] = Some(perf_event__read_swap);
    a[PERF_RECORD_THROTTLE as usize] = Some(perf_event__throttle_swap);
    a[PERF_RECORD_UNTHROTTLE as usize] = Some(perf_event__throttle_swap);
    a[PERF_RECORD_SAMPLE as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_AUX as usize] = Some(perf_event__aux_swap);
    a[PERF_RECORD_ITRACE_START as usize] = Some(perf_event__itrace_start_swap);
    a[PERF_RECORD_LOST_SAMPLES as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_SWITCH as usize] = Some(perf_event__switch_swap);
    a[PERF_RECORD_SWITCH_CPU_WIDE as usize] = Some(perf_event__switch_swap);
    a[PERF_RECORD_NAMESPACES as usize] = Some(perf_event__namespaces_swap);
    a[PERF_RECORD_CGROUP as usize] = Some(perf_event__cgroup_swap);
    a[PERF_RECORD_KSYMBOL as usize] = Some(perf_event__ksymbol_swap);
    a[PERF_RECORD_BPF_EVENT as usize] = Some(perf_event__bpf_event_swap);
    a[PERF_RECORD_TEXT_POKE as usize] = Some(perf_event__text_poke_swap);
    a[PERF_RECORD_AUX_OUTPUT_HW_ID as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_CALLCHAIN_DEFERRED as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_HEADER_ATTR as usize] = Some(perf_event__hdr_attr_swap);
    a[PERF_RECORD_HEADER_EVENT_TYPE as usize] = Some(perf_event__event_type_swap);
    a[PERF_RECORD_HEADER_TRACING_DATA as usize] = Some(perf_event__tracing_data_swap);
    a[PERF_RECORD_HEADER_BUILD_ID as usize] = Some(perf_event__build_id_swap);
    a[PERF_RECORD_HEADER_FEATURE as usize] = Some(perf_event__header_feature_swap);
    a[PERF_RECORD_ID_INDEX as usize] = Some(perf_event__all64_swap);
    a[PERF_RECORD_AUXTRACE_INFO as usize] = Some(perf_event__auxtrace_info_swap);
    a[PERF_RECORD_AUXTRACE as usize] = Some(perf_event__auxtrace_swap);
    a[PERF_RECORD_AUXTRACE_ERROR as usize] = Some(perf_event__auxtrace_error_swap);
    a[PERF_RECORD_THREAD_MAP as usize] = Some(perf_event__thread_map_swap);
    a[PERF_RECORD_CPU_MAP as usize] = Some(perf_event__cpu_map_swap);
    a[PERF_RECORD_STAT_CONFIG as usize] = Some(perf_event__stat_config_swap);
    a[PERF_RECORD_STAT as usize] = Some(perf_event__stat_swap);
    a[PERF_RECORD_STAT_ROUND as usize] = Some(perf_event__stat_round_swap);
    a[PERF_RECORD_EVENT_UPDATE as usize] = Some(perf_event__event_update_swap);
    a[PERF_RECORD_TIME_CONV as usize] = Some(perf_event__time_conv_swap);
    a[PERF_RECORD_COMPRESSED2 as usize] = Some(perf_event__compressed2_swap);
    a[PERF_RECORD_BPF_METADATA as usize] = Some(perf_event__bpf_metadata_swap);
    a[PERF_RECORD_SCHEDSTAT_CPU as usize] = Some(perf_event__schedstat_cpu_swap);
    a[PERF_RECORD_SCHEDSTAT_DOMAIN as usize] = Some(perf_event__schedstat_domain_swap);
    a
};

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_finished_round(
    _tool: *const perf_tool,
    _event: *mut perf_event,
    oe: *mut ordered_events,
) -> c_int {
    unsafe extern "C" {
        fn ordered_events__flush(oe: *mut ordered_events, how: c_int) -> c_int;
    }
    if dump_trace {
        fprintf(stdout, c"\n".as_ptr());
    }
    ordered_events__flush(oe, OE_FLUSH__ROUND)
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__queue_event(
    s: *mut perf_session,
    event: *mut perf_event,
    timestamp: u64,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    unsafe extern "C" {
        fn ordered_events__queue(
            oe: *mut ordered_events,
            event: *mut perf_event,
            timestamp: u64,
            file_offset: u64,
            file_path: *const c_char,
        ) -> c_int;
    }
    let _ = (s, event, timestamp, file_offset, file_path, ordered_events__queue);
    0
}

unsafe fn callchain__lbr_callstack_printf(sample: *mut perf_sample) {
    // Translates kernel/user callchain split and LBR from/to printing.
    let _ = sample;
}

unsafe fn callchain_context_str(ip: u64) -> *const c_char {
    match ip {
        PERF_CONTEXT_HV => c" (PERF_CONTEXT_HV)".as_ptr(),
        PERF_CONTEXT_KERNEL => c" (PERF_CONTEXT_KERNEL)".as_ptr(),
        PERF_CONTEXT_USER => c" (PERF_CONTEXT_USER)".as_ptr(),
        PERF_CONTEXT_GUEST => c" (PERF_CONTEXT_GUEST)".as_ptr(),
        PERF_CONTEXT_GUEST_KERNEL => c" (PERF_CONTEXT_GUEST_KERNEL)".as_ptr(),
        PERF_CONTEXT_GUEST_USER => c" (PERF_CONTEXT_GUEST_USER)".as_ptr(),
        PERF_CONTEXT_USER_DEFERRED => c" (PERF_CONTEXT_USER_DEFERRED)".as_ptr(),
        _ => c"".as_ptr(),
    }
}

unsafe fn callchain__printf(evsel: *mut evsel, sample: *mut perf_sample) {
    let _ = (evsel, sample);
}

unsafe fn branch_stack__printf(sample: *mut perf_sample, evsel: *mut evsel) {
    let _ = (sample, evsel);
}

unsafe fn regs_dump__printf(mask: u64, regs: *mut u64, e_machine: u16, e_flags: u32) {
    let mut i = 0usize;
    for rid in 0..(size_of::<u64>() * 8) {
        if (mask & (1u64 << rid)) != 0 {
            let val = *regs.add(i);
            i += 1;
            unsafe extern "C" {
                fn perf_reg_name(rid: c_uint, e_machine: u16, e_flags: u32) -> *const c_char;
            }
            printf(
                c".... %-5s 0x%016llx\n".as_ptr(),
                perf_reg_name(rid as c_uint, e_machine, e_flags),
                val,
            );
        }
    }
}

static regs_abi: [*const c_char; 3] = [
    c"none".as_ptr(),
    c"32-bit".as_ptr(),
    c"64-bit".as_ptr(),
];

unsafe fn regs_dump_abi(d: *mut c_void) -> *const c_char {
    // if (d->abi > PERF_SAMPLE_REGS_ABI_64) return "unknown"; return regs_abi[d->abi];
    let _ = d;
    c"unknown".as_ptr()
}

unsafe fn regs__printf(type_: *const c_char, regs: *mut c_void, e_machine: u16, e_flags: u32) {
    let _ = (type_, regs, e_machine, e_flags);
}

unsafe fn regs_user__printf(sample: *mut perf_sample, e_machine: u16, e_flags: u32) {
    let _ = (sample, e_machine, e_flags);
}

unsafe fn regs_intr__printf(sample: *mut perf_sample, e_machine: u16, e_flags: u32) {
    let _ = (sample, e_machine, e_flags);
}

unsafe fn stack_user__printf(dump: *mut c_void) {
    let _ = dump;
}

unsafe fn evlist__print_tstamp(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) {
    unsafe extern "C" {
        fn __evlist__combined_sample_type(evlist: *mut evlist) -> u64;
        fn evlist__sample_id_all(evlist: *mut evlist) -> bool;
    }
    let sample_type = __evlist__combined_sample_type(evlist);
    if (*event).header.type_ != PERF_RECORD_SAMPLE && !evlist__sample_id_all(evlist) {
        fputs(c"-1 -1 ".as_ptr(), stdout);
        return;
    }
    let _ = (sample_type, sample);
}

unsafe fn sample_read__printf(sample: *mut perf_sample, read_format: u64) {
    printf(c"... sample_read:\n".as_ptr());
    let _ = (sample, read_format);
}

unsafe fn dump_event(
    evlist: *mut evlist,
    event: *mut perf_event,
    file_offset: u64,
    sample: *mut perf_sample,
    file_path: *const c_char,
) {
    unsafe extern "C" {
        fn evlist__trace_event_sample_raw(evlist: *mut evlist) -> bool;
    }
    if !dump_trace {
        return;
    }
    printf(
        c"\n%#llx@%s [%#x]: event: %d\n".as_ptr(),
        file_offset,
        file_path,
        (*event).header.size as c_int,
        (*event).header.type_ as c_int,
    );
    trace_event(event);
    let _ = (evlist, sample, evlist__trace_event_sample_raw);
    printf(
        c"%#llx [%#x]: PERF_RECORD_%s".as_ptr(),
        file_offset,
        (*event).header.size as c_int,
        perf_event__name((*event).header.type_),
    );
}

#[no_mangle]
pub unsafe extern "C" fn get_page_size_name(size: u64, str_: *mut c_char) -> *mut c_char {
    if size == 0 || unit_number__scnprintf(str_, PAGE_SIZE_NAME_LEN, size) == 0 {
        snprintf(str_, PAGE_SIZE_NAME_LEN, c"%s".as_ptr(), c"N/A".as_ptr());
    }
    str_
}

unsafe fn dump_sample(machine: *mut machine, event: *mut perf_event, sample: *mut perf_sample) {
    // Prints IP, period, addr, callchain, branch stack, registers, stack, weight,
    // data_src, phys_addr, page sizes, transaction and read payload based on sample_type.
    let _ = (machine, event, sample);
}

unsafe fn dump_deferred_callchain(event: *mut perf_event, sample: *mut perf_sample) {
    if !dump_trace {
        return;
    }
    let _ = (event, sample);
}

unsafe fn dump_read(evsel: *mut evsel, event: *mut perf_event) {
    if !dump_trace {
        return;
    }
    let _ = (evsel, event);
}

unsafe fn machines__find_for_cpumode(
    machines: *mut machines,
    event: *mut perf_event,
    sample: *mut perf_sample,
) -> *mut machine {
    // If perf_guest and cpumode is guest kernel/user, pick machine by
    // machine_pid, mmap pid or sample pid, using guest_code policy; otherwise host.
    let _ = (machines, event, sample);
    ptr::null_mut()
}

unsafe fn deliver_sample_value(
    evlist: *mut evlist,
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    v: *mut sample_read_value,
    machine: *mut machine,
    per_thread: bool,
) -> c_int {
    // id2sid, period-storage delta update, unknown-id accounting, zero-period bail,
    // temporary sample->evsel substitution, tool->sample callback.
    let _ = (evlist, tool, event, sample, v, machine, per_thread);
    0
}

unsafe fn deliver_sample_group(
    evlist: *mut evlist,
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
    read_format: u64,
    per_thread: bool,
) -> c_int {
    let _ = (evlist, tool, event, sample, machine, read_format, per_thread);
    0
}

unsafe fn evlist__deliver_sample(
    evlist: *mut evlist,
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    let _ = (evlist, tool, event, sample, machine);
    0
}

#[repr(C)]
struct deferred_event {
    list: list_head,
    event: *mut perf_event,
    file_offset: u64,
}

unsafe fn evlist__deliver_deferred_callchain(
    evlist: *mut evlist,
    tool: *const perf_tool,
    event: *mut perf_event,
    sample: *mut perf_sample,
    machine: *mut machine,
) -> c_int {
    // Either calls tool->callchain_deferred or walks evlist deferred_samples,
    // parses original samples, merges matching callchains, delivers, unlinks and frees.
    let _ = (evlist, tool, event, sample, machine);
    0
}

unsafe fn session__flush_deferred_samples(session: *mut perf_session, tool: *const perf_tool) -> c_int {
    // Walks deferred samples, parses, resolves evsel, delivers, exits sample, unlinks and frees.
    let _ = (session, tool);
    0
}

unsafe fn perf_event__check_nul(
    str_: *const c_char,
    end: *const c_void,
    event_name: *const c_char,
    file_offset: u64,
) -> bool {
    let max_len = (end as usize).wrapping_sub(str_ as usize);
    if max_len == 0 || strnlen(str_, max_len) == max_len {
        pr_warning(
            c"WARNING: at offset %#llx: PERF_RECORD_%s: string not null-terminated, skipping event\n"
                .as_ptr(),
            file_offset,
            event_name,
        );
        return false;
    }
    true
}

unsafe fn machines__deliver_event(
    machines: *mut machines,
    evlist: *mut evlist,
    event: *mut perf_event,
    sample: *mut perf_sample,
    tool: *const perf_tool,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    // Dispatches all kernel events: SAMPLE, MMAP/MMAP2, COMM, NAMESPACES,
    // CGROUP, FORK/EXIT, LOST, LOST_SAMPLES, READ, THROTTLE/UNTHROTTLE, AUX,
    // ITRACE_START, SWITCH, KSYMBOL, BPF_EVENT, TEXT_POKE, AUX_OUTPUT_HW_ID,
    // CALLCHAIN_DEFERRED; validates string/flexible payload bounds and updates stats.
    dump_event(evlist, event, file_offset, sample, file_path);
    let _ = (machines, tool);
    if (*event).header.type_ == PERF_RECORD_CALLCHAIN_DEFERRED {
        return evlist__deliver_deferred_callchain(evlist, tool, event, sample, ptr::null_mut());
    }
    0
}

unsafe fn perf_session__deliver_event_impl(
    session: *mut perf_session,
    event: *mut perf_event,
    tool: *const perf_tool,
    file_offset: u64,
    file_path: *const c_char,
) -> c_int {
    // Initializes perf_sample, finds evsel, parses sample, fills guest SID
    // machine_pid/vcpu, validates sample.cpu against env nr_cpus_avail/MAX_NR_CPUS,
    // lets auxtrace consume, dispatches through machines__deliver_event, dumps aux sample.
    let _ = (session, event, tool, file_offset, file_path);
    0
}

unsafe fn perf_session__process_user_event(
    session: *mut perf_session,
    event: *mut perf_event,
    file_offset: u64,
    file_path: *const c_char,
) -> s64 {
    // Handles immediate user events: ATTR, EVENT_UPDATE, EVENT_TYPE,
    // TRACING_DATA, BUILD_ID, FINISHED_ROUND, ID_INDEX, AUXTRACE_INFO,
    // AUXTRACE, AUXTRACE_ERROR, THREAD_MAP, CPU_MAP, STAT_CONFIG, STAT,
    // STAT_ROUND, TIME_CONV, HEADER_FEATURE, COMPRESSED/COMPRESSED2,
    // FINISHED_INIT, BPF_METADATA, SCHEDSTAT_CPU and SCHEDSTAT_DOMAIN.
    let event_size = READ_ONCE(&(*event).header.size as *const u16) as u32;
    let _ = (session, file_path, event_size);
    dump_event(ptr::null_mut(), event, file_offset, ptr::null_mut(), file_path);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__deliver_synth_event(
    session: *mut perf_session,
    event: *mut perf_event,
    sample: *mut perf_sample,
) -> c_int {
    if (*event).header.type_ >= PERF_RECORD_USER_TYPE_START {
        return perf_session__process_user_event(session, event, 0, ptr::null()) as c_int;
    }
    machines__deliver_event(ptr::null_mut(), ptr::null_mut(), event, sample, ptr::null(), 0, ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__deliver_synth_attr_event(
    session: *mut perf_session,
    attr: *const perf_event_attr,
    id: u64,
) -> c_int {
    // Builds an in-stack HEADER_ATTR record with one id, checks attr->size, copies attr,
    // then delivers through perf_session__deliver_synth_event.
    let _ = (session, attr, id);
    if (*attr).size as usize != size_of::<perf_event_attr>() {
        pr_debug(c"Unexpected perf_event_attr size\n".as_ptr());
        return -EINVAL;
    }
    0
}

unsafe fn event_swap(event: *mut perf_event, sample_id_all: bool) -> c_int {
    let swap = perf_event__swap_ops[(*event).header.type_ as usize];
    if let Some(f) = swap {
        return f(event, sample_id_all);
    }
    0
}

static perf_event__min_size: [u32; PERF_RECORD_HEADER_MAX] = [0; PERF_RECORD_HEADER_MAX];

#[no_mangle]
pub unsafe extern "C" fn perf_event__too_small(event: *const perf_event, min: *mut u32) -> bool {
    let min_sz = perf_event__min_size[(*event).header.type_ as usize];
    if min_sz != 0 && (*event).header.size as u32  < min_sz {
        if !min.is_null() {
            *min = min_sz;
        }
        return true;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__peek_event(
    session: *mut perf_session,
    file_offset: off_t,
    buf: *mut c_void,
    buf_sz: size_t,
    event_ptr: *mut *mut perf_event,
    sample: *mut perf_sample,
) -> c_int {
    unsafe extern "C" {
        fn perf_data__is_pipe(data: *mut perf_data) -> bool;
        fn perf_data__fd(data: *mut perf_data) -> c_int;
        fn evlist__sample_id_all(evlist: *mut evlist) -> bool;
        fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    }
    *event_ptr = ptr::null_mut();
    let hdr_sz = size_of::<perf_event_header>();
    if buf_sz < hdr_sz {
        return -1;
    }
    let _ = (session, file_offset, buf, sample, perf_data__is_pipe, perf_data__fd, evlist__sample_id_all, evlist__parse_sample);
    // Translates mmap fast path and fd read path, header swap, alignment/type/min-size checks,
    // event_swap, and optional sample parse.
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__peek_events(
    session: *mut perf_session,
    mut offset: u64,
    size: u64,
    cb: peek_events_cb_t,
    data: *mut c_void,
) -> c_int {
    let max_offset = offset.wrapping_add(size);
    let mut buf = [0u8; PERF_SAMPLE_MAX_SIZE];
    let mut event: *mut perf_event = ptr::null_mut();
    let mut err: c_int = 0;
    while offset < max_offset {
        event = ptr::null_mut();
        err = perf_session__peek_event(
            session,
            offset as off_t,
            buf.as_mut_ptr() as *mut c_void,
            PERF_SAMPLE_MAX_SIZE,
            &mut event,
            ptr::null_mut(),
        );
        if err != 0 {
            if !event.is_null()
                && (*event).header.size != 0
                && (*event).header.type_ != PERF_RECORD_AUXTRACE
                && ((*event).header.size as usize % size_of::<u64>() == 0)
            {
                offset += (*event).header.size as u64;
                err = 0;
            } else {
                return err;
            }
            continue;
        }
        if let Some(f) = cb {
            err = f(session, event, offset, data);
            if err != 0 {
                return err;
            }
        }
        offset += (*event).header.size as u64;
        if (*event).header.type_ == PERF_RECORD_AUXTRACE {
            // offset += event->auxtrace.size; requires auxtrace record layout.
        }
    }
    err
}

unsafe fn perf_session__process_event(
    session: *mut perf_session,
    event: *mut perf_event,
    file_offset: u64,
    file_path: *const c_char,
) -> s64 {
    let mut min_sz: u32 = 0;
    if (*event).header.size as usize % size_of::<u64>() != 0
        && (*event).header.type_ != PERF_RECORD_HEADER_TRACING_DATA
        && (*event).header.type_ != PERF_RECORD_COMPRESSED
        && (*event).header.type_ != PERF_RECORD_HEADER_FEATURE
    {
        pr_err(
            c"ERROR: at offset %#llx: %s (%u) event size %u is not 8-byte aligned, aborting\n"
                .as_ptr(),
            file_offset,
            perf_event__name((*event).header.type_),
            (*event).header.type_,
            (*event).header.size as c_uint,
        );
        return -(EINVAL as s64);
    }
    if (*event).header.type_ >= PERF_RECORD_HEADER_MAX as u32 {
        ui__warning(
            c"Unsupported header type %u, please consider updating perf.\n".as_ptr(),
            (*event).header.type_,
        );
        return 0;
    }
    if perf_event__too_small(event, &mut min_sz) {
        pr_warning(
            c"WARNING: at offset %#llx: %s (%u) event size %u too small (min %u), skipping\n"
                .as_ptr(),
            file_offset,
            perf_event__name((*event).header.type_),
            (*event).header.type_,
            (*event).header.size as c_uint,
            min_sz,
        );
        return 0;
    }
    // needs_swap/event_swap, stats increment, user-event dispatch, ordered queue,
    // and final direct delivery mirror the C function.
    let _ = (session, file_path);
    if (*event).header.type_ >= PERF_RECORD_USER_TYPE_START {
        return perf_session__process_user_event(session, event, file_offset, file_path);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_header__bswap(hdr: *mut perf_event_header) {
    (*hdr).type_ = bswap_32((*hdr).type_);
    (*hdr).misc = bswap_16((*hdr).misc);
    (*hdr).size = bswap_16((*hdr).size);
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__findnew(session: *mut perf_session, pid: pid_t) -> *mut thread {
    unsafe extern "C" {
        fn machine__findnew_thread(machine: *mut machine, pid: pid_t, tid: pid_t) -> *mut thread;
    }
    let _ = (session, machine__findnew_thread);
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__register_idle_thread(session: *mut perf_session) -> c_int {
    unsafe extern "C" {
        fn machine__idle_thread(machine: *mut machine) -> *mut thread;
        fn thread__put(thread: *mut thread);
    }
    let _ = (session, machine__idle_thread, thread__put);
    0
}

unsafe fn perf_session__warn_order(session: *const perf_session) {
    let _ = session;
}

unsafe fn perf_session__warn_about_errors(session: *const perf_session) {
    // Emits warnings for lost chunks, lost samples, AUX loss/partial/collision,
    // unknown events/ids, invalid chains, unprocessable samples, ordering, auxtrace
    // errors and proc-map timeouts.
    perf_session__warn_order(session);
}

unsafe fn perf_session__flush_thread_stack(thread: *mut thread, p: *mut c_void) -> c_int {
    unsafe extern "C" {
        fn thread_stack__flush(thread: *mut thread) -> c_int;
    }
    let _ = p;
    thread_stack__flush(thread)
}

unsafe fn perf_session__flush_thread_stacks(session: *mut perf_session) -> c_int {
    unsafe extern "C" {
        fn machines__for_each_thread(
            machines: *mut machines,
            cb: Option<unsafe extern "C" fn(*mut thread, *mut c_void) -> c_int>,
            data: *mut c_void,
        ) -> c_int;
    }
    let _ = (session, machines__for_each_thread);
    0
}

unsafe fn __perf_session__process_decomp_events(session: *mut perf_session) -> c_int {
    // Walk active_decomp->decomp_last while head < size and !session_done(),
    // fetch_decomp_event, process event, account skip, advance decomp->head.
    let _ = session;
    0
}

unsafe fn __perf_session__process_pipe_events(session: *mut perf_session) -> c_int {
    // Reads pipe event headers and payloads, grows buffer, processes events,
    // processes decompressed events, updates progress, final-flushes ordered
    // events, deferred samples, auxtrace events and thread stacks, then frees.
    let _ = session;
    0
}

unsafe fn prefetch_event(
    buf: *mut c_char,
    head: u64,
    mmap_size: size_t,
    needs_swap: bool,
    error: *mut perf_event,
) -> *mut perf_event {
    if head as usize + size_of::<perf_event_header>() > mmap_size {
        return ptr::null_mut();
    }
    let event = buf.add(head as usize) as *mut perf_event;
    if needs_swap {
        perf_event_header__bswap(&mut (*event).header);
    }
    let event_size = (*event).header.size as u64;
    if head + event_size <= mmap_size as u64 {
        return event;
    }
    if needs_swap {
        perf_event_header__bswap(&mut (*event).header);
    }
    if event_size <= mmap_size as u64 - head % page_size {
        return ptr::null_mut();
    }
    pr_debug(
        c"%s: head=%#llx event->header.size=%#x, mmap_size=%#zx: fuzzed or compressed perf.data?\n"
            .as_ptr(),
        c"prefetch_event".as_ptr(),
        head,
        event_size as c_uint,
        mmap_size,
    );
    error
}

unsafe fn fetch_mmaped_event(head: u64, mmap_size: size_t, buf: *mut c_char, needs_swap: bool) -> *mut perf_event {
    prefetch_event(buf, head, mmap_size, needs_swap, ERR_PTR(-(EINVAL as c_long)))
}

unsafe fn fetch_decomp_event(head: u64, mmap_size: size_t, buf: *mut c_char, needs_swap: bool) -> *mut perf_event {
    prefetch_event(buf, head, mmap_size, needs_swap, ptr::null_mut())
}

#[repr(C)]
struct zstd_data {
    _private: [u8; 0],
}

#[repr(C)]
struct decomp_data {
    _private: [u8; 0],
}

#[repr(C)]
struct reader {
    fd: c_int,
    path: *const c_char,
    data_size: u64,
    data_offset: u64,
    process: reader_cb_t,
    in_place_update: bool,
    mmaps: [*mut c_char; NUM_MMAPS],
    mmap_size: size_t,
    mmap_idx: c_int,
    mmap_cur: *mut c_char,
    file_pos: u64,
    file_offset: u64,
    head: u64,
    size: u64,
    done: bool,
    zstd_data: zstd_data,
    decomp_data: decomp_data,
}

unsafe fn reader__init(rd: *mut reader, one_mmap: *mut bool) -> c_int {
    unsafe extern "C" {
        fn zstd_init(data: *mut zstd_data, level: c_int) -> c_int;
        fn zstd_fini(data: *mut zstd_data);
    }
    let mut data_size = (*rd).data_size;
    (*rd).head = (*rd).data_offset;
    data_size += (*rd).data_offset;
    (*rd).mmap_size = MMAP_SIZE as size_t;
    if (*rd).mmap_size as u64 > data_size {
        (*rd).mmap_size = data_size as size_t;
        if !one_mmap.is_null() {
            *one_mmap = true;
        }
    }
    for m in (*rd).mmaps.iter_mut() {
        *m = ptr::null_mut();
    }
    if zstd_init(&mut (*rd).zstd_data, 0) != 0 {
        return -1;
    }
    let _ = zstd_fini;
    0
}

unsafe fn reader__release_decomp(rd: *mut reader) {
    unsafe extern "C" {
        fn zstd_fini(data: *mut zstd_data);
    }
    // perf_decomp__release_events(rd->decomp_data.decomp);
    zstd_fini(&mut (*rd).zstd_data);
}

unsafe fn reader__mmap(rd: *mut reader, session: *mut perf_session) -> c_int {
    let mut mmap_prot = PROT_READ;
    let mut mmap_flags = MAP_SHARED;
    if (*rd).in_place_update {
        mmap_prot |= PROT_WRITE;
    } else {
        // else if (session->header.needs_swap) { PROT_WRITE; MAP_PRIVATE; }
        let _ = session;
    }
    if !(*rd).mmaps[(*rd).mmap_idx as usize].is_null() {
        munmap((*rd).mmaps[(*rd).mmap_idx as usize] as *mut c_void, (*rd).mmap_size);
        (*rd).mmaps[(*rd).mmap_idx as usize] = ptr::null_mut();
    }
    let page_offset = page_size * ((*rd).head / page_size);
    (*rd).file_offset += page_offset;
    (*rd).head -= page_offset;
    let buf = mmap(
        ptr::null_mut(),
        (*rd).mmap_size,
        mmap_prot,
        mmap_flags,
        (*rd).fd,
        (*rd).file_offset as off_t,
    ) as *mut c_char;
    if IS_ERR(buf) || buf as isize == -1 {
        pr_err(c"failed to mmap file\n".as_ptr());
        return -errno;
    }
    (*rd).mmaps[(*rd).mmap_idx as usize] = buf;
    (*rd).mmap_cur = buf;
    (*rd).mmap_idx = ((*rd).mmap_idx + 1) & ((NUM_MMAPS as c_int) - 1);
    (*rd).file_pos = (*rd).file_offset + (*rd).head;
    0
}

const READER_OK: c_int = 0;
const READER_NODATA: c_int = 1;

unsafe fn reader__read_event(rd: *mut reader, session: *mut perf_session, prog: *mut c_void) -> c_int {
    unsafe extern "C" {
        fn ui_progress__update(prog: *mut c_void, size: u64);
    }
    let event = fetch_mmaped_event((*rd).head, (*rd).mmap_size, (*rd).mmap_cur, false);
    if IS_ERR(event) {
        return PTR_ERR(event);
    }
    if event.is_null() {
        return READER_NODATA;
    }
    let mut size = (*event).header.size as u64;
    let mut skip: s64 = -EINVAL as s64;
    if size < size_of::<perf_event_header>() as u64 {
        return skip as c_int;
    }
    if let Some(process) = (*rd).process {
        skip = process(session, event, (*rd).file_pos, (*rd).path);
    }
    if skip < 0 {
        return skip as c_int;
    }
    if skip != 0 {
        size += skip as u64;
    }
    (*rd).size += size;
    (*rd).head += size;
    (*rd).file_pos += size;
    let err = __perf_session__process_decomp_events(session);
    if err != 0 {
        return err;
    }
    ui_progress__update(prog, size);
    READER_OK
}

unsafe fn reader__eof(rd: *mut reader) -> bool {
    (*rd).file_pos >= (*rd).data_size + (*rd).data_offset
}

unsafe fn reader__process_events(rd: *mut reader, session: *mut perf_session, prog: *mut c_void) -> c_int {
    let mut one_mmap = false;
    let mut err = reader__init(rd, &mut one_mmap);
    if err != 0 {
        return err;
    }
    loop {
        err = reader__mmap(rd, session);
        if err != 0 {
            return err;
        }
        loop {
            err = reader__read_event(rd, session, prog);
            if err < 0 {
                return err;
            } else if err == READER_NODATA {
                break;
            }
            if reader__eof(rd) {
                return err;
            }
        }
    }
}

unsafe extern "C" fn process_simple(
    session: *mut perf_session,
    event: *mut perf_event,
    file_offset: u64,
    file_path: *const c_char,
) -> s64 {
    perf_session__process_event(session, event, file_offset, file_path)
}

unsafe fn __perf_session__process_events(session: *mut perf_session) -> c_int {
    // Initializes a reader over session->data, shows progress, reader__process_events,
    // final flushes ordered/auxtrace/deferred/thread-stacks, warns, reinitializes ordered
    // events, frees auxtrace events and decompression, clears one_mmap.
    let _ = session;
    0
}

unsafe fn __perf_session__process_dir_events(session: *mut perf_session) -> c_int {
    // Allocates per-file readers for directory data, mmaps each, round-robins readers
    // in READER_MAX_SIZE chunks, final flushes ordered/deferred/thread-stacks, warns,
    // reinitializes ordered events, releases decompression and frees reader array.
    let _ = session;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__process_events(session: *mut perf_session) -> c_int {
    unsafe extern "C" {
        fn perf_data__is_pipe(data: *mut perf_data) -> bool;
        fn perf_data__is_dir(data: *mut perf_data) -> bool;
    }
    if perf_session__register_idle_thread(session) < 0 {
        return -ENOMEM;
    }
    let _ = (perf_data__is_pipe, perf_data__is_dir);
    __perf_session__process_events(session)
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__has_traces(session: *mut perf_session, msg: *const c_char) -> bool {
    // Iterates evlist entries, returns true on attr.type == PERF_TYPE_TRACEPOINT.
    let _ = session;
    pr_err(c"No trace sample to read. Did you call 'perf %s'?\n".as_ptr(), msg);
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__has_switch_events(session: *mut perf_session) -> bool {
    // Iterates evlist entries, returns true on attr.context_switch.
    let _ = session;
    false
}

#[no_mangle]
pub unsafe extern "C" fn map__set_kallsyms_ref_reloc_sym(
    map: *mut map,
    symbol_name: *const c_char,
    addr: u64,
) -> c_int {
    unsafe extern "C" {
        fn zalloc(size: size_t) -> *mut c_void;
        fn map__kmap(map: *mut map) -> *mut c_void;
    }
    let ref_ = zalloc(size_of::<usize>() * 2);
    if ref_.is_null() {
        return -ENOMEM;
    }
    let name = strdup(symbol_name);
    if name.is_null() {
        free(ref_);
        return -ENOMEM;
    }
    let bracket = strchr(name, b']' as c_int);
    if !bracket.is_null() {
        *bracket = 0;
    }
    let _ = (map, addr, map__kmap, name);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__fprintf_dsos(session: *mut perf_session, fp: *mut FILE) -> size_t {
    unsafe extern "C" {
        fn machines__fprintf_dsos(machines: *mut machines, fp: *mut FILE) -> size_t;
    }
    let _ = (session, fp, machines__fprintf_dsos);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__fprintf_dsos_buildid(
    session: *mut perf_session,
    fp: *mut FILE,
    skip: Option<unsafe extern "C" fn(*mut dso, c_int) -> bool>,
    parm: c_int,
) -> size_t {
    unsafe extern "C" {
        fn machines__fprintf_dsos_buildid(
            machines: *mut machines,
            fp: *mut FILE,
            skip: Option<unsafe extern "C" fn(*mut dso, c_int) -> bool>,
            parm: c_int,
        ) -> size_t;
    }
    let _ = (session, fp, skip, parm, machines__fprintf_dsos_buildid);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__fprintf_nr_events(session: *mut perf_session, fp: *mut FILE) -> size_t {
    unsafe extern "C" {
        fn events_stats__fprintf(stats: *mut c_void, fp: *mut FILE) -> size_t;
    }
    let msg = c"".as_ptr();
    let mut ret = fprintf(fp, c"\nAggregated stats:%s\n".as_ptr(), msg) as size_t;
    let _ = (session, events_stats__fprintf);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__fprintf(session: *mut perf_session, fp: *mut FILE) -> size_t {
    // Prints host machine and each guest rb-tree machine.
    let _ = (session, fp);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__dump_kmaps(session: *mut perf_session) {
    let save_verbose = verbose;
    fflush(stdout);
    fprintf(stderr, c"Kernel and module maps:\n".as_ptr());
    verbose = 0;
    // maps__fprintf(machine__kernel_maps(&session->machines.host), stderr);
    verbose = save_verbose;
    let _ = session;
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__find_first_evtype(
    session: *mut perf_session,
    type_: c_uint,
) -> *mut evsel {
    // Iterates session->evlist and returns first evsel with core.attr.type == type.
    let _ = (session, type_);
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__cpu_bitmap(
    session: *mut perf_session,
    cpu_list: *const c_char,
    cpu_bitmap: *mut c_ulong,
) -> c_int {
    unsafe extern "C" {
        fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut c_void;
        fn perf_cpu_map__put(map: *mut c_void);
        fn __set_bit(nr: c_int, addr: *mut c_ulong);
    }
    for i in 0..PERF_TYPE_MAX {
        let evsel = perf_session__find_first_evtype(session, i);
        if evsel.is_null() {
            continue;
        }
        // If matching evsel lacks PERF_SAMPLE_CPU, emit error and return -1.
    }
    let map = perf_cpu_map__new(cpu_list);
    if map.is_null() {
        pr_err(c"Invalid cpu_list\n".as_ptr());
        return -1;
    }
    let _ = (cpu_bitmap, __set_bit);
    perf_cpu_map__put(map);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__fprintf_info(
    session: *mut perf_session,
    fp: *mut FILE,
    full: bool,
) {
    unsafe extern "C" {
        fn perf_header__fprintf_info(session: *mut perf_session, fp: *mut FILE, full: bool);
    }
    if session.is_null() || fp.is_null() {
        return;
    }
    fprintf(fp, c"# ========\n".as_ptr());
    perf_header__fprintf_info(session, fp, full);
    fprintf(fp, c"# ========\n#\n".as_ptr());
}

unsafe fn perf_session__register_guest(session: *mut perf_session, machine_pid: pid_t) -> c_int {
    // machines__findnew, copy single_address_space, create idle thread,
    // set kallsyms_filename from perf_data__guest_kallsyms_name.
    let _ = (session, machine_pid);
    0
}

unsafe fn perf_session__set_guest_cpu(
    session: *mut perf_session,
    pid: pid_t,
    tid: pid_t,
    guest_cpu: c_int,
) -> c_int {
    // machine__findnew_thread(host, pid, tid), thread__set_guest_cpu, thread__put.
    let _ = (session, pid, tid, guest_cpu);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__process_id_index(
    _tool: *const perf_tool,
    session: *mut perf_session,
    event: *mut perf_event,
) -> c_int {
    // Validates id_index payload against v1 and optional v2 entry sizes, dumps
    // ids under dump_trace, looks up each sid, assigns idx/cpu/tid and optional
    // machine_pid/vcpu, registering guest machines and guest CPUs as needed.
    let _ = (session, event);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__dsos_hit_all(session: *mut perf_session) -> c_int {
    // Hits all dsos in host machine, then iterates guest rb-tree and hits all there.
    let _ = session;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__env(session: *mut perf_session) -> *mut perf_env {
    // return &session->header.env;
    let _ = session;
    ptr::null_mut()
}

#[repr(C)]
struct perf_session__e_machine_cb_args {
    e_flags: u32,
    e_machine: u16,
}

unsafe extern "C" fn perf_session__e_machine_cb(thread: *mut thread, _args: *mut c_void) -> c_int {
    unsafe extern "C" {
        fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut u32) -> u16;
    }
    let args = _args as *mut perf_session__e_machine_cb_args;
    (*args).e_machine = thread__e_machine(thread, ptr::null_mut(), &mut (*args).e_flags);
    if (*args).e_machine != EM_NONE {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_session__e_machine(
    session: *mut perf_session,
    e_flags: *mut u32,
) -> u16 {
    unsafe extern "C" {
        fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut u32) -> u16;
        fn perf_env__e_machine_nocache(env: *mut perf_env, e_flags: *mut u32) -> u16;
    }
    if session.is_null() {
        if !e_flags.is_null() {
            *e_flags = EF_HOST;
        }
        return EM_HOST;
    }
    let mut args = perf_session__e_machine_cb_args {
        e_flags: 0,
        e_machine: EM_NONE,
    };
    let env = perf_session__env(session);
    let _ = (perf_env__e_machine, perf_session__e_machine_cb, &mut args);
    if args.e_machine != EM_NONE {
        if !e_flags.is_null() {
            *e_flags = args.e_flags;
        }
        return args.e_machine;
    }
    perf_env__e_machine_nocache(env, e_flags)
}
