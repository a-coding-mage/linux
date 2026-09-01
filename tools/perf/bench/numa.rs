// SPDX-License-Identifier: GPL-2.0
/*
 * numa.c
 *
 * numa: Simulate NUMA-sensitive workload and measure their NUMA performance
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type ssize_t = isize;
type size_t = usize;
type pid_t = c_int;
type pthread_t = c_ulong;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cond {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bitmask {
    size: c_ulong,
    maskp: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    tv_sec: c_long,
    tv_usec: c_long,
}

#[repr(C)]
pub struct rusage {
    ru_utime: timeval,
    ru_stime: timeval,
}

const RUSAGE_THREAD: c_int = 1;
const HPSIZE: ssize_t = 2 * 1024 * 1024;
const MAX_ARGS: usize = 50;
const NUMA_NO_NODE: c_int = -1;
const NSEC_PER_SEC: u64 = 1_000_000_000;
const NSEC_PER_USEC: u64 = 1_000;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANON: c_int = 0x20;
const MADV_HUGEPAGE: c_int = 14;
const MADV_NOHUGEPAGE: c_int = 15;
const MPOL_DEFAULT: c_int = 0;
const MPOL_BIND: c_int = 2;
const PR_SET_NAME: c_int = 15;
const BENCH_FORMAT_DEFAULT: c_int = 0;

#[repr(C)]
struct thread_data {
    curr_cpu: c_int,
    bind_cpumask: *mut cpu_set_t,
    bind_node: c_int,
    process_data: *mut u8,
    process_nr: c_int,
    thread_nr: c_int,
    task_nr: c_int,
    loops_done: c_uint,
    val: u64,
    runtime_ns: u64,
    system_time_ns: u64,
    user_time_ns: u64,
    speed_gbs: c_double,
    process_lock: *mut mutex,
}

/* Parameters set by options: */
#[repr(C)]
#[derive(Copy, Clone)]
struct params {
    /* Startup synchronization: */
    serialize_startup: bool,
    /* Task hierarchy: */
    nr_proc: c_int,
    nr_threads: c_int,
    /* Working set sizes: */
    mb_global_str: *const c_char,
    mb_proc_str: *const c_char,
    mb_proc_locked_str: *const c_char,
    mb_thread_str: *const c_char,
    mb_global: c_double,
    mb_proc: c_double,
    mb_proc_locked: c_double,
    mb_thread: c_double,
    /* Access patterns to the working set: */
    data_reads: bool,
    data_writes: bool,
    data_backwards: bool,
    data_zero_memset: bool,
    data_rand_walk: bool,
    nr_loops: u32,
    nr_secs: u32,
    sleep_usecs: u32,
    /* Working set initialization: */
    init_zero: bool,
    init_random: bool,
    init_cpu0: bool,
    /* Misc options: */
    show_details: c_int,
    run_all: c_int,
    thp: c_int,
    bytes_global: c_long,
    bytes_process: c_long,
    bytes_process_locked: c_long,
    bytes_thread: c_long,
    nr_tasks: c_int,
    show_convergence: bool,
    measure_convergence: bool,
    perturb_secs: c_int,
    nr_cpus: c_int,
    nr_nodes: c_int,
    /* Affinity options -C and -N: */
    cpu_list_str: *mut c_char,
    node_list_str: *mut c_char,
}

/* Global, read-writable area, accessible to all processes and threads: */
#[repr(C)]
struct global_info {
    data: *mut u8,
    startup_mutex: mutex,
    startup_cond: cond,
    nr_tasks_started: c_int,
    start_work_mutex: mutex,
    start_work_cond: cond,
    nr_tasks_working: c_int,
    start_work: bool,
    stop_work_mutex: mutex,
    bytes_done: u64,
    threads: *mut thread_data,
    /* Convergence latency measurement: */
    all_converged: bool,
    stop_work: bool,
    print_once: c_int,
    p: params,
}

static mut g: *mut global_info = null_mut();
static mut p0: params = unsafe { zeroed() };

/* The parse-options table is supplied by perf's C option macros in the original. */
static options: [option; 0] = [];
static bench_numa_usage: [*const c_char; 2] = [c"perf bench numa <options>".as_ptr(), null()];
static numa_usage: [*const c_char; 2] = [c"perf bench numa mem [<options>]".as_ptr(), null()];

unsafe extern "C" {
    static mut quiet: bool;
    static mut bench_format: c_int;
    static mut numa_nodes_ptr: *mut bitmask;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    static mut stdout: *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn atol(nptr: *const c_char) -> c_long;
    fn atof(nptr: *const c_char) -> c_double;
    fn rand() -> c_int;
    fn bzero(s: *mut c_void, n: size_t);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mmap(addr: *mut c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: c_long) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn madvise(addr: *mut c_void, length: size_t, advice: c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn sched_getaffinity(pid: pid_t, cpusetsize: size_t, mask: *mut cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn sched_getcpu() -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn getrusage(who: c_int, usage: *mut rusage) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn system(command: *const c_char) -> c_int;
    fn pthread_create(thread: *mut pthread_t, attr: *const c_void, start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn numa_bitmask_isbitset(mask: *const bitmask, n: c_uint) -> c_int;
    fn numa_allocate_cpumask() -> *mut bitmask;
    fn numa_free_cpumask(mask: *mut bitmask);
    fn numa_node_to_cpus(node: c_int, mask: *mut bitmask) -> c_int;
    fn numa_num_possible_cpus() -> c_int;
    fn numa_num_configured_cpus() -> c_int;
    fn numa_max_node() -> c_int;
    fn numa_node_of_cpu(cpu: c_int) -> c_int;
    fn numa_allocate_nodemask() -> *mut bitmask;
    fn numa_bitmask_clearall(mask: *mut bitmask) -> *mut bitmask;
    fn numa_bitmask_setbit(mask: *mut bitmask, n: c_uint) -> *mut bitmask;
    fn numa_bitmask_free(mask: *mut bitmask);
    fn set_mempolicy(mode: c_int, nodemask: *const c_ulong, maxnode: c_ulong) -> c_int;
    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_FREE(cpuset: *mut cpu_set_t);
    fn mutex_init(m: *mut mutex);
    fn mutex_init_pshared(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn cond_init_pshared(c: *mut cond);
    fn cond_signal(c: *mut cond);
    fn cond_broadcast(c: *mut cond);
    fn cond_wait(c: *mut cond, m: *mut mutex);
    fn sysfs__read_str(entry: *const c_char, buf: *mut *mut c_char, size: *mut size_t) -> c_int;
    fn parse_options(argc: c_int, argv: *const *const c_char, options: *const option, usagestr: *const *const c_char, flags: c_int) -> c_int;
    fn usage_with_options(usagestr: *const *const c_char, options: *const option);
}

unsafe fn BUG_ON(cond: bool) {
    if cond {
        panic!("BUG_ON");
    }
}

unsafe fn timersub(a: *const timeval, b: *const timeval, r: *mut timeval) {
    (*r).tv_sec = (*a).tv_sec - (*b).tv_sec;
    (*r).tv_usec = (*a).tv_usec - (*b).tv_usec;
    if (*r).tv_usec < 0 {
        (*r).tv_sec -= 1;
        (*r).tv_usec += 1_000_000;
    }
}

/*
 * To get number of numa nodes present.
 */
unsafe fn nr_numa_nodes() -> c_int {
    let mut i = 0;
    let mut nr_nodes = 0;
    while i < (*g).p.nr_nodes {
        if numa_bitmask_isbitset(numa_nodes_ptr, i as c_uint) != 0 {
            nr_nodes += 1;
        }
        i += 1;
    }
    nr_nodes
}

/*
 * To check if given numa node is present.
 */
unsafe fn is_node_present(node: c_int) -> c_int {
    numa_bitmask_isbitset(numa_nodes_ptr, node as c_uint)
}

/*
 * To check given numa node has cpus.
 */
unsafe fn node_has_cpus(node: c_int) -> bool {
    let cpumask = numa_allocate_cpumask();
    let mut ret = false; /* fall back to nocpus */
    BUG_ON(cpumask.is_null());
    if numa_node_to_cpus(node, cpumask) == 0 {
        let mut cpu = 0;
        while cpu < (*cpumask).size as c_int {
            if numa_bitmask_isbitset(cpumask, cpu as c_uint) != 0 {
                ret = true;
                break;
            }
            cpu += 1;
        }
    }
    numa_free_cpumask(cpumask);
    ret
}

unsafe fn bind_to_cpu(target_cpu: c_int) -> *mut cpu_set_t {
    let nrcpus = numa_num_possible_cpus();
    let orig_mask = CPU_ALLOC(nrcpus);
    BUG_ON(orig_mask.is_null());
    let size = CPU_ALLOC_SIZE(nrcpus);
    CPU_ZERO_S(size, orig_mask);
    if sched_getaffinity(0, size, orig_mask) != 0 {
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    let mask = CPU_ALLOC(nrcpus);
    if mask.is_null() {
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    CPU_ZERO_S(size, mask);
    if target_cpu == -1 {
        let mut cpu = 0;
        while cpu < (*g).p.nr_cpus {
            CPU_SET_S(cpu, size, mask);
            cpu += 1;
        }
    } else {
        if target_cpu < 0 || target_cpu >= (*g).p.nr_cpus {
            CPU_FREE(mask);
            CPU_FREE(orig_mask);
            BUG_ON(true);
            return null_mut();
        }
        CPU_SET_S(target_cpu, size, mask);
    }
    if sched_setaffinity(0, size, mask) != 0 {
        CPU_FREE(mask);
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    orig_mask
}

unsafe fn bind_to_node(target_node: c_int) -> *mut cpu_set_t {
    let nrcpus = numa_num_possible_cpus();
    let orig_mask = CPU_ALLOC(nrcpus);
    BUG_ON(orig_mask.is_null());
    let size = CPU_ALLOC_SIZE(nrcpus);
    CPU_ZERO_S(size, orig_mask);
    if sched_getaffinity(0, size, orig_mask) != 0 {
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    let mask = CPU_ALLOC(nrcpus);
    if mask.is_null() {
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    CPU_ZERO_S(size, mask);
    if target_node == NUMA_NO_NODE {
        let mut cpu = 0;
        while cpu < (*g).p.nr_cpus {
            CPU_SET_S(cpu, size, mask);
            cpu += 1;
        }
    } else {
        let cpumask = numa_allocate_cpumask();
        if cpumask.is_null() {
            CPU_FREE(mask);
            CPU_FREE(orig_mask);
            BUG_ON(true);
            return null_mut();
        }
        if numa_node_to_cpus(target_node, cpumask) == 0 {
            let mut cpu = 0;
            while cpu < (*cpumask).size as c_int {
                if numa_bitmask_isbitset(cpumask, cpu as c_uint) != 0 {
                    CPU_SET_S(cpu, size, mask);
                }
                cpu += 1;
            }
        }
        numa_free_cpumask(cpumask);
    }
    if sched_setaffinity(0, size, mask) != 0 {
        CPU_FREE(mask);
        CPU_FREE(orig_mask);
        BUG_ON(true);
        return null_mut();
    }
    orig_mask
}

unsafe fn bind_to_cpumask(mask: *mut cpu_set_t) {
    let size = CPU_ALLOC_SIZE(numa_num_possible_cpus());
    let ret = sched_setaffinity(0, size, mask);
    if ret != 0 {
        CPU_FREE(mask);
        BUG_ON(ret != 0);
    }
}

unsafe fn mempol_restore() {
    let ret = set_mempolicy(MPOL_DEFAULT, null(), ((*g).p.nr_nodes - 1) as c_ulong);
    BUG_ON(ret != 0);
}

unsafe fn bind_to_memnode(node: c_int) {
    if node == NUMA_NO_NODE {
        return;
    }
    let node_mask = numa_allocate_nodemask();
    BUG_ON(node_mask.is_null());
    numa_bitmask_clearall(node_mask);
    numa_bitmask_setbit(node_mask, node as c_uint);
    let ret = set_mempolicy(MPOL_BIND, (*node_mask).maskp, (*node_mask).size + 1);
    numa_bitmask_free(node_mask);
    BUG_ON(ret != 0);
}

unsafe fn set_taskname2(fmt: *const c_char, a: c_int, b: c_int) {
    let mut name = [0 as c_char; 20];
    snprintf(name.as_mut_ptr(), 20, fmt, a, b);
    prctl(PR_SET_NAME, name.as_ptr());
}

unsafe fn alloc_data(bytes0: ssize_t, map_flags: c_int, init_zero: c_int, init_cpu0: c_int, thp: c_int, init_random: c_int) -> *mut u8 {
    if bytes0 == 0 {
        return null_mut();
    }
    let mut orig_mask: *mut cpu_set_t = null_mut();
    if init_cpu0 != 0 {
        let node = numa_node_of_cpu(0);
        orig_mask = bind_to_node(node);
        bind_to_memnode(node);
    }
    let bytes = bytes0 + HPSIZE;
    let mut buf = mmap(null_mut(), bytes as size_t, PROT_READ | PROT_WRITE, MAP_ANON | map_flags, -1, 0) as *mut u8;
    BUG_ON(buf == (-1isize) as *mut u8);
    if map_flags == MAP_PRIVATE {
        if thp > 0 {
            let ret = madvise(buf as *mut c_void, bytes as size_t, MADV_HUGEPAGE);
            if ret != 0 && (*g).print_once == 0 {
                (*g).print_once = 1;
                printf(c"WARNING: Could not enable THP - do: 'echo madvise > /sys/kernel/mm/transparent_hugepage/enabled'\n".as_ptr());
            }
        }
        if thp < 0 {
            let ret = madvise(buf as *mut c_void, bytes as size_t, MADV_NOHUGEPAGE);
            if ret != 0 && (*g).print_once == 0 {
                (*g).print_once = 1;
                printf(c"WARNING: Could not disable THP: run a CONFIG_TRANSPARENT_HUGEPAGE kernel?\n".as_ptr());
            }
        }
    }
    if init_zero != 0 {
        bzero(buf as *mut c_void, bytes as size_t);
    } else if init_random != 0 {
        let wbuf = buf as *mut u64;
        let off = rand() as c_long;
        let mut i = 0;
        while i < bytes / 8 {
            *wbuf.offset(i) = (i + off) as u64;
            i += 1;
        }
    }
    buf = (((buf as c_ulong + HPSIZE as c_ulong - 1) & !(HPSIZE as c_ulong - 1)) as *mut u8);
    if init_cpu0 != 0 {
        bind_to_cpumask(orig_mask);
        CPU_FREE(orig_mask);
        mempol_restore();
    }
    buf
}

unsafe fn free_data(data: *mut c_void, bytes: ssize_t) {
    if data.is_null() {
        return;
    }
    let ret = munmap(data, bytes as size_t);
    BUG_ON(ret != 0);
}

/* Create a shared memory buffer that can be shared between processes, zeroed: */
unsafe fn zalloc_shared_data(bytes: ssize_t) -> *mut c_void {
    alloc_data(bytes, MAP_SHARED, 1, (*g).p.init_cpu0 as c_int, (*g).p.thp, (*g).p.init_random as c_int) as *mut c_void
}

/* Create a shared memory buffer that can be shared between processes: */
unsafe fn setup_shared_data(bytes: ssize_t) -> *mut c_void {
    alloc_data(bytes, MAP_SHARED, 0, (*g).p.init_cpu0 as c_int, (*g).p.thp, (*g).p.init_random as c_int) as *mut c_void
}

/* Allocate process-local memory - this will either be shared between threads of this process, or only be accessed by this thread: */
unsafe fn setup_private_data(bytes: ssize_t) -> *mut c_void {
    alloc_data(bytes, MAP_PRIVATE, 0, (*g).p.init_cpu0 as c_int, (*g).p.thp, (*g).p.init_random as c_int) as *mut c_void
}

unsafe fn parse_cpu_list(arg: *const c_char) -> c_int {
    p0.cpu_list_str = strdup(arg);
    0
}

/*
 * Check whether a CPU is online
 *
 * Returns:
 *     1 -> if CPU is online
 *     0 -> if CPU is offline
 *    -1 -> error case
 */
unsafe fn is_cpu_online(cpu: c_uint) -> c_int {
    let mut s: *mut c_char = null_mut();
    let mut len: size_t = 0;
    let mut buf = [0 as c_char; 256];
    let mut status = -1;
    snprintf(buf.as_mut_ptr(), buf.len(), c"/sys/devices/system/cpu/cpu%d".as_ptr(), cpu);
    /* stat() checks are preserved in intent; sysfs__read_str supplies the file contents when present. */
    snprintf(buf.as_mut_ptr(), buf.len(), c"devices/system/cpu/cpu%d/online".as_ptr(), cpu);
    if sysfs__read_str(buf.as_ptr(), &mut s, &mut len) < 0 {
        return status;
    }
    status = atoi(s);
    free(s as *mut c_void);
    status
}

unsafe fn parse_setup_cpu_list() -> c_int {
    if (*g).p.cpu_list_str.is_null() {
        return 0;
    }
    let mut str0 = strdup((*g).p.cpu_list_str);
    let mut strp = str0;
    let mut t = 0;
    BUG_ON(strp.is_null());
    printf(c"# binding tasks to CPUs:\n#  ".as_ptr());
    loop {
        let tok = strsep(&mut strp, c",".as_ptr());
        if tok.is_null() {
            break;
        }
        let tok_end = strstr(tok, c"-".as_ptr());
        let (bind_cpu_0, bind_cpu_1) = if tok_end.is_null() {
            let c = atol(tok) as c_int;
            (c, c)
        } else {
            (atol(tok) as c_int, atol(tok_end.add(1)) as c_int)
        };
        let tok_step = strstr(tok, c"#".as_ptr());
        let step = if tok_step.is_null() { 1 } else { atol(tok_step.add(1)) as c_int };
        BUG_ON(step <= 0 || step >= (*g).p.nr_cpus);
        let tok_len = strstr(tok, c"_".as_ptr());
        let bind_len = if tok_len.is_null() { 1 } else { atol(tok_len.add(1)) as c_int };
        BUG_ON(bind_len <= 0 || bind_len > (*g).p.nr_cpus);
        let tok_mul = strstr(tok, c"x".as_ptr());
        let mul = if tok_mul.is_null() { 1 } else { atol(tok_mul.add(1)) as c_int };
        BUG_ON(mul <= 0);
        if bind_cpu_0 >= (*g).p.nr_cpus || bind_cpu_1 >= (*g).p.nr_cpus {
            printf(c"\nTest not applicable, system has only %d CPUs.\n".as_ptr(), (*g).p.nr_cpus);
            return -1;
        }
        if is_cpu_online(bind_cpu_0 as c_uint) != 1 || is_cpu_online(bind_cpu_1 as c_uint) != 1 {
            printf(c"\nTest not applicable, bind_cpu_0 or bind_cpu_1 is offline\n".as_ptr());
            return -1;
        }
        BUG_ON(bind_cpu_0 < 0 || bind_cpu_1 < 0);
        BUG_ON(bind_cpu_0 > bind_cpu_1);
        let mut bind_cpu = bind_cpu_0;
        while bind_cpu <= bind_cpu_1 {
            let size = CPU_ALLOC_SIZE((*g).p.nr_cpus);
            let mut i = 0;
            while i < mul {
                if t >= (*g).p.nr_tasks {
                    printf(c"\n# NOTE: ignoring bind CPUs starting at CPU#%d\n #".as_ptr(), bind_cpu);
                    free(str0 as *mut c_void);
                    return 0;
                }
                let td = (*g).threads.offset(t as isize);
                (*td).bind_cpumask = CPU_ALLOC((*g).p.nr_cpus);
                BUG_ON((*td).bind_cpumask.is_null());
                CPU_ZERO_S(size, (*td).bind_cpumask);
                let mut cpu = bind_cpu;
                while cpu < bind_cpu + bind_len {
                    if cpu < 0 || cpu >= (*g).p.nr_cpus {
                        CPU_FREE((*td).bind_cpumask);
                        BUG_ON(true);
                    }
                    CPU_SET_S(cpu, size, (*td).bind_cpumask);
                    cpu += 1;
                }
                t += 1;
                i += 1;
            }
            bind_cpu += step;
        }
    }
    if t < (*g).p.nr_tasks {
        printf(c"# NOTE: %d tasks bound, %d tasks unbound\n".as_ptr(), t, (*g).p.nr_tasks - t);
    }
    free(str0 as *mut c_void);
    0
}

unsafe extern "C" fn parse_cpus_opt(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if arg.is_null() { return -1; }
    parse_cpu_list(arg)
}

unsafe fn parse_node_list(arg: *const c_char) -> c_int {
    p0.node_list_str = strdup(arg);
    0
}

unsafe fn parse_setup_node_list() -> c_int {
    if (*g).p.node_list_str.is_null() {
        return 0;
    }
    let mut str0 = strdup((*g).p.node_list_str);
    let mut strp = str0;
    let mut t = 0;
    BUG_ON(strp.is_null());
    printf(c"# binding tasks to NODEs:\n# ".as_ptr());
    loop {
        let tok = strsep(&mut strp, c",".as_ptr());
        if tok.is_null() { break; }
        let tok_end = strstr(tok, c"-".as_ptr());
        let (bind_node_0, bind_node_1) = if tok_end.is_null() {
            let n = atol(tok) as c_int;
            (n, n)
        } else {
            (atol(tok) as c_int, atol(tok_end.add(1)) as c_int)
        };
        let tok_step = strstr(tok, c"#".as_ptr());
        let step = if tok_step.is_null() { 1 } else { atol(tok_step.add(1)) as c_int };
        BUG_ON(step <= 0 || step >= (*g).p.nr_nodes);
        let tok_mul = strstr(tok, c"x".as_ptr());
        let mul = if tok_mul.is_null() { 1 } else { atol(tok_mul.add(1)) as c_int };
        BUG_ON(mul <= 0);
        if bind_node_0 >= (*g).p.nr_nodes || bind_node_1 >= (*g).p.nr_nodes {
            printf(c"\nTest not applicable, system has only %d nodes.\n".as_ptr(), (*g).p.nr_nodes);
            return -1;
        }
        BUG_ON(bind_node_0 < 0 || bind_node_1 < 0);
        BUG_ON(bind_node_0 > bind_node_1);
        let mut bind_node = bind_node_0;
        while bind_node <= bind_node_1 {
            let mut i = 0;
            while i < mul {
                if t >= (*g).p.nr_tasks || !node_has_cpus(bind_node) {
                    printf(c"\n# NOTE: ignoring bind NODEs starting at NODE#%d\n".as_ptr(), bind_node);
                    free(str0 as *mut c_void);
                    return 0;
                }
                let td = (*g).threads.offset(t as isize);
                (*td).bind_node = bind_node;
                t += 1;
                i += 1;
            }
            bind_node += step;
        }
    }
    if t < (*g).p.nr_tasks {
        printf(c"# NOTE: %d tasks mem-bound, %d tasks unbound\n".as_ptr(), t, (*g).p.nr_tasks - t);
    }
    free(str0 as *mut c_void);
    0
}

unsafe extern "C" fn parse_nodes_opt(_opt: *const option, arg: *const c_char, _unset: c_int) -> c_int {
    if arg.is_null() { return -1; }
    parse_node_list(arg)
}

#[inline]
fn lfsr_32(lfsr: u32) -> u32 {
    let taps: u32 = (1u32 << 1) | (1u32 << 5) | (1u32 << 6) | (1u32 << 31);
    (lfsr >> 1) ^ ((0u32.wrapping_sub(lfsr & 1)) & taps)
}

/*
 * Make sure there's real data dependency to RAM (when read
 * accesses are enabled), so the compiler, the CPU and the
 * kernel (KSM, zero page, etc.) cannot optimize away RAM
 * accesses:
 */
unsafe fn access_data(data: *mut u64, mut val: u64) -> u64 {
    if (*g).p.data_reads {
        val = val.wrapping_add(*data);
    }
    if (*g).p.data_writes {
        *data = val.wrapping_add(1);
    }
    val
}

unsafe fn do_work(__data: *mut u8, bytes: c_long, nr: c_int, nr_max: c_int, loop_: c_int, mut val: u64) -> u64 {
    let words = bytes / size_of::<u64>() as c_long;
    let data = __data as *mut u64;
    BUG_ON(data.is_null() && words != 0);
    BUG_ON(!data.is_null() && words == 0);
    if data.is_null() {
        return val;
    }
    if (*g).p.data_zero_memset && !(*g).p.data_rand_walk {
        bzero(data as *mut c_void, bytes as size_t);
        return val;
    }
    let chunk_0 = words / nr_max as c_long;
    let chunk_1 = words / (*g).p.nr_loops as c_long;
    let mut off = nr as c_long * chunk_0 + loop_ as c_long * chunk_1;
    while off >= words {
        off -= words;
    }
    if (*g).p.data_rand_walk {
        let mut lfsr = (nr as u64).wrapping_add(loop_ as u64).wrapping_add(val) as u32;
        let mut i = 0;
        while i < words / 1024 {
            lfsr = lfsr_32(lfsr);
            let start = (lfsr as c_long) % words;
            let end = core::cmp::min(start + 1024, words - 1);
            if (*g).p.data_zero_memset {
                bzero(data.offset(start) as *mut c_void, ((end - start) as usize) * size_of::<u64>());
            } else {
                let mut j = start;
                while j < end {
                    val = access_data(data.offset(j), val);
                    j += 1;
                }
            }
            i += 1;
        }
    } else if !(*g).p.data_backwards || ((nr + loop_) & 1) != 0 {
        let d0 = data.offset(off);
        let mut d = data.offset(off + 1);
        let d1 = data.offset(words);
        loop {
            if d >= d1 { d = data; }
            if d == d0 { break; }
            val = access_data(d, val);
            d = d.add(1);
        }
    } else {
        let d0 = data.offset(off);
        let mut d = data.offset(off - 1);
        loop {
            if d < data { d = data.offset(words - 1); }
            if d == d0 { break; }
            val = access_data(d, val);
            d = d.offset(-1);
        }
    }
    val
}

unsafe fn update_curr_cpu(task_nr: c_int, bytes_worked: c_ulong) {
    let cpu = sched_getcpu();
    (*(*g).threads.offset(task_nr as isize)).curr_cpu = cpu;
    prctl(0, bytes_worked);
}

unsafe fn count_process_nodes(process_nr: c_int) -> c_int {
    let node_present = calloc((*g).p.nr_nodes as size_t, size_of::<c_char>()) as *mut c_char;
    BUG_ON(node_present.is_null());
    let mut t = 0;
    while t < (*g).p.nr_threads {
        let task_nr = process_nr * (*g).p.nr_threads + t;
        let td = (*g).threads.offset(task_nr as isize);
        let node = numa_node_of_cpu((*td).curr_cpu);
        if node < 0 {
            free(node_present as *mut c_void);
            return 0;
        }
        *node_present.offset(node as isize) = 1;
        t += 1;
    }
    let mut nodes = 0;
    let mut n = 0;
    while n < (*g).p.nr_nodes {
        nodes += *node_present.offset(n as isize) as c_int;
        n += 1;
    }
    free(node_present as *mut c_void);
    nodes
}

unsafe fn count_node_processes(node: c_int) -> c_int {
    let mut processes = 0;
    let mut p = 0;
    while p < (*g).p.nr_proc {
        let mut t = 0;
        while t < (*g).p.nr_threads {
            let task_nr = p * (*g).p.nr_threads + t;
            let td = (*g).threads.offset(task_nr as isize);
            if numa_node_of_cpu((*td).curr_cpu) == node {
                processes += 1;
                break;
            }
            t += 1;
        }
        p += 1;
    }
    processes
}

unsafe fn calc_convergence_compression(strong: *mut c_int) {
    let mut nodes_min: c_uint = !0;
    let mut nodes_max: c_uint = 0;
    let mut p = 0;
    while p < (*g).p.nr_proc {
        let nodes = count_process_nodes(p) as c_uint;
        if nodes == 0 {
            *strong = 0;
            return;
        }
        nodes_min = core::cmp::min(nodes, nodes_min);
        nodes_max = core::cmp::max(nodes, nodes_max);
        p += 1;
    }
    if nodes_min == 1 && nodes_max == 1 {
        *strong = 1;
    } else {
        *strong = 0;
        printf(c" {%d-%d}".as_ptr(), nodes_min, nodes_max);
    }
}

unsafe fn calc_convergence(runtime_ns_max: c_double, convergence: *mut c_double) {
    if !(*g).p.show_convergence && !(*g).p.measure_convergence {
        return;
    }
    let nodes = calloc((*g).p.nr_nodes as size_t, size_of::<c_int>()) as *mut c_int;
    BUG_ON(nodes.is_null());
    let mut loops_done_min: c_uint = !0;
    let mut loops_done_max: c_uint = 0;
    let mut t = 0;
    while t < (*g).p.nr_tasks {
        let td = (*g).threads.offset(t as isize);
        let cpu = (*td).curr_cpu;
        if cpu >= 0 {
            let node = numa_node_of_cpu(cpu);
            *nodes.offset(node as isize) += 1;
            loops_done_min = core::cmp::min((*td).loops_done, loops_done_min);
            loops_done_max = core::cmp::max((*td).loops_done, loops_done_max);
        }
        t += 1;
    }
    let mut nr_max = 0;
    let mut nr_min = (*g).p.nr_tasks;
    let mut sum = 0;
    let mut node = 0;
    while node < (*g).p.nr_nodes {
        if is_node_present(node) != 0 {
            let nr = *nodes.offset(node as isize);
            nr_min = core::cmp::min(nr, nr_min);
            nr_max = core::cmp::max(nr, nr_max);
            sum += nr;
        }
        node += 1;
    }
    BUG_ON(nr_min > nr_max);
    BUG_ON(sum > (*g).p.nr_tasks);
    let mut process_groups = 0;
    node = 0;
    while node < (*g).p.nr_nodes {
        if is_node_present(node) != 0 {
            let processes = count_node_processes(node);
            let nr = *nodes.offset(node as isize);
            printf(c" %2d/%-2d".as_ptr(), nr, processes);
            process_groups += processes;
        }
        node += 1;
    }
    let distance = nr_max - nr_min;
    printf(c" [%2d/%-2d]".as_ptr(), distance, process_groups);
    printf(c" l:%3d-%-3d (%3d)".as_ptr(), loops_done_min, loops_done_max, loops_done_max - loops_done_min);
    if loops_done_min != 0 && loops_done_max != 0 {
        let skew = 1.0 - loops_done_min as c_double / loops_done_max as c_double;
        printf(c" [%4.1f%%]".as_ptr(), skew * 100.0);
    }
    let mut strong = 0;
    calc_convergence_compression(&mut strong);
    if strong != 0 && process_groups == (*g).p.nr_proc {
        if *convergence == 0.0 {
            *convergence = runtime_ns_max;
            printf(c" (%6.1fs converged)\n".as_ptr(), *convergence / NSEC_PER_SEC as c_double);
            if (*g).p.measure_convergence {
                (*g).all_converged = true;
                (*g).stop_work = true;
            }
        }
    } else {
        if *convergence != 0.0 {
            printf(c" (%6.1fs de-converged)".as_ptr(), runtime_ns_max / NSEC_PER_SEC as c_double);
            *convergence = 0.0;
        }
        printf(c"\n".as_ptr());
    }
    free(nodes as *mut c_void);
}

unsafe fn show_summary(runtime_ns_max: c_double, l: c_int, convergence: *mut c_double) {
    printf(c"\r #  %5.1f%%  [%.1f mins]".as_ptr(), (l + 1) as c_double / (*g).p.nr_loops as c_double * 100.0, runtime_ns_max / NSEC_PER_SEC as c_double / 60.0);
    calc_convergence(runtime_ns_max, convergence);
    if (*g).p.show_details >= 0 {
        fflush(stdout);
    }
}

unsafe extern "C" fn worker_thread(__tdata: *mut c_void) -> *mut c_void {
    let td = __tdata as *mut thread_data;
    bind_to_cpumask((*td).bind_cpumask);
    bind_to_memnode((*td).bind_node);
    set_taskname2(c"thread %d/%d".as_ptr(), (*td).process_nr, (*td).thread_nr);
    let global_data = (*g).data;
    let process_data = (*td).process_data;
    let thread_data = setup_private_data((*g).p.bytes_thread) as *mut u8;
    let mut bytes_done: u64 = 0;
    let last_task = ((*td).process_nr == (*g).p.nr_proc - 1 && (*td).thread_nr == (*g).p.nr_threads - 1) as c_int;
    let first_task = ((*td).process_nr == 0 && (*td).thread_nr == 0) as c_int;
    if (*g).p.serialize_startup {
        mutex_lock(&mut (*g).startup_mutex);
        (*g).nr_tasks_started += 1;
        if (*g).nr_tasks_started == (*g).p.nr_tasks { cond_signal(&mut (*g).startup_cond); }
        mutex_unlock(&mut (*g).startup_mutex);
        mutex_lock(&mut (*g).start_work_mutex);
        (*g).start_work = false;
        (*g).nr_tasks_working += 1;
        while !(*g).start_work { cond_wait(&mut (*g).start_work_cond, &mut (*g).start_work_mutex); }
        mutex_unlock(&mut (*g).start_work_mutex);
    }
    let mut start0: timeval = zeroed();
    let mut start: timeval = zeroed();
    let mut stop: timeval = zeroed();
    let mut diff: timeval = zeroed();
    gettimeofday(&mut start0, null_mut());
    start = start0;
    stop = start0;
    let mut last_perturbance = start.tv_sec as c_ulong;
    let mut convergence = 0.0;
    let mut val = (*td).val;
    let mut l: u32 = 0;
    while l < (*g).p.nr_loops {
        start = stop;
        if (*g).stop_work { break; }
        val = val.wrapping_add(do_work(global_data, (*g).p.bytes_global, (*td).process_nr, (*g).p.nr_proc, l as c_int, val));
        val = val.wrapping_add(do_work(process_data, (*g).p.bytes_process, (*td).thread_nr, (*g).p.nr_threads, l as c_int, val));
        val = val.wrapping_add(do_work(thread_data, (*g).p.bytes_thread, 0, 1, l as c_int, val));
        if (*g).p.sleep_usecs != 0 {
            mutex_lock((*td).process_lock);
            usleep((*g).p.sleep_usecs);
            mutex_unlock((*td).process_lock);
        }
        if (*g).p.bytes_process_locked != 0 {
            mutex_lock((*td).process_lock);
            val = val.wrapping_add(do_work(process_data, (*g).p.bytes_process_locked, (*td).thread_nr, (*g).p.nr_threads, l as c_int, val));
            mutex_unlock((*td).process_lock);
        }
        let work_done = (*g).p.bytes_global + (*g).p.bytes_process + (*g).p.bytes_process_locked + (*g).p.bytes_thread;
        update_curr_cpu((*td).task_nr, work_done as c_ulong);
        bytes_done = bytes_done.wrapping_add(work_done as u64);
        if (*g).p.show_details < 0 && (*g).p.perturb_secs == 0 && !(*g).p.measure_convergence && (*g).p.nr_secs == 0 {
            l += 1;
            continue;
        }
        (*td).loops_done = l;
        gettimeofday(&mut stop, null_mut());
        if (*g).p.nr_secs != 0 {
            timersub(&stop, &start0, &mut diff);
            if diff.tv_sec as u32 >= (*g).p.nr_secs {
                (*g).stop_work = true;
                break;
            }
        }
        if start.tv_sec == stop.tv_sec {
            l += 1;
            continue;
        }
        if first_task != 0 && (*g).p.perturb_secs != 0 && (stop.tv_sec - last_perturbance as c_long) as c_int >= (*g).p.perturb_secs {
            last_perturbance = stop.tv_sec as c_ulong;
            let this_cpu = (*(*g).threads.offset((*td).task_nr as isize)).curr_cpu;
            let target_cpu = if this_cpu < (*g).p.nr_cpus / 2 { (*g).p.nr_cpus - 1 } else { 0 };
            let orig_mask = bind_to_cpu(target_cpu);
            bind_to_cpumask(orig_mask);
            CPU_FREE(orig_mask);
        }
        if last_task != 0 {
            timersub(&stop, &start0, &mut diff);
            let runtime_ns_max = diff.tv_sec as c_double * NSEC_PER_SEC as c_double + diff.tv_usec as c_double * NSEC_PER_USEC as c_double;
            show_summary(runtime_ns_max, l as c_int, &mut convergence);
        }
        l += 1;
    }
    gettimeofday(&mut stop, null_mut());
    timersub(&stop, &start0, &mut diff);
    (*td).runtime_ns = diff.tv_sec as u64 * NSEC_PER_SEC + diff.tv_usec as u64 * NSEC_PER_USEC;
    let secs = (*td).runtime_ns / NSEC_PER_SEC;
    (*td).speed_gbs = if secs != 0 { bytes_done as c_double / secs as c_double / 1e9 } else { 0.0 };
    let mut ru: rusage = zeroed();
    getrusage(RUSAGE_THREAD, &mut ru);
    (*td).system_time_ns = ru.ru_stime.tv_sec as u64 * NSEC_PER_SEC + ru.ru_stime.tv_usec as u64 * NSEC_PER_USEC;
    (*td).user_time_ns = ru.ru_utime.tv_sec as u64 * NSEC_PER_SEC + ru.ru_utime.tv_usec as u64 * NSEC_PER_USEC;
    free_data(thread_data as *mut c_void, (*g).p.bytes_thread);
    mutex_lock(&mut (*g).stop_work_mutex);
    (*g).bytes_done = (*g).bytes_done.wrapping_add(bytes_done);
    mutex_unlock(&mut (*g).stop_work_mutex);
    null_mut()
}

/* A worker process starts a couple of threads: */
unsafe fn worker_process(process_nr: c_int) {
    let mut process_lock: mutex = zeroed();
    mutex_init(&mut process_lock);
    set_taskname2(c"process %d".as_ptr(), process_nr, 0);
    let task_nr = process_nr * (*g).p.nr_threads;
    let td0 = (*g).threads.offset(task_nr as isize);
    bind_to_memnode((*td0).bind_node);
    bind_to_cpumask((*td0).bind_cpumask);
    let pthreads = calloc((*g).p.nr_threads as size_t, size_of::<pthread_t>()) as *mut pthread_t;
    let process_data = setup_private_data((*g).p.bytes_process) as *mut u8;
    let mut t = 0;
    while t < (*g).p.nr_threads {
        let task_nr = process_nr * (*g).p.nr_threads + t;
        let td = (*g).threads.offset(task_nr as isize);
        (*td).process_data = process_data;
        (*td).process_nr = process_nr;
        (*td).thread_nr = t;
        (*td).task_nr = task_nr;
        (*td).val = rand() as u64;
        (*td).curr_cpu = -1;
        (*td).process_lock = &mut process_lock;
        let ret = pthread_create(pthreads.offset(t as isize), null(), worker_thread, td as *mut c_void);
        BUG_ON(ret != 0);
        t += 1;
    }
    t = 0;
    while t < (*g).p.nr_threads {
        let ret = pthread_join(*pthreads.offset(t as isize), null_mut());
        BUG_ON(ret != 0);
        t += 1;
    }
    free_data(process_data as *mut c_void, (*g).p.bytes_process);
    free(pthreads as *mut c_void);
}

unsafe fn print_summary() {
    if (*g).p.show_details < 0 { return; }
    printf(c"\n ###\n".as_ptr());
    printf(c" # %d %s will execute (on %d nodes, %d CPUs):\n".as_ptr(), (*g).p.nr_tasks, if (*g).p.nr_tasks == 1 { c"task".as_ptr() } else { c"tasks".as_ptr() }, nr_numa_nodes(), (*g).p.nr_cpus);
    printf(c" #      %5dx %5ldMB global  shared mem operations\n".as_ptr(), (*g).p.nr_loops, (*g).p.bytes_global / 1024 / 1024);
    printf(c" #      %5dx %5ldMB process shared mem operations\n".as_ptr(), (*g).p.nr_loops, (*g).p.bytes_process / 1024 / 1024);
    printf(c" #      %5dx %5ldMB thread  local  mem operations\n".as_ptr(), (*g).p.nr_loops, (*g).p.bytes_thread / 1024 / 1024);
    printf(c" ###\n\n ###\n".as_ptr());
    fflush(stdout);
}

unsafe fn init_thread_data() {
    let size = (size_of::<thread_data>() as ssize_t) * (*g).p.nr_tasks as ssize_t;
    (*g).threads = zalloc_shared_data(size) as *mut thread_data;
    let mut t = 0;
    while t < (*g).p.nr_tasks {
        let td = (*g).threads.offset(t as isize);
        let cpuset_size = CPU_ALLOC_SIZE((*g).p.nr_cpus);
        (*td).bind_node = NUMA_NO_NODE;
        (*td).bind_cpumask = CPU_ALLOC((*g).p.nr_cpus);
        BUG_ON((*td).bind_cpumask.is_null());
        CPU_ZERO_S(cpuset_size, (*td).bind_cpumask);
        let mut cpu = 0;
        while cpu < (*g).p.nr_cpus {
            CPU_SET_S(cpu, cpuset_size, (*td).bind_cpumask);
            cpu += 1;
        }
        t += 1;
    }
}

unsafe fn deinit_thread_data() {
    let size = (size_of::<thread_data>() as ssize_t) * (*g).p.nr_tasks as ssize_t;
    let mut t = 0;
    while t < (*g).p.nr_tasks {
        CPU_FREE((*(*g).threads.offset(t as isize)).bind_cpumask);
        t += 1;
    }
    free_data((*g).threads as *mut c_void, size);
}

unsafe fn init() -> c_int {
    g = alloc_data(size_of::<global_info>() as ssize_t, MAP_SHARED, 1, 0, 0, 0) as *mut global_info;
    (*g).p = p0;
    (*g).p.nr_cpus = numa_num_configured_cpus();
    (*g).p.nr_nodes = numa_max_node() + 1;
    BUG_ON((*g).p.nr_nodes < 0);
    if quiet && (*g).p.show_details == 0 {
        (*g).p.show_details = -1;
    }
    if (*g).p.mb_global_str.is_null() && (*g).p.mb_proc_str.is_null() && (*g).p.mb_thread_str.is_null() {
        return -1;
    }
    if !(*g).p.mb_global_str.is_null() { (*g).p.mb_global = atof((*g).p.mb_global_str); BUG_ON((*g).p.mb_global < 0.0); }
    if !(*g).p.mb_proc_str.is_null() { (*g).p.mb_proc = atof((*g).p.mb_proc_str); BUG_ON((*g).p.mb_proc < 0.0); }
    if !(*g).p.mb_proc_locked_str.is_null() {
        (*g).p.mb_proc_locked = atof((*g).p.mb_proc_locked_str);
        BUG_ON((*g).p.mb_proc_locked < 0.0);
        BUG_ON((*g).p.mb_proc_locked > (*g).p.mb_proc);
    }
    if !(*g).p.mb_thread_str.is_null() { (*g).p.mb_thread = atof((*g).p.mb_thread_str); BUG_ON((*g).p.mb_thread < 0.0); }
    BUG_ON((*g).p.nr_threads <= 0);
    BUG_ON((*g).p.nr_proc <= 0);
    (*g).p.nr_tasks = (*g).p.nr_proc * (*g).p.nr_threads;
    (*g).p.bytes_global = ((*g).p.mb_global * 1024.0 * 1024.0) as c_long;
    (*g).p.bytes_process = ((*g).p.mb_proc * 1024.0 * 1024.0) as c_long;
    (*g).p.bytes_process_locked = ((*g).p.mb_proc_locked * 1024.0 * 1024.0) as c_long;
    (*g).p.bytes_thread = ((*g).p.mb_thread * 1024.0 * 1024.0) as c_long;
    (*g).data = setup_shared_data((*g).p.bytes_global) as *mut u8;
    mutex_init_pshared(&mut (*g).start_work_mutex);
    cond_init_pshared(&mut (*g).start_work_cond);
    mutex_init_pshared(&mut (*g).startup_mutex);
    cond_init_pshared(&mut (*g).startup_cond);
    mutex_init_pshared(&mut (*g).stop_work_mutex);
    init_thread_data();
    if parse_setup_cpu_list() != 0 || parse_setup_node_list() != 0 {
        return -1;
    }
    print_summary();
    0
}

unsafe fn deinit() {
    free_data((*g).data as *mut c_void, (*g).p.bytes_global);
    (*g).data = null_mut();
    deinit_thread_data();
    free_data(g as *mut c_void, size_of::<global_info>() as ssize_t);
    g = null_mut();
}

unsafe fn print_res(mut name: *const c_char, val: c_double, txt_unit: *const c_char, txt_short: *const c_char, txt_long: *const c_char) {
    if name.is_null() { name = c"main,".as_ptr(); }
    if !quiet {
        printf(c" %-30s %15.3f, %-15s %s\n".as_ptr(), name, val, txt_unit, txt_short);
    } else {
        printf(c" %14.3f %s\n".as_ptr(), val, txt_long);
    }
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn __bench_numa(name: *const c_char) -> c_int {
    let mut start: timeval = zeroed();
    let mut stop: timeval = zeroed();
    let mut diff: timeval = zeroed();
    if init() != 0 { return -1; }
    let pids = calloc((*g).p.nr_proc as size_t, size_of::<pid_t>()) as *mut pid_t;
    gettimeofday(&mut start, null_mut());
    let mut i = 0;
    while i < (*g).p.nr_proc {
        let pid = fork();
        BUG_ON(pid < 0);
        if pid == 0 {
            worker_process(i);
            exit(0);
        }
        *pids.offset(i as isize) = pid;
        i += 1;
    }
    if (*g).p.serialize_startup {
        let mut threads_ready = false;
        mutex_lock(&mut (*g).startup_mutex);
        while (*g).nr_tasks_started != (*g).p.nr_tasks {
            cond_wait(&mut (*g).startup_cond, &mut (*g).startup_mutex);
        }
        mutex_unlock(&mut (*g).startup_mutex);
        while !threads_ready {
            mutex_lock(&mut (*g).start_work_mutex);
            threads_ready = (*g).nr_tasks_working == (*g).p.nr_tasks;
            mutex_unlock(&mut (*g).start_work_mutex);
            if !threads_ready { usleep(1); }
        }
        gettimeofday(&mut stop, null_mut());
        start = stop;
        mutex_lock(&mut (*g).start_work_mutex);
        (*g).start_work = true;
        mutex_unlock(&mut (*g).start_work_mutex);
        cond_broadcast(&mut (*g).start_work_cond);
    } else {
        gettimeofday(&mut start, null_mut());
    }
    i = 0;
    while i < (*g).p.nr_proc {
        let mut wait_stat = 0;
        let wpid = waitpid(*pids.offset(i as isize), &mut wait_stat, 0);
        BUG_ON(wpid < 0);
        BUG_ON(!WIFEXITED(wait_stat));
        i += 1;
    }
    let mut runtime_ns_sum: u64 = 0;
    let mut runtime_ns_min: u64 = !0;
    let mut t = 0;
    while t < (*g).p.nr_tasks {
        let thread_runtime_ns = (*(*g).threads.offset(t as isize)).runtime_ns;
        runtime_ns_sum = runtime_ns_sum.wrapping_add(thread_runtime_ns);
        runtime_ns_min = core::cmp::min(thread_runtime_ns, runtime_ns_min);
        t += 1;
    }
    gettimeofday(&mut stop, null_mut());
    timersub(&stop, &start, &mut diff);
    BUG_ON(bench_format != BENCH_FORMAT_DEFAULT);
    let runtime_sec_max = (diff.tv_sec as c_double * NSEC_PER_SEC as c_double + diff.tv_usec as c_double * NSEC_PER_USEC as c_double) / NSEC_PER_SEC as c_double;
    let runtime_sec_min = runtime_ns_min as c_double / NSEC_PER_SEC as c_double;
    let bytes = (*g).bytes_done as c_double;
    let runtime_avg = runtime_ns_sum as c_double / (*g).p.nr_tasks as c_double / NSEC_PER_SEC as c_double;
    if (*g).p.measure_convergence {
        print_res(name, runtime_sec_max, c"secs,".as_ptr(), c"NUMA-convergence-latency".as_ptr(), c"secs latency to NUMA-converge".as_ptr());
    }
    print_res(name, runtime_sec_max, c"secs,".as_ptr(), c"runtime-max/thread".as_ptr(), c"secs slowest (max) thread-runtime".as_ptr());
    print_res(name, runtime_sec_min, c"secs,".as_ptr(), c"runtime-min/thread".as_ptr(), c"secs fastest (min) thread-runtime".as_ptr());
    print_res(name, runtime_avg, c"secs,".as_ptr(), c"runtime-avg/thread".as_ptr(), c"secs average thread-runtime".as_ptr());
    let delta_runtime = (runtime_sec_max - runtime_sec_min) / 2.0;
    print_res(name, delta_runtime / runtime_sec_max * 100.0, c"%,".as_ptr(), c"spread-runtime/thread".as_ptr(), c"% difference between max/avg runtime".as_ptr());
    print_res(name, bytes / (*g).p.nr_tasks as c_double / 1e9, c"GB,".as_ptr(), c"data/thread".as_ptr(), c"GB data processed, per thread".as_ptr());
    print_res(name, bytes / 1e9, c"GB,".as_ptr(), c"data-total".as_ptr(), c"GB data processed, total".as_ptr());
    print_res(name, runtime_sec_max * NSEC_PER_SEC as c_double / (bytes / (*g).p.nr_tasks as c_double), c"nsecs,".as_ptr(), c"runtime/byte/thread".as_ptr(), c"nsecs/byte/thread runtime".as_ptr());
    print_res(name, bytes / (*g).p.nr_tasks as c_double / 1e9 / runtime_sec_max, c"GB/sec,".as_ptr(), c"thread-speed".as_ptr(), c"GB/sec/thread speed".as_ptr());
    print_res(name, bytes / runtime_sec_max / 1e9, c"GB/sec,".as_ptr(), c"total-speed".as_ptr(), c"GB/sec total speed".as_ptr());
    free(pids as *mut c_void);
    deinit();
    0
}

unsafe fn command_size(mut argv: *const *const c_char) -> c_int {
    let mut size = 0;
    while !(*argv).is_null() {
        size += 1;
        argv = argv.add(1);
    }
    BUG_ON(size >= MAX_ARGS as c_int);
    size
}

unsafe fn init_params(p: *mut params, name: *const c_char, argc: c_int, argv: *const *const c_char) {
    printf(c"\n # Running %s \"perf bench numa".as_ptr(), name);
    let mut i = 0;
    while i < argc {
        printf(c" %s".as_ptr(), *argv.offset(i as isize));
        i += 1;
    }
    printf(c"\"\n".as_ptr());
    memset(p as *mut c_void, 0, size_of::<params>());
    (*p).serialize_startup = true;
    (*p).data_reads = true;
    (*p).data_writes = true;
    (*p).data_backwards = true;
    (*p).data_rand_walk = true;
    (*p).nr_loops = !0;
    (*p).init_random = true;
    (*p).mb_global_str = c"1".as_ptr();
    (*p).nr_proc = 1;
    (*p).nr_threads = 1;
    (*p).nr_secs = 5;
    (*p).run_all = (argc == 1) as c_int;
}

unsafe fn run_bench_numa(name: *const c_char, argv: *const *const c_char) -> c_int {
    let mut argc = command_size(argv);
    init_params(&mut p0, name, argc, argv);
    argc = parse_options(argc, argv, options.as_ptr(), bench_numa_usage.as_ptr(), 0);
    if argc != 0 { return -1; }
    if __bench_numa(name) != 0 { return -1; }
    0
}

/* Built-in option tuples from the C preprocessor macros:
 * OPT_BW_RAM      "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"
 * OPT_BW_RAM_NOTHP OPT_BW_RAM, "--thp", "-1"
 * OPT_CONV        "-s", "100", "-zZ0qcm", "--thp", " 1"
 * OPT_CONV_NOTHP  OPT_CONV, "--thp", "-1"
 * OPT_BW          "-s", "20", "-zZ0q", "--thp", " 1"
 * OPT_BW_NOTHP    OPT_BW, "--thp", "-1"
 */

/*
 * The built-in test-suite executed by "perf bench numa -a".
 *
 * (A minimum of 4 nodes and 16 GB of RAM is recommended.)
 */
static tests: &[&[&str]] = &[
    /* Basic single-stream NUMA bandwidth measurements: */
    &["RAM-bw-local,", "mem", "-p", "1", "-t", "1", "-P", "1024", "-C", "0", "-M", "0", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"],
    &["RAM-bw-local-NOTHP,", "mem", "-p", "1", "-t", "1", "-P", "1024", "-C", "0", "-M", "0", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk", "--thp", "-1"],
    &["RAM-bw-remote,", "mem", "-p", "1", "-t", "1", "-P", "1024", "-C", "0", "-M", "1", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"],
    /* 2-stream NUMA bandwidth measurements: */
    &["RAM-bw-local-2x,", "mem", "-p", "2", "-t", "1", "-P", "1024", "-C", "0,2", "-M", "0x2", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"],
    &["RAM-bw-remote-2x,", "mem", "-p", "2", "-t", "1", "-P", "1024", "-C", "0,2", "-M", "1x2", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"],
    /* Cross-stream NUMA bandwidth measurement: */
    &["RAM-bw-cross,", "mem", "-p", "2", "-t", "1", "-P", "1024", "-C", "0,8", "-M", "1,0", "-s", "20", "-zZq", "--thp", " 1", "--no-data_rand_walk"],
    /* Convergence latency measurements: */
    &[" 1x3-convergence,", "mem", "-p", "1", "-t", "3", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 1x4-convergence,", "mem", "-p", "1", "-t", "4", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 1x6-convergence,", "mem", "-p", "1", "-t", "6", "-P", "1020", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 2x3-convergence,", "mem", "-p", "2", "-t", "3", "-P", "1020", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 3x3-convergence,", "mem", "-p", "3", "-t", "3", "-P", "1020", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 4x4-convergence,", "mem", "-p", "4", "-t", "4", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 4x4-convergence-NOTHP,", "mem", "-p", "4", "-t", "4", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1", "--thp", "-1"],
    &[" 4x6-convergence,", "mem", "-p", "4", "-t", "6", "-P", "1020", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 4x8-convergence,", "mem", "-p", "4", "-t", "8", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 8x4-convergence,", "mem", "-p", "8", "-t", "4", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 8x4-convergence-NOTHP,", "mem", "-p", "8", "-t", "4", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1", "--thp", "-1"],
    &[" 3x1-convergence,", "mem", "-p", "3", "-t", "1", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 4x1-convergence,", "mem", "-p", "4", "-t", "1", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &[" 8x1-convergence,", "mem", "-p", "8", "-t", "1", "-P", "512", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &["16x1-convergence,", "mem", "-p", "16", "-t", "1", "-P", "256", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    &["32x1-convergence,", "mem", "-p", "32", "-t", "1", "-P", "128", "-s", "100", "-zZ0qcm", "--thp", " 1"],
    /* Various NUMA process/thread layout bandwidth measurements: */
    &[" 2x1-bw-process,", "mem", "-p", "2", "-t", "1", "-P", "1024", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 3x1-bw-process,", "mem", "-p", "3", "-t", "1", "-P", "1024", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 4x1-bw-process,", "mem", "-p", "4", "-t", "1", "-P", "1024", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 8x1-bw-process,", "mem", "-p", "8", "-t", "1", "-P", " 512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 8x1-bw-process-NOTHP,", "mem", "-p", "8", "-t", "1", "-P", " 512", "-s", "20", "-zZ0q", "--thp", " 1", "--thp", "-1"],
    &["16x1-bw-process,", "mem", "-p", "16", "-t", "1", "-P", "256", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 1x4-bw-thread,", "mem", "-p", "1", "-t", "4", "-T", "256", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 1x8-bw-thread,", "mem", "-p", "1", "-t", "8", "-T", "256", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["1x16-bw-thread,", "mem", "-p", "1", "-t", "16", "-T", "128", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["1x32-bw-thread,", "mem", "-p", "1", "-t", "32", "-T", "64", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 2x3-bw-process,", "mem", "-p", "2", "-t", "3", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 4x4-bw-process,", "mem", "-p", "4", "-t", "4", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 4x6-bw-process,", "mem", "-p", "4", "-t", "6", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 4x8-bw-process,", "mem", "-p", "4", "-t", "8", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 4x8-bw-process-NOTHP,", "mem", "-p", "4", "-t", "8", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1", "--thp", "-1"],
    &[" 3x3-bw-process,", "mem", "-p", "3", "-t", "3", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &[" 5x5-bw-process,", "mem", "-p", "5", "-t", "5", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["2x16-bw-process,", "mem", "-p", "2", "-t", "16", "-P", "512", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["1x32-bw-process,", "mem", "-p", "1", "-t", "32", "-P", "2048", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["numa02-bw,", "mem", "-p", "1", "-t", "32", "-T", "32", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["numa02-bw-NOTHP,", "mem", "-p", "1", "-t", "32", "-T", "32", "-s", "20", "-zZ0q", "--thp", " 1", "--thp", "-1"],
    &["numa01-bw-thread,", "mem", "-p", "2", "-t", "16", "-T", "192", "-s", "20", "-zZ0q", "--thp", " 1"],
    &["numa01-bw-thread-NOTHP,", "mem", "-p", "2", "-t", "16", "-T", "192", "-s", "20", "-zZ0q", "--thp", " 1", "--thp", "-1"],
];

unsafe fn bench_all() -> c_int {
    let ret = system(c"echo ' #'; echo ' # Running test on: '$(uname -a); echo ' #'".as_ptr());
    BUG_ON(ret < 0);
    let mut i = 0;
    while i < tests.len() {
        let row = tests[i];
        let mut c_strings: Vec<std::ffi::CString> = row.iter().map(|s| std::ffi::CString::new(*s).unwrap()).collect();
        let mut ptrs: Vec<*const c_char> = c_strings.iter_mut().map(|s| s.as_ptr()).collect();
        ptrs.push(null());
        run_bench_numa(ptrs[0], ptrs.as_ptr().add(1));
        i += 1;
    }
    printf(c"\n".as_ptr());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bench_numa(mut argc: c_int, argv: *const *const c_char) -> c_int {
    init_params(&mut p0, c"main,".as_ptr(), argc, argv);
    argc = parse_options(argc, argv, options.as_ptr(), bench_numa_usage.as_ptr(), 0);
    if argc != 0 {
        usage_with_options(numa_usage.as_ptr(), options.as_ptr());
        return -1;
    }
    if p0.run_all != 0 {
        return bench_all();
    }
    if __bench_numa(null()) != 0 {
        usage_with_options(numa_usage.as_ptr(), options.as_ptr());
        return -1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
