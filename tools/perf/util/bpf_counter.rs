// SPDX-License-Identifier: GPL-2.0

/* Copyright (c) 2019 Facebook */

/*
 * Rust translation of perf/util/bpf_counter.c.
 * C includes are intentionally not executable Rust; the referenced perf,
 * libbpf, BPF skeleton, and kernel ABI symbols are external dependencies.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type u32 = u32;

const ATTR_MAP_SIZE: c_uint = 16;
const UINT_MAX: u32 = u32::MAX;
const PATH_MAX: usize = 4096;
const RLIM_INFINITY: rlim_t = !0;
const RLIMIT_MEMLOCK: c_int = 8;
const BPF_F_TEST_RUN_ON_CPU: u32 = 1 << 0;
const BPF_ANY: u64 = 0;
const BPF_MAP_TYPE_HASH: c_uint = 1;
const F_OK: c_int = 0;
const LOCK_EX: c_int = 2;
const LOCK_UN: c_int = 8;
const EAGAIN: c_int = 11;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
struct bpf_counter {
    skel: *mut c_void,
    list: list_head,
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

type rlim_t = c_ulong;

#[repr(C)]
pub struct bpf_link_info {
    pub id: __u32,
    pub prog_id: __u32,
}

#[repr(C)]
pub struct bpf_map_info {
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub ctx_in: *mut c_void,
    pub ctx_size_in: __u32,
    pub flags: __u32,
    pub cpu: __u32,
    pub retval: __u32,
}

#[repr(C)]
pub struct bpf_func_info {
    pub type_id: __u32,
}

#[repr(C)]
pub struct perf_bpil_info {
    pub btf_id: __u32,
    pub func_info: __u64,
}

#[repr(C)]
pub struct perf_bpil {
    pub info: perf_bpil_info,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
}

#[repr(C)]
pub struct btf;
#[repr(C)]
pub struct bpf_program;
#[repr(C)]
pub struct bpf_object;
#[repr(C)]
pub struct bpf_map;
#[repr(C)]
pub struct bpf_link;
#[repr(C)]
pub struct evsel;
#[repr(C)]
pub struct target;
#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}
#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}
#[repr(C)]
pub struct perf_thread_map;
#[repr(C)]
pub struct perf_event_attr;
#[repr(C)]
pub struct perf_event_attr_map_entry {
    pub link_id: __u32,
    pub diff_map_id: __u32,
}
#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: u64,
    pub enabled: u64,
    pub running: u64,
}
#[repr(C)]
pub struct bperf_filter_value {
    pub index: __u32,
    pub accum: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bperf_filter_type {
    BPERF_FILTER_GLOBAL = 0,
    BPERF_FILTER_CPU = 1,
    BPERF_FILTER_PID = 2,
    BPERF_FILTER_TGID = 3,
}

#[repr(C)]
pub struct bpf_counter_ops {
    pub load: Option<unsafe extern "C" fn(*mut evsel, *mut target) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub install_pe: Option<unsafe extern "C" fn(*mut evsel, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct bpf_prog_profiler_bpf;
#[repr(C)]
pub struct bperf_leader_bpf;
#[repr(C)]
pub struct bperf_follower_bpf;

const PERF_BPIL_FUNC_INFO: c_ulong = 0;
const BPF_PERF_DEFAULT_ATTR_MAP_PATH: *const c_char = b"perf_attr_map\0".as_ptr() as *const c_char;

unsafe extern "C" {
    static mut bperf_cgrp_ops: bpf_counter_ops;
    static mut cgrp_event_expanded: bool;

    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn flock(fd: c_int, operation: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;

    fn bpf_obj_get_info_by_fd(fd: c_int, info: *mut c_void, info_len: *mut __u32) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_link_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_create(map_type: c_uint, name: *const c_char, key_size: usize, value_size: usize,
                      max_entries: c_uint, opts: *const c_void) -> c_int;
    fn bpf_obj_pin(fd: c_int, path: *const c_char) -> c_int;
    fn bpf_obj_get(path: *const c_char) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__set_attach_target(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn libbpf_num_possible_cpus() -> c_int;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn get_bpf_prog_info_linear(fd: c_int, flags: c_ulong) -> *mut perf_bpil;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__type_by_id(btf: *mut btf, id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *mut btf, offset: __u32) -> *const c_char;
    fn btf__free(btf: *mut btf);

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn sysfs__mountpoint() -> *const c_char;

    fn evsel__nr_cpus(evsel: *mut evsel) -> c_int;
    fn evsel__cpus(evsel: *mut evsel) -> *mut c_void;
    fn evsel__open(evsel: *mut evsel, cpus: *mut c_void, threads: *mut perf_thread_map) -> c_int;
    fn evsel__match_bpf_counter_events(name: *const c_char) -> bool;
    fn evlist__workload_pid(evlist: *mut c_void) -> c_int;
    fn perf_counts(counts: *mut c_void, cpu: c_uint, thread: c_uint) -> *mut perf_counts_values;
    fn perf_cpu_map__idx(cpus: *mut c_void, cpu: perf_cpu) -> c_int;
    fn perf_cpu_map__nr(cpus: *mut c_void) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut c_void, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__has_any_cpu_or_is_empty(cpus: *mut c_void) -> bool;
    fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, idx: __u32) -> __u32;
    fn thread_map__new_by_tid(tid: c_int) -> *mut perf_thread_map;
    fn perf_thread_map__put(threads: *mut perf_thread_map);
    fn cpu__max_cpu() -> perf_cpu;

    fn bpf_prog_profiler_bpf__open() -> *mut bpf_prog_profiler_bpf;
    fn bpf_prog_profiler_bpf__load(skel: *mut bpf_prog_profiler_bpf) -> c_int;
    fn bpf_prog_profiler_bpf__attach(skel: *mut bpf_prog_profiler_bpf) -> c_int;
    fn bpf_prog_profiler_bpf__detach(skel: *mut bpf_prog_profiler_bpf);
    fn bpf_prog_profiler_bpf__destroy(skel: *mut bpf_prog_profiler_bpf);
    fn bperf_leader_bpf__open() -> *mut bperf_leader_bpf;
    fn bperf_leader_bpf__load(skel: *mut bperf_leader_bpf) -> c_int;
    fn bperf_leader_bpf__destroy(skel: *mut bperf_leader_bpf);
    fn bperf_follower_bpf__open() -> *mut bperf_follower_bpf;
    fn bperf_follower_bpf__load(skel: *mut bperf_follower_bpf) -> c_int;
    fn bperf_follower_bpf__attach(skel: *mut bperf_follower_bpf) -> c_int;
    fn bperf_follower_bpf__destroy(skel: *mut bperf_follower_bpf);
}

unsafe fn u64_to_ptr(ptr: __u64) -> *mut c_void {
    ptr as c_ulong as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn set_max_rlimit() {
    let rinf = rlimit {
        rlim_cur: RLIM_INFINITY,
        rlim_max: RLIM_INFINITY,
    };

    setrlimit(RLIMIT_MEMLOCK, &rinf);
}

unsafe fn bpf_link_get_id(fd: c_int) -> __u32 {
    let mut link_info: bpf_link_info = core::mem::zeroed();
    link_info.id = 0;
    let mut link_info_len = size_of::<bpf_link_info>() as __u32;

    bpf_obj_get_info_by_fd(fd, &mut link_info as *mut _ as *mut c_void, &mut link_info_len);
    link_info.id
}

unsafe fn bpf_link_get_prog_id(fd: c_int) -> __u32 {
    let mut link_info: bpf_link_info = core::mem::zeroed();
    link_info.id = 0;
    let mut link_info_len = size_of::<bpf_link_info>() as __u32;

    bpf_obj_get_info_by_fd(fd, &mut link_info as *mut _ as *mut c_void, &mut link_info_len);
    link_info.prog_id
}

unsafe fn bpf_map_get_id(fd: c_int) -> __u32 {
    let mut map_info: bpf_map_info = core::mem::zeroed();
    map_info.id = 0;
    let mut map_info_len = size_of::<bpf_map_info>() as __u32;

    bpf_obj_get_info_by_fd(fd, &mut map_info as *mut _ as *mut c_void, &mut map_info_len);
    map_info.id
}

/* trigger the leader program on a cpu */
#[no_mangle]
pub unsafe extern "C" fn bperf_trigger_reading(prog_fd: c_int, cpu: c_int) -> c_int {
    let mut opts = bpf_test_run_opts {
        ctx_in: ptr::null_mut(),
        ctx_size_in: 0,
        flags: BPF_F_TEST_RUN_ON_CPU,
        cpu: cpu as __u32,
        retval: 0,
    };

    bpf_prog_test_run_opts(prog_fd, &mut opts)
}

unsafe fn bpf_counter_alloc() -> *mut bpf_counter {
    let counter = zalloc(size_of::<bpf_counter>()) as *mut bpf_counter;
    if !counter.is_null() {
        (*counter).list.next = &mut (*counter).list;
        (*counter).list.prev = &mut (*counter).list;
    }
    counter
}

unsafe extern "C" fn bpf_program_profiler__destroy(_evsel: *mut evsel) -> c_int {
    /*
     * list_for_each_entry_safe(counter, tmp, &evsel->bpf_counter_list, list) {
     *     list_del_init(&counter->list);
     *     bpf_prog_profiler_bpf__destroy(counter->skel);
     *     free(counter);
     * }
     * assert(list_empty(&evsel->bpf_counter_list));
     */
    0
}

unsafe fn bpf_target_prog_name(tgt_fd: c_int) -> *mut c_char {
    let mut btf: *mut btf = ptr::null_mut();
    let mut name: *mut c_char = ptr::null_mut();

    let info_linear = get_bpf_prog_info_linear(tgt_fd, 1u64.wrapping_shl(PERF_BPIL_FUNC_INFO as u32) as c_ulong);
    if IS_ERR_OR_NULL(info_linear as *const c_void) {
        pr_debug(b"failed to get info_linear for prog FD %d\n\0".as_ptr() as *const c_char, tgt_fd);
        return ptr::null_mut();
    }

    if (*info_linear).info.btf_id == 0 {
        pr_debug(b"prog FD %d doesn't have valid btf\n\0".as_ptr() as *const c_char, tgt_fd);
        goto_out_target_prog_name(btf, info_linear, name);
        return name;
    }

    btf = btf__load_from_kernel_by_id((*info_linear).info.btf_id);
    if libbpf_get_error(btf as *const c_void) != 0 {
        pr_debug(b"failed to load btf for prog FD %d\n\0".as_ptr() as *const c_char, tgt_fd);
        goto_out_target_prog_name(btf, info_linear, name);
        return name;
    }

    let func_info = u64_to_ptr((*info_linear).info.func_info) as *mut bpf_func_info;
    let t = btf__type_by_id(btf, (*func_info.add(0)).type_id);
    if t.is_null() {
        pr_debug(
            b"btf %d doesn't have type %d\n\0".as_ptr() as *const c_char,
            (*info_linear).info.btf_id,
            (*func_info.add(0)).type_id,
        );
        goto_out_target_prog_name(btf, info_linear, name);
        return name;
    }
    name = strdup(btf__name_by_offset(btf, (*t).name_off));

    goto_out_target_prog_name(btf, info_linear, name);
    name
}

unsafe fn goto_out_target_prog_name(btf: *mut btf, info_linear: *mut perf_bpil, _name: *mut c_char) {
    btf__free(btf);
    free(info_linear as *mut c_void);
}

unsafe extern "C" fn bpf_program_profiler_load_one(evsel: *mut evsel, prog_id: u32) -> c_int {
    let mut prog_name: *mut c_char = ptr::null_mut();
    let prog_fd = bpf_prog_get_fd_by_id(prog_id);
    if prog_fd < 0 {
        pr_err(b"Failed to open fd for bpf prog %u\n\0".as_ptr() as *const c_char, prog_id);
        return -1;
    }
    let counter = bpf_counter_alloc();
    if counter.is_null() {
        close(prog_fd);
        return -1;
    }

    let skel = bpf_prog_profiler_bpf__open();
    if skel.is_null() {
        pr_err(b"Failed to open bpf skeleton\n\0".as_ptr() as *const c_char);
        bpf_prog_profiler_bpf__destroy(skel);
        free(prog_name as *mut c_void);
        free(counter as *mut c_void);
        close(prog_fd);
        return -1;
    }

    /* skel->rodata->num_cpu = evsel__nr_cpus(evsel); */
    /* bpf_map__set_max_entries(skel->maps.events, evsel__nr_cpus(evsel)); */
    /* bpf_map__set_max_entries(skel->maps.fentry_readings, 1); */
    /* bpf_map__set_max_entries(skel->maps.accum_readings, 1); */

    prog_name = bpf_target_prog_name(prog_fd);
    if prog_name.is_null() {
        pr_err(
            b"Failed to get program name for bpf prog %u. Does it have BTF?\n\0".as_ptr() as *const c_char,
            prog_id,
        );
        bpf_prog_profiler_bpf__destroy(skel);
        free(prog_name as *mut c_void);
        free(counter as *mut c_void);
        close(prog_fd);
        return -1;
    }

    /*
     * bpf_object__for_each_program(prog, skel->obj) {
     *     err = bpf_program__set_attach_target(prog, prog_fd, prog_name);
     *     if (err) ...
     * }
     */
    set_max_rlimit();
    let err = bpf_prog_profiler_bpf__load(skel);
    if err != 0 {
        pr_err(b"bpf_prog_profiler_bpf__load failed\n\0".as_ptr() as *const c_char);
        bpf_prog_profiler_bpf__destroy(skel);
        free(prog_name as *mut c_void);
        free(counter as *mut c_void);
        close(prog_fd);
        return -1;
    }

    assert!(!skel.is_null());
    (*counter).skel = skel as *mut c_void;
    /* list_add(&counter->list, &evsel->bpf_counter_list); */
    free(prog_name as *mut c_void);
    close(prog_fd);
    0
}

unsafe extern "C" fn bpf_program_profiler__load(evsel: *mut evsel, target: *mut target) -> c_int {
    let mut saveptr: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char = ptr::null_mut();
    /* bpf_str_ = bpf_str = strdup(target->bpf_str); */
    let mut bpf_str_: *mut c_char = ptr::null_mut();
    let mut bpf_str = bpf_str_;
    if bpf_str.is_null() {
        return -1;
    }

    loop {
        let tok = strtok_r(bpf_str, b",\0".as_ptr() as *const c_char, &mut saveptr);
        if tok.is_null() {
            break;
        }
        let prog_id = strtoul(tok, &mut p, 10) as u32;
        if prog_id == 0 || prog_id == UINT_MAX || (!(*p == 0) && *p != b',' as c_char) {
            pr_err(b"Failed to parse bpf prog ids %s\n\0".as_ptr() as *const c_char /*, target->bpf_str */);
            free(bpf_str_ as *mut c_void);
            return -1;
        }

        let ret = bpf_program_profiler_load_one(evsel, prog_id);
        if ret != 0 {
            bpf_program_profiler__destroy(evsel);
            free(bpf_str_ as *mut c_void);
            return -1;
        }
        bpf_str = ptr::null_mut();
    }
    free(bpf_str_ as *mut c_void);
    0
}

unsafe extern "C" fn bpf_program_profiler__enable(evsel: *mut evsel) -> c_int {
    /*
     * list_for_each_entry(counter, &evsel->bpf_counter_list, list) {
     *     assert(counter->skel != NULL);
     *     ret = bpf_prog_profiler_bpf__attach(counter->skel);
     *     if (ret) {
     *         bpf_program_profiler__destroy(evsel);
     *         return ret;
     *     }
     * }
     */
    let _ = evsel;
    0
}

unsafe extern "C" fn bpf_program_profiler__disable(evsel: *mut evsel) -> c_int {
    /*
     * list_for_each_entry(counter, &evsel->bpf_counter_list, list) {
     *     assert(counter->skel != NULL);
     *     bpf_prog_profiler_bpf__detach(counter->skel);
     * }
     */
    let _ = evsel;
    0
}

unsafe extern "C" fn bpf_program_profiler__read(evsel: *mut evsel) -> c_int {
    // BPF_MAP_TYPE_PERCPU_ARRAY uses /sys/devices/system/cpu/possible
    // Sometimes possible > online, like on a Ryzen 3900X that has 24
    // threads but its possible showed 0-31 -acme
    let num_cpu_bpf = libbpf_num_possible_cpus();
    let mut values = vec![core::mem::zeroed::<bpf_perf_event_value>(); num_cpu_bpf as usize];
    let key: __u32 = 0;
    let _ = (evsel, &mut values, key);
    /*
     * if (list_empty(&evsel->bpf_counter_list))
     *     return -EAGAIN;
     * perf_cpu_map__for_each_idx(...) reset counts.
     * list_for_each_entry(...) read skel->maps.accum_readings and aggregate
     * per-BPF-CPU values into perf_counts.
     */
    0
}

unsafe extern "C" fn bpf_program_profiler__install_pe(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int {
    let _ = (evsel, cpu_map_idx, fd);
    /*
     * list_for_each_entry(counter, &evsel->bpf_counter_list, list) {
     *     skel = counter->skel;
     *     ret = bpf_map_update_elem(bpf_map__fd(skel->maps.events), &cpu, &fd, BPF_ANY);
     *     if (ret)
     *         return ret;
     * }
     */
    0
}

static mut bpf_program_profiler_ops: bpf_counter_ops = bpf_counter_ops {
    load: Some(bpf_program_profiler__load),
    enable: Some(bpf_program_profiler__enable),
    disable: Some(bpf_program_profiler__disable),
    read: Some(bpf_program_profiler__read),
    destroy: Some(bpf_program_profiler__destroy),
    install_pe: Some(bpf_program_profiler__install_pe),
};

unsafe fn bperf_attr_map_compatible(attr_map_fd: c_int) -> bool {
    let mut map_info: bpf_map_info = core::mem::zeroed();
    let mut map_info_len = size_of::<bpf_map_info>() as __u32;
    let err = bpf_obj_get_info_by_fd(attr_map_fd, &mut map_info as *mut _ as *mut c_void, &mut map_info_len);

    if err != 0 {
        return false;
    }
    map_info.key_size as usize == size_of::<perf_event_attr>()
        && map_info.value_size as usize == size_of::<perf_event_attr_map_entry>()
}

unsafe fn bperf_lock_attr_map(_target: *mut target) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let map_fd: c_int;
    let mut err: c_int;

    /*
     * if (target->attr_map)
     *     scnprintf(path, PATH_MAX, "%s", target->attr_map);
     * else
     *     scnprintf(path, PATH_MAX, "%s/fs/bpf/%s", sysfs__mountpoint(),
     *               BPF_PERF_DEFAULT_ATTR_MAP_PATH);
     */
    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        b"%s/fs/bpf/%s\0".as_ptr() as *const c_char,
        sysfs__mountpoint(),
        BPF_PERF_DEFAULT_ATTR_MAP_PATH,
    );

    if access(path.as_ptr(), F_OK) != 0 {
        let created = bpf_map_create(
            BPF_MAP_TYPE_HASH,
            ptr::null(),
            size_of::<perf_event_attr>(),
            size_of::<perf_event_attr_map_entry>(),
            ATTR_MAP_SIZE,
            ptr::null(),
        );
        if created < 0 {
            return -1;
        }

        err = bpf_obj_pin(created, path.as_ptr());
        if err != 0 {
            /* someone pinned the map in parallel? */
            close(created);
            let got = bpf_obj_get(path.as_ptr());
            if got < 0 {
                return -1;
            }
            map_fd = got;
        } else {
            map_fd = created;
        }
    } else {
        map_fd = bpf_obj_get(path.as_ptr());
        if map_fd < 0 {
            return -1;
        }
    }

    if !bperf_attr_map_compatible(map_fd) {
        close(map_fd);
        return -1;
    }
    err = flock(map_fd, LOCK_EX);
    if err != 0 {
        close(map_fd);
        return -1;
    }
    map_fd
}

unsafe fn bperf_check_target(
    evsel: *mut evsel,
    target: *mut target,
    filter_type: *mut bperf_filter_type,
    filter_entry_cnt_ptr: *mut __u32,
) -> c_int {
    /*
     * if (evsel->core.leader->nr_members > 1) ...
     * determine filter type based on target:
     * system_wide -> GLOBAL, cpu_list -> CPU, tid -> PID,
     * pid or workload pid -> TGID, otherwise unsupported.
     */
    let _ = (evsel, target);
    *filter_type = bperf_filter_type::BPERF_FILTER_GLOBAL;
    *filter_entry_cnt_ptr = 1;
    0
}

static mut filter_entry_cnt: __u32 = 0;

unsafe fn bperf_reload_leader_program(
    evsel: *mut evsel,
    attr_map_fd: c_int,
    entry: *mut perf_event_attr_map_entry,
) -> c_int {
    let skel = bperf_leader_bpf__open();
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut err: c_int;

    if skel.is_null() {
        pr_err(b"Failed to open leader skeleton\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /* bpf_map__set_max_entries(skel->maps.events, libbpf_num_possible_cpus()); */
    err = bperf_leader_bpf__load(skel);
    if err != 0 {
        pr_err(b"Failed to load leader skeleton\n\0".as_ptr() as *const c_char);
        bperf_leader_bpf__destroy(skel);
        bpf_link__destroy(link);
        return err;
    }

    /* link = bpf_program__attach(skel->progs.on_switch); */
    if IS_ERR(link as *const c_void) {
        pr_err(b"Failed to attach leader program\n\0".as_ptr() as *const c_char);
        err = PTR_ERR(link as *const c_void);
        bperf_leader_bpf__destroy(skel);
        bpf_link__destroy(link);
        return err;
    }

    let link_fd = bpf_link__fd(link);
    /* diff_map_fd = bpf_map__fd(skel->maps.diff_readings); */
    let diff_map_fd = -1;
    (*entry).link_id = bpf_link_get_id(link_fd);
    (*entry).diff_map_id = bpf_map_get_id(diff_map_fd);
    err = bpf_map_update_elem(attr_map_fd, ptr::null(), entry as *const c_void, BPF_ANY);
    assert!(err == 0);

    /* evsel->bperf_leader_link_fd = bpf_link_get_fd_by_id(entry->link_id); */
    let _ = evsel;
    /*
     * save leader_skel for install_pe, which is called within
     * following evsel__open_per_cpu call
     */
    /* evsel->leader_skel = skel; */
    /* assert(!perf_cpu_map__has_any_cpu_or_is_empty(evsel->core.cpus)); */
    /* Always open system wide. */
    let threads = thread_map__new_by_tid(-1);
    /* evsel__open(evsel, evsel->core.cpus, threads); */
    perf_thread_map__put(threads);

    bperf_leader_bpf__destroy(skel);
    bpf_link__destroy(link);
    err
}

unsafe fn bperf_attach_follower_program(
    skel: *mut bperf_follower_bpf,
    filter_type: bperf_filter_type,
    inherit: bool,
) -> c_int {
    let mut err = 0;

    if (filter_type == bperf_filter_type::BPERF_FILTER_PID
        || filter_type == bperf_filter_type::BPERF_FILTER_TGID)
        && inherit
    {
        /* attach all follower bpf progs to enable event inheritance */
        err = bperf_follower_bpf__attach(skel);
    } else {
        /* link = bpf_program__attach(skel->progs.fexit_XXX); */
        let link: *mut bpf_link = ptr::null_mut();
        if IS_ERR(link as *const c_void) {
            err = PTR_ERR(link as *const c_void);
        }
    }

    err
}

unsafe extern "C" fn bperf__load(evsel: *mut evsel, target: *mut target) -> c_int {
    let mut entry = perf_event_attr_map_entry {
        link_id: 0xffffffff,
        diff_map_id: 0xffffffff,
    };
    let mut diff_map_fd = -1;
    let mut filter_type = bperf_filter_type::BPERF_FILTER_GLOBAL;

    if bperf_check_target(evsel, target, &mut filter_type, &mut filter_entry_cnt) != 0 {
        return -1;
    }

    /* evsel->bperf_leader_prog_fd = -1; */
    /* evsel->bperf_leader_link_fd = -1; */

    /*
     * Step 1: hold a fd on the leader program and the bpf_link, if
     * the program is not already gone, reload the program.
     * Use flock() to ensure exclusive access to the perf_event_attr
     * map.
     */
    let attr_map_fd = bperf_lock_attr_map(target);
    if attr_map_fd < 0 {
        pr_err(b"Failed to lock perf_event_attr map\n\0".as_ptr() as *const c_char);
        return -1;
    }

    let mut err = bpf_map_lookup_elem(attr_map_fd, ptr::null(), &mut entry as *mut _ as *mut c_void);
    if err != 0 {
        err = bpf_map_update_elem(attr_map_fd, ptr::null(), &entry as *const _ as *const c_void, BPF_ANY);
        if err != 0 {
            flock(attr_map_fd, LOCK_UN);
            close(attr_map_fd);
            return err;
        }
    }

    /*
     * evsel->bperf_leader_link_fd = bpf_link_get_fd_by_id(entry.link_id);
     * if (evsel->bperf_leader_link_fd < 0 && bperf_reload_leader_program(...))
     */
    if bperf_reload_leader_program(evsel, attr_map_fd, &mut entry) != 0 {
        err = -1;
        flock(attr_map_fd, LOCK_UN);
        close(attr_map_fd);
        return err;
    }
    /*
     * The bpf_link holds reference to the leader program, and the
     * leader program holds reference to the maps. Therefore, if
     * link_id is valid, diff_map_id should also be valid.
     */
    /* evsel->bperf_leader_prog_fd = bpf_prog_get_fd_by_id(bpf_link_get_prog_id(...)); */
    diff_map_fd = bpf_map_get_fd_by_id(entry.diff_map_id);
    assert!(diff_map_fd >= 0);

    /*
     * bperf uses BPF_PROG_TEST_RUN to get accurate reading. Check
     * whether the kernel support it
     */
    err = bperf_trigger_reading(0, 0);
    if err != 0 {
        pr_err(
            b"The kernel does not support test_run for raw_tp BPF programs.\nTherefore, --use-bpf might show inaccurate readings\n\0"
                .as_ptr() as *const c_char,
        );
        close(diff_map_fd);
        flock(attr_map_fd, LOCK_UN);
        close(attr_map_fd);
        return err;
    }

    /* Step 2: load the follower skeleton */
    /* evsel->follower_skel = bperf_follower_bpf__open(); */
    let follower_skel = bperf_follower_bpf__open();
    if follower_skel.is_null() {
        err = -1;
        pr_err(b"Failed to open follower skeleton\n\0".as_ptr() as *const c_char);
        close(diff_map_fd);
        flock(attr_map_fd, LOCK_UN);
        close(attr_map_fd);
        return err;
    }

    /* attach fexit program to the leader program */
    /* bpf_program__set_attach_target(evsel->follower_skel->progs.fexit_XXX,
     *                                evsel->bperf_leader_prog_fd, "on_switch");
     */

    /* connect to leader diff_reading map */
    /* bpf_map__reuse_fd(evsel->follower_skel->maps.diff_readings, diff_map_fd); */

    /* set up reading map */
    /* bpf_map__set_max_entries(evsel->follower_skel->maps.accum_readings, filter_entry_cnt); */
    err = bperf_follower_bpf__load(follower_skel);
    if err != 0 {
        pr_err(b"Failed to load follower skeleton\n\0".as_ptr() as *const c_char);
        bperf_follower_bpf__destroy(follower_skel);
        close(diff_map_fd);
        flock(attr_map_fd, LOCK_UN);
        close(attr_map_fd);
        return err;
    }

    for i in 0..filter_entry_cnt {
        let mut key: __u32;
        let fval = bperf_filter_value { index: i, accum: 0 };

        if filter_type == bperf_filter_type::BPERF_FILTER_PID
            || filter_type == bperf_filter_type::BPERF_FILTER_TGID
        {
            /* key = perf_thread_map__pid(evsel->core.threads, i); */
            key = i;
        } else if filter_type == bperf_filter_type::BPERF_FILTER_CPU {
            /* key = perf_cpu_map__cpu(evsel->core.cpus, i).cpu; */
            key = i;
        } else {
            break;
        }

        /* filter_map_fd = bpf_map__fd(evsel->follower_skel->maps.filter); */
        let filter_map_fd = -1;
        bpf_map_update_elem(filter_map_fd, &key as *const _ as *const c_void, &fval as *const _ as *const c_void, BPF_ANY);
    }

    /* evsel->follower_skel->bss->type = filter_type; */
    /* evsel->follower_skel->bss->inherit = target->inherit; */

    err = bperf_attach_follower_program(follower_skel, filter_type, false);

    if err != 0 {
        /* close evsel leader link/prog fds if they were opened */
    }
    if diff_map_fd >= 0 {
        close(diff_map_fd);
    }

    flock(attr_map_fd, LOCK_UN);
    close(attr_map_fd);

    err
}

unsafe extern "C" fn bperf__install_pe(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int {
    /* struct bperf_leader_bpf *skel = evsel->leader_skel; */
    /* int cpu = perf_cpu_map__cpu(evsel->core.cpus, cpu_map_idx).cpu; */
    let _ = (evsel, cpu_map_idx, fd);
    /* return bpf_map_update_elem(bpf_map__fd(skel->maps.events), &cpu, &fd, BPF_ANY); */
    0
}

/*
 * trigger the leader prog on each cpu, so the accum_reading map could get
 * the latest readings.
 */
unsafe fn bperf_sync_counters(evsel: *mut evsel) -> c_int {
    /*
     * perf_cpu_map__for_each_cpu(cpu, idx, evsel->core.cpus)
     *     bperf_trigger_reading(evsel->bperf_leader_prog_fd, cpu.cpu);
     */
    let _ = evsel;
    0
}

unsafe extern "C" fn bperf__enable(evsel: *mut evsel) -> c_int {
    /* evsel->follower_skel->bss->enabled = 1; */
    let _ = evsel;
    0
}

unsafe extern "C" fn bperf__disable(evsel: *mut evsel) -> c_int {
    /* evsel->follower_skel->bss->enabled = 0; */
    let _ = evsel;
    0
}

unsafe extern "C" fn bperf__read(evsel: *mut evsel) -> c_int {
    /* struct bperf_follower_bpf *skel = evsel->follower_skel; */
    let num_cpu_bpf = cpu__max_cpu().cpu as __u32;
    let mut values = vec![core::mem::zeroed::<bpf_perf_event_value>(); num_cpu_bpf as usize];
    let mut err = 0;

    bperf_sync_counters(evsel);
    /* reading_map_fd = bpf_map__fd(skel->maps.accum_readings); */
    let reading_map_fd = -1;

    for i in 0..filter_entry_cnt {
        let mut cpu: __u32;

        err = bpf_map_lookup_elem(reading_map_fd, &i as *const _ as *const c_void, values.as_mut_ptr() as *mut c_void);
        if err != 0 {
            break;
        }
        /*
         * switch (evsel->follower_skel->bss->type) {
         * case BPERF_FILTER_GLOBAL:
         *     assert(i == 0);
         *     perf_cpu_map__for_each_cpu(entry, j, evsel__cpus(evsel)) {
         *         counts = perf_counts(evsel->counts, j, 0);
         *         counts->val = values[entry.cpu].counter;
         *         counts->ena = values[entry.cpu].enabled;
         *         counts->run = values[entry.cpu].running;
         *     }
         *     break;
         * case BPERF_FILTER_CPU:
         *     cpu = perf_cpu_map__cpu(evsel__cpus(evsel), i).cpu;
         *     assert(cpu >= 0);
         *     counts = perf_counts(evsel->counts, i, 0);
         *     counts->val = values[cpu].counter;
         *     counts->ena = values[cpu].enabled;
         *     counts->run = values[cpu].running;
         *     break;
         * case BPERF_FILTER_PID:
         * case BPERF_FILTER_TGID:
         *     counts = perf_counts(evsel->counts, 0, i);
         *     counts->val = 0;
         *     counts->ena = 0;
         *     counts->run = 0;
         *     for (cpu = 0; cpu < num_cpu_bpf; cpu++) {
         *         counts->val += values[cpu].counter;
         *         counts->ena += values[cpu].enabled;
         *         counts->run += values[cpu].running;
         *     }
         *     break;
         * default:
         *     break;
         * }
         */
        cpu = 0;
        while cpu < num_cpu_bpf {
            cpu += 1;
        }
    }
    err
}

unsafe extern "C" fn bperf__destroy(evsel: *mut evsel) -> c_int {
    /* bperf_follower_bpf__destroy(evsel->follower_skel); */
    /* close(evsel->bperf_leader_prog_fd); */
    /* close(evsel->bperf_leader_link_fd); */
    let _ = evsel;
    0
}

/*
 * bperf: share hardware PMCs with BPF
 *
 * perf uses performance monitoring counters (PMC) to monitor system
 * performance. The PMCs are limited hardware resources. For example,
 * Intel CPUs have 3x fixed PMCs and 4x programmable PMCs per cpu.
 *
 * Modern data center systems use these PMCs in many different ways:
 * system level monitoring, (maybe nested) container level monitoring, per
 * process monitoring, profiling (in sample mode), etc. In some cases,
 * there are more active perf_events than available hardware PMCs. To allow
 * all perf_events to have a chance to run, it is necessary to do expensive
 * time multiplexing of events.
 *
 * On the other hand, many monitoring tools count the common metrics
 * (cycles, instructions). It is a waste to have multiple tools create
 * multiple perf_events of "cycles" and occupy multiple PMCs.
 *
 * bperf tries to reduce such wastes by allowing multiple perf_events of
 * "cycles" or "instructions" (at different scopes) to share PMUs. Instead
 * of having each perf-stat session to read its own perf_events, bperf uses
 * BPF programs to read the perf_events and aggregate readings to BPF maps.
 * Then, the perf-stat session(s) reads the values from these BPF maps.
 *
 *                                ||
 *       shared progs and maps <- || -> per session progs and maps
 *                                ||
 *   ---------------              ||
 *   | perf_events |              ||
 *   ---------------       fexit  ||      -----------------
 *          |             --------||----> | follower prog |
 *       --------------- /        || ---  -----------------
 * cs -> | leader prog |/         ||/        |         |
 *   --> ---------------         /||  --------------  ------------------
 *  /       |         |         / ||  | filter map |  | accum_readings |
 * /  ------------  ------------  ||  --------------  ------------------
 * |  | prev map |  | diff map |  ||                        |
 * |  ------------  ------------  ||                        |
 *  \                             ||                        |
 * = \ ==================================================== | ============
 *    \                                                    /   user space
 *     \                                                  /
 *      \                                                /
 *    BPF_PROG_TEST_RUN                    BPF_MAP_LOOKUP_ELEM
 *        \                                            /
 *         \                                          /
 *          \------  perf-stat ----------------------/
 *
 * The figure above shows the architecture of bperf. Note that the figure
 * is divided into 3 regions: shared progs and maps (top left), per session
 * progs and maps (top right), and user space (bottom).
 *
 * The leader prog is triggered on each context switch (cs). The leader
 * prog reads perf_events and stores the difference (current_reading -
 * previous_reading) to the diff map. For the same metric, e.g. "cycles",
 * multiple perf-stat sessions share the same leader prog.
 *
 * Each perf-stat session creates a follower prog as fexit program to the
 * leader prog. It is possible to attach up to BPF_MAX_TRAMP_PROGS (38)
 * follower progs to the same leader prog. The follower prog checks current
 * task and processor ID to decide whether to add the value from the diff
 * map to its accumulated reading map (accum_readings).
 *
 * Finally, perf-stat user space reads the value from accum_reading map.
 *
 * Besides context switch, it is also necessary to trigger the leader prog
 * before perf-stat reads the value. Otherwise, the accum_reading map may
 * not have the latest reading from the perf_events. This is achieved by
 * triggering the event via sys_bpf(BPF_PROG_TEST_RUN) to each CPU.
 *
 * Comment before the definition of struct perf_event_attr_map_entry
 * describes how different sessions of perf-stat share information about
 * the leader prog.
 */

static mut bperf_ops: bpf_counter_ops = bpf_counter_ops {
    load: Some(bperf__load),
    enable: Some(bperf__enable),
    disable: Some(bperf__disable),
    read: Some(bperf__read),
    destroy: Some(bperf__destroy),
    install_pe: Some(bperf__install_pe),
};

unsafe fn bpf_counter_skip(_evsel: *mut evsel) -> bool {
    /* return evsel->bpf_counter_ops == NULL; */
    false
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__install_pe(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int {
    if bpf_counter_skip(evsel) {
        return 0;
    }
    /* return evsel->bpf_counter_ops->install_pe(evsel, cpu_map_idx, fd); */
    if let Some(install_pe) = bperf_ops.install_pe {
        return install_pe(evsel, cpu_map_idx, fd);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__load(evsel: *mut evsel, target: *mut target) -> c_int {
    /*
     * if (target->bpf_str)
     *     evsel->bpf_counter_ops = &bpf_program_profiler_ops;
     * else if (cgrp_event_expanded && target->use_bpf)
     *     evsel->bpf_counter_ops = &bperf_cgrp_ops;
     * else if (target->use_bpf || evsel->bpf_counter ||
     *          evsel__match_bpf_counter_events(evsel->name))
     *     evsel->bpf_counter_ops = &bperf_ops;
     *
     * if (evsel->bpf_counter_ops)
     *     return evsel->bpf_counter_ops->load(evsel, target);
     */
    if let Some(load) = bperf_ops.load {
        return load(evsel, target);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__enable(evsel: *mut evsel) -> c_int {
    if bpf_counter_skip(evsel) {
        return 0;
    }
    /* return evsel->bpf_counter_ops->enable(evsel); */
    if let Some(enable) = bperf_ops.enable {
        return enable(evsel);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__disable(evsel: *mut evsel) -> c_int {
    if bpf_counter_skip(evsel) {
        return 0;
    }
    /* return evsel->bpf_counter_ops->disable(evsel); */
    if let Some(disable) = bperf_ops.disable {
        return disable(evsel);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__read(evsel: *mut evsel) -> c_int {
    if bpf_counter_skip(evsel) {
        return -EAGAIN;
    }
    /* return evsel->bpf_counter_ops->read(evsel); */
    if let Some(read) = bperf_ops.read {
        return read(evsel);
    }
    -EAGAIN
}

#[no_mangle]
pub unsafe extern "C" fn bpf_counter__destroy(evsel: *mut evsel) {
    if bpf_counter_skip(evsel) {
        return;
    }
    /* evsel->bpf_counter_ops->destroy(evsel); */
    if let Some(destroy) = bperf_ops.destroy {
        destroy(evsel);
    }
    /* evsel->bpf_counter_ops = NULL; */
    /* evsel->bpf_skel = NULL; */
}
