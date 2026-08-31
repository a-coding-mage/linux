// SPDX-License-Identifier: GPL-2.0

/* Copyright (c) 2021 Facebook */
/* Copyright (c) 2021 Google */

// Translated from perf/util/bpf_counter_cgroup.c. C include dependencies are
// represented as external declarations below.

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u32 = u32;
type __u64 = u64;

const PERF_TYPE_SOFTWARE: u32 = 1;
const PERF_COUNT_SW_CGROUP_SWITCHES: u64 = 3;
const BPF_ANY: u64 = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub disabled: u64,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct evsel_core {
    pub fd: *mut c_void,
    pub cpus: *mut perf_cpu_map,
    pub idx: c_int,
}

#[repr(C)]
pub struct evlist_core {
    pub all_cpus: *mut perf_cpu_map,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub cgrp: *mut cgroup,
    pub supported: bool,
    pub evlist: *mut evlist,
    pub bperf_leader_prog_fd: c_int,
    pub bperf_leader_link_fd: c_int,
    pub follower_skel: *mut bperf_follower_bpf,
    pub counts: *mut c_void,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    pub name: *const c_char,
    pub id: __u64,
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bperf_follower_bpf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bperf_cgroup_rodata {
    pub num_cpus: c_int,
    pub num_events: c_int,
    pub use_cgroup_v2: c_int,
}

#[repr(C)]
pub struct bperf_cgroup_bss {
    pub enabled: c_int,
}

#[repr(C)]
pub struct bperf_cgroup_maps {
    pub events: *mut bpf_map,
    pub cgrp_idx: *mut bpf_map,
    pub prev_readings: *mut bpf_map,
    pub cgrp_readings: *mut bpf_map,
}

#[repr(C)]
pub struct bperf_cgroup_progs {
    pub on_cgrp_switch: *mut bpf_program,
    pub trigger_read: *mut bpf_program,
}

#[repr(C)]
pub struct bperf_cgroup_bpf {
    pub rodata: *mut bperf_cgroup_rodata,
    pub bss: *mut bperf_cgroup_bss,
    pub maps: bperf_cgroup_maps,
    pub progs: bperf_cgroup_progs,
}

#[repr(C)]
pub struct bpf_counter_ops {
    pub load: Option<unsafe extern "C" fn(*mut evsel, *mut target) -> c_int>,
    pub enable: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
    pub install_pe: Option<unsafe extern "C" fn(*mut evsel, c_int, c_int) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut evsel) -> c_int>,
}

unsafe extern "C" {
    static mut nr_cgroups: c_int;
    static BPERF_CGROUP__MAX_EVENTS: c_int;

    fn cpu__max_cpu() -> perf_cpu;
    fn cgroup_is_v2(name: *const c_char) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_uint) -> c_int;
    fn bperf_cgroup_bpf__open() -> *mut bperf_cgroup_bpf;
    fn bperf_cgroup_bpf__load(skel: *mut bperf_cgroup_bpf) -> c_int;
    fn bperf_cgroup_bpf__destroy(skel: *mut bperf_cgroup_bpf);
    fn set_max_rlimit();
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evsel__new(attr: *const perf_event_attr) -> *mut evsel;
    fn evsel__open_per_cpu(evsel: *mut evsel, cpus: *mut perf_cpu_map, threads: c_int) -> c_int;
    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn perf_cpu_map__nr(cpus: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, fd: c_int) -> *mut bpf_link;
    fn xyarray__entry(array: *mut c_void, x: c_int, y: c_int) -> *mut c_void;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn read_cgroup_id(cgrp: *mut cgroup) -> c_int;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
    fn evlist__find_evsel(evlist: *mut evlist, idx: c_int) -> *mut evsel;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bperf_trigger_reading(prog_fd: c_int, cpu: c_int) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn perf_counts(counts: *mut c_void, cpu_map_idx: c_int, thread: c_int) -> *mut perf_counts_values;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__put(evsel: *mut evsel);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn BUG_ON(condition: bool);
}

static mut cgrp_switch_attr: perf_event_attr = perf_event_attr {
    type_: PERF_TYPE_SOFTWARE,
    config: PERF_COUNT_SW_CGROUP_SWITCHES,
    size: core::mem::size_of::<perf_event_attr>() as u32,
    sample_period: 1,
    disabled: 1,
};

static mut cgrp_switch: *mut evsel = core::ptr::null_mut();
static mut skel: *mut bperf_cgroup_bpf = core::ptr::null_mut();

unsafe fn FD(evt: *mut evsel, cpu: c_uint) -> c_int {
    *(xyarray__entry((*evt).core.fd, cpu as c_int, 0) as *mut c_int)
}

unsafe extern "C" fn setup_rodata(sk: *mut bperf_cgroup_bpf, evlist_size: c_int) {
    let mut map_size: c_int;
    let total_cpus: c_int = cpu__max_cpu().cpu;

    (*(*sk).rodata).num_cpus = total_cpus;
    (*(*sk).rodata).num_events = evlist_size / nr_cgroups;

    if cgroup_is_v2(c"perf_event".as_ptr()) > 0 {
        (*(*sk).rodata).use_cgroup_v2 = 1;
    }

    BUG_ON(evlist_size % nr_cgroups != 0);

    /* we need one copy of events per cpu for reading */
    map_size = total_cpus * evlist_size / nr_cgroups;
    bpf_map__set_max_entries((*sk).maps.events, map_size as c_uint);
    bpf_map__set_max_entries((*sk).maps.cgrp_idx, nr_cgroups as c_uint);
    /* previous result is saved in a per-cpu array */
    map_size = evlist_size / nr_cgroups;
    bpf_map__set_max_entries((*sk).maps.prev_readings, map_size as c_uint);
    /* cgroup result needs all events (per-cpu) */
    map_size = evlist_size;
    bpf_map__set_max_entries((*sk).maps.cgrp_readings, map_size as c_uint);
}

unsafe extern "C" fn test_max_events_program_load() {
    // C conditional preserved: this body is compiled only when !defined(NDEBUG).
    #[cfg(not(debug_assertions))]
    {
        /*
         * Test that the program verifies with the maximum number of events. If
         * this test fails unfortunately perf needs recompiling with a lower
         * BPERF_CGROUP__MAX_EVENTS to avoid BPF verifier issues.
         */
        let mut err: c_int;
        let max_events: c_int = BPERF_CGROUP__MAX_EVENTS * nr_cgroups;
        let test_skel: *mut bperf_cgroup_bpf = bperf_cgroup_bpf__open();

        if test_skel.is_null() {
            pr_err(c"Failed to open cgroup skeleton\n".as_ptr());
            return;
        }
        setup_rodata(test_skel, max_events);
        err = bperf_cgroup_bpf__load(test_skel);
        if err != 0 {
            pr_err(
                c"Failed to load cgroup skeleton with max events %d.\n".as_ptr(),
                BPERF_CGROUP__MAX_EVENTS,
            );
        }
        bperf_cgroup_bpf__destroy(test_skel);
    }
}

unsafe extern "C" fn bperf_load_program(evlist: *mut evlist) -> c_int {
    let mut link: *mut bpf_link;
    let mut evsel: *mut evsel;
    let mut cgrp: *mut cgroup;
    let mut leader_cgrp: *mut cgroup = core::ptr::null_mut();
    let mut i: c_uint;
    let mut cpu: perf_cpu;
    let total_cpus: c_int = cpu__max_cpu().cpu;
    let mut map_fd: c_int;
    let prog_fd: c_int;
    let mut err: c_int;

    set_max_rlimit();

    if nr_cgroups == 0 || evlist__nr_entries(evlist) % nr_cgroups != 0 {
        pr_err(c"Invalid cgroup or event count\n".as_ptr());
        return -EINVAL;
    }

    test_max_events_program_load();

    skel = bperf_cgroup_bpf__open();
    if skel.is_null() {
        pr_err(c"Failed to open cgroup skeleton\n".as_ptr());
        return -1;
    }
    setup_rodata(skel, evlist__nr_entries(evlist));

    err = bperf_cgroup_bpf__load(skel);
    if err != 0 {
        pr_err(c"Failed to load cgroup skeleton\n".as_ptr());
        return err;
    }

    err = -1;

    cgrp_switch = evsel__new(core::ptr::addr_of!(cgrp_switch_attr));
    if evsel__open_per_cpu(cgrp_switch, (*evlist__core(evlist)).all_cpus, -1) < 0 {
        pr_err(c"Failed to open cgroup switches event\n".as_ptr());
        return err;
    }

    i = 0;
    while (i as c_int) < perf_cpu_map__nr((*evlist__core(evlist)).all_cpus) {
        cpu = perf_cpu_map__cpu((*evlist__core(evlist)).all_cpus, i as c_int);
        link = bpf_program__attach_perf_event((*skel).progs.on_cgrp_switch, FD(cgrp_switch, i));
        if IS_ERR(link as *const c_void) {
            pr_err(c"Failed to attach cgroup program\n".as_ptr());
            err = PTR_ERR(link as *const c_void);
            return err;
        }
        let _ = cpu;
        i += 1;
    }

    /*
     * Update cgrp_idx map from cgroup-id to event index.
     */
    cgrp = core::ptr::null_mut();
    i = 0;

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        if cgrp.is_null() || (*evsel).cgrp == leader_cgrp {
            let mut j: c_uint;

            leader_cgrp = (*evsel).cgrp;
            (*evsel).cgrp = core::ptr::null_mut();

            /* open single copy of the events w/o cgroup */
            err = evsel__open_per_cpu(evsel, (*evsel).core.cpus, -1);
            if err == 0 {
                (*evsel).supported = true;
            }

            map_fd = bpf_map__fd((*skel).maps.events);
            j = 0;
            while (j as c_int) < perf_cpu_map__nr((*evsel).core.cpus) {
                cpu = perf_cpu_map__cpu((*evsel).core.cpus, j as c_int);
                let fd: c_int = FD(evsel, j);
                let idx: __u32 = ((*evsel).core.idx * total_cpus + cpu.cpu) as __u32;

                bpf_map_update_elem(
                    map_fd,
                    core::ptr::addr_of!(idx) as *const c_void,
                    core::ptr::addr_of!(fd) as *const c_void,
                    BPF_ANY,
                );
                j += 1;
            }

            (*evsel).cgrp = leader_cgrp;
        }

        if (*evsel).cgrp == cgrp {
            evsel = evlist__next(evlist, evsel);
            continue;
        }

        cgrp = (*evsel).cgrp;

        if read_cgroup_id(cgrp) < 0 {
            pr_debug(c"Failed to get cgroup id for %s\n".as_ptr(), (*cgrp).name);
            (*cgrp).id = 0;
        }

        map_fd = bpf_map__fd((*skel).maps.cgrp_idx);
        err = bpf_map_update_elem(
            map_fd,
            core::ptr::addr_of!((*cgrp).id) as *const c_void,
            core::ptr::addr_of!(i) as *const c_void,
            BPF_ANY,
        );
        if err < 0 {
            pr_err(c"Failed to update cgroup index map\n".as_ptr());
            return err;
        }

        i += 1;
        evsel = evlist__next(evlist, evsel);
    }

    /*
     * Propagate supported flag from leaders to followers. Follower events
     * are not opened, so their supported flag remains false.
     */
    {
        let mut leader: *mut evsel;
        let num_events: c_int = evlist__nr_entries(evlist) / nr_cgroups;

        evsel = evlist__first(evlist);
        while !evsel.is_null() {
            leader = evlist__find_evsel(evlist, (*evsel).core.idx % num_events);
            if !leader.is_null() {
                (*evsel).supported = (*leader).supported;
            }
            evsel = evlist__next(evlist, evsel);
        }
    }

    /*
     * bperf uses BPF_PROG_TEST_RUN to get accurate reading. Check
     * whether the kernel support it
     */
    prog_fd = bpf_program__fd((*skel).progs.trigger_read);
    err = bperf_trigger_reading(prog_fd, 0);
    if err != 0 {
        pr_warning(
            c"The kernel does not support test_run for raw_tp BPF programs.\nTherefore, --for-each-cgroup might show inaccurate readings\n"
                .as_ptr(),
        );
        err = 0;
    }

    err
}

unsafe extern "C" fn bperf_cgrp__load(evsel: *mut evsel, _target: *mut target) -> c_int {
    static mut bperf_loaded: bool = false;

    (*evsel).bperf_leader_prog_fd = -1;
    (*evsel).bperf_leader_link_fd = -1;

    if !bperf_loaded && bperf_load_program((*evsel).evlist) != 0 {
        return -1;
    }

    bperf_loaded = true;
    /* just to bypass bpf_counter_skip() */
    (*evsel).follower_skel = skel as *mut bperf_follower_bpf;

    0
}

unsafe extern "C" fn bperf_cgrp__install_pe(
    _evsel: *mut evsel,
    _cpu_map_idx: c_int,
    _fd: c_int,
) -> c_int {
    /* nothing to do */
    0
}

/*
 * trigger the leader prog on each cpu, so the cgrp_reading map could get
 * the latest results.
 */
unsafe extern "C" fn bperf_cgrp__sync_counters(evlist: *mut evlist) -> c_int {
    let mut cpu: perf_cpu;
    let mut idx: c_uint;
    let prog_fd: c_int = bpf_program__fd((*skel).progs.trigger_read);

    idx = 0;
    while (idx as c_int) < perf_cpu_map__nr((*evlist__core(evlist)).all_cpus) {
        cpu = perf_cpu_map__cpu((*evlist__core(evlist)).all_cpus, idx as c_int);
        bperf_trigger_reading(prog_fd, cpu.cpu);
        idx += 1;
    }

    0
}

unsafe extern "C" fn bperf_cgrp__enable(evsel: *mut evsel) -> c_int {
    if (*evsel).core.idx != 0 {
        return 0;
    }

    bperf_cgrp__sync_counters((*evsel).evlist);

    (*(*skel).bss).enabled = 1;
    0
}

unsafe extern "C" fn bperf_cgrp__disable(evsel: *mut evsel) -> c_int {
    if (*evsel).core.idx != 0 {
        return 0;
    }

    bperf_cgrp__sync_counters((*evsel).evlist);

    (*(*skel).bss).enabled = 0;
    0
}

unsafe extern "C" fn bperf_cgrp__read(mut evsel: *mut evsel) -> c_int {
    let evlist: *mut evlist = (*evsel).evlist;
    let total_cpus: c_int = cpu__max_cpu().cpu;
    let mut counts: *mut perf_counts_values;
    let values: *mut bpf_perf_event_value;
    let reading_map_fd: c_int;
    let mut err: c_int = 0;

    if (*evsel).core.idx != 0 {
        return 0;
    }

    bperf_cgrp__sync_counters((*evsel).evlist);

    values = calloc(
        total_cpus as usize,
        core::mem::size_of::<bpf_perf_event_value>(),
    ) as *mut bpf_perf_event_value;
    if values.is_null() {
        return -ENOMEM;
    }

    reading_map_fd = bpf_map__fd((*skel).maps.cgrp_readings);

    evsel = evlist__first(evlist);
    while !evsel.is_null() {
        let idx: __u32 = (*evsel).core.idx as __u32;
        let mut i: c_uint;
        let mut cpu: perf_cpu;

        err = bpf_map_lookup_elem(
            reading_map_fd,
            core::ptr::addr_of!(idx) as *const c_void,
            values as *mut c_void,
        );
        if err != 0 {
            pr_err(
                c"bpf map lookup failed: idx=%u, event=%s, cgrp=%s\n".as_ptr(),
                idx,
                evsel__name(evsel),
                (*(*evsel).cgrp).name,
            );
            break;
        }

        i = 0;
        while (i as c_int) < perf_cpu_map__nr((*evsel).core.cpus) {
            cpu = perf_cpu_map__cpu((*evsel).core.cpus, i as c_int);
            counts = perf_counts((*evsel).counts, i as c_int, 0);
            (*counts).val = (*values.add(cpu.cpu as usize)).counter;
            (*counts).ena = (*values.add(cpu.cpu as usize)).enabled;
            (*counts).run = (*values.add(cpu.cpu as usize)).running;
            i += 1;
        }

        evsel = evlist__next(evlist, evsel);
    }

    free(values as *mut c_void);
    err
}

unsafe extern "C" fn bperf_cgrp__destroy(evsel: *mut evsel) -> c_int {
    if (*evsel).core.idx != 0 {
        return 0;
    }

    bperf_cgroup_bpf__destroy(skel);
    evsel__put(cgrp_switch); // it'll destroy on_switch progs too

    0
}

#[no_mangle]
pub static mut bperf_cgrp_ops: bpf_counter_ops = bpf_counter_ops {
    load: Some(bperf_cgrp__load),
    enable: Some(bperf_cgrp__enable),
    disable: Some(bperf_cgrp__disable),
    read: Some(bperf_cgrp__read),
    install_pe: Some(bperf_cgrp__install_pe),
    destroy: Some(bperf_cgrp__destroy),
};
