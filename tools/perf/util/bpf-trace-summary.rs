// SPDX-License-Identifier: GPL-2.0
//
// Translated from C implementation source:
// perf/util/bpf-trace-summary.c
//
// C include dependencies translated as external declarations/references:
// errno.h, inttypes.h, math.h, stdio.h, stdlib.h
// dwarf-regs.h (for EM_HOST), trace/beauty/syscalltbl.h, util/cgroup.h,
// util/hashmap.h, util/trace.h, util/util.h, bpf/bpf.h, linux/rbtree.h,
// linux/time64.h, tools/libc_compat.h (reallocarray),
// bpf_skel/syscall_summary.h, bpf_skel/syscall_summary.skel.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u64 = u64;

const ENOMEM: c_int = 12;
const NSEC_PER_MSEC: u64 = 1_000_000;

extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn sqrt(x: f64) -> f64;

    fn zalloc(size: size_t) -> *mut c_void;

    fn syscall_summary_bpf__open() -> *mut syscall_summary_bpf;
    fn syscall_summary_bpf__load(obj: *mut syscall_summary_bpf) -> c_int;
    fn syscall_summary_bpf__attach(obj: *mut syscall_summary_bpf) -> c_int;
    fn syscall_summary_bpf__destroy(obj: *mut syscall_summary_bpf);

    fn cgroup_is_v2(subsys: *const c_char) -> c_int;
    fn read_all_cgroups(root: *mut rb_root);
    fn __cgroup__find(root: *mut rb_root, id: u64) -> *mut cgroup;
    fn cgroup__put(cgrp: *mut cgroup);

    fn hashmap__init(
        map: *mut hashmap,
        hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
        equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
        ctx: *mut c_void,
    );
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut *mut syscall_data) -> bool;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: *mut syscall_data) -> c_int;
    fn hashmap__size(map: *const hashmap) -> c_int;
    fn hashmap__clear(map: *mut hashmap);

    fn bpf_map__get_next_key(
        map: *mut bpf_map,
        cur_key: *const c_void,
        next_key: *mut c_void,
        key_sz: size_t,
    ) -> c_int;
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;

    fn syscalltbl__name(arch: c_int, nr: c_int) -> *const c_char;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

const RB_ROOT: rb_root = rb_root {
    rb_node: ptr::null_mut(),
};

unsafe fn RB_EMPTY_ROOT(root: *const rb_root) -> bool {
    (*root).rb_node.is_null()
}

#[repr(C)]
pub struct cgroup {
    pub node: rb_node,
    pub name: *const c_char,
}

#[repr(C)]
pub struct syscall_summary_bpf {
    pub rodata: *mut syscall_summary_bpf_rodata,
    pub bss: *mut syscall_summary_bpf_bss,
    pub maps: syscall_summary_bpf_maps,
}

#[repr(C)]
pub struct syscall_summary_bpf_rodata {
    pub aggr_mode: c_int,
    pub use_cgroup_v2: c_int,
}

#[repr(C)]
pub struct syscall_summary_bpf_bss {
    pub enabled: c_int,
}

#[repr(C)]
pub struct syscall_summary_bpf_maps {
    pub syscall_stats_map: *mut bpf_map,
}

#[repr(C)]
pub enum trace_summary_mode {
    SUMMARY__BY_THREAD,
    SUMMARY__BY_CGROUP,
    SUMMARY__BY_CPU,
}

const SYSCALL_AGGR_THREAD: c_int = 0;
const SYSCALL_AGGR_CPU: c_int = 1;
const SYSCALL_AGGR_CGROUP: c_int = 2;
const EM_HOST: c_int = 0;

#[repr(C)]
pub struct syscall_key {
    pub nr: c_int,
    pub cpu_or_tid: c_long,
    pub cgroup: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_stats {
    pub total_time: u64,
    pub min_time: u64,
    pub max_time: u64,
    pub squared_sum: f64,
    pub count: c_uint,
    pub error: c_uint,
}

static mut skel: *mut syscall_summary_bpf = ptr::null_mut();
static mut cgroups: rb_root = RB_ROOT;

#[no_mangle]
pub unsafe extern "C" fn trace_prepare_bpf_summary(mode: trace_summary_mode) -> c_int {
    skel = syscall_summary_bpf__open();
    if skel.is_null() {
        fprintf(
            stderr,
            c"failed to open syscall summary bpf skeleton\n".as_ptr(),
        );
        return -1;
    }

    if matches!(mode, trace_summary_mode::SUMMARY__BY_THREAD) {
        (*(*skel).rodata).aggr_mode = SYSCALL_AGGR_THREAD;
    } else if matches!(mode, trace_summary_mode::SUMMARY__BY_CGROUP) {
        (*(*skel).rodata).aggr_mode = SYSCALL_AGGR_CGROUP;
    } else {
        (*(*skel).rodata).aggr_mode = SYSCALL_AGGR_CPU;
    }

    if cgroup_is_v2(c"perf_event".as_ptr()) > 0 {
        (*(*skel).rodata).use_cgroup_v2 = 1;
    }

    if syscall_summary_bpf__load(skel) < 0 {
        fprintf(
            stderr,
            c"failed to load syscall summary bpf skeleton\n".as_ptr(),
        );
        return -1;
    }

    if syscall_summary_bpf__attach(skel) < 0 {
        fprintf(
            stderr,
            c"failed to attach syscall summary bpf skeleton\n".as_ptr(),
        );
        return -1;
    }

    if matches!(mode, trace_summary_mode::SUMMARY__BY_CGROUP) {
        read_all_cgroups(&mut cgroups);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn trace_start_bpf_summary() {
    (*(*skel).bss).enabled = 1;
}

#[no_mangle]
pub unsafe extern "C" fn trace_end_bpf_summary() {
    (*(*skel).bss).enabled = 0;
}

#[repr(C)]
pub struct syscall_node {
    pub syscall_nr: c_int,
    pub stats: syscall_stats,
}

unsafe extern "C" fn rel_stddev(stat: *mut syscall_stats) -> f64 {
    let mut variance: f64;
    let average: f64;

    if (*stat).count < 2 {
        return 0.0;
    }

    average = ((*stat).total_time as f64) / ((*stat).count as f64);

    variance = (*stat).squared_sum;
    variance -= (((*stat).total_time * (*stat).total_time) / ((*stat).count as u64)) as f64;
    variance /= ((*stat).count - 1) as f64;

    100.0 * sqrt(variance / ((*stat).count as f64)) / average
}

/*
 * The syscall_data is to maintain syscall stats ordered by total time.
 * It supports different summary modes like per-thread or global.
 *
 * For per-thread stats, it uses two-level data strurcture -
 * syscall_data is keyed by TID and has an array of nodes which
 * represents each syscall for the thread.
 *
 * For global stats, it's still two-level technically but we don't need
 * per-cpu analysis so it's keyed by the syscall number to combine stats
 * from different CPUs.  And syscall_data always has a syscall_node so
 * it can effectively work as flat hierarchy.
 *
 * For per-cgroup stats, it uses two-level data structure like thread
 * syscall_data is keyed by CGROUP and has an array of node which
 * represents each syscall for the cgroup.
 */
#[repr(C)]
pub struct syscall_data {
    pub key: u64, /* tid if AGGR_THREAD, syscall-nr if AGGR_CPU, cgroup if AGGR_CGROUP */
    pub nr_events: c_int,
    pub nr_nodes: c_int,
    pub total_time: u64,
    pub nodes: *mut syscall_node,
}

unsafe extern "C" fn datacmp(a: *const c_void, b: *const c_void) -> c_int {
    let sa = a as *const *const syscall_data;
    let sb = b as *const *const syscall_data;

    if (**sa).total_time > (**sb).total_time {
        -1
    } else {
        1
    }
}

unsafe extern "C" fn nodecmp(a: *const c_void, b: *const c_void) -> c_int {
    let na = a as *const syscall_node;
    let nb = b as *const syscall_node;

    if (*na).stats.total_time > (*nb).stats.total_time {
        -1
    } else {
        1
    }
}

unsafe extern "C" fn sc_node_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn sc_node_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    key1 == key2
}

unsafe fn print_common_stats(data: *mut syscall_data, mut max_summary: c_int, fp: *mut FILE) -> c_int {
    let mut printed = 0;

    if max_summary == 0 || max_summary > (*data).nr_nodes {
        max_summary = (*data).nr_nodes;
    }

    for i in 0..max_summary {
        let node = (*data).nodes.add(i as usize);
        let stat = &mut (*node).stats as *mut syscall_stats;
        let total = ((*stat).total_time as f64) / (NSEC_PER_MSEC as f64);
        let min = ((*stat).min_time as f64) / (NSEC_PER_MSEC as f64);
        let max = ((*stat).max_time as f64) / (NSEC_PER_MSEC as f64);
        let avg = total / ((*stat).count as f64);
        let name: *const c_char;

        /* TODO: support other ABIs */
        name = syscalltbl__name(EM_HOST, (*node).syscall_nr);
        if !name.is_null() {
            printed += fprintf(fp, c"   %-15s".as_ptr(), name);
        } else {
            printed += fprintf(fp, c"   syscall:%-7d".as_ptr(), (*node).syscall_nr);
        }

        printed += fprintf(
            fp,
            c" %8u %6u %9.3f %9.3f %9.3f %9.3f %9.2f%%\n".as_ptr(),
            (*stat).count,
            (*stat).error,
            total,
            min,
            avg,
            max,
            rel_stddev(stat),
        );
    }
    printed
}

unsafe fn update_thread_stats(
    hash: *mut hashmap,
    map_key: *mut syscall_key,
    map_data: *mut syscall_stats,
) -> c_int {
    let mut data: *mut syscall_data = ptr::null_mut();
    let mut nodes: *mut syscall_node;

    if !hashmap__find(hash, (*map_key).cpu_or_tid, &mut data) {
        data = zalloc(size_of::<syscall_data>()) as *mut syscall_data;
        if data.is_null() {
            return -ENOMEM;
        }

        (*data).key = (*map_key).cpu_or_tid as u64;
        if hashmap__add(hash, (*data).key as c_long, data) < 0 {
            free(data as *mut c_void);
            return -ENOMEM;
        }
    }

    /* update thread total stats */
    (*data).nr_events += (*map_data).count as c_int;
    (*data).total_time += (*map_data).total_time;

    nodes = reallocarray(
        (*data).nodes as *mut c_void,
        ((*data).nr_nodes + 1) as size_t,
        size_of::<syscall_node>(),
    ) as *mut syscall_node;
    if nodes.is_null() {
        return -ENOMEM;
    }

    (*data).nodes = nodes;
    nodes = (*data).nodes.add((*data).nr_nodes as usize);
    (*data).nr_nodes += 1;
    (*nodes).syscall_nr = (*map_key).nr;

    /* each thread has an entry for each syscall, just use the stat */
    memcpy(
        &mut (*nodes).stats as *mut syscall_stats as *mut c_void,
        map_data as *const c_void,
        size_of::<syscall_stats>(),
    );
    0
}

unsafe fn print_thread_stat(data: *mut syscall_data, max_summary: c_int, fp: *mut FILE) -> c_int {
    let mut printed = 0;

    qsort(
        (*data).nodes as *mut c_void,
        (*data).nr_nodes as size_t,
        size_of::<syscall_node>(),
        Some(nodecmp),
    );

    printed += fprintf(fp, c" thread (%d), ".as_ptr(), (*data).key as c_int);
    printed += fprintf(fp, c"%d events\n\n".as_ptr(), (*data).nr_events);

    printed += fprintf(fp, c"   syscall            calls  errors  total       min       avg       max       stddev\n".as_ptr());
    printed += fprintf(fp, c"                                     (msec)    (msec)    (msec)    (msec)        (%%)\n".as_ptr());
    printed += fprintf(fp, c"   --------------- --------  ------ -------- --------- --------- ---------     ------\n".as_ptr());

    printed += print_common_stats(data, max_summary, fp);
    printed += fprintf(fp, c"\n\n".as_ptr());

    printed
}

unsafe fn print_thread_stats(
    data: *mut *mut syscall_data,
    nr_data: c_int,
    max_summary: c_int,
    fp: *mut FILE,
) -> c_int {
    let mut printed = 0;

    for i in 0..nr_data {
        printed += print_thread_stat(*data.add(i as usize), max_summary, fp);
    }

    printed
}

unsafe fn update_total_stats(
    hash: *mut hashmap,
    map_key: *mut syscall_key,
    map_data: *mut syscall_stats,
) -> c_int {
    let mut data: *mut syscall_data = ptr::null_mut();
    let stat: *mut syscall_stats;

    if !hashmap__find(hash, (*map_key).nr as c_long, &mut data) {
        data = zalloc(size_of::<syscall_data>()) as *mut syscall_data;
        if data.is_null() {
            return -ENOMEM;
        }

        (*data).nodes = zalloc(size_of::<syscall_node>()) as *mut syscall_node;
        if (*data).nodes.is_null() {
            free(data as *mut c_void);
            return -ENOMEM;
        }

        (*data).nr_nodes = 1;
        (*data).key = (*map_key).nr as u64;
        (*(*data).nodes).syscall_nr = (*data).key as c_int;

        if hashmap__add(hash, (*data).key as c_long, data) < 0 {
            free((*data).nodes as *mut c_void);
            free(data as *mut c_void);
            return -ENOMEM;
        }
    }

    /* update total stats for this syscall */
    (*data).nr_events += (*map_data).count as c_int;
    (*data).total_time += (*map_data).total_time;

    /* This is sum of the same syscall from different CPUs */
    stat = &mut (*(*data).nodes).stats as *mut syscall_stats;

    (*stat).total_time += (*map_data).total_time;
    (*stat).squared_sum += (*map_data).squared_sum;
    (*stat).count += (*map_data).count;
    (*stat).error += (*map_data).error;

    if (*stat).max_time < (*map_data).max_time {
        (*stat).max_time = (*map_data).max_time;
    }
    if (*stat).min_time > (*map_data).min_time || (*stat).min_time == 0 {
        (*stat).min_time = (*map_data).min_time;
    }

    0
}

unsafe fn print_total_stats(
    data: *mut *mut syscall_data,
    nr_data: c_int,
    mut max_summary: c_int,
    fp: *mut FILE,
) -> c_int {
    let mut printed = 0;
    let mut nr_events = 0;

    for i in 0..nr_data {
        nr_events += (**data.add(i as usize)).nr_events;
    }

    printed += fprintf(fp, c" total, %d events\n\n".as_ptr(), nr_events);

    printed += fprintf(fp, c"   syscall            calls  errors  total       min       avg       max       stddev\n".as_ptr());
    printed += fprintf(fp, c"                                     (msec)    (msec)    (msec)    (msec)        (%%)\n".as_ptr());
    printed += fprintf(fp, c"   --------------- --------  ------ -------- --------- --------- ---------     ------\n".as_ptr());

    if max_summary == 0 || max_summary > nr_data {
        max_summary = nr_data;
    }

    for i in 0..max_summary {
        printed += print_common_stats(*data.add(i as usize), max_summary, fp);
    }

    printed += fprintf(fp, c"\n\n".as_ptr());
    printed
}

unsafe fn update_cgroup_stats(
    hash: *mut hashmap,
    map_key: *mut syscall_key,
    map_data: *mut syscall_stats,
) -> c_int {
    let mut data: *mut syscall_data = ptr::null_mut();
    let mut nodes: *mut syscall_node;

    if !hashmap__find(hash, (*map_key).cgroup as c_long, &mut data) {
        data = zalloc(size_of::<syscall_data>()) as *mut syscall_data;
        if data.is_null() {
            return -ENOMEM;
        }

        (*data).key = (*map_key).cgroup;
        if hashmap__add(hash, (*data).key as c_long, data) < 0 {
            free(data as *mut c_void);
            return -ENOMEM;
        }
    }

    /* update thread total stats */
    (*data).nr_events += (*map_data).count as c_int;
    (*data).total_time += (*map_data).total_time;

    nodes = reallocarray(
        (*data).nodes as *mut c_void,
        ((*data).nr_nodes + 1) as size_t,
        size_of::<syscall_node>(),
    ) as *mut syscall_node;
    if nodes.is_null() {
        return -ENOMEM;
    }

    (*data).nodes = nodes;
    nodes = (*data).nodes.add((*data).nr_nodes as usize);
    (*data).nr_nodes += 1;
    (*nodes).syscall_nr = (*map_key).nr;

    /* each thread has an entry for each syscall, just use the stat */
    memcpy(
        &mut (*nodes).stats as *mut syscall_stats as *mut c_void,
        map_data as *const c_void,
        size_of::<syscall_stats>(),
    );
    0
}

unsafe fn print_cgroup_stat(data: *mut syscall_data, max_summary: c_int, fp: *mut FILE) -> c_int {
    let mut printed = 0;
    let cgrp = __cgroup__find(&mut cgroups, (*data).key);

    qsort(
        (*data).nodes as *mut c_void,
        (*data).nr_nodes as size_t,
        size_of::<syscall_node>(),
        Some(nodecmp),
    );

    if !cgrp.is_null() {
        printed += fprintf(fp, c" cgroup %s,".as_ptr(), (*cgrp).name);
    } else {
        printed += fprintf(fp, c" cgroup id:%lu,".as_ptr(), (*data).key as c_ulong);
    }

    printed += fprintf(fp, c" %d events\n\n".as_ptr(), (*data).nr_events);

    printed += fprintf(fp, c"   syscall            calls  errors  total       min       avg       max       stddev\n".as_ptr());
    printed += fprintf(fp, c"                                     (msec)    (msec)    (msec)    (msec)        (%%)\n".as_ptr());
    printed += fprintf(fp, c"   --------------- --------  ------ -------- --------- --------- ---------     ------\n".as_ptr());

    printed += print_common_stats(data, max_summary, fp);
    printed += fprintf(fp, c"\n\n".as_ptr());

    printed
}

unsafe fn print_cgroup_stats(
    data: *mut *mut syscall_data,
    nr_data: c_int,
    max_summary: c_int,
    fp: *mut FILE,
) -> c_int {
    let mut printed = 0;

    for i in 0..nr_data {
        printed += print_cgroup_stat(*data.add(i as usize), max_summary, fp);
    }

    printed
}

#[no_mangle]
pub unsafe extern "C" fn trace_print_bpf_summary(fp: *mut FILE, max_summary: c_int) -> c_int {
    let map = (*skel).maps.syscall_stats_map;
    let mut prev_key: *mut syscall_key;
    let mut key: syscall_key = core::mem::zeroed();
    let mut data: *mut *mut syscall_data = ptr::null_mut();
    let mut schash: hashmap = core::mem::zeroed();
    let mut entry: *mut hashmap_entry;
    let mut nr_data = 0;
    let mut printed = 0;
    let mut i: c_int;
    let mut bkt: size_t;

    hashmap__init(
        &mut schash,
        Some(sc_node_hash),
        Some(sc_node_equal),
        ptr::null_mut(),
    );

    printed = fprintf(fp, c"\n Summary of events:\n\n".as_ptr());

    /* get stats from the bpf map */
    prev_key = ptr::null_mut();
    while bpf_map__get_next_key(
        map,
        prev_key as *const c_void,
        &mut key as *mut syscall_key as *mut c_void,
        size_of::<syscall_key>(),
    ) == 0
    {
        let mut stat: syscall_stats = core::mem::zeroed();

        if bpf_map__lookup_elem(
            map,
            &key as *const syscall_key as *const c_void,
            size_of::<syscall_key>(),
            &mut stat as *mut syscall_stats as *mut c_void,
            size_of::<syscall_stats>(),
            0,
        ) == 0
        {
            match (*(*skel).rodata).aggr_mode {
                SYSCALL_AGGR_THREAD => {
                    update_thread_stats(&mut schash, &mut key, &mut stat);
                }
                SYSCALL_AGGR_CPU => {
                    update_total_stats(&mut schash, &mut key, &mut stat);
                }
                SYSCALL_AGGR_CGROUP => {
                    update_cgroup_stats(&mut schash, &mut key, &mut stat);
                }
                _ => {}
            }
        }

        prev_key = &mut key;
    }

    nr_data = hashmap__size(&schash);
    data = calloc(nr_data as size_t, size_of::<*mut syscall_data>()) as *mut *mut syscall_data;
    if data.is_null() {
        goto_out(&mut schash, printed)
    } else {
        i = 0;
        /*
         * C macro translated from:
         * hashmap__for_each_entry(&schash, entry, bkt)
         *     data[i++] = entry->pvalue;
         *
         * The iterator expansion is provided by util/hashmap.h in the original
         * translation unit and is not locally representable without that macro.
         */
        bkt = 0;
        entry = ptr::null_mut();
        while hashmap_for_each_entry_next(&mut schash, &mut entry, &mut bkt) {
            *data.add(i as usize) = (*entry).pvalue as *mut syscall_data;
            i += 1;
        }

        qsort(
            data as *mut c_void,
            nr_data as size_t,
            size_of::<*mut syscall_data>(),
            Some(datacmp),
        );

        match (*(*skel).rodata).aggr_mode {
            SYSCALL_AGGR_THREAD => {
                printed += print_thread_stats(data, nr_data, max_summary, fp);
            }
            SYSCALL_AGGR_CPU => {
                printed += print_total_stats(data, nr_data, max_summary, fp);
            }
            SYSCALL_AGGR_CGROUP => {
                printed += print_cgroup_stats(data, nr_data, max_summary, fp);
            }
            _ => {}
        }

        i = 0;
        while i < nr_data && !data.is_null() {
            free((**data.add(i as usize)).nodes as *mut c_void);
            free(*data.add(i as usize) as *mut c_void);
            i += 1;
        }
        free(data as *mut c_void);

        hashmap__clear(&mut schash);
        printed
    }
}

unsafe fn goto_out(schash: *mut hashmap, printed: c_int) -> c_int {
    hashmap__clear(schash);
    printed
}

unsafe fn hashmap_for_each_entry_next(
    _hash: *mut hashmap,
    _entry: *mut *mut hashmap_entry,
    _bkt: *mut size_t,
) -> bool {
    /*
     * Placeholder for the original hashmap__for_each_entry macro expansion.
     * This file-local translation preserves the call site and dependency intent;
     * the concrete iterator is supplied by util/hashmap.h in the C source.
     */
    false
}

#[no_mangle]
pub unsafe extern "C" fn trace_cleanup_bpf_summary() {
    if !RB_EMPTY_ROOT(&mut cgroups) {
        let mut cgrp: *mut cgroup;
        let mut tmp: *mut cgroup;

        /*
         * C macro translated from:
         * rbtree_postorder_for_each_entry_safe(cgrp, tmp, &cgroups, node)
         *     cgroup__put(cgrp);
         *
         * The postorder iterator expansion comes from linux/rbtree.h and is not
         * locally representable without that macro.
         */
        cgrp = ptr::null_mut();
        tmp = ptr::null_mut();
        while rbtree_postorder_for_each_entry_safe_next(&mut cgrp, &mut tmp, &mut cgroups) {
            cgroup__put(cgrp);
        }

        cgroups = RB_ROOT;
    }

    syscall_summary_bpf__destroy(skel);
}

unsafe fn rbtree_postorder_for_each_entry_safe_next(
    _cgrp: *mut *mut cgroup,
    _tmp: *mut *mut cgroup,
    _root: *mut rb_root,
) -> bool {
    /*
     * Placeholder for the original rbtree_postorder_for_each_entry_safe macro
     * expansion. The concrete traversal is supplied by linux/rbtree.h in C.
     */
    false
}
