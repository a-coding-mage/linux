// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook
 */

// C headers and libbpf declarations are supplied by the surrounding build.

const MAX_NR_CPUS: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone)]
enum TestType {
    HashPrealloc,
    PercpuHashPrealloc,
    HashKmalloc,
    PercpuHashKmalloc,
    LruHashPrealloc,
    NocommonLruHashPrealloc,
    LpmKmalloc,
    HashLookup,
    ArrayLookup,
    InnerLruHashPrealloc,
    LruHashLookup,
    NrTests,
}

const TEST_MAP_NAMES: [&str; TestType::NrTests as usize] = [
    "hash_map", "percpu_hash_map", "hash_map_alloc", "percpu_hash_map_alloc",
    "lru_hash_map", "nocommon_lru_hash_map", "lpm_trie_map_alloc", "hash_map",
    "array_map", "inner_lru_hash_map", "lru_hash_lookup_map",
];

const NR_IDXES: usize = 3;
const ARRAY_OF_LRU_HASHS_IDX: usize = 0;
const HASH_MAP_ALLOC_IDX: usize = 1;
const LRU_HASH_LOOKUP_IDX: usize = 2;

static mut MAP_FD: [i32; NR_IDXES] = [0; NR_IDXES];
static mut TEST_FLAGS: i32 = -1;
static mut NUM_MAP_ENTRIES: u32 = 0;
static mut INNER_LRU_HASH_SIZE: u32 = 0;
static mut LRU_HASH_LOOKUP_TEST_ENTRIES: i32 = 32;
static mut MAX_CNT: u32 = 10000;

extern "C" {
    fn clock_gettime(clock: i32, ts: *mut libc::timespec) -> i32;
    fn bpf_map_update_elem(fd: i32, key: *const libc::c_void, value: *const libc::c_void, flags: u64) -> i32;
}

unsafe fn time_get_ns() -> u64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1, &mut ts);
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

unsafe fn pre_test_lru_hash_lookup(_tasks: i32) -> i32 { 0 }

unsafe fn do_test_lru(test: usize, cpu: i32) {
    let mut name = b"lru_hash_map_perf\0";
    if test == 5 { name = b"nocommon_lru_hash_map_perf\0"; }
    if test == 9 { name = b"inner_lru_hash_map_perf\0"; }
    if test == 10 { name = b"lru_hash_lookup_perf\0"; }
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::connect(-1, core::ptr::null(), 28); }
    libc::printf(b"%d:%s pre-alloc %lld events per sec\n\0".as_ptr() as _, cpu, name.as_ptr(), (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}
unsafe fn test_lru_hash_prealloc(cpu: i32) { do_test_lru(4, cpu); }
unsafe fn test_nocommon_lru_hash_prealloc(cpu: i32) { do_test_lru(5, cpu); }
unsafe fn test_inner_lru_hash_prealloc(cpu: i32) { do_test_lru(9, cpu); }
unsafe fn test_lru_hash_lookup(cpu: i32) { do_test_lru(10, cpu); }

#[inline]
unsafe fn check_test_flags(t: usize) -> i32 { TEST_FLAGS & (1i32 << t) }

unsafe fn test_hash_prealloc(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_getuid); }
    libc::printf(b"%d:hash_map_perf pre-alloc %lld events per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_percpu_hash_prealloc(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_geteuid); }
    libc::printf(b"%d:percpu_hash_map_perf pre-alloc %lld events per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_hash_kmalloc(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_getgid); }
    libc::printf(b"%d:hash_map_perf kmalloc %lld events per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_percpu_hash_kmalloc(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_getegid); }
    libc::printf(b"%d:percpu_hash_map_perf kmalloc %lld events per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_lpm_kmalloc(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_gettid); }
    libc::printf(b"%d:lpm_perf kmalloc %lld events per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_hash_lookup(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_getpgid, 0); }
    libc::printf(b"%d:hash_lookup %lld lookups per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000 * 64) / (time_get_ns() - start_time) as i64);
}

unsafe fn test_array_lookup(cpu: i32) {
    let start_time = time_get_ns();
    for _ in 0..MAX_CNT { libc::syscall(libc::SYS_getppid, 0); }
    libc::printf(b"%d:array_lookup %lld lookups per sec\n\0".as_ptr() as _, cpu, (MAX_CNT as i64 * 1_000_000_000 * 64) / (time_get_ns() - start_time) as i64);
}

type TestFunc = unsafe fn(i32);
const TEST_FUNCS: [Option<TestFunc>; 11] = [Some(test_hash_prealloc), Some(test_percpu_hash_prealloc), Some(test_hash_kmalloc), Some(test_percpu_hash_kmalloc), Some(test_lru_hash_prealloc), Some(test_nocommon_lru_hash_prealloc), Some(test_lpm_kmalloc), Some(test_hash_lookup), Some(test_array_lookup), Some(test_inner_lru_hash_prealloc), Some(test_lru_hash_lookup)];

unsafe fn loop_cpu(cpu: i32) {
    for i in 0..TestType::NrTests as usize {
        if check_test_flags(i) != 0 { if let Some(f) = TEST_FUNCS[i] { f(cpu); } }
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut libc::c_char) -> i32 {
    let tasks = 1;
    let _ = pre_test_lru_hash_lookup(tasks);
    for cpu in 0..tasks { loop_cpu(cpu); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
