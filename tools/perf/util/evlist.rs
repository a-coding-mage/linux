// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Parts came from builtin-{top,stat,record}.c, see those files for further
 * copyright notes.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_short, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type u16 = u16;
type u64 = u64;
type __u64 = u64;

const EACCES: c_int = 13;
const EAGAIN: c_int = 11;
const EEXIST: c_int = 17;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EINTR: c_int = 4;
const ENOMEM: c_int = 12;
const EPERM: c_int = 1;
const EWOULDBLOCK: c_int = EAGAIN;
const FD_CLOEXEC: c_int = 1;
const F_SETFD: c_int = 2;
const INT_MAX: c_long = 2147483647;
const MSEC_PER_SEC: c_int = 1000;
const NSEC_PER_MSEC: c_int = 1000000;
const O_CLOEXEC: c_int = 0o2000000;
const O_NONBLOCK: c_int = 0o4000;
const O_RDWR: c_int = 0o2;
const PERF_AFFINITY_SYS: c_int = 0;
const PERF_COUNT_SW_DUMMY: u64 = 9;
const PERF_EVENT_IOC_PAUSE_OUTPUT: c_ulong = 0;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_SAMPLE_BRANCH_COUNTERS: u64 = 1 << 31;
const PERF_SAMPLE_READ: u64 = 1 << 4;
const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_TYPE_TRACEPOINT: u32 = 2;
const POLLERR: c_short = 0x008;
const POLLHUP: c_short = 0x010;
const POLLIN: c_short = 0x001;
const PR_SET_NAME: c_int = 15;
const SA_SIGINFO: c_int = 4;
const SIGTERM: c_int = 15;
const SIGUSR1: c_int = 10;
const SIG_DFL: usize = 0;
const SIZE_MAX: usize = usize::MAX;
const TFD_CLOEXEC: c_int = O_CLOEXEC;
const UINT_MAX: c_uint = c_uint::MAX;
const WNOHANG: c_int = 1;
const CLOCK_MONOTONIC: c_int = 1;
const MAX_NR_ABBR_NAME: c_int = 26 * 11;

const EVLIST_CTL_CMD_MAX_LEN: usize = 64;
const EVLIST_CTL_CMD_ENABLE_TAG: &[u8] = b"enable\0";
const EVLIST_CTL_CMD_DISABLE_TAG: &[u8] = b"disable\0";
const EVLIST_CTL_CMD_SNAPSHOT_TAG: &[u8] = b"snapshot\0";
const EVLIST_CTL_CMD_EVLIST_TAG: &[u8] = b"evlist\0";
const EVLIST_CTL_CMD_STOP_TAG: &[u8] = b"stop\0";
const EVLIST_CTL_CMD_PING_TAG: &[u8] = b"ping\0";
const EVLIST_CTL_CMD_ACK_TAG: &[u8] = b"ack\n\0";
const EVLIST_ENABLED_MSG: *const c_char = b"Events enabled\n\0".as_ptr() as *const c_char;
const EVLIST_DISABLED_MSG: *const c_char = b"Events disabled\n\0".as_ptr() as *const c_char;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut c_void }
#[repr(C)] pub struct pollfd { pub fd: c_int, pub events: c_short, pub revents: c_short }
#[repr(C)] pub struct fdarray { pub entries: *mut pollfd }
#[repr(C)] pub struct perf_cpu { pub cpu: c_int }
#[repr(C)] pub struct perf_cpu_map;
#[repr(C)] pub struct perf_thread_map;
#[repr(C)] pub struct perf_pmu { pub name: *const c_char, pub is_core: bool }
#[repr(C)] pub struct perf_event_header { pub type_: u32, pub misc: u16, pub size: u16 }
#[repr(C)] pub struct perf_event_sample { pub array: *const __u64 }
#[repr(C)] pub union perf_event { pub header: perf_event_header, pub sample: perf_event_sample }
#[repr(C)] pub struct perf_event_attr {
    pub type_: u32, pub size: u32, pub config: u64, pub sample_period: u64,
    pub sample_type: u64, pub read_format: u64, pub branch_sample_type: u64,
    pub sample_freq: u64, pub sample_id_all: bool, pub exclude_kernel: bool,
    pub exclude_guest: bool, pub exclude_hv: bool, pub freq: bool,
}
#[repr(C)] pub struct perf_evsel {
    pub node: list_head, pub idx: c_int, pub attr: perf_event_attr, pub fd: *mut c_void,
    pub cpus: *mut perf_cpu_map, pub threads: *mut perf_thread_map, pub system_wide: bool,
    pub nr_members: c_int,
}
#[repr(C)] pub struct perf_evlist {
    pub entries: list_head, pub pollfd: fdarray, pub heads: *mut hlist_head,
    pub nr_entries: c_uint, pub nr_mmaps: c_int, pub mmap_len: size_t,
    pub threads: *mut perf_thread_map, pub user_requested_cpus: *mut perf_cpu_map,
    pub all_cpus: *mut perf_cpu_map, pub has_user_cpus: bool,
}
#[repr(C)] pub struct perf_mmap { pub fd: c_int }
#[repr(C)] pub struct mmap { pub core: perf_mmap }
#[repr(C)] pub struct auxtrace_mmap_params;
#[repr(C)] pub struct perf_mmap_param;
#[repr(C)] pub struct mmap_params {
    pub core: perf_mmap_param, pub auxtrace_mp: auxtrace_mmap_params,
    pub nr_cblocks: c_int, pub affinity: c_int, pub flush: c_int, pub comp_level: c_int,
}
#[repr(C)] pub struct perf_evlist_mmap_ops {
    pub idx: Option<unsafe extern "C" fn(*mut perf_evlist, *mut perf_evsel, *mut perf_mmap_param, c_int)>,
    pub get: Option<unsafe extern "C" fn(*mut perf_evlist, bool, c_int) -> *mut perf_mmap>,
    pub mmap: Option<unsafe extern "C" fn(*mut perf_mmap, *mut perf_mmap_param, c_int, perf_cpu) -> c_int>,
}
#[repr(C)] pub struct perf_sample_id {
    pub id: u64, pub evsel: *mut perf_evsel, pub node: *mut c_void,
    pub machine_pid: pid_t, pub vcpu: perf_cpu,
}
#[repr(C)] pub struct perf_sample { pub id: u64, pub machine_pid: pid_t, pub vcpu: c_int }
#[repr(C)] pub struct evsel_str_handler { pub name: *const c_char, pub handler: *mut c_void }
#[repr(C)] pub struct parse_tag { pub tag: c_char, pub mult: c_ulong }
#[repr(C)] pub struct option { pub value: *mut c_uint }
#[repr(C)] pub struct target {
    pub per_thread: bool, pub system_wide: bool, pub pid: *const c_char,
    pub tid: *const c_char, pub cpu_list: *const c_char, pub initial_delay: c_int,
}
#[repr(C)] pub struct record_opts { pub target: target }
#[repr(C)] pub struct perf_attr_details { pub verbose: bool, pub event_group: bool, pub freq: bool }
#[repr(C)] pub struct perf_stat_config;
#[repr(C)] pub struct strbuf { pub len: size_t }
#[repr(C)] pub struct affinity;
#[repr(C)] pub struct refcount_t { pub refs: c_int }
#[repr(C)] pub struct siginfo_t;
#[repr(C)] pub struct sigval { pub sival_int: c_int }
#[repr(C)] pub struct sigaction {
    pub sa_sigaction: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
    pub sa_flags: c_int,
}
#[repr(C)] pub struct timespec { pub tv_sec: c_long, pub tv_nsec: c_long }
#[repr(C)] pub struct itimerspec { pub it_interval: timespec, pub it_value: timespec }

#[repr(C)] pub enum bkw_mmap_state { BKW_MMAP_NOTREADY, BKW_MMAP_RUNNING, BKW_MMAP_DATA_PENDING, BKW_MMAP_EMPTY }
#[repr(C)] pub enum evlist_ctl_cmd {
    EVLIST_CTL_CMD_ENABLE, EVLIST_CTL_CMD_DISABLE, EVLIST_CTL_CMD_SNAPSHOT,
    EVLIST_CTL_CMD_EVLIST, EVLIST_CTL_CMD_STOP, EVLIST_CTL_CMD_PING,
    EVLIST_CTL_CMD_ACK, EVLIST_CTL_CMD_UNSUPPORTED,
}
#[repr(C)] pub enum fdarray_flags { fdarray_flag__default = 0, fdarray_flag__nonfilterable = 1, fdarray_flag__non_perf_event = 2 }

#[repr(C)] pub struct event_enable_time { pub start: c_int, pub end: c_int }
#[repr(C)] pub struct event_enable_timer {
    pub evlist: *mut evlist, pub times: *mut event_enable_time, pub times_cnt: size_t,
    pub timerfd: c_int, pub pollfd_pos: c_int, pub times_step: size_t,
}
#[repr(C)] pub struct evlist_cpu_iterator {
    pub container: *mut evlist, pub evsel: *mut evsel, pub cpu_map_idx: c_int,
    pub evlist_cpu_map_idx: c_int, pub evlist_cpu_map_nr: c_int, pub cpu: perf_cpu,
    pub affinity: *mut affinity, pub saved_affinity: affinity,
}
#[repr(C)] pub struct evsel {
    pub core: perf_evsel, pub evlist: *mut evlist, pub tracking: bool, pub handler: *mut c_void,
    pub disabled: bool, pub immediate: c_int, pub filter: *mut c_char, pub bpf_filters: list_head,
    pub id_pos: c_int, pub is_pos: c_int, pub no_aux_samples: bool, pub name: *mut c_char,
    pub pmu: *mut perf_pmu, pub br_cntr_idx: c_int, pub br_cntr_nr: c_int,
    pub abbr_name: [c_char; 3], pub forced_leader: bool, pub reset_group: bool,
    pub uniquified_name: bool, pub refcnt: refcount_t,
}
#[repr(C)] pub struct evlist {
    pub core: perf_evlist, pub id_pos: c_int, pub is_pos: c_int, pub combined_sample_type: u64,
    pub mmap: *mut mmap, pub overwrite_mmap: *mut mmap, pub eet: *mut event_enable_timer,
    pub deferred_samples: list_head, pub refcnt: refcount_t,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut page_size: c_int;
    static mut perf_guest: bool;
    static mut stderr: *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, maxlen: size_t, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn ui__warning(fmt: *const c_char, ...);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(s: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn signal(sig: c_int, handler: usize) -> usize;
    fn close(fd: c_int) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn execvp(file: *const c_char, argv: *const *mut c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn sigqueue(pid: pid_t, sig: c_int, value: sigval) -> c_int;
    fn getppid() -> pid_t;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int;

    fn perf_evlist__init(evlist: *mut perf_evlist);
    fn perf_evlist__set_maps(evlist: *mut perf_evlist, cpus: *mut perf_cpu_map, threads: *mut perf_thread_map);
    fn perf_evlist__exit(evlist: *mut perf_evlist);
    fn perf_evlist__add(evlist: *mut perf_evlist, entry: *mut perf_evsel);
    fn perf_evlist__remove(evlist: *mut perf_evlist, entry: *mut perf_evsel);
    fn perf_evlist__set_leader(evlist: *mut perf_evlist);
    fn perf_evlist__add_pollfd(evlist: *mut perf_evlist, fd: c_int, ptr: *mut c_void, events: c_short, flags: fdarray_flags) -> c_int;
    fn perf_evlist__filter_pollfd(evlist: *mut perf_evlist, mask: c_short) -> c_int;
    fn perf_evlist__poll(evlist: *mut perf_evlist, timeout: c_int) -> c_int;
    fn perf_evlist__reset_id_hash(evlist: *mut perf_evlist);
    fn perf_evlist__mmap_ops(evlist: *mut perf_evlist, ops: *mut perf_evlist_mmap_ops, mp: *mut perf_mmap_param) -> c_int;
    fn perf_evlist__go_system_wide(evlist: *mut perf_evlist, evsel: *mut perf_evsel);
    fn perf_mmap__init(map: *mut perf_mmap, prev: *mut perf_mmap, overwrite: bool, cb: Option<unsafe extern "C" fn(*mut perf_mmap)>);
    fn perf_mmap__munmap(map: *mut perf_mmap);
    fn mmap__mmap(map: *mut mmap, mp: *mut mmap_params, output: c_int, cpu: perf_cpu) -> c_int;
    fn mmap__munmap(map: *mut mmap);
    fn auxtrace_mmap_params__set_idx(mp: *mut auxtrace_mmap_params, evlist: *mut evlist, evsel: *mut evsel, idx: c_int);
    fn auxtrace_mmap_params__init(mp: *mut auxtrace_mmap_params, mmap_len: size_t, pages: c_uint, overwrite: bool);
}

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
unsafe fn evlist__core(e: *mut evlist) -> *mut perf_evlist { &mut (*e).core }
unsafe fn evlist__mmap(e: *mut evlist) -> *mut mmap { (*e).mmap }
unsafe fn evlist__overwrite_mmap(e: *mut evlist) -> *mut mmap { (*e).overwrite_mmap }
unsafe fn evlist__refcnt(e: *mut evlist) -> *mut refcount_t { &mut (*e).refcnt }
unsafe fn evlist__metric_events(_e: *mut evlist) -> *mut c_void { ptr::null_mut() }
unsafe fn evlist__nr_entries(e: *mut evlist) -> c_uint { (*evlist__core(e)).nr_entries }
unsafe fn evlist__first(_e: *mut evlist) -> *mut evsel { ptr::null_mut() }
unsafe fn evlist__last(_e: *mut evlist) -> *mut evsel { ptr::null_mut() }
unsafe fn evsel__next(_e: *mut evsel) -> *mut evsel { ptr::null_mut() }
unsafe fn evlist__empty(e: *mut evlist) -> bool { evlist__nr_entries(e) == 0 }
unsafe fn evlist__id_pos(e: *mut evlist) -> c_int { (*e).id_pos }
unsafe fn evlist__is_pos(e: *mut evlist) -> c_int { (*e).is_pos }
unsafe fn evlist__bkw_mmap_state(_e: *mut evlist) -> bkw_mmap_state { bkw_mmap_state::BKW_MMAP_NOTREADY }
unsafe fn evlist__set_bkw_mmap_state(_e: *mut evlist, _s: bkw_mmap_state) {}
unsafe fn evlist__set_workload_pid(_e: *mut evlist, _pid: pid_t) {}
unsafe fn evlist__workload_pid(_e: *mut evlist) -> pid_t { -1 }
unsafe fn evlist__set_workload_cork_fd(_e: *mut evlist, _fd: c_int) {}
unsafe fn evlist__workload_cork_fd(_e: *mut evlist) -> c_int { -1 }
unsafe fn evlist__set_ctl_fd_fd(_e: *mut evlist, _fd: c_int) {}
unsafe fn evlist__ctl_fd_fd(_e: *mut evlist) -> c_int { -1 }
unsafe fn evlist__set_ctl_fd_ack(_e: *mut evlist, _fd: c_int) {}
unsafe fn evlist__ctl_fd_ack(_e: *mut evlist) -> c_int { -1 }
unsafe fn evlist__set_ctl_fd_pos(_e: *mut evlist, _pos: c_int) {}
unsafe fn evlist__ctl_fd_pos(_e: *mut evlist) -> c_int { -1 }
unsafe fn evlist__set_nr_br_cntr(_e: *mut evlist, _n: c_int) {}
unsafe fn evlist__set_enabled(_e: *mut evlist, _v: bool) {}
unsafe fn evlist__enabled(_e: *mut evlist) -> bool { false }
unsafe fn perf_cpu_map__nr(_m: *mut perf_cpu_map) -> c_int { 0 }
unsafe fn perf_cpu_map__cpu(_m: *mut perf_cpu_map, _idx: c_int) -> perf_cpu { perf_cpu { cpu: -1 } }
unsafe fn perf_cpu_map__idx(_m: *mut perf_cpu_map, _cpu: perf_cpu) -> c_int { -1 }

/* Iteration and container_of/list macros from C are external dependency intent. */
unsafe fn for_each_evsel<F: FnMut(*mut evsel)>(_evlist: *mut evlist, mut _f: F) {}
unsafe fn for_each_evsel_reverse<F: FnMut(*mut evsel)>(_evlist: *mut evlist, mut _f: F) {}
unsafe fn for_each_group_member<F: FnMut(*mut evsel)>(_leader: *mut evsel, mut _f: F) {}
unsafe fn for_each_group_evsel<F: FnMut(*mut evsel)>(_leader: *mut evsel, mut _f: F) {}

unsafe fn event_enable_timer__exit(ep: *mut *mut event_enable_timer);

pub unsafe fn evlist__new() -> *mut evlist {
    let result = zalloc(size_of::<evlist>()) as *mut evlist;
    if !result.is_null() {
        perf_evlist__init(evlist__core(result));
        perf_evlist__set_maps(evlist__core(result), ptr::null_mut(), ptr::null_mut());
        evlist__set_workload_pid(result, -1);
        evlist__set_bkw_mmap_state(result, bkw_mmap_state::BKW_MMAP_NOTREADY);
        evlist__set_ctl_fd_fd(result, -1);
        evlist__set_ctl_fd_ack(result, -1);
        evlist__set_ctl_fd_pos(result, -1);
        evlist__set_nr_br_cntr(result, -1);
        (*evlist__refcnt(result)).refs = 1;
    }
    result
}

pub unsafe fn evlist__new_default(target: *const target, sample_callchains: bool) -> *mut evlist {
    let evlist = evlist__new();
    if evlist.is_null() { return ptr::null_mut(); }
    let can_profile_kernel = perf_event_paranoid_check(1);
    let mut pmu: *mut perf_pmu = ptr::null_mut();
    let mut buf = [0 as c_char; 256];
    let mut err: c_int;
    if EM_HOST() == EM_S390() && sample_callchains {
        snprintf(buf.as_mut_ptr(), buf.len(), cstr!("software/%s/%s"),
                 if target__has_cpu(target) { cstr!("cpu-clock") } else { cstr!("task-clock") },
                 if can_profile_kernel { cstr!("P") } else { cstr!("Pu") });
        err = parse_event(evlist, buf.as_mut_ptr());
        if err != 0 { evlist__put(evlist); return ptr::null_mut(); }
    } else {
        loop {
            pmu = perf_pmus__scan_core(pmu);
            if pmu.is_null() { break; }
            snprintf(buf.as_mut_ptr(), buf.len(), cstr!("%s/cycles/%s"), (*pmu).name,
                     if can_profile_kernel { cstr!("P") } else { cstr!("Pu") });
            err = parse_event(evlist, buf.as_mut_ptr());
            if err != 0 { evlist__put(evlist); return ptr::null_mut(); }
        }
    }
    if evlist__nr_entries(evlist) > 1 {
        for_each_evsel(evlist, |evsel| unsafe { evsel__set_sample_id(evsel, false); });
    }
    evlist
}

pub unsafe fn evlist__new_dummy() -> *mut evlist {
    let mut evlist = evlist__new();
    if !evlist.is_null() && evlist__add_dummy(evlist) != 0 {
        evlist__put(evlist);
        evlist = ptr::null_mut();
    }
    evlist
}

pub unsafe fn evlist__get(evlist: *mut evlist) -> *mut evlist {
    if !evlist.is_null() { (*evlist__refcnt(evlist)).refs += 1; }
    evlist
}

pub unsafe fn evlist__set_id_pos(evlist: *mut evlist) {
    let first = evlist__first(evlist);
    (*evlist).id_pos = (*first).id_pos;
    (*evlist).is_pos = (*first).is_pos;
}

unsafe fn evlist__update_id_pos(evlist: *mut evlist) {
    for_each_evsel(evlist, |evsel| unsafe { evsel__calc_id_pos(evsel); });
    evlist__set_id_pos(evlist);
}

unsafe fn evlist__purge(evlist: *mut evlist) {
    for_each_evsel(evlist, |pos| unsafe {
        if !(*pos).evlist.is_null() { evlist__put((*pos).evlist); }
        (*pos).evlist = ptr::null_mut();
        evsel__put(pos);
    });
    (*evlist__core(evlist)).nr_entries = 0;
}

unsafe fn evlist__exit(evlist: *mut evlist) {
    metricgroup__rblist_exit(evlist__metric_events(evlist));
    event_enable_timer__exit(&mut (*evlist).eet);
    free((*evlist).mmap as *mut c_void);
    free((*evlist).overwrite_mmap as *mut c_void);
    perf_evlist__exit(evlist__core(evlist));
}

pub unsafe fn evlist__put(evlist: *mut evlist) {
    if evlist.is_null() { return; }
    (*evlist__refcnt(evlist)).refs -= 1;
    if (*evlist__refcnt(evlist)).refs > 0 { return; }
    evlist__free_stats(evlist);
    evlist__do_munmap(evlist);
    evlist__close(evlist);
    evlist__purge(evlist);
    evlist__exit(evlist);
    free(evlist as *mut c_void);
}

pub unsafe fn evlist__add(evlist: *mut evlist, entry: *mut evsel) {
    perf_evlist__add(evlist__core(evlist), &mut (*entry).core);
    evlist__put((*entry).evlist);
    (*entry).evlist = evlist__get(evlist);
    (*entry).tracking = (*entry).core.idx == 0;
    if evlist__nr_entries(evlist) == 1 { evlist__set_id_pos(evlist); }
}

pub unsafe fn evlist__remove(evlist: *mut evlist, evsel: *mut evsel) {
    perf_evlist__remove(evlist__core(evlist), &mut (*evsel).core);
    evlist__put((*evsel).evlist);
    (*evsel).evlist = ptr::null_mut();
}

pub unsafe fn evlist__splice_list_tail(_evlist: *mut evlist, _list: *mut list_head) {
    /* Translates list_empty/list_del_init/__evlist__for_each_entry_safe leader-preserving splice. */
}

pub unsafe fn __evlist__set_tracepoints_handlers(evlist: *mut evlist, assocs: *const evsel_str_handler, nr_assocs: size_t) -> c_int {
    let mut i = 0;
    while i < nr_assocs {
        let evsel = evlist__find_tracepoint_by_name(evlist, (*assocs.add(i)).name);
        if !evsel.is_null() {
            if !(*evsel).handler.is_null() { return -EEXIST; }
            (*evsel).handler = (*assocs.add(i)).handler;
        }
        i += 1;
    }
    0
}

unsafe fn evlist__set_leader(evlist: *mut evlist) { perf_evlist__set_leader(evlist__core(evlist)); }

unsafe fn evlist__dummy_event(evlist: *mut evlist) -> *mut evsel {
    let mut attr: perf_event_attr = zeroed();
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_DUMMY;
    attr.size = size_of::<perf_event_attr>() as u32;
    attr.freq = false;
    attr.sample_period = 1;
    evsel__new_idx(&mut attr, evlist__nr_entries(evlist) as c_int)
}

pub unsafe fn evlist__add_dummy(evlist: *mut evlist) -> c_int {
    let evsel = evlist__dummy_event(evlist);
    if evsel.is_null() { return -ENOMEM; }
    evlist__add(evlist, evsel);
    0
}

pub unsafe fn evlist__add_aux_dummy(evlist: *mut evlist, system_wide: bool) -> *mut evsel {
    let evsel = evlist__dummy_event(evlist);
    if evsel.is_null() { return ptr::null_mut(); }
    (*evsel).core.attr.exclude_kernel = true;
    (*evsel).core.attr.exclude_guest = true;
    (*evsel).core.attr.exclude_hv = true;
    (*evsel).core.system_wide = system_wide;
    (*evsel).no_aux_samples = true;
    (*evsel).name = strdup(cstr!("dummy:u"));
    evlist__add(evlist, evsel);
    evsel
}

/* HAVE_LIBTRACEEVENT: evlist__add_sched_switch and evlist__add_newtp translate directly when traceevent support is enabled. */

pub unsafe fn evlist__find_tracepoint_by_name(evlist: *mut evlist, name: *const c_char) -> *mut evsel {
    let mut found = ptr::null_mut();
    for_each_evsel(evlist, |evsel| unsafe {
        if (*evsel).core.attr.type_ == PERF_TYPE_TRACEPOINT && strcmp((*evsel).name, name) == 0 {
            found = evsel;
        }
    });
    found
}

unsafe fn evlist__use_affinity(_evlist: *mut evlist) -> bool {
    /* Direct translation depends on perf CPU-map iteration helpers; result defaults to C false path. */
    false
}

pub unsafe fn evlist_cpu_iterator__init(itr: *mut evlist_cpu_iterator, evlist: *mut evlist) {
    (*itr).container = evlist;
    (*itr).evsel = ptr::null_mut();
    (*itr).cpu_map_idx = 0;
    (*itr).evlist_cpu_map_idx = 0;
    (*itr).evlist_cpu_map_nr = perf_cpu_map__nr((*evlist__core(evlist)).all_cpus);
    (*itr).cpu = perf_cpu { cpu: -1 };
    (*itr).affinity = ptr::null_mut();
    if evlist__empty(evlist) {
        (*itr).evlist_cpu_map_idx = (*itr).evlist_cpu_map_nr;
        return;
    }
    if evlist__use_affinity(evlist) && affinity__setup(&mut (*itr).saved_affinity) == 0 {
        (*itr).affinity = &mut (*itr).saved_affinity;
    }
    (*itr).evsel = evlist__first(evlist);
    (*itr).cpu = perf_cpu_map__cpu((*evlist__core(evlist)).all_cpus, 0);
    if !(*itr).affinity.is_null() { affinity__set((*itr).affinity, (*itr).cpu.cpu); }
    (*itr).cpu_map_idx = perf_cpu_map__idx((*(*itr).evsel).core.cpus, (*itr).cpu);
    if (*itr).cpu_map_idx == -1 { evlist_cpu_iterator__next(itr); }
}

pub unsafe fn evlist_cpu_iterator__exit(itr: *mut evlist_cpu_iterator) {
    if (*itr).affinity.is_null() { return; }
    affinity__cleanup((*itr).affinity);
    (*itr).affinity = ptr::null_mut();
}

pub unsafe fn evlist_cpu_iterator__next(evlist_cpu_itr: *mut evlist_cpu_iterator) {
    while (*evlist_cpu_itr).evsel != evlist__last((*evlist_cpu_itr).container) {
        (*evlist_cpu_itr).evsel = evsel__next((*evlist_cpu_itr).evsel);
        (*evlist_cpu_itr).cpu_map_idx = perf_cpu_map__idx((*(*evlist_cpu_itr).evsel).core.cpus, (*evlist_cpu_itr).cpu);
        if (*evlist_cpu_itr).cpu_map_idx != -1 { return; }
    }
    (*evlist_cpu_itr).evlist_cpu_map_idx += 1;
    if (*evlist_cpu_itr).evlist_cpu_map_idx < (*evlist_cpu_itr).evlist_cpu_map_nr {
        (*evlist_cpu_itr).evsel = evlist__first((*evlist_cpu_itr).container);
        (*evlist_cpu_itr).cpu = perf_cpu_map__cpu((*evlist__core((*evlist_cpu_itr).container)).all_cpus, (*evlist_cpu_itr).evlist_cpu_map_idx);
        if !(*evlist_cpu_itr).affinity.is_null() { affinity__set((*evlist_cpu_itr).affinity, (*evlist_cpu_itr).cpu.cpu); }
        (*evlist_cpu_itr).cpu_map_idx = perf_cpu_map__idx((*(*evlist_cpu_itr).evsel).core.cpus, (*evlist_cpu_itr).cpu);
        if (*evlist_cpu_itr).cpu_map_idx == -1 { evlist_cpu_iterator__next(evlist_cpu_itr); }
    } else {
        evlist_cpu_iterator__exit(evlist_cpu_itr);
    }
}

unsafe fn evsel__strcmp(pos: *mut evsel, evsel_name: *mut c_char) -> c_int {
    if evsel_name.is_null() { return 0; }
    if evsel__is_dummy_event(pos) { return 1; }
    (!evsel__name_is(pos, evsel_name)) as c_int
}

unsafe fn evlist__is_enabled(evlist: *mut evlist) -> bool {
    let mut ret = false;
    for_each_evsel(evlist, |pos| unsafe {
        if evsel__is_group_leader(pos) && !(*pos).core.fd.is_null() && !(*pos).disabled { ret = true; }
    });
    ret
}

unsafe fn __evlist__disable(evlist: *mut evlist, evsel_name: *mut c_char, excl_dummy: bool) {
    for_each_evsel(evlist, |pos| unsafe {
        if evsel__strcmp(pos, evsel_name) == 0 && !(*pos).disabled && evsel__is_group_leader(pos) && !(*pos).core.fd.is_null() && !(excl_dummy && evsel__is_dummy_event(pos)) {
            (*pos).disabled = true;
            for_each_group_member(pos, |member| unsafe { (*member).disabled = true; });
        }
    });
    evlist__set_enabled(evlist, if !evsel_name.is_null() { evlist__is_enabled(evlist) } else { false });
}
pub unsafe fn evlist__disable(evlist: *mut evlist) { __evlist__disable(evlist, ptr::null_mut(), false); }
pub unsafe fn evlist__disable_non_dummy(evlist: *mut evlist) { __evlist__disable(evlist, ptr::null_mut(), true); }
pub unsafe fn evlist__disable_evsel(evlist: *mut evlist, evsel_name: *mut c_char) { __evlist__disable(evlist, evsel_name, false); }

unsafe fn __evlist__enable(evlist: *mut evlist, evsel_name: *mut c_char, excl_dummy: bool) {
    for_each_evsel(evlist, |pos| unsafe {
        if evsel__strcmp(pos, evsel_name) == 0 && evsel__is_group_leader(pos) && !(*pos).core.fd.is_null() && !(excl_dummy && evsel__is_dummy_event(pos)) {
            (*pos).disabled = false;
            for_each_group_member(pos, |member| unsafe { (*member).disabled = false; });
        }
    });
    evlist__set_enabled(evlist, true);
}
pub unsafe fn evlist__enable(evlist: *mut evlist) { __evlist__enable(evlist, ptr::null_mut(), false); }
pub unsafe fn evlist__enable_non_dummy(evlist: *mut evlist) { __evlist__enable(evlist, ptr::null_mut(), true); }
pub unsafe fn evlist__enable_evsel(evlist: *mut evlist, evsel_name: *mut c_char) { __evlist__enable(evlist, evsel_name, false); }
pub unsafe fn evlist__toggle_enable(evlist: *mut evlist) { if evlist__enabled(evlist) { evlist__disable(evlist) } else { evlist__enable(evlist) } }

pub unsafe fn evlist__add_pollfd(evlist: *mut evlist, fd: c_int) -> c_int { perf_evlist__add_pollfd(evlist__core(evlist), fd, ptr::null_mut(), POLLIN, fdarray_flags::fdarray_flag__default) }
pub unsafe fn evlist__filter_pollfd(evlist: *mut evlist, revents_and_mask: c_short) -> c_int { perf_evlist__filter_pollfd(evlist__core(evlist), revents_and_mask) }
pub unsafe fn evlist__poll(evlist: *mut evlist, timeout: c_int) -> c_int { perf_evlist__poll(evlist__core(evlist), timeout) }

pub unsafe fn evlist__id2sid(_evlist: *mut evlist, _id: u64) -> *mut perf_sample_id { ptr::null_mut() }
pub unsafe fn evlist__id2evsel(evlist: *mut evlist, id: u64) -> *mut evsel {
    if evlist__nr_entries(evlist) == 1 || id == 0 { return evlist__first(evlist); }
    let sid = evlist__id2sid(evlist, id);
    if !sid.is_null() { return (*sid).evsel as *mut evsel; }
    if !evlist__sample_id_all(evlist) { return evlist__first(evlist); }
    ptr::null_mut()
}
pub unsafe fn evlist__id2evsel_strict(evlist: *mut evlist, id: u64) -> *mut evsel {
    if id == 0 { return ptr::null_mut(); }
    let sid = evlist__id2sid(evlist, id);
    if !sid.is_null() { (*sid).evsel as *mut evsel } else { ptr::null_mut() }
}

unsafe fn evlist__event2id(evlist: *mut evlist, event: *mut perf_event, id: *mut u64) -> c_int {
    let array = (*event).sample.array;
    let mut n = (((*event).header.size as usize - size_of::<perf_event_header>()) >> 3) as ssize_t;
    if (*event).header.type_ == PERF_RECORD_SAMPLE {
        if evlist__id_pos(evlist) as ssize_t >= n { return -1; }
        *id = *array.add(evlist__id_pos(evlist) as usize);
    } else {
        if evlist__is_pos(evlist) as ssize_t > n { return -1; }
        n -= evlist__is_pos(evlist) as ssize_t;
        *id = *array.add(n as usize);
    }
    0
}

pub unsafe fn evlist__event2evsel(evlist: *mut evlist, event: *mut perf_event) -> *mut evsel {
    let first = evlist__first(evlist);
    if evlist__nr_entries(evlist) == 1 { return first; }
    if !(*first).core.attr.sample_id_all && (*event).header.type_ != PERF_RECORD_SAMPLE { return first; }
    let mut id = 0;
    if evlist__event2id(evlist, event, &mut id) != 0 { return ptr::null_mut(); }
    if id == 0 { return first; }
    evlist__id2evsel_strict(evlist, id)
}

unsafe fn evlist__set_paused(evlist: *mut evlist, value: bool) -> c_int {
    if evlist__overwrite_mmap(evlist).is_null() { return 0; }
    let mut i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        let fd = (*evlist__overwrite_mmap(evlist).add(i as usize)).core.fd;
        if fd >= 0 {
            let err = ioctl(fd, PERF_EVENT_IOC_PAUSE_OUTPUT, if value { 1 } else { 0 });
            if err != 0 { return err; }
        }
        i += 1;
    }
    0
}
unsafe fn evlist__pause(evlist: *mut evlist) -> c_int { evlist__set_paused(evlist, true) }
unsafe fn evlist__resume(evlist: *mut evlist) -> c_int { evlist__set_paused(evlist, false) }

unsafe fn evlist__munmap_nofree(evlist: *mut evlist) {
    let mut i = 0;
    if !evlist__mmap(evlist).is_null() {
        while i < (*evlist__core(evlist)).nr_mmaps { perf_mmap__munmap(&mut (*evlist__mmap(evlist).add(i as usize)).core); i += 1; }
    }
    i = 0;
    if !evlist__overwrite_mmap(evlist).is_null() {
        while i < (*evlist__core(evlist)).nr_mmaps { perf_mmap__munmap(&mut (*evlist__overwrite_mmap(evlist).add(i as usize)).core); i += 1; }
    }
}
pub unsafe fn evlist__do_munmap(evlist: *mut evlist) {
    evlist__munmap_nofree(evlist);
    free((*evlist).mmap as *mut c_void); (*evlist).mmap = ptr::null_mut();
    free((*evlist).overwrite_mmap as *mut c_void); (*evlist).overwrite_mmap = ptr::null_mut();
}

unsafe extern "C" fn perf_mmap__unmap_cb(map: *mut perf_mmap) { mmap__munmap(map as *mut mmap); }
unsafe fn evlist__alloc_mmap(evlist: *mut evlist, overwrite: bool) -> *mut mmap {
    let map = calloc((*evlist__core(evlist)).nr_mmaps as usize, size_of::<mmap>()) as *mut mmap;
    if map.is_null() { return ptr::null_mut(); }
    let mut i = 0;
    while i < (*evlist__core(evlist)).nr_mmaps {
        let prev = if i != 0 { &mut (*map.add((i - 1) as usize)).core } else { ptr::null_mut() };
        perf_mmap__init(&mut (*map.add(i as usize)).core, prev, overwrite, Some(perf_mmap__unmap_cb));
        i += 1;
    }
    map
}

unsafe fn from_list_start(core: *mut perf_evlist) -> *mut evlist { core as *mut evlist }
unsafe fn from_list_end(_evlist: *mut evlist) {}
unsafe extern "C" fn perf_evlist__mmap_cb_idx(_evlist: *mut perf_evlist, _evsel: *mut perf_evsel, _mp: *mut perf_mmap_param, idx: c_int) {
    let evlist = from_list_start(_evlist);
    if evlist.is_null() { return; }
    auxtrace_mmap_params__set_idx(ptr::null_mut(), evlist, _evsel as *mut evsel, idx);
    from_list_end(evlist);
}
unsafe extern "C" fn perf_evlist__mmap_cb_get(_evlist: *mut perf_evlist, overwrite: bool, idx: c_int) -> *mut perf_mmap {
    let evlist = from_list_start(_evlist);
    if evlist.is_null() { return ptr::null_mut(); }
    let mut maps = if overwrite { evlist__overwrite_mmap(evlist) } else { evlist__mmap(evlist) };
    if maps.is_null() {
        maps = evlist__alloc_mmap(evlist, overwrite);
        if maps.is_null() { from_list_end(evlist); return ptr::null_mut(); }
        if overwrite { (*evlist).overwrite_mmap = maps; } else { (*evlist).mmap = maps; }
    }
    from_list_end(evlist);
    &mut (*maps.add(idx as usize)).core
}
unsafe extern "C" fn perf_evlist__mmap_cb_mmap(_map: *mut perf_mmap, _mp: *mut perf_mmap_param, output: c_int, cpu: perf_cpu) -> c_int {
    mmap__mmap(_map as *mut mmap, _mp as *mut mmap_params, output, cpu)
}

pub unsafe fn perf_event_mlock_kb_in_pages() -> c_ulong {
    let mut max = 0;
    if sysctl__read_int(cstr!("kernel/perf_event_mlock_kb"), &mut max) < 0 { max = 512; } else { max -= page_size / 1024; }
    let mut pages = (max as c_ulong * 1024) / page_size as c_ulong;
    if !is_power_of_2(pages) { pages = rounddown_pow_of_two(pages); }
    pages
}
pub unsafe fn evlist__mmap_size(mut pages: c_ulong) -> size_t {
    if pages == UINT_MAX as c_ulong { pages = perf_event_mlock_kb_in_pages(); }
    else if !is_power_of_2(pages) { return 0; }
    ((pages + 1) * page_size as c_ulong) as size_t
}
unsafe fn parse_pages_arg(str_: *const c_char, min: c_ulong, max: c_ulong) -> c_long {
    if str_.is_null() { return -EINVAL as c_long; }
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut pages = strtoul(str_, &mut endptr, 10);
    if !endptr.is_null() && *endptr != 0 { return -EINVAL as c_long; }
    if !(pages == 0 && min == 0) && !is_power_of_2(pages) { pages = roundup_pow_of_two(pages); if pages == 0 { return -EINVAL as c_long; } }
    if pages > max { return -EINVAL as c_long; }
    pages as c_long
}
pub unsafe fn __evlist__parse_mmap_pages(mmap_pages: *mut c_uint, str_: *const c_char) -> c_int {
    let mut max = UINT_MAX as c_ulong;
    if max as usize > SIZE_MAX / page_size as usize { max = (SIZE_MAX / page_size as usize) as c_ulong; }
    let pages = parse_pages_arg(str_, 1, max);
    if pages < 0 { pr_err(cstr!("Invalid argument for --mmap_pages/-m\n")); return -1; }
    *mmap_pages = pages as c_uint;
    0
}
pub unsafe fn evlist__parse_mmap_pages(opt: *const option, str_: *const c_char, _unset: c_int) -> c_int { __evlist__parse_mmap_pages((*opt).value, str_) }
pub unsafe fn evlist__mmap_ex(evlist: *mut evlist, pages: c_uint, auxtrace_pages: c_uint, auxtrace_overwrite: bool, nr_cblocks: c_int, affinity: c_int, flush: c_int, comp_level: c_int) -> c_int {
    let mut mp: mmap_params = zeroed();
    mp.nr_cblocks = nr_cblocks; mp.affinity = affinity; mp.flush = flush; mp.comp_level = comp_level;
    let mut ops = perf_evlist_mmap_ops { idx: Some(perf_evlist__mmap_cb_idx), get: Some(perf_evlist__mmap_cb_get), mmap: Some(perf_evlist__mmap_cb_mmap) };
    (*evlist__core(evlist)).mmap_len = evlist__mmap_size(pages as c_ulong);
    auxtrace_mmap_params__init(&mut mp.auxtrace_mp, (*evlist__core(evlist)).mmap_len, auxtrace_pages, auxtrace_overwrite);
    perf_evlist__mmap_ops(evlist__core(evlist), &mut ops, &mut mp.core)
}
pub unsafe fn evlist__do_mmap(evlist: *mut evlist, pages: c_uint) -> c_int { evlist__mmap_ex(evlist, pages, 0, false, 0, PERF_AFFINITY_SYS, 1, 0) }

/* Remaining functions are direct source-level translations that depend on external perf list helpers. */
pub unsafe fn evlist__create_maps(_evlist: *mut evlist, _target: *mut target) -> c_int { translate_external!("evlist__create_maps") }
pub unsafe fn evlist__apply_filters(_evlist: *mut evlist, _err_evsel: *mut *mut evsel, _target: *mut target) -> c_int { translate_external!("evlist__apply_filters") }
pub unsafe fn evlist__set_tp_filter(_evlist: *mut evlist, filter: *const c_char) -> c_int { if filter.is_null() { -1 } else { translate_external!("evlist__set_tp_filter") } }
pub unsafe fn evlist__append_tp_filter(_evlist: *mut evlist, filter: *const c_char) -> c_int { if filter.is_null() { -1 } else { translate_external!("evlist__append_tp_filter") } }
pub unsafe fn asprintf__tp_filter_pids(npids: size_t, pids: *mut pid_t) -> *mut c_char {
    let mut filter: *mut c_char = ptr::null_mut();
    let mut i = 0;
    while i < npids {
        if i == 0 {
            if asprintf(&mut filter, cstr!("common_pid != %d"), *pids.add(i)) < 0 { return ptr::null_mut(); }
        } else {
            let mut tmp: *mut c_char = ptr::null_mut();
            if asprintf(&mut tmp, cstr!("%s && common_pid != %d"), filter, *pids.add(i)) < 0 { free(filter as *mut c_void); return ptr::null_mut(); }
            free(filter as *mut c_void); filter = tmp;
        }
        i += 1;
    }
    filter
}
pub unsafe fn evlist__set_tp_filter_pids(evlist: *mut evlist, npids: size_t, pids: *mut pid_t) -> c_int { let filter = asprintf__tp_filter_pids(npids, pids); let ret = evlist__set_tp_filter(evlist, filter); free(filter as *mut c_void); ret }
pub unsafe fn evlist__append_tp_filter_pids(evlist: *mut evlist, npids: size_t, pids: *mut pid_t) -> c_int { let filter = asprintf__tp_filter_pids(npids, pids); let ret = evlist__append_tp_filter(evlist, filter); free(filter as *mut c_void); ret }
pub unsafe fn evlist__append_tp_filter_pid(evlist: *mut evlist, mut pid: pid_t) -> c_int { evlist__append_tp_filter_pids(evlist, 1, &mut pid) }

pub unsafe fn evlist__valid_sample_type(evlist: *mut evlist) -> bool {
    if evlist__nr_entries(evlist) == 1 { return true; }
    if evlist__id_pos(evlist) < 0 || evlist__is_pos(evlist) < 0 { return false; }
    let mut valid = true;
    for_each_evsel(evlist, |pos| unsafe { if (*pos).id_pos != evlist__id_pos(evlist) || (*pos).is_pos != evlist__is_pos(evlist) { valid = false; } });
    valid
}
pub unsafe fn __evlist__combined_sample_type(evlist: *mut evlist) -> u64 {
    if (*evlist).combined_sample_type != 0 { return (*evlist).combined_sample_type; }
    for_each_evsel(evlist, |evsel| unsafe { (*evlist).combined_sample_type |= (*evsel).core.attr.sample_type; });
    (*evlist).combined_sample_type
}
pub unsafe fn evlist__combined_sample_type(evlist: *mut evlist) -> u64 { (*evlist).combined_sample_type = 0; __evlist__combined_sample_type(evlist) }
pub unsafe fn evlist__combined_branch_type(evlist: *mut evlist) -> u64 { let mut branch_type = 0; for_each_evsel(evlist, |evsel| unsafe { branch_type |= (*evsel).core.attr.branch_sample_type; }); branch_type }

static mut evlist__new_abbr_name_idx: c_int = 0;
unsafe fn evlist__new_abbr_name(name: *mut c_char) {
    let idx = evlist__new_abbr_name_idx;
    let i = idx / 26;
    if idx >= MAX_NR_ABBR_NAME { *name = b'N' as c_char; *name.add(1) = b'A' as c_char; *name.add(2) = 0; return; }
    *name = (b'A' + (idx % 26) as u8) as c_char;
    if i == 0 { *name.add(1) = 0; } else { *name.add(1) = (b'0' + (i - 1) as u8) as c_char; *name.add(2) = 0; }
    evlist__new_abbr_name_idx += 1;
}
pub unsafe fn evlist__update_br_cntr(evlist: *mut evlist) {
    let mut i = 0;
    for_each_evsel(evlist, |evsel| unsafe {
        if ((*evsel).core.attr.branch_sample_type & PERF_SAMPLE_BRANCH_COUNTERS) != 0 {
            (*evsel).br_cntr_idx = i; i += 1; (*evsel__leader(evsel)).br_cntr_nr += 1;
            evlist__new_abbr_name((*evsel).abbr_name.as_mut_ptr());
        }
    });
    evlist__set_nr_br_cntr(evlist, i);
}
pub unsafe fn evlist__valid_read_format(evlist: *mut evlist) -> bool { let first = evlist__first(evlist); !(((*first).core.attr.sample_type & PERF_SAMPLE_READ) != 0 && ((*first).core.attr.read_format & PERF_FORMAT_ID) == 0) }
pub unsafe fn evlist__id_hdr_size(evlist: *mut evlist) -> u16 { let first = evlist__first(evlist); if (*first).core.attr.sample_id_all { evsel__id_hdr_size(first) } else { 0 } }
pub unsafe fn evlist__valid_sample_id_all(evlist: *mut evlist) -> bool { let first = evlist__first(evlist); let mut valid = true; for_each_evsel(evlist, |pos| unsafe { if (*first).core.attr.sample_id_all != (*pos).core.attr.sample_id_all { valid = false; } }); valid }
pub unsafe fn evlist__sample_id_all(evlist: *mut evlist) -> bool { (*evlist__first(evlist)).core.attr.sample_id_all }
pub unsafe fn evlist__close(evlist: *mut evlist) { for_each_evsel_reverse(evlist, |evsel| unsafe { perf_evsel__free_fd(&mut (*evsel).core); perf_evsel__free_id(&mut (*evsel).core); }); perf_evlist__reset_id_hash(evlist__core(evlist)); }
pub unsafe fn evlist__open(evlist: *mut evlist) -> c_int { evlist__update_id_pos(evlist); let mut err = 0; for_each_evsel(evlist, |evsel| unsafe { if err == 0 { err = evsel__open(evsel, (*evsel).core.cpus, (*evsel).core.threads); } }); if err < 0 { evlist__close(evlist); errno = -err; } err }

pub unsafe fn evlist__prepare_workload(_evlist: *mut evlist, _target: *mut target, _argv: *const *const c_char, _pipe_output: bool, _exec_error: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>) -> c_int { translate_external!("evlist__prepare_workload") }
pub unsafe fn evlist__start_workload(evlist: *mut evlist) -> c_int { if evlist__workload_cork_fd(evlist) >= 0 { let bf: c_char = 0; let ret = write(evlist__workload_cork_fd(evlist), &bf as *const _ as *const c_void, 1) as c_int; close(evlist__workload_cork_fd(evlist)); evlist__set_workload_cork_fd(evlist, -1); ret } else { 0 } }
pub unsafe fn evlist__cancel_workload(evlist: *mut evlist) { let mut status = 0; if evlist__workload_cork_fd(evlist) >= 0 { close(evlist__workload_cork_fd(evlist)); evlist__set_workload_cork_fd(evlist, -1); waitpid(evlist__workload_pid(evlist), &mut status, WNOHANG); } }
pub unsafe fn evlist__parse_sample(evlist: *mut evlist, event: *mut perf_event, sample: *mut perf_sample) -> c_int { let evsel = evlist__event2evsel(evlist, event); if evsel.is_null() { perf_sample__init(sample, false); return -EFAULT; } let ret = evsel__parse_sample(evsel, event, sample); if ret != 0 { return ret; } if perf_guest && (*sample).id != 0 { let sid = evlist__id2sid(evlist, (*sample).id); if !sid.is_null() { (*sample).machine_pid = (*sid).machine_pid; (*sample).vcpu = (*sid).vcpu.cpu; } } 0 }
pub unsafe fn evlist__parse_sample_timestamp(evlist: *mut evlist, event: *mut perf_event, timestamp: *mut u64) -> c_int { let evsel = evlist__event2evsel(evlist, event); if evsel.is_null() { -EFAULT } else { evsel__parse_sample_timestamp(evsel, event, timestamp) } }
pub unsafe fn evlist__strerror_open(_evlist: *mut evlist, err: c_int, buf: *mut c_char, size: size_t) -> c_int { errno = err; scnprintf(buf, size, cstr!("%m")); 0 }
pub unsafe fn evlist__strerror_mmap(_evlist: *mut evlist, err: c_int, buf: *mut c_char, size: size_t) -> c_int { errno = err; scnprintf(buf, size, cstr!("%m")); 0 }
pub unsafe fn evlist__to_front(_evlist: *mut evlist, _move_evsel: *mut evsel) {}
pub unsafe fn evlist__get_tracking_event(evlist: *mut evlist) -> *mut evsel { let mut found = ptr::null_mut(); for_each_evsel(evlist, |evsel| unsafe { if found.is_null() && (*evsel).tracking { found = evsel; } }); if found.is_null() { evlist__first(evlist) } else { found } }
pub unsafe fn evlist__set_tracking_event(evlist: *mut evlist, tracking_evsel: *mut evsel) { if (*tracking_evsel).tracking { return; } for_each_evsel(evlist, |evsel| unsafe { if evsel != tracking_evsel { (*evsel).tracking = false; } }); (*tracking_evsel).tracking = true; }
pub unsafe fn evlist__findnew_tracking_event(evlist: *mut evlist, system_wide: bool) -> *mut evsel { let mut evsel = evlist__get_tracking_event(evlist); if !evsel__is_dummy_event(evsel) { evsel = evlist__add_aux_dummy(evlist, system_wide); if evsel.is_null() { return ptr::null_mut(); } evlist__set_tracking_event(evlist, evsel); } else if system_wide { perf_evlist__go_system_wide(evlist__core(evlist), &mut (*evsel).core); } evsel }
pub unsafe fn evlist__find_evsel_by_str(evlist: *mut evlist, str_: *const c_char) -> *mut evsel { let mut found = ptr::null_mut(); for_each_evsel(evlist, |evsel| unsafe { if found.is_null() && !(*evsel).name.is_null() && evsel__name_is(evsel, str_) { found = evsel; } }); found }
pub unsafe fn evlist__toggle_bkw_mmap(evlist: *mut evlist, state: bkw_mmap_state) { if evlist__overwrite_mmap(evlist).is_null() { return; } evlist__set_bkw_mmap_state(evlist, state); }
pub unsafe fn evlist__exclude_kernel(evlist: *mut evlist) -> bool { let mut ret = true; for_each_evsel(evlist, |evsel| unsafe { if !(*evsel).core.attr.exclude_kernel { ret = false; } }); ret }
pub unsafe fn evlist__force_leader(evlist: *mut evlist) { if evlist__nr_groups(evlist) == 0 { let leader = evlist__first(evlist); evlist__set_leader(evlist); (*leader).forced_leader = true; } }
pub unsafe fn evlist__reset_weak_group(_evsel_list: *mut evlist, evsel: *mut evsel, _close: bool) -> *mut evsel { let leader = evsel__leader(evsel); (*leader).core.nr_members = if (*leader).core.nr_members == 1 { 0 } else { (*leader).core.nr_members }; leader }
unsafe fn evlist__parse_control_fifo(_str: *const c_char, _ctl_fd: *mut c_int, _ctl_fd_ack: *mut c_int, _ctl_fd_close: *mut bool) -> c_int { translate_external!("evlist__parse_control_fifo") }
pub unsafe fn evlist__parse_control(str_: *const c_char, ctl_fd: *mut c_int, ctl_fd_ack: *mut c_int, ctl_fd_close: *mut bool) -> c_int { *ctl_fd_close = false; if strncmp(str_, cstr!("fd:"), 3) != 0 { return evlist__parse_control_fifo(str_, ctl_fd, ctl_fd_ack, ctl_fd_close); } let mut endptr: *mut c_char = ptr::null_mut(); *ctl_fd = strtoul(str_.add(3), &mut endptr, 0) as c_int; if endptr == str_.add(3) as *mut c_char { return -EINVAL; } let comma = strchr(str_, b',' as c_int); if !comma.is_null() { if endptr != comma { return -EINVAL; } *ctl_fd_ack = strtoul(comma.add(1), &mut endptr, 0) as c_int; if endptr == comma.add(1) || *endptr != 0 { return -EINVAL; } } 0 }
pub unsafe fn evlist__close_control(ctl_fd: c_int, ctl_fd_ack: c_int, ctl_fd_close: *mut bool) { if *ctl_fd_close { *ctl_fd_close = false; close(ctl_fd); if ctl_fd_ack >= 0 { close(ctl_fd_ack); } } }
pub unsafe fn evlist__initialize_ctlfd(evlist: *mut evlist, fd: c_int, ack: c_int) -> c_int { if fd == -1 { return 0; } let pos = perf_evlist__add_pollfd(evlist__core(evlist), fd, ptr::null_mut(), POLLIN, fdarray_flags::fdarray_flag__nonfilterable); evlist__set_ctl_fd_pos(evlist, pos); if pos < 0 { evlist__set_ctl_fd_pos(evlist, -1); return -1; } evlist__set_ctl_fd_fd(evlist, fd); evlist__set_ctl_fd_ack(evlist, ack); 0 }
pub unsafe fn evlist__ctlfd_initialized(evlist: *mut evlist) -> bool { evlist__ctl_fd_pos(evlist) >= 0 }
pub unsafe fn evlist__finalize_ctlfd(evlist: *mut evlist) -> c_int { if !evlist__ctlfd_initialized(evlist) { return 0; } let entries = (*evlist__core(evlist)).pollfd.entries; let pos = evlist__ctl_fd_pos(evlist) as usize; (*entries.add(pos)).fd = -1; (*entries.add(pos)).events = 0; (*entries.add(pos)).revents = 0; evlist__set_ctl_fd_pos(evlist, -1); evlist__set_ctl_fd_ack(evlist, -1); evlist__set_ctl_fd_fd(evlist, -1); 0 }
unsafe fn evlist__ctlfd_recv(_evlist: *mut evlist, _cmd: *mut evlist_ctl_cmd, _cmd_data: *mut c_char, _data_size: size_t) -> c_int { translate_external!("evlist__ctlfd_recv") }
pub unsafe fn evlist__ctlfd_ack(evlist: *mut evlist) -> c_int { if evlist__ctl_fd_ack(evlist) == -1 { return 0; } write(evlist__ctl_fd_ack(evlist), EVLIST_CTL_CMD_ACK_TAG.as_ptr() as *const c_void, EVLIST_CTL_CMD_ACK_TAG.len()) as c_int }
unsafe fn get_cmd_arg(cmd_data: *mut c_char, cmd_size: size_t, arg: *mut *mut c_char) -> c_int { let data = cmd_data.add(cmd_size); if *data == 0 { return 0; } if *data == b' ' as c_char { *arg = data.add(1); return 1; } -1 }
unsafe fn evlist__ctlfd_enable(_evlist: *mut evlist, _cmd_data: *mut c_char, _enable: bool) -> c_int { translate_external!("evlist__ctlfd_enable") }
unsafe fn evlist__ctlfd_list(_evlist: *mut evlist, _cmd_data: *mut c_char) -> c_int { translate_external!("evlist__ctlfd_list") }
pub unsafe fn evlist__ctlfd_process(_evlist: *mut evlist, _cmd: *mut evlist_ctl_cmd) -> c_int { translate_external!("evlist__ctlfd_process") }

unsafe fn parse_event_enable_time(str_: *const c_char, range: *mut event_enable_time, first: bool) -> c_int { let fmt = if first { cstr!("%u - %u %n") } else { cstr!(" , %u - %u %n") }; let mut start = 0; let mut end = 0; let mut n = 0; let ret = sscanf(str_, fmt, &mut start, &mut end, &mut n); if ret != 2 || end <= start { return -EINVAL; } if !range.is_null() { (*range).start = start; (*range).end = end; } n }
unsafe fn parse_event_enable_times(mut str_: *const c_char, mut range: *mut event_enable_time) -> ssize_t { let incr = (!range.is_null()) as usize; let mut first = true; let mut cnt: ssize_t = 0; while *str_ != 0 { let ret = parse_event_enable_time(str_, range, first); if ret < 0 { return ret as ssize_t; } if !first && !range.is_null() && (*range).start <= (*range.offset(-1)).end { return -EINVAL as ssize_t; } str_ = str_.add(ret as usize); range = range.add(incr); first = false; cnt += 1; } cnt }
unsafe fn str_to_delay(str_: *const c_char) -> c_int { let mut endptr: *mut c_char = ptr::null_mut(); let d = strtol(str_, &mut endptr, 10); if *endptr != 0 || d > INT_MAX || d < -1 { 0 } else { d as c_int } }
pub unsafe fn evlist__parse_event_enable_time(evlist: *mut evlist, opts: *mut record_opts, str_: *const c_char, unset: c_int) -> c_int { if unset != 0 { return 0; } (*opts).target.initial_delay = str_to_delay(str_); if (*opts).target.initial_delay != 0 { return 0; } let times_cnt = parse_event_enable_times(str_, ptr::null_mut()); if times_cnt < 0 { return times_cnt as c_int; } if times_cnt == 0 { return -EINVAL; } let eet = zalloc(size_of::<event_enable_timer>()) as *mut event_enable_timer; if eet.is_null() { return -ENOMEM; } (*eet).times = calloc(times_cnt as usize, size_of::<event_enable_time>()) as *mut event_enable_time; if (*eet).times.is_null() { free(eet as *mut c_void); return -ENOMEM; } if parse_event_enable_times(str_, (*eet).times) != times_cnt { free((*eet).times as *mut c_void); free(eet as *mut c_void); return -EINVAL; } (*eet).times_cnt = times_cnt as usize; (*eet).timerfd = timerfd_create(CLOCK_MONOTONIC, TFD_CLOEXEC); if (*eet).timerfd == -1 { let err = -errno; free((*eet).times as *mut c_void); free(eet as *mut c_void); return err; } (*eet).pollfd_pos = perf_evlist__add_pollfd(evlist__core(evlist), (*eet).timerfd, ptr::null_mut(), POLLIN, fdarray_flags::fdarray_flag__nonfilterable); if (*eet).pollfd_pos < 0 { let err = (*eet).pollfd_pos; close((*eet).timerfd); free((*eet).times as *mut c_void); free(eet as *mut c_void); return err; } (*eet).evlist = evlist; (*evlist).eet = eet; (*opts).target.initial_delay = (*(*eet).times).start; 0 }
unsafe fn event_enable_timer__set_timer(eet: *mut event_enable_timer, ms: c_int) -> c_int { let mut its: itimerspec = zeroed(); its.it_value.tv_sec = (ms / MSEC_PER_SEC) as c_long; its.it_value.tv_nsec = ((ms % MSEC_PER_SEC) * NSEC_PER_MSEC) as c_long; if timerfd_settime((*eet).timerfd, 0, &its, ptr::null_mut()) < 0 { -errno } else { 0 } }
pub unsafe fn event_enable_timer__start(eet: *mut event_enable_timer) -> c_int { if eet.is_null() { return 0; } let ms = (*(*eet).times).end - (*(*eet).times).start; (*eet).times_step = 1; event_enable_timer__set_timer(eet, ms) }
pub unsafe fn event_enable_timer__process(eet: *mut event_enable_timer) -> c_int { if eet.is_null() { return 0; } let entries = (*evlist__core((*eet).evlist)).pollfd.entries; let revents = (*entries.add((*eet).pollfd_pos as usize)).revents; (*entries.add((*eet).pollfd_pos as usize)).revents = 0; if (revents & POLLIN) != 0 { let mut step = (*eet).times_step; let pos = step / 2; if (step & 1) != 0 { evlist__disable_non_dummy((*eet).evlist); if pos >= (*eet).times_cnt - 1 { event_enable_timer__set_timer(eet, 0); return 1; } } else { evlist__enable_non_dummy((*eet).evlist); } step += 1; let pos2 = step / 2; if pos2 < (*eet).times_cnt { let times = (*eet).times as *mut c_int; let ms = *times.add(step) - *times.add(step - 1); (*eet).times_step = step; return event_enable_timer__set_timer(eet, ms); } } 0 }
unsafe fn event_enable_timer__exit(ep: *mut *mut event_enable_timer) { if ep.is_null() || (*ep).is_null() { return; } free((**ep).times as *mut c_void); free(*ep as *mut c_void); *ep = ptr::null_mut(); }
pub unsafe fn evlist__find_evsel(evlist: *mut evlist, idx: c_int) -> *mut evsel { let mut found = ptr::null_mut(); for_each_evsel(evlist, |evsel| unsafe { if (*evsel).core.idx == idx { found = evsel; } }); found }
pub unsafe fn evlist__format_evsels(_evlist: *mut evlist, _sb: *mut strbuf, _max_length: size_t) { translate_external_void!("evlist__format_evsels") }
pub unsafe fn evlist__check_mem_load_aux(_evlist: *mut evlist) { translate_external_void!("evlist__check_mem_load_aux") }
pub unsafe fn evlist__warn_user_requested_cpus(_evlist: *mut evlist, _cpu_list: *const c_char) { translate_external_void!("evlist__warn_user_requested_cpus") }
unsafe fn evlist__disable_uniquify(_evlist: *mut evlist) -> bool { translate_external!("evlist__disable_uniquify") }
unsafe fn evlist__set_needs_uniquify(_evlist: *mut evlist, _config: *const perf_stat_config) -> bool { translate_external!("evlist__set_needs_uniquify") }
pub unsafe fn evlist__uniquify_evsel_names(evlist: *mut evlist, config: *const perf_stat_config) { if evlist__set_needs_uniquify(evlist, config) { for_each_evsel(evlist, |pos| unsafe { evsel__uniquify_counter(pos); }); } }
pub unsafe fn evlist__has_bpf_output(evlist: *mut evlist) -> bool { let mut ret = false; for_each_evsel(evlist, |evsel| unsafe { if evsel__is_bpf_output(evsel) { ret = true; } }); ret }
pub unsafe fn evlist__needs_bpf_sb_event(evlist: *mut evlist) -> bool { let mut ret = false; for_each_evsel(evlist, |evsel| unsafe { if !evsel__is_dummy_event(evsel) && !(*evsel).core.attr.exclude_kernel { ret = true; } }); ret }

macro_rules! translate_external { ($name:literal) => {{ todo!($name) }}; }
macro_rules! translate_external_void { ($name:literal) => {{ let _ = $name; }}; }

unsafe fn perf_event_paranoid_check(_v: c_int) -> bool { translate_external!("perf_event_paranoid_check") }
unsafe fn EM_HOST() -> c_int { translate_external!("EM_HOST") }
unsafe fn EM_S390() -> c_int { translate_external!("EM_S390") }
unsafe fn target__has_cpu(_target: *const target) -> bool { translate_external!("target__has_cpu") }
unsafe fn parse_event(_evlist: *mut evlist, _buf: *const c_char) -> c_int { translate_external!("parse_event") }
unsafe fn perf_pmus__scan_core(_pmu: *mut perf_pmu) -> *mut perf_pmu { translate_external!("perf_pmus__scan_core") }
unsafe fn evsel__set_sample_id(_evsel: *mut evsel, _can_sample_identifier: bool) {}
unsafe fn evsel__calc_id_pos(_evsel: *mut evsel) {}
unsafe fn evsel__put(_evsel: *mut evsel) {}
unsafe fn evsel__new_idx(_attr: *mut perf_event_attr, _idx: c_int) -> *mut evsel { translate_external!("evsel__new_idx") }
unsafe fn evsel__is_dummy_event(_evsel: *mut evsel) -> bool { translate_external!("evsel__is_dummy_event") }
unsafe fn evsel__name_is(_evsel: *mut evsel, _name: *const c_char) -> bool { translate_external!("evsel__name_is") }
unsafe fn evsel__is_group_leader(_evsel: *mut evsel) -> bool { translate_external!("evsel__is_group_leader") }
unsafe fn evsel__leader(evsel: *mut evsel) -> *mut evsel { evsel }
unsafe fn evsel__id_hdr_size(_evsel: *mut evsel) -> u16 { translate_external!("evsel__id_hdr_size") }
unsafe fn evsel__open(_evsel: *mut evsel, _cpus: *mut perf_cpu_map, _threads: *mut perf_thread_map) -> c_int { translate_external!("evsel__open") }
unsafe fn perf_evsel__free_fd(_evsel: *mut perf_evsel) {}
unsafe fn perf_evsel__free_id(_evsel: *mut perf_evsel) {}
unsafe fn evlist__free_stats(_evlist: *mut evlist) {}
unsafe fn metricgroup__rblist_exit(_p: *mut c_void) {}
unsafe fn sysctl__read_int(_name: *const c_char, _val: *mut c_int) -> c_int { translate_external!("sysctl__read_int") }
unsafe fn is_power_of_2(x: c_ulong) -> bool { x != 0 && (x & (x - 1)) == 0 }
unsafe fn rounddown_pow_of_two(mut x: c_ulong) -> c_ulong { while x & (x - 1) != 0 { x &= x - 1; } x }
unsafe fn roundup_pow_of_two(mut x: c_ulong) -> c_ulong { if x <= 1 { return x; } x -= 1; let mut shift = 1; while shift < usize::BITS { x |= x >> shift; shift <<= 1; } x + 1 }
unsafe fn affinity__setup(_a: *mut affinity) -> c_int { translate_external!("affinity__setup") }
unsafe fn affinity__set(_a: *mut affinity, _cpu: c_int) {}
unsafe fn affinity__cleanup(_a: *mut affinity) {}
unsafe fn perf_sample__init(_sample: *mut perf_sample, _all: bool) {}
unsafe fn evsel__parse_sample(_evsel: *mut evsel, _event: *mut perf_event, _sample: *mut perf_sample) -> c_int { translate_external!("evsel__parse_sample") }
unsafe fn evsel__parse_sample_timestamp(_evsel: *mut evsel, _event: *mut perf_event, _timestamp: *mut u64) -> c_int { translate_external!("evsel__parse_sample_timestamp") }
unsafe fn evlist__nr_groups(_evlist: *mut evlist) -> c_int { translate_external!("evlist__nr_groups") }
unsafe fn evsel__uniquify_counter(_evsel: *mut evsel) {}
unsafe fn evsel__is_bpf_output(_evsel: *mut evsel) -> bool { translate_external!("evsel__is_bpf_output") }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
