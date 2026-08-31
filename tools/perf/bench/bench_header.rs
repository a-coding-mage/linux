/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/bench/bench.h.
// C dependency intent: #include <sys/time.h>

extern "C" {
    pub static mut bench__start: libc::timeval;
    pub static mut bench__end: libc::timeval;
    pub static mut bench__runtime: libc::timeval;
}

/*
 * The madvise transparent hugepage constants were added in glibc
 * 2.13. For compatibility with older versions of glibc, define these
 * tokens if they are not already defined.
 */
pub const MADV_HUGEPAGE: libc::c_int = 14;
pub const MADV_NOHUGEPAGE: libc::c_int = 15;

extern "C" {
    pub fn bench_numa(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_sched_messaging(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_sched_pipe(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_sched_seccomp_notify(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
    ) -> libc::c_int;
    pub fn bench_syscall_basic(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_syscall_getpgid(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_syscall_fork(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_syscall_execve(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_mem_memcpy(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_mem_memset(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_mem_mmap(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_mem_find_bit(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_futex_hash(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_futex_wake(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_futex_wake_parallel(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
    ) -> libc::c_int;
    pub fn bench_futex_requeue(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    /* pi futexes */
    pub fn bench_futex_lock_pi(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_epoll_wait(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_epoll_ctl(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_synthesize(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_kallsyms_parse(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_inject_build_id(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_evlist_open_close(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_breakpoint_thread(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_breakpoint_enable(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_uprobe_baseline(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_uprobe_empty(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_uprobe_trace_printk(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
    ) -> libc::c_int;
    pub fn bench_uprobe_empty_ret(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
    pub fn bench_uprobe_trace_printk_ret(
        argc: libc::c_int,
        argv: *const *const libc::c_char,
    ) -> libc::c_int;
    pub fn bench_pmu_scan(argc: libc::c_int, argv: *const *const libc::c_char) -> libc::c_int;
}

pub const BENCH_FORMAT_DEFAULT_STR: &str = "default";
pub const BENCH_FORMAT_DEFAULT: libc::c_int = 0;
pub const BENCH_FORMAT_SIMPLE_STR: &str = "simple";
pub const BENCH_FORMAT_SIMPLE: libc::c_int = 1;

pub const BENCH_FORMAT_UNKNOWN: libc::c_int = -1;

extern "C" {
    pub static mut bench_format: libc::c_int;
    pub static mut bench_repeat: libc::c_uint;
}

// C conditional intent:
// #ifndef HAVE_PTHREAD_ATTR_SETAFFINITY_NP
// #include <pthread.h>
// #include <linux/compiler.h>
#[cfg(not(HAVE_PTHREAD_ATTR_SETAFFINITY_NP))]
#[allow(non_camel_case_types)]
pub type pthread_attr_t = libc::pthread_attr_t;

#[cfg(not(HAVE_PTHREAD_ATTR_SETAFFINITY_NP))]
#[allow(non_camel_case_types)]
pub type cpu_set_t = libc::cpu_set_t;

#[cfg(not(HAVE_PTHREAD_ATTR_SETAFFINITY_NP))]
#[inline]
pub unsafe fn pthread_attr_setaffinity_np(
    attr: *mut pthread_attr_t,
    cpusetsize: libc::size_t,
    cpuset: *mut cpu_set_t,
) -> libc::c_int {
    let _ = attr;
    let _ = cpusetsize;
    let _ = cpuset;
    0
}
