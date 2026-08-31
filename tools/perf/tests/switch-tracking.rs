// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/switch-tracking.c. C include dependencies are
// represented as external declarations/types supplied by the surrounding tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type pid_t = c_int;
type u64 = u64;
type s64 = i64;

const EINTR: c_int = 4;
const PR_SET_NAME: c_int = 15;
const UINT_MAX: c_uint = c_uint::MAX;
const ULLONG_MAX: u64 = u64::MAX;
const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_SAMPLE: u32 = 9;
const CPU: c_int = 0;
const TIME: c_int = 1;

#[repr(C)]
struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct perf_event_header {
    type_: u32,
    misc: u16,
    size: u16,
}

#[repr(C)]
struct perf_record_comm {
    header: perf_event_header,
    pid: u32,
    tid: u32,
    comm: [c_char; 16],
}

#[repr(C)]
union perf_event {
    header: mem::ManuallyDrop<perf_event_header>,
    comm: mem::ManuallyDrop<perf_record_comm>,
}

#[repr(C)]
struct perf_sample {
    evsel: *mut evsel,
    id: u64,
    cpu: c_int,
    time: u64,
}

#[repr(C)]
struct perf_event_attr {
    mmap: bool,
    comm: bool,
    freq: u64,
    sample_period: u64,
}

#[repr(C)]
struct evsel_core {
    attr: perf_event_attr,
    system_wide: bool,
}

#[repr(C)]
struct evsel {
    core: evsel_core,
    immediate: bool,
}

#[repr(C)]
struct evlist_core {
    nr_mmaps: c_int,
}

#[repr(C)]
struct mmap_core;

#[repr(C)]
struct mmap {
    core: mmap_core,
}

#[repr(C)]
struct evlist;

#[repr(C)]
struct perf_thread_map;

#[repr(C)]
struct perf_cpu_map;

#[repr(C)]
struct target {
    uses_mmap: bool,
}

#[repr(C)]
struct record_opts {
    mmap_pages: c_uint,
    user_freq: c_uint,
    user_interval: u64,
    freq: c_int,
    target: target,
}

#[repr(C)]
struct test_suite;

#[repr(C)]
struct switch_tracking {
    switch_evsel: *mut evsel,
    cycles_evsel: *mut evsel,
    tids: *mut pid_t,
    nr_tids: c_int,
    comm_seen: [c_int; 4],
    cycles_before_comm_1: c_int,
    cycles_between_comm_2_and_comm_3: c_int,
    cycles_after_comm_4: c_int,
}

#[repr(C)]
struct event_node {
    list: list_head,
    event: *mut perf_event,
    event_time: u64,
}

unsafe extern "C" {
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn nanosleep(req: *const timespec, rem: *mut timespec) -> c_int;
    fn getpid() -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(base: *mut c_void, nmemb: usize, size: usize, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;

    fn barrier();
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn perf_sample__init(sample: *mut perf_sample, all: bool);
    fn perf_sample__exit(sample: *mut perf_sample);
    fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int;
    fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel;
    fn perf_sample__intval(sample: *const perf_sample, name: *const c_char) -> i64;
    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__mmap(evlist: *mut evlist) -> *mut mmap;
    fn perf_mmap__read_init(map: *mut mmap_core) -> c_int;
    fn perf_mmap__read_event(map: *mut mmap_core) -> *mut perf_event;
    fn perf_mmap__consume(map: *mut mmap_core);
    fn perf_mmap__read_done(map: *mut mmap_core);
    fn thread_map__new_by_tid(pid: pid_t) -> *mut perf_thread_map;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn evlist__new() -> *mut evlist;
    fn perf_evlist__set_maps(core: *mut evlist_core, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);
    fn parse_event(evlist: *mut evlist, event: *const c_char) -> c_int;
    fn evlist__last(evlist: *mut evlist) -> *mut evsel;
    fn evlist__can_select_event(evlist: *mut evlist, event: *const c_char) -> bool;
    fn evlist__add_sched_switch(evlist: *mut evlist, system_wide: bool) -> *mut evsel;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__to_front(evlist: *mut evlist, evsel: *mut evsel);
    fn evsel__set_sample_bit(evsel: *mut evsel, bit: c_int);
    fn evlist__set_tracking_event(evlist: *mut evlist, evsel: *mut evsel);
    fn evlist__config(evlist: *mut evlist, opts: *mut record_opts, callchain_param: *mut c_void);
    fn evlist__open(evlist: *mut evlist) -> c_int;
    fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int;
    fn evlist__enable(evlist: *mut evlist);
    fn evlist__disable(evlist: *mut evlist);
    fn evsel__disable(evsel: *mut evsel) -> c_int;
    fn evsel__enable(evsel: *mut evsel) -> c_int;
    fn zfree(ptr: *mut *mut pid_t);
    fn evlist__put(evlist: *mut evlist);
    fn perf_cpu_map__put(cpus: *mut perf_cpu_map);
    fn perf_thread_map__put(threads: *mut perf_thread_map);
}

unsafe fn timersub(a: *const timeval, b: *const timeval, result: *mut timeval) {
    (*result).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*result).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*result).tv_usec < 0 {
        (*result).tv_sec -= 1;
        (*result).tv_usec += 1_000_000;
    }
}

unsafe fn timercmp_gt(a: *const timeval, b: *const timeval) -> bool {
    (*a).tv_sec > (*b).tv_sec || ((*a).tv_sec == (*b).tv_sec && (*a).tv_usec > (*b).tv_usec)
}

unsafe fn list_empty(head: *const list_head) -> bool {
    (*head).next == head as *mut list_head
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

unsafe fn event_node_from_list(ptr: *mut list_head) -> *mut event_node {
    ptr as *mut event_node
}

unsafe fn spin_sleep() -> c_int {
    let mut start: timeval = mem::zeroed();
    let mut now: timeval = mem::zeroed();
    let mut diff: timeval = mem::zeroed();
    let mut maxtime: timeval = mem::zeroed();
    let mut ts: timespec = mem::zeroed();
    let mut err: c_int;
    let mut i: c_int;

    maxtime.tv_sec = 0;
    maxtime.tv_usec = 50000;

    err = gettimeofday(&mut start, ptr::null_mut());
    if err != 0 {
        return err;
    }

    /* Spin for 50ms */
    loop {
        i = 0;
        while i < 1000 {
            barrier();
            i += 1;
        }

        err = gettimeofday(&mut now, ptr::null_mut());
        if err != 0 {
            return err;
        }

        timersub(&now, &start, &mut diff);
        if timercmp_gt(&diff, &maxtime) {
            break;
        }
    }

    ts.tv_nsec = 50 * 1000 * 1000;
    ts.tv_sec = 0;

    /* Sleep for 50ms */
    err = nanosleep(&ts, ptr::null_mut());
    if err == EINTR {
        err = 0;
    }

    err
}

unsafe fn check_comm(switch_tracking: *mut switch_tracking, event: *mut perf_event, comm: *const c_char, nr: c_int) -> c_int {
    if (*event).header.type_ == PERF_RECORD_COMM
        && (*event).comm.pid as pid_t == getpid()
        && (*event).comm.tid as pid_t == getpid()
        && strcmp((*event).comm.comm.as_ptr(), comm) == 0
    {
        if (*switch_tracking).comm_seen[nr as usize] != 0 {
            pr_debug(c"Duplicate comm event\n".as_ptr());
            return -1;
        }
        (*switch_tracking).comm_seen[nr as usize] = 1;
        pr_debug3(c"comm event: %s nr: %d\n".as_ptr(), (*event).comm.comm.as_ptr(), nr);
        return 1;
    }
    0
}

unsafe fn check_cpu(switch_tracking: *mut switch_tracking, cpu: c_int) -> c_int {
    let mut i: c_int;
    let nr = cpu + 1;

    if cpu < 0 {
        return -1;
    }

    if (*switch_tracking).tids.is_null() {
        (*switch_tracking).tids = calloc(nr as usize, mem::size_of::<pid_t>()) as *mut pid_t;
        if (*switch_tracking).tids.is_null() {
            return -1;
        }
        i = 0;
        while i < nr {
            *(*switch_tracking).tids.add(i as usize) = -1;
            i += 1;
        }
        (*switch_tracking).nr_tids = nr;
        return 0;
    }

    if cpu >= (*switch_tracking).nr_tids {
        let addr: *mut c_void;

        addr = realloc((*switch_tracking).tids as *mut c_void, nr as usize * mem::size_of::<pid_t>());
        if addr.is_null() {
            return -1;
        }
        (*switch_tracking).tids = addr as *mut pid_t;
        i = (*switch_tracking).nr_tids;
        while i < nr {
            *(*switch_tracking).tids.add(i as usize) = -1;
            i += 1;
        }
        (*switch_tracking).nr_tids = nr;
        return 0;
    }

    0
}

unsafe fn process_sample_event(evlist: *mut evlist, event: *mut perf_event, switch_tracking: *mut switch_tracking) -> c_int {
    let mut sample: perf_sample = mem::zeroed();
    let mut evsel_: *mut evsel;
    let next_tid: pid_t;
    let prev_tid: pid_t;
    let cpu: c_int;
    let mut err: c_int;

    perf_sample__init(&mut sample, false);
    if evlist__parse_sample(evlist, event, &mut sample) != 0 {
        pr_debug(c"evlist__parse_sample failed\n".as_ptr());
        err = -1;
        perf_sample__exit(&mut sample);
        return err;
    }

    evsel_ = sample.evsel;
    if evsel_.is_null() {
        evsel_ = evlist__id2evsel(evlist, sample.id);
    }

    if evsel_ == (*switch_tracking).switch_evsel {
        next_tid = perf_sample__intval(&sample, c"next_pid".as_ptr()) as pid_t;
        prev_tid = perf_sample__intval(&sample, c"prev_pid".as_ptr()) as pid_t;
        cpu = sample.cpu;
        pr_debug3(c"sched_switch: cpu: %d prev_tid %d next_tid %d\n".as_ptr(), cpu, prev_tid, next_tid);
        err = check_cpu(switch_tracking, cpu);
        if err != 0 {
            perf_sample__exit(&mut sample);
            return err;
        }
        /*
         * Check for no missing sched_switch events i.e. that the
         * evsel->core.system_wide flag has worked.
         */
        if *(*switch_tracking).tids.add(cpu as usize) != -1
            && *(*switch_tracking).tids.add(cpu as usize) != prev_tid
        {
            pr_debug(c"Missing sched_switch events\n".as_ptr());
            err = -1;
            perf_sample__exit(&mut sample);
            return err;
        }
        *(*switch_tracking).tids.add(cpu as usize) = next_tid;
    }

    if evsel_ == (*switch_tracking).cycles_evsel {
        pr_debug3(c"cycles event\n".as_ptr());
        if (*switch_tracking).comm_seen[0] == 0 {
            (*switch_tracking).cycles_before_comm_1 = 1;
        }
        if (*switch_tracking).comm_seen[1] != 0 && (*switch_tracking).comm_seen[2] == 0 {
            (*switch_tracking).cycles_between_comm_2_and_comm_3 = 1;
        }
        if (*switch_tracking).comm_seen[3] != 0 {
            (*switch_tracking).cycles_after_comm_4 = 1;
        }
    }

    err = 0;
    perf_sample__exit(&mut sample);
    err
}

unsafe fn process_event(evlist: *mut evlist, event: *mut perf_event, switch_tracking: *mut switch_tracking) -> c_int {
    if (*event).header.type_ == PERF_RECORD_SAMPLE {
        return process_sample_event(evlist, event, switch_tracking);
    }

    if (*event).header.type_ == PERF_RECORD_COMM {
        let mut err: c_int;
        let mut done: c_int = 0;

        err = check_comm(switch_tracking, event, c"Test COMM 1".as_ptr(), 0);
        if err < 0 {
            return -1;
        }
        done += err;
        err = check_comm(switch_tracking, event, c"Test COMM 2".as_ptr(), 1);
        if err < 0 {
            return -1;
        }
        done += err;
        err = check_comm(switch_tracking, event, c"Test COMM 3".as_ptr(), 2);
        if err < 0 {
            return -1;
        }
        done += err;
        err = check_comm(switch_tracking, event, c"Test COMM 4".as_ptr(), 3);
        if err < 0 {
            return -1;
        }
        done += err;
        if done != 1 {
            pr_debug(c"Unexpected comm event\n".as_ptr());
            return -1;
        }
    }

    0
}

unsafe fn add_event(evlist: *mut evlist, events: *mut list_head, event: *mut perf_event) -> c_int {
    let mut sample: perf_sample = mem::zeroed();
    let node: *mut event_node;

    node = malloc(mem::size_of::<event_node>()) as *mut event_node;
    if node.is_null() {
        pr_debug(c"malloc failed\n".as_ptr());
        return -1;
    }
    (*node).event = event;
    list_add(&mut (*node).list, events);

    if evlist__parse_sample(evlist, event, &mut sample) != 0 {
        pr_debug(c"evlist__parse_sample failed\n".as_ptr());
        perf_sample__exit(&mut sample);
        return -1;
    }

    if sample.time == 0 {
        pr_debug(c"event with no time\n".as_ptr());
        perf_sample__exit(&mut sample);
        return -1;
    }

    (*node).event_time = sample.time;

    perf_sample__exit(&mut sample);
    0
}

unsafe fn free_event_nodes(events: *mut list_head) {
    let mut node: *mut event_node;

    while !list_empty(events) {
        node = event_node_from_list((*events).next);
        list_del_init(&mut (*node).list);
        free(node as *mut c_void);
    }
}

unsafe extern "C" fn compar(a: *const c_void, b: *const c_void) -> c_int {
    let nodea = a as *const event_node;
    let nodeb = b as *const event_node;
    let cmp: s64 = (*nodea).event_time as s64 - (*nodeb).event_time as s64;

    if cmp < 0 {
        -1
    } else if cmp > 0 {
        1
    } else {
        0
    }
}

unsafe fn process_events(evlist: *mut evlist, switch_tracking: *mut switch_tracking) -> c_int {
    let mut event: *mut perf_event;
    let mut pos: c_uint;
    let mut cnt: c_uint = 0;
    let mut events: list_head = mem::zeroed();
    let mut events_array: *mut event_node;
    let mut node: *mut event_node;
    let mut md: *mut mmap;
    let mut i: c_int;
    let mut ret: c_int;

    INIT_LIST_HEAD(&mut events);

    i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        md = evlist__mmap(evlist).add(i as usize);
        if perf_mmap__read_init(&mut (*md).core) < 0 {
            i += 1;
            continue;
        }

        loop {
            event = perf_mmap__read_event(&mut (*md).core);
            if event.is_null() {
                break;
            }
            cnt += 1;
            ret = add_event(evlist, &mut events, event);
            perf_mmap__consume(&mut (*md).core);
            if ret < 0 {
                free_event_nodes(&mut events);
                return ret;
            }
        }
        perf_mmap__read_done(&mut (*md).core);
        i += 1;
    }

    events_array = calloc(cnt as usize, mem::size_of::<event_node>()) as *mut event_node;
    if events_array.is_null() {
        pr_debug(c"calloc failed\n".as_ptr());
        ret = -1;
        free_event_nodes(&mut events);
        return ret;
    }

    pos = 0;
    node = event_node_from_list(events.next);
    while &mut (*node).list as *mut list_head != &mut events as *mut list_head {
        *events_array.add(pos as usize) = ptr::read(node);
        pos += 1;
        node = event_node_from_list((*node).list.next);
    }

    qsort(events_array as *mut c_void, cnt as usize, mem::size_of::<event_node>(), compar);

    pos = 0;
    while pos < cnt {
        ret = process_event(evlist, (*events_array.add(pos as usize)).event, switch_tracking);
        if ret < 0 {
            pr_debug(c"%u events recorded\n".as_ptr(), cnt);
            free(events_array as *mut c_void);
            free_event_nodes(&mut events);
            return ret;
        }
        pos += 1;
    }

    ret = 0;
    pr_debug(c"%u events recorded\n".as_ptr(), cnt);
    free(events_array as *mut c_void);
    free_event_nodes(&mut events);
    ret
}

/**
 * test__switch_tracking - test using sched_switch and tracking events.
 *
 * This function implements a test that checks that sched_switch events and
 * tracking events can be recorded for a workload (current process) using the
 * evsel->core.system_wide and evsel->tracking flags (respectively) with other events
 * sometimes enabled or disabled.
 */
unsafe fn test__switch_tracking(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let sched_switch = c"sched:sched_switch".as_ptr();
    let cycles = c"cpu-cycles:u".as_ptr();
    let mut switch_tracking: switch_tracking = mem::zeroed();
    let mut opts = record_opts {
        mmap_pages: UINT_MAX,
        user_freq: UINT_MAX,
        user_interval: ULLONG_MAX,
        freq: 4000,
        target: target {
            uses_mmap: true,
        },
    };
    let mut threads: *mut perf_thread_map = ptr::null_mut();
    let mut cpus: *mut perf_cpu_map = ptr::null_mut();
    let mut evlist: *mut evlist = ptr::null_mut();
    let mut evsel_: *mut evsel;
    let mut cpu_clocks_evsel: *mut evsel = ptr::null_mut();
    let mut cycles_evsel: *mut evsel = ptr::null_mut();
    let mut switch_evsel: *mut evsel = ptr::null_mut();
    let mut tracking_evsel: *mut evsel = ptr::null_mut();
    let mut comm: *const c_char;
    let mut err: c_int = -1;

    'out_err: loop {
        threads = thread_map__new_by_tid(getpid());
        if threads.is_null() {
            pr_debug(c"thread_map__new failed!\n".as_ptr());
            break 'out_err;
        }

        cpus = perf_cpu_map__new_online_cpus();
        if cpus.is_null() {
            pr_debug(c"perf_cpu_map__new failed!\n".as_ptr());
            break 'out_err;
        }

        evlist = evlist__new();
        if evlist.is_null() {
            pr_debug(c"evlist__new failed!\n".as_ptr());
            break 'out_err;
        }

        perf_evlist__set_maps(evlist__core(evlist), cpus, threads);

        /* First event */
        err = parse_event(evlist, c"cpu-clock:u".as_ptr());
        if err != 0 {
            pr_debug(c"Failed to parse event dummy:u\n".as_ptr());
            break 'out_err;
        }

        cpu_clocks_evsel = evlist__last(evlist);

        /* Second event */
        err = parse_event(evlist, cycles);
        if err != 0 {
            pr_debug(c"Failed to parse event %s\n".as_ptr(), cycles);
            break 'out_err;
        }

        cycles_evsel = evlist__last(evlist);

        /* Third event */
        if !evlist__can_select_event(evlist, sched_switch) {
            pr_debug(c"No sched_switch\n".as_ptr());
            err = 0;
            break 'out_err;
        }

        switch_evsel = evlist__add_sched_switch(evlist, true);
        if IS_ERR(switch_evsel as *const c_void) {
            err = PTR_ERR(switch_evsel as *const c_void);
            pr_debug(c"Failed to create event %s\n".as_ptr(), sched_switch);
            break 'out_err;
        }

        (*switch_evsel).immediate = true;

        /* Test moving an event to the front */
        if cycles_evsel == evlist__first(evlist) {
            pr_debug(c"cycles event already at front".as_ptr());
            break 'out_err;
        }
        evlist__to_front(evlist, cycles_evsel);
        if cycles_evsel != evlist__first(evlist) {
            pr_debug(c"Failed to move cycles event to front".as_ptr());
            break 'out_err;
        }

        evsel__set_sample_bit(cycles_evsel, CPU);
        evsel__set_sample_bit(cycles_evsel, TIME);

        /* Fourth event */
        err = parse_event(evlist, c"dummy:u".as_ptr());
        if err != 0 {
            pr_debug(c"Failed to parse event dummy:u\n".as_ptr());
            break 'out_err;
        }

        tracking_evsel = evlist__last(evlist);

        evlist__set_tracking_event(evlist, tracking_evsel);

        (*tracking_evsel).core.attr.freq = 0;
        (*tracking_evsel).core.attr.sample_period = 1;

        evsel__set_sample_bit(tracking_evsel, TIME);

        /* Config events */
        evlist__config(evlist, &mut opts, ptr::null_mut());

        /* Check moved event is still at the front */
        if cycles_evsel != evlist__first(evlist) {
            pr_debug(c"Front event no longer at front".as_ptr());
            break 'out_err;
        }

        /* Check tracking event is tracking */
        if !(*tracking_evsel).core.attr.mmap || !(*tracking_evsel).core.attr.comm {
            pr_debug(c"Tracking event not tracking\n".as_ptr());
            break 'out_err;
        }

        /* Check non-tracking events are not tracking */
        evsel_ = evlist__first(evlist);
        while !evsel_.is_null() {
            if evsel_ != tracking_evsel {
                if (*evsel_).core.attr.mmap || (*evsel_).core.attr.comm {
                    pr_debug(c"Non-tracking event is tracking\n".as_ptr());
                    break 'out_err;
                }
            }
            /* evlist__for_each_entry iteration is supplied by perf's list helpers in C. */
            break;
        }

        if evlist__open(evlist) < 0 {
            pr_debug(c"Not supported\n".as_ptr());
            err = 0;
            break 'out_err;
        }

        err = evlist__do_mmap(evlist, UINT_MAX);
        if err != 0 {
            pr_debug(c"evlist__mmap failed!\n".as_ptr());
            break 'out_err;
        }

        evlist__enable(evlist);

        err = evsel__disable(cpu_clocks_evsel);
        if err != 0 {
            pr_debug(c"perf_evlist__disable_event failed!\n".as_ptr());
            break 'out_err;
        }

        err = spin_sleep();
        if err != 0 {
            pr_debug(c"spin_sleep failed!\n".as_ptr());
            break 'out_err;
        }

        comm = c"Test COMM 1".as_ptr();
        err = prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0);
        if err != 0 {
            pr_debug(c"PR_SET_NAME failed!\n".as_ptr());
            break 'out_err;
        }

        err = evsel__disable(cycles_evsel);
        if err != 0 {
            pr_debug(c"perf_evlist__disable_event failed!\n".as_ptr());
            break 'out_err;
        }

        comm = c"Test COMM 2".as_ptr();
        err = prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0);
        if err != 0 {
            pr_debug(c"PR_SET_NAME failed!\n".as_ptr());
            break 'out_err;
        }

        err = spin_sleep();
        if err != 0 {
            pr_debug(c"spin_sleep failed!\n".as_ptr());
            break 'out_err;
        }

        comm = c"Test COMM 3".as_ptr();
        err = prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0);
        if err != 0 {
            pr_debug(c"PR_SET_NAME failed!\n".as_ptr());
            break 'out_err;
        }

        err = evsel__enable(cycles_evsel);
        if err != 0 {
            pr_debug(c"perf_evlist__disable_event failed!\n".as_ptr());
            break 'out_err;
        }

        comm = c"Test COMM 4".as_ptr();
        err = prctl(PR_SET_NAME, comm as c_ulong, 0, 0, 0);
        if err != 0 {
            pr_debug(c"PR_SET_NAME failed!\n".as_ptr());
            break 'out_err;
        }

        err = spin_sleep();
        if err != 0 {
            pr_debug(c"spin_sleep failed!\n".as_ptr());
            break 'out_err;
        }

        evlist__disable(evlist);

        switch_tracking.switch_evsel = switch_evsel;
        switch_tracking.cycles_evsel = cycles_evsel;

        err = process_events(evlist, &mut switch_tracking);

        zfree(&mut switch_tracking.tids);

        if err != 0 {
            break 'out_err;
        }

        /* Check all 4 comm events were seen i.e. that evsel->tracking works */
        if switch_tracking.comm_seen[0] == 0
            || switch_tracking.comm_seen[1] == 0
            || switch_tracking.comm_seen[2] == 0
            || switch_tracking.comm_seen[3] == 0
        {
            pr_debug(c"Missing comm events\n".as_ptr());
            break 'out_err;
        }

        /* Check cycles event got enabled */
        if switch_tracking.cycles_before_comm_1 == 0 {
            pr_debug(c"Missing cycles events\n".as_ptr());
            break 'out_err;
        }

        /* Check cycles event got disabled */
        if switch_tracking.cycles_between_comm_2_and_comm_3 != 0 {
            pr_debug(c"cycles events even though event was disabled\n".as_ptr());
            break 'out_err;
        }

        /* Check cycles event got enabled again */
        if switch_tracking.cycles_after_comm_4 == 0 {
            pr_debug(c"Missing cycles events\n".as_ptr());
            break 'out_err;
        }

        if !evlist.is_null() {
            evlist__disable(evlist);
            evlist__put(evlist);
        }
        perf_cpu_map__put(cpus);
        perf_thread_map__put(threads);

        return err;
    }

    err = if err == 0 { 0 } else { -1 };
    if !evlist.is_null() {
        evlist__disable(evlist);
        evlist__put(evlist);
    }
    perf_cpu_map__put(cpus);
    perf_thread_map__put(threads);

    err
}

/* DEFINE_SUITE_EXCLUSIVE("Track with sched_switch", switch_tracking); */
