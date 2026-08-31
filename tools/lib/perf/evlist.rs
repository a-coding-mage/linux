// SPDX-License-Identifier: GPL-2.0
// Translated from C source: lib/perf/evlist.c
// C include dependencies are intentionally left as external Rust declarations.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u64 = u64;

const PERF_EVLIST__HLIST_BITS: i32 = 8;
const PERF_EVLIST__HLIST_SIZE: usize = 1usize << PERF_EVLIST__HLIST_BITS;
const PERF_EVENT_IOC_ID: u64 = 0;
const PERF_EVENT_IOC_SET_OUTPUT: u64 = 0;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const POLLIN: i16 = 0x0001;
const POLLERR: i16 = 0x0008;
const POLLHUP: i16 = 0x0010;
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const F_SETFL: i32 = 4;
const O_NONBLOCK: i32 = 0o4000;
const ENOMEM: i32 = 12;
const EINVAL: i32 = 22;
const ENOTTY: i32 = 25;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: i32,
}

#[repr(C)]
pub struct perf_event_attr {
    pub read_format: u64,
    pub write_backward: bool_,
}

#[repr(C)]
pub struct xyarray {
    pub max_x: i32,
    pub max_y: i32,
}

#[repr(C)]
pub struct perf_sample_id {
    pub id: u64,
    pub evsel: *mut perf_evsel,
    pub node: hlist_node,
    pub idx: i32,
    pub cpu: perf_cpu,
    pub tid: i32,
}

#[repr(C)]
pub struct fdarray_priv {
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct fdarray {
    pub entries: *mut c_void,
    pub priv_: *mut fdarray_priv,
}

#[repr(C)]
pub struct perf_mmap {
    pub next: *mut perf_mmap,
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct perf_mmap_param {
    pub prot: i32,
    pub mask: usize,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_evsel {
    pub node: list_head,
    pub idx: i32,
    pub cpus: *mut perf_cpu_map,
    pub pmu_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub fd: *mut xyarray,
    pub sample_id: *mut xyarray,
    pub id: *mut u64,
    pub ids: i32,
    pub attr: perf_event_attr,
    pub requires_cpu: bool_,
    pub system_wide: bool_,
    pub reads_only_on_cpu_idx0: bool_,
    pub leader: *mut perf_evsel,
    pub nr_members: i32,
}

#[repr(C)]
pub struct perf_evlist {
    pub entries: list_head,
    pub nr_entries: i32,
    pub pollfd: fdarray,
    pub heads: [hlist_head; PERF_EVLIST__HLIST_SIZE],
    pub user_requested_cpus: *mut perf_cpu_map,
    pub all_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
    pub has_user_cpus: bool_,
    pub needs_map_propagation: bool_,
    pub mmap: *mut perf_mmap,
    pub mmap_ovw: *mut perf_mmap,
    pub mmap_first: *mut perf_mmap,
    pub mmap_ovw_first: *mut perf_mmap,
    pub nr_mmaps: i32,
    pub mmap_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fdarray_flags {
    fdarray_flag__default = 0,
    fdarray_flag__nonfilterable = 1,
}

#[repr(C)]
pub struct perf_evlist_mmap_ops {
    pub get: Option<unsafe extern "C" fn(*mut perf_evlist, bool_, i32) -> *mut perf_mmap>,
    pub mmap: Option<unsafe extern "C" fn(*mut perf_mmap, *mut perf_mmap_param, i32, perf_cpu) -> i32>,
    pub idx: Option<unsafe extern "C" fn(*mut perf_evlist, *mut perf_evsel, *mut perf_mmap_param, i32)>,
}

unsafe extern "C" {
    static mut page_size: usize;
    static mut errno: i32;

    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zfree(ptr: *mut c_void);
    fn ioctl(fd: i32, request: u64, ...) -> i32;
    fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
    fn fcntl(fd: i32, cmd: i32, ...) -> i32;

    fn fdarray__init(fda: *mut fdarray, nr: i32);
    fn fdarray__exit(fda: *mut fdarray);
    fn fdarray__available_entries(fda: *mut fdarray) -> i32;
    fn fdarray__grow(fda: *mut fdarray, nr: i32) -> i32;
    fn fdarray__add(fda: *mut fdarray, fd: i32, revents: i16, flags: fdarray_flags) -> i32;
    fn fdarray__filter(
        fda: *mut fdarray,
        revents_and_mask: i16,
        cb: unsafe extern "C" fn(*mut fdarray, i32, *mut c_void),
        arg: *mut c_void,
    ) -> i32;
    fn fdarray__poll(fda: *mut fdarray, timeout: i32) -> i32;

    fn perf_cpu_map__is_empty(cpus: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__get(cpus: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_cpu_map__has_any_cpu(cpus: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__equal(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__is_subset(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> bool_;
    fn perf_cpu_map__intersect(a: *mut perf_cpu_map, b: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> i32;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: i32) -> perf_cpu;
    fn perf_cpu_map__new_int(cpu: i32) -> *mut perf_cpu_map;
    fn perf_cpu_map__merge(dst: *mut *mut perf_cpu_map, src: *mut perf_cpu_map);
    fn perf_cpu_map__has_any_cpu_or_is_empty(cpus: *const perf_cpu_map) -> bool_;
    fn perf_cpu_map__idx(cpus: *mut perf_cpu_map, cpu: perf_cpu) -> i32;

    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn perf_thread_map__new_dummy() -> *mut perf_thread_map;
    fn perf_thread_map__get(threads: *mut perf_thread_map) -> *mut perf_thread_map;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> i32;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: i32) -> i32;

    fn perf_evsel__open(evsel: *mut perf_evsel, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map) -> i32;
    fn perf_evsel__close(evsel: *mut perf_evsel);
    fn perf_evsel__enable(evsel: *mut perf_evsel);
    fn perf_evsel__disable(evsel: *mut perf_evsel);
    fn perf_evsel__delete(evsel: *mut perf_evsel);
    fn perf_evsel__alloc_id(evsel: *mut perf_evsel, max_x: i32, max_y: i32) -> i32;

    fn xyarray__entry(xy: *mut xyarray, x: i32, y: i32) -> *mut c_void;

    fn perf_mmap__init(map: *mut perf_mmap, prev: *mut perf_mmap, overwrite: bool_, auxtrace_mmap: *mut c_void);
    fn perf_mmap__mmap(map: *mut perf_mmap, mp: *mut perf_mmap_param, output: i32, cpu: perf_cpu) -> i32;
    fn perf_mmap__get(map: *mut perf_mmap);
    fn perf_mmap__put(map: *mut perf_mmap);
    fn perf_mmap__munmap(map: *mut perf_mmap);

    fn refcount_set(r: *mut refcount_t, n: i32);
    fn hash_64(val: u64, bits: i32) -> i32;
    fn pr_debug(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn INIT_HLIST_HEAD(h: *mut hlist_head) {
    (*h).first = ptr::null_mut();
}

unsafe fn list_add_tail(new_: *mut list_head, head: *mut list_head) {
    (*new_).prev = (*head).prev;
    (*new_).next = head;
    (*(*head).prev).next = new_;
    (*head).prev = new_;
}

unsafe fn list_del_init(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    INIT_LIST_HEAD(entry);
}

unsafe fn hlist_add_head(n: *mut hlist_node, h: *mut hlist_head) {
    let first = (*h).first;
    (*n).next = first;
    if !first.is_null() {
        (*first).pprev = &mut (*n).next;
    }
    (*h).first = n;
    (*n).pprev = &mut (*h).first;
}

unsafe fn list_entry_perf_evsel(ptr: *mut list_head) -> *mut perf_evsel {
    (ptr as *mut u8).sub(core::mem::offset_of!(perf_evsel, node)) as *mut perf_evsel
}

unsafe fn SID(e: *mut perf_evsel, x: i32, y: i32) -> *mut perf_sample_id {
    xyarray__entry((*e).sample_id, x, y) as *mut perf_sample_id
}

unsafe fn FD(e: *mut perf_evsel, x: i32, y: i32) -> i32 {
    *(xyarray__entry((*e).fd, x, y) as *mut i32)
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__init(evlist: *mut perf_evlist) {
    INIT_LIST_HEAD(&mut (*evlist).entries);
    (*evlist).nr_entries = 0;
    fdarray__init(&mut (*evlist).pollfd, 64);
    perf_evlist__reset_id_hash(evlist);
}

unsafe fn __perf_evlist__propagate_maps(evlist: *mut perf_evlist, evsel: *mut perf_evsel) {
    if perf_cpu_map__is_empty((*evsel).cpus) {
        if perf_cpu_map__is_empty((*evsel).pmu_cpus) {
            /*
             * Assume the unset PMU cpus were for a system-wide
             * event, like a software or tracepoint.
             */
            (*evsel).pmu_cpus = perf_cpu_map__new_online_cpus();
        }
        if (*evlist).has_user_cpus && !(*evsel).system_wide {
            /*
             * Use the user CPUs unless the evsel is set to be
             * system wide, such as the dummy event.
             */
            (*evsel).cpus = perf_cpu_map__get((*evlist).user_requested_cpus);
        } else {
            /*
             * System wide and other modes, assume the cpu map
             * should be set to all PMU CPUs.
             */
            (*evsel).cpus = perf_cpu_map__get((*evsel).pmu_cpus);
        }
    }
    /*
     * Avoid "any CPU"(-1) for uncore and PMUs that require a CPU, even if
     * requested.
     */
    if (*evsel).requires_cpu && perf_cpu_map__has_any_cpu((*evsel).cpus) {
        perf_cpu_map__put((*evsel).cpus);
        (*evsel).cpus = perf_cpu_map__get((*evsel).pmu_cpus);
    }

    /*
     * Globally requested CPUs replace user requested unless the evsel is
     * set to be system wide.
     */
    if (*evlist).has_user_cpus && !(*evsel).system_wide {
        assert!(!perf_cpu_map__has_any_cpu((*evlist).user_requested_cpus));
        if !perf_cpu_map__equal((*evsel).cpus, (*evlist).user_requested_cpus) {
            perf_cpu_map__put((*evsel).cpus);
            (*evsel).cpus = perf_cpu_map__get((*evlist).user_requested_cpus);
        }
    }

    /* Ensure cpus only references valid PMU CPUs. */
    if !perf_cpu_map__has_any_cpu((*evsel).cpus)
        && !perf_cpu_map__is_subset((*evsel).pmu_cpus, (*evsel).cpus)
    {
        let tmp = perf_cpu_map__intersect((*evsel).pmu_cpus, (*evsel).cpus);

        perf_cpu_map__put((*evsel).cpus);
        (*evsel).cpus = tmp;
    }

    /*
     * Was event requested on all the PMU's CPUs but the user requested is
     * any CPU (-1)? If so switch to using any CPU (-1) to reduce the number
     * of events.
     */
    if !(*evsel).system_wide
        && !(*evsel).requires_cpu
        && perf_cpu_map__equal((*evsel).cpus, (*evsel).pmu_cpus)
        && perf_cpu_map__has_any_cpu((*evlist).user_requested_cpus)
    {
        perf_cpu_map__put((*evsel).cpus);
        (*evsel).cpus = perf_cpu_map__get((*evlist).user_requested_cpus);
    }

    /*
     * Tool events may only read on the first CPU index to avoid double
     * counting things like duration_time. Make the evsel->cpus contain just
     * that single entry otherwise we may spend time changing affinity to
     * CPUs that just have tool events, etc.
     */
    if (*evsel).reads_only_on_cpu_idx0 && perf_cpu_map__nr((*evsel).cpus) > 0 {
        let srcs: [*mut perf_cpu_map; 3] = [
            (*evlist).all_cpus,
            (*evlist).user_requested_cpus,
            (*evsel).pmu_cpus,
        ];
        let mut i: usize = 0;
        while i < srcs.len() {
            if srcs[i].is_null() {
                i += 1;
                continue;
            }

            perf_cpu_map__put((*evsel).cpus);
            (*evsel).cpus = perf_cpu_map__new_int(perf_cpu_map__cpu(srcs[i], 0).cpu);
            break;
        }
    }

    /* Sanity check assert before the evsel is potentially removed. */
    assert!(!(*evsel).requires_cpu || !perf_cpu_map__has_any_cpu((*evsel).cpus));

    /*
     * Empty cpu lists would eventually get opened as "any" so remove
     * genuinely empty ones before they're opened in the wrong place.
     */
    if perf_cpu_map__is_empty((*evsel).cpus) {
        let next = perf_evlist__next(evlist, evsel);

        perf_evlist__remove(evlist, evsel);
        /* Keep idx contiguous */
        if !next.is_null() {
            let mut pos = next;
            while &mut (*pos).node as *mut list_head != &mut (*evlist).entries {
                (*pos).idx -= 1;
                pos = list_entry_perf_evsel((*pos).node.next);
            }
        }

        return;
    }

    if (*evsel).system_wide {
        perf_thread_map__put((*evsel).threads);
        (*evsel).threads = perf_thread_map__new_dummy();
    } else {
        perf_thread_map__put((*evsel).threads);
        (*evsel).threads = perf_thread_map__get((*evlist).threads);
    }

    perf_cpu_map__merge(&mut (*evlist).all_cpus, (*evsel).cpus);
}

unsafe fn perf_evlist__propagate_maps(evlist: *mut perf_evlist) {
    (*evlist).needs_map_propagation = true;

    /* Clear the all_cpus set which will be merged into during propagation. */
    perf_cpu_map__put((*evlist).all_cpus);
    (*evlist).all_cpus = ptr::null_mut();

    /* 2 rounds so that reads_only_on_cpu_idx0 benefit from knowing the other CPU maps. */
    let mut round = 0;
    while round < 2 {
        let mut node = (*evlist).entries.next;
        while node != &mut (*evlist).entries {
            let evsel = list_entry_perf_evsel(node);
            let n = (*node).next;
            if ((!(*evsel).reads_only_on_cpu_idx0 && round == 0)
                || ((*evsel).reads_only_on_cpu_idx0 && round == 1))
            {
                __perf_evlist__propagate_maps(evlist, evsel);
            }
            node = n;
        }
        round += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__add(evlist: *mut perf_evlist, evsel: *mut perf_evsel) {
    (*evsel).idx = (*evlist).nr_entries;
    list_add_tail(&mut (*evsel).node, &mut (*evlist).entries);
    (*evlist).nr_entries += 1;

    if (*evlist).needs_map_propagation {
        __perf_evlist__propagate_maps(evlist, evsel);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__remove(evlist: *mut perf_evlist, evsel: *mut perf_evsel) {
    list_del_init(&mut (*evsel).node);
    (*evlist).nr_entries -= 1;
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__new() -> *mut perf_evlist {
    let evlist = zalloc(size_of::<perf_evlist>()) as *mut perf_evlist;

    if !evlist.is_null() {
        perf_evlist__init(evlist);
    }

    evlist
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__next(
    evlist: *mut perf_evlist,
    prev: *mut perf_evsel,
) -> *mut perf_evsel {
    let next: *mut perf_evsel;

    if prev.is_null() {
        next = list_entry_perf_evsel((*evlist).entries.next);
    } else {
        next = list_entry_perf_evsel((*prev).node.next);
    }

    /* Empty list is noticed here so don't need checking on entry. */
    if &mut (*next).node as *mut list_head == &mut (*evlist).entries {
        return ptr::null_mut();
    }

    next
}

unsafe fn perf_evlist__purge(evlist: *mut perf_evlist) {
    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let pos = list_entry_perf_evsel(node);
        let n = (*node).next;
        list_del_init(&mut (*pos).node);
        perf_evsel__delete(pos);
        node = n;
    }

    (*evlist).nr_entries = 0;
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__exit(evlist: *mut perf_evlist) {
    perf_cpu_map__put((*evlist).user_requested_cpus);
    perf_cpu_map__put((*evlist).all_cpus);
    perf_thread_map__put((*evlist).threads);
    (*evlist).user_requested_cpus = ptr::null_mut();
    (*evlist).all_cpus = ptr::null_mut();
    (*evlist).threads = ptr::null_mut();
    fdarray__exit(&mut (*evlist).pollfd);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__delete(evlist: *mut perf_evlist) {
    if evlist.is_null() {
        return;
    }

    perf_evlist__munmap(evlist);
    perf_evlist__close(evlist);
    perf_evlist__purge(evlist);
    perf_evlist__exit(evlist);
    free(evlist as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__set_maps(
    evlist: *mut perf_evlist,
    cpus: *mut perf_cpu_map,
    threads: *mut perf_thread_map,
) {
    /*
     * Allow for the possibility that one or another of the maps isn't being
     * changed i.e. don't put it.  Note we are assuming the maps that are
     * being applied are brand new and evlist is taking ownership of the
     * original reference count of 1.  If that is not the case it is up to
     * the caller to increase the reference count.
     */
    if cpus != (*evlist).user_requested_cpus {
        perf_cpu_map__put((*evlist).user_requested_cpus);
        (*evlist).user_requested_cpus = perf_cpu_map__get(cpus);
    }

    if threads != (*evlist).threads {
        perf_thread_map__put((*evlist).threads);
        (*evlist).threads = perf_thread_map__get(threads);
    }

    perf_evlist__propagate_maps(evlist);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__open(evlist: *mut perf_evlist) -> i32 {
    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        let err = perf_evsel__open(evsel, (*evsel).cpus, (*evsel).threads);
        if err < 0 {
            perf_evlist__close(evlist);
            return err;
        }
        node = (*node).next;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__close(evlist: *mut perf_evlist) {
    let mut node = (*evlist).entries.prev;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        node = (*node).prev;
        perf_evsel__close(evsel);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__enable(evlist: *mut perf_evlist) {
    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        perf_evsel__enable(evsel);
        node = (*node).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__disable(evlist: *mut perf_evlist) {
    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        perf_evsel__disable(evsel);
        node = (*node).next;
    }
}

unsafe fn perf_evlist__first(evlist: *mut perf_evlist) -> *mut perf_evsel {
    perf_evlist__next(evlist, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__read_format(evlist: *mut perf_evlist) -> u64 {
    let first = perf_evlist__first(evlist);

    (*first).attr.read_format
}

unsafe fn perf_evlist__id_hash(
    evlist: *mut perf_evlist,
    evsel: *mut perf_evsel,
    cpu_map_idx: i32,
    thread: i32,
    id: u64,
) {
    let sid = SID(evsel, cpu_map_idx, thread);

    (*sid).id = id;
    (*sid).evsel = evsel;
    let hash = hash_64((*sid).id, PERF_EVLIST__HLIST_BITS);
    hlist_add_head(&mut (*sid).node, &mut (*evlist).heads[hash as usize]);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__reset_id_hash(evlist: *mut perf_evlist) {
    let mut i = 0;

    while i < PERF_EVLIST__HLIST_SIZE {
        INIT_HLIST_HEAD(&mut (*evlist).heads[i]);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__id_add(
    evlist: *mut perf_evlist,
    evsel: *mut perf_evsel,
    cpu_map_idx: i32,
    thread: i32,
    id: u64,
) {
    if SID(evsel, cpu_map_idx, thread).is_null() {
        return;
    }

    perf_evlist__id_hash(evlist, evsel, cpu_map_idx, thread, id);
    *(*evsel).id.add((*evsel).ids as usize) = id;
    (*evsel).ids += 1;
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__id_add_fd(
    evlist: *mut perf_evlist,
    evsel: *mut perf_evsel,
    cpu_map_idx: i32,
    thread: i32,
    fd: i32,
) -> i32 {
    let mut read_data: [u64; 4] = [0; 4];
    let mut id_idx = 1; /* The first entry is the counter value */
    let mut id: u64 = 0;

    if SID(evsel, cpu_map_idx, thread).is_null() {
        return -1;
    }

    let ret = ioctl(fd, PERF_EVENT_IOC_ID, &mut id as *mut u64);
    if ret == 0 {
        perf_evlist__id_add(evlist, evsel, cpu_map_idx, thread, id);
        return 0;
    }

    if errno != ENOTTY {
        return -1;
    }

    /* Legacy way to get event id.. All hail to old kernels! */

    /*
     * This way does not work with group format read, so bail
     * out in that case.
     */
    if perf_evlist__read_format(evlist) & PERF_FORMAT_GROUP != 0 {
        return -1;
    }

    if ((*evsel).attr.read_format & PERF_FORMAT_ID) == 0
        || read(
            fd,
            read_data.as_mut_ptr() as *mut c_void,
            size_of::<[u64; 4]>(),
        ) == -1
    {
        return -1;
    }

    if (*evsel).attr.read_format & PERF_FORMAT_TOTAL_TIME_ENABLED != 0 {
        id_idx += 1;
    }
    if (*evsel).attr.read_format & PERF_FORMAT_TOTAL_TIME_RUNNING != 0 {
        id_idx += 1;
    }

    id = read_data[id_idx as usize];

    perf_evlist__id_add(evlist, evsel, cpu_map_idx, thread, id);
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__alloc_pollfd(evlist: *mut perf_evlist) -> i32 {
    let nr_cpus = perf_cpu_map__nr((*evlist).all_cpus);
    let nr_threads = perf_thread_map__nr((*evlist).threads);
    let mut nfds = 0;

    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        if (*evsel).system_wide {
            nfds += nr_cpus;
        } else {
            nfds += nr_cpus * nr_threads;
        }
        node = (*node).next;
    }

    if fdarray__available_entries(&mut (*evlist).pollfd) < nfds
        && fdarray__grow(&mut (*evlist).pollfd, nfds) < 0
    {
        return -ENOMEM;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__add_pollfd(
    evlist: *mut perf_evlist,
    fd: i32,
    ptr_: *mut c_void,
    revent: i16,
    flags: fdarray_flags,
) -> i32 {
    let pos = fdarray__add(
        &mut (*evlist).pollfd,
        fd,
        revent | POLLERR | POLLHUP,
        flags,
    );

    if pos >= 0 {
        (*(*evlist).pollfd.priv_.add(pos as usize)).ptr = ptr_;
        fcntl(fd, F_SETFL, O_NONBLOCK);
    }

    pos
}

unsafe extern "C" fn perf_evlist__munmap_filtered(
    fda: *mut fdarray,
    fd: i32,
    _arg: *mut c_void,
) {
    let map = (*(*fda).priv_.add(fd as usize)).ptr as *mut perf_mmap;

    if !map.is_null() {
        perf_mmap__put(map);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__filter_pollfd(
    evlist: *mut perf_evlist,
    revents_and_mask: i16,
) -> i32 {
    fdarray__filter(
        &mut (*evlist).pollfd,
        revents_and_mask,
        perf_evlist__munmap_filtered,
        ptr::null_mut(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__poll(evlist: *mut perf_evlist, timeout: i32) -> i32 {
    fdarray__poll(&mut (*evlist).pollfd, timeout)
}

unsafe fn perf_evlist__alloc_mmap(evlist: *mut perf_evlist, overwrite: bool_) -> *mut perf_mmap {
    let map = zalloc((*evlist).nr_mmaps as usize * size_of::<perf_mmap>()) as *mut perf_mmap;
    if map.is_null() {
        return ptr::null_mut();
    }

    let mut i = 0;
    while i < (*evlist).nr_mmaps {
        let prev = if i != 0 { map.add((i - 1) as usize) } else { ptr::null_mut() };

        /*
         * When the perf_mmap() call is made we grab one refcount, plus
         * one extra to let perf_mmap__consume() get the last
         * events after all real references (perf_mmap__get()) are
         * dropped.
         *
         * Each PERF_EVENT_IOC_SET_OUTPUT points to this mmap and
         * thus does perf_mmap__get() on it.
         */
        perf_mmap__init(map.add(i as usize), prev, overwrite, ptr::null_mut());
        i += 1;
    }

    map
}

unsafe fn perf_evsel__set_sid_idx(evsel: *mut perf_evsel, idx: i32, cpu: i32, thread: i32) {
    let sid = SID(evsel, cpu, thread);

    (*sid).idx = idx;
    (*sid).cpu = perf_cpu_map__cpu((*evsel).cpus, cpu);
    (*sid).tid = perf_thread_map__pid((*evsel).threads, thread);
}

unsafe extern "C" fn perf_evlist__mmap_cb_get(
    evlist: *mut perf_evlist,
    overwrite: bool_,
    idx: i32,
) -> *mut perf_mmap {
    let mut maps = if overwrite { (*evlist).mmap_ovw } else { (*evlist).mmap };

    if maps.is_null() {
        maps = perf_evlist__alloc_mmap(evlist, overwrite);
        if maps.is_null() {
            return ptr::null_mut();
        }

        if overwrite {
            (*evlist).mmap_ovw = maps;
        } else {
            (*evlist).mmap = maps;
        }
    }

    maps.add(idx as usize)
}

unsafe extern "C" fn perf_evlist__mmap_cb_mmap(
    map: *mut perf_mmap,
    mp: *mut perf_mmap_param,
    output: i32,
    cpu: perf_cpu,
) -> i32 {
    perf_mmap__mmap(map, mp, output, cpu)
}

unsafe fn perf_evlist__set_mmap_first(
    evlist: *mut perf_evlist,
    map: *mut perf_mmap,
    overwrite: bool_,
) {
    if overwrite {
        (*evlist).mmap_ovw_first = map;
    } else {
        (*evlist).mmap_first = map;
    }
}

unsafe fn mmap_per_evsel(
    evlist: *mut perf_evlist,
    ops: *mut perf_evlist_mmap_ops,
    idx: i32,
    mp: *mut perf_mmap_param,
    cpu_idx: i32,
    thread: i32,
    _output: *mut i32,
    _output_overwrite: *mut i32,
    nr_mmaps: *mut i32,
) -> i32 {
    let evlist_cpu = perf_cpu_map__cpu((*evlist).all_cpus, cpu_idx);
    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        let overwrite = (*evsel).attr.write_backward;

        if (*evsel).system_wide && thread != 0 {
            node = (*node).next;
            continue;
        }

        let cpu = perf_cpu_map__idx((*evsel).cpus, evlist_cpu);
        if cpu == -1 {
            node = (*node).next;
            continue;
        }

        let map = ((*ops).get.unwrap())(evlist, overwrite, idx);
        if map.is_null() {
            return -ENOMEM;
        }

        let output = if overwrite {
            (*mp).prot = PROT_READ;
            _output_overwrite
        } else {
            (*mp).prot = PROT_READ | PROT_WRITE;
            _output
        };

        let fd = FD(evsel, cpu, thread);

        if *output == -1 {
            *output = fd;

            /*
             * The last one will be done at perf_mmap__consume(), so that we
             * make sure we don't prevent tools from consuming every last event in
             * the ring buffer.
             *
             * I.e. we can get the POLLHUP meaning that the fd doesn't exist
             * anymore, but the last events for it are still in the ring buffer,
             * waiting to be consumed.
             *
             * Tools can chose to ignore this at their own discretion, but the
             * evlist layer can't just drop it when filtering events in
             * perf_evlist__filter_pollfd().
             */
            refcount_set(&mut (*map).refcnt, 2);

            if let Some(idx_fn) = (*ops).idx {
                idx_fn(evlist, evsel, mp, idx);
            }

            /* Debug message used by test scripts */
            pr_debug(b"idx %d: mmapping fd %d\n\0".as_ptr(), idx, *output);
            if ((*ops).mmap.unwrap())(map, mp, *output, evlist_cpu) < 0 {
                return -1;
            }

            *nr_mmaps += 1;

            if idx == 0 {
                perf_evlist__set_mmap_first(evlist, map, overwrite);
            }
        } else {
            /* Debug message used by test scripts */
            pr_debug(b"idx %d: set output fd %d -> %d\n\0".as_ptr(), idx, fd, *output);
            if ioctl(fd, PERF_EVENT_IOC_SET_OUTPUT, *output) != 0 {
                return -1;
            }

            perf_mmap__get(map);
        }

        let revent = if !overwrite { POLLIN } else { 0 };

        let flgs = if (*evsel).system_wide {
            fdarray_flags::fdarray_flag__nonfilterable
        } else {
            fdarray_flags::fdarray_flag__default
        };
        if perf_evlist__add_pollfd(evlist, fd, map as *mut c_void, revent, flgs) < 0 {
            perf_mmap__put(map);
            return -1;
        }

        if (*evsel).attr.read_format & PERF_FORMAT_ID != 0 {
            if perf_evlist__id_add_fd(evlist, evsel, cpu, thread, fd) < 0 {
                return -1;
            }
            perf_evsel__set_sid_idx(evsel, idx, cpu, thread);
        }

        node = (*node).next;
    }

    0
}

unsafe fn mmap_per_thread(
    evlist: *mut perf_evlist,
    ops: *mut perf_evlist_mmap_ops,
    mp: *mut perf_mmap_param,
) -> i32 {
    let nr_threads = perf_thread_map__nr((*evlist).threads);
    let nr_cpus = perf_cpu_map__nr((*evlist).all_cpus);
    let mut idx = 0;
    let mut nr_mmaps = 0;

    pr_debug(
        b"%s: nr cpu values (may include -1) %d nr threads %d\n\0".as_ptr(),
        b"mmap_per_thread\0".as_ptr(),
        nr_cpus,
        nr_threads,
    );

    /* per-thread mmaps */
    let mut thread = 0;
    while thread < nr_threads {
        let mut output = -1;
        let mut output_overwrite = -1;

        if mmap_per_evsel(
            evlist,
            ops,
            idx,
            mp,
            0,
            thread,
            &mut output,
            &mut output_overwrite,
            &mut nr_mmaps,
        ) != 0
        {
            perf_evlist__munmap(evlist);
            return -1;
        }
        thread += 1;
        idx += 1;
    }

    /* system-wide mmaps i.e. per-cpu */
    let mut cpu = 1;
    while cpu < nr_cpus {
        let mut output = -1;
        let mut output_overwrite = -1;

        if mmap_per_evsel(
            evlist,
            ops,
            idx,
            mp,
            cpu,
            0,
            &mut output,
            &mut output_overwrite,
            &mut nr_mmaps,
        ) != 0
        {
            perf_evlist__munmap(evlist);
            return -1;
        }
        cpu += 1;
        idx += 1;
    }

    if nr_mmaps != (*evlist).nr_mmaps {
        pr_err(
            b"Miscounted nr_mmaps %d vs %d\n\0".as_ptr(),
            nr_mmaps,
            (*evlist).nr_mmaps,
        );
    }

    0
}

unsafe fn mmap_per_cpu(
    evlist: *mut perf_evlist,
    ops: *mut perf_evlist_mmap_ops,
    mp: *mut perf_mmap_param,
) -> i32 {
    let nr_threads = perf_thread_map__nr((*evlist).threads);
    let nr_cpus = perf_cpu_map__nr((*evlist).all_cpus);
    let mut nr_mmaps = 0;

    pr_debug(
        b"%s: nr cpu values %d nr threads %d\n\0".as_ptr(),
        b"mmap_per_cpu\0".as_ptr(),
        nr_cpus,
        nr_threads,
    );

    let mut cpu = 0;
    while cpu < nr_cpus {
        let mut output = -1;
        let mut output_overwrite = -1;

        let mut thread = 0;
        while thread < nr_threads {
            if mmap_per_evsel(
                evlist,
                ops,
                cpu,
                mp,
                cpu,
                thread,
                &mut output,
                &mut output_overwrite,
                &mut nr_mmaps,
            ) != 0
            {
                perf_evlist__munmap(evlist);
                return -1;
            }
            thread += 1;
        }
        cpu += 1;
    }

    if nr_mmaps != (*evlist).nr_mmaps {
        pr_err(
            b"Miscounted nr_mmaps %d vs %d\n\0".as_ptr(),
            nr_mmaps,
            (*evlist).nr_mmaps,
        );
    }

    0
}

unsafe fn perf_evlist__nr_mmaps(evlist: *mut perf_evlist) -> i32 {
    let mut nr_mmaps: i32;

    /* One for each CPU */
    nr_mmaps = perf_cpu_map__nr((*evlist).all_cpus);
    if perf_cpu_map__has_any_cpu_or_is_empty((*evlist).all_cpus) {
        /* Plus one for each thread */
        nr_mmaps += perf_thread_map__nr((*evlist).threads);
        /* Minus the per-thread CPU (-1) */
        nr_mmaps -= 1;
    }

    nr_mmaps
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__mmap_ops(
    evlist: *mut perf_evlist,
    ops: *mut perf_evlist_mmap_ops,
    mp: *mut perf_mmap_param,
) -> i32 {
    let cpus = (*evlist).all_cpus as *const perf_cpu_map;

    if ops.is_null() || (*ops).get.is_none() || (*ops).mmap.is_none() {
        return -EINVAL;
    }

    (*mp).mask = (*evlist).mmap_len - page_size - 1;

    (*evlist).nr_mmaps = perf_evlist__nr_mmaps(evlist);

    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        if ((*evsel).attr.read_format & PERF_FORMAT_ID) != 0
            && (*evsel).sample_id.is_null()
            && perf_evsel__alloc_id(evsel, (*(*evsel).fd).max_x, (*(*evsel).fd).max_y) < 0
        {
            return -ENOMEM;
        }
        node = (*node).next;
    }

    if (*evlist).pollfd.entries.is_null() && perf_evlist__alloc_pollfd(evlist) < 0 {
        return -ENOMEM;
    }

    if perf_cpu_map__has_any_cpu_or_is_empty(cpus) {
        return mmap_per_thread(evlist, ops, mp);
    }

    mmap_per_cpu(evlist, ops, mp)
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__mmap(evlist: *mut perf_evlist, pages: i32) -> i32 {
    let mut mp: perf_mmap_param = core::mem::zeroed();
    let mut ops = perf_evlist_mmap_ops {
        get: Some(perf_evlist__mmap_cb_get),
        mmap: Some(perf_evlist__mmap_cb_mmap),
        idx: None,
    };

    (*evlist).mmap_len = (pages as usize + 1) * page_size;

    perf_evlist__mmap_ops(evlist, &mut ops, &mut mp)
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__munmap(evlist: *mut perf_evlist) {
    let mut i: i32;

    if !(*evlist).mmap.is_null() {
        i = 0;
        while i < (*evlist).nr_mmaps {
            perf_mmap__munmap((*evlist).mmap.add(i as usize));
            i += 1;
        }
    }

    if !(*evlist).mmap_ovw.is_null() {
        i = 0;
        while i < (*evlist).nr_mmaps {
            perf_mmap__munmap((*evlist).mmap_ovw.add(i as usize));
            i += 1;
        }
    }

    zfree(&mut (*evlist).mmap as *mut *mut perf_mmap as *mut c_void);
    zfree(&mut (*evlist).mmap_ovw as *mut *mut perf_mmap as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__next_mmap(
    evlist: *mut perf_evlist,
    map: *mut perf_mmap,
    overwrite: bool_,
) -> *mut perf_mmap {
    if !map.is_null() {
        return (*map).next;
    }

    if overwrite {
        (*evlist).mmap_ovw_first
    } else {
        (*evlist).mmap_first
    }
}

#[no_mangle]
pub unsafe extern "C" fn __perf_evlist__set_leader(
    list: *mut list_head,
    leader: *mut perf_evsel,
) {
    let mut n = 0;

    let mut node = (*list).next;
    while node != list {
        let evsel = list_entry_perf_evsel(node);
        (*evsel).leader = leader;
        n += 1;
        node = (*node).next;
    }
    (*leader).nr_members = n;
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__set_leader(evlist: *mut perf_evlist) {
    if (*evlist).nr_entries != 0 {
        let first = list_entry_perf_evsel((*evlist).entries.next);

        __perf_evlist__set_leader(&mut (*evlist).entries, first);
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__nr_groups(evlist: *mut perf_evlist) -> i32 {
    let mut nr_groups = 0;

    let mut node = (*evlist).entries.next;
    while node != &mut (*evlist).entries {
        let evsel = list_entry_perf_evsel(node);
        /*
         * evsels by default have a nr_members of 1, and they are their
         * own leader. If the nr_members is >1 then this is an
         * indication of a group.
         */
        if (*evsel).leader == evsel && (*evsel).nr_members > 1 {
            nr_groups += 1;
        }
        node = (*node).next;
    }
    nr_groups
}

#[no_mangle]
pub unsafe extern "C" fn perf_evlist__go_system_wide(
    evlist: *mut perf_evlist,
    evsel: *mut perf_evsel,
) {
    if !(*evsel).system_wide {
        (*evsel).system_wide = true;
        if (*evlist).needs_map_propagation {
            __perf_evlist__propagate_maps(evlist, evsel);
        }
    }
}
