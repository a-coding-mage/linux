// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Facebook
 */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

const BPF_MAP_TYPE_HASH: c_int = 1;
const BPF_MAP_TYPE_LRU_HASH: c_int = 9;
const BPF_F_NO_COMMON_LRU: c_int = 1 << 1;
const BPF_EXIST: c_int = 2;
const BPF_NOEXIST: c_int = 1;

#[repr(C)]
pub struct BpfMapCreateOpts {
    pub map_flags: c_int,
}

extern "C" {
    fn bpf_map_create(map_type: c_int, map_name: *const c_char, key_size: usize,
                      value_size: usize, max_entries: u32, opts: *const BpfMapCreateOpts) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_num_possible_cpus() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fork() -> c_int;
    fn waitpid(pid: c_int, status: *mut c_int, options: c_int) -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const c_void) -> c_int;
    fn rand() -> c_int;
    fn srand(seed: u32);
    fn time(t: *mut i64) -> i64;
    fn atoi(s: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
}

static mut NR_CPUS: c_int = 0;
static mut DIST_KEYS: *mut u64 = ptr::null_mut();
static mut DIST_KEY_COUNTS: u32 = 0;

#[repr(C)]
struct ListHead { next: *mut ListHead, prev: *mut ListHead }

unsafe fn init_list_head(list: *mut ListHead) { (*list).next = list; (*list).prev = list; }
unsafe fn __list_add(new: *mut ListHead, prev: *mut ListHead, next: *mut ListHead) {
    (*next).prev = new; (*new).next = next; (*new).prev = prev; (*prev).next = new;
}
unsafe fn list_add(new: *mut ListHead, head: *mut ListHead) { __list_add(new, head, (*head).next); }
unsafe fn __list_del(prev: *mut ListHead, next: *mut ListHead) { (*next).prev = prev; (*prev).next = next; }
unsafe fn __list_del_entry(entry: *mut ListHead) { __list_del((*entry).prev, (*entry).next); }
unsafe fn list_move(list: *mut ListHead, head: *mut ListHead) { __list_del_entry(list); list_add(list, head); }

#[repr(C)]
struct PfectLruNode { list: ListHead, key: u64 }
#[repr(C)]
struct PfectLru { list: ListHead, free_nodes: *mut PfectLruNode, cur_size: u32, lru_size: u32, nr_unique: u32, nr_misses: u32, total: u32, map_fd: c_int }

unsafe fn pfect_lru_init(lru: *mut PfectLru, lru_size: u32, nr_possible_elems: u32) {
    (*lru).map_fd = bpf_map_create(BPF_MAP_TYPE_HASH, ptr::null(), 8, std::mem::size_of::<*mut PfectLruNode>(), nr_possible_elems, ptr::null());
    assert!((*lru).map_fd != -1);
    (*lru).free_nodes = libc_malloc(lru_size as usize * std::mem::size_of::<PfectLruNode>()) as *mut PfectLruNode;
    assert!(!(*lru).free_nodes.is_null());
    init_list_head(&mut (*lru).list); (*lru).cur_size = 0; (*lru).lru_size = lru_size;
    (*lru).nr_unique = 0; (*lru).nr_misses = 0; (*lru).total = 0;
}
unsafe fn pfect_lru_destroy(lru: *mut PfectLru) { close((*lru).map_fd); libc_free((*lru).free_nodes as *mut c_void); }
unsafe fn pfect_lru_lookup_or_insert(lru: *mut PfectLru, key: u64) -> c_int {
    let mut node: *mut PfectLruNode = ptr::null_mut(); let mut seen = 0;
    (*lru).total += 1;
    if bpf_map_lookup_elem((*lru).map_fd, &key as *const _ as *const c_void, &mut node as *mut _ as *mut c_void) == 0 {
        if !node.is_null() { list_move(&mut (*node).list, &mut (*lru).list); return 1; } seen = 1;
    }
    if (*lru).cur_size < (*lru).lru_size { node = (*lru).free_nodes.add((*lru).cur_size as usize); (*lru).cur_size += 1; init_list_head(&mut (*node).list); }
    else { node = (*(*lru).list.prev as *mut PfectLruNode); let null_node: *mut PfectLruNode = ptr::null_mut(); bpf_map_update_elem((*lru).map_fd, &(*node).key as *const _ as *const c_void, &null_node as *const _ as *const c_void, BPF_EXIST as u64); }
    (*node).key = key; list_move(&mut (*node).list, &mut (*lru).list); (*lru).nr_misses += 1;
    if seen != 0 { assert!(bpf_map_update_elem((*lru).map_fd, &key as *const _ as *const c_void, &node as *const _ as *const c_void, BPF_EXIST as u64) == 0); }
    else { (*lru).nr_unique += 1; assert!(bpf_map_update_elem((*lru).map_fd, &key as *const _ as *const c_void, &node as *const _ as *const c_void, BPF_NOEXIST as u64) == 0); } seen
}

extern "C" { fn libc_malloc(size: usize) -> *mut c_void; fn libc_free(p: *mut c_void); }

unsafe fn create_map(map_type: c_int, map_flags: c_int, size: u32) -> c_int {
    let opts = BpfMapCreateOpts { map_flags }; let fd = bpf_map_create(map_type, ptr::null(), 8, 8, size, &opts);
    if fd == -1 { perror(b"bpf_create_map\0".as_ptr() as *const c_char); } fd
}

unsafe fn read_keys(_dist_file: *const c_char, keys: *mut *mut u64) -> u32 { *keys = ptr::null_mut(); 0 }
unsafe fn sched_next_online(_pid: c_int, mut next: c_int) -> c_int { if next == NR_CPUS { return -1; } while next < NR_CPUS { next += 1; break; } next }
type TestFn = unsafe fn(c_int, *mut c_void);
unsafe fn run_parallel(tasks: u32, f: TestFn, data: *mut c_void) {
    let mut pids = Vec::new(); let mut cpu = 0;
    for i in 0..tasks { let pid = fork(); if pid == 0 { cpu = sched_next_online(0, cpu); f(i as c_int, data); exit(0); } else if pid == -1 { exit(1); } else { pids.push(pid); cpu = sched_next_online(pid, cpu); } }
    for pid in pids { let mut status = 0; assert!(waitpid(pid, &mut status, 0) == pid); assert!(status == 0); }
}
unsafe fn do_test_lru_dist(task: c_int, data: *mut c_void) {
    let d = data as *mut u32; let mut lru = PfectLru { list: ListHead { next: ptr::null_mut(), prev: ptr::null_mut() }, free_nodes: ptr::null_mut(), cur_size: 0, lru_size: *d.add(1), nr_unique: 0, nr_misses: 0, total: 0, map_fd: -1 };
    pfect_lru_init(&mut lru, *d.add(1), DIST_KEY_COUNTS);
    for i in 0..DIST_KEY_COUNTS { let key = *DIST_KEYS.add(i as usize) + task as u64 * DIST_KEY_COUNTS as u64; pfect_lru_lookup_or_insert(&mut lru, key); }
    pfect_lru_destroy(&mut lru);
}
unsafe fn test_parallel_lru_dist(map_type: c_int, flags: c_int, tasks: u32, size: u32) { let fd = create_map(map_type, flags, if flags & BPF_F_NO_COMMON_LRU != 0 { NR_CPUS as u32 * size } else { tasks * size }); assert!(fd != -1); let mut d = [fd as u32, size]; run_parallel(tasks, do_test_lru_dist, d.as_mut_ptr() as *mut c_void); close(fd); }
unsafe fn test_lru_loss0(map_type: c_int, flags: c_int) { let fd = create_map(map_type, flags, if flags & BPF_F_NO_COMMON_LRU != 0 { 900 * NR_CPUS as u32 } else { 900 }); assert!(fd != -1); close(fd); }
unsafe fn test_lru_loss1(map_type: c_int, flags: c_int) { let fd = create_map(map_type, flags, if flags & BPF_F_NO_COMMON_LRU != 0 { 1000 * NR_CPUS as u32 } else { 1000 }); assert!(fd != -1); close(fd); }
unsafe fn do_test_parallel_lru_loss(_task: c_int, _data: *mut c_void) {}
unsafe fn test_parallel_lru_loss(map_type: c_int, flags: c_int, tasks: u32) { let fd = create_map(map_type, flags, if flags & BPF_F_NO_COMMON_LRU != 0 { NR_CPUS as u32 * 1200 } else { tasks * 1200 }); assert!(fd != -1); run_parallel(tasks, do_test_parallel_lru_loss, &fd as *const _ as *mut c_void); close(fd); }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc < 4 { return -1; }
    NR_CPUS = bpf_num_possible_cpus(); assert!(NR_CPUS != -1);
    let size = atoi(*argv.add(2)) as u32; let mut tasks = atoi(*argv.add(3)) as u32; tasks = std::cmp::min(tasks, NR_CPUS as u32);
    DIST_KEY_COUNTS = read_keys(*argv.add(1), &mut DIST_KEYS); if DIST_KEY_COUNTS == 0 { return -1; }
    for flags in [0, BPF_F_NO_COMMON_LRU] { test_lru_loss0(BPF_MAP_TYPE_LRU_HASH, flags); test_lru_loss1(BPF_MAP_TYPE_LRU_HASH, flags); test_parallel_lru_loss(BPF_MAP_TYPE_LRU_HASH, flags, tasks); test_parallel_lru_dist(BPF_MAP_TYPE_LRU_HASH, flags, tasks, size); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
