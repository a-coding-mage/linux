// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/bpf_lock_contention.c.
// C include dependencies are represented below as external declarations and
// opaque C-compatible types; their definitions are supplied by other files.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type s32 = i32;
type s64 = i64;
type u8 = u8;
type u32 = u32;
type u64 = u64;

const BTF_KIND_STRUCT: u32 = 4;
const BPF_ANY: u64 = 0;
const BPF_EXIST: u64 = 2;
const BPF_F_TEST_RUN_ON_CPU: u32 = 1;
const KSYM_NAME_LEN: usize = 256;
const LCD_F_MMAP_LOCK: u32 = 1 << 0;
const LCD_F_SIGHAND_LOCK: u32 = 1 << 1;
const LCB_F_SLAB_ID_MASK: u32 = 0xffff;
const LOCK_CLASS_RQLOCK: u32 = 1;
const LOCK_CLASS_ZONE_LOCK: u32 = 2;

#[repr(C)]
pub struct lock_contention_bpf {
    pub maps: lock_contention_bpf_maps,
    pub progs: lock_contention_bpf_progs,
    pub links: lock_contention_bpf_links,
    pub rodata: *mut lock_contention_rodata,
    pub bss: *mut lock_contention_bss,
}

#[repr(C)]
pub struct lock_contention_bpf_maps {
    pub stacks: *mut bpf_map,
    pub lock_stat: *mut bpf_map,
    pub tstamp: *mut bpf_map,
    pub task_data: *mut bpf_map,
    pub stack_buf: *mut bpf_map,
    pub owner_stacks: *mut bpf_map,
    pub owner_data: *mut bpf_map,
    pub owner_stat: *mut bpf_map,
    pub cpu_filter: *mut bpf_map,
    pub task_filter: *mut bpf_map,
    pub type_filter: *mut bpf_map,
    pub addr_filter: *mut bpf_map,
    pub cgroup_filter: *mut bpf_map,
    pub slab_filter: *mut bpf_map,
    pub slab_caches: *mut bpf_map,
    pub lock_delays: *mut bpf_map,
    pub lock_syms: *mut bpf_map,
    pub tstamp_cpu: *mut bpf_map,
}

#[repr(C)]
pub struct lock_contention_bpf_progs {
    pub slab_cache_iter: *mut bpf_program,
    pub collect_lock_syms: *mut bpf_program,
    pub end_timestamp: *mut bpf_program,
}

#[repr(C)]
pub struct lock_contention_bpf_links {
    pub slab_cache_iter: *mut bpf_link,
}

#[repr(C)]
pub struct lock_contention_rodata {
    pub sizeof_zone: c_int,
    pub contig_page_data_addr: u64,
    pub node_data_addr: u64,
    pub nr_nodes: c_long,
    pub max_stack: c_int,
    pub has_cpu: c_int,
    pub has_task: c_int,
    pub has_type: c_int,
    pub has_cgroup: c_int,
    pub has_addr: c_int,
    pub lock_delay: c_int,
    pub stack_skip: c_int,
    pub aggr_mode: lock_aggr_mode,
    pub needs_callstack: bool_,
    pub lock_owner: bool_,
    pub has_mmap_lock: bool_,
    pub use_cgroup_v2: c_int,
    pub has_slab: c_int,
}

#[repr(C)]
pub struct lock_contention_bss {
    pub end_ts: u64,
    pub enabled: c_int,
    pub task_fail: u64,
    pub stack_fail: u64,
    pub time_fail: u64,
    pub data_fail: u64,
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub pkey: *mut c_void,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct lock_contention {
    pub btf: *mut btf,
    pub map_nr_entries: c_int,
    pub machine: *mut machine,
    pub evlist: *mut evlist,
    pub target: *mut target,
    pub max_stack: c_int,
    pub aggr_mode: lock_aggr_mode,
    pub save_callstack: bool_,
    pub owner: bool_,
    pub filters: *mut lock_filter,
    pub nr_delays: c_int,
    pub delays: *mut lock_delay,
    pub stack_skip: c_int,
    pub cgroups: rb_root,
    pub fails: lock_contention_fails,
    pub nr_filtered: u64,
}

#[repr(C)]
pub struct lock_filter {
    pub nr_types: c_int,
    pub types: *mut u32,
    pub nr_cgrps: c_int,
    pub cgrps: *mut u64,
    pub nr_syms: c_int,
    pub syms: *mut *const c_char,
    pub nr_addrs: c_int,
    pub addrs: *mut c_ulong,
    pub nr_slabs: c_int,
    pub slabs: *mut *const c_char,
}

#[repr(C)]
pub struct lock_delay {
    pub sym: *const c_char,
    pub addr: c_ulong,
    pub time: u64,
}

#[repr(C)]
pub struct lock_contention_fails {
    pub task: u64,
    pub stack: u64,
    pub time: u64,
    pub data: u64,
}

#[repr(C)]
pub struct contention_key {
    pub stack_id: s32,
    pub pid: c_int,
    pub lock_addr_or_cgroup: u64,
}

#[repr(C)]
pub struct contention_data {
    pub total_time: u64,
    pub count: u64,
    pub max_time: u64,
    pub min_time: u64,
    pub flags: u32,
}

#[repr(C)]
pub struct contention_task_data {
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct tstamp_data {
    pub timestamp: u64,
    pub stack_id: s32,
    pub lock: u64,
    pub cgroup_id: u64,
    pub flags: c_int,
}

#[repr(C)]
pub struct slab_cache_data {
    pub id: c_long,
    pub name: [c_char; KSYM_NAME_LEN],
}

#[repr(C)]
pub struct lock_stat {
    pub name: *mut c_char,
    pub flags: u32,
    pub nr_contended: u64,
    pub wait_time_total: u64,
    pub wait_time_max: u64,
    pub wait_time_min: u64,
    pub avg_wait_time: u64,
    pub callstack: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lock_aggr_mode {
    LOCK_AGGR_CALLER = 0,
    LOCK_AGGR_TASK = 1,
    LOCK_AGGR_ADDR = 2,
    LOCK_AGGR_CGROUP = 3,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub flags: u32,
}

#[repr(C)]
pub struct bpf_map {
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
pub struct btf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub name: *const c_char,
}
#[repr(C)]
pub struct map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}
#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}
#[repr(C)]
pub struct target {
    _private: [u8; 0],
}
#[repr(C)]
pub struct evlist_core {
    pub user_requested_cpus: *mut perf_cpu_map,
    pub threads: *mut perf_thread_map,
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
pub struct thread {
    _private: [u8; 0],
}
#[repr(C)]
pub struct cgroup {
    pub node: rb_node,
    pub name: *const c_char,
}

unsafe extern "C" {
    static mut verbose: c_int;

    fn hashmap__init(
        map: *mut hashmap,
        hash_fn: unsafe extern "C" fn(c_long, *mut c_void) -> size_t,
        equal_fn: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool_,
        ctx: *mut c_void,
    );
    fn hashmap__add(map: *mut hashmap, key: c_long, value: *mut c_void) -> c_int;
    fn hashmap__find(
        map: *mut hashmap,
        key: c_long,
        value: *mut *mut slab_cache_data,
    ) -> bool_;
    fn hashmap__clear(map: *mut hashmap);
    fn hashmap__for_each_entry_next(
        map: *mut hashmap,
        cur: *mut *mut hashmap_entry,
        bkt: *mut c_uint,
    ) -> bool_;

    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: u32) -> c_int;
    fn btf__resolve_size(btf: *mut btf, id: c_int) -> c_int;
    fn btf__free(btf: *mut btf);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool_);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, entries: c_int) -> c_int;
    fn bpf_map__set_value_size(map: *mut bpf_map, size: c_int) -> c_int;
    fn bpf_map__set_key_size(map: *mut bpf_map, size: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn lock_contention_bpf__open() -> *mut lock_contention_bpf;
    fn lock_contention_bpf__load(skel: *mut lock_contention_bpf) -> c_int;
    fn lock_contention_bpf__attach(skel: *mut lock_contention_bpf) -> c_int;
    fn lock_contention_bpf__destroy(skel: *mut lock_contention_bpf);

    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__find_kernel_symbol_by_name(
        machine: *mut machine,
        name: *const c_char,
        mapp: *mut *mut map,
    ) -> *mut symbol;
    fn machine__find_kernel_symbol(machine: *mut machine, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    fn machine__is_lock_function(machine: *mut machine, addr: u64) -> bool_;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn map__load(map: *mut map) -> c_int;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__put(map: *mut map);

    fn evlist__core(evlist: *mut evlist) -> *mut evlist_core;
    fn evlist__workload_pid(evlist: *mut evlist) -> c_int;
    fn target__has_cpu(target: *mut target) -> bool_;
    fn target__has_task(target: *mut target) -> bool_;
    fn target__none(target: *mut target) -> bool_;
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_thread_map__nr(map: *mut perf_thread_map) -> c_int;
    fn perf_thread_map__pid(map: *mut perf_thread_map, idx: c_int) -> c_int;
    fn cpu__max_cpu() -> perf_cpu;

    fn cgroup_is_v2(name: *const c_char) -> bool_;
    fn read_all_cgroups(root: *mut rb_root);
    fn __cgroup__find(root: *mut rb_root, id: u64) -> *mut cgroup;
    fn cgroup__put(cgrp: *mut cgroup);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);

    fn sysfs__read_str(path: *const c_char, buf: *mut *mut c_char, len: *mut size_t) -> c_int;
    fn thread__set_comm(thread: *mut thread, comm: *const c_char, timestamp: u64) -> bool_;
    fn match_callstack_filter(machine: *mut machine, stack_trace: *mut u64, max_stack: c_int) -> bool_;
    fn lock_stat_find(key: s64) -> *mut lock_stat;
    fn lock_stat_findnew(key: s64, name: *const c_char, flags: u32) -> *mut lock_stat;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn memdup(src: *const c_void, len: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn close(fd: c_int) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
}

type c_uint = u32;

static mut skel: *mut lock_contention_bpf = ptr::null_mut();
static mut has_slab_iter: bool = false;
static mut slab_hash: hashmap = hashmap { _private: [] };

unsafe extern "C" fn slab_cache_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn slab_cache_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool_ {
    key1 == key2
}

unsafe fn check_slab_cache_iter(con: *mut lock_contention) {
    let ret: s32;

    hashmap__init(&mut slab_hash, slab_cache_hash, slab_cache_equal, ptr::null_mut());

    (*con).btf = btf__load_vmlinux_btf();
    if (*con).btf.is_null() {
        pr_debug(c"BTF loading failed: %m\n".as_ptr());
        return;
    }

    ret = btf__find_by_name_kind((*con).btf, c"bpf_iter__kmem_cache".as_ptr(), BTF_KIND_STRUCT);
    if ret < 0 {
        bpf_program__set_autoload((*skel).progs.slab_cache_iter, false);
        pr_debug(c"slab cache iterator is not available: %d\n".as_ptr(), ret);
        return;
    }

    has_slab_iter = true;

    bpf_map__set_max_entries((*skel).maps.slab_caches, (*con).map_nr_entries);
}

unsafe fn run_slab_cache_iter() {
    let mut fd: c_int;
    let mut buf = [0 as c_char; 256];
    let mut key: c_long = 0;
    let mut prev_key: *mut c_long;

    if !has_slab_iter {
        return;
    }

    fd = bpf_iter_create(bpf_link__fd((*skel).links.slab_cache_iter));
    if fd < 0 {
        pr_debug(c"cannot create slab cache iter: %d\n".as_ptr(), fd);
        return;
    }

    /* This will run the bpf program */
    while read(fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf)) > 0 {
        continue;
    }

    close(fd);

    /* Read the slab cache map and build a hash with IDs */
    fd = bpf_map__fd((*skel).maps.slab_caches);
    prev_key = ptr::null_mut();
    while bpf_map_get_next_key(fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
        let data: *mut slab_cache_data;

        data = malloc(size_of::<slab_cache_data>()) as *mut slab_cache_data;
        if data.is_null() {
            break;
        }

        if bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, data as *mut c_void) < 0 {
            break;
        }

        hashmap__add(&mut slab_hash, (*data).id, data as *mut c_void);
        prev_key = &mut key;
    }
}

unsafe fn exit_slab_cache_iter() {
    let mut cur: *mut hashmap_entry = ptr::null_mut();
    let mut bkt: c_uint = 0;

    while hashmap__for_each_entry_next(&mut slab_hash, &mut cur, &mut bkt) {
        free((*cur).pvalue);
    }

    hashmap__clear(&mut slab_hash);
}

unsafe fn init_numa_data(con: *mut lock_contention) {
    let mut sym: *mut symbol;
    let mut kmap: *mut map = ptr::null_mut();
    let mut buf: *mut c_char = ptr::null_mut();
    let mut p: *mut c_char;
    let mut len: size_t = 0;
    let mut last: c_long = -1;
    let mut ret: c_int;

    if (*con).btf.is_null() {
        return;
    }

    /*
     * 'struct zone' is embedded in 'struct pglist_data' as an array.
     * As we may not have full information of the struct zone in the
     * (fake) vmlinux.h, let's get the actual size from BTF.
     */
    ret = btf__find_by_name_kind((*con).btf, c"zone".as_ptr(), BTF_KIND_STRUCT);
    if ret < 0 {
        pr_debug(c"cannot get type of struct zone: %d\n".as_ptr(), ret);
        return;
    }

    ret = btf__resolve_size((*con).btf, ret);
    if ret < 0 {
        pr_debug(c"cannot get size of struct zone: %d\n".as_ptr(), ret);
        return;
    }
    (*(*skel).rodata).sizeof_zone = ret;

    /* UMA system doesn't have 'node_data[]' - just use contig_page_data. */
    sym = machine__find_kernel_symbol_by_name((*con).machine, c"contig_page_data".as_ptr(), &mut kmap);
    if !sym.is_null() {
        (*(*skel).rodata).contig_page_data_addr = map__unmap_ip(kmap, (*sym).start);
        map__put(kmap);
        return;
    }

    /*
     * The 'node_data' is an array of pointers to struct pglist_data.
     * It needs to follow the pointer for each node in BPF to get the
     * address of struct pglist_data and its zones.
     */
    sym = machine__find_kernel_symbol_by_name((*con).machine, c"node_data".as_ptr(), &mut kmap);
    if sym.is_null() {
        return;
    }

    (*(*skel).rodata).node_data_addr = map__unmap_ip(kmap, (*sym).start);
    map__put(kmap);

    /* get the number of online nodes using the last node number + 1 */
    ret = sysfs__read_str(c"devices/system/node/online".as_ptr(), &mut buf, &mut len);
    if ret < 0 {
        pr_debug(c"failed to read online node: %d\n".as_ptr(), ret);
        return;
    }

    p = buf;
    while !p.is_null() && *p != 0 {
        last = strtol(p, &mut p, 0);

        if !p.is_null() && (*p == b',' as c_char || *p == b'-' as c_char || *p == b'\n' as c_char) {
            p = p.add(1);
        }
    }
    (*(*skel).rodata).nr_nodes = last + 1;
    free(buf as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn lock_contention_prepare(con: *mut lock_contention) -> c_int {
    let mut i: c_int;
    let mut fd: c_int;
    let mut ncpus: c_int = 1;
    let mut ntasks: c_int = 1;
    let mut ntypes: c_int = 1;
    let mut naddrs: c_int = 1;
    let mut ncgrps: c_int = 1;
    let mut nslabs: c_int = 1;
    let evlist = (*con).evlist;
    let target = (*con).target;
    let mut has_mmap_lock = false;

    /* make sure it loads the kernel map before lookup */
    map__load(machine__kernel_map((*con).machine));

    skel = lock_contention_bpf__open();
    if skel.is_null() {
        pr_err(c"Failed to open lock-contention BPF skeleton\n".as_ptr());
        return -1;
    }

    bpf_map__set_value_size((*skel).maps.stacks, ((*con).max_stack as usize * size_of::<u64>()) as c_int);
    bpf_map__set_max_entries((*skel).maps.lock_stat, (*con).map_nr_entries);
    bpf_map__set_max_entries((*skel).maps.tstamp, (*con).map_nr_entries);

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_TASK {
        bpf_map__set_max_entries((*skel).maps.task_data, (*con).map_nr_entries);
    } else {
        bpf_map__set_max_entries((*skel).maps.task_data, 1);
    }

    if (*con).save_callstack {
        bpf_map__set_max_entries((*skel).maps.stacks, (*con).map_nr_entries);
        if (*con).owner {
            bpf_map__set_value_size((*skel).maps.stack_buf, ((*con).max_stack as usize * size_of::<u64>()) as c_int);
            bpf_map__set_key_size((*skel).maps.owner_stacks, ((*con).max_stack as usize * size_of::<u64>()) as c_int);
            bpf_map__set_max_entries((*skel).maps.owner_stacks, (*con).map_nr_entries);
            bpf_map__set_max_entries((*skel).maps.owner_data, (*con).map_nr_entries);
            bpf_map__set_max_entries((*skel).maps.owner_stat, (*con).map_nr_entries);
            (*(*skel).rodata).max_stack = (*con).max_stack;
        }
    } else {
        bpf_map__set_max_entries((*skel).maps.stacks, 1);
    }

    if target__has_cpu(target) {
        (*(*skel).rodata).has_cpu = 1;
        ncpus = perf_cpu_map__nr((*evlist__core(evlist)).user_requested_cpus);
    }
    if target__has_task(target) {
        (*(*skel).rodata).has_task = 1;
        ntasks = perf_thread_map__nr((*evlist__core(evlist)).threads);
    }
    if (*(*con).filters).nr_types != 0 {
        (*(*skel).rodata).has_type = 1;
        ntypes = (*(*con).filters).nr_types;
    }
    if (*(*con).filters).nr_cgrps != 0 {
        (*(*skel).rodata).has_cgroup = 1;
        ncgrps = (*(*con).filters).nr_cgrps;
    }

    /* resolve lock name filters to addr */
    if (*(*con).filters).nr_syms != 0 {
        let mut sym: *mut symbol;
        let mut kmap: *mut map = ptr::null_mut();
        let mut addrs: *mut c_ulong;

        i = 0;
        while i < (*(*con).filters).nr_syms {
            if strcmp(*(*(*con).filters).syms.add(i as usize), c"mmap_lock".as_ptr()) == 0 {
                has_mmap_lock = true;
                i += 1;
                continue;
            }

            sym = machine__find_kernel_symbol_by_name(
                (*con).machine,
                *(*(*con).filters).syms.add(i as usize),
                &mut kmap,
            );
            if sym.is_null() {
                pr_warning(c"ignore unknown symbol: %s\n".as_ptr(), *(*(*con).filters).syms.add(i as usize));
                i += 1;
                continue;
            }

            addrs = realloc(
                (*(*con).filters).addrs as *mut c_void,
                (((*(*con).filters).nr_addrs + 1) as usize * size_of::<c_ulong>()) as size_t,
            ) as *mut c_ulong;
            if addrs.is_null() {
                pr_warning(c"memory allocation failure\n".as_ptr());
                i += 1;
                continue;
            }

            *addrs.add((*(*con).filters).nr_addrs as usize) = map__unmap_ip(kmap, (*sym).start) as c_ulong;
            (*(*con).filters).nr_addrs += 1;
            (*(*con).filters).addrs = addrs;
            i += 1;
        }
        naddrs = if (*(*con).filters).nr_addrs != 0 {
            (*(*con).filters).nr_addrs
        } else if has_mmap_lock {
            1
        } else {
            0
        };
        (*(*skel).rodata).has_addr = 1;
    }

    /* resolve lock name in delays */
    if (*con).nr_delays != 0 {
        let mut sym: *mut symbol;
        let mut kmap: *mut map = ptr::null_mut();

        i = 0;
        while i < (*con).nr_delays {
            sym = machine__find_kernel_symbol_by_name((*con).machine, (*(*con).delays.add(i as usize)).sym, &mut kmap);
            if sym.is_null() {
                pr_warning(c"ignore unknown symbol: %s\n".as_ptr(), (*(*con).delays.add(i as usize)).sym);
                i += 1;
                continue;
            }

            (*(*con).delays.add(i as usize)).addr = map__unmap_ip(kmap, (*sym).start) as c_ulong;
            i += 1;
        }
        (*(*skel).rodata).lock_delay = 1;
        bpf_map__set_max_entries((*skel).maps.lock_delays, (*con).nr_delays);
    }

    bpf_map__set_max_entries((*skel).maps.cpu_filter, ncpus);
    bpf_map__set_max_entries((*skel).maps.task_filter, ntasks);
    bpf_map__set_max_entries((*skel).maps.type_filter, ntypes);
    bpf_map__set_max_entries((*skel).maps.addr_filter, naddrs);
    bpf_map__set_max_entries((*skel).maps.cgroup_filter, ncgrps);

    (*(*skel).rodata).stack_skip = (*con).stack_skip;
    (*(*skel).rodata).aggr_mode = (*con).aggr_mode;
    (*(*skel).rodata).needs_callstack = (*con).save_callstack;
    (*(*skel).rodata).lock_owner = (*con).owner;
    (*(*skel).rodata).has_mmap_lock = has_mmap_lock;

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_CGROUP || (*(*con).filters).nr_cgrps != 0 {
        if cgroup_is_v2(c"perf_event".as_ptr()) {
            (*(*skel).rodata).use_cgroup_v2 = 1;
        }
    }

    check_slab_cache_iter(con);

    if (*(*con).filters).nr_slabs != 0 && has_slab_iter {
        (*(*skel).rodata).has_slab = 1;
        nslabs = (*(*con).filters).nr_slabs;
    }

    bpf_map__set_max_entries((*skel).maps.slab_filter, nslabs);

    init_numa_data(con);

    if lock_contention_bpf__load(skel) < 0 {
        pr_err(c"Failed to load lock-contention BPF skeleton\n".as_ptr());
        return -1;
    }

    if target__has_cpu(target) {
        let mut cpu: u32;
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.cpu_filter);

        i = 0;
        while i < ncpus {
            cpu = perf_cpu_map__cpu((*evlist__core(evlist)).user_requested_cpus, i).cpu as u32;
            bpf_map_update_elem(fd, &cpu as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
            i += 1;
        }
    }

    if target__has_task(target) {
        let mut pid: u32;
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);

        i = 0;
        while i < ntasks {
            pid = perf_thread_map__pid((*evlist__core(evlist)).threads, i) as u32;
            bpf_map_update_elem(fd, &pid as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
            i += 1;
        }
    }

    if target__none(target) && evlist__workload_pid(evlist) > 0 {
        let pid: u32 = evlist__workload_pid(evlist) as u32;
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.task_filter);
        bpf_map_update_elem(fd, &pid as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
    }

    if (*(*con).filters).nr_types != 0 {
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.type_filter);

        i = 0;
        while i < (*(*con).filters).nr_types {
            bpf_map_update_elem(
                fd,
                (*(*con).filters).types.add(i as usize) as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if (*(*con).filters).nr_addrs != 0 {
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.addr_filter);

        i = 0;
        while i < (*(*con).filters).nr_addrs {
            bpf_map_update_elem(
                fd,
                (*(*con).filters).addrs.add(i as usize) as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if (*(*con).filters).nr_cgrps != 0 {
        let val: u8 = 1;

        fd = bpf_map__fd((*skel).maps.cgroup_filter);

        i = 0;
        while i < (*(*con).filters).nr_cgrps {
            bpf_map_update_elem(
                fd,
                (*(*con).filters).cgrps.add(i as usize) as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if (*con).nr_delays != 0 {
        fd = bpf_map__fd((*skel).maps.lock_delays);

        i = 0;
        while i < (*con).nr_delays {
            bpf_map_update_elem(
                fd,
                &(*(*con).delays.add(i as usize)).addr as *const _ as *const c_void,
                &(*(*con).delays.add(i as usize)).time as *const _ as *const c_void,
                BPF_ANY,
            );
            i += 1;
        }
    }

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_CGROUP {
        read_all_cgroups(&mut (*con).cgroups);
    }

    bpf_program__set_autoload((*skel).progs.collect_lock_syms, false);

    lock_contention_bpf__attach(skel);

    /* run the slab iterator after attaching */
    run_slab_cache_iter();

    if (*(*con).filters).nr_slabs != 0 {
        let val: u8 = 1;
        let cache_fd: c_int;
        let mut key: c_long = 0;
        let mut prev_key: *mut c_long;

        fd = bpf_map__fd((*skel).maps.slab_filter);

        /* Read the slab cache map and build a hash with its address */
        cache_fd = bpf_map__fd((*skel).maps.slab_caches);
        prev_key = ptr::null_mut();
        while bpf_map_get_next_key(cache_fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
            let mut data: slab_cache_data = core::mem::zeroed();

            if bpf_map_lookup_elem(cache_fd, &key as *const _ as *const c_void, &mut data as *mut _ as *mut c_void) < 0 {
                break;
            }

            i = 0;
            while i < (*(*con).filters).nr_slabs {
                if strcmp(*(*(*con).filters).slabs.add(i as usize), data.name.as_ptr()) == 0 {
                    bpf_map_update_elem(fd, &key as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
                    break;
                }
                i += 1;
            }
            prev_key = &mut key;
        }
    }

    0
}

/*
 * Run the BPF program directly using BPF_PROG_TEST_RUN to update the end
 * timestamp in ktime so that it can calculate delta easily.
 */
unsafe fn mark_end_timestamp() {
    let mut opts = bpf_test_run_opts {
        flags: BPF_F_TEST_RUN_ON_CPU,
    };
    let prog_fd = bpf_program__fd((*skel).progs.end_timestamp);

    bpf_prog_test_run_opts(prog_fd, &mut opts);
}

unsafe fn update_lock_stat(
    map_fd: c_int,
    pid: c_int,
    end_ts: u64,
    aggr_mode: lock_aggr_mode,
    ts_data: *mut tstamp_data,
) {
    let delta: u64;
    let mut stat_key: contention_key = core::mem::zeroed();
    let mut stat_data: contention_data = core::mem::zeroed();

    if (*ts_data).timestamp >= end_ts {
        return;
    }

    delta = end_ts - (*ts_data).timestamp;

    match aggr_mode {
        lock_aggr_mode::LOCK_AGGR_CALLER => {
            stat_key.stack_id = (*ts_data).stack_id;
        }
        lock_aggr_mode::LOCK_AGGR_TASK => {
            stat_key.pid = pid;
        }
        lock_aggr_mode::LOCK_AGGR_ADDR => {
            stat_key.lock_addr_or_cgroup = (*ts_data).lock;
        }
        lock_aggr_mode::LOCK_AGGR_CGROUP => {
            stat_key.lock_addr_or_cgroup = (*ts_data).cgroup_id;
        }
    }

    if bpf_map_lookup_elem(map_fd, &stat_key as *const _ as *const c_void, &mut stat_data as *mut _ as *mut c_void) < 0 {
        return;
    }

    stat_data.total_time = stat_data.total_time.wrapping_add(delta);
    stat_data.count = stat_data.count.wrapping_add(1);

    if delta > stat_data.max_time {
        stat_data.max_time = delta;
    }
    if delta < stat_data.min_time {
        stat_data.min_time = delta;
    }

    bpf_map_update_elem(map_fd, &stat_key as *const _ as *const c_void, &stat_data as *const _ as *const c_void, BPF_EXIST);
}

/*
 * Account entries in the tstamp map (which didn't see the corresponding
 * lock:contention_end tracepoint) using end_ts.
 */
unsafe fn account_end_timestamp(con: *mut lock_contention) {
    let mut ts_fd: c_int;
    let stat_fd: c_int;
    let mut prev_key: *mut c_int;
    let mut key: c_int = 0;
    let end_ts: u64 = (*(*skel).bss).end_ts;
    let total_cpus: c_int;
    let aggr_mode = (*con).aggr_mode;
    let mut ts_data: tstamp_data = core::mem::zeroed();
    let cpu_data: *mut tstamp_data;

    /* Iterate per-task tstamp map (key = TID) */
    ts_fd = bpf_map__fd((*skel).maps.tstamp);
    stat_fd = bpf_map__fd((*skel).maps.lock_stat);

    prev_key = ptr::null_mut();
    while bpf_map_get_next_key(ts_fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
        if bpf_map_lookup_elem(ts_fd, &key as *const _ as *const c_void, &mut ts_data as *mut _ as *mut c_void) == 0 {
            let mut pid = key;

            if aggr_mode == lock_aggr_mode::LOCK_AGGR_TASK && (*con).owner {
                pid = ts_data.flags;
            }

            update_lock_stat(stat_fd, pid, end_ts, aggr_mode, &mut ts_data);
        }

        prev_key = &mut key;
    }

    /* Now it'll check per-cpu tstamp map which doesn't have TID. */
    if aggr_mode == lock_aggr_mode::LOCK_AGGR_TASK || aggr_mode == lock_aggr_mode::LOCK_AGGR_CGROUP {
        return;
    }

    total_cpus = cpu__max_cpu().cpu;
    ts_fd = bpf_map__fd((*skel).maps.tstamp_cpu);

    cpu_data = calloc(total_cpus as size_t, size_of::<tstamp_data>()) as *mut tstamp_data;
    if cpu_data.is_null() {
        return;
    }

    prev_key = ptr::null_mut();
    while bpf_map_get_next_key(ts_fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
        if bpf_map_lookup_elem(ts_fd, &key as *const _ as *const c_void, cpu_data as *mut c_void) < 0 {
            prev_key = &mut key;
            continue;
        }

        let mut i = 0;
        while i < total_cpus {
            if (*cpu_data.add(i as usize)).lock == 0 {
                i += 1;
                continue;
            }

            update_lock_stat(stat_fd, -1, end_ts, aggr_mode, cpu_data.add(i as usize));
            i += 1;
        }

        prev_key = &mut key;
    }
    free(cpu_data as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn lock_contention_start() -> c_int {
    (*(*skel).bss).enabled = 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn lock_contention_stop() -> c_int {
    (*(*skel).bss).enabled = 0;
    mark_end_timestamp();
    0
}

unsafe fn lock_contention_get_name(
    con: *mut lock_contention,
    key: *mut contention_key,
    stack_trace: *mut u64,
    mut flags: u32,
) -> *const c_char {
    let mut idx: c_int = 0;
    let addr: u64;
    static mut NAME_BUF: [c_char; KSYM_NAME_LEN] = [0; KSYM_NAME_LEN];
    let mut sym: *mut symbol;
    let mut kmap: *mut map = ptr::null_mut();
    let machine = (*con).machine;

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_TASK {
        let mut task: contention_task_data = core::mem::zeroed();
        let pid = (*key).pid;
        let task_fd = bpf_map__fd((*skel).maps.task_data);

        /* do not update idle comm which contains CPU number */
        if pid != 0 {
            let t = machine__findnew_thread(machine, -1, pid);

            if !t.is_null()
                && bpf_map_lookup_elem(task_fd, &pid as *const _ as *const c_void, &mut task as *mut _ as *mut c_void) == 0
                && thread__set_comm(t, task.comm.as_ptr(), 0)
            {
                snprintf(NAME_BUF.as_mut_ptr(), NAME_BUF.len(), c"%s".as_ptr(), task.comm.as_ptr());
                return NAME_BUF.as_ptr();
            }
        }
        return c"".as_ptr();
    }

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_ADDR {
        let lock_fd = bpf_map__fd((*skel).maps.lock_syms);
        let mut slab_data: *mut slab_cache_data = ptr::null_mut();

        /* per-process locks set upper bits of the flags */
        if flags & LCD_F_MMAP_LOCK != 0 {
            return c"mmap_lock".as_ptr();
        }
        if flags & LCD_F_SIGHAND_LOCK != 0 {
            return c"siglock".as_ptr();
        }

        /* global locks with symbols */
        sym = machine__find_kernel_symbol(machine, (*key).lock_addr_or_cgroup, &mut kmap);
        if !sym.is_null() {
            return (*sym).name;
        }

        /* try semi-global locks collected separately */
        if bpf_map_lookup_elem(
            lock_fd,
            &(*key).lock_addr_or_cgroup as *const _ as *const c_void,
            &mut flags as *mut _ as *mut c_void,
        ) == 0
        {
            if flags == LOCK_CLASS_RQLOCK {
                return c"rq_lock".as_ptr();
            }
        }

        if bpf_map_lookup_elem(
            lock_fd,
            &(*key).lock_addr_or_cgroup as *const _ as *const c_void,
            &mut flags as *mut _ as *mut c_void,
        ) == 0
        {
            if flags == LOCK_CLASS_ZONE_LOCK {
                return c"zone_lock".as_ptr();
            }
        }

        /* look slab_hash for dynamic locks in a slab object */
        if hashmap__find(&mut slab_hash, (flags & LCB_F_SLAB_ID_MASK) as c_long, &mut slab_data) {
            snprintf(NAME_BUF.as_mut_ptr(), NAME_BUF.len(), c"&%s".as_ptr(), (*slab_data).name.as_ptr());
            return NAME_BUF.as_ptr();
        }

        return c"".as_ptr();
    }

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_CGROUP {
        let cgrp_id = (*key).lock_addr_or_cgroup;
        let cgrp = __cgroup__find(&mut (*con).cgroups, cgrp_id);

        if !cgrp.is_null() {
            return (*cgrp).name;
        }

        snprintf(NAME_BUF.as_mut_ptr(), NAME_BUF.len(), c"cgroup:%llu".as_ptr(), cgrp_id);
        return NAME_BUF.as_ptr();
    }

    /* LOCK_AGGR_CALLER: skip lock internal functions */
    while machine__is_lock_function(machine, *stack_trace.add(idx as usize)) && idx < (*con).max_stack - 1 {
        idx += 1;
    }

    addr = *stack_trace.add(idx as usize);
    sym = machine__find_kernel_symbol(machine, addr, &mut kmap);

    if !sym.is_null() {
        let offset: c_ulong;

        offset = (map__map_ip(kmap, addr) - (*sym).start) as c_ulong;

        if offset == 0 {
            return (*sym).name;
        }

        snprintf(NAME_BUF.as_mut_ptr(), NAME_BUF.len(), c"%s+%#lx".as_ptr(), (*sym).name, offset);
    } else {
        snprintf(NAME_BUF.as_mut_ptr(), NAME_BUF.len(), c"%#lx".as_ptr(), addr as c_ulong);
    }

    NAME_BUF.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn pop_owner_stack_trace(con: *mut lock_contention) -> *mut lock_stat {
    let stacks_fd: c_int;
    let stat_fd: c_int;
    let mut stack_trace: *mut u64 = ptr::null_mut();
    let mut stack_id: s32 = 0;
    let mut ckey: contention_key = core::mem::zeroed();
    let mut cdata: contention_data = core::mem::zeroed();
    let stack_size = (*con).max_stack as usize * size_of::<u64>();
    let mut st: *mut lock_stat = ptr::null_mut();

    stacks_fd = bpf_map__fd((*skel).maps.owner_stacks);
    stat_fd = bpf_map__fd((*skel).maps.owner_stat);
    if stacks_fd == 0 || stat_fd == 0 {
        free(stack_trace as *mut c_void);
        free(st as *mut c_void);
        return ptr::null_mut();
    }

    stack_trace = zalloc(stack_size) as *mut u64;
    if stack_trace.is_null() {
        free(stack_trace as *mut c_void);
        free(st as *mut c_void);
        return ptr::null_mut();
    }

    if bpf_map_get_next_key(stacks_fd, ptr::null(), stack_trace as *mut c_void) != 0 {
        free(stack_trace as *mut c_void);
        free(st as *mut c_void);
        return ptr::null_mut();
    }

    bpf_map_lookup_elem(stacks_fd, stack_trace as *const c_void, &mut stack_id as *mut _ as *mut c_void);
    ckey.stack_id = stack_id;
    bpf_map_lookup_elem(stat_fd, &ckey as *const _ as *const c_void, &mut cdata as *mut _ as *mut c_void);

    st = zalloc(size_of::<lock_stat>()) as *mut lock_stat;
    if st.is_null() {
        free(stack_trace as *mut c_void);
        free(st as *mut c_void);
        return ptr::null_mut();
    }

    (*st).name = strdup(if *stack_trace != 0 {
        lock_contention_get_name(con, ptr::null_mut(), stack_trace, 0)
    } else {
        c"unknown".as_ptr()
    });
    if (*st).name.is_null() {
        free(stack_trace as *mut c_void);
        free(st as *mut c_void);
        return ptr::null_mut();
    }

    (*st).flags = cdata.flags;
    (*st).nr_contended = cdata.count;
    (*st).wait_time_total = cdata.total_time;
    (*st).wait_time_max = cdata.max_time;
    (*st).wait_time_min = cdata.min_time;
    (*st).callstack = stack_trace;

    if cdata.count != 0 {
        (*st).avg_wait_time = cdata.total_time / cdata.count;
    }

    bpf_map_delete_elem(stacks_fd, stack_trace as *const c_void);
    bpf_map_delete_elem(stat_fd, &ckey as *const _ as *const c_void);

    st
}

#[no_mangle]
pub unsafe extern "C" fn lock_contention_read(con: *mut lock_contention) -> c_int {
    let fd: c_int;
    let stack: c_int;
    let mut err: c_int = 0;
    let mut prev_key: *mut contention_key;
    let mut key: contention_key = core::mem::zeroed();
    let mut data: contention_data = core::mem::zeroed();
    let mut st: *mut lock_stat;
    let machine = (*con).machine;
    let stack_trace: *mut u64;
    let stack_size = (*con).max_stack as usize * size_of::<u64>();

    fd = bpf_map__fd((*skel).maps.lock_stat);
    stack = bpf_map__fd((*skel).maps.stacks);

    (*con).fails.task = (*(*skel).bss).task_fail;
    (*con).fails.stack = (*(*skel).bss).stack_fail;
    (*con).fails.time = (*(*skel).bss).time_fail;
    (*con).fails.data = (*(*skel).bss).data_fail;

    stack_trace = zalloc(stack_size) as *mut u64;
    if stack_trace.is_null() {
        return -1;
    }

    account_end_timestamp(con);

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_TASK {
        let idle = machine__findnew_thread(machine, 0, 0);
        thread__set_comm(idle, c"swapper".as_ptr(), 0);
    }

    if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_ADDR {
        let mut opts = bpf_test_run_opts {
            flags: BPF_F_TEST_RUN_ON_CPU,
        };
        let prog_fd = bpf_program__fd((*skel).progs.collect_lock_syms);

        bpf_prog_test_run_opts(prog_fd, &mut opts);
    }

    prev_key = ptr::null_mut();
    while bpf_map_get_next_key(fd, prev_key as *const c_void, &mut key as *mut _ as *mut c_void) == 0 {
        let ls_key: s64;
        let name: *const c_char;

        /* to handle errors in the loop body */
        err = -1;

        bpf_map_lookup_elem(fd, &key as *const _ as *const c_void, &mut data as *mut _ as *mut c_void);
        if (*con).save_callstack {
            bpf_map_lookup_elem(stack, &key.stack_id as *const _ as *const c_void, stack_trace as *mut c_void);

            if !match_callstack_filter(machine, stack_trace, (*con).max_stack) {
                (*con).nr_filtered = (*con).nr_filtered.wrapping_add(data.count);
                prev_key = &mut key;
                err = 0;
                continue;
            }
        }

        match (*con).aggr_mode {
            lock_aggr_mode::LOCK_AGGR_CALLER => {
                ls_key = key.stack_id as s64;
            }
            lock_aggr_mode::LOCK_AGGR_TASK => {
                ls_key = key.pid as s64;
            }
            lock_aggr_mode::LOCK_AGGR_ADDR | lock_aggr_mode::LOCK_AGGR_CGROUP => {
                ls_key = key.lock_addr_or_cgroup as s64;
            }
        }

        st = lock_stat_find(ls_key);
        if !st.is_null() {
            (*st).wait_time_total = (*st).wait_time_total.wrapping_add(data.total_time);
            if (*st).wait_time_max < data.max_time {
                (*st).wait_time_max = data.max_time;
            }
            if (*st).wait_time_min > data.min_time {
                (*st).wait_time_min = data.min_time;
            }

            (*st).nr_contended = (*st).nr_contended.wrapping_add(data.count);
            if (*st).nr_contended != 0 {
                (*st).avg_wait_time = (*st).wait_time_total / (*st).nr_contended;
            }
            prev_key = &mut key;
            err = 0;
            continue;
        }

        name = lock_contention_get_name(con, &mut key, stack_trace, data.flags);
        st = lock_stat_findnew(ls_key, name, data.flags);
        if st.is_null() {
            break;
        }

        (*st).nr_contended = data.count;
        (*st).wait_time_total = data.total_time;
        (*st).wait_time_max = data.max_time;
        (*st).wait_time_min = data.min_time;

        if data.count != 0 {
            (*st).avg_wait_time = data.total_time / data.count;
        }

        if (*con).aggr_mode == lock_aggr_mode::LOCK_AGGR_CALLER && verbose > 0 {
            (*st).callstack = memdup(stack_trace as *const c_void, stack_size) as *mut u64;
            if (*st).callstack.is_null() {
                break;
            }
        }

        prev_key = &mut key;

        /* we're fine now, reset the error */
        err = 0;
    }

    free(stack_trace as *mut c_void);

    err
}

#[no_mangle]
pub unsafe extern "C" fn lock_contention_finish(con: *mut lock_contention) -> c_int {
    if !skel.is_null() {
        (*(*skel).bss).enabled = 0;
        lock_contention_bpf__destroy(skel);
    }

    while !RB_EMPTY_ROOT(&mut (*con).cgroups) {
        let node = rb_first(&mut (*con).cgroups);
        let cgrp = rb_entry_cgroup(node);

        rb_erase(node, &mut (*con).cgroups);
        cgroup__put(cgrp);
    }

    exit_slab_cache_iter();
    btf__free((*con).btf);

    0
}

unsafe fn RB_EMPTY_ROOT(root: *mut rb_root) -> bool {
    (*root).rb_node.is_null()
}

unsafe fn rb_entry_cgroup(node: *mut rb_node) -> *mut cgroup {
    node as *mut cgroup
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}
