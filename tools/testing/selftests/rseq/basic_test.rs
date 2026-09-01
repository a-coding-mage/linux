// SPDX-License-Identifier: LGPL-2.1
/*
 * Basic test coverage for critical regions and rseq_current_cpu().
 */

// C dependencies removed from executable Rust:
// _GNU_SOURCE, assert.h, sched.h, signal.h, stdio.h, string.h, sys/time.h,
// and "rseq.h".

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

type CpuSetT = libc::cpu_set_t;

const CPU_SETSIZE: c_int = 1024;

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut libc::FILE;

    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: libc::size_t, mask: *mut CpuSetT) -> c_int;
    fn sched_setaffinity(pid: libc::pid_t, cpusetsize: libc::size_t, mask: *const CpuSetT) -> c_int;
    fn sched_getcpu() -> c_int;

    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn CPU_ZERO(cpuset: *mut CpuSetT);
    fn CPU_ISSET(cpu: c_int, cpuset: *const CpuSetT) -> c_int;
    fn CPU_SET(cpu: c_int, cpuset: *mut CpuSetT);
    fn CPU_CLR(cpu: c_int, cpuset: *mut CpuSetT);

    fn rseq_current_cpu() -> c_int;
    fn rseq_current_cpu_raw() -> c_int;
    fn rseq_cpu_start() -> c_int;
    fn rseq_fallback_current_node() -> c_int;
    fn rseq_current_node_id() -> c_int;
    fn rseq_register_current_thread() -> c_int;
    fn rseq_unregister_current_thread() -> c_int;
}

pub unsafe fn test_cpu_pointer() {
    let mut affinity: CpuSetT = core::mem::zeroed();
    let mut test_affinity: CpuSetT = core::mem::zeroed();
    let mut i: c_int;

    sched_getaffinity(0, size_of::<CpuSetT>() as libc::size_t, &mut affinity);
    CPU_ZERO(&mut test_affinity);
    i = 0;
    while i < CPU_SETSIZE {
        if CPU_ISSET(i, &affinity) != 0 {
            let node: c_int;

            CPU_SET(i, &mut test_affinity);
            sched_setaffinity(
                0,
                size_of::<CpuSetT>() as libc::size_t,
                &test_affinity,
            );
            assert!(sched_getcpu() == i);
            assert!(rseq_current_cpu() == i);
            assert!(rseq_current_cpu_raw() == i);
            assert!(rseq_cpu_start() == i);
            node = rseq_fallback_current_node();
            assert!(rseq_current_node_id() == node);
            CPU_CLR(i, &mut test_affinity);
        }
        i += 1;
    }
    sched_setaffinity(0, size_of::<CpuSetT>() as libc::size_t, &affinity);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let _ = argc;
    let _ = argv;

    if rseq_register_current_thread() != 0 {
        fprintf(
            stderr,
            b"Error: rseq_register_current_thread(...) failed(%d): %s\n\0".as_ptr()
                as *const c_char,
            errno,
            strerror(errno),
        );
        return -1;
    }
    printf(b"testing current cpu\n\0".as_ptr() as *const c_char);
    test_cpu_pointer();
    if rseq_unregister_current_thread() != 0 {
        fprintf(
            stderr,
            b"Error: rseq_unregister_current_thread(...) failed(%d): %s\n\0".as_ptr()
                as *const c_char,
            errno,
            strerror(errno),
        );
        return -1;
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
