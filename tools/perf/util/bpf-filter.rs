// SPDX-License-Identifier: GPL-2.0
/**
 * Generic event filter for sampling events in BPF.
 *
 * The BPF program is fixed and just to read filter expressions in the 'filters'
 * map and compare the sample data in order to reject samples that don't match.
 * Each filter expression contains a sample flag (term) to compare, an operation
 * (==, >=, and so on) and a value.
 *
 * Note that each entry has an array of filter expressions and it only succeeds
 * when all of the expressions are satisfied.  But it supports the logical OR
 * using a GROUP operation which is satisfied when any of its member expression
 * is evaluated to true.  But it doesn't allow nested GROUP operations for now.
 *
 * To support non-root users, the filters map can be loaded and pinned in the BPF
 * filesystem by root (perf record --setup-filter pin).  Then each user will get
 * a new entry in the shared filters map to fill the filter expressions.  And the
 * BPF program will find the filter using (task-id, event-id) as a key.
 *
 * The pinned BPF object (shared for regular users) has:
 *
 *                  event_hash                   |
 *                  |        |                   |
 *   event->id ---> |   id   | ---+   idx_hash   |     filters
 *                  |        |    |   |      |   |    |       |
 *                  |  ....  |    +-> |  idx | --+--> | exprs | --->  perf_bpf_filter_entry[]
 *                                |   |      |   |    |       |               .op
 *   task id (tgid) --------------+   | .... |   |    |  ...  |               .term (+ part)
 *                                               |                            .value
 *                                               |
 *   ======= (root would skip this part) ========                     (compares it in a loop)
 *
 * This is used for per-task use cases while system-wide profiling (normally from
 * root user) uses a separate copy of the program and the maps for its own so that
 * it can proceed even if a lot of non-root users are using the filters at the
 * same time.  In this case the filters map has a single entry and no need to use
 * the hash maps to get the index (key) of the filters map (IOW it's always 0).
 *
 * The BPF program returns 1 to accept the sample or 0 to drop it.
 * The 'dropped' map is to keep how many samples it dropped by the filter and
 * it will be reported as lost samples.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;
type size_t = usize;
type mode_t = c_uint;
type YY_BUFFER_STATE = *mut c_void;

const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const CAP_BPF: c_int = 39;
const E2BIG: c_int = 7;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const O_PATH: c_int = 0o10000000;
const PERF_EVENT_IOC_ID: c_ulong = 0x80082407;
const PERF_EVENT_IOC_SET_BPF: c_ulong = 0x40042408;

const MAX_FILTERS: c_int = 64;
const MAX_EVT_HASH: c_int = 4096;
const MAX_IDX_HASH: c_int = 4096;

static PERF_BPF_FILTER_PIN_PATH: &[u8] = b"perf_filter\0";

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct xyarray {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    pub sample_type: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub fd: *mut xyarray,
    pub attr: perf_event_attr,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub bpf_filters: list_head,
    pub bpf_skel: *mut sample_filter_bpf,
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sample_filter_bpf_maps {
    pub filters: *mut bpf_map,
    pub event_hash: *mut bpf_map,
    pub idx_hash: *mut bpf_map,
    pub dropped: *mut bpf_map,
}

#[repr(C)]
pub struct sample_filter_bpf_progs {
    pub perf_sample_filter: *mut bpf_program,
}

#[repr(C)]
pub struct sample_filter_bpf_rodata {
    pub use_idx_hash: c_int,
}

#[repr(C)]
pub struct sample_filter_bpf {
    pub obj: *mut bpf_object,
    pub maps: sample_filter_bpf_maps,
    pub progs: sample_filter_bpf_progs,
    pub rodata: *mut sample_filter_bpf_rodata,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_term {
    PBF_TERM_IP,
    PBF_TERM_TID,
    PBF_TERM_PERIOD,
    PBF_TERM_ID,
    PBF_TERM_CPU,
    PBF_TERM_TIME,
    PBF_TERM_ADDR,
    PBF_TERM_DATA_SRC,
    PBF_TERM_PHYS_ADDR,
    PBF_TERM_WEIGHT,
    PBF_TERM_WEIGHT_STRUCT,
    PBF_TERM_TRANSACTION,
    PBF_TERM_CODE_PAGE_SIZE,
    PBF_TERM_DATA_PAGE_SIZE,
    PBF_TERM_CGROUP,
    PBF_TERM_UID,
    PBF_TERM_GID,
}

const PBF_TERM_SAMPLE_START: perf_bpf_filter_term = perf_bpf_filter_term::PBF_TERM_IP;
const PBF_TERM_SAMPLE_END: perf_bpf_filter_term = perf_bpf_filter_term::PBF_TERM_CGROUP;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpf_filter_op {
    PBF_OP_DONE,
    PBF_OP_GROUP_BEGIN,
    PBF_OP_GROUP_END,
}

#[repr(C)]
pub struct perf_bpf_filter_expr {
    pub list: list_head,
    pub groups: list_head,
    pub term: perf_bpf_filter_term,
    pub part: c_int,
    pub op: perf_bpf_filter_op,
    pub val: c_ulong,
}

#[repr(C)]
pub struct perf_bpf_filter_entry {
    pub op: perf_bpf_filter_op,
    pub part: c_int,
    pub term: perf_bpf_filter_term,
    pub value: c_ulong,
}

#[repr(C)]
pub struct idx_hash_key {
    pub evt_id: u64,
    pub tgid: c_int,
}

/* Index in the pinned 'filters' map.  Should be released after use. */
#[repr(C)]
pub struct pinned_filter_idx {
    pub list: list_head,
    pub evsel: *mut evsel,
    pub event_id: u64,
    pub hash_idx: c_int,
}

static mut pinned_filters: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};

#[repr(C)]
struct perf_sample_info {
    type_: perf_bpf_filter_term,
    name: *const c_char,
    option: *const c_char,
}

static sample_table: [perf_sample_info; 15] = [
    /* default sample flags */
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_IP, name: b"PERF_SAMPLE_IP\0".as_ptr() as *const c_char, option: ptr::null() },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_TID, name: b"PERF_SAMPLE_TID\0".as_ptr() as *const c_char, option: ptr::null() },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_PERIOD, name: b"PERF_SAMPLE_PERIOD\0".as_ptr() as *const c_char, option: ptr::null() },
    /* flags mostly set by default, but still have options */
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_ID, name: b"PERF_SAMPLE_ID\0".as_ptr() as *const c_char, option: b"--sample-identifier\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_CPU, name: b"PERF_SAMPLE_CPU\0".as_ptr() as *const c_char, option: b"--sample-cpu\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_TIME, name: b"PERF_SAMPLE_TIME\0".as_ptr() as *const c_char, option: b"-T\0".as_ptr() as *const c_char },
    /* optional sample flags */
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_ADDR, name: b"PERF_SAMPLE_ADDR\0".as_ptr() as *const c_char, option: b"-d\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_DATA_SRC, name: b"PERF_SAMPLE_DATA_SRC\0".as_ptr() as *const c_char, option: b"-d\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_PHYS_ADDR, name: b"PERF_SAMPLE_PHYS_ADDR\0".as_ptr() as *const c_char, option: b"--phys-data\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_WEIGHT, name: b"PERF_SAMPLE_WEIGHT\0".as_ptr() as *const c_char, option: b"-W\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_WEIGHT_STRUCT, name: b"PERF_SAMPLE_WEIGHT_STRUCT\0".as_ptr() as *const c_char, option: b"-W\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_TRANSACTION, name: b"PERF_SAMPLE_TRANSACTION\0".as_ptr() as *const c_char, option: b"--transaction\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_CODE_PAGE_SIZE, name: b"PERF_SAMPLE_CODE_PAGE_SIZE\0".as_ptr() as *const c_char, option: b"--code-page-size\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_DATA_PAGE_SIZE, name: b"PERF_SAMPLE_DATA_PAGE_SIZE\0".as_ptr() as *const c_char, option: b"--data-page-size\0".as_ptr() as *const c_char },
    perf_sample_info { type_: perf_bpf_filter_term::PBF_TERM_CGROUP, name: b"PERF_SAMPLE_CGROUP\0".as_ptr() as *const c_char, option: b"--all-cgroups\0".as_ptr() as *const c_char },
];

unsafe extern "C" {
    static mut errno: c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn procfs__read_str(path: *const c_char, buf: *mut *mut c_char, len: *mut size_t) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn xyarray__entry(array: *mut xyarray, x: c_int, y: c_int) -> *mut c_void;
    fn xyarray__max_x(array: *mut xyarray) -> c_int;
    fn xyarray__max_y(array: *mut xyarray) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_obj_get(pathname: *const c_char) -> c_int;
    fn perf_evsel__threads(core: *mut evsel_core) -> *mut perf_thread_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: c_int) -> c_int;
    fn target__has_cpu(target: *mut target) -> bool;
    fn geteuid() -> c_uint;
    fn sample_filter_bpf__open_and_load() -> *mut sample_filter_bpf;
    fn sample_filter_bpf__open() -> *mut sample_filter_bpf;
    fn sample_filter_bpf__load(skel: *mut sample_filter_bpf) -> c_int;
    fn sample_filter_bpf__destroy(skel: *mut sample_filter_bpf);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_uint) -> c_int;
    fn bpf_program__attach_perf_event_opts(prog: *mut bpf_program, pfd: c_int, opts: *const c_void) -> *mut bpf_link;
    fn bpf_object__pin(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_object__unpin(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn sysfs__mountpoint() -> *const c_char;
    fn perf_cap__capable(cap: c_int) -> bool;
    fn perf_bpf_filter__scan_string(str_: *const c_char) -> YY_BUFFER_STATE;
    fn perf_bpf_filter_parse(expr_head: *mut list_head) -> c_int;
    fn perf_bpf_filter__flush_buffer(buffer: YY_BUFFER_STATE);
    fn perf_bpf_filter__delete_buffer(buffer: YY_BUFFER_STATE);
    fn perf_bpf_filter_lex_destroy() -> c_int;
}

type c_long = isize;

unsafe fn list_init_once(head: *mut list_head) {
    if (*head).next.is_null() {
        (*head).next = head;
        (*head).prev = head;
    }
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*(head as *mut list_head)).next == head as *mut list_head
}

unsafe fn __list_add(new: *mut list_head, prev: *mut list_head, next: *mut list_head) {
    (*next).prev = new;
    (*new).next = next;
    (*new).prev = prev;
    (*prev).next = new;
}

unsafe fn list_add(new: *mut list_head, head: *mut list_head) {
    __list_add(new, head, (*head).next);
}

unsafe fn __list_del(prev: *mut list_head, next: *mut list_head) {
    (*next).prev = prev;
    (*prev).next = next;
}

unsafe fn list_del(entry: *mut list_head) {
    __list_del((*entry).prev, (*entry).next);
    (*entry).next = ptr::null_mut();
    (*entry).prev = ptr::null_mut();
}

unsafe fn FD(e: *mut evsel, x: c_int, y: c_int) -> c_int {
    *(xyarray__entry((*e).core.fd, x, y) as *mut c_int)
}

unsafe fn get_pinned_fd(name: *const c_char) -> c_int {
    let mut path: *mut c_char = ptr::null_mut();
    let mut fd: c_int;

    if asprintf(
        &mut path,
        b"%s/fs/bpf/%s/%s\0".as_ptr() as *const c_char,
        sysfs__mountpoint(),
        PERF_BPF_FILTER_PIN_PATH.as_ptr() as *const c_char,
        name,
    ) < 0
    {
        return -1;
    }

    fd = bpf_obj_get(path);

    free(path as *mut c_void);
    fd
}

unsafe fn get_sample_info(type_: perf_bpf_filter_term) -> *const perf_sample_info {
    let mut i: size_t = 0;

    while i < sample_table.len() {
        if sample_table[i].type_ == type_ {
            return &sample_table[i];
        }
        i += 1;
    }
    ptr::null()
}

unsafe fn check_sample_flags(evsel: *mut evsel, expr: *mut perf_bpf_filter_expr) -> c_int {
    let mut info: *const perf_sample_info;

    if (*expr).term as c_int >= PBF_TERM_SAMPLE_START as c_int
        && (*expr).term as c_int <= PBF_TERM_SAMPLE_END as c_int
        && (((*evsel).core.attr.sample_type
            & (1_u64 << ((*expr).term as c_int - PBF_TERM_SAMPLE_START as c_int))) != 0)
    {
        return 0;
    }

    if (*expr).term == perf_bpf_filter_term::PBF_TERM_UID
        || (*expr).term == perf_bpf_filter_term::PBF_TERM_GID
    {
        /* Not dependent on the sample_type as computed from a BPF helper. */
        return 0;
    }

    if (*expr).op == perf_bpf_filter_op::PBF_OP_GROUP_BEGIN {
        let mut pos = (*expr).groups.next;
        while pos != &mut (*expr).groups {
            let group = pos as *mut perf_bpf_filter_expr;
            if check_sample_flags(evsel, group) < 0 {
                return -1;
            }
            pos = (*pos).next;
        }
        return 0;
    }

    info = get_sample_info((*expr).term);
    if info.is_null() {
        pr_err(
            b"Error: %s event does not have sample flags %d\n\0".as_ptr() as *const c_char,
            evsel__name(evsel),
            (*expr).term as c_int,
        );
        return -1;
    }

    pr_err(
        b"Error: %s event does not have %s\n\0".as_ptr() as *const c_char,
        evsel__name(evsel),
        (*info).name,
    );
    if !(*info).option.is_null() {
        pr_err(
            b" Hint: please add %s option to perf record\n\0".as_ptr() as *const c_char,
            (*info).option,
        );
    }
    -1
}

unsafe fn get_filter_entries(evsel: *mut evsel, entry: *mut perf_bpf_filter_entry) -> c_int {
    let mut i: c_int = 0;
    let mut pos = (*evsel).bpf_filters.next;

    while pos != &mut (*evsel).bpf_filters {
        let expr = pos as *mut perf_bpf_filter_expr;
        if check_sample_flags(evsel, expr) < 0 {
            return -EINVAL;
        }

        if i == MAX_FILTERS {
            return -E2BIG;
        }

        (*entry.add(i as usize)).op = (*expr).op;
        (*entry.add(i as usize)).part = (*expr).part;
        (*entry.add(i as usize)).term = (*expr).term;
        (*entry.add(i as usize)).value = (*expr).val;
        i += 1;

        if (*expr).op == perf_bpf_filter_op::PBF_OP_GROUP_BEGIN {
            let mut gpos = (*expr).groups.next;
            while gpos != &mut (*expr).groups {
                let group = gpos as *mut perf_bpf_filter_expr;
                if i == MAX_FILTERS {
                    return -E2BIG;
                }

                (*entry.add(i as usize)).op = (*group).op;
                (*entry.add(i as usize)).part = (*group).part;
                (*entry.add(i as usize)).term = (*group).term;
                (*entry.add(i as usize)).value = (*group).val;
                i += 1;
                gpos = (*gpos).next;
            }

            if i == MAX_FILTERS {
                return -E2BIG;
            }

            (*entry.add(i as usize)).op = perf_bpf_filter_op::PBF_OP_GROUP_END;
            i += 1;
        }
        pos = (*pos).next;
    }

    if i < MAX_FILTERS {
        /* to terminate the loop early */
        (*entry.add(i as usize)).op = perf_bpf_filter_op::PBF_OP_DONE;
        i += 1;
    }
    0
}

unsafe fn convert_to_tgid(tid: c_int) -> c_int {
    let mut path = [0 as c_char; 128];
    let mut buf: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char;
    let mut q: *mut c_char = ptr::null_mut();
    let mut tgid: c_int;
    let mut len: size_t = 0;

    scnprintf(path.as_mut_ptr(), path.len(), b"%d/status\0".as_ptr() as *const c_char, tid);
    if procfs__read_str(path.as_ptr(), &mut buf, &mut len) < 0 {
        return -1;
    }

    p = strstr(buf, b"Tgid:\0".as_ptr() as *const c_char);
    if p.is_null() {
        free(buf as *mut c_void);
        return -1;
    }

    tgid = strtol(p.add(6), &mut q, 0) as c_int;
    free(buf as *mut c_void);
    if *q != b'\n' as c_char {
        return -1;
    }

    tgid
}

/*
 * The event might be closed already so we cannot get the list of ids using FD
 * like in create_event_hash() below, let's iterate the event_hash map and
 * delete all entries that have the event id as a key.
 */
unsafe fn destroy_event_hash(event_id: u64) {
    let mut fd: c_int;
    let mut key: u64 = 0;
    let mut prev_key: *mut u64 = ptr::null_mut();
    let mut num: c_int = 0;
    let mut alloced: c_int = 32;
    let mut ids = calloc(alloced as size_t, size_of::<u64>()) as *mut u64;

    if ids.is_null() {
        return;
    }

    fd = get_pinned_fd(b"event_hash\0".as_ptr() as *const c_char);
    if fd < 0 {
        pr_debug(b"cannot get fd for 'event_hash' map\n\0".as_ptr() as *const c_char);
        free(ids as *mut c_void);
        return;
    }

    /* Iterate the whole map to collect keys for the event id. */
    while bpf_map_get_next_key(fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
        let mut id: u64 = 0;

        if bpf_map_lookup_elem(fd, &mut key as *mut _ as *const c_void, &mut id as *mut _ as *mut c_void) == 0
            && id == event_id
        {
            if num == alloced {
                let mut tmp: *mut c_void;

                alloced *= 2;
                tmp = realloc(ids as *mut c_void, alloced as size_t * size_of::<u64>());
                if tmp.is_null() {
                    break;
                }

                ids = tmp as *mut u64;
            }
            *ids.add(num as usize) = key;
            num += 1;
        }

        prev_key = &mut key;
    }

    let mut i: c_int = 0;
    while i < num {
        bpf_map_delete_elem(fd, ids.add(i as usize) as *const c_void);
        i += 1;
    }

    free(ids as *mut c_void);
    close(fd);
}

/*
 * Return a representative id if ok, or 0 for failures.
 *
 * The perf_event->id is good for this, but an evsel would have multiple
 * instances for CPUs and tasks.  So pick up the first id and setup a hash
 * from id of each instance to the representative id (the first one).
 */
unsafe fn create_event_hash(evsel: *mut evsel) -> u64 {
    let mut x: c_int;
    let mut y: c_int;
    let mut fd: c_int;
    let mut the_id: u64 = 0;
    let mut id: u64 = 0;

    fd = get_pinned_fd(b"event_hash\0".as_ptr() as *const c_char);
    if fd < 0 {
        pr_err(b"cannot get fd for 'event_hash' map\n\0".as_ptr() as *const c_char);
        return 0;
    }

    x = 0;
    while x < xyarray__max_x((*evsel).core.fd) {
        y = 0;
        while y < xyarray__max_y((*evsel).core.fd) {
            let ret = ioctl(FD(evsel, x, y), PERF_EVENT_IOC_ID, &mut id);

            if ret < 0 {
                pr_err(b"Failed to get the event id\n\0".as_ptr() as *const c_char);
                if the_id != 0 {
                    destroy_event_hash(the_id);
                }
                return 0;
            }

            if the_id == 0 {
                the_id = id;
            }

            bpf_map_update_elem(fd, &mut id as *mut _ as *const c_void, &mut the_id as *mut _ as *const c_void, BPF_ANY);
            y += 1;
        }
        x += 1;
    }

    close(fd);
    the_id
}

unsafe fn destroy_idx_hash(pfi: *mut pinned_filter_idx) {
    let mut fd: c_int;
    let mut nr: c_int;
    let threads: *mut perf_thread_map;

    fd = get_pinned_fd(b"filters\0".as_ptr() as *const c_char);
    bpf_map_delete_elem(fd, &mut (*pfi).hash_idx as *mut _ as *const c_void);
    close(fd);

    if (*pfi).event_id != 0 {
        destroy_event_hash((*pfi).event_id);
    }

    threads = perf_evsel__threads(&mut (*(*pfi).evsel).core);
    if threads.is_null() {
        return;
    }

    fd = get_pinned_fd(b"idx_hash\0".as_ptr() as *const c_char);
    nr = perf_thread_map__nr(threads);
    let mut i: c_int = 0;
    while i < nr {
        /* The target task might be dead already, just try the pid */
        let mut key = idx_hash_key {
            evt_id: (*pfi).event_id,
            tgid: perf_thread_map__pid(threads, i),
        };

        bpf_map_delete_elem(fd, &mut key as *mut _ as *const c_void);
        i += 1;
    }
    close(fd);
}

/* Maintain a hashmap from (tgid, event-id) to filter index */
unsafe fn create_idx_hash(evsel: *mut evsel, entry: *mut perf_bpf_filter_entry) -> c_int {
    let mut filter_idx: c_int;
    let mut fd: c_int;
    let mut nr: c_int;
    let mut last: c_int;
    let mut event_id: u64 = 0;
    let mut pfi: *mut pinned_filter_idx = ptr::null_mut();
    let mut threads: *mut perf_thread_map;

    fd = get_pinned_fd(b"filters\0".as_ptr() as *const c_char);
    if fd < 0 {
        pr_err(b"cannot get fd for 'filters' map\n\0".as_ptr() as *const c_char);
        return fd;
    }

    /* Find the first available entry in the filters map */
    filter_idx = 0;
    while filter_idx < MAX_FILTERS {
        if bpf_map_update_elem(fd, &mut filter_idx as *mut _ as *const c_void, entry as *const c_void, BPF_NOEXIST) == 0 {
            break;
        }
        filter_idx += 1;
    }
    close(fd);

    if filter_idx == MAX_FILTERS {
        pr_err(b"Too many users for the filter map\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    pfi = calloc(1, size_of::<pinned_filter_idx>()) as *mut pinned_filter_idx;
    if pfi.is_null() {
        pr_err(b"Cannot save pinned filter index\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    (*pfi).evsel = evsel;
    (*pfi).hash_idx = filter_idx;

    event_id = create_event_hash(evsel);
    if event_id == 0 {
        pr_err(b"Cannot update the event hash\n\0".as_ptr() as *const c_char);
        destroy_idx_hash(pfi);
        free(pfi as *mut c_void);
        return -1;
    }

    (*pfi).event_id = event_id;

    threads = perf_evsel__threads(&mut (*evsel).core);
    if threads.is_null() {
        pr_err(b"Cannot get the thread list of the event\n\0".as_ptr() as *const c_char);
        destroy_idx_hash(pfi);
        free(pfi as *mut c_void);
        return -1;
    }

    /* save the index to a hash map */
    fd = get_pinned_fd(b"idx_hash\0".as_ptr() as *const c_char);
    if fd < 0 {
        pr_err(b"cannot get fd for 'idx_hash' map\n\0".as_ptr() as *const c_char);
        destroy_idx_hash(pfi);
        free(pfi as *mut c_void);
        return -1;
    }

    last = -1;
    nr = perf_thread_map__nr(threads);
    let mut i: c_int = 0;
    while i < nr {
        let pid = perf_thread_map__pid(threads, i);
        let mut tgid: c_int;
        let mut key = idx_hash_key {
            evt_id: event_id,
            tgid: 0,
        };

        /* it actually needs tgid, let's get tgid from /proc. */
        tgid = convert_to_tgid(pid);
        if tgid < 0 {
            /* the thread may be dead, ignore. */
            i += 1;
            continue;
        }

        if tgid == last {
            i += 1;
            continue;
        }
        last = tgid;
        key.tgid = tgid;

        if bpf_map_update_elem(fd, &mut key as *mut _ as *const c_void, &mut filter_idx as *mut _ as *const c_void, BPF_ANY) < 0 {
            pr_err(b"Failed to update the idx_hash\n\0".as_ptr() as *const c_char);
            close(fd);
            destroy_idx_hash(pfi);
            free(pfi as *mut c_void);
            return -1;
        }
        pr_debug(
            b"bpf-filter: idx_hash (task=%d,%s) -> %d\n\0".as_ptr() as *const c_char,
            tgid,
            evsel__name(evsel),
            filter_idx,
        );
        i += 1;
    }

    list_init_once(&mut pinned_filters);
    list_add(&mut (*pfi).list, &mut pinned_filters);
    close(fd);
    filter_idx
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__prepare(evsel: *mut evsel, target: *mut target) -> c_int {
    let mut i: c_int;
    let mut x: c_int;
    let mut y: c_int;
    let mut fd: c_int;
    let mut ret: c_int;
    let mut skel: *mut sample_filter_bpf = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut link: *mut bpf_link;
    let mut entry: *mut perf_bpf_filter_entry;
    let needs_idx_hash: bool = !target__has_cpu(target);
    /* LIBBPF_CURRENT_VERSION_GEQ(1, 7): bpf_perf_event_opts uses dont_enable = true when available. */
    let pe_opts: *const c_void = ptr::null();

    entry = calloc(MAX_FILTERS as size_t, size_of::<perf_bpf_filter_entry>()) as *mut perf_bpf_filter_entry;
    if entry.is_null() {
        return -1;
    }

    ret = get_filter_entries(evsel, entry);
    if ret < 0 {
        pr_err(b"Failed to process filter entries\n\0".as_ptr() as *const c_char);
        free(entry as *mut c_void);
        if !list_empty(&mut pinned_filters) {
            let mut pos = pinned_filters.next;
            while pos != &mut pinned_filters {
                let pfi = pos as *mut pinned_filter_idx;
                let tmp = (*pos).next;
                destroy_idx_hash(pfi);
                list_del(&mut (*pfi).list);
                free(pfi as *mut c_void);
                pos = tmp;
            }
        }
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    if needs_idx_hash && geteuid() != 0 {
        let mut zero: c_int = 0;

        /* The filters map is shared among other processes */
        ret = create_idx_hash(evsel, entry);
        if ret < 0 {
            free(entry as *mut c_void);
            sample_filter_bpf__destroy(skel);
            return ret;
        }

        fd = get_pinned_fd(b"dropped\0".as_ptr() as *const c_char);
        if fd < 0 {
            ret = fd;
            free(entry as *mut c_void);
            sample_filter_bpf__destroy(skel);
            return ret;
        }

        /* Reset the lost count */
        bpf_map_update_elem(fd, &mut ret as *mut _ as *const c_void, &mut zero as *mut _ as *const c_void, BPF_ANY);
        close(fd);

        fd = get_pinned_fd(b"perf_sample_filter\0".as_ptr() as *const c_char);
        if fd < 0 {
            ret = fd;
            free(entry as *mut c_void);
            sample_filter_bpf__destroy(skel);
            return ret;
        }

        x = 0;
        while x < xyarray__max_x((*evsel).core.fd) {
            y = 0;
            while y < xyarray__max_y((*evsel).core.fd) {
                ret = ioctl(FD(evsel, x, y), PERF_EVENT_IOC_SET_BPF, fd);
                if ret < 0 {
                    pr_err(b"Failed to attach perf sample-filter\n\0".as_ptr() as *const c_char);
                    close(fd);
                    free(entry as *mut c_void);
                    sample_filter_bpf__destroy(skel);
                    return ret;
                }
                y += 1;
            }
            x += 1;
        }

        close(fd);
        free(entry as *mut c_void);
        return 0;
    }

    skel = sample_filter_bpf__open_and_load();
    if skel.is_null() {
        ret = -errno;
        pr_err(b"Failed to load perf sample-filter BPF skeleton\n\0".as_ptr() as *const c_char);
        free(entry as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    i = 0;
    fd = bpf_map__fd((*skel).maps.filters);

    /* The filters map has only one entry in this case */
    if bpf_map_update_elem(fd, &mut i as *mut _ as *const c_void, entry as *const c_void, BPF_ANY) < 0 {
        ret = -errno;
        pr_err(b"Failed to update the filter map\n\0".as_ptr() as *const c_char);
        free(entry as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    prog = (*skel).progs.perf_sample_filter;
    x = 0;
    while x < xyarray__max_x((*evsel).core.fd) {
        y = 0;
        while y < xyarray__max_y((*evsel).core.fd) {
            link = bpf_program__attach_perf_event_opts(prog, FD(evsel, x, y), pe_opts);
            if (link as isize) < 0 && (link as isize) > -4096 {
                pr_err(b"Failed to attach perf sample-filter program\n\0".as_ptr() as *const c_char);
                ret = link as isize as c_int;
                free(entry as *mut c_void);
                sample_filter_bpf__destroy(skel);
                return ret;
            }
            y += 1;
        }
        x += 1;
    }
    free(entry as *mut c_void);
    (*evsel).bpf_skel = skel;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__destroy(evsel: *mut evsel) -> c_int {
    let mut pos = (*evsel).bpf_filters.next;
    while pos != &mut (*evsel).bpf_filters {
        let expr = pos as *mut perf_bpf_filter_expr;
        let tmp = (*pos).next;
        list_del(&mut (*expr).list);
        free(expr as *mut c_void);
        pos = tmp;
    }
    sample_filter_bpf__destroy((*evsel).bpf_skel);

    list_init_once(&mut pinned_filters);
    let mut ppos = pinned_filters.next;
    while ppos != &mut pinned_filters {
        let pfi = ppos as *mut pinned_filter_idx;
        let pos_next = (*ppos).next;
        destroy_idx_hash(pfi);
        list_del(&mut (*pfi).list);
        free(pfi as *mut c_void);
        ppos = pos_next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__lost_count(evsel: *mut evsel) -> u64 {
    let mut count: c_int = 0;

    if list_empty(&mut (*evsel).bpf_filters) {
        return 0;
    }

    list_init_once(&mut pinned_filters);
    if !list_empty(&mut pinned_filters) {
        let fd = get_pinned_fd(b"dropped\0".as_ptr() as *const c_char);

        if fd < 0 {
            return 0;
        }

        let mut pos = pinned_filters.next;
        while pos != &mut pinned_filters {
            let pfi = pos as *mut pinned_filter_idx;
            if (*pfi).evsel != evsel {
                pos = (*pos).next;
                continue;
            }

            bpf_map_lookup_elem(fd, &mut (*pfi).hash_idx as *mut _ as *const c_void, &mut count as *mut _ as *mut c_void);
            break;
        }
        close(fd);
    } else if !(*evsel).bpf_skel.is_null() {
        let skel = (*evsel).bpf_skel;
        let fd = bpf_map__fd((*skel).maps.dropped);
        let mut idx: c_int = 0;

        bpf_map_lookup_elem(fd, &mut idx as *mut _ as *const c_void, &mut count as *mut _ as *mut c_void);
    }

    count as u64
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter_expr__new(
    term: perf_bpf_filter_term,
    part: c_int,
    op: perf_bpf_filter_op,
    val: c_ulong,
) -> *mut perf_bpf_filter_expr {
    let mut expr: *mut perf_bpf_filter_expr;

    expr = malloc(size_of::<perf_bpf_filter_expr>()) as *mut perf_bpf_filter_expr;
    if !expr.is_null() {
        (*expr).term = term;
        (*expr).part = part;
        (*expr).op = op;
        (*expr).val = val;
        INIT_LIST_HEAD(&mut (*expr).groups);
    }
    expr
}

unsafe fn check_bpf_filter_capable() -> bool {
    let mut fd: c_int;

    if perf_cap__capable(CAP_BPF) {
        return true;
    }

    /* Check if root already pinned the filter programs and maps */
    fd = get_pinned_fd(b"filters\0".as_ptr() as *const c_char);
    if fd >= 0 {
        close(fd);
        return true;
    }

    pr_err(
        b"Error: BPF filter only works for users with the CAP_BPF capability!\n\tPlease run 'perf record --setup-filter pin' as root first.\n\0"
            .as_ptr() as *const c_char,
    );

    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__parse(expr_head: *mut list_head, str_: *const c_char) -> c_int {
    let buffer: YY_BUFFER_STATE;
    let ret: c_int;

    if !check_bpf_filter_capable() {
        return -EPERM;
    }

    buffer = perf_bpf_filter__scan_string(str_);

    ret = perf_bpf_filter_parse(expr_head);

    perf_bpf_filter__flush_buffer(buffer);
    perf_bpf_filter__delete_buffer(buffer);
    perf_bpf_filter_lex_destroy();

    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__pin() -> c_int {
    let mut skel: *mut sample_filter_bpf;
    let mut path: *mut c_char = ptr::null_mut();
    let mut dir_fd: c_int = 0;
    let mut ret: c_int = -1;

    skel = sample_filter_bpf__open();
    if skel.is_null() {
        ret = -errno;
        pr_err(b"Failed to open perf sample-filter BPF skeleton\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    /* pinned program will use pid-hash */
    bpf_map__set_max_entries((*skel).maps.filters, MAX_FILTERS as c_uint);
    bpf_map__set_max_entries((*skel).maps.event_hash, MAX_EVT_HASH as c_uint);
    bpf_map__set_max_entries((*skel).maps.idx_hash, MAX_IDX_HASH as c_uint);
    bpf_map__set_max_entries((*skel).maps.dropped, MAX_FILTERS as c_uint);
    (*(*skel).rodata).use_idx_hash = 1;

    if sample_filter_bpf__load(skel) < 0 {
        ret = -errno;
        pr_err(b"Failed to load perf sample-filter BPF skeleton\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    if asprintf(
        &mut path,
        b"%s/fs/bpf/%s\0".as_ptr() as *const c_char,
        sysfs__mountpoint(),
        PERF_BPF_FILTER_PIN_PATH.as_ptr() as *const c_char,
    ) < 0
    {
        ret = -errno;
        pr_err(b"Failed to allocate pathname in the BPF-fs\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    ret = bpf_object__pin((*skel).obj, path);
    if ret < 0 {
        pr_err(b"Failed to pin BPF filter objects\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    /* setup access permissions for the pinned objects */
    dir_fd = open(path, O_PATH);
    if dir_fd < 0 {
        bpf_object__unpin((*skel).obj, path);
        ret = dir_fd;
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    /* BPF-fs root has the sticky bit */
    if fchmodat(dir_fd, b"..\0".as_ptr() as *const c_char, 0o1755, 0) < 0 {
        pr_debug(b"chmod for BPF-fs failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
        close(dir_fd);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    /* perf_filter directory */
    if fchmodat(dir_fd, b".\0".as_ptr() as *const c_char, 0o755, 0) < 0 {
        pr_debug(b"chmod for perf_filter directory failed?\n\0".as_ptr() as *const c_char);
        ret = -errno;
        close(dir_fd);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    /* programs need write permission for some reason */
    if fchmodat(dir_fd, b"perf_sample_filter\0".as_ptr() as *const c_char, 0o777, 0) < 0 {
        pr_debug(b"chmod for perf_sample_filter failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
    }
    /* maps */
    if fchmodat(dir_fd, b"filters\0".as_ptr() as *const c_char, 0o666, 0) < 0 {
        pr_debug(b"chmod for filters failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
    }
    if fchmodat(dir_fd, b"event_hash\0".as_ptr() as *const c_char, 0o666, 0) < 0 {
        pr_debug(b"chmod for event_hash failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
    }
    if fchmodat(dir_fd, b"idx_hash\0".as_ptr() as *const c_char, 0o666, 0) < 0 {
        pr_debug(b"chmod for idx_hash failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
    }
    if fchmodat(dir_fd, b"dropped\0".as_ptr() as *const c_char, 0o666, 0) < 0 {
        pr_debug(b"chmod for dropped failed\n\0".as_ptr() as *const c_char);
        ret = -errno;
    }

    close(dir_fd);
    free(path as *mut c_void);
    sample_filter_bpf__destroy(skel);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_bpf_filter__unpin() -> c_int {
    let mut skel: *mut sample_filter_bpf;
    let mut path: *mut c_char = ptr::null_mut();
    let mut ret: c_int = -1;

    skel = sample_filter_bpf__open_and_load();
    if skel.is_null() {
        ret = -errno;
        pr_err(b"Failed to open perf sample-filter BPF skeleton\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    if asprintf(
        &mut path,
        b"%s/fs/bpf/%s\0".as_ptr() as *const c_char,
        sysfs__mountpoint(),
        PERF_BPF_FILTER_PIN_PATH.as_ptr() as *const c_char,
    ) < 0
    {
        ret = -errno;
        pr_err(b"Failed to allocate pathname in the BPF-fs\n\0".as_ptr() as *const c_char);
        free(path as *mut c_void);
        sample_filter_bpf__destroy(skel);
        return ret;
    }

    ret = bpf_object__unpin((*skel).obj, path);

    free(path as *mut c_void);
    sample_filter_bpf__destroy(skel);
    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
