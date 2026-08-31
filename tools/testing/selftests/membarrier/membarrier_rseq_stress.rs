// SPDX-License-Identifier: GPL-2.0
/*
 * Membarrier stress test for CFS throttle interactions.
 *
 * Reproducer for the interaction between CFS throttle and expedited membarrier.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicI32, AtomicI64, Ordering};

/* -- Architecture-specific rseq signature -- */
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
const RSEQ_SIG: c_uint = 0x53053053u32;
#[cfg(target_arch = "aarch64")]
const RSEQ_SIG: c_uint = 0xd428bc00u32;
#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
const RSEQ_SIG: c_uint = 0x0f000000u32;
#[cfg(any(target_arch = "s390", target_arch = "s390x"))]
const RSEQ_SIG: c_uint = 0x0c000000u32;
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "s390",
    target_arch = "s390x"
)))]
const RSEQ_SIG: c_uint = 0;

/* -- rseq ABI (kernel uapi; define locally for portability) -- */
const RSEQ_CPU_ID_UNINITIALIZED: u32 = !0u32;

#[repr(C, align(32))]
struct rseq_abi {
    cpu_id_start: u32,
    cpu_id: u32,
    rseq_cs: u64,
    flags: u32,
    node_id: u32,
    mm_cid: u32,
}

/* -- membarrier constants (not in all distro headers) -- */
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: c_int = 1 << 7;
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: c_int = 1 << 8;
const MEMBARRIER_CMD_FLAG_CPU: c_int = 1 << 0;

/* -- Test parameters -- */
const N_SIBLINGS: c_int = 2000;
const NEST_DEPTH: c_int = 5;
static mut g_cgroup_path: [c_char; 4096] = [0; 4096];
static mut use_cgroup_v2: c_int = 0;

const CFS_QUOTA_US: c_int = 1000;
const CFS_PERIOD_US: c_int = 5000;
const N_HAMMER_PER_CPU: c_int = 25;
const N_BURNER_PER_CPU: c_int = 50;
const MAX_STRESS_CPUS: c_int = 1024;
const TEST_DURATION_SEC: c_int = 20;

/* Latency thresholds for the sentinel */
const LATENCY_WARN_MS: c_long = 50;
const LATENCY_CRITICAL_MS: c_long = 200;

/* Sentinel sampling interval */
const SENTINEL_INTERVAL_US: c_long = 500;

/* -- Shared globals -- */
static g_stop: AtomicI32 = AtomicI32::new(0);
static g_stop_sentinel: AtomicI32 = AtomicI32::new(0);
static g_max_latency_us: AtomicI64 = AtomicI64::new(0);
static g_interval_max_latency_us: AtomicI64 = AtomicI64::new(0);
static g_mb_ok: AtomicI64 = AtomicI64::new(0);
static g_mb_err: AtomicI64 = AtomicI64::new(0);
static mut g_ncpus_stress: c_int = 0;
static mut g_stress_cpus: *mut c_int = ptr::null_mut();

static g_test_ready: AtomicI32 = AtomicI32::new(0);

/* Per-thread rseq ABI block registered with the kernel */
#[repr(align(32))]
struct TlsRseq(UnsafeCell<rseq_abi>);

unsafe impl Sync for TlsRseq {}

thread_local! {
    static tls_rseq: TlsRseq = TlsRseq(UnsafeCell::new(rseq_abi {
        cpu_id_start: 0,
        cpu_id: RSEQ_CPU_ID_UNINITIALIZED,
        rseq_cs: 0,
        flags: 0,
        node_id: 0,
        mm_cid: 0,
    }));
}

unsafe extern "C" {
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;

    fn __errno_location() -> *mut c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn clock_gettime(clk_id: c_int, tp: *mut libc::timespec) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *mut libc::cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *const libc::cpu_set_t) -> c_int;
    fn stat(path: *const c_char, buf: *mut libc::stat) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn mkdir(pathname: *const c_char, mode: libc::mode_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn opendir(name: *const c_char) -> *mut libc::DIR;
    fn readdir(dirp: *mut libc::DIR) -> *mut libc::dirent;
    fn closedir(dirp: *mut libc::DIR) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn getppid() -> libc::pid_t;
    fn getpid() -> libc::pid_t;
    fn _exit(status: c_int) -> !;
    fn mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: libc::off_t) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn clone(fn_: extern "C" fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void, ...) -> libc::pid_t;
    fn sigemptyset(set: *mut libc::sigset_t) -> c_int;
    fn sigaddset(set: *mut libc::sigset_t, signum: c_int) -> c_int;
    fn sigwait(set: *const libc::sigset_t, sig: *mut c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const libc::sigset_t, oldset: *mut libc::sigset_t) -> c_int;
    fn kill(pid: libc::pid_t, sig: c_int) -> c_int;
    fn waitpid(pid: libc::pid_t, stat_loc: *mut c_int, options: c_int) -> libc::pid_t;
    fn pthread_create(thread: *mut libc::pthread_t, attr: *const libc::pthread_attr_t, start_routine: extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sched_setscheduler(pid: libc::pid_t, policy: c_int, param: *const libc::sched_param) -> c_int;
    fn clock_nanosleep(clock_id: c_int, flags: c_int, request: *const libc::timespec, remain: *mut libc::timespec) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn geteuid() -> libc::uid_t;
}

const O_WRONLY: c_int = libc::O_WRONLY;
const O_CLOEXEC: c_int = libc::O_CLOEXEC;
const EIO: c_int = libc::EIO;
const EBUSY: c_int = libc::EBUSY;
const EINVAL: c_int = libc::EINVAL;
const EEXIST: c_int = libc::EEXIST;
const EAGAIN: c_int = libc::EAGAIN;
const CLOCK_MONOTONIC: c_int = libc::CLOCK_MONOTONIC;
const CPU_SETSIZE: c_int = 1024;
const DT_DIR: u8 = libc::DT_DIR;
const PROT_READ: c_int = libc::PROT_READ;
const PROT_WRITE: c_int = libc::PROT_WRITE;
const MAP_PRIVATE: c_int = libc::MAP_PRIVATE;
const MAP_ANONYMOUS: c_int = libc::MAP_ANONYMOUS;
const CLONE_VM: c_int = libc::CLONE_VM;
const CLONE_THREAD: c_int = libc::CLONE_THREAD;
const CLONE_SIGHAND: c_int = libc::CLONE_SIGHAND;
const SIGTERM: c_int = libc::SIGTERM;
const SIGCHLD: c_int = libc::SIGCHLD;
const SIG_BLOCK: c_int = libc::SIG_BLOCK;
const SIG_UNBLOCK: c_int = libc::SIG_UNBLOCK;
const WNOHANG: c_int = libc::WNOHANG;
const SCHED_FIFO: c_int = libc::SCHED_FIFO;
const PR_SET_PDEATHSIG: c_int = libc::PR_SET_PDEATHSIG;
const SYS_rseq: c_long = libc::SYS_rseq as c_long;
const SYS_membarrier: c_long = libc::SYS_membarrier as c_long;

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

unsafe fn CPU_ZERO(set: *mut libc::cpu_set_t) {
    ptr::write_bytes(set as *mut u8, 0, mem::size_of::<libc::cpu_set_t>());
}

unsafe fn cpu_bits(set: *const libc::cpu_set_t) -> *const c_ulong {
    set as *const c_ulong
}

unsafe fn cpu_bits_mut(set: *mut libc::cpu_set_t) -> *mut c_ulong {
    set as *mut c_ulong
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const libc::cpu_set_t) -> bool {
    let bits_per_word = 8 * mem::size_of::<c_ulong>() as c_int;
    let idx = (cpu / bits_per_word) as isize;
    let mask = (1 as c_ulong) << (cpu % bits_per_word);
    (*cpu_bits(set).offset(idx) & mask) != 0
}

unsafe fn CPU_SET(cpu: c_int, set: *mut libc::cpu_set_t) {
    let bits_per_word = 8 * mem::size_of::<c_ulong>() as c_int;
    let idx = (cpu / bits_per_word) as isize;
    let mask = (1 as c_ulong) << (cpu % bits_per_word);
    *cpu_bits_mut(set).offset(idx) |= mask;
}

/* -- Utility -- */
unsafe fn write_file(path: *const c_char, val: *const c_char) -> c_int {
    let fd = open(path, O_WRONLY | O_CLOEXEC);

    if fd < 0 {
        return -errno();
    }

    let len = strlen(val);
    let r = write(fd, val as *const c_void, len);

    close(fd);
    if r < 0 {
        return -errno();
    }
    if r as usize != len {
        return -EIO;
    }
    0
}

unsafe fn monotonic_us() -> u64 {
    let mut ts: libc::timespec = mem::zeroed();

    clock_gettime(CLOCK_MONOTONIC, &mut ts);
    ts.tv_sec as u64 * 1000000u64 + ts.tv_nsec as u64 / 1000u64
}

fn update_max_latency(lat: c_long) {
    let mut old = g_max_latency_us.load(Ordering::Relaxed);

    while (lat as i64) > old {
        match g_max_latency_us.compare_exchange_weak(old, lat as i64, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(next) => old = next,
        }
    }

    old = g_interval_max_latency_us.load(Ordering::Relaxed);
    while (lat as i64) > old {
        match g_interval_max_latency_us.compare_exchange_weak(old, lat as i64, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(next) => old = next,
        }
    }
}

unsafe fn init_stress_cpus() {
    let mut set: libc::cpu_set_t = mem::zeroed();
    let capacity = MAX_STRESS_CPUS;

    g_stress_cpus = malloc((capacity as usize) * mem::size_of::<c_int>()) as *mut c_int;
    if g_stress_cpus.is_null() {
        ksft_exit_fail_msg(c"malloc failed for g_stress_cpus\n".as_ptr());
    }

    if sched_getaffinity(0, mem::size_of_val(&set), &mut set) < 0 {
        ksft_exit_fail_msg(c"sched_getaffinity failed\n".as_ptr());
    }

    let mut i = 0;
    while i < CPU_SETSIZE && g_ncpus_stress < capacity {
        if CPU_ISSET(i, &set) {
            *g_stress_cpus.offset(g_ncpus_stress as isize) = i;
            g_ncpus_stress += 1;
        }
        i += 1;
    }

    if g_ncpus_stress == 0 {
        ksft_exit_skip(c"No CPUs available for stress test\n".as_ptr());
    }

    ksft_print_msg(c"Stressing %d CPUs discovered via affinity\n".as_ptr(), g_ncpus_stress);
}

/* -- rseq / membarrier helpers -- */
unsafe fn rseq_register_thread() -> c_int {
    let mut ret = 0;
    tls_rseq.with(|rseq| {
        let r = syscall(SYS_rseq, rseq.0.get(), mem::size_of::<rseq_abi>(), 0, RSEQ_SIG);
        ret = if r == 0 || errno() == EBUSY || errno() == EINVAL { 0 } else { -1 };
    });
    ret
}

unsafe fn rseq_register_thread_at(rseq: *mut rseq_abi) -> c_int {
    let r = syscall(SYS_rseq, rseq, mem::size_of::<rseq_abi>(), 0, RSEQ_SIG);

    if r == 0 || errno() == EBUSY || errno() == EINVAL { 0 } else { -1 }
}

unsafe fn membarrier_register_rseq_mm() -> c_long {
    syscall(SYS_membarrier, MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ, 0, 0)
}

/* -- cgroup helpers -- */
unsafe fn rm_cgroup_recursive(path: *const c_char) {
    let dir = opendir(path);

    if dir.is_null() {
        return;
    }
    let mut entry: *mut libc::dirent;

    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if libc::strcmp((*entry).d_name.as_ptr(), c".".as_ptr()) == 0 ||
           libc::strcmp((*entry).d_name.as_ptr(), c"..".as_ptr()) == 0 {
            continue;
        }
        if (*entry).d_type == DT_DIR {
            let mut sub_path: [c_char; 4096] = [0; 4096];

            snprintf(sub_path.as_mut_ptr(), sub_path.len(), c"%s/%s".as_ptr(), path, (*entry).d_name.as_ptr());
            rm_cgroup_recursive(sub_path.as_ptr());
        }
    }
    closedir(dir);
    rmdir(path);
}

unsafe fn cgroup_teardown() {
    rm_cgroup_recursive(g_cgroup_path.as_ptr());
}

unsafe fn cgroup_setup() -> c_int {
    let mut st: libc::stat = mem::zeroed();

    if stat(c"/sys/fs/cgroup/cpu".as_ptr(), &mut st) == 0 {
        use_cgroup_v2 = 0;
        snprintf(g_cgroup_path.as_mut_ptr(), g_cgroup_path.len(), c"/sys/fs/cgroup/cpu/membarrier_stress_test".as_ptr());
    } else if stat(c"/dev/cgroup/cpu".as_ptr(), &mut st) == 0 {
        use_cgroup_v2 = 0;
        snprintf(g_cgroup_path.as_mut_ptr(), g_cgroup_path.len(), c"/dev/cgroup/cpu/membarrier_stress_test".as_ptr());
    } else if stat(c"/cgroup/cpu".as_ptr(), &mut st) == 0 {
        use_cgroup_v2 = 0;
        snprintf(g_cgroup_path.as_mut_ptr(), g_cgroup_path.len(), c"/cgroup/cpu/membarrier_stress_test".as_ptr());
    } else if stat(c"/sys/fs/cgroup/cgroup.controllers".as_ptr(), &mut st) == 0 {
        use_cgroup_v2 = 1;
        snprintf(g_cgroup_path.as_mut_ptr(), g_cgroup_path.len(), c"/sys/fs/cgroup/membarrier_stress_test".as_ptr());
    } else {
        ksft_print_msg(c"WARN: cgroup mount not found. Using v2 at /sys/fs/cgroup\n".as_ptr());
        use_cgroup_v2 = 1;
        snprintf(g_cgroup_path.as_mut_ptr(), g_cgroup_path.len(), c"/sys/fs/cgroup/membarrier_stress_test".as_ptr());
    }

    /* Robust cleanup before setup */
    cgroup_teardown();

    if use_cgroup_v2 != 0 {
        /* Enable cpu controller in root cgroup */
        if write_file(c"/sys/fs/cgroup/cgroup.subtree_control".as_ptr(), c"+cpu".as_ptr()) < 0 {
            ksft_print_msg(c"WARN: failed to enable cpu controller in /sys/fs/cgroup\n".as_ptr());
        }
    }

    if mkdir(g_cgroup_path.as_ptr(), 0o755) < 0 && errno() != EEXIST {
        ksft_print_msg(c"mkdir base %s failed: %s\n".as_ptr(), g_cgroup_path.as_ptr(), strerror(errno()));
        return -1;
    }

    if use_cgroup_v2 != 0 {
        let mut ctrl_path: [c_char; 4096] = [0; 4096];

        snprintf(ctrl_path.as_mut_ptr(), ctrl_path.len(), c"%s/cgroup.subtree_control".as_ptr(), g_cgroup_path.as_ptr());
        if write_file(ctrl_path.as_ptr(), c"+cpu".as_ptr()) < 0 {
            ksft_print_msg(c"WARN: failed to enable cpu controller in %s\n".as_ptr(), g_cgroup_path.as_ptr());
        }
    }

    let mut i = 0;
    while i < N_SIBLINGS {
        let mut sibling_path: [c_char; 4096] = [0; 4096];

        snprintf(sibling_path.as_mut_ptr(), sibling_path.len(), c"%s/n%d".as_ptr(), g_cgroup_path.as_ptr(), i);
        if mkdir(sibling_path.as_ptr(), 0o755) < 0 && errno() != EEXIST {
            ksft_print_msg(c"mkdir wide %s failed: %s\n".as_ptr(), sibling_path.as_ptr(), strerror(errno()));
            return -1;
        }

        if use_cgroup_v2 != 0 {
            let mut ctrl_path: [c_char; 4096] = [0; 4096];

            snprintf(ctrl_path.as_mut_ptr(), ctrl_path.len(), c"%s/cgroup.subtree_control".as_ptr(), sibling_path.as_ptr());
            if write_file(ctrl_path.as_ptr(), c"+cpu".as_ptr()) < 0 {
                ksft_print_msg(c"WARN: failed to enable cpu controller in %s\n".as_ptr(), sibling_path.as_ptr());
            }
        }

        let mut current_path: [c_char; 4096] = [0; 4096];

        snprintf(current_path.as_mut_ptr(), current_path.len(), c"%s".as_ptr(), sibling_path.as_ptr());
        let mut j = 0;
        while j < NEST_DEPTH {
            let len = strlen(current_path.as_ptr());
            snprintf(current_path.as_mut_ptr().add(len), current_path.len() - len, c"/d%d".as_ptr(), j);
            if mkdir(current_path.as_ptr(), 0o755) < 0 && errno() != EEXIST {
                ksft_print_msg(c"mkdir deep %s failed: %s\n".as_ptr(), current_path.as_ptr(), strerror(errno()));
                return -1;
            }

            /* Enable for all but the leaf */
            if use_cgroup_v2 != 0 && j < NEST_DEPTH - 1 {
                let mut ctrl_path: [c_char; 4096] = [0; 4096];

                snprintf(ctrl_path.as_mut_ptr(), ctrl_path.len(), c"%s/cgroup.subtree_control".as_ptr(), current_path.as_ptr());
                if write_file(ctrl_path.as_ptr(), c"+cpu".as_ptr()) < 0 {
                    ksft_print_msg(c"WARN: cannot enable cpu controller in %s\n".as_ptr(), current_path.as_ptr());
                }
            }
            j += 1;
        }
        i += 1;
    }

    let mut quota: [c_char; 64] = [0; 64];
    let mut period: [c_char; 64] = [0; 64];
    let mut max_str: [c_char; 128] = [0; 128];

    snprintf(quota.as_mut_ptr(), quota.len(), c"%d".as_ptr(), CFS_QUOTA_US);
    snprintf(period.as_mut_ptr(), period.len(), c"%d".as_ptr(), CFS_PERIOD_US);
    snprintf(max_str.as_mut_ptr(), max_str.len(), c"%d %d".as_ptr(), CFS_QUOTA_US, CFS_PERIOD_US);

    if use_cgroup_v2 != 0 {
        let mut max_path: [c_char; 4096] = [0; 4096];

        snprintf(max_path.as_mut_ptr(), max_path.len(), c"%s/cpu.max".as_ptr(), g_cgroup_path.as_ptr());
        if write_file(max_path.as_ptr(), max_str.as_ptr()) < 0 {
            ksft_print_msg(c"ERROR: cannot write cpu.max at %s\n".as_ptr(), max_path.as_ptr());
            return -1;
        }
        ksft_print_msg(c"cgroup (v2) %s: cpu.max=%s\n".as_ptr(), g_cgroup_path.as_ptr(), max_str.as_ptr());
    } else {
        let mut quota_path: [c_char; 4096] = [0; 4096];
        let mut period_path: [c_char; 4096] = [0; 4096];

        snprintf(quota_path.as_mut_ptr(), quota_path.len(), c"%s/cpu.cfs_quota_us".as_ptr(), g_cgroup_path.as_ptr());
        snprintf(period_path.as_mut_ptr(), period_path.len(), c"%s/cpu.cfs_period_us".as_ptr(), g_cgroup_path.as_ptr());

        if write_file(period_path.as_ptr(), period.as_ptr()) < 0 {
            ksft_print_msg(c"ERROR: cannot write cpu.cfs_period_us at %s\n".as_ptr(), period_path.as_ptr());
            return -1;
        }
        if write_file(quota_path.as_ptr(), quota.as_ptr()) < 0 {
            ksft_print_msg(c"ERROR: cannot write cpu.cfs_quota_us at %s\n".as_ptr(), quota_path.as_ptr());
            return -1;
        }
        ksft_print_msg(c"cgroup (v1) %s: cpu.cfs_quota_us=%d cpu.cfs_period_us=%d\n".as_ptr(), g_cgroup_path.as_ptr(), CFS_QUOTA_US, CFS_PERIOD_US);
    }

    0
}

unsafe fn cgroup_add_pid_to_path(pid: libc::pid_t, path: *const c_char) -> c_int {
    let mut buf: [c_char; 32] = [0; 32];
    let mut file_path: [c_char; 4096] = [0; 4096];

    snprintf(buf.as_mut_ptr(), buf.len(), c"%d".as_ptr(), pid as c_int);
    if use_cgroup_v2 != 0 {
        snprintf(file_path.as_mut_ptr(), file_path.len(), c"%s/cgroup.procs".as_ptr(), path);
        return write_file(file_path.as_ptr(), buf.as_ptr());
    }
    /* In v1, try tasks first, fallback to cgroup.procs */
    snprintf(file_path.as_mut_ptr(), file_path.len(), c"%s/tasks".as_ptr(), path);
    let mut r = write_file(file_path.as_ptr(), buf.as_ptr());

    if r < 0 {
        snprintf(file_path.as_mut_ptr(), file_path.len(), c"%s/cgroup.procs".as_ptr(), path);
        r = write_file(file_path.as_ptr(), buf.as_ptr());
    }
    r
}

unsafe fn cgroup_unthrottle() {
    if use_cgroup_v2 != 0 {
        let mut max_path: [c_char; 4096] = [0; 4096];

        snprintf(max_path.as_mut_ptr(), max_path.len(), c"%s/cpu.max".as_ptr(), g_cgroup_path.as_ptr());
        write_file(max_path.as_ptr(), c"max".as_ptr());
    } else {
        let mut quota_path: [c_char; 4096] = [0; 4096];

        snprintf(quota_path.as_mut_ptr(), quota_path.len(), c"%s/cpu.cfs_quota_us".as_ptr(), g_cgroup_path.as_ptr());
        write_file(quota_path.as_ptr(), c"-1".as_ptr());
    }
}

/* -- CPU burner (inside throttled child process) -- */
extern "C" fn burner_thread_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let mut my_rseq: rseq_abi = mem::zeroed();
        let cpu = arg as usize as c_int;

        memset(&mut my_rseq as *mut _ as *mut c_void, 0, mem::size_of_val(&my_rseq));
        my_rseq.cpu_id = RSEQ_CPU_ID_UNINITIALIZED;

        if rseq_register_thread_at(&mut my_rseq) < 0 {
            perror(c"rseq_register (burner)".as_ptr());
            return ptr::null_mut();
        }

        let mut set: libc::cpu_set_t = mem::zeroed();

        CPU_ZERO(&mut set);
        CPU_SET(cpu, &mut set);
        if sched_setaffinity(0, mem::size_of_val(&set), &set) < 0 {
            perror(c"sched_setaffinity (burner)".as_ptr());
        }

        let mut sink: c_ulong = 0;

        while g_stop.load(Ordering::Relaxed) == 0 {
            sink = sink.wrapping_add(1);
            /* Prevent compiler from optimizing the loop away */
            core::arch::asm!("", inout(reg) sink);
        }

        ptr::null_mut()
    }
}

extern "C" fn burner_thread_fn_wrapper(arg: *mut c_void) -> c_int {
    burner_thread_fn(arg);
    0
}

extern "C" fn leaf_child_fn(arg: *mut c_void) -> c_int {
    unsafe {
        let i = arg as usize as c_int;
        let total_burners = g_ncpus_stress * N_BURNER_PER_CPU;
        let mut n_threads_per_leaf = total_burners / N_SIBLINGS;

        if i < (total_burners % N_SIBLINGS) {
            n_threads_per_leaf += 1;
        }

        prctl(PR_SET_PDEATHSIG, SIGTERM);
        if getppid() == 1 {
            _exit(1);
        }

        let mut leaf_path: [c_char; 4096] = [0; 4096];

        snprintf(leaf_path.as_mut_ptr(), leaf_path.len(), c"%s/n%d".as_ptr(), g_cgroup_path.as_ptr(), i);
        let mut j = 0;
        while j < NEST_DEPTH {
            let len = strlen(leaf_path.as_ptr());
            snprintf(leaf_path.as_mut_ptr().add(len), leaf_path.len() - len, c"/d%d".as_ptr(), j);
            j += 1;
        }

        let r = cgroup_add_pid_to_path(getpid(), leaf_path.as_ptr());

        if r < 0 {
            let mut buf: [c_char; 512] = [0; 512];
            let len = snprintf(buf.as_mut_ptr(), buf.len(), c"[leaf child %d] failed to join cgroup %s: err %d\n".as_ptr(), i, leaf_path.as_ptr(), -r);
            let _ = !write(2, buf.as_ptr() as *const c_void, len as usize);
            _exit(1);
        }

        j = 0;
        while j < n_threads_per_leaf {
            let cpu = *g_stress_cpus.offset(((i * n_threads_per_leaf + j) % g_ncpus_stress) as isize);

            /* Allocate stack via mmap (bypasses heap) */
            let stack_size: usize = 64 * 1024;
            let stack = mmap(ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
            if stack == libc::MAP_FAILED {
                let msg = c"mmap stack failed\n";
                let _ = !write(2, msg.as_ptr() as *const c_void, strlen(msg.as_ptr()));
                _exit(1);
            }

            /* Use raw clone to create a thread sharing the VM and thread group */
            let pid = clone(burner_thread_fn_wrapper, (stack as *mut u8).add(stack_size) as *mut c_void,
                            CLONE_VM | CLONE_THREAD | CLONE_SIGHAND,
                            cpu as usize as *mut c_void);
            if pid < 0 {
                let msg = c"clone burner failed\n";
                let _ = !write(2, msg.as_ptr() as *const c_void, strlen(msg.as_ptr()));
                _exit(1);
            }
            j += 1;
        }

        // Wait for SIGTERM
        let mut mask: libc::sigset_t = mem::zeroed();

        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGTERM);
        let mut sig: c_int = 0;

        sigwait(&mask, &mut sig);

        _exit(0);
    }
}

#[repr(C)]
struct leaf_info {
    pid: libc::pid_t,
    stack: *mut c_void,
}

extern "C" fn run_throttle_child(arg: *mut c_void) -> c_int {
    unsafe {
        let _ = arg;
        prctl(PR_SET_PDEATHSIG, SIGTERM);
        if getppid() == 1 {
            _exit(1);
        }

        let n_leafs = N_SIBLINGS;

        /* Block signals before spawning to avoid missing early failures */
        let mut mask: libc::sigset_t = mem::zeroed();

        sigemptyset(&mut mask);
        sigaddset(&mut mask, SIGTERM);
        sigaddset(&mut mask, SIGCHLD);
        sigprocmask(SIG_BLOCK, &mask, ptr::null_mut());

        /* Use mmap for tracking structures to avoid glibc heap usage */
        let leaves = mmap(ptr::null_mut(), n_leafs as usize * mem::size_of::<leaf_info>(),
                          PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0) as *mut leaf_info;
        if leaves as *mut c_void == libc::MAP_FAILED {
            let msg = c"mmap leaves array failed\n";
            let _ = !write(2, msg.as_ptr() as *const c_void, strlen(msg.as_ptr()));
            _exit(1);
        }

        let mut i = 0;
        while i < n_leafs {
            let stack_size: usize = 64 * 1024;
            let stack = mmap(ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE,
                             MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
            if stack == libc::MAP_FAILED {
                let msg = c"mmap leaf stack failed\n";
                let _ = !write(2, msg.as_ptr() as *const c_void, strlen(msg.as_ptr()));
                _exit(1);
            }

            (*leaves.offset(i as isize)).stack = stack;

            let pid = clone(leaf_child_fn, (stack as *mut u8).add(stack_size) as *mut c_void,
                            CLONE_VM | SIGCHLD, i as usize as *mut c_void);

            if pid < 0 {
                let msg = c"clone (leaf child) failed\n";
                let _ = !write(2, msg.as_ptr() as *const c_void, strlen(msg.as_ptr()));

                /* Clean up successfully spawned children */
                let mut j = 0;
                while j < i {
                    kill((*leaves.offset(j as isize)).pid, SIGTERM);
                    waitpid((*leaves.offset(j as isize)).pid, ptr::null_mut(), 0);
                    munmap((*leaves.offset(j as isize)).stack, stack_size);
                    j += 1;
                }
                munmap(leaves as *mut c_void, n_leafs as usize * mem::size_of::<leaf_info>());

                if errno() == EAGAIN {
                    _exit(4);
                } else {
                    _exit(1);
                }
            }
            (*leaves.offset(i as isize)).pid = pid;
            i += 1;
        }

        let mut failed = 0;

        loop {
            let mut sig: c_int = 0;

            sigwait(&mask, &mut sig);

            if sig == SIGTERM {
                break;
            } else if sig == SIGCHLD {
                let mut status: c_int = 0;
                let mut pid: libc::pid_t;

                // Reap all dead children
                loop {
                    pid = waitpid(-1, &mut status, WNOHANG);
                    if pid <= 0 {
                        break;
                    }
                    i = 0;
                    while i < n_leafs {
                        if (*leaves.offset(i as isize)).pid == pid {
                            (*leaves.offset(i as isize)).pid = 0;
                            break;
                        }
                        i += 1;
                    }
                    if (WIFEXITED(status) && WEXITSTATUS(status) != 0) ||
                       WIFSIGNALED(status) {
                        let mut buf: [c_char; 128] = [0; 128];
                        let len = snprintf(buf.as_mut_ptr(), buf.len(),
                                           c"[manager] child %d died unexpectedly (status %d)\n".as_ptr(),
                                           pid, WEXITSTATUS(status));
                        let _ = !write(2, buf.as_ptr() as *const c_void, len as usize);
                        failed = 1;
                    }
                }
                if failed != 0 {
                    break;
                }
            }
        }

        // Terminate all leaf kids
        i = 0;
        while i < n_leafs {
            if (*leaves.offset(i as isize)).pid > 0 {
                kill((*leaves.offset(i as isize)).pid, SIGTERM);
            }
            i += 1;
        }

        i = 0;
        while i < n_leafs {
            if (*leaves.offset(i as isize)).pid > 0 {
                waitpid((*leaves.offset(i as isize)).pid, ptr::null_mut(), 0);
            }
            munmap((*leaves.offset(i as isize)).stack, 64 * 1024);
            i += 1;
        }

        munmap(leaves as *mut c_void, n_leafs as usize * mem::size_of::<leaf_info>());

        _exit(if failed != 0 { 1 } else { 0 });
    }
}

/* -- Membarrier hammer thread -- */
extern "C" fn hammer_thread_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let target_cpu = *(arg as *mut c_int);
        let mut local_ok: c_long = 0;
        let mut local_err: c_long = 0;
        let mut count: c_int = 0;
        const batch_size: c_int = 1024;

        if rseq_register_thread() < 0 {
            ksft_print_msg(c"[hammer] rseq_register failed: %s\n".as_ptr(), strerror(errno()));
            return ptr::null_mut();
        }

        membarrier_register_rseq_mm();

        while g_stop.load(Ordering::Relaxed) == 0 {
            let r = syscall(SYS_membarrier,
                            MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
                            MEMBARRIER_CMD_FLAG_CPU,
                            target_cpu);
            if r == 0 {
                local_ok += 1;
            } else {
                local_err += 1;
            }

            count += 1;
            if count >= batch_size {
                g_mb_ok.fetch_add(local_ok as i64, Ordering::Relaxed);
                g_mb_err.fetch_add(local_err as i64, Ordering::Relaxed);
                local_ok = 0;
                local_err = 0;
                count = 0;
            }
        }

        /* Flush any remaining counts on exit */
        if local_ok > 0 {
            g_mb_ok.fetch_add(local_ok as i64, Ordering::Relaxed);
        }
        if local_err > 0 {
            g_mb_err.fetch_add(local_err as i64, Ordering::Relaxed);
        }

        ptr::null_mut()
    }
}

/* -- Latency sentinel -- */
extern "C" fn sentinel_thread_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let _ = arg;
        let sp = libc::sched_param { sched_priority: 20 };

        if sched_setscheduler(0, SCHED_FIFO, &sp) < 0 {
            ksft_print_msg(c"WARN: no SCHED_FIFO for sentinel (less precise)\n".as_ptr());
        }

        while g_test_ready.load(Ordering::Relaxed) == 0 &&
              g_stop_sentinel.load(Ordering::Relaxed) == 0 {
            let ts = libc::timespec { tv_sec: 0, tv_nsec: 1000 * 1000 }; /* 1ms */

            clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, ptr::null_mut());
        }

        let mut prev = monotonic_us();

        while g_stop_sentinel.load(Ordering::Relaxed) == 0 {
            let ts = libc::timespec {
                tv_sec: 0,
                tv_nsec: SENTINEL_INTERVAL_US * 1000,
            };
            clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, ptr::null_mut());

            let now = monotonic_us();
            let latency_us = (now - prev) as c_long - SENTINEL_INTERVAL_US;

            prev = now;

            if latency_us <= 0 {
                continue;
            }

            update_max_latency(latency_us);

            if latency_us > LATENCY_CRITICAL_MS * 1000 {
                ksft_print_msg(c"\n[SENTINEL] CRITICAL: %ld ms delay (lockup precursor!)\n".as_ptr(),
                               latency_us / 1000);
            } else if latency_us > LATENCY_WARN_MS * 1000 {
                ksft_print_msg(c"\n[SENTINEL] WARN: %ld ms latency spike\n".as_ptr(),
                               latency_us / 1000);
            }
        }
        ptr::null_mut()
    }
}

/* -- Progress reporter -- */
extern "C" fn reporter_thread_fn(arg: *mut c_void) -> *mut c_void {
    unsafe {
        let _ = arg;
        let mut elapsed: c_int = 0;

        while g_stop_sentinel.load(Ordering::Relaxed) == 0 {
            let mut i = 0;
            while i < 5 {
                sleep(1);
                if g_stop_sentinel.load(Ordering::Relaxed) != 0 {
                    break;
                }
                i += 1;
            }
            if g_stop_sentinel.load(Ordering::Relaxed) != 0 {
                break;
            }
            elapsed += 5;
            let interval_max = g_interval_max_latency_us.swap(0, Ordering::Relaxed);

            ksft_print_msg(c"[%3ds] mb: ok=%-10ld err=%-8ld | max_lat=%ld us\n".as_ptr(),
                           elapsed,
                           g_mb_ok.load(Ordering::SeqCst) as c_long,
                           g_mb_err.load(Ordering::SeqCst) as c_long,
                           interval_max as c_long);
        }
        ptr::null_mut()
    }
}

/* -- Main -- */
fn main() {
    unsafe {
        ksft_print_header();
        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "aarch64",
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "s390",
            target_arch = "s390x"
        )))]
        {
            ksft_exit_skip(c"Unsupported architecture\n".as_ptr());
        }
        ksft_set_plan(1);

        if geteuid() != 0 {
            ksft_exit_skip(c"Must run as root (cgroup + SCHED_FIFO)\n".as_ptr());
        }

        init_stress_cpus();

        ksft_print_msg(c"=== membarrier rseq + CFS unthrottle stress ===\n".as_ptr());
        ksft_print_msg(c"Stressing CPUs: %d\n".as_ptr(), g_ncpus_stress);
        ksft_print_msg(c"Quota: %d/%d us  (~%d unthrottles/sec/CPU)\n".as_ptr(),
                       CFS_QUOTA_US, CFS_PERIOD_US,
                       1000000 / CFS_PERIOD_US);
        ksft_print_msg(c"Hammer threads: %d per CPU (%d total)\n".as_ptr(),
                       N_HAMMER_PER_CPU, g_ncpus_stress * N_HAMMER_PER_CPU);
        ksft_print_msg(c"Duration: %d seconds\n\n".as_ptr(), TEST_DURATION_SEC);

        if cgroup_setup() < 0 {
            cgroup_teardown();
            ksft_exit_skip(c"cgroup_setup failed (missing permissions or v2 ctrls?)\n".as_ptr());
        }

        if rseq_register_thread() < 0 {
            ksft_print_msg(c"rseq_register (%s) failed: %s\n".as_ptr(), c"main".as_ptr(), strerror(errno()));
            cgroup_teardown();
            ksft_exit_skip(c"rseq syscall failed or not available\n".as_ptr());
        }
        if membarrier_register_rseq_mm() < 0 {
            ksft_print_msg(c"MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: %s\nKernel >= 5.10 with CONFIG_RSEQ required.\n".as_ptr(),
                           strerror(errno()));
            cgroup_teardown();
            ksft_exit_skip(c"membarrier register failed\n".as_ptr());
        }
        ksft_print_msg(c"rseq membarrier registered OK\n".as_ptr());

        let mut sigmask: libc::sigset_t = mem::zeroed();

        sigemptyset(&mut sigmask);
        sigaddset(&mut sigmask, SIGTERM);
        sigprocmask(SIG_BLOCK, &sigmask, ptr::null_mut());

        let stack = malloc(1024 * 1024);

        if stack.is_null() {
            perror(c"malloc stack".as_ptr());
            cgroup_teardown();
            ksft_exit_fail_msg(c"Malloc stack failed\n".as_ptr());
        }
        let child = clone(run_throttle_child, (stack as *mut u8).add(1024 * 1024) as *mut c_void, CLONE_VM | SIGCHLD, ptr::null_mut::<c_void>());

        if child < 0 {
            perror(c"clone".as_ptr());
            cgroup_teardown();
            ksft_exit_fail_msg(c"Clone failed\n".as_ptr());
        }

        sigprocmask(SIG_UNBLOCK, &sigmask, ptr::null_mut());
        ksft_print_msg(c"Throttle child PID %d started\n".as_ptr(), child);

        let n_threads = g_ncpus_stress * N_HAMMER_PER_CPU + 2;
        let threads = calloc(n_threads as usize, mem::size_of::<libc::pthread_t>()) as *mut libc::pthread_t;
        let cpuargs = calloc((g_ncpus_stress * N_HAMMER_PER_CPU) as usize, mem::size_of::<c_int>()) as *mut c_int;

        if threads.is_null() || cpuargs.is_null() {
            perror(c"calloc".as_ptr());
            kill(child, SIGTERM);
            waitpid(child, ptr::null_mut(), 0);
            cgroup_teardown();
            ksft_exit_fail_msg(c"Thread allocation failed\n".as_ptr());
        }

        let mut ti: c_int = 0;
        let mut ai: c_int = 0;
        let mut r: c_int;

        ksft_print_msg(c"Creating sentinel thread...\n".as_ptr());
        r = pthread_create(threads.offset(ti as isize), ptr::null(), sentinel_thread_fn, ptr::null_mut());
        if r != 0 {
            kill(child, SIGTERM);
            waitpid(child, ptr::null_mut(), 0);
            cgroup_teardown();
            free(threads as *mut c_void);
            free(cpuargs as *mut c_void);
            free(g_stress_cpus as *mut c_void);
            ksft_exit_fail_msg(c"pthread_create (sentinel) failed: %s\n".as_ptr(), strerror(r));
        }
        ti += 1;

        ksft_print_msg(c"Creating reporter thread...\n".as_ptr());
        r = pthread_create(threads.offset(ti as isize), ptr::null(), reporter_thread_fn, ptr::null_mut());
        if r != 0 {
            g_stop_sentinel.store(1, Ordering::SeqCst);
            pthread_join(*threads.offset(0), ptr::null_mut());
            kill(child, SIGTERM);
            waitpid(child, ptr::null_mut(), 0);
            cgroup_teardown();
            free(threads as *mut c_void);
            free(cpuargs as *mut c_void);
            free(g_stress_cpus as *mut c_void);
            ksft_exit_fail_msg(c"pthread_create (reporter) failed: %s\n".as_ptr(), strerror(r));
        }
        ti += 1;

        ksft_print_msg(c"Creating %d hammer threads...\n".as_ptr(), g_ncpus_stress * N_HAMMER_PER_CPU);
        let mut i = 0;
        while i < g_ncpus_stress {
            let cpu = *g_stress_cpus.offset(i as isize);

            let mut j = 0;
            while j < N_HAMMER_PER_CPU {
                *cpuargs.offset(ai as isize) = cpu;
                r = pthread_create(threads.offset(ti as isize), ptr::null(), hammer_thread_fn, cpuargs.offset(ai as isize) as *mut c_void);
                if r != 0 {
                    ksft_print_msg(c"pthread_create failed at thread %d: %s\n".as_ptr(),
                                   ti, strerror(r));

                    g_stop_sentinel.store(1, Ordering::SeqCst);
                    pthread_join(*threads.offset(0), ptr::null_mut());
                    pthread_join(*threads.offset(1), ptr::null_mut());

                    g_stop.store(1, Ordering::SeqCst);
                    let mut k = 2;
                    while k < ti {
                        pthread_join(*threads.offset(k as isize), ptr::null_mut());
                        k += 1;
                    }

                    kill(child, SIGTERM);
                    waitpid(child, ptr::null_mut(), 0);
                    cgroup_teardown();

                    free(threads as *mut c_void);
                    free(cpuargs as *mut c_void);
                    free(g_stress_cpus as *mut c_void);

                    if r == EAGAIN {
                        ksft_exit_skip(c"Resource limits prevent threads\n".as_ptr());
                    } else {
                        ksft_exit_fail_msg(c"Failed to create hammer thread\n".as_ptr());
                    }
                }
                ti += 1;
                ai += 1;
                j += 1;
            }
            i += 1;
        }

        ksft_print_msg(c"All threads running. Tip: monitor dmesg for lockups\n\n".as_ptr());

        g_test_ready.store(1, Ordering::Relaxed);
        let mut child_failed = 0;
        let mut child_status: c_int = 0;

        i = 0;
        while i < TEST_DURATION_SEC {
            sleep(1);
            let wr = waitpid(child, &mut child_status, WNOHANG);

            if wr == child {
                child_failed = 1;
                break;
            }
            i += 1;
        }

        g_stop_sentinel.store(1, Ordering::SeqCst);
        pthread_join(*threads.offset(0), ptr::null_mut());
        pthread_join(*threads.offset(1), ptr::null_mut());

        g_stop.store(1, Ordering::SeqCst);

        /* Unthrottle to allow children to exit quickly */
        cgroup_unthrottle();

        if child_failed == 0 {
            kill(child, SIGTERM);
            waitpid(child, ptr::null_mut(), 0);
        }
        i = 2;
        while i < ti {
            pthread_join(*threads.offset(i as isize), ptr::null_mut());
            i += 1;
        }

        let max_lat = g_max_latency_us.load(Ordering::SeqCst) as c_long;
        let total_ok = g_mb_ok.load(Ordering::SeqCst) as c_long;
        let total_err = g_mb_err.load(Ordering::SeqCst) as c_long;

        ksft_print_msg(c"\n=== RESULTS ===\n".as_ptr());
        ksft_print_msg(c"membarrier syscalls : %ld ok  %ld errors\n".as_ptr(), total_ok, total_err);
        ksft_print_msg(c"Max scheduler latency: %ld us  (%ld ms)\n".as_ptr(), max_lat, max_lat / 1000);
        cgroup_teardown();
        free(threads as *mut c_void);
        free(cpuargs as *mut c_void);
        free(g_stress_cpus as *mut c_void);

        if child_failed != 0 {
            if WIFEXITED(child_status) && WEXITSTATUS(child_status) == 4 {
                ksft_exit_skip(c"Manager child skipped (resource limits?)\n".as_ptr());
            }
            ksft_test_result_fail(c"membarrier_rseq_stress: Manager child died early\n".as_ptr());
            ksft_exit_fail();
        } else if total_ok == 0 {
            ksft_test_result_fail(c"membarrier_rseq_stress: No successful membarrier calls\n".as_ptr());
            ksft_exit_fail();
        } else if total_err > 0 {
            ksft_test_result_fail(c"membarrier_rseq_stress: syscall errors\n".as_ptr());
            ksft_exit_fail();
        } else if max_lat > LATENCY_CRITICAL_MS * 1000 {
            ksft_test_result_fail(c"membarrier_rseq_stress: LOCKUP PRECURSOR\n".as_ptr());
            ksft_exit_fail();
        } else if max_lat > LATENCY_WARN_MS * 1000 {
            ksft_test_result_fail(c"membarrier_rseq_stress: significant latency spike\n".as_ptr());
            ksft_exit_fail();
        } else {
            ksft_test_result_pass(c"membarrier_rseq_stress\n".as_ptr());
            ksft_exit_pass();
        }
    }
}
